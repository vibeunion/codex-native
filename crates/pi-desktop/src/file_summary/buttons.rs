use crate::theme::ThemeColors;
use gpui_kit::assets::IconName as Glyph;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Disableable, Icon, Sizable, StyledExt};
use gpui_kit::{div, px, relative, rgb, rgba, Div, ElementId, ParentElement, Pixels, Styled};

fn rewind_content(label: &'static str, font_size: Pixels) -> Div {
    div()
        .h_flex()
        .items_center()
        .justify_center()
        .gap(px(4.))
        .text_size(font_size)
        .line_height(relative(1.625))
        .font_normal()
        .whitespace_nowrap()
        .child(Icon::new(Glyph::Undo2).size(px(12.)).flex_shrink_0())
        .child(label)
}

pub(crate) fn unavailable_rewind_button(
    id: impl Into<ElementId>,
    label: &'static str,
    accessible_label: &'static str,
    font_size: Pixels,
    colors: &ThemeColors,
) -> Button {
    Styled::rounded(Button::new(id).ghost().small(), px(6.))
        .h(px(24.))
        .px(px(8.))
        .border_1()
        .border_color(rgba(0x00000000))
        .flex_shrink_0()
        .disabled(true)
        // Preserve reference disabled opacity without the library's muted fill.
        .bg(rgba(0x00000000))
        .text_color(rgb(colors.text_primary))
        .opacity(0.5)
        .accessibility_label(accessible_label)
        .child(rewind_content(label, font_size))
}

fn review_content(label: &'static str, font_size: Pixels) -> Div {
    div()
        .text_size(font_size)
        .line_height(relative(1.5))
        .font_normal()
        .whitespace_nowrap()
        .child(label)
}

pub(crate) fn review_button(
    id: impl Into<ElementId>,
    label: &'static str,
    font_size: Pixels,
    colors: &ThemeColors,
) -> Button {
    Styled::rounded(Button::new(id).outline().small(), px(8.))
        .h(px(28.))
        .px(px(8.))
        .border_1()
        .border_color(rgba(colors.settings_border))
        .bg(rgb(colors.dock_bg))
        .flex_shrink_0()
        .child(review_content(label, font_size))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{AlignItems, FontWeight, JustifyContent, WhiteSpace};

    #[test]
    fn unavailable_rewind_keeps_small_reference_geometry_and_disabled_paint() {
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            for label in ["撤销", "Rewind"] {
                for font_size in 12..=20 {
                    let mut button = unavailable_rewind_button(
                        "rewind-test",
                        label,
                        "Rewind (unavailable)",
                        px(font_size as f32),
                        &colors,
                    );
                    let style = button.style();
                    assert_eq!(style.size.height, Some(px(24.).into()));
                    assert!(style.size.width.is_none());
                    assert_eq!(style.padding.left, Some(px(8.).into()));
                    assert_eq!(style.padding.right, Some(px(8.).into()));
                    assert_eq!(style.corner_radii.top_left, Some(px(6.).into()));
                    assert_eq!(style.border_widths.top, Some(px(1.).into()));
                    assert_eq!(style.border_color, Some(rgba(0x00000000).into()));
                    assert_eq!(style.background, Some(rgba(0x00000000).into()));
                    assert_eq!(style.text.color, Some(rgb(colors.text_primary).into()));
                    assert_eq!(style.opacity, Some(0.5));
                }
            }
        }
    }

    #[test]
    fn review_keeps_default_reference_geometry_and_input_surface() {
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            for label in ["审查", "Review"] {
                for font_size in 12..=20 {
                    let mut button =
                        review_button("review-test", label, px(font_size as f32), &colors);
                    let style = button.style();
                    assert_eq!(style.size.height, Some(px(28.).into()));
                    assert!(style.size.width.is_none());
                    assert_eq!(style.padding.left, Some(px(8.).into()));
                    assert_eq!(style.padding.right, Some(px(8.).into()));
                    assert_eq!(style.corner_radii.top_left, Some(px(8.).into()));
                    assert_eq!(style.border_widths.top, Some(px(1.).into()));
                    assert_eq!(
                        style.border_color,
                        Some(rgba(colors.settings_border).into())
                    );
                    assert_eq!(style.background, Some(rgb(colors.dock_bg).into()));
                    assert!(style.opacity.is_none());
                }
            }
        }
    }

    #[test]
    fn action_text_follows_interface_size_without_a_fixed_width_or_line_box() {
        for font_size in 12..=20 {
            let font_size = px(font_size as f32);
            for label in ["撤销", "Rewind", "审查", "Review"] {
                for (mut content, line_height) in [
                    (rewind_content(label, font_size), 1.625),
                    (review_content(label, font_size), 1.5),
                ] {
                    let style = content.style();
                    assert!(style.size.width.is_none());
                    assert!(style.size.height.is_none());
                    assert_eq!(style.text.font_size, Some(font_size.into()));
                    assert_eq!(style.text.line_height, Some(relative(line_height)));
                    assert_eq!(style.text.font_weight, Some(FontWeight::NORMAL));
                    assert_eq!(style.text.white_space, Some(WhiteSpace::Nowrap));
                }
                let mut content = rewind_content(label, font_size);
                let style = content.style();
                assert_eq!(style.align_items, Some(AlignItems::Center));
                assert_eq!(style.justify_content, Some(JustifyContent::Center));
                assert_eq!(style.gap.width, Some(px(4.).into()));
            }
        }
    }
}
