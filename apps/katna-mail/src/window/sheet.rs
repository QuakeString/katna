// SPDX-License-Identifier: GPL-3.0-or-later

//! A sheet that rises from the bottom of a phone's window, where a desktop
//! has room for a menu or a card beside what it is about: a chat bubble's
//! menu (a long press) and a person's card. It rests over a faint veil;
//! a tap outside or a swipe down on it puts it away.

use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gpui::{
    AnyElement, Context, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Window,
    deferred, div, prelude::*, rgba,
};
use katna_ui::{px, unpx};

use super::MailWindow;
use crate::theme::Theme;
use crate::widgets::{elevation, frosted};

/// How long the sheet takes to rise or sink.
const SLIDE: Duration = Duration::from_millis(280);
/// Its top corners. The bottom ones sit below the window's edge.
const RADIUS: f32 = 18.0;
/// The height of the strip with the grab handle, where a swipe starts.
const GRAB: f32 = 22.0;
/// How far down a swipe goes before it puts the sheet away, at least.
const SWIPE_AWAY: f32 = 80.0;
/// The veil over the window behind it, at its darkest.
const VEIL: f32 = 0.16;
/// The most of the window it takes.
const TALLEST: f32 = 0.88;

/// Where a sheet is: rising, shown, sinking or put away, and dragged.
pub(super) struct Sheet {
    shown: bool,
    since: Instant,
    /// How far up it was when it last turned.
    from: f32,
    /// How far a swipe has pulled it down, and where the swipe started.
    pulled: Rc<Cell<f32>>,
    swipe: Rc<Cell<Option<f32>>>,
    /// Its height, as last drawn.
    height: Rc<Cell<f32>>,
}

impl Sheet {
    pub(super) fn new() -> Self {
        Self {
            shown: false,
            since: Instant::now() - SLIDE,
            from: 0.0,
            pulled: Rc::default(),
            swipe: Rc::default(),
            height: Rc::default(),
        }
    }

    /// A sheet that rises as it is first drawn.
    pub(super) fn rising() -> Self {
        let mut sheet = Self::new();
        sheet.show(true);
        sheet
    }

    pub(super) fn is_shown(&self) -> bool {
        self.shown
    }

    /// Rises (`true`) or sinks from wherever it is now.
    pub(super) fn show(&mut self, shown: bool) {
        if shown == self.shown {
            return;
        }
        self.from = self.t();
        self.shown = shown;
        self.since = Instant::now();
    }

    /// 0 = put away, 1 = all the way up, eased.
    fn t(&self) -> f32 {
        let p = (self.since.elapsed().as_secs_f32() / SLIDE.as_secs_f32()).min(1.0);
        let eased = 1.0 - (1.0 - p).powi(3);
        let to = if self.shown { 1.0 } else { 0.0 };
        self.from + (to - self.from) * eased
    }

    fn moving(&self) -> bool {
        self.since.elapsed() < SLIDE
    }

    /// A swipe let go: away if it went far enough, else back up from
    /// where it was let go.
    fn let_go(&mut self) -> bool {
        let pulled = self.pulled.replace(0.0);
        self.swipe.set(None);
        let height = self.height.get().max(1.0);
        let t = (1.0 - pulled / height).clamp(0.0, 1.0);
        let away = pulled > SWIPE_AWAY.min(height * 0.5);
        self.shown = !away;
        self.from = t;
        self.since = Instant::now();
        away
    }
}

/// What the sheet is filled with.
pub(super) enum Fill {
    /// A menu's colour, as solid as a dialog's over a busy chat.
    Menu,
    /// A floating card's colour: the cards' own in light colours, a step
    /// lighter in dark ones, so it stands out over the chat.
    Card,
}

impl MailWindow {
    /// Draws `sheet` with `body` while it shows; `sheet_of` finds it again
    /// and `close` puts it away (a tap outside or a swipe). `body` scrolls
    /// when it is taller than the window allows.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn bottom_sheet(
        &self,
        id: &'static str,
        sheet: &Sheet,
        fill: Fill,
        body: AnyElement,
        sheet_of: fn(&mut Self) -> Option<&mut Sheet>,
        close: fn(&mut Self, &mut Context<Self>),
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let t = sheet.t();
        if !sheet.shown && t <= 0.001 {
            return None;
        }
        if sheet.moving() {
            window.request_animation_frame();
        }
        let reduce = cx.reduce_motion();
        let t = if reduce {
            if sheet.shown { 1.0 } else { 0.0 }
        } else {
            t
        };
        // Not yet measured: drawn unseen once, so it rises from below.
        let height = sheet.height.get();
        if height <= 0.0 {
            window.request_animation_frame();
        }
        let pulled = sheet.pulled.get();
        let below = (1.0 - t) * height + pulled;
        let veil = VEIL * t * (1.0 - pulled / height.max(1.0)).clamp(0.0, 1.0);
        let measured = sheet.height.clone();
        // A swipe follows the pointer over the sheet and the veil alike,
        // which hide everything behind them.
        let follow = move |this: &mut Self, e: &MouseMoveEvent, cx: &mut Context<Self>| {
            let Some(sheet) = sheet_of(this) else {
                return;
            };
            let Some(start) = sheet.swipe.get() else {
                return;
            };
            if e.pressed_button != Some(MouseButton::Left) {
                return;
            }
            sheet.pulled.set((unpx(e.position.y) - start).max(0.0));
            cx.notify();
        };
        let let_go = move |this: &mut Self, cx: &mut Context<Self>| {
            let Some(sheet) = sheet_of(this) else {
                return;
            };
            if sheet.swipe.get().is_none() {
                return;
            }
            if sheet.let_go() {
                close(this, cx);
            }
            cx.notify();
        };
        let swipe = sheet.swipe.clone();
        let panel = div()
            .id(id)
            .absolute()
            .left_0()
            .right_0()
            // The bottom corners sit below the window's edge.
            .bottom(px(-RADIUS - below))
            .max_h(gpui::relative(TALLEST))
            .when(height <= 0.0, |d| d.opacity(0.0))
            .flex()
            .flex_col()
            .occlude()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_mouse_move(cx.listener(move |this, e, _, cx| follow(this, e, cx)))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(move |this, _: &MouseUpEvent, _, cx| let_go(this, cx)),
            )
            .map(|d| match fill {
                Fill::Menu => frosted(d, th, th.menu, RADIUS),
                Fill::Card => frosted(d, th, th.raised, RADIUS),
            })
            .shadow(elevation(th, 4.0))
            .pb(px(RADIUS))
            .child(
                div()
                    .id((id, 1_usize))
                    .flex_none()
                    .h(px(GRAB))
                    .w_full()
                    .flex()
                    .justify_center()
                    .items_center()
                    .cursor_grab()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |_, e: &MouseDownEvent, _, cx| {
                            swipe.set(Some(unpx(e.position.y)));
                            cx.stop_propagation();
                        }),
                    )
                    .child(
                        div()
                            .w(px(36.0))
                            .h(px(4.0))
                            .rounded_full()
                            .bg(rgba(th.text_faint))
                            .opacity(0.5),
                    ),
            )
            .child(
                div()
                    .id((id, 2_usize))
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .child(body),
            )
            .child(
                gpui::canvas(
                    move |bounds, _, _| {
                        measured.set(unpx(bounds.size.height) - RADIUS);
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            );
        Some(
            deferred(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .child(
                        div()
                            .id((id, 0_usize))
                            .absolute()
                            .size_full()
                            .bg(gpui::rgba(veil_color(veil)))
                            .occlude()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| close(this, cx)),
                            )
                            .on_mouse_move(cx.listener(move |this, e, _, cx| follow(this, e, cx)))
                            .on_mouse_up(
                                MouseButton::Left,
                                cx.listener(move |this, _: &MouseUpEvent, _, cx| let_go(this, cx)),
                            )
                            .on_mouse_down(
                                MouseButton::Right,
                                cx.listener(move |this, _, _, cx| close(this, cx)),
                            ),
                    )
                    .child(panel),
            )
            .with_priority(3)
            .into_any_element(),
        )
    }
}

/// Black at `alpha`, as `rgba` takes it.
fn veil_color(alpha: f32) -> u32 {
    (alpha.clamp(0.0, 1.0) * 255.0).round() as u32
}
