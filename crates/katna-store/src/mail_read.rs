// SPDX-License-Identifier: GPL-3.0-or-later

//! Reading messages from `mail.db`, for the search indexer and for showing
//! search results. Works in both read-write and read-only mode.

use std::collections::HashMap;

use katna_core::{AccountId, MailCategory};
use rusqlite::{Connection, OptionalExtension, Row, params};

use crate::blob::BlobHash;
use crate::error::{Error, Result};
use crate::mail::{MessageFlags, MessageId, ParticipantRole, ThreadId};

/// One address of a stored message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredParticipant {
    pub role: ParticipantRole,
    pub email_norm: String,
    pub domain: String,
    pub display_name: Option<String>,
}

/// A folder a stored message is in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredLocation {
    pub path: String,
    /// `folder.role`: `inbox`, `sent`, ... or `None`.
    pub role: Option<String>,
}

/// A message with everything the search index needs except its text, which
/// comes from the raw message in the blob store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredMessage {
    pub id: MessageId,
    pub account: AccountId,
    pub subject: String,
    /// Unix seconds.
    pub date: Option<i64>,
    pub size: u64,
    pub flags: MessageFlags,
    /// IMAP keywords and labels.
    pub keywords: Vec<String>,
    pub has_attachments: bool,
    pub list_id: Option<String>,
    pub blob_hash: Option<BlobHash>,
    pub snippet: Option<String>,
    /// The conversation; `None` until the daemon's backfill threads a
    /// message stored before threading.
    pub thread_id: Option<ThreadId>,
    /// Inbox tab; `None` when not classified yet (shown as Primary).
    pub category: Option<MailCategory>,
    pub participants: Vec<StoredParticipant>,
    pub locations: Vec<StoredLocation>,
}

impl StoredMessage {
    /// The first participant with `role`.
    pub fn first(&self, role: ParticipantRole) -> Option<&StoredParticipant> {
        self.participants.iter().find(|p| p.role == role)
    }
}

const MESSAGE_COLUMNS: &str = "id, account_id, subject, date, size, flags, keywords,
                               has_attachments, list_id, blob_hash, snippet, thread_id,
                               category";

/// Messages with ID greater than `after`, in ID order, at most `limit`.
pub(crate) fn messages_after(
    conn: &Connection,
    after: MessageId,
    limit: u32,
) -> Result<Vec<StoredMessage>> {
    let mut messages = conn
        .prepare_cached(&format!(
            "SELECT {MESSAGE_COLUMNS} FROM message WHERE id > ?1 ORDER BY id LIMIT ?2"
        ))?
        .query_map(params![after.0, limit], message_row)?
        .collect::<rusqlite::Result<Vec<_>>>()?
        .into_iter()
        .collect::<Result<Vec<_>>>()?;
    if let (Some(first), Some(last)) = (messages.first(), messages.last()) {
        let range = (first.id.0, last.id.0);
        attach_details(conn, &mut messages, range)?;
    }
    Ok(messages)
}

/// The messages with the given IDs that exist, in the order given.
pub(crate) fn messages_by_id(conn: &Connection, ids: &[MessageId]) -> Result<Vec<StoredMessage>> {
    let mut stmt = conn.prepare_cached(&format!(
        "SELECT {MESSAGE_COLUMNS} FROM message WHERE id = ?1"
    ))?;
    let mut messages = Vec::with_capacity(ids.len());
    for id in ids {
        if let Some(message) = stmt.query_row([id.0], message_row).optional()? {
            let mut message = vec![message?];
            attach_details(conn, &mut message, (id.0, id.0))?;
            messages.extend(message);
        }
    }
    Ok(messages)
}

/// Number of messages in all accounts.
pub(crate) fn message_count(conn: &Connection) -> Result<u64> {
    let count: i64 = conn
        .prepare_cached("SELECT count(*) FROM message")?
        .query_row([], |row| row.get(0))?;
    Ok(count.try_into().unwrap_or_default())
}

fn message_row(row: &Row<'_>) -> rusqlite::Result<Result<StoredMessage>> {
    let keywords: Option<String> = row.get(6)?;
    let blob_hash: Option<Vec<u8>> = row.get(9)?;
    let size: i64 = row.get(4)?;
    let flags: i64 = row.get(5)?;
    let id = MessageId(row.get(0)?);
    let build = || -> Result<StoredMessage> {
        Ok(StoredMessage {
            id,
            account: AccountId(row.get(1)?),
            subject: row.get(2)?,
            date: row.get(3)?,
            size: size.try_into().unwrap_or_default(),
            flags: MessageFlags::from_bits(u32::try_from(flags).unwrap_or_default()),
            keywords: match keywords {
                None => Vec::new(),
                Some(json) => serde_json::from_str(&json)
                    .map_err(|err| Error::InvalidData(format!("message {id}: keywords: {err}")))?,
            },
            has_attachments: row.get(7)?,
            list_id: row.get(8)?,
            blob_hash: blob_hash
                .map(|bytes| {
                    <[u8; 32]>::try_from(bytes.as_slice())
                        .map(BlobHash::from_bytes)
                        .map_err(|_| Error::InvalidData(format!("message {id}: bad blob hash")))
                })
                .transpose()?,
            snippet: row.get(10)?,
            thread_id: row.get::<_, Option<i64>>(11)?.map(ThreadId),
            category: row
                .get::<_, Option<i64>>(12)?
                .and_then(MailCategory::from_storage),
            participants: Vec::new(),
            locations: Vec::new(),
        })
    };
    Ok(build())
}

/// Fills in participants and locations of `messages`, whose IDs all lie in
/// `range` (inclusive). Two range scans instead of two queries per message.
fn attach_details(
    conn: &Connection,
    messages: &mut [StoredMessage],
    range: (i64, i64),
) -> Result<()> {
    let index: HashMap<i64, usize> = messages
        .iter()
        .enumerate()
        .map(|(i, message)| (message.id.0, i))
        .collect();

    let mut stmt = conn.prepare_cached(
        "SELECT message_id, role, email_norm, domain, display_name FROM participant
         WHERE message_id BETWEEN ?1 AND ?2 ORDER BY rowid",
    )?;
    let mut rows = stmt.query(params![range.0, range.1])?;
    while let Some(row) = rows.next()? {
        let Some(&i) = index.get(&row.get::<_, i64>(0)?) else {
            continue;
        };
        let role: String = row.get(1)?;
        let role = ParticipantRole::parse(&role)
            .ok_or_else(|| Error::InvalidData(format!("unknown participant role {role:?}")))?;
        messages[i].participants.push(StoredParticipant {
            role,
            email_norm: row.get(2)?,
            domain: row.get(3)?,
            display_name: row.get(4)?,
        });
    }

    let mut stmt = conn.prepare_cached(
        "SELECT l.message_id, f.path, f.role FROM message_location l
         JOIN folder f ON f.id = l.folder_id
         WHERE l.message_id BETWEEN ?1 AND ?2 ORDER BY l.message_id, f.id",
    )?;
    let mut rows = stmt.query(params![range.0, range.1])?;
    while let Some(row) = rows.next()? {
        let Some(&i) = index.get(&row.get::<_, i64>(0)?) else {
            continue;
        };
        messages[i].locations.push(StoredLocation {
            path: row.get(1)?,
            role: row.get(2)?,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use katna_core::{AccountKind, Paths};

    use crate::{
        Mode, NewMessage, NewParticipant, ParticipantRole, Store, StoredLocation, StoredParticipant,
    };

    use super::*;

    fn new_message<'a>(
        raw: &'a [u8],
        subject: &'a str,
        participants: &'a [NewParticipant<'a>],
    ) -> NewMessage<'a> {
        NewMessage {
            raw,
            message_id_hdr: None,
            subject: Some(subject),
            date: Some(1_000),
            flags: MessageFlags::SEEN,
            has_attachments: true,
            list_id: Some("list.example.org"),
            snippet: Some("preview"),
            participants,
            in_reply_to: None,
            references: &[],
            category: None,
        }
    }

    #[test]
    fn reads_messages_with_participants_and_folders() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
        let account = store.add_account(AccountKind::Local, "x", "x").unwrap().id;
        let from = [NewParticipant {
            role: ParticipantRole::From,
            email_norm: "ada@example.org",
            domain: "example.org",
            display_name: Some("Ada"),
        }];
        let to = [NewParticipant {
            role: ParticipantRole::To,
            email_norm: "bob@example.net",
            domain: "example.net",
            display_name: None,
        }];
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch.ensure_folder(account, "INBOX").unwrap();
        let archive = batch.ensure_folder(account, "Archive").unwrap();
        let first = new_message(b"Subject: one\r\n\r\n1", "one", &from);
        let second = new_message(b"Subject: two\r\n\r\n2", "two", &to);
        let crate::Added::Message(one) = batch.add_message(account, inbox, &first).unwrap() else {
            panic!("expected a new message");
        };
        batch.add_message(account, archive, &first).unwrap();
        let crate::Added::Message(two) = batch.add_message(account, inbox, &second).unwrap() else {
            panic!("expected a new message");
        };
        batch.commit().unwrap();

        let reader = Store::open(&paths, Mode::ReadOnly).unwrap();
        assert_eq!(reader.message_count().unwrap(), 2);
        let all = reader.messages_after(MessageId(0), 10).unwrap();
        assert_eq!(all.len(), 2);
        let message = &all[0];
        assert_eq!(message.id, one);
        assert_eq!(message.account, account);
        assert_eq!(message.subject, "one");
        assert_eq!(message.date, Some(1_000));
        assert_eq!(message.flags, MessageFlags::SEEN);
        assert!(message.has_attachments);
        assert_eq!(message.list_id.as_deref(), Some("list.example.org"));
        assert_eq!(
            message.blob_hash,
            Some(crate::BlobHash::of(b"Subject: one\r\n\r\n1"))
        );
        assert_eq!(
            message.first(ParticipantRole::From),
            Some(&StoredParticipant {
                role: ParticipantRole::From,
                email_norm: "ada@example.org".into(),
                domain: "example.org".into(),
                display_name: Some("Ada".into()),
            })
        );
        let paths: Vec<_> = message.locations.iter().map(|l| l.path.as_str()).collect();
        assert_eq!(paths, ["INBOX", "Archive"]);
        assert_eq!(
            all[1].locations,
            [StoredLocation {
                path: "INBOX".into(),
                role: None
            }]
        );
        assert_eq!(all[1].participants.len(), 1);

        assert_eq!(reader.messages_after(one, 10).unwrap()[0].id, two);
        assert_eq!(reader.messages_after(MessageId(0), 1).unwrap().len(), 1);
        assert!(reader.messages_after(two, 10).unwrap().is_empty());

        let picked = reader.messages_by_id(&[two, MessageId(999), one]).unwrap();
        let ids: Vec<_> = picked.iter().map(|m| m.id).collect();
        assert_eq!(ids, [two, one]);
        assert_eq!(picked[1].locations.len(), 2);
    }
}
