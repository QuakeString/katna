// SPDX-License-Identifier: GPL-3.0-or-later

//! The unread count on the app's taskbar or dock icon
//! (`docs/ARCHITECTURE.md` §15.2), through `com.canonical.Unity.LauncherEntry`.
//! Plasma's task manager, Dash to Dock, Ubuntu Dock and Dash to Panel listen
//! for its `Update` signal and match the app by its desktop file; the count
//! goes away when the sender leaves the bus.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use zbus::Connection;
use zbus::object_server::SignalEmitter;
use zbus::zvariant::{OwnedValue, Value};

/// What the launcher shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Shown {
    count: u64,
}

impl Shown {
    fn properties(self) -> HashMap<&'static str, OwnedValue> {
        let count = i64::try_from(self.count).unwrap_or(i64::MAX);
        [
            ("count", Value::from(count)),
            ("count-visible", Value::from(self.count > 0)),
        ]
        .into_iter()
        .filter_map(|(name, value)| Some((name, OwnedValue::try_from(value).ok()?)))
        .collect()
    }
}

struct EntryObject {
    app_uri: String,
    shown: Arc<Mutex<Shown>>,
}

#[zbus::interface(name = "com.canonical.Unity.LauncherEntry")]
impl EntryObject {
    /// What docks that start after the last `Update` ask for.
    #[zbus(out_args("app_uri", "properties"))]
    fn query(&self) -> (String, HashMap<&'static str, OwnedValue>) {
        (
            self.app_uri.clone(),
            self.shown.lock().unwrap().properties(),
        )
    }

    #[zbus(signal)]
    async fn update(
        emitter: &SignalEmitter<'_>,
        app_uri: &str,
        properties: HashMap<&'static str, OwnedValue>,
    ) -> zbus::Result<()>;
}

/// The launcher entry of one app.
pub struct LauncherEntry {
    connection: Connection,
    path: String,
    app_uri: String,
    shown: Arc<Mutex<Shown>>,
}

/// `application://ID.desktop`, how launchers name an app.
pub fn app_uri(desktop_id: &str) -> String {
    format!("application://{desktop_id}.desktop")
}

impl LauncherEntry {
    /// Serves the entry for the app with desktop file `desktop_id` (its app
    /// ID) at `path`. Nothing shows until [`LauncherEntry::set_count`].
    pub async fn serve(
        connection: &Connection,
        path: &str,
        desktop_id: &str,
    ) -> zbus::Result<Self> {
        let shown = Arc::new(Mutex::new(Shown::default()));
        let app_uri = app_uri(desktop_id);
        let object = EntryObject {
            app_uri: app_uri.clone(),
            shown: shown.clone(),
        };
        connection.object_server().at(path, object).await?;
        Ok(Self {
            connection: connection.clone(),
            path: path.to_owned(),
            app_uri,
            shown,
        })
    }

    /// Shows `count` on the icon, or nothing for 0. Sent even when it did
    /// not change, for panels that started since the last time.
    pub async fn set_count(&self, count: u64) -> zbus::Result<()> {
        let shown = Shown { count };
        *self.shown.lock().unwrap() = shown;
        let emitter = SignalEmitter::new(&self.connection, self.path.as_str())?;
        EntryObject::update(&emitter, &self.app_uri, shown.properties()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_the_desktop_file() {
        assert_eq!(
            app_uri("in.invenia.katna.Mail"),
            "application://in.invenia.katna.Mail.desktop"
        );
    }

    #[test]
    fn hides_a_zero_count() {
        let props = Shown { count: 0 }.properties();
        assert_eq!(props["count-visible"], OwnedValue::from(false));
        let props = Shown { count: 12 }.properties();
        assert_eq!(props["count"], OwnedValue::from(12i64));
        assert_eq!(props["count-visible"], OwnedValue::from(true));
    }
}
