// SPDX-License-Identifier: GPL-3.0-or-later

//! Installing an update that `katna-daemon` downloaded and checked, and
//! starting Katna Mail again afterwards (`docs/ARCHITECTURE.md` §21.2).
//! Each kind of package installs in its own way; the rest of the flow is
//! the same for all. No GPUI here.
//!
//! The app installs rather than the daemon because the system's password
//! prompt (the polkit agent) belongs to the desktop session the app runs
//! in; a user service has none.
//!
//! On Windows the new Katna Setup does it: Setup closes every Katna
//! program in the install folder, this one included, so a small hidden
//! PowerShell outside that folder runs Setup quietly, waits for it, and
//! then opens Katna Mail again.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use katna_core::update::{self, Package};

/// The flag that makes a new Katna Mail wait for the old one to close,
/// followed by its process ID.
pub const AFTER_FLAG: &str = "--after-update";

/// Longest wait for the old Katna Mail to close.
const WAIT: Duration = Duration::from_secs(20);

/// Why an update was not installed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallError {
    /// The password prompt was closed, or the password refused.
    Cancelled,
    /// This build cannot install updates itself.
    Unsupported,
    /// Installing failed; the installer's own words.
    Failed(String),
}

/// Installs the package in `file`, which must have the SHA-256 `sha256`,
/// after the system's password prompt. Blocks until done; on Windows,
/// only until Setup has started, which then opens the new Katna Mail
/// itself ([`start_new`] does nothing there).
pub fn install(file: &Path, sha256: &str) -> Result<(), InstallError> {
    match Package::current() {
        Package::Arch => install_arch(file, sha256),
        Package::Windows => install_windows(file),
        Package::AppImage => install_appimage(file),
        Package::Tarball => install_tarball(file),
        Package::Rpm | Package::Snap | Package::Flatpak | Package::Nix | Package::Other => {
            Err(InstallError::Unsupported)
        }
    }
}

/// The command that installs the downloaded build in `file` when Katna
/// cannot: for the RPM, Snap, Flatpak and Nix, and for a tarball
/// installed where only an administrator may write.
pub fn command(file: &Path) -> Option<String> {
    let package = Package::current();
    if package == Package::Tarball {
        let prefix = tarball_prefix()?;
        if writable(&prefix.join("bin")) {
            return None;
        }
        return Some(format!(
            "tar -xzf {} -C /tmp && sudo /tmp/{TARBALL_DIR}/install.sh --prefix {}",
            update::shell_quote(&file.display().to_string()),
            update::shell_quote(&prefix.display().to_string()),
        ));
    }
    update::update_command(package, &file.display().to_string())
}

/// The folder the tarball unpacks into.
const TARBALL_DIR: &str = "katna-linux-x86_64";

/// Puts the new AppImage in `file` in place of the one Katna runs from:
/// beside it first, then renamed over it, so the running Katna keeps
/// its image.
fn install_appimage(file: &Path) -> Result<(), InstallError> {
    let failed = |err: std::io::Error| InstallError::Failed(err.to_string());
    let image = appimage().ok_or(InstallError::Unsupported)?;
    let name = image
        .file_name()
        .ok_or(InstallError::Unsupported)?
        .to_string_lossy()
        .into_owned();
    let new = image.with_file_name(format!(".{name}.new"));
    let copied = std::fs::copy(file, &new)
        .and_then(|_| set_executable(&new))
        .and_then(|()| std::fs::rename(&new, &image));
    if let Err(err) = copied {
        let _ = std::fs::remove_file(&new);
        return Err(failed(err));
    }
    let _ = std::fs::remove_file(file);
    Ok(())
}

/// Unpacks the new tarball in `file` beside it and runs its
/// `install.sh` for the folder this Katna is installed in.
fn install_tarball(file: &Path) -> Result<(), InstallError> {
    let prefix = tarball_prefix().ok_or(InstallError::Unsupported)?;
    let work = file.with_extension("unpacked");
    let _ = std::fs::remove_dir_all(&work);
    std::fs::create_dir_all(&work).map_err(|err| InstallError::Failed(err.to_string()))?;
    let installed = run(Command::new("tar")
        .arg("-xzf")
        .arg(file)
        .arg("-C")
        .arg(&work))
    .and_then(|()| {
        run(Command::new("sh")
            .arg(work.join(TARBALL_DIR).join("install.sh"))
            .arg("--prefix")
            .arg(&prefix))
    });
    let _ = std::fs::remove_dir_all(&work);
    if installed.is_ok() {
        let _ = std::fs::remove_file(file);
    }
    installed
}

/// Runs `command`; its last line of errors when it fails.
fn run(command: &mut Command) -> Result<(), InstallError> {
    let output = command
        .output()
        .map_err(|err| InstallError::Failed(err.to_string()))?;
    if output.status.success() {
        return Ok(());
    }
    let said = String::from_utf8_lossy(&output.stderr)
        .lines()
        .rfind(|line| !line.trim().is_empty())
        .unwrap_or_default()
        .trim()
        .to_owned();
    tracing::warn!(status = ?output.status, said, "the update was not installed");
    Err(InstallError::Failed(if said.is_empty() {
        output.status.to_string()
    } else {
        said
    }))
}

/// Where the tarball's `install.sh` installed this Katna: the folder
/// above its `bin`.
fn tarball_prefix() -> Option<PathBuf> {
    let exe = installed_exe().ok()?;
    let bin = exe.parent()?;
    (bin.file_name()? == "bin").then(|| bin.parent().map(Path::to_path_buf))?
}

/// Whether this user may create files in `dir`.
fn writable(dir: &Path) -> bool {
    let probe = dir.join(format!(".katna-update-{}", std::process::id()));
    let made = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&probe)
        .is_ok();
    if made {
        let _ = std::fs::remove_file(&probe);
    }
    made
}

/// The AppImage this Katna runs from.
fn appimage() -> Option<PathBuf> {
    std::env::var_os("APPIMAGE")
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
}

#[cfg(unix)]
fn set_executable(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))
}

#[cfg(not(unix))]
fn set_executable(_path: &Path) -> std::io::Result<()> {
    Ok(())
}

/// Starts the new Katna Setup (`file`, checked by the daemon) quietly
/// from a hidden PowerShell that opens Katna Mail again once Setup is
/// done, whether or not it worked. Katna installed for everyone asks the
/// administrator's permission first (`--all-users`).
#[cfg(windows)]
fn install_windows(file: &Path) -> Result<(), InstallError> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    let exe = installed_exe().map_err(|err| InstallError::Failed(err.to_string()))?;
    let for_me = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .is_some_and(|local| exe.starts_with(local));
    let powershell = std::env::var_os("SystemRoot")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .unwrap_or_else(|| PathBuf::from(r"C:\Windows"))
        .join(r"System32\WindowsPowerShell\v1.0\powershell.exe");
    Command::new(powershell)
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle",
            "Hidden",
            "-Command",
        ])
        .arg(windows_script(file, &exe, for_me))
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map(drop)
        .map_err(|err| InstallError::Failed(format!("powershell: {err}")))
}

#[cfg(not(windows))]
fn install_windows(_file: &Path) -> Result<(), InstallError> {
    Err(InstallError::Unsupported)
}

/// The PowerShell that runs Setup in `setup` quietly, as an update of the
/// install it finds, then opens Katna Mail at `app`.
#[cfg_attr(not(windows), allow(dead_code))]
fn windows_script(setup: &Path, app: &Path, for_me: bool) -> String {
    // Single quotes take everything literally but themselves, doubled.
    let quote = |path: &Path| format!("'{}'", path.display().to_string().replace('\'', "''"));
    let (args, elevate) = if for_me {
        ("'--quiet','--update'", "")
    } else {
        ("'--quiet','--update','--all-users'", " -Verb RunAs")
    };
    format!(
        "try {{ Start-Process -FilePath {setup} -ArgumentList {args} -Wait{elevate} }} \
         catch {{ }}; Start-Process -FilePath {app}",
        setup = quote(setup),
        app = quote(app),
    )
}

/// `pkexec` runs the package's helper as root once the password is given
/// (`packaging/arch/katna-update-helper`).
fn install_arch(file: &Path, sha256: &str) -> Result<(), InstallError> {
    let output = Command::new("pkexec")
        .arg(update::ARCH_HELPER)
        .arg(file)
        .arg(sha256)
        .output()
        .map_err(|err| InstallError::Failed(format!("pkexec: {err}")))?;
    let said = String::from_utf8_lossy(&output.stderr)
        .lines()
        .rfind(|line| !line.trim().is_empty())
        .unwrap_or_default()
        .trim()
        .to_owned();
    match output.status.code() {
        Some(0) => Ok(()),
        // pkexec: the prompt was dismissed (126) or the password refused
        // (127).
        Some(126 | 127) => Err(InstallError::Cancelled),
        code => {
            tracing::warn!(?code, said, "the update was not installed");
            Err(InstallError::Failed(if said.is_empty() {
                format!("exit status {code:?}")
            } else {
                said
            }))
        }
    }
}

/// Starts the Katna Mail just installed, which waits for this one to
/// close before it opens its window. Quit this one next.
pub fn start_new() -> std::io::Result<()> {
    let mut command = match Package::current() {
        Package::Windows => return Ok(()),
        // The new image; started without a program name, it opens Katna
        // Mail.
        Package::AppImage => Command::new(appimage().ok_or(std::io::ErrorKind::NotFound)?),
        _ => Command::new(installed_exe()?),
    };
    command
        .arg(AFTER_FLAG)
        .arg(std::process::id().to_string())
        .spawn()
        .map(|_| ())
}

/// The program at the path this one was started from: after an update,
/// the new version.
fn installed_exe() -> std::io::Result<PathBuf> {
    let exe = std::env::current_exe()?;
    // Linux names a replaced program "<path> (deleted)".
    Ok(
        match exe.to_str().and_then(|s| s.strip_suffix(" (deleted)")) {
            Some(path) => PathBuf::from(path),
            None => exe,
        },
    )
}

/// Waits until process `pid`, the Katna Mail that was updated, is gone,
/// so this one can take its place.
pub fn wait_for_exit(pid: u32) {
    let start = Instant::now();
    while running(pid) && start.elapsed() < WAIT {
        std::thread::sleep(Duration::from_millis(100));
    }
}

#[cfg(target_os = "linux")]
fn running(pid: u32) -> bool {
    Path::new(&format!("/proc/{pid}")).exists()
}

#[cfg(not(target_os = "linux"))]
fn running(_pid: u32) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_build_from_source_does_not_install() {
        if Package::current() == Package::Other {
            assert_eq!(
                install(Path::new("/nonexistent"), "0"),
                Err(InstallError::Unsupported)
            );
        }
    }

    #[test]
    fn windows_setup_runs_quietly_then_reopens_katna() {
        let script = windows_script(
            Path::new(r"C:\Users\O'Neil\AppData\Local\katna\cache\updates\KatnaSetup.exe"),
            Path::new(r"C:\Users\O'Neil\AppData\Local\Programs\Katna\katna-mail.exe"),
            true,
        );
        assert_eq!(
            script,
            "try { Start-Process -FilePath 'C:\\Users\\O''Neil\\AppData\\Local\\katna\\cache\\updates\\KatnaSetup.exe' \
             -ArgumentList '--quiet','--update' -Wait } catch { }; \
             Start-Process -FilePath 'C:\\Users\\O''Neil\\AppData\\Local\\Programs\\Katna\\katna-mail.exe'"
        );
        let everyone = windows_script(Path::new("S.exe"), Path::new("K.exe"), false);
        assert!(everyone.contains("'--all-users' -Wait -Verb RunAs"));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn waits_only_for_a_running_process() {
        assert!(running(std::process::id()));
        let start = Instant::now();
        // No process has this ID (above the kernel's limit).
        wait_for_exit(u32::MAX);
        assert!(start.elapsed() < Duration::from_secs(1));
    }
}
