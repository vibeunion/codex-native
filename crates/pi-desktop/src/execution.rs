use super::*;
use anyhow::{bail, Context as _, Result};
use gpui_kit::component::notification::Notification;
use std::ffi::OsStr;
use std::io::Write;
use std::path::{Path, PathBuf};

struct HtmlPreviewNotice;

fn launch_in_default_browser(target: &OsStr) -> Result<()> {
    #[cfg(target_os = "macos")]
    const OPENER: &str = "open";
    #[cfg(target_os = "windows")]
    const OPENER: &str = "explorer.exe";
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    const OPENER: &str = "xdg-open";
    std::process::Command::new(OPENER)
        .arg(target)
        .spawn()
        .context("无法启动默认浏览器")?;
    Ok(())
}

fn external_http_url(url: &str) -> Result<reqwest::Url> {
    let parsed = reqwest::Url::parse(url).context("链接格式无效")?;
    if !matches!(parsed.scheme(), "http" | "https")
        || parsed.host_str().is_none()
        || !parsed.username().is_empty()
        || parsed.password().is_some()
    {
        bail!("只允许打开不含凭据的 HTTP(S) 链接");
    }
    Ok(parsed)
}

pub fn open_external_url(url: &str) -> Result<()> {
    let parsed = external_http_url(url)?;
    launch_in_default_browser(OsStr::new(parsed.as_str()))
}

fn write_artifact_file(file_path: &Path, content: &str) -> Result<()> {
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(file_path).context("无法创建 HTML 产物")?;
    file.write_all(content.as_bytes())
        .context("无法写入 HTML 产物")
}

pub fn open_html_artifact(
    content: &str,
    is_dark: bool,
    colors: &crate::theme::ThemeColors,
) -> Result<PathBuf> {
    if content.len() > crate::generative_ui::MAX_ARTIFACT_BYTES {
        bail!(
            "HTML 产物超过 {} 字节限制",
            crate::generative_ui::MAX_ARTIFACT_BYTES
        );
    }
    let themed_html =
        crate::generative_ui::inject_theme_variables_to_html(content, is_dark, colors);
    let file_path =
        std::env::temp_dir().join(format!("app_web_{}.html", crate::desktop_state::new_id()));
    write_artifact_file(&file_path, &themed_html)?;
    if let Err(error) = launch_in_default_browser(file_path.as_os_str()) {
        let _ = std::fs::remove_file(&file_path);
        return Err(error);
    }
    Ok(file_path)
}

fn local_command(prompt: &str) -> Option<String> {
    let trimmed = prompt.trim();
    if let Some(command) = trimmed.strip_prefix('$') {
        return Some(command.trim().to_string());
    }
    if ["cargo ", "git ", "cat ", "ls "]
        .iter()
        .any(|prefix| trimmed.starts_with(prefix))
    {
        return Some(trimmed.to_string());
    }
    match trimmed.to_lowercase().as_str() {
        "test" | "测试" => Some("cargo test -p pi-agent-core".into()),
        "lint" | "clippy" => Some("cargo clippy -p pi-agent-core".into()),
        "build" => Some("cargo check -p pi-desktop".into()),
        _ => None,
    }
}

pub fn reconcile_interrupted_tools(messages: &mut Vec<Message>) {
    let mut remaining = std::mem::take(messages).into_iter().peekable();
    messages.reserve(remaining.len());
    while let Some(message) = remaining.next() {
        // Provider IDs can be reused by later assistant messages.
        let mut following_results = Vec::new();
        while remaining
            .peek()
            .is_some_and(|next| next.role == pi_agent_core::Role::Tool)
        {
            following_results.push(remaining.next().expect("peeked tool result"));
        }
        let interrupted = interrupted_tool_results(&message, &following_results);
        messages.push(message);
        messages.extend(interrupted);
        messages.extend(following_results);
    }
}

fn interrupted_tool_results(message: &Message, following_results: &[Message]) -> Vec<Message> {
    let completed: std::collections::HashSet<&str> = following_results
        .iter()
        .filter_map(|message| message.tool_result.as_ref())
        .map(|result| result.tool_call_id.as_str())
        .collect();
    message
        .tool_calls
        .iter()
        .flatten()
        .filter(|call| !completed.contains(call.id.as_str()))
        .map(|call| {
            Message::tool_result(
                call.id.clone(),
                call.name.clone(),
                "Execution interrupted; outcome unknown. Check workspace before retrying.",
                true,
            )
        })
        .collect()
}
impl PiDesktopApp {
    pub fn open_html_preview(&self, content: &str, window: &mut Window, cx: &mut Context<Self>) {
        let is_dark = self.theme_preference == ThemePreference::Dark;
        let colors = if is_dark {
            ThemeColors::dark()
        } else {
            ThemeColors::light()
        };
        match open_html_artifact(content, is_dark, &colors) {
            Ok(_) => window.remove_notification::<HtmlPreviewNotice>(cx),
            Err(error) => window.push_notification(
                Notification::error(format!(
                    "{}: {error}",
                    self.tr("产物打开失败", "Could not open artifact")
                ))
                .id::<HtmlPreviewNotice>()
                .autohide(false),
                cx,
            ),
        }
    }

    pub fn handle_generative_ui_action(
        &mut self,
        action: &crate::generative_ui::UiAction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // 1. If copy_text is present, copy directly to system clipboard
        if let Some(text) = &action.copy_text {
            cx.write_to_clipboard(gpui::ClipboardItem::new_string(text.clone()));
            self.status_message = format!("已复制【{}】到剪贴板", action.label);
            cx.notify();
            return;
        }

        // 2. If url is present, open in default browser
        if let Some(url) = &action.url {
            if let Err(error) = open_external_url(url) {
                self.run_error = Some(error.to_string());
                self.status_message = self.tr("链接打开失败", "Could not open link").into();
                cx.notify();
            }
            return;
        }

        // 3. Artifact sidebar or web window actions
        if let Some(payload) = &action.payload {
            if let Some(act) = payload.get("action").and_then(|v| v.as_str()) {
                if act == "open_sidebar" {
                    let id = payload
                        .get("id")
                        .and_then(|v| v.as_str())
                        .unwrap_or(&action.id);
                    let title = payload
                        .get("title")
                        .and_then(|v| v.as_str())
                        .unwrap_or(&action.label);
                    let kind = payload
                        .get("kind")
                        .and_then(|v| v.as_str())
                        .unwrap_or("code");
                    let content = payload
                        .get("content")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    self.artifact_pane = Some(crate::ArtifactPaneState {
                        id: id.to_string(),
                        title: title.to_string(),
                        kind: kind.to_string(),
                        content: content.to_string(),
                    });
                    self.artifact_visible = true;
                    self.status_message = format!("已在侧边栏打开产物【{}】", title);
                    cx.notify();
                    return;
                } else if act == "open_web_window" {
                    let content = payload
                        .get("content")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    self.open_html_preview(content, window, cx);
                    return;
                }
            }
        }

        // 3. Prompt dispatch to Agent
        let prompt = if let Some(p) = &action.prompt {
            p.clone()
        } else if let Some(payload) = &action.payload {
            format!("已执行操作【{}】(参数: {})", action.label, payload)
        } else {
            format!("已执行操作【{}】(ID: {})", action.label, action.id)
        };

        self.input_state.update(cx, |input, cx| {
            input.set_value(prompt, window, cx);
        });
        self.trigger_send(window, cx);
    }

    pub fn trigger_send(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let prompt = self.input_state.read(cx).value().to_string();
        if prompt.trim().is_empty() {
            return;
        }
        self.input_state
            .update(cx, |input, cx| input.clean(window, cx));
        self.drafts.remove(&self.session.id);

        if let Some(cmd) = crate::workbench::parse_slash_command(&prompt) {
            match cmd {
                crate::workbench::SlashCommand::Compact => {
                    self.session.messages.push(Message::user("/compact"));
                    self.session.messages.push(Message::assistant(
                        self.tr("会话历史已压缩。", "Conversation history compacted."),
                    ));
                    self.persist_active_session();
                    cx.notify();
                    return;
                }
                crate::workbench::SlashCommand::SendGoal { objective } => {
                    self.task_states
                        .entry(self.session.id.clone())
                        .or_default()
                        .set_goal(objective.clone());
                    self.submit_prompt(objective, cx);
                    return;
                }
                crate::workbench::SlashCommand::ResumeGoal => {
                    let objective = self.task_states.get_mut(&self.session.id).and_then(|s| {
                        if s.resume_goal() {
                            s.goal.as_ref().map(|g| g.objective.clone())
                        } else {
                            None
                        }
                    });
                    if let Some(obj) = objective {
                        self.submit_prompt(obj, cx);
                    } else {
                        self.run_error = Some(
                            self.tr(
                                "当前没有可恢复的目标，请先使用 /goal 设定目标。",
                                "No goal to resume. Set a goal first using /goal.",
                            )
                            .into(),
                        );
                        cx.notify();
                    }
                    return;
                }
                crate::workbench::SlashCommand::ClearGoal => {
                    self.task_states
                        .entry(self.session.id.clone())
                        .or_default()
                        .clear_goal();
                    self.persist_active_session();
                    cx.notify();
                    return;
                }
                crate::workbench::SlashCommand::EmptyGoal => {
                    self.run_error = Some(
                        self.tr(
                            "请输入目标内容，例如: /goal 实现用户登录",
                            "Please provide a goal objective, e.g.: /goal implement user login",
                        )
                        .into(),
                    );
                    cx.notify();
                    return;
                }
            }
        }

        if self.is_executing {
            self.task_states
                .entry(self.session.id.clone())
                .or_default()
                .enqueue(prompt);
            self.persist_active_session();
            cx.notify();
            return;
        }
        self.submit_prompt(prompt, cx);
    }
    pub fn submit_prompt(&mut self, prompt: String, cx: &mut Context<Self>) {
        if self.is_executing || prompt.trim().is_empty() {
            return;
        }
        self.run_error = None;
        self.run_stopped = false;
        self.task_states
            .entry(self.session.id.clone())
            .or_default()
            .begin();
        if let Some(goal) = self
            .task_states
            .get_mut(&self.session.id)
            .and_then(|s| s.goal.as_mut())
        {
            goal.status = "running".into();
        }
        self.run_generation += 1;
        if !self
            .session
            .messages
            .iter()
            .any(|m| m.role == pi_agent_core::Role::User)
        {
            self.session.title = prompt.chars().take(48).collect();
            if let Some(entry) = self
                .conversations
                .iter_mut()
                .find(|entry| entry.id == self.session.id)
            {
                entry.title = self.session.title.clone();
            }
        }
        self.session.messages.push(Message::user(prompt.trim()));
        self.session.updated_at = desktop_state::now() as i64 * 1000;
        self.is_executing = true;
        self.status_message = self.tr("正在执行", "Working").into();
        self.show_settings = false;
        self.selected_nav = "chat";
        self.persist_active_session();
        if !self.storage_error.is_empty() {
            self.fail_unsaved_run(cx);
            return;
        }
        self.scroll_pinned = true;
        self.scroll.scroll_to_bottom();
        cx.notify();
        if let Some(command) = local_command(&prompt) {
            self.start_local_command(command, cx);
        } else if !self.api_key.trim().is_empty() {
            self.start_agent_turn(cx);
        } else {
            self.is_executing = false;
            self.run_error = Some(
                self.tr(
                    "请在设置中配置 API Key 后重试。",
                    "Configure an API key in Settings, then retry.",
                )
                .into(),
            );
            self.task_states
                .entry(self.session.id.clone())
                .or_default()
                .interrupted(self.run_error.clone().unwrap_or_default());
            self.persist_active_session();
            cx.notify();
        }
    }
    fn fail_unsaved_run(&mut self, cx: &mut Context<Self>) {
        self.run_error = Some(self.storage_error.clone());
        self.finish_run(cx);
    }

    fn start_local_command(&mut self, command: String, cx: &mut Context<Self>) {
        let agent = self.agent.clone();
        let generation = self.run_generation;
        let id = desktop_state::new_id();
        let mut call = Message::assistant("");
        call.tool_calls = Some(vec![pi_agent_core::ToolCall {
            id: id.clone(),
            name: "bash".into(),
            arguments: serde_json::json!({"command":command}),
        }]);
        self.session.messages.push(call);
        self.persist_active_session();
        if !self.storage_error.is_empty() {
            self.session.messages.push(Message::tool_result(
                id,
                "bash",
                self.tr(
                    "历史记录无法保存，命令未启动。",
                    "History could not be saved. The command was not started.",
                ),
                true,
            ));
            self.fail_unsaved_run(cx);
            return;
        }
        let worker = self.runtime.spawn(async move {
            let start = std::time::Instant::now();
            let result = match agent.tools.get("bash") {
                Some(tool) => tool.execute(serde_json::json!({"command":command})).await,
                None => Err(anyhow::anyhow!("bash is unavailable")),
            };
            (result, start.elapsed())
        });
        self.running = Some(worker.abort_handle());
        cx.spawn(async move |this, cx| {
            let outcome = worker.await;
            let _ = this.update(cx, |this, cx| {
                if this.run_generation != generation {
                    return;
                }
                let (output, is_error) = match outcome {
                    Ok((Ok(output), elapsed)) => {
                        (format!("{output}\n\n{:.2}s", elapsed.as_secs_f32()), false)
                    }
                    Ok((Err(error), _)) => (error.to_string(), true),
                    Err(error) => (error.to_string(), true),
                };
                if is_error && !this.run_stopped {
                    this.run_error = Some(output.clone());
                }
                this.session.messages.push(Message::tool_result(
                    id.clone(),
                    "bash",
                    output,
                    is_error,
                ));
                if is_error {
                    this.expanded_tools.insert(id);
                }
                this.finish_run(cx);
            });
        })
        .detach();
    }
    fn start_agent_turn(&mut self, cx: &mut Context<Self>) {
        let generation = self.run_generation;
        let mut messages = self.session.messages.clone();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        let (checkpoint_tx, mut checkpoints) = tokio::sync::mpsc::channel(1);
        let model = ModelConfig {
            provider: "openai-compatible".into(),
            model: self.model_identifier.clone(),
            api_base: Some(self.api_base.clone()),
            api_key: Some(self.api_key.clone()),
            temperature: 0.2,
            max_tokens: Some(8192),
        };
        let profile = match self.session.engine {
            pi_agent_core::EngineKind::CodexCompatible => pi_agent_core::PolicyProfile::CodexStrict,
            pi_agent_core::EngineKind::PiNative => self.preferences.policy_profile,
        };
        let mut agent = PiAgent::with_profile(self.workspace_dir.clone(), model, profile);
        agent.tool_checkpoints = Some(checkpoint_tx);
        self.workflow_controls = agent.workflow_controls.clone();
        let worker = self.runtime.spawn(async move {
            let outcome = agent.run_turn(&mut messages, tx).await;
            (outcome, messages)
        });
        self.running = Some(worker.abort_handle());
        cx.spawn(async move |this, cx| {
            loop {
                tokio::select! {
                    Some(event) = rx.recv() => {
                        if this.update(cx, |this, cx| {
                            if this.run_generation == generation {
                                this.apply_agent_event(event, cx);
                            }
                        }).is_err() {
                            worker.abort();
                            break;
                        }
                    }
                    Some(checkpoint) = checkpoints.recv() => {
                        // The start event precedes this request; drain it and earlier results
                        // so the saved history preserves their execution order.
                        let mut pending_events = Vec::new();
                        while let Ok(event) = rx.try_recv() {
                            pending_events.push(event);
                        }
                        let saved = this.update(cx, |this, cx| {
                            if this.run_generation != generation {
                                anyhow::bail!("Run is no longer active");
                            }
                            for event in pending_events {
                                this.apply_agent_event(event, cx);
                            }
                            if this.run_stopped {
                                anyhow::bail!("Run was stopped");
                            }
                            this.persist_active_session();
                            if !this.storage_error.is_empty() {
                                anyhow::bail!("{}", this.storage_error);
                            }
                            Ok(())
                        }).unwrap_or_else(Err);
                        let _ = checkpoint.acknowledgement.send(saved);
                    }
                    else => break,
                }
            }
            drop(checkpoints);
            drop(rx);
            let outcome = worker.await;
            let _ = this.update(cx, |this, cx| {
                if this.run_generation != generation {
                    return;
                }
                match outcome {
                    Ok((Ok(()), messages)) => {
                        this.session.messages = messages
                            .into_iter()
                            .filter(|m| m.role != pi_agent_core::Role::System)
                            .collect()
                    }
                    Ok((Err(error), _)) if !this.run_stopped => {
                        this.run_error = Some(error.to_string())
                    }
                    Err(error) if !this.run_stopped => this.run_error = Some(error.to_string()),
                    _ => {}
                }
                this.finish_run(cx);
            });
        })
        .detach();
    }
    fn apply_agent_event(&mut self, event: AgentEvent, cx: &mut Context<Self>) {
        match event {
            AgentEvent::WorkflowUpdated(snapshot) => {
                if self.session.messages.iter().any(|message| {
                    message.id == snapshot.parent_message_id
                        && message.role == pi_agent_core::Role::User
                }) {
                    self.task_states
                        .entry(self.session.id.clone())
                        .or_default()
                        .update_workflow(snapshot);
                }
            }
            AgentEvent::TextDelta(text) => {
                if self.session.messages.last().is_none_or(|m| {
                    m.role != pi_agent_core::Role::Assistant || m.tool_calls.is_some()
                }) {
                    self.session.messages.push(Message::assistant(""));
                }
                self.session
                    .messages
                    .last_mut()
                    .unwrap()
                    .content
                    .push_str(&text);
            }
            AgentEvent::Thought(text) => {
                if self.session.messages.last().is_none_or(|m| {
                    m.role != pi_agent_core::Role::Assistant || m.tool_calls.is_some()
                }) {
                    self.session.messages.push(Message::assistant(""));
                }
                self.session
                    .messages
                    .last_mut()
                    .unwrap()
                    .reasoning
                    .get_or_insert_default()
                    .push_str(&text);
            }
            AgentEvent::ToolCallStarted {
                id,
                name,
                arguments,
            } => {
                self.status_message = format!("{}: {name}", self.tr("运行工具", "Running tool"));
                let mut message = Message::assistant("");
                message.tool_calls = Some(vec![pi_agent_core::ToolCall {
                    id,
                    name,
                    arguments,
                }]);
                self.session.messages.push(message);
            }
            AgentEvent::ToolCallFinished {
                id,
                name,
                output,
                is_error,
            } => {
                if is_error {
                    self.expanded_tools.insert(id.clone());
                }
                self.session
                    .messages
                    .push(Message::tool_result(id, name, output, is_error));
            }
            AgentEvent::Completed { total_tokens } => {
                if let Some(tokens) = total_tokens {
                    self.tokens_used = tokens as usize;
                }
            }
            AgentEvent::Error(error) if !self.run_stopped => self.run_error = Some(error),
            AgentEvent::Error(_) => {}
        }
        if self.scroll_pinned {
            self.scroll.scroll_to_bottom();
        }
        self.schedule_save(cx);
        cx.notify();
    }
    fn finish_run(&mut self, cx: &mut Context<Self>) {
        self.running = None;
        if let Some(state) = self.task_states.get_mut(&self.session.id) {
            for workflow in &mut state.workflows {
                workflow.interrupt();
            }
        }
        reconcile_interrupted_tools(&mut self.session.messages);
        self.is_executing = false;
        self.status_message = self.tr("已完成", "Completed").into();
        if self.run_stopped {
            self.status_message = self.tr("已停止", "Stopped").into();
        } else if let Some(error) = &self.run_error {
            self.status_message = self.tr("执行失败", "Failed").into();
            self.task_states
                .entry(self.session.id.clone())
                .or_default()
                .interrupted(error.clone());
        } else {
            self.task_states
                .entry(self.session.id.clone())
                .or_default()
                .error = None;
        }
        if let Some(state) = self.task_states.get_mut(&self.session.id) {
            if let Some(goal) = &mut state.goal {
                if self.run_stopped {
                    goal.status = "paused".into();
                } else if self.run_error.is_some() {
                    goal.status = "failed".into();
                } else {
                    goal.iteration += 1;
                    goal.status = "completed".into();
                }
            }
        }
        self.capture_task_state();
        self.persist_active_session();
        if self.run_error.is_none() {
            if let Some(prompt) = self
                .task_states
                .entry(self.session.id.clone())
                .or_default()
                .next_prompt()
            {
                self.submit_prompt(prompt, cx);
            }
        }
        if self.review_visible {
            self.refresh_review(cx);
        }
        cx.notify();
    }
    pub fn stop_run(&mut self, cx: &mut Context<Self>) {
        if let Some(task) = self.running.take() {
            task.abort();
        }
        self.status_message = self.tr("正在停止", "Stopping").into();
        self.run_stopped = true;
        // Keep the run busy until the join callback observes cancellation and cleanup.
        self.run_error = Some(
            self.tr(
                "任务已停止；请检查已执行的工具结果。",
                "Stopped. Review any tool actions already performed.",
            )
            .into(),
        );
        self.task_states
            .entry(self.session.id.clone())
            .or_default()
            .interrupted(self.run_error.clone().unwrap_or_default());
        self.persist_active_session();
        cx.notify();
    }

    pub fn stop_workflow(&mut self, workflow_id: String, cx: &mut Context<Self>) {
        if self.workflow_controls.stop(&workflow_id) {
            // The card owns its pending-stop label. The parent turn continues.
            cx.notify();
        }
    }
    pub fn retry_run(&mut self, cx: &mut Context<Self>) {
        if self.is_executing {
            return;
        }
        if let Some(last) = self
            .session
            .messages
            .iter()
            .rposition(|m| m.role == pi_agent_core::Role::User)
        {
            let prompt = self.session.messages[last].content.clone();
            self.submit_prompt(prompt, cx);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::{local_command, mask_api_key, Language};

    fn pending_bash_call(id: &str) -> pi_agent_core::Message {
        let mut message = pi_agent_core::Message::assistant("partial");
        message.tool_calls = Some(vec![pi_agent_core::ToolCall {
            id: id.into(),
            name: "bash".into(),
            arguments: serde_json::json!({}),
        }]);
        message
    }

    #[test]
    fn local_command_dispatch_preserves_existing_routes() {
        assert_eq!(local_command("$ git status").as_deref(), Some("git status"));
        assert_eq!(local_command("cargo test").as_deref(), Some("cargo test"));
        assert_eq!(
            local_command("测试").as_deref(),
            Some("cargo test -p pi-agent-core")
        );
        assert_eq!(
            local_command("lint").as_deref(),
            Some("cargo clippy -p pi-agent-core")
        );
        assert_eq!(
            local_command("build").as_deref(),
            Some("cargo check -p pi-desktop")
        );
        assert_eq!(local_command("hello"), None);
    }

    #[test]
    fn interrupted_tools_are_reconciled_without_losing_completed_results() {
        use super::*;
        let mut assistant = Message::assistant("partial");
        assistant.tool_calls = Some(vec![
            pi_agent_core::ToolCall {
                id: "done".into(),
                name: "bash".into(),
                arguments: serde_json::json!({}),
            },
            pi_agent_core::ToolCall {
                id: "pending".into(),
                name: "bash".into(),
                arguments: serde_json::json!({}),
            },
        ]);
        let mut messages = vec![
            assistant,
            Message::tool_result("done", "bash", "actual output", false),
        ];
        reconcile_interrupted_tools(&mut messages);
        assert_eq!(messages.len(), 3);
        assert!(messages[1].tool_result.as_ref().unwrap().is_error);
        assert_eq!(messages[2].content, "actual output");
        reconcile_interrupted_tools(&mut messages);
        assert_eq!(messages.len(), 3);
    }

    #[test]
    fn recovery_reused_tool_ids_do_not_hide_another_messages_pending_call() {
        use super::*;
        for (messages, missing_index, completed_index) in [
            (
                vec![
                    pending_bash_call("reused"),
                    Message::tool_result("reused", "bash", "actual output", false),
                    pending_bash_call("reused"),
                ],
                3,
                1,
            ),
            (
                vec![
                    pending_bash_call("reused"),
                    pending_bash_call("reused"),
                    Message::tool_result("reused", "bash", "actual output", false),
                ],
                1,
                3,
            ),
        ] {
            let directory = tempfile::tempdir().unwrap();
            let path = directory.path().join("session.json");
            let mut session = pi_agent_core::Session::new("reused-id", "Recovery fixture");
            session.messages = messages;
            session.save_to_file(&path).unwrap();
            let mut restored = pi_agent_core::Session::load_from_file(&path).unwrap();

            reconcile_interrupted_tools(&mut restored.messages);
            assert_eq!(restored.messages.len(), 4);
            let missing = restored.messages[missing_index]
                .tool_result
                .as_ref()
                .unwrap();
            assert_eq!(missing.tool_call_id, "reused");
            assert!(missing.is_error);
            assert!(missing.output.contains("outcome unknown"));
            let completed = &restored.messages[completed_index];
            assert_eq!(completed.content, "actual output");
            assert!(!completed.tool_result.as_ref().unwrap().is_error);

            let recovered = serde_json::to_value(&restored.messages).unwrap();
            reconcile_interrupted_tools(&mut restored.messages);
            assert_eq!(serde_json::to_value(&restored.messages).unwrap(), recovered);
        }
    }

    #[test]
    fn recovery_tool_results_do_not_cross_user_or_assistant_boundaries() {
        use super::*;
        for boundary in [
            Message::user("next turn"),
            Message::assistant("next response"),
            Message::system("new context"),
        ] {
            let mut messages = vec![
                pending_bash_call("reused"),
                boundary,
                Message::tool_result("reused", "bash", "later output", false),
            ];
            reconcile_interrupted_tools(&mut messages);
            assert_eq!(messages.len(), 4);
            assert!(messages[1].tool_result.as_ref().unwrap().is_error);
            assert_eq!(messages[3].content, "later output");
        }
    }

    #[test]
    fn recovery_preserves_out_of_order_results_and_missing_call_order() {
        use super::*;
        let mut assistant = pending_bash_call("a");
        assistant.tool_calls = Some(
            ["a", "b", "c", "d"]
                .into_iter()
                .map(|id| pi_agent_core::ToolCall {
                    id: id.into(),
                    name: "bash".into(),
                    arguments: serde_json::json!({}),
                })
                .collect(),
        );
        let mut messages = vec![
            assistant,
            Message::tool_result("b", "bash", "output b", false),
            Message::tool_result("a", "bash", "output a", false),
        ];
        reconcile_interrupted_tools(&mut messages);
        let results: Vec<_> = messages
            .iter()
            .filter_map(|message| message.tool_result.as_ref())
            .map(|result| (result.tool_call_id.as_str(), result.is_error))
            .collect();
        assert_eq!(
            results,
            [("c", true), ("d", true), ("b", false), ("a", false)]
        );
        assert_eq!(messages[3].content, "output b");
        assert_eq!(messages[4].content, "output a");
    }

    #[test]
    fn recovery_preserves_empty_history_and_unpaired_tool_records() {
        use super::*;
        let mut empty = Vec::new();
        reconcile_interrupted_tools(&mut empty);
        assert!(empty.is_empty());

        let orphan = Message::tool_result("reused", "bash", "earlier output", false);
        let mut incomplete = Message::tool_result("incomplete", "bash", "partial output", false);
        incomplete.tool_result = None;
        let expected_orphan = serde_json::to_value(&orphan).unwrap();
        let expected_incomplete = serde_json::to_value(&incomplete).unwrap();
        let mut messages = vec![
            orphan,
            Message::system("restored context"),
            pending_bash_call("reused"),
            incomplete,
        ];
        reconcile_interrupted_tools(&mut messages);
        assert_eq!(messages.len(), 5);
        assert_eq!(serde_json::to_value(&messages[0]).unwrap(), expected_orphan);
        assert!(messages[3].tool_result.as_ref().unwrap().is_error);
        assert_eq!(
            serde_json::to_value(&messages[4]).unwrap(),
            expected_incomplete
        );
        let recovered = serde_json::to_value(&messages).unwrap();
        reconcile_interrupted_tools(&mut messages);
        assert_eq!(serde_json::to_value(&messages).unwrap(), recovered);
    }

    #[test]
    fn masks_long_short_and_unicode_credentials() {
        for key in [
            "dummy-test-credential",
            "短密钥",
            "12345678",
            "测试用的长字符串凭证",
        ] {
            let masked = mask_api_key(key, Language::English);
            assert!(!masked.contains(key));
            assert!(masked.contains("••••"));
        }
        assert_eq!(mask_api_key("", Language::English), "Not configured");
    }

    #[test]
    fn browser_links_only_allow_http_without_credentials() {
        for url in [
            "javascript:alert(1)",
            "file:///tmp/widget.html",
            "data:text/html,test",
            "https://user:pass@example.com",
            "https://user@example.com",
            "--help",
        ] {
            assert!(super::external_http_url(url).is_err(), "{url}");
        }
        let parsed = super::external_http_url("HTTPS://EXAMPLE.COM/hello world?q=a&b=c").unwrap();
        assert_eq!(parsed.as_str(), "https://example.com/hello%20world?q=a&b=c");
    }

    #[test]
    fn artifact_creation_never_overwrites_existing_files() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("widget.html");
        super::write_artifact_file(&path, "original").unwrap();
        assert!(super::write_artifact_file(&path, "replacement").is_err());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "original");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                std::fs::metadata(path).unwrap().permissions().mode() & 0o777,
                0o600
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn artifact_creation_rejects_existing_symlinks() {
        let directory = tempfile::tempdir().unwrap();
        let destination = directory.path().join("original.html");
        let link = directory.path().join("widget.html");
        std::fs::write(&destination, "original").unwrap();
        std::os::unix::fs::symlink(&destination, &link).unwrap();
        assert!(super::write_artifact_file(&link, "replacement").is_err());
        assert_eq!(std::fs::read_to_string(destination).unwrap(), "original");
    }
}
