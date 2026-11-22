use super::{Glyph, ThemeColors};
use gpui_kit::component::button::Button;
use gpui_kit::component::{Icon, Sizable, StyledExt};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, px, relative, rgba, Div, ElementId, ParentElement, Pixels, Styled};

const ACTION_SIZE: Pixels = px(28.);
const CONTROL_GAP: Pixels = px(4.);
const GROUP_GAP: Pixels = px(12.);
const MODEL_ICON_WIDTH: Pixels = px(30.);

#[derive(Debug, PartialEq)]
pub(super) struct Fit {
    pub mode_compact: bool,
    pub model_compact: bool,
    pub model_width: Pixels,
}

/// Use the same measured text and fixed chrome as the rendered controls.
pub(super) fn fit(
    available: Pixels,
    mode_label_width: Pixels,
    model_label_width: Pixels,
    usage_width: Option<Pixels>,
    executing: bool,
) -> Fit {
    let model_width = model_label_width + px(34.);
    let fixed = ACTION_SIZE * 2.
        + CONTROL_GAP * 2.
        + GROUP_GAP
        + usage_width.map_or(px(0.), |width| width + CONTROL_GAP)
        + if executing {
            ACTION_SIZE + CONTROL_GAP
        } else {
            px(0.)
        };
    let full_mode_width = mode_label_width + px(56.);
    let mode_compact = fixed + full_mode_width + model_width > available;
    let mode_width = if mode_compact {
        ACTION_SIZE
    } else {
        full_mode_width
    };
    let model_compact = fixed + mode_width + model_width > available;
    Fit {
        mode_compact,
        model_compact,
        model_width: if model_compact {
            MODEL_ICON_WIDTH
        } else {
            model_width
        },
    }
}

pub(super) fn row() -> Div {
    div().h_flex().min_w_0().items_end().gap(GROUP_GAP)
}

pub(super) fn leading() -> Div {
    div().h_flex().flex_1().min_w_0().items_center()
}

pub(super) fn leading_content() -> Div {
    div()
        .h_flex()
        .flex_shrink_0()
        .items_center()
        .gap(CONTROL_GAP)
}

pub(super) fn trailing() -> Div {
    div()
        .h_flex()
        .flex_shrink_0()
        .items_center()
        .justify_end()
        .gap(CONTROL_GAP)
}

pub(super) fn button(id: impl Into<ElementId>) -> Button {
    Button::new(id)
        .small()
        .h(ACTION_SIZE)
        .px(px(8.))
        .border_1()
        .border_color(rgba(0x00000000))
        .rounded(px(8.))
        .font_normal()
        .line_height(relative(1.5))
}

pub(super) fn icon_button(id: impl Into<ElementId>, icon: Glyph) -> Button {
    button(id)
        .w(ACTION_SIZE)
        .p_0()
        .child(Icon::new(icon).size(px(16.)).flex_shrink_0())
}

pub(super) fn mode_content(
    label: &'static str,
    compact: bool,
    font_size: Pixels,
    colors: &ThemeColors,
) -> Div {
    div()
        .h_flex()
        .items_center()
        .gap(CONTROL_GAP)
        .text_size(font_size)
        .line_height(relative(1.5))
        .font_normal()
        .whitespace_nowrap()
        .text_color(rgba(colors.foreground_subtle))
        .child(Icon::new(Glyph::Hand).size(px(16.)).flex_shrink_0())
        .when(!compact, |content| {
            content.child(label).child(
                Icon::new(Glyph::ChevronDown)
                    .size(px(14.))
                    .flex_shrink_0()
                    .text_color(rgba(colors.foreground_subtlest)),
            )
        })
}

pub(super) fn model_content(
    label: String,
    compact: bool,
    font_size: Pixels,
    colors: &ThemeColors,
) -> Div {
    div()
        .h_flex()
        .min_w_0()
        .items_center()
        .gap(CONTROL_GAP)
        .text_size(font_size)
        .line_height(relative(1.5))
        .font_normal()
        .whitespace_nowrap()
        .text_color(rgba(colors.foreground_subtle))
        .when(compact, |content| {
            content.child(Icon::new(Glyph::Package).size(px(16.)).flex_shrink_0())
        })
        .when(!compact, |content| {
            content
                .child(div().min_w_0().truncate().child(label))
                .child(
                    Icon::new(Glyph::ChevronDown)
                        .size(px(14.))
                        .flex_shrink_0()
                        .text_color(rgba(colors.foreground_subtlest)),
                )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{AlignItems, FlexDirection};

    #[test]
    fn controls_keep_fixed_geometry_when_interface_rem_changes() {
        let mut label_button = button("mode");
        let mut icon = icon_button("send", Glyph::ArrowUp);
        for rem in 12..=20 {
            let rem = px(rem as f32);
            for control in [&mut label_button, &mut icon] {
                let style = control.style();
                assert_eq!(style.size.height, Some(ACTION_SIZE.into()));
                assert_eq!(style.border_widths.left, Some(px(1.).into()));
                assert_eq!(style.text.line_height, Some(relative(1.5)));
            }
            assert_eq!(
                label_button
                    .style()
                    .padding
                    .left
                    .unwrap()
                    .to_pixels(px(480.).into(), rem),
                px(8.)
            );
            assert_eq!(icon.style().size.width, Some(ACTION_SIZE.into()));
            assert_eq!(icon.style().padding.left, Some(px(0.).into()));
        }
    }

    #[test]
    fn toolbar_reserves_trailing_actions_and_uses_fixed_reference_gaps() {
        let mut row = row();
        let style = row.style();
        assert_eq!(style.flex_direction, Some(FlexDirection::Row));
        assert_eq!(style.align_items, Some(AlignItems::FlexEnd));
        assert_eq!(style.min_size.width, Some(px(0.).into()));
        assert_eq!(style.gap.width, Some(GROUP_GAP.into()));
        let mut leading = leading();
        assert_eq!(leading.style().flex_grow, Some(1.));
        assert_eq!(leading.style().min_size.width, Some(px(0.).into()));
        for mut group in [leading_content(), trailing()] {
            assert_eq!(group.style().flex_shrink, Some(0.));
            assert_eq!(group.style().gap.width, Some(CONTROL_GAP.into()));
        }
    }

    #[test]
    fn labels_follow_interface_size_in_both_themes_and_compact_states() {
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            for size in 12..=20 {
                for compact in [false, true] {
                    for mut content in [
                        mode_content("Confirm before edit", compact, px(size as f32), &colors),
                        model_content("Model".into(), compact, px(size as f32), &colors),
                    ] {
                        let style = content.style();
                        assert_eq!(style.text.font_size, Some(px(size as f32).into()));
                        assert_eq!(style.text.line_height, Some(relative(1.5)));
                        assert_eq!(style.gap.width, Some(CONTROL_GAP.into()));
                    }
                }
            }
        }
    }

    #[test]
    fn fitting_collapses_mode_before_model_and_expands_without_sticky_state() {
        // fixed chrome=76, full mode=156, model=134: exactly 366px.
        let check = |width| fit(px(width), px(100.), px(100.), None, false);
        assert_eq!(
            check(366.),
            Fit {
                mode_compact: false,
                model_compact: false,
                model_width: px(134.)
            }
        );
        assert!(check(365.).mode_compact);
        assert!(!check(365.).model_compact);
        assert!(!check(238.).model_compact);
        assert!(check(237.).model_compact);
        assert_eq!(check(237.).model_width, MODEL_ICON_WIDTH);
        assert!(!check(366.).mode_compact);
    }

    #[test]
    fn usage_and_queue_participate_in_fitting() {
        let plain = fit(px(300.), px(70.), px(100.), None, false);
        assert!(plain.mode_compact);
        assert!(!plain.model_compact);
        let busy = fit(px(300.), px(70.), px(100.), Some(px(44.)), true);
        assert!(busy.mode_compact);
        assert!(busy.model_compact);
        for executing in [false, true] {
            for usage in [None, Some(px(150.))] {
                let fitted = fit(px(422.), px(200.), px(500.), usage, executing);
                let occupied = ACTION_SIZE * 2.
                    + CONTROL_GAP * 2.
                    + GROUP_GAP
                    + if fitted.mode_compact {
                        ACTION_SIZE
                    } else {
                        px(256.)
                    }
                    + fitted.model_width
                    + usage.map_or(px(0.), |width| width + CONTROL_GAP)
                    + if executing {
                        ACTION_SIZE + CONTROL_GAP
                    } else {
                        px(0.)
                    };
                assert!(occupied <= px(422.));
            }
        }
    }

    #[test]
    fn natural_model_labels_expand_beyond_the_old_caps_before_collapsing() {
        // A 334px model fits in full until mode and then model must collapse.
        let check = |width| fit(px(width), px(70.), px(300.), None, false);
        assert_eq!(
            check(536.),
            Fit {
                mode_compact: false,
                model_compact: false,
                model_width: px(334.),
            }
        );
        assert_eq!(
            check(535.),
            Fit {
                mode_compact: true,
                model_compact: false,
                model_width: px(334.),
            }
        );
        assert!(!check(438.).model_compact);
        assert!(check(437.).model_compact);
        assert_eq!(check(437.).model_width, MODEL_ICON_WIDTH);
        for label_width in [px(116.), px(160.), px(240.), px(1000.)] {
            let fitted = fit(label_width + px(250.), px(70.), label_width, None, false);
            assert!(!fitted.mode_compact);
            assert!(!fitted.model_compact);
            assert_eq!(fitted.model_width, label_width + px(34.));
        }
        assert_eq!(check(536.).model_width, px(334.));
    }
}
