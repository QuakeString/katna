// SPDX-License-Identifier: GPL-3.0-or-later

//! A thin scrollbar over a [`gpui::list`], like KDE's overlay scrollbars:
//! it shows while the list scrolls and while the pointer is over the
//! list, then fades out. Its thumb drags, and a click on the track above
//! or below it jumps there.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    App, Bounds, DispatchPhase, ElementId, IntoElement, ListState, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, ParentElement, Pixels, Stateful, Styled, Window, canvas, div,
    point, prelude::*, rgba,
};

use crate::motion::{SMOOTH, Spring};
use crate::{px, unpx};

/// How long the bar stays after the list stops scrolling.
const LINGER: Duration = Duration::from_millis(900);
/// The shortest thumb, so a long list's thumb can still be grabbed.
const MIN_THUMB: f32 = 32.0;
/// The strip along the right edge that takes clicks and drags.
const TRACK: f32 = 14.0;
const THIN: f32 = 6.0;
/// The thumb's width while the pointer is on it or drags it.
const WIDE: f32 = 8.0;

/// The bar's state; keep one per list, beside its [`ListState`].
#[derive(Clone)]
pub struct ScrollBar(Rc<RefCell<Inner>>);

struct Inner {
    opacity: Spring,
    /// Where the list was scrolled at the last frame.
    offset: f32,
    scrolled: Option<Instant>,
    over_list: bool,
    over_bar: bool,
    /// While the thumb drags: where on the thumb it was grabbed.
    grab: Option<f32>,
    track: Option<Bounds<Pixels>>,
    /// A frame is asked for when the bar should start to fade.
    timer: bool,
}

impl Default for ScrollBar {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Inner {
            opacity: Spring::new(SMOOTH, 0.0),
            offset: 0.0,
            scrolled: None,
            over_list: false,
            over_bar: false,
            grab: None,
            track: None,
            timer: false,
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
    /// `content`, the list drawn from `state`, with the bar over its
    /// right edge. `color` is the thumb's `0xRRGGBBAA` at full strength.
    pub fn wrap(
        &self,
        id: impl Into<ElementId>,
        state: &ListState,
        content: impl IntoElement,
        color: u32,
        window: &mut Window,
        cx: &mut App,
    ) -> Stateful<gpui::Div> {
        let thumb = Thumb::new(
            unpx(state.viewport_bounds().size.height),
            unpx(state.max_offset_for_scrollbar().y),
            -unpx(state.scroll_px_offset_for_scrollbar().y),
        );
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
        let strength = inner.opacity.tick(window, cx.reduce_motion());
        let wide = inner.over_bar || inner.grab.is_some();
        let grabbed = inner.grab;
        drop(inner);

        let bar = self.0.clone();
        let root = div()
            .id(id)
            .relative()
            .size_full()
            .on_hover(move |hovered, window, _| {
                bar.borrow_mut().over_list = *hovered;
                window.refresh();
            })
            .child(content);
        let Some(thumb) = thumb.filter(|_| strength > 0.01) else {
            return root;
        };
        let alpha = (color & 0xff) as f32 * if wide { 1.0 } else { 0.75 } * strength;
        let fill = color & 0xffff_ff00 | alpha.round().clamp(0.0, 255.0) as u32;
        let width = if wide { WIDE } else { THIN };

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
                            list.set_offset_from_scrollbar(point(
                                px(0.0),
                                -px(thumb.offset_at(y - grab)),
                            ));
                            grab
                        };
                        inner.grab = Some(grab);
                        list.scrollbar_drag_started();
                        window.refresh();
                    },
                )
                .child(
                    div()
                        .absolute()
                        .right(px((TRACK - width) / 2.0))
                        .top(px(thumb.top))
                        .w(px(width))
                        .h(px(thumb.height))
                        .rounded_full()
                        .bg(rgba(fill)),
                )
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
                                    list.set_offset_from_scrollbar(point(
                                        px(0.0),
                                        -px(thumb.offset_at(y - grab)),
                                    ));
                                    window.refresh();
                                },
                            );
                            window.on_mouse_event(move |_: &MouseUpEvent, phase, window, _| {
                                if phase != DispatchPhase::Bubble {
                                    return;
                                }
                                released.borrow_mut().grab = None;
                                drag_list.scrollbar_drag_ended();
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
