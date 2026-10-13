// SPDX-License-Identifier: GPL-3.0-or-later

//! `katna-daemon install-user-service`: a systemd user unit and a D-Bus
//! activation file for the running binary, and the files that put Katna's
//! search in KRunner and GNOME Shell, until distribution packages install
//! them system-wide (`docs/ARCHITECTURE.md` §9.2, §15.3).

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use katna_core::ids;

/// Name of the systemd user unit.
pub const UNIT: &str = "katna-daemon.service";

/// The systemd user unit for `exe`.
pub fn unit(exe: &Path) -> String {
    format!(
        "[Unit]\n\
         Description=Katna mail and calendar service\n\
         \n\
         [Service]\n\
         Type=dbus\n\
         BusName={bus}\n\
         ExecStart={exe}\n\
         Restart=on-failure\n\
         RestartSec=5\n\
         \n\
         [Install]\n\
         WantedBy=default.target\n",
        bus = ids::DAEMON_BUS_NAME,
        exe = exe.display(),
    )
}

/// The D-Bus activation file for `exe`. With systemd, activation starts
/// the unit, so there is only ever one daemon.
pub fn dbus_service(exe: &Path) -> String {
    format!(
        "[D-BUS Service]\n\
         Name={bus}\n\
         Exec={exe}\n\
         SystemdService={UNIT}\n",
        bus = ids::DAEMON_BUS_NAME,
        exe = exe.display(),
    )
}

/// KRunner's file for the daemon's runner (`krunner/dbusplugins/`).
pub fn krunner_plugin() -> String {
    format!(
        "[Desktop Entry]\n\
         Type=Service\n\
         Name=Katna Mail\n\
         Comment=Mail and contacts from Katna Mail\n\
         Icon={app}\n\
         X-KDE-ServiceTypes=Plasma/Runner\n\
         X-KDE-PluginInfo-Name={app}\n\
         X-KDE-PluginInfo-Author=Mozammel Hossain\n\
         X-KDE-PluginInfo-License=GPL-3.0-or-later\n\
         X-KDE-PluginInfo-EnabledByDefault=true\n\
         X-Plasma-API=DBus\n\
         X-Plasma-API-Minimum-Version=2.0\n\
         X-Plasma-DBusRunner-Service={bus}\n\
         X-Plasma-DBusRunner-Path={path}\n\
         X-Plasma-Request-Actions-Once=true\n\
         X-Plasma-Runner-Min-Letter-Count=3\n",
        app = ids::MAIL_APP_ID,
        bus = ids::DAEMON_BUS_NAME,
        path = ids::RUNNER_OBJECT_PATH,
    )
}

/// GNOME Shell's file for the daemon's search provider
/// (`gnome-shell/search-providers/`).
pub fn search_provider() -> String {
    format!(
        "[Shell Search Provider]\n\
         DesktopId={app}.desktop\n\
         BusName={bus}\n\
         ObjectPath={path}\n\
         Version=2\n",
        app = ids::MAIL_APP_ID,
        bus = ids::DAEMON_BUS_NAME,
        path = ids::SEARCH_PROVIDER_OBJECT_PATH,
    )
}

/// Writes every file under `config_home` and `data_home` (normally
/// `~/.config` and `~/.local/share`). Returns their paths.
pub fn install(exe: &Path, config_home: &Path, data_home: &Path) -> io::Result<[PathBuf; 4]> {
    let unit_path = config_home.join("systemd/user").join(UNIT);
    let service_path = data_home
        .join("dbus-1/services")
        .join(format!("{}.service", ids::DAEMON_BUS_NAME));
    let runner_path = data_home
        .join("krunner/dbusplugins")
        .join(format!("{}.desktop", ids::MAIL_APP_ID));
    let provider_path = data_home
        .join("gnome-shell/search-providers")
        .join(format!("{}.search-provider.ini", ids::MAIL_APP_ID));
    for (path, text) in [
        (&unit_path, unit(exe)),
        (&service_path, dbus_service(exe)),
        (&runner_path, krunner_plugin()),
        (&provider_path, search_provider()),
    ] {
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(path, text)?;
    }
    Ok([unit_path, service_path, runner_path, provider_path])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_unit_and_activation_file() {
        let tmp = tempfile::tempdir().unwrap();
        let exe = Path::new("/usr/local/bin/katna-daemon");
        let [unit_path, service_path, runner_path, provider_path] =
            install(exe, &tmp.path().join("config"), &tmp.path().join("data")).unwrap();
        assert!(runner_path.ends_with("krunner/dbusplugins/in.invenia.katna.Mail.desktop"));
        assert!(
            provider_path.ends_with(
                "gnome-shell/search-providers/in.invenia.katna.Mail.search-provider.ini"
            )
        );
        let runner = fs::read_to_string(runner_path).unwrap();
        assert!(
            runner.contains("X-Plasma-DBusRunner-Path=/in/invenia/katna/Daemon/Runner\n"),
            "{runner}"
        );
        let unit = fs::read_to_string(unit_path).unwrap();
        assert!(unit.contains("BusName=in.invenia.katna.Daemon\n"), "{unit}");
        assert!(unit.contains("ExecStart=/usr/local/bin/katna-daemon\n"));
        assert!(service_path.ends_with("dbus-1/services/in.invenia.katna.Daemon.service"));
        let service = fs::read_to_string(service_path).unwrap();
        assert!(
            service.contains("SystemdService=katna-daemon.service\n"),
            "{service}"
        );
    }

    /// The files distribution packages install (`packaging/`) say the same
    /// as the ones this command writes, apart from comments.
    #[test]
    fn matches_packaged_files() {
        let packaging = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packaging");
        let without_comments = |path: PathBuf| {
            let text =
                fs::read_to_string(&path).unwrap_or_else(|err| panic!("{}: {err}", path.display()));
            text.lines()
                .filter(|line| !line.starts_with('#'))
                .map(|line| format!("{line}\n"))
                .collect::<String>()
        };
        let exe = Path::new("/usr/bin/katna-daemon");
        assert_eq!(
            without_comments(packaging.join("systemd").join(UNIT)),
            unit(exe)
        );
        assert_eq!(
            without_comments(
                packaging
                    .join("dbus")
                    .join(format!("{}.service", ids::DAEMON_BUS_NAME))
            ),
            dbus_service(exe)
        );
        assert_eq!(
            without_comments(
                packaging
                    .join("krunner")
                    .join(format!("{}.desktop", ids::MAIL_APP_ID))
            ),
            krunner_plugin()
        );
        assert_eq!(
            without_comments(
                packaging
                    .join("gnome-shell")
                    .join(format!("{}.search-provider.ini", ids::MAIL_APP_ID))
            ),
            search_provider()
        );
    }
}
