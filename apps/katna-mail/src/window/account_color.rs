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

use gpui::{AnyElement, Context, canvas, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_platform::colors::parse_css_color;
use katna_ui::px;

use super::MailWindow;
use super::scheme_color::Target;
use crate::theme::{
    ACCOUNT_COLORS, Theme, account_color, account_dark, default_account_color,
    settle_account_colors,
};
use crate::widgets::{color_swatch, color_wheel, tip};

/// The color an account wears.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Wears {
    /// One of [`ACCOUNT_COLORS`], by name.
    Named(&'static str),
    /// One of one's own: its light-mode color.
    Own(u32),
}

/// Each color of the strip, across, with its ring.
const SWATCH: f32 = 26.0;

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

    fn set_account_color(&mut self, address: &str, name: &'static str, cx: &mut Context<Self>) {
        self.config
            .mail
            .account_colors
            .insert(address.trim().to_lowercase(), name.to_owned());
        self.save_config();
        cx.notify();
    }

    /// Gives the account at `address` the color of its own `color`, from
    /// the picker.
    pub(super) fn set_account_custom(&mut self, address: &str, color: u32, cx: &mut Context<Self>) {
        self.config.mail.account_colors.insert(
            address.trim().to_lowercase(),
            format!("#{:06x}", color >> 8),
        );
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

    /// The colors to pick from for the account at `address`, the one it
    /// wears ringed. Clicking one wears it at once.
    pub(super) fn account_color_strip(
        &self,
        id: &'static str,
        address: &str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let wears = self.account_wears(address);
        let own = match wears {
            Wears::Own(_) => Some(self.account_color(address, th)),
            Wears::Named(_) => None,
        };
        let wheel = self
            .accounts
            .iter()
            .find(|a| a.address.eq_ignore_ascii_case(address))
            .map(|account| {
                let target = Target::Account(account.id);
                let swatches = self.color_swatches.clone();
                color_wheel((id, ACCOUNT_COLORS.len()), own, SWATCH, th)
                    .relative()
                    .tooltip(tip(tr!("account-color-own"), th))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        // Popovers never stack: the right-click menu
                        // makes way.
                        this.close_nav_menu(cx);
                        this.toggle_color_picker(target, window, cx)
                    }))
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
            });
        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(2.0))
            .children(
                ACCOUNT_COLORS
                    .iter()
                    .enumerate()
                    .map(|(n, &(name, light, dark))| {
                        let color = if th.dark { dark } else { light };
                        let address = address.to_owned();
                        color_swatch((id, n), color, wears == Wears::Named(name), SWATCH, th)
                            .tooltip(tip(color_label(name), th))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.set_account_color(&address, name, cx)
                            }))
                    }),
            )
            .children(wheel)
            .into_any_element()
    }

    /// `picture` of the account at `address`, `size` across, in a ring
    /// of its color, a hair apart.
    pub(super) fn account_ring(
        &self,
        address: &str,
        picture: AnyElement,
        size: f32,
        th: &Theme,
    ) -> AnyElement {
        div()
            .flex_none()
            .size(px(size + 6.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .border_2()
            .border_color(rgba(self.account_color(address, th)))
            .child(picture)
            .into_any_element()
    }
}

/// A color's name, for its swatch's tooltip.
fn color_label(name: &str) -> String {
    match name {
        "red" => tr!("account-color-red"),
        "pink" => tr!("account-color-pink"),
        "brown" => tr!("account-color-brown"),
        "olive" => tr!("account-color-olive"),
        "teal" => tr!("account-color-teal"),
        "indigo" => tr!("account-color-indigo"),
        _ => tr!("account-color-slate"),
    }
}
