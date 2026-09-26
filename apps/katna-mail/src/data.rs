// SPDX-License-Identifier: GPL-3.0-or-later

//! The app's read-only view of the store and the search index. No GPUI
//! here. Only `katna-daemon` writes; the app opens both read-only.

use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use katna_core::{Account, Paths};
use katna_search::{Query, SearchIndex, SearchOptions, SearchResults};
use katna_store::{
    FolderId, FolderSummary, MessageFlags, MessageId, Mode, ParticipantRole, Store, StoredMessage,
};

/// At most this many search results are listed.
pub const SEARCH_LIMIT: usize = 1000;

/// How often to try opening a missing search index again.
const INDEX_RETRY: Duration = Duration::from_secs(2);

/// The contacts page lists at most this many people.
const PEOPLE_LIMIT: u32 = 2000;

/// Rows kept in memory; the cache is dropped when it grows past this.
const ROW_CACHE: usize = 5000;

/// What one line of the list stands for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntryKey {
    Message(MessageId),
}

/// One line of the list: a message, or a conversation shown by its newest
/// message in the folder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    pub key: EntryKey,
    pub latest: MessageId,
}

impl Entry {
    pub fn message(id: MessageId) -> Self {
        Self {
            key: EntryKey::Message(id),
            latest: id,
        }
    }
}

/// An inbox category tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Primary,
    Promotions,
    Social,
    Updates,
    Forums,
}

impl Category {
    pub const ALL: [Self; 5] = [
        Self::Primary,
        Self::Promotions,
        Self::Social,
        Self::Updates,
        Self::Forums,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Primary => "Primary",
            Self::Promotions => "Promotions",
            Self::Social => "Social",
            Self::Updates => "Updates",
            Self::Forums => "Forums",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Primary => "inbox",
            Self::Promotions => "tag",
            Self::Social => "people",
            Self::Updates => "info",
            Self::Forums => "forum",
        }
    }

    pub fn index(self) -> usize {
        Self::ALL.iter().position(|c| *c == self).unwrap_or(0)
    }
}

/// One line of the message list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub key: EntryKey,
    /// The message the line shows: the newest of a conversation.
    pub id: MessageId,
    /// Sender, or the recipients in sent and draft folders; for a
    /// conversation, its senders.
    pub correspondent: String,
    /// Messages in the conversation; 1 for a single message.
    pub count: u32,
    pub subject: String,
    /// Unix seconds.
    pub date: Option<i64>,
    pub unread: bool,
    pub flagged: bool,
    pub attachments: bool,
    pub snippet: String,
}

impl Row {
    pub fn new(message: &StoredMessage, show_recipients: bool) -> Self {
        let name = |p: &katna_store::StoredParticipant| {
            p.display_name
                .as_deref()
                .map(str::trim)
                .filter(|n| !n.is_empty())
                .unwrap_or(&p.email_norm)
                .to_owned()
        };
        let correspondent = if show_recipients {
            let to: Vec<String> = message
                .participants
                .iter()
                .filter(|p| matches!(p.role, ParticipantRole::To | ParticipantRole::Cc))
                .map(name)
                .collect();
            if to.is_empty() {
                "(no recipients)".to_owned()
            } else {
                format!("To: {}", to.join(", "))
            }
        } else {
            message
                .first(ParticipantRole::From)
                .or_else(|| message.first(ParticipantRole::Sender))
                .map(name)
                .unwrap_or_else(|| "(unknown sender)".to_owned())
        };
        let subject = message.subject.trim();
        Self {
            key: EntryKey::Message(message.id),
            id: message.id,
            count: 1,
            correspondent,
            subject: if subject.is_empty() {
                "(no subject)".to_owned()
            } else {
                subject.to_owned()
            },
            date: message.date,
            unread: !message.flags.contains(MessageFlags::SEEN),
            flagged: message.flags.contains(MessageFlags::FLAGGED),
            attachments: message.has_attachments,
            snippet: message
                .snippet
                .as_deref()
                .unwrap_or_default()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" "),
        }
    }
}

/// Why the store could not be opened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenError {
    /// The daemon has not created the databases yet.
    NoStore {
        data_dir: String,
    },
    Other(String),
}

/// The open store and, if it exists, the search index.
pub struct Mail {
    store: Store,
    index_dir: PathBuf,
    index: Option<Arc<SearchIndex>>,
    index_error: Option<String>,
    index_tried: Instant,
    rows: HashMap<EntryKey, Rc<Row>>,
}

impl Mail {
    pub fn open(paths: &Paths) -> Result<Self, OpenError> {
        let store = Store::open(paths, Mode::ReadOnly).map_err(|err| match err {
            katna_store::Error::NotFound { .. } => OpenError::NoStore {
                data_dir: paths.data_dir().display().to_string(),
            },
            err => OpenError::Other(err.to_string()),
        })?;
        let index_dir = paths.index_dir();
        let (index, index_error) = open_index(&index_dir);
        Ok(Self {
            store,
            index_dir,
            index,
            index_error,
            index_tried: Instant::now(),
            rows: HashMap::new(),
        })
    }

    pub fn accounts(&self) -> Vec<Account> {
        self.store.accounts().unwrap_or_else(|err| {
            tracing::warn!("reading accounts: {err}");
            Vec::new()
        })
    }

    pub fn folders(&self) -> Vec<FolderSummary> {
        self.store.folder_summaries().unwrap_or_else(|err| {
            tracing::warn!("reading folders: {err}");
            Vec::new()
        })
    }

    /// The lines of `folder`, newest first: conversations or messages.
    /// `category` picks an inbox tab.
    pub fn entries(
        &self,
        folder: FolderId,
        _category: Option<Category>,
        _conversations: bool,
    ) -> Vec<Entry> {
        match self.store.folder_message_ids(folder) {
            Ok(ids) => ids.into_iter().map(Entry::message).collect(),
            Err(err) => {
                tracing::warn!("reading folder {}: {err}", folder.0);
                Vec::new()
            }
        }
    }

    /// Search hits as lines: grouped into conversations when asked.
    pub fn hit_entries(&self, hits: &[MessageId], _conversations: bool) -> Vec<Entry> {
        hits.iter().copied().map(Entry::message).collect()
    }

    /// The messages of a line, oldest first: a whole conversation.
    pub fn entry_messages(&self, key: EntryKey) -> Vec<MessageId> {
        match key {
            EntryKey::Message(id) => vec![id],
        }
    }

    /// The messages of a line that are in `folder`, for moving them out.
    pub fn entry_messages_in(&self, key: EntryKey, folder: FolderId) -> Vec<MessageId> {
        let _ = folder;
        self.entry_messages(key)
    }

    /// Unread lines per inbox tab.
    pub fn category_unread(&self, _folder: FolderId) -> HashMap<Category, u64> {
        HashMap::new()
    }

    /// The search index, shared with background searches. The daemon
    /// builds it; until it exists (or while the daemon rebuilds it after an
    /// upgrade), opening it is retried every few seconds.
    pub fn index(&mut self) -> Option<Arc<SearchIndex>> {
        if self.index.is_none() && self.index_tried.elapsed() >= INDEX_RETRY {
            (self.index, self.index_error) = open_index(&self.index_dir);
            self.index_tried = Instant::now();
        }
        self.index.clone()
    }

    /// Whether the search index is open.
    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    /// Why search is unavailable, if it is.
    pub fn index_error(&self) -> Option<&str> {
        self.index_error.as_deref()
    }

    /// Picks up what the daemon wrote since the last call.
    pub fn refresh(&mut self) {
        self.rows.clear();
        if let Some(index) = &self.index
            && let Err(err) = index.reload()
        {
            tracing::warn!("reloading the search index: {err}");
        }
    }

    /// The rows of `entries`, reading the ones not cached yet in one go.
    /// Lines whose mail no longer exists are `None`.
    pub fn rows(&mut self, entries: &[Entry], show_recipients: bool) -> Vec<Option<Rc<Row>>> {
        let missing: Vec<MessageId> = entries
            .iter()
            .filter(|e| !self.rows.contains_key(&e.key))
            .map(|e| e.latest)
            .collect();
        if !missing.is_empty() {
            if self.rows.len() + missing.len() > ROW_CACHE {
                self.rows.clear();
            }
            match self.store.messages_by_id(&missing) {
                Ok(messages) => {
                    for message in &messages {
                        let row = Row::new(message, show_recipients);
                        self.rows.insert(row.key, Rc::new(row));
                    }
                }
                Err(err) => tracing::warn!("reading messages: {err}"),
            }
        }
        entries
            .iter()
            .map(|e| self.rows.get(&e.key).cloned())
            .collect()
    }

    /// Rows of single messages (the parts of a conversation).
    pub fn message_rows(&mut self, ids: &[MessageId]) -> Vec<Option<Rc<Row>>> {
        let entries: Vec<Entry> = ids.iter().copied().map(Entry::message).collect();
        self.rows(&entries, false)
    }

    /// Forgets cached rows, for example when the sender/recipient column
    /// changes.
    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    /// The raw message `id`, if its body is stored.
    pub fn raw(&self, id: MessageId) -> Option<Vec<u8>> {
        let message = self.store.messages_by_id(&[id]).ok()?.pop()?;
        match self.store.blobs().get(&message.blob_hash?) {
            Ok(raw) => raw,
            Err(err) => {
                tracing::warn!("reading message {}: {err}", id.0);
                None
            }
        }
    }
}

fn open_index(dir: &std::path::Path) -> (Option<Arc<SearchIndex>>, Option<String>) {
    match SearchIndex::open_read_only(dir) {
        Ok(index) => (Some(Arc::new(index)), None),
        Err(katna_search::Error::NotFound(_)) => (
            None,
            Some("Search is not ready: the index has not been built yet.".to_owned()),
        ),
        Err(err) => (None, Some(format!("Search is not ready: {err}"))),
    }
}

/// Unread messages per folder. Opens its own connection, so it can run on
/// a background thread while the UI uses [`Mail`].
pub fn unread_counts(paths: &Paths) -> HashMap<FolderId, u64> {
    let counts = Store::open(paths, Mode::ReadOnly).and_then(|store| store.unread_counts());
    match counts {
        Ok(counts) => counts.into_iter().collect(),
        Err(err) => {
            tracing::warn!("counting unread mail: {err}");
            HashMap::new()
        }
    }
}

/// The people in the mail, most written with first. Opens its own
/// connection, for a background thread.
pub fn people(paths: &Paths) -> Result<Vec<katna_store::Person>, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.people(PEOPLE_LIMIT))
        .map_err(|err| format!("Reading people from the mail failed: {err}"))
}

/// Runs a search typed into the search box. `now` is Unix seconds, for
/// relative dates such as `newer_than:`.
pub fn search(index: &SearchIndex, text: &str, now: i64) -> Result<SearchResults, String> {
    let query = Query::parse_as_you_type(text, now).map_err(|err| err.to_string())?;
    index
        .search(
            &query,
            &SearchOptions {
                limit: SEARCH_LIMIT,
                count: true,
                ..SearchOptions::default()
            },
        )
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use katna_core::AccountKind;
    use katna_store::{Added, NewMessage, NewParticipant};

    use super::*;

    const RAW: &[u8] = b"From: Ada <ada@example.org>\r\nTo: bob@example.net\r\n\
Subject: Budget\r\nDate: Mon, 14 May 2001 16:39:00 +0000\r\n\r\nThe budget is final.\r\n";

    fn store_with_mail(paths: &Paths) -> (FolderId, MessageId) {
        let mut store = Store::open(paths, Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Local, "Enron", "enron")
            .unwrap()
            .id;
        let participants = [
            NewParticipant {
                role: ParticipantRole::From,
                email_norm: "ada@example.org",
                domain: "example.org",
                display_name: Some("Ada"),
            },
            NewParticipant {
                role: ParticipantRole::To,
                email_norm: "bob@example.net",
                domain: "example.net",
                display_name: None,
            },
        ];
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch.ensure_folder(account, "INBOX").unwrap();
        let Added::Message(id) = batch
            .add_message(
                account,
                inbox,
                &NewMessage {
                    raw: RAW,
                    message_id_hdr: None,
                    subject: Some(" Budget "),
                    date: Some(989_858_340),
                    flags: MessageFlags::FLAGGED,
                    has_attachments: false,
                    list_id: None,
                    snippet: Some("The budget\r\n is final."),
                    participants: &participants,
                },
            )
            .unwrap()
        else {
            panic!("expected a new message");
        };
        batch.commit().unwrap();
        (inbox, id)
    }

    #[test]
    fn no_store_yet() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        assert!(matches!(Mail::open(&paths), Err(OpenError::NoStore { .. })));
    }

    #[test]
    fn folders_rows_and_bodies() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let (inbox, id) = store_with_mail(&paths);

        let mut mail = Mail::open(&paths).unwrap();
        assert!(mail.index().is_none() && !mail.has_index());
        assert!(mail.index_error().is_some());
        assert_eq!(mail.accounts()[0].display_name, "Enron");
        assert_eq!(mail.folders()[0].total, 1);
        let entries = mail.entries(inbox, None, false);
        assert_eq!(entries, [Entry::message(id)]);

        let rows = mail.rows(&[Entry::message(id), Entry::message(MessageId(999))], false);
        assert_eq!(
            **rows[0].as_ref().unwrap(),
            Row {
                key: EntryKey::Message(id),
                id,
                count: 1,
                correspondent: "Ada".into(),
                subject: "Budget".into(),
                date: Some(989_858_340),
                unread: true,
                flagged: true,
                attachments: false,
                snippet: "The budget is final.".into(),
            }
        );
        assert_eq!(rows[1], None);
        mail.clear_rows();
        let sent = mail.rows(&[Entry::message(id)], true);
        assert_eq!(
            sent[0].as_ref().unwrap().correspondent,
            "To: bob@example.net"
        );

        assert_eq!(unread_counts(&paths), HashMap::from([(inbox, 1)]));
        assert_eq!(mail.raw(id).as_deref(), Some(RAW));
        assert_eq!(mail.raw(MessageId(999)), None);
    }

    #[test]
    fn search_through_the_index() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let (_, id) = store_with_mail(&paths);
        {
            let store = Store::open(&paths, Mode::ReadOnly).unwrap();
            let index = SearchIndex::open(&paths.index_dir()).unwrap();
            index
                .update(&store, &katna_search::IndexOptions::default(), |_| {})
                .unwrap();
        }
        let mut mail = Mail::open(&paths).unwrap();
        let index = mail.index().expect("index opened");
        let results = search(&index, "budg", 0).unwrap();
        assert_eq!(results.hits.len(), 1);
        assert_eq!(results.hits[0].message, id);
        assert_eq!(results.total, Some(1));
        assert!(search(&index, "from:nobody", 0).unwrap().hits.is_empty());
    }
}
