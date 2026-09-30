// SPDX-License-Identifier: GPL-3.0-or-later

//! What the mail window shows, kept when it closes and shown again when it
//! opens while the Katna service still runs (`crate::placement`,
//! `docs/ARCHITECTURE.md` §13.1).

use gpui::Context;
use katna_core::AccountId;
use katna_core::window::ViewState;
use katna_store::FolderId;

use super::{MailWindow, RailApp};
use crate::sidebar::Unified;

impl MailWindow {
    /// What the window shows now.
    pub fn view_state(&self) -> ViewState {
        let mut expanded: Vec<String> = self.expanded.iter().cloned().collect();
        expanded.sort();
        let mut pages_folded: Vec<String> = self
            .page_sides_folded
            .iter()
            .map(|app| app.key().to_owned())
            .collect();
        pages_folded.sort();
        ViewState {
            app: if self.app == RailApp::Mail {
                String::new()
            } else {
                self.app.key().to_owned()
            },
            folder: self.folder.map(|folder| folder.0),
            unified: self.unified.map(|(view, _)| view.key().to_owned()),
            unified_account: self.unified.and_then(|(_, account)| account.map(|a| a.0)),
            tab: self.tab,
            nav_folded: !self.nav_open,
            expanded,
            accounts: self
                .open_accounts
                .iter()
                .map(|(account, open)| (account.0.to_string(), *open))
                .collect(),
            all_accounts_folded: !self.all_accounts_open,
            pages_folded,
        }
    }

    /// Shows what `view` showed, as far as it still exists, without the
    /// motion of changing it. The folder tree must be loaded.
    pub(super) fn restore_view(&mut self, view: &ViewState, cx: &mut Context<Self>) {
        if view.nav_folded {
            self.nav_open = false;
            self.nav_spring.snap(0.0);
            self.reserve_spring.snap(0.0);
        }
        self.expanded.extend(view.expanded.iter().cloned());
        self.open_accounts = view
            .accounts
            .iter()
            .filter_map(|(id, open)| Some((AccountId(id.parse().ok()?), *open)))
            .collect();
        self.all_accounts_open = !view.all_accounts_folded;
        self.rebuild_nav();
        let unified = view
            .unified
            .as_deref()
            .and_then(|key| Unified::ALL.into_iter().find(|u| u.key() == key));
        let folder = view
            .folder
            .map(FolderId)
            .filter(|&folder| self.tree.node(folder).is_some());
        if let Some(folder) = folder {
            self.open_folder(folder, cx);
            if view.tab < self.tabs.len() {
                self.open_tab(view.tab, cx);
            }
        } else if let Some(unified) = unified
            && self.shows_unified()
        {
            let account = view
                .unified_account
                .map(AccountId)
                .filter(|&id| self.tree.accounts.iter().any(|a| a.id == id));
            self.open_unified(unified, account, cx);
        }
        self.page_sides_folded = view
            .pages_folded
            .iter()
            .filter_map(|key| RailApp::from_key(key))
            .collect();
        if let Some(app) = RailApp::from_key(&view.app) {
            self.open_app(app, cx);
            self.title_from = app;
            self.title_roll.snap(1.0);
        }
    }
}
