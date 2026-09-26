// SPDX-License-Identifier: GPL-3.0-or-later

//! What POP3 accounts have downloaded, by UIDL (plan task 1.10,
//! `docs/ARCHITECTURE.md` §6.4). POP3 mail is stored like imported mail:
//! whole messages in local folders.

use katna_core::AccountId;
use rusqlite::params;

use crate::Store;
use crate::error::Result;
use crate::mail::{Added, FolderId, MailBatch, MessageId, NewMessage};

/// A server message a POP3 account downloaded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pop3Uidl {
    pub uidl: String,
    /// The local message, or `None` once it was deleted in Katna.
    pub message: Option<MessageId>,
    /// When it was downloaded, in Unix seconds.
    pub first_seen: i64,
}

impl Store {
    /// Every UIDL `account` downloaded and has not forgotten.
    pub fn pop3_uidls(&self, account: AccountId) -> Result<Vec<Pop3Uidl>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT uidl, message_id, first_seen FROM pop3_uidl
             WHERE account_id = ?1 ORDER BY first_seen, uidl",
        )?;
        let rows = stmt.query_map([account.0], |row| {
            Ok(Pop3Uidl {
                uidl: row.get(0)?,
                message: row.get::<_, Option<i64>>(1)?.map(MessageId),
                first_seen: row.get(2)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }
}

impl MailBatch<'_> {
    /// Stores a message downloaded from a POP3 server as `uidl` at `now`.
    pub fn add_pop3_message(
        &mut self,
        account: AccountId,
        folder: FolderId,
        uidl: &str,
        message: &NewMessage<'_>,
        now: i64,
    ) -> Result<Added> {
        let added = self.add_message(account, folder, message)?;
        let (Added::Message(id) | Added::Location(id) | Added::Duplicate(id)) = added;
        self.tx()
            .prepare_cached(
                "INSERT INTO pop3_uidl (account_id, uidl, message_id, first_seen)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT (account_id, uidl) DO UPDATE SET message_id = excluded.message_id",
            )?
            .execute(params![account.0, uidl, id.0, now])?;
        Ok(added)
    }

    /// Forgets `uidls` of `account`: they were deleted from the server or
    /// are gone from it. Local messages stay. Returns how many were known.
    pub fn forget_pop3_uidls(&mut self, account: AccountId, uidls: &[&str]) -> Result<usize> {
        let mut stmt = self
            .tx()
            .prepare_cached("DELETE FROM pop3_uidl WHERE account_id = ?1 AND uidl = ?2")?;
        let mut forgotten = 0;
        for uidl in uidls {
            forgotten += stmt.execute(params![account.0, uidl])?;
        }
        Ok(forgotten)
    }

    /// Forgets every UIDL of `account`, for example when it is removed.
    pub fn clear_pop3(&mut self, account: AccountId) -> Result<()> {
        self.tx()
            .prepare_cached("DELETE FROM pop3_uidl WHERE account_id = ?1")?
            .execute([account.0])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MessageFlags, Mode};
    use katna_core::{AccountKind, Paths};

    fn message(raw: &[u8]) -> NewMessage<'_> {
        NewMessage {
            raw,
            message_id_hdr: None,
            subject: Some("Hello"),
            date: Some(1_700_000_000),
            flags: MessageFlags::empty(),
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
    fn remembers_uidls_after_local_deletes() {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Pop3, "pop", "pop@example.org")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch.ensure_folder(account, "INBOX").unwrap();
        let Added::Message(first) = batch
            .add_pop3_message(
                account,
                inbox,
                "u1",
                &message(b"Subject: 1\r\n\r\n1\r\n"),
                10,
            )
            .unwrap()
        else {
            panic!("new message expected");
        };
        batch
            .add_pop3_message(
                account,
                inbox,
                "u2",
                &message(b"Subject: 2\r\n\r\n2\r\n"),
                20,
            )
            .unwrap();
        batch.commit().unwrap();

        let uidls = store.pop3_uidls(account).unwrap();
        assert_eq!(uidls.len(), 2);
        assert_eq!(
            uidls[0],
            Pop3Uidl {
                uidl: "u1".into(),
                message: Some(first),
                first_seen: 10
            }
        );

        let mut batch = store.mail_batch().unwrap();
        batch.remove_from_folder(first, inbox).unwrap();
        batch.commit().unwrap();
        let uidls = store.pop3_uidls(account).unwrap();
        assert_eq!((uidls[0].uidl.as_str(), uidls[0].message), ("u1", None));

        let mut batch = store.mail_batch().unwrap();
        assert_eq!(
            batch.forget_pop3_uidls(account, &["u1", "gone"]).unwrap(),
            1
        );
        batch.commit().unwrap();
        assert_eq!(store.pop3_uidls(account).unwrap().len(), 1);

        let mut batch = store.mail_batch().unwrap();
        batch.clear_pop3(account).unwrap();
        batch.commit().unwrap();
        assert!(store.pop3_uidls(account).unwrap().is_empty());
    }
}
