// SPDX-License-Identifier: GPL-3.0-or-later

//! The Accounts part of the Settings page: the accounts Katna syncs, with
//! a way to remove each one, and a way to delete everything Katna keeps.
//! Both ask first in a dialog that says in red what goes, and that mail on
//! the server stays. The daemon does the deleting.

use gpui::{
    AnyElement, Context, Div, Entity, Focusable, FontWeight, Stateful, Subscription, Window, div,
    prelude::*, px, rgba,
};
use katna_core::config::AccountsShown;
use katna_core::{Account, AccountKind, Config};
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::{InputEvent, TextInput};

use super::settings::Change;
use super::{Listing, MailWindow, keymap};
use crate::daemon;
use crate::data::Mail;
use crate::theme::{Theme, fade};
use crate::widgets::{FocusRing, elevation, icon};

const WIDTH: f32 = 500.0;
/// What to type before everything is deleted.
const CONFIRM_WORD: &str = "delete";

/// A question before deleting.
pub(super) struct Danger {
    what: What,
    busy: bool,
    error: Option<String>,
    closing: bool,
    shown: Spring,
}

enum What {
    RemoveAccount(Account),
    DeleteAll {
        typed: Entity<TextInput>,
        _subscription: Subscription,
    },
}

impl MailWindow {
    pub(super) fn accounts_section(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let accounts: Vec<Account> = self
            .accounts
            .iter()
            .filter(|a| a.kind.is_mail())
            .cloned()
            .collect();
        let list = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .children(accounts.into_iter().enumerate().map(|(ix, account)| {
                let name = if account.display_name.trim().is_empty() {
                    account.address.clone()
                } else {
                    account.display_name.clone()
                };
                let about = div()
                    .flex_grow(1.0)
                    .flex_basis(px(180.0))
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .truncate()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .child(name.clone()),
                    )
                    .child(
                        div()
                            .truncate()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_faint))
                            .child(format!(
                                "{} \u{b7} {}",
                                account.address,
                                kind_name(account.kind)
                            )),
                    );
                let id = account.id;
                let avatar = self.person_avatar(&name, &account.address, 36.0);
                let buttons = div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap(px(8.0))
                    .when(self.remote.has_own_picture(id), |d| {
                        d.child(
                            text_button(("account-picture-reset", ix), "Use desktop picture", th)
                                .map(|d| self.page_control(d, th, cx))
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.reset_account_picture(id, cx)
                                })),
                        )
                    })
                    .child(
                        text_button(("account-picture", ix), "Change picture", th)
                            .map(|d| self.page_control(d, th, cx))
                            .on_click(
                                cx.listener(move |this, _, _, cx| {
                                    this.pick_account_picture(id, cx)
                                }),
                            ),
                    )
                    .child(
                        danger_button(("account-remove", ix), "Remove", false, th)
                            .map(|d| self.page_control(d, th, cx))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.ask(What::RemoveAccount(account.clone()), cx)
                            })),
                    );
                // The buttons go below the name, together, where the row is
                // narrow.
                div()
                    .py(px(10.0))
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap(px(12.0))
                    .child(avatar)
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .min_h(px(36.0))
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .items_center()
                            .gap_x(px(12.0))
                            .gap_y(px(4.0))
                            .child(about)
                            .child(buttons),
                    )
            }))
            .when(self.accounts.is_empty(), |d| {
                d.child(
                    div()
                        .py(px(8.0))
                        .text_size(px(14.0))
                        .text_color(rgba(th.text_faint))
                        .child("No accounts yet."),
                )
            })
            .child(
                div().pt(px(8.0)).flex().child(
                    crate::widgets::outlined_button("account-add-page", "Add an account", th)
                        .map(|d| self.page_control(d, th, cx))
                        .on_click(
                            cx.listener(|this, _, window, cx| this.open_add_account(window, cx)),
                        ),
                ),
            );
        let delete_all = div()
            .flex()
            .flex_col()
            .items_start()
            .gap(px(12.0))
            .child(
                div()
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text_dim))
                    .child(
                        "Deletes every account, all stored mail, contacts and calendars, the \
                         search index, your settings and saved passwords from this computer. \
                         Nothing changes on your mail servers.",
                    ),
            )
            .child(
                danger_button("delete-all-open", "Delete all Katna data", false, th)
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(|this, _, window, cx| this.ask_delete_all(window, cx))),
            );
        let shown = self.config.mail.accounts_shown;
        let mut pane = div().flex().flex_col().gap(px(2.0));
        for (choice, id, label) in [
            (
                AccountsShown::One,
                "page-accounts-one",
                "One account at a time; switch in the account card",
            ),
            (
                AccountsShown::All,
                "page-accounts-all",
                "All accounts, one after another",
            ),
        ] {
            pane = pane.child(self.radio_row(
                id,
                label,
                shown == choice,
                Change::AccountsShown(choice),
                th,
                cx,
            ));
        }
        div()
            .flex()
            .flex_col()
            .child(super::settings_page::row(
                "Folder pane",
                Some("Which accounts' folders the pane on the left shows."),
                pane,
                th,
            ))
            .child(super::settings_page::row(
                "Accounts",
                Some(
                    "Removing an account deletes Katna's copy of its mail on this computer. \
                     The mail stays on the server.",
                ),
                list,
                th,
            ))
            .child(super::settings_page::row(
                "Delete all data",
                Some("Start over, as on a new install."),
                delete_all,
                th,
            ))
            .into_any_element()
    }

    fn ask(&mut self, what: What, cx: &mut Context<Self>) {
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.danger = Some(Danger {
            what,
            busy: false,
            error: None,
            closing: false,
            shown,
        });
        cx.notify();
    }

    fn ask_delete_all(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let accent = rgba(self.theme(window).error).into();
        let typed = cx.new(|cx| {
            let mut input = TextInput::new(format!("Type \u{201c}{CONFIRM_WORD}\u{201d}"), cx);
            input.set_accent(accent);
            input
        });
        let subscription =
            cx.subscribe_in(
                &typed,
                window,
                |this, _, event: &InputEvent, _, cx| match event {
                    InputEvent::Submit => this.confirm_danger(cx),
                    InputEvent::Cancel => this.close_danger(cx),
                    _ => cx.notify(),
                },
            );
        window.focus(&typed.focus_handle(cx), cx);
        self.ask(
            What::DeleteAll {
                typed,
                _subscription: subscription,
            },
            cx,
        );
    }

    fn close_danger(&mut self, cx: &mut Context<Self>) {
        if let Some(danger) = &mut self.danger
            && !danger.busy
        {
            danger.closing = true;
            danger.shown.set(0.0);
        }
        cx.notify();
    }

    /// Whether the dialog's red button may be pressed.
    fn danger_ready(&self, cx: &Context<Self>) -> bool {
        match &self.danger {
            Some(Danger { busy: true, .. }) | None => false,
            Some(Danger {
                what: What::DeleteAll { typed, .. },
                ..
            }) => typed
                .read(cx)
                .text()
                .trim()
                .eq_ignore_ascii_case(CONFIRM_WORD),
            Some(_) => true,
        }
    }

    fn confirm_danger(&mut self, cx: &mut Context<Self>) {
        if !self.danger_ready(cx) {
            return;
        }
        let Some(danger) = &mut self.danger else {
            return;
        };
        danger.busy = true;
        danger.error = None;
        let remove = match &danger.what {
            What::RemoveAccount(account) => Some(account.clone()),
            What::DeleteAll { .. } => None,
        };
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let account = remove.as_ref().map(|a| a.id.0);
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    match account {
                        Some(id) => daemon::remove_account(&connection, id).await,
                        None => daemon::delete_all_data(&connection).await,
                    }
                })
                .await;
            this.update(cx, |this, cx| match result {
                Ok(()) => {
                    if let Some(danger) = &mut this.danger {
                        danger.busy = false;
                    }
                    this.close_danger(cx);
                    match remove {
                        Some(account) => this.account_removed(&account, cx),
                        None => this.all_data_deleted(cx),
                    }
                }
                Err(err) => {
                    if let Some(danger) = &mut this.danger {
                        danger.busy = false;
                        danger.error = Some(err);
                    }
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
        cx.notify();
    }

    fn account_removed(&mut self, account: &Account, cx: &mut Context<Self>) {
        if self
            .config
            .mail
            .account_tabs
            .remove(&account.address.to_lowercase())
            .is_some()
        {
            self.save_config();
        }
        let listed = self
            .folder
            .is_some_and(|f| self.tree.account_of(f) == Some(account.id));
        if listed || matches!(self.listing, Some(Listing::Search { .. })) {
            self.close_listing(cx);
        }
        self.refresh(true, cx);
        let text = if account.kind == AccountKind::Local {
            format!("{} was removed from Katna.", account.address)
        } else {
            format!(
                "{} was removed from Katna. Its mail is still on the server.",
                account.address
            )
        };
        self.show_snackbar(text, None, cx);
    }

    /// Back to a first start: the daemon deleted every file and exited.
    fn all_data_deleted(&mut self, cx: &mut Context<Self>) {
        self.config = Config::default();
        keymap::bind(&self.config.shortcuts, cx);
        self.compose = None;
        self.unsent = None;
        self.add_account = None;
        self.settings_page = None;
        self.close_listing(cx);
        self.unread.clear();
        self.mail = Mail::open(&self.paths);
        self.load_tree();
        self.show_snackbar("All Katna data was deleted from this computer.", None, cx);
    }

    /// Lists nothing, so the next refresh opens the first inbox.
    fn close_listing(&mut self, cx: &mut Context<Self>) {
        self.listing = None;
        self.folder = None;
        self.reader = None;
        self.reading = false;
        self.entries.clear();
        self.tabs.clear();
        self.tab = 0;
        self.selected = None;
        self.checked.clear();
        self.clear_search(cx);
    }

    pub(super) fn render_danger(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let danger = self.danger.as_mut()?;
        let t = danger.shown.tick(window, reduce);
        if danger.closing && danger.shown.settled() {
            self.danger = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let ready = self.danger_ready(cx);
        let danger = self.danger.as_ref()?;
        let (title, action, busy_text, items): (String, &str, &str, Vec<String>) =
            match &danger.what {
                What::RemoveAccount(account) => {
                    let folders = match self.tree.folders_of(account.id).len() {
                        0 => String::new(),
                        1 => " in its folder".to_owned(),
                        n => format!(" in its {n} folders"),
                    };
                    (
                        format!("Remove {}?", account.address),
                        "Remove account",
                        "Removing\u{2026}",
                        if account.kind == AccountKind::Local {
                            vec![
                                format!("All mail imported into this account{folders}"),
                                "Its Katna settings".into(),
                            ]
                        } else {
                            vec![
                                format!("All of this account's mail stored by Katna{folders}"),
                                "Its messages waiting in the outbox".into(),
                                "Its saved password and its Katna settings".into(),
                            ]
                        },
                    )
                }
                What::DeleteAll { .. } => (
                    "Delete all Katna data?".into(),
                    "Delete everything",
                    "Deleting\u{2026}",
                    vec![
                        "Every account, and all mail and attachments stored by Katna".into(),
                        "Contacts, calendars and the search index".into(),
                        "All settings, signatures and keyboard shortcuts".into(),
                        "Every saved password".into(),
                    ],
                ),
            };
        let warning = div()
            .mt(px(20.0))
            .p(px(16.0))
            .rounded(px(12.0))
            .border_1()
            .border_color(rgba(fade(th.error, 0.45)))
            .bg(rgba(fade(th.error, 0.1)))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgba(th.error))
                    .child("Deleted from this computer:"),
            )
            .children(items.into_iter().map(|item| {
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .child(div().text_color(rgba(th.error)).child("\u{2022}"))
                    .child(div().flex_1().min_w_0().child(item))
            }))
            .child(
                div()
                    .pt(px(4.0))
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgba(th.error))
                    .child("This cannot be undone."),
            );
        let server = div()
            .mt(px(12.0))
            .p(px(12.0))
            .rounded(px(12.0))
            .bg(rgba(fade(th.accent, 0.08)))
            .flex()
            .flex_row()
            .gap(px(10.0))
            .child(icon("info", th.accent, 20.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .text_size(px(13.0))
                    .line_height(px(19.0))
                    .text_color(rgba(th.text_dim))
                    .child(match &danger.what {
                        What::DeleteAll { .. } => {
                            "Nothing changes on your mail servers: your mail stays there, \
                             and adding an account again downloads it again. Mail imported \
                             from files is only in Katna; the files are not touched."
                        }
                        What::RemoveAccount(account) if account.kind == AccountKind::Local => {
                            "This mail was imported from files, so Katna has the only copy. \
                             The files it came from are not touched; import them again to \
                             get it back."
                        }
                        What::RemoveAccount(_) => {
                            "Nothing changes on the mail server: your mail stays there, and \
                             adding the account again downloads it again."
                        }
                    }),
            );
        let confirm = match &danger.what {
            What::DeleteAll { typed, .. } => {
                let focus = typed.focus_handle(cx);
                Some(
                    div()
                        .mt(px(20.0))
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .child(format!("To confirm, type \u{201c}{CONFIRM_WORD}\u{201d}:")),
                        )
                        .child(
                            div()
                                .id("delete-all-typed")
                                .h(px(44.0))
                                .px(px(14.0))
                                .flex()
                                .items_center()
                                .rounded(px(8.0))
                                .border_2()
                                .border_color(rgba(if ready {
                                    th.error
                                } else {
                                    fade(th.text_faint, 0.8)
                                }))
                                .text_size(px(15.0))
                                .cursor_text()
                                .on_click(move |_, window, cx| window.focus(&focus, cx))
                                .child(div().flex_1().child(typed.clone())),
                        ),
                )
            }
            What::RemoveAccount(_) => None,
        };
        let error = danger.error.clone().map(|err| {
            div()
                .mt(px(12.0))
                .text_size(px(13.0))
                .text_color(rgba(th.error))
                .child(err)
        });
        let busy = danger.busy;
        let body = div()
            .id("danger-body")
            .flex()
            .flex_col()
            .overflow_y_scroll()
            .px(px(32.0))
            .pt(px(28.0))
            .pb(px(24.0))
            .child(
                div()
                    .size(px(48.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .bg(rgba(fade(th.error, 0.14)))
                    .child(icon("warning", th.error, 28.0)),
            )
            .child(
                div()
                    .mt(px(16.0))
                    .text_size(px(22.0))
                    .line_height(px(30.0))
                    .child(title),
            )
            .child(warning)
            .child(server)
            .children(confirm)
            .children(error)
            .child(
                div()
                    .mt(px(28.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(div().flex_1())
                    .child(
                        div()
                            .id("danger-cancel")
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
                            .on_click(cx.listener(|this, _, _, cx| this.close_danger(cx)))
                            .child("Cancel"),
                    )
                    .child(
                        danger_button(
                            "danger-confirm",
                            if busy { busy_text } else { action },
                            true,
                            th,
                        )
                        .focus_ring(th)
                        .when(!ready, |d| d.opacity(0.45).cursor_default())
                        .on_click(cx.listener(|this, _, _, cx| this.confirm_danger(cx))),
                    ),
            );
        let viewport = window.viewport_size();
        let (vw, vh) = (f32::from(viewport.width), f32::from(viewport.height));
        let card = div()
            .id("danger")
            .occlude()
            .w(px(WIDTH.min(vw - 32.0)))
            .max_h(px((vh - 48.0).max(200.0)))
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(px(28.0))
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
                        .id("danger-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| this.close_danger(cx))),
                )
                .child(div().opacity(t).mt(px(lerp(24.0, 0.0, t))).child(card))
                .into_any_element(),
        )
    }
}

/// A red button: outlined in the page, filled in the dialog.
/// A quiet button: accent text, a background under the pointer.
fn text_button(id: impl Into<gpui::ElementId>, label: &'static str, th: &Theme) -> Stateful<Div> {
    div()
        .id(id)
        .flex_none()
        .h(px(36.0))
        .px(px(12.0))
        .flex()
        .items_center()
        .rounded_full()
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgba(th.accent))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .child(label)
}

fn danger_button(
    id: impl Into<gpui::ElementId>,
    label: &'static str,
    filled: bool,
    th: &Theme,
) -> Stateful<Div> {
    let button = div()
        .id(id)
        .flex_none()
        .h(px(36.0))
        .px(px(20.0))
        .flex()
        .items_center()
        .rounded_full()
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .cursor_pointer();
    if filled {
        button
            .bg(rgba(th.error))
            .text_color(rgba(th.surface))
            .hover(|s| s.shadow(elevation(th, 1.0)))
            .child(label)
    } else {
        button
            .border_1()
            .border_color(rgba(fade(th.error, 0.6)))
            .text_color(rgba(th.error))
            .hover(|s| s.bg(rgba(fade(th.error, 0.08))))
            .child(label)
    }
}

fn kind_name(kind: AccountKind) -> &'static str {
    match kind {
        AccountKind::Imap => "IMAP",
        AccountKind::Jmap => "JMAP",
        AccountKind::Pop3 => "POP3",
        AccountKind::Local => "Imported",
        AccountKind::CalDav => "CalDAV",
        AccountKind::CardDav => "CardDAV",
    }
}
