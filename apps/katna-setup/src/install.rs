// SPDX-License-Identifier: GPL-3.0-or-later

//! What Setup does (`docs/ARCHITECTURE.md` §27.2): installs Katna for the
//! current user into `%LOCALAPPDATA%\Programs\Katna`, without an
//! administrator prompt, and removes it again.
//!
//! Installing closes a running Katna, unpacks the payload, copies Setup in
//! as `katna-setup.exe` (Settings > Apps runs it to uninstall) and adds
//! Katna to Windows: the Start menu shortcut, the uninstall entry, the name
//! and icon of its toasts, the mail handler and, on a first install, start
//! at login. Everything is under the user's own profile and registry.

use std::io;
use std::path::{Path, PathBuf};

use crate::payload::Payload;

/// The steps [`install`] reports, in order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Step {
    /// Closing a running Katna.
    Stopping,
    /// Unpacking files; the share done so far.
    Copying(f32),
    /// Adding Katna to Windows.
    Registering,
}

/// Katna's programs, as the payload has them.
pub const MAIL_EXE: &str = "katna-mail.exe";
/// Setup's own copy, for uninstalling.
pub const SETUP_EXE: &str = "katna-setup.exe";
/// The icon, for the uninstall entry and toasts.
#[cfg_attr(not(windows), allow(dead_code))]
pub const ICON: &str = "katna.ico";

/// Where Katna is installed and where Windows looks for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    /// `%LOCALAPPDATA%\Programs\Katna`
    pub programs: PathBuf,
    /// The Start menu's programs folder.
    pub start_menu: PathBuf,
}

impl Layout {
    /// The layout for this user, from `%LOCALAPPDATA%` and `%APPDATA%`.
    pub fn from_env() -> io::Result<Self> {
        Self::from_lookup(|name| std::env::var_os(name))
    }

    pub fn from_lookup(lookup: impl Fn(&str) -> Option<std::ffi::OsString>) -> io::Result<Self> {
        let folder = |name: &str| {
            lookup(name)
                .map(PathBuf::from)
                .filter(|p| p.is_absolute())
                .ok_or_else(|| io::Error::other(format!("%{name}% is not set")))
        };
        Ok(Self {
            programs: folder("LOCALAPPDATA")?.join("Programs").join("Katna"),
            start_menu: folder("APPDATA")?
                .join("Microsoft")
                .join("Windows")
                .join("Start Menu")
                .join("Programs"),
        })
    }

    /// The Start menu shortcut.
    pub fn shortcut(&self) -> PathBuf {
        self.start_menu.join("Katna Mail.lnk")
    }

    /// Whether Katna is installed here already.
    pub fn installed(&self) -> bool {
        self.programs.join(MAIL_EXE).is_file()
    }
}

/// Installs `payload` into `layout`, calling `progress` as it goes.
pub fn install(
    layout: &Layout,
    payload: &Payload<'_>,
    version: &str,
    progress: &dyn Fn(Step),
) -> io::Result<()> {
    let first = !layout.installed();
    progress(Step::Stopping);
    stop(&layout.programs);
    unpack(&layout.programs, payload, &|share| {
        progress(Step::Copying(share))
    })?;
    progress(Step::Registering);
    // Setup copies itself in; it is not in the payload.
    if let Ok(me) = std::env::current_exe()
        && me != layout.programs.join(SETUP_EXE)
    {
        std::fs::copy(me, layout.programs.join(SETUP_EXE))?;
    }
    register(layout, payload, version, first)
}

/// Unpacks every file of `payload` into `dir`, a file at a time: each is
/// written beside its old copy and then put in its place.
pub fn unpack(dir: &Path, payload: &Payload<'_>, progress: &dyn Fn(f32)) -> io::Result<()> {
    let total = payload.size().max(1) as f32;
    let mut done = 0u64;
    for entry in &payload.entries {
        let path = dir.join(entry.path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let partial = path.with_extension("katna-new");
        {
            let mut file = io::BufWriter::new(std::fs::File::create(&partial)?);
            entry.unpack(&mut file)?;
            io::Write::flush(&mut file)?;
        }
        replace(&partial, &path)?;
        done += entry.size;
        progress(done as f32 / total);
    }
    Ok(())
}

/// Moves `new` over `path`, trying again for a moment while a closing
/// program still holds it.
fn replace(new: &Path, path: &Path) -> io::Result<()> {
    let mut tries = 0;
    loop {
        match std::fs::rename(new, path) {
            Ok(()) => return Ok(()),
            Err(err) if tries < 20 => {
                tracing::debug!(%err, path = %path.display(), "file in use");
                tries += 1;
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            Err(err) => {
                let _ = std::fs::remove_file(new);
                return Err(io::Error::new(
                    err.kind(),
                    format!("{}: {err}", path.display()),
                ));
            }
        }
    }
}

/// Removes Katna from `layout`; with `data`, also the user's mail and
/// settings on this computer.
pub fn uninstall(layout: &Layout, data: bool) -> io::Result<()> {
    stop(&layout.programs);
    unregister(layout);
    let _ = std::fs::remove_file(layout.shortcut());
    if data && let Ok(paths) = katna_core::Paths::from_env() {
        paths
            .delete_all_data()
            .map_err(|err| io::Error::other(err.to_string()))?;
        delete_passwords();
    }
    remove_programs(&layout.programs)
}

/// Closes every Katna program running from `programs`: the app, the
/// daemon and Katna's bus. SQLite keeps the mail consistent if the daemon
/// is stopped in the middle of a write.
#[cfg(windows)]
fn stop(programs: &Path) {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    if !programs.is_dir() {
        return;
    }
    // Only the processes started from this folder: another program may
    // run its own dbus-daemon.exe.
    let folder = programs.display().to_string().replace('\'', "''");
    let script = format!(
        "Get-Process | Where-Object {{ $_.Path -and $_.Path.StartsWith('{folder}\\', \
         [StringComparison]::OrdinalIgnoreCase) -and $_.Id -ne {me} }} | Stop-Process -Force",
        me = std::process::id()
    );
    let stopped = std::process::Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .creation_flags(CREATE_NO_WINDOW)
        .status();
    if let Err(err) = stopped {
        tracing::warn!(%err, "cannot close the running Katna");
    }
}

#[cfg(not(windows))]
fn stop(_programs: &Path) {}

/// The uninstall entry in Settings > Apps.
#[cfg_attr(not(windows), allow(dead_code))]
const UNINSTALL_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Uninstall\Katna";
/// Where Windows finds an app's name and icon for its toasts.
#[cfg_attr(not(windows), allow(dead_code))]
fn app_id_key() -> String {
    format!(
        r"Software\Classes\AppUserModelId\{}",
        katna_core::ids::MAIL_APP_ID
    )
}

#[cfg(windows)]
fn register(layout: &Layout, payload: &Payload<'_>, version: &str, first: bool) -> io::Result<()> {
    use katna_i18n::tr;
    use winreg::RegKey;
    use winreg::enums::HKEY_CURRENT_USER;

    let user = RegKey::predef(HKEY_CURRENT_USER);
    let dir = &layout.programs;
    let mail = dir.join(MAIL_EXE);
    let icon = dir.join(ICON);
    let setup = dir.join(SETUP_EXE);

    std::fs::create_dir_all(&layout.start_menu)?;
    let mut link = mslnk::ShellLink::new(&mail).map_err(io::Error::other)?;
    link.set_working_dir(Some(dir.display().to_string()));
    link.set_icon_location(Some(icon.display().to_string()));
    link.create_lnk(layout.shortcut())
        .map_err(io::Error::other)?;

    let (key, _) = user.create_subkey(UNINSTALL_KEY)?;
    let name = tr!("setup-app-name");
    key.set_value("DisplayName", &name)?;
    key.set_value("DisplayVersion", &version)?;
    key.set_value("Publisher", &"Katna")?;
    key.set_value("DisplayIcon", &icon.display().to_string())?;
    key.set_value("InstallLocation", &dir.display().to_string())?;
    key.set_value(
        "UninstallString",
        &format!("\"{}\" --uninstall", setup.display()),
    )?;
    key.set_value(
        "QuietUninstallString",
        &format!("\"{}\" --uninstall --quiet", setup.display()),
    )?;
    key.set_value("URLInfoAbout", &"https://katna.invenia.in")?;
    key.set_value("NoModify", &1u32)?;
    key.set_value("NoRepair", &1u32)?;
    let kb = u32::try_from(payload.size() / 1024).unwrap_or(u32::MAX);
    key.set_value("EstimatedSize", &kb)?;

    let (key, _) = user.create_subkey(app_id_key())?;
    key.set_value("DisplayName", &name)?;
    key.set_value("IconUri", &icon.display().to_string())?;

    katna_platform::mail_handler::register(&mail)?;

    // Start at login is on by default; an update keeps what the user
    // chose (Katna Mail's Settings, or Windows' Startup apps).
    if first {
        let (key, _) = user.create_subkey(r"Software\Microsoft\Windows\CurrentVersion\Run")?;
        key.set_value("Katna", &format!("\"{}\" --background", mail.display()))?;
    }
    Ok(())
}

#[cfg(not(windows))]
fn register(
    _layout: &Layout,
    _payload: &Payload<'_>,
    _version: &str,
    _first: bool,
) -> io::Result<()> {
    Ok(())
}

#[cfg(windows)]
fn unregister(_layout: &Layout) {
    use winreg::RegKey;
    use winreg::enums::HKEY_CURRENT_USER;

    let user = RegKey::predef(HKEY_CURRENT_USER);
    for key in [
        UNINSTALL_KEY.to_owned(),
        app_id_key(),
        format!(
            r"Software\Classes\{}",
            katna_platform::mail_handler::MAILTO_CLASS
        ),
        r"Software\Clients\Mail\Katna".to_owned(),
    ] {
        let _ = user.delete_subkey_all(key);
    }
    if let Ok(key) = user.open_subkey_with_flags(
        r"Software\RegisteredApplications",
        winreg::enums::KEY_SET_VALUE,
    ) {
        let _ = key.delete_value("Katna Mail");
    }
    if let Ok(key) = user.open_subkey_with_flags(
        r"Software\Microsoft\Windows\CurrentVersion\Run",
        winreg::enums::KEY_SET_VALUE,
    ) {
        let _ = key.delete_value("Katna");
    }
}

#[cfg(not(windows))]
fn unregister(_layout: &Layout) {}

/// Deletes the passwords Katna kept in the Credential Manager.
#[cfg(windows)]
fn delete_passwords() {
    let deleted = katna_platform::credentials::Credentials::open().and_then(|c| c.delete_all());
    if let Err(err) = deleted {
        tracing::warn!(%err, "cannot delete Katna's passwords");
    }
}

#[cfg(not(windows))]
fn delete_passwords() {}

/// Deletes the programs folder. When Windows' Apps settings uninstall
/// Katna, Setup's own copy in that folder is running, so a short-lived
/// `cmd` removes the folder once Setup has exited.
fn remove_programs(programs: &Path) -> io::Result<()> {
    let running_here = std::env::current_exe()
        .ok()
        .is_some_and(|me| me.starts_with(programs));
    if !running_here {
        return match std::fs::remove_dir_all(programs) {
            Err(err) if err.kind() != io::ErrorKind::NotFound => Err(err),
            _ => Ok(()),
        };
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        std::process::Command::new("cmd.exe")
            .raw_arg(format!(
                "/d /c ping -n 3 127.0.0.1 >nul & rmdir /s /q \"{}\"",
                programs.display()
            ))
            .current_dir(std::env::temp_dir())
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::*;

    #[test]
    fn installs_under_the_users_own_folders() {
        let layout = Layout::from_lookup(|name| match name {
            "LOCALAPPDATA" => Some(OsString::from("/Users/ada/AppData/Local")),
            "APPDATA" => Some(OsString::from("/Users/ada/AppData/Roaming")),
            _ => None,
        })
        .unwrap();
        assert_eq!(
            layout.programs,
            Path::new("/Users/ada/AppData/Local/Programs/Katna")
        );
        assert!(
            layout
                .shortcut()
                .ends_with("Start Menu/Programs/Katna Mail.lnk")
        );
        assert!(Layout::from_lookup(|_| None).is_err());
    }

    #[test]
    fn unpacks_and_replaces_files() {
        let source = tempfile::tempdir().unwrap();
        std::fs::write(source.path().join(MAIL_EXE), b"new").unwrap();
        let mut bytes = Vec::new();
        crate::payload::write(source.path(), 1, &mut bytes).unwrap();
        let payload = Payload::read(&bytes).unwrap();

        let target = tempfile::tempdir().unwrap();
        std::fs::write(target.path().join(MAIL_EXE), b"old").unwrap();
        let shares = std::cell::RefCell::new(Vec::new());
        unpack(target.path(), &payload, &|share| {
            shares.borrow_mut().push(share)
        })
        .unwrap();
        assert_eq!(std::fs::read(target.path().join(MAIL_EXE)).unwrap(), b"new");
        assert_eq!(*shares.borrow(), [1.0]);
        assert!(!target.path().join("katna-mail.katna-new").exists());
    }

    #[test]
    fn uninstall_removes_the_programs() {
        let root = tempfile::tempdir().unwrap();
        let layout = Layout {
            programs: root.path().join("Programs/Katna"),
            start_menu: root.path().join("Start Menu"),
        };
        std::fs::create_dir_all(&layout.programs).unwrap();
        std::fs::write(layout.programs.join(MAIL_EXE), b"x").unwrap();
        assert!(layout.installed());
        uninstall(&layout, false).unwrap();
        assert!(!layout.programs.exists());
    }
}
