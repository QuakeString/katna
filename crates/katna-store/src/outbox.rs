// SPDX-License-Identifier: GPL-3.0-or-later

//! Mail waiting to be sent (`outbox`, `docs/ARCHITECTURE.md` §11).
//!
//! An outgoing message is a `message` row with no folder: it shows in no
//! mailbox and is not journaled, so clients and the search index skip it.
//! Once sent and filed (or given up), [`MailBatch::forget_outgoing`]
//! removes it; the copy in the Sent folder arrives with the next sync.

use katna_core::AccountId;
use rusqlite::{OptionalExtension, params};

use crate::Store;
use crate::error::Result;
use crate::mail::{MailBatch, MessageId, NewMessage};

/// Where an outgoing message is (`outbox.state`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendState {
    /// Waiting for `send_at` (the undo delay, or a retry).
    Queued,
    /// Handed to the SMTP server right now.
    Sending,
    Sent,
    /// Refused for good; the message stays for the user to fix.
    Failed,
    /// Undone before it went out.
    Cancelled,
}

impl SendState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Sending => "sending",
            Self::Sent => "sent",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    fn parse(text: &str) -> Self {
        match text {
            "sending" => Self::Sending,
            "sent" => Self::Sent,
            "failed" => Self::Failed,
            "cancelled" => Self::Cancelled,
            _ => Self::Queued,
        }
    }
}

/// One outbox row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutboxEntry {
    pub id: i64,
    pub account: AccountId,
    pub message: MessageId,
    pub subject: String,
    /// Unix seconds.
    pub send_at: i64,
    pub state: SendState,
    /// Refused tries so far.
    pub attempts: u32,
}

const ENTRY_QUERY: &str = "SELECT o.id, m.account_id, o.draft_message_id, m.subject,
                                  o.send_at, o.state, o.attempts
                           FROM outbox o JOIN message m ON m.id = o.draft_message_id";

fn entry(row: &rusqlite::Row<'_>) -> rusqlite::Result<OutboxEntry> {
    Ok(OutboxEntry {
        id: row.get(0)?,
        account: AccountId(row.get(1)?),
        message: MessageId(row.get(2)?),
        subject: row.get(3)?,
        send_at: row.get(4)?,
        state: SendState::parse(&row.get::<_, String>(5)?),
        attempts: row.get(6)?,
    })
}

impl Store {
    /// Every outbox entry, oldest first.
    pub fn outbox(&self) -> Result<Vec<OutboxEntry>> {
        let mut stmt = self
            .mail
            .prepare_cached(&format!("{ENTRY_QUERY} ORDER BY o.id"))?;
        let rows = stmt.query_map([], entry)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// One outbox entry.
    pub fn outbox_entry(&self, id: i64) -> Result<Option<OutboxEntry>> {
        Ok(self
            .mail
            .prepare_cached(&format!("{ENTRY_QUERY} WHERE o.id = ?1"))?
            .query_row([id], entry)
            .optional()?)
    }

    /// Queued entries due at `now`, earliest first, at most `limit`.
    pub fn due_sends(&self, now: i64, limit: u32) -> Result<Vec<OutboxEntry>> {
        let mut stmt = self.mail.prepare_cached(&format!(
            "{ENTRY_QUERY} WHERE o.state = 'queued' AND o.send_at <= ?1
             ORDER BY o.send_at, o.id LIMIT ?2"
        ))?;
        let rows = stmt.query_map(params![now, limit], entry)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// When the next queued message is due, if any.
    pub fn next_send_at(&self) -> Result<Option<i64>> {
        Ok(self
            .mail
            .prepare_cached("SELECT min(send_at) FROM outbox WHERE state = 'queued'")?
            .query_row([], |row| row.get(0))?)
    }
}

impl MailBatch<'_> {
    /// Stores an outgoing message of `account` outside every folder.
    pub fn add_outgoing(
        &mut self,
        account: AccountId,
        message: &NewMessage<'_>,
    ) -> Result<MessageId> {
        let hash = self.blobs().put(message.raw)?;
        let size = i64::try_from(message.raw.len()).unwrap_or(i64::MAX);
        let tx = self.tx();
        tx.prepare_cached(
            "INSERT INTO message (account_id, message_id_hdr, subject, date, size, flags,
                                  has_attachments, list_id, blob_hash, snippet)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        )?
        .execute(params![
            account.0,
            message.message_id_hdr,
            message.subject.unwrap_or_default(),
            message.date,
            size,
            message.flags.bits(),
            message.has_attachments,
            message.list_id,
            hash.as_bytes(),
            message.snippet,
        ])?;
        let id = tx.last_insert_rowid();
        let mut insert_participant = tx.prepare_cached(
            "INSERT INTO participant (message_id, role, email_norm, domain, display_name)
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;
        for participant in message.participants {
            insert_participant.execute(params![
                id,
                participant.role.as_str(),
                participant.email_norm,
                participant.domain,
                participant.display_name,
            ])?;
        }
        Ok(MessageId(id))
    }

    /// Queues `message` to be sent at `send_at`. Returns the entry's ID.
    pub fn queue_send(&mut self, message: MessageId, send_at: i64) -> Result<i64> {
        let tx = self.tx();
        tx.prepare_cached("INSERT INTO outbox (draft_message_id, send_at) VALUES (?1, ?2)")?
            .execute(params![message.0, send_at])?;
        Ok(tx.last_insert_rowid())
    }

    /// Sets an entry's state; `send_at` and `attempts` only when given.
    pub fn set_send_state(
        &mut self,
        id: i64,
        state: SendState,
        send_at: Option<i64>,
        attempts: Option<u32>,
    ) -> Result<()> {
        self.tx()
            .prepare_cached(
                "UPDATE outbox SET state = ?2, send_at = coalesce(?3, send_at),
                                   attempts = coalesce(?4, attempts)
                 WHERE id = ?1",
            )?
            .execute(params![id, state.as_str(), send_at, attempts])?;
        Ok(())
    }

    /// Cancels a queued entry. Returns `false` once it is being sent or
    /// done: SMTP cannot take mail back.
    pub fn cancel_send(&mut self, id: i64) -> Result<bool> {
        Ok(self
            .tx()
            .prepare_cached(
                "UPDATE outbox SET state = 'cancelled' WHERE id = ?1 AND state = 'queued'",
            )?
            .execute([id])?
            > 0)
    }

    /// Puts entries left `sending` by a crash back in the queue. The
    /// server may have taken them already; sending twice beats losing mail.
    pub fn requeue_sending(&mut self) -> Result<usize> {
        Ok(self
            .tx()
            .prepare_cached("UPDATE outbox SET state = 'queued' WHERE state = 'sending'")?
            .execute([])?)
    }

    /// Removes the outbox entries of `message`, and the message itself if
    /// no folder holds it.
    pub fn forget_outgoing(&mut self, message: MessageId) -> Result<()> {
        let tx = self.tx();
        tx.prepare_cached("DELETE FROM outbox WHERE draft_message_id = ?1")?
            .execute([message.0])?;
        let unfiled: bool = tx
            .prepare_cached(
                "SELECT NOT EXISTS (SELECT 1 FROM message_location WHERE message_id = ?1)",
            )?
            .query_row([message.0], |row| row.get(0))?;
        if unfiled {
            tx.prepare_cached("DELETE FROM participant WHERE message_id = ?1")?
                .execute([message.0])?;
            tx.prepare_cached("DELETE FROM message WHERE id = ?1")?
                .execute([message.0])?;
        }
        Ok(())
    }

    /// Forgets every outgoing message of `account`, for example when it
    /// is removed.
    pub fn clear_outbox(&mut self, account: AccountId) -> Result<()> {
        let ids: Vec<i64> = {
            let tx = self.tx();
            let mut stmt = tx.prepare_cached(
                "SELECT o.draft_message_id FROM outbox o
                 JOIN message m ON m.id = o.draft_message_id WHERE m.account_id = ?1",
            )?;
            let rows = stmt.query_map([account.0], |row| row.get(0))?;
            rows.collect::<rusqlite::Result<_>>()?
        };
        for id in ids {
            self.forget_outgoing(MessageId(id))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MessageFlags, Mode};
    use katna_core::{AccountKind, Paths};

    fn outgoing<'a>(raw: &'a [u8], subject: &'a str) -> NewMessage<'a> {
        NewMessage {
            raw,
            message_id_hdr: None,
            subject: Some(subject),
            date: None,
            flags: MessageFlags::SEEN,
            has_attachments: false,
            list_id: None,
            snippet: None,
            participants: &[],
            in_reply_to: None,
            references: &[],
            category: None,
        }
    }

    #[test]
    fn queue_cancel_and_forget() {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Imap, "Work", "ada@example.org")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let first = batch
            .add_outgoing(account, &outgoing(b"Subject: one\r\n\r\nHi.\r\n", "one"))
            .unwrap();
        let second = batch
            .add_outgoing(account, &outgoing(b"Subject: two\r\n\r\nHi.\r\n", "two"))
            .unwrap();
        let a = batch.queue_send(first, 100).unwrap();
        let b = batch.queue_send(second, 50).unwrap();
        batch.commit().unwrap();

        assert_eq!(store.next_send_at().unwrap(), Some(50));
        assert!(store.due_sends(10, 10).unwrap().is_empty());
        let due = store.due_sends(100, 10).unwrap();
        assert_eq!(due.iter().map(|e| e.id).collect::<Vec<_>>(), [b, a]);
        assert_eq!(due[1].subject, "one");
        assert_eq!(due[1].state, SendState::Queued);
        // Outgoing mail is in no folder.
        assert!(
            store.messages_by_id(&[first]).unwrap()[0]
                .locations
                .is_empty()
        );

        let mut batch = store.mail_batch().unwrap();
        batch
            .set_send_state(b, SendState::Sending, None, None)
            .unwrap();
        assert!(!batch.cancel_send(b).unwrap(), "too late");
        assert!(batch.cancel_send(a).unwrap());
        assert_eq!(batch.requeue_sending().unwrap(), 1);
        batch
            .set_send_state(b, SendState::Queued, Some(300), Some(1))
            .unwrap();
        batch.commit().unwrap();
        assert_eq!(
            store.outbox_entry(a).unwrap().unwrap().state,
            SendState::Cancelled
        );
        let retried = store.outbox_entry(b).unwrap().unwrap();
        assert_eq!((retried.send_at, retried.attempts), (300, 1));
        assert_eq!(store.next_send_at().unwrap(), Some(300));

        let mut batch = store.mail_batch().unwrap();
        batch.forget_outgoing(second).unwrap();
        batch.clear_outbox(account).unwrap();
        batch.commit().unwrap();
        assert!(store.outbox().unwrap().is_empty());
        assert!(store.messages_by_id(&[first, second]).unwrap().is_empty());
    }
}
