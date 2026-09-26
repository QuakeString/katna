// SPDX-License-Identifier: GPL-3.0-or-later

//! Gmail messages stored before schema v4, one row per label.
//!
//! Since v4 a Gmail message is one row with a location per label, found by
//! its `X-GM-MSGID` (`message.gm_msgid`). Stores synced before that have a
//! row per label, all without `gm_msgid`: the same mail twice in counts,
//! search and conversations, and a flag change that reaches only one copy.
//! `katna_sync` fetches the missing ids from the server and hands each to
//! [`MailBatch::adopt_gm_msgid`], which sets it, or merges the row into the
//! one that already has it. Nothing changes on the server.

use katna_core::AccountId;
use rusqlite::{OptionalExtension, params};

use crate::Store;
use crate::error::Result;
use crate::journal::{self, ChangeOp, ObjectKind};
use crate::mail::{FolderId, MailBatch, MessageId};

/// What [`MailBatch::adopt_gm_msgid`] did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adopted {
    /// The message got its `gm_msgid`; no other row had it.
    Set(MessageId),
    /// The message was a second copy of this one and is now part of it.
    Merged(MessageId),
    /// A local change of this copy is still waiting for the server, so it
    /// is left alone this time.
    Busy,
    /// No such UID, or it has its `gm_msgid` already.
    Unchanged,
}

impl Store {
    /// UIDs above `after` in `folder`, ascending, of messages without
    /// `gm_msgid`. On Gmail these were stored before schema v4. At most
    /// `limit`.
    pub fn uids_without_gm_msgid(
        &self,
        folder: FolderId,
        after: u32,
        limit: u32,
    ) -> Result<Vec<u32>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT l.uid FROM message_location l
             JOIN message m ON m.id = l.message_id
             WHERE l.folder_id = ?1 AND l.uid > ?2 AND m.gm_msgid IS NULL
             ORDER BY l.uid LIMIT ?3",
        )?;
        let uids = stmt.query_map(params![folder.0, after, limit], |row| row.get(0))?;
        Ok(uids.collect::<rusqlite::Result<_>>()?)
    }
}

impl MailBatch<'_> {
    /// Records that the message at `uid` in `folder` is Gmail's message
    /// `gm_msgid`. When another row of `account` already is, the two become
    /// that one row:
    ///
    /// - it gets this row's folders (Gmail labels), flags and keywords, so
    ///   nothing set on either copy is lost;
    /// - it takes this row's body, category, thread, attachments and
    ///   senders where it has none;
    /// - a pin carries over;
    /// - this row is deleted.
    pub fn adopt_gm_msgid(
        &mut self,
        account: AccountId,
        folder: FolderId,
        uid: u32,
        gm_msgid: u64,
    ) -> Result<Adopted> {
        let tx = self.tx();
        let found: Option<(i64, Option<i64>)> = tx
            .prepare_cached(
                "SELECT m.id, m.gm_msgid FROM message_location l
                 JOIN message m ON m.id = l.message_id
                 WHERE l.folder_id = ?1 AND l.uid = ?2",
            )?
            .query_row(params![folder.0, uid], |row| Ok((row.get(0)?, row.get(1)?)))
            .optional()?;
        let Some((copy, None)) = found else {
            return Ok(Adopted::Unchanged);
        };
        let gm_msgid = gm_msgid as i64;
        let keeper: Option<i64> = tx
            .prepare_cached("SELECT id FROM message WHERE account_id = ?1 AND gm_msgid = ?2")?
            .query_row(params![account.0, gm_msgid], |row| row.get(0))
            .optional()?;
        let Some(keeper) = keeper else {
            tx.prepare_cached("UPDATE message SET gm_msgid = ?2 WHERE id = ?1")?
                .execute(params![copy, gm_msgid])?;
            journal::record(tx, ObjectKind::Message, copy, ChangeOp::Update)?;
            return Ok(Adopted::Set(MessageId(copy)));
        };
        // Queued changes and the outbox name the row by ID; merge once they
        // are done.
        let busy: bool = tx
            .prepare_cached(
                "SELECT EXISTS (SELECT 1 FROM op_queue
                                WHERE state IN ('pending', 'running')
                                  AND json_extract(op_json, '$.message') = ?1)
                     OR EXISTS (SELECT 1 FROM outbox WHERE draft_message_id = ?1)",
            )?
            .query_row([copy], |row| row.get(0))?;
        if busy {
            return Ok(Adopted::Busy);
        }

        let keywords: (Option<String>, Option<String>) = tx
            .prepare_cached(
                "SELECT k.keywords, c.keywords FROM message k, message c
                 WHERE k.id = ?1 AND c.id = ?2",
            )?
            .query_row(params![keeper, copy], |row| Ok((row.get(0)?, row.get(1)?)))?;
        let keywords = merged_keywords(keywords.0.as_deref(), keywords.1.as_deref());
        tx.prepare_cached(
            "UPDATE message SET
                 flags = flags | (SELECT flags FROM message WHERE id = ?2),
                 keywords = ?3,
                 thread_id = coalesce(thread_id, (SELECT thread_id FROM message WHERE id = ?2)),
                 category = coalesce(category, (SELECT category FROM message WHERE id = ?2))
             WHERE id = ?1",
        )?
        .execute(params![keeper, copy, keywords])?;
        // The downloaded body, with what was read from it.
        tx.prepare_cached(
            "UPDATE message SET
                 (body_state, blob_hash, snippet, has_attachments, auth_results_json) =
                 (SELECT body_state, blob_hash, snippet, has_attachments, auth_results_json
                  FROM message WHERE id = ?2)
             WHERE id = ?1 AND blob_hash IS NULL
               AND (SELECT blob_hash FROM message WHERE id = ?2) IS NOT NULL",
        )?
        .execute(params![keeper, copy])?;
        tx.prepare_cached(
            "UPDATE OR IGNORE message_location SET message_id = ?1 WHERE message_id = ?2",
        )?
        .execute(params![keeper, copy])?;
        tx.prepare_cached("UPDATE OR IGNORE attachment SET message_id = ?1 WHERE message_id = ?2")?
            .execute(params![keeper, copy])?;
        tx.prepare_cached(
            "UPDATE participant SET message_id = ?1 WHERE message_id = ?2
             AND NOT EXISTS (SELECT 1 FROM participant WHERE message_id = ?1)",
        )?
        .execute(params![keeper, copy])?;
        tx.prepare_cached(
            "INSERT OR IGNORE INTO pin (message_id, pinned_at)
             SELECT ?1, pinned_at FROM pin WHERE message_id = ?2",
        )?
        .execute(params![keeper, copy])?;
        // Cascades take what is left: locations and attachments the kept
        // row has already, senders, the pin.
        tx.prepare_cached("DELETE FROM message WHERE id = ?1")?
            .execute([copy])?;
        journal::record(tx, ObjectKind::Message, copy, ChangeOp::Delete)?;
        journal::record(tx, ObjectKind::Message, keeper, ChangeOp::Update)?;
        Ok(Adopted::Merged(MessageId(keeper)))
    }
}

/// Both keyword lists (JSON arrays), without repeats, in first-seen order.
fn merged_keywords(a: Option<&str>, b: Option<&str>) -> Option<String> {
    let mut all: Vec<String> = Vec::new();
    for list in [a, b].into_iter().flatten() {
        let list: Vec<String> = serde_json::from_str(list).unwrap_or_default();
        for keyword in list {
            if !all.contains(&keyword) {
                all.push(keyword);
            }
        }
    }
    (!all.is_empty()).then(|| serde_json::to_string(&all).expect("strings serialize"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MessageFlags, Mode, RemoteMessage};
    use katna_core::{AccountKind, Paths};

    fn open() -> (tempfile::TempDir, Store, AccountId, FolderId, FolderId) {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Imap, "Gmail", "ada@gmail.com")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch.upsert_folder(account, "INBOX", None).unwrap();
        let all = batch
            .upsert_folder(account, "[Gmail]/All Mail", None)
            .unwrap();
        batch.commit().unwrap();
        (tmp, store, account, inbox, all)
    }

    /// Stores a copy the way syncs before schema v4 did: no `gm_msgid`.
    fn old_copy(
        store: &mut Store,
        account: AccountId,
        folder: FolderId,
        uid: u32,
        flags: MessageFlags,
        keywords: &[String],
    ) -> MessageId {
        let mut batch = store.mail_batch().unwrap();
        let added = batch
            .add_remote_message(
                account,
                folder,
                &RemoteMessage {
                    uid,
                    message_id_hdr: Some("1@example.org"),
                    subject: Some("Hello"),
                    date: Some(1_790_000_000),
                    size: 1234,
                    flags,
                    keywords,
                    has_attachments: false,
                    list_id: None,
                    participants: &[],
                    in_reply_to: None,
                    references: &[],
                    gm_thread_id: None,
                    gm_msgid: None,
                    category: None,
                    attachments: &[],
                },
            )
            .unwrap();
        batch.commit().unwrap();
        match added {
            crate::Added::Message(id) => id,
            other => panic!("stored as {other:?}"),
        }
    }

    fn adopt(store: &mut Store, account: AccountId, folder: FolderId, uid: u32) -> Adopted {
        let mut batch = store.mail_batch().unwrap();
        let adopted = batch.adopt_gm_msgid(account, folder, uid, 77).unwrap();
        batch.commit().unwrap();
        adopted
    }

    fn count(store: &Store, sql: &str) -> i64 {
        store.mail.query_row(sql, [], |row| row.get(0)).unwrap()
    }

    #[test]
    fn merges_copies_stored_before_v4() {
        let (_tmp, mut store, account, inbox, all) = open();
        let starred = ["$Work".to_owned()];
        let first = old_copy(&mut store, account, inbox, 5, MessageFlags::SEEN, &[]);
        let second = old_copy(
            &mut store,
            account,
            all,
            50,
            MessageFlags::FLAGGED,
            &starred,
        );
        let mut batch = store.mail_batch().unwrap();
        batch
            .set_message_body(second, b"Subject: Hello\r\n\r\nHi", Some("Hi"), false)
            .unwrap();
        batch.set_pinned(second, Some(1_790_000_100)).unwrap();
        batch.commit().unwrap();
        assert_eq!(store.uids_without_gm_msgid(all, 0, 10).unwrap(), [50]);

        assert_eq!(adopt(&mut store, account, inbox, 5), Adopted::Set(first));
        assert_eq!(adopt(&mut store, account, all, 50), Adopted::Merged(first));

        assert_eq!(count(&store, "SELECT count(*) FROM message"), 1);
        let (flags, keywords, body, gm): (u32, Option<String>, Option<Vec<u8>>, i64) = store
            .mail
            .query_row(
                "SELECT flags, keywords, blob_hash, gm_msgid FROM message",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .unwrap();
        assert_eq!(flags, (MessageFlags::SEEN | MessageFlags::FLAGGED).bits());
        assert_eq!(keywords.as_deref(), Some(r#"["$Work"]"#));
        assert!(body.is_some());
        assert_eq!(gm, 77);
        let folders: Vec<FolderId> = store
            .locations(first)
            .unwrap()
            .into_iter()
            .map(|l| l.folder)
            .collect();
        assert_eq!(folders, [inbox, all]);
        assert_eq!(store.pinned().unwrap()[0].message, first);
        assert!(store.uids_without_gm_msgid(all, 0, 10).unwrap().is_empty());
        assert_eq!(count(&store, "SELECT sum(message_count) FROM thread"), 1);

        // Done once: asking again changes nothing.
        assert_eq!(adopt(&mut store, account, all, 50), Adopted::Unchanged);
    }

    #[test]
    fn waits_for_queued_changes_of_a_copy() {
        let (_tmp, mut store, account, inbox, all) = open();
        old_copy(&mut store, account, inbox, 5, MessageFlags::empty(), &[]);
        let second = old_copy(&mut store, account, all, 50, MessageFlags::empty(), &[]);
        let mut batch = store.mail_batch().unwrap();
        let op = format!(r#"{{"op":"flags","message":{}}}"#, second.0);
        batch.enqueue_op(account, &op).unwrap();
        batch.commit().unwrap();

        adopt(&mut store, account, inbox, 5);
        assert_eq!(adopt(&mut store, account, all, 50), Adopted::Busy);
        assert_eq!(count(&store, "SELECT count(*) FROM message"), 2);

        store
            .mail
            .execute("UPDATE op_queue SET state = 'done'", [])
            .unwrap();
        assert!(matches!(
            adopt(&mut store, account, all, 50),
            Adopted::Merged(_)
        ));
        assert_eq!(count(&store, "SELECT count(*) FROM message"), 1);
    }

    #[test]
    fn keywords_are_merged_without_repeats() {
        assert_eq!(merged_keywords(None, None), None);
        assert_eq!(
            merged_keywords(Some(r#"["a","b"]"#), Some(r#"["b","c"]"#)).as_deref(),
            Some(r#"["a","b","c"]"#)
        );
    }
}
