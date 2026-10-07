// SPDX-License-Identifier: GPL-3.0-or-later

//! The `meta` table of `pim.db`: a JSON value per object and feature, with
//! an optional expiry (`docs/ARCHITECTURE.md` §10). `katna-meta` gives the
//! values their types and runs the scheduler in the daemon; this module
//! only stores them.

use katna_core::{AccountId, MailCategory};
use rusqlite::{OptionalExtension, params};

use crate::Store;
use crate::error::Result;
use crate::mail::MessageId;

/// One row of the `meta` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaRow {
    /// What the value is attached to: `message`, `outbox`, …
    pub object_kind: String,
    pub object_id: i64,
    /// The feature that owns the value: `snooze`, `follow-up`, …
    pub plugin: String,
    pub value_json: String,
    /// Counts the changes of the value.
    pub version: i64,
    /// When the scheduler should act on it (Unix seconds), if ever.
    pub expires_at: Option<i64>,
}

const COLUMNS: &str = "object_kind, object_id, plugin, value_json, version, expires_at";

fn row(row: &rusqlite::Row<'_>) -> rusqlite::Result<MetaRow> {
    Ok(MetaRow {
        object_kind: row.get(0)?,
        object_id: row.get(1)?,
        plugin: row.get(2)?,
        value_json: row.get(3)?,
        version: row.get(4)?,
        expires_at: row.get(5)?,
    })
}

impl Store {
    /// Sets the value of `plugin` on an object, replacing any earlier one.
    pub fn set_meta(
        &mut self,
        object_kind: &str,
        object_id: i64,
        plugin: &str,
        value_json: &str,
        expires_at: Option<i64>,
    ) -> Result<()> {
        self.check_writable()?;
        self.pim
            .prepare_cached(
                "INSERT INTO meta (object_kind, object_id, plugin, value_json, expires_at, dirty)
                 VALUES (?1, ?2, ?3, ?4, ?5, 1)
                 ON CONFLICT (object_kind, object_id, plugin) DO UPDATE SET
                     value_json = excluded.value_json,
                     expires_at = excluded.expires_at,
                     version = version + 1,
                     dirty = 1",
            )?
            .execute(params![
                object_kind,
                object_id,
                plugin,
                value_json,
                expires_at
            ])?;
        Ok(())
    }

    /// Removes the value of `plugin` from an object. Returns whether there
    /// was one.
    pub fn remove_meta(&mut self, object_kind: &str, object_id: i64, plugin: &str) -> Result<bool> {
        self.check_writable()?;
        Ok(self
            .pim
            .prepare_cached(
                "DELETE FROM meta WHERE object_kind = ?1 AND object_id = ?2 AND plugin = ?3",
            )?
            .execute(params![object_kind, object_id, plugin])?
            > 0)
    }

    /// The value of `plugin` on an object, if it has one.
    pub fn meta(&self, object_kind: &str, object_id: i64, plugin: &str) -> Result<Option<MetaRow>> {
        Ok(self
            .pim
            .prepare_cached(&format!(
                "SELECT {COLUMNS} FROM meta
                 WHERE object_kind = ?1 AND object_id = ?2 AND plugin = ?3"
            ))?
            .query_row(params![object_kind, object_id, plugin], row)
            .optional()?)
    }

    /// Every value of `plugin`, soonest expiry first (none last).
    pub fn meta_of(&self, plugin: &str) -> Result<Vec<MetaRow>> {
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {COLUMNS} FROM meta WHERE plugin = ?1
             ORDER BY expires_at IS NULL, expires_at, object_id"
        ))?;
        let rows = stmt.query_map([plugin], row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Values whose expiry is at or before `now`, soonest first.
    pub fn due_meta(&self, now: i64) -> Result<Vec<MetaRow>> {
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {COLUMNS} FROM meta WHERE expires_at <= ?1
             ORDER BY expires_at, object_id"
        ))?;
        let rows = stmt.query_map([now], row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The soonest expiry of any value, if one has one.
    pub fn next_meta_expiry(&self) -> Result<Option<i64>> {
        Ok(self
            .pim
            .prepare_cached("SELECT min(expires_at) FROM meta")?
            .query_row([], |row| row.get(0))?)
    }
}

/// What the reminders built on `meta` read from `mail.db`.
impl Store {
    /// The `Message-ID` of a stored message, if it has one.
    pub fn message_id_header(&self, message: MessageId) -> Result<Option<String>> {
        Ok(self
            .mail
            .prepare_cached("SELECT message_id_hdr FROM message WHERE id = ?1")?
            .query_row([message.0], |row| row.get(0))
            .optional()?
            .flatten())
    }

    /// The messages of `account` with `Message-ID` `header` that are in a
    /// folder (not the outgoing copy), oldest first.
    pub fn messages_with_header(&self, account: AccountId, header: &str) -> Result<Vec<MessageId>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT m.id FROM message m
             WHERE m.account_id = ?1 AND m.message_id_hdr = ?2
               AND EXISTS (SELECT 1 FROM message_location l WHERE l.message_id = m.id)
             ORDER BY m.id",
        )?;
        let rows = stmt.query_map(params![account.0, header], |row| Ok(MessageId(row.get(0)?)))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The newest message of any account with `Message-ID` `header` that
    /// is in a folder: where a task made from a mail leads back to.
    pub fn message_with_header(&self, header: &str) -> Result<Option<MessageId>> {
        Ok(self
            .mail
            .prepare_cached(
                "SELECT m.id FROM message m
                 WHERE m.message_id_hdr = ?1
                   AND EXISTS (SELECT 1 FROM message_location l WHERE l.message_id = m.id)
                 ORDER BY m.id DESC LIMIT 1",
            )?
            .query_row([header], |row| Ok(MessageId(row.get(0)?)))
            .optional()?)
    }

    /// The messages of the conversation of `message` written after it
    /// (another `Message-ID`, a later date) that are in a folder, oldest
    /// first: replies, or follow-ups.
    pub fn later_in_thread(&self, message: MessageId) -> Result<Vec<LaterMessage>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT o.message_id_hdr, COALESCE(o.subject, ''), o.category
             FROM message m JOIN message o ON o.thread_id = m.thread_id
             WHERE m.id = ?1 AND o.id <> m.id
               AND o.message_id_hdr IS NOT m.message_id_hdr
               AND o.date > m.date
               AND EXISTS (SELECT 1 FROM message_location l WHERE l.message_id = o.id)
             ORDER BY o.date, o.id",
        )?;
        let rows = stmt.query_map([message.0], |row| {
            Ok(LaterMessage {
                message_id: row.get(0)?,
                subject: row.get(1)?,
                category: row
                    .get::<_, Option<i64>>(2)?
                    .and_then(MailCategory::from_storage),
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The newest message row: messages stored later have larger IDs.
    pub fn newest_message(&self) -> Result<MessageId> {
        Ok(MessageId(self.mail.query_row(
            "SELECT COALESCE(MAX(id), 0) FROM message",
            [],
            |row| row.get(0),
        )?))
    }

    /// The messages of the conversation of `message` stored after row
    /// `after` ([`Store::newest_message`] then) that are in an Inbox,
    /// oldest first: what brings a snoozed conversation back early.
    pub fn arrived_in_inbox_after(
        &self,
        message: MessageId,
        after: MessageId,
    ) -> Result<Vec<LaterMessage>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT o.message_id_hdr, COALESCE(o.subject, ''), o.category
             FROM message m JOIN message o ON o.thread_id = m.thread_id
             WHERE m.id = ?1 AND o.id > ?2
               AND o.message_id_hdr IS NOT m.message_id_hdr
               AND EXISTS (SELECT 1 FROM message_location l
                           JOIN folder f ON f.id = l.folder_id
                           WHERE l.message_id = o.id AND f.role = 'inbox')
             ORDER BY o.id",
        )?;
        let rows = stmt.query_map([message.0, after.0], |row| {
            Ok(LaterMessage {
                message_id: row.get(0)?,
                subject: row.get(1)?,
                category: row
                    .get::<_, Option<i64>>(2)?
                    .and_then(MailCategory::from_storage),
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }
}

/// A message later in a conversation ([`Store::later_in_thread`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaterMessage {
    /// Its `Message-ID`, without angle brackets.
    pub message_id: Option<String>,
    pub subject: String,
    /// Its inbox tab, if it was sorted into one.
    pub category: Option<MailCategory>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Mode;
    use katna_core::Paths;

    fn store() -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(dir.path());
        let store = Store::open(&paths, Mode::ReadWrite).unwrap();
        (dir, store)
    }

    #[test]
    fn values_are_set_replaced_and_removed() {
        let (_dir, mut store) = store();
        store
            .set_meta("message", 7, "snooze", r#"{"until":100}"#, Some(100))
            .unwrap();
        let first = store.meta("message", 7, "snooze").unwrap().unwrap();
        assert_eq!(first.expires_at, Some(100));
        assert_eq!(first.version, 0);
        store
            .set_meta("message", 7, "snooze", r#"{"until":200}"#, Some(200))
            .unwrap();
        let second = store.meta("message", 7, "snooze").unwrap().unwrap();
        assert_eq!(second.value_json, r#"{"until":200}"#);
        assert_eq!(second.version, 1);
        assert!(store.remove_meta("message", 7, "snooze").unwrap());
        assert!(!store.remove_meta("message", 7, "snooze").unwrap());
        assert_eq!(store.meta("message", 7, "snooze").unwrap(), None);
    }

    #[test]
    fn due_values_come_soonest_first() {
        let (_dir, mut store) = store();
        assert_eq!(store.next_meta_expiry().unwrap(), None);
        store
            .set_meta("message", 1, "snooze", "{}", Some(300))
            .unwrap();
        store
            .set_meta("message", 2, "snooze", "{}", Some(100))
            .unwrap();
        store
            .set_meta("outbox", 3, "follow-up", "{}", Some(200))
            .unwrap();
        store
            .set_meta("message", 4, "pinned-note", "{}", None)
            .unwrap();
        assert_eq!(store.next_meta_expiry().unwrap(), Some(100));
        let due: Vec<i64> = store
            .due_meta(200)
            .unwrap()
            .iter()
            .map(|m| m.object_id)
            .collect();
        assert_eq!(due, [2, 3]);
        let snoozed: Vec<i64> = store
            .meta_of("snooze")
            .unwrap()
            .iter()
            .map(|m| m.object_id)
            .collect();
        assert_eq!(snoozed, [2, 1]);
    }

    #[test]
    fn a_reader_cannot_write() {
        let dir = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(dir.path());
        drop(Store::open(&paths, Mode::ReadWrite).unwrap());
        let mut reader = Store::open(&paths, Mode::ReadOnly).unwrap();
        assert!(reader.set_meta("message", 1, "snooze", "{}", None).is_err());
    }

    #[test]
    fn finds_what_reached_the_inbox_after_a_snooze() {
        use crate::FolderRole;
        use crate::mail::{Added, MessageFlags, NewMessage};
        let (_dir, mut store) = store();
        let account = store
            .add_account(katna_core::AccountKind::Local, "a", "a@local")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let sent = batch
            .upsert_folder(account, "Sent", Some(FolderRole::Sent))
            .unwrap();
        let mut add = |folder, id: &str, reply: Option<&str>, date| {
            let raw = format!("Message-ID: {id}\r\nSubject: Offer\r\n\r\nHi.\r\n");
            let added = batch
                .add_message(
                    account,
                    folder,
                    &NewMessage {
                        raw: raw.as_bytes(),
                        message_id_hdr: Some(id),
                        subject: Some("Offer"),
                        date: Some(date),
                        flags: MessageFlags::SEEN,
                        has_attachments: false,
                        list_id: None,
                        snippet: None,
                        participants: &[],
                        in_reply_to: reply,
                        references: &[],
                        category: None,
                    },
                )
                .unwrap();
            let Added::Message(id) = added else {
                unreachable!()
            };
            id
        };
        let first = add(inbox, "<1@x>", None, 100);
        let earlier = add(inbox, "<2@x>", Some("<1@x>"), 200);
        let mine = add(sent, "<3@x>", Some("<2@x>"), 300);
        let newest = MessageId(mine.0);
        let reply = add(inbox, "<4@x>", Some("<3@x>"), 400);
        batch.commit().unwrap();
        assert!(earlier.0 < newest.0 && newest.0 < reply.0);
        // Only what came after the snooze, and only into the Inbox.
        let after = store.arrived_in_inbox_after(first, newest).unwrap();
        assert_eq!(after.len(), 1);
        assert_eq!(after[0].message_id.as_deref(), Some("<4@x>"));
        assert_eq!(store.newest_message().unwrap(), reply);
        assert!(
            store
                .arrived_in_inbox_after(first, reply)
                .unwrap()
                .is_empty()
        );
    }
}
