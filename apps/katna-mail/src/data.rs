// SPDX-License-Identifier: GPL-3.0-or-later

//! The app's read-only view of the store and the search index. No GPUI
//! here. Only `katna-daemon` writes; the app opens both read-only.

use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use katna_core::{Account, AccountId, MailCategory, Paths};
use katna_search::{Query, SearchIndex, SearchOptions, SearchResults};
pub use katna_store::Marks;
use katna_store::{
    FolderId, FolderMarks, FolderSummary, MessageFlags, MessageId, Mode, ParticipantRole, Store,
    StoredMessage, ThreadId, ThreadSender, ThreadSummary,
};

/// At most this many search results are listed.
pub const SEARCH_LIMIT: usize = 1000;

/// How often to try opening a missing search index again.
const INDEX_RETRY: Duration = Duration::from_secs(2);

/// The contacts page lists at most this many people.
pub(crate) const PEOPLE_LIMIT: u32 = 2000;

/// Rows kept in memory; the cache is dropped when it grows past this.
const ROW_CACHE: usize = 5000;

/// What one line of the list stands for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntryKey {
    Message(MessageId),
    Thread(ThreadId),
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

/// One line of the message list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub key: EntryKey,
    /// The message the line shows: the newest of a conversation.
    pub id: MessageId,
    /// Sender, or the recipients in sent and draft folders; for a
    /// conversation, its senders.
    pub correspondent: String,
    /// The address of the message's sender.
    pub sender: String,
    /// Messages in the conversation; 1 for a single message.
    pub count: u32,
    pub subject: String,
    /// Unix seconds.
    pub date: Option<i64>,
    pub unread: bool,
    pub flagged: bool,
    pub important: bool,
    /// Pinned to the top of the list.
    pub pinned: bool,
    pub attachments: bool,
    /// The named attachments, in conversation order, for the chips under
    /// the line. Empty when only `attachments` is known (mail synced
    /// before attachment lists were read, POP3 and imported mail).
    pub files: Vec<RowFile>,
    pub snippet: String,
}

/// An attachment shown on a line of the list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowFile {
    /// The message it is attached to.
    pub message: MessageId,
    pub name: String,
    /// `type/subtype`, lower case.
    pub mime: String,
    /// Decoded size in bytes (estimated).
    pub size: u64,
    /// How many attachments of the same name come before it in its
    /// message, to find it again in the parsed message.
    pub nth: usize,
    /// Its place among the named attachments of its message.
    pub order: usize,
}

/// The chips of a line: named attachments of `messages`, in that order,
/// each name once.
fn row_files(
    messages: &[MessageId],
    lists: &HashMap<MessageId, Vec<katna_store::StoredAttachment>>,
) -> Vec<RowFile> {
    let mut files: Vec<RowFile> = Vec::new();
    for &message in messages {
        let Some(list) = lists.get(&message) else {
            continue;
        };
        let mut seen: HashMap<&str, usize> = HashMap::new();
        let mut order = 0;
        for attachment in list {
            let Some(name) = attachment.filename.as_deref().map(str::trim) else {
                continue;
            };
            if name.is_empty() {
                continue;
            }
            let nth = seen.entry(name).or_default();
            let file = RowFile {
                message,
                name: name.to_owned(),
                mime: attachment.mime.clone(),
                size: attachment.size,
                nth: *nth,
                order,
            };
            *nth += 1;
            order += 1;
            // The same file sent again later in the conversation shows once.
            if !files
                .iter()
                .any(|f| f.name == file.name && f.mime == file.mime)
            {
                files.push(file);
            }
        }
    }
    files
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
        let sender = message
            .first(ParticipantRole::From)
            .or_else(|| message.first(ParticipantRole::Sender))
            .map(|p| p.email_norm.clone())
            .unwrap_or_default();
        let subject = message.subject.trim();
        Self {
            key: EntryKey::Message(message.id),
            id: message.id,
            count: 1,
            correspondent,
            sender,
            subject: if subject.is_empty() {
                "(no subject)".to_owned()
            } else {
                subject.to_owned()
            },
            date: message.date,
            unread: !message.flags.contains(MessageFlags::SEEN),
            flagged: message.flags.contains(MessageFlags::FLAGGED),
            important: message.flags.contains(MessageFlags::IMPORTANT),
            pinned: false,
            attachments: message.has_attachments,
            files: Vec::new(),
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

impl Row {
    /// Makes the row of a message the line of its conversation.
    fn conversation(
        mut self,
        thread: ThreadId,
        summary: Option<&ThreadSummary>,
        me: &[String],
        show_recipients: bool,
    ) -> Self {
        self.key = EntryKey::Thread(thread);
        if let Some(summary) = summary {
            self.count = summary.message_count.max(1);
            self.unread = summary.unread;
            self.flagged = summary.flagged;
            self.important = summary.important;
            self.attachments = summary.has_attachments;
            if !show_recipients && !summary.senders.is_empty() {
                self.correspondent = senders(&summary.senders, me);
            }
        }
        self
    }
}

/// "Kay, Bob, me": the senders of a conversation, first names when there
/// are several, as webmail shows them.
fn senders(list: &[ThreadSender], me: &[String]) -> String {
    let full = |s: &ThreadSender| {
        s.name
            .as_deref()
            .map(str::trim)
            .filter(|n| !n.is_empty())
            .unwrap_or(&s.email)
            .to_owned()
    };
    let is_me = |s: &ThreadSender| me.iter().any(|m| m.eq_ignore_ascii_case(&s.email));
    if let [only] = list {
        return if is_me(only) {
            "me".to_owned()
        } else {
            full(only)
        };
    }
    let short = |s: &ThreadSender| {
        if is_me(s) {
            return "me".to_owned();
        }
        match s.name.as_deref().map(str::trim).filter(|n| !n.is_empty()) {
            Some(name) => name.split_whitespace().next().unwrap_or(name).to_owned(),
            None => s.email.split('@').next().unwrap_or(&s.email).to_owned(),
        }
    };
    let names: Vec<String> = list.iter().map(short).collect();
    if names.len() > 3 {
        format!("{} .. {}", names[0], names[names.len() - 2..].join(", "))
    } else {
        names.join(", ")
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
    /// The accounts' addresses, for "me".
    me: Vec<String>,
    pins: Pins,
}

/// Pinned mail, by how recently it was pinned (0 is the newest pin).
#[derive(Debug, Default)]
struct Pins {
    messages: HashMap<MessageId, usize>,
    threads: HashMap<ThreadId, usize>,
}

impl Pins {
    fn read(store: &Store) -> Self {
        let list = store.pinned().unwrap_or_else(|err| {
            tracing::warn!("reading pinned mail: {err}");
            Vec::new()
        });
        let mut pins = Self::default();
        for (rank, pin) in list.into_iter().enumerate() {
            pins.messages.entry(pin.message).or_insert(rank);
            if let Some(thread) = pin.thread {
                pins.threads.entry(thread).or_insert(rank);
            }
        }
        pins
    }

    /// Where a line goes among the pinned ones, if it is pinned. A
    /// conversation is pinned when one of its messages is.
    fn rank(&self, key: EntryKey) -> Option<usize> {
        match key {
            EntryKey::Message(id) => self.messages.get(&id),
            EntryKey::Thread(thread) => self.threads.get(&thread),
        }
        .copied()
    }
}

/// Moves pinned lines to the top, newest pin first; the rest keep their
/// order.
fn pinned_first(entries: Vec<Entry>, pins: &Pins) -> Vec<Entry> {
    if pins.messages.is_empty() {
        return entries;
    }
    let (mut pinned, rest): (Vec<Entry>, Vec<Entry>) = entries
        .into_iter()
        .partition(|e| pins.rank(e.key).is_some());
    pinned.sort_by_key(|e| pins.rank(e.key));
    pinned.extend(rest);
    pinned
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
        let me = store
            .accounts()
            .map(|accounts| accounts.into_iter().map(|a| a.address).collect())
            .unwrap_or_default();
        Ok(Self {
            me,
            pins: Pins::read(&store),
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

    /// The IMAP server of an account, to tell its provider.
    pub fn incoming_host(&self, account: katna_core::AccountId) -> Option<String> {
        let settings = self.store.account_settings(account).ok()??;
        settings.imap.map(|server| server.host)
    }

    pub fn folders(&self) -> Vec<FolderSummary> {
        self.store.folder_summaries().unwrap_or_else(|err| {
            tracing::warn!("reading folders: {err}");
            Vec::new()
        })
    }

    /// The lines of `folder`, pinned ones first, then newest first:
    /// conversations or messages. `categories` picks an inbox tab.
    pub fn entries(
        &self,
        folder: FolderId,
        categories: Option<&[MailCategory]>,
        conversations: bool,
    ) -> Vec<Entry> {
        let entries = if conversations {
            self.store
                .folder_threads(folder, categories)
                .map(|threads| {
                    threads
                        .into_iter()
                        .map(|entry| match entry.thread {
                            Some(thread) => Entry {
                                key: EntryKey::Thread(thread),
                                latest: entry.latest,
                            },
                            None => Entry::message(entry.latest),
                        })
                        .collect()
                })
        } else {
            match categories {
                Some(categories) => self.store.folder_messages_in(folder, categories),
                None => self.store.folder_message_ids(folder),
            }
            .map(|ids| ids.into_iter().map(Entry::message).collect())
        };
        let entries = entries.unwrap_or_else(|err| {
            tracing::warn!("reading folder {}: {err}", folder.0);
            Vec::new()
        });
        pinned_first(entries, &self.pins)
    }

    /// Search hits as lines: grouped into conversations when asked, each
    /// where its best hit is.
    /// Lines for search hits, of account `only` when given.
    pub fn hit_entries(
        &self,
        hits: &[MessageId],
        conversations: bool,
        only: Option<AccountId>,
    ) -> Vec<Entry> {
        if !conversations && only.is_none() {
            return hits.iter().copied().map(Entry::message).collect();
        }
        let messages = self.store.messages_by_id(hits).unwrap_or_else(|err| {
            tracing::warn!("reading search hits: {err}");
            Vec::new()
        });
        let kept: HashMap<MessageId, Option<ThreadId>> = messages
            .into_iter()
            .filter(|m| only.is_none_or(|a| a == m.account))
            .map(|m| (m.id, m.thread_id))
            .collect();
        let hits = hits
            .iter()
            .filter(|id| only.is_none() || kept.contains_key(id));
        if !conversations {
            return hits.copied().map(Entry::message).collect();
        }
        let threads: HashMap<MessageId, ThreadId> = kept
            .iter()
            .filter_map(|(id, thread)| Some((*id, (*thread)?)))
            .collect();
        let mut seen = std::collections::HashSet::new();
        hits.filter_map(|id| match threads.get(id) {
            Some(thread) => seen.insert(*thread).then_some(Entry {
                key: EntryKey::Thread(*thread),
                latest: *id,
            }),
            None => Some(Entry::message(*id)),
        })
        .collect()
    }

    /// The account of message `id`.
    pub fn message_account(&self, id: MessageId) -> Option<AccountId> {
        self.store
            .messages_by_id(&[id])
            .ok()?
            .first()
            .map(|m| m.account)
    }

    /// The conversation of message `id`, if it has one.
    pub fn message_thread(&self, id: MessageId) -> Option<ThreadId> {
        self.store
            .messages_by_id(&[id])
            .ok()?
            .first()
            .and_then(|m| m.thread_id)
    }

    /// The messages of a line, oldest first: a whole conversation.
    pub fn entry_messages(&self, key: EntryKey) -> Vec<MessageId> {
        match key {
            EntryKey::Message(id) => vec![id],
            EntryKey::Thread(thread) => self.store.thread_messages(thread).unwrap_or_else(|err| {
                tracing::warn!("reading conversation {}: {err}", thread.0);
                Vec::new()
            }),
        }
    }

    /// The folders message `id` is stored in, as the daemon lists them.
    pub fn message_folders(&self, id: MessageId) -> Vec<FolderId> {
        match self.store.locations(id) {
            Ok(locations) => locations.into_iter().map(|l| l.folder).collect(),
            Err(err) => {
                tracing::warn!("reading where message {} is: {err}", id.0);
                Vec::new()
            }
        }
    }

    /// Where Delete puts `account`'s mail; `None` when it deletes for good.
    pub fn trash_folder(&self, account: AccountId) -> Option<FolderId> {
        self.store.trash_folder(account).unwrap_or_else(|err| {
            tracing::warn!("reading the Trash folder: {err}");
            None
        })
    }

    /// `messages` and their other stored copies, with their flags: what a
    /// flag change must touch so the line shows it (see
    /// [`Store::with_copies`](katna_store::Store::with_copies)).
    pub fn with_copies(&self, messages: &[MessageId]) -> Vec<(MessageId, MessageFlags)> {
        self.store.with_copies(messages).unwrap_or_else(|err| {
            tracing::warn!("reading copies of messages: {err}");
            messages
                .iter()
                .map(|&id| (id, MessageFlags::empty()))
                .collect()
        })
    }

    /// The messages of a line that are in `folder`, for moving them out.
    pub fn entry_messages_in(&self, key: EntryKey, folder: FolderId) -> Vec<MessageId> {
        match key {
            EntryKey::Message(id) => vec![id],
            EntryKey::Thread(thread) => self
                .store
                .folder_thread_messages(folder, thread)
                .unwrap_or_else(|err| {
                    tracing::warn!("reading conversation {}: {err}", thread.0);
                    Vec::new()
                }),
        }
    }

    /// Unread conversations per category of an inbox.
    pub fn category_unread(&self, folder: FolderId) -> HashMap<MailCategory, u64> {
        match self.store.category_unread(folder) {
            Ok(counts) => counts.into_iter().collect(),
            Err(err) => {
                tracing::warn!("counting unread conversations: {err}");
                HashMap::new()
            }
        }
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
        self.pins = Pins::read(&self.store);
        if let Some(index) = &self.index
            && let Err(err) = index.reload()
        {
            tracing::warn!("reloading the search index: {err}");
        }
    }

    /// The rows of `entries`, reading the ones not cached yet in one go.
    /// `folder` is the listed folder (`None` for search results), whose
    /// unread state a conversation's line shows. Lines whose mail no longer
    /// exists are `None`.
    pub fn rows(
        &mut self,
        entries: &[Entry],
        folder: Option<FolderId>,
        show_recipients: bool,
    ) -> Vec<Option<Rc<Row>>> {
        let missing: Vec<Entry> = entries
            .iter()
            .filter(|e| !self.rows.contains_key(&e.key))
            .copied()
            .collect();
        if !missing.is_empty() {
            if self.rows.len() + missing.len() > ROW_CACHE {
                self.rows.clear();
            }
            self.read_rows(&missing, folder, show_recipients);
        }
        entries
            .iter()
            .map(|e| self.rows.get(&e.key).cloned())
            .collect()
    }

    fn read_rows(&mut self, entries: &[Entry], folder: Option<FolderId>, show_recipients: bool) {
        let latest: Vec<MessageId> = entries.iter().map(|e| e.latest).collect();
        let messages: HashMap<MessageId, StoredMessage> = match self.store.messages_by_id(&latest) {
            Ok(messages) => messages.into_iter().map(|m| (m.id, m)).collect(),
            Err(err) => {
                tracing::warn!("reading messages: {err}");
                return;
            }
        };
        let threads: Vec<ThreadId> = entries
            .iter()
            .filter_map(|e| match e.key {
                EntryKey::Thread(thread) => Some(thread),
                EntryKey::Message(_) => None,
            })
            .collect();
        let mut summaries: HashMap<ThreadId, ThreadSummary> = HashMap::new();
        if !threads.is_empty() {
            let read = match folder {
                Some(folder) => self.store.thread_summaries(&threads, folder),
                // Search results: each conversation as seen from the folder
                // its hit is in.
                None => Ok(Vec::new()),
            };
            match read {
                Ok(list) => summaries.extend(list.into_iter().map(|s| (s.thread, s))),
                Err(err) => tracing::warn!("reading conversations: {err}"),
            }
        }
        // The messages whose attachments each line shows.
        let mut attached: HashMap<EntryKey, Vec<MessageId>> = HashMap::new();
        for entry in entries {
            let has = match entry.key {
                EntryKey::Thread(thread) if summaries.contains_key(&thread) => {
                    summaries[&thread].has_attachments
                }
                // Search results have no summary: their hit tells.
                _ => messages
                    .get(&entry.latest)
                    .is_some_and(|m| m.has_attachments),
            };
            if !has {
                continue;
            }
            let ids = match entry.key {
                EntryKey::Thread(thread) => self.store.thread_messages(thread).unwrap_or_default(),
                EntryKey::Message(id) => vec![id],
            };
            attached.insert(entry.key, ids);
        }
        let lists = if attached.is_empty() {
            HashMap::new()
        } else {
            let ids: Vec<MessageId> = attached.values().flatten().copied().collect();
            self.store.attachment_lists(&ids).unwrap_or_else(|err| {
                tracing::warn!("reading attachments: {err}");
                HashMap::new()
            })
        };
        for entry in entries {
            let Some(message) = messages.get(&entry.latest) else {
                continue;
            };
            let mut row = Row::new(message, show_recipients);
            if let Some(ids) = attached.get(&entry.key) {
                row.files = row_files(ids, &lists);
            }
            let row = match entry.key {
                EntryKey::Message(_) => row,
                EntryKey::Thread(thread) => {
                    let mut row =
                        row.conversation(thread, summaries.get(&thread), &self.me, show_recipients);
                    if folder.is_none() {
                        row.count = self
                            .store
                            .thread_messages(thread)
                            .map_or(1, |ids| ids.len().max(1) as u32);
                    }
                    row
                }
            };
            let row = Row {
                pinned: self.pins.rank(row.key).is_some(),
                ..row
            };
            self.rows.insert(row.key, Rc::new(row));
        }
    }

    /// Whether each of `entries` is unread and starred, as its row shows
    /// it (before changes the daemon has not confirmed). Reads the whole
    /// folder in two queries, so it stays quick for folders of any size;
    /// search results, which have no folder, read their rows.
    pub fn marks(
        &mut self,
        entries: &[Entry],
        folder: Option<FolderId>,
        show_recipients: bool,
    ) -> Vec<Marks> {
        let known = folder.map(|folder| {
            self.store.folder_marks(folder).unwrap_or_else(|err| {
                tracing::warn!("reading folder {}: {err}", folder.0);
                FolderMarks::default()
            })
        });
        let found = |e: &Entry| {
            let known = known.as_ref()?;
            match e.key {
                EntryKey::Thread(thread) => known.threads.get(&thread).copied(),
                EntryKey::Message(id) => known.messages.get(&id).copied(),
            }
        };
        let mut marks: Vec<Option<Marks>> = entries.iter().map(found).collect();
        // Lines the folder does not hold (none, or a pinned conversation
        // from elsewhere): their rows tell, in pages to keep the cache.
        let missing: Vec<usize> = (0..entries.len()).filter(|&i| marks[i].is_none()).collect();
        for page in missing.chunks(ROW_CACHE / 2) {
            let page_entries: Vec<Entry> = page.iter().map(|&i| entries[i]).collect();
            let rows = self.rows(&page_entries, folder, show_recipients);
            for (&i, row) in page.iter().zip(rows) {
                marks[i] = row.map(|r| Marks {
                    unread: r.unread,
                    flagged: r.flagged,
                });
            }
        }
        marks.into_iter().map(Option::unwrap_or_default).collect()
    }

    /// Rows of single messages (the parts of a conversation).
    pub fn message_rows(&mut self, ids: &[MessageId]) -> Vec<Option<Rc<Row>>> {
        let entries: Vec<Entry> = ids.iter().copied().map(Entry::message).collect();
        self.rows(&entries, None, false)
    }

    /// Forgets cached rows, for example when the sender/recipient column
    /// changes.
    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    /// The account of message `id`.
    pub fn account_of(&self, id: MessageId) -> Option<AccountId> {
        Some(self.store.messages_by_id(&[id]).ok()?.pop()?.account)
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

/// The address book for recipient suggestions, read from the store (a
/// few seconds on a big mailbox). Opens its own connection, for a
/// background thread.
pub fn address_book(paths: &Paths) -> Result<katna_search::contacts::ContactBook, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.correspondents())
        .map(katna_search::contacts::ContactBook::new)
        .map_err(|err| format!("Reading addresses from the mail failed: {err}"))
}

/// Where the address book is kept between runs, so suggestions work at
/// once while it is read again.
fn address_book_file(paths: &Paths) -> PathBuf {
    paths.cache_dir().join("addresses.json")
}

/// The address book saved by [`save_address_book`], if any.
pub fn cached_address_book(paths: &Paths) -> Option<katna_search::contacts::ContactBook> {
    let bytes = std::fs::read(address_book_file(paths)).ok()?;
    let contacts = serde_json::from_slice(&bytes)
        .inspect_err(|err| tracing::warn!("reading the saved address book: {err}"))
        .ok()?;
    Some(katna_search::contacts::ContactBook::from_contacts(contacts))
}

/// Saves the address book for the next run, readable only by the user.
pub fn save_address_book(paths: &Paths, book: &katna_search::contacts::ContactBook) {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    let file = address_book_file(paths);
    let partial = file.with_extension("json.part");
    let saved = serde_json::to_vec(book.contacts())
        .map_err(std::io::Error::other)
        .and_then(|bytes| {
            std::fs::create_dir_all(paths.cache_dir())?;
            std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(&partial)?
                .write_all(&bytes)?;
            std::fs::rename(&partial, &file)
        });
    if let Err(err) = saved {
        tracing::warn!("saving the address book: {err}");
    }
}

/// Runs a search typed into the search box. With `correct`, a text with a
/// word that is not in the mail is corrected to the nearest words that are,
/// as a web search does, and the corrected text is returned with the
/// results if it found any. `now` is Unix seconds, for relative dates such
/// as `newer_than:`.
pub fn search(
    index: &SearchIndex,
    text: &str,
    now: i64,
    correct: bool,
) -> Result<(SearchResults, Option<String>), String> {
    let run = |text: &str| {
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
    };
    if correct && let Some(corrected) = index.suggest(text, true).map_err(|err| err.to_string())? {
        let results = run(&corrected)?;
        if !results.hits.is_empty() {
            return Ok((results, Some(corrected)));
        }
    }
    Ok((run(text)?, None))
}

/// Which of `copies` setting `flag` to `on` changes: the ones not in that
/// state yet. Undo sets the opposite on exactly these, so it restores the
/// copies as they were.
pub fn flag_changes(
    copies: &[(MessageId, MessageFlags)],
    flag: MessageFlags,
    on: bool,
) -> Vec<MessageId> {
    copies
        .iter()
        .filter(|(_, flags)| flags.contains(flag) != on)
        .map(|(id, _)| *id)
        .collect()
}

#[cfg(test)]
mod tests {
    use katna_core::AccountKind;
    use katna_store::{Added, NewMessage, NewParticipant};

    use super::*;

    const RAW: &[u8] = b"From: Ada <ada@example.org>\r\nTo: bob@example.net\r\n\
Subject: Budget\r\nDate: Mon, 14 May 2001 16:39:00 +0000\r\n\r\nThe budget is final.\r\n";

    #[test]
    fn flag_changes_touch_only_what_changes() {
        let copies = [
            (MessageId(1), MessageFlags::FLAGGED | MessageFlags::SEEN),
            (MessageId(2), MessageFlags::SEEN),
            (MessageId(3), MessageFlags::FLAGGED),
        ];
        // Unstarring clears every starred copy; undo stars those again.
        assert_eq!(
            flag_changes(&copies, MessageFlags::FLAGGED, false),
            [MessageId(1), MessageId(3)]
        );
        assert_eq!(
            flag_changes(&copies, MessageFlags::FLAGGED, true),
            [MessageId(2)]
        );
        assert_eq!(
            flag_changes(&copies, MessageFlags::SEEN, false),
            [MessageId(1), MessageId(2)]
        );
    }

    #[test]
    fn pinned_lines_go_first() {
        let line = |n| Entry::message(MessageId(n));
        let thread = Entry {
            key: EntryKey::Thread(ThreadId(7)),
            latest: MessageId(4),
        };
        let pins = Pins {
            messages: HashMap::from([(MessageId(3), 1), (MessageId(9), 0)]),
            threads: HashMap::from([(ThreadId(7), 0)]),
        };
        let entries = vec![line(1), line(2), thread, line(3), line(5)];
        assert_eq!(
            pinned_first(entries, &pins),
            [thread, line(3), line(1), line(2), line(5)]
        );
    }

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
                    in_reply_to: None,
                    references: &[],
                    category: None,
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
    fn line_attachments() {
        let file = |part: &str, mime: &str, name: Option<&str>| katna_store::StoredAttachment {
            part: part.into(),
            mime: mime.into(),
            filename: name.map(Into::into),
            size: 10,
        };
        let (first, reply) = (MessageId(1), MessageId(2));
        let lists = HashMap::from([
            (
                first,
                vec![
                    file("2", "application/pdf", Some("a.pdf")),
                    file("3", "image/png", None),
                    file("4", "application/pdf", Some("a.pdf")),
                ],
            ),
            (
                reply,
                vec![
                    file("2", "application/pdf", Some("a.pdf")),
                    file("3", "image/jpeg", Some(" photo.jpg ")),
                ],
            ),
        ]);
        let files: Vec<_> = row_files(&[first, reply, MessageId(3)], &lists)
            .into_iter()
            .map(|f| (f.message, f.name, f.nth))
            .collect();
        assert_eq!(
            files,
            [
                (first, "a.pdf".to_owned(), 0),
                (reply, "photo.jpg".to_owned(), 0)
            ],
            "unnamed parts are left out, and a name shows once"
        );
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

        let rows = mail.rows(
            &[Entry::message(id), Entry::message(MessageId(999))],
            None,
            false,
        );
        assert_eq!(
            **rows[0].as_ref().unwrap(),
            Row {
                key: EntryKey::Message(id),
                id,
                count: 1,
                correspondent: "Ada".into(),
                sender: "ada@example.org".into(),
                subject: "Budget".into(),
                date: Some(989_858_340),
                unread: true,
                flagged: true,
                important: false,
                pinned: false,
                attachments: false,
                files: Vec::new(),
                snippet: "The budget is final.".into(),
            }
        );
        assert_eq!(rows[1], None);
        mail.clear_rows();
        let sent = mail.rows(&[Entry::message(id)], None, true);
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
        let (results, corrected) = search(&index, "budg", 0, true).unwrap();
        assert_eq!(corrected, None);
        assert_eq!(results.hits.len(), 1);
        assert_eq!(results.hits[0].message, id);
        assert_eq!(results.total, Some(1));
        assert!(
            search(&index, "from:nobody", 0, true)
                .unwrap()
                .0
                .hits
                .is_empty()
        );
        let (results, corrected) = search(&index, "budgte fnial", 0, true).unwrap();
        assert_eq!(corrected.as_deref(), Some("budget final"));
        assert_eq!(results.hits.len(), 1);
        let (results, corrected) = search(&index, "budgte fnial", 0, false).unwrap();
        assert_eq!(corrected, None);
        // Searched as typed: only near matches, if any.
        assert!(results.fuzzy);
    }
}
