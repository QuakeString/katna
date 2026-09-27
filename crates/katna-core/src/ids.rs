// SPDX-License-Identifier: GPL-3.0-or-later

//! Application IDs, D-Bus names and object paths.
//!
//! The prefix is the reversed project domain `katna.invenia.in`
//! (implementation plan, decision D6). It is defined only in [`prefix!`];
//! every other identifier is derived from it. These IDs end up in desktop
//! files, Flatpak and user settings, so they must not change after release.

/// Reverse-DNS prefix for every Katna identifier.
macro_rules! prefix {
    () => {
        "in.invenia.katna"
    };
}

/// Reverse-DNS prefix for every Katna identifier.
pub const PREFIX: &str = prefix!();

/// Application ID of Katna Mail (desktop file, Flatpak, D-Bus activation).
pub const MAIL_APP_ID: &str = concat!(prefix!(), ".Mail");

/// Application ID of Katna Calendar.
pub const CALENDAR_APP_ID: &str = concat!(prefix!(), ".Calendar");

/// Well-known D-Bus name owned by `katna-daemon`.
pub const DAEMON_BUS_NAME: &str = concat!(prefix!(), ".Daemon");

/// D-Bus interface of the daemon API.
pub const PIM_INTERFACE: &str = concat!(prefix!(), ".Pim1");

/// D-Bus object path of the daemon API.
pub const PIM_OBJECT_PATH: &str = "/in/invenia/katna/Pim1";

/// Object path of Katna Mail's `org.freedesktop.Application` interface,
/// served under the bus name [`MAIL_APP_ID`] while the app runs.
pub const MAIL_OBJECT_PATH: &str = "/in/invenia/katna/Mail";

/// Object path of Katna Mail's menu bar (`com.canonical.dbusmenu`), which
/// the KDE global menu shows.
pub const MAIL_MENU_BAR_PATH: &str = "/in/invenia/katna/Mail/MenuBar";

/// Object path of the daemon's `com.canonical.Unity.LauncherEntry`, the
/// unread count on Katna Mail's taskbar icon.
pub const LAUNCHER_ENTRY_PATH: &str = "/in/invenia/katna/Daemon/LauncherEntry";

/// Katna's crash tracker: the Sentry project crash reports are sent to,
/// only after the user agrees (`docs/ARCHITECTURE.md` §19.2). A DSN is
/// the project's public address, not a secret. Empty turns sending off;
/// `feedback.dsn` in the settings file can point somewhere else.
pub const SENTRY_DSN: &str = "https://1ebb96bdfbca71ddd5a26968b39d5e47@o4512156164096000.ingest.de.sentry.io/4512156171698256";

/// Returns whether `id` is usable as an application ID, D-Bus well-known name
/// and D-Bus interface name at the same time.
///
/// Rules (strictest of the three): at least three dot-separated elements, each
/// non-empty, made of ASCII letters, digits and `_`, not starting with a digit,
/// and at most 255 bytes in total.
pub fn is_valid_app_id(id: &str) -> bool {
    if id.is_empty() || id.len() > 255 {
        return false;
    }
    let elements: Vec<&str> = id.split('.').collect();
    elements.len() >= 3
        && elements.iter().all(|element| {
            let mut chars = element.chars();
            chars
                .next()
                .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
                && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        })
}

/// Calls `$callback!(interface, bus name, object path)` with the D-Bus names
/// of the daemon as string literals.
///
/// zbus's `#[proxy]` and `#[interface]` attributes need literals, not
/// constants; this keeps those literals here, next to the constants a test
/// checks them against.
#[macro_export]
macro_rules! with_dbus_names {
    ($callback:ident) => {
        $callback!(
            "in.invenia.katna.Pim1",
            "in.invenia.katna.Daemon",
            "/in/invenia/katna/Pim1"
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! names {
        ($interface:tt, $bus_name:tt, $path:tt) => {
            assert_eq!($interface, PIM_INTERFACE);
            assert_eq!($bus_name, DAEMON_BUS_NAME);
            assert_eq!($path, PIM_OBJECT_PATH);
        };
    }

    #[test]
    fn literal_dbus_names_match() {
        with_dbus_names!(names);
    }

    #[test]
    fn all_identifiers_are_valid() {
        for id in [MAIL_APP_ID, CALENDAR_APP_ID, DAEMON_BUS_NAME, PIM_INTERFACE] {
            assert!(is_valid_app_id(id), "invalid identifier: {id}");
        }
    }

    #[test]
    fn object_paths_follow_the_bus_names() {
        let path = |name: &str| format!("/{}", name.replace('.', "/"));
        assert_eq!(MAIL_OBJECT_PATH, path(MAIL_APP_ID));
        assert!(MAIL_MENU_BAR_PATH.starts_with(MAIL_OBJECT_PATH));
        assert!(LAUNCHER_ENTRY_PATH.starts_with(&path(DAEMON_BUS_NAME)));
    }

    #[test]
    fn identifiers_use_project_domain() {
        assert_eq!(MAIL_APP_ID, "in.invenia.katna.Mail");
        assert_eq!(CALENDAR_APP_ID, "in.invenia.katna.Calendar");
        assert_eq!(DAEMON_BUS_NAME, "in.invenia.katna.Daemon");
        assert_eq!(PIM_INTERFACE, "in.invenia.katna.Pim1");
    }

    #[test]
    fn object_path_matches_prefix() {
        let expected = format!("/{}/Pim1", PREFIX.replace('.', "/"));
        assert_eq!(PIM_OBJECT_PATH, expected);
    }

    #[test]
    fn rejects_invalid_ids() {
        for id in [
            "",
            "org.katna",
            "org..Mail",
            "org.katna.9Mail",
            "org.katna-app.Mail",
            "org.katna.Mail.",
        ] {
            assert!(!is_valid_app_id(id), "accepted invalid identifier: {id:?}");
        }
    }

    #[test]
    fn accepts_code_hosting_ids() {
        assert!(is_valid_app_id("io.github.quakestring.KatnaMail"));
        assert!(is_valid_app_id("app.katna.Mail"));
    }
}
