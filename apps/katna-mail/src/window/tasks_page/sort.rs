// SPDX-License-Identifier: GPL-3.0-or-later

//! Sorting a list, as Google Tasks does from its ⋮ menu: My order, Date,
//! Starred recently and Title. The choice is kept per list on this
//! computer (`Config::tasks`); the service keeps My order. In a sorted list
//! a task can't be dragged to another place: dropped there it goes back,
//! and a task dropped from another list goes where the sort puts it.

use std::cmp::Ordering;

use gpui::{AnyElement, Context, FontWeight, SharedString, div, prelude::*, rgba};
use katna_core::config::TaskSort;
use katna_i18n::tr;
use katna_store::tasks::Task as TaskItem;
use katna_ui::px;
use katna_ui::tokens::{space, text};

use super::{Column, TasksPage};
use crate::theme::Theme;
use crate::widgets::icon;
use crate::window::MailWindow;

/// The choices, in the menu's order.
const SORTS: [TaskSort; 4] = [
    TaskSort::MyOrder,
    TaskSort::Date,
    TaskSort::Starred,
    TaskSort::Title,
];

fn sort_label(sort: TaskSort) -> String {
    match sort {
        TaskSort::MyOrder => tr!("tasks-sort-my-order"),
        TaskSort::Date => tr!("tasks-sort-date"),
        TaskSort::Starred => tr!("tasks-sort-starred"),
        TaskSort::Title => tr!("tasks-sort-title"),
    }
}

impl TasksPage {
    /// How list `list` is sorted.
    pub(super) fn sort_of(&self, list: i64) -> TaskSort {
        self.sorts.get(&list).copied().unwrap_or_default()
    }

    /// `column`'s tasks in its sort, each followed by its steps (which
    /// keep their own order).
    pub(super) fn ordered<'a>(&self, column: &'a Column) -> Vec<&'a TaskItem> {
        let sort = self.sort_of(column.list.id);
        if sort == TaskSort::MyOrder {
            return column.tasks.iter().collect();
        }
        // Each task with the steps after it.
        let mut groups: Vec<Vec<&TaskItem>> = Vec::new();
        for task in &column.tasks {
            match groups.last_mut() {
                Some(group) if task.parent.is_some() && task.parent == Some(group[0].id) => {
                    group.push(task);
                }
                _ => groups.push(vec![task]),
            }
        }
        // A stable sort: ties keep My order.
        groups.sort_by(|a, b| compare(sort, a[0], b[0], self));
        groups.into_iter().flatten().collect()
    }
}

/// How `a` and `b` compare in `sort`.
fn compare(sort: TaskSort, a: &TaskItem, b: &TaskItem, page: &TasksPage) -> Ordering {
    match sort {
        TaskSort::MyOrder => Ordering::Equal,
        // Undated last; on a day, by time, those without one last.
        TaskSort::Date => (a.due.is_empty(), &a.due, a.due_time.is_none(), a.due_time).cmp(&(
            b.due.is_empty(),
            &b.due,
            b.due_time.is_none(),
            b.due_time,
        )),
        TaskSort::Starred => page.starred(b).cmp(&page.starred(a)),
        TaskSort::Title => a.title.to_lowercase().cmp(&b.title.to_lowercase()),
    }
}

impl MailWindow {
    /// Sorts list `list` by `sort`, remembered on this computer.
    pub(super) fn task_set_sort(&mut self, list: i64, sort: TaskSort, cx: &mut Context<Self>) {
        self.tasks.menu = None;
        if sort == TaskSort::MyOrder {
            self.tasks.sorts.remove(&list);
            self.config.tasks.sort.remove(&list.to_string());
        } else {
            self.tasks.sorts.insert(list, sort);
            self.config.tasks.sort.insert(list.to_string(), sort);
        }
        self.save_config();
        cx.notify();
    }

    /// Reads how each list is sorted from the settings.
    pub(super) fn read_task_sorts(&mut self) {
        self.tasks.sorts = self
            .config
            .tasks
            .sort
            .iter()
            .filter_map(|(list, sort)| Some((list.parse().ok()?, *sort)))
            .collect();
    }

    /// The Sort by part of list `list`'s ⋮ menu: its heading and the four
    /// choices, with a tick on the one in use.
    pub(super) fn render_sort_items(
        &self,
        list: i64,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let now = self.tasks.sort_of(list);
        let heading = div()
            .px(px(space::S5))
            .pt(px(space::S2))
            .pb(px(space::S2))
            .text_size(px(text::CAPTION))
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgba(th.text_dim))
            .child(tr!("tasks-sort-by"))
            .into_any_element();
        let items = SORTS.into_iter().map(|sort| {
            let on = sort == now;
            let id: SharedString = format!("tasks-sort-{sort:?}").into();
            div()
                .id(id)
                .h(px(36.0))
                .mx(px(space::S2))
                .pl(px(space::S3))
                .pr(px(space::S6))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(space::S3))
                .rounded(px(katna_ui::tokens::radius::SM))
                .cursor_pointer()
                .when(on, |d| d.bg(rgba(th.hover)))
                .hover(|s| s.bg(rgba(th.hover)))
                .child(
                    div()
                        .size(px(20.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .when(on, |d| d.child(icon("check", th.text, 16.0))),
                )
                .child(div().flex_1().min_w_0().truncate().child(sort_label(sort)))
                .on_click(cx.listener(move |this, _, _, cx| this.task_set_sort(list, sort, cx)))
                .into_any_element()
        });
        std::iter::once(heading).chain(items).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katna_store::tasks::TaskList;

    fn task(id: i64, title: &str, due: &str, starred: bool, parent: Option<i64>) -> TaskItem {
        TaskItem {
            id,
            list: 1,
            parent,
            title: title.into(),
            due: due.into(),
            starred,
            ..TaskItem::default()
        }
    }

    fn column() -> Column {
        Column {
            list: TaskList {
                id: 1,
                account: None,
                title: "Groceries".into(),
                is_default: false,
            },
            account: String::new(),
            to_do: false,
            tasks: vec![
                task(1, "curd", "", false, None),
                task(2, "Basmati rice", "2026-10-09", true, None),
                task(3, "brown", "", false, Some(2)),
                task(4, "Coriander", "2026-10-04", false, None),
            ],
        }
    }

    fn order(sort: TaskSort) -> Vec<i64> {
        let mut page = TasksPage::default();
        page.sorts.insert(1, sort);
        let column = column();
        page.ordered(&column).iter().map(|t| t.id).collect()
    }

    #[test]
    fn sorts_keep_steps_under_their_task() {
        assert_eq!(order(TaskSort::MyOrder), [1, 2, 3, 4]);
        assert_eq!(order(TaskSort::Date), [4, 2, 3, 1]);
        assert_eq!(order(TaskSort::Starred), [2, 3, 1, 4]);
        assert_eq!(order(TaskSort::Title), [2, 3, 4, 1]);
    }
}
