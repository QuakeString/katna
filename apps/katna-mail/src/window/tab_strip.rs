// SPDX-License-Identifier: GPL-3.0-or-later

//! A row of tabs that always stays on one line. When the tabs don't fit,
//! the row scrolls sideways by wheel or touchpad, and an arrow shows at
//! each edge that has more tabs past it. The arrows, and picking a tab
//! that is partly hidden, glide the row instead of jumping.

use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    Animation, AnimationExt, AnyElement, ElementId, IntoElement, Pixels, ScrollHandle, canvas, div,
    ease_out_quint, linear_color_stop, linear_gradient, point, prelude::*, rgba,
};
use katna_ui::px;

use crate::theme::{Theme, fade};
use crate::widgets::icon_button;

/// How long the row takes to glide to where it is going.
const GLIDE: Duration = Duration::from_millis(280);
/// The width an arrow covers at its edge, fade included; a tab brought
/// into view clears it.
const ARROW_ROOM: f32 = 56.0;

#[derive(Clone, Copy)]
struct Glide {
    from: Pixels,
    to: Pixels,
    start: Instant,
    /// Where the glide last put the row; anything else means the wheel
    /// moved it, and the glide gives way.
    last: Pixels,
}

#[derive(Clone, Copy, Default)]
struct State {
    glide: Option<Glide>,
    /// A tab to bring into view, and whether to jump there at once.
    reveal: Option<(usize, bool)>,
    /// The arrows drawn by the last render: left, right.
    shown: (bool, bool),
}

#[derive(Clone, Default)]
pub(super) struct TabStrip {
    scroll: ScrollHandle,
    state: Rc<Cell<State>>,
}

impl TabStrip {
    /// Brings tab `ix` fully into view: gliding, or at once when `instant`.
    pub(super) fn reveal(&self, ix: usize, instant: bool) {
        self.update(|s| s.reveal = Some((ix, instant)));
    }

    fn update(&self, f: impl FnOnce(&mut State)) {
        let mut state = self.state.get();
        f(&mut state);
        self.state.set(state);
    }

    /// Whether there are tabs past the left and the right edge.
    fn edges(&self) -> (bool, bool) {
        let x = self.scroll.offset().x;
        let max = self.scroll.max_offset().x;
        (x < px(-0.5), max > px(0.5) && x > -max + px(0.5))
    }

    /// Glides the row most of its width towards `dir` (-1 left, 1 right).
    fn nudge(&self, dir: f32) {
        let now = self.scroll.offset().x;
        let max = self.scroll.max_offset().x;
        let width = self.scroll.bounds().size.width;
        let step = ((width - px(2.0 * ARROW_ROOM)) * 0.8).max(px(120.0));
        let from = self.state.get().glide.map_or(now, |g| g.to);
        let to = (from - step * dir).min(px(0.0)).max(-max);
        self.update(|s| {
            s.glide = Some(Glide {
                from: now,
                to,
                start: Instant::now(),
                last: now,
            })
        });
    }

    /// The row of `tabs`, with arrows unless `arrows` is off (on a phone
    /// the row is swiped instead). `pad` is the room before the first tab
    /// and after the last.
    pub(super) fn render(
        &self,
        id: &'static str,
        tabs: impl IntoIterator<Item = impl IntoElement>,
        arrows: bool,
        pad: f32,
        th: &Theme,
    ) -> AnyElement {
        let (left, right) = if arrows { self.edges() } else { (false, false) };
        self.update(|s| s.shown = (left, right));
        let strip = self.clone();
        let arrow = |dir: f32| {
            let strip = self.clone();
            let solid = rgba(th.surface);
            let clear = rgba(fade(th.surface, 0.0));
            div()
                .id(ElementId::Name(if dir < 0.0 {
                    "tab-strip-left".into()
                } else {
                    "tab-strip-right".into()
                }))
                .absolute()
                .top_0()
                .bottom_0()
                .w(px(ARROW_ROOM))
                .map(|d| if dir < 0.0 { d.left_0() } else { d.right_0() })
                .flex()
                .items_center()
                .map(|d| {
                    if dir < 0.0 {
                        d.justify_start()
                    } else {
                        d.justify_end()
                    }
                })
                .px(px(4.0))
                // Solid under the button, fading out over the tabs.
                .bg(linear_gradient(
                    if dir < 0.0 { 90.0 } else { 270.0 },
                    linear_color_stop(solid, 0.6),
                    linear_color_stop(clear, 1.0),
                ))
                .occlude()
                .child(
                    icon_button(
                        ElementId::Name(if dir < 0.0 {
                            "tab-strip-left-button".into()
                        } else {
                            "tab-strip-right-button".into()
                        }),
                        if dir < 0.0 {
                            "chevron-left"
                        } else {
                            "chevron-right"
                        },
                        20.0,
                        th,
                    )
                    .on_click(move |_, window, _| {
                        strip.nudge(dir);
                        window.refresh();
                    }),
                )
                .with_animation(
                    ElementId::Name(if dir < 0.0 {
                        "tab-strip-left-in".into()
                    } else {
                        "tab-strip-right-in".into()
                    }),
                    Animation::new(Duration::from_millis(160)).with_easing(ease_out_quint()),
                    |el, t| el.opacity(t),
                )
        };
        div()
            .relative()
            .flex_none()
            .border_b_1()
            .border_color(rgba(th.divider))
            .child(
                div()
                    .id(id)
                    .flex()
                    .flex_row()
                    .px(px(pad))
                    .overflow_x_scroll()
                    .track_scroll(&self.scroll)
                    .children(tabs),
            )
            .child(
                canvas(
                    move |_, window, _| strip.prepaint(arrows, window),
                    |_, _, _, _| {},
                )
                .absolute()
                .size_0(),
            )
            .when(left, |d| d.child(arrow(-1.0)))
            .when(right, |d| d.child(arrow(1.0)))
            .into_any_element()
    }

    /// Runs each frame after the row is laid out: brings a picked tab into
    /// view, moves a glide along, and redraws when the arrows need to
    /// change.
    fn prepaint(&self, arrows: bool, window: &mut gpui::Window) {
        let mut state = self.state.get();
        let view = self.scroll.bounds();
        let max = self.scroll.max_offset().x;
        let offset = self.scroll.offset();
        if let Some((ix, instant)) = state.reveal.take() {
            match self.scroll.bounds_for_item(ix) {
                Some(item) if view.size.width > px(0.0) => {
                    let room = px(if arrows && max > px(0.0) {
                        ARROW_ROOM
                    } else {
                        0.0
                    });
                    let start = item.left() - view.left() - room;
                    let end = item.right() - view.left() + room;
                    let from = state.glide.map_or(offset.x, |g| g.to);
                    let mut to = from;
                    if start < -from {
                        to = -start;
                    } else if end > -from + view.size.width {
                        to = view.size.width - end;
                    }
                    let to = to.min(px(0.0)).max(-max);
                    if instant {
                        state.glide = None;
                        self.scroll.set_offset(point(to, offset.y));
                    } else if to != offset.x {
                        state.glide = Some(Glide {
                            from: offset.x,
                            to,
                            start: Instant::now(),
                            last: offset.x,
                        });
                    }
                    window.request_animation_frame();
                }
                // Not laid out yet: try again next frame.
                _ => {
                    state.reveal = Some((ix, instant));
                    window.request_animation_frame();
                }
            }
        }
        if let Some(mut glide) = state.glide {
            if (self.scroll.offset().x - glide.last).abs() > px(0.5) {
                state.glide = None;
            } else {
                let t = (glide.start.elapsed().as_secs_f32() / GLIDE.as_secs_f32()).min(1.0);
                let eased = 1.0 - (1.0 - t).powi(3);
                let x = (glide.from + (glide.to - glide.from) * eased)
                    .min(px(0.0))
                    .max(-max);
                self.scroll.set_offset(point(x, offset.y));
                glide.last = x;
                state.glide = (t < 1.0).then_some(glide);
                window.request_animation_frame();
            }
        }
        if arrows && self.edges() != state.shown {
            window.request_animation_frame();
        }
        self.state.set(state);
    }
}
