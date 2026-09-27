// SPDX-License-Identifier: GPL-3.0-or-later

//! Which language to show: the user's choice, else the desktop's.
//!
//! The desktop's language is the first in `LANGUAGE` (a list, `bn:en_US`)
//! that Katna has, else `LC_ALL`, `LC_MESSAGES`, `LANG`; on Plasma also
//! `plasma-localerc`, because the daemon, started by systemd, may not have
//! the session's variables.

use std::path::PathBuf;

use crate::OVERRIDE_VAR;
use crate::languages::{Language, find};

/// The language and formats in use.
#[derive(Debug, Clone)]
pub struct Resolved {
    pub language: &'static Language,
    /// The locale for dates and numbers (BCP 47).
    pub formats: String,
    /// Follows the desktop ("System default").
    pub system: bool,
}

/// The language for the setting `choice` (a tag; empty for System
/// default). [`OVERRIDE_VAR`] wins over both.
pub fn resolve(choice: &str) -> Resolved {
    let forced = std::env::var(OVERRIDE_VAR).ok().and_then(|tag| find(&tag));
    let chosen = forced.or_else(|| find(choice).filter(|l| !l.hidden));
    match chosen {
        Some(language) => Resolved {
            language,
            formats: language.formats.clone(),
            system: false,
        },
        None => {
            let language = system_language();
            Resolved {
                language,
                formats: system_formats().unwrap_or_else(|| language.formats.clone()),
                system: true,
            }
        }
    }
}

/// The desktop's language, or English (US) when Katna does not have it.
pub fn system_language() -> &'static Language {
    let env = |name: &str| std::env::var(name).ok().filter(|v| !v.is_empty());
    let mut wanted: Vec<String> = Vec::new();
    // `LANGUAGE` is a list, used only when a locale is set (as gettext does).
    let locale = ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .find_map(|name| env(name))
        .filter(|v| !is_c(v));
    if locale.is_some()
        && let Some(list) = env("LANGUAGE")
    {
        wanted.extend(list.split(':').map(str::to_owned));
    }
    wanted.extend(locale);
    if let Some(plasma) = plasma("Translations", "LANGUAGE") {
        wanted.extend(plasma.split(':').map(str::to_owned));
    }
    wanted
        .iter()
        .find_map(|name| from_posix(name))
        .or_else(|| find("en-US"))
        .expect("English (US) is in languages.toml")
}

/// The desktop's locale for dates and numbers, as BCP 47 (`en-IN`), when
/// one is set.
pub fn system_formats() -> Option<String> {
    let env = |name: &str| std::env::var(name).ok().filter(|v| !v.is_empty());
    ["LC_ALL", "LC_TIME", "LANG"]
        .iter()
        .find_map(|name| env(name))
        .or_else(|| plasma("Formats", "LC_TIME"))
        .or_else(|| plasma("Formats", "LANG"))
        .filter(|v| !is_c(v))
        .and_then(|posix| bcp47(&posix))
}

fn is_c(locale: &str) -> bool {
    matches!(locale.split(['.', '@']).next(), Some("C" | "POSIX" | ""))
}

/// A POSIX locale (`bn_IN.UTF-8`, `sr_RS@latin`) as a BCP 47 tag that
/// parses (`bn-IN`).
fn bcp47(posix: &str) -> Option<String> {
    let base = posix.split(['.', '@']).next()?.replace('_', "-");
    icu_locale_core::Locale::try_from_str(&base)
        .ok()
        .map(|l| l.to_string())
}

/// The language Katna offers for a POSIX locale or a language name from
/// `LANGUAGE`.
pub(crate) fn from_posix(name: &str) -> Option<&'static Language> {
    let base = name.split(['.', '@']).next()?.replace('_', "-");
    if base.is_empty() || is_c(&base) {
        return None;
    }
    let mut parts = base.split('-');
    let lang = parts.next()?.to_ascii_lowercase();
    let region = parts.next().map(str::to_ascii_uppercase);
    let tag = match (lang.as_str(), region.as_deref()) {
        ("zh", Some("TW" | "HK" | "MO" | "HANT")) => "zh-Hant".to_owned(),
        ("zh", _) => "zh-Hans".to_owned(),
        ("pt", _) => "pt-BR".to_owned(),
        ("tl" | "fil", _) => "fil".to_owned(),
        ("iw", _) => "he".to_owned(),
        ("in", _) => "id".to_owned(),
        ("en", Some("IN")) => "en-IN".to_owned(),
        // Commonwealth English writes dates the British way.
        ("en", Some("GB" | "IE" | "AU" | "NZ" | "ZA")) => "en-GB".to_owned(),
        ("en", _) => "en-US".to_owned(),
        _ => lang,
    };
    find(&tag).filter(|l| !l.hidden)
}

/// A key from Plasma's `~/.config/plasma-localerc`, when Plasma is the
/// desktop.
fn plasma(group: &str, key: &str) -> Option<String> {
    let config = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    let text = std::fs::read_to_string(config.join("plasma-localerc")).ok()?;
    ini_value(&text, group, key)
}

fn ini_value(text: &str, group: &str, key: &str) -> Option<String> {
    let mut in_group = false;
    for line in text.lines().map(str::trim) {
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            in_group = name == group;
        } else if in_group
            && let Some((k, v)) = line.split_once('=')
            && k.trim() == key
        {
            return Some(v.trim().to_owned()).filter(|v| !v.is_empty());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tag(name: &str) -> Option<&'static str> {
        from_posix(name).map(|l| l.tag.as_str())
    }

    #[test]
    fn maps_posix_locales() {
        assert_eq!(tag("bn_IN.UTF-8"), Some("bn"));
        assert_eq!(tag("bn_BD"), Some("bn"));
        assert_eq!(tag("en_IN.UTF-8"), Some("en-IN"));
        assert_eq!(tag("en_GB"), Some("en-GB"));
        assert_eq!(tag("en_AU.UTF-8"), Some("en-GB"));
        assert_eq!(tag("en_US.UTF-8"), Some("en-US"));
        assert_eq!(tag("en"), Some("en-US"));
        assert_eq!(tag("zh_TW.UTF-8"), Some("zh-Hant"));
        assert_eq!(tag("zh_HK"), Some("zh-Hant"));
        assert_eq!(tag("zh_CN.UTF-8"), Some("zh-Hans"));
        assert_eq!(tag("pt_PT"), Some("pt-BR"));
        assert_eq!(tag("tl_PH"), Some("fil"));
        assert_eq!(tag("fil_PH"), Some("fil"));
        assert_eq!(tag("iw_IL"), Some("he"));
        assert_eq!(tag("ur_PK.UTF-8"), Some("ur"));
        assert_eq!(tag("pa_IN"), Some("pa"));
        assert_eq!(tag("dz_BT"), Some("dz"));
        assert_eq!(tag("sr_RS@latin"), None);
        assert_eq!(tag("C.UTF-8"), None);
        assert_eq!(tag("qps-ploc"), None);
    }

    #[test]
    fn formats_as_bcp47() {
        assert_eq!(bcp47("en_IN.UTF-8").as_deref(), Some("en-IN"));
        assert_eq!(bcp47("de_DE@euro").as_deref(), Some("de-DE"));
    }

    #[test]
    fn reads_plasma_localerc() {
        let text = "[Formats]\nLANG=en_IN.UTF-8\n\n[Translations]\nLANGUAGE=bn:en_US\n";
        assert_eq!(
            ini_value(text, "Translations", "LANGUAGE").as_deref(),
            Some("bn:en_US")
        );
        assert_eq!(
            ini_value(text, "Formats", "LANG").as_deref(),
            Some("en_IN.UTF-8")
        );
        assert_eq!(ini_value(text, "Formats", "LC_TIME"), None);
    }
}
