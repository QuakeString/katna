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
//! - **GNOME and other desktops:** the GlobalShortcuts portal
//!   (`org.freedesktop.portal.GlobalShortcuts`). The first time, the
//!   desktop asks whether to allow them, and its settings can change the
//!   keys (GNOME: Settings > Apps > Katna Mail). A desktop without the
//!   portal gets none.

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

    /// The key as the XDG shortcuts spec writes it (`LOGO+ALT+t`): what
    /// the GlobalShortcuts portal takes as a preferred trigger.
    pub fn xdg(self) -> String {
        let mut parts: Vec<String> = [
            (self.ctrl, "CTRL"),
            (self.alt, "ALT"),
            (self.shift, "SHIFT"),
            (self.meta, "LOGO"),
        ]
        .into_iter()
        .filter(|(on, _)| *on)
        .map(|(_, name)| name.to_owned())
        .collect();
        // Keysym names of letters are lower case; of digits, the digit.
        parts.push(self.key.to_ascii_lowercase().to_string());
        parts.join("+")
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

/// Registers `shortcuts` with the desktop for as long as `connection`
/// lasts, and calls `handler` when one is pressed: KDE's global shortcuts
/// service on Plasma, the GlobalShortcuts portal elsewhere. `component` is
/// the app's ID; `component_name` its name in Plasma's settings.
#[cfg(not(windows))]
pub async fn serve(
    connection: &zbus::Connection,
    component: &str,
    component_name: &str,
    shortcuts: Vec<Shortcut>,
    handler: Handler,
) -> zbus::Result<()> {
    if on_plasma() {
        return kde::serve(connection, component, component_name, shortcuts, handler).await;
    }
    // A connection of its own: the portal learns the app's ID from the
    // connection, which has to say it before any other portal call.
    let own = zbus::connection::Builder::session()?.build().await?;
    portal::serve(&own, component, shortcuts, handler).await
}

/// Whether this is a Plasma session, whose shortcuts are KDE's service's
/// (other desktops may start that service too, as part of KDE apps, but
/// cannot take keys through it).
#[cfg(not(windows))]
fn on_plasma() -> bool {
    std::env::var("XDG_CURRENT_DESKTOP").is_ok_and(|desktops| {
        desktops
            .split(':')
            .any(|desktop| desktop.eq_ignore_ascii_case("KDE"))
    })
}

#[cfg(not(windows))]
pub mod portal {
    //! The GlobalShortcuts portal: a session holds the shortcuts, bound
    //! once (the desktop asks the user the first time) and reported by
    //! `Activated` while the session lasts.

    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU32, Ordering};

    use futures_lite::StreamExt;
    use zbus::message::Type;
    use zbus::zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Value};
    use zbus::{Connection, MatchRule, MessageStream};

    use super::{Handler, Shortcut};

    const SERVICE: &str = "org.freedesktop.portal.Desktop";
    const PATH: &str = "/org/freedesktop/portal/desktop";
    const INTERFACE: &str = "org.freedesktop.portal.GlobalShortcuts";
    const REQUEST: &str = "org.freedesktop.portal.Request";
    const REGISTRY: &str = "org.freedesktop.host.portal.Registry";

    /// What a portal call answers in its request's `Response`.
    type Results = HashMap<String, OwnedValue>;

    /// Binds `shortcuts` for the app `app_id` through the portal and calls
    /// `handler` when one is pressed. `connection` is used for nothing
    /// else before this. Returns at once when the desktop has no such
    /// portal or refuses, otherwise when the connection closes.
    pub async fn serve(
        connection: &Connection,
        app_id: &str,
        shortcuts: Vec<Shortcut>,
        handler: Handler,
    ) -> zbus::Result<()> {
        // An app outside Flatpak names itself, so the desktop knows whose
        // shortcuts these are; an older portal has no registry.
        let registered = connection
            .call_method(
                Some(SERVICE),
                PATH,
                Some(REGISTRY),
                "Register",
                &(app_id, HashMap::<&str, Value<'_>>::new()),
            )
            .await;
        if let Err(err) = registered {
            tracing::debug!(%err, "portal registry");
        }
        let version: zbus::Result<OwnedValue> = connection
            .call_method(
                Some(SERVICE),
                PATH,
                Some("org.freedesktop.DBus.Properties"),
                "Get",
                &(INTERFACE, "version"),
            )
            .await
            .and_then(|reply| reply.body().deserialize());
        if let Err(err) = version {
            tracing::info!(%err, "no GlobalShortcuts portal: no global shortcuts");
            return Ok(());
        }
        // Listened for first, so a press right after binding is not lost.
        let rule = MatchRule::builder()
            .msg_type(Type::Signal)
            .interface(INTERFACE)?
            .member("Activated")?
            .build();
        let mut presses = MessageStream::for_match_rule(rule, connection, None).await?;

        let token = next_token();
        let session_token = next_token();
        let options: HashMap<&str, Value<'_>> = HashMap::from([
            ("handle_token", Value::from(token.as_str())),
            ("session_handle_token", Value::from(session_token.as_str())),
        ]);
        let created = request(connection, &token, "CreateSession", &(options,)).await?;
        let Some(session) = created.and_then(|results| session_handle(&results)) else {
            tracing::warn!("the GlobalShortcuts portal made no session");
            return Ok(());
        };

        let list: Vec<(String, HashMap<String, Value<'static>>)> = shortcuts
            .iter()
            .map(|shortcut| {
                (
                    shortcut.action.clone(),
                    HashMap::from([
                        ("description".to_owned(), Value::from(shortcut.name.clone())),
                        (
                            "preferred_trigger".to_owned(),
                            Value::from(shortcut.keys.xdg()),
                        ),
                    ]),
                )
            })
            .collect();
        let bind_token = next_token();
        let options: HashMap<&str, Value<'_>> =
            HashMap::from([("handle_token", Value::from(bind_token.as_str()))]);
        let bound = request(
            connection,
            &bind_token,
            "BindShortcuts",
            &(&session, list, "", options),
        )
        .await?;
        if bound.is_none() {
            tracing::info!("global shortcuts not allowed");
            return Ok(());
        }
        tracing::debug!(%session, "global shortcuts bound");

        while let Some(message) = presses.next().await {
            let Ok(message) = message else { continue };
            let activated =
                message
                    .body()
                    .deserialize::<(OwnedObjectPath, String, u64, HashMap<String, OwnedValue>)>();
            if let Ok((by, action, _, _)) = activated
                && by == session
                && shortcuts.iter().any(|s| s.action == action)
            {
                tracing::info!(action, "global shortcut");
                handler(&action);
            }
        }
        Ok(())
    }

    /// Calls `method` with `args`, whose options carry `token`, and waits
    /// for its request's `Response`: the results when it went through,
    /// `None` when the user or the desktop said no.
    async fn request<A>(
        connection: &Connection,
        token: &str,
        method: &str,
        args: &A,
    ) -> zbus::Result<Option<Results>>
    where
        A: serde::Serialize + zbus::zvariant::DynamicType,
    {
        let path = request_path(connection, token)?;
        // Subscribed before the call: the answer can come before the call
        // returns.
        let rule = MatchRule::builder()
            .msg_type(Type::Signal)
            .interface(REQUEST)?
            .member("Response")?
            .path(path.clone())?
            .build();
        let mut responses = MessageStream::for_match_rule(rule, connection, None).await?;
        connection
            .call_method(Some(SERVICE), PATH, Some(INTERFACE), method, args)
            .await?;
        while let Some(message) = responses.next().await {
            let Ok(message) = message else { continue };
            let (response, results) = message.body().deserialize::<(u32, Results)>()?;
            return Ok((response == 0).then_some(results));
        }
        Ok(None)
    }

    /// Where the portal makes the request for `token` on this connection.
    fn request_path(connection: &Connection, token: &str) -> zbus::Result<OwnedObjectPath> {
        let sender = connection
            .unique_name()
            .map(|name| name.trim_start_matches(':').replace('.', "_"))
            .unwrap_or_default();
        let path = format!("{PATH}/request/{sender}/{token}");
        Ok(ObjectPath::try_from(path)?.into())
    }

    /// The session a `CreateSession` made: a string in the spec, an
    /// object path from some portals.
    fn session_handle(results: &Results) -> Option<OwnedObjectPath> {
        let value = results.get("session_handle")?;
        if let Ok(path) = OwnedObjectPath::try_from(value.try_clone().ok()?) {
            return Some(path);
        }
        let text = String::try_from(value.try_clone().ok()?).ok()?;
        ObjectPath::try_from(text).ok().map(Into::into)
    }

    /// A handle token unique to this process.
    fn next_token() -> String {
        static NEXT: AtomicU32 = AtomicU32::new(0);
        format!(
            "katna_{}_{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        )
    }
}

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

    #[test]
    fn keys_read_as_the_xdg_spec_writes_them() {
        assert_eq!(Keys::meta_alt('T').xdg(), "ALT+LOGO+t");
        let ctrl_shift_1 = Keys {
            meta: false,
            ctrl: true,
            alt: false,
            shift: true,
            key: '1',
        };
        assert_eq!(ctrl_shift_1.xdg(), "CTRL+SHIFT+1");
    }
}
