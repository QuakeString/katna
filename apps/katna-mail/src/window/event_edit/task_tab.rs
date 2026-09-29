// SPDX-License-Identifier: GPL-3.0-or-later

//! The new-event card's Task tab, as Google Calendar has it: the same
//! title, day and time make a task due then in a task list picked below,
//! instead of an event. The task goes to the daemon as the Tasks page's
//! "Add a task" does (`crate::tasks::TaskCommand`), and shows on the
//! calendar once read back.

use gpui::{AnyElement, ClickEvent, Context, Window, div, prelude::*, rgba};
use jiff::civil::Time;
use katna_i18n::tr;
use katna_ui::px;

use super::super::MailWindow;
use super::{Draft, Pick, Repeat, chip, focus_later, icon};
use crate::daemon::Command;
use crate::tasks::{Column, TaskCommand, TaskEdit};
use crate::theme::Theme;

/// How long a new task's block is on the day grid (as tasks show there).
pub(super) const TASK_BLOCK_MINUTES: f32 = 30.0;

impl MailWindow {
    /// The task lists a new task can go in: every list read.
    fn task_columns(&self) -> &[Column] {
        self.tasks.columns()
    }

    /// The list a new task goes in: the default list of the account of the
    /// new event's calendar, else of the account picked, else the first
    /// default list, else the first list; `0` (the daemon's default list)
    /// while none are read.
    pub(super) fn default_task_list(&self, calendar: i64) -> i64 {
        let columns = self.task_columns();
        let of_calendar = self
            .calendar
            .calendars
            .iter()
            .find(|c| c.id == calendar)
            .and_then(|c| c.account);
        let default_of = |account: Option<katna_core::AccountId>| {
            columns
                .iter()
                .find(|c| account.is_some() && c.list.account == account && c.list.is_default)
        };
        default_of(of_calendar)
            .or_else(|| default_of(self.account()))
            .or_else(|| columns.iter().find(|c| c.list.is_default))
            .or_else(|| columns.first())
            .map_or(0, |c| c.list.id)
    }

    /// Whether there is a task list to add a task to.
    pub(super) fn has_task_lists(&self) -> bool {
        !self.task_columns().is_empty()
    }

    /// Turns the card into a new task (`on`) or back into an event.
    pub(super) fn set_draft_task(&mut self, on: bool, window: &mut Window, cx: &mut Context<Self>) {
        let list = match &self.calendar.draft {
            Some(draft) if draft.task != on && draft.editing.is_none() => {
                (on && draft.task_list == 0).then(|| self.default_task_list(draft.calendar))
            }
            _ => return,
        };
        let Some(draft) = &mut self.calendar.draft else {
            return;
        };
        if on {
            // "Focus time" and the like name an event, not a task.
            let typed = draft.title.read(cx).text().trim().to_owned();
            if !typed.is_empty() && typed == super::kind_title(draft.kind) {
                draft.title.update(cx, |input, cx| input.set_text("", cx));
            }
            draft.kind = Default::default();
            draft.busy = true;
            draft.reminder = Some(10);
            if let Some(list) = list {
                draft.task_list = list;
            }
        }
        draft.task = on;
        draft.pick = None;
        focus_later(&draft.title, window, cx);
        cx.notify();
    }

    /// Adds the card's task: its title, due day and time (none when all
    /// day), description and repeat, in the list picked.
    pub(super) fn save_task_draft(&mut self, cx: &mut Context<Self>) {
        let Some(draft) = &self.calendar.draft else {
            return;
        };
        let title = self.draft_title(draft, cx);
        let notes = draft.notes.read(cx).text().trim().to_owned();
        let due_time = (!draft.all_day).then(|| minutes(draft.start_time));
        let repeat = draft.repeat.rule(draft.start_day);
        let fields = TaskEdit {
            due_time: due_time.map(Some),
            notes: (!notes.is_empty()).then_some(notes),
            repeat: (draft.repeat != Repeat::Never).then_some(repeat),
            ..TaskEdit::default()
        };
        let add = TaskCommand::Add {
            list: draft.task_list,
            parent: None,
            title,
            due: draft.start_day.to_string(),
            mail: String::new(),
        };
        let command = if fields == TaskEdit::default() {
            add
        } else {
            TaskCommand::AddThen(Box::new(add), fields)
        };
        let list = self.task_list_name(draft.task_list);
        let done = if list.is_empty() {
            tr!("calendar-task-added")
        } else {
            tr!("calendar-task-added-to", list = list)
        };
        self.calendar.draft = None;
        self.send(
            Command::Task(Box::new(command)),
            Some(done),
            None,
            false,
            cx,
        );
        cx.notify();
    }

    /// List `id`'s name, as the Tasks page shows it; empty for the
    /// daemon's default list before the lists are read.
    fn task_list_name(&self, id: i64) -> String {
        self.task_columns()
            .iter()
            .find(|c| c.list.id == id)
            .map(super::super::tasks_page::list_title)
            .unwrap_or_default()
    }

    /// The Task tab's rows: the due day and time (or all day), a
    /// description, and the list.
    pub(super) fn render_task_fields(
        &self,
        draft: &Draft,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let row = |name: &'static str| {
            div().flex().flex_row().items_center().gap(px(16.0)).child(
                div()
                    .flex_none()
                    .w(px(24.0))
                    .child(icon(name, th.text_dim, 20.0)),
            )
        };
        let mut when = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(4.0))
            .child(self.date_chip(draft, Pick::StartDay, th, cx));
        if !draft.all_day {
            when = when.child(self.time_chip(draft, Pick::StartTime, th, cx));
        }
        let list = self.task_list_name(draft.task_list);
        let list = if list.is_empty() {
            tr!("tasks-my-tasks")
        } else {
            list
        };
        div()
            .px(px(8.0))
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(row("schedule").child(when))
            .child(div().ml(px(36.0)).child(self.render_all_day(draft, th, cx)))
            .when(draft.repeat != Repeat::Never, |d| {
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
            .child(
                row("notes").items_start().child(
                    div()
                        .flex_1()
                        .mr(px(8.0))
                        .min_h(px(36.0))
                        .px(px(10.0))
                        .py(px(8.0))
                        .rounded(px(4.0))
                        .bg(rgba(th.hover))
                        .text_size(px(14.0))
                        .child(draft.notes.clone()),
                ),
            )
            .child(
                row("tasks").child(
                    chip(
                        "draft-task-list",
                        list,
                        draft.pick.map(|p| p.0) == Some(Pick::TaskList),
                        th,
                    )
                    .on_click(cx.listener(
                        |this, event: &ClickEvent, _, cx| {
                            this.open_pick(Pick::TaskList, event.position(), cx)
                        },
                    )),
                ),
            )
            .into_any_element()
    }

    /// The lists to pick from, each with its account's address under it.
    pub(super) fn render_task_lists(
        &self,
        draft: &Draft,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let items = self
            .task_columns()
            .iter()
            .map(|column| {
                let id = column.list.id;
                let on = id == draft.task_list;
                let account = if column.account.is_empty() {
                    tr!("calendar-task-list-local")
                } else {
                    column.account.clone()
                };
                div()
                    .id(("draft-task-list-item", id as usize))
                    .min_h(px(44.0))
                    .px(px(16.0))
                    .py(px(4.0))
                    .flex()
                    .flex_col()
                    .justify_center()
                    .cursor_pointer()
                    .when(on, |d| d.bg(rgba(th.nav_selected)))
                    .hover(|s| s.bg(rgba(th.hover)))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .child(super::super::tasks_page::list_title(column)),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_dim))
                            .child(account),
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if let Some(draft) = &mut this.calendar.draft {
                            draft.task_list = id;
                            draft.pick = None;
                        }
                        cx.notify();
                    }))
            })
            .collect::<Vec<_>>();
        div()
            .id("draft-task-lists")
            .min_w(px(240.0))
            .max_h(px(320.0))
            .overflow_y_scroll()
            .children(items)
            .into_any_element()
    }
}

/// A new task's due time as minutes after midnight.
fn minutes(time: Time) -> u32 {
    u32::try_from(i32::from(time.hour()) * 60 + i32::from(time.minute())).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn due_times_are_minutes_after_midnight() {
        assert_eq!(minutes(Time::constant(0, 0, 0, 0)), 0);
        assert_eq!(minutes(Time::constant(14, 30, 0, 0)), 870);
    }
}
