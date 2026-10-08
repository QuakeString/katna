// SPDX-License-Identifier: GPL-3.0-or-later

//! Selecting several tasks, as the mail list does: Ctrl+click selects or
//! lets go of a task, Shift+click selects every task from the last one
//! clicked, and Esc lets go of them all. While some are selected, a bar
//! over the top of the list offers Complete, Move to list, Set date, Star
//! and Delete for all of them, each with one Undo. Selected tasks drag
//! together.

use gpui::{
    Animation, AnimationExt, AnyElement, Context, FontWeight, MouseButton, Pixels, Point, div,
    ease_out_quint, prelude::*, rgba,
};
use jiff::civil::Date;
use katna_i18n::{format, tr};
use katna_ui::px;
use katna_ui::tokens::{duration, radius, space, text};

use super::{Menu, NAV_WIDTH, SINGLE_WIDTH, TasksPage, View, list_title, today};
use crate::tasks::{TaskCommand, TaskEdit};
use crate::theme::Theme;
use crate::widgets::{elevation, icon_button, tip};
use crate::window::MailWindow;
use crate::window::compose::schedule;

/// One day on the Set date grid.
const DAY: f32 = 32.0;

impl TasksPage {
    /// The tasks a drag of task `id` takes: the selected ones, in the
    /// order shown, when `id` is one of several; else `id` alone. A drag
    /// between lists (`by_day` false) takes open tasks only, not steps.
    pub(super) fn dragged_with(&self, id: i64, by_day: bool) -> Vec<i64> {
        if self.selected.len() < 2 || !self.selected.contains(&id) {
            return vec![id];
        }
        let ids: Vec<i64> = self
            .shown()
            .into_iter()
            .filter(|t| self.selected.contains(t))
            .filter(|t| {
                by_day
                    || self
                        .task(*t)
                        .is_some_and(|t| t.parent.is_none() && !self.done(t))
            })
            .collect();
        if ids.contains(&id) { ids } else { vec![id] }
    }

    /// The selected tasks still here, in the lists' order.
    fn selected_tasks(&self) -> Vec<katna_store::tasks::Task> {
        self.columns()
            .iter()
            .flat_map(|c| c.tasks.iter())
            .filter(|t| self.selected.contains(&t.id))
            .cloned()
            .collect()
    }
}

impl MailWindow {
    /// Ctrl+click on task `id`: selects it, or lets it go. The task picked
    /// before is selected with it, so Ctrl+click on a second task selects
    /// both.
    pub(super) fn task_click_select(&mut self, id: i64, cx: &mut Context<Self>) {
        let page = &mut self.tasks;
        if page.selected.is_empty()
            && let Some(picked) = page.picked.filter(|p| *p != id)
        {
            page.selected.insert(picked);
        }
        if !page.selected.remove(&id) {
            page.selected.insert(id);
        }
        page.anchor = Some(id);
        page.picked = None;
        page.menu = None;
        cx.notify();
    }

    /// Shift+click on task `id`: selects every task shown from the last
    /// one clicked (or the picked one) to it.
    pub(super) fn task_select_range(&mut self, id: i64, cx: &mut Context<Self>) {
        let shown = self.tasks.shown();
        let page = &mut self.tasks;
        let from = page.anchor.or(page.picked).unwrap_or(id);
        let at = |t: i64| shown.iter().position(|s| *s == t);
        if let (Some(a), Some(b)) = (at(from), at(id)) {
            page.selected
                .extend(shown[a.min(b)..=a.max(b)].iter().copied());
        } else {
            page.selected.insert(id);
        }
        page.anchor = Some(id);
        page.picked = None;
        page.menu = None;
        cx.notify();
    }

    /// Lets go of every selected task.
    pub(super) fn tasks_clear_selection(&mut self, cx: &mut Context<Self>) {
        self.tasks.selected.clear();
        self.tasks.anchor = None;
        self.tasks.menu = None;
        cx.notify();
    }

    /// Ticks off the selected tasks, or unticks them when all are ticked.
    fn tasks_complete_selected(&mut self, cx: &mut Context<Self>) {
        let tasks = self.tasks.selected_tasks();
        let done = !tasks.iter().all(|t| self.tasks.done(t));
        let tasks: Vec<_> = tasks
            .into_iter()
            .filter(|t| self.tasks.done(t) != done)
            .collect();
        let mut commands = Vec::new();
        let mut undo = Vec::new();
        for task in &tasks {
            // A repeating task moves to its next day and stays open; Undo
            // puts its day back.
            if done && let Some((due, repeat)) = self.tasks.next_due(task) {
                undo.push(TaskCommand::Edit(task.id, TaskEdit::all_of(task)));
                if let Some(shown) = self.tasks.task_mut(task.id) {
                    shown.due = due;
                    shown.repeat = repeat;
                }
            } else {
                self.tasks.pending.entry(task.id).or_default().done = Some(done);
                undo.push(TaskCommand::SetDone(task.id, !done));
            }
            commands.push(TaskCommand::SetDone(task.id, done));
        }
        let count = commands.len() as u64;
        let text = if done {
            tr!("tasks-toast-done-several", count = count)
        } else {
            tr!("tasks-toast-open-several", count = count)
        };
        self.tasks_clear_selection(cx);
        self.send_tasks(commands, Some(text), undo, cx);
    }

    /// Stars the selected tasks, or takes their stars when all have one.
    fn tasks_star_selected(&mut self, cx: &mut Context<Self>) {
        let tasks = self.tasks.selected_tasks();
        let starred = !tasks.iter().all(|t| self.tasks.starred(t));
        let tasks: Vec<_> = tasks
            .into_iter()
            .filter(|t| self.tasks.starred(t) != starred)
            .collect();
        let edit = |id, starred| {
            TaskCommand::Edit(
                id,
                TaskEdit {
                    starred: Some(starred),
                    ..TaskEdit::default()
                },
            )
        };
        let mut commands = Vec::new();
        let mut undo = Vec::new();
        for task in &tasks {
            self.tasks.pending.entry(task.id).or_default().starred = Some(starred);
            commands.push(edit(task.id, starred));
            undo.push(edit(task.id, !starred));
        }
        let count = commands.len() as u64;
        let text = if starred {
            tr!("tasks-toast-starred", count = count)
        } else {
            tr!("tasks-toast-unstarred", count = count)
        };
        self.tasks.menu = None;
        self.send_tasks(commands, Some(text), undo, cx);
        cx.notify();
    }

    /// Moves the selected tasks (not steps: they go with their task) to
    /// list `list`.
    fn tasks_move_selected(&mut self, list: i64, cx: &mut Context<Self>) {
        let mut commands = Vec::new();
        let mut undo = Vec::new();
        for task in self.tasks.selected_tasks() {
            if task.parent.is_none() && task.list != list {
                commands.push(TaskCommand::Move(task.id, list));
                undo.push(TaskCommand::Move(task.id, task.list));
            }
        }
        let name = self
            .tasks
            .columns()
            .iter()
            .find(|c| c.list.id == list)
            .map(list_title)
            .unwrap_or_default();
        self.tasks_clear_selection(cx);
        self.send_tasks(
            commands,
            Some(tr!("tasks-toast-moved", list = name)),
            undo,
            cx,
        );
    }

    /// Gives tasks `ids` due day `day` (`None`: no date), keeping their
    /// times; a reminder moves with its task. Undo puts them back.
    pub(super) fn tasks_set_due(&mut self, ids: &[i64], day: Option<Date>, cx: &mut Context<Self>) {
        let due = day.map(|d| d.to_string()).unwrap_or_default();
        let mut commands = Vec::new();
        let mut undo = Vec::new();
        for &id in ids {
            let Some(task) = self.tasks.task(id).cloned() else {
                continue;
            };
            if task.due == due {
                continue;
            }
            let time = if due.is_empty() { None } else { task.due_time };
            let remind_at = task.remind_at.filter(|_| !due.is_empty()).map(|_| {
                katna_dav::todo::moved_reminder(
                    task.remind_at,
                    (&task.due, task.due_time),
                    (&due, time),
                    &jiff::tz::TimeZone::system(),
                )
            });
            undo.push(TaskCommand::Edit(
                id,
                TaskEdit {
                    due: Some(task.due.clone()),
                    due_time: Some(task.due_time),
                    remind_at: remind_at.map(|_| task.remind_at),
                    ..TaskEdit::default()
                },
            ));
            commands.push(TaskCommand::Edit(
                id,
                TaskEdit {
                    due: Some(due.clone()),
                    due_time: Some(time),
                    remind_at,
                    ..TaskEdit::default()
                },
            ));
            // Shown there at once; the store follows.
            if let Some(shown) = self.tasks.task_mut(id) {
                shown.due = due.clone();
                shown.due_time = time;
            }
        }
        let count = commands.len() as u64;
        self.tasks.selected.clear();
        self.tasks.anchor = None;
        self.tasks.menu = None;
        self.send_tasks(
            commands,
            Some(tr!("tasks-toast-rescheduled-several", count = count)),
            undo,
            cx,
        );
        cx.notify();
    }

    /// Deletes the selected tasks with their steps; Undo brings them back.
    pub(super) fn tasks_delete_selected(&mut self, cx: &mut Context<Self>) {
        let Some(Ok(board)) = &self.tasks.board else {
            return;
        };
        let selected = &self.tasks.selected;
        // A step goes with its task when both are selected.
        let tasks: Vec<_> = self
            .tasks
            .selected_tasks()
            .into_iter()
            .filter(|t| t.parent.is_none_or(|p| !selected.contains(&p)))
            .collect();
        let mut commands = Vec::new();
        let mut undo = Vec::new();
        for task in &tasks {
            commands.push(TaskCommand::Delete(task.id));
            undo.push(TaskCommand::Restore {
                task: task.clone(),
                steps: board.steps(task.id),
                files: self.task_files_for_undo(task.id),
            });
        }
        let ids: Vec<i64> = tasks.iter().map(|t| t.id).collect();
        // They fold away, then are gone from the page; the store follows.
        let gone: Vec<i64> = board
            .columns
            .iter()
            .flat_map(|c| c.tasks.iter())
            .filter(|t| ids.contains(&t.id) || t.parent.is_some_and(|p| ids.contains(&p)))
            .map(|t| t.id)
            .collect();
        self.task_leave_motion(&gone, Self::tasks_take_off, cx);
        if self.tasks.picked.is_some_and(|p| ids.contains(&p)) {
            self.tasks.picked = None;
        }
        let count = commands.len() as u64;
        self.tasks_clear_selection(cx);
        self.send_tasks(
            commands,
            Some(tr!("tasks-toast-deleted-several", count = count)),
            undo,
            cx,
        );
    }

    /// The bar over the top of the list while tasks are selected: how
    /// many, and what can be done to them all.
    pub(super) fn render_select_bar(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let page = &self.tasks;
        let count = page.selected.len();
        if count == 0 {
            return None;
        }
        let tasks = page.selected_tasks();
        let all_done = tasks.iter().all(|t| page.done(t));
        let all_starred = tasks.iter().all(|t| page.starred(t));
        // As wide as the card under it; across the board on All tasks.
        let shape = self.layout.shape;
        let side = self.page_side_width(NAV_WIDTH);
        let room = shape.width - shape.rail() - side - shape.card_margin() - 2.0 * space::S5;
        let single = page.view != View::All;
        let button = |id: &'static str, name: &'static str, label: String| {
            // Not over a menu it opened.
            icon_button(id, name, 20.0, th).when(page.menu.is_none(), |d| d.tooltip(tip(label, th)))
        };
        let pill = div()
            .id("tasks-select-bar")
            .occlude()
            .when(single, |d| {
                d.w(px(SINGLE_WIDTH.min(room) - 2.0 * space::S2))
            })
            .when(!single, |d| d.w_full())
            .h(px(space::S8))
            .pl(px(space::S2))
            .pr(px(space::S2))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(space::S1))
            .rounded_full()
            .bg(rgba(if th.dark { th.read_row } else { th.surface }))
            .border_1()
            .border_color(rgba(th.outline))
            .shadow(elevation(th, katna_ui::tokens::elevation::CARD))
            .child(
                button("tasks-select-clear", "close", tr!("tasks-select-clear"))
                    .on_click(cx.listener(|this, _, _, cx| this.tasks_clear_selection(cx))),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .pl(px(space::S2))
                    .truncate()
                    .text_size(px(text::BODY))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgba(th.text_dim))
                    .child(tr!("tasks-selected", count = count as u64)),
            )
            .child(
                button(
                    "tasks-select-done",
                    "check-circle",
                    if all_done {
                        tr!("tasks-mark-open")
                    } else {
                        tr!("tasks-mark-done")
                    },
                )
                .on_click(cx.listener(|this, _, _, cx| this.tasks_complete_selected(cx))),
            )
            .child(
                button("tasks-select-move", "move-to", tr!("tasks-select-move")).on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, event: &gpui::MouseDownEvent, _, cx| {
                        this.tasks.menu = Some(Menu::MoveSelected { at: event.position });
                        cx.stop_propagation();
                        cx.notify();
                    }),
                ),
            )
            .child(
                button("tasks-select-date", "event", tr!("tasks-select-date")).on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, event: &gpui::MouseDownEvent, _, cx| {
                        this.tasks.menu = Some(Menu::DateSelected {
                            at: event.position,
                            month: today(),
                        });
                        cx.stop_propagation();
                        cx.notify();
                    }),
                ),
            )
            .child(
                button(
                    "tasks-select-star",
                    if all_starred { "star-filled" } else { "star" },
                    if all_starred {
                        tr!("tasks-unstar")
                    } else {
                        tr!("tasks-star")
                    },
                )
                .on_click(cx.listener(|this, _, _, cx| this.tasks_star_selected(cx))),
            )
            .child(
                button("tasks-select-delete", "trash", tr!("tasks-delete"))
                    .on_click(cx.listener(|this, _, _, cx| this.tasks_delete_selected(cx))),
            )
            .with_animation(
                "tasks-select-bar-in",
                Animation::new(katna_ui::motion::time(duration::FAST))
                    .with_easing(ease_out_quint()),
                |el, t| el.opacity(t).mt(px(-space::S2 * (1.0 - t))),
            );
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .right_0()
                .pt(px(space::S5 + space::S2))
                .px(px(space::S5 + space::S2))
                .flex()
                .justify_center()
                .child(pill)
                .into_any_element(),
        )
    }

    /// The select bar's menus: the lists to move to, or the days to set.
    pub(super) fn render_select_menu(
        &self,
        menu: &Menu,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<(Point<Pixels>, f32, Vec<AnyElement>)> {
        let item = |id: gpui::ElementId, name: &'static str, label: String| {
            super::menu_row(id, Some(name), label, th)
        };
        match *menu {
            Menu::MoveSelected { at } => {
                let items = self
                    .tasks
                    .columns()
                    .iter()
                    .map(|c| {
                        let list = c.list.id;
                        item(
                            ("tasks-select-to", list as usize).into(),
                            "list-bulleted",
                            list_title(c),
                        )
                        .on_click(
                            cx.listener(move |this, _, _, cx| this.tasks_move_selected(list, cx)),
                        )
                        .into_any_element()
                    })
                    .collect();
                Some((at, super::MENU_WIDTH, items))
            }
            Menu::DateSelected { at, month } => {
                let now = today();
                let day = |n: i64| now.checked_add(jiff::Span::new().days(n)).ok();
                // Next Monday, or the week's first day where it isn't.
                let first = format::first_weekday();
                let next_week = (1..=7).filter_map(day).find(|d| d.weekday() == first);
                let mut items: Vec<AnyElement> = Vec::new();
                for (ix, (name, label, date)) in [
                    ("today", tr!("tasks-due-today"), Some(now)),
                    ("sunrise", tr!("tasks-due-tomorrow"), day(1)),
                    ("event", tr!("tasks-next-week"), next_week),
                    ("close", tr!("tasks-no-date"), None),
                ]
                .into_iter()
                .enumerate()
                {
                    items.push(
                        item(("tasks-select-day", ix).into(), name, label)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                let ids: Vec<i64> = this.tasks.selected.iter().copied().collect();
                                this.tasks_set_due(&ids, date, cx)
                            }))
                            .into_any_element(),
                    );
                }
                items.push(super::menu_separator(th).into_any_element());
                items.push(self.render_select_month(at, month, th, cx));
                Some((at, 7.0 * DAY + 2.0 * (space::S4 + space::S2), items))
            }
            _ => None,
        }
    }

    /// The month grid of Set date: a click on a day sets it.
    fn render_select_month(
        &self,
        at: Point<Pixels>,
        month: Date,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let now = today();
        let days = schedule::month_grid(month, format::first_weekday())
            .into_iter()
            .enumerate()
            .map(|(ix, date)| {
                let other = date.month() != month.month();
                div()
                    .id(("tasks-select-grid", ix))
                    .size(px(DAY))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .text_size(px(text::CAPTION))
                    .cursor_pointer()
                    .when(other, |d| d.text_color(rgba(th.text_faint)))
                    .when(date == now, |d| {
                        d.border_1()
                            .border_color(rgba(th.accent))
                            .text_color(rgba(th.accent))
                    })
                    .relative()
                    .child(katna_ui::Glow::new(
                        ("tasks-select-grid-glow", ix),
                        rgba(th.hover),
                    ))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        let ids: Vec<i64> = this.tasks.selected.iter().copied().collect();
                        this.tasks_set_due(&ids, Some(date), cx)
                    }))
                    .child(format::number(date.day() as u64))
            });
        let weekdays = format::weekdays_short().into_iter().map(|(_, d)| {
            div()
                .size(px(DAY))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(text::MICRO))
                .text_color(rgba(th.text_dim))
                .child(d)
        });
        let step = move |months: i32| {
            move |this: &mut Self,
                  _: &gpui::ClickEvent,
                  _: &mut gpui::Window,
                  cx: &mut Context<Self>| {
                if let Ok(m) = month
                    .first_of_month()
                    .checked_add(jiff::Span::new().months(months))
                {
                    this.tasks.menu = Some(Menu::DateSelected { at, month: m });
                }
                cx.notify();
            }
        };
        div()
            .px(px(space::S4))
            .flex()
            .flex_col()
            .child(
                div()
                    .h(px(space::S7))
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .pl(px(space::S2))
                            .text_size(px(text::BODY))
                            .font_weight(FontWeight::MEDIUM)
                            .child(format::month_year(month)),
                    )
                    .child(
                        icon_button("tasks-select-prev", "chevron-left", 20.0, th)
                            .size(px(space::S7))
                            .on_click(cx.listener(step(-1))),
                    )
                    .child(
                        icon_button("tasks-select-next", "chevron-right", 20.0, th)
                            .size(px(space::S7))
                            .on_click(cx.listener(step(1))),
                    ),
            )
            .child(
                div()
                    .mt(px(space::S2))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .rounded(px(radius::SM))
                    .children(weekdays)
                    .children(days),
            )
            .into_any_element()
    }
}
