// SPDX-License-Identifier: GPL-3.0-or-later

//! Global shortcuts: keys that do something in Katna whichever app is in
//! front, such as Meta+Alt+T for quick capture (`docs/ARCHITECTURE.md`
//! §15.5).
//!
//! - **Plasma:** registered with KDE's global shortcuts service
//!   (`org.kde.KGlobalAccel`), so they are listed under the app's name in
//!   System Settings > Keyboard > Shortcuts, where they can be changed; a
//!   change made there wins over the default here. They are registered
//!   again whenever the service starts (KWin restarting).
//! - **Windows:** `RegisterHotKey`, through the `global-hotkey` crate, on
//!   the tray icon's thread, whose message loop receives the presses.
//! - **GNOME and other desktops:** nothing yet. Their way is the
//!   GlobalShortcuts portal, which Katna does not use so far.

use std::sync::Arc;

/// A key with modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keys {
    /// The Super, Windows or Meta key.
    pub meta: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    /// An ASCII letter or digit.
    pub key: char,
}

impl Keys {
    /// Meta+Alt and `key`.
    pub const fn meta_alt(key: char) -> Self {
        Self {
            meta: true,
            ctrl: false,
            alt: true,
            shift: false,
            key,
        }
    }

    /// The key as Qt combines it with its modifiers (`QKeyCombination`):
    /// what KDE's service takes.
    pub fn qt(self) -> i32 {
        const SHIFT: i32 = 0x0200_0000;
        const CTRL: i32 = 0x0400_0000;
        const ALT: i32 = 0x0800_0000;
        const META: i32 = 0x1000_0000;
        // Qt's codes for letters and digits are their upper-case ASCII.
        let mut code = self.key.to_ascii_uppercase() as i32;
        for (on, bit) in [
            (self.shift, SHIFT),
            (self.ctrl, CTRL),
            (self.alt, ALT),
            (self.meta, META),
        ] {
            if on {
                code |= bit;
            }
        }
        code
    }
}

/// One global shortcut.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shortcut {
    /// What the handler is called with when it is pressed; also its ID in
    /// the desktop's settings, so it keeps a changed key.
    pub action: String,
    /// Its name in the desktop's settings, in the current language.
    pub name: String,
    /// Its key unless changed in the desktop's settings.
    pub keys: Keys,
}

/// Called with a [`Shortcut::action`] when its key is pressed.
pub type Handler = Arc<dyn Fn(&str) + Send + Sync>;

#[cfg(not(windows))]
pub use kde::serve;

#[cfg(not(windows))]
mod kde {
    use futures_lite::StreamExt;
    use zbus::fdo::DBusProxy;
    use zbus::message::Type;
    use zbus::names::BusName;
    use zbus::zvariant::OwnedObjectPath;
    use zbus::{Connection, MatchRule, MessageStream};

    use super::{Handler, Shortcut};

    const SERVICE: &str = "org.kde.kglobalaccel";
    const PATH: &str = "/kglobalaccel";
    const INTERFACE: &str = "org.kde.KGlobalAccel";
    const COMPONENT_INTERFACE: &str = "org.kde.kglobalaccel.Component";
    const PRESSED: &str = "globalShortcutPressed";

    /// `KGlobalAccel::SetShortcutFlag`: the key is the one in use; the
    /// key is the default.
    const SET_PRESENT: u32 = 2;
    const IS_DEFAULT: u32 = 8;

    /// A `QKeySequence` on the wire: up to four keys, unused ones 0.
    type Sequence = (Vec<i32>,);

    /// Registers `shortcuts` for the component `component` (its name in
    /// System Settings: `component_name`) whenever KDE's global shortcuts
    /// service runs, and calls `handler` when one is pressed. Returns when
    /// the bus cannot be watched or the connection closes; on a desktop
    /// without the service it waits for one.
    pub async fn serve(
        connection: &Connection,
        component: &str,
        component_name: &str,
        shortcuts: Vec<Shortcut>,
        handler: Handler,
    ) -> zbus::Result<()> {
        let bus = DBusProxy::new(connection).await?;
        let service = BusName::try_from(SERVICE).expect("valid bus name");
        let mut owners = bus
            .receive_name_owner_changed_with_args(&[(0, service.as_str())])
            .await?;
        let rule = MatchRule::builder()
            .msg_type(Type::Signal)
            .interface(COMPONENT_INTERFACE)?
            .member(PRESSED)?
            .build();
        let mut presses = MessageStream::for_match_rule(rule, connection, None).await?;
        if bus.name_has_owner(service.clone()).await.unwrap_or(false) {
            register(connection, component, component_name, &shortcuts).await;
        }
        enum Next {
            /// Whether the service has an owner now.
            Owner(bool),
            Pressed(zbus::Message),
            /// The connection closed: nothing more can come.
            Closed,
        }
        loop {
            let next = futures_lite::future::or(
                async {
                    match owners.next().await {
                        Some(changed) => {
                            Next::Owner(changed.args().is_ok_and(|args| args.new_owner().is_some()))
                        }
                        None => Next::Closed,
                    }
                },
                async {
                    loop {
                        match presses.next().await {
                            Some(Ok(message)) => break Next::Pressed(message),
                            Some(Err(_)) => continue,
                            None => break Next::Closed,
                        }
                    }
                },
            )
            .await;
            match next {
                Next::Owner(true) => {
                    register(connection, component, component_name, &shortcuts).await;
                }
                Next::Owner(false) => {}
                Next::Pressed(message) => {
                    let pressed = message.body().deserialize::<(String, String, i64)>();
                    if let Ok((by, action, _)) = pressed
                        && by == component
                        && shortcuts.iter().any(|s| s.action == action)
                    {
                        tracing::info!(action, "global shortcut");
                        handler(&action);
                    }
                }
                Next::Closed => return Ok(()),
            }
        }
    }

    /// Tells the service about each shortcut and its default key. A key
    /// changed in System Settings stays: the service keeps it and loads it
    /// in place of the one given.
    async fn register(
        connection: &Connection,
        component: &str,
        component_name: &str,
        shortcuts: &[Shortcut],
    ) {
        for shortcut in shortcuts {
            let id = [
                component,
                shortcut.action.as_str(),
                component_name,
                shortcut.name.as_str(),
            ];
            if let Err(err) = register_one(connection, &id, shortcut).await {
                tracing::warn!(%err, action = shortcut.action, "global shortcut not registered");
            }
        }
        // Asked for so the service makes the component's object, which
        // sends the presses.
        let component: zbus::Result<OwnedObjectPath> = connection
            .call_method(
                Some(SERVICE),
                PATH,
                Some(INTERFACE),
                "getComponent",
                &(component,),
            )
            .await
            .and_then(|reply| reply.body().deserialize());
        match component {
            Ok(path) => tracing::debug!(%path, "global shortcuts registered"),
            Err(err) => tracing::warn!(%err, "global shortcuts have no component"),
        }
    }

    async fn register_one(
        connection: &Connection,
        id: &[&str; 4],
        shortcut: &Shortcut,
    ) -> zbus::Result<()> {
        let kglobalaccel: zbus::Proxy<'_> = zbus::proxy::Builder::new(connection)
            .destination(SERVICE)?
            .path(PATH)?
            .interface(INTERFACE)?
            .cache_properties(zbus::proxy::CacheProperties::No)
            .build()
            .await?;
        // A slice goes out as `as`; the array itself would be `(ssss)`.
        let id = id.as_slice();
        kglobalaccel.call_method("doRegister", &(id,)).await?;
        let key = shortcut.keys.qt();
        let sequence: Vec<Sequence> = vec![(vec![key, 0, 0, 0],)];
        for flags in [IS_DEFAULT, SET_PRESENT] {
            // KDE Frameworks 6 takes key sequences; 5 took plain keys.
            let keys = kglobalaccel
                .call_method("setShortcutKeys", &(id, &sequence, flags))
                .await;
            if keys.is_err() {
                kglobalaccel
                    .call_method("setShortcut", &(id, vec![key], flags))
                    .await?;
            }
        }
        Ok(())
    }
}

#[cfg(windows)]
pub use crate::tray::serve_shortcuts as serve;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_combine_as_qt_does() {
        // Qt::META | Qt::ALT | Qt::Key_T
        assert_eq!(Keys::meta_alt('t').qt(), 0x1800_0054);
        assert_eq!(Keys::meta_alt('N').qt(), 0x1800_004e);
        let ctrl_shift_1 = Keys {
            meta: false,
            ctrl: true,
            alt: false,
            shift: true,
            key: '1',
        };
        assert_eq!(ctrl_shift_1.qt(), 0x0600_0031);
    }
}
