// SPDX-License-Identifier: GPL-3.0-or-later

//! Threads and categories for messages stored before schema v2, and
//! Gmail's own inbox categories.
//!
//! Stores from before threading have messages with no thread and no
//! category. `katna_import::backfill` re-reads their headers from the blob
//! store; `katna_sync` fetches them again from the server for messages
//! whose body was never downloaded. Both hand the facts to
//! [`MailBatch::backfill_message`], which only fills in what is missing,
//! so running it twice, or from both places, is harmless.

use katna_core::MailCategory;
use rusqlite::{OptionalExtension, params};

use crate::Store;
use crate::blob::BlobHash;
use crate::error::{Error, Result};
use crate::journal::{self, ChangeOp, ObjectKind};
use crate::mail::{FolderId, MailBatch, MessageId};
use crate::thread::Links;
use katna_core::AccountId;

/// Header facts of a stored message that has no thread or category yet.
/// `Message-ID`, subject and date come from the stored row.
#[derive(Debug, Clone, Copy, Default)]
pub struct Backfill<'a> {
    /// `In-Reply-To` without angle brackets.
    pub in_reply_to: Option<&'a str>,
    /// `References`, oldest first, without angle brackets.
    pub references: &'a [&'a str],
    /// Gmail's `X-GM-THRID`.
    pub gm_thread_id: Option<u64>,
    pub category: Option<MailCategory>,
}

impl Store {
    /// Messages with ID above `after` that have their raw message stored
    /// but no thread or no category yet, in ID order, at most `limit`.
    pub fn unthreaded_with_body(
        &self,
        after: MessageId,
        limit: u32,
    ) -> Result<Vec<(MessageId, BlobHash)>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT id, blob_hash FROM message
             WHERE (thread_id IS NULL OR category IS NULL) AND id > ?1
               AND blob_hash IS NOT NULL
             ORDER BY id LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![after.0, limit], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, Vec<u8>>(1)?))
        })?;
        rows.map(|row| {
            let (id, hash) = row?;
            let hash = <[u8; 32]>::try_from(hash.as_slice())
                .map_err(|_| Error::InvalidData(format!("message {id}: bad blob hash")))?;
            Ok((MessageId(id), BlobHash::from_bytes(hash)))
        })
        .collect()
    }

    /// Forgets every thread and category, as in a store from before schema
    /// v2. For measuring and testing the backfill.
    pub fn reset_threads(&mut self) -> Result<()> {
        self.check_writable()?;
        let tx = self
            .mail
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        tx.execute_batch(
            "UPDATE message SET thread_id = NULL, category = NULL
                 WHERE thread_id IS NOT NULL OR category IS NOT NULL;
             DELETE FROM thread_ref;
             DELETE FROM thread;",
        )?;
        tx.commit()?;
        Ok(())
    }

    /// How many messages have no thread or no category yet.
    pub fn unthreaded_count(&self) -> Result<u64> {
        let count: i64 = self
            .mail
            .prepare_cached(
                "SELECT count(*) FROM message WHERE thread_id IS NULL OR category IS NULL",
            )?
            .query_row([], |row| row.get(0))?;
        Ok(count.try_into().unwrap_or_default())
    }

    /// UIDs in `folder`, ascending, of messages without a stored body that
    /// have no thread or no category yet: their headers must come from the
    /// server again. At most `limit`.
    pub fn uids_needing_headers(&self, folder: FolderId, limit: u32) -> Result<Vec<u32>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT l.uid FROM message m
             JOIN message_location l ON l.message_id = m.id AND l.folder_id = ?1
             WHERE (m.thread_id IS NULL OR m.category IS NULL)
               AND m.blob_hash IS NULL AND l.uid IS NOT NULL
             ORDER BY l.uid LIMIT ?2",
        )?;
        let uids = stmt.query_map(params![folder.0, limit], |row| row.get(0))?;
        Ok(uids.collect::<rusqlite::Result<_>>()?)
    }
}

impl MailBatch<'_> {
    /// Gives a stored message its thread and category, where it has none
    /// yet. Returns whether anything changed.
    ///
    /// Journaled as a change of the thread, not of the message, so the
    /// search index is not rebuilt for every old message.
    pub fn backfill_message(&mut self, message: MessageId, facts: &Backfill<'_>) -> Result<bool> {
        type Row = (
            i64,
            Option<String>,
            String,
            Option<i64>,
            Option<i64>,
            Option<i64>,
        );
        let row: Option<Row> = self
            .tx()
            .prepare_cached(
                "SELECT account_id, message_id_hdr, subject, date, thread_id, category
                 FROM message WHERE id = ?1",
            )?
            .query_row([message.0], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            })
            .optional()?;
        let Some((account, message_id_hdr, subject, date, thread, category)) = row else {
            return Ok(false);
        };
        let mut changed = false;
        let thread = match thread {
            Some(thread) => thread,
            None => {
                let thread = self.assign_thread(&Links {
                    account: AccountId(account),
                    message_id_hdr: message_id_hdr.as_deref(),
                    in_reply_to: facts.in_reply_to,
                    references: facts.references,
                    subject: &subject,
                    date,
                    gm_thread_id: facts.gm_thread_id,
                })?;
                self.tx()
                    .prepare_cached("UPDATE message SET thread_id = ?2 WHERE id = ?1")?
                    .execute(params![message.0, thread])?;
                changed = true;
                thread
            }
        };
        if let (None, Some(new)) = (category, facts.category) {
            self.tx()
                .prepare_cached("UPDATE message SET category = ?2 WHERE id = ?1")?
                .execute(params![message.0, new.to_storage()])?;
            self.thread_changed(thread);
            changed = true;
        }
        Ok(changed)
    }

    /// [`MailBatch::backfill_message`] for the message at `uid` in
    /// `folder`. Returns whether anything changed.
    pub fn backfill_remote(
        &mut self,
        folder: FolderId,
        uid: u32,
        facts: &Backfill<'_>,
    ) -> Result<bool> {
        match self.message_at_uid(folder, uid)? {
            Some(id) => self.backfill_message(id, facts),
            None => Ok(false),
        }
    }

    /// Sets the category of the messages at the given UIDs of `folder`
    /// (Gmail's own categories for the inbox). Returns how many changed.
    pub fn set_categories(
        &mut self,
        folder: FolderId,
        categories: &[(u32, MailCategory)],
    ) -> Result<usize> {
        let mut changed = 0;
        for &(uid, category) in categories {
            let Some(id) = self.message_at_uid(folder, uid)? else {
                continue;
            };
            let tx = self.tx();
            let updated = tx
                .prepare_cached(
                    "UPDATE message SET category = ?2 WHERE id = ?1 AND category IS NOT ?2",
                )?
                .execute(params![id.0, category.to_storage()])?;
            if updated == 0 {
                continue;
            }
            changed += 1;
            let thread: Option<i64> = tx
                .prepare_cached("SELECT thread_id FROM message WHERE id = ?1")?
                .query_row([id.0], |row| row.get(0))?;
            match thread {
                Some(thread) => self.thread_changed(thread),
                None => journal::record(self.tx(), ObjectKind::Message, id.0, ChangeOp::Update)
                    .map(drop)?,
            }
        }
        Ok(changed)
    }

    pub(crate) fn message_at_uid(&self, folder: FolderId, uid: u32) -> Result<Option<MessageId>> {
        Ok(self
            .tx()
            .prepare_cached(
                "SELECT message_id FROM message_location WHERE folder_id = ?1 AND uid = ?2",
            )?
            .query_row(params![folder.0, uid], |row| row.get(0))
            .optional()?
            .map(MessageId))
    }
}
