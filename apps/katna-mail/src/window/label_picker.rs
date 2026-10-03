// SPDX-License-Identifier: GPL-3.0-or-later

//! The label picker Notes and Tasks share, as in Keep: a box to find or
//! make a label over the labels to tick. Notes and tasks have one set of
//! labels, so "Home" on a note is "Home" on a task
//! ([`MailWindow::shared_labels`]).

use gpui::{AnyElement, Context, Entity, FontWeight, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_ui::{TextInput, px};

use super::MailWindow;
use crate::theme::{Theme, fade};
use crate::widgets::icon;

/// The longest label, in characters (the daemon's limit too).
pub(super) const MAX_LABEL: usize = 50;

/// `text` as a label: trimmed, and no longer than labels may be.
pub(super) fn clean(text: &str) -> String {
    text.trim().chars().take(MAX_LABEL).collect()
}

/// The element IDs of one picker, so two pages' pickers never share one.
#[derive(Clone, Copy)]
pub(super) struct PickerIds {
    pub create: &'static str,
    pub pick: &'static str,
    pub check: &'static str,
    pub list: &'static str,
}

/// What one picker shows.
pub(super) struct Pick<'a> {
    /// Over the box: what is labelled ("Label note").
    pub heading: String,
    pub input: &'a Entity<TextInput>,
    /// Every label offered.
    pub labels: Vec<String>,
    /// Those ticked.
    pub ticked: &'a [String],
    pub ids: PickerIds,
    /// Puts a label on, or takes it off.
    pub toggle: fn(&mut MailWindow, String, &mut Context<MailWindow>),
}

impl MailWindow {
    /// Every label on a note or a task, in order of name: the one set the
    /// picker offers on both pages.
    pub(super) fn shared_labels(&self) -> Vec<String> {
        let mut labels = self.note_labels();
        if let Some(board) = self.tasks.board() {
            for label in &board.labels {
                if !labels.contains(label) {
                    labels.push(label.clone());
                }
            }
        }
        labels.sort_by_key(|l| l.to_lowercase());
        labels.dedup();
        labels
    }
}

/// The picker: the labels whose names hold what is typed, each with a
/// box, and a row to make the typed one when it is new.
pub(super) fn label_picker(pick: Pick<'_>, th: &Theme, cx: &mut Context<MailWindow>) -> AnyElement {
    let Pick {
        heading,
        input,
        labels,
        ticked,
        ids,
        toggle,
    } = pick;
    let typed = clean(input.read(cx).text());
    let lower = typed.to_lowercase();
    let shown: Vec<String> = labels
        .into_iter()
        .filter(|l| l.to_lowercase().contains(&lower))
        .collect();
    let create = (!typed.is_empty() && !shown.contains(&typed)).then(|| {
        let label = typed.clone();
        let input = input.clone();
        div()
            .id(ids.create)
            .h(px(32.0))
            .px(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .rounded(px(6.0))
            .text_size(px(14.0))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(cx.listener(move |this, _, _, cx| {
                toggle(this, label.clone(), cx);
                input.update(cx, |input, cx| input.set_text("", cx));
            }))
            .child(icon("add", th.text_dim, 18.0))
            .child(tr!("notes-label-create", name = typed.clone()))
    });
    let rows = shown.into_iter().enumerate().map(|(ix, label)| {
        let on = ticked.contains(&label);
        let name = label.clone();
        div()
            .id((ids.pick, ix))
            .h(px(32.0))
            .px(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .rounded(px(6.0))
            .text_size(px(14.0))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(cx.listener(move |this, _, _, cx| toggle(this, label.clone(), cx)))
            .child(crate::widgets::checkbox(
                (ids.check, ix),
                crate::widgets::Check::from(on),
                th,
            ))
            .child(div().flex_1().min_w_0().truncate().child(name))
    });
    div()
        .flex_none()
        .mx(px(12.0))
        .mb(px(8.0))
        .p(px(8.0))
        .flex()
        .flex_col()
        .rounded(px(8.0))
        .bg(rgba(fade(th.text, 0.05)))
        .child(
            div()
                .mb(px(4.0))
                .px(px(8.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_dim))
                .child(heading),
        )
        .child(
            div()
                .h(px(32.0))
                .px(px(8.0))
                .flex()
                .items_center()
                .text_size(px(14.0))
                .child(input.clone()),
        )
        .child(
            div()
                .id(ids.list)
                .max_h(px(200.0))
                .overflow_y_scroll()
                .flex()
                .flex_col()
                .children(rows)
                .children(create),
        )
        .into_any_element()
}
