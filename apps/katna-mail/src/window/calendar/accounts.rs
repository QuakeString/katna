// SPDX-License-Identifier: GPL-3.0-or-later

//! Every account in the Calendar page's side list, also one whose
//! calendars did not come: one short line under it says why, with the one
//! click that fixes it ("Sign in again to show calendars", "Try again"),
//! as the daemon reports each account's calendar sync
//! (`katna_dbus::calendar_state`).

use std::collections::{HashMap, HashSet};

use gpui::{AnyElement, Context, FontWeight, SharedString, Task, div, prelude::*, rgba};
use katna_dbus::calendar_state;
use katna_i18n::tr;
use katna_ui::px;

use super::super::MailWindow;
use crate::daemon::{self, AddError, CalendarStatus};
use crate::theme::Theme;
use crate::widgets::tip;

/// Where the accounts' calendars stand, as last read from the daemon.
#[derive(Default)]
pub(in crate::window) struct AccountStatus {
    status: HashMap<i64, CalendarStatus>,
    /// Accounts being fixed now (signing in, or syncing again).
    busy: HashSet<i64>,
    task: Option<Task<()>>,
    fixing: HashMap<i64, Task<()>>,
}

/// What an account without calendars shows, and the click that fixes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fix {
    SignIn,
    TryAgain,
    None,
}

impl MailWindow {
    /// Asks the daemon where each account's calendar sync stands.
    pub(super) fn load_calendar_status(&mut self, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        self.calendar.accounts.task = Some(cx.spawn(async move |this, cx| {
            let status = cx
                .background_executor()
                .spawn(async move { daemon::calendar_status(&connection).await })
                .await;
            this.update(cx, |this, cx| match status {
                Ok(status) => {
                    let page = &mut this.calendar.accounts;
                    if page.status != status {
                        page.status = status;
                        cx.notify();
                    }
                }
                Err(err) => tracing::info!(%err, "reading the calendar status failed"),
            })
            .ok();
        }));
    }

    fn fix_calendar_account(&mut self, id: i64, fix: Fix, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let page = &mut self.calendar.accounts;
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
                                        "calendar-account-sign-in-refused",
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
                this.calendar.accounts.busy.remove(&id);
                match result {
                    Ok(()) if fix == Fix::SignIn => {
                        let text = tr!("calendar-account-signed-in", address = address.as_str());
                        this.show_snackbar(text, None, cx);
                    }
                    Ok(()) => {}
                    Err(err) => this.show_snackbar(err, None, cx),
                }
                this.load_calendar_status(cx);
                cx.notify();
            })
            .ok();
        });
        page.fixing.insert(id, task);
        cx.notify();
    }

    /// The line under account `id` when it shows no calendars: why, and
    /// the click that fixes it.
    pub(super) fn render_calendar_account_note(
        &self,
        id: i64,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let page = &self.calendar.accounts;
        let status = page.status.get(&id);
        let state = status.map_or(calendar_state::OK, |s| s.state.as_str());
        let detail = status.map_or("", |s| s.detail.as_str());
        let signs_in = status.is_some_and(|s| s.sign_in.is_some());
        let (text, fix) = match state {
            calendar_state::NEEDS_SIGN_IN if signs_in => (String::new(), Fix::SignIn),
            calendar_state::NEEDS_SIGN_IN => (tr!("calendar-account-refused"), Fix::TryAgain),
            calendar_state::NOT_ENABLED => (tr!("calendar-account-not-enabled"), Fix::TryAgain),
            calendar_state::ERROR if detail.is_empty() => {
                (tr!("calendar-account-failed"), Fix::TryAgain)
            }
            calendar_state::ERROR => (
                tr!("calendar-account-error", reason = detail),
                Fix::TryAgain,
            ),
            calendar_state::NONE => (tr!("calendar-account-none"), Fix::None),
            _ => (tr!("calendar-account-looking"), Fix::None),
        };
        let busy = page.busy.contains(&id);
        let action =
            match fix {
                Fix::None => None,
                _ if busy => Some(
                    div()
                        .text_color(rgba(th.text_faint))
                        .child(tr!("calendar-account-fixing"))
                        .into_any_element(),
                ),
                _ => {
                    let (label, hint) = if fix == Fix::SignIn {
                        let provider = status.and_then(|s| s.sign_in).map_or("", |p| p.name());
                        (
                            tr!("calendar-account-sign-in"),
                            tr!("sign-in-again-tooltip", provider = provider),
                        )
                    } else {
                        (
                            tr!("calendar-account-try-again"),
                            tr!("calendar-account-try-again-tooltip"),
                        )
                    };
                    Some(
                        div()
                            .id(SharedString::from(format!("calendar-account-fix-{id}")))
                            .cursor_pointer()
                            .rounded(px(4.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.accent))
                            .hover(|s| s.underline())
                            .tooltip(tip(hint, th))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.fix_calendar_account(id, fix, cx)
                            }))
                            .child(label)
                            .into_any_element(),
                    )
                }
            };
        // A sign-in's button says it all; the others say why first.
        let text = (fix != Fix::SignIn).then_some(text);
        div()
            .pl(px(8.0))
            .pr(px(8.0))
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
