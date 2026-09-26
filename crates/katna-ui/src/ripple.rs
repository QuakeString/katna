// SPDX-License-Identifier: GPL-3.0-or-later

//! An ink ripple: on press, a circle grows from the pointer and fades, as in
//! Material Design. Put [`Ripple`] first among the children of a
//! `relative()` element with rounded corners and `overflow_hidden()`.

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

/// The ripple layer. Presses still reach the element under it.
#[derive(IntoElement)]
pub struct Ripple {
    id: ElementId,
    color: Hsla,
    centered: bool,
}

impl Ripple {
    /// `id` must be unique among the ripples of the view.
    pub fn new(id: impl Into<ElementId>, color: impl Into<Hsla>) -> Self {
        Self {
            id: id.into(),
            color: color.into(),
            centered: false,
        }
    }

    /// Grow from the middle instead of the pointer (icon buttons).
    pub fn centered(mut self) -> Self {
        self.centered = true;
        self
    }
}

impl RenderOnce for Ripple {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| State::default());
        let bounds = state.read(cx).bounds.clone();
        let press = state.read(cx).press;
        let centered = self.centered;
        let color = self.color;

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
            div().absolute().rounded_full().with_animation(
                ("wave", n),
                Animation::new(DURATION),
                move |el, t| {
                    let r = radius * grow((t / 0.8).min(1.0)).max(0.05);
                    let fade = 1.0 - ((t - 0.4) / 0.6).clamp(0.0, 1.0);
                    el.left(px(x - r))
                        .top(px(y - r))
                        .size(px(2.0 * r))
                        .bg(Hsla {
                            a: color.a * fade,
                            ..color
                        })
                },
            )
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
