// SPDX-License-Identifier: GPL-3.0-or-later

//! In-app updates (`docs/ARCHITECTURE.md` §21.2): what kind of package
//! this build is, where its newest build is described, and whether that
//! build is newer than this one. The daemon checks and downloads; Katna
//! Mail installs, with the system's password prompt, and restarts.
//!
//! Each kind of package plugs into the same flow with its own [`Package`]:
//! the file it offers, and (in Katna Mail) how that file is installed.
//! Only the Arch package updates itself so far.

use serde::{Deserialize, Serialize};

/// The version this build reports ([`crate::crash::VERSION`]).
pub use crate::crash::VERSION;

/// Where the builds of each package are published: the repository's
/// GitHub releases.
const RELEASES: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "/releases/download");

/// The file beside each build that describes it.
pub const MANIFEST_FILE: &str = "katna-update.json";

/// The Arch package's program that installs a downloaded update as root,
/// through `pkexec` (`packaging/arch/katna-update-helper`). Its path is
/// the one the polkit action ([`crate::ids::UPDATE_ACTION`]) allows.
pub const ARCH_HELPER: &str = "/usr/lib/katna/katna-update-helper";

/// The kind of package this build came in, from `$KATNA_PACKAGE` at build
/// time (`packaging/arch/PKGBUILD` sets `arch`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Package {
    /// The Arch Linux package (`katna-git`), from the `arch-latest`
    /// release; installed with `pacman -U` behind polkit.
    Arch,
    /// Built from source, or a package that does not update itself yet:
    /// its own package manager, or the user, updates it.
    Other,
}

impl Package {
    /// This build's package.
    pub fn current() -> Self {
        Self::parse(option_env!("KATNA_PACKAGE").unwrap_or_default())
    }

    fn parse(name: &str) -> Self {
        match name {
            "arch" => Self::Arch,
            _ => Self::Other,
        }
    }

    /// The release this package's newest build is published in, if it
    /// updates itself.
    pub fn release(self) -> Option<&'static str> {
        match self {
            Self::Arch => Some("arch-latest"),
            Self::Other => None,
        }
    }

    /// The URL of the [`Manifest`] of this package's newest build.
    pub fn manifest_url(self) -> Option<String> {
        self.release()
            .map(|release| format!("{RELEASES}/{release}/{MANIFEST_FILE}"))
    }

    /// The URL of `file`, a file of this package's newest build.
    pub fn file_url(self, file: &str) -> Option<String> {
        self.release()
            .map(|release| format!("{RELEASES}/{release}/{file}"))
    }
}

/// What `katna-update.json` says about the newest build: CI writes it
/// beside the package on every build of `main`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    /// As [`VERSION`] names it, such as `0.0.0.r236.g1a2b3c4`.
    pub version: String,
    /// The package's file name in the same release.
    pub file: String,
    /// The file's SHA-256, in lowercase hex.
    pub sha256: String,
    /// The file's size in bytes.
    pub size: u64,
    /// When the build's commit was made (Unix seconds), or 0 in a
    /// manifest from before it was written.
    #[serde(default)]
    pub built: i64,
    /// The build's commit, in full, or empty.
    #[serde(default)]
    pub commit: String,
    /// The newest What's new highlights, newest first, in English: the
    /// Update dialog shows those the installed version does not have.
    #[serde(default)]
    pub highlights: Vec<NewHighlight>,
    /// The newest commits on `main`, newest first, down to at most
    /// [`MAX_CHANGES`]: the Update dialog lists those after the installed
    /// version's commit.
    #[serde(default)]
    pub changes: Vec<Change>,
}

/// A What's new highlight of a build, as `katna-mail --highlights`
/// prints it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewHighlight {
    /// The file's name without `.toml`, such as `2026-09-28-2059-slug`.
    pub name: String,
    pub title: String,
    pub text: String,
}

/// A commit of a build.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Change {
    /// The commit's hash, at least its first seven digits.
    pub commit: String,
    /// The commit's first line.
    pub title: String,
}

/// Most commits a manifest lists.
pub const MAX_CHANGES: usize = 200;
/// Most highlights a manifest lists.
const MAX_HIGHLIGHTS: usize = 50;

/// Largest package accepted, far above Katna's size budgets.
pub const MAX_SIZE: u64 = 512 * 1024 * 1024;

impl Manifest {
    /// Reads a manifest, refusing one that could not be a real build: a
    /// file name that is not a plain name, a checksum that is not SHA-256,
    /// or a size of nothing or too much.
    pub fn parse(json: &[u8]) -> Option<Self> {
        let mut manifest: Self = serde_json::from_slice(json).ok()?;
        manifest.changes.truncate(MAX_CHANGES);
        manifest.highlights.truncate(MAX_HIGHLIGHTS);
        let plain_name = !manifest.file.is_empty()
            && manifest
                .file
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | '+'))
            && !manifest.file.starts_with('.');
        let sha = manifest.sha256.len() == 64
            && manifest
                .sha256
                .chars()
                .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c));
        let size = (1..=MAX_SIZE).contains(&manifest.size);
        (plain_name && sha && size && parse_version(&manifest.version).is_some())
            .then_some(manifest)
    }

    /// Whether this build is newer than `installed`.
    pub fn newer_than(&self, installed: &str) -> bool {
        newer(installed, &self.version)
    }

    /// The commits after `installed`, a version naming its commit, newest
    /// first, and whether that is all of them: `false` when the installed
    /// commit is older than the list, or unknown.
    pub fn changes_since(&self, installed: &str) -> (&[Change], bool) {
        let at = commit_of(installed).and_then(|commit| {
            self.changes
                .iter()
                .position(|c| c.commit.starts_with(commit) || commit.starts_with(&c.commit))
        });
        match at {
            Some(at) => (&self.changes[..at], true),
            None => (&self.changes, false),
        }
    }
}

/// The commit a package's version names (`…gHASH`).
pub fn commit_of(version: &str) -> Option<&str> {
    let (_, hash) = version.rsplit_once(".g")?;
    (hash.len() >= 7 && hash.bytes().all(|b| b.is_ascii_hexdigit())).then_some(hash)
}

/// A version as the packages write it: `X.Y.Z.rN.gHASH` (N commits after
/// the tag `vX.Y.Z`), or plain `X.Y.Z` for a build of a tag.
fn parse_version(version: &str) -> Option<([u64; 3], u64)> {
    let mut parts = version.split('.');
    let mut numbers = [0; 3];
    for number in &mut numbers {
        *number = parts.next()?.parse().ok()?;
    }
    let commits = match parts.next() {
        None => 0,
        Some(r) => r.strip_prefix('r')?.parse().ok()?,
    };
    Some((numbers, commits))
}

/// Whether `offered` is a later build than `installed`. A version that
/// is not a package's (a build from source) is never updated.
pub fn newer(installed: &str, offered: &str) -> bool {
    match (parse_version(installed), parse_version(offered)) {
        (Some(installed), Some(offered)) => offered > installed,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn later_builds_are_newer() {
        assert!(newer("0.0.0.r235.gfd13ac6", "0.0.0.r236.g1a2b3c4"));
        assert!(!newer("0.0.0.r236.g1a2b3c4", "0.0.0.r236.g1a2b3c4"));
        assert!(!newer("0.0.0.r236.g1a2b3c4", "0.0.0.r235.gfd13ac6"));
        assert!(newer("0.0.0.r900.gaaaaaaa", "0.1.0.r1.gbbbbbbb"));
        assert!(!newer("0.1.0.r3.gaaaaaaa", "0.1.0"));
        assert!(!newer("0.1.0", "0.0.9.r50.gaaaaaaa"));
        // Built from source: the crate's own version, never updated.
        assert!(!newer("dev", "0.0.0.r236.g1a2b3c4"));
        assert!(!newer("0.0.0.r1.gaaaaaaa", "latest"));
    }

    #[test]
    fn reads_a_manifest() {
        let json = br#"{"version":"0.0.0.r236.g1a2b3c4","file":"katna-git-x86_64.pkg.tar.zst",
            "sha256":"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
            "size":31457280}"#;
        let manifest = Manifest::parse(json).unwrap();
        assert!(manifest.newer_than("0.0.0.r235.gfd13ac6"));
        let bad = |from: &str, to: &str| {
            let text = std::str::from_utf8(json).unwrap().replace(from, to);
            Manifest::parse(text.as_bytes())
        };
        assert!(bad("katna-git-x86_64", "../etc/passwd").is_none());
        assert!(bad("9f86d0", "zz86d0").is_none());
        assert!(bad("31457280", "0").is_none());
        assert!(bad("0.0.0.r236.g1a2b3c4", "soon").is_none());
    }

    #[test]
    fn reads_what_changed() {
        let json = br#"{"version":"0.0.0.r3.gccccccc","file":"k.pkg.tar.zst",
            "sha256":"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
            "size":10,"built":1790000000,"commit":"cccccccdddd",
            "highlights":[{"name":"2026-09-28-2100-x","title":"X","text":"Y"}],
            "changes":[{"commit":"ccccccc","title":"c"},{"commit":"bbbbbbb","title":"b"},
                {"commit":"aaaaaaa","title":"a"}]}"#;
        let manifest = Manifest::parse(json).unwrap();
        assert_eq!(manifest.built, 1_790_000_000);
        assert_eq!(manifest.highlights[0].title, "X");
        fn titles((changes, all): (&[Change], bool)) -> (Vec<&str>, bool) {
            (changes.iter().map(|c| c.title.as_str()).collect(), all)
        }
        assert_eq!(
            titles(manifest.changes_since("0.0.0.r1.gaaaaaaa")),
            (vec!["c", "b"], true)
        );
        assert_eq!(
            titles(manifest.changes_since("0.0.0.r3.gccccccc")),
            (vec![], true)
        );
        assert_eq!(
            titles(manifest.changes_since("0.0.0.r0.g0123456")).0.len(),
            3
        );
        assert!(!manifest.changes_since("dev").1);
        // A manifest from before these were written still reads.
        let old = br#"{"version":"0.0.0.r3.gccccccc","file":"k.pkg.tar.zst",
            "sha256":"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
            "size":10}"#;
        let old = Manifest::parse(old).unwrap();
        assert!(old.changes.is_empty() && old.built == 0);
    }

    #[test]
    fn packages_and_their_urls() {
        assert_eq!(Package::parse("arch"), Package::Arch);
        assert_eq!(Package::parse(""), Package::Other);
        assert_eq!(Package::Other.manifest_url(), None);
        let url = Package::Arch.manifest_url().unwrap();
        assert!(url.starts_with("https://github.com/"), "{url}");
        assert!(
            url.ends_with("/releases/download/arch-latest/katna-update.json"),
            "{url}"
        );
    }
}
