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
use katna_i18n::tr;
use katna_platform::dbusmenu::{Menu, MenuItem};
use katna_store::MessageId;

use super::compose::Kind;
use super::{MailWindow, RailApp};
use crate::data::EntryKey;
use crate::instance::Request;

/// One entry of a menu of the menu bar.
enum Entry {
    /// The id of its label's message (`_` marks the mnemonic) and the
    /// action it runs.
    Item(&'static str, &'static str),
    Separator,
}

use Entry::{Item, Separator};

/// The menu bar, as KDE apps lay it out: each menu's message id and its
/// entries.
const MENU_BAR: &[(&str, &[Entry])] = &[
    (
        "desktop-menu-file",
        &[
            Item("desktop-menu-new-message", "katna_mail::Compose"),
            Separator,
            Item("desktop-menu-quit", "katna_mail::Quit"),
        ],
    ),
    (
        "desktop-menu-edit",
        &[
            Item("desktop-menu-undo", "katna_mail::Undo"),
            Separator,
            Item("desktop-menu-select-all", "katna_mail::SelectAll"),
            Item("desktop-menu-select-none", "katna_mail::SelectNone"),
            Separator,
            Item("desktop-menu-find", "katna_mail::FocusSearch"),
        ],
    ),
    (
        "desktop-menu-view",
        &[
            Item("desktop-menu-folder-list", "katna_mail::ToggleNavigation"),
            Item("desktop-menu-refresh", "katna_mail::Reload"),
        ],
    ),
    (
        "desktop-menu-go",
        &[
            Item("desktop-menu-inbox", "katna_mail::GoToInbox"),
            Item("desktop-menu-starred", "katna_mail::GoToStarred"),
            Item("desktop-menu-sent", "katna_mail::GoToSent"),
            Item("desktop-menu-drafts", "katna_mail::GoToDrafts"),
            Item("desktop-menu-all-mail", "katna_mail::GoToAllMail"),
            Separator,
            Item("desktop-menu-page-mail", "katna_mail::ShowMail"),
            Item("desktop-menu-page-calendar", "katna_mail::ShowCalendar"),
            Item("desktop-menu-page-contacts", "katna_mail::ShowContacts"),
            Item("desktop-menu-page-tasks", "katna_mail::ShowTasks"),
            Item("desktop-menu-page-notes", "katna_mail::ShowNotes"),
            Item("desktop-menu-page-files", "katna_mail::ShowFiles"),
            Separator,
            Item("desktop-menu-next", "katna_mail::SelectNext"),
            Item("desktop-menu-previous", "katna_mail::SelectPrevious"),
        ],
    ),
    (
        "desktop-menu-message",
        &[
            Item("desktop-menu-open", "katna_mail::OpenMessage"),
            Item("desktop-menu-reply", "katna_mail::Reply"),
            Item("desktop-menu-reply-all", "katna_mail::ReplyAll"),
            Item("desktop-menu-forward", "katna_mail::Forward"),
            Separator,
            Item("desktop-menu-archive", "katna_mail::Archive"),
            Item("desktop-menu-delete", "katna_mail::Delete"),
            Item("desktop-menu-spam", "katna_mail::ReportSpam"),
            Item("desktop-menu-move-to", "katna_mail::MoveTo"),
            Separator,
            Item("desktop-menu-mark-read", "katna_mail::MarkRead"),
            Item("desktop-menu-mark-unread", "katna_mail::MarkUnread"),
            Item("desktop-menu-star", "katna_mail::ToggleStar"),
            Item("desktop-menu-important", "katna_mail::MarkImportant"),
            Item("desktop-menu-not-important", "katna_mail::MarkNotImportant"),
        ],
    ),
    (
        "desktop-menu-settings",
        &[
            Item("desktop-menu-quick-settings", "katna_mail::ToggleSettings"),
            Item("desktop-menu-configure", "katna_mail::OpenSettings"),
        ],
    ),
    (
        "desktop-menu-help",
        &[
            Item("desktop-menu-check-updates", "katna_mail::CheckForUpdates"),
            Item("desktop-menu-whats-new", "katna_mail::ShowWhatsNew"),
            Item("desktop-menu-shortcuts", "katna_mail::ShowShortcuts"),
            Item("desktop-menu-send-feedback", "katna_mail::SendFeedback"),
            Separator,
            Item("desktop-menu-about", "katna_mail::ShowAbout"),
        ],
    ),
];

/// The menu bar with the actions this build has, and their shortcuts.
pub fn menu_bar(cx: &App) -> Vec<MenuItem> {
    let side_panel = cx.try_global::<SidePanelMenu>().is_some_and(|page| page.0);
    // The pages of the apps turned off in Settings > Apps.
    let off = cx
        .try_global::<super::apps_off::OffApps>()
        .map(|off| off.0.clone())
        .unwrap_or_default();
    MENU_BAR
        .iter()
        .filter_map(|(label, entries)| {
            let items: Vec<MenuItem> = entries
                .iter()
                .filter_map(|entry| match entry {
                    Separator => Some(MenuItem::Separator),
                    Item(_, name) if off.contains(name) => None,
                    Item(label, name) => {
                        let action = cx.build_action(name, None).ok()?;
                        let label = if *label == "desktop-menu-folder-list" && side_panel {
                            "desktop-menu-side-panel"
                        } else {
                            label
                        };
                        Some(MenuItem::action(tr!(label), *name).shortcut(shortcut(&*action, cx)))
                    }
                })
                .collect();
            let items = tidy(items);
            (!items.is_empty()).then(|| MenuItem::submenu(tr!(*label), items))
        })
        .collect()
}

/// The menu bar served for the KDE global menu, when there is one.
pub struct MenuBar(pub Menu);

impl Global for MenuBar {}

/// Whether the window shows a page other than Mail, whose menu button folds
/// a side panel rather than the folder list.
#[derive(Default)]
pub struct SidePanelMenu(pub bool);

impl Global for SidePanelMenu {}

/// Names the View menu's fold item for the page the window shows.
pub fn menu_page_changed(side_panel: bool, cx: &mut App) {
    if cx
        .try_global::<SidePanelMenu>()
        .is_some_and(|page| page.0 == side_panel)
    {
        return;
    }
    cx.set_global(SidePanelMenu(side_panel));
    refresh_menu_bar(cx);
}

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
            Request::Mailto(uri) => {
                if let Some(mail) = crate::mailto::Mailto::parse(&uri) {
                    self.open_mailto(mail, window, cx);
                }
            }
            Request::Menu(name) => {
                // The menu bar also serves the conversation windows: what
                // the one in front can do happens there, the rest here.
                let main = window.window_handle().downcast::<MailWindow>();
                if let Some((other, main)) = cx
                    .active_window()
                    .filter(|w| *w != window.window_handle())
                    .and_then(|w| w.downcast::<MailWindow>())
                    .zip(main)
                {
                    cx.defer(move |cx| {
                        let ran = other.update(cx, |view, window, cx| {
                            let available = cx.build_action(&name, None).is_ok_and(|action| {
                                window.is_action_available(&*action, cx)
                                    || window.is_action_available_in(&*action, &view.window_focus)
                            });
                            available && view.run_action(&name, window, cx)
                        });
                        if !matches!(ran, Ok(true)) {
                            let _ = main.update(cx, |view, window, cx| {
                                view.run_action(&name, window, cx);
                            });
                        }
                    });
                    return;
                }
                self.run_action(&name, window, cx);
                return;
            }
            Request::Search(text) => self.search_for(text, window, cx),
            // The card opens over whatever is in front; the mail window
            // stays where it is.
            Request::Capture(param) => {
                super::capture::open(&param, cx);
                return;
            }
            Request::Attach { from, paths } => self.open_with_files(from, paths, window, cx),
            // The app may reopen on another page: the mail is on Mail.
            Request::ShowMessage(id) => {
                self.show_page(RailApp::Mail, window, cx);
                self.show_message(MessageId(id), window, cx);
            }
            // `calendar:<day>` shows that day on the Calendar page (with
            // `:new`, a new event on it); `tasks:<id>` opens that task, and
            // `notes:<id>` that note.
            Request::Page(page) => {
                if page == "gallery" {
                    self.open_gallery(window, cx);
                    return;
                }
                let (name, detail, new_event) = app_action::page_parts(&page);
                let Some(app) = RailApp::from_key(name) else {
                    tracing::warn!(page, "unknown page");
                    return;
                };
                // A turned-off app's launcher action or old link says so.
                if let Some(kind) = app.kind().filter(|_| !self.app_on(app)) {
                    self.say_app_off(kind, cx);
                    return;
                }
                self.show_page(app, window, cx);
                match app {
                    RailApp::Calendar => {
                        if let Some(day) =
                            detail.and_then(|day| day.parse::<jiff::civil::Date>().ok())
                        {
                            self.open_calendar_on(day, cx);
                            if new_event {
                                // The page reads its calendars in the
                                // background; the new event needs them now.
                                if self.calendar.calendars.is_empty() {
                                    self.calendar.calendars =
                                        std::rc::Rc::new(super::calendar::read_calendars(
                                            &self.paths,
                                            &self.calendar_left_out(),
                                        ));
                                }
                                self.create_event_key(window, cx);
                                // Typed in the New event window.
                                if let Some(title) = app_action::new_event_title(&page) {
                                    self.set_draft_title(title, window, cx);
                                }
                            }
                        }
                    }
                    RailApp::Tasks => {
                        if let Some(id) = detail.and_then(|id| id.parse::<i64>().ok()) {
                            self.task_open_when_read(id, window, cx);
                        }
                    }
                    // `notes:<id>`: a note's reminder was clicked.
                    RailApp::Notes => {
                        if let Some(id) = detail.and_then(|id| id.parse::<i64>().ok()) {
                            self.open_note_by_id(id, window, cx);
                        }
                    }
                    // From a notification that something needs the user:
                    // `mail:outbox` opens the Outbox, `mail:fix-<id>` the
                    // fix of that account's problem.
                    RailApp::Mail => {
                        if detail == Some("outbox") {
                            self.leave_settings(window, cx);
                            self.open_outbox(cx);
                        } else if let Some(id) = detail.and_then(app_action::fix_account) {
                            self.leave_settings(window, cx);
                            self.fix_problem_when_known(id, window, cx);
                        }
                    }
                    _ => {}
                }
            }
            Request::Action {
                name,
                message,
                text,
            } => match name.as_str() {
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
                // From a notification: in a window of its own, in front
                // (the click's activation token raises it); this one stays
                // behind, on whatever page it shows.
                app_action::OPEN_MESSAGE | app_action::REPLY_ALL | app_action::REPLY => {
                    if let Some(id) = message {
                        let reply = match name.as_str() {
                            app_action::REPLY_ALL => Some(Kind::ReplyAll),
                            app_action::REPLY => Some(Kind::Reply),
                            _ => None,
                        };
                        self.message_in_window(MessageId(id), reply, text, cx);
                        return;
                    }
                }
                app_action::INSTALL_UPDATE => self.show_update(window, cx),
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
    /// if this build has no such action. Focus left on something no longer
    /// drawn (the list, once Settings or a conversation fills the page)
    /// would send it nowhere, so the window takes the focus first.
    pub(super) fn run_action(
        &mut self,
        name: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        match cx.build_action(name, None) {
            Ok(action) => {
                if !window.is_action_available(&*action, cx)
                    && window.is_action_available_in(&*action, &self.window_focus)
                {
                    window.focus(&self.window_focus, cx);
                }
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

    /// Opens the conversation of `message` from the Inbox list, looking in
    /// every inbox tab; when it is not listed there, shows the Inbox with
    /// the conversation open by itself and returns `false`.
    pub(super) fn show_message(
        &mut self,
        message: MessageId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
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
        let thread = self
            .mail
            .as_ref()
            .ok()
            .and_then(|m| m.message_thread(message));
        let listed = |entries: &[crate::data::Entry]| {
            entries.iter().position(|entry| {
                entry.latest == message
                    || entry.key == EntryKey::Message(message)
                    || thread.is_some_and(|t| entry.key == EntryKey::Thread(t))
            })
        };
        let mut found = listed(&self.entries);
        // New mail is usually in the first tab, but a filter or the user
        // may have put it in another one.
        let (first, tabs) = (self.tab, self.tabs.len());
        for tab in (0..tabs).filter(|&t| t != first) {
            if found.is_some() {
                break;
            }
            self.open_tab(tab, cx);
            found = listed(&self.entries);
        }
        match found {
            Some(ix) => {
                self.open(ix, window, cx);
                true
            }
            None => {
                if tabs > 0 {
                    self.open_tab(first, cx);
                }
                // Not in the list (another view, or a line the list folds
                // differently): open it in the reader by itself anyway.
                let entry = match thread {
                    Some(thread) => crate::data::Entry {
                        key: EntryKey::Thread(thread),
                        latest: message,
                    },
                    None => crate::data::Entry::message(message),
                };
                tracing::debug!(
                    message = message.0,
                    "opening a message the Inbox does not list"
                );
                self.open_contact_entry(entry, window, cx);
                false
            }
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
                    let label = tr!(*label);
                    assert!(name.starts_with("katna_mail::"), "{name}");
                    assert_eq!(label.matches('_').count(), 1, "{label} has one mnemonic");
                }
            }
        }
    }
}
