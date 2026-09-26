// SPDX-License-Identifier: GPL-3.0-or-later

//! One account at a time: with several accounts the folder pane shows the
//! one picked in the account card, as webmail does, unless the settings
//! ask for all of them. The list, search, Go to and new mail follow that
//! account. The unread count on the taskbar icon and new-mail
//! notifications still cover every account, so no mail goes unseen.

use gpui::Context;
use katna_core::AccountId;
use katna_core::config::AccountsShown;
use katna_store::FolderId;

use super::MailWindow;

impl MailWindow {
    /// The account the folder pane shows, or `None` when it shows all.
    pub(super) fn shown_account(&self) -> Option<AccountId> {
        if self.config.mail.accounts_shown == AccountsShown::All {
            return None;
        }
        let current = &self.config.mail.current_account;
        let with_folders = |id: AccountId| self.tree.accounts.iter().any(|a| a.id == id);
        let mail = || self.accounts.iter().filter(|a| a.kind.is_mail());
        mail()
            .find(|a| !current.is_empty() && a.address.eq_ignore_ascii_case(current))
            .or_else(|| mail().find(|a| with_folders(a.id)))
            .or_else(|| mail().next())
            .map(|a| a.id)
    }

    /// The folder to open first: in the shown account, or in any.
    pub(super) fn default_folder(&self) -> Option<(FolderId, Vec<String>)> {
        self.tree.default_folder_in(self.shown_account())
    }

    /// Makes `account` the one on show, remembered for next time.
    pub(super) fn set_shown_account(&mut self, account: AccountId) {
        let Some(address) = self
            .accounts
            .iter()
            .find(|a| a.id == account)
            .map(|a| a.address.to_lowercase())
        else {
            return;
        };
        if self.config.mail.current_account != address {
            self.config.mail.current_account = address;
            self.save_config();
        }
    }

    /// Before `folder` opens: with one account at a time, its account
    /// becomes the one on show, as when a notification opens mail of
    /// another account.
    pub(super) fn follow_folder_account(&mut self, folder: FolderId) {
        if self.config.mail.accounts_shown == AccountsShown::All {
            return;
        }
        if let Some(account) = self.tree.account_of(folder)
            && self.shown_account() != Some(account)
        {
            self.set_shown_account(account);
            self.rebuild_nav();
        }
    }

    /// From the account card: shows `account` and opens its inbox.
    pub(super) fn switch_account(&mut self, account: AccountId, cx: &mut Context<Self>) {
        self.set_shown_account(account);
        self.clear_search(cx);
        let folder = self
            .tree
            .role_folder(account, crate::sidebar::Role::Inbox)
            .or_else(|| self.tree.default_folder_in(Some(account)).map(|(f, _)| f));
        self.rebuild_nav();
        match folder {
            Some(folder) => self.open_folder(folder, cx),
            None => {
                // Its folders come with the first sync.
                self.listing = None;
                self.folder = None;
                self.entries.clear();
                self.tabs.clear();
            }
        }
        self.card_seq += 1;
        cx.notify();
    }
}
