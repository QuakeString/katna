// SPDX-License-Identifier: GPL-3.0-or-later

//! "Open Katna Mail at login": a desktop entry in the XDG autostart folder
//! (`$XDG_CONFIG_HOME/autostart`), which KDE, GNOME and the other desktops
//! start at login. The file is the setting, so turning it off in the
//! desktop's own autostart settings shows here too.

use std::io;
use std::path::{Path, PathBuf};

use katna_core::ids;

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

/// Whether Katna Mail opens at login.
pub fn is_on() -> bool {
    path().is_some_and(|p| p.is_file())
}

/// Makes Katna Mail open at login, or not.
pub fn set(on: bool) -> io::Result<()> {
    let path = path().ok_or_else(|| io::Error::other("no home folder"))?;
    if !on {
        return match std::fs::remove_file(&path) {
            Err(err) if err.kind() != io::ErrorKind::NotFound => Err(err),
            _ => Ok(()),
        };
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let exe = std::env::current_exe()?;
    std::fs::write(&path, entry(&exe))
}

/// The desktop entry that starts `exe`.
fn entry(exe: &Path) -> String {
    // The installed program by name, so an update that moves it still
    // starts; a build run from elsewhere by its path, quoted.
    let exec = if exe == Path::new("/usr/bin/katna-mail") {
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
    format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=Katna Mail\n\
         Comment=Opened at login (Settings > General)\n\
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

    #[test]
    fn installed_program_starts_by_name() {
        let text = entry(Path::new("/usr/bin/katna-mail"));
        assert!(text.contains("\nExec=katna-mail\n"), "{text}");
        assert!(text.contains(&format!("\nIcon={}\n", ids::MAIL_APP_ID)));
    }

    #[test]
    fn other_builds_start_by_quoted_path() {
        let text = entry(Path::new("/home/me/my katna/katna-mail"));
        assert!(
            text.contains("\nExec=\"/home/me/my katna/katna-mail\"\n"),
            "{text}"
        );
    }
}
