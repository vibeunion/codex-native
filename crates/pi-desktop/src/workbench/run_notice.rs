use super::{error_banner_tone, ErrorBannerTone};
use crate::i18n::Language;
use crate::theme::ThemeColors;
use crate::PiDesktopApp;
use gpui_kit::assets::IconName as Glyph;
use gpui_kit::base::{transition, Transition};
use gpui_kit::component::button::{Button, ButtonCustomVariant, ButtonVariants};
use gpui_kit::component::notification::Notification;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::{Disableable, Icon, Sizable, StyledExt, WindowExt};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, px, relative, rgb, rgba, AnyElement, App, ClipboardItem, Context, Div, ElementId,
    HoverListenerMode, Hsla, InteractiveElement, IntoElement, ParentElement, Pixels, RenderOnce,
    Role, Stateful, StatefulInteractiveElement, Styled, Window,
};
use std::time::Duration;

struct CopyErrorNotice;

fn column(width: Pixels, empty: bool) -> Div {
    div()
        .w_full()
        .max_w(width)
        .min_w_0()
        .flex_shrink_0()
        .mb(px(24.))
        .when(!empty, |column| column.px(px(16.)))
}

fn surface(font_size: Pixels, colors: &ThemeColors) -> Div {
    div()
        .w_full()
        .min_w_0()
        .flex_shrink_0()
        .h_flex()
        .flex_wrap()
        .items_center()
        .gap(px(8.))
        .px(px(12.))
        .py(px(8.))
        .rounded(px(12.))
        .border_1()
        .border_color(rgba(colors.settings_border))
        .bg(rgba(colors.card_user_bg))
        .text_size(font_size)
        .line_height(relative(1.5))
        .text_color(rgb(colors.text_primary))
}

fn message_label(text: String) -> Div {
    div()
        .min_w_0()
        .truncate()
        .font_medium()
        .child(message_summary(&text))
}

fn message_summary(text: &str) -> String {
    // GPUI nowrap keeps hard line breaks; CSS nowrap collapses them in the reference.
    text.split_ascii_whitespace().collect::<Vec<_>>().join(" ")
}

fn message_glyph(tone: ErrorBannerTone) -> Glyph {
    match tone {
        ErrorBannerTone::Stopped => Glyph::Square,
        ErrorBannerTone::Failure => Glyph::Info,
    }
}

fn message(tone: ErrorBannerTone, text: String, colors: &ThemeColors) -> Stateful<Div> {
    let tooltip_text = text.clone();
    div()
        .id("run-notice-message")
        .h_flex()
        .items_center()
        .flex_1()
        .min_w_0()
        .gap(px(8.))
        .text_color(match tone {
            ErrorBannerTone::Stopped => rgba(colors.foreground_subtle),
            ErrorBannerTone::Failure => rgb(colors.text_primary),
        })
        .tooltip(move |window, cx| Tooltip::new(tooltip_text.clone()).build(window, cx))
        .child(Icon::new(message_glyph(tone)).size(px(16.)).flex_shrink_0())
        .child(message_label(text))
}

fn action_label(label: &'static str, font_size: Pixels) -> Div {
    div()
        .text_size(font_size)
        .line_height(relative(1.625))
        .font_normal()
        .whitespace_nowrap()
        .child(label)
}

fn outline_button(id: &'static str, disabled: bool, colors: &ThemeColors) -> Button {
    Styled::rounded(Button::new(id).small(), px(6.))
        .h(px(24.))
        .border_1()
        .border_color(rgba(colors.settings_border))
        .bg(rgba(0x00000000))
        .text_color(rgb(colors.text_primary))
        .flex_shrink_0()
        .disabled(disabled)
        .when(disabled, |button| button.opacity(0.5))
}

fn retry_button(
    label: &'static str,
    font_size: Pixels,
    disabled: bool,
    colors: &ThemeColors,
) -> Button {
    outline_button("retry-run", disabled, colors)
        .px(px(8.))
        .accessibility_label(label)
        .child(action_label(label, font_size))
}

fn copy_button(label: &'static str, font_size: Pixels, colors: &ThemeColors) -> Button {
    outline_button("copy-run-error", false, colors)
        .px(px(8.))
        .accessibility_label(label)
        .child(icon_action_content(Glyph::Copy, label, font_size))
}

fn icon_action_content(glyph: Glyph, label: &'static str, font_size: Pixels) -> Div {
    div()
        .h_flex()
        .items_center()
        .flex_shrink_0()
        .gap(px(6.))
        .child(Icon::new(glyph).size(px(14.)).flex_shrink_0())
        .child(action_label(label, font_size))
}

fn error_copy_text(error: &str, language: Language) -> String {
    let (heading, summary) = match language {
        Language::Chinese => ("Pi 报错信息", "报错摘要"),
        Language::English => ("Pi Error Info", "Error Summary"),
    };
    // The native error is already the full payload; do not invent detail or trace fields.
    format!("{heading}\n\n{summary}\n{error}\n")
}

fn dismiss_button(
    id: &'static str,
    label: &'static str,
    disabled: bool,
    colors: &ThemeColors,
) -> Button {
    let mut button = outline_button(id, disabled, colors)
        .w(px(24.))
        .p_0()
        .accessibility_label(label)
        .child(Icon::new(Glyph::X).size(px(16.)).flex_shrink_0());
    button
        .interactivity()
        .tooltip(move |window, cx| Tooltip::new(label).build(window, cx));
    button
}

fn setup_notice_visible(show: bool, api_key: &str) -> bool {
    show && api_key.trim().is_empty()
}

fn setup_wrapper() -> Div {
    div()
        .w_full()
        .min_w_0()
        .flex_shrink_0()
        .px(px(16.))
        .h_flex()
        .justify_center()
}

fn setup_message(language: Language) -> &'static str {
    match language {
        Language::Chinese => "当前没有可用模型。请配置自定义模型。",
        Language::English => "No model available. Set a custom model.",
    }
}

fn configure_button(label: &'static str, font_size: Pixels, colors: &ThemeColors) -> Button {
    outline_button("onboard-settings", false, colors)
        .px(px(8.))
        .accessibility_label(label)
        .child(icon_action_content(Glyph::Settings, label, font_size))
}

fn dismiss_error(error: &mut Option<String>, executing: bool) -> bool {
    // Completion still needs the error to classify the run and hold queued prompts.
    !executing && error.take().is_some()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct OutlinePaint {
    background: Hsla,
    border: Hsla,
}

fn outline_paint(colors: &ThemeColors, hovered: bool, disabled: bool) -> OutlinePaint {
    let hovered = hovered && !disabled;
    OutlinePaint {
        background: rgb(colors.dock_bg)
            .alpha(if hovered { 0.5 } else { 0. })
            .into(),
        border: rgba(if hovered {
            colors.dock_border_active
        } else {
            colors.settings_border
        })
        .into(),
    }
}

fn outline_transition_duration(reduced_motion: bool, disabled: bool) -> Duration {
    if reduced_motion || disabled {
        Duration::ZERO
    } else {
        Duration::from_millis(150)
    }
}

fn action_observer() -> Div {
    div().h_flex().flex_shrink_0()
}

fn action_key(id: &'static str, phase: &'static str) -> ElementId {
    (ElementId::from(id), phase).into()
}

#[derive(IntoElement)]
struct NoticeAction {
    id: &'static str,
    button: Button,
    colors: ThemeColors,
    disabled: bool,
    reduced_motion: bool,
}

impl RenderOnce for NoticeAction {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let pointer = window.use_keyed_state(action_key(self.id, "pointer"), cx, |_, _| false);
        let target = outline_paint(&self.colors, *pointer.read(cx), self.disabled);
        let policy = Transition::new(outline_transition_duration(
            self.reduced_motion,
            self.disabled,
        ))
        .ease(gpui_kit::base::animation::cubic_bezier(0.4, 0., 0.2, 1.));
        let background = transition(
            action_key(self.id, "background"),
            target.background,
            policy.clone(),
            window,
            cx,
        );
        let border = transition(
            action_key(self.id, "border"),
            target.border,
            policy,
            window,
            cx,
        );
        // Custom's color also supplies its hover border; instance fill stays separate.
        let button = self
            .button
            .custom(
                ButtonCustomVariant::new(cx)
                    .color(border)
                    .foreground(rgb(self.colors.text_primary).into())
                    .hover(background)
                    .active(background),
            )
            .bg(background)
            .border_color(border);
        // The observer has no click listener, so a held press does not clear hover.
        action_observer()
            .id(action_key(self.id, "pointer-observer"))
            .hover_listener_mode(HoverListenerMode::InputModalityIndependent)
            .on_hover(move |hovered, window, cx| {
                if pointer.read(cx) != hovered {
                    pointer.update(cx, |state, _| *state = *hovered);
                    window.refresh();
                }
            })
            .child(button)
    }
}

fn notice_role(tone: ErrorBannerTone) -> Role {
    match tone {
        ErrorBannerTone::Stopped => Role::Status,
        ErrorBannerTone::Failure => Role::Alert,
    }
}

impl PiDesktopApp {
    pub(super) fn model_setup_notice(
        &self,
        width: Pixels,
        colors: &ThemeColors,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !setup_notice_visible(self.show_onboard_banner, &self.api_key) {
            return None;
        }
        Some(
            setup_wrapper()
                .child(
                    column(width.min(px(672.)), true)
                        .id("draft-onboard-notice")
                        .role(Role::Alert)
                        .child(
                            surface(self.ui_text(14.), colors)
                                .child(message(
                                    ErrorBannerTone::Failure,
                                    setup_message(self.language).into(),
                                    colors,
                                ))
                                .child(NoticeAction {
                                    id: "onboard-settings",
                                    button: configure_button(
                                        self.tr("配置", "Set"),
                                        self.ui_text(14.),
                                        colors,
                                    )
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.show_settings = true;
                                        this.settings_section = crate::settings::Section::Providers;
                                        this.preferences.settings_section =
                                            Some(crate::settings::Section::Providers.key().into());
                                        this.settings_error.clear();
                                        this.settings_saved = false;
                                        this.root_focus.focus(window, cx);
                                        this.persist_active_session();
                                        cx.notify();
                                    })),
                                    colors: *colors,
                                    disabled: false,
                                    reduced_motion: self.preferences.reduced_motion,
                                })
                                .child(NoticeAction {
                                    id: "onboard-dismiss",
                                    button: dismiss_button(
                                        "onboard-dismiss",
                                        self.tr("关闭错误提示", "Dismiss error"),
                                        false,
                                        colors,
                                    )
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.show_onboard_banner = false;
                                        this.root_focus.focus(window, cx);
                                        cx.notify();
                                    })),
                                    colors: *colors,
                                    disabled: false,
                                    reduced_motion: self.preferences.reduced_motion,
                                }),
                        ),
                )
                .into_any_element(),
        )
    }

    pub(super) fn run_notice(
        &self,
        width: Pixels,
        empty: bool,
        colors: &ThemeColors,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let text = self.run_error.clone()?;
        let tone = error_banner_tone(self.run_stopped);
        Some(
            column(width, empty)
                .id("run-notice")
                .role(notice_role(tone))
                .child(
                    surface(self.ui_text(14.), colors)
                        .child(message(tone, text, colors))
                        .child(NoticeAction {
                            id: "copy-run-error",
                            button: copy_button(self.tr("复制", "Copy"), self.ui_text(14.), colors)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    let Some(error) = this.run_error.as_deref() else {
                                        return;
                                    };
                                    cx.write_to_clipboard(ClipboardItem::new_string(
                                        error_copy_text(error, this.language),
                                    ));
                                    window.push_notification(
                                        Notification::new().id::<CopyErrorNotice>().message(
                                            this.tr(
                                                "已复制完整报错信息",
                                                "Full error details copied",
                                            ),
                                        ),
                                        cx,
                                    );
                                })),
                            colors: *colors,
                            disabled: false,
                            reduced_motion: self.preferences.reduced_motion,
                        })
                        .child(NoticeAction {
                            id: "retry-run",
                            button: retry_button(
                                self.tr("重试", "Retry"),
                                self.ui_text(14.),
                                self.is_executing,
                                colors,
                            )
                            .on_click(cx.listener(|this, _, _, cx| this.retry_run(cx))),
                            colors: *colors,
                            disabled: self.is_executing,
                            reduced_motion: self.preferences.reduced_motion,
                        })
                        .child(NoticeAction {
                            id: "dismiss-run-notice",
                            button: dismiss_button(
                                "dismiss-run-notice",
                                self.tr("关闭错误提示", "Dismiss error"),
                                self.is_executing,
                                colors,
                            )
                            .on_click(cx.listener(
                                |this, _, window, cx| {
                                    if dismiss_error(&mut this.run_error, this.is_executing) {
                                        this.root_focus.focus(window, cx);
                                        cx.notify();
                                    }
                                },
                            )),
                            colors: *colors,
                            disabled: self.is_executing,
                            reduced_motion: self.preferences.reduced_motion,
                        }),
                )
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{FontWeight, WhiteSpace};

    #[test]
    fn notice_column_uses_the_composer_inset_and_reference_bottom_gap() {
        for empty in [false, true] {
            let mut column = column(px(672.), empty);
            let style = column.style();
            assert_eq!(style.size.width, Some(relative(1.).into()));
            assert_eq!(style.max_size.width, Some(px(672.).into()));
            assert_eq!(style.min_size.width, Some(px(0.).into()));
            assert_eq!(style.margin.bottom, Some(px(24.).into()));
            assert_eq!(style.padding.left, (!empty).then(|| px(16.).into()));
            assert_eq!(style.padding.right, (!empty).then(|| px(16.).into()));
            assert_eq!(style.flex_shrink, Some(0.));
        }
    }

    #[test]
    fn failure_surface_is_neutral_and_wraps_its_separate_actions() {
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            for size in 12..=20 {
                let font_size = px(size as f32);
                let mut surface = surface(font_size, &colors);
                let style = surface.style();
                assert_eq!(style.background, Some(rgba(colors.card_user_bg).into()));
                assert_eq!(
                    style.border_color,
                    Some(rgba(colors.settings_border).into())
                );
                assert_eq!(style.corner_radii.top_left, Some(px(12.).into()));
                assert_eq!(style.padding.left, Some(px(12.).into()));
                assert_eq!(style.padding.top, Some(px(8.).into()));
                assert_eq!(style.gap.width, Some(px(8.).into()));
                assert_eq!(style.flex_wrap, Some(gpui_kit::FlexWrap::Wrap));
                assert_eq!(style.text.font_size, Some(font_size.into()));
                assert_eq!(style.text.line_height, Some(relative(1.5)));
                assert_eq!(style.text.color, Some(rgb(colors.text_primary).into()));
                assert_eq!(style.flex_shrink, Some(0.));
                assert!(style.size.height.is_none());
                assert!(style.box_shadow.is_none());
            }
        }
    }

    #[test]
    fn stopped_and_failed_runs_keep_distinct_glyphs_and_announcements() {
        assert_eq!(message_glyph(ErrorBannerTone::Stopped), Glyph::Square);
        assert_eq!(message_glyph(ErrorBannerTone::Failure), Glyph::Info);
        assert_eq!(notice_role(ErrorBannerTone::Stopped), Role::Status);
        assert_eq!(notice_role(ErrorBannerTone::Failure), Role::Alert);
        for text in ["模型请求失败", "The model request failed"] {
            let mut label = message_label(text.into());
            let style = label.style();
            assert_eq!(style.min_size.width, Some(px(0.).into()));
            assert_eq!(style.text.font_weight, Some(FontWeight::MEDIUM));
            assert_eq!(style.text.white_space, Some(WhiteSpace::Nowrap));
            assert!(style.size.height.is_none());
        }
    }

    #[test]
    fn summary_collapses_hard_breaks_without_altering_nonbreaking_or_cjk_text() {
        assert_eq!(
            message_summary("  请求失败。\r\n第二行：\t完整详情。 \n "),
            "请求失败。 第二行： 完整详情。"
        );
        assert_eq!(message_summary("Keep\u{a0}together"), "Keep\u{a0}together");
        assert_eq!(message_summary(" \t\r\n"), "");
    }

    #[test]
    fn retry_is_a_small_text_only_action_at_every_interface_size() {
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            for size in 12..=20 {
                let font_size = px(size as f32);
                for label in ["重试", "Retry"] {
                    for disabled in [false, true] {
                        let mut button = retry_button(label, font_size, disabled, &colors);
                        let style = button.style();
                        assert_eq!(style.size.height, Some(px(24.).into()));
                        assert_eq!(style.padding.left, Some(px(8.).into()));
                        assert_eq!(style.padding.right, Some(px(8.).into()));
                        assert_eq!(style.corner_radii.top_left, Some(px(6.).into()));
                        assert_eq!(style.border_widths.top, Some(px(1.).into()));
                        assert_eq!(style.background, Some(rgba(0x00000000).into()));
                        assert_eq!(style.text.color, Some(rgb(colors.text_primary).into()));
                        assert_eq!(style.opacity, disabled.then_some(0.5));
                        assert!(style.size.width.is_none());
                    }
                    let mut label = action_label(label, font_size);
                    let style = label.style();
                    assert_eq!(style.text.font_size, Some(font_size.into()));
                    assert_eq!(style.text.line_height, Some(relative(1.625)));
                    assert_eq!(style.text.font_weight, Some(FontWeight::NORMAL));
                    assert_eq!(style.text.white_space, Some(WhiteSpace::Nowrap));
                    assert!(style.size.height.is_none());
                    assert!(style.size.width.is_none());
                }
            }
        }
    }

    #[test]
    fn retry_hover_uses_half_input_fill_and_the_reference_border() {
        for (colors, input, border, hover_border) in [
            (ThemeColors::light(), 0xffffff, 0x0d0d0d1a, 0x0d0d0d26),
            (ThemeColors::dark(), 0x2b2b2b, 0xffffff1a, 0xffffff26),
        ] {
            let idle = outline_paint(&colors, false, false);
            let hovered = outline_paint(&colors, true, false);
            assert_eq!(idle.background, Hsla::from(rgb(input).alpha(0.)));
            assert_eq!(hovered.background, Hsla::from(rgb(input).alpha(0.5)));
            assert_eq!(idle.border, Hsla::from(rgba(border)));
            assert_eq!(hovered.border, Hsla::from(rgba(hover_border)));
            assert_ne!(hovered.background, Hsla::from(rgba(colors.hover)));
        }
    }

    #[test]
    fn disabled_retry_stays_idle_even_with_the_pointer_inside() {
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            let idle = outline_paint(&colors, false, false);
            assert_eq!(outline_paint(&colors, false, true), idle);
            assert_eq!(outline_paint(&colors, true, true), idle);
        }
    }

    #[test]
    fn retry_color_motion_snaps_when_reduced_or_disabled() {
        assert_eq!(
            outline_transition_duration(false, false),
            Duration::from_millis(150)
        );
        for (reduced, disabled) in [(true, false), (false, true), (true, true)] {
            assert_eq!(
                outline_transition_duration(reduced, disabled),
                Duration::ZERO
            );
        }
    }

    #[test]
    fn retry_pointer_observer_has_no_second_frame_or_fixed_dimensions() {
        let mut observer = action_observer();
        let style = observer.style();
        assert_eq!(style.flex_shrink, Some(0.));
        assert!(style.size.width.is_none());
        assert!(style.size.height.is_none());
        assert!(style.background.is_none());
        assert!(style.border_color.is_none());
        assert!(style.padding.left.is_none());
        assert!(style.padding.top.is_none());
        assert!(style.margin.bottom.is_none());
    }

    #[test]
    fn notice_actions_keep_distinct_typed_state_and_transition_keys() {
        let phases = ["pointer", "pointer-observer", "background", "border"];
        let keys: std::collections::HashSet<_> = [
            "copy-run-error",
            "retry-run",
            "dismiss-run-notice",
            "onboard-settings",
            "onboard-dismiss",
        ]
        .into_iter()
        .flat_map(|id| phases.into_iter().map(move |phase| action_key(id, phase)))
        .collect();
        assert_eq!(keys.len(), 20);
        assert_eq!(
            action_key("retry-run", "pointer"),
            action_key("retry-run", "pointer")
        );
    }

    #[test]
    fn copy_keeps_reference_geometry_without_disabled_styling() {
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            for size in 12..=20 {
                for label in ["复制", "Copy"] {
                    let mut button = copy_button(label, px(size as f32), &colors);
                    let style = button.style();
                    assert_eq!(style.size.height, Some(px(24.).into()));
                    assert_eq!(style.padding.left, Some(px(8.).into()));
                    assert_eq!(style.padding.right, Some(px(8.).into()));
                    assert_eq!(style.corner_radii.top_left, Some(px(6.).into()));
                    assert_eq!(style.border_widths.top, Some(px(1.).into()));
                    assert_eq!(
                        style.border_color,
                        Some(rgba(colors.settings_border).into())
                    );
                    assert_eq!(style.background, Some(rgba(0x00000000).into()));
                    assert_eq!(style.text.color, Some(rgb(colors.text_primary).into()));
                    assert_eq!(style.flex_shrink, Some(0.));
                    assert!(style.opacity.is_none());
                    assert!(style.size.width.is_none());
                    let mut content = icon_action_content(Glyph::Copy, label, px(size as f32));
                    let style = content.style();
                    assert_eq!(style.gap.width, Some(px(6.).into()));
                    assert_eq!(style.align_items, Some(gpui_kit::AlignItems::Center));
                    assert_eq!(style.flex_shrink, Some(0.));
                    assert!(style.size.width.is_none());
                    assert!(style.size.height.is_none());
                }
            }
        }
    }

    #[test]
    fn copied_error_uses_the_reference_template_without_fabricated_diagnostics() {
        assert_eq!(
            error_copy_text("模型请求失败", Language::Chinese),
            "Pi 报错信息\n\n报错摘要\n模型请求失败\n"
        );
        assert_eq!(
            error_copy_text("The model request failed", Language::English),
            "Pi Error Info\n\nError Summary\nThe model request failed\n"
        );
    }

    #[test]
    fn copied_error_preserves_multiline_unicode_and_long_untruncated_payloads() {
        let error = format!("  请求失败：测试\r\n{}\n\n", "detail: ".repeat(1024));
        for language in [Language::Chinese, Language::English] {
            let copy = error_copy_text(&error, language);
            assert!(copy.ends_with(&format!("{error}\n")));
            assert_eq!(copy.matches(&error).count(), 1);
        }
    }

    #[test]
    fn model_setup_notice_does_not_warn_after_configuration_or_dismissal() {
        for api_key in ["", " ", "\t\r\n"] {
            assert!(setup_notice_visible(true, api_key));
            assert!(!setup_notice_visible(false, api_key));
        }
        for show in [true, false] {
            assert!(!setup_notice_visible(show, "configured-in-memory"));
        }
        assert_eq!(
            setup_message(Language::Chinese),
            "当前没有可用模型。请配置自定义模型。"
        );
        assert_eq!(
            setup_message(Language::English),
            "No model available. Set a custom model."
        );
    }

    #[test]
    fn model_setup_notice_uses_the_same_draft_gutters_as_the_composer() {
        let mut wrapper = setup_wrapper();
        assert_eq!(wrapper.style().padding.left, Some(px(16.).into()));
        assert_eq!(wrapper.style().padding.right, Some(px(16.).into()));
        assert_eq!(wrapper.style().size.width, Some(relative(1.).into()));
        assert_eq!(wrapper.style().min_size.width, Some(px(0.).into()));
        assert_eq!(wrapper.style().flex_shrink, Some(0.));
    }

    #[test]
    fn model_configuration_action_keeps_reference_outline_and_content_geometry() {
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            for size in 12..=20 {
                for label in ["配置", "Set"] {
                    let font_size = px(size as f32);
                    let mut button = configure_button(label, font_size, &colors);
                    let style = button.style();
                    assert_eq!(style.size.height, Some(px(24.).into()));
                    assert_eq!(style.padding.left, Some(px(8.).into()));
                    assert_eq!(style.padding.right, Some(px(8.).into()));
                    assert_eq!(style.corner_radii.top_left, Some(px(6.).into()));
                    assert_eq!(style.border_widths.top, Some(px(1.).into()));
                    assert_eq!(style.background, Some(rgba(0x00000000).into()));
                    assert_eq!(
                        style.border_color,
                        Some(rgba(colors.settings_border).into())
                    );
                    assert_eq!(style.flex_shrink, Some(0.));
                    assert!(style.size.width.is_none());
                    assert!(style.opacity.is_none());
                    let mut content = icon_action_content(Glyph::Settings, label, font_size);
                    assert_eq!(content.style().gap.width, Some(px(6.).into()));
                }
            }
        }
    }

    #[test]
    fn dismiss_keeps_reference_icon_geometry_in_both_themes_and_disabled_states() {
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            for label in ["关闭错误提示", "Dismiss error"] {
                for disabled in [false, true] {
                    let mut button = dismiss_button("dismiss-run-notice", label, disabled, &colors);
                    let style = button.style();
                    assert_eq!(style.size.width, Some(px(24.).into()));
                    assert_eq!(style.size.height, Some(px(24.).into()));
                    assert_eq!(style.padding.left, Some(px(0.).into()));
                    assert_eq!(style.padding.right, Some(px(0.).into()));
                    assert_eq!(style.padding.top, Some(px(0.).into()));
                    assert_eq!(style.padding.bottom, Some(px(0.).into()));
                    assert_eq!(style.corner_radii.top_left, Some(px(6.).into()));
                    assert_eq!(style.border_widths.top, Some(px(1.).into()));
                    assert_eq!(
                        style.border_color,
                        Some(rgba(colors.settings_border).into())
                    );
                    assert_eq!(style.background, Some(rgba(0x00000000).into()));
                    assert_eq!(style.text.color, Some(rgb(colors.text_primary).into()));
                    assert_eq!(style.flex_shrink, Some(0.));
                    assert_eq!(style.opacity, disabled.then_some(0.5));
                }
            }
        }
    }

    #[test]
    fn dismiss_cannot_clear_an_error_before_completion_or_cancellation_cleanup() {
        let mut error = Some("Run failed".into());
        assert!(!dismiss_error(&mut error, true));
        assert_eq!(error.as_deref(), Some("Run failed"));
        assert!(dismiss_error(&mut error, false));
        assert!(error.is_none());
        assert!(!dismiss_error(&mut error, false));
    }

    #[test]
    fn dismiss_does_not_resume_an_interrupted_queue_or_change_a_failed_goal() {
        let mut state = crate::task_state::TaskState::default();
        state.enqueue("Keep this pending".into());
        state.set_goal("Preserve the failed goal".into());
        state.goal.as_mut().unwrap().status = "failed".into();
        state.interrupted("Run failed".into());
        let mut error = state.error.clone();

        assert!(dismiss_error(&mut error, false));
        // Subsequent session capture may copy the dismissed error back to task state.
        state.error = error;
        assert!(state.queue_paused);
        assert!(state.next_prompt().is_none());
        assert_eq!(
            state.queue.front().map(String::as_str),
            Some("Keep this pending")
        );
        let goal = state.goal.as_ref().unwrap();
        assert_eq!(goal.objective, "Preserve the failed goal");
        assert_eq!(goal.status, "failed");
        assert_eq!(goal.iteration, 1);
    }
}
