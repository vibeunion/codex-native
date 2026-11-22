use anyhow::{Context as _, Result};
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tracing::{error, info};

use crate::provider::LlmClient;
use crate::runtime::PolicyProfile;
use crate::skills::SkillRegistry;
use crate::tools::ToolRegistry;
use crate::types::{AgentEvent, Message, ModelConfig, ToolCall};
use crate::AppIdentity;

pub struct ToolCheckpoint {
    pub call: ToolCall,
    pub acknowledgement: oneshot::Sender<Result<()>>,
}

#[derive(Clone)]
pub struct PiAgent {
    pub workspace_dir: PathBuf,
    pub model_config: ModelConfig,
    pub tools: ToolRegistry,
    pub skills: SkillRegistry,
    pub client: LlmClient,
    pub max_iterations: usize,
    /// Upper bound for one delegated specialist, including provider and tools.
    pub delegation_timeout: Duration,
    pub workflow_controls: crate::workflow_control::WorkflowControls,
    /// Desktop hosts acknowledge durable history before a parent tool starts.
    pub tool_checkpoints: Option<mpsc::Sender<ToolCheckpoint>>,
    pub policy_profile: PolicyProfile,
}

impl PiAgent {
    pub fn new(workspace_dir: impl Into<PathBuf>, model_config: ModelConfig) -> Self {
        Self::with_profile(workspace_dir, model_config, PolicyProfile::PiBalanced)
    }

    pub fn with_profile(
        workspace_dir: impl Into<PathBuf>,
        model_config: ModelConfig,
        policy_profile: PolicyProfile,
    ) -> Self {
        let ws = workspace_dir.into();
        let tools = ToolRegistry::with_standard_tools(ws.clone());
        let skills = SkillRegistry::with_builtin_skills();
        Self {
            workspace_dir: ws,
            model_config,
            tools,
            skills,
            client: LlmClient::new(),
            max_iterations: 15,
            delegation_timeout: Duration::from_secs(90),
            workflow_controls: Default::default(),
            tool_checkpoints: None,
            policy_profile,
        }
    }

    pub fn default_system_prompt(&self) -> String {
        let identity = AppIdentity::from_environment();
        let skills_appendix = self.skills.build_system_prompt_appendix();
        let profile_rules = match self.policy_profile {
            PolicyProfile::CodexStrict => {
                "9. Follow structured turn/tool/approval semantics. Never hide a tool action from the host.\n\
                 10. Treat the workspace and approval policy supplied by the host as authoritative."
            }
            PolicyProfile::PiBalanced => {
                "9. Prefer direct progress while keeping destructive actions visible and reviewable."
            }
            PolicyProfile::PiUnrestricted => {
                "9. The user has selected a permissive native profile; still report every tool action and error."
            }
        };
        format!(
            "You are {} ({}), a high-performance, autonomous native AI coding and office assistant.\n\
            Current workspace directory: {}\n\n\
            You have access to native tools for inspecting files, editing code, searching patterns, executing shell commands, high-performance office table/document processing (`office`), and scientific computing (`calc`).\n\
            Rules:\n\
            1. Analyze the context before editing. Use read_file to inspect target files.\n\
            2. When modifying files, prefer edit_file for surgical edits, or write_file for new files.\n\
            3. Run shell commands using bash when necessary to build, test, or verify your changes.\n\
            4. For table queries, CSV/TSV processing, Markdown tables, docx extraction, slide generation, and reports, use the native `office` tool.\n\
            5. For mathematical expressions, statistical analysis, linear regression, matrix operations, loan PMT, CAGR, unit conversions, and dates, use the native `calc` tool.\n\
            6. Keep responses concise, accurate, and focused on the solution.\n\
            7. Proceed step-by-step until the task is completely finished.\n\
            8. For rich interactive interfaces, approval cards, forms, metrics, charts, or HTML artifacts, output structured ```json-ui code blocks (or <agent-embed>). The desktop host renders them with native GPU acceleration.\n\
            {}\n\
            {}",
            identity.agent_name,
            identity.brand_name,
            self.workspace_dir.display(),
            profile_rules,
            skills_appendix
        )
    }

    pub async fn run_turn(
        &self,
        messages: &mut Vec<Message>,
        event_tx: mpsc::Sender<AgentEvent>,
    ) -> Result<()> {
        self.run_turn_inner(messages, event_tx, true).await
    }

    async fn checkpoint_tool(&self, call: &ToolCall) -> Result<()> {
        let Some(checkpoints) = &self.tool_checkpoints else {
            return Ok(());
        };
        let (acknowledgement, saved) = oneshot::channel();
        checkpoints
            .send(ToolCheckpoint {
                call: call.clone(),
                acknowledgement,
            })
            .await
            .context("Tool was not started: history checkpoint receiver closed")?;
        tokio::time::timeout(Duration::from_secs(30), saved)
            .await
            .context("Tool was not started: history checkpoint timed out")?
            .context("Tool was not started: history checkpoint acknowledgement closed")?
            .context("Tool was not started: history checkpoint failed")
    }

    // Boxing gives the parent/child call graph a finite future type. Children
    // receive a restricted registry and cannot recursively delegate.
    pub(crate) fn run_turn_inner<'a>(
        &'a self,
        messages: &'a mut Vec<Message>,
        event_tx: mpsc::Sender<AgentEvent>,
        allow_delegation: bool,
    ) -> futures::future::BoxFuture<'a, Result<()>> {
        Box::pin(async move {
            if messages.is_empty() || messages[0].role != crate::types::Role::System {
                messages.insert(0, Message::system(self.default_system_prompt()));
            }

            let mut tools_schema = self.tools.to_schema_json();
            if allow_delegation {
                tools_schema
                    .as_array_mut()
                    .expect("tool schema array")
                    .push(crate::delegation::schema());
            }

            for iter in 0..self.max_iterations {
                info!("Agent iteration {}/{}", iter + 1, self.max_iterations);

                let assistant_msg = self
                    .client
                    .stream_chat(
                        &self.model_config,
                        messages,
                        Some(tools_schema.clone()),
                        event_tx.clone(),
                    )
                    .await?;

                messages.push(assistant_msg.clone());

                let tool_calls = match &assistant_msg.tool_calls {
                    Some(calls) if !calls.is_empty() => calls.clone(),
                    _ => {
                        // No more tool calls; agent has completed its turn
                        info!("Agent completed turn without pending tool calls");
                        return Ok(());
                    }
                };

                for tc in tool_calls {
                    event_tx
                        .send(AgentEvent::ToolCallStarted {
                            id: tc.id.clone(),
                            name: tc.name.clone(),
                            arguments: tc.arguments.clone(),
                        })
                        .await
                        .map_err(|_| {
                            anyhow::anyhow!("Tool was not started: event receiver closed")
                        })?;

                    if let Err(error) = self.checkpoint_tool(&tc).await {
                        let output = format!("{error:#}");
                        let _ = event_tx
                            .send(AgentEvent::ToolCallFinished {
                                id: tc.id.clone(),
                                name: tc.name.clone(),
                                output: output.clone(),
                                is_error: true,
                            })
                            .await;
                        messages.push(Message::tool_result(tc.id, tc.name, output, true));
                        return Err(error);
                    }

                    let output_res = if allow_delegation && tc.name == crate::delegation::TOOL_NAME
                    {
                        crate::delegation::execute(self, &tc, messages, &event_tx).await
                    } else if let Some(tool) = self.tools.get(&tc.name) {
                        tool.execute(tc.arguments.clone()).await
                    } else {
                        Err(anyhow::anyhow!("Unknown tool: {}", tc.name))
                    };

                    let (output, is_error) = match output_res {
                        Ok(out) => (out, false),
                        Err(err) => {
                            error!("Tool {} execution failed: {:?}", tc.name, err);
                            (format!("Error: {:#}", err), true)
                        }
                    };

                    let _ = event_tx
                        .send(AgentEvent::ToolCallFinished {
                            id: tc.id.clone(),
                            name: tc.name.clone(),
                            output: output.clone(),
                            is_error,
                        })
                        .await;

                    let tool_msg = Message::tool_result(tc.id, tc.name, output, is_error);
                    messages.push(tool_msg);
                }
            }

            anyhow::bail!("Reached maximum iteration limit of {}", self.max_iterations);
        })
    }
}
