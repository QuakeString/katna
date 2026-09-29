// SPDX-License-Identifier: GPL-3.0-or-later

//! The right-click menu of the mail list, as in webmail: reply and
//! forward, archive, delete, read, snooze and star, then submenus: "Move
//! to" with the folders, "Follow up" (tasks, notes, meetings, calls) and
//! "More" (spam, importance, pin), and "Find emails from" the sender. It
//! opens where the pointer is and always fits the window. It acts on the
//! ticked lines when the clicked line is one of them, else on the clicked
//! line.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Div, ElementId, FontWeight, KeyDownEvent,
    MouseButton, Pixels, Point, SharedString, Stateful, Window, anchored, deferred, div,
    ease_out_quint, point, prelude::*, rgba,
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
const SUB_WIDTH: f32 = 240.0;
const ITEM_HEIGHT: f32 = 36.0;
const PADDING: f32 = 8.0;
const RULE_MARGIN: f32 = 6.0;
const RULE_HEIGHT: f32 = 2.0 * RULE_MARGIN + 1.0;
/// Room the menu keeps from the window's edges.
const MARGIN: f32 = 8.0;

/// The open right-click menu.
pub(super) struct ContextMenu {
    ix: usize,
    key: EntryKey,
    /// Where the pointer was, in the window.
    at: Point<Pixels>,
    row: Rc<Row>,
    /// The open submenu.
    open: Option<Sub>,
    /// How tall the menu stands without a submenu, as last drawn.
    height: Cell<f32>,
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
            open: None,
            height: Cell::new(0.0),
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

    /// Opens submenu `sub` of the right-click menu, or closes the open one.
    fn open_context_sub(&mut self, sub: Option<Sub>, cx: &mut Context<Self>) {
        if let Some(menu) = &mut self.context_menu
            && menu.open != sub
        {
            menu.open = sub;
            cx.notify();
        }
    }

    /// Escape: closes the open submenu before the menu. Returns whether it
    /// did.
    pub(super) fn context_menu_back(&mut self, cx: &mut Context<Self>) -> bool {
        match &mut self.context_menu {
            Some(menu) if menu.open.is_some() => {
                menu.open = None;
                cx.notify();
                true
            }
            _ => false,
        }
    }

    pub(super) fn render_context_menu(
        &self,
        th: &Theme,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let menu = self.context_menu.as_ref()?;
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let row = &menu.row;
        // Resting on an item of the menu itself folds an open submenu away.
        let plain = |id: &'static str, name: &str, label: &str| {
            menu_row(id, name, label.to_owned().into(), th).on_hover(cx.listener(
                |this, hovered: &bool, _, cx| {
                    if *hovered {
                        this.open_context_sub(None, cx);
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

        let role = self.folder_role();
        let drafts = role == Role::Drafts;
        let archives = !matches!(
            role,
            Role::Drafts | Role::Sent | Role::Junk | Role::Trash | Role::Archive | Role::All
        );
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

        // What fits the folder, as in webmail: no answering drafts, no
        // archiving what is archived, in Spam or in Trash, and Trash
        // restores and deletes for good. The everyday actions stay on top;
        // the rarer ones wait in submenus, so the menu fits a short window.
        let mut main = Rows::new();
        if !drafts {
            main.item(
                plain("context-reply", "reply", &tr!("menu-reply")).on_click(reply(Kind::Reply)),
            );
            main.item(
                plain("context-reply-all", "reply-all", &tr!("menu-reply-all"))
                    .on_click(reply(Kind::ReplyAll)),
            );
            main.item(
                plain("context-forward", "forward", &tr!("menu-forward"))
                    .on_click(reply(Kind::Forward)),
            );
            main.rule(th);
        }
        if archives {
            main.item(
                plain("context-archive", "archive", &tr!("menu-archive"))
                    .on_click(act(Act::Archive)),
            );
        }
        if let Some(inbox) = to_inbox {
            main.item(
                plain("context-to-inbox", "inbox", &tr!("menu-move-to-inbox"))
                    .on_click(act(Act::MoveTo(inbox))),
            );
        }
        main.item(if role == Role::Trash {
            plain("context-delete", "trash", &tr!("menu-delete-forever")).on_click(act(Act::Delete))
        } else {
            plain("context-delete", "trash", &tr!("menu-delete")).on_click(act(Act::Delete))
        });
        main.item(if row.unread {
            plain("context-read", "mark-read", &tr!("menu-mark-read"))
                .on_click(act(Act::Read(true)))
        } else {
            plain("context-read", "mail", &tr!("menu-mark-unread")).on_click(act(Act::Read(false)))
        });
        if snoozes {
            main.item(
                plain("context-snooze", "schedule", &tr!("menu-snooze"))
                    .on_click(cx.listener(|this, _, _, cx| this.context_snooze(cx))),
            );
        }
        if snoozed {
            main.item(
                plain("context-unsnooze", "inbox", &tr!("menu-unsnooze"))
                    .on_click(act(Act::Unsnooze)),
            );
        }
        main.item(if row.flagged {
            plain("context-star", "star", &tr!("menu-unstar")).on_click(act(Act::Star(false)))
        } else {
            plain("context-star", "star", &tr!("menu-star")).on_click(act(Act::Star(true)))
        });
        main.rule(th);
        let mut parents = Vec::new();
        for sub in Sub::ALL {
            if sub == Sub::FollowUp && drafts {
                continue;
            }
            let top = main.item(self.context_parent(sub, th, cx));
            parents.push((sub, top));
        }
        main.rule(th);
        let sender = row.sender.clone();
        if !sender.is_empty() {
            let name = if row.correspondent.is_empty() || row.correspondent.starts_with("To: ") {
                sender.clone()
            } else {
                row.correspondent.clone()
            };
            main.item(
                menu_row(
                    "context-find",
                    "search",
                    tr!("menu-find-from", name = name.as_str()).into(),
                    th,
                )
                .on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                    if *hovered {
                        this.open_context_sub(None, cx);
                    }
                }))
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.close_context_menu(cx);
                    this.search_for(format!("from:{sender}"), window, cx);
                })),
            );
        }
        main.item(
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
        );

        // Where the window has no room for a submenu beside the menu, or
        // for the whole menu, a submenu opens in the menu's place under a
        // row back, as the main menu does on a phone.
        let fits = |h: f32| h.min(vh - 2.0 * MARGIN);
        menu.height.set(main.height());
        let drills = self.context_menu_drills(window);
        // Opens where the pointer is, flipping left or up where there is no
        // room, else pushed back in from the edge.
        let place = |at: f32, size: f32, room: f32| {
            if at + size <= room - MARGIN {
                at
            } else if at - size >= MARGIN {
                at - size
            } else {
                (room - MARGIN - size).max(MARGIN)
            }
        };
        let at = (unpx(menu.at.x), unpx(menu.at.y));
        let x = place(at.0, MENU_WIDTH, vw);
        let y = place(at.1, fits(main.height()), vh);

        let open = menu
            .open
            .map(|sub| (sub, self.context_sub_rows(sub, th, cx)));
        let (content, card_y, beside) = match open {
            Some((sub, rows)) if drills => {
                let mut card = Rows::new();
                card.item(
                    menu_row("context-back", "back", sub.label(), th)
                        .font_weight(FontWeight::MEDIUM)
                        .on_click(cx.listener(|this, _, _, cx| {
                            cx.stop_propagation();
                            this.open_context_sub(None, cx);
                        })),
                );
                card.rule(th);
                card.els.extend(rows.els);
                card.h += rows.h - PADDING;
                let h = fits(card.height());
                (card, y.min(vh - MARGIN - h).max(MARGIN), None)
            }
            Some((sub, rows)) => {
                // Beside its row, the first item level with it.
                let top = parents
                    .iter()
                    .find(|(s, _)| *s == sub)
                    .map_or(0.0, |(_, top)| *top);
                let sub_x = if x + MENU_WIDTH - 4.0 + SUB_WIDTH <= vw - MARGIN {
                    x + MENU_WIDTH - 4.0
                } else {
                    (x - SUB_WIDTH + 4.0).max(MARGIN)
                };
                let h = fits(rows.height());
                let sub_y = (y + top - PADDING).min(vh - MARGIN - h).max(MARGIN);
                (main, y, Some((sub, rows, sub_x, sub_y)))
            }
            None => (main, y, None),
        };

        let card = |id: SharedString, rows: Rows, x: f32, y: f32, width: f32, priority: usize| {
            let h = fits(rows.height());
            deferred(
                anchored()
                    .position(point(px(x), px(y)))
                    .snap_to_window_with_margin(px(MARGIN))
                    .child(
                        div().occlude().child(
                            div()
                                .id(id.clone())
                                .key_context(crate::widgets::MENU_CONTEXT)
                                .w(px(width))
                                .max_h(px(h))
                                .overflow_y_scroll()
                                .py(px(PADDING))
                                .flex()
                                .flex_col()
                                .map(|d| raised(d, th, 8.0, 3.0))
                                .text_size(px(14.0))
                                .text_color(rgba(th.text))
                                // Left goes back from a submenu.
                                .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                                    if !event.keystroke.modifiers.modified()
                                        && event.keystroke.key == "left"
                                        && this.context_menu_back(cx)
                                    {
                                        cx.stop_propagation();
                                    }
                                }))
                                .children(rows.els)
                                .with_animation(
                                    id,
                                    Animation::new(Duration::from_millis(140))
                                        .with_easing(ease_out_quint()),
                                    |el, t| el.opacity(t).mt(px(-4.0 * (1.0 - t))),
                                ),
                        ),
                    ),
            )
            .with_priority(priority)
        };
        let close = || {
            cx.listener(|this: &mut Self, _: &gpui::MouseDownEvent, _, cx| {
                this.close_context_menu(cx)
            })
        };
        let key = match (menu.open, drills) {
            (Some(sub), true) => format!("context-menu-{}-{sub:?}", menu.ix),
            _ => format!("context-menu-{}", menu.ix),
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
                .child(card(key.into(), content, x, card_y, MENU_WIDTH, 4))
                .children(beside.map(|(sub, rows, sub_x, sub_y)| {
                    card(
                        format!("context-sub-{sub:?}").into(),
                        rows,
                        sub_x,
                        sub_y,
                        SUB_WIDTH,
                        5,
                    )
                }))
                .into_any_element(),
        )
    }

    /// The row that opens submenu `sub`: as the pointer comes to it, or on
    /// a click where submenus open in the menu's place.
    fn context_parent(&self, sub: Sub, th: &Theme, cx: &Context<Self>) -> Stateful<Div> {
        let open = self
            .context_menu
            .as_ref()
            .is_some_and(|m| m.open == Some(sub));
        menu_row(sub.id(), sub.icon(), sub.label(), th)
            .when(open, |d| d.bg(rgba(th.hover)))
            .on_hover(cx.listener(move |this, hovered: &bool, window, cx| {
                if *hovered && !this.context_menu_drills(window) {
                    this.open_context_sub(Some(sub), cx);
                }
            }))
            .on_click(cx.listener(move |this, _, _, cx| {
                cx.stop_propagation();
                let on = this
                    .context_menu
                    .as_ref()
                    .is_some_and(|m| m.open != Some(sub));
                this.open_context_sub(on.then_some(sub), cx);
            }))
            .on_key_down(cx.listener(move |this, event: &KeyDownEvent, _, cx| {
                if !event.keystroke.modifiers.modified() && event.keystroke.key == "right" {
                    this.open_context_sub(Some(sub), cx);
                    cx.stop_propagation();
                }
            }))
            .child(icon("chevron-right", th.text_dim, 18.0))
    }

    /// Whether submenus open in the menu's place: the window is too narrow
    /// for one beside the menu, or too short for the whole menu.
    fn context_menu_drills(&self, window: &Window) -> bool {
        let viewport = window.viewport_size();
        let height = self.context_menu.as_ref().map_or(0.0, |m| m.height.get());
        unpx(viewport.width) < MENU_WIDTH + SUB_WIDTH + 2.0 * MARGIN
            || unpx(viewport.height) - 2.0 * MARGIN < height
    }

    /// The items of submenu `sub`.
    fn context_sub_rows(&self, sub: Sub, th: &Theme, cx: &Context<Self>) -> Rows {
        let row = self.context_menu.as_ref().map(|m| m.row.clone());
        let act = |act: Act| {
            cx.listener(
                move |this: &mut Self, _: &gpui::ClickEvent, _: &mut Window, cx| {
                    this.context_act(act, cx)
                },
            )
        };
        let mut rows = Rows::new();
        match sub {
            Sub::MoveTo => {
                // Search results can be anywhere, so every folder is offered.
                let current = self.listed_folder();
                let folders = self
                    .account()
                    .map(|a| self.tree.folders_of(a))
                    .unwrap_or_default();
                for (id, name, role) in folders.into_iter().filter(|(id, ..)| Some(*id) != current)
                {
                    rows.item(
                        menu_row(
                            ("context-move", id.0 as usize),
                            super::nav::role_icon(role),
                            name.into(),
                            th,
                        )
                        .on_click(act(Act::MoveTo(id))),
                    );
                }
            }
            Sub::FollowUp => {
                rows.item(
                    menu_row(
                        "context-add-to-tasks",
                        "tasks",
                        tr!("menu-add-to-tasks").into(),
                        th,
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        let Some(menu) = this.context_menu.take() else {
                            return;
                        };
                        let keys = this.context_targets(menu.key);
                        this.add_to_tasks_from(keys, cx);
                    })),
                );
                rows.item(
                    menu_row("context-add-note", "notes", tr!("menu-add-note").into(), th)
                        .on_click(cx.listener(|this, _, window, cx| {
                            let Some(menu) = this.context_menu.take() else {
                                return;
                            };
                            let keys = this.context_targets(menu.key);
                            this.add_note_from(keys, window, cx);
                        })),
                );
                rows.item(
                    menu_row(
                        "context-schedule-meeting",
                        "calendar",
                        tr!("menu-schedule-meeting").into(),
                        th,
                    )
                    .on_click(cx.listener(|this, _, window, cx| {
                        let Some(menu) = this.context_menu.take() else {
                            return;
                        };
                        this.schedule_meeting_from(Some(menu.key), window, cx);
                    })),
                );
                rows.item(
                    menu_row(
                        "context-start-call",
                        "video",
                        tr!("menu-start-call").into(),
                        th,
                    )
                    .on_click(cx.listener(|this, _, window, cx| {
                        let Some(menu) = this.context_menu.take() else {
                            return;
                        };
                        this.start_call_from(Some(menu.key), window, cx);
                    })),
                );
            }
            Sub::More => {
                let role = self.folder_role();
                if !matches!(role, Role::Drafts | Role::Sent | Role::Trash) {
                    rows.item(
                        menu_row("context-spam", "junk", self.spam_label(true).into(), th)
                            .on_click(act(Act::Spam)),
                    );
                }
                let important = row.as_ref().is_some_and(|r| r.important);
                rows.item(if important {
                    menu_row(
                        "context-important",
                        "important",
                        tr!("menu-not-important").into(),
                        th,
                    )
                    .on_click(act(Act::Important(false)))
                } else {
                    menu_row(
                        "context-important",
                        "important",
                        tr!("menu-important").into(),
                        th,
                    )
                    .on_click(act(Act::Important(true)))
                });
                let pinned = row.as_ref().is_some_and(|r| r.pinned);
                rows.item(if pinned {
                    menu_row("context-pin", "pin-filled", tr!("menu-unpin").into(), th)
                        .on_click(act(Act::Pin(false)))
                } else {
                    menu_row("context-pin", "pin", tr!("menu-pin").into(), th)
                        .on_click(act(Act::Pin(true)))
                });
            }
        }
        rows
    }
}

/// A submenu of the right-click menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Sub {
    /// The folders and labels.
    MoveTo,
    /// Tasks, notes, meetings and calls from the mail.
    FollowUp,
    /// Spam, importance and pinning.
    More,
}

impl Sub {
    const ALL: [Sub; 3] = [Sub::MoveTo, Sub::FollowUp, Sub::More];

    fn id(self) -> &'static str {
        match self {
            Sub::MoveTo => "context-move-to",
            Sub::FollowUp => "context-follow-up",
            Sub::More => "context-more",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Sub::MoveTo => "move-to",
            Sub::FollowUp => "event",
            Sub::More => "more",
        }
    }

    fn label(self) -> SharedString {
        match self {
            Sub::MoveTo => tr!("menu-move-to"),
            Sub::FollowUp => tr!("menu-follow-up"),
            Sub::More => tr!("menu-more"),
        }
        .into()
    }
}

/// A menu's rows and how tall they stand.
struct Rows {
    els: Vec<AnyElement>,
    /// From the menu's top to below the last row.
    h: f32,
}

impl Rows {
    fn new() -> Self {
        Rows {
            els: Vec::new(),
            h: PADDING,
        }
    }

    /// Adds a row; returns where it starts, from the menu's top.
    fn item(&mut self, row: impl IntoElement) -> f32 {
        let top = self.h;
        self.els.push(row.into_any_element());
        self.h += ITEM_HEIGHT;
        top
    }

    fn rule(&mut self, th: &Theme) {
        self.els.push(
            div()
                .my(px(RULE_MARGIN))
                .h(px(1.0))
                .bg(rgba(th.divider))
                .into_any_element(),
        );
        self.h += RULE_HEIGHT;
    }

    /// The whole menu's height, padding below included.
    fn height(&self) -> f32 {
        self.h + PADDING
    }
}

/// One item of the menu: its icon and label.
fn menu_row(
    id: impl Into<ElementId>,
    icon_name: &str,
    label: SharedString,
    th: &Theme,
) -> Stateful<Div> {
    div()
        .id(id)
        .flex_none()
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
        .child(icon(icon_name, th.text_dim, 20.0))
        .child(div().flex_1().min_w_0().truncate().child(label))
}
