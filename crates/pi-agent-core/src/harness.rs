//! Lightweight Codex-style app-server harness.
//!
//! The transport is JSON-RPC over newline-delimited stdio. It intentionally
//! keeps protocol types separate from the Pi UI and exposes stable thread/turn
//! lifecycle events for desktop and IDE hosts.

use crate::{
    agent::PiAgent,
    runtime::{PolicyProfile, RuntimeCapabilities},
    session::Session,
    types::{EngineKind, Message},
    ModelConfig,
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufWriter},
    sync::{mpsc, Mutex},
    task::AbortHandle,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    #[serde(default)]
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: &'static str,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessEvent {
    pub thread_id: String,
    pub turn_id: String,
    pub event: crate::types::AgentEvent,
}

#[derive(Clone)]
pub struct CodexHarnessServer {
    workspace_dir: PathBuf,
    model_config: ModelConfig,
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    turns: Arc<Mutex<HashMap<String, AbortHandle>>>,
}

impl CodexHarnessServer {
    pub fn new(workspace_dir: impl Into<PathBuf>, model_config: ModelConfig) -> Self {
        let model_config = hydrate_credentials(model_config);
        Self {
            workspace_dir: workspace_dir.into(),
            model_config,
            sessions: Arc::new(Mutex::new(HashMap::new())),
            turns: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn handle_request<W>(
        &self,
        request: JsonRpcRequest,
        writer: Arc<Mutex<BufWriter<W>>>,
    ) -> JsonRpcResponse
    where
        W: AsyncWrite + Unpin + Send + 'static,
    {
        let id = request.id.clone();
        let result = match request.method.as_str() {
            "initialize" => self.initialize(),
            "thread/start" => self.thread_start(&request.params).await,
            "thread/read" => self.thread_read(&request.params).await,
            "thread/import" => self.thread_import(&request.params).await,
            "thread/export" => self.thread_export(&request.params).await,
            "session/fork" => self.session_fork(&request.params).await,
            "turn/start" => self.turn_start(&request.params, writer).await,
            "turn/interrupt" => self.turn_interrupt(&request.params).await,
            "auth/providers" => Ok(json!({
                "api_key": true,
                "oauth": ["chatgpt", "deepseek", "zcode", "pi", "custom"],
            })),
            "shutdown" => Ok(json!({"ok": true})),
            method => Err(anyhow::anyhow!("unknown method: {method}")),
        };
        match result {
            Ok(result) => JsonRpcResponse {
                jsonrpc: "2.0",
                id,
                result: Some(result),
                error: None,
            },
            Err(error) => JsonRpcResponse {
                jsonrpc: "2.0",
                id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32000,
                    message: error.to_string(),
                    data: None,
                }),
            },
        }
    }

    pub async fn run_stdio(self) -> Result<()> {
        let stdin = BufReader::new(tokio::io::stdin());
        let stdout = BufWriter::new(tokio::io::stdout());
        self.run_stream(stdin, stdout).await
    }

    pub async fn run_stream<R, W>(&self, reader: R, writer: W) -> Result<()>
    where
        R: tokio::io::AsyncRead + Unpin,
        W: AsyncWrite + Unpin + Send + 'static,
    {
        let mut lines = BufReader::new(reader).lines();
        let writer = Arc::new(Mutex::new(BufWriter::new(writer)));
        while let Some(line) = lines.next_line().await? {
            if line.trim().is_empty() {
                continue;
            }
            let request: JsonRpcRequest =
                serde_json::from_str(&line).context("invalid JSON-RPC request")?;
            let shutdown = request.method == "shutdown";
            let response = self.handle_request(request, writer.clone()).await;
            write_json(&writer, &response).await?;
            if shutdown {
                break;
            }
        }
        Ok(())
    }

    fn initialize(&self) -> Result<Value> {
        Ok(json!({
            "server": "codex-native",
            "protocol": 1,
            "capabilities": RuntimeCapabilities::for_engine(EngineKind::CodexCompatible),
        }))
    }

    async fn thread_start(&self, params: &Value) -> Result<Value> {
        let id = params
            .get("thread_id")
            .and_then(Value::as_str)
            .map(str::to_string)
            .unwrap_or_else(|| format!("thread-{}", crate::auth::random_id_for_tests()));
        let title = params
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or("New Codex session");
        let mut session = Session::new(id.clone(), title);
        session.set_engine(EngineKind::CodexCompatible);
        if let Some(codex_thread_id) = params.get("codex_thread_id").and_then(Value::as_str) {
            session.mark_codex_thread(codex_thread_id);
        }
        self.sessions.lock().await.insert(id, session.clone());
        Ok(serde_json::to_value(session)?)
    }

    async fn thread_read(&self, params: &Value) -> Result<Value> {
        let id = required_string(params, "thread_id")?;
        let sessions = self.sessions.lock().await;
        let session = sessions.get(id).cloned().context("thread not found")?;
        Ok(serde_json::to_value(session)?)
    }

    async fn thread_import(&self, params: &Value) -> Result<Value> {
        let raw = params
            .get("session")
            .or_else(|| params.get("raw"))
            .context("session payload is required")?;
        let session = if let Some(raw) = raw.as_str() {
            Session::import_codex_json(raw)?
        } else {
            Session::from_codex_value(raw)?
        };
        let id = session.id.clone();
        self.sessions.lock().await.insert(id, session.clone());
        Ok(serde_json::to_value(session)?)
    }

    async fn thread_export(&self, params: &Value) -> Result<Value> {
        let id = required_string(params, "thread_id")?;
        let sessions = self.sessions.lock().await;
        let session = sessions.get(id).cloned().context("thread not found")?;
        Ok(session.export_codex_value())
    }

    async fn session_fork(&self, params: &Value) -> Result<Value> {
        let parent_id = required_string(params, "thread_id")?;
        let engine = match params.get("engine").and_then(Value::as_str) {
            Some("pi_native") | Some("pi") => EngineKind::PiNative,
            _ => EngineKind::CodexCompatible,
        };
        if !engine.is_compiled() {
            anyhow::bail!("engine is not enabled in this build");
        }
        let mut sessions = self.sessions.lock().await;
        let parent = sessions
            .get(parent_id)
            .cloned()
            .context("thread not found")?;
        let id = params
            .get("new_thread_id")
            .and_then(Value::as_str)
            .map(str::to_string)
            .unwrap_or_else(|| format!("fork-{}", crate::auth::random_id_for_tests()));
        let fork = parent.fork(id.clone(), engine);
        sessions.insert(id, fork.clone());
        Ok(serde_json::to_value(fork)?)
    }

    async fn turn_start<W>(&self, params: &Value, writer: Arc<Mutex<BufWriter<W>>>) -> Result<Value>
    where
        W: AsyncWrite + Unpin + Send + 'static,
    {
        let thread_id = required_string(params, "thread_id")?.to_owned();
        let prompt = params
            .get("input")
            .or_else(|| params.get("prompt"))
            .and_then(Value::as_str)
            .context("turn input is required")?
            .to_owned();
        let turn_id = params
            .get("turn_id")
            .and_then(Value::as_str)
            .map(str::to_string)
            .unwrap_or_else(|| format!("turn-{}", crate::auth::random_id_for_tests()));
        let (mut messages, engine) = {
            let sessions = self.sessions.lock().await;
            let session = sessions
                .get(&thread_id)
                .cloned()
                .context("thread not found")?;
            (session.messages, session.engine)
        };
        messages.push(Message::user(prompt));
        let workspace = self.workspace_dir.clone();
        let model_config = self.model_config.clone();
        let sessions = self.sessions.clone();
        let turns = self.turns.clone();
        let thread_for_task = thread_id.clone();
        let turn_for_task = turn_id.clone();
        let (event_tx, mut event_rx) = mpsc::channel(128);
        let worker = tokio::spawn(async move {
            let profile = match engine {
                EngineKind::CodexCompatible => PolicyProfile::CodexStrict,
                EngineKind::PiNative => PolicyProfile::PiBalanced,
            };
            let agent = PiAgent::with_profile(workspace, model_config, profile);
            let task = tokio::spawn(async move {
                let mut messages = messages;
                let result = agent.run_turn(&mut messages, event_tx).await;
                (result, messages)
            });
            let mut task = Box::pin(task);
            let mut outcome = None;
            loop {
                tokio::select! {
                    Some(event) = event_rx.recv() => {
                        let notification = json!({
                            "jsonrpc": "2.0",
                            "method": "turn/event",
                            "params": HarnessEvent {
                                thread_id: thread_for_task.clone(),
                                turn_id: turn_for_task.clone(),
                                event,
                            },
                        });
                        let _ = write_json(&writer, &notification).await;
                    }
                    result = &mut task, if outcome.is_none() => {
                        outcome = Some(result.context("turn task failed")?);
                    }
                    else => break,
                }
                if outcome.is_some() && event_rx.is_empty() {
                    break;
                }
            }
            let (result, messages) = outcome.context("turn did not produce a result")?;
            let mut sessions = sessions.lock().await;
            if let Some(session) = sessions.get_mut(&thread_for_task) {
                session.messages = messages
                    .into_iter()
                    .filter(|message| message.role != crate::types::Role::System)
                    .collect();
                session.updated_at = current_time_ms();
            }
            let completed = json!({
                "jsonrpc": "2.0",
                "method": "turn/completed",
                "params": {
                    "thread_id": thread_for_task,
                    "turn_id": turn_for_task,
                    "ok": result.is_ok(),
                },
            });
            let _ = write_json(&writer, &completed).await;
            turns.lock().await.remove(&turn_for_task);
            Ok::<(), anyhow::Error>(())
        });
        self.turns
            .lock()
            .await
            .insert(turn_id.clone(), worker.abort_handle());
        Ok(json!({
            "thread_id": thread_id,
            "turn_id": turn_id,
            "status": "running",
        }))
    }

    async fn turn_interrupt(&self, params: &Value) -> Result<Value> {
        let turn_id = required_string(params, "turn_id")?;
        let handle = self.turns.lock().await.remove(turn_id);
        if let Some(handle) = handle {
            handle.abort();
            Ok(json!({"turn_id": turn_id, "status": "interrupted"}))
        } else {
            Ok(json!({"turn_id": turn_id, "status": "not_found"}))
        }
    }
}

fn hydrate_credentials(mut model_config: ModelConfig) -> ModelConfig {
    if model_config.api_key.is_some() {
        return model_config;
    }
    let Some(provider) = std::env::var("VOLT_AUTH_PROVIDER")
        .ok()
        .and_then(|value| crate::auth::AuthProvider::parse(&value))
    else {
        return model_config;
    };
    if let Ok(records) =
        crate::auth::CredentialStore::new(crate::auth::CredentialStore::default_path()).load()
    {
        if let Some(record) = records.get(&provider) {
            model_config.api_key = Some(record.access_token.clone());
        }
    }
    model_config
}

async fn write_json<W>(writer: &Arc<Mutex<BufWriter<W>>>, value: &impl Serialize) -> Result<()>
where
    W: AsyncWrite + Unpin + Send + 'static,
{
    let line = serde_json::to_string(value)?;
    let mut writer = writer.lock().await;
    writer.write_all(line.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;
    Ok(())
}

fn required_string<'a>(params: &'a Value, name: &str) -> Result<&'a str> {
    params
        .get(name)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .with_context(|| format!("{name} is required"))
}

fn current_time_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn initialize_and_thread_start_are_protocol_stable() {
        let server = CodexHarnessServer::new(".", ModelConfig::default());
        let output = Arc::new(Mutex::new(BufWriter::new(tokio::io::sink())));
        let init = server
            .handle_request(
                JsonRpcRequest {
                    jsonrpc: "2.0".into(),
                    id: Some(json!(1)),
                    method: "initialize".into(),
                    params: Value::Null,
                },
                output.clone(),
            )
            .await;
        assert!(init.error.is_none());
        let thread = server
            .handle_request(
                JsonRpcRequest {
                    jsonrpc: "2.0".into(),
                    id: Some(json!(2)),
                    method: "thread/start".into(),
                    params: json!({"title":"Imported"}),
                },
                output,
            )
            .await;
        assert_eq!(
            thread
                .result
                .as_ref()
                .and_then(|value| value.get("engine"))
                .and_then(Value::as_str),
            Some("codex_compatible")
        );
    }

    #[cfg(feature = "pi-native")]
    #[tokio::test]
    async fn imported_threads_can_be_exported_and_forked() {
        let server = CodexHarnessServer::new(".", ModelConfig::default());
        let output = Arc::new(Mutex::new(BufWriter::new(tokio::io::sink())));
        let imported = server
            .handle_request(
                JsonRpcRequest {
                    jsonrpc: "2.0".into(),
                    id: Some(json!(1)),
                    method: "thread/import".into(),
                    params: json!({
                        "session": {
                            "thread_id": "codex-import",
                            "items": [{"type":"user_message","content":"hello"}]
                        }
                    }),
                },
                output.clone(),
            )
            .await;
        assert!(imported.error.is_none());
        let exported = server
            .handle_request(
                JsonRpcRequest {
                    jsonrpc: "2.0".into(),
                    id: Some(json!(2)),
                    method: "thread/export".into(),
                    params: json!({"thread_id":"codex-import"}),
                },
                output.clone(),
            )
            .await;
        assert_eq!(
            exported
                .result
                .as_ref()
                .and_then(|value| value.get("thread_id"))
                .and_then(Value::as_str),
            Some("codex-import")
        );
        let fork = server
            .handle_request(
                JsonRpcRequest {
                    jsonrpc: "2.0".into(),
                    id: Some(json!(3)),
                    method: "session/fork".into(),
                    params: json!({"thread_id":"codex-import","engine":"pi_native"}),
                },
                output,
            )
            .await;
        assert_eq!(
            fork.result
                .as_ref()
                .and_then(|value| value.get("engine"))
                .and_then(Value::as_str),
            Some("pi_native")
        );
    }
}
