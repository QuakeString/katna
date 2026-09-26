// SPDX-License-Identifier: GPL-3.0-or-later

//! The compose window: "New Message" docked at the bottom right, as in
//! webmail, with To (and Cc, Bcc), Subject, the body with the signature,
//! and the Send button with its menu, formatting and attachment buttons
//! and discard. Compose, Reply, Reply all and Forward all open it. It can
//! be minimized to its title bar or opened large in the middle.
//!
//! Sending needs the background service's outbox, so Send only says it is
//! not ready yet and the message stays open.

use gpui::{
    AnyElement, Context, Entity, FocusHandle, Focusable, FontWeight, Hsla, ScrollHandle,
    SharedString, Subscription, Window, deferred, div, prelude::*, px, rgba,
};
use katna_render::{Address, MessageView};
use katna_store::MessageId;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::{InputEvent, TextArea, TextInput};

use super::{MailWindow, NOT_YET};
use crate::format;
use crate::theme::Theme;
use crate::widgets::{elevation, icon, icon_button, icon_button_colored, menu, menu_item};

const WIDTH: f32 = 560.0;
const MAX_HEIGHT: f32 = 620.0;
const MINIMIZED_WIDTH: f32 = 300.0;
const TITLE_HEIGHT: f32 = 40.0;

/// What the window starts from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Kind {
    New,
    Reply,
    ReplyAll,
    Forward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Open,
    Minimized,
    Full,
}

pub(super) struct Compose {
    to: Entity<TextInput>,
    cc: Entity<TextInput>,
    bcc: Entity<TextInput>,
    subject: Entity<TextInput>,
    body: Entity<TextArea>,
    /// The body as it was opened, to tell whether anything was written.
    start: String,
    show_cc: bool,
    show_bcc: bool,
    mode: Mode,
    send_menu: bool,
    shown: Spring,
    closing: bool,
    body_scroll: ScrollHandle,
    _subscriptions: Vec<Subscription>,
}

impl Compose {
    /// Something was written that closing would lose.
    fn touched(&self, cx: &gpui::App) -> bool {
        let filled = |input: &Entity<TextInput>| !input.read(cx).text().trim().is_empty();
        [&self.to, &self.cc, &self.bcc, &self.subject]
            .into_iter()
            .any(filled)
            && self.body.read(cx).text() != self.start
            || self.body.read(cx).text() != self.start
    }

    fn title(&self, cx: &gpui::App) -> SharedString {
        let subject = self.subject.read(cx).text().trim();
        if subject.is_empty() {
            "New Message".into()
        } else {
            subject.to_owned().into()
        }
    }
}

/// The message a reply or forward starts from.
pub(super) struct Original<'a> {
    pub view: &'a MessageView,
    /// Its date as the reader shows it.
    pub date: String,
}

/// What a compose window starts with.
#[derive(Debug, Default, PartialEq, Eq)]
struct Draft {
    to: String,
    cc: String,
    subject: String,
    body: String,
}

fn address(a: &Address) -> String {
    match a.name.as_deref().map(str::trim).filter(|n| !n.is_empty()) {
        Some(name) => format!("{name} <{}>", a.email),
        None => a.email.clone(),
    }
}

fn addresses<'a>(list: impl IntoIterator<Item = &'a Address>) -> String {
    let mut seen = std::collections::HashSet::new();
    list.into_iter()
        .filter(|a| seen.insert(a.email.to_lowercase()))
        .map(address)
        .collect::<Vec<_>>()
        .join(", ")
}

/// `subject` with `prefix` ("Re:" or "Fwd:") once.
fn prefixed(prefix: &str, subject: &str) -> String {
    let subject = subject.trim();
    let has = subject
        .get(..prefix.len())
        .is_some_and(|start| start.eq_ignore_ascii_case(prefix));
    if has {
        subject.to_owned()
    } else if subject.is_empty() {
        prefix.to_owned()
    } else {
        format!("{prefix} {subject}")
    }
}

fn draft(
    kind: Kind,
    original: Option<&Original>,
    is_me: impl Fn(&str) -> bool,
    signature: &str,
) -> Draft {
    let signature = signature.trim_end();
    let mut body = String::from("\n");
    if !signature.is_empty() {
        body.push_str("\n-- \n");
        body.push_str(signature);
        body.push('\n');
    }
    let Some(Original { view, date }) = original.filter(|_| kind != Kind::New) else {
        return Draft {
            body,
            ..Draft::default()
        };
    };
    let sender = view.from.first();
    let from_text = sender.map(address).unwrap_or_default();
    match kind {
        Kind::New => unreachable!(),
        Kind::Reply | Kind::ReplyAll => {
            // A reply to my own message goes to its recipients.
            let mine = sender.is_some_and(|a| is_me(&a.email));
            let mut to: Vec<&Address> = if mine {
                view.to.iter().collect()
            } else {
                sender.into_iter().collect()
            };
            let mut cc = Vec::new();
            if kind == Kind::ReplyAll {
                to.extend(view.to.iter().filter(|a| !is_me(&a.email)));
                cc.extend(view.cc.iter().filter(|a| !is_me(&a.email)));
            }
            let to_emails: Vec<String> = to.iter().map(|a| a.email.to_lowercase()).collect();
            cc.retain(|a| !to_emails.contains(&a.email.to_lowercase()));
            body.push('\n');
            body.push_str(&format!("On {date}, {from_text} wrote:\n"));
            for line in view.body.trim_end().lines() {
                if line.is_empty() {
                    body.push_str(">\n");
                } else {
                    body.push_str(&format!("> {line}\n"));
                }
            }
            Draft {
                to: addresses(to),
                cc: addresses(cc),
                subject: prefixed("Re:", &view.subject),
                body,
            }
        }
        Kind::Forward => {
            body.push_str("\n---------- Forwarded message ---------\n");
            body.push_str(&format!("From: {from_text}\n"));
            body.push_str(&format!("Date: {date}\n"));
            body.push_str(&format!("Subject: {}\n", view.subject));
            body.push_str(&format!("To: {}\n", addresses(&view.to)));
            if !view.cc.is_empty() {
                body.push_str(&format!("Cc: {}\n", addresses(&view.cc)));
            }
            body.push('\n');
            body.push_str(view.body.trim_end());
            body.push('\n');
            Draft {
                subject: prefixed("Fwd:", &view.subject),
                body,
                ..Draft::default()
            }
        }
    }
}

impl MailWindow {
    /// Opens the compose window. `source` picks the message a reply or
    /// forward starts from; by default the newest of the open conversation.
    pub(super) fn open_compose(
        &mut self,
        kind: Kind,
        source: Option<MessageId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(compose) = &mut self.compose
            && !compose.closing
        {
            if compose.touched(cx) {
                compose.mode = Mode::Open;
                self.show_snackbar("Send or discard the open message first.", None, cx);
                return;
            }
        }
        let date = |d: Option<i64>| {
            d.and_then(|d| format::local(d, &self.tz))
                .map(format::long_date)
                .unwrap_or_default()
        };
        let original = self
            .reader
            .as_ref()
            .and_then(|reader| reader.view(source))
            .map(|view| Original {
                view,
                date: date(view.date),
            });
        let accounts = &self.accounts;
        let is_me = |email: &str| accounts.iter().any(|a| a.address.eq_ignore_ascii_case(email));
        let draft = draft(kind, original.as_ref(), is_me, &self.config.sending.signature);

        let accent: Hsla = rgba(self.theme(window).accent).into();
        let input = |placeholder: &str, text: &str, cx: &mut Context<Self>| {
            let (placeholder, text) = (placeholder.to_owned(), text.to_owned());
            cx.new(|cx| {
                let mut input = TextInput::new(placeholder, cx);
                input.set_text(text, cx);
                input.set_accent(accent);
                input
            })
        };
        let to = input("", &draft.to, cx);
        let cc = input("", &draft.cc, cx);
        let bcc = input("", "", cx);
        let subject = input("Subject", &draft.subject, cx);
        let body = cx.new(|cx| {
            let mut area = TextArea::new("", cx);
            area.set_text(draft.body.clone(), 0, cx);
            area.set_accent(accent);
            area
        });
        let mut subscriptions = Vec::new();
        // Enter in a field moves on to the next one.
        let fields: [(&Entity<TextInput>, FocusHandle); 4] = [
            (&to, subject.focus_handle(cx)),
            (&cc, subject.focus_handle(cx)),
            (&bcc, subject.focus_handle(cx)),
            (&subject, body.focus_handle(cx)),
        ];
        for (field, next) in fields {
            subscriptions.push(cx.subscribe_in(
                field,
                window,
                move |_, _, event: &InputEvent, window, cx| match event {
                    InputEvent::Submit => window.focus(&next, cx),
                    InputEvent::Changed | InputEvent::Cancel => cx.notify(),
                },
            ));
        }
        subscriptions.push(cx.subscribe_in(
            &body,
            window,
            |this, _, event: &InputEvent, _, cx| match event {
                InputEvent::Submit => this.send_compose(cx),
                InputEvent::Changed => this.keep_cursor_in_view(),
                InputEvent::Cancel => {}
            },
        ));
        let focus = if matches!(kind, Kind::Reply | Kind::ReplyAll) && !draft.to.is_empty() {
            body.focus_handle(cx)
        } else {
            to.focus_handle(cx)
        };
        window.focus(&focus, cx);
        self.compose = Some(Compose {
            to,
            show_cc: !draft.cc.is_empty(),
            cc,
            show_bcc: false,
            bcc,
            subject,
            start: draft.body,
            body,
            mode: Mode::Open,
            send_menu: false,
            shown: Spring::new(motion::SLIDE, 0.0),
            closing: false,
            body_scroll: ScrollHandle::new(),
            _subscriptions: subscriptions,
        });
        cx.notify();
    }

    /// Scrolls the body so the cursor stays visible while typing.
    fn keep_cursor_in_view(&mut self) {
        // The body grows with its text; a scroll to the cursor waits for
        // the next layout, which the scroll handle does on its own when the
        // cursor child is tracked. Nothing to do until the text area
        // reports a cursor row.
    }

    fn send_compose(&mut self, cx: &mut Context<Self>) {
        if let Some(compose) = &mut self.compose {
            compose.send_menu = false;
        }
        self.show_snackbar(NOT_YET, None, cx);
    }

    fn close_compose(&mut self, discarded: bool, cx: &mut Context<Self>) {
        if let Some(compose) = &mut self.compose {
            compose.closing = true;
            compose.send_menu = false;
        }
        if discarded {
            self.show_snackbar("Draft discarded", None, cx);
        }
        cx.notify();
    }

    fn compose_mode(&mut self, mode: Mode, cx: &mut Context<Self>) {
        if let Some(compose) = &mut self.compose {
            compose.mode = if compose.mode == mode { Mode::Open } else { mode };
            compose.send_menu = false;
        }
        cx.notify();
    }

    pub(super) fn render_compose(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let compose = self.compose.as_mut()?;
        compose.shown.set(if compose.closing { 0.0 } else { 1.0 });
        let t = compose.shown.tick(window, reduce);
        if compose.closing && compose.shown.settled() {
            self.compose = None;
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let compose = self.compose.as_ref()?;
        let viewport = window.viewport_size();
        let (vw, vh) = (f32::from(viewport.width), f32::from(viewport.height));
        let mode = compose.mode;
        let title = compose.title(cx);

        let title_bar = div()
            .id("compose-title")
            .flex_none()
            .h(px(TITLE_HEIGHT))
            .pl(px(16.0))
            .pr(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0))
            .bg(rgba(if th.dark { th.menu } else { th.page }))
            .cursor_pointer()
            .on_click(cx.listener(|this, _, _, cx| this.compose_mode(Mode::Minimized, cx)))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .child(title),
            )
            .child(small_button("compose-minimize", "minimize", th).on_click(cx.listener(
                |this, _, _, cx| {
                    cx.stop_propagation();
                    this.compose_mode(Mode::Minimized, cx)
                },
            )))
            .child(
                small_button(
                    "compose-full",
                    if mode == Mode::Full { "close-full" } else { "open-full" },
                    th,
                )
                .on_click(cx.listener(|this, _, _, cx| {
                    cx.stop_propagation();
                    this.compose_mode(Mode::Full, cx)
                })),
            )
            .child(small_button("compose-close", "close", th).on_click(cx.listener(
                |this, _, _, cx| {
                    cx.stop_propagation();
                    this.close_compose(false, cx)
                },
            )));

        let (width, height) = match mode {
            Mode::Open => (WIDTH.min(vw - 32.0), MAX_HEIGHT.min(vh - 96.0)),
            Mode::Minimized => (MINIMIZED_WIDTH, TITLE_HEIGHT),
            Mode::Full => ((vw - 128.0).clamp(WIDTH, 1000.0), vh - 96.0),
        };
        let panel = div()
            .id("compose")
            .key_context("Compose")
            .occlude()
            .w(px(width))
            .h(px(height))
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(rgba(th.surface))
            .text_color(rgba(th.text))
            .shadow(elevation(th, 3.0))
            .map(|d| match mode {
                Mode::Full => d.rounded(px(12.0)),
                _ => d.rounded_t(px(12.0)),
            })
            .child(title_bar)
            .when(mode != Mode::Minimized, |d| {
                d.child(self.render_compose_fields(th, cx))
                    .child(self.render_compose_body(th, cx))
                    .child(self.render_compose_actions(th, cx))
            });

        Some(match mode {
            Mode::Full => div()
                .absolute()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgba(0x0000_0000 | (0x66 as f32 * t) as u32))
                .child(
                    div()
                        .id("compose-scrim")
                        .absolute()
                        .size_full()
                        .on_click(cx.listener(|this, _, _, cx| this.compose_mode(Mode::Full, cx))),
                )
                .child(div().opacity(t).mt(px(lerp(24.0, 0.0, t))).child(panel))
                .into_any_element(),
            _ => div()
                .absolute()
                .right(px(24.0))
                .bottom(px(lerp(-48.0, 0.0, t)))
                .opacity(t)
                .child(panel)
                .into_any_element(),
        })
    }

    fn render_compose_fields(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let row = |label: &'static str, input: &Entity<TextInput>| {
            div()
                .flex_none()
                .mx(px(16.0))
                .h(px(40.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .border_b_1()
                .border_color(rgba(th.divider))
                .text_size(px(14.0))
                .when(!label.is_empty(), |d| {
                    d.child(
                        div()
                            .flex_none()
                            .text_color(rgba(th.text_dim))
                            .child(label),
                    )
                })
                .child(div().flex_1().min_w_0().child(input.clone()))
        };
        let link = |id: &'static str, label: &'static str| {
            div()
                .id(id)
                .px(px(4.0))
                .rounded(px(4.0))
                .text_size(px(14.0))
                .text_color(rgba(th.text_dim))
                .cursor_pointer()
                .hover(|s| s.text_color(rgba(th.text)).bg(rgba(th.hover)))
        };
        let to = row("To", &compose.to).child(
            div()
                .flex_none()
                .flex()
                .flex_row()
                .gap(px(4.0))
                .when(!compose.show_cc, |d| {
                    d.child(link("compose-cc", "Cc").child("Cc").on_click(cx.listener(
                        |this, _, window, cx| {
                            if let Some(c) = &mut this.compose {
                                c.show_cc = true;
                                window.focus(&c.cc.focus_handle(cx), cx);
                            }
                            cx.notify();
                        },
                    )))
                })
                .when(!compose.show_bcc, |d| {
                    d.child(link("compose-bcc", "Bcc").child("Bcc").on_click(cx.listener(
                        |this, _, window, cx| {
                            if let Some(c) = &mut this.compose {
                                c.show_bcc = true;
                                window.focus(&c.bcc.focus_handle(cx), cx);
                            }
                            cx.notify();
                        },
                    )))
                }),
        );
        div()
            .flex_none()
            .flex()
            .flex_col()
            .child(to)
            .when(compose.show_cc, |d| d.child(row("Cc", &compose.cc)))
            .when(compose.show_bcc, |d| d.child(row("Bcc", &compose.bcc)))
            .child(row("", &compose.subject))
            .into_any_element()
    }

    fn render_compose_body(&self, _th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let focus = compose.body.focus_handle(cx);
        div()
            .id("compose-body")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .track_scroll(&compose.body_scroll)
            .px(px(16.0))
            .py(px(12.0))
            .text_size(px(14.0))
            .line_height(px(20.0))
            .cursor_text()
            // A click below the text still puts the cursor in the body.
            .on_click(move |_, window, cx| window.focus(&focus, cx))
            .child(compose.body.clone())
            .into_any_element()
    }

    fn render_compose_actions(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let later = |what: &'static str| {
            move |this: &mut Self, _: &gpui::ClickEvent, _: &mut Window, cx: &mut Context<Self>| {
                this.show_snackbar(format!("{what} is not ready yet."), None, cx)
            }
        };
        let send = div()
            .relative()
            .flex_none()
            .h(px(36.0))
            .flex()
            .flex_row()
            .items_center()
            .rounded_full()
            .bg(rgba(th.accent))
            .text_color(rgba(th.on_accent))
            .text_size(px(14.0))
            .font_weight(FontWeight::MEDIUM)
            .child(
                div()
                    .id("compose-send")
                    .h_full()
                    .pl(px(20.0))
                    .pr(px(14.0))
                    .flex()
                    .items_center()
                    .rounded_l_full()
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(0xffffff1f)))
                    .on_click(cx.listener(|this, _, _, cx| this.send_compose(cx)))
                    .child("Send"),
            )
            .child(div().w(px(1.0)).h(px(20.0)).bg(rgba(0xffffff66)))
            .child(
                div()
                    .id("compose-send-more")
                    .h_full()
                    .pl(px(6.0))
                    .pr(px(10.0))
                    .flex()
                    .items_center()
                    .rounded_r_full()
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(0xffffff1f)))
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(c) = &mut this.compose {
                            c.send_menu = !c.send_menu;
                        }
                        cx.notify();
                    }))
                    .child(icon("drop-down", th.on_accent, 20.0)),
            )
            .when(compose.send_menu, |d| {
                d.child(
                    deferred(
                        div()
                            .absolute()
                            .bottom(px(44.0))
                            .left(px(0.0))
                            .occlude()
                            .child(
                                menu(th).w(px(220.0)).child(
                                    menu_item("compose-schedule", "Schedule send", th)
                                        .child(icon("schedule", th.text_dim, 20.0))
                                        .on_click(cx.listener(later("Scheduled sending"))),
                                ),
                            ),
                    )
                    .with_priority(2),
                )
            });
        let tool = |id: &'static str, name: &'static str, what: &'static str| {
            icon_button(id, name, 20.0, th).on_click(cx.listener(later(what)))
        };
        div()
            .flex_none()
            .h(px(60.0))
            .px(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0))
            .child(send)
            .child(div().w(px(8.0)))
            .child(tool("compose-format", "format-text", "Formatting"))
            .child(tool("compose-attach", "attachment", "Attaching files"))
            .child(tool("compose-link", "link", "Inserting links"))
            .child(tool("compose-emoji", "emoji", "Inserting emoji"))
            .child(tool("compose-image", "image", "Inserting images"))
            .child(tool("compose-more", "more", "More options"))
            .child(div().flex_1())
            .child(
                icon_button_colored("compose-discard", "trash", 20.0, th.text_dim, th)
                    .on_click(cx.listener(|this, _, _, cx| this.close_compose(true, cx))),
            )
            .into_any_element()
    }
}

/// A small button of the title bar.
fn small_button(
    id: &'static str,
    name: &'static str,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .size(px(28.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .child(icon(name, th.text_dim, 18.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr(name: Option<&str>, email: &str) -> Address {
        Address {
            name: name.map(str::to_owned),
            email: email.to_owned(),
        }
    }

    fn view() -> MessageView {
        MessageView {
            subject: "Gas prices".to_owned(),
            from: vec![addr(Some("Kay Mann"), "kay@enron.com")],
            to: vec![addr(None, "me@enron.com"), addr(Some("Bob"), "bob@enron.com")],
            cc: vec![addr(None, "sara@enron.com"), addr(None, "kay@enron.com")],
            date: None,
            body: "Hello.\n\nSee you.".to_owned(),
            truncated: false,
            from_html: false,
            attachments: Vec::new(),
        }
    }

    fn me(email: &str) -> bool {
        email == "me@enron.com"
    }

    #[test]
    fn new_mail_has_the_signature() {
        let d = draft(Kind::New, None, me, "Kay\n");
        assert_eq!(d.body, "\n\n-- \nKay\n");
        assert_eq!(draft(Kind::New, None, me, "").body, "\n");
    }

    #[test]
    fn replies() {
        let view = view();
        let original = Original {
            view: &view,
            date: "Tue, 25 Jun 2002, 22:23".to_owned(),
        };
        let d = draft(Kind::Reply, Some(&original), me, "");
        assert_eq!(d.to, "Kay Mann <kay@enron.com>");
        assert_eq!(d.cc, "");
        assert_eq!(d.subject, "Re: Gas prices");
        assert_eq!(
            d.body,
            "\n\nOn Tue, 25 Jun 2002, 22:23, Kay Mann <kay@enron.com> wrote:\n> Hello.\n>\n> See you.\n"
        );
        let d = draft(Kind::ReplyAll, Some(&original), me, "");
        assert_eq!(d.to, "Kay Mann <kay@enron.com>, Bob <bob@enron.com>");
        assert_eq!(d.cc, "sara@enron.com");
    }

    #[test]
    fn forwards() {
        let view = view();
        let original = Original {
            view: &view,
            date: "Tue".to_owned(),
        };
        let d = draft(Kind::Forward, Some(&original), me, "K");
        assert_eq!(d.to, "");
        assert_eq!(d.subject, "Fwd: Gas prices");
        assert!(d.body.starts_with("\n\n-- \nK\n\n---------- Forwarded message ---------\n"));
        assert!(d.body.contains("From: Kay Mann <kay@enron.com>\nDate: Tue\n"));
        assert!(d.body.ends_with("\nHello.\n\nSee you.\n"));
    }

    #[test]
    fn prefixes_once() {
        assert_eq!(prefixed("Re:", "RE: x"), "RE: x");
        assert_eq!(prefixed("Re:", "x"), "Re: x");
        assert_eq!(prefixed("Fwd:", ""), "Fwd:");
    }
}
