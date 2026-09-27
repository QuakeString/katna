// SPDX-License-Identifier: GPL-3.0-or-later

//! Gathers the What's new highlights, one TOML file each in
//! `whats-new/highlights/`, into `$OUT_DIR/highlights.rs` for
//! `src/whats_new.rs`. One file per highlight, so changes merged side by
//! side never touch the same lines. `whats-new/README.md` says how to
//! write one.

use std::fmt::Write as _;
use std::path::Path;

fn main() {
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
