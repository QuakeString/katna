// SPDX-License-Identifier: GPL-3.0-or-later

//! Waiting for reply: the mail the user sent that a follow-up or a "remind
//! me if no reply" waits on (`katna-meta`, kept by the daemon). It lists
//! under Sent in the folder pane while there is some; each line, there
//! and in any other list, has a chip with what happens next, and the open
//! conversation a card with Edit (the time), Send now and Stop.

use gpui::{AnyElement, ClickEvent, Context, FontWeight, div, prelude::*, rgba};
use jiff::Timestamp;
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::tokens::{radius, space, text};

use super::compose::schedule;
use super::{Listing, MailWindow};
use crate::daemon::Command;
use crate::data::{LineFollowUp, Row};
use crate::theme::{Theme, mix};
use crate::widgets::{icon, line_chip, outlined_button, tip, tonal_fill};

/// The folder pane line of Waiting for reply.
pub(super) const NAV_KEY: &str = "katna:waiting";

impl MailWindow {
    /// Opens Waiting for reply, as `open_folder` opens a folder.
    pub(super) fn open_waiting(&mut self, cx: &mut Context<Self>) {
        let conversations = self.config.mail.conversations;
        let Ok(mail) = &mut self.mail else {
            return;
        };
        // Mail the user sent: its lines show who it went to.
        if !self.show_recipients {
            self.show_recipients = true;
            mail.clear_rows();
        }
        self.entries = mail.waiting_entries(conversations);
        self.tabs = Vec::new();
        self.tab = 0;
        self.folder = None;
        self.unified = None;
        self.listing = Some(Listing::Waiting);
        self.reset_list(false);
        self.selected = (!self.entries.is_empty()).then_some(0);
        self.checked.clear();
        self.check_anchor = None;
        self.checked_all = false;
        self.page_pick = None;
        self.picked = None;
        self.menu = None;
        self.show_list();
        cx.notify();
    }

    /// How many follow-ups wait, for the folder pane.
    pub(super) fn waiting_count(&self) -> usize {
        self.mail.as_ref().map_or(0, |mail| mail.waiting_count())
    }

    /// What the chip of `follow_up` says, and its colour: the warning
    /// colour while it waits for the user.
    fn follow_up_chip(&self, follow_up: &LineFollowUp, th: &Theme) -> (&'static str, String, u32) {
        let at = Timestamp::from_second(follow_up.at)
            .map(|at| schedule::short(&at.to_zoned(self.tz.clone())))
            .unwrap_or_default();
        if follow_up.waiting {
            return ("warning", tr!("row-follow-up-waiting"), th.warning);
        }
        if !follow_up.sends {
            return (
                "bell",
                tr!("follow-up-chip-remind-on", date = at),
                th.accent,
            );
        }
        // The second of two says so; the first reads as any follow-up.
        let label = if follow_up.step > 1 {
            tr!(
                "row-follow-up-step",
                step = follow_up.step,
                steps = follow_up.steps,
                date = at
            )
        } else {
            tr!("follow-up-chip-send-on", date = at)
        };
        ("send", label, th.accent)
    }

    /// The chip on a line a follow-up waits on. Clicking it opens the line,
    /// as clicking the line does.
    pub(super) fn render_row_follow_up(
        &self,
        ix: usize,
        row: &Row,
        th: &Theme,
    ) -> Option<AnyElement> {
        let follow_up = row.follow_up?;
        let (name, label, color) = self.follow_up_chip(&follow_up, th);
        Some(
            line_chip(
                ("row-follow-up", ix),
                ("row-follow-up-glow", ix),
                name,
                label,
                color,
                th,
            )
            .tooltip(tip(self.follow_up_says(&follow_up), th))
            .into_any_element(),
        )
    }

    /// What will happen, in a sentence: the chip's tooltip and the card's
    /// text.
    fn follow_up_says(&self, follow_up: &LineFollowUp) -> String {
        let at = Timestamp::from_second(follow_up.at)
            .map(|at| schedule::describe(at, &self.tz))
            .unwrap_or_default();
        match (follow_up.waiting, follow_up.sends) {
            (true, _) => tr!("follow-up-card-waiting"),
            (false, false) => tr!("follow-up-card-remind", date = at),
            (false, true) if follow_up.step < follow_up.steps => {
                tr!("follow-up-card-send-twice", date = at)
            }
            (false, true) => tr!("follow-up-card-send", date = at),
        }
    }

    /// The card over the open conversation while a follow-up waits on it:
    /// what happens next, with Edit (the time), Send now and Stop.
    pub(super) fn render_follow_up_card(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let reader = self.reader.as_ref()?;
        let ids: Vec<_> = reader.message_ids().into_iter().collect();
        let follow_up = self.mail.as_ref().ok()?.follow_up_in(&ids)?;
        let outbox = follow_up.outbox;
        let (fill, color, name) = if follow_up.waiting {
            (
                mix(th.surface, th.warning | 0xff, 0.14),
                th.warning,
                "warning",
            )
        } else if follow_up.sends {
            (tonal_fill(th).0, th.accent, "send")
        } else {
            (tonal_fill(th).0, th.accent, "bell")
        };
        let title = if follow_up.waiting {
            tr!("follow-up-card-title-waiting")
        } else {
            tr!("follow-up-card-title")
        };
        let edit = outlined_button("follow-up-edit", tr!("follow-up-card-edit"), th).on_click(
            cx.listener(move |this, event: &ClickEvent, _, cx| {
                this.open_follow_up_times(outbox, event.position(), cx);
            }),
        );
        let send_now = follow_up.sends.then(|| {
            outlined_button("follow-up-send-now", tr!("follow-up-card-send-now"), th).on_click(
                cx.listener(move |this, _, _, cx| {
                    this.send(
                        Command::SendFollowUpNow(outbox),
                        Some(tr!("toast-follow-up-sent")),
                        None,
                        false,
                        cx,
                    );
                }),
            )
        });
        let stop = crate::widgets::button("follow-up-stop", crate::widgets::ButtonStyle::Text, th)
            .child(tr!("follow-up-card-stop"))
            .on_click(cx.listener(move |this, _, _, cx| this.stop_follow_up(outbox, cx)));
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
                .bg(rgba(fill))
                .child(
                    div()
                        .flex_none()
                        .pt(px(space::S1))
                        .child(icon(name, color, 20.0)),
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
                                .child(title),
                        )
                        .child(
                            div()
                                .text_size(px(text::SMALL))
                                .text_color(rgba(th.text_dim))
                                .child(self.follow_up_says(&follow_up)),
                        )
                        .child(
                            div()
                                .pt(px(space::S3))
                                .flex()
                                .flex_row()
                                .flex_wrap()
                                .items_center()
                                .gap(px(space::S3))
                                .child(edit)
                                .children(send_now)
                                .child(stop),
                        ),
                )
                .into_any_element(),
        )
    }

    /// Stops the follow-up of outbox entry `outbox`, with an Undo while
    /// it can be set again as it was (the first one, not waiting).
    fn stop_follow_up(&mut self, outbox: i64, cx: &mut Context<Self>) {
        let undo = self
            .mail
            .as_ref()
            .ok()
            .and_then(|mail| mail.follow_up_value(outbox))
            .filter(|f| f.sent.is_empty() && !f.waiting)
            .map(|f| Command::SetFollowUp(outbox, f.after, f.again, f.mail));
        self.send(
            Command::StopFollowUp(outbox),
            Some(tr!("toast-follow-up-stopped")),
            undo,
            false,
            cx,
        );
    }

    /// Moves the follow-up of outbox entry `outbox` to `at`, with an Undo
    /// back to its time unless it was waiting (that time has passed).
    pub(super) fn move_follow_up(&mut self, outbox: i64, at: i64, cx: &mut Context<Self>) {
        let undo = self
            .mail
            .as_ref()
            .ok()
            .and_then(|mail| mail.follow_up_value(outbox))
            .filter(|f| !f.waiting)
            .map(|f| Command::MoveFollowUp(outbox, f.remind_at));
        let when = Timestamp::from_second(at)
            .map(|at| schedule::describe(at, &self.tz))
            .unwrap_or_default();
        self.send(
            Command::MoveFollowUp(outbox, at),
            Some(tr!("toast-follow-up-moved", date = when)),
            undo,
            false,
            cx,
        );
    }
}
