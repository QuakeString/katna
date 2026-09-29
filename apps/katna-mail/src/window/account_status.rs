// SPDX-License-Identifier: GPL-3.0-or-later

//! The line under an account in a page's side list when its lists could
//! not come: one short sentence says why, with the one click that fixes it
//! ("Sign in again to show calendars", "Change password", "Try again"),
//! as the daemon
//! reports each account's sync (`katna_dbus::calendar_state`,
//! `katna_dbus::task_state`, `katna_dbus::contacts_state`). Each page gives its own
//! words ([`Say`]); the shape, the states and the fixes are the same on
//! every page.

use std::collections::{HashMap, HashSet};

use gpui::{AnyElement, Context, FontWeight, SharedString, Task, Window, div, prelude::*, rgba};
use katna_dbus::task_state;
use katna_ui::px;

use super::MailWindow;
use super::settings_page::Section;
use crate::daemon::{self, AccountState, AddError};
use crate::theme::Theme;
use crate::widgets::tip;

/// Where one page's accounts stand, as last read from the daemon.
#[derive(Default)]
pub(super) struct AccountStatus {
    status: HashMap<i64, AccountState>,
    /// Accounts being fixed now (signing in, or syncing again).
    busy: HashSet<i64>,
    task: Option<Task<()>>,
    fixing: HashMap<i64, Task<()>>,
}

impl AccountStatus {
    /// Whether account `id` shows a line even when it has lists: the
    /// lists it shows are old ones that no longer sync.
    pub(super) fn failing(&self, id: i64) -> bool {
        self.status.get(&id).is_some_and(|s| {
            matches!(
                s.state.as_str(),
                task_state::NEEDS_SIGN_IN
                    | task_state::USE_SIGN_IN
                    | task_state::NOT_ENABLED
                    | task_state::ERROR
            )
        })
    }
}

/// The page whose accounts a line is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Of {
    Calendar,
    Tasks,
    Contacts,
}

/// What a page says; each page answers with its own words.
pub(super) enum Say<'a> {
    /// The button of an OAuth2 account whose sign-in did not allow this.
    SignIn,
    /// The provider did not let Katna in when signing in again.
    SignInRefused {
        provider: &'a str,
    },
    /// Signed in again; the lists are on their way.
    SignedIn {
        address: &'a str,
    },
    /// The server refused the password of an account without OAuth2.
    Refused,
    /// The button that opens Settings > Accounts to change it.
    ChangePassword,
    ChangePasswordTooltip,
    /// The provider has the API switched off for Katna.
    NotEnabled,
    /// The last sync failed, and the server said why (in English).
    Error {
        reason: &'a str,
    },
    /// The last sync failed.
    Failed,
    /// The account has nothing Katna can reach.
    None,
    /// The same, with what the server answered (in English).
    NoneWhy {
        reason: &'a str,
    },
    /// A password account at a provider that lets Katna in only through
    /// its own sign-in (Google, Microsoft).
    UseSignIn {
        provider: &'a str,
    },
    /// The button that signs in with that provider.
    SignInWith {
        provider: &'a str,
    },
    /// Not synced yet.
    Looking,
    TryAgain,
    TryAgainTooltip,
    /// While a fix runs.
    Fixing,
}

/// The click that fixes an account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fix {
    SignIn,
    /// The password was refused: change it in Settings > Accounts.
    Password,
    TryAgain,
    None,
}

impl Of {
    fn say(self, say: Say<'_>) -> String {
        match self {
            Self::Calendar => super::calendar::say(say),
            Self::Tasks => super::tasks_page::say(say),
            Self::Contacts => super::contacts_page::say(say),
        }
    }

    /// How far the line sits in, under the account's heading.
    fn inset(self) -> f32 {
        match self {
            Self::Calendar => 8.0,
            Self::Tasks => 24.0,
            // Under the address, past the account's icon.
            Self::Contacts => 54.0,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Calendar => "calendar",
            Self::Tasks => "tasks",
            Self::Contacts => "contacts",
        }
    }
}

impl MailWindow {
    fn account_status(&mut self, of: Of) -> &mut AccountStatus {
        match of {
            Of::Calendar => &mut self.calendar.accounts,
            Of::Tasks => &mut self.tasks.accounts,
            Of::Contacts => &mut self.contacts.accounts,
        }
    }

    fn account_status_ref(&self, of: Of) -> &AccountStatus {
        match of {
            Of::Calendar => &self.calendar.accounts,
            Of::Tasks => &self.tasks.accounts,
            Of::Contacts => &self.contacts.accounts,
        }
    }

    /// Asks the daemon where each account of page `of` stands.
    pub(super) fn load_account_status(&mut self, of: Of, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let task = cx.spawn(async move |this, cx| {
            let status = cx
                .background_executor()
                .spawn(async move {
                    match of {
                        Of::Calendar => daemon::calendar_status(&connection).await,
                        Of::Tasks => crate::tasks::status(&connection).await,
                        Of::Contacts => daemon::contacts_status(&connection).await,
                    }
                })
                .await;
            this.update(cx, |this, cx| match status {
                Ok(status) => {
                    let page = this.account_status(of);
                    if page.status != status {
                        page.status = status;
                        cx.notify();
                    }
                }
                Err(err) => {
                    tracing::info!(%err, page = of.name(), "reading the account status failed")
                }
            })
            .ok();
        });
        self.account_status(of).task = Some(task);
    }

    fn fix_account(
        &mut self,
        of: Of,
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
        let address = self
            .accounts
            .iter()
            .find(|a| a.id.0 == id)
            .map(|a| a.address.clone())
            .unwrap_or_default();
        let page = self.account_status(of);
        let provider = page.status.get(&id).and_then(AccountState::provider);
        if fix == Fix::None || !page.busy.insert(id) {
            return;
        }
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
                                    AddError::Password(_) => of.say(Say::SignInRefused {
                                        provider: provider.name(),
                                    }),
                                    AddError::Other(err) => err,
                                })
                        }
                        _ => daemon::sync_now(&connection, id).await,
                    }
                })
                .await;
            this.update(cx, |this, cx| {
                this.account_status(of).busy.remove(&id);
                match result {
                    Ok(()) if fix == Fix::SignIn => {
                        let text = of.say(Say::SignedIn {
                            address: address.as_str(),
                        });
                        this.show_snackbar(text, None, cx);
                    }
                    Ok(()) => {}
                    Err(err) => this.show_snackbar(err, None, cx),
                }
                this.load_account_status(of, cx);
                cx.notify();
            })
            .ok();
        });
        self.account_status(of).fixing.insert(id, task);
        cx.notify();
    }

    /// The line under account `id` in page `of`'s side list: why its
    /// lists did not come, and the click that fixes it.
    pub(super) fn render_account_status(
        &self,
        of: Of,
        id: i64,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let page = self.account_status_ref(of);
        let status = page.status.get(&id);
        let state = status.map_or(task_state::OK, |s| s.state.as_str());
        let detail = status.map_or("", |s| s.detail.as_str());
        let signs_in = status.is_some_and(|s| s.sign_in.is_some());
        let provider = status
            .and_then(AccountState::provider)
            .map_or("", |p| p.name());
        let (text, fix) = match state {
            task_state::NEEDS_SIGN_IN if signs_in => (String::new(), Fix::SignIn),
            task_state::USE_SIGN_IN if !provider.is_empty() => {
                (of.say(Say::UseSignIn { provider }), Fix::SignIn)
            }
            task_state::NEEDS_SIGN_IN => (of.say(Say::Refused), Fix::Password),
            task_state::NOT_ENABLED => (of.say(Say::NotEnabled), Fix::TryAgain),
            task_state::ERROR if detail.is_empty() => (of.say(Say::Failed), Fix::TryAgain),
            task_state::ERROR => (of.say(Say::Error { reason: detail }), Fix::TryAgain),
            task_state::NONE if detail.is_empty() => (of.say(Say::None), Fix::TryAgain),
            task_state::NONE => (of.say(Say::NoneWhy { reason: detail }), Fix::TryAgain),
            _ => (of.say(Say::Looking), Fix::None),
        };
        let action = match fix {
            Fix::None => None,
            _ if page.busy.contains(&id) => Some(
                div()
                    .text_color(rgba(th.text_faint))
                    .child(of.say(Say::Fixing))
                    .into_any_element(),
            ),
            _ => {
                let (label, hint) = match fix {
                    Fix::SignIn => (
                        if signs_in {
                            of.say(Say::SignIn)
                        } else {
                            of.say(Say::SignInWith { provider })
                        },
                        katna_i18n::tr!("sign-in-again-tooltip", provider = provider),
                    ),
                    Fix::Password => (
                        of.say(Say::ChangePassword),
                        of.say(Say::ChangePasswordTooltip),
                    ),
                    _ => (of.say(Say::TryAgain), of.say(Say::TryAgainTooltip)),
                };
                Some(
                    div()
                        .id(SharedString::from(format!(
                            "{}-account-fix-{id}",
                            of.name()
                        )))
                        .cursor_pointer()
                        .rounded(px(4.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(th.accent))
                        .hover(|s| s.underline())
                        .tooltip(tip(hint, th))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.fix_account(of, id, fix, window, cx)
                        }))
                        .child(label)
                        .into_any_element(),
                )
            }
        };
        // A sign-in's button says it all; the others say why first.
        let text = (!text.is_empty()).then_some(text);
        div()
            .pl(px(of.inset()))
            .pr(px(of.inset().min(16.0)))
            .pb(px(6.0))
            .flex()
            .flex_col()
            .gap(px(2.0))
            .text_size(px(13.0))
            .line_height(px(18.0))
            .text_color(rgba(th.text_faint))
            .children(text)
            .children(action)
            .into_any_element()
    }
}
