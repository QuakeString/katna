// SPDX-License-Identifier: GPL-3.0-or-later

//! Packs the files Setup installs, embeds Setup's translations and, for
//! Windows, the icon and version resource.
//!
//! `$KATNA_SETUP_PAYLOAD` names the folder with the files to install
//! (`ci/windows-package.ps1` fills it); without it Setup carries nothing
//! and says so when it starts.

use std::fmt::Write as _;
use std::path::Path;

// Only the writer is used here.
#[allow(dead_code)]
#[path = "src/payload.rs"]
mod payload;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out = Path::new(&out_dir);
    pack(out);
    translations(out);
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        resources();
    }
}

fn pack(out: &Path) {
    println!("cargo:rerun-if-env-changed=KATNA_SETUP_PAYLOAD");
    let dest = out.join("payload.bin");
    let Some(dir) = std::env::var_os("KATNA_SETUP_PAYLOAD") else {
        std::fs::write(dest, []).unwrap();
        return;
    };
    let dir = Path::new(&dir);
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut file = std::io::BufWriter::new(std::fs::File::create(&dest).unwrap());
    // Setup is downloaded once and unpacked once: pack it hard.
    payload::write(dir, 19, &mut file).expect("packing the payload");
}

/// Setup's text in every language: `i18n/<language>/katna-setup/*.ftl`.
/// English is in `katna-i18n`.
fn translations(out: &Path) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../i18n");
    println!("cargo:rerun-if-changed={}", root.display());
    let mut files = Vec::new();
    for language in std::fs::read_dir(&root).unwrap().flatten() {
        let name = language.file_name().to_string_lossy().into_owned();
        if name == "en" || !language.path().is_dir() {
            continue;
        }
        let dir = language.path().join("katna-setup");
        println!("cargo:rerun-if-changed={}", dir.display());
        for file in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            if file.path().extension().is_some_and(|e| e == "ftl") {
                files.push((name.clone(), file.path().canonicalize().unwrap()));
            }
        }
    }
    files.sort();
    let mut text = String::from("&[\n");
    for (name, path) in files {
        writeln!(
            text,
            "    ({name:?}, include_str!({:?})),",
            path.display().to_string()
        )
        .unwrap();
    }
    text.push(']');
    std::fs::write(out.join("translations.rs"), text).unwrap();
}

/// The icon Explorer shows for Setup, and its description.
fn resources() {
    let icon = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packaging/windows/katna.ico");
    println!("cargo:rerun-if-changed={}", icon.display());
    let mut res = winresource::WindowsResource::new();
    res.set_icon(&icon.to_string_lossy())
        .set("FileDescription", "Katna Setup")
        .set("ProductName", "Katna Mail");
    res.compile().expect("compiling the Windows resources");
}
