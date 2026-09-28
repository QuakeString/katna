// SPDX-License-Identifier: GPL-3.0-or-later

//! Builds two lists into Katna Mail:
//!
//! - the translations (`i18n/<language>/katna-mail/*.ftl` and
//!   `katna-ui.ftl`) for `katna_i18n::init`, in `$OUT_DIR/translations.rs`; English is in
//!   `katna-i18n`;
//! - the What's new highlights, one TOML file each in
//!   `whats-new/highlights/`, in `$OUT_DIR/highlights.rs` for
//!   `src/whats_new.rs`. One file per highlight, so changes merged side by
//!   side never touch the same lines. `whats-new/README.md` says how to
//!   write one. Their translations, one `whats-new.toml` per language
//!   beside its `.ftl` files (`i18n/<language>/katna-mail/`), with a table
//!   per highlight named by its file, go in with them.

use std::fmt::Write as _;
use std::path::Path;

/// The binaries whose text Katna Mail shows: `<binary>.ftl` or the files
/// in `<binary>/` of each language folder.
const BINARIES: &[&str] = &["katna-ui", "katna-mail"];

fn main() {
    translations();
    highlights();
    windows_icon();
}

fn translations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../i18n");
    println!("cargo:rerun-if-changed={}", root.display());
    let mut folders: Vec<_> = std::fs::read_dir(&root)
        .expect("i18n exists")
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir() && e.file_name() != "en")
        .map(|e| e.path())
        .collect();
    folders.sort();
    let mut out = String::from("&[\n");
    for folder in folders {
        println!("cargo:rerun-if-changed={}", folder.display());
        let name = folder.file_name().unwrap().to_string_lossy().into_owned();
        for binary in BINARIES {
            let mut paths = Vec::new();
            let single = folder.join(format!("{binary}.ftl"));
            if single.is_file() {
                paths.push(single);
            }
            if let Ok(entries) = std::fs::read_dir(folder.join(binary)) {
                let mut files: Vec<_> = entries
                    .flatten()
                    .map(|e| e.path())
                    .filter(|p| p.extension().is_some_and(|e| e == "ftl"))
                    .collect();
                files.sort();
                paths.extend(files);
            }
            for path in paths {
                println!("cargo:rerun-if-changed={}", path.display());
                let path = path.canonicalize().unwrap();
                writeln!(
                    out,
                    "    ({name:?}, include_str!({:?})),",
                    path.display().to_string()
                )
                .unwrap();
            }
        }
    }
    out.push(']');
    let dest = Path::new(&std::env::var("OUT_DIR").unwrap()).join("translations.rs");
    std::fs::write(dest, out).unwrap();
}

fn highlights() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("whats-new");
    let highlights = dir.join("highlights");
    println!("cargo:rerun-if-changed={}", dir.display());

    let mut names: Vec<String> = std::fs::read_dir(&highlights)
        .unwrap_or_else(|e| panic!("{}: {e}", highlights.display()))
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    let stems: Vec<&str> = names
        .iter()
        .map(|file| file.strip_suffix(".toml").unwrap_or(file))
        .collect();
    let translated = translated_highlights(&stems);

    let mut out = String::from(
        "/// Every highlight, oldest first (in name order).\npub static HIGHLIGHTS: &[Highlight] = &[\n",
    );
    for file in &names {
        let Some(name) = file.strip_suffix(".toml") else {
            panic!("whats-new/highlights/{file}: only .toml files belong here");
        };
        assert!(
            is_name(name),
            "whats-new/highlights/{file}: name it YYYY-MM-DD-HHMM-short-slug.toml \
             (the UTC time it was written, lowercase words joined by -)"
        );
        let path = highlights.join(file);
        let source = std::fs::read_to_string(&path).unwrap();
        let table: toml::Table = source
            .parse()
            .unwrap_or_else(|e| panic!("whats-new/highlights/{file}: {e}"));
        for key in table.keys() {
            assert!(
                matches!(key.as_str(), "title" | "text" | "animation"),
                "whats-new/highlights/{file}: unknown key {key:?}"
            );
        }
        let field = |key: &str| {
            table.get(key).map(|value| {
                let text = value.as_str().unwrap_or_else(|| {
                    panic!("whats-new/highlights/{file}: {key} must be a string")
                });
                // Written across lines in the file, shown as one paragraph.
                paragraph(text)
            })
        };
        let title = field("title").filter(|t| !t.is_empty());
        let text = field("text").filter(|t| !t.is_empty());
        let (Some(title), Some(text)) = (title, text) else {
            panic!("whats-new/highlights/{file}: needs a title and a text");
        };
        let animation = match field("animation") {
            None => "None".to_owned(),
            Some(clip) => {
                let [light, dark] = ["light", "dark"].map(|theme| {
                    let path = dir.join(format!("{clip}-{theme}.webp"));
                    assert!(
                        path.is_file(),
                        "whats-new/highlights/{file}: missing {}",
                        path.display()
                    );
                    path
                });
                format!(
                    "Some(Animation {{ light: include_bytes!({:?}), dark: include_bytes!({:?}) }})",
                    light.display().to_string(),
                    dark.display().to_string()
                )
            }
        };
        let mut translations = String::from("&[");
        for (folder, title, text) in translated.get(name).into_iter().flatten() {
            write!(
                translations,
                "Translation {{ folder: {folder:?}, title: {title:?}, text: {text:?} }}, "
            )
            .unwrap();
        }
        translations.push(']');
        writeln!(
            out,
            "    Highlight {{ name: {name:?}, title: {title:?}, text: {text:?}, \
             translations: {translations}, animation: {animation} }},"
        )
        .unwrap();
    }
    out.push_str("];\n");

    let dest = Path::new(&std::env::var("OUT_DIR").unwrap()).join("highlights.rs");
    std::fs::write(dest, out).unwrap();
}

/// The highlights' translations, `i18n/<folder>/katna-mail/whats-new.toml`
/// (a table per highlight, named by its file: `title` and `text`), by
/// highlight: (folder, title, text), in folder order. A table for a
/// highlight that does not exist stops the build.
fn translated_highlights(
    stems: &[&str],
) -> std::collections::HashMap<String, Vec<(String, String, String)>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../i18n");
    let mut folders: Vec<_> = std::fs::read_dir(&root)
        .expect("i18n exists")
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir() && e.file_name() != "en")
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    folders.sort();
    let mut translated = std::collections::HashMap::<_, Vec<_>>::new();
    for folder in folders {
        let path = root.join(&folder).join("katna-mail/whats-new.toml");
        println!("cargo:rerun-if-changed={}", path.display());
        let Ok(source) = std::fs::read_to_string(&path) else {
            continue;
        };
        let file = format!("i18n/{folder}/katna-mail/whats-new.toml");
        let table: toml::Table = source.parse().unwrap_or_else(|e| panic!("{file}: {e}"));
        for (stem, value) in table {
            assert!(
                stems.contains(&stem.as_str()),
                "{file}: [{stem:?}] is not a highlight \
                 (apps/katna-mail/whats-new/highlights/{stem}.toml)"
            );
            let Some(entry) = value.as_table() else {
                panic!("{file}: {stem:?} must be a table with a title and a text");
            };
            for key in entry.keys() {
                assert!(
                    matches!(key.as_str(), "title" | "text"),
                    "{file}: [{stem:?}]: unknown key {key:?}"
                );
            }
            let [title, text] = ["title", "text"].map(|key| {
                let text = entry
                    .get(key)
                    .and_then(toml::Value::as_str)
                    .unwrap_or_else(|| panic!("{file}: [{stem:?}]: needs a {key} string"));
                // Written across lines in the file, shown as one paragraph.
                let text = paragraph(text);
                assert!(!text.is_empty(), "{file}: [{stem:?}]: {key} is empty");
                text
            });
            translated
                .entry(stem)
                .or_default()
                .push((folder.clone(), title, text));
        }
    }
    translated
}

/// `2026-09-27-0444-about-katna`: a date, a UTC time and a slug.
fn is_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    let digits = |range: std::ops::Range<usize>| {
        bytes
            .get(range)
            .is_some_and(|b| b.iter().all(u8::is_ascii_digit))
    };
    let slug = name.get(16..).unwrap_or_default();
    digits(0..4)
        && bytes.get(4) == Some(&b'-')
        && digits(5..7)
        && bytes.get(7) == Some(&b'-')
        && digits(8..10)
        && bytes.get(10) == Some(&b'-')
        && digits(11..15)
        && bytes.get(15) == Some(&b'-')
        && !slug.is_empty()
        && !slug.starts_with('-')
        && !slug.ends_with('-')
        && !slug.contains("--")
        && slug
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// `text`'s lines joined into one paragraph: with a space, except between
/// two Chinese or Japanese characters, which are written without spaces.
/// (Thai, Lao, Khmer and Burmese put a space between phrases, and their
/// lines break there, so they keep it.)
fn paragraph(text: &str) -> String {
    fn unspaced(c: char) -> bool {
        matches!(c as u32,
            0x3000..=0x30FF // CJK punctuation, kana
            | 0x3400..=0x4DBF | 0x4E00..=0x9FFF // CJK ideographs
            | 0xFF00..=0xFFEF) // full-width forms
    }
    let mut out = String::new();
    for word in text.split_whitespace() {
        let joins = out.chars().next_back().is_some_and(unspaced)
            && word.chars().next().is_some_and(unspaced);
        if !out.is_empty() && !joins {
            out.push(' ');
        }
        out.push_str(word);
    }
    out
}

/// The icon Explorer and the taskbar show for the program on Windows.
fn windows_icon() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let icon = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packaging/windows/katna.ico");
    println!("cargo:rerun-if-changed={}", icon.display());
    let mut res = winresource::WindowsResource::new();
    res.set_icon(&icon.to_string_lossy())
        .set("FileDescription", "Katna Mail")
        .set("ProductName", "Katna Mail");
    res.compile().expect("compiling the Windows resources");
}
