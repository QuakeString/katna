// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna Notes' commands (`docs/ARCHITECTURE.md` §13.11). Notes are kept
//! in `pim.db`; Trash empties itself a week after a note went there.

use katna_dbus::NoteItem;
use katna_store::Note;

use super::{CommandError, Daemon, unix_now};

/// The longest note title taken, in characters.
const MAX_NOTE_TITLE: usize = 1_000;
/// The longest note text taken, in bytes (Keep's limit is about 20,000
/// characters; an IMAP note is one small message).
const MAX_NOTE_BODY: usize = 1_000_000;
/// The most labels on one note.
const MAX_NOTE_LABELS: usize = 50;

impl Daemon {
    /// Saves a note (a new one for ID 0). Returns its ID.
    pub fn save_note(&self, note: NoteItem) -> Result<i64, CommandError> {
        if note.title.chars().count() > MAX_NOTE_TITLE {
            return Err(CommandError::InvalidArgs(format!(
                "a note's title is at most {MAX_NOTE_TITLE} characters"
            )));
        }
        if note.body.len() > MAX_NOTE_BODY {
            return Err(CommandError::InvalidArgs(format!(
                "a note is at most {} kB",
                MAX_NOTE_BODY / 1_000
            )));
        }
        if note.labels.len() > MAX_NOTE_LABELS {
            return Err(CommandError::InvalidArgs(format!(
                "a note has at most {MAX_NOTE_LABELS} labels"
            )));
        }
        let account_id = (note.account != 0).then_some(note.account);
        if let Some(account) = account_id
            && self
                .store()
                .account_settings(katna_core::AccountId(account))?
                .is_none()
        {
            return Err(CommandError::UnknownAccount(account));
        }
        let saved = Note {
            id: note.id,
            account_id,
            title: note.title.trim_end().to_owned(),
            body: note.body,
            color: note.color.clamp(0, 99),
            pinned: note.pinned,
            archived: note.archived,
            labels: note
                .labels
                .into_iter()
                .map(|l| l.trim().to_owned())
                .filter(|l| !l.is_empty())
                .collect(),
            link: Some(note.link).filter(|l| !l.is_empty()),
            ..Note::default()
        };
        let id = self.store().save_note(&saved)?;
        tracing::debug!(id, "note saved");
        Ok(id)
    }

    /// Moves notes to Trash or back. Returns how many changed.
    pub fn trash_notes(&self, ids: &[i64], trashed: bool) -> Result<u32, CommandError> {
        let mut store = self.store();
        let changed = store.trash_notes(ids, trashed)?;
        store.purge_note_trash(unix_now())?;
        tracing::info!(changed, trashed, "notes moved");
        Ok(u32::try_from(changed).unwrap_or(u32::MAX))
    }

    /// Deletes notes for good. Returns how many existed.
    pub fn delete_notes(&self, ids: &[i64]) -> Result<u32, CommandError> {
        let deleted = self.store().delete_notes(ids)?;
        tracing::info!(deleted, "notes deleted");
        Ok(u32::try_from(deleted).unwrap_or(u32::MAX))
    }
}
