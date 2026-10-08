// SPDX-License-Identifier: GPL-3.0-or-later

//! Help > Send feedback (`docs/ARCHITECTURE.md` §19.2, plan C.7): what it
//! is about, the message, an optional address for a reply (never filled
//! in from the accounts) and whether Katna's version and the system go
//! along. "What is sent" shows the exact text before it goes; the daemon
//! posts that text and nothing else. Sending is itself the consent for
//! that one message, whatever the switches in Settings > User feedback
//! say. It opens from the global menu, Quick settings > Help and Settings
//! > User feedback.

use gpui::{
    AnyElement, Context, Entity, FocusHandle, Focusable, FontWeight, KeyDownEvent, MouseButton,
    Subscription, Window, div, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::tokens::{radius, space, text};
use katna_ui::{InputEvent, TextArea, TextInput, px};

use super::{MailWindow, SendFeedback};
use crate::daemon;
use crate::theme::{Theme, fade};
use crate::widgets::{
    ButtonStyle, Check, FocusRing, button, checkbox, choice_chip, field, filled_button,
    icon_button, line_field,
};

const WIDTH: f32 = 520.0;

/// What the feedback is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Kind {
    Problem,
    Idea,
    Other,
}

impl Kind {
    const ALL: [Kind; 3] = [Kind::Problem, Kind::Idea, Kind::Other];

    /// The tag sent along.
    fn key(self) -> &'static str {
        match self {
            Kind::Problem => "problem",
            Kind::Idea => "idea",
            Kind::Other => "other",
        }
    }

    /// The word in the text sent, which stays English for whoever reads it.
    fn sent_name(self) -> &'static str {
        match self {
            Kind::Problem => "Problem",
            Kind::Idea => "Idea",
            Kind::Other => "Something else",
        }
    }

    fn label(self) -> String {
        match self {
            Kind::Problem => tr!("send-feedback-problem"),
            Kind::Idea => tr!("send-feedback-idea"),
            Kind::Other => tr!("send-feedback-other"),
        }
    }
}

/// "Katna Mail 0.0.0-r612" and "Arch Linux, KDE on wayland".
fn version_and_system() -> (String, String) {
    (
        format!("Katna Mail {}", katna_core::crash::VERSION),
        katna_core::crash::system(),
    )
}

/// The exact text sent. Its field names stay English, as in crash
/// reports: they are read by whoever fixes Katna.
pub(super) fn feedback_text(
    kind: Kind,
    message: &str,
    reply_to: &str,
    system: Option<(String, String)>,
) -> String {
    let mut out = format!("Kind: {}\nMessage: {}\n", kind.sent_name(), message.trim());
    let reply_to = reply_to.trim();
    out.push_str(&format!(
        "Reply to: {}\n",
        if reply_to.is_empty() {
            "(none)"
        } else {
            reply_to
        }
    ));
    if let Some((version, system)) = system {
        out.push_str(&format!("Version: {version}\nSystem: {system}\n"));
    }
    out
}

pub(super) struct FeedbackForm {
    focus: FocusHandle,
    closing: bool,
    shown: Spring,
    kind: Kind,
    message: Entity<TextArea>,
    reply_to: Entity<TextInput>,
    /// Katna's version and the system go along.
    system: bool,
    /// "What is sent" is open.
    preview: bool,
    sending: bool,
    _subscriptions: Vec<Subscription>,
}

impl MailWindow {
    pub(super) fn send_feedback_action(
        &mut self,
        _: &SendFeedback,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_feedback_form(window, cx);
    }

    pub(super) fn open_feedback_form(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.settings_open = false;
        if self.feedback_form.as_ref().is_some_and(|f| !f.closing) {
            return;
        }
        let accent = rgba(self.theme(window).accent).into();
        let message = cx.new(|cx| {
            let mut area = TextArea::new(tr!("send-feedback-message-placeholder"), cx);
            area.set_accent(accent);
            area
        });
        let reply_to = cx.new(|cx| {
            let mut input = TextInput::new(tr!("send-feedback-reply-placeholder"), cx);
            input.set_accent(accent);
            input
        });
        let changed = |this: &mut Self, event: &InputEvent, cx: &mut Context<Self>| match event {
            InputEvent::Cancel => this.close_feedback_form(cx),
            InputEvent::Changed | InputEvent::Submit => cx.notify(),
        };
        let subscriptions = vec![
            cx.subscribe(&message, move |this, _, event: &InputEvent, cx| {
                changed(this, event, cx)
            }),
            cx.subscribe(&reply_to, move |this, _, event: &InputEvent, cx| {
                changed(this, event, cx)
            }),
        ];
        window.focus(&message.focus_handle(cx), cx);
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.feedback_form = Some(FeedbackForm {
            focus: cx.focus_handle(),
            closing: false,
            shown,
            kind: Kind::Problem,
            message,
            reply_to,
            system: true,
            preview: false,
            sending: false,
            _subscriptions: subscriptions,
        });
        cx.notify();
    }

    /// The form is open and not on its way out.
    pub(super) fn feedback_form_open(&self) -> bool {
        self.feedback_form.as_ref().is_some_and(|f| !f.closing)
    }

    pub(super) fn close_feedback_form(&mut self, cx: &mut Context<Self>) {
        if let Some(form) = &mut self.feedback_form
            && !form.closing
            && !form.sending
        {
            form.closing = true;
            form.shown.set(0.0);
        }
        cx.notify();
    }

    /// The text "What is sent" shows, and the reply address.
    fn feedback_to_send(&self, cx: &gpui::App) -> Option<(Kind, String, String)> {
        let form = self.feedback_form.as_ref()?;
        let message = form.message.read(cx).text().to_owned();
        let reply_to = form.reply_to.read(cx).text().trim().to_owned();
        let system = form.system.then(version_and_system);
        Some((
            form.kind,
            feedback_text(form.kind, &message, &reply_to, system),
            reply_to,
        ))
    }

    fn feedback_ready(&self, cx: &gpui::App) -> bool {
        self.feedback_form
            .as_ref()
            .is_some_and(|form| !form.sending && !form.message.read(cx).text().trim().is_empty())
    }

    fn send_feedback_now(&mut self, cx: &mut Context<Self>) {
        if !self.feedback_ready(cx) {
            return;
        }
        let Some((kind, text, reply_to)) = self.feedback_to_send(cx) else {
            return;
        };
        if let Some(form) = &mut self.feedback_form {
            form.sending = true;
        }
        cx.notify();
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::send_feedback(&connection, &text, kind.key(), &reply_to).await
                })
                .await;
            this.update(cx, |this, cx| {
                if let Some(form) = &mut this.feedback_form {
                    form.sending = false;
                }
                match result {
                    Ok(()) => {
                        this.close_feedback_form(cx);
                        this.show_snackbar(tr!("send-feedback-sent"), None, cx);
                    }
                    Err(err) => {
                        this.show_snackbar(tr!("send-feedback-failed", error = err), None, cx)
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn feedback_form_key(&mut self, event: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        // Escape reaches `popovers` first; Ctrl+Enter sends.
        let stroke = &event.keystroke;
        if stroke.key == "enter" && (stroke.modifiers.control || stroke.modifiers.platform) {
            self.send_feedback_now(cx);
            cx.stop_propagation();
        }
    }

    pub(super) fn render_feedback_form(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let form = self.feedback_form.as_mut()?;
        let t = form.shown.tick(window, reduce);
        if form.closing && form.shown.settled() {
            self.feedback_form = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let ready = self.feedback_ready(cx);
        let (_, preview_text, _) = self.feedback_to_send(cx)?;
        let form = self.feedback_form.as_ref()?;
        let phone = self.layout.shape.is_phone();
        let vw = self.room_width();
        let width = if phone { vw } else { WIDTH.min(vw - 48.0) };
        let (version, system) = version_and_system();

        let label = |words: String| {
            div()
                .pt(px(space::S4))
                .pb(px(space::S2))
                .text_size(px(text::CAPTION))
                .text_color(rgba(th.text_faint))
                .child(words)
        };
        let mut chips = div().flex().flex_row().flex_wrap().gap(px(space::S3));
        for kind in Kind::ALL {
            chips = chips.child(
                choice_chip(
                    ("send-feedback-kind", kind as usize),
                    kind.label(),
                    form.kind == kind,
                    th,
                )
                .on_click(cx.listener(move |this, _, _, cx| {
                    if let Some(form) = &mut this.feedback_form {
                        form.kind = kind;
                    }
                    cx.notify();
                })),
            );
        }
        let message_focus = form.message.focus_handle(cx);
        let message = field("send-feedback-message", &message_focus, th)
            .min_h(px(if phone { 140.0 } else { 104.0 }))
            .py(px(space::S3))
            .text_size(px(text::BODY))
            .child(form.message.clone());
        let reply_to = line_field("send-feedback-reply", &form.reply_to, th, cx)
            .text_size(px(text::BODY))
            .child(div().flex_1().child(form.reply_to.clone()));
        let system_on = form.system;
        let system_row = div()
            .id("send-feedback-system")
            .mt(px(space::S4))
            .flex()
            .flex_row()
            .items_start()
            .gap(px(space::S4))
            .cursor_pointer()
            .on_click(cx.listener(|this, _, _, cx| {
                if let Some(form) = &mut this.feedback_form {
                    form.system = !form.system;
                }
                cx.notify();
            }))
            .child(checkbox(
                "send-feedback-system-box",
                Check::from(system_on),
                th,
            ))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(text::BODY))
                            .child(tr!("send-feedback-system")),
                    )
                    .child(
                        div()
                            .text_size(px(text::CAPTION))
                            .text_color(rgba(th.text_faint))
                            .child(format!("{version}, {system}")),
                    ),
            );
        let preview_open = form.preview;
        let preview = div()
            .mt(px(space::S4))
            .flex()
            .flex_col()
            .rounded(px(radius::SM))
            .border_1()
            .border_color(rgba(th.outline))
            .child(
                div()
                    .id("send-feedback-preview")
                    .px(px(space::S4))
                    .py(px(space::S3))
                    .flex()
                    .flex_row()
                    .items_center()
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(form) = &mut this.feedback_form {
                            form.preview = !form.preview;
                        }
                        cx.notify();
                    }))
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(text::SMALL))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(tr!("send-feedback-what-is-sent")),
                    )
                    .child(
                        div()
                            .text_size(px(text::SMALL))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.accent))
                            .child(if preview_open {
                                tr!("send-feedback-hide")
                            } else {
                                tr!("send-feedback-show")
                            }),
                    ),
            )
            .when(preview_open, |d| {
                d.child(
                    div()
                        .px(px(space::S4))
                        .py(px(space::S3))
                        .border_t_1()
                        .border_color(rgba(th.outline))
                        .text_size(px(text::CAPTION))
                        .child(
                            div()
                                .font_family("monospace")
                                .text_color(rgba(th.text_dim))
                                .children(
                                    preview_text
                                        .lines()
                                        .map(|line| self.copyable(line.to_owned(), th)),
                                ),
                        )
                        .child(
                            div()
                                .pt(px(space::S3))
                                .text_color(rgba(th.text_faint))
                                .child(tr!("send-feedback-where")),
                        ),
                )
            });

        let sending = form.sending;
        let send_label = if sending {
            tr!("send-feedback-sending")
        } else {
            tr!("send-feedback-send")
        };
        let send = filled_button("send-feedback-send", send_label, th)
            .focus_ring_filled(th)
            .when(!ready, |d| d.opacity(0.45).cursor_default())
            .on_click(cx.listener(|this, _, _, cx| this.send_feedback_now(cx)));
        let body = div()
            .id("send-feedback-body")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .px(px(space::S6))
            .pt(px(if phone { space::S3 } else { space::S6 }))
            .pb(px(space::S3))
            .flex()
            .flex_col()
            .when(!phone, |d| {
                d.child(
                    div()
                        .text_size(px(text::TITLE))
                        .line_height(px(text::line_height(text::TITLE)))
                        .child(tr!("send-feedback-title")),
                )
            })
            .child(label(tr!("send-feedback-about")))
            .child(chips)
            .child(label(tr!("send-feedback-message")))
            .child(message)
            .child(label(tr!("send-feedback-reply")))
            .child(reply_to)
            .child(system_row)
            .child(preview);
        let card = div()
            .id("send-feedback")
            .track_focus(&form.focus)
            .map(|d| super::popovers::keep_tab_inside(d, &form.focus))
            .on_key_down(cx.listener(Self::feedback_form_key))
            .occlude()
            .w(px(width))
            .when(phone, |d| d.h_full())
            .when(!phone, |d| d.max_h_full().min_h_0())
            .flex()
            .flex_col()
            .map(|d| {
                if phone {
                    d.bg(rgba(th.surface))
                } else {
                    crate::widgets::dialog(d, th, th.surface)
                }
            })
            .text_color(rgba(th.text))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .when(phone, |d| {
                // The title and Send in a bar at the top, as on a phone.
                d.child(
                    div()
                        .flex_none()
                        .h(px(space::S8 + space::S2))
                        .px(px(space::S3))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(space::S3))
                        .child(
                            icon_button("send-feedback-close", "close", 20.0, th).on_click(
                                cx.listener(|this, _, _, cx| this.close_feedback_form(cx)),
                            ),
                        )
                        .child(
                            div()
                                .flex_1()
                                .text_size(px(text::SUBTITLE))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(tr!("send-feedback-title")),
                        )
                        .child(send),
                )
            })
            .child(body)
            .when(!phone, |d| {
                d.child(
                    div()
                        .flex_none()
                        .px(px(space::S6))
                        .pt(px(space::S4))
                        .pb(px(space::S5))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(space::S3))
                        .child(div().flex_1())
                        .child(
                            button("send-feedback-cancel", ButtonStyle::Text, th)
                                .on_click(
                                    cx.listener(|this, _, _, cx| this.close_feedback_form(cx)),
                                )
                                .child(tr!("send-feedback-cancel")),
                        )
                        .child(send_desktop(ready, sending, th, cx)),
                )
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
                .when(!phone, |d| d.p(px(space::S6)))
                .bg(rgba(fade(0x0000_0066, t)))
                .child(
                    div()
                        .id("send-feedback-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| this.close_feedback_form(cx))),
                )
                .child(
                    div()
                        .max_h_full()
                        .when(phone, |d| d.h_full())
                        .flex()
                        .flex_col()
                        .opacity(t)
                        .mt(px(lerp(space::S6, 0.0, t)))
                        .child(card),
                )
                .into_any_element(),
        )
    }
}

/// Send at the foot of the dialog (the phone's is in its top bar).
fn send_desktop(
    ready: bool,
    sending: bool,
    th: &Theme,
    cx: &mut Context<MailWindow>,
) -> impl IntoElement {
    let label = if sending {
        tr!("send-feedback-sending")
    } else {
        tr!("send-feedback-send")
    };
    filled_button("send-feedback-send-foot", label, th)
        .focus_ring_filled(th)
        .when(!ready, |d| d.opacity(0.45).cursor_default())
        .on_click(cx.listener(|this, _, _, cx| this.send_feedback_now(cx)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_text_is_what_the_form_says() {
        let text = feedback_text(
            Kind::Idea,
            "  Pin a label  ",
            "",
            Some(("Katna Mail 1".into(), "Arch Linux".into())),
        );
        assert_eq!(
            text,
            "Kind: Idea\nMessage: Pin a label\nReply to: (none)\nVersion: Katna Mail 1\nSystem: Arch Linux\n"
        );
        let text = feedback_text(Kind::Problem, "x", " me@example.org ", None);
        assert_eq!(
            text,
            "Kind: Problem\nMessage: x\nReply to: me@example.org\n"
        );
    }
}
