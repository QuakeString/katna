// SPDX-License-Identifier: GPL-3.0-or-later

//! The session bus every Katna program talks on (`docs/ARCHITECTURE.md`
//! §27.1).
//!
//! On Linux it is the desktop's session bus. Windows has none, so Katna
//! brings its own: `dbus-daemon.exe`, installed beside Katna's programs, is
//! started by the first Katna program that needs it and runs until logout.
//! It listens on a nonce-protected TCP port on 127.0.0.1 that only this
//! user can use, and starts `katna-daemon.exe` when a client calls it, as
//! D-Bus activation does on Linux. Its address is kept in
//! `%LOCALAPPDATA%\Katna\State\bus\address`.

/// Connects to the session bus, starting Katna's own on Windows when it is
/// not running yet.
#[cfg(not(windows))]
pub async fn session() -> zbus::Result<zbus::Connection> {
    zbus::Connection::session().await
}

#[cfg(windows)]
pub use windows::session;

/// The files that start Katna's bus: its configuration, and the activation
/// file of each Katna program the bus starts on demand. `programs` is the
/// folder of Katna's programs; `dir` the folder these files go in.
#[cfg_attr(not(windows), allow(dead_code))]
fn bus_files(
    programs: &std::path::Path,
    dir: &std::path::Path,
) -> Vec<(std::path::PathBuf, String)> {
    use katna_core::ids;
    let services = dir.join("services");
    let config = format!(
        r#"<!DOCTYPE busconfig PUBLIC "-//freedesktop//DTD D-Bus Bus Configuration 1.0//EN"
 "http://www.freedesktop.org/standards/dbus/1.0/busconfig.dtd">
<!-- Written by Katna each time it starts its session bus. -->
<busconfig>
  <type>session</type>
  <listen>nonce-tcp:host=127.0.0.1,bind=127.0.0.1,port=0</listen>
  <auth>EXTERNAL</auth>
  <servicedir>{services}</servicedir>
  <policy context="default">
    <allow send_destination="*" eavesdrop="true"/>
    <allow eavesdrop="true"/>
    <allow own="*"/>
  </policy>
</busconfig>
"#,
        services = xml_text(&services.to_string_lossy()),
    );
    let service = |name: &str, program: &str| {
        let exec = exec_path(&programs.join(program));
        (
            services.join(format!("{name}.service")),
            format!("[D-BUS Service]\nName={name}\nExec={exec}\n"),
        )
    };
    vec![
        (dir.join("katna-bus.conf"), config),
        service(ids::DAEMON_BUS_NAME, "katna-daemon.exe"),
    ]
}

/// `path` for an activation file's `Exec=` line: in double quotes, where
/// dbus-daemon reads a backslash as an escape, so each one is doubled.
#[cfg_attr(not(windows), allow(dead_code))]
fn exec_path(path: &std::path::Path) -> String {
    let text = path.to_string_lossy();
    format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
}

#[cfg_attr(not(windows), allow(dead_code))]
fn xml_text(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(windows)]
mod windows {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::os::windows::process::CommandExt;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};

    use katna_core::Paths;

    /// Starts a console program without opening a console window.
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    pub async fn session() -> zbus::Result<zbus::Connection> {
        // A bus someone set up by hand (for example to debug) comes first.
        if std::env::var_os("DBUS_SESSION_BUS_ADDRESS").is_some() {
            return zbus::Connection::session().await;
        }
        let paths = Paths::from_env().map_err(|err| zbus::Error::Failure(err.to_string()))?;
        let dir = paths.state_dir().join("bus");
        if let Some(connection) = connect_saved(&dir).await {
            return Ok(connection);
        }
        let started = smol::unblock(move || start(&dir)).await;
        let address = started.map_err(|err| zbus::Error::Failure(format!("session bus: {err}")))?;
        zbus::connection::Builder::address(address.as_str())?
            .build()
            .await
    }

    /// The bus whose address is saved in `dir`, if it still runs.
    async fn connect_saved(dir: &Path) -> Option<zbus::Connection> {
        let address = std::fs::read_to_string(dir.join("address")).ok()?;
        zbus::connection::Builder::address(address.trim())
            .ok()?
            .build()
            .await
            .ok()
    }

    /// Starts `dbus-daemon.exe` and saves its address, unless another Katna
    /// program started one meanwhile. Returns the address.
    fn start(dir: &Path) -> std::io::Result<String> {
        std::fs::create_dir_all(dir)?;
        // One starter at a time, so every program ends up on the same bus.
        let lock = File::create(dir.join("lock"))?;
        lock.lock()?;
        let saved = dir.join("address");
        if let Ok(address) = std::fs::read_to_string(&saved)
            && smol::block_on(async {
                zbus::connection::Builder::address(address.trim())
                    .ok()?
                    .build()
                    .await
                    .ok()
            })
            .is_some()
        {
            return Ok(address.trim().to_owned());
        }
        let programs = programs_dir()?;
        for (path, text) in super::bus_files(&programs, dir) {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(path, text)?;
        }
        let mut child = Command::new(programs.join("dbus-daemon.exe"))
            .arg(format!(
                "--config-file={}",
                dir.join("katna-bus.conf").display()
            ))
            .arg("--print-address")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()?;
        let mut address = String::new();
        if let Some(stdout) = child.stdout.take() {
            BufReader::new(stdout).read_line(&mut address)?;
        }
        let address = address.trim().to_owned();
        if address.is_empty() {
            return Err(std::io::Error::other("dbus-daemon.exe did not start"));
        }
        std::fs::write(&saved, &address)?;
        Ok(address)
    }

    /// The folder of Katna's programs: the one this program is in.
    fn programs_dir() -> std::io::Result<PathBuf> {
        let exe = std::env::current_exe()?;
        exe.parent()
            .map(Path::to_path_buf)
            .ok_or_else(|| std::io::Error::other("no folder for this program"))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn activation_files_start_the_daemon_from_the_programs_folder() {
        let files = bus_files(
            Path::new(r"C:\Users\Ada\AppData\Local\Programs\Katna"),
            Path::new("/state/bus"),
        );
        let (path, config) = &files[0];
        assert!(path.ends_with("katna-bus.conf"));
        assert!(config.contains("<listen>nonce-tcp:host=127.0.0.1"));
        let services = Path::new("/state/bus").join("services");
        assert!(config.contains(&format!("<servicedir>{}</servicedir>", services.display())));
        let (path, service) = &files[1];
        assert!(path.ends_with("in.invenia.katna.Daemon.service"));
        assert!(service.contains("Name=in.invenia.katna.Daemon\n"));
        assert!(service.contains(r"katna-daemon.exe"));
        assert!(service.contains(r#"Exec="C:\\Users\\Ada\\AppData\\Local\\Programs\\Katna"#));
    }

    #[test]
    fn exec_paths_are_quoted_for_dbus() {
        assert_eq!(
            exec_path(Path::new(r"C:\Program Files\Katna\katna-daemon.exe")),
            r#""C:\\Program Files\\Katna\\katna-daemon.exe""#
        );
    }
}
