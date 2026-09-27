// SPDX-License-Identifier: GPL-3.0-or-later

//! What's new: the highlights shown once after an update
//! (`docs/ARCHITECTURE.md` §13.6). The list is curated by hand and built
//! into the app; a change people will notice appends a highlight at the end
//! of [`HIGHLIGHTS`] with the next id. A major feature may bring a short
//! animation (animated WebPs in `apps/katna-mail/whats-new/`, one per theme).

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
    /// Goes up by one with each highlight; the config remembers the
    /// newest one shown.
    pub id: u32,
    pub title: &'static str,
    pub text: &'static str,
    /// A short animation of the feature, for major ones only.
    pub animation: Option<Animation>,
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

/// Oldest first. Append new highlights at the end.
pub const HIGHLIGHTS: &[Highlight] = &[
    Highlight {
        id: 1,
        title: "What's new, after every update",
        text: "After an update Katna Mail shows what changed, once, instead of the \
               welcome tour. Quick settings opens it again.",
        animation: None,
    },
    Highlight {
        id: 2,
        title: "Reply buttons that always fit",
        text: "Reply, Reply all and Forward stay on one line: as the window narrows \
               they fold into icons one at a time. On a phone they share the width.",
        animation: Some(Animation {
            light: include_bytes!("../whats-new/reply-row-light.webp"),
            dark: include_bytes!("../whats-new/reply-row-dark.webp"),
        }),
    },
    Highlight {
        id: 3,
        title: "Notifications open their mail",
        text: "Click a new-mail notification to open that message, even when Katna \
               Mail is closed. Notifications also have Reply all.",
        animation: None,
    },
    Highlight {
        id: 4,
        title: "Previews of more attachments",
        text: "Spreadsheets, CSV, text files and documents show a preview on their \
               card, and the viewer's backdrop is lighter.",
        animation: None,
    },
    Highlight {
        id: 5,
        title: "Frosted menus",
        text: "With Settings > Experimental > Blurred background on, menus and \
               popovers turn to frosted glass.",
        animation: None,
    },
    Highlight {
        id: 6,
        title: "About Katna",
        text: "Help > About Katna, also in Quick settings, shows the version, \
               the changelog and the free software Katna is built on. The \
               version in the Settings header opens it too.",
        animation: None,
    },
];

/// The newest highlight's id.
pub fn latest() -> u32 {
    HIGHLIGHTS.last().map_or(0, |h| h.id)
}

/// Highlights newer than `seen`, newest first but major ones (with an
/// animation) before the rest, and how many more there are beyond
/// [`SHOWN`].
pub fn unseen(seen: Option<u32>) -> (Vec<&'static Highlight>, usize) {
    let seen = seen.unwrap_or(0);
    newest(HIGHLIGHTS.iter().filter(|h| h.id > seen))
}

/// The newest highlights, for opening What's new by hand.
pub fn recent() -> (Vec<&'static Highlight>, usize) {
    newest(HIGHLIGHTS.iter())
}

fn newest<'a>(
    highlights: impl DoubleEndedIterator<Item = &'a Highlight>,
) -> (Vec<&'a Highlight>, usize) {
    let all: Vec<_> = highlights.rev().collect();
    let more = all.len().saturating_sub(SHOWN);
    let mut shown: Vec<_> = all.into_iter().take(SHOWN).collect();
    shown.sort_by_key(|h| h.animation.is_none());
    (shown, more)
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
/// first-start help was seen; `seen` is the newest highlight shown;
/// `config_existed` is whether a settings file was there before this start
/// (only Katna Mail writes one). Versions before What's new set neither
/// `seen` nor, if their tour was left open, `done`, but they did save
/// settings.
pub fn start(done: bool, seen: Option<u32>, config_existed: bool) -> Start {
    if done || seen.is_some() || config_existed {
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
    fn ids_go_up_by_one() {
        for (ix, highlight) in HIGHLIGHTS.iter().enumerate() {
            assert_eq!(highlight.id as usize, ix + 1, "{}", highlight.title);
            assert!(!highlight.title.is_empty() && !highlight.text.is_empty());
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
                    highlight.title,
                    bytes.len()
                );
                assert_eq!(&bytes[..4], b"RIFF");
                assert_eq!(&bytes[8..12], b"WEBP");
            }
        }
    }

    #[test]
    fn unseen_is_newest_first_and_capped() {
        let (all, more) = unseen(None);
        assert_eq!(all.len(), HIGHLIGHTS.len().min(SHOWN));
        assert_eq!(more, HIGHLIGHTS.len().saturating_sub(SHOWN));
        let major: Vec<bool> = all.iter().map(|h| h.animation.is_some()).collect();
        assert!(major.windows(2).all(|w| w[0] >= w[1]), "major ones first");
        let rest: Vec<u32> = all
            .iter()
            .filter(|h| h.animation.is_none())
            .map(|h| h.id)
            .collect();
        assert!(rest.windows(2).all(|w| w[0] > w[1]), "then newest first");
        assert!(unseen(Some(latest())).0.is_empty());
        let (one, _) = unseen(Some(latest() - 1));
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].id, latest());
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
        assert_eq!(start(false, None, false), Start::FirstRun);
        // Settings saved by a version before What's new: an update.
        assert_eq!(start(false, None, true), Start::Returning);
        assert_eq!(start(true, None, false), Start::Returning);
        assert_eq!(start(false, Some(2), false), Start::Returning);
    }
}
