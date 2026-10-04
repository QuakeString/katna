// SPDX-License-Identifier: GPL-3.0-or-later

//! Links between notes: typing `[[` suggests notes by title, and picking
//! one puts in its title as a link to it (`katna-note:` and its UUID, a
//! normal link in the note's HTML, so Apple Notes still shows the text).
//! A note lists the notes linking to it under "Linked from", and a click
//! on a link opens the note.

use std::rc::Rc;

use gpui::{
    AnyElement, Context, Focusable as _, Window, anchored, deferred, div, point, prelude::*, rgba,
};
use katna_dbus::NoteItem;
use katna_i18n::tr;
use katna_store::{NOTE_LINK_SCHEME, Note};
use katna_ui::px;
use katna_ui::rich::PickKey;
use katna_ui::tokens::{elevation, radius, space, text};

use super::MailWindow;
use crate::daemon;
use crate::theme::Theme;
use crate::widgets::{icon, raised};

/// The most notes suggested.
const SUGGESTED: usize = 5;
/// A `[[` further back than this many characters is not a link being
/// typed.
const MAX_QUERY: usize = 60;

/// A link being typed after `[[`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct LinkPick {
    /// What is typed after `[[`.
    query: String,
    /// The suggestion Enter takes.
    choice: usize,
}

/// What a suggestion is.
enum Choice {
    Note(Rc<Note>),
    New(String),
}

/// The text after the last `[[` before the cursor, when a link is being
/// typed there.
fn typed_link(before: &str) -> Option<&str> {
    let at = before.rfind("[[")?;
    let query = &before[at + 2..];
    (!query.contains("]]") && query.chars().count() <= MAX_QUERY).then_some(query)
}

/// A note's name: its title, else its first line.
pub(super) fn note_name(note: &Note) -> String {
    let title = note.title.trim();
    if !title.is_empty() {
        return title.to_owned();
    }
    note.body
        .lines()
        .map(|l| l.trim().trim_start_matches(['☐', '☑']).trim())
        .find(|l| !l.is_empty())
        .map_or_else(|| tr!("notes-untitled"), str::to_owned)
}

/// The notes whose name has `query`, names starting with it first.
fn suggested(notes: &[Note], query: &str, skip: i64) -> Vec<Rc<Note>> {
    let lower = query.trim().to_lowercase();
    let mut found: Vec<(bool, Rc<Note>)> = notes
        .iter()
        .filter(|n| n.id != skip && n.trashed_at.is_none() && !n.uuid.is_empty())
        .filter_map(|n| {
            let name = note_name(n).to_lowercase();
            name.contains(&lower)
                .then(|| (!name.starts_with(&lower), Rc::new(n.clone())))
        })
        .collect();
    found.sort_by_key(|(later, n)| (*later, std::cmp::Reverse(n.updated_at)));
    found.into_iter().take(SUGGESTED).map(|(_, n)| n).collect()
}

/// Whether `note`'s text links to the note with `uuid`.
pub(super) fn links_to(note: &Note, uuid: &str) -> bool {
    !uuid.is_empty() && note.html.contains(&format!("{NOTE_LINK_SCHEME}{uuid}"))
}

impl MailWindow {
    fn all_notes(&self) -> Rc<Vec<Note>> {
        self.notes
            .as_ref()
            .and_then(|p| p.notes.as_ref())
            .and_then(|n| n.as_ref().ok())
            .cloned()
            .unwrap_or_default()
    }

    /// The suggestions for the link being typed.
    fn link_choices(&self) -> Vec<Choice> {
        let Some(editor) = self.notes.as_ref().and_then(|p| p.editor.as_ref()) else {
            return Vec::new();
        };
        let Some(pick) = &editor.link_pick else {
            return Vec::new();
        };
        let mut choices: Vec<Choice> = suggested(&self.all_notes(), &pick.query, editor.id)
            .into_iter()
            .map(Choice::Note)
            .collect();
        if !pick.query.trim().is_empty() {
            choices.push(Choice::New(pick.query.trim().to_owned()));
        }
        choices
    }

    /// Shows or hides the suggestions as the text before the cursor has
    /// a `[[` link being typed.
    pub(super) fn update_link_pick(&mut self, cx: &mut Context<Self>) {
        let Some(editor) = self.notes.as_mut().and_then(|p| p.editor.as_mut()) else {
            return;
        };
        let area = editor.body.read(cx);
        let query = (!area.has_selection())
            .then(|| typed_link(area.text_before_cursor()).map(str::to_owned))
            .flatten();
        let pick = query.map(|query| LinkPick {
            choice: editor
                .link_pick
                .as_ref()
                .filter(|p| p.query == query)
                .map_or(0, |p| p.choice),
            query,
        });
        if pick != editor.link_pick {
            let on = pick.is_some();
            editor.link_pick = pick;
            editor.body.update(cx, |area, cx| area.set_picking(on, cx));
            cx.notify();
        }
    }

    /// Escape: hides the suggestions, if shown. Returns whether they were.
    pub(super) fn cancel_link_pick(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(editor) = self.notes.as_mut().and_then(|p| p.editor.as_mut()) else {
            return false;
        };
        if editor.link_pick.take().is_none() {
            return false;
        }
        // The `[[` stays as typed; closing the brackets ends it.
        editor.body.update(cx, |area, cx| {
            area.set_picking(false, cx);
            area.insert("]]", cx);
        });
        cx.notify();
        true
    }

    /// The arrows, Enter, Tab and Escape among the suggestions.
    pub(super) fn link_pick_key(
        &mut self,
        key: PickKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.link_choices().len();
        let Some(pick) = self
            .notes
            .as_mut()
            .and_then(|p| p.editor.as_mut())
            .and_then(|e| e.link_pick.as_mut())
        else {
            return;
        };
        match key {
            PickKey::Up => {
                pick.choice = pick
                    .choice
                    .checked_sub(1)
                    .unwrap_or(count.saturating_sub(1))
            }
            PickKey::Down => {
                pick.choice = if pick.choice + 1 >= count {
                    0
                } else {
                    pick.choice + 1
                }
            }
            PickKey::Choose => {
                let choice = pick.choice;
                self.choose_link(choice, window, cx);
                return;
            }
            PickKey::Cancel => {
                self.cancel_link_pick(cx);
                return;
            }
        }
        cx.notify();
    }

    /// Puts in suggestion `ix`: a link to that note, or to a new note
    /// named as typed.
    fn choose_link(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        let mut choices = self.link_choices();
        if ix >= choices.len() {
            return;
        }
        let Some(editor) = self.notes.as_mut().and_then(|p| p.editor.as_mut()) else {
            return;
        };
        let Some(pick) = editor.link_pick.take() else {
            return;
        };
        let typed = pick.query.len() + 2;
        let body = editor.body.clone();
        body.update(cx, |area, cx| area.set_picking(false, cx));
        window.focus(&body.focus_handle(cx), cx);
        match choices.swap_remove(ix) {
            Choice::Note(note) => {
                let url = format!("{NOTE_LINK_SCHEME}{}", note.uuid);
                body.update(cx, |area, cx| {
                    area.replace_before_cursor(typed, &note_name(&note), Some(url.into()), cx)
                });
            }
            Choice::New(title) => {
                // The new note's UUID comes from the store: the link goes
                // in once it is saved.
                let item = NoteItem {
                    title: title.clone(),
                    account: editor.account.unwrap_or(0),
                    labels: editor.labels.clone(),
                    ..NoteItem::default()
                };
                let connection = self.daemon.clone();
                let paths = self.paths.clone();
                cx.spawn(async move |this, cx| {
                    let saved = cx
                        .background_executor()
                        .spawn(async move {
                            let connection = match connection {
                                Some(connection) => connection,
                                None => daemon::connect().await?,
                            };
                            let id = daemon::save_note(&connection, &item).await?;
                            let notes = crate::data::notes(&paths)?;
                            notes
                                .into_iter()
                                .find(|n| n.id == id)
                                .map(|n| n.uuid)
                                .ok_or_else(|| String::from("The new note is gone."))
                        })
                        .await;
                    this.update(cx, |this, cx| match saved {
                        Ok(uuid) => {
                            let url = format!("{NOTE_LINK_SCHEME}{uuid}");
                            body.update(cx, |area, cx| {
                                area.replace_before_cursor(typed, &title, Some(url.into()), cx)
                            });
                            this.load_notes(cx);
                        }
                        Err(err) => this.show_snackbar(err, None, cx),
                    })
                    .ok();
                })
                .detach();
            }
        }
        cx.notify();
    }

    /// ⋮ > Link a note: types `[[` at the cursor, so the suggestions show.
    pub(super) fn start_note_link(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(editor) = self.notes.as_ref().and_then(|p| p.editor.as_ref()) else {
            return;
        };
        let body = editor.body.clone();
        window.focus(&body.focus_handle(cx), cx);
        body.update(cx, |area, cx| area.insert("[[", cx));
        self.update_link_pick(cx);
    }

    /// A click on a link to a note opens that note.
    pub(super) fn open_note_link(
        &mut self,
        url: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(uuid) = url.strip_prefix(NOTE_LINK_SCHEME) else {
            return;
        };
        let found = self.all_notes().iter().find(|n| n.uuid == uuid).cloned();
        match found {
            Some(note) if note.trashed_at.is_none() => {
                self.close_note_now(cx);
                self.open_note(Some(&note), false, None, window, cx);
            }
            _ => self.show_snackbar(tr!("notes-link-gone"), None, cx),
        }
    }

    /// The suggestions under the cursor while a `[[` link is typed.
    pub(super) fn render_link_pick(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let editor = self.notes.as_ref()?.editor.as_ref()?;
        let pick = editor.link_pick.as_ref()?;
        let caret = editor.body.read(cx).cursor_bounds()?;
        let choices = self.link_choices();
        let rows = choices.into_iter().enumerate().map(|(ix, choice)| {
            let on = ix == pick.choice;
            let row = div()
                .id(("note-link-choice", ix))
                .h(px(32.0))
                .px(px(space::S3))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(space::S3))
                .rounded(px(radius::SM))
                .when(on, |d| d.bg(rgba(th.hover)))
                .hover(|s| s.bg(rgba(th.hover)))
                .cursor_pointer()
                .on_mouse_down(gpui::MouseButton::Left, |_, window, _| {
                    window.prevent_default()
                })
                .on_click(cx.listener(move |this, _, window, cx| this.choose_link(ix, window, cx)));
            match choice {
                Choice::Note(note) => row
                    .child(icon("notes", th.text_dim, 16.0))
                    .child(div().flex_1().min_w_0().truncate().child(note_name(&note)))
                    .children(note.labels.first().map(|label| {
                        div()
                            .flex_none()
                            .text_size(px(text::MICRO))
                            .text_color(rgba(th.text_faint))
                            .child(label.clone())
                    })),
                Choice::New(title) => row.child(icon("add", th.text_dim, 16.0)).child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .child(tr!("notes-link-new", title = title)),
                ),
            }
        });
        let at = point(
            caret.origin.x,
            caret.origin.y + caret.size.height + px(space::S2),
        );
        Some(
            deferred(
                anchored()
                    .position(at)
                    .snap_to_window_with_margin(px(space::S3))
                    .child(
                        raised(
                            div()
                                .id("note-link-pick")
                                .occlude()
                                .w(px(240.0))
                                .p(px(space::S2 + space::S1))
                                .flex()
                                .flex_col()
                                .gap(px(1.0))
                                .text_size(px(text::BODY))
                                .text_color(rgba(th.text)),
                            th,
                            radius::MD,
                            elevation::POPOVER,
                        )
                        .child(
                            div()
                                .px(px(space::S3))
                                .pt(px(space::S2))
                                .pb(px(space::S1))
                                .text_size(px(text::MICRO))
                                .text_color(rgba(th.text_faint))
                                .child(tr!("notes-link-note")),
                        )
                        .children(rows),
                    ),
            )
            .with_priority(5)
            .into_any_element(),
        )
    }

    /// "Linked from": the notes linking to the open one, at its foot.
    pub(super) fn render_linked_from(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let editor = self.notes.as_ref()?.editor.as_ref()?;
        let uuid = if editor.uuid.is_empty() {
            self.all_notes()
                .iter()
                .find(|n| n.id == editor.id && editor.id != 0)?
                .uuid
                .clone()
        } else {
            editor.uuid.clone()
        };
        let from: Vec<Note> = self
            .all_notes()
            .iter()
            .filter(|n| n.id != editor.id && n.trashed_at.is_none() && links_to(n, &uuid))
            .cloned()
            .collect();
        if from.is_empty() {
            return None;
        }
        Some(
            div()
                .flex_none()
                .mx(px(space::S5))
                .py(px(space::S3))
                .flex()
                .flex_col()
                .gap(px(space::S2 + space::S1))
                .border_t_1()
                .border_color(rgba(th.divider))
                .child(
                    div()
                        .text_size(px(text::MICRO))
                        .text_color(rgba(th.text_faint))
                        .child(tr!("notes-linked-from").to_uppercase()),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap(px(space::S2 + space::S1))
                        .children(from.into_iter().enumerate().map(|(ix, note)| {
                            let open = note.clone();
                            div()
                                .id(("note-linked-from", ix))
                                .h(px(24.0))
                                .pl(px(space::S3))
                                .pr(px(space::S4))
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(space::S2))
                                .rounded_full()
                                .border_1()
                                .border_color(rgba(th.outline))
                                .text_size(px(text::CAPTION))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgba(th.hover)))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.close_note_now(cx);
                                    this.open_note(Some(&open), false, None, window, cx)
                                }))
                                .child(icon("link", th.text_dim, 14.0))
                                .child(note_name(&note))
                        })),
                )
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_link_is_typed_after_two_brackets() {
        assert_eq!(typed_link("post it in [[Lau"), Some("Lau"));
        assert_eq!(typed_link("post it in [["), Some(""));
        assert_eq!(typed_link("see [[Launch plan]] then"), None);
        assert_eq!(typed_link("no link"), None);
    }

    #[test]
    fn names_starting_with_the_words_come_first() {
        let note = |id: i64, title: &str| Note {
            id,
            uuid: format!("U{id}"),
            title: title.to_owned(),
            ..Note::default()
        };
        let notes = [
            note(1, "Party launch guests"),
            note(2, "Launch plan"),
            note(3, "Books"),
        ];
        let found: Vec<i64> = suggested(&notes, "lau", 0).iter().map(|n| n.id).collect();
        assert_eq!(found, [2, 1]);
        assert!(suggested(&notes, "lau", 2).iter().all(|n| n.id != 2));
    }
}
