// SPDX-License-Identifier: GPL-3.0-or-later

//! Reading `mail.db` for the mail app: the folder list with counts and the
//! messages of one folder. Works in both read-write and read-only mode.

use katna_core::AccountId;
use rusqlite::Connection;

use crate::error::Result;
use crate::mail::{FolderId, MessageFlags, MessageId};

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
        let folders = reader.folders().unwrap();
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
