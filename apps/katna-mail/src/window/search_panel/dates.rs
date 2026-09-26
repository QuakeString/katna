// SPDX-License-Identifier: GPL-3.0-or-later

//! The "Custom" choice of "Date within": a popover from its chip, with a
//! notch pointing at it, to pick mail on a day, before or since one, or
//! between two, typed or from a calendar. The dates become `after:`/
//! `before:` at local midnight.

use std::cell::Cell;
use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;
use std::rc::Rc;

use gpui::{
    AnyElement, App, Bounds, Context, Div, Entity, Focusable, FontWeight, MouseButton, Pixels,
    Transformation, Window, anchored, deferred, div, point, prelude::*, px, radians, rgba, svg,
};
use jiff::civil::{Date, Time};
use jiff::tz::TimeZone;
use katna_ui::TextInput;

use super::{MailWindow, chip};
use crate::theme::Theme;
use crate::widgets::{filled_button, icon_button, raised, tip};

/// What a custom date filter finds.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Span {
    /// Mail from one day.
    On,
    /// Mail before a day (not that day).
    Before,
    /// Mail from a day (that day included).
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

/// `after:`/`before:` for a custom span in `tz`, from typed dates
/// (`2026-09-01`), each meaning that whole local day. The last date is
/// only read for [`Span::Between`].
pub(super) fn custom_dates(
    span: Span,
    first: &str,
    last: &str,
    tz: &TimeZone,
) -> Result<String, &'static str> {
    let day = |text: &str| match text.trim() {
        "" => Err("Pick a date"),
        text => parse_day(text).ok_or("Use a date like 2026-09-01"),
    };
    // Local midnight, with its offset.
    let at = |day: Date| -> Result<String, &'static str> {
        let zoned = day
            .to_datetime(Time::midnight())
            .to_zoned(tz.clone())
            .map_err(|_| "That date is out of range")?;
        let offset = zoned.offset().seconds();
        let sign = if offset < 0 { '-' } else { '+' };
        let offset = offset.unsigned_abs();
        Ok(format!(
            "{day}T00:00{sign}{:02}:{:02}",
            offset / 3_600,
            offset / 60 % 60
        ))
    };
    let next = |day: Date| day.tomorrow().map_err(|_| "That date is out of range");
    let first = day(first)?;
    Ok(match span {
        Span::On => format!("after:{} before:{}", at(first)?, at(next(first)?)?),
        Span::Before => format!("before:{}", at(first)?),
        Span::Since => format!("after:{}", at(first)?),
        Span::Between => {
            let last = day(last)?;
            let (from, to) = if last < first {
                (last, first)
            } else {
                (first, last)
            };
            format!("after:{} before:{}", at(from)?, at(next(to)?)?)
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

/// The custom dates and the popover that edits them.
pub(super) struct CustomDates {
    pub span: Span,
    /// First and (for Between) last date, as typed.
    pub dates: [Entity<TextInput>; 2],
    /// The date the calendar fills: 0 or 1.
    field: usize,
    /// The month the calendar shows.
    year: i16,
    month: i8,
    pub open: bool,
    /// Why the dates can't be searched.
    pub error: Option<&'static str>,
    /// The "Date within" choice before Custom, back on Cancel if the
    /// dates are incomplete.
    pub before: usize,
    /// Where the Custom chip is, for placing the popover beside it.
    pub chip: Rc<Cell<Option<Bounds<Pixels>>>>,
}

impl CustomDates {
    pub fn new(cx: &mut Context<MailWindow>) -> Self {
        let date = |cx: &mut Context<MailWindow>| cx.new(|cx| TextInput::new("YYYY-MM-DD", cx));
        let today = jiff::Zoned::now().date();
        Self {
            span: Span::On,
            dates: [date(cx), date(cx)],
            field: 0,
            year: today.year(),
            month: today.month(),
            open: false,
            error: None,
            before: 0,
            chip: Rc::default(),
        }
    }

    /// The typed first and last date.
    fn texts(&self, cx: &App) -> [String; 2] {
        [0, 1].map(|ix| self.dates[ix].read(cx).text().trim().to_owned())
    }

    /// `after:`/`before:` for the dates, in the local time zone.
    pub fn query(&self, cx: &App) -> Result<String, &'static str> {
        let [first, last] = self.texts(cx);
        custom_dates(self.span, &first, &last, &TimeZone::system())
    }

    /// What the Custom chip says once the dates are good: "May 14, 2002",
    /// "Since May 14, 2002", "May 1 – May 31, 2002".
    pub fn label(&self, cx: &App) -> Option<String> {
        self.query(cx).ok()?;
        let [first, last] = self.texts(cx);
        custom_label(self.span, &first, &last)
    }
}

/// The chip text for good custom dates (see [`CustomDates::label`]).
fn custom_label(span: Span, first: &str, last: &str) -> Option<String> {
    let long = |d: Date| d.strftime("%b %-d, %Y").to_string();
    let first = parse_day(first)?;
    Some(match span {
        Span::On => long(first),
        Span::Before => format!("Before {}", long(first)),
        Span::Since => format!("Since {}", long(first)),
        Span::Between => {
            let last = parse_day(last)?;
            let (a, b) = if last < first {
                (last, first)
            } else {
                (first, last)
            };
            if a.year() == b.year() {
                format!("{} – {}", a.strftime("%b %-d"), long(b))
            } else {
                format!("{} – {}", long(a), long(b))
            }
        }
    })
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

/// Popover sizes, in px. The days make the width; every row has a set
/// height so the popover's height is known before it is laid out.
const PAD: f32 = 16.0;
const GAP: f32 = 12.0;
const DAY: f32 = 42.0;
const ROW: f32 = 34.0;
const CHIPS: f32 = 28.0;
const FIELD: f32 = 32.0;
const CAPTION: f32 = 16.0;
const HEADER: f32 = 32.0;
const WEEKDAY_ROW: f32 = 24.0;
const ERROR: f32 = 18.0;
const BUTTONS: f32 = 36.0;
const WIDTH: f32 = 7.0 * DAY + 2.0 * PAD;
const RADIUS: f32 = 15.0;
/// The notch's length out of the popover, and the popover's distance from
/// the chip and from the window's edges.
const NOTCH: f32 = 10.0;
const SPACE: f32 = 2.0;
const MARGIN: f32 = 8.0;

/// Where the popover sits against the chip.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Side {
    Below,
    Above,
    Right,
    Left,
}

/// The popover's top left corner, its side of the chip and where the notch
/// meets its edge (from its left for Below/Above, from its top otherwise).
/// Below the chip if it fits, else above, right, left; else below, kept
/// inside the window.
fn place(chip: Bounds<Pixels>, size: (f32, f32), viewport: (f32, f32)) -> (f32, f32, Side, f32) {
    let (w, h) = size;
    let (vw, vh) = viewport;
    let (left, top) = (f32::from(chip.origin.x), f32::from(chip.origin.y));
    let (right, bottom) = (
        left + f32::from(chip.size.width),
        top + f32::from(chip.size.height),
    );
    let (cx, cy) = ((left + right) / 2.0, (top + bottom) / 2.0);
    let away = NOTCH + SPACE;
    let clamp = |v: f32, lo: f32, hi: f32| v.min(hi).max(lo);
    let x_centered = clamp(cx - w / 2.0, MARGIN, vw - w - MARGIN);
    let y_centered = clamp(cy - h / 2.0, MARGIN, vh - h - MARGIN);
    let along_x = |x: f32| clamp(cx - x, RADIUS + NOTCH, w - RADIUS - NOTCH);
    let along_y = |y: f32| clamp(cy - y, RADIUS + NOTCH, h - RADIUS - NOTCH);
    let below = bottom + away;
    if below + h <= vh - MARGIN {
        return (x_centered, below, Side::Below, along_x(x_centered));
    }
    let above = top - away - h;
    if above >= MARGIN {
        return (x_centered, above, Side::Above, along_x(x_centered));
    }
    let beside = right + away;
    if beside + w <= vw - MARGIN {
        return (beside, y_centered, Side::Right, along_y(y_centered));
    }
    let before = left - away - w;
    if before >= MARGIN {
        return (before, y_centered, Side::Left, along_y(y_centered));
    }
    let y = clamp(below, MARGIN, vh - h - MARGIN);
    (x_centered, y, Side::Below, along_x(x_centered))
}

/// Weeks the calendar shows for a month, Monday first.
fn weeks(first: Date) -> usize {
    let lead = first.weekday().to_monday_zero_offset() as usize;
    (lead + first.days_in_month() as usize).div_ceil(7)
}

impl MailWindow {
    /// Shows the popover, keeping the choice to go back to on Cancel.
    pub(super) fn open_custom_dates(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(panel) = &mut self.search_panel else {
            return;
        };
        if panel.within != super::CUSTOM {
            panel.custom.before = panel.within;
        }
        panel.within = super::CUSTOM;
        let custom = &mut panel.custom;
        custom.open = true;
        custom.error = None;
        custom.field = 0;
        show_month(custom, cx);
        window.focus(&custom.dates[0].focus_handle(cx), cx);
        cx.notify();
    }

    /// Done: closes the popover if the dates are good.
    pub(super) fn custom_done(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(panel) = &mut self.search_panel else {
            return;
        };
        match panel.custom.query(cx) {
            Ok(_) => {
                panel.custom.open = false;
                panel.custom.error = None;
                window.focus(&panel.from.focus_handle(cx), cx);
            }
            Err(error) => panel.custom.error = Some(error),
        }
        cx.notify();
    }

    /// Cancel, Escape or a click outside: closes the popover, going back to
    /// the earlier choice if the dates aren't complete.
    pub(super) fn custom_cancel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(panel) = &mut self.search_panel else {
            return;
        };
        if panel.custom.query(cx).is_err() {
            panel.within = panel.custom.before;
        }
        panel.custom.open = false;
        panel.custom.error = None;
        window.focus(&panel.from.focus_handle(cx), cx);
        cx.notify();
    }

    fn custom_mut(&mut self) -> Option<&mut CustomDates> {
        self.search_panel.as_mut().map(|panel| &mut panel.custom)
    }

    /// Makes date `field` the one the calendar fills.
    fn pick_field(&mut self, field: usize, cx: &mut Context<Self>) {
        if let Some(custom) = self.custom_mut()
            && custom.field != field
        {
            custom.field = field;
            show_month(custom, cx);
            cx.notify();
        }
    }

    /// Moves the calendar by `months`.
    fn turn_calendar(&mut self, months: i32, cx: &mut Context<Self>) {
        if let Some(custom) = self.custom_mut() {
            let index = i32::from(custom.year) * 12 + i32::from(custom.month) - 1 + months;
            let year = index.div_euclid(12);
            if (1..=9999).contains(&year) {
                custom.year = year as i16;
                custom.month = (index.rem_euclid(12) + 1) as i8;
            }
        }
        cx.notify();
    }

    /// Fills the calendar's date; in Between, the last date is next.
    fn pick_day(&mut self, day: Date, window: &mut Window, cx: &mut Context<Self>) {
        let Some(custom) = self.custom_mut() else {
            return;
        };
        custom.error = None;
        let field = custom.field;
        custom.dates[field].update(cx, |input, cx| input.set_text(day.to_string(), cx));
        if custom.span == Span::Between && field == 0 {
            custom.field = 1;
            window.focus(&custom.dates[1].focus_handle(cx), cx);
        }
        cx.notify();
    }

    /// The popover, over everything, when open.
    pub(super) fn render_custom_popover(
        &self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let custom = &self.search_panel.as_ref()?.custom;
        if !custom.open {
            return None;
        }
        let chip_bounds = custom.chip.get()?;
        let first = Date::new(custom.year, custom.month, 1).ok()?;
        let between = custom.span == Span::Between;
        let fields = if between {
            CAPTION + 4.0 + FIELD
        } else {
            FIELD
        };
        let calendar = HEADER + WEEKDAY_ROW + weeks(first) as f32 * ROW;
        let error = custom.error.map_or(0.0, |_| ERROR + GAP);
        let height = 2.0 * PAD + CHIPS + fields + calendar + BUTTONS + 3.0 * GAP + error;
        let viewport = window.viewport_size();
        let (vw, vh) = (f32::from(viewport.width), f32::from(viewport.height));
        let (x, y, side, along) = place(chip_bounds, (WIDTH, height), (vw, vh));

        let span = custom.span;
        let spans: Vec<_> = SPANS
            .iter()
            .map(|&(value, label)| {
                chip(("span", value as usize), label, value == span, th).on_click(cx.listener(
                    move |this, _, _, cx| {
                        if let Some(custom) = this.custom_mut() {
                            custom.span = value;
                            custom.error = None;
                            if value != Span::Between {
                                custom.field = 0;
                            }
                        }
                        cx.notify();
                    },
                ))
            })
            .collect();
        // One date across, or From and To side by side.
        let field = |ix: usize, cx: &mut Context<Self>| {
            let active = custom.field == ix;
            let date = div()
                .id(("date-field", ix))
                .w_full()
                .h(px(FIELD))
                .pl(px(10.0))
                .pr(px(6.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(4.0))
                .rounded(px(8.0))
                .border_1()
                .border_color(rgba(if active && between {
                    th.accent
                } else {
                    th.divider
                }))
                .text_size(px(14.0))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| this.pick_field(ix, cx)),
                )
                .child(div().flex_1().min_w_0().child(custom.dates[ix].clone()))
                .child(crate::widgets::icon(
                    "calendar",
                    if active { th.accent } else { th.text_dim },
                    18.0,
                ));
            let label = between.then(|| {
                div()
                    .h(px(CAPTION))
                    .text_size(px(12.0))
                    .text_color(rgba(if active { th.accent } else { th.text_dim }))
                    .child(if ix == 0 { "From" } else { "To" })
            });
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .children(label)
                .child(date)
        };
        let mut rows = div().flex().flex_row().gap(px(8.0)).child(field(0, cx));
        if between {
            rows = rows.child(field(1, cx));
        }
        let error_line = custom.error.map(|error| {
            div()
                .h(px(ERROR))
                .text_size(px(13.0))
                .text_color(rgba(th.error))
                .child(error)
        });
        let buttons = div()
            .h(px(BUTTONS))
            .flex()
            .flex_row()
            .items_center()
            .justify_end()
            .gap(px(8.0))
            .child(
                div()
                    .id("custom-cancel")
                    .px(px(12.0))
                    .h(px(BUTTONS))
                    .flex()
                    .items_center()
                    .rounded_full()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(th.accent))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .on_click(cx.listener(|this, _, window, cx| this.custom_cancel(window, cx)))
                    .child("Cancel"),
            )
            .child(
                filled_button("custom-done", "Done", th)
                    .on_click(cx.listener(|this, _, window, cx| this.custom_done(window, cx))),
            );
        let popover = div()
            .id("custom-dates")
            .absolute()
            .left(px(x))
            .top(px(y))
            .w(px(WIDTH))
            .h(px(height))
            .p(px(PAD))
            .flex()
            .flex_col()
            .gap(px(GAP))
            .border_1()
            .border_color(rgba(th.divider))
            .map(|d| raised(d, th, RADIUS, 4.0))
            .text_color(rgba(th.text))
            .occlude()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .h(px(CHIPS))
                    .flex()
                    .flex_row()
                    .gap(px(6.0))
                    .children(spans),
            )
            .child(rows)
            .child(self.render_calendar(custom, first, th, cx))
            .children(error_line)
            .child(buttons)
            .children(notch(side, along, (WIDTH, height), th));
        let layer = div()
            .id("custom-dates-scrim")
            .relative()
            .w(px(vw))
            .h(px(vh))
            .occlude()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| this.custom_cancel(window, cx)),
            )
            .child(popover);
        Some(
            deferred(anchored().position(point(px(0.0), px(0.0))).child(layer))
                .with_priority(3)
                .into_any_element(),
        )
    }

    /// A month of days, Monday first, with the month and year to turn.
    fn render_calendar(
        &self,
        custom: &CustomDates,
        first: Date,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        let today = jiff::Zoned::now().date();
        let picked = [0, 1].map(|ix| parse_day(custom.dates[ix].read(cx).text()));
        let range = match (custom.span, picked) {
            (Span::Between, [Some(a), Some(b)]) => Some((a.min(b), a.max(b))),
            _ => None,
        };
        let turn = |id: &'static str, name: &'static str, months: i32, label: &'static str| {
            icon_button(id, name, 18.0, th)
                .tooltip(tip(label, th))
                .on_click(cx.listener(move |this, _, _, cx| this.turn_calendar(months, cx)))
        };
        // 4 × 40 px arrows + 90 + 44 = 294 px, the width of the days.
        let header = div()
            .h(px(HEADER))
            .flex()
            .flex_row()
            .items_center()
            .text_size(px(14.0))
            .font_weight(FontWeight::MEDIUM)
            .child(turn("month-back", "chevron-left", -1, "Previous month"))
            .child(
                div()
                    .w(px(90.0))
                    .flex()
                    .justify_center()
                    .child(MONTHS[custom.month as usize - 1]),
            )
            .child(turn("month-on", "chevron-right", 1, "Next month"))
            .child(turn("year-back", "chevron-left", -12, "Previous year"))
            .child(
                div()
                    .w(px(44.0))
                    .flex()
                    .justify_center()
                    .child(custom.year.to_string()),
            )
            .child(turn("year-on", "chevron-right", 12, "Next year"));
        let cell = || div().w(px(DAY)).flex().items_center().justify_center();
        let weekdays = div()
            .h(px(WEEKDAY_ROW))
            .flex()
            .flex_row()
            .children(WEEKDAYS.iter().map(|day| {
                cell()
                    .h(px(WEEKDAY_ROW))
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_faint))
                    .child(*day)
            }));
        let (th_accent, th_hover) = (th.accent, th.hover);
        let lead = first.weekday().to_monday_zero_offset();
        let mut rows = Vec::new();
        let mut week = Vec::new();
        for _ in 0..lead {
            week.push(cell().h(px(ROW)).into_any_element());
        }
        for number in 1..=first.days_in_month() {
            let Ok(day) = Date::new(custom.year, custom.month, number) else {
                continue;
            };
            let on = picked.contains(&Some(day));
            let inside = range.is_some_and(|(a, b)| a < day && day < b);
            let mut element = div()
                .id(("day", number as usize))
                .size(px(32.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .text_size(px(13.0))
                .cursor_pointer()
                .text_color(rgba(if on {
                    th.on_accent
                } else if day > today {
                    th.text_faint
                } else {
                    th.text
                }))
                // Set once: GPUI panics on a second hover style.
                .hover(move |s| s.bg(rgba(if on { th_accent } else { th_hover })))
                .on_click(cx.listener(move |this, _, window, cx| this.pick_day(day, window, cx)))
                .child(number.to_string());
            if on {
                element = element.bg(rgba(th.accent));
            } else if inside {
                element = element.bg(rgba(th.nav_selected));
            } else if day == today {
                element = element.border_1().border_color(rgba(th.accent));
            }
            week.push(cell().h(px(ROW)).child(element).into_any_element());
            if week.len() == 7 {
                rows.push(div().flex().flex_row().children(std::mem::take(&mut week)));
            }
        }
        if !week.is_empty() {
            rows.push(div().flex().flex_row().children(week));
        }
        div()
            .flex()
            .flex_col()
            .child(header)
            .child(weekdays)
            .children(rows)
    }
}

/// Puts the calendar on the month of the date it fills, or this month.
fn show_month(custom: &mut CustomDates, cx: &App) {
    let shown = parse_day(custom.dates[custom.field].read(cx).text())
        .unwrap_or_else(|| jiff::Zoned::now().date());
    custom.year = shown.year();
    custom.month = shown.month();
}

/// The notch on the popover's edge facing the chip: a border-colored
/// triangle with a popover-colored one just inside it, covering the
/// border where they meet.
fn notch(side: Side, along: f32, (w, h): (f32, f32), th: &Theme) -> [AnyElement; 2] {
    // The middle of the notch's base on the popover's edge, the way out
    // and the turn of an upward triangle to face that way.
    let (base, out, turn) = match side {
        Side::Below => ((along, 0.0), (0.0, -1.0), 0.0),
        Side::Above => ((along, h), (0.0, 1.0), PI),
        Side::Right => ((0.0, along), (-1.0, 0.0), -FRAC_PI_2),
        Side::Left => ((w, along), (1.0, 0.0), FRAC_PI_2),
    };
    let triangle = |scale: f32, inset: f32, color: u32| {
        // Its middle, half its length out from where its base sits.
        let (bw, bh) = (20.0 * scale, NOTCH * scale);
        let reach = bh / 2.0 - inset;
        let (mx, my) = (base.0 + out.0 * reach, base.1 + out.1 * reach);
        svg()
            .path("icons/notch.svg")
            .absolute()
            .left(px(mx - bw / 2.0))
            .top(px(my - bh / 2.0))
            .w(px(bw))
            .h(px(bh))
            .text_color(rgba(color))
            .with_transformation(Transformation::rotate(radians(turn)))
            .into_any_element()
    };
    [triangle(1.0, 0.0, th.divider), triangle(0.9, 1.0, th.menu)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_days() {
        assert_eq!(parse_day("2026-09-01"), Some(jiff::civil::date(2026, 9, 1)));
        assert_eq!(parse_day("2001/5/14"), Some(jiff::civil::date(2001, 5, 14)));
        assert_eq!(parse_day("2026-02-30"), None);
        assert_eq!(parse_day("1 Sep 2026"), None);
    }

    #[test]
    fn custom_dates_are_local() {
        let dhaka = TimeZone::fixed(jiff::tz::offset(6));
        let dates = |span, first, last| custom_dates(span, first, last, &dhaka);
        assert_eq!(
            dates(Span::On, "2026-09-01", "").unwrap(),
            "after:2026-09-01T00:00+06:00 before:2026-09-02T00:00+06:00"
        );
        assert_eq!(
            dates(Span::Before, "2026-09-01", "").unwrap(),
            "before:2026-09-01T00:00+06:00"
        );
        assert_eq!(
            dates(Span::Since, "2026-09-01", "").unwrap(),
            "after:2026-09-01T00:00+06:00"
        );
        assert_eq!(
            dates(Span::Between, "2026-09-01", "2026-09-10").unwrap(),
            "after:2026-09-01T00:00+06:00 before:2026-09-11T00:00+06:00"
        );
        // Given backwards; both days are included.
        assert_eq!(
            dates(Span::Between, "2026-09-10", "2026-09-01").unwrap(),
            "after:2026-09-01T00:00+06:00 before:2026-09-11T00:00+06:00"
        );
        assert_eq!(dates(Span::On, "", ""), Err("Pick a date"));
        assert_eq!(dates(Span::Between, "2026-09-01", ""), Err("Pick a date"));
        assert!(dates(Span::Since, "2026-13-01", "").is_err());

        // The search parser reads them as the right instants.
        let query = dates(Span::On, "2001-05-14", "").unwrap();
        let start = 989_798_400 - 6 * 3_600;
        assert_eq!(
            katna_search::Query::parse_at(&query, 0).unwrap(),
            katna_search::Query::And(vec![
                katna_search::Query::Filter(katna_search::Filter::After(start)),
                katna_search::Query::Filter(katna_search::Filter::Before(start + 86_400)),
            ])
        );
    }

    #[test]
    fn labels_the_chip() {
        assert_eq!(
            custom_label(Span::On, "2002-05-14", "").as_deref(),
            Some("May 14, 2002")
        );
        assert_eq!(
            custom_label(Span::Since, "2002-05-14", "").as_deref(),
            Some("Since May 14, 2002")
        );
        assert_eq!(
            custom_label(Span::Between, "2002-05-31", "2002-05-01").as_deref(),
            Some("May 1 – May 31, 2002")
        );
        assert_eq!(
            custom_label(Span::Between, "2001-12-30", "2002-01-02").as_deref(),
            Some("Dec 30, 2001 – Jan 2, 2002")
        );
    }

    #[test]
    fn places_the_popover() {
        let chip = |x: f32, y: f32| Bounds {
            origin: point(px(x), px(y)),
            size: gpui::size(px(72.0), px(28.0)),
        };
        let size = (WIDTH, 440.0);
        // Room below: under the chip, notch at the chip's middle.
        let (x, y, side, along) = place(chip(660.0, 382.0), size, (1400.0, 1000.0));
        assert_eq!(side, Side::Below);
        assert_eq!(y, 382.0 + 28.0 + NOTCH + SPACE);
        assert_eq!(x + along, 696.0);
        // No room below or above: beside it, notch at its middle.
        let (x, y, side, along) = place(chip(660.0, 382.0), size, (1400.0, 800.0));
        assert_eq!(side, Side::Right);
        assert_eq!(x, 732.0 + NOTCH + SPACE);
        assert_eq!(y + along, 396.0);
        // Near the bottom: above.
        let (_, y, side, _) = place(chip(660.0, 700.0), size, (1400.0, 800.0));
        assert_eq!(side, Side::Above);
        assert_eq!(y, 700.0 - NOTCH - SPACE - 440.0);
    }

    #[test]
    fn counts_weeks() {
        assert_eq!(weeks(jiff::civil::date(2002, 5, 1)), 5);
        assert_eq!(weeks(jiff::civil::date(2026, 3, 1)), 6);
        assert_eq!(weeks(jiff::civil::date(2027, 2, 1)), 4);
    }
}
