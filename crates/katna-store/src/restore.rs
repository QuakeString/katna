// SPDX-License-Identifier: GPL-3.0-or-later

//! Going back to the copies made before an update (`docs/ARCHITECTURE.md`
//! §21.2, "After an update").
//!
//! Each migration copies its database into `backup/` first ([`crate::db`]).
//! One update migrates several databases within moments, so copies made
//! close together form one [`RestorePoint`]. Restoring one moves the
//! databases as they are now into `before-restore-<unix time>/` beside
//! them, so nothing is lost, and puts the copies in their place. Only the
//! daemon does this, before it opens the store.

use std::path::{Path, PathBuf};

use katna_core::Paths;

use crate::db::{Backup, backups};
use crate::error::{Error, Result};

/// Copies made at most this far apart belong to one update, in seconds.
const SAME_UPDATE: i64 = 10 * 60;

/// The databases an update can migrate.
fn databases(paths: &Paths) -> [PathBuf; 3] {
    [paths.mail_db(), paths.pim_db(), paths.blobs_db()]
}

/// The copies one update made, one per database it migrated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestorePoint {
    /// When the newest of its copies was made, in Unix seconds. It names
    /// the point in [`restore`].
    pub made_at: i64,
    /// Each copy, with the database it replaces.
    pub copies: Vec<(PathBuf, Backup)>,
}

impl RestorePoint {
    /// The file names of the databases it restores, such as `mail.db`.
    pub fn names(&self) -> Vec<String> {
        self.copies
            .iter()
            .filter_map(|(db, _)| Some(db.file_name()?.to_string_lossy().into_owned()))
            .collect()
    }
}

/// The restore points there are, newest first.
pub fn restore_points(paths: &Paths) -> Vec<RestorePoint> {
    let mut all: Vec<(PathBuf, Backup)> = databases(paths)
        .into_iter()
        .flat_map(|db| backups(&db).into_iter().map(move |b| (db.clone(), b)))
        .collect();
    all.sort_by_key(|(_, b)| std::cmp::Reverse(b.made_at));
    let mut points: Vec<RestorePoint> = Vec::new();
    for (db, backup) in all {
        match points.last_mut() {
            // Newest first: this copy is at most as new as the point's
            // oldest, and its database isn't in the point yet.
            Some(point)
                if point
                    .copies
                    .last()
                    .is_some_and(|(_, last)| last.made_at - backup.made_at <= SAME_UPDATE)
                    && !point.copies.iter().any(|(d, _)| *d == db) =>
            {
                point.copies.push((db, backup));
            }
            _ => points.push(RestorePoint {
                made_at: backup.made_at,
                copies: vec![(db, backup)],
            }),
        }
    }
    points
}

/// Restores the point made at `made_at`: the databases it covers are moved
/// into a new `before-restore-<now>/` folder beside them, then replaced by
/// their copies. Returns that folder. The store must not be open.
pub fn restore(paths: &Paths, made_at: i64, now: i64) -> Result<PathBuf> {
    let point = restore_points(paths)
        .into_iter()
        .find(|p| p.made_at == made_at)
        .ok_or(Error::NoSuchBackup(made_at))?;
    let saved = paths.data_dir().join(format!("before-restore-{now}"));
    std::fs::create_dir_all(&saved).map_err(|err| Error::io(&saved, err))?;
    for (db, backup) in &point.copies {
        tracing::info!(db = %db.display(), from = %backup.path.display(), "restoring");
        // The copy goes beside the database first, so a full disk fails
        // before anything is moved.
        let incoming = db.with_extension("restoring");
        std::fs::copy(&backup.path, &incoming).map_err(|err| Error::io(&incoming, err))?;
        for suffix in ["", "-wal", "-shm"] {
            let from = with_suffix(db, suffix);
            if from.exists() {
                let name = from.file_name().unwrap_or_default();
                let to = saved.join(name);
                std::fs::rename(&from, &to).map_err(|err| Error::io(&from, err))?;
            }
        }
        std::fs::rename(&incoming, db).map_err(|err| Error::io(db, err))?;
    }
    Ok(saved)
}

/// `path` with `suffix` added to its file name, as SQLite names its WAL.
fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(suffix);
    PathBuf::from(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::backup_dir;

    fn backup(paths: &Paths, db: &str, version: u32, at: i64, text: &str) {
        let dir = backup_dir(&paths.mail_db());
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(format!("{db}-v{version}-{at}.db")), text).unwrap();
    }

    #[test]
    fn copies_of_one_update_form_one_point() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        backup(&paths, "mail", 14, 1000, "m14");
        backup(&paths, "pim", 17, 1003, "p17");
        backup(&paths, "mail", 13, 500, "m13");
        let points = restore_points(&paths);
        assert_eq!(points.len(), 2);
        assert_eq!(points[0].made_at, 1003);
        assert_eq!(points[0].names(), ["pim.db", "mail.db"]);
        assert_eq!(points[1].made_at, 500);
        assert_eq!(points[1].names(), ["mail.db"]);
    }

    #[test]
    fn restore_keeps_what_was_there() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        std::fs::create_dir_all(paths.data_dir()).unwrap();
        std::fs::write(paths.mail_db(), "now").unwrap();
        std::fs::write(with_suffix(&paths.mail_db(), "-wal"), "wal").unwrap();
        std::fs::write(paths.pim_db(), "pim now").unwrap();
        backup(&paths, "mail", 14, 1000, "m14");
        let saved = restore(&paths, 1000, 2000).unwrap();
        assert_eq!(saved, paths.data_dir().join("before-restore-2000"));
        assert_eq!(std::fs::read_to_string(paths.mail_db()).unwrap(), "m14");
        assert!(!with_suffix(&paths.mail_db(), "-wal").exists());
        assert_eq!(
            std::fs::read_to_string(saved.join("mail.db")).unwrap(),
            "now"
        );
        assert_eq!(
            std::fs::read_to_string(saved.join("mail.db-wal")).unwrap(),
            "wal"
        );
        // Not in the point: left alone.
        assert_eq!(std::fs::read_to_string(paths.pim_db()).unwrap(), "pim now");
        assert!(matches!(
            restore(&paths, 1, 2001),
            Err(Error::NoSuchBackup(1))
        ));
    }
}
