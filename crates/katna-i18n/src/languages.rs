// SPDX-License-Identifier: GPL-3.0-or-later

//! The languages Katna offers, from `i18n/languages.toml`.

use std::sync::OnceLock;

use serde::Deserialize;

/// How far a language's text can be trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// English, which the others are translated from.
    Source,
    /// Drafted by AI; a native speaker has not reviewed it yet.
    Machine,
    /// Reviewed by a native speaker.
    Reviewed,
}

/// One entry of the language picker.
#[derive(Debug, Clone, Deserialize)]
pub struct Language {
    /// The setting's value, BCP 47: `bn`, `en-IN`, `zh-Hant`.
    pub tag: String,
    /// The language's own name: `বাংলা`.
    pub name: String,
    /// Its English name: `Bengali`.
    pub english: String,
    /// The flag shown beside it (ISO 3166 code, lower case).
    pub flag: String,
    /// The folder under `i18n/` with its text; the English entries share
    /// `en`.
    pub translation: String,
    /// The locale its dates and numbers are formatted in.
    pub formats: String,
    /// It reads right to left, so the layout mirrors.
    #[serde(default)]
    pub rtl: bool,
    pub status: Status,
    /// Native speakers who reviewed it.
    #[serde(default)]
    pub reviewers: Vec<String>,
    /// Not in the picker (the pseudo-languages for testing).
    #[serde(skip)]
    pub hidden: bool,
}

#[derive(Deserialize)]
struct File {
    language: Vec<Language>,
}

const LANGUAGES: &str = include_str!("../../../i18n/languages.toml");

/// Every language, the picker's in its order, then the hidden ones.
pub fn all() -> &'static [Language] {
    static ALL: OnceLock<Vec<Language>> = OnceLock::new();
    ALL.get_or_init(|| {
        let mut all = toml::from_str::<File>(LANGUAGES)
            .expect("i18n/languages.toml is valid (checked by a test)")
            .language;
        all.extend(crate::pseudo::languages());
        all
    })
}

/// The picker's entries, in order.
pub fn picker() -> impl Iterator<Item = &'static Language> {
    all().iter().filter(|l| !l.hidden)
}

/// The language with this tag (ignoring case, `_` for `-`).
pub fn find(tag: &str) -> Option<&'static Language> {
    let tag = tag.trim().replace('_', "-");
    all().iter().find(|l| l.tag.eq_ignore_ascii_case(&tag))
}

impl Language {
    /// Whether the picker's search `query` finds this language: its own
    /// name, its English name or its tag, ignoring case and accents
    /// (`tieng` finds Tiếng Việt).
    pub fn matches(&self, query: &str) -> bool {
        let query = fold(query.trim());
        query.is_empty()
            || [&self.name, &self.english, &self.tag]
                .iter()
                .any(|text| fold(text).contains(&query))
    }
}

/// Lower case without the accents of Latin letters.
pub fn fold(text: &str) -> String {
    const ACCENTED: &[(char, &str)] = &[
        ('a', "àáâãäåāăąạảấầẩẫậắằẳẵặ"),
        ('c', "çćč"),
        ('d', "ďđ"),
        ('e', "èéêëēęěẹẻẽếềểễệ"),
        ('g', "ğ"),
        ('i', "ìíîïīịỉĩı"),
        ('l', "ł"),
        ('n', "ñńň"),
        ('o', "òóôõöøōőọỏốồổỗộớờởỡợ"),
        ('s', "śšşṣ"),
        ('t', "ţť"),
        ('u', "ùúûüūůűụủũứừửữự"),
        ('y', "ýÿỳỵỷỹ"),
        ('z', "źżž"),
    ];
    text.to_lowercase()
        .chars()
        .map(|c| {
            ACCENTED
                .iter()
                .find(|(_, variants)| variants.contains(c))
                .map_or(c, |(base, _)| *base)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_picker_has_51_entries_and_49_translations() {
        assert_eq!(picker().count(), 51);
        let mut translations: Vec<&str> = picker().map(|l| l.translation.as_str()).collect();
        translations.sort_unstable();
        translations.dedup();
        assert_eq!(translations.len(), 49);
    }

    #[test]
    fn every_translation_has_its_files() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../i18n");
        for language in picker() {
            for binary in ["katna-mail", "katna-daemon", "katna-setup"] {
                let folder = root.join(&language.translation).join(binary);
                let files = std::fs::read_dir(&folder).map_or(0, |e| e.count());
                assert!(files > 0, "{}", folder.display());
            }
        }
    }

    #[test]
    fn four_languages_read_right_to_left() {
        let mut rtl: Vec<&str> = picker().filter(|l| l.rtl).map(|l| l.tag.as_str()).collect();
        rtl.sort_unstable();
        assert_eq!(rtl, ["ar", "fa", "he", "ur"]);
    }

    #[test]
    fn tags_are_unique_and_formats_parse() {
        let mut tags: Vec<&str> = all().iter().map(|l| l.tag.as_str()).collect();
        tags.sort_unstable();
        let before = tags.len();
        tags.dedup();
        assert_eq!(tags.len(), before);
        for language in all() {
            assert!(
                icu_locale_core::Locale::try_from_str(&language.formats).is_ok(),
                "{}",
                language.formats
            );
        }
    }

    #[test]
    fn search_ignores_case_and_accents() {
        let vi = find("vi").unwrap();
        assert!(vi.matches("tieng"));
        assert!(vi.matches("VIET"));
        assert!(find("bn").unwrap().matches("বাংলা"));
        assert!(find("bn").unwrap().matches("beng"));
        assert!(find("fr").unwrap().matches("francais"));
        assert!(find("yo").unwrap().matches("yoruba"));
        assert!(!find("de").unwrap().matches("french"));
        assert!(find("de").unwrap().matches(" "));
    }

    /// The "Translation correction" issue form lists every language of the
    /// picker, in its order. `KATNA_I18N_WRITE_TEMPLATE=1` writes the list.
    #[test]
    fn issue_template_lists_every_language() {
        const BEGIN: &str = "        # Begin languages";
        const END: &str = "        # End languages.";
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../.github/ISSUE_TEMPLATE/translation-correction.yml");
        let text = std::fs::read_to_string(&path).unwrap();
        let start = text.find(BEGIN).expect("begin marker");
        let start = start + text[start..].find("\n        - ").expect("options") + 1;
        let end = text.find(END).expect("end marker");
        let options: String = picker()
            .map(|l| {
                let label = if l.name == l.english {
                    l.name.clone()
                } else {
                    format!("{} ({})", l.name, l.english)
                };
                format!("        - \"{label}\"\n")
            })
            .collect();
        if text[start..end] == options {
            return;
        }
        if std::env::var_os("KATNA_I18N_WRITE_TEMPLATE").is_some() {
            let updated = format!("{}{options}{}", &text[..start], &text[end..]);
            std::fs::write(&path, updated).unwrap();
            return;
        }
        panic!(
            "{} is behind i18n/languages.toml; run \
             `KATNA_I18N_WRITE_TEMPLATE=1 cargo test -p katna-i18n issue_template`",
            path.display()
        );
    }

    #[test]
    fn finds_by_tag() {
        assert_eq!(find("en_in").unwrap().english, "English (India)");
        assert_eq!(find("zh-hant").unwrap().flag, "tw");
        assert!(find("xx").is_none());
    }
}
