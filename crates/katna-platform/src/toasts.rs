// SPDX-License-Identifier: GPL-3.0-or-later

//! Notifications on Windows (`docs/ARCHITECTURE.md` §27.1): the daemon
//! serves `org.freedesktop.Notifications` on Katna's own session bus and
//! shows each notification as a Windows toast, with its actions as buttons.
//! A click sends `ActionInvoked` as a Linux notification server does, so
//! `katna-notify` and the daemon's handling of clicks stay the same.
//!
//! Toasts appear under Katna Mail's AppUserModelID, which [`serve`]
//! registers for the user (`HKCU\Software\Classes\AppUserModelId`), so
//! Windows shows them with Katna Mail's name without a Start menu shortcut.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

use katna_core::ids;
use tauri_winrt_notification::{Sound, Toast};
use zbus::object_server::SignalEmitter;
use zbus::zvariant::Value;
use zbus::{Connection, interface};

const PATH: &str = "/org/freedesktop/Notifications";
const NAME: &str = "org.freedesktop.Notifications";

/// A click, for the signal task: notification ID and action key.
type Click = (u32, String);

struct Server {
    next: AtomicU32,
    clicks: async_channel::Sender<Click>,
}

#[interface(name = "org.freedesktop.Notifications")]
impl Server {
    #[allow(clippy::too_many_arguments)]
    async fn notify(
        &self,
        _app_name: &str,
        replaces_id: u32,
        _app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<String>,
        hints: HashMap<String, Value<'_>>,
        _expire_timeout: i32,
    ) -> u32 {
        let id = if replaces_id != 0 {
            replaces_id
        } else {
            self.next.fetch_add(1, Ordering::Relaxed)
        };
        let quiet = matches!(hints.get("suppress-sound"), Some(Value::Bool(true)));
        let mut toast = Toast::new(ids::MAIL_APP_ID).title(&plain(summary));
        let mut lines = body.lines().map(plain);
        if let Some(line) = lines.next() {
            toast = toast.text1(&line);
        }
        let rest: Vec<String> = lines.collect();
        if !rest.is_empty() {
            toast = toast.text2(&rest.join("\n"));
        }
        // Key, label, key, label: the click on the toast itself is
        // "default", which gets no button.
        for pair in actions.chunks(2) {
            if let [key, label] = pair
                && key != "default"
            {
                toast = toast.add_button(label, key);
            }
        }
        let clicks = self.clicks.clone();
        let toast = toast
            .sound(if quiet { None } else { Some(Sound::Mail) })
            .on_activated(move |action| {
                let key = action.unwrap_or_else(|| "default".to_owned());
                let _ = clicks.try_send((id, key));
                Ok(())
            });
        if let Err(err) = toast.show() {
            tracing::warn!(%err, "cannot show a toast");
        }
        id
    }

    /// Windows keeps shown toasts in the notification center; they stay.
    async fn close_notification(&self, _id: u32) {}

    async fn get_capabilities(&self) -> Vec<&'static str> {
        vec!["actions", "body"]
    }

    async fn get_server_information(
        &self,
    ) -> (&'static str, &'static str, &'static str, &'static str) {
        ("Katna toasts", "Katna", env!("CARGO_PKG_VERSION"), "1.2")
    }

    #[zbus(signal)]
    async fn action_invoked(
        emitter: &SignalEmitter<'_>,
        id: u32,
        action_key: &str,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn notification_closed(
        emitter: &SignalEmitter<'_>,
        id: u32,
        reason: u32,
    ) -> zbus::Result<()>;
}

/// The notification's text without its markup: `katna-notify` escapes
/// `<`, `>` and `&` for Linux servers; a toast shows text as it is.
fn plain(text: &str) -> String {
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
}

/// Registers Katna Mail's AppUserModelID, then serves notifications on
/// `connection` as toasts until it closes.
pub async fn serve(connection: &Connection) -> zbus::Result<()> {
    if let Err(err) = register_app_id() {
        tracing::warn!(%err, "cannot register Katna Mail for toasts");
    }
    let (clicks, clicked) = async_channel::unbounded::<Click>();
    connection
        .object_server()
        .at(
            PATH,
            Server {
                next: AtomicU32::new(1),
                clicks,
            },
        )
        .await?;
    connection.request_name(NAME).await?;
    let emitter = SignalEmitter::new(connection, PATH)?.into_owned();
    connection
        .executor()
        .spawn(
            async move {
                while let Ok((id, key)) = clicked.recv().await {
                    let _ = Server::action_invoked(&emitter, id, &key).await;
                    // The toast is gone once clicked; 2 = dismissed.
                    let _ = Server::notification_closed(&emitter, id, 2).await;
                }
            },
            "katna toast clicks",
        )
        .detach();
    Ok(())
}

/// `HKCU\Software\Classes\AppUserModelId\in.invenia.katna.Mail`, with the
/// name toasts show. Setup adds the icon.
fn register_app_id() -> std::io::Result<()> {
    let (key, _) = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER).create_subkey(
        format!(r"Software\Classes\AppUserModelId\{}", ids::MAIL_APP_ID),
    )?;
    key.set_value("DisplayName", &"Katna Mail")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markup_comes_out() {
        assert_eq!(plain("Q3 &lt;draft&gt; &amp; notes"), "Q3 <draft> & notes");
        assert_eq!(plain("a &amp;lt; b"), "a &lt; b");
    }
}
