use pi_agent_core::agent_config::{self, ImportedConfig, MAX_DEEP_LINK_URL_BYTES};
use tokio::sync::mpsc::Sender;

pub fn read_bundle_file(path: &std::path::Path) -> anyhow::Result<String> {
    use std::io::Read as _;
    if !std::fs::metadata(path)?.is_file() {
        anyhow::bail!("AgentConfig import requires a regular file");
    }
    let mut raw = String::new();
    std::fs::File::open(path)?
        .take(agent_config::MAX_BUNDLE_BYTES as u64 + 1)
        .read_to_string(&mut raw)?;
    if raw.len() > agent_config::MAX_BUNDLE_BYTES {
        anyhow::bail!("AgentConfig file is too large");
    }
    Ok(raw)
}

#[derive(Debug, thiserror::Error)]
pub enum ImportFailure {
    #[error("Invalid AgentConfig content or password")]
    InvalidBundle(#[source] anyhow::Error),
    #[error("The selected model needs a valid HTTP(S) endpoint")]
    InvalidEndpoint,
}

pub fn can_decrypt(
    preview: &agent_config::AgentConfigPreview,
    model_index: Option<usize>,
    password: &str,
) -> bool {
    preview.requires_password
        && model_index.is_none_or(|index| index < preview.models.len())
        && !password.trim().is_empty()
}

pub fn resolve_import(
    raw: &str,
    password: Option<&str>,
    model_index: Option<usize>,
) -> Result<ImportedConfig, ImportFailure> {
    let imported = match model_index {
        Some(_) => agent_config::import_bundle_selection(raw, password, model_index),
        None => agent_config::resolve_bundle(raw, password),
    }
    .map_err(ImportFailure::InvalidBundle)?;
    if imported.model_config.as_ref().is_some_and(|config| {
        !config
            .api_base
            .as_deref()
            .is_some_and(crate::desktop_state::validate_endpoint)
    }) {
        return Err(ImportFailure::InvalidEndpoint);
    }
    Ok(imported)
}

pub fn enqueue_url(sender: &Sender<String>, url: String) {
    let is_agentconfig = url
        .split_once(':')
        .is_some_and(|(scheme, _)| scheme.eq_ignore_ascii_case("agentconfig"));
    if !is_agentconfig {
        return;
    }
    if url.len() > MAX_DEEP_LINK_URL_BYTES {
        tracing::warn!("Rejected oversized AgentConfig URL");
        return;
    }
    // Never log the URL: bundles may contain encrypted credentials or private endpoints.
    if sender.try_send(url).is_err() {
        tracing::warn!("AgentConfig URL queue unavailable; paste the link in settings to retry");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oversized_files_are_rejected_before_reading_the_whole_file() {
        use std::io::Write as _;
        let path = std::env::temp_dir().join(crate::desktop_state::new_id());
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(&vec![b' '; agent_config::MAX_BUNDLE_BYTES + 1])
            .unwrap();
        drop(file);
        assert!(read_bundle_file(&path).is_err());
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn agentconfig_decrypt_does_not_require_model_activation() {
        let mut preview = agent_config::AgentConfigPreview {
            label: None,
            source: None,
            trust: "self".into(),
            capabilities: vec![],
            models: vec![agent_config::PreviewModel {
                provider: "example".into(),
                id: "model".into(),
                alias: None,
            }],
            skill_ids: vec![],
            agent_ids: vec![],
            mcp_names: vec![],
            prompt_ids: vec![],
            resource_uris: vec![],
            requires_password: true,
        };
        for password in ["", " ", "\n"] {
            assert!(!can_decrypt(&preview, Some(0), password));
        }
        assert!(can_decrypt(&preview, None, "test-password"));
        assert!(!can_decrypt(&preview, Some(1), "test-password"));
        assert!(can_decrypt(&preview, Some(0), "test-password"));
        preview.requires_password = false;
        assert!(!can_decrypt(&preview, Some(0), "test-password"));
        preview.requires_password = true;
        preview.models.clear();
        assert!(!can_decrypt(&preview, Some(0), "test-password"));
        assert!(can_decrypt(&preview, None, "test-password"));
    }

    #[test]
    fn preview_requires_an_endpoint_and_does_not_invent_credentials() {
        let config = pi_agent_core::ModelConfig {
            provider: "example".into(),
            model: "example-model".into(),
            api_base: Some("https://example.com/v1".into()),
            api_key: None,
            temperature: 0.2,
            max_tokens: None,
        };
        let (raw, _) =
            agent_config::export_bundle(&config, &[], agent_config::ExportOptions::default())
                .unwrap();
        let resolved = resolve_import(&raw, None, Some(0)).unwrap();
        assert_eq!(
            resolved.model_config.unwrap().api_base.as_deref(),
            Some("https://example.com/v1")
        );
        let mut envelope: serde_json::Value = serde_json::from_str(&raw).unwrap();
        envelope["payload"]["ct"] = "e30=".into();
        assert!(matches!(
            resolve_import(&serde_json::to_string(&envelope).unwrap(), None, Some(0)),
            Err(ImportFailure::InvalidEndpoint)
        ));
        assert!(resolve_import(&raw, None, Some(1)).is_err());
        assert!(resolve_import(&raw, None, Some(0))
            .unwrap()
            .model_config
            .unwrap()
            .api_key
            .is_none());
        let mut multiple: serde_json::Value = serde_json::from_str(&raw).unwrap();
        multiple["pub"]["models"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!({
                "provider": "example", "id": "second-model"
            }));
        let imported = resolve_import(&multiple.to_string(), None, None).unwrap();
        assert!(imported.model_config.is_none());
        assert_eq!(imported.preview.models.len(), 2);
        envelope["pub"]["models"] = serde_json::json!([]);
        assert!(
            resolve_import(&serde_json::to_string(&envelope).unwrap(), None, None)
                .unwrap()
                .model_config
                .is_none()
        );
        assert!(matches!(
            resolve_import("invalid-json", None, None),
            Err(ImportFailure::InvalidBundle(_))
        ));
    }

    #[test]
    fn startup_and_running_links_use_the_same_bounded_queue() {
        let (sender, mut receiver) = tokio::sync::mpsc::channel(2);
        enqueue_url(&sender, "--portable".into());
        enqueue_url(&sender, "https://example.com".into());
        enqueue_url(&sender, "agentconfig://import?v=1&bundle=first".into());
        enqueue_url(&sender, "AGENTCONFIG://import?v=1&bundle=second".into());
        enqueue_url(&sender, "agentconfig://import?v=1&bundle=overflow".into());
        assert!(receiver.try_recv().unwrap().ends_with("first"));
        assert!(receiver.try_recv().unwrap().ends_with("second"));
        assert!(receiver.try_recv().is_err());
        enqueue_url(
            &sender,
            format!("agentconfig:{}", "a".repeat(MAX_DEEP_LINK_URL_BYTES)),
        );
        assert!(receiver.try_recv().is_err());
    }
}
