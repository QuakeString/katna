// SPDX-License-Identifier: GPL-3.0-or-later

//! Asking the systemd user manager, over the session bus, to restart this
//! daemon after an update and to give Katna Mail a group of its own. Where
//! there is no systemd, the calls fail and the callers go on without it.

use katna_core::ids;
use zbus::zvariant::{OwnedObjectPath, Value};

const DESTINATION: &str = "org.freedesktop.systemd1";
const PATH: &str = "/org/freedesktop/systemd1";
const MANAGER: &str = "org.freedesktop.systemd1.Manager";
const UNIT: &str = "org.freedesktop.systemd1.Unit";

async fn manager(connection: &zbus::Connection) -> zbus::Result<zbus::Proxy<'static>> {
    zbus::Proxy::new(connection, DESTINATION, PATH, MANAGER).await
}

/// The systemd service this process runs as, such as `katna-daemon.service`,
/// or `None` when something else started it (a terminal, D-Bus without
/// systemd).
pub(crate) async fn own_service(connection: &zbus::Connection) -> Option<String> {
    let manager = manager(connection).await.ok()?;
    let path: OwnedObjectPath = manager
        .call("GetUnitByPID", &(std::process::id(),))
        .await
        .ok()?;
    let unit = zbus::Proxy::new(connection, DESTINATION, path, UNIT)
        .await
        .ok()?;
    let id: String = unit.get_property("Id").await.ok()?;
    id.ends_with(".service").then_some(id)
}

/// Has systemd stop `service` and start it again. It stops this process
/// with SIGTERM, then starts the binary now on disk.
pub(crate) async fn restart(connection: &zbus::Connection, service: &str) -> zbus::Result<()> {
    let _job: OwnedObjectPath = manager(connection)
        .await?
        .call("RestartUnit", &(service, "replace"))
        .await?;
    Ok(())
}

/// The scope a Katna Mail started by this daemon moves to, named as
/// desktops name the apps they start (`app-<app id>-<pid>.scope`).
fn scope_name(pid: u32) -> String {
    format!("app-{}-{pid}.scope", ids::MAIL_APP_ID)
}

/// Moves the process `pid` (a Katna Mail this daemon started) out of the
/// daemon's group into a scope of its own. Left in the daemon's group,
/// systemd would close its windows whenever the daemon stops or restarts,
/// as after every update.
pub(crate) async fn move_to_own_scope(connection: &zbus::Connection, pid: u32) -> zbus::Result<()> {
    let properties: Vec<(&str, Value<'_>)> = vec![
        ("Description", Value::from("Katna Mail")),
        ("PIDs", Value::from(vec![pid])),
        ("CollectMode", Value::from("inactive-or-failed")),
    ];
    let aux: Vec<(&str, Vec<(&str, Value<'_>)>)> = Vec::new();
    let _job: OwnedObjectPath = manager(connection)
        .await?
        .call(
            "StartTransientUnit",
            &(scope_name(pid), "fail", properties, aux),
        )
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_the_scope_like_a_desktop_would() {
        assert_eq!(
            scope_name(4242),
            format!("app-{}-4242.scope", ids::MAIL_APP_ID)
        );
    }
}
