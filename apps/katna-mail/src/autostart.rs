// SPDX-License-Identifier: GPL-3.0-or-later

//! "Start Katna at login": a desktop entry in the XDG autostart folder
//! (`$XDG_CONFIG_HOME/autostart`), which KDE, GNOME and the other desktops
//! start at login. The file is the setting, so turning it off in the
//! desktop's own autostart settings shows here too.
//!
//! By default the entry runs `katna-mail --background`, which starts the
//! Katna service (sync, new-mail notifications, the tray icon) without a
//! window; "Open the window too" drops the flag. It is on by default: the
//! first run after [`General::start_at_login_set`] was added writes it once,
//! and from then on only the switch (or the desktop) changes it.
//!
//! On Windows the setting is the `Katna` value of the user's `Run` key
//! (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`), which Settings >
//! Apps > Startup also shows and turns off.
//!
//! [`General::start_at_login_set`]: katna_core::config::General::start_at_login_set

use std::io;
use std::path::Path;
#[cfg(not(windows))]
use std::path::PathBuf;
use std::process::ExitCode;

use katna_core::ids;

/// The flag that starts the service and leaves the window closed.
pub const BACKGROUND_FLAG: &str = "--background";

/// What starts at login.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Start {
    /// The Katna service only: sync, notifications and the tray icon.
    Quietly,
    /// The Katna Mail window as well.
    Window,
}

#[cfg(not(windows))]
/// The autostart entry's path, or `None` without a home folder.
fn path() -> Option<PathBuf> {
    let absolute = |name: &str| {
        std::env::var_os(name)
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
    };
    let config =
        absolute("XDG_CONFIG_HOME").or_else(|| absolute("HOME").map(|h| h.join(".config")))?;
    Some(
        config
            .join("autostart")
            .join(format!("{}.desktop", ids::MAIL_APP_ID)),
    )
}

#[cfg(not(windows))]
/// What Katna starts at login, or `None` for nothing.
pub fn get() -> Option<Start> {
    let text = std::fs::read_to_string(path()?).ok()?;
    Some(start_of(&text))
}

/// What the entry `text` starts.
#[cfg(not(windows))]
fn start_of(text: &str) -> Start {
    let quiet = text
        .lines()
        .filter_map(|line| line.strip_prefix("Exec="))
        .any(|exec| exec.split_whitespace().any(|word| word == BACKGROUND_FLAG));
    if quiet { Start::Quietly } else { Start::Window }
}

#[cfg(not(windows))]
/// Makes Katna start at login as `start` says, or not at all.
pub fn set(start: Option<Start>) -> io::Result<()> {
    let path = path().ok_or_else(|| io::Error::other("no home folder"))?;
    let Some(start) = start else {
        stop_service_unit();
        return match std::fs::remove_file(&path) {
            Err(err) if err.kind() != io::ErrorKind::NotFound => Err(err),
            _ => Ok(()),
        };
    };
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let exe = std::env::current_exe()?;
    std::fs::write(&path, entry(&exe, start))
}

/// The first run with the setting: Katna starts quietly at login unless it
/// already starts somehow. Returns whether the default is now in place, so
/// it isn't tried again.
pub fn set_default() -> bool {
    if get().is_some() {
        return true;
    }
    match set(Some(Start::Quietly)) {
        Ok(()) => true,
        Err(err) => {
            tracing::warn!(%err, "cannot start Katna at login");
            false
        }
    }
}

#[cfg(not(windows))]
/// Off also means the service's systemd unit, which the install notes used
/// to have people enable, no longer starts it at login. Where there is no
/// systemd, or the unit was never enabled, this does nothing.
fn stop_service_unit() {
    std::thread::spawn(|| {
        let _ = std::process::Command::new("systemctl")
            .args(["--user", "--quiet", "disable", "katna-daemon.service"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
    });
}

/// The user's `Run` key and Katna's value in it.
#[cfg(windows)]
const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
#[cfg(windows)]
const RUN_VALUE: &str = "Katna";

/// What Katna starts at login, or `None` for nothing.
#[cfg(windows)]
pub fn get() -> Option<Start> {
    let key = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER)
        .open_subkey(RUN_KEY)
        .ok()?;
    let command: String = key.get_value(RUN_VALUE).ok()?;
    Some(start_of_command(&command))
}

/// What the `Run` command `command` starts.
#[cfg_attr(not(windows), allow(dead_code))]
fn start_of_command(command: &str) -> Start {
    if command
        .split_whitespace()
        .any(|word| word == BACKGROUND_FLAG)
    {
        Start::Quietly
    } else {
        Start::Window
    }
}

/// The `Run` command that starts `exe` as `start` says.
#[cfg_attr(not(windows), allow(dead_code))]
fn run_command(exe: &Path, start: Start) -> String {
    let program = format!("\"{}\"", exe.display());
    match start {
        Start::Quietly => format!("{program} {BACKGROUND_FLAG}"),
        Start::Window => program,
    }
}

/// Makes Katna start at login as `start` says, or not at all.
#[cfg(windows)]
pub fn set(start: Option<Start>) -> io::Result<()> {
    let (key, _) =
        winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER).create_subkey(RUN_KEY)?;
    let Some(start) = start else {
        return match key.delete_value(RUN_VALUE) {
            Err(err) if err.kind() != io::ErrorKind::NotFound => Err(err),
            _ => Ok(()),
        };
    };
    let exe = std::env::current_exe()?;
    key.set_value(RUN_VALUE, &run_command(&exe, start))
}

/// `katna-mail --background`: starts the Katna service through D-Bus
/// activation (the systemd unit where there is one, so there is only ever
/// one) and exits without a window.
pub fn start_service() -> ExitCode {
    let started = futures_lite::future::block_on(async {
        let connection = katna_dbus::session().await?;
        connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "StartServiceByName",
                &(ids::DAEMON_BUS_NAME, 0u32),
            )
            .await
            .map(drop)
    });
    match started {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("katna-mail: cannot start the Katna service: {err}");
            ExitCode::FAILURE
        }
    }
}

/// The desktop entry that starts `exe` as `start` says.
#[cfg(not(windows))]
fn entry(exe: &Path, start: Start) -> String {
    // The installed program by name, so an update that moves it still
    // starts; a build run from elsewhere by its path, quoted.
    let program = if exe == Path::new("/usr/bin/katna-mail") {
        "katna-mail".to_owned()
    } else {
        let path = exe.display().to_string();
        let escaped = path
            .replace('\\', "\\\\\\\\")
            .replace('"', "\\\\\"")
            .replace('`', "\\\\`")
            .replace('$', "\\\\$");
        format!("\"{escaped}\"")
    };
    let exec = match start {
        Start::Quietly => format!("{program} {BACKGROUND_FLAG}"),
        Start::Window => program,
    };
    format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=Katna Mail\n\
         Comment=Started at login (Settings > General > Desktop)\n\
         Exec={exec}\n\
         Icon={id}\n\
         Terminal=false\n\
         StartupWMClass={id}\n\
         X-GNOME-Autostart-enabled=true\n",
        id = ids::MAIL_APP_ID,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(windows))]
    #[test]
    fn installed_program_starts_by_name() {
        let text = entry(Path::new("/usr/bin/katna-mail"), Start::Window);
        assert!(text.contains("\nExec=katna-mail\n"), "{text}");
        assert!(text.contains(&format!("\nIcon={}\n", ids::MAIL_APP_ID)));
    }

    #[cfg(not(windows))]
    #[test]
    fn other_builds_start_by_quoted_path() {
        let text = entry(Path::new("/home/me/my katna/katna-mail"), Start::Window);
        assert!(
            text.contains("\nExec=\"/home/me/my katna/katna-mail\"\n"),
            "{text}"
        );
    }

    #[cfg(not(windows))]
    #[test]
    fn quiet_start_has_the_flag_and_reads_back() {
        for exe in ["/usr/bin/katna-mail", "/home/me/my katna/katna-mail"] {
            for start in [Start::Quietly, Start::Window] {
                let text = entry(Path::new(exe), start);
                assert_eq!(start_of(&text), start, "{text}");
            }
        }
        let text = entry(Path::new("/usr/bin/katna-mail"), Start::Quietly);
        assert!(text.contains("\nExec=katna-mail --background\n"), "{text}");
    }

    #[test]
    fn windows_run_command_reads_back() {
        let exe = Path::new(r"C:\Users\Ada\AppData\Local\Programs\Katna\katna-mail.exe");
        let quiet = run_command(exe, Start::Quietly);
        assert_eq!(
            quiet,
            r#""C:\Users\Ada\AppData\Local\Programs\Katna\katna-mail.exe" --background"#
        );
        assert_eq!(start_of_command(&quiet), Start::Quietly);
        assert_eq!(
            start_of_command(&run_command(exe, Start::Window)),
            Start::Window
        );
    }

    #[cfg(not(windows))]
    #[test]
    fn an_entry_from_before_opens_the_window() {
        // What "Open Katna Mail at login" wrote before the quiet start.
        let old = "[Desktop Entry]\nType=Application\nExec=katna-mail\n";
        assert_eq!(start_of(old), Start::Window);
    }
}
