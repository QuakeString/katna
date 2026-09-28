// SPDX-License-Identifier: GPL-3.0-or-later

//! Installing an update that `katna-daemon` downloaded and checked, and
//! starting Katna Mail again afterwards (`docs/ARCHITECTURE.md` §21.2).
//! Each kind of package installs in its own way; the rest of the flow is
//! the same for all. No GPUI here.
//!
//! The app installs rather than the daemon because the system's password
//! prompt (the polkit agent) belongs to the desktop session the app runs
//! in; a user service has none.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use katna_core::update::{self, Package};

/// The flag that makes a new Katna Mail wait for the old one to close,
/// followed by its process ID.
pub const AFTER_FLAG: &str = "--after-update";

/// Longest wait for the old Katna Mail to close.
const WAIT: Duration = Duration::from_secs(20);

/// Why an update was not installed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallError {
    /// The password prompt was closed, or the password refused.
    Cancelled,
    /// This build cannot install updates itself.
    Unsupported,
    /// Installing failed; the installer's own words.
    Failed(String),
}

/// Installs the package in `file`, which must have the SHA-256 `sha256`,
/// after the system's password prompt. Blocks until done.
pub fn install(file: &Path, sha256: &str) -> Result<(), InstallError> {
    match Package::current() {
        Package::Arch => install_arch(file, sha256),
        Package::Other => Err(InstallError::Unsupported),
    }
}

/// `pkexec` runs the package's helper as root once the password is given
/// (`packaging/arch/katna-update-helper`).
fn install_arch(file: &Path, sha256: &str) -> Result<(), InstallError> {
    let output = Command::new("pkexec")
        .arg(update::ARCH_HELPER)
        .arg(file)
        .arg(sha256)
        .output()
        .map_err(|err| InstallError::Failed(format!("pkexec: {err}")))?;
    let said = String::from_utf8_lossy(&output.stderr)
        .lines()
        .rfind(|line| !line.trim().is_empty())
        .unwrap_or_default()
        .trim()
        .to_owned();
    match output.status.code() {
        Some(0) => Ok(()),
        // pkexec: the prompt was dismissed (126) or the password refused
        // (127).
        Some(126 | 127) => Err(InstallError::Cancelled),
        code => {
            tracing::warn!(?code, said, "the update was not installed");
            Err(InstallError::Failed(if said.is_empty() {
                format!("exit status {code:?}")
            } else {
                said
            }))
        }
    }
}

/// Starts the Katna Mail just installed, which waits for this one to
/// close before it opens its window. Quit this one next.
pub fn start_new() -> std::io::Result<()> {
    Command::new(installed_exe()?)
        .arg(AFTER_FLAG)
        .arg(std::process::id().to_string())
        .spawn()
        .map(|_| ())
}

/// The program at the path this one was started from: after an update,
/// the new version.
fn installed_exe() -> std::io::Result<PathBuf> {
    let exe = std::env::current_exe()?;
    // Linux names a replaced program "<path> (deleted)".
    Ok(
        match exe.to_str().and_then(|s| s.strip_suffix(" (deleted)")) {
            Some(path) => PathBuf::from(path),
            None => exe,
        },
    )
}

/// Waits until process `pid`, the Katna Mail that was updated, is gone,
/// so this one can take its place.
pub fn wait_for_exit(pid: u32) {
    let start = Instant::now();
    while running(pid) && start.elapsed() < WAIT {
        std::thread::sleep(Duration::from_millis(100));
    }
}

#[cfg(target_os = "linux")]
fn running(pid: u32) -> bool {
    Path::new(&format!("/proc/{pid}")).exists()
}

#[cfg(not(target_os = "linux"))]
fn running(_pid: u32) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_build_from_source_does_not_install() {
        if Package::current() == Package::Other {
            assert_eq!(
                install(Path::new("/nonexistent"), "0"),
                Err(InstallError::Unsupported)
            );
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn waits_only_for_a_running_process() {
        assert!(running(std::process::id()));
        let start = Instant::now();
        // No process has this ID (above the kernel's limit).
        wait_for_exit(u32::MAX);
        assert!(start.elapsed() < Duration::from_secs(1));
    }
}
