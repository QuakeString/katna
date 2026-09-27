// SPDX-License-Identifier: GPL-3.0-or-later

//! Builds two lists into Katna Mail:
//!
//! - the translations (`i18n/<language>/katna-mail.ftl` and `katna-ui.ftl`)
//!   for `katna_i18n::init`, in `$OUT_DIR/translations.rs`; English is in
//!   `katna-i18n`;
//! - the What's new highlights, one TOML file each in
//!   `whats-new/highlights/`, in `$OUT_DIR/highlights.rs` for
//!   `src/whats_new.rs`. One file per highlight, so changes merged side by
//!   side never touch the same lines. `whats-new/README.md` says how to
//!   write one.

use std::fmt::Write as _;
use std::path::Path;

const FILES: &[&str] = &["katna-ui.ftl", "katna-mail.ftl"];

fn main() {
    translations();
    highlights();
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
        for file in FILES {
            let path = folder.join(file);
            if path.is_file() {
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
                text.split_whitespace().collect::<Vec<_>>().join(" ")
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
        writeln!(
            out,
            "    Highlight {{ name: {name:?}, title: {title:?}, text: {text:?}, animation: {animation} }},"
        )
        .unwrap();
    }
    out.push_str("];\n");

    let dest = Path::new(&std::env::var("OUT_DIR").unwrap()).join("highlights.rs");
    std::fs::write(dest, out).unwrap();
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
