// SPDX-License-Identifier: GPL-3.0-or-later

//! One Katna Mail per session (`docs/ARCHITECTURE.md` §15.2): the first
//! instance owns the bus name `in.invenia.katna.Mail` and serves
//! `org.freedesktop.Application` there. Launching the app again, a desktop
//! file action (right-click on the taskbar icon), the tray menu and
//! notifications all end up as a [`Request`] to that instance.
//! No GPUI here.

use std::collections::HashMap;

use async_channel::{Receiver, Sender};
use futures_lite::future;
use katna_core::ids;
use katna_dbus::app_action;
use zbus::Connection;
use zbus::fdo::{RequestNameFlags, RequestNameReply};
use zbus::zvariant::{OwnedValue, Value};

/// Something asked of the running app.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    /// Raise the window.
    Activate,
    /// One of [`app_action`]'s actions; `message` for `open-message`.
    Action { name: String, message: Option<i64> },
    /// A click in the menu bar on the GPUI action with this name.
    Menu(String),
    /// A `mailto:` link to write a message for.
    Mailto(String),
    /// Text to search for (`app_action::SEARCH`), from KRunner or GNOME's
    /// search.
    Search(String),
}

impl Request {
    /// The request that the command-line flag `flag` stands for.
    pub fn from_flag(flag: &str) -> Option<Self> {
        [
            app_action::OPEN_INBOX,
            app_action::COMPOSE,
            app_action::PREFERENCES,
        ]
        .into_iter()
        .find(|action| app_action::flag(action) == Some(flag))
        .map(Self::action)
    }

    /// The request that `flag` followed by message ID `id` stands for.
    pub fn for_message(flag: &str, id: i64) -> Option<Self> {
        [app_action::OPEN_MESSAGE, app_action::REPLY_ALL]
            .into_iter()
            .find(|action| app_action::flag(action) == Some(flag))
            .map(|name| Self::Action {
                name: name.to_owned(),
                message: Some(id),
            })
    }

    pub fn action(name: &str) -> Self {
        Self::Action {
            name: name.to_owned(),
            message: None,
        }
    }
}

/// How this process started.
pub enum Started {
    /// This is the app: requests arrive on the receiver. `connection` is
    /// `None` without a session bus.
    First {
        connection: Option<Connection>,
        sender: Sender<Request>,
        requests: Receiver<Request>,
    },
    /// Another instance was running and took the request; exit.
    HandedOff,
}

/// `org.freedesktop.Application` on the app's object path.
struct Application {
    requests: Sender<Request>,
}

#[zbus::interface(name = "org.freedesktop.Application")]
impl Application {
    fn activate(&self, _platform_data: HashMap<String, OwnedValue>) {
        let _ = self.requests.try_send(Request::Activate);
    }

    /// Opens `mailto:` links; Katna Mail opens no files.
    fn open(&self, uris: Vec<String>, _platform_data: HashMap<String, OwnedValue>) {
        let mut sent = false;
        for uri in uris
            .into_iter()
            .filter(|uri| crate::mailto::Mailto::parse(uri).is_some())
        {
            sent |= self.requests.try_send(Request::Mailto(uri)).is_ok();
        }
        if !sent {
            let _ = self.requests.try_send(Request::Activate);
        }
    }

    fn activate_action(
        &self,
        action_name: String,
        parameter: Vec<OwnedValue>,
        _platform_data: HashMap<String, OwnedValue>,
    ) {
        if action_name == app_action::SEARCH {
            if let Some(text) = parameter
                .into_iter()
                .next()
                .and_then(|value| String::try_from(value).ok())
            {
                let _ = self.requests.try_send(Request::Search(text));
            }
            return;
        }
        let message = parameter
            .into_iter()
            .next()
            .and_then(|value| i64::try_from(value).ok());
        let _ = self.requests.try_send(Request::Action {
            name: action_name,
            message,
        });
    }
}

/// Becomes the running instance, or hands `request` (plain activation if
/// `None`) to the one already running. `single` is `false` for a private
/// data directory (`--data-dir`): such a window stands on its own.
pub fn start(request: Option<Request>, single: bool) -> Started {
    let (sender, requests) = async_channel::unbounded();
    let connection = future::block_on(async {
        let connection = Connection::session().await.ok()?;
        if !single {
            return Some(Ok(connection));
        }
        let object = Application {
            requests: sender.clone(),
        };
        if let Err(err) = connection
            .object_server()
            .at(ids::MAIL_OBJECT_PATH, object)
            .await
        {
            tracing::warn!(%err, "cannot serve org.freedesktop.Application");
            return Some(Ok(connection));
        }
        let owned = connection
            .request_name_with_flags(ids::MAIL_APP_ID, RequestNameFlags::DoNotQueue.into())
            .await;
        match owned {
            Ok(RequestNameReply::PrimaryOwner | RequestNameReply::AlreadyOwner) => {
                Some(Ok(connection))
            }
            Ok(RequestNameReply::Exists | RequestNameReply::InQueue)
            | Err(zbus::Error::NameTaken) => Some(Err(connection)),
            Err(err) => {
                tracing::warn!(%err, "cannot take the app's bus name");
                Some(Ok(connection))
            }
        }
    });
    match connection {
        Some(Err(connection)) => {
            if future::block_on(hand_off(&connection, request.as_ref())) {
                return Started::HandedOff;
            }
            // The other instance did not answer; run anyway.
            if let Some(request) = request {
                let _ = sender.try_send(request);
            }
            Started::First {
                connection: Some(connection),
                sender,
                requests,
            }
        }
        Some(Ok(connection)) => {
            if let Some(request) = request {
                let _ = sender.try_send(request);
            }
            Started::First {
                connection: Some(connection),
                sender,
                requests,
            }
        }
        None => {
            if let Some(request) = request {
                let _ = sender.try_send(request);
            }
            Started::First {
                connection: None,
                sender,
                requests,
            }
        }
    }
}

/// Asks the running instance to do `request`; `false` if it did not answer.
async fn hand_off(connection: &Connection, request: Option<&Request>) -> bool {
    let mut platform: HashMap<&str, Value<'_>> = HashMap::new();
    // The launcher's token lets the other instance's window take focus.
    let token = std::env::var("XDG_ACTIVATION_TOKEN").ok();
    if let Some(token) = &token {
        platform.insert("activation-token", Value::from(token.as_str()));
    }
    let interface = Some("org.freedesktop.Application");
    let (app, path) = (Some(ids::MAIL_APP_ID), ids::MAIL_OBJECT_PATH);
    let called = match request {
        None | Some(Request::Activate | Request::Menu(_)) => {
            connection
                .call_method(app, path, interface, "Activate", &platform)
                .await
        }
        Some(Request::Mailto(uri)) => {
            connection
                .call_method(
                    app,
                    path,
                    interface,
                    "Open",
                    &(vec![uri.as_str()], platform),
                )
                .await
        }
        Some(Request::Search(text)) => {
            let params = vec![Value::from(text.as_str())];
            connection
                .call_method(
                    app,
                    path,
                    interface,
                    "ActivateAction",
                    &(app_action::SEARCH, params, platform),
                )
                .await
        }
        Some(Request::Action { name, message }) => {
            let params: Vec<Value<'_>> = message.iter().map(|&id| Value::from(id)).collect();
            connection
                .call_method(
                    app,
                    path,
                    interface,
                    "ActivateAction",
                    &(name.as_str(), params, platform),
                )
                .await
        }
    };
    match called {
        Ok(_) => true,
        Err(err) => {
            tracing::warn!(%err, "the running Katna Mail did not answer");
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_map_to_actions() {
        assert_eq!(
            Request::from_flag("--compose"),
            Some(Request::action("compose"))
        );
        assert_eq!(
            Request::from_flag("--settings"),
            Some(Request::action("preferences"))
        );
        assert_eq!(
            Request::from_flag("--inbox"),
            Some(Request::action("open-inbox"))
        );
        assert_eq!(Request::from_flag("--open"), None);
        assert_eq!(Request::from_flag("--message"), None);
    }

    #[test]
    fn message_flags_carry_the_id() {
        let request = |name: &str| Request::Action {
            name: name.to_owned(),
            message: Some(42),
        };
        assert_eq!(
            Request::for_message("--message", 42),
            Some(request("open-message"))
        );
        assert_eq!(
            Request::for_message("--reply-all", 42),
            Some(request("reply-all"))
        );
        assert_eq!(Request::for_message("--inbox", 42), None);
    }
}
