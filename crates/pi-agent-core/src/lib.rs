#[cfg(not(any(feature = "codex-harness", feature = "pi-native")))]
compile_error!("Enable at least one runtime feature: codex-harness or pi-native");

pub mod agent;
pub mod agent_config;
pub mod auth;
mod delegation;
#[cfg(feature = "codex-harness")]
pub mod harness;
pub mod identity;
pub mod provider;
pub mod runtime;
pub mod session;
pub mod skills;
pub mod tools;
pub mod types;
pub mod workflow;
pub mod workflow_control;

pub use agent::PiAgent;
#[cfg(feature = "codex-harness")]
pub use harness::{CodexHarnessServer, JsonRpcRequest, JsonRpcResponse};
pub use identity::AppIdentity;
pub use provider::LlmClient;
pub use runtime::{EngineSelection, PolicyProfile, RuntimeCapabilities};
pub use session::Session;
pub use skills::{CliExample, Skill, SkillCategory, SkillRegistry};
pub use tools::{Tool, ToolRegistry};
pub use types::{
    AgentEvent, EngineKind, Message, ModelConfig, Role, SessionSource, ToolCall, ToolResult,
};
