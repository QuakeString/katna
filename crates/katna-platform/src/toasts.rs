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
//! From the Store package they appear under the package's own app ID.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

use katna_core::ids;
use windows::Data::Xml::Dom::XmlDocument;
use windows::Foundation::{IReference, TypedEventHandler};
use windows::UI::Notifications::{
    ToastActivatedEventArgs, ToastNotification, ToastNotificationManager,
};
use windows::core::{HSTRING, IInspectable, Interface};
use zbus::object_server::SignalEmitter;
use zbus::zvariant::Value;
use zbus::{Connection, interface};

const PATH: &str = "/org/freedesktop/Notifications";
const NAME: &str = "org.freedesktop.Notifications";
/// The toast group Katna's toasts are in; each one's tag is its ID, so a
/// notification that replaces another replaces its toast.
const GROUP: &str = "katna";
/// Plasma's key for a reply typed into the notification: on a toast, a
/// text box with a Send button.
const INLINE_REPLY: &str = "inline-reply";
/// The text box's ID in the toast.
const REPLY_BOX: &str = "reply";
/// The most buttons a toast has.
const MOST_BUTTONS: usize = 5;

/// What happened to a toast, for the signal task.
enum Event {
    /// A click: notification ID and action key.
    Click(u32, String),
    /// A reply typed into it and sent.
    Reply(u32, String),
}

struct Server {
    next: AtomicU32,
    events: async_channel::Sender<Event>,
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
        // The sound `katna_platform::sound` names, e.g. "Mail".
        let sound = match hints.get("sound-name") {
            Some(Value::Str(name)) => Some(name.as_str()),
            _ => None,
        };
        let hint = |key: &str| match hints.get(key) {
            Some(Value::Str(text)) => Some(text.to_string()),
            _ => None,
        };
        let reply = Reply {
            placeholder: hint("x-kde-reply-placeholder-text").unwrap_or_default(),
            send: hint("x-kde-reply-submit-button-text").unwrap_or_default(),
        };
        let toast = Toast {
            summary: plain(summary),
            body: plain(body),
            actions: actions
                .chunks(2)
                .filter_map(|pair| match pair {
                    [key, label] => Some((key.clone(), label.clone())),
                    _ => None,
                })
                .collect(),
            sound: if quiet {
                Audio::Silent
            } else {
                Audio::named(sound)
            },
            reply,
        };
        if let Err(err) = show(id, &toast, self.events.clone()) {
            tracing::warn!(%err, "cannot show a toast");
        }
        id
    }

    /// Takes the toast out of the notification center.
    async fn close_notification(&self, id: u32) {
        if let Err(err) = remove(id) {
            tracing::debug!(%err, id, "cannot remove a toast");
        }
    }

    async fn get_capabilities(&self) -> Vec<&'static str> {
        vec!["actions", "body", INLINE_REPLY]
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

    #[zbus(signal)]
    async fn notification_replied(
        emitter: &SignalEmitter<'_>,
        id: u32,
        text: &str,
    ) -> zbus::Result<()>;
}

/// A toast's sound.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Audio {
    /// Windows' usual notification sound.
    Usual,
    Silent,
    /// One of Windows' notification sounds, as `ms-winsoundevent:` names it.
    Event(&'static str),
}

impl Audio {
    /// The sound `katna_platform::sound` calls `name`.
    fn named(name: Option<&str>) -> Self {
        match name {
            Some("Mail") => Self::Event("Notification.Mail"),
            Some("IM") => Self::Event("Notification.IM"),
            Some("Reminder") => Self::Event("Notification.Reminder"),
            Some("SMS") => Self::Event("Notification.SMS"),
            Some("Alarm") => Self::Event("Notification.Looping.Alarm"),
            _ => Self::Usual,
        }
    }
}

/// A reply box's grey text and the label of its Send button.
#[derive(Debug, Clone, Default)]
struct Reply {
    placeholder: String,
    send: String,
}

/// What a toast shows.
#[derive(Debug, Clone)]
struct Toast {
    summary: String,
    body: String,
    /// Key and label of each action; `default` is the click on the toast
    /// itself, and [`INLINE_REPLY`] a text box with a Send button.
    actions: Vec<(String, String)>,
    sound: Audio,
    reply: Reply,
}

impl Toast {
    /// The toast's XML (Windows' toast schema): the title, the first line
    /// of the body, the rest of it, then the reply box and the buttons.
    fn xml(&self) -> String {
        let mut xml =
            String::from(r#"<toast launch="default"><visual><binding template="ToastGeneric">"#);
        let mut lines = self.body.lines();
        let first = lines.next().unwrap_or_default();
        let rest = lines.collect::<Vec<_>>().join("\n");
        for text in [self.summary.as_str(), first, rest.as_str()] {
            if !text.trim().is_empty() {
                xml.push_str(&format!("<text>{}</text>", escape(text)));
            }
        }
        xml.push_str("</binding></visual>");
        let mut actions = String::new();
        let mut buttons = 0;
        for (key, label) in &self.actions {
            if key == "default" || buttons == MOST_BUTTONS {
                continue;
            }
            if key == INLINE_REPLY {
                actions.push_str(&format!(
                    r#"<input id="{REPLY_BOX}" type="text" placeHolderContent="{}"/>"#,
                    escape(&self.reply.placeholder)
                ));
                let send = if self.reply.send.is_empty() {
                    label
                } else {
                    &self.reply.send
                };
                actions.push_str(&format!(
                    r#"<action content="{}" arguments="{INLINE_REPLY}" hint-inputId="{REPLY_BOX}"/>"#,
                    escape(send)
                ));
            } else {
                actions.push_str(&format!(
                    r#"<action content="{}" arguments="{}"/>"#,
                    escape(label),
                    escape(key)
                ));
            }
            buttons += 1;
        }
        if !actions.is_empty() {
            xml.push_str(&format!("<actions>{actions}</actions>"));
        }
        match self.sound {
            Audio::Usual => {}
            Audio::Silent => xml.push_str(r#"<audio silent="true"/>"#),
            Audio::Event(name) => {
                xml.push_str(&format!(r#"<audio src="ms-winsoundevent:{name}"/>"#));
            }
        }
        xml.push_str("</toast>");
        xml
    }
}

/// `text` for the toast's XML.
fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Shows `toast` as notification `id`, in place of the toast it had;
/// a click or a sent reply goes to `events`.
fn show(id: u32, toast: &Toast, events: async_channel::Sender<Event>) -> windows::core::Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml(&HSTRING::from(toast.xml()))?;
    let notification = ToastNotification::CreateToastNotification(&doc)?;
    notification.SetTag(&HSTRING::from(id.to_string()))?;
    notification.SetGroup(&HSTRING::from(GROUP))?;
    notification.Activated(&TypedEventHandler::<ToastNotification, IInspectable>::new(
        move |_, args| {
            let Some(args) = args
                .as_ref()
                .and_then(|a| a.cast::<ToastActivatedEventArgs>().ok())
            else {
                return Ok(());
            };
            let key = args
                .Arguments()
                .map(|a| a.to_string())
                .ok()
                .filter(|a| !a.is_empty())
                .unwrap_or_else(|| "default".to_owned());
            let event = if key == INLINE_REPLY {
                let text = args
                    .UserInput()
                    .and_then(|input| input.Lookup(&HSTRING::from(REPLY_BOX)))
                    .and_then(|value| value.cast::<IReference<HSTRING>>())
                    .and_then(|value| value.Value())
                    .map(|text| text.to_string())
                    .unwrap_or_default();
                Event::Reply(id, text)
            } else {
                Event::Click(id, key)
            };
            let _ = events.try_send(event);
            Ok(())
        },
    ))?;
    let notifier = if in_store_package() {
        ToastNotificationManager::CreateToastNotifier()?
    } else {
        ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(ids::MAIL_APP_ID))?
    };
    notifier.Show(&notification)
}

/// Whether Katna runs from its Store package, whose toasts go under the
/// package's own app ID: Windows ignores another one for it.
fn in_store_package() -> bool {
    katna_core::update::Package::current() == katna_core::update::Package::MsStore
}

/// Takes toast `id` out of the notification center.
fn remove(id: u32) -> windows::core::Result<()> {
    let (tag, group) = (HSTRING::from(id.to_string()), HSTRING::from(GROUP));
    let history = ToastNotificationManager::History()?;
    if in_store_package() {
        history.RemoveGroupedTag(&tag, &group)
    } else {
        history.RemoveGroupedTagWithId(&tag, &group, &HSTRING::from(ids::MAIL_APP_ID))
    }
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
    let (events, happened) = async_channel::unbounded::<Event>();
    connection
        .object_server()
        .at(
            PATH,
            Server {
                next: AtomicU32::new(1),
                events,
            },
        )
        .await?;
    connection.request_name(NAME).await?;
    let emitter = SignalEmitter::new(connection, PATH)?.into_owned();
    connection
        .executor()
        .spawn(
            async move {
                while let Ok(event) = happened.recv().await {
                    let id = match event {
                        Event::Click(id, key) => {
                            let _ = Server::action_invoked(&emitter, id, &key).await;
                            id
                        }
                        Event::Reply(id, text) => {
                            let _ = Server::notification_replied(&emitter, id, &text).await;
                            id
                        }
                    };
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
    fn a_toast_has_a_reply_box_and_its_buttons() {
        let toast = Toast {
            summary: "Alex <Lee>".into(),
            body: "Q3\nCan we meet?\nAt 3".into(),
            actions: vec![
                ("default".into(), "Open".into()),
                (INLINE_REPLY.into(), "Reply".into()),
                ("mark-read".into(), "Mark as read".into()),
            ],
            sound: Audio::named(Some("Mail")),
            reply: Reply {
                placeholder: "Reply to Alex…".into(),
                send: "Send".into(),
            },
        };
        assert_eq!(
            toast.xml(),
            r#"<toast launch="default"><visual><binding template="ToastGeneric"><text>Alex &lt;Lee&gt;</text><text>Q3</text><text>Can we meet?
At 3</text></binding></visual><actions><input id="reply" type="text" placeHolderContent="Reply to Alex…"/><action content="Send" arguments="inline-reply" hint-inputId="reply"/><action content="Mark as read" arguments="mark-read"/></actions><audio src="ms-winsoundevent:Notification.Mail"/></toast>"#
        );
        let quiet = Toast {
            sound: Audio::Silent,
            actions: Vec::new(),
            ..toast
        };
        assert!(
            quiet
                .xml()
                .ends_with(r#"</visual><audio silent="true"/></toast>"#)
        );
    }

    #[test]
    fn markup_comes_out() {
        assert_eq!(plain("Q3 &lt;draft&gt; &amp; notes"), "Q3 <draft> & notes");
        assert_eq!(plain("a &amp;lt; b"), "a &lt; b");
    }
}
