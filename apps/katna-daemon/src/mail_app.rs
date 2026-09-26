// SPDX-License-Identifier: GPL-3.0-or-later

//! Asking Katna Mail to do something (open a message, start a new one, …)
//! for a notification or the tray: through its `org.freedesktop.Application`
//! interface when it is running, else by starting it.

use std::collections::HashMap;
use std::process::Command;

use katna_core::ids;
use katna_dbus::app_action;
use zbus::zvariant::Value;

/// Runs `action` (see [`app_action`]) with `params` in Katna Mail, or just
/// raises its window for `None`. `token` lets the window take focus on
/// Wayland. Returns whether a running app took it.
pub(crate) async fn run(
    connection: &zbus::Connection,
    action: Option<&str>,
    params: Vec<Value<'_>>,
    token: Option<String>,
) -> bool {
    let mut platform: HashMap<&str, Value<'_>> = HashMap::new();
    if let Some(token) = &token {
        platform.insert("activation-token", Value::from(token.as_str()));
    }
    let interface = Some("org.freedesktop.Application");
    let path = ids::MAIL_OBJECT_PATH;
    let called = match action {
        Some(action) => {
            connection
                .call_method(
                    Some(ids::MAIL_APP_ID),
                    path,
                    interface,
                    "ActivateAction",
                    &(action, params, platform),
                )
                .await
        }
        None => {
            connection
                .call_method(
                    Some(ids::MAIL_APP_ID),
                    path,
                    interface,
                    "Activate",
                    &platform,
                )
                .await
        }
    };
    match called {
        Ok(_) => return true,
        Err(err) => tracing::debug!(%err, ?action, "Katna Mail is not running"),
    }
    if action == Some(app_action::QUIT) {
        return false;
    }
    let mut command = Command::new("katna-mail");
    if let Some(flag) = action.and_then(app_action::flag) {
        command.arg(flag);
    }
    if let Some(token) = &token {
        command
            .env("XDG_ACTIVATION_TOKEN", token)
            .env("DESKTOP_STARTUP_ID", token);
    }
    match command.spawn() {
        // Reaped on its own thread, so it leaves no zombie behind.
        Ok(mut child) => {
            std::thread::spawn(move || child.wait());
        }
        Err(err) => tracing::warn!(%err, "could not start katna-mail"),
    }
    false
}
