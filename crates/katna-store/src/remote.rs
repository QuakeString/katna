// SPDX-License-Identifier: GPL-3.0-or-later

//! Mail that lives on a server: folders with their sync state, and messages
//! known by folder and UID before any body is downloaded (sync level 1,
//! `docs/ARCHITECTURE.md` §6.2).
//!
//! Each server copy of a message is its own `message` row with one
//! `message_location`, because IMAP flags belong to the copy. Merging copies
//! (Gmail labels) comes with threading.

use std::fmt;
use std::str::FromStr;

use katna_core::AccountId;
use rusqlite::{OptionalExtension, Transaction, params};

use crate::error::{Error, Result};
use crate::journal::{self, ChangeOp, ObjectKind};
use crate::mail::{Added, FolderId, MailBatch, MessageFlags, MessageId, NewParticipant};
use crate::{Store, StoredMessage};

/// `folder.role`: what a folder is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FolderRole {
    Inbox,
    Sent,
    Drafts,
    Trash,
    Junk,
    Archive,
    /// Every message of the account (Gmail's All Mail).
    All,
    /// A virtual folder of starred messages.
    Flagged,
}

impl FolderRole {
    const ALL: [Self; 8] = [
        Self::Inbox,
        Self::Sent,
        Self::Drafts,
        Self::Trash,
        Self::Junk,
        Self::Archive,
        Self::All,
        Self::Flagged,
    ];

    /// Stable name stored in `folder.role`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Inbox => "inbox",
            Self::Sent => "sent",
            Self::Drafts => "drafts",
            Self::Trash => "trash",
            Self::Junk => "junk",
            Self::Archive => "archive",
            Self::All => "all",
            Self::Flagged => "flagged",
        }
    }
}

impl fmt::Display for FolderRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for FolderRole {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::ALL
            .into_iter()
            .find(|role| role.as_str() == s)
            .ok_or_else(|| Error::InvalidData(format!("unknown folder role {s:?}")))
    }
}

/// A folder as the store knows it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredFolder {
    pub id: FolderId,
    pub path: String,
    pub role: Option<FolderRole>,
    pub uidvalidity: Option<u32>,
    pub highestmodseq: Option<u64>,
    /// Free-form JSON owned by `katna-sync`.
    pub sync_state: Option<String>,
}

/// A message known from the server's headers (no body yet).
#[derive(Debug, Clone, Copy)]
pub struct RemoteMessage<'a> {
    pub uid: u32,
    /// `Message-ID` without angle brackets.
    pub message_id_hdr: Option<&'a str>,
    pub subject: Option<&'a str>,
    /// Unix seconds.
    pub date: Option<i64>,
    /// Size of the whole message on the server, in bytes.
    pub size: u64,
    pub flags: MessageFlags,
    /// IMAP keywords other than the ones in [`MessageFlags`].
    pub keywords: &'a [String],
    pub has_attachments: bool,
    pub list_id: Option<&'a str>,
    pub participants: &'a [NewParticipant<'a>],
}

impl Store {
    /// The folders of `account`, ordered by path.
    pub fn folders(&self, account: AccountId) -> Result<Vec<StoredFolder>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT id, path, role, uidvalidity, highestmodseq, sync_state
             FROM folder WHERE account_id = ?1 ORDER BY path",
        )?;
        let rows = stmt.query_map([account.0], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, Option<u32>>(3)?,
                row.get::<_, Option<i64>>(4)?,
                row.get::<_, Option<String>>(5)?,
            ))
        })?;
        rows.map(|row| {
            let (id, path, role, uidvalidity, highestmodseq, sync_state) = row?;
            Ok(StoredFolder {
                id: FolderId(id),
                path,
                role: role.map(|r| r.parse()).transpose()?,
                uidvalidity,
                highestmodseq: highestmodseq.map(|m| m as u64),
                sync_state,
            })
        })
        .collect()
    }

    /// The messages in `folder`, in UID order.
    pub fn messages_in_folder(&self, folder: FolderId) -> Result<Vec<StoredMessage>> {
        let ids: Vec<MessageId> = self
            .mail
            .prepare_cached(
                "SELECT message_id FROM message_location WHERE folder_id = ?1 ORDER BY uid",
            )?
            .query_map([folder.0], |row| row.get(0).map(MessageId))?
            .collect::<rusqlite::Result<_>>()?;
        crate::mail_read::messages_by_id(&self.mail, &ids)
    }

    /// Messages in `folder` whose body is not stored yet, newest first, as
    /// `(message, uid)`. Only messages dated at or after `since` (when
    /// given) and at most `max_size` bytes on the server; at most `limit`.
    pub fn messages_without_body(
        &self,
        folder: FolderId,
        since: Option<i64>,
        max_size: u64,
        limit: u32,
    ) -> Result<Vec<(MessageId, u32)>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT m.id, l.uid FROM message m
             JOIN message_location l ON l.message_id = m.id
             WHERE l.folder_id = ?1 AND l.uid IS NOT NULL AND m.blob_hash IS NULL
               AND m.size <= ?3 AND (?2 IS NULL OR m.date >= ?2)
             ORDER BY m.date DESC, l.uid DESC LIMIT ?4",
        )?;
        let rows = stmt.query_map(
            params![
                folder.0,
                since,
                i64::try_from(max_size).unwrap_or(i64::MAX),
                limit
            ],
            |row| Ok((MessageId(row.get(0)?), row.get(1)?)),
        )?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Where a message is on the server: its account, a folder and the UID
    /// there. `None` if it is unknown or only stored locally.
    pub fn remote_location(
        &self,
        message: MessageId,
    ) -> Result<Option<(AccountId, StoredFolder, u32)>> {
        let found: Option<(i64, i64, u32)> = self
            .mail
            .prepare_cached(
                "SELECT m.account_id, l.folder_id, l.uid FROM message m
                 JOIN message_location l ON l.message_id = m.id
                 WHERE m.id = ?1 AND l.uid IS NOT NULL
                 ORDER BY l.folder_id LIMIT 1",
            )?
            .query_row([message.0], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .optional()?;
        let Some((account, folder, uid)) = found else {
            return Ok(None);
        };
        let account = AccountId(account);
        Ok(self
            .folders(account)?
            .into_iter()
            .find(|f| f.id == FolderId(folder))
            .map(|folder| (account, folder, uid)))
    }

    /// The UIDs stored for `folder`, ascending.
    pub fn folder_uids(&self, folder: FolderId) -> Result<Vec<u32>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT uid FROM message_location
             WHERE folder_id = ?1 AND uid IS NOT NULL ORDER BY uid",
        )?;
        let uids = stmt.query_map([folder.0], |row| row.get(0))?;
        Ok(uids.collect::<rusqlite::Result<_>>()?)
    }
}

impl MailBatch<'_> {
    /// Returns the folder `path` of `account`, creating it or updating its
    /// role as needed.
    pub fn upsert_folder(
        &mut self,
        account: AccountId,
        path: &str,
        role: Option<FolderRole>,
    ) -> Result<FolderId> {
        let tx = self.tx();
        let role = role.map(FolderRole::as_str);
        let existing: Option<(i64, Option<String>)> = tx
            .prepare_cached("SELECT id, role FROM folder WHERE account_id = ?1 AND path = ?2")?
            .query_row(params![account.0, path], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .optional()?;
        match existing {
            Some((id, old)) if old.as_deref() == role => Ok(FolderId(id)),
            Some((id, _)) => {
                tx.prepare_cached("UPDATE folder SET role = ?2 WHERE id = ?1")?
                    .execute(params![id, role])?;
                journal::record(tx, ObjectKind::Folder, id, ChangeOp::Update)?;
                Ok(FolderId(id))
            }
            None => {
                tx.prepare_cached(
                    "INSERT INTO folder (account_id, path, role) VALUES (?1, ?2, ?3)",
                )?
                .execute(params![account.0, path, role])?;
                let id = tx.last_insert_rowid();
                journal::record(tx, ObjectKind::Folder, id, ChangeOp::Insert)?;
                Ok(FolderId(id))
            }
        }
    }

    /// Removes a folder and the messages that were only in it.
    pub fn remove_folder(&mut self, folder: FolderId) -> Result<()> {
        self.clear_folder(folder)?;
        let tx = self.tx();
        if tx.execute("DELETE FROM folder WHERE id = ?1", [folder.0])? > 0 {
            journal::record(tx, ObjectKind::Folder, folder.0, ChangeOp::Delete)?;
        }
        Ok(())
    }

    /// Saves what the sync engine needs to continue where it stopped.
    pub fn set_folder_state(
        &mut self,
        folder: FolderId,
        uidvalidity: Option<u32>,
        highestmodseq: Option<u64>,
        sync_state: Option<&str>,
    ) -> Result<()> {
        self.tx()
            .prepare_cached(
                "UPDATE folder SET uidvalidity = ?2, highestmodseq = ?3, sync_state = ?4
                 WHERE id = ?1",
            )?
            .execute(params![
                folder.0,
                uidvalidity,
                highestmodseq.map(|m| m as i64),
                sync_state
            ])?;
        Ok(())
    }

    /// Stores a message seen at `message.uid` in `folder`. Returns
    /// [`Added::Duplicate`] if that UID is already stored.
    pub fn add_remote_message(
        &mut self,
        account: AccountId,
        folder: FolderId,
        message: &RemoteMessage<'_>,
    ) -> Result<Added> {
        let tx = self.tx();
        if let Some(id) = message_at(tx, folder, message.uid)? {
            return Ok(Added::Duplicate(id));
        }
        tx.prepare_cached(
            "INSERT INTO message (account_id, message_id_hdr, subject, date, size, flags,
                                  keywords, has_attachments, list_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        )?
        .execute(params![
            account.0,
            message.message_id_hdr,
            message.subject.unwrap_or_default(),
            message.date,
            i64::try_from(message.size).unwrap_or(i64::MAX),
            message.flags.bits(),
            keywords_json(message.keywords),
            message.has_attachments,
            message.list_id,
        ])?;
        let id = tx.last_insert_rowid();
        tx.prepare_cached(
            "INSERT INTO message_location (message_id, folder_id, uid) VALUES (?1, ?2, ?3)",
        )?
        .execute(params![id, folder.0, message.uid])?;
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
        journal::record(tx, ObjectKind::Message, id, ChangeOp::Insert)?;
        Ok(Added::Message(MessageId(id)))
    }

    /// Stores the full raw message of `message` (blob first, then the row)
    /// and marks its body as downloaded.
    pub fn set_message_body(
        &mut self,
        message: MessageId,
        raw: &[u8],
        snippet: Option<&str>,
        has_attachments: bool,
    ) -> Result<()> {
        let hash = self.blobs().put(raw)?;
        let tx = self.tx();
        let updated = tx
            .prepare_cached(
                "UPDATE message SET blob_hash = ?2, snippet = ?3, has_attachments = ?4,
                                    body_state = 2
                 WHERE id = ?1",
            )?
            .execute(params![
                message.0,
                hash.as_bytes(),
                snippet,
                has_attachments
            ])?;
        if updated > 0 {
            journal::record(tx, ObjectKind::Message, message.0, ChangeOp::Update)?;
        }
        Ok(())
    }

    /// Updates the flags of the message at `uid` in `folder`. Returns the
    /// message if the flags changed, `None` if they did not or the UID is
    /// unknown.
    pub fn set_remote_flags(
        &mut self,
        folder: FolderId,
        uid: u32,
        flags: MessageFlags,
        keywords: &[String],
    ) -> Result<Option<MessageId>> {
        let tx = self.tx();
        let Some(id) = message_at(tx, folder, uid)? else {
            return Ok(None);
        };
        let changed = tx
            .prepare_cached(
                "UPDATE message SET flags = ?2, keywords = ?3
                 WHERE id = ?1 AND (flags != ?2 OR keywords IS NOT ?3)",
            )?
            .execute(params![id.0, flags.bits(), keywords_json(keywords)])?;
        if changed == 0 {
            return Ok(None);
        }
        journal::record(tx, ObjectKind::Message, id.0, ChangeOp::Update)?;
        Ok(Some(id))
    }

    /// Forgets the messages at `uids` in `folder` (expunged on the server).
    /// Returns how many were stored.
    pub fn remove_remote_messages(&mut self, folder: FolderId, uids: &[u32]) -> Result<usize> {
        let tx = self.tx();
        let mut removed = 0;
        for &uid in uids {
            if let Some(id) = message_at(tx, folder, uid)? {
                remove_location(tx, id, folder)?;
                removed += 1;
            }
        }
        Ok(removed)
    }

    /// Forgets every message in `folder`, for example after its UIDVALIDITY
    /// changed. Returns how many there were.
    pub fn clear_folder(&mut self, folder: FolderId) -> Result<usize> {
        let tx = self.tx();
        let ids: Vec<i64> = tx
            .prepare_cached("SELECT message_id FROM message_location WHERE folder_id = ?1")?
            .query_map([folder.0], |row| row.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        for &id in &ids {
            remove_location(tx, MessageId(id), folder)?;
        }
        Ok(ids.len())
    }
}

fn message_at(tx: &Transaction<'_>, folder: FolderId, uid: u32) -> Result<Option<MessageId>> {
    Ok(tx
        .prepare_cached(
            "SELECT message_id FROM message_location WHERE folder_id = ?1 AND uid = ?2",
        )?
        .query_row(params![folder.0, uid], |row| row.get(0))
        .optional()?
        .map(MessageId))
}

/// Removes one location, and the message too if that was its last one.
pub(crate) fn remove_location(tx: &Transaction<'_>, id: MessageId, folder: FolderId) -> Result<()> {
    tx.prepare_cached("DELETE FROM message_location WHERE message_id = ?1 AND folder_id = ?2")?
        .execute(params![id.0, folder.0])?;
    let deleted = tx
        .prepare_cached(
            "DELETE FROM message WHERE id = ?1
             AND NOT EXISTS (SELECT 1 FROM message_location WHERE message_id = ?1)",
        )?
        .execute([id.0])?;
    let op = if deleted > 0 {
        ChangeOp::Delete
    } else {
        ChangeOp::Update
    };
    journal::record(tx, ObjectKind::Message, id.0, op)?;
    Ok(())
}

fn keywords_json(keywords: &[String]) -> Option<String> {
    (!keywords.is_empty()).then(|| serde_json::to_string(keywords).expect("strings serialize"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DbKind, Mode, ParticipantRole};
    use katna_core::{AccountKind, Paths};

    fn open() -> (tempfile::TempDir, Store, AccountId) {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Imap, "Work", "ada@example.org")
            .unwrap()
            .id;
        (tmp, store, account)
    }

    const FROM: NewParticipant<'static> = NewParticipant {
        role: ParticipantRole::From,
        email_norm: "bob@example.org",
        domain: "example.org",
        display_name: Some("Bob"),
    };

    fn remote(
        uid: u32,
        participants: &'static [NewParticipant<'static>],
    ) -> RemoteMessage<'static> {
        RemoteMessage {
            uid,
            message_id_hdr: Some("1@example.org"),
            subject: Some("Hello"),
            date: Some(1_790_000_000),
            size: 1234,
            flags: MessageFlags::empty(),
            keywords: &[],
            has_attachments: false,
            list_id: None,
            participants,
        }
    }

    fn message_count(store: &Store) -> i64 {
        store
            .mail
            .query_row("SELECT count(*) FROM message", [], |row| row.get(0))
            .unwrap()
    }

    #[test]
    fn folder_roles_round_trip() {
        for role in FolderRole::ALL {
            assert_eq!(role.as_str().parse::<FolderRole>().unwrap(), role);
        }
        assert!("nope".parse::<FolderRole>().is_err());
    }

    #[test]
    fn upserts_folders_and_keeps_state() {
        let (_tmp, mut store, account) = open();
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let sent = batch.upsert_folder(account, "Sent", None).unwrap();
        assert_eq!(
            batch
                .upsert_folder(account, "Sent", Some(FolderRole::Sent))
                .unwrap(),
            sent
        );
        batch
            .set_folder_state(inbox, Some(7), Some(99), Some(r#"{"last_uid":3}"#))
            .unwrap();
        batch.commit().unwrap();

        let folders = store.folders(account).unwrap();
        assert_eq!(
            folders,
            vec![
                StoredFolder {
                    id: inbox,
                    path: "INBOX".into(),
                    role: Some(FolderRole::Inbox),
                    uidvalidity: Some(7),
                    highestmodseq: Some(99),
                    sync_state: Some(r#"{"last_uid":3}"#.into()),
                },
                StoredFolder {
                    id: sent,
                    path: "Sent".into(),
                    role: Some(FolderRole::Sent),
                    uidvalidity: None,
                    highestmodseq: None,
                    sync_state: None,
                },
            ]
        );
        // Insert, insert, role update.
        assert_eq!(store.latest_change(DbKind::Mail).unwrap(), 3);
    }

    #[test]
    fn adds_updates_and_removes_remote_messages() {
        let (_tmp, mut store, account) = open();
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let Added::Message(first) = batch
            .add_remote_message(account, inbox, &remote(1, &[FROM]))
            .unwrap()
        else {
            panic!("expected a new message");
        };
        assert_eq!(
            batch
                .add_remote_message(account, inbox, &remote(1, &[]))
                .unwrap(),
            Added::Duplicate(first)
        );
        batch
            .add_remote_message(account, inbox, &remote(2, &[]))
            .unwrap();
        batch
            .add_remote_message(account, inbox, &remote(5, &[]))
            .unwrap();

        let keywords = vec!["$Label1".to_owned()];
        assert_eq!(
            batch
                .set_remote_flags(inbox, 1, MessageFlags::SEEN, &keywords)
                .unwrap(),
            Some(first)
        );
        assert_eq!(
            batch
                .set_remote_flags(inbox, 1, MessageFlags::SEEN, &keywords)
                .unwrap(),
            None,
            "unchanged flags are not an update"
        );
        assert_eq!(
            batch
                .set_remote_flags(inbox, 9, MessageFlags::SEEN, &[])
                .unwrap(),
            None
        );
        assert_eq!(batch.remove_remote_messages(inbox, &[2, 3]).unwrap(), 1);
        batch.commit().unwrap();

        assert_eq!(store.folder_uids(inbox).unwrap(), vec![1, 5]);
        let (flags, kw): (u32, String) = store
            .mail
            .query_row(
                "SELECT flags, keywords FROM message WHERE id = ?1",
                [first.0],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(flags, MessageFlags::SEEN.bits());
        assert_eq!(kw, r#"["$Label1"]"#);
        assert_eq!(message_count(&store), 2);

        let mut batch = store.mail_batch().unwrap();
        assert_eq!(batch.clear_folder(inbox).unwrap(), 2);
        batch.commit().unwrap();
        assert!(store.folder_uids(inbox).unwrap().is_empty());
        assert_eq!(message_count(&store), 0);
        let participants: i64 = store
            .mail
            .query_row("SELECT count(*) FROM participant", [], |row| row.get(0))
            .unwrap();
        assert_eq!(participants, 0, "participants go with their message");
    }

    #[test]
    fn removing_a_folder_removes_its_messages() {
        let (_tmp, mut store, account) = open();
        let mut batch = store.mail_batch().unwrap();
        let old = batch.upsert_folder(account, "Old", None).unwrap();
        batch
            .add_remote_message(account, old, &remote(1, &[]))
            .unwrap();
        batch.remove_folder(old).unwrap();
        batch.commit().unwrap();
        assert!(store.folders(account).unwrap().is_empty());
        assert_eq!(message_count(&store), 0);
    }

    #[test]
    fn bodies_are_downloaded_newest_first_within_limits() {
        let (_tmp, mut store, account) = open();
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        for (uid, date, size) in [(1, 100, 10), (2, 300, 10), (3, 200, 10), (4, 400, 99_999)] {
            let message = RemoteMessage {
                date: Some(date),
                size,
                ..remote(uid, &[])
            };
            batch.add_remote_message(account, inbox, &message).unwrap();
        }
        batch.commit().unwrap();

        let wanted = store.messages_without_body(inbox, None, 1000, 10).unwrap();
        let uids: Vec<u32> = wanted.iter().map(|(_, uid)| *uid).collect();
        assert_eq!(uids, [2, 3, 1], "newest first, big one left out");
        let recent = store
            .messages_without_body(inbox, Some(200), 1000, 10)
            .unwrap();
        assert_eq!(recent.len(), 2);

        let (id, _) = wanted[0];
        let (found_account, folder, uid) = store.remote_location(id).unwrap().unwrap();
        assert_eq!(
            (found_account, folder.path.as_str(), uid),
            (account, "INBOX", 2)
        );

        let before = store.latest_change(DbKind::Mail).unwrap();
        let raw = b"Subject: Hello\r\n\r\nThe body.\r\n";
        let mut batch = store.mail_batch().unwrap();
        batch
            .set_message_body(id, raw, Some("The body."), false)
            .unwrap();
        batch.commit().unwrap();
        assert!(store.latest_change(DbKind::Mail).unwrap() > before);
        let stored = &store.messages_by_id(&[id]).unwrap()[0];
        assert_eq!(stored.snippet.as_deref(), Some("The body."));
        let hash = stored.blob_hash.unwrap();
        assert_eq!(store.blobs().get(&hash).unwrap().unwrap(), raw);
        let left = store.messages_without_body(inbox, None, 1000, 10).unwrap();
        assert_eq!(left.len(), 2);
        assert_eq!(store.remote_location(MessageId(999)).unwrap(), None);
    }
}
