// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna Notes' commands (`docs/ARCHITECTURE.md` §13.11). Notes are kept
//! in `pim.db`; Trash empties itself a week after a note went there.

use std::collections::BTreeSet;
use std::sync::Weak;
use std::time::Duration;

use async_channel::Receiver;
use futures_lite::FutureExt;
use katna_core::{AccountId, AccountKind};
use katna_dbus::NoteItem;
use katna_store::{Mode, Note, Store};

use super::{CommandError, Daemon, Notice, unix_now};

/// How often every account's Notes folder is looked at for notes written
/// elsewhere.
const NOTES_EVERY: Duration = Duration::from_secs(10 * 60);
/// The first look, after the accounts have started.
const NOTES_FIRST: Duration = Duration::from_secs(45);
/// Changes made here within this long go up together.
const NOTES_SETTLE: Duration = Duration::from_secs(3);

/// The longest note title taken, in characters.
const MAX_NOTE_TITLE: usize = 1_000;
/// The longest note text taken, in bytes (Keep's limit is about 20,000
/// characters; an IMAP note is one small message).
const MAX_NOTE_BODY: usize = 1_000_000;
/// The most labels on one note.
const MAX_NOTE_LABELS: usize = 50;
/// The longest label, in characters (Keep's limit).
const MAX_LABEL: usize = 50;

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
        let old_account = if saved.id == 0 {
            None
        } else {
            self.store().note(saved.id)?.and_then(|n| n.account_id)
        };
        let id = self.store().save_note(&saved)?;
        tracing::debug!(id, "note saved");
        self.notes_changed(account_id);
        if old_account != account_id {
            self.notes_changed(old_account);
        }
        Ok(id)
    }

    /// Moves notes to Trash or back. Returns how many changed.
    pub fn trash_notes(&self, ids: &[i64], trashed: bool) -> Result<u32, CommandError> {
        let accounts = self.note_accounts(ids)?;
        let mut store = self.store();
        let changed = store.trash_notes(ids, trashed)?;
        store.purge_note_trash(unix_now())?;
        drop(store);
        tracing::info!(changed, trashed, "notes moved");
        for account in accounts {
            self.notes_changed(Some(account));
        }
        Ok(u32::try_from(changed).unwrap_or(u32::MAX))
    }

    /// Deletes notes for good. Returns how many existed.
    pub fn delete_notes(&self, ids: &[i64]) -> Result<u32, CommandError> {
        let accounts = self.note_accounts(ids)?;
        let deleted = self.store().delete_notes(ids)?;
        tracing::info!(deleted, "notes deleted");
        for account in accounts {
            self.notes_changed(Some(account));
        }
        Ok(u32::try_from(deleted).unwrap_or(u32::MAX))
    }

    /// Renames, deletes or adds a label on notes `ids` (see
    /// `RelabelNotes`). Returns how many changed.
    pub fn relabel_notes(&self, ids: &[i64], old: &str, new: &str) -> Result<u32, CommandError> {
        let (old, new) = (old.trim(), new.trim());
        if new.chars().count() > MAX_LABEL {
            return Err(CommandError::InvalidArgs(format!(
                "a label is at most {MAX_LABEL} characters"
            )));
        }
        if old.is_empty() && new.is_empty() {
            return Ok(0);
        }
        let accounts = self.note_accounts(ids)?;
        let changed = self.store().relabel_notes(ids, old, new)?;
        tracing::info!(changed, "notes relabeled");
        for account in accounts {
            self.notes_changed(Some(account));
        }
        Ok(u32::try_from(changed).unwrap_or(u32::MAX))
    }

    /// The accounts of notes `ids`.
    fn note_accounts(&self, ids: &[i64]) -> Result<BTreeSet<i64>, CommandError> {
        let store = self.store();
        let mut accounts = BTreeSet::new();
        for &id in ids {
            if let Some(account) = store.note(id)?.and_then(|n| n.account_id) {
                accounts.insert(account);
            }
        }
        Ok(accounts)
    }

    /// Has the notes of `account` go to its Notes folder soon.
    fn notes_changed(&self, account: Option<i64>) {
        if let Some(account) = account {
            let _ = self.notes_wake.0.try_send(AccountId(account));
        }
    }

    /// Brings `account`'s notes and its Notes folder in step, on a
    /// connection of its own.
    async fn sync_account_notes(&self, account: AccountId) -> Result<(), CommandError> {
        let from = self.account(account)?.address;
        let connection = self.connect_on_demand(account).await?;
        let mut store = Store::open(&self.paths, Mode::ReadWrite)?;
        let done = katna_sync::notes::sync_notes(&connection, &mut store, account, &from)
            .await
            .map_err(|err| CommandError::Failed(format!("notes: {err}")))?;
        if done != katna_sync::notes::NotesSynced::default() {
            tracing::info!(%account, ?done, "notes synced");
        }
        if done.changed_here() {
            // Apps read notes again when the mail changes.
            let _ = self.notices.try_send(Notice::MailChanged(account));
        }
        Ok(())
    }
}

/// The notes sync: accounts whose notes changed here go up a moment
/// later; every IMAP account is looked at every [`NOTES_EVERY`].
pub(super) async fn run(daemon: Weak<Daemon>, wakes: Receiver<AccountId>) {
    let mut next_all = std::time::Instant::now() + NOTES_FIRST;
    loop {
        let wait = next_all.saturating_duration_since(std::time::Instant::now());
        let woken = async { wakes.recv().await.ok().map(Some) }
            .or(async {
                async_io::Timer::after(wait).await;
                Some(None)
            })
            .await;
        let Some(woken) = woken else {
            return;
        };
        let mut accounts = BTreeSet::new();
        match woken {
            Some(account) => {
                accounts.insert(account);
                async_io::Timer::after(NOTES_SETTLE).await;
                while let Ok(account) = wakes.try_recv() {
                    accounts.insert(account);
                }
            }
            None => {
                next_all = std::time::Instant::now() + NOTES_EVERY;
                let Some(daemon) = daemon.upgrade() else {
                    return;
                };
                let Ok(all) = daemon.store().accounts() else {
                    continue;
                };
                accounts.extend(
                    all.into_iter()
                        .filter(|a| a.kind == AccountKind::Imap)
                        .map(|a| a.id),
                );
            }
        }
        let Some(daemon) = daemon.upgrade() else {
            return;
        };
        if daemon.closing() {
            return;
        }
        for account in accounts {
            if let Err(err) = daemon.sync_account_notes(account).await {
                tracing::info!(%account, %err, "notes not synced");
            }
        }
        drop(daemon);
    }
}
