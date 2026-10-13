// SPDX-License-Identifier: GPL-3.0-or-later

//! What AI assistants did through `katnactl mcp`, newest last, for
//! Settings > MCP server > Recently: a few lines of JSON in the state
//! folder, kept on this computer only. `katnactl` adds to it; Katna Mail
//! reads and clears it.

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::Paths;

/// Lines kept; older ones are dropped.
pub const KEPT: usize = 50;

/// One thing an assistant did.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Activity {
    /// Unix seconds.
    pub time: i64,
    /// The assistant, as it named itself ("Claude Desktop"); may be empty.
    pub client: String,
    pub kind: ActivityKind,
    /// The search, the subject read, or whom the draft is to.
    pub detail: String,
    /// The subject of a draft.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub subject: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActivityKind {
    Search,
    Read,
    Draft,
}

fn file(paths: &Paths) -> PathBuf {
    paths.state_dir().join("mcp-activity.jsonl")
}

/// What assistants did, newest first; nothing when the file is missing
/// or unreadable. Lines that don't parse are skipped.
pub fn read(paths: &Paths) -> Vec<Activity> {
    let Ok(text) = fs::read_to_string(file(paths)) else {
        return Vec::new();
    };
    let mut all: Vec<Activity> = text
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();
    all.reverse();
    all
}

/// Adds `activity`, keeping the last [`KEPT`].
pub fn append(paths: &Paths, activity: &Activity) -> std::io::Result<()> {
    let path = file(paths);
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let mut lines: Vec<String> = fs::read_to_string(&path)
        .unwrap_or_default()
        .lines()
        .map(str::to_owned)
        .collect();
    lines.push(serde_json::to_string(activity).map_err(std::io::Error::other)?);
    let start = lines.len().saturating_sub(KEPT);
    let mut out = String::new();
    for line in &lines[start..] {
        out.push_str(line);
        out.push('\n');
    }
    // Written beside it and renamed, so a reader never sees half a file.
    let tmp = path.with_extension("jsonl.tmp");
    let mut f = fs::File::create(&tmp)?;
    f.write_all(out.as_bytes())?;
    drop(f);
    fs::rename(tmp, path)
}

/// Forgets everything assistants did.
pub fn clear(paths: &Paths) -> std::io::Result<()> {
    match fs::remove_file(file(paths)) {
        Err(err) if err.kind() != std::io::ErrorKind::NotFound => Err(err),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_the_newest_and_clears() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(tmp.path());
        assert!(read(&paths).is_empty());
        for n in 0..(KEPT as i64 + 5) {
            let activity = Activity {
                time: n,
                client: "Claude Desktop".into(),
                kind: ActivityKind::Search,
                detail: format!("q{n}"),
                subject: String::new(),
            };
            append(&paths, &activity).unwrap();
        }
        let all = read(&paths);
        assert_eq!(all.len(), KEPT);
        assert_eq!(all[0].time, KEPT as i64 + 4);
        assert_eq!(all.last().unwrap().time, 5);
        clear(&paths).unwrap();
        assert!(read(&paths).is_empty());
        clear(&paths).unwrap();
    }
}
