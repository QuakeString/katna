// SPDX-License-Identifier: GPL-3.0-or-later

//! Signatures found in the mail apps on this computer: Thunderbird,
//! Evolution and KMail, read from their own settings files, never
//! changed. Gmail's come from the daemon. Signatures that run a program
//! (KMail's command, Evolution's script) are never run, and are left out.
//! Pictures on this computer that a signature shows are brought inside as
//! `data:` URIs; pictures on the web are left to the import, which
//! downloads them through the daemon. No GPUI here.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Largest signature file read.
const MAX_FILE: u64 = 256 * 1024;
/// Largest picture on this computer brought inside.
const MAX_PICTURE: u64 = 512 * 1024;

/// Where a signature was found.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Source {
    Gmail,
    Thunderbird,
    Evolution,
    KMail,
}

impl Source {
    /// The app's name, as its makers write it.
    pub fn name(self) -> &'static str {
        match self {
            Source::Gmail => "Gmail",
            Source::Thunderbird => "Thunderbird",
            Source::Evolution => "Evolution",
            Source::KMail => "KMail",
        }
    }
}

/// A signature found in another app.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Found {
    pub source: Source,
    /// The signature's name there, or the address it signs.
    pub name: String,
    /// As HTML, with pictures on this computer inside; empty for plain
    /// text.
    pub html: String,
    /// As plain text; empty when it is HTML.
    pub text: String,
}

/// Every signature the mail apps on this computer keep, for the user
/// whose home is `home`, with `config` and `data` the XDG config and data
/// directories.
pub fn find(home: &Path, config: &Path, data: &Path) -> Vec<Found> {
    let mut all = thunderbird(home, config);
    all.extend(evolution(home, config, data));
    all.extend(kmail(home, config));
    dedup(&mut all);
    all
}

/// Drops signatures seen twice (the same app installed twice).
fn dedup(all: &mut Vec<Found>) {
    let mut seen = std::collections::HashSet::new();
    all.retain(|f| seen.insert((f.source, f.html.clone(), f.text.clone())));
}

// Thunderbird

/// Thunderbird's identities with a signature, from every profile of its
/// usual, Flatpak and Snap installs.
fn thunderbird(home: &Path, config: &Path) -> Vec<Found> {
    let bases = [
        home.join(".thunderbird"),
        config.join("thunderbird"),
        home.join(".var/app/org.mozilla.Thunderbird/.thunderbird"),
        home.join(".var/app/net.thunderbird.Thunderbird/.thunderbird"),
        home.join("snap/thunderbird/common/.thunderbird"),
    ];
    let mut profiles: Vec<PathBuf> = Vec::new();
    for base in &bases {
        for profile in profile_dirs(base) {
            if !profiles.contains(&profile) {
                profiles.push(profile);
            }
        }
    }
    profiles
        .iter()
        .filter_map(|p| Some((p, read_small(&p.join("prefs.js"))?)))
        .flat_map(|(profile, prefs)| thunderbird_prefs(&prefs, profile))
        .collect()
}

/// The profile directories `profiles.ini` in `base` names, else every
/// directory there with a `prefs.js`.
fn profile_dirs(base: &Path) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(ini) = read_small(&base.join("profiles.ini")) {
        for group in ini_groups(&ini).values() {
            let Some(path) = group.get("Path") else {
                continue;
            };
            let relative = group.get("IsRelative").is_none_or(|r| r.trim() == "1");
            dirs.push(if relative {
                base.join(path)
            } else {
                PathBuf::from(path)
            });
        }
    }
    if dirs.is_empty()
        && let Ok(entries) = fs::read_dir(base)
    {
        dirs.extend(
            entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.join("prefs.js").is_file()),
        );
        dirs.sort();
    }
    dirs
}

/// The signatures of the identities in a `prefs.js`.
pub fn thunderbird_prefs(prefs: &str, profile: &Path) -> Vec<Found> {
    let values = prefs_values(prefs);
    // Identities in the order they were made: id1, id2, ...
    let mut ids: Vec<&str> = values
        .keys()
        .filter_map(|k| k.strip_prefix("mail.identity.")?.split_once('.'))
        .map(|(id, _)| id)
        .collect();
    ids.sort_by_key(|id| {
        (
            id.trim_start_matches("id")
                .parse::<u32>()
                .unwrap_or(u32::MAX),
            *id,
        )
    });
    ids.dedup();
    let mut found = Vec::new();
    for id in ids {
        let get = |key: &str| values.get(&format!("mail.identity.{id}.{key}"));
        let on = |key: &str| get(key).is_some_and(|v| v == "true");
        let mut html = None;
        let mut text = None;
        if on("attach_signature") {
            // A file: `sig_file`, or `sig_file-rel` as `[ProfD]name`.
            let file = get("sig_file").map(PathBuf::from).or_else(|| {
                get("sig_file-rel")?
                    .strip_prefix("[ProfD]")
                    .map(|rel| profile.join(rel))
            });
            if let Some(file) = file
                && let Some(content) = read_small(&file)
            {
                let base = file.parent().map(Path::to_path_buf);
                if looks_html(&file, &content) {
                    html = Some(inline_local_pictures(&content, base.as_deref()));
                } else {
                    text = Some(content);
                }
            }
        }
        if html.is_none() && text.is_none() {
            let Some(sig) = get("htmlSigText").filter(|s| !s.trim().is_empty()) else {
                continue;
            };
            if on("htmlSigFormat") {
                html = Some(inline_local_pictures(sig, Some(profile)));
            } else {
                text = Some(sig.clone());
            }
        }
        let address = get("useremail").cloned().unwrap_or_default();
        let name = get("fullName").cloned().unwrap_or_default();
        push(
            &mut found,
            Source::Thunderbird,
            label(&name, &address),
            html,
            text,
        );
    }
    found
}

/// The values of `user_pref("name", value);` lines: strings unescaped,
/// numbers and booleans as written.
fn prefs_values(prefs: &str) -> HashMap<String, String> {
    let mut values = HashMap::new();
    for line in prefs.lines() {
        let Some(rest) = line.trim().strip_prefix("user_pref(") else {
            continue;
        };
        let Some((name, rest)) = js_string(rest.trim_start()) else {
            continue;
        };
        let Some(rest) = rest.trim_start().strip_prefix(',') else {
            continue;
        };
        let rest = rest.trim_start();
        let value = match js_string(rest) {
            Some((value, _)) => value,
            None => rest
                .trim_end()
                .trim_end_matches(';')
                .trim_end()
                .trim_end_matches(')')
                .trim()
                .to_owned(),
        };
        values.insert(name, value);
    }
    values
}

/// A double-quoted JavaScript string at the start of `s`, and what
/// follows it.
fn js_string(s: &str) -> Option<(String, &str)> {
    let mut chars = s.char_indices();
    if chars.next()?.1 != '"' {
        return None;
    }
    let mut out = String::new();
    while let Some((i, c)) = chars.next() {
        match c {
            '"' => return Some((out, &s[i + 1..])),
            '\\' => {
                let (_, e) = chars.next()?;
                match e {
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    'u' => {
                        let hex: String =
                            (0..4).filter_map(|_| chars.next().map(|c| c.1)).collect();
                        let unit = u16::from_str_radix(&hex, 16).ok()?;
                        out.push(char::from_u32(u32::from(unit)).unwrap_or('\u{fffd}'));
                    }
                    other => out.push(other),
                }
            }
            c => out.push(c),
        }
    }
    None
}

// Evolution

/// Evolution's signatures: a `.source` key file each under its config
/// directory, the signature itself under its data directory, named by
/// the source's ID.
fn evolution(home: &Path, config: &Path, data: &Path) -> Vec<Found> {
    let installs = [
        (config.join("evolution"), data.join("evolution")),
        (
            home.join(".var/app/org.gnome.Evolution/config/evolution"),
            home.join(".var/app/org.gnome.Evolution/data/evolution"),
        ),
    ];
    let mut found = Vec::new();
    for (config, data) in installs {
        let Ok(entries) = fs::read_dir(config.join("sources")) else {
            continue;
        };
        let mut sources: Vec<PathBuf> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|e| e == "source"))
            .collect();
        sources.sort();
        for source in sources {
            let Some(key_file) = read_small(&source) else {
                continue;
            };
            let groups = ini_groups(&key_file);
            let Some(signature) = groups.get("Mail Signature") else {
                continue;
            };
            let Some(uid) = source.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let file = data.join("signatures").join(uid);
            // A script is kept as a link to it: never run, never read.
            if fs::symlink_metadata(&file).is_ok_and(|m| m.file_type().is_symlink()) {
                continue;
            }
            let Some(content) = read_small(&file) else {
                continue;
            };
            let name = groups
                .get("Data Source")
                .and_then(|g| g.get("DisplayName"))
                .cloned()
                .unwrap_or_default();
            let is_html = signature
                .get("MimeType")
                .is_some_and(|m| m.trim() == "text/html");
            let (html, text) = if is_html {
                (Some(inline_local_pictures(&content, None)), None)
            } else {
                (None, Some(content))
            };
            push(&mut found, Source::Evolution, name, html, text);
        }
    }
    found
}

// KMail

/// KMail's identities with a signature, from `emailidentities`.
fn kmail(home: &Path, config: &Path) -> Vec<Found> {
    let files = [
        config.join("emailidentities"),
        home.join(".var/app/org.kde.kmail2/config/emailidentities"),
    ];
    files
        .iter()
        .filter_map(|f| read_small(f))
        .flat_map(|content| kmail_identities(&content))
        .collect()
}

/// The signatures of the identities in an `emailidentities` file.
pub fn kmail_identities(content: &str) -> Vec<Found> {
    let groups = ini_groups(content);
    let mut identities: Vec<(&String, &HashMap<String, String>)> = groups
        .iter()
        .filter(|(name, _)| name.starts_with("Identity #"))
        .collect();
    identities.sort_by_key(|(name, _)| {
        name.trim_start_matches("Identity #")
            .parse::<u32>()
            .unwrap_or(u32::MAX)
    });
    let mut found = Vec::new();
    for (_, group) in identities {
        let get = |key: &str| group.get(key).map(|v| kconfig_unescape(v));
        let is_html = get("Inline Signature Is Html").is_some_and(|v| v == "true");
        let images = get("Image Location").map(PathBuf::from);
        let (html, text) = match get("Signature Type").as_deref() {
            Some("inline") => {
                let Some(sig) = get("Inline Signature").filter(|s| !s.trim().is_empty()) else {
                    continue;
                };
                if is_html {
                    (Some(inline_local_pictures(&sig, images.as_deref())), None)
                } else {
                    (None, Some(sig))
                }
            }
            Some("file") => {
                let Some(file) = get("Signature File").map(PathBuf::from) else {
                    continue;
                };
                let Some(content) = read_small(&file) else {
                    continue;
                };
                if looks_html(&file, &content) {
                    let base = file.parent().map(Path::to_path_buf);
                    (Some(inline_local_pictures(&content, base.as_deref())), None)
                } else {
                    (None, Some(content))
                }
            }
            // "command" runs a program: never. "disabled": none.
            _ => continue,
        };
        let name = get("Identity")
            .filter(|n| !n.trim().is_empty())
            .unwrap_or_else(|| {
                label(
                    &get("Name").unwrap_or_default(),
                    &get("Email Address").unwrap_or_default(),
                )
            });
        push(&mut found, Source::KMail, name, html, text);
    }
    found
}

/// A KConfig value with its escapes (`\n`, `\t`, `\s`, `\\`, `\xNN`)
/// undone.
fn kconfig_unescape(value: &str) -> String {
    let mut out = Vec::with_capacity(value.len());
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            i += 1;
            match bytes[i] {
                b'n' => out.push(b'\n'),
                b't' => out.push(b'\t'),
                b'r' => out.push(b'\r'),
                b's' => out.push(b' '),
                b'x' if i + 2 < bytes.len() => match u8::from_str_radix(&value[i + 1..i + 3], 16) {
                    Ok(b) => {
                        out.push(b);
                        i += 2;
                    }
                    Err(_) => out.push(b'x'),
                },
                other => out.push(other),
            }
        } else {
            out.push(bytes[i]);
        }
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

// Shared

/// The groups of an INI or key file: group → key → value. Keys with a
/// locale or option (`Name[de]`, `Name[$e]`) keep only the plain one.
fn ini_groups(content: &str) -> HashMap<String, HashMap<String, String>> {
    let mut groups: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut current = String::new();
    for line in content.lines() {
        let line = line.trim_start();
        if line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if let Some(name) = line
            .strip_prefix('[')
            .and_then(|l| l.trim_end().strip_suffix(']'))
        {
            current = name.to_owned();
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let key = match key.split_once('[') {
            Some((plain, option)) if option.starts_with('$') => plain.trim(),
            Some(_) => continue,
            None => key,
        };
        groups
            .entry(current.clone())
            .or_default()
            .entry(key.to_owned())
            .or_insert_with(|| value.to_owned());
    }
    groups
}

fn push(
    found: &mut Vec<Found>,
    source: Source,
    name: String,
    html: Option<String>,
    text: Option<String>,
) {
    let html = html.filter(|h| !h.trim().is_empty()).unwrap_or_default();
    let text = if html.is_empty() {
        text.unwrap_or_default().trim_end().to_owned()
    } else {
        String::new()
    };
    if html.is_empty() && text.trim().is_empty() {
        return;
    }
    let name = if name.trim().is_empty() {
        source.name().to_owned()
    } else {
        name.trim().to_owned()
    };
    found.push(Found {
        source,
        name,
        html,
        text,
    });
}

/// The address an identity signs, else its name: short, for the name
/// the signature gets.
fn label(name: &str, address: &str) -> String {
    if address.trim().is_empty() {
        name.trim().to_owned()
    } else {
        address.trim().to_owned()
    }
}

fn read_small(path: &Path) -> Option<String> {
    let meta = fs::metadata(path).ok()?;
    if !meta.is_file() || meta.len() > MAX_FILE {
        return None;
    }
    let bytes = fs::read(path).ok()?;
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

fn looks_html(path: &Path, content: &str) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| matches!(e.to_ascii_lowercase().as_str(), "html" | "htm" | "xhtml"))
        || content.trim_start().starts_with('<')
}

/// `html` with the pictures on this computer it shows (`file:` URLs,
/// absolute paths, and paths relative to `base`) inside as `data:` URIs.
/// Pictures on the web and `data:` ones stay as they are; one that can't
/// be read is left for the cleaning to drop.
pub fn inline_local_pictures(html: &str, base: Option<&Path>) -> String {
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(at) = find_src(rest) {
        let (before, after) = rest.split_at(at);
        out.push_str(before);
        let quote = after.as_bytes()[0];
        let Some(end) = after[1..].find(quote as char) else {
            out.push_str(after);
            return out;
        };
        let url = &after[1..1 + end];
        out.push(quote as char);
        match local_picture(url, base) {
            Some(data) => out.push_str(&data),
            None => out.push_str(url),
        }
        out.push(quote as char);
        rest = &after[end + 2..];
    }
    out.push_str(rest);
    out
}

/// Where the quoted value of the next `src=` attribute starts.
fn find_src(s: &str) -> Option<usize> {
    let lower = s.to_ascii_lowercase();
    let mut from = 0;
    while let Some(i) = lower[from..].find("src") {
        let at = from + i;
        let before = lower[..at].chars().next_back();
        let rest = lower[at + 3..].trim_start();
        if before.is_some_and(|c| c.is_ascii_whitespace())
            && let Some(value) = rest.strip_prefix('=')
        {
            let value = value.trim_start();
            if value.starts_with('"') || value.starts_with('\'') {
                return Some(s.len() - value.len());
            }
        }
        from = at + 3;
    }
    None
}

fn local_picture(url: &str, base: Option<&Path>) -> Option<String> {
    let lower = url.trim().to_ascii_lowercase();
    if ["http:", "https:", "data:", "cid:"]
        .iter()
        .any(|scheme| lower.starts_with(scheme))
    {
        return None;
    }
    let path = if let Some(path) = url.trim().strip_prefix("file://") {
        PathBuf::from(percent_decode(path))
    } else if url.starts_with('/') {
        PathBuf::from(url)
    } else if url.contains(':') {
        return None;
    } else {
        base?.join(percent_decode(url))
    };
    let meta = fs::metadata(&path).ok()?;
    if !meta.is_file() || meta.len() > MAX_PICTURE {
        return None;
    }
    let bytes = fs::read(&path).ok()?;
    let mime = picture_type(&bytes)?;
    Some(format!(
        "data:{mime};base64,{}",
        katna_ui::rich::html::base64_encode(&bytes)
    ))
}

fn picture_type(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(b"\x89PNG") {
        Some("image/png")
    } else if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
        Some("image/jpeg")
    } else if bytes.starts_with(b"GIF8") {
        Some("image/gif")
    } else if bytes.len() > 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        Some("image/webp")
    } else {
        None
    }
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let Ok(b) = u8::from_str_radix(&s[i + 1..i + 3], 16)
        {
            out.push(b);
            i += 3;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests;
