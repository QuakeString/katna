// SPDX-License-Identifier: GPL-3.0-or-later

//! Tasks Katna keeps on this computer (`task` in `pim.db`), for the
//! desktop clock's Tasks list.

use rusqlite::{OptionalExtension, params};

use crate::Store;
use crate::db::unix_now;
use crate::error::Result;

/// One task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub notes: String,
    /// `YYYY-MM-DD`, or empty.
    pub due: String,
    /// When it was ticked off (Unix seconds); `None` while open.
    pub done_at: Option<i64>,
}

impl Store {
    /// Open tasks, and those ticked off at or after `done_since` (Unix
    /// seconds): those with a due day first, earliest first, then the rest
    /// newest first.
    pub fn tasks(&self, done_since: i64) -> Result<Vec<Task>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT id, title, notes, due, done_at FROM task
             WHERE done_at IS NULL OR done_at >= ?1
             ORDER BY due IS NULL, due, created_at DESC, id DESC",
        )?;
        let rows = stmt.query_map([done_since], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                notes: row.get(2)?,
                due: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
                done_at: row.get(4)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Adds an open task. `due` is `YYYY-MM-DD` or empty. Returns its ID.
    pub fn add_task(&mut self, title: &str, due: &str) -> Result<i64> {
        let now = unix_now();
        let due = (!due.is_empty()).then_some(due);
        self.pim.execute(
            "INSERT INTO task (title, due, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)",
            params![title, due, now],
        )?;
        Ok(self.pim.last_insert_rowid())
    }

    /// Ticks task `id` off, or opens it again. Returns whether it exists.
    pub fn set_task_done(&mut self, id: i64, done: bool) -> Result<bool> {
        let now = unix_now();
        let done_at = done.then_some(now);
        // Ticking a done task again keeps when it was first done.
        let changed = self.pim.execute(
            "UPDATE task SET done_at = CASE WHEN ?2 IS NULL THEN NULL ELSE COALESCE(done_at, ?2) END,
                             updated_at = ?3
             WHERE id = ?1",
            params![id, done_at, now],
        )?;
        Ok(changed > 0)
    }

    /// Deletes task `id`. Returns it, so it can be put back.
    pub fn delete_task(&mut self, id: i64) -> Result<Option<Task>> {
        let task = self
            .pim
            .prepare_cached("SELECT title, notes, due, done_at FROM task WHERE id = ?1")?
            .query_row([id], |row| {
                Ok(Task {
                    id,
                    title: row.get(0)?,
                    notes: row.get(1)?,
                    due: row.get::<_, Option<String>>(2)?.unwrap_or_default(),
                    done_at: row.get(3)?,
                })
            })
            .optional()?;
        if task.is_some() {
            self.pim.execute("DELETE FROM task WHERE id = ?1", [id])?;
        }
        Ok(task)
    }
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

    fn titles(tasks: &[Task]) -> Vec<&str> {
        tasks.iter().map(|task| task.title.as_str()).collect()
    }

    #[test]
    fn lists_due_tasks_first_by_day() {
        let (_dir, mut store) = store();
        store.add_task("someday", "").unwrap();
        store.add_task("friday", "2026-10-02").unwrap();
        store.add_task("today", "2026-09-29").unwrap();
        let tasks = store.tasks(0).unwrap();
        assert_eq!(titles(&tasks), ["today", "friday", "someday"]);
        assert_eq!(tasks[0].due, "2026-09-29");
        assert_eq!(tasks[2].due, "");
    }

    #[test]
    fn done_tasks_leave_the_list_after_a_while() {
        let (_dir, mut store) = store();
        let id = store.add_task("renew domain", "").unwrap();
        assert!(store.set_task_done(id, true).unwrap());
        let done = store.tasks(0).unwrap();
        let done_at = done[0].done_at.expect("ticked off");
        assert_eq!(titles(&store.tasks(done_at).unwrap()), ["renew domain"]);
        assert!(store.tasks(done_at + 1).unwrap().is_empty());

        assert!(store.set_task_done(id, false).unwrap());
        assert_eq!(store.tasks(i64::MAX).unwrap()[0].done_at, None);
        assert!(!store.set_task_done(id + 1, true).unwrap());
    }

    #[test]
    fn deleting_returns_the_task() {
        let (_dir, mut store) = store();
        let id = store.add_task("call the bank", "2026-09-30").unwrap();
        let task = store.delete_task(id).unwrap().expect("existed");
        assert_eq!(task.title, "call the bank");
        assert_eq!(task.due, "2026-09-30");
        assert!(store.tasks(0).unwrap().is_empty());
        assert_eq!(store.delete_task(id).unwrap(), None);
    }
}
