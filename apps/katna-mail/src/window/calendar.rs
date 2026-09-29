// SPDX-License-Identifier: GPL-3.0-or-later

//! The Calendar page (`docs/ARCHITECTURE.md` §18), laid out like Google
//! Calendar: a side column with a small month and the calendars of each
//! account, and the main view as Day, Week, Month or Schedule, with the
//! page's own bar (Today, back and forward, the dates shown, the views).
//! Events come from `pim.db`, which the daemon syncs; repeating ones are
//! expanded by `katna_dav`. Google Calendar's keys work: T today, J or N
//! next, K or P back, D W M A (or 1 2 3 4) for the views.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use gpui::{
    Animation, AnimationExt, AnyElement, App, ClickEvent, Context, FocusHandle, FontWeight,
    KeyBinding, MouseButton, Pixels, Point, ScrollHandle, SharedString, Task, Window, anchored,
    deferred, div, ease_out_quint, prelude::*, rgba,
};
use jiff::civil::{Date, DateTime, Time};
use jiff::tz::TimeZone;
use jiff::{ToSpan, Zoned};
use katna_core::Paths;
use katna_dav::Occurrence;
use katna_i18n::{format, tr};
use katna_store::calendar::{Calendar, EventStatus};
use katna_store::{Mode, Store};
use katna_ui::px;

use super::MailWindow;
use crate::theme::{Theme, fade, mix};
use crate::widgets::{icon, icon_button, outlined_button, raised, tip};

gpui::actions!(
    katna_calendar,
    [
        CalendarToday,
        CalendarNext,
        CalendarPrevious,
        CalendarDayView,
        CalendarWeekView,
        CalendarMonthView,
        CalendarScheduleView,
        CalendarCloseEvent,
    ]
);

/// The key context of the Calendar page.
pub(super) const CALENDAR_CONTEXT: &str = "CalendarPage";

/// Google Calendar's keys, in the Calendar page only.
pub(super) fn bindings() -> Vec<KeyBinding> {
    let c = Some(CALENDAR_CONTEXT);
    vec![
        KeyBinding::new("t", CalendarToday, c),
        KeyBinding::new("j", CalendarNext, c),
        KeyBinding::new("n", CalendarNext, c),
        KeyBinding::new("k", CalendarPrevious, c),
        KeyBinding::new("p", CalendarPrevious, c),
        KeyBinding::new("d", CalendarDayView, c),
        KeyBinding::new("1", CalendarDayView, c),
        KeyBinding::new("w", CalendarWeekView, c),
        KeyBinding::new("2", CalendarWeekView, c),
        KeyBinding::new("m", CalendarMonthView, c),
        KeyBinding::new("3", CalendarMonthView, c),
        KeyBinding::new("a", CalendarScheduleView, c),
        KeyBinding::new("4", CalendarScheduleView, c),
        KeyBinding::new("escape", CalendarCloseEvent, c),
    ]
}

/// The side column: the small month and the calendars.
const SIDE_WIDTH: f32 = 256.0;
/// The page's own bar.
const BAR_HEIGHT: f32 = 64.0;
/// An hour of the Day and Week grids.
const HOUR_HEIGHT: f32 = 48.0;
/// The hours down the left of the grid.
const GUTTER: f32 = 64.0;
/// A line of the whole-day row.
const ALL_DAY_LINE: f32 = 24.0;
/// A line of an event in a month's day.
const MONTH_LINE: f32 = 22.0;
/// The small month's days.
const MINI_DAY: f32 = 28.0;
/// How many days Schedule lists at once.
const SCHEDULE_DAYS: i64 = 60;
/// The event card's width.
const CARD_WIDTH: f32 = 400.0;
/// Where the grid opens: a little before 8 in the morning, as Google does.
const MORNING_HOUR: f32 = 7.5;

/// How the page shows the days.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CalView {
    Day,
    Week,
    Month,
    Schedule,
}

impl CalView {
    const ALL: [Self; 4] = [Self::Day, Self::Week, Self::Month, Self::Schedule];

    fn label(self) -> String {
        tr!(match self {
            Self::Day => "calendar-view-day",
            Self::Week => "calendar-view-week",
            Self::Month => "calendar-view-month",
            Self::Schedule => "calendar-view-schedule",
        })
    }
}

/// What was read for the dates on show.
struct Loaded {
    occurrences: Rc<Vec<Occurrence>>,
}

/// An open event's card.
struct OpenEvent {
    occurrence: Occurrence,
    at: Point<Pixels>,
}

/// The state of the Calendar page.
pub(super) struct CalendarPage {
    view: CalView,
    /// The day the views are on.
    day: Date,
    /// The month the small month shows, by its first day.
    mini: Date,
    loaded: Option<Loaded>,
    calendars: Rc<Vec<Calendar>>,
    /// Calendars unticked here and not yet read back as hidden.
    hidden: HashSet<i64>,
    /// Accounts whose calendars are folded away in the side column.
    folded: HashSet<Option<i64>>,
    loading: bool,
    error: Option<String>,
    task: Option<Task<()>>,
    grid_scroll: ScrollHandle,
    /// The grid was scrolled to the morning since it last appeared.
    scrolled: bool,
    open: Option<OpenEvent>,
    pub(super) focus: FocusHandle,
}

impl CalendarPage {
    pub(super) fn new(cx: &mut App) -> Self {
        let today = Zoned::now().date();
        Self {
            view: CalView::Week,
            day: today,
            mini: today.first_of_month(),
            loaded: None,
            calendars: Rc::new(Vec::new()),
            hidden: HashSet::new(),
            folded: HashSet::new(),
            loading: false,
            error: None,
            task: None,
            grid_scroll: ScrollHandle::new(),
            scrolled: false,
            open: None,
            focus: cx.focus_handle(),
        }
    }

    /// The first day of the week `day` is in, as the language starts weeks.
    fn week_start(day: Date) -> Date {
        let first = format::first_weekday();
        let back =
            (day.weekday().to_monday_zero_offset() - first.to_monday_zero_offset()).rem_euclid(7);
        day.checked_sub(i64::from(back).days()).unwrap_or(day)
    }

    /// The days on show: first and after the last.
    fn days(&self) -> (Date, Date) {
        match self.view {
            CalView::Day => (self.day, self.day.tomorrow().unwrap_or(self.day)),
            CalView::Week => {
                let first = Self::week_start(self.day);
                (first, first.checked_add(7.days()).unwrap_or(first))
            }
            CalView::Month => {
                let first = Self::week_start(self.day.first_of_month());
                (first, first.checked_add(42.days()).unwrap_or(first))
            }
            CalView::Schedule => (
                self.day,
                self.day
                    .checked_add(SCHEDULE_DAYS.days())
                    .unwrap_or(self.day),
            ),
        }
    }

    /// Moves a view's worth of days: forward for 1, back for -1.
    fn step(&mut self, by: i64) {
        let span = match self.view {
            CalView::Day => by.days(),
            CalView::Week => (7 * by).days(),
            CalView::Month => by.months(),
            CalView::Schedule => (SCHEDULE_DAYS * by).days(),
        };
        if let Ok(day) = self.day.checked_add(span) {
            self.day = if self.view == CalView::Month {
                day.first_of_month()
            } else {
                day
            };
            self.mini = self.day.first_of_month();
        }
    }
}

fn midnight(day: Date, tz: &TimeZone) -> i64 {
    day.to_zoned(tz.clone())
        .map_or(0, |z| z.timestamp().as_second())
}

fn civil(seconds: i64, tz: &TimeZone) -> DateTime {
    jiff::Timestamp::from_second(seconds)
        .unwrap_or(jiff::Timestamp::UNIX_EPOCH)
        .to_zoned(tz.clone())
        .datetime()
}

/// `#rrggbb` as `0xRRGGBBAA`.
fn parse_color(text: &str) -> Option<u32> {
    let hex = text.strip_prefix('#')?;
    (hex.len() == 6)
        .then(|| u32::from_str_radix(hex, 16).ok())
        .flatten()
        .map(|rgb| (rgb << 8) | 0xff)
}

/// Google's blue, for calendars without a color.
const DEFAULT_COLOR: u32 = 0x039b_e5ff;

/// Reads the calendars and the occurrences in `from..to`.
fn read(
    paths: &Paths,
    from: i64,
    to: i64,
    tz: &TimeZone,
) -> Result<(Vec<Calendar>, Vec<Occurrence>), String> {
    let store = Store::open(paths, Mode::ReadOnly).map_err(|err| err.to_string())?;
    let calendars = store.calendars().map_err(|err| err.to_string())?;
    let rows = store
        .event_rows_in_range(from, to)
        .map_err(|err| err.to_string())?;
    Ok((calendars, katna_dav::occurrences(rows, from, to, tz)))
}

impl MailWindow {
    /// Reads what the Calendar page shows, when it is on show.
    pub(super) fn load_calendar(&mut self, cx: &mut Context<Self>) {
        let (first, end) = self.calendar.days();
        let tz = self.tz.clone();
        let (from, to) = (midnight(first, &tz), midnight(end, &tz));
        let paths = self.paths.clone();
        self.calendar.loading = true;
        self.calendar.task = Some(cx.spawn(async move |this, cx| {
            let read = cx
                .background_executor()
                .spawn(async move { read(&paths, from, to, &tz) })
                .await;
            this.update(cx, |this, cx| {
                let page = &mut this.calendar;
                page.loading = false;
                match read {
                    Ok((calendars, occurrences)) => {
                        page.hidden
                            .retain(|id| calendars.iter().any(|c| c.id == *id && !c.hidden));
                        page.calendars = Rc::new(calendars);
                        page.loaded = Some(Loaded {
                            occurrences: Rc::new(occurrences),
                        });
                        page.error = None;
                    }
                    Err(err) => {
                        tracing::warn!(%err, "reading the calendar failed");
                        page.error = Some(err);
                    }
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn calendar_moved(&mut self, cx: &mut Context<Self>) {
        self.calendar.open = None;
        self.load_calendar(cx);
        cx.notify();
    }

    fn calendar_today(&mut self, _: &CalendarToday, _: &mut Window, cx: &mut Context<Self>) {
        let today = Zoned::now().with_time_zone(self.tz.clone()).date();
        self.calendar.day = today;
        self.calendar.mini = today.first_of_month();
        self.calendar.scrolled = false;
        self.calendar_moved(cx);
    }

    fn calendar_next(&mut self, _: &CalendarNext, _: &mut Window, cx: &mut Context<Self>) {
        self.calendar.step(1);
        self.calendar_moved(cx);
    }

    fn calendar_previous(&mut self, _: &CalendarPrevious, _: &mut Window, cx: &mut Context<Self>) {
        self.calendar.step(-1);
        self.calendar_moved(cx);
    }

    fn set_calendar_view(&mut self, view: CalView, cx: &mut Context<Self>) {
        if self.calendar.view != view {
            self.calendar.view = view;
            self.calendar.scrolled = false;
            self.calendar_moved(cx);
        }
    }

    fn close_calendar_event(
        &mut self,
        _: &CalendarCloseEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.calendar.open.take().is_some() {
            cx.notify();
        } else {
            cx.propagate();
        }
    }

    fn open_calendar_day(&mut self, day: Date, view: Option<CalView>, cx: &mut Context<Self>) {
        self.calendar.day = day;
        self.calendar.mini = day.first_of_month();
        if let Some(view) = view {
            self.calendar.view = view;
        }
        self.calendar.scrolled = false;
        self.calendar_moved(cx);
    }

    fn toggle_calendar(&mut self, id: i64, cx: &mut Context<Self>) {
        let shown = !self.calendar_hidden(id);
        if shown {
            self.calendar.hidden.insert(id);
        } else {
            self.calendar.hidden.remove(&id);
        }
        self.set_calendar_shown(id, !shown, cx);
        cx.notify();
    }

    fn calendar_hidden(&self, id: i64) -> bool {
        self.calendar.hidden.contains(&id)
            || self
                .calendar
                .calendars
                .iter()
                .any(|c| c.id == id && c.hidden)
    }

    /// Tells the daemon to show or hide calendar `id`'s events; the page
    /// hides them at once and reads the change back.
    fn set_calendar_shown(&mut self, id: i64, shown: bool, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        cx.spawn(async move |this, cx| {
            let result = crate::daemon::set_calendar_hidden(&connection, id, !shown).await;
            this.update(cx, |this, cx| {
                if let Err(err) = result {
                    tracing::warn!(%err, "could not show or hide a calendar");
                    if shown {
                        this.calendar.hidden.insert(id);
                    } else {
                        this.calendar.hidden.remove(&id);
                    }
                }
                this.load_calendar(cx);
            })
            .ok();
        })
        .detach();
    }

    /// The color of an event: its own, else its calendar's.
    fn event_color(&self, occurrence: &Occurrence) -> u32 {
        parse_color(&occurrence.event.data.color)
            .or_else(|| {
                self.calendar
                    .calendars
                    .iter()
                    .find(|c| c.id == occurrence.event.calendar_id)
                    .and_then(|c| parse_color(&c.color))
            })
            .unwrap_or(DEFAULT_COLOR)
    }

    /// The occurrences on show, without those of calendars just unticked.
    fn shown_occurrences(&self) -> Vec<Occurrence> {
        let Some(loaded) = &self.calendar.loaded else {
            return Vec::new();
        };
        loaded
            .occurrences
            .iter()
            .filter(|o| !self.calendar.hidden.contains(&o.event.calendar_id))
            .cloned()
            .collect()
    }

    pub(super) fn render_calendar_page(
        &mut self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        if self.calendar.loaded.is_none() && !self.calendar.loading && self.calendar.error.is_none()
        {
            self.load_calendar(cx);
        }
        let main = if let Some(err) = &self.calendar.error {
            crate::widgets::placeholder(&tr!("calendar-read-failed", error = err.clone()), th)
        } else if self.calendar.loaded.is_none() {
            loading(th)
        } else if self.calendar.calendars.is_empty() {
            self.render_calendar_empty(th)
        } else {
            match self.calendar.view {
                CalView::Day | CalView::Week => self.render_time_grid(th, cx),
                CalView::Month => self.render_month(th, cx),
                CalView::Schedule => self.render_schedule(th, cx),
            }
        };
        div()
            .id("calendar-page")
            .key_context(CALENDAR_CONTEXT)
            .track_focus(&self.calendar.focus)
            .on_action(cx.listener(Self::calendar_today))
            .on_action(cx.listener(Self::calendar_next))
            .on_action(cx.listener(Self::calendar_previous))
            .on_action(cx.listener(|this, _: &CalendarDayView, _, cx| {
                this.set_calendar_view(CalView::Day, cx)
            }))
            .on_action(cx.listener(|this, _: &CalendarWeekView, _, cx| {
                this.set_calendar_view(CalView::Week, cx)
            }))
            .on_action(cx.listener(|this, _: &CalendarMonthView, _, cx| {
                this.set_calendar_view(CalView::Month, cx)
            }))
            .on_action(cx.listener(|this, _: &CalendarScheduleView, _, cx| {
                this.set_calendar_view(CalView::Schedule, cx)
            }))
            .on_action(cx.listener(Self::close_calendar_event))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    window.focus(&this.calendar.focus, cx);
                }),
            )
            .relative()
            .size_full()
            .flex()
            .flex_row()
            .child(self.render_calendar_side(th, cx))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .flex()
                    .flex_col()
                    .child(self.render_calendar_bar(th, cx))
                    .child(div().flex_1().min_h_0().child(main)),
            )
            .children(self.render_event_card(th, cx))
            .into_any_element()
    }

    fn render_calendar_bar(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let page = &self.calendar;
        let (first, end) = page.days();
        let last = end.yesterday().unwrap_or(end);
        let title = match page.view {
            CalView::Day => format::day_month_year(page.day.to_datetime(Time::midnight())),
            CalView::Month => format::month_year(page.day),
            CalView::Week | CalView::Schedule => {
                if first.year() == last.year() && first.month() == last.month() {
                    format::month_year(first)
                } else if first.year() == last.year() {
                    tr!(
                        "calendar-title-months",
                        first = format::month_name(first.month()),
                        last = format::month_year(last)
                    )
                } else {
                    tr!(
                        "calendar-title-months",
                        first = format::month_year(first),
                        last = format::month_year(last)
                    )
                }
            }
        };
        let (previous, next) = match page.view {
            CalView::Day => ("calendar-previous-day", "calendar-next-day"),
            CalView::Week => ("calendar-previous-week", "calendar-next-week"),
            CalView::Month => ("calendar-previous-month", "calendar-next-month"),
            CalView::Schedule => ("calendar-previous-period", "calendar-next-period"),
        };
        let views = CalView::ALL.into_iter().map(|view| {
            let on = view == page.view;
            div()
                .id(("calendar-view", view as usize))
                .h(px(32.0))
                .px(px(14.0))
                .flex()
                .items_center()
                .rounded_full()
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .cursor_pointer()
                .when(on, |d| {
                    d.bg(rgba(th.nav_selected))
                        .text_color(rgba(th.nav_selected_text))
                })
                .when(!on, |d| {
                    d.text_color(rgba(th.text_dim))
                        .hover(|s| s.bg(rgba(th.hover)))
                })
                .on_click(cx.listener(move |this, _, _, cx| this.set_calendar_view(view, cx)))
                .child(view.label())
        });
        div()
            .flex_none()
            .h(px(BAR_HEIGHT))
            .px(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .border_b_1()
            .border_color(rgba(th.divider))
            .child(
                outlined_button("calendar-today", tr!("calendar-today"), th)
                    .tooltip(tip(tr!("calendar-today-tip"), th))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.calendar_today(&CalendarToday, window, cx)
                    })),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .child(
                        icon_button("calendar-previous", "chevron-left", 22.0, th)
                            .tooltip(tip(tr!(previous), th))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.calendar_previous(&CalendarPrevious, window, cx)
                            })),
                    )
                    .child(
                        icon_button("calendar-next", "chevron-right", 22.0, th)
                            .tooltip(tip(tr!(next), th))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.calendar_next(&CalendarNext, window, cx)
                            })),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .text_size(px(22.0))
                    .text_color(rgba(th.text))
                    .child(title),
            )
            .when(page.loading, |d| {
                d.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_faint))
                        .child(tr!("calendar-loading")),
                )
            })
            .child(
                div()
                    .flex_none()
                    .p(px(2.0))
                    .flex()
                    .flex_row()
                    .gap(px(2.0))
                    .rounded_full()
                    .border_1()
                    .border_color(rgba(fade(th.text_faint, 0.5)))
                    .children(views),
            )
            .into_any_element()
    }

    fn render_calendar_side(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            .id("calendar-side")
            .flex_none()
            .w(px(SIDE_WIDTH))
            .h_full()
            .overflow_y_scroll()
            .px(px(12.0))
            .pt(px(16.0))
            .pb(px(16.0))
            .flex()
            .flex_col()
            .gap(px(16.0))
            .border_r_1()
            .border_color(rgba(th.divider))
            .child(self.render_mini_month(th, cx))
            .child(self.render_calendar_list(th, cx))
            .into_any_element()
    }

    fn render_mini_month(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let page = &self.calendar;
        let today = Zoned::now().with_time_zone(self.tz.clone()).date();
        let first = CalendarPage::week_start(page.mini);
        let (shown_first, shown_end) = page.days();
        let head = format::weekdays_short().into_iter().map(|(_, name)| {
            div()
                .w(px(MINI_DAY))
                .h(px(20.0))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(11.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_faint))
                .child(name.chars().take(1).collect::<String>())
        });
        let weeks = (0..6).map(|week| {
            let days = (0..7).map(move |ix| {
                let day = first.checked_add((week * 7 + ix).days()).unwrap_or(first);
                (day, ix)
            });
            div().flex().flex_row().children(days.map(|(day, _)| {
                let is_today = day == today;
                let picked = day == page.day;
                let in_month = day.month() == page.mini.month();
                let on_show = page.view != CalView::Month
                    && page.view != CalView::Schedule
                    && day >= shown_first
                    && day < shown_end;
                div()
                    .id(SharedString::from(format!("mini-{day}")))
                    .w(px(MINI_DAY))
                    .h(px(MINI_DAY))
                    .flex()
                    .items_center()
                    .justify_center()
                    .when(on_show && !is_today, |d| {
                        d.bg(rgba(fade(th.nav_selected, 0.5)))
                    })
                    .child(
                        div()
                            .size(px(MINI_DAY - 4.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_full()
                            .cursor_pointer()
                            .text_size(px(11.0))
                            .when(is_today, |d| {
                                d.bg(rgba(th.accent))
                                    .text_color(rgba(th.on_accent))
                                    .font_weight(FontWeight::BOLD)
                            })
                            .when(!is_today && picked, |d| {
                                d.bg(rgba(th.nav_selected))
                                    .text_color(rgba(th.nav_selected_text))
                                    .font_weight(FontWeight::BOLD)
                            })
                            .when(!is_today && !picked, |d| {
                                d.text_color(rgba(if in_month { th.text } else { th.text_faint }))
                                    .hover(|s| s.bg(rgba(th.hover)))
                            })
                            .child(format::number(day.day() as u64)),
                    )
                    .on_click(
                        cx.listener(move |this, _, _, cx| this.open_calendar_day(day, None, cx)),
                    )
            }))
        });
        let turn = |id: &'static str, name: &'static str, by: i64| {
            icon_button(id, name, 18.0, th)
                .size(px(28.0))
                .on_click(cx.listener(move |this, _, _, cx| {
                    if let Ok(month) = this.calendar.mini.checked_add(by.months()) {
                        this.calendar.mini = month;
                        cx.notify();
                    }
                }))
        };
        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .h(px(32.0))
                    .pl(px(8.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.text))
                            .child(format::month_year(page.mini)),
                    )
                    .child(turn("mini-previous", "chevron-left", -1))
                    .child(turn("mini-next", "chevron-right", 1)),
            )
            .child(div().flex().flex_row().children(head))
            .children(weeks)
            .into_any_element()
    }

    fn render_calendar_list(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let mut groups: Vec<(Option<i64>, Vec<&Calendar>)> = Vec::new();
        for calendar in self.calendar.calendars.iter() {
            let key = calendar.account.map(|a| a.0);
            match groups.iter_mut().find(|(k, _)| *k == key) {
                Some((_, list)) => list.push(calendar),
                None => groups.push((key, vec![calendar])),
            }
        }
        let names: HashMap<i64, String> = self
            .accounts
            .iter()
            .map(|a: &katna_core::Account| (a.id.0, a.address.clone()))
            .collect();
        let groups = groups.into_iter().map(|(account, calendars)| {
            let folded = self.calendar.folded.contains(&account);
            let name = match account {
                Some(id) => names
                    .get(&id)
                    .cloned()
                    .unwrap_or_else(|| tr!("calendar-account-gone")),
                None => tr!("calendar-local"),
            };
            let rows = calendars.into_iter().map(|calendar| {
                let id = calendar.id;
                let shown = !self.calendar_hidden(id);
                let color = parse_color(&calendar.color).unwrap_or(DEFAULT_COLOR);
                div()
                    .id(("calendar-row", id as usize))
                    .h(px(32.0))
                    .pl(px(8.0))
                    .pr(px(8.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .rounded(px(8.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .on_click(cx.listener(move |this, _, _, cx| this.toggle_calendar(id, cx)))
                    .child(
                        div()
                            .flex_none()
                            .size(px(18.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded(px(3.0))
                            .border_2()
                            .border_color(rgba(color))
                            .when(shown, |d| d.bg(rgba(color)))
                            .when(shown, |d| d.child(icon("check", 0xffff_ffff, 14.0))),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .text_size(px(14.0))
                            .text_color(rgba(th.text))
                            .child(calendar.name.clone()),
                    )
            });
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .id(SharedString::from(format!("calendar-account-{account:?}")))
                        .h(px(36.0))
                        .pl(px(8.0))
                        .pr(px(4.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .rounded(px(8.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(th.hover)))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if !this.calendar.folded.remove(&account) {
                                this.calendar.folded.insert(account);
                            }
                            cx.notify();
                        }))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .truncate()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgba(th.text))
                                .child(name),
                        )
                        .child(icon(
                            if folded { "chevron-down" } else { "chevron-up" },
                            th.text_dim,
                            20.0,
                        )),
                )
                .when(!folded, |d| d.children(rows))
        });
        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .children(groups)
            .into_any_element()
    }

    fn render_calendar_empty(&self, th: &Theme) -> AnyElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(12.0))
            .child(
                div()
                    .size(px(96.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .bg(rgba(th.nav_selected))
                    .child(icon("calendar", th.nav_selected_text, 48.0)),
            )
            .child(
                div()
                    .pt(px(8.0))
                    .text_size(px(22.0))
                    .text_color(rgba(th.text))
                    .child(tr!("calendar-empty-title")),
            )
            .child(
                div()
                    .max_w(px(440.0))
                    .text_center()
                    .text_size(px(14.0))
                    .line_height(px(21.0))
                    .text_color(rgba(th.text_faint))
                    .child(tr!("calendar-empty-text")),
            )
            .into_any_element()
    }

    /// The Day and Week views: the days across, their whole-day events on
    /// top, and the hours down, scrolled to the morning.
    fn render_time_grid(&mut self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let (first, end) = self.calendar.days();
        let count = (end - first).get_days().max(1) as usize;
        let days: Vec<Date> = (0..count)
            .filter_map(|i| first.checked_add((i as i64).days()).ok())
            .collect();
        let tz = self.tz.clone();
        let now = Zoned::now().with_time_zone(tz.clone());
        let today = now.date();
        let occurrences = self.shown_occurrences();
        let (all_day, timed): (Vec<Occurrence>, Vec<Occurrence>) = occurrences
            .into_iter()
            .partition(|o| o.all_day() || o.end - o.start >= 24 * 3600);

        if !self.calendar.scrolled {
            self.calendar.scrolled = true;
            let hour = if days.contains(&today) {
                (now.hour() as f32 - 1.5).clamp(0.0, 16.0)
            } else {
                MORNING_HOUR
            };
            self.calendar
                .grid_scroll
                .set_offset(gpui::point(px(0.0), -px(HOUR_HEIGHT * hour)));
        }

        // The header: weekday and date of each day.
        let head = days
            .iter()
            .map(|&day| {
                let is_today = day == today;
                let past = day < today;
                div()
                    .id(SharedString::from(format!("day-head-{day}")))
                    .flex_1()
                    .min_w_0()
                    .pt(px(8.0))
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(2.0))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.open_calendar_day(day, Some(CalView::Day), cx)
                    }))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(if is_today { th.accent } else { th.text_dim }))
                            .child(
                                format::weekday(day.to_datetime(Time::midnight())).to_uppercase(),
                            ),
                    )
                    .child(
                        div()
                            .size(px(44.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_full()
                            .text_size(px(24.0))
                            .when(is_today, |d| {
                                d.bg(rgba(th.accent)).text_color(rgba(th.on_accent))
                            })
                            .when(!is_today, |d| {
                                d.text_color(rgba(if past { th.text_dim } else { th.text }))
                                    .hover(|s| s.bg(rgba(th.hover)))
                            })
                            .child(format::number(day.day() as u64)),
                    )
            })
            .collect::<Vec<_>>();

        // Whole-day events, in lanes across the days they cover.
        let day_index = |seconds: i64| -> i64 {
            let date = civil(seconds, &tz).date();
            (date - first).get_days() as i64
        };
        let mut lanes: Vec<Vec<(i64, i64)>> = Vec::new();
        let mut bars = Vec::new();
        for occurrence in &all_day {
            let from = day_index(occurrence.start).max(0);
            let last = day_index((occurrence.end - 1).max(occurrence.start)).min(count as i64 - 1);
            if last < from {
                continue;
            }
            let lane = lanes
                .iter()
                .position(|taken| taken.iter().all(|&(a, b)| last < a || from > b))
                .unwrap_or_else(|| {
                    lanes.push(Vec::new());
                    lanes.len() - 1
                });
            lanes[lane].push((from, last));
            bars.push((occurrence.clone(), lane, from, last));
        }
        let all_day_height = (lanes.len().max(1) as f32) * ALL_DAY_LINE + 4.0;
        let bars = bars
            .into_iter()
            .map(|(occurrence, lane, from, last)| {
                let color = self.event_color(&occurrence);
                let width = (last - from + 1) as f32 / count as f32;
                let left = from as f32 / count as f32;
                let title = occurrence.event.data.title.clone();
                let open = occurrence.clone();
                div()
                    .id(("all-day", occurrence.event.id as usize * 64 + lane))
                    .absolute()
                    .top(px(2.0 + lane as f32 * ALL_DAY_LINE))
                    .left(gpui::relative(left))
                    .w(gpui::relative(width))
                    .h(px(ALL_DAY_LINE - 2.0))
                    .px(px(2.0))
                    .child(
                        div()
                            .size_full()
                            .px(px(8.0))
                            .flex()
                            .items_center()
                            .rounded(px(4.0))
                            .bg(rgba(color))
                            .text_color(rgba(0xffff_ffff))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::MEDIUM)
                            .cursor_pointer()
                            .truncate()
                            .child(title),
                    )
                    .on_click(cx.listener(move |this, event: &ClickEvent, _, cx| {
                        this.open_calendar_event(open.clone(), event.position(), cx)
                    }))
            })
            .collect::<Vec<_>>();

        // The hours down the left.
        let hours = (1..24).map(|hour| {
            let label = format::time(
                today.to_datetime(Time::new(hour, 0, 0, 0).unwrap_or(Time::midnight())),
            );
            div()
                .absolute()
                .top(px(hour as f32 * HOUR_HEIGHT - 7.0))
                .right(px(8.0))
                .text_size(px(10.0))
                .text_color(rgba(th.text_faint))
                .child(label)
        });
        let lines = (1..24).map(|hour| {
            div()
                .absolute()
                .top(px(hour as f32 * HOUR_HEIGHT))
                .left_0()
                .right_0()
                .h(px(1.0))
                .bg(rgba(th.divider))
        });
        let columns = days.iter().enumerate().map(|(ix, &day)| {
            let start = midnight(day, &tz);
            let next = midnight(day.tomorrow().unwrap_or(day), &tz);
            let mine: Vec<&Occurrence> = timed
                .iter()
                .filter(|o| {
                    o.start < next && (o.end > start || (o.end == o.start && o.start >= start))
                })
                .collect();
            let placed = lay_out(&mine, start, next);
            let is_today = day == today;
            let now_y = ((now.timestamp().as_second() - start) as f32 / 3600.0) * HOUR_HEIGHT;
            div()
                .relative()
                .flex_1()
                .min_w_0()
                .h_full()
                .when(ix > 0, |d| d.border_l_1().border_color(rgba(th.divider)))
                .children(
                    placed
                        .into_iter()
                        .map(|(occurrence, top, height, col, cols)| {
                            self.render_timed_event(occurrence, top, height, col, cols, &tz, th, cx)
                        }),
                )
                .when(is_today, |d| {
                    d.child(
                        div()
                            .absolute()
                            .top(px(now_y - 1.0))
                            .left_0()
                            .right_0()
                            .h(px(2.0))
                            .bg(rgba(0xea43_35ff)),
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(now_y - 6.0))
                            .left(px(-6.0))
                            .size(px(12.0))
                            .rounded_full()
                            .bg(rgba(0xea43_35ff)),
                    )
                })
        });
        let zone = now.strftime("GMT%:z").to_string().replace(":00", "");
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex_none()
                    .flex()
                    .flex_row()
                    .child(div().flex_none().w(px(GUTTER)))
                    .children(head),
            )
            .child(
                div()
                    .flex_none()
                    .flex()
                    .flex_row()
                    .border_b_1()
                    .border_color(rgba(th.divider))
                    .child(
                        div()
                            .flex_none()
                            .w(px(GUTTER))
                            .pr(px(8.0))
                            .flex()
                            .justify_end()
                            .items_end()
                            .text_size(px(10.0))
                            .text_color(rgba(th.text_faint))
                            .child(zone),
                    )
                    .child(
                        div()
                            .relative()
                            .flex_1()
                            .h(px(all_day_height))
                            .children(bars),
                    ),
            )
            .child(
                div()
                    .id("calendar-grid")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .track_scroll(&self.calendar.grid_scroll)
                    .child(
                        div()
                            .relative()
                            .h(px(24.0 * HOUR_HEIGHT))
                            .flex()
                            .flex_row()
                            .child(
                                div()
                                    .relative()
                                    .flex_none()
                                    .w(px(GUTTER))
                                    .h_full()
                                    .children(hours),
                            )
                            .child(
                                div()
                                    .relative()
                                    .flex_1()
                                    .h_full()
                                    .flex()
                                    .flex_row()
                                    .border_l_1()
                                    .border_color(rgba(th.divider))
                                    .children(lines)
                                    .children(columns),
                            ),
                    ),
            )
            .into_any_element()
    }

    #[allow(clippy::too_many_arguments)]
    fn render_timed_event(
        &self,
        occurrence: Occurrence,
        top: f32,
        height: f32,
        col: usize,
        cols: usize,
        tz: &TimeZone,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let color = self.event_color(&occurrence);
        let data = &occurrence.event.data;
        let declined = data.self_status == "declined";
        let tentative = data.self_status == "tentative" || data.self_status == "needs_action";
        let past = occurrence.end < jiff::Timestamp::now().as_second();
        let fill = if th.dark {
            mix(th.surface, color, 0.45)
        } else {
            mix(th.surface, color, 0.22)
        };
        let fill = if past {
            mix(th.surface, fill, 0.6)
        } else {
            fill
        };
        let text = if past { th.text_dim } else { th.text };
        let width = 1.0 / cols as f32;
        let left = col as f32 * width;
        let time = time_range(occurrence.start, occurrence.end, tz);
        let short = height < 34.0;
        let title = if data.title.is_empty() {
            tr!("calendar-no-title")
        } else {
            data.title.clone()
        };
        let open = occurrence.clone();
        div()
            .id(SharedString::from(format!(
                "event-{}-{}",
                occurrence.event.id, occurrence.start
            )))
            .absolute()
            .top(px(top + 1.0))
            .h(px((height - 2.0).max(18.0)))
            .left(gpui::relative(left))
            .w(gpui::relative(width))
            .pr(px(4.0))
            .child(
                div()
                    .size_full()
                    .overflow_hidden()
                    .px(px(6.0))
                    .py(px(if short { 1.0 } else { 3.0 }))
                    .rounded(px(6.0))
                    .when(!declined && !tentative, |d| d.bg(rgba(fill)))
                    .when(tentative && !declined, |d| {
                        d.bg(rgba(fade(fill, 0.5)))
                            .border_1()
                            .border_color(rgba(color))
                    })
                    .when(declined, |d| {
                        d.bg(rgba(th.surface))
                            .border_1()
                            .border_color(rgba(fade(color, 0.6)))
                    })
                    .border_l_4()
                    .border_color(rgba(color))
                    .cursor_pointer()
                    .hover(|s| s.shadow(crate::widgets::elevation(th, 1.0)))
                    .text_color(rgba(text))
                    .text_size(px(12.0))
                    .line_height(px(15.0))
                    .when(short, |d| {
                        d.truncate().child(tr!(
                            "calendar-short-event",
                            title = title.clone(),
                            time = format::time(civil(occurrence.start, tz))
                        ))
                    })
                    .when(!short, |d| {
                        d.child(
                            div()
                                .font_weight(FontWeight::MEDIUM)
                                .when(declined, |d| d.line_through())
                                .child(title),
                        )
                        .child(div().text_color(rgba(th.text_dim)).child(time))
                        .when(
                            !data.location.is_empty() && height > 60.0,
                            |d| {
                                d.child(
                                    div()
                                        .truncate()
                                        .text_color(rgba(th.text_dim))
                                        .child(data.location.clone()),
                                )
                            },
                        )
                    }),
            )
            .on_click(cx.listener(move |this, event: &ClickEvent, _, cx| {
                this.open_calendar_event(open.clone(), event.position(), cx)
            }))
            .into_any_element()
    }

    /// The Month view: six weeks of days with their events.
    fn render_month(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let (first, _) = self.calendar.days();
        let tz = self.tz.clone();
        let today = Zoned::now().with_time_zone(tz.clone()).date();
        let occurrences = self.shown_occurrences();
        let month = self.calendar.day.month();
        let head = (0..7).map(|ix| {
            let day = first.checked_add((ix as i64).days()).unwrap_or(first);
            let name = format::weekday(day.to_datetime(Time::midnight()));
            div()
                .flex_1()
                .pt(px(6.0))
                .flex()
                .justify_center()
                .text_size(px(11.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_dim))
                .child(name.to_uppercase())
        });
        let weeks = (0..6).map(|week| {
            let cells = (0..7).map(|ix| {
                let day = first.checked_add((week * 7 + ix).days()).unwrap_or(first);
                let start = midnight(day, &tz);
                let next = midnight(day.tomorrow().unwrap_or(day), &tz);
                let mine: Vec<&Occurrence> = occurrences
                    .iter()
                    .filter(|o| o.start < next && (o.end > start || o.start == start))
                    .collect();
                let more = mine.len().saturating_sub(3);
                let shown = if more > 0 { &mine[..2] } else { &mine[..] };
                let is_today = day == today;
                let in_month = day.month() == month;
                let lines = shown.iter().map(|occurrence| {
                    let color = self.event_color(occurrence);
                    let data = &occurrence.event.data;
                    let open = (*occurrence).clone();
                    let title = if data.title.is_empty() {
                        tr!("calendar-no-title")
                    } else {
                        data.title.clone()
                    };
                    let bar =
                        occurrence.all_day() || occurrence.end - occurrence.start >= 24 * 3600;
                    div()
                        .id(SharedString::from(format!(
                            "month-{}-{}-{}",
                            day, occurrence.event.id, occurrence.start
                        )))
                        .h(px(MONTH_LINE))
                        .mx(px(4.0))
                        .px(px(6.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(6.0))
                        .rounded(px(4.0))
                        .text_size(px(12.0))
                        .cursor_pointer()
                        .when(bar, |d| {
                            d.bg(rgba(color))
                                .text_color(rgba(0xffff_ffff))
                                .font_weight(FontWeight::MEDIUM)
                        })
                        .when(!bar, |d| {
                            d.text_color(rgba(th.text))
                                .hover(|s| s.bg(rgba(th.hover)))
                                .child(
                                    div()
                                        .flex_none()
                                        .size(px(8.0))
                                        .rounded_full()
                                        .bg(rgba(color)),
                                )
                                .child(
                                    div()
                                        .flex_none()
                                        .text_color(rgba(th.text_dim))
                                        .child(format::time(civil(occurrence.start, &tz))),
                                )
                        })
                        .child(div().truncate().child(title))
                        .on_click(cx.listener(move |this, event: &ClickEvent, _, cx| {
                            this.open_calendar_event(open.clone(), event.position(), cx)
                        }))
                });
                div()
                    .id(SharedString::from(format!("month-day-{day}")))
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .overflow_hidden()
                    .flex()
                    .flex_col()
                    .gap(px(1.0))
                    .when(ix > 0, |d| d.border_l_1().border_color(rgba(th.divider)))
                    .child(
                        div().flex().justify_center().pt(px(4.0)).child(
                            div()
                                .id(SharedString::from(format!("month-num-{day}")))
                                .size(px(24.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded_full()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::MEDIUM)
                                .cursor_pointer()
                                .when(is_today, |d| {
                                    d.bg(rgba(th.accent)).text_color(rgba(th.on_accent))
                                })
                                .when(!is_today, |d| {
                                    d.text_color(rgba(if in_month {
                                        th.text
                                    } else {
                                        th.text_faint
                                    }))
                                    .hover(|s| s.bg(rgba(th.hover)))
                                })
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.open_calendar_day(day, Some(CalView::Day), cx)
                                }))
                                .child(if day.day() == 1 {
                                    format::day_month(day.to_datetime(Time::midnight()))
                                } else {
                                    format::number(day.day() as u64)
                                }),
                        ),
                    )
                    .children(lines)
                    .when(more > 0, |d| {
                        d.child(
                            div()
                                .id(SharedString::from(format!("month-more-{day}")))
                                .h(px(MONTH_LINE))
                                .mx(px(4.0))
                                .px(px(6.0))
                                .flex()
                                .items_center()
                                .rounded(px(4.0))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgba(th.text_dim))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgba(th.hover)))
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.open_calendar_day(day, Some(CalView::Day), cx)
                                }))
                                .child(tr!("calendar-more", count = mine.len() - 2)),
                        )
                    })
            });
            div()
                .flex_1()
                .min_h_0()
                .flex()
                .flex_row()
                .border_b_1()
                .border_color(rgba(th.divider))
                .children(cells)
        });
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(div().flex_none().flex().flex_row().children(head))
            .children(weeks)
            .into_any_element()
    }

    /// The Schedule view: the coming days with events, as a list.
    fn render_schedule(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let tz = self.tz.clone();
        let today = Zoned::now().with_time_zone(tz.clone()).date();
        let occurrences = self.shown_occurrences();
        let mut days: Vec<(Date, Vec<Occurrence>)> = Vec::new();
        let (first, end) = self.calendar.days();
        for occurrence in occurrences {
            // A long event is listed on each of its days in range.
            let mut day = civil(occurrence.start, &tz).date().max(first);
            let last = civil((occurrence.end - 1).max(occurrence.start), &tz).date();
            while day <= last && day < end {
                match days.iter_mut().find(|(d, _)| *d == day) {
                    Some((_, list)) => list.push(occurrence.clone()),
                    None => days.push((day, vec![occurrence.clone()])),
                }
                day = match day.tomorrow() {
                    Ok(next) => next,
                    Err(_) => break,
                };
            }
        }
        days.sort_by_key(|(d, _)| *d);
        if days.is_empty() {
            return crate::widgets::placeholder(&tr!("calendar-schedule-empty"), th);
        }
        let rows = days.into_iter().map(|(day, list)| {
            let is_today = day == today;
            let events = list.into_iter().map(|occurrence| {
                let color = self.event_color(&occurrence);
                let data = &occurrence.event.data;
                let when = if occurrence.all_day() {
                    tr!("calendar-all-day")
                } else {
                    time_range(occurrence.start, occurrence.end, &tz)
                };
                let title = if data.title.is_empty() {
                    tr!("calendar-no-title")
                } else {
                    data.title.clone()
                };
                let open = occurrence.clone();
                div()
                    .id(SharedString::from(format!(
                        "schedule-{}-{}-{}",
                        day, occurrence.event.id, occurrence.start
                    )))
                    .h(px(40.0))
                    .px(px(12.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(16.0))
                    .rounded(px(8.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .child(
                        div()
                            .flex_none()
                            .size(px(12.0))
                            .rounded_full()
                            .bg(rgba(color)),
                    )
                    .child(
                        div()
                            .flex_none()
                            .w(px(150.0))
                            .text_size(px(13.0))
                            .text_color(rgba(th.text_dim))
                            .child(when),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.text))
                            .child(title),
                    )
                    .on_click(cx.listener(move |this, event: &ClickEvent, _, cx| {
                        this.open_calendar_event(open.clone(), event.position(), cx)
                    }))
            });
            div()
                .flex()
                .flex_row()
                .items_start()
                .py(px(8.0))
                .border_b_1()
                .border_color(rgba(th.divider))
                .child(
                    div()
                        .flex_none()
                        .w(px(200.0))
                        .h(px(40.0))
                        .pl(px(16.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(10.0))
                        .child(
                            div()
                                .size(px(32.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded_full()
                                .text_size(px(20.0))
                                .when(is_today, |d| {
                                    d.bg(rgba(th.accent)).text_color(rgba(th.on_accent))
                                })
                                .when(!is_today, |d| d.text_color(rgba(th.text)))
                                .child(format::number(day.day() as u64)),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgba(if is_today { th.accent } else { th.text_dim }))
                                .child(
                                    format!(
                                        "{}, {}",
                                        format::weekday(day.to_datetime(Time::midnight())),
                                        format::month_name(day.month())
                                    )
                                    .to_uppercase(),
                                ),
                        ),
                )
                .child(div().flex_1().min_w_0().flex().flex_col().children(events))
        });
        div()
            .id("calendar-schedule")
            .size_full()
            .overflow_y_scroll()
            .children(rows)
            .into_any_element()
    }

    fn open_calendar_event(
        &mut self,
        occurrence: Occurrence,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        cx.stop_propagation();
        self.calendar.open = Some(OpenEvent { occurrence, at });
        cx.notify();
    }

    /// The card of the event clicked: when, where, the call to join, who
    /// comes and what it says.
    fn render_event_card(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let open = self.calendar.open.as_ref()?;
        let occurrence = &open.occurrence;
        let data = &occurrence.event.data;
        let tz = &self.tz;
        let color = self.event_color(occurrence);
        let calendar = self
            .calendar
            .calendars
            .iter()
            .find(|c| c.id == occurrence.event.calendar_id);
        let start = civil(occurrence.start, tz);
        let when = if occurrence.all_day() {
            let last = civil((occurrence.end - 1).max(occurrence.start), tz);
            if last.date() == start.date() {
                format::day_month_year(start)
            } else {
                tr!(
                    "calendar-days-range",
                    first = format::day_month(start),
                    last = format::day_month_year(last)
                )
            }
        } else {
            tr!(
                "calendar-when",
                day = format::day_month_year(start),
                time = time_range(occurrence.start, occurrence.end, tz)
            )
        };
        let line = |name: &'static str, text: String| {
            div()
                .flex()
                .flex_row()
                .items_start()
                .gap(px(16.0))
                .child(
                    div()
                        .flex_none()
                        .pt(px(1.0))
                        .child(icon(name, th.text_dim, 20.0)),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .text_size(px(14.0))
                        .line_height(px(20.0))
                        .text_color(rgba(th.text))
                        .child(text),
                )
        };
        let guests = data.attendees.len();
        let answers = |status: &str| data.attendees.iter().filter(|a| a.status == status).count();
        let join = data.join_url.clone();
        let web = data.web_link.clone();
        let title = if data.title.is_empty() {
            tr!("calendar-no-title")
        } else {
            data.title.clone()
        };
        let mut card = raised(
            div()
                .id("event-card")
                .occlude()
                .w(px(CARD_WIDTH))
                .max_h(px(520.0))
                .overflow_y_scroll()
                .p(px(8.0))
                .pb(px(20.0))
                .flex()
                .flex_col()
                .gap(px(12.0)),
            th,
            15.0,
            3.0,
        )
        .child(
            div()
                .flex()
                .flex_row()
                .justify_end()
                .gap(px(4.0))
                .when(!web.is_empty(), |d| {
                    d.child(
                        icon_button("event-web", "open-external", 20.0, th)
                            .tooltip(tip(tr!("calendar-open-web"), th))
                            .on_click(move |_, _, cx| cx.open_url(&web)),
                    )
                })
                .child(
                    icon_button("event-close", "close", 20.0, th)
                        .tooltip(tip(tr!("calendar-close"), th))
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.calendar.open = None;
                            cx.notify();
                        })),
                ),
        )
        .child(
            div()
                .px(px(16.0))
                .flex()
                .flex_row()
                .items_start()
                .gap(px(16.0))
                .child(
                    div()
                        .flex_none()
                        .mt(px(6.0))
                        .size(px(16.0))
                        .rounded(px(4.0))
                        .bg(rgba(color)),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap(px(4.0))
                        .child(
                            div()
                                .text_size(px(22.0))
                                .line_height(px(28.0))
                                .text_color(rgba(th.text))
                                .when(data.status == EventStatus::Cancelled, |d| d.line_through())
                                .child(title),
                        )
                        .child(
                            div()
                                .text_size(px(14.0))
                                .text_color(rgba(th.text_dim))
                                .child(when),
                        )
                        .when(
                            !data.rrule.is_empty() || occurrence.series_start.is_some(),
                            |d| {
                                d.child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(rgba(th.text_faint))
                                        .child(tr!("calendar-repeats")),
                                )
                            },
                        ),
                ),
        );
        let body = div()
            .px(px(16.0))
            .flex()
            .flex_col()
            .gap(px(14.0))
            .when(!join.is_empty(), |d| {
                let url = join.clone();
                d.child(
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(16.0))
                        .items_center()
                        .child(icon("event", th.text_dim, 20.0))
                        .child(
                            crate::widgets::filled_button("event-join", tr!("calendar-join"), th)
                                .on_click(move |_, _, cx| cx.open_url(&url)),
                        ),
                )
            })
            .when(!data.location.is_empty(), |d| {
                d.child(line("pin", data.location.clone()))
            })
            .when(guests > 0, |d| {
                let mut text = tr!("calendar-guests", count = guests);
                let yes = answers("accepted");
                let maybe = answers("tentative");
                let no = answers("declined");
                let waiting = answers("needs_action");
                text.push_str(&format!(
                    "\n{}",
                    tr!(
                        "calendar-guest-answers",
                        yes = yes,
                        maybe = maybe,
                        no = no,
                        waiting = waiting
                    )
                ));
                for attendee in data.attendees.iter().take(12) {
                    let name = if attendee.name.is_empty() {
                        attendee.email.clone()
                    } else {
                        attendee.name.clone()
                    };
                    let name = if attendee.organizer {
                        tr!("calendar-organizer-name", name = name)
                    } else {
                        name
                    };
                    text.push_str(&format!("\n{name}"));
                }
                d.child(line("people", text))
            })
            .when(!data.description.is_empty(), |d| {
                let text: String = data.description.chars().take(1200).collect();
                d.child(line("notes", text))
            })
            .when_some(calendar, |d, calendar| {
                d.child(line("calendar", calendar.name.clone()))
            });
        card = card.child(body);
        let at = open.at;
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .child(
                    deferred(
                        div()
                            .id("event-scrim")
                            .absolute()
                            .top(px(-2000.0))
                            .left(px(-4000.0))
                            .w(px(8000.0))
                            .h(px(6000.0))
                            .occlude()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.calendar.open = None;
                                    cx.notify();
                                }),
                            ),
                    )
                    .with_priority(3),
                )
                .child(
                    deferred(
                        anchored()
                            .position(at)
                            .snap_to_window_with_margin(px(8.0))
                            .child(
                                card.with_animation(
                                    ("event-card", occurrence.event.id as usize),
                                    Animation::new(std::time::Duration::from_millis(160))
                                        .with_easing(ease_out_quint()),
                                    |el, t| el.opacity(t).mt(px(-6.0 * (1.0 - t))),
                                ),
                            ),
                    )
                    .with_priority(4),
                )
                .into_any_element(),
        )
    }
}

/// "9:00 – 9:30 AM": the start and end times of an event.
fn time_range(start: i64, end: i64, tz: &TimeZone) -> String {
    tr!(
        "calendar-time-range",
        start = format::time(civil(start, tz)),
        end = format::time(civil(end, tz))
    )
}

/// The page's spinner while the first read runs.
fn loading(th: &Theme) -> AnyElement {
    crate::widgets::placeholder(&tr!("calendar-loading"), th)
}

/// Places the timed events of a day: `(event, top, height, column,
/// columns)`, side by side where they overlap, as Google does.
fn lay_out(
    events: &[&Occurrence],
    day_start: i64,
    day_end: i64,
) -> Vec<(Occurrence, f32, f32, usize, usize)> {
    let mut sorted: Vec<&Occurrence> = events.to_vec();
    sorted.sort_by(|a, b| a.start.cmp(&b.start).then(b.end.cmp(&a.end)));
    let y = |t: i64| ((t.clamp(day_start, day_end) - day_start) as f32 / 3600.0) * HOUR_HEIGHT;
    let mut out = Vec::new();
    let mut cluster: Vec<(&Occurrence, usize)> = Vec::new();
    let mut columns_end: Vec<i64> = Vec::new();
    let mut cluster_end = i64::MIN;
    let flush = |cluster: &mut Vec<(&Occurrence, usize)>,
                 columns: usize,
                 out: &mut Vec<(Occurrence, f32, f32, usize, usize)>| {
        for (occurrence, col) in cluster.drain(..) {
            // A short event still gets room for its title.
            let end = occurrence.end.max(occurrence.start + 15 * 60);
            let top = y(occurrence.start);
            out.push((
                occurrence.clone(),
                top,
                (y(end) - top).max(HOUR_HEIGHT / 4.0),
                col,
                columns.max(1),
            ));
        }
    };
    for occurrence in sorted {
        let end = occurrence.end.max(occurrence.start + 15 * 60);
        if occurrence.start >= cluster_end && !cluster.is_empty() {
            let n = columns_end.len();
            flush(&mut cluster, n, &mut out);
            columns_end.clear();
        }
        let col = match columns_end.iter().position(|&e| e <= occurrence.start) {
            Some(col) => {
                columns_end[col] = end;
                col
            }
            None => {
                columns_end.push(end);
                columns_end.len() - 1
            }
        };
        cluster.push((occurrence, col));
        cluster_end = if cluster.len() == 1 {
            end
        } else {
            cluster_end.max(end)
        };
    }
    let n = columns_end.len();
    flush(&mut cluster, n, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use katna_store::calendar::{EventData, StoredEvent};
    use std::sync::Arc;

    fn occurrence(id: i64, start: i64, end: i64) -> Occurrence {
        Occurrence {
            event: Arc::new(StoredEvent {
                id,
                calendar_id: 1,
                data: EventData::default(),
            }),
            start,
            end,
            series_start: None,
        }
    }

    #[test]
    fn overlapping_events_share_the_width() {
        let h = 3600;
        let a = occurrence(1, 9 * h, 10 * h);
        let b = occurrence(2, 9 * h + 1800, 11 * h);
        let c = occurrence(3, 10 * h, 10 * h + 1800);
        let d = occurrence(4, 14 * h, 15 * h);
        let placed = lay_out(&[&a, &b, &c, &d], 0, 24 * h);
        let cols: Vec<(i64, usize, usize)> = placed
            .iter()
            .map(|(o, _, _, col, cols)| (o.event.id, *col, *cols))
            .collect();
        assert_eq!(cols, [(1, 0, 2), (2, 1, 2), (3, 0, 2), (4, 0, 1)]);
        assert_eq!(placed[0].1, 9.0 * HOUR_HEIGHT);
        assert_eq!(placed[0].2, HOUR_HEIGHT);
    }

    #[test]
    fn weeks_start_on_the_language_s_first_day() {
        let day = jiff::civil::date(2026, 9, 30);
        let start = CalendarPage::week_start(day);
        assert_eq!(start.weekday(), format::first_weekday());
        assert!(start <= day && (day - start).get_days() < 7);
    }

    #[test]
    fn colors_parse() {
        assert_eq!(parse_color("#039be5"), Some(0x039b_e5ff));
        assert_eq!(parse_color("039be5"), None);
        assert_eq!(parse_color("#fff"), None);
    }
}
