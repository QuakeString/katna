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
}

/// Largest package accepted, far above Katna's size budgets.
pub const MAX_SIZE: u64 = 512 * 1024 * 1024;

impl Manifest {
    /// Reads a manifest, refusing one that could not be a real build: a
    /// file name that is not a plain name, a checksum that is not SHA-256,
    /// or a size of nothing or too much.
    pub fn parse(json: &[u8]) -> Option<Self> {
        let manifest: Self = serde_json::from_slice(json).ok()?;
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
