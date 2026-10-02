// SPDX-License-Identifier: GPL-3.0-or-later

//! Each account's color, picked in Settings > Accounts or from the
//! account's right-click menu in the folder pane. It is the dot after the
//! names on lines of the unified inbox, the ring round the account's
//! picture (the top bar, the account menu, Settings, the right-click
//! card) and its letter avatar; all of them read it from
//! [`MailWindow::account_color`]. The colors are
//! [`crate::theme::ACCOUNT_COLORS`], none of them an inbox tab's.

use gpui::{AnyElement, Context, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_ui::px;

use super::MailWindow;
use crate::theme::{ACCOUNT_COLORS, Theme, account_color, default_account_color};
use crate::widgets::tip;

/// Each color of the strip, across.
const SWATCH: f32 = 16.0;

impl MailWindow {
    /// The name of the color the account at `address` wears.
    pub(super) fn account_color_name(&self, address: &str) -> &'static str {
        let key = address.trim().to_lowercase();
        self.config
            .mail
            .account_colors
            .get(&key)
            .and_then(|name| ACCOUNT_COLORS.iter().find(|(n, ..)| n == name))
            .map_or_else(|| default_account_color(&key), |(n, ..)| n)
    }

    /// The color of the account at `address`, for `th`'s light or dark.
    pub(super) fn account_color(&self, address: &str, th: &Theme) -> u32 {
        let (light, dark) = account_color(self.account_color_name(address));
        if th.dark { dark } else { light }
    }

    /// The fill of its letter avatar: the light color, which white text
    /// reads on in either mode.
    pub(super) fn account_fill(&self, address: &str) -> u32 {
        account_color(self.account_color_name(address)).0
    }

    fn set_account_color(&mut self, address: &str, name: &'static str, cx: &mut Context<Self>) {
        self.config
            .mail
            .account_colors
            .insert(address.trim().to_lowercase(), name.to_owned());
        self.save_config();
        cx.notify();
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
        let current = self.account_color_name(address);
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
                        let on = name == current;
                        let address = address.to_owned();
                        div()
                            .id((id, n))
                            .size(px(SWATCH + 10.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_full()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .tooltip(tip(color_label(name), th))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.set_account_color(&address, name, cx)
                            }))
                            .child(
                                div()
                                    .size(px(SWATCH + 6.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded_full()
                                    .border_2()
                                    .border_color(rgba(if on { color } else { 0 }))
                                    .child(
                                        div().size(px(SWATCH - 2.0)).rounded_full().bg(rgba(color)),
                                    ),
                            )
                    }),
            )
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
