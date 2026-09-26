// SPDX-License-Identifier: GPL-3.0-or-later

//! Change journal.
//!
//! Every write the daemon makes to `mail.db` or `pim.db` appends a row to that
//! database's `change_log` in the same transaction. After a D-Bus change
//! signal (`docs/ARCHITECTURE.md` §14.2), an app reads the entries after the
//! last sequence number it has seen and refreshes only what changed.

use std::fmt;
use std::str::FromStr;

use rusqlite::{Connection, params};

use crate::db::unix_now;
use crate::error::{Error, Result};

/// What kind of object changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectKind {
    Account,
    Folder,
    Message,
    Thread,
    Organization,
    Contact,
}

impl ObjectKind {
    const ALL: [Self; 6] = [
        Self::Account,
        Self::Folder,
        Self::Message,
        Self::Thread,
        Self::Organization,
        Self::Contact,
    ];

    /// Stable name stored in `change_log.object_kind`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::Folder => "folder",
            Self::Message => "message",
            Self::Thread => "thread",
            Self::Organization => "organization",
            Self::Contact => "contact",
        }
    }
}

impl fmt::Display for ObjectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ObjectKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::ALL
            .into_iter()
            .find(|kind| kind.as_str() == s)
            .ok_or_else(|| Error::InvalidData(format!("unknown object kind {s:?}")))
    }
}

/// How an object changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChangeOp {
    Insert,
    Update,
    Delete,
}

impl ChangeOp {
    /// Stable name stored in `change_log.op`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Insert => "insert",
            Self::Update => "update",
            Self::Delete => "delete",
        }
    }
}

impl FromStr for ChangeOp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "insert" => Ok(Self::Insert),
            "update" => Ok(Self::Update),
            "delete" => Ok(Self::Delete),
            _ => Err(Error::InvalidData(format!("unknown change op {s:?}"))),
        }
    }
}

/// One journal entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    /// Increasing sequence number; pass the last one seen to
    /// [`changes_since`].
    pub seq: i64,
    pub kind: ObjectKind,
    pub object_id: i64,
    pub op: ChangeOp,
    /// Unix seconds.
    pub changed_at: i64,
}

/// Appends an entry. Call it inside the transaction that makes the change.
pub(crate) fn record(
    conn: &Connection,
    kind: ObjectKind,
    object_id: i64,
    op: ChangeOp,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO change_log (object_kind, object_id, op, changed_at) VALUES (?1, ?2, ?3, ?4)",
        params![kind.as_str(), object_id, op.as_str(), unix_now()],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Entries with a sequence number greater than `after`, oldest first, at
/// most `limit` of them.
pub(crate) fn changes_since(conn: &Connection, after: i64, limit: u32) -> Result<Vec<Change>> {
    let mut stmt = conn.prepare_cached(
        "SELECT seq, object_kind, object_id, op, changed_at FROM change_log
         WHERE seq > ?1 ORDER BY seq LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![after, limit], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, i64>(4)?,
        ))
    })?;
    rows.map(|row| {
        let (seq, kind, object_id, op, changed_at) = row?;
        Ok(Change {
            seq,
            kind: kind.parse()?,
            object_id,
            op: op.parse()?,
            changed_at,
        })
    })
    .collect()
}

/// The newest sequence number, or 0 when the journal is empty.
pub(crate) fn latest_seq(conn: &Connection) -> Result<i64> {
    Ok(
        conn.query_row("SELECT coalesce(max(seq), 0) FROM change_log", [], |row| {
            row.get(0)
        })?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{self, DbKind, Mode};

    #[test]
    fn records_and_reads_in_order() {
        let tmp = tempfile::tempdir().unwrap();
        let conn = db::open(&tmp.path().join("mail.db"), DbKind::Mail, Mode::ReadWrite).unwrap();
        assert_eq!(latest_seq(&conn).unwrap(), 0);
        let first = record(&conn, ObjectKind::Message, 7, ChangeOp::Insert).unwrap();
        let second = record(&conn, ObjectKind::Folder, 2, ChangeOp::Update).unwrap();
        record(&conn, ObjectKind::Message, 7, ChangeOp::Delete).unwrap();
        assert!(second > first);
        assert_eq!(latest_seq(&conn).unwrap(), second + 1);

        let all = changes_since(&conn, 0, 100).unwrap();
        assert_eq!(all.len(), 3);
        assert_eq!(
            (all[0].kind, all[0].object_id, all[0].op),
            (ObjectKind::Message, 7, ChangeOp::Insert)
        );
        assert!(all.windows(2).all(|w| w[0].seq < w[1].seq));

        let rest = changes_since(&conn, first, 1).unwrap();
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0].seq, second);
    }

    #[test]
    fn names_round_trip() {
        for kind in ObjectKind::ALL {
            assert_eq!(kind.as_str().parse::<ObjectKind>().unwrap(), kind);
        }
        for op in [ChangeOp::Insert, ChangeOp::Update, ChangeOp::Delete] {
            assert_eq!(op.as_str().parse::<ChangeOp>().unwrap(), op);
        }
        assert!("mailbox".parse::<ObjectKind>().is_err());
    }
}
