// SPDX-License-Identifier: GPL-3.0-or-later

//! A button's hover background that eases in, as in Gmail: on a round
//! button the circle grows from the middle while it fades in, and on
//! leaving it fades out quickly where it is. Put [`Glow`] first among the
//! children of a `relative()` button, before its [`crate::Ripple`]; call
//! [`Glow::fade`] for pills, whose background only fades.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use crate::scale::px;
use crate::scale::unpx;
use gpui::{
    Animation, AnimationExt, App, Bounds, ElementId, Hsla, IntoElement, Pixels, RenderOnce, Window,
    canvas, div, ease_out_quint, prelude::*,
};

/// How long the circle takes to grow to full size.
const GROW: Duration = Duration::from_millis(130);
/// How long it takes to fade in.
const FADE_IN: f32 = 90.0;
/// How long it takes to fade out when the pointer leaves.
const FADE_OUT: Duration = Duration::from_millis(70);
/// The circle's size when it starts to grow, against its full size.
const START: f32 = 0.35;

#[derive(Default)]
struct State {
    bounds: Rc<Cell<Bounds<Pixels>>>,
    /// Whether the pointer is on the button, and how many times that
    /// changed, which names the running animation.
    hovered: Option<(bool, usize)>,
}

/// The hover layer. The pointer still reaches the button under it.
#[derive(IntoElement)]
pub struct Glow {
    id: ElementId,
    color: Hsla,
    grow: bool,
}

impl Glow {
    /// `id` must be unique among the glows of the view.
    pub fn new(id: impl Into<ElementId>, color: impl Into<Hsla>) -> Self {
        Self {
            id: id.into(),
            color: color.into(),
            grow: true,
        }
    }

    /// Only fade, filling the whole pill from the start.
    pub fn fade(mut self) -> Self {
        self.grow = false;
        self
    }
}

impl RenderOnce for Glow {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| State::default());
        let bounds = state.read(cx).bounds.clone();
        let color = self.color;
        let grow = self.grow;

        let layer = state.read(cx).hovered.map(|(on, n)| {
            let size = bounds.get().size;
            let (w, h) = (unpx(size.width), unpx(size.height));
            let full = w.min(h);
            let paint = move |el: gpui::Div, scale: f32, alpha: f32| {
                let el = el.bg(Hsla {
                    a: color.a * alpha,
                    ..color
                });
                if grow {
                    let d = full * scale;
                    el.left(px((w - d) / 2.0))
                        .top(px((h - d) / 2.0))
                        .size(px(d))
                        .rounded_full()
                } else {
                    el.inset_0().rounded_full()
                }
            };
            if on {
                let ease = ease_out_quint();
                let ms = GROW.as_secs_f32() * 1000.0;
                div()
                    .absolute()
                    .with_animation(("glow", n), Animation::new(GROW), move |el, t| {
                        let scale = START + (1.0 - START) * ease(t);
                        paint(el, scale, (t * ms / FADE_IN).min(1.0))
                    })
                    .into_any_element()
            } else {
                div()
                    .absolute()
                    .with_animation(("glow", n), Animation::new(FADE_OUT), move |el, t| {
                        paint(el, 1.0, 1.0 - t)
                    })
                    .into_any_element()
            }
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
            .children(layer)
            .on_hover(move |&on, _, cx| {
                state.update(cx, |s, cx| {
                    if s.hovered.map(|(was, _)| was) == Some(on) || (!on && s.hovered.is_none()) {
                        return;
                    }
                    let n = s.hovered.map_or(0, |(_, n)| n + 1);
                    s.hovered = Some((on, n));
                    cx.notify();
                });
            })
    }
}
