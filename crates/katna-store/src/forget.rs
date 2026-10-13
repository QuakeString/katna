// SPDX-License-Identifier: GPL-3.0-or-later

//! "Remove the copy" when an app is turned off (Settings › Apps): the rows
//! an app downloaded from the accounts go, so the next sync, once the app
//! is on again, downloads them afresh. Nothing here is ever sent as a
//! delete: the syncs only delete on the service what is marked (`deleted`,
//! `note_gone`, queued calendar steps), never a row that is missing.
//!
//! What would be lost for good stays: anything on this computer only, and
//! anything changed here that has not reached its service yet.

use std::collections::HashSet;

use crate::Store;
use crate::error::Result;

impl Store {
    /// Deletes the account calendars and their events, but not the
    /// calendars in `keep` (with changes still to send), those on this
    /// computer, or those with an event waiting to go to its service.
    /// Returns how many calendars went.
    pub fn forget_account_calendars(&mut self, keep: &HashSet<i64>) -> Result<usize> {
        self.check_writable()?;
        let tx = self.pim.transaction()?;
        let ids: Vec<i64> = {
            let mut statement = tx.prepare(
                "SELECT id FROM calendar c
                 WHERE c.source != 'local' AND c.account_id IS NOT NULL
                   AND NOT EXISTS (
                       SELECT 1 FROM event e WHERE e.calendar_id = c.id AND e.pending != 0
                   )",
            )?;
            statement
                .query_map([], |row| row.get(0))?
                .collect::<rusqlite::Result<_>>()?
        };
        let mut gone = 0;
        for id in ids.into_iter().filter(|id| !keep.contains(id)) {
            gone += tx.execute("DELETE FROM calendar WHERE id = ?1", [id])?;
        }
        tx.commit()?;
        Ok(gone)
    }

    /// Deletes the accounts' address books (their contacts, groups and
    /// pictures go with them) and their other contacts, with where each
    /// read left off. Books on this computer stay. Returns whether anything
    /// went.
    pub fn forget_account_contacts(&mut self) -> Result<bool> {
        self.check_writable()?;
        let tx = self.pim.transaction()?;
        let books = tx.execute(
            "DELETE FROM address_book WHERE source != 'local' AND account_id IS NOT NULL",
            [],
        )?;
        let others = tx.execute("DELETE FROM other_contact", [])?;
        tx.execute("UPDATE other_contact_sync SET sync_token = NULL", [])?;
        tx.commit()?;
        Ok(books + others > 0)
    }

    /// Deletes the accounts' task lists and their tasks, but not a list
    /// with anything still to send (a list, task or file added, changed
    /// or deleted here), nor one holding what only this computer keeps (a
    /// task's link to its mail, a file kept here only). Returns how many
    /// lists went.
    pub fn forget_account_task_lists(&mut self) -> Result<usize> {
        self.check_writable()?;
        Ok(self.pim.execute(
            "DELETE FROM task_list
             WHERE account_id IS NOT NULL
               AND remote_id IS NOT NULL AND dirty = 0 AND deleted = 0 AND move_out = 0
               AND NOT EXISTS (
                   SELECT 1 FROM task t
                   WHERE t.list_id = task_list.id
                     AND (t.dirty != 0 OR t.deleted != 0 OR t.remote_id IS NULL
                          OR t.mail != ''
                          OR EXISTS (
                              SELECT 1 FROM task_file f
                              WHERE f.task_id = t.id
                                AND (f.local_only != 0 OR f.deleted != 0
                                     OR f.remote_id IS NULL)
                          ))
               )",
            [],
        )?)
    }

    /// Deletes the notes the accounts keep (their pictures, reminders and
    /// past versions go with them), but not a note changed here and not
    /// sent yet, one on this computer only, or one in Trash (Trash is on
    /// this computer only). `note_gone` stays, so notes deleted here still
    /// go from their accounts. Returns how many notes went.
    pub fn forget_account_notes(&mut self) -> Result<usize> {
        self.check_writable()?;
        Ok(self.pim.execute(
            "DELETE FROM note
             WHERE account_id IS NOT NULL AND server_uid IS NOT NULL
               AND dirty = 0 AND trashed_at IS NULL",
            [],
        )?)
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

    fn count(store: &Store, table: &str) -> i64 {
        store
            .pim
            .query_row(&format!("SELECT count(*) FROM {table}"), [], |r| r.get(0))
            .unwrap()
    }

    #[test]
    fn calendars_with_changes_to_send_or_here_only_stay() {
        let (_dir, mut store) = store();
        store
            .pim
            .execute_batch(
                "INSERT INTO calendar (id, account_id, source, remote_id, name)
                 VALUES (1, 1, 'google', 'a', 'Synced'), (2, 1, 'google', 'b', 'Sending'),
                        (3, NULL, 'local', '', 'Here'), (4, 1, 'caldav', 'd', 'Held');
                 INSERT INTO event (calendar_id, start, end, pending)
                 VALUES (1, 0, 1, 0), (2, 0, 1, 1);",
            )
            .unwrap();
        let keep = HashSet::from([4]);
        assert_eq!(store.forget_account_calendars(&keep).unwrap(), 1);
        let left: Vec<i64> = store
            .pim
            .prepare("SELECT id FROM calendar ORDER BY id")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        assert_eq!(left, [2, 3, 4]);
        assert_eq!(count(&store, "event"), 1, "its events went with it");
    }

    #[test]
    fn notes_not_sent_here_only_or_in_trash_stay() {
        let (_dir, mut store) = store();
        store
            .pim
            .execute_batch(
                "INSERT INTO note (account_id, uuid, created_at, updated_at, server_uid, dirty, trashed_at)
                 VALUES (1, 'a', 0, 0, 10, 0, NULL), (1, 'b', 0, 0, 11, 1, NULL),
                        (NULL, 'c', 0, 0, NULL, 0, NULL), (1, 'd', 0, 0, 12, 0, 5),
                        (1, 'e', 0, 0, NULL, 0, NULL);
                 INSERT INTO note_gone (account_id, server_uid) VALUES (1, 9);",
            )
            .unwrap();
        assert_eq!(store.forget_account_notes().unwrap(), 1);
        assert_eq!(count(&store, "note"), 4);
        assert_eq!(count(&store, "note_gone"), 1, "deletes still go out");
    }

    #[test]
    fn task_lists_with_anything_to_send_or_kept_here_stay() {
        let (_dir, mut store) = store();
        store
            .pim
            .execute_batch(
                "PRAGMA foreign_keys = OFF;
                 INSERT INTO task_list (id, account_id, remote_id, title)
                 VALUES (11, 1, 'a', 'Synced'), (12, 1, 'b', 'Edited'), (13, 1, 'c', 'Linked'),
                        (14, NULL, NULL, 'Here'), (15, 1, NULL, 'New');
                 INSERT INTO task (list_id, title, remote_id, dirty, mail, created_at, updated_at)
                 VALUES (11, 'a', 'x', 0, '', 0, 0), (12, 'b', 'y', 1, '', 0, 0),
                        (13, 'c', 'z', 0, '<m@x>', 0, 0);
                 PRAGMA foreign_keys = ON;",
            )
            .unwrap();
        let (lists, tasks) = (count(&store, "task_list"), count(&store, "task"));
        assert_eq!(store.forget_account_task_lists().unwrap(), 1);
        assert_eq!(count(&store, "task_list"), lists - 1);
        assert_eq!(count(&store, "task"), tasks - 1, "its tasks went with it");
    }
}
