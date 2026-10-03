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

/// Desktop entry that Katna's notifications name (`desktop-entry` hint): a
/// hidden copy of Katna Mail's with `StartupNotify=false`, so a click on a
/// notification button shows no launch feedback (KWin bounces the app's
/// icon for every token a button asks for, though most buttons open no
/// window).
pub const NOTIFICATIONS_DESKTOP_ID: &str = concat!(prefix!(), ".Mail.Notifications");

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

/// Object path of the daemon's KRunner runner (`org.kde.krunner1`), named
/// in its `krunner/dbusplugins` file.
pub const RUNNER_OBJECT_PATH: &str = "/in/invenia/katna/Daemon/Runner";

/// Object path of the daemon's GNOME Shell search provider
/// (`org.gnome.Shell.SearchProvider2`), named in its
/// `gnome-shell/search-providers` file.
pub const SEARCH_PROVIDER_OBJECT_PATH: &str = "/in/invenia/katna/Daemon/SearchProvider";

/// D-Bus interface of the daemon's events and tasks for the desktop's
/// clock (Katna Digital Clock on Plasma, the GNOME Shell extension).
pub const AGENDA_INTERFACE: &str = concat!(prefix!(), ".Agenda1");

/// Object path of [`AGENDA_INTERFACE`], served under [`DAEMON_BUS_NAME`].
pub const AGENDA_OBJECT_PATH: &str = "/in/invenia/katna/Daemon/Agenda";

/// Plugin ID of Katna Digital Clock, the Plasma widget
/// (`integrations/plasma-clock/package/metadata.json`).
pub const CLOCK_APPLET_ID: &str = concat!(prefix!(), ".digitalclock");

/// UUID of Katna's GNOME Shell extension, which adds Katna's events and
/// tasks to the clock's menu (`integrations/gnome-shell-extension/`).
/// GNOME wants an e-mail-like UUID, so it is the project domain.
pub const CLOCK_EXTENSION_UUID: &str = "clock@katna.invenia.in";

/// The polkit action that lets Katna Mail install an update of Katna
/// after the system's password prompt (`packaging/polkit/`).
pub const UPDATE_ACTION: &str = concat!(prefix!(), ".update");

/// Katna's crash tracker: the Sentry project crash reports are sent to,
/// only after the user agrees (`docs/ARCHITECTURE.md` §19.2). A DSN is
/// the project's public address, not a secret. Empty turns sending off;
/// `feedback.dsn` in the settings file can point somewhere else.
pub const SENTRY_DSN: &str = "https://1ebb96bdfbca71ddd5a26968b39d5e47@o4512156164096000.ingest.de.sentry.io/4512156171698256";

/// Katna Server, which records opens and clicks of mail the user chose to
/// track (`docs/ARCHITECTURE.md` §16.1, `server/katna-server`). Empty
/// turns tracking off.
pub const TRACKING_SERVER_URL: &str = "https://server.katna.invenia.in";

/// OAuth2 client ID of Katna's "Desktop app" in Google Cloud, for "Sign in
/// with Google" (`docs/ARCHITECTURE.md` §6.4). Set at build time from
/// `KATNA_GOOGLE_OAUTH_CLIENT_ID` (a GitHub secret for the packages);
/// empty hides the button.
pub const GOOGLE_OAUTH_CLIENT_ID: &str = match option_env!("KATNA_GOOGLE_OAUTH_CLIENT_ID") {
    Some(id) => id,
    None => "",
};

/// The client secret Google gives a desktop app, from
/// `KATNA_GOOGLE_OAUTH_CLIENT_SECRET` at build time. Google says it is not
/// secret for installed apps (PKCE protects the sign-in), but its token
/// endpoint still asks for it; it is kept out of the repository anyway.
pub const GOOGLE_OAUTH_CLIENT_SECRET: &str = match option_env!("KATNA_GOOGLE_OAUTH_CLIENT_SECRET") {
    Some(secret) => secret,
    None => "",
};

/// Application (client) ID of Katna's public client in Microsoft Entra,
/// for "Sign in with Microsoft", from `KATNA_MICROSOFT_OAUTH_CLIENT_ID` at
/// build time. Empty hides the button.
pub const MICROSOFT_OAUTH_CLIENT_ID: &str = match option_env!("KATNA_MICROSOFT_OAUTH_CLIENT_ID") {
    Some(id) => id,
    None => "",
};

/// Client ID of Katna's app in the Zoho API Console, for "Sign in with
/// Zoho" (tasks and calendars), from `KATNA_ZOHO_OAUTH_CLIENT_ID` at build
/// time. Empty hides the button.
pub const ZOHO_OAUTH_CLIENT_ID: &str = match option_env!("KATNA_ZOHO_OAUTH_CLIENT_ID") {
    Some(id) => id,
    None => "",
};

/// The client secret the Zoho API Console gives a server-based app, from
/// `KATNA_ZOHO_OAUTH_CLIENT_SECRET` at build time; like Google's, it cannot
/// stay secret in an app people install (PKCE protects the sign-in), but
/// it is kept out of the repository.
pub const ZOHO_OAUTH_CLIENT_SECRET: &str = match option_env!("KATNA_ZOHO_OAUTH_CLIENT_SECRET") {
    Some(secret) => secret,
    None => "",
};

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

/// Calls `$callback!(interface, bus name, object path)` with the D-Bus names
/// of the daemon's events and tasks ([`AGENDA_INTERFACE`]) as literals, as
/// [`with_dbus_names!`] does for the main API.
#[macro_export]
macro_rules! with_agenda_names {
    ($callback:ident) => {
        $callback!(
            "in.invenia.katna.Agenda1",
            "in.invenia.katna.Daemon",
            "/in/invenia/katna/Daemon/Agenda"
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

    macro_rules! agenda_names {
        ($interface:tt, $bus_name:tt, $path:tt) => {
            assert_eq!($interface, AGENDA_INTERFACE);
            assert_eq!($bus_name, DAEMON_BUS_NAME);
            assert_eq!($path, AGENDA_OBJECT_PATH);
        };
    }

    #[test]
    fn literal_agenda_names_match() {
        with_agenda_names!(agenda_names);
    }

    #[test]
    fn clock_extension_uuid_is_the_project_domain() {
        let domain: Vec<&str> = PREFIX.split('.').rev().collect();
        assert_eq!(CLOCK_EXTENSION_UUID, format!("clock@{}", domain.join(".")));
    }

    #[test]
    fn all_identifiers_are_valid() {
        for id in [
            MAIL_APP_ID,
            CALENDAR_APP_ID,
            DAEMON_BUS_NAME,
            PIM_INTERFACE,
            AGENDA_INTERFACE,
            CLOCK_APPLET_ID,
        ] {
            assert!(is_valid_app_id(id), "invalid identifier: {id}");
        }
    }

    #[test]
    fn object_paths_follow_the_bus_names() {
        let path = |name: &str| format!("/{}", name.replace('.', "/"));
        assert_eq!(MAIL_OBJECT_PATH, path(MAIL_APP_ID));
        assert!(MAIL_MENU_BAR_PATH.starts_with(MAIL_OBJECT_PATH));
        for daemon_path in [
            LAUNCHER_ENTRY_PATH,
            RUNNER_OBJECT_PATH,
            SEARCH_PROVIDER_OBJECT_PATH,
            AGENDA_OBJECT_PATH,
        ] {
            assert!(daemon_path.starts_with(&path(DAEMON_BUS_NAME)));
        }
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
