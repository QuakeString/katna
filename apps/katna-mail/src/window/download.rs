// SPDX-License-Identifier: GPL-3.0-or-later

//! Opening a message that is not downloaded yet (older than the offline
//! window) downloads it at once: the reading view says so, then shows it,
//! or says why it could not and offers to try again.

use gpui::{Animation, AnimationExt, AnyElement, Context, FontWeight, div, prelude::*, px, rgba};
use katna_store::MessageId;
use std::time::Duration;

use super::MailWindow;
use crate::theme::Theme;
use crate::widgets::icon;
use crate::{daemon, format};

pub(super) enum Download {
    Running,
    Failed(String),
}

impl MailWindow {
    /// Starts downloading every open message of the conversation that is
    /// not stored yet, unless it is being downloaded or failed.
    pub(super) fn download_bodies(&mut self, cx: &mut Context<Self>) {
        let Some(reader) = &self.reader else {
            return;
        };
        let missing: Vec<MessageId> = reader
            .missing_bodies()
            .into_iter()
            .filter(|id| !self.downloads.contains_key(id))
            .collect();
        for id in missing {
            self.downloads.insert(id, Download::Running);
            let connection = self.daemon.clone();
            cx.spawn(async move |this, cx| {
                let result = cx
                    .background_executor()
                    .spawn(async move {
                        let connection = match connection {
                            Some(connection) => connection,
                            None => daemon::connect().await?,
                        };
                        daemon::fetch_body(&connection, id.0).await
                    })
                    .await;
                this.update(cx, |this, cx| {
                    match result {
                        Ok(()) => {
                            this.downloads.remove(&id);
                            if let (Some(reader), Ok(mail)) = (&mut this.reader, &this.mail) {
                                reader.reload_body(id, mail);
                            }
                        }
                        Err(reason) => {
                            tracing::warn!(message = id.0, %reason, "download failed");
                            this.downloads.insert(id, Download::Failed(reason));
                        }
                    }
                    cx.notify();
                })
                .ok();
            })
            .detach();
        }
    }

    /// What the reading view shows for message `id` while it has no body.
    pub(super) fn download_note(
        &self,
        id: MessageId,
        ix: usize,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let note = div()
            .pt(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(10.0))
            .text_size(px(14.0))
            .text_color(rgba(th.text_faint));
        match self.downloads.get(&id) {
            Some(Download::Failed(reason)) => note
                .child(icon("warning", th.error, 18.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_color(rgba(th.text))
                                .child("Could not download this message."),
                        )
                        .child(div().text_size(px(12.0)).child(format::sentence(reason))),
                )
                .child(
                    div()
                        .id(("download-retry", ix))
                        .flex_none()
                        .px(px(12.0))
                        .py(px(4.0))
                        .rounded_full()
                        .text_color(rgba(th.accent))
                        .font_weight(FontWeight::MEDIUM)
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(th.hover)))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.downloads.remove(&id);
                            this.download_bodies(cx);
                            cx.notify();
                        }))
                        .child("Try again"),
                )
                .into_any_element(),
            // Starting, or started on the next frame.
            _ => note
                .child(
                    div()
                        .flex_none()
                        .child(icon("download", th.accent, 18.0))
                        .with_animation(
                            ("downloading", ix),
                            Animation::new(Duration::from_millis(900))
                                .repeat()
                                .with_easing(gpui::pulsating_between(0.35, 1.0)),
                            |icon, t| icon.opacity(t),
                        ),
                )
                .child("Downloading this message from the server\u{2026}")
                .into_any_element(),
        }
    }
}
