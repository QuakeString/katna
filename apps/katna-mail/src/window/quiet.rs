// SPDX-License-Identifier: GPL-3.0-or-later

//! Folder bells and mutes (`docs/ARCHITECTURE.md` §15.1.1): what notifies
//! and counts on the taskbar and tray, set where the folder, tab or
//! account shows. The list's toolbar has a bell for the open folder or
//! inbox tab; a folder's or an account's right-click menu has Mute (or
//! Unmute, or Notify for new mail), which opens the same choices where it
//! was clicked: for an hour, until tomorrow morning, or until turned back
//! on. Every change has Undo.

use gpui::{
    AnyElement, Context, MouseButton, Pixels, Point, SharedString, anchored, deferred, div,
    prelude::*,
};
use jiff::Timestamp;
use katna_core::{AccountId, MailCategory};
use katna_i18n::tr;
use katna_store::{Bell, FolderId, MessageId, MuteTarget, ThreadId};

use crate::data::EntryKey;
use katna_ui::px;

use super::{Listing, MailWindow, Menu};
use crate::daemon::{Command, Muted};
use crate::sidebar::Role;
use crate::theme::Theme;
use crate::widgets::{icon_button, menu, menu_item_icon, tip};

/// What a bell or mute is set on, from the app's side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Quiet {
    /// A folder; for an inbox, the tab with these categories (empty: the
    /// inbox as a whole, which is its Primary tab).
    Folder {
        folder: FolderId,
        categories: Vec<MailCategory>,
    },
    Account(AccountId),
}

/// How long a mute lasts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum For {
    Hour,
    /// Until 8 in the morning.
    Tomorrow,
    /// Until turned back on.
    Always,
}

/// Where a folder, tab or account stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct QuietState {
    /// Its bell is on (an account always is).
    pub bell: bool,
    /// Muted for a while: until then; `Some(None)` until unmuted.
    pub muted: Option<Option<i64>>,
    /// The bell differs from a folder's default.
    pub changed: bool,
}

impl QuietState {
    /// Its new mail notifies.
    pub fn rings(self) -> bool {
        self.bell && self.muted.is_none()
    }
}

/// The right-click menu's mute choices, opened where Mute was clicked.
pub(super) struct QuietMenu {
    target: Quiet,
    at: Point<Pixels>,
}

impl MailWindow {
    /// The open folder, or inbox tab, as something to mute.
    pub(super) fn open_quiet_target(&self) -> Option<Quiet> {
        let Some(Listing::Folder(folder)) = &self.listing else {
            return None;
        };
        let inbox = self
            .tree
            .node(*folder)
            .is_some_and(|n| n.role == Role::Inbox);
        let categories = if inbox {
            self.tabs
                .get(self.tab)
                .map(|t| t.categories.clone())
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        Some(Quiet::Folder {
            folder: *folder,
            categories,
        })
    }

    /// Where `target` stands now.
    pub(super) fn quiet_state(&self, target: &Quiet) -> QuietState {
        match target {
            Quiet::Account(account) => QuietState {
                bell: true,
                muted: self
                    .alerts
                    .mute(&MuteTarget::Account(*account))
                    .map(|m| m.until),
                changed: false,
            },
            Quiet::Folder { folder, categories } => {
                let role = self
                    .tree
                    .node(*folder)
                    .is_some_and(|n| n.role == Role::Inbox)
                    .then_some(katna_store::FolderRole::Inbox.as_str());
                let bells: Vec<Bell> = if categories.is_empty() {
                    vec![self.alerts.bell(*folder, role, None)]
                } else {
                    categories
                        .iter()
                        .map(|c| self.alerts.bell(*folder, role, Some(*c)))
                        .collect()
                };
                let defaults: Vec<Bell> = if categories.is_empty() {
                    vec![Bell::default_for(role, None)]
                } else {
                    categories
                        .iter()
                        .map(|c| Bell::default_for(role, Some(*c)))
                        .collect()
                };
                QuietState {
                    bell: bells.iter().any(|b| b.notify),
                    muted: self
                        .alerts
                        .mute(&MuteTarget::Folder(*folder))
                        .map(|m| m.until),
                    changed: bells != defaults,
                }
            }
        }
    }

    /// Whether `target` may be muted for a while: an account, a folder,
    /// or an inbox's Primary tab (a timed mute quiets the whole inbox).
    fn quiet_timed(&self, target: &Quiet) -> bool {
        match target {
            Quiet::Account(_) => true,
            Quiet::Folder { categories, .. } => {
                categories.is_empty() || categories.contains(&MailCategory::Primary)
            }
        }
    }

    /// What `target` is called in a note: a folder's or tab's name, or
    /// the account's.
    fn quiet_name(&self, target: &Quiet) -> String {
        match target {
            Quiet::Account(account) => self
                .accounts
                .iter()
                .find(|a| a.id == *account)
                .map(|a| {
                    if a.display_name.is_empty() {
                        a.address.clone()
                    } else {
                        a.display_name.clone()
                    }
                })
                .unwrap_or_default(),
            Quiet::Folder { folder, categories } => {
                let tab = (!categories.is_empty())
                    .then(|| self.tabs.iter().find(|t| &t.categories == categories))
                    .flatten();
                match tab {
                    Some(tab) if self.tabs.len() > 1 => tab.label().to_string(),
                    _ => self
                        .tree
                        .node(*folder)
                        .map(|n| n.label())
                        .unwrap_or_default(),
                }
            }
        }
    }

    /// The commands that set `target` to `state`, as far as a bell and a
    /// mute go.
    fn quiet_commands(&self, target: &Quiet, state: QuietState) -> Vec<Command> {
        let mut commands = Vec::new();
        match target {
            Quiet::Account(account) => commands.push(match state.muted {
                Some(until) => Command::Mute(Muted::Account(*account), until.unwrap_or(0)),
                None => Command::Unmute(Muted::Account(*account)),
            }),
            Quiet::Folder { folder, categories } => {
                let bell = if state.bell { Bell::ON } else { Bell::OFF };
                if categories.is_empty() {
                    commands.push(Command::SetBell(*folder, None, bell));
                } else {
                    for category in categories {
                        commands.push(Command::SetBell(*folder, Some(*category), bell));
                    }
                }
                commands.push(match state.muted {
                    Some(until) => Command::Mute(Muted::Folder(*folder), until.unwrap_or(0)),
                    None => Command::Unmute(Muted::Folder(*folder)),
                });
            }
        }
        commands
    }

    /// Puts `target` in `state`, with a note that says `done` and undoes
    /// it.
    fn set_quiet(
        &mut self,
        target: &Quiet,
        state: QuietState,
        done: String,
        cx: &mut Context<Self>,
    ) {
        let before = self.quiet_state(target);
        let command = Command::Several(self.quiet_commands(target, state));
        let undo = Command::Several(self.quiet_commands(target, before));
        self.menu = None;
        self.quiet_menu = None;
        self.nav_menu = None;
        // Shown at once; the daemon's change brings the rest.
        self.apply_quiet_locally(target, state);
        self.send(command, Some(done), Some(undo), false, cx);
        cx.notify();
    }

    /// Shows `state` before the daemon confirms it.
    fn apply_quiet_locally(&mut self, target: &Quiet, state: QuietState) {
        let now = Timestamp::now().as_second();
        let mute = |target: MuteTarget, label: String| katna_store::Mute {
            target,
            label,
            until: state.muted.flatten(),
            server: false,
            created_at: now,
        };
        let (key, label) = match target {
            Quiet::Account(a) => (MuteTarget::Account(*a), String::new()),
            Quiet::Folder { folder, categories } => {
                let role = self
                    .tree
                    .node(*folder)
                    .is_some_and(|n| n.role == Role::Inbox)
                    .then_some(katna_store::FolderRole::Inbox.as_str());
                let bell = if state.bell { Bell::ON } else { Bell::OFF };
                let tabs: Vec<Option<MailCategory>> = if categories.is_empty() {
                    vec![role.map(|_| MailCategory::Primary)]
                } else {
                    categories.iter().copied().map(Some).collect()
                };
                for category in tabs {
                    self.alerts.bells.insert((*folder, category), bell);
                }
                (MuteTarget::Folder(*folder), String::new())
            }
        };
        self.alerts.mutes.retain(|m| m.target != key);
        if state.muted.is_some() {
            self.alerts.mutes.push(mute(key, label));
        }
    }

    /// Mutes `target` for `how` long.
    pub(super) fn quiet(&mut self, target: Quiet, how: For, cx: &mut Context<Self>) {
        // A while mutes the whole folder, so the note names the folder
        // rather than the tab.
        let name = match (&target, how) {
            (Quiet::Folder { folder, .. }, For::Hour | For::Tomorrow) => {
                self.quiet_name(&Quiet::Folder {
                    folder: *folder,
                    categories: Vec::new(),
                })
            }
            _ => self.quiet_name(&target),
        };
        let until = match how {
            For::Hour => Some(Timestamp::now().as_second() + 3600),
            For::Tomorrow => super::snooze::tomorrow_morning(&self.tz),
            For::Always => None,
        };
        let mut state = self.quiet_state(&target);
        let done = match (how, until, &target) {
            (For::Always, _, Quiet::Folder { .. }) => {
                // For good, a folder's bell goes off.
                state.bell = false;
                state.muted = None;
                tr!("quiet-off", name = name)
            }
            (_, Some(until), _) => {
                state.muted = Some(Some(until));
                tr!(
                    "quiet-muted-until",
                    name = name,
                    when = super::snooze::describe(until, &self.tz)
                )
            }
            _ => {
                state.muted = Some(None);
                tr!("quiet-off", name = name)
            }
        };
        self.set_quiet(&target, state, done, cx);
    }

    /// Turns `target` back on: unmuted, and for a folder its bell on.
    pub(super) fn unquiet(&mut self, target: Quiet, cx: &mut Context<Self>) {
        // A mute for a while was on the whole folder.
        let name = match &target {
            Quiet::Folder { folder, .. } if self.quiet_state(&target).muted.is_some() => self
                .quiet_name(&Quiet::Folder {
                    folder: *folder,
                    categories: Vec::new(),
                }),
            _ => self.quiet_name(&target),
        };
        let state = QuietState {
            bell: true,
            muted: None,
            changed: false,
        };
        self.set_quiet(&target, state, tr!("quiet-on", name = name), cx);
    }

    /// Opens the mute choices for `target` at `at`, in place of the
    /// folder pane's menu.
    pub(super) fn open_quiet_menu(
        &mut self,
        target: Quiet,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.nav_menu = None;
        self.menu = None;
        self.quiet_menu = Some(QuietMenu { target, at });
        cx.notify();
    }

    pub(super) fn close_quiet_menu(&mut self, cx: &mut Context<Self>) -> bool {
        if self.quiet_menu.take().is_some() {
            cx.notify();
            true
        } else {
            false
        }
    }

    /// The choices for `target`: how long to mute it when it rings, else
    /// turning it back on.
    pub(super) fn quiet_items(
        &self,
        target: &Quiet,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> gpui::Div {
        let state = self.quiet_state(target);
        let mut items = menu(th);
        if state.rings() {
            if self.quiet_timed(target) {
                for (id, how, label) in [
                    ("quiet-hour", For::Hour, tr!("quiet-for-hour")),
                    ("quiet-tomorrow", For::Tomorrow, tr!("quiet-until-tomorrow")),
                ] {
                    let target = target.clone();
                    items = items.child(menu_item_icon(id, "schedule", &label, th).on_click(
                        cx.listener(move |this, _, _, cx| this.quiet(target.clone(), how, cx)),
                    ));
                }
            }
            let target = target.clone();
            items.child(
                menu_item_icon("quiet-always", "bell-off", &tr!("quiet-until-unmuted"), th)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.quiet(target.clone(), For::Always, cx)
                    })),
            )
        } else {
            let target = target.clone();
            items.child(
                menu_item_icon("quiet-on", "bell", &tr!("quiet-turn-on"), th)
                    .on_click(cx.listener(move |this, _, _, cx| this.unquiet(target.clone(), cx))),
            )
        }
    }

    /// What a menu row about `target` says: Mute… while it rings, else
    /// Unmute or Notify for new mail.
    pub(super) fn quiet_menu_label(&self, target: &Quiet) -> (&'static str, SharedString) {
        let state = self.quiet_state(target);
        if state.rings() {
            ("bell-off", tr!("quiet-mute").into())
        } else if state.muted.is_some() {
            ("bell", tr!("quiet-unmute").into())
        } else {
            ("bell", tr!("quiet-notify").into())
        }
    }

    /// A folder-pane menu row's click on `target`: the choices while it
    /// rings, else straight back on.
    pub(super) fn quiet_menu_click(
        &mut self,
        target: Quiet,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        if self.quiet_state(&target).rings() {
            self.open_quiet_menu(target, at, cx);
        } else {
            self.unquiet(target, cx);
        }
    }

    /// The list toolbar's bell for the open folder or tab.
    pub(super) fn quiet_button(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let target = self.open_quiet_target()?;
        let state = self.quiet_state(&target);
        let open = self.menu == Some(Menu::Quiet);
        let button = if state.rings() {
            icon_button("list-bell", "bell", 20.0, th)
                .when(!open, |d| d.tooltip(tip(tr!("quiet-tip-rings"), th)))
                .on_click(cx.listener(|this, _, _, cx| this.toggle_menu(Menu::Quiet, cx)))
        } else {
            let text = match state.muted {
                Some(Some(until)) => tr!(
                    "quiet-tip-muted-until",
                    when = super::snooze::describe(until, &self.tz)
                ),
                _ => tr!("quiet-tip-off"),
            };
            icon_button("list-bell", "bell-off", 20.0, th)
                .tooltip(tip(text, th))
                .on_click(cx.listener(move |this, _, _, cx| this.unquiet(target.clone(), cx)))
        };
        Some(self.with_menu(button, Menu::Quiet, th, cx))
    }

    /// The menu of the toolbar's bell.
    pub(super) fn quiet_toolbar_items(&self, th: &Theme, cx: &mut Context<Self>) -> gpui::Div {
        match self.open_quiet_target() {
            Some(target) => self.quiet_items(&target, th, cx),
            None => menu(th),
        }
    }

    /// The folder pane's marker of `folder`: a bell when it notifies but
    /// would not by default, a crossed bell when it is muted or would
    /// notify by default but does not.
    pub(super) fn folder_bell_icon(&self, folder: FolderId) -> Option<&'static str> {
        let state = self.quiet_state(&Quiet::Folder {
            folder,
            categories: Vec::new(),
        });
        if state.muted.is_some() {
            Some("bell-off")
        } else if state.changed {
            Some(if state.bell { "bell" } else { "bell-off" })
        } else {
            None
        }
    }

    /// The crossed bell on a muted account's heading.
    pub(super) fn account_bell_icon(&self, account: AccountId) -> Option<&'static str> {
        self.quiet_state(&Quiet::Account(account))
            .muted
            .map(|_| "bell-off")
    }

    pub(super) fn render_quiet_menu(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let open = self.quiet_menu.as_ref()?;
        let items = self.quiet_items(&open.target, th, cx);
        let close = || {
            cx.listener(|this: &mut Self, _: &gpui::MouseDownEvent, _, cx| {
                this.close_quiet_menu(cx);
            })
        };
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .child(
                    deferred(
                        div()
                            .id("quiet-menu-scrim")
                            .absolute()
                            .top(px(-2000.0))
                            .left(px(-4000.0))
                            .w(px(8000.0))
                            .h(px(6000.0))
                            .occlude()
                            .on_mouse_down(MouseButton::Left, close())
                            .on_mouse_down(MouseButton::Right, close()),
                    )
                    .with_priority(3),
                )
                .child(
                    deferred(
                        anchored()
                            .position(open.at)
                            .snap_to_window_with_margin(px(8.0))
                            .child(div().occlude().child(items)),
                    )
                    .with_priority(4),
                )
                .into_any_element(),
        )
    }
}

// Conversations and senders

impl MailWindow {
    /// The conversation of line `key`, and a message of it, which the
    /// daemon finds the conversation by.
    fn line_conversation(&self, key: EntryKey) -> Option<(Option<ThreadId>, MessageId)> {
        let mail = self.mail.as_ref().ok()?;
        match key {
            EntryKey::Thread(thread) => {
                Some((Some(thread), mail.entry_messages(key).last().copied()?))
            }
            EntryKey::Message(id) => Some((mail.message_thread(id), id)),
        }
    }

    /// Whether the conversation of line `key` is muted. A message line
    /// outside the conversation view is checked only by its own thread.
    pub(super) fn line_muted(&self, key: EntryKey) -> bool {
        match key {
            EntryKey::Thread(thread) => self.alerts.muted_threads.contains(&thread),
            EntryKey::Message(_) => false,
        }
    }

    /// Whether the conversations of `keys` are all muted.
    pub(super) fn lines_muted(&self, keys: &[EntryKey]) -> bool {
        !keys.is_empty()
            && keys.iter().all(|k| {
                self.line_conversation(*k)
                    .and_then(|(thread, _)| thread)
                    .is_some_and(|t| self.alerts.muted_threads.contains(&t))
            })
    }

    /// Whether mail from `address` is muted.
    pub(super) fn sender_muted(&self, address: &str) -> bool {
        let address = address.trim().to_lowercase();
        self.alerts.mute(&MuteTarget::Sender(address)).is_some()
    }

    /// Mutes the conversations of `keys`, or unmutes them, with Undo.
    pub(super) fn mute_lines(&mut self, keys: Vec<EntryKey>, mute: bool, cx: &mut Context<Self>) {
        self.menu = None;
        let lines: Vec<(Option<ThreadId>, MessageId)> = keys
            .iter()
            .filter_map(|k| self.line_conversation(*k))
            .collect();
        if lines.is_empty() {
            return;
        }
        let subject = self
            .reader
            .as_ref()
            .filter(|r| keys.contains(&r.key))
            .and_then(|r| r.mute_info())
            .map(|(subject, ..)| subject)
            .unwrap_or_default();
        let now = Timestamp::now().as_second();
        let mut command = Vec::new();
        let mut undo = Vec::new();
        for (thread, message) in &lines {
            let what = Muted::Conversation(*message);
            if mute {
                command.push(Command::Mute(what.clone(), 0));
                undo.push(Command::Unmute(what));
            } else {
                command.push(Command::Unmute(what.clone()));
                undo.push(Command::Mute(what, 0));
            }
            // Shown at once; the daemon's change brings the rest.
            if let Some(thread) = thread {
                self.alerts
                    .mutes
                    .retain(|m| m.target != MuteTarget::Thread(*thread));
                if mute {
                    self.alerts.muted_threads.insert(*thread);
                    self.alerts.thread_message.insert(*thread, *message);
                    self.alerts.mutes.push(katna_store::Mute {
                        target: MuteTarget::Thread(*thread),
                        label: subject.clone(),
                        until: None,
                        server: false,
                        created_at: now,
                    });
                } else {
                    self.alerts.muted_threads.remove(thread);
                }
            }
        }
        let count = lines.len() as u64;
        let done = if mute {
            tr!("quiet-conversation-muted", count = count)
        } else {
            tr!("quiet-conversation-unmuted", count = count)
        };
        self.send(
            Command::Several(command),
            Some(done),
            Some(Command::Several(undo)),
            false,
            cx,
        );
        cx.notify();
    }

    /// Mutes the targets' conversations, or unmutes them when all are
    /// muted: the reader's bell, the More menus and the shortcut.
    pub(super) fn toggle_mute_targets(&mut self, cx: &mut Context<Self>) {
        let keys = self.target_keys();
        let mute = !self.lines_muted(&keys);
        self.mute_lines(keys, mute, cx);
    }

    /// Mutes mail from `address`, or unmutes it, with Undo.
    pub(super) fn mute_sender(&mut self, address: String, mute: bool, cx: &mut Context<Self>) {
        self.menu = None;
        let address = address.trim().to_lowercase();
        if address.is_empty() {
            return;
        }
        let what = Muted::Sender(address.clone());
        let (command, undo, done) = if mute {
            (
                Command::Mute(what.clone(), 0),
                Command::Unmute(what),
                tr!("quiet-sender-muted", address = address.clone()),
            )
        } else {
            (
                Command::Unmute(what.clone()),
                Command::Mute(what, 0),
                tr!("quiet-sender-unmuted", address = address.clone()),
            )
        };
        let target = MuteTarget::Sender(address);
        self.alerts.mutes.retain(|m| m.target != target);
        if mute {
            self.alerts.mutes.push(katna_store::Mute {
                target,
                label: String::new(),
                until: None,
                server: false,
                created_at: Timestamp::now().as_second(),
            });
        }
        self.send(command, Some(done), Some(undo), false, cx);
        cx.notify();
    }

    /// The sender of the open conversation's newest message, for the
    /// reading pane's More menu.
    pub(super) fn reader_sender(&self) -> Option<String> {
        let (_, _, sender) = self.reader.as_ref()?.mute_info()?;
        Some(sender).filter(|s| !s.is_empty())
    }

    /// Mute conversation (or Unmute) for the More menus, and in the
    /// reading pane's, Mute sender.
    pub(super) fn mute_menu_items(
        &self,
        conversation: bool,
        reader: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> gpui::Div {
        let muted = self.lines_muted(&self.target_keys());
        let conversation = conversation.then(|| {
            if muted {
                menu_item_icon("more-mute", "bell", &tr!("quiet-unmute-conversation"), th)
            } else {
                menu_item_icon("more-mute", "bell-off", &tr!("quiet-mute-conversation"), th)
            }
            .on_click(cx.listener(|this, _, _, cx| this.toggle_mute_targets(cx)))
        });
        let sender = reader
            .then(|| self.reader_sender())
            .flatten()
            .map(|sender| {
                let muted = self.sender_muted(&sender);
                if muted {
                    menu_item_icon("more-mute-sender", "bell", &tr!("quiet-unmute-sender"), th)
                } else {
                    menu_item_icon(
                        "more-mute-sender",
                        "bell-off",
                        &tr!("quiet-mute-sender"),
                        th,
                    )
                }
                .on_click(
                    cx.listener(move |this, _, _, cx| this.mute_sender(sender.clone(), !muted, cx)),
                )
            });
        div()
            .flex()
            .flex_col()
            .children(conversation)
            .children(sender)
    }

    /// The reading pane's bell: mutes or unmutes the open conversation.
    pub(super) fn reader_mute_button(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let muted = self
            .reader
            .as_ref()
            .is_some_and(|r| self.lines_muted(&[r.key]));
        let (glyph, text) = if muted {
            ("bell-off", tr!("quiet-unmute-conversation"))
        } else {
            ("bell", tr!("quiet-mute-conversation"))
        };
        icon_button("reader-mute", glyph, 20.0, th)
            .tooltip(tip(text, th))
            .on_click(cx.listener(|this, _, _, cx| {
                if let Some(key) = this.reader.as_ref().map(|r| r.key) {
                    let mute = !this.lines_muted(&[key]);
                    this.mute_lines(vec![key], mute, cx);
                }
            }))
            .into_any_element()
    }

    /// The strip over a muted conversation, with Unmute.
    pub(super) fn render_muted_strip(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let key = self.reader.as_ref()?.key;
        if !self.lines_muted(&[key]) {
            return None;
        }
        Some(
            div()
                .ml(px(self.reader_indent()))
                .mr(px(16.0))
                .mb(px(8.0))
                .px(px(12.0))
                .min_h(px(40.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(12.0))
                .rounded(px(8.0))
                .bg(gpui::rgba(th.chip))
                .text_size(px(13.0))
                .text_color(gpui::rgba(th.text_dim))
                .child(crate::widgets::icon("bell-off", th.text_dim, 18.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .child(tr!("quiet-conversation-strip")),
                )
                .child(
                    div()
                        .id("muted-strip-unmute")
                        .flex_none()
                        .px(px(10.0))
                        .h(px(28.0))
                        .flex()
                        .items_center()
                        .rounded(px(14.0))
                        .cursor_pointer()
                        .text_color(gpui::rgba(th.accent))
                        .hover(|s| s.bg(gpui::rgba(th.hover)))
                        .child(tr!("quiet-unmute"))
                        .on_click(
                            cx.listener(move |this, _, _, cx| {
                                this.mute_lines(vec![key], false, cx)
                            }),
                        ),
                )
                .into_any_element(),
        )
    }
}
