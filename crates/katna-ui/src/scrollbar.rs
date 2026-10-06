// SPDX-License-Identifier: GPL-3.0-or-later

//! A thin scrollbar over a [`gpui::list`] or a scrolling `div`, like
//! KDE's overlay scrollbars: it shows while the list scrolls and while
//! the pointer is over the list, then fades out. It is a hairline at rest
//! and grows as the pointer nears the right edge. Its thumb drags, and a
//! click on the track above or below it jumps there.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    App, Bounds, DispatchPhase, ElementId, IntoElement, ListState, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, ParentElement, Pixels, ScrollHandle, Stateful, Styled, Window,
    canvas, div, point, prelude::*, rgba,
};

use crate::motion::{SMOOTH, Spring, lerp};
use crate::{px, unpx};

/// How long the bar stays after the list stops scrolling.
const LINGER: Duration = Duration::from_millis(900);
/// The shortest thumb, so a long list's thumb can still be grabbed.
const MIN_THUMB: f32 = 32.0;
/// The strip along the right edge that takes clicks and drags.
const TRACK: f32 = 14.0;
/// The thumb's width at rest.
const THIN: f32 = 3.0;
/// The thumb's width while the pointer is on it or drags it.
const WIDE: f32 = 9.0;
/// How far from the right edge the pointer starts to widen the thumb.
const REACH: f32 = 96.0;

/// What the bar scrolls: a [`gpui::list`]'s [`ListState`] or a scrolling
/// `div`'s [`ScrollHandle`].
pub trait Scrolls: Clone + 'static {
    /// The visible height, the most it can scroll, and how far it has.
    fn heights(&self) -> (f32, f32, f32);
    /// Scrolls to `offset` down from the top, from the bar.
    fn scroll_to(&self, offset: f32);
    fn drag_started(&self) {}
    fn drag_ended(&self) {}
}

impl Scrolls for ListState {
    fn heights(&self) -> (f32, f32, f32) {
        (
            unpx(self.viewport_bounds().size.height),
            unpx(self.max_offset_for_scrollbar().y),
            -unpx(self.scroll_px_offset_for_scrollbar().y),
        )
    }

    fn scroll_to(&self, offset: f32) {
        self.set_offset_from_scrollbar(point(px(0.0), -px(offset)));
    }

    fn drag_started(&self) {
        self.scrollbar_drag_started();
    }

    fn drag_ended(&self) {
        self.scrollbar_drag_ended();
    }
}

impl Scrolls for ScrollHandle {
    fn heights(&self) -> (f32, f32, f32) {
        (
            unpx(self.bounds().size.height),
            unpx(self.max_offset().y),
            -unpx(self.offset().y),
        )
    }

    fn scroll_to(&self, offset: f32) {
        self.set_offset(point(self.offset().x, -px(offset)));
    }
}

/// The bar's state; keep one per list, beside its [`ListState`] or
/// [`ScrollHandle`].
#[derive(Clone)]
pub struct ScrollBar(Rc<RefCell<Inner>>);

struct Inner {
    opacity: Spring,
    /// Where the list was scrolled at the last frame.
    offset: f32,
    scrolled: Option<Instant>,
    over_list: bool,
    over_bar: bool,
    /// How close the pointer is to the right edge, from 0 (`REACH` or
    /// more away) to 1 (on the edge).
    near: f32,
    /// The thumb growing from `THIN` (0) to `WIDE` (1).
    grow: Spring,
    /// While the thumb drags: where on the thumb it was grabbed.
    grab: Option<f32>,
    track: Option<Bounds<Pixels>>,
    /// A frame is asked for when the bar should start to fade.
    timer: bool,
    /// How strongly the bar shows this frame, and how wide.
    strength: f32,
    wide: f32,
}

impl Default for ScrollBar {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Inner {
            opacity: Spring::new(SMOOTH, 0.0),
            offset: 0.0,
            scrolled: None,
            over_list: false,
            over_bar: false,
            near: 0.0,
            grow: Spring::new(SMOOTH, 0.0),
            grab: None,
            track: None,
            timer: false,
            strength: 0.0,
            wide: 0.0,
        })))
    }
}

/// The thumb's size and place, in unscaled pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Thumb {
    /// Scrolled distance, and the most it can scroll.
    offset: f32,
    max: f32,
    /// The visible height, the thumb's height and its top.
    view: f32,
    height: f32,
    top: f32,
}

impl Thumb {
    fn new(view: f32, max: f32, offset: f32) -> Option<Self> {
        if max < 1.0 || view < 1.0 {
            return None;
        }
        let height = (view * view / (view + max)).max(MIN_THUMB).min(view);
        let offset = offset.clamp(0.0, max);
        Some(Self {
            offset,
            max,
            view,
            height,
            top: offset / max * (view - height),
        })
    }

    /// The scroll offset that puts the thumb's top at `top`.
    fn offset_at(&self, top: f32) -> f32 {
        let room = self.view - self.height;
        if room <= 0.0 {
            return 0.0;
        }
        top.clamp(0.0, room) / room * self.max
    }
}

impl ScrollBar {
    /// `content`, the list or div scrolled by `state`, with the bar over its
    /// right edge. `color` is the thumb's `0xRRGGBBAA` at full strength.
    pub fn wrap(
        &self,
        id: impl Into<ElementId>,
        state: &impl Scrolls,
        content: impl IntoElement,
        color: u32,
        window: &mut Window,
        cx: &mut App,
    ) -> Stateful<gpui::Div> {
        self.tick(state, window, cx);
        self.draw(id, state, content, color)
    }

    /// Moves the bar toward shown or hidden for this frame: call once a
    /// frame before [`ScrollBar::draw`], where the window is at hand.
    pub fn tick(&self, state: &impl Scrolls, window: &mut Window, cx: &mut App) {
        let (visible, max, scrolled) = state.heights();
        let thumb = Thumb::new(visible, max, scrolled);
        let now = Instant::now();
        let mut inner = self.0.borrow_mut();
        if let Some(thumb) = thumb
            && (thumb.offset - inner.offset).abs() > 0.5
        {
            inner.offset = thumb.offset;
            inner.scrolled = Some(now);
        }
        let lingering = inner
            .scrolled
            .is_some_and(|at| now.saturating_duration_since(at) < LINGER);
        if lingering && !inner.timer {
            // One more frame when the bar should start to fade.
            inner.timer = true;
            let bar = self.0.clone();
            window
                .spawn(cx, async move |cx| {
                    cx.background_executor().timer(LINGER).await;
                    bar.borrow_mut().timer = false;
                    cx.update(|window, _| window.refresh()).ok();
                })
                .detach();
        }
        let shown = thumb.is_some() && (inner.over_list || inner.grab.is_some() || lingering);
        inner.opacity.set(if shown { 1.0 } else { 0.0 });
        inner.strength = inner.opacity.tick(window, cx.reduce_motion());
        let grow = if inner.over_bar || inner.grab.is_some() {
            1.0
        } else if inner.over_list {
            inner.near
        } else {
            0.0
        };
        inner.grow.set(grow);
        inner.wide = inner.grow.tick(window, cx.reduce_motion());
    }

    /// `content` with the bar over its right edge, as [`ScrollBar::tick`]
    /// left it this frame.
    pub fn draw(
        &self,
        id: impl Into<ElementId>,
        state: &impl Scrolls,
        content: impl IntoElement,
        color: u32,
    ) -> Stateful<gpui::Div> {
        let (visible, max, scrolled) = state.heights();
        let thumb = Thumb::new(visible, max, scrolled);
        let inner = self.0.borrow();
        let strength = inner.strength;
        let wide = inner.wide.clamp(0.0, 1.0);
        let grabbed = inner.grab;
        drop(inner);

        let (bar, nearing) = (self.0.clone(), self.0.clone());
        let root = div()
            .id(id)
            .relative()
            .size_full()
            .on_hover(move |hovered, window, _| {
                bar.borrow_mut().over_list = *hovered;
                window.refresh();
            })
            .on_mouse_move(move |event: &MouseMoveEvent, window, _| {
                let mut inner = nearing.borrow_mut();
                let Some(track) = inner.track else {
                    return;
                };
                let away = unpx(track.right() - event.position.x).max(0.0);
                let near = (1.0 - away / REACH).clamp(0.0, 1.0);
                if (near - inner.near).abs() > 0.01 {
                    inner.near = near;
                    window.refresh();
                }
            })
            .child(content);
        let Some(thumb) = thumb.filter(|_| strength > 0.01) else {
            return root;
        };
        let alpha = (color & 0xff) as f32 * lerp(0.75, 1.0, wide) * strength;
        let fill = color & 0xffff_ff00 | alpha.round().clamp(0.0, 255.0) as u32;
        let width = lerp(THIN, WIDE, wide);

        let (hover, press, record, list) = (
            self.0.clone(),
            self.0.clone(),
            self.0.clone(),
            state.clone(),
        );
        let (drag, drag_list) = (self.0.clone(), state.clone());
        root.child(
            div()
                .id("scrollbar")
                .absolute()
                .top_0()
                .right_0()
                .bottom_0()
                .w(px(TRACK))
                .on_hover(move |hovered, window, _| {
                    hover.borrow_mut().over_bar = *hovered;
                    window.refresh();
                })
                .on_mouse_down(
                    MouseButton::Left,
                    move |event: &MouseDownEvent, window, cx| {
                        cx.stop_propagation();
                        let mut inner = press.borrow_mut();
                        let Some(track) = inner.track else {
                            return;
                        };
                        let y = unpx(event.position.y - track.top());
                        let grab = if (thumb.top..thumb.top + thumb.height).contains(&y) {
                            y - thumb.top
                        } else {
                            // A click on the track brings the thumb's middle
                            // there, and keeps it under the pointer.
                            let grab = thumb.height / 2.0;
                            list.scroll_to(thumb.offset_at(y - grab));
                            grab
                        };
                        inner.grab = Some(grab);
                        list.drag_started();
                        window.refresh();
                    },
                )
                .child({
                    // The thumb is placed as it paints, from the sizes laid
                    // out this frame: those read while building it are last
                    // frame's, so a list that just grew or shrank (a bar
                    // above it came or went) would show it out of place for
                    // a frame.
                    let laid = state.clone();
                    canvas(
                        |_, _, _| (),
                        move |track, (), window, _| {
                            let (visible, max, scrolled) = laid.heights();
                            let at = Thumb::new(visible, max, scrolled).unwrap_or(thumb);
                            let bounds = Bounds::new(
                                point(
                                    track.right() - px((TRACK + width) / 2.0),
                                    track.top() + px(at.top),
                                ),
                                gpui::size(px(width), px(at.height)),
                            );
                            window.paint_quad(
                                gpui::fill(bounds, rgba(fill)).corner_radii(px(width / 2.0)),
                            );
                        },
                    )
                    .absolute()
                    .size_full()
                })
                .child(
                    canvas(
                        move |bounds, _, _| record.borrow_mut().track = Some(bounds),
                        move |_, _, window, _| {
                            let Some(grab) = grabbed else {
                                return;
                            };
                            // While dragging, the pointer counts anywhere
                            // in the window.
                            let (moved, released) = (drag.clone(), drag);
                            let list = drag_list.clone();
                            window.on_mouse_event(
                                move |event: &MouseMoveEvent, phase, window, _| {
                                    if phase != DispatchPhase::Bubble {
                                        return;
                                    }
                                    let Some(track) = moved.borrow().track else {
                                        return;
                                    };
                                    let y = unpx(event.position.y - track.top());
                                    list.scroll_to(thumb.offset_at(y - grab));
                                    window.refresh();
                                },
                            );
                            window.on_mouse_event(move |_: &MouseUpEvent, phase, window, _| {
                                if phase != DispatchPhase::Bubble {
                                    return;
                                }
                                released.borrow_mut().grab = None;
                                drag_list.drag_ended();
                                window.refresh();
                            });
                        },
                    )
                    .absolute()
                    .size_full(),
                ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thumb_follows_the_scrolling() {
        assert_eq!(Thumb::new(500.0, 0.0, 0.0), None);
        let top = Thumb::new(500.0, 500.0, 0.0).unwrap();
        assert_eq!((top.height, top.top), (250.0, 0.0));
        let end = Thumb::new(500.0, 500.0, 500.0).unwrap();
        assert_eq!(end.top, 250.0);
        assert_eq!(end.offset_at(125.0), 250.0);
        assert_eq!(end.offset_at(-10.0), 0.0);
        assert_eq!(end.offset_at(9999.0), 500.0);
        // A very long list keeps a thumb that can be grabbed.
        assert_eq!(Thumb::new(500.0, 1e6, 0.0).unwrap().height, MIN_THUMB);
    }
}
