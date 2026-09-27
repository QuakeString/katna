// SPDX-License-Identifier: GPL-3.0-or-later

//! Embeds the English text (`i18n/en/*.ftl`), which every binary has.

use std::fmt::Write as _;
use std::path::Path;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../i18n");
    let english = root.join("en");
    println!("cargo:rerun-if-changed={}", english.display());
    println!(
        "cargo:rerun-if-changed={}",
        root.join("languages.toml").display()
    );
    let mut files: Vec<_> = std::fs::read_dir(&english)
        .expect("i18n/en exists")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "ftl"))
        .collect();
    files.sort();
    let mut out = String::from("&[\n");
    for path in files {
        let name = path.file_stem().unwrap().to_string_lossy().into_owned();
        let path = path.canonicalize().unwrap();
        writeln!(
            out,
            "    ({name:?}, include_str!({:?})),",
            path.display().to_string()
        )
        .unwrap();
    }
    out.push(']');
    let dest = Path::new(&std::env::var("OUT_DIR").unwrap()).join("english.rs");
    std::fs::write(dest, out).unwrap();
}
