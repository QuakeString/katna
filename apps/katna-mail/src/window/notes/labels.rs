// SPDX-License-Identifier: GPL-3.0-or-later

//! Labels on notes, as in Keep: the label picker on an open note, the
//! labels in the side list that filter the board, and the Edit labels
//! dialog that renames and deletes them. A label lives only on its
//! notes, so one with no notes left is gone.

use std::rc::Rc;

use gpui::{
    AnyElement, Context, Entity, Focusable, FontWeight, SharedString, Subscription, Window, div,
    prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::tokens::{radius, space, text};
use katna_ui::{InputEvent, TextInput, px};

use super::{MailWindow, NotesView};
use crate::daemon::Command;
use crate::theme::{Theme, fade};
use crate::widgets::{Check, filled_button, icon, icon_button, tip};

/// The longest label, in characters (the daemon's limit too).
const MAX_LABEL: usize = 50;

/// A label picker: a box to find or make a label, over the labels to
/// tick. Notes use it on an open note and on the ticked cards, and Tasks
/// with the same labels.
pub(in crate::window) struct LabelPicker {
    pub input: Entity<TextInput>,
    _subscription: Subscription,
}

/// What a picker does with a label typed and entered.
type OnLabel = fn(&mut MailWindow, String, &mut Context<MailWindow>);
/// What a picker does when Escape closes it.
type OnClose = fn(&mut MailWindow, &mut Window, &mut Context<MailWindow>);

impl LabelPicker {
    /// A picker whose Enter calls `on_submit` with the label typed (made
    /// if new) and whose Escape calls `on_cancel`. It takes the focus.
    pub(in crate::window) fn new(
        accent: u32,
        window: &mut Window,
        cx: &mut Context<MailWindow>,
        on_submit: OnLabel,
        on_cancel: OnClose,
    ) -> Self {
        let accent = rgba(accent).into();
        let input = cx.new(|cx| {
            let mut input = TextInput::new(tr!("notes-label-name"), cx);
            input.set_accent(accent);
            input
        });
        let subscription = cx.subscribe_in(
            &input,
            window,
            move |this, input, event: &InputEvent, window, cx| match event {
                InputEvent::Changed => cx.notify(),
                InputEvent::Submit => {
                    let label = clean(input.read(cx).text());
                    if label.is_empty() {
                        return;
                    }
                    on_submit(this, label, cx);
                    input.update(cx, |input, cx| input.set_text("", cx));
                }
                InputEvent::Cancel => on_cancel(this, window, cx),
            },
        );
        window.focus(&input.focus_handle(cx), cx);
        Self {
            input,
            _subscription: subscription,
        }
    }

    /// The label typed, cleaned.
    pub(in crate::window) fn typed(&self, cx: &gpui::App) -> String {
        clean(self.input.read(cx).text())
    }
}

/// What ticking or unticking a label in [`render_label_choices`] does.
pub(in crate::window) type OnToggle = Rc<dyn Fn(&mut MailWindow, String, &mut Context<MailWindow>)>;

/// A [`LabelPicker`]'s box and the labels of `labels` matching what is
/// typed, each ticked as `state` says, with "Create" for a new one;
/// clicking one calls `on_toggle`.
pub(in crate::window) fn render_label_choices(
    id: &'static str,
    picker: &LabelPicker,
    mut labels: Vec<String>,
    state: &dyn Fn(&str) -> Check,
    on_toggle: OnToggle,
    th: &Theme,
    cx: &mut Context<MailWindow>,
) -> AnyElement {
    let typed = picker.typed(cx);
    let lower = typed.to_lowercase();
    labels.sort_by_key(|l| l.to_lowercase());
    labels.dedup();
    let shown: Vec<String> = labels
        .into_iter()
        .filter(|l| l.to_lowercase().contains(&lower))
        .collect();
    let create = (!typed.is_empty() && !shown.contains(&typed)).then(|| {
        let label = typed.clone();
        let toggle = on_toggle.clone();
        let input = picker.input.clone();
        div()
            .id((id, usize::MAX))
            .h(px(32.0))
            .px(px(space::S3))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(space::S3))
            .rounded(px(radius::SM))
            .text_size(px(text::BODY))
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
        let check = state(&label);
        let name = label.clone();
        let toggle = on_toggle.clone();
        div()
            .id((id, ix))
            .h(px(32.0))
            .px(px(space::S3))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(space::S3))
            .rounded(px(radius::SM))
            .text_size(px(text::BODY))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(cx.listener(move |this, _, _, cx| toggle(this, label.clone(), cx)))
            .child(crate::widgets::checkbox(
                (SharedString::from(format!("{id}-box")), ix),
                check,
                th,
            ))
            .child(div().flex_1().min_w_0().truncate().child(name))
    });
    div()
        .flex()
        .flex_col()
        .child(
            div()
                .mb(px(space::S2))
                .px(px(space::S3))
                .text_size(px(text::CAPTION))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_dim))
                .child(tr!("notes-label-note")),
        )
        .child(
            div()
                .h(px(32.0))
                .px(px(space::S3))
                .flex()
                .items_center()
                .text_size(px(text::BODY))
                .child(picker.input.clone()),
        )
        .child(
            div()
                .id((id, usize::MAX - 1))
                .max_h(px(200.0))
                .overflow_y_scroll()
                .flex()
                .flex_col()
                .children(rows)
                .children(create),
        )
        .into_any_element()
}

/// The Edit labels dialog: a box per label, to rename it.
pub(super) struct LabelsDialog {
    rows: Vec<(String, Entity<TextInput>)>,
    _subscriptions: Vec<Subscription>,
}

/// `text` as a label: trimmed, and no longer than labels may be.
fn clean(text: &str) -> String {
    text.trim().chars().take(MAX_LABEL).collect()
}

impl MailWindow {
    /// Every label on a note, in order of name.
    pub(in crate::window) fn note_labels(&self) -> Vec<String> {
        let mut labels: Vec<String> = self
            .notes
            .as_ref()
            .and_then(|p| p.notes.as_ref())
            .and_then(|n| n.as_ref().ok())
            .map(|notes| {
                notes
                    .iter()
                    .flat_map(|n| n.labels.iter().cloned())
                    .collect()
            })
            .unwrap_or_default();
        labels.sort_by_key(|l| l.to_lowercase());
        labels.dedup();
        labels
    }

    /// The notes with label `label`, and which of them also have `other`.
    fn notes_with(&self, label: &str, other: &str) -> (Vec<i64>, Vec<i64>) {
        let notes = self
            .notes
            .as_ref()
            .and_then(|p| p.notes.as_ref())
            .and_then(|n| n.as_ref().ok());
        let mut with = Vec::new();
        let mut both = Vec::new();
        for note in notes.into_iter().flat_map(|n| n.iter()) {
            if note.labels.iter().any(|l| l == label) {
                with.push(note.id);
                if note.labels.iter().any(|l| l == other) {
                    both.push(note.id);
                }
            }
        }
        (with, both)
    }

    /// Shows the board of notes with `label`.
    pub(super) fn show_label(&mut self, label: String, cx: &mut Context<Self>) {
        self.close_note(cx);
        if let Some(page) = &mut self.notes {
            page.view = NotesView::Notes;
            page.label = Some(label);
        }
        cx.notify();
    }

    /// Puts label `label` on the open note, or takes it off.
    fn toggle_note_label(&mut self, label: String, cx: &mut Context<Self>) {
        let Some(editor) = self.notes.as_mut().and_then(|p| p.editor.as_mut()) else {
            return;
        };
        if let Some(ix) = editor.labels.iter().position(|l| *l == label) {
            editor.labels.remove(ix);
        } else if !label.is_empty() {
            editor.labels.push(label);
        }
        if editor.id == 0 {
            self.note_typed(cx);
        } else {
            let item = editor.item(cx);
            self.change_note(item, cx);
        }
    }

    /// Opens the label picker on the open note, or closes it.
    pub(super) fn toggle_label_picker(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let accent = self.theme(window).accent;
        let Some(editor) = self.notes.as_mut().and_then(|p| p.editor.as_mut()) else {
            return;
        };
        editor.palette = false;
        editor.places = false;
        if editor.picker.take().is_some() {
            let focus = editor.body.focus_handle(cx);
            window.focus(&focus, cx);
            cx.notify();
            return;
        }
        let picker = LabelPicker::new(
            accent,
            window,
            cx,
            |this, label, cx| {
                let has = this
                    .notes
                    .as_ref()
                    .and_then(|p| p.editor.as_ref())
                    .is_some_and(|e| e.labels.contains(&label));
                if !has {
                    this.toggle_note_label(label, cx);
                }
            },
            |this, window, cx| this.toggle_label_picker(window, cx),
        );
        if let Some(editor) = self.notes.as_mut().and_then(|p| p.editor.as_mut()) {
            editor.picker = Some(picker);
        }
        cx.notify();
    }

    /// The label picker under the open note's text, when open.
    pub(super) fn render_label_picker(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let editor = self.notes.as_ref()?.editor.as_ref()?;
        let picker = editor.picker.as_ref()?;
        let mut labels = self.note_labels();
        // Labels just put on this note, before the board has them.
        labels.extend(editor.labels.iter().cloned());
        let on = editor.labels.clone();
        let list = render_label_choices(
            "note-label-pick",
            picker,
            labels,
            &move |label: &str| Check::from(on.iter().any(|l| l == label)),
            Rc::new(|this: &mut Self, label: String, cx: &mut Context<Self>| {
                this.toggle_note_label(label, cx)
            }),
            th,
            cx,
        );
        Some(
            div()
                .flex_none()
                .mx(px(space::S4))
                .mb(px(space::S3))
                .p(px(space::S3))
                .rounded(px(radius::SM))
                .bg(rgba(fade(th.text, 0.05)))
                .child(list)
                .into_any_element(),
        )
    }

    /// The open note's labels, as chips that take themselves off.
    pub(super) fn render_note_label_chips(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let editor = self.notes.as_ref()?.editor.as_ref()?;
        if editor.labels.is_empty() {
            return None;
        }
        Some(
            div()
                .flex_none()
                .px(px(16.0))
                .pb(px(8.0))
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(6.0))
                .children(editor.labels.iter().enumerate().map(|(ix, label)| {
                    let group: SharedString = format!("note-label-chip-{ix}").into();
                    let off = label.clone();
                    div()
                        .id(("note-label-chip", ix))
                        .group(group.clone())
                        .relative()
                        .h(px(24.0))
                        .px(px(10.0))
                        .flex()
                        .items_center()
                        .rounded_full()
                        .bg(rgba(fade(th.text, 0.08)))
                        .text_size(px(12.0))
                        .child(label.clone())
                        .child(
                            div()
                                .id(("note-label-off", ix))
                                .absolute()
                                .right(px(2.0))
                                .size(px(20.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded_full()
                                .bg(rgba(th.surface))
                                .opacity(0.0)
                                .group_hover(group, |s| s.opacity(1.0))
                                .cursor_pointer()
                                .tooltip(tip(tr!("notes-label-remove"), th))
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.toggle_note_label(off.clone(), cx)
                                }))
                                .child(icon("close", th.text_dim, 14.0)),
                        )
                }))
                .into_any_element(),
        )
    }

    /// The side list's labels and its Edit labels entry.
    pub(super) fn render_side_labels(&self, th: &Theme, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let current = self.notes.as_ref().and_then(|p| p.label.clone());
        let entry = |id: gpui::ElementId, name: &'static str, text: String, on: bool| {
            crate::window::nav::side_row(id, name, text, on, th)
        };
        let labels = self.note_labels();
        let mut out: Vec<AnyElement> = labels
            .into_iter()
            .enumerate()
            .map(|(ix, label)| {
                let on = current.as_deref() == Some(label.as_str());
                let show = label.clone();
                entry(("notes-label", ix).into(), "label", label, on)
                    .on_click(cx.listener(move |this, _, _, cx| this.show_label(show.clone(), cx)))
                    .into_any_element()
            })
            .collect();
        out.push(
            entry(
                "notes-edit-labels".into(),
                "compose",
                tr!("notes-edit-labels"),
                false,
            )
            .on_click(cx.listener(|this, _, window, cx| this.open_labels_dialog(window, cx)))
            .into_any_element(),
        );
        out
    }

    /// Opens the Edit labels dialog.
    fn open_labels_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.close_note(cx);
        let accent = rgba(self.theme(window).accent).into();
        let mut rows = Vec::new();
        let mut subscriptions = Vec::new();
        for label in self.note_labels() {
            let input = cx.new(|cx| {
                let mut input = TextInput::new(tr!("notes-label-name"), cx);
                input.set_accent(accent);
                input.set_text(label.clone(), cx);
                input
            });
            subscriptions.push(cx.subscribe_in(
                &input,
                window,
                |this, _, event: &InputEvent, _, cx| match event {
                    InputEvent::Submit => this.close_labels_dialog(true, cx),
                    InputEvent::Cancel => this.close_labels_dialog(false, cx),
                    InputEvent::Changed => {}
                },
            ));
            rows.push((label, input));
        }
        if let Some((_, first)) = rows.first() {
            window.focus(&first.focus_handle(cx), cx);
        }
        if let Some(page) = &mut self.notes {
            page.labels_dialog = Some(LabelsDialog {
                rows,
                _subscriptions: subscriptions,
            });
        }
        cx.notify();
    }

    /// Closes the Edit labels dialog, renaming the labels changed in it
    /// when `apply`.
    fn close_labels_dialog(&mut self, apply: bool, cx: &mut Context<Self>) {
        let Some(dialog) = self.notes.as_mut().and_then(|p| p.labels_dialog.take()) else {
            return;
        };
        if apply {
            for (old, input) in dialog.rows {
                let new = clean(input.read(cx).text());
                if !new.is_empty() && new != old {
                    self.rename_label(old, new, cx);
                }
            }
        }
        cx.notify();
    }

    /// Renames label `old` to `new` on every note, with Undo; a note that
    /// had both keeps one.
    fn rename_label(&mut self, old: String, new: String, cx: &mut Context<Self>) {
        let (ids, both) = self.notes_with(&old, &new);
        if ids.is_empty() {
            return;
        }
        let only: Vec<i64> = ids
            .iter()
            .copied()
            .filter(|id| !both.contains(id))
            .collect();
        let mut undo = Vec::new();
        if !only.is_empty() {
            undo.push(Command::RelabelNotes(only, new.clone(), old.clone()));
        }
        if !both.is_empty() {
            undo.push(Command::RelabelNotes(both, String::new(), old.clone()));
        }
        if let Some(page) = &mut self.notes
            && page.label.as_deref() == Some(old.as_str())
        {
            page.label = Some(new.clone());
        }
        let done = tr!("notes-label-renamed", name = new.clone());
        self.send(
            Command::RelabelNotes(ids, old, new),
            Some(done),
            Some(Command::Several(undo)),
            false,
            cx,
        );
    }

    /// Takes label `label` off every note, with Undo.
    fn delete_label(&mut self, label: String, cx: &mut Context<Self>) {
        let (ids, _) = self.notes_with(&label, "");
        if let Some(page) = &mut self.notes {
            if page.label.as_deref() == Some(label.as_str()) {
                page.label = None;
            }
            if let Some(dialog) = &mut page.labels_dialog {
                dialog.rows.retain(|(l, _)| *l != label);
            }
        }
        if ids.is_empty() {
            cx.notify();
            return;
        }
        let done = tr!("notes-label-deleted", name = label.clone());
        self.send(
            Command::RelabelNotes(ids.clone(), label.clone(), String::new()),
            Some(done),
            Some(Command::RelabelNotes(ids, String::new(), label)),
            false,
            cx,
        );
        cx.notify();
    }

    /// The Edit labels dialog, over the board.
    pub(super) fn render_labels_dialog(
        &self,
        th: &Theme,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let dialog = self.notes.as_ref()?.labels_dialog.as_ref()?;
        let rows = dialog.rows.iter().enumerate().map(|(ix, (label, input))| {
            let gone = label.clone();
            div()
                .h(px(40.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .child(icon("label", th.text_dim, 20.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .h(px(32.0))
                        .flex()
                        .items_center()
                        .border_b_1()
                        .border_color(rgba(th.divider))
                        .text_size(px(14.0))
                        .child(input.clone()),
                )
                .child(
                    icon_button(("notes-label-delete", ix), "trash", 18.0, th)
                        .tooltip(tip(tr!("notes-label-delete"), th))
                        .on_click(
                            cx.listener(move |this, _, _, cx| this.delete_label(gone.clone(), cx)),
                        ),
                )
        });
        let empty = dialog.rows.is_empty().then(|| {
            div()
                .py(px(12.0))
                .text_size(px(14.0))
                .text_color(rgba(th.text_dim))
                .child(tr!("notes-labels-none"))
        });
        let card = div()
            .id("notes-labels-dialog")
            .occlude()
            .w(px(360.0))
            .max_h(px(super::super::about::dialog_max_height(window)))
            .p(px(24.0))
            .flex()
            .flex_col()
            .map(|d| crate::widgets::dialog(d, th, th.menu))
            .text_color(rgba(th.text))
            .on_click(|_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .mb(px(12.0))
                    .text_size(px(20.0))
                    .child(tr!("notes-edit-labels")),
            )
            .child(
                div()
                    .id("notes-labels-rows")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .flex()
                    .flex_col()
                    .children(rows)
                    .children(empty),
            )
            .child(
                div().mt(px(20.0)).flex().flex_row().justify_end().child(
                    filled_button("notes-labels-done", tr!("notes-labels-done"), th)
                        .on_click(cx.listener(|this, _, _, cx| this.close_labels_dialog(true, cx))),
                ),
            );
        Some(
            div()
                .id("notes-labels-scrim")
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgba(if th.dark { 0x00000099 } else { 0x0000004d }))
                .on_click(cx.listener(|this, _, _, cx| this.close_labels_dialog(true, cx)))
                .child(card)
                .into_any_element(),
        )
    }
}
