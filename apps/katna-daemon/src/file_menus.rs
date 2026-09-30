// SPDX-License-Identifier: GPL-3.0-or-later

//! Keeps "Send with Katna Mail" in the file managers' right-click menus in
//! step with the mail accounts (`katna_platform::file_menus`): at start
//! and whenever an account is added, removed or renamed.

#[cfg(not(windows))]
use std::path::PathBuf;

use katna_core::AccountKind;
use katna_i18n::tr;
use katna_platform::file_menus::{self, MenuAccount};

use crate::daemon::Daemon;

/// The file GNOME Files' extension reads, in Katna's data folder.
#[cfg(not(windows))]
const NAUTILUS_FILE: &str = "send-menu.json";

/// Rewrites the menus for the accounts there are now.
pub(crate) fn refresh(daemon: &Daemon) {
    let accounts = match daemon.store().accounts() {
        Ok(accounts) => accounts,
        Err(err) => {
            tracing::warn!(%err, "cannot read the accounts for the file menus");
            return;
        }
    };
    let accounts: Vec<MenuAccount> = accounts
        .into_iter()
        .filter(|a| {
            matches!(
                a.kind,
                AccountKind::Imap | AccountKind::Jmap | AccountKind::Pop3
            )
        })
        .map(|a| MenuAccount {
            name: a.display_name,
            address: a.address,
        })
        .collect();
    let label = tr!("file-menu-send");
    if let Err(err) = write(daemon, &label, &accounts) {
        tracing::warn!(%err, "cannot update the file managers' menus");
    }
}

#[cfg(not(windows))]
fn write(daemon: &Daemon, label: &str, accounts: &[MenuAccount]) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let json = daemon.paths().data_dir().join(NAUTILUS_FILE);
    if let Some(dir) = json.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(&json, file_menus::nautilus_json(label, accounts))?;

    let Some(dir) = service_menus() else {
        return Ok(());
    };
    let path = dir.join(file_menus::service_menu_file());
    match file_menus::service_menu(label, accounts) {
        Some(menu) => {
            std::fs::create_dir_all(&dir)?;
            std::fs::write(&path, menu)?;
            // KIO runs a user's own service menus only when executable.
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))?;
        }
        // The package's one entry is the menu.
        None => match std::fs::remove_file(&path) {
            Err(err) if err.kind() != std::io::ErrorKind::NotFound => return Err(err),
            _ => {}
        },
    }
    Ok(())
}

/// The user's own service menu folder, `~/.local/share/kio/servicemenus`.
#[cfg(not(windows))]
fn service_menus() -> Option<PathBuf> {
    let data = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|dir| dir.is_absolute())
        .or_else(|| Some(PathBuf::from(std::env::var_os("HOME")?).join(".local/share")))?;
    Some(data.join("kio/servicemenus"))
}

#[cfg(windows)]
fn write(_daemon: &Daemon, label: &str, accounts: &[MenuAccount]) -> std::io::Result<()> {
    use winreg::RegKey;
    use winreg::enums::HKEY_CURRENT_USER;

    let exe = std::env::current_exe()?
        .parent()
        .map(|dir| dir.join("katna-mail.exe"))
        .filter(|exe| exe.is_file());
    // Not installed by Setup (a build run from its folder): leave the
    // registry alone.
    let Some(exe) = exe else {
        return Ok(());
    };
    let root = RegKey::predef(HKEY_CURRENT_USER);
    file_menus::windows::apply(&root, &exe.display().to_string(), label, accounts)
}
