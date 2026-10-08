// SPDX-License-Identifier: GPL-3.0-or-later

//! Waiting for reply: the mail the user sent that a follow-up or a "remind
//! me if no reply" waits on (`katna-meta`, kept by the daemon). It lists
//! under Sent in the folder pane while there is some; each line, there
//! and in any other list, has a chip with what happens next, and the open
//! conversation a card with Edit (the time), Send now and Stop.

use crate::widgets::Tip as _;
use gpui::{AnyElement, ClickEvent, Context, FontWeight, div, prelude::*, relative, rgba};
use jiff::Timestamp;
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::tokens::{radius, space, text};

use super::compose::schedule;
use super::{Listing, MailWindow};
use crate::daemon::Command;
use crate::data::{LineFollowUp, Row};
use crate::theme::{Theme, fade, mix};
use crate::widgets::{ButtonStyle, button, icon, icon_tag, line_chip, outlined_button, tonal_fill};

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
            .tip(self.follow_up_says(&follow_up), th)
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
                                .children(self.follow_up_buttons(&follow_up, th, cx)),
                        ),
                )
                .into_any_element(),
        )
    }

    /// Edit (the time), Send now (for one Katna sends) and Stop: on the
    /// card over the open mail and under the follow-up in a chat.
    fn follow_up_buttons(
        &self,
        follow_up: &LineFollowUp,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let outbox = follow_up.outbox;
        let edit = outlined_button("follow-up-edit", tr!("follow-up-card-edit"), th).on_click(
            cx.listener(move |this, event: &ClickEvent, _, cx| {
                this.open_follow_up_times(outbox, event.position(), cx);
            }),
        );
        let send_now = follow_up.sends.then(|| {
            outlined_button("follow-up-send-now", tr!("follow-up-card-send-now"), th)
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.send(
                        Command::SendFollowUpNow(outbox),
                        Some(tr!("toast-follow-up-sent")),
                        None,
                        false,
                        cx,
                    );
                }))
                .into_any_element()
        });
        let stop = button("follow-up-stop", ButtonStyle::Text, th)
            .child(tr!("follow-up-card-stop"))
            .on_click(cx.listener(move |this, _, _, cx| this.stop_follow_up(outbox, cx)));
        let mut buttons = vec![edit.into_any_element()];
        buttons.extend(send_now);
        buttons.push(stop.into_any_element());
        buttons
    }

    /// The follow-up waiting on the open conversation, in a chat: the one
    /// Katna sends as a faint dashed bubble where it will go, with its
    /// text and Edit, Send now and Stop under it; a reminder as a small
    /// line, like the day labels.
    pub(super) fn render_chat_follow_up(
        &self,
        phone: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let reader = self.reader.as_ref()?;
        let ids: Vec<_> = reader.message_ids().into_iter().collect();
        let mail = self.mail.as_ref().ok()?;
        let follow_up = mail.follow_up_in(&ids)?;
        let when = Timestamp::from_second(follow_up.at)
            .map(|at| schedule::short(&at.to_zoned(self.tz.clone())))
            .unwrap_or_default();
        if !follow_up.sends {
            // Edit and Stop sit in the line itself, as small links.
            let outbox = follow_up.outbox;
            let link = |id: &'static str, label: String| {
                div()
                    .id(id)
                    .px(px(space::S1))
                    .rounded(px(radius::XS))
                    .text_color(rgba(th.accent))
                    .font_weight(FontWeight::MEDIUM)
                    .flex_none()
                    .cursor_pointer()
                    .hover(|s| s.underline())
                    .child(label)
            };
            return Some(
                icon_tag(
                    "bell",
                    div()
                        .min_w_0()
                        .truncate()
                        .child(tr!("follow-up-chat-remind", date = when)),
                    th,
                )
                // A narrow chat shortens the text; Edit and Stop stay.
                .flex_shrink(1.0)
                .min_w_0()
                .self_center()
                .max_w(relative(0.9))
                .my(px(space::S3))
                .child(
                    link("follow-up-edit", tr!("follow-up-card-edit")).on_click(cx.listener(
                        move |this, event: &ClickEvent, _, cx| {
                            this.open_follow_up_times(outbox, event.position(), cx);
                        },
                    )),
                )
                .child(
                    link("follow-up-stop", tr!("follow-up-card-stop")).on_click(
                        cx.listener(move |this, _, _, cx| this.stop_follow_up(outbox, cx)),
                    ),
                )
                .into_any_element(),
            );
        }
        let (name, color, head) = if follow_up.waiting {
            ("warning", th.warning, tr!("follow-up-chat-waiting"))
        } else if follow_up.step > 1 {
            (
                "history",
                th.accent,
                tr!(
                    "follow-up-chat-step",
                    step = follow_up.step,
                    steps = follow_up.steps,
                    date = when
                ),
            )
        } else {
            (
                "history",
                th.accent,
                tr!("follow-up-chat-send", date = when),
            )
        };
        let said = mail
            .follow_up_value(follow_up.outbox)
            .and_then(|f| f.mail)
            .map(|raw| follow_up_text(&raw))
            .unwrap_or_default();
        let edge = if follow_up.waiting {
            fade(th.warning, 0.55)
        } else {
            fade(th.text, 0.22)
        };
        let bubble = div()
            .self_end()
            .max_w(relative(if phone { 0.82 } else { 0.72 }))
            .px(px(space::S4))
            .py(px(space::S3))
            .flex()
            .flex_col()
            .gap(px(space::S2))
            .border_1()
            .border_dashed()
            .border_color(rgba(edge))
            .rounded(px(radius::LG))
            .rounded_br(px(radius::XS))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(space::S2))
                    .items_start()
                    .text_size(px(text::CAPTION))
                    .text_color(rgba(color))
                    .child(div().pt(px(space::S1)).child(icon(name, color, 14.0)))
                    .child(div().min_w_0().child(head)),
            )
            .when(!said.is_empty(), |d| {
                d.child(
                    div()
                        .text_size(px(text::BODY))
                        .text_color(rgba(th.text_dim))
                        .line_clamp(4)
                        .child(said),
                )
            });
        Some(
            div()
                .w_full()
                .mt(px(space::S3))
                .flex()
                .flex_col()
                .gap(px(space::S3))
                .child(bubble)
                .child(
                    div()
                        .self_end()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .justify_end()
                        .gap(px(space::S3))
                        .children(self.follow_up_buttons(&follow_up, th, cx)),
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

/// What the user wrote in a follow-up (RFC 5322, as the app built it):
/// its text and signature, without the quoted message under them.
fn follow_up_text(raw: &str) -> String {
    let Some(message) = mail_parser::MessageParser::default().parse(raw.as_bytes()) else {
        return String::new();
    };
    let body = message.body_text(0).unwrap_or_default();
    let lines: Vec<&str> = body.lines().collect();
    let quote = lines
        .iter()
        .position(|line| line.starts_with('>'))
        .unwrap_or(lines.len());
    // The "On …, … wrote:" line over the quote goes with it.
    let said = lines[..quote]
        .iter()
        .rposition(|line| !line.trim().is_empty())
        .filter(|_| quote < lines.len())
        .unwrap_or(quote);
    lines[..said].join("\n").trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_follow_up_shows_what_was_written_without_the_quote() {
        let raw = "From: a@x\r\nTo: b@x\r\nSubject: Re: Offer\r\n\r\n\
            Hi Priya, just checking.\r\n\r\nAlice\r\n\r\n\
            On Mon, 5 Oct, Alice wrote:\r\n> The offer\r\n>\r\n> Thanks\r\n";
        assert_eq!(follow_up_text(raw), "Hi Priya, just checking.\n\nAlice");
        assert_eq!(
            follow_up_text("To: b@x\r\n\r\nNo quote here\r\n"),
            "No quote here"
        );
    }
}
