// SPDX-License-Identifier: GPL-3.0-or-later

//! Reading `mail.db` for the mail app: the folder list with counts and the
//! messages of one folder. Works in both read-write and read-only mode.

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
    let mut stmt = conn.prepare_cached(
        "SELECT l.message_id FROM message_location l JOIN message m ON m.id = l.message_id
         WHERE l.folder_id = ?1
         ORDER BY m.date IS NULL, m.date DESC, m.id DESC",
    )?;
    let rows = stmt.query_map([folder.0], |row| Ok(MessageId(row.get(0)?)))?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
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
    /// Some message of the conversation has attachments.
    pub has_attachments: bool,
    /// Distinct `From` addresses, in the order they first wrote.
    pub senders: Vec<ThreadSender>,
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
    let mut stmt = conn.prepare_cached(
        "SELECT m.id, m.thread_id, m.category, m.flags FROM message_location l
         JOIN message m ON m.id = l.message_id
         WHERE l.folder_id = ?1
         ORDER BY m.date IS NULL, m.date DESC, m.id DESC",
    )?;
    let rows = stmt.query_map([folder.0], |row| {
        let flags: i64 = row.get(3)?;
        Ok(FolderRow {
            id: row.get(0)?,
            thread: row.get(1)?,
            category: row
                .get::<_, Option<i64>>(2)?
                .and_then(MailCategory::from_storage),
            unread: flags & i64::from(MessageFlags::SEEN.bits()) == 0,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// Unclassified messages count as Primary.
fn tab(category: Option<MailCategory>) -> MailCategory {
    category.unwrap_or(MailCategory::Primary)
}

/// The conversations in `folder`, newest first, each with its newest
/// message in the folder. With `category`, only conversations whose newest
/// message in the folder has that category (unclassified counts as
/// Primary), so each conversation is in exactly one tab.
pub(crate) fn folder_threads(
    conn: &Connection,
    folder: FolderId,
    category: Option<MailCategory>,
) -> Result<Vec<ThreadEntry>> {
    let mut seen = HashSet::new();
    Ok(folder_rows(conn, folder)?
        .into_iter()
        .filter(|row| row.thread.is_none_or(|thread| seen.insert(thread)))
        .filter(|row| category.is_none_or(|wanted| tab(row.category) == wanted))
        .map(|row| ThreadEntry {
            thread: row.thread.map(ThreadId),
            latest: MessageId(row.id),
        })
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

/// The messages of `folder` in one inbox tab, newest first.
pub(crate) fn folder_messages_in(
    conn: &Connection,
    folder: FolderId,
    category: MailCategory,
) -> Result<Vec<MessageId>> {
    Ok(folder_rows(conn, folder)?
        .into_iter()
        .filter(|row| tab(row.category) == category)
        .map(|row| MessageId(row.id))
        .collect())
}

/// Conversations in `folder` with unread mail, per tab (every category, in
/// tab order, zeros included).
pub(crate) fn category_unread(
    conn: &Connection,
    folder: FolderId,
) -> Result<Vec<(MailCategory, u64)>> {
    // Thread → (tab of its newest message, unread).
    let mut threads: HashMap<i64, (MailCategory, bool)> = HashMap::new();
    let mut counts: HashMap<MailCategory, u64> = HashMap::new();
    for row in folder_rows(conn, folder)? {
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
    Ok(MailCategory::ALL
        .into_iter()
        .map(|c| (c, counts.get(&c).copied().unwrap_or_default()))
        .collect())
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
            has_attachments: copies().any(|m| m.has_attachments),
            senders: Vec::new(),
        };
        for copies in &messages {
            let message = &copies[0];
            let message_unread = copies.iter().any(|m| !m.flags.contains(MessageFlags::SEEN));
            let mut rows = senders_of.query([message.id])?;
            while let Some(row) = rows.next()? {
                let email: String = row.get(0)?;
                let name: Option<String> = row.get(1)?;
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
}
