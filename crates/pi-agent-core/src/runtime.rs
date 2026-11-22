use crate::types::EngineKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyProfile {
    CodexStrict,
    PiBalanced,
    PiUnrestricted,
}

impl Default for PolicyProfile {
    fn default() -> Self {
        #[cfg(all(feature = "pi-native", not(feature = "codex-harness")))]
        {
            Self::PiBalanced
        }
        #[cfg(not(all(feature = "pi-native", not(feature = "codex-harness"))))]
        {
            Self::CodexStrict
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeCapabilities {
    pub engine: EngineKind,
    pub streaming: bool,
    pub tools: bool,
    pub approvals: bool,
    pub session_resume: bool,
    pub session_fork: bool,
    pub oauth: bool,
}

impl RuntimeCapabilities {
    pub fn for_engine(engine: EngineKind) -> Self {
        Self {
            engine,
            streaming: true,
            tools: true,
            approvals: matches!(engine, EngineKind::CodexCompatible),
            session_resume: true,
            session_fork: true,
            oauth: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineSelection {
    pub default_engine: EngineKind,
    pub default_policy: PolicyProfile,
    pub project_override: Option<EngineKind>,
    pub session_override: Option<EngineKind>,
}

impl Default for EngineSelection {
    fn default() -> Self {
        Self {
            default_engine: EngineKind::default(),
            default_policy: PolicyProfile::default(),
            project_override: None,
            session_override: None,
        }
    }
}

impl EngineSelection {
    pub fn effective_engine(&self) -> EngineKind {
        self.session_override
            .or(self.project_override)
            .unwrap_or(self.default_engine)
    }
}
