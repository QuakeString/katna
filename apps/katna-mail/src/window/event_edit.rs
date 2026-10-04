// SPDX-License-Identifier: GPL-3.0-or-later

//! Adding, changing and deleting events on the Calendar page, as Google
//! Calendar does: a click on an empty time (or C) opens a small card to
//! name the event and save it; More options (or E on an open event) opens
//! the whole editor with the dates, repeat, place, reminder, calendar and
//! notes. A change to a repeating event asks which occurrences it is for.
//! Every change goes to the daemon (`Pim1.EditEvent`), which writes it to
//! `pim.db` at once and then to the calendar's service; the snackbar
//! offers Undo, and Ctrl+Z works too.

use std::rc::Rc;

use gpui::{
    AnyElement, ClickEvent, Context, Entity, Focusable, FontWeight, MouseButton, Pixels, Point,
    ScrollHandle, SharedString, Subscription, Window, anchored, deferred, div, prelude::*, rgba,
};
use jiff::civil::{Date, DateTime, Time, Weekday};
use jiff::tz::TimeZone;
use jiff::{Timestamp, ToSpan, Zoned};
use katna_dav::Occurrence;
use katna_i18n::{format, tr};
use katna_store::calendar::{Attendee, Calendar, EditScope, EventChange, EventEdit, EventKind};
use katna_ui::px;
use katna_ui::text_area::TextArea;
use katna_ui::text_input::{InputEvent, TextInput};

use super::MailWindow;
use super::apps::App;
use super::calendar::civil;
use super::compose::schedule;
use crate::daemon::{self, Command};
use crate::data::EntryKey;
use crate::theme::{Theme, fade};
use crate::widgets::{filled_button, icon, icon_button, menu, radio, raised, tip};
use katna_core::quick_add::{self, Typed};

mod task_tab;

/// How long a new event lasts.
const NEW_EVENT_MINUTES: i64 = 60;
/// The steps of the time lists.
const TIME_STEP: i32 = 15;
/// The small card's width.
const QUICK_WIDTH: f32 = 480.0;
/// The reminders offered, in minutes before the start.
const REMINDERS: [i64; 7] = [0, 5, 10, 15, 30, 60, 24 * 60];

/// How an event repeats, as the editor offers it (Google's list).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Repeat {
    Never,
    Daily,
    /// On the start's weekday.
    Weekly,
    /// On the start's weekday of the same week of the month.
    Monthly,
    Yearly,
    /// Monday to Friday.
    Weekdays,
    /// A rule the editor does not offer, kept as it was.
    Kept(String),
}

fn weekday_code(day: Weekday) -> &'static str {
    match day {
        Weekday::Monday => "MO",
        Weekday::Tuesday => "TU",
        Weekday::Wednesday => "WE",
        Weekday::Thursday => "TH",
        Weekday::Friday => "FR",
        Weekday::Saturday => "SA",
        Weekday::Sunday => "SU",
    }
}

/// Which of its weekdays in the month `day` is: 1 to 4, or -1 for the
/// last (a fifth one is always the last).
fn week_of_month(day: Date) -> i8 {
    let nth = (day.day() - 1) / 7 + 1;
    if nth >= 5 { -1 } else { nth }
}

impl Repeat {
    const OFFERED: [Self; 6] = [
        Self::Never,
        Self::Daily,
        Self::Weekly,
        Self::Monthly,
        Self::Yearly,
        Self::Weekdays,
    ];

    /// The RRULE for an event starting on `day`.
    pub(super) fn rule(&self, day: Date) -> String {
        match self {
            Self::Never => String::new(),
            Self::Daily => "FREQ=DAILY".to_owned(),
            Self::Weekly => format!("FREQ=WEEKLY;BYDAY={}", weekday_code(day.weekday())),
            Self::Monthly => format!(
                "FREQ=MONTHLY;BYDAY={}{}",
                week_of_month(day),
                weekday_code(day.weekday())
            ),
            Self::Yearly => "FREQ=YEARLY".to_owned(),
            Self::Weekdays => "FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR".to_owned(),
            Self::Kept(rule) => rule.clone(),
        }
    }

    /// What `rule` is for an event starting on `day`.
    pub(super) fn of(rule: &str, day: Date) -> Self {
        if rule.is_empty() {
            return Self::Never;
        }
        Self::OFFERED
            .into_iter()
            .find(|repeat| repeat.rule(day) == rule)
            .unwrap_or_else(|| Self::Kept(rule.to_owned()))
    }

    fn label(&self, day: Date) -> String {
        let weekday = format::weekday(day.to_datetime(Time::midnight()));
        match self {
            Self::Never => tr!("calendar-repeat-never"),
            Self::Daily => tr!("calendar-repeat-daily"),
            Self::Weekly => tr!("calendar-repeat-weekly", weekday = weekday),
            Self::Monthly => tr!(
                "calendar-repeat-monthly",
                nth = i64::from(week_of_month(day)),
                weekday = weekday
            ),
            Self::Yearly => tr!(
                "calendar-repeat-yearly",
                day = format::day_month(day.to_datetime(Time::midnight()))
            ),
            Self::Weekdays => tr!("calendar-repeat-weekdays"),
            Self::Kept(_) => tr!("calendar-repeat-custom"),
        }
    }
}

fn reminder_label(minutes: Option<i64>) -> String {
    match minutes {
        None => tr!("calendar-reminder-none"),
        Some(0) => tr!("calendar-reminder-at-start"),
        Some(m) if m % (24 * 60) == 0 => tr!("calendar-reminder-days", count = m / (24 * 60)),
        Some(m) if m % 60 == 0 => tr!("calendar-reminder-hours", count = m / 60),
        Some(m) => tr!("calendar-reminder-minutes", count = m),
    }
}

/// A list the editor has open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Pick {
    StartDay,
    EndDay,
    StartTime,
    EndTime,
    Calendar,
    Repeat,
    Reminder,
    Busy,
    /// The Task tab's task list.
    TaskList,
}

/// An event being added or changed.
pub(super) struct Draft {
    /// The occurrence changed; `None` for a new event.
    editing: Option<Occurrence>,
    calendar: i64,
    title: Entity<TextInput>,
    location: Entity<TextInput>,
    notes: Entity<TextArea>,
    /// Where guests' addresses are typed; Enter adds one.
    guest: Entity<TextInput>,
    guests: Vec<Attendee>,
    /// A video call is to be added when saved.
    add_call: bool,
    /// `#rrggbb`, or empty for the calendar's: the event's own when
    /// editing or duplicating it.
    color: String,
    start_day: Date,
    end_day: Date,
    start_time: Time,
    end_time: Time,
    all_day: bool,
    repeat: Repeat,
    reminder: Option<i64>,
    busy: bool,
    /// The whole editor, not the small card.
    full: bool,
    /// Focus time, out of office or a working location; chosen for a new
    /// event only, as services fix it when the event is made.
    kind: EventKind,
    /// The Task tab is on: saving adds a task due at the start instead.
    task: bool,
    /// The task list a task goes in (`0`: the daemon's default list).
    task_list: i64,
    /// Where the small card points.
    at: Point<Pixels>,
    pick: Option<(Pick, Point<Pixels>)>,
    /// The month a date list shows.
    pick_month: Date,
    list_scroll: ScrollHandle,
    /// Typed quick add: the fields as they were before the new event's
    /// title named a day, time or place.
    quick: Option<QuickBase>,
    _events: Vec<Subscription>,
}

/// Reads a typed title (`katna_core::quick_add`) in the language in use.
pub(super) fn read_typed(text: &str, today: Date) -> Typed {
    let language = katna_i18n::current().language.tag.clone();
    quick_add::parse(text, today, quick_add::Words::for_language(&language))
}

/// [`Draft::quick`].
struct QuickBase {
    start_day: Date,
    end_day: Date,
    start_time: Time,
    end_time: Time,
    all_day: bool,
    repeat: Repeat,
    location: String,
    /// The place field holds the title's place.
    placed: bool,
    /// The repeat is the title's.
    repeated: bool,
}

impl Draft {
    /// Whether the event repeats (or is a changed occurrence of a series).
    fn repeating(&self) -> bool {
        self.editing
            .as_ref()
            .is_some_and(|o| !o.event.data.rrule.is_empty() || o.series_start.is_some())
    }

    /// The new event's block: its start and end in minutes into its day,
    /// and its title.
    pub(super) fn placeholder<T>(&self, cx: &Context<T>) -> (f32, f32, String) {
        let minutes = |t: Time| f32::from(t.hour()) * 60.0 + f32::from(t.minute());
        let start = minutes(self.start_time);
        let end = if self.task {
            start + task_tab::TASK_BLOCK_MINUTES
        } else if self.end_day == self.start_day {
            minutes(self.end_time)
        } else {
            24.0 * 60.0
        };
        let title = self.title.read(cx).text().trim().to_owned();
        // Without the day, time and place typed into it.
        let title = if self.quick.is_some() {
            read_typed(&title, Zoned::now().date()).title
        } else {
            title
        };
        let title = if title.is_empty() {
            tr!("calendar-no-title")
        } else {
            title
        };
        (start, end.max(start + 15.0), title)
    }

    /// Whether the new event's block shows on `day` in the grid.
    pub(super) fn placeholder_on(&self, day: Date) -> bool {
        self.editing.is_none() && !self.full && !self.all_day && self.start_day == day
    }
}

/// A question the page asks before a change: which occurrences.
pub(super) struct ScopeAsk {
    change: ScopeChange,
    scope: EditScope,
}

enum ScopeChange {
    Save(Occurrence, Box<EventEdit>, i64),
    Delete(Occurrence),
    /// An answer to an invitation.
    Respond(Occurrence, String),
}

/// What Undo does once the daemon answers with the event's ID.
enum UndoPlan {
    None,
    Fixed(EventChange),
    /// Deletes the event just added.
    DeleteAdded,
    /// Puts these fields back on the event just changed.
    ChangeBack(EventEdit),
}

/// The next whole hour after now, on `day`.
fn next_hour(day: Date, tz: &TimeZone) -> Time {
    let now = Zoned::now().with_time_zone(tz.clone());
    if now.date() == day {
        Time::new((now.hour() + 1).min(23), 0, 0, 0).unwrap_or(Time::midnight())
    } else {
        Time::constant(9, 0, 0, 0)
    }
}

fn add_minutes(time: Time, minutes: i64) -> (Time, i64) {
    let total = i64::from(time.hour()) * 60 + i64::from(time.minute()) + minutes;
    let days = total.div_euclid(24 * 60);
    let rest = total.rem_euclid(24 * 60);
    (
        Time::new((rest / 60) as i8, (rest % 60) as i8, 0, 0).unwrap_or(Time::midnight()),
        days,
    )
}

fn utc_date(seconds: i64) -> Date {
    Timestamp::from_second(seconds)
        .map(|t| t.to_zoned(TimeZone::UTC).date())
        .unwrap_or(Date::constant(1970, 1, 1))
}

fn utc_midnight(day: Date) -> i64 {
    day.to_datetime(Time::midnight())
        .to_zoned(TimeZone::UTC)
        .map(|z| z.timestamp().as_second())
        .unwrap_or(0)
}

impl MailWindow {
    /// The calendar new events go in: the main calendar of the account
    /// picked under the account picture, else the first account's main
    /// calendar the user can change, else any they can.
    fn default_calendar(&self) -> Option<&Calendar> {
        let calendars = &self.calendar.calendars;
        let main =
            |c: &&Calendar| c.is_primary && c.access.can_edit() && !self.calendar_hidden(c.id);
        let picked = self.account();
        calendars
            .iter()
            .find(|c| picked.is_some() && c.account == picked && main(c))
            .or_else(|| calendars.iter().find(main))
            .or_else(|| calendars.iter().find(|c| c.access.can_edit()))
    }

    fn new_draft(
        &mut self,
        editing: Option<Occurrence>,
        at: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Draft> {
        let calendar = match &editing {
            Some(o) => o.event.calendar_id,
            None => match self.default_calendar() {
                Some(c) => c.id,
                // Tasks can still be added.
                None if self.has_task_lists() => 0,
                None => {
                    self.show_snackbar(tr!("calendar-none-editable"), None, cx);
                    return None;
                }
            },
        };
        let accent: gpui::Hsla = rgba(self.theme(window).accent).into();
        let data = editing.as_ref().map(|o| o.event.data.clone());
        let title = cx.new(|cx| {
            let mut input = TextInput::new(tr!("calendar-add-title"), cx);
            input.set_accent(accent);
            if let Some(data) = &data {
                input.set_text(data.title.clone(), cx);
            }
            input
        });
        let location = cx.new(|cx| {
            let mut input = TextInput::new(tr!("calendar-add-location"), cx);
            input.set_accent(accent);
            if let Some(data) = &data {
                input.set_text(data.location.clone(), cx);
            }
            input
        });
        let notes = cx.new(|cx| {
            let mut area = TextArea::new(tr!("calendar-add-notes"), cx);
            area.set_accent(accent);
            if let Some(data) = &data {
                let len = data.description.len();
                area.set_text(data.description.clone(), len, cx);
            }
            area
        });
        let guest = cx.new(|cx| {
            let mut input = TextInput::new(tr!("calendar-add-guests"), cx);
            input.set_accent(accent);
            input
        });
        let on_input =
            |this: &mut Self, event: &InputEvent, window: &mut Window, cx: &mut Context<Self>| {
                match event {
                    InputEvent::Submit => this.save_event_draft(None, window, cx),
                    InputEvent::Cancel => this.close_event_draft(cx),
                    InputEvent::Changed => cx.notify(),
                }
            };
        let events = vec![
            cx.subscribe_in(
                &title,
                window,
                move |this, _, event: &InputEvent, window, cx| {
                    if let InputEvent::Changed = event {
                        this.read_typed_title(cx);
                    }
                    on_input(this, event, window, cx)
                },
            ),
            cx.subscribe_in(
                &location,
                window,
                move |this, _, event: &InputEvent, window, cx| on_input(this, event, window, cx),
            ),
            cx.subscribe_in(
                &guest,
                window,
                |this, input, event: &InputEvent, window, cx| match event {
                    InputEvent::Submit => {
                        let text = input.read(cx).text().to_owned();
                        if text.trim().is_empty() {
                            this.save_event_draft(None, window, cx);
                        } else {
                            this.add_guests(&text, cx);
                            input.update(cx, |input, cx| input.set_text("", cx));
                        }
                    }
                    InputEvent::Cancel => this.close_event_draft(cx),
                    InputEvent::Changed => cx.notify(),
                },
            ),
            cx.subscribe_in(
                &notes,
                window,
                |this, _, event: &InputEvent, window, cx| match event {
                    InputEvent::Submit => this.save_event_draft(None, window, cx),
                    InputEvent::Cancel => this.close_event_draft(cx),
                    InputEvent::Changed => {}
                },
            ),
        ];
        // With no calendar to add an event to, the card adds a task.
        let editing_none_without_calendar = editing.is_none() && calendar == 0;
        let tz = self.tz.clone();
        let (start_day, end_day, start_time, end_time, all_day) = match &editing {
            Some(o) if o.all_day() => {
                let first = utc_date(o.start);
                let last = utc_date((o.end - 1).max(o.start));
                (
                    first,
                    last,
                    Time::constant(9, 0, 0, 0),
                    Time::constant(10, 0, 0, 0),
                    true,
                )
            }
            Some(o) => {
                let (start, end) = (civil(o.start, &tz), civil(o.end, &tz));
                (start.date(), end.date(), start.time(), end.time(), false)
            }
            None => {
                let day = self.calendar.view_day();
                let start = next_hour(day, &tz);
                let (end, over) = add_minutes(start, NEW_EVENT_MINUTES);
                let end_day = day.checked_add(over.days()).unwrap_or(day);
                (day, end_day, start, end, false)
            }
        };
        let repeat = data
            .as_ref()
            .map(|d| Repeat::of(&d.rrule, start_day))
            .unwrap_or(Repeat::Never);
        let reminder = data
            .as_ref()
            .map_or(Some(10), |d| d.reminders.first().copied());
        let busy = data.as_ref().is_none_or(|d| d.busy);
        Some(Draft {
            editing,
            calendar,
            title,
            location,
            notes,
            guest,
            guests: data
                .as_ref()
                .map(|d| d.attendees.clone())
                .unwrap_or_default(),
            add_call: false,
            color: data.as_ref().map(|d| d.color.clone()).unwrap_or_default(),
            start_day,
            end_day,
            start_time,
            end_time,
            all_day,
            repeat,
            reminder,
            busy,
            full: false,
            kind: data.as_ref().map(|d| d.kind).unwrap_or_default(),
            task: editing_none_without_calendar,
            task_list: if editing_none_without_calendar {
                self.default_task_list(calendar)
            } else {
                0
            },
            at,
            pick: None,
            pick_month: start_day,
            list_scroll: ScrollHandle::new(),
            quick: None,
            _events: events,
        })
    }

    /// Opens the small card for a new event from `start` (minutes into
    /// `day`), or a whole-day one.
    pub(super) fn start_new_event(
        &mut self,
        day: Date,
        start: Option<Time>,
        all_day: bool,
        at: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.calendar.open = None;
        // Beside the click, as Google does, so the new event's block shows.
        let width = window.viewport_size().width;
        let x = if at.x > width / 2.0 {
            at.x - px(QUICK_WIDTH + 24.0)
        } else {
            at.x + px(24.0)
        };
        let at = gpui::point(x, at.y - px(120.0));
        let Some(mut draft) = self.new_draft(None, at, window, cx) else {
            return;
        };
        draft.start_day = day;
        draft.end_day = day;
        draft.all_day = all_day;
        draft.pick_month = day;
        if let Some(start) = start {
            let (end, over) = add_minutes(start, NEW_EVENT_MINUTES);
            draft.start_time = start;
            draft.end_time = end;
            draft.end_day = day.checked_add(over.days()).unwrap_or(day);
        }
        focus_later(&draft.title, window, cx);
        self.calendar.draft = Some(draft);
        cx.notify();
    }

    /// The left bar's New event: the small card for an event at the next
    /// hour on the day on show, in the middle of the window.
    pub(super) fn create_event_button(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let size = window.viewport_size();
        let at = gpui::point(size.width / 2.0 - px(QUICK_WIDTH / 2.0), size.height / 3.0);
        let day = self.calendar.view_day();
        self.start_new_event(day, None, false, at, window, cx);
    }

    /// C: a new event at the next hour, in the whole editor.
    pub(super) fn create_event_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let size = window.viewport_size();
        let at = gpui::point(size.width / 2.0 - px(QUICK_WIDTH / 2.0), size.height / 3.0);
        let day = self.calendar.view_day();
        self.start_new_event(day, None, false, at, window, cx);
        // Google's C opens the whole editor.
        self.more_options(window, cx);
    }

    /// Schedule a meeting from the conversation on line `key`: the whole
    /// editor on the Calendar page, titled with its subject and with its
    /// people as guests, at the next hour.
    pub(super) fn schedule_meeting_from(
        &mut self,
        key: Option<EntryKey>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((subject, people)) = key.and_then(|key| {
            self.mail
                .as_ref()
                .ok()
                .and_then(|mail| mail.meeting_source(key))
        }) else {
            return;
        };
        self.show_page(App::Calendar, window, cx);
        // The page reads its calendars in the background; the new event
        // needs them now.
        if self.calendar.calendars.is_empty() {
            self.calendar.calendars = Rc::new(super::calendar::read_calendars(&self.paths));
        }
        self.create_event_key(window, cx);
        let Some(draft) = &mut self.calendar.draft else {
            return;
        };
        draft
            .title
            .update(cx, |input, cx| input.set_text(subject, cx));
        draft.guests = people
            .into_iter()
            .map(|(name, email)| Attendee {
                email,
                name,
                status: "needs_action".to_owned(),
                ..Attendee::default()
            })
            .collect();
        cx.notify();
    }

    /// The open event's guests other than the user, as addresses.
    pub(super) fn other_guests(&self, occurrence: &Occurrence) -> Vec<String> {
        occurrence
            .event
            .data
            .attendees
            .iter()
            .filter(|a| {
                !a.is_self
                    && a.email.contains('@')
                    && !self
                        .accounts
                        .iter()
                        .any(|me| me.address.eq_ignore_ascii_case(&a.email))
            })
            .map(|a| {
                if a.name.is_empty() {
                    a.email.clone()
                } else {
                    format!("{} <{}>", a.name.replace(['<', '>', ','], ""), a.email)
                }
            })
            .collect()
    }

    /// A new mail to the open event's guests, from the calendar's account:
    /// titled with the event, or, when `late`, saying the user is running
    /// late, as Google Calendar's Email guests and Running late.
    pub(super) fn email_guests(&mut self, late: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(open) = &self.calendar.open else {
            return;
        };
        let occurrence = open.occurrence.clone();
        let to = self.other_guests(&occurrence);
        if to.is_empty() {
            return;
        }
        let data = &occurrence.event.data;
        let title = if data.title.is_empty() {
            tr!("calendar-no-title")
        } else {
            data.title.clone()
        };
        let account = self
            .calendar
            .calendars
            .iter()
            .find(|c| c.id == occurrence.event.calendar_id)
            .and_then(|c| c.account);
        let (subject, body) = if late {
            (
                tr!("calendar-late-subject", title = title.clone()),
                tr!("calendar-late-body", title = title),
            )
        } else {
            (title, String::new())
        };
        self.calendar.open = None;
        self.open_mailto(
            crate::mailto::Mailto {
                to,
                subject,
                body,
                ..Default::default()
            },
            window,
            cx,
        );
        if let Some(account) = account {
            self.send_compose_from(account);
        }
        cx.notify();
    }

    /// Opens the whole editor on the open event.
    pub(super) fn edit_open_event(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(open) = self.calendar.open.take() else {
            return;
        };
        if !self.can_edit(&open.occurrence) {
            self.show_snackbar(tr!("calendar-read-only"), None, cx);
            return;
        }
        if let Some(mut draft) = self.new_draft(Some(open.occurrence), open.at, window, cx) {
            draft.full = true;
            focus_later(&draft.title, window, cx);
            self.calendar.draft = Some(draft);
        }
        cx.notify();
    }

    pub(super) fn can_edit(&self, occurrence: &Occurrence) -> bool {
        self.calendar
            .calendars
            .iter()
            .any(|c| c.id == occurrence.event.calendar_id && c.access.can_edit())
    }

    pub(super) fn close_event_draft(&mut self, cx: &mut Context<Self>) {
        if let Some(draft) = &mut self.calendar.draft
            && draft.pick.take().is_some()
        {
            cx.notify();
            return;
        }
        self.calendar.draft = None;
        cx.notify();
    }

    fn more_options(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(draft) = &mut self.calendar.draft {
            draft.full = true;
            draft.pick = None;
            focus_later(&draft.title, window, cx);
        }
        cx.notify();
    }

    /// The new event's title without the day, time and place typed into
    /// it.
    fn draft_title(&self, draft: &Draft, cx: &Context<Self>) -> String {
        let text = draft.title.read(cx).text().trim().to_owned();
        if draft.quick.is_none() {
            return text;
        }
        let today = Zoned::now().with_time_zone(self.tz.clone()).date();
        read_typed(&text, today).title
    }

    /// Typed quick add: fills a new event's day, times and place from what
    /// its title says, and puts them back when the words go.
    fn read_typed_title(&mut self, cx: &mut Context<Self>) {
        let today = Zoned::now().with_time_zone(self.tz.clone()).date();
        let Some(draft) = &mut self.calendar.draft else {
            return;
        };
        if draft.editing.is_some() || draft.kind != EventKind::Default {
            return;
        }
        let typed = read_typed(draft.title.read(cx).text(), today);
        if draft.quick.is_none() && !typed.found() {
            return;
        }
        let base = draft.quick.get_or_insert_with(|| QuickBase {
            start_day: draft.start_day,
            end_day: draft.end_day,
            start_time: draft.start_time,
            end_time: draft.end_time,
            all_day: draft.all_day,
            repeat: draft.repeat.clone(),
            location: draft.location.read(cx).text().to_owned(),
            placed: false,
            repeated: false,
        });
        let length = base
            .start_day
            .to_datetime(base.start_time)
            .duration_until(base.end_day.to_datetime(base.end_time))
            .as_secs()
            / 60;
        let length = typed.minutes.unwrap_or(length.max(15));
        let start_day = typed.day.unwrap_or(base.start_day);
        let (mut start_time, mut end_time, mut end_day) = (
            base.start_time,
            base.end_time,
            start_day
                .checked_add((base.end_day - base.start_day).get_days().days())
                .unwrap_or(start_day),
        );
        if let Some(start) = typed.start {
            start_time = start;
        }
        if typed.start.is_some() || typed.minutes.is_some() {
            let (end, over) = match typed.end {
                Some(end) => (end, i64::from(end <= start_time)),
                None => add_minutes(start_time, length),
            };
            end_time = end;
            end_day = start_day.checked_add(over.days()).unwrap_or(start_day);
        }
        draft.start_day = start_day;
        draft.end_day = end_day;
        draft.start_time = start_time;
        draft.end_time = end_time;
        draft.all_day = base.all_day && typed.start.is_none();
        base.repeated = typed.repeat.is_some();
        draft.repeat = match &typed.repeat {
            Some(rule) => Repeat::of(rule, start_day),
            None => base.repeat.clone(),
        };
        draft.pick_month = start_day;
        let place = match (&typed.location, base.placed) {
            (Some(place), _) => Some(place.clone()),
            (None, true) => Some(base.location.clone()),
            (None, false) => None,
        };
        base.placed = typed.location.is_some();
        if let Some(place) = place
            && draft.location.read(cx).text() != place
        {
            draft
                .location
                .update(cx, |input, cx| input.set_text(place, cx));
        }
    }

    /// The draft's fields as the daemon takes them.
    fn draft_edit(&self, draft: &Draft, cx: &Context<Self>) -> Option<EventEdit> {
        let tz = &self.tz;
        let (start, end) = if draft.all_day {
            let end = draft.end_day.max(draft.start_day);
            (
                utc_midnight(draft.start_day),
                utc_midnight(end.tomorrow().unwrap_or(end)),
            )
        } else {
            (
                schedule::moment(draft.start_day, draft.start_time, tz)?.as_second(),
                schedule::moment(draft.end_day, draft.end_time, tz)?.as_second(),
            )
        };
        Some(EventEdit {
            title: self.draft_title(draft, cx),
            location: draft.location.read(cx).text().trim().to_owned(),
            description: draft.notes.read(cx).text().to_owned(),
            start,
            end,
            all_day: draft.all_day,
            time_zone: tz.iana_name().unwrap_or_default().to_owned(),
            rrule: draft.repeat.rule(draft.start_day),
            busy: draft.busy,
            color: draft.color.clone(),
            reminders: draft.reminder.into_iter().collect(),
            attendees: draft.guests.clone(),
            add_call: draft.add_call,
            kind: draft.kind,
        })
    }

    /// Makes the new event focus time, out of office, a working location
    /// or a plain event, with what Google gives each: its name as the
    /// title until one is typed, busy or free, and no reminder but for
    /// events; a working location is for the whole day.
    pub(super) fn set_draft_kind(&mut self, kind: EventKind, cx: &mut Context<Self>) {
        let Some(draft) = &mut self.calendar.draft else {
            return;
        };
        if (draft.kind == kind && !draft.task) || draft.editing.is_some() {
            return;
        }
        if draft.task && draft.calendar == 0 {
            self.show_snackbar(tr!("calendar-none-editable"), None, cx);
            return;
        }
        draft.task = false;
        let typed = draft.title.read(cx).text().trim().to_owned();
        if typed.is_empty() || typed == kind_title(draft.kind) {
            let title = kind_title(kind);
            draft
                .title
                .update(cx, |input, cx| input.set_text(title, cx));
        }
        draft.kind = kind;
        draft.busy = kind != EventKind::WorkingLocation;
        draft.reminder = (kind == EventKind::Default).then_some(10);
        if kind == EventKind::WorkingLocation {
            draft.all_day = true;
        }
        cx.notify();
    }

    /// Event, Task (on the small card), Focus time, Out of office and
    /// Working location, above a new event's times, as Google's tabs.
    fn render_kind_tabs(
        &self,
        draft: &Draft,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if draft.editing.is_some() {
            return None;
        }
        let tab = |id: gpui::ElementId, label: String, on: bool| {
            div()
                .id(id)
                .h(px(32.0))
                .px(px(8.0))
                .whitespace_nowrap()
                .flex()
                .items_center()
                .rounded(px(4.0))
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .cursor_pointer()
                .when(on, |d| {
                    d.bg(rgba(fade(th.accent, 0.14)))
                        .text_color(rgba(th.accent))
                })
                .when(!on, |d| {
                    d.text_color(rgba(th.text_dim))
                        .hover(|s| s.bg(rgba(th.hover)))
                })
                .child(label)
        };
        let kind_tab = |kind: EventKind| {
            tab(
                ("draft-kind", kind as usize).into(),
                kind_label(kind),
                draft.kind == kind && !draft.task,
            )
            .on_click(cx.listener(move |this, _, _, cx| this.set_draft_kind(kind, cx)))
        };
        let mut tabs = vec![kind_tab(EventKind::Default)];
        // A task is made on the small card only; the whole editor is an
        // event's.
        if !draft.full {
            tabs.push(
                tab(
                    "draft-kind-task".into(),
                    tr!("calendar-kind-task"),
                    draft.task,
                )
                .on_click(cx.listener(|this, _, window, cx| this.set_draft_task(true, window, cx))),
            );
        }
        tabs.extend(
            [
                EventKind::Focus,
                EventKind::OutOfOffice,
                EventKind::WorkingLocation,
            ]
            .map(kind_tab),
        );
        Some(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(2.0))
                .children(tabs)
                .into_any_element(),
        )
    }

    /// Adds the addresses in `text` (separated by commas, semicolons or
    /// spaces, maybe as `Name <address>`) as guests.
    fn add_guests(&mut self, text: &str, cx: &mut Context<Self>) {
        let Some(draft) = &mut self.calendar.draft else {
            return;
        };
        for part in text.split([',', ';', '\n']) {
            let part = part.trim();
            let (name, email) = match part.rsplit_once('<') {
                Some((name, rest)) => (
                    name.trim().trim_matches('"').to_owned(),
                    rest.trim_end_matches('>').trim().to_owned(),
                ),
                None => (String::new(), part.to_owned()),
            };
            for email in email.split_whitespace() {
                if !email.contains('@')
                    || draft
                        .guests
                        .iter()
                        .any(|g| g.email.eq_ignore_ascii_case(email))
                {
                    continue;
                }
                draft.guests.push(Attendee {
                    email: email.to_owned(),
                    name: name.clone(),
                    status: "needs_action".to_owned(),
                    ..Attendee::default()
                });
            }
        }
        cx.notify();
    }

    /// Saves the draft; a repeating event asks which occurrences first,
    /// unless `scope` says.
    pub(super) fn save_event_draft(
        &mut self,
        scope: Option<EditScope>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(draft) = &self.calendar.draft else {
            return;
        };
        if draft.task {
            self.save_task_draft(cx);
            return;
        }
        let Some(edit) = self.draft_edit(draft, cx) else {
            self.show_snackbar(tr!("calendar-no-such-time"), None, cx);
            return;
        };
        if edit.end < edit.start {
            self.show_snackbar(tr!("calendar-end-before-start"), None, cx);
            return;
        }
        let calendar = draft.calendar;
        let Some(occurrence) = draft.editing.clone() else {
            self.calendar.draft = None;
            self.edit_calendar(
                EventChange::Add { calendar, edit },
                UndoPlan::DeleteAdded,
                tr!("calendar-saved"),
                cx,
            );
            return;
        };
        if draft.repeating() && scope.is_none() {
            self.calendar.ask = Some(ScopeAsk {
                change: ScopeChange::Save(occurrence, Box::new(edit), calendar),
                scope: EditScope::This,
            });
            cx.notify();
            return;
        }
        self.calendar.draft = None;
        self.save_change(occurrence, edit, calendar, scope.unwrap_or_default(), cx);
    }

    fn save_change(
        &mut self,
        occurrence: Occurrence,
        edit: EventEdit,
        calendar: i64,
        scope: EditScope,
        cx: &mut Context<Self>,
    ) {
        let event = &occurrence.event;
        let before = EventEdit::of(&event.data);
        let mut old = before.clone();
        // The occurrence's own times, which a series' row does not have.
        old.start = occurrence.start;
        old.end = occurrence.end;
        let undo = match scope {
            EditScope::This => UndoPlan::ChangeBack(old),
            EditScope::All => UndoPlan::Fixed(EventChange::Change {
                event: event.id,
                scope: EditScope::All,
                occurrence: Some(occurrence.start),
                edit: {
                    let mut back = before;
                    back.start = occurrence.start;
                    back.end = occurrence.end;
                    back
                },
                calendar: (calendar != event.calendar_id).then_some(event.calendar_id),
            }),
            EditScope::Following => UndoPlan::None,
        };
        let change = EventChange::Change {
            event: event.id,
            scope,
            occurrence: occurrence.series_start.map(|_| occurrence.start).or((!event
                .data
                .rrule
                .is_empty())
            .then_some(occurrence.start)),
            edit,
            calendar: (calendar != event.calendar_id).then_some(calendar),
        };
        self.edit_calendar(change, undo, tr!("calendar-saved"), cx);
    }

    /// Ends a drag in the Day or Week grid: the event moves (or its end
    /// does) by what the drag showed, after asking which occurrences for
    /// a repeating one.
    pub(super) fn drop_dragged_event(&mut self, cx: &mut Context<Self>) {
        let Some(drag) = self.calendar.drag.take() else {
            return;
        };
        cx.notify();
        if !drag.moved || (drag.minutes == 0 && drag.days == 0) {
            return;
        }
        let occurrence = drag.occurrence;
        let tz = self.tz.clone();
        let shift = |seconds: i64| -> i64 {
            civil(seconds, &tz)
                .checked_add(drag.days.days().minutes(drag.minutes))
                .ok()
                .and_then(|at| at.to_zoned(tz.clone()).ok())
                .map_or(seconds, |z| z.timestamp().as_second())
        };
        let mut edit = occurrence_edit(&occurrence);
        if drag.resize {
            edit.end = (occurrence.end + drag.minutes * 60).max(occurrence.start + 15 * 60);
        } else {
            edit.start = shift(occurrence.start);
            edit.end = shift(occurrence.end);
        }
        let calendar = occurrence.event.calendar_id;
        self.change_occurrence(occurrence, edit, calendar, cx);
    }

    /// Saves `edit` for `occurrence` in `calendar`, after asking which
    /// occurrences for a repeating event.
    fn change_occurrence(
        &mut self,
        occurrence: Occurrence,
        edit: EventEdit,
        calendar: i64,
        cx: &mut Context<Self>,
    ) {
        if !occurrence.event.data.rrule.is_empty() || occurrence.series_start.is_some() {
            self.calendar.ask = Some(ScopeAsk {
                change: ScopeChange::Save(occurrence, Box::new(edit), calendar),
                scope: EditScope::This,
            });
            cx.notify();
            return;
        }
        self.save_change(occurrence, edit, calendar, EditScope::This, cx);
    }

    /// Gives the event `color` (`#rrggbb`, or empty for its calendar's),
    /// from its right-click menu.
    pub(super) fn recolor_event(
        &mut self,
        occurrence: Occurrence,
        color: String,
        cx: &mut Context<Self>,
    ) {
        if occurrence.event.data.color == color {
            return;
        }
        let mut edit = occurrence_edit(&occurrence);
        edit.color = color;
        let calendar = occurrence.event.calendar_id;
        self.change_occurrence(occurrence, edit, calendar, cx);
    }

    /// Moves the event, the whole series of a repeating one, to
    /// `calendar`, from its right-click menu.
    pub(super) fn move_event_to(
        &mut self,
        occurrence: Occurrence,
        calendar: i64,
        cx: &mut Context<Self>,
    ) {
        if occurrence.event.calendar_id == calendar {
            return;
        }
        let edit = occurrence_edit(&occurrence);
        self.save_change(occurrence, edit, calendar, EditScope::All, cx);
    }

    /// Duplicate: the whole editor on a new event copied from
    /// `occurrence`, as Google Calendar's; saving adds it.
    pub(super) fn duplicate_event(
        &mut self,
        occurrence: Occurrence,
        at: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.calendar.open = None;
        let Some(mut draft) = self.new_draft(Some(occurrence), at, window, cx) else {
            return;
        };
        draft.editing = None;
        draft.full = true;
        focus_later(&draft.title, window, cx);
        self.calendar.draft = Some(draft);
        cx.notify();
    }

    /// Closes a new event's small card, not the whole editor.
    pub(super) fn close_quick_draft(&mut self) {
        if self.calendar.draft.as_ref().is_some_and(|d| !d.full) {
            self.calendar.draft = None;
        }
    }

    /// Yes, No or Maybe on an invitation's card; a repeating one asks
    /// which occurrences first.
    pub(super) fn respond_to_event(&mut self, status: &str, cx: &mut Context<Self>) {
        let Some(open) = &self.calendar.open else {
            return;
        };
        let occurrence = open.occurrence.clone();
        if occurrence.event.data.self_status == status {
            return;
        }
        self.calendar.open = None;
        if !occurrence.event.data.rrule.is_empty() || occurrence.series_start.is_some() {
            self.calendar.ask = Some(ScopeAsk {
                change: ScopeChange::Respond(occurrence, status.to_owned()),
                scope: EditScope::All,
            });
            cx.notify();
            return;
        }
        self.send_response(occurrence, status.to_owned(), EditScope::This, cx);
    }

    pub(super) fn send_response(
        &mut self,
        occurrence: Occurrence,
        status: String,
        scope: EditScope,
        cx: &mut Context<Self>,
    ) {
        let event = &occurrence.event;
        let series = !event.data.rrule.is_empty() || occurrence.series_start.is_some();
        let at = series.then_some(occurrence.series_start.unwrap_or(occurrence.start));
        let before = event.data.self_status.clone();
        // An invitation not answered yet cannot be unanswered.
        let undo = if matches!(before.as_str(), "accepted" | "tentative" | "declined") {
            UndoPlan::Fixed(EventChange::Respond {
                event: event.id,
                scope,
                occurrence: at,
                status: before,
            })
        } else {
            UndoPlan::None
        };
        let done = match status.as_str() {
            "accepted" => tr!("calendar-answered-yes"),
            "declined" => tr!("calendar-answered-no"),
            _ => tr!("calendar-answered-maybe"),
        };
        let change = EventChange::Respond {
            event: event.id,
            scope,
            occurrence: at,
            status,
        };
        self.edit_calendar(change, undo, done, cx);
    }

    /// Delete, or Delete on the open event's card.
    pub(super) fn delete_open_event(&mut self, cx: &mut Context<Self>) {
        let Some(open) = self.calendar.open.take() else {
            return;
        };
        let occurrence = open.occurrence;
        if !self.can_edit(&occurrence) {
            self.show_snackbar(tr!("calendar-read-only"), None, cx);
            return;
        }
        if !occurrence.event.data.rrule.is_empty() || occurrence.series_start.is_some() {
            self.calendar.ask = Some(ScopeAsk {
                change: ScopeChange::Delete(occurrence),
                scope: EditScope::This,
            });
            cx.notify();
            return;
        }
        self.delete_occurrence(occurrence, EditScope::This, cx);
    }

    fn delete_occurrence(
        &mut self,
        occurrence: Occurrence,
        scope: EditScope,
        cx: &mut Context<Self>,
    ) {
        let event = &occurrence.event;
        let series = !event.data.rrule.is_empty() || occurrence.series_start.is_some();
        let undo = match scope {
            EditScope::This if series => UndoPlan::Fixed(EventChange::Restore {
                event: event.id,
                occurrence: occurrence.series_start.unwrap_or(occurrence.start),
            }),
            EditScope::Following if event.data.rrule.is_empty() => UndoPlan::None,
            EditScope::Following => UndoPlan::Fixed(EventChange::Change {
                event: event.id,
                scope: EditScope::All,
                occurrence: None,
                edit: EventEdit::of(&event.data),
                calendar: None,
            }),
            EditScope::This | EditScope::All => UndoPlan::Fixed(EventChange::Add {
                calendar: event.calendar_id,
                edit: EventEdit::of(&event.data),
            }),
        };
        let change = EventChange::Delete {
            event: event.id,
            scope,
            occurrence: series.then_some(occurrence.series_start.unwrap_or(occurrence.start)),
        };
        self.edit_calendar(change, undo, tr!("calendar-deleted"), cx);
    }

    /// Sends `change` to the daemon, then reads the calendar again and
    /// says `done`, with Undo when `undo` allows.
    fn edit_calendar(
        &mut self,
        change: EventChange,
        undo: UndoPlan,
        done: String,
        cx: &mut Context<Self>,
    ) {
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = async {
                let connection = match connection {
                    Some(connection) => connection,
                    None => daemon::connect().await?,
                };
                daemon::edit_event(&connection, &change).await
            }
            .await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(id) => {
                        let undo = match undo {
                            UndoPlan::None => None,
                            UndoPlan::Fixed(change) => Some(change),
                            UndoPlan::DeleteAdded if id > 0 => Some(EventChange::Delete {
                                event: id,
                                scope: EditScope::All,
                                occurrence: None,
                            }),
                            UndoPlan::ChangeBack(edit) if id > 0 => Some(EventChange::Change {
                                event: id,
                                scope: EditScope::This,
                                occurrence: None,
                                edit,
                                calendar: None,
                            }),
                            UndoPlan::DeleteAdded | UndoPlan::ChangeBack(_) => None,
                        };
                        this.show_snackbar(done, undo.map(|c| Command::Event(Box::new(c))), cx);
                    }
                    Err(err) => this.show_snackbar(err, None, cx),
                }
                this.load_calendar(cx);
                this.load_agenda(cx);
            })
            .ok();
        })
        .detach();
    }

    /// Undo of a calendar change: sends it back without a new Undo.
    pub(super) fn undo_event_change(&mut self, change: EventChange, cx: &mut Context<Self>) {
        self.edit_calendar(change, UndoPlan::None, tr!("toast-undone"), cx);
    }

    fn answer_scope(&mut self, ok: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(ask) = self.calendar.ask.take() else {
            return;
        };
        if ok {
            match ask.change {
                ScopeChange::Save(occurrence, edit, calendar) => {
                    self.calendar.draft = None;
                    self.save_change(occurrence, *edit, calendar, ask.scope, cx);
                }
                ScopeChange::Delete(occurrence) => {
                    self.delete_occurrence(occurrence, ask.scope, cx);
                }
                ScopeChange::Respond(occurrence, status) => {
                    self.send_response(occurrence, status, ask.scope, cx);
                }
            }
        } else if let Some(draft) = &self.calendar.draft {
            focus_later(&draft.title, window, cx);
        }
        cx.notify();
    }

    fn open_pick(&mut self, pick: Pick, at: Point<Pixels>, cx: &mut Context<Self>) {
        cx.stop_propagation();
        let Some(draft) = &mut self.calendar.draft else {
            return;
        };
        if draft.pick.is_some_and(|(p, _)| p == pick) {
            draft.pick = None;
        } else {
            draft.pick = Some((pick, at));
            draft.pick_month = match pick {
                Pick::EndDay => draft.end_day,
                _ => draft.start_day,
            };
            let time = match pick {
                Pick::StartTime => Some(draft.start_time),
                Pick::EndTime => Some(draft.end_time),
                _ => None,
            };
            if let Some(time) = time {
                let ix = (i32::from(time.hour()) * 60 + i32::from(time.minute())) / TIME_STEP;
                draft
                    .list_scroll
                    .scroll_to_item((ix as usize).saturating_sub(2));
            }
        }
        cx.notify();
    }

    /// Sets the start day, keeping the length.
    fn set_start_day(&mut self, day: Date, cx: &mut Context<Self>) {
        if let Some(draft) = &mut self.calendar.draft {
            let length = draft.end_day - draft.start_day;
            draft.start_day = day;
            draft.end_day = day.checked_add(length).unwrap_or(day);
            draft.pick = None;
        }
        cx.notify();
    }

    /// Sets the start time, keeping the length.
    fn set_start_time(&mut self, time: Time, cx: &mut Context<Self>) {
        if let Some(draft) = &mut self.calendar.draft {
            let length = self_length(draft);
            draft.start_time = time;
            let (end, over) = add_minutes(time, length);
            draft.end_time = end;
            draft.end_day = draft
                .start_day
                .checked_add(over.days())
                .unwrap_or(draft.start_day);
            draft.pick = None;
        }
        cx.notify();
    }

    /// The small card for a new event, or the whole editor, and the
    /// question which occurrences a change is for.
    pub(super) fn render_event_draft(&self, th: &Theme, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let mut layers = Vec::new();
        if let Some(draft) = &self.calendar.draft
            && !draft.full
        {
            layers.push(
                deferred(
                    div()
                        .id("draft-scrim")
                        .absolute()
                        .top(px(-2000.0))
                        .left(px(-4000.0))
                        .w(px(8000.0))
                        .h(px(6000.0))
                        .occlude()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                cx.stop_propagation();
                                this.calendar.draft = None;
                                cx.notify();
                            }),
                        ),
                )
                .with_priority(3)
                .into_any_element(),
            );
            layers.push(self.render_quick_card(draft, th, cx));
        }
        if let Some(draft) = &self.calendar.draft
            && let Some((pick, at)) = draft.pick
        {
            layers.push(self.render_pick(draft, pick, at, th, cx));
        }
        if let Some(ask) = &self.calendar.ask {
            layers.push(self.render_scope_ask(ask, th, cx));
        }
        layers
    }

    /// Whether the whole editor is open, in place of the page.
    pub(super) fn event_editor_open(&self) -> bool {
        self.calendar.draft.as_ref().is_some_and(|d| d.full)
    }

    fn date_chip(
        &self,
        draft: &Draft,
        pick: Pick,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let day = if pick == Pick::EndDay {
            draft.end_day
        } else {
            draft.start_day
        };
        let at = day.to_datetime(Time::midnight());
        let label = tr!(
            "calendar-weekday-day",
            weekday = format::weekday(at),
            day = format::day_month(at)
        );
        chip(
            ("draft-chip", pick as usize),
            label,
            draft.pick.map(|p| p.0) == Some(pick),
            th,
        )
        .on_click(cx.listener(move |this, event: &ClickEvent, _, cx| {
            this.open_pick(pick, event.position(), cx)
        }))
        .into_any_element()
    }

    fn time_chip(
        &self,
        draft: &Draft,
        pick: Pick,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let time = if pick == Pick::EndTime {
            draft.end_time
        } else {
            draft.start_time
        };
        chip(
            ("draft-chip", pick as usize),
            schedule::clock(time),
            draft.pick.map(|p| p.0) == Some(pick),
            th,
        )
        .on_click(cx.listener(move |this, event: &ClickEvent, _, cx| {
            this.open_pick(pick, event.position(), cx)
        }))
        .into_any_element()
    }

    /// The dates and times: "Tuesday, 29 September 10:00 – 11:00".
    fn render_when(&self, draft: &Draft, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let mut row = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(4.0))
            .child(self.date_chip(draft, Pick::StartDay, th, cx));
        if draft.all_day {
            if draft.end_day != draft.start_day || draft.full {
                row = row
                    .child(dash(th))
                    .child(self.date_chip(draft, Pick::EndDay, th, cx));
            }
        } else {
            row = row
                .child(self.time_chip(draft, Pick::StartTime, th, cx))
                .child(dash(th))
                .child(self.time_chip(draft, Pick::EndTime, th, cx));
            if draft.end_day != draft.start_day || draft.full {
                row = row.child(self.date_chip(draft, Pick::EndDay, th, cx));
            }
        }
        row.into_any_element()
    }

    fn render_all_day(&self, draft: &Draft, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let on = draft.all_day;
        div()
            .id("draft-all-day")
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .h(px(32.0))
            .px(px(4.0))
            .rounded(px(4.0))
            .cursor_pointer()
            .relative()
            .child(crate::widgets::hover_fade("hover-glow", Some(4.0), th))
            .text_size(px(14.0))
            .text_color(rgba(th.text))
            .child(crate::widgets::checkbox(
                "calendar-all-day-box",
                crate::widgets::Check::from(on),
                th,
            ))
            .child(tr!("calendar-all-day-box"))
            .on_click(cx.listener(|this, _, _, cx| {
                if let Some(draft) = &mut this.calendar.draft {
                    draft.all_day = !draft.all_day;
                    draft.pick = None;
                }
                cx.notify();
            }))
            .into_any_element()
    }

    fn calendar_chip(&self, draft: &Draft, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let calendar = self
            .calendar
            .calendars
            .iter()
            .find(|c| c.id == draft.calendar);
        let name = calendar
            .map(super::calendar::calendar_name)
            .unwrap_or_default();
        let color = calendar
            .map(super::calendar::calendar_color)
            .unwrap_or(th.accent);
        chip_with_dot(
            "draft-calendar",
            name,
            draft.pick.map(|p| p.0) == Some(Pick::Calendar),
            Some(color),
            th,
        )
        .on_click(cx.listener(|this, event: &ClickEvent, _, cx| {
            this.open_pick(Pick::Calendar, event.position(), cx)
        }))
        .into_any_element()
    }

    fn render_quick_card(&self, draft: &Draft, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let row = |name: &'static str| {
            div().flex().flex_row().items_center().gap(px(16.0)).child(
                div()
                    .flex_none()
                    .w(px(24.0))
                    .child(icon(name, th.text_dim, 20.0)),
            )
        };
        let card = raised(
            div()
                .id("event-draft")
                .occlude()
                // A phone's window, less a margin at each side.
                .w(px(QUICK_WIDTH.min(self.layout.shape.width - 16.0)))
                .p(px(8.0))
                .pb(px(16.0))
                .flex()
                .flex_col()
                .gap(px(12.0))
                .text_color(rgba(th.text))
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation()),
            th,
            15.0,
            3.0,
        )
        .child(
            div().flex().flex_row().justify_end().child(
                icon_button("draft-close", "close", 20.0, th)
                    .tooltip(tip(tr!("calendar-close"), th))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.calendar.draft = None;
                        cx.notify();
                    })),
            ),
        )
        .child(
            div()
                .ml(px(48.0))
                .mr(px(16.0))
                .pb(px(4.0))
                .border_b_2()
                .border_color(rgba(th.accent))
                .text_size(px(22.0))
                .child(draft.title.clone()),
        )
        .children(
            self.render_kind_tabs(draft, th, cx)
                .map(|tabs| div().ml(px(40.0)).mr(px(8.0)).child(tabs)),
        )
        .when(draft.task, |d| {
            d.child(self.render_task_fields(draft, th, cx))
        })
        .when(!draft.task, |d| {
            d.child(
                div()
                    .px(px(8.0))
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(row("schedule").child(self.render_when(draft, th, cx)))
                    // The place the title named (typed quick add).
                    .when(draft.quick.as_ref().is_some_and(|q| q.placed), |d| {
                        d.child(
                            row("location").child(
                                div()
                                    .h(px(36.0))
                                    .px(px(10.0))
                                    .flex()
                                    .items_center()
                                    .text_size(px(14.0))
                                    .child(draft.location.read(cx).text().to_owned()),
                            ),
                        )
                    })
                    // The repeat the title named.
                    .when(draft.quick.as_ref().is_some_and(|q| q.repeated), |d| {
                        d.child(
                            row("repeat").child(
                                div()
                                    .h(px(36.0))
                                    .px(px(10.0))
                                    .flex()
                                    .items_center()
                                    .text_size(px(14.0))
                                    .child(draft.repeat.label(draft.start_day)),
                            ),
                        )
                    })
                    .child(row("calendar").child(self.calendar_chip(draft, th, cx))),
            )
        })
        .child(
            div()
                .px(px(16.0))
                .flex()
                .flex_row()
                .justify_end()
                .items_center()
                .gap(px(8.0))
                .when(!draft.task, |d| {
                    d.child(
                        text_button("draft-more", tr!("calendar-more-options"), th).on_click(
                            cx.listener(|this, _, window, cx| this.more_options(window, cx)),
                        ),
                    )
                })
                .child(
                    filled_button("draft-save", tr!("calendar-save"), th).on_click(
                        cx.listener(|this, _, window, cx| this.save_event_draft(None, window, cx)),
                    ),
                ),
        );
        deferred(
            anchored()
                .position(draft.at)
                .snap_to_window_with_margin(px(8.0))
                .child(card),
        )
        .with_priority(4)
        .into_any_element()
    }

    /// The whole editor, in place of the page (Google's "More options").
    pub(super) fn render_event_editor(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(draft) = &self.calendar.draft else {
            return div().into_any_element();
        };
        let field = |name: &'static str| {
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .min_h(px(40.0))
                .child(
                    div()
                        .flex_none()
                        .w(px(24.0))
                        .child(icon(name, th.text_dim, 20.0)),
                )
        };
        let input_box = || {
            div()
                .flex_1()
                .h(px(40.0))
                .px(px(12.0))
                .flex()
                .items_center()
                .rounded(px(4.0))
                .bg(rgba(th.hover))
                .text_size(px(14.0))
        };
        let repeat_label = draft.repeat.label(draft.start_day);
        let busy_label = if draft.busy {
            tr!("calendar-busy")
        } else {
            tr!("calendar-free")
        };
        div()
            .id("event-editor")
            .size_full()
            .flex()
            .flex_col()
            .text_color(rgba(th.text))
            .child(
                div()
                    .flex_none()
                    .h(px(64.0))
                    .px(px(16.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(16.0))
                    .child(
                        icon_button("editor-close", "close", 24.0, th)
                            .tooltip(tip(tr!("calendar-discard"), th))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.calendar.draft = None;
                                cx.notify();
                            })),
                    )
                    .child(
                        div()
                            .flex_1()
                            .max_w(px(720.0))
                            .h(px(48.0))
                            .px(px(12.0))
                            .flex()
                            .items_center()
                            .rounded(px(4.0))
                            .bg(rgba(th.hover))
                            .border_b_2()
                            .border_color(rgba(th.accent))
                            .text_size(px(22.0))
                            .child(draft.title.clone()),
                    )
                    .child(
                        filled_button("editor-save", tr!("calendar-save"), th).on_click(
                            cx.listener(|this, _, window, cx| {
                                this.save_event_draft(None, window, cx)
                            }),
                        ),
                    ),
            )
            .child(
                div()
                    .id("editor-body")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .px(px(72.0))
                    .pb(px(32.0))
                    .child(
                        div()
                            .max_w(px(720.0))
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .children(self.render_kind_tabs(draft, th, cx))
                            .child(self.render_when(draft, th, cx))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(16.0))
                                    .child(self.render_all_day(draft, th, cx))
                                    .child(
                                        chip(
                                            "draft-repeat",
                                            repeat_label,
                                            draft.pick.map(|p| p.0) == Some(Pick::Repeat),
                                            th,
                                        )
                                        .on_click(
                                            cx.listener(|this, event: &ClickEvent, _, cx| {
                                                this.open_pick(Pick::Repeat, event.position(), cx)
                                            }),
                                        ),
                                    ),
                            )
                            .child(
                                div()
                                    .mt(px(12.0))
                                    .pb(px(8.0))
                                    .border_b_1()
                                    .border_color(rgba(th.divider))
                                    .text_size(px(14.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgba(th.accent))
                                    .child(tr!("calendar-event-details")),
                            )
                            .child(field("pin").child(input_box().child(draft.location.clone())))
                            .child(
                                field("bell").child(
                                    chip(
                                        "draft-reminder",
                                        reminder_label(draft.reminder),
                                        draft.pick.map(|p| p.0) == Some(Pick::Reminder),
                                        th,
                                    )
                                    .on_click(cx.listener(
                                        |this, event: &ClickEvent, _, cx| {
                                            this.open_pick(Pick::Reminder, event.position(), cx)
                                        },
                                    )),
                                ),
                            )
                            .child(
                                field("calendar")
                                    .child(self.calendar_chip(draft, th, cx))
                                    .child(
                                        chip(
                                            "draft-busy",
                                            busy_label,
                                            draft.pick.map(|p| p.0) == Some(Pick::Busy),
                                            th,
                                        )
                                        .on_click(
                                            cx.listener(|this, event: &ClickEvent, _, cx| {
                                                this.open_pick(Pick::Busy, event.position(), cx)
                                            }),
                                        ),
                                    ),
                            )
                            .children(self.render_call_row(draft, th, cx))
                            .child(field("people").child(input_box().child(draft.guest.clone())))
                            .children(self.render_draft_guests(draft, th, cx))
                            .child(
                                field("notes").items_start().child(
                                    div()
                                        .flex_1()
                                        .min_h(px(160.0))
                                        .p(px(12.0))
                                        .rounded(px(4.0))
                                        .bg(rgba(th.hover))
                                        .text_size(px(14.0))
                                        .child(draft.notes.clone()),
                                ),
                            ),
                    ),
            )
            .children(self.render_event_draft(th, cx))
            .into_any_element()
    }

    /// "Add Google Meet video call" (or Teams), for calendars whose
    /// service makes calls; the link once there is one.
    fn render_call_row(
        &self,
        draft: &Draft,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        use katna_store::calendar::CalendarSource;
        let calendar = self
            .calendar
            .calendars
            .iter()
            .find(|c| c.id == draft.calendar)?;
        let join = draft
            .editing
            .as_ref()
            .map(|o| o.event.data.join_url.clone())
            .unwrap_or_default();
        let label = match calendar.source {
            _ if !join.is_empty() => tr!("calendar-has-call"),
            CalendarSource::Google => tr!("calendar-add-meet"),
            CalendarSource::Microsoft => tr!("calendar-add-teams"),
            CalendarSource::CalDav | CalendarSource::Zoho | CalendarSource::Local => return None,
        };
        let on = draft.add_call;
        let row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(16.0))
            .min_h(px(40.0))
            .child(
                div()
                    .flex_none()
                    .w(px(24.0))
                    .child(icon("event", th.text_dim, 20.0)),
            );
        Some(if join.is_empty() {
            row.child(
                div()
                    .id("draft-call")
                    .h(px(36.0))
                    .px(px(16.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .rounded(px(4.0))
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .cursor_pointer()
                    .when(on, |d| {
                        d.bg(rgba(th.nav_selected))
                            .text_color(rgba(th.nav_selected_text))
                            .child(icon("check", th.nav_selected_text, 18.0))
                    })
                    .when(!on, |d| {
                        d.bg(rgba(th.accent))
                            .text_color(rgba(th.on_accent))
                            .hover(|s| s.shadow(crate::widgets::elevation(th, 1.0)))
                    })
                    .child(label)
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(draft) = &mut this.calendar.draft {
                            draft.add_call = !draft.add_call;
                        }
                        cx.notify();
                    })),
            )
            .into_any_element()
        } else {
            row.child(
                div()
                    .text_size(px(14.0))
                    .text_color(rgba(th.text))
                    .child(label),
            )
            .into_any_element()
        })
    }

    /// The guests added, each with a button to take them off.
    fn render_draft_guests(
        &self,
        draft: &Draft,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if draft.guests.is_empty() {
            return None;
        }
        let rows = draft.guests.iter().enumerate().map(|(ix, guest)| {
            let name = if guest.name.is_empty() {
                guest.email.clone()
            } else {
                guest.name.clone()
            };
            let email = guest.email.clone();
            div()
                .id(("draft-guest", ix))
                .h(px(40.0))
                .px(px(8.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(12.0))
                .rounded(px(4.0))
                .relative()
                .child(crate::widgets::hover_fade("hover-glow", Some(4.0), th))
                .child(crate::widgets::avatar(&name, &guest.email, 28.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .text_size(px(14.0))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(6.0))
                                .child(div().min_w_0().truncate().child(name))
                                .children(self.muted_mark(&guest.email, 16.0, th)),
                        )
                        .when(guest.organizer, |d| {
                            d.child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgba(th.text_dim))
                                    .child(tr!("calendar-organizer")),
                            )
                        }),
                )
                .when(!guest.organizer && !guest.is_self, |d| {
                    d.child(
                        icon_button(("draft-guest-remove", ix), "close", 18.0, th)
                            .tooltip(tip(tr!("calendar-remove-guest"), th))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if let Some(draft) = &mut this.calendar.draft {
                                    draft.guests.retain(|g| g.email != email);
                                }
                                cx.notify();
                            })),
                    )
                })
        });
        Some(
            div()
                .ml(px(40.0))
                .flex()
                .flex_col()
                .children(rows)
                .into_any_element(),
        )
    }

    fn render_pick(
        &self,
        draft: &Draft,
        pick: Pick,
        at: Point<Pixels>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let item = |id: (&'static str, usize), text: String, on: bool| {
            div()
                .id(id)
                .h(px(36.0))
                .px(px(16.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(12.0))
                .cursor_pointer()
                .when(on, |d| d.bg(rgba(th.nav_selected)))
                .hover(|s| s.bg(rgba(th.hover)))
                .when(!text.is_empty(), |d| d.child(text))
        };
        let body: AnyElement = match pick {
            Pick::StartDay | Pick::EndDay => self.render_pick_month(draft, pick, th, cx),
            Pick::StartTime | Pick::EndTime => {
                let current = if pick == Pick::StartTime {
                    draft.start_time
                } else {
                    draft.end_time
                };
                let times = (0..24 * 60 / TIME_STEP).map(|ix| {
                    let (time, _) = add_minutes(Time::midnight(), i64::from(ix * TIME_STEP));
                    item(
                        ("draft-time", ix as usize),
                        schedule::clock(time),
                        time == current,
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if pick == Pick::StartTime {
                            this.set_start_time(time, cx);
                        } else if let Some(draft) = &mut this.calendar.draft {
                            draft.end_time = time;
                            if time <= draft.start_time && draft.end_day == draft.start_day {
                                draft.end_day =
                                    draft.start_day.tomorrow().unwrap_or(draft.start_day);
                            }
                            draft.pick = None;
                            cx.notify();
                        }
                    }))
                });
                div()
                    .id("draft-times")
                    .w(px(160.0))
                    .max_h(px(280.0))
                    .overflow_y_scroll()
                    .track_scroll(&draft.list_scroll)
                    .children(times)
                    .into_any_element()
            }
            Pick::TaskList => self.render_task_lists(draft, th, cx),
            Pick::Calendar => {
                let items = self
                    .calendar
                    .calendars
                    .iter()
                    .filter(|c| c.access.can_edit())
                    .map(|c| {
                        let id = c.id;
                        let color = super::calendar::calendar_color(c);
                        item(
                            ("draft-cal", id as usize),
                            String::new(),
                            id == draft.calendar,
                        )
                        .child(div().size(px(12.0)).rounded_full().bg(rgba(color)))
                        .child(super::calendar::calendar_name(c))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if let Some(draft) = &mut this.calendar.draft {
                                draft.calendar = id;
                                draft.pick = None;
                            }
                            cx.notify();
                        }))
                    })
                    .collect::<Vec<_>>();
                div().min_w(px(220.0)).children(items).into_any_element()
            }
            Pick::Repeat => {
                let day = draft.start_day;
                let mut options: Vec<Repeat> = Repeat::OFFERED.to_vec();
                if let Repeat::Kept(_) = draft.repeat {
                    options.push(draft.repeat.clone());
                }
                let items = options
                    .into_iter()
                    .enumerate()
                    .map(|(ix, repeat)| {
                        let on = repeat == draft.repeat;
                        item(("draft-repeat-item", ix), repeat.label(day), on).on_click(
                            cx.listener(move |this, _, _, cx| {
                                if let Some(draft) = &mut this.calendar.draft {
                                    draft.repeat = repeat.clone();
                                    draft.pick = None;
                                }
                                cx.notify();
                            }),
                        )
                    })
                    .collect::<Vec<_>>();
                div().min_w(px(260.0)).children(items).into_any_element()
            }
            Pick::Reminder => {
                let options = std::iter::once(None).chain(REMINDERS.into_iter().map(Some));
                let items = options
                    .enumerate()
                    .map(|(ix, minutes)| {
                        item(
                            ("draft-reminder-item", ix),
                            reminder_label(minutes),
                            minutes == draft.reminder,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if let Some(draft) = &mut this.calendar.draft {
                                draft.reminder = minutes;
                                draft.pick = None;
                            }
                            cx.notify();
                        }))
                    })
                    .collect::<Vec<_>>();
                div().min_w(px(220.0)).children(items).into_any_element()
            }
            Pick::Busy => {
                let items = [(true, tr!("calendar-busy")), (false, tr!("calendar-free"))]
                    .into_iter()
                    .map(|(busy, text)| {
                        item(("draft-busy-item", busy as usize), text, busy == draft.busy).on_click(
                            cx.listener(move |this, _, _, cx| {
                                if let Some(draft) = &mut this.calendar.draft {
                                    draft.busy = busy;
                                    draft.pick = None;
                                }
                                cx.notify();
                            }),
                        )
                    })
                    .collect::<Vec<_>>();
                div().min_w(px(160.0)).children(items).into_any_element()
            }
        };
        div()
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .child(
                deferred(
                    div()
                        .id("draft-pick-scrim")
                        .absolute()
                        .top(px(-2000.0))
                        .left(px(-4000.0))
                        .w(px(8000.0))
                        .h(px(6000.0))
                        .occlude()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                cx.stop_propagation();
                                if let Some(draft) = &mut this.calendar.draft {
                                    draft.pick = None;
                                }
                                cx.notify();
                            }),
                        ),
                )
                .with_priority(5),
            )
            .child(
                deferred(
                    anchored()
                        .position(at)
                        .snap_to_window_with_margin(px(8.0))
                        .child(
                            menu(th)
                                .id("draft-pick")
                                .occlude()
                                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                                .child(body),
                        ),
                )
                .with_priority(6),
            )
            .into_any_element()
    }

    fn render_pick_month(
        &self,
        draft: &Draft,
        pick: Pick,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let month = draft.pick_month;
        let chosen = if pick == Pick::EndDay {
            draft.end_day
        } else {
            draft.start_day
        };
        let today = Zoned::now().with_time_zone(self.tz.clone()).date();
        let days = schedule::month_grid(month, format::first_weekday())
            .into_iter()
            .enumerate()
            .map(|(ix, date)| {
                let selected = date == chosen;
                let early = pick == Pick::EndDay && date < draft.start_day;
                div()
                    .id(("draft-day", ix))
                    .size(px(32.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .text_size(px(12.0))
                    .when(date.month() != month.month(), |d| {
                        d.text_color(rgba(th.text_faint))
                    })
                    .when(early, |d| d.opacity(0.38))
                    .when(date == today && !selected, |d| {
                        d.text_color(rgba(th.accent))
                    })
                    .when(selected, |d| {
                        d.bg(rgba(th.accent)).text_color(rgba(th.on_accent))
                    })
                    .when(!early, |d| {
                        d.cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if pick == Pick::StartDay {
                                    this.set_start_day(date, cx);
                                } else if let Some(draft) = &mut this.calendar.draft {
                                    draft.end_day = date;
                                    draft.pick = None;
                                    cx.notify();
                                }
                            }))
                    })
                    .child(format::number(date.day() as u64))
            })
            .collect::<Vec<_>>();
        let step = |months: i32| {
            move |this: &mut Self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>| {
                if let Some(draft) = &mut this.calendar.draft
                    && let Ok(m) = draft
                        .pick_month
                        .first_of_month()
                        .checked_add(jiff::Span::new().months(months))
                {
                    draft.pick_month = m;
                }
                cx.notify();
            }
        };
        let weekdays = format::weekdays_short().into_iter().map(|(_, d)| {
            div()
                .size(px(32.0))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(11.0))
                .text_color(rgba(th.text_dim))
                .child(d)
        });
        div()
            .px(px(12.0))
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .pl(px(4.0))
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .child(format::month_year(month)),
                    )
                    .child(
                        icon_button("draft-month-prev", "chevron-left", 20.0, th)
                            .size(px(28.0))
                            .on_click(cx.listener(step(-1))),
                    )
                    .child(
                        icon_button("draft-month-next", "chevron-right", 20.0, th)
                            .size(px(28.0))
                            .on_click(cx.listener(step(1))),
                    ),
            )
            .child(
                div()
                    .w(px(7.0 * 32.0))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .children(weekdays)
                    .children(days),
            )
            .into_any_element()
    }

    fn render_scope_ask(&self, ask: &ScopeAsk, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let title = match ask.change {
            ScopeChange::Delete(_) => tr!("calendar-scope-delete-title"),
            ScopeChange::Respond(..) => tr!("calendar-scope-respond-title"),
            ScopeChange::Save(..) => tr!("calendar-scope-edit-title"),
        };
        let options = [
            (EditScope::This, tr!("calendar-scope-this")),
            (EditScope::Following, tr!("calendar-scope-following")),
            (EditScope::All, tr!("calendar-scope-all")),
        ]
        .into_iter()
        .map(|(scope, text)| {
            let on = ask.scope == scope;
            div()
                .id(("scope", scope as usize))
                .h(px(40.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .cursor_pointer()
                .text_size(px(14.0))
                .child(radio(if on { 1.0 } else { 0.0 }, th))
                .child(text)
                .on_click(cx.listener(move |this, _, _, cx| {
                    if let Some(ask) = &mut this.calendar.ask {
                        ask.scope = scope;
                    }
                    cx.notify();
                }))
        })
        .collect::<Vec<_>>();
        let dialog = raised(
            div()
                .id("scope-ask")
                .occlude()
                .w(px(360.0))
                .p(px(24.0))
                .flex()
                .flex_col()
                .gap(px(8.0))
                .text_color(rgba(th.text)),
            th,
            15.0,
            4.0,
        )
        // The frosted glass is the dialog's first child, under the text.
        .child(div().mb(px(8.0)).text_size(px(20.0)).child(title))
        .children(options)
        .child(
            div()
                .mt(px(16.0))
                .flex()
                .flex_row()
                .justify_end()
                .gap(px(8.0))
                .child(
                    text_button("scope-cancel", tr!("calendar-cancel"), th).on_click(
                        cx.listener(|this, _, window, cx| this.answer_scope(false, window, cx)),
                    ),
                )
                .child(filled_button("scope-ok", tr!("calendar-ok"), th).on_click(
                    cx.listener(|this, _, window, cx| this.answer_scope(true, window, cx)),
                )),
        );
        deferred(
            div()
                .id("scope-scrim")
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgba(0x0000_0052))
                .occlude()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, window, cx| this.answer_scope(false, window, cx)),
                )
                .child(dialog.on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())),
        )
        .with_priority(7)
        .into_any_element()
    }
}

/// Focuses `input` once the click that opened it is over, which
/// otherwise gives the focus to the page under it.
fn focus_later(input: &Entity<TextInput>, window: &mut Window, cx: &mut Context<MailWindow>) {
    let handle = input.focus_handle(cx);
    window.defer(cx, move |window, cx| window.focus(&handle, cx));
}

/// The draft's length in minutes.
fn self_length(draft: &Draft) -> i64 {
    let start = DateTime::from_parts(draft.start_day, draft.start_time);
    let end = DateTime::from_parts(draft.end_day, draft.end_time);
    end.since(start)
        .ok()
        .and_then(|span| span.total(jiff::Unit::Minute).ok())
        .map_or(NEW_EVENT_MINUTES, |m| m as i64)
        .max(0)
}

/// The name of an event kind, as its tab says.
pub(super) fn kind_label(kind: EventKind) -> String {
    match kind {
        EventKind::Focus => tr!("calendar-kind-focus"),
        EventKind::OutOfOffice => tr!("calendar-kind-out-of-office"),
        EventKind::WorkingLocation => tr!("calendar-kind-working-location"),
        EventKind::Default | EventKind::Birthday => tr!("calendar-kind-event"),
    }
}

/// The title a new event of `kind` starts with; empty for an event.
fn kind_title(kind: EventKind) -> String {
    match kind {
        EventKind::Focus => tr!("calendar-kind-focus"),
        EventKind::OutOfOffice => tr!("calendar-kind-out-of-office"),
        EventKind::WorkingLocation => tr!("calendar-working-home"),
        EventKind::Default | EventKind::Birthday => String::new(),
    }
}

/// The icon beside an event of `kind` in the views; none for events.
pub(super) fn kind_icon(kind: EventKind) -> Option<&'static str> {
    match kind {
        EventKind::Focus => Some("headphones"),
        EventKind::OutOfOffice => Some("flight"),
        EventKind::WorkingLocation => Some("home"),
        EventKind::Birthday => Some("cake"),
        EventKind::Default => None,
    }
}

/// A dropdown's face: its value, which opens its list on a click.
fn chip(
    id: impl Into<gpui::ElementId>,
    label: impl Into<SharedString>,
    open: bool,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
    chip_with_dot(id, label, open, None, th)
}

/// A [`chip`] with a colored dot before its value.
fn chip_with_dot(
    id: impl Into<gpui::ElementId>,
    label: impl Into<SharedString>,
    open: bool,
    dot: Option<u32>,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .h(px(36.0))
        .px(px(10.0))
        .gap(px(8.0))
        .when_some(dot, |d, color| {
            d.child(div().size(px(12.0)).rounded_full().bg(rgba(color)))
        })
        .flex()
        .flex_row()
        .items_center()
        .rounded(px(4.0))
        .text_size(px(14.0))
        .text_color(rgba(th.text))
        .cursor_pointer()
        .when(open, |d| d.bg(rgba(th.nav_selected)))
        .hover(|s| s.bg(rgba(th.hover)))
        .child(label.into())
}

fn text_button(
    id: impl Into<gpui::ElementId>,
    label: impl Into<SharedString>,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .h(px(36.0))
        .px(px(16.0))
        .flex()
        .items_center()
        .rounded_full()
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(th.accent))
        .cursor_pointer()
        .relative()
        .child(crate::widgets::hover_fade("hover-glow", None, th))
        .child(label.into())
}

fn dash(th: &Theme) -> AnyElement {
    div()
        .text_size(px(14.0))
        .text_color(rgba(th.text_dim))
        .child("–")
        .into_any_element()
}

/// The event as saved, at `occurrence`'s own times, which a series' row
/// does not have.
fn occurrence_edit(occurrence: &Occurrence) -> EventEdit {
    let mut edit = EventEdit::of(&occurrence.event.data);
    edit.start = occurrence.start;
    edit.end = occurrence.end;
    edit
}

/// The time at `y` pixels into a day of the grid, to 15 minutes.
pub(super) fn time_at(y: f32, hour_height: f32) -> Time {
    let minutes = ((y / hour_height) * 60.0).max(0.0) as i64;
    let minutes = (minutes / i64::from(TIME_STEP)) * i64::from(TIME_STEP);
    add_minutes(
        Time::midnight(),
        minutes.min(24 * 60 - i64::from(TIME_STEP)),
    )
    .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeats_read_back_from_their_rules() {
        let tuesday = Date::constant(2026, 9, 29);
        for repeat in Repeat::OFFERED {
            assert_eq!(Repeat::of(&repeat.rule(tuesday), tuesday), repeat);
        }
        assert_eq!(Repeat::Weekly.rule(tuesday), "FREQ=WEEKLY;BYDAY=TU");
        // The 29th is the fifth Tuesday: the last one.
        assert_eq!(Repeat::Monthly.rule(tuesday), "FREQ=MONTHLY;BYDAY=-1TU");
        assert_eq!(
            Repeat::Monthly.rule(Date::constant(2026, 9, 15)),
            "FREQ=MONTHLY;BYDAY=3TU"
        );
        assert_eq!(
            Repeat::of("FREQ=WEEKLY;INTERVAL=2;BYDAY=TU", tuesday),
            Repeat::Kept("FREQ=WEEKLY;INTERVAL=2;BYDAY=TU".into())
        );
    }

    #[test]
    fn grid_clicks_land_on_quarter_hours() {
        assert_eq!(
            time_at(48.0 * 9.0 + 20.0, 48.0),
            Time::constant(9, 15, 0, 0)
        );
        assert_eq!(time_at(-5.0, 48.0), Time::midnight());
        assert_eq!(time_at(48.0 * 30.0, 48.0), Time::constant(23, 45, 0, 0));
    }
}
