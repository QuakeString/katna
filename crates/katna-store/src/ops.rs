// SPDX-License-Identifier: GPL-3.0-or-later

//! Local changes waiting to be replayed on the server (`op_queue`), and the
//! optimistic edits that go with them (`docs/ARCHITECTURE.md` §6.1).
//!
//! The operations themselves are JSON owned by `katna-sync`; this module
//! only stores and schedules them.

use katna_core::{AccountId, MailCategory};
use rusqlite::{OptionalExtension, params};

use crate::error::Result;
use crate::journal::{self, ChangeOp, ObjectKind};
use crate::mail::{FolderId, MailBatch, MessageFlags, MessageId};
use crate::remote::remove_location;
use crate::{Store, ThreadId};

/// One queued operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueuedOp {
    pub id: i64,
    pub account: AccountId,
    pub op_json: String,
    /// Failed tries so far.
    pub attempts: u32,
}

/// Where a message is stored: a folder, and its UID there if known.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub folder: FolderId,
    pub uid: Option<u32>,
}

/// A pinned message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PinnedMessage {
    pub message: MessageId,
    pub account: AccountId,
    pub thread: Option<ThreadId>,
    /// Unix seconds.
    pub pinned_at: i64,
}

impl Store {
    /// How many changes and messages wait to go to each account's
    /// servers: queued operations and unsent mail in the outbox. Accounts
    /// with none are left out.
    pub fn waiting(&self) -> Result<std::collections::HashMap<AccountId, u64>> {
        let mut waiting = std::collections::HashMap::new();
        let mut stmt = self.mail.prepare_cached(
            "SELECT account_id, count(*) FROM op_queue
             WHERE state IN ('pending', 'running') GROUP BY account_id",
        )?;
        let rows = stmt.query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)))?;
        for row in rows {
            let (account, count) = row?;
            *waiting.entry(AccountId(account)).or_insert(0) += count.max(0) as u64;
        }
        for entry in self.outbox()? {
            if matches!(
                entry.state,
                crate::SendState::Queued | crate::SendState::Sending
            ) {
                *waiting.entry(entry.account).or_insert(0) += 1;
            }
        }
        Ok(waiting)
    }

    /// Every pinned message, most recently pinned first.
    pub fn pinned(&self) -> Result<Vec<PinnedMessage>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT p.message_id, m.account_id, m.thread_id, p.pinned_at
             FROM pin p JOIN message m ON m.id = p.message_id
             ORDER BY p.pinned_at DESC, p.message_id DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(PinnedMessage {
                message: MessageId(row.get(0)?),
                account: AccountId(row.get(1)?),
                thread: row.get::<_, Option<i64>>(2)?.map(ThreadId),
                pinned_at: row.get(3)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Pending operations of `account` that are due at `now`, oldest
    /// first, at most `limit`.
    pub fn due_ops(&self, account: AccountId, now: i64, limit: u32) -> Result<Vec<QueuedOp>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT id, account_id, op_json, attempts FROM op_queue
             WHERE account_id = ?1 AND state = 'pending'
               AND (next_try_at IS NULL OR next_try_at <= ?2)
             ORDER BY id LIMIT ?3",
        )?;
        let rows = stmt.query_map(params![account.0, now, limit], |row| {
            Ok(QueuedOp {
                id: row.get(0)?,
                account: AccountId(row.get(1)?),
                op_json: row.get(2)?,
                attempts: row.get(3)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// When the next pending operation of `account` is due (Unix seconds;
    /// 0 for now), or `None` if nothing is pending.
    pub fn next_op_due(&self, account: AccountId) -> Result<Option<i64>> {
        Ok(self
            .mail
            .prepare_cached(
                "SELECT min(coalesce(next_try_at, 0)) FROM op_queue
                 WHERE account_id = ?1 AND state = 'pending'",
            )?
            .query_row([account.0], |row| row.get(0))?)
    }

    /// The folders `message` is stored in.
    pub fn locations(&self, message: MessageId) -> Result<Vec<Location>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT folder_id, uid FROM message_location WHERE message_id = ?1
             ORDER BY folder_id",
        )?;
        let rows = stmt.query_map([message.0], |row| {
            Ok(Location {
                folder: FolderId(row.get(0)?),
                uid: row.get(1)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The inbox tab `message` was sorted into, if it was.
    pub fn message_category(&self, message: MessageId) -> Result<Option<MailCategory>> {
        let stored: Option<Option<i64>> = self
            .mail
            .prepare_cached("SELECT category FROM message WHERE id = ?1")?
            .query_row([message.0], |row| row.get(0))
            .optional()?;
        Ok(stored.flatten().and_then(MailCategory::from_storage))
    }
}

impl MailBatch<'_> {
    /// Queues an operation for `account`. Returns its ID.
    pub fn enqueue_op(&mut self, account: AccountId, op_json: &str) -> Result<i64> {
        let tx = self.tx();
        tx.prepare_cached("INSERT INTO op_queue (account_id, op_json) VALUES (?1, ?2)")?
            .execute(params![account.0, op_json])?;
        Ok(tx.last_insert_rowid())
    }

    /// Forgets every operation of `account`, for example when it is
    /// removed. Returns how many there were.
    pub fn clear_ops(&mut self, account: AccountId) -> Result<usize> {
        Ok(self
            .tx()
            .prepare_cached("DELETE FROM op_queue WHERE account_id = ?1")?
            .execute([account.0])?)
    }

    /// Pending operations of `account`, due or not, oldest first.
    pub fn pending_ops(&mut self, account: AccountId) -> Result<Vec<QueuedOp>> {
        let mut stmt = self.tx().prepare_cached(
            "SELECT id, account_id, op_json, attempts FROM op_queue
             WHERE account_id = ?1 AND state = 'pending' ORDER BY id",
        )?;
        let rows = stmt.query_map([account.0], |row| {
            Ok(QueuedOp {
                id: row.get(0)?,
                account: AccountId(row.get(1)?),
                op_json: row.get(2)?,
                attempts: row.get(3)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Replaces a queued operation, for example once a UID it waited for
    /// is known.
    pub fn set_op_json(&mut self, id: i64, op_json: &str) -> Result<()> {
        self.tx()
            .prepare_cached("UPDATE op_queue SET op_json = ?2 WHERE id = ?1")?
            .execute(params![id, op_json])?;
        Ok(())
    }

    /// The operation is done: forget it.
    pub fn finish_op(&mut self, id: i64) -> Result<()> {
        self.tx()
            .prepare_cached("DELETE FROM op_queue WHERE id = ?1")?
            .execute([id])?;
        Ok(())
    }

    /// The operation failed and will be tried again at `next_try_at`.
    pub fn retry_op(&mut self, id: i64, next_try_at: i64) -> Result<()> {
        self.tx()
            .prepare_cached(
                "UPDATE op_queue SET attempts = attempts + 1, next_try_at = ?2 WHERE id = ?1",
            )?
            .execute(params![id, next_try_at])?;
        Ok(())
    }

    /// The operation failed for good; it stays for inspection.
    pub fn fail_op(&mut self, id: i64) -> Result<()> {
        self.tx()
            .prepare_cached(
                "UPDATE op_queue SET attempts = attempts + 1, state = 'failed' WHERE id = ?1",
            )?
            .execute([id])?;
        Ok(())
    }

    /// Sets the flags of `message`. Returns whether they changed.
    pub fn set_message_flags(&mut self, message: MessageId, flags: MessageFlags) -> Result<bool> {
        let tx = self.tx();
        let changed = tx
            .prepare_cached("UPDATE message SET flags = ?2 WHERE id = ?1 AND flags != ?2")?
            .execute(params![message.0, flags.bits()])?
            > 0;
        if changed {
            journal::record(tx, ObjectKind::Message, message.0, ChangeOp::Update)?;
        }
        Ok(changed)
    }

    /// Pins `message` at Unix time `at`, or unpins it with `None`.
    /// Returns whether that changed anything.
    pub fn set_pinned(&mut self, message: MessageId, at: Option<i64>) -> Result<bool> {
        let tx = self.tx();
        let changed = match at {
            Some(at) => tx
                .prepare_cached(
                    "INSERT OR IGNORE INTO pin (message_id, pinned_at) VALUES (?1, ?2)",
                )?
                .execute(params![message.0, at])?,
            None => tx
                .prepare_cached("DELETE FROM pin WHERE message_id = ?1")?
                .execute([message.0])?,
        };
        Ok(changed > 0)
    }

    /// Moves `message` from folder `from` to `to`, where its UID is `uid`
    /// (`None` until the server reports it). Returns whether it was in
    /// `from`. A message already in `to` (a Gmail message archived to All
    /// Mail) only leaves `from`, and keeps its UID in `to` unless `uid`
    /// gives one.
    pub fn move_location(
        &mut self,
        message: MessageId,
        from: FolderId,
        to: FolderId,
        uid: Option<u32>,
    ) -> Result<bool> {
        let tx = self.tx();
        let already_there = tx
            .prepare_cached(
                "SELECT 1 FROM message_location WHERE message_id = ?1 AND folder_id = ?2",
            )?
            .query_row(params![message.0, to.0], |_| Ok(()))
            .optional()?
            .is_some();
        if already_there && from != to {
            let left = tx
                .prepare_cached(
                    "DELETE FROM message_location WHERE message_id = ?1 AND folder_id = ?2",
                )?
                .execute(params![message.0, from.0])?
                > 0;
            if uid.is_some() {
                tx.prepare_cached(
                    "UPDATE message_location SET uid = ?3 WHERE message_id = ?1 AND folder_id = ?2",
                )?
                .execute(params![message.0, to.0, uid])?;
            }
            if left {
                journal::record(tx, ObjectKind::Message, message.0, ChangeOp::Update)?;
            }
            return Ok(left);
        }
        let moved = tx
            .prepare_cached(
                "UPDATE message_location SET folder_id = ?3, uid = ?4
                 WHERE message_id = ?1 AND folder_id = ?2",
            )?
            .execute(params![message.0, from.0, to.0, uid])?
            > 0;
        if moved {
            journal::record(tx, ObjectKind::Message, message.0, ChangeOp::Update)?;
        }
        Ok(moved)
    }

    /// Adds `message` to `folder` as well, where its UID is `uid` (`None`
    /// until the server reports it): a copy, or a Gmail label. Returns
    /// whether it was not there yet.
    pub fn add_location(
        &mut self,
        message: MessageId,
        folder: FolderId,
        uid: Option<u32>,
    ) -> Result<bool> {
        let tx = self.tx();
        let added = tx
            .prepare_cached(
                "INSERT OR IGNORE INTO message_location (message_id, folder_id, uid)
                 VALUES (?1, ?2, ?3)",
            )?
            .execute(params![message.0, folder.0, uid])?
            > 0;
        if added {
            journal::record(tx, ObjectKind::Message, message.0, ChangeOp::Update)?;
        }
        Ok(added)
    }

    /// The message in `folder` with no known UID yet, if any: a local move
    /// the server has not confirmed.
    pub fn unconfirmed_in(&self, message: MessageId, folder: FolderId) -> Result<bool> {
        Ok(self
            .tx()
            .prepare_cached(
                "SELECT 1 FROM message_location
                 WHERE message_id = ?1 AND folder_id = ?2 AND uid IS NULL",
            )?
            .query_row(params![message.0, folder.0], |_| Ok(()))
            .optional()?
            .is_some())
    }

    /// The messages in `folder` whose `Message-ID` is `message_id`
    /// (without angle brackets): the saved copies of one draft.
    pub fn with_message_id_in(&self, folder: FolderId, message_id: &str) -> Result<Vec<MessageId>> {
        let mut stmt = self.tx().prepare_cached(
            "SELECT m.id FROM message m JOIN message_location l ON l.message_id = m.id
             WHERE l.folder_id = ?1 AND m.message_id_hdr = ?2 ORDER BY m.id",
        )?;
        let rows = stmt.query_map(params![folder.0, message_id], |row| {
            Ok(MessageId(row.get(0)?))
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Makes the next sync of `folder` fetch every flag again, replacing
    /// local changes the server refused.
    pub fn forget_modseq(&mut self, folder: FolderId) -> Result<()> {
        self.tx()
            .prepare_cached("UPDATE folder SET highestmodseq = NULL WHERE id = ?1")?
            .execute([folder.0])?;
        Ok(())
    }

    /// Removes `message` from `folder`, and the message itself if that was
    /// its last folder.
    pub fn remove_from_folder(&mut self, message: MessageId, folder: FolderId) -> Result<()> {
        remove_location(self.tx(), message, folder)
    }

    /// Takes back a move the server refused: `message` goes from `to` back
    /// to `from` at `uid`. When a sync has meanwhile listed that UID in
    /// `from` as another message, that one stays and `message` only leaves
    /// `to`, so the mail isn't shown twice.
    pub fn move_back(
        &mut self,
        message: MessageId,
        to: FolderId,
        from: FolderId,
        uid: Option<u32>,
    ) -> Result<()> {
        let tx = self.tx();
        let taken = match uid {
            Some(uid) => tx
                .prepare_cached(
                    "SELECT 1 FROM message_location
                     WHERE folder_id = ?1 AND uid = ?2 AND message_id != ?3",
                )?
                .query_row(params![from.0, uid, message.0], |_| Ok(()))
                .optional()?
                .is_some(),
            None => false,
        };
        if taken {
            return remove_location(tx, message, to);
        }
        self.move_location(message, to, from, uid)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FolderRole, Mode, RemoteMessage};
    use katna_core::{AccountKind, Paths};

    #[test]
    fn ops_are_scheduled_retried_and_finished() {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Imap, "Work", "ada@example.org")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let first = batch.enqueue_op(account, r#"{"op":"a"}"#).unwrap();
        let second = batch.enqueue_op(account, r#"{"op":"b"}"#).unwrap();
        batch.commit().unwrap();
        assert_eq!(store.waiting().unwrap().get(&account), Some(&2));
        assert_eq!(store.next_op_due(account).unwrap(), Some(0));
        let due = store.due_ops(account, 100, 10).unwrap();
        assert_eq!(
            due.iter().map(|op| op.id).collect::<Vec<_>>(),
            [first, second]
        );

        let mut batch = store.mail_batch().unwrap();
        batch.retry_op(first, 200).unwrap();
        batch.finish_op(second).unwrap();
        batch.commit().unwrap();
        assert!(store.due_ops(account, 100, 10).unwrap().is_empty());
        assert_eq!(store.next_op_due(account).unwrap(), Some(200));
        let due = store.due_ops(account, 200, 10).unwrap();
        assert_eq!((due[0].id, due[0].attempts), (first, 1));

        let mut batch = store.mail_batch().unwrap();
        batch.fail_op(first).unwrap();
        batch.commit().unwrap();
        assert_eq!(store.next_op_due(account).unwrap(), None);
        assert!(store.due_ops(account, 999, 10).unwrap().is_empty());

        let mut batch = store.mail_batch().unwrap();
        batch.enqueue_op(account, r#"{"op":"c"}"#).unwrap();
        assert_eq!(batch.clear_ops(account).unwrap(), 2, "failed ones too");
        batch.commit().unwrap();
        assert_eq!(store.next_op_due(account).unwrap(), None);
        assert!(store.waiting().unwrap().is_empty());
    }

    #[test]
    fn local_flags_and_moves() {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Imap, "Work", "ada@example.org")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let trash = batch
            .upsert_folder(account, "Trash", Some(FolderRole::Trash))
            .unwrap();
        let message = RemoteMessage {
            uid: 7,
            message_id_hdr: None,
            subject: Some("Hi"),
            date: None,
            size: 1,
            flags: MessageFlags::empty(),
            keywords: &[],
            has_attachments: false,
            list_id: None,
            participants: &[],
            in_reply_to: None,
            references: &[],
            gm_thread_id: None,
            gm_msgid: None,
            category: None,
            attachments: &[],
        };
        let crate::Added::Message(id) = batch.add_remote_message(account, inbox, &message).unwrap()
        else {
            panic!("new message expected");
        };
        batch.commit().unwrap();

        let mut batch = store.mail_batch().unwrap();
        assert!(batch.set_pinned(id, Some(100)).unwrap());
        assert!(!batch.set_pinned(id, Some(200)).unwrap(), "pinned already");
        batch.commit().unwrap();
        assert_eq!(
            store.pinned().unwrap(),
            [PinnedMessage {
                message: id,
                account,
                thread: store.messages_by_id(&[id]).unwrap()[0].thread_id,
                pinned_at: 100,
            }]
        );

        let mut batch = store.mail_batch().unwrap();
        assert!(batch.set_message_flags(id, MessageFlags::SEEN).unwrap());
        assert!(!batch.set_message_flags(id, MessageFlags::SEEN).unwrap());
        assert!(batch.move_location(id, inbox, trash, None).unwrap());
        assert!(batch.unconfirmed_in(id, trash).unwrap());
        batch.commit().unwrap();
        assert_eq!(
            store.locations(id).unwrap(),
            [Location {
                folder: trash,
                uid: None
            }]
        );
        assert!(store.folder_uids(inbox).unwrap().is_empty());

        let mut batch = store.mail_batch().unwrap();
        assert!(batch.move_location(id, trash, trash, Some(3)).unwrap());
        assert!(!batch.unconfirmed_in(id, trash).unwrap());
        batch.remove_from_folder(id, trash).unwrap();
        batch.commit().unwrap();
        assert!(store.messages_by_id(&[id]).unwrap().is_empty());
        assert!(
            store.pinned().unwrap().is_empty(),
            "pins go with the message"
        );
    }
}
