// SPDX-License-Identifier: GPL-3.0-or-later

//! Asking Katna Mail to do something (open a message, start a new one, …)
//! for a notification or the tray: through its `org.freedesktop.Application`
//! interface when it is running, else by starting it.

use std::collections::HashMap;
use std::process::Command;
use std::time::Duration;

use futures_lite::FutureExt;

use katna_core::ids;
use katna_dbus::app_action;
use zbus::zvariant::Value;

/// How long a running Katna Mail has to answer the tray.
const ANSWER_WITHIN: Duration = Duration::from_secs(5);

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
    let argument = match params.first() {
        Some(Value::I64(id)) => Some(id.to_string()),
        Some(Value::Str(text)) => Some(text.to_string()),
        _ => None,
    };
    let mut platform: HashMap<&str, Value<'_>> = HashMap::new();
    if let Some(token) = &token {
        platform.insert("activation-token", Value::from(token.as_str()));
    }
    let interface = Some("org.freedesktop.Application");
    let path = ids::MAIL_OBJECT_PATH;
    let call = async {
        match action {
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
        }
    };
    // A Katna Mail that hangs must not keep the tray from answering the
    // next click: those wait for this one.
    let called = async { Some(call.await) }
        .or(async {
            smol::Timer::after(ANSWER_WITHIN).await;
            None
        })
        .await;
    match called {
        Some(Ok(_)) => return true,
        Some(Err(err)) => tracing::debug!(%err, ?action, "Katna Mail is not running"),
        // Another copy would only hand over to the one that hangs.
        None => {
            tracing::warn!(?action, "Katna Mail did not answer the tray");
            return false;
        }
    }
    if action == Some(app_action::QUIT) {
        return false;
    }
    let mut command = Command::new(mail_program());
    if let Some(action) = action
        && let Some(flag) = app_action::flag(action)
    {
        if app_action::takes_message(action) || app_action::takes_text(action) {
            // Without the message there is nothing to open: just start.
            if let Some(argument) = argument {
                command.arg(flag).arg(argument);
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
    spawn(command);
    false
}

/// Has Katna Mail write a new message as the `mailto:` link `uri` asks.
/// Returns whether a running app took it.
pub(crate) async fn open_mailto(
    connection: &zbus::Connection,
    uri: &str,
    token: Option<String>,
) -> bool {
    let mut platform: HashMap<&str, Value<'_>> = HashMap::new();
    if let Some(token) = &token {
        platform.insert("activation-token", Value::from(token.as_str()));
    }
    let called = connection
        .call_method(
            Some(ids::MAIL_APP_ID),
            ids::MAIL_OBJECT_PATH,
            Some("org.freedesktop.Application"),
            "Open",
            &(vec![uri], platform),
        )
        .await;
    match called {
        Ok(_) => return true,
        Err(err) => tracing::debug!(%err, "Katna Mail is not running"),
    }
    let mut command = Command::new(mail_program());
    command.arg(uri);
    if let Some(token) = &token {
        command
            .env("XDG_ACTIVATION_TOKEN", token)
            .env("DESKTOP_STARTUP_ID", token);
    }
    spawn(command);
    false
}

/// Katna Mail: from `PATH` on Linux; on Windows the `katna-mail.exe`
/// beside this program, as Setup installs them together.
fn mail_program() -> std::path::PathBuf {
    let name = std::path::PathBuf::from("katna-mail");
    if !cfg!(windows) {
        return name;
    }
    std::env::current_exe()
        .ok()
        .and_then(|exe| Some(exe.parent()?.join("katna-mail.exe")))
        .unwrap_or(name)
}

fn spawn(mut command: Command) {
    match command.spawn() {
        // Reaped on its own thread, so it leaves no zombie behind.
        Ok(mut child) => {
            std::thread::spawn(move || child.wait());
        }
        Err(err) => tracing::warn!(%err, "could not start katna-mail"),
    }
}
