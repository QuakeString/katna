// SPDX-License-Identifier: GPL-3.0-or-later

//! Scheduled mail: the messages the background service holds for a later
//! time, listed under "Scheduled" in the navigation, and Cancel send, which
//! takes one back and opens it to edit, as webmail does.

use std::collections::HashMap;
use std::sync::Arc;

use futures_lite::StreamExt;
use gpui::{AnyElement, Context, FontWeight, Window, div, prelude::*, rgba};
use katna_core::AccountId;
use katna_dbus::OutboxItem;
use katna_dbus::zbus::Connection;
use katna_i18n::tr;
use katna_store::MessageId;
use katna_ui::px;
use katna_ui::rich::html;
use katna_ui::unpx;
use mail_parser::{MessageParser, MimeHeaders};

use super::super::MailWindow;
use super::security::{self, Sealing};
use super::{Draft, Threading, Unsent, addresses, schedule};
use crate::daemon::{self, Command};
use crate::theme::{Theme, fade};
use crate::widgets::{elevation, icon, icon_button, outlined_button, tip};

use super::attach::Attachment;

/// The navigation row of scheduled mail.
pub(in crate::window) const NAV_KEY: &str = "katna:scheduled";

/// A queued message counts as scheduled when it goes out later than this
/// many seconds after the undo delay would end.
const LATER: i64 = 5;

impl MailWindow {
    /// Follows the outbox, keeping the list of scheduled mail current.
    pub(in crate::window) fn watch_scheduled(
        &mut self,
        connection: Connection,
        cx: &mut Context<Self>,
    ) {
        self.writing.watch = Some(cx.spawn(async move |this, cx| {
            let Ok(mut changes) = daemon::outbox_changes(&connection).await else {
                return;
            };
            loop {
                let items = daemon::outbox(&connection).await.unwrap_or_default();
                let now = jiff::Timestamp::now().as_second();
                let alive = this
                    .update(cx, |this, cx| {
                        let later = now + i64::from(this.config.sending.undo_send_seconds) + LATER;
                        let mut scheduled: Vec<OutboxItem> = items
                            .into_iter()
                            .filter(|i| {
                                (i.state == katna_dbus::send_state::QUEUED
                                    && i.detail.is_empty()
                                    && i.send_at > later)
                                    || i.state == katna_dbus::send_state::HELD
                            })
                            .collect();
                        scheduled.sort_by_key(|i| i.send_at);
                        let before = this.writing.scheduled.len();
                        this.writing.scheduled = scheduled;
                        if before != this.writing.scheduled.len() {
                            this.rebuild_nav();
                        }
                        cx.notify();
                    })
                    .is_ok();
                if !alive || changes.next().await.is_none() {
                    break;
                }
            }
        }));
    }

    /// Reads the outbox again after scheduling a message.
    pub(in crate::window) fn scheduled_changed(&mut self, cx: &mut Context<Self>) {
        if let Some(connection) = self.daemon.clone() {
            self.watch_scheduled(connection, cx);
        }
    }

    /// The account the open message goes from.
    fn sending_account(&self) -> Option<AccountId> {
        let compose = self.compose.as_ref()?;
        compose
            .from
            .or_else(|| self.compose_account(compose.kind).map(|a| a.id))
    }

    /// Whether the mail server of the open message's account holds
    /// scheduled mail: `None` until the daemon has said.
    pub(super) fn server_holds_mail(&self) -> Option<bool> {
        let account = self.sending_account()?;
        self.writing
            .hold_limits
            .get(&account)
            .map(|&limit| limit > 0)
    }

    /// Whether the mail server of the open message's account sends
    /// delivery receipts: `None` until the daemon has said.
    pub(super) fn delivery_receipts_offered(&self) -> Option<bool> {
        let account = self.sending_account()?;
        self.writing.delivery_receipts.get(&account).copied()
    }

    /// Asks the daemon, once per account, whether its mail server sends
    /// delivery receipts, for the Delivery receipt switch.
    pub(super) fn ask_delivery_receipts(&mut self, cx: &mut Context<Self>) {
        let Some(account) = self.sending_account() else {
            return;
        };
        if self.writing.delivery_receipts.contains_key(&account) {
            return;
        }
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::server_delivery_receipts(&connection, account.0).await
                })
                .await;
            match result {
                Ok(offered) => {
                    this.update(cx, |this, cx| {
                        this.writing.delivery_receipts.insert(account, offered);
                        cx.notify();
                    })
                    .ok();
                }
                // Offline: ask again next time.
                Err(err) => tracing::debug!(%err, "asking whether the server sends receipts"),
            }
        })
        .detach();
    }

    /// Asks the daemon, once per account, whether its mail server holds
    /// scheduled mail, for the schedule menu to say who sends it.
    pub(super) fn ask_hold_limit(&mut self, cx: &mut Context<Self>) {
        let Some(account) = self.sending_account() else {
            return;
        };
        if self.writing.hold_limits.contains_key(&account) {
            return;
        }
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::server_hold_limit(&connection, account.0).await
                })
                .await;
            match result {
                Ok(limit) => {
                    this.update(cx, |this, cx| {
                        this.writing.hold_limits.insert(account, limit);
                        cx.notify();
                    })
                    .ok();
                }
                // Offline: ask again next time.
                Err(err) => tracing::debug!(%err, "asking whether the server holds mail"),
            }
        })
        .detach();
    }

    pub(in crate::window) fn open_scheduled(&mut self, cx: &mut Context<Self>) {
        self.writing.scheduled_open = true;
        cx.notify();
    }

    /// Takes a scheduled message back and opens it to edit.
    fn cancel_scheduled(&mut self, item: OutboxItem, window: &mut Window, cx: &mut Context<Self>) {
        if self
            .compose
            .as_ref()
            .is_some_and(|c| !c.closing && c.touched(cx))
        {
            self.show_snackbar(tr!("schedule-open-first"), None, cx);
            return;
        }
        // Read it before the outbox forgets it.
        let raw = self
            .mail
            .as_ref()
            .ok()
            .and_then(|mail| mail.raw(MessageId(item.message)));
        let connection = self.daemon.clone();
        let id = item.id;
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
                    let result = daemon::send(&connection, &Command::UndoSend(id)).await;
                    // Encrypted mail is decrypted again to edit it.
                    let opened = match &result {
                        Ok(()) => raw.and_then(security::unseal),
                        Err(_) => None,
                    };
                    (result, opened)
                })
                .await;
            this.update_in(cx, |this, window, cx| {
                this.writing.scheduled.retain(|i| i.id != id);
                this.rebuild_nav();
                match result {
                    Ok(()) => {
                        this.writing.scheduled_open = false;
                        this.show_snackbar(tr!("schedule-cancelled"), None, cx);
                        match opened.and_then(|(raw, sealing)| {
                            let mut unsent = unsent_from_raw(&raw)?;
                            unsent.sealing = sealing;
                            Some(unsent)
                        }) {
                            Some(unsent) => {
                                this.unsent = Some(unsent);
                                this.reopen_unsent(window, cx);
                            }
                            None => {
                                this.show_snackbar(tr!("schedule-cancelled-not-opened"), None, cx)
                            }
                        }
                    }
                    Err(err) => this.show_snackbar(err, None, cx),
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// The list of scheduled mail, when opened from the navigation.
    pub(in crate::window) fn render_scheduled(
        &self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !self.writing.scheduled_open {
            return None;
        }
        let viewport = window.viewport_size();
        let height = (unpx(viewport.height) - 160.0).clamp(200.0, 560.0);
        let rows = self.writing.scheduled.iter().enumerate().map(|(ix, item)| {
            let subject = if item.subject.trim().is_empty() {
                tr!("schedule-no-subject")
            } else {
                item.subject.clone()
            };
            let when = schedule::describe(
                jiff::Timestamp::from_second(item.send_at).unwrap_or_default(),
                &self.tz,
            );
            // The mail server has it: it can't be taken back.
            let held = item.state == katna_dbus::send_state::HELD;
            let item = item.clone();
            div()
                .id(("scheduled", ix))
                .flex_none()
                .min_h(px(64.0))
                .px(px(24.0))
                .py(px(10.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .border_b_1()
                .border_color(rgba(th.divider))
                .child(icon("schedule", th.text_dim, 20.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap(px(2.0))
                        .child(
                            div()
                                .truncate()
                                .text_size(px(14.0))
                                .font_weight(FontWeight::MEDIUM)
                                .child(subject),
                        )
                        .child(div().text_size(px(13.0)).text_color(rgba(th.accent)).child(
                            if held {
                                tr!("schedule-server-sends-at", when = when)
                            } else {
                                tr!("schedule-sends-at", when = when)
                            },
                        )),
                )
                .when(!held, |d| {
                    d.child(
                        outlined_button(("scheduled-cancel", ix), tr!("schedule-cancel-send"), th)
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.cancel_scheduled(item.clone(), window, cx)
                            })),
                    )
                })
        });
        let empty = self.writing.scheduled.is_empty().then(|| {
            div()
                .flex_1()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(12.0))
                .text_color(rgba(th.text_dim))
                .text_size(px(14.0))
                .child(icon("schedule", th.text_faint, 48.0))
                .child(tr!("schedule-nothing"))
        });
        let close = cx.listener(|this, _, _, cx| {
            this.writing.scheduled_open = false;
            cx.notify();
        });
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgba(fade(0x0000_0066, 1.0)))
                .child(
                    div()
                        .id("scheduled-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(close),
                )
                .child(
                    div()
                        .id("scheduled-dialog")
                        .occlude()
                        .w(px(560.0_f32.min(unpx(viewport.width) - 32.0)))
                        .h(px(height))
                        .flex()
                        .flex_col()
                        .rounded(px(16.0))
                        .overflow_hidden()
                        .bg(rgba(th.surface))
                        .shadow(elevation(th, 3.0))
                        .child(
                            div()
                                .flex_none()
                                .h(px(64.0))
                                .pl(px(24.0))
                                .pr(px(12.0))
                                .flex()
                                .flex_row()
                                .items_center()
                                .border_b_1()
                                .border_color(rgba(th.divider))
                                .child(
                                    div()
                                        .flex_1()
                                        .text_size(px(20.0))
                                        .child(tr!("folder-scheduled")),
                                )
                                .child(
                                    icon_button("scheduled-close", "close", 20.0, th)
                                        .tooltip(tip(tr!("schedule-close"), th))
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.writing.scheduled_open = false;
                                            cx.notify();
                                        })),
                                ),
                        )
                        .child(
                            div()
                                .id("scheduled-list")
                                .flex_1()
                                .min_h_0()
                                .flex()
                                .flex_col()
                                .overflow_y_scroll()
                                .children(rows)
                                .children(empty),
                        ),
                )
                .into_any_element(),
        )
    }
}

/// The message a scheduled `raw` message was written from, to edit again.
pub(super) fn unsent_from_raw(raw: &[u8]) -> Option<Unsent> {
    let message = MessageParser::default().parse(raw)?;
    let list = |a: Option<&mail_parser::Address<'_>>| {
        let list: Vec<katna_render::Address> = a
            .into_iter()
            .flat_map(|a| a.iter())
            .filter_map(|a| {
                Some(katna_render::Address {
                    name: a.name.as_deref().map(str::to_owned),
                    email: a.address.as_deref()?.to_owned(),
                })
            })
            .collect();
        addresses(&list)
    };
    let mut html_text = None;
    let mut text = None;
    let mut inline: HashMap<String, (String, Vec<u8>)> = HashMap::new();
    let mut attachments = Vec::new();
    for part in &message.parts {
        let mime = part
            .content_type()
            .map(|ct| {
                format!("{}/{}", ct.ctype(), ct.subtype().unwrap_or("octet-stream"))
                    .to_ascii_lowercase()
            })
            .unwrap_or_else(|| "text/plain".to_owned());
        let attached = part
            .content_disposition()
            .is_some_and(|d| d.ctype().eq_ignore_ascii_case("attachment"));
        if attached {
            attachments.push(Attachment {
                name: part.attachment_name().unwrap_or("attachment").to_owned(),
                mime,
                data: Arc::new(part.contents().to_vec()),
            });
            continue;
        }
        match &part.body {
            mail_parser::PartType::Html(body) if html_text.is_none() => {
                html_text = Some(body.to_string());
            }
            mail_parser::PartType::Text(body)
                if text.is_none() && part.attachment_name().is_none() =>
            {
                text = Some(body.to_string());
            }
            mail_parser::PartType::Binary(data) | mail_parser::PartType::InlineBinary(data) => {
                let data = data.to_vec();
                match part.content_id() {
                    Some(cid) => {
                        inline.insert(cid.trim_matches(['<', '>']).to_owned(), (mime, data));
                    }
                    None => attachments.push(Attachment {
                        name: part.attachment_name().unwrap_or("attachment").to_owned(),
                        mime,
                        data: Arc::new(data),
                    }),
                }
            }
            _ => {}
        }
    }
    let plain = html_text.is_none();
    let body = match html_text {
        Some(mut html_text) => {
            // Pictures in the text come back from their parts.
            for (cid, (mime, data)) in &inline {
                let uri = format!("data:{mime};base64,{}", html::base64_encode(data));
                html_text = html_text.replace(&format!("cid:{cid}"), &uri);
            }
            let mut next = 0;
            html::from_html(&html_text, &mut next)
        }
        None => html::from_plain(text.as_deref().unwrap_or_default()),
    };
    let references = message
        .references()
        .as_text_list()
        .map(|l| l.iter().map(|s| s.to_string()).collect())
        .unwrap_or_default();
    Some(Unsent {
        draft: Draft {
            to: list(message.to()),
            cc: list(message.cc()),
            bcc: list(message.bcc()),
            subject: message.subject().unwrap_or_default().to_owned(),
            body,
        },
        thread: Threading {
            in_reply_to: message.in_reply_to().as_text().map(str::to_owned),
            references,
        },
        sealing: Sealing::default(),
        // The signature stays in the text as it was written.
        signature: None,
        attachments,
        drive: Vec::new(),
        plain,
        from: None,
        answering: None,
        unarchive: None,
        message_id: None,
        saved: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::outgoing::{self, Mailbox, Outgoing, Part};
    use katna_ui::rich::Block;

    #[test]
    fn scheduled_mail_opens_as_written() {
        let raw = outgoing::build(&Outgoing {
            from: Some(Mailbox {
                name: None,
                email: "me@example.com".to_owned(),
            }),
            to: outgoing::parse_addresses("Kay <kay@example.com>").unwrap(),
            bcc: outgoing::parse_addresses("boss@example.com").unwrap(),
            subject: "Plan".to_owned(),
            body: "Hello *there*".to_owned(),
            html: Some(
                "<div dir=\"ltr\"><div>Hello <b>there</b></div><img src=\"cid:p1@x\"></div>"
                    .to_owned(),
            ),
            inline: vec![Part {
                name: "dot.png".to_owned(),
                mime: "image/png".to_owned(),
                data: Arc::new(vec![1, 2, 3]),
                content_id: Some("p1@x".to_owned()),
            }],
            attachments: vec![Part {
                name: "notes.txt".to_owned(),
                mime: "text/plain".to_owned(),
                data: Arc::new(b"notes".to_vec()),
                content_id: None,
            }],
            in_reply_to: Some("a@b".to_owned()),
            ..Outgoing::default()
        });
        let unsent = unsent_from_raw(&raw).unwrap();
        assert_eq!(unsent.draft.to, "Kay <kay@example.com>");
        assert_eq!(unsent.draft.bcc, "boss@example.com");
        assert_eq!(unsent.draft.subject, "Plan");
        assert!(!unsent.plain);
        assert_eq!(unsent.thread.in_reply_to.as_deref(), Some("a@b"));
        let para = match &unsent.draft.body.blocks[0] {
            Block::Para(p) => p,
            other => panic!("{other:?}"),
        };
        assert_eq!(para.text, "Hello there");
        assert!(para.runs.iter().any(|r| r.style.bold));
        assert_eq!(unsent.draft.body.images().count(), 1);
        assert_eq!(unsent.attachments.len(), 1);
        assert_eq!(unsent.attachments[0].name, "notes.txt");
    }
}
