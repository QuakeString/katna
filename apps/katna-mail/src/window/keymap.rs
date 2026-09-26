// SPDX-License-Identifier: GPL-3.0-or-later

//! Keyboard shortcuts: every one Katna Mail has, with webmail's keys as
//! defaults, and the user's changes from `[shortcuts]` in `config.toml`.
//! The Settings page lists and edits them; [`bind`] loads them into GPUI.

use gpui::{Action, App, KeyBinding, Keystroke};
use katna_core::config::Shortcuts;

use super::{
    Archive, CloseMessage, Compose, Delete, FocusList, FocusSearch, Forward, GoToAllMail,
    GoToDrafts, GoToInbox, GoToSent, GoToStarred, LIST_CONTEXT, MarkRead, MarkUnread, MoveTo,
    OpenMessage, OpenSettings, PageDown, PageUp, Quit, READER_CONTEXT, Reload, Reply, ReplyAll,
    ReportSpam, SEARCH_CONTEXT, ScrollDown, ScrollPageDown, ScrollPageUp, ScrollUp, SelectAll,
    SelectFirst, SelectLast, SelectNext, SelectNone, SelectPrevious, ShowShortcuts, ToggleCheck,
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
            Self::Anywhere if is_single_key(keys) => &[LIST_CONTEXT, READER_CONTEXT],
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

    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Moving => "Moving around",
            Self::Actions => "Actions",
            Self::GoTo => "Go to",
            Self::App => "Application",
        }
    }
}

/// One shortcut.
pub(super) struct Shortcut {
    /// Stable name, the key in `[shortcuts.keys]`.
    pub name: &'static str,
    pub label: &'static str,
    pub group: Group,
    pub scope: Scope,
    pub defaults: &'static [&'static str],
    action: fn() -> Box<dyn Action>,
}

macro_rules! shortcut {
    ($name:literal, $label:literal, $group:ident, $scope:ident, [$($key:literal),*], $action:expr) => {
        Shortcut {
            name: $name,
            label: $label,
            group: Group::$group,
            scope: Scope::$scope,
            defaults: &[$($key),*],
            action: || Box::new($action),
        }
    };
}

/// Every shortcut, in the order the Settings page lists them.
pub(super) static SHORTCUTS: &[Shortcut] = &[
    shortcut!("next", "Next conversation", Moving, Mail, ["j"], SelectNext),
    shortcut!(
        "previous",
        "Previous conversation",
        Moving,
        Mail,
        ["k"],
        SelectPrevious
    ),
    shortcut!(
        "down",
        "Move down the list",
        Moving,
        List,
        ["down"],
        SelectNext
    ),
    shortcut!(
        "up",
        "Move up the list",
        Moving,
        List,
        ["up"],
        SelectPrevious
    ),
    shortcut!(
        "first",
        "First in the list",
        Moving,
        List,
        ["home"],
        SelectFirst
    ),
    shortcut!(
        "last",
        "Last in the list",
        Moving,
        List,
        ["end"],
        SelectLast
    ),
    shortcut!(
        "page_down",
        "Page down the list",
        Moving,
        List,
        ["pagedown"],
        PageDown
    ),
    shortcut!(
        "page_up",
        "Page up the list",
        Moving,
        List,
        ["pageup"],
        PageUp
    ),
    shortcut!(
        "open",
        "Open conversation",
        Moving,
        List,
        ["enter", "o"],
        OpenMessage
    ),
    shortcut!(
        "back",
        "Back to the list",
        Moving,
        Reader,
        ["u", "escape", "backspace"],
        CloseMessage
    ),
    shortcut!(
        "scroll_down",
        "Scroll down",
        Moving,
        Reader,
        ["down"],
        ScrollDown
    ),
    shortcut!("scroll_up", "Scroll up", Moving, Reader, ["up"], ScrollUp),
    shortcut!(
        "scroll_page_down",
        "Scroll a page down",
        Moving,
        Reader,
        ["pagedown", "space"],
        ScrollPageDown
    ),
    shortcut!(
        "scroll_page_up",
        "Scroll a page up",
        Moving,
        Reader,
        ["pageup", "shift-space"],
        ScrollPageUp
    ),
    shortcut!("compose", "Compose", Actions, Anywhere, ["c"], Compose),
    shortcut!("reply", "Reply", Actions, Reader, ["r"], Reply),
    shortcut!("reply_all", "Reply all", Actions, Reader, ["a"], ReplyAll),
    shortcut!("forward", "Forward", Actions, Reader, ["f"], Forward),
    shortcut!("archive", "Archive", Actions, Mail, ["e"], Archive),
    shortcut!("delete", "Delete", Actions, Mail, ["#", "delete"], Delete),
    shortcut!("spam", "Report spam", Actions, Mail, ["!"], ReportSpam),
    shortcut!("move_to", "Move to", Actions, Mail, ["v"], MoveTo),
    shortcut!(
        "mark_read",
        "Mark as read",
        Actions,
        Mail,
        ["shift-i"],
        MarkRead
    ),
    shortcut!(
        "mark_unread",
        "Mark as unread",
        Actions,
        Mail,
        ["shift-u"],
        MarkUnread
    ),
    shortcut!("star", "Star or unstar", Actions, Mail, ["s"], ToggleStar),
    shortcut!(
        "check",
        "Tick the conversation",
        Actions,
        List,
        ["x"],
        ToggleCheck
    ),
    shortcut!(
        "select_all",
        "Tick all conversations",
        Actions,
        List,
        ["* a"],
        SelectAll
    ),
    shortcut!(
        "select_none",
        "Untick all conversations",
        Actions,
        List,
        ["* n"],
        SelectNone
    ),
    shortcut!(
        "undo",
        "Undo the last action",
        Actions,
        Anywhere,
        ["z"],
        Undo
    ),
    shortcut!("go_inbox", "Inbox", GoTo, Anywhere, ["g i"], GoToInbox),
    shortcut!(
        "go_starred",
        "Starred",
        GoTo,
        Anywhere,
        ["g s"],
        GoToStarred
    ),
    shortcut!("go_sent", "Sent", GoTo, Anywhere, ["g t"], GoToSent),
    shortcut!("go_drafts", "Drafts", GoTo, Anywhere, ["g d"], GoToDrafts),
    shortcut!("go_all", "All mail", GoTo, Anywhere, ["g a"], GoToAllMail),
    shortcut!(
        "search",
        "Search mail",
        App,
        Anywhere,
        ["/", "ctrl-f"],
        FocusSearch
    ),
    shortcut!(
        "navigation",
        "Show or fold the menu",
        App,
        Anywhere,
        [],
        ToggleNavigation
    ),
    shortcut!(
        "quick_settings",
        "Quick settings",
        App,
        Anywhere,
        ["ctrl-,"],
        ToggleSettings
    ),
    shortcut!("settings", "All settings", App, Anywhere, [], OpenSettings),
    shortcut!(
        "shortcuts",
        "Keyboard shortcuts",
        App,
        Anywhere,
        ["?"],
        ShowShortcuts
    ),
    shortcut!(
        "reload",
        "Check for new mail",
        App,
        Anywhere,
        ["f5", "ctrl-r"],
        Reload
    ),
    shortcut!("quit", "Quit", App, Anywhere, ["ctrl-q"], Quit),
];

pub(super) fn find(name: &str) -> Option<&'static Shortcut> {
    SHORTCUTS.iter().find(|s| s.name == name)
}

/// The keys of `shortcut` now: the user's, else the defaults.
pub(super) fn keys<'a>(shortcut: &Shortcut, config: &'a Shortcuts) -> Vec<&'a str> {
    match config.keys.get(shortcut.name) {
        Some(keys) => keys.iter().map(String::as_str).collect(),
        None => shortcut.defaults.to_vec(),
    }
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
    for shortcut in SHORTCUTS {
        for keys in self::keys(shortcut, config) {
            if !valid(keys) {
                tracing::warn!("shortcut {}: {keys:?} is not a key", shortcut.name);
                continue;
            }
            if !config.single_keys && is_single_key(keys) {
                continue;
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
    // Down in the search box goes to the list; not a shortcut to change.
    bindings.push(KeyBinding::new("down", FocusList, Some(SEARCH_CONTEXT)));
    cx.bind_keys(bindings);
    katna_ui::text_input::bind_keys(cx);
    katna_ui::text_area::bind_keys(cx);
}

/// How keys read on screen: `ctrl-shift-a` is "Ctrl+Shift+A", `g i` is
/// "G then I".
pub(super) fn label(keys: &str) -> String {
    keys.split_whitespace()
        .map(stroke_label)
        .collect::<Vec<_>>()
        .join(" then ")
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

    #[test]
    fn pressed_keys() {
        let stroke = Keystroke::parse("ctrl-shift-k").unwrap();
        assert_eq!(from_stroke(&stroke).as_deref(), Some("ctrl-shift-k"));
        let stroke = Keystroke::parse("shift").unwrap();
        assert_eq!(from_stroke(&stroke), None);
    }
}
