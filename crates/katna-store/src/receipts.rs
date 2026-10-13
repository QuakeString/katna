// SPDX-License-Identifier: GPL-3.0-or-later

//! Delivery and read receipts per recipient of sent mail, recorded as the
//! receipts arrive (`docs/ARCHITECTURE.md` §16.1).

use rusqlite::{OptionalExtension, params};

use crate::Store;
use crate::blob::BlobHash;
use crate::error::{Error, Result};
use crate::mail::{MailBatch, MessageId};

/// What a receipt says happened to a sent message for one recipient.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiptKind {
    /// Their mail server took it.
    Delivered,
    /// It could not be delivered (a bounce).
    Failed,
    /// Their app showed it.
    Read,
}

/// What became of a sent message for one recipient. Times are Unix
/// seconds.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Receipt {
    /// Lower case.
    pub recipient: String,
    /// When the user's mail server took it, when sent from here.
    pub sent_at: Option<i64>,
    /// When their mail server took it.
    pub delivered_at: Option<i64>,
    /// When it bounced.
    pub failed_at: Option<i64>,
    /// When their app showed it.
    pub read_at: Option<i64>,
}

impl MailBatch<'_> {
    /// Records what a receipt said about the message with `Message-ID`
    /// `original` (without angle brackets) for `recipient` at `at`. The
    /// earliest time is kept.
    pub fn record_receipt(
        &mut self,
        original: &str,
        recipient: &str,
        kind: ReceiptKind,
        at: i64,
    ) -> Result<()> {
        let column = match kind {
            ReceiptKind::Delivered => "delivered_at",
            ReceiptKind::Failed => "failed_at",
            ReceiptKind::Read => "read_at",
        };
        self.upsert(column, original, recipient, at)
    }

    /// Records that the message with `Message-ID` `original` went out to
    /// `recipients` at `at`.
    pub fn receipt_sent(&mut self, original: &str, recipients: &[String], at: i64) -> Result<()> {
        for recipient in recipients {
            self.upsert("sent_at", original, recipient, at)?;
        }
        Ok(())
    }

    /// Records that `message` is a receipt for mail here, which the lists
    /// leave out (mail.db v14). Returns whether it was new.
    pub fn mark_receipt_mail(&mut self, message: MessageId) -> Result<bool> {
        let tx = self.tx();
        let added = tx
            .prepare_cached("INSERT OR IGNORE INTO receipt_mail (message_id) VALUES (?1)")?
            .execute([message.0])?
            > 0;
        let thread: Option<i64> = tx
            .prepare_cached("SELECT thread_id FROM message WHERE id = ?1")?
            .query_row([message.0], |row| row.get(0))
            .optional()?
            .flatten();
        if added && let Some(thread) = thread {
            self.thread_changed(thread);
        }
        Ok(added)
    }

    fn upsert(&mut self, column: &str, original: &str, recipient: &str, at: i64) -> Result<()> {
        // `column` is one of ours, never outside input.
        let sql = format!(
            "INSERT INTO receipt (message_id_hdr, recipient, {column}) VALUES (?1, ?2, ?3)
             ON CONFLICT DO UPDATE SET {column} = min(coalesce({column}, ?3), ?3)"
        );
        self.tx()
            .prepare_cached(&sql)?
            .execute(params![original, recipient.to_lowercase(), at])?;
        Ok(())
    }
}

impl Store {
    /// What receipts said about the message with `Message-ID` `original`,
    /// per recipient.
    pub fn receipts(&self, original: &str) -> Result<Vec<Receipt>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT recipient, sent_at, delivered_at, failed_at, read_at FROM receipt
             WHERE message_id_hdr = ?1 ORDER BY recipient",
        )?;
        let rows = stmt.query_map([original], |row| {
            Ok(Receipt {
                recipient: row.get(0)?,
                sent_at: row.get(1)?,
                delivered_at: row.get(2)?,
                failed_at: row.get(3)?,
                read_at: row.get(4)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Downloaded messages that may be receipts stored before mail.db v14
    /// marked them: in a conversation with sent mail that a receipt has
    /// answered, not themselves that sent mail, and not marked yet.
    pub fn possible_receipt_mail(&self) -> Result<Vec<(MessageId, BlobHash)>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT m.id, m.blob_hash FROM message m
             WHERE m.blob_hash IS NOT NULL
               AND m.thread_id IN (
                   SELECT o.thread_id FROM message o JOIN receipt r
                       ON r.message_id_hdr = o.message_id_hdr
                   WHERE r.read_at IS NOT NULL OR r.delivered_at IS NOT NULL
               )
               AND (m.message_id_hdr IS NULL
                    OR m.message_id_hdr NOT IN (SELECT message_id_hdr FROM receipt))
               AND m.id NOT IN (SELECT message_id FROM receipt_mail)
             ORDER BY m.id",
        )?;
        let rows = stmt.query_map([], |row| {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Mode;
    use katna_core::Paths;

    #[test]
    fn keeps_the_first_time_of_each() {
        use ReceiptKind::*;
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(dir.path()), Mode::ReadWrite).unwrap();
        let mut batch = store.mail_batch().unwrap();
        batch
            .receipt_sent("m1@x", &["bea@x.org".into(), "carl@x.org".into()], 1)
            .unwrap();
        for (who, kind, at) in [
            ("Bea@x.org", Delivered, 20),
            ("bea@x.org", Delivered, 10),
            ("bea@x.org", Read, 30),
            ("bea@x.org", Read, 40),
            ("carl@x.org", Failed, 50),
        ] {
            batch.record_receipt("m1@x", who, kind, at).unwrap();
        }
        batch
            .record_receipt("m2@x", "dan@x.org", Delivered, 5)
            .unwrap();
        batch.commit().unwrap();
        assert_eq!(
            store.receipts("m1@x").unwrap(),
            [
                Receipt {
                    recipient: "bea@x.org".into(),
                    sent_at: Some(1),
                    delivered_at: Some(10),
                    failed_at: None,
                    read_at: Some(30),
                },
                Receipt {
                    recipient: "carl@x.org".into(),
                    sent_at: Some(1),
                    failed_at: Some(50),
                    ..Receipt::default()
                },
            ]
        );
        assert!(store.receipts("m3@x").unwrap().is_empty());
    }
}
