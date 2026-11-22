//! Adapter for VibeUnion's AgentConfig Bundle format.
//!
//! The format is kept at the import/export boundary. The runtime continues to
//! use `ModelConfig`, `SkillRegistry`, and native workflow types, so importing
//! a bundle cannot silently change execution policy.

use crate::skills::Skill;
use crate::types::ModelConfig;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::{bail, Context, Result};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

pub const DEFAULT_SOURCE: &str = "codex-native";
pub const SCHEMA_ID: &str = "agentconfig-bundle";
pub const VERSION: u32 = 1;
pub const FILE_EXTENSION: &str = ".acfg";
pub const DEEP_LINK_SCHEME: &str = "agentconfig";
pub const MAX_BUNDLE_BYTES: usize = 1_000_000;
pub const MAX_DEEP_LINK_BYTES: usize = 20_000;
// Allow percent-encoded payloads, but bound the carrier before URL parsing.
pub const MAX_DEEP_LINK_URL_BYTES: usize = MAX_DEEP_LINK_BYTES * 3 + 1024;
const MIN_ITERATIONS: u32 = 100_000;
const MAX_ITERATIONS: u32 = 1_000_000;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TrustMode {
    #[default]
    Shared,
    #[serde(rename = "self")]
    Self_,
    Managed,
}

impl TrustMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Shared => "shared",
            Self::Self_ => "self",
            Self::Managed => "managed",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExportOptions {
    pub label: Option<String>,
    pub trust: TrustMode,
    pub password: Option<String>,
    pub include_credentials: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentConfigPreview {
    pub label: Option<String>,
    pub source: Option<String>,
    pub trust: String,
    pub capabilities: Vec<String>,
    pub models: Vec<PreviewModel>,
    pub skill_ids: Vec<String>,
    pub agent_ids: Vec<String>,
    pub mcp_names: Vec<String>,
    pub prompt_ids: Vec<String>,
    pub resource_uris: Vec<String>,
    pub requires_password: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewModel {
    pub provider: String,
    pub id: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ImportedConfig {
    pub preview: AgentConfigPreview,
    pub model_config: Option<ModelConfig>,
    pub enabled_skill_ids: Vec<String>,
    pub custom_prompts: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct ModelEntry {
    provider: String,
    id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    context_window: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    model_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    generation_modes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    parameters: Option<HashMap<String, serde_json::Value>>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SkillEntry {
    id: String,
    #[serde(default = "default_true")]
    enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    order: Option<i64>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AgentEntry {
    id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    skill_ids: Vec<String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct PublicSection {
    #[serde(default)]
    mcp: Vec<serde_json::Value>,
    #[serde(default)]
    models: Vec<ModelEntry>,
    #[serde(default)]
    skills: Vec<SkillEntry>,
    #[serde(default)]
    prompts: Vec<serde_json::Value>,
    #[serde(default)]
    agents: Vec<AgentEntry>,
    #[serde(default)]
    resources: Vec<serde_json::Value>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct ProviderSecret {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    env: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    headers: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    oauth: Option<serde_json::Value>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct ProviderHint {
    provider: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    base_url: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct SecretSection {
    #[serde(default)]
    endpoints: HashMap<String, String>,
    #[serde(default)]
    custom_prompts: HashMap<String, String>,
    #[serde(default)]
    provider_hints: Vec<ProviderHint>,
    #[serde(default)]
    secrets: HashMap<String, ProviderSecret>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Payload {
    alg: String,
    ct: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    iterations: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    salt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    iv: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Bundle {
    schema: String,
    v: u32,
    created: i64,
    #[serde(default = "default_shared")]
    trust: String,
    #[serde(default)]
    capabilities: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    src: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hint: Option<String>,
    payload: Payload,
    #[serde(rename = "pub")]
    pub_section: PublicSection,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

fn default_true() -> bool {
    true
}

fn default_shared() -> String {
    "shared".into()
}

pub fn export_bundle(
    model_config: &ModelConfig,
    skills: &[Skill],
    options: ExportOptions,
) -> Result<(String, String)> {
    if options.include_credentials && options.trust == TrustMode::Shared {
        bail!("shared AgentConfig bundles cannot carry provider credentials");
    }
    if options.include_credentials
        && model_config
            .api_key
            .as_deref()
            .is_some_and(|key| !key.is_empty())
        && options.password.as_deref().is_none()
    {
        bail!("credentials require a password-encrypted AgentConfig bundle");
    }

    let public = PublicSection {
        models: vec![ModelEntry {
            provider: model_config.provider.clone(),
            id: model_config.model.clone(),
            max_tokens: model_config.max_tokens.map(u64::from),
            max_output_tokens: model_config.max_tokens.map(u64::from),
            model_type: Some("text".into()),
            ..ModelEntry::default()
        }],
        skills: skills
            .iter()
            .enumerate()
            .map(|(index, skill)| SkillEntry {
                id: skill.id.clone(),
                enabled: true,
                order: Some(index as i64),
                extra: HashMap::new(),
            })
            .collect(),
        agents: vec![AgentEntry {
            id: "codex-native".into(),
            name: Some(crate::AppIdentity::from_environment().brand_name),
            model: Some(model_config.model.clone()),
            skill_ids: skills.iter().map(|skill| skill.id.clone()).collect(),
            extra: HashMap::new(),
        }],
        ..PublicSection::default()
    };

    let mut secret = SecretSection::default();
    if let Some(api_base) = model_config
        .api_base
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        secret
            .endpoints
            .insert(model_config.provider.clone(), api_base.to_string());
        secret.provider_hints.push(ProviderHint {
            provider: model_config.provider.clone(),
            base_url: Some(api_base.to_string()),
            extra: HashMap::new(),
        });
    }
    if options.include_credentials {
        if let Some(api_key) = model_config
            .api_key
            .as_deref()
            .filter(|value| !value.is_empty())
        {
            secret.secrets.insert(
                model_config.provider.clone(),
                ProviderSecret {
                    api_key: Some(api_key.to_string()),
                    ..ProviderSecret::default()
                },
            );
        }
    }

    let secret_json = serde_json::to_string(&secret).context("serialize AgentConfig secrets")?;
    let payload = if let Some(password) = options.password.as_deref() {
        let encrypted = encrypt(&secret_json, password, MIN_ITERATIONS)?;
        Payload {
            alg: "PBKDF2-SHA256-AES-256-GCM".into(),
            ct: encode_b64(&encrypted.ciphertext),
            iterations: Some(encrypted.iterations),
            salt: Some(encode_b64(&encrypted.salt)),
            iv: Some(encode_b64(&encrypted.iv)),
            extra: HashMap::new(),
        }
    } else {
        Payload {
            alg: "none".into(),
            ct: encode_b64(secret_json.as_bytes()),
            iterations: None,
            salt: None,
            iv: None,
            extra: HashMap::new(),
        }
    };
    let bundle = Bundle {
        schema: SCHEMA_ID.into(),
        v: VERSION,
        created: now_ms(),
        trust: options.trust.as_str().into(),
        capabilities: capabilities(&public),
        label: options.label,
        src: Some(DEFAULT_SOURCE.into()),
        hint: None,
        payload,
        pub_section: public,
        extra: HashMap::new(),
    };
    validate_bundle(&bundle)?;
    let file = serde_json::to_string_pretty(&bundle).context("serialize AgentConfig bundle")?;
    if file.len() > MAX_BUNDLE_BYTES {
        bail!("AgentConfig bundle exceeds {MAX_BUNDLE_BYTES} bytes");
    }
    let link_payload = serde_json::to_vec(&bundle).context("encode AgentConfig deep link")?;
    let encoded = URL_SAFE_NO_PAD.encode(link_payload);
    if encoded.len() > MAX_DEEP_LINK_BYTES {
        bail!("AgentConfig deep link exceeds {MAX_DEEP_LINK_BYTES} characters");
    }
    let link = format!("{DEEP_LINK_SCHEME}://import?v={VERSION}&bundle={encoded}");
    Ok((file, link))
}

pub fn preview_bundle(raw: &str) -> Result<AgentConfigPreview> {
    let bundle = parse_file_bundle(raw)?;
    Ok(preview_from_bundle(&bundle))
}

pub fn import_bundle(raw: &str, password: Option<&str>) -> Result<ImportedConfig> {
    import_bundle_selection(raw, password, None)
}

pub fn import_bundle_selection(
    raw: &str,
    password: Option<&str>,
    model_index: Option<usize>,
) -> Result<ImportedConfig> {
    let bundle = parse_file_bundle(raw)?;
    if bundle.pub_section.models.len() > 1 && model_index.is_none() {
        bail!("AgentConfig bundle contains multiple models; choose one before import");
    }
    let selected_model = match model_index {
        Some(index) => Some(
            bundle
                .pub_section
                .models
                .get(index)
                .context("selected AgentConfig model is out of range")?,
        ),
        None => bundle.pub_section.models.first(),
    };
    let secret = reveal_secret(&bundle, password)?;
    let model_config = selected_model.map(|model| {
        let api_base = secret.endpoints.get(&model.provider).cloned().or_else(|| {
            secret
                .provider_hints
                .iter()
                .find(|hint| hint.provider == model.provider)
                .and_then(|hint| hint.base_url.clone())
        });
        let api_key = secret
            .secrets
            .get(&model.provider)
            .and_then(|provider| provider.api_key.clone());
        ModelConfig {
            provider: model.provider.clone(),
            model: model.id.clone(),
            api_base,
            api_key,
            temperature: 0.2,
            max_tokens: model
                .max_output_tokens
                .or(model.max_tokens)
                .and_then(|tokens| u32::try_from(tokens).ok()),
        }
    });
    let mut imported = imported_from_bundle(&bundle, secret);
    imported.model_config = model_config;
    Ok(imported)
}

/// Resolve a complete bundle without requiring or activating a model.
pub fn resolve_bundle(raw: &str, password: Option<&str>) -> Result<ImportedConfig> {
    let bundle = parse_file_bundle(raw)?;
    let secret = reveal_secret(&bundle, password)?;
    Ok(imported_from_bundle(&bundle, secret))
}

fn imported_from_bundle(bundle: &Bundle, secret: SecretSection) -> ImportedConfig {
    ImportedConfig {
        preview: preview_from_bundle(bundle),
        model_config: None,
        enabled_skill_ids: bundle
            .pub_section
            .skills
            .iter()
            .filter(|skill| skill.enabled)
            .map(|skill| skill.id.clone())
            .collect(),
        custom_prompts: secret.custom_prompts,
    }
}

pub fn import_deep_link(raw_url: &str, password: Option<&str>) -> Result<ImportedConfig> {
    import_bundle(&bundle_from_deep_link(raw_url)?, password)
}

pub fn bundle_from_deep_link(raw_url: &str) -> Result<String> {
    if raw_url.len() > MAX_DEEP_LINK_URL_BYTES {
        bail!("AgentConfig URL is too large");
    }
    let url = Url::parse(raw_url).context("parse AgentConfig deep link")?;
    if url.scheme() != DEEP_LINK_SCHEME
        || url.host_str() != Some("import")
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port().is_some()
        || !matches!(url.path(), "" | "/")
        || url.fragment().is_some()
    {
        bail!("expected an agentconfig://import deep link");
    }
    let version =
        unique_query_parameter(&url, "v")?.context("deep link is missing the version parameter")?;
    if version != VERSION.to_string() {
        bail!("unsupported AgentConfig deep link version");
    }
    let bundle = unique_query_parameter(&url, "bundle")?
        .context("deep link is missing the bundle parameter")?;
    if bundle.len() > MAX_DEEP_LINK_BYTES {
        bail!("AgentConfig deep link exceeds {MAX_DEEP_LINK_BYTES} characters");
    }
    let bytes = URL_SAFE_NO_PAD
        .decode(bundle)
        .context("decode AgentConfig deep link payload")?;
    let raw = String::from_utf8(bytes).context("AgentConfig deep link is not UTF-8")?;
    parse_file_bundle(&raw)?;
    Ok(raw)
}

fn unique_query_parameter(url: &Url, name: &str) -> Result<Option<String>> {
    let mut matches = url.query_pairs().filter(|(key, _)| key == name);
    let first = matches.next().map(|(_, value)| value.into_owned());
    if matches.next().is_some() {
        bail!("duplicate AgentConfig {name} parameter");
    }
    Ok(first)
}

fn parse_file_bundle(raw: &str) -> Result<Bundle> {
    if raw.len() > MAX_BUNDLE_BYTES {
        bail!("AgentConfig bundle exceeds {MAX_BUNDLE_BYTES} bytes");
    }
    let bundle: Bundle = serde_json::from_str(raw).context("parse AgentConfig JSON")?;
    validate_bundle(&bundle)?;
    Ok(bundle)
}

fn validate_bundle(bundle: &Bundle) -> Result<()> {
    if bundle.schema != SCHEMA_ID {
        bail!("unsupported AgentConfig schema: {}", bundle.schema);
    }
    if bundle.v != VERSION {
        bail!("unsupported AgentConfig version: {}", bundle.v);
    }
    if !matches!(bundle.trust.as_str(), "shared" | "self" | "managed") {
        bail!("unsupported AgentConfig trust mode: {}", bundle.trust);
    }
    match bundle.payload.alg.as_str() {
        "none" => {
            let raw = decode_b64(&bundle.payload.ct)?;
            let secret: SecretSection =
                serde_json::from_slice(&raw).context("parse AgentConfig secret section")?;
            validate_secret_policy(&bundle.trust, &secret, false)?;
        }
        "PBKDF2-SHA256-AES-256-GCM" => {
            let iterations = bundle
                .payload
                .iterations
                .context("encrypted AgentConfig payload is missing iterations")?;
            if !(MIN_ITERATIONS..=MAX_ITERATIONS).contains(&iterations) {
                bail!("AgentConfig PBKDF2 iterations must be between {MIN_ITERATIONS} and {MAX_ITERATIONS}");
            }
            let salt = decode_b64(bundle.payload.salt.as_deref().context("missing salt")?)?;
            let iv = decode_b64(bundle.payload.iv.as_deref().context("missing IV")?)?;
            let ciphertext = decode_b64(&bundle.payload.ct)?;
            if salt.len() != 16 || iv.len() != 12 || ciphertext.len() < 16 {
                bail!("invalid AgentConfig encryption field lengths");
            }
        }
        algorithm => bail!("unsupported AgentConfig payload algorithm: {algorithm}"),
    }
    Ok(())
}

fn reveal_secret(bundle: &Bundle, password: Option<&str>) -> Result<SecretSection> {
    let bytes = match bundle.payload.alg.as_str() {
        "none" => decode_b64(&bundle.payload.ct)?,
        "PBKDF2-SHA256-AES-256-GCM" => {
            let password = password.context("AgentConfig password required")?;
            let salt = decode_b64(bundle.payload.salt.as_deref().context("missing salt")?)?;
            let iv = decode_b64(bundle.payload.iv.as_deref().context("missing IV")?)?;
            let ciphertext = decode_b64(&bundle.payload.ct)?;
            decrypt(
                &ciphertext,
                password,
                &salt,
                &iv,
                bundle.payload.iterations.context("missing iterations")?,
            )?
        }
        _ => bail!("unsupported AgentConfig payload algorithm"),
    };
    let secret: SecretSection =
        serde_json::from_slice(&bytes).context("parse AgentConfig secret section")?;
    validate_secret_policy(&bundle.trust, &secret, bundle.payload.alg != "none")?;
    Ok(secret)
}

fn preview_from_bundle(bundle: &Bundle) -> AgentConfigPreview {
    AgentConfigPreview {
        label: bundle.label.clone(),
        source: bundle.src.clone(),
        trust: bundle.trust.clone(),
        capabilities: bundle.capabilities.clone(),
        models: bundle
            .pub_section
            .models
            .iter()
            .map(|model| PreviewModel {
                provider: model.provider.clone(),
                id: model.id.clone(),
                alias: model.alias.clone(),
            })
            .collect(),
        skill_ids: bundle
            .pub_section
            .skills
            .iter()
            .map(|skill| skill.id.clone())
            .collect(),
        agent_ids: bundle
            .pub_section
            .agents
            .iter()
            .map(|agent| agent.id.clone())
            .collect(),
        mcp_names: public_strings(&bundle.pub_section.mcp, "name"),
        prompt_ids: public_strings(&bundle.pub_section.prompts, "id"),
        resource_uris: public_strings(&bundle.pub_section.resources, "uri"),
        requires_password: bundle.payload.alg != "none",
    }
}

fn public_strings(entries: &[serde_json::Value], key: &str) -> Vec<String> {
    entries
        .iter()
        .filter_map(|entry| entry.get(key)?.as_str().map(str::to_owned))
        .collect()
}

fn capabilities(public: &PublicSection) -> Vec<String> {
    let mut result = Vec::new();
    if !public.mcp.is_empty() {
        result.push("mcp".into());
    }
    if !public.models.is_empty() {
        result.push("models".into());
    }
    if !public.skills.is_empty() {
        result.push("skills".into());
    }
    if !public.prompts.is_empty() {
        result.push("prompts".into());
    }
    if !public.agents.is_empty() {
        result.push("agents".into());
    }
    if !public.resources.is_empty() {
        result.push("resources".into());
    }
    result
}

fn has_credentials(secret: &SecretSection) -> bool {
    secret.secrets.values().any(|provider| {
        provider
            .api_key
            .as_deref()
            .is_some_and(|value| !value.is_empty())
            || !provider.env.is_empty()
            || !provider.headers.is_empty()
            || provider.oauth.is_some()
    })
}

fn validate_secret_policy(trust: &str, secret: &SecretSection, encrypted: bool) -> Result<()> {
    for endpoint in secret.endpoints.values() {
        if !valid_endpoint(endpoint) {
            bail!("AgentConfig endpoint must be an HTTP(S) URL without credentials");
        }
    }
    for hint in &secret.provider_hints {
        if let Some(base_url) = hint.base_url.as_deref() {
            if !valid_endpoint(base_url) {
                bail!("AgentConfig provider hint must be an HTTP(S) URL without credentials");
            }
        }
    }
    if !has_credentials(secret) {
        return Ok(());
    }
    if trust == "shared" {
        bail!("shared AgentConfig bundles cannot carry credentials");
    }
    if !encrypted {
        bail!("credential-bearing AgentConfig bundles must be encrypted");
    }
    Ok(())
}

fn valid_endpoint(endpoint: &str) -> bool {
    Url::parse(endpoint).is_ok_and(|url| {
        matches!(url.scheme(), "http" | "https")
            && url.host_str().is_some()
            && url.username().is_empty()
            && url.password().is_none()
            && url.query().is_none()
            && url.fragment().is_none()
    })
}

struct EncryptionResult {
    ciphertext: Vec<u8>,
    salt: Vec<u8>,
    iv: Vec<u8>,
    iterations: u32,
}

fn encrypt(plaintext: &str, password: &str, iterations: u32) -> Result<EncryptionResult> {
    if password.is_empty() {
        bail!("AgentConfig password must not be empty");
    }
    let mut salt = [0u8; 16];
    let mut iv = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut salt);
    rand::thread_rng().fill_bytes(&mut iv);
    let key = derive_key(password, &salt, iterations);
    let cipher = Aes256Gcm::new_from_slice(&key).context("create AES-256-GCM cipher")?;
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&iv), plaintext.as_bytes())
        .map_err(|error| anyhow::anyhow!("encrypt AgentConfig secret section: {error}"))?;
    Ok(EncryptionResult {
        ciphertext,
        salt: salt.to_vec(),
        iv: iv.to_vec(),
        iterations,
    })
}

fn decrypt(
    ciphertext: &[u8],
    password: &str,
    salt: &[u8],
    iv: &[u8],
    iterations: u32,
) -> Result<Vec<u8>> {
    if password.is_empty() {
        bail!("AgentConfig password must not be empty");
    }
    let key = derive_key(password, salt, iterations);
    let cipher = Aes256Gcm::new_from_slice(&key).context("create AES-256-GCM cipher")?;
    cipher
        .decrypt(Nonce::from_slice(iv), ciphertext)
        .map_err(|error| anyhow::anyhow!("decrypt AgentConfig secret section: {error}"))
}

fn derive_key(password: &str, salt: &[u8], iterations: u32) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, iterations, &mut key);
    key
}

fn encode_b64(value: &[u8]) -> String {
    STANDARD.encode(value)
}

fn decode_b64(value: &str) -> Result<Vec<u8>> {
    let bytes = STANDARD
        .decode(value)
        .context("invalid AgentConfig base64")?;
    if STANDARD.encode(&bytes) != value {
        bail!("non-canonical AgentConfig base64");
    }
    Ok(bytes)
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skills::SkillRegistry;

    fn model(api_key: Option<&str>) -> ModelConfig {
        ModelConfig {
            provider: "example".into(),
            model: "example-chat".into(),
            api_base: Some("https://example.com/v1".into()),
            api_key: api_key.map(str::to_string),
            temperature: 0.2,
            max_tokens: Some(2048),
        }
    }

    #[test]
    fn shared_export_contains_preview_but_no_credentials() {
        let skills = SkillRegistry::with_builtin_skills();
        let (raw, link) = export_bundle(
            &model(Some("secret")),
            skills.skills(),
            ExportOptions::default(),
        )
        .unwrap();
        assert!(link.starts_with("agentconfig://import?v=1&bundle="));
        let preview = preview_bundle(&raw).unwrap();
        assert_eq!(preview.trust, "shared");
        assert!(!preview.requires_password);
        let imported = import_bundle(&raw, None).unwrap();
        assert_eq!(imported.model_config.unwrap().api_key, None);
        let imported_from_link = import_deep_link(&link, None).unwrap();
        assert_eq!(
            imported_from_link.model_config.unwrap().model,
            "example-chat"
        );
    }

    #[test]
    fn self_export_round_trips_encrypted_credentials() {
        let skills = SkillRegistry::with_builtin_skills();
        let (raw, _) = export_bundle(
            &model(Some("secret")),
            skills.skills(),
            ExportOptions {
                label: Some("Test setup".into()),
                trust: TrustMode::Self_,
                password: Some("correct horse battery staple".into()),
                include_credentials: true,
            },
        )
        .unwrap();
        assert!(preview_bundle(&raw).unwrap().requires_password);
        assert!(import_bundle(&raw, None).is_err());
        let imported = import_bundle(&raw, Some("correct horse battery staple")).unwrap();
        assert_eq!(
            imported.model_config.unwrap().api_key.as_deref(),
            Some("secret")
        );
    }

    #[test]
    fn shared_credentials_are_rejected() {
        let skills = SkillRegistry::with_builtin_skills();
        let result = export_bundle(
            &model(Some("secret")),
            skills.skills(),
            ExportOptions {
                include_credentials: true,
                ..ExportOptions::default()
            },
        );
        assert!(result.is_err());
    }

    #[test]
    fn endpoint_credentials_are_rejected_before_export() {
        let skills = SkillRegistry::with_builtin_skills();
        let mut config = model(None);
        config.api_base = Some("https://user:secret@example.com/v1".into());
        assert!(export_bundle(&config, skills.skills(), ExportOptions::default()).is_err());
    }

    #[test]
    fn unknown_public_fields_are_ignored_and_previewed() {
        let raw = r#"{
          "schema":"agentconfig-bundle","v":1,"created":1,
          "trust":"shared","capabilities":["models","future-cap"],
          "payload":{"alg":"none","ct":"eyJlbmRwb2ludHMiOnt9fQ=="},
          "pub":{"models":[{"provider":"p","id":"m","futureModelField":true}]},
          "futureEnvelopeField":{"kept":true}
        }"#;
        let preview = preview_bundle(raw).unwrap();
        assert_eq!(preview.capabilities, vec!["models", "future-cap"]);
        assert_eq!(preview.models[0].id, "m");
    }

    #[test]
    fn encrypted_link_can_be_previewed_without_a_password() {
        let (raw, link) = export_bundle(
            &model(Some("synthetic-test-key")),
            &[],
            ExportOptions {
                trust: TrustMode::Self_,
                password: Some("test-password".into()),
                include_credentials: true,
                ..ExportOptions::default()
            },
        )
        .unwrap();
        let decoded = bundle_from_deep_link(&link).unwrap();
        assert_eq!(
            preview_bundle(&decoded).unwrap(),
            preview_bundle(&raw).unwrap()
        );
        assert!(preview_bundle(&decoded).unwrap().requires_password);
        assert!(import_deep_link(&link, Some("wrong-password")).is_err());
        assert_eq!(
            import_deep_link(&link, Some("test-password"))
                .unwrap()
                .model_config
                .unwrap()
                .api_key
                .as_deref(),
            Some("synthetic-test-key")
        );
    }

    #[test]
    fn deep_link_rejects_ambiguous_routes_versions_and_payloads() {
        let (_, link) = export_bundle(&model(None), &[], ExportOptions::default()).unwrap();
        let encoded = link.split("bundle=").nth(1).unwrap();
        // Missing / duplicate / unsupported parameters, wrong route, malformed / oversized payloads.
        for invalid in [
            format!("agentconfig://import?bundle={encoded}"),
            format!("agentconfig://import?v=2&bundle={encoded}"),
            format!("{link}&v=1"),
            format!("{link}&bundle={encoded}"),
            format!("{link}#fragment"),
            link.replace("://import?", "://import/extra?"),
            link.replace("://import?", "://user@import?"),
            link.replace("://import?", "://import:80?"),
            link.replace("://import?", "://auth/callback?"),
            link.replace("agentconfig:", "https:"),
            "agentconfig://import?v=1".into(),
            "agentconfig://import?v=1&bundle=***".into(),
            format!(
                "agentconfig://import?v=1&bundle={}",
                "a".repeat(MAX_DEEP_LINK_BYTES + 1)
            ),
            format!("{link}&extra={}", "x".repeat(MAX_DEEP_LINK_URL_BYTES)),
        ] {
            assert!(bundle_from_deep_link(&invalid).is_err());
        }
        let mut envelope: serde_json::Value =
            serde_json::from_str(&bundle_from_deep_link(&link).unwrap()).unwrap();
        envelope["v"] = 2.into();
        let mismatched = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&envelope).unwrap());
        assert!(
            bundle_from_deep_link(&format!("agentconfig://import?v=1&bundle={mismatched}"))
                .is_err()
        );
    }

    #[test]
    fn multiple_models_require_an_explicit_valid_selection() {
        let (raw, _) = export_bundle(&model(None), &[], ExportOptions::default()).unwrap();
        let mut envelope: serde_json::Value = serde_json::from_str(&raw).unwrap();
        envelope["pub"]["models"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!({"provider":"example", "id":"second-model"}));
        let raw = serde_json::to_string(&envelope).unwrap();
        assert!(import_bundle(&raw, None).is_err());
        assert!(import_bundle_selection(&raw, None, Some(2)).is_err());
        assert_eq!(
            import_bundle_selection(&raw, None, Some(1))
                .unwrap()
                .model_config
                .unwrap()
                .model,
            "second-model"
        );
    }
}
