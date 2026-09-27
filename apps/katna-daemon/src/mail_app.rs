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
    // Kept for starting the app, as the call takes `params`.
    let message = match params.first() {
        Some(Value::I64(id)) => Some(*id),
        _ => None,
    };
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
    if let Some(action) = action
        && let Some(flag) = app_action::flag(action)
    {
        if app_action::takes_message(action) {
            // Without the message there is nothing to open: just start.
            if let Some(id) = message {
                command.arg(flag).arg(id.to_string());
            }
        } else {
            command.arg(flag);
        }
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
