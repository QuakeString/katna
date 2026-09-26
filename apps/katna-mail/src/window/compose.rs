// SPDX-License-Identifier: GPL-3.0-or-later

//! The compose window: "New Message" docked at the bottom right, as in
//! webmail, with To (and Cc, Bcc), Subject, the body with the signature,
//! and the Send button with its menu, formatting and attachment buttons
//! and discard. Compose opens it; it can be minimized to its title bar or
//! opened large in the middle.
//!
//! Reply, Reply all and Forward write inline instead, at the foot of the
//! open conversation, and can pop out into the window.
//!
//! Send hands the message to the background service's outbox, which holds
//! it for the undo-send delay; the snackbar's Undo takes it back and opens
//! it again.

use gpui::{
    AnyElement, Context, Entity, FocusHandle, Focusable, FontWeight, Hsla, ScrollHandle,
    SharedString, Subscription, Window, deferred, div, prelude::*, px, rgba,
};
use katna_render::{Address, MessageView};
use katna_store::MessageId;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::{InputEvent, TextArea, TextInput};

use std::time::Duration;

use super::{MailWindow, SNACKBAR_TIME};

mod security;
use crate::daemon::{self, Command};
use crate::data::EntryKey;
use crate::format;
use crate::outgoing::{self, Mailbox, Outgoing};
use crate::theme::{Theme, fade};
use crate::widgets::{
    avatar, elevation, icon, icon_button, icon_button_colored, menu, menu_item, tip,
};
use security::Sealing;

const WIDTH: f32 = 560.0;
const MAX_HEIGHT: f32 = 620.0;
const MINIMIZED_WIDTH: f32 = 300.0;
const TITLE_HEIGHT: f32 = 40.0;
/// How long after an edit the text area has drawn its new cursor.
const CURSOR_SETTLE: Duration = Duration::from_millis(24);

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
    /// At the foot of the conversation it answers.
    Inline,
}

pub(super) struct Compose {
    to: Entity<TextInput>,
    cc: Entity<TextInput>,
    bcc: Entity<TextInput>,
    subject: Entity<TextInput>,
    body: Entity<TextArea>,
    /// The fields as they were opened, to tell whether anything was
    /// written.
    start: Draft,
    /// The message this replies to.
    thread: Threading,
    kind: Kind,
    /// The conversation an inline reply belongs to.
    conversation: Option<EntryKey>,
    show_cc: bool,
    show_bcc: bool,
    mode: Mode,
    send_menu: bool,
    /// Sign and encrypt.
    sealing: Sealing,
    shown: Spring,
    closing: bool,
    body_scroll: ScrollHandle,
    _subscriptions: Vec<Subscription>,
}

impl Compose {
    /// What the fields hold now.
    fn fields(&self, cx: &gpui::App) -> Draft {
        let text = |input: &Entity<TextInput>| input.read(cx).text().to_owned();
        Draft {
            to: text(&self.to),
            cc: text(&self.cc),
            bcc: text(&self.bcc),
            subject: text(&self.subject),
            body: self.body.read(cx).text().to_owned(),
        }
    }

    /// Something was written that closing would lose.
    fn touched(&self, cx: &gpui::App) -> bool {
        self.fields(cx) != self.start
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

/// What the fields of a compose window hold.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct Draft {
    to: String,
    cc: String,
    bcc: String,
    subject: String,
    body: String,
}

/// The `In-Reply-To` and `References` of a reply.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Threading {
    in_reply_to: Option<String>,
    references: Vec<String>,
}

impl Threading {
    fn of(kind: Kind, view: Option<&MessageView>) -> Self {
        let Some(view) = view.filter(|_| matches!(kind, Kind::Reply | Kind::ReplyAll)) else {
            return Self::default();
        };
        let mut references = view.references.clone();
        references.extend(view.message_id.clone());
        Self {
            in_reply_to: view.message_id.clone(),
            references,
        }
    }
}

/// A message handed to the outbox, kept so that Undo can reopen it.
pub(super) struct Unsent {
    draft: Draft,
    thread: Threading,
    sealing: Sealing,
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
                ..Draft::default()
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
        let open = self.reader.as_ref().map(|r| r.key);
        if let Some(compose) = &mut self.compose
            && !compose.closing
            && compose.touched(cx)
        {
            let inline_here = compose.mode == Mode::Inline && compose.conversation == open;
            if inline_here {
                window.focus(&compose.body.focus_handle(cx), cx);
            } else {
                if compose.mode == Mode::Minimized {
                    compose.mode = Mode::Open;
                }
                self.show_snackbar("Send or discard the open message first.", None, cx);
            }
            cx.notify();
            return;
        }
        let date = |d: Option<i64>| {
            d.and_then(|d| format::local(d, &self.tz))
                .map(format::long_date)
                .unwrap_or_default()
        };
        let view = self.reader.as_ref().and_then(|reader| reader.view(source));
        let sealing = match kind {
            Kind::New => Sealing::default(),
            _ => Sealing::answering(self.reader.as_ref().and_then(|r| r.security(source))),
        };
        let original = view.map(|view| Original {
            view,
            date: date(view.date),
        });
        let accounts = &self.accounts;
        let is_me = |email: &str| {
            accounts
                .iter()
                .any(|a| a.address.eq_ignore_ascii_case(email))
        };
        let draft = draft(
            kind,
            original.as_ref(),
            is_me,
            &self.config.sending.signature,
        );
        let thread = Threading::of(kind, view);
        let reply = matches!(kind, Kind::Reply | Kind::ReplyAll) && !draft.to.is_empty();
        let start = draft.clone();
        // Answers to the open conversation are written inside it.
        let conversation = self
            .reader
            .as_ref()
            .filter(|_| self.reading && kind != Kind::New)
            .map(|r| r.key);
        let mode = if conversation.is_some() {
            Mode::Inline
        } else {
            Mode::Open
        };
        self.show_compose(draft, start, thread, reply, window, cx);
        if let Some(compose) = &mut self.compose {
            compose.kind = kind;
            compose.mode = mode;
            compose.conversation = conversation;
            compose.sealing = sealing;
        }
    }

    /// Opens the compose window on `draft`; `start` is what counts as
    /// untouched.
    fn show_compose(
        &mut self,
        draft: Draft,
        start: Draft,
        thread: Threading,
        focus_body: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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
        let to = input("Recipients", &draft.to, cx);
        let cc = input("", &draft.cc, cx);
        let bcc = input("", &draft.bcc, cx);
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
            |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Submit => this.send_compose(window, cx),
                InputEvent::Changed => this.keep_cursor_in_view(cx),
                InputEvent::Cancel => {}
            },
        ));
        let focus = if focus_body {
            body.focus_handle(cx)
        } else {
            to.focus_handle(cx)
        };
        window.focus(&focus, cx);
        self.compose = Some(Compose {
            to,
            show_cc: !draft.cc.is_empty(),
            cc,
            show_bcc: !draft.bcc.is_empty(),
            bcc,
            subject,
            body,
            start,
            thread,
            kind: Kind::New,
            conversation: None,
            mode: Mode::Open,
            send_menu: false,
            sealing: Sealing::default(),
            shown: Spring::new(motion::SLIDE, 0.0),
            closing: false,
            body_scroll: ScrollHandle::new(),
            _subscriptions: subscriptions,
        });
        cx.notify();
    }

    /// Scrolls the body so the cursor stays in view while typing. The text
    /// area reports where its cursor was drawn, so this waits for the
    /// frame that draws the change.
    fn keep_cursor_in_view(&mut self, cx: &mut Context<Self>) {
        let Some(compose) = &self.compose else {
            return;
        };
        let (scroll, body) = (compose.body_scroll.clone(), compose.body.clone());
        cx.spawn(async move |_, cx| {
            cx.background_executor().timer(CURSOR_SETTLE).await;
            cx.update(|cx| {
                let Some(cursor) = body.read(cx).cursor_bounds() else {
                    return;
                };
                let view = scroll.bounds();
                let pad = px(12.0);
                let mut offset = scroll.offset();
                if cursor.bottom() + pad > view.bottom() {
                    offset.y -= cursor.bottom() + pad - view.bottom();
                } else if cursor.top() - pad < view.top() {
                    offset.y += view.top() - (cursor.top() - pad);
                } else {
                    return;
                }
                offset.y = offset.y.min(px(0.0));
                scroll.set_offset(offset);
                body.update(cx, |_, cx| cx.notify());
            });
        })
        .detach();
    }

    /// The account new mail goes out from: that of the open folder, or the
    /// first.
    fn compose_account(&self) -> Option<&katna_core::Account> {
        let open = self.folder.and_then(|folder| self.tree.account_of(folder));
        open.and_then(|id| self.accounts.iter().find(|a| a.id == id))
            .or_else(|| self.accounts.first())
    }

    fn send_compose(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        compose.send_menu = false;
        let draft = compose.fields(cx);
        let thread = compose.thread.clone();
        let sealing = compose.sealing;
        let parse = |text: &str| outgoing::parse_addresses(text);
        let (to, cc, bcc) = match (parse(&draft.to), parse(&draft.cc), parse(&draft.bcc)) {
            (Ok(to), Ok(cc), Ok(bcc)) => (to, cc, bcc),
            (Err(bad), ..) | (_, Err(bad), _) | (.., Err(bad)) => {
                self.show_snackbar(
                    format!("\u{201c}{bad}\u{201d} is not an email address."),
                    None,
                    cx,
                );
                return;
            }
        };
        if to.is_empty() && cc.is_empty() && bcc.is_empty() {
            self.show_snackbar("Add at least one recipient.", None, cx);
            return;
        }
        let Some(account) = self.compose_account() else {
            self.show_snackbar("Add an account to send mail from.", None, cx);
            self.open_add_account(window, cx);
            return;
        };
        let from = Mailbox {
            name: Some(account.display_name.trim().to_owned()).filter(|n| !n.is_empty()),
            email: account.address.clone(),
        };
        let emails = |list: &[Mailbox]| list.iter().map(|m| m.email.clone()).collect::<Vec<_>>();
        let sender = account.address.clone();
        let visible = [emails(&to), emails(&cc)].concat();
        let hidden = emails(&bcc);
        let raw = outgoing::build(&Outgoing {
            from: Some(from),
            to,
            cc,
            bcc,
            subject: draft.subject.clone(),
            body: draft.body.clone(),
            in_reply_to: thread.in_reply_to.clone(),
            references: thread.references.clone(),
        });
        let account = account.id.0;
        let delay = self.config.sending.undo_send_seconds;
        self.unsent = Some(Unsent {
            draft,
            thread,
            sealing,
        });
        self.close_compose(false, cx);
        self.show_snackbar("Sending\u{2026}", None, cx);
        let connection = self.daemon.clone();
        cx.spawn_in(window, async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    // Signed and encrypted before the outbox sees it.
                    let raw = security::seal(raw, sealing, sender, visible, hidden)?;
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::queue_send(&connection, account, &raw, delay).await
                })
                .await;
            this.update_in(cx, |this, window, cx| match result {
                Ok(id) => {
                    let (text, undo) = if delay > 0 {
                        ("Message sent", Some(Command::UndoSend(id)))
                    } else {
                        ("Message sent", None)
                    };
                    let time = Duration::from_secs(u64::from(delay)).max(SNACKBAR_TIME);
                    this.show_snackbar_for(text, undo, time, cx);
                }
                Err(err) => {
                    // Nothing went out: the message comes back as it was.
                    this.reopen_unsent(window, cx);
                    this.show_snackbar(err, None, cx);
                }
            })
            .ok();
        })
        .detach();
    }

    /// Opens the message that was just handed to the outbox again, after
    /// Undo or a failure to queue it.
    pub(super) fn reopen_unsent(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(Unsent {
            draft,
            thread,
            sealing,
        }) = self.unsent.take()
        else {
            return;
        };
        if self
            .compose
            .as_ref()
            .is_some_and(|c| !c.closing && c.touched(cx))
        {
            return;
        }
        self.show_compose(draft, Draft::default(), thread, true, window, cx);
        if let Some(compose) = &mut self.compose {
            compose.sealing = sealing;
        }
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
            compose.mode = if compose.mode == mode {
                Mode::Open
            } else {
                mode
            };
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
        let (mode, conversation, closing) = self
            .compose
            .as_ref()
            .map(|c| (c.mode, c.conversation, c.closing))?;
        if mode == Mode::Inline {
            let here = self.reading
                && self
                    .reader
                    .as_ref()
                    .is_some_and(|r| Some(r.key) == conversation);
            let touched = self.compose.as_ref().is_some_and(|c| c.touched(cx));
            if closing || (!here && !touched) {
                self.compose = None;
                return None;
            }
            if here {
                return None;
            }
            // The conversation was left with a reply half written: it
            // carries on in the window.
            if let Some(compose) = &mut self.compose {
                compose.mode = Mode::Open;
                compose.shown.snap(1.0);
            }
        }
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
            .child(
                small_button("compose-minimize", "minimize", th)
                    .tooltip(tip("Minimize", th))
                    .on_click(cx.listener(|this, _, _, cx| {
                        cx.stop_propagation();
                        this.compose_mode(Mode::Minimized, cx)
                    })),
            )
            .child(
                small_button(
                    "compose-full",
                    if mode == Mode::Full {
                        "close-full"
                    } else {
                        "open-full"
                    },
                    th,
                )
                .tooltip(tip(
                    if mode == Mode::Full {
                        "Exit full screen"
                    } else {
                        "Full screen"
                    },
                    th,
                ))
                .on_click(cx.listener(|this, _, _, cx| {
                    cx.stop_propagation();
                    this.compose_mode(Mode::Full, cx)
                })),
            )
            .child(
                small_button("compose-close", "close", th)
                    .tooltip(tip("Close", th))
                    .on_click(cx.listener(|this, _, _, cx| {
                        cx.stop_propagation();
                        // Drafts are not saved yet, so closing loses the text.
                        let touched = this.compose.as_ref().is_some_and(|c| c.touched(cx));
                        this.close_compose(touched, cx)
                    })),
            );

        let (width, height) = match mode {
            Mode::Open | Mode::Inline => (WIDTH.min(vw - 32.0), MAX_HEIGHT.min(vh - 96.0)),
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
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgba(fade(0x0000_0066, t)))
                .child(
                    div()
                        .id("compose-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
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

    /// The reply being written at the foot of conversation `key`, if any:
    /// the sender's picture beside a card with the recipients, the text and
    /// the Send row. `max_body` caps the text's height; longer text scrolls.
    pub(super) fn render_inline_reply(
        &self,
        key: EntryKey,
        max_body: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let compose = self
            .compose
            .as_ref()
            .filter(|c| c.mode == Mode::Inline && !c.closing && c.conversation == Some(key))?;
        let (kind_icon, kind_label) = match compose.kind {
            Kind::ReplyAll => ("reply-all", "Reply all"),
            Kind::Forward => ("forward", "Forward"),
            Kind::Reply | Kind::New => ("reply", "Reply"),
        };
        let me = self.compose_account().map(|a| {
            let name = if a.display_name.trim().is_empty() {
                a.address.clone()
            } else {
                a.display_name.clone()
            };
            avatar(&name, &a.address, 40.0)
        });
        let header = div()
            .flex_none()
            .h(px(44.0))
            .pl(px(12.0))
            .pr(px(6.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .text_size(px(14.0))
            .child(
                div()
                    .id("inline-kind")
                    .flex_none()
                    .tooltip(tip(kind_label, th))
                    .child(icon(kind_icon, th.text_dim, 20.0)),
            )
            .child(div().flex_1().min_w_0().child(compose.to.clone()))
            .when(!compose.show_cc, |d| {
                d.child(
                    div()
                        .id("inline-cc")
                        .px(px(4.0))
                        .rounded(px(4.0))
                        .text_color(rgba(th.text_dim))
                        .cursor_pointer()
                        .hover(|s| s.text_color(rgba(th.text)).bg(rgba(th.hover)))
                        .on_click(cx.listener(|this, _, window, cx| {
                            if let Some(c) = &mut this.compose {
                                c.show_cc = true;
                                window.focus(&c.cc.focus_handle(cx), cx);
                            }
                            cx.notify();
                        }))
                        .child("Cc"),
                )
            })
            .child(
                small_button("inline-pop-out", "open-full", th)
                    .tooltip(tip("Pop out reply", th))
                    .on_click(cx.listener(|this, _, _, cx| this.compose_mode(Mode::Open, cx))),
            );
        let cc = compose.show_cc.then(|| {
            div()
                .flex_none()
                .mx(px(12.0))
                .h(px(36.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .border_t_1()
                .border_color(rgba(th.divider))
                .text_size(px(14.0))
                .child(div().flex_none().text_color(rgba(th.text_dim)).child("Cc"))
                .child(div().flex_1().min_w_0().child(compose.cc.clone()))
        });
        let focus = compose.body.focus_handle(cx);
        let body = div()
            .id("inline-body")
            .min_h(px(96.0))
            .max_h(px(max_body.max(96.0)))
            .overflow_y_scroll()
            .track_scroll(&compose.body_scroll)
            .px(px(12.0))
            .py(px(8.0))
            .text_size(px(14.0))
            .line_height(px(20.0))
            .cursor_text()
            .on_click(move |_, window, cx| window.focus(&focus, cx))
            .child(compose.body.clone());
        let card = div()
            .id("inline-reply")
            .key_context("Compose")
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .rounded(px(12.0))
            .bg(rgba(th.surface))
            .border_1()
            .border_color(rgba(th.divider))
            .shadow(elevation(th, 1.5))
            .child(header)
            .children(cc)
            .child(body)
            .child(self.render_compose_actions(th, cx));
        Some(
            div()
                .flex()
                .flex_row()
                .items_start()
                .gap(px(12.0))
                .pl(px(16.0))
                .pr(px(16.0))
                .pt(px(12.0))
                .pb(px(16.0))
                .children(me)
                .child(card)
                .into_any_element(),
        )
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
                    d.child(div().flex_none().text_color(rgba(th.text_dim)).child(label))
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
                .child(label)
        };
        let to = row("To", &compose.to).child(
            div()
                .flex_none()
                .flex()
                .flex_row()
                .gap(px(4.0))
                .when(!compose.show_cc, |d| {
                    d.child(link("compose-cc", "Cc").on_click(cx.listener(
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
                    d.child(link("compose-bcc", "Bcc").on_click(cx.listener(
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
                    .on_click(cx.listener(|this, _, window, cx| this.send_compose(window, cx)))
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
        let tool =
            |id: &'static str, name: &'static str, label: &'static str, what: &'static str| {
                icon_button(id, name, 20.0, th)
                    .tooltip(tip(label, th))
                    .on_click(cx.listener(later(what)))
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
            .child(tool(
                "compose-format",
                "format-text",
                "Formatting options",
                "Formatting",
            ))
            .child(tool(
                "compose-attach",
                "attachment",
                "Attach files",
                "Attaching files",
            ))
            .child(tool(
                "compose-link",
                "link",
                "Insert link",
                "Inserting links",
            ))
            .child(tool(
                "compose-emoji",
                "emoji",
                "Insert emoji",
                "Inserting emoji",
            ))
            .child(tool(
                "compose-image",
                "image",
                "Insert photo",
                "Inserting images",
            ))
            .child(tool("compose-more", "more", "More options", "More options"))
            .children(self.render_sealing(th, cx))
            .child(div().flex_1())
            .child(
                icon_button_colored("compose-discard", "trash", 20.0, th.text_dim, th)
                    .tooltip(tip("Discard draft", th))
                    .on_click(cx.listener(|this, _, _, cx| this.close_compose(true, cx))),
            )
            .into_any_element()
    }
}

/// A small button of the title bar.
fn small_button(id: &'static str, name: &'static str, th: &Theme) -> gpui::Stateful<gpui::Div> {
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
            to: vec![
                addr(None, "me@enron.com"),
                addr(Some("Bob"), "bob@enron.com"),
            ],
            cc: vec![addr(None, "sara@enron.com"), addr(None, "kay@enron.com")],
            body: "Hello.\n\nSee you.".to_owned(),
            message_id: Some("1@enron.com".to_owned()),
            references: vec!["0@enron.com".to_owned()],
            ..MessageView::default()
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
        assert!(
            d.body
                .starts_with("\n\n-- \nK\n\n---------- Forwarded message ---------\n")
        );
        assert!(
            d.body
                .contains("From: Kay Mann <kay@enron.com>\nDate: Tue\n")
        );
        assert!(d.body.ends_with("\nHello.\n\nSee you.\n"));
    }

    #[test]
    fn prefixes_once() {
        assert_eq!(prefixed("Re:", "RE: x"), "RE: x");
        assert_eq!(prefixed("Re:", "x"), "Re: x");
        assert_eq!(prefixed("Fwd:", ""), "Fwd:");
    }
}
