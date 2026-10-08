// SPDX-License-Identifier: GPL-3.0-or-later

//! The account picture at the top right while All Accounts is open: the
//! pictures of the accounts in it overlap in a row, the first account in
//! front at the left, with no ring, so it never looks like one account
//! on show. From four accounts on, the third place counts the rest. On
//! hover the pictures ease apart.

use gpui::{
    AnimationExt, AnyElement, Context, FontWeight, IntoElement, ParentElement, SpringAnimation,
    Styled, div, prelude::*, rgba,
};
use katna_core::{Account, AccountId};
use katna_i18n::tr;
use katna_ui::{motion, px, tokens};

use super::{MailWindow, MenuKey};
use crate::sidebar::Unified;
use crate::theme::{Theme, mix};
use crate::widgets::{ScaledEdge, icon};

/// Each picture across.
pub(super) const PICTURE: f32 = 30.0;
/// The cut in the page's colour around each picture, which parts it from
/// the one behind.
const CUT: f32 = tokens::space::S1;
/// How far each picture slides under the one before it, at rest and on
/// hover, for a picture [`PICTURE`] across; smaller ones overlap in step.
const OVERLAP: f32 = 9.0;
const OVERLAP_SPREAD: f32 = 4.0;
/// The most places in the row.
const PLACES: usize = 3;

impl MailWindow {
    /// The mail accounts the unified lists gather, in the account card's
    /// order, while All Accounts is open; `None` otherwise or with one.
    pub(super) fn stacked_accounts(&self) -> Option<Vec<&Account>> {
        if !matches!(self.unified, Some((_, None))) {
            return None;
        }
        self.unified_accounts()
    }

    /// The mail accounts the unified lists gather, in the account card's
    /// order; `None` with fewer than two.
    pub(super) fn unified_accounts(&self) -> Option<Vec<&Account>> {
        let accounts: Vec<&Account> = self
            .accounts
            .iter()
            .filter(|a| a.kind.is_mail() && self.in_unified(a.id))
            .filter(|a| self.tree.accounts.iter().any(|t| t.id == a.id))
            .collect();
        (accounts.len() > 1).then_some(accounts)
    }

    /// What the account menu marks as open: All Accounts (`Some(None)`),
    /// one account, or nothing.
    pub(super) fn menu_current(&self) -> Option<Option<AccountId>> {
        if self.shows_unified() && matches!(self.unified, Some((_, None))) {
            return Some(None);
        }
        if let Some(id) = self.shown_account() {
            return Some(Some(id));
        }
        match self.unified {
            Some((_, Some(id))) => Some(Some(id)),
            _ => self.folder.and_then(|f| self.tree.account_of(f)).map(Some),
        }
    }

    /// Opens the unified Inbox, from the account menu or the wheel.
    pub(super) fn pick_all_accounts(&mut self, cx: &mut Context<Self>) {
        self.settings_page = None;
        self.open_unified(Unified::Inbox, None, cx);
        cx.notify();
    }

    /// How much wider than the single picture in its ring the stack is,
    /// spread on hover: room the search box gives up.
    pub(super) fn account_stack_room(&self) -> f32 {
        self.stacked_accounts().map_or(0.0, |accounts| {
            let places = accounts.len().min(PLACES) as f32;
            let side = PICTURE + 2.0 * CUT;
            let spread = side + (places - 1.0) * (side - OVERLAP_SPREAD);
            (spread - super::account_color::RING_WIDTH).max(0.0)
        })
    }

    /// The pictures of `accounts`, each `size` across, overlapping, eased
    /// apart while `spread`, each in a cut of `cut`: the colour behind
    /// them. With `reserve` the row always takes its spread width and the
    /// pictures ease apart to the left inside it, so nothing beside it
    /// moves.
    pub(super) fn render_account_stack(
        &self,
        accounts: &[&Account],
        size: f32,
        spread: bool,
        reserve: bool,
        cut: u32,
        th: &Theme,
    ) -> AnyElement {
        let count = accounts.len();
        // With more than fit, the last place counts the rest.
        let faces = if count > PLACES { PLACES - 1 } else { count };
        let mut places: Vec<AnyElement> = accounts[..faces]
            .iter()
            .map(|account| {
                let name = if account.display_name.trim().is_empty() {
                    &account.address
                } else {
                    &account.display_name
                };
                self.person_avatar(name, &account.address, size)
            })
            .collect();
        if count > faces {
            places.push(
                div()
                    .size(px(size))
                    .rounded_full()
                    .bg(rgba(th.chip))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(tokens::text::MICRO))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgba(th.text_dim))
                    .child(format!("+{}", count - faces))
                    .into_any_element(),
            );
        }
        let target = if spread { 1.0 } else { 0.0 };
        let n = places.len() as f32;
        // Each place is a picture in its cut; the row is as wide as the
        // places less their overlaps.
        let side = size + 2.0 * CUT;
        // Whole pixels at rest, so the edges stay sharp.
        let (rest, apart) = (
            (OVERLAP * size / PICTURE).round(),
            (OVERLAP_SPREAD * size / PICTURE).round(),
        );
        let width = move |s: f32| side + (n - 1.0) * (side - motion::lerp(rest, apart, s));
        let spring = || SpringAnimation::new(motion::scaled(motion::SLIDE)).to(target);
        let hover = th.hover;
        // Placed side by side and painted last to first, so the first
        // picture is in front.
        div()
            .flex_none()
            .relative()
            .h(px(side))
            .children(places.into_iter().enumerate().rev().map(|(ix, picture)| {
                div()
                    .absolute()
                    .top_0()
                    .rounded_full()
                    .border_px(CUT)
                    .child(picture)
                    .with_spring(("account-stack", ix), spring(), move |place, s: f32| {
                        // The cut takes on the hover's tint as it fades in.
                        let tint = (hover & 0xff) as f32 / 255.0 * s.clamp(0.0, 1.0);
                        let place = place.border_color(rgba(mix(cut, hover | 0xff, tint)));
                        let step = side - motion::lerp(rest, apart, s);
                        // In a reserved row the last place stays put at the
                        // right and the first moves out into the room.
                        if reserve {
                            place.left(px(width(1.0) - side - (n - 1.0 - ix as f32) * step))
                        } else {
                            place.left(px(ix as f32 * step))
                        }
                    })
                    .into_any_element()
            }))
            .with_spring("account-stack-width", spring(), move |row, s: f32| {
                row.w(px(width(if reserve { 1.0 } else { s })))
            })
            .into_any_element()
    }
}

/// The All Accounts row's pictures, each across.
const ROW_PICTURE: f32 = 22.0;

impl MailWindow {
    /// The account menu's first row, back to All Accounts: the stack, the
    /// number of accounts and the unified Inbox's unread count, marked
    /// while `open`. `None` without a unified inbox.
    pub(super) fn all_accounts_row(
        &self,
        open: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !self.shows_unified() {
            return None;
        }
        let accounts = self.unified_accounts()?;
        let unread = self.tree.unified_inbox_unread();
        // The cut takes the row's colour: the menu's, or the selected fill
        // over it.
        let menu = th.menu | 0xff;
        let cut = if open {
            mix(
                menu,
                th.row_selected | 0xff,
                (th.row_selected & 0xff) as f32 / 255.0,
            )
        } else {
            menu
        };
        let stack = self.render_account_stack(&accounts, ROW_PICTURE, false, false, cut, th);
        Some(
            div()
                .id("account-row-all")
                .h(px(56.0))
                .px(px(tokens::space::S5))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(tokens::space::S4))
                .rounded(px(tokens::radius::SM))
                .cursor_pointer()
                .when(open, |d| d.bg(rgba(th.row_selected)))
                .hover(move |s| s.bg(rgba(if open { th.row_selected } else { th.hover })))
                .menu_key(th)
                .on_click(cx.listener(|this, _, _, cx| {
                    this.account_menu = false;
                    this.pick_all_accounts(cx);
                }))
                // As wide as an account's picture in its ring, so the names
                // line up.
                .child(
                    div()
                        .flex_none()
                        .w(px(super::account_color::RING_WIDTH))
                        .flex()
                        .justify_center()
                        .child(stack),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .truncate()
                                .text_size(px(tokens::text::BODY))
                                .font_weight(FontWeight::MEDIUM)
                                .child(tr!("nav-all-accounts")),
                        )
                        .child(
                            div()
                                .truncate()
                                .text_size(px(tokens::text::CAPTION))
                                .text_color(rgba(th.text_faint))
                                .child(tr!(
                                    "account-menu-all-detail",
                                    count = crate::format::thousands(accounts.len() as u64)
                                )),
                        ),
                )
                .when(unread > 0, |d| {
                    d.child(
                        div()
                            .text_size(px(tokens::text::CAPTION))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgba(th.text_dim))
                            .child(crate::format::thousands(unread)),
                    )
                })
                .when(open, |d| d.child(icon("check", th.text, 20.0)))
                .into_any_element(),
        )
    }
}
