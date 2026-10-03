// SPDX-License-Identifier: GPL-3.0-or-later

//! The day's agenda beside the inbox, as Gmail's side panel has it: a
//! Calendar button on the top bar, beside Settings, opens a
//! card at the right of the mail with one day's events (today first), their times, and Join for a
//! call about to start. Clicking an event opens the Calendar page on its
//! day. Desktop windows only, as the contact panel; the two take turns.

use std::rc::Rc;

use gpui::{AnyElement, Context, FontWeight, SharedString, Task, Window, div, prelude::*, rgba};
use jiff::civil::{Date, Time};
use jiff::{ToSpan, Zoned};
use katna_dav::Occurrence;
use katna_i18n::{format, tr};
use katna_store::calendar::Calendar;
use katna_ui::motion::{self, Spring};
use katna_ui::px;

use super::MailWindow;
use super::apps::App as RailApp;
use super::calendar::{civil, event_color, midnight, read};
use crate::theme::{Theme, fade};
use crate::widgets::{card_outline, filled_button, icon, icon_button_colored, tip};

/// The card's width.
const AGENDA_WIDTH: f32 = 300.0;
/// The top bar's button, as wide as Settings.
pub(super) const AGENDA_BUTTON_WIDTH: f32 = 40.0;
/// The gap between the cards and the agenda card.
const GAP: f32 = 16.0;
/// Join shows this long before a call starts.
const JOIN_EARLY: i64 = 10 * 60;
/// The mail keeps at least this much width beside the card.
const KEEP: f32 = 700.0;

/// A day's calendars and occurrences.
type Day = (Rc<Vec<Calendar>>, Rc<Vec<Occurrence>>);

/// The panel's state.
pub(super) struct AgendaPanel {
    /// 0 = hidden, 1 = shown.
    spring: Spring,
    /// The day shown.
    day: Date,
    /// The day's calendars and occurrences, once read.
    loaded: Option<Day>,
    task: Option<Task<()>>,
}

impl AgendaPanel {
    pub(super) fn new() -> Self {
        Self {
            spring: Spring::new(motion::SLIDE, 0.0),
            day: Zoned::now().date(),
            loaded: None,
            task: None,
        }
    }
}

impl MailWindow {
    /// Whether the button shows: the main window's Mail page on a desktop.
    pub(super) fn agenda_button_shown(&self) -> bool {
        !self.detached
            && self.layout.shape.is_desktop()
            && self.app == RailApp::Mail
            && !self.settings_in_main()
    }

    /// Whether the card fits beside the mail.
    fn agenda_fits(&self, available: f32) -> bool {
        available - AGENDA_WIDTH - GAP >= KEEP
    }

    /// The agenda card is open, so the contact panel waits.
    pub(super) fn agenda_open(&self) -> bool {
        self.config.mail.agenda_panel && self.agenda_button_shown()
    }

    /// Moves the card toward shown or hidden for this frame, and returns
    /// the width the card takes now and when settled.
    pub(super) fn tick_agenda(
        &mut self,
        available: f32,
        window: &Window,
        reduce: bool,
    ) -> (f32, f32) {
        if !self.agenda_button_shown() {
            self.agenda.spring.snap(0.0);
            return (0.0, 0.0);
        }
        let open = self.config.mail.agenda_panel && self.agenda_fits(available);
        self.agenda.spring.set(if open { 1.0 } else { 0.0 });
        let t = self.agenda.spring.tick(window, reduce).clamp(0.0, 1.0);
        let room = |t: f32| (AGENDA_WIDTH + GAP) * t;
        (room(t), room(self.agenda.spring.target()))
    }

    fn toggle_agenda(&mut self, cx: &mut Context<Self>) {
        self.config.mail.agenda_panel = !self.config.mail.agenda_panel;
        self.save_config();
        if self.config.mail.agenda_panel {
            self.agenda.day = Zoned::now().with_time_zone(self.tz.clone()).date();
            self.load_agenda(cx);
        }
        cx.notify();
    }

    /// Reads the shown day's events, when the card is open.
    pub(super) fn load_agenda(&mut self, cx: &mut Context<Self>) {
        if !self.agenda_open() {
            return;
        }
        let tz = self.tz.clone();
        let day = self.agenda.day;
        let (from, to) = (
            midnight(day, &tz),
            midnight(day.tomorrow().unwrap_or(day), &tz),
        );
        let paths = self.paths.clone();
        let birthdays = !self.config.contacts.hide_birthdays;
        self.agenda.task = Some(cx.spawn(async move |this, cx| {
            let read = cx
                .background_executor()
                .spawn(async move { read(&paths, from, to, &tz, birthdays) })
                .await;
            this.update(cx, |this, cx| {
                match read {
                    Ok((calendars, occurrences)) => {
                        this.agenda.loaded = Some((Rc::new(calendars), Rc::new(occurrences)));
                    }
                    Err(err) => tracing::warn!(%err, "reading the agenda failed"),
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn turn_agenda(&mut self, days: i64, cx: &mut Context<Self>) {
        if let Ok(day) = self.agenda.day.checked_add(days.days()) {
            self.agenda.day = day;
            self.load_agenda(cx);
            cx.notify();
        }
    }

    /// The top bar's button, beside Settings, which opens and
    /// closes the card.
    pub(super) fn render_agenda_button(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let on = self.config.mail.agenda_panel;
        icon_button_colored(
            "agenda-toggle",
            "calendar",
            24.0,
            if on { th.accent } else { th.text_dim },
            th,
        )
        .when(on, |d| d.bg(rgba(th.nav_selected)))
        .tooltip(tip(
            if on {
                tr!("agenda-hide")
            } else {
                tr!("agenda-show")
            },
            th,
        ))
        .on_click(cx.listener(|this, _, _, cx| this.toggle_agenda(cx)))
        .into_any_element()
    }

    /// The agenda card, at the right of the cards.
    pub(super) fn render_agenda_panel(
        &mut self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !self.agenda_button_shown() {
            return None;
        }
        if self.agenda_open() && self.agenda.loaded.is_none() && self.agenda.task.is_none() {
            self.load_agenda(cx);
        }
        let t = self.agenda.spring.value().clamp(0.0, 1.0);
        let room = crate::widgets::CARD_SHADOW_ROOM;
        if t <= 0.001 {
            return None;
        }
        Some(
            div()
                .flex_none()
                .w(px((AGENDA_WIDTH + GAP) * t + room))
                // The shadow's room reaches into the window's margin, so
                // the card lines up with the cards' right edge.
                .mt(px(-room))
                .mb(px(-room))
                .mr(px(-room))
                .py(px(room))
                .pl(px(GAP * t))
                .overflow_hidden()
                .child(
                    div()
                        .w(px(AGENDA_WIDTH))
                        .h_full()
                        .ml(px(24.0 * (1.0 - t)))
                        .opacity(t)
                        .child(self.render_agenda_card(th, cx)),
                )
                .into_any_element(),
        )
    }

    fn render_agenda_card(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let tz = self.tz.clone();
        let day = self.agenda.day;
        let now = jiff::Timestamp::now().as_second();
        let today = Zoned::now().with_time_zone(tz.clone()).date();
        let title = if day == today {
            tr!(
                "agenda-today",
                date = format::day_month(day.to_datetime(Time::midnight()))
            )
        } else {
            let date = day.to_datetime(Time::midnight());
            tr!(
                "agenda-day",
                weekday = format::weekday(date),
                date = format::day_month(date)
            )
        };
        let (calendars, occurrences) = match &self.agenda.loaded {
            Some((c, o)) => (c.clone(), o.clone()),
            None => (Rc::new(Vec::new()), Rc::new(Vec::new())),
        };
        let rows = occurrences.iter().map(|occurrence| {
            let data = &occurrence.event.data;
            let color = event_color(&calendars, occurrence);
            let when = if occurrence.all_day() {
                tr!("calendar-all-day")
            } else {
                tr!(
                    "calendar-time-range",
                    start = format::time(civil(occurrence.start, &tz)),
                    end = format::time(civil(occurrence.end, &tz))
                )
            };
            let happening =
                !occurrence.all_day() && occurrence.start <= now && now < occurrence.end;
            let join = (!data.join_url.is_empty()
                && now >= occurrence.start - JOIN_EARLY
                && now < occurrence.end)
                .then(|| data.join_url.clone());
            let past = occurrence.end <= now;
            let title = if data.title.is_empty() {
                tr!("calendar-no-title")
            } else {
                data.title.clone()
            };
            let open_day = civil(occurrence.start, &tz).date();
            div()
                .id(SharedString::from(format!(
                    "agenda-{}-{}",
                    occurrence.event.id, occurrence.start
                )))
                .mx(px(8.0))
                .px(px(12.0))
                .py(px(8.0))
                .flex()
                .flex_row()
                .items_start()
                .gap(px(12.0))
                .rounded(px(8.0))
                .cursor_pointer()
                .when(happening, |d| d.bg(rgba(fade(color, 0.14))))
                .hover(|s| s.bg(rgba(th.hover)))
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.show_page(RailApp::Calendar, window, cx);
                    this.open_calendar_on(open_day, cx);
                }))
                .child(
                    div()
                        .flex_none()
                        .mt(px(2.0))
                        .w(px(4.0))
                        .h(px(34.0))
                        .rounded_full()
                        .bg(rgba(color)),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .truncate()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgba(if past { th.text_dim } else { th.text }))
                                .child(title),
                        )
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgba(th.text_dim))
                                .child(when),
                        ),
                )
                .children(join.map(|url| {
                    filled_button(
                        SharedString::from(format!("agenda-join-{}", occurrence.event.id)),
                        tr!("calendar-join"),
                        th,
                    )
                    .h(px(30.0))
                    .px(px(14.0))
                    .on_click(move |_, _, cx| {
                        cx.stop_propagation();
                        cx.open_url(&url);
                    })
                }))
        });
        let body = if self.agenda.loaded.is_none() {
            empty(&tr!("calendar-loading"), th)
        } else if occurrences.is_empty() {
            empty(&tr!("agenda-empty"), th)
        } else {
            div()
                .id("agenda-list")
                .flex_1()
                .min_h_0()
                .overflow_y_scroll()
                .py(px(4.0))
                .flex()
                .flex_col()
                .children(rows)
                .into_any_element()
        };
        let (radius, outline) = (
            self.layout.shape.card_radius(),
            self.layout.shape.card_outline(),
        );
        let (shadow, edge) = self.card_edges(0.0, outline);
        div()
            .relative()
            .size_full()
            .map(|d| crate::widgets::card(d, th, th.pane(), radius, shadow))
            .flex()
            .flex_col()
            .child(
                div()
                    .flex_none()
                    .h(px(56.0))
                    .pl(px(20.0))
                    .pr(px(4.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .text_size(px(16.0))
                            .text_color(rgba(th.text))
                            .child(title),
                    )
                    .child(
                        icon_button_colored("agenda-back", "chevron-left", 20.0, th.text_dim, th)
                            .tooltip(tip(tr!("calendar-previous-day"), th))
                            .on_click(cx.listener(|this, _, _, cx| this.turn_agenda(-1, cx))),
                    )
                    .child(
                        icon_button_colored("agenda-on", "chevron-right", 20.0, th.text_dim, th)
                            .tooltip(tip(tr!("calendar-next-day"), th))
                            .on_click(cx.listener(|this, _, _, cx| this.turn_agenda(1, cx))),
                    ),
            )
            .child(body)
            .children(card_outline(th, radius, edge))
            .into_any_element()
    }
}

fn empty(text: &str, th: &Theme) -> AnyElement {
    div()
        .flex_1()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(12.0))
        .px(px(24.0))
        .child(icon("calendar", th.text_faint, 40.0))
        .child(
            div()
                .text_center()
                .text_size(px(14.0))
                .text_color(rgba(th.text_faint))
                .child(text.to_owned()),
        )
        .into_any_element()
}
