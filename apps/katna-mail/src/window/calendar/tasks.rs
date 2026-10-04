// SPDX-License-Identifier: GPL-3.0-or-later

//! Tasks on the Calendar (`docs/ARCHITECTURE.md` §18.1), as Google
//! Calendar shows them: a task due on a day sits with that day's whole-day
//! events, one due at a time sits at that time; the circle ticks it off,
//! a click opens it, and dragging it to another day or time moves its due
//! day and time, blocking that time for it. In Month view a task dragged
//! to another day keeps its time. The side list's Tasks switch hides them
//! all, remembered on this computer as Birthdays is.

use gpui::{
    AnyElement, ClickEvent, Context, Div, ElementId, FontWeight, MouseButton, MouseMoveEvent,
    Pixels, Point, SharedString, Stateful, Window, div, prelude::*, rgba,
};
use jiff::ToSpan;
use jiff::civil::{Date, Time};
use katna_i18n::{format, tr};
use katna_store::tasks::Task as TaskItem;
use katna_ui::px;
use katna_ui::tokens::{elevation, radius, space, text};

use super::super::MailWindow;
use super::super::event_edit::time_at;
use super::menu::CalTarget;
use super::{ALL_DAY_LINE, MONTH_LINE};
use crate::theme::{Theme, fade, mix};
use crate::widgets::{FocusRing, icon};

/// A task on a day: whether it is ticked off, and its time.
type DayTask = (TaskItem, bool, Option<u32>);

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

/// How many of `events` events and `tasks` tasks fit in a Month view cell:
/// three lines, or two and "N more"; events first, but a task being
/// dragged there (`held`, the first task) always shows.
fn month_fit(events: usize, tasks: usize, held: bool) -> (usize, usize) {
    let room = if events + tasks > 3 { 2 } else { 3 };
    let shown = events.min(room - usize::from(held));
    (shown, tasks.min(room - shown))
}

/// `minutes` after midnight as a time of day.
pub(super) fn clock(minutes: u32) -> Time {
    Time::new(
        i8::try_from(minutes / 60).unwrap_or(0),
        i8::try_from(minutes % 60).unwrap_or(0),
        0,
        0,
    )
    .unwrap_or(Time::midnight())
}

impl MailWindow {
    /// The tasks with a due day the Calendar shows: none while the side
    /// list's Tasks is unticked.
    pub(super) fn calendar_tasks(&self) -> Vec<(&TaskItem, bool)> {
        if self.config.calendar.hide_tasks {
            return Vec::new();
        }
        self.dated_tasks()
    }

    /// Shows or hides tasks on the Calendar, remembered in the settings.
    fn toggle_calendar_tasks(&mut self, cx: &mut Context<Self>) {
        let view = &mut self.config.calendar;
        view.hide_tasks = !view.hide_tasks;
        self.calendar.task_drag = None;
        self.save_config();
        cx.notify();
    }

    /// The side list's Tasks row, as a calendar's: a box in the tasks'
    /// colour that shows or hides them, under the accounts' calendars.
    /// Only once there are task lists.
    pub(super) fn render_tasks_switch(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if self.tasks.columns().is_empty() {
            return None;
        }
        let shown = !self.config.calendar.hide_tasks;
        let row = div()
            .id("calendar-tasks-row")
            .h(px(space::S7))
            .pl(px(space::S3))
            .pr(px(space::S3))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(space::S4))
            .rounded(px(radius::FULL))
            .cursor_pointer()
            .relative()
            .child(crate::widgets::hover_fade("hover-glow", None, th))
            .focus_ring(th)
            .on_click(cx.listener(|this, _, _, cx| this.toggle_calendar_tasks(cx)))
            .child(crate::widgets::checkbox_tinted(
                "calendar-tasks-box",
                shown,
                th.accent,
                th,
            ))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .text_size(px(text::BODY))
                    .text_color(rgba(th.text))
                    .child(tr!("calendar-tasks")),
            );
        // Under a line: tasks are every account's, not the group's above.
        Some(
            div()
                .mt(px(space::S2))
                .pt(px(space::S2))
                .border_t_1()
                .border_color(rgba(th.divider))
                .child(row)
                .into_any_element(),
        )
    }

    /// Where `task` shows: its due day and time, or where it is being
    /// dragged to.
    pub(super) fn task_place(&self, task: &TaskItem) -> Option<(Date, Option<u32>)> {
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
            .calendar_tasks()
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
                // The press that opens it picked it up; it isn't dragged.
                this.cancel_calendar_drags();
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

    /// What a Month view cell holds of `events` events and the tasks on
    /// `day`: three lines, or two and "N more". Returns how many events
    /// show, the tasks that show, and how many are left out. A task being
    /// dragged here always shows, first.
    pub(super) fn month_lines(&self, day: Date, events: usize) -> (usize, Vec<DayTask>, usize) {
        let mut tasks = self.tasks_on(day);
        let dragged = self.dragged_task_to(day);
        let held = dragged.and_then(|id| tasks.iter().position(|t| t.0.id == id));
        if let Some(ix) = held {
            let task = tasks.remove(ix);
            tasks.insert(0, task);
        }
        let (shown, room) = month_fit(events, tasks.len(), held.is_some());
        let more = events + tasks.len() - shown - room;
        tasks.truncate(room);
        (shown, tasks, more)
    }

    /// The task being dragged onto `day`, if one is.
    fn dragged_task_to(&self, day: Date) -> Option<i64> {
        let drag = self.calendar.task_drag.as_ref()?;
        drag.to.filter(|(on, _)| *on == day).map(|_| drag.id)
    }

    /// The tasks on `day` in a Month view cell, from [`Self::month_lines`].
    /// Pressing one picks it up, to drag it to another day.
    pub(super) fn render_month_tasks(
        &self,
        tasks: Vec<DayTask>,
        day: Date,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let dragged = self.dragged_task_to(day);
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
                    true,
                    th,
                    cx,
                )
                .h(px(MONTH_LINE))
                .mx(px(space::S2))
                .when(dragged == Some(task.id), |d| {
                    d.opacity(0.85)
                        .shadow(crate::widgets::elevation(th, elevation::MENU))
                })
                .into_any_element()
            })
            .collect()
    }

    /// Month view's `day` following the pointer while a task is dragged
    /// over it.
    pub(super) fn month_drag_over(
        &self,
        day: Date,
        cx: &Context<Self>,
    ) -> impl Fn(&MouseMoveEvent, &mut Window, &mut gpui::App) + 'static {
        cx.listener(move |this, event: &MouseMoveEvent, _, cx| {
            this.drag_task_to_day(day, event, cx)
        })
    }

    /// Follows the pointer over Month view's `day` while a task is
    /// dragged: it would be due that day, at the time it has.
    fn drag_task_to_day(&mut self, day: Date, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        let Some(drag) = &self.calendar.task_drag else {
            return;
        };
        if event.pressed_button != Some(MouseButton::Left) {
            self.cancel_calendar_drags();
            cx.notify();
            return;
        }
        let id = drag.id;
        let Some((task, _)) = self.dated_tasks().into_iter().find(|(t, _)| t.id == id) else {
            return;
        };
        let home = due_day(task).map(|d| (d, task.due_time));
        let to = Some((day, task.due_time)).filter(|to| Some(*to) != home);
        let Some(drag) = &mut self.calendar.task_drag else {
            return;
        };
        if drag.to != to {
            drag.to = to;
            cx.notify();
        }
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
                    .relative()
                    .child(crate::widgets::hover_fade("hover-glow", Some(8.0), th))
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

#[cfg(test)]
mod tests {
    use super::month_fit;

    #[test]
    fn month_cells_fit_three_lines() {
        // Room for all three.
        assert_eq!(month_fit(1, 2, false), (1, 2));
        // Two lines and "N more", events first.
        assert_eq!(month_fit(3, 2, false), (2, 0));
        assert_eq!(month_fit(1, 4, false), (1, 1));
        // A task dragged to a full day still shows.
        assert_eq!(month_fit(3, 2, true), (1, 1));
        assert_eq!(month_fit(0, 1, true), (0, 1));
        assert_eq!(month_fit(2, 1, true), (2, 1));
    }
}
