// SPDX-License-Identifier: GPL-3.0-or-later

//! The app's read-only view of the store and the search index. No GPUI
//! here. Only `katna-daemon` writes; the app opens both read-only.

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use katna_core::{Account, Paths};
use katna_search::{Query, SearchIndex, SearchOptions, SearchResults};
use katna_store::{
    FolderId, FolderSummary, MessageFlags, MessageId, Mode, ParticipantRole, Store, StoredMessage,
};

/// At most this many search results are listed.
pub const SEARCH_LIMIT: usize = 1000;

/// Rows kept in memory; the cache is dropped when it grows past this.
const ROW_CACHE: usize = 5000;

/// One line of the message list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub id: MessageId,
    /// Sender, or the recipients in sent and draft folders.
    pub correspondent: String,
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
            id: message.id,
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
    index: Option<Arc<SearchIndex>>,
    index_error: Option<String>,
    rows: HashMap<MessageId, Rc<Row>>,
}

impl Mail {
    pub fn open(paths: &Paths) -> Result<Self, OpenError> {
        let store = Store::open(paths, Mode::ReadOnly).map_err(|err| match err {
            katna_store::Error::NotFound { .. } => OpenError::NoStore {
                data_dir: paths.data_dir().display().to_string(),
            },
            err => OpenError::Other(err.to_string()),
        })?;
        let (index, index_error) = if paths.index_dir().join("meta.json").is_file() {
            match SearchIndex::open_read_only(&paths.index_dir()) {
                Ok(index) => (Some(Arc::new(index)), None),
                Err(err) => (None, Some(err.to_string())),
            }
        } else {
            (
                None,
                Some("The search index has not been built yet.".to_owned()),
            )
        };
        Ok(Self {
            store,
            index,
            index_error,
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
        self.store.folders().unwrap_or_else(|err| {
            tracing::warn!("reading folders: {err}");
            Vec::new()
        })
    }

    pub fn folder_message_ids(&self, folder: FolderId) -> Vec<MessageId> {
        self.store.folder_message_ids(folder).unwrap_or_else(|err| {
            tracing::warn!("reading folder {}: {err}", folder.0);
            Vec::new()
        })
    }

    /// The search index, shared with background searches.
    pub fn index(&self) -> Option<Arc<SearchIndex>> {
        self.index.clone()
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

    /// The rows of `ids`, reading the ones not cached yet in one go.
    /// Messages that no longer exist are `None`.
    pub fn rows(&mut self, ids: &[MessageId], show_recipients: bool) -> Vec<Option<Rc<Row>>> {
        let missing: Vec<MessageId> = ids
            .iter()
            .copied()
            .filter(|id| !self.rows.contains_key(id))
            .collect();
        if !missing.is_empty() {
            if self.rows.len() + missing.len() > ROW_CACHE {
                self.rows.clear();
            }
            match self.store.messages_by_id(&missing) {
                Ok(messages) => {
                    for message in &messages {
                        self.rows
                            .insert(message.id, Rc::new(Row::new(message, show_recipients)));
                    }
                }
                Err(err) => tracing::warn!("reading messages: {err}"),
            }
        }
        ids.iter().map(|id| self.rows.get(id).cloned()).collect()
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
        assert!(mail.index().is_none());
        assert!(mail.index_error().is_some());
        assert_eq!(mail.accounts()[0].display_name, "Enron");
        assert_eq!(mail.folders()[0].total, 1);
        let ids = mail.folder_message_ids(inbox);
        assert_eq!(ids, [id]);

        let rows = mail.rows(&[id, MessageId(999)], false);
        assert_eq!(
            **rows[0].as_ref().unwrap(),
            Row {
                id,
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
        let sent = mail.rows(&[id], true);
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
        let mail = Mail::open(&paths).unwrap();
        let index = mail.index().expect("index opened");
        let results = search(&index, "budg", 0).unwrap();
        assert_eq!(results.hits.len(), 1);
        assert_eq!(results.hits[0].message, id);
        assert_eq!(results.total, Some(1));
        assert!(search(&index, "from:nobody", 0).unwrap().hits.is_empty());
    }
}
