// SPDX-License-Identifier: GPL-3.0-or-later

//! The outbox: mail that has not gone out, because it waits for a
//! connection or for its account to be signed in again, or because the
//! server refused it. "Outbox" shows in the navigation only while there is
//! some, and its list says why in plain words, with Try again, Edit and
//! Delete.

use gpui::{AnyElement, ClickEvent, Context, FontWeight, Window, div, prelude::*, rgba};
use katna_core::AccountId;
use katna_dbus::OutboxItem;
use katna_i18n::tr;
use katna_store::MessageId;
use katna_ui::px;
use katna_ui::tokens::{space, text};

use super::super::MailWindow;
use super::scheduled::{list_dialog, unsent_from_raw};
use super::security;
use crate::daemon;
use crate::theme::Theme;
use crate::widgets::{ButtonStyle, button, icon, outlined_button};

/// The navigation row of the outbox.
pub(in crate::window) const NAV_KEY: &str = "katna:outbox";

/// How a message waiting to be sent begins its detail when the account's
/// login was refused (`katna_sync::outbox::SIGN_IN`).
const SIGN_IN: &str = "sign-in: ";
/// And when there was no connection.
const OFFLINE: &str = "offline: ";

/// Why a message in the outbox has not gone out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Why {
    /// The server refused it for good: Try again, Edit or Delete.
    NotSent(Reason),
    /// It waits for its account to be signed in again.
    SignIn,
    /// It waits for a connection.
    Offline,
    /// The server refused it; Katna tries again by itself.
    Retrying(Reason),
}

impl Why {
    /// Whether it waits for the user (amber), not only for time (grey).
    pub(super) fn needs_you(&self) -> bool {
        matches!(self, Self::NotSent(_) | Self::SignIn)
    }
}

/// What a server's refusal means, in plain words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Reason {
    NoRecipients,
    Address,
    TooLarge,
    Blocked,
    Gone,
    Refused,
}

impl Reason {
    /// Reads the server's answer.
    pub(super) fn of(detail: &str) -> Self {
        let d = detail.to_ascii_lowercase();
        let has = |words: &[&str]| words.iter().any(|w| d.contains(w));
        if has(&["no recipients"]) {
            Self::NoRecipients
        } else if has(&["gone from the store"]) {
            Self::Gone
        } else if has(&[
            "552",
            "5.3.4",
            "5.2.3",
            "too large",
            "too big",
            "size limit",
        ]) {
            Self::TooLarge
        } else if has(&[
            "not an address",
            "5.1.1",
            "5.1.2",
            "553",
            "user unknown",
            "no such user",
            "does not exist",
            "mailbox unavailable",
            "recipient address rejected",
        ]) {
            Self::Address
        } else if has(&["spam", "5.7.1", "blocked", "policy", "554"]) {
            Self::Blocked
        } else {
            Self::Refused
        }
    }

    fn text(self) -> String {
        match self {
            Self::NoRecipients => tr!("outbox-reason-no-recipients"),
            Self::Address => tr!("outbox-reason-address"),
            Self::TooLarge => tr!("outbox-reason-too-large"),
            Self::Blocked => tr!("outbox-reason-blocked"),
            Self::Gone => tr!("outbox-reason-gone"),
            Self::Refused => tr!("outbox-reason-refused"),
        }
    }
}

/// What the server's answer `detail` means, in plain words.
pub(in crate::window) fn reason_text(detail: &str) -> String {
    Reason::of(detail).text()
}

/// Why `item` is in the outbox; `None` for mail that waits only for its
/// undo delay or its scheduled time, sent or cancelled mail.
pub(super) fn why(item: &OutboxItem) -> Option<Why> {
    use katna_dbus::send_state;
    match item.state.as_str() {
        send_state::FAILED => Some(Why::NotSent(Reason::of(&item.detail))),
        send_state::QUEUED | send_state::SENDING if !item.detail.is_empty() => {
            Some(if item.detail.starts_with(SIGN_IN) {
                Why::SignIn
            } else if item.detail.starts_with(OFFLINE) {
                Why::Offline
            } else {
                Why::Retrying(Reason::of(&item.detail))
            })
        }
        _ => None,
    }
}

impl MailWindow {
    /// Keeps the outbox's list from the daemon's `items`; the navigation
    /// row comes and goes with it.
    pub(super) fn set_outbox(&mut self, items: &[OutboxItem]) {
        let outbox: Vec<OutboxItem> = items.iter().filter(|i| why(i).is_some()).cloned().collect();
        let before = (self.writing.outbox.len(), self.writing.outbox_needs_you());
        self.writing.outbox = outbox;
        if self.writing.outbox.is_empty() {
            self.writing.outbox_open = false;
        }
        if before != (self.writing.outbox.len(), self.writing.outbox_needs_you()) {
            self.rebuild_nav();
        }
    }

    pub(in crate::window) fn open_outbox(&mut self, cx: &mut Context<Self>) {
        self.writing.outbox_open = true;
        cx.notify();
    }

    /// The account's name, as the list shows it under the subject.
    fn outbox_account(&self, account: i64) -> String {
        self.accounts
            .iter()
            .find(|a| a.id == AccountId(account))
            .map(|a| a.address.clone())
            .unwrap_or_default()
    }

    /// Sends a refused message again now.
    fn retry_outbox(&mut self, id: i64, cx: &mut Context<Self>) {
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::retry_send(&connection, id).await
                })
                .await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(true) => this.show_snackbar(tr!("outbox-sending-again"), None, cx),
                    Ok(false) => {}
                    Err(err) => this.show_snackbar(err, None, cx),
                }
                this.scheduled_changed(cx);
            })
            .ok();
        })
        .detach();
    }

    /// Takes a message out of the outbox: to open it to edit (`edit`), or
    /// to forget it.
    fn take_out_of_outbox(
        &mut self,
        item: OutboxItem,
        edit: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if edit
            && self
                .compose
                .as_ref()
                .is_some_and(|c| !c.closing && c.touched(cx))
        {
            self.show_snackbar(tr!("schedule-open-first"), None, cx);
            return;
        }
        // Read it before the outbox forgets it.
        let raw = edit
            .then(|| {
                self.mail
                    .as_ref()
                    .ok()
                    .and_then(|mail| mail.raw(MessageId(item.message)))
            })
            .flatten();
        let connection = self.daemon.clone();
        let id = item.id;
        let failed = item.state == katna_dbus::send_state::FAILED;
        let account = item.account;
        cx.spawn_in(window, async move |this, cx| {
            let (result, opened) = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => match daemon::connect().await {
                            Ok(connection) => connection,
                            Err(err) => return (Err(err), None),
                        },
                    };
                    // Waiting mail is taken back first; then it is
                    // forgotten.
                    let result = if failed {
                        daemon::discard_send(&connection, id).await.map(|_| ())
                    } else {
                        daemon::send(&connection, &daemon::Command::UndoSend(id)).await
                    };
                    let opened = match &result {
                        Ok(()) => raw.and_then(security::unseal),
                        Err(_) => None,
                    };
                    (result, opened)
                })
                .await;
            this.update_in(cx, |this, window, cx| {
                match result {
                    Ok(()) => {
                        this.writing.outbox.retain(|i| i.id != id);
                        if this.writing.outbox.is_empty() {
                            this.writing.outbox_open = false;
                        }
                        this.rebuild_nav();
                        if edit {
                            this.writing.outbox_open = false;
                            match opened.and_then(|(raw, sealing)| {
                                let mut unsent = unsent_from_raw(&raw)?;
                                unsent.sealing = sealing;
                                // From the account it was going from.
                                unsent.from = Some(AccountId(account));
                                Some(unsent)
                            }) {
                                Some(unsent) => {
                                    this.unsent = Some(unsent);
                                    this.reopen_unsent(window, cx);
                                }
                                None => this.show_snackbar(
                                    tr!("schedule-cancelled-not-opened"),
                                    None,
                                    cx,
                                ),
                            }
                        } else {
                            this.show_snackbar(tr!("outbox-deleted"), None, cx);
                        }
                    }
                    Err(err) => this.show_snackbar(err, None, cx),
                }
                this.scheduled_changed(cx);
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// The outbox's list, when opened from the navigation.
    pub(in crate::window) fn render_outbox(
        &self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !self.writing.outbox_open {
            return None;
        }
        let rows =
            self.writing
                .outbox
                .iter()
                .enumerate()
                .filter_map(|(ix, item)| {
                    let why = why(item)?;
                    let subject = if item.subject.trim().is_empty() {
                        tr!("schedule-no-subject")
                    } else {
                        item.subject.clone()
                    };
                    let address = self.outbox_account(item.account);
                    let line = match &why {
                        Why::NotSent(reason) => tr!("outbox-not-sent", reason = reason.text()),
                        Why::SignIn => match self.account_problem(AccountId(item.account)) {
                            Some(super::super::problems::Problem::Password { .. }) => {
                                tr!("outbox-waiting-password", address = address.as_str())
                            }
                            _ => tr!("outbox-waiting-sign-in", address = address.as_str()),
                        },
                        Why::Offline => tr!("outbox-waiting-connection"),
                        Why::Retrying(reason) => tr!("outbox-retrying", reason = reason.text()),
                    };
                    let color = if why.needs_you() {
                        th.warning
                    } else {
                        th.text_dim
                    };
                    // Signed out: the account's own fix, as on the line over the
                    // mail list.
                    let fix = matches!(why, Why::SignIn)
                        .then(|| self.account_problem(AccountId(item.account)))
                        .flatten()
                        .filter(|p| p.needs_you());
                    let item = item.clone();
                    let detail = item.detail.clone();
                    Some(
                        div()
                            .id(("outbox", ix))
                            .flex_none()
                            .px(px(space::S6))
                            .py(px(space::S3))
                            .flex()
                            .flex_row()
                            .items_start()
                            .gap(px(space::S4))
                            .border_b_1()
                            .border_color(rgba(th.divider))
                            .child(icon(
                                if why.needs_you() {
                                    "warning"
                                } else {
                                    "schedule"
                                },
                                color,
                                text::SUBTITLE,
                            ))
                            .child(
                                div()
                                    .flex_1()
                                    .min_w_0()
                                    .flex()
                                    .flex_col()
                                    .gap(px(space::S1 / 2.0))
                                    .child(
                                        div()
                                            .truncate()
                                            .text_size(px(text::BODY))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(subject),
                                    )
                                    .child(
                                        div()
                                            .id(("outbox-why", ix))
                                            .text_size(px(text::SMALL))
                                            .text_color(rgba(color))
                                            .child(line)
                                            // The server's own words, for whoever
                                            // asks its admin.
                                            .when(!detail.is_empty(), |d| {
                                                d.tooltip(crate::widgets::tip(detail.clone(), th))
                                            }),
                                    )
                                    .when(!matches!(why, Why::SignIn), |d| {
                                        d.child(
                                            div()
                                                .truncate()
                                                .text_size(px(text::SMALL))
                                                .text_color(rgba(th.text_faint))
                                                .child(address),
                                        )
                                    })
                                    .child(
                                        div()
                                            .pt(px(space::S2))
                                            .flex()
                                            .flex_row()
                                            .flex_wrap()
                                            .items_center()
                                            .gap(px(space::S1))
                                            .children(fix.map(|problem| {
                                                outlined_button(
                                                    ("outbox-fix", ix),
                                                    problem.action(),
                                                    th,
                                                )
                                                .on_click(cx.listener(
                                                    move |this, event: &ClickEvent, window, cx| {
                                                        this.fix_problem(
                                                            problem.clone(),
                                                            event.position(),
                                                            window,
                                                            cx,
                                                        )
                                                    },
                                                ))
                                            }))
                                            .when(matches!(why, Why::NotSent(_)), |d| {
                                                let id = item.id;
                                                d.child(
                                                    outlined_button(
                                                        ("outbox-retry", ix),
                                                        tr!("outbox-try-again"),
                                                        th,
                                                    )
                                                    .on_click(cx.listener(move |this, _, _, cx| {
                                                        this.retry_outbox(id, cx)
                                                    })),
                                                )
                                            })
                                            .child({
                                                let item = item.clone();
                                                button(("outbox-edit", ix), ButtonStyle::Text, th)
                                                    .child(tr!("outbox-edit"))
                                                    .on_click(cx.listener(
                                                        move |this, _, window, cx| {
                                                            this.take_out_of_outbox(
                                                                item.clone(),
                                                                true,
                                                                window,
                                                                cx,
                                                            )
                                                        },
                                                    ))
                                            })
                                            .child({
                                                let item = item.clone();
                                                button(("outbox-delete", ix), ButtonStyle::Text, th)
                                                    .child(tr!("outbox-delete"))
                                                    .on_click(cx.listener(
                                                        move |this, _, window, cx| {
                                                            this.take_out_of_outbox(
                                                                item.clone(),
                                                                false,
                                                                window,
                                                                cx,
                                                            )
                                                        },
                                                    ))
                                            }),
                                    ),
                            ),
                    )
                });
        let rows: Vec<AnyElement> = rows.map(IntoElement::into_any_element).collect();
        Some(list_dialog(
            "outbox",
            tr!("folder-outbox"),
            rows,
            None,
            |this: &mut MailWindow| this.writing.outbox_open = false,
            th,
            window,
            cx,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(state: &str, detail: &str) -> OutboxItem {
        OutboxItem {
            id: 1,
            account: 1,
            message: 1,
            subject: "Plan".to_owned(),
            send_at: 0,
            state: state.to_owned(),
            detail: detail.to_owned(),
        }
    }

    #[test]
    fn says_why_mail_has_not_gone() {
        use katna_dbus::send_state::*;
        assert_eq!(why(&item(QUEUED, "")), None);
        assert_eq!(why(&item(SENT, "")), None);
        assert_eq!(
            why(&item(QUEUED, "sign-in: authentication failed: 535")),
            Some(Why::SignIn)
        );
        assert_eq!(
            why(&item(QUEUED, "offline: connection refused")),
            Some(Why::Offline)
        );
        assert_eq!(
            why(&item(
                FAILED,
                "server refused the command: rejected: 550 5.1.1 user unknown"
            )),
            Some(Why::NotSent(Reason::Address))
        );
        assert!(why(&item(FAILED, "")).unwrap().needs_you());
        assert!(!why(&item(QUEUED, "offline: x")).unwrap().needs_you());
    }

    #[test]
    fn reads_refusals_in_plain_words() {
        assert_eq!(
            Reason::of("the message has no recipients"),
            Reason::NoRecipients
        );
        assert_eq!(
            Reason::of("552 5.3.4 Message size exceeds"),
            Reason::TooLarge
        );
        assert_eq!(
            Reason::of("554 5.7.1 Message rejected as spam"),
            Reason::Blocked
        );
        assert_eq!(Reason::of("451 try later"), Reason::Refused);
    }
}
