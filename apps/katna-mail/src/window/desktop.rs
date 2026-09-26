// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna Mail on the desktop (`docs/ARCHITECTURE.md` §15.2): what the tray,
//! the taskbar icon's actions and notifications ask of the window
//! ([`Request`]), and the menu bar that the KDE global menu shows.
//!
//! Menu items name GPUI actions (`katna_mail::Compose`); an item whose
//! action this build does not have is left out, and its shortcut comes from
//! the keymap.

use gpui::{Action, App, AppContext, Context, Global, Window};
use katna_dbus::app_action;
use katna_platform::dbusmenu::{Menu, MenuItem};
use katna_store::MessageId;

use super::MailWindow;
use crate::data::EntryKey;
use crate::instance::Request;

/// One entry of a menu of the menu bar.
enum Entry {
    /// A label (`_` marks the mnemonic) and the action it runs.
    Item(&'static str, &'static str),
    Separator,
}

use Entry::{Item, Separator};

/// The menu bar, as KDE apps lay it out.
const MENU_BAR: &[(&str, &[Entry])] = &[
    (
        "_File",
        &[
            Item("_New Message", "katna_mail::Compose"),
            Separator,
            Item("_Quit", "katna_mail::Quit"),
        ],
    ),
    (
        "_Edit",
        &[
            Item("_Undo", "katna_mail::Undo"),
            Separator,
            Item("Select _All", "katna_mail::SelectAll"),
            Item("Select _None", "katna_mail::SelectNone"),
            Separator,
            Item("_Find…", "katna_mail::FocusSearch"),
        ],
    ),
    (
        "_View",
        &[
            Item("Show _Folder List", "katna_mail::ToggleNavigation"),
            Item("_Refresh", "katna_mail::Reload"),
        ],
    ),
    (
        "_Go",
        &[
            Item("_Inbox", "katna_mail::GoToInbox"),
            Item("_Starred", "katna_mail::GoToStarred"),
            Item("S_ent", "katna_mail::GoToSent"),
            Item("_Drafts", "katna_mail::GoToDrafts"),
            Item("_All Mail", "katna_mail::GoToAllMail"),
            Separator,
            Item("_Next Conversation", "katna_mail::SelectNext"),
            Item("_Previous Conversation", "katna_mail::SelectPrevious"),
        ],
    ),
    (
        "_Message",
        &[
            Item("_Open", "katna_mail::OpenMessage"),
            Item("_Reply", "katna_mail::Reply"),
            Item("Reply _All", "katna_mail::ReplyAll"),
            Item("_Forward", "katna_mail::Forward"),
            Separator,
            Item("Arc_hive", "katna_mail::Archive"),
            Item("_Delete", "katna_mail::Delete"),
            Item("Report _Spam", "katna_mail::ReportSpam"),
            Item("_Move To…", "katna_mail::MoveTo"),
            Separator,
            Item("Mark as R_ead", "katna_mail::MarkRead"),
            Item("Mark as _Unread", "katna_mail::MarkUnread"),
            Item("S_tar", "katna_mail::ToggleStar"),
            Item("Mark as Im_portant", "katna_mail::MarkImportant"),
            Item("Mark as _Not Important", "katna_mail::MarkNotImportant"),
        ],
    ),
    (
        "_Settings",
        &[
            Item("_Quick Settings", "katna_mail::ToggleSettings"),
            Item("_Configure Katna Mail…", "katna_mail::OpenSettings"),
        ],
    ),
    (
        "_Help",
        &[Item("_Keyboard Shortcuts", "katna_mail::ShowShortcuts")],
    ),
];

/// The menu bar with the actions this build has, and their shortcuts.
pub fn menu_bar(cx: &App) -> Vec<MenuItem> {
    MENU_BAR
        .iter()
        .filter_map(|(label, entries)| {
            let items: Vec<MenuItem> = entries
                .iter()
                .filter_map(|entry| match entry {
                    Separator => Some(MenuItem::Separator),
                    Item(label, name) => {
                        let action = cx.build_action(name, None).ok()?;
                        Some(MenuItem::action(*label, *name).shortcut(shortcut(&*action, cx)))
                    }
                })
                .collect();
            let items = tidy(items);
            (!items.is_empty()).then(|| MenuItem::submenu(*label, items))
        })
        .collect()
}

/// The menu bar served for the KDE global menu, when there is one.
pub struct MenuBar(pub Menu);

impl Global for MenuBar {}

/// Sends the menu bar again, for new shortcuts or actions.
pub fn refresh_menu_bar(cx: &mut App) {
    let Some(MenuBar(menu)) = cx.try_global::<MenuBar>() else {
        return;
    };
    let menu = menu.clone();
    let items = menu_bar(cx);
    cx.background_spawn(async move {
        if let Err(err) = menu.set_items(items).await {
            tracing::warn!(%err, "cannot update the menu bar");
        }
    })
    .detach();
}

/// Drops separators at either end and next to each other.
fn tidy(items: Vec<MenuItem>) -> Vec<MenuItem> {
    let mut out: Vec<MenuItem> = Vec::with_capacity(items.len());
    for item in items {
        let separator = item == MenuItem::Separator;
        if separator && out.last().is_none_or(|last| *last == MenuItem::Separator) {
            continue;
        }
        out.push(item);
    }
    if out.last() == Some(&MenuItem::Separator) {
        out.pop();
    }
    out
}

/// The shortcut to show for `action`: the last single-keystroke binding
/// with a modifier, else the last single-keystroke one, as dbusmenu wants
/// it (`["Control", "Q"]`).
fn shortcut(action: &dyn Action, cx: &App) -> Vec<String> {
    let keymap = cx.key_bindings();
    let keymap = keymap.borrow();
    let strokes: Vec<_> = keymap
        .bindings_for_action(action)
        .filter_map(|binding| match binding.keystrokes() {
            [stroke] => Some(stroke.inner().clone()),
            _ => None,
        })
        .collect();
    let Some(stroke) = strokes
        .iter()
        .rev()
        .find(|s| s.modifiers.modified())
        .or(strokes.last())
    else {
        return Vec::new();
    };
    let mut keys = Vec::new();
    let modifiers = stroke.modifiers;
    for (on, name) in [
        (modifiers.control, "Control"),
        (modifiers.alt, "Alt"),
        (modifiers.shift, "Shift"),
        (modifiers.platform, "Super"),
    ] {
        if on {
            keys.push(name.to_owned());
        }
    }
    keys.push(key_name(&stroke.key));
    keys
}

/// A GPUI key name as Qt spells it.
fn key_name(key: &str) -> String {
    match key {
        "enter" => "Return".to_owned(),
        "escape" => "Esc".to_owned(),
        "delete" => "Del".to_owned(),
        "pageup" => "PgUp".to_owned(),
        "pagedown" => "PgDown".to_owned(),
        key if key.chars().count() == 1 => key.to_uppercase(),
        key => {
            let mut chars = key.chars();
            chars
                .next()
                .map(|first| first.to_uppercase().chain(chars).collect())
                .unwrap_or_default()
        }
    }
}

impl MailWindow {
    /// Does what the tray, a taskbar action, a notification or the menu
    /// bar asked, and brings the window forward.
    pub fn handle_request(
        &mut self,
        request: Request,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match request {
            Request::Activate => {}
            Request::Menu(name) => {
                self.run_action(&name, window, cx);
                return;
            }
            Request::Action { name, message } => match name.as_str() {
                app_action::OPEN_INBOX => {
                    if !self.run_action("katna_mail::GoToInbox", window, cx) {
                        self.show_inbox(cx);
                    }
                }
                app_action::COMPOSE => {
                    self.run_action("katna_mail::Compose", window, cx);
                }
                app_action::PREFERENCES => {
                    if !self.run_action("katna_mail::OpenSettings", window, cx)
                        && !self.settings_open
                    {
                        self.run_action("katna_mail::ToggleSettings", window, cx);
                    }
                }
                app_action::OPEN_MESSAGE => {
                    if let Some(id) = message {
                        self.show_message(MessageId(id), window, cx);
                    }
                }
                app_action::QUIT => {
                    cx.quit();
                    return;
                }
                other => tracing::warn!(action = other, "unknown app action"),
            },
        }
        window.activate_window();
    }

    /// Runs the action named `name` where the keyboard focus is; `false`
    /// if this build has no such action.
    fn run_action(&mut self, name: &str, window: &mut Window, cx: &mut Context<Self>) -> bool {
        match cx.build_action(name, None) {
            Ok(action) => {
                window.dispatch_action(action, cx);
                true
            }
            Err(_) => false,
        }
    }

    /// Lists the shown account's Inbox, leaving search and the open mail.
    fn show_inbox(&mut self, cx: &mut Context<Self>) {
        let Some((folder, ancestors)) = self.default_folder() else {
            return;
        };
        self.clear_search(cx);
        self.expanded.extend(ancestors);
        self.rebuild_nav();
        self.open_folder(folder, cx);
    }

    /// Opens the conversation of `message` from the Inbox list, or shows the
    /// Inbox when it is not listed there.
    fn show_message(&mut self, message: MessageId, window: &mut Window, cx: &mut Context<Self>) {
        // Mail of another account: show that account.
        if let Some(account) = self
            .mail
            .as_ref()
            .ok()
            .and_then(|m| m.message_account(message))
            && self.shown_account().is_some_and(|shown| shown != account)
        {
            self.set_shown_account(account);
            self.rebuild_nav();
        }
        self.show_inbox(cx);
        let found = self
            .entries
            .iter()
            .position(|entry| entry.latest == message || entry.key == EntryKey::Message(message));
        if let Some(ix) = found {
            self.open(ix, window, cx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tidy_drops_stray_separators() {
        let item = |a: &str| MenuItem::action(a, a);
        let items = vec![
            MenuItem::Separator,
            item("a"),
            MenuItem::Separator,
            MenuItem::Separator,
            item("b"),
            MenuItem::Separator,
        ];
        assert_eq!(tidy(items), [item("a"), MenuItem::Separator, item("b")]);
        assert!(tidy(vec![MenuItem::Separator]).is_empty());
    }

    #[test]
    fn key_names_follow_qt() {
        assert_eq!(key_name("q"), "Q");
        assert_eq!(key_name("f5"), "F5");
        assert_eq!(key_name(","), ",");
        assert_eq!(key_name("#"), "#");
        assert_eq!(key_name("enter"), "Return");
        assert_eq!(key_name("home"), "Home");
    }

    #[test]
    fn menu_bar_names_real_actions() {
        for (_, entries) in MENU_BAR {
            for entry in *entries {
                if let Item(label, name) = entry {
                    assert!(name.starts_with("katna_mail::"), "{name}");
                    assert_eq!(label.matches('_').count(), 1, "{label} has one mnemonic");
                }
            }
        }
    }
}
