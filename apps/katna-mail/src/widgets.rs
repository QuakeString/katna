// SPDX-License-Identifier: GPL-3.0-or-later

//! Small building blocks of the mail window: icons, buttons, avatars,
//! menus and shadows.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use gpui::{
    AnimationExt, AnyElement, AnyView, App, Bounds, BoxShadow, Div, ElementId, FocusHandle,
    FontWeight, Pixels, ScrollHandle, SharedString, Stateful, StyleRefinement, Window, canvas, div,
    img, point, prelude::*, rgba, svg,
};
use katna_ui::motion::lerp;
use katna_ui::px;
use katna_ui::{Ripple, Tooltip};

use crate::theme::{Theme, avatar_color, fade, initial};
use crate::window::MenuKey;

pub const TOOLBAR_HEIGHT: f32 = 48.0;

pub fn icon(name: &str, color: u32, size: f32) -> AnyElement {
    svg()
        .path(SharedString::from(format!("icons/{name}.svg")))
        .size(px(size))
        .flex_none()
        .text_color(rgba(color))
        .into_any_element()
}

/// Katna's logo, the k on its teal disc, `size` px square. Below 48 px it
/// leaves out the shadow, which blurs to mush that small
/// (`packaging/icons/src/`).
pub fn katna_mark(size: f32) -> AnyElement {
    img(SharedString::from(crate::assets::logo_path(
        size * katna_ui::scale::scale(),
    )))
    .size(px(size))
    .flex_none()
    .into_any_element()
}

/// Katna Mail's wordmark, "katna mail" in script on the teal disc,
/// `height` px tall.
pub fn katna_wordmark(height: f32) -> AnyElement {
    let (w, h) = crate::assets::WORDMARK_SIZE;
    img(SharedString::from(crate::assets::wordmark_path(
        height * katna_ui::scale::scale(),
    )))
    .w(px(height * w as f32 / h as f32))
    .h(px(height))
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
    /// The same for a filled button, whose own color stays: the ring goes
    /// round it, a little apart.
    fn focus_ring_filled(self, th: &Theme) -> Self;
}

impl FocusRing for Stateful<Div> {
    fn focus_ring(self, th: &Theme) -> Self {
        self.tab_index(0).focus_visible(ring_style(th))
    }

    fn focus_ring_filled(self, th: &Theme) -> Self {
        let (gap, ring) = (rgba(th.surface), rgba(th.accent));
        self.tab_index(0).focus_visible(move |s| {
            s.shadow(vec![
                BoxShadow {
                    color: gap.into(),
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(2.0),
                    inset: false,
                },
                BoxShadow {
                    color: ring.into(),
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(4.0),
                    inset: false,
                },
            ])
        })
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

/// The ring inside the edge of a line that has the keys (the folder
/// pane's), the same as [`FocusRing`]'s.
pub fn keys_ring(th: &Theme) -> Vec<BoxShadow> {
    vec![BoxShadow {
        color: rgba(th.accent).into(),
        offset: point(px(0.0), px(0.0)),
        blur_radius: px(0.0),
        spread_radius: px(2.0),
        inset: true,
    }]
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

/// Material-style elevation: `level` 0 is flat, 3 floats well above. In
/// dark colors, where a shadow barely shows, a faint light edge
/// ([`Theme::rim`]) outlines it too; it fades in over the first level.
pub fn elevation(th: &Theme, level: f32) -> Vec<BoxShadow> {
    if level <= 0.001 {
        return Vec::new();
    }
    let t = (level / 3.0).min(1.0);
    let mut shadows = vec![
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
    ];
    // Last, so the shadows do not darken it.
    if th.rim & 0xff != 0 {
        shadows.push(BoxShadow {
            color: rgba(fade(th.rim, level.min(1.0))).into(),
            offset: point(px(0.0), px(0.0)),
            blur_radius: px(0.0),
            spread_radius: px(1.0),
            inset: false,
        });
    }
    shadows
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
        // Its own box, so a long line wraps in a narrow window.
        .child(div().min_w_0().text_center().child(text.to_owned()))
        .into_any_element()
}

/// The key context of a menu, whose items the arrow keys go through
/// (`window::MenuKey`).
pub const MENU_CONTEXT: &str = "Menu";

/// A floating menu: a column of [`menu_item`]s on a raised surface.
pub fn menu(th: &Theme) -> Div {
    raised(
        div()
            .key_context(MENU_CONTEXT)
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

/// The surface of a floating panel (menu, popover, dropdown): `th.menu`
/// with corners of `radius` and a shadow of `level`. When
/// [`Theme::frost`] is on it is frosted glass: the color translucent over
/// a blur of what is behind. Call it before adding the panel's children,
/// which must draw over the glass.
pub fn raised<E: Styled + ParentElement>(panel: E, th: &Theme, radius: f32, level: f32) -> E {
    frosted(
        panel.rounded(px(radius)).shadow(elevation(th, level)),
        th,
        th.menu,
        radius,
    )
}

/// Fills a dialog or floating card with `fill`, as frosted glass when
/// [`Theme::frost`] is on, like [`raised`] does for menus. `radius` is
/// the card's corner radius. Call it before adding the card's children,
/// which must draw over the glass.
pub fn frosted<E: Styled + ParentElement>(panel: E, th: &Theme, fill: u32, radius: f32) -> E {
    if th.frost == 0 {
        return panel.bg(rgba(fill));
    }
    panel.child(katna_ui::frost::glass(
        rgba(fade(fill, f32::from(th.frost_tint) / 100.0)).into(),
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
        .menu_key(th)
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
        .menu_key(th)
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

/// What a [`checkbox`] shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Check {
    Off,
    On,
    /// Some of what it stands for, as the list's select-all box shows: a
    /// smaller square inside the edge.
    Partial,
}

impl Check {
    pub fn from(on: bool) -> Self {
        if on { Check::On } else { Check::Off }
    }
}

/// The box of a checkbox: 16 px with a 2 px edge, as big as the star and
/// label icons it sits beside in a mail row, and at the weight of a
/// [`radio`] ring.
const CHECK_BOX: f32 = 16.0;
/// The gap between the edge and the square of a partly checked box.
const PARTIAL_GAP: f32 = 1.0;

/// A checkbox in the accent colour. Its tick draws itself in when checked
/// and wipes back out when cleared; `id` keys that motion, so give each box
/// on screen its own.
pub fn checkbox(id: impl Into<ElementId>, state: Check, th: &Theme) -> AnyElement {
    checkbox_colored(id, state, th.accent, th)
}

/// A checkbox filled with `fill` when checked, such as a calendar's own
/// colour. Pass `th.text_faint` for one that can't be changed.
pub fn checkbox_colored(
    id: impl Into<ElementId>,
    state: Check,
    fill: u32,
    th: &Theme,
) -> AnyElement {
    check_box(id.into(), state, fill, th.text_dim, th)
}

/// A checkbox edged in `color` even when clear, as Calendar shows each
/// calendar's own colour.
pub fn checkbox_tinted(id: impl Into<ElementId>, on: bool, color: u32, th: &Theme) -> AnyElement {
    check_box(id.into(), Check::from(on), color, color, th)
}

fn check_box(id: ElementId, state: Check, fill: u32, rest: u32, th: &Theme) -> AnyElement {
    let tick = tick_color(fill, th);
    // One spring runs clear (0), partly (1) and checked (2), so any change
    // between the three moves through the ones between.
    let target = match state {
        Check::Off => 0.0,
        Check::Partial => 1.0,
        Check::On => 2.0,
    };
    // A slot the size of a radio button, so rows of both line up.
    div()
        .size(px(20.0))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .child(div().size(px(CHECK_BOX)).flex_none().with_spring(
            id,
            gpui::SpringAnimation::new(katna_ui::motion::SMOOTH).to(target),
            move |d, v: f32| {
                // How far the edge has taken its colour, and how far the box
                // has filled and the tick has drawn.
                let edge = v.clamp(0.0, 1.0);
                let full = (v - 1.0).clamp(0.0, 1.0);
                // Partly checked is a smaller square inside the edge, with a
                // gap between them; checking grows it to fill the box. The
                // square is inset by the same length on every side rather
                // than sized and centred: at scales like 175% a sized square
                // and the box inside the edge differ by an odd number of
                // pixels, which left the gap thinner on the top and right.
                let half = (CHECK_BOX - 4.0) / 2.0;
                let gap = half - (half - lerp(PARTIAL_GAP, 0.0, full)) * edge;
                d.rounded(px(3.0))
                    .border_px(2.0)
                    .border_color(rgba(crate::theme::mix(rest, fill, edge)))
                    .relative()
                    .when(gap < half - 0.05, |d| {
                        d.child(
                            div()
                                .absolute()
                                .top(px(gap))
                                .left(px(gap))
                                .bottom(px(gap))
                                .right(px(gap))
                                .rounded(px(lerp(1.0, 0.0, full)))
                                .bg(rgba(fill)),
                        )
                    })
                    .when(full > 0.001, |d| d.bg(rgba(fade(fill, full))))
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .size_full()
                            .child(check_mark(full, tick)),
                    )
            },
        ))
        .into_any_element()
}

/// White on a dark fill, near-black on a light one.
fn tick_color(fill: u32, th: &Theme) -> u32 {
    if fill == th.accent {
        return th.on_accent;
    }
    let channel = |shift: u32| ((fill >> shift) & 0xff) as f32 / 255.0;
    let light = 0.299 * channel(24) + 0.587 * channel(16) + 0.114 * channel(8);
    if light > 0.6 {
        0x2021_24ff
    } else {
        0xffff_ffff
    }
}

/// The tick drawn from its start to `t` of its length, inside the box's
/// 2 px edge.
fn check_mark(t: f32, color: u32) -> impl IntoElement {
    // Points in a 14 px square, scaled to the box less its edge.
    const POINTS: [(f32, f32); 3] = [(1.8, 7.2), (5.2, 10.6), (12.2, 3.6)];
    let k = (CHECK_BOX - 4.0) / 14.0;
    let points = POINTS.map(|(x, y)| (x * k, y * k));
    // The tick starts once the box has begun to fill.
    let drawn = ((t - 0.25) / 0.75).clamp(0.0, 1.0);
    canvas(
        |_, _, _| {},
        move |bounds, _, window, _| {
            if drawn <= 0.0 {
                return;
            }
            let o = bounds.origin;
            let at = |(x, y): (f32, f32)| point(o.x + px(x), o.y + px(y));
            let lengths: Vec<f32> = points
                .windows(2)
                .map(|w| ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt())
                .collect();
            let mut left = drawn * lengths.iter().sum::<f32>();
            let mut path = gpui::PathBuilder::stroke(px(2.0));
            path.move_to(at(points[0]));
            for (w, len) in points.windows(2).zip(&lengths) {
                if left <= 0.0 {
                    break;
                }
                let k = (left / len).min(1.0);
                let end = (
                    w[0].0 + (w[1].0 - w[0].0) * k,
                    w[0].1 + (w[1].1 - w[0].1) * k,
                );
                path.line_to(at(end));
                left -= len;
            }
            if let Ok(path) = path.build() {
                window.paint_path(path, rgba(color));
            }
        },
    )
    .size_full()
}

/// An edge in design pixels, scaled with the interface like every other
/// length. GPUI's `border_2()` and the like are fixed device-independent
/// pixels, so at 200% they would be half as thick, and a checkbox's tick
/// would sit off the centre of a box drawn for the scaled edge.
pub trait ScaledEdge: Styled + Sized {
    /// An edge `width` design pixels wide all round.
    fn border_px(mut self, width: f32) -> Self {
        let width: gpui::AbsoluteLength = px(width).into();
        let edges = &mut self.style().border_widths;
        edges.top = Some(width);
        edges.right = Some(width);
        edges.bottom = Some(width);
        edges.left = Some(width);
        self
    }
}

impl<E: Styled> ScaledEdge for E {}

/// A radio button drawn at `t` (0 off, 1 on).
pub fn radio(t: f32, th: &Theme) -> AnyElement {
    let ring = crate::theme::mix(th.text_dim, th.accent, t);
    div()
        .border_px(2.0)
        .size(px(20.0))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .border_color(rgba(ring))
        .child(div().size(px(10.0 * t)).rounded_full().bg(rgba(th.accent)))
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// Every `raised(...)` and `frosted(...)` panel gets its glass before
    /// any child: the frost is
    /// added as a child, and a child added earlier draws under the glass,
    /// so the panel looks empty with frosted menus on.
    #[test]
    fn frosted_panels_take_children_after_the_glass() {
        let mut wrong = Vec::new();
        walk(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
            &mut wrong,
        );
        assert!(wrong.is_empty(), "children before the glass: {wrong:?}");
    }

    fn walk(dir: &Path, wrong: &mut Vec<String>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, wrong);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let text = std::fs::read_to_string(&path).unwrap();
                // Tests, like the ones here, may spell out wrong calls.
                let code = text.split("#[cfg(test)]").next().unwrap_or_default();
                for line in children_before_glass(code) {
                    wrong.push(format!("{}:{line}", path.display()));
                }
            }
        }
    }

    /// The lines of `raised(` and `frosted(` calls whose panel already has
    /// children: in their first argument, or, for `.map(|d| raised(d, ...))`,
    /// earlier in the chain the call is part of.
    fn children_before_glass(text: &str) -> Vec<usize> {
        let mut lines = Vec::new();
        for name in ["raised(", "frosted("] {
            for (at, _) in text.match_indices(name) {
                let before = &text[..at];
                if before.ends_with("fn ")
                    || before.ends_with(|c: char| c.is_alphanumeric() || c == '_' || c == '.')
                {
                    continue;
                }
                let panel = first_argument(&text[at + name.len()..]);
                let panel = if panel.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    chain_before(before)
                } else {
                    panel
                };
                if panel.contains(".child(") || panel.contains(".children(") {
                    lines.push(before.matches('\n').count() + 1);
                }
            }
        }
        lines.sort_unstable();
        lines
    }

    /// The first argument of a call, from just after its `(`.
    fn first_argument(rest: &str) -> &str {
        let mut depth = 0;
        for (i, c) in rest.char_indices() {
            match c {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' | ',' if depth == 0 => return &rest[..i],
                ')' | ']' | '}' => depth -= 1,
                _ => {}
            }
        }
        rest
    }

    /// The method chain that ends at the `.map(` a call sits in: back to
    /// the `div()` that starts it, skipping `div()`s nested in arguments.
    fn chain_before(before: &str) -> &str {
        let Some(map) = before.rfind(".map(") else {
            return "";
        };
        let head = &before[..map];
        let mut from = head.len();
        while let Some(at) = head[..from].rfind("div()") {
            let chain = &head[at..];
            let mut depth = 0i32;
            let nested = chain.chars().any(|c| {
                match c {
                    '(' | '[' | '{' => depth += 1,
                    ')' | ']' | '}' => depth -= 1,
                    _ => {}
                }
                depth < 0
            });
            if !nested {
                return chain;
            }
            from = at;
        }
        ""
    }

    #[test]
    fn finds_children_before_the_glass() {
        let good = "raised(div().p(px(4.0)), th, 8.0, 3.0).child(body)";
        let bad = "\nraised(div().child(body), th, 8.0, 3.0)";
        assert!(children_before_glass(good).is_empty());
        assert_eq!(children_before_glass(bad), [2]);
        let good = "div()\n.p(px(4.0))\n.map(|d| frosted(d, th, f, 8.0))";
        assert!(children_before_glass(good).is_empty());
        let good = ".child(div().p(px(1.0)))\n.child(\ndiv()\n.map(|d| raised(d, th, 8.0, 3.0))";
        assert!(children_before_glass(good).is_empty());
        let bad = "div()\n.child(body)\n.map(|d| frosted(d, th, f, 8.0))";
        assert_eq!(children_before_glass(bad), [3]);
    }
}
