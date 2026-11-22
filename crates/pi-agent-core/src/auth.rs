//! Provider-agnostic API-key and OAuth authentication.
//!
//! Generic OAuth endpoints are supplied by provider metadata or environment
//! configuration. ChatGPT uses the public Codex client defaults, while one
//! provider has its own device/polling flow.

use anyhow::{bail, Context, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::RngCore;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::{Read, Write},
    net::TcpListener as StdTcpListener,
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthProvider {
    ChatGpt,
    DeepSeek,
    #[serde(rename = "zcode")]
    Provider,
    /// Kept for compatibility with older local preference files.
    Pi,
    Custom,
}

impl AuthProvider {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChatGpt => "chatgpt",
            Self::DeepSeek => "deepseek",
            Self::Provider => "zcode",
            Self::Pi => "pi",
            Self::Custom => "custom",
        }
    }

    fn env_prefix(self) -> &'static str {
        match self {
            Self::ChatGpt => "CHATGPT",
            Self::DeepSeek => "DEEPSEEK",
            Self::Provider => "ZCODE",
            Self::Pi => "PI",
            Self::Custom => "CUSTOM",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "chatgpt" | "chatgpt_oauth" => Some(Self::ChatGpt),
            "deepseek" | "deepseek_oauth" => Some(Self::DeepSeek),
            "provider" | "provider_oauth" | "zcode" | "zcode_oauth" | "z.ai" | "zai" => {
                Some(Self::Provider)
            }
            "pi" | "pi_oauth" => Some(Self::Pi),
            "custom" | "custom_oauth" => Some(Self::Custom),
            _ => None,
        }
    }

    pub fn supplies_model_credential(self) -> bool {
        // Browser-poll login returns an account token, not a Coding Plan model credential.
        self != Self::Provider
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProviderConfig {
    pub provider: AuthProvider,
    pub authorization_url: Url,
    pub token_url: Url,
    pub client_id: String,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default)]
    pub extra_authorization_params: HashMap<String, String>,
}

impl OAuthProviderConfig {
    /// Read official provider metadata from explicit environment variables.
    ///
    /// Example for a provider whose public OAuth documentation supplies the
    /// endpoints:
    ///
    /// `VOLT_CHATGPT_OAUTH_AUTH_URL`, `VOLT_CHATGPT_OAUTH_TOKEN_URL`,
    /// `VOLT_CHATGPT_OAUTH_CLIENT_ID`, and `VOLT_CHATGPT_OAUTH_SCOPES`.
    pub fn from_environment(provider: AuthProvider) -> Result<Self> {
        let prefix = format!("VOLT_{}_OAUTH", provider.env_prefix());
        let required = |suffix: &str| {
            std::env::var(format!("{prefix}_{suffix}"))
                .with_context(|| format!("missing {prefix}_{suffix}"))
        };
        let (authorization_url, token_url, client_id) = if provider == AuthProvider::ChatGpt
            && std::env::var(format!("{prefix}_AUTH_URL")).is_err()
            && std::env::var(format!("{prefix}_TOKEN_URL")).is_err()
            && std::env::var(format!("{prefix}_CLIENT_ID")).is_err()
        {
            (
                Url::parse("https://auth.openai.com/oauth/authorize")?,
                Url::parse("https://auth.openai.com/oauth/token")?,
                "app_EMoamEEZ73f0CkXaXp7hrann".to_string(),
            )
        } else {
            (
                Url::parse(&required("AUTH_URL")?).context("OAuth authorization URL is invalid")?,
                Url::parse(&required("TOKEN_URL")?).context("OAuth token URL is invalid")?,
                required("CLIENT_ID")?,
            )
        };
        validate_oauth_endpoint(&authorization_url)?;
        validate_oauth_endpoint(&token_url)?;
        let scopes = std::env::var(format!("{prefix}_SCOPES"))
            .unwrap_or_else(|_| {
                if provider == AuthProvider::ChatGpt {
                    "openid profile email offline_access api.connectors.read api.connectors.invoke"
                        .into()
                } else {
                    String::new()
                }
            })
            .split_whitespace()
            .filter(|scope| !scope.is_empty())
            .map(str::to_string)
            .collect();
        let mut extra_authorization_params = if provider == AuthProvider::ChatGpt {
            HashMap::from([
                ("id_token_add_organizations".into(), "true".into()),
                ("codex_cli_simplified_flow".into(), "true".into()),
                ("originator".into(), "codex_cli_rs".into()),
            ])
        } else {
            HashMap::new()
        };
        if let Some(configured) = std::env::var(format!("{prefix}_EXTRA_PARAMS"))
            .ok()
            .map(|raw| {
                serde_json::from_str::<HashMap<String, String>>(&raw)
                    .context("OAuth extra authorization parameters are invalid")
            })
            .transpose()?
        {
            extra_authorization_params.extend(configured);
        }
        Ok(Self {
            provider,
            authorization_url,
            token_url,
            client_id,
            scopes,
            extra_authorization_params,
        })
    }

    pub async fn from_openid_configuration(
        provider: AuthProvider,
        discovery_url: &str,
        client_id: impl Into<String>,
        scopes: Vec<String>,
    ) -> Result<Self> {
        let discovery_url = Url::parse(discovery_url).context("OIDC discovery URL is invalid")?;
        validate_oauth_endpoint(&discovery_url)?;
        let document: OpenIdConfiguration = Client::new()
            .get(discovery_url)
            .send()
            .await
            .context("OIDC discovery request failed")?
            .error_for_status()
            .context("OIDC discovery returned an error")?
            .json()
            .await
            .context("OIDC discovery response was invalid")?;
        let authorization_url = Url::parse(&document.authorization_endpoint)
            .context("authorization endpoint invalid")?;
        let token_url = Url::parse(&document.token_endpoint).context("token endpoint invalid")?;
        validate_oauth_endpoint(&authorization_url)?;
        validate_oauth_endpoint(&token_url)?;
        Ok(Self {
            provider,
            authorization_url,
            token_url,
            client_id: client_id.into(),
            scopes,
            extra_authorization_params: HashMap::new(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct OpenIdConfiguration {
    authorization_endpoint: String,
    token_endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PkceAuthorization {
    pub provider: AuthProvider,
    pub state: String,
    pub code_verifier: String,
    pub redirect_uri: String,
    pub authorization_url: Url,
}

pub fn begin_pkce(
    config: &OAuthProviderConfig,
    redirect_uri: impl Into<String>,
) -> Result<PkceAuthorization> {
    let redirect_uri = redirect_uri.into();
    validate_redirect_uri(&redirect_uri)?;
    let state = random_urlsafe(32);
    let code_verifier = random_urlsafe(64);
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(code_verifier.as_bytes()));
    let mut url = config.authorization_url.clone();
    {
        let mut query = url.query_pairs_mut();
        query
            .append_pair("response_type", "code")
            .append_pair("client_id", &config.client_id)
            .append_pair("redirect_uri", &redirect_uri)
            .append_pair("code_challenge", &challenge)
            .append_pair("code_challenge_method", "S256")
            .append_pair("state", &state);
        if !config.scopes.is_empty() {
            query.append_pair("scope", &config.scopes.join(" "));
        }
        for (key, value) in &config.extra_authorization_params {
            query.append_pair(key, value);
        }
    }
    Ok(PkceAuthorization {
        provider: config.provider,
        state,
        code_verifier,
        redirect_uri,
        authorization_url: url,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthToken {
    pub provider: AuthProvider,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub scope: Option<String>,
    pub expires_at: Option<u64>,
    #[serde(default)]
    pub raw: serde_json::Value,
}

impl OAuthToken {
    pub fn is_expired(&self, now: u64) -> bool {
        self.expires_at
            .is_some_and(|expires_at| now + 30 >= expires_at)
    }
}

/// One provider's CLI login is a short-lived browser flow completed by
/// polling the provider, rather than a localhost OAuth callback.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderOAuthFlow {
    pub provider: AuthProvider,
    pub base_url: Url,
    pub flow_id: String,
    pub poll_token: String,
    pub authorization_url: Url,
    pub expires_at: u64,
    pub poll_interval: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderPollStatus {
    Pending,
    Ready,
    Failed,
}

#[derive(Debug, Clone)]
pub struct ProviderPollResult {
    pub status: ProviderPollStatus,
    pub token: Option<OAuthToken>,
}

impl ProviderOAuthFlow {
    pub fn from_environment(provider: AuthProvider) -> Result<Self> {
        if provider != AuthProvider::Provider {
            bail!("Provider OAuth flow requires the matching provider");
        }
        let base_url = std::env::var("APP_PROVIDER_OAUTH_BASE_URL")
            .or_else(|_| std::env::var("VOLT_ZCODE_OAUTH_BASE_URL"))
            .unwrap_or_else(|_| "https://zcode.z.ai/api/v1".into());
        let base_url = Url::parse(&format!("{}/", base_url.trim_end_matches('/')))
            .context("Provider OAuth base URL is invalid")?;
        validate_oauth_endpoint(&base_url)?;
        Ok(Self {
            provider,
            base_url,
            flow_id: String::new(),
            poll_token: String::new(),
            authorization_url: Url::parse("https://zcode.z.ai").unwrap(),
            expires_at: 0,
            poll_interval: Duration::from_secs(2),
        })
    }

    pub async fn begin(client: &Client, provider_id: &str) -> Result<Self> {
        let provider = AuthProvider::Provider;
        let mut flow = Self::from_environment(provider)?;
        flow.poll_token = random_hex(32);
        let response = client
            .post(flow.base_url.join("oauth/cli/init")?)
            .bearer_auth(&flow.poll_token)
            .json(&json!({ "provider": provider_id }))
            .send()
            .await
            .context("Provider OAuth initialization failed")?
            .error_for_status()
            .context("Provider OAuth initialization returned an error")?;
        let envelope: ProviderEnvelope<ProviderInitData> = response
            .json()
            .await
            .context("Provider OAuth initialization response was invalid")?;
        if envelope.code != 0 {
            bail!(
                "Provider OAuth initialization failed: {}",
                envelope.msg.unwrap_or_else(|| envelope.code.to_string())
            );
        }
        let data = envelope
            .data
            .context("Provider OAuth init data was missing")?;
        let authorization_url =
            Url::parse(&data.authorize_url).context("Provider authorization URL is invalid")?;
        validate_oauth_endpoint(&authorization_url)?;
        if data.flow_id.trim().is_empty() || data.expires_at == 0 {
            bail!("Provider OAuth init data was incomplete");
        }
        flow.flow_id = data.flow_id;
        flow.authorization_url = authorization_url;
        flow.expires_at = data.expires_at;
        flow.poll_interval = Duration::from_secs(data.poll_interval_sec.max(1));
        Ok(flow)
    }

    pub async fn poll(&self, client: &Client, provider_id: &str) -> Result<ProviderPollResult> {
        if self.flow_id.is_empty() || self.poll_token.is_empty() {
            bail!("Provider OAuth flow is not initialized");
        }
        let endpoint = self.poll_endpoint()?;
        let response = client
            .get(endpoint)
            .bearer_auth(&self.poll_token)
            .send()
            .await
            .context("Provider OAuth polling failed")?
            .error_for_status()
            .context("Provider OAuth polling returned an error")?;
        let envelope: ProviderEnvelope<serde_json::Value> = response
            .json()
            .await
            .context("Provider OAuth polling response was invalid")?;
        if envelope.code != 0 {
            bail!(
                "Provider OAuth polling failed: {}",
                envelope.msg.unwrap_or_else(|| envelope.code.to_string())
            );
        }
        let data = envelope
            .data
            .context("Provider OAuth poll data was missing")?;
        match data.get("status").and_then(Value::as_str) {
            Some("pending") => Ok(ProviderPollResult {
                status: ProviderPollStatus::Pending,
                token: None,
            }),
            Some("failed") => Ok(ProviderPollResult {
                status: ProviderPollStatus::Failed,
                token: None,
            }),
            Some("ready") => {
                let token = data
                    .get(provider_id)
                    .and_then(Value::as_object)
                    .and_then(|provider| {
                        provider
                            .get("access_token")
                            .or_else(|| provider.get("accessToken"))
                            .and_then(Value::as_str)
                    })
                    .context("Provider OAuth ready response did not contain an access token")?;
                let refresh_token =
                    data.get(provider_id)
                        .and_then(Value::as_object)
                        .and_then(|provider| {
                            provider
                                .get("refresh_token")
                                .or_else(|| provider.get("refreshToken"))
                                .and_then(Value::as_str)
                                .map(str::to_string)
                        });
                Ok(ProviderPollResult {
                    status: ProviderPollStatus::Ready,
                    token: Some(OAuthToken {
                        provider: AuthProvider::Provider,
                        access_token: token.to_string(),
                        refresh_token,
                        token_type: "Bearer".into(),
                        scope: None,
                        expires_at: None,
                        raw: data,
                    }),
                })
            }
            Some(status) => bail!("Provider OAuth returned an unknown status: {status}"),
            None => bail!("Provider OAuth poll status was missing"),
        }
    }

    pub async fn wait_ready(&self, client: &Client, provider_id: &str) -> Result<OAuthToken> {
        let deadline = UNIX_EPOCH + Duration::from_secs(self.expires_at);
        loop {
            if SystemTime::now() >= deadline {
                bail!("Provider OAuth authorization expired");
            }
            match self.poll(client, provider_id).await? {
                ProviderPollResult {
                    status: ProviderPollStatus::Pending,
                    ..
                } => tokio::time::sleep(self.poll_interval).await,
                ProviderPollResult {
                    status: ProviderPollStatus::Failed,
                    ..
                } => bail!("Provider OAuth authorization failed"),
                ProviderPollResult {
                    status: ProviderPollStatus::Ready,
                    token: Some(token),
                } => return Ok(token),
                ProviderPollResult {
                    status: ProviderPollStatus::Ready,
                    token: None,
                } => bail!("Provider OAuth ready response did not contain a token"),
            }
        }
    }

    fn poll_endpoint(&self) -> Result<Url> {
        let mut endpoint = self.base_url.join("oauth/cli/poll")?;
        endpoint
            .path_segments_mut()
            .map_err(|_| anyhow::anyhow!("Provider OAuth base URL cannot accept path segments"))?
            .push(&self.flow_id);
        Ok(endpoint)
    }
}

#[derive(Debug, Deserialize)]
struct ProviderEnvelope<T> {
    code: i64,
    data: Option<T>,
    msg: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProviderInitData {
    authorize_url: String,
    expires_at: u64,
    flow_id: String,
    poll_interval_sec: u64,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    #[serde(default = "default_token_type")]
    token_type: String,
    refresh_token: Option<String>,
    expires_in: Option<u64>,
    scope: Option<String>,
    #[serde(flatten)]
    raw: serde_json::Map<String, serde_json::Value>,
}

pub async fn exchange_code(
    client: &Client,
    config: &OAuthProviderConfig,
    flow: &PkceAuthorization,
    code: &str,
) -> Result<OAuthToken> {
    if code.trim().is_empty() {
        bail!("OAuth authorization code is empty");
    }
    let response = client
        .post(config.token_url.clone())
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", flow.redirect_uri.as_str()),
            ("client_id", config.client_id.as_str()),
            ("code_verifier", flow.code_verifier.as_str()),
        ])
        .send()
        .await
        .context("OAuth token exchange failed")?
        .error_for_status()
        .context("OAuth token endpoint returned an error")?;
    token_from_response(config.provider, response).await
}

pub async fn refresh_access_token(
    client: &Client,
    config: &OAuthProviderConfig,
    refresh_token: &str,
) -> Result<OAuthToken> {
    if refresh_token.trim().is_empty() {
        bail!("OAuth refresh token is empty");
    }
    let response = client
        .post(config.token_url.clone())
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", config.client_id.as_str()),
        ])
        .send()
        .await
        .context("OAuth token refresh failed")?
        .error_for_status()
        .context("OAuth refresh endpoint returned an error")?;
    token_from_response(config.provider, response).await
}

async fn token_from_response(
    provider: AuthProvider,
    response: reqwest::Response,
) -> Result<OAuthToken> {
    let body: TokenResponse = response
        .json()
        .await
        .context("OAuth token response was invalid")?;
    if body.access_token.trim().is_empty() {
        bail!("OAuth token response did not contain an access token");
    }
    let expires_at = body
        .expires_in
        .map(|seconds| unix_now().saturating_add(seconds));
    Ok(OAuthToken {
        provider,
        access_token: body.access_token,
        refresh_token: body.refresh_token,
        token_type: body.token_type,
        scope: body.scope,
        expires_at,
        raw: serde_json::Value::Object(body.raw),
    })
}

#[derive(Debug)]
pub struct OAuthCallbackListener {
    listener: StdTcpListener,
    redirect_uri: String,
}

impl OAuthCallbackListener {
    pub fn bind_sync() -> Result<Self> {
        let listener = StdTcpListener::bind(("127.0.0.1", 0))
            .context("failed to bind local OAuth callback")?;
        let address = listener.local_addr()?;
        Ok(Self {
            listener,
            redirect_uri: format!("http://{address}/oauth/callback"),
        })
    }

    pub async fn bind() -> Result<Self> {
        Self::bind_sync()
    }

    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    pub async fn wait(self, expected_state: &str) -> Result<String> {
        let expected_state = expected_state.to_string();
        tokio::task::spawn_blocking(move || {
            let (mut stream, _address) = self.listener.accept()?;
            let mut request = vec![0u8; 16 * 1024];
            let length = stream.read(&mut request)?;
            let first_line = std::str::from_utf8(&request[..length])
                .context("OAuth callback was not valid UTF-8")?
                .lines()
                .next()
                .context("OAuth callback request was empty")?;
            let target = first_line
                .strip_prefix("GET ")
                .and_then(|line| line.split_whitespace().next())
                .context("OAuth callback was not an HTTP GET")?;
            let callback = Url::parse(&format!("http://localhost{target}"))
                .context("OAuth callback URL was invalid")?;
            let state = callback
                .query_pairs()
                .find(|(key, _)| key == "state")
                .map(|(_, value)| value.into_owned())
                .unwrap_or_default();
            if state != expected_state {
                bail!("OAuth callback state did not match");
            }
            if let Some(error) = callback
                .query_pairs()
                .find(|(key, _)| key == "error")
                .map(|(_, value)| value.into_owned())
            {
                bail!("OAuth authorization failed: {error}");
            }
            let code = callback
                .query_pairs()
                .find(|(key, _)| key == "code" || key == "authCode")
                .map(|(_, value)| value.into_owned())
                .context("OAuth callback did not contain a code")?;
            let response =
                b"HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nConnection: close\r\n\r\nLogin complete. You can close this window.";
            let _ = stream.write_all(response);
            Ok(code)
        })
        .await
        .context("OAuth callback worker failed")?
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialRecord {
    pub provider: AuthProvider,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_at: Option<u64>,
}

impl From<OAuthToken> for CredentialRecord {
    fn from(token: OAuthToken) -> Self {
        Self {
            provider: token.provider,
            access_token: token.access_token,
            refresh_token: token.refresh_token,
            token_type: token.token_type,
            expires_at: token.expires_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CredentialFile {
    version: u32,
    records: HashMap<AuthProvider, CredentialRecord>,
}

#[derive(Debug, Clone)]
pub struct CredentialStore {
    path: PathBuf,
}

impl CredentialStore {
    pub fn default_path() -> PathBuf {
        std::env::var_os("VOLT_CREDENTIALS_FILE")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME")
                    .map(PathBuf::from)
                    .map(|home| home.join(".local/share/volt-desktop/credentials.json"))
            })
            .unwrap_or_else(|| PathBuf::from("credentials.json"))
    }

    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn load(&self) -> Result<HashMap<AuthProvider, CredentialRecord>> {
        match fs::read(&self.path) {
            Ok(bytes) => Ok(serde_json::from_slice::<CredentialFile>(&bytes)?.records),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(HashMap::new()),
            Err(error) => Err(error.into()),
        }
    }

    pub fn save(&self, record: CredentialRecord) -> Result<()> {
        let parent = self
            .path
            .parent()
            .context("credential store has no parent directory")?;
        fs::create_dir_all(parent)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(parent, fs::Permissions::from_mode(0o700))?;
        }
        let mut records = self.load()?;
        records.insert(record.provider, record);
        let payload = serde_json::to_vec_pretty(&CredentialFile {
            version: 1,
            records,
        })?;
        let temp = parent.join(format!(".credentials-{}.tmp", random_urlsafe(12)));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(&temp)?;
        file.write_all(&payload)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&temp, &self.path)?;
        Ok(())
    }

    pub fn remove(&self, provider: AuthProvider) -> Result<()> {
        let mut records = self.load()?;
        records.remove(&provider);
        if records.is_empty() {
            match fs::remove_file(&self.path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
            return Ok(());
        }
        let record = records
            .values()
            .next()
            .cloned()
            .context("credential store became inconsistent")?;
        let _ = record;
        let payload = serde_json::to_vec_pretty(&CredentialFile {
            version: 1,
            records,
        })?;
        fs::write(&self.path, payload)?;
        Ok(())
    }
}

fn validate_oauth_endpoint(url: &Url) -> Result<()> {
    if url.scheme() != "https" && url.host_str() != Some("127.0.0.1") {
        bail!("OAuth endpoints must use HTTPS");
    }
    if url.username() != "" || url.password().is_some() || url.fragment().is_some() {
        bail!("OAuth endpoints must not contain credentials or fragments");
    }
    Ok(())
}

fn validate_redirect_uri(uri: &str) -> Result<()> {
    let parsed = Url::parse(uri).context("OAuth redirect URI is invalid")?;
    if parsed.scheme() != "http"
        || parsed.host_str() != Some("127.0.0.1")
        || parsed.path() != "/oauth/callback"
    {
        bail!("OAuth redirect URI must be a local 127.0.0.1 callback");
    }
    Ok(())
}

fn random_urlsafe(bytes: usize) -> String {
    let mut buffer = vec![0u8; bytes];
    rand::thread_rng().fill_bytes(&mut buffer);
    URL_SAFE_NO_PAD.encode(buffer)
}

fn random_hex(bytes: usize) -> String {
    let mut buffer = vec![0u8; bytes];
    rand::thread_rng().fill_bytes(&mut buffer);
    buffer.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn random_id_for_tests() -> String {
    random_urlsafe(12)
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_secs()
}

fn default_token_type() -> String {
    "Bearer".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pkce_url_contains_state_and_challenge() {
        let config = OAuthProviderConfig {
            provider: AuthProvider::Custom,
            authorization_url: Url::parse("https://example.com/authorize").unwrap(),
            token_url: Url::parse("https://example.com/token").unwrap(),
            client_id: "client".into(),
            scopes: vec!["openid".into(), "profile".into()],
            extra_authorization_params: HashMap::new(),
        };
        let flow = begin_pkce(&config, "http://127.0.0.1:4567/oauth/callback").unwrap();
        let query: HashMap<_, _> = flow.authorization_url.query_pairs().into_owned().collect();
        assert_eq!(query.get("client_id"), Some(&"client".to_string()));
        assert_eq!(
            query.get("code_challenge_method"),
            Some(&"S256".to_string())
        );
        assert_eq!(query.get("state"), Some(&flow.state));
        assert!(query
            .get("code_challenge")
            .is_some_and(|value| !value.is_empty()));
    }

    #[test]
    fn credential_store_does_not_use_desktop_state_file() {
        let temp = tempfile::tempdir().unwrap();
        let store = CredentialStore::new(temp.path().join("credentials.json"));
        store
            .save(CredentialRecord {
                provider: AuthProvider::DeepSeek,
                access_token: "access".into(),
                refresh_token: Some("refresh".into()),
                token_type: "Bearer".into(),
                expires_at: None,
            })
            .unwrap();
        let records = store.load().unwrap();
        assert_eq!(records[&AuthProvider::DeepSeek].access_token, "access");
        assert!(!temp.path().join("desktop.json").exists());
    }

    #[test]
    fn provider_names_are_explicit_and_case_insensitive() {
        assert_eq!(AuthProvider::parse("ChatGPT OAuth"), None);
        assert_eq!(
            AuthProvider::parse("chatgpt_oauth"),
            Some(AuthProvider::ChatGpt)
        );
        assert_eq!(
            AuthProvider::parse("deepseek"),
            Some(AuthProvider::DeepSeek)
        );
        assert_eq!(AuthProvider::parse("zcode"), Some(AuthProvider::Provider));
        assert_eq!(AuthProvider::parse("pi"), Some(AuthProvider::Pi));
    }

    #[test]
    fn account_oauth_keeps_the_wire_id_but_is_not_a_model_credential() {
        let provider = AuthProvider::Provider;
        assert_eq!(provider.as_str(), "zcode");
        assert_eq!(serde_json::to_value(provider).unwrap(), json!("zcode"));
        assert!(!provider.supplies_model_credential());
        assert!(AuthProvider::Custom.supplies_model_credential());
    }

    #[test]
    fn account_oauth_poll_encodes_the_flow_as_one_path_segment() {
        let flow = ProviderOAuthFlow {
            provider: AuthProvider::Provider,
            base_url: Url::parse("https://example.org/api/v1/").unwrap(),
            flow_id: "flow/id".into(),
            poll_token: String::new(),
            authorization_url: Url::parse("https://example.org/").unwrap(),
            expires_at: 0,
            poll_interval: Duration::from_secs(2),
        };
        assert_eq!(
            flow.poll_endpoint().unwrap().path(),
            "/api/v1/oauth/cli/poll/flow%2Fid"
        );
    }

    #[test]
    fn chatgpt_uses_the_public_codex_oauth_defaults() {
        let config = OAuthProviderConfig::from_environment(AuthProvider::ChatGpt).unwrap();
        assert_eq!(
            config.authorization_url.as_str(),
            "https://auth.openai.com/oauth/authorize"
        );
        assert_eq!(
            config.token_url.as_str(),
            "https://auth.openai.com/oauth/token"
        );
        assert_eq!(config.client_id, "app_EMoamEEZ73f0CkXaXp7hrann");
        assert!(config.scopes.iter().any(|scope| scope == "offline_access"));
    }
}
