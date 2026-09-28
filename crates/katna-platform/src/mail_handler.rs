// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna Mail as a mail app on Windows (`docs/ARCHITECTURE.md` §27.1): the
//! registrations that list it in Settings > Apps > Default apps for
//! `mailto:` links, under `HKEY_CURRENT_USER` (or `HKEY_LOCAL_MACHINE` when
//! Setup installs for everyone).
//!
//! Windows 10 and later let only the user pick the default app, so Katna
//! registers itself and opens the Default apps page; the user's choice is
//! read back from `UserChoice`.

use std::io;
use std::path::Path;

use winreg::RegKey;
use winreg::enums::HKEY_CURRENT_USER;

/// Katna's class for `mailto:` links.
pub const MAILTO_CLASS: &str = "Katna.Mailto";

/// Where Katna describes what it can open.
const CAPABILITIES: &str = r"Software\Clients\Mail\Katna\Capabilities";

/// The registry values that make `exe` a mail app for this user:
/// (key, value name, data). An empty name is the key's default value.
pub fn entries(exe: &Path) -> Vec<(String, &'static str, String)> {
    let command = format!("\"{}\" \"%1\"", exe.display());
    let icon = format!("\"{}\",0", exe.display());
    let class = format!(r"Software\Classes\{MAILTO_CLASS}");
    vec![
        (class.clone(), "", "URL:MailTo Protocol".to_owned()),
        (class.clone(), "URL Protocol", String::new()),
        (format!(r"{class}\DefaultIcon"), "", icon),
        (format!(r"{class}\shell\open\command"), "", command.clone()),
        (
            r"Software\Clients\Mail\Katna".to_owned(),
            "",
            "Katna Mail".to_owned(),
        ),
        (
            r"Software\Clients\Mail\Katna\shell\open\command".to_owned(),
            "",
            format!("\"{}\"", exe.display()),
        ),
        (
            CAPABILITIES.to_owned(),
            "ApplicationName",
            "Katna Mail".to_owned(),
        ),
        (
            CAPABILITIES.to_owned(),
            "ApplicationDescription",
            "Fast, local-first email".to_owned(),
        ),
        (
            format!(r"{CAPABILITIES}\URLAssociations"),
            "mailto",
            MAILTO_CLASS.to_owned(),
        ),
        (
            r"Software\RegisteredApplications".to_owned(),
            "Katna Mail",
            CAPABILITIES.to_owned(),
        ),
    ]
}

/// Registers `exe` as a mail app for this user.
pub fn register(exe: &Path) -> io::Result<()> {
    register_in(&RegKey::predef(HKEY_CURRENT_USER), exe)
}

/// Registers `exe` as a mail app under `root`: `HKEY_LOCAL_MACHINE` for
/// everyone on the computer, when Setup installs for everyone.
pub fn register_in(root: &RegKey, exe: &Path) -> io::Result<()> {
    for (path, name, data) in entries(exe) {
        let (key, _) = root.create_subkey(&path)?;
        key.set_value(name, &data)?;
    }
    Ok(())
}

/// Whether the user picked Katna Mail for `mailto:` links.
pub fn is_default() -> bool {
    RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(
            r"Software\Microsoft\Windows\Shell\Associations\UrlAssociations\mailto\UserChoice",
        )
        .and_then(|key| key.get_value::<String, _>("ProgId"))
        .is_ok_and(|id| id == MAILTO_CLASS)
}

/// The page where the user picks the default mail app.
pub const DEFAULT_APPS_PAGE: &str = "ms-settings:defaultapps";
