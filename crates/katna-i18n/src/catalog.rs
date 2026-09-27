// SPDX-License-Identifier: GPL-3.0-or-later

//! The current language's messages, with English under them.

use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock, RwLock};

use fluent_bundle::concurrent::FluentBundle;
use fluent_bundle::{FluentArgs, FluentResource};
use unic_langid::LanguageIdentifier;

use crate::pseudo;
use crate::system::{Resolved, resolve};

/// A binary's translations: (folder under `i18n/`, contents of one
/// `.ftl` file), as its build script embeds them. English is built into
/// this crate and need not be listed.
pub type Sources = &'static [(&'static str, &'static str)];

/// Variables for [`lookup`].
pub type Args<'a> = FluentArgs<'a>;

type Bundle = FluentBundle<FluentResource>;

/// The English files, embedded by the build script.
const ENGLISH: &[(&str, &str)] = include!(concat!(env!("OUT_DIR"), "/english.rs"));

pub(crate) struct Catalog {
    resolved: Resolved,
    /// `None` for English, which is `english` itself.
    bundle: Option<Bundle>,
    english: Arc<Bundle>,
    #[cfg(feature = "format")]
    pub(crate) formats: crate::format::Formats,
}

struct Setup {
    sources: Sources,
    overrides: Option<PathBuf>,
}

static SETUP: OnceLock<Setup> = OnceLock::new();
static CURRENT: RwLock<Option<Arc<Catalog>>> = RwLock::new(None);

/// Gives the binary's translations and the folder whose files override
/// them (`$XDG_DATA_HOME/katna/i18n`: `<folder>/<file>.ftl`, loaded over
/// the built-in text message by message). Call once, before [`apply`].
pub fn init(sources: Sources, overrides: Option<PathBuf>) {
    let _ = SETUP.set(Setup { sources, overrides });
}

/// Switches to the language for the setting `choice` (a tag, or empty
/// for the desktop's language) and returns what it resolved to.
pub fn apply(choice: &str) -> Resolved {
    let resolved = resolve(choice);
    let catalog = Arc::new(build(resolved.clone()));
    if let Ok(mut current) = CURRENT.write() {
        *current = Some(catalog);
    }
    tracing::debug!(
        language = %resolved.language.tag,
        formats = %resolved.formats,
        "language"
    );
    resolved
}

/// The language in use.
pub fn current() -> Resolved {
    catalog().resolved.clone()
}

/// Whether the layout reads right to left.
pub fn rtl() -> bool {
    catalog().resolved.language.rtl
}

pub(crate) fn catalog() -> Arc<Catalog> {
    if let Ok(current) = CURRENT.read()
        && let Some(catalog) = current.as_ref()
    {
        return Arc::clone(catalog);
    }
    // Nothing applied yet (tests, early start): English.
    static FALLBACK: OnceLock<Arc<Catalog>> = OnceLock::new();
    Arc::clone(FALLBACK.get_or_init(|| {
        let english = crate::find("en-US").expect("English (US) exists");
        Arc::new(build(Resolved {
            language: english,
            formats: english.formats.clone(),
            system: false,
        }))
    }))
}

/// The text of message `id` with `args`, in the current language, else
/// English, else the id itself.
pub fn lookup(id: &str, args: Option<&FluentArgs<'_>>) -> String {
    let catalog = catalog();
    for bundle in [catalog.bundle.as_ref(), Some(&*catalog.english)]
        .into_iter()
        .flatten()
    {
        if let Some(pattern) = bundle.get_message(id).and_then(|m| m.value()) {
            let mut errors = Vec::new();
            let text = bundle.format_pattern(pattern, args, &mut errors);
            if !errors.is_empty() {
                tracing::debug!(id, ?errors, "message");
            }
            return text.into_owned();
        }
    }
    tracing::debug!(id, "no such message");
    id.to_owned()
}

fn english() -> Arc<Bundle> {
    static ENGLISH_BUNDLE: OnceLock<Arc<Bundle>> = OnceLock::new();
    Arc::clone(ENGLISH_BUNDLE.get_or_init(|| {
        let mut bundle = new_bundle("en-US", false);
        add(&mut bundle, "en", ENGLISH.iter().map(|(_, text)| *text));
        Arc::new(bundle)
    }))
}

fn build(resolved: Resolved) -> Catalog {
    let language = resolved.language;
    let pseudo = pseudo::is_pseudo(&language.tag);
    let bundle = (language.translation != "en" || pseudo).then(|| {
        let mut bundle = new_bundle(&language.tag, language.rtl);
        if pseudo {
            bundle.set_transform(Some(pseudo::transform));
        }
        let setup = SETUP.get();
        let embedded = if language.translation == "en" {
            ENGLISH.iter().map(|(_, text)| *text).collect::<Vec<_>>()
        } else {
            setup
                .map(|s| s.sources)
                .unwrap_or_default()
                .iter()
                .filter(|(folder, _)| *folder == language.translation)
                .map(|(_, text)| *text)
                .collect()
        };
        add(&mut bundle, &language.translation, embedded);
        bundle
    });
    // The override folder corrects any language, English included.
    let english = match overrides("en") {
        Some(files) if !files.is_empty() && bundle.is_none() => {
            let mut bundle = new_bundle("en-US", false);
            add(&mut bundle, "en", ENGLISH.iter().map(|(_, text)| *text));
            add_owned(&mut bundle, files);
            Arc::new(bundle)
        }
        _ => english(),
    };
    let bundle = bundle.map(|mut bundle| {
        add_owned(
            &mut bundle,
            overrides(&language.translation).unwrap_or_default(),
        );
        bundle
    });
    Catalog {
        #[cfg(feature = "format")]
        formats: crate::format::Formats::new(&resolved.formats),
        resolved,
        bundle,
        english,
    }
}

fn new_bundle(tag: &str, rtl: bool) -> Bundle {
    let id: LanguageIdentifier = tag
        .parse()
        .or_else(|_| tag.split('-').next().unwrap_or("en").parse())
        .unwrap_or_default();
    let mut bundle = Bundle::new_concurrent(vec![id]);
    // Variables are wrapped in isolation marks (FSI…PDI) only where the
    // text reads right to left, so a Latin name keeps an Arabic sentence's
    // direction; left-to-right text has no use for them.
    bundle.set_use_isolating(rtl);
    #[cfg(feature = "format")]
    bundle.set_formatter(Some(crate::format::fluent_number));
    bundle
}

fn add<'a>(bundle: &mut Bundle, folder: &str, texts: impl IntoIterator<Item = &'a str>) {
    for text in texts {
        match FluentResource::try_new(text.to_owned()) {
            Ok(resource) => bundle.add_resource_overriding(resource),
            Err((resource, errors)) => {
                tracing::warn!(folder, ?errors, "translation has errors");
                bundle.add_resource_overriding(resource);
            }
        }
    }
}

fn add_owned(bundle: &mut Bundle, texts: Vec<String>) {
    for text in texts {
        let resource = match FluentResource::try_new(text) {
            Ok(resource) => resource,
            Err((resource, errors)) => {
                tracing::warn!(?errors, "override has errors");
                resource
            }
        };
        bundle.add_resource_overriding(resource);
    }
}

/// The `.ftl` files in the override folder for `folder`.
fn overrides(folder: &str) -> Option<Vec<String>> {
    let dir = SETUP.get()?.overrides.as_deref()?.join(folder);
    Some(read_dir(&dir))
}

fn read_dir(dir: &Path) -> Vec<String> {
    read_dir_paths(dir)
        .iter()
        .filter_map(|p| std::fs::read_to_string(p).ok())
        .collect()
}

/// The `.ftl` files in `dir`, sorted.
fn read_dir_paths(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut paths: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "ftl"))
        .collect();
    paths.sort();
    paths
}

/// For [`crate::format`]: the current catalog's formatters.
#[cfg(feature = "format")]
pub(crate) fn with_formats<T>(f: impl FnOnce(&crate::format::Formats) -> T) -> T {
    f(&catalog().formats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_is_there_before_anything_is_applied() {
        assert_eq!(lookup("language-system-default", None), "System default");
        assert_eq!(lookup("no-such-message", None), "no-such-message");
    }

    /// Every `tr!("id"…)` in the code has an English message.
    #[test]
    fn every_id_in_the_code_is_in_english() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .unwrap();
        let mut missing = Vec::new();
        let mut stack = vec![root.join("apps"), root.join("crates"), root.join("tools")];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "rs")
                    && !path.starts_with(env!("CARGO_MANIFEST_DIR"))
                {
                    let text = std::fs::read_to_string(&path).unwrap_or_default();
                    for (at, _) in text.match_indices("tr!(") {
                        // Not `include_str!(` and the like.
                        let before = text[..at].chars().next_back();
                        if before.is_some_and(|c| c.is_alphanumeric() || c == '_') {
                            continue;
                        }
                        // rustfmt may put the id on the next line.
                        let Some(rest) = text[at + 4..].trim_start().strip_prefix('"') else {
                            continue;
                        };
                        let id = rest.split('"').next().unwrap_or_default();
                        if english().get_message(id).is_none() {
                            missing.push(format!("{}: {id}", path.display()));
                        }
                    }
                }
            }
        }
        assert!(missing.is_empty(), "no English message for {missing:#?}");
    }

    /// Each translation parses, has only messages English has, and uses
    /// the same variables.
    #[test]
    fn translations_match_english() {
        use fluent_syntax::ast;
        fn variables(pattern: &ast::Pattern<&str>, out: &mut Vec<String>) {
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
                        variables(&variant.value, out);
                    }
                }
                ast::Expression::Inline(inline) => inline_variables(inline, out),
            }
        }
        fn inline_variables(inline: &ast::InlineExpression<&str>, out: &mut Vec<String>) {
            match inline {
                ast::InlineExpression::VariableReference { id } => out.push(id.name.to_owned()),
                ast::InlineExpression::Placeable { expression } => {
                    expression_variables(expression, out)
                }
                _ => {}
            }
        }
        fn messages(text: &str) -> std::collections::BTreeMap<String, Vec<String>> {
            let resource = fluent_syntax::parser::parse(text).unwrap_or_else(|(_, e)| {
                panic!("{e:?}");
            });
            resource
                .body
                .iter()
                .filter_map(|entry| match entry {
                    ast::Entry::Message(m) => {
                        let mut vars = Vec::new();
                        if let Some(value) = &m.value {
                            variables(value, &mut vars);
                        }
                        vars.sort();
                        vars.dedup();
                        Some((m.id.name.to_owned(), vars))
                    }
                    _ => None,
                })
                .collect()
        }
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../i18n");
        let english: std::collections::BTreeMap<_, _> = ENGLISH
            .iter()
            .flat_map(|(name, text)| messages(text).into_iter().map(move |m| (name, m)))
            .map(|(name, (id, vars))| (id, (name.to_string(), vars)))
            .collect();
        let mut problems = Vec::new();
        for language in crate::picker().filter(|l| l.translation != "en") {
            let dir = root.join(&language.translation);
            for path in read_dir_paths(&dir) {
                let text = std::fs::read_to_string(&path).unwrap();
                let file = path.file_stem().unwrap().to_string_lossy().into_owned();
                for (id, vars) in messages(&text) {
                    match english.get(&id) {
                        None => {
                            problems.push(format!("{}: {id} is not in English", path.display()))
                        }
                        Some((name, _)) if *name != file => {
                            problems.push(format!("{}: {id} belongs in {name}.ftl", path.display()))
                        }
                        Some((_, english_vars)) if *english_vars != vars => problems.push(format!(
                            "{}: {id} uses {vars:?}, English {english_vars:?}",
                            path.display()
                        )),
                        _ => {}
                    }
                }
            }
        }
        assert!(problems.is_empty(), "{problems:#?}");
    }

    #[test]
    fn english_files_parse_cleanly() {
        for (name, text) in ENGLISH {
            let parsed = fluent_syntax::parser::parse(*text);
            assert!(parsed.is_ok(), "{name}: {:?}", parsed.err().map(|e| e.1));
        }
    }
}
