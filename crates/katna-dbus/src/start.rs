// SPDX-License-Identifier: GPL-3.0-or-later

//! Starting `katna-daemon` where D-Bus cannot (`packaging/README.md`).
//!
//! Distribution packages install a D-Bus activation file, so the session
//! bus starts the daemon when a Katna program calls it. A Snap cannot put
//! one on the host without snapd's experimental user daemons, and neither
//! can an unpacked tarball before its `install.sh` has run. There the
//! programs start the daemon installed beside them themselves.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use katna_core::ids;

/// How long to wait for a daemon this started to take its bus name.
const WAIT: Duration = Duration::from_secs(10);

/// Makes sure `katna-daemon` runs or can be started by D-Bus: when its bus
/// name has no owner and no activation file, starts the `katna-daemon`
/// beside this program and waits until it owns the name. Checks once per
/// process; failures are logged, and calls to the daemon then report it
/// is not running.
pub async fn ensure_daemon(connection: &zbus::Connection) {
    static CHECKED: AtomicBool = AtomicBool::new(false);
    if CHECKED.swap(true, Ordering::SeqCst) {
        return;
    }
    let Ok(dbus) = zbus::fdo::DBusProxy::new(connection).await else {
        return;
    };
    let Ok(name) = zbus::names::BusName::try_from(ids::DAEMON_BUS_NAME) else {
        return;
    };
    if dbus.name_has_owner(name.clone()).await.unwrap_or(true) {
        return;
    }
    let activatable = dbus.list_activatable_names().await.unwrap_or_default();
    if activatable
        .iter()
        .any(|n| n.as_str() == ids::DAEMON_BUS_NAME)
    {
        return;
    }
    let Some(program) = std::env::current_exe().ok().as_deref().and_then(beside) else {
        return;
    };
    let child = std::process::Command::new(&program)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .spawn();
    match child {
        Ok(mut child) => {
            // Reaped on its own thread, so it leaves no zombie behind; it
            // keeps running after this program quits.
            std::thread::spawn(move || child.wait());
        }
        Err(err) => {
            eprintln!("cannot start {}: {err}", program.display());
            return;
        }
    }
    let start = Instant::now();
    while start.elapsed() < WAIT {
        if dbus.name_has_owner(name.clone()).await.unwrap_or(false) {
            return;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// The `katna-daemon` installed beside `exe`, if there is one.
fn beside(exe: &Path) -> Option<PathBuf> {
    let name = if cfg!(windows) {
        "katna-daemon.exe"
    } else {
        "katna-daemon"
    };
    Some(exe.parent()?.join(name)).filter(|path| path.is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_only_an_existing_daemon() {
        let dir = std::env::temp_dir().join(format!("katna-start-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let exe = dir.join("katnactl");
        assert_eq!(beside(&exe), None);
        let daemon = dir.join(if cfg!(windows) {
            "katna-daemon.exe"
        } else {
            "katna-daemon"
        });
        std::fs::write(&daemon, "").unwrap();
        assert_eq!(beside(&exe), Some(daemon));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
