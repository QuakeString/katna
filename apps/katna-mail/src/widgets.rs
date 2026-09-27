// SPDX-License-Identifier: GPL-3.0-or-later

//! Small building blocks of the mail window: icons, buttons, avatars,
//! menus and shadows.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use gpui::{
    AnyElement, AnyView, App, Bounds, BoxShadow, Div, ElementId, FocusHandle, FontWeight, Pixels,
    ScrollHandle, SharedString, Stateful, StyleRefinement, Window, canvas, div, img, point,
    prelude::*, rgba, svg,
};
use katna_ui::motion::lerp;
use katna_ui::px;
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

/// Katna's logo, `size` px square. Below 48 px it takes the logo's small
/// form, whose card carries only ক, since the whole word can't be read
/// there (`packaging/icons/src/`).
pub fn katna_mark(size: f32) -> AnyElement {
    img(SharedString::from(crate::assets::logo_path(
        size * katna_ui::scale::scale(),
    )))
    .size(px(size))
    .flex_none()
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

/// An outlined button with an icon and a label. The label folds away as
/// `shown` goes from 1 to 0, leaving a round button with the icon alone;
/// `word` is the label's width, measured in the font it shows in.
pub fn pill_button(
    id: impl Into<gpui::ElementId>,
    name: &str,
    label: impl Into<SharedString>,
    word: f32,
    shown: f32,
    th: &Theme,
) -> Stateful<Div> {
    let id = id.into();
    let t = shown.clamp(0.0, 1.0);
    div()
        .id(id.clone())
        .relative()
        .overflow_hidden()
        .h(px(36.0))
        .pl(px(lerp(17.0, 16.0, t)))
        .pr(px(lerp(17.0, 22.0, t)))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.0 * t))
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
        .child(
            div()
                .flex_none()
                .whitespace_nowrap()
                .overflow_hidden()
                .when(t < 0.999, |d| d.w(px(word * t)).opacity(t))
                .child(label.into()),
        )
}

/// How far a card's shadow reaches past its edges.
pub const CARD_SHADOW_ROOM: f32 = 4.0;

/// The very short, soft shadow under a card, for a little depth. `t`
/// fades it away (0 on a phone, whose cards run edge to edge).
pub fn card_shadow(th: &Theme, t: f32) -> Vec<BoxShadow> {
    if t <= 0.001 {
        return Vec::new();
    }
    vec![BoxShadow {
        color: rgba(fade(th.shadow, 0.3 * t.min(1.0))).into(),
        offset: point(px(0.0), px(1.0)),
        blur_radius: px(3.0),
        spread_radius: px(0.0),
        inset: false,
    }]
}

/// A faint line around a card (the list, the reading pane, Quick
/// settings). It is drawn over the card's content, so lines of the list
/// that fill the card's width do not hide it; the card keeps `t` px of
/// padding inside it. `t` fades it away (0 on a phone, edge to edge).
pub fn card_outline(th: &Theme, radius: f32, t: f32) -> Option<AnyElement> {
    (t > 0.001).then(|| {
        div()
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .bottom_0()
            .rounded(px(radius))
            .border_1()
            .border_color(rgba(fade(th.divider, t.min(1.0))))
            .into_any_element()
    })
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

/// Makes a control reachable with Tab and Shift+Tab. Enter and Space then
/// click it, and a tint with a ring inside its edge shows it has the
/// keyboard focus (not after a mouse click). Clicking it also takes the
/// focus, so keep it off buttons that act on a text field.
pub trait FocusRing: Sized {
    fn focus_ring(self, th: &Theme) -> Self;
    /// The same for a control in a scrolling page: the page scrolls to
    /// show it when Tab moves to it.
    fn focus_ring_in(self, stops: &TabStops, th: &Theme, cx: &App) -> Self;
}

impl FocusRing for Stateful<Div> {
    fn focus_ring(self, th: &Theme) -> Self {
        self.tab_index(0).focus_visible(ring_style(th))
    }

    fn focus_ring_in(mut self, stops: &TabStops, th: &Theme, cx: &App) -> Self {
        let Some(id) = self.interactivity().element_id.clone() else {
            return self.focus_ring(th);
        };
        let handle = stops.handle(id, cx);
        let (stops, focus) = (stops.clone(), handle.clone());
        self.track_focus(&handle)
            .focus_visible(ring_style(th))
            .child(
                canvas(
                    move |bounds, window, _| stops.reveal(&focus, bounds, window),
                    |_, _, _, _| {},
                )
                .absolute()
                .top_0()
                .left_0()
                .size_full(),
            )
    }
}

fn ring_style(th: &Theme) -> impl FnOnce(StyleRefinement) -> StyleRefinement + use<> {
    let ring = rgba(th.accent);
    let tint = rgba(fade(th.accent, 0.08));
    move |s| {
        s.bg(tint).shadow(vec![BoxShadow {
            color: ring.into(),
            offset: point(px(0.0), px(0.0)),
            blur_radius: px(0.0),
            spread_radius: px(2.0),
            inset: true,
        }])
    }
}

/// The Tab stops of a scrolling page, by element id, and the page's scroll.
#[derive(Clone)]
pub struct TabStops(Rc<Stops>);

struct Stops {
    handles: RefCell<HashMap<ElementId, FocusHandle>>,
    scroll: ScrollHandle,
    /// Tab just moved the focus: the next frame shows the focused control.
    reveal: Cell<bool>,
}

impl TabStops {
    pub fn new(scroll: ScrollHandle) -> Self {
        Self(Rc::new(Stops {
            handles: RefCell::default(),
            scroll,
            reveal: Cell::new(false),
        }))
    }

    fn handle(&self, id: ElementId, cx: &App) -> FocusHandle {
        self.0
            .handles
            .borrow_mut()
            .entry(id)
            .or_insert_with(|| cx.focus_handle().tab_stop(true))
            .clone()
    }

    /// Whether the control `id` has the keyboard focus.
    pub fn focused(&self, id: &ElementId, window: &Window) -> bool {
        self.0
            .handles
            .borrow()
            .get(id)
            .is_some_and(|handle| handle.is_focused(window))
    }

    /// Scrolls the control Tab moves to into view on the next frame.
    pub fn reveal_focus(&self) {
        self.0.reveal.set(true);
    }

    fn reveal(&self, focus: &FocusHandle, bounds: Bounds<Pixels>, window: &mut Window) {
        if !self.0.reveal.get() || !focus.is_focused(window) {
            return;
        }
        self.0.reveal.set(false);
        let scroll = &self.0.scroll;
        let view = scroll.bounds();
        let margin = px(16.0);
        let offset = scroll.offset();
        let y = if bounds.bottom() + margin > view.bottom() {
            offset.y - (bounds.bottom() + margin - view.bottom())
        } else if bounds.top() - margin < view.top() {
            offset.y + (view.top() - (bounds.top() - margin))
        } else {
            return;
        };
        let y = y.clamp(-scroll.max_offset().y, px(0.0));
        scroll.set_offset(point(offset.x, y));
        // Drawn at the new offset in the next frame.
        window.request_animation_frame();
    }
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
    raised(
        div()
            .py(px(8.0))
            .min_w(px(180.0))
            .flex()
            .flex_col()
            .text_size(px(14.0))
            .text_color(rgba(th.text)),
        th,
        8.0,
        3.0,
    )
}

/// How opaque a frosted panel's color is over the blur.
const FROST_ALPHA: f32 = 0.78;

/// The surface of a floating panel (menu, popover, dropdown): `th.menu`
/// with corners of `radius` and a shadow of `level`. When
/// [`Theme::frost`] is on it is frosted glass: the color translucent over
/// a blur of what is behind. Call it before adding the panel's children,
/// which must draw over the glass.
pub fn raised<E: Styled + ParentElement>(panel: E, th: &Theme, radius: f32, level: f32) -> E {
    let panel = panel.rounded(px(radius)).shadow(elevation(th, level));
    if th.frost == 0 {
        return panel.bg(rgba(th.menu));
    }
    panel.child(katna_ui::frost::glass(
        rgba(fade(th.menu, FROST_ALPHA)).into(),
        px(radius),
        th.frost as f32,
    ))
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

/// A [`menu_item`] with an icon before its label.
pub fn menu_item_icon(
    id: impl Into<gpui::ElementId>,
    name: &str,
    label: &str,
    th: &Theme,
) -> Stateful<Div> {
    div()
        .id(id)
        .h(px(36.0))
        .pl(px(16.0))
        .pr(px(24.0))
        .flex()
        .items_center()
        .gap(px(16.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .child(icon(name, th.text_dim, 20.0))
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
