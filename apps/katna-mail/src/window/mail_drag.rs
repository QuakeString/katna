// SPDX-License-Identifier: GPL-3.0-or-later

//! Mail dragged from the list onto a folder in the folder pane: the line,
//! or the ticked lines when the dragged one is ticked, follow the pointer
//! as a small pill, and the folder under it lights up when it can take
//! them. A drop moves them there, as Move to does, with Undo; on Gmail a
//! label of the user's own is put on them instead, and only Inbox, Spam
//! and Trash move.

use gpui::{Context, DragMoveEvent, Pixels, Point, SharedString, Window, div, prelude::*, rgba};
use katna_core::AccountId;
use katna_i18n::tr;
use katna_store::FolderId;
use katna_ui::{px, unpx};

use super::{Act, MailWindow};
use crate::daemon::Command;
use crate::data::EntryKey;
use crate::sidebar::Role;
use crate::theme::Theme;
use crate::widgets::{elevation, icon};

/// How far right of and below the pointer the pill shows.
const GAP: Point<f32> = Point { x: 14.0, y: 16.0 };
const MAX_WIDTH: f32 = 280.0;

/// Mail on its way to a folder.
#[derive(Clone)]
pub(super) struct MailDrag {
    pub(super) keys: Vec<EntryKey>,
    account: AccountId,
    /// The folder the list shows, where the mail already is.
    from: Option<FolderId>,
    /// The subject, or how many lines.
    label: SharedString,
    th: Theme,
    /// Where the pointer took the line, from its corner: the pill is drawn
    /// from there.
    offset: Point<Pixels>,
}

impl MailDrag {
    /// Whether folder `folder` of account `to`, with role `role`, takes it.
    pub(super) fn takes(&self, to: AccountId, folder: FolderId, role: Role) -> bool {
        takes_drop(self.account, self.from, to, folder, role)
    }
}

impl Render for MailDrag {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let th = &self.th;
        div()
            .pl(px(unpx(self.offset.x) + GAP.x))
            .pt(px(unpx(self.offset.y) + GAP.y))
            .child(
                div()
                    .max_w(px(MAX_WIDTH))
                    .h(px(32.0))
                    .px(px(12.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(10.0))
                    .rounded_full()
                    .bg(rgba(th.raised))
                    .shadow(elevation(th, 3.0))
                    .text_size(px(14.0))
                    .text_color(rgba(th.text))
                    .cursor_grabbing()
                    .child(icon("mail", th.accent, 18.0))
                    .child(div().min_w_0().truncate().child(self.label.clone())),
            )
    }
}

/// Whether mail of `account` from `from` may be dropped on `folder` of
/// `to`, a folder with role `role`.
pub(super) fn takes_drop(
    account: AccountId,
    from: Option<FolderId>,
    to: AccountId,
    folder: FolderId,
    role: Role,
) -> bool {
    account == to
        && from != Some(folder)
        && !matches!(
            role,
            Role::Drafts | Role::Sent | Role::Flagged | Role::Snoozed
        )
}

impl MailWindow {
    /// What dragging line `key`, showing `subject`, carries.
    pub(super) fn mail_drag(
        &self,
        key: EntryKey,
        account: AccountId,
        subject: &str,
        th: &Theme,
    ) -> MailDrag {
        let keys = if self.checked.contains(&key) {
            self.entries
                .iter()
                .filter(|e| self.checked.contains(&e.key))
                .map(|e| e.key)
                .collect()
        } else {
            vec![key]
        };
        let label = if keys.len() == 1 && !subject.trim().is_empty() {
            subject.to_owned()
        } else {
            let kind = if self.config.mail.conversations {
                "conversation"
            } else {
                "message"
            };
            tr!("drag-mail", count = keys.len() as u64, kind = kind)
        };
        MailDrag {
            keys,
            account,
            from: self.listed_folder(),
            label: label.into(),
            th: *th,
            offset: Point::default(),
        }
    }

    /// Starts the pill for `drag`, taken `offset` from the line's corner.
    pub(super) fn start_mail_drag(drag: &MailDrag, offset: Point<Pixels>) -> MailDrag {
        MailDrag {
            offset,
            ..drag.clone()
        }
    }

    /// Notes which lines a drag carries, so they fade in the list.
    pub(super) fn mail_drag_moved(
        &mut self,
        event: &DragMoveEvent<MailDrag>,
        cx: &mut Context<Self>,
    ) {
        let drag = event.drag(cx);
        if self.mail_dragging != drag.keys {
            self.mail_dragging = drag.keys.clone();
            cx.notify();
        }
    }

    /// Whether line `key` is being dragged.
    pub(super) fn mail_dragged(&self, key: EntryKey, cx: &gpui::App) -> bool {
        cx.has_active_drag() && self.mail_dragging.contains(&key)
    }

    /// Whether `drag` may be dropped on `folder`.
    pub(super) fn takes_mail(&self, drag: &MailDrag, folder: FolderId) -> bool {
        let (Some(to), Some(node)) = (self.tree.account_of(folder), self.tree.node(folder)) else {
            return false;
        };
        drag.takes(to, folder, node.role)
    }

    /// `drag` dropped on `folder`: moves the mail there, or on Gmail puts
    /// the label on it.
    pub(super) fn drop_mail(&mut self, drag: &MailDrag, folder: FolderId, cx: &mut Context<Self>) {
        self.mail_dragging.clear();
        if !self.takes_mail(drag, folder) {
            return;
        }
        let role = self.tree.node(folder).map_or(Role::Other, |n| n.role);
        if self.tree.is_gmail(drag.account) && role == Role::Other {
            let name = self
                .tree
                .node(folder)
                .map(|n| n.label())
                .unwrap_or_default();
            let ids: Vec<_> = match &self.mail {
                Ok(mail) => drag
                    .keys
                    .iter()
                    .flat_map(|k| mail.entry_messages(*k))
                    .collect(),
                Err(_) => return,
            };
            if ids.is_empty() {
                return;
            }
            self.send(
                Command::Labels(ids.clone(), vec![folder], Vec::new()),
                Some(tr!("toast-label-added", label = name.as_str())),
                Some(Command::Labels(ids, Vec::new(), vec![folder])),
                false,
                cx,
            );
        } else {
            self.act(Act::MoveTo(folder), drag.keys.clone(), cx);
        }
        cx.notify();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_folder_takes_mail_of_its_own_account_only() {
        let (a, b) = (AccountId(1), AccountId(2));
        let inbox = Some(FolderId(1));
        assert!(takes_drop(a, inbox, a, FolderId(5), Role::Other));
        assert!(takes_drop(a, inbox, a, FolderId(9), Role::Trash));
        assert!(takes_drop(a, None, a, FolderId(1), Role::Inbox));
        assert!(
            !takes_drop(a, inbox, a, FolderId(1), Role::Inbox),
            "already there"
        );
        assert!(
            !takes_drop(a, inbox, b, FolderId(5), Role::Other),
            "another account"
        );
        assert!(!takes_drop(a, inbox, a, FolderId(3), Role::Drafts));
        assert!(!takes_drop(a, inbox, a, FolderId(4), Role::Sent));
    }
}
