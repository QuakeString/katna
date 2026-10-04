// SPDX-License-Identifier: GPL-3.0-or-later

//! Files on tasks (`task_file` in `pim.db`, bytes in `blobs.db`): dropped
//! on a task, or kept from the mail a task was made from. Each goes to the
//! task's service where it can keep files (To Do's attachments, a CalDAV
//! server's inline `ATTACH`); the others, and those the service refused,
//! stay on this computer (`local_only`).

use rusqlite::{OptionalExtension, Row, Transaction, TransactionBehavior, params};

use crate::Store;
use crate::blob::BlobHash;
use crate::db::unix_now;
use crate::error::Result;

/// A file on a task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskFile {
    pub id: i64,
    pub task: i64,
    pub name: String,
    /// `type/subtype`.
    pub mime: String,
    pub size: u64,
    /// Its bytes in the blob store.
    pub hash: BlobHash,
    /// The service's ID once sent.
    pub remote_id: Option<String>,
    /// The service can't keep it: it stays on this computer.
    pub local_only: bool,
}

/// A file as the service has it. `data` is its content when the service
/// sent it with the list (CalDAV); `None` asks for it apart (To Do).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RemoteFile {
    pub remote_id: String,
    pub name: String,
    pub mime: String,
    pub size: u64,
    pub data: Option<Vec<u8>>,
}

/// A file change waiting to go to the service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingFile {
    pub file: TaskFile,
    /// Its task's ID on the service.
    pub task_remote: String,
    /// Removed here: the service removes it too.
    pub deleted: bool,
}

/// What [`Store::sync_task_files`] did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SyncedFiles {
    pub changed: bool,
    /// The service's files Katna doesn't have yet whose content came
    /// without them: read each and give it to [`Store::add_remote_task_file`].
    pub wanted: Vec<RemoteFile>,
}

const FILE_COLUMNS: &str = "id, task_id, name, mime, size, hash, remote_id, local_only";

fn file_row(row: &Row<'_>) -> rusqlite::Result<TaskFile> {
    let hash: Vec<u8> = row.get(5)?;
    let hash = <[u8; 32]>::try_from(hash.as_slice()).unwrap_or([0; 32]);
    Ok(TaskFile {
        id: row.get(0)?,
        task: row.get(1)?,
        name: row.get(2)?,
        mime: row.get(3)?,
        size: u64::try_from(row.get::<_, i64>(4)?).unwrap_or(0),
        hash: BlobHash::from_bytes(hash),
        remote_id: row.get(6)?,
        local_only: row.get(7)?,
    })
}

/// Task `task` moved to another list: its files go to that list's service
/// (or stay here) afresh; those on the old service stay there with the
/// task's tombstone.
pub(super) fn moved(tx: &Transaction<'_>, task: i64) -> rusqlite::Result<()> {
    tx.execute(
        "DELETE FROM task_file WHERE task_id = ?1 AND deleted = 1",
        [task],
    )?;
    tx.execute(
        "UPDATE task_file SET remote_id = NULL, local_only = 0 WHERE task_id = ?1",
        [task],
    )?;
    Ok(())
}

/// A safe file name: no folders, no control characters, not empty.
fn clean_name(name: &str) -> String {
    let base = name.rsplit(['/', '\\']).next().unwrap_or_default();
    let clean: String = base.chars().filter(|c| !c.is_control()).collect();
    let clean = clean.trim();
    if clean.is_empty() {
        "file".to_owned()
    } else {
        clean.chars().take(200).collect()
    }
}

impl Store {
    /// The files of every task, oldest first, for the Tasks page.
    pub fn task_files(&self) -> Result<Vec<TaskFile>> {
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {FILE_COLUMNS} FROM task_file WHERE deleted = 0 ORDER BY task_id, id"
        ))?;
        let rows = stmt.query_map([], file_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The files of task `task`, oldest first.
    pub fn files_of_task(&self, task: i64) -> Result<Vec<TaskFile>> {
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {FILE_COLUMNS} FROM task_file WHERE task_id = ?1 AND deleted = 0 ORDER BY id"
        ))?;
        let rows = stmt.query_map([task], file_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// File `id`, unless removed.
    pub fn task_file(&self, id: i64) -> Result<Option<TaskFile>> {
        Ok(self
            .pim
            .prepare_cached(&format!(
                "SELECT {FILE_COLUMNS} FROM task_file WHERE id = ?1 AND deleted = 0"
            ))?
            .query_row([id], file_row)
            .optional()?)
    }

    /// The bytes of file `id`; `None` when it or its bytes are gone.
    pub fn task_file_data(&self, id: i64) -> Result<Option<Vec<u8>>> {
        let Some(file) = self.task_file(id)? else {
            return Ok(None);
        };
        self.blobs.get(&file.hash)
    }

    /// Puts a file on task `task` (a deleted one takes none). Returns its
    /// ID.
    pub fn add_task_file(
        &mut self,
        task: i64,
        name: &str,
        mime: &str,
        data: &[u8],
    ) -> Result<Option<i64>> {
        self.check_writable()?;
        let exists: bool = self.pim.query_row(
            "SELECT EXISTS (SELECT 1 FROM task WHERE id = ?1 AND deleted = 0)",
            [task],
            |row| row.get(0),
        )?;
        if !exists {
            return Ok(None);
        }
        // The bytes first: a row never names a blob that isn't there.
        let hash = self.blobs.put(data)?;
        let mime = if mime.trim().is_empty() {
            "application/octet-stream"
        } else {
            mime.trim()
        };
        self.pim.execute(
            "INSERT INTO task_file (task_id, name, mime, size, hash, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                task,
                clean_name(name),
                mime,
                i64::try_from(data.len()).unwrap_or(i64::MAX),
                hash.as_bytes(),
                unix_now()
            ],
        )?;
        Ok(Some(self.pim.last_insert_rowid()))
    }

    /// Takes file `id` off its task. One on the service stays as a
    /// tombstone until the service removed it too. Its bytes stay in the
    /// blob store, so Undo can put it back. Returns whether it was there.
    pub fn remove_task_file(&mut self, id: i64) -> Result<bool> {
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let removed = tx.execute(
            "UPDATE task_file SET deleted = 1
             WHERE id = ?1 AND deleted = 0 AND remote_id IS NOT NULL AND local_only = 0",
            [id],
        )? + tx.execute(
            "DELETE FROM task_file WHERE id = ?1 AND deleted = 0
               AND (remote_id IS NULL OR local_only = 1)",
            [id],
        )?;
        tx.commit()?;
        Ok(removed > 0)
    }

    /// The file changes of list `list` waiting to go to its service: new
    /// files of tasks already there, and files removed here that are there.
    pub fn pending_task_files(&self, list: i64) -> Result<Vec<PendingFile>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT f.id, f.task_id, f.name, f.mime, f.size, f.hash, f.remote_id,
                    f.local_only, t.remote_id, f.deleted
             FROM task_file f JOIN task t ON t.id = f.task_id
             WHERE t.list_id = ?1 AND t.remote_id IS NOT NULL AND f.local_only = 0
               AND ((f.deleted = 0 AND f.remote_id IS NULL AND t.deleted = 0)
                    OR (f.deleted = 1 AND f.remote_id IS NOT NULL))
             ORDER BY f.task_id, f.id",
        )?;
        let rows = stmt.query_map([list], |row| {
            Ok(PendingFile {
                file: file_row(row)?,
                task_remote: row.get(8)?,
                deleted: row.get(9)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// File `id` is on the service as `remote_id`; `None`: the service
    /// can't keep it, so it stays on this computer.
    pub fn task_file_pushed(&mut self, id: i64, remote_id: Option<&str>) -> Result<()> {
        self.pim.execute(
            "UPDATE task_file SET remote_id = COALESCE(?2, remote_id), local_only = ?3
             WHERE id = ?1",
            params![id, remote_id, remote_id.is_none()],
        )?;
        Ok(())
    }

    /// Forgets file `id` for good: its removal reached the service.
    pub fn forget_task_file(&mut self, id: i64) -> Result<()> {
        self.pim
            .execute("DELETE FROM task_file WHERE id = ?1", [id])?;
        Ok(())
    }

    /// Takes the service's files of its task `task_remote` in list `list`:
    /// adds those with their content, says which it still wants, and drops
    /// those gone from the service. Files not sent yet, and those kept on
    /// this computer only, stay.
    pub fn sync_task_files(
        &mut self,
        list: i64,
        task_remote: &str,
        files: &[RemoteFile],
    ) -> Result<SyncedFiles> {
        let task: Option<i64> = self
            .pim
            .query_row(
                "SELECT id FROM task WHERE list_id = ?1 AND remote_id = ?2 AND deleted = 0",
                params![list, task_remote],
                |row| row.get(0),
            )
            .optional()?;
        let Some(task) = task else {
            return Ok(SyncedFiles::default());
        };
        let known: Vec<(i64, String)> = self
            .pim
            .prepare_cached(
                "SELECT id, remote_id FROM task_file
                 WHERE task_id = ?1 AND remote_id IS NOT NULL AND local_only = 0",
            )?
            .query_map([task], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<rusqlite::Result<_>>()?;
        let mut out = SyncedFiles::default();
        for (id, remote) in &known {
            if !files.iter().any(|f| f.remote_id == *remote) {
                self.forget_task_file(*id)?;
                out.changed = true;
            }
        }
        for file in files {
            if known.iter().any(|(_, r)| *r == file.remote_id) {
                continue;
            }
            match &file.data {
                Some(_) => out.changed |= self.add_remote_task_file(list, task_remote, file)?,
                None => out.wanted.push(file.clone()),
            }
        }
        Ok(out)
    }

    /// Adds the service's file `file` (with its content) to its task
    /// `task_remote` in list `list`. Returns whether it was added.
    pub fn add_remote_task_file(
        &mut self,
        list: i64,
        task_remote: &str,
        file: &RemoteFile,
    ) -> Result<bool> {
        let Some(data) = &file.data else {
            return Ok(false);
        };
        let task: Option<i64> = self
            .pim
            .query_row(
                "SELECT id FROM task WHERE list_id = ?1 AND remote_id = ?2 AND deleted = 0",
                params![list, task_remote],
                |row| row.get(0),
            )
            .optional()?;
        let Some(task) = task else {
            return Ok(false);
        };
        let Some(id) = self.add_task_file(task, &file.name, &file.mime, data)? else {
            return Ok(false);
        };
        self.task_file_pushed(id, Some(&file.remote_id))?;
        Ok(true)
    }

    /// The blobs task files use, for the blob store's clean-up.
    pub(crate) fn task_file_hashes(&self) -> Result<Vec<BlobHash>> {
        let mut stmt = self.pim.prepare("SELECT DISTINCT hash FROM task_file")?;
        let rows = stmt.query_map([], |row| row.get::<_, Vec<u8>>(0))?;
        let mut out = Vec::new();
        for hash in rows {
            if let Ok(bytes) = <[u8; 32]>::try_from(hash?.as_slice()) {
                out.push(BlobHash::from_bytes(bytes));
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_names_lose_folders_and_control_characters() {
        assert_eq!(clean_name("/home/me/Bills/june.pdf"), "june.pdf");
        assert_eq!(clean_name("C:\\x\\a\nb.txt"), "ab.txt");
        assert_eq!(clean_name("  "), "file");
    }
}
