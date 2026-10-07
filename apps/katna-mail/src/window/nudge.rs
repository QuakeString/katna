// SPDX-License-Identifier: GPL-3.0-or-later

//! Nudges: a question the user sent that nobody answered in three days
//! comes back to the top of the Inbox, with "Sent 3 days ago. Follow up?"
//! on its line (as in Gmail). Follow up writes to everyone in it; Dismiss
//! puts it away for good. The open conversation shows the same as a card,
//! or in a chat as a small line like the day labels. The daemon finds them
//! (`katna-meta`); Settings > Inbox > Nudges turns them off.

use gpui::{AnyElement, Context, FontWeight, Window, div, prelude::*, relative, rgba};
use jiff::Timestamp;
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::tokens::{radius, space, text};

use super::MailWindow;
use super::compose::Kind;
use crate::daemon::Command;
use crate::data::{LineNudge, Row};
use crate::theme::{Theme, mix};
use crate::widgets::{ButtonStyle, button, icon, icon_tag, line_chip, outlined_button, tip};
use katna_store::MessageId;

const DAY: i64 = 24 * 3600;

/// Whole days since `sent`, at least one.
fn days_since(sent: i64) -> i64 {
    ((Timestamp::now().as_second() - sent) / DAY).max(1)
}

impl MailWindow {
    /// Writes the follow-up to a nudged question: a reply to everyone in
    /// it, or in a chat the reply box.
    fn follow_up_nudge(&mut self, nudge: LineNudge, window: &mut Window, cx: &mut Context<Self>) {
        if self.chat_shown()
            && let Some(focus) = self.chat_reply_focus(cx)
        {
            window.focus(&focus, cx);
            return;
        }
        self.open_compose(Kind::ReplyAll, Some(nudge.message), window, cx);
    }

    /// Puts a nudge away: it does not come back.
    fn dismiss_nudge(&mut self, message: MessageId, cx: &mut Context<Self>) {
        self.send(
            Command::DismissNudge(message),
            Some(tr!("toast-nudge-dismissed")),
            None,
            false,
            cx,
        );
    }

    /// "Sent 3 days ago. Follow up?" on a nudged line: clicking it writes
    /// the follow-up; its x dismisses it.
    pub(super) fn render_row_nudge(
        &self,
        ix: usize,
        row: &Row,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let nudge = row.nudge?;
        let days = days_since(nudge.sent);
        Some(
            div()
                .flex_none()
                .min_w_0()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(space::S1))
                .child(
                    line_chip(
                        ("row-nudge", ix),
                        ("row-nudge-glow", ix),
                        "reply",
                        tr!("nudge-row", days = days),
                        th.warning,
                        th,
                    )
                    .tooltip(tip(tr!("nudge-row-tip"), th))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        cx.stop_propagation();
                        this.follow_up_nudge(nudge, window, cx);
                    })),
                )
                .child(
                    div()
                        .id(("row-nudge-dismiss", ix))
                        .flex_none()
                        .size(px(22.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(th.hover)))
                        .tooltip(tip(tr!("nudge-dismiss"), th))
                        .child(icon("close", th.text_faint, 14.0))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            cx.stop_propagation();
                            this.dismiss_nudge(nudge.message, cx);
                        })),
                )
                .into_any_element(),
        )
    }

    /// The nudge on the open conversation, if it has one.
    fn open_nudge(&self) -> Option<LineNudge> {
        let reader = self.reader.as_ref()?;
        let ids: Vec<MessageId> = reader.message_ids().into_iter().collect();
        self.mail.as_ref().ok()?.nudge_in(&ids)
    }

    /// The card over an open nudged conversation: nobody answered the
    /// question, with Follow up and Dismiss.
    pub(super) fn render_nudge_card(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let nudge = self.open_nudge()?;
        let days = days_since(nudge.sent);
        let follow_up = outlined_button("nudge-follow-up", tr!("nudge-follow-up"), th).on_click(
            cx.listener(move |this, _, window, cx| this.follow_up_nudge(nudge, window, cx)),
        );
        let dismiss = button("nudge-dismiss", ButtonStyle::Text, th)
            .child(tr!("nudge-dismiss"))
            .on_click(cx.listener(move |this, _, _, cx| this.dismiss_nudge(nudge.message, cx)));
        Some(
            div()
                .ml(px(self.reader_indent()))
                .mr(px(space::S5))
                .mb(px(space::S3))
                .p(px(space::S5))
                .flex()
                .flex_row()
                .gap(px(space::S4))
                .rounded(px(radius::MD))
                .bg(rgba(mix(th.surface, th.warning | 0xff, 0.14)))
                .child(
                    div()
                        .flex_none()
                        .pt(px(space::S1))
                        .child(icon("reply", th.warning, 20.0)),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .gap(px(space::S2))
                        .child(
                            div()
                                .text_size(px(text::BODY))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgba(th.text))
                                .child(tr!("nudge-card-title")),
                        )
                        .child(
                            div()
                                .text_size(px(text::SMALL))
                                .text_color(rgba(th.text_dim))
                                .child(tr!("nudge-card-text", days = days)),
                        )
                        .child(
                            div()
                                .pt(px(space::S3))
                                .flex()
                                .flex_row()
                                .flex_wrap()
                                .items_center()
                                .gap(px(space::S3))
                                .child(follow_up)
                                .child(dismiss),
                        ),
                )
                .into_any_element(),
        )
    }

    /// The nudge in a chat: a small line at its end, like the day labels,
    /// with Follow up (the reply box) and Dismiss.
    pub(super) fn render_chat_nudge(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let nudge = self.open_nudge()?;
        let days = days_since(nudge.sent);
        let link = |id: &'static str, label: String| {
            div()
                .id(id)
                .flex_none()
                .px(px(space::S1))
                .rounded(px(radius::XS))
                .text_color(rgba(th.accent))
                .font_weight(FontWeight::MEDIUM)
                .cursor_pointer()
                .hover(|s| s.underline())
                .child(label)
        };
        Some(
            icon_tag(
                "reply",
                div()
                    .min_w_0()
                    .truncate()
                    .child(tr!("nudge-chat-line", days = days)),
                th,
            )
            .flex_shrink(1.0)
            .min_w_0()
            .self_center()
            .max_w(relative(0.9))
            .my(px(space::S3))
            .child(
                link("chat-nudge-follow-up", tr!("nudge-follow-up")).on_click(
                    cx.listener(move |this, _, window, cx| this.follow_up_nudge(nudge, window, cx)),
                ),
            )
            .child(
                link("chat-nudge-dismiss", tr!("nudge-dismiss")).on_click(
                    cx.listener(move |this, _, _, cx| this.dismiss_nudge(nudge.message, cx)),
                ),
            )
            .into_any_element(),
        )
    }
}
