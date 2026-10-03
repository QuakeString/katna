// SPDX-License-Identifier: GPL-3.0-or-later

//! Upcoming and Completed, from every list: Upcoming shows what is
//! overdue, then the next fortnight with a heading for each day, and a
//! task dragged onto a day is due then; Completed shows every ticked task
//! by the day it was ticked, and unticking one brings it back.

use gpui::{AnyElement, Context, FontWeight, SharedString, div, prelude::*, rgba};
use jiff::civil::{Date, Time};
use katna_i18n::{format, tr};
use katna_store::tasks::Task as TaskItem;
use katna_ui::px;
use katna_ui::tokens::{space, text};

use super::{
    Placed, Quiet, RowLook, SINGLE_WIDTH, TaskDragged, TasksPage, card_heading, list_title, today,
};
use crate::theme::{Theme, fade};
use crate::widgets::icon;
use crate::window::MailWindow;

/// How many days ahead Upcoming shows, after today.
pub(super) const UPCOMING_DAYS: i64 = 14;

/// At most this many ticked tasks show on Completed, the latest.
const COMPLETED_MOST: usize = 500;

/// A day's tasks on Upcoming.
pub(super) type Day<'a> = (Date, Vec<Placed<'a>>);

/// `minutes` after midnight as a time of day.
fn time_of(minutes: u32) -> Time {
    Time::new(
        i8::try_from(minutes / 60).unwrap_or(0),
        i8::try_from(minutes % 60).unwrap_or(0),
        0,
        0,
    )
    .unwrap_or_default()
}

/// The local day and time of Unix second `at`.
fn local(at: i64) -> Option<jiff::civil::DateTime> {
    let at = jiff::Timestamp::from_second(at).ok()?;
    Some(at.to_zoned(jiff::tz::TimeZone::system()).datetime())
}

impl TasksPage {
    /// The open tasks found (steps too) that are overdue, and those due
    /// on each of the next [`UPCOMING_DAYS`] days after `today`, each by
    /// time (those without one last).
    pub(super) fn upcoming(&self, today: Date) -> (Vec<Placed<'_>>, Vec<Day<'_>>) {
        let mut dated: Vec<(Date, Placed<'_>)> = self
            .columns()
            .iter()
            .flat_map(|c| c.tasks.iter().map(move |t| (c, t)))
            .filter(|(_, t)| !self.done(t) && self.parent_open(t) && self.found(t))
            .filter_map(|(c, t)| Some((t.due.parse().ok()?, (c, t))))
            .collect();
        dated.sort_by_key(|(day, (_, t))| (*day, t.due_time.is_none(), t.due_time));
        let overdue = dated
            .iter()
            .filter(|(day, _)| *day < today)
            .map(|(_, p)| *p)
            .collect();
        let days = (1..=UPCOMING_DAYS)
            .filter_map(|n| today.checked_add(jiff::Span::new().days(n)).ok())
            .map(|day| {
                let tasks = dated
                    .iter()
                    .filter(|(d, _)| *d == day)
                    .map(|(_, p)| *p)
                    .collect();
                (day, tasks)
            })
            .collect();
        (overdue, days)
    }

    /// The ticked tasks found, latest first: steps too, and ticks not yet
    /// read back (as ticked now).
    pub(super) fn completed(&self) -> Vec<Placed<'_>> {
        let now = jiff::Timestamp::now().as_second();
        let mut done: Vec<(i64, Placed<'_>)> = self
            .columns()
            .iter()
            .flat_map(|c| c.tasks.iter().map(move |t| (c, t)))
            .filter(|(_, t)| self.done(t) && self.found(t))
            .map(|(c, t)| (t.done_at.unwrap_or(now), (c, t)))
            .collect();
        done.sort_by_key(|(at, _)| std::cmp::Reverse(*at));
        done.truncate(COMPLETED_MOST);
        done.into_iter().map(|(_, p)| p).collect()
    }

    /// Task `id`'s steps: how many are ticked, of how many.
    fn steps_done(&self, id: i64) -> Option<(usize, usize)> {
        let steps: Vec<&TaskItem> = self
            .columns()
            .iter()
            .flat_map(|c| c.tasks.iter())
            .filter(|t| t.parent == Some(id))
            .collect();
        (!steps.is_empty()).then(|| (steps.iter().filter(|t| self.done(t)).count(), steps.len()))
    }
}

impl MailWindow {
    /// The quiet line under a title on Upcoming and Completed: where it
    /// came from, its time, its steps and its list, between dots.
    pub(super) fn quiet_line(
        &self,
        task: &TaskItem,
        quiet: Quiet,
        list: Option<&str>,
        th: &Theme,
    ) -> gpui::Div {
        let color = th.text_dim;
        let mut parts: Vec<AnyElement> = Vec::new();
        let word = |text: String| div().child(text).into_any_element();
        let with_icon = |name: &'static str, text: String| {
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(space::S2))
                .child(icon(name, color, 14.0))
                .child(text)
                .into_any_element()
        };
        match quiet {
            Quiet::Upcoming { overdue } => {
                if super::super::notes::note_of_task(&task.mail).is_some() {
                    parts.push(with_icon("notes", tr!("tasks-from-note-quiet")));
                } else if !task.mail.is_empty() {
                    parts.push(with_icon("mail", tr!("tasks-from-mail-quiet")));
                }
                let day: Option<Date> = task.due.parse().ok();
                let time = task
                    .due_time
                    .zip(day)
                    .map(|(m, d)| format::time(d.to_datetime(time_of(m))));
                match (overdue, day) {
                    (true, Some(day)) => {
                        let at = day.to_datetime(Time::midnight());
                        let day = tr!(
                            "tasks-upcoming-overdue-day",
                            weekday = format::weekday(at),
                            day = format::day_month(at)
                        );
                        parts.push(word(match time {
                            Some(time) => tr!("tasks-due-at", day = day, time = time),
                            None => day,
                        }));
                    }
                    _ => parts.extend(time.map(word)),
                }
                if let Some((done, all)) = self.tasks.steps_done(task.id) {
                    parts.push(with_icon(
                        "subtask",
                        tr!("tasks-steps-done", done = done as u64, count = all as u64),
                    ));
                }
                if !task.repeat.is_empty() {
                    parts.push(icon("refresh", color, 14.0));
                }
            }
            Quiet::Completed => {}
        }
        parts.extend(list.map(|l| word(l.to_owned())));
        if quiet == Quiet::Completed
            && let Some(at) = task.done_at.and_then(local)
        {
            parts.push(word(format::time(at)));
        }
        let mut line = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .text_size(px(text::CAPTION))
            .line_height(px(text::line_height(text::CAPTION)))
            .text_color(rgba(color));
        for (ix, part) in parts.into_iter().enumerate() {
            if ix > 0 {
                line = line.child(div().px(px(space::S2)).child("·"));
            }
            line = line.child(part);
        }
        line
    }

    /// Upcoming: what is overdue, then each of the next fortnight's days
    /// with its tasks and a row to add one due that day. A task dragged
    /// onto a day is due then.
    pub(super) fn render_upcoming(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let page = &self.tasks;
        let now = today();
        let (overdue, days) = page.upcoming(now);
        let mut body = div().child(card_heading(tr!("tasks-upcoming"), th));
        if !overdue.is_empty() {
            let count = overdue.len();
            let rows: Vec<AnyElement> = overdue
                .into_iter()
                .flat_map(|p| self.upcoming_rows(p, true, th, cx))
                .collect();
            body = body
                .child(section_heading(
                    tr!("tasks-overdue"),
                    format::number(count as u64),
                    th.error,
                    th,
                ))
                .children(rows);
        }
        for (ix, (day, tasks)) in days.into_iter().enumerate() {
            let at = day.to_datetime(Time::midnight());
            let (label, date) = if ix == 0 {
                (
                    tr!("tasks-due-tomorrow"),
                    format::weekday_day_month_long(at),
                )
            } else {
                (format::weekday_long(at), format::day_month_long(at))
            };
            let rows: Vec<AnyElement> = tasks
                .into_iter()
                .flat_map(|p| self.upcoming_rows(p, false, th, cx))
                .collect();
            let due = day.to_string();
            let adding = page
                .adding
                .as_ref()
                .filter(|a| a.parent.is_none() && a.due == due);
            let add = match adding {
                Some(adding) => self.adding_row(adding, false, th, cx),
                None => {
                    let due = due.clone();
                    let label = tr!("tasks-upcoming-add", day = format::weekday_long(at));
                    self.add_row(("tasks-upcoming-add", ix).into(), label, th)
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.task_start_adding(0, None, due.clone(), window, cx)
                        }))
                        .into_any_element()
                }
            };
            let accent = th.accent;
            body = body.child(
                div()
                    .id(("tasks-upcoming-day", ix))
                    .flex()
                    .flex_col()
                    .child(section_heading(label, date, th.text, th))
                    .children(rows)
                    .child(add)
                    // A task dropped on a day is due that day.
                    .drag_over::<TaskDragged>(move |style, _, _, _| {
                        style.bg(rgba(fade(accent, 0.08)))
                    })
                    .on_drop(cx.listener(move |this, dragged: &TaskDragged, _, cx| {
                        this.tasks.drag = None;
                        this.tasks_set_due(&dragged.ids, Some(day), cx);
                    })),
            );
        }
        self.single_card("tasks-upcoming-card".into(), th, body)
    }

    /// A task's row on Upcoming, and the row adding a step to it.
    fn upcoming_rows(
        &self,
        (c, t): Placed<'_>,
        overdue: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let name = list_title(c);
        let look = RowLook {
            list: Some(&name),
            quiet: Some(Quiet::Upcoming { overdue }),
            drag: true,
            star_shown: true,
        };
        let mut rows = vec![self.render_task_row_as(t, look, today(), th, cx)];
        if let Some(adding) = self
            .tasks
            .adding
            .as_ref()
            .filter(|a| a.parent == Some(t.id))
        {
            rows.push(self.adding_row(adding, true, th, cx));
        }
        rows
    }

    /// Completed: every ticked task, from every list, under the day it was
    /// ticked, latest first. Its tick unticks it and it goes back to its
    /// list.
    pub(super) fn render_completed(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let page = &self.tasks;
        let now = today();
        let done = page.completed();
        let mut body = div().child(card_heading(tr!("tasks-completed-view"), th));
        if done.is_empty() {
            body = body.child(
                div()
                    .py(px(space::S6))
                    .px(px(space::S6))
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(space::S3))
                    .text_center()
                    .child(icon("check-circle", th.text_faint, 40.0))
                    .child(
                        self.copyable(tr!("tasks-completed-empty"), th)
                            .text_size(px(text::BODY))
                            .text_color(rgba(th.text_dim)),
                    ),
            );
        }
        let mut last: Option<Date> = None;
        for (c, t) in done {
            let day = t.done_at.and_then(local).map_or(now, |at| at.date());
            if last != Some(day) {
                last = Some(day);
                let heading = if day == now {
                    tr!("tasks-due-today")
                } else {
                    format::weekday_day_month_long(day.to_datetime(Time::midnight()))
                };
                body = body.child(
                    div()
                        .mt(px(space::S3))
                        .px(px(space::S6))
                        .h(px(space::S7))
                        .flex()
                        .items_center()
                        .text_size(px(text::CAPTION))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(th.text_dim))
                        .child(heading.to_uppercase()),
                );
            }
            let name = list_title(c);
            let look = RowLook {
                list: Some(&name),
                quiet: Some(Quiet::Completed),
                ..RowLook::default()
            };
            body = body.child(self.render_task_row_as(t, look, now, th, cx));
        }
        self.single_card("tasks-completed-card".into(), th, body)
    }

    /// One card in the middle of the page, as Today and Starred show.
    fn single_card(&self, id: SharedString, th: &Theme, rows: gpui::Div) -> AnyElement {
        div()
            .size_full()
            .p(px(space::S5))
            .flex()
            .items_start()
            .justify_center()
            .child(self.card_frame(id, SINGLE_WIDTH, th, rows))
            .into_any_element()
    }
}

/// A heading on Upcoming: the day (or Overdue) and, quieter, its date
/// (or count), over a line.
fn section_heading(label: String, after: String, color: u32, th: &Theme) -> gpui::Div {
    div()
        .mt(px(space::S4))
        .mx(px(space::S5))
        .h(px(space::S7 + space::S1))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(space::S3))
        .border_b_1()
        .border_color(rgba(th.divider))
        .child(
            div()
                .text_size(px(text::BODY))
                .font_weight(FontWeight::BOLD)
                .text_color(rgba(color))
                .child(label),
        )
        .child(
            div()
                .text_size(px(text::CAPTION))
                .text_color(rgba(th.text_dim))
                .child(after),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::{Board, Column};
    use katna_store::tasks::TaskList;

    fn page(tasks: Vec<TaskItem>) -> TasksPage {
        let column = Column {
            list: TaskList {
                id: 1,
                account: None,
                title: "My Tasks".into(),
                is_default: true,
            },
            account: String::new(),
            to_do: false,
            tasks,
        };
        TasksPage {
            board: Some(Ok(Board {
                columns: vec![column],
            })),
            ..TasksPage::default()
        }
    }

    fn task(id: i64, due: &str, time: Option<u32>, done_at: Option<i64>) -> TaskItem {
        TaskItem {
            id,
            list: 1,
            title: format!("Task {id}"),
            due: due.into(),
            due_time: time,
            done_at,
            ..TaskItem::default()
        }
    }

    #[test]
    fn upcoming_has_overdue_then_a_fortnight_by_day_and_time() {
        let today: Date = "2026-10-03".parse().unwrap();
        let page = page(vec![
            task(1, "2026-10-01", None, None),
            task(2, "2026-10-03", None, None),
            task(3, "2026-10-04", None, None),
            task(4, "2026-10-04", Some(600), None),
            task(5, "2026-10-17", None, None),
            task(6, "2026-10-18", None, None),
            task(7, "2026-10-05", None, Some(1)),
            task(8, "", None, None),
        ]);
        let (overdue, days) = page.upcoming(today);
        let ids = |tasks: &[Placed<'_>]| tasks.iter().map(|(_, t)| t.id).collect::<Vec<_>>();
        assert_eq!(ids(&overdue), [1]);
        assert_eq!(days.len(), 14);
        assert_eq!(days[0].0.to_string(), "2026-10-04");
        // Timed first; today's and ticked tasks and those past the
        // fortnight are left out.
        assert_eq!(ids(&days[0].1), [4, 3]);
        assert!(days[1].1.is_empty());
        assert_eq!(ids(&days[13].1), [5]);
    }

    #[test]
    fn completed_is_latest_first() {
        let page = page(vec![
            task(1, "", None, Some(100)),
            task(2, "", None, None),
            task(3, "", None, Some(300)),
            task(4, "", None, Some(200)),
        ]);
        let ids: Vec<i64> = page.completed().iter().map(|(_, t)| t.id).collect();
        assert_eq!(ids, [3, 4, 1]);
    }
}
