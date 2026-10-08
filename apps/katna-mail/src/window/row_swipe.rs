// SPDX-License-Identifier: GPL-3.0-or-later

//! Swipe left on a line to snooze it, on a phone (as Gmail's app does): a
//! sideways swipe pulls the line left over a Snooze strip; let go far
//! enough and the snooze menu opens for it, else the line springs back.

use std::time::Duration;

use gpui::{
    AnyElement, Context, FontWeight, Pixels, Point, ScrollWheelEvent, Task, TouchPhase, div,
    prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::tokens::{space, text};
use katna_ui::{px, unpx};

use super::MailWindow;
use crate::theme::Theme;
use crate::widgets::icon;

/// How far a line is pulled before letting go snoozes it.
const ENOUGH: f32 = 96.0;
/// How far it can be pulled at most.
const MOST: f32 = 160.0;
/// A swipe with no movement for this long has ended, for wheels that do
/// not say so.
const QUIET: Duration = Duration::from_millis(180);

/// The line being swiped.
#[derive(Default)]
pub(super) struct RowSwipe {
    /// Its index and how far it is pulled left.
    pulled: Option<(usize, f32)>,
    /// Ends it once the swipe goes quiet.
    end: Option<Task<()>>,
    /// Where it was last pulled, where the menu opens.
    at: Option<Point<Pixels>>,
}

impl MailWindow {
    /// A wheel or touchpad event over line `ix`: a sideways one pulls it.
    pub(super) fn swipe_row(
        &mut self,
        ix: usize,
        event: &ScrollWheelEvent,
        cx: &mut Context<Self>,
    ) {
        if !self.layout.shape.is_phone() {
            return;
        }
        let delta = event.delta.pixel_delta(px(20.0));
        let (dx, dy) = (unpx(delta.x), unpx(delta.y));
        let pulled = match self.row_swipe.pulled {
            Some((at, pulled)) if at == ix => pulled,
            _ => 0.0,
        };
        // Up and down scrolls the list; only sideways pulls.
        if pulled == 0.0 && dx.abs() <= dy.abs() {
            return;
        }
        cx.stop_propagation();
        let pulled = (pulled - dx).clamp(0.0, MOST);
        self.row_swipe.pulled = Some((ix, pulled));
        self.row_swipe.at = Some(event.position);
        if matches!(event.touch_phase, TouchPhase::Ended | TouchPhase::Cancelled) {
            self.end_swipe(event.touch_phase == TouchPhase::Ended, cx);
            return;
        }
        self.row_swipe.end = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(QUIET).await;
            this.update(cx, |this, cx| this.end_swipe(true, cx)).ok();
        }));
        cx.notify();
    }

    /// The swipe let go: the snooze menu when pulled far enough.
    fn end_swipe(&mut self, commit: bool, cx: &mut Context<Self>) {
        self.row_swipe.end = None;
        let Some((ix, pulled)) = self.row_swipe.pulled.take() else {
            return;
        };
        let at = self.row_swipe.at.take();
        if commit
            && pulled >= ENOUGH
            && let (Some(entry), Some(at)) = (self.entries.get(ix), at)
        {
            self.open_snooze_menu(vec![entry.key], at, cx);
        }
        cx.notify();
    }

    /// Line `ix`, `row`, as the swipe has pulled it, over the Snooze strip.
    pub(super) fn swiped_row(&self, ix: usize, row: AnyElement, th: &Theme) -> AnyElement {
        let pulled = match self.row_swipe.pulled {
            Some((at, pulled)) if at == ix && pulled > 0.0 => pulled,
            _ => return row,
        };
        let ready = pulled >= ENOUGH;
        let (fill, ink) = if ready {
            (th.accent, th.on_accent)
        } else {
            (crate::widgets::tonal_fill(th).0, th.accent)
        };
        div()
            .relative()
            .overflow_hidden()
            .child(
                div()
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .right_0()
                    .w(px(pulled))
                    .bg(rgba(fill))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_end()
                    .gap(px(space::S2))
                    .pr(px(space::S5))
                    .text_size(px(text::SMALL))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(ink))
                    .child(icon("schedule", ink, 20.0))
                    .when(pulled > 72.0, |d| d.child(tr!("menu-snooze"))),
            )
            .child(div().relative().left(px(-pulled)).child(row))
            .into_any_element()
    }
}
