// SPDX-License-Identifier: GPL-3.0-or-later

//! Small building blocks of the mail window: icons, buttons, avatars,
//! menus and shadows.

use gpui::{
    AnyElement, AnyView, App, BoxShadow, Div, FontWeight, SharedString, Stateful, Window, div,
    point, prelude::*, px, rgba, svg,
};
use katna_ui::{Ripple, Tooltip};

use crate::theme::{Theme, avatar_color, fade, initial};

pub const TOOLBAR_HEIGHT: f32 = 48.0;

pub fn icon(name: &str, color: u32, size: f32) -> AnyElement {
    svg()
        .path(SharedString::from(format!("icons/{name}.svg")))
        .size(px(size))
        .flex_none()
        .text_color(rgba(color))
        .into_any_element()
}

/// A tooltip saying `text`, for `.tooltip()`: it shows once the pointer
/// rests on the element.
pub fn tip(
    text: impl Into<SharedString>,
    th: &Theme,
) -> impl Fn(&mut Window, &mut App) -> AnyView + 'static {
    Tooltip::text(text, rgba(th.snackbar), rgba(th.snackbar_text))
}

/// A round icon button with a centered ripple.
pub fn icon_button(
    id: impl Into<gpui::ElementId>,
    name: &str,
    size: f32,
    th: &Theme,
) -> Stateful<Div> {
    icon_button_colored(id, name, size, th.text_dim, th)
}

pub fn icon_button_colored(
    id: impl Into<gpui::ElementId>,
    name: &str,
    size: f32,
    color: u32,
    th: &Theme,
) -> Stateful<Div> {
    let id = id.into();
    div()
        .id(id.clone())
        .relative()
        .overflow_hidden()
        .size(px(40.0))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        // Keep the header bar from starting a window move.
        .on_mouse_move(|_, _, cx| cx.stop_propagation())
        .child(Ripple::new(("ripple", id_hash(&id)), rgba(th.ripple)).centered())
        .child(icon(name, color, size))
}

/// A stable number for a ripple's ID derived from its button's ID.
fn id_hash(id: &gpui::ElementId) -> usize {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    id.to_string().hash(&mut hasher);
    hasher.finish() as usize
}

/// An outlined button with an icon and a label.
pub fn pill_button(
    id: impl Into<gpui::ElementId>,
    name: &str,
    label: impl Into<SharedString>,
    th: &Theme,
) -> Stateful<Div> {
    let id = id.into();
    div()
        .id(id.clone())
        .relative()
        .overflow_hidden()
        .h(px(36.0))
        .pl(px(16.0))
        .pr(px(22.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.0))
        .rounded_full()
        .border_1()
        .border_color(rgba(fade(th.text_faint, 0.7)))
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(th.text_dim))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .child(Ripple::new(("ripple", id_hash(&id)), rgba(th.ripple)))
        .child(icon(name, th.text_dim, 20.0))
        .child(label.into())
}

/// A filled, rounded button (the primary action of a panel).
pub fn filled_button(
    id: impl Into<gpui::ElementId>,
    label: impl Into<SharedString>,
    th: &Theme,
) -> Stateful<Div> {
    let id = id.into();
    div()
        .id(id.clone())
        .relative()
        .overflow_hidden()
        .h(px(36.0))
        .px(px(24.0))
        .flex()
        .items_center()
        .rounded_full()
        .bg(rgba(th.accent))
        .text_color(rgba(th.on_accent))
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .cursor_pointer()
        .hover(|s| s.shadow(elevation(th, 1.0)))
        .child(Ripple::new(("ripple", id_hash(&id)), rgba(0xffffff3d)))
        .child(label.into())
}

/// An outlined, rounded button with a label.
pub fn outlined_button(
    id: impl Into<gpui::ElementId>,
    label: impl Into<SharedString>,
    th: &Theme,
) -> Stateful<Div> {
    let id = id.into();
    div()
        .id(id.clone())
        .relative()
        .overflow_hidden()
        .h(px(36.0))
        .px(px(20.0))
        .flex()
        .items_center()
        .rounded_full()
        .border_1()
        .border_color(rgba(fade(th.text_faint, 0.7)))
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(th.accent))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .child(Ripple::new(("ripple", id_hash(&id)), rgba(th.ripple)))
        .child(label.into())
}

pub fn toolbar(th: &Theme) -> Div {
    div()
        .flex_none()
        .h(px(TOOLBAR_HEIGHT))
        .px(px(8.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(2.0))
        .border_b_1()
        .border_color(rgba(th.divider))
}

/// A letter avatar for `name`, colored by `address`.
pub fn avatar(name: &str, address: &str, size: f32) -> AnyElement {
    let key = if address.is_empty() { name } else { address };
    div()
        .size(px(size))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .bg(rgba(avatar_color(key)))
        .text_color(rgba(0xffffffff))
        .text_size(px(size * 0.45))
        .font_weight(FontWeight::MEDIUM)
        .child(initial(name))
        .into_any_element()
}

/// Material-style elevation: `level` 0 is flat, 3 floats well above.
pub fn elevation(th: &Theme, level: f32) -> Vec<BoxShadow> {
    if level <= 0.001 {
        return Vec::new();
    }
    let t = (level / 3.0).min(1.0);
    vec![
        BoxShadow {
            color: rgba(fade(th.shadow, 0.9 * t)).into(),
            offset: point(px(0.0), px(1.0)),
            blur_radius: px(2.0 * level.min(2.0)),
            spread_radius: px(0.0),
            inset: false,
        },
        BoxShadow {
            color: rgba(fade(th.shadow, 0.5 * t)).into(),
            offset: point(px(0.0), px(level)),
            blur_radius: px(3.0 * level),
            spread_radius: px(level / 2.0),
            inset: false,
        },
    ]
}

pub fn placeholder(text: &str, th: &Theme) -> AnyElement {
    div()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .p(px(24.0))
        .text_size(px(14.0))
        .text_color(rgba(th.text_faint))
        .child(text.to_owned())
        .into_any_element()
}

/// A floating menu: a column of [`menu_item`]s on a raised surface.
pub fn menu(th: &Theme) -> Div {
    div()
        .py(px(8.0))
        .min_w(px(180.0))
        .flex()
        .flex_col()
        .rounded(px(8.0))
        .bg(rgba(th.menu))
        .shadow(elevation(th, 3.0))
        .text_size(px(14.0))
        .text_color(rgba(th.text))
}

pub fn menu_item(id: impl Into<gpui::ElementId>, label: &str, th: &Theme) -> Stateful<Div> {
    div()
        .id(id)
        .h(px(32.0))
        .px(px(16.0))
        .flex()
        .items_center()
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .child(label.to_owned())
}

/// A two-state switch drawn at `t` (0 off, 1 on), for animating.
pub fn switch(t: f32, th: &Theme) -> AnyElement {
    let track = crate::theme::mix(th.switch_off, th.accent, t);
    let knob = crate::theme::mix(th.text_faint, th.on_accent, t);
    div()
        .relative()
        .w(px(36.0))
        .h(px(20.0))
        .flex_none()
        .rounded_full()
        .bg(rgba(track))
        .child({
            // The knob grows a little when on, and stays centered in the
            // track with the same gap all round.
            let size = 14.0 + 2.0 * t;
            let gap = (20.0 - size) / 2.0;
            div()
                .absolute()
                .top(px(gap))
                .left(px(gap + (36.0 - size - 2.0 * gap) * t))
                .size(px(size))
                .rounded_full()
                .bg(rgba(knob))
        })
        .into_any_element()
}

/// A radio button drawn at `t` (0 off, 1 on).
pub fn radio(t: f32, th: &Theme) -> AnyElement {
    let ring = crate::theme::mix(th.text_dim, th.accent, t);
    div()
        .size(px(20.0))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .border_2()
        .border_color(rgba(ring))
        .child(div().size(px(10.0 * t)).rounded_full().bg(rgba(th.accent)))
        .into_any_element()
}
