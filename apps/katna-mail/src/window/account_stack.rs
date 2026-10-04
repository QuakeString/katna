// SPDX-License-Identifier: GPL-3.0-or-later

//! The account picture at the top right while All Accounts is open: the
//! pictures of the accounts in it overlap in a row, the first account in
//! front at the left, with no ring, so it never looks like one account
//! on show. From four accounts on, the third place counts the rest. On
//! hover the pictures ease apart.

use gpui::{
    AnimationExt, AnyElement, FontWeight, IntoElement, ParentElement, SpringAnimation, Styled, div,
    rgba,
};
use katna_core::Account;
use katna_ui::{motion, px, tokens};

use super::MailWindow;
use crate::theme::{Theme, mix};
use crate::widgets::ScaledEdge;

/// Each picture across.
pub(super) const PICTURE: f32 = 30.0;
/// The cut in the page's colour around each picture, which parts it from
/// the one behind.
const CUT: f32 = tokens::space::S1;
/// How far each picture slides under the one before it, at rest and on
/// hover.
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
        let accounts: Vec<&Account> = self
            .accounts
            .iter()
            .filter(|a| a.kind.is_mail() && self.in_unified(a.id))
            .filter(|a| self.tree.accounts.iter().any(|t| t.id == a.id))
            .collect();
        (accounts.len() > 1).then_some(accounts)
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

    /// The pictures of `accounts` overlapping, eased apart while
    /// `spread`, each in a cut of `cut`: the colour behind them.
    pub(super) fn render_account_stack(
        &self,
        accounts: &[&Account],
        spread: bool,
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
                self.person_avatar(name, &account.address, PICTURE)
            })
            .collect();
        if count > faces {
            places.push(
                div()
                    .size(px(PICTURE))
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
        let side = PICTURE + 2.0 * CUT;
        let width =
            move |s: f32| side + (n - 1.0) * (side - motion::lerp(OVERLAP, OVERLAP_SPREAD, s));
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
                        place.left(px(
                            ix as f32 * (side - motion::lerp(OVERLAP, OVERLAP_SPREAD, s))
                        ))
                    })
                    .into_any_element()
            }))
            .with_spring("account-stack-width", spring(), move |row, s: f32| {
                row.w(px(width(s)))
            })
            .into_any_element()
    }
}
