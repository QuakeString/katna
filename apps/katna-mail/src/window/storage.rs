// SPDX-License-Identifier: GPL-3.0-or-later

//! How full the account's mail storage is, as a bar and a line at the
//! foot of the folder pane: "34% of 15 GB used". It follows the account
//! whose folder is open and shows only when its server reports a quota
//! (IMAP QUOTA; the daemon reads it on each full sync).

use gpui::{AnyElement, div, prelude::*, rgba};
use katna_core::AccountId;
use katna_i18n::tr;
use katna_ui::px;

use super::{MailWindow, NAV_WIDTH};
use crate::format;
use crate::theme::Theme;
use crate::widgets::tip;

/// Where the bar turns to the error color.
const NEARLY_FULL: f32 = 0.9;

impl MailWindow {
    /// The account the storage line is about: the one whose folder is
    /// open, else the one it showed last (a unified or search list spans
    /// several), else the account on show.
    fn storage_account(&self) -> Option<AccountId> {
        let open = self
            .listed_folder()
            .and_then(|folder| self.tree.account_of(folder))
            .or_else(|| self.shown_account());
        let account = open
            .or_else(|| self.storage_account.get())
            .filter(|id| self.accounts.iter().any(|a| a.id == *id))
            .or_else(|| {
                let current = &self.config.mail.current_account;
                self.accounts
                    .iter()
                    .find(|a| !current.is_empty() && a.address.eq_ignore_ascii_case(current))
                    .or_else(|| self.accounts.iter().find(|a| a.kind.is_mail()))
                    .map(|a| a.id)
            });
        self.storage_account.set(account);
        account
    }

    pub(super) fn render_storage(&self, th: &Theme) -> Option<AnyElement> {
        let account = self.storage_account()?;
        let quota = *self.quotas.get(&account)?;
        let fraction = quota.fraction();
        let percent = katna_i18n::format::number(u64::from((fraction * 100.0).round() as u8));
        let line = tr!(
            "storage-used",
            percent = percent,
            total = format::storage_size(quota.limit)
        );
        let address = self
            .accounts
            .iter()
            .find(|a| a.id == account)
            .map(|a| a.address.clone())
            .unwrap_or_default();
        let detail = tr!(
            "storage-used-detail",
            address = address,
            used = format::storage_size(quota.used),
            total = format::storage_size(quota.limit)
        );
        let fill = if fraction >= NEARLY_FULL {
            th.error
        } else {
            th.text_dim
        };
        let width = NAV_WIDTH - 26.0 - 28.0;
        Some(
            div()
                .id("storage")
                .flex_none()
                .w(px(NAV_WIDTH))
                .pl(px(26.0))
                .pr(px(28.0))
                .pt(px(12.0))
                .pb(px(16.0))
                .tooltip(tip(detail, th))
                .child(
                    div()
                        .w(px(width))
                        .h(px(4.0))
                        .rounded_full()
                        .bg(rgba(th.divider))
                        .child(
                            div()
                                .h_full()
                                // Some use shows as a sliver, never as empty.
                                .w(px((width * fraction).max(if quota.used > 0 {
                                    4.0
                                } else {
                                    0.0
                                })))
                                .rounded_full()
                                .bg(rgba(fill)),
                        ),
                )
                .child(
                    div()
                        .pt(px(8.0))
                        .text_size(px(13.0))
                        .text_color(rgba(th.text_dim))
                        .truncate()
                        .child(line),
                )
                .into_any_element(),
        )
    }
}
