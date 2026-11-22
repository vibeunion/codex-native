use anyhow::{bail, Context as _, Result};
use pi_agent_core::agent_config::{self, MAX_BUNDLE_BYTES};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    net::IpAddr,
    time::Duration,
};

pub const MAX_CONFIGURATIONS: usize = 32;

#[derive(Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub id: String,
    pub source_url: Option<String>,
    // Preserve the original envelope, including unknown fields and encrypted secrets.
    pub bundle: String,
    pub imported_at: u64,
    pub checked_at: Option<u64>,
}

impl Configuration {
    pub fn label(&self) -> String {
        agent_config::preview_bundle(&self.bundle)
            .ok()
            .and_then(|preview| preview.label)
            .unwrap_or_else(|| "AgentConfig".into())
    }

    pub fn source_label(&self) -> String {
        self.source_url
            .as_deref()
            .and_then(|source| Url::parse(source).ok())
            .and_then(|url| url.host_str().map(str::to_owned))
            .unwrap_or_default()
    }
}

pub struct PendingConfiguration<'a> {
    pub target: Option<&'a str>,
    pub original: Option<&'a str>,
    pub source: Option<String>,
    pub checked_at: Option<u64>,
    pub bundle: String,
}

pub fn import_candidate(
    current: &[Configuration],
    pending: PendingConfiguration<'_>,
) -> Result<Vec<Configuration>> {
    agent_config::preview_bundle(&pending.bundle)?;
    let mut updated = current.to_vec();
    if let Some(target) = pending.target {
        let entry = updated
            .iter_mut()
            .find(|config| config.id == target)
            .context("配置源已被移除，请重新检查 / Source removed; check again")?;
        if pending.original != Some(entry.bundle.as_str()) {
            bail!("本地配置已变化，请重新检查 / Local configuration changed; check again");
        }
        entry.bundle = pending.bundle;
        entry.imported_at = crate::desktop_state::now();
        if let Some(checked_at) = pending.checked_at {
            entry.checked_at = Some(checked_at);
        }
    } else {
        if current.len() >= MAX_CONFIGURATIONS {
            bail!("配置包数量已达上限 / Configuration library is full");
        }
        if pending.source.as_ref().is_some_and(|source| {
            current
                .iter()
                .any(|config| config.source_url.as_ref() == Some(source))
        }) {
            bail!("配置源已存在，请从列表检查更新 / Source already exists; check updates from the list");
        }
        updated.push(Configuration {
            id: crate::desktop_state::new_id(),
            checked_at: pending.checked_at,
            source_url: pending.source,
            bundle: pending.bundle,
            imported_at: crate::desktop_state::now(),
        });
    }
    Ok(updated)
}

pub fn source_url(raw: &str) -> Result<Url> {
    if raw.len() > 8192 {
        bail!("Configuration source URL is too long");
    }
    let url = Url::parse(raw).context("Invalid configuration source URL")?;
    if url.scheme() != "https"
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        bail!("A configuration source must be an HTTPS URL without userinfo or fragment");
    }
    Ok(url)
}

// Application extension, separate from ACB's snapshot-only `import` route.
pub fn subscription_link(source: &str) -> Result<String> {
    let source = source_url(source)?;
    let mut link = Url::parse("agentconfig://subscribe").expect("constant subscription URL");
    link.query_pairs_mut()
        .append_pair("v", "1")
        .append_pair("url", source.as_str());
    Ok(link.into())
}

pub fn subscription_source(raw: &str) -> Result<Option<String>> {
    if raw.len() > agent_config::MAX_DEEP_LINK_URL_BYTES {
        bail!("Subscription link is too long");
    }
    let url = Url::parse(raw)?;
    if url.host_str() != Some("subscribe") {
        return Ok(None);
    }
    if url.scheme() != "agentconfig"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port().is_some()
        || !matches!(url.path(), "" | "/")
        || url.fragment().is_some()
    {
        bail!("Invalid subscription route");
    }
    let mut version = None;
    let mut source = None;
    for (name, content) in url.query_pairs() {
        match name.as_ref() {
            "v" if version.is_none() => version = Some(content.into_owned()),
            "url" if source.is_none() => source = Some(content.into_owned()),
            _ => bail!("Duplicate or unsupported subscription parameter"),
        }
    }
    if version.as_deref() != Some("1") {
        bail!("Unsupported subscription version");
    }
    Ok(Some(
        source_url(&source.context("Missing subscription source")?)?.to_string(),
    ))
}

fn public_address(address: IpAddr) -> bool {
    match address {
        IpAddr::V4(ip) => {
            let [a, b, c, _] = ip.octets();
            !(ip.is_private()
                || ip.is_loopback()
                || ip.is_link_local()
                || ip.is_broadcast()
                || ip.is_documentation()
                || ip.is_multicast()
                || a == 0
                || a >= 240
                || (a == 100 && (64..=127).contains(&b))
                || (a == 198 && (b == 18 || b == 19))
                || (a == 192 && b == 0 && c == 0))
        }
        IpAddr::V6(ip) => {
            let segments = ip.segments();
            // Only global unicast; reject mapped/transition and documentation ranges.
            (segments[0] & 0xe000) == 0x2000
                && segments[0] != 0x2002
                && !(segments[0] == 0x2001 && (segments[1] < 0x0200 || segments[1] == 0x0db8))
                && !(segments[0] == 0x3fff && (segments[1] & 0xf000) == 0)
        }
    }
}

pub async fn fetch_bundle(raw_url: &str) -> Result<String> {
    let url = source_url(raw_url)?;
    // The timeout includes DNS, TLS and streaming, not just response headers.
    tokio::time::timeout(Duration::from_secs(20), fetch_public_bundle(url))
        .await
        .context("Configuration source timed out")?
}

async fn fetch_public_bundle(url: Url) -> Result<String> {
    let host = url.host_str().context("Missing source host")?;
    let addresses: Vec<_> = tokio::net::lookup_host((
        host.trim_matches(['[', ']']),
        url.port_or_known_default().unwrap_or(443),
    ))
    .await?
    .collect();
    if addresses.is_empty()
        || addresses
            .iter()
            .any(|address| !public_address(address.ip()))
    {
        bail!("Configuration sources require a public network address");
    }
    let client = Client::builder()
        .https_only(true)
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .resolve_to_addrs(host, &addresses)
        .timeout(Duration::from_secs(20))
        .build()?;
    download_bundle(&client, url).await
}

async fn download_bundle(client: &Client, url: Url) -> Result<String> {
    let mut response = client
        .get(url)
        .header(
            reqwest::header::ACCEPT,
            "application/x-agentconfig+json, application/json",
        )
        .send()
        .await?;
    if !response.status().is_success() {
        bail!("Configuration source did not return a successful response");
    }
    if response
        .content_length()
        .is_some_and(|size| size > MAX_BUNDLE_BYTES as u64)
    {
        bail!("Configuration source is too large");
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await? {
        if bytes.len() + chunk.len() > MAX_BUNDLE_BYTES {
            bail!("Configuration source is too large");
        }
        bytes.extend_from_slice(&chunk);
    }
    let raw = String::from_utf8(bytes).context("Configuration source is not UTF-8")?;
    agent_config::preview_bundle(&raw)?;
    Ok(raw)
}

pub fn same_content(previous: &str, incoming: &str) -> Result<bool> {
    let mut previous: Value = serde_json::from_str(previous)?;
    let mut incoming: Value = serde_json::from_str(incoming)?;
    for envelope in [&mut previous, &mut incoming] {
        envelope
            .as_object_mut()
            .context("Invalid bundle")?
            .remove("created");
    }
    Ok(previous == incoming)
}

#[derive(Debug, PartialEq, Eq)]
pub struct SectionChange {
    pub section: String,
    pub added: usize,
    pub removed: usize,
    pub changed: usize,
}

pub fn changes(previous: Option<&str>, incoming: &str) -> Result<Vec<SectionChange>> {
    let previous: Value = previous
        .map(serde_json::from_str)
        .transpose()?
        .unwrap_or_default();
    let incoming: Value = serde_json::from_str(incoming)?;
    let old_public = previous.get("pub").and_then(Value::as_object);
    let new_public = incoming.get("pub").and_then(Value::as_object);
    let sections: BTreeSet<_> = old_public
        .into_iter()
        .flat_map(|section| section.keys())
        .chain(new_public.into_iter().flat_map(|section| section.keys()))
        .collect();
    let mut changes = Vec::new();
    for section in sections {
        let old = old_public.and_then(|public| public.get(section));
        let new = new_public.and_then(|public| public.get(section));
        if old != new {
            changes.push(section_change(section, old, new));
        }
    }
    let envelope_fields: BTreeSet<_> = previous
        .as_object()
        .into_iter()
        .flat_map(|envelope| envelope.keys())
        .chain(
            incoming
                .as_object()
                .into_iter()
                .flat_map(|envelope| envelope.keys()),
        )
        .collect();
    for section in envelope_fields {
        if !matches!(section.as_str(), "pub" | "created")
            && previous.get(section) != incoming.get(section)
        {
            changes.push(section_change(
                section,
                previous.get(section),
                incoming.get(section),
            ));
        }
    }
    Ok(changes)
}

fn section_change(section: &str, old: Option<&Value>, new: Option<&Value>) -> SectionChange {
    let old = entry_groups(section, old);
    let new = entry_groups(section, new);
    let mut change = SectionChange {
        section: section.into(),
        added: 0,
        removed: 0,
        changed: 0,
    };
    let keys: BTreeSet<_> = old.keys().chain(new.keys()).collect();
    for key in keys {
        if old.get(key) != new.get(key) {
            let old_count = old.get(key).map_or(0, Vec::len);
            let new_count = new.get(key).map_or(0, Vec::len);
            change.added += new_count.saturating_sub(old_count);
            change.removed += old_count.saturating_sub(new_count);
            change.changed += old_count.min(new_count);
        }
    }
    change
}

fn entry_groups<'a>(section: &str, field: Option<&'a Value>) -> BTreeMap<String, Vec<&'a Value>> {
    let mut groups: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
    let Some(field) = field else {
        return groups;
    };
    if let Some(entries) = field.as_array() {
        for entry in entries {
            let identity = match section {
                "models" => serde_json::json!([entry.get("provider"), entry.get("id")]),
                "mcp" => entry.get("name").cloned().unwrap_or_else(|| entry.clone()),
                "skills" | "agents" | "prompts" => {
                    entry.get("id").cloned().unwrap_or_else(|| entry.clone())
                }
                "resources" => entry.get("uri").cloned().unwrap_or_else(|| entry.clone()),
                _ => entry.clone(),
            };
            groups.entry(identity.to_string()).or_default().push(entry);
        }
    } else {
        groups.insert(section.into(), vec![field]);
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_urls_are_https_without_embedded_userinfo() {
        for source in [
            "http://example.com/a.acfg",
            "file:///a",
            "https://u:p@example.com/a",
            "https://example.com/a#x",
        ] {
            assert!(source_url(source).is_err());
        }
        assert!(source_url("https://example.com/setup.acfg?revision=1").is_ok());
        assert!(source_url(&format!("https://example.com/{}", "a".repeat(8192))).is_err());
    }

    #[test]
    fn subscription_extension_round_trips_and_rejects_ambiguous_routes() {
        let source = "https://example.com/team.acfg?revision=1&team=a";
        let link = subscription_link(source).unwrap();
        assert_eq!(subscription_source(&link).unwrap().as_deref(), Some(source));
        assert!(subscription_source("agentconfig://import?v=1&bundle=abc")
            .unwrap()
            .is_none());
        for link in [
            "agentconfig://subscribe?v=1&v=1&url=https%3A%2F%2Fexample.com",
            "agentconfig://subscribe?v=1&url=https%3A%2F%2Fexample.com&url=https%3A%2F%2Fother.com",
            "agentconfig://subscribe?v=2&url=https%3A%2F%2Fexample.com",
            "agentconfig://subscribe?v=1&url=file%3A%2F%2F%2Ftmp",
            "agentconfig://subscribe/path?v=1&url=https%3A%2F%2Fexample.com",
            "agentconfig://user@subscribe?v=1&url=https%3A%2F%2Fexample.com",
            "https://subscribe?v=1&url=https%3A%2F%2Fexample.com",
        ] {
            assert!(subscription_source(link).is_err(), "{link}");
        }
    }

    #[test]
    fn sources_cannot_reach_private_or_special_networks() {
        for ip in [
            "127.0.0.1",
            "10.0.0.1",
            "169.254.169.254",
            "192.168.0.1",
            "100.64.1.1",
            "0.0.0.0",
            "192.0.2.1",
            "198.18.0.1",
            "224.0.0.1",
            "::1",
            "fc00::1",
            "fe80::1",
            "::ffff:127.0.0.1",
            "2001:db8::1",
            "2002:7f00:1::",
            "2001::1",
        ] {
            assert!(!public_address(ip.parse().unwrap()), "{ip}");
        }
        assert!(public_address("8.8.8.8".parse().unwrap()));
        assert!(public_address("2606:4700:4700::1111".parse().unwrap()));
    }

    #[test]
    fn whole_bundle_diff_preserves_unknown_sections_and_ignores_only_creation_time() {
        let old = r#"{"created":1,"pub":{"models":[{"id":"a"}],"future":[{"id":"x"}]},"payload":{"ct":"a"}}"#;
        let reordered = r#"{"payload":{"ct":"a"},"pub":{"future":[{"id":"x"}],"models":[{"id":"a"}]},"created":2}"#;
        assert!(same_content(old, reordered).unwrap());
        let new = r#"{"created":3,"pub":{"models":[{"id":"b"},{"id":"c"}]},"payload":{"ct":"b"}}"#;
        let diff = changes(Some(old), new).unwrap();
        assert!(diff.contains(&SectionChange {
            section: "models".into(),
            added: 2,
            removed: 1,
            changed: 0
        }));
        assert!(diff.contains(&SectionChange {
            section: "future".into(),
            added: 0,
            removed: 1,
            changed: 0
        }));
        assert!(diff.iter().any(|change| change.section == "payload"));
        assert!(changes(None, old)
            .unwrap()
            .iter()
            .any(|change| change.section == "future" && change.added == 1));
        let unknown = r#"{"created":1,"pub":{},"futureEnvelope":{"enabled":true}}"#;
        assert!(changes(None, unknown)
            .unwrap()
            .iter()
            .any(|change| change.section == "futureEnvelope"));
        let change = section_change(
            "models",
            Some(&serde_json::json!([
                {"provider":"p","id":"a","maxTokens":10}
            ])),
            Some(&serde_json::json!([
                {"provider":"p","id":"a","maxTokens":20}
            ])),
        );
        assert_eq!((change.added, change.removed, change.changed), (0, 0, 1));
    }

    fn model_free_bundle() -> String {
        r#"{"schema":"agentconfig-bundle","v":1,"created":1,"trust":"shared","capabilities":["mcp","future"],"payload":{"alg":"none","ct":"e30="},"pub":{"models":[],"mcp":[{"name":"example","transport":"stdio","command":"example"}],"future":{"keep":true}},"unknownEnvelope":{"keep":true}}"#.into()
    }

    #[test]
    fn import_updates_a_complete_bundle_only_after_consent_and_checks_the_baseline() {
        let original = model_free_bundle();
        let current = import_candidate(
            &[],
            PendingConfiguration {
                target: None,
                original: None,
                source: Some("https://example.com/team.acfg".into()),
                checked_at: Some(123),
                bundle: original.clone(),
            },
        )
        .unwrap();
        let id = current[0].id.clone();
        let mut revised: Value = serde_json::from_str(&original).unwrap();
        revised["pub"]["mcp"][0]["command"] = "revised".into();
        let revised = revised.to_string();
        assert_eq!(current[0].bundle, original);
        let updated = import_candidate(
            &current,
            PendingConfiguration {
                target: Some(&id),
                original: Some(&original),
                source: current[0].source_url.clone(),
                checked_at: None,
                bundle: revised.clone(),
            },
        )
        .unwrap();
        assert_eq!(updated.len(), 1);
        assert_eq!(updated[0].bundle, revised);
        assert_eq!(updated[0].source_url, current[0].source_url);
        assert_eq!(updated[0].checked_at, Some(123));
        assert!(agent_config::resolve_bundle(&updated[0].bundle, None)
            .unwrap()
            .model_config
            .is_none());
        let preserved: Value = serde_json::from_str(&updated[0].bundle).unwrap();
        assert_eq!(preserved["unknownEnvelope"]["keep"], true);
        assert_eq!(preserved["pub"]["future"]["keep"], true);
        assert!(import_candidate(
            &updated,
            PendingConfiguration {
                target: Some(&id),
                original: Some(&original),
                source: None,
                checked_at: None,
                bundle: original.clone(),
            }
        )
        .is_err());
        assert!(import_candidate(
            &[],
            PendingConfiguration {
                target: Some(&id),
                original: Some(&original),
                source: None,
                checked_at: None,
                bundle: original.clone(),
            }
        )
        .is_err());
        assert!(import_candidate(
            &current,
            PendingConfiguration {
                target: None,
                original: None,
                source: current[0].source_url.clone(),
                checked_at: None,
                bundle: original,
            }
        )
        .is_err());
    }

    #[tokio::test]
    async fn download_validates_and_retains_the_entire_model_free_bundle() {
        let raw = model_free_bundle();
        let url = serve_once(format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            raw.len(),
            raw
        ));
        let client = Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        assert_eq!(download_bundle(&client, url).await.unwrap(), raw);
        assert!(fetch_bundle("https://127.0.0.1/config.acfg").await.is_err());
        assert!(fetch_bundle("https://[::1]/config.acfg").await.is_err());
    }

    #[tokio::test]
    async fn download_is_bounded_and_rejects_redirects_html_and_http_errors() {
        for reply in [
            "HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1/private\r\nContent-Length: 0\r\n\r\n"
                .to_owned(),
            "HTTP/1.1 500 Error\r\nContent-Length: 0\r\n\r\n".into(),
            "HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\nhtml".into(),
            format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
                MAX_BUNDLE_BYTES + 1
            ),
            format!(
                "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n{:x}\r\n{}\r\n0\r\n\r\n",
                MAX_BUNDLE_BYTES + 1,
                "x".repeat(MAX_BUNDLE_BYTES + 1)
            ),
        ] {
            let url = serve_once(reply);
            let client = Client::builder()
                .no_proxy()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .unwrap();
            assert!(download_bundle(&client, url).await.is_err());
        }
    }

    fn serve_once(reply: String) -> Url {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut request = [0; 4096];
            assert!(stream.read(&mut request).unwrap() > 0);
            // Oversized-response tests intentionally close the socket mid-body.
            let _ = stream.write_all(reply.as_bytes());
        });
        Url::parse(&format!("http://{address}/config.acfg")).unwrap()
    }
}
