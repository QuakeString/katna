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
    Animation, AnimationExt, AnyElement, App, ClickEvent, Context, Div, FocusHandle, FontWeight,
    HighlightStyle, InteractiveText, KeyBinding, MouseButton, Pixels, Point, ScrollHandle,
    SharedString, StyledText, Task, UnderlineStyle, Window, anchored, canvas, deferred, div,
    ease_out_quint, prelude::*, rgba,
};
use jiff::civil::{Date, DateTime, Time};
use jiff::tz::TimeZone;
use jiff::{ToSpan, Zoned};
use katna_core::Paths;
use katna_core::config::CalendarDensity;
use katna_dav::Occurrence;
use katna_i18n::{format, tr};
use katna_store::calendar::{Calendar, EventKind, EventStatus};
use katna_store::{Mode, Store};
use katna_ui::px;

use super::account_status::{AccountStatus, Of, Say};
use super::event_edit::{Draft, ScopeAsk, kind_icon, kind_label};
use super::{MailWindow, Menu, MenuKey};
use menu::CalTarget;

mod birthdays;
mod description;
mod free;
mod from_mail;
pub(super) mod menu;
mod search;
mod sets;
mod side_menu;
mod tasks;
mod year_peek;
use crate::theme::{Theme, fade, mix};
use crate::widgets::{
    FocusRing, icon, icon_button, menu, menu_item, menu_item_icon, outlined_button, raised, tip,
};

gpui::actions!(
    katna_calendar,
    [
        CalendarToday,
        CalendarNext,
        CalendarPrevious,
        CalendarDayView,
        CalendarWeekView,
        CalendarMonthView,
        CalendarYearView,
        CalendarScheduleView,
        CalendarCustomView,
        CalendarCloseEvent,
        CalendarCreateEvent,
        CalendarEditEvent,
        CalendarDeleteEvent,
    ]
);

/// The key context of the Calendar page.
pub(super) const CALENDAR_CONTEXT: &str = "CalendarPage";

/// Google Calendar's keys, in the Calendar page only, and not while
/// typing in a field on it (a calendar set's name).
pub(super) fn bindings() -> Vec<KeyBinding> {
    let keys = format!(
        "{CALENDAR_CONTEXT} && !{}",
        katna_ui::text_input::KEY_CONTEXT
    );
    let c = Some(keys.as_str());
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
        KeyBinding::new("y", CalendarYearView, c),
        KeyBinding::new("5", CalendarYearView, c),
        KeyBinding::new("a", CalendarScheduleView, c),
        KeyBinding::new("4", CalendarScheduleView, c),
        KeyBinding::new("x", CalendarCustomView, c),
        KeyBinding::new("6", CalendarCustomView, c),
        KeyBinding::new("escape", CalendarCloseEvent, c),
        KeyBinding::new("c", CalendarCreateEvent, c),
        KeyBinding::new("e", CalendarEditEvent, c),
        KeyBinding::new("delete", CalendarDeleteEvent, c),
        KeyBinding::new("backspace", CalendarDeleteEvent, c),
    ]
}

/// The side column: the small month and the calendars.
const SIDE_WIDTH: f32 = 256.0;
/// The page's own bar.
const BAR_HEIGHT: f32 = 64.0;
/// An hour of the Day and Week grids.
const HOUR_HEIGHT: f32 = 48.0;
/// An hour of Day and Week in the compact density.
const COMPACT_HOUR: f32 = 36.0;
/// The responsive density's hours: a twelfth of the grid, within these.
const RESPONSIVE_HOURS: (f32, f32) = (40.0, 72.0);
/// The hours down the left of the grid.
const GUTTER: f32 = 64.0;
/// A line of the whole-day row.
const ALL_DAY_LINE: f32 = 24.0;
/// Below this width the calendar bar folds its view buttons into a menu.
const COMPACT_BAR: f32 = 1000.0;
/// A line of an event in a month's day.
const MONTH_LINE: f32 = 22.0;
/// The small month's days.
const MINI_DAY: f32 = 28.0;
/// A day in Year's small months.
const YEAR_DAY: f32 = 32.0;
/// How many days Schedule lists at once.
const SCHEDULE_DAYS: i64 = 60;
/// The event card's width.
const CARD_WIDTH: f32 = 400.0;
/// Where the grid opens: a little before 8 in the morning, as Google does.
const MORNING_HOUR: f32 = 7.5;

/// The second time zones the options menu offers, besides one set in
/// the settings file.
const ZONES: [&str; 16] = [
    "UTC",
    "America/Los_Angeles",
    "America/Denver",
    "America/Chicago",
    "America/New_York",
    "America/Sao_Paulo",
    "Europe/London",
    "Europe/Berlin",
    "Africa/Cairo",
    "Asia/Dubai",
    "Asia/Kolkata",
    "Asia/Dhaka",
    "Asia/Singapore",
    "Asia/Shanghai",
    "Asia/Tokyo",
    "Australia/Sydney",
];

/// How the page shows the days.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CalView {
    Day,
    Week,
    Month,
    Year,
    Schedule,
    /// The custom view: a few days from the one picked.
    Days,
}

impl CalView {
    const ALL: [Self; 6] = [
        Self::Day,
        Self::Week,
        Self::Month,
        Self::Year,
        Self::Schedule,
        Self::Days,
    ];

    /// The view's name; `days` is how many the custom view shows.
    fn label(self, days: u8) -> String {
        match self {
            Self::Day => tr!("calendar-view-day"),
            Self::Week => tr!("calendar-view-week"),
            Self::Month => tr!("calendar-view-month"),
            Self::Year => tr!("calendar-view-year"),
            Self::Schedule => tr!("calendar-view-schedule"),
            Self::Days => tr!("calendar-view-days", count = days),
        }
    }
}

/// What was read for the dates on show.
struct Loaded {
    occurrences: Rc<Vec<Occurrence>>,
}

/// An open event's card.
pub(super) struct OpenEvent {
    pub(super) occurrence: Occurrence,
    pub(super) at: Point<Pixels>,
}

/// How far the pointer goes, pressed, before an event moves with it.
const DRAG_START: f32 = 4.0;

/// An event being dragged to another time or day, or its end to another
/// time.
pub(super) struct EventDrag {
    pub(super) occurrence: Occurrence,
    origin: Point<Pixels>,
    pub(super) resize: bool,
    /// How far it has gone, in minutes (by quarter hours) and days.
    pub(super) minutes: i64,
    pub(super) days: i64,
    pub(super) moved: bool,
}

impl EventDrag {
    fn is(&self, occurrence: &Occurrence) -> bool {
        self.occurrence.event.id == occurrence.event.id && self.occurrence.start == occurrence.start
    }
}

/// The state of the Calendar page.
pub(super) struct CalendarPage {
    view: CalView,
    /// How many days the custom view shows.
    custom_days: u8,
    /// The day the views are on.
    day: Date,
    /// The month the small month shows, by its first day.
    mini: Date,
    loaded: Option<Loaded>,
    pub(super) calendars: Rc<Vec<Calendar>>,
    /// Calendars unticked here and not yet read back as hidden.
    hidden: HashSet<i64>,
    /// Accounts whose calendars are folded away in the side column.
    folded: HashSet<Option<i64>>,
    loading: bool,
    error: Option<String>,
    task: Option<Task<()>>,
    grid_scroll: ScrollHandle,
    /// Where the bar and the views are, as last drawn: how wide they are.
    main_bounds: ScrollHandle,
    /// The grid was scrolled to the morning since it last appeared.
    scrolled: bool,
    /// How tall an hour of the grid is, as last drawn.
    pub(super) hour: f32,
    pub(super) open: Option<OpenEvent>,
    /// An event being added or changed.
    pub(super) draft: Option<Draft>,
    pub(super) drag: Option<EventDrag>,
    /// A task being dragged to another day or time.
    task_drag: Option<tasks::TaskDrag>,
    /// Which occurrences of a repeating event a change is for, being asked.
    pub(super) ask: Option<ScopeAsk>,
    pub(super) focus: FocusHandle,
    /// A new calendar set's name being typed.
    naming_set: Option<sets::Naming>,
    /// A calendar's name being typed in the side panel.
    naming: Option<side_menu::Naming>,
    /// The top bar's search box, finding events.
    pub(super) search: search::Search,
    /// Where each account's calendar sync stands.
    pub(super) accounts: AccountStatus,
    /// The Year view's day popover.
    peek: year_peek::YearPeek,
}

impl CalendarPage {
    pub(super) fn new(custom_days: u8, cx: &mut App) -> Self {
        let today = Zoned::now().date();
        Self {
            view: CalView::Week,
            custom_days,
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
            main_bounds: ScrollHandle::new(),
            scrolled: false,
            hour: HOUR_HEIGHT,
            open: None,
            draft: None,
            drag: None,
            task_drag: None,
            ask: None,
            focus: cx.focus_handle(),
            naming_set: None,
            naming: None,
            search: search::Search::default(),
            accounts: AccountStatus::default(),
            peek: year_peek::YearPeek::default(),
        }
    }

    /// The day new events start on: today when it is on show, else the
    /// first day shown.
    pub(super) fn view_day(&self) -> Date {
        let today = Zoned::now().date();
        let (first, end) = self.days();
        if (first..end).contains(&today) {
            today
        } else if matches!(self.view, CalView::Month | CalView::Year) {
            self.day
        } else {
            first
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
            CalView::Year => {
                let first = self.day.first_of_year();
                (first, first.checked_add(1.year()).unwrap_or(first))
            }
            CalView::Schedule => (
                self.day,
                self.day
                    .checked_add(SCHEDULE_DAYS.days())
                    .unwrap_or(self.day),
            ),
            CalView::Days => (
                self.day,
                self.day
                    .checked_add(i64::from(self.custom_days).days())
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
            CalView::Year => by.years(),
            CalView::Schedule => (SCHEDULE_DAYS * by).days(),
            CalView::Days => (i64::from(self.custom_days) * by).days(),
        };
        if let Ok(day) = self.day.checked_add(span) {
            self.day = match self.view {
                CalView::Month => day.first_of_month(),
                CalView::Year => day.first_of_year(),
                _ => day,
            };
            self.mini = self.day.first_of_month();
        }
    }
}

/// A menu item with a check before it when it is the one chosen.
fn checked_item(
    id: impl Into<gpui::ElementId>,
    label: &str,
    on: bool,
    th: &Theme,
) -> gpui::Stateful<Div> {
    div()
        .id(id)
        .flex_none()
        .h(px(32.0))
        .pl(px(12.0))
        .pr(px(24.0))
        .flex()
        .items_center()
        .gap(px(8.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .menu_key(th)
        .child(
            div()
                .size(px(18.0))
                .flex_none()
                .when(on, |d| d.child(icon("check", th.accent, 18.0))),
        )
        .child(label.to_owned())
}

/// A UTC offset as the grid and the zone list show it: "GMT-4",
/// "GMT+5:30", "GMT".
fn gmt(zoned: &Zoned) -> String {
    let seconds = zoned.offset().seconds();
    if seconds == 0 {
        return "GMT".to_owned();
    }
    let sign = if seconds < 0 { '-' } else { '+' };
    let (hours, minutes) = (seconds.abs() / 3600, seconds.abs() % 3600 / 60);
    if minutes == 0 {
        format!("GMT{sign}{hours}")
    } else {
        format!("GMT{sign}{hours}:{minutes:02}")
    }
}

/// A time zone as people know it: its city ("New York"), or UTC.
fn zone_name(iana: &str) -> String {
    iana.rsplit('/').next().unwrap_or(iana).replace('_', " ")
}

pub(super) fn midnight(day: Date, tz: &TimeZone) -> i64 {
    day.to_zoned(tz.clone())
        .map_or(0, |z| z.timestamp().as_second())
}

pub(super) fn civil(seconds: i64, tz: &TimeZone) -> DateTime {
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

/// A calendar's color.
/// A calendar's name; the calendar the daemon makes on this computer has
/// none of its own.
pub(super) fn calendar_name(calendar: &Calendar) -> String {
    if calendar.name.is_empty() && calendar.account.is_none() {
        tr!("calendar-local")
    } else {
        calendar.name.clone()
    }
}

pub(super) fn calendar_color(calendar: &Calendar) -> u32 {
    parse_color(&calendar.color).unwrap_or(DEFAULT_COLOR)
}

/// The color of an event: its own, else its calendar's.
pub(super) fn event_color(calendars: &[Calendar], occurrence: &Occurrence) -> u32 {
    parse_color(&occurrence.event.data.color)
        .or_else(|| {
            calendars
                .iter()
                .find(|c| c.id == occurrence.event.calendar_id)
                .and_then(|c| parse_color(&c.color))
        })
        .unwrap_or(DEFAULT_COLOR)
}

/// Google's blue, for calendars without a color.
const DEFAULT_COLOR: u32 = 0x039b_e5ff;

/// Reads the calendars and the occurrences in `from..to`, with saved
/// contacts' birthdays when `birthdays` are shown.
pub(super) fn read(
    paths: &Paths,
    from: i64,
    to: i64,
    tz: &TimeZone,
    birthdays: bool,
) -> Result<(Vec<Calendar>, Vec<Occurrence>), String> {
    let store = Store::open(paths, Mode::ReadOnly).map_err(|err| err.to_string())?;
    let mut calendars = store.calendars().map_err(|err| err.to_string())?;
    let rows = store
        .event_rows_in_range(from, to)
        .map_err(|err| err.to_string())?;
    let mut occurrences = katna_dav::occurrences(rows, from, to, tz);
    birthdays::add_birthdays(
        &store,
        birthdays,
        from,
        to,
        tz,
        &mut calendars,
        &mut occurrences,
    );
    Ok((calendars, occurrences))
}

/// Reads the calendars alone, for an event made before the page has read
/// them.
pub(super) fn read_calendars(paths: &Paths) -> Vec<Calendar> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.calendars())
        .unwrap_or_else(|err| {
            tracing::warn!(%err, "reading the calendars failed");
            Vec::new()
        })
}

impl MailWindow {
    /// Reads what the Calendar page shows, when it is on show.
    pub(super) fn load_calendar(&mut self, cx: &mut Context<Self>) {
        let (first, end) = self.calendar.days();
        let tz = self.tz.clone();
        let (from, to) = (midnight(first, &tz), midnight(end, &tz));
        let paths = self.paths.clone();
        let birthdays = !self.config.contacts.hide_birthdays;
        self.calendar.loading = true;
        self.load_account_status(Of::Calendar, cx);
        self.calendar.task = Some(cx.spawn(async move |this, cx| {
            let read = cx
                .background_executor()
                .spawn(async move { read(&paths, from, to, &tz, birthdays) })
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
        self.cancel_calendar_drags();
        // Moving on puts search results away; Esc brings them back.
        self.calendar.search.away = true;
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.cancel_calendar_set(window, cx) || self.cancel_calendar_naming(window, cx) {
            return;
        }
        self.cancel_calendar_drags();
        let card = self.calendar.open.take().is_some();
        // Esc goes back to search results put away.
        let search = &mut self.calendar.search;
        let results = !search.query.trim().is_empty() && search.away;
        if results {
            search.away = false;
        }
        if card || results {
            cx.notify();
        } else {
            cx.propagate();
        }
    }

    pub(super) fn open_calendar_day(
        &mut self,
        day: Date,
        view: Option<CalView>,
        cx: &mut Context<Self>,
    ) {
        self.calendar.day = day;
        self.calendar.mini = day.first_of_month();
        if let Some(view) = view {
            self.calendar.view = view;
        }
        self.calendar.scrolled = false;
        self.calendar_moved(cx);
    }

    fn toggle_calendar(&mut self, id: i64, cx: &mut Context<Self>) {
        if id == birthdays::BIRTHDAYS {
            self.toggle_birthdays(cx);
            return;
        }
        let shown = !self.calendar_hidden(id);
        if shown {
            self.calendar.hidden.insert(id);
        } else {
            self.calendar.hidden.remove(&id);
        }
        self.set_calendar_shown(id, !shown, cx);
        cx.notify();
    }

    pub(super) fn calendar_hidden(&self, id: i64) -> bool {
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
        event_color(&self.calendar.calendars, occurrence)
    }

    /// Shows the Calendar page's Day view on `day`.
    /// How tall an hour of Day and Week is in the chosen density.
    fn hour_height(&self, cx: &mut Context<Self>) -> f32 {
        match self.config.calendar.density {
            CalendarDensity::Compact => COMPACT_HOUR,
            CalendarDensity::Comfortable => HOUR_HEIGHT,
            CalendarDensity::Responsive => {
                let seen = katna_ui::unpx(self.calendar.grid_scroll.bounds().size.height);
                if seen <= 0.0 {
                    // Not laid out yet: again once it is.
                    let this = cx.entity().downgrade();
                    cx.defer(move |cx| {
                        let _ = this.update(cx, |_, cx| cx.notify());
                    });
                    return self.calendar.hour;
                }
                (seen / 12.0).clamp(RESPONSIVE_HOURS.0, RESPONSIVE_HOURS.1)
            }
        }
    }

    /// The second time zone of Day and Week, when one is chosen.
    fn second_zone(&self) -> Option<TimeZone> {
        let name = self.config.calendar.second_time_zone.trim();
        (!name.is_empty())
            .then(|| TimeZone::get(name).ok())
            .flatten()
    }

    /// The width of the hours at the left of Day and Week.
    fn gutter(&self) -> f32 {
        if self.second_zone().is_some() {
            2.0 * GUTTER
        } else {
            GUTTER
        }
    }

    fn set_calendar_density(&mut self, density: CalendarDensity, cx: &mut Context<Self>) {
        self.config.calendar.density = density;
        self.save_config();
        self.menu = None;
        cx.notify();
    }

    fn set_second_zone(&mut self, name: String, cx: &mut Context<Self>) {
        self.config.calendar.second_time_zone = name;
        self.save_config();
        self.menu = None;
        cx.notify();
    }

    /// Shows the custom view with `days` days.
    fn set_custom_days(&mut self, days: u8, cx: &mut Context<Self>) {
        self.config.calendar.custom_days = days;
        self.save_config();
        self.calendar.custom_days = days;
        self.menu = None;
        self.calendar.view = CalView::Days;
        self.calendar.scrolled = false;
        self.calendar_moved(cx);
    }

    /// The calendar bar's options: the density, how many days the custom
    /// view shows, and the second time zone (which opens the list of
    /// zones).
    pub(super) fn calendar_options_menu(&self, th: &Theme, cx: &mut Context<Self>) -> Div {
        let density = self.config.calendar.density;
        let heading = |text: String| {
            div()
                .px(px(16.0))
                .pt(px(4.0))
                .pb(px(4.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_dim))
                .child(text)
        };
        let densities = [
            (CalendarDensity::Responsive, "calendar-density-responsive"),
            (CalendarDensity::Comfortable, "calendar-density-comfortable"),
            (CalendarDensity::Compact, "calendar-density-compact"),
        ]
        .into_iter()
        .map(|(choice, label)| {
            checked_item(
                ("calendar-density", choice as usize),
                &tr!(label),
                choice == density,
                th,
            )
            .on_click(cx.listener(move |this, _, _, cx| this.set_calendar_density(choice, cx)))
        });
        let custom = self.calendar.custom_days;
        let counts = (2..=7u8).map(|days| {
            let on = days == custom;
            div()
                .id(("calendar-custom-days", usize::from(days)))
                .size(px(30.0))
                .flex()
                .flex_none()
                .items_center()
                .justify_center()
                .rounded_full()
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .cursor_pointer()
                .when(on, |d| {
                    d.bg(rgba(th.nav_selected))
                        .text_color(rgba(th.nav_selected_text))
                })
                .when(!on, |d| {
                    d.text_color(rgba(th.text)).hover(|s| s.bg(rgba(th.hover)))
                })
                .on_click(cx.listener(move |this, _, _, cx| this.set_custom_days(days, cx)))
                .child(format::number(u64::from(days)))
        });
        let zone = self.second_zone().map_or_else(
            || tr!("calendar-zone-none"),
            |_| zone_name(&self.config.calendar.second_time_zone),
        );
        menu(th)
            .min_w(px(240.0))
            .child(heading(tr!("calendar-density")))
            .children(densities)
            .child(div().my(px(8.0)).h(px(1.0)).bg(rgba(th.divider)))
            .child(heading(tr!("calendar-custom-days")))
            .child(
                div()
                    .px(px(12.0))
                    .pb(px(4.0))
                    .flex()
                    .gap(px(4.0))
                    .children(counts),
            )
            .child(div().my(px(8.0)).h(px(1.0)).bg(rgba(th.divider)))
            .child(heading(tr!("calendar-second-zone")))
            .child(
                menu_item("calendar-second-zone", &zone, th)
                    .justify_between()
                    .gap(px(16.0))
                    .child(icon("chevron-right", th.text_dim, 18.0))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.menu = Some(Menu::CalendarZones);
                        cx.notify();
                    })),
            )
            .child(div().my(px(8.0)).h(px(1.0)).bg(rgba(th.divider)))
            .child(
                menu_item_icon(
                    "calendar-share-free",
                    "event",
                    &tr!("calendar-share-free"),
                    th,
                )
                .on_click(cx.listener(|this, _, window, cx| this.share_free_times(window, cx))),
            )
    }

    /// The list of second time zones, with their offsets now.
    pub(super) fn calendar_zones_menu(&self, th: &Theme, cx: &mut Context<Self>) -> Div {
        let chosen = self.config.calendar.second_time_zone.trim().to_owned();
        let now = jiff::Timestamp::now();
        let mut names: Vec<String> = ZONES.iter().map(|z| (*z).to_owned()).collect();
        if !chosen.is_empty() && !names.contains(&chosen) && TimeZone::get(&chosen).is_ok() {
            names.insert(0, chosen.clone());
        }
        let none = checked_item(
            "calendar-zone-none",
            &tr!("calendar-zone-none"),
            chosen.is_empty(),
            th,
        )
        .on_click(cx.listener(|this, _, _, cx| this.set_second_zone(String::new(), cx)));
        let zones = names.into_iter().enumerate().filter_map(|(ix, name)| {
            let zone = TimeZone::get(&name).ok()?;
            let label = if name == "UTC" {
                name.clone()
            } else {
                tr!(
                    "calendar-zone",
                    zone = zone_name(&name),
                    offset = gmt(&now.to_zoned(zone))
                )
            };
            let on = name == chosen;
            Some(
                checked_item(("calendar-zone", ix), &label, on, th).on_click(
                    cx.listener(move |this, _, _, cx| this.set_second_zone(name.clone(), cx)),
                ),
            )
        });
        menu(th).min_w(px(240.0)).child(
            div()
                .id("calendar-zones")
                .max_h(px(420.0))
                .overflow_y_scroll()
                .flex()
                .flex_col()
                .child(none)
                .children(zones),
        )
    }

    /// Shows `occurrence`'s day in the Calendar with its card open, from
    /// outside the Calendar page (the contact panel's meetings).
    pub(super) fn open_calendar_occurrence(
        &mut self,
        occurrence: Occurrence,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let day = civil(occurrence.start, &self.tz).date();
        self.show_page(super::apps::App::Calendar, window, cx);
        self.open_calendar_day(day, Some(CalView::Day), cx);
        let size = window.viewport_size();
        let at = gpui::point(size.width / 2.0, size.height / 3.0);
        self.open_calendar_event(occurrence, at, cx);
    }

    /// The views, in the narrow bar's menu.
    pub(super) fn calendar_views_menu(&self, th: &Theme, cx: &mut Context<Self>) -> Div {
        let page = &self.calendar;
        menu(th)
            .min_w(px(180.0))
            .children(CalView::ALL.into_iter().map(|view| {
                checked_item(
                    ("calendar-views-item", view as usize),
                    &view.label(page.custom_days),
                    view == page.view,
                    th,
                )
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.menu = None;
                    this.set_calendar_view(view, cx);
                    cx.notify();
                }))
            }))
    }

    pub(super) fn open_calendar_on(&mut self, day: Date, cx: &mut Context<Self>) {
        self.open_calendar_day(day, Some(CalView::Day), cx);
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
        } else if self.calendar.search.showing() {
            self.render_calendar_search(th, cx)
        } else {
            match self.calendar.view {
                CalView::Day | CalView::Week | CalView::Days => self.render_time_grid(th, cx),
                CalView::Month => self.render_month(th, cx),
                CalView::Year => self.render_year(th, cx),
                CalView::Schedule => self.render_schedule(th, cx),
            }
        };
        if self.event_editor_open() {
            return self.render_event_editor(th, cx);
        }
        let side = self.render_calendar_side(th, cx);
        let side = self.page_side(side, SIDE_WIDTH, false, th, cx);
        let page = div()
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
            .on_action(cx.listener(|this, _: &CalendarYearView, _, cx| {
                this.set_calendar_view(CalView::Year, cx)
            }))
            .on_action(cx.listener(|this, _: &CalendarScheduleView, _, cx| {
                this.set_calendar_view(CalView::Schedule, cx)
            }))
            .on_action(cx.listener(|this, _: &CalendarCustomView, _, cx| {
                this.set_calendar_view(CalView::Days, cx)
            }))
            .on_action(cx.listener(Self::close_calendar_event))
            .on_action(cx.listener(|this, _: &CalendarCreateEvent, window, cx| {
                this.create_event_key(window, cx)
            }))
            .on_action(cx.listener(|this, _: &CalendarEditEvent, window, cx| {
                this.edit_open_event(window, cx)
            }))
            .on_action(
                cx.listener(|this, _: &CalendarDeleteEvent, _, cx| this.delete_open_event(cx)),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    if this.calendar.draft.is_none() {
                        window.focus(&this.calendar.focus, cx);
                    }
                }),
            )
            .relative()
            .size_full()
            .flex()
            .flex_row()
            .children(side.docked)
            .child(
                div()
                    .id("calendar-main")
                    .track_scroll(&self.calendar.main_bounds)
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .flex()
                    .flex_col()
                    .child(self.render_calendar_bar(th, cx))
                    .child(div().flex_1().min_h_0().child(main)),
            )
            .children(self.render_event_card(th, cx))
            .children(self.render_year_peek(th, cx))
            .children(side.drawer);
        div()
            .relative()
            .size_full()
            .child(page)
            .children(self.render_event_draft(th, cx))
            .children(self.render_task_details(th, cx))
            .into_any_element()
    }

    fn render_calendar_bar(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let page = &self.calendar;
        let (first, end) = page.days();
        let last = end.yesterday().unwrap_or(end);
        let title = match page.view {
            CalView::Day => format::day_month_year(page.day.to_datetime(Time::midnight())),
            CalView::Month => format::month_year(page.day),
            CalView::Year => format::year(page.day.year()),
            CalView::Week | CalView::Schedule | CalView::Days => {
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
            CalView::Year => ("calendar-previous-year", "calendar-next-year"),
            CalView::Schedule | CalView::Days => {
                ("calendar-previous-period", "calendar-next-period")
            }
        };
        // Too narrow for a button per view (with Today, the arrows and
        // the title beside them): one button with the views in a menu, as
        // Google Calendar does.
        let width = katna_ui::unpx(page.main_bounds.bounds().size.width);
        let compact = width > 0.0 && width < COMPACT_BAR;
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
                .focus_ring(th)
                .on_click(cx.listener(move |this, _, _, cx| this.set_calendar_view(view, cx)))
                .child(view.label(page.custom_days))
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
                    .focus_ring(th)
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
                            .focus_ring(th)
                            .tooltip(tip(tr!(previous), th))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.calendar_previous(&CalendarPrevious, window, cx)
                            })),
                    )
                    .child(
                        icon_button("calendar-next", "chevron-right", 22.0, th)
                            .focus_ring(th)
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
            .when(!compact, |d| {
                d.child(
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
            })
            .when(compact, |d| {
                d.child(
                    self.with_menu(
                        div()
                            .id("calendar-views")
                            .flex_none()
                            .h(px(36.0))
                            .pl(px(16.0))
                            .pr(px(10.0))
                            .flex()
                            .items_center()
                            .gap(px(4.0))
                            .rounded_full()
                            .border_1()
                            .border_color(rgba(fade(th.text_faint, 0.5)))
                            .text_size(px(13.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.text))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .focus_ring(th)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.toggle_menu(Menu::CalendarViews, cx)
                            }))
                            .child(page.view.label(page.custom_days))
                            .child(icon("drop-down", th.text_dim, 20.0)),
                        Menu::CalendarViews,
                        th,
                        cx,
                    ),
                )
            })
            .child(
                self.with_menu(
                    icon_button("calendar-options", "tune", 22.0, th)
                        .focus_ring(th)
                        .when(self.menu.is_none(), |d| {
                            d.tooltip(tip(tr!("calendar-options"), th))
                        })
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.toggle_menu(Menu::CalendarOptions, cx)
                        })),
                    self.menu
                        .filter(|m| *m == Menu::CalendarZones)
                        .unwrap_or(Menu::CalendarOptions),
                    th,
                    cx,
                ),
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
            .when(!self.calendar.calendars.is_empty(), |d| {
                d.child(self.render_calendar_sets(th, cx))
            })
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
                let on_show = matches!(page.view, CalView::Day | CalView::Week | CalView::Days)
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
        // Every account shows, also one whose calendars did not come, with
        // the reason under it.
        let mut groups: Vec<(Option<i64>, Vec<&Calendar>)> = self
            .accounts
            .iter()
            .map(|a| (Some(a.id.0), Vec::new()))
            .collect();
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
            let note = match account {
                Some(id) if calendars.is_empty() && !folded => {
                    Some(self.render_account_status(Of::Calendar, id, th, cx))
                }
                _ => None,
            };
            let naming = self.calendar.naming.as_ref();
            let rows = calendars.into_iter().map(|calendar| {
                let id = calendar.id;
                if let Some(naming) = naming.filter(|n| n.what == side_menu::NameFor::Rename(id)) {
                    return self.render_calendar_naming(naming, th, cx);
                }
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
                    .rounded_full()
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .focus_ring(th)
                    .on_click(cx.listener(move |this, _, _, cx| this.toggle_calendar(id, cx)))
                    .on_mouse_down(
                        MouseButton::Right,
                        self.calendar_menu_on(CalTarget::Calendar(id), cx),
                    )
                    .child(crate::widgets::checkbox_tinted(
                        ("calendar-box", id as usize),
                        shown,
                        color,
                        th,
                    ))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .text_size(px(14.0))
                            .text_color(rgba(th.text))
                            .child(calendar_name(calendar)),
                    )
                    .into_any_element()
            });
            let new_here = naming
                .filter(|n| n.what == side_menu::NameFor::New(account))
                .map(|n| self.render_calendar_naming(n, th, cx));
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
                        .rounded_full()
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(th.hover)))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if !this.calendar.folded.remove(&account) {
                                this.calendar.folded.insert(account);
                            }
                            cx.notify();
                        }))
                        .on_mouse_down(
                            MouseButton::Right,
                            self.calendar_menu_on(CalTarget::Account(account), cx),
                        )
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
                .children(note)
                .when(!folded, |d| d.children(rows).children(new_here))
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

        // The density's hour; the grid keeps the same time at its top when
        // it changes.
        let hour_height = self.hour_height(cx);
        if hour_height != self.calendar.hour {
            let offset = self.calendar.grid_scroll.offset();
            let by = hour_height / self.calendar.hour;
            self.calendar
                .grid_scroll
                .set_offset(gpui::point(offset.x, offset.y * by));
            self.calendar.hour = hour_height;
        }
        let second = self.second_zone();
        let gutter = self.gutter();

        if !self.calendar.scrolled {
            self.calendar.scrolled = true;
            let hour = if days.contains(&today) {
                (now.hour() as f32 - 1.5).clamp(0.0, 16.0)
            } else {
                MORNING_HOUR
            };
            self.calendar
                .grid_scroll
                .set_offset(gpui::point(px(0.0), -px(hour_height * hour)));
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
        let (task_lanes, task_bars) = self.render_all_day_tasks(&days, lanes.len(), th, cx);
        let all_day_height = ((lanes.len() + task_lanes).max(1) as f32) * ALL_DAY_LINE + 4.0;
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
                            .child(div().min_w_0().truncate().child(title)),
                    )
                    .on_mouse_down(
                        MouseButton::Right,
                        self.calendar_menu_on(CalTarget::Event(Box::new(occurrence.clone())), cx),
                    )
                    .on_click(cx.listener(move |this, event: &ClickEvent, _, cx| {
                        this.open_calendar_event(open.clone(), event.position(), cx)
                    }))
            })
            .collect::<Vec<_>>();

        // The hours down the left, and in the second time zone beside them.
        let hour_labels = |zone: Option<&TimeZone>| {
            (1..24)
                .map(|hour| {
                    let at =
                        today.to_datetime(Time::new(hour, 0, 0, 0).unwrap_or(Time::midnight()));
                    let at = match zone {
                        Some(zone) => at
                            .to_zoned(tz.clone())
                            .map(|z| z.with_time_zone(zone.clone()).datetime())
                            .unwrap_or(at),
                        None => at,
                    };
                    div()
                        .absolute()
                        .top(px(hour as f32 * hour_height - 7.0))
                        .right(px(8.0))
                        .text_size(px(10.0))
                        .text_color(rgba(th.text_faint))
                        .child(format::time(at))
                })
                .collect::<Vec<_>>()
        };
        let hours = hour_labels(None);
        let second_hours = second.as_ref().map(|zone| hour_labels(Some(zone)));
        let lines = (1..24).map(|hour| {
            div()
                .absolute()
                .top(px(hour as f32 * hour_height))
                .left_0()
                .right_0()
                .h(px(1.0))
                .bg(rgba(th.divider))
        });
        let columns = days
            .iter()
            .enumerate()
            .map(|(ix, &day)| {
                let start = midnight(day, &tz);
                let next = midnight(day.tomorrow().unwrap_or(day), &tz);
                let mine: Vec<&Occurrence> = timed
                    .iter()
                    .filter(|o| {
                        o.start < next && (o.end > start || (o.end == o.start && o.start >= start))
                    })
                    .collect();
                let busy: Vec<(i64, i64)> = mine
                    .iter()
                    .map(|o| ((o.start - start) / 60, (o.end - start) / 60))
                    .collect();
                let day_tasks = self.render_timed_tasks(day, &busy, th, cx);
                let placed = lay_out(&mine, start, next, hour_height);
                let is_today = day == today;
                let now_y = ((now.timestamp().as_second() - start) as f32 / 3600.0) * hour_height;
                let scroll = self.calendar.grid_scroll.clone();
                let placeholder = self
                    .calendar
                    .draft
                    .as_ref()
                    .filter(|d| d.placeholder_on(day))
                    .map(|d| d.placeholder(cx));
                div()
                    .id(("grid-day", ix))
                    .relative()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .when(ix > 0, |d| d.border_l_1().border_color(rgba(th.divider)))
                    .on_mouse_down(MouseButton::Right, {
                        let scroll = scroll.clone();
                        cx.listener(move |this, event: &gpui::MouseDownEvent, _, cx| {
                            let top = scroll.bounds().top() + scroll.offset().y;
                            let y = katna_ui::unpx(event.position.y - top);
                            let time = Some(super::event_edit::time_at(y, hour_height));
                            this.open_calendar_menu(
                                CalTarget::Slot { day, time },
                                event.position,
                                cx,
                            );
                        })
                    })
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, event: &gpui::MouseDownEvent, window, cx| {
                            let top = scroll.bounds().top() + scroll.offset().y;
                            let y = katna_ui::unpx(event.position.y - top);
                            let time = super::event_edit::time_at(y, hour_height);
                            this.start_new_event(
                                day,
                                Some(time),
                                false,
                                event.position,
                                window,
                                cx,
                            );
                        }),
                    )
                    .children(placeholder.map(|(start, end, title)| {
                        let top = (start / 60.0) * hour_height;
                        let height = ((end - start) / 60.0 * hour_height).max(20.0);
                        div()
                            .absolute()
                            .top(px(top))
                            .left(px(2.0))
                            .right(px(8.0))
                            .h(px(height - 2.0))
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(px(6.0))
                            .bg(rgba(th.accent))
                            .shadow(crate::widgets::elevation(th, 2.0))
                            .text_color(rgba(th.on_accent))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::MEDIUM)
                            .child(title)
                    }))
                    .children(
                        placed
                            .into_iter()
                            .map(|(occurrence, top, height, col, cols)| {
                                self.render_timed_event(
                                    occurrence, top, height, col, cols, &tz, th, cx,
                                )
                            }),
                    )
                    .children(day_tasks)
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
            })
            .collect::<Vec<_>>();
        let zones = std::iter::once(gmt(&now)).chain(
            second
                .as_ref()
                .map(|zone| gmt(&now.with_time_zone(zone.clone()))),
        );
        let zones = zones
            .rev()
            .map(|label| div().flex_1().pr(px(8.0)).flex().justify_end().child(label));
        div()
            .id("calendar-days")
            .on_mouse_move(cx.listener(|this, event: &gpui::MouseMoveEvent, _, cx| {
                if this.calendar.task_drag.is_none() {
                    return;
                }
                if event.pressed_button != Some(MouseButton::Left) {
                    this.cancel_calendar_drags();
                    cx.notify();
                    return;
                }
                this.drag_task_to(event.position, cx);
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| this.drop_dragged_task(cx)),
            )
            .size_full()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex_none()
                    .flex()
                    .flex_row()
                    .child(div().flex_none().w(px(gutter)))
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
                            .w(px(gutter))
                            .flex()
                            .items_end()
                            .text_size(px(10.0))
                            .text_color(rgba(th.text_faint))
                            .children(zones),
                    )
                    .child(
                        div()
                            .relative()
                            .flex_1()
                            .h(px(all_day_height))
                            .on_mouse_down(
                                MouseButton::Right,
                                cx.listener(|this, event: &gpui::MouseDownEvent, _, cx| {
                                    if let Some((day, _)) = this.grid_point(event.position) {
                                        let slot = CalTarget::Slot { day, time: None };
                                        this.open_calendar_menu(slot, event.position, cx);
                                    }
                                }),
                            )
                            .children(bars)
                            .children(task_bars),
                    ),
            )
            .child(
                div()
                    .id("calendar-grid")
                    .on_mouse_move(cx.listener(|this, event: &gpui::MouseMoveEvent, _, cx| {
                        if this.calendar.drag.is_none() {
                            return;
                        }
                        // The button came up where the grid didn't hear it.
                        if event.pressed_button != Some(MouseButton::Left) {
                            this.cancel_calendar_drags();
                            cx.notify();
                            return;
                        }
                        this.drag_event_to(event.position, cx);
                    }))
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| this.drop_dragged_event(cx)),
                    )
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .track_scroll(&self.calendar.grid_scroll)
                    .child(
                        div()
                            .relative()
                            .h(px(24.0 * hour_height))
                            .flex()
                            .flex_row()
                            .children(second_hours.map(|hours| {
                                div()
                                    .relative()
                                    .flex_none()
                                    .w(px(GUTTER))
                                    .h_full()
                                    .children(hours)
                            }))
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
        let mut left = col as f32 * width;
        let (mut top, mut height) = (top, height);
        let (mut start, mut end) = (occurrence.start, occurrence.end);
        let dragged = self.calendar.drag.as_ref().filter(|d| d.is(&occurrence));
        if let Some(drag) = dragged {
            let by = drag.minutes as f32 / 60.0 * self.calendar.hour;
            if drag.resize {
                height = (height + by).max(self.calendar.hour / 4.0);
                end = (end + drag.minutes * 60).max(start + 15 * 60);
            } else {
                top += by;
                left += drag.days as f32;
                start += drag.minutes * 60 + drag.days * 24 * 3600;
                end += drag.minutes * 60 + drag.days * 24 * 3600;
            }
        }
        let editable = self.can_edit(&occurrence);
        let time = time_range(start, end, tz);
        let short = height < 34.0;
        let title = if data.title.is_empty() {
            tr!("calendar-no-title")
        } else {
            data.title.clone()
        };
        let open = occurrence.clone();
        let block = div()
            .id(SharedString::from(format!(
                "event-{}-{}",
                occurrence.event.id, occurrence.start
            )))
            .on_mouse_down(MouseButton::Left, {
                let occurrence = occurrence.clone();
                cx.listener(move |this, event: &gpui::MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    if editable {
                        this.start_event_drag(occurrence.clone(), event.position, false);
                    }
                })
            })
            .on_mouse_down(
                MouseButton::Right,
                self.calendar_menu_on(CalTarget::Event(Box::new(occurrence.clone())), cx),
            )
            .when(dragged.is_some(), |d| d.opacity(0.85))
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
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(4.0))
                                .children(kind_icon(data.kind).map(|name| icon(name, text, 12.0)))
                                .child(
                                    div()
                                        .min_w_0()
                                        .truncate()
                                        .font_weight(FontWeight::MEDIUM)
                                        .when(declined, |d| d.line_through())
                                        .child(title),
                                ),
                        )
                        .child(div().truncate().text_color(rgba(th.text_dim)).child(time))
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
            .when(editable && height >= 30.0, |d| {
                let occurrence = occurrence.clone();
                d.child(
                    div()
                        .id("resize")
                        .absolute()
                        .bottom_0()
                        .left_0()
                        .right(px(4.0))
                        .h(px(6.0))
                        .cursor(gpui::CursorStyle::ResizeUpDown)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, event: &gpui::MouseDownEvent, _, cx| {
                                cx.stop_propagation();
                                this.start_event_drag(occurrence.clone(), event.position, true);
                            }),
                        ),
                )
            })
            .on_click(cx.listener(move |this, event: &ClickEvent, _, cx| {
                // The end of a drag, not a click.
                if this.calendar.drag.as_ref().is_some_and(|d| d.moved) {
                    return;
                }
                this.open_calendar_event(open.clone(), event.position(), cx)
            }));
        if dragged.is_some() {
            // Over the events it passes.
            return deferred(block.shadow(crate::widgets::elevation(th, 3.0)))
                .with_priority(1)
                .into_any_element();
        }
        block.into_any_element()
    }

    /// Puts down an event or task picked up and not dropped, where it
    /// was: nothing is saved. Returns whether one was.
    pub(super) fn cancel_calendar_drags(&mut self) -> bool {
        let event = self.calendar.drag.take().is_some();
        let task = self.calendar.task_drag.take().is_some();
        event || task
    }

    fn start_event_drag(&mut self, occurrence: Occurrence, at: Point<Pixels>, resize: bool) {
        self.calendar.drag = Some(EventDrag {
            occurrence,
            origin: at,
            resize,
            minutes: 0,
            days: 0,
            moved: false,
        });
    }

    /// Follows the pointer while an event is dragged: quarter hours up and
    /// down, days left and right.
    fn drag_event_to(&mut self, at: Point<Pixels>, cx: &mut Context<Self>) {
        let (first, end) = self.calendar.days();
        let count = (end - first).get_days().max(1) as f32;
        let width = katna_ui::unpx(self.calendar.grid_scroll.bounds().size.width) - self.gutter();
        let Some(drag) = &mut self.calendar.drag else {
            return;
        };
        let dy = katna_ui::unpx(at.y - drag.origin.y);
        let dx = katna_ui::unpx(at.x - drag.origin.x);
        // A press that wobbles is still a click.
        if !drag.moved && dx.hypot(dy) < DRAG_START {
            return;
        }
        let minutes = ((dy / self.calendar.hour * 60.0 / 15.0).round() as i64) * 15;
        let days = if drag.resize || width <= 0.0 {
            0
        } else {
            (dx / (width / count)).round() as i64
        };
        if (minutes, days) != (drag.minutes, drag.days) {
            drag.minutes = minutes;
            drag.days = days;
            drag.moved = true;
            cx.notify();
        }
    }

    /// The Month view: six weeks of days with their events.
    /// Year: the twelve months side by side, as small months; a dot
    /// under days with events, and a day opens in Day.
    fn render_year(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let tz = self.tz.clone();
        let today = Zoned::now().with_time_zone(tz.clone()).date();
        let year = self.calendar.day.first_of_year();
        let mut busy: HashMap<Date, u32> = HashMap::new();
        for occurrence in self.shown_occurrences() {
            let first = civil(occurrence.start, &tz).date();
            let last = civil((occurrence.end - 1).max(occurrence.start), &tz).date();
            let color = self.event_color(&occurrence);
            let mut day = first;
            while day <= last {
                busy.entry(day).or_insert(color);
                match day.tomorrow() {
                    Ok(next) => day = next,
                    Err(_) => break,
                }
            }
        }
        // Tasks too, in the accent, on days without events.
        for (task, _) in self.dated_tasks() {
            if let Some((day, _)) = self.task_place(task)
                && day.year() == year.year()
            {
                busy.entry(day).or_insert(th.accent);
            }
        }
        let phone = self.layout.shape.size == super::layout::Size::Phone;
        // Each day's cell, painted after the popover reads them: kept
        // from frame to frame, each rewritten when painted.
        let cells = self.calendar.peek.cells.clone();
        let head = format::weekdays_short();
        let months = (0..12)
            .filter_map(|m| year.checked_add(m.months()).ok())
            .map(|month| {
                let first = CalendarPage::week_start(month);
                let head = head.iter().map(|(_, name)| {
                    div()
                        .w(px(YEAR_DAY))
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
                    div().flex().flex_row().children((0..7).map(|ix| {
                        let day = first.checked_add((week * 7 + ix).days()).unwrap_or(first);
                        let cell = div().w(px(YEAR_DAY)).h(px(YEAR_DAY));
                        if day.month() != month.month() || day.year() != month.year() {
                            return cell.into_any_element();
                        }
                        let is_today = day == today;
                        let dot = busy.get(&day).copied();
                        cell.id(SharedString::from(format!("year-{day}")))
                            .relative()
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .on_mouse_down(
                                MouseButton::Right,
                                self.calendar_menu_on(CalTarget::Slot { day, time: None }, cx),
                            )
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if phone && this.year_day_tap(day, cx) {
                                    return;
                                }
                                this.close_year_peek(cx);
                                this.open_calendar_day(day, Some(CalView::Day), cx)
                            }))
                            .when(dot.is_some() && !phone, |d| {
                                d.on_hover(cx.listener(move |this, hovered: &bool, _, cx| {
                                    this.year_day_hover(day, *hovered, cx)
                                }))
                            })
                            .when(dot.is_some(), |d| {
                                let cells = cells.clone();
                                d.child(
                                    canvas(
                                        move |bounds, window, _| {
                                            cells
                                                .borrow_mut()
                                                .insert(day, (bounds, window.viewport_size()));
                                        },
                                        |_, _, _, _| {},
                                    )
                                    .absolute()
                                    .top_0()
                                    .left_0()
                                    .size_full(),
                                )
                            })
                            .child(
                                div()
                                    .size(px(YEAR_DAY - 4.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded_full()
                                    .text_size(px(12.0))
                                    .when(is_today, |d| {
                                        d.bg(rgba(th.accent))
                                            .text_color(rgba(th.on_accent))
                                            .font_weight(FontWeight::BOLD)
                                    })
                                    .when(!is_today, |d| {
                                        d.text_color(rgba(th.text)).hover(|s| s.bg(rgba(th.hover)))
                                    })
                                    .child(format::number(day.day() as u64)),
                            )
                            .when_some(dot.filter(|_| !is_today), |d, color| {
                                d.child(
                                    div()
                                        .absolute()
                                        .bottom(px(1.0))
                                        .left(px(YEAR_DAY / 2.0 - 2.0))
                                        .size(px(4.0))
                                        .rounded_full()
                                        .bg(rgba(color)),
                                )
                            })
                            .into_any_element()
                    }))
                });
                div()
                    .id(SharedString::from(format!("year-month-{month}")))
                    .flex_none()
                    .p(px(12.0))
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .id(SharedString::from(format!("year-title-{month}")))
                            .pl(px(8.0))
                            .pb(px(8.0))
                            .text_size(px(15.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(
                                if month.year() == today.year() && month.month() == today.month() {
                                    th.accent
                                } else {
                                    th.text
                                },
                            ))
                            .cursor_pointer()
                            .hover(|s| s.underline())
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.open_calendar_day(month, Some(CalView::Month), cx)
                            }))
                            .child(format::month_name(month.month())),
                    )
                    .child(div().flex().flex_row().children(head))
                    .children(weeks)
            });
        div()
            .id("calendar-year")
            .size_full()
            .overflow_y_scroll()
            // The popover belongs where its day was.
            .on_scroll_wheel(cx.listener(|this, _, _, cx| this.close_year_peek(cx)))
            .p(px(16.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .justify_center()
                    .gap(px(8.0))
                    .children(months),
            )
            .into_any_element()
    }

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
                let mut day_tasks = self.tasks_on(day);
                // Three lines, or two and "N more".
                let total = mine.len() + day_tasks.len();
                let room = if total > 3 { 2 } else { 3 };
                let shown = &mine[..mine.len().min(room)];
                day_tasks.truncate(room - shown.len());
                let more = total - shown.len() - day_tasks.len();
                let task_lines = self.render_month_tasks(day_tasks, day, th, cx);
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
                        .overflow_hidden()
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
                        .child(div().min_w_0().truncate().child(title))
                        .on_mouse_down(
                            MouseButton::Right,
                            self.calendar_menu_on(
                                CalTarget::Event(Box::new((*occurrence).clone())),
                                cx,
                            ),
                        )
                        .on_click(cx.listener(move |this, event: &ClickEvent, _, cx| {
                            this.open_calendar_event(open.clone(), event.position(), cx)
                        }))
                });
                div()
                    .id(SharedString::from(format!("month-day-{day}")))
                    .on_mouse_down(
                        MouseButton::Right,
                        self.calendar_menu_on(CalTarget::Slot { day, time: None }, cx),
                    )
                    .on_click(cx.listener(move |this, event: &ClickEvent, window, cx| {
                        this.start_new_event(day, None, true, event.position(), window, cx)
                    }))
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
                                // A circle round the number; a pill round
                                // the 1st's "01 Oct", so it holds it all.
                                .h(px(24.0))
                                .min_w(px(24.0))
                                .when(day.day() == 1, |d| d.px(px(8.0)))
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
                                    cx.stop_propagation();
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
                    .children(task_lines)
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
                                    cx.stop_propagation();
                                    this.open_calendar_day(day, Some(CalView::Day), cx)
                                }))
                                .child(tr!("calendar-more", count = more)),
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
        // Days with only tasks due.
        let mut day = first;
        while day < end {
            if !days.iter().any(|(d, _)| *d == day) && !self.tasks_on(day).is_empty() {
                days.push((day, Vec::new()));
            }
            day = match day.tomorrow() {
                Ok(next) => next,
                Err(_) => break,
            };
        }
        days.sort_by_key(|(d, _)| *d);
        if days.is_empty() {
            return crate::widgets::placeholder(&tr!("calendar-schedule-empty"), th);
        }
        let rows = days.into_iter().map(|(day, list)| {
            let events = list.into_iter().map(|occurrence| {
                let open = occurrence.clone();
                let menu =
                    self.calendar_menu_on(CalTarget::Event(Box::new(occurrence.clone())), cx);
                self.schedule_event(&day.to_string(), occurrence, th)
                    .on_mouse_down(MouseButton::Right, menu)
                    .on_click(cx.listener(move |this, event: &ClickEvent, _, cx| {
                        this.open_calendar_event(open.clone(), event.position(), cx)
                    }))
            });
            schedule_day(day, day == today, false, th).child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .children(events)
                    .children(self.render_schedule_tasks(day, th, cx)),
            )
        });
        div()
            .id("calendar-schedule")
            .size_full()
            .overflow_y_scroll()
            .children(rows)
            .into_any_element()
    }

    /// One event's line in Schedule and in search results: its color, when
    /// and title; a click opens its card.
    pub(super) fn schedule_event(
        &self,
        key: &str,
        occurrence: Occurrence,
        th: &Theme,
    ) -> gpui::Stateful<Div> {
        let tz = &self.tz;
        let color = self.event_color(&occurrence);
        let data = &occurrence.event.data;
        let when = if occurrence.all_day() {
            tr!("calendar-all-day")
        } else {
            time_range(occurrence.start, occurrence.end, tz)
        };
        let title = if data.title.is_empty() {
            tr!("calendar-no-title")
        } else {
            data.title.clone()
        };
        let id = SharedString::from(format!(
            "schedule-{key}-{}-{}",
            occurrence.event.id, occurrence.start
        ));
        div()
            .id(id)
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
    }

    fn open_calendar_event(
        &mut self,
        occurrence: Occurrence,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        cx.stop_propagation();
        // The press that opened it picked the event up; it isn't dragged.
        self.cancel_calendar_drags();
        self.calendar.open = Some(OpenEvent { occurrence, at });
        // Its card lists the event's meeting notes.
        self.notes_page(cx);
        cx.notify();
    }

    /// An event's description, tidied, its web addresses links.
    fn render_event_description(&self, text: &str, th: &Theme) -> AnyElement {
        let (text, links) = description::tidy(text);
        let style = HighlightStyle {
            color: Some(rgba(th.accent).into()),
            underline: Some(UnderlineStyle {
                thickness: px(1.0),
                ..UnderlineStyle::default()
            }),
            ..HighlightStyle::default()
        };
        let styled = StyledText::new(text)
            .with_highlights(links.iter().map(|(range, _)| (range.clone(), style)));
        let (ranges, urls): (Vec<_>, Vec<_>) = links.into_iter().unzip();
        let body =
            InteractiveText::new("event-description", styled).on_click(ranges, move |ix, _, cx| {
                if let Some(url) = urls.get(ix) {
                    cx.open_url(url);
                }
            });
        div()
            .flex()
            .flex_row()
            .items_start()
            .gap(px(16.0))
            .child(
                div()
                    .flex_none()
                    .pt(px(1.0))
                    .child(icon("notes", th.text_dim, 20.0)),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text))
                    .child(body),
            )
            .into_any_element()
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
        // The call to join: the event's own, else a link in its place or
        // description (a Teams invitation read from mail or CalDAV).
        let jitsi = katna_core::meeting::jitsi_host(&self.config.meetings.jitsi_server);
        let calls = katna_core::meeting::find(
            [data.join_url.as_str()]
                .into_iter()
                .filter(|l| !l.is_empty()),
            [data.location.as_str(), data.description.as_str()],
            jitsi.as_deref(),
        );
        let join = match calls.first() {
            Some((service, link)) => Some((Some(*service), link.clone())),
            None => (!data.join_url.is_empty()).then(|| (None, data.join_url.clone())),
        };
        let web = data.web_link.clone();
        // An event Gmail made from a mail opens that mail.
        let gmail = from_mail::gmail_link(&data.description).map(|link| (link, occurrence.clone()));
        let editable = self.can_edit(occurrence);
        let emails = !self.other_guests(occurrence).is_empty();
        // A saved contact's birthday opens their contact page.
        let birthday_of =
            (occurrence.event.calendar_id == birthdays::BIRTHDAYS).then_some(occurrence.event.id);
        // Running late, from an hour before the start to the end.
        let now = jiff::Timestamp::now().as_second();
        let late = emails
            && !occurrence.all_day()
            && data.status != EventStatus::Cancelled
            && now >= occurrence.start - 3600
            && now < occurrence.end;
        let title = if data.title.is_empty() {
            tr!("calendar-no-title")
        } else {
            data.title.clone()
        };
        let mut card = raised(
            div()
                .id("event-card")
                .occlude()
                .w(px(CARD_WIDTH.min(self.layout.shape.width - 16.0)))
                .max_h(px(520.0))
                .overflow_hidden()
                .p(px(8.0))
                .pb(px(12.0))
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
                .when(editable, |d| {
                    d.child(
                        icon_button("event-edit", "compose", 20.0, th)
                            .tooltip(tip(tr!("calendar-edit"), th))
                            .on_click(
                                cx.listener(|this, _, window, cx| this.edit_open_event(window, cx)),
                            ),
                    )
                    .child(
                        icon_button("event-delete", "trash", 20.0, th)
                            .tooltip(tip(tr!("calendar-delete"), th))
                            .on_click(cx.listener(|this, _, _, cx| this.delete_open_event(cx))),
                    )
                })
                .when(emails, |d| {
                    d.child(
                        icon_button("event-email", "mail", 20.0, th)
                            .tooltip(tip(tr!("calendar-email-guests"), th))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.email_guests(false, window, cx)
                            })),
                    )
                })
                .when(birthday_of.is_some(), |d| {
                    d.child(
                        icon_button("event-contact", "contacts", 20.0, th)
                            .tooltip(tip(tr!("calendar-open-contact"), th))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                if let Some(card) = birthday_of {
                                    this.open_birthday_contact(card, window, cx);
                                }
                            })),
                    )
                })
                .when_some(gmail, |d, (link, occurrence)| {
                    d.child(
                        icon_button("event-mail", "inbox", 20.0, th)
                            .tooltip(tip(tr!("calendar-open-mail"), th))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.open_event_mail(&occurrence, link.clone(), window, cx)
                            })),
                    )
                })
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
            .when_some(join, |d, (service, url)| {
                let label = match service {
                    Some(service) => tr!(
                        "calendar-join-with",
                        service = super::meeting::service_name(service)
                    ),
                    None => tr!("calendar-join"),
                };
                let shown = url
                    .split_once("://")
                    .map_or(url.as_str(), |(_, rest)| rest)
                    .to_owned();
                let open = url.clone();
                d.child(
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(16.0))
                        .items_center()
                        .child(icon("video", th.text_dim, 20.0))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .flex()
                                .flex_col()
                                .items_start()
                                .gap(px(4.0))
                                .child(
                                    crate::widgets::filled_button("event-join", label, th)
                                        .on_click(move |_, _, cx| cx.open_url(&open)),
                                )
                                .child(
                                    div()
                                        .w_full()
                                        .truncate()
                                        .text_size(px(12.0))
                                        .text_color(rgba(th.text_dim))
                                        .child(shown),
                                ),
                        ),
                )
            })
            .when(late, |d| {
                d.child(
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(16.0))
                        .items_center()
                        .child(icon("schedule", th.text_dim, 20.0))
                        .child(
                            outlined_button("event-late", tr!("calendar-running-late"), th)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.email_guests(true, window, cx)
                                })),
                        ),
                )
            })
            .when_some(
                kind_icon(data.kind).filter(|_| data.kind != EventKind::Birthday),
                |d, name| d.child(line(name, kind_label(data.kind))),
            )
            .when(!data.location.is_empty(), |d| {
                d.child(line("pin", data.location.clone()))
            })
            .when(guests > 0, |d| {
                let head = format!(
                    "{}\n{}",
                    tr!("calendar-guests", count = guests),
                    tr!(
                        "calendar-guest-answers",
                        yes = answers("accepted"),
                        maybe = answers("tentative"),
                        no = answers("declined"),
                        waiting = answers("needs_action")
                    )
                );
                let rows = data.attendees.iter().take(20).map(|attendee| {
                    let name = if attendee.name.is_empty() {
                        attendee.email.clone()
                    } else {
                        attendee.name.clone()
                    };
                    let mark = match attendee.status.as_str() {
                        "accepted" => Some(("check", 0x1e8e_3eff)),
                        "declined" => Some(("close", 0xd930_25ff)),
                        "tentative" => Some(("info", th.text_dim)),
                        _ => None,
                    };
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(10.0))
                        .child(
                            div()
                                .relative()
                                .flex_none()
                                .child(crate::widgets::avatar(&name, &attendee.email, 24.0))
                                .children(mark.map(|(name, color)| {
                                    div()
                                        .absolute()
                                        .bottom(px(-3.0))
                                        .right(px(-3.0))
                                        .size(px(13.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .rounded_full()
                                        .bg(rgba(th.menu))
                                        .child(icon(name, color, 11.0))
                                })),
                        )
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .flex()
                                .flex_col()
                                .text_size(px(13.0))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(6.0))
                                        .child(div().min_w_0().truncate().child(name))
                                        .children(self.muted_mark(&attendee.email, 14.0, th)),
                                )
                                .when(attendee.organizer || attendee.optional, |d| {
                                    d.child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(rgba(th.text_dim))
                                            .child(if attendee.organizer {
                                                tr!("calendar-organizer")
                                            } else {
                                                tr!("calendar-optional")
                                            }),
                                    )
                                }),
                        )
                });
                d.child(line("people", head)).child(
                    div()
                        .pl(px(36.0))
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .children(rows),
                )
            })
            .when(!data.description.is_empty(), |d| {
                d.child(self.render_event_description(&data.description, th))
            })
            // Nothing to take notes of on a birthday.
            .when(data.kind != EventKind::Birthday, |d| {
                d.child(self.render_event_notes(occurrence, th, cx))
            })
            .when_some(calendar, |d, calendar| {
                d.child(line("calendar", calendar_name(calendar)))
            });
        // Only the details scroll: the title above and the answers
        // below stay in sight.
        card = card.child(
            div()
                .id("event-card-body")
                .flex_1()
                .min_h_0()
                .overflow_y_scroll()
                .pb(px(8.0))
                .child(body),
        );
        let me = data.attendees.iter().find(|a| a.is_self);
        let invited = editable && me.is_some_and(|a| !a.organizer);
        let mine = if data.self_status.is_empty() {
            me.map(|a| a.status.clone()).unwrap_or_default()
        } else {
            data.self_status.clone()
        };
        if invited {
            let answer = |id: &'static str, label: String, status: &'static str| {
                let on = mine == status;
                div()
                    .id(id)
                    .h(px(32.0))
                    .px(px(16.0))
                    .flex()
                    .items_center()
                    .rounded_full()
                    .border_1()
                    .border_color(rgba(if on { th.accent } else { th.divider }))
                    .when(on, |d| d.bg(rgba(fade(th.accent, 0.12))))
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(if on { th.accent } else { th.text }))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .child(label)
                    .on_click(cx.listener(move |this, _, _, cx| this.respond_to_event(status, cx)))
            };
            card = card.child(
                div()
                    .flex_none()
                    .pt(px(12.0))
                    .px(px(16.0))
                    .border_t_1()
                    .border_color(rgba(th.divider))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(14.0))
                            .text_color(rgba(th.text_dim))
                            .child(tr!("calendar-going")),
                    )
                    .child(answer("answer-yes", tr!("calendar-answer-yes"), "accepted"))
                    .child(answer("answer-no", tr!("calendar-answer-no"), "declined"))
                    .child(answer(
                        "answer-maybe",
                        tr!("calendar-answer-maybe"),
                        "tentative",
                    )),
            );
        }
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
/// A day's row in Schedule and in search results: its number (ringed when
/// today), weekday and month, with its events beside. `year` adds the
/// year, for days in another year.
pub(super) fn schedule_day(day: Date, is_today: bool, year: bool, th: &Theme) -> Div {
    let date = day.to_datetime(Time::midnight());
    let month = if year {
        format::month_year(day)
    } else {
        format::month_name(day.month())
    };
    let label = format!("{}, {month}", format::weekday(date));
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
                        .child(label.to_uppercase()),
                ),
        )
}

pub(super) fn time_range(start: i64, end: i64, tz: &TimeZone) -> String {
    tr!(
        "calendar-time-range",
        start = format::time(civil(start, tz)),
        end = format::time(civil(end, tz))
    )
}

/// The page's spinner while the first read runs.
pub(super) fn loading(th: &Theme) -> AnyElement {
    crate::widgets::placeholder(&tr!("calendar-loading"), th)
}

/// Places the timed events of a day: `(event, top, height, column,
/// columns)`, side by side where they overlap, as Google does.
fn lay_out(
    events: &[&Occurrence],
    day_start: i64,
    day_end: i64,
    hour_height: f32,
) -> Vec<(Occurrence, f32, f32, usize, usize)> {
    let mut sorted: Vec<&Occurrence> = events.to_vec();
    sorted.sort_by(|a, b| a.start.cmp(&b.start).then(b.end.cmp(&a.end)));
    let y = |t: i64| ((t.clamp(day_start, day_end) - day_start) as f32 / 3600.0) * hour_height;
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
                (y(end) - top).max(hour_height / 4.0),
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

/// What the line under an account in the side list says on this page.
pub(super) fn say(say: Say<'_>) -> String {
    match say {
        Say::SignIn => tr!("calendar-account-sign-in"),
        Say::SignInRefused { provider } => {
            tr!("calendar-account-sign-in-refused", provider = provider)
        }
        Say::SignedIn { address } => tr!("calendar-account-signed-in", address = address),
        Say::Refused => tr!("calendar-account-refused"),
        Say::ChangePassword => tr!("calendar-account-change-password"),
        Say::ChangePasswordTooltip => tr!("calendar-account-change-password-tooltip"),
        Say::NotEnabled => tr!("calendar-account-not-enabled"),
        Say::Error { reason } => tr!("calendar-account-error", reason = reason),
        Say::Failed => tr!("calendar-account-failed"),
        Say::None => tr!("calendar-account-none"),
        Say::NoneWhy { reason } => tr!("calendar-account-none-why", reason = reason),
        Say::UseSignIn { provider } => tr!("calendar-account-use-sign-in", provider = provider),
        Say::SignInWith { provider } => tr!("calendar-account-sign-in-with", provider = provider),
        Say::Looking => tr!("calendar-account-looking"),
        Say::TryAgain => tr!("calendar-account-try-again"),
        Say::TryAgainTooltip => tr!("calendar-account-try-again-tooltip"),
        Say::Fixing => tr!("calendar-account-fixing"),
    }
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
        let placed = lay_out(&[&a, &b, &c, &d], 0, 24 * h, HOUR_HEIGHT);
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
