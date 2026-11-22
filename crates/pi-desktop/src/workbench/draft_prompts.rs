use crate::theme::ThemeColors;
use gpui_kit::assets::IconName as Glyph;
use gpui_kit::base::{
    animate_keyframes, transition, Easing, Keyframe, Keyframes, SignedDuration, Timing, Transition,
};
use gpui_kit::component::button::{Button, ButtonCustomVariant, ButtonVariants};
use gpui_kit::component::{Disableable, Icon, Sizable, StyledExt};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, point, px, rgb, rgba, App, Bounds, Div, ElementId, FocusHandle, HoverListenerMode, Hsla,
    InteractiveElement, IntoElement, ParentElement, Pixels, RenderOnce, ScrollHandle, SharedString,
    Stateful, StatefulInteractiveElement, Styled, Window,
};
use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

pub(super) fn wrapper() -> Div {
    div()
        .w_full()
        .min_w_0()
        .flex_shrink_0()
        .mt(px(24.))
        .px(px(16.))
        .h_flex()
        .justify_center()
}

pub(super) fn viewport(width: Pixels) -> Stateful<Div> {
    div()
        .id("draft-suggested-prompts")
        .w_full()
        .max_w(width)
        .min_w_0()
        .h_flex()
        .overflow_x_scroll()
}

pub(super) fn strip() -> Div {
    // Auto flex margins center short collections and become zero when they overflow.
    div()
        .h_flex()
        .flex_shrink_0()
        .mx_auto()
        .items_center()
        .gap(px(16.))
}

fn button_frame(button: Button, disabled: bool) -> Button {
    Styled::rounded(button.large(), px(8.))
        .h(px(32.))
        .px(px(12.))
        .border_1()
        .flex_shrink_0()
        .overflow_hidden()
        .disabled(disabled)
        .when(disabled, |button| button.opacity(0.5))
}

fn label(text: &'static str, font_size: Pixels, foreground: Hsla) -> Div {
    div()
        .min_w_0()
        .max_w(px(256.))
        .truncate()
        .text_size(font_size)
        .line_height(px(18.))
        .font_normal()
        .text_color(foreground)
        .child(text)
}

fn content(text: &'static str, glyph: Glyph, font_size: Pixels, foreground: Hsla) -> Div {
    div()
        .h_flex()
        .items_center()
        .min_w_0()
        .gap(px(6.))
        .child(
            Icon::new(glyph)
                .size(px(16.))
                .flex_shrink_0()
                .text_color(foreground),
        )
        .child(label(text, font_size, foreground))
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Paint {
    background: Hsla,
    border: Hsla,
    foreground: Hsla,
}

fn paint(colors: &ThemeColors, hovered: bool, disabled: bool) -> Paint {
    let hovered = hovered && !disabled;
    Paint {
        background: rgb(colors.dock_bg)
            .alpha(if hovered { 0.5 } else { 0. })
            .into(),
        border: rgba(if hovered {
            colors.dock_border_active
        } else {
            colors.settings_border
        })
        .into(),
        foreground: rgb(colors.text_primary)
            .alpha(if hovered { 1. } else { 0.7 })
            .into(),
    }
}

fn duration(reduced_motion: bool, disabled: bool) -> Duration {
    if reduced_motion || disabled {
        Duration::ZERO
    } else {
        Duration::from_millis(150)
    }
}

fn entrance_timing(ordinal: usize, reduced_motion: bool) -> Timing {
    if reduced_motion {
        return Timing::new(Duration::ZERO);
    }
    Timing::new(Duration::from_millis(260))
        .delay(SignedDuration::positive(Duration::from_millis(
            (ordinal as u64).saturating_mul(65),
        )))
        .ease(Easing::CubicBezier {
            x1: 0.22,
            y1: 1.,
            x2: 0.36,
            y2: 1.,
        })
}

fn entrance_clip(progress: f32) -> Div {
    let progress = progress.clamp(0., 1.);
    // Move the clip with the button, matching CSS clip-path followed by translateY.
    div()
        .h_flex()
        .items_start()
        .flex_shrink_0()
        .min_h_0()
        .h(px(32. * progress))
        .relative()
        .top(px(-12. * (1. - progress)))
        .overflow_hidden()
        .opacity(progress)
}

fn action_slot() -> Div {
    div().h_flex().h(px(32.)).flex_shrink_0()
}

fn reveal_x(
    viewport: Bounds<Pixels>,
    item: Bounds<Pixels>,
    current: Pixels,
    max: Pixels,
) -> Pixels {
    reveal_offset(
        viewport.left(),
        viewport.size.width,
        item.left(),
        item.size.width,
        current,
        max,
    )
}

fn reveal_offset(
    start: Pixels,
    length: Pixels,
    item_start: Pixels,
    item_length: Pixels,
    current: Pixels,
    max: Pixels,
) -> Pixels {
    // The reference browser keeps partially exposed chips stationary on focus.
    let next = if item_start + item_length + current <= start {
        start - item_start
    } else if item_start + current >= start + length {
        if item_length > length {
            start - item_start
        } else {
            start + length - item_start - item_length
        }
    } else {
        current
    };
    next.clamp(-max.max(px(0.)), px(0.))
}

struct FocusReveal {
    focus: FocusHandle,
    was_focused: Rc<Cell<bool>>,
}

#[derive(IntoElement)]
pub(super) struct PromptAction {
    pub id: SharedString,
    pub button: Button,
    pub label: &'static str,
    pub glyph: Glyph,
    pub font_size: Pixels,
    pub colors: ThemeColors,
    pub disabled: bool,
    pub reduced_motion: bool,
    pub ordinal: usize,
    pub scroll: ScrollHandle,
    pub outer_scroll: ScrollHandle,
}

impl RenderOnce for PromptAction {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let key = ElementId::Name(self.id);
        let pointer = window.use_keyed_state((key.clone(), "prompt-pointer"), cx, |_, _| false);
        let reveal =
            window.use_keyed_state((key.clone(), "prompt-reveal"), cx, |_, cx| FocusReveal {
                focus: cx.focus_handle().tab_stop(false),
                was_focused: Rc::new(Cell::new(false)),
            });
        let focus = reveal.read(cx).focus.clone();
        let measured_focus = focus.clone();
        let was_focused = reveal.read(cx).was_focused.clone();
        let measure_scroll = self.scroll;
        let measure_outer_scroll = self.outer_scroll;
        let target = paint(&self.colors, *pointer.read(cx), self.disabled);
        let policy = Transition::new(duration(self.reduced_motion, self.disabled))
            .ease(gpui_kit::base::animation::cubic_bezier(0.4, 0., 0.2, 1.));
        let background = transition(
            (key.clone(), "prompt-background"),
            target.background,
            policy.clone(),
            window,
            cx,
        );
        let border = transition(
            (key.clone(), "prompt-border"),
            target.border,
            policy.clone(),
            window,
            cx,
        );
        let foreground = transition(
            (key.clone(), "prompt-foreground"),
            target.foreground,
            policy,
            window,
            cx,
        );
        let frames = Keyframes::try_new([Keyframe::new(0., 0.), Keyframe::new(1., 1.)])
            .expect("prompt entrance has fixed ordered endpoints");
        let progress = animate_keyframes(
            (key.clone(), "prompt-waterfall"),
            &frames,
            entrance_timing(self.ordinal, self.reduced_motion),
            window,
            cx,
        )
        .value;
        let button = button_frame(self.button, self.disabled)
            .custom(
                ButtonCustomVariant::new(cx)
                    .color(border)
                    .foreground(rgb(self.colors.text_primary).into())
                    .hover(background)
                    .active(background),
            )
            .bg(background)
            .border_color(border)
            .accessibility_label(self.label)
            .child(content(self.label, self.glyph, self.font_size, foreground));
        div()
            .on_children_prepainted(move |children, window, cx| {
                let focused = measured_focus.contains_focused(window, cx);
                let entered = focused && !was_focused.get();
                was_focused.set(focused);
                if entered {
                    if let Some(mut item) = children.first().copied() {
                        let scroll = measure_scroll.clone();
                        let outer_scroll = measure_outer_scroll.clone();
                        let focus = measured_focus.clone();
                        let measured_offset = scroll.offset();
                        let measured_outer_offset = outer_scroll.offset();
                        item.origin.x -= measured_offset.x;
                        item.origin.y -= measured_outer_offset.y;
                        // Finish this frame before changing its scroll transform or hit regions.
                        window.on_next_frame(move |window, cx| {
                            if !focus.contains_focused(window, cx)
                                || scroll.offset() != measured_offset
                                || outer_scroll.offset() != measured_outer_offset
                            {
                                return;
                            }
                            let x = reveal_x(
                                scroll.bounds(),
                                item,
                                measured_offset.x,
                                scroll.max_offset().x,
                            );
                            let outer_bounds = outer_scroll.bounds();
                            let y = reveal_offset(
                                outer_bounds.top(),
                                outer_bounds.size.height,
                                item.top(),
                                item.size.height,
                                measured_outer_offset.y,
                                outer_scroll.max_offset().y,
                            );
                            if x != measured_offset.x {
                                scroll.set_offset(point(x, measured_offset.y));
                            }
                            if y != measured_outer_offset.y {
                                outer_scroll.set_offset(point(measured_outer_offset.x, y));
                            }
                            if x != measured_offset.x || y != measured_outer_offset.y {
                                window.refresh();
                            }
                        });
                    }
                }
            })
            .id((key, "prompt-observer"))
            .h_flex()
            .h(px(32.))
            .items_start()
            .flex_shrink_0()
            .track_focus(&focus)
            .tab_stop(false)
            // This ancestor observes descendant focus; it must never take mouse focus itself.
            .capture_any_mouse_down(|_, window, _| window.prevent_default())
            .hover_listener_mode(HoverListenerMode::InputModalityIndependent)
            .on_hover(move |hovered, window, cx| {
                if pointer.read(cx) != hovered {
                    pointer.update(cx, |state, _| *state = *hovered);
                    window.refresh();
                }
            })
            .child(action_slot().child(entrance_clip(progress).child(button)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{relative, FlexDirection, FlexWrap, FontWeight, Length, Overflow};

    #[test]
    fn prompt_collection_scrolls_horizontally_without_wrapping_or_shrinking() {
        let mut wrapper = wrapper();
        assert_eq!(wrapper.style().padding.left, Some(px(16.).into()));
        assert_eq!(wrapper.style().padding.right, Some(px(16.).into()));
        assert_eq!(wrapper.style().margin.top, Some(px(24.).into()));
        assert_eq!(wrapper.style().flex_shrink, Some(0.));
        let mut viewport = viewport(px(672.));
        assert_eq!(viewport.style().max_size.width, Some(px(672.).into()));
        assert_eq!(viewport.style().min_size.width, Some(px(0.).into()));
        assert_eq!(viewport.style().overflow.x, Some(Overflow::Scroll));
        assert_eq!(viewport.style().size.width, Some(relative(1.).into()));
        let mut strip = strip();
        let style = strip.style();
        assert_eq!(style.flex_direction, Some(FlexDirection::Row));
        assert_ne!(style.flex_wrap, Some(FlexWrap::Wrap));
        assert_eq!(style.flex_shrink, Some(0.));
        assert_eq!(style.margin.left, Some(Length::Auto));
        assert_eq!(style.margin.right, Some(Length::Auto));
        assert_eq!(style.gap.width, Some(px(16.).into()));
    }

    #[test]
    fn prompt_button_uses_reference_rectangle_and_fixed_visual_height() {
        for disabled in [false, true] {
            let mut button = button_frame(Button::new("prompt-test"), disabled);
            let style = button.style();
            assert_eq!(style.size.height, Some(px(32.).into()));
            assert_eq!(style.corner_radii.top_left, Some(px(8.).into()));
            assert_eq!(style.padding.left, Some(px(12.).into()));
            assert_eq!(style.padding.right, Some(px(12.).into()));
            assert_eq!(style.flex_shrink, Some(0.));
            assert_eq!(style.opacity, disabled.then_some(0.5));
            assert!(style.size.width.is_none());
        }
    }

    #[test]
    fn prompt_content_keeps_reference_gap_and_18px_text_line() {
        for size in 12..=20 {
            let font_size = px(size as f32);
            for text in ["周报总结", "Weekly report"] {
                let mut label = label(text, font_size, rgb(0xd4d4d4).into());
                let style = label.style();
                assert_eq!(style.text.font_size, Some(font_size.into()));
                assert_eq!(style.text.line_height, Some(px(18.).into()));
                assert_eq!(style.text.font_weight, Some(FontWeight::NORMAL));
                assert_eq!(style.max_size.width, Some(px(256.).into()));
                let mut content = content(text, Glyph::AlarmClock, font_size, rgb(0xd4d4d4).into());
                assert_eq!(content.style().gap.width, Some(px(6.).into()));
            }
        }
    }

    #[test]
    fn prompt_hover_and_disabled_colors_follow_the_pinned_outline_and_opacity() {
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            let idle = paint(&colors, false, false);
            let hovered = paint(&colors, true, false);
            assert_eq!(idle.foreground, rgb(colors.text_primary).alpha(0.7).into());
            assert_eq!(hovered.foreground, rgb(colors.text_primary).into());
            assert_eq!(hovered.background, rgb(colors.dock_bg).alpha(0.5).into());
            assert_eq!(hovered.border, rgba(colors.dock_border_active).into());
            assert_eq!(paint(&colors, true, true), idle);
            assert_eq!(paint(&colors, false, true), idle);
        }
        assert_eq!(duration(false, false), Duration::from_millis(150));
        assert_eq!(duration(true, false), Duration::ZERO);
        assert_eq!(duration(false, true), Duration::ZERO);
    }

    #[test]
    fn waterfall_uses_full_260ms_travel_after_each_65ms_stagger() {
        use gpui_kit::base::MotionPhase;

        for ordinal in 0..4 {
            let timing = entrance_timing(ordinal, false);
            let delay = Duration::from_millis(ordinal as u64 * 65);
            if ordinal > 0 {
                let before = timing.sample(delay - Duration::from_millis(1));
                assert_eq!(before.phase, MotionPhase::Before);
                assert_eq!(before.directed_progress, 0.);
            }
            let start = timing.sample(delay);
            assert_eq!(start.phase, MotionPhase::Active);
            assert_eq!(start.directed_progress, 0.);
            let halfway = timing.sample(delay + Duration::from_millis(130));
            // The pinned (0.22, 1, 0.36, 1) curve has travelled ~96% at t=0.5.
            assert!((halfway.directed_progress - 0.9614).abs() < 0.001);
            let end = timing.sample(delay + Duration::from_millis(260));
            assert_eq!(end.phase, MotionPhase::After);
            assert_eq!(end.directed_progress, 1.);
            assert!(end.finished);
        }
    }

    #[test]
    fn reduced_motion_skips_both_waterfall_delay_and_travel() {
        use gpui_kit::base::MotionPhase;

        for ordinal in [0, 3, usize::MAX] {
            let sample = entrance_timing(ordinal, true).sample(Duration::ZERO);
            assert_eq!(sample.phase, MotionPhase::After);
            assert_eq!(sample.directed_progress, 1.);
            assert!(sample.finished);
        }
    }

    #[test]
    fn waterfall_clips_and_translates_together_without_resizing_the_button() {
        use gpui_kit::Position;

        for (input, progress) in [(-1., 0.), (0., 0.), (0.5, 0.5), (1., 1.), (2., 1.)] {
            let mut clip = entrance_clip(input);
            let style = clip.style();
            assert_eq!(style.size.height, Some(px(32. * progress).into()));
            assert_eq!(style.inset.top, Some(px(-12. * (1. - progress)).into()));
            assert_eq!(style.position, Some(Position::Relative));
            assert_eq!(style.overflow.y, Some(Overflow::Hidden));
            assert_eq!(style.opacity, Some(progress));
            assert_eq!(style.flex_shrink, Some(0.));
            let mut button = button_frame(Button::new("waterfall-test"), false);
            assert_eq!(button.style().size.height, Some(px(32.).into()));
        }
    }

    #[test]
    fn focus_reveal_matches_reference_by_leaving_partly_visible_items_in_place() {
        use gpui_kit::size;

        let viewport = Bounds::new(point(px(16.), px(60.)), size(px(448.), px(32.)));
        let item = |x, width| Bounds::new(point(px(x), px(60.)), size(px(width), px(32.)));
        assert_eq!(
            reveal_x(viewport, item(16., 170.), px(0.), px(180.)),
            px(0.)
        );
        assert_eq!(
            reveal_x(viewport, item(478., 166.), px(0.), px(180.)),
            px(-180.)
        );
        assert_eq!(
            reveal_x(viewport, item(300., 170.), px(0.), px(180.)),
            px(0.)
        );
        assert_eq!(
            reveal_x(viewport, item(16., 170.), px(-180.), px(180.)),
            px(0.)
        );
        assert_eq!(
            reveal_x(viewport, item(300., 170.), px(-100.), px(180.)),
            px(-100.)
        );
        assert_eq!(
            reveal_x(viewport, item(16., 170.), px(-100.), px(180.)),
            px(-100.)
        );
    }

    #[test]
    fn focus_reveal_clamps_to_real_range_and_handles_oversized_items() {
        use gpui_kit::size;

        let viewport = Bounds::new(point(px(16.), px(60.)), size(px(448.), px(32.)));
        let item = Bounds::new(point(px(500.), px(60.)), size(px(500.), px(32.)));
        assert_eq!(reveal_x(viewport, item, px(0.), px(600.)), px(-484.));
        assert_eq!(reveal_x(viewport, item, px(0.), px(100.)), px(-100.));
        assert_eq!(reveal_x(viewport, item, px(0.), px(0.)), px(0.));
        assert_eq!(reveal_x(viewport, item, px(0.), px(-1.)), px(0.));
    }

    #[test]
    fn focus_reveal_brings_offscreen_rows_into_the_outer_vertical_viewport() {
        let reveal = |top, current, max| {
            reveal_offset(px(48.), px(592.), px(top), px(32.), px(current), px(max))
        };
        // The grown composer pushes the strip below a 480x640 window.
        assert_eq!(reveal(660., 0., 100.), px(-52.));
        assert_eq!(reveal(660., -52., 100.), px(-52.));
        // A return to an entirely hidden row above the current viewport reveals its top.
        assert_eq!(reveal(60., -100., 100.), px(-12.));
        // Partially exposed rows and a manually chosen position stay stationary.
        assert_eq!(reveal(630., 0., 100.), px(0.));
        assert_eq!(reveal(660., -40., 100.), px(-40.));
        assert_eq!(reveal(660., 0., 20.), px(-20.));
        assert_eq!(reveal(660., 0., 0.), px(0.));
    }

    #[test]
    fn focus_reveal_measures_the_stable_slot_throughout_entrance() {
        for progress in [0., 0.25, 0.5, 1.] {
            let mut slot = action_slot().child(entrance_clip(progress));
            let style = slot.style();
            assert_eq!(style.size.height, Some(px(32.).into()));
            assert_eq!(style.inset.top, None);
            assert_eq!(style.flex_shrink, Some(0.));
            assert_eq!(
                reveal_offset(px(48.), px(592.), px(660.), px(32.), px(0.), px(100.)),
                px(-52.)
            );
        }
    }
}
