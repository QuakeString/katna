// SPDX-License-Identifier: GPL-3.0-or-later

//! Labels on tasks, the same labels as on notes: the side list's Labels
//! that show every task with one, the quiet line under a task's title
//! ("📎 1 · Home · Bills"), and the label picker in the task's details,
//! which is Notes' own ([`LabelPicker`]).

use std::rc::Rc;

use gpui::{AnyElement, Context, SharedString, Window, div};
use gpui::{prelude::*, rgba};
use katna_i18n::tr;
use katna_store::tasks::Task as TaskItem;
use katna_ui::px;
use katna_ui::tokens::{radius, space, text};

use super::super::MailWindow;
use super::super::notes::labels::{LabelPicker, render_label_choices};
use super::{Column, View, card_heading, list_title, today};
use crate::theme::{Theme, fade};
use crate::widgets::{Check, icon, tip};

/// The label picker open in a task's details: Notes' own.
pub(super) type Picker = LabelPicker;

impl super::TasksPage {
    /// The labels on task `task`, as shown now.
    pub(super) fn labels_of<'a>(&'a self, task: &'a TaskItem) -> &'a [String] {
        &task.labels
    }
}

impl MailWindow {
    /// The quiet line under a task's title: a paper clip with how many
    /// files it has, then its labels, as in the study's mockup.
    pub(super) fn render_task_meta(&self, task: &TaskItem, th: &Theme) -> Option<AnyElement> {
        let page = &self.tasks;
        let files = page.board().map_or(0, |b| b.files_of(task.id).len());
        let labels = page.labels_of(task);
        if files == 0 && labels.is_empty() {
            return None;
        }
        let color = th.text_faint;
        let dot = || div().child("·");
        let mut line = div()
            .pt(px(space::S1))
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(space::S2))
            .text_size(px(text::CAPTION))
            .line_height(px(16.0))
            .text_color(rgba(color));
        if files > 0 {
            line = line.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(space::S1))
                    .child(icon("attachment", color, 14.0))
                    .child(katna_i18n::format::number(files as u64)),
            );
        }
        for (ix, label) in labels.iter().enumerate() {
            if files > 0 || ix > 0 {
                line = line.child(dot());
            }
            line = line.child(div().child(label.clone()));
        }
        Some(line.into_any_element())
    }

    /// The side list's Labels: each label on a task, showing every task
    /// with it.
    pub(super) fn render_task_labels_nav(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let page = &self.tasks;
        let labels = page.board().map(|b| b.task_labels()).unwrap_or_default();
        if labels.is_empty() {
            return Vec::new();
        }
        let mut out = vec![
            div()
                .mt(px(space::S4))
                .mb(px(space::S2))
                .px(px(space::S6))
                .text_size(px(text::MICRO))
                .font_weight(gpui::FontWeight::MEDIUM)
                .text_color(rgba(th.text_faint))
                .child(tr!("tasks-labels-heading").to_uppercase())
                .into_any_element(),
        ];
        for (ix, label) in labels.into_iter().enumerate() {
            let on = page.view == View::Label && page.label == label;
            let open = label.clone();
            out.push(
                super::super::nav::side_row(("tasks-label", ix), "label", label, on, th)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.tasks.label = open.clone();
                        this.task_set_view(View::Label, cx)
                    }))
                    .into_any_element(),
            );
        }
        out
    }

    /// Every open task with label `label`, from every list, with its list.
    pub(super) fn render_label_view(
        &self,
        label: &str,
        columns: Vec<&Column>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let page = &self.tasks;
        let day = today();
        let rows: Vec<AnyElement> = columns
            .iter()
            .flat_map(|c| c.tasks.iter().map(move |t| (*c, t)))
            .filter(|(_, t)| t.labels.iter().any(|l| l == label))
            .filter(|(_, t)| !page.done(t) && page.found(t))
            .map(|(c, t)| self.render_task_row(t, Some(&list_title(c)), day, th, cx))
            .collect();
        let empty = rows.is_empty();
        div()
            .size_full()
            .p(px(space::S5))
            .flex()
            .items_start()
            .justify_center()
            .child(
                self.card_frame(
                    "tasks-label-card".into(),
                    super::SINGLE_WIDTH,
                    th,
                    div()
                        .child(card_heading(label.to_owned(), th))
                        .when(empty, |d| {
                            d.child(
                                div()
                                    .py(px(space::S7))
                                    .px(px(space::S6))
                                    .text_center()
                                    .text_size(px(text::BODY))
                                    .text_color(rgba(th.text_faint))
                                    .child(tr!("tasks-label-empty")),
                            )
                        })
                        .children(rows),
                ),
            )
            .into_any_element()
    }

    // --- In the details ------------------------------------------------------

    /// Puts label `label` on the task in the details, or takes it off.
    fn task_details_toggle_label(&mut self, label: String, cx: &mut Context<Self>) {
        let Some(details) = &mut self.tasks.details else {
            return;
        };
        let labels = details.labels_mut();
        if let Some(ix) = labels.iter().position(|l| *l == label) {
            labels.remove(ix);
        } else if !label.is_empty() {
            labels.push(label);
        }
        cx.notify();
    }

    /// Opens the label picker in the details, or closes it.
    pub(super) fn task_details_label_picker(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let accent = self.theme(window).accent;
        let Some(details) = &mut self.tasks.details else {
            return;
        };
        if details.picker.take().is_some() {
            cx.notify();
            return;
        }
        let picker = LabelPicker::new(
            accent,
            window,
            cx,
            // Enter ticks the label typed, making it if it is new.
            |this, label, cx| {
                let has = this
                    .tasks
                    .details
                    .as_ref()
                    .is_some_and(|d| d.labels().contains(&label));
                if !has {
                    this.task_details_toggle_label(label, cx);
                }
            },
            |this, window, cx| this.task_details_label_picker(window, cx),
        );
        if let Some(details) = &mut self.tasks.details {
            details.picker = Some(picker);
        }
        cx.notify();
    }

    /// The details' labels: a chip for each, which takes itself off, and
    /// the button that opens the picker, with the picker under them.
    pub(super) fn render_details_labels(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(details) = self.tasks.details.as_ref() else {
            return div().into_any_element();
        };
        let ticked = details.labels();
        let chips: Vec<_> = ticked
            .iter()
            .enumerate()
            .map(|(ix, label)| {
                let group: SharedString = format!("task-label-chip-{ix}").into();
                let off = label.clone();
                div()
                    .id(("task-label-chip", ix))
                    .group(group.clone())
                    .relative()
                    .h(px(24.0))
                    .px(px(space::S3))
                    .flex()
                    .items_center()
                    .rounded_full()
                    .bg(rgba(fade(th.text, 0.08)))
                    .text_size(px(text::CAPTION))
                    .child(label.clone())
                    .child(
                        div()
                            .id(("task-label-off", ix))
                            .absolute()
                            .right(px(space::S1))
                            .size(px(20.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_full()
                            .bg(rgba(th.raised))
                            .opacity(0.0)
                            .group_hover(group, |s| s.opacity(1.0))
                            .cursor_pointer()
                            .tooltip(tip(tr!("notes-label-remove"), th))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.task_details_toggle_label(off.clone(), cx)
                            }))
                            .child(icon("close", th.text_dim, 14.0)),
                    )
            })
            .collect();
        let open = details.picker.is_some();
        let add = div()
            .id("task-label-add")
            .h(px(24.0))
            .px(px(space::S3))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(space::S2))
            .rounded_full()
            .border_1()
            .border_color(rgba(if open { th.accent } else { th.divider }))
            .text_size(px(text::CAPTION))
            .text_color(rgba(if open { th.accent } else { th.text_dim }))
            .cursor_pointer()
            .relative()
            .child(crate::widgets::hover_fade("hover-glow", None, th))
            .on_click(cx.listener(|this, _, window, cx| this.task_details_label_picker(window, cx)))
            .child(icon(
                "add",
                if open { th.accent } else { th.text_dim },
                14.0,
            ))
            .child(tr!("tasks-label-add"));
        let picker = details.picker.as_ref().map(|picker| {
            // Notes' and tasks' labels, and those just put on this task,
            // before the store has them.
            let mut labels = self.note_labels();
            labels.extend(
                self.tasks
                    .board()
                    .into_iter()
                    .flat_map(|b| b.labels.clone()),
            );
            labels.extend(ticked.iter().cloned());
            let on = ticked.to_vec();
            let list = render_label_choices(
                "task-label-pick",
                tr!("tasks-label-task"),
                picker,
                labels,
                &move |label: &str| Check::from(on.iter().any(|l| l == label)),
                Rc::new(|this: &mut Self, label: String, cx: &mut Context<Self>| {
                    this.task_details_toggle_label(label, cx)
                }),
                th,
                cx,
            );
            div()
                .mt(px(space::S3))
                .p(px(space::S3))
                .rounded(px(radius::SM))
                .bg(rgba(fade(th.text, 0.05)))
                .child(list)
        });
        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .min_h(px(32.0))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap(px(space::S3))
                    .child(icon("label", th.text_dim, 18.0))
                    .child(div().w(px(space::S1)))
                    .children(chips)
                    .child(add),
            )
            .children(picker)
            .into_any_element()
    }
}
