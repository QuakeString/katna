// SPDX-License-Identifier: GPL-3.0-or-later

//! Make it a task: the checklist line the cursor is on becomes a task in
//! the default list, as Obsidian and Bear treat "- [ ]" lines. The task
//! keeps `note:<id>` where a task made from a mail keeps its Message-ID,
//! so its Note chip on the Tasks page opens the note again.

use gpui::{Context, Focusable, Window};
use katna_i18n::tr;

use super::{MailWindow, check_of};
use crate::daemon::Command;
use crate::tasks::TaskCommand;

/// What starts the link of a task made from a note.
const NOTE: &str = "note:";

/// The link a task made from note `id` keeps.
fn note_link(id: i64) -> String {
    format!("{NOTE}{id}")
}

/// The note a task was made from, if it was made from one.
pub(in crate::window) fn note_of_task(link: &str) -> Option<i64> {
    link.strip_prefix(NOTE)?.parse().ok()
}

/// The text of `line` if it is an unticked checklist item with any.
fn task_text(line: &str) -> Option<&str> {
    match check_of(line) {
        Some((false, rest)) if !rest.trim().is_empty() => Some(rest.trim()),
        _ => None,
    }
}

impl MailWindow {
    /// The line Make it a task would make a task of: the open note's
    /// unticked checklist line under the cursor, once the note is saved.
    pub(super) fn task_line(&self, cx: &gpui::App) -> Option<String> {
        let editor = self.notes.as_ref()?.editor.as_ref()?;
        if editor.id == 0 {
            return None;
        }
        let area = editor.body.read(cx);
        let para = area.doc().para(area.cursor().path)?;
        task_text(&para.text).map(str::to_owned)
    }

    /// Make it a task (a note's toolbar).
    pub(super) fn make_line_task(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(title) = self.task_line(cx) else {
            return;
        };
        let Some(editor) = self.notes.as_ref().and_then(|p| p.editor.as_ref()) else {
            return;
        };
        let link = note_link(editor.id);
        window.focus(&editor.body.focus_handle(cx), cx);
        self.send(
            Command::Task(Box::new(TaskCommand::Add {
                list: 0,
                parent: None,
                title,
                due: String::new(),
                mail: link.clone(),
            })),
            Some(tr!("tasks-toast-added", count = 1_u64)),
            Some(Command::Task(Box::new(TaskCommand::RemoveFromMail(vec![
                link,
            ])))),
            false,
            cx,
        );
    }

    /// Opens note `id` on the Notes page (a task's Note chip).
    pub(in crate::window) fn open_task_note(
        &mut self,
        id: i64,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let loaded = self
            .notes
            .as_ref()
            .and_then(|p| p.notes.as_ref())
            .and_then(|n| n.as_ref().ok())
            .and_then(|notes| notes.iter().find(|n| n.id == id).cloned());
        // The page may not have read the notes yet.
        let note = loaded.or_else(|| {
            crate::data::notes(&self.paths)
                .ok()
                .and_then(|notes| notes.into_iter().find(|n| n.id == id))
        });
        match note.filter(|n| n.trashed_at.is_none()) {
            Some(note) => {
                self.open_app(super::super::apps::App::Notes, cx);
                self.open_note(Some(&note), false, None, window, cx);
            }
            None => self.show_snackbar(tr!("tasks-note-gone"), None, cx),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unticked_item_is_a_task() {
        assert_eq!(task_text("☐ book train "), Some("book train"));
        assert_eq!(task_text("☑ pack"), None);
        assert_eq!(task_text("☐ "), None);
        assert_eq!(task_text("plain"), None);
    }

    #[test]
    fn a_task_link_gives_back_its_note() {
        assert_eq!(note_of_task(&note_link(42)), Some(42));
        assert_eq!(note_of_task("id@example.org"), None);
    }
}
