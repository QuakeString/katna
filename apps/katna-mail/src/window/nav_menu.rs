// SPDX-License-Identifier: GPL-3.0-or-later

//! The right-click menu of the folder pane, as in Thunderbird and webmail:
//! "Check for new mail" for that folder only (its account from an
//! account's heading, every account from All Accounts), "Mark all as
//! read", a new folder or label inside it, and "Empty Trash". On an
//! account's heading, or its row under All Accounts, the menu opens with
//! the account: its name and address, whether it is in sync and its
//! storage, then Sign in again when its sign-in stopped working, New mail
//! from this account and Account settings.

use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Div, MouseButton, Pixels, Point, SharedString,
    Stateful, Transformation, Window, anchored, deferred, div, ease_out_quint, percentage,
    prelude::*, rgba, svg,
};
use katna_core::AccountId;
use katna_i18n::tr;
use katna_store::FolderId;
use katna_ui::px;

use super::MenuKey;
use super::{Act, Listing, MailWindow};
use crate::daemon;
use crate::format;
use crate::sidebar::{self, Role};
use crate::theme::Theme;
use crate::widgets::{avatar, icon, raised};
use futures_lite::FutureExt;
use katna_core::AccountKind;
use katna_core::OAuthProvider;
use katna_dbus::state;

const MENU_WIDTH: f32 = 240.0;
/// Wider when an account's address heads the menu.
const ACCOUNT_MENU_WIDTH: f32 = 300.0;
/// The storage bar under the account's address.
const ACCOUNT_BAR_WIDTH: f32 = ACCOUNT_MENU_WIDTH - 16.0 - 36.0 - 12.0 - 16.0;
const ITEM_HEIGHT: f32 = 36.0;
/// How long a check may keep the refresh arrow turning.
const CHECK_LIMIT: Duration = Duration::from_secs(90);

/// A check for new mail under way.
pub(super) struct Check {
    id: u64,
    /// The account checked, or every account.
    account: Option<AccountId>,
    /// Only these folders were checked, when there are any.
    folders: Vec<FolderId>,
}

/// The refresh arrow, turning: mail is being checked for.
pub(super) fn turning_arrow(id: &'static str, color: u32, size: f32) -> impl IntoElement {
    svg()
        .path("icons/refresh.svg")
        .size(px(size))
        .flex_none()
        .text_color(rgba(color))
        .with_animation(
            id,
            Animation::new(Duration::from_millis(900)).repeat(),
            |arrow, t| arrow.with_transformation(Transformation::rotate(percentage(t))),
        )
}

/// The open right-click menu of the folder pane.
pub(super) struct NavMenu {
    ix: usize,
    /// Where the pointer was, in the window.
    at: Point<Pixels>,
    /// The account to check, or every account.
    account: Option<AccountId>,
    /// The folders to check instead, when there are any.
    check: Vec<(AccountId, FolderId)>,
    /// The folder the line opens, for Mark all as read.
    folder: Option<FolderId>,
    unread: u64,
    role: Role,
    /// A new folder or label may go inside it.
    nests: bool,
    /// The account the line stands for, shown on top of the menu.
    about: Option<AccountId>,
    /// Where that account's sync stands, once the daemon said.
    status: Option<katna_dbus::AccountStatus>,
}

impl MailWindow {
    /// Opens the menu for the folder pane's line `ix`, where it was
    /// right-clicked.
    pub(super) fn open_nav_menu(&mut self, ix: usize, at: Point<Pixels>, cx: &mut Context<Self>) {
        let Some(row) = self.nav_rows.get(ix) else {
            return;
        };
        let imap = |account: AccountId| {
            self.accounts
                .iter()
                .any(|a| a.id == account && a.kind == AccountKind::Imap)
        };
        let owned = |folders: Vec<FolderId>| -> Vec<(AccountId, FolderId)> {
            folders
                .into_iter()
                .filter_map(|f| Some((self.tree.account_of(f)?, f)))
                .collect()
        };
        let menu = match row {
            sidebar::Row::AllAccounts { .. } => NavMenu {
                ix,
                at,
                account: None,
                check: Vec::new(),
                folder: None,
                unread: 0,
                role: Role::Other,
                nests: false,
                about: None,
                status: None,
            },
            // Each account's own folder of that kind; a list by flag
            // spans every folder, so every account is checked.
            sidebar::Row::Unified { view, .. } => NavMenu {
                ix,
                at,
                account: None,
                check: if view.role().is_some() {
                    owned(self.tree.unified_folders(*view, None))
                } else {
                    Vec::new()
                },
                folder: None,
                unread: 0,
                role: Role::Other,
                nests: false,
                about: None,
                status: None,
            },
            sidebar::Row::Account { id, .. } | sidebar::Row::Labels { account: id } => NavMenu {
                ix,
                at,
                account: Some(*id),
                check: Vec::new(),
                folder: None,
                unread: 0,
                role: Role::Other,
                nests: false,
                about: matches!(row, sidebar::Row::Account { .. }).then_some(*id),
                status: None,
            },
            sidebar::Row::UnifiedAccount {
                account,
                folder,
                unread,
                ..
            } => NavMenu {
                ix,
                at,
                account: Some(*account),
                check: folder.map(|f| (*account, f)).into_iter().collect(),
                folder: *folder,
                unread: *unread,
                role: folder
                    .and_then(|f| self.tree.node(f))
                    .map_or(Role::Other, |n| n.role),
                nests: false,
                about: Some(*account),
                status: None,
            },
            sidebar::Row::Folder {
                folder: Some(folder),
                role,
                unread,
                ..
            } => {
                let account = self.tree.account_of(*folder);
                NavMenu {
                    ix,
                    at,
                    account,
                    check: account.map(|a| (a, *folder)).into_iter().collect(),
                    folder: Some(*folder),
                    unread: *unread,
                    role: *role,
                    // Gmail nests labels only; other servers nest in their
                    // own folders and the inbox.
                    nests: account.is_some_and(|a| {
                        imap(a)
                            && matches!(role, Role::Other | Role::Inbox)
                            && self.tree.nest_targets(a).iter().any(|(id, _)| id == folder)
                    }),
                    about: None,
                    status: None,
                }
            }
            // Scheduled mail lives on this computer only.
            sidebar::Row::Folder { .. } => return,
        };
        let about = menu.about;
        self.menu = None;
        self.context_menu = None;
        self.nav_menu = Some(menu);
        if let Some(account) = about {
            self.load_nav_account_status(ix, account, cx);
        }
        cx.notify();
    }

    /// Asks the daemon where `account`'s sync stands, for the menu on line
    /// `ix` while it stays open.
    fn load_nav_account_status(&mut self, ix: usize, account: AccountId, cx: &mut Context<Self>) {
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let status = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::account_status(&connection, account).await
                })
                .await;
            this.update(cx, |this, cx| {
                let Ok(Some(status)) = status else {
                    return;
                };
                if let Some(menu) = &mut this.nav_menu
                    && menu.ix == ix
                    && menu.about == Some(account)
                {
                    menu.status = Some(status);
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    /// Opens a new message from `account`.
    fn nav_new_mail(&mut self, account: AccountId, window: &mut Window, cx: &mut Context<Self>) {
        self.nav_menu = None;
        self.open_compose(super::compose::Kind::New, None, window, cx);
        self.send_compose_from(account);
        cx.notify();
    }

    /// The account on top of the menu: its letter, name and address, how
    /// its sync stands and how full its storage is.
    fn render_nav_account(
        &self,
        account: AccountId,
        status: Option<&katna_dbus::AccountStatus>,
        th: &Theme,
    ) -> Option<AnyElement> {
        let info = self.accounts.iter().find(|a| a.id == account)?;
        let name = if info.display_name.is_empty() {
            info.address.clone()
        } else {
            info.display_name.clone()
        };
        let state = status.and_then(|status| {
            let provider = status.sign_in.parse::<OAuthProvider>().ok();
            let (color, text) = match status.state.as_str() {
                state::ONLINE => (
                    th.accent,
                    format::ago(status.last_sync, unix_now())
                        .filter(|_| status.last_sync > 0)
                        .map_or_else(
                            || tr!("nav-account-in-sync"),
                            |ago| tr!("nav-account-checked", ago = ago),
                        ),
                ),
                state::CONNECTING => (th.text_faint, tr!("nav-account-connecting")),
                state::OFFLINE => (th.text_faint, tr!("nav-account-offline")),
                state::AUTH_FAILED => (
                    th.error,
                    match provider {
                        Some(provider) => {
                            tr!("nav-account-signed-out", provider = provider.name())
                        }
                        None => tr!("nav-account-password-refused"),
                    },
                ),
                _ => return None,
            };
            Some(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .text_size(px(12.5))
                    .text_color(rgba(th.text_dim))
                    .child(
                        div()
                            .flex_none()
                            .size(px(8.0))
                            .rounded_full()
                            .bg(rgba(color)),
                    )
                    .child(div().min_w_0().truncate().child(text)),
            )
        });
        let storage = self.quotas.get(&account).copied().map(|quota| {
            let fraction = quota.fraction();
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .h(px(4.0))
                        .rounded(px(2.0))
                        .bg(rgba(th.divider))
                        .child(
                            div()
                                .h(px(4.0))
                                .rounded(px(2.0))
                                .w(px(ACCOUNT_BAR_WIDTH * fraction))
                                .bg(rgba(if fraction >= 0.9 { th.error } else { th.accent })),
                        ),
                )
                .child(
                    div()
                        .text_size(px(12.5))
                        .text_color(rgba(th.text_dim))
                        .child(tr!(
                            "nav-account-storage",
                            used = format::storage_size(quota.used),
                            total = format::storage_size(quota.limit)
                        )),
                )
        });
        Some(
            div()
                .px(px(16.0))
                .pt(px(6.0))
                .pb(px(10.0))
                .flex()
                .flex_row()
                .gap(px(12.0))
                .child(div().flex_none().child(avatar(&name, &info.address, 36.0)))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap(px(6.0))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .truncate()
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .child(name.clone()),
                                )
                                .when(name != info.address, |d| {
                                    d.child(
                                        div()
                                            .truncate()
                                            .text_size(px(12.5))
                                            .text_color(rgba(th.text_dim))
                                            .child(info.address.clone()),
                                    )
                                }),
                        )
                        .children(state)
                        .children(storage),
                )
                .into_any_element(),
        )
    }

    pub(super) fn close_nav_menu(&mut self, cx: &mut Context<Self>) {
        if self.nav_menu.take().is_some() {
            cx.notify();
        }
    }

    /// Has the daemon check `account` (every account for `None`) for new
    /// mail now. Until it has, the refresh arrow and the account's inbox
    /// show a turning arrow.
    pub(super) fn check_mail(&mut self, account: Option<AccountId>, cx: &mut Context<Self>) {
        self.check(account, Vec::new(), cx);
    }

    /// Has the daemon check only `folders` (each with its account) for new
    /// mail now, with a turning arrow on them until it has.
    fn check_folders(&mut self, folders: Vec<(AccountId, FolderId)>, cx: &mut Context<Self>) {
        self.check(None, folders, cx);
    }

    fn check(
        &mut self,
        account: Option<AccountId>,
        folders: Vec<(AccountId, FolderId)>,
        cx: &mut Context<Self>,
    ) {
        self.check_seq += 1;
        let id = self.check_seq;
        self.checking.push(Check {
            id,
            account,
            folders: folders.iter().map(|(_, f)| *f).collect(),
        });
        cx.notify();
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let limit = cx.background_executor().timer(CHECK_LIMIT);
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    if folders.is_empty() {
                        daemon::check_mail(&connection, account).await
                    } else {
                        daemon::check_folders(&connection, &folders).await
                    }
                })
                .or(async {
                    limit.await;
                    Ok(())
                })
                .await;
            this.update(cx, |this, cx| {
                this.checking.retain(|c| c.id != id);
                if let Err(err) = result {
                    tracing::info!("checking for mail: {err}");
                    this.show_snackbar(err, None, cx);
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Whether mail is being checked for now.
    pub(super) fn checking_mail(&self) -> bool {
        !self.checking.is_empty()
    }

    /// Whether `account`'s mail is being checked for now, on its own or
    /// with every account's.
    pub(super) fn checking_account(&self, account: AccountId) -> bool {
        self.checking
            .iter()
            .any(|c| c.folders.is_empty() && c.account.is_none_or(|a| a == account))
    }

    /// Whether every account's mail is being checked for now.
    pub(super) fn checking_all(&self) -> bool {
        self.checking
            .iter()
            .any(|c| c.folders.is_empty() && c.account.is_none())
    }

    /// Whether `folder` alone was asked to be checked, and still is.
    pub(super) fn checking_folder(&self, folder: FolderId) -> bool {
        self.checking.iter().any(|c| c.folders.contains(&folder))
    }

    fn nav_mark_all_read(&mut self, folder: FolderId, cx: &mut Context<Self>) {
        self.nav_menu = None;
        let keys = match &self.mail {
            Ok(mail) => mail
                .entries(folder, None, self.config.mail.conversations)
                .into_iter()
                .map(|e| e.key)
                .collect(),
            Err(_) => Vec::new(),
        };
        self.act(Act::Read(true), keys, cx);
        self.refresh(false, cx);
    }

    /// Opens Trash and asks before deleting all of it for good.
    fn empty_trash(&mut self, folder: FolderId, window: &mut Window, cx: &mut Context<Self>) {
        self.nav_menu = None;
        self.leave_listing(Listing::Folder(folder), cx);
        self.open_folder(folder, cx);
        self.picked_from_nav(window, cx);
        let keys = self.entries.iter().map(|e| e.key).collect();
        self.act(Act::Delete, keys, cx);
    }

    fn nav_new_folder(
        &mut self,
        account: AccountId,
        parent: FolderId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.nav_menu = None;
        self.open_new_label(account, window, cx);
        self.nest_new_label(parent);
        cx.notify();
    }

    pub(super) fn render_nav_menu(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let menu = self.nav_menu.as_ref()?;
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
        let account = menu.account;
        let check = menu.check.clone();
        let about = menu.about;
        let card = about.and_then(|a| self.render_nav_account(a, menu.status.as_ref(), th));
        // Sign in again, for an account whose provider stopped letting it in.
        let sign_in = menu
            .status
            .as_ref()
            .filter(|s| s.state == state::AUTH_FAILED)
            .and_then(|s| {
                Some((
                    s.id,
                    s.address.clone(),
                    s.sign_in.parse::<OAuthProvider>().ok()?,
                ))
            });
        // Mute… (or Unmute) the folder, or the account on its heading.
        let at = menu.at;
        let quiet = match (menu.folder, about) {
            (Some(folder), _) => Some(super::quiet::Quiet::Folder {
                folder,
                categories: Vec::new(),
            }),
            (None, Some(account)) => Some(super::quiet::Quiet::Account(account)),
            _ => None,
        }
        .map(|target| {
            let (glyph, label) = self.quiet_menu_label(&target);
            (target, glyph, label)
        });
        let quiet_item = quiet.map(|(target, glyph, label)| {
            item("nav-menu-quiet", glyph, label).on_click(
                cx.listener(move |this, _, _, cx| this.quiet_menu_click(target.clone(), at, cx)),
            )
        });
        let divider = || div().my(px(6.0)).h(px(1.0)).bg(rgba(th.divider));
        let gmail = account.is_some_and(|a| self.tree.is_gmail(a));
        let list = div()
            .key_context(crate::widgets::MENU_CONTEXT)
            .w(px(if about.is_some() {
                ACCOUNT_MENU_WIDTH
            } else {
                MENU_WIDTH
            }))
            .py(px(8.0))
            .flex()
            .flex_col()
            .map(|d| raised(d, th, 8.0, 3.0))
            .text_size(px(14.0))
            .text_color(rgba(th.text))
            .when_some(card, |d, card| d.child(card).child(divider()))
            .when_some(sign_in, |d, (id, address, provider)| {
                d.child(
                    item(
                        "nav-menu-sign-in",
                        "warning",
                        tr!("nav-menu-sign-in-again").into(),
                    )
                    .text_color(rgba(th.error))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.nav_menu = None;
                        this.sign_in_account(id, address.clone(), provider, cx);
                    })),
                )
                .child(divider())
            })
            .child(
                item(
                    "nav-menu-check",
                    "refresh",
                    if about.is_some() && menu.role == Role::Inbox {
                        tr!("nav-menu-check-inbox")
                    } else {
                        tr!("nav-menu-check-mail")
                    }
                    .into(),
                )
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.nav_menu = None;
                    if check.is_empty() {
                        this.check_mail(account, cx);
                    } else {
                        this.check_folders(check.clone(), cx);
                    }
                })),
            )
            .when_some(menu.folder.filter(|_| menu.unread > 0), |d, folder| {
                d.child(
                    item(
                        "nav-menu-read",
                        "mark-read",
                        tr!("menu-mark-all-read").into(),
                    )
                    .on_click(
                        cx.listener(move |this, _, _, cx| this.nav_mark_all_read(folder, cx)),
                    ),
                )
            })
            .children(quiet_item)
            .when_some(
                menu.folder.zip(account).filter(|_| menu.nests),
                |d, (folder, account)| {
                    d.child(
                        item(
                            "nav-menu-new",
                            "add",
                            if gmail {
                                tr!("nav-menu-new-sublabel")
                            } else {
                                tr!("nav-menu-new-subfolder")
                            }
                            .into(),
                        )
                        .on_click(cx.listener(
                            move |this, _, window, cx| {
                                this.nav_new_folder(account, folder, window, cx)
                            },
                        )),
                    )
                },
            )
            .when_some(
                about.filter(|a| self.accounts.iter().any(|x| x.id == *a && x.kind.is_mail())),
                |d, about| {
                    d.child(
                        item(
                            "nav-menu-new-mail",
                            "compose",
                            tr!("nav-menu-new-mail").into(),
                        )
                        .on_click(cx.listener(
                            move |this, _, window, cx| this.nav_new_mail(about, window, cx),
                        )),
                    )
                },
            )
            .when(about.is_some(), |d| {
                d.child(divider()).child(
                    item(
                        "nav-menu-settings",
                        "settings",
                        tr!("nav-menu-account-settings").into(),
                    )
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.nav_menu = None;
                        this.open_settings_page(
                            super::settings_page::Section::Accounts,
                            window,
                            cx,
                        );
                    })),
                )
            })
            .when_some(
                menu.folder.filter(|_| menu.role == Role::Trash),
                |d, folder| {
                    d.child(divider()).child(
                        item(
                            "nav-menu-empty",
                            "trash",
                            tr!("nav-menu-empty-trash").into(),
                        )
                        .on_click(cx.listener(
                            move |this, _, window, cx| this.empty_trash(folder, window, cx),
                        )),
                    )
                },
            )
            .with_animation(
                ("nav-menu", menu.ix),
                Animation::new(Duration::from_millis(140)).with_easing(ease_out_quint()),
                |el, t| el.opacity(t).mt(px(-4.0 * (1.0 - t))),
            );
        let close = || {
            cx.listener(|this: &mut Self, _: &gpui::MouseDownEvent, _, cx| this.close_nav_menu(cx))
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
                            .id("nav-menu-scrim")
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

fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64)
}
