use super::*;
use desktop_state::{Automation, SavedDesktop};
use pi_agent_core::agent_config::{self, ExportOptions, TrustMode};
use pi_agent_core::ModelConfig;
use std::fs;
use std::time::Duration;
extern crate gpui_kit as gpui;
gpui_kit::actions!(
    application,
    [
        NewTask,
        ToggleSidebar,
        ToggleSearch,
        Settings,
        Dismiss,
        FocusComposer,
        ToggleReview,
        ToggleTerminal
    ]
);
pub fn bind_keys(cx: &mut App) {
    let primary = if cfg!(target_os = "macos") {
        "cmd"
    } else {
        "ctrl"
    };
    cx.bind_keys([
        KeyBinding::new(&format!("{primary}-n"), NewTask, Some("Desktop")),
        KeyBinding::new(&format!("{primary}-b"), ToggleSidebar, Some("Desktop")),
        KeyBinding::new(&format!("{primary}-k"), ToggleSearch, Some("Desktop")),
        KeyBinding::new(&format!("{primary}-,"), Settings, Some("Desktop")),
        KeyBinding::new(&format!("{primary}-l"), FocusComposer, Some("Desktop")),
        KeyBinding::new("escape", Dismiss, Some("Desktop")),
        KeyBinding::new(&format!("{primary}-shift-d"), ToggleReview, Some("Desktop")),
        KeyBinding::new(&format!("{primary}-j"), ToggleTerminal, Some("Desktop")),
    ]);
}
impl PiDesktopApp {
    pub fn receive_agentconfig_url(
        &mut self,
        url: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.show_settings = true;
        self.settings_section = settings::Section::AgentConfig;
        match agentconfig_sources::subscription_source(url) {
            Ok(Some(source)) => self.fetch_agentconfig_source(source, window, cx),
            Ok(None) => {
                self.stage_agentconfig(agent_config::bundle_from_deep_link(url), window, cx)
            }
            Err(error) => self.stage_agentconfig(Err(error), window, cx),
        }
        if self.agentconfig_pending_preview.is_some() {
            self.agentconfig_url_input
                .update(cx, |input, cx| input.clean(window, cx));
        }
        window.activate_window();
        cx.activate(true);
    }

    pub fn import_agentconfig_url(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let url = self.agentconfig_url_input.read(cx).value().to_string();
        if url.trim().starts_with("https:") {
            self.fetch_agentconfig_source(url.trim().to_owned(), window, cx);
        } else {
            self.receive_agentconfig_url(url.trim(), window, cx);
        }
    }

    pub(crate) fn stage_agentconfig(
        &mut self,
        bundle: anyhow::Result<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.agentconfig_fetch_generation = self.agentconfig_fetch_generation.wrapping_add(1);
        if let Some(worker) = self.agentconfig_fetch_abort.take() {
            worker.abort();
        }
        self.agentconfig_fetching = false;
        self.agentconfig_remove_target = None;
        self.agentconfig_pending_source = None;
        self.agentconfig_source_checked_at = None;
        self.agentconfig_pending_target = None;
        self.agentconfig_pending_original = None;
        self.agentconfig_pending_raw = None;
        self.agentconfig_pending_preview = None;
        self.agentconfig_resolved_import = None;
        self.agentconfig_password_input
            .update(cx, |input, cx| input.clean(window, cx));
        self.agentconfig_backup_password_input
            .update(cx, |input, cx| input.clean(window, cx));
        self.agentconfig_backup_expanded = false;
        match bundle.and_then(|raw| Ok((agent_config::preview_bundle(&raw)?, raw))) {
            Ok((preview, raw)) => {
                let mut models = vec![SharedString::from(
                    self.tr("不更改当前模型", "Keep current model"),
                )];
                models.extend(
                    preview
                        .models
                        .iter()
                        .map(|model| SharedString::from(format!("{}/{}", model.provider, model.id)))
                        .collect::<Vec<_>>(),
                );
                self.agentconfig_model_select.update(cx, |select, cx| {
                    select.set_items(models, window, cx);
                    select.set_selected_index(Some(IndexPath::new(0)), window, cx);
                });
                self.agentconfig_pending_raw = Some(raw);
                let can_resolve = !preview.requires_password;
                self.agentconfig_pending_preview = Some(preview);
                self.agentconfig_status.clear();
                if can_resolve {
                    self.preview_agentconfig(cx);
                }
            }
            Err(_) => {
                self.agentconfig_status = self.tr(
                    "配置无效，请检查链接或文件的格式、版本与完整性。",
                    "Invalid configuration. Check the link or file format, version and completeness.",
                ).into();
            }
        }
        cx.notify();
    }

    pub fn cancel_agentconfig(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.agentconfig_fetch_generation = self.agentconfig_fetch_generation.wrapping_add(1);
        if let Some(worker) = self.agentconfig_fetch_abort.take() {
            worker.abort();
        }
        self.agentconfig_fetching = false;
        self.agentconfig_remove_target = None;
        self.agentconfig_pending_source = None;
        self.agentconfig_source_checked_at = None;
        self.agentconfig_pending_target = None;
        self.agentconfig_pending_original = None;
        self.agentconfig_pending_raw = None;
        self.agentconfig_pending_preview = None;
        self.agentconfig_resolved_import = None;
        self.agentconfig_password_input
            .update(cx, |input, cx| input.clean(window, cx));
        self.agentconfig_url_input
            .update(cx, |input, cx| input.clean(window, cx));
        self.agentconfig_status.clear();
        cx.notify();
    }

    pub fn copy_agentconfig_link(&mut self, cx: &mut Context<Self>) {
        let config = ModelConfig {
            provider: "openai-compatible".into(),
            model: self.model_identifier.clone(),
            api_base: Some(self.api_base.clone()),
            api_key: None,
            ..ModelConfig::default()
        };
        match agent_config::export_bundle(&config, &[], ExportOptions::default()) {
            Ok((_, link)) => {
                cx.write_to_clipboard(ClipboardItem::new_string(link));
                self.agentconfig_utility_status = self
                    .tr(
                        "分享链接已复制，不含凭据",
                        "Shared link copied (no credentials)",
                    )
                    .into();
            }
            Err(error) => {
                self.agentconfig_utility_status =
                    format!("{}: {error}", self.tr("分享失败", "Sharing failed"));
            }
        }
        cx.notify();
    }

    pub fn register_agentconfig_links(&mut self, cx: &mut Context<Self>) {
        #[cfg(target_os = "windows")]
        let registration = Task::ready(installer::register_agentconfig_protocol());
        #[cfg(not(target_os = "windows"))]
        let registration = cx.register_url_scheme(agent_config::DEEP_LINK_SCHEME);
        cx.spawn(async move |this, cx| {
            let registration = registration.await;
            let _ = this.update(cx, |this, cx| {
                this.agentconfig_utility_status = match registration {
                    Ok(()) => this
                        .tr("链接打开方式已注册", "Link handler registered")
                        .into(),
                    Err(error) => {
                        format!("{}: {error}", this.tr("注册失败", "Registration failed"))
                    }
                };
                cx.notify();
            });
        })
        .detach();
    }

    pub fn import_agentconfig(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.cancel_agentconfig(window, cx);
        self.agentconfig_fetching = true;
        let generation = self.agentconfig_fetch_generation;
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("Import AgentConfig bundle".into()),
        });
        cx.spawn_in(window, async move |this, cx| {
            let result: anyhow::Result<Option<String>> = async {
                let selected = receiver.await??;
                let Some(path) = selected.and_then(|paths| paths.into_iter().next()) else {
                    return Ok(None);
                };
                cx.background_executor()
                    .spawn(async move { agentconfig_links::read_bundle_file(&path).map(Some) })
                    .await
            }.await;
            let _ = this.update_in(cx, |this, window, cx| {
                if generation != this.agentconfig_fetch_generation {
                    return;
                }
                this.agentconfig_fetching = false;
                match result {
                    Ok(Some(raw)) => this.stage_agentconfig(Ok(raw), window, cx),
                    Ok(None) => {}
                    Err(_) => {
                        this.agentconfig_status = this.tr(
                            "无法读取配置文件，请检查文件类型、权限与大小。",
                            "Could not read configuration. Check the file type, permissions and size.",
                        ).into();
                        cx.notify();
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub fn preview_agentconfig(&mut self, cx: &mut Context<Self>) {
        self.agentconfig_resolved_import = None;
        let Some(raw) = self.agentconfig_pending_raw.clone() else {
            self.agentconfig_status = self
                .tr(
                    "请先导入 AgentConfig 链接或文件。",
                    "Import an AgentConfig link or file first.",
                )
                .into();
            cx.notify();
            return;
        };
        let password = self
            .agentconfig_password_input
            .read(cx)
            .unmask_value()
            .to_string();
        let password = (!password.trim().is_empty()).then_some(password.as_str());
        let selected_index = self
            .agentconfig_model_select
            .read(cx)
            .selected_index(cx)
            .and_then(|index| index.row.checked_sub(1));
        let imported = match agentconfig_links::resolve_import(&raw, password, selected_index) {
            Ok(imported) => imported,
            Err(error) => {
                self.agentconfig_status = match error {
                    agentconfig_links::ImportFailure::InvalidBundle(_) => self.tr(
                        "配置内容或解密密码不正确，请检查后重试。",
                        "The configuration or password is invalid. Check it and try again.",
                    ),
                    agentconfig_links::ImportFailure::InvalidEndpoint => self.tr(
                        "所选模型缺少有效的 HTTP(S) 服务地址，无法应用。请更换配置包。",
                        "The selected model has no valid HTTP(S) service URL. Use another bundle.",
                    ),
                }
                .into();
                cx.notify();
                return;
            }
        };
        self.agentconfig_resolved_import = Some(imported);
        self.agentconfig_status.clear();
        cx.notify();
    }

    pub fn apply_agentconfig(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.settings_saved = false;
        let Some(imported) = self.agentconfig_resolved_import.clone() else {
            self.agentconfig_status = self
                .tr(
                    "请先解密并预览变更。",
                    "Preview the resolved changes first.",
                )
                .into();
            cx.notify();
            return;
        };
        if self.storage_read_failed {
            self.agentconfig_status = self
                .tr(
                    "本地配置无法读取，导入已停止以保护已有数据。",
                    "Local storage could not be read. Import stopped to protect existing data.",
                )
                .into();
            cx.notify();
            return;
        }
        if let Err(error) = self.save_agentconfig_bundle(imported.model_config.as_ref()) {
            self.agentconfig_status = error.to_string();
            cx.notify();
            return;
        }
        if let Some(config) = imported.model_config {
            self.activate_agentconfig_model(config, window, cx);
        }
        self.agentconfig_pending_raw = None;
        self.agentconfig_pending_preview = None;
        self.agentconfig_resolved_import = None;
        self.agentconfig_pending_source = None;
        self.agentconfig_source_checked_at = None;
        self.agentconfig_pending_target = None;
        self.agentconfig_pending_original = None;
        self.agentconfig_password_input
            .update(cx, |input, cx| input.clean(window, cx));
        self.agentconfig_url_input
            .update(cx, |input, cx| input.clean(window, cx));
        self.agentconfig_status = self
            .tr("配置包已导入。", "Configuration bundle imported.")
            .into();
        self.settings_saved = true;
        cx.notify();
    }

    fn activate_agentconfig_model(
        &mut self,
        config: ModelConfig,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.model_identifier = config.model.clone();
        // A shared link must not send an existing provider's key to a new endpoint.
        self.api_base = config.api_base.unwrap_or_default();
        self.api_key = config.api_key.unwrap_or_default();
        self.settings_base_input.update(cx, |input, cx| {
            input.set_value(self.api_base.clone(), window, cx)
        });
        self.settings_model_input.update(cx, |input, cx| {
            input.set_value(self.model_identifier.clone(), window, cx)
        });
        self.settings_key_input
            .update(cx, |input, cx| input.clean(window, cx));
        self.preferences.api_base = Some(self.api_base.clone());
        self.preferences.allow_environment_credentials = false;
        self.preferences.model = self.model_identifier.clone();
    }

    pub fn export_agentconfig(
        &mut self,
        include_credentials: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let password = self
            .agentconfig_backup_password_input
            .read(cx)
            .unmask_value()
            .to_string();
        if include_credentials && password.trim().is_empty() {
            self.agentconfig_utility_status = self
                .tr(
                    "包含凭据的备份必须设置密码。",
                    "A password is required for credential backups.",
                )
                .into();
            cx.notify();
            return;
        }
        let config = ModelConfig {
            provider: "openai-compatible".into(),
            model: self.model_identifier.clone(),
            api_base: Some(self.api_base.clone()),
            api_key: (!self.api_key.trim().is_empty()).then_some(self.api_key.clone()),
            temperature: 0.2,
            max_tokens: Some(8192),
        };
        let options = ExportOptions {
            label: Some(format!("{} setup", self.identity.brand_name)),
            trust: if include_credentials {
                TrustMode::Self_
            } else {
                TrustMode::Shared
            },
            password: include_credentials.then_some(password),
            include_credentials,
        };
        let (raw, _deep_link) =
            match agent_config::export_bundle(&config, self.skill_registry.skills(), options) {
                Ok(bundle) => bundle,
                Err(error) => {
                    self.agentconfig_utility_status =
                        format!("{}: {error}", self.tr("导出失败", "Export failed"));
                    cx.notify();
                    return;
                }
            };
        let receiver = cx.prompt_for_new_path(&self.workspace_dir, Some("codex-native.acfg"));
        cx.spawn_in(window, async move |this, cx| {
            let result = receiver.await;
            let _ = this.update_in(cx, |this, window, cx| {
                match result {
                    Ok(Ok(Some(path))) => match fs::write(&path, raw) {
                        Ok(()) => {
                            this.agentconfig_utility_status =
                                format!("{}: {}", this.tr("已导出", "Exported"), path.display());
                        }
                        Err(error) => {
                            this.agentconfig_utility_status =
                                format!("{}: {error}", this.tr("导出失败", "Export failed"));
                        }
                    },
                    Ok(Ok(None)) => {}
                    Ok(Err(error)) => {
                        this.agentconfig_utility_status =
                            format!("{}: {error}", this.tr("导出失败", "Export failed"));
                    }
                    Err(error) => {
                        this.agentconfig_utility_status =
                            format!("{}: {error}", this.tr("导出失败", "Export failed"));
                    }
                }
                if include_credentials {
                    this.agentconfig_backup_password_input
                        .update(cx, |input, cx| input.clean(window, cx));
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub fn import_codex_session(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_executing {
            self.settings_error = self
                .tr(
                    "当前会话正在运行，完成或停止后再导入。",
                    "Finish or stop the active turn before importing.",
                )
                .into();
            cx.notify();
            return;
        }
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some(self.tr("导入 Codex 会话", "Import Codex session").into()),
        });
        cx.spawn_in(window, async move |this, cx| {
            let result: anyhow::Result<Option<Session>> = async {
                let selected = receiver.await??;
                let Some(path) = selected.and_then(|paths| paths.into_iter().next()) else {
                    return Ok(None);
                };
                cx.background_executor()
                    .spawn(async move {
                        let raw = fs::read_to_string(path)?;
                        Session::import_codex_json(&raw)
                    })
                    .await
                    .map(Some)
            }
            .await;
            let _ = this.update_in(cx, |this, window, cx| match result {
                Ok(Some(session)) => this.activate_imported_session(session, window, cx),
                Ok(None) => {}
                Err(error) => {
                    this.settings_error =
                        format!("{}: {error:#}", this.tr("导入失败", "Import failed"));
                    cx.notify();
                }
            });
        })
        .detach();
    }

    pub fn export_codex_session(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let raw = match serde_json::to_string_pretty(&self.session.export_codex_value()) {
            Ok(raw) => raw,
            Err(error) => {
                self.settings_error = format!("{}: {error}", self.tr("导出失败", "Export failed"));
                cx.notify();
                return;
            }
        };
        let receiver = cx.prompt_for_new_path(&self.workspace_dir, Some("codex-session.json"));
        cx.spawn_in(window, async move |this, cx| {
            let result = receiver.await;
            let _ = this.update_in(cx, |this, _, cx| {
                match result {
                    Ok(Ok(Some(path))) => match fs::write(&path, raw) {
                        Ok(()) => {
                            this.settings_error = format!(
                                "{}: {}",
                                this.tr("会话已导出", "Session exported"),
                                path.display()
                            );
                        }
                        Err(error) => {
                            this.settings_error =
                                format!("{}: {error}", this.tr("导出失败", "Export failed"));
                        }
                    },
                    Ok(Ok(None)) => {}
                    Ok(Err(error)) => {
                        this.settings_error =
                            format!("{}: {error}", this.tr("导出失败", "Export failed"));
                    }
                    Err(error) => {
                        this.settings_error =
                            format!("{}: {error}", this.tr("导出失败", "Export failed"));
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn activate_imported_session(
        &mut self,
        mut session: Session,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !session.engine.is_compiled() {
            session.set_engine(pi_agent_core::EngineKind::default());
        }
        self.persist_active_session();
        self.remember_scroll();
        let id = session.id.clone();
        self.session_history.insert(id.clone(), session.clone());
        self.session = session;
        if let Some(entry) = self.conversations.iter_mut().find(|entry| entry.id == id) {
            entry.title = self.session.title.clone();
        } else {
            self.conversations.insert(
                0,
                ConversationItem {
                    id: id.clone(),
                    title: self.session.title.clone(),
                    is_active: true,
                },
            );
        }
        for entry in &mut self.conversations {
            entry.is_active = entry.id == id;
        }
        self.scroll = ScrollHandle::new();
        self.scroll_pinned = true;
        self.restore_task_state();
        self.input_state.update(cx, |input, cx| {
            input.clean(window, cx);
            input.focus(window, cx);
        });
        self.settings_engine.update(cx, |state, cx| {
            state.set_selected_index(
                Some(IndexPath::new(settings::engine_index(self.session.engine))),
                window,
                cx,
            );
        });
        self.preferences.engine = self.session.engine;
        self.preferences.policy_profile = match self.session.engine {
            pi_agent_core::EngineKind::CodexCompatible => pi_agent_core::PolicyProfile::CodexStrict,
            pi_agent_core::EngineKind::PiNative => pi_agent_core::PolicyProfile::PiBalanced,
        };
        self.show_settings = false;
        self.selected_nav = "chat";
        self.status_message = self
            .tr("Codex 会话已导入。", "Codex session imported.")
            .into();
        self.settings_error.clear();
        self.persist_active_session();
        cx.notify();
    }

    pub fn persist_active_session(&mut self) {
        if self.storage_read_failed {
            return;
        } // Never overwrite an unreadable history with a default document.
        self.capture_task_state();
        self.session_history
            .insert(self.session.id.clone(), self.session.clone());
        let saved = self.desktop_snapshot();
        self.storage_error = match self.store.save(&saved) {
            Ok(()) => String::new(),
            Err(error) => format!("保存失败 / Save failed: {error}"),
        };
    }

    pub(crate) fn desktop_snapshot(&self) -> SavedDesktop {
        let mut sessions = self.session_history.clone();
        sessions.insert(self.session.id.clone(), self.session.clone());
        SavedDesktop {
            sessions,
            task_states: self.task_states.clone(),
            active_id: self.session.id.clone(),
            drafts: self.drafts.clone(),
            expanded_tools: self.expanded_tools.clone(),
            preferences: self.preferences.clone(),
            automations: self.automations.clone(),
            agentconfig_configurations: self.agentconfig_configurations.clone(),
        }
    }
    pub fn schedule_save(&mut self, cx: &mut Context<Self>) {
        self.save_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(400))
                .await;
            let _ = this.update(cx, |this, cx| {
                this.persist_active_session();
                cx.notify();
            });
        }));
    }
    pub fn new_conversation(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_executing {
            return;
        }
        self.project_navigation_generation = self.project_navigation_generation.wrapping_add(1);
        self.persist_active_session();
        self.remember_scroll();
        self.workflow_pane = None;
        self.session = Session::new(desktop_state::new_id(), self.t().new_conversation);
        self.scroll = ScrollHandle::new();
        self.scroll_pinned = true;
        for entry in &mut self.conversations {
            entry.is_active = false;
        }
        self.conversations.insert(
            0,
            ConversationItem {
                id: self.session.id.clone(),
                title: self.session.title.clone(),
                is_active: true,
            },
        );
        self.restore_task_state();
        self.input_state.update(cx, |input, cx| {
            input.clean(window, cx);
            input.focus(window, cx);
        });
        self.show_settings = false;
        self.selected_nav = "chat";
        self.persist_active_session();
        cx.notify();
    }
    pub fn activate_conversation(&mut self, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_executing {
            return;
        }
        let Some(session) = self.session_history.get(id).cloned() else {
            return;
        };
        self.project_navigation_generation = self.project_navigation_generation.wrapping_add(1);
        self.persist_active_session();
        self.remember_scroll();
        self.workflow_pane = None;
        self.session = session;
        (self.scroll, self.scroll_pinned) = self
            .scroll_memory
            .remove(id)
            .unwrap_or_else(|| (ScrollHandle::new(), true));
        for entry in &mut self.conversations {
            entry.is_active = entry.id == id;
        }
        let draft = self.drafts.get(id).cloned().unwrap_or_default();
        self.input_state.update(cx, |input, cx| {
            input.set_value(draft, window, cx);
            input.focus(window, cx);
        });
        self.show_settings = false;
        self.selected_nav = "chat";
        self.restore_task_state();
        self.persist_active_session();
        cx.notify();
    }
    pub fn focus_composer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.show_settings = false;
        self.selected_nav = "chat";
        if panels::review_uses_single_column(window.viewport_size().width) {
            self.workflow_pane = None;
        }
        self.input_state
            .update(cx, |input, cx| input.focus(window, cx));
        cx.notify();
    }
    fn remember_scroll(&mut self) {
        // Renderer-local only: never serialize reading position into chat data.
        if self.scroll_memory.len() >= 200 {
            if let Some(key) = self.scroll_memory.keys().next().cloned() {
                self.scroll_memory.remove(&key);
            }
        }
        self.scroll_memory.insert(
            self.session.id.clone(),
            (self.scroll.clone(), self.scroll_pinned),
        );
    }
    pub fn save_preferences(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let editing_provider = self.settings_section == settings::Section::Providers;
        let base = self.settings_base_input.read(cx).value().trim().to_string();
        let model = self
            .settings_model_input
            .read(cx)
            .value()
            .trim()
            .to_string();
        self.settings_saved = false;
        if editing_provider && (!desktop_state::validate_endpoint(&base) || model.is_empty()) {
            self.settings_error = self
                .tr(
                    "请输入有效 HTTP(S) 地址和模型名称",
                    "Enter a valid HTTP(S) URL and model name",
                )
                .into();
            cx.notify();
            return;
        }
        if editing_provider {
            self.api_base = base.clone();
            self.model_identifier = model.clone();
            let key = self.settings_key_input.read(cx).unmask_value().to_string();
            if !key.trim().is_empty() {
                self.api_key = key;
            }
            self.settings_key_input.update(cx, |input, cx| {
                // Clear the credential without emitting a user edit that cancels save feedback.
                input.set_value("", window, cx);
            });
        }
        self.preferences = desktop_state::Preferences {
            api_base: Some(self.api_base.clone()),
            allow_environment_credentials: self.preferences.allow_environment_credentials,
            model: self.model_identifier.clone(),
            language: self.language,
            theme: self.theme_preference,
            reduced_motion: self.preferences.reduced_motion,
            settings_section: Some(self.settings_section.key().into()),
            ui_font_size_px: self.preferences.ui_font_size_px,
            engine: self.session.engine,
            policy_profile: self.preferences.policy_profile,
            auth_provider: self.preferences.auth_provider,
        };
        self.persist_active_session();
        self.settings_error = self.storage_error.clone();
        self.settings_saved = !self.storage_read_failed && self.storage_error.is_empty();
        cx.notify();
    }
    pub fn change_language(
        &mut self,
        language: Language,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.language = language;
        let theme = self.theme_preference;
        let theme_labels = settings::settings_choices(settings::theme_labels(language));
        for state in [&self.settings_appearance_theme, &self.settings_footer_theme] {
            state.update(cx, |state, cx| {
                state.set_items(theme_labels.clone(), window, cx);
                state.set_selected_index(
                    Some(IndexPath::new(settings::theme_index(theme))),
                    window,
                    cx,
                );
            });
        }
        let language_index = if language == Language::Chinese { 0 } else { 1 };
        self.settings_engine.update(cx, |state, cx| {
            state.set_items(
                settings::settings_choices(settings::engine_labels(language)),
                window,
                cx,
            );
            state.set_selected_index(
                Some(IndexPath::new(settings::engine_index(self.session.engine))),
                window,
                cx,
            );
        });
        self.settings_auth_provider.update(cx, |state, cx| {
            state.set_items(
                settings::settings_choices(settings::auth_provider_labels(language)),
                window,
                cx,
            );
            state.set_selected_index(
                Some(IndexPath::new(settings::auth_provider_index(
                    self.preferences.auth_provider,
                ))),
                window,
                cx,
            );
        });
        for state in [
            &self.settings_general_language,
            &self.settings_footer_language,
        ] {
            state.update(cx, |state, cx| {
                state.set_selected_index(Some(IndexPath::new(language_index)), window, cx);
            });
        }
        self.settings_saved = false;
        let placeholder = self.t().input_placeholder_for(&self.identity);
        self.input_state.update(cx, |input, cx| {
            input.set_placeholder(placeholder, window, cx)
        });
        self.settings_key_input.update(cx, |input, cx| {
            input.set_placeholder(language.text().api_key_placeholder, window, cx)
        });
        self.search_input.update(cx, |input, cx| {
            input.set_placeholder(
                if language == Language::Chinese {
                    "搜索任务..."
                } else {
                    "Search tasks..."
                },
                window,
                cx,
            );
        });
        cx.notify();
    }

    pub fn change_engine(&mut self, value: &SharedString, cx: &mut Context<Self>) {
        if !settings::engine_switch_available() {
            return;
        }
        if self.is_executing {
            self.settings_error = self
                .tr(
                    "当前会话正在运行，完成或停止后再切换引擎。",
                    "Finish or stop the active turn before switching engines.",
                )
                .into();
            cx.notify();
            return;
        }
        let engine = if value.as_ref() == "Pi 自由模式" || value.as_ref() == "Pi native" {
            pi_agent_core::EngineKind::PiNative
        } else {
            pi_agent_core::EngineKind::CodexCompatible
        };
        if self.session.engine == engine {
            return;
        }
        self.persist_active_session();
        self.remember_scroll();
        let fork = self.session.fork(
            format!("{}-{}", self.session.id, desktop_state::new_id()),
            engine,
        );
        self.session_history.insert(fork.id.clone(), fork.clone());
        self.session = fork;
        for entry in &mut self.conversations {
            entry.is_active = false;
        }
        self.conversations.insert(
            0,
            ConversationItem {
                id: self.session.id.clone(),
                title: self.session.title.clone(),
                is_active: true,
            },
        );
        self.scroll = ScrollHandle::new();
        self.scroll_pinned = true;
        self.restore_task_state();
        self.preferences.engine = engine;
        self.preferences.policy_profile = match engine {
            pi_agent_core::EngineKind::CodexCompatible => pi_agent_core::PolicyProfile::CodexStrict,
            pi_agent_core::EngineKind::PiNative => pi_agent_core::PolicyProfile::PiBalanced,
        };
        self.settings_saved = false;
        self.settings_error.clear();
        self.persist_active_session();
        cx.notify();
    }

    pub fn change_auth_provider(&mut self, value: &SharedString, cx: &mut Context<Self>) {
        let provider = match value.as_ref() {
            "ChatGPT OAuth" => Some(pi_agent_core::auth::AuthProvider::ChatGpt),
            "DeepSeek OAuth" => Some(pi_agent_core::auth::AuthProvider::DeepSeek),
            settings::ACCOUNT_OAUTH_LABEL => Some(pi_agent_core::auth::AuthProvider::Provider),
            "Native OAuth" => Some(pi_agent_core::auth::AuthProvider::Pi),
            "自定义 OAuth" | "Custom OAuth" => Some(pi_agent_core::auth::AuthProvider::Custom),
            _ => None,
        };
        self.preferences.auth_provider = provider;
        self.settings_saved = false;
        self.settings_error.clear();
        cx.notify();
    }

    pub fn start_oauth_login(&mut self, cx: &mut Context<Self>) {
        let Some(provider) = self.preferences.auth_provider else {
            self.settings_error = self
                .tr("请先选择 OAuth 提供方。", "Select an OAuth provider first.")
                .into();
            cx.notify();
            return;
        };
        if provider == pi_agent_core::auth::AuthProvider::Provider {
            self.start_provider_oauth_login(cx);
            return;
        }
        let listener = match pi_agent_core::auth::OAuthCallbackListener::bind_sync() {
            Ok(listener) => listener,
            Err(error) => {
                self.settings_error = format!("OAuth callback unavailable: {error:#}");
                cx.notify();
                return;
            }
        };
        let config = match pi_agent_core::auth::OAuthProviderConfig::from_environment(provider) {
            Ok(config) => config,
            Err(error) => {
                self.settings_error = format!(
                    "{}: {error:#}",
                    self.tr(
                        "OAuth 提供方未配置官方端点",
                        "Official OAuth endpoints are not configured",
                    )
                );
                cx.notify();
                return;
            }
        };
        let flow = match pi_agent_core::auth::begin_pkce(&config, listener.redirect_uri()) {
            Ok(flow) => flow,
            Err(error) => {
                self.settings_error = format!("OAuth flow unavailable: {error:#}");
                cx.notify();
                return;
            }
        };
        if let Err(error) = crate::execution::open_external_url(flow.authorization_url.as_str()) {
            self.settings_error = format!(
                "{}: {error:#}",
                self.tr("无法打开登录页", "Could not open login page")
            );
            cx.notify();
            return;
        }
        self.settings_error = self
            .tr(
                "已打开浏览器，等待 OAuth 回调……",
                "Browser opened; waiting for the OAuth callback…",
            )
            .into();
        let store = pi_agent_core::auth::CredentialStore::new(
            pi_agent_core::auth::CredentialStore::default_path(),
        );
        let worker = self.runtime.spawn(async move {
            let code = listener.wait(&flow.state).await?;
            let token =
                pi_agent_core::auth::exchange_code(&reqwest::Client::new(), &config, &flow, &code)
                    .await?;
            store.save(token.clone().into())?;
            Ok::<_, anyhow::Error>((provider, token.access_token))
        });
        cx.spawn(async move |this, cx| {
            let result = worker.await;
            let _ = this.update(cx, |this, cx| {
                match result {
                    Ok(Ok((provider, access_token))) => {
                        this.preferences.auth_provider = Some(provider);
                        this.api_key = access_token;
                        this.settings_error.clear();
                        this.settings_saved = false;
                        this.status_message =
                            this.tr("OAuth 登录成功", "OAuth login complete").into();
                        this.persist_active_session();
                    }
                    Ok(Err(error)) => {
                        this.settings_error = format!("OAuth login failed: {error:#}");
                    }
                    Err(error) => {
                        this.settings_error = format!("OAuth login task failed: {error}");
                    }
                }
                cx.notify();
            });
        })
        .detach();
        cx.notify();
    }

    fn start_provider_oauth_login(&mut self, cx: &mut Context<Self>) {
        self.settings_error = self
            .tr("正在初始化提供方登录……", "Starting the provider login…")
            .into();
        let worker = self.runtime.spawn(async move {
            let client = reqwest::Client::new();
            let flow = pi_agent_core::auth::ProviderOAuthFlow::begin(&client, "zai").await?;
            crate::execution::open_external_url(flow.authorization_url.as_str())?;
            let token = flow.wait_ready(&client, "zai").await?;
            let store = pi_agent_core::auth::CredentialStore::new(
                pi_agent_core::auth::CredentialStore::default_path(),
            );
            store.save(token.into())
        });
        cx.spawn(async move |this, cx| {
            let result = worker.await;
            let _ = this.update(cx, |this, cx| {
                match result {
                    Ok(Ok(())) => {
                        this.settings_error.clear();
                        this.status_message = this.tr(
                            "账户授权成功。模型调用仍需 GLM Coding Plan API Key。",
                            "Account authorized. Model requests still need a GLM Coding Plan API key.",
                        ).into();
                    }
                    Ok(Err(error)) => {
                        this.settings_error = format!("OAuth login failed: {error:#}");
                    }
                    Err(error) => {
                        this.settings_error = format!("OAuth login task failed: {error}");
                    }
                }
                cx.notify();
            });
        })
        .detach();
        cx.notify();
    }

    pub fn change_theme(
        &mut self,
        value: &SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let labels = settings::theme_labels(self.language);
        let Some(index) = labels.iter().position(|label| label == value) else {
            return;
        };
        self.theme_preference = [
            ThemePreference::Light,
            ThemePreference::Dark,
            ThemePreference::Auto,
        ][index];
        for state in [&self.settings_appearance_theme, &self.settings_footer_theme] {
            state.update(cx, |state, cx| {
                state.set_selected_index(Some(IndexPath::new(index)), window, cx);
            });
        }
        self.settings_saved = false;
        cx.notify();
    }
    pub fn edit_automation(
        &mut self,
        id: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let job = id
            .as_ref()
            .and_then(|id| self.automations.iter().find(|job| &job.id == id))
            .cloned();
        self.automation_name.update(cx, |input, cx| {
            input.set_value(
                job.as_ref().map(|j| j.name.clone()).unwrap_or_default(),
                window,
                cx,
            )
        });
        self.automation_prompt.update(cx, |input, cx| {
            input.set_value(
                job.as_ref().map(|j| j.prompt.clone()).unwrap_or_default(),
                window,
                cx,
            )
        });
        self.automation_minutes.update(cx, |input, cx| {
            input.set_value(
                job.as_ref().map(|j| j.minutes).unwrap_or(60).to_string(),
                window,
                cx,
            )
        });
        self.automation_editing = id;
        self.automation_form = true;
        self.automation_error.clear();
        cx.notify();
    }
    pub fn save_automation(&mut self, cx: &mut Context<Self>) {
        let name = self.automation_name.read(cx).value().to_string();
        let prompt = self.automation_prompt.read(cx).value().to_string();
        let minutes =
            match Automation::validate(&name, &prompt, &self.automation_minutes.read(cx).value()) {
                Ok(minutes) => minutes,
                Err(error) => {
                    self.automation_error = error.into();
                    cx.notify();
                    return;
                }
            };
        let id = self
            .automation_editing
            .clone()
            .unwrap_or_else(desktop_state::new_id);
        let job = Automation {
            id: id.clone(),
            name,
            prompt,
            minutes,
            enabled: false,
            next_run: desktop_state::now() + u64::from(minutes) * 60,
        };
        if let Some(index) = self.automations.iter().position(|j| j.id == id) {
            self.automations[index] = job;
        } else {
            self.automations.push(job);
        }
        self.persist_active_session();
        if self.storage_read_failed || !self.storage_error.is_empty() {
            self.automation_error = self.storage_error.clone();
        } else {
            self.automation_form = false;
        }
        cx.notify();
    }
    pub fn start_timer(&mut self, window: &Window, cx: &mut Context<Self>) {
        self.timer = Some(cx.spawn_in(window, async move |this, cx| loop {
            cx.background_executor().timer(Duration::from_secs(1)).await;
            if this
                .update_in(cx, |this, window, cx| {
                    if this.is_executing || this.storage_read_failed {
                        return;
                    }
                    let now = desktop_state::now();
                    if let Some(index) = this
                        .automations
                        .iter()
                        .position(|job| job.enabled && job.next_run <= now)
                    {
                        let prompt = this.automations[index].prompt.clone();
                        this.automations[index].next_run =
                            now + u64::from(this.automations[index].minutes) * 60;
                        // A scheduled run gets its own task; the unsent composer draft stays with its original task.
                        this.persist_active_session();
                        this.project_navigation_generation =
                            this.project_navigation_generation.wrapping_add(1);
                        this.session = Session::new(
                            desktop_state::new_id(),
                            this.automations[index].name.clone(),
                        );
                        for entry in &mut this.conversations {
                            entry.is_active = false;
                        }
                        this.conversations.insert(
                            0,
                            ConversationItem {
                                id: this.session.id.clone(),
                                title: this.session.title.clone(),
                                is_active: true,
                            },
                        );
                        this.restore_task_state();
                        this.input_state
                            .update(cx, |input, cx| input.clean(window, cx));
                        this.submit_prompt(prompt, cx);
                    }
                })
                .is_err()
            {
                break;
            }
        }));
    }
}
