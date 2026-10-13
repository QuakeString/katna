// SPDX-License-Identifier: GPL-3.0-or-later

//! Opening SQLite databases and running schema migrations.
//!
//! Each database file has a list of migrations. Migration `n` (1-based)
//! upgrades the schema from version `n - 1` to `n`; the current version is
//! kept in `PRAGMA user_version`. Migrations are append-only: once released,
//! a migration never changes, a new one is added instead.
//!
//! Before raising the version of an existing database, the daemon copies it
//! into `backup/` beside it (keeping the last two copies per database) and
//! refuses to migrate when the disk has no room for the copy. A database
//! newer than this build is still opened when its `schema_meta` table says
//! this build's schema can read it (expand, then contract).

use std::path::{Path, PathBuf};
use std::time::Duration;

use rusqlite::{Connection, OpenFlags, OptionalExtension, TransactionBehavior};

use crate::error::{Error, Result};

/// How long a connection waits for a lock held by another connection.
const BUSY_TIMEOUT: Duration = Duration::from_secs(5);

/// Whether a store may be written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Create, migrate and write. Only `katna-daemon` uses this.
    ReadWrite,
    /// Read an existing, fully migrated store. Used by the apps.
    ReadOnly,
}

/// The database files and their migrations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbKind {
    /// `mail.db`
    Mail,
    /// `pim.db`
    Pim,
    /// `blobs.db`
    Blobs,
}

impl DbKind {
    /// Migrations in order; index `i` produces schema version `i + 1`.
    pub(crate) fn migrations(self) -> &'static [&'static str] {
        match self {
            Self::Mail => &[
                include_str!("schema/mail_v1.sql"),
                include_str!("schema/mail_v2.sql"),
                include_str!("schema/mail_v3.sql"),
                include_str!("schema/mail_v4.sql"),
                include_str!("schema/mail_v5.sql"),
                include_str!("schema/mail_v6.sql"),
                include_str!("schema/mail_v7.sql"),
                include_str!("schema/mail_v8.sql"),
                include_str!("schema/mail_v9.sql"),
                include_str!("schema/mail_v10.sql"),
                include_str!("schema/mail_v11.sql"),
                include_str!("schema/mail_v12.sql"),
                include_str!("schema/mail_v13.sql"),
                include_str!("schema/mail_v14.sql"),
                include_str!("schema/mail_v15.sql"),
            ],
            Self::Pim => &[
                include_str!("schema/pim_v1.sql"),
                include_str!("schema/pim_v2.sql"),
                include_str!("schema/pim_v3.sql"),
                include_str!("schema/pim_v4.sql"),
                include_str!("schema/pim_v5.sql"),
                include_str!("schema/pim_v6.sql"),
                include_str!("schema/pim_v7.sql"),
                include_str!("schema/pim_v8.sql"),
                include_str!("schema/pim_v9.sql"),
                include_str!("schema/pim_v10.sql"),
                include_str!("schema/pim_v11.sql"),
                include_str!("schema/pim_v12.sql"),
                include_str!("schema/pim_v13.sql"),
                include_str!("schema/pim_v14.sql"),
                include_str!("schema/pim_v15.sql"),
                include_str!("schema/pim_v16.sql"),
                include_str!("schema/pim_v17.sql"),
                include_str!("schema/pim_v18.sql"),
            ],
            Self::Blobs => &[
                include_str!("schema/blobs_v1.sql"),
                include_str!("schema/blobs_v2.sql"),
            ],
        }
    }

    /// The schema version this build creates and expects.
    pub fn schema_version(self) -> u32 {
        // A handful of migrations per database; cannot overflow.
        self.migrations().len() as u32
    }

    /// Pragmas that only take effect on an empty database file.
    fn new_file_pragmas(self) -> &'static [(&'static str, &'static str)] {
        match self {
            // Return space when messages leave the offline window (§5.2).
            Self::Blobs => &[("auto_vacuum", "INCREMENTAL")],
            Self::Mail | Self::Pim => &[],
        }
    }
}

/// Opens `path` in `mode`, applying pending migrations in read-write mode.
pub(crate) fn open(path: &Path, kind: DbKind, mode: Mode) -> Result<Connection> {
    match mode {
        Mode::ReadWrite => open_read_write(path, kind),
        Mode::ReadOnly => open_read_only(path, kind),
    }
}

fn open_read_write(path: &Path, kind: DbKind) -> Result<Connection> {
    let mut conn = Connection::open(path)?;
    conn.busy_timeout(BUSY_TIMEOUT)?;
    if user_version(&conn)? == 0 {
        for (name, value) in kind.new_file_pragmas() {
            conn.pragma_update(None, name, value)?;
        }
    }
    // WAL lets the apps read while the daemon writes (§5.1).
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    conn.pragma_update(None, "foreign_keys", true)?;
    let version = user_version(&conn)?;
    if version > 0 && version < kind.schema_version() {
        back_up(&conn, path, version)?;
    }
    migrate(&mut conn, path, kind)?;
    Ok(conn)
}

/// Copies of each database kept in `backup/` beside it.
const BACKUPS_KEPT: usize = 2;

/// Free space left over after a backup, so the migration and the WAL still
/// have room.
const BACKUP_HEADROOM: u64 = 64 * 1024 * 1024;

/// The directory that holds `path`'s backups.
pub fn backup_dir(path: &Path) -> PathBuf {
    path.parent().unwrap_or(Path::new(".")).join("backup")
}

/// Copies the database at `path`, still at schema `version`, to
/// `backup/<name>-v<version>-<unix time>.db` and removes all but the newest
/// [`BACKUPS_KEPT`] copies of it.
fn back_up(conn: &Connection, path: &Path, version: u32) -> Result<()> {
    let dir = backup_dir(path);
    std::fs::create_dir_all(&dir).map_err(|err| Error::io(&dir, err))?;
    let pragma = |name: &str| -> Result<u64> {
        let value: i64 = conn.pragma_query_value(None, name, |row| row.get(0))?;
        Ok(u64::try_from(value).unwrap_or(0))
    };
    let used = pragma("page_count")?.saturating_sub(pragma("freelist_count")?);
    let needed = used * pragma("page_size")? + BACKUP_HEADROOM;
    let available = fs4::available_space(&dir).map_err(|err| Error::io(&dir, err))?;
    if available < needed {
        return Err(Error::NoRoomForBackup {
            path: path.to_owned(),
            needed,
            available,
        });
    }
    let name = db_name(path);
    let target = dir.join(format!("{name}-v{version}-{}.db", unix_now()));
    let partial = target.with_extension("partial");
    let _ = std::fs::remove_file(&partial);
    tracing::info!(db = %path.display(), backup = %target.display(), "backing up before migrating");
    // VACUUM INTO writes a consistent, compact copy while holding only a
    // read transaction.
    conn.execute("VACUUM INTO ?1", [partial.to_string_lossy()])?;
    std::fs::rename(&partial, &target).map_err(|err| Error::io(&target, err))?;
    for old in backups(path).into_iter().skip(BACKUPS_KEPT) {
        if let Err(err) = std::fs::remove_file(&old.path) {
            tracing::warn!(backup = %old.path.display(), %err, "could not remove an old backup");
        }
    }
    Ok(())
}

/// The file name of `path` without `.db`, as used in backup names.
fn db_name(path: &Path) -> String {
    path.file_stem()
        .map_or_else(|| "db".to_owned(), |s| s.to_string_lossy().into_owned())
}

/// One backup of a database, made before a migration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Backup {
    pub path: PathBuf,
    /// The schema version the database had when it was copied.
    pub version: u32,
    /// When the copy was made, in Unix seconds.
    pub made_at: i64,
}

/// The backups of the database at `path`, newest first.
pub fn backups(path: &Path) -> Vec<Backup> {
    let prefix = format!("{}-v", db_name(path));
    let Ok(entries) = std::fs::read_dir(backup_dir(path)) else {
        return Vec::new();
    };
    let mut found: Vec<Backup> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let name = path.file_name()?.to_str()?;
            let rest = name.strip_prefix(&prefix)?.strip_suffix(".db")?;
            let (version, made_at) = rest.split_once('-')?;
            Some(Backup {
                version: version.parse().ok()?,
                made_at: made_at.parse().ok()?,
                path,
            })
        })
        .collect();
    found.sort_by(|a, b| b.made_at.cmp(&a.made_at).then(b.version.cmp(&a.version)));
    found
}

/// Whether a database newer than `supported` says this build can read it.
fn readable(conn: &Connection, supported: u32) -> Result<bool> {
    Ok(min_reader_version(conn)?.is_some_and(|min| min <= supported))
}

/// `schema_meta.min_reader_version`, if the database has it.
pub(crate) fn min_reader_version(conn: &Connection) -> Result<Option<u32>> {
    let has_meta: bool = conn.query_row(
        "SELECT EXISTS (SELECT 1 FROM sqlite_schema WHERE type = 'table' AND name = 'schema_meta')",
        [],
        |row| row.get(0),
    )?;
    if !has_meta {
        return Ok(None);
    }
    let value: Option<String> = conn
        .query_row(
            "SELECT value FROM schema_meta WHERE key = 'min_reader_version'",
            [],
            |row| row.get(0),
        )
        .optional()?;
    Ok(value.and_then(|v| v.parse().ok()))
}

fn open_read_only(path: &Path, kind: DbKind) -> Result<Connection> {
    if !path.exists() {
        return Err(Error::NotFound {
            path: path.to_owned(),
        });
    }
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY
        | OpenFlags::SQLITE_OPEN_NO_MUTEX
        | OpenFlags::SQLITE_OPEN_URI;
    let conn = Connection::open_with_flags(path, flags)?;
    conn.busy_timeout(BUSY_TIMEOUT)?;
    conn.pragma_update(None, "query_only", true)?;
    let found = user_version(&conn)?;
    let expected = kind.schema_version();
    if found > expected {
        if readable(&conn, expected)? {
            return Ok(conn);
        }
        return Err(too_new(path, found, expected));
    }
    if found < expected {
        return Err(Error::SchemaOutdated {
            path: path.to_owned(),
            found,
            expected,
        });
    }
    Ok(conn)
}

/// Applies every pending migration, each in its own transaction.
fn migrate(conn: &mut Connection, path: &Path, kind: DbKind) -> Result<()> {
    let migrations = kind.migrations();
    let supported = kind.schema_version();
    loop {
        // IMMEDIATE takes the write lock before reading the version, so two
        // processes cannot apply the same migration.
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let version = user_version(&tx)?;
        if version > supported {
            if readable(&tx, supported)? {
                tracing::info!(db = %path.display(), found = version, supported, "opening a newer schema this build can read");
                return Ok(());
            }
            return Err(too_new(path, version, supported));
        }
        let Some(sql) = migrations.get(version as usize) else {
            return Ok(());
        };
        tracing::info!(db = %path.display(), from = version, to = version + 1, "migrating schema");
        apply_migration(&tx, sql, version + 1)?;
        tx.commit()?;
    }
}

/// Runs one migration's SQL and records `to` as the schema version.
pub(crate) fn apply_migration(conn: &Connection, sql: &str, to: u32) -> Result<()> {
    conn.execute_batch(sql)?;
    // A migration that rewrote the schema's text (pim_v12) bumps the
    // schema cookie, so every connection reads the new text.
    if sql.contains("writable_schema") {
        let cookie: i64 = conn.pragma_query_value(None, "schema_version", |row| row.get(0))?;
        conn.pragma_update(None, "schema_version", cookie + 1)?;
    }
    conn.pragma_update(None, "user_version", to)?;
    Ok(())
}

pub(crate) fn user_version(conn: &Connection) -> Result<u32> {
    Ok(conn.pragma_query_value(None, "user_version", |row| row.get(0))?)
}

fn too_new(path: &Path, found: u32, supported: u32) -> Error {
    Error::SchemaTooNew {
        path: PathBuf::from(path),
        found,
        supported,
    }
}

/// Current time as Unix seconds, the time format of every table.
pub(crate) fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [DbKind; 3] = [DbKind::Mail, DbKind::Pim, DbKind::Blobs];

    fn tables(conn: &Connection) -> Vec<String> {
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_schema WHERE type = 'table' ORDER BY name")
            .unwrap();
        stmt.query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap()
    }

    #[test]
    fn creates_every_schema_from_scratch() {
        let tmp = tempfile::tempdir().unwrap();
        for kind in ALL {
            let path = tmp.path().join(format!("{kind:?}.db"));
            let conn = open(&path, kind, Mode::ReadWrite).unwrap();
            assert_eq!(user_version(&conn).unwrap(), kind.schema_version());
            let mode: String = conn
                .pragma_query_value(None, "journal_mode", |row| row.get(0))
                .unwrap();
            assert_eq!(mode, "wal");
            let fk: bool = conn
                .pragma_query_value(None, "foreign_keys", |row| row.get(0))
                .unwrap();
            assert!(fk);
            let issues: Vec<String> = conn
                .prepare("PRAGMA foreign_key_check")
                .unwrap()
                .query_map([], |row| row.get(0))
                .unwrap()
                .collect::<Result<_, _>>()
                .unwrap();
            assert!(issues.is_empty(), "{issues:?}");
        }
    }

    #[test]
    fn schema_has_the_documented_tables() {
        let tmp = tempfile::tempdir().unwrap();
        let mail = open(&tmp.path().join("mail.db"), DbKind::Mail, Mode::ReadWrite).unwrap();
        assert_eq!(
            tables(&mail),
            [
                "attachment",
                "change_log",
                "chat_pin",
                "conversation_summary",
                "folder",
                "folder_alert",
                "message",
                "message_location",
                "mute",
                "notification",
                "op_queue",
                "outbox",
                "participant",
                "pin",
                "pop3_uidl",
                "quota",
                "receipt",
                "receipt_mail",
                "schema_meta",
                "thread",
                "thread_ref",
                "translation",
            ]
        );
        let pim = open(&tmp.path().join("pim.db"), DbKind::Pim, Mode::ReadWrite).unwrap();
        assert_eq!(
            tables(&pim),
            [
                "account",
                "address_book",
                "calendar",
                "change_log",
                "contact",
                "contact_address",
                "contact_group",
                "contact_group_member",
                "contact_photo",
                "event",
                "mail_rule",
                "mail_rule_note",
                "mail_rule_remote",
                "mail_rule_server",
                "mail_rule_starter",
                "meta",
                "note",
                "note_gone",
                "note_picture",
                "note_reminder",
                "note_version",
                "org_alias",
                "org_member",
                "org_rule",
                "organization",
                "other_contact",
                "other_contact_sync",
                "schema_meta",
                "suggestion",
                "task",
                "task_file",
                "task_labels",
                "task_list",
                "template",
                "template_attachment",
                "tracked_message",
                "tracked_recipient",
                "tracking_event",
            ]
        );
    }

    #[test]
    fn blobs_db_uses_incremental_auto_vacuum() {
        let tmp = tempfile::tempdir().unwrap();
        let conn = open(&tmp.path().join("b.db"), DbKind::Blobs, Mode::ReadWrite).unwrap();
        let auto_vacuum: i64 = conn
            .pragma_query_value(None, "auto_vacuum", |row| row.get(0))
            .unwrap();
        assert_eq!(auto_vacuum, 2, "2 = INCREMENTAL");
    }

    #[test]
    fn pim_v12_lets_calendars_come_from_zoho_and_keeps_every_event() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("pim.db");
        let conn = Connection::open(&path).unwrap();
        conn.pragma_update(None, "foreign_keys", true).unwrap();
        for sql in &DbKind::Pim.migrations()[..11] {
            conn.execute_batch(sql).unwrap();
        }
        conn.pragma_update(None, "user_version", 11).unwrap();
        conn.execute_batch(
            "INSERT INTO calendar (id, source, name) VALUES (1, 'caldav', 'Work');
             INSERT INTO event (id, calendar_id, title, start, end) VALUES (1, 1, 'Kept', 0, 60);",
        )
        .unwrap();
        assert!(
            conn.execute(
                "INSERT INTO calendar (source, name) VALUES ('zoho', 'Z')",
                []
            )
            .is_err()
        );
        drop(conn);

        let conn = open(&path, DbKind::Pim, Mode::ReadWrite).unwrap();
        assert_eq!(user_version(&conn).unwrap(), DbKind::Pim.schema_version());
        conn.execute(
            "INSERT INTO calendar (source, name) VALUES ('zoho', 'Z')",
            [],
        )
        .unwrap();
        assert!(
            conn.execute(
                "INSERT INTO calendar (source, name) VALUES ('other', 'O')",
                []
            )
            .is_err()
        );
        let title: String = conn
            .query_row("SELECT title FROM event WHERE id = 1", [], |row| row.get(0))
            .unwrap();
        assert_eq!(title, "Kept");
        let ok: String = conn
            .query_row("PRAGMA integrity_check", [], |row| row.get(0))
            .unwrap();
        assert_eq!(ok, "ok");
        // Other connections read the new schema too.
        let other = Connection::open(&path).unwrap();
        other
            .execute(
                "INSERT INTO calendar (source, name) VALUES ('zoho', 'Z2')",
                [],
            )
            .unwrap();
    }

    #[test]
    fn mail_v2_upgrades_a_v1_store() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("mail.db");
        let conn = Connection::open(&path).unwrap();
        conn.execute_batch(DbKind::Mail.migrations()[0]).unwrap();
        conn.pragma_update(None, "user_version", 1).unwrap();
        conn.execute_batch(
            "INSERT INTO folder (id, account_id, path) VALUES (1, 1, 'INBOX');
             INSERT INTO message (id, account_id, subject) VALUES (1, 1, 'Old');
             INSERT INTO message_location (message_id, folder_id) VALUES (1, 1);",
        )
        .unwrap();
        drop(conn);

        let conn = open(&path, DbKind::Mail, Mode::ReadWrite).unwrap();
        assert_eq!(user_version(&conn).unwrap(), DbKind::Mail.schema_version());
        let row: (Option<i64>, Option<i64>) = conn
            .query_row(
                "SELECT thread_id, category FROM message WHERE id = 1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(row, (None, None), "threaded later by the backfill");
        // The triggers keep thread counts from now on.
        conn.execute_batch(
            "INSERT INTO thread (id, account_id) VALUES (7, 1);
             UPDATE message SET thread_id = 7 WHERE id = 1;",
        )
        .unwrap();
        let count: i64 = conn
            .query_row("SELECT message_count FROM thread WHERE id = 7", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn reopening_is_idempotent() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("mail.db");
        drop(open(&path, DbKind::Mail, Mode::ReadWrite).unwrap());
        let conn = open(&path, DbKind::Mail, Mode::ReadWrite).unwrap();
        assert_eq!(user_version(&conn).unwrap(), DbKind::Mail.schema_version());
    }

    #[test]
    fn refuses_newer_schema() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("pim.db");
        let conn = open(&path, DbKind::Pim, Mode::ReadWrite).unwrap();
        conn.pragma_update(None, "user_version", 99).unwrap();
        conn.execute(
            "UPDATE schema_meta SET value = '99' WHERE key = 'min_reader_version'",
            [],
        )
        .unwrap();
        drop(conn);
        for mode in [Mode::ReadWrite, Mode::ReadOnly] {
            let err = open(&path, DbKind::Pim, mode).unwrap_err();
            assert!(
                matches!(err, Error::SchemaTooNew { found: 99, .. }),
                "{mode:?}: {err:?}"
            );
        }
    }

    #[test]
    fn opens_a_newer_schema_that_says_it_is_readable() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("mail.db");
        let conn = open(&path, DbKind::Mail, Mode::ReadWrite).unwrap();
        let current = DbKind::Mail.schema_version();
        // A later release added something and kept min_reader_version.
        conn.execute_batch("CREATE TABLE later (id INTEGER PRIMARY KEY)")
            .unwrap();
        conn.pragma_update(None, "user_version", current + 1)
            .unwrap();
        drop(conn);
        for mode in [Mode::ReadWrite, Mode::ReadOnly] {
            let conn = open(&path, DbKind::Mail, mode).unwrap();
            assert_eq!(user_version(&conn).unwrap(), current + 1, "{mode:?}");
        }
        assert!(backups(&path).is_empty(), "nothing was migrated");
    }

    #[test]
    fn min_reader_version_is_recorded() {
        let tmp = tempfile::tempdir().unwrap();
        for (kind, min) in [(DbKind::Mail, 14), (DbKind::Pim, 17), (DbKind::Blobs, 1)] {
            let conn = open(
                &tmp.path().join(format!("{kind:?}.db")),
                kind,
                Mode::ReadWrite,
            )
            .unwrap();
            assert_eq!(min_reader_version(&conn).unwrap(), Some(min), "{kind:?}");
        }
    }

    #[test]
    fn backs_up_before_migrating_and_keeps_two() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("mail.db");
        drop(open(&path, DbKind::Mail, Mode::ReadWrite).unwrap());
        assert!(backups(&path).is_empty(), "a new database needs no backup");
        let current = DbKind::Mail.schema_version();
        for round in 0..3 {
            // Pretend the last migration is still to run, and make each
            // backup name distinct.
            let conn = Connection::open(&path).unwrap();
            conn.execute_batch("DROP TABLE schema_meta").unwrap();
            conn.pragma_update(None, "user_version", current - 1)
                .unwrap();
            drop(conn);
            let dir = backup_dir(&path);
            std::fs::create_dir_all(&dir).unwrap();
            if round > 0 {
                for old in backups(&path) {
                    let older = dir.join(format!("mail-v{}-{}.db", old.version, old.made_at - 10));
                    std::fs::rename(&old.path, older).unwrap();
                }
            }
            let conn = open(&path, DbKind::Mail, Mode::ReadWrite).unwrap();
            assert_eq!(user_version(&conn).unwrap(), current);
        }
        let kept = backups(&path);
        assert_eq!(kept.len(), BACKUPS_KEPT);
        let copy = Connection::open(&kept[0].path).unwrap();
        assert_eq!(kept[0].version, current - 1);
        assert_eq!(user_version(&copy).unwrap(), current - 1);
        assert!(kept[0].made_at > kept[1].made_at);
    }

    #[test]
    fn read_only_needs_an_existing_migrated_database() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("mail.db");
        assert!(matches!(
            open(&path, DbKind::Mail, Mode::ReadOnly),
            Err(Error::NotFound { .. })
        ));
        assert!(!path.exists(), "read-only open must not create the file");

        drop(Connection::open(&path).unwrap()); // empty, version 0
        assert!(matches!(
            open(&path, DbKind::Mail, Mode::ReadOnly),
            Err(Error::SchemaOutdated { found: 0, .. })
        ));
    }

    #[test]
    fn read_only_rejects_writes() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("mail.db");
        drop(open(&path, DbKind::Mail, Mode::ReadWrite).unwrap());
        let conn = open(&path, DbKind::Mail, Mode::ReadOnly).unwrap();
        assert!(
            conn.execute(
                "INSERT INTO folder (account_id, path) VALUES (1, 'INBOX')",
                []
            )
            .is_err()
        );
        let count: i64 = conn
            .query_row("SELECT count(*) FROM folder", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn reader_sees_writes_while_writer_is_open() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("mail.db");
        let writer = open(&path, DbKind::Mail, Mode::ReadWrite).unwrap();
        let reader = open(&path, DbKind::Mail, Mode::ReadOnly).unwrap();
        writer
            .execute(
                "INSERT INTO folder (account_id, path) VALUES (1, 'INBOX')",
                [],
            )
            .unwrap();
        let count: i64 = reader
            .query_row("SELECT count(*) FROM folder", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn schema_constraints_hold() {
        let tmp = tempfile::tempdir().unwrap();
        let conn = open(&tmp.path().join("mail.db"), DbKind::Mail, Mode::ReadWrite).unwrap();
        // Bad enum value.
        assert!(
            conn.execute(
                "INSERT INTO message (account_id, body_state) VALUES (1, 7)",
                []
            )
            .is_err()
        );
        // Deleting a message cascades to its locations and participants.
        conn.execute_batch(
            "INSERT INTO folder (id, account_id, path) VALUES (1, 1, 'INBOX');
             INSERT INTO message (id, account_id) VALUES (1, 1);
             INSERT INTO message_location (message_id, folder_id, uid) VALUES (1, 1, 42);
             INSERT INTO participant (message_id, role, email_norm, domain)
                 VALUES (1, 'from', 'ada@example.org', 'example.org');
             DELETE FROM message WHERE id = 1;",
        )
        .unwrap();
        for table in ["message_location", "participant"] {
            let count: i64 = conn
                .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| {
                    row.get(0)
                })
                .unwrap();
            assert_eq!(count, 0, "{table}");
        }
    }
}
