// SPDX-License-Identifier: GPL-3.0-or-later

//! One person as the mail shows them, for the contact panel beside a
//! conversation: how much mail was exchanged, the recent conversations
//! and the files. Local data only.
//!
//! Server copies of one message (Gmail's Inbox and All Mail, the same
//! message in two accounts) count once, by their `Message-ID`.

use katna_core::Account;
use rusqlite::{Connection, params};

use crate::error::Result;
use crate::mail::{MessageId, ThreadId};

/// The roles a person takes part in a message with.
const TAKES_PART: &str = "p.role IN ('from', 'to', 'cc', 'bcc')";

/// How much mail the user and one address exchanged.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ContactSummary {
    /// The display name the address signs its own mail with most often,
    /// else one it was written to with.
    pub name: Option<String>,
    /// Messages the address is on.
    pub messages: u64,
    /// Messages from it.
    pub from_them: u64,
    /// Messages from one of the user's accounts to it.
    pub to_them: u64,
    /// The oldest and newest of the messages (Unix seconds).
    pub first: Option<i64>,
    pub last: Option<i64>,
}

/// A conversation with the address, newest first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactConversation {
    /// `None` for a message not threaded (yet).
    pub thread: Option<ThreadId>,
    /// Its newest message with the address.
    pub message: MessageId,
    pub subject: String,
    pub date: Option<i64>,
    /// Messages in the conversation.
    pub count: u32,
}

/// A named attachment on mail with the address, newest first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactFile {
    pub message: MessageId,
    pub name: String,
    /// `type/subtype`, lower case.
    pub mime: String,
    /// Decoded size in bytes (estimated).
    pub size: u64,
    pub date: Option<i64>,
    /// Sent by the address (else by the user, or a third person).
    pub from_them: bool,
    /// How many attachments of the same name come before it in its
    /// message, and its place among the message's named attachments.
    pub nth: usize,
    pub order: usize,
}

/// The JSON array of the accounts' addresses, lower case, for `json_each`.
fn own_addresses(accounts: &[Account]) -> String {
    let own: Vec<String> = accounts
        .iter()
        .map(|a| a.address.trim().to_lowercase())
        .collect();
    serde_json::to_string(&own).unwrap_or_else(|_| "[]".to_owned())
}

/// See [`crate::Store::contact_summary`].
pub(crate) fn summary(
    conn: &Connection,
    accounts: &[Account],
    email: &str,
) -> Result<ContactSummary> {
    let email = email.trim().to_lowercase();
    let mut stmt = conn.prepare_cached(&format!(
        "WITH own (email) AS (SELECT value FROM json_each(?2)),
         mine (message_id) AS (
             SELECT p.message_id FROM participant p JOIN own o ON o.email = p.email_norm
             WHERE p.role = 'from'
         ),
         theirs AS (
             SELECT coalesce(m.message_id_hdr, 'id:' || m.id) AS mail,
                    max(p.role = 'from') AS from_them,
                    max(p.role != 'from' AND m.id IN (SELECT message_id FROM mine)) AS to_them,
                    m.date AS date
             FROM participant p JOIN message m ON m.id = p.message_id
             WHERE p.email_norm = ?1 AND {TAKES_PART}
             GROUP BY m.id
         )
         SELECT count(DISTINCT mail),
                count(DISTINCT CASE WHEN from_them THEN mail END),
                count(DISTINCT CASE WHEN to_them THEN mail END),
                min(date), max(date)
         FROM theirs"
    ))?;
    let count = |n: i64| u64::try_from(n).unwrap_or_default();
    let mut summary = stmt.query_row(params![email, own_addresses(accounts)], |row| {
        Ok(ContactSummary {
            name: None,
            messages: count(row.get(0)?),
            from_them: count(row.get(1)?),
            to_them: count(row.get(2)?),
            first: row.get(3)?,
            last: row.get(4)?,
        })
    })?;
    // Their own name for themselves first, then what others call them.
    let mut stmt = conn.prepare_cached(&format!(
        "SELECT trim(p.display_name) AS name
         FROM participant p
         WHERE p.email_norm = ?1 AND {TAKES_PART}
           AND nullif(trim(p.display_name), '') IS NOT NULL
           AND lower(trim(p.display_name)) != ?1
         GROUP BY name
         ORDER BY max(p.role = 'from') DESC, count(*) DESC, name
         LIMIT 1"
    ))?;
    let mut rows = stmt.query([&email])?;
    if let Some(row) = rows.next()? {
        summary.name = Some(row.get(0)?);
    }
    Ok(summary)
}

/// See [`crate::Store::contact_conversations`].
pub(crate) fn conversations(
    conn: &Connection,
    email: &str,
    limit: usize,
) -> Result<Vec<ContactConversation>> {
    let email = email.trim().to_lowercase();
    // Newest messages first; the first of each thread stands for it.
    let mut stmt = conn.prepare_cached(&format!(
        "SELECT m.id, m.thread_id, m.subject, m.date, t.message_count
         FROM participant p
         JOIN message m ON m.id = p.message_id
         LEFT JOIN thread t ON t.id = m.thread_id
         WHERE p.email_norm = ?1 AND {TAKES_PART}
         GROUP BY m.id
         ORDER BY m.date DESC, m.id DESC
         LIMIT ?2"
    ))?;
    // Enough rows for `limit` conversations of a few messages each.
    let scan = i64::try_from(limit.saturating_mul(40)).unwrap_or(i64::MAX);
    let rows = stmt.query_map(params![email, scan], |row| {
        Ok(ContactConversation {
            message: MessageId(row.get(0)?),
            thread: row.get::<_, Option<i64>>(1)?.map(ThreadId),
            subject: row.get(2)?,
            date: row.get(3)?,
            count: row
                .get::<_, Option<i64>>(4)?
                .and_then(|n| u32::try_from(n).ok())
                .unwrap_or(1)
                .max(1),
        })
    })?;
    let mut out: Vec<ContactConversation> = Vec::new();
    for row in rows {
        let row = row?;
        let same = |c: &ContactConversation| match (c.thread, row.thread) {
            (Some(a), Some(b)) => a == b,
            // Copies of one unthreaded message in two folders.
            _ => c.subject == row.subject && c.date == row.date,
        };
        if !out.iter().any(same) {
            out.push(row);
            if out.len() == limit {
                break;
            }
        }
    }
    Ok(out)
}

/// See [`crate::Store::contact_files`].
pub(crate) fn files(conn: &Connection, email: &str, limit: usize) -> Result<Vec<ContactFile>> {
    let email = email.trim().to_lowercase();
    let mut stmt = conn.prepare_cached(&format!(
        "WITH shared (message_id, from_them) AS (
             SELECT p.message_id, max(p.role = 'from')
             FROM participant p
             WHERE p.email_norm = ?1 AND {TAKES_PART}
             GROUP BY p.message_id
         ),
         named AS (
             SELECT a.message_id, trim(a.filename) AS name, a.mime, a.size,
                    row_number() OVER (PARTITION BY a.message_id, trim(a.filename)
                                       ORDER BY a.id) - 1 AS nth,
                    row_number() OVER (PARTITION BY a.message_id ORDER BY a.id) - 1 AS ord
             FROM attachment a JOIN shared s ON s.message_id = a.message_id
             WHERE nullif(trim(a.filename), '') IS NOT NULL
         )
         SELECT n.message_id, n.name, n.mime, n.size, m.date, s.from_them, n.nth, n.ord
         FROM named n
         JOIN message m ON m.id = n.message_id
         JOIN shared s ON s.message_id = n.message_id
         ORDER BY m.date DESC, n.message_id DESC, n.ord
         LIMIT ?2"
    ))?;
    let scan = i64::try_from(limit.saturating_mul(8)).unwrap_or(i64::MAX);
    let rows = stmt.query_map(params![email, scan], |row| {
        let index = |n: i64| usize::try_from(n).unwrap_or_default();
        Ok(ContactFile {
            message: MessageId(row.get(0)?),
            name: row.get(1)?,
            mime: row.get(2)?,
            size: row.get::<_, i64>(3)?.try_into().unwrap_or_default(),
            date: row.get(4)?,
            from_them: row.get::<_, i64>(5)? != 0,
            nth: index(row.get(6)?),
            order: index(row.get(7)?),
        })
    })?;
    let mut out: Vec<ContactFile> = Vec::new();
    for row in rows {
        let row = row?;
        // The same file sent again, or a server copy of its message.
        if !out.iter().any(|f| f.name == row.name && f.size == row.size) {
            out.push(row);
            if out.len() == limit {
                break;
            }
        }
    }
    Ok(out)
}

/// See [`crate::Store::messages_from`].
pub(crate) fn messages_from(
    conn: &Connection,
    email: &str,
    limit: usize,
) -> Result<Vec<MessageId>> {
    let email = email.trim().to_lowercase();
    let mut stmt = conn.prepare_cached(
        "SELECT m.id
         FROM participant p JOIN message m ON m.id = p.message_id
         WHERE p.email_norm = ?1 AND p.role = 'from' AND m.blob_hash IS NOT NULL
         GROUP BY coalesce(m.message_id_hdr, 'id:' || m.id)
         ORDER BY max(m.date) DESC
         LIMIT ?2",
    )?;
    let limit = i64::try_from(limit).unwrap_or(i64::MAX);
    let rows = stmt.query_map(params![email, limit], |row| Ok(MessageId(row.get(0)?)))?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

#[cfg(test)]
mod tests {
    use katna_core::{AccountKind, Paths};

    use crate::{Mode, NewAttachment, NewMessage, NewParticipant, ParticipantRole, Store};

    fn person(
        role: ParticipantRole,
        email: &'static str,
        name: Option<&'static str>,
    ) -> NewParticipant<'static> {
        NewParticipant {
            role,
            email_norm: email,
            domain: email.rsplit('@').next().unwrap(),
            display_name: name,
        }
    }

    #[test]
    fn summary_conversations_and_files() {
        use ParticipantRole::{From, To};
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
        let me = store
            .add_account(AccountKind::Local, "Me", "me@example.org")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch.ensure_folder(me, "INBOX").unwrap();
        let all = batch.ensure_folder(me, "All Mail").unwrap();
        let ada_to_me = [
            person(From, "ada@example.net", Some("Ada Lovelace")),
            person(To, "me@example.org", None),
        ];
        let me_to_ada = [
            person(From, "me@example.org", Some("Me")),
            person(To, "ada@example.net", Some("Ada")),
        ];
        // (raw, Message-ID, In-Reply-To, date, subject, from Ada, folder)
        let mail = [
            (&b"1"[..], "a@x", None, 100, "Plans", true, inbox),
            (
                &b"2"[..],
                "b@x",
                Some("a@x"),
                200,
                "Re: Plans",
                false,
                inbox,
            ),
            (&b"3"[..], "c@x", None, 300, "Invoice", true, inbox),
            // Gmail's All Mail copy of the invoice.
            (&b"3 "[..], "c@x", None, 300, "Invoice", true, all),
        ];
        let mut ids = Vec::new();
        for (raw, hdr, reply_to, date, subject, from_ada, folder) in mail {
            let message = NewMessage {
                raw,
                message_id_hdr: Some(hdr),
                subject: Some(subject),
                date: Some(date),
                flags: crate::MessageFlags::empty(),
                has_attachments: false,
                list_id: None,
                snippet: None,
                participants: if from_ada { &ada_to_me } else { &me_to_ada },
                in_reply_to: reply_to,
                references: &[],
                category: None,
            };
            let (crate::Added::Message(id)
            | crate::Added::Location(id)
            | crate::Added::Duplicate(id)) = batch.add_message(me, folder, &message).unwrap();
            ids.push(id);
        }
        let file = |name| NewAttachment {
            part: "2",
            mime: "application/pdf",
            filename: Some(name),
            size: 10,
        };
        batch
            .list_attachments(ids[0], &[file("notes.txt")])
            .unwrap();
        for &id in &ids[2..] {
            batch.list_attachments(id, &[file("invoice.pdf")]).unwrap();
        }
        batch.commit().unwrap();

        let store = Store::open(&paths, Mode::ReadOnly).unwrap();
        let summary = store.contact_summary("Ada@Example.net").unwrap();
        assert_eq!(summary.name.as_deref(), Some("Ada Lovelace"));
        assert_eq!(
            (summary.messages, summary.from_them, summary.to_them),
            (3, 2, 1)
        );
        assert_eq!((summary.first, summary.last), (Some(100), Some(300)));

        let conversations = store.contact_conversations("ada@example.net", 5).unwrap();
        let subjects: Vec<_> = conversations.iter().map(|c| c.subject.as_str()).collect();
        assert_eq!(subjects, ["Invoice", "Re: Plans"]);
        assert_eq!(conversations[1].count, 2);

        let files = store.contact_files("ada@example.net", 5).unwrap();
        let names: Vec<_> = files
            .iter()
            .map(|f| (f.name.as_str(), f.from_them))
            .collect();
        assert_eq!(names, [("invoice.pdf", true), ("notes.txt", true)]);

        let from = store.messages_from("ada@example.net", 5).unwrap();
        assert_eq!(from.len(), 2);
        assert!(ids[2..].contains(&from[0]) && from[1] == ids[0]);
        assert_eq!(
            store
                .contact_summary("nobody@example.net")
                .unwrap()
                .messages,
            0
        );
    }
}
