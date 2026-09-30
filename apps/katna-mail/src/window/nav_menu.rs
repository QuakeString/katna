// SPDX-License-Identifier: GPL-3.0-or-later

//! The right-click menu of the folder pane, as in Thunderbird and webmail:
//! "Check for new mail" for that folder only (its account from an
//! account's heading, every account from All Accounts), "Mark all as
//! read", a new folder or label inside it, and "Empty Trash".

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
use crate::sidebar::{self, Role};
use crate::theme::Theme;
use crate::widgets::{icon, raised};
use futures_lite::FutureExt;
use katna_core::AccountKind;

const MENU_WIDTH: f32 = 240.0;
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
                }
            }
            // Scheduled mail lives on this computer only.
            sidebar::Row::Folder { .. } => return,
        };
        self.menu = None;
        self.context_menu = None;
        self.nav_menu = Some(menu);
        cx.notify();
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
        let gmail = account.is_some_and(|a| self.tree.is_gmail(a));
        let list = div()
            .key_context(crate::widgets::MENU_CONTEXT)
            .w(px(MENU_WIDTH))
            .py(px(8.0))
            .flex()
            .flex_col()
            .map(|d| raised(d, th, 8.0, 3.0))
            .text_size(px(14.0))
            .text_color(rgba(th.text))
            .child(
                item(
                    "nav-menu-check",
                    "refresh",
                    tr!("nav-menu-check-mail").into(),
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
                menu.folder.filter(|_| menu.role == Role::Trash),
                |d, folder| {
                    d.child(div().my(px(6.0)).h(px(1.0)).bg(rgba(th.divider)))
                        .child(
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
