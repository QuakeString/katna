// SPDX-License-Identifier: GPL-3.0-or-later

//! The right-click menu of the mail list, as in webmail: reply and
//! forward, archive, delete, read, snooze and star, then submenus: "Move
//! to" with the folders, "Follow up" (tasks, notes, meetings, calls) and
//! "More" (spam, importance, pin), "Find emails from" the sender and
//! "Make a rule…" (the rule editor, filled in with the sender). It
//! opens where the pointer is and always fits the window. It acts on the
//! ticked lines when the clicked line is one of them, else on the clicked
//! line.
//!
//! The Calendar page has its own menus in the same card
//! (`calendar/menu.rs`): on a free time or day, an event and a task.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Instant;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Div, ElementId, FontWeight, KeyDownEvent,
    MouseButton, Pixels, Point, SharedString, Stateful, Window, deferred, div, ease_out_quint,
    point, prelude::*, rgba,
};
use katna_ui::anchored;
use katna_ui::px;
use katna_ui::tokens::duration;
use katna_ui::unpx;

use katna_i18n::tr;

use super::MenuKey;
use super::apps::App;
use super::compose::Kind;
use super::folder_pick::{PickFrom, PickMode};
use super::sheet::{Fill, Sheet};
use super::{Act, MailWindow};
use crate::data::{EntryKey, Row};
use crate::sidebar::Role;
use crate::theme::Theme;
use crate::widgets::{icon, raised};

const MENU_WIDTH: f32 = 264.0;
const SUB_WIDTH: f32 = 240.0;
const ITEM_HEIGHT: f32 = 36.0;
/// A row of a menu that rises as a sheet on a phone.
const SHEET_ITEM_HEIGHT: f32 = 48.0;
/// Items come no closer than this in a short window; below it the menu
/// scrolls.
const MIN_ITEM_HEIGHT: f32 = 28.0;
const PADDING: f32 = 8.0;
const RULE_MARGIN: f32 = 6.0;
const RULE_HEIGHT: f32 = 2.0 * RULE_MARGIN + 1.0;
/// Room the menu keeps from the window's edges.
const MARGIN: f32 = 8.0;

use super::calendar::menu::CalTarget;
use katna_core::config::SoundEvent;

/// The open right-click menu.
pub(super) struct ContextMenu {
    what: MenuFor,
    /// Where the pointer was, in the window.
    at: Point<Pixels>,
    /// The open submenu.
    open: Option<Sub>,
    /// How tall the menu stands without a submenu, as last drawn.
    height: Cell<f32>,
    /// On a phone, a chat bubble's menu rises as a sheet.
    sheet: Sheet,
    /// When it began to fade out, after it closed. It stays drawn,
    /// fading and out of reach, until the fade ends.
    closing: Option<Instant>,
}

impl Clone for ContextMenu {
    /// What a closed menu leaves behind to fade. Its sheet starts fresh.
    fn clone(&self) -> Self {
        ContextMenu {
            what: self.what.clone(),
            at: self.at,
            open: self.open,
            height: self.height.clone(),
            sheet: Sheet::new(),
            closing: self.closing,
        }
    }
}

/// What a right-click menu is for.
#[derive(Clone)]
enum MenuFor {
    /// Line `ix` of the mail list.
    Mail {
        ix: usize,
        key: EntryKey,
        row: Rc<Row>,
    },
    Calendar(CalTarget),
    /// A color scheme's card in Settings > Appearance > Colors.
    Scheme(&'static str),
    /// A signature in the list in Settings > Compose > Signatures.
    Signature(u32),
    /// The sounds to pick for an event in Settings > Notifications.
    Sound(SoundEvent),
    /// A mail's bubble in the chat view, or one of its files.
    Bubble(katna_store::MessageId, Option<usize>),
}

impl ContextMenu {
    fn new(what: MenuFor, at: Point<Pixels>) -> Self {
        ContextMenu {
            what,
            at,
            open: None,
            height: Cell::new(0.0),
            sheet: Sheet::rising(),
            closing: None,
        }
    }

    /// The mail list line it is for.
    fn line(&self) -> Option<(usize, EntryKey)> {
        match &self.what {
            MenuFor::Mail { ix, key, .. } => Some((*ix, *key)),
            MenuFor::Calendar(_)
            | MenuFor::Scheme(_)
            | MenuFor::Signature(_)
            | MenuFor::Sound(_)
            | MenuFor::Bubble(..) => None,
        }
    }
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
        let Some(mut row) = row.map(|r| self.with_pending(r)) else {
            return;
        };
        // Several ticked lines: each pair offers what makes them all alike.
        if self.checked.contains(&key) && self.checked.len() > 1 {
            let pairs = super::list::Pairs::of(&self.checked_rows());
            let mut all = (*row).clone();
            all.unread = pairs.read;
            all.flagged = !pairs.star;
            all.important = !pairs.important;
            all.pinned = !pairs.pin;
            row = Rc::new(all);
        }
        self.menu = None;
        self.selected = Some(ix);
        self.context_menu = Some(ContextMenu::new(MenuFor::Mail { ix, key, row }, at));
        cx.notify();
    }

    /// Opens the menu for a thing on the Calendar page.
    pub(super) fn open_calendar_context_menu(
        &mut self,
        target: CalTarget,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.menu = None;
        self.context_menu = Some(ContextMenu::new(MenuFor::Calendar(target), at));
        cx.notify();
    }

    /// Opens the menu of the color scheme `id`'s card.
    pub(super) fn open_scheme_menu(
        &mut self,
        id: &'static str,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.menu = None;
        self.context_menu = Some(ContextMenu::new(MenuFor::Scheme(id), at));
        cx.notify();
    }

    /// Opens the menu of signature `id` in the list of signatures.
    pub(super) fn open_signature_menu(
        &mut self,
        id: u32,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.menu = None;
        self.context_menu = Some(ContextMenu::new(MenuFor::Signature(id), at));
        cx.notify();
    }

    /// Opens the menu of sounds for `event`.
    pub(super) fn open_sound_context_menu(
        &mut self,
        event: SoundEvent,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.menu = None;
        self.context_menu = Some(ContextMenu::new(MenuFor::Sound(event), at));
        cx.notify();
    }

    /// Opens the menu of mail `id`'s bubble in the chat view.
    pub(super) fn open_chat_context_menu(
        &mut self,
        id: katna_store::MessageId,
        file: Option<usize>,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.menu = None;
        self.context_menu = Some(ContextMenu::new(MenuFor::Bubble(id, file), at));
        cx.notify();
    }

    /// Closes the menu; returns the Calendar thing it was for and where
    /// it opened.
    pub(super) fn take_calendar_target(&mut self) -> Option<(CalTarget, Point<Pixels>)> {
        let menu = self.take_context_menu()?;
        match menu.what {
            MenuFor::Calendar(target) => Some((target, menu.at)),
            MenuFor::Mail { .. }
            | MenuFor::Scheme(_)
            | MenuFor::Signature(_)
            | MenuFor::Sound(_)
            | MenuFor::Bubble(..) => None,
        }
    }

    pub(super) fn close_context_menu(&mut self, cx: &mut Context<Self>) {
        if self.take_context_menu().is_some() {
            cx.notify();
        }
    }

    /// The right-click menu, unless it is closed and fading out.
    pub(super) fn open_context_menu_ref(&self) -> Option<&ContextMenu> {
        self.context_menu.as_ref().filter(|m| m.closing.is_none())
    }

    fn open_context_menu_mut(&mut self) -> Option<&mut ContextMenu> {
        self.context_menu.as_mut().filter(|m| m.closing.is_none())
    }

    /// Closes the menu and returns it. What stays behind fades out
    /// (`motion` FAST), then goes.
    pub(super) fn take_context_menu(&mut self) -> Option<ContextMenu> {
        let menu = self.open_context_menu_mut()?;
        menu.closing = Some(Instant::now());
        Some(menu.clone())
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

    /// Closes the menu; returns the mail list line it was for.
    fn take_context_line(&mut self) -> Option<(usize, EntryKey)> {
        self.take_context_menu().and_then(|m| m.line())
    }

    fn context_act(&mut self, act: Act, cx: &mut Context<Self>) {
        let Some((_, key)) = self.take_context_line() else {
            return;
        };
        let keys = self.context_targets(key);
        self.act(act, keys, cx);
    }

    /// Opens the snooze menu where the right-click menu was.
    fn context_snooze(&mut self, cx: &mut Context<Self>) {
        let Some(menu) = self.take_context_menu() else {
            return;
        };
        let Some((_, key)) = menu.line() else {
            return;
        };
        let keys = self.context_targets(key);
        self.open_snooze_menu(keys, menu.at, cx);
    }

    /// Opens the conversation and starts the answer in it.
    fn context_reply(&mut self, kind: Kind, window: &mut Window, cx: &mut Context<Self>) {
        let Some((ix, _)) = self.take_context_line() else {
            return;
        };
        self.open(ix, window, cx);
        self.open_compose(kind, None, window, cx);
    }

    /// Opens submenu `sub` of the right-click menu, or closes the open one.
    fn open_context_sub(&mut self, sub: Option<Sub>, cx: &mut Context<Self>) {
        let Some(menu) = self.open_context_menu_mut() else {
            return;
        };
        if menu.open == sub {
            return;
        }
        menu.open = sub;
        let account = match &menu.what {
            MenuFor::Mail { row, .. } => Some(row.account),
            _ => None,
        };
        let line = menu.line();
        // Move to and Label as open on their search box.
        let mode = match sub {
            Some(Sub::MoveTo) => Some(PickMode::Move),
            Some(Sub::LabelAs) => Some(PickMode::Label),
            _ => None,
        };
        match (mode, account, line) {
            (Some(mode), Some(account), Some((_, key))) => {
                let keys = self.context_targets(key);
                self.open_folder_pick(mode, PickFrom::Context, account, keys, cx);
            }
            _ => {
                if self.folder_pick.as_ref().map(|p| p.from) == Some(PickFrom::Context) {
                    self.folder_pick = None;
                }
            }
        }
        cx.notify();
    }

    /// Escape: closes the open submenu before the menu. Returns whether it
    /// did.
    pub(super) fn context_menu_back(&mut self, cx: &mut Context<Self>) -> bool {
        match self.open_context_menu_mut() {
            Some(menu) if menu.open.is_some() => {
                menu.open = None;
                if self.folder_pick.as_ref().map(|p| p.from) == Some(PickFrom::Context) {
                    self.folder_pick = None;
                }
                cx.notify();
                true
            }
            _ => false,
        }
    }

    pub(super) fn render_context_menu(
        &mut self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // A closed menu fades out, then goes.
        let fade = katna_ui::motion::time(duration::FAST);
        let closing = self.context_menu.as_ref().and_then(|m| m.closing);
        if let Some(since) = closing {
            if since.elapsed() >= fade || cx.reduce_motion() {
                self.context_menu = None;
                return None;
            }
            window.request_animation_frame();
        }
        let menu = self.context_menu.as_ref()?;
        // A phone's long press on a chat bubble: the menu rises from the
        // bottom, its rows tall enough for a finger.
        if let MenuFor::Bubble(id, file) = menu.what
            && self.layout.shape.is_phone()
        {
            // The sheet sinks on its own.
            if closing.is_some() {
                return None;
            }
            let rows = self.bubble_menu_rows(id, file, SHEET_ITEM_HEIGHT, th, cx);
            let body = div()
                .pb(px(PADDING))
                .flex()
                .flex_col()
                .text_size(px(15.0))
                .text_color(rgba(th.text))
                .children(rows.els)
                .into_any_element();
            return self.bottom_sheet(
                "bubble-sheet",
                &menu.sheet,
                Fill::Menu,
                body,
                |this| this.context_menu.as_mut().map(|m| &mut m.sheet),
                |this, cx| this.close_context_menu(cx),
                th,
                window,
                cx,
            );
        }
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        // The menu, and a submenu, get shorter with the window, their
        // items closer together, down to MIN_ITEM_HEIGHT; below that they
        // scroll.
        let room = vh - 2.0 * MARGIN;
        let squeeze = |rows: &Rows, room: f32| {
            let rules = rows.rules as f32 * RULE_HEIGHT + rows.fixed;
            ((room - 2.0 * PADDING - rules) / rows.items.max(1) as f32)
                .clamp(MIN_ITEM_HEIGHT, ITEM_HEIGHT)
        };
        let (mut main, mut parents) = self.context_main_rows(ITEM_HEIGHT, th, cx);
        let mut rh = ITEM_HEIGHT;
        if main.height() > room {
            rh = squeeze(&main, room);
            (main, parents) = self.context_main_rows(rh, th, cx);
        }

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
        // Right to left the menu opens toward the left of the pointer and
        // its submenus on its left: worked out as if mirrored, then
        // mirrored back.
        let rtl = self.layout.shape.rtl;
        let mirror = |x: f32, width: f32| if rtl { vw - x - width } else { x };
        let x = mirror(place(mirror(at.0, 0.0), MENU_WIDTH, vw), MENU_WIDTH);
        let y = place(at.1, fits(main.height()), vh);

        let open = menu.open.map(|sub| {
            // In the menu's place, under the row back.
            let room = if drills {
                room - rh - RULE_HEIGHT
            } else {
                room
            };
            let mut rows = self.context_sub_rows(sub, ITEM_HEIGHT, th, cx);
            if rows.height() > room {
                rows = self.context_sub_rows(sub, squeeze(&rows, room), th, cx);
            }
            (sub, rows)
        });
        let (content, card_y, beside) = match open {
            Some((sub, rows)) if drills => {
                let mut card = Rows::new(rh);
                card.item(
                    menu_row("context-back", "back", sub.label(), th, rh)
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
                let start = mirror(x, MENU_WIDTH);
                let sub_x = if start + MENU_WIDTH - 4.0 + SUB_WIDTH <= vw - MARGIN {
                    start + MENU_WIDTH - 4.0
                } else {
                    (start - SUB_WIDTH + 4.0).max(MARGIN)
                };
                let sub_x = mirror(sub_x, SUB_WIDTH);
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
                    .screen_corner()
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
                                .on_key_down(cx.listener(
                                    |this, event: &KeyDownEvent, window, cx| {
                                        if !event.keystroke.modifiers.modified()
                                            && katna_ui::direction::arrow(
                                                &event.keystroke.key,
                                                katna_ui::direction::is_rtl(window),
                                            ) == "left"
                                            && !this.folder_pick_typing(window, cx)
                                            && this.context_menu_back(cx)
                                        {
                                            cx.stop_propagation();
                                        }
                                    },
                                ))
                                .children(rows.els)
                                .map(|d| match closing {
                                    // Out of reach while it fades.
                                    Some(since) => {
                                        let t = (since.elapsed().as_secs_f32()
                                            / fade.as_secs_f32())
                                        .min(1.0);
                                        d.opacity(1.0 - ease_out_quint()(t))
                                            .child(
                                                div()
                                                    .absolute()
                                                    .top_0()
                                                    .left_0()
                                                    .size_full()
                                                    .occlude(),
                                            )
                                            .into_any_element()
                                    }
                                    None => d
                                        .with_animation(
                                            id,
                                            Animation::new(fade).with_easing(ease_out_quint()),
                                            |el, t| el.opacity(t).mt(px(-4.0 * (1.0 - t))),
                                        )
                                        .into_any_element(),
                                }),
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
        let base = match &menu.what {
            MenuFor::Mail { ix, .. } => format!("context-menu-{ix}"),
            MenuFor::Calendar(target) => format!("context-menu-{}", target.key()),
            MenuFor::Scheme(id) => format!("context-menu-scheme-{id}"),
            MenuFor::Signature(id) => format!("context-menu-signature-{id}"),
            MenuFor::Sound(event) => format!("context-menu-sound-{event:?}"),
            MenuFor::Bubble(id, file) => format!(
                "context-menu-bubble-{}-{}",
                id.0,
                file.map_or(-1, |f| f as i64)
            ),
        };
        let key = match (menu.open, drills) {
            (Some(sub), true) => format!("{base}-{sub:?}"),
            _ => base,
        };
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                // A fading menu lets the window be clicked at once.
                .when(closing.is_none(), |d| {
                    d.child(
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
                                .on_mouse_down(
                                    MouseButton::Right,
                                    cx.listener(
                                        |this, event: &gpui::MouseDownEvent, window, cx| {
                                            this.close_context_menu(cx);
                                            super::popovers::pass_right_press(event, window);
                                        },
                                    ),
                                ),
                        )
                        .with_priority(3),
                    )
                })
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

    /// The menu's own items at `rh` each, with where each submenu's row
    /// starts.
    fn context_main_rows(
        &self,
        rh: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> (Rows, Vec<(Sub, f32)>) {
        match self.context_menu.as_ref().map(|m| &m.what) {
            Some(MenuFor::Mail { row, .. }) => self.mail_menu_rows(row, rh, th, cx),
            Some(MenuFor::Calendar(target)) => self.calendar_menu_rows(target, rh, th, cx),
            Some(MenuFor::Scheme(id)) => (self.scheme_menu_rows(id, rh, th, cx), Vec::new()),
            Some(MenuFor::Signature(id)) => (self.signature_menu_rows(*id, rh, th, cx), Vec::new()),
            Some(MenuFor::Sound(event)) => (self.sound_menu_rows(*event, rh, th, cx), Vec::new()),
            Some(MenuFor::Bubble(id, file)) => {
                (self.bubble_menu_rows(*id, *file, rh, th, cx), Vec::new())
            }
            None => (Rows::new(rh), Vec::new()),
        }
    }

    /// An item of the menu itself: resting on it folds an open submenu
    /// away.
    pub(super) fn context_item(
        &self,
        id: impl Into<ElementId>,
        name: &str,
        label: impl Into<SharedString>,
        rh: f32,
        th: &Theme,
        cx: &Context<Self>,
    ) -> Stateful<Div> {
        menu_row(id, name, label.into(), th, rh).on_hover(cx.listener(
            |this, hovered: &bool, _, cx| {
                if *hovered {
                    this.open_context_sub(None, cx);
                }
            },
        ))
    }

    /// An item whose icon is in the accent: AI's sparkle, as Rephrase's.
    fn context_item_tinted(
        &self,
        id: impl Into<ElementId>,
        name: &str,
        label: impl Into<SharedString>,
        rh: f32,
        th: &Theme,
        cx: &Context<Self>,
    ) -> Stateful<Div> {
        menu_row_with(id, icon(name, th.accent, 20.0), label.into(), th, rh).on_hover(cx.listener(
            |this, hovered: &bool, _, cx| {
                if *hovered {
                    this.open_context_sub(None, cx);
                }
            },
        ))
    }

    /// The mail list's menu.
    fn mail_menu_rows(
        &self,
        row: &Row,
        rh: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> (Rows, Vec<(Sub, f32)>) {
        let plain = |id: &'static str, name: &str, label: &str| {
            self.context_item(id, name, label.to_owned(), rh, th, cx)
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
        let mut main = Rows::new(rh);
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
            if self.summaries_on() {
                main.item(
                    self.context_item_tinted(
                        "context-summarize",
                        "sparkle",
                        tr!("summary-summarize"),
                        rh,
                        th,
                        cx,
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        // Where the right-click was: the card opens there.
                        let at = this.context_menu.as_ref().map(|m| m.at);
                        if let Some((ix, key)) = this.take_context_line() {
                            this.summarize_line(ix, key, at, cx);
                        }
                    })),
                );
            }
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
            plain("context-read", "mark-unread", &tr!("menu-mark-unread"))
                .on_click(act(Act::Read(false)))
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
        let gmail = self.tree.is_gmail(row.account);
        for sub in Sub::ALL {
            if sub == Sub::FollowUp && drafts || sub == Sub::LabelAs && !gmail {
                continue;
            }
            let top = main.item(self.context_parent(sub, rh, th, cx));
            parents.push((sub, top));
        }
        main.rule(th);
        let sender = row.sender.clone();
        if !sender.is_empty() {
            let name = if row.correspondent.is_empty()
                || row.correspondent.starts_with(&crate::data::to_prefix())
            {
                sender.clone()
            } else {
                row.correspondent.clone()
            };
            main.item(
                self.context_item(
                    "context-find",
                    "search",
                    tr!("menu-find-from", name = name.as_str()),
                    rh,
                    th,
                    cx,
                )
                .on_click({
                    let sender = sender.clone();
                    cx.listener(move |this, _, window, cx| {
                        this.close_context_menu(cx);
                        this.search_for(format!("from:{sender}"), window, cx);
                    })
                }),
            );
            // The rule editor, filled in with the sender.
            let account = row.account;
            main.item(
                self.context_item(
                    "context-make-rule",
                    "filter",
                    tr!("menu-make-rule"),
                    rh,
                    th,
                    cx,
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.make_rule_from(name.clone(), sender.clone(), account, window, cx)
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
                let key = this.take_context_line().map(|(_, key)| key);
                cx.notify();
                if let Some(ix) = this.entries.iter().position(|e| Some(e.key) == key) {
                    this.open_in_window(ix, cx);
                }
            })),
        );
        (main, parents)
    }

    /// The row that opens submenu `sub`: as the pointer comes to it, or on
    /// a click where submenus open in the menu's place.
    pub(super) fn context_parent(
        &self,
        sub: Sub,
        rh: f32,
        th: &Theme,
        cx: &Context<Self>,
    ) -> Stateful<Div> {
        let open = self
            .context_menu
            .as_ref()
            .is_some_and(|m| m.open == Some(sub));
        menu_row(sub.id(), sub.icon(), sub.label(), th, rh)
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
            .on_key_down(cx.listener(move |this, event: &KeyDownEvent, window, cx| {
                if !event.keystroke.modifiers.modified()
                    && katna_ui::direction::arrow(
                        &event.keystroke.key,
                        katna_ui::direction::is_rtl(window),
                    ) == "right"
                {
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
    fn context_sub_rows(&self, sub: Sub, rh: f32, th: &Theme, cx: &Context<Self>) -> Rows {
        let row = match self.context_menu.as_ref().map(|m| &m.what) {
            Some(MenuFor::Mail { row, .. }) => Some(row.clone()),
            Some(MenuFor::Calendar(target)) => {
                return self.calendar_sub_rows(target, sub, rh, th, cx);
            }
            Some(
                MenuFor::Scheme(_)
                | MenuFor::Signature(_)
                | MenuFor::Sound(_)
                | MenuFor::Bubble(..),
            )
            | None => None,
        };
        let act = |act: Act| {
            cx.listener(
                move |this: &mut Self, _: &gpui::ClickEvent, _: &mut Window, cx| {
                    this.context_act(act, cx)
                },
            )
        };
        let mut rows = Rows::new(rh);
        match sub {
            Sub::MoveTo | Sub::LabelAs => {
                // Search results can be anywhere, so every folder is offered.
                if let Some(pick) = self.folder_pick_in(PickFrom::Context, sub.pick_mode()) {
                    for (el, h) in self.render_folder_pick(pick, rh, th, cx) {
                        rows.line(el, h, h == rh);
                    }
                    if let Some((el, h)) = self.render_always_move(pick, th, cx) {
                        rows.line(el, h, false);
                    }
                }
            }
            Sub::FollowUp => {
                if self.app_on(App::Tasks) {
                    rows.item(
                        menu_row(
                            "context-add-to-tasks",
                            "tasks",
                            tr!("menu-add-to-tasks").into(),
                            th,
                            rh,
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            let Some((_, key)) = this.take_context_line() else {
                                return;
                            };
                            let keys = this.context_targets(key);
                            this.add_to_tasks_from(keys, cx);
                        })),
                    );
                }
                if self.app_on(App::Notes) {
                    rows.item(
                        menu_row(
                            "context-add-note",
                            "notes",
                            tr!("menu-add-note").into(),
                            th,
                            rh,
                        )
                        .on_click(cx.listener(|this, _, window, cx| {
                            let Some((_, key)) = this.take_context_line() else {
                                return;
                            };
                            let keys = this.context_targets(key);
                            this.add_note_from(keys, window, cx);
                        })),
                    );
                }
                if self.app_on(App::Calendar) {
                    rows.item(
                        menu_row(
                            "context-schedule-meeting",
                            "calendar",
                            tr!("menu-schedule-meeting").into(),
                            th,
                            rh,
                        )
                        .on_click(cx.listener(|this, _, window, cx| {
                            let Some((_, key)) = this.take_context_line() else {
                                return;
                            };
                            this.schedule_meeting_from(Some(key), window, cx);
                        })),
                    );
                }
                rows.item(
                    menu_row(
                        "context-start-call",
                        "video",
                        tr!("menu-start-call").into(),
                        th,
                        rh,
                    )
                    .on_click(cx.listener(|this, _, window, cx| {
                        let Some((_, key)) = this.take_context_line() else {
                            return;
                        };
                        this.start_call_from(Some(key), window, cx);
                    })),
                );
            }
            Sub::More => {
                let role = self.folder_role();
                if !matches!(role, Role::Drafts | Role::Sent | Role::Trash) {
                    rows.item(
                        menu_row("context-spam", "junk", self.spam_label(true).into(), th, rh)
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
                        rh,
                    )
                    .on_click(act(Act::Important(false)))
                } else {
                    menu_row(
                        "context-important",
                        "important",
                        tr!("menu-important").into(),
                        th,
                        rh,
                    )
                    .on_click(act(Act::Important(true)))
                });
                let pinned = row.as_ref().is_some_and(|r| r.pinned);
                rows.item(if pinned {
                    menu_row(
                        "context-pin",
                        "pin-filled",
                        tr!("menu-unpin").into(),
                        th,
                        rh,
                    )
                    .on_click(act(Act::Pin(false)))
                } else {
                    menu_row("context-pin", "pin", tr!("menu-pin").into(), th, rh)
                        .on_click(act(Act::Pin(true)))
                });
                // Mute the conversation, or its sender (§15.1.1).
                if let Some(key) = self
                    .context_menu
                    .as_ref()
                    .and_then(|m| m.line())
                    .map(|l| l.1)
                {
                    let muted = self.lines_muted(&self.context_targets(key));
                    rows.item(
                        menu_row(
                            "context-mute",
                            if muted { "bell" } else { "bell-off" },
                            if muted {
                                tr!("quiet-unmute-conversation")
                            } else {
                                tr!("quiet-mute-conversation")
                            }
                            .into(),
                            th,
                            rh,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            let Some((_, key)) = this.take_context_line() else {
                                return;
                            };
                            let keys = this.context_targets(key);
                            this.mute_lines(keys, !muted, cx);
                        })),
                    );
                }
                if let Some(sender) = row
                    .as_ref()
                    .map(|r| r.sender.clone())
                    .filter(|s| !s.is_empty())
                {
                    let muted = self.sender_muted(&sender);
                    rows.item(
                        menu_row(
                            "context-mute-sender",
                            if muted { "bell" } else { "bell-off" },
                            if muted {
                                tr!("quiet-unmute-sender")
                            } else {
                                tr!("quiet-mute-sender")
                            }
                            .into(),
                            th,
                            rh,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.take_context_menu();
                            this.mute_sender(sender.clone(), !muted, cx);
                        })),
                    );
                }
            }
            // The Calendar's, above.
            Sub::Color | Sub::Calendar | Sub::Answer | Sub::Date => {}
        }
        rows
    }
}

/// A submenu of the right-click menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Sub {
    /// The folders and labels.
    MoveTo,
    /// Gmail: the labels to put on or take off.
    LabelAs,
    /// Tasks, notes, meetings and calls from the mail.
    FollowUp,
    /// Spam, importance and pinning.
    More,
    /// An event's color.
    Color,
    /// The calendars an event can move to.
    Calendar,
    /// Yes, No or Maybe to an invitation.
    Answer,
    /// Another day for a task.
    Date,
}

impl Sub {
    const ALL: [Sub; 4] = [Sub::MoveTo, Sub::LabelAs, Sub::FollowUp, Sub::More];

    /// What its search does, for Move to and Label as.
    fn pick_mode(self) -> PickMode {
        if self == Sub::LabelAs {
            PickMode::Label
        } else {
            PickMode::Move
        }
    }

    fn id(self) -> &'static str {
        match self {
            Sub::MoveTo => "context-move-to",
            Sub::LabelAs => "context-label-as",
            Sub::FollowUp => "context-follow-up",
            Sub::More => "context-more",
            Sub::Color => "context-color",
            Sub::Calendar => "context-calendar",
            Sub::Answer => "context-answer",
            Sub::Date => "context-date",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Sub::MoveTo => "move-to",
            Sub::LabelAs => "tag",
            Sub::FollowUp => "event",
            Sub::More => "more",
            Sub::Color => "contrast",
            Sub::Calendar => "move-to",
            Sub::Answer => "check-circle",
            Sub::Date => "today",
        }
    }

    fn label(self) -> SharedString {
        match self {
            Sub::MoveTo => tr!("menu-move-to"),
            Sub::LabelAs => tr!("menu-label-as"),
            Sub::FollowUp => tr!("menu-follow-up"),
            Sub::More => tr!("menu-more"),
            Sub::Color => tr!("calendar-menu-color"),
            Sub::Calendar => tr!("menu-move-to"),
            Sub::Answer => tr!("calendar-going"),
            Sub::Date => tr!("tasks-date"),
        }
        .into()
    }
}

/// A menu's rows and how tall they stand.
pub(super) struct Rows {
    els: Vec<AnyElement>,
    /// Each item's height.
    row: f32,
    /// From the menu's top to below the last row.
    h: f32,
    items: usize,
    rules: usize,
    /// The height of lines that keep theirs in a short window.
    fixed: f32,
}

impl Rows {
    pub(super) fn new(row: f32) -> Self {
        Rows {
            els: Vec::new(),
            row,
            h: PADDING,
            items: 0,
            rules: 0,
            fixed: 0.0,
        }
    }

    /// Adds a row; returns where it starts, from the menu's top.
    pub(super) fn item(&mut self, row: impl IntoElement) -> f32 {
        let top = self.h;
        self.els.push(row.into_any_element());
        self.h += self.row;
        self.items += 1;
        top
    }

    /// Adds a line `h` tall: an item when `item` (its height follows the
    /// others'), else one that keeps its height.
    pub(super) fn line(&mut self, el: AnyElement, h: f32, item: bool) {
        self.els.push(el);
        self.h += h;
        if item {
            self.items += 1;
        } else {
            self.fixed += h;
        }
    }

    pub(super) fn rule(&mut self, th: &Theme) {
        self.els.push(
            div()
                .my(px(RULE_MARGIN))
                .h(px(1.0))
                .bg(rgba(th.divider))
                .into_any_element(),
        );
        self.h += RULE_HEIGHT;
        self.rules += 1;
    }

    /// The whole menu's height, padding below included.
    fn height(&self) -> f32 {
        self.h + PADDING
    }
}

/// One item of the menu: its icon and label.
pub(super) fn menu_row(
    id: impl Into<ElementId>,
    icon_name: &str,
    label: SharedString,
    th: &Theme,
    height: f32,
) -> Stateful<Div> {
    menu_row_with(id, icon(icon_name, th.text_dim, 20.0), label, th, height)
}

/// An item led by `lead`, 20 px wide, in place of an icon.
pub(super) fn menu_row_with(
    id: impl Into<ElementId>,
    lead: AnyElement,
    label: SharedString,
    th: &Theme,
    height: f32,
) -> Stateful<Div> {
    div()
        .id(id)
        .flex_none()
        .h(px(height))
        .pl(px(16.0))
        .pr(px(24.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(16.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .menu_key(th)
        .child(
            div()
                .flex_none()
                .size(px(20.0))
                .flex()
                .items_center()
                .justify_center()
                .child(lead),
        )
        .child(div().flex_1().min_w_0().truncate().child(label))
}
