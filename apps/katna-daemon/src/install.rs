// SPDX-License-Identifier: GPL-3.0-or-later

//! `katna-daemon install-user-service`: a systemd user unit and a D-Bus
//! activation file for the running binary, until distribution packages
//! install them system-wide (`docs/ARCHITECTURE.md` §9.2).

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

/// Writes both files under `config_home` and `data_home` (normally
/// `~/.config` and `~/.local/share`). Returns their paths.
pub fn install(exe: &Path, config_home: &Path, data_home: &Path) -> io::Result<[PathBuf; 2]> {
    let unit_path = config_home.join("systemd/user").join(UNIT);
    let service_path = data_home
        .join("dbus-1/services")
        .join(format!("{}.service", ids::DAEMON_BUS_NAME));
    for (path, text) in [(&unit_path, unit(exe)), (&service_path, dbus_service(exe))] {
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(path, text)?;
    }
    Ok([unit_path, service_path])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_unit_and_activation_file() {
        let tmp = tempfile::tempdir().unwrap();
        let exe = Path::new("/usr/local/bin/katna-daemon");
        let [unit_path, service_path] =
            install(exe, &tmp.path().join("config"), &tmp.path().join("data")).unwrap();
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
}
