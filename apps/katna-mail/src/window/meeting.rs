// SPDX-License-Identifier: GPL-3.0-or-later

//! Video calls around mail (`docs/ARCHITECTURE.md` §18.2): Start a video
//! call from a conversation or from the message being written, with the
//! account's own Google Meet, else a Jitsi Meet room; and a Join button
//! under a mail for each call link it carries (Meet, Teams, Zoom, Webex,
//! Jitsi, WhatsApp, Telegram). The call itself opens in the browser or
//! the service's own app.

use std::future::Future;

use gpui::{AnyElement, Context, Window, div, prelude::*, rgba};
use katna_core::AccountId;
use katna_core::meeting::{self, Service};
use katna_i18n::tr;
use katna_ui::px;

use super::MailWindow;
use crate::data::EntryKey;
use crate::theme::Theme;
use crate::widgets::{icon, outlined_button};

/// The name people know `service` by.
pub(super) fn service_name(service: Service) -> String {
    match service {
        Service::GoogleMeet => tr!("meeting-service-google-meet"),
        Service::Teams => tr!("meeting-service-teams"),
        Service::Zoom => tr!("meeting-service-zoom"),
        Service::Webex => tr!("meeting-service-webex"),
        Service::Jitsi => tr!("meeting-service-jitsi"),
        Service::WhatsApp => tr!("meeting-service-whatsapp"),
        Service::Telegram => tr!("meeting-service-telegram"),
    }
}

/// A new Jitsi Meet room on `server`.
fn jitsi_room(server: &str) -> String {
    let mut random = [0u8; 16];
    if let Err(err) = getrandom::fill(&mut random) {
        // Unguessable names need the system's randomness; the clock is a
        // poor stand-in, but a call still starts.
        tracing::warn!(%err, "no system randomness for a Jitsi room name");
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        random = now.to_le_bytes();
    }
    meeting::jitsi_link(server, &random)
}

impl MailWindow {
    /// A new call link for `account`: its mail service's own (Google Meet)
    /// when the daemon can make one, else a Jitsi Meet room on the server
    /// set in Settings.
    pub(super) fn new_call_link(
        &self,
        account: Option<AccountId>,
    ) -> impl Future<Output = String> + 'static {
        let connection = self.daemon.clone();
        let server = meeting::jitsi_server(&self.config.meetings.jitsi_server);
        async move {
            if let (Some(connection), Some(account)) = (connection, account) {
                match crate::daemon::meeting_link(&connection, account.0).await {
                    Ok(link) if !link.is_empty() => return link,
                    Ok(_) => {}
                    Err(err) => tracing::info!(%err, "no meeting from the mail service"),
                }
            }
            jitsi_room(&server)
        }
    }

    /// Start a video call from the conversation on line `key`: opens the
    /// call, and a new mail to everyone in the conversation with its link,
    /// from the conversation's account.
    pub(super) fn start_call_from(
        &mut self,
        key: Option<EntryKey>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((key, mail)) = key.zip(self.mail.as_ref().ok()) else {
            return;
        };
        let Some((subject, people)) = mail.meeting_source(key) else {
            return;
        };
        let account = mail
            .entry_messages(key)
            .last()
            .and_then(|id| mail.message_account(*id));
        let link = self.new_call_link(account);
        self.show_snackbar(tr!("meeting-starting"), None, cx);
        cx.spawn_in(window, async move |this, cx| {
            let link = link.await;
            this.update_in(cx, |this, window, cx| {
                cx.open_url(&link);
                let to = people
                    .into_iter()
                    .map(|(name, email)| {
                        if name.is_empty() {
                            email
                        } else {
                            format!("{} <{email}>", name.replace(['<', '>', ','], ""))
                        }
                    })
                    .collect();
                let subject = if subject.trim().is_empty() {
                    tr!("meeting-mail-subject-plain")
                } else {
                    tr!("meeting-mail-subject", subject = subject)
                };
                this.open_mailto(
                    crate::mailto::Mailto {
                        to,
                        subject,
                        body: tr!("meeting-mail-body", link = link),
                        ..Default::default()
                    },
                    window,
                    cx,
                );
                if let Some(account) = account {
                    this.send_compose_from(account);
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// A Join button for each call link of a mail.
    pub(super) fn call_links(
        &self,
        ix: usize,
        calls: &[(Service, String)],
        th: &Theme,
    ) -> Option<AnyElement> {
        if calls.is_empty() {
            return None;
        }
        Some(
            div()
                .mb(px(12.0))
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(8.0))
                .children(calls.iter().enumerate().map(|(n, (service, link))| {
                    let url = link.clone();
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(10.0))
                        .pl(px(12.0))
                        .pr(px(6.0))
                        .py(px(6.0))
                        .rounded(px(12.0))
                        .bg(rgba(th.read_row))
                        .child(icon("video", th.text_dim, 20.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .text_color(rgba(th.text))
                                .child(tr!("meeting-call-on", service = service_name(*service))),
                        )
                        .child(
                            outlined_button(
                                ("call-join", ix * meeting::MAX_LINKS + n),
                                tr!("meeting-join"),
                                th,
                            )
                            .on_click(move |_, _, cx| cx.open_url(&url)),
                        )
                })),
        )
        .map(IntoElement::into_any_element)
    }
}
