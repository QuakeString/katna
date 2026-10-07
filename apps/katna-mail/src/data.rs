// SPDX-License-Identifier: GPL-3.0-or-later

//! The app's read-only view of the store and the search index. No GPUI
//! here. Only `katna-daemon` writes; the app opens both read-only.

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use katna_core::{Account, AccountId, MailCategory, Paths};
use katna_search::{Query, SearchIndex, SearchOptions, SearchResults};
pub use katna_store::Marks;
use katna_store::{
    Bell, FlagFilter, FolderId, FolderMarks, FolderSummary, InboxThreads, MessageFlags, MessageId,
    Mode, Mute, MuteTarget, ParticipantRole, SpreadTabs, Store, StoredMessage, ThreadId,
    ThreadSender, ThreadSummary,
};

mod preload;

pub use preload::{ListRead, Preload, Preloading, remember as remember_first_list};

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
    /// The account the message is in, marked on lines of the unified
    /// inbox.
    pub account: AccountId,
    /// Sender, or the recipients in sent and draft folders; for a
    /// conversation, its senders.
    pub correspondent: String,
    /// `correspondent` in pieces: each person's name with their address,
    /// and the text between (an empty list: show it whole). Lets the line
    /// mark a muted sender beside their name.
    pub people: Vec<(String, Option<String>)>,
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
    /// Snoozed: when it comes back (Unix seconds), shown in place of the
    /// date.
    pub snoozed_until: Option<i64>,
    /// A follow-up waits on the user's message in it: its chip.
    pub follow_up: Option<LineFollowUp>,
    pub attachments: bool,
    /// The named attachments, in conversation order, for the chips under
    /// the line. Empty when only `attachments` is known (mail synced
    /// before attachment lists were read, POP3 and imported mail).
    pub files: Vec<RowFile>,
    pub snippet: String,
    /// For mail sent with open and click tracking, what its recipients did.
    pub tracking: Option<Tracked>,
    /// The user answered: they wrote the conversation's newest mail, or
    /// the newest mail they got is marked answered (a reply whose sent
    /// copy is not here). Never set in sent and draft folders.
    pub replied: bool,
}

/// A follow-up or "remind me if no reply" on mail the user sent, for the
/// chip on its line and the card on the open conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineFollowUp {
    /// The sent message's outbox entry, which the daemon keys it on.
    pub outbox: i64,
    /// When it is due (Unix seconds): for one Katna sends, the working
    /// time it goes out.
    pub at: i64,
    /// Katna sends a follow-up, rather than remind.
    pub sends: bool,
    /// The follow-up due next (1 or 2) of how many.
    pub step: usize,
    pub steps: usize,
    /// It fell due while the computer was off and waits for the user.
    pub waiting: bool,
}

impl LineFollowUp {
    fn of(outbox: i64, follow_up: &katna_meta::FollowUp, tz: &jiff::tz::TimeZone) -> Self {
        let sends = follow_up.sends();
        let at = if sends && !follow_up.waiting {
            katna_meta::working_time(follow_up.remind_at, tz)
        } else {
            follow_up.remind_at
        };
        Self {
            outbox,
            at,
            sends,
            step: follow_up.sent.len() + 1,
            steps: if sends && follow_up.again > 0 { 2 } else { 1 },
            waiting: follow_up.waiting,
        }
    }
}

/// What the recipients of a tracked message did, for its line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tracked {
    pub recipients: usize,
    pub opened: usize,
    pub clicked: usize,
}

impl From<&katna_store::MessageActivity> for Tracked {
    fn from(activity: &katna_store::MessageActivity) -> Self {
        Self {
            recipients: activity.recipients.len(),
            opened: activity.opened(),
            clicked: activity.clicked(),
        }
    }
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
        let people: Vec<(String, Option<String>)> = if show_recipients {
            let to: Vec<(String, Option<String>)> = message
                .participants
                .iter()
                .filter(|p| matches!(p.role, ParticipantRole::To | ParticipantRole::Cc))
                .map(|p| (name(p), Some(p.email_norm.clone())))
                .collect();
            if to.is_empty() {
                vec![("(no recipients)".to_owned(), None)]
            } else {
                std::iter::once(("To: ".to_owned(), None))
                    .chain(between(to, ", "))
                    .collect()
            }
        } else {
            vec![match message
                .first(ParticipantRole::From)
                .or_else(|| message.first(ParticipantRole::Sender))
            {
                Some(p) => (name(p), Some(p.email_norm.clone())),
                None => ("(unknown sender)".to_owned(), None),
            }]
        };
        let correspondent = joined(&people);
        let sender = message
            .first(ParticipantRole::From)
            .or_else(|| message.first(ParticipantRole::Sender))
            .map(|p| p.email_norm.clone())
            .unwrap_or_default();
        let subject = message.subject.trim();
        Self {
            key: EntryKey::Message(message.id),
            id: message.id,
            account: message.account,
            count: 1,
            correspondent,
            people,
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
            snoozed_until: None,
            follow_up: None,
            attachments: message.has_attachments,
            files: Vec::new(),
            tracking: None,
            replied: !show_recipients && message.flags.contains(MessageFlags::ANSWERED),
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
                self.people = senders(&summary.senders, me);
                self.correspondent = joined(&self.people);
            }
            self.replied = !show_recipients
                && (summary.last_answered
                    || summary
                        .last_from
                        .iter()
                        .any(|a| me.iter().any(|m| m.eq_ignore_ascii_case(a))));
        }
        self
    }
}

/// The text of a line's `people`.
fn joined(people: &[(String, Option<String>)]) -> String {
    people.iter().map(|(text, _)| text.as_str()).collect()
}

/// `names` with `separator` between them.
fn between(
    names: Vec<(String, Option<String>)>,
    separator: &str,
) -> impl Iterator<Item = (String, Option<String>)> {
    let separator = separator.to_owned();
    names.into_iter().enumerate().flat_map(move |(ix, name)| {
        (ix > 0)
            .then(|| (separator.clone(), None))
            .into_iter()
            .chain(std::iter::once(name))
    })
}

/// "Kay, Bob, me": the senders of a conversation, first names when there
/// are several, as webmail shows them; each name with its address.
fn senders(list: &[ThreadSender], me: &[String]) -> Vec<(String, Option<String>)> {
    let full = |s: &ThreadSender| {
        s.name
            .as_deref()
            .map(str::trim)
            .filter(|n| !n.is_empty())
            .unwrap_or(&s.email)
            .to_owned()
    };
    let is_me = |s: &ThreadSender| me.iter().any(|m| m.eq_ignore_ascii_case(&s.email));
    let email = |s: &ThreadSender| Some(s.email.clone());
    if let [only] = list {
        return vec![if is_me(only) {
            ("me".to_owned(), None)
        } else {
            (full(only), email(only))
        }];
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
    let mut names: Vec<(String, Option<String>)> = list
        .iter()
        .map(|s| (short(s), if is_me(s) { None } else { email(s) }))
        .collect();
    if names.len() > 3 {
        let last = names.split_off(names.len() - 2);
        names.truncate(1);
        names.push((" .. ".to_owned(), None));
        names.extend(between(last, ", "));
        names
    } else {
        between(names, ", ").collect()
    }
}

/// Why the store could not be opened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenError {
    /// The daemon has not created the databases yet.
    NoStore {
        data_dir: String,
    },
    /// The daemon has not yet moved a database up to this version's
    /// schema, as just after an update: it does so as it starts.
    Migrating(String),
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
    reminders: Reminders,
    /// Whether any mail was sent with tracking, read on each refresh: the
    /// window asks on every frame.
    tracked: bool,
    /// Read while the window started, until it is taken or the store is
    /// read again.
    preloaded: RefCell<Option<Preload>>,
    /// The last list of conversations read, which the next start reads
    /// early.
    last_list: RefCell<Option<ListRead>>,
    /// Whether a list was read in another way, which is not read early.
    other_list: Cell<bool>,
}

/// Snoozed mail, and mail back from snooze or a follow-up reminder
/// (`katna-meta`, written by the daemon).
#[derive(Debug, Default)]
struct Reminders {
    /// Snoozed messages: when they come back.
    snoozed: HashMap<MessageId, i64>,
    /// Lines that came back to the Inbox, by when: they sort as if they
    /// arrived then.
    surfaced_messages: HashMap<MessageId, i64>,
    surfaced_threads: HashMap<ThreadId, i64>,
    /// Follow-ups nobody answered yet, by the sent message (every copy)
    /// and its conversation; the messages oldest follow-up first.
    follow_ups: HashMap<MessageId, LineFollowUp>,
    follow_up_threads: HashMap<ThreadId, LineFollowUp>,
    waiting: Vec<MessageId>,
}

impl Reminders {
    fn read(store: &Store) -> Self {
        let mut reminders = Self::default();
        match katna_meta::snoozed(store) {
            Ok(list) => {
                reminders.snoozed = list.into_iter().map(|(id, s)| (id, s.until)).collect();
            }
            Err(err) => tracing::warn!("reading snoozed mail: {err}"),
        }
        reminders.read_follow_ups(store);
        let surfaced = katna_meta::surfaced(store).unwrap_or_else(|err| {
            tracing::warn!("reading mail back from snooze: {err}");
            Vec::new()
        });
        if surfaced.is_empty() {
            return reminders;
        }
        let ids: Vec<MessageId> = surfaced.iter().map(|(id, _)| *id).collect();
        let threads: HashMap<MessageId, Option<ThreadId>> = store
            .messages_by_id(&ids)
            .unwrap_or_default()
            .into_iter()
            .map(|m| (m.id, m.thread_id))
            .collect();
        for (id, at) in surfaced {
            reminders.surfaced_messages.insert(id, at);
            if let Some(Some(thread)) = threads.get(&id) {
                let newest = reminders.surfaced_threads.entry(*thread).or_insert(at);
                *newest = (*newest).max(at);
            }
        }
        reminders
    }

    /// The follow-ups the daemon still watches whose conversation got no
    /// answer yet (the daemon drops the answered ones when they fall due).
    fn read_follow_ups(&mut self, store: &Store) {
        let list = katna_meta::follow_ups(store).unwrap_or_else(|err| {
            tracing::warn!("reading follow-ups: {err}");
            Vec::new()
        });
        let tz = jiff::tz::TimeZone::system();
        for (outbox, follow_up) in list {
            if katna_meta::replied(store, &follow_up).unwrap_or(false) {
                continue;
            }
            let copies = store
                .messages_with_header(AccountId(follow_up.account), &follow_up.message_id)
                .unwrap_or_default();
            let Some(&first) = copies.first() else {
                continue;
            };
            let line = LineFollowUp::of(outbox, &follow_up, &tz);
            self.waiting.push(first);
            for message in store.messages_by_id(&copies).unwrap_or_default() {
                self.follow_ups.insert(message.id, line);
                if let Some(thread) = message.thread_id {
                    self.follow_up_threads.insert(thread, line);
                }
            }
        }
    }

    /// The follow-up waiting on a line, if one does.
    fn follow_up(&self, key: EntryKey, latest: MessageId) -> Option<LineFollowUp> {
        match key {
            EntryKey::Thread(thread) => self.follow_up_threads.get(&thread),
            EntryKey::Message(id) => self.follow_ups.get(&id),
        }
        .or_else(|| self.follow_ups.get(&latest))
        .copied()
    }

    /// When a line came back to the Inbox, if it did.
    fn surfaced(&self, entry: &Entry) -> Option<i64> {
        match entry.key {
            EntryKey::Thread(thread) => self.surfaced_threads.get(&thread).copied(),
            EntryKey::Message(id) => self.surfaced_messages.get(&id).copied(),
        }
        .or_else(|| self.surfaced_messages.get(&entry.latest).copied())
    }
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

/// Puts lines that came back to the Inbox (from snooze, or as a reminder)
/// where mail that arrived at that time would be: `entries` are newest
/// first, and `date` tells a line's date.
fn surfaced_in_place(
    entries: Vec<Entry>,
    reminders: &Reminders,
    date: impl Fn(&Entry) -> Option<i64>,
) -> Vec<Entry> {
    if reminders.surfaced_messages.is_empty() {
        return entries;
    }
    let (mut back, mut rest): (Vec<(Entry, i64)>, Vec<Entry>) = (Vec::new(), Vec::new());
    for entry in entries {
        match reminders.surfaced(&entry) {
            Some(at) => back.push((entry, at)),
            None => rest.push(entry),
        }
    }
    if back.is_empty() {
        return rest;
    }
    // Latest first, so each goes above those that came back before it.
    back.sort_by_key(|(_, at)| std::cmp::Reverse(*at));
    let mut out = Vec::with_capacity(rest.len() + back.len());
    let mut rest = rest.into_iter().peekable();
    for (entry, at) in back {
        while let Some(next) = rest.peek() {
            if date(next).is_some_and(|d| d > at) {
                out.push(rest.next().expect("peeked"));
            } else {
                break;
            }
        }
        out.push(entry);
    }
    out.extend(rest);
    out
}

/// A folder's conversations as lines.
fn thread_entries(threads: Vec<katna_store::ThreadEntry>) -> Vec<Entry> {
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
            err @ katna_store::Error::SchemaOutdated { .. } => {
                OpenError::Migrating(err.to_string())
            }
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
            reminders: Reminders::read(&store),
            tracked: store.has_tracking().unwrap_or(false),
            store,
            index_dir,
            index,
            index_error,
            index_tried: Instant::now(),
            rows: HashMap::new(),
            preloaded: RefCell::default(),
            last_list: RefCell::default(),
            other_list: Cell::new(false),
        })
    }

    /// Takes what [`Preloading`] read while the window started, unless the
    /// mail changed since.
    pub fn use_preload(&mut self, preload: Option<Preload>) {
        let now = self.store.latest_change(katna_store::DbKind::Mail).ok();
        *self.preloaded.get_mut() = preload.filter(|p| Some(p.change) == now);
    }

    /// Once the window shows its first list: drops what was read early and
    /// not asked for, and gives the list to read early next time, if the
    /// last one read can be.
    pub fn started(&mut self) -> Option<ListRead> {
        *self.preloaded.get_mut() = None;
        let last = self.last_list.get_mut().clone();
        last.filter(|_| !self.other_list.get())
    }

    /// The conversations `read` asks for: as read while the window started,
    /// or now.
    fn list_read(&self, read: ListRead) -> katna_store::Result<InboxThreads> {
        self.other_list.set(false);
        let early = self.preloaded.borrow_mut().as_mut().and_then(|p| {
            p.list
                .take_if(|(early, _)| *early == read)
                .map(|(_, threads)| threads)
        });
        let threads = match early {
            Some(threads) => Ok(threads),
            None => read.read(&self.store),
        };
        *self.last_list.borrow_mut() = Some(read);
        threads
    }

    pub fn accounts(&self) -> Vec<Account> {
        self.store.accounts().unwrap_or_else(|err| {
            tracing::warn!("reading accounts: {err}");
            Vec::new()
        })
    }

    /// How full each account's mail storage is, for the accounts whose
    /// server reports it.
    pub fn quotas(&self) -> HashMap<katna_core::AccountId, katna_store::StorageQuota> {
        self.accounts()
            .iter()
            .filter_map(|a| Some((a.id, self.store.quota(a.id).ok()??)))
            .collect()
    }

    /// How many changes and messages wait to go to each account's
    /// servers, for an account taken offline.
    pub fn waiting(&self) -> HashMap<katna_core::AccountId, u64> {
        self.store.waiting().unwrap_or_default()
    }

    /// The IMAP server of an account, to tell its provider.
    pub fn incoming_host(&self, account: katna_core::AccountId) -> Option<String> {
        let settings = self.store.account_settings(account).ok()??;
        settings.imap.map(|server| server.host)
    }

    /// What a POP3 account does with mail on the server.
    pub fn pop3_keep(&self, account: katna_core::AccountId) -> katna_core::Pop3Keep {
        self.store
            .account_settings(account)
            .ok()
            .flatten()
            .map(|s| s.pop3_keep)
            .unwrap_or_default()
    }

    /// The provider an account signs in with, if not a password.
    pub fn sign_in_provider(
        &self,
        account: katna_core::AccountId,
    ) -> Option<katna_core::OAuthProvider> {
        self.store.account_settings(account).ok()??.oauth
    }

    pub fn folders(&self) -> Vec<FolderSummary> {
        let early = self
            .preloaded
            .borrow_mut()
            .as_mut()
            .and_then(|p| p.folders.take());
        if let Some(folders) = early {
            return folders;
        }
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
            self.list_read(ListRead::Folder {
                folder,
                categories: categories.map(<[_]>::to_vec),
            })
            .map(|(threads, _)| thread_entries(threads))
        } else {
            self.other_list.set(true);
            match categories {
                Some(categories) => self.store.folder_messages_in(folder, categories),
                None => self.store.folder_message_ids(folder),
            }
            .map(|ids| ids.into_iter().map(Entry::message).collect())
        };
        self.folder_lines(folder, entries)
    }

    /// An inbox's lines, as [`Mail::entries`] gives them, with its unread
    /// conversations per tab ([`Mail::category_unread`]). Listing
    /// conversations, both come from one read of the folder.
    pub fn inbox_entries(
        &self,
        folder: FolderId,
        categories: Option<&[MailCategory]>,
        conversations: bool,
    ) -> (Vec<Entry>, HashMap<MailCategory, u64>) {
        if !conversations {
            return (
                self.entries(folder, categories, false),
                self.category_unread(folder),
            );
        }
        let read = ListRead::Inbox {
            folder,
            categories: categories.map(<[_]>::to_vec),
        };
        match self.list_read(read) {
            Ok((threads, unread)) => (
                self.folder_lines(folder, Ok(thread_entries(threads))),
                unread.into_iter().collect(),
            ),
            Err(err) => (self.folder_lines(folder, Err(err)), HashMap::new()),
        }
    }

    /// `folder`'s lines as read, with mail back from snooze in place and
    /// pinned lines first.
    fn folder_lines(
        &self,
        folder: FolderId,
        entries: katna_store::Result<Vec<Entry>>,
    ) -> Vec<Entry> {
        let entries = entries.unwrap_or_else(|err| {
            tracing::warn!("reading folder {}: {err}", folder.0);
            Vec::new()
        });
        let entries = surfaced_in_place(entries, &self.reminders, |e| self.date_of(e.latest));
        pinned_first(entries, &self.pins)
    }

    /// The lines of a list across folders, such as the unified inbox's:
    /// the mail in any of `folders` that `filter` keeps.
    pub fn spread_entries(
        &self,
        folders: &[FolderId],
        filter: FlagFilter,
        conversations: bool,
    ) -> Vec<Entry> {
        let entries = if conversations {
            self.list_read(ListRead::Spread {
                folders: folders.to_vec(),
                filter,
            })
            .map(|(threads, _)| thread_entries(threads))
        } else {
            self.other_list.set(true);
            self.store
                .spread_message_ids(folders, filter)
                .map(|ids| ids.into_iter().map(Entry::message).collect())
        };
        let entries = entries.unwrap_or_else(|err| {
            tracing::warn!("reading {} folders: {err}", folders.len());
            Vec::new()
        });
        let entries = surfaced_in_place(entries, &self.reminders, |e| self.date_of(e.latest));
        pinned_first(entries, &self.pins)
    }

    /// The unified inbox's lines in one tab ([`SpreadTabs`]), with its
    /// unread conversations per tab, as [`Mail::inbox_entries`] gives an
    /// inbox's.
    pub fn spread_inbox_entries(
        &self,
        folders: &[FolderId],
        tabs: &SpreadTabs,
        conversations: bool,
    ) -> (Vec<Entry>, HashMap<MailCategory, u64>) {
        let (entries, unread) = if conversations {
            let read = self.list_read(ListRead::SpreadInbox {
                folders: folders.to_vec(),
                tabs: tabs.clone(),
            });
            match read {
                Ok((threads, unread)) => (Ok(thread_entries(threads)), unread),
                Err(err) => (Err(err), Vec::new()),
            }
        } else {
            self.other_list.set(true);
            let unread = self
                .store
                .spread_inbox_threads(folders, tabs)
                .map(|(_, unread)| unread)
                .unwrap_or_default();
            let ids = self.store.spread_inbox_message_ids(folders, tabs);
            (
                ids.map(|ids| ids.into_iter().map(Entry::message).collect()),
                unread,
            )
        };
        let entries = entries.unwrap_or_else(|err| {
            tracing::warn!("reading {} inboxes: {err}", folders.len());
            Vec::new()
        });
        let entries = surfaced_in_place(entries, &self.reminders, |e| self.date_of(e.latest));
        (
            pinned_first(entries, &self.pins),
            unread.into_iter().collect(),
        )
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

    /// What a task made from line `key` holds: the conversation's subject
    /// (its first message's) and the newest message's Message-ID.
    pub fn task_source(&self, key: EntryKey) -> Option<(String, String)> {
        let messages = self.entry_messages(key);
        let (first, newest) = (*messages.first()?, *messages.last()?);
        let subject = self
            .store
            .messages_by_id(&[first])
            .ok()?
            .first()
            .map(|m| m.subject.trim().to_owned())
            .unwrap_or_default();
        let header = self.message_id_header(newest).unwrap_or_default();
        Some((subject, header))
    }

    /// What a meeting set up from line `key` starts with: the
    /// conversation's subject without `Re:` and `Fwd:`, and everyone in it
    /// but the user, as (name, address), in the order they first appear.
    pub fn meeting_source(&self, key: EntryKey) -> Option<(String, Vec<(String, String)>)> {
        let messages = self.store.messages_by_id(&self.entry_messages(key)).ok()?;
        let subject =
            katna_core::subject::without_reply_prefixes(&messages.first()?.subject).to_owned();
        let mut people: Vec<(String, String)> = Vec::new();
        for p in messages.iter().flat_map(|m| &m.participants) {
            let role = matches!(
                p.role,
                ParticipantRole::From | ParticipantRole::To | ParticipantRole::Cc
            );
            if !role
                || !p.email_norm.contains('@')
                || self.is_me(&p.email_norm)
                || people
                    .iter()
                    .any(|(_, e)| e.eq_ignore_ascii_case(&p.email_norm))
            {
                continue;
            }
            let name = p.display_name.as_deref().unwrap_or_default().trim();
            people.push((name.to_owned(), p.email_norm.clone()));
        }
        Some((subject, people))
    }

    /// The newest stored message with `Message-ID` `header`.
    pub fn message_with_header(&self, header: &str) -> Option<MessageId> {
        self.store.message_with_header(header).ok().flatten()
    }

    /// The chat pins of the conversation of `messages` (all of them), in
    /// their order.
    pub fn chat_pins(&self, messages: &[MessageId]) -> Vec<katna_store::ChatPin> {
        self.store.chat_pins(messages).unwrap_or_else(|err| {
            tracing::warn!("reading chat pins: {err}");
            Vec::new()
        })
    }

    /// The kept summaries of the conversation of `messages` (all of
    /// them), the newest first.
    pub fn summaries(&self, messages: &[MessageId]) -> Vec<katna_store::StoredSummary> {
        self.store
            .conversation_summaries(messages)
            .unwrap_or_else(|err| {
                tracing::warn!("reading summaries: {err}");
                Vec::new()
            })
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

    /// When `message` was sent, if known.
    fn date_of(&self, message: MessageId) -> Option<i64> {
        self.store
            .messages_by_id(&[message])
            .ok()?
            .pop()
            .and_then(|m| m.date)
    }

    /// Lines of the sent mail a follow-up waits on, for Waiting for
    /// reply: newest first, as conversations when `conversations`.
    pub fn waiting_entries(&self, conversations: bool) -> Vec<Entry> {
        let mut ids = self.reminders.waiting.clone();
        ids.sort_by_key(|id| std::cmp::Reverse(self.date_of(*id)));
        self.hit_entries(&ids, conversations, None)
    }

    /// The follow-up of outbox entry `outbox` as the daemon keeps it, for
    /// an Undo that sets it again.
    pub fn follow_up_value(&self, outbox: i64) -> Option<katna_meta::FollowUp> {
        katna_meta::follow_up_of(&self.store, outbox).ok().flatten()
    }

    /// How many follow-ups wait, for the folder list.
    pub fn waiting_count(&self) -> usize {
        self.reminders.waiting.len()
    }

    /// The follow-up waiting on one of `messages` (a conversation), if
    /// any: the soonest due.
    pub fn follow_up_in(&self, messages: &[MessageId]) -> Option<LineFollowUp> {
        messages
            .iter()
            .filter_map(|id| self.reminders.follow_ups.get(id))
            .min_by_key(|f| f.at)
            .copied()
    }

    /// When snoozed `messages` come back: the soonest, if any is snoozed.
    pub fn snoozed_until(&self, messages: &[MessageId]) -> Option<i64> {
        messages
            .iter()
            .filter_map(|id| self.reminders.snoozed.get(id))
            .min()
            .copied()
    }

    /// When the line `entry` comes back from snooze, if it is snoozed.
    pub fn entry_snoozed_until(&self, entry: &Entry) -> Option<i64> {
        match entry.key {
            EntryKey::Message(id) => self.reminders.snoozed.get(&id).copied(),
            key => self.snoozed_until(&self.entry_messages(key)),
        }
    }

    /// Picks up what the daemon wrote since the last call.
    pub fn refresh(&mut self) {
        self.rows.clear();
        *self.preloaded.get_mut() = None;
        self.pins = Pins::read(&self.store);
        self.reminders = Reminders::read(&self.store);
        self.tracked = self.store.has_tracking().unwrap_or(false);
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
        let activity = self.store.tracking_activity(&latest).unwrap_or_else(|err| {
            tracing::warn!("reading tracking: {err}");
            HashMap::new()
        });
        for entry in entries {
            let Some(message) = messages.get(&entry.latest) else {
                continue;
            };
            let mut row = Row::new(message, show_recipients);
            row.tracking = activity.get(&entry.latest).map(Tracked::from);
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
                snoozed_until: self.reminders.snoozed.get(&row.id).copied(),
                follow_up: self.reminders.follow_up(row.key, row.id),
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

    /// What the recipients of `message` did, if it was sent with tracking.
    pub fn activity(&self, message: MessageId) -> Option<katna_store::MessageActivity> {
        self.store
            .tracking_activity(&[message])
            .unwrap_or_else(|err| {
                tracing::warn!("reading tracking: {err}");
                HashMap::new()
            })
            .remove(&message)
    }

    /// Whether `email` is one of the accounts' addresses.
    pub fn is_me(&self, email: &str) -> bool {
        self.me.iter().any(|me| me.eq_ignore_ascii_case(email))
    }

    /// Message `id` as a read receipt, if it is one.
    pub fn receipt(&self, id: MessageId) -> Option<crate::receipts::Receipt> {
        let message = self.store.messages_by_id(&[id]).ok()?.pop()?;
        if message.size > crate::receipts::MAX_SIZE {
            return None;
        }
        crate::receipts::parse(&self.store.blobs().get(&message.blob_hash?).ok()??)
    }

    /// What delivery and read receipts said about the message with
    /// `Message-ID` `original`, per recipient.
    pub fn receipts(&self, original: &str) -> Vec<katna_store::Receipt> {
        self.store.receipts(original).unwrap_or_else(|err| {
            tracing::warn!("reading receipts: {err}");
            Vec::new()
        })
    }

    /// The inbox tab message `id` was sorted into, if it was.
    pub fn message_category(&self, id: MessageId) -> Option<MailCategory> {
        self.store.message_category(id).ok().flatten()
    }

    /// The `Message-ID` of message `id`, without angle brackets.
    pub fn message_id_header(&self, id: MessageId) -> Option<String> {
        self.store.message_id_header(id).ok().flatten()
    }

    /// Recent mail sent with tracking, newest first, with what its
    /// recipients did.
    pub fn tracked(&self, limit: u32) -> Vec<katna_store::MessageActivity> {
        let tracked = self.store.tracked_messages(limit).unwrap_or_else(|err| {
            tracing::warn!("reading tracking: {err}");
            Vec::new()
        });
        tracked
            .into_iter()
            .filter_map(|t| self.store.activity(t).ok())
            .collect()
    }

    /// Opens and clicks since `since` (Unix milliseconds) and after event
    /// `after`, newest first.
    pub fn activity_feed(
        &self,
        since: i64,
        after: i64,
        limit: u32,
    ) -> Vec<katna_store::ActivityItem> {
        self.store
            .activity_feed(since, after, limit)
            .unwrap_or_else(|err| {
                tracing::warn!("reading tracking: {err}");
                Vec::new()
            })
    }

    /// Opens and clicks by people after event `seq`.
    pub fn activity_after(&self, seq: i64) -> usize {
        self.store.activity_after(seq).unwrap_or(0)
    }

    /// The number of the newest open or click kept.
    pub fn last_activity(&self) -> i64 {
        self.store.last_tracking_seq().unwrap_or(0)
    }

    /// The stored copy of the sent message `message_id` of `account`.
    pub fn sent_copy(&self, account: AccountId, message_id: &str) -> Option<MessageId> {
        self.store.filed_message(account, message_id).ok().flatten()
    }

    /// Whether any mail was sent with tracking.
    pub fn has_tracking(&self) -> bool {
        self.tracked
    }

    /// Forgets cached rows, for example when the sender/recipient column
    /// changes.
    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    /// The people on messages `ids` other than the user, each once: for
    /// each message in turn its sender, then its recipients. Addresses
    /// are lower case.
    pub fn message_people(&self, ids: &[MessageId]) -> Vec<(String, Option<String>)> {
        use katna_store::ParticipantRole as Role;
        let messages = self.store.messages_by_id(ids).unwrap_or_else(|err| {
            tracing::warn!("reading the people of a conversation: {err}");
            Vec::new()
        });
        let mut people: Vec<(String, Option<String>)> = Vec::new();
        // The user's own addresses, for mail only between them.
        let mut own: Vec<(String, Option<String>)> = Vec::new();
        for id in ids {
            let Some(message) = messages.iter().find(|m| m.id == *id) else {
                continue;
            };
            for role in [Role::From, Role::To, Role::Cc] {
                for p in message.participants.iter().filter(|p| p.role == role) {
                    let email = p.email_norm.trim().to_lowercase();
                    let mine = self
                        .me
                        .iter()
                        .any(|me| me.trim().eq_ignore_ascii_case(&email));
                    if !email.contains('@') {
                        continue;
                    }
                    let name = p
                        .display_name
                        .as_deref()
                        .map(str::trim)
                        .filter(|n| !n.is_empty() && !n.eq_ignore_ascii_case(&email))
                        .map(str::to_owned);
                    let people = if mine { &mut own } else { &mut people };
                    match people.iter_mut().find(|(e, _)| *e == email) {
                        Some((_, known)) => {
                            if known.is_none() {
                                *known = name;
                            }
                        }
                        None => people.push((email, name)),
                    }
                }
            }
        }
        if people.is_empty() { own } else { people }
    }

    /// The drafts among `ids`: messages flagged `\Draft`.
    pub fn drafts(&self, ids: &[MessageId]) -> Vec<MessageId> {
        self.store
            .messages_by_id(ids)
            .unwrap_or_default()
            .into_iter()
            .filter(|m| m.flags.contains(MessageFlags::DRAFT))
            .map(|m| m.id)
            .collect()
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

/// What rings and counts (`docs/ARCHITECTURE.md` §15.1.1): the folder
/// bells that differ from the default, and what is muted.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Alerts {
    /// By folder and inbox tab (`None` for the whole folder).
    pub bells: HashMap<(FolderId, Option<MailCategory>), Bell>,
    pub mutes: Vec<Mute>,
    pub muted_threads: HashSet<ThreadId>,
    /// A message of each muted conversation, which Unmute names.
    pub thread_message: HashMap<ThreadId, MessageId>,
}

impl Alerts {
    fn read(store: &Store, now: i64) -> katna_store::Result<Self> {
        let mutes = store.mutes(now)?;
        let mut thread_message = HashMap::new();
        for mute in &mutes {
            if let MuteTarget::Thread(thread) = mute.target
                && let Some(message) = store.thread_messages(thread)?.first()
            {
                thread_message.insert(thread, *message);
            }
        }
        Ok(Self {
            bells: store
                .folder_bells()?
                .into_iter()
                .map(|b| ((b.folder, b.category), b.bell))
                .collect(),
            mutes,
            thread_message,
            muted_threads: store.muted_threads(now)?,
        })
    }

    /// The bell of `folder` (`role` its role), or of its inbox tab
    /// `category` (`None`: an inbox's Primary tab).
    pub fn bell(
        &self,
        folder: FolderId,
        role: Option<&str>,
        category: Option<MailCategory>,
    ) -> Bell {
        let inbox = role == Some(katna_store::FolderRole::Inbox.as_str());
        let category = if inbox {
            Some(category.unwrap_or_default())
        } else {
            None
        };
        self.bells
            .get(&(folder, category))
            .copied()
            .unwrap_or_else(|| Bell::default_for(role, category))
    }

    /// The mute of `target` in force, if any.
    pub fn mute(&self, target: &MuteTarget) -> Option<&Mute> {
        self.mutes.iter().find(|m| &m.target == target)
    }
}

/// The folders with their message counts, unread messages per folder and
/// what rings and counts. Opens its own connection, so it can run on a
/// background thread while the UI uses [`Mail`]. `None` for the folders
/// when they could not be read.
pub fn folders_and_unread(
    paths: &Paths,
) -> (Option<Vec<FolderSummary>>, HashMap<FolderId, u64>, Alerts) {
    let now = jiff::Timestamp::now().as_second();
    let read = Store::open(paths, Mode::ReadOnly).and_then(|store| {
        Ok((
            store.folder_summaries()?,
            store.unread_counts()?,
            Alerts::read(&store, now)?,
        ))
    });
    match read {
        Ok((folders, counts, alerts)) => (Some(folders), counts.into_iter().collect(), alerts),
        Err(err) => {
            tracing::warn!("counting unread mail: {err}");
            (None, HashMap::new(), Alerts::default())
        }
    }
}

/// Mailbox insights from `since` to before `until` (Unix seconds) in
/// `account` (or every account) for the user's addresses `me`, with hours
/// in `tz`. Opens its own connection, for a background thread.
pub fn insights(
    paths: &Paths,
    account: Option<AccountId>,
    me: &[String],
    since: i64,
    until: i64,
    tz: &jiff::tz::TimeZone,
) -> Result<katna_store::Insights, String> {
    let local = |unix: i64| {
        jiff::Timestamp::from_second(unix).map_or((0, 0), |at| {
            let at = at.to_zoned(tz.clone());
            (
                usize::try_from(at.weekday().to_monday_zero_offset()).unwrap_or(0),
                usize::try_from(at.hour()).unwrap_or(0),
            )
        })
    };
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.mailbox_insights(account, me, since, until, local))
        .map_err(|err| format!("Counting mail failed: {err}"))
}

/// The people in the mail, most written with first. Opens its own
/// connection, for a background thread.
pub fn people(paths: &Paths) -> Result<Vec<katna_store::Person>, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.people(PEOPLE_LIMIT))
        .map_err(|err| format!("Reading people from the mail failed: {err}"))
}

/// The files of the Files page, newest first; see
/// [`katna_store::LibraryFile`]. Opens its own connection, for a
/// background thread.
pub fn library(paths: &Paths, limit: usize) -> Result<Vec<katna_store::LibraryFile>, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.library_files(limit))
        .map_err(|err| katna_i18n::tr!("files-load-failed", error = err.to_string()))
}

/// The raw messages of `ids` whose bodies are stored, for thumbnails made
/// in the background. Opens its own connection.
pub fn raw_messages(paths: &Paths, ids: &[MessageId]) -> HashMap<MessageId, Vec<u8>> {
    let Ok(store) = Store::open(paths, Mode::ReadOnly) else {
        return HashMap::new();
    };
    let Ok(messages) = store.messages_by_id(ids) else {
        return HashMap::new();
    };
    messages
        .into_iter()
        .filter_map(|m| {
            let raw = store.blobs().get(&m.blob_hash?).ok()??;
            Some((m.id, raw))
        })
        .collect()
}

/// Every note, pinned first. Opens its own connection, for a background
/// thread.
pub fn notes(paths: &Paths) -> Result<Vec<katna_store::Note>, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.notes())
        .map_err(|err| format!("Reading notes failed: {err}"))
}

/// Note `id`'s pictures. Opens its own connection.
pub fn note_pictures(paths: &Paths, id: i64) -> Result<Vec<katna_store::NotePicture>, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.note_pictures(id))
        .map_err(|err| format!("Reading a note's pictures failed: {err}"))
}

/// The first picture of each note that has one, for the cards. Opens its
/// own connection, for a background thread.
pub fn note_covers(
    paths: &Paths,
) -> Result<std::collections::HashMap<i64, katna_store::NotePicture>, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.note_covers())
        .map_err(|err| format!("Reading notes' pictures failed: {err}"))
}

/// Note `id`'s earlier versions, newest first. Opens its own connection.
pub fn note_versions(paths: &Paths, id: i64) -> Result<Vec<katna_store::NoteVersion>, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.note_versions(id))
        .map_err(|err| format!("Reading a note's history failed: {err}"))
}

/// The saved contacts with their labels and address books, for the
/// Contacts page. Opens its own connection, for a background thread.
pub fn saved_contacts(paths: &Paths) -> Result<SavedBook, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| {
            Ok(SavedBook {
                people: store.saved_contacts()?,
                labels: store.contact_labels()?,
                books: store.address_books()?,
                others: store.other_contacts()?,
                others_blocked: store.other_contacts_blocked()?,
            })
        })
        .map_err(|err| format!("Reading contacts failed: {err}"))
}

impl SavedBook {
    /// Leaves out the `hidden` accounts' address books and other
    /// contacts, and the people saved only in them.
    pub fn leave_out(&mut self, hidden: &HashSet<AccountId>) {
        if hidden.is_empty() {
            return;
        }
        let shown = |account: &Option<AccountId>| account.is_none_or(|a| !hidden.contains(&a));
        self.people
            .retain(|p| p.accounts.is_empty() || p.accounts.iter().any(shown));
        self.books.retain(|b| shown(&b.account));
        self.others.retain(|o| !hidden.contains(&o.account));
    }
}

/// Everyone saved, one entry per person, with the labels and books.
#[derive(Debug, Default)]
pub struct SavedBook {
    pub people: Vec<katna_store::SavedContact>,
    pub labels: Vec<katna_store::ContactLabel>,
    pub books: Vec<katna_store::AddressBook>,
    /// Google's other contacts, and the accounts that have to allow them.
    pub others: Vec<katna_store::OtherContact>,
    pub others_blocked: Vec<katna_core::AccountId>,
}

/// The cards that have a picture.
pub fn people_with_photos(paths: &Paths) -> Result<std::collections::HashSet<i64>, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.contacts_with_photos())
        .map_err(|err| format!("Reading contact pictures failed: {err}"))
}

/// The saved cards `ids` of one person.
pub fn saved_cards(paths: &Paths, ids: &[i64]) -> Result<Vec<katna_store::StoredCard>, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.saved_cards(ids))
        .map_err(|err| format!("Reading a contact failed: {err}"))
}

/// The picture of the first of `ids` that has one.
pub fn contact_photo(paths: &Paths, ids: &[i64]) -> Result<Option<Vec<u8>>, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.contact_photo(ids))
        .map_err(|err| format!("Reading a contact picture failed: {err}"))
}

/// The mail templates, by name. Opens its own connection, for a
/// background thread.
pub fn templates(paths: &Paths) -> Result<Vec<katna_store::TemplateSummary>, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.templates())
        .map_err(|err| format!("Reading templates failed: {err}"))
}

/// Template `id` with its body and attachments.
pub fn template(paths: &Paths, id: i64) -> Result<Option<katna_store::Template>, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.template(id))
        .map_err(|err| format!("Reading a template failed: {err}"))
}

/// The mail rules, in the order they run (Settings > Folders & rules).
pub fn rules(paths: &Paths) -> Result<Vec<katna_store::rules::Rule>, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| store.rules())
        .map_err(|err| format!("Reading the mail rules failed: {err}"))
}

/// How many messages in the inboxes of `rule`'s accounts from the last
/// `days` days before `now` it matches, as the daemon would match them:
/// with the stored text of each message when a condition needs it.
pub fn rule_preview(
    paths: &Paths,
    rule: &katna_store::rules::Rule,
    days: u32,
    now: i64,
) -> Result<u32, String> {
    let store = Store::open(paths, Mode::ReadOnly).map_err(|err| err.to_string())?;
    let blobs = store.blobs();
    store
        .rule_preview(rule, days, now, |message| {
            let hash = message.blob_hash.as_ref()?;
            let raw = blobs.get(hash).ok()??;
            Some(katna_search::document::message_text(&raw).body)
        })
        .map_err(|err| format!("Counting the rule's mail failed: {err}"))
}

/// The address book for recipient suggestions, read from the store (a
/// few seconds on a big mailbox): the people mailed, and with `saved` the
/// saved contacts too (Contacts is on). Opens its own connection, for a
/// background thread.
pub fn address_book(
    paths: &Paths,
    saved: bool,
) -> Result<katna_search::contacts::ContactBook, String> {
    Store::open(paths, Mode::ReadOnly)
        .and_then(|store| {
            let rows = store.correspondents()?;
            // An older store without saved contacts still suggests.
            let saved = if saved {
                store.saved_names().unwrap_or_default()
            } else {
                Vec::new()
            };
            Ok(katna_search::contacts::ContactBook::with_saved(rows, saved))
        })
        .map_err(|err| format!("Reading addresses from the mail failed: {err}"))
}

/// Where the pixel sizes of attached pictures are kept, so the Files page
/// can leave out small ones without reading their mail again.
fn picture_sizes_file(paths: &Paths) -> PathBuf {
    paths.cache_dir().join("files-picture-sizes.json")
}

/// Pixel sizes of attached pictures, by message and place among its named
/// attachments; `(0, 0)` for one whose size cannot be read (an SVG).
pub type PictureSizes = HashMap<(MessageId, usize), (u32, u32)>;

/// The sizes saved by [`save_picture_sizes`], if any.
pub fn picture_sizes(paths: &Paths) -> PictureSizes {
    let Ok(bytes) = std::fs::read(picture_sizes_file(paths)) else {
        return PictureSizes::new();
    };
    let saved: Vec<(i64, usize, u32, u32)> = serde_json::from_slice(&bytes)
        .inspect_err(|err| tracing::warn!("reading the picture sizes: {err}"))
        .unwrap_or_default();
    saved
        .into_iter()
        .map(|(message, order, w, h)| ((MessageId(message), order), (w, h)))
        .collect()
}

/// Saves the picture sizes for the next time the Files page opens.
pub fn save_picture_sizes(paths: &Paths, sizes: &PictureSizes) {
    let file = picture_sizes_file(paths);
    let partial = file.with_extension("json.part");
    let mut rows: Vec<(i64, usize, u32, u32)> = sizes
        .iter()
        .map(|((message, order), (w, h))| (message.0, *order, *w, *h))
        .collect();
    rows.sort_unstable();
    let saved = serde_json::to_vec(&rows)
        .map_err(std::io::Error::other)
        .and_then(|bytes| {
            std::fs::create_dir_all(paths.cache_dir())?;
            std::fs::write(&partial, bytes)?;
            std::fs::rename(&partial, &file)
        });
    if let Err(err) = saved {
        tracing::warn!("saving the picture sizes: {err}");
    }
}

/// Where the address book is kept between runs, so suggestions work at
/// once while it is read again.
/// The saved copy of the address book, one with the saved contacts and
/// one of the people mailed only.
fn address_book_file(paths: &Paths, saved: bool) -> PathBuf {
    paths.cache_dir().join(if saved {
        "addresses.json"
    } else {
        "addresses-mailed.json"
    })
}

/// The address book saved by [`save_address_book`], if any.
pub fn cached_address_book(
    paths: &Paths,
    saved: bool,
) -> Option<katna_search::contacts::ContactBook> {
    let bytes = std::fs::read(address_book_file(paths, saved)).ok()?;
    let contacts = serde_json::from_slice(&bytes)
        .inspect_err(|err| tracing::warn!("reading the saved address book: {err}"))
        .ok()?;
    Some(katna_search::contacts::ContactBook::from_contacts(contacts))
}

/// Saves the address book for the next run, readable only by the user.
pub fn save_address_book(paths: &Paths, book: &katna_search::contacts::ContactBook, saved: bool) {
    use std::io::Write;
    #[cfg(unix)]
    use std::os::unix::fs::OpenOptionsExt;
    let file = address_book_file(paths, saved);
    let partial = file.with_extension("json.part");
    let saved = serde_json::to_vec(book.contacts())
        .map_err(std::io::Error::other)
        .and_then(|bytes| {
            std::fs::create_dir_all(paths.cache_dir())?;
            let mut options = std::fs::OpenOptions::new();
            options.write(true).create(true).truncate(true);
            #[cfg(unix)]
            options.mode(0o600);
            options.open(&partial)?.write_all(&bytes)?;
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

    #[test]
    fn follow_up_chips_say_which_follow_up_is_next_and_when() {
        let tz = jiff::tz::TimeZone::get("Asia/Kolkata").unwrap();
        // Saturday 10 Oct 2026, 11:00 in Kolkata: sent on Monday.
        let saturday = 1_791_610_200;
        let monday_nine = saturday + 2 * 86_400 - 2 * 3600;
        let mut f = katna_meta::FollowUp {
            remind_at: saturday,
            mail: Some("To: b@x\r\n\r\nHi".into()),
            again: 7 * 86_400,
            ..Default::default()
        };
        let line = LineFollowUp::of(4, &f, &tz);
        assert_eq!((line.at, line.step, line.steps), (monday_nine, 1, 2));
        f.sent.push("f1@katna".into());
        let line = LineFollowUp::of(4, &f, &tz);
        assert_eq!((line.step, line.steps), (2, 2));
        // A reminder keeps its time; a waiting one says so.
        f.mail = None;
        let line = LineFollowUp::of(4, &f, &tz);
        assert_eq!((line.at, line.sends, line.steps), (saturday, false, 1));
        f.waiting = true;
        assert!(LineFollowUp::of(4, &f, &tz).waiting);
    }

    #[test]
    fn mail_back_from_snooze_sorts_by_when_it_came_back() {
        let line = |n| Entry::message(MessageId(n));
        // Dates: 5 newest, then 4, 3, 2, 1; 1 and 2 came back at 450 and
        // 350, 7's conversation at 999.
        let dates = HashMap::from([(5, 500), (4, 400), (3, 300), (2, 200), (1, 100), (6, 50)]);
        let thread = Entry {
            key: EntryKey::Thread(ThreadId(7)),
            latest: MessageId(6),
        };
        let reminders = Reminders {
            snoozed: HashMap::new(),
            surfaced_messages: HashMap::from([(MessageId(1), 450), (MessageId(2), 350)]),
            surfaced_threads: HashMap::from([(ThreadId(7), 999)]),
            ..Reminders::default()
        };
        let entries = vec![line(5), line(4), line(3), line(2), line(1), thread];
        assert_eq!(
            surfaced_in_place(entries, &reminders, |e| dates.get(&e.latest.0).copied()),
            [thread, line(5), line(1), line(4), line(2), line(3)]
        );
        let none = Reminders::default();
        assert_eq!(
            surfaced_in_place(vec![line(2), line(1)], &none, |_| None),
            [line(2), line(1)]
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
    fn store_waiting_for_the_daemon() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        drop(Store::open(&paths, Mode::ReadWrite).unwrap());
        // As an update leaves it: a schema one version behind. SQLite keeps
        // `user_version` at byte 60 of the header.
        let db = paths.pim_db();
        let mut bytes = std::fs::read(&db).unwrap();
        let version = u32::from_be_bytes(bytes[60..64].try_into().unwrap());
        bytes[60..64].copy_from_slice(&(version - 1).to_be_bytes());
        std::fs::write(&db, bytes).unwrap();
        assert!(matches!(Mail::open(&paths), Err(OpenError::Migrating(_))));
        drop(Store::open(&paths, Mode::ReadWrite).unwrap());
        assert!(Mail::open(&paths).is_ok());
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
                account: mail.accounts()[0].id,
                count: 1,
                correspondent: "Ada".into(),
                people: vec![("Ada".into(), Some("ada@example.org".into()))],
                sender: "ada@example.org".into(),
                subject: "Budget".into(),
                date: Some(989_858_340),
                unread: true,
                flagged: true,
                important: false,
                pinned: false,
                snoozed_until: None,
                follow_up: None,
                attachments: false,
                files: Vec::new(),
                snippet: "The budget is final.".into(),
                tracking: None,
                replied: false,
            }
        );
        assert_eq!(rows[1], None);
        mail.clear_rows();
        let sent = mail.rows(&[Entry::message(id)], None, true);
        assert_eq!(
            sent[0].as_ref().unwrap().correspondent,
            "To: bob@example.net"
        );

        assert_eq!(folders_and_unread(&paths).1, HashMap::from([(inbox, 1)]));
        assert_eq!(mail.raw(id).as_deref(), Some(RAW));
        assert_eq!(mail.raw(MessageId(999)), None);
    }

    #[test]
    fn the_first_list_is_read_early_and_only_while_current() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        // No store yet: nothing is read, and the window does not wait.
        assert!(Preloading::start(&paths).wait().is_none());

        let (inbox, id) = store_with_mail(&paths);
        let read = ListRead::Folder {
            folder: inbox,
            categories: None,
        };
        // No list remembered yet: the folders only.
        let preload = Preloading::start(&paths).wait().unwrap();
        assert!(preload.list.is_none());
        assert_eq!(preload.folders.as_ref().unwrap()[0].total, 1);
        let mut mail = Mail::open(&paths).unwrap();
        mail.use_preload(Some(preload));
        assert_eq!(mail.folders()[0].total, 1);
        let lines = mail.entries(inbox, None, true);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].latest, id);
        let first = mail.started().unwrap();
        assert_eq!(first, read);
        remember_first_list(&paths, &first);
        let preload = Preloading::start(&paths).wait().unwrap();
        assert_eq!(preload.list.as_ref().unwrap().0, read);

        // What was read early stands in for the store's read: here nothing.
        let early = |change| Preload {
            change,
            list: Some((read.clone(), (Vec::new(), Vec::new()))),
            folders: None,
        };
        let mut mail = Mail::open(&paths).unwrap();
        mail.use_preload(Some(early(preload.change)));
        assert_eq!(mail.entries(inbox, None, true), []);
        // Once: the next read is the store's.
        assert_eq!(mail.entries(inbox, None, true), lines);
        // Mail changed since it was read: it is not used.
        let mut mail = Mail::open(&paths).unwrap();
        mail.use_preload(Some(early(preload.change + 1)));
        assert_eq!(mail.entries(inbox, None, true), lines);
        // Nor for another list, and listing messages is not read early.
        let mut mail = Mail::open(&paths).unwrap();
        mail.use_preload(Some(early(preload.change)));
        let tab = [MailCategory::Primary];
        assert_eq!(mail.entries(inbox, Some(&tab), true), lines);
        assert_eq!(mail.entries(inbox, None, false), [Entry::message(id)]);
        assert_eq!(mail.started(), None);
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
