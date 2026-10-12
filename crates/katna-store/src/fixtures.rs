// SPDX-License-Identifier: GPL-3.0-or-later

//! Migration fixtures (plan U.2): a committed database of every released
//! schema version, each migrated to the current version and checked.
//!
//! `crates/katna-store/fixtures/<db>_v<N>.db.zst` is a zstd-compressed
//! SQLite file with `user_version` N and a few rows in every table, made by
//! applying migrations 1..=N to an empty file and filling it. A fixture is
//! written once, when its migration is added, and never changed after:
//!
//! ```sh
//! KATNA_WRITE_FIXTURES=1 cargo test -p katna-store write_missing_fixtures -- --ignored
//! ```
//!
//! `answers.txt` beside them holds what the store's read calls return for
//! each migrated fixture; `KATNA_WRITE_FIXTURES=1` on the normal test
//! rewrites it, and the diff shows what a migration changed.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use katna_core::Paths;
use regex::Regex;
use rusqlite::types::Value;
use rusqlite::{Connection, params_from_iter};

use crate::db::{self, DbKind, Mode};
use crate::{FolderId, Store};

const KINDS: [DbKind; 3] = [DbKind::Mail, DbKind::Pim, DbKind::Blobs];

/// Rows written to every table.
const ROWS: usize = 3;

fn dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

fn stem(kind: DbKind) -> &'static str {
    match kind {
        DbKind::Mail => "mail",
        DbKind::Pim => "pim",
        DbKind::Blobs => "blobs",
    }
}

fn file_name(kind: DbKind) -> &'static str {
    match kind {
        DbKind::Mail => "mail.db",
        DbKind::Pim => "pim.db",
        DbKind::Blobs => "blobs.db",
    }
}

fn fixture(kind: DbKind, version: u32) -> PathBuf {
    dir().join(format!("{}_v{version}.db.zst", stem(kind)))
}

fn rewrite() -> bool {
    std::env::var_os("KATNA_WRITE_FIXTURES").is_some()
}

/// Writes the fixture's database to `to`.
fn unpack(kind: DbKind, version: u32, to: &Path) {
    let path = fixture(kind, version);
    let packed = std::fs::read(&path).unwrap_or_else(|err| {
        panic!(
            "{}: {err}; add it with KATNA_WRITE_FIXTURES=1 cargo test -p katna-store \
             write_missing_fixtures -- --ignored",
            path.display()
        )
    });
    let data = zstd::stream::decode_all(packed.as_slice()).unwrap();
    std::fs::write(to, data).unwrap();
}

fn user_tables(conn: &Connection) -> Vec<String> {
    let mut stmt = conn
        .prepare(
            "SELECT name FROM sqlite_schema
             WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name",
        )
        .unwrap();
    stmt.query_map([], |row| row.get(0))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap()
}

fn counts(conn: &Connection) -> BTreeMap<String, i64> {
    user_tables(conn)
        .into_iter()
        .map(|table| {
            let n = conn
                .query_row(&format!("SELECT count(*) FROM \"{table}\""), [], |row| {
                    row.get(0)
                })
                .unwrap();
            (table, n)
        })
        .collect()
}

fn schema(conn: &Connection) -> Vec<(String, String, Option<String>)> {
    let mut stmt = conn
        .prepare("SELECT type, name, sql FROM sqlite_schema ORDER BY type, name")
        .unwrap();
    stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap()
}

/// Tables whose rows a migration removes on purpose: (database, the
/// migration that removes them, table).
const DROPPED: &[(DbKind, u32, &str)] = &[
    // pim_v6 remade the v1 contact tables, which nothing had written.
    (DbKind::Pim, 6, "contact"),
    (DbKind::Pim, 6, "contact_address"),
    (DbKind::Pim, 6, "org_member"),
];

#[test]
fn every_schema_version_has_a_fixture() {
    for kind in KINDS {
        for version in 1..=kind.schema_version() {
            assert!(
                fixture(kind, version).exists(),
                "no fixture for {} v{version}: run KATNA_WRITE_FIXTURES=1 cargo test \
                 -p katna-store write_missing_fixtures -- --ignored",
                stem(kind)
            );
        }
    }
}

#[test]
fn every_fixture_migrates_to_the_current_schema_and_keeps_its_rows() {
    let tmp = tempfile::tempdir().unwrap();
    for kind in KINDS {
        let fresh = db::open(
            &tmp.path().join(format!("fresh-{}", file_name(kind))),
            kind,
            Mode::ReadWrite,
        )
        .unwrap();
        for version in 1..=kind.schema_version() {
            let path = tmp.path().join(format!("{}_v{version}.db", stem(kind)));
            unpack(kind, version, &path);
            let before = {
                let conn = Connection::open(&path).unwrap();
                assert_eq!(db::user_version(&conn).unwrap(), version);
                counts(&conn)
            };
            let conn = db::open(&path, kind, Mode::ReadWrite)
                .unwrap_or_else(|err| panic!("{} v{version}: {err}", stem(kind)));
            let at = format!("{} v{version}", stem(kind));
            assert_eq!(
                db::user_version(&conn).unwrap(),
                kind.schema_version(),
                "{at}"
            );
            let ok: String = conn
                .query_row("PRAGMA integrity_check", [], |row| row.get(0))
                .unwrap();
            assert_eq!(ok, "ok", "{at}");
            let broken: Vec<String> = conn
                .prepare("PRAGMA foreign_key_check")
                .unwrap()
                .query_map([], |row| row.get(0))
                .unwrap()
                .collect::<Result<_, _>>()
                .unwrap();
            assert!(broken.is_empty(), "{at}: foreign keys broken in {broken:?}");
            assert_eq!(schema(&conn), schema(&fresh), "{at}: schema differs");
            let after = counts(&conn);
            for (table, rows) in before {
                let dropped = DROPPED
                    .iter()
                    .any(|&(k, v, t)| k == kind && v > version && t == table);
                if dropped {
                    continue;
                }
                // A table a later migration renamed or folded into another
                // is gone; its rows are checked through the answers below.
                if let Some(&now) = after.get(&table) {
                    assert!(now >= rows, "{at}: {table} had {rows} rows, now {now}");
                }
            }
        }
    }
}

/// The newest fixture of each database, for pairing with an older one.
fn latest(kind: DbKind) -> u32 {
    kind.schema_version()
}

/// What the store's read calls return, one line each.
fn answers(store: &Store) -> String {
    fn line<T>(out: &mut String, name: &str, got: crate::Result<T>, show: impl Fn(T) -> String) {
        let shown = match got {
            Ok(value) => show(value),
            Err(err) => format!("error: {err}"),
        };
        writeln!(out, "  {name}: {shown}").unwrap();
    }
    let len = |n: usize| n.to_string();
    let mut out = String::new();
    let o = &mut out;
    line(o, "accounts", store.accounts(), |v| len(v.len()));
    line(o, "message_count", store.message_count(), |n| n.to_string());
    line(o, "folder_summaries", store.folder_summaries(), |v| {
        len(v.len())
    });
    line(o, "unread_counts", store.unread_counts(), |v| len(v.len()));
    line(
        o,
        "messages_after",
        store.messages_after(crate::MessageId(0), 100),
        |v| len(v.len()),
    );
    line(
        o,
        "folder_messages",
        store.folder_message_ids(FolderId(1)),
        |v| len(v.len()),
    );
    line(o, "people", store.people(100), |v| len(v.len()));
    line(
        o,
        "correspondents",
        store.correspondents(),
        |v| len(v.len()),
    );
    line(o, "library_files", store.library_files(100), |v| {
        len(v.len())
    });
    line(o, "outbox", store.outbox(), |v| len(v.len()));
    line(o, "pinned", store.pinned(), |v| len(v.len()));
    line(o, "waiting", store.waiting(), |v| len(v.len()));
    line(o, "folder_bells", store.folder_bells(), |v| len(v.len()));
    line(o, "calendars", store.calendars(), |v| len(v.len()));
    line(o, "task_lists", store.task_lists(), |v| len(v.len()));
    line(o, "notes", store.notes(), |v| len(v.len()));
    line(o, "address_books", store.address_books(), |v| len(v.len()));
    line(
        o,
        "saved_contacts",
        store.saved_contacts(),
        |v| len(v.len()),
    );
    line(
        o,
        "contact_labels",
        store.contact_labels(),
        |v| len(v.len()),
    );
    line(
        o,
        "other_contacts",
        store.other_contacts(),
        |v| len(v.len()),
    );
    line(o, "templates", store.templates(), |v| len(v.len()));
    line(o, "rules", store.rules(), |v| len(v.len()));
    for db in [DbKind::Mail, DbKind::Pim] {
        line(
            o,
            &format!("changes_since {db:?}"),
            store.changes_since(db, 0, 1000),
            |v| len(v.len()),
        );
    }
    out
}

#[test]
fn migrated_fixtures_answer_the_same_reads() {
    let tmp = tempfile::tempdir().unwrap();
    let mut report = String::new();
    for kind in KINDS {
        for version in 1..=kind.schema_version() {
            let root = tmp.path().join(format!("{}_v{version}", stem(kind)));
            let paths = Paths::with_root(&root);
            paths.create_dirs().unwrap();
            // The other two databases come from their newest fixture.
            for other in KINDS {
                let v = if other == kind {
                    version
                } else {
                    latest(other)
                };
                let to = paths.data_dir().join(file_name(other));
                unpack(other, v, &to);
            }
            let store = Store::open(&paths, Mode::ReadWrite)
                .unwrap_or_else(|err| panic!("{} v{version}: {err}", stem(kind)));
            writeln!(report, "{} v{version}", stem(kind)).unwrap();
            report.push_str(&answers(&store));
        }
    }
    let path = dir().join("answers.txt");
    if rewrite() {
        std::fs::write(&path, &report).unwrap();
        return;
    }
    let expected = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(
        expected == report,
        "reads on migrated fixtures changed; if that is intended, run \
         KATNA_WRITE_FIXTURES=1 cargo test -p katna-store fixtures and review \
         the diff of {}\n{report}",
        path.display()
    );
}

/// Writes a fixture for every schema version that has none. Existing
/// fixtures are never rewritten: they stand for released databases.
#[test]
#[ignore = "writes files; run with KATNA_WRITE_FIXTURES=1 when adding a migration"]
fn write_missing_fixtures() {
    if !rewrite() {
        return;
    }
    std::fs::create_dir_all(dir()).unwrap();
    let tmp = tempfile::tempdir().unwrap();
    for kind in KINDS {
        for version in 1..=kind.schema_version() {
            let out = fixture(kind, version);
            if out.exists() {
                continue;
            }
            let path = tmp.path().join(format!("{}_v{version}.db", stem(kind)));
            let conn = Connection::open(&path).unwrap();
            conn.pragma_update(None, "page_size", 1024).unwrap();
            for (index, sql) in kind.migrations()[..version as usize].iter().enumerate() {
                // At most a few dozen migrations; cannot overflow.
                db::apply_migration(&conn, sql, index as u32 + 1).unwrap();
            }
            fill(&conn, kind);
            conn.execute_batch("VACUUM").unwrap();
            drop(conn);
            let data = std::fs::read(&path).unwrap();
            let packed = zstd::bulk::compress(&data, 19).unwrap();
            std::fs::write(&out, packed).unwrap();
            println!("wrote {}", out.display());
        }
    }
}

struct Column {
    name: String,
    decl: String,
    not_null: bool,
    has_default: bool,
    pk: bool,
}

/// Values for columns whose text the store parses. Everything else gets a
/// value from its type and name.
fn known(kind: DbKind, table: &str, column: &str, row: usize) -> Option<Value> {
    let text = |s: &str| Some(Value::Text(s.to_owned()));
    let _ = row;
    // Inline blobs only: a file blob would need its file beside the db.
    if (kind, table, column) == (DbKind::Blobs, "blob", "storage") {
        return Some(Value::Integer(0));
    }
    match column {
        "settings_json" | "payload_json" | "value_json" => text("{}"),
        "conditions_json" | "actions_json" | "accounts_json" | "attendees_json" | "links_json"
        | "keywords" => text("[]"),
        "object_kind" => text("message"),
        _ => None,
    }
}

fn columns(conn: &Connection, table: &str) -> Vec<Column> {
    let mut stmt = conn
        .prepare(&format!("PRAGMA table_xinfo(\"{table}\")"))
        .unwrap();
    stmt.query_map([], |row| {
        let hidden: i64 = row.get(6)?;
        Ok((
            hidden,
            Column {
                name: row.get(1)?,
                decl: row.get::<_, String>(2)?.to_uppercase(),
                not_null: row.get::<_, i64>(3)? != 0,
                has_default: row.get::<_, Option<String>>(4)?.is_some(),
                pk: row.get::<_, i64>(5)? != 0,
            },
        ))
    })
    .unwrap()
    .filter_map(|row| {
        let (hidden, column) = row.unwrap();
        (hidden == 0).then_some(column)
    })
    .collect()
}

/// (column, parent table, parent column) for each foreign key of `table`.
fn foreign_keys(conn: &Connection, table: &str) -> Vec<(String, String, Option<String>)> {
    let mut stmt = conn
        .prepare(&format!("PRAGMA foreign_key_list(\"{table}\")"))
        .unwrap();
    stmt.query_map([], |row| Ok((row.get(3)?, row.get(2)?, row.get(4)?)))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap()
}

/// Literals allowed by `CHECK (column IN (...))` in the table's SQL.
fn allowed(sql: &str) -> HashMap<String, Vec<Value>> {
    let re = Regex::new(r"(?i)\b(\w+)\s+IN\s*\(([^()]*)\)").unwrap();
    let mut out = HashMap::new();
    for cap in re.captures_iter(sql) {
        let values = cap[2]
            .split(',')
            .map(|v| {
                let v = v.trim();
                if let Some(s) = v.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')) {
                    Value::Text(s.to_owned())
                } else {
                    Value::Integer(v.parse().unwrap_or(0))
                }
            })
            .collect();
        out.insert(cap[1].to_lowercase(), values);
    }
    out
}

/// Fills every table with [`ROWS`] rows, parents before children.
fn fill(conn: &Connection, kind: DbKind) {
    conn.pragma_update(None, "foreign_keys", true).unwrap();
    let mut pending = user_tables(conn);
    let mut done: Vec<String> = Vec::new();
    while !pending.is_empty() {
        let before = pending.len();
        pending.retain(|table| {
            let ready = foreign_keys(conn, table)
                .iter()
                .all(|(_, parent, _)| parent == table || done.contains(parent));
            if ready {
                fill_table(conn, kind, table);
                done.push(table.clone());
            }
            !ready
        });
        assert!(
            pending.len() < before,
            "foreign key cycle among {pending:?}"
        );
    }
}

fn fill_table(conn: &Connection, kind: DbKind, table: &str) {
    let sql: String = conn
        .query_row(
            "SELECT sql FROM sqlite_schema WHERE type = 'table' AND name = ?1",
            [table],
            |row| row.get(0),
        )
        .unwrap();
    let allowed = allowed(&sql);
    let fks = foreign_keys(conn, table);
    let cols = columns(conn, table);
    let single_int_pk = cols.iter().filter(|c| c.pk).count() == 1
        && cols.iter().any(|c| c.pk && c.decl == "INTEGER");
    for row in 0..ROWS {
        let mut names = Vec::new();
        let mut values = Vec::new();
        for col in &cols {
            let lower = col.name.to_lowercase();
            let value = if let Some((_, parent, to)) = fks.iter().find(|(c, ..)| *c == col.name) {
                if parent == table {
                    if col.not_null {
                        Value::Integer(1)
                    } else {
                        Value::Null
                    }
                } else {
                    let to = to.clone().unwrap_or_else(|| "rowid".to_owned());
                    conn.query_row(
                        &format!(
                            "SELECT \"{to}\" FROM \"{parent}\" ORDER BY rowid LIMIT 1 OFFSET ?1"
                        ),
                        [row as i64],
                        |r| r.get::<_, Value>(0),
                    )
                    .unwrap()
                }
            } else if col.pk && single_int_pk {
                continue;
            } else if let Some(v) = known(kind, table, &col.name, row) {
                v
            } else if let Some(list) = allowed.get(&lower) {
                list[row % list.len()].clone()
            } else if col.has_default && !col.pk && lower.ends_with("_json") {
                continue;
            } else {
                guess(table, col, row)
            };
            names.push(format!("\"{}\"", col.name));
            values.push(value);
        }
        let marks = vec!["?"; values.len()].join(", ");
        let insert = if names.is_empty() {
            format!("INSERT INTO \"{table}\" DEFAULT VALUES")
        } else {
            format!(
                "INSERT INTO \"{table}\" ({}) VALUES ({marks})",
                names.join(", ")
            )
        };
        conn.execute(&insert, params_from_iter(values.iter()))
            .unwrap_or_else(|err| panic!("{} {table} row {row}: {err}", stem(kind)));
    }
}

/// A plausible value from the column's declared type and name.
fn guess(table: &str, col: &Column, row: usize) -> Value {
    let name = col.name.to_lowercase();
    let n = row as i64 + 1;
    if col.decl.contains("BLOB") || name.ends_with("hash") {
        return Value::Blob(vec![u8::try_from(n).unwrap_or(0); 32]);
    }
    if col.decl.contains("INT") {
        let timeish = [
            "_at", "time", "date", "until", "since", "_ts", "expires", "start", "end",
        ];
        if timeish.iter().any(|t| name.contains(t)) {
            return Value::Integer(1_760_000_000 + n * 3600);
        }
        return Value::Integer(n);
    }
    if col.decl.contains("REAL") {
        return Value::Real(n as f64 + 0.5);
    }
    if name.contains("email") || name.contains("address") || name == "addr" {
        return Value::Text(format!("person{n}@example.com"));
    }
    Value::Text(format!("{table}-{name}-{n}"))
}
