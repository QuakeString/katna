// SPDX-License-Identifier: GPL-3.0-or-later

//! Every named attachment of every account in one list, newest first:
//! what Katna Mail's Files page shows. Local data only: the lists sync
//! reads from each message's structure. Mail in Trash or Spam is left
//! out, and a file sent again (the same name and size, also a copy of
//! its message in another account) shows once, at its newest.

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use katna_core::{Account, AccountId};
use rusqlite::{Connection, params};

use crate::error::Result;
use crate::mail::MessageId;

/// A named attachment, with what the Files page says about its mail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryFile {
    pub message: MessageId,
    pub account: AccountId,
    pub name: String,
    /// `type/subtype`, lower case.
    pub mime: String,
    /// Decoded size in bytes (estimated).
    pub size: u64,
    pub date: Option<i64>,
    pub subject: String,
    /// Who sent the mail: the name it signs with (if any) and the address.
    pub from_name: Option<String>,
    pub from_email: String,
    /// Sent from one of the user's own addresses.
    pub mine: bool,
    /// Its message is downloaded, so it opens at once.
    pub downloaded: bool,
    /// How many attachments of the same name come before it in its
    /// message, and its place among the message's named attachments.
    pub nth: usize,
    pub order: usize,
    /// In how many conversations its sender sent the same file (name and
    /// size): a picture in many is a signature logo, not a file.
    pub conversations: usize,
}

/// See [`crate::Store::library_files`].
pub(crate) fn files(
    conn: &Connection,
    accounts: &[Account],
    limit: usize,
) -> Result<Vec<LibraryFile>> {
    let own: Vec<String> = accounts
        .iter()
        .map(|a| a.address.trim().to_lowercase())
        .collect();
    let own = serde_json::to_string(&own).unwrap_or_else(|_| "[]".to_owned());
    let mut stmt = conn.prepare_cached(
        "WITH own (email) AS (SELECT value FROM json_each(?1)),
         named AS (
             SELECT a.message_id, trim(a.filename) AS name, lower(a.mime) AS mime, a.size,
                    row_number() OVER (PARTITION BY a.message_id, trim(a.filename)
                                       ORDER BY a.id) - 1 AS nth,
                    row_number() OVER (PARTITION BY a.message_id ORDER BY a.id) - 1 AS ord
             FROM attachment a
             WHERE nullif(trim(a.filename), '') IS NOT NULL
         )
         SELECT n.message_id, m.account_id, n.name, n.mime, n.size, m.date, m.subject,
                f.display_name, coalesce(f.email_norm, ''),
                f.email_norm IN (SELECT email FROM own),
                m.blob_hash IS NOT NULL, n.nth, n.ord, coalesce(m.thread_id, -m.id)
         FROM named n
         JOIN message m ON m.id = n.message_id
         LEFT JOIN participant f ON f.rowid = (
             SELECT p.rowid FROM participant p
             WHERE p.message_id = m.id AND p.role IN ('from', 'sender')
             ORDER BY p.role = 'sender', p.rowid LIMIT 1
         )
         WHERE NOT EXISTS (
             SELECT 1 FROM message_location l JOIN folder d ON d.id = l.folder_id
             WHERE l.message_id = m.id AND d.role IN ('trash', 'junk')
         )
         ORDER BY m.date DESC, n.message_id DESC, n.ord
         LIMIT ?2",
    )?;
    // Room for the copies and files sent again that are left out.
    let scan = i64::try_from(limit.saturating_mul(2)).unwrap_or(i64::MAX);
    let rows = stmt.query_map(params![own, scan], |row| {
        let index = |n: i64| usize::try_from(n).unwrap_or_default();
        let name: Option<String> = row.get(7)?;
        let thread: i64 = row.get(13)?;
        Ok((
            thread,
            LibraryFile {
                message: MessageId(row.get(0)?),
                account: AccountId(row.get(1)?),
                name: row.get(2)?,
                mime: row.get(3)?,
                size: row.get::<_, i64>(4)?.try_into().unwrap_or_default(),
                date: row.get(5)?,
                subject: row.get(6)?,
                from_name: name.map(|n| n.trim().to_owned()).filter(|n| !n.is_empty()),
                from_email: row.get(8)?,
                mine: row.get::<_, Option<bool>>(9)?.unwrap_or(false),
                downloaded: row.get(10)?,
                nth: index(row.get(11)?),
                order: index(row.get(12)?),
                conversations: 1,
            },
        ))
    })?;
    // Each file once (the newest copy), with the conversations its sender
    // sent it in.
    let mut seen: HashMap<(String, u64), (usize, HashSet<i64>)> = HashMap::new();
    let mut out: Vec<LibraryFile> = Vec::new();
    for row in rows {
        let (thread, row) = row?;
        match seen.entry((row.name.clone(), row.size)) {
            Entry::Occupied(mut first) => {
                let (at, threads) = first.get_mut();
                let kept = &mut out[*at];
                if kept.from_email == row.from_email && threads.insert(thread) {
                    kept.conversations = threads.len();
                }
            }
            Entry::Vacant(slot) => {
                // Past the limit, rows only count copies of files kept.
                if out.len() < limit {
                    slot.insert((out.len(), HashSet::from([thread])));
                    out.push(row);
                }
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use katna_core::{AccountKind, Paths};

    use crate::{
        FolderRole, Mode, NewAttachment, NewMessage, NewParticipant, ParticipantRole, Store,
    };

    fn person(role: ParticipantRole, email: &'static str) -> NewParticipant<'static> {
        NewParticipant {
            role,
            email_norm: email,
            domain: email.rsplit('@').next().unwrap(),
            display_name: Some("Someone"),
        }
    }

    #[test]
    fn every_file_once_newest_first_without_trash() {
        use ParticipantRole::{From, To};
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
        let me = store
            .add_account(AccountKind::Imap, "Me", "me@example.org")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(me, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        let trash = batch
            .upsert_folder(me, "Trash", Some(FolderRole::Trash))
            .unwrap();
        let theirs = [
            person(From, "ada@example.net"),
            person(To, "me@example.org"),
        ];
        let mine = [
            person(From, "me@example.org"),
            person(To, "ada@example.net"),
        ];
        // (raw, Message-ID, date, from me, folder, files)
        let mail = [
            (&b"1"[..], "a@x", 100, false, inbox, &["plan.pdf", ""][..]),
            (
                b"2",
                "b@x",
                200,
                true,
                inbox,
                &["photo.jpg", "photo.jpg"][..],
            ),
            (b"3", "c@x", 300, false, inbox, &["plan.pdf"][..]),
            (b"4", "d@x", 400, false, trash, &["old.zip"][..]),
        ];
        for (raw, hdr, date, from_me, folder, files) in mail {
            let message = NewMessage {
                raw,
                message_id_hdr: Some(hdr),
                subject: Some(hdr),
                date: Some(date),
                flags: crate::MessageFlags::empty(),
                has_attachments: true,
                list_id: None,
                snippet: None,
                participants: if from_me { &mine } else { &theirs },
                in_reply_to: None,
                references: &[],
                category: None,
            };
            let (crate::Added::Message(id)
            | crate::Added::Location(id)
            | crate::Added::Duplicate(id)) = batch.add_message(me, folder, &message).unwrap();
            let parts: Vec<String> = (0..files.len()).map(|n| (n + 2).to_string()).collect();
            let list: Vec<NewAttachment> = files
                .iter()
                .zip(&parts)
                .map(|(name, part)| NewAttachment {
                    part,
                    mime: "Application/PDF",
                    filename: Some(name),
                    // The second photo differs from the first.
                    size: if part == "3" { 20 } else { 10 },
                })
                .collect();
            batch.list_attachments(id, &list).unwrap();
        }
        batch.commit().unwrap();

        let store = Store::open(&paths, Mode::ReadOnly).unwrap();
        let files = store.library_files(100).unwrap();
        let seen: Vec<_> = files
            .iter()
            .map(|f| (f.name.as_str(), f.subject.as_str(), f.mine, f.nth, f.order))
            .collect();
        assert_eq!(
            seen,
            [
                ("plan.pdf", "c@x", false, 0, 0),
                ("photo.jpg", "b@x", true, 0, 0),
                ("photo.jpg", "b@x", true, 1, 1),
            ]
        );
        assert_eq!(files[0].mime, "application/pdf");
        // Ada sent the plan in two conversations; one photo copy is a file.
        assert_eq!(files[0].conversations, 2);
        assert_eq!(files[1].conversations, 1);
        assert_eq!(files[0].from_email, "ada@example.net");
        assert_eq!(files[0].from_name.as_deref(), Some("Someone"));
        assert!(files[0].downloaded);
        assert_eq!(store.library_files(1).unwrap().len(), 1);
    }
}
