use super::*;

fn section(title: &'static str, font_size: Pixels, c: &ThemeColors) -> Div {
    div().v_flex().min_w_0().gap(px(12.)).child(
        div()
            .text_size(font_size)
            .font_semibold()
            .text_color(rgb(c.text_primary))
            .child(title),
    )
}

fn source_action(id: String, label: &'static str, glyph: Glyph) -> Button {
    let mut button = Button::new(id)
        .ghost()
        .small()
        .w(px(32.))
        .h(px(32.))
        .p_0()
        .rounded(px(6.))
        .child(Icon::new(glyph).size(px(16.)))
        .accessibility_label(label);
    button.interactivity().tooltip(move |window, cx| {
        gpui_kit::component::tooltip::Tooltip::new(label).build(window, cx)
    });
    button
}

impl PiDesktopApp {
    pub(super) fn agentconfig_settings(
        &self,
        c: &ThemeColors,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let pending = self.agentconfig_pending_preview.is_some();
        div()
            .v_flex()
            .min_w_0()
            .max_w(px(760.))
            .gap(px(28.))
            .child(if pending {
                self.agentconfig_preview(c, cx)
            } else {
                div()
                    .v_flex()
                    .min_w_0()
                    .gap(px(24.))
                    .child(self.agentconfig_library(c, cx))
                    .child(self.agentconfig_entry(c, cx))
                    .into_any_element()
            })
            .when(!self.agentconfig_status.is_empty(), |view| {
                view.child(
                    settings_caption(self.agentconfig_status.clone(), self.ui_text(14.), c)
                        .line_height(relative(1.5)),
                )
            })
            .when(!pending, |view| {
                view.child(self.agentconfig_sharing(c, cx))
                    .child(self.agentconfig_backup(c, cx))
                    .child(self.agentconfig_association(c, cx))
                    .when(!self.agentconfig_utility_status.is_empty(), |view| {
                        view.child(settings_caption(
                            self.agentconfig_utility_status.clone(),
                            self.ui_text(14.),
                            c,
                        ))
                    })
            })
            .into_any_element()
    }

    fn agentconfig_field(
        &self,
        label: &'static str,
        input: &Entity<InputState>,
        c: &ThemeColors,
    ) -> Div {
        provider_field(
            label,
            self.ui_text(14.),
            SettingsField::new(
                input,
                settings_input(Input::new(input), px(36.), self.ui_text(14.), c)
                    .w_full()
                    .aria_label(label),
                c,
                self.preferences.reduced_motion,
            ),
            c,
        )
    }

    fn agentconfig_entry(&self, c: &ThemeColors, cx: &mut Context<Self>) -> AnyElement {
        section(
            self.tr("添加配置源", "Add configuration source"),
            self.ui_text(14.),
            c,
        )
        .child(self.agentconfig_field(
            self.tr(
                "配置源 URL / AgentConfig 链接",
                "Source URL / AgentConfig link",
            ),
            &self.agentconfig_url_input,
            c,
        ))
        .child(
            settings_save_row()
                .child(
                    SettingsCommand::new(
                        "agentconfig-import-url",
                        settings_save_button(
                            if self.agentconfig_fetching {
                                self.tr("正在获取...", "Fetching...")
                            } else {
                                self.tr("获取并预览", "Fetch and preview")
                            },
                            self.ui_text(14.),
                        )
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.import_agentconfig_url(window, cx);
                        })),
                        CommandKind::Primary,
                        c,
                        self.preferences.reduced_motion,
                    )
                    .disabled(
                        self.agentconfig_fetching
                            || self
                                .agentconfig_url_input
                                .read(cx)
                                .value()
                                .trim()
                                .is_empty(),
                    ),
                )
                .when(self.agentconfig_fetching, |row| {
                    row.child(SettingsCommand::new(
                        "agentconfig-cancel-fetch",
                        settings_secondary_button(
                            "agentconfig-cancel-fetch",
                            self.tr("取消", "Cancel"),
                            Glyph::X,
                            self.ui_text(14.),
                        )
                        .on_click(
                            cx.listener(|this, _, window, cx| this.cancel_agentconfig(window, cx)),
                        ),
                        CommandKind::Neutral,
                        c,
                        self.preferences.reduced_motion,
                    ))
                })
                .child(
                    SettingsCommand::new(
                        "agentconfig-import-file",
                        settings_secondary_button(
                            "agentconfig-import-file",
                            self.tr("从文件导入...", "Import from file..."),
                            Glyph::Download,
                            self.ui_text(14.),
                        )
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.import_agentconfig(window, cx);
                        })),
                        CommandKind::Neutral,
                        c,
                        self.preferences.reduced_motion,
                    )
                    .disabled(self.agentconfig_fetching),
                ),
        )
        .into_any_element()
    }

    fn agentconfig_preview(&self, c: &ThemeColors, cx: &mut Context<Self>) -> AnyElement {
        let preview = self
            .agentconfig_pending_preview
            .as_ref()
            .expect("pending preview");
        section(
            if self.agentconfig_pending_target.is_some() {
                self.tr("配置更新预览", "Configuration update preview")
            } else {
                self.tr("配置导入预览", "Configuration import preview")
            },
            self.ui_text(14.), c,
        )
            .child(
                div()
                    .v_flex()
                    .gap(px(4.))
                    .child(
                        preview
                            .label
                            .clone()
                            .unwrap_or_else(|| "AgentConfig".into()),
                    )
                    .child(settings_caption(
                        format!(
                            "{} · {} {} · {} MCP · {} {} · {} {} · {} {} · {} {}",
                            self.tr(
                                if preview.requires_password {
                                    "加密包"
                                } else {
                                    "明文包"
                                },
                                if preview.requires_password {
                                    "Encrypted bundle"
                                } else {
                                    "Plain bundle"
                                },
                            ),
                            preview.models.len(),
                            self.tr("个模型", "models"),
                            preview.mcp_names.len(),
                            preview.skill_ids.len(),
                            self.tr("技能", "skills"),
                            preview.prompt_ids.len(),
                            self.tr("提示词", "prompts"),
                            preview.agent_ids.len(),
                            self.tr("代理", "agents"),
                            preview.resource_uris.len(),
                            self.tr("资源引用", "resource references"),
                        ),
                        self.ui_text(14.),
                        c,
                    )),
            )
            .when_some(self.agentconfig_pending_source.as_ref(), |view, source| {
                let host = reqwest::Url::parse(source).ok()
                    .and_then(|url| url.host_str().map(str::to_owned)).unwrap_or_default();
                view.child(provider_field(
                    self.tr("更新来源", "Update source"),
                    self.ui_text(14.), div().min_w_0().child(host), c,
                ))
            })
            .child(self.agentconfig_bundle_changes(c))
            .child(self.agentconfig_contents(c))
            .child(self.agentconfig_model(c))
            .when(
                preview.requires_password && self.agentconfig_resolved_import.is_none(),
                |view| {
                    view.child(self.agentconfig_field(
                        self.tr("解密密码", "Decryption password"),
                        &self.agentconfig_password_input,
                        c,
                    ))
                },
            )
            .when_some(
                self.agentconfig_resolved_import.as_ref(),
                |view, imported| view.child(self.agentconfig_changes(imported, c)),
            )
            .when(
                !preview.skill_ids.is_empty() || !preview.mcp_names.is_empty()
                    || !preview.prompt_ids.is_empty() || !preview.agent_ids.is_empty()
                    || !preview.resource_uris.is_empty(),
                |view| {
                view.child(settings_caption(
                    self.tr(
                        "MCP、技能状态、提示词、代理和资源引用：保留于配置包，尚未在运行时启用。",
                        "MCP, skill states, prompts, agents and resource references: retained in the bundle, not activated in the runtime.",
                    ),
                    self.ui_text(14.),
                    c,
                ))
            })
            .child(self.agentconfig_import_actions(c, cx))
            .into_any_element()
    }

    fn agentconfig_model(&self, c: &ThemeColors) -> AnyElement {
        let preview = self
            .agentconfig_pending_preview
            .as_ref()
            .expect("pending preview");
        let label = self.tr("同时启用模型（可选）", "Activate a model (optional)");
        if preview.models.is_empty() {
            return div().into_any_element();
        }
        provider_field(
            label,
            self.ui_text(14.),
            SettingsSelect::new(
                "agentconfig-model",
                settings_select(
                    Select::new(&self.agentconfig_model_select)
                        .placeholder(self.tr("不更改当前模型", "Keep current model")),
                    self.ui_text(14.),
                    c,
                )
                .w_full()
                .accessibility_label(label),
                c,
                self.preferences.reduced_motion,
            )
            .w_full(),
            c,
        )
        .into_any_element()
    }

    fn agentconfig_changes(
        &self,
        imported: &pi_agent_core::agent_config::ImportedConfig,
        c: &ThemeColors,
    ) -> Div {
        let Some(config) = imported.model_config.as_ref() else {
            return settings_caption(
                self.tr(
                    "当前模型与本地凭据保持不变。",
                    "Current model and local credentials remain unchanged.",
                ),
                self.ui_text(14.),
                c,
            );
        };
        section(self.tr("将发生的变更", "Changes to apply"), self.ui_text(14.), c)
            .children([
                (self.tr("当前模型", "Current model"), self.model_identifier.clone()),
                (self.tr("导入后模型", "Imported model"), format!("{}/{}", config.provider, config.model)),
                (self.tr("当前服务地址", "Current service URL"), self.api_base.clone()),
                (self.tr("导入后服务地址", "Imported service URL"), config.api_base.clone().unwrap_or_default()),
            ].into_iter().map(|(label, content)| {
                provider_field(label, self.ui_text(14.), div().min_w_0().child(content), c)
            }))
            .child(settings_caption(
                if config.api_key.as_ref().is_some_and(|key| !key.is_empty()) {
                    self.tr(
                        "将使用包内密钥，仅本次运行有效；环境凭据将停用。",
                        "The bundled key is used for this run only; environment credentials will be disabled.",
                    )
                } else {
                    self.tr(
                        "将清空当前 API 密钥，并停用环境凭据。",
                        "The current API key will be cleared and environment credentials disabled.",
                    )
                },
                self.ui_text(14.),
                c,
            ))
    }

    fn agentconfig_import_actions(&self, c: &ThemeColors, cx: &mut Context<Self>) -> Div {
        let can_apply = self.agentconfig_resolved_import.is_some();
        let can_decrypt = self
            .agentconfig_pending_preview
            .as_ref()
            .is_some_and(|preview| {
                crate::agentconfig_links::can_decrypt(
                    preview,
                    self.agentconfig_model_select
                        .read(cx)
                        .selected_index(cx)
                        .and_then(|index| index.row.checked_sub(1)),
                    &self.agentconfig_password_input.read(cx).unmask_value(),
                )
            });
        let encrypted = self
            .agentconfig_pending_preview
            .as_ref()
            .is_some_and(|preview| preview.requires_password);
        settings_save_row()
            .when(can_apply || encrypted, |row| {
                row.child(
                    SettingsCommand::new(
                        "agentconfig-confirm",
                        settings_save_button(
                            if can_apply {
                                if self.agentconfig_pending_target.is_some() {
                                    self.tr("确认同步", "Confirm update")
                                } else {
                                    self.tr("确认导入", "Confirm import")
                                }
                            } else {
                                self.tr("解密并预览", "Decrypt and preview")
                            },
                            self.ui_text(14.),
                        )
                        .on_click(cx.listener(|this, _, window, cx| {
                            if this.agentconfig_resolved_import.is_some() {
                                this.apply_agentconfig(window, cx);
                            } else {
                                this.preview_agentconfig(cx);
                            }
                        })),
                        CommandKind::Primary,
                        c,
                        self.preferences.reduced_motion,
                    )
                    .disabled(!can_apply && !can_decrypt),
                )
            })
            .child(SettingsCommand::new(
                "agentconfig-cancel",
                settings_secondary_button(
                    "agentconfig-cancel",
                    self.tr("取消导入", "Cancel import"),
                    Glyph::X,
                    self.ui_text(14.),
                )
                .on_click(cx.listener(|this, _, window, cx| {
                    this.cancel_agentconfig(window, cx);
                })),
                CommandKind::Neutral,
                c,
                self.preferences.reduced_motion,
            ))
    }

    fn agentconfig_sharing(&self, c: &ThemeColors, cx: &mut Context<Self>) -> Div {
        section(
            self.tr("分享当前模型配置", "Share current model configuration"),
            self.ui_text(14.),
            c,
        )
        .pt(px(20.))
        .border_t_1()
        .border_color(rgba(c.settings_border))
        .child(
            settings_save_row()
                .child(SettingsCommand::new(
                    "agentconfig-copy-link",
                    settings_secondary_button(
                        "agentconfig-copy-link",
                        self.tr("复制分享链接", "Copy sharing link"),
                        Glyph::Copy,
                        self.ui_text(14.),
                    )
                    .on_click(cx.listener(|this, _, _, cx| this.copy_agentconfig_link(cx))),
                    CommandKind::Neutral,
                    c,
                    self.preferences.reduced_motion,
                ))
                .child(SettingsCommand::new(
                    "agentconfig-export",
                    settings_secondary_button(
                        "agentconfig-export",
                        self.tr("导出分享文件...", "Export shared file..."),
                        Glyph::Upload,
                        self.ui_text(14.),
                    )
                    .on_click(
                        cx.listener(|this, _, window, cx| {
                            this.export_agentconfig(false, window, cx)
                        }),
                    ),
                    CommandKind::Neutral,
                    c,
                    self.preferences.reduced_motion,
                )),
        )
    }

    fn agentconfig_bundle_changes(&self, c: &ThemeColors) -> Div {
        let changes = self.agentconfig_pending_raw.as_deref().map(|incoming| {
            crate::agentconfig_sources::changes(
                self.agentconfig_pending_original.as_deref(),
                incoming,
            )
        });
        let mut view = section(
            self.tr("配置包差异", "Bundle changes"),
            self.ui_text(14.),
            c,
        );
        let Some(Ok(changes)) = changes else {
            return view;
        };
        if changes.is_empty() {
            return view.child(settings_caption(
                self.tr("配置包内容未变。", "Bundle content is unchanged."),
                self.ui_text(14.),
                c,
            ));
        }
        for change in changes {
            let label = match change.section.as_str() {
                "models" => self.tr("模型", "Models"),
                "mcp" => "MCP",
                "skills" => self.tr("技能状态", "Skill states"),
                "prompts" => self.tr("提示词", "Prompts"),
                "agents" => self.tr("代理", "Agents"),
                "resources" => self.tr("资源引用", "Resource references"),
                "payload" => self.tr("载荷（内容隐藏）", "Payload (content hidden)"),
                "trust" => self.tr("信任模式", "Trust mode"),
                "capabilities" => self.tr("能力声明", "Capabilities"),
                "label" => self.tr("名称", "Label"),
                "src" => self.tr("生成客户端", "Producing client"),
                "hint" => self.tr("密码提示", "Password hint"),
                _ => self.tr("扩展字段", "Extension field"),
            };
            view = view.child(div().h_flex().flex_wrap().gap(px(8.)).child(label).child(
                settings_caption(
                    format!(
                        "+{} / -{} / {} {}",
                        change.added,
                        change.removed,
                        change.changed,
                        self.tr("变更", "changed")
                    ),
                    self.ui_text(14.),
                    c,
                ),
            ));
        }
        view
    }

    fn agentconfig_contents(&self, c: &ThemeColors) -> Div {
        let preview = self
            .agentconfig_pending_preview
            .as_ref()
            .expect("pending preview");
        let mut view = div().v_flex().min_w_0().gap(px(8.));
        let models = preview
            .models
            .iter()
            .map(|model| format!("{}/{}", model.provider, model.id))
            .collect::<Vec<_>>();
        for (label, entries) in [
            (self.tr("模型", "Models"), &models),
            ("MCP", &preview.mcp_names),
            (self.tr("技能状态", "Skill states"), &preview.skill_ids),
            (self.tr("提示词", "Prompts"), &preview.prompt_ids),
            (self.tr("代理", "Agents"), &preview.agent_ids),
            (
                self.tr("资源引用", "Resource references"),
                &preview.resource_uris,
            ),
        ] {
            if !entries.is_empty() {
                let mut summary = entries
                    .iter()
                    .take(8)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ");
                if entries.len() > 8 {
                    summary.push_str(&format!(" (+{})", entries.len() - 8));
                }
                view = view.child(provider_field(
                    label,
                    self.ui_text(14.),
                    div().min_w_0().whitespace_normal().child(summary),
                    c,
                ));
            }
        }
        view
    }

    fn agentconfig_library(&self, c: &ThemeColors, cx: &mut Context<Self>) -> Div {
        let mut view = section(
            self.tr("配置源", "Configuration sources"),
            self.ui_text(14.),
            c,
        );
        if self.agentconfig_configurations.is_empty() {
            return view.child(settings_caption(
                self.tr("暂无已导入配置", "No imported configurations"),
                self.ui_text(14.),
                c,
            ));
        }
        for config in &self.agentconfig_configurations {
            view = view.child(self.agentconfig_source_row(config, c, cx));
        }
        view
    }

    fn agentconfig_source_row(
        &self,
        config: &crate::agentconfig_sources::Configuration,
        c: &ThemeColors,
        cx: &mut Context<Self>,
    ) -> Div {
        let mut row = self
            .agentconfig_source_summary(config, c)
            .child(self.agentconfig_source_actions(config, c, cx));
        if self.agentconfig_remove_target.as_deref() == Some(config.id.as_str()) {
            row = row.child(self.agentconfig_remove_confirmation(&config.id, c, cx));
        }
        row
    }

    fn agentconfig_source_summary(
        &self,
        config: &crate::agentconfig_sources::Configuration,
        c: &ThemeColors,
    ) -> Div {
        let mut row = div()
            .v_flex()
            .min_w_0()
            .gap(px(8.))
            .py(px(12.))
            .border_b_1()
            .border_color(rgba(c.settings_border))
            .child(div().min_w_0().child(config.label()))
            .child(settings_caption(
                if config.source_url.is_some() {
                    format!(
                        "{} · {}",
                        config.source_label(),
                        self.tr(
                            "可更新来源 · 扩展配置",
                            "Updatable source · configuration extension"
                        )
                    )
                } else {
                    self.tr("快照配置包", "Snapshot bundle").into()
                },
                self.ui_text(14.),
                c,
            ));
        if let Some(checked) = config.checked_at {
            row = row.child(settings_caption(
                format!(
                    "{} {} {}",
                    self.tr("上次检查：", "Last checked:"),
                    desktop_state::now().saturating_sub(checked) / 60,
                    self.tr("分钟前", "minutes ago")
                ),
                self.ui_text(14.),
                c,
            ));
        }
        row
    }

    fn agentconfig_source_actions(
        &self,
        config: &crate::agentconfig_sources::Configuration,
        c: &ThemeColors,
        cx: &mut Context<Self>,
    ) -> Div {
        let open_id = config.id.clone();
        let remove_id = config.id.clone();
        let copy_id = config.id.clone();
        let mut actions = settings_save_row().child(
            SettingsCommand::new(
                format!("agentconfig-open-{}", config.id),
                source_action(
                    format!("agentconfig-open-{}", config.id),
                    self.tr("查看配置包", "View bundle"),
                    Glyph::File,
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.open_saved_agentconfig(&open_id, window, cx)
                })),
                CommandKind::Neutral,
                c,
                self.preferences.reduced_motion,
            )
            .disabled(self.agentconfig_fetching),
        );
        if let Some(source) = config.source_url.clone() {
            actions =
                actions
                    .child(
                        SettingsCommand::new(
                            format!("agentconfig-check-{}", config.id),
                            source_action(
                                format!("agentconfig-check-{}", config.id),
                                self.tr("检查更新", "Check for updates"),
                                Glyph::RefreshCw,
                            )
                            .on_click(cx.listener(
                                move |this, _, window, cx| {
                                    this.fetch_agentconfig_source(source.clone(), window, cx)
                                },
                            )),
                            CommandKind::Neutral,
                            c,
                            self.preferences.reduced_motion,
                        )
                        .disabled(self.agentconfig_fetching),
                    )
                    .child(SettingsCommand::new(
                        format!("agentconfig-copy-source-{}", config.id),
                        source_action(
                            format!("agentconfig-copy-source-{}", config.id),
                            self.tr("复制分发链接", "Copy source link"),
                            Glyph::Copy,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.copy_agentconfig_source(&copy_id, cx)
                        })),
                        CommandKind::Neutral,
                        c,
                        self.preferences.reduced_motion,
                    ));
        }
        let confirming_remove =
            self.agentconfig_remove_target.as_deref() == Some(config.id.as_str());
        actions.when(!confirming_remove, |actions| {
            actions.child(
                SettingsCommand::new(
                    format!("agentconfig-remove-{}", config.id),
                    source_action(
                        format!("agentconfig-remove-{}", config.id),
                        self.tr("移除配置源", "Remove source"),
                        Glyph::X,
                    )
                    .on_click(
                        cx.listener(move |this, _, _, cx| this.remove_agentconfig(&remove_id, cx)),
                    ),
                    CommandKind::Neutral,
                    c,
                    self.preferences.reduced_motion,
                )
                .disabled(self.agentconfig_fetching),
            )
        })
    }

    fn agentconfig_remove_confirmation(
        &self,
        id: &str,
        c: &ThemeColors,
        cx: &mut Context<Self>,
    ) -> Div {
        let remove_id = id.to_owned();
        div()
            .v_flex()
            .gap(px(8.))
            .child(settings_caption(
                self.tr(
                    "移除后不再检查此来源，当前已启用配置保持不变。",
                    "Removing this source stops update checks; active settings stay unchanged.",
                ),
                self.ui_text(14.),
                c,
            ))
            .child(
                settings_save_row()
                    .child(SettingsCommand::new(
                        format!("agentconfig-confirm-remove-{id}"),
                        settings_secondary_button(
                            format!("agentconfig-confirm-remove-{id}"),
                            self.tr("确认移除", "Confirm removal"),
                            Glyph::X,
                            self.ui_text(14.),
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.remove_agentconfig(&remove_id, cx)
                        })),
                        CommandKind::Neutral,
                        c,
                        self.preferences.reduced_motion,
                    ))
                    .child(SettingsCommand::new(
                        format!("agentconfig-cancel-remove-{id}"),
                        settings_secondary_button(
                            format!("agentconfig-cancel-remove-{id}"),
                            self.tr("取消", "Cancel"),
                            Glyph::ArrowLeft,
                            self.ui_text(14.),
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.agentconfig_remove_target = None;
                            cx.notify();
                        })),
                        CommandKind::Neutral,
                        c,
                        self.preferences.reduced_motion,
                    )),
            )
    }

    fn agentconfig_backup(&self, c: &ThemeColors, cx: &mut Context<Self>) -> Div {
        div()
            .v_flex()
            .min_w_0()
            .gap(px(12.))
            .child(SettingsCommand::new(
                "agentconfig-backup-toggle",
                settings_secondary_button(
                    "agentconfig-backup-toggle",
                    self.tr("个人备份", "Personal backup"),
                    if self.agentconfig_backup_expanded {
                        Glyph::ChevronDown
                    } else {
                        Glyph::ChevronRight
                    },
                    self.ui_text(14.),
                )
                .self_start()
                .on_click(cx.listener(|this, _, window, cx| {
                    this.agentconfig_backup_expanded = !this.agentconfig_backup_expanded;
                    this.agentconfig_backup_password_input
                        .update(cx, |input, cx| input.clean(window, cx));
                    cx.notify();
                })),
                CommandKind::Neutral,
                c,
                self.preferences.reduced_motion,
            ))
            .when(self.agentconfig_backup_expanded, |view| {
                view.child(self.agentconfig_field(
                    self.tr("备份密码", "Backup password"),
                    &self.agentconfig_backup_password_input,
                    c,
                ))
                .child(
                    SettingsCommand::new(
                        "agentconfig-backup",
                        settings_secondary_button(
                            "agentconfig-backup",
                            self.tr("导出加密备份...", "Export encrypted backup..."),
                            Glyph::FileKey,
                            self.ui_text(14.),
                        )
                        .self_start()
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.export_agentconfig(true, window, cx)
                        })),
                        CommandKind::Neutral,
                        c,
                        self.preferences.reduced_motion,
                    )
                    .disabled(
                        self.agentconfig_backup_password_input
                            .read(cx)
                            .unmask_value()
                            .trim()
                            .is_empty(),
                    ),
                )
            })
    }

    fn agentconfig_association(&self, c: &ThemeColors, cx: &mut Context<Self>) -> Div {
        section(
            self.tr("系统链接关联", "System link association"),
            self.ui_text(14.),
            c,
        )
        .pt(px(20.))
        .border_t_1()
        .border_color(rgba(c.settings_border))
        .child(SettingsCommand::new(
            "agentconfig-register-links",
            settings_secondary_button(
                "agentconfig-register-links",
                self.tr("设置链接打开方式...", "Set link handler..."),
                Glyph::ExternalLink,
                self.ui_text(14.),
            )
            .self_start()
            .on_click(cx.listener(|this, _, _, cx| this.register_agentconfig_links(cx))),
            CommandKind::Neutral,
            c,
            self.preferences.reduced_motion,
        ))
    }
}
