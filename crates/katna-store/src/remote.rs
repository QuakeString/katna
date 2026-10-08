// SPDX-License-Identifier: GPL-3.0-or-later

//! Mail that lives on a server: folders with their sync state, and messages
//! known by folder and UID before any body is downloaded (sync level 1,
//! `docs/ARCHITECTURE.md` §6.2).
//!
//! Each server copy of a message is its own `message` row with one
//! `message_location`, because IMAP flags belong to the copy. Merging copies
//! (Gmail labels) comes with threading.

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use katna_core::{AccountId, MailCategory};
use rusqlite::{OptionalExtension, Transaction, params};

use crate::error::{Error, Result};
use crate::journal::{self, ChangeOp, ObjectKind};
use crate::mail::{Added, FolderId, MailBatch, MessageFlags, MessageId, NewParticipant};
use crate::thread::Links;
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

    /// The role a folder's name suggests, for servers and imports that do
    /// not mark their special folders ("Deleted Items", `_sent_mail`).
    /// `name` is the last part of the path.
    pub fn from_name(name: &str) -> Option<Self> {
        Some(
            match name
                .to_lowercase()
                .trim_matches('_')
                .replace('_', " ")
                .as_str()
            {
                "inbox" => Self::Inbox,
                "starred" | "flagged" => Self::Flagged,
                "drafts" => Self::Drafts,
                "sent" | "sent items" | "sent mail" | "sent messages" => Self::Sent,
                "archive" | "archives" => Self::Archive,
                "junk" | "spam" | "junk e-mail" | "junk email" => Self::Junk,
                "trash" | "deleted items" | "deleted messages" | "bin" => Self::Trash,
                "all mail" => Self::All,
                _ => return None,
            },
        )
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
    /// `In-Reply-To` without angle brackets.
    pub in_reply_to: Option<&'a str>,
    /// `References`, oldest first, without angle brackets.
    pub references: &'a [&'a str],
    /// Gmail's `X-GM-THRID`, when the server has `X-GM-EXT-1`.
    pub gm_thread_id: Option<u64>,
    /// Gmail's `X-GM-MSGID`: the same message under another label is
    /// stored once, in several folders.
    pub gm_msgid: Option<u64>,
    /// Inbox tab; `None` leaves it unclassified (shown as Primary).
    pub category: Option<MailCategory>,
    /// The attachments the message's structure names, when the server
    /// sent it (IMAP `BODYSTRUCTURE`).
    pub attachments: &'a [NewAttachment<'a>],
}

/// An attachment of a message whose body may not be downloaded yet.
#[derive(Debug, Clone, Copy)]
pub struct NewAttachment<'a> {
    /// IMAP body section, like `2` or `1.3`.
    pub part: &'a str,
    /// `type/subtype`, lower case.
    pub mime: &'a str,
    pub filename: Option<&'a str>,
    /// Decoded size in bytes, estimated from the encoded one.
    pub size: u64,
}

/// An attachment of a stored message, from [`Store::attachments`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredAttachment {
    /// IMAP body section, like `2` or `1.3`.
    pub part: String,
    /// `type/subtype`, lower case.
    pub mime: String,
    pub filename: Option<String>,
    /// Decoded size in bytes (estimated).
    pub size: u64,
}

impl Store {
    /// The attachments of `message` its structure names, in body order.
    /// Empty for messages synced before structures were read, and for
    /// POP3 and imported mail: there only `has_attachments` is known.
    pub fn attachments(&self, message: MessageId) -> Result<Vec<StoredAttachment>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT part_id, mime, filename, size FROM attachment
             WHERE message_id = ?1 ORDER BY id",
        )?;
        let rows = stmt.query_map([message.0], |row| {
            Ok(StoredAttachment {
                part: row.get(0)?,
                mime: row.get(1)?,
                filename: row.get(2)?,
                size: row.get::<_, i64>(3)?.try_into().unwrap_or_default(),
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// [`attachments`](Self::attachments) of several messages, for the
    /// message list. Messages without any are left out.
    pub fn attachment_lists(
        &self,
        messages: &[MessageId],
    ) -> Result<HashMap<MessageId, Vec<StoredAttachment>>> {
        let mut out = HashMap::new();
        for &message in messages {
            let list = self.attachments(message)?;
            if !list.is_empty() {
                out.insert(message, list);
            }
        }
        Ok(out)
    }

    /// The newest message ID of `account`; 0 when it has none.
    pub fn latest_message(&self, account: AccountId) -> Result<MessageId> {
        let id: Option<i64> = self
            .mail
            .prepare_cached("SELECT max(id) FROM message WHERE account_id = ?1")?
            .query_row([account.0], |row| row.get(0))?;
        Ok(MessageId(id.unwrap_or(0)))
    }

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

    /// Where Delete puts `account`'s mail: the folder marked Trash, else a
    /// top-level one (or one under INBOX) named so, such as "Deleted
    /// Items". `None` means Delete removes mail for good.
    pub fn trash_folder(&self, account: AccountId) -> Result<Option<FolderId>> {
        let folders = self.folders(account)?;
        let marked = folders.iter().find(|f| f.role == Some(FolderRole::Trash));
        let named = || {
            folders.iter().find(|f| {
                let name = match f.path.split_once('/') {
                    None => f.path.as_str(),
                    Some((parent, name))
                        if parent.eq_ignore_ascii_case("INBOX") && !name.contains('/') =>
                    {
                        name
                    }
                    Some(_) => return false,
                };
                FolderRole::from_name(name) == Some(FolderRole::Trash)
            })
        };
        Ok(marked.or_else(named).map(|f| f.id))
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

    /// The messages stored in `folder` with their UIDs there, by UID.
    /// Messages whose UID is not known yet are left out.
    pub fn folder_message_uids(&self, folder: FolderId) -> Result<Vec<(u32, MessageId)>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT uid, message_id FROM message_location
             WHERE folder_id = ?1 AND uid IS NOT NULL ORDER BY uid",
        )?;
        let rows = stmt.query_map([folder.0], |row| Ok((row.get(0)?, MessageId(row.get(1)?))))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Whether mail of `account` (of any account when `None`) is on a
    /// server but not downloaded: what only a search on the server finds
    /// by its text (`docs/ARCHITECTURE.md` §7.3).
    pub fn has_mail_not_downloaded(&self, account: Option<AccountId>) -> Result<bool> {
        Ok(self
            .mail
            .prepare_cached(
                "SELECT EXISTS (SELECT 1 FROM message m
                 WHERE m.blob_hash IS NULL AND (?1 IS NULL OR m.account_id = ?1)
                   AND EXISTS (SELECT 1 FROM message_location l
                               WHERE l.message_id = m.id AND l.uid IS NOT NULL))",
            )?
            .query_row([account.map(|a| a.0)], |row| row.get(0))?)
    }

    /// The folders of `account` holding mail not downloaded, with how much
    /// of it, most first.
    pub fn folders_not_downloaded(&self, account: AccountId) -> Result<Vec<(FolderId, u64)>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT l.folder_id, count(*) AS n FROM message_location l
             JOIN message m ON m.id = l.message_id
             WHERE m.account_id = ?1 AND m.blob_hash IS NULL AND l.uid IS NOT NULL
             GROUP BY l.folder_id ORDER BY n DESC, l.folder_id",
        )?;
        let rows = stmt.query_map([account.0], |row| {
            Ok((FolderId(row.get(0)?), row.get::<_, i64>(1)? as u64))
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Of the messages at `uids` in `folder`, the ones not downloaded, with
    /// their dates.
    pub fn not_downloaded_at(
        &self,
        folder: FolderId,
        uids: &[u32],
    ) -> Result<Vec<(MessageId, Option<i64>)>> {
        let mut stmt = self.mail.prepare_cached(
            "SELECT m.id, m.date FROM message_location l
             JOIN message m ON m.id = l.message_id
             WHERE l.folder_id = ?1 AND m.blob_hash IS NULL
               AND l.uid IN (SELECT value FROM json_each(?2))",
        )?;
        let mut found = Vec::new();
        for chunk in uids.chunks(1_000) {
            let list = format!(
                "[{}]",
                chunk
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            );
            let rows = stmt.query_map(params![folder.0, list], |row| {
                Ok((MessageId(row.get(0)?), row.get(1)?))
            })?;
            for row in rows {
                found.push(row?);
            }
        }
        Ok(found)
    }

    /// The account `folder` belongs to, if it exists.
    pub fn folder_account(&self, folder: FolderId) -> Result<Option<AccountId>> {
        Ok(self
            .mail
            .prepare_cached("SELECT account_id FROM folder WHERE id = ?1")?
            .query_row([folder.0], |row| row.get(0).map(AccountId))
            .optional()?)
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

    /// Renames folder `old` of `account` to `new`, and the folders inside
    /// it (`old`, `delimiter`, then more) to the same place under `new`,
    /// keeping their messages and sync state. A folder already at one of
    /// the new paths (a sync that saw the server's rename first) is
    /// removed in favour of the renamed one. Returns how many folders were
    /// renamed.
    pub fn rename_folder_tree(
        &mut self,
        account: AccountId,
        old: &str,
        new: &str,
        delimiter: Option<char>,
    ) -> Result<usize> {
        let inside = |path: &str| {
            path == old
                || delimiter.is_some_and(|d| {
                    path.strip_prefix(old)
                        .is_some_and(|rest| rest.starts_with(d))
                })
        };
        let folders: Vec<(i64, String)> = self
            .tx()
            .prepare_cached("SELECT id, path FROM folder WHERE account_id = ?1")?
            .query_map([account.0], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<rusqlite::Result<_>>()?;
        let renames: Vec<(i64, String)> = folders
            .iter()
            .filter(|(_, path)| inside(path))
            .map(|(id, path)| (*id, format!("{new}{}", &path[old.len()..])))
            .collect();
        let in_the_way: Vec<i64> = folders
            .iter()
            .filter(|(id, path)| {
                !renames.iter().any(|(moving, _)| moving == id)
                    && renames.iter().any(|(_, new)| new == path)
            })
            .map(|(id, _)| *id)
            .collect();
        for id in in_the_way {
            self.remove_folder(FolderId(id))?;
        }
        let tx = self.tx();
        for (id, path) in &renames {
            tx.prepare_cached("UPDATE folder SET path = ?2 WHERE id = ?1")?
                .execute(params![id, path])?;
            journal::record(tx, ObjectKind::Folder, *id, ChangeOp::Update)?;
        }
        Ok(renames.len())
    }

    /// Puts `message` back in `folder` at `uid`, when both still exist.
    /// Returns whether it was added.
    pub fn restore_location(
        &mut self,
        message: MessageId,
        folder: FolderId,
        uid: Option<u32>,
    ) -> Result<bool> {
        let tx = self.tx();
        let added = tx
            .prepare_cached(
                "INSERT OR IGNORE INTO message_location (message_id, folder_id, uid)
                 SELECT ?1, ?2, ?3
                 WHERE EXISTS (SELECT 1 FROM message WHERE id = ?1)
                   AND EXISTS (SELECT 1 FROM folder WHERE id = ?2)",
            )?
            .execute(params![message.0, folder.0, uid])?
            > 0;
        if added {
            journal::record(tx, ObjectKind::Message, message.0, ChangeOp::Update)?;
        }
        Ok(added)
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
    /// [`Added::Duplicate`] if that UID is already stored, and
    /// [`Added::Location`] if the account has the message under another
    /// Gmail label (same `gm_msgid`): then only the folder is added.
    pub fn add_remote_message(
        &mut self,
        account: AccountId,
        folder: FolderId,
        message: &RemoteMessage<'_>,
    ) -> Result<Added> {
        if let Some(id) = message_at(self.tx(), folder, message.uid)? {
            return Ok(Added::Duplicate(id));
        }
        if let Some(gm_msgid) = message.gm_msgid {
            let tx = self.tx();
            let known: Option<i64> = tx
                .prepare_cached("SELECT id FROM message WHERE account_id = ?1 AND gm_msgid = ?2")?
                .query_row(params![account.0, gm_msgid as i64], |row| row.get(0))
                .optional()?;
            if let Some(id) = known {
                // A location without a UID is a local move this confirms.
                tx.prepare_cached(
                    "INSERT INTO message_location (message_id, folder_id, uid) VALUES (?1, ?2, ?3)
                     ON CONFLICT (message_id, folder_id) DO UPDATE SET uid = excluded.uid",
                )?
                .execute(params![id, folder.0, message.uid])?;
                journal::record(tx, ObjectKind::Message, id, ChangeOp::Update)?;
                return Ok(Added::Location(MessageId(id)));
            }
        }
        let thread = self.assign_thread(&Links {
            account,
            message_id_hdr: message.message_id_hdr,
            in_reply_to: message.in_reply_to,
            references: message.references,
            subject: message.subject.unwrap_or_default(),
            date: message.date,
            gm_thread_id: message.gm_thread_id,
        })?;
        let tx = self.tx();
        tx.prepare_cached(
            "INSERT INTO message (account_id, message_id_hdr, subject, date, size, flags,
                                  keywords, has_attachments, list_id, thread_id, category,
                                  gm_msgid)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
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
            thread,
            message.category.map(MailCategory::to_storage),
            message.gm_msgid.map(|id| id as i64),
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
        crate::attachments::insert(tx, MessageId(id), message.attachments)?;
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
            in_reply_to: None,
            references: &[],
            gm_thread_id: None,
            gm_msgid: None,
            category: None,
            attachments: &[],
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
    fn finds_trash_by_mark_or_name() {
        let (_tmp, mut store, account) = open();
        let mut batch = store.mail_batch().unwrap();
        batch.upsert_folder(account, "INBOX", None).unwrap();
        // Nested elsewhere (an imported mailbox's "deleted_items"): not
        // where Delete goes.
        batch
            .upsert_folder(account, "alin-m/deleted_items", None)
            .unwrap();
        batch.commit().unwrap();
        assert_eq!(store.trash_folder(account).unwrap(), None);

        let mut batch = store.mail_batch().unwrap();
        let named = batch.upsert_folder(account, "Deleted Items", None).unwrap();
        batch.commit().unwrap();
        assert_eq!(store.trash_folder(account).unwrap(), Some(named));

        let mut batch = store.mail_batch().unwrap();
        let marked = batch
            .upsert_folder(account, "Bin2", Some(FolderRole::Trash))
            .unwrap();
        batch.commit().unwrap();
        assert_eq!(store.trash_folder(account).unwrap(), Some(marked));
    }

    #[test]
    fn roles_from_names() {
        assert_eq!(FolderRole::from_name("_sent_mail"), Some(FolderRole::Sent));
        assert_eq!(
            FolderRole::from_name("deleted_items"),
            Some(FolderRole::Trash)
        );
        assert_eq!(FolderRole::from_name("Junk E-mail"), Some(FolderRole::Junk));
        assert_eq!(FolderRole::from_name("Projects"), None);
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
    fn finds_mail_not_downloaded() {
        let (_tmp, mut store, account) = open();
        assert!(!store.has_mail_not_downloaded(None).unwrap());
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let mut ids = Vec::new();
        for uid in [1, 2, 3] {
            let Added::Message(id) = batch
                .add_remote_message(account, inbox, &remote(uid, &[]))
                .unwrap()
            else {
                panic!("expected a new message");
            };
            ids.push(id);
        }
        batch
            .set_message_body(ids[1], b"Subject: x\r\n\r\nbody", None, false)
            .unwrap();
        batch.commit().unwrap();
        assert!(store.has_mail_not_downloaded(Some(account)).unwrap());
        assert!(!store.has_mail_not_downloaded(Some(AccountId(99))).unwrap());
        assert_eq!(store.folders_not_downloaded(account).unwrap(), [(inbox, 2)]);
        let found: Vec<MessageId> = store
            .not_downloaded_at(inbox, &[1, 2, 3, 7])
            .unwrap()
            .into_iter()
            .map(|(id, _)| id)
            .collect();
        assert_eq!(found, [ids[0], ids[2]]);
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
    fn keeps_the_attachments_a_structure_names() {
        let (_tmp, mut store, account) = open();
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let all = batch
            .upsert_folder(account, "[Gmail]/All Mail", Some(FolderRole::All))
            .unwrap();
        let attachments = [
            NewAttachment {
                part: "2",
                mime: "application/pdf",
                filename: Some("€ rates.pdf"),
                size: 51_200,
            },
            NewAttachment {
                part: "3.1",
                mime: "image/png",
                filename: None,
                size: 900,
            },
        ];
        let message = RemoteMessage {
            has_attachments: true,
            attachments: &attachments,
            gm_msgid: Some(7),
            ..remote(1, &[])
        };
        let Added::Message(id) = batch.add_remote_message(account, inbox, &message).unwrap() else {
            panic!("expected a new message");
        };
        // The same Gmail message under another label keeps one list.
        let label = RemoteMessage { uid: 4, ..message };
        batch.add_remote_message(account, all, &label).unwrap();
        batch.commit().unwrap();

        let stored = store.attachments(id).unwrap();
        assert_eq!(
            stored,
            [
                StoredAttachment {
                    part: "2".into(),
                    mime: "application/pdf".into(),
                    filename: Some("€ rates.pdf".into()),
                    size: 51_200,
                },
                StoredAttachment {
                    part: "3.1".into(),
                    mime: "image/png".into(),
                    filename: None,
                    size: 900,
                },
            ]
        );
        let lists = store
            .attachment_lists(&[id, MessageId(id.0 + 100)])
            .unwrap();
        assert_eq!(lists.len(), 1, "messages without attachments are left out");
        assert_eq!(lists[&id], stored);

        let mut batch = store.mail_batch().unwrap();
        batch.remove_folder(inbox).unwrap();
        batch.remove_folder(all).unwrap();
        batch.commit().unwrap();
        assert!(store.attachments(id).unwrap().is_empty());
        let rows: i64 = store
            .mail
            .query_row("SELECT count(*) FROM attachment", [], |row| row.get(0))
            .unwrap();
        assert_eq!(rows, 0, "attachments go with their message");
    }

    #[test]
    fn finds_every_copy_of_a_message() {
        let (_tmp, mut store, account) = open();
        let other = store
            .add_account(AccountKind::Imap, "Home", "ada@example.net")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let all = batch
            .upsert_folder(account, "[Gmail]/All Mail", Some(FolderRole::All))
            .unwrap();
        let elsewhere = batch
            .upsert_folder(other, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let mut add = |account, folder, message: RemoteMessage<'_>| {
            let Added::Message(id) = batch.add_remote_message(account, folder, &message).unwrap()
            else {
                panic!("expected a new message");
            };
            id
        };
        // Gmail mail synced before Gmail's message IDs: one row per label.
        let starred = RemoteMessage {
            flags: MessageFlags::FLAGGED,
            ..remote(1, &[])
        };
        let in_inbox = add(account, inbox, starred);
        let in_all = add(account, all, remote(1, &[]));
        // The same Message-ID in another account is another message.
        let other_account = add(other, elsewhere, remote(1, &[]));
        let no_id = add(
            account,
            inbox,
            RemoteMessage {
                message_id_hdr: None,
                ..remote(2, &[])
            },
        );
        batch.commit().unwrap();

        assert_eq!(
            store.with_copies(&[in_all, no_id, in_inbox]).unwrap(),
            [
                (in_inbox, MessageFlags::FLAGGED),
                (in_all, MessageFlags::empty()),
                (no_id, MessageFlags::empty()),
            ]
        );
        assert_eq!(
            store.with_copies(&[other_account]).unwrap(),
            [(other_account, MessageFlags::empty())]
        );
    }

    #[test]
    fn finds_new_inbox_mail_for_notifications() {
        let (_tmp, mut store, account) = open();
        assert_eq!(store.latest_message(account).unwrap(), MessageId(0));
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let archive = batch.upsert_folder(account, "Archive", None).unwrap();
        let mut add = |folder, uid, date, flags, category| {
            let message = RemoteMessage {
                date: Some(date),
                flags,
                category,
                ..remote(uid, &[])
            };
            match batch.add_remote_message(account, folder, &message).unwrap() {
                Added::Message(id) => id,
                other => panic!("{other:?}"),
            }
        };
        let none = MessageFlags::empty();
        let before = add(inbox, 1, 2_000, none, None);
        let primary = add(inbox, 2, 2_000, none, Some(MailCategory::Primary));
        add(inbox, 3, 2_000, MessageFlags::SEEN, None);
        add(inbox, 4, 2_000, none, Some(MailCategory::Promotions));
        add(archive, 5, 2_000, none, None);
        add(inbox, 6, 500, none, None);
        let unclassified = add(inbox, 7, 3_000, none, None);
        batch.commit().unwrap();

        assert_eq!(store.latest_message(account).unwrap(), unclassified);
        assert_eq!(
            store
                .new_ringing_mail(account, before, 1_000, 10, 4_000)
                .unwrap(),
            [primary, unclassified]
        );
        assert_eq!(
            store
                .new_ringing_mail(account, before, 1_000, 1, 4_000)
                .unwrap(),
            [unclassified]
        );
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
    fn renaming_a_folder_renames_the_folders_inside() {
        let (_tmp, mut store, account) = open();
        let mut batch = store.mail_batch().unwrap();
        let work = batch.upsert_folder(account, "Work", None).unwrap();
        let child = batch.upsert_folder(account, "Work/2026", None).unwrap();
        let grandchild = batch.upsert_folder(account, "Work/2026/Q1", None).unwrap();
        let sibling = batch.upsert_folder(account, "Workshop", None).unwrap();
        // A sync that saw the new name before the store did.
        let early = batch.upsert_folder(account, "Jobs", None).unwrap();
        batch
            .add_remote_message(account, child, &remote(1, &[]))
            .unwrap();
        batch
            .set_folder_state(work, Some(7), Some(9), Some("{}"))
            .unwrap();
        assert_eq!(
            batch
                .rename_folder_tree(account, "Work", "Jobs", Some('/'))
                .unwrap(),
            3
        );
        batch.commit().unwrap();

        let folders = store.folders(account).unwrap();
        let path_of = |id| folders.iter().find(|f| f.id == id).map(|f| f.path.as_str());
        assert_eq!(path_of(work), Some("Jobs"));
        assert_eq!(path_of(child), Some("Jobs/2026"));
        assert_eq!(path_of(grandchild), Some("Jobs/2026/Q1"));
        assert_eq!(path_of(sibling), Some("Workshop"), "only folders inside");
        assert_eq!(path_of(early), None, "the renamed one takes its place");
        let renamed = folders.iter().find(|f| f.id == work).unwrap();
        assert_eq!(
            (renamed.uidvalidity, renamed.highestmodseq),
            (Some(7), Some(9))
        );
        assert_eq!(store.folder_uids(child).unwrap(), [1], "messages stay");
        assert_eq!(store.folder_account(child).unwrap(), Some(account));
        assert_eq!(store.folder_account(early).unwrap(), None);
        let uids = store.folder_message_uids(child).unwrap();
        assert_eq!(uids.len(), 1);
        assert_eq!(uids[0].0, 1);
    }

    #[test]
    fn a_location_is_restored_only_where_both_exist() {
        let (_tmp, mut store, account) = open();
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let label = batch.upsert_folder(account, "Label", None).unwrap();
        let Added::Message(id) = batch
            .add_remote_message(account, inbox, &remote(1, &[]))
            .unwrap()
        else {
            panic!("new message expected");
        };
        assert!(batch.restore_location(id, label, Some(4)).unwrap());
        assert!(!batch.restore_location(id, label, Some(4)).unwrap());
        assert!(!batch.restore_location(id, FolderId(999), None).unwrap());
        assert!(!batch.restore_location(MessageId(999), label, None).unwrap());
        batch.commit().unwrap();
        assert_eq!(store.folder_uids(label).unwrap(), [4]);
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
