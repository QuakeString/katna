// SPDX-License-Identifier: GPL-3.0-or-later

//! The files in `packaging/` are named after, and refer to, the IDs in
//! `katna_core::ids`. These tests keep them from drifting apart.

use std::{
    fs,
    path::{Path, PathBuf},
};

use katna_core::ids::{
    CLOCK_APPLET_ID, CLOCK_EXTENSION_UUID, DAEMON_BUS_NAME, MAIL_APP_ID, PREFIX,
    RUNNER_OBJECT_PATH, SEARCH_PROVIDER_OBJECT_PATH, UPDATE_ACTION,
};
use katna_core::update::{ARCH_HELPER, ARCH_INSTALLED};

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
    assert_eq!(
        actions,
        [
            "new-message",
            "inbox",
            "calendar",
            "contacts",
            "preferences"
        ]
    );
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
                "katna-mail --page calendar",
                "katna-mail --page contacts",
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
        if size == "symbolic" {
            // The tray's one-colour icon, which the desktop recolours.
            assert_eq!(names, [format!("{MAIL_APP_ID}-symbolic.svg")]);
            let text = fs::read_to_string(hicolor.join("symbolic/apps").join(&names[0])).unwrap();
            assert!(text.contains("ColorScheme-Text") && text.contains("currentColor"));
            continue;
        }
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

/// "Send with Katna Mail" in Dolphin: files and folders, local only, in the
/// menu itself rather than under Actions, starting `katna-mail --attach`
/// (the name is `katna_platform::file_menus::service_menu_file`, which
/// the daemon's per-user copy with the accounts replaces).
#[test]
fn dolphin_menu_attaches_the_files() {
    let text = read("kio", &format!("{MAIL_APP_ID}.SendFiles.desktop"));
    assert_eq!(value(&text, "Type"), Some("Service"));
    assert_eq!(value(&text, "MimeType"), Some("all/all;"));
    assert_eq!(value(&text, "X-KDE-Protocols"), Some("file"));
    assert_eq!(value(&text, "X-KDE-Priority"), Some("TopLevel"));
    assert_eq!(value(&text, "Name"), Some("Send with Katna Mail"));
    assert_eq!(value(&text, "Icon"), Some(MAIL_APP_ID));
    assert_eq!(value(&text, "Exec"), Some("katna-mail --attach %F"));
    let nautilus = read("nautilus", "katna-mail.py");
    assert!(nautilus.contains("\"katna-mail\", \"--attach\""));
}

/// The Flatpak's ID is the prefix of Katna Mail's and the service's names:
/// Flatpak exports only files named after the app ID or under it (the
/// desktop entry, icons and the D-Bus activation file), and lets the app
/// own only those bus names.
#[test]
fn flatpak_id_is_the_prefix() {
    let text = read("flatpak", &format!("{PREFIX}.yml"));
    assert!(text.contains(&format!("\nid: {PREFIX}\n")), "{text}");
    for id in [MAIL_APP_ID, DAEMON_BUS_NAME] {
        assert!(id.starts_with(&format!("{PREFIX}.")), "{id}");
    }
}

/// The Snap owns Katna Mail's and the service's bus names, and its menu
/// entry and "Start Katna at login" entry are Katna Mail's.
#[test]
fn snap_names_match_ids() {
    let text = read("snap", "snapcraft.yaml");
    for line in [
        format!("    name: {DAEMON_BUS_NAME}\n"),
        format!("    name: {MAIL_APP_ID}\n"),
        format!("    desktop: usr/share/applications/{MAIL_APP_ID}.desktop\n"),
        format!("    common-id: {MAIL_APP_ID}\n"),
        format!("    autostart: {MAIL_APP_ID}.desktop\n"),
    ] {
        assert!(text.contains(&line), "snapcraft.yaml has no {line:?}");
    }
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
        format!("polkit/{UPDATE_ACTION}.policy"),
        format!("kio/{MAIL_APP_ID}.SendFiles.desktop"),
        format!("flatpak/{PREFIX}.yml"),
        "snap/snapcraft.yaml".to_owned(),
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

/// The polkit action lets `pkexec` run the Arch package's update helper,
/// where the PKGBUILD installs it, and nothing else.
#[test]
fn update_action_runs_the_update_helper() {
    let text = read("polkit", &format!("{UPDATE_ACTION}.policy"));
    assert!(
        text.contains(&format!("<action id=\"{UPDATE_ACTION}\">")),
        "{text}"
    );
    assert!(
        text.contains(&format!("<icon_name>{MAIL_APP_ID}</icon_name>")),
        "{text}"
    );
    assert!(
        text.contains(&format!(
            "<annotate key=\"org.freedesktop.policykit.exec.path\">{ARCH_HELPER}</annotate>"
        )),
        "{text}"
    );
    let helper = packaging().join("arch").join("katna-update-helper");
    assert!(helper.is_file(), "{}", helper.display());
    let pkgbuild = fs::read_to_string(packaging().join("arch/PKGBUILD")).unwrap();
    assert!(
        pkgbuild.contains(&format!(
            "packaging/arch/katna-update-helper \"$pkgdir{ARCH_HELPER}\""
        )),
        "the PKGBUILD installs the helper at {ARCH_HELPER}"
    );
}

/// The update helper keeps the installed package where the daemon looks
/// for it to patch from, and removing the package removes that copy.
#[test]
fn installed_copy_is_where_the_daemon_looks() {
    let helper = fs::read_to_string(packaging().join("arch/katna-update-helper")).unwrap();
    assert!(
        helper.contains(&format!("readonly INSTALLED={ARCH_INSTALLED}\n")),
        "the helper keeps the installed package in {ARCH_INSTALLED}"
    );
    let install = fs::read_to_string(packaging().join("arch/katna-git.install")).unwrap();
    assert!(
        install.contains(&format!("rm -rf {ARCH_INSTALLED} ")),
        "{install}"
    );
}

/// Katna Digital Clock's plugin ID and the GNOME extension's UUID, which
/// is also its folder's name (`integrations/`).
#[test]
fn desktop_clock_ids_match() {
    let integrations = packaging().join("../integrations");
    let applet =
        fs::read_to_string(integrations.join("plasma-clock/package/metadata.json")).unwrap();
    assert!(
        applet.contains(&format!("\"Id\": \"{CLOCK_APPLET_ID}\"")),
        "Katna Digital Clock's metadata.json has no Id {CLOCK_APPLET_ID}"
    );
    let extension = integrations
        .join("gnome-shell-extension")
        .join(CLOCK_EXTENSION_UUID);
    let metadata = fs::read_to_string(extension.join("metadata.json")).unwrap();
    assert!(
        metadata.contains(&format!("\"uuid\": \"{CLOCK_EXTENSION_UUID}\"")),
        "the GNOME extension's metadata.json has no uuid {CLOCK_EXTENSION_UUID}"
    );
    let folders = fs::read_dir(integrations.join("gnome-shell-extension"))
        .unwrap()
        .count();
    assert_eq!(
        folders, 1,
        "one GNOME extension, in a folder named by its UUID"
    );
}
