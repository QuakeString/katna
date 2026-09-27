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
        title: "Search your settings",
        text: "With Settings open, the search box finds any setting and takes you \
               to it. Settings has more tabs, in a clearer order, and long \
               explanations sit behind an (i) button.",
        animation: None,
    },
    Highlight {
        id: 7,
        title: "Crash reports stay on your computer",
        text: "If Katna Mail or its background service crashes, the next start says so, \
               with the report to view or copy for a bug report. Settings > User \
               feedback lists them.",
        animation: None,
    },
    Highlight {
        id: 8,
        title: "A calmer reply",
        text: "Reply in the reading pane keeps Send and the formatting at the \
               bottom, folds the quoted mail behind \"...\", and Pop out opens \
               the reply in its own window. On a phone, writing takes the whole \
               window.",
        animation: None,
    },
    Highlight {
        id: 9,
        title: "Mail in its own window",
        text: "Shift+click a message, right-click it or use In new window on the \
               reader toolbar to open it in a window of its own. Print all prints \
               the whole conversation.",
        animation: None,
    },
    Highlight {
        id: 10,
        title: "Settings on a phone",
        text: "On a narrow screen Settings and Quick settings fill the window, \
               and every menu and popover closes with Escape or a click outside.",
        animation: None,
    },
    Highlight {
        id: 11,
        title: "Choose how much mail stays offline",
        text: "Settings > General > Offline mail keeps a week, a month, three \
               months, a year or all of your mail on this computer. Older mail \
               downloads when you open it.",
        animation: None,
    },
    Highlight {
        id: 12,
        title: "A new folder pane button",
        text: "The button at the top left shows and hides the folder pane, and \
               its left side is filled while the folders show.",
        animation: None,
    },
    Highlight {
        id: 13,
        title: "Cleaner cards, calmer phone",
        text: "The list and reader sit on cards with a faint outline and a short \
               shadow, menus always stay inside the window, and on a phone the \
               search and toolbar rows slide away as you scroll.",
        animation: None,
    },
    Highlight {
        id: 14,
        title: "About Katna",
        text: "Help > About Katna, also in Quick settings, shows the version, \
               the changelog and every library Katna is built on, with its \
               authors and license. The version in the Settings header opens \
               it too.",
        animation: None,
    },
    Highlight {
        id: 15,
        title: "Settings tabs on one line",
        text: "The Settings tabs stay on one line: when they don't fit, arrows at \
               the edges and the scroll wheel glide the rest into view. Signatures \
               and templates share a Compose tab, and folders and mail rules \
               share one too.",
        animation: None,
    },
    Highlight {
        id: 16,
        title: "Help improve Katna, if you like",
        text: "Katna asks once whether to send crash reports to help fix what went \
               wrong. They go without your IP address, messages or email addresses, \
               and only if you say yes. Change it any time in Settings > User feedback.",
        animation: None,
    },
    Highlight {
        id: 17,
        title: "Attachments of older mail open at once",
        text: "Clicking an attachment of older mail that is not on this computer yet \
               downloads it: the attachment fills up while it does, then opens.",
        animation: None,
    },
    Highlight {
        id: 18,
        title: "Read conversations your way",
        text: "Settings > General > Reading can show the newest message first, open \
               the full headers of every message, and name recipients in full \
               instead of by first name.",
        animation: None,
    },
    Highlight {
        id: 19,
        title: "Send and archive",
        text: "The menu beside Send on a reply sends it and archives the \
               conversation. Settings > Compose can make that what Send does, and \
               can send new mail from the same account every time.",
        animation: None,
    },
    Highlight {
        id: 20,
        title: "Make everything bigger or smaller",
        text: "Settings > Appearance > Scaling sizes the whole window, text, icons \
               and spacing alike, from 75% to 200% on top of your desktop's scale.",
        animation: None,
    },
    Highlight {
        id: 21,
        title: "Shortcuts from the mail app you know",
        text: "Settings > Shortcuts can start from the keys of Gmail, Inbox by Gmail, \
               Apple Mail, Outlook or Thunderbird, lists them in two columns, and \
               Restore defaults takes back your changes.",
        animation: None,
    },
    Highlight {
        id: 22,
        title: "More settings",
        text: "Open Katna Mail at login, choose when mail is marked read, make the \
               reply button reply to everyone, always show images, mute the new-mail \
               sound, hide Important markers, narrow long lines, keep mail's own \
               colors in dark mode, turn off attachment previews, write in plain \
               text, pick the spelling language and see saved files in their folder.",
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
        if HIGHLIGHTS.iter().any(|h| h.animation.is_some()) {
            assert!(major[0], "an older major highlight still shows");
        }
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
