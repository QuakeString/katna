// SPDX-License-Identifier: GPL-3.0-or-later

//! How much of each language is translated, as Markdown: per binary the
//! messages it has of English's, the ids it lacks (shown in English), the
//! ids English no longer has (never shown) and the messages whose
//! variables differ from English's (shown in English), and the What's new
//! highlights it has not translated yet.
//!
//! A report only: it never fails. With `KATNA_I18N_COVERAGE` set to a file
//! the report is appended to it (CI passes `$GITHUB_STEP_SUMMARY`), else it
//! is printed:
//!
//! ```sh
//! cargo test -p katna-i18n --test translation_coverage -- --nocapture
//! KATNA_I18N_COVERAGE=coverage.md cargo test -p katna-i18n --test translation_coverage
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use fluent_syntax::ast;

/// At most this many ids are listed per kind and language.
const LIST_LIMIT: usize = 60;

/// One language folder's (or English's) messages: binary → id → variables.
type Messages = BTreeMap<String, BTreeMap<String, Vec<String>>>;

#[test]
fn translation_coverage() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let report = report(&root);
    match std::env::var_os("KATNA_I18N_COVERAGE") {
        Some(file) => {
            use std::io::Write as _;
            let written = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&file)
                .and_then(|mut f| f.write_all(report.as_bytes()));
            if let Err(error) = written {
                eprintln!("could not write {}: {error}", PathBuf::from(file).display());
            }
        }
        None => println!("{report}"),
    }
}

fn report(root: &Path) -> String {
    let i18n = root.join("i18n");
    let mut parse_errors = Vec::new();
    let english = messages(&i18n.join("en"), &mut parse_errors);
    let binaries: Vec<&String> = english.keys().filter(|b| !english[*b].is_empty()).collect();
    let highlights = highlights(&root.join("apps/katna-mail/whats-new/highlights"));

    let mut out = String::from("## Translation coverage\n\n");
    let counts: Vec<String> = binaries
        .iter()
        .map(|b| format!("{b} {}", english[*b].len()))
        .collect();
    let _ = writeln!(
        out,
        "English has {} messages ({}) and {} What's new highlights. A missing \
         message, or one whose variables differ from English, shows in English; \
         a stale one (an id English no longer has) is never shown. \
         This report never fails the build; see `i18n/README.md`.\n",
        english.values().map(BTreeMap::len).sum::<usize>(),
        counts.join(", "),
        highlights.len(),
    );

    let mut header = String::from("| Language | Folder | Status |");
    let mut rule = String::from("|---|---|---|");
    for binary in &binaries {
        let _ = write!(header, " {binary} |");
        rule.push_str("---:|");
    }
    header.push_str(" What's new | Missing | Stale | Other variables |");
    rule.push_str("---:|---:|---:|---:|");
    let _ = writeln!(out, "{header}\n{rule}");

    let mut details = String::new();
    let mut seen = BTreeSet::new();
    for language in katna_i18n::picker().filter(|l| l.translation != "en") {
        let folder = &language.translation;
        if !seen.insert(folder.clone()) {
            continue;
        }
        let dir = i18n.join(folder);
        let translated = messages(&dir, &mut parse_errors);
        let done_highlights = whats_new(&dir.join("katna-mail/whats-new.toml"));

        let mut missing = Vec::new();
        let mut stale = Vec::new();
        let mut other_vars = Vec::new();
        let mut row = format!(
            "| {} | `{folder}` | {} |",
            label(&language.name, &language.english),
            format!("{:?}", language.status).to_lowercase()
        );
        for binary in &binaries {
            let theirs = translated.get(*binary);
            let ours = &english[*binary];
            let have = ours
                .keys()
                .filter(|id| theirs.is_some_and(|t| t.contains_key(*id)))
                .count();
            let _ = write!(row, " {} |", fraction(have, ours.len()));
            for (id, vars) in ours {
                match theirs.and_then(|t| t.get(id)) {
                    None => missing.push(format!("{binary}: {id}")),
                    Some(their_vars) if their_vars != vars => {
                        other_vars.push(format!("{binary}: {id}"))
                    }
                    Some(_) => {}
                }
            }
        }
        for (binary, ids) in &translated {
            let ours = english.get(binary);
            for id in ids.keys() {
                if !ours.is_some_and(|o| o.contains_key(id)) {
                    stale.push(format!("{binary}: {id}"));
                }
            }
        }
        let untranslated: Vec<String> = highlights
            .iter()
            .filter(|h| !done_highlights.contains(*h))
            .cloned()
            .collect();
        let _ = writeln!(
            row,
            " {} | {} | {} | {} |",
            fraction(highlights.len() - untranslated.len(), highlights.len()),
            missing.len(),
            stale.len(),
            other_vars.len()
        );
        out.push_str(&row);

        if missing.is_empty()
            && stale.is_empty()
            && other_vars.is_empty()
            && untranslated.is_empty()
        {
            continue;
        }
        let _ = writeln!(
            details,
            "<details><summary><code>{folder}</code> {}: {} missing, {} stale, {} with other \
             variables, {} What's new untranslated</summary>\n",
            language.english,
            missing.len(),
            stale.len(),
            other_vars.len(),
            untranslated.len()
        );
        list(&mut details, "Missing", &missing);
        list(&mut details, "Stale", &stale);
        list(&mut details, "Other variables than English", &other_vars);
        list(&mut details, "What's new, untranslated", &untranslated);
        details.push_str("</details>\n\n");
    }
    out.push('\n');
    out.push_str(&details);

    let unlisted: Vec<String> = std::fs::read_dir(&i18n)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|e| e.path().is_dir())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .filter(|name| name != "en" && !seen.contains(name))
                .collect()
        })
        .unwrap_or_default();
    if !unlisted.is_empty() {
        let _ = writeln!(
            out,
            "Folders not in `i18n/languages.toml` (never loaded): {}\n",
            unlisted.join(", ")
        );
    }
    if !parse_errors.is_empty() {
        let _ = writeln!(out, "Files with Fluent syntax errors:\n");
        for error in &parse_errors {
            let _ = writeln!(out, "- {error}");
        }
        out.push('\n');
    }
    out
}

fn label(name: &str, english: &str) -> String {
    if name == english {
        name.to_owned()
    } else {
        format!("{name} ({english})")
    }
}

/// `1234/1234`, with the percentage when it is not all.
fn fraction(have: usize, total: usize) -> String {
    if have == total {
        format!("{have}/{total}")
    } else {
        // Rounded down, so 99.96 % never reads as 100 %.
        let tenths = have * 1000 / total.max(1);
        format!("{have}/{total} ({}.{} %)", tenths / 10, tenths % 10)
    }
}

fn list(out: &mut String, title: &str, ids: &[String]) {
    if ids.is_empty() {
        return;
    }
    let _ = writeln!(out, "{title}:\n");
    for id in ids.iter().take(LIST_LIMIT) {
        let _ = writeln!(out, "- `{id}`");
    }
    if ids.len() > LIST_LIMIT {
        let _ = writeln!(out, "- … and {} more", ids.len() - LIST_LIMIT);
    }
    out.push('\n');
}

/// The messages of one language folder by binary: `<binary>.ftl`, or any
/// `.ftl` file in `<binary>/`.
fn messages(dir: &Path, errors: &mut Vec<String>) -> Messages {
    let mut out = Messages::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    let is_ftl = |p: &Path| p.extension().is_some_and(|e| e == "ftl");
    for path in entries.filter_map(Result::ok).map(|e| e.path()) {
        let name = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let files: Vec<PathBuf> = if path.is_dir() {
            std::fs::read_dir(&path)
                .map(|e| {
                    e.filter_map(Result::ok)
                        .map(|e| e.path())
                        .filter(|p| is_ftl(p))
                        .collect()
                })
                .unwrap_or_default()
        } else if is_ftl(&path) {
            vec![path.clone()]
        } else {
            continue;
        };
        let binary = out.entry(name).or_default();
        for file in files {
            let text = std::fs::read_to_string(&file).unwrap_or_default();
            let resource = match fluent_syntax::parser::parse(text.as_str()) {
                Ok(resource) => resource,
                Err((resource, e)) => {
                    errors.push(format!("`{}`: {} errors", file.display(), e.len()));
                    resource
                }
            };
            for entry in &resource.body {
                if let ast::Entry::Message(m) = entry {
                    let mut vars = Vec::new();
                    if let Some(value) = &m.value {
                        pattern_variables(value, &mut vars);
                    }
                    vars.sort();
                    vars.dedup();
                    binary.insert(m.id.name.to_owned(), vars);
                }
            }
        }
    }
    out
}

fn pattern_variables(pattern: &ast::Pattern<&str>, out: &mut Vec<String>) {
    for element in &pattern.elements {
        if let ast::PatternElement::Placeable { expression } = element {
            expression_variables(expression, out);
        }
    }
}

fn expression_variables(expression: &ast::Expression<&str>, out: &mut Vec<String>) {
    match expression {
        ast::Expression::Select { selector, variants } => {
            inline_variables(selector, out);
            for variant in variants {
                pattern_variables(&variant.value, out);
            }
        }
        ast::Expression::Inline(inline) => inline_variables(inline, out),
    }
}

fn inline_variables(inline: &ast::InlineExpression<&str>, out: &mut Vec<String>) {
    match inline {
        ast::InlineExpression::VariableReference { id } => out.push(id.name.to_owned()),
        ast::InlineExpression::Placeable { expression } => expression_variables(expression, out),
        _ => {}
    }
}

/// The What's new highlights, by file name without `.toml`.
fn highlights(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|e| e == "toml"))
                .filter_map(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

/// The highlights a language's `whats-new.toml` translates.
fn whats_new(file: &Path) -> BTreeSet<String> {
    std::fs::read_to_string(file)
        .ok()
        .and_then(|text| text.parse::<toml::Table>().ok())
        .map(|table| table.keys().cloned().collect())
        .unwrap_or_default()
}
