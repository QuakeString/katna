// SPDX-License-Identifier: GPL-3.0-or-later

//! The person card's summary as a popover: where the window has no room
//! for the panel beside the open mail, a click on a name or picture opens
//! the name, the round buttons and the details over the mail, with a
//! notch pointing at where the click was. Once the window has room again,
//! the panel takes its place: the two are never open together.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    AnimationExt, AnyElement, Bounds, Context, Pixels, Point, Window, anchored, canvas, deferred,
    div, point, prelude::*, size,
};
use katna_ui::{px, unpx};

use super::super::MailWindow;
use super::super::notched::{self, notch};
use super::CONTACT_WIDTH;
use crate::data::EntryKey;
use crate::theme::Theme;

const RADIUS: f32 = notched::RADIUS;
/// The popover's height until it is measured.
const FIRST_HEIGHT: f32 = 240.0;
/// The square around the click the notch points at.
const ORIGIN: f32 = 16.0;

/// An open popover: its conversation, where it was opened and its
/// height as last laid out.
pub(in crate::window) struct ContactPeek {
    key: EntryKey,
    at: Point<Pixels>,
    height: Rc<Cell<f32>>,
}

impl ContactPeek {
    pub(super) fn new(key: EntryKey, at: Point<Pixels>) -> Self {
        Self {
            key,
            at,
            height: Rc::new(Cell::new(FIRST_HEIGHT)),
        }
    }
}

impl MailWindow {
    /// Before a frame: the popover goes with its conversation, and gives
    /// way to the panel once the window has room for it.
    pub(super) fn settle_contact_peek(&mut self) {
        let Some(peek) = &self.contact.peek else {
            return;
        };
        let open = self.reading && self.reader.as_ref().is_some_and(|r| r.key == peek.key);
        if !open || self.layout.shape.is_phone() {
            self.contact.peek = None;
            return;
        }
        if self.contact_offered() {
            self.contact.peek = None;
            if !self.config.mail.contact_panel {
                self.config.mail.contact_panel = true;
                self.contact.nav_hold = false;
                self.save_config();
            }
        }
    }

    /// Closes the popover, if open.
    pub(in crate::window) fn close_contact_peek(&mut self, cx: &mut Context<Self>) {
        if self.contact.peek.take().is_some() {
            cx.notify();
        }
    }

    /// The popover, over everything, its notch on where it was opened.
    pub(in crate::window) fn render_contact_peek(
        &mut self,
        th: &Theme,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let peek = self.contact.peek.as_ref()?;
        let (at, measured) = (peek.at, peek.height.clone());
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let height = measured.get().min(vh - 2.0 * notched::MARGIN);
        let origin = Bounds::new(
            at - point(px(ORIGIN / 2.0), px(ORIGIN / 2.0)),
            size(px(ORIGIN), px(ORIGIN)),
        );
        let (x, y, side, along) = notched::place(origin, (CONTACT_WIDTH, height), (vw, vh), RADIUS);
        let body = self.contact_card_body(true, th, cx).0;
        let measure = canvas(
            move |bounds, window, _| {
                // With the border round it.
                let h = unpx(bounds.size.height) + 2.0;
                if (measured.get() - h).abs() > 0.5 {
                    measured.set(h);
                    window.refresh();
                }
            },
            |_, _, _, _| {},
        )
        .absolute()
        .top_0()
        .left_0()
        .size_full();
        let popover = div()
            .id("contact-peek")
            .absolute()
            .left(px(x))
            .top(px(y))
            .w(px(CONTACT_WIDTH))
            .h(px(height))
            .map(|d| notched::popover(d, th))
            .on_mouse_down_out(cx.listener(|this, _, _, cx| this.close_contact_peek(cx)))
            .child(
                div()
                    .id("contact-peek-scroll")
                    .size_full()
                    .overflow_y_scroll()
                    .child(div().relative().child(body).child(measure)),
            )
            .children(notch(side, along, th))
            .with_animation(
                "contact-peek",
                gpui::Animation::new(Duration::from_millis(160))
                    .with_easing(gpui::ease_out_quint()),
                |el, t| el.opacity(t),
            );
        let layer = div().relative().w(px(vw)).h(px(vh)).child(popover);
        Some(
            deferred(anchored().position(point(px(0.0), px(0.0))).child(layer))
                .with_priority(3)
                .into_any_element(),
        )
    }
}
