// SPDX-License-Identifier: GPL-3.0-or-later

//! Forgetting downloaded mail so sync downloads it again, for Settings ›
//! Reset cache.

use katna_core::AccountId;
use rusqlite::params;

use crate::Store;
use crate::blob::BlobHash;
use crate::error::Result;

/// What [`Store::forget_downloaded_mail`] deleted.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Forgotten {
    /// Messages whose body is gone.
    pub messages: usize,
    /// Bytes of stored mail deleted, uncompressed.
    pub bytes: u64,
}

impl Store {
    /// Forgets the downloaded bodies of the mail in `accounts` that is still
    /// on the server (a folder has it at a UID), so sync or opening it
    /// downloads it again, and deletes the stored bytes nothing uses any
    /// more.
    ///
    /// Everything else stays: folders, flags, labels, threads, snippets,
    /// attachment lists and pins. So does the body of mail that exists only
    /// here: drafts and mail in the outbox or named by a change not yet
    /// made on the server. Give only accounts whose server keeps the mail
    /// (IMAP), never POP3 or imported mail.
    pub fn forget_downloaded_mail(&mut self, accounts: &[AccountId]) -> Result<Forgotten> {
        self.check_writable()?;
        let tx = self
            .mail
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        let mut messages = 0;
        {
            let mut forget = tx.prepare_cached(
                "UPDATE message SET blob_hash = NULL, body_state = 0
                 WHERE account_id = ?1 AND blob_hash IS NOT NULL
                   AND EXISTS (SELECT 1 FROM message_location l
                               WHERE l.message_id = message.id AND l.uid IS NOT NULL)
                   AND NOT EXISTS (SELECT 1 FROM outbox WHERE draft_message_id = message.id)
                   AND NOT EXISTS (SELECT 1 FROM op_queue
                                   WHERE state IN ('pending', 'running')
                                     AND json_extract(op_json, '$.message') = message.id)",
            )?;
            for account in accounts {
                messages += forget.execute(params![account.0])?;
            }
        }
        let keep: Vec<BlobHash> = {
            let mut stmt = tx.prepare(
                "SELECT blob_hash FROM message WHERE blob_hash IS NOT NULL
                 UNION SELECT blob_hash FROM attachment WHERE blob_hash IS NOT NULL",
            )?;
            let rows = stmt.query_map([], |row| row.get::<_, Vec<u8>>(0))?;
            rows.filter_map(|hash| match hash {
                Ok(hash) => <[u8; 32]>::try_from(hash.as_slice())
                    .ok()
                    .map(|bytes| Ok(BlobHash::from_bytes(bytes))),
                Err(err) => Some(Err(err)),
            })
            .collect::<rusqlite::Result<_>>()?
        };
        // Rows first: a crash in between leaves unused blobs, never a
        // message pointing at a deleted one. The blob lock is taken before
        // the rows are committed, so no batch stores a body in between.
        self.blobs.begin()?;
        if let Err(err) = tx.commit() {
            let _ = self.blobs.end(false);
            return Err(err.into());
        }
        let (_, bytes) = self.blobs.retain(&keep)?;
        self.blobs.incremental_vacuum(None)?;
        Ok(Forgotten { messages, bytes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Added, FolderId, MessageFlags, MessageId, Mode, NewMessage, RemoteMessage};
    use katna_core::{AccountKind, Paths};

    fn remote(store: &mut Store, account: AccountId, folder: FolderId, uid: u32) -> MessageId {
        let mut batch = store.mail_batch().unwrap();
        let added = batch
            .add_remote_message(
                account,
                folder,
                &RemoteMessage {
                    uid,
                    message_id_hdr: None,
                    subject: Some("Hello"),
                    date: Some(1_790_000_000),
                    size: 1234,
                    flags: MessageFlags::SEEN,
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
                },
            )
            .unwrap();
        let Added::Message(id) = added else {
            panic!("stored as {added:?}");
        };
        let raw = format!("Subject: Hello\r\n\r\nBody {uid}");
        batch
            .set_message_body(id, raw.as_bytes(), Some("Body"), false)
            .unwrap();
        batch.commit().unwrap();
        id
    }

    fn stored(store: &Store, id: MessageId) -> crate::StoredMessage {
        store.messages_by_id(&[id]).unwrap().remove(0)
    }

    fn body(store: &Store, id: MessageId) -> Option<Vec<u8>> {
        let hash = stored(store, id).blob_hash?;
        store.blobs().get(&hash).unwrap()
    }

    #[test]
    fn forgets_bodies_the_server_has_and_keeps_local_ones() {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let imap = store
            .add_account(AccountKind::Imap, "Work", "ada@example.org")
            .unwrap()
            .id;
        let pop = store
            .add_account(AccountKind::Pop3, "Home", "ada@example.net")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch.upsert_folder(imap, "INBOX", None).unwrap();
        let pop_inbox = batch.upsert_folder(pop, "INBOX", None).unwrap();
        batch.commit().unwrap();

        let plain = remote(&mut store, imap, inbox, 1);
        let queued = remote(&mut store, imap, inbox, 2);
        let popped = remote(&mut store, pop, pop_inbox, 3);
        let mut batch = store.mail_batch().unwrap();
        batch
            .enqueue_op(
                imap,
                &format!(r#"{{"kind":"move","message":{}}}"#, queued.0),
            )
            .unwrap();
        let draft = batch
            .add_outgoing(
                imap,
                &NewMessage {
                    raw: b"Subject: Draft\r\n\r\nNot sent yet",
                    message_id_hdr: None,
                    subject: Some("Draft"),
                    date: Some(1_790_000_000),
                    flags: MessageFlags::SEEN,
                    has_attachments: false,
                    list_id: None,
                    snippet: None,
                    participants: &[],
                    in_reply_to: None,
                    references: &[],
                    category: None,
                },
            )
            .unwrap();
        batch.set_pinned(plain, Some(1_790_000_100)).unwrap();
        batch.commit().unwrap();

        let forgotten = store.forget_downloaded_mail(&[imap]).unwrap();
        assert_eq!(forgotten.messages, 1);
        assert_eq!(
            forgotten.bytes,
            b"Subject: Hello\r\n\r\nBody 1".len() as u64
        );

        let message = stored(&store, plain);
        assert_eq!(message.blob_hash, None);
        assert_eq!(message.snippet.as_deref(), Some("Body"));
        assert_eq!(store.pinned().unwrap()[0].message, plain);
        assert!(body(&store, queued).is_some());
        assert!(body(&store, popped).is_some());
        assert!(body(&store, draft).is_some());
        let without: Vec<MessageId> = store
            .messages_without_body(inbox, None, u64::MAX, 10)
            .unwrap()
            .into_iter()
            .map(|(id, _)| id)
            .collect();
        assert_eq!(without, [plain]);

        // Downloading it again works as before.
        let mut batch = store.mail_batch().unwrap();
        batch
            .set_message_body(plain, b"Subject: Hello\r\n\r\nBody 1", Some("Body"), false)
            .unwrap();
        batch.commit().unwrap();
        assert!(body(&store, plain).is_some());
        assert_eq!(
            store.forget_downloaded_mail(&[]).unwrap(),
            Forgotten::default()
        );
    }
}
