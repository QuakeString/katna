// SPDX-License-Identifier: GPL-3.0-or-later

//! "Help improve Katna": whether crash reports are sent to Katna's crash
//! tracker (`docs/ARCHITECTURE.md` §19.2). A first start asks on its own
//! onboarding page; someone who installed Katna before this existed is
//! asked once, in the same words, by this dialog after the update. Both
//! answers weigh the same and nothing is sent before one is given.
//! Settings > User feedback changes it later.

use gpui::{
    AnyElement, Context, FocusHandle, KeyDownEvent, MouseButton, Window, div, prelude::*, px, rgba,
};
use katna_ui::motion::{self, Spring, lerp};

use super::onboarding::{feature, lead, title};
use super::settings::Change;
use super::{MailWindow, PANEL_RADIUS};
use crate::theme::{Theme, fade};
use crate::widgets::{elevation, icon, outlined_button};

const WIDTH: f32 = 520.0;

pub(super) const TITLE: &str = "Help improve Katna";
pub(super) const LEAD: &str = "When Katna crashes, it saves a report on this computer. \
     Sending these reports helps fix what went wrong. You can change this any time in \
     Settings > User feedback.";

/// What is sent, what never is, and where it goes.
pub(super) fn points(th: &Theme) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .child(feature(
            "document",
            "What is sent",
            "The crash report: what crashed and where in Katna, the version, and the name \
             of your Linux system and desktop.",
            th,
        ))
        .child(feature(
            "shield-check",
            "What is never sent",
            "Your mail, contacts, accounts, passwords, file names, IP address, user name or \
             computer name.",
            th,
        ))
        .child(feature(
            "send",
            "Where it goes",
            "Katna's crash tracker at Sentry, stored in the EU. No ID ties reports to you.",
            th,
        ))
        .into_any_element()
}

/// The two answers, side by side and looking the same.
pub(super) fn answers(
    prefix: &'static str,
    answer: impl Fn(&mut MailWindow, bool, &mut Window, &mut Context<MailWindow>) + Copy + 'static,
    th: &Theme,
    cx: &mut Context<MailWindow>,
) -> AnyElement {
    let id = |what: &str| gpui::SharedString::from(format!("{prefix}-{what}"));
    div()
        .flex()
        .flex_row()
        .flex_wrap()
        .justify_end()
        .gap(px(12.0))
        .child(
            outlined_button(id("no"), "Don\u{2019}t send", th)
                .on_click(cx.listener(move |this, _, window, cx| answer(this, false, window, cx))),
        )
        .child(
            outlined_button(id("yes"), "Send crash reports", th)
                .on_click(cx.listener(move |this, _, window, cx| answer(this, true, window, cx))),
        )
        .into_any_element()
}

pub(super) struct ShareAsk {
    focus: FocusHandle,
    closing: bool,
    shown: Spring,
}

impl MailWindow {
    /// Whether "Help improve Katna" still needs an answer.
    pub(super) fn share_unanswered(&self) -> bool {
        self.config.feedback.send_crash_reports.is_none()
    }

    pub(super) fn open_share_ask(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.share_ask_later = false;
        if self.share_ask.is_some() || !self.share_unanswered() {
            return;
        }
        let focus = cx.focus_handle();
        window.focus(&focus, cx);
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        self.share_ask = Some(ShareAsk {
            focus,
            closing: false,
            shown,
        });
        cx.notify();
    }

    /// The dialog is open and not on its way out.
    pub(super) fn share_ask_open(&self) -> bool {
        self.share_ask.as_ref().is_some_and(|d| !d.closing)
    }

    /// Closes the dialog; without an answer, the next start asks again.
    pub(super) fn close_share_ask(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(dialog) = &mut self.share_ask
            && !dialog.closing
        {
            dialog.closing = true;
            dialog.shown.set(0.0);
            window.focus(&self.list_focus, cx);
        }
        cx.notify();
    }

    fn answer_share_ask(&mut self, send: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.apply(Change::SendCrashReports(send), cx);
        self.close_share_ask(window, cx);
        self.show_snackbar(
            if send {
                "Crash reports will be sent. Thank you."
            } else {
                "Crash reports stay on this computer."
            },
            None,
            cx,
        );
    }

    fn share_ask_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        // Escape reaches `popovers` first.
        if event.keystroke.key == "escape" {
            self.close_share_ask(window, cx);
            cx.stop_propagation();
        }
    }

    pub(super) fn render_share_ask(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let dialog = self.share_ask.as_mut()?;
        let t = dialog.shown.tick(window, reduce);
        if dialog.closing && dialog.shown.settled() {
            self.share_ask = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let focus = dialog.focus.clone();
        let phone = self.layout.shape.is_phone();
        let vw = f32::from(window.viewport_size().width);
        let width = if phone { vw } else { WIDTH.min(vw - 48.0) };
        let body = div()
            .id("share-ask-body")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .px(px(24.0))
            .pt(px(24.0))
            .pb(px(8.0))
            .flex()
            .flex_col()
            .items_center()
            .gap(px(12.0))
            .child(
                div()
                    .flex_none()
                    .size(px(48.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .bg(rgba(fade(th.accent, 0.14)))
                    .child(icon("shield-check", th.accent, 26.0)),
            )
            .child(title(TITLE, th))
            .child(lead(LEAD, th))
            .child(div().pt(px(8.0)).w_full().child(points(th)));
        let footer = div()
            .flex_none()
            .px(px(24.0))
            .pt(px(16.0))
            .pb(px(20.0))
            .child(answers(
                "share-ask",
                |this, send, window, cx| this.answer_share_ask(send, window, cx),
                th,
                cx,
            ));
        let card = div()
            .id("share-ask")
            .track_focus(&focus)
            .on_key_down(cx.listener(Self::share_ask_key))
            .occlude()
            .w(px(width))
            .when(phone, |d| d.h_full())
            .when(!phone, |d| d.max_h_full().min_h_0())
            .flex()
            .flex_col()
            .overflow_hidden()
            .when(!phone, |d| {
                d.rounded(px(PANEL_RADIUS)).shadow(elevation(th, 3.0))
            })
            .bg(rgba(th.surface))
            .text_color(rgba(th.text))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(body)
            .child(footer);
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .when(!phone, |d| d.p(px(24.0)))
                .bg(rgba(fade(0x0000_0066, t)))
                .child(
                    div()
                        .id("share-ask-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(
                            cx.listener(|this, _, window, cx| this.close_share_ask(window, cx)),
                        ),
                )
                .child(
                    div()
                        .max_h_full()
                        .when(phone, |d| d.h_full())
                        .flex()
                        .flex_col()
                        .opacity(t)
                        .mt(px(lerp(24.0, 0.0, t)))
                        .child(card),
                )
                .into_any_element(),
        )
    }
}
