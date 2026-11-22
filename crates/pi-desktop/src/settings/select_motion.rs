use crate::theme::ThemeColors;
use gpui_kit::assets::IconName as Glyph;
use gpui_kit::base::{transition, Transition};
use gpui_kit::component::searchable_list::{SearchableListDelegate, SearchableListItem};
use gpui_kit::component::select::Select;
use gpui_kit::component::{ActiveTheme, Icon, StyledExt};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, px, relative, rems, rgba, AnyElement, App, Div, ElementId, HoverListenerMode, Hsla,
    InteractiveElement, IntoElement, ParentElement, Pixels, RenderOnce, Role, SharedString,
    Stateful, StatefulInteractiveElement, Styled, Window,
};
use std::time::Duration;

const BORDER_TRAVEL: Duration = Duration::from_millis(150);
const REFERENCE_MENU_PADDING: Pixels = px(4.);
const REFERENCE_MENU_ROW_GAP: Pixels = px(2.);
const REFERENCE_MENU_CHECK_SIZE: Pixels = px(16.);
const REFERENCE_MENU_CHECK_RIGHT: Pixels = px(0.);

#[derive(Clone)]
pub(crate) struct SettingsChoice {
    label: SharedString,
}

#[derive(Clone, Default)]
pub(crate) struct SettingsChoices(Vec<SettingsChoice>);

pub(crate) fn settings_choices(labels: Vec<SharedString>) -> SettingsChoices {
    SettingsChoices(
        labels
            .into_iter()
            .map(|label| SettingsChoice { label })
            .collect(),
    )
}

fn menu_vertical_compensation(rem_size: Pixels) -> Pixels {
    // gpui-component's py_1 is 0.25rem; Pi Desktop keeps menu padding at 4px.
    REFERENCE_MENU_PADDING - rem_size * 0.25
}

fn menu_label(label: SharedString, rem_size: Pixels) -> Stateful<Div> {
    // Override the native Medium row's 0.875rem text, not its selection wrapper.
    div()
        .id(format!("settings-menu-label-{label}"))
        .role(Role::Label)
        .aria_label(label.clone())
        .text_size(rems(1.))
        .line_height(relative(1.625))
        .my(menu_vertical_compensation(rem_size))
        .child(label)
}

fn menu_item(label: SharedString, checked: bool, rem_size: Pixels, check_color: Hsla) -> Div {
    div()
        .relative()
        .w_full()
        .h_flex()
        .items_center()
        // Pi Desktop keeps a fixed 2px separation between menu rows.
        .mb(REFERENCE_MENU_ROW_GAP)
        .child(menu_label(label, rem_size))
        .child(
            Icon::new(Glyph::Check)
                .size(REFERENCE_MENU_CHECK_SIZE)
                .flex_shrink_0()
                .text_color(check_color)
                // `render_item` is wrapped in the library's left content slot,
                // which clips negative offsets. Keep the reference-sized mark
                // inside that slot so the selected check remains visible.
                .absolute()
                .right(REFERENCE_MENU_CHECK_RIGHT)
                .when(!checked, |icon| icon.invisible()),
        )
}

impl SearchableListItem for SettingsChoice {
    type Value = SharedString;

    fn title(&self) -> SharedString {
        self.label.clone()
    }

    fn value(&self) -> &Self::Value {
        &self.label
    }

    fn render(&self, window: &mut Window, _: &mut App) -> impl IntoElement {
        menu_label(self.label.clone(), window.rem_size())
    }
}

impl SearchableListDelegate for SettingsChoices {
    type Item = SettingsChoice;

    fn items_count(&self, _: usize) -> usize {
        self.0.len()
    }

    fn item(&self, ix: gpui_kit::component::IndexPath) -> Option<&Self::Item> {
        self.0.get(ix.row)
    }

    fn position<V>(&self, value: &V) -> Option<gpui_kit::component::IndexPath>
    where
        Self::Item: SearchableListItem<Value = V>,
        V: PartialEq,
    {
        self.0
            .iter()
            .position(|item| item.value() == value)
            .map(gpui_kit::component::IndexPath::new)
    }

    fn render_item(
        &self,
        _ix: gpui_kit::component::IndexPath,
        item: &Self::Item,
        checked: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        Some(
            menu_item(
                item.label.clone(),
                checked,
                window.rem_size(),
                cx.theme().muted_foreground,
            )
            .into_any_element(),
        )
    }
}

fn hover_border(colors: &ThemeColors, hovered: bool) -> Hsla {
    rgba(if hovered {
        colors.dock_border_active
    } else {
        colors.settings_border
    })
    .into()
}

fn duration(reduced_motion: bool) -> Duration {
    if reduced_motion {
        Duration::ZERO
    } else {
        BORDER_TRAVEL
    }
}

fn select_shell() -> Div {
    div().min_w_0().h(px(32.)).flex_shrink_0()
}

fn select_indicator() -> Icon {
    // Select reapplies xsmall to custom icons; constraints preserve its 14px indicator.
    Icon::new(Glyph::ChevronDown)
        .min_w(px(14.))
        .max_w(px(14.))
        .min_h(px(14.))
        .max_h(px(14.))
}

#[derive(IntoElement)]
pub(super) struct SettingsSelect<D: SearchableListDelegate + 'static> {
    id: SharedString,
    shell: Div,
    select: Select<D>,
    colors: ThemeColors,
    reduced_motion: bool,
}

impl<D: SearchableListDelegate + 'static> SettingsSelect<D> {
    pub(super) fn new(
        id: &'static str,
        select: Select<D>,
        colors: &ThemeColors,
        reduced_motion: bool,
    ) -> Self {
        Self {
            id: id.into(),
            shell: select_shell(),
            select: select.icon(select_indicator()),
            colors: *colors,
            reduced_motion,
        }
    }
}

impl<D: SearchableListDelegate + 'static> Styled for SettingsSelect<D> {
    fn style(&mut self) -> &mut gpui_kit::StyleRefinement {
        self.shell.style()
    }
}

impl<D: SearchableListDelegate + 'static> RenderOnce for SettingsSelect<D> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = ElementId::from(self.id);
        let pointer =
            window.use_keyed_state((id.clone(), "settings-select-pointer"), cx, |_, _| false);
        let border = transition(
            (id.clone(), "settings-select-border"),
            hover_border(&self.colors, *pointer.read(cx)),
            Transition::new(duration(self.reduced_motion))
                .ease(gpui_kit::base::animation::cubic_bezier(0.4, 0., 0.2, 1.)),
            window,
            cx,
        );
        // The native Select still owns focus, open-state paint and its popup.
        self.shell
            .id(id)
            .hover_listener_mode(HoverListenerMode::InputModalityIndependent)
            .on_hover(move |hovered, window, cx| {
                if pointer.read(cx) != hovered {
                    pointer.update(cx, |state, _| *state = *hovered);
                    window.refresh();
                }
            })
            .child(self.select.border_color(border))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        duration, hover_border, menu_item, menu_label, menu_vertical_compensation,
        select_indicator, select_shell, settings_choices, REFERENCE_MENU_CHECK_RIGHT,
        REFERENCE_MENU_ROW_GAP,
    };
    use crate::theme::ThemeColors;
    use gpui_kit::base::Interpolate;
    use gpui_kit::component::searchable_list::{SearchableListDelegate, SearchableListItem};
    use gpui_kit::component::{IndexPath, Sizable};
    use gpui_kit::{px, relative, rems, rgba, Element, Hsla, Role, SharedString, Styled};
    use std::time::Duration;

    #[test]
    fn menu_labels_use_interface_rem_and_fixed_reference_padding_compensation() {
        for label in [
            "中文",
            "English",
            "明亮",
            "暗黑",
            "跟随系统",
            "Light",
            "Dark",
            "System",
        ] {
            for font_size in 12..=20 {
                let font_size = px(font_size as f32);
                let mut label = menu_label(label.into(), font_size);
                let style = label.style();
                assert_eq!(style.text.font_size, Some(rems(1.).into()));
                assert_eq!(style.text.line_height, Some(relative(1.625)));
                assert_eq!(
                    style.text.font_size.unwrap().to_pixels(font_size),
                    font_size
                );
                assert_eq!(
                    style
                        .text
                        .line_height
                        .unwrap()
                        .to_pixels(font_size.into(), font_size),
                    font_size * 1.625
                );
                assert_eq!(
                    style.margin.top,
                    Some(menu_vertical_compensation(font_size).into())
                );
                assert_eq!(style.margin.bottom, style.margin.top);
                assert_eq!(
                    style.margin.top.unwrap(),
                    (px(4.) - font_size * 0.25).into()
                );
                assert!(style.size.width.is_none());
                assert!(style.size.height.is_none());
                assert!(style.padding.left.is_none());
                assert!(style.padding.right.is_none());
                assert!(style.padding.top.is_none());
                assert!(style.padding.bottom.is_none());
                assert!(style.background.is_none());
                assert!(style.text.color.is_none());
                assert!(style.text.font_weight.is_none());
            }
        }
    }

    #[test]
    fn menu_presentation_preserves_plain_titles_values_and_native_selection_lookup() {
        for labels in [
            vec!["中文", "English"],
            vec!["明亮", "暗黑", "跟随系统"],
            vec!["Light", "Dark", "System"],
        ] {
            let labels: Vec<SharedString> = labels.into_iter().map(Into::into).collect();
            let choices = settings_choices(labels.clone());
            assert_eq!(choices.items_count(0), labels.len());
            for (row, label) in labels.iter().enumerate() {
                let index = IndexPath::new(row);
                let item = choices.item(index).unwrap();
                assert_eq!(&item.title(), label);
                assert_eq!(item.value(), label);
                assert_eq!(choices.position(label), Some(index));
                assert!(item.display_title().is_none());
                assert!(!item.disabled());
            }
            assert!(choices.position(&SharedString::from("missing")).is_none());
        }
        assert_eq!(settings_choices(Vec::new()).items_count(0), 0);
    }

    #[test]
    fn menu_content_exposes_each_localized_label_without_adding_a_tab_stop() {
        for label in [
            "中文",
            "English",
            "明亮",
            "暗黑",
            "跟随系统",
            "Light",
            "Dark",
            "System",
        ] {
            let content = menu_label(label.into(), px(14.));
            assert_eq!(content.a11y_role(), Some(Role::Label));
            let mut node = gpui_kit::accesskit::Node::new(Role::Label);
            content.write_a11y_info(&mut node);
            assert_eq!(node.label(), Some(label));
            assert!(!node.supports_action(gpui_kit::accesskit::Action::Focus));
        }
    }

    #[test]
    fn menu_item_uses_the_reference_sized_check_slot_without_changing_row_geometry() {
        for checked in [false, true] {
            let mut item = menu_item("Dark".into(), checked, px(14.), rgba(0x26262699).into());
            let style = item.style();
            assert_eq!(style.position, Some(gpui_kit::Position::Relative));
            assert_eq!(style.size.width, Some(relative(1.).into()));
            assert!(style.size.height.is_none());
            assert_eq!(style.align_items, Some(gpui_kit::AlignItems::Center));
            assert_eq!(style.margin.bottom, Some(REFERENCE_MENU_ROW_GAP.into()));
            assert!(style.margin.top.is_none());
            assert!(style.margin.left.is_none());
            assert!(style.margin.right.is_none());
            assert_eq!(style.padding, Default::default());
        }
        assert_eq!(REFERENCE_MENU_CHECK_RIGHT, px(0.));
    }

    #[test]
    fn indicator_keeps_14px_constraints_after_native_xsmall_adaptation() {
        let mut indicator = select_indicator().xsmall();
        let style = indicator.style();
        assert_eq!(style.min_size.width, Some(px(14.).into()));
        assert_eq!(style.max_size.width, Some(px(14.).into()));
        assert_eq!(style.min_size.height, Some(px(14.).into()));
        assert_eq!(style.max_size.height, Some(px(14.).into()));
        assert!(style.padding.left.is_none());
        assert!(style.padding.right.is_none());
        assert!(style.border_widths.left.is_none());
        assert!(style.background.is_none());
    }

    #[test]
    fn select_hover_uses_the_reference_neutral_border_in_both_themes() {
        for (colors, idle, hovered) in [
            (ThemeColors::light(), 0x0d0d0d1a, 0x0d0d0d26),
            (ThemeColors::dark(), 0xffffff1a, 0xffffff26),
        ] {
            assert_eq!(hover_border(&colors, false), Hsla::from(rgba(idle)));
            assert_eq!(hover_border(&colors, true), Hsla::from(rgba(hovered)));
        }
    }

    #[test]
    fn select_hover_motion_is_bounded_and_respects_reduced_motion() {
        assert_eq!(duration(false), Duration::from_millis(150));
        assert_eq!(duration(true), Duration::ZERO);
        for colors in [ThemeColors::light(), ThemeColors::dark()] {
            let from = hover_border(&colors, false);
            let to = hover_border(&colors, true);
            for progress in [0., 0.25, 0.5, 0.75, 1.] {
                let color = from.interpolate(&to, progress);
                assert!(color.a >= from.a && color.a <= to.a);
                assert_eq!((color.h, color.s, color.l), (from.h, from.s, from.l));
            }
        }
    }

    #[test]
    fn select_shell_reserves_one_trigger_frame_without_extra_paint_or_padding() {
        let mut shell = select_shell();
        let style = shell.style();
        assert!(style.size.width.is_none());
        assert_eq!(style.size.height, Some(px(32.).into()));
        assert_eq!(style.min_size.width, Some(px(0.).into()));
        assert_eq!(style.flex_shrink, Some(0.));
        assert!(style.background.is_none());
        assert!(style.border_widths.top.is_none());
        assert!(style.padding.top.is_none());
        assert!(style.padding.right.is_none());
        assert!(style.padding.bottom.is_none());
        assert!(style.padding.left.is_none());
        assert!(style.overflow.x.is_none());
        assert!(style.overflow.y.is_none());
    }

    #[test]
    fn select_shell_leaves_width_to_the_content_column_or_footer_pair() {
        let mut content = select_shell().w_full();
        assert_eq!(content.style().size.width, Some(relative(1.).into()));
        let mut footer = select_shell().flex_1();
        let style = footer.style();
        assert!(style.size.width.is_none());
        assert_eq!(style.flex_grow, Some(1.));
        assert_eq!(style.flex_shrink, Some(1.));
        assert_eq!(style.flex_basis, Some(relative(0.).into()));
        assert_eq!(style.min_size.width, Some(px(0.).into()));
        assert_eq!(style.size.height, Some(px(32.).into()));
    }
}
