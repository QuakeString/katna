// SPDX-License-Identifier: GPL-3.0-or-later

//! A button's hover background that eases in softly: on a round button
//! the circle grows from two thirds of its size while it fades in, and on
//! leaving it shrinks back a little as it fades out. Put [`Glow`] first among the
//! children of a `relative()` button, before its [`crate::Ripple`]; call
//! [`Glow::fade`] for pills, whose background only fades, and
//! [`Glow::corners`] for one part of a split button.

use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::scale::px;
use crate::scale::unpx;
use gpui::{
    Animation, AnimationExt, App, Bounds, ElementId, Hsla, IntoElement, Pixels, RenderOnce, Window,
    canvas, div, prelude::*,
};

/// How long the circle takes to grow to full size, and a pill to fade in.
const GROW: Duration = Duration::from_millis(220);
/// How long it takes to fade out (and the circle to shrink back towards
/// [`START`]) when the pointer leaves.
const FADE_OUT: Duration = Duration::from_millis(160);
/// The circle's size when it starts to grow, against its full size.
const START: f32 = 0.6;

/// CSS's `ease-out` and `ease-in`, near enough.
fn ease_out(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

fn ease_in(t: f32) -> f32 {
    t * t
}

/// One run of the layer towards shown (`on`) or hidden.
#[derive(Clone, Copy)]
struct Run {
    on: bool,
    /// Names the running animation.
    n: usize,
    /// When it started, how much showed then, and how long it takes.
    at: Instant,
    from: f32,
    takes: Duration,
}

impl Run {
    /// How much of the layer shows `t` of the way through the run: it
    /// starts where the last run had got to, so a pointer that leaves
    /// before the layer is in fades it out from there, never from full.
    fn shown(&self, t: f32) -> f32 {
        if self.on {
            self.from + (1.0 - self.from) * ease_out(t)
        } else {
            self.from * (1.0 - ease_in(t))
        }
    }

    fn shown_at(&self, now: Instant) -> f32 {
        let t = if self.takes.is_zero() {
            1.0
        } else {
            (now.saturating_duration_since(self.at).as_secs_f32() / self.takes.as_secs_f32())
                .min(1.0)
        };
        self.shown(t)
    }

    /// The run after this one (or none) when the pointer comes (`on`) or
    /// goes at `now`; `None` when nothing changes.
    fn next(last: Option<Run>, on: bool, now: Instant) -> Option<Run> {
        match last {
            Some(run) if run.on == on => return None,
            None if !on => return None,
            _ => {}
        }
        let from = last.map_or(0.0, |run| run.shown_at(now));
        let takes = if on {
            crate::motion::time(GROW).mul_f32(1.0 - from)
        } else {
            crate::motion::time(FADE_OUT).mul_f32(from)
        };
        Some(Run {
            on,
            n: last.map_or(0, |run| run.n + 1),
            at: now,
            from,
            takes,
        })
    }
}

#[derive(Default)]
struct State {
    bounds: Rc<Cell<Bounds<Pixels>>>,
    /// The pointer's last change, if it ever came.
    run: Option<Run>,
}

/// The hover layer. The pointer still reaches the button under it.
#[derive(IntoElement)]
pub struct Glow {
    id: ElementId,
    color: Hsla,
    grow: bool,
    /// The fading layer's corners, top left round to bottom left; `None`
    /// rounds it fully.
    corners: Option<[f32; 4]>,
}

impl Glow {
    /// `id` must be unique among the glows of the view.
    pub fn new(id: impl Into<ElementId>, color: impl Into<Hsla>) -> Self {
        Self {
            id: id.into(),
            color: color.into(),
            grow: true,
            corners: None,
        }
    }

    /// Only fade, filling the whole pill from the start.
    pub fn fade(mut self) -> Self {
        self.grow = false;
        self
    }

    /// Only fade, with these corners (top left, top right, bottom right,
    /// bottom left): one half of a split button, square where it meets
    /// the other.
    pub fn corners(mut self, corners: [f32; 4]) -> Self {
        self.grow = false;
        self.corners = Some(corners);
        self
    }
}

impl RenderOnce for Glow {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| State::default());
        let bounds = state.read(cx).bounds.clone();
        let color = self.color;
        let grow = self.grow;
        let corners = self.corners;

        let layer = state.read(cx).run.map(|run| {
            let size = bounds.get().size;
            let (w, h) = (unpx(size.width), unpx(size.height));
            let full = w.min(h);
            let paint = move |el: gpui::Div, shown: f32| {
                let el = el.bg(Hsla {
                    a: color.a * shown,
                    ..color
                });
                if grow {
                    // The circle grows from START as it fades in, and
                    // shrinks back towards it as it fades out.
                    let d = full * (START + (1.0 - START) * shown);
                    el.left(px((w - d) / 2.0))
                        .top(px((h - d) / 2.0))
                        .size(px(d))
                        .rounded_full()
                } else if let Some([tl, tr, br, bl]) = corners {
                    el.inset_0()
                        .rounded_tl(px(tl))
                        .rounded_tr(px(tr))
                        .rounded_br(px(br))
                        .rounded_bl(px(bl))
                } else {
                    el.inset_0().rounded_full()
                }
            };
            div()
                .absolute()
                .with_animation(
                    ("glow", run.n),
                    Animation::new(run.takes.max(Duration::from_millis(1))),
                    move |el, t| paint(el, run.shown(t)),
                )
                .into_any_element()
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
                    if let Some(run) = Run::next(s.run, on, Instant::now()) {
                        s.run = Some(run);
                        cx.notify();
                    }
                });
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaving_early_fades_from_where_it_got_to() {
        let start = Instant::now();
        let came = Run::next(None, true, start).unwrap();
        assert_eq!(came.from, 0.0);
        // The pointer only brushed past: the layer had barely begun.
        let went = Run::next(Some(came), false, start + Duration::from_millis(10)).unwrap();
        assert!(
            went.from < 0.3,
            "it fades out from {}, not from full",
            went.from
        );
        assert!(went.shown(0.0) < 0.3);
        assert_eq!(went.shown(1.0), 0.0);
    }

    #[test]
    fn coming_back_carries_on_from_where_it_was() {
        let start = Instant::now();
        let came = Run::next(None, true, start).unwrap();
        let shown = came.shown_at(start + GROW * 4);
        assert_eq!(shown, 1.0);
        let went = Run::next(Some(came), false, start + GROW * 4).unwrap();
        assert_eq!(went.from, 1.0);
        let back = Run::next(Some(went), true, start + GROW * 4).unwrap();
        assert_eq!(back.from, 1.0);
        assert_eq!(back.n, 2);
    }

    #[test]
    fn no_change_is_no_run() {
        let now = Instant::now();
        assert!(Run::next(None, false, now).is_none());
        let came = Run::next(None, true, now).unwrap();
        assert!(Run::next(Some(came), true, now).is_none());
    }
}
