// SPDX-License-Identifier: GPL-3.0-or-later

//! Threading: which conversation a message belongs to
//! (`docs/ARCHITECTURE.md` §6.5).
//!
//! Per account, a message joins:
//!
//! 1. with a Gmail thread ID (`X-GM-THRID`): the thread with that ID. The
//!    first time an ID is seen, a thread found by references (step 2) that
//!    has no Gmail ID yet adopts it, so threads built before the ID was known
//!    carry on.
//! 2. the threads of stored messages whose `Message-ID` is in its
//!    `In-Reply-To` or `References`, or equals its own `Message-ID` (another
//!    copy of the same message), and the threads that were waiting for its
//!    `Message-ID` (`thread_ref`: a reply stored before the message it
//!    answers). Several such threads are merged into the oldest.
//! 3. otherwise, when the subject has a reply or forward prefix, the newest
//!    thread with the same normalized subject whose last message is within
//!    30 days of this one.
//! 4. otherwise a new thread.
//!
//! References that match no threaded message go to `thread_ref`, so the
//! message they name joins this thread when it arrives.
//!
//! `thread.message_count` and `thread.last_date` are kept by triggers
//! (`schema/mail_v2.sql`), so every path that adds, moves or deletes
//! messages keeps them right.

use std::collections::BTreeSet;

use katna_core::AccountId;
use katna_core::subject::normalize_subject;
use rusqlite::{Connection, OptionalExtension, params};

use crate::error::Result;
use crate::journal::{self, ChangeOp, ObjectKind};

/// How far apart (seconds) two messages may be to join by subject alone.
const SUBJECT_WINDOW: i64 = 30 * 24 * 60 * 60;

/// At most this many unmatched references are remembered per message: the
/// `In-Reply-To` and the newest `References`.
const MAX_PENDING_REFS: usize = 16;

/// What threading needs to know about a message.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Links<'a> {
    pub account: AccountId,
    pub message_id_hdr: Option<&'a str>,
    pub in_reply_to: Option<&'a str>,
    pub references: &'a [&'a str],
    pub subject: &'a str,
    pub date: Option<i64>,
    pub gm_thread_id: Option<u64>,
}

/// Returns the thread for a message that is about to be stored, or is
/// stored without a thread, and whether that thread is new (and already
/// journaled). Threads merged away or into are added to `changed`.
pub(crate) fn assign(
    conn: &Connection,
    links: &Links<'_>,
    changed: &mut BTreeSet<i64>,
) -> Result<(i64, bool)> {
    let account = links.account.0;
    let subject = normalize_subject(links.subject);

    if let Some(thrid) = links.gm_thread_id {
        // Stored as a signed 64-bit integer, bit for bit.
        let thrid = i64::from_ne_bytes(thrid.to_ne_bytes());
        let existing: Option<i64> = conn
            .prepare_cached("SELECT id FROM thread WHERE account_id = ?1 AND gm_thrid = ?2")?
            .query_row(params![account, thrid], |row| row.get(0))
            .optional()?;
        if let Some(thread) = existing {
            return Ok((thread, false));
        }
        let (found, _) = referenced_threads(conn, links)?;
        for thread in found {
            let adopted = conn
                .prepare_cached(
                    "UPDATE thread SET gm_thrid = ?2 WHERE id = ?1 AND gm_thrid IS NULL",
                )?
                .execute(params![thread, thrid])?;
            if adopted > 0 {
                return Ok((thread, false));
            }
        }
        return Ok((create(conn, account, &subject.text, Some(thrid))?, true));
    }

    let (mut threads, unmatched) = referenced_threads(conn, links)?;
    if let Some(own) = links.message_id_hdr {
        let mut stmt = conn.prepare_cached(
            "SELECT thread_id FROM thread_ref WHERE account_id = ?1 AND message_id_hdr = ?2",
        )?;
        let waiting = stmt.query_map(params![account, own], |row| row.get::<_, i64>(0))?;
        for thread in waiting {
            threads.insert(thread?);
        }
        conn.prepare_cached(
            "DELETE FROM thread_ref WHERE account_id = ?1 AND message_id_hdr = ?2",
        )?
        .execute(params![account, own])?;
    }

    if threads.is_empty()
        && subject.is_reply
        && !subject.text.is_empty()
        && let Some(date) = links.date
    {
        let found: Option<i64> = conn
            .prepare_cached(
                "SELECT id FROM thread
                 WHERE account_id = ?1 AND subject_norm = ?2 AND last_date BETWEEN ?3 AND ?4
                 ORDER BY last_date DESC LIMIT 1",
            )?
            .query_row(
                params![
                    account,
                    subject.text,
                    date.saturating_sub(SUBJECT_WINDOW),
                    date.saturating_add(SUBJECT_WINDOW)
                ],
                |row| row.get(0),
            )
            .optional()?;
        threads.extend(found);
    }

    let (thread, created) = match pick_survivor(conn, &threads)? {
        Some((keep, others)) => {
            for other in others {
                merge(conn, other, keep, changed)?;
            }
            (keep, false)
        }
        None => (create(conn, account, &subject.text, None)?, true),
    };
    // A conversation's first message, arriving after its replies (the
    // backfill goes by ID, not date), names the thread for step 3.
    let is_root = links.in_reply_to.is_none() && links.references.is_empty() && !subject.is_reply;
    if !created && is_root && !subject.text.is_empty() {
        conn.prepare_cached("UPDATE thread SET subject_norm = ?2 WHERE id = ?1")?
            .execute(params![thread, subject.text])?;
    }

    let mut insert = conn.prepare_cached(
        "INSERT OR IGNORE INTO thread_ref (account_id, message_id_hdr, thread_id)
         VALUES (?1, ?2, ?3)",
    )?;
    for reference in unmatched.iter().take(MAX_PENDING_REFS) {
        insert.execute(params![account, reference, thread])?;
    }
    Ok((thread, created))
}

/// Threads of the threaded messages that `links` names (its references and
/// its own `Message-ID`), and the references that matched none, newest
/// first.
fn referenced_threads<'a>(
    conn: &Connection,
    links: &Links<'a>,
) -> Result<(BTreeSet<i64>, Vec<&'a str>)> {
    let mut stmt = conn.prepare_cached(
        "SELECT DISTINCT thread_id FROM message
         WHERE message_id_hdr = ?1 AND account_id = ?2 AND thread_id IS NOT NULL",
    )?;
    let mut threads = BTreeSet::new();
    let mut unmatched = Vec::new();
    let mut seen = BTreeSet::new();
    // Newest reference first: In-Reply-To, then References from the end.
    let references = links
        .in_reply_to
        .into_iter()
        .chain(links.references.iter().rev().copied())
        .map(|r| (r, true))
        .chain(links.message_id_hdr.map(|own| (own, false)));
    for (reference, is_reference) in references {
        let reference = reference.trim();
        if reference.is_empty() || !seen.insert(reference) {
            continue;
        }
        let rows = stmt.query_map(params![reference, links.account.0], |row| row.get(0))?;
        let mut found = false;
        for thread in rows {
            threads.insert(thread?);
            found = true;
        }
        if !found && is_reference && Some(reference) != links.message_id_hdr {
            unmatched.push(reference);
        }
    }
    Ok((threads, unmatched))
}

/// The thread to keep among `threads`, and the ones to merge into it. A
/// Gmail thread is kept over others, and two Gmail threads are never merged
/// (Gmail's thread IDs are authoritative).
fn pick_survivor(conn: &Connection, threads: &BTreeSet<i64>) -> Result<Option<(i64, Vec<i64>)>> {
    let Some(&oldest) = threads.first() else {
        return Ok(None);
    };
    if threads.len() == 1 {
        return Ok(Some((oldest, Vec::new())));
    }
    let mut stmt = conn.prepare_cached("SELECT gm_thrid IS NOT NULL FROM thread WHERE id = ?1")?;
    let mut gmail = Vec::new();
    let mut plain = Vec::new();
    for &thread in threads {
        let is_gmail: Option<bool> = stmt.query_row([thread], |row| row.get(0)).optional()?;
        match is_gmail {
            Some(true) => gmail.push(thread),
            Some(false) => plain.push(thread),
            None => {}
        }
    }
    let keep = gmail.first().copied().unwrap_or(oldest);
    let others = plain.into_iter().filter(|&t| t != keep).collect();
    Ok(Some((keep, others)))
}

fn create(
    conn: &Connection,
    account: i64,
    subject_norm: &str,
    gm_thrid: Option<i64>,
) -> Result<i64> {
    conn.prepare_cached(
        "INSERT INTO thread (account_id, subject_norm, gm_thrid) VALUES (?1, ?2, ?3)",
    )?
    .execute(params![account, subject_norm, gm_thrid])?;
    let id = conn.last_insert_rowid();
    journal::record(conn, ObjectKind::Thread, id, ChangeOp::Insert)?;
    Ok(id)
}

/// Moves every message and pending reference of thread `from` to `into`,
/// then removes `from`.
fn merge(conn: &Connection, from: i64, into: i64, changed: &mut BTreeSet<i64>) -> Result<()> {
    conn.prepare_cached(
        "INSERT OR IGNORE INTO thread_ref (account_id, message_id_hdr, thread_id)
         SELECT account_id, message_id_hdr, ?2 FROM thread_ref WHERE thread_id = ?1",
    )?
    .execute(params![from, into])?;
    // A muted conversation stays muted (before the trigger removes `from`).
    conn.prepare_cached(
        "UPDATE OR IGNORE mute SET thread_id = ?2 WHERE kind = 'thread' AND thread_id = ?1",
    )?
    .execute(params![from, into])?;
    conn.prepare_cached("UPDATE message SET thread_id = ?2 WHERE thread_id = ?1")?
        .execute(params![from, into])?;
    // Usually gone already: the trigger removes a thread with no messages.
    conn.prepare_cached("DELETE FROM thread WHERE id = ?1")?
        .execute([from])?;
    // Journaled as a thread delete and update, not per message: nothing
    // else about the messages changed, so the search index can stay.
    changed.insert(from);
    changed.insert(into);
    Ok(())
}

/// Journals each thread in `threads`: an update if it still exists, a
/// delete if it was merged away or emptied.
pub(crate) fn journal_changes(conn: &Connection, threads: &BTreeSet<i64>) -> Result<()> {
    let mut exists = conn.prepare_cached("SELECT 1 FROM thread WHERE id = ?1")?;
    for &thread in threads {
        let op = if exists.query_row([thread], |_| Ok(())).optional()?.is_some() {
            ChangeOp::Update
        } else {
            ChangeOp::Delete
        };
        journal::record(conn, ObjectKind::Thread, thread, op)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
