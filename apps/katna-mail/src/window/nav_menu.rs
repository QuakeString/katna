// SPDX-License-Identifier: GPL-3.0-or-later

//! The right-click menu of the folder pane, as in Thunderbird and webmail:
//! "Check for new mail" for the folder's account (every account from
//! All Accounts), "Mark all as read", a new folder or label inside it,
//! and "Empty Trash".

use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Div, MouseButton, Pixels, Point, SharedString,
    Stateful, Window, anchored, deferred, div, ease_out_quint, prelude::*, rgba,
};
use katna_core::AccountId;
use katna_i18n::tr;
use katna_store::FolderId;
use katna_ui::px;

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

/// The open right-click menu of the folder pane.
pub(super) struct NavMenu {
    ix: usize,
    /// Where the pointer was, in the window.
    at: Point<Pixels>,
    /// The account to check, or every account.
    account: Option<AccountId>,
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
        let menu = match row {
            sidebar::Row::AllAccounts { .. } | sidebar::Row::Unified { .. } => NavMenu {
                ix,
                at,
                account: None,
                folder: None,
                unread: 0,
                role: Role::Other,
                nests: false,
            },
            sidebar::Row::Account { id, .. } | sidebar::Row::Labels { account: id } => NavMenu {
                ix,
                at,
                account: Some(*id),
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
    /// mail now; the refresh arrow turns until it has.
    pub(super) fn check_mail(&mut self, account: Option<AccountId>, cx: &mut Context<Self>) {
        self.checking += 1;
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
                    daemon::check_mail(&connection, account).await
                })
                .or(async {
                    limit.await;
                    Ok(())
                })
                .await;
            this.update(cx, |this, cx| {
                this.checking = this.checking.saturating_sub(1);
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
        self.checking > 0
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
                .child(icon(name, th.text_dim, 20.0))
                .child(div().flex_1().min_w_0().truncate().child(label))
        };
        let account = menu.account;
        let gmail = account.is_some_and(|a| self.tree.is_gmail(a));
        let list = div()
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
                    this.check_mail(account, cx);
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
