// SPDX-License-Identifier: GPL-3.0-or-later

//! People from the mail: every address that sent or received mail, with
//! how often it appears. The start of Katna's contacts, read-only.

use katna_core::{Account, AccountId};
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

/// The display name `account`'s own messages from `address` most often
/// carry; see [`crate::Store::name_in_own_mail`].
pub(crate) fn name_in_own_mail(
    conn: &Connection,
    account: AccountId,
    address: &str,
) -> Result<Option<String>> {
    let mut stmt = conn.prepare_cached(
        "SELECT trim(p.display_name) AS name
         FROM participant p JOIN message m ON m.id = p.message_id
         WHERE p.email_norm = lower(trim(?2)) AND p.role = 'from' AND m.account_id = ?1
           AND nullif(trim(p.display_name), '') IS NOT NULL
         GROUP BY name
         ORDER BY count(*) DESC, name
         LIMIT 1",
    )?;
    let mut rows = stmt.query(rusqlite::params![account.0, address])?;
    Ok(match rows.next()? {
        Some(row) => Some(row.get(0)?),
        None => None,
    })
}

/// How one account has written with one address, for suggesting
/// recipients.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Correspondent {
    pub account: AccountId,
    /// Lower-cased address.
    pub email: String,
    /// A display name the address used, if any.
    pub name: Option<String>,
    /// Messages the account sent to it (To, Cc or Bcc), and the newest.
    pub sent: u32,
    pub last_sent: Option<i64>,
    /// Messages it sent the account, and the newest.
    pub received: u32,
    pub last_received: Option<i64>,
    /// Messages to others it was also on, and the newest.
    pub copied: u32,
    pub last_copied: Option<i64>,
}

/// Every address in the mail of `accounts`, one row per account. A message
/// counts as sent by the account when it is from the account's address or
/// in a Sent folder.
///
/// Groups the whole participant table like [`people`]; call it off the UI
/// thread.
pub(crate) fn correspondents(
    conn: &Connection,
    accounts: &[Account],
) -> Result<Vec<Correspondent>> {
    let own: Vec<(i64, String)> = accounts
        .iter()
        .map(|a| (a.id.0, a.address.trim().to_lowercase()))
        .collect();
    let own = serde_json::to_string(&own).unwrap_or_else(|_| "[]".to_owned());
    let mut stmt = conn.prepare_cached(
        "WITH own (account_id, email) AS (
             SELECT json_extract(value, '$[0]'), json_extract(value, '$[1]')
             FROM json_each(?1)
         ),
         sent (message_id) AS (
             SELECT p.message_id
             FROM participant p
             JOIN message m ON m.id = p.message_id
             JOIN own o ON o.email = p.email_norm AND o.account_id = m.account_id
             WHERE p.role = 'from'
             UNION
             SELECT l.message_id
             FROM message_location l JOIN folder f ON f.id = l.folder_id
             WHERE f.role = 'sent'
         )
         SELECT m.account_id, p.email_norm, max(nullif(trim(p.display_name), '')),
                sum(s.message_id IS NOT NULL AND p.role != 'from'),
                max(CASE WHEN s.message_id IS NOT NULL AND p.role != 'from' THEN m.date END),
                sum(s.message_id IS NULL AND p.role = 'from'),
                max(CASE WHEN s.message_id IS NULL AND p.role = 'from' THEN m.date END),
                sum(s.message_id IS NULL AND p.role != 'from'),
                max(CASE WHEN s.message_id IS NULL AND p.role != 'from' THEN m.date END)
         FROM participant p
         JOIN message m ON m.id = p.message_id
         LEFT JOIN sent s ON s.message_id = p.message_id
         WHERE p.role IN ('from', 'to', 'cc', 'bcc') AND p.email_norm LIKE '%_@_%'
         GROUP BY m.account_id, p.email_norm",
    )?;
    let count = |n: i64| u32::try_from(n).unwrap_or(u32::MAX);
    let rows = stmt.query_map([own], |row| {
        Ok(Correspondent {
            account: AccountId(row.get(0)?),
            email: row.get(1)?,
            name: row.get(2)?,
            sent: count(row.get(3)?),
            last_sent: row.get(4)?,
            received: count(row.get(5)?),
            last_received: row.get(6)?,
            copied: count(row.get(7)?),
            last_copied: row.get(8)?,
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

    #[test]
    fn own_name_and_rename() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
        let me = store
            .add_account(AccountKind::Local, "me@example.org", "Me@Example.org")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let sent = batch.ensure_folder(me, "Sent").unwrap();
        let from = |name: Option<&'static str>| NewParticipant {
            role: ParticipantRole::From,
            email_norm: "me@example.org",
            domain: "example.org",
            display_name: name,
        };
        for (raw, name) in [
            (&b"1"[..], Some("Ada Lovelace")),
            (&b"2"[..], Some(" Ada Lovelace ")),
            (&b"3"[..], Some("ada")),
            (&b"4"[..], None),
        ] {
            let participants = [from(name)];
            let message = NewMessage {
                raw,
                message_id_hdr: None,
                subject: Some("s"),
                date: Some(1),
                flags: crate::MessageFlags::empty(),
                has_attachments: false,
                list_id: None,
                snippet: None,
                participants: &participants,
                in_reply_to: None,
                references: &[],
                category: None,
            };
            batch.add_message(me, sent, &message).unwrap();
        }
        batch.commit().unwrap();
        let name = store.name_in_own_mail(me, "Me@Example.org").unwrap();
        assert_eq!(name.as_deref(), Some("Ada Lovelace"));
        assert_eq!(
            store.name_in_own_mail(me, "other@example.org").unwrap(),
            None
        );

        assert!(store.rename_account(me, "Ada").unwrap());
        assert_eq!(store.accounts().unwrap()[0].display_name, "Ada");
    }

    #[test]
    fn counts_sent_received_and_copied() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
        let me = store
            .add_account(AccountKind::Local, "Me", "Me@Example.org")
            .unwrap()
            .id;
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch.ensure_folder(me, "INBOX").unwrap();
        let person = |role, email: &'static str, name: Option<&'static str>| NewParticipant {
            role,
            email_norm: email,
            domain: email.rsplit('@').next().unwrap(),
            display_name: name,
        };
        use ParticipantRole::{Cc, From, To};
        for (raw, date, participants) in [
            // I write to Ada twice.
            (
                &b"1"[..],
                100,
                vec![
                    person(From, "me@example.org", Some("Me")),
                    person(To, "ada@example.org", Some("Ada Lovelace")),
                ],
            ),
            (
                &b"2"[..],
                300,
                vec![
                    person(From, "me@example.org", None),
                    person(To, "ada@example.org", None),
                ],
            ),
            // Bob writes to me, copying Ada.
            (
                &b"3"[..],
                200,
                vec![
                    person(From, "bob@example.net", Some("Bob")),
                    person(To, "me@example.org", None),
                    person(Cc, "ada@example.org", None),
                ],
            ),
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
            batch.add_message(me, inbox, &message).unwrap();
        }
        batch.commit().unwrap();

        let mut rows = Store::open(&paths, Mode::ReadOnly)
            .unwrap()
            .correspondents()
            .unwrap();
        rows.sort_by(|a, b| a.email.cmp(&b.email));
        let summary: Vec<_> = rows
            .iter()
            .map(|c| {
                (
                    c.email.as_str(),
                    c.name.as_deref(),
                    (c.sent, c.last_sent),
                    (c.received, c.last_received),
                    (c.copied, c.last_copied),
                )
            })
            .collect();
        assert_eq!(
            summary,
            [
                (
                    "ada@example.org",
                    Some("Ada Lovelace"),
                    (2, Some(300)),
                    (0, None),
                    (1, Some(200))
                ),
                (
                    "bob@example.net",
                    Some("Bob"),
                    (0, None),
                    (1, Some(200)),
                    (0, None)
                ),
                (
                    "me@example.org",
                    Some("Me"),
                    (0, None),
                    (0, None),
                    (1, Some(200))
                ),
            ]
        );
        assert!(rows.iter().all(|c| c.account == me));
    }
}
