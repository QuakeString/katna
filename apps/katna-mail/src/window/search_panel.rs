// SPDX-License-Identifier: GPL-3.0-or-later

//! Search options: the panel the tune button in the search box opens, with
//! fields that build a search query (`from:`, `subject:`, `newer_than:`,
//! ...), as webmail's advanced search does. "Custom" dates pick a day or a
//! range in the past, typed or from a calendar, with optional times; they
//! become `after:`/`before:` in local time.

use gpui::{
    AnyElement, Context, Div, Entity, Focusable, FontWeight, Stateful, Subscription, Window, div,
    prelude::*, px, rgba,
};
use jiff::civil::{Date, Time};
use jiff::tz::TimeZone;
use katna_ui::{InputEvent, TextInput};

use super::MailWindow;
use crate::theme::Theme;
use crate::widgets::{elevation, filled_button, icon, icon_button, icon_button_colored, tip};

/// "Date within" choices: label and `newer_than:` value.
const WITHIN: [(&str, &str); 8] = [
    ("Any time", ""),
    ("1 day", "1d"),
    ("3 days", "3d"),
    ("1 week", "1w"),
    ("2 weeks", "2w"),
    ("1 month", "1m"),
    ("6 months", "6m"),
    ("1 year", "1y"),
];

/// The "Custom" chip, after [`WITHIN`].
const CUSTOM: usize = WITHIN.len();

/// What a custom date filter finds.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Span {
    /// Mail from one day.
    On,
    /// Mail before a day (not that day), or before a time.
    Before,
    /// Mail from a day (that day included), or from a time.
    Since,
    /// Mail from the first day to the last, both included.
    Between,
}

const SPANS: [(Span, &str); 4] = [
    (Span::On, "On"),
    (Span::Before, "Before"),
    (Span::Since, "Since"),
    (Span::Between, "Between"),
];

/// The month the calendar under a date field shows.
struct Calendar {
    /// 0: the first date field, 1: the second.
    field: usize,
    year: i16,
    month: i8,
}

pub(super) struct SearchPanel {
    from: Entity<TextInput>,
    to: Entity<TextInput>,
    subject: Entity<TextInput>,
    words: Entity<TextInput>,
    without: Entity<TextInput>,
    within: usize,
    span: Span,
    /// Custom dates and times: first and (for Between) last.
    dates: [Entity<TextInput>; 2],
    times: [Entity<TextInput>; 2],
    calendar: Option<Calendar>,
    /// Why the custom dates can't be searched.
    error: Option<&'static str>,
    attachment: bool,
    _subscriptions: Vec<Subscription>,
}

impl SearchPanel {
    /// The search query the fields make, or why the custom dates are wrong.
    fn query(&self, cx: &gpui::App) -> Result<String, &'static str> {
        let text = |input: &Entity<TextInput>| input.read(cx).text().trim().to_owned();
        let quote = |value: &str| {
            if value.contains(char::is_whitespace) {
                format!("\"{value}\"")
            } else {
                value.to_owned()
            }
        };
        let dates = match WITHIN.get(self.within) {
            Some((_, "")) => String::new(),
            Some((_, age)) => format!("newer_than:{age}"),
            None => custom_dates(
                self.span,
                [&text(&self.dates[0]), &text(&self.times[0])],
                [&text(&self.dates[1]), &text(&self.times[1])],
                &TimeZone::system(),
            )?,
        };
        Ok(build_query(
            &text(&self.from),
            &text(&self.to),
            &text(&self.subject),
            &text(&self.words),
            &text(&self.without),
            &dates,
            self.attachment,
            quote,
        ))
    }
}

/// `after:`/`before:` for a custom span in `tz`, from typed dates
/// (`2026-09-01`) and optional times (`9:30`, `9:30 pm`, `21:30`). The
/// second date and time are only read for [`Span::Between`].
fn custom_dates(
    span: Span,
    [first_day, first_time]: [&str; 2],
    [last_day, last_time]: [&str; 2],
    tz: &TimeZone,
) -> Result<String, &'static str> {
    let day = |text: &str| match text.trim() {
        "" => Err("Pick a date"),
        text => parse_day(text).ok_or("Use a date like 2026-09-01"),
    };
    let time = |text: &str| match text.trim() {
        "" => Ok(None),
        text => parse_time(text)
            .map(Some)
            .ok_or("Use a time like 9:30, 9:30 pm or 21:30"),
    };
    let at = |day: Date, time: Time| -> Result<String, &'static str> {
        let zoned = day
            .to_datetime(time)
            .to_zoned(tz.clone())
            .map_err(|_| "That date is out of range")?;
        let offset = zoned.offset().seconds();
        let sign = if offset < 0 { '-' } else { '+' };
        let offset = offset.unsigned_abs();
        Ok(format!(
            "{day}T{:02}:{:02}{sign}{:02}:{:02}",
            time.hour(),
            time.minute(),
            offset / 3_600,
            offset / 60 % 60
        ))
    };
    let next = |day: Date| day.tomorrow().map_err(|_| "That date is out of range");
    let first = day(first_day)?;
    // On hides the time fields; ignore what they hold.
    let first_time = match span {
        Span::On => None,
        _ => time(first_time)?,
    };
    Ok(match span {
        Span::On => format!(
            "after:{} before:{}",
            at(first, Time::midnight())?,
            at(next(first)?, Time::midnight())?
        ),
        Span::Before => format!("before:{}", at(first, first_time.unwrap_or_default())?),
        Span::Since => format!("after:{}", at(first, first_time.unwrap_or_default())?),
        Span::Between => {
            let mut from = (first, first_time);
            let mut to = (day(last_day)?, time(last_time)?);
            if (to.0, to.1.unwrap_or(Time::MAX)) < (from.0, from.1.unwrap_or_default()) {
                std::mem::swap(&mut from, &mut to);
            }
            let end = match to.1 {
                // Up to and including that minute.
                Some(time) => match time.checked_add(jiff::SignedDuration::from_mins(1)) {
                    Ok(end) if end > time => at(to.0, end)?,
                    _ => at(next(to.0)?, Time::midnight())?,
                },
                None => at(next(to.0)?, Time::midnight())?,
            };
            format!(
                "after:{} before:{end}",
                at(from.0, from.1.unwrap_or_default())?
            )
        }
    })
}

/// `2026-09-01` or `2026/9/1`.
fn parse_day(text: &str) -> Option<Date> {
    let mut parts = text.split(['-', '/']);
    let year = parts.next()?.trim().parse().ok()?;
    let month = parts.next()?.trim().parse().ok()?;
    let day = parts.next()?.trim().parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Date::new(year, month, day).ok()
}

/// `9:30`, `09:30`, `21:30`, `9pm`, `9:30 PM`, `12 am`.
fn parse_time(text: &str) -> Option<Time> {
    let text = text.trim().to_ascii_lowercase();
    let (clock, pm) = if let Some(clock) = text.strip_suffix("pm") {
        (clock.trim_end(), Some(true))
    } else if let Some(clock) = text.strip_suffix("am") {
        (clock.trim_end(), Some(false))
    } else {
        (text.as_str(), None)
    };
    let (hour, minute) = match clock.split_once([':', '.']) {
        Some((hour, minute)) if minute.len() == 2 => (hour, minute),
        Some(_) => return None,
        None if pm.is_some() => (clock, "0"),
        None => return None,
    };
    if hour.is_empty() || hour.len() > 2 || !hour.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let mut hour: i8 = hour.parse().ok()?;
    let minute: i8 = minute.parse().ok()?;
    match pm {
        Some(_) if !(1..=12).contains(&hour) => return None,
        Some(true) if hour < 12 => hour += 12,
        Some(false) if hour == 12 => hour = 0,
        _ => {}
    }
    Time::new(hour, minute, 0, 0).ok()
}

#[allow(clippy::too_many_arguments)]
fn build_query(
    from: &str,
    to: &str,
    subject: &str,
    words: &str,
    without: &str,
    dates: &str,
    attachment: bool,
    quote: impl Fn(&str) -> String,
) -> String {
    let mut parts = Vec::new();
    if !from.is_empty() {
        parts.push(format!("from:{}", quote(from)));
    }
    if !to.is_empty() {
        parts.push(format!("to:{}", quote(to)));
    }
    if !subject.is_empty() {
        parts.push(format!("subject:{}", quote(subject)));
    }
    if !words.is_empty() {
        parts.push(words.to_owned());
    }
    parts.extend(without.split_whitespace().map(|w| format!("-{w}")));
    if !dates.is_empty() {
        parts.push(dates.to_owned());
    }
    if attachment {
        parts.push("has:attachment".to_owned());
    }
    parts.join(" ")
}

impl MailWindow {
    pub(super) fn toggle_search_panel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.search_panel.take().is_some() {
            cx.notify();
            return;
        }
        self.settings_open = false;
        let input = |cx: &mut Context<Self>| cx.new(|cx| TextInput::new("", cx));
        let (from, to, subject, words, without) =
            (input(cx), input(cx), input(cx), input(cx), input(cx));
        let date = |cx: &mut Context<Self>| cx.new(|cx| TextInput::new("YYYY-MM-DD", cx));
        let time = |cx: &mut Context<Self>| cx.new(|cx| TextInput::new("Time", cx));
        let dates = [date(cx), date(cx)];
        let times = [time(cx), time(cx)];
        let subscriptions =
            [&from, &to, &subject, &words, &without]
                .into_iter()
                .chain(&dates)
                .chain(&times)
                .map(|input| {
                    cx.subscribe_in(input, window, |this, _, event: &InputEvent, window, cx| {
                        match event {
                            InputEvent::Submit => this.run_search_panel(window, cx),
                            InputEvent::Cancel => {
                                this.search_panel = None;
                                cx.notify();
                            }
                            InputEvent::Changed => {}
                        }
                    })
                })
                .collect();
        window.focus(&from.focus_handle(cx), cx);
        self.search_panel = Some(SearchPanel {
            from,
            to,
            subject,
            words,
            without,
            within: 0,
            span: Span::On,
            dates,
            times,
            calendar: None,
            error: None,
            attachment: false,
            _subscriptions: subscriptions,
        });
        cx.notify();
    }

    fn run_search_panel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(panel) = &mut self.search_panel else {
            return;
        };
        let query = match panel.query(cx) {
            Ok(query) => query,
            Err(error) => {
                panel.error = Some(error);
                cx.notify();
                return;
            }
        };
        self.search_panel = None;
        if query.is_empty() {
            cx.notify();
            return;
        }
        self.search_for(query, window, cx);
        self.focus_list(&super::FocusList, window, cx);
    }

    pub(super) fn render_search_panel(
        &mut self,
        th: &Theme,
        viewport: f32,
        width: f32,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let t = self.search_panel_spring.value().clamp(0.0, 1.0);
        let panel = self.search_panel.as_ref()?;
        let field = |label: &'static str, input: &Entity<TextInput>| {
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .child(
                    div()
                        .w(px(120.0))
                        .flex_none()
                        .text_size(px(14.0))
                        .text_color(rgba(th.text_dim))
                        .child(label),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .h(px(36.0))
                        .flex()
                        .items_center()
                        .border_b_1()
                        .border_color(rgba(th.divider))
                        .text_size(px(14.0))
                        .child(input.clone()),
                )
        };
        let within = panel.within;
        let labels = WITHIN.iter().map(|(label, _)| *label).chain(["Custom"]);
        let chips = labels.enumerate().map(|(ix, label)| {
            chip(("within", ix), label, ix == within, th).on_click(cx.listener(
                move |this, _, _, cx| {
                    let Some(panel) = &mut this.search_panel else {
                        return;
                    };
                    panel.within = ix;
                    panel.error = None;
                    // Custom with no date yet: show the calendar.
                    if ix == CUSTOM
                        && panel.calendar.is_none()
                        && panel.dates[0].read(cx).text().trim().is_empty()
                    {
                        this.toggle_calendar(0, cx);
                    }
                    cx.notify();
                },
            ))
        });
        let chips: Vec<_> = chips.collect();
        let custom = (within == CUSTOM).then(|| self.render_custom_dates(th, cx));
        let attachment = panel.attachment;
        let body = div()
            .id("search-panel")
            .occlude()
            .w(px(width))
            .p(px(24.0))
            .pt(px(12.0))
            .flex()
            .flex_col()
            .gap(px(8.0))
            .rounded(px(12.0))
            .bg(rgba(th.menu))
            .shadow(elevation(th, 3.0))
            .text_color(rgba(th.text))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::MEDIUM)
                            .child("Search options"),
                    )
                    .child(
                        icon_button("search-panel-close", "close", 20.0, th)
                            .tooltip(tip("Close", th))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.search_panel = None;
                                cx.notify();
                            })),
                    ),
            )
            .child(field("From", &panel.from))
            .child(field("To", &panel.to))
            .child(field("Subject", &panel.subject))
            .child(field("Has the words", &panel.words))
            .child(field("Doesn't have", &panel.without))
            .child(
                div()
                    .pt(px(8.0))
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap(px(16.0))
                    .child(
                        div()
                            .w(px(120.0))
                            .pt(px(4.0))
                            .flex_none()
                            .text_size(px(14.0))
                            .text_color(rgba(th.text_dim))
                            .child("Date within"),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .flex_wrap()
                                    .gap(px(6.0))
                                    .children(chips),
                            )
                            .children(custom),
                    ),
            )
            .child(
                div()
                    .id("has-attachment")
                    .pt(px(8.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .cursor_pointer()
                    .text_size(px(14.0))
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(panel) = &mut this.search_panel {
                            panel.attachment = !panel.attachment;
                        }
                        cx.notify();
                    }))
                    .child(if attachment {
                        icon("checkbox-checked", th.accent, 20.0)
                    } else {
                        icon("checkbox", th.text_dim, 20.0)
                    })
                    .child("Has attachment"),
            )
            .child(
                div()
                    .pt(px(16.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_end()
                    .gap(px(16.0))
                    .child(
                        div()
                            .id("search-panel-clear")
                            .px(px(12.0))
                            .h(px(36.0))
                            .flex()
                            .items_center()
                            .rounded_full()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.accent))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.search_panel = None;
                                this.toggle_search_panel(window, cx);
                            }))
                            .child("Clear filter"),
                    )
                    .child(filled_button("search-panel-go", "Search", th).on_click(
                        cx.listener(|this, _, window, cx| this.run_search_panel(window, cx)),
                    )),
            );
        Some(
            div()
                .absolute()
                .top(px(-4.0 + 8.0 * (1.0 - t)))
                .left(px(((viewport - width) / 2.0).max(0.0)))
                .opacity(t)
                .child(body)
                .into_any_element(),
        )
    }
}

/// A chip in a row of choices.
fn chip(id: impl Into<gpui::ElementId>, label: &str, on: bool, th: &Theme) -> Stateful<Div> {
    div()
        .id(id)
        .px(px(10.0))
        .h(px(28.0))
        .flex()
        .items_center()
        .rounded(px(8.0))
        .border_1()
        .border_color(rgba(if on { th.nav_selected } else { th.divider }))
        .bg(rgba(if on { th.nav_selected } else { th.surface }))
        .text_color(rgba(if on {
            th.nav_selected_text
        } else {
            th.text_dim
        }))
        .text_size(px(13.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .child(label.to_owned())
}

const WEEKDAYS: [&str; 7] = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

impl MailWindow {
    /// Under the "Custom" chip: On/Before/Since/Between, the date and time
    /// fields, the calendar and any error.
    fn render_custom_dates(&self, th: &Theme, cx: &mut Context<Self>) -> Div {
        let Some(panel) = &self.search_panel else {
            return div();
        };
        let span = panel.span;
        let spans = SPANS.iter().map(|&(value, label)| {
            chip(("span", value as usize), label, value == span, th).on_click(cx.listener(
                move |this, _, _, cx| {
                    if let Some(panel) = &mut this.search_panel {
                        panel.span = value;
                        panel.error = None;
                        if value != Span::Between
                            && panel.calendar.as_ref().is_some_and(|c| c.field == 1)
                        {
                            panel.calendar = None;
                        }
                    }
                    cx.notify();
                },
            ))
        });
        let spans: Vec<_> = spans.collect();
        let boxed = |input: &Entity<TextInput>, width: f32| {
            div()
                .w(px(width))
                .h(px(32.0))
                .px(px(8.0))
                .flex()
                .flex_row()
                .items_center()
                .rounded(px(8.0))
                .border_1()
                .border_color(rgba(th.divider))
                .text_size(px(14.0))
                .child(div().flex_1().min_w_0().child(input.clone()))
        };
        let field = |ix: usize, cx: &mut Context<Self>| {
            let open = panel.calendar.as_ref().is_some_and(|c| c.field == ix);
            let date = boxed(&panel.dates[ix], 150.0).child(
                icon_button_colored(
                    ("pick-date", ix),
                    "calendar",
                    18.0,
                    if open { th.accent } else { th.text_dim },
                    th,
                )
                .tooltip(tip("Pick from a calendar", th))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.toggle_calendar(ix, cx);
                })),
            );
            let row = div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .child(date);
            if span == Span::On {
                row
            } else {
                row.child(boxed(&panel.times[ix], 90.0))
            }
        };
        let mut fields = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(8.0))
            .child(field(0, cx));
        if span == Span::Between {
            fields = fields
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(rgba(th.text_dim))
                        .child("and"),
                )
                .child(field(1, cx));
        }
        let calendar = panel
            .calendar
            .as_ref()
            .map(|calendar| self.render_calendar(calendar, th, cx));
        let error = panel.error.map(|error| {
            div()
                .text_size(px(13.0))
                .text_color(rgba(th.error))
                .child(error)
        });
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(6.0))
                    .children(spans),
            )
            .child(fields)
            .children(calendar)
            .children(error)
    }

    fn toggle_calendar(&mut self, field: usize, cx: &mut Context<Self>) {
        let Some(panel) = &mut self.search_panel else {
            return;
        };
        if panel.calendar.as_ref().is_some_and(|c| c.field == field) {
            panel.calendar = None;
        } else {
            let shown = parse_day(panel.dates[field].read(cx).text())
                .unwrap_or_else(|| jiff::Zoned::now().date());
            panel.calendar = Some(Calendar {
                field,
                year: shown.year(),
                month: shown.month(),
            });
        }
        cx.notify();
    }

    /// Moves the calendar by `months`.
    fn turn_calendar(&mut self, months: i32, cx: &mut Context<Self>) {
        if let Some(calendar) = self.search_panel.as_mut().and_then(|p| p.calendar.as_mut()) {
            let index = i32::from(calendar.year) * 12 + i32::from(calendar.month) - 1 + months;
            let year = index.div_euclid(12);
            if (1..=9999).contains(&year) {
                calendar.year = year as i16;
                calendar.month = (index.rem_euclid(12) + 1) as i8;
            }
        }
        cx.notify();
    }

    fn pick_date(&mut self, day: Date, cx: &mut Context<Self>) {
        let Some(panel) = &mut self.search_panel else {
            return;
        };
        let Some(calendar) = panel.calendar.take() else {
            return;
        };
        panel.error = None;
        panel.dates[calendar.field].update(cx, |input, cx| input.set_text(day.to_string(), cx));
        cx.notify();
    }

    /// A month of days, Monday first, with the month and year to turn.
    fn render_calendar(&self, calendar: &Calendar, th: &Theme, cx: &mut Context<Self>) -> Div {
        let Ok(first) = Date::new(calendar.year, calendar.month, 1) else {
            return div();
        };
        let today = jiff::Zoned::now().date();
        let picked = self
            .search_panel
            .as_ref()
            .and_then(|p| parse_day(p.dates[calendar.field].read(cx).text()));
        let turn = |id: &'static str, name: &'static str, months: i32, label: &'static str| {
            icon_button(id, name, 18.0, th)
                .tooltip(tip(label, th))
                .on_click(cx.listener(move |this, _, _, cx| this.turn_calendar(months, cx)))
        };
        let header = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .text_size(px(14.0))
            .font_weight(FontWeight::MEDIUM)
            .child(turn("month-back", "chevron-left", -1, "Previous month"))
            .child(
                div()
                    .w(px(92.0))
                    .flex()
                    .justify_center()
                    .child(MONTHS[calendar.month as usize - 1]),
            )
            .child(turn("month-on", "chevron-right", 1, "Next month"))
            .child(div().w(px(12.0)))
            .child(turn("year-back", "chevron-left", -12, "Previous year"))
            .child(
                div()
                    .w(px(44.0))
                    .flex()
                    .justify_center()
                    .child(calendar.year.to_string()),
            )
            .child(turn("year-on", "chevron-right", 12, "Next year"));
        let cell = || {
            div()
                .w(px(34.0))
                .h(px(30.0))
                .flex()
                .items_center()
                .justify_center()
        };
        let weekdays = div().flex().flex_row().children(WEEKDAYS.iter().map(|day| {
            cell()
                .text_size(px(12.0))
                .text_color(rgba(th.text_faint))
                .child(*day)
        }));
        let lead = first.weekday().to_monday_zero_offset();
        let days = first.days_in_month();
        let mut weeks = Vec::new();
        let mut week = Vec::new();
        for _ in 0..lead {
            week.push(cell().into_any_element());
        }
        for number in 1..=days {
            let Ok(day) = Date::new(calendar.year, calendar.month, number) else {
                continue;
            };
            let on = picked == Some(day);
            let is_today = day == today;
            let future = day > today;
            let mut element = cell()
                .id(("day", number as usize))
                .rounded_full()
                .text_size(px(13.0))
                .cursor_pointer()
                .text_color(rgba(if on {
                    th.on_accent
                } else if future {
                    th.text_faint
                } else {
                    th.text
                }))
                .hover(|s| s.bg(rgba(th.hover)))
                .on_click(cx.listener(move |this, _, _, cx| this.pick_date(day, cx)))
                .child(number.to_string());
            if on {
                element = element.bg(rgba(th.accent)).hover(|s| s.bg(rgba(th.accent)));
            } else if is_today {
                element = element.border_1().border_color(rgba(th.accent));
            }
            week.push(element.into_any_element());
            if week.len() == 7 {
                weeks.push(div().flex().flex_row().children(std::mem::take(&mut week)));
            }
        }
        if !week.is_empty() {
            weeks.push(div().flex().flex_row().children(week));
        }
        div()
            .w(px(7.0 * 34.0 + 24.0))
            .p(px(12.0))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .rounded(px(12.0))
            .border_1()
            .border_color(rgba(th.divider))
            .bg(rgba(th.surface))
            .child(header)
            .child(weekdays)
            .children(weeks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_queries() {
        let quote = |v: &str| {
            if v.contains(' ') {
                format!("\"{v}\"")
            } else {
                v.to_owned()
            }
        };
        assert_eq!(
            build_query(
                "kay",
                "",
                "gas deal",
                "price",
                "draft old",
                "newer_than:1w",
                true,
                quote
            ),
            "from:kay subject:\"gas deal\" price -draft -old newer_than:1w has:attachment"
        );
        assert_eq!(build_query("", "", "", "", "", "", false, quote), "");
    }

    #[test]
    fn reads_days_and_times() {
        assert_eq!(parse_day("2026-09-01"), Some(jiff::civil::date(2026, 9, 1)));
        assert_eq!(parse_day("2001/5/14"), Some(jiff::civil::date(2001, 5, 14)));
        assert_eq!(parse_day("2026-02-30"), None);
        assert_eq!(parse_day("1 Sep 2026"), None);
        let t = |h, m| Some(Time::new(h, m, 0, 0).unwrap());
        assert_eq!(parse_time("9:30"), t(9, 30));
        assert_eq!(parse_time("21:05"), t(21, 5));
        assert_eq!(parse_time("9:30 pm"), t(21, 30));
        assert_eq!(parse_time("9PM"), t(21, 0));
        assert_eq!(parse_time("12 am"), t(0, 0));
        assert_eq!(parse_time("12:15pm"), t(12, 15));
        assert_eq!(parse_time("9.45"), t(9, 45));
        for bad in [
            "", "9", "24:00", "9:5", "13 pm", "0 am", "nine", "-1:00", "9:30:00",
        ] {
            assert_eq!(parse_time(bad), None, "{bad}");
        }
    }

    #[test]
    fn custom_dates_are_local() {
        let dhaka = TimeZone::fixed(jiff::tz::offset(6));
        let dates =
            |span, first: [&str; 2], last: [&str; 2]| custom_dates(span, first, last, &dhaka);
        let none = ["", ""];
        assert_eq!(
            dates(Span::On, ["2026-09-01", "10:00"], none).unwrap(),
            "after:2026-09-01T00:00+06:00 before:2026-09-02T00:00+06:00"
        );
        assert_eq!(
            dates(Span::Before, ["2026-09-01", ""], none).unwrap(),
            "before:2026-09-01T00:00+06:00"
        );
        assert_eq!(
            dates(Span::Since, ["2026-09-01", "9:30 pm"], none).unwrap(),
            "after:2026-09-01T21:30+06:00"
        );
        assert_eq!(
            dates(Span::Between, ["2026-09-01", ""], ["2026-09-10", ""]).unwrap(),
            "after:2026-09-01T00:00+06:00 before:2026-09-11T00:00+06:00"
        );
        // Given backwards, with times; the last minute is included.
        assert_eq!(
            dates(
                Span::Between,
                ["2026-09-10", "17:00"],
                ["2026-09-01", "8:00"]
            )
            .unwrap(),
            "after:2026-09-01T08:00+06:00 before:2026-09-10T17:01+06:00"
        );
        assert_eq!(
            dates(Span::Between, ["2026-09-01", ""], ["2026-09-01", "23:59"]).unwrap(),
            "after:2026-09-01T00:00+06:00 before:2026-09-02T00:00+06:00"
        );
        assert_eq!(dates(Span::On, none, none), Err("Pick a date"));
        assert_eq!(
            dates(Span::Between, ["2026-09-01", ""], none),
            Err("Pick a date")
        );
        assert!(dates(Span::Since, ["2026-13-01", ""], none).is_err());
        assert!(dates(Span::Since, ["2026-09-01", "25:00"], none).is_err());
        assert!(dates(Span::On, ["2026-09-01", "25:00"], none).is_ok());

        // The search parser reads them as the right instants.
        let query = dates(Span::On, ["2001-05-14", ""], none).unwrap();
        let start = 989_798_400 - 6 * 3_600;
        assert_eq!(
            katna_search::Query::parse_at(&query, 0).unwrap(),
            katna_search::Query::And(vec![
                katna_search::Query::Filter(katna_search::Filter::After(start)),
                katna_search::Query::Filter(katna_search::Filter::Before(start + 86_400)),
            ])
        );
    }
}
