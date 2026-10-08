// SPDX-License-Identifier: GPL-3.0-or-later

//! Notes (`note` in `pim.db`, §13.11): Katna Notes' cards. A note of a
//! mail account is kept in that account's Notes folder too; `dirty` marks
//! one changed here and not written there yet, and `note_gone` lists the
//! server copies of notes deleted here.

use rusqlite::{OptionalExtension, Row, TransactionBehavior, params};

use crate::Store;
use crate::db::unix_now;
use crate::error::Result;
use crate::journal::{self, ChangeOp, ObjectKind};

/// How long a note stays in Trash before it is gone for good, in seconds
/// (Google Keep's seven days).
pub const NOTE_TRASH_KEEP: i64 = 7 * 24 * 60 * 60;

/// How long a note's earlier text is kept on this computer, in seconds.
pub const NOTE_VERSION_KEEP: i64 = 30 * 24 * 60 * 60;

/// Saves within this many seconds of the last make one version.
const VERSION_GAP: i64 = 10 * 60;

/// The start of a link to another note in a note's HTML, the note's UUID
/// after it.
pub const NOTE_LINK_SCHEME: &str = "katna-note:";

/// A note.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Note {
    /// 0 for one not saved yet.
    pub id: i64,
    /// The mail account whose Notes folder keeps it; `None` for this
    /// computer only.
    pub account_id: Option<i64>,
    /// Stable across computers and apps (`X-Universally-Unique-Identifier`).
    pub uuid: String,
    pub title: String,
    /// Plain text; lines starting "☐ " or "☑ " are checklist items.
    pub body: String,
    /// The same text formatted, as HTML with one paragraph per line of
    /// `body`; empty when it has no formatting.
    pub html: String,
    /// 0 for none, else a number in the Notes palette.
    pub color: i64,
    pub pinned: bool,
    pub archived: bool,
    pub labels: Vec<String>,
    /// The `Message-ID` of the mail the note is about.
    pub link: Option<String>,
    /// Larger shows first among notes of the same kind.
    pub position: i64,
    pub created_at: i64,
    pub updated_at: i64,
    /// When it went to Trash.
    pub trashed_at: Option<i64>,
    /// Its UID in the account's Notes folder, once written there.
    pub server_uid: Option<i64>,
    /// Changed here, and not yet on the server.
    pub dirty: bool,
    /// When it reminds, in UTC seconds.
    pub remind_at: Option<i64>,
}

/// A picture in a note, named in its HTML as `cid:<cid>`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NotePicture {
    pub cid: String,
    pub name: String,
    pub mime: String,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

/// Earlier text of a note, kept for [`NOTE_VERSION_KEEP`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NoteVersion {
    pub id: i64,
    /// When it was last like this.
    pub at: i64,
    pub title: String,
    pub body: String,
    pub html: String,
    /// Where it was written.
    pub source: VersionSource,
}

/// Where a version of a note was written.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum VersionSource {
    /// On this computer.
    #[default]
    Here,
    /// On another device, named when its mail app says which.
    Elsewhere(Option<String>),
}

impl VersionSource {
    fn to_sql(&self) -> String {
        match self {
            VersionSource::Here => "here".to_owned(),
            VersionSource::Elsewhere(None) => "sync".to_owned(),
            VersionSource::Elsewhere(Some(device)) => format!("sync:{device}"),
        }
    }

    fn from_sql(text: &str) -> Self {
        match text {
            "here" => VersionSource::Here,
            "sync" => VersionSource::Elsewhere(None),
            other => VersionSource::Elsewhere(
                other
                    .strip_prefix("sync:")
                    .filter(|d| !d.is_empty())
                    .map(str::to_owned),
            ),
        }
    }
}

const COLUMNS: &str = "id, account_id, uuid, title, body, color, pinned, archived, labels, link,
     position, created_at, updated_at, trashed_at, server_uid, dirty, html,
     (SELECT at FROM note_reminder WHERE note_id = note.id)";

fn note_row(row: &Row<'_>) -> rusqlite::Result<Note> {
    let labels: String = row.get(8)?;
    Ok(Note {
        id: row.get(0)?,
        account_id: row.get(1)?,
        uuid: row.get(2)?,
        title: row.get(3)?,
        body: row.get(4)?,
        color: row.get(5)?,
        pinned: row.get(6)?,
        archived: row.get(7)?,
        labels: serde_json::from_str(&labels).unwrap_or_default(),
        link: row.get(9)?,
        position: row.get(10)?,
        created_at: row.get(11)?,
        updated_at: row.get(12)?,
        trashed_at: row.get(13)?,
        server_uid: row.get(14)?,
        dirty: row.get(15)?,
        html: row.get(16)?,
        remind_at: row.get(17)?,
    })
}

/// A new random UUID in the usual 8-4-4-4-12 form, upper case like Apple's.
const NEW_UUID: &str = "upper(substr(h, 1, 8) || '-' || substr(h, 9, 4) || '-4' || substr(h, 14, 3)
     || '-' || substr('89AB', 1 + (abs(random()) % 4), 1) || substr(h, 18, 3) || '-'
     || substr(h, 21, 12))";

impl Store {
    /// Every note not deleted for good: pinned first, then newest first.
    pub fn notes(&self) -> Result<Vec<Note>> {
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {COLUMNS} FROM note
             ORDER BY pinned DESC, position DESC, updated_at DESC, id DESC"
        ))?;
        let rows = stmt.query_map([], note_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Note `id`, if it exists.
    pub fn note(&self, id: i64) -> Result<Option<Note>> {
        Ok(self
            .pim
            .prepare_cached(&format!("SELECT {COLUMNS} FROM note WHERE id = ?1"))?
            .query_row([id], note_row)
            .optional()?)
    }

    /// The notes about the mail with `Message-ID` `link`, not in Trash.
    pub fn notes_about(&self, link: &str) -> Result<Vec<Note>> {
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {COLUMNS} FROM note WHERE link = ?1 AND trashed_at IS NULL
             ORDER BY updated_at DESC"
        ))?;
        let rows = stmt.query_map([link], note_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Saves what the user changed of `note`: a new one when its ID is 0
    /// (or no longer exists), on top of the others, else in place of the
    /// old one. Returns its ID. The note is marked for the server; its
    /// server copy and Trash state stay as they were.
    pub fn save_note(&mut self, note: &Note) -> Result<i64> {
        self.save_note_with(note, None)
    }

    /// [`Store::save_note`], its pictures put in place of the ones it had
    /// unless `pictures` is `None`.
    pub fn save_note_with(&mut self, note: &Note, pictures: Option<&[NotePicture]>) -> Result<i64> {
        self.check_writable()?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let now = unix_now();
        let labels = serde_json::to_string(&note.labels).unwrap_or_else(|_| "[]".to_owned());
        let old_account: Option<Option<i64>> = if note.id == 0 {
            None
        } else {
            tx.query_row(
                "SELECT account_id FROM note WHERE id = ?1",
                [note.id],
                |row| row.get(0),
            )
            .optional()?
        };
        let id = match old_account {
            Some(old_account) => {
                // Moved to another account: the old one's copy goes.
                if old_account != note.account_id {
                    forget_server_copy(&tx, note.id)?;
                }
                tx.execute(
                    "UPDATE note SET account_id = ?2, title = ?3, body = ?4, color = ?5,
                            pinned = ?6, archived = ?7, labels = ?8, link = ?9,
                            updated_at = ?10, dirty = 1, html = ?11
                     WHERE id = ?1",
                    params![
                        note.id,
                        note.account_id,
                        note.title,
                        note.body,
                        note.color,
                        note.pinned,
                        note.archived,
                        labels,
                        note.link,
                        now,
                        note.html
                    ],
                )?;
                journal::record(&tx, ObjectKind::Note, note.id, ChangeOp::Update)?;
                note.id
            }
            None => {
                tx.execute(
                    &format!(
                        "WITH r(h) AS (SELECT hex(randomblob(16)))
                         INSERT INTO note (account_id, uuid, title, body, color, pinned,
                                           archived, labels, link, position, created_at,
                                           updated_at, dirty, html)
                         SELECT ?1, {NEW_UUID}, ?2, ?3, ?4, ?5, ?6, ?7, ?8,
                                (SELECT coalesce(max(position), 0) + 1 FROM note), ?9, ?9, 1,
                                ?10
                         FROM r"
                    ),
                    params![
                        note.account_id,
                        note.title,
                        note.body,
                        note.color,
                        note.pinned,
                        note.archived,
                        labels,
                        note.link,
                        now,
                        note.html
                    ],
                )?;
                let id = tx.last_insert_rowid();
                journal::record(&tx, ObjectKind::Note, id, ChangeOp::Insert)?;
                id
            }
        };
        put_reminder(&tx, id, note.remind_at)?;
        if let Some(pictures) = pictures {
            put_pictures(&tx, id, pictures)?;
        }
        keep_version(&tx, id, now, &VersionSource::Here)?;
        tx.commit()?;
        Ok(id)
    }

    /// Note `id`'s pictures, in the order they were added.
    pub fn note_pictures(&self, id: i64) -> Result<Vec<NotePicture>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT cid, name, mime, width, height, data FROM note_picture
             WHERE note_id = ?1 ORDER BY ord",
        )?;
        let rows = stmt.query_map([id], picture_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The first picture of each note that has one, by note: what its card
    /// shows across the top.
    pub fn note_covers(&self) -> Result<std::collections::HashMap<i64, NotePicture>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT p.note_id, p.cid, p.name, p.mime, p.width, p.height, p.data
             FROM note_picture p
             WHERE p.ord = (SELECT min(ord) FROM note_picture q WHERE q.note_id = p.note_id)",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                NotePicture {
                    cid: row.get(1)?,
                    name: row.get(2)?,
                    mime: row.get(3)?,
                    width: row.get(4)?,
                    height: row.get(5)?,
                    data: row.get(6)?,
                },
            ))
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Note `id`'s versions, newest first; the first is how it is now.
    pub fn note_versions(&self, id: i64) -> Result<Vec<NoteVersion>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT id, at, title, body, html, source FROM note_version
             WHERE note_id = ?1 ORDER BY at DESC, id DESC",
        )?;
        let rows = stmt.query_map([id], |row| {
            Ok(NoteVersion {
                id: row.get(0)?,
                at: row.get(1)?,
                title: row.get(2)?,
                body: row.get(3)?,
                html: row.get(4)?,
                source: VersionSource::from_sql(&row.get::<_, String>(5)?),
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Forgets versions older than [`NOTE_VERSION_KEEP`] by `now`, but
    /// never a note's newest. Returns how many.
    pub fn purge_note_versions(&mut self, now: i64) -> Result<usize> {
        self.check_writable()?;
        Ok(self.pim.execute(
            "DELETE FROM note_version
             WHERE at < ?1
               AND id != (SELECT v.id FROM note_version v
                          WHERE v.note_id = note_version.note_id
                          ORDER BY v.at DESC, v.id DESC LIMIT 1)",
            [now - NOTE_VERSION_KEEP],
        )?)
    }

    /// The notes not in Trash that remind in `from < at <= to`, soonest
    /// first, as `(id, title, body, at)`.
    pub fn notes_reminding(&self, from: i64, to: i64) -> Result<Vec<(i64, String, String, i64)>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT n.id, n.title, n.body, r.at FROM note_reminder r JOIN note n ON n.id = r.note_id
             WHERE r.at > ?1 AND r.at <= ?2 AND n.trashed_at IS NULL
             ORDER BY r.at",
        )?;
        let rows = stmt.query_map([from, to], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// When the next note not in Trash reminds after `after`.
    pub fn next_note_reminder(&self, after: i64) -> Result<Option<i64>> {
        Ok(self.pim.query_row(
            "SELECT min(r.at) FROM note_reminder r JOIN note n ON n.id = r.note_id
             WHERE r.at > ?1 AND n.trashed_at IS NULL",
            [after],
            |row| row.get(0),
        )?)
    }

    /// The notes not in Trash whose HTML links to the note with `uuid`.
    pub fn notes_linking(&self, uuid: &str) -> Result<Vec<Note>> {
        if uuid.is_empty() {
            return Ok(Vec::new());
        }
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {COLUMNS} FROM note
             WHERE instr(html, ?1) > 0 AND trashed_at IS NULL AND uuid != ?2
             ORDER BY updated_at DESC"
        ))?;
        let rows = stmt.query_map(params![format!("{NOTE_LINK_SCHEME}{uuid}"), uuid], note_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Moves `ids` to Trash (`trashed` true) or back out of it. A note
    /// in Trash leaves its account's Notes folder, as deleting it on a
    /// phone does, and goes back there when restored. Returns how many
    /// changed.
    pub fn trash_notes(&mut self, ids: &[i64], trashed: bool) -> Result<usize> {
        self.check_writable()?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let now = unix_now();
        let mut changed = 0;
        for &id in ids {
            let n = if trashed {
                forget_server_copy(&tx, id)?;
                tx.execute(
                    "UPDATE note SET trashed_at = ?2, pinned = 0
                     WHERE id = ?1 AND trashed_at IS NULL",
                    params![id, now],
                )?
            } else {
                tx.execute(
                    "UPDATE note SET trashed_at = NULL, dirty = 1, updated_at = ?2,
                            position = (SELECT coalesce(max(position), 0) + 1 FROM note)
                     WHERE id = ?1 AND trashed_at IS NOT NULL",
                    params![id, now],
                )?
            };
            if n > 0 {
                journal::record(&tx, ObjectKind::Note, id, ChangeOp::Update)?;
                changed += 1;
            }
        }
        tx.commit()?;
        Ok(changed)
    }

    /// Puts notes `ids` in this order, the first on top, in the places
    /// they had among themselves, so the notes between them stay where
    /// they are: dragging a card on the board. The order stays on this
    /// computer. Returns how many moved.
    pub fn order_notes(&mut self, ids: &[i64]) -> Result<usize> {
        self.check_writable()?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut found = Vec::with_capacity(ids.len());
        let mut places = Vec::with_capacity(ids.len());
        for &id in ids {
            let place: Option<i64> = tx
                .query_row("SELECT position FROM note WHERE id = ?1", [id], |r| {
                    r.get(0)
                })
                .optional()?;
            if let Some(place) = place {
                found.push(id);
                places.push(place);
            }
        }
        places.sort_unstable_by(|a, b| b.cmp(a));
        // Two notes in one place would tie; the lower goes one down.
        for ix in 1..places.len() {
            places[ix] = places[ix].min(places[ix - 1] - 1);
        }
        let mut moved = 0;
        for (id, place) in found.into_iter().zip(places) {
            let n = tx.execute(
                "UPDATE note SET position = ?2 WHERE id = ?1 AND position != ?2",
                params![id, place],
            )?;
            if n > 0 {
                journal::record(&tx, ObjectKind::Note, id, ChangeOp::Update)?;
                moved += 1;
            }
        }
        tx.commit()?;
        Ok(moved)
    }

    /// Deletes `ids` for good, here and in their Notes folders. Returns
    /// how many existed.
    /// Takes label `old` off notes `ids` and puts `new` on them (an empty
    /// one is neither taken nor put): renaming, deleting and adding a label
    /// in one. Returns how many notes changed.
    pub fn relabel_notes(&mut self, ids: &[i64], old: &str, new: &str) -> Result<usize> {
        self.check_writable()?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let now = unix_now();
        let mut changed = 0;
        for &id in ids {
            let Some(labels) = tx
                .query_row("SELECT labels FROM note WHERE id = ?1", [id], |row| {
                    row.get::<_, String>(0)
                })
                .optional()?
            else {
                continue;
            };
            let labels: Vec<String> = serde_json::from_str(&labels).unwrap_or_default();
            let relabeled = relabel(&labels, old, new);
            if relabeled == labels {
                continue;
            }
            let json = serde_json::to_string(&relabeled).unwrap_or_else(|_| "[]".to_owned());
            tx.execute(
                "UPDATE note SET labels = ?2, updated_at = ?3, dirty = 1 WHERE id = ?1",
                params![id, json, now],
            )?;
            journal::record(&tx, ObjectKind::Note, id, ChangeOp::Update)?;
            changed += 1;
        }
        tx.commit()?;
        Ok(changed)
    }

    pub fn delete_notes(&mut self, ids: &[i64]) -> Result<usize> {
        self.check_writable()?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut deleted = 0;
        for &id in ids {
            forget_server_copy(&tx, id)?;
            if tx.execute("DELETE FROM note WHERE id = ?1", [id])? > 0 {
                journal::record(&tx, ObjectKind::Note, id, ChangeOp::Delete)?;
                deleted += 1;
            }
        }
        tx.commit()?;
        Ok(deleted)
    }

    /// Deletes the notes that have been in Trash for [`NOTE_TRASH_KEEP`]
    /// by `now`. Returns how many.
    pub fn purge_note_trash(&mut self, now: i64) -> Result<usize> {
        let ids: Vec<i64> = {
            let mut stmt = self
                .pim
                .prepare_cached("SELECT id FROM note WHERE trashed_at <= ?1")?;
            let rows = stmt.query_map([now - NOTE_TRASH_KEEP], |row| row.get(0))?;
            rows.collect::<rusqlite::Result<_>>()?
        };
        if ids.is_empty() {
            return Ok(0);
        }
        self.delete_notes(&ids)
    }
}

/// A note as its account's Notes folder has it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RemoteNote {
    pub uuid: String,
    pub title: String,
    pub body: String,
    /// As [`Note::html`].
    pub html: String,
    pub color: i64,
    pub pinned: bool,
    pub archived: bool,
    pub labels: Vec<String>,
    pub link: Option<String>,
    /// Unix seconds, from the message's `Date`.
    pub updated_at: i64,
    pub remind_at: Option<i64>,
    pub pictures: Vec<NotePicture>,
    /// The device it was written on, when its mail app says.
    pub device: Option<String>,
}

impl Store {
    /// Every note of `account`, Trash too.
    pub fn account_notes(&self, account: i64) -> Result<Vec<Note>> {
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {COLUMNS} FROM note WHERE account_id = ?1 ORDER BY id"
        ))?;
        let rows = stmt.query_map([account], note_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The UIDs in `account`'s Notes folder of notes deleted here.
    pub fn notes_gone(&self, account: i64) -> Result<Vec<i64>> {
        let mut stmt = self
            .pim
            .prepare_cached("SELECT server_uid FROM note_gone WHERE account_id = ?1")?;
        let rows = stmt.query_map([account], |row| row.get(0))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Forgets `uids` of [`Store::notes_gone`]: deleted on the server.
    pub fn clear_notes_gone(&mut self, account: i64, uids: &[i64]) -> Result<()> {
        self.check_writable()?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        for uid in uids {
            tx.execute(
                "DELETE FROM note_gone WHERE account_id = ?1 AND server_uid = ?2",
                params![account, uid],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Note `id` was written to its Notes folder as it was at `updated_at`
    /// (its old copy deleted): clean unless it changed since. Its new UID
    /// comes with the next look at the folder.
    pub fn note_uploaded(&mut self, id: i64, updated_at: i64) -> Result<()> {
        self.check_writable()?;
        self.pim.execute(
            "UPDATE note SET server_uid = NULL,
                    dirty = CASE WHEN updated_at = ?2 THEN 0 ELSE dirty END
             WHERE id = ?1",
            params![id, updated_at],
        )?;
        Ok(())
    }

    /// `remote`, found at `uid` in `account`'s Notes folder: a new note,
    /// or news of one kept here. A note changed here keeps its changes (it
    /// goes up again in place of that copy).
    pub fn apply_remote_note(&mut self, account: i64, uid: i64, remote: &RemoteNote) -> Result<()> {
        self.check_writable()?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let labels = serde_json::to_string(&remote.labels).unwrap_or_else(|_| "[]".to_owned());
        let found: Option<(i64, bool, Option<i64>)> = tx
            .query_row(
                "SELECT id, dirty, server_uid FROM note WHERE uuid = ?1",
                [&remote.uuid],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .optional()?;
        match found {
            Some((id, dirty, old_uid)) => {
                if dirty {
                    // Ours wins; this copy goes when ours goes up.
                    if let Some(old) = old_uid.filter(|old| *old != uid) {
                        tx.execute(
                            "INSERT OR IGNORE INTO note_gone (account_id, server_uid)
                             VALUES (?1, ?2)",
                            params![account, old],
                        )?;
                    }
                    tx.execute(
                        "UPDATE note SET server_uid = ?2 WHERE id = ?1",
                        params![id, uid],
                    )?;
                } else {
                    tx.execute(
                        "UPDATE note SET account_id = ?2, server_uid = ?3, title = ?4,
                                body = ?5, color = ?6, pinned = ?7, archived = ?8,
                                labels = ?9, link = ?10,
                                updated_at = max(updated_at, ?11), html = ?12
                         WHERE id = ?1",
                        params![
                            id,
                            account,
                            uid,
                            remote.title,
                            remote.body,
                            remote.color,
                            remote.pinned,
                            remote.archived,
                            labels,
                            remote.link,
                            remote.updated_at,
                            remote.html
                        ],
                    )?;
                    put_reminder(&tx, id, remote.remind_at)?;
                    put_pictures(&tx, id, &remote.pictures)?;
                    keep_version(
                        &tx,
                        id,
                        remote.updated_at,
                        &VersionSource::Elsewhere(remote.device.clone()),
                    )?;
                    journal::record(&tx, ObjectKind::Note, id, ChangeOp::Update)?;
                }
            }
            None => {
                tx.execute(
                    "INSERT INTO note (account_id, uuid, title, body, color, pinned, archived,
                                       labels, link, position, created_at, updated_at,
                                       server_uid, dirty, html)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9,
                             (SELECT coalesce(max(position), 0) + 1 FROM note), ?10, ?10,
                             ?11, 0, ?12)",
                    params![
                        account,
                        remote.uuid,
                        remote.title,
                        remote.body,
                        remote.color,
                        remote.pinned,
                        remote.archived,
                        labels,
                        remote.link,
                        remote.updated_at,
                        uid,
                        remote.html
                    ],
                )?;
                let id = tx.last_insert_rowid();
                put_reminder(&tx, id, remote.remind_at)?;
                put_pictures(&tx, id, &remote.pictures)?;
                keep_version(
                    &tx,
                    id,
                    remote.updated_at,
                    &VersionSource::Elsewhere(remote.device.clone()),
                )?;
                journal::record(&tx, ObjectKind::Note, id, ChangeOp::Insert)?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    /// `account`'s Notes folder holds only `uids` of the copies known
    /// here: the notes whose copy is gone were deleted elsewhere, unless
    /// changed here (then they go up again). Returns how many went.
    pub fn forget_remote_notes(&mut self, account: i64, uids: &[i64]) -> Result<usize> {
        self.check_writable()?;
        let present: std::collections::HashSet<i64> = uids.iter().copied().collect();
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let known: Vec<(i64, i64, bool)> = {
            let mut stmt = tx.prepare(
                "SELECT id, server_uid, dirty FROM note
                 WHERE account_id = ?1 AND server_uid IS NOT NULL",
            )?;
            let rows =
                stmt.query_map([account], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;
            rows.collect::<rusqlite::Result<_>>()?
        };
        let mut gone = 0;
        for (id, uid, dirty) in known {
            if present.contains(&uid) {
                continue;
            }
            if dirty {
                tx.execute("UPDATE note SET server_uid = NULL WHERE id = ?1", [id])?;
            } else {
                tx.execute("DELETE FROM note WHERE id = ?1", [id])?;
                journal::record(&tx, ObjectKind::Note, id, ChangeOp::Delete)?;
                gone += 1;
            }
        }
        tx.commit()?;
        Ok(gone)
    }
}

fn picture_row(row: &Row<'_>) -> rusqlite::Result<NotePicture> {
    Ok(NotePicture {
        cid: row.get(0)?,
        name: row.get(1)?,
        mime: row.get(2)?,
        width: row.get(3)?,
        height: row.get(4)?,
        data: row.get(5)?,
    })
}

/// Sets when note `id` reminds, or that it doesn't.
fn put_reminder(tx: &rusqlite::Transaction<'_>, id: i64, at: Option<i64>) -> Result<()> {
    match at {
        Some(at) => tx.execute(
            "INSERT INTO note_reminder (note_id, at) VALUES (?1, ?2)
             ON CONFLICT (note_id) DO UPDATE SET at = excluded.at",
            params![id, at],
        )?,
        None => tx.execute("DELETE FROM note_reminder WHERE note_id = ?1", [id])?,
    };
    Ok(())
}

/// Puts `pictures` in place of note `id`'s.
fn put_pictures(tx: &rusqlite::Transaction<'_>, id: i64, pictures: &[NotePicture]) -> Result<()> {
    tx.execute("DELETE FROM note_picture WHERE note_id = ?1", [id])?;
    for (ord, p) in pictures.iter().enumerate() {
        tx.execute(
            "INSERT OR REPLACE INTO note_picture
                 (note_id, cid, ord, name, mime, width, height, data)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                id,
                p.cid,
                i64::try_from(ord).unwrap_or(i64::MAX),
                p.name,
                p.mime,
                p.width,
                p.height,
                p.data
            ],
        )?;
    }
    Ok(())
}

/// Keeps note `id` as it now is among its versions, written at `at` from
/// `source`: in place of the newest when that was written here moments
/// ago too, so typing doesn't make a version a second.
fn keep_version(
    tx: &rusqlite::Transaction<'_>,
    id: i64,
    at: i64,
    source: &VersionSource,
) -> Result<()> {
    let (title, body, html): (String, String, String) = tx.query_row(
        "SELECT title, body, html FROM note WHERE id = ?1",
        [id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;
    let newest: Option<(i64, i64, String, String, String, String)> = tx
        .query_row(
            "SELECT id, at, source, title, body, html FROM note_version
             WHERE note_id = ?1 ORDER BY id DESC LIMIT 1",
            [id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            },
        )
        .optional()?;
    let source_sql = source.to_sql();
    if let Some((vid, vat, vsource, vtitle, vbody, vhtml)) = newest {
        if (vtitle.as_str(), vbody.as_str(), vhtml.as_str())
            == (title.as_str(), body.as_str(), html.as_str())
        {
            return Ok(());
        }
        if vsource == source_sql && *source == VersionSource::Here && at - vat < VERSION_GAP {
            tx.execute(
                "UPDATE note_version SET at = ?2, title = ?3, body = ?4, html = ?5
                 WHERE id = ?1",
                params![vid, at.max(vat), title, body, html],
            )?;
            return Ok(());
        }
    }
    tx.execute(
        "INSERT INTO note_version (note_id, at, title, body, html, source)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, at, title, body, html, source_sql],
    )?;
    Ok(())
}

/// Queues note `id`'s server copy, if any, for deletion and forgets it.
fn forget_server_copy(tx: &rusqlite::Transaction<'_>, id: i64) -> Result<()> {
    tx.execute(
        "INSERT OR IGNORE INTO note_gone (account_id, server_uid)
         SELECT account_id, server_uid FROM note
         WHERE id = ?1 AND account_id IS NOT NULL AND server_uid IS NOT NULL",
        [id],
    )?;
    tx.execute(
        "UPDATE note SET server_uid = NULL, dirty = 1 WHERE id = ?1",
        [id],
    )?;
    Ok(())
}

/// `labels` with `old` taken off and `new` put on once, in `old`'s place;
/// with no `old`, `new` goes at the end. Labels without `old` stay as
/// they are.
fn relabel(labels: &[String], old: &str, new: &str) -> Vec<String> {
    if !old.is_empty() && !labels.iter().any(|l| l == old) {
        return labels.to_vec();
    }
    let mut out: Vec<String> = Vec::with_capacity(labels.len() + 1);
    let mut placed = new.is_empty();
    for label in labels {
        if !old.is_empty() && label == old {
            if !placed {
                out.push(new.to_owned());
                placed = true;
            }
        } else if label != new || !placed {
            if label == new {
                placed = true;
            }
            out.push(label.clone());
        }
    }
    if !placed {
        out.push(new.to_owned());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Mode;
    use katna_core::Paths;

    fn store() -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(dir.path());
        let store = Store::open(&paths, Mode::ReadWrite).unwrap();
        (dir, store)
    }

    fn note(title: &str) -> Note {
        Note {
            title: title.to_owned(),
            body: "☐ milk\n☑ eggs".to_owned(),
            labels: vec!["Home".to_owned()],
            ..Note::default()
        }
    }

    #[test]
    fn relabel_renames_deletes_and_adds_once() {
        let labels = |l: &[&str]| l.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
        assert_eq!(relabel(&labels(&["a", "c"]), "a", "b"), ["b", "c"]);
        assert_eq!(relabel(&labels(&["a", "b"]), "a", "b"), ["b"]);
        assert_eq!(relabel(&labels(&["b", "a"]), "a", "b"), ["b"]);
        assert_eq!(relabel(&labels(&["a", "c"]), "a", ""), ["c"]);
        assert_eq!(relabel(&labels(&["c"]), "", "b"), ["c", "b"]);
        assert_eq!(relabel(&labels(&["b"]), "", "b"), ["b"]);
        assert_eq!(relabel(&labels(&["c"]), "a", "b"), ["c"]);
    }

    #[test]
    fn relabeling_notes_marks_only_the_changed_ones() {
        let (_dir, mut store) = store();
        let a = store.save_note(&note("a")).unwrap();
        let b = store
            .save_note(&Note {
                labels: vec!["Work".to_owned()],
                ..note("b")
            })
            .unwrap();
        let journal = journal::changes_since(&store.pim, 0, 1000).unwrap().len();
        assert_eq!(store.relabel_notes(&[a, b], "Home", "Family").unwrap(), 1);
        assert_eq!(store.note(a).unwrap().unwrap().labels, ["Family"]);
        assert_eq!(store.note(b).unwrap().unwrap().labels, ["Work"]);
        assert_eq!(
            journal::changes_since(&store.pim, 0, 1000).unwrap().len(),
            journal + 1
        );
        assert_eq!(store.relabel_notes(&[a, b], "", "Home").unwrap(), 2);
        assert_eq!(store.note(b).unwrap().unwrap().labels, ["Work", "Home"]);
        assert_eq!(store.relabel_notes(&[a, b, 999], "Home", "").unwrap(), 2);
        assert_eq!(store.note(a).unwrap().unwrap().labels, ["Family"]);
    }

    #[test]
    fn a_new_note_gets_a_uuid_and_goes_on_top() {
        let (_dir, mut store) = store();
        let a = store.save_note(&note("a")).unwrap();
        let b = store.save_note(&note("b")).unwrap();
        let notes = store.notes().unwrap();
        assert_eq!(
            notes.iter().map(|n| n.id).collect::<Vec<_>>(),
            [b, a],
            "the newest first"
        );
        let a = store.note(a).unwrap().unwrap();
        assert_eq!(a.uuid.len(), 36);
        assert_eq!(a.uuid, a.uuid.to_uppercase());
        assert_eq!(&a.uuid[14..15], "4");
        assert_ne!(a.uuid, notes[0].uuid);
        assert_eq!(a.labels, ["Home"]);
        assert!(a.dirty);
    }

    #[test]
    fn saving_keeps_the_uuid_and_pinned_notes_come_first() {
        let (_dir, mut store) = store();
        let a = store.save_note(&note("a")).unwrap();
        store.save_note(&note("b")).unwrap();
        let mut saved = store.note(a).unwrap().unwrap();
        let uuid = saved.uuid.clone();
        saved.pinned = true;
        saved.body = "changed".to_owned();
        assert_eq!(store.save_note(&saved).unwrap(), a);
        let notes = store.notes().unwrap();
        assert_eq!(notes[0].id, a);
        assert_eq!(notes[0].uuid, uuid);
        assert_eq!(notes[0].body, "changed");
    }

    #[test]
    fn order_notes_moves_notes_among_their_own_places() {
        let (_dir, mut store) = store();
        let ids: Vec<i64> = ["a", "b", "c", "d"]
            .iter()
            .map(|t| store.save_note(&note(t)).unwrap())
            .collect();
        let titles = |store: &Store| -> Vec<String> {
            store
                .notes()
                .unwrap()
                .into_iter()
                .map(|n| n.title)
                .collect()
        };
        assert_eq!(titles(&store), ["d", "c", "b", "a"]);
        // "b" goes above "c"; "d" and "a" stay where they are.
        assert_eq!(store.order_notes(&[ids[1], ids[2]]).unwrap(), 2);
        assert_eq!(titles(&store), ["d", "b", "c", "a"]);
        assert_eq!(store.order_notes(&[ids[1], ids[2]]).unwrap(), 0);
        assert_eq!(store.order_notes(&[ids[2], ids[1]]).unwrap(), 2);
        assert_eq!(titles(&store), ["d", "c", "b", "a"]);
    }

    #[test]
    fn trash_takes_the_server_copy_and_restore_brings_it_back() {
        let (_dir, mut store) = store();
        let mut n = note("a");
        n.account_id = Some(3);
        let id = store.save_note(&n).unwrap();
        store
            .pim
            .execute(
                "UPDATE note SET server_uid = 9, dirty = 0 WHERE id = ?1",
                [id],
            )
            .unwrap();
        assert_eq!(store.trash_notes(&[id], true).unwrap(), 1);
        let gone: i64 = store
            .pim
            .query_row(
                "SELECT server_uid FROM note_gone WHERE account_id = 3",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(gone, 9);
        let trashed = store.note(id).unwrap().unwrap();
        assert!(trashed.trashed_at.is_some());
        assert_eq!(trashed.server_uid, None);
        assert_eq!(store.trash_notes(&[id], false).unwrap(), 1);
        let back = store.note(id).unwrap().unwrap();
        assert_eq!(back.trashed_at, None);
        assert!(back.dirty);
    }

    #[test]
    fn trash_empties_itself_after_seven_days() {
        let (_dir, mut store) = store();
        let id = store.save_note(&note("a")).unwrap();
        let keep = store.save_note(&note("b")).unwrap();
        store.trash_notes(&[id], true).unwrap();
        let now = unix_now();
        assert_eq!(store.purge_note_trash(now).unwrap(), 0);
        assert_eq!(store.purge_note_trash(now + NOTE_TRASH_KEEP).unwrap(), 1);
        assert_eq!(
            store
                .notes()
                .unwrap()
                .iter()
                .map(|n| n.id)
                .collect::<Vec<_>>(),
            [keep]
        );
    }

    fn remote(uuid: &str, title: &str) -> RemoteNote {
        RemoteNote {
            uuid: uuid.to_owned(),
            title: title.to_owned(),
            body: "text".to_owned(),
            updated_at: 1_000,
            ..RemoteNote::default()
        }
    }

    #[test]
    fn a_note_from_the_server_comes_in_and_goes_when_deleted_there() {
        let (_dir, mut store) = store();
        store
            .apply_remote_note(2, 7, &remote("U1", "phone note"))
            .unwrap();
        let notes = store.account_notes(2).unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].server_uid, Some(7));
        assert!(!notes[0].dirty);
        // Edited on the phone: a new copy in place of the old.
        store
            .apply_remote_note(2, 8, &remote("U1", "edited"))
            .unwrap();
        let notes = store.account_notes(2).unwrap();
        assert_eq!((notes.len(), notes[0].title.as_str()), (1, "edited"));
        assert_eq!(store.forget_remote_notes(2, &[8]).unwrap(), 0);
        assert_eq!(store.forget_remote_notes(2, &[]).unwrap(), 1);
        assert!(store.account_notes(2).unwrap().is_empty());
    }

    #[test]
    fn a_note_changed_here_keeps_its_changes() {
        let (_dir, mut store) = store();
        store
            .apply_remote_note(2, 7, &remote("U1", "phone note"))
            .unwrap();
        let mut note = store.account_notes(2).unwrap().remove(0);
        note.title = "mine".to_owned();
        store.save_note(&note).unwrap();
        store
            .apply_remote_note(2, 9, &remote("U1", "theirs"))
            .unwrap();
        let note = store.note(note.id).unwrap().unwrap();
        assert_eq!(note.title, "mine");
        assert_eq!(note.server_uid, Some(9));
        assert!(note.dirty);
        // The copy it had goes when it goes up again.
        assert_eq!(store.notes_gone(2).unwrap(), [7]);
        store.note_uploaded(note.id, note.updated_at).unwrap();
        let note = store.note(note.id).unwrap().unwrap();
        assert!(!note.dirty);
        assert_eq!(note.server_uid, None);
    }

    #[test]
    fn notes_about_a_mail_are_found_by_its_message_id() {
        let (_dir, mut store) = store();
        let mut n = note("call Ravi");
        n.link = Some("abc@example.com".to_owned());
        let id = store.save_note(&n).unwrap();
        store.save_note(&note("other")).unwrap();
        let about = store.notes_about("abc@example.com").unwrap();
        assert_eq!(about.iter().map(|n| n.id).collect::<Vec<_>>(), [id]);
    }

    #[test]
    fn versions_keep_earlier_text_and_typing_makes_one() {
        let (_dir, mut store) = store();
        let id = store.save_note(&note("a")).unwrap();
        let mut n = store.note(id).unwrap().unwrap();
        n.body = "one".to_owned();
        store.save_note(&n).unwrap();
        n.body = "two".to_owned();
        store.save_note(&n).unwrap();
        // Typing moments apart is one version.
        let versions = store.note_versions(id).unwrap();
        assert_eq!(versions.len(), 1);
        assert_eq!(versions[0].body, "two");
        // An hour later is another.
        store
            .pim
            .execute("UPDATE note_version SET at = at - 3600", [])
            .unwrap();
        n.body = "three".to_owned();
        store.save_note(&n).unwrap();
        let versions = store.note_versions(id).unwrap();
        assert_eq!(
            versions.iter().map(|v| v.body.as_str()).collect::<Vec<_>>(),
            ["three", "two"]
        );
        // Old ones go, but never the newest.
        let now = unix_now();
        assert_eq!(store.purge_note_versions(now + 3600).unwrap(), 0);
        assert_eq!(
            store
                .purge_note_versions(now + NOTE_VERSION_KEEP + 60)
                .unwrap(),
            1
        );
        assert_eq!(store.note_versions(id).unwrap().len(), 1);
    }

    #[test]
    fn a_change_from_the_server_is_a_version_of_its_own() {
        let (_dir, mut store) = store();
        let mut r = remote("U1", "phone note");
        r.device = Some("iPhone".to_owned());
        store.apply_remote_note(2, 7, &r).unwrap();
        r.body = "edited".to_owned();
        r.updated_at += 10;
        store.apply_remote_note(2, 8, &r).unwrap();
        let id = store.account_notes(2).unwrap()[0].id;
        // Typed here since, then the same server copy read again: it is
        // older than the typing but no new version.
        let mut here = store.note(id).unwrap().unwrap();
        here.body = "typed".to_owned();
        store.save_note(&here).unwrap();
        r.body = "typed".to_owned();
        store.apply_remote_note(2, 9, &r).unwrap();
        store.apply_remote_note(2, 9, &r).unwrap();
        let versions = store.note_versions(id).unwrap();
        assert_eq!(versions.len(), 3);
        assert_eq!(versions[0].source, VersionSource::Here);
        assert_eq!(
            versions[1].source,
            VersionSource::Elsewhere(Some("iPhone".to_owned()))
        );
    }

    #[test]
    fn pictures_reminders_and_links_are_kept() {
        let (_dir, mut store) = store();
        let picture = NotePicture {
            cid: "p1".to_owned(),
            name: "shelf.png".to_owned(),
            mime: "image/png".to_owned(),
            width: 4,
            height: 3,
            data: vec![1, 2, 3],
        };
        let mut n = note("a");
        n.remind_at = Some(5_000);
        let a = store
            .save_note_with(&n, Some(std::slice::from_ref(&picture)))
            .unwrap();
        assert_eq!(
            store.note_pictures(a).unwrap(),
            std::slice::from_ref(&picture)
        );
        assert_eq!(store.note_covers().unwrap()[&a], picture);
        // Saved without pictures given: they stay.
        store.save_note(&store.note(a).unwrap().unwrap()).unwrap();
        assert_eq!(store.note_pictures(a).unwrap().len(), 1);
        let due = store.notes_reminding(4_000, 5_000).unwrap();
        assert_eq!(due.iter().map(|d| d.0).collect::<Vec<_>>(), [a]);
        assert!(store.notes_reminding(5_000, 6_000).unwrap().is_empty());
        let uuid = store.note(a).unwrap().unwrap().uuid;
        let mut b = note("b");
        b.html = format!("<p><a href=\"{NOTE_LINK_SCHEME}{uuid}\">a</a></p>");
        let b = store.save_note(&b).unwrap();
        assert_eq!(
            store
                .notes_linking(&uuid)
                .unwrap()
                .iter()
                .map(|n| n.id)
                .collect::<Vec<_>>(),
            [b]
        );
    }
}
