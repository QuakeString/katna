// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna in the desktop's search (`docs/ARCHITECTURE.md` §15.3): KRunner's
//! `org.kde.krunner1` and GNOME Shell's `org.gnome.Shell.SearchProvider2`,
//! answered from the search index and the addresses in the mail.
//!
//! People the user writes with come up as they are typed. Mail comes up
//! only when every word is in its subject or sender, so a word typed to
//! start an app does not fill the list with mail; `mail:` searches all of
//! it as Katna Mail's search box does. Open tasks come up when every word
//! starts a word of their title or details, and open in the Tasks page.
//! Events of the coming year come up the same way, by their title, place
//! or details, and open in the Calendar on their day.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use jiff::tz::TimeZone;
use katna_core::{Paths, ids};
use katna_dbus::app_action;
use katna_i18n::tr;
use katna_search::contacts::ContactBook;
use katna_search::{Filter, Query, SearchIndex, SearchOptions, Sort, TextField};
use katna_store::calendar::{EventData, EventStatus, StoredEvent};
use katna_store::{MessageFlags, MessageId, Mode, ParticipantRole, Store};
use zbus::zvariant::Value;

use crate::mail_app;

/// What searches everything in the mail: with this prefix, or one of the
/// trigger words from Settings (`general.search_triggers`) and a space, a
/// query is Katna Mail's search box's.
const MAIL_PREFIX: &str = "mail:";
/// Shorter queries find nothing (KRunner asks from three letters too).
const MIN_CHARS: usize = 3;
/// People shown at most.
const CONTACTS: usize = 5;
/// Messages shown at most, without and with `mail:`.
const CONFIDENT_MAIL: usize = 3;
const ALL_MAIL: usize = 10;
/// The address book is read again at most this often while mail changes.
const BOOK_AGE: Duration = Duration::from_secs(10 * 60);

/// Match IDs: a message ID or an address after a letter saying which.
const MAIL_ID: char = 'm';
const CONTACT_ID: char = 'c';
const TASK_ID: char = 't';
/// Tasks shown at most.
const TASKS: usize = 3;
const EVENT_ID: char = 'e';
/// Events shown at most, and how far ahead they are looked for.
const EVENTS: usize = 3;
const EVENT_DAYS: i64 = 366;
const DAY: i64 = 24 * 60 * 60;

/// Action IDs on KRunner's results. Without one, a message opens and a
/// person gets a new message.
const REPLY_ALL: &str = "reply-all";
const COPY: &str = "copy";
const FIND: &str = "find";

/// KRunner's match types (`KRunner::QueryMatch::CategoryRelevance`).
const EXACT_MATCH: i32 = 100;
const POSSIBLE_MATCH: i32 = 30;

/// One result.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Found {
    pub id: String,
    pub text: String,
    pub subtext: String,
    pub icon: &'static str,
    /// KRunner's match type and relevance (0 to 1).
    pub kind: i32,
    pub relevance: f64,
}

impl Found {
    fn is_mail(&self) -> bool {
        self.id.starts_with(MAIL_ID)
    }

    fn is_task(&self) -> bool {
        self.id.starts_with(TASK_ID)
    }

    fn is_event(&self) -> bool {
        self.id.starts_with(EVENT_ID)
    }
}

/// Finds results; shared by both interfaces.
pub(crate) struct Finder {
    paths: Paths,
    /// Opened on first use and again after a failure.
    store: Mutex<Option<Store>>,
    index: Mutex<Option<Arc<SearchIndex>>>,
    book: Mutex<Book>,
    /// Mail changed since the book was read.
    stale: AtomicBool,
    /// The last results, for GNOME's `GetResultMetas`.
    recent: Mutex<HashMap<String, Found>>,
    /// KRunner's token for the window the next `Run` opens (Wayland).
    token: Mutex<Option<String>>,
    /// Words that search all the mail (`general.search_triggers`).
    triggers: Mutex<Vec<String>>,
}

#[derive(Default)]
struct Book {
    book: Option<Arc<ContactBook>>,
    read_at: Option<Instant>,
    reading: bool,
}

impl Finder {
    pub(crate) fn new(paths: Paths, triggers: Vec<String>) -> Arc<Self> {
        Arc::new(Self {
            triggers: Mutex::new(triggers),
            paths,
            store: Mutex::default(),
            index: Mutex::default(),
            book: Mutex::default(),
            stale: AtomicBool::new(false),
            recent: Mutex::default(),
            token: Mutex::default(),
        })
    }

    /// Reads the address book ahead of the first search, so the first
    /// letters typed find people already.
    pub(crate) fn warm_up(self: &Arc<Self>) {
        self.book();
    }

    /// The trigger words changed in Settings.
    pub(crate) fn set_triggers(&self, triggers: Vec<String>) {
        *lock(&self.triggers) = triggers;
    }

    /// What follows `mail:` or a trigger word at the start of `text`.
    fn strip_trigger<'a>(&self, text: &'a str) -> Option<&'a str> {
        strip_trigger(text, &lock(&self.triggers))
    }

    /// Mail changed: the book is read again at the next search, once it is
    /// [`BOOK_AGE`] old.
    pub(crate) fn mail_changed(&self) {
        self.stale.store(true, Ordering::Relaxed);
    }

    /// The results for `text`, best first, at most `limit`.
    pub(crate) fn find(self: &Arc<Self>, text: &str, limit: usize) -> Vec<Found> {
        let started = Instant::now();
        let text = text.trim();
        let mut found = Vec::new();
        if let Some(rest) = self.strip_trigger(text) {
            if rest.chars().count() >= MIN_CHARS {
                found = self.mail(rest, true);
            }
        } else if text.chars().count() >= MIN_CHARS {
            found = self.contacts(text);
            found.extend(self.tasks(text, None));
            found.extend(self.events(text, None));
            found.extend(self.mail(text, false));
        }
        found.sort_by(|a, b| {
            b.kind
                .cmp(&a.kind)
                .then_with(|| b.relevance.total_cmp(&a.relevance))
        });
        found.truncate(limit);
        let mut recent = lock(&self.recent);
        if recent.len() > 200 {
            recent.clear();
        }
        for result in &found {
            recent.insert(result.id.clone(), result.clone());
        }
        tracing::debug!(
            results = found.len(),
            ms = started.elapsed().as_secs_f32() * 1000.0,
            "desktop search"
        );
        found
    }

    /// A result seen lately, or looked up again.
    pub(crate) fn result(&self, id: &str) -> Option<Found> {
        if let Some(found) = lock(&self.recent).get(id) {
            return Some(found.clone());
        }
        match parse_id(id)? {
            Target::Mail(message) => self.messages(&[message], None).pop(),
            Target::Task(task) => self.tasks("", Some(task)).pop(),
            Target::Event(event, start) => self.events("", Some((event, start))).pop(),
            Target::Contact(email) => {
                let book = lock(&self.book).book.clone()?;
                let contact = book.contacts().iter().find(|c| c.email == email)?;
                Some(contact_result(
                    &contact.email,
                    contact.name.as_deref(),
                    0.0,
                    false,
                ))
            }
        }
    }

    /// People whose name or address starts with the words of `text`.
    fn contacts(self: &Arc<Self>, text: &str) -> Vec<Found> {
        if text.contains(':') {
            return Vec::new();
        }
        let Some(book) = self.book() else {
            return Vec::new();
        };
        let wanted = text.to_lowercase();
        book.suggest(text, None, unix_now(), &[], CONTACTS)
            .into_iter()
            .map(|s| {
                let exact = s.email == wanted
                    || s.name
                        .as_deref()
                        .is_some_and(|n| n.to_lowercase() == wanted);
                contact_result(&s.email, s.name.as_deref(), s.score, exact)
            })
            .collect()
    }

    /// Open tasks with each word of `text` starting a word of their title
    /// or details, those due first first; or only task `only`.
    fn tasks(&self, text: &str, only: Option<i64>) -> Vec<Found> {
        if only.is_none() && text.contains(':') {
            return Vec::new();
        }
        let words: Vec<String> = text.split_whitespace().map(str::to_lowercase).collect();
        let read = |store: &Store| -> katna_store::Result<_> {
            Ok((store.tasks(unix_now())?, store.task_lists()?))
        };
        let Some((tasks, lists)) = self.read_store("tasks", read) else {
            return Vec::new();
        };
        let mut matched: Vec<_> = tasks
            .into_iter()
            .filter(|t| t.done_at.is_none() && !t.title.trim().is_empty())
            .filter(|t| match only {
                Some(id) => t.id == id,
                None => starts_words(&format!("{} {}", t.title, t.notes).to_lowercase(), &words),
            })
            .collect();
        // Those due first, then the rest.
        matched.sort_by(|a, b| {
            (a.due.is_empty(), &a.due, a.due_time).cmp(&(b.due.is_empty(), &b.due, b.due_time))
        });
        matched.truncate(TASKS);
        let wanted = words.join(" ");
        matched
            .into_iter()
            .enumerate()
            .map(|(rank, task)| {
                let list = lists
                    .iter()
                    .find(|l| l.id == task.list)
                    .map(|l| l.title.trim())
                    .filter(|t| !t.is_empty());
                let exact = task.title.trim().to_lowercase() == wanted;
                Found {
                    id: format!("{TASK_ID}{}", task.id),
                    text: task.title.trim().to_owned(),
                    subtext: match list {
                        Some(list) => tr!("search-task-in", list = list),
                        None => String::new(),
                    },
                    icon: "view-task",
                    kind: if exact { EXACT_MATCH } else { POSSIBLE_MATCH },
                    // Below people, above mail.
                    relevance: 0.58 - rank as f64 * 0.01,
                }
            })
            .collect()
    }

    /// `read` from the store, opened read-only on first use; `None` when
    /// it fails.
    fn read_store<T>(
        &self,
        what: &str,
        read: impl FnOnce(&Store) -> katna_store::Result<T>,
    ) -> Option<T> {
        let mut guard = lock(&self.store);
        if guard.is_none() {
            match Store::open(&self.paths, Mode::ReadOnly) {
                Ok(store) => *guard = Some(store),
                Err(err) => {
                    tracing::warn!(%err, "desktop search: opening the store");
                    return None;
                }
            }
        }
        match read(guard.as_ref()?) {
            Ok(read) => Some(read),
            Err(err) => {
                tracing::debug!(%err, what, "desktop search: reading");
                None
            }
        }
    }

    /// The next occurrences of events in the coming year with each word
    /// of `text` starting a word of their title, place or details, soonest
    /// first, one for each event; or only the occurrence `only`.
    fn events(&self, text: &str, only: Option<(i64, i64)>) -> Vec<Found> {
        if only.is_none() && text.contains(':') {
            return Vec::new();
        }
        let words: Vec<String> = text.split_whitespace().map(str::to_lowercase).collect();
        let tz = TimeZone::system();
        let now = unix_now();
        let (from, to) = match only {
            Some((_, start)) => (start, start.saturating_add(1)),
            None => (now, now.saturating_add(EVENT_DAYS * DAY)),
        };
        let read = |store: &Store| -> katna_store::Result<_> {
            Ok((store.event_rows_in_range(from, to)?, store.calendars()?))
        };
        let Some((rows, calendars)) = self.read_store("events", read) else {
            return Vec::new();
        };
        let matches = |data: &EventData| {
            starts_words(
                &format!("{} {} {}", data.title, data.location, data.description).to_lowercase(),
                &words,
            )
        };
        // Only the events that match, with their changed occurrences.
        let uids: HashSet<&str> = rows
            .iter()
            .filter(|row| only.is_some() || matches(&row.data))
            .map(|row| row.data.uid.as_str())
            .collect();
        let rows: Vec<StoredEvent> = rows
            .iter()
            .filter(|row| uids.contains(row.data.uid.as_str()))
            .cloned()
            .collect();
        let mut seen = HashSet::new();
        let found: Vec<_> = katna_dav::occurrences(rows, from, to, &tz)
            .into_iter()
            .filter(|o| {
                let data = &o.event.data;
                data.status != EventStatus::Cancelled
                    && !data.title.trim().is_empty()
                    && match only {
                        Some((id, start)) => o.event.id == id && o.start == start,
                        None => o.end > now && matches(data),
                    }
            })
            // Occurrences come soonest first; keep each event's first.
            .filter(|o| seen.insert(o.event.data.uid.clone()))
            .take(EVENTS)
            .collect();
        let wanted = words.join(" ");
        found
            .into_iter()
            .enumerate()
            .map(|(rank, o)| {
                let data = &o.event.data;
                let when = event_when(o.start, o.end, data.all_day, now, &tz);
                let place = data.location.trim();
                let calendar = calendars
                    .iter()
                    .find(|c| c.id == o.event.calendar_id)
                    .map(|c| c.name.trim())
                    .filter(|n| !n.is_empty());
                let subtext = match (place.is_empty(), calendar) {
                    (false, _) => tr!("search-event-at", when = when, place = place),
                    (true, Some(calendar)) => {
                        tr!("search-event-in", when = when, calendar = calendar)
                    }
                    (true, None) => when,
                };
                let exact = data.title.trim().to_lowercase() == wanted;
                Found {
                    id: format!("{EVENT_ID}{}:{}", o.event.id, o.start),
                    text: data.title.trim().to_owned(),
                    subtext,
                    icon: "view-calendar-day",
                    kind: if exact { EXACT_MATCH } else { POSSIBLE_MATCH },
                    // Below people and tasks, above mail.
                    relevance: 0.55 - rank as f64 * 0.01,
                }
            })
            .collect()
    }

    /// Mail for `text`: the search box's results with `all`, else only
    /// messages with every word in the subject or the sender.
    fn mail(&self, text: &str, all: bool) -> Vec<Found> {
        let query = if all {
            match Query::parse_as_you_type(text, unix_now()) {
                Ok(query) if !text.contains("in:") => not_deleted(query),
                Ok(query) => query,
                Err(_) => return Vec::new(),
            }
        } else {
            match confident_query(text) {
                Some(query) => query,
                None => return Vec::new(),
            }
        };
        let Some(index) = self.index() else {
            return Vec::new();
        };
        let limit = if all { ALL_MAIL } else { CONFIDENT_MAIL };
        let options = SearchOptions {
            // Room for several messages of one conversation.
            limit: limit * 4,
            sort: Sort::Relevance,
            ..SearchOptions::default()
        };
        let results = match index.search(&query, &options) {
            Ok(results) => results,
            Err(err) => {
                tracing::warn!(%err, "desktop search");
                return Vec::new();
            }
        };
        // Words a typo away are not what was asked for.
        if results.fuzzy && !all {
            return Vec::new();
        }
        let ids: Vec<MessageId> = results.hits.iter().map(|hit| hit.message).collect();
        // The index also finds names a typo away ("kenn" finds "Kean");
        // only words that start as typed are sure.
        let words: Vec<String> = text.split_whitespace().map(str::to_lowercase).collect();
        let mut found = self.messages(&ids, (!all).then_some(words.as_slice()));
        found.truncate(limit);
        let (kind, top) = if all {
            (EXACT_MATCH, 0.9)
        } else {
            (POSSIBLE_MATCH, 0.5)
        };
        for (rank, result) in found.iter_mut().enumerate() {
            result.kind = kind;
            result.relevance = top - rank as f64 * 0.01;
        }
        found
    }

    /// Results for `ids`, in that order, one per conversation; with
    /// `words`, only messages with a word in the subject or the sender
    /// starting with each of them.
    fn messages(&self, ids: &[MessageId], words: Option<&[String]>) -> Vec<Found> {
        let mut guard = lock(&self.store);
        if guard.is_none() {
            match Store::open(&self.paths, Mode::ReadOnly) {
                Ok(store) => *guard = Some(store),
                Err(err) => {
                    tracing::warn!(%err, "desktop search: opening the store");
                    return Vec::new();
                }
            }
        }
        let Some(store) = guard.as_ref() else {
            return Vec::new();
        };
        let stored = match store.messages_by_id(ids) {
            Ok(stored) => stored,
            Err(err) => {
                tracing::warn!(%err, "desktop search: reading messages");
                *guard = None;
                return Vec::new();
            }
        };
        let mut threads = Vec::new();
        let mut found = Vec::new();
        for message in stored {
            let from = message.first(ParticipantRole::From);
            if let Some(words) = words {
                let mut text = message.subject.to_lowercase();
                if let Some(from) = from {
                    text.push(' ');
                    text.push_str(&from.email_norm);
                    if let Some(name) = &from.display_name {
                        text.push(' ');
                        text.push_str(&name.to_lowercase());
                    }
                }
                if !starts_words(&text, words) {
                    continue;
                }
            }
            if let Some(thread) = message.thread_id {
                if threads.contains(&thread) {
                    continue;
                }
                threads.push(thread);
            }
            let sender = from.map(|p| {
                p.display_name
                    .clone()
                    .unwrap_or_else(|| p.email_norm.clone())
            });
            let subject = message.subject.trim();
            found.push(Found {
                id: format!("{MAIL_ID}{}", message.id.0),
                text: if subject.is_empty() {
                    tr!("search-no-subject")
                } else {
                    subject.to_owned()
                },
                subtext: sender
                    .map(|sender| tr!("search-mail-from", sender = sender))
                    .unwrap_or_default(),
                icon: if message.flags.contains(MessageFlags::SEEN) {
                    "mail-read"
                } else {
                    "mail-unread"
                },
                kind: POSSIBLE_MATCH,
                relevance: 0.5,
            });
        }
        found
    }

    /// The index, opened read-only as the apps do (the indexer writes it).
    fn index(&self) -> Option<Arc<SearchIndex>> {
        let mut guard = lock(&self.index);
        if guard.is_none() {
            match SearchIndex::open_read_only(&self.paths.index_dir()) {
                Ok(index) => *guard = Some(Arc::new(index)),
                Err(err) => tracing::debug!(%err, "desktop search: no index yet"),
            }
        }
        guard.clone()
    }

    /// The address book; read on a thread of its own the first time (a
    /// few seconds on a big mailbox) and when mail changed, meanwhile the
    /// old one answers.
    fn book(self: &Arc<Self>) -> Option<Arc<ContactBook>> {
        let mut book = lock(&self.book);
        let old = book
            .read_at
            .is_none_or(|at| self.stale.load(Ordering::Relaxed) && at.elapsed() > BOOK_AGE);
        if old && !book.reading {
            book.reading = true;
            self.stale.store(false, Ordering::Relaxed);
            let finder = self.clone();
            let started = std::thread::Builder::new()
                .name("katna-address-book".into())
                .spawn(move || finder.read_book());
            if let Err(err) = started {
                tracing::warn!(%err, "desktop search: cannot read the address book");
                book.reading = false;
            }
        }
        book.book.clone()
    }

    fn read_book(&self) {
        let started = Instant::now();
        let read = Store::open(&self.paths, Mode::ReadOnly).and_then(|store| {
            let rows = store.correspondents()?;
            // Saved contacts' names, and saved people never written to;
            // an older store without contacts still answers.
            let saved = store.saved_names().unwrap_or_default();
            Ok(ContactBook::with_saved(rows, saved))
        });
        let mut book = lock(&self.book);
        book.reading = false;
        book.read_at = Some(Instant::now());
        match read {
            Ok(read) => {
                tracing::debug!(
                    addresses = read.len(),
                    seconds = started.elapsed().as_secs_f32(),
                    "desktop search: read the address book"
                );
                book.book = Some(Arc::new(read));
            }
            Err(err) => tracing::warn!(%err, "desktop search: reading the address book"),
        }
    }

    /// Does what the result `id` stands for: `action` is one of KRunner's
    /// action IDs, or empty for the result's own (open the message, write
    /// to the person).
    pub(crate) async fn run(&self, connection: &zbus::Connection, id: &str, action: &str) {
        let token = lock(&self.token).take();
        match (parse_id(id), action) {
            (Some(Target::Mail(message)), "" | REPLY_ALL) => {
                let name = if action.is_empty() {
                    app_action::OPEN_MESSAGE
                } else {
                    app_action::REPLY_ALL
                };
                mail_app::run(connection, Some(name), vec![Value::from(message.0)], token).await;
            }
            (Some(Target::Contact(email)), "") => {
                mail_app::open_mailto(connection, &mailto(&email), token).await;
            }
            (Some(Target::Contact(email)), COPY) => copy(connection, &email).await,
            (Some(Target::Task(task)), "") => {
                let page = vec![Value::from(format!("tasks:{task}"))];
                mail_app::run(connection, Some(app_action::OPEN_PAGE), page, token).await;
            }
            (Some(Target::Event(_, start)), "") => {
                let Ok(start) = jiff::Timestamp::from_second(start) else {
                    return;
                };
                let day = start.to_zoned(TimeZone::system()).date().to_string();
                let page = vec![Value::from(app_action::calendar_page(&day, false))];
                mail_app::run(connection, Some(app_action::OPEN_PAGE), page, token).await;
            }
            (Some(Target::Contact(email)), FIND) => {
                let query = format!("from:{email} OR to:{email}");
                search(connection, &query, token).await;
            }
            _ => tracing::warn!(id, action, "desktop search: unknown result or action"),
        }
    }

    /// Opens Katna Mail searching for `text`, the words typed in the
    /// desktop's search.
    pub(crate) async fn launch_search(&self, connection: &zbus::Connection, text: &str) {
        let token = lock(&self.token).take();
        let text = self.strip_trigger(text.trim()).unwrap_or(text.trim());
        search(connection, text, token).await;
    }

    pub(crate) fn set_token(&self, token: String) {
        *lock(&self.token) = Some(token);
    }

    /// KRunner closed: forget what only helped while typing.
    pub(crate) fn teardown(&self) {
        lock(&self.recent).clear();
        *lock(&self.token) = None;
    }
}

fn contact_result(email: &str, name: Option<&str>, score: f64, exact: bool) -> Found {
    let name = name.filter(|n| !n.trim().is_empty());
    Found {
        id: format!("{CONTACT_ID}{email}"),
        text: name.unwrap_or(email).to_owned(),
        subtext: if name.is_some() {
            email.to_owned()
        } else {
            String::new()
        },
        icon: "user-identity",
        kind: if exact { EXACT_MATCH } else { POSSIBLE_MATCH },
        // Above mail; the people written with most first.
        relevance: 0.6 + 0.35 * score / (1.0 + score),
    }
}

enum Target {
    Mail(MessageId),
    Contact(String),
    Task(i64),
    /// An event's row and the start of the occurrence found.
    Event(i64, i64),
}

/// When an occurrence is, from `now`: "Now", "Today", "Tomorrow" or "In 3
/// days" (the service formats no dates).
fn event_when(start: i64, end: i64, all_day: bool, now: i64, tz: &TimeZone) -> String {
    if !all_day && start <= now && now < end {
        return tr!("search-event-now");
    }
    let day = |at: i64| {
        jiff::Timestamp::from_second(at)
            .map(|t| {
                if all_day {
                    // Whole days are UTC midnights.
                    t.to_zoned(TimeZone::UTC).date()
                } else {
                    t.to_zoned(tz.clone()).date()
                }
            })
            .ok()
    };
    let today = jiff::Timestamp::from_second(now)
        .ok()
        .map(|t| t.to_zoned(tz.clone()).date());
    let days = match (day(start), today) {
        (Some(start), Some(today)) => (start - today).get_days().max(0),
        _ => 0,
    };
    match days {
        0 => tr!("search-event-today"),
        1 => tr!("search-event-tomorrow"),
        count => tr!("search-event-in-days", count = count),
    }
}

fn parse_id(id: &str) -> Option<Target> {
    let mut chars = id.chars();
    match chars.next()? {
        MAIL_ID => chars
            .as_str()
            .parse()
            .ok()
            .map(|id| Target::Mail(MessageId(id))),
        CONTACT_ID if chars.as_str().contains('@') => Some(Target::Contact(chars.as_str().into())),
        TASK_ID => chars.as_str().parse().ok().map(Target::Task),
        EVENT_ID => {
            let (event, start) = chars.as_str().split_once(':')?;
            Some(Target::Event(event.parse().ok()?, start.parse().ok()?))
        }
        _ => None,
    }
}

/// What follows `mail:`, or one of `triggers` and a space or a colon, at
/// the start of `text`, in any case.
fn strip_trigger<'a>(text: &'a str, triggers: &[String]) -> Option<&'a str> {
    let starts = |word: &str| {
        let head = text.get(..word.len())?;
        (head.to_lowercase() == word.to_lowercase()).then(|| &text[word.len()..])
    };
    if let Some(rest) = starts(MAIL_PREFIX) {
        return Some(rest.trim());
    }
    triggers
        .iter()
        .map(|word| word.trim_end_matches(':'))
        .filter(|word| !word.is_empty())
        .find_map(|word| {
            let rest = starts(word)?;
            rest.starts_with(|c: char| c.is_whitespace() || c == ':')
                .then(|| rest.trim_start_matches(':').trim())
        })
}

/// Messages with every word of `text` in the subject or the sender, not in
/// the trash or spam; `None` when `text` is more than plain words.
fn confident_query(text: &str) -> Option<Query> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let plain = |word: &&str| {
        word.chars().count() >= MIN_CHARS && word.chars().all(|c| c.is_alphanumeric())
    };
    if words.is_empty() || !words.iter().all(plain) {
        return None;
    }
    let items = words
        .iter()
        .map(|word| {
            let text = |field| Query::Text {
                field,
                text: (*word).to_owned(),
            };
            Query::Or(vec![text(TextField::Subject), text(TextField::From)])
        })
        .collect();
    Some(not_deleted(Query::And(items)))
}

/// Whether each of `words` starts a word of `text`, all lower case.
fn starts_words(text: &str, words: &[String]) -> bool {
    words.iter().all(|word| {
        text.split(|c: char| !c.is_alphanumeric())
            .any(|part| part.starts_with(word.as_str()))
    })
}

fn not_deleted(query: Query) -> Query {
    let not_in = |folder: &str| Query::Not(Box::new(Query::Filter(Filter::In(folder.into()))));
    Query::And(vec![query, not_in("trash"), not_in("junk")])
}

/// A `mailto:` link to `email`, which is written as is except for
/// characters a link cannot hold.
fn mailto(email: &str) -> String {
    let mut link = String::from("mailto:");
    for byte in email.bytes() {
        if byte.is_ascii_alphanumeric() || b"@.-_~".contains(&byte) {
            link.push(byte as char);
        } else {
            link.push_str(&format!("%{byte:02X}"));
        }
    }
    link
}

async fn search(connection: &zbus::Connection, text: &str, token: Option<String>) {
    let params = vec![Value::from(text)];
    mail_app::run(connection, Some(app_action::SEARCH), params, token).await;
}

/// Puts `text` on the clipboard through Plasma's clipboard (Klipper); the
/// daemon has no window to own the clipboard with.
async fn copy(connection: &zbus::Connection, text: &str) {
    let copied = connection
        .call_method(
            Some("org.kde.klipper"),
            "/klipper",
            Some("org.kde.klipper.klipper"),
            "setClipboardContents",
            &(text,),
        )
        .await;
    if let Err(err) = copied {
        tracing::warn!(%err, "desktop search: could not copy the address");
    }
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|e| e.into_inner())
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64)
}

/// A KRunner match: ID, text, icon, type, relevance and properties.
type Match = (
    String,
    String,
    String,
    i32,
    f64,
    HashMap<String, Value<'static>>,
);

/// `org.kde.krunner1` at [`ids::RUNNER_OBJECT_PATH`].
pub(crate) struct Runner {
    finder: Arc<Finder>,
}

#[zbus::interface(name = "org.kde.krunner1")]
impl Runner {
    async fn actions(&self) -> Vec<(String, String, String)> {
        vec![
            (
                REPLY_ALL.into(),
                tr!("search-reply-all"),
                "mail-reply-all".into(),
            ),
            (COPY.into(), tr!("search-copy-address"), "edit-copy".into()),
            (FIND.into(), tr!("search-find-mail"), "edit-find".into()),
        ]
    }

    #[zbus(name = "Match")]
    async fn find(&self, query: String) -> Vec<Match> {
        let finder = self.finder.clone();
        let found = smol::unblock(move || finder.find(&query, ALL_MAIL + CONTACTS)).await;
        found.into_iter().map(krunner_match).collect()
    }

    async fn run(
        &self,
        #[zbus(connection)] connection: &zbus::Connection,
        match_id: String,
        action_id: String,
    ) {
        self.finder.run(connection, &match_id, &action_id).await;
    }

    async fn set_activation_token(&self, token: String) {
        self.finder.set_token(token);
    }

    async fn teardown(&self) {
        self.finder.teardown();
    }
}

fn krunner_match(found: Found) -> Match {
    let (category, actions) = if found.is_mail() {
        (tr!("search-category-mail"), vec![REPLY_ALL])
    } else if found.is_task() {
        (tr!("search-category-tasks"), vec![])
    } else if found.is_event() {
        (tr!("search-category-events"), vec![])
    } else {
        (tr!("search-category-people"), vec![COPY, FIND])
    };
    let mut properties = HashMap::new();
    properties.insert("subtext".to_owned(), Value::from(found.subtext));
    properties.insert("category".to_owned(), Value::from(category));
    properties.insert("actions".to_owned(), Value::from(actions));
    (
        found.id,
        found.text,
        found.icon.to_owned(),
        found.kind,
        found.relevance,
        properties,
    )
}

/// `org.gnome.Shell.SearchProvider2` at [`ids::SEARCH_PROVIDER_OBJECT_PATH`].
pub(crate) struct SearchProvider {
    finder: Arc<Finder>,
}

/// GNOME shows a few results per app.
const GNOME_RESULTS: usize = 5;

#[zbus::interface(name = "org.gnome.Shell.SearchProvider2")]
impl SearchProvider {
    async fn get_initial_result_set(&self, terms: Vec<String>) -> Vec<String> {
        self.ids(terms).await
    }

    async fn get_subsearch_result_set(
        &self,
        _previous_results: Vec<String>,
        terms: Vec<String>,
    ) -> Vec<String> {
        self.ids(terms).await
    }

    async fn get_result_metas(
        &self,
        identifiers: Vec<String>,
    ) -> Vec<HashMap<String, Value<'static>>> {
        let finder = self.finder.clone();
        smol::unblock(move || {
            identifiers
                .iter()
                .filter_map(|id| finder.result(id))
                .map(|found| {
                    HashMap::from([
                        ("id".to_owned(), Value::from(found.id)),
                        ("name".to_owned(), Value::from(found.text)),
                        ("description".to_owned(), Value::from(found.subtext)),
                        ("gicon".to_owned(), Value::from(found.icon)),
                    ])
                })
                .collect()
        })
        .await
    }

    async fn activate_result(
        &self,
        #[zbus(connection)] connection: &zbus::Connection,
        identifier: String,
        _terms: Vec<String>,
        _timestamp: u32,
    ) {
        self.finder.run(connection, &identifier, "").await;
    }

    async fn launch_search(
        &self,
        #[zbus(connection)] connection: &zbus::Connection,
        terms: Vec<String>,
        _timestamp: u32,
    ) {
        self.finder
            .launch_search(connection, &terms.join(" "))
            .await;
    }
}

impl SearchProvider {
    async fn ids(&self, terms: Vec<String>) -> Vec<String> {
        let finder = self.finder.clone();
        let found = smol::unblock(move || finder.find(&terms.join(" "), GNOME_RESULTS)).await;
        found.into_iter().map(|found| found.id).collect()
    }
}

/// Serves both interfaces on `connection`.
pub(crate) async fn serve(connection: &zbus::Connection, finder: Arc<Finder>) -> zbus::Result<()> {
    let server = connection.object_server();
    server
        .at(
            ids::RUNNER_OBJECT_PATH,
            Runner {
                finder: finder.clone(),
            },
        )
        .await?;
    server
        .at(ids::SEARCH_PROVIDER_OBJECT_PATH, SearchProvider { finder })
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_round_trip() {
        assert!(matches!(parse_id("m42"), Some(Target::Mail(MessageId(42)))));
        assert!(
            matches!(parse_id("cada@example.org"), Some(Target::Contact(e)) if e == "ada@example.org")
        );
        assert!(matches!(parse_id("t7"), Some(Target::Task(7))));
        assert!(matches!(
            parse_id("e3:1790000000"),
            Some(Target::Event(3, 1_790_000_000))
        ));
        assert!(parse_id("e3").is_none());
        assert!(parse_id("cnobody").is_none());
        assert!(parse_id("x1").is_none());
        assert!(parse_id("").is_none());
    }

    #[test]
    fn events_say_how_soon_they_are() {
        let tz = TimeZone::get("Asia/Kolkata").unwrap();
        let at = |d: i8, h: i8| {
            jiff::civil::date(2026, 9, d)
                .at(h, 0, 0, 0)
                .to_zoned(tz.clone())
                .unwrap()
                .timestamp()
                .as_second()
        };
        let now = at(29, 10);
        assert_eq!(event_when(at(29, 9), at(29, 11), false, now, &tz), "Now");
        assert_eq!(event_when(at(29, 23), at(30, 0), false, now, &tz), "Today");
        assert_eq!(
            event_when(at(30, 1), at(30, 2), false, now, &tz),
            "Tomorrow"
        );
        // A whole day is a UTC midnight, whatever the zone.
        let midnight = jiff::civil::date(2026, 10, 2)
            .at(0, 0, 0, 0)
            .to_zoned(TimeZone::UTC)
            .unwrap()
            .timestamp()
            .as_second();
        assert_eq!(
            event_when(midnight, midnight + DAY, true, now, &tz),
            "In 3 days"
        );
    }

    #[test]
    fn mail_prefix_and_trigger_words_are_found_in_any_case() {
        let none: &[String] = &[];
        assert_eq!(strip_trigger("mail: budget", none), Some("budget"));
        assert_eq!(strip_trigger("Mail:budget", none), Some("budget"));
        assert_eq!(strip_trigger("mai", none), None);
        assert_eq!(strip_trigger("budget", none), None);
        // Not a character boundary at the prefix's length.
        assert_eq!(strip_trigger("মেইল", none), None);

        let triggers = ["k".to_owned(), "m".to_owned(), "মেইল".to_owned()];
        assert_eq!(strip_trigger("k budget", &triggers), Some("budget"));
        assert_eq!(strip_trigger("K  budget ", &triggers), Some("budget"));
        assert_eq!(strip_trigger("m:budget", &triggers), Some("budget"));
        assert_eq!(strip_trigger("মেইল বাজেট", &triggers), Some("বাজেট"));
        assert_eq!(strip_trigger("mail: budget", &triggers), Some("budget"));
        // A word that only starts with a trigger is a plain search.
        assert_eq!(strip_trigger("kenneth", &triggers), None);
        assert_eq!(strip_trigger("mark", &triggers), None);
        assert_eq!(strip_trigger("k", &triggers), None);
        assert_eq!(strip_trigger("k ", &triggers), Some(""));
    }

    #[test]
    fn only_plain_words_make_a_confident_query() {
        assert!(confident_query("budget meeting").is_some());
        assert!(confident_query("bu").is_none());
        assert!(confident_query("budget to").is_none());
        assert!(confident_query("from:ada").is_none());
        assert!(confident_query("-budget").is_none());
        assert!(confident_query("").is_none());
    }

    #[test]
    fn words_must_start_as_typed() {
        let words = |text: &str| -> Vec<String> { text.split(' ').map(str::to_owned).collect() };
        let text = "2001 budget review kenneth.lay@enron.com kenneth lay";
        assert!(starts_words(text, &words("kenn budget")));
        assert!(starts_words(text, &words("enron")));
        assert!(!starts_words(text, &words("kean")));
        assert!(!starts_words(text, &words("udget")));
    }

    #[test]
    fn mailto_links_escape_what_they_must() {
        assert_eq!(mailto("ada@example.org"), "mailto:ada@example.org");
        assert_eq!(mailto("a+b@x.org"), "mailto:a%2Bb@x.org");
        assert_eq!(mailto("a b?@x.org"), "mailto:a%20b%3F@x.org");
    }
}
