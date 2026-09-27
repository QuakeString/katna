// SPDX-License-Identifier: GPL-3.0-or-later

//! Embeds the English text, which every binary has: `i18n/en/<binary>.ftl`
//! or every `.ftl` file in `i18n/en/<binary>/`.

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
    let files = ftl_files(&english);
    let mut out = String::from("&[\n");
    for (name, path) in files {
        println!("cargo:rerun-if-changed={}", path.display());
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

/// The `.ftl` files of one language folder with the binary each belongs
/// to: `<binary>.ftl`, or any file in a `<binary>/` folder, sorted.
fn ftl_files(folder: &Path) -> Vec<(String, std::path::PathBuf)> {
    let is_ftl = |p: &Path| p.extension().is_some_and(|e| e == "ftl");
    let mut files = Vec::new();
    for entry in std::fs::read_dir(folder)
        .expect("language folder exists")
        .flatten()
    {
        let path = entry.path();
        let name = path.file_stem().unwrap().to_string_lossy().into_owned();
        if path.is_dir() {
            for file in std::fs::read_dir(&path).unwrap().flatten() {
                if is_ftl(&file.path()) {
                    files.push((name.clone(), file.path()));
                }
            }
        } else if is_ftl(&path) {
            files.push((name, path));
        }
    }
    files.sort();
    files
}
