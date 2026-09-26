// SPDX-License-Identifier: GPL-3.0-or-later

//! Writing messages to `mail.db` (schema in `schema/mail_v1.sql`).
//!
//! Writes go through a [`MailBatch`]: one `IMMEDIATE` transaction on
//! `mail.db` and one on `blobs.db`, so bulk imports and sync commit thousands
//! of messages at once. Blobs are committed before the rows that reference
//! them (`docs/ARCHITECTURE.md` §5.2); if the second commit fails, the
//! unreferenced blobs are removed by the background cleanup.

use std::fmt;
use std::ops::{BitOr, BitOrAssign};

use std::collections::BTreeSet;

use katna_core::{AccountId, MailCategory};
use rusqlite::{OptionalExtension, Transaction, TransactionBehavior, params};

use crate::blob::BlobStore;
use crate::error::Result;
use crate::journal::{self, ChangeOp, ObjectKind};
use crate::thread::{self, Links};

/// Database ID of a folder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FolderId(pub i64);

/// Database ID of a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageId(pub i64);

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Database ID of a thread (conversation). Threads are merged when a
/// message links two of them, so a thread ID can disappear; the change
/// journal records that as a thread `delete`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ThreadId(pub i64);

impl fmt::Display for ThreadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// `message.flags`: IMAP system flags as a bit set.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MessageFlags(u32);

impl MessageFlags {
    pub const SEEN: Self = Self(1 << 0);
    pub const ANSWERED: Self = Self(1 << 1);
    pub const FLAGGED: Self = Self(1 << 2);
    pub const DRAFT: Self = Self(1 << 3);
    pub const DELETED: Self = Self(1 << 4);
    /// `$Forwarded` keyword; kept here because every client shows it.
    pub const FORWARDED: Self = Self(1 << 5);

    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn bits(self) -> u32 {
        self.0
    }

    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Adds `other` when `on` is true.
    pub fn set(&mut self, other: Self, on: bool) {
        if on {
            self.0 |= other.0;
        } else {
            self.0 &= !other.0;
        }
    }
}

impl BitOr for MessageFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for MessageFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// `participant.role`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParticipantRole {
    From,
    To,
    Cc,
    Bcc,
    ReplyTo,
    Sender,
}

impl ParticipantRole {
    /// Stable name stored in `participant.role`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::From => "from",
            Self::To => "to",
            Self::Cc => "cc",
            Self::Bcc => "bcc",
            Self::ReplyTo => "reply_to",
            Self::Sender => "sender",
        }
    }

    /// Reads a name stored in `participant.role`.
    pub fn parse(name: &str) -> Option<Self> {
        [
            Self::From,
            Self::To,
            Self::Cc,
            Self::Bcc,
            Self::ReplyTo,
            Self::Sender,
        ]
        .into_iter()
        .find(|role| role.as_str() == name)
    }
}

/// One address of a new message.
#[derive(Debug, Clone, Copy)]
pub struct NewParticipant<'a> {
    pub role: ParticipantRole,
    /// Trimmed, lower-cased address.
    pub email_norm: &'a str,
    /// Part after the last `@`, or empty.
    pub domain: &'a str,
    pub display_name: Option<&'a str>,
}

/// A message to store, with the header fields the store indexes.
#[derive(Debug, Clone, Copy)]
pub struct NewMessage<'a> {
    /// The raw RFC 5322 message; stored as a blob.
    pub raw: &'a [u8],
    /// `Message-ID` without angle brackets.
    pub message_id_hdr: Option<&'a str>,
    pub subject: Option<&'a str>,
    /// Unix seconds.
    pub date: Option<i64>,
    pub flags: MessageFlags,
    pub has_attachments: bool,
    pub list_id: Option<&'a str>,
    pub snippet: Option<&'a str>,
    pub participants: &'a [NewParticipant<'a>],
    /// `In-Reply-To` without angle brackets.
    pub in_reply_to: Option<&'a str>,
    /// `References`, oldest first, without angle brackets.
    pub references: &'a [&'a str],
    /// Inbox tab; `None` leaves it unclassified (shown as Primary).
    pub category: Option<MailCategory>,
}

/// What [`MailBatch::add_message`] did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Added {
    /// A new message.
    Message(MessageId),
    /// The same raw message was already in the account; it is now also in
    /// this folder (a copy or a label).
    Location(MessageId),
    /// The same raw message was already in this folder; nothing changed.
    Duplicate(MessageId),
}

/// A write transaction on `mail.db` and `blobs.db`. Dropping it without
/// [`commit`](Self::commit) rolls both back.
pub struct MailBatch<'s> {
    tx: Option<Transaction<'s>>,
    blobs: &'s BlobStore,
    /// Threads that gained or lost messages; journaled once at commit.
    threads: BTreeSet<i64>,
}

impl<'s> MailBatch<'s> {
    pub(crate) fn begin(mail: &'s mut rusqlite::Connection, blobs: &'s BlobStore) -> Result<Self> {
        let tx = mail.transaction_with_behavior(TransactionBehavior::Immediate)?;
        blobs.begin()?;
        Ok(Self {
            tx: Some(tx),
            blobs,
            threads: BTreeSet::new(),
        })
    }

    pub(crate) fn blobs(&self) -> &'s BlobStore {
        self.blobs
    }

    pub(crate) fn tx(&self) -> &Transaction<'s> {
        self.tx.as_ref().expect("transaction is open until commit")
    }

    /// The thread a message joins, creating or merging threads as needed.
    /// Remembers it for the journal.
    pub(crate) fn assign_thread(&mut self, links: &Links<'_>) -> Result<i64> {
        let tx = self.tx.as_ref().expect("transaction is open until commit");
        let (thread, created) = thread::assign(tx, links, &mut self.threads)?;
        if !created {
            self.threads.insert(thread);
        }
        Ok(thread)
    }

    /// Remembers that `thread` changed, for the journal.
    pub(crate) fn thread_changed(&mut self, thread: i64) {
        self.threads.insert(thread);
    }

    /// Returns the folder `path` of `account`, creating it if needed.
    pub fn ensure_folder(&mut self, account: AccountId, path: &str) -> Result<FolderId> {
        let tx = self.tx();
        let existing = tx
            .prepare_cached("SELECT id FROM folder WHERE account_id = ?1 AND path = ?2")?
            .query_row(params![account.0, path], |row| row.get(0))
            .optional()?;
        if let Some(id) = existing {
            return Ok(FolderId(id));
        }
        tx.prepare_cached("INSERT INTO folder (account_id, path) VALUES (?1, ?2)")?
            .execute(params![account.0, path])?;
        let id = tx.last_insert_rowid();
        journal::record(tx, ObjectKind::Folder, id, ChangeOp::Insert)?;
        Ok(FolderId(id))
    }

    /// Stores a message in `folder` of `account`.
    ///
    /// Messages are identified by their raw bytes: the same bytes again in
    /// the same account add a location, not a second message.
    pub fn add_message(
        &mut self,
        account: AccountId,
        folder: FolderId,
        message: &NewMessage<'_>,
    ) -> Result<Added> {
        let hash = self.blobs.put(message.raw)?;
        let tx = self.tx();

        let existing: Option<i64> = tx
            .prepare_cached(
                "SELECT id FROM message WHERE blob_hash = ?1 AND account_id = ?2 LIMIT 1",
            )?
            .query_row(params![hash.as_bytes(), account.0], |row| row.get(0))
            .optional()?;
        if let Some(id) = existing {
            let added = tx
                .prepare_cached(
                    "INSERT OR IGNORE INTO message_location (message_id, folder_id) VALUES (?1, ?2)",
                )?
                .execute(params![id, folder.0])?;
            if added == 0 {
                return Ok(Added::Duplicate(MessageId(id)));
            }
            journal::record(tx, ObjectKind::Message, id, ChangeOp::Update)?;
            return Ok(Added::Location(MessageId(id)));
        }

        let size = i64::try_from(message.raw.len()).unwrap_or(i64::MAX);
        let thread = self.assign_thread(&Links {
            account,
            message_id_hdr: message.message_id_hdr,
            in_reply_to: message.in_reply_to,
            references: message.references,
            subject: message.subject.unwrap_or_default(),
            date: message.date,
            gm_thread_id: None,
        })?;
        let tx = self.tx();
        tx.prepare_cached(
            "INSERT INTO message (account_id, message_id_hdr, subject, date, size, flags,
                                  has_attachments, list_id, blob_hash, snippet, thread_id,
                                  category)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
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
            thread,
            message.category.map(MailCategory::to_storage),
        ])?;
        let id = tx.last_insert_rowid();
        tx.prepare_cached("INSERT INTO message_location (message_id, folder_id) VALUES (?1, ?2)")?
            .execute(params![id, folder.0])?;
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

    /// Commits the blobs, then the rows.
    pub fn commit(mut self) -> Result<()> {
        thread::journal_changes(self.tx(), &self.threads)?;
        self.blobs.end(true)?;
        let tx = self.tx.take().expect("transaction is open until commit");
        tx.commit()?;
        Ok(())
    }
}

impl Drop for MailBatch<'_> {
    fn drop(&mut self) {
        if self.tx.is_some() {
            // The mail transaction rolls back when dropped. A failed rollback
            // leaves nothing to clean up that the next transaction won't.
            let _ = self.blobs.end(false);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DbKind, Mode, Store};
    use katna_core::{AccountKind, Paths};

    fn message<'a>(raw: &'a [u8], participants: &'a [NewParticipant<'a>]) -> NewMessage<'a> {
        NewMessage {
            raw,
            message_id_hdr: Some("1@example.org"),
            subject: Some("Budget"),
            date: Some(989_883_540),
            flags: MessageFlags::SEEN | MessageFlags::FLAGGED,
            has_attachments: false,
            list_id: None,
            snippet: Some("Here is our forecast"),
            participants,
            in_reply_to: None,
            references: &[],
            category: None,
        }
    }

    const FROM: NewParticipant<'static> = NewParticipant {
        role: ParticipantRole::From,
        email_norm: "kenneth.lay@enron.com",
        domain: "enron.com",
        display_name: Some("Kenneth Lay"),
    };

    #[test]
    fn flags_bit_set() {
        let mut flags = MessageFlags::SEEN | MessageFlags::DRAFT;
        assert!(flags.contains(MessageFlags::SEEN));
        assert!(!flags.contains(MessageFlags::FLAGGED));
        flags.set(MessageFlags::SEEN, false);
        flags |= MessageFlags::FORWARDED;
        assert_eq!(flags.bits(), (1 << 3) | (1 << 5));
        assert_eq!(MessageFlags::from_bits(flags.bits()), flags);
    }

    #[test]
    fn adds_messages_locations_and_duplicates() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Local, "Enron", "enron")
            .unwrap()
            .id;
        let reader = Store::open(&paths, Mode::ReadOnly).unwrap();

        let raw = b"From: kenneth.lay@enron.com\r\n\r\nHere is our forecast\r\n";
        let participants = [FROM];
        let new = message(raw, &participants);

        let mut batch = store.mail_batch().unwrap();
        let inbox = batch.ensure_folder(account, "lay-k/inbox").unwrap();
        assert_eq!(batch.ensure_folder(account, "lay-k/inbox").unwrap(), inbox);
        let all = batch.ensure_folder(account, "lay-k/all_documents").unwrap();
        let Added::Message(id) = batch.add_message(account, inbox, &new).unwrap() else {
            panic!("expected a new message");
        };
        assert_eq!(
            batch.add_message(account, inbox, &new).unwrap(),
            Added::Duplicate(id)
        );
        assert_eq!(
            batch.add_message(account, all, &new).unwrap(),
            Added::Location(id)
        );
        // Nothing is visible before the commit.
        assert_eq!(reader.latest_change(DbKind::Mail).unwrap(), 0);
        batch.commit().unwrap();

        let changes: Vec<_> = reader
            .changes_since(DbKind::Mail, 0, 10)
            .unwrap()
            .iter()
            .map(|c| (c.kind, c.op))
            .collect();
        assert_eq!(
            changes,
            [
                (ObjectKind::Folder, ChangeOp::Insert),
                (ObjectKind::Folder, ChangeOp::Insert),
                (ObjectKind::Thread, ChangeOp::Insert),
                (ObjectKind::Message, ChangeOp::Insert),
                (ObjectKind::Message, ChangeOp::Update),
            ]
        );
        let hash = crate::BlobHash::of(raw);
        assert_eq!(reader.blobs().get(&hash).unwrap().unwrap(), raw);

        let row = store
            .mail
            .query_row(
                "SELECT subject, date, size, flags, snippet, blob_hash FROM message WHERE id = ?1",
                [id.0],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, i64>(2)?,
                        row.get::<_, u32>(3)?,
                        row.get::<_, String>(4)?,
                        row.get::<_, Vec<u8>>(5)?,
                    ))
                },
            )
            .unwrap();
        assert_eq!(
            row,
            (
                "Budget".to_owned(),
                989_883_540,
                raw.len() as i64,
                0b101,
                "Here is our forecast".to_owned(),
                hash.as_bytes().to_vec(),
            )
        );
        let participant: (String, String, String) = store
            .mail
            .query_row(
                "SELECT role, email_norm, display_name FROM participant WHERE message_id = ?1",
                [id.0],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();
        assert_eq!(
            participant,
            (
                "from".to_owned(),
                "kenneth.lay@enron.com".to_owned(),
                "Kenneth Lay".to_owned()
            )
        );
    }

    #[test]
    fn dropped_batch_rolls_back() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
        let account = store.add_account(AccountKind::Local, "x", "x").unwrap().id;
        let raw = b"Subject: gone\r\n\r\n";
        {
            let mut batch = store.mail_batch().unwrap();
            let folder = batch.ensure_folder(account, "INBOX").unwrap();
            batch
                .add_message(account, folder, &message(raw, &[]))
                .unwrap();
        }
        assert_eq!(store.latest_change(DbKind::Mail).unwrap(), 0);
        assert!(!store.blobs().contains(&crate::BlobHash::of(raw)).unwrap());

        // The store is usable again afterwards.
        let mut batch = store.mail_batch().unwrap();
        batch.ensure_folder(account, "INBOX").unwrap();
        batch.commit().unwrap();
        assert_eq!(store.latest_change(DbKind::Mail).unwrap(), 1);
    }

    #[test]
    fn read_only_store_has_no_batches() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        drop(Store::open(&paths, Mode::ReadWrite).unwrap());
        let mut app = Store::open(&paths, Mode::ReadOnly).unwrap();
        assert!(matches!(app.mail_batch(), Err(crate::Error::ReadOnly)));
    }
}
