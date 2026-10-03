// SPDX-License-Identifier: GPL-3.0-or-later

//! Task lists and tasks (`task_list` and `task` in `pim.db`): each
//! account's own lists, synced with its task service (Google Tasks,
//! Microsoft To Do), and a list kept on this computer
//! (`docs/ARCHITECTURE.md` §18.1).
//!
//! A change made here marks the row dirty; the daemon's task sync sends
//! it to the service and clears the mark. A deleted row stays as a
//! tombstone until the service has deleted it too. What the service
//! can't keep (Google Tasks: a due time, reminders, repeat, the star)
//! is kept here only.

use std::collections::HashMap;

use katna_core::{AccountId, OAuthProvider};
use rusqlite::{OptionalExtension, Row, Transaction, TransactionBehavior, params};

use crate::Store;
use crate::db::unix_now;
use crate::error::Result;

mod files;
pub use files::{PendingFile, RemoteFile, SyncedFiles, TaskFile};

/// One task.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Task {
    pub id: i64,
    /// Its [`TaskList`].
    pub list: i64,
    /// The task it is a step (subtask) of.
    pub parent: Option<i64>,
    pub title: String,
    pub notes: String,
    /// `YYYY-MM-DD`, or empty.
    pub due: String,
    /// A time on the due day: minutes after local midnight.
    pub due_time: Option<u32>,
    /// When to remind (Unix seconds).
    pub remind_at: Option<i64>,
    /// An RFC 5545 `RRULE` value (`FREQ=WEEKLY;BYDAY=MO`), or empty.
    pub repeat: String,
    pub starred: bool,
    /// When it was ticked off (Unix seconds); `None` while open.
    pub done_at: Option<i64>,
    /// The service's order within the list: compared as text.
    pub position: String,
    /// The Message-ID (no angle brackets) of the mail it was made from, or
    /// `note:<id>` for a task made from a note's checklist line.
    pub mail: String,
    /// Its labels: the same names as the notes' labels.
    pub labels: Vec<String>,
}

/// What a person sets on a task; [`Store::edit_task`] writes all of it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TaskFields {
    pub title: String,
    pub notes: String,
    pub due: String,
    pub due_time: Option<u32>,
    pub remind_at: Option<i64>,
    pub repeat: String,
    pub starred: bool,
    pub mail: String,
    pub labels: Vec<String>,
}

impl TaskFields {
    /// Everything `task` has that a person sets, to change some of it.
    pub fn of(task: &Task) -> Self {
        Self {
            title: task.title.clone(),
            notes: task.notes.clone(),
            due: task.due.clone(),
            due_time: task.due_time,
            remind_at: task.remind_at,
            repeat: task.repeat.clone(),
            starred: task.starred,
            mail: task.mail.clone(),
            labels: task.labels.clone(),
        }
    }
}

/// A task list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskList {
    pub id: i64,
    /// `None`: kept on this computer.
    pub account: Option<AccountId>,
    pub title: String,
    /// The account's own default list (Google's and To Do's "Tasks").
    pub is_default: bool,
}

/// A list as the service has it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteTaskList {
    pub remote_id: String,
    pub title: String,
    pub is_default: bool,
}

/// What the service keeps beyond Google Tasks' fields; `None` in
/// [`RemoteTask::extras`] leaves Katna's own values alone. The time on the
/// due day is Katna's own everywhere: To Do keeps only the day too.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TaskExtras {
    pub remind_at: Option<i64>,
    pub repeat: String,
}

/// A task as the service has it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RemoteTask {
    pub remote_id: String,
    /// The service's ID of its parent task.
    pub parent: Option<String>,
    /// Deleted on the service.
    pub deleted: bool,
    pub title: String,
    pub notes: String,
    /// `YYYY-MM-DD`, or empty.
    pub due: String,
    pub done_at: Option<i64>,
    pub position: String,
    pub etag: String,
    pub extras: Option<TaskExtras>,
    /// The star, where the service keeps one (To Do's high importance,
    /// CalDAV's priority 1 to 4, Zoho's high priority); `None` leaves
    /// Katna's own.
    pub starred: Option<bool>,
    /// Its labels, where the service keeps them (To Do's categories,
    /// CalDAV's `CATEGORIES`); `None` leaves Katna's own.
    pub labels: Option<Vec<String>>,
    /// Its files, where the service keeps them (To Do's attachments,
    /// CalDAV's inline `ATTACH`); `None` when it keeps none or didn't
    /// say ([`Store::sync_task_files`]).
    pub files: Option<Vec<RemoteFile>>,
}

/// A list change waiting to go to the service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingTaskList {
    pub id: i64,
    pub remote_id: Option<String>,
    pub title: String,
    pub deleted: bool,
}

/// A task change waiting to go to the service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingTask {
    pub task: Task,
    pub remote_id: Option<String>,
    /// The service's ID of its parent, once the parent is there.
    pub parent_remote: Option<String>,
    pub etag: Option<String>,
    pub deleted: bool,
    /// Its fields changed here (or it is new), besides its place.
    pub edited: bool,
    /// Where it goes on the service, when it was dragged to a new place
    /// in its list here (Google Tasks only).
    pub place: Option<Place>,
    /// `updated_at` when read: [`Store::task_pushed`] keeps the row dirty
    /// if it changed again meanwhile.
    pub stamp: i64,
}

/// Where a task dragged here goes among its list's tasks on the service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Place {
    First,
    /// Right after the task with this service ID.
    After(String),
    /// After a task not on the service yet: next round.
    Waiting,
}

/// What `task.dirty` holds: bits of what is waiting to go to the service.
/// A change of its fields (or a new task, or a deletion)…
const EDITED: i64 = 1;
/// …and a new place in its list ([`Store::place_task`]).
const MOVED: i64 = 2;

const TASK_COLUMNS: &str = "id, list_id, parent_id, title, notes, due, due_time, remind_at, \
                            repeat, starred, done_at, position, mail, labels";

fn task_row(row: &Row<'_>) -> rusqlite::Result<Task> {
    Ok(Task {
        id: row.get(0)?,
        list: row.get(1)?,
        parent: row.get(2)?,
        title: row.get(3)?,
        notes: row.get(4)?,
        due: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
        due_time: row.get(6)?,
        remind_at: row.get(7)?,
        repeat: row.get(8)?,
        starred: row.get(9)?,
        done_at: row.get(10)?,
        position: row.get(11)?,
        mail: row.get(12)?,
        labels: labels_of(&row.get::<_, String>(13)?),
    })
}

/// Labels kept as a JSON array; none when unreadable.
fn labels_of(json: &str) -> Vec<String> {
    serde_json::from_str(json).unwrap_or_default()
}

/// Labels as kept: a JSON array, each once, in the order given.
fn labels_json(labels: &[String]) -> String {
    let mut seen: Vec<&String> = Vec::with_capacity(labels.len());
    for label in labels {
        if !label.trim().is_empty() && !seen.contains(&label) {
            seen.push(label);
        }
    }
    serde_json::to_string(&seen).unwrap_or_else(|_| "[]".to_owned())
}

fn list_row(row: &Row<'_>) -> rusqlite::Result<TaskList> {
    Ok(TaskList {
        id: row.get(0)?,
        account: row.get::<_, Option<i64>>(1)?.map(AccountId),
        title: row.get(2)?,
        is_default: row.get(3)?,
    })
}

/// Whether list `list` belongs to an account, so changes go to a service.
fn synced(tx: &Transaction<'_>, list: i64) -> rusqlite::Result<bool> {
    Ok(tx
        .query_row(
            "SELECT account_id IS NOT NULL FROM task_list WHERE id = ?1",
            [list],
            |row| row.get(0),
        )
        .optional()?
        .unwrap_or(false))
}

fn due_or_null(due: &str) -> Option<&str> {
    (!due.is_empty()).then_some(due)
}

/// Task `id`, unless deleted.
fn task_in(tx: &Transaction<'_>, id: i64) -> rusqlite::Result<Option<Task>> {
    tx.prepare_cached(&format!(
        "SELECT {TASK_COLUMNS} FROM task WHERE id = ?1 AND deleted = 0"
    ))?
    .query_row([id], task_row)
    .optional()
}

/// The tasks (not steps) of list `list`, done ones too, in the order
/// [`Store::tasks_in`] shows them: each with its position.
fn top_level(conn: &rusqlite::Connection, list: i64) -> rusqlite::Result<Vec<(i64, String)>> {
    conn.prepare_cached(
        "SELECT id, position FROM task
         WHERE list_id = ?1 AND parent_id IS NULL AND deleted = 0
         ORDER BY position = '' DESC, position, created_at DESC, id DESC",
    )?
    .query_map([list], |row| Ok((row.get(0)?, row.get(1)?)))?
    .collect()
}

/// Moves task `id`, with its steps, to list `list`, on top: what was on
/// the old list's service goes as a tombstone; the task itself is sent to
/// the new list's service.
fn move_to_list(tx: &Transaction<'_>, id: i64, list: i64) -> rusqlite::Result<()> {
    let now = unix_now();
    let dirty = synced(tx, list)?;
    let mut ids = vec![id];
    ids.extend(
        tx.prepare_cached("SELECT id FROM task WHERE parent_id = ?1 AND deleted = 0")?
            .query_map([id], |row| row.get::<_, i64>(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?,
    );
    for each in ids {
        let (old_list, remote, etag): (i64, Option<String>, Option<String>) = tx.query_row(
            "SELECT list_id, remote_id, etag FROM task WHERE id = ?1",
            [each],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;
        tx.execute(
            "UPDATE task SET list_id = ?2, remote_id = NULL, etag = NULL, position = '',
                             dirty = ?3, updated_at = ?4
             WHERE id = ?1",
            params![each, list, dirty, now],
        )?;
        // Its files go to the new list's service, or stay here.
        files::moved(tx, each)?;
        if remote.is_some() {
            tx.execute(
                "INSERT INTO task (list_id, title, remote_id, etag, deleted, dirty,
                                   created_at, updated_at)
                 VALUES (?1, '', ?2, ?3, 1, 1, ?4, ?4)",
                params![old_list, remote, etag, now],
            )?;
        }
    }
    Ok(())
}

/// A position that sorts (as text) after `low` and before `high` (none:
/// after `low` only), as short as it can be and never ending in `0`, so
/// there is always room for another; `None` when there is none between
/// (`high` not above `low`). Positions are digits, as Google's are.
pub(crate) fn between(low: &str, high: Option<&str>) -> Option<String> {
    if low
        .bytes()
        .chain(high.unwrap_or("").bytes())
        .any(|b| !b.is_ascii_digit())
    {
        return None;
    }
    let digit = |text: &str, ix: usize| text.as_bytes().get(ix).map(|b| b - b'0');
    let mut out = String::new();
    // While `out` is still the start of `high`, the next digit can't go
    // above `high`'s.
    let mut under_high = high;
    for ix in 0.. {
        // Past its end, `low` goes on as zeros.
        let l = digit(low, ix).unwrap_or(0);
        let h = match under_high {
            // `high` ended: it is not above `low`.
            Some(high) => digit(high, ix)?,
            None => 10,
        };
        if h < l {
            return None;
        }
        if h - l >= 2 {
            out.push(char::from(b'0' + (l + h) / 2));
            return Some(out);
        }
        if h - l == 1 {
            // Below `high` now, whatever follows.
            under_high = None;
        }
        out.push(char::from(b'0' + l));
    }
    None
}

impl Store {
    // --- Lists -----------------------------------------------------------

    /// Every list: the accounts' in account order, each account's default
    /// list first, then the ones on this computer.
    pub fn task_lists(&self) -> Result<Vec<TaskList>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT id, account_id, title, is_default FROM task_list WHERE deleted = 0
             ORDER BY account_id IS NULL, account_id, is_default DESC, title COLLATE NOCASE, id",
        )?;
        let rows = stmt.query_map([], list_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The list new tasks go to when none is named: the first account's
    /// default list once it has synced, else the one on this computer.
    pub fn default_task_list(&mut self) -> Result<i64> {
        let synced = self
            .pim
            .query_row(
                "SELECT id FROM task_list
                 WHERE account_id IS NOT NULL AND is_default = 1 AND deleted = 0
                   AND remote_id IS NOT NULL
                 ORDER BY account_id LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()?;
        if let Some(id) = synced {
            return Ok(id);
        }
        let local = self
            .pim
            .query_row(
                "SELECT id FROM task_list WHERE account_id IS NULL AND deleted = 0
                 ORDER BY is_default DESC, id LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()?;
        match local {
            Some(id) => Ok(id),
            None => self.add_task_list(None, "My Tasks"),
        }
    }

    /// Adds a list to `account`, or to this computer. Returns its ID.
    pub fn add_task_list(&mut self, account: Option<AccountId>, title: &str) -> Result<i64> {
        self.pim.execute(
            "INSERT INTO task_list (account_id, title, dirty) VALUES (?1, ?2, ?3)",
            params![account.map(|a| a.0), title, account.is_some()],
        )?;
        Ok(self.pim.last_insert_rowid())
    }

    /// Renames list `id`. Returns whether it exists.
    pub fn rename_task_list(&mut self, id: i64, title: &str) -> Result<bool> {
        let changed = self.pim.execute(
            "UPDATE task_list SET title = ?2, dirty = (account_id IS NOT NULL)
             WHERE id = ?1 AND deleted = 0",
            params![id, title],
        )?;
        Ok(changed > 0)
    }

    /// Deletes list `id` and its tasks. A list on the service stays
    /// hidden until the service has deleted it. Returns whether it existed.
    pub fn delete_task_list(&mut self, id: i64) -> Result<bool> {
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let remote: Option<Option<String>> = tx
            .query_row(
                "SELECT remote_id FROM task_list WHERE id = ?1 AND deleted = 0",
                [id],
                |row| row.get(0),
            )
            .optional()?;
        let Some(remote) = remote else {
            return Ok(false);
        };
        if remote.is_some() {
            tx.execute(
                "UPDATE task_list SET deleted = 1, dirty = 1 WHERE id = ?1",
                [id],
            )?;
        } else {
            tx.execute("DELETE FROM task_list WHERE id = ?1", [id])?;
        }
        tx.commit()?;
        Ok(true)
    }

    // --- Tasks -----------------------------------------------------------

    /// Open tasks, and those ticked off at or after `done_since` (Unix
    /// seconds), of every list: those with a due day first, earliest
    /// first, then the rest in the service's order, newest first.
    pub fn tasks(&self, done_since: i64) -> Result<Vec<Task>> {
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {TASK_COLUMNS} FROM task
             WHERE deleted = 0 AND (done_at IS NULL OR done_at >= ?1)
             ORDER BY due IS NULL, due, due_time IS NULL, due_time,
                      position = '', position, created_at DESC, id DESC"
        ))?;
        let rows = stmt.query_map([done_since], task_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Every task of list `list`, done ones too, in the service's order
    /// (new ones on top), steps after their task (new ones at the end, as
    /// in Google Tasks).
    pub fn tasks_in(&self, list: i64) -> Result<Vec<Task>> {
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {TASK_COLUMNS} FROM task WHERE list_id = ?1 AND deleted = 0
             ORDER BY position = '' DESC, position, created_at DESC, id DESC"
        ))?;
        let all: Vec<Task> = stmt
            .query_map([list], task_row)?
            .collect::<rusqlite::Result<_>>()?;
        // Each top-level task, then its steps.
        let mut steps: HashMap<i64, Vec<Task>> = HashMap::new();
        let mut top = Vec::new();
        for task in all {
            match task.parent {
                Some(parent) => steps.entry(parent).or_default().push(task),
                None => top.push(task),
            }
        }
        let mut ordered = Vec::with_capacity(top.len());
        for list in steps.values_mut() {
            // Not yet placed by the service: oldest first, after the rest.
            let (mut unplaced, placed): (Vec<Task>, Vec<Task>) =
                list.drain(..).partition(|t| t.position.is_empty());
            unplaced.reverse();
            list.extend(placed);
            list.extend(unplaced);
        }
        for task in top {
            let id = task.id;
            ordered.push(task);
            ordered.extend(steps.remove(&id).unwrap_or_default());
        }
        // Steps whose task is gone show at the end rather than not at all.
        ordered.extend(steps.into_values().flatten());
        Ok(ordered)
    }

    /// Every label on a task or a note (not one in Trash), each once, in
    /// order of name: the one set of labels Tasks and Notes share.
    pub fn labels_in_use(&self) -> Result<Vec<String>> {
        let mut labels: Vec<String> = Vec::new();
        for sql in [
            "SELECT labels FROM task WHERE deleted = 0 AND labels != '[]'",
            "SELECT labels FROM note WHERE trashed_at IS NULL AND labels != '[]'",
        ] {
            let mut stmt = self.pim.prepare_cached(sql)?;
            let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
            for json in rows {
                for label in labels_of(&json?) {
                    if !labels.contains(&label) {
                        labels.push(label);
                    }
                }
            }
        }
        labels.sort_by_key(|l| l.to_lowercase());
        Ok(labels)
    }

    /// Task `id`, unless deleted.
    pub fn task(&self, id: i64) -> Result<Option<Task>> {
        Ok(self
            .pim
            .prepare_cached(&format!(
                "SELECT {TASK_COLUMNS} FROM task WHERE id = ?1 AND deleted = 0"
            ))?
            .query_row([id], task_row)
            .optional()?)
    }

    /// Adds an open task to the default list ([`Self::default_task_list`]).
    /// `due` is `YYYY-MM-DD` or empty. Returns its ID.
    pub fn add_task(&mut self, title: &str, due: &str) -> Result<i64> {
        let list = self.default_task_list()?;
        let fields = TaskFields {
            title: title.to_owned(),
            due: due.to_owned(),
            ..TaskFields::default()
        };
        self.add_task_to(list, None, &fields)
    }

    /// Adds an open task to `list`, as a step of `parent` if given.
    /// Returns its ID.
    pub fn add_task_to(
        &mut self,
        list: i64,
        parent: Option<i64>,
        fields: &TaskFields,
    ) -> Result<i64> {
        let now = unix_now();
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let dirty = synced(&tx, list)?;
        tx.execute(
            "INSERT INTO task (list_id, parent_id, title, notes, due, due_time, remind_at,
                               repeat, starred, mail, dirty, created_at, updated_at, labels)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?12, ?13)",
            params![
                list,
                parent,
                fields.title,
                fields.notes,
                due_or_null(&fields.due),
                fields.due_time,
                fields.remind_at,
                fields.repeat,
                fields.starred,
                fields.mail,
                dirty,
                now,
                labels_json(&fields.labels)
            ],
        )?;
        let id = tx.last_insert_rowid();
        tx.commit()?;
        Ok(id)
    }

    /// Sets everything a person sets on task `id`. Returns whether it
    /// exists.
    pub fn edit_task(&mut self, id: i64, fields: &TaskFields) -> Result<bool> {
        let changed = self.pim.execute(
            "UPDATE task SET title = ?2, notes = ?3, due = ?4, due_time = ?5, remind_at = ?6,
                             repeat = ?7, starred = ?8, mail = ?9, updated_at = ?10, labels = ?11,
                             dirty = (dirty & 2) | (SELECT account_id IS NOT NULL FROM task_list
                                                    WHERE task_list.id = task.list_id)
             WHERE id = ?1 AND deleted = 0",
            params![
                id,
                fields.title,
                fields.notes,
                due_or_null(&fields.due),
                fields.due_time,
                fields.remind_at,
                fields.repeat,
                fields.starred,
                fields.mail,
                unix_now(),
                labels_json(&fields.labels)
            ],
        )?;
        Ok(changed > 0)
    }

    /// Ticks task `id` off, or opens it again. Returns whether it exists.
    pub fn set_task_done(&mut self, id: i64, done: bool) -> Result<bool> {
        let now = unix_now();
        let done_at = done.then_some(now);
        // Ticking a done task again keeps when it was first done.
        let changed = self.pim.execute(
            "UPDATE task SET done_at = CASE WHEN ?2 IS NULL THEN NULL ELSE COALESCE(done_at, ?2) END,
                             updated_at = ?3,
                             dirty = (dirty & 2) | (SELECT account_id IS NOT NULL FROM task_list
                                                    WHERE task_list.id = task.list_id)
             WHERE id = ?1 AND deleted = 0",
            params![id, done_at, now],
        )?;
        Ok(changed > 0)
    }

    /// Moves task `id` (with its steps) to list `list`, on top. On the
    /// service that is a delete from the old list and an insert into the
    /// new one, as Google's own apps do it. Returns whether it exists.
    pub fn move_task(&mut self, id: i64, list: i64) -> Result<bool> {
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let Some(task) = task_in(&tx, id)? else {
            return Ok(false);
        };
        if task.list != list {
            move_to_list(&tx, id, list)?;
        }
        tx.commit()?;
        Ok(true)
    }

    /// Puts task `id` (a task, not a step) in list `list` right after
    /// task `after` of that list, or first, as dragging it on the Tasks
    /// page does; from another list it moves as with [`Self::move_task`].
    /// Google Tasks keeps the order, so there the task is marked to be
    /// moved on Google too ([`PendingTask::place`]); other lists keep it
    /// here only, their tasks numbered anew. Returns whether it exists (a
    /// step is never placed).
    pub fn place_task(&mut self, id: i64, list: i64, after: Option<i64>) -> Result<bool> {
        let google = self.list_keeps_order(list)?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let Some(task) = task_in(&tx, id)? else {
            return Ok(false);
        };
        if task.parent.is_some() {
            return Ok(false);
        }
        if task.list != list {
            move_to_list(&tx, id, list)?;
        }
        let siblings: Vec<(i64, String)> = top_level(&tx, list)?
            .into_iter()
            .filter(|(sibling, _)| *sibling != id)
            .collect();
        // Right after `after`; first when it is gone meanwhile.
        let at = after
            .and_then(|after| siblings.iter().position(|(s, _)| *s == after))
            .map_or(0, |ix| ix + 1);
        let now = unix_now();
        if google {
            // Between its neighbours as Google placed them; those not sent
            // yet have no place there, and sort first anyway.
            let before = siblings[..at]
                .iter()
                .rev()
                .map(|(_, p)| p.as_str())
                .find(|p| !p.is_empty())
                .unwrap_or("");
            let next = siblings[at..]
                .iter()
                .map(|(_, p)| p.as_str())
                .find(|p| !p.is_empty());
            let position = between(before, next).unwrap_or_else(|| format!("{before}5"));
            tx.execute(
                &format!(
                    "UPDATE task SET position = ?2, dirty = dirty | {MOVED}, updated_at = ?3
                     WHERE id = ?1"
                ),
                params![id, position, now],
            )?;
        } else {
            let mut order: Vec<i64> = siblings.iter().map(|(s, _)| *s).collect();
            order.insert(at, id);
            let old: HashMap<i64, String> = siblings.into_iter().collect();
            for (ix, each) in order.into_iter().enumerate() {
                let position = format!("{:010}", ix + 1);
                if old.get(&each) != Some(&position) {
                    tx.execute(
                        "UPDATE task SET position = ?2 WHERE id = ?1",
                        params![each, position],
                    )?;
                }
            }
        }
        tx.commit()?;
        Ok(true)
    }

    /// Whether list `list` is on Google Tasks, which keeps the order of
    /// its tasks; the order of other lists is kept here only.
    fn list_keeps_order(&self, list: i64) -> Result<bool> {
        let account: Option<i64> = self
            .pim
            .query_row(
                "SELECT account_id FROM task_list WHERE id = ?1",
                [list],
                |row| row.get(0),
            )
            .optional()?
            .flatten();
        let Some(account) = account else {
            return Ok(false);
        };
        Ok(self
            .account_settings(AccountId(account))?
            .and_then(|s| s.oauth)
            == Some(OAuthProvider::Google))
    }

    /// Deletes task `id` and its steps. Returns it, so it can be put back
    /// ([`Self::add_task_to`]).
    pub fn delete_task(&mut self, id: i64) -> Result<Option<Task>> {
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let task = tx
            .prepare_cached(&format!(
                "SELECT {TASK_COLUMNS} FROM task WHERE id = ?1 AND deleted = 0"
            ))?
            .query_row([id], task_row)
            .optional()?;
        if task.is_some() {
            let now = unix_now();
            // Those the service has become tombstones until it deleted them
            // too; steps go with their task.
            tx.execute(
                "UPDATE task SET deleted = 1, dirty = 1, updated_at = ?2
                 WHERE (id = ?1 OR parent_id = ?1) AND remote_id IS NOT NULL",
                params![id, now],
            )?;
            for sql in [
                "DELETE FROM task WHERE parent_id = ?1 AND remote_id IS NULL",
                "DELETE FROM task WHERE id = ?1 AND remote_id IS NULL",
            ] {
                tx.execute(sql, [id])?;
            }
        }
        tx.commit()?;
        Ok(task)
    }

    // --- Sync --------------------------------------------------------------

    /// The lists of `account` on the service, as last synced.
    pub fn account_task_lists(&self, account: AccountId) -> Result<Vec<(i64, Option<String>)>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT id, remote_id FROM task_list WHERE account_id = ?1 AND deleted = 0 ORDER BY id",
        )?;
        let rows = stmt.query_map([account.0], |row| Ok((row.get(0)?, row.get(1)?)))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Takes the service's lists of `account`: adds new ones, renames, and
    /// drops those gone from the service (with their tasks) unless renamed
    /// here meanwhile. Returns whether anything changed.
    pub fn sync_task_lists(
        &mut self,
        account: AccountId,
        lists: &[RemoteTaskList],
    ) -> Result<bool> {
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut changed = 0;
        for list in lists {
            changed += tx.execute(
                "INSERT INTO task_list (account_id, remote_id, title, is_default)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT (account_id, remote_id) DO UPDATE SET
                     title = CASE WHEN dirty THEN title ELSE excluded.title END,
                     is_default = excluded.is_default
                 WHERE (title != excluded.title AND NOT dirty) OR is_default != excluded.is_default",
                params![account.0, list.remote_id, list.title, list.is_default],
            )?;
        }
        let known: Vec<(i64, String)> = tx
            .prepare_cached(
                "SELECT id, remote_id FROM task_list
                 WHERE account_id = ?1 AND remote_id IS NOT NULL AND dirty = 0",
            )?
            .query_map([account.0], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<rusqlite::Result<_>>()?;
        for (id, remote) in known {
            if !lists.iter().any(|list| list.remote_id == remote) {
                changed += tx.execute("DELETE FROM task_list WHERE id = ?1", [id])?;
            }
        }
        tx.commit()?;
        Ok(changed > 0)
    }

    /// The list changes of `account` waiting to go to the service.
    pub fn pending_task_lists(&self, account: AccountId) -> Result<Vec<PendingTaskList>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT id, remote_id, title, deleted FROM task_list
             WHERE account_id = ?1 AND (dirty = 1 OR remote_id IS NULL) ORDER BY id",
        )?;
        let rows = stmt.query_map([account.0], |row| {
            Ok(PendingTaskList {
                id: row.get(0)?,
                remote_id: row.get(1)?,
                title: row.get(2)?,
                deleted: row.get(3)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// List `id` is on the service as `remote_id`, as it is here.
    pub fn task_list_pushed(&mut self, id: i64, remote_id: &str) -> Result<()> {
        self.pim.execute(
            "UPDATE task_list SET remote_id = ?2, dirty = 0 WHERE id = ?1",
            params![id, remote_id],
        )?;
        Ok(())
    }

    /// Forgets list `id` and its tasks: the service deleted it.
    pub fn forget_task_list(&mut self, id: i64) -> Result<()> {
        self.pim
            .execute("DELETE FROM task_list WHERE id = ?1", [id])?;
        Ok(())
    }

    /// Where the last pull of list `id` ended.
    pub fn task_list_sync_state(&self, id: i64) -> Result<Option<String>> {
        Ok(self
            .pim
            .query_row(
                "SELECT sync_state FROM task_list WHERE id = ?1",
                [id],
                |row| row.get(0),
            )
            .optional()?
            .flatten())
    }

    pub fn set_task_list_sync_state(&mut self, id: i64, state: Option<&str>) -> Result<()> {
        self.pim.execute(
            "UPDATE task_list SET sync_state = ?2 WHERE id = ?1",
            params![id, state],
        )?;
        Ok(())
    }

    /// The task changes of list `list` waiting to go to the service:
    /// tasks before their steps, so a step's parent is there first.
    pub fn pending_tasks(&self, list: i64) -> Result<Vec<PendingTask>> {
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {TASK_COLUMNS}, remote_id, etag, deleted, updated_at,
                    (SELECT p.remote_id FROM task p WHERE p.id = task.parent_id), dirty
             FROM task WHERE list_id = ?1 AND dirty != 0
             ORDER BY parent_id IS NOT NULL, created_at, id"
        ))?;
        let rows = stmt.query_map([list], |row| {
            let dirty: i64 = row.get(19)?;
            Ok(PendingTask {
                task: task_row(row)?,
                remote_id: row.get(14)?,
                etag: row.get(15)?,
                deleted: row.get(16)?,
                stamp: row.get(17)?,
                parent_remote: row.get(18)?,
                edited: dirty & EDITED != 0,
                place: (dirty & MOVED != 0).then_some(Place::First),
            })
        })?;
        let mut pending: Vec<PendingTask> = rows.collect::<rusqlite::Result<_>>()?;
        if pending.iter().any(|p| p.place.is_some()) {
            // The task before each moved one, as the list shows them.
            let order = top_level(&self.pim, list)?;
            for each in pending.iter_mut().filter(|p| p.place.is_some()) {
                let ix = order.iter().position(|(id, _)| *id == each.task.id);
                let before = ix.and_then(|ix| ix.checked_sub(1)).map(|ix| order[ix].0);
                each.place = Some(match before {
                    None => Place::First,
                    Some(before) => {
                        let remote: Option<String> = self.pim.query_row(
                            "SELECT remote_id FROM task WHERE id = ?1",
                            [before],
                            |row| row.get(0),
                        )?;
                        remote.map_or(Place::Waiting, Place::After)
                    }
                });
            }
        }
        Ok(pending)
    }

    /// Task `id` is on the service as `remote`, as it was at `stamp`, and
    /// in its place there unless `placed` is false (it waits for the task
    /// before it); it stays dirty if it changed again since. A service
    /// that keeps no order (an empty [`RemoteTask::position`]) leaves the
    /// place it has here.
    pub fn task_pushed(
        &mut self,
        id: i64,
        stamp: i64,
        remote: &RemoteTask,
        placed: bool,
    ) -> Result<()> {
        let position = if placed { remote.position.as_str() } else { "" };
        self.pim.execute(
            &format!(
                "UPDATE task SET remote_id = ?3, etag = ?4,
                                 position = CASE WHEN ?5 = '' THEN position ELSE ?5 END,
                                 dirty = CASE WHEN updated_at != ?2 THEN dirty
                                              WHEN ?6 THEN 0 ELSE {MOVED} END
                 WHERE id = ?1"
            ),
            params![id, stamp, remote.remote_id, remote.etag, position, placed],
        )?;
        Ok(())
    }

    /// Marks the steps of list `list` that never reached the service to
    /// be sent: To Do steps were once kept on this computer only.
    pub fn resend_unsent_steps(&mut self, list: i64) -> Result<()> {
        self.pim.execute(
            "UPDATE task SET dirty = 1
             WHERE list_id = ?1 AND parent_id IS NOT NULL AND remote_id IS NULL
               AND dirty = 0 AND deleted = 0",
            [list],
        )?;
        Ok(())
    }

    /// Deletes the steps of list `list` under the tasks whose service IDs
    /// are `parents` that `tasks` doesn't hold: the service read all of
    /// those tasks' steps, so the others went there. Steps changed here and
    /// not yet sent stay. Returns whether any went.
    pub fn drop_unlisted_steps(
        &mut self,
        list: i64,
        parents: &[String],
        tasks: &[RemoteTask],
    ) -> Result<bool> {
        if parents.is_empty() {
            return Ok(false);
        }
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut changed = 0;
        for parent in parents {
            let steps: Vec<(i64, String)> = tx
                .prepare_cached(
                    "SELECT s.id, s.remote_id FROM task s JOIN task p ON p.id = s.parent_id
                     WHERE s.list_id = ?1 AND p.remote_id = ?2
                       AND s.remote_id IS NOT NULL AND s.dirty = 0",
                )?
                .query_map(params![list, parent], |row| Ok((row.get(0)?, row.get(1)?)))?
                .collect::<rusqlite::Result<_>>()?;
            for (id, remote) in steps {
                if !tasks.iter().any(|t| t.remote_id == remote && !t.deleted) {
                    changed += tx.execute("DELETE FROM task WHERE id = ?1", [id])?;
                }
            }
        }
        tx.commit()?;
        Ok(changed > 0)
    }

    /// Forgets task `id` for good: its deletion reached the service, or
    /// the service had it deleted already.
    pub fn forget_task(&mut self, id: i64) -> Result<()> {
        self.pim.execute("DELETE FROM task WHERE id = ?1", [id])?;
        Ok(())
    }

    /// Takes the service's tasks of list `list`: `all` is every task the
    /// service has, else only those that changed. Rows changed here and
    /// not yet sent keep their values. Returns whether anything changed.
    pub fn sync_tasks(&mut self, list: i64, tasks: &[RemoteTask], all: bool) -> Result<bool> {
        let now = unix_now();
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut changed = 0;
        for task in tasks {
            let local: Option<(i64, bool, Option<String>)> = tx
                .query_row(
                    "SELECT id, dirty, etag FROM task WHERE list_id = ?1 AND remote_id = ?2",
                    params![list, task.remote_id],
                    |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
                )
                .optional()?;
            match local {
                Some((_, true, _)) => {}
                Some((id, false, _)) if task.deleted => {
                    changed += tx.execute("DELETE FROM task WHERE id = ?1", [id])?;
                }
                None if task.deleted => {}
                Some((_, false, etag))
                    if !task.etag.is_empty() && etag.as_deref() == Some(&task.etag) => {}
                Some((id, false, _)) => {
                    changed += tx.execute(
                        "UPDATE task SET title = ?2, notes = ?3, due = ?4, done_at = ?5,
                                         position = CASE WHEN ?6 = '' THEN position ELSE ?6 END,
                                         etag = ?7, updated_at = ?8
                         WHERE id = ?1",
                        params![
                            id,
                            task.title,
                            task.notes,
                            due_or_null(&task.due),
                            task.done_at,
                            task.position,
                            task.etag,
                            now
                        ],
                    )?;
                    if let Some(extras) = &task.extras {
                        tx.execute(
                            "UPDATE task SET remind_at = ?2, repeat = ?3 WHERE id = ?1",
                            params![id, extras.remind_at, extras.repeat],
                        )?;
                    }
                    if let Some(starred) = task.starred {
                        tx.execute(
                            "UPDATE task SET starred = ?2 WHERE id = ?1",
                            params![id, starred],
                        )?;
                    }
                    if let Some(labels) = &task.labels {
                        tx.execute(
                            "UPDATE task SET labels = ?2 WHERE id = ?1",
                            params![id, labels_json(labels)],
                        )?;
                    }
                }
                None => {
                    let extras = task.extras.clone().unwrap_or_default();
                    let labels = task.labels.as_deref().unwrap_or_default();
                    changed += tx.execute(
                        "INSERT INTO task (list_id, remote_id, title, notes, due, done_at,
                                           position, etag, remind_at, repeat, starred,
                                           created_at, updated_at, labels)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?12, ?13)",
                        params![
                            list,
                            task.remote_id,
                            task.title,
                            task.notes,
                            due_or_null(&task.due),
                            task.done_at,
                            task.position,
                            task.etag,
                            extras.remind_at,
                            extras.repeat,
                            task.starred.unwrap_or(false),
                            now,
                            labels_json(labels)
                        ],
                    )?;
                }
            }
        }
        // Parents, once every task is there.
        for task in tasks.iter().filter(|task| !task.deleted) {
            changed += tx.execute(
                "UPDATE task SET parent_id =
                     (SELECT p.id FROM task p WHERE p.list_id = ?1 AND p.remote_id = ?3)
                 WHERE list_id = ?1 AND remote_id = ?2 AND dirty = 0
                   AND parent_id IS NOT (SELECT p.id FROM task p
                                         WHERE p.list_id = ?1 AND p.remote_id = ?3)",
                params![list, task.remote_id, task.parent],
            )?;
        }
        if all {
            let known: Vec<(i64, String)> = tx
                .prepare_cached(
                    "SELECT id, remote_id FROM task
                     WHERE list_id = ?1 AND remote_id IS NOT NULL AND dirty = 0",
                )?
                .query_map([list], |row| Ok((row.get(0)?, row.get(1)?)))?
                .collect::<rusqlite::Result<_>>()?;
            for (id, remote) in known {
                if !tasks.iter().any(|t| t.remote_id == remote && !t.deleted) {
                    changed += tx.execute("DELETE FROM task WHERE id = ?1", [id])?;
                }
            }
        }
        tx.commit()?;
        Ok(changed > 0)
    }

    /// Moves the tasks of lists marked to move out (those added in the
    /// desktop clock before any account's list had synced) into the
    /// default list, once one on a service exists. Returns how many moved.
    pub fn move_out_local_tasks(&mut self) -> Result<usize> {
        let target = self.default_task_list()?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        if !synced(&tx, target)? {
            return Ok(0);
        }
        let moved = tx.execute(
            "UPDATE task SET list_id = ?1, dirty = 1, updated_at = ?2
             WHERE list_id IN (SELECT id FROM task_list WHERE move_out = 1)",
            params![target, unix_now()],
        )?;
        // The emptied list goes; a person can make one on this computer
        // again whenever they like.
        tx.execute("DELETE FROM task_list WHERE move_out = 1", [])?;
        tx.commit()?;
        Ok(moved)
    }
}

#[cfg(test)]
mod tests;
