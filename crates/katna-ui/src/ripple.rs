// SPDX-License-Identifier: GPL-3.0-or-later

//! An ink ripple: on press, a circle grows from the pointer and fades, as in
//! Material Design. Put [`Ripple`] first among the children of a
//! `relative()` element, and give it the element's corner radius with
//! [`Ripple::rounded`] (pills and round buttons need nothing).
//!
//! GPUI clips children to a rectangle, not to rounded corners, so the wave
//! never draws outside the element: it is the circle cut to the element's
//! box, with the element's own corners where it reaches them.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, App, Bounds, ElementId, Hsla, IntoElement, MouseButton, Pixels, Point,
    RenderOnce, Window, canvas, div, ease_out_quint, prelude::*, px,
};

/// How long one ripple lasts.
const DURATION: Duration = Duration::from_millis(550);

#[derive(Default)]
struct State {
    bounds: Rc<Cell<Bounds<Pixels>>>,
    /// Where the last press was, relative to the element, and its number.
    press: Option<(Point<Pixels>, usize)>,
}

/// Corner radii: top left, top right, bottom right, bottom left.
pub type Corners = [f32; 4];

/// The ripple layer. Presses still reach the element under it.
#[derive(IntoElement)]
pub struct Ripple {
    id: ElementId,
    color: Hsla,
    centered: bool,
    corners: Corners,
}

impl Ripple {
    /// `id` must be unique among the ripples of the view.
    pub fn new(id: impl Into<ElementId>, color: impl Into<Hsla>) -> Self {
        Self {
            id: id.into(),
            color: color.into(),
            centered: false,
            corners: [f32::INFINITY; 4],
        }
    }

    /// Grow from the middle instead of the pointer (icon buttons).
    pub fn centered(mut self) -> Self {
        self.centered = true;
        self
    }

    /// The element's corner radius. By default the corners are fully
    /// round, as for pills and round buttons.
    pub fn rounded(self, radius: f32) -> Self {
        self.corners([radius; 4])
    }

    /// The element's corner radii, when they differ.
    pub fn corners(mut self, corners: Corners) -> Self {
        self.corners = corners;
        self
    }
}

/// The wave of radius `r` around (`x`, `y`) cut to a `w` by `h` box with
/// `corners`: its box and its corner radii.
pub fn wave_shape(x: f32, y: f32, r: f32, w: f32, h: f32, corners: Corners) -> ([f32; 4], Corners) {
    let (left, top) = ((x - r).max(0.0), (y - r).max(0.0));
    let (right, bottom) = ((x + r).min(w), (y + r).min(h));
    let (ww, wh) = ((right - left).max(0.0), (bottom - top).max(0.0));
    let cap = |radius: f32, limit: f32| radius.min(limit / 2.0).max(0.0);
    let own = w.min(h);
    let wave = ww.min(wh);
    // A corner of the wave that sits on a corner of the box takes the box's
    // rounding; elsewhere it is the circle's.
    let at = [
        left <= 0.0 && top <= 0.0,
        right >= w && top <= 0.0,
        right >= w && bottom >= h,
        left <= 0.0 && bottom >= h,
    ];
    let mut radii = [0.0; 4];
    for i in 0..4 {
        let radius = if at[i] { cap(corners[i], own) } else { r };
        radii[i] = cap(radius, wave);
    }
    ([left, top, ww, wh], radii)
}

impl RenderOnce for Ripple {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| State::default());
        let bounds = state.read(cx).bounds.clone();
        let press = state.read(cx).press;
        let centered = self.centered;
        let color = self.color;
        let corners = self.corners;

        let wave = press.map(|(origin, n)| {
            let size = bounds.get().size;
            let (w, h) = (f32::from(size.width), f32::from(size.height));
            let (x, y) = (f32::from(origin.x), f32::from(origin.y));
            // Reach the farthest corner.
            let radius = [(0.0, 0.0), (w, 0.0), (0.0, h), (w, h)]
                .iter()
                .map(|(cx, cy)| ((cx - x).powi(2) + (cy - y).powi(2)).sqrt())
                .fold(0.0_f32, f32::max);
            let grow = ease_out_quint();
            div()
                .absolute()
                .with_animation(("wave", n), Animation::new(DURATION), move |el, t| {
                    let r = radius * grow((t / 0.8).min(1.0)).max(0.05);
                    let fade = 1.0 - ((t - 0.4) / 0.6).clamp(0.0, 1.0);
                    let ([left, top, ww, wh], [tl, tr, br, bl]) =
                        wave_shape(x, y, r, w, h, corners);
                    el.left(px(left))
                        .top(px(top))
                        .w(px(ww))
                        .h(px(wh))
                        .rounded_tl(px(tl))
                        .rounded_tr(px(tr))
                        .rounded_br(px(br))
                        .rounded_bl(px(bl))
                        .bg(Hsla {
                            a: color.a * fade,
                            ..color
                        })
                })
        });

        let store = bounds.clone();
        div()
            .id(self.id)
            .absolute()
            .inset_0()
            .child(
                canvas(move |b, _, _| store.set(b), |_, _, _, _| {})
                    .absolute()
                    .size_full(),
            )
            .children(wave)
            .on_mouse_down(MouseButton::Left, move |e, _, cx| {
                let b = bounds.get();
                let origin = if centered {
                    Point::new(b.size.width / 2.0, b.size.height / 2.0)
                } else {
                    e.position - b.origin
                };
                state.update(cx, |s, cx| {
                    let n = s.press.map_or(0, |(_, n)| n + 1);
                    s.press = Some((origin, n));
                    cx.notify();
                });
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_small_wave_is_a_circle() {
        let ([l, t, w, h], radii) = wave_shape(50.0, 20.0, 5.0, 100.0, 40.0, [16.0; 4]);
        assert_eq!([l, t, w, h], [45.0, 15.0, 10.0, 10.0]);
        assert_eq!(radii, [5.0; 4]);
    }

    #[test]
    fn a_full_wave_takes_the_corners_of_the_box() {
        let (rect, radii) = wave_shape(50.0, 20.0, 200.0, 100.0, 40.0, [16.0; 4]);
        assert_eq!(rect, [0.0, 0.0, 100.0, 40.0]);
        assert_eq!(radii, [16.0; 4]);
        // A pill: fully round ends.
        let (_, radii) = wave_shape(50.0, 20.0, 200.0, 100.0, 40.0, [f32::INFINITY; 4]);
        assert_eq!(radii, [20.0; 4]);
        // A square row.
        let (_, radii) = wave_shape(50.0, 20.0, 200.0, 100.0, 40.0, [0.0; 4]);
        assert_eq!(radii, [0.0; 4]);
    }

    #[test]
    fn a_wave_at_one_side_keeps_its_curve_elsewhere() {
        // Pressed near the left: reaches the left corners but not the right.
        let ([l, _, w, _], [tl, tr, br, bl]) = wave_shape(10.0, 20.0, 30.0, 200.0, 40.0, [8.0; 4]);
        assert_eq!((l, w), (0.0, 40.0));
        assert_eq!((tl, bl), (8.0, 8.0));
        assert_eq!((tr, br), (20.0, 20.0));
    }
}
