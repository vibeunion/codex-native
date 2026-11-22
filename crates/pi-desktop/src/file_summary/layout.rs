use crate::theme::ThemeColors;
use gpui_kit::component::StyledExt;
use gpui_kit::{
    div, px, relative, rgb, rgba, Div, FontFeatures, InteractiveElement, Pixels, Styled,
};
use std::sync::Arc;

pub(crate) fn file_surface(colors: &ThemeColors) -> Div {
    let mut background = rgb(colors.root_bg);
    background.a = 0.5;
    div()
        .w_full()
        .min_w_0()
        .flex_shrink_0()
        .overflow_hidden()
        .bg(background)
}

pub(crate) fn file_row(font_size: Pixels, colors: &ThemeColors) -> Div {
    let mut hover = rgba(colors.hover);
    hover.a *= 0.3;
    div()
        .w_full()
        .min_w_0()
        .h_flex()
        .items_center()
        .gap(px(4.))
        .px(px(8.))
        .py(px(8.))
        .text_size(font_size)
        .line_height(relative(1.5))
        .hover(move |row| row.bg(hover))
}

pub(crate) fn file_label() -> Div {
    div().h_flex().items_center().gap(px(6.)).min_w_0()
}

pub(crate) fn counts(font_size: Pixels) -> Div {
    div()
        .h_flex()
        .items_center()
        .flex_shrink_0()
        .text_size(font_size)
        .line_height(relative(1.5))
        .font_normal()
        .font_features(FontFeatures(Arc::new(vec![("tnum".into(), 1)])))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{AlignItems, FontWeight, Overflow};

    #[test]
    fn file_surface_uses_half_opacity_background_without_a_fixed_row_height() {
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            let mut surface = file_surface(&colors);
            let style = surface.style();
            let mut background = rgb(colors.root_bg);
            background.a = 0.5;
            assert_eq!(style.background, Some(background.into()));
            assert_eq!(style.size.width, Some(relative(1.).into()));
            assert!(style.size.height.is_none());
            assert_eq!(style.flex_shrink, Some(0.));
            assert_eq!(style.overflow.x, Some(Overflow::Hidden));
            assert_eq!(style.overflow.y, Some(Overflow::Hidden));
        }
    }

    #[test]
    fn rows_keep_reference_padding_and_natural_height_at_all_interface_sizes() {
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            for font_size in 12..=20 {
                let font_size = px(font_size as f32);
                let mut row = file_row(font_size, &colors);
                let style = row.style();
                assert!(style.size.height.is_none());
                assert!(style.min_size.height.is_none());
                assert!(style.max_size.height.is_none());
                assert_eq!(style.size.width, Some(relative(1.).into()));
                assert_eq!(style.align_items, Some(AlignItems::Center));
                assert_eq!(style.gap.width, Some(px(4.).into()));
                assert_eq!(style.padding.left, Some(px(8.).into()));
                assert_eq!(style.padding.right, Some(px(8.).into()));
                assert_eq!(style.padding.top, Some(px(8.).into()));
                assert_eq!(style.padding.bottom, Some(px(8.).into()));
                assert_eq!(style.text.font_size, Some(font_size.into()));
                assert_eq!(style.text.line_height, Some(relative(1.5)));
                assert!(style.background.is_none());
            }
        }
    }

    #[test]
    fn file_label_is_shrinkable_and_uses_the_reference_six_pixel_gap() {
        let mut label = file_label();
        let style = label.style();
        assert_eq!(style.min_size.width, Some(px(0.).into()));
        assert_eq!(style.align_items, Some(AlignItems::Center));
        assert_eq!(style.gap.width, Some(px(6.).into()));
        assert!(style.size.width.is_none());
    }

    #[test]
    fn counts_use_tabular_digits_in_the_interface_font_instead_of_monospace() {
        for font_size in 12..=20 {
            let font_size = px(font_size as f32);
            let mut content = counts(font_size);
            let style = content.style();
            assert_eq!(style.flex_shrink, Some(0.));
            assert_eq!(style.text.font_size, Some(font_size.into()));
            assert_eq!(style.text.line_height, Some(relative(1.5)));
            assert_eq!(style.text.font_weight, Some(FontWeight::NORMAL));
            assert!(style.text.font_family.is_none());
            assert_eq!(
                style.text.font_features,
                Some(FontFeatures(Arc::new(vec![("tnum".into(), 1)])))
            );
        }
    }
}
