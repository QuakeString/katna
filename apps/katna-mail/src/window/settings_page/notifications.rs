// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings › Notifications: new-mail notices and the taskbar count, which
//! folders and inbox tabs notify and count, and what is muted
//! (`docs/ARCHITECTURE.md` §15.1.1). The bells and mutes are also set
//! where their folder shows; this page is the overview.

use gpui::{AnyElement, Context, Div, div, prelude::*, rgba};
use katna_core::{Account, MailCategory};
use katna_i18n::tr;
use katna_store::{Bell, FolderId, Mute, MuteTarget};
use katna_ui::px;

use super::MailWindow;
use crate::daemon::{Command, Muted};
use crate::sidebar::Role;
use crate::theme::Theme;
use crate::widgets::{Check, checkbox_colored, icon, outlined_button};

/// The width of the Notify and Count columns.
const COLUMN: f32 = 64.0;

/// A line of an account's table: a folder, or an inbox tab.
struct Line {
    folder: FolderId,
    /// The tab's categories; empty for a whole folder.
    categories: Vec<MailCategory>,
    icon: &'static str,
    label: String,
    /// Under the inbox, as its tab.
    tab: bool,
}

/// Folders whose mail is the user's own doing or already dealt with:
/// nothing new lands there to notify about.
fn quiet_role(role: Role) -> bool {
    matches!(
        role,
        Role::Drafts | Role::Sent | Role::Trash | Role::Snoozed | Role::Flagged | Role::All
    )
}

impl MailWindow {
    pub(super) fn notifications_section(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let accounts: Vec<Account> = self
            .accounts
            .iter()
            .filter(|a| a.kind.is_mail())
            .cloned()
            .collect();
        let general = &self.config.general;
        div()
            .flex()
            .flex_col()
            .child(self.row(
                tr!("settings-general-notifications"),
                Some(&tr!("settings-general-notifications-detail")),
                self.notification_switches(th, cx),
                th,
            ))
            .child(self.row(
                tr!("settings-notifications-sounds"),
                Some(&tr!("settings-notifications-sounds-detail")),
                self.sound_lines(th, cx),
                th,
            ))
            .child(self.row(
                tr!("settings-notifications-count"),
                Some(&tr!("settings-notifications-count-detail")),
                self.switch_row(
                    "page-unread-badge",
                    tr!("settings-general-unread-badge"),
                    tr!("settings-general-unread-badge-detail"),
                    general.unread_badge,
                    super::Change::UnreadBadge(!general.unread_badge),
                    th,
                    cx,
                ),
                th,
            ))
            .children(accounts.iter().enumerate().map(|(ix, account)| {
                let name = account.display_name.trim();
                let (title, address) = if name.is_empty() || name == account.address {
                    (account.address.clone(), None)
                } else {
                    (name.to_owned(), Some(account.address.as_str()))
                };
                self.row(title, address, self.bell_table(ix, account, th, cx), th)
            }))
            .child(self.row(
                tr!("settings-notifications-muted"),
                Some(&tr!("settings-notifications-muted-detail")),
                self.muted_list(th, cx),
                th,
            ))
            .into_any_element()
    }

    /// The lines of `account`'s table: its inbox, or its tabs, then the
    /// folders new mail may land in.
    fn bell_lines(&self, account: &Account) -> Vec<Line> {
        let tabs = self.account_tabs(account.id);
        let mut lines = Vec::new();
        for (folder, label, role) in self.tree.folders_of(account.id) {
            if quiet_role(role) {
                continue;
            }
            let inbox = role == Role::Inbox;
            lines.push(Line {
                folder,
                categories: Vec::new(),
                icon: super::super::nav::role_icon(role),
                label,
                tab: false,
            });
            if inbox && tabs.len() > 1 {
                lines.extend(tabs.iter().map(|tab| Line {
                    folder,
                    categories: tab.categories.clone(),
                    icon: tab.icon,
                    label: tab.label(),
                    tab: true,
                }));
            }
        }
        lines
    }

    /// The Notify and Count boxes of each folder and inbox tab of
    /// `account`.
    fn bell_table(
        &self,
        ix: usize,
        account: &Account,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let header = div()
            .h(px(28.0))
            .px(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .text_size(px(12.0))
            .text_color(rgba(th.text_faint))
            .child(div().flex_1())
            .child(column(th).child(tr!("settings-notifications-notify")))
            .child(column(th).child(tr!("settings-notifications-counts")));
        let lines = self.bell_lines(account);
        let rows = lines.into_iter().enumerate().map(|(n, line)| {
            let id = ix * 1000 + n;
            let role = self
                .tree
                .node(line.folder)
                .map(|node| node.role)
                .filter(|r| *r == Role::Inbox)
                .map(|_| katna_store::FolderRole::Inbox.as_str());
            // An inbox shown with tabs has its boxes on the tabs.
            let boxes = line.tab || !line.categories.is_empty() || {
                let tabs = self.account_tabs(account.id);
                role.is_none() || tabs.len() <= 1
            };
            let categories: Vec<Option<MailCategory>> = if line.categories.is_empty() {
                vec![None]
            } else {
                line.categories.iter().copied().map(Some).collect()
            };
            let bells: Vec<Bell> = categories
                .iter()
                .map(|c| self.alerts.bell(line.folder, role, *c))
                .collect();
            let notify = bells.iter().any(|b| b.notify);
            let count = bells.iter().any(|b| b.count);
            let cell = |which: usize, on: bool, cx: &mut Context<Self>| {
                let folder = line.folder;
                let categories = categories.clone();
                div()
                    .id(("bell-cell", id * 2 + which))
                    .w(px(COLUMN))
                    .h(px(32.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(8.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .child(checkbox_colored(
                        ("bell-box", id * 2 + which),
                        Check::from(on),
                        th.accent,
                        th,
                    ))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        let bell = if which == 0 {
                            Bell { notify: !on, count }
                        } else {
                            Bell { notify, count: !on }
                        };
                        this.set_bells(folder, &categories, bell, cx);
                    }))
            };
            div()
                .h(px(32.0))
                .pl(px(if line.tab { 36.0 } else { 8.0 }))
                .pr(px(8.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(10.0))
                .text_size(px(14.0))
                .child(icon(line.icon, th.text_dim, 18.0))
                .child(div().flex_1().min_w_0().truncate().child(line.label))
                .when(boxes, |d| {
                    d.child(cell(0, notify, cx)).child(cell(1, count, cx))
                })
        });
        div()
            .flex()
            .flex_col()
            .child(header)
            .children(rows)
            .into_any_element()
    }

    /// Sets the bell of `folder`'s `categories` (`None`: the folder), with
    /// Undo.
    fn set_bells(
        &mut self,
        folder: FolderId,
        categories: &[Option<MailCategory>],
        bell: Bell,
        cx: &mut Context<Self>,
    ) {
        let role = self
            .tree
            .node(folder)
            .filter(|n| n.role == Role::Inbox)
            .map(|_| katna_store::FolderRole::Inbox.as_str());
        let mut command = Vec::new();
        let mut undo = Vec::new();
        for category in categories {
            let before = self.alerts.bell(folder, role, *category);
            undo.push(Command::SetBell(folder, *category, before));
            command.push(Command::SetBell(folder, *category, bell));
            let key = if role.is_some() {
                Some(category.unwrap_or_default())
            } else {
                None
            };
            self.alerts.bells.insert((folder, key), bell);
        }
        self.send(
            Command::Several(command),
            None,
            Some(Command::Several(undo)),
            true,
            cx,
        );
        cx.notify();
    }

    /// Everything muted, with when it ends and Unmute.
    fn muted_list(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        if self.alerts.mutes.is_empty() {
            return self
                .quiet_note(tr!("settings-notifications-nothing-muted"), th)
                .into_any_element();
        }
        let rows = self.alerts.mutes.iter().enumerate().map(|(n, mute)| {
            let (glyph, name) = self.mute_name(mute);
            let until = match mute.until {
                Some(at) => tr!(
                    "settings-notifications-until",
                    when = super::super::snooze::describe(at, &self.tz)
                ),
                None => tr!("settings-notifications-until-unmuted"),
            };
            let what = self.muted_of(mute);
            div()
                .min_h(px(48.0))
                .px(px(8.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(12.0))
                .child(icon(glyph, th.text_dim, 18.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .child(div().truncate().text_size(px(14.0)).child(name))
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgba(th.text_dim))
                                .child(until),
                        ),
                )
                .when_some(what, |d, what| {
                    d.child(
                        outlined_button(("unmute", n), tr!("quiet-unmute"), th).on_click(
                            cx.listener(move |this, _, _, cx| {
                                this.unmute_from_settings(what.clone(), cx)
                            }),
                        ),
                    )
                })
        });
        div().flex().flex_col().children(rows).into_any_element()
    }

    /// What a muted thing is called, with its icon.
    fn mute_name(&self, mute: &Mute) -> (&'static str, String) {
        match &mute.target {
            MuteTarget::Account(account) => (
                "mail",
                self.accounts
                    .iter()
                    .find(|a| a.id == *account)
                    .map(|a| a.address.clone())
                    .unwrap_or_default(),
            ),
            MuteTarget::Folder(folder) => {
                let node = self.tree.node(*folder);
                let label = node.map(|n| n.label()).unwrap_or_default();
                let account = self
                    .tree
                    .account_of(*folder)
                    .and_then(|a| self.accounts.iter().find(|x| x.id == a))
                    .map(|a| a.address.clone());
                (
                    node.map_or("folder", |n| super::super::nav::role_icon(n.role)),
                    match account {
                        Some(account) if self.accounts.len() > 1 => format!("{label} · {account}"),
                        _ => label,
                    },
                )
            }
            MuteTarget::Thread(_) => (
                "forum",
                if mute.label.is_empty() {
                    tr!("settings-notifications-a-conversation")
                } else {
                    mute.label.clone()
                },
            ),
            MuteTarget::Sender(address) => ("contacts", address.clone()),
        }
    }

    /// What Unmute names for `mute`.
    fn muted_of(&self, mute: &Mute) -> Option<Muted> {
        Some(match &mute.target {
            MuteTarget::Account(a) => Muted::Account(*a),
            MuteTarget::Folder(f) => Muted::Folder(*f),
            MuteTarget::Thread(t) => Muted::Conversation(*self.alerts.thread_message.get(t)?),
            MuteTarget::Sender(s) => Muted::Sender(s.clone()),
        })
    }

    fn unmute_from_settings(&mut self, what: Muted, cx: &mut Context<Self>) {
        let Some(ix) = self
            .alerts
            .mutes
            .iter()
            .position(|m| self.muted_of(m).as_ref() == Some(&what))
        else {
            return;
        };
        let mute = self.alerts.mutes.remove(ix);
        let undo = Command::Mute(what.clone(), mute.until.unwrap_or(0));
        let (_, name) = self.mute_name(&mute);
        self.send(
            Command::Unmute(what),
            Some(tr!("quiet-on", name = name)),
            Some(undo),
            false,
            cx,
        );
        cx.notify();
    }
}

/// A Notify or Count column's cell, centred.
fn column(_th: &Theme) -> Div {
    div().w(px(COLUMN)).flex().flex_row().justify_center()
}
