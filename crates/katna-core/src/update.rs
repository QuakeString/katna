// SPDX-License-Identifier: GPL-3.0-or-later

//! In-app updates (`docs/ARCHITECTURE.md` §21.2): what kind of package
//! this build is, where its newest build is described, and whether that
//! build is newer than this one. The daemon checks and downloads; Katna
//! Mail installs, with the system's password prompt, and restarts.
//!
//! Each kind of package plugs into the same flow with its own [`Package`]:
//! the file it offers, and (in Katna Mail) how that file is installed.
//! Only the Arch package updates itself so far.

use std::collections::{BTreeMap, HashMap};

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

/// Where the Arch helper keeps a copy of the package it last installed,
/// which only root can change: the next update is a patch from it.
pub const ARCH_INSTALLED: &str = "/var/lib/katna/installed";

/// pacman's cache, which holds the builds `pacman -Syu` installed.
const ARCH_CACHE: &str = "/var/cache/pacman/pkg";

/// The Arch package's name, which starts its files' names.
const ARCH_PACKAGE: &str = "katna-git";

/// The kind of package this build came in, from `$KATNA_PACKAGE` at build
/// time (`packaging/arch/PKGBUILD` sets `arch`, `ci/windows-package.ps1`
/// `windows`, the Fedora spec `rpm`, the Nix package `nix`, and the
/// portable Linux build `linux`, which the tarball, AppImage, Flatpak and
/// Snap all carry, so which of those it is shows only at run time).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Package {
    /// The Arch Linux package (`katna-git`), from the `arch-latest`
    /// release; installed with `pacman -U` behind polkit.
    Arch,
    /// Katna Setup on Windows (`KatnaSetup.exe`), from the
    /// `windows-latest` release; installed by running the new Setup
    /// quietly. Always the full Setup: no patches yet.
    Windows,
    /// The AppImage from `linux-latest`: the new AppImage replaces the
    /// file Katna runs from (`$APPIMAGE`).
    AppImage,
    /// The `linux-latest` tarball, installed with its `install.sh`: Katna
    /// runs the new one's `install.sh` for the same folder.
    Tarball,
    /// The Fedora RPM from `linux-latest`, the Snap and the Flatpak
    /// bundle: there is no repository yet, so Katna downloads the new
    /// file and shows the command that installs it ([`update_command`]).
    Rpm,
    Snap,
    Flatpak,
    /// The Nix flake: Katna says a new build is out and shows the
    /// command; Nix builds it.
    Nix,
    /// Built from source, or a package that does not update itself yet:
    /// its own package manager, or the user, updates it.
    Other,
}

impl Package {
    /// This build's package.
    pub fn current() -> Self {
        match Self::parse(option_env!("KATNA_PACKAGE").unwrap_or_default()) {
            Self::Tarball => Self::portable(
                |name| std::env::var_os(name).is_some_and(|value| !value.is_empty()),
                std::path::Path::new("/.flatpak-info").exists(),
            ),
            package => package,
        }
    }

    fn parse(name: &str) -> Self {
        match name {
            "arch" => Self::Arch,
            "windows" => Self::Windows,
            "linux" => Self::Tarball,
            "rpm" => Self::Rpm,
            "nix" => Self::Nix,
            _ => Self::Other,
        }
    }

    /// Which of the packages the portable Linux build goes into this one
    /// runs from: `set` says whether an environment variable is set, and
    /// `flatpak_info` whether `/.flatpak-info` exists.
    fn portable(set: impl Fn(&str) -> bool, flatpak_info: bool) -> Self {
        if flatpak_info || set("FLATPAK_ID") {
            Self::Flatpak
        } else if set("SNAP") {
            Self::Snap
        } else if set("APPIMAGE") {
            Self::AppImage
        } else {
            Self::Tarball
        }
    }

    /// The name of this package's file in a `linux-latest` manifest's
    /// [`Manifest::files`].
    fn format(self) -> Option<&'static str> {
        match self {
            Self::AppImage => Some("appimage"),
            Self::Tarball => Some("tarball"),
            Self::Rpm => Some("rpm"),
            Self::Snap => Some("snap"),
            Self::Flatpak => Some("flatpak"),
            Self::Arch | Self::Windows | Self::Nix | Self::Other => None,
        }
    }

    /// Whether Katna downloads this package's new builds: Nix builds its
    /// own.
    pub fn downloads(self) -> bool {
        !matches!(self, Self::Nix | Self::Other)
    }

    /// Whether Katna installs the downloaded build itself; else it shows
    /// [`update_command`].
    pub fn installs_itself(self) -> bool {
        matches!(
            self,
            Self::Arch | Self::Windows | Self::AppImage | Self::Tarball
        )
    }

    /// The release this package's newest build is published in, if it
    /// updates itself.
    pub fn release(self) -> Option<&'static str> {
        match self {
            Self::Arch => Some("arch-latest"),
            Self::Windows => Some("windows-latest"),
            Self::AppImage | Self::Tarball | Self::Rpm | Self::Snap | Self::Flatpak | Self::Nix => {
                Some("linux-latest")
            }
            Self::Other => None,
        }
    }

    /// The URL of the [`Manifest`] of this package's newest build.
    pub fn manifest_url(self) -> Option<String> {
        self.release()
            .map(|release| format!("{RELEASES}/{release}/{MANIFEST_FILE}"))
    }

    /// The folders, writable only by root, that may hold the installed
    /// build's package, to patch the next one from: the copy the update
    /// helper keeps, and pacman's cache after `pacman -Syu`.
    pub fn installed_dirs(self) -> &'static [&'static str] {
        match self {
            Self::Arch => &[ARCH_INSTALLED, ARCH_CACHE],
            _ => &[],
        }
    }

    /// The URL of `file`, a file of this package's newest build.
    pub fn file_url(self, file: &str) -> Option<String> {
        self.release()
            .map(|release| format!("{RELEASES}/{release}/{file}"))
    }
}

/// The command that installs the new build for a package Katna does not
/// install itself: `file` is the downloaded build, unused for Nix.
pub fn update_command(package: Package, file: &str) -> Option<String> {
    let file = shell_quote(file);
    match package {
        Package::Rpm => Some(format!("sudo dnf install {file}")),
        Package::Snap => Some(format!("sudo snap install --dangerous {file}")),
        Package::Flatpak => Some(format!("flatpak install --user --reinstall -y {file}")),
        Package::Nix => Some("nix profile upgrade katna".to_owned()),
        _ => None,
    }
}

/// `text` as one word for a POSIX shell.
pub fn shell_quote(text: &str) -> String {
    if !text.is_empty()
        && text
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "/._-+=:,@".contains(c))
    {
        text.to_owned()
    } else {
        format!("'{}'", text.replace('\'', r"'\''"))
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
    /// The file's minisign signature (the text of a `.minisig` file), by
    /// the update signing key (`packaging/keys/`). Saved beside the
    /// download, where the root update helper checks it once the package
    /// carries the key. `None` from builds before signing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minisig: Option<String>,
    /// The package without its zstd compression: what a patch makes.
    /// `None` from builds before patches.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tar: Option<Tar>,
    /// Patches from a few earlier builds to [`Manifest::tar`], so an
    /// update downloads only what changed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patches: Vec<Patch>,
    /// Patches between earlier builds, carried over from their own
    /// manifests: an installed build without a patch in
    /// [`Manifest::patches`] reaches this one through two or three of
    /// them ([`Manifest::route`]).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chain: Vec<Hop>,
    /// In `linux-latest`'s manifest: each package's own file, by
    /// [`Package`] format (`appimage`, `tarball`, `rpm`, `snap`,
    /// `flatpak`); [`Manifest::for_package`] picks one.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub files: BTreeMap<String, Download>,
}

/// A file of a build: name, SHA-256 and size.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Download {
    pub file: String,
    pub sha256: String,
    pub size: u64,
}

/// A build's package without its zstd compression (`….pkg.tar`), which
/// pacman installs too.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tar {
    pub sha256: String,
    pub size: u64,
    /// Its own signature, as [`Manifest::minisig`] for the package.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minisig: Option<String>,
}

/// A patch that turns an earlier build's [`Tar`] into this one's: `zstd
/// --patch-from` with the earlier one as the reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Patch {
    /// The earlier build's version.
    pub from: String,
    /// The patch's file name in the same release.
    pub file: String,
    pub sha256: String,
    pub size: u64,
}

/// Most patches a manifest lists.
const MAX_PATCHES: usize = 10;

/// A patch from one earlier build's [`Tar`] to another's, as that
/// build's manifest listed it in [`Manifest::patches`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hop {
    pub from: String,
    pub to: String,
    /// The patch's file name in the same release.
    pub file: String,
    pub sha256: String,
    pub size: u64,
    /// When `to`'s commit was made (Unix seconds), which CI uses to
    /// drop old hops; 0 when unknown.
    #[serde(default)]
    pub built: i64,
}

/// Most hops a manifest lists: patches from the last 48 builds, seven
/// each.
const MAX_CHAIN: usize = 400;
/// Most patches an update is made from, one after another.
const MAX_HOPS: usize = 4;

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
        manifest.patches.truncate(MAX_PATCHES);
        let tar = manifest.tar.as_ref().is_none_or(|tar| {
            is_sha256(&tar.sha256) && is_size(tar.size) && is_minisig(tar.minisig.as_deref())
        });
        // A patch that does not look right is left out; the full
        // package stays.
        manifest.patches.retain(|patch| {
            is_plain_name(&patch.file)
                && is_sha256(&patch.sha256)
                && is_size(patch.size)
                && parse_version(&patch.from).is_some()
        });
        manifest.chain.truncate(MAX_CHAIN);
        manifest.chain.retain(|hop| {
            is_plain_name(&hop.file)
                && is_sha256(&hop.sha256)
                && is_size(hop.size)
                && parse_version(&hop.from).is_some()
                && parse_version(&hop.to).is_some()
        });
        if manifest.tar.is_none() {
            manifest.patches.clear();
            manifest.chain.clear();
        }
        manifest.files.retain(|_, download| {
            is_plain_name(&download.file) && is_sha256(&download.sha256) && is_size(download.size)
        });
        (is_plain_name(&manifest.file)
            && is_sha256(&manifest.sha256)
            && is_size(manifest.size)
            && is_minisig(manifest.minisig.as_deref())
            && tar
            && parse_version(&manifest.version).is_some())
        .then_some(manifest)
    }

    /// The patch from `installed` to this build, if one is published.
    pub fn patch_from(&self, installed: &str) -> Option<&Patch> {
        self.tar.as_ref()?;
        self.patches.iter().find(|patch| patch.from == installed)
    }

    /// This manifest as `package` downloads it: from `linux-latest`'s,
    /// its own file in place of the tarball. `None` when the build has no
    /// file for it.
    pub fn for_package(mut self, package: Package) -> Option<Self> {
        if let Some(format) = package.format()
            && !self.files.is_empty()
        {
            let download = self.files.remove(format)?;
            self.file = download.file;
            self.sha256 = download.sha256;
            self.size = download.size;
            self.minisig = None;
            self.tar = None;
            self.patches.clear();
            self.chain.clear();
        }
        Some(self)
    }

    /// The patches that make this build from `installed`, in the order
    /// they apply, downloading the fewest bytes: a direct patch when
    /// there is one, else up to [`MAX_HOPS`] through earlier builds,
    /// ending with one of [`Manifest::patches`]. `None` when there are
    /// none, or when together they are not clearly smaller than the full
    /// package.
    pub fn route(&self, installed: &str) -> Option<Vec<Hop>> {
        self.tar.as_ref()?;
        let direct = self.patches.iter().map(|patch| Hop {
            from: patch.from.clone(),
            to: self.version.clone(),
            file: patch.file.clone(),
            sha256: patch.sha256.clone(),
            size: patch.size,
            built: self.built,
        });
        let hops: Vec<Hop> = self
            .chain
            .iter()
            .filter(|hop| hop.to != self.version && hop.from != hop.to)
            .cloned()
            .chain(direct)
            .collect();
        // Fewest bytes to each build in at most `round` hops
        // (Bellman-Ford, a few hundred hops at most).
        let mut best: HashMap<&str, (u64, Vec<usize>)> = HashMap::new();
        best.insert(installed, (0, Vec::new()));
        for _round in 0..MAX_HOPS {
            let mut next = best.clone();
            for (i, hop) in hops.iter().enumerate() {
                let Some((bytes, path)) = best.get(hop.from.as_str()) else {
                    continue;
                };
                let bytes = bytes.saturating_add(hop.size);
                if next.get(hop.to.as_str()).is_none_or(|(b, _)| bytes < *b) {
                    let mut path = path.clone();
                    path.push(i);
                    next.insert(&hop.to, (bytes, path));
                }
            }
            best = next;
        }
        let (bytes, path) = best.remove(self.version.as_str())?;
        // Each patch costs a moment to apply and can fail; past this
        // share of the full package, the full one is the better buy.
        (!path.is_empty() && bytes.saturating_mul(10) <= self.size.saturating_mul(7))
            .then(|| path.into_iter().map(|i| hops[i].clone()).collect())
    }

    /// The name the uncompressed package is saved under: the package's
    /// without `.zst`.
    pub fn tar_file(&self) -> String {
        self.file
            .strip_suffix(".zst")
            .unwrap_or(&self.file)
            .to_owned()
    }

    /// Whether this build is newer than `installed`.
    pub fn newer_than(&self, installed: &str) -> bool {
        // A Nix build from GitHub cannot count its commits (`r0`): newer
        // when its commit is among this build's earlier ones.
        if parse_version(installed).is_some_and(|(_, commits)| commits == 0)
            && commit_of(installed).is_some()
        {
            let (since, found) = self.changes_since(installed);
            return found && !since.is_empty();
        }
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

fn is_plain_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | '+'))
        && !name.starts_with('.')
}

fn is_sha256(sha: &str) -> bool {
    sha.len() == 64
        && sha
            .chars()
            .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
}

fn is_size(size: u64) -> bool {
    (1..=MAX_SIZE).contains(&size)
}

/// A few short lines of printable text.
fn is_minisig(sig: Option<&str>) -> bool {
    sig.is_none_or(|sig| {
        sig.len() <= 1024
            && sig
                .chars()
                .all(|c| c == '\n' || c.is_ascii_graphic() || c == ' ')
    })
}

/// The version in the name of one of this package's files, as pacman
/// names them: `katna-git-<version>-<release>-<arch>.pkg.tar[.zst]`.
pub fn package_version(file_name: &str) -> Option<&str> {
    let rest = file_name.strip_prefix(ARCH_PACKAGE)?.strip_prefix('-')?;
    let rest = rest
        .strip_suffix(".pkg.tar.zst")
        .or_else(|| rest.strip_suffix(".pkg.tar"))?;
    let (rest, _arch) = rest.rsplit_once('-')?;
    let (version, release) = rest.rsplit_once('-')?;
    (!release.is_empty()
        && release.bytes().all(|b| b.is_ascii_digit() || b == b'.')
        && parse_version(version).is_some())
    .then_some(version)
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
    fn a_build_without_a_commit_count_goes_by_its_commit() {
        let manifest = Manifest::parse(
            br#"{"version":"0.0.0.r700.gccccccc","file":"k.tar.gz",
            "sha256":"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08","size":10,
            "changes":[{"commit":"ccccccc","title":"c"},{"commit":"bbbbbbb","title":"b"}]}"#,
        )
        .unwrap();
        assert!(manifest.newer_than("0.0.0.r0.gbbbbbbb"));
        assert!(!manifest.newer_than("0.0.0.r0.gccccccc"));
        assert!(
            !manifest.newer_than("0.0.0.r0.gddddddd"),
            "not in the list: unknown"
        );
        assert!(manifest.newer_than("0.0.0.r699.gbbbbbbb"));
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
        // Signed builds carry their signature; old ones have none.
        assert_eq!(manifest.minisig, None);
        let sig =
            "untrusted comment: signature\nRUQf6LRCGA9i5+Q=\ntrusted comment: katna-git\nabc=\n";
        let signed = bad("\"size\"", &format!("\"minisig\":{sig:?},\"size\""));
        assert_eq!(signed.unwrap().minisig.as_deref(), Some(sig));
        assert!(bad("\"size\"", "\"minisig\":\"a\\u0000b\",\"size\"").is_none());
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
    fn reads_patches() {
        let json = br#"{"version":"0.0.0.r543.gccccccc","file":"katna-git-0.0.0.r543.gccccccc-1-x86_64.pkg.tar.zst",
            "sha256":"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08","size":10,
            "tar":{"sha256":"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08","size":30},
            "patches":[{"from":"0.0.0.r542.gbbbbbbb","file":"katna-git-0.0.0.r542.gbbbbbbb-to-0.0.0.r543.gccccccc.patch.zst",
                "sha256":"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08","size":3},
                {"from":"0.0.0.r541.gaaaaaaa","file":"../evil","sha256":"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08","size":3}]}"#;
        let manifest = Manifest::parse(json).unwrap();
        assert_eq!(manifest.patches.len(), 1, "the bad patch is left out");
        assert_eq!(manifest.patch_from("0.0.0.r542.gbbbbbbb").unwrap().size, 3);
        assert!(manifest.patch_from("0.0.0.r541.gaaaaaaa").is_none());
        assert_eq!(
            manifest.tar_file(),
            "katna-git-0.0.0.r543.gccccccc-1-x86_64.pkg.tar"
        );
        // Without the uncompressed package's checksum, no patch is used.
        let text = std::str::from_utf8(json).unwrap();
        let start = text.find("\"tar\"").unwrap();
        let end = text.find("\"patches\"").unwrap();
        let no_tar = format!("{}{}", &text[..start], &text[end..]);
        assert!(
            Manifest::parse(no_tar.as_bytes())
                .unwrap()
                .patches
                .is_empty()
        );
    }

    #[test]
    fn routes_through_earlier_builds() {
        let sha = "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08";
        let v = |r: u32| format!("0.0.0.r{r}.g{r:07}");
        let hop = |from: u32, to: u32, size: u64| Hop {
            from: v(from),
            to: v(to),
            file: format!("katna-git-{}-to-{}.patch.zst", v(from), v(to)),
            sha256: sha.into(),
            size,
            built: 0,
        };
        let patch = |from: u32, size: u64| Patch {
            from: v(from),
            file: format!("katna-git-{}-to-{}.patch.zst", v(from), v(20)),
            sha256: sha.into(),
            size,
        };
        let manifest = Manifest {
            version: v(20),
            file: format!("katna-git-{}-1-x86_64.pkg.tar.zst", v(20)),
            sha256: sha.into(),
            size: 100,
            built: 0,
            commit: String::new(),
            highlights: vec![],
            changes: vec![],
            minisig: None,
            tar: Some(Tar {
                sha256: sha.into(),
                size: 300,
                minisig: None,
            }),
            patches: vec![patch(19, 5), patch(18, 6), patch(14, 9), patch(2, 80)],
            chain: vec![
                hop(10, 14, 8),
                hop(10, 12, 4),
                hop(12, 14, 3),
                hop(8, 10, 1),
                hop(5, 8, 1),
                hop(4, 5, 1),
            ],
            files: BTreeMap::new(),
        };
        let json = serde_json::to_vec(&manifest).unwrap();
        let manifest = Manifest::parse(&json).unwrap();
        let route = |from: u32| {
            manifest.route(&v(from)).map(|hops| {
                hops.iter()
                    .map(|hop| (hop.from.clone(), hop.to.clone()))
                    .collect::<Vec<_>>()
            })
        };
        // A direct patch.
        assert_eq!(route(19), Some(vec![(v(19), v(20))]));
        // The fewest bytes: 4 + 3 + 9, not 8 + 9.
        assert_eq!(
            route(10),
            Some(vec![(v(10), v(12)), (v(12), v(14)), (v(14), v(20))])
        );
        assert_eq!(route(8).unwrap().len(), 4);
        assert_eq!(route(5).unwrap().len(), 4);
        // Five hops are too many.
        assert_eq!(route(4), None);
        // More than 70% of the full package.
        assert_eq!(route(2), None);
        assert_eq!(route(3), None);
        assert_eq!(route(20), None);
    }

    #[test]
    fn versions_in_package_names() {
        assert_eq!(
            package_version("katna-git-0.0.0.r542.g39ff4f0-1-x86_64.pkg.tar.zst"),
            Some("0.0.0.r542.g39ff4f0")
        );
        assert_eq!(
            package_version("katna-git-0.1.0-2-aarch64.pkg.tar"),
            Some("0.1.0")
        );
        assert_eq!(package_version("katna-git-x86_64.pkg.tar.zst"), None);
        assert_eq!(
            package_version("katna-git-0.1.0-1-x86_64.pkg.tar.zst.minisig"),
            None
        );
        assert_eq!(package_version("other-0.1.0-1-x86_64.pkg.tar.zst"), None);
    }

    #[test]
    fn the_portable_build_knows_its_package() {
        let env = |names: &'static [&'static str]| move |name: &str| names.contains(&name);
        assert_eq!(Package::portable(env(&[]), false), Package::Tarball);
        assert_eq!(
            Package::portable(env(&["APPIMAGE"]), false),
            Package::AppImage
        );
        assert_eq!(Package::portable(env(&["SNAP"]), false), Package::Snap);
        assert_eq!(Package::portable(env(&[]), true), Package::Flatpak);
        assert_eq!(
            Package::portable(env(&["FLATPAK_ID"]), false),
            Package::Flatpak
        );
        assert_eq!(Package::parse("linux"), Package::Tarball);
        assert_eq!(Package::parse("rpm"), Package::Rpm);
        assert_eq!(Package::parse("nix"), Package::Nix);
        assert_eq!(
            Package::Snap.manifest_url().unwrap(),
            format!("{RELEASES}/linux-latest/{MANIFEST_FILE}")
        );
        assert!(!Package::Nix.downloads() && Package::Rpm.downloads());
        assert!(Package::AppImage.installs_itself() && !Package::Flatpak.installs_itself());
    }

    #[test]
    fn each_linux_package_gets_its_own_file() {
        let sha = "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08";
        let json = format!(
            r#"{{"version":"0.0.0.r9.g1234567","file":"katna-linux-x86_64.tar.gz","sha256":"{sha}","size":10,
            "files":{{"tarball":{{"file":"katna-linux-x86_64.tar.gz","sha256":"{sha}","size":10}},
                      "appimage":{{"file":"Katna-x86_64.AppImage","sha256":"{sha}","size":20}},
                      "snap":{{"file":"../evil","sha256":"{sha}","size":20}}}}}}"#
        );
        let manifest = Manifest::parse(json.as_bytes()).unwrap();
        let appimage = manifest.clone().for_package(Package::AppImage).unwrap();
        assert_eq!(
            (appimage.file.as_str(), appimage.size),
            ("Katna-x86_64.AppImage", 20)
        );
        assert!(
            manifest.clone().for_package(Package::Snap).is_none(),
            "a bad name is left out"
        );
        assert!(manifest.clone().for_package(Package::Rpm).is_none());
        assert_eq!(manifest.clone().for_package(Package::Nix).unwrap().size, 10);
        assert_eq!(
            update_command(
                Package::Rpm,
                "/home/me/.cache/katna/updates/katna-x86_64.rpm"
            )
            .unwrap(),
            "sudo dnf install /home/me/.cache/katna/updates/katna-x86_64.rpm"
        );
        assert_eq!(
            update_command(Package::Snap, "/home/o'neil/k a.snap").unwrap(),
            r"sudo snap install --dangerous '/home/o'\''neil/k a.snap'"
        );
        assert_eq!(update_command(Package::AppImage, "x"), None);
    }

    #[test]
    fn packages_and_their_urls() {
        assert_eq!(Package::parse("arch"), Package::Arch);
        assert_eq!(Package::parse("windows"), Package::Windows);
        assert_eq!(
            Package::Windows.manifest_url().unwrap(),
            format!("{RELEASES}/windows-latest/{MANIFEST_FILE}")
        );
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
