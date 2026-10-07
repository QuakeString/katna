// SPDX-License-Identifier: GPL-3.0-or-later

//! Each account's color, picked in Settings > Accounts or from the
//! account's right-click menu in the folder pane. It is the dot after the
//! names on lines of the unified inbox, the ring round the account's
//! picture (the top bar, the account menu, Settings, the right-click
//! card) and its letter avatar; all of them read it from
//! [`MailWindow::account_color`]. The colors are
//! [`crate::theme::ACCOUNT_COLORS`], none of them an inbox tab's, and the
//! rainbow wheel after them opens the color picker for one of one's own.
//! An account without a color gets one no other account wears when the
//! folder pane loads ([`crate::theme::settle_account_colors`]).

use gpui::{AnyElement, Context, Window, canvas, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_platform::colors::parse_css_color;
use katna_ui::px;

use super::MailWindow;
use super::scheme_color::Target;
use crate::theme::{
    ACCOUNT_COLORS, Theme, account_color, account_dark, default_account_color,
    settle_account_colors,
};
use crate::widgets::ScaledEdge;

/// The color an account wears.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Wears {
    /// One of [`ACCOUNT_COLORS`], by name.
    Named(&'static str),
    /// One of one's own: its light-mode color.
    Own(u32),
}

/// The ring round an account's picture, and the gap inside it.
const RING: f32 = 3.0;
const RING_GAP: f32 = 2.5;
/// A 32 px picture in its ring, across.
pub(super) const RING_WIDTH: f32 = 32.0 + 2.0 * (RING + RING_GAP);
/// The dot of the account's color, across.
const DOT: f32 = 16.0;

impl MailWindow {
    /// The color the account at `address` wears.
    fn account_wears(&self, address: &str) -> Wears {
        let key = address.trim().to_lowercase();
        let picked = self.config.mail.account_colors.get(&key);
        if let Some((name, ..)) = picked.and_then(|p| ACCOUNT_COLORS.iter().find(|(n, ..)| n == p))
        {
            return Wears::Named(name);
        }
        if let Some(own) = picked.and_then(|p| parse_css_color(p)) {
            return Wears::Own(own | 0xff);
        }
        Wears::Named(default_account_color(&key))
    }

    /// The light and dark colors of the account at `address`.
    fn account_colors(&self, address: &str) -> (u32, u32) {
        match self.account_wears(address) {
            Wears::Named(name) => account_color(name),
            Wears::Own(light) => (light, account_dark(light)),
        }
    }

    /// The color of the account at `address`, for `th`'s light or dark.
    pub(super) fn account_color(&self, address: &str, th: &Theme) -> u32 {
        let (light, dark) = self.account_colors(address);
        if th.dark { dark } else { light }
    }

    /// Its light-mode color, which white text reads on: the fill of its
    /// letter avatar, and what the color picker starts from.
    pub(super) fn account_light(&self, address: &str) -> u32 {
        self.account_colors(address).0
    }

    /// The fill of its letter avatar.
    pub(super) fn account_fill(&self, address: &str) -> u32 {
        self.account_light(address)
    }

    /// Gives the account at `address` `color` from the picker: by name
    /// when it is a standard one, else as a color of its own.
    pub(super) fn set_account_custom(&mut self, address: &str, color: u32, cx: &mut Context<Self>) {
        let value = ACCOUNT_COLORS
            .iter()
            .find(|(_, light, _)| *light == color | 0xff)
            .map_or_else(
                || format!("#{:06x}", color >> 8),
                |(name, ..)| (*name).to_owned(),
            );
        self.config
            .mail
            .account_colors
            .insert(address.trim().to_lowercase(), value);
        self.save_config();
        cx.notify();
    }

    /// The address of the account `id`.
    pub(super) fn account_address(&self, id: katna_core::AccountId) -> Option<String> {
        self.accounts
            .iter()
            .find(|a| a.id == id)
            .map(|a| a.address.clone())
    }

    /// Gives each account without a color one no other account wears.
    pub(super) fn settle_account_colors(&mut self) {
        let addresses: Vec<String> = self
            .accounts
            .iter()
            .map(|a| a.address.trim().to_lowercase())
            .collect();
        if settle_account_colors(&addresses, &mut self.config.mail.account_colors) {
            self.save_config();
        }
    }

    /// The account's color as a dot, for the end of the "Colour" button
    /// (Settings > Accounts, the right-click menu) that opens the color
    /// picker beside it, and what the picker colors.
    pub(super) fn account_color_dot(
        &self,
        address: &str,
        th: &Theme,
    ) -> Option<(Target, AnyElement)> {
        let account = self
            .accounts
            .iter()
            .find(|a| a.address.eq_ignore_ascii_case(address))?;
        let target = Target::Account(account.id);
        let swatches = self.color_swatches.clone();
        let dot = div()
            .relative()
            .flex_none()
            .size(px(DOT))
            .rounded_full()
            .bg(rgba(self.account_color(address, th)))
            .child(
                canvas(
                    move |bounds, _, _| {
                        swatches.borrow_mut().insert(target, bounds);
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            )
            .into_any_element();
        Some((target, dot))
    }

    /// Opens the color picker on the account's color (closing the
    /// right-click menu: popovers never stack), or closes it.
    pub(super) fn pick_account_color(
        &mut self,
        target: Target,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.close_nav_menu(cx);
        self.toggle_color_picker(target, window, cx);
    }

    /// `picture` of the account at `address` in a ring
    /// of its color [`RING`] wide, [`RING_GAP`] apart.
    pub(super) fn account_ring(
        &self,
        address: &str,
        picture: AnyElement,
        th: &Theme,
    ) -> AnyElement {
        // The gap is padding, not centring: layout snaps padding and
        // border to whole device pixels the same on every side, where a
        // centred picture lands half a pixel off and snaps one way.
        div()
            .flex_none()
            .flex()
            .rounded_full()
            .border_px(RING)
            .p(px(RING_GAP))
            .border_color(rgba(self.account_color(address, th)))
            .child(picture)
            .into_any_element()
    }
}

/// A color's name, for its swatch's tooltip.
pub(super) fn color_label(name: &str) -> String {
    match name {
        "red" => tr!("account-color-red"),
        "pink" => tr!("account-color-pink"),
        "magenta" => tr!("account-color-magenta"),
        "brown" => tr!("account-color-brown"),
        "olive" => tr!("account-color-olive"),
        "teal" => tr!("account-color-teal"),
        "indigo" => tr!("account-color-indigo"),
        _ => tr!("account-color-slate"),
    }
}
