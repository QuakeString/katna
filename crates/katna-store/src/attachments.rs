// SPDX-License-Identifier: GPL-3.0-or-later

//! Attachment lists for messages that were stored without one: mail
//! synced before lists were read from `BODYSTRUCTURE`, mail whose
//! structure could not be read, and imported mail. Lists come from the
//! downloaded body or from the structure fetched again, and are only
//! written where a message has none, so running a repair twice, or from
//! both places, is harmless.

use rusqlite::{Transaction, params};

use crate::Store;
use crate::blob::BlobHash;
use crate::error::{Error, Result};
use crate::journal::{self, ChangeOp, ObjectKind};
use crate::mail::{FolderId, MailBatch, MessageId};
use crate::remote::NewAttachment;

/// Inserts `attachments` for `message`; parts already stored are kept.
pub(crate) fn insert(
    tx: &Transaction<'_>,
    message: MessageId,
    attachments: &[NewAttachment<'_>],
) -> Result<()> {
    let mut insert = tx.prepare_cached(
        "INSERT OR IGNORE INTO attachment (message_id, part_id, filename, mime, size)
         VALUES (?1, ?2, ?3, ?4, ?5)",
    )?;
    for attachment in attachments {
        insert.execute(params![
            message.0,
            attachment.part,
            attachment.filename,
            attachment.mime,
            i64::try_from(attachment.size).unwrap_or(i64::MAX),
        ])?;
    }
    Ok(())
}

/// Messages said to have attachments but with none listed.
const UNLISTED: &str = "m.has_attachments = 1
    AND NOT EXISTS (SELECT 1 FROM attachment a WHERE a.message_id = m.id)";

impl Store {
    /// Messages with ID above `after` that have their raw message stored
    /// and attachments but no attachment list, in ID order, at most
    /// `limit`.
    pub fn unlisted_with_body(
        &self,
        after: MessageId,
        limit: u32,
    ) -> Result<Vec<(MessageId, BlobHash)>> {
        let mut stmt = self.mail.prepare_cached(&format!(
            "SELECT m.id, m.blob_hash FROM message m
             WHERE {UNLISTED} AND m.id > ?1 AND m.blob_hash IS NOT NULL
             ORDER BY m.id LIMIT ?2"
        ))?;
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

    /// UIDs in `folder`, ascending, of messages without a stored body that
    /// have attachments but no attachment list: their structure must come
    /// from the server again. At most `limit`.
    pub fn uids_needing_structure(&self, folder: FolderId, limit: u32) -> Result<Vec<u32>> {
        let mut stmt = self.mail.prepare_cached(&format!(
            "SELECT l.uid FROM message m
             JOIN message_location l ON l.message_id = m.id AND l.folder_id = ?1
             WHERE {UNLISTED} AND m.blob_hash IS NULL AND l.uid IS NOT NULL
             ORDER BY l.uid LIMIT ?2"
        ))?;
        let uids = stmt.query_map(params![folder.0, limit], |row| row.get(0))?;
        Ok(uids.collect::<rusqlite::Result<_>>()?)
    }

    /// Forgets listed attachments of a type in `body_types` that have no
    /// file name: versions of the message text (Gmail's AMP body) that
    /// earlier versions counted as files. A message left with none loses
    /// its paperclip. Returns how many messages changed.
    pub fn forget_body_parts(&mut self, body_types: &[&str]) -> Result<usize> {
        self.check_writable()?;
        let tx = self
            .mail
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        let mut changed = Vec::new();
        {
            let mut find = tx.prepare_cached(
                "SELECT id, message_id FROM attachment
                 WHERE filename IS NULL AND lower(mime) = ?1",
            )?;
            let mut delete = tx.prepare_cached("DELETE FROM attachment WHERE id = ?1")?;
            for mime in body_types {
                let rows: Vec<(i64, i64)> = find
                    .query_map([mime.to_ascii_lowercase()], |row| {
                        Ok((row.get(0)?, row.get(1)?))
                    })?
                    .collect::<rusqlite::Result<_>>()?;
                for (id, message) in rows {
                    delete.execute([id])?;
                    changed.push(message);
                }
            }
        }
        changed.sort_unstable();
        changed.dedup();
        for &message in &changed {
            tx.prepare_cached(
                "UPDATE message SET has_attachments = 0 WHERE id = ?1
                   AND NOT EXISTS (SELECT 1 FROM attachment WHERE message_id = ?1)",
            )?
            .execute([message])?;
            journal::record(&tx, ObjectKind::Message, message, ChangeOp::Update)?;
        }
        tx.commit()?;
        Ok(changed.len())
    }
}

impl MailBatch<'_> {
    /// Gives `message` the attachment list read from its body or its
    /// structure if it has none yet, and makes `has_attachments` agree
    /// with it. A list already stored stays as it is. Returns whether
    /// anything changed.
    pub fn list_attachments(
        &mut self,
        message: MessageId,
        attachments: &[NewAttachment<'_>],
    ) -> Result<bool> {
        let tx = self.tx();
        let listed: bool = tx
            .prepare_cached("SELECT EXISTS (SELECT 1 FROM attachment WHERE message_id = ?1)")?
            .query_row([message.0], |row| row.get(0))?;
        if listed {
            return Ok(false);
        }
        insert(tx, message, attachments)?;
        let flagged = tx
            .prepare_cached(
                "UPDATE message SET has_attachments = ?2
                 WHERE id = ?1 AND has_attachments IS NOT ?2",
            )?
            .execute(params![message.0, !attachments.is_empty()])?;
        let changed = !attachments.is_empty() || flagged > 0;
        if changed {
            journal::record(tx, ObjectKind::Message, message.0, ChangeOp::Update)?;
        }
        Ok(changed)
    }

    /// [`list_attachments`](Self::list_attachments) for the message at
    /// `uid` in `folder`; `false` when the UID is unknown.
    pub fn list_attachments_at(
        &mut self,
        folder: FolderId,
        uid: u32,
        attachments: &[NewAttachment<'_>],
    ) -> Result<bool> {
        match self.message_at_uid(folder, uid)? {
            Some(id) => self.list_attachments(id, attachments),
            None => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::remote::{FolderRole, RemoteMessage};
    use crate::{Added, MessageFlags, Mode};
    use katna_core::{AccountId, AccountKind, Paths};

    fn open() -> (tempfile::TempDir, Store, AccountId) {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Imap, "Work", "ada@example.org")
            .unwrap()
            .id;
        (tmp, store, account)
    }

    /// A message at `uid` stored as sync did before it read structures:
    /// the header's guess and no list.
    fn guessed<'a>(uid: u32, message_id: &'a str) -> RemoteMessage<'a> {
        RemoteMessage {
            uid,
            message_id_hdr: Some(message_id),
            subject: Some("Files"),
            date: Some(1_790_000_000),
            size: 1234,
            flags: MessageFlags::empty(),
            keywords: &[],
            has_attachments: true,
            list_id: None,
            participants: &[],
            in_reply_to: None,
            references: &[],
            gm_thread_id: None,
            gm_msgid: None,
            category: None,
            attachments: &[],
        }
    }

    fn flagged(store: &Store, inbox: FolderId, message: MessageId) -> bool {
        store
            .messages_in_folder(inbox)
            .unwrap()
            .into_iter()
            .find(|m| m.id == message)
            .unwrap()
            .has_attachments
    }

    const PDF: NewAttachment<'static> = NewAttachment {
        part: "2",
        mime: "application/pdf",
        filename: Some("rates.pdf"),
        size: 3000,
    };

    #[test]
    fn lists_old_mail_once() {
        let (_tmp, mut store, account) = open();
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let mut add = |uid, id| {
            let Added::Message(message) = batch
                .add_remote_message(account, inbox, &guessed(uid, id))
                .unwrap()
            else {
                panic!("expected a new message");
            };
            message
        };
        let (files, none, downloaded) = (add(1, "1@x"), add(2, "2@x"), add(3, "3@x"));
        batch
            .set_message_body(downloaded, b"Subject: x\r\n\r\nx", None, true)
            .unwrap();
        batch.commit().unwrap();

        assert_eq!(store.uids_needing_structure(inbox, 10).unwrap(), [1, 2]);
        let with_body: Vec<_> = store
            .unlisted_with_body(MessageId(0), 10)
            .unwrap()
            .into_iter()
            .map(|(id, _)| id)
            .collect();
        assert_eq!(with_body, [downloaded]);

        let mut batch = store.mail_batch().unwrap();
        assert!(batch.list_attachments_at(inbox, 1, &[PDF]).unwrap());
        // The structure says there are none: no paperclip.
        assert!(batch.list_attachments_at(inbox, 2, &[]).unwrap());
        assert!(batch.list_attachments(downloaded, &[PDF]).unwrap());
        // A list already stored stays.
        let other = NewAttachment { part: "3", ..PDF };
        assert!(!batch.list_attachments(files, &[other]).unwrap());
        assert!(!batch.list_attachments_at(inbox, 9, &[PDF]).unwrap());
        batch.commit().unwrap();

        let parts: Vec<_> = store
            .attachments(files)
            .unwrap()
            .into_iter()
            .map(|a| a.part)
            .collect();
        assert_eq!(parts, ["2"]);
        assert!(!flagged(&store, inbox, none));
        assert!(store.uids_needing_structure(inbox, 10).unwrap().is_empty());
        assert!(
            store
                .unlisted_with_body(MessageId(0), 10)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn forgets_versions_of_the_body() {
        let (_tmp, mut store, account) = open();
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let amp = NewAttachment {
            part: "1.3",
            mime: "text/x-amp-html",
            filename: None,
            size: 900,
        };
        let only_amp = [amp];
        let amp_and_pdf = [amp, PDF];
        let mut add = |uid, id, attachments| {
            let message = RemoteMessage {
                attachments,
                ..guessed(uid, id)
            };
            let Added::Message(message) =
                batch.add_remote_message(account, inbox, &message).unwrap()
            else {
                panic!("expected a new message");
            };
            message
        };
        let (newsletter, files) = (
            add(1, "1@x", &only_amp[..]),
            add(2, "2@x", &amp_and_pdf[..]),
        );
        batch.commit().unwrap();

        assert_eq!(
            store
                .forget_body_parts(&["text/html", "text/x-amp-html"])
                .unwrap(),
            2
        );
        assert!(!flagged(&store, inbox, newsletter));
        assert!(store.attachments(newsletter).unwrap().is_empty());
        assert!(flagged(&store, inbox, files));
        assert_eq!(store.attachments(files).unwrap().len(), 1);
        assert_eq!(store.forget_body_parts(&["text/x-amp-html"]).unwrap(), 0);
    }
}
