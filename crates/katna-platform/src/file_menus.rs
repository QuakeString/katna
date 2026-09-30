// SPDX-License-Identifier: GPL-3.0-or-later

//! "Send with Katna Mail" in the file managers' right-click menus
//! (`docs/ARCHITECTURE.md` §15.2): one entry that starts
//! `katna-mail --attach FILE…`, or with several mail accounts a submenu
//! with one line per account (`--attach --from ADDRESS FILE…`).
//!
//! - Dolphin reads service menus, fixed files: the package installs the
//!   one-entry menu, and the daemon writes the user's own copy of the same
//!   file, with the submenu, while there are several accounts (KIO takes
//!   the user's file over the system's of the same name).
//! - GNOME Files runs Katna's small Python extension, which reads the
//!   accounts from [`nautilus_json`] each time the menu opens.
//! - Windows Explorer reads the registry ([`registry_entries`]), which
//!   Setup fills for one entry and the daemon rewrites with the accounts.
//!
//! No I/O here but [`windows::apply`].

use katna_core::ids;

/// A mail account as a line of the submenu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuAccount {
    pub name: String,
    pub address: String,
}

impl MenuAccount {
    /// "Work (kay@example.com)", or the address alone when the name is
    /// empty or the address itself.
    pub fn label(&self) -> String {
        let name = self.name.trim();
        if name.is_empty() || name.eq_ignore_ascii_case(&self.address) {
            self.address.clone()
        } else {
            format!("{name} ({})", self.address)
        }
    }
}

/// The service menu's file name, the same in `/usr/share/kio/servicemenus`
/// (from the package) and in the user's `~/.local/share/kio/servicemenus`.
pub fn service_menu_file() -> String {
    format!("{}.SendFiles.desktop", ids::MAIL_APP_ID)
}

/// The user's Dolphin service menu for `accounts`, titled `label`: `None`
/// when there are fewer than two, as the package's one entry does.
pub fn service_menu(label: &str, accounts: &[MenuAccount]) -> Option<String> {
    if accounts.len() < 2 {
        return None;
    }
    let mut text = format!(
        "# Written by katna-daemon for your mail accounts; it rewrites this\n\
         # file when they change.\n\
         [Desktop Entry]\n\
         Type=Service\n\
         MimeType=all/all;\n\
         X-KDE-Protocols=file\n\
         X-KDE-Priority=TopLevel\n\
         X-KDE-Submenu={}\n\
         Icon={}\n\
         Actions={};\n",
        entry_value(label),
        ids::MAIL_APP_ID,
        (0..accounts.len())
            .map(|ix| format!("account{ix}"))
            .collect::<Vec<_>>()
            .join(";"),
    );
    for (ix, account) in accounts.iter().enumerate() {
        text.push_str(&format!(
            "\n[Desktop Action account{ix}]\nName={}\nIcon={}\nExec=katna-mail --attach --from {} %F\n",
            entry_value(&account.label()),
            ids::MAIL_APP_ID,
            exec_arg(&account.address),
        ));
    }
    Some(text)
}

/// `value` on one line of a desktop file.
fn entry_value(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace(['\n', '\r'], " ")
}

/// `arg` as one argument of a desktop file's `Exec` line: quoted, with the
/// characters the specification reserves escaped, `%` doubled and the
/// string's own backslashes doubled again (the file's value is unescaped
/// before the command line is read).
fn exec_arg(arg: &str) -> String {
    let mut quoted = String::from("\"");
    for c in arg.chars() {
        match c {
            '"' | '`' | '$' => {
                quoted.push_str("\\\\");
                quoted.push(c);
            }
            '\\' => quoted.push_str("\\\\\\\\"),
            '%' => quoted.push_str("%%"),
            '\n' | '\r' => quoted.push(' '),
            c => quoted.push(c),
        }
    }
    quoted.push('"');
    quoted
}

/// What GNOME Files' extension reads: the menu's label and the accounts.
pub fn nautilus_json(label: &str, accounts: &[MenuAccount]) -> String {
    let accounts: Vec<_> = accounts
        .iter()
        .map(|a| {
            format!(
                "{{\"label\":{},\"address\":{}}}",
                json_string(&a.label()),
                json_string(&a.address)
            )
        })
        .collect();
    format!(
        "{{\"label\":{},\"accounts\":[{}]}}\n",
        json_string(label),
        accounts.join(",")
    )
}

fn json_string(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if u32::from(c) < 0x20 => out.push_str(&format!("\\u{:04x}", u32::from(c))),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// The registry key of the menu entry on files and on folders, under
/// `HKEY_CURRENT_USER` (or `HKEY_LOCAL_MACHINE`).
pub const REGISTRY_VERBS: [&str; 2] = [
    r"Software\Classes\*\shell\KatnaMail.Send",
    r"Software\Classes\Directory\shell\KatnaMail.Send",
];

/// Where the submenu's lines are, under `Software\Classes`.
const SUBMENU: &str = "KatnaMail.SendMenu";

/// Every key [`registry_entries`] writes, to delete before writing them
/// again and when Katna is removed.
pub fn registry_keys() -> Vec<String> {
    let mut keys: Vec<String> = REGISTRY_VERBS.iter().map(|k| (*k).to_owned()).collect();
    keys.push(format!(r"Software\Classes\{SUBMENU}"));
    keys
}

/// The registry values of the Explorer entry for Katna Mail at `exe`:
/// (key, value name, data), an empty name for the key's default value.
/// With fewer than two `accounts` it is one entry, else a submenu.
pub fn registry_entries(
    exe: &str,
    label: &str,
    accounts: &[MenuAccount],
) -> Vec<(String, &'static str, String)> {
    let icon = format!("\"{exe}\",0");
    let mut entries = Vec::new();
    for verb in REGISTRY_VERBS {
        entries.push((verb.to_owned(), "MUIVerb", label.to_owned()));
        entries.push((verb.to_owned(), "Icon", icon.clone()));
        // Any number of files, not only up to 15.
        entries.push((verb.to_owned(), "MultiSelectModel", "Player".to_owned()));
        if accounts.len() < 2 {
            entries.push((
                format!(r"{verb}\command"),
                "",
                format!("\"{exe}\" --attach \"%1\""),
            ));
        } else {
            entries.push((verb.to_owned(), "SubCommands", String::new()));
            entries.push((
                verb.to_owned(),
                "ExtendedSubCommandsKey",
                SUBMENU.to_owned(),
            ));
        }
    }
    if accounts.len() >= 2 {
        for (ix, account) in accounts.iter().enumerate() {
            let key = format!(r"Software\Classes\{SUBMENU}\shell\{ix:02}");
            entries.push((key.clone(), "MUIVerb", account.label()));
            let address = account.address.replace('"', "");
            entries.push((
                format!(r"{key}\command"),
                "",
                format!("\"{exe}\" --attach --from \"{address}\" \"%1\""),
            ));
        }
    }
    entries
}

/// Writes the Explorer entry into the registry.
#[cfg(windows)]
pub mod windows {
    use std::io;

    use winreg::RegKey;

    use super::MenuAccount;

    /// Replaces the entry under `root` with the one for `accounts`.
    pub fn apply(
        root: &RegKey,
        exe: &str,
        label: &str,
        accounts: &[MenuAccount],
    ) -> io::Result<()> {
        remove(root);
        for (path, name, data) in super::registry_entries(exe, label, accounts) {
            let (key, _) = root.create_subkey(&path)?;
            key.set_value(name, &data)?;
        }
        Ok(())
    }

    /// Takes the entry out of the menus.
    pub fn remove(root: &RegKey) {
        for key in super::registry_keys() {
            let _ = root.delete_subkey_all(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn accounts() -> Vec<MenuAccount> {
        vec![
            MenuAccount {
                name: "Work".to_owned(),
                address: "kay@example.com".to_owned(),
            },
            MenuAccount {
                name: "kay.home@example.org".to_owned(),
                address: "kay.home@example.org".to_owned(),
            },
        ]
    }

    #[test]
    fn labels_show_the_name_when_it_says_more() {
        let [work, home] = accounts().try_into().unwrap();
        assert_eq!(work.label(), "Work (kay@example.com)");
        assert_eq!(home.label(), "kay.home@example.org");
    }

    #[test]
    fn one_account_keeps_the_packaged_menu() {
        assert_eq!(service_menu("Send with Katna Mail", &accounts()[..1]), None);
        assert_eq!(service_menu("Send with Katna Mail", &[]), None);
    }

    #[test]
    fn several_accounts_get_a_submenu() {
        let menu = service_menu("Send with Katna Mail", &accounts()).unwrap();
        assert!(menu.contains("X-KDE-Submenu=Send with Katna Mail\n"));
        assert!(menu.contains("Actions=account0;account1;\n"));
        assert!(menu.contains(
            "[Desktop Action account0]\nName=Work (kay@example.com)\nIcon=in.invenia.katna.Mail\n\
             Exec=katna-mail --attach --from \"kay@example.com\" %F\n"
        ));
        assert!(menu.contains("Exec=katna-mail --attach --from \"kay.home@example.org\" %F"));
        assert_eq!(
            service_menu_file(),
            "in.invenia.katna.Mail.SendFiles.desktop"
        );
    }

    #[test]
    fn exec_arguments_are_escaped() {
        assert_eq!(exec_arg("a\"b"), "\"a\\\\\"b\"");
        assert_eq!(exec_arg("100%"), "\"100%%\"");
        assert_eq!(exec_arg("$x"), "\"\\\\$x\"");
    }

    #[test]
    fn nautilus_gets_label_and_accounts() {
        assert_eq!(
            nautilus_json("Send \"it\"", &accounts()[..1]),
            "{\"label\":\"Send \\\"it\\\"\",\"accounts\":\
             [{\"label\":\"Work (kay@example.com)\",\"address\":\"kay@example.com\"}]}\n"
        );
    }

    #[test]
    fn explorer_gets_one_entry_or_a_submenu() {
        let exe = r"C:\Users\kay\AppData\Local\Programs\Katna\katna-mail.exe";
        let one = registry_entries(exe, "Send with Katna Mail", &accounts()[..1]);
        assert!(one.contains(&(
            r"Software\Classes\*\shell\KatnaMail.Send\command".to_owned(),
            "",
            format!("\"{exe}\" --attach \"%1\""),
        )));
        assert!(one.iter().all(|(key, ..)| !key.contains(SUBMENU)));

        let several = registry_entries(exe, "Send with Katna Mail", &accounts());
        assert!(several.contains(&(
            r"Software\Classes\Directory\shell\KatnaMail.Send".to_owned(),
            "ExtendedSubCommandsKey",
            SUBMENU.to_owned(),
        )));
        assert!(several.contains(&(
            r"Software\Classes\KatnaMail.SendMenu\shell\01\command".to_owned(),
            "",
            format!("\"{exe}\" --attach --from \"kay.home@example.org\" \"%1\""),
        )));
        assert!(
            several
                .iter()
                .all(|(key, ..)| !key.ends_with(r"KatnaMail.Send\command"))
        );
        for (key, ..) in several {
            assert!(registry_keys().iter().any(|k| key.starts_with(k.as_str())));
        }
    }
}
