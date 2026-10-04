// SPDX-License-Identifier: GPL-3.0-or-later

//! SQLite schema, migrations and message storage. Only `katna-daemon` opens the
//! databases for writing; apps open them read-only. See `docs/ARCHITECTURE.md` §5.
//!
//! All SQL in Katna lives in this crate.

mod alerts;
mod attachments;
mod backfill;
pub mod blob;
mod cache;
pub mod calendar;
mod chat_pins;
mod contact;
pub mod contacts;
mod db;
pub mod error;
mod forget;
mod gmail_merge;
pub mod insights;
pub mod journal;
mod library;
pub mod mail;
mod mail_read;
mod mail_view;
pub mod meta;
pub mod notes;
pub mod ops;
mod other_contacts;
pub mod outbox;
mod people;
pub mod pop3;
mod quota;
mod receipts;
pub mod remote;
pub mod rules;
mod sender_auth;
mod summaries;
pub mod tasks;
pub mod templates;
mod thread;
pub mod tracking;
mod translation;

use katna_core::{Account, AccountId, AccountKind, AccountSettings, Paths};
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};

pub use alerts::{Bell, FolderBell, Mute, MuteTarget};
pub use backfill::Backfill;
pub use blob::{BlobHash, BlobStore};
pub use cache::Forgotten;
pub use chat_pins::{ChatPin, MAX_CHAT_PINS, Pinned};
pub use contact::{ContactConversation, ContactFile, ContactSummary};
pub use contacts::{
    AddressBook, BookSource, BookState, BookSync, ContactLabel, ContactRef, SavedContact,
    StoredCard, SyncedContact, SyncedGroup,
};
pub use db::{DbKind, Mode};
pub use error::{Error, Result};
pub use gmail_merge::Adopted;
pub use insights::{Insights, Partner, Replies};
pub use journal::{Change, ChangeOp, ObjectKind};
pub use katna_core::MailCategory;
pub use library::LibraryFile;
pub use mail::{
    Added, FolderId, MailBatch, MessageFlags, MessageId, NewMessage, NewParticipant,
    ParticipantRole, ThreadId,
};
pub use mail_read::{StoredLocation, StoredMessage, StoredParticipant};
pub use mail_view::{
    FlagFilter, FolderMarks, FolderSummary, InboxThreads, Marks, SpreadTabs, ThreadEntry,
    ThreadSender, ThreadSummary,
};
pub use meta::MetaRow;
pub use notes::{
    NOTE_LINK_SCHEME, NOTE_TRASH_KEEP, NOTE_VERSION_KEEP, Note, NotePicture, NoteVersion,
    RemoteNote, VersionSource,
};
pub use ops::{Location, PinnedMessage, QueuedOp};
pub use other_contacts::OtherContact;
pub use outbox::{DELIVERY_RECEIPT_HEADER, OutboxEntry, SendState, take_delivery_receipt};
pub use people::{Correspondent, Person};
pub use pop3::Pop3Uidl;
pub use quota::StorageQuota;
pub use receipts::{Receipt, ReceiptKind};
pub use remote::{FolderRole, NewAttachment, RemoteMessage, StoredAttachment, StoredFolder};
pub use summaries::{StoredSummary, SummaryKind};
pub use templates::{Template, TemplateFile, TemplateSummary};
pub use tracking::{
    ActivityItem, MessageActivity, NewRecipient, RecipientActivity, TrackedMessage,
    TrackedRecipient, TrackingEvent, TrackingNews,
};
pub use translation::Translation;

/// The open Katna databases: `mail.db`, `pim.db` and the blob store.
#[derive(Debug)]
pub struct Store {
    mail: Connection,
    pim: Connection,
    blobs: BlobStore,
    mode: Mode,
}

impl Store {
    /// Opens the store in the directories given by `paths`.
    ///
    /// [`Mode::ReadWrite`] creates the data directories and databases and
    /// applies pending migrations. [`Mode::ReadOnly`] requires databases the
    /// daemon has already created and migrated.
    pub fn open(paths: &Paths, mode: Mode) -> Result<Self> {
        if mode == Mode::ReadWrite {
            paths.create_dirs()?;
        }
        // blobs.db first: metadata may reference blobs, never the reverse.
        let blobs = BlobStore::new(
            db::open(&paths.blobs_db(), DbKind::Blobs, mode)?,
            paths.attachments_dir(),
            mode,
        );
        let pim = db::open(&paths.pim_db(), DbKind::Pim, mode)?;
        let mail = db::open(&paths.mail_db(), DbKind::Mail, mode)?;
        Ok(Self {
            mail,
            pim,
            blobs,
            mode,
        })
    }

    /// Whether this store may be written.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// The content-addressed blob store.
    pub fn blobs(&self) -> &BlobStore {
        &self.blobs
    }

    /// Adds an account and returns it with its new ID.
    pub fn add_account(
        &mut self,
        kind: AccountKind,
        display_name: &str,
        address: &str,
    ) -> Result<Account> {
        self.check_writable()?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute(
            "INSERT INTO account (kind, display_name, address) VALUES (?1, ?2, ?3)",
            params![kind.as_str(), display_name, address],
        )?;
        let id = tx.last_insert_rowid();
        journal::record(&tx, ObjectKind::Account, id, ChangeOp::Insert)?;
        tx.commit()?;
        Ok(Account {
            id: AccountId(id),
            kind,
            display_name: display_name.to_owned(),
            address: address.to_owned(),
        })
    }

    /// Renames an account. Returns whether it existed.
    pub fn rename_account(&mut self, id: AccountId, display_name: &str) -> Result<bool> {
        self.check_writable()?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let updated = tx.execute(
            "UPDATE account SET display_name = ?2 WHERE id = ?1",
            params![id.0, display_name],
        )? > 0;
        if updated {
            journal::record(&tx, ObjectKind::Account, id.0, ChangeOp::Update)?;
        }
        tx.commit()?;
        Ok(updated)
    }

    /// The name `account` writes its mail under: the display name its own
    /// messages from `address` most often carry, if any has one.
    pub fn name_in_own_mail(&self, account: AccountId, address: &str) -> Result<Option<String>> {
        people::name_in_own_mail(&self.mail, account, address)
    }

    /// Removes an account. Returns whether it existed.
    ///
    /// Its mail is removed separately by the daemon (Phase 1), because it
    /// lives in `mail.db`.
    pub fn remove_account(&mut self, id: AccountId) -> Result<bool> {
        self.check_writable()?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let removed = tx.execute("DELETE FROM account WHERE id = ?1", [id.0])? > 0;
        if removed {
            journal::record(&tx, ObjectKind::Account, id.0, ChangeOp::Delete)?;
        }
        tx.commit()?;
        Ok(removed)
    }

    /// All accounts, ordered by ID.
    pub fn accounts(&self) -> Result<Vec<Account>> {
        let mut stmt = self
            .pim
            .prepare_cached("SELECT id, kind, display_name, address FROM account ORDER BY id")?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?;
        rows.map(|row| {
            let (id, kind, display_name, address) = row?;
            Ok(Account {
                id: AccountId(id),
                kind: kind
                    .parse()
                    .map_err(|err: katna_core::account::UnknownAccountKind| {
                        Error::InvalidData(err.to_string())
                    })?,
                display_name,
                address,
            })
        })
        .collect()
    }

    /// Server settings of an account, or `None` if it does not exist.
    pub fn account_settings(&self, id: AccountId) -> Result<Option<AccountSettings>> {
        let json: Option<String> = self
            .pim
            .query_row(
                "SELECT settings_json FROM account WHERE id = ?1",
                [id.0],
                |row| row.get(0),
            )
            .optional()?;
        json.map(|json| {
            serde_json::from_str(&json)
                .map_err(|err| Error::InvalidData(format!("settings of account {id}: {err}")))
        })
        .transpose()
    }

    /// Replaces the server settings of an account. Returns whether it exists.
    pub fn set_account_settings(
        &mut self,
        id: AccountId,
        settings: &AccountSettings,
    ) -> Result<bool> {
        self.check_writable()?;
        let json = serde_json::to_string(settings)
            .map_err(|err| Error::InvalidData(format!("settings of account {id}: {err}")))?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let updated = tx.execute(
            "UPDATE account SET settings_json = ?2 WHERE id = ?1",
            params![id.0, json],
        )? > 0;
        if updated {
            journal::record(&tx, ObjectKind::Account, id.0, ChangeOp::Update)?;
        }
        tx.commit()?;
        Ok(updated)
    }

    /// Starts a batch of mail writes (see [`MailBatch`]).
    pub fn mail_batch(&mut self) -> Result<MailBatch<'_>> {
        self.check_writable()?;
        MailBatch::begin(&mut self.mail, &self.blobs)
    }

    /// Messages with ID greater than `after`, in ID order, at most `limit`,
    /// with their participants and folders. Used to index the whole store.
    pub fn messages_after(&self, after: MessageId, limit: u32) -> Result<Vec<StoredMessage>> {
        mail_read::messages_after(&self.mail, after, limit)
    }

    /// The messages with the given IDs, in that order; IDs that no longer
    /// exist are left out.
    pub fn messages_by_id(&self, ids: &[MessageId]) -> Result<Vec<StoredMessage>> {
        mail_read::messages_by_id(&self.mail, ids)
    }

    /// All folders of all accounts, ordered by account and path, with their
    /// message counts.
    pub fn folder_summaries(&self) -> Result<Vec<FolderSummary>> {
        mail_view::folders(&self.mail)
    }

    /// Unread messages per folder, for folders that have any. Slow on big
    /// stores; call it off the UI thread.
    pub fn unread_counts(&self) -> Result<Vec<(FolderId, u64)>> {
        mail_view::unread_counts(&self.mail)
    }

    /// The messages in `folder`, newest first.
    pub fn folder_message_ids(&self, folder: FolderId) -> Result<Vec<MessageId>> {
        mail_view::folder_message_ids(&self.mail, folder)
    }

    /// The messages of `thread` that are in `folder`, oldest first: what
    /// archiving or moving the conversation out of the folder moves.
    pub fn folder_thread_messages(
        &self,
        folder: FolderId,
        thread: ThreadId,
    ) -> Result<Vec<MessageId>> {
        mail_view::folder_thread_messages(&self.mail, folder, thread)
    }

    /// The messages of `folder` in an inbox tab, one of `categories`,
    /// newest first (undated last); unclassified messages count as
    /// [`MailCategory::Primary`]. For the list without conversations.
    pub fn folder_messages_in(
        &self,
        folder: FolderId,
        categories: &[MailCategory],
    ) -> Result<Vec<MessageId>> {
        mail_view::folder_messages_in(&self.mail, folder, categories)
    }

    /// The conversations in `folder`, newest first: one entry per thread,
    /// with its newest message in the folder, ordered by that message's
    /// date (undated last).
    ///
    /// With `categories` (an inbox tab), only conversations whose newest
    /// message in the folder has one of them; unclassified messages count
    /// as [`MailCategory::Primary`]. So each conversation shows in exactly
    /// one tab.
    ///
    /// A message without a thread yet (a store from before threading, until
    /// the daemon's backfill reaches it) is an entry of its own with
    /// `thread: None`.
    pub fn folder_threads(
        &self,
        folder: FolderId,
        categories: Option<&[MailCategory]>,
    ) -> Result<Vec<ThreadEntry>> {
        mail_view::folder_threads(&self.mail, folder, categories)
    }

    /// The conversations with a message in any of `folders` that `filter`
    /// keeps, newest first, each with its newest such message: the lists
    /// of the unified inbox, over the folders of several accounts.
    pub fn spread_threads(
        &self,
        folders: &[FolderId],
        filter: FlagFilter,
    ) -> Result<Vec<ThreadEntry>> {
        mail_view::spread_threads(&self.mail, folders, filter)
    }

    /// The messages in any of `folders` that `filter` keeps, newest first,
    /// server copies of one message once.
    pub fn spread_message_ids(
        &self,
        folders: &[FolderId],
        filter: FlagFilter,
    ) -> Result<Vec<MessageId>> {
        mail_view::spread_message_ids(&self.mail, folders, filter)
    }

    /// The unified inbox over `folders` (each account's inbox), in one tab
    /// ([`SpreadTabs`]), with its unread conversations per tab.
    pub fn spread_inbox_threads(
        &self,
        folders: &[FolderId],
        tabs: &SpreadTabs,
    ) -> Result<InboxThreads> {
        mail_view::spread_inbox_threads(&self.mail, folders, tabs)
    }

    /// The messages of the unified inbox over `folders` in one tab.
    pub fn spread_inbox_message_ids(
        &self,
        folders: &[FolderId],
        tabs: &SpreadTabs,
    ) -> Result<Vec<MessageId>> {
        mail_view::spread_inbox_message_ids(&self.mail, folders, tabs)
    }

    /// `messages` and every other stored copy of them (the same
    /// `Message-ID` in the same account: copies on servers without
    /// Gmail's message IDs, and Gmail mail synced before them), each once,
    /// with its flags. A change the user makes to a message is made to
    /// all of them, since the list shows a conversation starred or unread
    /// when any copy is.
    pub fn with_copies(&self, messages: &[MessageId]) -> Result<Vec<(MessageId, MessageFlags)>> {
        mail_view::with_copies(&self.mail, messages)
    }

    /// The messages of `thread`, oldest first (undated last).
    ///
    /// Server copies of one message (the same `Message-ID`, such as Gmail's
    /// Inbox and All Mail copies) appear once: the copy that is not in
    /// trash or junk, preferably one whose body is stored. Messages only in
    /// trash or junk folders are left out, unless every message of the
    /// thread is there.
    pub fn thread_messages(&self, thread: ThreadId) -> Result<Vec<MessageId>> {
        mail_view::thread_messages(&self.mail, thread)
    }

    /// List summaries of `threads` as seen from `folder`, in the given
    /// order; threads that no longer exist (merged or emptied) are left
    /// out. For entries with `thread: None`, show the message itself.
    pub fn thread_summaries(
        &self,
        threads: &[ThreadId],
        folder: FolderId,
    ) -> Result<Vec<ThreadSummary>> {
        mail_view::thread_summaries(&self.mail, threads, folder)
    }

    /// Whether each message of `folder`, and each conversation with a
    /// message there, is unread and starred; see [`FolderMarks`].
    pub fn folder_marks(&self, folder: FolderId) -> Result<FolderMarks> {
        mail_view::folder_marks(&self.mail, folder)
    }

    /// Unread **conversations** (not messages) in `folder` per tab, for tab
    /// badges: every category in tab order, zeros included. A conversation
    /// counts in the tab of its newest message in the folder, and is unread
    /// when any of its messages in the folder is.
    pub fn category_unread(&self, folder: FolderId) -> Result<Vec<(MailCategory, u64)>> {
        mail_view::category_unread(&self.mail, folder)
    }

    /// [`Store::folder_threads`] and [`Store::category_unread`] of an inbox
    /// together, reading the folder once rather than twice.
    pub fn inbox_threads(
        &self,
        folder: FolderId,
        categories: Option<&[MailCategory]>,
    ) -> Result<InboxThreads> {
        mail_view::inbox_threads(&self.mail, folder, categories)
    }

    /// Number of messages in all accounts.
    /// The `limit` addresses on the most messages; see [`Person`].
    pub fn people(&self, limit: u32) -> Result<Vec<Person>> {
        people::people(&self.mail, limit)
    }

    /// Every address each account has written with; see [`Correspondent`].
    pub fn correspondents(&self) -> Result<Vec<Correspondent>> {
        people::correspondents(&self.mail, &self.accounts()?)
    }

    /// How much mail the user and `email` exchanged; see
    /// [`ContactSummary`].
    pub fn contact_summary(&self, email: &str) -> Result<ContactSummary> {
        contact::summary(&self.mail, &self.accounts()?, email)
    }

    /// The `limit` newest conversations with `email`, newest first.
    pub fn contact_conversations(
        &self,
        email: &str,
        limit: usize,
    ) -> Result<Vec<ContactConversation>> {
        contact::conversations(&self.mail, email, limit)
    }

    /// The `limit` newest named attachments on mail with `email`, each
    /// name and size once.
    pub fn contact_files(&self, email: &str, limit: usize) -> Result<Vec<ContactFile>> {
        contact::files(&self.mail, email, limit)
    }

    /// The `limit` newest named attachments of all accounts, for the
    /// Files page; see [`LibraryFile`].
    pub fn library_files(&self, limit: usize) -> Result<Vec<LibraryFile>> {
        library::files(&self.mail, &self.accounts()?, limit)
    }

    /// Which of the mails `headers` (`Message-ID`s, as tasks keep them)
    /// `email` takes part in, in their conversation.
    pub fn contact_on_mail(&self, email: &str, headers: &[String]) -> Result<Vec<String>> {
        contact::on_mail(&self.mail, email, headers)
    }

    /// The `limit` newest messages from `email` whose body is stored, one
    /// per server copy, newest first.
    pub fn messages_from(&self, email: &str, limit: usize) -> Result<Vec<MessageId>> {
        contact::messages_from(&self.mail, email, limit)
    }

    pub fn message_count(&self) -> Result<u64> {
        mail_read::message_count(&self.mail)
    }

    /// Journal entries of `db` after sequence number `after`, oldest first,
    /// at most `limit`. `db` must be [`DbKind::Mail`] or [`DbKind::Pim`].
    pub fn changes_since(&self, db: DbKind, after: i64, limit: u32) -> Result<Vec<Change>> {
        journal::changes_since(self.journaled(db)?, after, limit)
    }

    /// The newest journal sequence number of `db`, or 0 if it has none.
    pub fn latest_change(&self, db: DbKind) -> Result<i64> {
        journal::latest_seq(self.journaled(db)?)
    }

    fn journaled(&self, db: DbKind) -> Result<&Connection> {
        match db {
            DbKind::Mail => Ok(&self.mail),
            DbKind::Pim => Ok(&self.pim),
            DbKind::Blobs => Err(Error::InvalidData(
                "blobs.db has no change journal".to_owned(),
            )),
        }
    }

    fn check_writable(&self) -> Result<()> {
        match self.mode {
            Mode::ReadWrite => Ok(()),
            Mode::ReadOnly => Err(Error::ReadOnly),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write_creates_everything() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let store = Store::open(&paths, Mode::ReadWrite).unwrap();
        assert_eq!(store.mode(), Mode::ReadWrite);
        for db in [paths.mail_db(), paths.pim_db(), paths.blobs_db()] {
            assert!(db.is_file(), "{}", db.display());
        }
        assert!(paths.attachments_dir().is_dir());
        assert!(store.accounts().unwrap().is_empty());
    }

    #[test]
    fn read_only_before_the_daemon_ran() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        assert!(matches!(
            Store::open(&paths, Mode::ReadOnly),
            Err(Error::NotFound { .. })
        ));
        assert!(!paths.data_dir().exists());
    }

    #[test]
    fn account_settings_round_trip() {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Imap, "Work", "alice@example.org")
            .unwrap();
        assert_eq!(
            store.account_settings(account.id).unwrap(),
            Some(AccountSettings::default())
        );
        let settings = AccountSettings {
            imap: Some(katna_core::Server {
                host: "imap.example.org".into(),
                port: 993,
                security: katna_core::Security::Tls,
                username: "alice".into(),
                accept_invalid_certs: false,
            }),
            ..AccountSettings::default()
        };
        let before = store.latest_change(DbKind::Pim).unwrap();
        assert!(store.set_account_settings(account.id, &settings).unwrap());
        assert_eq!(
            store.account_settings(account.id).unwrap(),
            Some(settings.clone())
        );
        assert!(store.latest_change(DbKind::Pim).unwrap() > before);

        assert!(
            !store
                .set_account_settings(AccountId(99), &settings)
                .unwrap()
        );
        assert_eq!(store.account_settings(AccountId(99)).unwrap(), None);
    }

    #[test]
    fn accounts_are_journaled_and_visible_to_readers() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let mut daemon = Store::open(&paths, Mode::ReadWrite).unwrap();
        let app = Store::open(&paths, Mode::ReadOnly).unwrap();
        let seen = app.latest_change(DbKind::Pim).unwrap();

        let work = daemon
            .add_account(AccountKind::Imap, "Work", "ada@example.org")
            .unwrap();
        let cal = daemon
            .add_account(AccountKind::CalDav, "Calendar", "ada")
            .unwrap();
        assert_ne!(work.id, cal.id);
        assert_eq!(app.accounts().unwrap(), vec![work.clone(), cal.clone()]);

        let changes = app.changes_since(DbKind::Pim, seen, 10).unwrap();
        let summary: Vec<_> = changes
            .iter()
            .map(|c| (c.kind, c.object_id, c.op))
            .collect();
        assert_eq!(
            summary,
            [
                (ObjectKind::Account, work.id.0, ChangeOp::Insert),
                (ObjectKind::Account, cal.id.0, ChangeOp::Insert),
            ]
        );

        assert!(daemon.remove_account(work.id).unwrap());
        assert!(!daemon.remove_account(work.id).unwrap());
        assert_eq!(app.accounts().unwrap(), vec![cal]);
        let last = app.changes_since(DbKind::Pim, changes[1].seq, 10).unwrap();
        assert_eq!(last.len(), 1);
        assert_eq!(last[0].op, ChangeOp::Delete);

        assert!(app.changes_since(DbKind::Blobs, 0, 10).is_err());
        assert_eq!(app.latest_change(DbKind::Mail).unwrap(), 0);
    }

    #[test]
    fn read_only_store_refuses_writes() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        drop(Store::open(&paths, Mode::ReadWrite).unwrap());
        let mut app = Store::open(&paths, Mode::ReadOnly).unwrap();
        assert!(matches!(
            app.add_account(AccountKind::Imap, "x", "x@example.org"),
            Err(Error::ReadOnly)
        ));
        assert!(matches!(app.blobs().put(b"x"), Err(Error::ReadOnly)));
    }

    #[test]
    fn data_survives_reopening() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let (account, hash) = {
            let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
            let account = store
                .add_account(AccountKind::Jmap, "Home", "ada@example.net")
                .unwrap();
            (account, store.blobs().put(b"raw message").unwrap())
        };
        let store = Store::open(&paths, Mode::ReadWrite).unwrap();
        assert_eq!(store.accounts().unwrap(), vec![account]);
        assert_eq!(store.blobs().get(&hash).unwrap().unwrap(), b"raw message");
    }
}
