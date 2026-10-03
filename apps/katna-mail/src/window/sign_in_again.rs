// SPDX-License-Identifier: GPL-3.0-or-later

//! "Google asks you to sign in again": a note at the bottom of the window
//! when an account that signed in with Google or Microsoft (OAuth2) stopped
//! being let in, because the sign-in was revoked, expired or its password
//! changed. "Sign in" opens the provider's page in the browser again
//! (`SignIn` for that account); nothing else fixes it.

use std::collections::HashSet;

use gpui::{AnyElement, Context, Task, Window, div, prelude::*, rgba};
use katna_core::OAuthProvider;
use katna_i18n::tr;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;

use super::MailWindow;
use crate::daemon::{self, AddError};
use crate::theme::Theme;
use crate::widgets::{elevation, icon, tip};

/// An account whose provider asks to sign in again.
type SignedOut = (i64, String, OAuthProvider);

pub(super) struct SignInAgain {
    accounts: Vec<SignedOut>,
    /// Accounts whose note the user closed, this session.
    dismissed: HashSet<i64>,
    /// The account shown last, kept while the note slides away.
    last: Option<SignedOut>,
    /// The account signing in in the browser now.
    busy: Option<i64>,
    shown: Spring,
    _check: Option<Task<()>>,
    _sign_in: Option<Task<()>>,
}

impl Default for SignInAgain {
    fn default() -> Self {
        Self {
            accounts: Vec::new(),
            dismissed: HashSet::new(),
            last: None,
            busy: None,
            shown: Spring::new(motion::SLIDE, 0.0),
            _check: None,
            _sign_in: None,
        }
    }
}

impl SignInAgain {
    fn current(&self) -> Option<&SignedOut> {
        self.accounts
            .iter()
            .find(|(id, ..)| !self.dismissed.contains(id))
    }
}

impl MailWindow {
    /// Asks the daemon which accounts have to sign in again; after every
    /// change it reports.
    pub(super) fn check_signed_out(&mut self, cx: &mut Context<Self>) {
        self.remote.load_provider_pictures();
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        self.sign_in_again._check = Some(cx.spawn(async move |this, cx| {
            let accounts = cx
                .background_executor()
                .spawn(async move { daemon::signed_out(&connection).await })
                .await;
            this.update(cx, |this, cx| {
                let Ok(accounts) = accounts else {
                    return;
                };
                if this.sign_in_again.accounts != accounts {
                    this.sign_in_again.accounts = accounts;
                    cx.notify();
                }
            })
            .ok();
        }));
    }

    fn sign_in_again(&mut self, cx: &mut Context<Self>) {
        let Some((id, address, provider)) = self.sign_in_again.current().cloned() else {
            return;
        };
        self.sign_in_account(id, address, provider, cx);
    }

    /// Opens `provider`'s sign-in page for account `id` (`address`), as
    /// the note's Sign in does; also from the folder pane's menu.
    pub(super) fn sign_in_account(
        &mut self,
        id: i64,
        address: String,
        provider: OAuthProvider,
        cx: &mut Context<Self>,
    ) {
        if self.sign_in_again.busy.is_some() {
            return;
        }
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let note = &mut self.sign_in_again;
        note.busy = Some(id);
        note._sign_in = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { daemon::sign_in(&connection, provider, Some(id), "").await })
                .await;
            this.update(cx, |this, cx| {
                this.sign_in_again.busy = None;
                let text = match result {
                    Ok(_) => {
                        this.sign_in_again.accounts.retain(|(a, ..)| *a != id);
                        tr!("sign-in-again-done", address = address.as_str())
                    }
                    Err(AddError::Password(detail)) => {
                        tracing::info!(%detail, "sign-in refused");
                        tr!("add-account-sign-in-refused", provider = provider.name())
                    }
                    Err(AddError::Other(err)) => err,
                };
                this.show_snackbar(text, None, cx);
                this.check_signed_out(cx);
            })
            .ok();
        }));
        cx.notify();
    }

    fn close_sign_in_again(&mut self, cx: &mut Context<Self>) {
        let note = &mut self.sign_in_again;
        if let Some((id, ..)) = note.current().cloned() {
            note.dismissed.insert(id);
        }
        cx.notify();
    }

    pub(super) fn render_sign_in_again(
        &mut self,
        th: &Theme,
        window: &Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let note = &mut self.sign_in_again;
        let current = note.current().cloned();
        note.shown.set(if current.is_some() { 1.0 } else { 0.0 });
        if current.is_some() {
            note.last = current;
        }
        let s = note.shown.tick(window, reduce);
        if note.shown.target() == 0.0 && note.shown.settled() {
            note.last = None;
            return None;
        }
        let (id, address, provider) = note.last.clone()?;
        let busy = note.busy == Some(id);
        let s = s.max(0.0);
        let text = tr!(
            "sign-in-again-text",
            provider = provider.name(),
            address = address.as_str()
        );
        // Same place and look as the snackbar, and above it and the crash
        // note while they show.
        let shape = self.layout.shape;
        let edge = lerp(24.0, 8.0, shape.phone);
        let above = if self.snackbar.is_some() { 64.0 } else { 0.0 }
            + if self.crash_notice.is_some() {
                64.0
            } else {
                0.0
            };
        let accent = if th.dark { th.nav_selected } else { 0xa8c7faff };
        let action = div()
            .id("sign-in-again")
            .flex_none()
            .px(px(12.0))
            .py(px(8.0))
            .rounded(px(4.0))
            .text_color(rgba(accent))
            .font_weight(gpui::FontWeight::MEDIUM);
        let action = if busy {
            action.opacity(0.7).child(tr!("sign-in-again-waiting"))
        } else {
            action
                .cursor_pointer()
                .hover(|s| s.bg(rgba(0xffffff1f)))
                .tooltip(tip(
                    tr!("sign-in-again-tooltip", provider = provider.name()),
                    th,
                ))
                .on_click(cx.listener(|this, _, _, cx| this.sign_in_again(cx)))
                .child(tr!("sign-in-again-button"))
        };
        Some(
            div()
                .absolute()
                .left(px(edge))
                .when(shape.is_phone(), |d| d.right(px(edge)))
                .bottom(px(shape.bottom_bar() + above + lerp(-12.0, edge, s)))
                .opacity(s.min(1.0))
                .min_w(px(288.0))
                .max_w(px(640.0))
                .pl(px(16.0))
                .pr(px(4.0))
                .py(px(6.0))
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(8.0))
                .rounded(px(6.0))
                .bg(rgba(th.snackbar))
                .text_color(rgba(th.snackbar_text))
                .text_size(px(14.0))
                .shadow(elevation(th, 3.0))
                .child(
                    self.copyable(text, th)
                        .flex_1()
                        .min_w(px(180.0))
                        .py(px(8.0)),
                )
                .child(
                    div().flex().flex_row().items_center().child(action).child(
                        div()
                            .id("sign-in-again-close")
                            .size(px(36.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_full()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(0xffffff1f)))
                            .tooltip(tip(tr!("sign-in-again-close"), th))
                            .on_click(cx.listener(|this, _, _, cx| this.close_sign_in_again(cx)))
                            .child(icon("close", th.snackbar_text, 18.0)),
                    ),
                )
                .into_any_element(),
        )
    }
}
