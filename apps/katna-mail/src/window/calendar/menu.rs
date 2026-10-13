// SPDX-License-Identifier: GPL-3.0-or-later

//! The Calendar page's right-click menus, in the mail list's menu card
//! (`window/context_menu.rs`), so they open where the pointer is and fit
//! the window the same way:
//!
//! - on a free time or day: a new event, focus time or out of office
//!   there, and the day on its own;
//! - on an event: its details, edit, duplicate and delete, Going? (Yes, No,
//!   Maybe), join the call, email the guests, its color and calendar, and
//!   the browser or the contact;
//! - on a task: its details, done, star, another day and delete;
//! - in the side panel, on a calendar or an account (`side_menu.rs`).
//!
//! Changes say so with Undo, as the same changes made elsewhere do.

use gpui::{
    AnyElement, ClickEvent, Context, MouseDownEvent, Pixels, Point, SharedString, Window, div,
    prelude::*, rgba,
};
use jiff::civil::{Date, Time};
use jiff::{ToSpan, Zoned};
use katna_core::config::AppKind;
use katna_dav::Occurrence;
use katna_i18n::tr;
use katna_store::calendar::EventKind;
use katna_ui::px;

use super::super::MailWindow;
use super::super::context_menu::{Rows, Sub, menu_row, menu_row_with};
use super::{CalView, OpenEvent, birthdays, calendar_color, calendar_name, parse_color};
use crate::theme::Theme;
use crate::widgets::icon;

/// What a right-click on the Calendar page was on.
#[derive(Clone)]
pub(in crate::window) enum CalTarget {
    /// A free time on `day`, or the whole day when `time` is `None`.
    Slot {
        day: Date,
        time: Option<Time>,
    },
    Event(Box<Occurrence>),
    /// A task, by its id.
    Task(i64),
    /// A calendar in the side panel.
    Calendar(i64),
    /// An account's heading in the side panel (`None`: this computer).
    Account(Option<i64>),
}

impl CalTarget {
    /// Tells the menus apart, so each opens with its own animation.
    pub(in crate::window) fn key(&self) -> String {
        match self {
            CalTarget::Slot { day, time } => format!("slot-{day}-{time:?}"),
            CalTarget::Event(o) => format!("event-{}-{}", o.event.id, o.start),
            CalTarget::Task(id) => format!("task-{id}"),
            CalTarget::Calendar(id) => format!("calendar-{id}"),
            CalTarget::Account(id) => format!("account-{id:?}"),
        }
    }
}

/// Google Calendar's event colors, in its order.
pub(super) const COLORS: [&str; 11] = [
    "#d50000", "#e67c73", "#f4511e", "#f6bf26", "#33b679", "#0b8043", "#039be5", "#3f51b5",
    "#7986cb", "#8e24aa", "#616161",
];

/// The name of color `ix` of [`COLORS`], as Google Calendar calls it.
pub(super) fn color_name(ix: usize) -> String {
    match ix {
        0 => tr!("calendar-color-tomato"),
        1 => tr!("calendar-color-flamingo"),
        2 => tr!("calendar-color-tangerine"),
        3 => tr!("calendar-color-banana"),
        4 => tr!("calendar-color-sage"),
        5 => tr!("calendar-color-basil"),
        6 => tr!("calendar-color-peacock"),
        7 => tr!("calendar-color-blueberry"),
        8 => tr!("calendar-color-lavender"),
        9 => tr!("calendar-color-grape"),
        _ => tr!("calendar-color-graphite"),
    }
}

/// A dot of `color` in place of an icon.
pub(super) fn dot(color: u32) -> AnyElement {
    div()
        .size(px(14.0))
        .rounded_full()
        .bg(rgba(color))
        .into_any_element()
}

/// A tick for the choice in force, else room for one.
pub(super) fn tick(on: bool, th: &Theme) -> AnyElement {
    if on {
        icon("check", th.accent, 20.0)
    } else {
        div().size(px(20.0)).into_any_element()
    }
}

impl MailWindow {
    /// A listener opening the right-click menu on `target`.
    pub(super) fn calendar_menu_on(
        &self,
        target: CalTarget,
        cx: &Context<Self>,
    ) -> impl Fn(&MouseDownEvent, &mut Window, &mut gpui::App) + 'static {
        cx.listener(move |this, event: &MouseDownEvent, _, cx| {
            this.open_calendar_menu(target.clone(), event.position, cx)
        })
    }

    /// Opens the right-click menu on `target` where the pointer is. A new
    /// event's small card or an event's card gives way to it.
    pub(super) fn open_calendar_menu(
        &mut self,
        target: CalTarget,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        cx.stop_propagation();
        self.calendar.open = None;
        self.close_quick_draft();
        self.open_calendar_context_menu(target, at, cx);
    }

    /// The menu's own items for `target`, at `rh` each, with where each
    /// submenu's row starts.
    pub(in crate::window) fn calendar_menu_rows(
        &self,
        target: &CalTarget,
        rh: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> (Rows, Vec<(Sub, f32)>) {
        let mut rows = Rows::new(rh);
        let mut parents = Vec::new();
        let item = |id: &'static str, name: &str, label: String| {
            self.context_item(id, name, label, rh, th, cx)
        };
        match target {
            CalTarget::Calendar(_) | CalTarget::Account(_) => {
                return self.side_menu_rows(target, rh, th, cx);
            }
            CalTarget::Slot { day, time } => {
                let (day, time) = (*day, *time);
                let new = |kind: Option<EventKind>| {
                    cx.listener(move |this: &mut Self, _: &ClickEvent, window, cx| {
                        this.calendar_new_at(day, time, kind, window, cx)
                    })
                };
                rows.item(
                    item("cal-new-event", "event", tr!("calendar-menu-new-event"))
                        .on_click(new(Some(EventKind::Default))),
                );
                if self.config.app_on(AppKind::Tasks) {
                    rows.item(
                        item("cal-new-task", "tasks", tr!("calendar-kind-task"))
                            .on_click(new(None)),
                    );
                }
                if time.is_some() {
                    rows.item(
                        item("cal-new-focus", "headphones", tr!("calendar-kind-focus"))
                            .on_click(new(Some(EventKind::Focus))),
                    );
                }
                rows.item(
                    item("cal-new-away", "flight", tr!("calendar-kind-out-of-office"))
                        .on_click(new(Some(EventKind::OutOfOffice))),
                );
                if self.calendar.view != CalView::Day || self.calendar.day != day {
                    rows.rule(th);
                    rows.item(
                        item("cal-open-day", "today", tr!("calendar-menu-open-day")).on_click(
                            cx.listener(move |this, _, _, cx| {
                                this.close_context_menu(cx);
                                this.open_calendar_day(day, Some(CalView::Day), cx);
                            }),
                        ),
                    );
                }
            }
            CalTarget::Event(occurrence) => {
                let data = &occurrence.event.data;
                let birthday = occurrence.event.calendar_id == birthdays::BIRTHDAYS;
                let editable = self.can_edit(occurrence) && !birthday;
                let me = data.attendees.iter().find(|a| a.is_self);
                let invited = editable && me.is_some_and(|a| !a.organizer);
                let emails = !self.other_guests(occurrence).is_empty();
                let moves = editable
                    && self
                        .calendar
                        .calendars
                        .iter()
                        .any(|c| c.id != occurrence.event.calendar_id && c.access.can_edit());
                rows.item(
                    item("cal-details", "info", tr!("calendar-event-details")).on_click(
                        cx.listener(|this, _, _, cx| {
                            if let Some((CalTarget::Event(o), at)) = this.take_calendar_target() {
                                this.open_calendar_event(*o, at, cx);
                            }
                        }),
                    ),
                );
                if editable {
                    rows.item(item("cal-edit", "compose", tr!("calendar-edit")).on_click(
                        cx.listener(|this, _, window, cx| {
                            if this.open_menu_event() {
                                this.edit_open_event(window, cx);
                            }
                        }),
                    ));
                    rows.item(
                        item("cal-duplicate", "copy", tr!("calendar-menu-duplicate")).on_click(
                            cx.listener(|this, _, window, cx| {
                                if let Some((CalTarget::Event(o), at)) = this.take_calendar_target()
                                {
                                    this.duplicate_event(*o, at, window, cx);
                                }
                            }),
                        ),
                    );
                    rows.item(
                        item("cal-delete", "trash", tr!("calendar-delete")).on_click(cx.listener(
                            |this, _, _, cx| {
                                if this.open_menu_event() {
                                    this.delete_open_event(cx);
                                }
                            },
                        )),
                    );
                }
                let join = data.join_url.clone();
                if invited || !join.is_empty() || emails || editable {
                    rows.rule(th);
                }
                if invited {
                    parents.push((
                        Sub::Answer,
                        rows.item(self.context_parent(Sub::Answer, rh, th, cx)),
                    ));
                }
                if !join.is_empty() {
                    rows.item(item("cal-join", "video", tr!("calendar-join")).on_click(
                        cx.listener(move |this, _, _, cx| {
                            this.close_context_menu(cx);
                            cx.open_url(&join);
                        }),
                    ));
                }
                if emails {
                    rows.item(
                        item("cal-email", "mail", tr!("calendar-email-guests")).on_click(
                            cx.listener(|this, _, window, cx| {
                                if this.open_menu_event() {
                                    this.email_guests(false, window, cx);
                                }
                            }),
                        ),
                    );
                }
                if editable {
                    parents.push((
                        Sub::Color,
                        rows.item(self.context_parent(Sub::Color, rh, th, cx)),
                    ));
                }
                if moves {
                    parents.push((
                        Sub::Calendar,
                        rows.item(self.context_parent(Sub::Calendar, rh, th, cx)),
                    ));
                }
                let web = data.web_link.clone();
                if !web.is_empty() || birthday {
                    rows.rule(th);
                }
                if !web.is_empty() {
                    rows.item(
                        item("cal-web", "open-external", tr!("calendar-open-web")).on_click(
                            cx.listener(move |this, _, _, cx| {
                                this.close_context_menu(cx);
                                cx.open_url(&web);
                            }),
                        ),
                    );
                }
                if birthday {
                    let card = occurrence.event.id;
                    rows.item(
                        item("cal-contact", "contacts", tr!("calendar-open-contact")).on_click(
                            cx.listener(move |this, _, window, cx| {
                                this.close_context_menu(cx);
                                this.open_birthday_contact(card, window, cx);
                            }),
                        ),
                    );
                }
            }
            CalTarget::Task(id) => {
                let id = *id;
                let (done, starred) = self
                    .tasks_task(id)
                    .map_or((false, false), |(_, done, starred)| (done, starred));
                rows.item(
                    item("cal-task-details", "info", tr!("tasks-details")).on_click(cx.listener(
                        move |this, _, window, cx| {
                            this.close_context_menu(cx);
                            this.task_open_details(id, window, cx);
                        },
                    )),
                );
                let (name, label) = if done {
                    ("check-circle", tr!("tasks-mark-open"))
                } else {
                    ("check", tr!("tasks-mark-done"))
                };
                rows.item(item("cal-task-done", name, label).on_click(cx.listener(
                    move |this, _, _, cx| {
                        this.close_context_menu(cx);
                        this.task_toggle_done(id, cx);
                    },
                )));
                let (name, label) = if starred {
                    ("star-filled", tr!("tasks-unstar"))
                } else {
                    ("star", tr!("tasks-star"))
                };
                rows.item(item("cal-task-star", name, label).on_click(cx.listener(
                    move |this, _, _, cx| {
                        this.close_context_menu(cx);
                        this.task_toggle_star(id, cx);
                    },
                )));
                parents.push((
                    Sub::Date,
                    rows.item(self.context_parent(Sub::Date, rh, th, cx)),
                ));
                rows.rule(th);
                rows.item(
                    item("cal-task-delete", "trash", tr!("tasks-delete")).on_click(cx.listener(
                        move |this, _, _, cx| {
                            this.close_context_menu(cx);
                            this.task_delete(id, cx);
                        },
                    )),
                );
            }
        }
        (rows, parents)
    }

    /// The items of submenu `sub` for `target`.
    pub(in crate::window) fn calendar_sub_rows(
        &self,
        target: &CalTarget,
        sub: Sub,
        rh: f32,
        th: &Theme,
        cx: &Context<Self>,
    ) -> Rows {
        let mut rows = Rows::new(rh);
        match (target, sub) {
            (CalTarget::Calendar(id), Sub::Color) => return self.side_color_rows(*id, rh, th, cx),
            (CalTarget::Event(occurrence), Sub::Answer) => {
                let data = &occurrence.event.data;
                let me = data.attendees.iter().find(|a| a.is_self);
                let mine = if data.self_status.is_empty() {
                    me.map(|a| a.status.clone()).unwrap_or_default()
                } else {
                    data.self_status.clone()
                };
                for (ix, (status, label)) in [
                    ("accepted", tr!("calendar-answer-yes")),
                    ("declined", tr!("calendar-answer-no")),
                    ("tentative", tr!("calendar-answer-maybe")),
                ]
                .into_iter()
                .enumerate()
                {
                    rows.item(
                        menu_row_with(
                            ("cal-answer", ix),
                            tick(mine == status, th),
                            label.into(),
                            th,
                            rh,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if this.open_menu_event() {
                                this.respond_to_event(status, cx);
                            }
                        })),
                    );
                }
            }
            (CalTarget::Event(occurrence), Sub::Color) => {
                let own = occurrence.event.data.color.to_ascii_lowercase();
                let calendar = self
                    .calendar
                    .calendars
                    .iter()
                    .find(|c| c.id == occurrence.event.calendar_id);
                let default = calendar.map_or(th.accent, calendar_color);
                rows.item(
                    menu_row_with(
                        "cal-color-default",
                        dot(default),
                        tr!("calendar-menu-color-calendar").into(),
                        th,
                        rh,
                    )
                    .when(own.is_empty(), |d| d.child(tick(true, th)))
                    .on_click(self.recolor(String::new(), cx)),
                );
                for (ix, hex) in COLORS.into_iter().enumerate() {
                    rows.item(
                        menu_row_with(
                            ("cal-color", ix),
                            dot(parse_color(hex).unwrap_or(default)),
                            color_name(ix).into(),
                            th,
                            rh,
                        )
                        .when(own == hex, |d| d.child(tick(true, th)))
                        .on_click(self.recolor(hex.to_owned(), cx)),
                    );
                }
            }
            (CalTarget::Event(occurrence), Sub::Calendar) => {
                let from = occurrence.event.calendar_id;
                for calendar in self
                    .calendar
                    .calendars
                    .iter()
                    .filter(|c| c.id != from && c.access.can_edit())
                {
                    let to = calendar.id;
                    rows.item(
                        menu_row_with(
                            ("cal-move", to as usize),
                            dot(calendar_color(calendar)),
                            SharedString::from(calendar_name(calendar)),
                            th,
                            rh,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if let Some((CalTarget::Event(o), _)) = this.take_calendar_target() {
                                this.move_event_to(*o, to, cx);
                            }
                        })),
                    );
                }
            }
            (CalTarget::Task(id), Sub::Date) => {
                let id = *id;
                let today = Zoned::now().with_time_zone(self.tz.clone()).date();
                let time = self.tasks_task(id).and_then(|(task, ..)| task.due_time);
                let day = |ix: usize, name: &'static str, label: String, on: Date| {
                    menu_row(("cal-task-day", ix), name, label.into(), th, rh).on_click(
                        cx.listener(move |this, _, _, cx| {
                            this.close_context_menu(cx);
                            this.task_move_due(id, on.to_string(), time, cx);
                        }),
                    )
                };
                let tomorrow = today.tomorrow().unwrap_or(today);
                let week = today.checked_add(7.days()).unwrap_or(today);
                rows.item(day(0, "today", tr!("tasks-due-today"), today));
                rows.item(day(1, "event", tr!("tasks-due-tomorrow"), tomorrow));
                rows.item(day(2, "calendar", tr!("calendar-menu-in-a-week"), week));
                if let Some((task, ..)) = self.tasks_task(id).filter(|_| time.is_some()) {
                    let due = task.due.clone();
                    rows.item(
                        menu_row(
                            "cal-task-all-day",
                            "schedule",
                            tr!("calendar-all-day").into(),
                            th,
                            rh,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.close_context_menu(cx);
                            this.task_move_due(id, due.clone(), None, cx);
                        })),
                    );
                }
                rows.item(
                    menu_row(
                        "cal-task-no-date",
                        "close",
                        tr!("tasks-no-date").into(),
                        th,
                        rh,
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.close_context_menu(cx);
                        this.task_move_due(id, String::new(), None, cx);
                    })),
                );
            }
            _ => {}
        }
        rows
    }

    /// A click giving the menu's event `color`.
    fn recolor(
        &self,
        color: String,
        cx: &Context<Self>,
    ) -> impl Fn(&ClickEvent, &mut Window, &mut gpui::App) + 'static {
        cx.listener(move |this, _, _, cx| {
            if let Some((CalTarget::Event(o), _)) = this.take_calendar_target() {
                this.recolor_event(*o, color.clone(), cx);
            }
        })
    }

    /// Closes the menu and makes its event the open one, as its card's
    /// buttons act on; returns whether it was on an event.
    fn open_menu_event(&mut self) -> bool {
        match self.take_calendar_target() {
            Some((CalTarget::Event(occurrence), at)) => {
                self.calendar.open = Some(OpenEvent {
                    occurrence: *occurrence,
                    at,
                });
                true
            }
            _ => false,
        }
    }

    /// A new event of `kind` at `time` on `day`, or that whole day, in the
    /// small card where the menu was; no kind opens the card on its Task tab.
    fn calendar_new_at(
        &mut self,
        day: Date,
        time: Option<Time>,
        kind: Option<EventKind>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((_, at)) = self.take_calendar_target() else {
            return;
        };
        self.start_new_event(day, time, time.is_none(), at, window, cx);
        match kind {
            None => self.set_draft_task(true, window, cx),
            Some(EventKind::Default) => {}
            Some(kind) => self.set_draft_kind(kind, cx),
        }
    }
}
