// SPDX-License-Identifier: GPL-3.0-or-later

//! Every mail account in the Contacts page's column (`docs/ARCHITECTURE.md`
//! §8.6), as the Calendar lists them: a click lists the people saved in
//! it, and one short line under an account whose contacts did not come
//! says why, with the one click that fixes it ("Sign in again to show
//! contacts", "Try again", the password to change), as the daemon reports
//! each account's contacts sync (`katna_dbus::contacts_state`).

use std::collections::{HashMap, HashSet};

use gpui::{AnyElement, Context, FontWeight, SharedString, Task, Window, div, prelude::*, rgba};
use katna_core::AccountKind;
use katna_dbus::contacts_state;
use katna_i18n::tr;
use katna_ui::{Ripple, px};

use super::MailWindow;
use super::contacts_edit::visible;
use super::contacts_page::View;
use super::settings_page::Section;
use crate::daemon::{self, AccountState, AddError};
use crate::data::SavedBook;
use crate::theme::Theme;
use crate::widgets::{icon, tip};

/// Where the accounts' contacts stand, as last read from the daemon.
#[derive(Default)]
pub(super) struct AccountStatus {
    status: HashMap<i64, AccountState>,
    /// Accounts being fixed now (signing in, or syncing again).
    busy: HashSet<i64>,
    task: Option<Task<()>>,
    fixing: HashMap<i64, Task<()>>,
}

/// What an account without contacts shows, and the click that fixes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fix {
    SignIn,
    /// The password was refused: change it in Settings > Accounts.
    Password,
    TryAgain,
    None,
}

/// What the line under an account says, and its fix, for `state` and
/// `detail` as the daemon gave them; `signs_in` for an OAuth2 account.
fn note(state: &str, detail: &str, signs_in: bool) -> (String, Fix) {
    match state {
        contacts_state::NEEDS_SIGN_IN if signs_in => (String::new(), Fix::SignIn),
        contacts_state::NEEDS_SIGN_IN => (tr!("contacts-account-password"), Fix::Password),
        contacts_state::ERROR if detail.is_empty() => {
            (tr!("contacts-account-failed"), Fix::TryAgain)
        }
        contacts_state::ERROR => (
            tr!("contacts-account-error", reason = detail),
            Fix::TryAgain,
        ),
        contacts_state::NONE => (tr!("contacts-account-none"), Fix::TryAgain),
        _ => (tr!("contacts-account-looking"), Fix::None),
    }
}

impl MailWindow {
    /// Asks the daemon where each account's contacts sync stands.
    pub(super) fn load_contacts_status(&mut self, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        self.contacts.accounts.task = Some(cx.spawn(async move |this, cx| {
            let status = cx
                .background_executor()
                .spawn(async move { daemon::contacts_status(&connection).await })
                .await;
            this.update(cx, |this, cx| match status {
                Ok(status) => {
                    let page = &mut this.contacts.accounts;
                    if page.status != status {
                        page.status = status;
                        cx.notify();
                    }
                }
                Err(err) => tracing::info!(%err, "reading the contacts status failed"),
            })
            .ok();
        }));
    }

    fn fix_contacts_account(
        &mut self,
        id: i64,
        fix: Fix,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if fix == Fix::Password {
            self.open_settings_page(Section::Accounts, window, cx);
            return;
        }
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let page = &mut self.contacts.accounts;
        let provider = page.status.get(&id).and_then(|s| s.sign_in);
        if fix == Fix::None || !page.busy.insert(id) {
            return;
        }
        let address = self
            .accounts
            .iter()
            .find(|a| a.id.0 == id)
            .map(|a| a.address.clone())
            .unwrap_or_default();
        let task = cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    match (fix, provider) {
                        (Fix::SignIn, Some(provider)) => {
                            daemon::sign_in(&connection, provider, Some(id), "")
                                .await
                                .map(|_| ())
                                .map_err(|err| match err {
                                    AddError::Password(_) => tr!(
                                        "contacts-account-sign-in-refused",
                                        provider = provider.name()
                                    ),
                                    AddError::Other(err) => err,
                                })
                        }
                        _ => daemon::sync_now(&connection, id).await,
                    }
                })
                .await;
            this.update(cx, |this, cx| {
                this.contacts.accounts.busy.remove(&id);
                this.contacts.accounts.fixing.remove(&id);
                match result {
                    Ok(()) if fix == Fix::SignIn => {
                        let text = tr!("contacts-account-signed-in", address = address.as_str());
                        this.show_snackbar(text, None, cx);
                    }
                    Ok(()) => {}
                    Err(err) => this.show_snackbar(err, None, cx),
                }
                this.load_contacts_status(cx);
                cx.notify();
            })
            .ok();
        });
        self.contacts.accounts.fixing.insert(id, task);
        cx.notify();
    }

    /// "Accounts" in the column: every mail account with how many people
    /// are saved in it, and why one shows none.
    pub(super) fn contacts_accounts_nav(
        &self,
        book: Option<&SavedBook>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        // A mail archive on this computer has no address book.
        let accounts: Vec<(i64, String)> = self
            .accounts
            .iter()
            .filter(|a| a.kind.is_mail() && a.kind != AccountKind::Local)
            .map(|a| (a.id.0, a.address.clone()))
            .collect();
        let rows = accounts.into_iter().enumerate().map(|(ix, (id, address))| {
            let on = self.contacts.view == View::Account(id);
            let (people, books) = book.map_or((0, 0), |b| {
                let people = visible(&b.people, &self.contacts.hidden)
                    .into_iter()
                    .filter(|&i| {
                        b.people[i]
                            .accounts
                            .iter()
                            .any(|a| a.is_some_and(|a| a.0 == id))
                    })
                    .count();
                let books = b
                    .books
                    .iter()
                    .filter(|x| x.account.is_some_and(|a| a.0 == id))
                    .count();
                (people, books)
            });
            let row =
                div()
                    .id(("contacts-account", ix))
                    .relative()
                    .overflow_hidden()
                    .h(px(36.0))
                    .mr(px(12.0))
                    .pl(px(20.0))
                    .pr(px(16.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(14.0))
                    .rounded_r_full()
                    .cursor_pointer()
                    .when(on, |d| d.bg(rgba(th.nav_selected)))
                    .when(!on, |d| d.hover(|s| s.bg(rgba(th.hover))))
                    .child(Ripple::new(("contacts-account", ix), rgba(th.ripple)))
                    .child(icon(
                        "cloud",
                        if on {
                            th.nav_selected_text
                        } else {
                            th.text_dim
                        },
                        20.0,
                    ))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .text_size(px(14.0))
                            .when(on, |d| d.font_weight(FontWeight::SEMIBOLD))
                            .text_color(rgba(if on { th.nav_selected_text } else { th.text }))
                            .child(address),
                    )
                    .when(people > 0, |d| {
                        d.child(
                            div()
                                .flex_none()
                                .text_size(px(12.0))
                                .text_color(rgba(if on {
                                    th.nav_selected_text
                                } else {
                                    th.text_faint
                                }))
                                .child(katna_i18n::format::number(people as u64)),
                        )
                    })
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.set_contacts_view(View::Account(id), cx)
                    }));
            div()
                .flex()
                .flex_col()
                .child(row)
                .children(self.render_contacts_account_note(id, books > 0, th, cx))
        });
        div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(
                div()
                    .pt(px(18.0))
                    .pb(px(6.0))
                    .pl(px(20.0))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgba(th.text))
                    .child(tr!("contacts-accounts")),
            )
            .children(rows)
            .into_any_element()
    }

    /// The line under account `id` when its contacts did not come (or,
    /// without `books`, have not come yet): why, and the click that fixes
    /// it. `None` when all is well.
    fn render_contacts_account_note(
        &self,
        id: i64,
        books: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let page = &self.contacts.accounts;
        let status = page.status.get(&id);
        let state = status.map_or(contacts_state::OK, |s| s.state.as_str());
        if state == contacts_state::OK && books {
            return None;
        }
        let detail = status.map_or("", |s| s.detail.as_str());
        let signs_in = status.is_some_and(|s| s.sign_in.is_some());
        let (text, fix) = note(state, detail, signs_in);
        let busy = page.busy.contains(&id);
        let action = match fix {
            Fix::None => None,
            _ if busy => Some(
                div()
                    .text_color(rgba(th.text_faint))
                    .child(tr!("contacts-account-fixing"))
                    .into_any_element(),
            ),
            _ => {
                let (label, hint) = match fix {
                    Fix::SignIn => {
                        let provider = status.and_then(|s| s.sign_in).map_or("", |p| p.name());
                        (
                            tr!("contacts-account-sign-in"),
                            tr!("sign-in-again-tooltip", provider = provider),
                        )
                    }
                    Fix::Password => (
                        tr!("contacts-account-change-password"),
                        tr!("contacts-account-change-password-tooltip"),
                    ),
                    _ => (
                        tr!("contacts-account-try-again"),
                        tr!("contacts-account-try-again-tooltip"),
                    ),
                };
                Some(
                    div()
                        .id(SharedString::from(format!("contacts-account-fix-{id}")))
                        .cursor_pointer()
                        .rounded(px(4.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(th.accent))
                        .hover(|s| s.underline())
                        .tooltip(tip(hint, th))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.fix_contacts_account(id, fix, window, cx)
                        }))
                        .child(label)
                        .into_any_element(),
                )
            }
        };
        // A sign-in's button says it all; the others say why first.
        let text = (fix != Fix::SignIn).then_some(text);
        Some(
            div()
                .pl(px(54.0))
                .pr(px(16.0))
                .pb(px(6.0))
                .flex()
                .flex_col()
                .gap(px(2.0))
                .text_size(px(13.0))
                .line_height(px(18.0))
                .text_color(rgba(th.text_faint))
                .children(text)
                .children(action)
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_state_has_its_fix() {
        assert_eq!(note(contacts_state::NEEDS_SIGN_IN, "", true).1, Fix::SignIn);
        assert_eq!(
            note(contacts_state::NEEDS_SIGN_IN, "401", false).1,
            Fix::Password
        );
        let (text, fix) = note(contacts_state::ERROR, "timed out", false);
        assert!(text.contains("timed out"));
        assert_eq!(fix, Fix::TryAgain);
        assert_eq!(note(contacts_state::NONE, "", false).1, Fix::TryAgain);
        assert_eq!(note(contacts_state::OK, "", false).1, Fix::None);
    }
}
