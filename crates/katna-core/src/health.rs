// SPDX-License-Identifier: GPL-3.0-or-later

//! The daemon's health file, `$XDG_STATE_HOME/katna/health.toml`
//! (`docs/ARCHITECTURE.md` §21.2, "After an update").
//!
//! Each start of `katna-daemon` is recorded before it opens anything and
//! cleared once the start reaches "healthy". Starts that never got there
//! pile up; [`FAILED_STARTS`] of them within [`START_WINDOW`] seconds mean
//! the next start should be in safe mode. The first start of a new version
//! also records the result of its self-check, which the apps read for the
//! debug report.

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Failed starts that put the next start in safe mode.
pub const FAILED_STARTS: usize = 3;

/// How far back failed starts count, in seconds.
pub const START_WINDOW: i64 = 10 * 60;

/// What `health.toml` holds.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Health {
    /// The version that last reached "healthy" or ran the self-check.
    pub version: String,
    /// When the self-check last ran, in Unix seconds.
    pub checked_at: i64,
    /// Whether the last self-check passed.
    pub healthy: bool,
    /// Each self-check item: "ok" or what went wrong.
    pub checks: BTreeMap<String, String>,
    /// Starts that have not reached "healthy" yet, in Unix seconds. The last
    /// one is the start running now, if any.
    pub starts: Vec<i64>,
    /// Whether the start running now is in safe mode.
    pub safe_mode: bool,
}

impl Health {
    /// Reads `path`. A missing or unreadable file gives an empty record, so
    /// a broken health file never stops the daemon.
    pub fn load(path: &Path) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|text| toml::from_str(&text).ok())
            .unwrap_or_default()
    }

    /// Writes the record to `path` through a temporary file.
    pub fn save(&self, path: &Path) -> Result<()> {
        let text = toml::to_string_pretty(self)?;
        let dir = path
            .parent()
            .filter(|dir| !dir.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        fs::create_dir_all(dir).map_err(|err| Error::io(dir, err))?;
        let tmp = dir.join(format!(".health.toml.{}.tmp", std::process::id()));
        let written = fs::File::create(&tmp)
            .and_then(|mut file| {
                file.write_all(text.as_bytes())?;
                file.sync_all()
            })
            .and_then(|()| fs::rename(&tmp, path));
        written.map_err(|err: io::Error| {
            let _ = fs::remove_file(&tmp);
            Error::io(path, err)
        })
    }

    /// Records a start at `now` and decides whether it runs in safe mode:
    /// [`FAILED_STARTS`] earlier starts within [`START_WINDOW`] never became
    /// healthy.
    pub fn begin_start(&mut self, now: i64) -> bool {
        self.starts
            .retain(|&at| now - at < START_WINDOW && at <= now);
        self.safe_mode = self.starts.len() >= FAILED_STARTS;
        self.starts.push(now);
        self.safe_mode
    }

    /// Whether `version` should run the self-check: a new version, or the
    /// last check failed.
    pub fn needs_check(&self, version: &str) -> bool {
        self.version != version || !self.healthy
    }

    /// Records a self-check of `version` at `now`.
    pub fn checked(&mut self, version: &str, now: i64, checks: BTreeMap<String, String>) {
        self.version = version.to_owned();
        self.checked_at = now;
        self.healthy = checks.values().all(|result| result == OK);
        self.checks = checks;
    }

    /// The start reached "healthy": it no longer counts as failed.
    pub fn reached_healthy(&mut self) {
        self.starts.clear();
    }
}

/// A passing self-check item.
pub const OK: &str = "ok";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_failed_starts_in_ten_minutes_mean_safe_mode() {
        let mut health = Health::default();
        assert!(!health.begin_start(1000));
        assert!(!health.begin_start(1010));
        assert!(!health.begin_start(1020));
        assert!(health.begin_start(1030));
        assert!(health.safe_mode);
        health.reached_healthy();
        assert!(!health.begin_start(1040));
    }

    #[test]
    fn old_failed_starts_stop_counting() {
        let mut health = Health::default();
        for at in [0, 10, 20] {
            health.begin_start(at);
        }
        assert!(!health.begin_start(20 + START_WINDOW));
        assert_eq!(health.starts.len(), 1);
    }

    #[test]
    fn a_new_version_or_a_failed_check_checks_again() {
        let mut health = Health::default();
        assert!(health.needs_check("r1"));
        let ok = BTreeMap::from([("mail.db".to_owned(), OK.to_owned())]);
        health.checked("r1", 5, ok);
        assert!(!health.needs_check("r1"));
        assert!(health.needs_check("r2"));
        let bad = BTreeMap::from([("mail.db".to_owned(), "malformed".to_owned())]);
        health.checked("r2", 6, bad);
        assert!(!health.healthy);
        assert!(health.needs_check("r2"));
    }

    #[test]
    fn saves_and_loads_and_survives_a_broken_file() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("state/health.toml");
        assert_eq!(Health::load(&path), Health::default());
        let mut health = Health::default();
        health.begin_start(7);
        health.checked(
            "r1",
            8,
            BTreeMap::from([("index".to_owned(), OK.to_owned())]),
        );
        health.save(&path).unwrap();
        assert_eq!(Health::load(&path), health);
        fs::write(&path, "not toml [").unwrap();
        assert_eq!(Health::load(&path), Health::default());
    }
}
