use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::Path;

use crate::types::{EngineKind, Message, SessionSource};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub messages: Vec<Message>,
    pub engine: EngineKind,
    pub source: SessionSource,
    pub codex_thread_id: Option<String>,
    pub parent_session_id: Option<String>,
    pub schema_version: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub raw_events: Vec<Value>,
}

impl Default for Session {
    fn default() -> Self {
        Self::new("local-session", "New session")
    }
}

impl Session {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;
        Self {
            id: id.into(),
            title: title.into(),
            created_at: now,
            updated_at: now,
            messages: Vec::new(),
            engine: EngineKind::default(),
            source: SessionSource::default(),
            codex_thread_id: None,
            parent_session_id: None,
            schema_version: 1,
            raw_events: Vec::new(),
        }
    }

    pub fn set_engine(&mut self, engine: EngineKind) {
        self.engine = engine;
        self.source = match engine {
            EngineKind::CodexCompatible => SessionSource::CodexHarness,
            EngineKind::PiNative => SessionSource::Pi,
        };
        self.updated_at = now_ms();
    }

    pub fn fork(&self, id: impl Into<String>, engine: EngineKind) -> Self {
        let mut fork = self.clone();
        fork.id = id.into();
        fork.parent_session_id = Some(self.id.clone());
        fork.set_engine(engine);
        fork.codex_thread_id = None;
        fork.raw_events = self.raw_events.clone();
        fork
    }

    pub fn mark_codex_thread(&mut self, thread_id: impl Into<String>) {
        self.set_engine(EngineKind::CodexCompatible);
        self.codex_thread_id = Some(thread_id.into());
        self.updated_at = now_ms();
    }

    pub fn import_codex_json(raw: &str) -> Result<Self> {
        let value: Value = match serde_json::from_str(raw) {
            Ok(value) => value,
            Err(error) => {
                let items = raw
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(serde_json::from_str::<Value>)
                    .collect::<std::result::Result<Vec<_>, _>>()
                    .map_err(|_| error)
                    .context("Codex session JSON is invalid")?;
                json!({ "items": items })
            }
        };
        Self::from_codex_value(&value)
    }

    pub fn from_codex_value(value: &Value) -> Result<Self> {
        let root = value.get("thread").unwrap_or(value);
        let id = first_string(root, &["thread_id", "id", "session_id"])
            .unwrap_or_else(|| format!("codex-{}", now_ms()));
        let title = first_string(root, &["title", "name"])
            .unwrap_or_else(|| "Imported Codex session".into());
        let mut session = Self::new(id.clone(), title);
        session.engine = EngineKind::CodexCompatible;
        session.source = SessionSource::CodexDesktop;
        session.codex_thread_id = Some(id);
        session.raw_events = root
            .get("events")
            .or_else(|| root.get("items"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let entries = root
            .get("messages")
            .or_else(|| root.get("items"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        for entry in entries {
            if let Some(message) = message_from_codex_value(&entry) {
                session.messages.push(message);
            }
        }
        if let Some(timestamp) = root
            .get("updated_at")
            .or_else(|| root.get("updatedAt"))
            .and_then(Value::as_i64)
        {
            session.updated_at = timestamp;
        }
        Ok(session)
    }

    pub fn export_codex_value(&self) -> Value {
        json!({
            "thread_id": self.codex_thread_id.as_ref().unwrap_or(&self.id),
            "title": self.title,
            "created_at": self.created_at,
            "updated_at": self.updated_at,
            "messages": self.messages,
            "events": self.raw_events,
            "engine": self.engine,
            "source": self.source,
            "parent_session_id": self.parent_session_id,
        })
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let session = serde_json::from_str(&content)?;
        Ok(session)
    }
}

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

fn first_string(value: &Value, names: &[&str]) -> Option<String> {
    names
        .iter()
        .find_map(|name| value.get(*name).and_then(Value::as_str).map(str::to_string))
}

fn message_from_codex_value(value: &Value) -> Option<Message> {
    let kind = value
        .get("type")
        .or_else(|| value.get("kind"))
        .and_then(Value::as_str)
        .unwrap_or_default();
    let role = value
        .get("role")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let content = value
        .get("content")
        .or_else(|| value.get("text"))
        .or_else(|| value.get("output"))
        .and_then(Value::as_str)
        .unwrap_or_default();
    match (kind, role) {
        ("tool_call", _) | ("function_call", _) => {
            let call = crate::types::ToolCall {
                id: first_string(value, &["id", "call_id"])
                    .unwrap_or_else(|| format!("call-{}", now_ms())),
                name: first_string(value, &["name", "tool"]).unwrap_or_else(|| "tool".into()),
                arguments: value.get("arguments").cloned().unwrap_or_else(|| json!({})),
            };
            let mut message = Message::assistant("");
            message.tool_calls = Some(vec![call]);
            Some(message)
        }
        ("tool_result", _) | ("function_result", _) => Some(Message::tool_result(
            first_string(value, &["tool_call_id", "call_id", "id"]).unwrap_or_default(),
            first_string(value, &["name", "tool"]).unwrap_or_else(|| "tool".into()),
            content,
            value
                .get("is_error")
                .and_then(Value::as_bool)
                .unwrap_or(false),
        )),
        (_, "system") => Some(Message::system(content)),
        (_, "user") | ("user_message", _) => Some(Message::user(content)),
        (_, "tool") => Some(Message::tool_result(
            first_string(value, &["tool_call_id", "call_id"]).unwrap_or_default(),
            first_string(value, &["name", "tool"]).unwrap_or_else(|| "tool".into()),
            content,
            value
                .get("is_error")
                .and_then(Value::as_bool)
                .unwrap_or(false),
        )),
        ("reasoning", _) => {
            let mut message = Message::assistant("");
            message.reasoning = Some(content.into());
            Some(message)
        }
        _ => Some(Message::assistant(content)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports_common_codex_messages_and_preserves_raw_items() {
        let session = Session::import_codex_json(
            r#"{
                "thread_id":"thread-1",
                "title":"Fix auth",
                "items":[
                    {"type":"user_message","content":"Implement login"},
                    {"type":"agent_message","content":"I will inspect the project."},
                    {"type":"tool_call","id":"call-1","name":"bash","arguments":{"command":"pwd"}},
                    {"type":"tool_result","tool_call_id":"call-1","name":"bash","output":"/workspace"}
                ]
            }"#,
        )
        .unwrap();
        assert_eq!(session.source, SessionSource::CodexDesktop);
        assert_eq!(session.engine, EngineKind::CodexCompatible);
        assert_eq!(session.messages.len(), 4);
        assert_eq!(session.raw_events.len(), 4);
        assert_eq!(session.export_codex_value()["thread_id"], "thread-1");
    }

    #[test]
    fn fork_keeps_lineage_and_changes_runtime() {
        let session = Session::new("source", "Source");
        let fork = session.fork("fork", EngineKind::PiNative);
        assert_eq!(fork.parent_session_id.as_deref(), Some("source"));
        assert_eq!(fork.engine, EngineKind::PiNative);
        assert_eq!(fork.source, SessionSource::Pi);
    }

    #[test]
    fn new_and_legacy_sessions_use_the_compiled_default_engine() {
        let session = Session::new("default", "Default");
        assert_eq!(session.engine, EngineKind::default());
        assert_eq!(
            session.source,
            match session.engine {
                EngineKind::CodexCompatible => SessionSource::CodexHarness,
                EngineKind::PiNative => SessionSource::Pi,
            }
        );

        let legacy: Session = serde_json::from_str(r#"{"id":"legacy","title":"Legacy"}"#).unwrap();
        assert_eq!(legacy.engine, EngineKind::default());
        assert_eq!(
            legacy.source,
            match legacy.engine {
                EngineKind::CodexCompatible => SessionSource::CodexHarness,
                EngineKind::PiNative => SessionSource::Pi,
            }
        );
    }

    #[test]
    fn imports_codex_jsonl_exports_as_a_compatible_session() {
        let session = Session::import_codex_json(
            r#"{"type":"user_message","content":"hello"}
{"type":"agent_message","content":"world"}"#,
        )
        .unwrap();
        assert_eq!(session.messages.len(), 2);
        assert_eq!(session.raw_events.len(), 2);
    }
}
