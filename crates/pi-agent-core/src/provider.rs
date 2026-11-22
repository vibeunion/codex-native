mod stream;
#[cfg(test)]
mod tests;

use anyhow::{bail, Context, Result};
use futures::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use std::time::Duration;
use tokio::sync::mpsc;

use crate::types::{AgentEvent, Message, ModelConfig, Role};
use stream::ResponseStream;

#[derive(Clone, Copy)]
struct RequestTimeouts {
    total: Duration,
    headers: Duration,
    idle: Duration,
}

impl Default for RequestTimeouts {
    fn default() -> Self {
        Self {
            total: Duration::from_secs(300),
            headers: Duration::from_secs(30),
            idle: Duration::from_secs(60),
        }
    }
}

#[derive(Clone)]
pub struct LlmClient {
    client: reqwest::Client,
    timeouts: RequestTimeouts,
}

impl LlmClient {
    pub fn new() -> Self {
        Self::with_timeouts(RequestTimeouts::default())
    }

    fn with_timeouts(timeouts: RequestTimeouts) -> Self {
        Self {
            client: reqwest::Client::builder()
                .connect_timeout(Duration::from_secs(10))
                .build()
                .expect("failed to initialize the LLM HTTP client"),
            timeouts,
        }
    }

    pub async fn stream_chat(
        &self,
        config: &ModelConfig,
        messages: &[Message],
        tools: Option<Value>,
        tx: mpsc::Sender<AgentEvent>,
    ) -> Result<Message> {
        let completed = self.stream_chat_inner(config, messages, tools, tx).await;
        completed.map_err(|error| {
            anyhow::anyhow!(redact_credential(
                &format!("{error:#}"),
                config.api_key.as_deref()
            ))
        })
    }

    async fn stream_chat_inner(
        &self,
        config: &ModelConfig,
        messages: &[Message],
        tools: Option<Value>,
        tx: mpsc::Sender<AgentEvent>,
    ) -> Result<Message> {
        let deadline = tokio::time::Instant::now() + self.timeouts.total;
        let request = self
            .client
            .post(request_url(config.api_base.as_deref())?)
            .headers(request_headers(config.api_key.as_deref())?)
            .json(&request_body(config, messages, tools)?);
        let header_deadline = deadline.min(tokio::time::Instant::now() + self.timeouts.headers);
        let response = tokio::time::timeout_at(header_deadline, request.send())
            .await
            .context("LLM response headers timed out")?
            .map_err(reqwest::Error::without_url)
            .context("Failed to receive LLM response")?;

        if !response.status().is_success() {
            let message = self.http_error(response, deadline).await;
            let message = redact_credential(&message, config.api_key.as_deref());
            let _ = tx.try_send(AgentEvent::Error(message.clone()));
            bail!(message);
        }
        tokio::time::timeout_at(deadline, self.read_stream(response, tx))
            .await
            .with_context(|| {
                format!(
                    "LLM request exceeded its {} second deadline; output may be incomplete",
                    self.timeouts.total.as_secs()
                )
            })?
    }

    async fn http_error(
        &self,
        response: reqwest::Response,
        deadline: tokio::time::Instant,
    ) -> String {
        let status = response.status();
        // Diagnostic reads must never replace an already known HTTP failure with a timeout.
        let details = match tokio::time::timeout_at(deadline, self.error_details(response)).await {
            Ok(Ok(details)) => details,
            Ok(Err(error)) => format!("error details unavailable: {error:#}"),
            Err(_) => "error details exceeded the request deadline".to_owned(),
        };
        format!("LLM API returned error {status}: {details}")
    }

    async fn read_stream(
        &self,
        response: reqwest::Response,
        tx: mpsc::Sender<AgentEvent>,
    ) -> Result<Message> {
        let mut bytes = response.bytes_stream();
        let mut decoded = ResponseStream::default();
        while let Some(chunk) = tokio::time::timeout(self.timeouts.idle, bytes.next())
            .await
            .context("LLM stream stalled; output may be incomplete")?
        {
            let chunk = chunk
                .map_err(reqwest::Error::without_url)
                .context("Failed to read LLM stream")?;
            let parsed = decoded.push(&chunk);
            for event in decoded.take_events() {
                let _ = tx.send(event).await;
            }
            parsed?;
            if decoded.is_done() {
                break;
            }
        }
        let (message, total_tokens) = decoded.finish()?;
        let _ = tx.send(AgentEvent::Completed { total_tokens }).await;
        Ok(message)
    }

    async fn error_details(&self, response: reqwest::Response) -> Result<String> {
        const MAX_ERROR_BYTES: usize = 8 * 1024;
        let mut bytes = response.bytes_stream();
        let mut body = Vec::new();
        while let Some(chunk) = tokio::time::timeout(self.timeouts.idle, bytes.next())
            .await
            .context("error response stalled")?
        {
            let chunk = chunk.map_err(reqwest::Error::without_url)?;
            if body.len().saturating_add(chunk.len()) > MAX_ERROR_BYTES {
                bail!("error response exceeds 8 KiB");
            }
            body.extend_from_slice(&chunk);
        }
        Ok(String::from_utf8_lossy(&body).into_owned())
    }
}

impl Default for LlmClient {
    fn default() -> Self {
        Self::new()
    }
}

fn request_url(base: Option<&str>) -> Result<reqwest::Url> {
    let mut url = reqwest::Url::parse(base.unwrap_or("https://api.openai.com/v1"))
        .context("Invalid LLM endpoint")?;
    if !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        bail!("LLM endpoint must be an HTTP(S) URL without credentials, query or fragment");
    }
    let path = url.path().trim_end_matches('/').to_owned();
    if !path.ends_with("/chat/completions") {
        url.set_path(&format!("{path}/chat/completions"));
    } else {
        url.set_path(&path);
    }
    Ok(url)
}

fn request_headers(key: Option<&str>) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    if let Some(key) = key {
        if key.trim().is_empty() {
            bail!("API credential is empty");
        }
        let mut authorization = HeaderValue::from_str(&format!("Bearer {}", key.trim()))
            .context("Invalid API credential header")?;
        authorization.set_sensitive(true);
        headers.insert(AUTHORIZATION, authorization);
    }
    Ok(headers)
}

fn request_body(config: &ModelConfig, messages: &[Message], tools: Option<Value>) -> Result<Value> {
    if config.model.trim().is_empty()
        || !config.temperature.is_finite()
        || config.max_tokens == Some(0)
    {
        bail!("Invalid model, temperature or token limit");
    }
    let mut body = json!({
        "model": config.model,
        "messages": messages.iter().map(api_message).collect::<Vec<_>>(),
        "stream": true,
        "temperature": config.temperature,
    });
    if let Some(max_tokens) = config.max_tokens {
        body["max_tokens"] = json!(max_tokens);
    }
    if let Some(tools) =
        tools.filter(|tools| tools.as_array().is_some_and(|tools| !tools.is_empty()))
    {
        body["tools"] = tools;
    }
    Ok(body)
}

fn api_message(message: &Message) -> Value {
    let role = match message.role {
        Role::System => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    };
    let mut encoded = json!({"role": role, "content": message.content});
    if let Some(calls) = &message.tool_calls {
        encoded["tool_calls"] = json!(calls
            .iter()
            .map(|call| json!({
                "id": call.id,
                "type": "function",
                "function": {"name": call.name, "arguments": call.arguments.to_string()},
            }))
            .collect::<Vec<_>>());
    }
    if let Some(receipt) = &message.tool_result {
        encoded["tool_call_id"] = json!(receipt.tool_call_id);
    }
    encoded
}

fn redact_credential(text: &str, key: Option<&str>) -> String {
    match key.map(str::trim).filter(|key| !key.is_empty()) {
        Some(key) => text.replace(key, "[REDACTED_SECRET]"),
        None => text.to_owned(),
    }
}
