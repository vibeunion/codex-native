use super::*;
use pi_agent_core::agent_config;

struct SourceCandidate {
    url: String,
    original: Option<agentconfig_sources::Configuration>,
    bundle: String,
}

impl PiDesktopApp {
    pub(crate) fn fetch_agentconfig_source(
        &mut self,
        source: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let source = match agentconfig_sources::source_url(&source) {
            Ok(url) => url.to_string(),
            Err(_) => {
                self.agentconfig_status = self
                    .tr(
                        "配置源必须是有效的 HTTPS 链接。",
                        "A configuration source must be a valid HTTPS URL.",
                    )
                    .into();
                cx.notify();
                return;
            }
        };
        self.cancel_agentconfig(window, cx);
        self.agentconfig_url_input
            .update(cx, |input, cx| input.set_value(source.clone(), window, cx));
        let generation = self.agentconfig_fetch_generation;
        let original = self
            .agentconfig_configurations
            .iter()
            .find(|config| config.source_url.as_deref() == Some(&source))
            .cloned();
        self.agentconfig_fetching = true;
        self.agentconfig_status = self
            .tr("正在检查配置源…", "Checking configuration source...")
            .into();
        let requested_source = source.clone();
        let worker = self
            .runtime
            .spawn(async move { agentconfig_sources::fetch_bundle(&requested_source).await });
        self.agentconfig_fetch_abort = Some(worker.abort_handle());
        cx.spawn_in(window, async move |this, cx| {
            let outcome = worker.await;
            let _ = this.update_in(cx, |this, window, cx| {
                if generation != this.agentconfig_fetch_generation {
                    return;
                }
                this.agentconfig_fetching = false;
                this.agentconfig_fetch_abort = None;
                match outcome {
                    Ok(Ok(bundle)) => this.preview_source_update(
                        SourceCandidate { url: source, original, bundle }, window, cx,
                    ),
                    // reqwest errors may contain a private URL; do not log or render them.
                    _ => {
                        this.agentconfig_status = this.tr(
                            "无法获取配置。请检查网络、公开 HTTPS 地址及配置包格式；原有配置未更改。",
                            "Could not fetch configuration. Check the network, public HTTPS URL and bundle format. Existing configuration is unchanged.",
                        ).into();
                        cx.notify();
                    }
                }
            });
        }).detach();
        cx.notify();
    }

    fn preview_source_update(
        &mut self,
        candidate: SourceCandidate,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let SourceCandidate {
            url: source,
            original,
            bundle: raw,
        } = candidate;
        let unchanged = original.as_ref().is_some_and(|config| {
            agentconfig_sources::same_content(&config.bundle, &raw).unwrap_or(false)
        });
        if unchanged && !self.storage_read_failed {
            if let Some(original) = original {
                let mut saved = self.desktop_snapshot();
                if let Some(config) = saved
                    .agentconfig_configurations
                    .iter_mut()
                    .find(|config| config.id == original.id)
                {
                    config.checked_at = Some(desktop_state::now());
                }
                match self.store.save(&saved) {
                    Ok(()) => {
                        self.agentconfig_configurations = saved.agentconfig_configurations;
                        self.agentconfig_status = self
                            .tr("配置已是最新。", "Configuration is up to date.")
                            .into();
                    }
                    Err(_) => {
                        self.agentconfig_status = self.tr("配置已是最新，但检查时间未能保存。", "Configuration is up to date, but the check time could not be saved.").into();
                    }
                }
            }
            cx.notify();
            return;
        }
        self.stage_agentconfig(Ok(raw), window, cx);
        self.agentconfig_pending_source = Some(source);
        self.agentconfig_source_checked_at = Some(desktop_state::now());
        if let Some(original) = original {
            self.agentconfig_pending_target = Some(original.id);
            self.agentconfig_pending_original = Some(original.bundle);
        }
        cx.notify();
    }

    pub(crate) fn open_saved_agentconfig(
        &mut self,
        id: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(config) = self
            .agentconfig_configurations
            .iter()
            .find(|config| config.id == id)
            .cloned()
        else {
            return;
        };
        self.stage_agentconfig(Ok(config.bundle.clone()), window, cx);
        self.agentconfig_pending_source = config.source_url;
        self.agentconfig_pending_target = Some(config.id);
        self.agentconfig_pending_original = Some(config.bundle);
    }

    pub(crate) fn remove_agentconfig(&mut self, id: &str, cx: &mut Context<Self>) {
        if self.storage_read_failed || self.agentconfig_fetching {
            return;
        }
        if self.agentconfig_remove_target.as_deref() != Some(id) {
            self.agentconfig_remove_target = Some(id.into());
            cx.notify();
            return;
        }
        let mut saved = self.desktop_snapshot();
        saved
            .agentconfig_configurations
            .retain(|config| config.id != id);
        match self.store.save(&saved) {
            Ok(()) => {
                self.agentconfig_configurations = saved.agentconfig_configurations;
                self.agentconfig_remove_target = None;
                self.agentconfig_status = self.tr(
                    "已移除配置源；当前运行配置未改变。",
                    "Configuration source removed; active settings are unchanged.",
                ).into();
            }
            Err(_) => self.agentconfig_status = self.tr(
                "未能移除配置源，原有配置未改变。",
                "Could not remove the configuration source. Existing configuration is unchanged.",
            ).into(),
        }
        cx.notify();
    }

    pub(crate) fn copy_agentconfig_source(&mut self, id: &str, cx: &mut Context<Self>) {
        if let Some(source) = self
            .agentconfig_configurations
            .iter()
            .find(|config| config.id == id)
            .and_then(|config| config.source_url.clone())
        {
            let Ok(link) = agentconfig_sources::subscription_link(&source) else {
                return;
            };
            cx.write_to_clipboard(ClipboardItem::new_string(link));
            self.agentconfig_status = self
                .tr("配置源链接已复制。", "Configuration source link copied.")
                .into();
            cx.notify();
        }
    }

    pub(crate) fn save_agentconfig_bundle(
        &mut self,
        model: Option<&ModelConfig>,
    ) -> anyhow::Result<()> {
        let raw = self
            .agentconfig_pending_raw
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("缺少配置包 / Missing bundle"))?;
        agent_config::preview_bundle(raw)?;
        let configurations = agentconfig_sources::import_candidate(
            &self.agentconfig_configurations,
            agentconfig_sources::PendingConfiguration {
                target: self.agentconfig_pending_target.as_deref(),
                original: self.agentconfig_pending_original.as_deref(),
                source: self.agentconfig_pending_source.clone(),
                checked_at: self.agentconfig_source_checked_at,
                bundle: raw.clone(),
            },
        )?;
        self.capture_task_state();
        let mut saved = self.desktop_snapshot();
        saved.agentconfig_configurations = configurations;
        if let Some(model) = model {
            saved.preferences.model = model.model.clone();
            saved.preferences.api_base = model.api_base.clone();
            saved.preferences.allow_environment_credentials = false;
        }
        self.store.save(&saved).map_err(|_| anyhow::anyhow!(
            "未能保存配置，原有配置未改变。请检查磁盘权限；若其他实例已保存，请重新打开后重试 / Could not save configuration. Check disk permissions, or reopen if another instance has saved."
        ))?;
        self.agentconfig_configurations = saved.agentconfig_configurations;
        self.preferences = saved.preferences;
        self.storage_error.clear();
        Ok(())
    }
}
