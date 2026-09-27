// SPDX-License-Identifier: GPL-3.0-or-later

//! The desktop's default app for a MIME type or URL scheme, as the XDG
//! "Association between MIME types and applications" spec keeps it in
//! `mimeapps.list` files. Katna uses it to become the `mailto:` handler
//! (`docs/ARCHITECTURE.md` §15.2). Plasma and GNOME read the same files.

use std::io;
use std::path::{Path, PathBuf};

/// The URL scheme type of `mailto:` links.
pub const MAILTO: &str = "x-scheme-handler/mailto";

const DEFAULTS: &str = "Default Applications";

/// Where the lists live, most important first.
#[derive(Debug, Clone)]
pub struct Lists {
    /// `$XDG_CONFIG_HOME`: the user's choices, where changes are written.
    config_home: PathBuf,
    /// `$XDG_CONFIG_DIRS`, then `applications/` under the data dirs.
    others: Vec<PathBuf>,
    /// `$XDG_CURRENT_DESKTOP`, lower case: `kde`, `gnome`, …
    desktops: Vec<String>,
}

impl Lists {
    /// The lists of this session, from the XDG environment variables.
    pub fn from_env() -> Option<Self> {
        let var = |name: &str| std::env::var(name).ok().filter(|v| !v.is_empty());
        let home = PathBuf::from(var("HOME")?);
        let config_home =
            var("XDG_CONFIG_HOME").map_or_else(|| home.join(".config"), PathBuf::from);
        let data_home =
            var("XDG_DATA_HOME").map_or_else(|| home.join(".local/share"), PathBuf::from);
        let split = |value: Option<String>, default: &str| -> Vec<PathBuf> {
            value
                .as_deref()
                .unwrap_or(default)
                .split(':')
                .filter(|d| !d.is_empty())
                .map(PathBuf::from)
                .collect()
        };
        let mut others = split(var("XDG_CONFIG_DIRS"), "/etc/xdg");
        others.push(data_home.join("applications"));
        others.extend(
            split(var("XDG_DATA_DIRS"), "/usr/local/share:/usr/share")
                .into_iter()
                .map(|d| d.join("applications")),
        );
        let desktops = var("XDG_CURRENT_DESKTOP")
            .unwrap_or_default()
            .split(':')
            .filter(|d| !d.is_empty())
            .map(str::to_lowercase)
            .collect();
        Some(Self {
            config_home,
            others,
            desktops,
        })
    }

    /// The files to read, most important first: in each directory the
    /// desktop's own list before the shared one.
    fn files(&self) -> Vec<PathBuf> {
        std::iter::once(&self.config_home)
            .chain(&self.others)
            .flat_map(|dir| {
                self.desktops
                    .iter()
                    .map(|d| dir.join(format!("{d}-mimeapps.list")))
                    .chain(std::iter::once(dir.join("mimeapps.list")))
            })
            .collect()
    }

    /// The desktop file ID of the default app for `mime`, if one is set.
    pub fn default_app(&self, mime: &str) -> Option<String> {
        self.files().iter().find_map(|file| {
            let text = std::fs::read_to_string(file).ok()?;
            first_app(&text, mime)
        })
    }

    /// Makes `app` (a desktop file ID) the default for `mime`, in the
    /// user's `mimeapps.list` and in any desktop-specific list of theirs
    /// that names a default for it (which would win otherwise).
    pub fn set_default(&self, mime: &str, app: &str) -> io::Result<()> {
        std::fs::create_dir_all(&self.config_home)?;
        let shared = self.config_home.join("mimeapps.list");
        let desktop_lists = self
            .desktops
            .iter()
            .map(|d| self.config_home.join(format!("{d}-mimeapps.list")));
        for file in std::iter::once(shared.clone()).chain(desktop_lists) {
            let text = match std::fs::read_to_string(&file) {
                Ok(text) => text,
                Err(err) if err.kind() == io::ErrorKind::NotFound => String::new(),
                Err(err) => return Err(err),
            };
            if file != shared && first_app(&text, mime).is_none() {
                continue;
            }
            write(&file, &with_default(&text, mime, app))?;
        }
        Ok(())
    }
}

/// Replaces `path` in one step, so a reader never sees half a file.
fn write(path: &Path, text: &str) -> io::Result<()> {
    let tmp = path.with_extension("list.katna-tmp");
    std::fs::write(&tmp, text)?;
    std::fs::rename(&tmp, path)
}

/// The first app `text` (a `mimeapps.list`) names as the default for
/// `mime`.
fn first_app(text: &str, mime: &str) -> Option<String> {
    let mut in_defaults = false;
    for line in text.lines().map(str::trim) {
        if let Some(section) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            in_defaults = section == DEFAULTS;
        } else if in_defaults
            && let Some((key, value)) = line.split_once('=')
            && key.trim() == mime
        {
            return value
                .split(';')
                .map(str::trim)
                .find(|app| !app.is_empty())
                .map(str::to_owned);
        }
    }
    None
}

/// `text` (a `mimeapps.list`) with `app` as the default for `mime`, the
/// rest of the file as it was.
fn with_default(text: &str, mime: &str, app: &str) -> String {
    let entry = format!("{mime}={app};");
    let mut out = Vec::new();
    let (mut in_defaults, mut found_section, mut done) = (false, false, false);
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(section) = trimmed.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            if in_defaults && !done {
                insert_before_blank(&mut out, entry.clone());
                done = true;
            }
            in_defaults = section == DEFAULTS;
            found_section |= in_defaults;
        } else if in_defaults
            && let Some((key, _)) = trimmed.split_once('=')
            && key.trim() == mime
        {
            if !done {
                out.push(entry.clone());
                done = true;
            }
            continue;
        }
        out.push(line.to_owned());
    }
    if in_defaults && !done {
        insert_before_blank(&mut out, entry.clone());
        done = true;
    }
    if !found_section && !done {
        if out.last().is_some_and(|l| !l.trim().is_empty()) {
            out.push(String::new());
        }
        out.push(format!("[{DEFAULTS}]"));
        out.push(entry);
    }
    let mut text = out.join("\n");
    text.push('\n');
    text
}

/// Adds `line` at the end of the section just read, before its trailing
/// blank lines.
fn insert_before_blank(out: &mut Vec<String>, line: String) {
    let at = out
        .iter()
        .rposition(|l| !l.trim().is_empty())
        .map_or(out.len(), |i| i + 1);
    out.insert(at, line);
}

#[cfg(test)]
mod tests {
    use super::*;

    const APP: &str = "in.invenia.katna.Mail.desktop";

    #[test]
    fn reads_the_first_default() {
        let text = "[Added Associations]\nx-scheme-handler/mailto=a.desktop;\n\n\
                    [Default Applications]\ntext/plain=kate.desktop;\n\
                    x-scheme-handler/mailto=org.kde.kmail2.desktop;b.desktop;\n";
        assert_eq!(
            first_app(text, MAILTO).as_deref(),
            Some("org.kde.kmail2.desktop")
        );
        assert_eq!(first_app(text, "image/png"), None);
    }

    #[test]
    fn replaces_an_existing_default_in_place() {
        let text = "[Default Applications]\ntext/plain=kate.desktop;\n\
                    x-scheme-handler/mailto=org.kde.kmail2.desktop;\n\n\
                    [Added Associations]\ntext/plain=kate.desktop;\n";
        assert_eq!(
            with_default(text, MAILTO, APP),
            "[Default Applications]\ntext/plain=kate.desktop;\n\
             x-scheme-handler/mailto=in.invenia.katna.Mail.desktop;\n\n\
             [Added Associations]\ntext/plain=kate.desktop;\n"
        );
    }

    #[test]
    fn adds_to_the_section_or_creates_it() {
        let text = "[Default Applications]\ntext/plain=kate.desktop;\n\n[Other]\nx=y\n";
        assert_eq!(
            with_default(text, MAILTO, APP),
            "[Default Applications]\ntext/plain=kate.desktop;\n\
             x-scheme-handler/mailto=in.invenia.katna.Mail.desktop;\n\n[Other]\nx=y\n"
        );
        assert_eq!(
            with_default("[Added Associations]\na=b;\n", MAILTO, APP),
            "[Added Associations]\na=b;\n\n[Default Applications]\n\
             x-scheme-handler/mailto=in.invenia.katna.Mail.desktop;\n"
        );
        assert_eq!(
            with_default("", MAILTO, APP),
            "[Default Applications]\nx-scheme-handler/mailto=in.invenia.katna.Mail.desktop;\n"
        );
    }

    #[test]
    fn the_desktops_own_list_wins_and_is_updated() {
        let root = std::env::temp_dir().join(format!("katna-mimeapps-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let (home, system) = (root.join("config"), root.join("xdg"));
        std::fs::create_dir_all(&home).unwrap();
        std::fs::create_dir_all(&system).unwrap();
        std::fs::write(
            system.join("mimeapps.list"),
            "[Default Applications]\nx-scheme-handler/mailto=thunderbird.desktop;\n",
        )
        .unwrap();
        let lists = Lists {
            config_home: home.clone(),
            others: vec![system],
            desktops: vec!["kde".into()],
        };
        assert_eq!(
            lists.default_app(MAILTO).as_deref(),
            Some("thunderbird.desktop")
        );
        std::fs::write(
            home.join("kde-mimeapps.list"),
            "[Default Applications]\nx-scheme-handler/mailto=org.kde.kmail2.desktop;\n",
        )
        .unwrap();
        assert_eq!(
            lists.default_app(MAILTO).as_deref(),
            Some("org.kde.kmail2.desktop")
        );
        lists.set_default(MAILTO, APP).unwrap();
        assert_eq!(lists.default_app(MAILTO).as_deref(), Some(APP));
        let shared = std::fs::read_to_string(home.join("mimeapps.list")).unwrap();
        assert_eq!(first_app(&shared, MAILTO).as_deref(), Some(APP));
        std::fs::remove_dir_all(&root).unwrap();
    }
}
