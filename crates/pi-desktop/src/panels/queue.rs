use crate::theme::ThemeColors;
use gpui_kit::assets::IconName as Glyph;
use gpui_kit::component::button::{Button, ButtonCustomVariant, ButtonVariants};
use gpui_kit::component::{Disableable, Icon, Sizable, StyledExt};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, px, relative, rgb, rgba, App, Div, ElementId, InteractiveElement, ParentElement, Pixels,
    Stateful, StatefulInteractiveElement, Styled,
};

pub(super) fn wrapper(empty: bool) -> Div {
    div()
        .w_full()
        .min_w_0()
        .flex_shrink_0()
        .h_flex()
        .justify_center()
        .when(empty, |wrapper| wrapper.px(px(16.)))
}

pub(super) fn column(width: Pixels, empty: bool) -> Div {
    div()
        .w_full()
        .max_w(width)
        .min_w_0()
        .v_flex()
        .when(!empty, |column| column.px(px(16.)))
}

pub(super) fn surface(font_size: Pixels, colors: &ThemeColors) -> Div {
    div()
        .relative()
        .w_full()
        .min_w_0()
        .flex_shrink_0()
        .v_flex()
        .overflow_hidden()
        .rounded_t(px(16.))
        .border_1()
        .border_color(rgba(colors.settings_border))
        .bg(rgba(colors.card_user_bg))
        .p(px(4.))
        // The next composer paints over this padded tail, not over a queue row.
        .pb(px(28.))
        .mb(px(-28.))
        .text_size(font_size)
        .line_height(relative(1.5))
        .text_color(rgb(colors.text_primary))
}

pub(super) fn standalone(surface: Div) -> Div {
    // A run notice can intervene, so only join the padded tail directly to the composer.
    surface.pb(px(4.)).mb(px(8.)).rounded_b(px(16.))
}

pub(super) fn paused_banner(colors: &ThemeColors) -> Div {
    let mut border = rgba(colors.settings_border);
    border.a *= 0.7;
    // The pinned stylesheet defines no surface-raised token; leave its fill unset.
    div()
        .min_w_0()
        .flex_shrink_0()
        .h_flex()
        .items_center()
        .min_h(px(40.))
        .gap(px(12.))
        .mb(px(4.))
        .px(px(12.))
        .py(px(8.))
        .rounded(px(12.))
        .border_1()
        .border_color(border)
}

pub(super) fn paused_message(label: &'static str) -> Div {
    div().flex_1().min_w_0().whitespace_normal().child(label)
}

pub(super) fn items() -> Stateful<Div> {
    div()
        .id("pending-queue-items")
        .v_flex()
        .min_w_0()
        .gap(px(2.))
        .max_h(px(140.))
        .overflow_y_scroll()
}

pub(super) fn row(colors: &ThemeColors) -> Div {
    let mut hover = rgba(colors.hover);
    hover.a *= 0.3;
    div()
        .relative()
        .h_flex()
        .min_w_0()
        .flex_shrink_0()
        .items_center()
        .gap(px(8.))
        .pl(px(6.))
        .pr(px(4.))
        .py(px(4.))
        .rounded(px(12.))
        .hover(move |row| row.bg(hover))
}

pub(super) fn prompt_label() -> Div {
    div().flex_1().min_w_0().truncate()
}

fn ghost_button(id: impl Into<ElementId>, colors: &ThemeColors, cx: &App) -> Button {
    Button::new(id)
        .custom(
            ButtonCustomVariant::new(cx)
                .foreground(rgb(colors.text_primary).into())
                .hover(rgba(colors.hover).into())
                .active(rgba(colors.hover).into()),
        )
        .small()
        .border_1()
        .border_color(rgba(0x00000000))
        .flex_shrink_0()
}

fn resume_frame(button: Button, disabled: bool) -> Button {
    Styled::rounded(button.h(px(24.)).px(px(8.)), px(6.))
        .disabled(disabled)
        .when(disabled, |button| button.bg(rgba(0x00000000)).opacity(0.5))
}

fn resume_label(
    label: &'static str,
    font_size: Pixels,
    colors: &ThemeColors,
    disabled: bool,
) -> Div {
    let foreground = rgb(colors.text_primary);
    div()
        .text_size(font_size)
        .line_height(relative(1.625))
        .font_normal()
        .whitespace_nowrap()
        .text_color(rgba(colors.foreground_subtle))
        .when(!disabled, |label| {
            label.group_hover("queue-resume", move |text| text.text_color(foreground))
        })
        .child(label)
}

pub(super) fn resume_button(
    label: &'static str,
    font_size: Pixels,
    colors: &ThemeColors,
    disabled: bool,
    cx: &App,
) -> Button {
    resume_frame(ghost_button("queue-resume", colors, cx), disabled)
        .group("queue-resume")
        .accessibility_label(label)
        .child(resume_label(label, font_size, colors, disabled))
}

fn icon_frame(button: Button) -> Button {
    Styled::rounded(button.w(px(28.)).h(px(28.)).p(px(0.)), px(8.))
}

pub(super) fn item_button(
    id: impl Into<ElementId>,
    label: &'static str,
    glyph: Glyph,
    colors: &ThemeColors,
    cx: &App,
) -> Button {
    icon_frame(ghost_button(id, colors, cx))
        .accessibility_label(label)
        .tooltip(label)
        .child(Icon::new(glyph).size(px(16.)).flex_shrink_0())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::{AlignItems, FontWeight, Overflow, WhiteSpace};

    #[test]
    fn queue_and_composer_use_the_same_side_insets() {
        for empty in [false, true] {
            let mut wrapper = wrapper(empty);
            let mut column = column(px(672.), empty);
            assert_eq!(wrapper.style().flex_shrink, Some(0.));
            assert_eq!(wrapper.style().size.width, Some(relative(1.).into()));
            assert_eq!(column.style().max_size.width, Some(px(672.).into()));
            assert_eq!(wrapper.style().padding.left, empty.then(|| px(16.).into()));
            assert_eq!(
                column.style().padding.left,
                (!empty).then(|| px(16.).into())
            );
        }
    }

    #[test]
    fn queue_surface_reserves_only_the_composer_overlap_tail() {
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            for size in 12..=20 {
                let font_size = px(size as f32);
                let mut surface = surface(font_size, &colors);
                let style = surface.style();
                assert_eq!(style.flex_shrink, Some(0.));
                assert!(style.size.height.is_none());
                assert_eq!(style.padding.left, Some(px(4.).into()));
                assert_eq!(style.padding.top, Some(px(4.).into()));
                assert_eq!(style.padding.bottom, Some(px(28.).into()));
                assert_eq!(style.margin.bottom, Some(px(-28.).into()));
                assert_eq!(style.corner_radii.top_left, Some(px(16.).into()));
                assert!(style.corner_radii.bottom_left.is_none());
                assert_eq!(style.background, Some(rgba(colors.card_user_bg).into()));
                assert_eq!(style.text.font_size, Some(font_size.into()));
                assert_eq!(style.text.line_height, Some(relative(1.5)));
                assert_eq!(style.overflow.x, Some(Overflow::Hidden));
                assert!(style.box_shadow.is_none());
            }
        }
    }

    #[test]
    fn paused_notice_keeps_natural_text_and_a_minimum_height() {
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            let mut banner = paused_banner(&colors);
            let style = banner.style();
            let mut border = rgba(colors.settings_border);
            border.a *= 0.7;
            assert!(style.size.height.is_none());
            assert_eq!(style.min_size.height, Some(px(40.).into()));
            assert_eq!(style.gap.width, Some(px(12.).into()));
            assert_eq!(style.padding.left, Some(px(12.).into()));
            assert_eq!(style.padding.top, Some(px(8.).into()));
            assert_eq!(style.margin.bottom, Some(px(4.).into()));
            assert_eq!(style.border_color, Some(border.into()));
            assert!(style.background.is_none());
        }
        for text in ["队列已暂停", "Queue paused"] {
            let mut message = paused_message(text);
            let style = message.style();
            assert_eq!(style.flex_grow, Some(1.));
            assert_eq!(style.min_size.width, Some(px(0.).into()));
            assert_eq!(style.text.white_space, Some(WhiteSpace::Normal));
            assert!(style.text.text_overflow.is_none());
        }
    }

    #[test]
    fn queue_rows_are_unframed_and_keep_the_reference_spacing() {
        let mut list = items();
        assert_eq!(list.style().gap.height, Some(px(2.).into()));
        assert_eq!(list.style().max_size.height, Some(px(140.).into()));
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            let mut row = row(&colors);
            let style = row.style();
            assert_eq!(style.align_items, Some(AlignItems::Center));
            assert_eq!(style.padding.left, Some(px(6.).into()));
            assert_eq!(style.padding.right, Some(px(4.).into()));
            assert_eq!(style.padding.top, Some(px(4.).into()));
            assert_eq!(style.gap.width, Some(px(8.).into()));
            assert_eq!(style.corner_radii.top_left, Some(px(12.).into()));
            assert!(style.size.height.is_none());
            assert!(style.background.is_none());
            assert!(style.border_widths.top.is_none());
        }
        let mut label = prompt_label();
        assert_eq!(label.style().flex_grow, Some(1.));
        assert_eq!(label.style().min_size.width, Some(px(0.).into()));
    }

    #[test]
    fn queue_commands_keep_small_text_and_icon_geometry_separate() {
        for disabled in [false, true] {
            let mut resume = resume_frame(Button::new("queue-resume-test"), disabled);
            assert_eq!(resume.style().size.height, Some(px(24.).into()));
            assert_eq!(resume.style().padding.left, Some(px(8.).into()));
            assert_eq!(resume.style().corner_radii.top_left, Some(px(6.).into()));
            assert!(resume.style().size.width.is_none());
            assert_eq!(resume.style().opacity, disabled.then_some(0.5));
            assert_eq!(
                resume.style().background,
                disabled.then(|| rgba(0x00000000).into())
            );
        }
        let mut icon = icon_frame(Button::new("queue-icon-test"));
        assert_eq!(icon.style().size.width, Some(px(28.).into()));
        assert_eq!(icon.style().size.height, Some(px(28.).into()));
        assert_eq!(icon.style().padding.left, Some(px(0.).into()));
        assert_eq!(icon.style().corner_radii.top_left, Some(px(8.).into()));
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            for size in 12..=20 {
                for text in ["继续", "Resume"] {
                    let font_size = px(size as f32);
                    for disabled in [false, true] {
                        let mut label = resume_label(text, font_size, &colors, disabled);
                        let style = label.style();
                        assert_eq!(style.text.font_size, Some(font_size.into()));
                        assert_eq!(style.text.line_height, Some(relative(1.625)));
                        assert_eq!(style.text.font_weight, Some(FontWeight::NORMAL));
                        assert_eq!(style.text.white_space, Some(WhiteSpace::Nowrap));
                        assert!(style.size.height.is_none());
                    }
                }
            }
        }
    }
}
