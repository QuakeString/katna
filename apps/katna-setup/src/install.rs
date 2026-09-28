// SPDX-License-Identifier: GPL-3.0-or-later

//! What Setup does (`docs/ARCHITECTURE.md` §27.2): installs Katna for the
//! current user into `%LOCALAPPDATA%\Programs\Katna` without an
//! administrator prompt, or for everyone into `%ProgramFiles%\Katna`
//! (Windows asks for an administrator once), and removes it again.
//!
//! Installing closes a running Katna, unpacks the payload, copies Setup in
//! as `katna-setup.exe` (Settings > Apps runs it to uninstall) and adds
//! Katna to Windows: the shortcuts the user chose, the uninstall entry, the
//! name and icon of its toasts, the mail handler and, if chosen, start at
//! sign-in. A per-user install writes only the user's own profile and
//! registry; an install for everyone writes the machine's.

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
/// The shortcuts' file name.
const LINK: &str = "Katna Mail.lnk";

/// Who Katna is installed for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// The current user only; no administrator needed.
    User,
    /// Everyone on the computer; needs an administrator.
    Machine,
}

/// What the user chose to add to Windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Choices {
    pub desktop: bool,
    pub start_menu: bool,
    /// Start Katna at sign-in, in the background.
    pub autostart: bool,
}

impl Default for Choices {
    fn default() -> Self {
        Self {
            desktop: false,
            start_menu: true,
            autostart: true,
        }
    }
}

/// Where Katna is installed and where Windows looks for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    pub scope: Scope,
    /// Katna's programs: by default `%LOCALAPPDATA%\Programs\Katna` for one
    /// user, `%ProgramFiles%\Katna` for everyone.
    pub programs: PathBuf,
    /// The Start menu's programs folder.
    pub start_menu: PathBuf,
    /// The desktop the shortcut goes on: the user's, or the public one.
    pub desktop: PathBuf,
}

impl Layout {
    /// The default layout for `scope`, from the environment.
    pub fn from_env(scope: Scope) -> io::Result<Self> {
        Self::from_lookup(scope, |name| std::env::var_os(name))
    }

    pub fn from_lookup(
        scope: Scope,
        lookup: impl Fn(&str) -> Option<std::ffi::OsString>,
    ) -> io::Result<Self> {
        let folder = |name: &str| {
            lookup(name)
                .map(PathBuf::from)
                .filter(|p| p.is_absolute())
                .ok_or_else(|| io::Error::other(format!("%{name}% is not set")))
        };
        let start_menu = |root: PathBuf| {
            root.join("Microsoft")
                .join("Windows")
                .join("Start Menu")
                .join("Programs")
        };
        Ok(match scope {
            Scope::User => Self {
                scope,
                programs: folder("LOCALAPPDATA")?.join("Programs").join("Katna"),
                start_menu: start_menu(folder("APPDATA")?),
                desktop: folder("USERPROFILE")?.join("Desktop"),
            },
            Scope::Machine => Self {
                scope,
                programs: folder("ProgramFiles")?.join("Katna"),
                start_menu: start_menu(folder("ProgramData")?),
                desktop: folder("PUBLIC")?.join("Desktop"),
            },
        })
    }

    /// This layout with the programs in `dir`: a folder named Katna, or a
    /// new Katna folder inside `dir`, so removing Katna never deletes a
    /// folder the user picked.
    pub fn in_folder(mut self, dir: &Path) -> Self {
        self.programs = if dir
            .file_name()
            .is_some_and(|n| n.eq_ignore_ascii_case("Katna"))
        {
            dir.to_owned()
        } else {
            dir.join("Katna")
        };
        self
    }

    /// The Start menu shortcut.
    pub fn shortcut(&self) -> PathBuf {
        self.start_menu.join(LINK)
    }

    /// The desktop shortcut.
    pub fn desktop_shortcut(&self) -> PathBuf {
        self.desktop.join(LINK)
    }

    /// Whether Katna is installed here already.
    pub fn installed(&self) -> bool {
        self.programs.join(MAIL_EXE).is_file()
    }

    /// Whether the programs folder can take Katna: a new folder, or one
    /// that holds Katna already or is empty, and not a link to somewhere
    /// else. Setup never mixes Katna into another program's files, which
    /// removing Katna would delete.
    pub fn check_folder(&self) -> io::Result<()> {
        let dir = &self.programs;
        let usable = match std::fs::symlink_metadata(dir) {
            Ok(meta) if meta.file_type().is_symlink() || !meta.is_dir() => false,
            Ok(_) => self.installed() || std::fs::read_dir(dir)?.next().is_none(),
            Err(err) if err.kind() == io::ErrorKind::NotFound => true,
            Err(err) => return Err(err),
        };
        if usable {
            Ok(())
        } else {
            let path = dir.display().to_string();
            Err(io::Error::other(katna_i18n::tr!(
                "setup-folder-not-empty",
                path = path.as_str()
            )))
        }
    }

    /// Whether the programs are inside `%ProgramFiles%`, whose rules
    /// already keep users from changing them.
    #[cfg_attr(not(windows), allow(dead_code))]
    fn in_program_files(&self) -> bool {
        let lower = |p: &Path| p.display().to_string().to_lowercase();
        std::env::var_os("ProgramFiles")
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
            .is_some_and(|root| {
                let (root, dir) = (lower(&root), lower(&self.programs));
                dir.strip_prefix(root.trim_end_matches('\\'))
                    .is_some_and(|rest| rest.starts_with('\\'))
            })
    }

    /// The shortcuts and start at sign-in as they are now, for an update
    /// to keep.
    pub fn current_choices(&self) -> Choices {
        Choices {
            desktop: self.desktop_shortcut().is_file(),
            start_menu: self.shortcut().is_file(),
            autostart: autostart_enabled(self.scope),
        }
    }

    /// Katna as installed on this computer: where the uninstall entry
    /// says, for this user first, then for everyone.
    pub fn existing() -> Option<Self> {
        for scope in [Scope::User, Scope::Machine] {
            let Ok(layout) = Self::from_env(scope) else {
                continue;
            };
            let layout = match install_location(scope) {
                Some(dir) => Self {
                    programs: dir,
                    ..layout
                },
                None => layout,
            };
            if layout.installed() {
                return Some(layout);
            }
        }
        None
    }
}

/// The folder the uninstall entry for `scope` names.
#[cfg(windows)]
fn install_location(scope: Scope) -> Option<PathBuf> {
    root(scope)
        .open_subkey(UNINSTALL_KEY)
        .and_then(|key| key.get_value::<String, _>("InstallLocation"))
        .ok()
        .map(PathBuf::from)
}

#[cfg(not(windows))]
fn install_location(_scope: Scope) -> Option<PathBuf> {
    None
}

/// The registry Katna's entries go in for `scope`.
#[cfg(windows)]
fn root(scope: Scope) -> winreg::RegKey {
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
    winreg::RegKey::predef(match scope {
        Scope::User => HKEY_CURRENT_USER,
        Scope::Machine => HKEY_LOCAL_MACHINE,
    })
}

/// Where Windows starts programs at sign-in.
#[cfg_attr(not(windows), allow(dead_code))]
const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";

#[cfg(windows)]
fn autostart_enabled(scope: Scope) -> bool {
    root(scope)
        .open_subkey(RUN_KEY)
        .and_then(|key| key.get_value::<String, _>("Katna"))
        .is_ok()
}

#[cfg(not(windows))]
fn autostart_enabled(_scope: Scope) -> bool {
    false
}

/// Whether this process may write the machine's registry and Program
/// Files, as an install for everyone must.
#[cfg(windows)]
pub fn elevated() -> bool {
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_WRITE};
    winreg::RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags("SOFTWARE", KEY_WRITE)
        .is_ok()
}

#[cfg(not(windows))]
pub fn elevated() -> bool {
    true
}

/// Runs this Setup again with `args`, as an administrator: Windows asks
/// the user first. Returns its exit code; an error when the user said no.
#[cfg(windows)]
pub fn run_elevated(args: &[String]) -> io::Result<i32> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    let me = std::env::current_exe()?;
    let list = args
        .iter()
        .map(|a| format!("\"{a}\""))
        .collect::<Vec<_>>()
        .join(" ");
    let script = format!(
        "$p = Start-Process -FilePath '{}' -ArgumentList '{}' -Verb RunAs -Wait -PassThru; \
         exit $p.ExitCode",
        ps_quote(&me.display().to_string()),
        ps_quote(&list)
    );
    let status = std::process::Command::new(system_program(POWERSHELL))
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .creation_flags(CREATE_NO_WINDOW)
        .status()?;
    status
        .code()
        .ok_or_else(|| io::Error::other("Setup stopped"))
}

#[cfg(not(windows))]
pub fn run_elevated(_args: &[String]) -> io::Result<i32> {
    Err(io::Error::other("only on Windows"))
}

/// Installs `payload` into `layout`, calling `progress` as it goes.
/// With `choices`, the shortcuts and start at sign-in are set as chosen;
/// without, an update keeps them as they are.
pub fn install(
    layout: &Layout,
    payload: &Payload<'_>,
    version: &str,
    choices: Option<Choices>,
    progress: &dyn Fn(Step),
) -> io::Result<()> {
    let choices = choices.unwrap_or_else(|| layout.current_choices());
    layout.check_folder()?;
    progress(Step::Stopping);
    stop(&layout.programs);
    std::fs::create_dir_all(&layout.programs)?;
    if layout.scope == Scope::Machine && !layout.in_program_files() {
        lock(&layout.programs)?;
    }
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
    register(layout, payload, version, choices)
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
    let _ = std::fs::remove_file(layout.desktop_shortcut());
    if data {
        delete_data()?;
    }
    remove_programs(&layout.programs)
}

/// Deletes this user's mail, settings and passwords on this computer.
pub fn delete_data() -> io::Result<()> {
    if let Ok(paths) = katna_core::Paths::from_env() {
        paths
            .delete_all_data()
            .map_err(|err| io::Error::other(err.to_string()))?;
        delete_passwords();
    }
    Ok(())
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
    let folder = ps_quote(&programs.display().to_string());
    let script = format!(
        "Get-Process | Where-Object {{ $_.Path -and $_.Path.StartsWith('{folder}\\', \
         [StringComparison]::OrdinalIgnoreCase) -and $_.Id -ne {me} }} | Stop-Process -Force",
        me = std::process::id()
    );
    let stopped = std::process::Command::new(system_program(POWERSHELL))
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .creation_flags(CREATE_NO_WINDOW)
        .status();
    if let Err(err) = stopped {
        tracing::warn!(%err, "cannot close the running Katna");
    }
}

#[cfg(not(windows))]
fn stop(_programs: &Path) {}

/// Gives a folder for everyone outside Program Files the same rules as
/// Program Files: administrators and Windows own it and can change it,
/// users can only read and run it. Otherwise any user could replace the
/// programs that start when every other user signs in.
#[cfg(windows)]
fn lock(dir: &Path) -> io::Result<()> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    // Administrators, SYSTEM and Users by their well-known SIDs, which
    // every language of Windows has.
    let steps: [&[&str]; 2] = [
        &["/setowner", "*S-1-5-32-544", "/T", "/C", "/Q"],
        &[
            "/inheritance:r",
            "/grant:r",
            "*S-1-5-32-544:(OI)(CI)F",
            "*S-1-5-18:(OI)(CI)F",
            "*S-1-5-32-545:(OI)(CI)RX",
            "/T",
            "/C",
            "/Q",
        ],
    ];
    for args in steps {
        let status = std::process::Command::new(system_program("icacls.exe"))
            .arg(dir)
            .args(args)
            .creation_flags(CREATE_NO_WINDOW)
            .status()?;
        if !status.success() {
            return Err(io::Error::other(format!(
                "{}: icacls {status}",
                dir.display()
            )));
        }
    }
    Ok(())
}

#[cfg(not(windows))]
fn lock(_dir: &Path) -> io::Result<()> {
    Ok(())
}

/// Where a Setup running as administrator puts how far it got, for the
/// Setup that started it to read: a key only administrators can change,
/// rather than a file the caller names, which Setup as administrator
/// would overwrite wherever it is.
#[cfg_attr(not(windows), allow(dead_code))]
const PROGRESS_KEY: &str = r"SOFTWARE\Katna\Setup";

#[cfg_attr(not(windows), allow(dead_code))]
fn progress_value(id: u32) -> String {
    format!("Progress-{id}")
}

/// Records `line` for the Setup with process ID `id`.
#[cfg(windows)]
pub fn write_progress(id: u32, line: &str) {
    use winreg::enums::HKEY_LOCAL_MACHINE;
    if let Ok((key, _)) = winreg::RegKey::predef(HKEY_LOCAL_MACHINE).create_subkey(PROGRESS_KEY) {
        let _ = key.set_value(progress_value(id), &line.to_owned());
    }
}

#[cfg(not(windows))]
pub fn write_progress(_id: u32, _line: &str) {}

/// What the Setup running as administrator recorded for this one, `id`.
#[cfg(windows)]
pub fn read_progress(id: u32) -> Option<String> {
    use winreg::enums::HKEY_LOCAL_MACHINE;
    winreg::RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(PROGRESS_KEY)
        .and_then(|key| key.get_value::<String, _>(progress_value(id)))
        .ok()
}

#[cfg(not(windows))]
pub fn read_progress(_id: u32) -> Option<String> {
    None
}

/// Removes what [`write_progress`] recorded, and the keys once empty.
#[cfg(windows)]
pub fn clear_progress(id: u32) {
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_SET_VALUE};
    let machine = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = machine.open_subkey_with_flags(PROGRESS_KEY, KEY_SET_VALUE) {
        let _ = key.delete_value(progress_value(id));
    }
    // Windows deletes a key with values in it, so only empty ones.
    for path in [PROGRESS_KEY, r"SOFTWARE\Katna"] {
        let empty = machine.open_subkey(path).is_ok_and(|key| {
            key.enum_values().next().is_none() && key.enum_keys().next().is_none()
        });
        if empty {
            let _ = machine.delete_subkey(path);
        }
    }
}

#[cfg(not(windows))]
pub fn clear_progress(_id: u32) {}

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
fn register(
    layout: &Layout,
    payload: &Payload<'_>,
    version: &str,
    choices: Choices,
) -> io::Result<()> {
    use katna_i18n::tr;

    let root = root(layout.scope);
    let dir = &layout.programs;
    let mail = dir.join(MAIL_EXE);
    let icon = dir.join(ICON);
    let setup = dir.join(SETUP_EXE);

    let link = |path: PathBuf, on: bool| -> io::Result<()> {
        if !on {
            let _ = std::fs::remove_file(&path);
            return Ok(());
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut link = mslnk::ShellLink::new(&mail).map_err(io::Error::other)?;
        link.set_working_dir(Some(dir.display().to_string()));
        link.set_icon_location(Some(icon.display().to_string()));
        link.create_lnk(path).map_err(io::Error::other)
    };
    link(layout.shortcut(), choices.start_menu)?;
    link(layout.desktop_shortcut(), choices.desktop)?;

    let (key, _) = root.create_subkey(UNINSTALL_KEY)?;
    let name = tr!("setup-app-name");
    key.set_value("DisplayName", &name)?;
    key.set_value("DisplayVersion", &version)?;
    key.set_value("Publisher", &"Katna")?;
    key.set_value("DisplayIcon", &icon.display().to_string())?;
    key.set_value("InstallLocation", &dir.display().to_string())?;
    let everyone = match layout.scope {
        Scope::User => "",
        Scope::Machine => " --all-users",
    };
    key.set_value(
        "UninstallString",
        &format!("\"{}\" --uninstall{everyone}", setup.display()),
    )?;
    key.set_value(
        "QuietUninstallString",
        &format!("\"{}\" --uninstall --quiet{everyone}", setup.display()),
    )?;
    key.set_value("URLInfoAbout", &"https://katna.invenia.in")?;
    key.set_value("NoModify", &1u32)?;
    key.set_value("NoRepair", &1u32)?;
    let kb = u32::try_from(payload.size() / 1024).unwrap_or(u32::MAX);
    key.set_value("EstimatedSize", &kb)?;

    let (key, _) = root.create_subkey(app_id_key())?;
    key.set_value("DisplayName", &name)?;
    key.set_value("IconUri", &icon.display().to_string())?;

    katna_platform::mail_handler::register_in(&root, &mail)?;

    // For everyone, the machine's Run key starts Katna for each user;
    // Katna Mail's own setting changes only the user's.
    let (key, _) = root.create_subkey(RUN_KEY)?;
    if choices.autostart {
        key.set_value("Katna", &format!("\"{}\" --background", mail.display()))?;
    } else {
        let _ = key.delete_value("Katna");
    }
    Ok(())
}

#[cfg(not(windows))]
fn register(
    layout: &Layout,
    _payload: &Payload<'_>,
    _version: &str,
    choices: Choices,
) -> io::Result<()> {
    // Only the shortcuts' places, so tests can check them on Linux.
    for (path, on) in [
        (layout.shortcut(), choices.start_menu),
        (layout.desktop_shortcut(), choices.desktop),
    ] {
        if on {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(path, b"")?;
        } else {
            let _ = std::fs::remove_file(path);
        }
    }
    Ok(())
}

#[cfg(windows)]
fn unregister(layout: &Layout) {
    let root = root(layout.scope);
    for key in [
        UNINSTALL_KEY.to_owned(),
        app_id_key(),
        format!(
            r"Software\Classes\{}",
            katna_platform::mail_handler::MAILTO_CLASS
        ),
        r"Software\Clients\Mail\Katna".to_owned(),
    ] {
        let _ = root.delete_subkey_all(key);
    }
    if let Ok(key) = root.open_subkey_with_flags(
        r"Software\RegisteredApplications",
        winreg::enums::KEY_SET_VALUE,
    ) {
        let _ = key.delete_value("Katna Mail");
    }
    if let Ok(key) = root.open_subkey_with_flags(RUN_KEY, winreg::enums::KEY_SET_VALUE) {
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
/// PowerShell removes the folder once Setup has exited.
fn remove_programs(programs: &Path) -> io::Result<()> {
    // Only Katna's own folder: never one without Katna Mail in it.
    if !programs.join(MAIL_EXE).is_file() {
        return Ok(());
    }
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
        let script = format!(
            "Start-Sleep -Seconds 2; Remove-Item -LiteralPath '{}' -Recurse -Force",
            ps_quote(&programs.display().to_string())
        );
        std::process::Command::new(system_program(POWERSHELL))
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .current_dir(std::env::temp_dir())
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()?;
    }
    Ok(())
}

/// `s` inside a single-quoted PowerShell string. PowerShell also ends such
/// a string at the typographic single quotes, so those are doubled too.
#[cfg_attr(not(windows), allow(dead_code))]
fn ps_quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if matches!(c, '\'' | '\u{2018}' | '\u{2019}' | '\u{201A}' | '\u{201B}') {
            out.push(c);
        }
        out.push(c);
    }
    out
}

/// PowerShell, under `%SystemRoot%\System32`.
#[cfg_attr(not(windows), allow(dead_code))]
const POWERSHELL: &str = r"WindowsPowerShell\v1.0\powershell.exe";

/// A program of Windows by its full path under `%SystemRoot%\System32`.
/// By name alone, Windows would look in Setup's own folder first, often
/// Downloads, where any page can leave a `powershell.exe` that would then
/// run as administrator.
#[cfg_attr(not(windows), allow(dead_code))]
fn system_program(name: &str) -> PathBuf {
    std::env::var_os("SystemRoot")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .unwrap_or_else(|| PathBuf::from(r"C:\Windows"))
        .join("System32")
        .join(name)
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::*;

    /// An absolute path on this system: Windows needs a drive.
    fn at(path: &str) -> PathBuf {
        Path::new(if cfg!(windows) { "C:\\" } else { "/" }).join(path)
    }

    fn windows(name: &str) -> Option<OsString> {
        let path = match name {
            "LOCALAPPDATA" => "Users/ada/AppData/Local",
            "APPDATA" => "Users/ada/AppData/Roaming",
            "USERPROFILE" => "Users/ada",
            "ProgramFiles" => "Program Files",
            "ProgramData" => "ProgramData",
            "PUBLIC" => "Users/Public",
            _ => return None,
        };
        Some(at(path).into_os_string())
    }

    #[test]
    fn installs_under_the_users_own_folders() {
        let layout = Layout::from_lookup(Scope::User, windows).unwrap();
        assert_eq!(
            layout.programs,
            at("Users/ada/AppData/Local/Programs/Katna")
        );
        assert_eq!(
            layout.shortcut(),
            at("Users/ada/AppData/Roaming/Microsoft/Windows/Start Menu/Programs/Katna Mail.lnk")
        );
        assert_eq!(
            layout.desktop_shortcut(),
            at("Users/ada/Desktop/Katna Mail.lnk")
        );
        assert!(Layout::from_lookup(Scope::User, |_| None).is_err());
    }

    #[test]
    fn installs_for_everyone_in_the_machines_folders() {
        let layout = Layout::from_lookup(Scope::Machine, windows).unwrap();
        assert_eq!(layout.programs, at("Program Files/Katna"));
        assert!(
            layout
                .shortcut()
                .starts_with(at("ProgramData/Microsoft/Windows/Start Menu"))
        );
        assert_eq!(
            layout.desktop_shortcut(),
            at("Users/Public/Desktop/Katna Mail.lnk")
        );
    }

    #[test]
    fn a_picked_folder_gets_its_own_katna_folder() {
        let layout = Layout::from_lookup(Scope::User, windows).unwrap();
        assert_eq!(
            layout.clone().in_folder(&at("D/Apps")).programs,
            at("D/Apps/Katna")
        );
        assert_eq!(
            layout.in_folder(&at("D/Apps/katna")).programs,
            at("D/Apps/katna")
        );
    }

    #[test]
    fn shortcuts_follow_the_choices() {
        let root = tempfile::tempdir().unwrap();
        let source = tempfile::tempdir().unwrap();
        std::fs::write(source.path().join(MAIL_EXE), b"mail").unwrap();
        let mut bytes = Vec::new();
        crate::payload::write(source.path(), 1, &mut bytes).unwrap();
        let payload = Payload::read(&bytes).unwrap();
        let layout = Layout {
            scope: Scope::User,
            programs: root.path().join("Programs/Katna"),
            start_menu: root.path().join("Start Menu"),
            desktop: root.path().join("Desktop"),
        };
        let choices = Choices {
            desktop: true,
            start_menu: false,
            autostart: false,
        };
        install(&layout, &payload, "1", Some(choices), &|_| {}).unwrap();
        assert!(layout.desktop_shortcut().is_file());
        assert!(!layout.shortcut().exists());
        // An update keeps them.
        install(&layout, &payload, "2", None, &|_| {}).unwrap();
        assert_eq!(layout.current_choices(), choices);
        uninstall(&layout, false).unwrap();
        assert!(!layout.desktop_shortcut().exists());
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
            scope: Scope::User,
            programs: root.path().join("Programs/Katna"),
            start_menu: root.path().join("Start Menu"),
            desktop: root.path().join("Desktop"),
        };
        std::fs::create_dir_all(&layout.programs).unwrap();
        std::fs::write(layout.programs.join(MAIL_EXE), b"x").unwrap();
        assert!(layout.installed());
        uninstall(&layout, false).unwrap();
        assert!(!layout.programs.exists());
    }
    #[test]
    fn powershell_strings_keep_every_quote() {
        assert_eq!(ps_quote("C:\\Bob's"), "C:\\Bob''s");
        assert_eq!(
            ps_quote("a\u{2019}b\u{2018}"),
            "a\u{2019}\u{2019}b\u{2018}\u{2018}"
        );
    }

    #[test]
    fn installs_only_into_an_empty_or_katna_folder() {
        let dir = tempfile::tempdir().unwrap();
        let layout = Layout::from_lookup(Scope::User, windows)
            .unwrap()
            .in_folder(dir.path());
        // A new folder, then an empty one.
        layout.check_folder().unwrap();
        std::fs::create_dir(&layout.programs).unwrap();
        layout.check_folder().unwrap();
        // Someone else's files.
        std::fs::write(layout.programs.join("other.exe"), b"").unwrap();
        assert!(layout.check_folder().is_err());
        // Katna's own folder, for an update.
        std::fs::write(layout.programs.join(MAIL_EXE), b"").unwrap();
        layout.check_folder().unwrap();
    }
}
