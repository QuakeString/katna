// SPDX-License-Identifier: GPL-3.0-or-later

//! Starting `katna-daemon` where D-Bus cannot (`packaging/README.md`).
//!
//! Distribution packages install a D-Bus activation file, so the session
//! bus starts the daemon when a Katna program calls it. A Snap cannot put
//! one on the host without snapd's experimental user daemons, and neither
//! can an unpacked tarball before its `install.sh` has run. There the
//! programs start the daemon installed beside them themselves.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use katna_core::ids;

/// How long to wait for a daemon this started to take its bus name.
const WAIT: Duration = Duration::from_secs(10);

/// How long to wait for a daemon that D-Bus started to take its bus name.
const STARTED_WAIT: Duration = Duration::from_secs(5);

/// How long to wait for the daemon systemd started to take its bus name:
/// with many accounts it opens a while before it takes it.
#[cfg(not(windows))]
const UNIT_WAIT: Duration = Duration::from_secs(15);

/// The data directory this program was given (`--data-dir`), if any.
static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Has the daemons this program starts use `dir`, as `--data-dir` does:
/// they are then always the `katna-daemon` beside this program, since
/// systemd and D-Bus activation start one with the default folders.
pub fn use_data_dir(dir: PathBuf) {
    let _ = DATA_DIR.set(dir);
}

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
    if DATA_DIR.get().is_none() && activatable(&dbus).await {
        return;
    }
    if let Err(err) = start_beside(&dbus, WAIT).await {
        eprintln!("{err}");
    }
}

/// Whether `katna-daemon` runs now.
pub async fn daemon_running(connection: &zbus::Connection) -> bool {
    let Ok(dbus) = zbus::fdo::DBusProxy::new(connection).await else {
        return false;
    };
    let Ok(name) = zbus::names::BusName::try_from(ids::DAEMON_BUS_NAME) else {
        return false;
    };
    dbus.name_has_owner(name).await.unwrap_or(false)
}

/// Starts `katna-daemon` if it isn't running, the way it is installed:
/// through its systemd user unit where there is one (first cleared of an
/// earlier failure, which otherwise keeps systemd from starting it), else
/// through D-Bus activation or the `katna-daemon` beside this program.
/// Returns once it owns its bus name, or why it didn't, in words for a
/// report.
///
/// With a systemd unit it never starts a copy of its own: one outside
/// systemd would keep the unit from ever starting (it finds the name
/// taken), so `systemctl` would show the service as failed.
pub async fn start_daemon(connection: &zbus::Connection) -> Result<(), String> {
    let dbus = zbus::fdo::DBusProxy::new(connection)
        .await
        .map_err(|err| format!("D-Bus: {err}"))?;
    let name = zbus::names::BusName::try_from(ids::DAEMON_BUS_NAME)
        .map_err(|err| format!("D-Bus: {err}"))?;
    if dbus.name_has_owner(name.clone()).await.unwrap_or(false) {
        return Ok(());
    }
    if DATA_DIR.get().is_some() {
        return start_beside(&dbus, WAIT).await;
    }
    #[cfg(not(windows))]
    if let Some(started) = start_unit(connection).await {
        started?;
        return wait_for_owner(&dbus, &name, UNIT_WAIT).await;
    }
    if !activatable(&dbus).await {
        return start_beside(&dbus, WAIT).await;
    }
    let started: zbus::Result<u32> = connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "StartServiceByName",
            &(ids::DAEMON_BUS_NAME, 0u32),
        )
        .await
        .and_then(|reply| reply.body().deserialize());
    let activated = match started {
        Ok(_) => wait_for_owner(&dbus, &name, STARTED_WAIT).await,
        Err(err) => Err(format!("starting {}: {err}", ids::DAEMON_BUS_NAME)),
    };
    // When D-Bus won't start it, the `katna-daemon` beside this program
    // still can: the user never needs a terminal for it.
    match activated {
        Ok(()) => Ok(()),
        Err(activation) => start_beside(&dbus, WAIT)
            .await
            .map_err(|beside| format!("{activation}; {beside}")),
    }
}

/// Has systemd start the daemon's user unit, as `systemctl --user start`
/// does, after clearing an earlier failure. `None` where there is no
/// systemd or no such unit; else whether systemd took the job.
#[cfg(not(windows))]
async fn start_unit(connection: &zbus::Connection) -> Option<Result<(), String>> {
    let manager = zbus::Proxy::new(
        connection,
        "org.freedesktop.systemd1",
        "/org/freedesktop/systemd1",
        "org.freedesktop.systemd1.Manager",
    )
    .await
    .ok()?;
    let path: zbus::zvariant::OwnedObjectPath = manager.call("LoadUnit", &(UNIT,)).await.ok()?;
    let unit = zbus::Proxy::new(
        connection,
        "org.freedesktop.systemd1",
        path,
        "org.freedesktop.systemd1.Unit",
    )
    .await
    .ok()?;
    let state: String = unit.get_property("LoadState").await.ok()?;
    if state != "loaded" {
        return None;
    }
    let _: zbus::Result<()> = manager.call("ResetFailedUnit", &(UNIT,)).await;
    let job: zbus::Result<zbus::zvariant::OwnedObjectPath> =
        manager.call("StartUnit", &(UNIT, "replace")).await;
    Some(
        job.map(|_| ())
            .map_err(|err| format!("systemd, starting {UNIT}: {err}")),
    )
}

/// Whether the session bus has an activation file for the daemon.
async fn activatable(dbus: &zbus::fdo::DBusProxy<'_>) -> bool {
    dbus.list_activatable_names()
        .await
        .unwrap_or_default()
        .iter()
        .any(|n| n.as_str() == ids::DAEMON_BUS_NAME)
}

/// The daemon's systemd user unit (`packaging/systemd`).
#[cfg(not(windows))]
const UNIT: &str = "katna-daemon.service";

/// Starts the `katna-daemon` beside this program and waits up to `wait`
/// until it owns its bus name.
async fn start_beside(dbus: &zbus::fdo::DBusProxy<'_>, wait: Duration) -> Result<(), String> {
    let program = std::env::current_exe()
        .ok()
        .as_deref()
        .and_then(beside)
        .ok_or_else(|| "no katna-daemon beside this program".to_owned())?;
    let mut command = std::process::Command::new(&program);
    if let Some(dir) = DATA_DIR.get() {
        command.arg("--data-dir").arg(dir);
    }
    let child = command
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .spawn();
    match child {
        Ok(mut child) => {
            // Reaped on its own thread, so it leaves no zombie behind; it
            // keeps running after this program quits.
            std::thread::spawn(move || child.wait());
        }
        Err(err) => return Err(format!("cannot start {}: {err}", program.display())),
    }
    let name = zbus::names::BusName::try_from(ids::DAEMON_BUS_NAME)
        .map_err(|err| format!("D-Bus: {err}"))?;
    wait_for_owner(dbus, &name, wait).await
}

/// Waits up to `wait` until `name` has an owner.
async fn wait_for_owner(
    dbus: &zbus::fdo::DBusProxy<'_>,
    name: &zbus::names::BusName<'_>,
    wait: Duration,
) -> Result<(), String> {
    let start = Instant::now();
    while start.elapsed() < wait {
        if dbus.name_has_owner(name.clone()).await.unwrap_or(false) {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    Err(format!(
        "{} did not appear within {} s",
        ids::DAEMON_BUS_NAME,
        wait.as_secs()
    ))
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
