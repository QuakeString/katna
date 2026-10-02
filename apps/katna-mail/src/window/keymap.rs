// SPDX-License-Identifier: GPL-3.0-or-later

//! Keyboard shortcuts: every one Katna Mail has, with webmail's keys as
//! defaults, another mail app's keys if the user picks its set, and the
//! user's changes from `[shortcuts]` in `config.toml`. The Settings page
//! lists and edits them; [`bind`] loads them into GPUI.

use gpui::{Action, App, KeyBinding, Keystroke};
use katna_core::config::{ShortcutSet, Shortcuts};
use katna_i18n::tr;

use super::{
    AddToTasks, Archive, CloseMessage, Compose, Delete, FocusList, FocusNext, FocusPrevious,
    FocusSearch, Forward, GoToAllMail, GoToDrafts, GoToInbox, GoToSent, GoToStarred, LIST_CONTEXT,
    ListTop, MarkImportant, MarkNotImportant, MarkRead, MarkUnread, MoveTo, NAV_CONTEXT, NextPane,
    OpenContextMenu, OpenMessage, OpenSettings, PageDown, PageUp, PreviousPane, Quit,
    READER_CONTEXT, Reload, RephraseSelection, Reply, ReplyAll, ReportSpam, SEARCH_CONTEXT,
    ScrollDown, ScrollPageDown, ScrollPageUp, ScrollUp, SelectAll, SelectFirst, SelectLast,
    SelectNext, SelectNone, SelectPrevious, SendMail, ShowCalendar, ShowContacts, ShowFiles,
    ShowMail, ShowNotes, ShowShortcuts, ShowTasks, Summarize, ToggleCheck, ToggleMute,
    ToggleNavigation, ToggleSettings, ToggleStar, Undo, WINDOW_CONTEXT,
};

/// Where a shortcut works.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Scope {
    /// The message list.
    List,
    /// The open conversation.
    Reader,
    /// The list and the open conversation.
    Mail,
    /// The whole window. Keys without Ctrl or Alt still only work in the
    /// list and the conversation, so they never take a typed letter.
    Anywhere,
}

impl Scope {
    fn contexts(self, keys: &str) -> &'static [&'static str] {
        match self {
            Self::List => &[LIST_CONTEXT],
            Self::Reader => &[READER_CONTEXT],
            Self::Mail => &[LIST_CONTEXT, READER_CONTEXT],
            // The folder pane too, so Compose, search and the like still
            // work after a click on a folder.
            Self::Anywhere if is_single_key(keys) => &[LIST_CONTEXT, READER_CONTEXT, NAV_CONTEXT],
            Self::Anywhere => &[WINDOW_CONTEXT],
        }
    }

    /// Whether the same key could mean two things.
    fn overlaps(self, other: Self) -> bool {
        !matches!(
            (self, other),
            (Self::List, Self::Reader) | (Self::Reader, Self::List)
        )
    }
}

/// Headings of the shortcut list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Group {
    Moving,
    Actions,
    GoTo,
    App,
}

impl Group {
    pub(super) const ALL: [Self; 4] = [Self::Moving, Self::Actions, Self::GoTo, Self::App];

    /// The heading, in the current language.
    pub(super) fn title(self) -> String {
        match self {
            Self::Moving => tr!("shortcut-group-moving"),
            Self::Actions => tr!("shortcut-group-actions"),
            Self::GoTo => tr!("shortcut-group-go-to"),
            Self::App => tr!("shortcut-group-app"),
        }
    }
}

/// One shortcut.
pub(super) struct Shortcut {
    /// Stable name, the key in `[shortcuts.keys]`.
    pub name: &'static str,
    pub group: Group,
    pub scope: Scope,
    pub defaults: &'static [&'static str],
    action: fn() -> Box<dyn Action>,
}

impl Shortcut {
    /// What the shortcut does, in the current language: message
    /// `shortcut-<name>`, with `-` for `_`.
    pub(super) fn title(&self) -> String {
        tr!(&self.message())
    }

    /// What the shortcut does, in English.
    pub(super) fn english_title(&self) -> String {
        katna_i18n::english(&self.message())
    }

    fn message(&self) -> String {
        format!("shortcut-{}", self.name.replace('_', "-"))
    }
}

macro_rules! shortcut {
    ($name:literal, $group:ident, $scope:ident, [$($key:literal),*], $action:expr) => {
        Shortcut {
            name: $name,
            group: Group::$group,
            scope: Scope::$scope,
            defaults: &[$($key),*],
            action: || Box::new($action),
        }
    };
}

/// Every shortcut, in the order the Settings page lists them.
pub(super) static SHORTCUTS: &[Shortcut] = &[
    shortcut!("next", Moving, Mail, ["j"], SelectNext),
    shortcut!("previous", Moving, Mail, ["k"], SelectPrevious),
    shortcut!("down", Moving, List, ["down"], SelectNext),
    shortcut!("up", Moving, List, ["up"], SelectPrevious),
    shortcut!("first", Moving, List, ["home"], SelectFirst),
    shortcut!("last", Moving, List, ["end"], SelectLast),
    shortcut!("page_down", Moving, List, ["pagedown"], PageDown),
    shortcut!("page_up", Moving, List, ["pageup"], PageUp),
    shortcut!("open", Moving, List, ["enter", "o"], OpenMessage),
    shortcut!(
        "back",
        Moving,
        Mail,
        ["u", "escape", "backspace"],
        CloseMessage
    ),
    shortcut!("scroll_down", Moving, Reader, ["down"], ScrollDown),
    shortcut!("scroll_up", Moving, Reader, ["up"], ScrollUp),
    shortcut!(
        "scroll_page_down",
        Moving,
        Reader,
        ["pagedown", "space"],
        ScrollPageDown
    ),
    shortcut!(
        "scroll_page_up",
        Moving,
        Reader,
        ["pageup", "shift-space"],
        ScrollPageUp
    ),
    shortcut!("compose", Actions, Anywhere, ["c"], Compose),
    shortcut!("reply", Actions, Reader, ["r"], Reply),
    shortcut!("reply_all", Actions, Reader, ["a"], ReplyAll),
    shortcut!("forward", Actions, Reader, ["f"], Forward),
    shortcut!("archive", Actions, Mail, ["e"], Archive),
    shortcut!("delete", Actions, Mail, ["#", "delete"], Delete),
    shortcut!("spam", Actions, Mail, ["!"], ReportSpam),
    shortcut!("move_to", Actions, Mail, ["v"], MoveTo),
    shortcut!("mark_read", Actions, Mail, ["shift-i"], MarkRead),
    shortcut!("mark_unread", Actions, Mail, ["shift-u"], MarkUnread),
    shortcut!("star", Actions, Mail, ["s"], ToggleStar),
    shortcut!("add_to_tasks", Actions, Mail, ["shift-t"], AddToTasks),
    shortcut!("important", Actions, Mail, ["+", "="], MarkImportant),
    shortcut!("not_important", Actions, Mail, ["-"], MarkNotImportant),
    shortcut!("mute", Actions, Mail, ["m"], ToggleMute),
    shortcut!("summarize", Actions, Mail, ["shift-s"], Summarize),
    shortcut!("check", Actions, List, ["x"], ToggleCheck),
    shortcut!("select_all", Actions, List, ["* a"], SelectAll),
    shortcut!("select_none", Actions, List, ["* n"], SelectNone),
    shortcut!("undo", Actions, Anywhere, ["z", "ctrl-z"], Undo),
    shortcut!("go_inbox", GoTo, Anywhere, ["g i"], GoToInbox),
    shortcut!("go_starred", GoTo, Anywhere, ["g s"], GoToStarred),
    shortcut!("go_sent", GoTo, Anywhere, ["g t"], GoToSent),
    shortcut!("go_drafts", GoTo, Anywhere, ["g d"], GoToDrafts),
    shortcut!("go_all", GoTo, Anywhere, ["g a"], GoToAllMail),
    // The pages of the window, on Outlook's keys.
    shortcut!("page_mail", GoTo, Anywhere, ["ctrl-1"], ShowMail),
    shortcut!("page_calendar", GoTo, Anywhere, ["ctrl-2"], ShowCalendar),
    shortcut!("page_contacts", GoTo, Anywhere, ["ctrl-3"], ShowContacts),
    shortcut!("page_tasks", GoTo, Anywhere, ["ctrl-4"], ShowTasks),
    shortcut!("page_notes", GoTo, Anywhere, ["ctrl-5"], ShowNotes),
    shortcut!("page_files", GoTo, Anywhere, ["ctrl-7"], ShowFiles),
    shortcut!("search", App, Anywhere, ["/", "ctrl-f"], FocusSearch),
    shortcut!("navigation", App, Anywhere, [], ToggleNavigation),
    shortcut!("quick_settings", App, Anywhere, ["ctrl-,"], ToggleSettings),
    shortcut!("settings", App, Anywhere, [], OpenSettings),
    shortcut!("shortcuts", App, Anywhere, ["?"], ShowShortcuts),
    shortcut!("reload", App, Anywhere, ["f5", "ctrl-r"], Reload),
    shortcut!("quit", App, Anywhere, ["ctrl-q"], Quit),
];

pub(super) fn find(name: &str) -> Option<&'static Shortcut> {
    SHORTCUTS.iter().find(|s| s.name == name)
}

/// Keys a set gives shortcuts, in place of Katna's; a shortcut the other
/// app has no key for keeps Katna's.
type Preset = &'static [(&'static str, &'static [&'static str])];

/// Gmail's own keys: Katna's, and D also writes a new message.
const GMAIL: Preset = &[("compose", &["c", "d"])];

/// Inbox by Gmail: Y marks done too, and nothing is important.
const INBOX_BY_GMAIL: Preset = &[
    ("archive", &["e", "y"]),
    ("important", &[]),
    ("not_important", &[]),
];

/// Apple Mail's, with Ctrl for Cmd and Alt for Control.
const APPLE_MAIL: Preset = &[
    ("compose", &["ctrl-n"]),
    ("reply", &["ctrl-r"]),
    ("reply_all", &["ctrl-shift-r"]),
    ("forward", &["ctrl-shift-f"]),
    ("archive", &["ctrl-alt-a"]),
    ("delete", &["delete", "backspace"]),
    ("back", &["escape"]),
    ("spam", &["ctrl-shift-j"]),
    ("mark_unread", &["ctrl-shift-u"]),
    ("star", &["ctrl-shift-l"]),
    ("select_all", &["ctrl-a"]),
    ("undo", &["ctrl-z"]),
    ("go_inbox", &["ctrl-1"]),
    ("page_mail", &[]),
    ("search", &["ctrl-alt-f"]),
    ("reload", &["ctrl-shift-n"]),
];

/// Outlook's on Windows.
const OUTLOOK: Preset = &[
    ("next", &["ctrl-."]),
    ("previous", &["ctrl-,"]),
    ("open", &["enter"]),
    ("back", &["escape"]),
    ("compose", &["ctrl-n", "ctrl-shift-m"]),
    ("reply", &["ctrl-r"]),
    ("reply_all", &["ctrl-shift-r"]),
    ("forward", &["ctrl-f"]),
    ("archive", &["backspace"]),
    ("delete", &["delete", "ctrl-d"]),
    ("spam", &["ctrl-alt-j"]),
    ("move_to", &["ctrl-shift-v"]),
    ("mark_read", &["ctrl-q"]),
    ("mark_unread", &["ctrl-u"]),
    ("star", &["insert"]),
    ("important", &[]),
    ("not_important", &[]),
    ("select_all", &["ctrl-a"]),
    ("undo", &["ctrl-z"]),
    ("go_inbox", &["ctrl-shift-i"]),
    ("search", &["ctrl-e", "f3"]),
    ("quick_settings", &[]),
    ("reload", &["f9", "f5"]),
    ("quit", &[]),
];

/// Thunderbird's.
const THUNDERBIRD: Preset = &[
    ("next", &["f"]),
    ("previous", &["b"]),
    ("compose", &["ctrl-n", "ctrl-m"]),
    ("reply", &["ctrl-r"]),
    ("reply_all", &["ctrl-shift-r"]),
    ("forward", &["ctrl-l"]),
    ("archive", &["a"]),
    ("delete", &["delete"]),
    ("spam", &["j"]),
    ("mark_read", &["r"]),
    ("mark_unread", &["m"]),
    // Ignore thread.
    ("mute", &["k"]),
    ("select_all", &["ctrl-a"]),
    ("undo", &["ctrl-z"]),
    ("search", &["ctrl-k", "ctrl-shift-k"]),
    ("reload", &["f5", "ctrl-t"]),
];

/// Every set, in the order the Settings page offers them, with its name.
pub(super) const SETS: [(ShortcutSet, &str); 6] = [
    (ShortcutSet::Katna, "Katna Mail"),
    (ShortcutSet::Gmail, "Gmail"),
    (ShortcutSet::InboxByGmail, "Inbox by Gmail"),
    (ShortcutSet::AppleMail, "Apple Mail"),
    (ShortcutSet::Outlook, "Outlook"),
    (ShortcutSet::Thunderbird, "Thunderbird"),
];

fn preset(set: ShortcutSet) -> Preset {
    match set {
        ShortcutSet::Katna => &[],
        ShortcutSet::Gmail => GMAIL,
        ShortcutSet::InboxByGmail => INBOX_BY_GMAIL,
        ShortcutSet::AppleMail => APPLE_MAIL,
        ShortcutSet::Outlook => OUTLOOK,
        ShortcutSet::Thunderbird => THUNDERBIRD,
    }
}

/// The keys `set` gives `shortcut`, before the user's changes.
pub(super) fn set_keys(shortcut: &Shortcut, set: ShortcutSet) -> &'static [&'static str] {
    preset(set)
        .iter()
        .find(|(name, _)| *name == shortcut.name)
        .map_or(shortcut.defaults, |(_, keys)| keys)
}

/// The keys of `shortcut` now: the user's, else the set's.
pub(super) fn keys<'a>(shortcut: &Shortcut, config: &'a Shortcuts) -> Vec<&'a str> {
    match config.keys.get(shortcut.name) {
        Some(keys) => keys.iter().map(String::as_str).collect(),
        None => set_keys(shortcut, config.set).to_vec(),
    }
}

/// How the shortcut `name` reads on screen, for a hint beside what it
/// does: its keys with Ctrl or Alt first, then a single key while those
/// are on; `None` when it has no keys that work.
pub(super) fn hint(name: &str, config: &Shortcuts) -> Option<String> {
    let keys = keys(find(name)?, config);
    let pick = keys
        .iter()
        .find(|k| !is_single_key(k))
        .or_else(|| keys.iter().find(|_| config.single_keys))?;
    Some(label(pick))
}

/// Keys without Ctrl, Alt or Super: letters, `#`, `?`, sequences like
/// `g i`. Keys that do not type a character (Enter, arrows, F5, …) are not.
pub(super) fn is_single_key(keys: &str) -> bool {
    let Some(first) = keys.split_whitespace().next() else {
        return false;
    };
    let Ok(stroke) = Keystroke::parse(first) else {
        return false;
    };
    let m = stroke.modifiers;
    !(m.control || m.alt || m.platform || m.function) && stroke.key.chars().count() == 1
}

/// Whether the first key of `keys` types text in a field: a single key,
/// or Space.
fn types_text(keys: &str) -> bool {
    let first = keys.split_whitespace().next().unwrap_or_default();
    is_single_key(first) || matches!(first, "space" | "shift-space")
}

/// Whether `keys` parse as GPUI keystrokes.
pub(super) fn valid(keys: &str) -> bool {
    !keys.trim().is_empty() && keys.split_whitespace().all(|k| Keystroke::parse(k).is_ok())
}

/// The shortcut other than `name` that `keys` already starts where it
/// would also work.
pub(super) fn conflict(name: &str, keys: &str, config: &Shortcuts) -> Option<&'static Shortcut> {
    let scope = find(name)?.scope;
    SHORTCUTS.iter().find(|s| {
        s.name != name && s.scope.overlaps(scope) && self::keys(s, config).contains(&keys)
    })
}

/// Loads every shortcut into GPUI, replacing what was bound before.
pub fn bind(config: &Shortcuts, cx: &mut App) {
    cx.clear_key_bindings();
    let mut bindings = Vec::new();
    let mut typed = std::collections::BTreeSet::new();
    for shortcut in SHORTCUTS {
        for keys in self::keys(shortcut, config) {
            if !valid(keys) {
                tracing::warn!("shortcut {}: {keys:?} is not a key", shortcut.name);
                continue;
            }
            if !config.single_keys && is_single_key(keys) {
                continue;
            }
            if types_text(keys) {
                typed.insert(keys);
            }
            for context in shortcut.scope.contexts(keys) {
                let action = (shortcut.action)();
                let context = gpui::KeyBindingContextPredicate::parse(context).ok();
                match KeyBinding::load(
                    keys,
                    action,
                    context.map(Into::into),
                    false,
                    None,
                    &gpui::DummyKeyboardMapper,
                ) {
                    Ok(binding) => bindings.push(binding),
                    Err(err) => tracing::warn!("shortcut {}: {err}", shortcut.name),
                }
            }
        }
    }
    // Clicked text of a message copies and selects as it does anywhere.
    bindings.push(KeyBinding::new(
        "ctrl-c",
        super::select::CopyText,
        Some(super::select::TEXT_CONTEXT),
    ));
    bindings.push(KeyBinding::new(
        "ctrl-a",
        super::select::SelectAllText,
        Some(super::select::TEXT_CONTEXT),
    ));
    // Google Calendar's keys on the Calendar page.
    bindings.extend(super::calendar::bindings());
    // Home with the keys in no pane takes the list to its top; the
    // list's own Home (a shortcut) selects its first line.
    bindings.push(KeyBinding::new("home", ListTop, Some(WINDOW_CONTEXT)));
    // Down in the search box goes to the list; not a shortcut to change.
    bindings.push(KeyBinding::new("down", FocusList, Some(SEARCH_CONTEXT)));
    // Tab and Shift+Tab move between fields and buttons, as in any desktop
    // form. A rich editor uses them first in tables and lists.
    bindings.push(KeyBinding::new("tab", FocusNext, Some(WINDOW_CONTEXT)));
    bindings.push(KeyBinding::new(
        "shift-tab",
        FocusPrevious,
        Some(WINDOW_CONTEXT),
    ));
    // F6 and Shift+F6 go round the panes from anywhere, fields included.
    bindings.push(KeyBinding::new("f6", NextPane, Some(WINDOW_CONTEXT)));
    bindings.push(KeyBinding::new(
        "shift-f6",
        PreviousPane,
        Some(WINDOW_CONTEXT),
    ));
    // Shift+F10 and the Menu key open the selected line's menu.
    for keys in ["shift-f10", "menu"] {
        bindings.push(KeyBinding::new(keys, OpenContextMenu, Some(LIST_CONTEXT)));
    }
    // Ctrl+Enter sends from any field of a message, not only its text.
    bindings.push(KeyBinding::new(
        "ctrl-enter",
        SendMail,
        Some("Compose > TextInput"),
    ));
    // Ctrl+J rephrases the text selected in a message with AI.
    bindings.push(KeyBinding::new(
        "ctrl-j",
        RephraseSelection,
        Some("Compose > RichText"),
    ));
    // Typing in a field inside the reader (the inline reply) types: keys
    // that type text do nothing else there, and do not wait for a second
    // key. Bound last, so they also end sequences like "g i".
    for keys in typed {
        for context in [
            katna_ui::TEXT_AREA_CONTEXT,
            katna_ui::text_input::KEY_CONTEXT,
        ] {
            bindings.push(KeyBinding::new(keys, gpui::NoAction, Some(context)));
        }
    }
    // Ctrl+Z in a field is about its text, never the mail: the rich
    // editor undoes typing with it, and the plain fields do nothing.
    for context in [
        katna_ui::TEXT_AREA_CONTEXT,
        katna_ui::text_input::KEY_CONTEXT,
    ] {
        bindings.push(KeyBinding::new("ctrl-z", gpui::NoAction, Some(context)));
    }
    // In the attachment viewer, Ctrl+Z and redo are about the marks made
    // on a PDF, and Ctrl+R (Shift for anticlockwise) turns its pages; the
    // viewer takes them itself.
    for keys in ["ctrl-z", "ctrl-shift-z", "ctrl-y", "ctrl-r", "ctrl-shift-r"] {
        bindings.push(KeyBinding::new(
            keys,
            gpui::NoAction,
            Some(super::viewer::KEY_CONTEXT),
        ));
    }
    cx.bind_keys(bindings);
    katna_ui::text_input::bind_keys(cx);
    katna_ui::text_area::bind_keys(cx);
    katna_ui::rich::bind_keys(cx);
}

/// How keys read on screen: `ctrl-shift-a` is "Ctrl+Shift+A", `g i` is
/// "G then I".
pub(super) fn label(keys: &str) -> String {
    keys.split_whitespace()
        .map(stroke_label)
        .reduce(|first, second| {
            tr!(
                "shortcut-sequence",
                first = first.as_str(),
                second = second.as_str()
            )
        })
        .unwrap_or_default()
}

fn stroke_label(stroke: &str) -> String {
    let Ok(stroke) = Keystroke::parse(stroke) else {
        return stroke.to_owned();
    };
    let m = stroke.modifiers;
    let mut parts: Vec<String> = Vec::new();
    for (on, name) in [
        (m.control, "Ctrl"),
        (m.alt, "Alt"),
        (m.platform, "Super"),
        (m.function, "Fn"),
        (m.shift, "Shift"),
    ] {
        if on {
            parts.push(name.to_owned());
        }
    }
    let key = match stroke.key.as_str() {
        "enter" => "Enter".to_owned(),
        "escape" => "Esc".to_owned(),
        "backspace" => "Backspace".to_owned(),
        "delete" => "Delete".to_owned(),
        "insert" => "Insert".to_owned(),
        "space" => "Space".to_owned(),
        "tab" => "Tab".to_owned(),
        "up" => "\u{2191}".to_owned(),
        "down" => "\u{2193}".to_owned(),
        "left" => "\u{2190}".to_owned(),
        "right" => "\u{2192}".to_owned(),
        "home" => "Home".to_owned(),
        "end" => "End".to_owned(),
        "pageup" => "Page Up".to_owned(),
        "pagedown" => "Page Down".to_owned(),
        key if key.len() > 1 && key.starts_with('f') && key[1..].parse::<u8>().is_ok() => {
            key.to_uppercase()
        }
        key => {
            let mut chars = key.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect(),
                None => String::new(),
            }
        }
    };
    parts.push(key);
    parts.join("+")
}

/// Keys as GPUI writes them, from a pressed keystroke; `None` for a
/// modifier alone.
pub(super) fn from_stroke(stroke: &Keystroke) -> Option<String> {
    let key = stroke.key.as_str();
    if key.is_empty() || matches!(key, "shift" | "control" | "alt" | "platform" | "function") {
        return None;
    }
    Some(stroke.unparse())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn names_are_unique_and_defaults_parse() {
        let mut names = HashSet::new();
        for s in SHORTCUTS {
            assert!(names.insert(s.name), "{} twice", s.name);
            for keys in s.defaults {
                assert!(valid(keys), "{}: {keys}", s.name);
            }
        }
    }

    #[test]
    fn defaults_do_not_conflict() {
        let config = Shortcuts::default();
        for s in SHORTCUTS {
            for keys in s.defaults {
                assert!(
                    conflict(s.name, keys, &config).is_none(),
                    "{} {keys} conflicts with {:?}",
                    s.name,
                    conflict(s.name, keys, &config).map(|c| c.name)
                );
            }
        }
    }

    #[test]
    fn sets_name_real_shortcuts_and_do_not_conflict() {
        for (set, _) in SETS {
            for (name, keys) in preset(set) {
                assert!(find(name).is_some(), "{set:?}: no shortcut {name}");
                for keys in *keys {
                    assert!(valid(keys), "{set:?} {name}: {keys}");
                }
            }
            let config = Shortcuts {
                set,
                ..Shortcuts::default()
            };
            for s in SHORTCUTS {
                for keys in keys(s, &config) {
                    assert!(
                        conflict(s.name, keys, &config).is_none(),
                        "{set:?}: {} {keys} conflicts with {:?}",
                        s.name,
                        conflict(s.name, keys, &config).map(|c| c.name)
                    );
                }
            }
        }
    }

    #[test]
    fn user_keys_sit_on_the_set() {
        let mut config = Shortcuts {
            set: ShortcutSet::Outlook,
            ..Shortcuts::default()
        };
        let reply = find("reply").unwrap();
        assert_eq!(keys(reply, &config), ["ctrl-r"]);
        // Katna's keys where Outlook has none.
        assert_eq!(keys(find("go_sent").unwrap(), &config), ["g t"]);
        config.keys.insert("reply".into(), vec!["r".into()]);
        assert_eq!(keys(reply, &config), ["r"]);
    }

    #[test]
    fn user_keys_replace_the_defaults() {
        let mut config = Shortcuts::default();
        let archive = find("archive").unwrap();
        assert_eq!(keys(archive, &config), ["e"]);
        config.keys.insert("archive".into(), vec!["y".into()]);
        assert_eq!(keys(archive, &config), ["y"]);
        config.keys.insert("archive".into(), Vec::new());
        assert!(keys(archive, &config).is_empty());
        // `e` is free now; `y` conflicts with nothing, `s` with star.
        assert!(conflict("delete", "e", &config).is_none());
        assert_eq!(
            conflict("delete", "s", &config).map(|s| s.name),
            Some("star")
        );
        // The list and the conversation do not share keys.
        assert!(conflict("down", "down", &config).is_none());
        assert!(conflict("scroll_down", "down", &config).is_none());
    }

    #[test]
    fn single_keys() {
        for keys in ["e", "#", "shift-i", "g i", "?", "* a"] {
            assert!(is_single_key(keys), "{keys}");
        }
        for keys in ["ctrl-f", "f5", "enter", "down", "alt-x", "escape", "space"] {
            assert!(!is_single_key(keys), "{keys}");
        }
    }

    #[test]
    fn labels() {
        assert_eq!(label("ctrl-shift-a"), "Ctrl+Shift+A");
        assert_eq!(label("shift-i"), "Shift+I");
        assert_eq!(label("g i"), "G then I");
        assert_eq!(label("#"), "#");
        assert_eq!(label("pagedown"), "Page Down");
        assert_eq!(label("f5"), "F5");
        assert_eq!(label("ctrl-,"), "Ctrl+,");
        assert_eq!(label("escape"), "Esc");
    }

    /// Every shortcut and group has its English message.
    #[test]
    fn titles_have_messages() {
        for s in SHORTCUTS {
            let title = s.title();
            assert!(!title.starts_with("shortcut-"), "no message for {}", s.name);
        }
        assert_eq!(Group::Moving.title(), "Moving around");
        assert_eq!(SHORTCUTS[0].title(), "Next conversation");
    }

    #[test]
    fn pressed_keys() {
        let stroke = Keystroke::parse("ctrl-shift-k").unwrap();
        assert_eq!(from_stroke(&stroke).as_deref(), Some("ctrl-shift-k"));
        let stroke = Keystroke::parse("shift").unwrap();
        assert_eq!(from_stroke(&stroke), None);
    }
}
