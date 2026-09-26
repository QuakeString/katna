// SPDX-License-Identifier: GPL-3.0-or-later

//! Walks a mail directory tree and yields one entry per message file.
//!
//! Two layouts are understood, and may be mixed in one tree:
//!
//! - **Maildir** (a directory with `cur/` or `new/`): messages are the files
//!   in `cur/` and `new/`; `tmp/` is ignored. Flags come from the
//!   `:2,<letters>` suffix of the file name. Subfolders are either Maildir++
//!   directories (`.Sent`, `.Archive.2020` → `Sent`, `Archive/2020`) or plain
//!   nested directories. The root Maildir is the folder `INBOX`.
//! - **Plain tree** (no `cur/` or `new/`, as in the Enron corpus or MH): every
//!   regular file is a message and every subdirectory is a subfolder, so
//!   `maildir/allen-p/inbox/1.` is message `1.` in folder `allen-p/inbox`.
//!
//! Hidden files (`.complete`, `.mh_sequences`) are skipped, and so are files
//! next to `cur/`/`new/` in a Maildir (`dovecot-uidlist`). Entries are yielded in
//! a stable order (sorted by name, numbers compared numerically) so repeated
//! imports produce the same result.

use std::cmp::Ordering;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::Flags;

/// Folder name of a Maildir at the root of the imported tree.
pub const ROOT_MAILDIR_FOLDER: &str = "INBOX";

/// One message file found by [`Walker`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Folder path with `/` separators, relative to the imported root.
    pub folder: String,
    /// The message file.
    pub path: PathBuf,
    /// Flags from the Maildir file name (empty for plain trees and `new/`).
    pub flags: Flags,
}

/// Iterator over all message files below a root directory.
///
/// Directory read errors are yielded as `Err` items; the walk continues with
/// the next directory.
pub struct Walker {
    /// Directories still to visit, with their folder names. Popped from the
    /// end, so they are pushed in reverse order.
    pending: Vec<(PathBuf, String)>,
    /// Messages of the current directory, in reverse order.
    ready: Vec<Entry>,
}

impl Walker {
    /// Starts a walk at `root`.
    pub fn new(root: impl Into<PathBuf>) -> io::Result<Self> {
        let root = root.into();
        if !fs::metadata(&root)?.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                format!("{} is not a directory", root.display()),
            ));
        }
        let folder = if is_maildir(&root) {
            ROOT_MAILDIR_FOLDER.to_owned()
        } else {
            String::new()
        };
        Ok(Self {
            pending: vec![(root, folder)],
            ready: Vec::new(),
        })
    }

    fn visit(&mut self, dir: &Path, folder: &str) -> io::Result<()> {
        let mut children = Vec::new();
        if is_maildir(dir) {
            for sub in ["new", "cur"] {
                let sub_dir = dir.join(sub);
                if !sub_dir.is_dir() {
                    continue;
                }
                for (name, path) in sorted_entries(&sub_dir)? {
                    if is_hidden(&name) || !path.is_file() {
                        continue;
                    }
                    let flags = if sub == "cur" {
                        Flags::from_maildir_name(&name)
                    } else {
                        Flags::default()
                    };
                    self.ready.push(Entry {
                        folder: folder.to_owned(),
                        path,
                        flags,
                    });
                }
            }
            for (name, path) in sorted_entries(dir)? {
                if matches!(name.as_str(), "cur" | "new" | "tmp") || !path.is_dir() {
                    continue;
                }
                if let Some(dotted) = name.strip_prefix('.') {
                    // Maildir++ subfolder; only at the root of a Maildir++ tree.
                    if !dotted.is_empty() && is_maildir(&path) {
                        let base = if folder == ROOT_MAILDIR_FOLDER {
                            String::new()
                        } else {
                            format!("{folder}/")
                        };
                        children.push((path, format!("{base}{}", dotted.replace('.', "/"))));
                    }
                } else {
                    children.push((path, join_folder(folder, &name)));
                }
            }
        } else {
            for (name, path) in sorted_entries(dir)? {
                if is_hidden(&name) {
                    continue;
                }
                if path.is_dir() {
                    children.push((path, join_folder(folder, &name)));
                } else if path.is_file() {
                    self.ready.push(Entry {
                        folder: folder.to_owned(),
                        path,
                        flags: Flags::default(),
                    });
                }
            }
        }
        self.ready.reverse();
        children.reverse();
        self.pending.extend(children);
        Ok(())
    }
}

impl Iterator for Walker {
    type Item = io::Result<Entry>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(entry) = self.ready.pop() {
                return Some(Ok(entry));
            }
            let (dir, folder) = self.pending.pop()?;
            if let Err(err) = self.visit(&dir, &folder) {
                self.ready.clear();
                return Some(Err(io::Error::new(
                    err.kind(),
                    format!("{}: {err}", dir.display()),
                )));
            }
        }
    }
}

fn is_maildir(dir: &Path) -> bool {
    dir.join("cur").is_dir() || dir.join("new").is_dir()
}

fn is_hidden(name: &str) -> bool {
    name.starts_with('.')
}

fn join_folder(parent: &str, name: &str) -> String {
    if parent.is_empty() {
        name.to_owned()
    } else {
        format!("{parent}/{name}")
    }
}

/// Directory entries as (lossy UTF-8 name, path), in natural order.
fn sorted_entries(dir: &Path) -> io::Result<Vec<(String, PathBuf)>> {
    let mut entries = fs::read_dir(dir)?
        .map(|entry| {
            let entry = entry?;
            Ok((
                entry.file_name().to_string_lossy().into_owned(),
                entry.path(),
            ))
        })
        .collect::<io::Result<Vec<_>>>()?;
    entries.sort_by(|a, b| natural_cmp(&a.0, &b.0));
    Ok(entries)
}

/// Compares names so that `2.` sorts before `10.`.
fn natural_cmp(a: &str, b: &str) -> Ordering {
    let key = |s: &str| {
        let digits = s.bytes().take_while(u8::is_ascii_digit).count();
        (s[..digits].parse::<u64>().ok(), s[digits..].to_owned())
    };
    match (key(a), key(b)) {
        ((Some(x), rest_a), (Some(y), rest_b)) => x.cmp(&y).then_with(|| rest_a.cmp(&rest_b)),
        _ => a.cmp(b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(root: &Path, rel: &str) {
        let path = root.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, b"Subject: x\r\n\r\nbody\r\n").unwrap();
    }

    fn walk(root: &Path) -> Vec<(String, String, Flags)> {
        Walker::new(root)
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                let name = entry
                    .path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                (entry.folder, name, entry.flags)
            })
            .collect()
    }

    #[test]
    fn plain_tree_like_enron() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "allen-p/inbox/10.");
        write(dir.path(), "allen-p/inbox/2.");
        write(dir.path(), "allen-p/sent_items/1.");
        write(dir.path(), "allen-p/inbox/.mh_sequences");
        write(dir.path(), "arnold-j/deleted_items/1.");
        fs::write(dir.path().join(".complete"), b"").unwrap();

        let got = walk(dir.path());
        let got: Vec<_> = got.iter().map(|(f, n, _)| format!("{f}/{n}")).collect();
        assert_eq!(
            got,
            [
                "allen-p/inbox/2.",
                "allen-p/inbox/10.",
                "allen-p/sent_items/1.",
                "arnold-j/deleted_items/1.",
            ]
        );
    }

    #[test]
    fn maildir_plus_plus() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "new/1700000002.M1.host");
        write(dir.path(), "cur/1700000001.M1.host:2,FS");
        write(dir.path(), "tmp/1700000003.M1.host");
        write(dir.path(), "dovecot-uidlist");
        write(dir.path(), ".Archive.2020/cur/1600000000.M1.host:2,S");
        write(dir.path(), ".Sent/cur/1700000004.M1.host:2,RS");
        fs::create_dir_all(dir.path().join(".Sent/new")).unwrap();

        let got = walk(dir.path());
        let seen = Flags {
            seen: true,
            ..Flags::default()
        };
        assert_eq!(
            got,
            [
                (
                    "INBOX".into(),
                    "1700000002.M1.host".into(),
                    Flags::default()
                ),
                (
                    "INBOX".into(),
                    "1700000001.M1.host:2,FS".into(),
                    Flags {
                        flagged: true,
                        ..seen
                    }
                ),
                ("Archive/2020".into(), "1600000000.M1.host:2,S".into(), seen),
                (
                    "Sent".into(),
                    "1700000004.M1.host:2,RS".into(),
                    Flags {
                        answered: true,
                        ..seen
                    }
                ),
            ]
        );
    }

    #[test]
    fn nested_maildirs() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "work/cur/1:2,");
        write(dir.path(), "work/projects/new/2");
        let got = walk(dir.path());
        let got: Vec<_> = got.iter().map(|(f, n, _)| format!("{f}/{n}")).collect();
        assert_eq!(got, ["work/1:2,", "work/projects/2"]);
    }

    #[test]
    fn rejects_a_file_as_root() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "msg");
        assert!(Walker::new(dir.path().join("msg")).is_err());
    }

    #[test]
    fn natural_order() {
        let mut names = vec!["10.", "9.", "1.", "b", "a", "100."];
        names.sort_by(|a, b| natural_cmp(a, b));
        assert_eq!(names, ["1.", "9.", "10.", "100.", "a", "b"]);
    }
}
