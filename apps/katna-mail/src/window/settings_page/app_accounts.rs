// SPDX-License-Identifier: GPL-3.0-or-later

//! Leaving an account out of an app: a "Show in …" switch per account on
//! each app's Settings page (`[hidden_accounts]`). A hidden account's
//! items stay synced but are not shown in that app, so turning it back on
//! is instant; its mail is not affected.

use std::collections::HashSet;

use gpui::{AnyElement, Context, div, prelude::*};
use katna_core::AccountId;
use katna_core::config::AppKind;
use katna_i18n::tr;

use super::super::settings::Change;
use crate::theme::Theme;
use crate::window::MailWindow;

/// The app's name, as the rail shows it.
fn app_name(app: AppKind) -> String {
    match app {
        AppKind::Calendar => tr!("rail-calendar"),
        AppKind::Contacts => tr!("rail-contacts"),
        AppKind::Tasks => tr!("rail-tasks"),
        AppKind::Notes => tr!("rail-notes"),
        AppKind::Files => tr!("rail-files"),
    }
}

impl MailWindow {
    /// The accounts `app` leaves out.
    pub(in crate::window) fn hidden_ids(&self, app: AppKind) -> HashSet<AccountId> {
        let hidden = &self.config.hidden_accounts;
        if hidden.in_app(app).is_empty() {
            return HashSet::new();
        }
        self.accounts
            .iter()
            .filter(|a| hidden.hides(app, &a.address))
            .map(|a| a.id)
            .collect()
    }

    /// Shows account `id` in `app` or leaves it out, and reads the app's
    /// items again.
    pub(in crate::window) fn set_app_account_shown(
        &mut self,
        app: AppKind,
        id: AccountId,
        shown: bool,
        cx: &mut Context<Self>,
    ) {
        let Some(address) = self.accounts.iter().find(|a| a.id == id) else {
            return;
        };
        let address = address.address.clone();
        self.config.hidden_accounts.set_shown(app, &address, shown);
        self.save_config();
        // The daemon's reminders follow.
        self.send(crate::daemon::Command::ReloadConfig, None, None, true, cx);
        match app {
            AppKind::Calendar => self.load_calendar(cx),
            AppKind::Contacts => self.load_contacts(cx),
            AppKind::Tasks => self.load_tasks(cx),
            AppKind::Notes => self.load_notes(cx),
            AppKind::Files => {
                self.load_drives();
                self.load_library(cx);
            }
        }
        cx.notify();
    }

    /// The "Accounts" group of an app's Settings page: a "Show in …"
    /// switch for each account.
    pub(in crate::window) fn app_accounts_rows(
        &self,
        app: AppKind,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let name = app_name(app);
        let hidden = &self.config.hidden_accounts;
        let key = app as usize * 1_000_000;
        let detail = tr!("settings-app-accounts-detail", app = name.clone());
        let switches: Vec<_> = self
            .accounts
            .iter()
            .map(|account| {
                let shown = !hidden.hides(app, &account.address);
                let label = if account.display_name.trim().is_empty() {
                    account.address.clone()
                } else {
                    account.display_name.clone()
                };
                let detail = if shown {
                    tr!("settings-app-account-shown", app = name.clone())
                } else {
                    tr!("settings-app-account-hidden", app = name.clone())
                };
                let detail = if label == account.address {
                    detail
                } else {
                    format!("{} · {detail}", account.address)
                };
                self.switch_row(
                    ("app-account", key + account.id.0 as usize),
                    label,
                    detail,
                    shown,
                    Change::AppAccount(app, account.id, !shown),
                    th,
                    cx,
                )
            })
            .collect();
        self.row(
            tr!("settings-app-accounts"),
            Some(&detail),
            div().flex().flex_col().children(switches),
            th,
        )
        .into_any_element()
    }
}
