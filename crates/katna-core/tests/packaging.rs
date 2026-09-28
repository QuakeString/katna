// SPDX-License-Identifier: GPL-3.0-or-later

//! The files in `packaging/` are named after, and refer to, the IDs in
//! `katna_core::ids`. These tests keep them from drifting apart.

use std::{
    fs,
    path::{Path, PathBuf},
};

use katna_core::ids::{
    DAEMON_BUS_NAME, MAIL_APP_ID, PREFIX, RUNNER_OBJECT_PATH, SEARCH_PROVIDER_OBJECT_PATH,
};

/// Name of the systemd user unit (also `katna_daemon::install::UNIT`).
const UNIT: &str = "katna-daemon.service";

fn packaging() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packaging")
}

/// Reads `packaging/<dir>/<name>`; it must be the only file in `dir`
/// with that extension, so a stale file under an old name cannot hide.
fn read(dir: &str, name: &str) -> String {
    let dir = packaging().join(dir);
    let extension = Path::new(name).extension();
    let mut names: Vec<String> = fs::read_dir(&dir)
        .unwrap_or_else(|err| panic!("{}: {err}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension() == extension)
        .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    names.sort();
    assert_eq!(names, [name], "files in {}", dir.display());
    let text = fs::read_to_string(dir.join(name)).unwrap();
    assert!(
        text.lines()
            .take(2)
            .any(|line| line.contains("SPDX-License-Identifier: GPL-3.0-or-later")),
        "{name} has no SPDX header"
    );
    text
}

/// The `key=value` lines of a desktop-entry style file, without comments.
fn entries(text: &str) -> Vec<(&str, &str)> {
    text.lines()
        .filter(|line| !line.starts_with('#') && !line.starts_with('['))
        .filter_map(|line| line.split_once('='))
        .collect()
}

fn value<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    entries(text)
        .into_iter()
        .find_map(|(k, v)| (k == key).then_some(v))
}

#[test]
fn desktop_entry_matches_app_id() {
    let text = read("desktop", &format!("{MAIL_APP_ID}.desktop"));
    assert!(text.contains("\n[Desktop Entry]\n"), "{text}");
    assert_eq!(value(&text, "Name"), Some("Katna Mail"));
    // `%u`: a `mailto:` link when Katna Mail is the default mail app.
    assert_eq!(value(&text, "Exec"), Some("katna-mail %u"));
    assert_eq!(value(&text, "MimeType"), Some("x-scheme-handler/mailto;"));
    assert_eq!(value(&text, "Icon"), Some(MAIL_APP_ID));
    assert_eq!(value(&text, "StartupWMClass"), Some(MAIL_APP_ID));
}

/// The actions on the taskbar icon's right-click menu start Katna Mail with
/// a flag it knows (`katna_dbus::app_action::flag`).
#[test]
fn desktop_actions_run_katna_mail_with_a_flag() {
    let text = read("desktop", &format!("{MAIL_APP_ID}.desktop"));
    let actions: Vec<&str> = value(&text, "Actions")
        .unwrap()
        .split(';')
        .filter(|a| !a.is_empty())
        .collect();
    assert_eq!(actions, ["new-message", "inbox", "preferences"]);
    let groups: Vec<&str> = text.split("\n[").skip(1).collect();
    for action in actions {
        let group = groups
            .iter()
            .find(|g| g.starts_with(&format!("Desktop Action {action}]")))
            .unwrap_or_else(|| panic!("no group for {action}"));
        assert!(group.contains("\nName="), "{action} has no name");
        let exec = entries(group)
            .into_iter()
            .find_map(|(k, v)| (k == "Exec").then_some(v))
            .unwrap();
        assert!(
            [
                "katna-mail --compose",
                "katna-mail --inbox",
                "katna-mail --settings"
            ]
            .contains(&exec),
            "{action}: {exec}"
        );
    }
}

#[test]
fn icon_is_named_after_app_id() {
    let text = read("icons", &format!("{MAIL_APP_ID}.svg"));
    assert!(text.contains("<svg"), "not an SVG");
    // Qt SVG, which draws icons on KDE, skips filters.
    assert!(!text.contains("<filter"), "the installed icon has filters");
    let hicolor = packaging().join("icons/hicolor");
    let mut sizes = 0;
    for dir in fs::read_dir(&hicolor).unwrap() {
        let size = dir.unwrap().file_name().to_string_lossy().into_owned();
        let names: Vec<String> = fs::read_dir(hicolor.join(&size).join("apps"))
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, [format!("{MAIL_APP_ID}.png")], "{size}");
        sizes += 1;
    }
    assert!(sizes >= 4, "{sizes} PNG sizes");
}

#[test]
fn dbus_activation_file_matches_bus_name() {
    let text = read("dbus", &format!("{DAEMON_BUS_NAME}.service"));
    assert!(text.contains("\n[D-BUS Service]\n"), "{text}");
    assert_eq!(value(&text, "Name"), Some(DAEMON_BUS_NAME));
    assert_eq!(value(&text, "Exec"), Some("/usr/bin/katna-daemon"));
    assert_eq!(value(&text, "SystemdService"), Some(UNIT));
}

#[test]
fn systemd_unit_matches_bus_name() {
    let text = read("systemd", UNIT);
    assert_eq!(value(&text, "Type"), Some("dbus"));
    assert_eq!(value(&text, "BusName"), Some(DAEMON_BUS_NAME));
    assert_eq!(value(&text, "ExecStart"), Some("/usr/bin/katna-daemon"));
}

/// KRunner asks the daemon's runner for results (`krunner/dbusplugins`).
#[test]
fn krunner_plugin_names_the_runner() {
    let text = read("krunner", &format!("{MAIL_APP_ID}.desktop"));
    assert!(text.contains("\n[Desktop Entry]\n"), "{text}");
    assert_eq!(value(&text, "X-Plasma-API"), Some("DBus"));
    assert_eq!(
        value(&text, "X-Plasma-DBusRunner-Service"),
        Some(DAEMON_BUS_NAME)
    );
    assert_eq!(
        value(&text, "X-Plasma-DBusRunner-Path"),
        Some(RUNNER_OBJECT_PATH)
    );
}

/// GNOME Shell asks the daemon's search provider for results, under Katna
/// Mail's name and icon.
#[test]
fn search_provider_names_the_provider() {
    let text = read("gnome-shell", &format!("{MAIL_APP_ID}.search-provider.ini"));
    assert!(text.contains("\n[Shell Search Provider]\n"), "{text}");
    assert_eq!(
        value(&text, "DesktopId"),
        Some(format!("{MAIL_APP_ID}.desktop").as_str())
    );
    assert_eq!(value(&text, "BusName"), Some(DAEMON_BUS_NAME));
    assert_eq!(
        value(&text, "ObjectPath"),
        Some(SEARCH_PROVIDER_OBJECT_PATH)
    );
    assert_eq!(value(&text, "Version"), Some("2"));
}

/// Other packaging files (`packaging/*/*`) use the IDs only through file
/// names (the PKGBUILD installs with globs), so they never need changing.
/// Subdirectories are makepkg output and are not checked.
#[test]
fn prefix_only_in_checked_files() {
    let checked = [
        format!("desktop/{MAIL_APP_ID}.desktop"),
        format!("dbus/{DAEMON_BUS_NAME}.service"),
        format!("systemd/{UNIT}"),
        format!("krunner/{MAIL_APP_ID}.desktop"),
        format!("gnome-shell/{MAIL_APP_ID}.search-provider.ini"),
    ];
    for dir in fs::read_dir(packaging()).unwrap() {
        let dir = dir.unwrap().path();
        if !dir.is_dir() {
            continue;
        }
        for file in fs::read_dir(&dir).unwrap() {
            let path = file.unwrap().path();
            let relative = path
                .strip_prefix(packaging())
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            if path.is_dir() || checked.contains(&relative) {
                continue;
            }
            // Built packages are not text.
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            assert!(
                !text.contains(PREFIX),
                "{relative} hard-codes {PREFIX}; use a glob or add a check here"
            );
        }
    }
}
