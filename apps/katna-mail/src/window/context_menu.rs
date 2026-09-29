// SPDX-License-Identifier: GPL-3.0-or-later

//! The right-click menu of the mail list, as in webmail: reply and
//! forward, archive, delete, spam, read and star, "Move to" with the
//! folders, and "Find emails from" the sender. It acts on the ticked lines
//! when the clicked line is one of them, else on the clicked line.

use std::rc::Rc;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Div, MouseButton, Pixels, Point, SharedString,
    Stateful, Window, anchored, deferred, div, ease_out_quint, prelude::*, rgba,
};
use katna_ui::px;
use katna_ui::unpx;

use katna_i18n::tr;

use super::MenuKey;
use super::compose::Kind;
use super::{Act, MailWindow};
use crate::data::{EntryKey, Row};
use crate::sidebar::Role;
use crate::theme::Theme;
use crate::widgets::{icon, raised};

const MENU_WIDTH: f32 = 264.0;
const ITEM_HEIGHT: f32 = 36.0;
const FOLDERS_WIDTH: f32 = 240.0;

/// The open right-click menu.
pub(super) struct ContextMenu {
    ix: usize,
    key: EntryKey,
    /// Where the pointer was, in the window.
    at: Point<Pixels>,
    row: Rc<Row>,
    /// "Move to" shows its folders.
    move_to: bool,
}

impl MailWindow {
    pub(super) fn open_context_menu(
        &mut self,
        ix: usize,
        key: EntryKey,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        let Some(entry) = self.entries.get(ix).copied() else {
            return;
        };
        let folder = self.listed_folder();
        let row = match &mut self.mail {
            Ok(mail) => mail
                .rows(&[entry], folder, self.show_recipients)
                .pop()
                .flatten(),
            Err(_) => None,
        };
        let Some(row) = row.map(|r| self.with_pending(r)) else {
            return;
        };
        self.menu = None;
        self.selected = Some(ix);
        self.context_menu = Some(ContextMenu {
            ix,
            key,
            at,
            row,
            move_to: false,
        });
        cx.notify();
    }

    pub(super) fn close_context_menu(&mut self, cx: &mut Context<Self>) {
        if self.context_menu.take().is_some() {
            cx.notify();
        }
    }

    /// The ticked lines if the clicked one is ticked, else the clicked one.
    fn context_targets(&self, key: EntryKey) -> Vec<EntryKey> {
        if self.checked.contains(&key) {
            self.entries
                .iter()
                .filter(|e| self.checked.contains(&e.key))
                .map(|e| e.key)
                .collect()
        } else {
            vec![key]
        }
    }

    fn context_act(&mut self, act: Act, cx: &mut Context<Self>) {
        let Some(menu) = self.context_menu.take() else {
            return;
        };
        let keys = self.context_targets(menu.key);
        self.act(act, keys, cx);
    }

    /// Opens the snooze menu where the right-click menu was.
    fn context_snooze(&mut self, cx: &mut Context<Self>) {
        let Some(menu) = self.context_menu.take() else {
            return;
        };
        let keys = self.context_targets(menu.key);
        self.open_snooze_menu(keys, menu.at, cx);
    }

    /// Opens the conversation and starts the answer in it.
    fn context_reply(&mut self, kind: Kind, window: &mut Window, cx: &mut Context<Self>) {
        let Some(menu) = self.context_menu.take() else {
            return;
        };
        self.open(menu.ix, window, cx);
        self.open_compose(kind, None, window, cx);
    }

    fn set_move_to(&mut self, on: bool, cx: &mut Context<Self>) {
        if let Some(menu) = &mut self.context_menu
            && menu.move_to != on
        {
            menu.move_to = on;
            cx.notify();
        }
    }

    pub(super) fn render_context_menu(
        &self,
        th: &Theme,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let menu = self.context_menu.as_ref()?;
        // The folders open to the right and down, unless that leaves the
        // window.
        let viewport = window.viewport_size();
        let at = (unpx(menu.at.x), unpx(menu.at.y));
        let flip_x = at.0 + MENU_WIDTH + FOLDERS_WIDTH > unpx(viewport.width);
        let flip_y = at.1 > unpx(viewport.height) / 2.0;
        let row = &menu.row;
        let item = |id: &'static str, name: &str, label: SharedString| -> Stateful<Div> {
            div()
                .id(id)
                .h(px(ITEM_HEIGHT))
                .pl(px(16.0))
                .pr(px(24.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .menu_key(th)
                .child(icon(name, th.text_dim, 20.0))
                .child(div().flex_1().min_w_0().truncate().child(label))
        };
        // Resting on any other item folds "Move to" away again.
        let plain = |id: &'static str, name: &str, label: &str| {
            item(id, name, label.to_owned().into()).on_hover(cx.listener(
                |this, hovered: &bool, _, cx| {
                    if *hovered {
                        this.set_move_to(false, cx);
                    }
                },
            ))
        };
        let act = |act: Act| {
            cx.listener(
                move |this: &mut Self, _: &gpui::ClickEvent, _: &mut Window, cx| {
                    this.context_act(act, cx)
                },
            )
        };
        let reply = |kind: Kind| {
            cx.listener(move |this: &mut Self, _: &gpui::ClickEvent, window, cx| {
                this.context_reply(kind, window, cx)
            })
        };
        let separator = || div().my(px(6.0)).h(px(1.0)).bg(rgba(th.divider));

        let folders = menu.move_to.then(|| {
            // Search results can be anywhere, so every folder is offered.
            let current = self.listed_folder();
            let folders = self
                .account()
                .map(|a| self.tree.folders_of(a))
                .unwrap_or_default();
            div()
                .absolute()
                .map(|d| {
                    if flip_y {
                        d.bottom(px(-8.0))
                    } else {
                        d.top(px(-8.0))
                    }
                })
                .map(|d| {
                    if flip_x {
                        d.right(px(MENU_WIDTH - 4.0))
                    } else {
                        d.left(px(MENU_WIDTH - 4.0))
                    }
                })
                .w(px(FOLDERS_WIDTH))
                .py(px(8.0))
                // It hangs outside the menu, so it must hide the scrim below
                // itself: else pressing a folder closes the menu first and
                // the move never happens.
                .occlude()
                .map(|d| raised(d, th, 8.0, 3.0))
                .child(
                    div()
                        .id("context-folders")
                        .max_h(px(360.0))
                        .overflow_y_scroll()
                        .children(
                            folders
                                .into_iter()
                                .filter(|(id, ..)| Some(*id) != current)
                                .map(|(id, name, role)| {
                                    div()
                                        .id(("context-move", id.0 as usize))
                                        .h(px(ITEM_HEIGHT))
                                        .px(px(16.0))
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(16.0))
                                        .cursor_pointer()
                                        .hover(|s| s.bg(rgba(th.hover)))
                                        .menu_key(th)
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            this.context_act(Act::MoveTo(id), cx)
                                        }))
                                        .child(icon(super::nav::role_icon(role), th.text_dim, 20.0))
                                        .child(div().truncate().child(name))
                                }),
                        ),
                )
        });
        let move_to = item("context-move-to", "move-to", tr!("menu-move-to").into())
            .relative()
            .when(menu.move_to, |d| d.bg(rgba(th.hover)))
            .on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                if *hovered {
                    this.set_move_to(true, cx);
                }
            }))
            .on_click(cx.listener(|this, _, _, cx| {
                let on = this.context_menu.as_ref().is_some_and(|m| !m.move_to);
                this.set_move_to(on, cx);
            }))
            .child(icon("chevron-right", th.text_dim, 18.0))
            .children(folders);
        let sender = row.sender.clone();
        let find = (!sender.is_empty()).then(|| {
            let name = if row.correspondent.is_empty() || row.correspondent.starts_with("To: ") {
                sender.clone()
            } else {
                row.correspondent.clone()
            };
            item(
                "context-find",
                "search",
                tr!("menu-find-from", name = name.as_str()).into(),
            )
            .on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                if *hovered {
                    this.set_move_to(false, cx);
                }
            }))
            .on_click(cx.listener(move |this, _, window, cx| {
                this.close_context_menu(cx);
                this.search_for(format!("from:{sender}"), window, cx);
            }))
        });

        let role = self.folder_role();
        let drafts = role == Role::Drafts;
        let archives = !matches!(
            role,
            Role::Drafts | Role::Sent | Role::Junk | Role::Trash | Role::Archive | Role::All
        );
        let spam = !matches!(role, Role::Drafts | Role::Sent | Role::Trash);
        let snoozed = role == Role::Snoozed;
        let snoozes = !matches!(
            role,
            Role::Drafts | Role::Sent | Role::Trash | Role::Junk | Role::Snoozed
        );
        // Trash restores to the inbox; archived mail goes back there too.
        let to_inbox = matches!(role, Role::Trash | Role::Archive | Role::All)
            .then(|| {
                self.account()
                    .and_then(|a| self.tree.role_folder(a, Role::Inbox))
            })
            .flatten();
        let list = div()
            .key_context(crate::widgets::MENU_CONTEXT)
            .w(px(MENU_WIDTH))
            .py(px(8.0))
            .flex()
            .flex_col()
            .map(|d| raised(d, th, 8.0, 3.0))
            .text_size(px(14.0))
            .text_color(rgba(th.text))
            // What fits the folder, as in webmail: no answering drafts, no
            // archiving what is archived, in Spam or in Trash, and Trash
            // restores and deletes for good.
            .when(!drafts, |d| {
                d.child(
                    plain("context-reply", "reply", &tr!("menu-reply"))
                        .on_click(reply(Kind::Reply)),
                )
                .child(
                    plain("context-reply-all", "reply-all", &tr!("menu-reply-all"))
                        .on_click(reply(Kind::ReplyAll)),
                )
                .child(
                    plain("context-forward", "forward", &tr!("menu-forward"))
                        .on_click(reply(Kind::Forward)),
                )
                .child(separator())
            })
            .when(archives, |d| {
                d.child(
                    plain("context-archive", "archive", &tr!("menu-archive"))
                        .on_click(act(Act::Archive)),
                )
            })
            .when_some(to_inbox, |d, inbox| {
                d.child(
                    plain("context-to-inbox", "inbox", &tr!("menu-move-to-inbox"))
                        .on_click(act(Act::MoveTo(inbox))),
                )
            })
            .child(if role == Role::Trash {
                plain("context-delete", "trash", &tr!("menu-delete-forever"))
                    .on_click(act(Act::Delete))
            } else {
                plain("context-delete", "trash", &tr!("menu-delete")).on_click(act(Act::Delete))
            })
            .when(spam, |d| {
                d.child(
                    plain("context-spam", "junk", &self.spam_label(true)).on_click(act(Act::Spam)),
                )
            })
            .child(if row.unread {
                plain("context-read", "mark-read", &tr!("menu-mark-read"))
                    .on_click(act(Act::Read(true)))
            } else {
                plain("context-read", "mail", &tr!("menu-mark-unread"))
                    .on_click(act(Act::Read(false)))
            })
            .when(snoozes, |d| {
                d.child(
                    plain("context-snooze", "schedule", &tr!("menu-snooze"))
                        .on_click(cx.listener(|this, _, _, cx| this.context_snooze(cx))),
                )
            })
            .when(snoozed, |d| {
                d.child(
                    plain("context-unsnooze", "inbox", &tr!("menu-unsnooze"))
                        .on_click(act(Act::Unsnooze)),
                )
            })
            .child(if row.flagged {
                plain("context-star", "star", &tr!("menu-unstar")).on_click(act(Act::Star(false)))
            } else {
                plain("context-star", "star", &tr!("menu-star")).on_click(act(Act::Star(true)))
            })
            .child(if row.important {
                plain("context-important", "important", &tr!("menu-not-important"))
                    .on_click(act(Act::Important(false)))
            } else {
                plain("context-important", "important", &tr!("menu-important"))
                    .on_click(act(Act::Important(true)))
            })
            .child(if row.pinned {
                plain("context-pin", "pin-filled", &tr!("menu-unpin"))
                    .on_click(act(Act::Pin(false)))
            } else {
                plain("context-pin", "pin", &tr!("menu-pin")).on_click(act(Act::Pin(true)))
            })
            .child(separator())
            .child(move_to)
            .when_some(find, |d, find| d.child(separator()).child(find))
            // A conversation window has no list to right-click.
            .child(separator())
            .child(
                plain(
                    "context-new-window",
                    "open-external",
                    &tr!("menu-new-window"),
                )
                .on_click(cx.listener(|this, _, _, cx| {
                    let key = this.context_menu.as_ref().map(|m| m.key);
                    this.close_context_menu(cx);
                    if let Some(ix) = this.entries.iter().position(|e| Some(e.key) == key) {
                        this.open_in_window(ix, cx);
                    }
                })),
            )
            .with_animation(
                ("context-menu", menu.ix),
                Animation::new(Duration::from_millis(140)).with_easing(ease_out_quint()),
                |el, t| el.opacity(t).mt(px(-4.0 * (1.0 - t))),
            );
        let close = || {
            cx.listener(|this: &mut Self, _: &gpui::MouseDownEvent, _, cx| {
                this.close_context_menu(cx)
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
                            .id("context-scrim")
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
                            .position(menu.at)
                            .snap_to_window_with_margin(px(8.0))
                            .child(div().occlude().child(list)),
                    )
                    .with_priority(4),
                )
                .into_any_element(),
        )
    }
}
