// SPDX-License-Identifier: GPL-3.0-or-later

//! Embeds Katna Mail's translations (`i18n/<language>/katna-mail.ftl` and
//! `katna-ui.ftl`) for `katna_i18n::init`. English is in `katna-i18n`.

use std::fmt::Write as _;
use std::path::Path;

const FILES: &[&str] = &["katna-ui.ftl", "katna-mail.ftl"];

fn main() {
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
