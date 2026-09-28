// SPDX-License-Identifier: GPL-3.0-or-later

//! What's new: the highlights shown once after an update
//! (`docs/ARCHITECTURE.md` §13.6). They are curated by hand, one TOML file
//! each in `apps/katna-mail/whats-new/highlights/`, which `build.rs` builds
//! into [`HIGHLIGHTS`]; a change people will notice adds a file. A major
//! feature may bring a short animation (animated WebPs in
//! `apps/katna-mail/whats-new/`, one per theme). They are written in
//! English; `i18n/<language>/katna-mail/whats-new.toml` translates them.

use std::collections::HashSet;

/// The version of this build, as the package names it (`0.0.0.r90.gabc1234`
/// for a build of `main`). The Arch package sets `KATNA_VERSION`; other
/// builds show the crate's version.
pub const VERSION: &str = match option_env!("KATNA_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

/// At most this many highlights at once; the full changelog has the rest.
pub const SHOWN: usize = 6;

/// One change worth telling people about.
#[derive(Debug)]
pub struct Highlight {
    /// The file's name without `.toml`: when it was written and a slug.
    /// Highlights are in this order, and the config remembers the names
    /// shown.
    pub name: &'static str,
    /// In English, as the file has them; [`Highlight::title`] and
    /// [`Highlight::text`] give the current language's.
    title: &'static str,
    text: &'static str,
    /// Its translations, in folder order.
    translations: &'static [Translation],
    /// A short animation of the feature, for major ones only.
    pub animation: Option<Animation>,
}

/// A highlight in another language, from
/// `i18n/<folder>/katna-mail/whats-new.toml`.
#[derive(Debug)]
pub struct Translation {
    /// The language's folder under `i18n/` (`ja`, `pt-BR`).
    folder: &'static str,
    title: &'static str,
    text: &'static str,
}

impl Highlight {
    /// The title in the current language, else English.
    pub fn title(&self) -> &'static str {
        self.in_language(&katna_i18n::current().language.translation)
            .0
    }

    /// The text in the current language, else English.
    pub fn text(&self) -> &'static str {
        self.in_language(&katna_i18n::current().language.translation)
            .1
    }

    /// The title and text in the language whose translation is in
    /// `i18n/<folder>/`, else English.
    fn in_language(&self, folder: &str) -> (&'static str, &'static str) {
        self.translations
            .iter()
            .find(|t| t.folder == folder)
            .map_or((self.title, self.text), |t| (t.title, t.text))
    }
}

/// Animated WebPs of a feature in the light and the dark theme, recorded
/// at the size they are drawn.
#[derive(Debug)]
pub struct Animation {
    pub light: &'static [u8],
    pub dark: &'static [u8],
}

impl Animation {
    pub fn for_theme(&self, dark: bool) -> &'static [u8] {
        if dark { self.dark } else { self.light }
    }
}

include!(concat!(env!("OUT_DIR"), "/highlights.rs"));

/// The highlights as they were numbered before they moved to files: a
/// settings file of that time keeps the number of the newest one shown
/// (`onboarding.whats_new_seen`). Never grows.
const NUMBERED: [&str; 26] = [
    "2026-09-27-0319-whats-new",
    "2026-09-27-0320-reply-buttons-fit",
    "2026-09-27-0321-notifications-open-mail",
    "2026-09-27-0322-attachment-previews",
    "2026-09-27-0323-frosted-menus",
    "2026-09-27-0346-settings-search",
    "2026-09-27-0410-crash-reports-kept",
    "2026-09-27-0438-calmer-reply",
    "2026-09-27-0439-mail-in-own-window",
    "2026-09-27-0440-settings-on-phone",
    "2026-09-27-0441-offline-mail",
    "2026-09-27-0442-folder-pane-button",
    "2026-09-27-0443-cards-and-phone-rows",
    "2026-09-27-0444-about-katna",
    "2026-09-27-0457-settings-tabs-one-line",
    "2026-09-27-0515-send-crash-reports",
    "2026-09-27-0536-older-attachments-open",
    "2026-09-27-0556-reading-options",
    "2026-09-27-0609-send-and-archive",
    "2026-09-27-0627-scaling",
    "2026-09-27-0645-shortcut-sets",
    "2026-09-27-0646-more-settings",
    "2026-09-27-0706-your-language",
    "2026-09-27-0723-select-and-copy-text",
    "2026-09-27-0743-address-suggestions",
    "2026-09-27-0804-main-window-translated",
];

/// What the settings file says was shown: the names of the highlights
/// (`onboarding.whats_new_shown`) and, from before they had names, the
/// number of the newest one (`onboarding.whats_new_seen`).
pub struct Seen<'a> {
    names: HashSet<&'a str>,
    numbered: usize,
}

impl<'a> Seen<'a> {
    pub fn new(names: &'a [String], numbered: Option<u32>) -> Self {
        Self {
            names: names.iter().map(String::as_str).collect(),
            numbered: numbered.map_or(0, |n| n as usize),
        }
    }

    /// Anything was shown before: this is not the first start.
    pub fn any(&self) -> bool {
        !self.names.is_empty() || self.numbered > 0
    }

    fn contains(&self, name: &str) -> bool {
        self.names.contains(name) || NUMBERED[..self.numbered.min(NUMBERED.len())].contains(&name)
    }
}

/// The name of every highlight in this build, for the settings file once
/// they have been shown.
pub fn names() -> Vec<String> {
    HIGHLIGHTS.iter().map(|h| h.name.to_owned()).collect()
}

/// Highlights not shown yet, newest first but major ones (with an
/// animation) before the rest, and how many more there are beyond
/// [`SHOWN`]. One merged after a newer one was shown still counts.
pub fn unseen(seen: &Seen) -> (Vec<&'static Highlight>, usize) {
    newest(HIGHLIGHTS.iter().filter(|h| !seen.contains(h.name)))
}

/// The newest highlights, for opening What's new by hand. An older
/// major one is not brought forward: it would show the same animation
/// long after its update.
pub fn recent() -> (Vec<&'static Highlight>, usize) {
    let start = HIGHLIGHTS.len().saturating_sub(SHOWN);
    let (shown, _) = newest(HIGHLIGHTS[start..].iter());
    (shown, start)
}

/// At most [`SHOWN`] highlights: major ones (with an animation) first,
/// even when older, then the newest.
fn newest<'a>(
    highlights: impl DoubleEndedIterator<Item = &'a Highlight>,
) -> (Vec<&'a Highlight>, usize) {
    let mut all: Vec<_> = highlights.rev().collect();
    all.sort_by_key(|h| h.animation.is_none());
    let more = all.len().saturating_sub(SHOWN);
    all.truncate(SHOWN);
    (all, more)
}

/// The commit a package version was built from: `abc1234` in
/// `0.0.0.r90.gabc1234`.
fn commit(version: &str) -> Option<&str> {
    let (_, hash) = version.rsplit_once(".g")?;
    (hash.len() >= 7 && hash.bytes().all(|b| b.is_ascii_hexdigit())).then_some(hash)
}

/// Where to read every change: the commits since `from`, the version that
/// ran before, when both versions name their commit.
pub fn changelog_url(from: Option<&str>) -> String {
    changelog_url_for(from, VERSION)
}

fn changelog_url_for(from: Option<&str>, to: &str) -> String {
    let repo = env!("CARGO_PKG_REPOSITORY");
    match (from.and_then(commit), commit(to)) {
        (Some(old), Some(new)) if old != new => format!("{repo}/compare/{old}...{new}"),
        (_, Some(new)) => format!("{repo}/commits/{new}"),
        _ => format!("{repo}/commits/main"),
    }
}

/// What the window shows when it starts.
#[derive(Debug, PartialEq, Eq)]
pub enum Start {
    /// The first start: onboarding or the tour. Nothing older is news.
    FirstRun,
    /// Started before: What's new, if there are highlights not shown yet.
    Returning,
}

/// Tells a first start from a start after an update. `done` is whether the
/// first-start help was seen; `seen` is whether any highlight was shown;
/// `config_existed` is whether a settings file was there before this start
/// (only Katna Mail writes one). Versions before What's new set neither
/// `seen` nor, if their tour was left open, `done`, but they did save
/// settings.
pub fn start(done: bool, seen: bool, config_existed: bool) -> Start {
    if done || seen || config_existed {
        Start::Returning
    } else {
        Start::FirstRun
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The largest animation allowed, to keep the app within its size budget.
    const MAX_ANIMATION_BYTES: usize = 600 * 1024;

    #[test]
    fn highlights_are_in_name_order() {
        assert!(HIGHLIGHTS.windows(2).all(|w| w[0].name < w[1].name));
        for highlight in HIGHLIGHTS {
            assert!(!highlight.title.is_empty() && !highlight.text.is_empty());
            assert!(!highlight.text.contains("  "), "{}", highlight.name);
        }
        // The numbered ones stay, in their old order.
        let numbered: Vec<_> = HIGHLIGHTS
            .iter()
            .take(NUMBERED.len())
            .map(|h| h.name)
            .collect();
        assert_eq!(numbered, NUMBERED);
    }

    #[test]
    fn translated_or_english() {
        const JA: &[Translation] = &[Translation {
            folder: "ja",
            title: "新着",
            text: "本文",
        }];
        let highlight = Highlight {
            name: "2026-09-27-0444-about-katna",
            title: "About Katna",
            text: "Help > About Katna",
            translations: JA,
            animation: None,
        };
        assert_eq!(highlight.in_language("ja"), ("新着", "本文"));
        assert_eq!(
            highlight.in_language("de"),
            ("About Katna", "Help > About Katna")
        );
        assert_eq!(
            highlight.in_language("en"),
            ("About Katna", "Help > About Katna")
        );
        // Nothing applied in tests: English.
        assert_eq!(highlight.title(), "About Katna");
        assert_eq!(highlight.text(), "Help > About Katna");
        // Every built-in translation is whole and one paragraph.
        for highlight in HIGHLIGHTS {
            for t in highlight.translations {
                assert!(!t.title.is_empty() && !t.text.is_empty());
                assert!(!t.text.contains("  "), "{} {}", t.folder, highlight.name);
                assert_eq!(highlight.in_language(t.folder), (t.title, t.text));
            }
        }
    }

    #[test]
    fn animations_are_small_animated_webp() {
        for highlight in HIGHLIGHTS {
            let Some(animation) = &highlight.animation else {
                continue;
            };
            for bytes in [animation.light, animation.dark] {
                assert!(
                    bytes.len() <= MAX_ANIMATION_BYTES,
                    "{}: {} bytes",
                    highlight.name,
                    bytes.len()
                );
                assert_eq!(&bytes[..4], b"RIFF");
                assert_eq!(&bytes[8..12], b"WEBP");
            }
        }
    }

    #[test]
    fn unseen_is_newest_first_and_capped() {
        let none = Seen::new(&[], None);
        let (all, more) = unseen(&none);
        assert_eq!(all.len(), HIGHLIGHTS.len().min(SHOWN));
        assert_eq!(more, HIGHLIGHTS.len().saturating_sub(SHOWN));
        let major: Vec<bool> = all.iter().map(|h| h.animation.is_some()).collect();
        assert!(major.windows(2).all(|w| w[0] >= w[1]), "major ones first");
        let rest: Vec<&str> = all
            .iter()
            .filter(|h| h.animation.is_none())
            .map(|h| h.name)
            .collect();
        assert!(rest.windows(2).all(|w| w[0] > w[1]), "then newest first");
        if HIGHLIGHTS.iter().any(|h| h.animation.is_some()) {
            assert!(major[0], "an older major highlight still shows");
        }
        let every = names();
        assert!(unseen(&Seen::new(&every, None)).0.is_empty());
        // One merged late, after newer ones were shown, still shows.
        let late = HIGHLIGHTS[1].name;
        let others: Vec<String> = every.iter().filter(|n| *n != late).cloned().collect();
        let (one, more) = unseen(&Seen::new(&others, None));
        assert_eq!((one.len(), more, one[0].name), (1, 0, late));
    }

    #[test]
    fn recent_is_the_newest_only() {
        let (shown, more) = recent();
        assert_eq!(shown.len(), HIGHLIGHTS.len().min(SHOWN));
        assert_eq!(more, HIGHLIGHTS.len().saturating_sub(SHOWN));
        let start = HIGHLIGHTS.len().saturating_sub(SHOWN);
        let newest: HashSet<&str> = HIGHLIGHTS[start..].iter().map(|h| h.name).collect();
        assert!(
            shown.iter().all(|h| newest.contains(h.name)),
            "no older one"
        );
    }

    #[test]
    fn numbered_settings_carry_over() {
        // A settings file from before the names: 14 shown.
        let seen = Seen::new(&[], Some(14));
        assert!(seen.any());
        let (shown, more) = unseen(&seen);
        let expected = HIGHLIGHTS.len() - 14;
        assert_eq!(shown.len() + more, expected);
        assert!(shown.iter().all(|h| !NUMBERED[..14].contains(&h.name)));
        let all = Seen::new(&[], Some(NUMBERED.len() as u32));
        assert_eq!(
            unseen(&all).0.len() + unseen(&all).1,
            HIGHLIGHTS.len() - NUMBERED.len()
        );
        assert!(!Seen::new(&[], None).any());
    }

    #[test]
    fn changelog_links() {
        let repo = env!("CARGO_PKG_REPOSITORY");
        assert_eq!(
            changelog_url_for(Some("0.0.0.r84.gf89d172"), "0.0.0.r90.gabc1234"),
            format!("{repo}/compare/f89d172...abc1234")
        );
        assert_eq!(
            changelog_url_for(None, "0.0.0.r90.gabc1234"),
            format!("{repo}/commits/abc1234")
        );
        assert_eq!(
            changelog_url_for(Some("0.0.0.r90.gabc1234"), "0.0.0.r90.gabc1234"),
            format!("{repo}/commits/abc1234")
        );
        assert_eq!(
            changelog_url_for(Some("0.1.0"), "0.2.0"),
            format!("{repo}/commits/main")
        );
        assert_eq!(commit("1.2.0.r3.g0123abc"), Some("0123abc"));
        assert_eq!(commit("0.0.0.r3.gnothex"), None);
    }

    #[test]
    fn first_run_or_update() {
        assert_eq!(start(false, false, false), Start::FirstRun);
        // Settings saved by a version before What's new: an update.
        assert_eq!(start(false, false, true), Start::Returning);
        assert_eq!(start(true, false, false), Start::Returning);
        assert_eq!(start(false, true, false), Start::Returning);
    }
}
