// SPDX-License-Identifier: GPL-3.0-or-later

//! Labels on the Contacts page (`docs/ARCHITECTURE.md` §8.6), as in Google
//! Contacts: a person's "Label" menu ticks labels on and off or makes a
//! new one; a label's own menu (its ⋮, or a right-click in the column)
//! renames it, deletes it or writes to everyone on it. Every change shows
//! at once, goes to the daemon, and has an Undo.

use std::collections::BTreeSet;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Entity, Focusable, FontWeight, MouseButton,
    Pixels, Point, SharedString, Subscription, Window, anchored, deferred, div, ease_out_quint,
    prelude::*, rgba,
};
use katna_i18n::tr;
use katna_store::StoredCard;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::{InputEvent, TextInput, px, unpx};

use super::MailWindow;
use super::apps::App;
use super::contacts_page::View;
use crate::daemon::{self, Command};
use crate::format;
use crate::theme::{Theme, fade};
use crate::widgets::{FocusRing, ScaledEdge, elevation, filled_button, icon, raised};

const MENU_WIDTH: f32 = 260.0;
const DIALOG_WIDTH: f32 = 400.0;

/// A labels menu and where it opened.
#[derive(Debug, Clone, PartialEq)]
pub(super) enum LabelMenu {
    /// The open person's labels, to tick on and off.
    Person { at: Point<Pixels> },
    /// One label's own menu.
    Label { name: String, at: Point<Pixels> },
}

/// The dialog naming a new label, or renaming one.
pub(super) struct LabelDialog {
    /// The label renamed; `None` for a new label for the open person.
    old: Option<String>,
    name: Entity<TextInput>,
    _subscription: Subscription,
    busy: bool,
    error: Option<String>,
    closing: bool,
    shown: Spring,
}

/// `labels` with `name` put on or taken off.
pub(super) fn toggled(labels: &[String], name: &str, on: bool) -> Vec<String> {
    let mut out: Vec<String> = labels
        .iter()
        .filter(|l| !l.eq_ignore_ascii_case(name))
        .cloned()
        .collect();
    if on {
        out.push(name.to_owned());
    }
    out
}

fn has(labels: &[String], name: &str) -> bool {
    labels.iter().any(|l| l.eq_ignore_ascii_case(name))
}

impl MailWindow {
    /// The labels shown for the person whose first card is `key`.
    pub(super) fn person_labels(&self, key: Option<i64>, labels: &[String]) -> Vec<String> {
        key.and_then(|k| self.contacts.shown_labels.get(&k))
            .cloned()
            .unwrap_or_else(|| labels.to_vec())
    }

    pub(super) fn open_label_menu(&mut self, menu: LabelMenu, cx: &mut Context<Self>) {
        self.contacts.label_menu = Some(menu);
        cx.notify();
    }

    /// Puts label `name` on the open person, or takes it off: on every
    /// card of theirs, so each account keeps it.
    fn set_person_label(&mut self, name: &str, on: bool, cx: &mut Context<Self>) {
        let Some(open) = &mut self.contacts.open else {
            return;
        };
        let Some(Ok(cards)) = &mut open.cards else {
            return;
        };
        let mut changes: Vec<(i64, Vec<String>)> = Vec::new();
        let mut undo: Vec<(i64, Vec<String>)> = Vec::new();
        for card in cards.iter_mut() {
            if has(&card.labels, name) == on {
                continue;
            }
            undo.push((card.id, card.labels.clone()));
            card.labels = toggled(&card.labels, name, on);
            changes.push((card.id, card.labels.clone()));
        }
        if changes.is_empty() {
            return;
        }
        let key = open.person.ids.first().copied();
        let shown = toggled(
            &self.person_labels(
                key,
                &self
                    .contacts
                    .open
                    .as_ref()
                    .map_or_else(Vec::new, |o| o.person.labels.clone()),
            ),
            name,
            on,
        );
        if let Some(key) = key {
            self.contacts.shown_labels.insert(key, shown);
        }
        let text = if on {
            tr!("contacts-label-added", name = name.to_owned())
        } else {
            tr!("contacts-label-removed", name = name.to_owned())
        };
        self.send_contact_labels(
            Command::ContactLabels(changes),
            text,
            Command::ContactLabels(undo),
            cx,
        );
        cx.notify();
    }

    /// Sends `change`; says `done` with an Undo that sends `undo`, or why
    /// it failed.
    fn send_contact_labels(
        &mut self,
        change: Command,
        done: String,
        undo: Command,
        cx: &mut Context<Self>,
    ) {
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::send(&connection, &change).await
                })
                .await;
            this.update(cx, |this, cx| match result {
                Ok(()) => this.show_snackbar(done, Some(undo), cx),
                Err(err) => {
                    // Shows the labels as they are again.
                    this.contacts.shown_labels.clear();
                    this.load_contacts(cx);
                    this.show_snackbar(format::sentence(&err), None, cx);
                }
            })
            .ok();
        })
        .detach();
    }

    /// Opens the dialog for a new label (`None`) or renaming `old`.
    pub(super) fn start_label_dialog(
        &mut self,
        old: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.contacts.label_menu = None;
        let accent = rgba(self.theme(window).accent).into();
        let text = old.clone().unwrap_or_default();
        let name = cx.new(|cx| {
            let mut input = TextInput::new(tr!("contacts-label-name"), cx);
            input.set_accent(accent);
            input.set_text(text, cx);
            input
        });
        let subscription =
            cx.subscribe_in(
                &name,
                window,
                |this, _, event: &InputEvent, _, cx| match event {
                    InputEvent::Submit => this.submit_label_dialog(cx),
                    InputEvent::Cancel => this.close_label_dialog(cx),
                    InputEvent::Changed => {
                        if let Some(dialog) = &mut this.contacts.label_dialog {
                            dialog.error = None;
                        }
                        cx.notify();
                    }
                },
            );
        window.focus(&name.focus_handle(cx), cx);
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.contacts.label_dialog = Some(LabelDialog {
            old,
            name,
            _subscription: subscription,
            busy: false,
            error: None,
            closing: false,
            shown,
        });
        cx.notify();
    }

    fn close_label_dialog(&mut self, cx: &mut Context<Self>) {
        if let Some(dialog) = &mut self.contacts.label_dialog
            && !dialog.busy
        {
            dialog.closing = true;
            dialog.shown.set(0.0);
        }
        cx.notify();
    }

    fn label_dialog_name(&self, cx: &Context<Self>) -> Option<String> {
        let dialog = self.contacts.label_dialog.as_ref()?;
        let name = dialog.name.read(cx).text().trim().to_owned();
        let unchanged = dialog.old.as_deref().is_some_and(|old| old == name);
        (!dialog.busy && !name.is_empty() && !unchanged).then_some(name)
    }

    fn submit_label_dialog(&mut self, cx: &mut Context<Self>) {
        let Some(name) = self.label_dialog_name(cx) else {
            return;
        };
        let Some(dialog) = &mut self.contacts.label_dialog else {
            return;
        };
        let Some(old) = dialog.old.clone() else {
            // A new label goes on the open person.
            self.close_label_dialog(cx);
            self.set_person_label(&name, true, cx);
            return;
        };
        dialog.busy = true;
        let connection = self.daemon.clone();
        let change = Command::RenameContactLabel(old.clone(), name.clone());
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::send(&connection, &change).await
                })
                .await;
            this.update(cx, |this, cx| {
                if let Some(dialog) = &mut this.contacts.label_dialog {
                    dialog.busy = false;
                }
                match result {
                    Ok(()) => {
                        this.close_label_dialog(cx);
                        if this.contacts.view == View::Label(old.clone()) {
                            this.contacts.view = View::Label(name.clone());
                        }
                        this.show_snackbar(
                            tr!("contacts-label-renamed", name = name.clone()),
                            Some(Command::RenameContactLabel(name, old)),
                            cx,
                        );
                    }
                    Err(err) => {
                        if let Some(dialog) = &mut this.contacts.label_dialog {
                            dialog.error = Some(format::sentence(&err));
                        }
                        cx.notify();
                    }
                }
            })
            .ok();
        })
        .detach();
        cx.notify();
    }

    /// Takes label `name` away; its people stay. Undo puts it back on
    /// each of their cards.
    fn delete_contact_label(&mut self, name: String, cx: &mut Context<Self>) {
        self.contacts.label_menu = None;
        let Some(Ok(book)) = &self.contacts.book else {
            return;
        };
        let ids: Vec<i64> = book
            .people
            .iter()
            .filter(|p| has(&p.labels, &name))
            .flat_map(|p| p.ids.iter().copied())
            .collect();
        if self.contacts.view == View::Label(name.clone()) {
            self.contacts.view = View::Contacts;
        }
        let paths = self.paths.clone();
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let gone = name.clone();
            let result = cx
                .background_executor()
                .spawn(async move {
                    let cards = crate::data::saved_cards(&paths, &ids)?;
                    let undo: Vec<(i64, Vec<String>)> = cards
                        .into_iter()
                        .filter(|c: &StoredCard| has(&c.labels, &gone))
                        .map(|c| (c.id, c.labels))
                        .collect();
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::send(
                        &connection,
                        &Command::RenameContactLabel(gone, String::new()),
                    )
                    .await?;
                    Ok::<_, String>(undo)
                })
                .await;
            this.update(cx, |this, cx| match result {
                Ok(undo) => this.show_snackbar(
                    tr!("contacts-label-deleted", name = name),
                    Some(Command::ContactLabels(undo)),
                    cx,
                ),
                Err(err) => this.show_snackbar(format::sentence(&err), None, cx),
            })
            .ok();
        })
        .detach();
        cx.notify();
    }

    /// Opens a new message to everyone with label `name`.
    pub(super) fn email_label(&mut self, name: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.contacts.label_menu = None;
        let Some(Ok(book)) = &self.contacts.book else {
            return;
        };
        let mut seen = BTreeSet::new();
        let to: Vec<String> = book
            .people
            .iter()
            .filter(|p| {
                !self
                    .contacts
                    .hidden
                    .contains(&p.ids.first().copied().unwrap_or(0))
            })
            .filter(|p| has(&self.person_labels(p.ids.first().copied(), &p.labels), name))
            .filter_map(|p| p.emails.first().cloned())
            .filter(|e| seen.insert(e.to_lowercase()))
            .collect();
        if to.is_empty() {
            self.show_snackbar(tr!("contacts-label-no-email"), None, cx);
            return;
        }
        self.open_app(App::Mail, cx);
        let mail = crate::mailto::Mailto {
            to,
            ..Default::default()
        };
        self.open_mailto(mail, window, cx);
    }

    /// The labels menu, over everything.
    pub(super) fn render_label_menu(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let menu = self.contacts.label_menu.clone()?;
        let item = |id: SharedString, lead: AnyElement, label: String| {
            div()
                .id(id)
                .flex_none()
                .h(px(36.0))
                .pl(px(16.0))
                .pr(px(24.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .child(lead)
                .child(div().flex_1().min_w_0().truncate().child(label))
        };
        let separator = || div().my(px(6.0)).h(px(1.0)).bg(rgba(th.divider));
        let (at, items): (Point<Pixels>, Vec<AnyElement>) =
            match menu {
                LabelMenu::Person { at } => {
                    let open = self.contacts.open.as_ref()?;
                    let key = open.person.ids.first().copied();
                    let mine = self.person_labels(key, &open.person.labels);
                    let all: Vec<String> = match &self.contacts.book {
                        Some(Ok(book)) => book.labels.iter().map(|l| l.name.clone()).collect(),
                        _ => Vec::new(),
                    };
                    let mut items: Vec<AnyElement> = vec![
                        div()
                            .px(px(16.0))
                            .pb(px(6.0))
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_faint))
                            .child(tr!("contacts-label-menu"))
                            .into_any_element(),
                    ];
                    let rows =
                        all.into_iter().enumerate().map(|(ix, name)| {
                            let on = has(&mine, &name);
                            item(
                                format!("contact-label-{ix}").into(),
                                crate::widgets::checkbox(
                                    ("contact-label-box", ix),
                                    crate::widgets::Check::from(on),
                                    th,
                                ),
                                name.clone(),
                            )
                            .on_click(cx.listener(
                                move |this, _, _, cx| this.set_person_label(&name, !on, cx),
                            ))
                        });
                    items.push(
                        div()
                            .id("contact-label-list")
                            .max_h(px(320.0))
                            .overflow_y_scroll()
                            .flex()
                            .flex_col()
                            .children(rows)
                            .into_any_element(),
                    );
                    items.push(separator().into_any_element());
                    items.push(
                        item(
                            "contact-label-new".into(),
                            icon("add", th.text_dim, 20.0),
                            tr!("contacts-label-new"),
                        )
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.start_label_dialog(None, window, cx)
                        }))
                        .into_any_element(),
                    );
                    (at, items)
                }
                LabelMenu::Label { name, at } => {
                    let rename = name.clone();
                    let email = name.clone();
                    let delete = name.clone();
                    let items = vec![
                        item(
                            "contact-label-rename".into(),
                            icon("compose", th.text_dim, 20.0),
                            tr!("contacts-label-rename"),
                        )
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.start_label_dialog(Some(rename.clone()), window, cx)
                        }))
                        .into_any_element(),
                        item(
                            "contact-label-email".into(),
                            icon("mail", th.text_dim, 20.0),
                            tr!("contacts-label-email"),
                        )
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.email_label(&email, window, cx)
                        }))
                        .into_any_element(),
                        separator().into_any_element(),
                        item(
                            "contact-label-delete".into(),
                            icon("trash", th.text_dim, 20.0),
                            tr!("contacts-label-delete"),
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.delete_contact_label(delete.clone(), cx)
                        }))
                        .into_any_element(),
                    ];
                    (at, items)
                }
            };
        let list = div()
            .w(px(MENU_WIDTH))
            .py(px(8.0))
            .flex()
            .flex_col()
            .map(|d| raised(d, th, 8.0, 3.0))
            .text_size(px(14.0))
            .text_color(rgba(th.text))
            .children(items)
            .with_animation(
                "contact-label-menu",
                Animation::new(std::time::Duration::from_millis(140)).with_easing(ease_out_quint()),
                |el, t| el.opacity(t).mt(px(-4.0 * (1.0 - t))),
            );
        let close = || {
            cx.listener(|this: &mut Self, _: &gpui::MouseDownEvent, _, cx| {
                this.contacts.label_menu = None;
                cx.notify();
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
                            .id("contact-label-scrim")
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
                            .position(at)
                            .snap_to_window_with_margin(px(8.0))
                            .child(div().occlude().child(list)),
                    )
                    .with_priority(4),
                )
                .into_any_element(),
        )
    }

    /// The New label or Rename label dialog.
    pub(super) fn render_label_dialog(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let dialog = self.contacts.label_dialog.as_mut()?;
        let t = dialog.shown.tick(window, reduce);
        if dialog.closing && dialog.shown.settled() {
            self.contacts.label_dialog = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let ready = self.label_dialog_name(cx).is_some();
        let dialog = self.contacts.label_dialog.as_ref()?;
        let renaming = dialog.old.is_some();
        let focus = dialog.name.focus_handle(cx);
        let focused = focus.is_focused(window);
        let field = div()
            .id("contact-label-name")
            .mt(px(20.0))
            .h(px(44.0))
            .px(px(14.0))
            .flex()
            .items_center()
            .rounded(px(8.0))
            .border_px(2.0)
            .border_color(rgba(if focused {
                th.accent
            } else {
                fade(th.text_faint, 0.8)
            }))
            .text_size(px(15.0))
            .cursor_text()
            .on_click(move |_, window, cx| window.focus(&focus, cx))
            .child(div().flex_1().child(dialog.name.clone()));
        let error = dialog.error.clone().map(|err| {
            div()
                .mt(px(12.0))
                .text_size(px(13.0))
                .text_color(rgba(th.error))
                .child(err)
        });
        let busy = dialog.busy;
        let body = div()
            .flex()
            .flex_col()
            .px(px(24.0))
            .pt(px(24.0))
            .pb(px(20.0))
            .child(
                div()
                    .text_size(px(22.0))
                    .line_height(px(30.0))
                    .child(if renaming {
                        tr!("contacts-label-rename")
                    } else {
                        tr!("contacts-label-new")
                    }),
            )
            .child(field)
            .children(error)
            .child(
                div()
                    .mt(px(24.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(div().flex_1())
                    .child(
                        div()
                            .id("contact-label-cancel")
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
                            .on_click(cx.listener(|this, _, _, cx| this.close_label_dialog(cx)))
                            .child(tr!("contacts-edit-cancel")),
                    )
                    .child(
                        filled_button(
                            "contact-label-save",
                            if busy {
                                tr!("contacts-edit-saving")
                            } else {
                                tr!("contacts-edit-save")
                            },
                            th,
                        )
                        .focus_ring_filled(th)
                        .when(!ready, |d| d.opacity(0.45).cursor_default())
                        .on_click(cx.listener(|this, _, _, cx| this.submit_label_dialog(cx))),
                    ),
            );
        let viewport = window.viewport_size();
        let vw = unpx(viewport.width);
        let card = div()
            .id("contact-label-dialog")
            .track_focus(&self.dialog_focus)
            .map(|d| super::popovers::keep_tab_inside(d, &self.dialog_focus))
            .occlude()
            .w(px(DIALOG_WIDTH.min(vw - 32.0)))
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(px(super::PANEL_RADIUS))
            .bg(rgba(th.surface))
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
                        .id("contact-label-dialog-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| this.close_label_dialog(cx))),
                )
                .child(div().opacity(t).mt(px(lerp(24.0, 0.0, t))).child(card))
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_go_on_and_off_once() {
        let labels = vec!["Family".to_owned(), "Work".to_owned()];
        assert_eq!(toggled(&labels, "family", false), ["Work"]);
        assert_eq!(
            toggled(&labels, "Friends", true),
            ["Family", "Work", "Friends"]
        );
        assert_eq!(toggled(&labels, "work", true), ["Family", "work"]);
        assert!(has(&labels, "WORK"));
    }
}
