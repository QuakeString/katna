// SPDX-License-Identifier: GPL-3.0-or-later

//! Reading `mail.db` for the mail app: the folder list with counts and the
//! messages of one folder. Works in both read-write and read-only mode.

use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use katna_core::{AccountId, MailCategory};
use rusqlite::{Connection, params};

use crate::error::Result;
use crate::mail::{FolderId, MessageFlags, MessageId, ThreadId};

/// A folder with its message count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderSummary {
    pub id: FolderId,
    pub account: AccountId,
    pub path: String,
    /// `folder.role`: `inbox`, `sent`, ... or `None`.
    pub role: Option<String>,
    pub total: u64,
}

/// All folders of all accounts, ordered by account and path, with their
/// message counts. Reads only the location index, so it stays fast with
/// hundreds of thousands of messages.
pub(crate) fn folders(conn: &Connection) -> Result<Vec<FolderSummary>> {
    let mut stmt = conn.prepare_cached(
        "SELECT f.id, f.account_id, f.path, f.role, coalesce(c.total, 0)
         FROM folder f
         LEFT JOIN (
             SELECT folder_id, count(*) AS total FROM message_location GROUP BY folder_id
         ) c ON c.folder_id = f.id
         ORDER BY f.account_id, f.path",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(FolderSummary {
            id: FolderId(row.get(0)?),
            account: AccountId(row.get(1)?),
            path: row.get(2)?,
            role: row.get(3)?,
            total: row.get::<_, i64>(4)?.try_into().unwrap_or_default(),
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// Unread messages per folder; folders without unread mail are left out.
///
/// Scans every message's flags, so it takes a while on big stores (about
/// 100 ms per 100,000 unread messages); the app runs it in the background.
pub(crate) fn unread_counts(conn: &Connection) -> Result<Vec<(FolderId, u64)>> {
    let mut stmt = conn.prepare_cached(
        "SELECT folder_id, count(*) FROM message_location
         WHERE message_id IN (SELECT id FROM message WHERE (flags & ?1) = 0)
         GROUP BY folder_id ORDER BY folder_id",
    )?;
    let rows = stmt.query_map([MessageFlags::SEEN.bits()], |row| {
        Ok((
            FolderId(row.get(0)?),
            row.get::<_, i64>(1)?.try_into().unwrap_or_default(),
        ))
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// The messages in `folder`, newest first. Messages without a date come last.
pub(crate) fn folder_message_ids(conn: &Connection, folder: FolderId) -> Result<Vec<MessageId>> {
    // Sorted here, as in `folder_rows`.
    let mut stmt = conn.prepare_cached(
        "SELECT l.message_id, m.date FROM message_location l JOIN message m ON m.id = l.message_id
         WHERE l.folder_id = ?1",
    )?;
    let rows = stmt.query_map([folder.0], |row| Ok((row.get::<_, i64>(0)?, row.get(1)?)))?;
    let mut rows = rows.collect::<rusqlite::Result<Vec<(i64, Option<i64>)>>>()?;
    rows.sort_unstable_by_key(|(id, date)| newest_first(*date, *id));
    Ok(rows.into_iter().map(|(id, _)| MessageId(id)).collect())
}

/// One conversation in a folder's list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadEntry {
    /// `None` for a message that has no thread yet (stores from before
    /// threading, until the background backfill reaches it): show it as a
    /// conversation of one.
    pub thread: Option<ThreadId>,
    /// The newest message of the conversation that is in the folder.
    pub latest: MessageId,
}

/// A sender in a conversation, for "Alice, Bob, me (3)".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadSender {
    /// Lower-case address; compare with the account's to show "me".
    pub email: String,
    pub name: Option<String>,
    /// Some message from this sender is unread.
    pub unread: bool,
}

/// What a folder's list shows for a conversation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadSummary {
    pub thread: ThreadId,
    /// Messages in the whole conversation, in every folder, as
    /// [`thread_messages`](crate::Store::thread_messages) counts them.
    pub message_count: u32,
    /// Some message of the conversation in this folder is unread.
    pub unread: bool,
    /// Some message of the conversation is flagged.
    pub flagged: bool,
    /// Some message of the conversation is marked important.
    pub important: bool,
    /// Some message of the conversation has attachments.
    pub has_attachments: bool,
    /// Distinct `From` addresses, in the order they first wrote.
    pub senders: Vec<ThreadSender>,
    /// The `From` addresses of the newest message that is not a draft,
    /// to tell whether the user wrote last.
    pub last_from: Vec<String>,
    /// The newest message that is not a draft is marked answered.
    pub last_answered: bool,
}

/// One row of a folder scan: message, thread (or none) and category.
struct FolderRow {
    id: i64,
    thread: Option<i64>,
    category: Option<MailCategory>,
    unread: bool,
}

/// Every message in `folder`, newest first (undated last).
fn folder_rows(conn: &Connection, folder: FolderId) -> Result<Vec<FolderRow>> {
    // Sorted here rather than by SQLite: no index orders a folder by date,
    // and SQLite's sorter takes about as long again as reading the rows
    // (60 ms of 110 ms for a 100,000-message inbox).
    let mut stmt = conn.prepare_cached(
        "SELECT m.id, m.thread_id, m.category, m.flags, m.date FROM message_location l
         JOIN message m ON m.id = l.message_id
         WHERE l.folder_id = ?1",
    )?;
    let rows = stmt.query_map([folder.0], |row| {
        let flags: i64 = row.get(3)?;
        let date: Option<i64> = row.get(4)?;
        let row = FolderRow {
            id: row.get(0)?,
            thread: row.get(1)?,
            category: row
                .get::<_, Option<i64>>(2)?
                .and_then(MailCategory::from_storage),
            unread: flags & i64::from(MessageFlags::SEEN.bits()) == 0,
        };
        Ok((date, row))
    })?;
    let mut rows = rows.collect::<rusqlite::Result<Vec<_>>>()?;
    rows.sort_unstable_by_key(|(date, row)| newest_first(*date, row.id));
    Ok(rows.into_iter().map(|(_, row)| row).collect())
}

/// The order of a folder's list: newest first, then the most recently
/// stored; undated messages last. As `ORDER BY date IS NULL, date DESC,
/// id DESC` would sort them.
fn newest_first(date: Option<i64>, id: i64) -> (bool, Reverse<Option<i64>>, Reverse<i64>) {
    (date.is_none(), Reverse(date), Reverse(id))
}

/// Unclassified messages count as Primary.
fn tab(category: Option<MailCategory>) -> MailCategory {
    category.unwrap_or(MailCategory::Primary)
}

/// The conversations in `folder`, newest first, each with its newest
/// message in the folder. With `categories`, only conversations whose newest
/// message in the folder has one of them (unclassified counts as Primary),
/// so each conversation is in exactly one tab.
pub(crate) fn folder_threads(
    conn: &Connection,
    folder: FolderId,
    categories: Option<&[MailCategory]>,
) -> Result<Vec<ThreadEntry>> {
    Ok(threads_of(&folder_rows(conn, folder)?, categories))
}

/// An inbox's conversations, as [`folder_threads`] lists them, and its
/// unread conversations per tab, as [`category_unread`] counts them.
pub type InboxThreads = (Vec<ThreadEntry>, Vec<(MailCategory, u64)>);

/// [`folder_threads`] and [`category_unread`] of an inbox together, from
/// one read of the folder: each read sorts the whole folder.
pub(crate) fn inbox_threads(
    conn: &Connection,
    folder: FolderId,
    categories: Option<&[MailCategory]>,
) -> Result<InboxThreads> {
    let rows = folder_rows(conn, folder)?;
    Ok((threads_of(&rows, categories), unread_by_category(&rows)))
}

/// The conversations of a folder's `rows`, as [`folder_threads`] gives them.
fn threads_of(rows: &[FolderRow], categories: Option<&[MailCategory]>) -> Vec<ThreadEntry> {
    let mut seen = HashSet::new();
    rows.iter()
        .filter(|row| row.thread.is_none_or(|thread| seen.insert(thread)))
        .filter(|row| categories.is_none_or(|wanted| wanted.contains(&tab(row.category))))
        .map(|row| ThreadEntry {
            thread: row.thread.map(ThreadId),
            latest: MessageId(row.id),
        })
        .collect()
}

/// Which messages a list across folders keeps, by their flags: those with
/// every flag of `set` and none of `unset`. The default keeps all.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct FlagFilter {
    pub set: MessageFlags,
    pub unset: MessageFlags,
}

impl FlagFilter {
    /// Unread messages.
    pub const UNREAD: Self = Self {
        set: MessageFlags::empty(),
        unset: MessageFlags::SEEN,
    };
    /// Starred messages.
    pub const STARRED: Self = Self {
        set: MessageFlags::FLAGGED,
        unset: MessageFlags::empty(),
    };
    /// Messages marked important.
    pub const IMPORTANT: Self = Self {
        set: MessageFlags::IMPORTANT,
        unset: MessageFlags::empty(),
    };
}

/// One message of a list across folders.
struct SpreadRow {
    row: FolderRow,
    account: i64,
    message_id_hdr: Option<String>,
}

/// Every message in any of `folders` that `filter` keeps, each once, newest
/// first (undated last).
fn spread_rows(
    conn: &Connection,
    folders: &[FolderId],
    filter: FlagFilter,
) -> Result<Vec<SpreadRow>> {
    if folders.is_empty() {
        return Ok(Vec::new());
    }
    let marks = vec!["?"; folders.len()].join(",");
    let mut stmt = conn.prepare_cached(&format!(
        "SELECT m.id, m.thread_id, m.category, m.flags, m.account_id, m.message_id_hdr, m.date
         FROM message m
         WHERE m.id IN (SELECT message_id FROM message_location WHERE folder_id IN ({marks}))
           AND (m.flags & ?) = ?"
    ))?;
    let mask = i64::from((filter.set | filter.unset).bits());
    let want = i64::from(filter.set.bits());
    let params = folders
        .iter()
        .map(|f| f.0)
        .chain([mask, want])
        .collect::<Vec<i64>>();
    let rows = stmt.query_map(rusqlite::params_from_iter(params), |row| {
        let flags: i64 = row.get(3)?;
        let spread = SpreadRow {
            row: FolderRow {
                id: row.get(0)?,
                thread: row.get(1)?,
                category: row
                    .get::<_, Option<i64>>(2)?
                    .and_then(MailCategory::from_storage),
                unread: flags & i64::from(MessageFlags::SEEN.bits()) == 0,
            },
            account: row.get(4)?,
            message_id_hdr: row.get(5)?,
        };
        Ok((row.get::<_, Option<i64>>(6)?, spread))
    })?;
    // Sorted here, as in `folder_rows`.
    let mut rows = rows.collect::<rusqlite::Result<Vec<_>>>()?;
    rows.sort_unstable_by_key(|(date, r)| newest_first(*date, r.row.id));
    Ok(rows.into_iter().map(|(_, r)| r).collect())
}

/// The conversations with a message in any of `folders` that `filter`
/// keeps, newest first, each with its newest such message: the lists of
/// the unified inbox, over the folders of several accounts.
pub(crate) fn spread_threads(
    conn: &Connection,
    folders: &[FolderId],
    filter: FlagFilter,
) -> Result<Vec<ThreadEntry>> {
    let mut seen = HashSet::new();
    Ok(spread_rows(conn, folders, filter)?
        .into_iter()
        .filter(|r| r.row.thread.is_none_or(|thread| seen.insert(thread)))
        .map(|r| ThreadEntry {
            thread: r.row.thread.map(ThreadId),
            latest: MessageId(r.row.id),
        })
        .collect())
}

/// The messages in any of `folders` that `filter` keeps, newest first;
/// server copies of one message (the same `Message-ID` in one account,
/// such as Gmail's Inbox and All Mail copies) show once.
pub(crate) fn spread_message_ids(
    conn: &Connection,
    folders: &[FolderId],
    filter: FlagFilter,
) -> Result<Vec<MessageId>> {
    let mut seen = HashSet::new();
    Ok(spread_rows(conn, folders, filter)?
        .into_iter()
        .filter(|r| match &r.message_id_hdr {
            Some(hdr) => seen.insert((r.account, hdr.clone())),
            None => true,
        })
        .map(|r| MessageId(r.row.id))
        .collect())
}

/// Which inbox tab of the unified inbox to list. Each account files its
/// mail in its own tabs; here every message takes the tab of its category,
/// except where its account lists that category in its first tab (a tab
/// turned off, or tabs off altogether): there it counts as Primary.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SpreadTabs {
    /// The tab's categories; `None` lists every tab.
    pub categories: Option<Vec<MailCategory>>,
    /// Per account, the categories it lists in its first tab.
    pub folded: Vec<(AccountId, Vec<MailCategory>)>,
}

impl SpreadTabs {
    /// The tab a message of `account` with `category` shows in.
    fn tab_of(&self, account: i64, category: Option<MailCategory>) -> MailCategory {
        let category = tab(category);
        let folded = self
            .folded
            .iter()
            .any(|(a, folded)| a.0 == account && folded.contains(&category));
        if folded {
            MailCategory::Primary
        } else {
            category
        }
    }

    fn keeps(&self, tab: MailCategory) -> bool {
        self.categories
            .as_ref()
            .is_none_or(|wanted| wanted.contains(&tab))
    }
}

/// The unified inbox's conversations in one tab, as [`spread_threads`]
/// lists them, and its unread conversations per tab: in each, the newest
/// message picks the tab, as in one account's inbox.
pub(crate) fn spread_inbox_threads(
    conn: &Connection,
    folders: &[FolderId],
    tabs: &SpreadTabs,
) -> Result<InboxThreads> {
    let rows = spread_rows(conn, folders, FlagFilter::default())?;
    let mut seen = HashSet::new();
    let mut entries = Vec::new();
    let mut tabbed = Vec::new();
    for r in &rows {
        let row = FolderRow {
            category: Some(tabs.tab_of(r.account, r.row.category)),
            ..r.row
        };
        if row.thread.is_none_or(|thread| seen.insert(thread)) && tabs.keeps(tab(row.category)) {
            entries.push(ThreadEntry {
                thread: row.thread.map(ThreadId),
                latest: MessageId(row.id),
            });
        }
        tabbed.push(row);
    }
    Ok((entries, unread_by_category(&tabbed)))
}

/// The unified inbox's messages in one tab, as [`spread_message_ids`]
/// lists them.
pub(crate) fn spread_inbox_message_ids(
    conn: &Connection,
    folders: &[FolderId],
    tabs: &SpreadTabs,
) -> Result<Vec<MessageId>> {
    let mut seen = HashSet::new();
    Ok(spread_rows(conn, folders, FlagFilter::default())?
        .into_iter()
        .filter(|r| match &r.message_id_hdr {
            Some(hdr) => seen.insert((r.account, hdr.clone())),
            None => true,
        })
        .filter(|r| tabs.keeps(tabs.tab_of(r.account, r.row.category)))
        .map(|r| MessageId(r.row.id))
        .collect())
}

/// The messages of `thread` that are in `folder`, oldest first.
pub(crate) fn folder_thread_messages(
    conn: &Connection,
    folder: FolderId,
    thread: ThreadId,
) -> Result<Vec<MessageId>> {
    let mut stmt = conn.prepare_cached(
        "SELECT m.id FROM message_location l JOIN message m ON m.id = l.message_id
         WHERE l.folder_id = ?1 AND m.thread_id = ?2
         ORDER BY m.date IS NULL, m.date, m.id",
    )?;
    let rows = stmt.query_map([folder.0, thread.0], |row| Ok(MessageId(row.get(0)?)))?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// The messages of `folder` in one inbox tab (some categories), newest
/// first.
pub(crate) fn folder_messages_in(
    conn: &Connection,
    folder: FolderId,
    categories: &[MailCategory],
) -> Result<Vec<MessageId>> {
    Ok(folder_rows(conn, folder)?
        .into_iter()
        .filter(|row| categories.contains(&tab(row.category)))
        .map(|row| MessageId(row.id))
        .collect())
}

/// Conversations in `folder` with unread mail, per tab (every category, in
/// tab order, zeros included).
pub(crate) fn category_unread(
    conn: &Connection,
    folder: FolderId,
) -> Result<Vec<(MailCategory, u64)>> {
    Ok(unread_by_category(&folder_rows(conn, folder)?))
}

/// The unread conversations of a folder's `rows` per inbox tab, as
/// [`category_unread`] counts them.
fn unread_by_category(rows: &[FolderRow]) -> Vec<(MailCategory, u64)> {
    // Thread → (tab of its newest message, unread).
    let mut threads: HashMap<i64, (MailCategory, bool)> = HashMap::new();
    let mut counts: HashMap<MailCategory, u64> = HashMap::new();
    for row in rows {
        match row.thread {
            Some(thread) => {
                let entry = threads.entry(thread).or_insert((tab(row.category), false));
                entry.1 |= row.unread;
            }
            None if row.unread => *counts.entry(tab(row.category)).or_default() += 1,
            None => {}
        }
    }
    for (category, unread) in threads.into_values() {
        if unread {
            *counts.entry(category).or_default() += 1;
        }
    }
    MailCategory::ALL
        .into_iter()
        .map(|c| (c, counts.get(&c).copied().unwrap_or_default()))
        .collect()
}

/// One message row of a thread.
struct ThreadRow {
    id: i64,
    flags: MessageFlags,
    has_attachments: bool,
    /// Only in trash or junk folders.
    discarded: bool,
}

/// The messages of `thread` that a reader sees, oldest first (undated
/// last): server copies of one message (same `Message-ID`, for example
/// Gmail's Inbox and All Mail) count once, and messages only in trash or
/// junk are left out unless the whole thread is there. For each message,
/// every copy that is shown is returned, the representative first.
fn thread_rows(conn: &Connection, thread: ThreadId) -> Result<Vec<Vec<ThreadRow>>> {
    let mut stmt = conn.prepare_cached(
        "SELECT m.id, m.message_id_hdr, m.flags, m.has_attachments, m.blob_hash IS NOT NULL,
                NOT EXISTS (
                    SELECT 1 FROM message_location l JOIN folder f ON f.id = l.folder_id
                    WHERE l.message_id = m.id
                      AND (f.role IS NULL OR f.role NOT IN ('trash', 'junk'))
                )
         FROM message m WHERE m.thread_id = ?1
         ORDER BY m.date IS NULL, m.date, m.id",
    )?;
    let mut groups: Vec<(Vec<ThreadRow>, bool)> = Vec::new();
    let mut by_header: HashMap<String, usize> = HashMap::new();
    let mut rows = stmt.query([thread.0])?;
    while let Some(row) = rows.next()? {
        let header: Option<String> = row.get(1)?;
        let has_body: bool = row.get(4)?;
        let flags: i64 = row.get(2)?;
        let message = ThreadRow {
            id: row.get(0)?,
            flags: MessageFlags::from_bits(u32::try_from(flags).unwrap_or_default()),
            has_attachments: row.get(3)?,
            discarded: row.get(5)?,
        };
        let group = match header {
            Some(header) => *by_header.entry(header).or_insert_with(|| {
                groups.push((Vec::new(), false));
                groups.len() - 1
            }),
            None => {
                groups.push((Vec::new(), false));
                groups.len() - 1
            }
        };
        let (copies, best_has_body) = &mut groups[group];
        // Representative: not discarded, then with a body, then oldest ID.
        let better = copies.first().is_none_or(|best: &ThreadRow| {
            (best.discarded && !message.discarded)
                || (best.discarded == message.discarded && has_body && !*best_has_body)
        });
        if better {
            copies.insert(0, message);
            *best_has_body = has_body;
        } else {
            copies.push(message);
        }
    }
    let mut groups: Vec<Vec<ThreadRow>> = groups.into_iter().map(|(copies, _)| copies).collect();
    if groups.iter().any(|copies| !copies[0].discarded) {
        groups.retain(|copies| !copies[0].discarded);
        for copies in &mut groups {
            copies.retain(|copy| !copy.discarded);
        }
    }
    Ok(groups)
}

/// See [`Store::with_copies`](crate::Store::with_copies).
pub(crate) fn with_copies(
    conn: &Connection,
    messages: &[MessageId],
) -> Result<Vec<(MessageId, MessageFlags)>> {
    let mut stmt = conn.prepare_cached(
        "SELECT c.id, c.flags FROM message m JOIN message c
           ON c.id = m.id
           OR (c.account_id = m.account_id AND c.message_id_hdr = m.message_id_hdr)
         WHERE m.id = ?1
         ORDER BY c.id",
    )?;
    let mut out: Vec<(MessageId, MessageFlags)> = Vec::new();
    for message in messages {
        let rows = stmt.query_map([message.0], |row| {
            let flags: i64 = row.get(1)?;
            Ok((
                MessageId(row.get(0)?),
                MessageFlags::from_bits(u32::try_from(flags).unwrap_or_default()),
            ))
        })?;
        for row in rows {
            let row = row?;
            if !out.iter().any(|(id, _)| *id == row.0) {
                out.push(row);
            }
        }
    }
    Ok(out)
}

/// All messages of `thread`, oldest first; see [`thread_rows`].
pub(crate) fn thread_messages(conn: &Connection, thread: ThreadId) -> Result<Vec<MessageId>> {
    Ok(thread_rows(conn, thread)?
        .into_iter()
        .map(|copies| MessageId(copies[0].id))
        .collect())
}

/// List summaries of `threads` as seen from `folder`, in the order given.
/// Threads that no longer exist are left out.
pub(crate) fn thread_summaries(
    conn: &Connection,
    threads: &[ThreadId],
    folder: FolderId,
) -> Result<Vec<ThreadSummary>> {
    let mut unread_here = conn.prepare_cached(
        "SELECT EXISTS (
             SELECT 1 FROM message m JOIN message_location l ON l.message_id = m.id
             WHERE m.thread_id = ?1 AND l.folder_id = ?2 AND (m.flags & ?3) = 0
         )",
    )?;
    let mut senders_of = conn.prepare_cached(
        "SELECT email_norm, display_name FROM participant
         WHERE message_id = ?1 AND role = 'from' ORDER BY rowid",
    )?;
    let mut out = Vec::with_capacity(threads.len());
    for &thread in threads {
        let messages = thread_rows(conn, thread)?;
        if messages.is_empty() {
            continue;
        }
        let unread: bool = unread_here.query_row(
            params![thread.0, folder.0, MessageFlags::SEEN.bits()],
            |row| row.get(0),
        )?;
        let copies = || messages.iter().flatten();
        let mut summary = ThreadSummary {
            thread,
            message_count: u32::try_from(messages.len()).unwrap_or(u32::MAX),
            unread,
            flagged: copies().any(|m| m.flags.contains(MessageFlags::FLAGGED)),
            important: copies().any(|m| m.flags.contains(MessageFlags::IMPORTANT)),
            has_attachments: copies().any(|m| m.has_attachments),
            senders: Vec::new(),
            last_from: Vec::new(),
            last_answered: false,
        };
        for copies in &messages {
            let message = &copies[0];
            let message_unread = copies.iter().any(|m| !m.flags.contains(MessageFlags::SEEN));
            let draft = copies.iter().any(|m| m.flags.contains(MessageFlags::DRAFT));
            if !draft {
                summary.last_from.clear();
                summary.last_answered = copies
                    .iter()
                    .any(|m| m.flags.contains(MessageFlags::ANSWERED));
            }
            let mut rows = senders_of.query([message.id])?;
            while let Some(row) = rows.next()? {
                let email: String = row.get(0)?;
                let name: Option<String> = row.get(1)?;
                if !draft {
                    summary.last_from.push(email.clone());
                }
                match summary.senders.iter_mut().find(|s| s.email == email) {
                    Some(sender) => {
                        sender.unread |= message_unread;
                        if sender.name.is_none() {
                            sender.name = name;
                        }
                    }
                    None => summary.senders.push(ThreadSender {
                        email,
                        name,
                        unread: message_unread,
                    }),
                }
            }
        }
        out.push(summary);
    }
    Ok(out)
}

/// Whether a line of the list is unread and starred, as its row shows it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Marks {
    pub unread: bool,
    pub flagged: bool,
}

/// [`Marks`] of every message in a folder and of every conversation with
/// a message there, read in two queries, for picking lines ("select all
/// unread") without reading each row.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FolderMarks {
    pub messages: HashMap<MessageId, Marks>,
    /// Unread as [`ThreadSummary::unread`] (a message in the folder is),
    /// starred as [`ThreadSummary::flagged`] (any message of it is).
    pub threads: HashMap<ThreadId, Marks>,
}

pub(crate) fn folder_marks(conn: &Connection, folder: FolderId) -> Result<FolderMarks> {
    let mut marks = FolderMarks::default();
    let mut here = conn.prepare_cached(
        "SELECT m.id, m.thread_id, m.flags FROM message_location l
         JOIN message m ON m.id = l.message_id
         WHERE l.folder_id = ?1",
    )?;
    let mut rows = here.query([folder.0])?;
    while let Some(row) = rows.next()? {
        let flags: i64 = row.get(2)?;
        let flags = MessageFlags::from_bits(u32::try_from(flags).unwrap_or_default());
        let unread = !flags.contains(MessageFlags::SEEN);
        marks.messages.insert(
            MessageId(row.get(0)?),
            Marks {
                unread,
                flagged: flags.contains(MessageFlags::FLAGGED),
            },
        );
        if let Some(thread) = row.get::<_, Option<i64>>(1)? {
            marks.threads.entry(ThreadId(thread)).or_default().unread |= unread;
        }
    }
    let mut starred = conn.prepare_cached(
        "SELECT DISTINCT m.thread_id FROM message m
         WHERE (m.flags & ?2) != 0 AND m.thread_id IN (
             SELECT h.thread_id FROM message_location l
             JOIN message h ON h.id = l.message_id
             WHERE l.folder_id = ?1 AND h.thread_id IS NOT NULL
         )",
    )?;
    let mut rows = starred.query(params![folder.0, MessageFlags::FLAGGED.bits()])?;
    while let Some(row) = rows.next()? {
        marks
            .threads
            .entry(ThreadId(row.get(0)?))
            .or_default()
            .flagged = true;
    }
    Ok(marks)
}

#[cfg(test)]
mod tests {
    use katna_core::{AccountKind, Paths};

    use crate::{Mode, NewMessage, Store};

    use super::*;

    fn message<'a>(raw: &'a [u8], date: Option<i64>, flags: MessageFlags) -> NewMessage<'a> {
        NewMessage {
            raw,
            message_id_hdr: None,
            subject: Some("s"),
            date,
            flags,
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
    fn folders_with_counts_and_messages_newest_first() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
        let work = store.add_account(AccountKind::Local, "w", "w").unwrap().id;
        let home = store.add_account(AccountKind::Local, "h", "h").unwrap().id;

        let mut batch = store.mail_batch().unwrap();
        let inbox = batch.ensure_folder(work, "INBOX").unwrap();
        let archive = batch.ensure_folder(work, "Archive").unwrap();
        let empty = batch.ensure_folder(home, "Empty").unwrap();
        let mut add = |folder, raw: &'static [u8], date, flags| match batch
            .add_message(work, folder, &message(raw, date, flags))
            .unwrap()
        {
            crate::Added::Message(id) | crate::Added::Location(id) => id,
            crate::Added::Duplicate(_) => panic!("unexpected duplicate"),
        };
        let old = add(inbox, b"1", Some(100), MessageFlags::SEEN);
        let undated = add(inbox, b"2", None, MessageFlags::empty());
        let new = add(inbox, b"3", Some(300), MessageFlags::FLAGGED);
        let same_day = add(inbox, b"4", Some(300), MessageFlags::SEEN);
        // The same message in a second folder counts in both.
        assert_eq!(add(archive, b"1", Some(100), MessageFlags::SEEN), old);
        batch.commit().unwrap();

        let reader = Store::open(&paths, Mode::ReadOnly).unwrap();
        let folders = reader.folder_summaries().unwrap();
        let summary: Vec<_> = folders
            .iter()
            .map(|f| (f.id, f.account, f.path.as_str(), f.total))
            .collect();
        assert_eq!(
            summary,
            [
                (archive, work, "Archive", 1),
                (inbox, work, "INBOX", 4),
                (empty, home, "Empty", 0),
            ]
        );
        assert_eq!(reader.unread_counts().unwrap(), [(inbox, 2)]);
        assert_eq!(
            reader.folder_message_ids(inbox).unwrap(),
            [same_day, new, old, undated]
        );
        assert_eq!(reader.folder_message_ids(archive).unwrap(), [old]);
        assert!(reader.folder_message_ids(empty).unwrap().is_empty());
        assert!(reader.folder_message_ids(FolderId(999)).unwrap().is_empty());
    }

    #[test]
    fn lists_across_folders_and_accounts() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
        let work = store.add_account(AccountKind::Local, "w", "w").unwrap().id;
        let home = store.add_account(AccountKind::Local, "h", "h").unwrap().id;

        let mut batch = store.mail_batch().unwrap();
        let work_inbox = batch.ensure_folder(work, "INBOX").unwrap();
        let work_all = batch.ensure_folder(work, "All Mail").unwrap();
        let home_inbox = batch.ensure_folder(home, "INBOX").unwrap();
        let mut add = |account, folder, raw: &'static [u8], hdr, date, flags| {
            let mut new = message(raw, Some(date), flags);
            new.message_id_hdr = hdr;
            match batch.add_message(account, folder, &new).unwrap() {
                crate::Added::Message(id) | crate::Added::Location(id) => id,
                crate::Added::Duplicate(_) => panic!("unexpected duplicate"),
            }
        };
        let a = add(
            work,
            work_inbox,
            b"a",
            Some("<a@x>"),
            100,
            MessageFlags::SEEN,
        );
        // A server copy of `a` in All Mail, unread there.
        let a_copy = add(
            work,
            work_all,
            b"a2",
            Some("<a@x>"),
            100,
            MessageFlags::empty(),
        );
        let b = add(
            home,
            home_inbox,
            b"b",
            Some("<b@x>"),
            300,
            MessageFlags::FLAGGED,
        );
        let c = add(work, work_inbox, b"c", None, 200, MessageFlags::IMPORTANT);
        batch.commit().unwrap();
        assert_ne!(a, a_copy);

        let reader = Store::open(&paths, Mode::ReadOnly).unwrap();
        let all = FlagFilter::default();
        let inboxes = [work_inbox, home_inbox];
        assert_eq!(reader.spread_message_ids(&inboxes, all).unwrap(), [b, c, a]);
        // Copies of one message show once (the newer copy).
        assert_eq!(
            reader
                .spread_message_ids(&[work_inbox, work_all, home_inbox], all)
                .unwrap(),
            [b, c, a_copy]
        );
        assert_eq!(
            reader
                .spread_message_ids(&[work_all, work_inbox], FlagFilter::UNREAD)
                .unwrap(),
            [c, a_copy]
        );
        assert_eq!(
            reader
                .spread_message_ids(&inboxes, FlagFilter::STARRED)
                .unwrap(),
            [b]
        );
        assert_eq!(
            reader
                .spread_message_ids(&inboxes, FlagFilter::IMPORTANT)
                .unwrap(),
            [c]
        );
        let latest: Vec<MessageId> = reader
            .spread_threads(&inboxes, all)
            .unwrap()
            .into_iter()
            .map(|t| t.latest)
            .collect();
        assert_eq!(latest, [b, c, a]);
        assert!(reader.spread_message_ids(&[], all).unwrap().is_empty());
    }
}
