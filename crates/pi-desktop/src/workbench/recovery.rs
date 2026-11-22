use crate::theme::ThemeColors;
use gpui_kit::component::button::{Button, ButtonCustomVariant, ButtonVariants};
use gpui_kit::component::StyledExt;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, px, relative, rgb, rgba, App, Div, FontWeight, ParentElement, Pixels, Styled};

pub(super) fn wrapper(empty: bool) -> Div {
    div()
        .w_full()
        .min_w_0()
        .flex_shrink_0()
        .when(empty, |wrapper| wrapper.px(px(16.)))
        .h_flex()
        .justify_center()
}

pub(super) fn column(content_width: Pixels, empty: bool) -> Div {
    let width = if empty {
        content_width.min(px(672.))
    } else {
        super::conversation_composer_width(content_width)
    };
    super::composer_column(width, empty)
}

pub(super) fn surface(font_size: Pixels, colors: &ThemeColors) -> Div {
    div()
        .w_full()
        .min_w_0()
        .flex_shrink_0()
        .mb(px(12.))
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

pub(super) fn message(label: &'static str) -> Div {
    div().min_w_0().flex_1().whitespace_normal().child(label)
}

fn command_label(label: &'static str, font_size: Pixels) -> Div {
    div()
        .text_size(font_size)
        .line_height(relative(1.5))
        .font_weight(FontWeight::NORMAL)
        .whitespace_nowrap()
        .child(label)
}

fn command_frame(button: Button, inset: Pixels) -> Button {
    // These reference controls are padded plain buttons, not fixed-height variants.
    Styled::rounded(button.h_auto().px(inset).py(px(4.)).border_0(), px(6.))
}

pub(super) fn resend_button(label: &'static str, font_size: Pixels) -> Button {
    command_frame(Button::new("resend-pending-command").primary(), px(10.))
        .child(command_label(label, font_size))
        .accessibility_label(label)
}

pub(super) fn dismiss_button(
    label: &'static str,
    font_size: Pixels,
    colors: &ThemeColors,
    cx: &App,
) -> Button {
    command_frame(
        Button::new("dismiss-pending-command").custom(
            ButtonCustomVariant::new(cx)
                .foreground(rgba(colors.foreground_subtle).into())
                .hover(rgba(colors.hover).into())
                .active(rgba(colors.hover).into()),
        ),
        px(8.),
    )
    .child(command_label(label, font_size))
    .accessibility_label(label)
}

#[cfg(test)]
mod tests {
    use super::{column, command_frame, command_label, message, resend_button, surface, wrapper};
    use crate::theme::ThemeColors;
    use gpui_kit::base::LengthExt;
    use gpui_kit::component::button::Button;
    use gpui_kit::{
        px, relative, rgb, rgba, AlignItems, FlexWrap, FontWeight, Length, Styled, WhiteSpace,
    };

    #[test]
    fn recovery_tracks_composer_width_with_one_fixed_gutter_per_side() {
        for (empty, content_width, expected_surface_width) in [
            (true, 480., 448.),
            (true, 639., 607.),
            (true, 640., 608.),
            (true, 864., 672.),
            (true, 1200., 672.),
            (true, 1600., 672.),
            (false, 480., 448.),
            (false, 639., 607.),
            (false, 640., 608.),
            (false, 863., 831.),
            (false, 864., 736.),
            (false, 932., 804.),
            (false, 1200., 864.),
            (false, 1279., 864.),
            (false, 1280., 864.),
            (false, 1600., 1120.),
        ] {
            let width = px(content_width);
            let mut wrapper = wrapper(empty);
            let mut column = column(width, empty);
            let wrapper = wrapper.style();
            let column = column.style();
            assert_eq!(wrapper.size.width, Some(relative(1.).into()));
            assert_eq!(column.size.width, Some(relative(1.).into()));
            assert_eq!(wrapper.flex_shrink, Some(0.));
            assert_eq!(column.flex_shrink, Some(0.));
            assert_eq!(wrapper.min_size.width, Some(px(0.).into()));
            assert_eq!(column.min_size.width, Some(px(0.).into()));
            for style in [&wrapper, &column] {
                assert!(style.size.height.is_none());
                assert!(style.padding.bottom.is_none());
                assert!(style.margin.bottom.is_none());
                assert!(style.overflow.x.is_none());
                assert!(style.overflow.y.is_none());
            }
            for rem in 12..=20 {
                let rem = px(rem as f32);
                let outer_left = wrapper
                    .padding
                    .left
                    .map_or(px(0.), |v| v.to_pixels(width.into(), rem));
                let outer_right = wrapper
                    .padding
                    .right
                    .map_or(px(0.), |v| v.to_pixels(width.into(), rem));
                let inner_left = column
                    .padding
                    .left
                    .map_or(px(0.), |v| v.to_pixels(width.into(), rem));
                let inner_right = column
                    .padding
                    .right
                    .map_or(px(0.), |v| v.to_pixels(width.into(), rem));
                assert_eq!(outer_left + inner_left, px(16.));
                assert_eq!(outer_right + inner_right, px(16.));
                let cap = column
                    .max_size
                    .width
                    .unwrap()
                    .to_pixels(width.into(), rem)
                    .unwrap();
                let surface_width =
                    (width - outer_left - outer_right).min(cap) - inner_left - inner_right;
                assert_eq!(surface_width, px(expected_surface_width));
            }
        }
    }

    #[test]
    fn recovery_surface_wraps_without_a_fixed_height_or_card_shadow() {
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            for font_size in 12..=20 {
                let font_size = px(font_size as f32);
                let mut banner = surface(font_size, &colors);
                let style = banner.style();
                assert_eq!(style.size.width, Some(relative(1.).into()));
                assert!(style.size.height.is_none());
                assert_eq!(style.min_size.width, Some(px(0.).into()));
                assert_eq!(style.flex_shrink, Some(0.));
                assert_eq!(style.flex_wrap, Some(FlexWrap::Wrap));
                assert_eq!(style.align_items, Some(AlignItems::Center));
                assert_eq!(style.margin.bottom, Some(px(12.).into()));
                assert_eq!(style.padding.left, Some(px(12.).into()));
                assert_eq!(style.padding.right, Some(px(12.).into()));
                assert_eq!(style.padding.top, Some(px(8.).into()));
                assert_eq!(style.padding.bottom, Some(px(8.).into()));
                assert_eq!(style.gap.width, Some(px(8.).into()));
                assert_eq!(style.gap.height, Some(px(8.).into()));
                assert_eq!(style.border_widths.top, Some(px(1.).into()));
                assert_eq!(
                    style.border_color,
                    Some(rgba(colors.settings_border).into())
                );
                assert_eq!(style.background, Some(rgba(colors.card_user_bg).into()));
                assert_eq!(style.text.color, Some(rgb(colors.text_primary).into()));
                assert_eq!(style.text.font_size, Some(font_size.into()));
                assert_eq!(style.text.line_height, Some(relative(1.5)));
                assert!(style.box_shadow.is_none());
            }
        }
    }

    #[test]
    fn recovery_message_keeps_its_complete_text_and_natural_height() {
        for label in [
            "CLI 重启前已提交的输入没有进入对话",
            "A command submitted before the CLI restarted did not enter the conversation.",
        ] {
            let mut message = message(label);
            let style = message.style();
            assert_eq!(style.min_size.width, Some(px(0.).into()));
            assert_eq!(style.flex_grow, Some(1.));
            assert_eq!(style.text.white_space, Some(WhiteSpace::Normal));
            assert!(style.size.height.is_none());
            assert!(style.text.text_overflow.is_none());
            assert!(style.overflow.x.is_none());
        }
    }

    #[test]
    fn recovery_commands_follow_padded_text_height_instead_of_native_small_geometry() {
        for label in ["重新发送", "Resend", "忽略", "Dismiss"] {
            for font_size in 12..=20 {
                let font_size = px(font_size as f32);
                let mut label = command_label(label, font_size);
                let style = label.style();
                assert_eq!(style.text.font_size, Some(font_size.into()));
                assert_eq!(style.text.line_height, Some(relative(1.5)));
                assert_eq!(style.text.font_weight, Some(FontWeight::NORMAL));
                assert_eq!(style.text.white_space, Some(WhiteSpace::Nowrap));
                assert!(style.size.width.is_none());
                assert!(style.size.height.is_none());
                for inset in [8., 10.] {
                    let mut button = command_frame(Button::new("recovery-test"), px(inset));
                    let style = button.style();
                    assert_eq!(style.size.height, Some(Length::Auto));
                    assert!(style.size.width.is_none());
                    assert_eq!(style.padding.left, Some(px(inset).into()));
                    assert_eq!(style.padding.right, Some(px(inset).into()));
                    assert_eq!(style.padding.top, Some(px(4.).into()));
                    assert_eq!(style.padding.bottom, Some(px(4.).into()));
                    assert_eq!(style.border_widths.top, Some(px(0.).into()));
                    assert_eq!(style.corner_radii.top_left, Some(px(6.).into()));
                }
            }
        }
        let mut resend = resend_button("Resend", px(20.));
        assert_eq!(resend.style().size.height, Some(Length::Auto));
        assert_eq!(resend.style().padding.left, Some(px(10.).into()));
    }
}
