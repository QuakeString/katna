// SPDX-License-Identifier: GPL-3.0-or-later

//! People from the mail: every address that sent or received mail, with
//! how often it appears. The start of Katna's contacts, read-only.

use rusqlite::Connection;

use crate::error::Result;

/// One address seen in the mail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    /// Lower-cased address.
    pub email: String,
    /// A display name the address used, if any.
    pub name: Option<String>,
    /// Messages the address is on.
    pub messages: u64,
    /// The newest of those messages (Unix seconds).
    pub last: Option<i64>,
}

/// The `limit` addresses on the most messages, most first.
///
/// Groups the whole participant table, so it takes a while on big stores
/// (about a second per million addresses); call it off the UI thread.
pub(crate) fn people(conn: &Connection, limit: u32) -> Result<Vec<Person>> {
    let mut stmt = conn.prepare_cached(
        "SELECT p.email_norm, max(nullif(trim(p.display_name), '')),
                count(DISTINCT p.message_id), max(m.date)
         FROM participant p JOIN message m ON m.id = p.message_id
         WHERE p.email_norm LIKE '%@%'
         GROUP BY p.email_norm
         ORDER BY 3 DESC, 1
         LIMIT ?1",
    )?;
    let rows = stmt.query_map([limit], |row| {
        Ok(Person {
            email: row.get(0)?,
            name: row.get(1)?,
            messages: row.get::<_, i64>(2)?.try_into().unwrap_or_default(),
            last: row.get(3)?,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

#[cfg(test)]
mod tests {
    use katna_core::{AccountKind, Paths};

    use crate::{Mode, NewMessage, NewParticipant, ParticipantRole, Store};

    #[test]
    fn most_frequent_first() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
        let account = store.add_account(AccountKind::Local, "a", "a").unwrap().id;
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch.ensure_folder(account, "INBOX").unwrap();
        let ada = |role| NewParticipant {
            role,
            email_norm: "ada@example.org",
            domain: "example.org",
            display_name: Some("Ada"),
        };
        let bob = NewParticipant {
            role: ParticipantRole::To,
            email_norm: "bob@example.net",
            domain: "example.net",
            display_name: None,
        };
        for (raw, date, participants) in [
            (&b"1"[..], 100, vec![ada(ParticipantRole::From), bob]),
            (&b"2"[..], 200, vec![ada(ParticipantRole::From)]),
        ] {
            let message = NewMessage {
                raw,
                message_id_hdr: None,
                subject: Some("s"),
                date: Some(date),
                flags: crate::MessageFlags::empty(),
                has_attachments: false,
                list_id: None,
                snippet: None,
                participants: &participants,
                in_reply_to: None,
                references: &[],
                category: None,
            };
            batch.add_message(account, inbox, &message).unwrap();
        }
        batch.commit().unwrap();

        let people = Store::open(&paths, Mode::ReadOnly)
            .unwrap()
            .people(10)
            .unwrap();
        let summary: Vec<_> = people
            .iter()
            .map(|p| (p.email.as_str(), p.name.as_deref(), p.messages, p.last))
            .collect();
        assert_eq!(
            summary,
            [
                ("ada@example.org", Some("Ada"), 2, Some(200)),
                ("bob@example.net", None, 1, Some(100)),
            ]
        );
    }
}
