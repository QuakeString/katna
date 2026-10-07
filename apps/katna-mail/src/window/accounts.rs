// SPDX-License-Identifier: GPL-3.0-or-later

//! The Accounts part of the Settings page: the accounts Katna syncs, with
//! a way to remove each one, and a way to delete everything Katna keeps.
//! Both ask first in a dialog that says in red what goes, and that mail on
//! the server stays. The daemon does the deleting. Resetting the cache (in
//! General) asks in the same dialog, saying what is downloaded again and
//! what is kept.

use gpui::{
    AnyElement, ClickEvent, Context, Div, DragMoveEvent, ElementId, Entity, Focusable, FontWeight,
    MouseButton, MouseDownEvent, SharedString, Stateful, Subscription, Window, deferred, div,
    prelude::*, rgba,
};
use katna_core::config::AccountsShown;
use katna_core::{Account, AccountId, AccountKind, Config, Pop3Keep};
use katna_i18n::tr;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;
use katna_ui::tokens::{space, text};
use katna_ui::unpx;
use katna_ui::{InputEvent, TextInput};

use super::settings::Change;
use super::{Listing, MailWindow, keymap};
use crate::daemon;
use crate::data::Mail;
use crate::theme::{Theme, fade};
use crate::widgets::{FocusRing, ScaledEdge, elevation, icon};

const WIDTH: f32 = 500.0;

/// Days a POP3 account keeps mail on the server when first asked to
/// remove it after some days, and the steps of the buttons beside it.
const DEFAULT_DAYS: u32 = 14;
const DAY_STEPS: [u32; 9] = [1, 3, 7, 14, 30, 60, 90, 180, 365];

/// The next step up or down from `days`, if there is one.
fn step_days(days: u32, up: bool) -> Option<u32> {
    if up {
        DAY_STEPS.into_iter().find(|d| *d > days)
    } else {
        DAY_STEPS.into_iter().rev().find(|d| *d < days)
    }
}

/// What a POP3 account does with mail on the server, as Settings offers
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pop3Choice {
    /// Removed once deleted for good in Katna (the default).
    WithKatna,
    AfterDownload,
    AfterDays,
    Never,
}

impl Pop3Choice {
    fn of(keep: Pop3Keep) -> Self {
        if !keep.leave_on_server {
            Self::AfterDownload
        } else if keep.days.is_some() {
            Self::AfterDays
        } else if keep.delete_with_local {
            Self::WithKatna
        } else {
            Self::Never
        }
    }

    fn keep(self, days: u32) -> Pop3Keep {
        match self {
            Self::WithKatna => Pop3Keep::default(),
            Self::AfterDownload => Pop3Keep {
                leave_on_server: false,
                days: None,
                delete_with_local: false,
            },
            Self::AfterDays => Pop3Keep {
                leave_on_server: true,
                days: Some(days.max(1)),
                delete_with_local: true,
            },
            Self::Never => Pop3Keep {
                leave_on_server: true,
                days: None,
                delete_with_local: false,
            },
        }
    }

    fn id(self) -> &'static str {
        match self {
            Self::WithKatna => "pop3-with-katna",
            Self::AfterDownload => "pop3-at-once",
            Self::AfterDays => "pop3-after-days",
            Self::Never => "pop3-never",
        }
    }
}

/// A question before deleting.
pub(super) struct Danger {
    what: What,
    busy: bool,
    error: Option<String>,
    closing: bool,
    /// The dialog has had the keys given to it.
    focused: bool,
    shown: Spring,
}

/// An account's name being changed in Settings > Accounts.
pub(super) struct Renaming {
    account: AccountId,
    input: Entity<TextInput>,
    _subscription: Subscription,
}

/// An account being dragged to a new place in Settings > Accounts. The
/// row itself follows the pointer, so what GPUI draws under it is empty.
#[derive(Clone)]
struct AccountDrag {
    ix: usize,
}

impl Render for AccountDrag {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

/// The space between two account rows.
const ROW_GAP: f32 = 4.0;

/// Reordering Settings > Accounts by dragging a whole row.
pub(super) type Reorder = super::row_reorder::Reorder<AccountId>;

enum What {
    RemoveAccount(Account),
    /// Deleting a calendar (`delete`), or taking one shared with the
    /// person off their list, on its account's service (`account`, its
    /// address; empty for this computer).
    Calendar {
        id: i64,
        name: String,
        account: String,
        delete: bool,
    },
    DeleteAll {
        typed: Entity<TextInput>,
        _subscription: Subscription,
    },
    ResetCache,
}

/// What the daemon did for the dialog.
enum Done {
    Removed(Account),
    DeletedAll,
    /// Messages that lost their body, and bytes deleted.
    CacheReset(u64, u64),
    /// A calendar went: its name, and whether it was deleted.
    Calendar(String, bool),
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
            .gap(px(ROW_GAP))
            .on_drag_move(
                cx.listener(|this, event: &DragMoveEvent<AccountDrag>, _, cx| {
                    let from = event.drag(cx).ix;
                    this.account_dragged(from, unpx(event.event.position.y), cx)
                }),
            )
            .on_drop(cx.listener(|this, _: &AccountDrag, _, cx| this.drop_account(cx)))
            .children({
                let count = accounts.len();
                accounts
                    .iter()
                    .enumerate()
                    .map(|(ix, account)| self.account_row(ix, count, account, th, cx))
                    .collect::<Vec<_>>()
            })
            .when(self.accounts.is_empty(), |d| {
                d.child(
                    div()
                        .py(px(8.0))
                        .text_size(px(14.0))
                        .text_color(rgba(th.text_faint))
                        .child(tr!("accounts-none")),
                )
            })
            .child(
                div().pt(px(8.0)).flex().child(
                    crate::widgets::outlined_button("account-add-page", tr!("account-add"), th)
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
                    .child(tr!("accounts-delete-all-about")),
            )
            .child(
                danger_button(
                    "delete-all-open",
                    tr!("accounts-delete-all-open"),
                    false,
                    th,
                )
                .map(|d| self.page_control(d, th, cx))
                .on_click(cx.listener(|this, _, window, cx| this.ask_delete_all(window, cx))),
            );
        let shown = self.config.mail.accounts_shown;
        let mut pane = div().flex().flex_col().gap(px(2.0));
        for (choice, id, label) in [
            (
                AccountsShown::One,
                "page-accounts-one",
                tr!("accounts-shown-one"),
            ),
            (
                AccountsShown::All,
                "page-accounts-all",
                tr!("accounts-shown-all"),
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
        let unified = self.config.mail.unified_inbox;
        div()
            .flex()
            .flex_col()
            .child(self.row(
                tr!("accounts-folder-pane"),
                Some(tr!("accounts-folder-pane-detail").as_str()),
                pane,
                th,
            ))
            .child(self.row(
                tr!("accounts-unified"),
                None,
                self.switch_row(
                    "page-accounts-unified",
                    tr!("accounts-unified-switch"),
                    tr!("accounts-unified-switch-detail"),
                    unified,
                    Change::UnifiedInbox(!unified),
                    th,
                    cx,
                ),
                th,
            ))
            .child(self.row(
                tr!("accounts-row"),
                Some(tr!("accounts-row-detail").as_str()),
                list,
                th,
            ))
            .children(self.connected_section(&accounts, th, cx))
            .children(self.pop3_section(th, cx))
            .child(self.row(
                tr!("accounts-delete-all-row"),
                Some(tr!("accounts-delete-all-row-detail").as_str()),
                delete_all,
                th,
            ))
            .into_any_element()
    }

    /// What each POP3 account does with mail on the server once it is
    /// downloaded; only with a POP3 account.
    fn pop3_section(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let accounts: Vec<Account> = self
            .accounts
            .iter()
            .filter(|a| a.kind == AccountKind::Pop3)
            .cloned()
            .collect();
        if accounts.is_empty() {
            return None;
        }
        let several = accounts.len() > 1;
        let mut pane = div().flex().flex_col().gap(px(16.0));
        for (ix, account) in accounts.iter().enumerate() {
            let keep = self.pop3_keep_of(account.id);
            let now = Pop3Choice::of(keep);
            let days = keep.days.unwrap_or(DEFAULT_DAYS);
            let id = account.id;
            let option = |choice: Pop3Choice, label: String| {
                let on = now == choice;
                self.page_control(div().id((choice.id(), ix)), th, cx)
                    .relative()
                    .overflow_hidden()
                    .min_h(px(40.0))
                    .py(px(8.0))
                    .px(px(8.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(14.0))
                    .rounded(px(8.0))
                    .text_size(px(14.0))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.set_pop3_keep(id, choice.keep(days), cx)
                    }))
                    .child(super::settings::animated_radio(
                        (choice.id(), ix + 1000),
                        on,
                        th,
                    ))
                    .child(div().flex_1().min_w_0().child(label))
            };
            let stepper = |dir: &'static str, to: Option<u32>| {
                let button = crate::widgets::icon_button(
                    (dir, ix),
                    if dir == "pop3-days-less" {
                        "remove"
                    } else {
                        "add"
                    },
                    18.0,
                    th,
                )
                .tooltip(crate::widgets::tip(
                    if dir == "pop3-days-less" {
                        tr!("accounts-pop3-days-less")
                    } else {
                        tr!("accounts-pop3-days-more")
                    },
                    th,
                ));
                match to.filter(|_| now == Pop3Choice::AfterDays) {
                    Some(to) => button
                        .map(|d| self.page_control(d, th, cx))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.set_pop3_keep(id, Pop3Choice::AfterDays.keep(to), cx)
                        })),
                    None => button.opacity(0.3).cursor_default(),
                }
            };
            let after_days = div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(4.0))
                .child(div().flex_1().min_w(px(200.0)).child(option(
                    Pop3Choice::AfterDays,
                    tr!("accounts-pop3-after-days", count = days),
                )))
                .child(stepper("pop3-days-less", step_days(days, false)))
                .child(stepper("pop3-days-more", step_days(days, true)));
            pane = pane.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .when(several, |d| {
                        d.child(
                            div()
                                .pb(px(4.0))
                                .text_size(px(13.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgba(th.text_dim))
                                .child(account.address.clone()),
                        )
                    })
                    .child(option(
                        Pop3Choice::WithKatna,
                        tr!("accounts-pop3-with-katna"),
                    ))
                    .child(option(
                        Pop3Choice::AfterDownload,
                        tr!("accounts-pop3-at-once"),
                    ))
                    .child(after_days)
                    .child(option(Pop3Choice::Never, tr!("accounts-pop3-never"))),
            );
        }
        Some(
            self.row(
                tr!("accounts-pop3-row"),
                Some(tr!("accounts-pop3-row-detail").as_str()),
                pane,
                th,
            )
            .into_any_element(),
        )
    }

    /// A POP3 account's choice: the one just made, else the store's.
    fn pop3_keep_of(&self, id: AccountId) -> Pop3Keep {
        self.pop3_keep.get(&id).copied().unwrap_or_else(|| {
            self.mail
                .as_ref()
                .map(|mail| mail.pop3_keep(id))
                .unwrap_or_default()
        })
    }

    fn set_pop3_keep(&mut self, id: AccountId, keep: Pop3Keep, cx: &mut Context<Self>) {
        if self.pop3_keep_of(id) == keep {
            return;
        }
        self.pop3_keep.insert(id, keep);
        cx.notify();
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::set_pop3_keep(&connection, id.0, keep).await
                })
                .await;
            this.update(cx, |this, cx| {
                if let Err(err) = result {
                    tracing::warn!(%err, "POP3 choice not saved");
                    this.pop3_keep.remove(&id);
                    this.show_snackbar(err, None, cx);
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// One account in Settings > Accounts: a handle to drag it, its
    /// picture, its name (or the field renaming it), the buttons for its
    /// name and picture, Remove, and Move up and Move down.
    fn account_row(
        &self,
        ix: usize,
        count: usize,
        account: &Account,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let name = if account.display_name.trim().is_empty() {
            account.address.clone()
        } else {
            account.display_name.clone()
        };
        let id = account.id;
        let problem = self.account_problem(id);
        let renaming = self
            .settings_page
            .as_ref()
            .and_then(|p| p.renaming.as_ref())
            .filter(|r| r.account == id);
        let about = div()
            .flex_grow(1.0)
            .flex_basis(px(180.0))
            .min_w_0()
            .flex()
            .flex_col()
            .map(|d| match renaming {
                Some(renaming) => d.child(
                    div()
                        .id(("account-name-field", ix))
                        .h(px(36.0))
                        .px(px(12.0))
                        .flex()
                        .items_center()
                        .rounded(px(8.0))
                        .border_px(2.0)
                        .border_color(rgba(th.accent))
                        .text_size(px(14.0))
                        .child(div().flex_1().min_w_0().child(renaming.input.clone())),
                ),
                None => d.child(
                    self.copyable(name.clone(), th)
                        .truncate()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::MEDIUM),
                ),
            })
            .child(
                self.copyable(
                    format!("{} \u{b7} {}", account.address, kind_name(account.kind)),
                    th,
                )
                .truncate()
                .text_size(px(12.0))
                .text_color(rgba(th.text_faint)),
            )
            // What is wrong with it, as the line over the mail list says.
            .children(problem.as_ref().map(|problem| {
                div()
                    .pt(px(space::S1))
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap(px(space::S2))
                    .text_size(px(text::SMALL))
                    .text_color(rgba(th.text_dim))
                    .child(div().flex_none().child(icon(
                        problem.icon(),
                        problem.color(th),
                        text::SMALL + 2.0,
                    )))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .child(problem.text())
                            // Its fix, as on the line over the mail list.
                            .child(
                                div()
                                    .id(("account-fix", ix))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgba(th.accent))
                                    .cursor_pointer()
                                    .hover(|s| s.underline())
                                    .child(problem.action())
                                    .map(|d| self.page_control(d, th, cx))
                                    .on_click({
                                        let problem = problem.clone();
                                        cx.listener(move |this, event: &ClickEvent, window, cx| {
                                            this.fix_problem(
                                                problem.clone(),
                                                event.position(),
                                                window,
                                                cx,
                                            )
                                        })
                                    }),
                            ),
                    )
            }));
        let own = self.remote.has_own_picture(id);
        let desktop = self.remote.has_desktop_picture();
        let buttons = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(4.0))
            .map(|d| match renaming {
                Some(_) => d
                    .child(
                        text_button(("account-name-save", ix), tr!("accounts-name-save"), th)
                            .map(|d| self.page_control(d, th, cx))
                            .on_click(cx.listener(|this, _, _, cx| this.finish_rename(true, cx))),
                    )
                    .child(
                        text_button(("account-name-cancel", ix), tr!("accounts-name-cancel"), th)
                            .map(|d| self.page_control(d, th, cx))
                            .on_click(cx.listener(|this, _, _, cx| this.finish_rename(false, cx))),
                    ),
                None => d.child(
                    text_button(("account-rename", ix), tr!("accounts-rename"), th)
                        .map(|d| self.page_control(d, th, cx))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.start_rename(id, window, cx)
                        })),
                ),
            })
            .child(
                text_button(("account-picture", ix), tr!("accounts-picture-change"), th)
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(move |this, _, _, cx| this.pick_account_picture(id, cx))),
            )
            .when(!own && desktop, |d| {
                d.child(
                    text_button(
                        ("account-picture-desktop", ix),
                        tr!("accounts-picture-reset"),
                        th,
                    )
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(move |this, _, _, cx| this.use_desktop_picture(id, cx))),
                )
            })
            .when(own, |d| {
                d.child(
                    text_button(
                        ("account-picture-remove", ix),
                        tr!("accounts-picture-remove"),
                        th,
                    )
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(
                        cx.listener(move |this, _, _, cx| this.reset_account_picture(id, cx)),
                    ),
                )
            })
            .children(
                self.account_color_dot(&account.address, th)
                    .map(|(target, dot)| {
                        text_button(("account-color", ix), tr!("account-color-menu"), th)
                            .gap(px(8.0))
                            .child(dot)
                            .map(|d| self.page_control(d, th, cx))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.pick_account_color(target, window, cx)
                            }))
                    }),
            )
            .child({
                let account = account.clone();
                danger_button(("account-remove", ix), tr!("accounts-remove"), false, th)
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.ask(What::RemoveAccount(account.clone()), cx)
                    }))
            });
        let arrow = |dir: &'static str, to: Option<usize>, label: String| {
            let button = crate::widgets::icon_button(
                (dir, ix),
                if dir == "account-up" {
                    "chevron-up"
                } else {
                    "chevron-down"
                },
                20.0,
                th,
            )
            .tooltip(crate::widgets::tip(label, th));
            match to {
                Some(to) => button
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(move |this, _, _, cx| this.move_account(ix, to, cx))),
                None => button.opacity(0.3).cursor_default(),
            }
        };
        let reorder = self.settings_page.as_ref().map(|p| &p.reorder);
        let raised = reorder.is_some_and(|r| r.raised(ix, id));
        let dragging = reorder.is_some_and(|r| r.dragging());
        let lifted = reorder.is_some_and(|r| r.lifted(ix));
        // Move up and Move down show on the row the pointer is over, or
        // whose button has the keyboard focus; not on the rows a drag
        // passes over.
        let arrows_shown = lifted || (!dragging && reorder.is_some_and(|r| r.focused == Some(ix)));
        let group = SharedString::from(format!("account-row-{ix}"));
        let handle = div()
            .id(("account-drag", ix))
            .flex_none()
            .size(px(36.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .cursor_grab()
            .relative()
            .child(crate::widgets::hover_fade("hover-glow", None, th))
            .tooltip(crate::widgets::tip(tr!("accounts-drag"), th))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, event: &MouseDownEvent, _, _| {
                    if let Some(page) = this.settings_page.as_mut() {
                        page.reorder.grab(ix, unpx(event.position.y));
                    }
                }),
            )
            .on_drag(AccountDrag { ix }, |drag, _, _, cx| {
                cx.new(|_| drag.clone())
            })
            .child(icon("drag-handle", th.text_faint, 20.0));
        let avatar = self.problem_badge(
            self.account_ring(
                &account.address,
                self.person_avatar(&name, &account.address, 36.0),
                th,
            ),
            36.0,
            problem.as_ref(),
            th,
        );
        // The buttons go below the name, together, where the row is
        // narrow.
        let row = div()
            .id(("account-row", ix))
            .group(group.clone())
            .mx(px(-8.0))
            .px(px(8.0))
            .py(px(6.0))
            .rounded(px(12.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .when_some(reorder, |d, r| r.row(d, ix, id, th))
            .child(handle)
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
            .child(
                div()
                    .flex_none()
                    .flex()
                    .flex_col()
                    .when(!arrows_shown, |d| {
                        d.opacity(0.0)
                            .when(!dragging, |d| d.group_hover(group, |s| s.opacity(1.0)))
                    })
                    .child(arrow(
                        "account-up",
                        ix.checked_sub(1),
                        tr!("accounts-move-up"),
                    ))
                    .child(arrow(
                        "account-down",
                        (ix + 1 < count).then_some(ix + 1),
                        tr!("accounts-move-down"),
                    )),
            );
        // Drawn over the rows after it while lifted or landing.
        if raised {
            deferred(row).with_priority(1).into_any_element()
        } else {
            row.into_any_element()
        }
    }

    /// The General row's button and what it does.
    pub(super) fn reset_cache_control(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            .px(px(8.0))
            .flex()
            .flex_col()
            .items_start()
            .gap(px(12.0))
            .child(
                div()
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("reset-cache-about")),
            )
            .child(
                crate::widgets::outlined_button("reset-cache-open", tr!("reset-cache-button"), th)
                    .map(|d| self.page_control(d, th, cx))
                    .on_click(cx.listener(|this, _, _, cx| this.ask(What::ResetCache, cx))),
            )
            .into_any_element()
    }

    /// Moves the mail account at `from` to `to` in Settings > Accounts;
    /// the folder pane, the account menu and every other list follow. The
    /// rows glide to their new places, a dragged one from where it was
    /// dropped.
    fn move_account(&mut self, from: usize, to: usize, cx: &mut Context<Self>) {
        let accounts: Vec<Account> = self
            .accounts
            .iter()
            .filter(|a| a.kind.is_mail())
            .cloned()
            .collect();
        let old: Vec<AccountId> = accounts.iter().map(|a| a.id).collect();
        if let Some(page) = self.settings_page.as_mut() {
            page.reorder.moved(&old, from, to, ROW_GAP);
        }
        if from != to {
            self.config.mail.move_account(&accounts, from, to);
            self.save_config();
            self.load_tree();
        }
        cx.notify();
    }

    /// The pointer moved while dragging the account at `from` to `y`.
    fn account_dragged(&mut self, from: usize, y: f32, cx: &mut Context<Self>) {
        let ids: Vec<AccountId> = self
            .accounts
            .iter()
            .filter(|a| a.kind.is_mail())
            .map(|a| a.id)
            .collect();
        if let Some(page) = self.settings_page.as_mut() {
            page.reorder.dragged(&ids, from, y, ROW_GAP);
        }
        cx.notify();
    }

    /// The dragged account was let go: it goes where the rows made room.
    fn drop_account(&mut self, cx: &mut Context<Self>) {
        let Some((from, to)) = self
            .settings_page
            .as_ref()
            .and_then(|p| p.reorder.drop_move())
        else {
            return;
        };
        self.move_account(from, to, cx);
    }

    /// Advances the rows sliding in Settings > Accounts for this frame.
    pub(super) fn tick_reorder(&mut self, window: &Window, reduce: bool, cx: &mut Context<Self>) {
        let dropped = self
            .settings_page
            .as_ref()
            .is_some_and(|p| p.reorder.dragging())
            && !cx.has_active_drag();
        if dropped {
            // Let go somewhere with nowhere to drop it.
            self.drop_account(cx);
        }
        let count = self.accounts.iter().filter(|a| a.kind.is_mail()).count();
        let Some(page) = self.settings_page.as_mut() else {
            return;
        };
        let stops = page.tab_stops().clone();
        let reorder = &mut page.reorder;
        reorder.focused = (0..count).find(|&ix| {
            ["account-up", "account-down"]
                .into_iter()
                .any(|dir| stops.focused(&ElementId::from((dir, ix)), window))
        });
        reorder.tick(count, window, reduce);
        self.tick_rule_reorder(window, reduce, cx);
    }

    fn start_rename(&mut self, account: AccountId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(current) = self.accounts.iter().find(|a| a.id == account) else {
            return;
        };
        let accent = rgba(self.theme(window).accent).into();
        let text = current.display_name.clone();
        let input = cx.new(|cx| {
            let mut input = TextInput::new(tr!("accounts-name-placeholder"), cx);
            input.set_accent(accent);
            input.set_text(text, cx);
            input.select_all_text(cx);
            input
        });
        let subscription = cx.subscribe(&input, |this, _, event: &InputEvent, cx| match event {
            InputEvent::Submit => this.finish_rename(true, cx),
            InputEvent::Cancel => this.finish_rename(false, cx),
            InputEvent::Changed => {}
        });
        window.focus(&input.focus_handle(cx), cx);
        if let Some(page) = self.settings_page.as_mut() {
            page.renaming = Some(Renaming {
                account,
                input,
                _subscription: subscription,
            });
        }
        cx.notify();
    }

    /// Saves the typed name (`save`), or leaves the old one.
    fn finish_rename(&mut self, save: bool, cx: &mut Context<Self>) {
        let Some(renaming) = self.settings_page.as_mut().and_then(|p| p.renaming.take()) else {
            return;
        };
        cx.notify();
        if !save {
            return;
        }
        let name = renaming.input.read(cx).text().trim().to_owned();
        let id = renaming.account.0;
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::rename_account(&connection, id, &name).await
                })
                .await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(()) => this.load_tree(),
                    Err(err) => {
                        this.show_snackbar(tr!("accounts-rename-failed", error = err), None, cx)
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Asks before deleting calendar `id` (`delete`), or taking it off the
    /// person's list, on the service of `account` (empty: this computer).
    pub(super) fn ask_calendar_removal(
        &mut self,
        id: i64,
        name: String,
        account: String,
        delete: bool,
        cx: &mut Context<Self>,
    ) {
        self.ask(
            What::Calendar {
                id,
                name,
                account,
                delete,
            },
            cx,
        );
    }

    fn ask(&mut self, what: What, cx: &mut Context<Self>) {
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.danger = Some(Danger {
            what,
            busy: false,
            error: None,
            closing: false,
            focused: false,
            shown,
        });
        cx.notify();
    }

    fn ask_delete_all(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let accent = rgba(self.theme(window).error).into();
        let typed = cx.new(|cx| {
            let mut input = TextInput::new(tr!("accounts-confirm-placeholder"), cx);
            input.set_accent(accent);
            input
        });
        let subscription =
            cx.subscribe_in(
                &typed,
                window,
                |this, _, event: &InputEvent, _, cx| match event {
                    InputEvent::Submit => this.confirm_danger(cx),
                    InputEvent::Cancel => {
                        this.close_danger(cx);
                    }
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
        // Its field has the keys already.
        if let Some(danger) = &mut self.danger {
            danger.focused = true;
        }
    }

    /// Closes the dialog unless it is at work. Returns whether it closed.
    pub(super) fn close_danger(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(danger) = self.danger.as_mut().filter(|d| !d.busy && !d.closing) else {
            return false;
        };
        danger.closing = true;
        danger.shown.set(0.0);
        cx.notify();
        true
    }

    /// Whether the dialog's red button may be pressed.
    fn danger_ready(&self, cx: &Context<Self>) -> bool {
        match &self.danger {
            Some(Danger { busy: true, .. }) | None => false,
            Some(Danger {
                what: What::DeleteAll { typed, .. },
                ..
            }) => {
                // The English word also works, for a keyboard without the
                // language's letters or accents.
                let typed = typed.read(cx).text().trim().to_lowercase();
                typed == tr!("accounts-confirm-word").to_lowercase() || typed == "delete"
            }
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
            What::DeleteAll { .. } | What::ResetCache | What::Calendar { .. } => None,
        };
        let calendar = match &danger.what {
            What::Calendar {
                id, name, delete, ..
            } => Some((*id, name.clone(), *delete)),
            _ => None,
        };
        let reset = matches!(danger.what, What::ResetCache);
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    if let Some((id, name, delete)) = calendar {
                        let edit = daemon::CalendarEdit::Delete(id, delete);
                        return daemon::edit_calendar(&connection, &edit)
                            .await
                            .map(|_| Done::Calendar(name, delete));
                    }
                    match remove {
                        Some(account) => daemon::remove_account(&connection, account.id.0)
                            .await
                            .map(|()| Done::Removed(account)),
                        None if reset => daemon::reset_cache(&connection)
                            .await
                            .map(|(messages, bytes)| Done::CacheReset(messages, bytes)),
                        None => daemon::delete_all_data(&connection)
                            .await
                            .map(|()| Done::DeletedAll),
                    }
                })
                .await;
            this.update(cx, |this, cx| match result {
                Ok(done) => {
                    if let Some(danger) = &mut this.danger {
                        danger.busy = false;
                    }
                    this.close_danger(cx);
                    match done {
                        Done::Removed(account) => this.account_removed(&account, cx),
                        Done::DeletedAll => this.all_data_deleted(cx),
                        Done::CacheReset(messages, bytes) => this.cache_reset(messages, bytes, cx),
                        Done::Calendar(name, delete) => this.calendar_removed(&name, delete, cx),
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
        let key = account.address.to_lowercase();
        let tabs = self.config.mail.account_tabs.remove(&key).is_some();
        let color = self.config.mail.account_colors.remove(key.trim()).is_some();
        if tabs || color {
            self.save_config();
        }
        let id = account.id;
        self.drop_color_picker(|t| t == super::scheme_color::Target::Account(id));
        let listed = self
            .folder
            .is_some_and(|f| self.tree.account_of(f) == Some(account.id));
        if listed
            || matches!(
                self.listing,
                Some(
                    Listing::Search { .. }
                        | Listing::Unified { .. }
                        | Listing::Waiting
                        | Listing::Reminders
                )
            )
        {
            self.close_listing(cx);
        }
        self.refresh(true, cx);
        let address = account.address.as_str();
        let text = if account.kind == AccountKind::Local {
            tr!("accounts-removed-local", address = address)
        } else {
            tr!("accounts-removed", address = address)
        };
        self.show_snackbar(text, None, cx);
    }

    /// Back to a first start: the daemon deleted every file and exited.
    fn all_data_deleted(&mut self, cx: &mut Context<Self>) {
        self.config = Config::default();
        keymap::bind(&self.config.shortcuts, cx);
        self.compose = None;
        self.writing.parked = None;
        self.unsent = None;
        self.add_account = None;
        self.settings_page = None;
        self.close_listing(cx);
        self.unread.clear();
        let nudges = self.config.mail.nudges;
        self.mail = Mail::open(&self.paths).map(|mail| mail.with_nudges(nudges));
        self.load_tree();
        self.show_snackbar(tr!("accounts-all-deleted"), None, cx);
    }

    /// The daemon deleted what it downloaded and is downloading it again.
    fn cache_reset(&mut self, messages: u64, bytes: u64, cx: &mut Context<Self>) {
        self.refresh(true, cx);
        let text = if messages == 0 {
            tr!("reset-cache-done")
        } else {
            tr!("reset-cache-done-freed", size = crate::format::size(bytes))
        };
        self.show_snackbar(text, None, cx);
    }

    /// Lists nothing, so the next refresh opens the first inbox.
    fn close_listing(&mut self, cx: &mut Context<Self>) {
        self.listing = None;
        self.folder = None;
        self.unified = None;
        self.reader = None;
        self.reading = false;
        self.entries.clear();
        self.tabs.clear();
        self.tab = 0;
        self.selected = None;
        self.checked.clear();
        self.check_anchor = None;
        self.clear_search(cx);
    }

    pub(super) fn render_danger(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let danger = self.danger.as_mut()?;
        let t = danger.shown.tick(window, reduce);
        if danger.closing && danger.shown.settled() {
            self.danger = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        // The keys go to the dialog (to its field, if it has one).
        if !danger.focused {
            danger.focused = true;
            window.focus(&self.dialog_focus, cx);
        }
        let ready = self.danger_ready(cx);
        let danger = self.danger.as_ref()?;
        let (title, action, busy_text, items): (String, String, String, Vec<String>) =
            match &danger.what {
                What::ResetCache => (
                    tr!("reset-cache-title"),
                    tr!("reset-cache-confirm"),
                    tr!("reset-cache-busy"),
                    vec![
                        tr!("reset-cache-mail"),
                        tr!("reset-cache-index"),
                        tr!("reset-cache-pictures"),
                    ],
                ),
                What::RemoveAccount(account) => {
                    let folders = self.tree.folders_of(account.id).len();
                    (
                        tr!("accounts-remove-title", address = account.address.as_str()),
                        tr!("accounts-remove-confirm"),
                        tr!("accounts-removing"),
                        if account.kind == AccountKind::Local {
                            vec![
                                tr!("accounts-remove-local-mail", folders = folders),
                                tr!("accounts-remove-local-settings"),
                            ]
                        } else {
                            vec![
                                tr!("accounts-remove-mail", folders = folders),
                                tr!("accounts-remove-outbox"),
                                tr!("accounts-remove-settings"),
                            ]
                        },
                    )
                }
                What::Calendar {
                    name,
                    account,
                    delete: true,
                    ..
                } => {
                    let mut items = vec![tr!("calendar-delete-events")];
                    // One on this computer is shared with no one.
                    if !account.is_empty() {
                        items.push(tr!("calendar-delete-shared"));
                    }
                    (
                        tr!("calendar-delete-title", name = name.as_str()),
                        tr!("calendar-delete-confirm"),
                        tr!("calendar-deleting"),
                        items,
                    )
                }
                What::Calendar { name, .. } => (
                    tr!("calendar-remove-title", name = name.as_str()),
                    tr!("calendar-remove-confirm"),
                    tr!("calendar-removing"),
                    vec![tr!("calendar-remove-events")],
                ),
                What::DeleteAll { .. } => (
                    tr!("accounts-delete-all-title"),
                    tr!("accounts-delete-all-confirm"),
                    tr!("accounts-deleting"),
                    vec![
                        tr!("accounts-delete-all-accounts"),
                        tr!("accounts-delete-all-contacts"),
                        tr!("accounts-delete-all-settings"),
                        tr!("accounts-delete-all-passwords"),
                    ],
                ),
            };
        let reset = matches!(danger.what, What::ResetCache);
        // Taking a shared calendar off the list loses nothing: its owner
        // keeps it.
        let unlist = matches!(danger.what, What::Calendar { delete: false, .. });
        // Resetting deletes nothing that cannot be downloaded again, so it
        // is not red.
        let tone = if reset || unlist { th.accent } else { th.error };
        let warning = div()
            .mt(px(20.0))
            .p(px(16.0))
            .rounded(px(12.0))
            .border_1()
            .border_color(rgba(fade(tone, 0.45)))
            .bg(rgba(fade(tone, 0.1)))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(
                self.copyable(
                    match &danger.what {
                        What::ResetCache => tr!("reset-cache-deleted"),
                        What::Calendar { delete: true, .. } => tr!("calendar-delete-heading"),
                        What::Calendar { .. } => tr!("calendar-remove-heading"),
                        _ => tr!("accounts-deleted-heading"),
                    },
                    th,
                )
                .text_size(px(14.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgba(tone)),
            )
            .children(items.into_iter().map(|item| {
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .child(div().text_color(rgba(tone)).child("\u{2022}"))
                    .child(self.copyable(item, th).flex_1().min_w_0())
            }))
            .when(!reset && !unlist, |d| {
                d.child(
                    self.copyable(tr!("accounts-cannot-undo"), th)
                        .pt(px(4.0))
                        .text_size(px(14.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgba(tone)),
                )
            });
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
                self.copyable(
                    match &danger.what {
                        What::ResetCache => tr!("reset-cache-kept"),
                        What::DeleteAll { .. } => tr!("accounts-server-delete-all"),
                        What::RemoveAccount(account) if account.kind == AccountKind::Local => {
                            tr!("accounts-server-local")
                        }
                        What::RemoveAccount(_) => tr!("accounts-server-remove"),
                        What::Calendar { account, .. } if account.is_empty() => {
                            tr!("calendar-delete-local")
                        }
                        What::Calendar {
                            account,
                            delete: true,
                            ..
                        } => tr!("calendar-delete-server", account = account.as_str()),
                        What::Calendar { .. } => tr!("calendar-remove-server"),
                    },
                    th,
                )
                .flex_1()
                .min_w_0()
                .text_size(px(13.0))
                .line_height(px(19.0))
                .text_color(rgba(th.text_dim)),
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
                                .child(tr!("accounts-confirm-prompt")),
                        )
                        .child(
                            div()
                                .id("delete-all-typed")
                                .h(px(44.0))
                                .px(px(14.0))
                                .flex()
                                .items_center()
                                .rounded(px(8.0))
                                .border_px(2.0)
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
            What::RemoveAccount(_) | What::ResetCache | What::Calendar { .. } => None,
        };
        let error = danger.error.clone().map(|err| {
            div()
                .mt(px(12.0))
                .text_size(px(13.0))
                .text_color(rgba(th.error))
                .child(self.copyable(err, th))
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
                    .bg(rgba(fade(tone, 0.14)))
                    .child(if reset {
                        icon("refresh", tone, 28.0)
                    } else if unlist {
                        icon("remove", tone, 28.0)
                    } else {
                        icon("warning", tone, 28.0)
                    }),
            )
            .child(
                self.copyable(title, th)
                    .mt(px(16.0))
                    .text_size(px(22.0))
                    .line_height(px(30.0)),
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
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.close_danger(cx);
                            }))
                            .child(tr!("accounts-cancel")),
                    )
                    .child(
                        {
                            let label = if busy { busy_text } else { action };
                            if reset || unlist {
                                crate::widgets::filled_button("danger-confirm", label, th)
                            } else {
                                danger_button("danger-confirm", label, true, th)
                            }
                        }
                        .focus_ring_filled(th)
                        .when(!ready, |d| d.opacity(0.45).cursor_default())
                        .on_click(cx.listener(|this, _, _, cx| this.confirm_danger(cx))),
                    ),
            );
        let (vw, vh) = (self.room_width(), self.room_height(window));
        let card = div()
            .id("danger")
            .track_focus(&self.dialog_focus)
            .map(|d| super::popovers::keep_tab_inside(d, &self.dialog_focus))
            .occlude()
            .w(px(WIDTH.min(vw - 32.0)))
            .max_h(px((vh - 48.0).max(200.0)))
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
                        .id("danger-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.close_danger(cx);
                        })),
                )
                .child(div().opacity(t).mt(px(lerp(24.0, 0.0, t))).child(card))
                .into_any_element(),
        )
    }
}

/// A red button: outlined in the page, filled in the dialog.
/// A quiet button: accent text, a background under the pointer.
fn text_button(
    id: impl Into<gpui::ElementId>,
    label: impl Into<SharedString>,
    th: &Theme,
) -> Stateful<Div> {
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
        .relative()
        .child(crate::widgets::hover_fade("hover-glow", None, th))
        .child(label.into())
}

fn danger_button(
    id: impl Into<gpui::ElementId>,
    label: impl Into<SharedString>,
    filled: bool,
    th: &Theme,
) -> Stateful<Div> {
    let label = label.into();
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

/// The account's type; protocol names stay as they are.
fn kind_name(kind: AccountKind) -> String {
    match kind {
        AccountKind::Imap => "IMAP".into(),
        AccountKind::Jmap => "JMAP".into(),
        AccountKind::Pop3 => "POP3".into(),
        AccountKind::Local => tr!("accounts-kind-imported"),
        AccountKind::CalDav => "CalDAV".into(),
        AccountKind::CardDav => "CardDAV".into(),
    }
}
