// SPDX-License-Identifier: GPL-3.0-or-later

//! Follow up if no reply: the popover beside Send and its chip. When
//! nobody answers a message in time, the daemon brings the conversation
//! back to the Inbox, or sends the follow-up written here for the user,
//! in working hours, once or twice (`docs/ARCHITECTURE.md` §10.1). It
//! stops as soon as anyone replies; auto-replies don't count.

use gpui::{
    Anchor, AnyElement, Context, Entity, Focusable, FontWeight, Window, deferred, div, point,
    prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::TextArea;
use katna_ui::anchored;
use katna_ui::px;
use katna_ui::tokens::{elevation, radius, space, text};

use super::super::MailWindow;
use super::super::date_pick::slide_back;
use super::recipients::Field;
use super::schedule;
use super::tools::{Popup, menu_divider};
use crate::outgoing::{self, Mailbox, Outgoing};
use crate::theme::Theme;
use crate::widgets::{
    ButtonStyle, button, choice_chip, field, filled_button, icon, menu, menu_item, radio, raised,
    switch, text_button, tip,
};

const DAY: u32 = 24 * 60 * 60;
/// The times to choose from, after sending.
const AFTER: [u32; 3] = [DAY, 3 * DAY, 7 * DAY];
/// When the second follow-up may go, after the first.
const AGAIN: [u32; 3] = [3 * DAY, 7 * DAY, 14 * DAY];
/// The popover's width.
const WIDTH: f32 = 440.0;
/// The room the chip beside Send takes in the row, and with only its
/// icon.
pub(super) const CHIP_WIDTH: f32 = 184.0;
pub(super) const CHIP_ICON_WIDTH: f32 = 48.0;

/// What the message's follow-up is to be.
pub(super) struct FollowUp {
    /// Seconds after sending; 0 for none.
    pub after: u32,
    /// The date and time picked, rather than one of [`AFTER`].
    pub picked: Option<jiff::Timestamp>,
    /// Katna sends the follow-up, rather than bring the conversation back.
    pub send: bool,
    /// The follow-up's text.
    pub text: Entity<TextArea>,
    /// Seconds after the first follow-up to send a second; 0 for none.
    pub again: u32,
    /// The menu of [`AGAIN`] is open.
    pub again_menu: bool,
    /// The choices as the popover opened, which Cancel puts back.
    before: Option<(u32, Option<jiff::Timestamp>, bool, u32, String)>,
}

impl FollowUp {
    pub fn new(accent: gpui::Hsla, cx: &mut Context<MailWindow>) -> Self {
        let text = cx.new(|cx| {
            let mut area = TextArea::new(tr!("follow-up-text-placeholder"), cx);
            area.set_accent(accent);
            area
        });
        FollowUp {
            after: 0,
            picked: None,
            send: false,
            text,
            again: 0,
            again_menu: false,
            before: None,
        }
    }

    /// Whether there is a follow-up or reminder.
    pub fn on(&self) -> bool {
        self.after > 0 || self.picked.is_some()
    }

    /// Seconds after `sent` (Unix seconds) the follow-up is due.
    pub fn after_sending(&self, sent: i64) -> i64 {
        match self.picked {
            Some(at) => (at.as_second() - sent).max(60),
            None => i64::from(self.after),
        }
    }
}

/// `menu` under its parent, at the left or right edge (`corner`), over
/// the popover.
fn below(corner: Anchor, menu: impl IntoElement) -> AnyElement {
    let edge = div().absolute().bottom_0();
    let edge = if corner == Anchor::TopRight {
        edge.right_0()
    } else {
        edge.left_0()
    };
    deferred(
        edge.child(
            anchored()
                .anchor(corner)
                .offset(point(px(0.0), px(space::S2)))
                .snap_to_window_with_margin(px(space::S3))
                .child(div().occlude().child(menu)),
        ),
    )
    .with_priority(3)
    .into_any_element()
}

/// "3 days", "1 week": how long `seconds` is, in whole days or weeks.
fn span(seconds: u32) -> String {
    let days = seconds.div_ceil(DAY).max(1);
    if days.is_multiple_of(7) {
        tr!("follow-up-weeks", weeks = days / 7)
    } else {
        tr!("follow-up-days", days = days)
    }
}

impl MailWindow {
    /// Opens the popover, with a text to start from.
    pub(super) fn open_follow_up(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        let f = &mut c.follow_up;
        f.again_menu = false;
        f.before = Some((
            f.after,
            f.picked,
            f.send,
            f.again,
            f.text.read(cx).text().to_owned(),
        ));
        if f.text.read(cx).text().trim().is_empty() {
            let name = c
                .chips
                .get(Field::To)
                .iter()
                .find_map(|chip| chip.mailbox())
                .and_then(|m| m.name)
                .and_then(|n| n.split_whitespace().next().map(str::to_owned));
            let start = match name {
                Some(name) => tr!("follow-up-text-named", name = name),
                None => tr!("follow-up-text"),
            };
            let end = start.len();
            f.text.update(cx, |t, cx| t.set_text(start, end, cx));
        }
        c.dialog.pick = None;
        c.dialog.back = false;
        c.popup = Some(Popup::FollowUp);
        if c.follow_up.send {
            window.focus(&c.follow_up.text.focus_handle(cx), cx);
        }
        self.load_templates(cx);
        cx.notify();
    }

    /// Cancel: the choices as they were when it opened.
    pub(super) fn cancel_follow_up(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(c) = &mut self.compose
            && let Some((after, picked, send, again, text)) = c.follow_up.before.take()
        {
            let f = &mut c.follow_up;
            (f.after, f.picked, f.send, f.again) = (after, picked, send, again);
            let end = text.len();
            f.text.update(cx, |t, cx| t.set_text(text, end, cx));
        }
        self.close_popup(window, cx);
    }

    /// The date and time picker chose when to follow up.
    pub(super) fn follow_up_picked(&mut self, at: jiff::Timestamp, cx: &mut Context<Self>) {
        if at <= jiff::Timestamp::now() {
            self.show_snackbar(tr!("compose-past-time"), None, cx);
            return;
        }
        if let Some(c) = &mut self.compose {
            c.follow_up.picked = Some(at);
            c.follow_up.after = 0;
            c.dialog.for_follow_up = false;
            c.dialog.pick = None;
            c.dialog.back = true;
            c.popup = Some(Popup::FollowUp);
        }
        cx.notify();
    }

    /// Puts template `id`'s text in the follow-up.
    fn follow_up_template(&mut self, id: i64, cx: &mut Context<Self>) {
        let paths = self.paths.clone();
        let me = self.compose_sender();
        let recipient = self.compose.as_ref().and_then(|c| {
            c.chips
                .get(Field::To)
                .iter()
                .find_map(|chip| chip.mailbox())
        });
        if let Some(c) = &mut self.compose {
            c.follow_up.again_menu = false;
        }
        cx.spawn(async move |this, cx| {
            let found = cx
                .background_executor()
                .spawn(async move {
                    let template = crate::data::template(&paths, id).ok().flatten()?;
                    let mut text = template.text;
                    for (field, value) in crate::templates::fields(recipient.as_ref(), &me) {
                        text = text.replace(field, &value);
                    }
                    Some(text.trim().to_owned())
                })
                .await;
            this.update(cx, |this, cx| match found {
                Some(text) => {
                    if let Some(c) = &mut this.compose {
                        let end = text.len();
                        c.follow_up
                            .text
                            .update(cx, |t, cx| t.set_text(text, end, cx));
                        c.popup = Some(Popup::FollowUp);
                    }
                    cx.notify();
                }
                None => this.show_snackbar(tr!("compose-template-open-failed"), None, cx),
            })
            .ok();
        })
        .detach();
    }

    /// The chip beside Send while a follow-up is set: what and when; it
    /// opens the popover. A `compact` one, where the row is narrow, shows
    /// only its icon, with the words in its tooltip.
    pub(super) fn render_follow_up_chip(
        &self,
        compact: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(c) = &self.compose else {
            return div().into_any_element();
        };
        let f = &c.follow_up;
        let label = match (f.picked, f.send) {
            (Some(at), true) => tr!(
                "follow-up-chip-send-on",
                date = schedule::short(&at.to_zoned(self.tz.clone()))
            ),
            (Some(at), false) => tr!(
                "follow-up-chip-remind-on",
                date = schedule::short(&at.to_zoned(self.tz.clone()))
            ),
            (None, true) => tr!("follow-up-chip-send", time = span(f.after)),
            (None, false) => tr!("follow-up-chip-remind", time = span(f.after)),
        };
        let (fill, hover) = crate::widgets::tonal_fill(th);
        div()
            .id("compose-follow-up-chip")
            .flex_none()
            .max_w(px(CHIP_WIDTH - space::S3))
            .min_w_0()
            .h(px(28.0))
            .ml(px(space::S3))
            .px(px(space::S3))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(space::S2))
            .rounded(px(radius::SM))
            .bg(rgba(fill))
            .hover(move |s| s.bg(rgba(hover)))
            .cursor_pointer()
            .text_size(px(text::SMALL))
            .text_color(rgba(th.accent))
            .child(icon("history", th.accent, 16.0))
            .when(compact, |d| d.tooltip(tip(label.clone(), th)))
            .when(!compact, |d| {
                d.child(div().min_w_0().truncate().child(label))
            })
            .on_click(cx.listener(|this, _, window, cx| {
                let open = this
                    .compose
                    .as_ref()
                    .is_some_and(|c| c.popup == Some(Popup::FollowUp));
                if open {
                    this.close_popup(window, cx);
                } else {
                    this.open_follow_up(window, cx);
                }
            }))
            .into_any_element()
    }

    /// The popover: when, remind or send, the text, a second time.
    pub(super) fn render_follow_up(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(c) = &self.compose else {
            return div().into_any_element();
        };
        if let Some(picker) = self.render_time_picker(th, cx) {
            return picker;
        }
        let back = c.dialog.back;
        let f = &c.follow_up;
        // An encrypted message's follow-up would quote it in the clear.
        let can_send = !c.sealing.encrypt;
        let send = f.send && can_send;
        let has_signature = c.signature.is_some();
        let chips = AFTER
            .into_iter()
            .map(|after| (Some(after), span(after)))
            .chain([(None, String::new())]);
        let chips = std::iter::once((Some(0), tr!("follow-up-off")))
            .chain(chips)
            .enumerate()
            .map(|(ix, (after, label))| match after {
                Some(after) => {
                    let on = f.picked.is_none() && f.after == after;
                    choice_chip(("follow-up-after", ix), label, on, th)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if let Some(c) = &mut this.compose {
                                c.follow_up.after = after;
                                c.follow_up.picked = None;
                            }
                            cx.notify();
                        }))
                        .into_any_element()
                }
                None => {
                    let label = f.picked.map_or_else(
                        || tr!("follow-up-pick"),
                        |at| schedule::short(&at.to_zoned(self.tz.clone())),
                    );
                    choice_chip(("follow-up-after", ix), label, f.picked.is_some(), th)
                        .when(f.picked.is_none(), |d| {
                            d.child(icon("calendar", th.text_dim, 16.0))
                        })
                        .on_click(cx.listener(|this, _, window, cx| {
                            if let Some(c) = &mut this.compose {
                                c.dialog.for_follow_up = true;
                            }
                            this.open_time_picker(window, cx);
                        }))
                        .into_any_element()
                }
            });
        let choice = |id: &'static str, on: bool, enabled: bool, title: String, note: String| {
            div()
                .id(id)
                .py(px(space::S3))
                .flex()
                .flex_row()
                .items_start()
                .gap(px(space::S4))
                .when(!enabled, |d| d.opacity(katna_ui::tokens::state::DISABLED))
                .when(enabled, |d| d.cursor_pointer())
                .child(
                    div()
                        .pt(px(space::S1))
                        .child(radio(f32::from(u8::from(on)), th)),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .child(div().text_size(px(text::BODY)).child(title))
                        .child(
                            div()
                                .text_size(px(text::CAPTION))
                                .text_color(rgba(th.text_dim))
                                .child(note),
                        ),
                )
        };
        let pick_mode = |send: bool| {
            cx.listener(move |this: &mut Self, _: &gpui::ClickEvent, window, cx| {
                if let Some(c) = &mut this.compose {
                    c.follow_up.send = send;
                    if c.follow_up.after == 0 && c.follow_up.picked.is_none() {
                        c.follow_up.after = 3 * DAY;
                    }
                    if send {
                        window.focus(&c.follow_up.text.focus_handle(cx), cx);
                    }
                }
                cx.notify();
            })
        };
        let text_focus = f.text.focus_handle(cx);
        let again_on = f.again > 0;
        let again = div()
            .relative()
            .child(
                button("follow-up-again-time", ButtonStyle::Outlined, th)
                    .h(px(32.0))
                    .px(px(space::S4))
                    .gap(px(space::S2))
                    .text_size(px(text::SMALL))
                    .text_color(rgba(th.text))
                    .when(!again_on, |d| d.opacity(katna_ui::tokens::state::DISABLED))
                    .child(span(if again_on { f.again } else { 7 * DAY }))
                    .child(icon("drop-down", th.text_dim, 18.0))
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(c) = &mut this.compose {
                            c.follow_up.again_menu = !c.follow_up.again_menu;
                        }
                        cx.notify();
                    })),
            )
            .when(f.again_menu, |d| {
                d.child(below(
                    Anchor::TopRight,
                    menu(th)
                        .min_w(px(140.0))
                        .children(AGAIN.into_iter().enumerate().map(|(ix, again)| {
                            menu_item(("follow-up-again-item", ix), &span(again), th).on_click(
                                cx.listener(move |this, _, _, cx| {
                                    if let Some(c) = &mut this.compose {
                                        c.follow_up.again = again;
                                        c.follow_up.again_menu = false;
                                    }
                                    cx.notify();
                                }),
                            )
                        })),
                ))
            });
        let templates = self.writing_templates();
        // Under the radio's label.
        let write = div()
            .pl(px(space::S7))
            .flex()
            .flex_col()
            .gap(px(space::S3))
            .child(
                field("follow-up-text", &text_focus, th)
                    .min_h(px(88.0))
                    .py(px(space::S3))
                    .child(f.text.clone()),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .when(!templates.is_empty(), |d| {
                        d.child(
                            div()
                                .relative()
                                .child(
                                    text_button(
                                        "follow-up-template",
                                        "template",
                                        tr!("follow-up-template"),
                                        th,
                                    )
                                    .h(px(28.0))
                                    .text_size(px(text::SMALL))
                                    .on_click(cx.listener(
                                        |this, _, _, cx| {
                                            if let Some(c) = &mut this.compose {
                                                c.follow_up.again_menu = false;
                                                c.dialog.follow_up_templates =
                                                    !c.dialog.follow_up_templates;
                                            }
                                            cx.notify();
                                        },
                                    )),
                                )
                                .when(c.dialog.follow_up_templates, |d| {
                                    d.child(below(
                                        Anchor::TopLeft,
                                        menu(th).children(templates.iter().enumerate().map(
                                            |(ix, t)| {
                                                let id = t.id;
                                                menu_item(
                                                    ("follow-up-template-item", ix),
                                                    &t.name,
                                                    th,
                                                )
                                                .on_click(cx.listener(move |this, _, _, cx| {
                                                    if let Some(c) = &mut this.compose {
                                                        c.dialog.follow_up_templates = false;
                                                    }
                                                    this.follow_up_template(id, cx)
                                                }))
                                            },
                                        )),
                                    ))
                                }),
                        )
                    })
                    .child(div().flex_1())
                    .when(has_signature, |d| {
                        d.child(
                            div()
                                .text_size(px(text::CAPTION))
                                .text_color(rgba(th.text_dim))
                                .child(tr!("follow-up-signature")),
                        )
                    }),
            )
            .child(
                div()
                    .id("follow-up-again")
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(space::S4))
                    .child(
                        div()
                            .id("follow-up-again-switch")
                            .cursor_pointer()
                            .child(switch(f32::from(u8::from(again_on)), th))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if let Some(c) = &mut this.compose {
                                    c.follow_up.again = if again_on { 0 } else { 7 * DAY };
                                }
                                cx.notify();
                            })),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_size(px(text::BODY))
                            .child(tr!("follow-up-again")),
                    )
                    .child(again),
            );
        let actions = div()
            .flex()
            .flex_row()
            .justify_end()
            .gap(px(space::S3))
            .child(
                button("follow-up-cancel", ButtonStyle::Text, th)
                    .child(tr!("follow-up-cancel"))
                    .on_click(cx.listener(|this, _, window, cx| this.cancel_follow_up(window, cx))),
            )
            .child(
                filled_button("follow-up-done", tr!("follow-up-done"), th).on_click(cx.listener(
                    |this, _, window, cx| {
                        if let Some(c) = &mut this.compose {
                            c.follow_up.before = None;
                        }
                        this.close_popup(window, cx)
                    },
                )),
            );
        // A phone's window keeps a margin on each side.
        let width = WIDTH.min(self.room_width() - 2.0 * space::S3);
        raised(
            div()
                .id("follow-up-popover")
                .w(px(width))
                .flex()
                .flex_col()
                .text_color(rgba(th.text)),
            th,
            radius::LG,
            elevation::POPOVER,
        )
        .when(back, |d| d.overflow_hidden())
        .child(slide_back(
            "follow-up-back",
            back,
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .px(px(space::S5))
                        .pt(px(space::S5))
                        .pb(px(space::S4))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(space::S4))
                        .child(icon("history", th.text, 20.0))
                        .child(
                            div()
                                .text_size(px(text::SUBTITLE))
                                .font_weight(FontWeight::MEDIUM)
                                .child(tr!("follow-up-title")),
                        ),
                )
                .child(
                    div()
                        .px(px(space::S5))
                        .pb(px(space::S4))
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap(px(space::S3))
                        .children(chips),
                )
                .child(menu_divider(th).my_0())
                .child(
                    div()
                        .px(px(space::S5))
                        .py(px(space::S3))
                        .flex()
                        .flex_col()
                        .child(
                            choice(
                                "follow-up-remind",
                                !send,
                                true,
                                tr!("follow-up-remind"),
                                tr!("follow-up-remind-note"),
                            )
                            .on_click(pick_mode(false)),
                        )
                        .child(
                            choice(
                                "follow-up-send",
                                send,
                                can_send,
                                tr!("follow-up-send"),
                                if can_send {
                                    tr!("follow-up-send-note")
                                } else {
                                    tr!("follow-up-send-encrypted")
                                },
                            )
                            .when(can_send, |d| d.on_click(pick_mode(true))),
                        )
                        .when(send, |d| d.child(write)),
                )
                .child(
                    div()
                        .px(px(space::S5))
                        .pt(px(space::S3))
                        .flex()
                        .flex_row()
                        .items_start()
                        .gap(px(space::S3))
                        .text_size(px(text::CAPTION))
                        .text_color(rgba(th.text_dim))
                        .child(icon("info", th.text_dim, 16.0))
                        .child(div().flex_1().min_w_0().child(if send {
                            tr!(
                                "follow-up-note-send",
                                start = schedule::clock(jiff::civil::Time::constant(9, 0, 0, 0)),
                                end = schedule::clock(jiff::civil::Time::constant(17, 0, 0, 0))
                            )
                        } else {
                            tr!("follow-up-note")
                        })),
                )
                .child(div().p(px(space::S5)).child(actions)),
        ))
        .into_any_element()
    }
}

/// What a follow-up Katna sends is made of.
pub(super) struct Mail<'a> {
    pub from: &'a Mailbox,
    pub to: &'a [Mailbox],
    pub cc: &'a [Mailbox],
    pub bcc: &'a [Mailbox],
    /// The message's subject; the follow-up's is its "Re:".
    pub subject: &'a str,
    /// The message's `Message-ID` and `References`.
    pub message_id: &'a str,
    pub references: &'a [String],
    /// What the user wrote, and their signature as text.
    pub text: &'a str,
    pub signature: Option<&'a str>,
    /// "On …, … wrote:" and the message's text, quoted below.
    pub quote: (&'a str, &'a str),
}

/// The follow-up, threaded under the message, for the daemon to send if
/// nobody replies: without `Date` and `Message-ID`, which it gets then.
pub(super) fn build(mail: &Mail<'_>) -> Vec<u8> {
    let mut body = mail.text.trim_end().to_owned();
    if let Some(signature) = mail.signature.map(str::trim).filter(|s| !s.is_empty()) {
        body.push_str("\n\n");
        body.push_str(signature);
    }
    let (said, quoted) = mail.quote;
    body.push_str("\n\n");
    body.push_str(said);
    body.push('\n');
    for line in quoted.trim_end().lines() {
        if line.is_empty() {
            body.push_str(">\n");
        } else {
            body.push_str("> ");
            body.push_str(line);
            body.push('\n');
        }
    }
    let mut references = mail.references.to_vec();
    references.push(mail.message_id.to_owned());
    outgoing::build(&Outgoing {
        from: Some(mail.from.clone()),
        to: mail.to.to_vec(),
        cc: mail.cc.to_vec(),
        bcc: mail.bcc.to_vec(),
        subject: super::prefixed("Re:", mail.subject),
        body,
        in_reply_to: Some(mail.message_id.to_owned()),
        references,
        html: None,
        inline: Vec::new(),
        attachments: Vec::new(),
        date: None,
        message_id: None,
        calendar: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_follow_up_answers_the_message_and_quotes_it() {
        let me = Mailbox {
            name: Some("Ada".into()),
            email: "ada@example.org".into(),
        };
        let priya = Mailbox {
            name: Some("Priya Nair".into()),
            email: "priya@example.org".into(),
        };
        let raw = build(&Mail {
            from: &me,
            to: std::slice::from_ref(&priya),
            cc: &[],
            bcc: &[],
            subject: "Proposal",
            message_id: "m1@example.org",
            references: &["r0@example.org".to_owned()],
            text: "Hi Priya, just checking you saw my message below.\n",
            signature: Some("Ada"),
            quote: (
                "On 7 Oct 2026, Ada <ada@example.org> wrote:",
                "Here it is.\n\nAda",
            ),
        });
        let raw = String::from_utf8(raw).unwrap();
        assert!(raw.contains("Subject: Re: Proposal\r\n"));
        assert!(raw.contains("In-Reply-To: <m1@example.org>\r\n"));
        assert!(raw.contains("References: <r0@example.org> <m1@example.org>\r\n"));
        assert!(!raw.contains("Message-ID:"));
        assert!(!raw.contains("\r\nDate:"));
        assert!(raw.contains("below.\r\n\r\nAda\r\n\r\nOn 7 Oct 2026"));
        assert!(raw.contains("> Here it is.\r\n>\r\n> Ada"));
    }
}
