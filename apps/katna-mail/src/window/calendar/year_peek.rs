// SPDX-License-Identifier: GPL-3.0-or-later

//! The Year view's day popover: resting the pointer on a day with events
//! or tasks shows them beside it, with a notch pointing at the day, as a
//! short list (an icon saying what each is, its time and title). A row
//! opens its event or task; the day's name opens the day. On a phone,
//! where nothing hovers, a tap on the day shows it instead.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Bounds, ClickEvent, Context, FontWeight, MouseButton,
    Pixels, SharedString, Size, Task, Window, anchored, deferred, div, ease_out_quint, point,
    prelude::*, rgba,
};
use jiff::civil::{Date, Time};
use katna_dav::Occurrence;
use katna_i18n::{format, tr};
use katna_store::calendar::EventKind;
use katna_store::tasks::Task as TaskItem;
use katna_ui::{px, unpx};

use super::super::MailWindow;
use super::super::event_edit::kind_icon;
use super::super::notched::{self, notch};
use super::{CalView, civil, midnight};
use crate::theme::Theme;
use crate::widgets::{icon, raised};

/// How long the pointer rests on a day before its popover opens, so
/// passing over the months opens nothing.
const OPEN_DELAY: Duration = Duration::from_millis(280);
/// How long the popover stays after the pointer leaves the day and the
/// popover, so the way from one to the other doesn't close it.
const LINGER: Duration = Duration::from_millis(220);

const WIDTH: f32 = 288.0;
const RADIUS: f32 = 12.0;
const PAD: f32 = 6.0;
const HEAD: f32 = 34.0;
const ROW: f32 = 32.0;
/// Rows shown before "N more", which opens the day.
const ROWS: usize = 8;

/// Each day's cell and the window's size, as last painted.
pub(super) type Cells = Rc<RefCell<HashMap<Date, (Bounds<Pixels>, Size<Pixels>)>>>;

/// Where the days with something on are, as last painted, and which
/// day's popover is open.
#[derive(Default)]
pub(in crate::window) struct YearPeek {
    /// Each day's cell and the window's size when it was painted.
    pub(super) cells: Cells,
    open: Option<Open>,
    /// A day the pointer rests on, waiting out [`OPEN_DELAY`].
    waiting: Option<(Date, Task<()>)>,
}

struct Open {
    day: Date,
    over_day: bool,
    over_popover: bool,
    /// Opened by a tap: stays until a press outside it.
    pinned: bool,
    closing: Option<Task<()>>,
}

impl Open {
    fn new(day: Date, pinned: bool) -> Self {
        Self {
            day,
            over_day: !pinned,
            over_popover: false,
            pinned,
            closing: None,
        }
    }
}

/// One line of the popover.
enum Item {
    Event(Occurrence),
    Task(TaskItem, bool, Option<u32>),
}

impl MailWindow {
    /// The events and tasks on `day`: events by start (whole days first),
    /// then tasks.
    fn year_day_items(&self, day: Date) -> Vec<Item> {
        let tz = &self.tz;
        let (start, next) = (
            midnight(day, tz),
            midnight(day.tomorrow().unwrap_or(day), tz),
        );
        let mut items: Vec<Item> = self
            .shown_occurrences()
            .into_iter()
            .filter(|o| o.start < next && (o.end > start || o.start == start))
            .map(Item::Event)
            .collect();
        items.extend(
            self.tasks_on(day)
                .into_iter()
                .map(|(task, done, time)| Item::Task(task, done, time)),
        );
        items
    }

    /// The pointer went onto (`hovered`) or off day `day` of the Year view.
    pub(super) fn year_day_hover(&mut self, day: Date, hovered: bool, cx: &mut Context<Self>) {
        let peek = &mut self.calendar.peek;
        if hovered {
            match &mut peek.open {
                Some(open) if open.day == day => {
                    open.over_day = true;
                    open.closing = None;
                }
                // Another day's is open: this one's shows at once.
                Some(_) => {
                    peek.open = Some(Open::new(day, false));
                    cx.notify();
                }
                None => {
                    let wait = cx.spawn(async move |this, cx| {
                        cx.background_executor().timer(OPEN_DELAY).await;
                        this.update(cx, |this, cx| {
                            let peek = &mut this.calendar.peek;
                            if peek.waiting.as_ref().is_some_and(|(d, _)| *d == day) {
                                peek.waiting = None;
                                peek.open = Some(Open::new(day, false));
                                cx.notify();
                            }
                        })
                        .ok();
                    });
                    peek.waiting = Some((day, wait));
                }
            }
            return;
        }
        if peek.waiting.as_ref().is_some_and(|(d, _)| *d == day) {
            peek.waiting = None;
        }
        if let Some(open) = peek.open.as_mut().filter(|o| o.day == day) {
            open.over_day = false;
            self.year_peek_linger(cx);
        }
    }

    /// The pointer went onto or off the popover.
    fn year_popover_hover(&mut self, hovered: bool, cx: &mut Context<Self>) {
        let Some(open) = &mut self.calendar.peek.open else {
            return;
        };
        open.over_popover = hovered;
        if hovered {
            open.closing = None;
        } else {
            self.year_peek_linger(cx);
        }
    }

    /// Closes the popover after [`LINGER`] unless the pointer comes back.
    fn year_peek_linger(&mut self, cx: &mut Context<Self>) {
        let Some(open) = &mut self.calendar.peek.open else {
            return;
        };
        if open.pinned || open.over_day || open.over_popover {
            return;
        }
        let day = open.day;
        open.closing = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(LINGER).await;
            this.update(cx, |this, cx| {
                let peek = &mut this.calendar.peek;
                if peek
                    .open
                    .as_ref()
                    .is_some_and(|o| o.day == day && !o.over_day && !o.over_popover)
                {
                    peek.open = None;
                    cx.notify();
                }
            })
            .ok();
        }));
    }

    /// A tap on day `day` on a phone: its popover, or the day when it has
    /// nothing on. Returns whether the popover opened.
    pub(super) fn year_day_tap(&mut self, day: Date, cx: &mut Context<Self>) -> bool {
        if self.year_day_items(day).is_empty() {
            return false;
        }
        let peek = &mut self.calendar.peek;
        peek.waiting = None;
        peek.open = Some(Open::new(day, true));
        cx.notify();
        true
    }

    /// Closes the popover, if open.
    pub(super) fn close_year_peek(&mut self, cx: &mut Context<Self>) {
        let peek = &mut self.calendar.peek;
        peek.waiting = None;
        if peek.open.take().is_some() {
            cx.notify();
        }
    }

    /// The popover of the day it is open for, over everything.
    pub(super) fn render_year_peek(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if self.calendar.view != CalView::Year {
            return None;
        }
        let day = self.calendar.peek.open.as_ref()?.day;
        let (cell, viewport) = *self.calendar.peek.cells.borrow().get(&day)?;
        let items = self.year_day_items(day);
        if items.is_empty() {
            return None;
        }
        let shown = items
            .len()
            .min(if items.len() > ROWS { ROWS - 1 } else { ROWS });
        let more = items.len() - shown;
        let height = 2.0 * PAD + HEAD + shown as f32 * ROW + if more > 0 { ROW } else { 0.0 };
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let (x, y, side, along) = notched::place(cell, (WIDTH, height), (vw, vh), RADIUS);

        let date = day.to_datetime(Time::midnight());
        let head = div()
            .id("year-peek-day")
            .h(px(HEAD))
            .mx(px(4.0))
            .px(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .rounded(px(6.0))
            .text_size(px(13.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgba(th.text))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .child(tr!(
                "calendar-peek-day",
                weekday = format::weekday(date),
                day = format::day_month(date)
            ))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.close_year_peek(cx);
                this.open_calendar_day(day, Some(CalView::Day), cx);
            }));
        let rows = items
            .iter()
            .take(shown)
            .enumerate()
            .map(|(ix, item)| self.year_peek_row(ix, day, item, th, cx))
            .collect::<Vec<_>>();
        let more_row = (more > 0).then(|| {
            div()
                .id("year-peek-more")
                .h(px(ROW))
                .mx(px(4.0))
                .px(px(8.0))
                .flex()
                .items_center()
                .rounded(px(6.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_dim))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .child(tr!("calendar-more", count = more))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.close_year_peek(cx);
                    this.open_calendar_day(day, Some(CalView::Day), cx);
                }))
        });

        let popover = div()
            .id("year-peek")
            .absolute()
            .left(px(x))
            .top(px(y))
            .w(px(WIDTH))
            .h(px(height))
            .py(px(PAD))
            .flex()
            .flex_col()
            .border_1()
            .border_color(rgba(th.divider))
            .map(|d| raised(d, th, RADIUS, 4.0))
            .occlude()
            .on_hover(
                cx.listener(|this, hovered: &bool, _, cx| this.year_popover_hover(*hovered, cx)),
            )
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_mouse_down_out(cx.listener(|this, _, _, cx| this.close_year_peek(cx)))
            .child(head)
            .children(rows)
            .children(more_row)
            .children(notch(side, along, (WIDTH, height), th))
            .with_animation(
                SharedString::from(format!("year-peek-{day}")),
                Animation::new(Duration::from_millis(140)).with_easing(ease_out_quint()),
                |el, t| el.opacity(t),
            );
        let layer = div().relative().w(px(vw)).h(px(vh)).child(popover);
        Some(
            deferred(anchored().position(point(px(0.0), px(0.0))).child(layer))
                .with_priority(3)
                .into_any_element(),
        )
    }

    /// One line: what it is, its time (none for a whole day) and title.
    fn year_peek_row(
        &self,
        ix: usize,
        day: Date,
        item: &Item,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let tz = &self.tz;
        let (glyph, color, time, title, done) = match item {
            Item::Event(occurrence) => {
                let data = &occurrence.event.data;
                let whole = occurrence.all_day() || occurrence.end - occurrence.start >= 24 * 3600;
                let time = (!whole).then(|| format::time(civil(occurrence.start, tz)));
                let glyph = match data.kind {
                    EventKind::Default => "event",
                    kind => kind_icon(kind).unwrap_or("event"),
                };
                (
                    glyph,
                    self.event_color(occurrence),
                    time,
                    data.title.clone(),
                    false,
                )
            }
            Item::Task(task, done, time) => {
                let time =
                    time.map(|minutes| format::time(day.to_datetime(super::tasks::clock(minutes))));
                let glyph = if *done { "check-circle" } else { "tasks" };
                (glyph, th.accent, time, task.title.clone(), *done)
            }
        };
        let title = if title.is_empty() {
            tr!("calendar-no-title")
        } else {
            title
        };
        let open = match item {
            Item::Event(occurrence) => Ok(occurrence.clone()),
            Item::Task(task, ..) => Err(task.id),
        };
        div()
            .id(("year-peek-row", ix))
            .h(px(ROW))
            .mx(px(4.0))
            .px(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(10.0))
            .rounded(px(6.0))
            .text_size(px(13.0))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .child(div().flex_none().child(icon(glyph, color, 18.0)))
            .when_some(time, |d, time| {
                d.child(
                    div()
                        .flex_none()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim))
                        .child(time),
                )
            })
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .text_color(rgba(if done { th.text_dim } else { th.text }))
                    .when(done, |d| d.line_through())
                    .child(title),
            )
            .on_click(
                cx.listener(move |this, event: &ClickEvent, window: &mut Window, cx| {
                    this.close_year_peek(cx);
                    match &open {
                        Ok(occurrence) => {
                            this.open_calendar_event(occurrence.clone(), event.position(), cx)
                        }
                        Err(task) => this.task_open_details(*task, window, cx),
                    }
                }),
            )
            .into_any_element()
    }
}
