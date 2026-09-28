// SPDX-License-Identifier: GPL-3.0-or-later

//! Builds the translations of the daemon's text (notifications, the tray;
//! `i18n/<language>/katna-daemon/*.ftl`) into `$OUT_DIR/translations.rs`
//! for `katna_i18n::init`. English is in `katna-i18n`.

use std::fmt::Write as _;
use std::path::Path;

/// The binaries whose text the daemon shows: `<binary>.ftl` or the files
/// in `<binary>/` of each language folder. `katna-notify`'s text is the
/// daemon's.
const BINARIES: &[&str] = &["katna-daemon"];

fn main() {
    windows_icon();
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
            let dir = folder.join(binary);
            println!("cargo:rerun-if-changed={}", dir.display());
            if let Ok(entries) = std::fs::read_dir(&dir) {
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

/// The icon Explorer and the taskbar show for the program on Windows.
fn windows_icon() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let icon = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packaging/windows/katna.ico");
    println!("cargo:rerun-if-changed={}", icon.display());
    let mut res = winresource::WindowsResource::new();
    res.set_icon(&icon.to_string_lossy())
        .set("FileDescription", "Katna background service")
        .set("ProductName", "Katna Mail");
    res.compile().expect("compiling the Windows resources");
}
