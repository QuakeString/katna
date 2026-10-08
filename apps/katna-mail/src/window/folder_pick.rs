// SPDX-License-Identifier: GPL-3.0-or-later

//! The search box on top of "Move to" and, on Gmail, "Label as": typing
//! filters the folders (or labels), Up, Down and Enter pick one, and a
//! name no folder has yet offers "Create", which makes the folder (or
//! label) on the server and moves the mail there (or labels it). "Label
//! as" ticks the labels the mail carries; a click puts one on or takes it
//! off and leaves the menu open. Both the right-click menu's submenus and
//! the toolbar's menus show it.
//!
//! Under Move to's folders, when all the mail is from one sender, a tick
//! "Always move mail from … here" also saves a rule (§9.4) with the click
//! that moves it: from that address, move to that folder.

use std::cell::Cell;
use std::collections::HashMap;

use gpui::{AnyElement, Context, Entity, Focusable, Subscription, Window, div, prelude::*, rgba};
use katna_core::{AccountId, AccountKind};
use katna_i18n::tr;
use katna_store::rules::{Action, Comparator, Condition, Field, Rule};
use katna_store::{FolderId, MessageId};
use katna_ui::px;
use katna_ui::text_input::{Down, Up};
use katna_ui::{InputEvent, TextInput};

use super::context_menu::{menu_row, menu_row_with};
use super::{Act, MailWindow};
use crate::daemon::{self, Command};
use crate::data::{Entry, EntryKey};
use crate::format;
use crate::sidebar::Role;
use crate::theme::{Theme, fade};
use crate::widgets::{Check, checkbox};

/// The search box's line, with the room around the box.
pub(super) const SEARCH_HEIGHT: f32 = 44.0;
/// The faint "No folder called" line.
pub(super) const NONE_HEIGHT: f32 = 24.0;
/// Labels whose state is read for a tick, at most.
const MAX_LABEL_LOOKUPS: usize = 200;
/// The "Always move mail from" line, two lines of text tall.
const ALWAYS_HEIGHT: f32 = 52.0;
/// The divider over it, with its room.
const ALWAYS_RULE: f32 = 13.0;

/// What the list under the search box does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PickMode {
    /// Move to: a click moves the mail.
    Move,
    /// Gmail's Label as: a click ticks or unticks a label.
    Label,
}

/// Which menu holds it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PickFrom {
    /// The list's or the reader's toolbar.
    Toolbar,
    /// The right-click menu's submenu.
    Context,
}

/// The open search over the folders.
pub(super) struct FolderPick {
    pub(super) mode: PickMode,
    pub(super) from: PickFrom,
    account: AccountId,
    /// The lines it acts on.
    keys: Vec<EntryKey>,
    query: Entity<TextInput>,
    /// The line Enter picks, among the folders and Create.
    highlight: usize,
    /// Labels ticked or unticked here, until the store shows it.
    toggled: HashMap<FolderId, bool>,
    /// Move to on mail all from one sender: who, for "Always move mail
    /// from … here".
    always: Option<Sender>,
    /// "Always move mail from … here" is ticked.
    always_on: bool,
    /// Focused, with the theme's accent, once drawn.
    ready: Cell<bool>,
    _subscription: Subscription,
}

/// The one sender of the mail Move to acts on.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Sender {
    /// Their name, else their address.
    name: String,
    address: String,
}

/// The rule "Always move mail from … here" saves: mail of `account` from
/// `sender`'s address goes to `folder`.
fn always_move_rule(sender: &Sender, account: AccountId, folder: FolderId) -> Rule {
    Rule {
        name: sender.name.clone(),
        conditions: vec![Condition {
            field: Field::From,
            comparator: Comparator::Contains,
            value: sender.address.clone(),
        }],
        actions: vec![Action::Move { folder: folder.0 }],
        accounts: vec![account.0],
        ..Rule::default()
    }
}

/// A line under the search box.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PickItem {
    Folder {
        id: FolderId,
        name: String,
        role: Role,
    },
    /// Makes a folder (or label) of this name.
    Create(String),
}

/// The lines for `query` among `folders` (the ones that can be picked,
/// with their names): those whose name holds it, ignoring case, then
/// "Create" when `can_create` and no folder of the account (`taken`, every
/// name) is called that already. Also whether nothing matched.
pub(super) fn pick_items(
    folders: &[(FolderId, String, Role)],
    taken: &[String],
    query: &str,
    can_create: bool,
) -> (Vec<PickItem>, bool) {
    let query = query.trim();
    let wanted = query.to_lowercase();
    let mut items: Vec<PickItem> = folders
        .iter()
        .filter(|(_, name, _)| name.to_lowercase().contains(&wanted))
        .map(|(id, name, role)| PickItem::Folder {
            id: *id,
            name: name.clone(),
            role: *role,
        })
        .collect();
    let none = !query.is_empty() && items.is_empty();
    if can_create && !query.is_empty() && !taken.iter().any(|name| name.to_lowercase() == wanted) {
        items.push(PickItem::Create(query.to_owned()));
    }
    (items, none)
}

impl MailWindow {
    /// Opens the search over the folders of `account` for `keys`.
    pub(super) fn open_folder_pick(
        &mut self,
        mode: PickMode,
        from: PickFrom,
        account: AccountId,
        keys: Vec<EntryKey>,
        cx: &mut Context<Self>,
    ) {
        let query = cx.new(|cx| {
            TextInput::new(
                match mode {
                    PickMode::Move => tr!("menu-move-to-search"),
                    PickMode::Label => tr!("menu-label-as-search"),
                },
                cx,
            )
        });
        let subscription = cx.subscribe(&query, |this, _, event: &InputEvent, cx| match event {
            InputEvent::Changed => {
                if let Some(pick) = &mut this.folder_pick {
                    pick.highlight = 0;
                }
                cx.notify();
            }
            InputEvent::Submit => this.pick_highlighted(cx),
            InputEvent::Cancel => this.cancel_folder_pick(cx),
        });
        let always = match mode {
            PickMode::Move => self.one_sender(account, &keys),
            PickMode::Label => None,
        };
        self.folder_pick = Some(FolderPick {
            mode,
            from,
            account,
            keys,
            query,
            highlight: 0,
            toggled: HashMap::new(),
            always,
            always_on: false,
            ready: Cell::new(false),
            _subscription: subscription,
        });
        cx.notify();
    }

    /// The one sender of all the lines `keys` of `account`, if they have
    /// one that isn't the account's own address.
    fn one_sender(&mut self, account: AccountId, keys: &[EntryKey]) -> Option<Sender> {
        if keys.is_empty() || keys.len() > MAX_LABEL_LOOKUPS {
            return None;
        }
        let reading = self.reader.as_ref().and_then(|r| r.entry());
        let entries: Vec<Entry> = keys
            .iter()
            .map(|key| {
                self.entries
                    .iter()
                    .find(|e| e.key == *key)
                    .copied()
                    .or(reading.filter(|e| e.key == *key))
            })
            .collect::<Option<_>>()?;
        let folder = self.listed_folder();
        let rows = match &mut self.mail {
            Ok(mail) => mail.rows(&entries, folder, self.show_recipients),
            Err(_) => return None,
        };
        let mut found: Option<Sender> = None;
        for row in rows {
            let row = row?;
            let address = row.sender.as_str();
            // Sent mail and drafts show their recipients.
            if row.account != account
                || address.is_empty()
                || row.correspondent.starts_with(&crate::data::to_prefix())
            {
                return None;
            }
            // A conversation others wrote in too isn't one sender's.
            let named = |a: &Option<String>| a.as_deref().map(|a| a.eq_ignore_ascii_case(address));
            if row.people.iter().any(|(_, a)| named(a) == Some(false)) {
                return None;
            }
            match &found {
                Some(sender) if !sender.address.eq_ignore_ascii_case(address) => return None,
                Some(_) => {}
                None => {
                    let name = row
                        .people
                        .iter()
                        .find(|(_, a)| named(a) == Some(true))
                        .map(|(name, _)| name.trim().to_owned())
                        .or_else(|| (row.count == 1).then(|| row.correspondent.trim().to_owned()))
                        .filter(|name| !name.is_empty())
                        .unwrap_or_else(|| address.to_owned());
                    found = Some(Sender {
                        name,
                        address: address.to_owned(),
                    });
                }
            }
        }
        // The account's own mail never meets a rule.
        let sender = found?;
        let own = self
            .accounts
            .iter()
            .any(|a| a.id == account && a.address.eq_ignore_ascii_case(&sender.address));
        (!own).then_some(sender)
    }

    /// The open search, if `from` holds it.
    pub(super) fn folder_pick_in(&self, from: PickFrom, mode: PickMode) -> Option<&FolderPick> {
        self.folder_pick
            .as_ref()
            .filter(|p| p.from == from && p.mode == mode)
    }

    /// Opens the search for the toolbar's Move to or Label as when that
    /// menu is open, and drops it when the menu that held it closed.
    pub(super) fn sync_folder_pick(&mut self, cx: &mut Context<Self>) {
        let toolbar = match self.menu {
            Some(super::Menu::MoveTo) => Some(PickMode::Move),
            Some(super::Menu::LabelAs) => Some(PickMode::Label),
            _ => None,
        };
        let held = match self.folder_pick.as_ref().map(|p| (p.from, p.mode)) {
            Some((PickFrom::Toolbar, mode)) => toolbar == Some(mode),
            Some((PickFrom::Context, _)) => self.open_context_menu_ref().is_some(),
            None => false,
        };
        if !held {
            self.folder_pick = None;
            if let (Some(mode), Some(account)) = (toolbar, self.account()) {
                let keys = self.target_keys();
                self.open_folder_pick(mode, PickFrom::Toolbar, account, keys, cx);
            }
        }
    }

    /// Gives a search just opened the keys and the theme's accent.
    pub(super) fn ready_folder_pick(
        &mut self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.sync_folder_pick(cx);
        let Some(pick) = &self.folder_pick else {
            return;
        };
        if pick.ready.replace(true) {
            return;
        }
        let accent = rgba(th.accent).into();
        let query = pick.query.clone();
        query.update(cx, |input, _| input.set_accent(accent));
        window.focus(&query.focus_handle(cx), cx);
    }

    /// Whether the search box has the keys, so Left moves in the text.
    pub(super) fn folder_pick_typing(&self, window: &Window, cx: &gpui::App) -> bool {
        self.folder_pick
            .as_ref()
            .is_some_and(|p| p.query.focus_handle(cx).is_focused(window))
    }

    /// Escape in the search box: back out of the submenu, or close the
    /// toolbar's menu.
    fn cancel_folder_pick(&mut self, cx: &mut Context<Self>) {
        match self.folder_pick.take().map(|p| p.from) {
            Some(PickFrom::Context) => {
                self.context_menu_back(cx);
            }
            Some(PickFrom::Toolbar) => self.menu = None,
            None => {}
        }
        cx.notify();
    }

    /// Closes the search and the menu that holds it.
    fn close_folder_pick(&mut self, cx: &mut Context<Self>) {
        match self.folder_pick.take().map(|p| p.from) {
            Some(PickFrom::Context) => {
                self.take_context_menu();
            }
            Some(PickFrom::Toolbar) => self.menu = None,
            None => {}
        }
        cx.notify();
    }

    /// The folders the search goes through, with their names, and every
    /// name the account has.
    fn pick_folders(&self, pick: &FolderPick) -> (Vec<(FolderId, String, Role)>, Vec<String>) {
        let all = self.tree.folders_of(pick.account);
        let mut taken: Vec<String> = all.iter().map(|(_, name, _)| name.clone()).collect();
        taken.extend(
            all.iter()
                .filter_map(|(id, ..)| self.tree.node(*id).map(|n| n.path.clone())),
        );
        let folders = match pick.mode {
            PickMode::Move => {
                let current = self.listed_folder();
                all.into_iter()
                    .filter(|(id, _, role)| {
                        Some(*id) != current
                            && !matches!(
                                role,
                                Role::Drafts | Role::Sent | Role::Flagged | Role::Snoozed
                            )
                    })
                    .collect()
            }
            PickMode::Label => self
                .tree
                .nest_targets(pick.account)
                .into_iter()
                .map(|(id, path)| (id, path, Role::Other))
                .collect(),
        };
        (folders, taken)
    }

    /// What the search shows now, and the typed text when nothing matched.
    fn pick_view(&self, pick: &FolderPick, cx: &Context<Self>) -> (Vec<PickItem>, Option<String>) {
        let (folders, taken) = self.pick_folders(pick);
        let query = pick.query.read(cx).text().to_owned();
        // Made on the server, so only on accounts that have one.
        let can_create = self
            .accounts
            .iter()
            .any(|a| a.id == pick.account && a.kind == AccountKind::Imap);
        let (items, none) = pick_items(&folders, &taken, &query, can_create);
        (items, none.then(|| query.trim().to_owned()))
    }

    /// Up (-1) or Down (1) through the lines.
    fn pick_step(&mut self, step: isize, cx: &mut Context<Self>) {
        let Some(pick) = &self.folder_pick else {
            return;
        };
        let count = self.pick_view(pick, cx).0.len();
        let Some(pick) = &mut self.folder_pick else {
            return;
        };
        if count > 0 {
            pick.highlight = (pick.highlight as isize + step).rem_euclid(count as isize) as usize;
        }
        cx.notify();
    }

    /// Enter: picks the highlighted line.
    fn pick_highlighted(&mut self, cx: &mut Context<Self>) {
        let Some(pick) = &self.folder_pick else {
            return;
        };
        let items = self.pick_view(pick, cx).0;
        if let Some(item) = items.get(pick.highlight.min(items.len().saturating_sub(1))) {
            self.pick_item(item.clone(), cx);
        }
    }

    /// The messages of the lines the search acts on.
    fn pick_messages(&self, keys: &[EntryKey]) -> Vec<MessageId> {
        match &self.mail {
            Ok(mail) => keys.iter().flat_map(|k| mail.entry_messages(*k)).collect(),
            Err(_) => Vec::new(),
        }
    }

    /// Whether the lines carry `label`: all of them, some or none.
    fn label_state(&self, pick: &FolderPick, label: FolderId) -> Check {
        if let Some(on) = pick.toggled.get(&label) {
            return Check::from(*on);
        }
        let Ok(mail) = &self.mail else {
            return Check::Off;
        };
        let keys = &pick.keys[..pick.keys.len().min(MAX_LABEL_LOOKUPS)];
        let carrying = keys
            .iter()
            .filter(|k| {
                mail.entry_messages(**k)
                    .into_iter()
                    .any(|id| mail.message_folders(id).contains(&label))
            })
            .count();
        match carrying {
            0 => Check::Off,
            n if n == keys.len() => Check::On,
            _ => Check::Partial,
        }
    }

    /// Does what line `item` says.
    fn pick_item(&mut self, item: PickItem, cx: &mut Context<Self>) {
        let Some(pick) = &self.folder_pick else {
            return;
        };
        let (mode, account, keys) = (pick.mode, pick.account, pick.keys.clone());
        let always = pick.always.clone().filter(|_| pick.always_on);
        match (mode, item) {
            (PickMode::Move, PickItem::Folder { id, role, .. }) => {
                self.close_folder_pick(cx);
                self.act(Act::MoveTo(id), keys, cx);
                // Rules look at inbox mail: one moving it there does nothing.
                if let Some(sender) = always.filter(|_| role != Role::Inbox) {
                    self.save_always_move(&sender, account, id, cx);
                }
            }
            (PickMode::Label, PickItem::Folder { id, name, .. }) => {
                let on = self.label_state(pick, id) != Check::On;
                self.toggle_label(keys, id, &name, on, cx);
                if let Some(pick) = &mut self.folder_pick {
                    pick.toggled.insert(id, on);
                }
            }
            (mode, PickItem::Create(name)) => {
                self.close_folder_pick(cx);
                self.create_and_pick(mode, account, keys, name, always, cx);
            }
        }
    }

    /// Puts `label` on the lines `keys`, or takes it off, with an Undo.
    pub(super) fn toggle_label(
        &mut self,
        keys: Vec<EntryKey>,
        label: FolderId,
        name: &str,
        on: bool,
        cx: &mut Context<Self>,
    ) {
        let ids = self.pick_messages(&keys);
        if ids.is_empty() {
            return;
        }
        let (command, undo, text) = if on {
            (
                Command::Labels(ids.clone(), vec![label], Vec::new()),
                Command::Labels(ids, Vec::new(), vec![label]),
                tr!("toast-label-added", label = name),
            )
        } else {
            (
                Command::Labels(ids.clone(), Vec::new(), vec![label]),
                Command::Labels(ids, vec![label], Vec::new()),
                tr!("toast-label-removed", label = name),
            )
        };
        self.send(command, Some(text), Some(undo), false, cx);
        cx.notify();
    }

    /// Makes the folder (or label) `name` at the top of `account`, then
    /// moves the lines there (or labels them).
    fn create_and_pick(
        &mut self,
        mode: PickMode,
        account: AccountId,
        keys: Vec<EntryKey>,
        name: String,
        always: Option<Sender>,
        cx: &mut Context<Self>,
    ) {
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let created = name.clone();
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::create_folder(&connection, account.0, &created, None).await
                })
                .await;
            this.update(cx, |this, cx| match result {
                Ok(id) => {
                    let id = FolderId(id);
                    this.refresh(false, cx);
                    match mode {
                        PickMode::Move => {
                            this.act(Act::MoveTo(id), keys, cx);
                            if let Some(sender) = &always {
                                this.save_always_move(sender, account, id, cx);
                            }
                        }
                        PickMode::Label => this.toggle_label(keys, id, &name, true, cx),
                    }
                }
                Err(err) => this.show_snackbar(format::sentence(&err), None, cx),
            })
            .ok();
        })
        .detach();
    }

    /// Saves the rule "Always move mail from `sender` here": mail of
    /// `account` from them goes to `folder`. Only a failure is told: the
    /// move's own message, with its Undo, stays.
    fn save_always_move(
        &mut self,
        sender: &Sender,
        account: AccountId,
        folder: FolderId,
        cx: &mut Context<Self>,
    ) {
        let rule = always_move_rule(sender, account, folder);
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::rules::save(&connection, &rule).await
                })
                .await;
            this.update(cx, |this, cx| match result {
                Ok(_) => this.load_rules(cx),
                Err(err) => {
                    let error = format::sentence(&err);
                    this.show_snackbar(tr!("toast-always-move-failed", error = error), None, cx);
                }
            })
            .ok();
        })
        .detach();
    }

    /// Under Move to's folders, after a divider: "Always move mail from …
    /// here", when the mail has one sender. With its height.
    pub(super) fn render_always_move(
        &self,
        pick: &FolderPick,
        th: &Theme,
        cx: &Context<Self>,
    ) -> Option<(AnyElement, f32)> {
        let sender = pick.always.as_ref()?;
        let row = div()
            .id("pick-always")
            .flex_none()
            .h(px(ALWAYS_HEIGHT))
            .pl(px(16.0))
            .pr(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(16.0))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .child(
                div()
                    .flex_none()
                    .size(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(checkbox("pick-always-box", Check::from(pick.always_on), th)),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .text_size(px(13.0))
                    .line_height(px(16.0))
                    .line_clamp(2)
                    .child(tr!("menu-always-move", name = sender.name.as_str())),
            )
            .on_click(cx.listener(|this, _, _, cx| {
                cx.stop_propagation();
                if let Some(pick) = &mut this.folder_pick {
                    pick.always_on = !pick.always_on;
                }
                cx.notify();
            }));
        let line = div()
            .flex_none()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex_none()
                    .my(px((ALWAYS_RULE - 1.0) / 2.0))
                    .h(px(1.0))
                    .bg(rgba(th.divider)),
            )
            .child(row);
        Some((line.into_any_element(), ALWAYS_RULE + ALWAYS_HEIGHT))
    }

    /// The search box and the lines under it, each with its height; lines
    /// are `rh` tall.
    pub(super) fn render_folder_pick(
        &self,
        pick: &FolderPick,
        rh: f32,
        th: &Theme,
        cx: &Context<Self>,
    ) -> Vec<(AnyElement, f32)> {
        let (items, none) = self.pick_view(pick, cx);
        let highlight = pick.highlight.min(items.len().saturating_sub(1));
        let mut out = Vec::new();
        let search = div()
            .id("pick-search")
            .flex_none()
            .h(px(SEARCH_HEIGHT))
            .px(px(8.0))
            .py(px(4.0))
            // The field leaves Up and Down to whatever holds it.
            .on_action(cx.listener(|this, _: &Up, _, cx| this.pick_step(-1, cx)))
            .on_action(cx.listener(|this, _: &Down, _, cx| this.pick_step(1, cx)))
            .child(
                div()
                    .size_full()
                    .px(px(12.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .rounded_full()
                    .bg(rgba(fade(th.text, 0.06)))
                    .cursor_text()
                    .child(crate::widgets::icon("search", th.text_dim, 16.0))
                    .child(div().flex_1().min_w_0().child(pick.query.clone())),
            );
        out.push((search.into_any_element(), SEARCH_HEIGHT));
        if let Some(name) = none {
            out.push((
                div()
                    .flex_none()
                    .h(px(NONE_HEIGHT))
                    .px(px(16.0))
                    .flex()
                    .items_center()
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_faint))
                    .child(div().min_w_0().truncate().child(match pick.mode {
                        PickMode::Move => tr!("menu-no-folder", name = name.as_str()),
                        PickMode::Label => tr!("menu-no-label", name = name.as_str()),
                    }))
                    .into_any_element(),
                NONE_HEIGHT,
            ));
        }
        for (ix, item) in items.into_iter().enumerate() {
            let lit = ix == highlight;
            let row = match &item {
                PickItem::Folder { id, name, role } => match pick.mode {
                    PickMode::Move => menu_row(
                        ("pick-folder", id.0 as usize),
                        super::nav::role_icon(*role),
                        name.clone().into(),
                        th,
                        rh,
                    ),
                    PickMode::Label => menu_row_with(
                        ("pick-label", id.0 as usize),
                        checkbox(
                            ("pick-label-box", id.0 as usize),
                            self.label_state(pick, *id),
                            th,
                        ),
                        name.clone().into(),
                        th,
                        rh,
                    ),
                },
                PickItem::Create(name) => menu_row(
                    "pick-create",
                    "folder-add",
                    tr!("menu-create-folder", name = name.as_str()).into(),
                    th,
                    rh,
                ),
            };
            let row = row
                .when(lit, |d| d.bg(rgba(th.hover)))
                .on_click(cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
                    this.pick_item(item.clone(), cx);
                }));
            out.push((row.into_any_element(), rh));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn folders() -> Vec<(FolderId, String, Role)> {
        vec![
            (FolderId(1), "Archive".to_owned(), Role::Archive),
            (FolderId(2), "Spam".to_owned(), Role::Junk),
            (FolderId(3), "Projects".to_owned(), Role::Other),
            (FolderId(4), "Receipts".to_owned(), Role::Other),
        ]
    }

    fn names(items: &[PickItem]) -> Vec<String> {
        items
            .iter()
            .map(|item| match item {
                PickItem::Folder { name, .. } => name.clone(),
                PickItem::Create(name) => format!("+{name}"),
            })
            .collect()
    }

    #[test]
    fn typing_filters_and_offers_a_new_folder() {
        let taken = vec![
            "Inbox".to_owned(),
            "Archive".to_owned(),
            "Projects".to_owned(),
        ];
        let (items, none) = pick_items(&folders(), &taken, "", true);
        assert_eq!(names(&items), ["Archive", "Spam", "Projects", "Receipts"]);
        assert!(!none);

        let (items, none) = pick_items(&folders(), &taken, "  REC ", true);
        assert_eq!(names(&items), ["Receipts", "+REC"]);
        assert!(!none);

        // A name taken, whatever its case, is not offered again.
        let (items, _) = pick_items(&folders(), &taken, "projects", true);
        assert_eq!(names(&items), ["Projects"]);
        let (items, _) = pick_items(&folders(), &taken, "inbox", true);
        assert!(items.is_empty(), "the inbox is there, though not listed");

        let (items, none) = pick_items(&folders(), &taken, "Travel", true);
        assert_eq!(names(&items), ["+Travel"]);
        assert!(none);
        let (items, none) = pick_items(&folders(), &taken, "Travel", false);
        assert!(items.is_empty() && none, "nowhere to make it");
    }

    #[test]
    fn always_move_makes_a_rule_named_after_the_sender() {
        let sender = Sender {
            name: "Bob Example".to_owned(),
            address: "bob@example.com".to_owned(),
        };
        let rule = always_move_rule(&sender, AccountId(3), FolderId(9));
        assert_eq!(rule.name, "Bob Example");
        assert_eq!(rule.id, 0, "a new rule");
        assert!(rule.enabled && !rule.stop);
        assert_eq!(rule.accounts, [3]);
        assert_eq!(
            rule.conditions,
            [Condition {
                field: Field::From,
                comparator: Comparator::Contains,
                value: "bob@example.com".to_owned(),
            }]
        );
        assert_eq!(rule.actions, [Action::Move { folder: 9 }]);
        assert!(rule.validate().is_ok());
    }
}
