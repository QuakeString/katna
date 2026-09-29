// SPDX-License-Identifier: GPL-3.0-or-later

//! Tasks on the Calendar (`docs/ARCHITECTURE.md` §18.1), as Google
//! Calendar shows them: a task due on a day sits with that day's whole-day
//! events, one due at a time sits at that time; the circle ticks it off,
//! a click opens it, and dragging it to another day or time moves its due
//! day and time, blocking that time for it.

use gpui::{
    AnyElement, ClickEvent, Context, Div, ElementId, FontWeight, MouseButton, Pixels, Point,
    SharedString, Stateful, div, prelude::*, rgba,
};
use jiff::ToSpan;
use jiff::civil::{Date, Time};
use katna_i18n::{format, tr};
use katna_store::tasks::Task as TaskItem;
use katna_ui::px;

use super::super::MailWindow;
use super::super::event_edit::time_at;
use super::menu::CalTarget;
use super::{ALL_DAY_LINE, MONTH_LINE};
use crate::theme::{Theme, fade, mix};
use crate::widgets::icon;

/// How much time a task with a time takes on the day grid.
const TASK_MINUTES: u32 = 30;

/// A task being dragged on the day grid.
pub(in crate::window) struct TaskDrag {
    id: i64,
    /// Minutes between the task's time and the pointer's when it was
    /// picked up, so it moves with the pointer instead of jumping to it.
    grab: i64,
    /// Where it would go now: the day, and the time or `None` for the
    /// whole day. `None` until it leaves its place.
    to: Option<(Date, Option<u32>)>,
}

/// The day `task` is due.
fn due_day(task: &TaskItem) -> Option<Date> {
    task.due.parse().ok()
}

/// `minutes` after midnight as a time of day.
fn clock(minutes: u32) -> Time {
    Time::new(
        i8::try_from(minutes / 60).unwrap_or(0),
        i8::try_from(minutes % 60).unwrap_or(0),
        0,
        0,
    )
    .unwrap_or(Time::midnight())
}

impl MailWindow {
    /// Where `task` shows: its due day and time, or where it is being
    /// dragged to.
    fn task_place(&self, task: &TaskItem) -> Option<(Date, Option<u32>)> {
        let dragged = self.calendar.task_drag.as_ref().filter(|d| d.id == task.id);
        match dragged.and_then(|d| d.to) {
            Some(to) => Some(to),
            None => due_day(task).map(|day| (day, task.due_time)),
        }
    }

    /// The tasks on `day` with whether each is ticked off: the whole-day
    /// ones first, then by time.
    pub(super) fn tasks_on(&self, day: Date) -> Vec<(TaskItem, bool, Option<u32>)> {
        let mut tasks: Vec<(TaskItem, bool, Option<u32>)> = self
            .dated_tasks()
            .into_iter()
            .filter_map(|(task, done)| {
                let (on, time) = self.task_place(task)?;
                (on == day).then(|| (task.clone(), done, time))
            })
            .collect();
        tasks.sort_by(|a, b| (a.2, &a.0.title).cmp(&(b.2, &b.0.title)));
        tasks
    }

    /// A task's chip: the circle that ticks it off and its title; a click
    /// opens it, and on the day grid (`drag`) pressing it picks it up.
    fn task_chip(
        &self,
        id: impl Into<ElementId>,
        task: &TaskItem,
        done: bool,
        drag: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let task_id = task.id;
        let fill = if done {
            mix(th.surface, th.accent, 0.35)
        } else {
            th.accent
        };
        let title = if task.title.is_empty() {
            tr!("calendar-no-title")
        } else {
            task.title.clone()
        };
        let circle = div()
            .id(SharedString::from(format!("cal-task-tick-{task_id}")))
            .flex_none()
            .size(px(14.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .border_1()
            .border_color(rgba(th.on_accent))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(fade(th.on_accent, 0.3))))
            .tooltip(crate::widgets::tip(
                tr!(if done {
                    "tasks-mark-open"
                } else {
                    "tasks-mark-done"
                }),
                th,
            ))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_click(cx.listener(move |this, _, _, cx| {
                cx.stop_propagation();
                this.task_toggle_done(task_id, cx);
            }))
            .when(done, |d| d.child(icon("check", th.on_accent, 12.0)));
        div()
            .id(id)
            .px(px(6.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .rounded(px(4.0))
            .bg(rgba(fill))
            .text_color(rgba(th.on_accent))
            .text_size(px(12.0))
            .font_weight(FontWeight::MEDIUM)
            .cursor_pointer()
            .hover(|s| s.shadow(crate::widgets::elevation(th, 1.0)))
            .on_mouse_down(
                MouseButton::Right,
                self.calendar_menu_on(CalTarget::Task(task_id), cx),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, event: &gpui::MouseDownEvent, _, cx| {
                    cx.stop_propagation();
                    if drag {
                        this.start_task_drag(task_id, event.position);
                    }
                }),
            )
            .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                cx.stop_propagation();
                // The end of a drag, not a click.
                if this
                    .calendar
                    .task_drag
                    .as_ref()
                    .is_some_and(|d| d.to.is_some())
                {
                    return;
                }
                this.task_open_details(task_id, window, cx);
            }))
            .child(circle)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .when(done, |d| d.line_through())
                    .child(title),
            )
    }

    /// The whole-day tasks of `days` in the day grid's top row, in lanes
    /// under the `first_lane` lanes of whole-day events. Returns how many
    /// lanes they take.
    pub(super) fn render_all_day_tasks(
        &self,
        days: &[Date],
        first_lane: usize,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> (usize, Vec<AnyElement>) {
        let count = days.len().max(1) as f32;
        let mut lanes = 0;
        let mut bars = Vec::new();
        for (ix, &day) in days.iter().enumerate() {
            let untimed: Vec<_> = self
                .tasks_on(day)
                .into_iter()
                .filter(|(_, _, time)| time.is_none())
                .collect();
            lanes = lanes.max(untimed.len());
            for (lane, (task, done, _)) in untimed.into_iter().enumerate() {
                let chip = self.task_chip(
                    SharedString::from(format!("cal-task-day-{}", task.id)),
                    &task,
                    done,
                    true,
                    th,
                    cx,
                );
                bars.push(
                    div()
                        .absolute()
                        .top(px(2.0 + (first_lane + lane) as f32 * ALL_DAY_LINE))
                        .left(gpui::relative(ix as f32 / count))
                        .w(gpui::relative(1.0 / count))
                        .h(px(ALL_DAY_LINE - 2.0))
                        .px(px(2.0))
                        .child(chip.size_full())
                        .into_any_element(),
                );
            }
        }
        (lanes, bars)
    }

    /// The tasks due at a time on `day`, in its column of the day grid,
    /// beside the timed events there (`events`, as minutes after midnight).
    pub(super) fn render_timed_tasks(
        &self,
        day: Date,
        events: &[(i64, i64)],
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        self.tasks_on(day)
            .into_iter()
            .filter_map(|(task, done, time)| {
                let at = time?;
                let (from, to) = (i64::from(at), i64::from(at + TASK_MINUTES));
                // Beside an event it overlaps, as Google puts it.
                let beside = events.iter().any(|&(a, b)| a < to && b > from);
                let dragged = self
                    .calendar
                    .task_drag
                    .as_ref()
                    .is_some_and(|d| d.id == task.id && d.to.is_some());
                let chip = self.task_chip(
                    SharedString::from(format!("cal-task-at-{}", task.id)),
                    &task,
                    done,
                    true,
                    th,
                    cx,
                );
                Some(
                    div()
                        .absolute()
                        .top(px(at as f32 / 60.0 * self.calendar.hour + 1.0))
                        .h(px(TASK_MINUTES as f32 / 60.0 * self.calendar.hour - 2.0))
                        .left(gpui::relative(if beside { 0.5 } else { 0.0 }))
                        .right(px(8.0))
                        .when(dragged, |d| {
                            d.opacity(0.85).shadow(crate::widgets::elevation(th, 3.0))
                        })
                        .child(chip.size_full())
                        .into_any_element(),
                )
            })
            .collect()
    }

    /// The tasks on `day` in a Month view cell, `room` lines at most.
    pub(super) fn render_month_tasks(
        &self,
        tasks: Vec<(TaskItem, bool, Option<u32>)>,
        day: Date,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        tasks
            .into_iter()
            .map(|(task, done, time)| {
                let label = match time {
                    Some(at) => tr!(
                        "calendar-short-event",
                        title = task.title.clone(),
                        time = format::time(day.to_datetime(clock(at)))
                    ),
                    None => task.title.clone(),
                };
                let mut shown = task.clone();
                shown.title = label;
                self.task_chip(
                    SharedString::from(format!("cal-task-month-{}", task.id)),
                    &shown,
                    done,
                    false,
                    th,
                    cx,
                )
                .h(px(MONTH_LINE))
                .mx(px(4.0))
                .into_any_element()
            })
            .collect()
    }

    /// The tasks on `day` in the Schedule view, laid out as its events are.
    pub(super) fn render_schedule_tasks(
        &self,
        day: Date,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        self.tasks_on(day)
            .into_iter()
            .map(|(task, done, time)| {
                let id = task.id;
                let when = match time {
                    Some(at) => format::time(day.to_datetime(clock(at))),
                    None => tr!("calendar-all-day"),
                };
                let title = if task.title.is_empty() {
                    tr!("calendar-no-title")
                } else {
                    task.title.clone()
                };
                div()
                    .id(SharedString::from(format!("schedule-task-{id}")))
                    .h(px(40.0))
                    .px(px(12.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(16.0))
                    .rounded(px(8.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .on_mouse_down(
                        MouseButton::Right,
                        self.calendar_menu_on(CalTarget::Task(id), cx),
                    )
                    .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                        this.task_open_details(id, window, cx);
                    }))
                    .child(
                        div()
                            .id(SharedString::from(format!("schedule-task-tick-{id}")))
                            .flex_none()
                            .size(px(14.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_full()
                            .border_1()
                            .border_color(rgba(th.accent))
                            .when(done, |d| d.bg(rgba(th.accent)))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                cx.stop_propagation();
                                this.task_toggle_done(id, cx);
                            }))
                            .when(done, |d| d.child(icon("check", th.on_accent, 10.0))),
                    )
                    .child(
                        div()
                            .flex_none()
                            .w(px(148.0))
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
                            .text_color(rgba(if done { th.text_dim } else { th.text }))
                            .when(done, |d| d.line_through())
                            .child(title),
                    )
                    .into_any_element()
            })
            .collect()
    }

    /// Where the pointer is on the day grid: the day under it, and the
    /// minutes after midnight, or `None` above the hours (the whole-day
    /// row).
    pub(super) fn grid_point(&self, at: Point<Pixels>) -> Option<(Date, Option<i64>)> {
        let (first, end) = self.calendar.days();
        let count = i64::from((end - first).get_days().max(1));
        let scroll = &self.calendar.grid_scroll;
        let bounds = scroll.bounds();
        let width = katna_ui::unpx(bounds.size.width) - self.gutter();
        if width <= 0.0 {
            return None;
        }
        let x = katna_ui::unpx(at.x - bounds.left()) - self.gutter();
        let column = ((x / (width / count as f32)).floor() as i64).clamp(0, count - 1);
        let day = first.checked_add(column.days()).ok()?;
        let minutes = (at.y >= bounds.top()).then(|| {
            let y = katna_ui::unpx(at.y - bounds.top() - scroll.offset().y);
            let time = time_at(y, self.calendar.hour);
            i64::from(time.hour()) * 60 + i64::from(time.minute())
        });
        Some((day, minutes))
    }

    fn start_task_drag(&mut self, id: i64, at: Point<Pixels>) {
        let Some((task, _)) = self.dated_tasks().into_iter().find(|(t, _)| t.id == id) else {
            return;
        };
        let pointer = self.grid_point(at).and_then(|(_, minutes)| minutes);
        // A whole-day task is held by its middle once it has a time.
        let grab = match (task.due_time, pointer) {
            (Some(time), Some(pointer)) => pointer - i64::from(time),
            _ => i64::from(TASK_MINUTES / 2),
        };
        self.calendar.task_drag = Some(TaskDrag { id, grab, to: None });
    }

    /// Follows the pointer while a task is dragged: the day under it, and
    /// the quarter hour, or the whole day in the top row.
    pub(super) fn drag_task_to(&mut self, at: Point<Pixels>, cx: &mut Context<Self>) {
        let Some(drag) = &self.calendar.task_drag else {
            return;
        };
        let (id, grab) = (drag.id, drag.grab);
        let Some((day, minutes)) = self.grid_point(at) else {
            return;
        };
        let last = i64::from(24 * 60 - TASK_MINUTES);
        let time = minutes.map(|m| {
            let m = (m - grab).clamp(0, last);
            u32::try_from(m / 15 * 15).unwrap_or(0)
        });
        let Some((task, _)) = self.dated_tasks().into_iter().find(|(t, _)| t.id == id) else {
            return;
        };
        let home = due_day(task).map(|d| (d, task.due_time));
        let to = Some((day, time)).filter(|to| Some(*to) != home);
        let Some(drag) = &mut self.calendar.task_drag else {
            return;
        };
        if drag.to != to {
            drag.to = to;
            cx.notify();
        }
    }

    /// Lets go of a dragged task: it is due where it was dropped.
    pub(super) fn drop_dragged_task(&mut self, cx: &mut Context<Self>) {
        let Some(drag) = self.calendar.task_drag.take() else {
            return;
        };
        cx.notify();
        if let Some((day, time)) = drag.to {
            self.task_move_due(drag.id, day.to_string(), time, cx);
        }
    }
}
