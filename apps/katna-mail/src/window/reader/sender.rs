// SPDX-License-Identifier: GPL-3.0-or-later

//! Who really sent a message, by the checks the user's provider ran on it
//! (its `Authentication-Results`: DMARC, DKIM and SPF; see
//! [`katna_render::sender_checks`]). Mail that failed them gets a soft red
//! banner above its body, with what each check found, Looks safe and Move
//! to spam; its images stay hidden and its links ask before they open. A
//! sender nothing confirmed gets a "?" on their picture
//! (`docs/ARCHITECTURE.md` §12).

use std::collections::HashSet;
use std::time::Instant;

use gpui::{
    AnyElement, Bounds, Context, FontWeight, MouseDownEvent, Pixels, Point, Size, Window, deferred,
    div, point, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_render::{Check, Outcome, SenderChecks, Verdict};
use katna_store::MessageId;
use katna_ui::px;
use katna_ui::tokens::{radius, space, text};
use katna_ui::unpx;

use super::super::Act;
use super::super::mail_providers::MailProvider;
use super::super::notched;
use super::Part;
use crate::theme::{Theme, fade};
use crate::widgets::{BUTTON_HEIGHT, ButtonStyle, button, filled_button, icon, tip};
use crate::window::MailWindow;

/// What the user said about failed mail while the window is open.
#[derive(Default)]
pub(in crate::window) struct SenderState {
    /// Failed mail the user said looks safe: no banner, and its images
    /// and links act as other mail's.
    safe: HashSet<MessageId>,
    /// The question before a link of failed mail opens.
    ask: Option<LinkAsk>,
}

/// A link of failed mail, clicked at `at`, waiting for an answer.
pub(in crate::window) struct LinkAsk {
    url: String,
    at: Point<Pixels>,
    fading: Option<Instant>,
}

const ASK_WIDTH: f32 = 360.0;

impl MailWindow {
    /// The sender of `part` failed the provider's checks, and the user
    /// did not say it looks safe.
    pub(super) fn sender_failed(&self, part: &Part) -> bool {
        part.body.as_ref().is_some_and(super::Body::failed)
            && !self.sender_checks.safe.contains(&part.id)
    }

    /// Failed mail `id` was said to look safe ([`SenderState::safe`]).
    pub(in crate::window) fn looks_safe(&self, id: MessageId) -> bool {
        self.sender_checks.safe.contains(&id)
    }

    /// The red banner above the body of mail whose sender failed the
    /// provider's checks.
    pub(super) fn sender_banner(
        &self,
        ix: usize,
        part: &Part,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !self.sender_failed(part) {
            return None;
        }
        let checks = part.body.as_ref()?.checks.as_ref()?;
        let red = th.error;
        let id = part.id;
        let open = part.checks_open;
        let action = |name: &'static str, label: String| {
            button((name, ix), ButtonStyle::Text, th)
                .h(px(32.0))
                .px(px(space::S3))
                .text_size(px(text::SMALL))
                .child(label)
        };
        let actions = div()
            .mt(px(space::S2))
            .ml(px(-space::S3))
            .flex()
            .flex_row()
            .flex_wrap()
            .child(
                action(
                    "sender-details",
                    if open {
                        tr!("sender-details-hide")
                    } else {
                        tr!("sender-details")
                    },
                )
                .on_click(cx.listener(move |this, _, _, cx| {
                    if let Some(part) = this.reader.as_mut().and_then(|r| r.parts.get_mut(ix)) {
                        part.checks_open = !part.checks_open;
                    }
                    cx.notify();
                })),
            )
            .child(
                action("sender-safe", tr!("sender-looks-safe")).on_click(cx.listener(
                    move |this, _, _, cx| {
                        this.sender_checks.safe.insert(id);
                        this.fetch_remote(cx);
                        cx.notify();
                    },
                )),
            )
            .child(
                action("sender-spam", tr!("sender-move-to-spam")).on_click(cx.listener(
                    |this, _, _, cx| {
                        this.act_on_targets(Act::Spam, cx);
                    },
                )),
            );
        let provider = provider_name(checks);
        Some(
            div()
                .mt(px(space::S4))
                .px(px(space::S4))
                .py(px(space::S3))
                .flex()
                .flex_row()
                .items_start()
                .gap(px(space::S3))
                .rounded(px(radius::SM))
                .bg(rgba(fade(red, if th.dark { 0.12 } else { 0.08 })))
                .text_size(px(text::SMALL))
                .line_height(px(text::line_height(text::SMALL)))
                .child(
                    div()
                        .pt(px(space::S1))
                        .child(icon("shield-alert", red, 20.0)),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .child(
                            self.copyable(
                                tr!("sender-failed-title", domain = checks.domain.as_str()),
                                th,
                            )
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgba(th.text)),
                        )
                        .child(
                            self.copyable(
                                tr!("sender-failed-body", provider = provider.as_str()),
                                th,
                            )
                            .text_color(rgba(th.text_dim)),
                        )
                        .child(actions)
                        .when(open, |d| d.child(self.check_rows(checks, &provider, th))),
                )
                .into_any_element(),
        )
    }

    /// What each check found, one row each, under the banner.
    fn check_rows(&self, checks: &SenderChecks, provider: &str, th: &Theme) -> AnyElement {
        let checked_by = if checks.server.is_empty() {
            tr!("sender-checked-by", provider = provider)
        } else {
            tr!(
                "sender-checked-by-server",
                provider = provider,
                server = checks.server.as_str()
            )
        };
        let rows = [
            (tr!("sender-dmarc"), Method::Dmarc, checks.dmarc.as_ref()),
            (tr!("sender-dkim"), Method::Dkim, checks.dkim.as_ref()),
            (tr!("sender-spf"), Method::Spf, checks.spf.as_ref()),
        ];
        div()
            .mt(px(space::S2))
            .pt(px(space::S3))
            .border_t_1()
            .border_color(rgba(th.divider))
            .flex()
            .flex_col()
            .gap(px(space::S3))
            .child(
                self.copyable(checked_by, th)
                    .text_size(px(text::CAPTION))
                    .text_color(rgba(th.text_faint)),
            )
            .children(rows.into_iter().map(|(name, method, check)| {
                let outcome = check.map(|c| c.outcome);
                let (word, color) = result_word(outcome, th);
                let why = explain(method, check, &checks.domain);
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(space::S4))
                            .child(
                                self.copyable(name, th)
                                    .flex_1()
                                    .min_w_0()
                                    .text_color(rgba(th.text)),
                            )
                            .child(
                                self.copyable(word, th)
                                    .flex_none()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgba(color)),
                            ),
                    )
                    .children(why.map(|why| {
                        self.copyable(why, th)
                            .text_size(px(text::CAPTION))
                            .text_color(rgba(th.text_faint))
                    }))
            }))
            .into_any_element()
    }

    /// The "?" on the picture of a sender the provider checked and nothing
    /// confirmed, which says so under the pointer.
    pub(super) fn unconfirmed_badge(
        &self,
        ix: usize,
        part: &Part,
        th: &Theme,
    ) -> Option<AnyElement> {
        let checks = part.body.as_ref()?.checks.as_ref()?;
        if checks.verdict != Verdict::Unconfirmed {
            return None;
        }
        let provider = provider_name(checks);
        let words = tr!(
            "sender-unconfirmed",
            provider = provider.as_str(),
            domain = checks.domain.as_str()
        );
        Some(
            div()
                .id(("sender-unconfirmed", ix))
                .absolute()
                .right(px(-space::S1))
                .bottom(px(-space::S1))
                .size(px(18.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .bg(rgba(th.surface))
                .tooltip(tip(words, th))
                .child(icon("help-circle", th.text_faint, 16.0))
                .into_any_element(),
        )
    }

    /// Asks before `url`, a link of failed mail clicked at `at`, opens.
    pub(in crate::window) fn ask_link(
        &mut self,
        url: String,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.sender_checks.ask = Some(LinkAsk {
            url,
            at,
            fading: None,
        });
        cx.notify();
    }

    /// Closes the question without opening the link. Returns whether it
    /// was open.
    pub(in crate::window) fn close_link_ask(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(ask) = self
            .sender_checks
            .ask
            .as_mut()
            .filter(|a| a.fading.is_none())
        else {
            return false;
        };
        match notched::fade_out(cx) {
            Some(since) => ask.fading = Some(since),
            None => self.sender_checks.ask = None,
        }
        cx.notify();
        true
    }

    fn open_asked_link(&mut self, cx: &mut Context<Self>) {
        if let Some(ask) = self.sender_checks.ask.as_ref() {
            cx.open_url(&ask.url);
        }
        self.close_link_ask(cx);
    }

    /// The question before a link of failed mail opens, where it was
    /// clicked: where the link really goes, Cancel and Open.
    pub(in crate::window) fn render_link_ask(
        &mut self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if self
            .sender_checks
            .ask
            .as_ref()
            .and_then(|a| a.fading)
            .is_some_and(|since| notched::faded(since, cx))
        {
            self.sender_checks.ask = None;
        }
        let ask = self.sender_checks.ask.as_ref()?;
        let th = &th.lifted();
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        let width = ASK_WIDTH.min(vw - 2.0 * notched::MARGIN);
        let host = link_host(&ask.url);
        // Title, two lines of words, the address (up to three lines) and
        // the buttons.
        let height = space::S5 * 2.0
            + text::line_height(text::SUBTITLE)
            + space::S2
            + 2.0 * text::line_height(text::SMALL)
            + space::S3
            + 3.0 * text::line_height(text::CAPTION)
            + space::S4
            + BUTTON_HEIGHT;
        let chip = Bounds::new(ask.at, Size::new(px(1.0), px(1.0)));
        let (x, y, side, along) = notched::place(chip, (width, height), (vw, vh), notched::RADIUS);
        let at = ask.at;
        let panel = div()
            .id("link-ask")
            .absolute()
            .left(px(x))
            .top(px(y))
            .w(px(width))
            .p(px(space::S5))
            .flex()
            .flex_col()
            .map(|d| notched::popover(d, th))
            .text_color(rgba(th.text))
            .on_mouse_down_out(cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                // The click that opened it is not one outside it.
                if event.position != at {
                    this.close_link_ask(cx);
                }
            }))
            .child(
                div()
                    .text_size(px(text::SUBTITLE))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(tr!("sender-link-title")),
            )
            .child(
                self.copyable(tr!("sender-link-body", host = host.as_str()), th)
                    .mt(px(space::S2))
                    .text_size(px(text::SMALL))
                    .text_color(rgba(th.text_dim)),
            )
            .child(
                self.copyable(ask.url.clone(), th)
                    .mt(px(space::S3))
                    .max_h(px(3.0 * text::line_height(text::CAPTION)))
                    .overflow_hidden()
                    .text_size(px(text::CAPTION))
                    .text_color(rgba(th.text_faint)),
            )
            .child(
                div()
                    .mt(px(space::S4))
                    .flex()
                    .flex_row()
                    .justify_end()
                    .gap(px(space::S3))
                    .child(
                        button("link-ask-cancel", ButtonStyle::Text, th)
                            .child(tr!("sender-link-cancel"))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.close_link_ask(cx);
                            })),
                    )
                    .child(
                        filled_button("link-ask-open", tr!("sender-link-open"), th)
                            .on_click(cx.listener(|this, _, _, cx| this.open_asked_link(cx))),
                    ),
            )
            .children(notched::notch(side, along, th));
        let panel = match ask.fading {
            Some(_) => notched::fading(panel, "link-ask-out"),
            None => panel.into_any_element(),
        };
        let layer = div().relative().w(px(vw)).h(px(vh)).child(panel);
        Some(
            deferred(
                katna_ui::anchored()
                    .position(point(px(0.0), px(0.0)))
                    .child(layer),
            )
            .with_priority(5)
            .into_any_element(),
        )
    }
}

#[derive(Clone, Copy)]
enum Method {
    Dmarc,
    Dkim,
    Spf,
}

/// The provider's mail service by name ("Gmail"), else "your mail
/// provider".
fn provider_name(checks: &SenderChecks) -> String {
    match MailProvider::for_server(&checks.server) {
        MailProvider::Other => tr!("sender-provider-unknown"),
        known => known.mail_name(),
    }
}

/// A check's result in a word, and its colour.
fn result_word(outcome: Option<Outcome>, th: &Theme) -> (String, u32) {
    match outcome {
        Some(Outcome::Pass) => (
            tr!("sender-result-pass"),
            if th.dark { 0x81c995ff } else { 0x188038ff },
        ),
        Some(Outcome::Fail) => (tr!("sender-result-fail"), th.error),
        Some(Outcome::Unsure) => (tr!("sender-result-unsure"), th.text_dim),
        Some(Outcome::None) => (tr!("sender-result-none"), th.text_faint),
        None => (tr!("sender-result-missing"), th.text_faint),
    }
}

/// What a check's result means, in a sentence; `None` when it did not run.
fn explain(method: Method, check: Option<&Check>, from: &str) -> Option<String> {
    let check = check?;
    let domain = check
        .domain
        .as_deref()
        .filter(|d| d.contains('.'))
        .unwrap_or(from);
    Some(match (method, check.outcome) {
        (Method::Dmarc, Outcome::Pass) => tr!("sender-dmarc-pass", domain = domain),
        (Method::Dmarc, Outcome::Fail) => tr!("sender-dmarc-fail", domain = domain),
        (Method::Dmarc, Outcome::None) => tr!("sender-dmarc-none", domain = domain),
        (Method::Dkim, Outcome::Pass) => tr!("sender-dkim-pass", domain = domain),
        (Method::Dkim, Outcome::Fail) => tr!("sender-dkim-fail", domain = domain),
        (Method::Dkim, Outcome::None) => tr!("sender-dkim-none"),
        (Method::Spf, Outcome::Pass) => tr!("sender-spf-pass", domain = domain),
        (Method::Spf, Outcome::Fail) => tr!("sender-spf-fail", domain = domain),
        (Method::Spf, Outcome::None) => tr!("sender-spf-none", domain = domain),
        (_, Outcome::Unsure) => tr!("sender-check-unsure"),
    })
}

/// The host a link goes to ("login.example.com"), else the link itself.
fn link_host(url: &str) -> String {
    url::Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(str::to_owned))
        .unwrap_or_else(|| url.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hosts_and_providers() {
        assert_eq!(
            link_host("https://login.bank.example/verify?a=1"),
            "login.bank.example"
        );
        assert_eq!(link_host("mailto:a@b.example"), "mailto:a@b.example");
        let checks = |server: &str| SenderChecks {
            server: server.to_owned(),
            domain: "bank.example".to_owned(),
            verdict: Verdict::Failed,
            dmarc: None,
            dkim: None,
            spf: None,
        };
        assert_eq!(provider_name(&checks("mx.google.com")), "Gmail");
        assert_eq!(provider_name(&checks("mx.zohomail.com")), "Zoho Mail");
        assert_eq!(
            provider_name(&checks("notgoogle.com")),
            "your mail provider"
        );
        assert_eq!(provider_name(&checks("")), "your mail provider");
    }
}
