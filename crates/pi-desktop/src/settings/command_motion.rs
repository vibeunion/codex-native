use crate::theme::ThemeColors;
use gpui_kit::base::{transition, Transition};
use gpui_kit::component::button::{Button, ButtonCustomVariant, ButtonVariants};
use gpui_kit::component::{ActiveTheme, Disableable, FocusableExt, StyledExt};
use gpui_kit::{
    div, relative, rgb, rgba, App, Div, ElementId, HoverListenerMode, Hsla, InteractiveElement,
    IntoElement, ParentElement, Pixels, RenderOnce, SharedString, StatefulInteractiveElement,
    StyleRefinement, Styled, Window,
};
use std::time::Duration;

const COLOR_TRAVEL: Duration = Duration::from_millis(150);

#[derive(Clone, Copy)]
pub(super) enum CommandKind {
    Primary,
    Neutral,
    Navigation { selected: bool },
    Back,
}

struct Paint {
    background: Hsla,
    foreground: Hsla,
}

impl CommandKind {
    fn paint(self, colors: &ThemeColors, dark: bool, hovered: bool, disabled: bool) -> Paint {
        let hovered = hovered && !disabled;
        match self {
            Self::Primary => {
                let mut background = rgb(if dark { 0xffffff } else { 0x000000 });
                background.a = if hovered { 0.8 } else { 1. };
                Paint {
                    background: background.into(),
                    foreground: rgb(if dark { 0x000000 } else { 0xffffff }).into(),
                }
            }
            Self::Neutral => {
                let mut background = rgba(colors.hover);
                if !hovered {
                    background.a = 0.;
                }
                Paint {
                    background: background.into(),
                    foreground: rgb(colors.text_primary).into(),
                }
            }
            Self::Navigation { selected } => {
                let mut background = rgba(colors.selected);
                if !selected && !hovered {
                    background.a = 0.;
                }
                Paint {
                    background: background.into(),
                    foreground: rgb(colors.text_primary).into(),
                }
            }
            Self::Back => Paint {
                background: rgba(if hovered {
                    colors.selected
                } else {
                    colors.card_user_bg
                })
                .into(),
                foreground: rgb(if hovered {
                    colors.text_primary
                } else {
                    colors.remote_foreground_muted
                })
                .into(),
            },
        }
    }
}

fn duration(reduced_motion: bool, disabled: bool) -> Duration {
    if reduced_motion || disabled {
        Duration::ZERO
    } else {
        COLOR_TRAVEL
    }
}

fn command_frame(button: Button, paint: &Paint, disabled: bool) -> Button {
    // Instance colors also win over the component's selected/disabled palette.
    button
        .bg(paint.background)
        .text_color(paint.foreground)
        .opacity(if disabled { 0.5 } else { 1. })
        // Pi Desktop clears focus outlines/shadows globally; keep native focus handling.
        .focus_ring(false)
}

fn padding_box_background(
    style: &StyleRefinement,
    background: Hsla,
    rem_size: Pixels,
) -> Option<Div> {
    let border = style.border_widths.top?;
    if border.to_pixels(rem_size) <= Pixels::ZERO
        || [
            style.border_widths.right,
            style.border_widths.bottom,
            style.border_widths.left,
        ]
        .iter()
        .any(|width| *width != Some(border))
    {
        return None;
    }
    let inset = border.to_pixels(rem_size);
    let radius =
        |value: gpui_kit::AbsoluteLength| (value.to_pixels(rem_size) - inset).max(Pixels::ZERO);
    // CSS bg-clip-padding leaves the transparent border outside the fill.
    Some(
        div()
            .absolute()
            .top(inset)
            .right(inset)
            .bottom(inset)
            .left(inset)
            .rounded_tl(radius(style.corner_radii.top_left?))
            .rounded_tr(radius(style.corner_radii.top_right?))
            .rounded_br(radius(style.corner_radii.bottom_right?))
            .rounded_bl(radius(style.corner_radii.bottom_left?))
            .bg(background),
    )
}

fn command_observer(button: &mut Button) -> Div {
    let mut observer = div().h_flex().flex_shrink_0();
    let outer = observer.style();
    let inner = button.style();
    // Parent-facing layout belongs to the shell; paint and activation stay native.
    outer.size.width = inner.size.width.take();
    outer.min_size.width = inner.min_size.width.take();
    outer.max_size.width = inner.max_size.width.take();
    outer.margin = std::mem::take(&mut inner.margin);
    outer.position = inner.position.take();
    outer.inset = std::mem::take(&mut inner.inset);
    outer.align_self = inner.align_self.take();
    outer.flex_basis = inner.flex_basis.take();
    outer.flex_grow = inner.flex_grow.take();
    outer.flex_shrink = inner.flex_shrink.take().or(outer.flex_shrink);
    if outer.size.width.is_some() {
        inner.size.width = Some(relative(1.).into());
    }
    observer
}

#[derive(IntoElement)]
pub(super) struct SettingsCommand {
    id: SharedString,
    button: Button,
    kind: CommandKind,
    colors: ThemeColors,
    reduced_motion: bool,
    disabled: bool,
}

impl SettingsCommand {
    pub(super) fn new(
        id: impl Into<SharedString>,
        button: Button,
        kind: CommandKind,
        colors: &ThemeColors,
        reduced_motion: bool,
    ) -> Self {
        Self {
            id: id.into(),
            button,
            kind,
            colors: *colors,
            reduced_motion,
            disabled: false,
        }
    }

    pub(super) fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for SettingsCommand {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let pointer = window.use_keyed_state(
            (ElementId::Name(self.id.clone()), "settings-command-pointer"),
            cx,
            |_, _| false,
        );
        let paint = self.kind.paint(
            &self.colors,
            cx.theme().is_dark(),
            *pointer.read(cx),
            self.disabled,
        );
        let policy = Transition::new(duration(self.reduced_motion, self.disabled))
            .ease(gpui_kit::base::animation::cubic_bezier(0.4, 0., 0.2, 1.));
        let background = transition(
            (self.id.clone(), "settings-command-background"),
            paint.background,
            policy.clone(),
            window,
            cx,
        );
        let foreground = transition(
            (self.id.clone(), "settings-command-foreground"),
            paint.foreground,
            policy,
            window,
            cx,
        );
        let mut button = self.button.disabled(self.disabled);
        let background_layer =
            padding_box_background(button.style(), background, window.rem_size());
        let clipped = background_layer.is_some();
        let button_background = if clipped {
            rgba(0x00000000).into()
        } else {
            background
        };
        let mut button = command_frame(
            button.custom(
                ButtonCustomVariant::new(cx)
                    // The library reuses this color for hover/active borders.
                    .color(rgba(0x00000000).into())
                    .foreground(foreground)
                    .hover(button_background)
                    .active(button_background),
            ),
            &Paint {
                background: button_background,
                foreground,
            },
            self.disabled && !clipped,
        );
        // A listener-only shell keeps hover accurate during a held native press.
        command_observer(&mut button)
            .id((ElementId::Name(self.id), "settings-command-observer"))
            .hover_listener_mode(HoverListenerMode::InputModalityIndependent)
            .on_hover(move |hovered, window, cx| {
                if pointer.read(cx) != hovered {
                    pointer.update(cx, |state, _| *state = *hovered);
                    window.refresh();
                }
            })
            .opacity(if self.disabled && clipped { 0.5 } else { 1. })
            .children(background_layer)
            .child(button)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        command_frame, command_observer, duration, padding_box_background, CommandKind,
        COLOR_TRAVEL,
    };
    use crate::settings::buttons::{api_key_visibility_button, save_button, secondary_button};
    use crate::theme::ThemeColors;
    use crate::Language;
    use gpui_kit::assets::IconName as Glyph;
    use gpui_kit::base::Interpolate;
    use gpui_kit::component::button::Button;
    use gpui_kit::component::FocusableExt;
    use gpui_kit::{div, px, relative, rgb, rgba, Element, Hsla, Position, Styled};
    use std::time::Duration;

    const KINDS: [CommandKind; 5] = [
        CommandKind::Primary,
        CommandKind::Neutral,
        CommandKind::Navigation { selected: false },
        CommandKind::Navigation { selected: true },
        CommandKind::Back,
    ];

    #[test]
    fn command_focus_paint_does_not_add_a_library_ring_or_change_geometry() {
        for (colors, dark) in [(ThemeColors::light(), false), (ThemeColors::dark(), true)] {
            for kind in KINDS {
                for disabled in [false, true] {
                    let mut button = save_button("Save Preferences", px(14.));
                    assert!(button.is_focus_ring_enabled());
                    let before = button.style().clone();
                    let mut button = command_frame(
                        button,
                        &kind.paint(&colors, dark, false, disabled),
                        disabled,
                    );
                    assert!(!button.is_focus_ring_enabled());
                    let after = button.style();
                    assert_eq!(after.size, before.size);
                    assert_eq!(after.padding, before.padding);
                    assert_eq!(after.margin, before.margin);
                    assert_eq!(after.border_widths, before.border_widths);
                    assert_eq!(after.corner_radii, before.corner_radii);
                }
            }
        }
    }

    #[test]
    fn action_fill_uses_the_padding_box_without_changing_the_button_frame() {
        for language in [Language::Chinese, Language::English] {
            for font_size in 12..=20 {
                for (mut button, radius) in [
                    (
                        save_button(language.text().save_btn, px(font_size as f32)),
                        7.,
                    ),
                    (
                        secondary_button(
                            "padding-box-secondary",
                            "Use in task",
                            Glyph::Plus,
                            px(font_size as f32),
                        ),
                        5.,
                    ),
                    (api_key_visibility_button(true, language), 5.),
                    (api_key_visibility_button(false, language), 5.),
                ] {
                    let before = button.style().clone();
                    let color = Hsla::from(rgba(0xffffffcc));
                    let mut layer =
                        padding_box_background(button.style(), color, px(font_size as f32))
                            .unwrap();
                    assert_eq!(layer.a11y_role(), None);
                    let style = layer.style();
                    assert_eq!(style.position, Some(Position::Absolute));
                    for inset in [
                        style.inset.top,
                        style.inset.right,
                        style.inset.bottom,
                        style.inset.left,
                    ] {
                        assert_eq!(inset, Some(px(1.).into()));
                    }
                    for corner in [
                        style.corner_radii.top_left,
                        style.corner_radii.top_right,
                        style.corner_radii.bottom_right,
                        style.corner_radii.bottom_left,
                    ] {
                        assert_eq!(corner, Some(px(radius).into()));
                    }
                    assert_eq!(style.background, Some(color.into()));
                    assert!(style.size.width.is_none());
                    assert!(style.size.height.is_none());
                    assert!(style.border_widths.top.is_none());
                    assert!(style.padding.left.is_none());
                    assert_eq!(button.style(), &before);
                }
            }
        }
    }

    #[test]
    fn padding_box_leaves_borderless_or_incomplete_frames_on_the_native_path() {
        let color = rgb(0xffffff).into();
        for mut button in [
            Button::new("borderless"),
            Button::new("missing-radius").border_1(),
            Button::new("zero").border_0(),
            Button::new("asymmetric").border_1().border_l_2(),
        ] {
            assert!(padding_box_background(button.style(), color, px(14.)).is_none());
        }
        let mut frame = div().border_1().rounded(px(0.5));
        let mut layer = padding_box_background(frame.style(), color, px(14.)).unwrap();
        assert_eq!(layer.style().corner_radii.top_left, Some(px(0.).into()));
    }

    #[test]
    fn pointer_shell_keeps_save_content_sized_and_leaves_its_frame_on_the_button() {
        for language in [Language::Chinese, Language::English] {
            for font_size in [12., 14., 20.] {
                let mut button = save_button(language.text().save_btn, px(font_size));
                let before = button.style().clone();
                let mut observer = command_observer(&mut button);
                let outer = observer.style();
                assert!(outer.size.width.is_none());
                assert!(outer.size.height.is_none());
                assert_eq!(outer.align_self, before.align_self);
                assert_eq!(outer.flex_shrink, Some(0.));
                assert!(outer.background.is_none());
                assert!(outer.border_widths.top.is_none());
                assert!(outer.padding.left.is_none());
                let inner = button.style();
                assert_eq!(inner.size, before.size);
                assert_eq!(inner.padding, before.padding);
                assert_eq!(inner.border_widths, before.border_widths);
                assert_eq!(inner.corner_radii, before.corner_radii);
                assert!(inner.align_self.is_none());
            }
        }
    }

    #[test]
    fn pointer_shell_preserves_navigation_widths_and_back_margins() {
        for mut button in [
            Button::new("wide").w_full().h(px(32.)),
            Button::new("compact").w(px(40.)).h(px(40.)).mx_auto(),
            Button::new("back").w(px(244.)).h(px(32.)).m(px(4.)),
        ] {
            let before = button.style().clone();
            let mut observer = command_observer(&mut button);
            assert_eq!(observer.style().size.width, before.size.width);
            assert_eq!(observer.style().margin, before.margin);
            assert_eq!(button.style().size.width, Some(relative(1.).into()));
            assert_eq!(button.style().size.height, before.size.height);
            assert!(button.style().margin.top.is_none());
            assert!(button.style().margin.left.is_none());
        }
    }

    #[test]
    fn pointer_shell_owns_absolute_key_placement_without_expanding_the_hit_target() {
        let mut button = api_key_visibility_button(true, Language::English)
            .absolute()
            .right(px(6.))
            .top(px(6.));
        let mut observer = command_observer(&mut button);
        let outer = observer.style();
        assert_eq!(outer.position, Some(Position::Absolute));
        assert_eq!(outer.inset.right, Some(px(6.).into()));
        assert_eq!(outer.inset.top, Some(px(6.).into()));
        assert_eq!(outer.size.width, Some(px(24.).into()));
        assert!(outer.size.height.is_none());
        let inner = button.style();
        assert!(inner.position.is_none());
        assert!(inner.inset.top.is_none());
        assert!(inner.inset.right.is_none());
        assert_eq!(inner.size.width, Some(relative(1.).into()));
        assert_eq!(inner.size.height, Some(px(24.).into()));
    }

    #[test]
    fn primary_keeps_black_white_ink_and_fades_only_the_fill_on_hover() {
        for (colors, dark, background, foreground) in [
            (ThemeColors::light(), false, 0x000000, 0xffffff),
            (ThemeColors::dark(), true, 0xffffff, 0x000000),
        ] {
            let idle = CommandKind::Primary.paint(&colors, dark, false, false);
            let hovered = CommandKind::Primary.paint(&colors, dark, true, false);
            assert_eq!(idle.background, Hsla::from(rgb(background)));
            assert_eq!(hovered.background, Hsla::from(rgb(background).alpha(0.8)));
            assert_eq!(idle.foreground, Hsla::from(rgb(foreground)));
            assert_eq!(hovered.foreground, idle.foreground);
        }
    }

    #[test]
    fn neutral_uses_the_hover_token_instead_of_accent_or_selected() {
        for (colors, dark) in [(ThemeColors::light(), false), (ThemeColors::dark(), true)] {
            let idle = CommandKind::Neutral.paint(&colors, dark, false, false);
            let hovered = CommandKind::Neutral.paint(&colors, dark, true, false);
            assert_eq!(idle.background.a, 0.);
            assert_eq!(hovered.background, Hsla::from(rgba(colors.hover)));
            assert_eq!(idle.foreground, Hsla::from(rgb(colors.text_primary)));
            assert_eq!(hovered.foreground, idle.foreground);
            if dark {
                assert_ne!(hovered.background, Hsla::from(rgba(colors.selected)));
            }
        }
    }

    #[test]
    fn navigation_selection_stays_emphasized_after_the_pointer_leaves() {
        for (colors, dark) in [(ThemeColors::light(), false), (ThemeColors::dark(), true)] {
            for selected in [false, true] {
                let kind = CommandKind::Navigation { selected };
                let idle = kind.paint(&colors, dark, false, false);
                let hovered = kind.paint(&colors, dark, true, false);
                assert_eq!(hovered.background, Hsla::from(rgba(colors.selected)));
                assert_eq!(idle.foreground, Hsla::from(rgb(colors.text_primary)));
                assert_eq!(hovered.foreground, idle.foreground);
                if selected {
                    assert_eq!(idle.background, hovered.background);
                } else {
                    assert_eq!(idle.background.a, 0.);
                }
            }
        }
    }

    #[test]
    fn back_transitions_surface_and_muted_ink_together() {
        for (colors, dark) in [(ThemeColors::light(), false), (ThemeColors::dark(), true)] {
            let idle = CommandKind::Back.paint(&colors, dark, false, false);
            let hovered = CommandKind::Back.paint(&colors, dark, true, false);
            assert_eq!(idle.background, Hsla::from(rgba(colors.card_user_bg)));
            assert_eq!(hovered.background, Hsla::from(rgba(colors.selected)));
            assert_eq!(
                idle.foreground,
                Hsla::from(rgb(colors.remote_foreground_muted))
            );
            assert_eq!(hovered.foreground, Hsla::from(rgb(colors.text_primary)));
            for progress in [0., 0.25, 0.5, 0.75, 1.] {
                let foreground = idle.foreground.interpolate(&hovered.foreground, progress);
                assert_eq!(foreground.a, 1.);
                assert!(foreground.l >= idle.foreground.l.min(hovered.foreground.l));
                assert!(foreground.l <= idle.foreground.l.max(hovered.foreground.l));
            }
        }
    }

    #[test]
    fn disabled_targets_are_idle_even_when_the_pointer_remains_inside() {
        for (colors, dark) in [(ThemeColors::light(), false), (ThemeColors::dark(), true)] {
            for kind in KINDS {
                let idle = kind.paint(&colors, dark, false, false);
                let disabled = kind.paint(&colors, dark, true, true);
                assert_eq!(disabled.background, idle.background);
                assert_eq!(disabled.foreground, idle.foreground);
            }
        }
    }

    #[test]
    fn disabled_commands_dim_the_whole_idle_palette_without_changing_geometry() {
        for (colors, dark) in [(ThemeColors::light(), false), (ThemeColors::dark(), true)] {
            for kind in KINDS {
                for disabled in [false, true] {
                    let paint = kind.paint(&colors, dark, false, disabled);
                    let mut button = command_frame(
                        Button::new("settings-command-frame")
                            .h(px(32.))
                            .px(px(10.))
                            .border_1()
                            .border_color(rgba(0x00000000)),
                        &paint,
                        disabled,
                    );
                    let style = button.style();
                    assert_eq!(style.opacity, Some(if disabled { 0.5 } else { 1. }));
                    assert_eq!(style.background, Some(paint.background.into()));
                    assert_eq!(style.text.color, Some(paint.foreground));
                    assert_eq!(style.size.height, Some(px(32.).into()));
                    assert!(style.size.width.is_none());
                    assert_eq!(style.padding.left, Some(px(10.).into()));
                    assert_eq!(style.padding.right, Some(px(10.).into()));
                    assert_eq!(style.border_color, Some(rgba(0x00000000).into()));
                    assert_eq!(style.border_widths.top, Some(px(1.).into()));
                }
            }
        }
    }

    #[test]
    fn reference_timing_snaps_when_reduced_or_disabled_and_alpha_stays_bounded() {
        assert_eq!(COLOR_TRAVEL, Duration::from_millis(150));
        assert_eq!(duration(false, false), COLOR_TRAVEL);
        assert_eq!(duration(true, false), Duration::ZERO);
        assert_eq!(duration(false, true), Duration::ZERO);
        assert_eq!(duration(true, true), Duration::ZERO);
        for (colors, dark) in [(ThemeColors::light(), false), (ThemeColors::dark(), true)] {
            for kind in KINDS {
                let from = kind.paint(&colors, dark, false, false).background;
                let to = kind.paint(&colors, dark, true, false).background;
                let halfway = from.interpolate(&to, 0.5);
                assert!(halfway.a >= from.a.min(to.a));
                assert!(halfway.a <= from.a.max(to.a));
                assert!((halfway.a - (from.a + to.a) / 2.).abs() < 0.0001);
            }
        }
    }
}
