// SPDX-License-Identifier: GPL-3.0-or-later

//! Noticing a package update. Installing a new `katna-daemon` replaces the
//! file on disk while the old one keeps running, so new features (the tray
//! icon, notifications) would wait for the next login. The daemon checks
//! its binary now and then and, once it was replaced, has systemd restart
//! it, or without systemd shuts down and starts the new one in its place
//! (`docs/ARCHITECTURE.md` §9.2).

use std::path::{Path, PathBuf};
use std::time::Duration;

use futures_lite::StreamExt;

/// How often the binary is checked.
const CHECK_EVERY: Duration = Duration::from_secs(30);

/// Where Linux shows the running binary.
const SELF_EXE: &str = "/proc/self/exe";

/// Resolves with the path of the new binary once the running one was
/// replaced on disk.
pub async fn replaced() -> PathBuf {
    let mut ticks = smol::Timer::interval(CHECK_EVERY);
    loop {
        ticks.next().await;
        if let Ok(link) = std::fs::read_link(SELF_EXE)
            && let Some(new) = new_binary(&link)
            && new.is_file()
        {
            tracing::info!(path = %new.display(), "katna-daemon was updated");
            return new;
        }
    }
}

/// The path a replaced binary had: Linux adds ` (deleted)` to
/// `/proc/self/exe` once the file it ran from is gone.
fn new_binary(link: &Path) -> Option<PathBuf> {
    link.to_str()?.strip_suffix(" (deleted)").map(PathBuf::from)
}

/// Asks systemd to restart the service this daemon runs as, which starts
/// the new binary. Returns whether it took the job: it then stops this
/// process with SIGTERM, and the caller waits for that.
///
/// Starting the new binary in place ([`restart`]) is wrong under systemd:
/// shutting down releases the bus name, systemd stops a `Type=dbus`
/// service that loses its name, its SIGTERM got lost in the exec, and 90
/// seconds later it killed the new daemon with its whole group, a Katna
/// Mail window the tray had opened included.
pub async fn restart_by_systemd(connection: &zbus::Connection) -> bool {
    let Some(service) = crate::systemd::own_service(connection).await else {
        return false;
    };
    match crate::systemd::restart(connection, &service).await {
        Ok(()) => {
            tracing::info!(%service, "systemd restarts katna-daemon");
            true
        }
        Err(err) => {
            tracing::warn!(%err, %service, "systemd did not restart katna-daemon");
            false
        }
    }
}

/// Starts `binary` in place of this process, with the same arguments.
/// Returns only if that fails.
#[cfg(unix)]
pub fn restart(binary: &Path) -> std::io::Error {
    use std::os::unix::process::CommandExt;
    std::process::Command::new(binary)
        .args(std::env::args_os().skip(1))
        .exec()
}

/// Starts `binary` with the same arguments; the caller then exits, as
/// Windows cannot replace a running process. Returns only if that fails.
#[cfg(windows)]
pub fn restart(binary: &Path) -> std::io::Error {
    match std::process::Command::new(binary)
        .args(std::env::args_os().skip(1))
        .spawn()
    {
        Ok(_) => std::process::exit(0),
        Err(err) => err,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sees_a_replaced_binary() {
        assert_eq!(
            new_binary(Path::new("/usr/bin/katna-daemon (deleted)")),
            Some(PathBuf::from("/usr/bin/katna-daemon"))
        );
        assert_eq!(new_binary(Path::new("/usr/bin/katna-daemon")), None);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn this_binary_is_not_replaced() {
        let link = std::fs::read_link(SELF_EXE).unwrap();
        assert_eq!(new_binary(&link), None);
    }
}
