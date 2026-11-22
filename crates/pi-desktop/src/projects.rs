use super::*;
use gpui_kit::assets::IconName as Glyph;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::list::{List, ListDelegate, ListItem, ListState};
use gpui_kit::component::popover::Popover;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::prelude::FluentBuilder;
use std::path::Path;

#[derive(Clone)]
struct ProjectRequest {
    path: PathBuf,
    intent: ProjectIntent,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ProjectIntent {
    session_id: String,
    navigation_generation: u64,
    run_generation: u64,
}

impl EventEmitter<ProjectRequest> for PiDesktopApp {}

pub(super) fn project_name(path: &Path) -> String {
    path.file_name()
        .filter(|name| !name.is_empty())
        .unwrap_or(path.as_os_str())
        .to_string_lossy()
        .into_owned()
}

fn validate_directory(path: &Path) -> anyhow::Result<PathBuf> {
    anyhow::ensure!(path.is_absolute(), "Project path must be absolute");
    let path = path.canonicalize()?;
    anyhow::ensure!(path.is_dir(), "Project path must be a directory");
    Ok(path)
}

fn resolve_project(path: &Path, known: &[PathBuf]) -> anyhow::Result<PathBuf> {
    let canonical = validate_directory(path)?;
    // A known alias must keep its original workspace store key.
    Ok(known
        .iter()
        .find(|root| root.canonicalize().ok().as_ref() == Some(&canonical))
        .cloned()
        .unwrap_or(canonical))
}

fn can_leave(executing: bool, importing: bool, pending_preview: bool, read_failed: bool) -> bool {
    !executing && !importing && !pending_preview && !read_failed
}

fn selection_is_current(expected: &ProjectIntent, active: &ProjectIntent, executing: bool) -> bool {
    expected == active && !executing
}

pub(super) fn project_session(
    history: &HashMap<String, Session>,
    active_id: &str,
    start_task: bool,
    title: &str,
) -> Session {
    history
        .get(active_id)
        .filter(|_| !start_task)
        .cloned()
        .unwrap_or_else(|| Session::new(desktop_state::new_id(), title))
}

fn menu_row_height(font_size: Pixels) -> Pixels {
    (font_size * 1.5 + px(12.)).max(px(28.))
}

pub(super) struct ProjectList {
    roots: Vec<PathBuf>,
    visible: Vec<PathBuf>,
    current: PathBuf,
    empty_label: &'static str,
    current_label: &'static str,
}

impl ProjectList {
    pub(super) fn new(roots: Vec<PathBuf>, current: PathBuf) -> Self {
        Self {
            visible: roots.clone(),
            roots,
            current,
            empty_label: "No matching projects",
            current_label: "Current project",
        }
    }

    fn filter(&mut self, query: &str) {
        let query = query.trim().to_lowercase();
        self.visible = self
            .roots
            .iter()
            .filter(|path| path.to_string_lossy().to_lowercase().contains(&query))
            .cloned()
            .collect();
    }

    pub(super) fn path_at(&self, index: IndexPath) -> Option<&PathBuf> {
        (index.section == 0)
            .then(|| self.visible.get(index.row))
            .flatten()
    }
}

impl ListDelegate for ProjectList {
    type Item = ListItem;

    fn items_count(&self, _: usize, _: &App) -> usize {
        self.visible.len()
    }

    fn perform_search(
        &mut self,
        query: &str,
        _: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Task<()> {
        self.filter(query);
        cx.notify();
        Task::ready(())
    }

    fn set_selected_index(
        &mut self,
        _: Option<IndexPath>,
        _: &mut Window,
        _: &mut Context<ListState<Self>>,
    ) {
    }

    fn render_item(
        &mut self,
        index: IndexPath,
        _: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        let path = self.path_at(index)?;
        let current = path == &self.current;
        Some(
            ListItem::new(("project", index.row))
                .h(menu_row_height(cx.theme().font_size))
                .px(px(8.))
                .py(px(6.))
                .gap(px(8.))
                .rounded(px(6.))
                .line_height(relative(1.5))
                .accessibility_label(if current {
                    format!("{} ({})", path.display(), self.current_label)
                } else {
                    path.display().to_string()
                })
                .child(
                    h_flex()
                        .min_w_0()
                        .flex_1()
                        .gap(px(8.))
                        .child(
                            Icon::new(Glyph::Folder)
                                .size(px(16.))
                                .flex_shrink_0()
                                .text_color(cx.theme().muted_foreground),
                        )
                        .child(div().min_w_0().text_ellipsis().child(project_name(path))),
                )
                .suffix(move |_, cx| {
                    div()
                        .size(px(16.))
                        .flex_shrink_0()
                        .when(current, |indicator| {
                            indicator.child(
                                Icon::new(Glyph::Check)
                                    .size(px(16.))
                                    .text_color(cx.theme().muted_foreground),
                            )
                        })
                })
                .tooltip({
                    let label = path.display().to_string();
                    move |window, cx| Tooltip::new(label.clone()).build(window, cx)
                })
                .text_size(cx.theme().font_size),
        )
    }

    fn render_empty(
        &mut self,
        _: &mut Window,
        _: &mut Context<ListState<Self>>,
    ) -> impl IntoElement {
        div()
            .px(px(8.))
            .py(px(8.))
            .min_w_0()
            .child(self.empty_label)
    }
}

impl PiDesktopApp {
    fn project_intent(&self) -> ProjectIntent {
        ProjectIntent {
            session_id: self.session.id.clone(),
            navigation_generation: self.project_navigation_generation,
            run_generation: self.run_generation,
        }
    }

    pub(super) fn project_menu(&self, trigger: Button, cx: &mut Context<Self>) -> impl IntoElement {
        let weak = cx.entity().downgrade();
        let list = self.project_menu.clone();
        let footer_owner = weak.clone();
        let footer_label = self.tr("打开文件夹...", "Open Folder...");
        let placeholder = self.tr("搜索项目...", "Search projects...");
        let error = self.project_error.clone();
        let busy = self.project_selecting || self.is_executing;
        let empty_label = self.tr("未找到项目", "No matching projects");
        let current_label = self.tr("当前项目", "Current project");
        let font_size = self.ui_text(14.);
        self.project_menu.update(cx, |list, _| {
            list.delegate_mut().empty_label = empty_label;
            list.delegate_mut().current_label = current_label;
        });
        Popover::new("composer-project-menu")
            .p_0()
            .w(px(288.))
            .anchor(Anchor::BottomLeft)
            .offset(px(4.))
            .open(self.project_menu_open)
            .track_focus(&self.project_menu.focus_handle(cx))
            .on_open_change(move |open, window, cx| {
                let _ = weak.update(cx, |this, cx| {
                    this.project_menu_open = *open;
                    if *open {
                        this.project_menu.update(cx, |list, cx| {
                            list.set_query("", window, cx);
                            list.focus(window, cx);
                        });
                    }
                    cx.notify();
                });
            })
            .trigger(
                trigger
                    .track_focus(&self.project_trigger_focus)
                    .disabled(busy),
            )
            .content(move |_, _, cx| {
                v_flex()
                    .w_full()
                    .min_w_0()
                    .when(!error.is_empty(), |panel| {
                        panel.child(
                            div()
                                .px(px(8.))
                                .py(px(4.))
                                .text_color(cx.theme().danger)
                                .child(error.clone()),
                        )
                    })
                    .child(
                        List::new(&list)
                            .search_placeholder(placeholder)
                            .max_h(px(240.))
                            .p(px(4.)),
                    )
                    .child(
                        div()
                            .border_t_1()
                            .border_color(cx.theme().border)
                            .p(px(4.))
                            .child(
                                Button::new("project-open-folder")
                                    .ghost()
                                    .h((font_size * 1.625 + px(8.)).max(px(28.)))
                                    .px(px(8.))
                                    .py(px(4.))
                                    .rounded(px(6.))
                                    .w_full()
                                    .child(
                                        h_flex()
                                            .w_full()
                                            .gap(px(8.))
                                            .text_size(font_size)
                                            .line_height(relative(1.625))
                                            .child(Icon::new(Glyph::FolderPlus).size(px(16.)))
                                            .child(footer_label),
                                    )
                                    .accessibility_label(footer_label)
                                    .disabled(busy)
                                    .on_click({
                                        let owner = footer_owner.clone();
                                        move |_, window, cx| {
                                            let _ = owner.update(cx, |this, cx| {
                                                this.select_project(None, window, cx);
                                            });
                                        }
                                    }),
                            ),
                    )
            })
    }

    fn project_failure(&mut self, message: String, cx: &mut Context<Self>) {
        self.project_error = message;
        self.project_menu_open = true;
        cx.notify();
    }

    pub(super) fn select_project(
        &mut self,
        path: Option<PathBuf>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.project_selecting {
            return;
        }
        if !can_leave(
            self.is_executing,
            self.agentconfig_fetching,
            self.agentconfig_pending_raw.is_some() || self.agentconfig_pending_preview.is_some(),
            self.storage_read_failed,
        ) {
            self.project_failure(
                self.tr(
                    "当前任务、导入或历史读取尚未完成，无法切换项目。",
                    "Finish the current task, import or history recovery before switching projects.",
                )
                .into(),
                cx,
            );
            return;
        }
        self.project_error.clear();
        self.project_menu_open = false;
        self.project_selecting = true;
        let intent = self.project_intent();
        let roots = self.project_menu.read(cx).delegate().roots.clone();
        let picker = path.is_none().then(|| {
            cx.prompt_for_paths(PathPromptOptions {
                files: false,
                directories: true,
                multiple: false,
                prompt: Some(self.tr("打开项目文件夹", "Open project folder").into()),
            })
        });
        self.project_selection = Some(cx.spawn_in(window, async move |this, cx| {
            let result: anyhow::Result<Option<PathBuf>> = async {
                let path = match picker {
                    Some(receiver) => receiver.await??.and_then(|paths| paths.into_iter().next()),
                    None => path,
                };
                let Some(path) = path else { return Ok(None) };
                cx.background_executor()
                    .spawn(async move { resolve_project(&path, &roots).map(Some) })
                    .await
            }
            .await;
            let _ = this.update_in(cx, |this, window, cx| {
                this.project_selecting = false;
                if !selection_is_current(&intent, &this.project_intent(), this.is_executing) {
                    cx.notify();
                    return;
                }
                match result {
                    Ok(Some(path)) => cx.emit(ProjectRequest { path, intent }),
                    Ok(None) => this.project_trigger_focus.focus(window, cx),
                    Err(_) => this.project_failure(
                        this.tr(
                            "无法打开文件夹，请检查路径和访问权限。",
                            "Could not open folder. Check its path and permissions.",
                        )
                        .into(),
                        cx,
                    ),
                }
                cx.notify();
            });
        }));
        cx.notify();
    }

    fn save_before_project_switch(&mut self, cx: &mut Context<Self>) -> bool {
        if !can_leave(
            self.is_executing,
            self.agentconfig_fetching,
            self.agentconfig_pending_raw.is_some() || self.agentconfig_pending_preview.is_some(),
            self.storage_read_failed,
        ) {
            self.project_failure(
                self.tr(
                    "当前项目尚未就绪，无法切换。",
                    "Current project is not ready to switch.",
                )
                .into(),
                cx,
            );
            return false;
        }
        self.drafts.insert(
            self.session.id.clone(),
            self.input_state.read(cx).value().to_string(),
        );
        self.persist_active_session();
        if !self.storage_error.is_empty() {
            self.project_failure(self.storage_error.clone(), cx);
            return false;
        }
        true
    }
}

/// The window and deep-link receiver outlive each workspace-owned agent and store.
pub(super) struct DesktopShell {
    current: Entity<PiDesktopApp>,
    roots: Vec<PathBuf>,
    runtime_keys: HashMap<PathBuf, String>,
    subscription: Subscription,
}

impl DesktopShell {
    pub(super) fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let current = cx.new(|cx| PiDesktopApp::new(window, cx));
        let roots = vec![current.read(cx).workspace_dir.clone()];
        let subscription = cx.subscribe_in(&current, window, Self::switch_project);
        Self {
            current,
            roots,
            runtime_keys: HashMap::new(),
            subscription,
        }
    }

    fn switch_project(
        &mut self,
        source: &Entity<PiDesktopApp>,
        request: &ProjectRequest,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if source != &self.current
            || !selection_is_current(
                &request.intent,
                &self.current.read(cx).project_intent(),
                self.current.read(cx).is_executing,
            )
        {
            return;
        }
        if !self
            .current
            .update(cx, |app, cx| app.save_before_project_switch(cx))
        {
            return;
        }
        // Keep the existing store key for an alias of the current project.
        if self.current.read(cx).workspace_dir == request.path {
            self.current
                .update(cx, |app, cx| app.new_conversation(window, cx));
            return;
        }
        let runtime_key = self.runtime_keys.get(&request.path).cloned();
        let next = cx.new(|cx| {
            PiDesktopApp::for_workspace(request.path.clone(), true, runtime_key, window, cx)
        });
        if next.read(cx).storage_read_failed {
            let error = next.read(cx).storage_error.clone();
            self.current.update(cx, |app, cx| {
                app.project_failure(error, cx);
                app.project_trigger_focus.focus(window, cx);
            });
            return;
        }
        next.update(cx, |app, _| app.persist_active_session());
        if !next.read(cx).storage_error.is_empty() {
            let error = next.read(cx).storage_error.clone();
            self.current.update(cx, |app, cx| {
                app.project_failure(error, cx);
                app.project_trigger_focus.focus(window, cx);
            });
            return;
        }
        if !self.roots.contains(&request.path) {
            self.roots.push(request.path.clone());
        }
        // Session-only credentials stay in memory and can only return to their own root.
        let source = self.current.read(cx);
        self.runtime_keys
            .insert(source.workspace_dir.clone(), source.api_key.clone());
        next.update(cx, |app, cx| {
            let roots = self.roots.clone();
            let current = app.workspace_dir.clone();
            app.project_menu.update(cx, |list, cx| {
                *list.delegate_mut() = ProjectList::new(roots, current);
                cx.notify();
            });
        });
        // Stop source timers/listeners before releasing it or changing focus.
        self.current.update(cx, |app, _| {
            app.save_task = None;
            app.timer = None;
            *app.greeting_refresh.borrow_mut() = None;
            *app.copy_reset.borrow_mut() = None;
            app._subscriptions.clear();
        });
        self.subscription = cx.subscribe_in(&next, window, Self::switch_project);
        self.current = next;
        self.current
            .update(cx, |app, cx| app.focus_composer(window, cx));
        cx.notify();
    }

    pub(super) fn receive_agentconfig_url(
        &mut self,
        url: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.current
            .update(cx, |app, cx| app.receive_agentconfig_url(url, window, cx));
    }
}

impl Render for DesktopShell {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.current.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        can_leave, menu_row_height, project_name, project_session, resolve_project,
        selection_is_current, validate_directory, ProjectIntent, ProjectList,
    };
    use gpui_kit::component::IndexPath;
    use gpui_kit::px;
    use pi_agent_core::Session;
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};

    #[test]
    fn project_search_matches_name_path_and_case_without_changing_identity() {
        let a = PathBuf::from("/projects/Alpha");
        let b = PathBuf::from("/clients/Beta");
        let mut list = ProjectList::new(vec![a.clone(), b.clone()], a.clone());
        for query in ["alpha", " PROJECTS ", "/projects/Al"] {
            list.filter(query);
            assert_eq!(list.visible, vec![a.clone()]);
        }
        list.filter("CLIENTS");
        assert_eq!(list.path_at(IndexPath::new(0)), Some(&b));
        assert_eq!(list.current, a);
        assert!(list.path_at(IndexPath::new(1)).is_none());
        list.filter("missing");
        assert!(list.visible.is_empty());
        list.filter("");
        assert_eq!(list.visible.len(), 2);
    }

    #[test]
    fn selection_results_do_not_follow_task_changes_or_running_work() {
        let request = ProjectIntent {
            session_id: "draft-a".into(),
            navigation_generation: 0,
            run_generation: 0,
        };
        assert!(selection_is_current(&request, &request, false));
        assert!(!selection_is_current(&request, &request, true));
        for current in [
            ProjectIntent {
                session_id: "draft-b".into(),
                ..request.clone()
            },
            ProjectIntent {
                navigation_generation: 2,
                ..request.clone()
            },
            ProjectIntent {
                run_generation: 1,
                ..request.clone()
            },
        ] {
            assert!(!selection_is_current(&request, &current, false));
        }
        for (executing, importing, pending_preview, read_failed) in [
            (true, false, false, false),
            (false, true, false, false),
            (false, false, true, false),
            (false, false, false, true),
        ] {
            assert!(!can_leave(
                executing,
                importing,
                pending_preview,
                read_failed
            ));
        }
        assert!(can_leave(false, false, false, false));
    }

    #[test]
    fn project_paths_reject_files_missing_and_relative_roots() {
        let temp = tempfile::tempdir().unwrap();
        let file = temp.path().join("file");
        std::fs::write(&file, b"fixture").unwrap();
        assert!(validate_directory(&file).is_err());
        assert!(validate_directory(&temp.path().join("missing")).is_err());
        assert!(validate_directory(Path::new(".")).is_err());
        assert_eq!(
            validate_directory(temp.path()).unwrap(),
            temp.path().canonicalize().unwrap()
        );
        assert_eq!(project_name(Path::new("/projects/Alpha")), "Alpha");
        assert_eq!(project_name(Path::new("/")), "/");
    }

    #[cfg(unix)]
    #[test]
    fn known_project_aliases_preserve_the_original_history_key() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("project");
        let alias = temp.path().join("alias");
        std::fs::create_dir(&root).unwrap();
        std::os::unix::fs::symlink(&root, &alias).unwrap();
        assert_eq!(
            resolve_project(&root, std::slice::from_ref(&alias)).unwrap(),
            alias
        );
        assert_eq!(
            resolve_project(&alias, std::slice::from_ref(&root)).unwrap(),
            root
        );
        assert_eq!(
            resolve_project(&alias, &[]).unwrap(),
            root.canonicalize().unwrap()
        );
    }

    #[test]
    fn project_selection_starts_a_new_task_without_replacing_target_history() {
        let previous = Session::new("target-task", "Existing target conversation");
        let mut history = HashMap::new();
        history.insert(previous.id.clone(), previous.clone());
        let resumed = project_session(&history, &previous.id, false, "New task");
        assert_eq!(resumed.id, previous.id);
        let fresh = project_session(&history, &previous.id, true, "New task");
        assert_ne!(fresh.id, previous.id);
        assert_eq!(fresh.title, "New task");
        assert!(fresh.messages.is_empty());
        assert_eq!(history.len(), 1);
        assert_eq!(history[&previous.id].title, previous.title);
    }

    #[test]
    fn project_menu_rows_keep_large_interface_text_inside_the_frame() {
        for font_size in 12..=20 {
            let text = px(font_size as f32);
            assert_eq!(menu_row_height(text), (text * 1.5 + px(12.)).max(px(28.)));
        }
    }
}
