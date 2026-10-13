// SPDX-License-Identifier: GPL-3.0-or-later

//! The `.desktop` files' names come from `i18n/en/desktop.ftl`: each file
//! names its ids with a `# i18n: <prefix>` line, its English text matches
//! the messages, and `packaging/linux/localize-desktop.sh` adds the other
//! languages when packaging.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use fluent_syntax::ast;

/// The keys a menu shows, translated per language (`Key[de]=`).
const KEYS: [&str; 4] = ["Name", "GenericName", "Comment", "Keywords"];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every desktop-entry file in `packaging/` and `integrations/`.
fn desktop_files() -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut stack = vec![root().join("packaging"), root().join("integrations")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for path in entries.filter_map(Result::ok).map(|e| e.path()) {
            if path.is_dir() {
                stack.push(path);
            } else if let Ok(text) = std::fs::read_to_string(&path)
                && text.lines().any(|l| l.trim() == "[Desktop Entry]")
            {
                found.push(path);
            }
        }
    }
    found.sort();
    assert!(!found.is_empty(), "no .desktop files found");
    found
}

/// The single-line messages of a Fluent file: id → text.
fn messages(text: &str) -> BTreeMap<String, String> {
    let resource = fluent_syntax::parser::parse(text).unwrap_or_else(|(_, e)| panic!("{e:?}"));
    let mut out = BTreeMap::new();
    for entry in resource.body {
        let ast::Entry::Message(message) = entry else {
            continue;
        };
        let Some(value) = message.value else {
            continue;
        };
        let mut text = String::new();
        for element in &value.elements {
            match element {
                ast::PatternElement::TextElement { value } => text.push_str(value),
                ast::PatternElement::Placeable { .. } => {
                    panic!("{}: a .desktop name has no placeables", message.id.name)
                }
            }
        }
        assert!(
            !text.contains('\n'),
            "{}: a .desktop name is one line",
            message.id.name
        );
        out.insert(message.id.name.to_owned(), text);
    }
    out
}

fn english() -> BTreeMap<String, String> {
    let path = root().join("i18n/en/desktop.ftl");
    messages(&std::fs::read_to_string(&path).expect("i18n/en/desktop.ftl"))
}

/// A file's translatable lines as (message id, key, English text), and
/// its `# i18n:` prefix.
fn entries(path: &Path) -> (Option<String>, Vec<(String, String, String)>) {
    let text = std::fs::read_to_string(path).unwrap();
    let prefix = text
        .lines()
        .find_map(|l| l.strip_prefix("# i18n:"))
        .map(|p| p.trim().to_owned());
    let mut group = String::new();
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim_end();
        if line.starts_with('[') {
            group = line.to_owned();
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if line.starts_with('#') {
            continue;
        }
        let base = key.split('[').next().unwrap_or(key);
        if !KEYS.contains(&base) {
            continue;
        }
        assert_eq!(
            base,
            key,
            "{}: {key} is written by localize-desktop.sh from i18n/<language>/desktop.ftl, not by hand",
            path.display()
        );
        let p = prefix.as_deref().unwrap_or("?");
        let id = if group == "[Desktop Entry]" {
            let suffix = match key {
                "GenericName" => "generic-name".to_owned(),
                other => other.to_lowercase(),
            };
            format!("{p}-{suffix}")
        } else if let Some(action) = group
            .strip_prefix("[Desktop Action ")
            .and_then(|a| a.strip_suffix(']'))
            && key == "Name"
        {
            format!("{p}-action-{action}")
        } else {
            continue;
        };
        out.push((id, key.to_owned(), value.to_owned()));
    }
    (prefix, out)
}

/// Each `.desktop` file's English names are its messages in
/// `i18n/en/desktop.ftl`, and every message there belongs to a file.
#[test]
fn desktop_files_match_english() {
    let english = english();
    let mut used = BTreeSet::new();
    let mut problems = Vec::new();
    for path in desktop_files() {
        let (prefix, lines) = entries(&path);
        if lines.is_empty() {
            continue;
        }
        if prefix.is_none() {
            problems.push(format!(
                "{}: no \"# i18n: <prefix>\" line naming its ids in i18n/en/desktop.ftl",
                path.display()
            ));
            continue;
        }
        for (id, key, value) in lines {
            match english.get(&id) {
                None => problems.push(format!(
                    "{}: {key}={value} has no message {id} in i18n/en/desktop.ftl",
                    path.display()
                )),
                Some(text) if *text != value => problems.push(format!(
                    "{}: {key}={value}, but i18n/en/desktop.ftl has {id} = {text}",
                    path.display()
                )),
                Some(_) => {}
            }
            used.insert(id);
        }
    }
    for id in english.keys().filter(|id| !used.contains(*id)) {
        problems.push(format!("i18n/en/desktop.ftl: {id} is in no .desktop file"));
    }
    assert!(problems.is_empty(), "{problems:#?}");
}

/// Translations of `desktop.ftl` hold only its ids, on one line each, so
/// the packaging script can read them.
#[test]
fn desktop_translations_fit() {
    let english = english();
    let mut problems = Vec::new();
    for entry in std::fs::read_dir(root().join("i18n")).unwrap().flatten() {
        let path = entry.path().join("desktop.ftl");
        if entry.file_name() == "en" || !path.is_file() {
            continue;
        }
        let text = std::fs::read_to_string(&path).unwrap();
        for id in messages(&text).keys() {
            if !english.contains_key(id) {
                problems.push(format!("{}: {id} is not in English", path.display()));
            }
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}

/// The packaging script writes `Key[locale]=` lines from the translations
/// and leaves everything else as it was.
#[cfg(unix)]
#[test]
fn localize_script_adds_translations() {
    let tmp = Path::new(env!("CARGO_TARGET_TMPDIR")).join("localize-desktop");
    let _ = std::fs::remove_dir_all(&tmp);
    for (folder, text) in [
        (
            "de",
            "desktop-mail-name = Katna Mail\n\
             desktop-mail-generic-name = E-Mail für alle Ihre Konten\n\
             desktop-mail-keywords = E-Mail;Post\n\
             desktop-mail-action-inbox = Posteingang öffnen\n\
             desktop-mail-comment =\n    zwei\n    Zeilen\n",
        ),
        ("zh-Hans", "desktop-mail-action-inbox = 打开收件箱\n"),
        (
            "pt-BR",
            "desktop-mail-action-inbox = Abrir a caixa de entrada\n",
        ),
    ] {
        std::fs::create_dir_all(tmp.join("i18n").join(folder)).unwrap();
        std::fs::write(tmp.join("i18n").join(folder).join("desktop.ftl"), text).unwrap();
    }
    let script = root().join("packaging/linux/localize-desktop.sh");
    let run = |file: &Path, out: &Path, i18n: Option<&Path>| {
        let mut command = std::process::Command::new("sh");
        command.arg(&script).arg(file).arg(out);
        if let Some(i18n) = i18n {
            command.arg(i18n);
        }
        let status = command.status().expect("sh runs");
        assert!(status.success(), "{}", file.display());
        std::fs::read_to_string(out).unwrap()
    };

    let mail = root().join("packaging/desktop/in.invenia.katna.Mail.desktop");
    let source = std::fs::read_to_string(&mail).unwrap();
    let out = run(&mail, &tmp.join("mail.desktop"), Some(&tmp.join("i18n")));
    let added: Vec<&str> = out
        .lines()
        .filter(|l| !source.lines().any(|s| s == *l))
        .collect();
    assert_eq!(
        added,
        [
            "GenericName[de]=E-Mail für alle Ihre Konten",
            "Keywords[de]=E-Mail;Post;",
            "Name[de]=Posteingang öffnen",
            "Name[pt_BR]=Abrir a caixa de entrada",
            "Name[zh_CN]=打开收件箱",
        ],
        "{out}"
    );
    // Without the added lines, the file is as it was.
    let kept: Vec<&str> = out.lines().filter(|l| !added.contains(l)).collect();
    assert_eq!(kept, source.lines().collect::<Vec<_>>());
    // Each translation goes right under its English line.
    assert!(out.contains("[Desktop Action inbox]\nName=Open Inbox\nName[de]="));

    // With the real translations every file still has its English lines.
    for file in desktop_files() {
        let source = std::fs::read_to_string(&file).unwrap();
        let out = run(&file, &tmp.join("real.desktop"), None);
        let kept: Vec<&str> = out
            .lines()
            .filter(|l| {
                !l.split_once('=')
                    .is_some_and(|(k, _)| k.contains('[') && !k.starts_with('['))
            })
            .collect();
        assert_eq!(
            kept,
            source.lines().collect::<Vec<_>>(),
            "{}",
            file.display()
        );
    }
}
