// SPDX-License-Identifier: GPL-3.0-or-later

//! The unified inbox: with several accounts and Settings > General >
//! Unified inbox on, "All Accounts" heads the folder pane with each
//! special folder (Inbox, Sent, ...) of every account in one list, and
//! lists of the unread, starred and important mail of all. Each list opens
//! to one row per account. The accounts below start folded; the arrow
//! beside each account's name folds and opens it either way.

use gpui::Context;
use katna_core::AccountId;

use super::{Listing, MailWindow};
use crate::data::Entry;
use crate::sidebar::Unified;

impl MailWindow {
    /// Whether `account`'s folders show under its name: as its arrow
    /// left it, else folded under the unified inbox and open without.
    pub(super) fn account_open(&self, account: AccountId) -> bool {
        self.open_accounts
            .get(&account)
            .copied()
            .unwrap_or(!self.config.mail.unified_inbox)
    }

    /// The arrow beside an account's name.
    pub(super) fn toggle_account(&mut self, account: AccountId, cx: &mut Context<Self>) {
        let open = !self.account_open(account);
        self.open_accounts.insert(account, open);
        self.rebuild_nav();
        cx.notify();
    }

    /// The arrow beside "All Accounts".
    pub(super) fn toggle_all_accounts(&mut self, cx: &mut Context<Self>) {
        self.all_accounts_open = !self.all_accounts_open;
        self.rebuild_nav();
        cx.notify();
    }

    /// Whether the folder pane has the unified inbox: turned on, with
    /// several accounts.
    pub(super) fn shows_unified(&self) -> bool {
        self.config.mail.unified_inbox && self.tree.accounts.len() > 1
    }

    /// The lines of `view`, of `account` or of all.
    pub(super) fn unified_entries(&self, view: Unified, account: Option<AccountId>) -> Vec<Entry> {
        let Ok(mail) = &self.mail else {
            return Vec::new();
        };
        let folders = self.tree.unified_folders(view, account);
        mail.spread_entries(&folders, view.filter(), self.config.mail.conversations)
    }

    /// What the list of `view` is called: "Inbox", or with one account's
    /// part of it, "Unread · ada@example.org".
    pub(super) fn unified_name(&self, view: Unified, account: Option<AccountId>) -> String {
        let name = account.and_then(|id| self.tree.accounts.iter().find(|a| a.id == id));
        match name {
            Some(node) => format!("{} · {}", view.title(), node.name),
            None => view.title(),
        }
    }

    /// Opens a list of the unified inbox, as `open_folder` opens a folder.
    pub(super) fn open_unified(
        &mut self,
        view: Unified,
        account: Option<AccountId>,
        cx: &mut Context<Self>,
    ) {
        let entries = self.unified_entries(view, account);
        let Ok(mail) = &mut self.mail else {
            return;
        };
        if view.shows_recipients() != self.show_recipients {
            self.show_recipients = view.shows_recipients();
            mail.clear_rows();
        }
        // Inbox tabs belong to one account's inbox.
        self.tabs.clear();
        self.tab = 0;
        self.category_unread.clear();
        self.folder = None;
        self.unified = Some((view, account));
        self.listing = Some(Listing::Unified { view, account });
        self.entries = entries;
        self.reset_list(false);
        self.selected = (!self.entries.is_empty()).then_some(0);
        self.checked.clear();
        self.checked_all = false;
        self.page_pick = None;
        self.picked = None;
        self.menu = None;
        self.show_list();
        cx.notify();
    }

    /// Lists again what was listed before a search: the folder, or the
    /// unified inbox's list.
    pub(super) fn open_listed(&mut self, cx: &mut Context<Self>) {
        if let Some(folder) = self.folder {
            self.open_folder(folder, cx);
        } else if let Some((view, account)) = self.unified {
            self.open_unified(view, account, cx);
        }
    }

    /// Settings > General > Unified inbox. On, the accounts fold under
    /// it; off, they open again, and a list of it gives way to the first
    /// inbox.
    pub(super) fn set_unified_inbox(&mut self, on: bool, cx: &mut Context<Self>) {
        if self.config.mail.unified_inbox == on {
            return;
        }
        self.config.mail.unified_inbox = on;
        self.save_config();
        self.open_accounts.clear();
        self.all_accounts_open = true;
        self.rebuild_nav();
        if !on && self.unified.is_some() {
            self.unified = None;
            if matches!(self.listing, Some(Listing::Unified { .. })) {
                self.listing = None;
                self.reader = None;
                self.reading = false;
                self.card_seq += 1;
                self.open_default_folder(cx);
            }
        }
        cx.notify();
    }
}
