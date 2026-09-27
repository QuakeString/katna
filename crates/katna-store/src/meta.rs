// SPDX-License-Identifier: GPL-3.0-or-later

//! The `meta` table of `pim.db`: a JSON value per object and feature, with
//! an optional expiry (`docs/ARCHITECTURE.md` §10). `katna-meta` gives the
//! values their types and runs the scheduler in the daemon; this module
//! only stores them.

use katna_core::AccountId;
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

    /// Whether the conversation of `message` has a message written after
    /// it (another `Message-ID`, a later date): a reply, or a follow-up.
    pub fn has_later_in_thread(&self, message: MessageId) -> Result<bool> {
        Ok(self
            .mail
            .prepare_cached(
                "SELECT EXISTS (
                     SELECT 1 FROM message m JOIN message o ON o.thread_id = m.thread_id
                     WHERE m.id = ?1 AND o.id <> m.id
                       AND o.message_id_hdr IS NOT m.message_id_hdr
                       AND o.date > m.date
                       AND EXISTS (SELECT 1 FROM message_location l WHERE l.message_id = o.id)
                 )",
            )?
            .query_row([message.0], |row| row.get(0))?)
    }
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
}
