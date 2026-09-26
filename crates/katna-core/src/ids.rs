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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_identifiers_are_valid() {
        for id in [MAIL_APP_ID, CALENDAR_APP_ID, DAEMON_BUS_NAME, PIM_INTERFACE] {
            assert!(is_valid_app_id(id), "invalid identifier: {id}");
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
