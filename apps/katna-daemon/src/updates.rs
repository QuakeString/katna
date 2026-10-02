// SPDX-License-Identifier: GPL-3.0-or-later

//! In-app updates (`docs/ARCHITECTURE.md` §21.2): the daemon looks for a
//! newer build of its package now and then, downloads it (by itself when
//! `updates.auto_download` is on, never on a metered connection), checks
//! it against the SHA-256 its manifest names, and shows a notification
//! with an Update button. Katna Mail installs the file and restarts; the
//! daemon restarts itself once its binary is replaced ([`crate::update`]).
//!
//! When the manifest has a patch from the installed build and the root
//! helper kept a copy of that build ([`Package::installed_dirs`]), the
//! daemon downloads only the patch and makes the new package from the
//! two, checked against the manifest just as a full download is. If
//! anything about that fails, it downloads the full package.
//!
//! Packages that do not update themselves ([`Package::Other`]) are never
//! checked.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, Weak};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use async_channel::{Receiver, Sender};
use futures_lite::FutureExt;
use katna_core::update::{self, MAX_SIZE, Manifest, Package};
use katna_dbus::{UpdateStatus, update_state as state};
use katna_sync::autoconfig::http;
use katna_sync::net::Tls;

use crate::daemon::{Daemon, Notice, settings};

/// The first check waits a little, so starting stays quick.
const FIRST_CHECK: Duration = Duration::from_secs(120);
/// How often to look for a newer build.
const EVERY: Duration = Duration::from_secs(6 * 3600);
/// Sooner again after a check skipped on a metered connection.
const METERED_RETRY: Duration = Duration::from_secs(3600);
const MANIFEST_TIMEOUT: Duration = Duration::from_secs(30);
/// How many times a download is tried before it counts as failed, and
/// the pause before the second try (longer before each later one).
const DOWNLOAD_TRIES: u32 = 3;
const RETRY_PAUSE: Duration = Duration::from_secs(10);
/// The longest match distance a patch may use (`zstd --long=28`): room
/// for an earlier package of up to 256 MB.
const PATCH_WINDOW_LOG: u32 = 28;
/// Progress is told to Katna Mail at most this often.
const PROGRESS_EVERY: Duration = Duration::from_millis(250);

/// Why the update task woke.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Wake {
    /// "Check for updates" in Katna Mail.
    Check,
    /// "Download" in Katna Mail.
    Download,
    /// The settings changed: downloading may have been turned on.
    Settings,
    /// Time for the next check.
    Timer,
}

/// Where an update stands, and the task's wake-ups.
pub(crate) struct Updates {
    status: Mutex<UpdateStatus>,
    /// The newest build, once a check found one newer than this.
    offered: Mutex<Option<Manifest>>,
    wake: (Sender<Wake>, Receiver<Wake>),
    /// The notification that an update is ready, while it shows.
    notice: Mutex<Option<u32>>,
    /// The version that notification was shown for, so each is shown once.
    noticed: Mutex<Option<String>>,
}

impl Default for Updates {
    fn default() -> Self {
        let first = match Package::current().manifest_url() {
            Some(_) => state::IDLE,
            None => state::UNSUPPORTED,
        };
        Self {
            status: Mutex::new(UpdateStatus {
                state: first.to_owned(),
                ..UpdateStatus::default()
            }),
            offered: Mutex::default(),
            wake: async_channel::bounded(4),
            notice: Mutex::default(),
            noticed: Mutex::default(),
        }
    }
}

impl Updates {
    pub(crate) fn status(&self) -> UpdateStatus {
        self.status.lock().unwrap().clone()
    }

    /// The manifest of the version on offer, as JSON, or empty.
    pub(crate) fn details(&self) -> String {
        self.offered
            .lock()
            .unwrap()
            .as_ref()
            .and_then(|manifest| serde_json::to_string(manifest).ok())
            .unwrap_or_default()
    }

    pub(crate) fn check_now(&self) {
        let _ = self.wake.0.try_send(Wake::Check);
    }

    pub(crate) fn download_now(&self) {
        let _ = self.wake.0.try_send(Wake::Download);
    }

    pub(crate) fn settings_changed(&self) {
        let _ = self.wake.0.try_send(Wake::Settings);
    }

    /// Whether notification `id` is the one that an update is ready; it is
    /// forgotten.
    pub(crate) fn take_notice(&self, id: u32) -> bool {
        let mut notice = self.notice.lock().unwrap();
        if *notice == Some(id) {
            *notice = None;
            true
        } else {
            false
        }
    }

    pub(crate) fn is_notice(&self, id: u32) -> bool {
        *self.notice.lock().unwrap() == Some(id)
    }

    fn set(&self, daemon: &Daemon, change: impl FnOnce(&mut UpdateStatus)) {
        change(&mut self.status.lock().unwrap());
        let _ = daemon.notices().try_send(Notice::UpdateChanged);
    }

    fn state(&self) -> String {
        self.status.lock().unwrap().state.clone()
    }
}

/// Checks now and then, and when asked, until the daemon is gone.
pub(crate) async fn run(daemon: Weak<Daemon>) {
    let package = Package::current();
    if package.manifest_url().is_none() {
        tracing::debug!("this build does not update itself");
        return;
    }
    let Some(wake) = daemon.upgrade().map(|d| d.updates().wake.1.clone()) else {
        return;
    };
    let mut pause = FIRST_CHECK;
    loop {
        let why = async { wake.recv().await.unwrap_or(Wake::Timer) }
            .or(async {
                async_io::Timer::after(pause).await;
                Wake::Timer
            })
            .await;
        let Some(daemon) = daemon.upgrade() else {
            return;
        };
        if daemon.closing() {
            return;
        }
        match why {
            Wake::Timer if daemon.metered() => {
                tracing::debug!("metered connection; the update check waits");
                pause = METERED_RETRY;
                continue;
            }
            Wake::Timer | Wake::Check => {
                pause = EVERY;
                check(&daemon, package, why == Wake::Check).await;
            }
            Wake::Download => download(&daemon, package).await,
            Wake::Settings => {
                if daemon.updates().state() == state::AVAILABLE && may_download(&daemon) {
                    download(&daemon, package).await;
                }
            }
        }
    }
}

/// Whether to download without being asked.
fn may_download(daemon: &Daemon) -> bool {
    settings(daemon.paths()).updates.auto_download && !daemon.metered()
}

/// Where downloads wait to be installed.
fn download_dir(daemon: &Daemon) -> PathBuf {
    daemon.paths().cache_dir().join("updates")
}

/// Looks for a newer build, and downloads it when allowed. `asked`: the
/// user asked, so a failure says why.
async fn check(daemon: &Daemon, package: Package, asked: bool) {
    let updates = daemon.updates();
    let before = updates.status();
    updates.set(daemon, |s| {
        s.state = state::CHECKING.to_owned();
        s.detail.clear();
    });
    let manifest = match fetch_manifest(package).await {
        Ok(manifest) => manifest,
        Err(err) => {
            tracing::info!(%err, "no update check");
            updates.set(daemon, |s| {
                // A downloaded update stays ready.
                if before.state == state::READY {
                    *s = before.clone();
                } else {
                    s.state = state::FAILED.to_owned();
                    s.detail = if asked { err } else { String::new() };
                }
            });
            return;
        }
    };
    let checked = unix_now();
    if !manifest.newer_than(update::VERSION) {
        tracing::info!(
            installed = update::VERSION,
            newest = manifest.version,
            "up to date"
        );
        *updates.offered.lock().unwrap() = None;
        let dir = download_dir(daemon);
        smol::unblock(move || remove_downloads(&dir, None)).await;
        updates.set(daemon, |s| {
            *s = UpdateStatus {
                state: state::UP_TO_DATE.to_owned(),
                checked,
                ..UpdateStatus::default()
            };
        });
        return;
    }
    tracing::info!(version = manifest.version, "an update is available");
    *updates.offered.lock().unwrap() = Some(manifest.clone());
    if before.state == state::READY && before.version == manifest.version {
        updates.set(daemon, |s| {
            *s = UpdateStatus { checked, ..before };
        });
        return;
    }
    // Downloaded before the daemon restarted, in full or made from a
    // patch.
    let found = smol::unblock({
        let dir = download_dir(daemon);
        let manifest = manifest.clone();
        move || {
            [Fetched::full(&dir, &manifest)]
                .into_iter()
                .chain(Fetched::from_patch(&dir, &manifest))
                .find(|fetched| sha256_file(&fetched.file).is_ok_and(|sha| sha == fetched.sha256))
        }
    })
    .await;
    if let Some(fetched) = found {
        ready(daemon, &manifest, &fetched, checked).await;
        return;
    }
    // What the download will be: only the patch when it can be used.
    let total = match patch_and_base(package, &manifest) {
        Some((patch, _)) => patch.size,
        None => manifest.size,
    };
    updates.set(daemon, |s| {
        *s = UpdateStatus {
            state: state::AVAILABLE.to_owned(),
            version: manifest.version.clone(),
            total,
            checked,
            ..UpdateStatus::default()
        };
    });
    if may_download(daemon) {
        download(daemon, package).await;
    }
}

async fn fetch_manifest(package: Package) -> Result<Manifest, String> {
    let url = package
        .manifest_url()
        .ok_or_else(|| "this build does not update itself".to_owned())?;
    let tls = Tls::system().map_err(|err| err.to_string())?;
    let body = http::get(&url, &tls, MANIFEST_TIMEOUT)
        .await
        .map_err(|err| err.to_string())?
        .ok_or_else(|| format!("{url}: not found"))?;
    Manifest::parse(&body).ok_or_else(|| format!("{url}: not a Katna update manifest"))
}

/// Downloads the build a check found, checking its size and SHA-256.
/// A failed download is tried again: CI replaces the release's files
/// while a download may be running, so the next try reads the manifest
/// again.
async fn download(daemon: &Daemon, package: Package) {
    let updates = daemon.updates();
    let offered = updates.offered.lock().unwrap().clone();
    let Some(mut manifest) = offered else {
        // Nothing found yet: look first, then download what is found.
        Box::pin(check(daemon, package, true)).await;
        if updates.state() == state::AVAILABLE {
            Box::pin(download(daemon, package)).await;
        }
        return;
    };
    if updates.state() == state::READY {
        return;
    }
    let dir = download_dir(daemon);
    let mut try_number = 1;
    let fetched = loop {
        let err = match download_once(daemon, package, &dir, &manifest).await {
            Ok(file) => break Ok(file),
            Err(err) => err,
        };
        let Some(pause) = retry_pause(try_number) else {
            break Err(err);
        };
        tracing::info!(%err, try_number, "update download failed; trying again");
        async_io::Timer::after(pause).await;
        try_number += 1;
        // The release may have changed under the download.
        match fetch_manifest(package).await {
            Ok(newest) if newest.newer_than(update::VERSION) => {
                *updates.offered.lock().unwrap() = Some(newest.clone());
                manifest = newest;
            }
            Ok(_) => break Err("the update is no longer offered".to_owned()),
            Err(err) => break Err(err),
        }
    };
    match fetched {
        Ok(fetched) => ready(daemon, &manifest, &fetched, unix_now()).await,
        Err(err) => {
            tracing::warn!(%err, "update download failed");
            updates.set(daemon, |s| {
                s.state = state::DOWNLOAD_FAILED.to_owned();
                s.version = manifest.version.clone();
                s.done = 0;
                s.detail = err;
            });
        }
    }
}

/// How long to wait after try `try_number` of a download failed, or
/// `None` when it was the last.
fn retry_pause(try_number: u32) -> Option<Duration> {
    (try_number < DOWNLOAD_TRIES).then(|| RETRY_PAUSE * try_number)
}

/// A downloaded package, checked, and what to check it against again.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Fetched {
    file: PathBuf,
    sha256: String,
    size: u64,
    minisig: Option<String>,
}

impl Fetched {
    /// The full package `manifest` names, in `dir`.
    fn full(dir: &Path, manifest: &Manifest) -> Self {
        Self {
            file: dir.join(&manifest.file),
            sha256: manifest.sha256.clone(),
            size: manifest.size,
            minisig: manifest.minisig.clone(),
        }
    }

    /// The uncompressed package a patch makes, in `dir`.
    fn from_patch(dir: &Path, manifest: &Manifest) -> Option<Self> {
        let tar = manifest.tar.as_ref()?;
        Some(Self {
            file: dir.join(manifest.tar_file()),
            sha256: tar.sha256.clone(),
            size: tar.size,
            minisig: tar.minisig.clone(),
        })
    }
}

/// The patch from the installed build to the one `manifest` names, and
/// the copy of the installed build the root helper kept, when both exist.
fn patch_and_base(package: Package, manifest: &Manifest) -> Option<(update::Patch, PathBuf)> {
    let patch = manifest.patch_from(update::VERSION)?.clone();
    let base = package
        .installed_dirs()
        .iter()
        .find_map(|dir| installed_copy(Path::new(dir), update::VERSION))?;
    Some((patch, base))
}

/// The package of `version` in `dir`.
fn installed_copy(dir: &Path, version: &str) -> Option<PathBuf> {
    std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .find(|path| {
            path.is_file()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .and_then(update::package_version)
                    == Some(version)
        })
}

/// Downloads the build `manifest` names into `dir` and checks it: only
/// the patch from the installed build when there is one, else the full
/// package. Returns what was fetched.
async fn download_once(
    daemon: &Daemon,
    package: Package,
    dir: &Path,
    manifest: &Manifest,
) -> Result<Fetched, String> {
    if let Some((patch, base)) = patch_and_base(package, manifest) {
        match download_patch(daemon, package, dir, manifest, &patch, &base).await {
            Ok(fetched) => return Ok(fetched),
            Err(err) => {
                tracing::warn!(%err, from = patch.from, "the update patch did not work; downloading the full package");
            }
        }
    }
    let fetched = Fetched::full(dir, manifest);
    download_checked(
        daemon,
        package,
        dir,
        manifest,
        &manifest.file,
        fetched.size,
        &fetched.sha256,
        &fetched.file,
    )
    .await?;
    Ok(fetched)
}

/// Downloads `patch` and makes the new package from it and `base`, the
/// installed build's package, checking the result as a full download.
async fn download_patch(
    daemon: &Daemon,
    package: Package,
    dir: &Path,
    manifest: &Manifest,
    patch: &update::Patch,
    base: &Path,
) -> Result<Fetched, String> {
    let patch_file = dir.join(&patch.file);
    download_checked(
        daemon,
        package,
        dir,
        manifest,
        &patch.file,
        patch.size,
        &patch.sha256,
        &patch_file,
    )
    .await?;
    let fetched =
        Fetched::from_patch(dir, manifest).ok_or("the manifest has no package to patch")?;
    let made = smol::unblock({
        let base = base.to_owned();
        let patch_file = patch_file.clone();
        let fetched = fetched.clone();
        move || {
            let made = apply_patch(&base, &patch_file, &fetched);
            let _ = std::fs::remove_file(&patch_file);
            made
        }
    })
    .await;
    tracing::info!(
        from = patch.from,
        size = patch.size,
        ok = made.is_ok(),
        "made the update from a patch"
    );
    made.map(|()| fetched)
}

/// Makes the package `fetched` names from `base` and the patch in
/// `patch_file`, and checks its size and SHA-256.
fn apply_patch(base: &Path, patch_file: &Path, fetched: &Fetched) -> Result<(), String> {
    use std::io::Read;
    // The patch refers to the uncompressed earlier package.
    let mut reference = Vec::new();
    let base_file =
        std::fs::File::open(base).map_err(|err| format!("{}: {err}", base.display()))?;
    let read = if base.extension().is_some_and(|ext| ext == "zst") {
        zstd::stream::read::Decoder::new(base_file).and_then(|mut decoder| {
            decoder.window_log_max(31)?;
            decoder.take(MAX_SIZE + 1).read_to_end(&mut reference)
        })
    } else {
        base_file.take(MAX_SIZE + 1).read_to_end(&mut reference)
    };
    read.map_err(|err| format!("reading {}: {err}", base.display()))?;
    let input = std::fs::File::open(patch_file).map_err(|err| err.to_string())?;
    let mut decoder =
        zstd::stream::read::Decoder::with_ref_prefix(std::io::BufReader::new(input), &reference)
            .map_err(|err| err.to_string())?;
    decoder
        .window_log_max(PATCH_WINDOW_LOG)
        .map_err(|err| err.to_string())?;
    let part = part_path(&fetched.file);
    let made = (|| {
        let mut out = std::io::BufWriter::new(std::fs::File::create(&part)?);
        let mut digest = ring::digest::Context::new(&ring::digest::SHA256);
        let mut buf = vec![0; 256 * 1024];
        let mut size = 0u64;
        let mut limited = decoder.take(fetched.size + 1);
        loop {
            let n = limited.read(&mut buf)?;
            if n == 0 {
                break;
            }
            out.write_all(&buf[..n])?;
            digest.update(&buf[..n]);
            size += n as u64;
        }
        out.flush()?;
        out.get_ref().sync_all()?;
        Ok::<_, std::io::Error>((size, hex(digest.finish().as_ref())))
    })();
    let checked = made.map_err(|err| err.to_string()).and_then(|(size, sha)| {
        if size != fetched.size || sha != fetched.sha256 {
            Err(format!("the patched package does not match the published build ({size} bytes, SHA-256 {sha})"))
        } else {
            std::fs::rename(&part, &fetched.file).map_err(|err| err.to_string())
        }
    });
    if checked.is_err() {
        let _ = std::fs::remove_file(&part);
    }
    checked
}

fn part_path(file: &Path) -> PathBuf {
    let mut path = file.as_os_str().to_owned();
    path.push(".part");
    PathBuf::from(path)
}

/// Downloads `name` from the release into `file`, checking its size and
/// SHA-256, and shows its progress.
#[allow(clippy::too_many_arguments)]
async fn download_checked(
    daemon: &Daemon,
    package: Package,
    dir: &Path,
    manifest: &Manifest,
    name: &str,
    size: u64,
    sha256: &str,
    file: &Path,
) -> Result<(), String> {
    let updates = daemon.updates();
    let url = package
        .file_url(name)
        .ok_or_else(|| "this build does not update itself".to_owned())?;
    let part = part_path(file);
    updates.set(daemon, |s| {
        s.state = state::DOWNLOADING.to_owned();
        s.version = manifest.version.clone();
        s.done = 0;
        s.total = size;
        s.detail.clear();
    });
    tracing::info!(url, "downloading an update");
    let fetched = fetch_file(daemon, &url, dir, &part, size).await;
    let checked = fetched.and_then(|(got, sha)| {
        if got != size || sha != sha256 {
            Err(format!(
                "the download does not match the published build \
                 ({got} bytes, SHA-256 {sha})"
            ))
        } else {
            std::fs::rename(&part, file).map_err(|err| err.to_string())
        }
    });
    if checked.is_err() {
        let _ = std::fs::remove_file(&part);
    }
    checked
}

/// Downloads `url` into `part`, in `dir` with only this download in it.
/// Returns its size and SHA-256.
async fn fetch_file(
    daemon: &Daemon,
    url: &str,
    dir: &Path,
    part: &Path,
    size: u64,
) -> Result<(u64, String), String> {
    let prepared = {
        let dir = dir.to_owned();
        let part = part.to_owned();
        smol::unblock(move || {
            std::fs::create_dir_all(&dir)?;
            remove_downloads(&dir, None);
            std::fs::File::create(&part)
        })
        .await
    };
    let mut out = std::io::BufWriter::new(prepared.map_err(|err| err.to_string())?);
    let tls = Tls::system().map_err(|err| err.to_string())?;
    let mut digest = ring::digest::Context::new(&ring::digest::SHA256);
    let mut done = 0u64;
    let mut told = Instant::now();
    let updates = daemon.updates();
    let mut sink = |bytes: &[u8], _total: u64| -> std::io::Result<()> {
        out.write_all(bytes)?;
        digest.update(bytes);
        done += bytes.len() as u64;
        if told.elapsed() >= PROGRESS_EVERY {
            told = Instant::now();
            updates.set(daemon, |s| s.done = done);
        }
        Ok(())
    };
    let got = http::download(url, &tls, MAX_SIZE.min(size), &mut sink)
        .await
        .map_err(|err| err.to_string())?;
    out.flush().map_err(|err| err.to_string())?;
    out.get_ref().sync_all().map_err(|err| err.to_string())?;
    Ok((got, hex(digest.finish().as_ref())))
}

/// The update in `file` is downloaded and checked: Katna Mail can install
/// it. Says so once per version.
async fn ready(daemon: &Daemon, manifest: &Manifest, fetched: &Fetched, checked: i64) {
    // The helper looks for the signature beside the package.
    let signature = signature_path(&fetched.file);
    let written = match &fetched.minisig {
        Some(sig) => std::fs::write(&signature, sig),
        None => match std::fs::remove_file(&signature) {
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
            other => other,
        },
    };
    if let Err(error) = written {
        tracing::warn!(%error, "could not save the update's signature");
    }
    // What was downloaded: the patch, when the package was made from one.
    let downloaded = if fetched.file.extension().is_some_and(|ext| ext == "zst") {
        manifest.size
    } else {
        manifest
            .patch_from(update::VERSION)
            .map_or(fetched.size, |patch| patch.size)
    };
    let updates = daemon.updates();
    updates.set(daemon, |s| {
        *s = UpdateStatus {
            state: state::READY.to_owned(),
            version: manifest.version.clone(),
            done: downloaded,
            total: downloaded,
            file: fetched.file.display().to_string(),
            sha256: fetched.sha256.clone(),
            checked,
            detail: String::new(),
        };
    });
    tracing::info!(version = manifest.version, "update ready to install");
    let first = updates.noticed.lock().unwrap().as_ref() != Some(&manifest.version);
    if !first {
        return;
    }
    *updates.noticed.lock().unwrap() = Some(manifest.version.clone());
    if let Some(notices) = daemon.new_mail_notices()
        && let Some(id) = notices.update_ready(&manifest.version).await
    {
        *updates.notice.lock().unwrap() = Some(id);
    }
}

/// Where the signature of the package in `file` is saved: `<file>.minisig`,
/// where `packaging/arch/katna-update-helper` reads it.
fn signature_path(file: &Path) -> PathBuf {
    let mut path = file.as_os_str().to_owned();
    path.push(".minisig");
    PathBuf::from(path)
}

/// Deletes the downloads in `dir`, except `keep`.
fn remove_downloads(dir: &Path, keep: Option<&Path>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if Some(path.as_path()) != keep && path.is_file() {
            let _ = std::fs::remove_file(path);
        }
    }
}

fn sha256_file(path: &Path) -> std::io::Result<String> {
    use std::io::Read;
    let mut file = std::fs::File::open(path)?;
    let mut digest = ring::digest::Context::new(&ring::digest::SHA256);
    let mut buf = vec![0; 64 * 1024];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        digest.update(&buf[..n]);
    }
    Ok(hex(digest.finish().as_ref()))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_a_file() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("pkg");
        std::fs::write(&path, b"test").unwrap();
        assert_eq!(
            sha256_file(&path).unwrap(),
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
        );
    }

    #[test]
    fn a_failed_download_is_tried_again() {
        assert_eq!(retry_pause(1), Some(RETRY_PAUSE));
        assert_eq!(retry_pause(2), Some(RETRY_PAUSE * 2));
        assert_eq!(retry_pause(DOWNLOAD_TRIES), None);
    }

    /// A patch as CI makes it (`zstd --patch-from`), from `old` to `new`.
    fn make_patch(old: &[u8], new: &[u8]) -> Vec<u8> {
        let mut encoder =
            zstd::stream::write::Encoder::with_ref_prefix(Vec::new(), 19, old).unwrap();
        encoder.window_log(PATCH_WINDOW_LOG).unwrap();
        encoder.long_distance_matching(true).unwrap();
        encoder.write_all(new).unwrap();
        encoder.finish().unwrap()
    }

    fn sha(bytes: &[u8]) -> String {
        hex(ring::digest::digest(&ring::digest::SHA256, bytes).as_ref())
    }

    #[test]
    fn makes_the_update_from_a_patch() {
        let tmp = tempfile::tempdir().unwrap();
        let old: Vec<u8> = (0..200_000u32).flat_map(|n| n.to_le_bytes()).collect();
        let mut new = old.clone();
        new.splice(1000..1000, b"a new feature".iter().copied());
        let patch = make_patch(&old, &new);
        assert!(patch.len() < 1000, "{} bytes", patch.len());
        let patch_file = tmp.path().join("p.patch.zst");
        std::fs::write(&patch_file, &patch).unwrap();
        let fetched = Fetched {
            file: tmp
                .path()
                .join("katna-git-0.0.0.r2.gbbbbbbb-1-x86_64.pkg.tar"),
            sha256: sha(&new),
            size: new.len() as u64,
            minisig: None,
        };
        // From the kept package, compressed as the helper keeps it, or not.
        let zst = tmp
            .path()
            .join("katna-git-0.0.0.r1.gaaaaaaa-1-x86_64.pkg.tar.zst");
        std::fs::write(&zst, zstd::encode_all(&old[..], 3).unwrap()).unwrap();
        apply_patch(&zst, &patch_file, &fetched).unwrap();
        assert_eq!(std::fs::read(&fetched.file).unwrap(), new);
        let plain = tmp
            .path()
            .join("katna-git-0.0.0.r1.gaaaaaaa-1-x86_64.pkg.tar");
        std::fs::write(&plain, &old).unwrap();
        std::fs::remove_file(&fetched.file).unwrap();
        apply_patch(&plain, &patch_file, &fetched).unwrap();
        assert_eq!(std::fs::read(&fetched.file).unwrap(), new);
    }

    #[test]
    fn a_wrong_base_makes_nothing() {
        let tmp = tempfile::tempdir().unwrap();
        // Data that does not compress, so the patch must refer to it.
        let old: Vec<u8> = (0..100_000u64)
            .map(|n| (n.wrapping_mul(6_364_136_223_846_793_005) >> 56) as u8)
            .collect();
        let new = [&old[..], b"more"].concat();
        let patch_file = tmp.path().join("p.patch.zst");
        std::fs::write(&patch_file, make_patch(&old, &new)).unwrap();
        let base = tmp
            .path()
            .join("katna-git-0.0.0.r1.gaaaaaaa-1-x86_64.pkg.tar");
        std::fs::write(&base, vec![9u8; 100_000]).unwrap();
        let fetched = Fetched {
            file: tmp.path().join("new.pkg.tar"),
            sha256: sha(&new),
            size: new.len() as u64,
            minisig: None,
        };
        assert!(apply_patch(&base, &patch_file, &fetched).is_err());
        assert!(!fetched.file.exists());
        assert!(!part_path(&fetched.file).exists());
    }

    #[test]
    fn finds_the_installed_package() {
        let tmp = tempfile::tempdir().unwrap();
        for name in [
            "katna-git-0.0.0.r1.gaaaaaaa-1-x86_64.pkg.tar.zst",
            "katna-git-0.0.0.r2.gbbbbbbb-1-x86_64.pkg.tar.zst.sig",
            "katna-git-0.0.0.r2.gbbbbbbb-1-x86_64.pkg.tar.zst",
        ] {
            std::fs::write(tmp.path().join(name), b"x").unwrap();
        }
        assert_eq!(
            installed_copy(tmp.path(), "0.0.0.r2.gbbbbbbb"),
            Some(
                tmp.path()
                    .join("katna-git-0.0.0.r2.gbbbbbbb-1-x86_64.pkg.tar.zst")
            )
        );
        assert_eq!(installed_copy(tmp.path(), "0.0.0.r3.gccccccc"), None);
    }

    #[test]
    fn clears_old_downloads() {
        let tmp = tempfile::tempdir().unwrap();
        let old = tmp.path().join("old.pkg.tar.zst");
        let new = tmp.path().join("new.pkg.tar.zst");
        std::fs::write(&old, b"1").unwrap();
        std::fs::write(&new, b"2").unwrap();
        remove_downloads(tmp.path(), Some(&new));
        assert!(!old.exists());
        assert!(new.exists());
    }
}
