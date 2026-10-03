// SPDX-License-Identifier: GPL-3.0-or-later

//! The question before deleting: "Move 12 conversations to Trash?" when
//! several go at once (unless the person said not to ask again), and
//! always before deleting for good (in Trash, or on an account without
//! one). A single conversation moved to Trash goes without asking; its
//! snackbar offers Undo.
//!
//! The same card asks before deleting a folder or label the user made,
//! from the folder pane's right-click menu: how much mail it holds and
//! where that goes.

use gpui::{AnyElement, Context, FontWeight, Window, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_store::FolderId;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::{px, unpx};

use super::settings::Change;
use super::{Act, MailWindow};
use crate::data::EntryKey;
use crate::sidebar::Role;
use crate::theme::{Theme, fade};
use crate::widgets::{FocusRing, elevation, icon};
use crate::{daemon, format};

const WIDTH: f32 = 440.0;

/// A delete waiting for an answer.
pub(super) struct DeleteAsk {
    keys: Vec<EntryKey>,
    /// Deleted for good, not moved to Trash: no "Don't ask again".
    forever: bool,
    dont_ask: bool,
    closing: bool,
    /// The dialog has had the keys given to it.
    focused: bool,
    shown: Spring,
    /// Deleting this folder or label, not mail.
    folder: Option<FolderAsk>,
}

/// A folder or label to delete.
struct FolderAsk {
    folder: FolderId,
    name: String,
    /// A Gmail label: its mail stays where else it is.
    gmail: bool,
    /// The account has a Trash for its mail to go to.
    trash: bool,
    /// The lines in it and in the folders inside it.
    count: u64,
}

impl MailWindow {
    /// Whether deleting `count` lines must be asked about first.
    pub(super) fn delete_needs_asking(&self, count: usize, forever: bool) -> bool {
        !self.delete_confirmed && (forever || (count >= 2 && self.config.mail.confirm_delete))
    }

    /// Asks before deleting `keys`.
    pub(super) fn ask_delete(
        &mut self,
        keys: Vec<EntryKey>,
        forever: bool,
        cx: &mut Context<Self>,
    ) {
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.delete_ask = Some(DeleteAsk {
            keys,
            forever,
            dont_ask: false,
            closing: false,
            focused: false,
            shown,
            folder: None,
        });
        cx.notify();
    }

    /// Asks before deleting `folder`, a folder or label the user made, and
    /// the folders inside it.
    pub(super) fn ask_delete_folder(&mut self, folder: FolderId, cx: &mut Context<Self>) {
        let (Some(account), Some(node)) = (self.tree.account_of(folder), self.tree.node(folder))
        else {
            return;
        };
        let name = node.label();
        let conversations = self.config.mail.conversations;
        let count = match &self.mail {
            Ok(mail) => {
                let mut keys = std::collections::HashSet::new();
                for id in self.tree.subtree(folder) {
                    keys.extend(
                        mail.entries(id, None, conversations)
                            .into_iter()
                            .map(|e| e.key),
                    );
                }
                keys.len() as u64
            }
            Err(_) => 0,
        };
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.delete_ask = Some(DeleteAsk {
            keys: Vec::new(),
            forever: true,
            dont_ask: false,
            closing: false,
            focused: false,
            shown,
            folder: Some(FolderAsk {
                folder,
                name,
                gmail: self.tree.is_gmail(account),
                trash: self.tree.role_folder(account, Role::Trash).is_some(),
                count,
            }),
        });
        cx.notify();
    }

    /// Deletes `folder` on the server through the daemon; once gone, says
    /// so, and opens the inbox if the list showed it or a folder inside.
    fn delete_folder(
        &mut self,
        folder: FolderId,
        name: String,
        gmail: bool,
        cx: &mut Context<Self>,
    ) {
        let connection = self.daemon.clone();
        let gone = self.tree.subtree(folder);
        let inbox = self
            .tree
            .account_of(folder)
            .and_then(|a| self.tree.role_folder(a, Role::Inbox));
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::delete_folder(&connection, folder.0).await
                })
                .await;
            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    if let (Some(listed), Some(inbox)) = (this.listed_folder(), inbox)
                        && gone.contains(&listed)
                    {
                        this.open_folder(inbox, cx);
                    }
                    this.refresh(false, cx);
                    let text = if gmail {
                        tr!("label-deleted", name = name.as_str())
                    } else {
                        tr!("folder-deleted", name = name.as_str())
                    };
                    this.show_snackbar(text, None, cx);
                }
                Err(err) => this.show_snackbar(format::sentence(&err), None, cx),
            })
            .ok();
        })
        .detach();
    }

    /// Closes the question without deleting. Returns whether it was open.
    pub(super) fn close_delete_ask(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(ask) = self.delete_ask.as_mut().filter(|a| !a.closing) else {
            return false;
        };
        ask.closing = true;
        ask.shown.set(0.0);
        cx.notify();
        true
    }

    fn confirm_delete_ask(&mut self, cx: &mut Context<Self>) {
        let Some(ask) = self.delete_ask.as_mut().filter(|a| !a.closing) else {
            return;
        };
        ask.closing = true;
        ask.shown.set(0.0);
        if let Some(folder) = &ask.folder {
            let (id, name, gmail) = (folder.folder, folder.name.clone(), folder.gmail);
            self.delete_folder(id, name, gmail, cx);
            return;
        }
        let keys = ask.keys.clone();
        if ask.dont_ask && !ask.forever {
            self.apply(Change::ConfirmDelete(false), cx);
        }
        self.delete_confirmed = true;
        self.act(Act::Delete, keys, cx);
        self.delete_confirmed = false;
    }

    pub(super) fn render_delete_ask(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let ask = self.delete_ask.as_mut()?;
        let t = ask.shown.tick(window, reduce);
        if ask.closing && ask.shown.settled() {
            self.delete_ask = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        // The keys go to the question, off the list behind it: Enter
        // deletes, Esc cancels, Tab goes round its buttons.
        if !ask.focused {
            ask.focused = true;
            window.focus(&self.dialog_focus, cx);
        }
        let ask = self.delete_ask.as_ref()?;
        let count = ask.keys.len() as u64;
        let kind = if self.config.mail.conversations {
            "conversation"
        } else {
            "message"
        };
        let (title, body, action) = if let Some(folder) = &ask.folder {
            let name = folder.name.as_str();
            let body = if folder.gmail {
                tr!("folder-delete-label-body")
            } else if folder.trash {
                tr!("folder-delete-body", count = folder.count, kind = kind)
            } else {
                tr!(
                    "folder-delete-forever-body",
                    count = folder.count,
                    kind = kind
                )
            };
            (
                tr!("folder-delete-title", name = name),
                body,
                if folder.gmail {
                    tr!("folder-delete-label-confirm")
                } else {
                    tr!("folder-delete-confirm")
                },
            )
        } else if ask.forever {
            (
                tr!("delete-forever-title", count = count, kind = kind),
                tr!("delete-forever-body", count = count),
                tr!("delete-forever-confirm"),
            )
        } else {
            (
                tr!("delete-ask-title", count = count, kind = kind),
                tr!("delete-ask-body", count = count),
                tr!("delete-ask-confirm"),
            )
        };
        let tone = if ask.forever { th.error } else { th.accent };
        let dont_ask = (!ask.forever).then(|| {
            let on = ask.dont_ask;
            div()
                .id("delete-ask-dont")
                .focus_ring(th)
                .mt(px(16.0))
                .ml(px(-6.0))
                .px(px(6.0))
                .h(px(36.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(10.0))
                .rounded(px(8.0))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .text_size(px(14.0))
                .on_click(cx.listener(|this, _, _, cx| {
                    if let Some(ask) = &mut this.delete_ask {
                        ask.dont_ask = !ask.dont_ask;
                    }
                    cx.notify();
                }))
                .child(crate::widgets::checkbox(
                    "delete-ask-box",
                    crate::widgets::Check::from(on),
                    th,
                ))
                .child(tr!("delete-ask-dont-ask"))
        });
        let confirm = div()
            .id("delete-ask-confirm")
            .focus_ring_filled(th)
            .flex_none()
            .h(px(36.0))
            .px(px(20.0))
            .flex()
            .items_center()
            .rounded_full()
            .text_size(px(14.0))
            .font_weight(FontWeight::MEDIUM)
            .cursor_pointer()
            .bg(rgba(tone))
            .text_color(rgba(th.surface))
            .hover(|s| s.shadow(elevation(th, 1.0)))
            .on_click(cx.listener(|this, _, _, cx| this.confirm_delete_ask(cx)))
            .child(action);
        let cancel = div()
            .id("delete-ask-cancel")
            .focus_ring(th)
            .h(px(36.0))
            .px(px(16.0))
            .flex()
            .items_center()
            .rounded_full()
            .text_size(px(14.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgba(th.accent))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(fade(th.accent, 0.08))))
            .on_click(cx.listener(|this, _, _, cx| {
                this.close_delete_ask(cx);
            }))
            .child(tr!("delete-ask-cancel"));
        let body = div()
            .flex()
            .flex_col()
            .px(px(28.0))
            .pt(px(24.0))
            .pb(px(20.0))
            .child(
                div()
                    .size(px(44.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .bg(rgba(fade(tone, 0.14)))
                    .child(icon(
                        if ask.folder.is_some() {
                            "warning"
                        } else {
                            "trash"
                        },
                        tone,
                        24.0,
                    )),
            )
            .child(
                self.copyable(title, th)
                    .mt(px(14.0))
                    .text_size(px(20.0))
                    .line_height(px(28.0)),
            )
            .child(
                self.copyable(body, th)
                    .mt(px(8.0))
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text_dim)),
            )
            .children(dont_ask)
            .child(
                div()
                    .mt(px(20.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(div().flex_1())
                    .child(cancel)
                    .child(confirm),
            );
        let viewport = window.viewport_size();
        let vw = unpx(viewport.width);
        let focus = self.dialog_focus.clone();
        let card = div()
            .id("delete-ask")
            .track_focus(&focus)
            .map(|d| super::popovers::keep_tab_inside(d, &focus))
            .on_key_down(
                cx.listener(move |this, event: &gpui::KeyDownEvent, window, cx| {
                    let stroke = &event.keystroke;
                    // Enter on a focused button presses that button.
                    if stroke.key == "enter"
                        && !stroke.modifiers.modified()
                        && focus.is_focused(window)
                    {
                        cx.stop_propagation();
                        this.confirm_delete_ask(cx);
                    }
                }),
            )
            .occlude()
            .w(px(WIDTH.min(vw - 32.0)))
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(px(28.0))
            .map(|d| crate::widgets::frosted(d, th, th.surface, 28.0))
            .text_color(rgba(th.text))
            .shadow(elevation(th, 3.0))
            .child(body);
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgba(fade(0x0000_0066, t)))
                .child(
                    div()
                        .id("delete-ask-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.close_delete_ask(cx);
                        })),
                )
                .child(div().opacity(t).mt(px(lerp(24.0, 0.0, t))).child(card))
                .into_any_element(),
        )
    }
}
