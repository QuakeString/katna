// SPDX-License-Identifier: GPL-3.0-or-later

//! Replying from the summary card beside a line of the list. Reply turns
//! the card itself into a Write reply card, with no other popup: ideas
//! from the conversation and a box for the user's own words, then an
//! editable draft. Send sends it as the conversation's reply; Open moves
//! it into the conversation's reply box. A draft not sent stays with the
//! conversation: the card shows it again, and so does the reply box.
//! The draft has no sign-off: the signature tag under it picks the
//! signature the reply goes out with, as in Compose.

use crate::widgets::Tip as _;
use gpui::{
    AnyElement, Context, Entity, Focusable, Subscription, Task, Window, div, prelude::*, rgba,
};
use katna_ai::draft::{DraftKind, DraftRequest, Length, Manner};
use katna_ai::wire::problem;
use katna_i18n::tr;
use katna_ui::{InputEvent, TextArea, TextInput, px};

use super::super::super::MailWindow;
use super::super::super::compose::{Kind, below_end_over, signature_name, signature_tag};
use super::{Fix, idea_placeholder, placeholder, problem_text};
use crate::daemon;
use crate::data::EntryKey;
use crate::theme::Theme;
use crate::widgets::choice_chip;
use crate::widgets::{filled_button, icon, icon_button_colored, outlined_button};

/// Writing a reply in the summary card.
pub(super) struct PeekReply {
    request: DraftRequest,
    ideas: Ideas,
    draft: Drafted,
    /// The draft, editable.
    area: Entity<TextArea>,
    /// The user's own words for what it should say.
    own: Entity<TextInput>,
    /// The summary's gist shown whole above, not one line.
    unfold: bool,
    unfold_arrow: crate::widgets::Fold,
    /// The signature the reply goes out with, a
    /// [`katna_core::config::Signature::id`].
    signature: Option<u32>,
    /// The signature tag's list shows.
    signatures_open: bool,
    _ideas: Option<Task<()>>,
    _draft: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

enum Ideas {
    Loading,
    Ready(Vec<String>),
    /// A [`problem`] name.
    Failed(String),
}

#[derive(PartialEq)]
enum Drafted {
    /// Still picking an idea.
    No,
    Loading,
    /// In the box.
    Ready,
    /// A [`problem`] name.
    Failed(String),
}

/// Whether `text` reads formal: a reply to it starts Formal.
fn reads_formal(text: &str) -> bool {
    let lower = text.to_lowercase();
    [
        "dear ",
        "sir",
        "madam",
        "regards",
        "sincerely",
        "respected",
        "kindly",
    ]
    .iter()
    .any(|word| lower.contains(word))
}

impl MailWindow {
    /// Reply in the summary card: the card turns into a Write reply card.
    pub(super) fn start_peek_reply(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(key) = self.summaries.peek.as_ref().map(|p| p.key) else {
            return;
        };
        let Some(gathered) = self.gather_summary(key, false) else {
            return;
        };
        let mails = gathered.request.mails;
        let newest = mails.iter().rev().find(|m| m.from != "me");
        let to = newest.map(|m| m.from.clone()).unwrap_or_default();
        let manner = if newest.is_some_and(|m| reads_formal(&m.text)) {
            Manner::Formal
        } else {
            Manner::Friendly
        };
        let me = self
            .account()
            .and_then(|id| self.accounts.iter().find(|a| a.id == id))
            .map(|a| a.display_name.trim().to_owned())
            .unwrap_or_default();
        let request = DraftRequest {
            kind: DraftKind::Reply,
            subject: gathered.request.subject,
            mails,
            me,
            to,
            ideas: true,
            idea: String::new(),
            length: Length::Short,
            manner,
        };
        let accent = rgba(self.theme(window).accent).into();
        let own = cx.new(|cx| {
            let mut input = TextInput::new(tr!("compose-ai-write-own"), cx);
            input.set_accent(accent);
            input
        });
        let area = cx.new(|cx| {
            let mut area = TextArea::new(String::new(), cx);
            area.set_accent(accent);
            area
        });
        let subscriptions = vec![
            cx.subscribe_in(
                &own,
                window,
                |this, input, event: &InputEvent, _, cx| match event {
                    InputEvent::Submit => {
                        let idea = input.read(cx).text().trim().to_owned();
                        if !idea.is_empty() {
                            this.peek_reply_draft(idea, cx);
                        }
                    }
                    InputEvent::Cancel => {
                        this.close_summary_peek(cx);
                    }
                    InputEvent::Changed => {}
                },
            ),
            cx.subscribe_in(
                &area,
                window,
                |this, _, event: &InputEvent, window, cx| match event {
                    InputEvent::Submit => this.peek_reply_out(true, window, cx),
                    InputEvent::Cancel => {
                        this.close_summary_peek(cx);
                    }
                    InputEvent::Changed => {}
                },
            ),
        ];
        // As Compose would sign it.
        let signature = if self.reader.as_ref().is_some_and(|r| r.key == key) {
            self.signature_for(Kind::Reply)
        } else {
            let sending = &self.config.sending;
            sending.signature(sending.reply_signature).map(|s| s.id)
        };
        // A draft kept from before comes back as it was left.
        let kept = self.summaries.kept_replies.get(&key).cloned();
        let draft = match &kept {
            Some(text) => {
                area.update(cx, |area, cx| area.set_text(text.clone(), text.len(), cx));
                Drafted::Ready
            }
            None => Drafted::No,
        };
        let Some(peek) = &mut self.summaries.peek else {
            return;
        };
        peek.reply = Some(PeekReply {
            request,
            ideas: Ideas::Loading,
            draft,
            area: area.clone(),
            own: own.clone(),
            unfold: false,
            unfold_arrow: crate::widgets::Fold::default(),
            signature,
            signatures_open: false,
            _ideas: None,
            _draft: None,
            _subscriptions: subscriptions,
        });
        if kept.is_some() {
            window.focus(&area.focus_handle(cx), cx);
        } else {
            window.focus(&own.focus_handle(cx), cx);
        }
        self.peek_reply_ideas(cx);
    }

    fn peek_reply_mut(&mut self) -> Option<&mut PeekReply> {
        self.summaries.peek.as_mut()?.reply.as_mut()
    }

    fn peek_reply_ideas(&mut self, cx: &mut Context<Self>) {
        let connection = self.daemon.clone();
        let Some(r) = self.peek_reply_mut() else {
            return;
        };
        let mut request = r.request.clone();
        request.ideas = true;
        r.ideas = Ideas::Loading;
        r._ideas = Some(cx.spawn(async move |this, cx| {
            let result = ask(connection, request, cx).await;
            this.update(cx, |this, cx| {
                if let Some(r) = this.peek_reply_mut() {
                    r.ideas = match result {
                        Ok(text) => match serde_json::from_str::<Vec<String>>(&text) {
                            Ok(ideas) if !ideas.is_empty() => Ideas::Ready(ideas),
                            _ => Ideas::Failed(problem::FAILED.to_owned()),
                        },
                        Err(problem) => Ideas::Failed(problem),
                    };
                    cx.notify();
                }
            })
            .ok();
        }));
        cx.notify();
    }

    /// Writes the draft for `idea`, an idea or the user's own words.
    fn peek_reply_draft(&mut self, idea: String, cx: &mut Context<Self>) {
        let connection = self.daemon.clone();
        let Some(r) = self.peek_reply_mut() else {
            return;
        };
        r.request.idea = idea;
        let mut request = r.request.clone();
        request.ideas = false;
        r.draft = Drafted::Loading;
        r._draft = Some(cx.spawn(async move |this, cx| {
            let asked = request.idea.clone();
            let result = ask(connection, request, cx).await;
            this.update(cx, |this, cx| {
                let Some(r) = this.peek_reply_mut() else {
                    return;
                };
                if r.request.idea != asked || r.draft != Drafted::Loading {
                    return;
                }
                match result {
                    Ok(text) => {
                        let area = r.area.clone();
                        r.draft = Drafted::Ready;
                        area.update(cx, |area, cx| area.set_text(text.clone(), text.len(), cx));
                    }
                    Err(problem) => r.draft = Drafted::Failed(problem),
                }
                cx.notify();
            })
            .ok();
        }));
        cx.notify();
    }

    /// The draft's text when there is one worth keeping.
    pub(super) fn peek_reply_text(&self, cx: &gpui::App) -> Option<(EntryKey, String)> {
        let peek = self.summaries.peek.as_ref()?;
        let r = peek.reply.as_ref()?;
        if r.draft != Drafted::Ready {
            return None;
        }
        let text = r.area.read(cx).text().trim().to_owned();
        (!text.is_empty()).then_some((peek.key, text))
    }

    /// Send (`send`) or Open: the draft goes into the conversation's reply,
    /// which Send then sends.
    fn peek_reply_out(&mut self, send: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some((key, text)) = self.peek_reply_text(cx) else {
            return;
        };
        let signature = self.peek_reply_mut().map(|r| r.signature);
        // Closing keeps the draft; the reply below takes it in.
        self.close_summary_peek(cx);
        self.summaries.kept_replies.insert(key, text);
        let Some(ix) = self.entries.iter().position(|e| e.key == key) else {
            return;
        };
        self.open(ix, window, cx);
        let kind = if self.config.mail.reply_all {
            Kind::ReplyAll
        } else {
            Kind::Reply
        };
        // In the chat view the draft goes into the chat's reply box.
        if self.chat_shown() {
            self.chat_reply(None, kind, window, cx);
        } else {
            self.open_compose(kind, None, window, cx);
        }
        // Signed as picked in the card.
        if let Some(signature) = signature {
            self.choose_signature(signature, cx);
        }
        // Taken in: send it as it is.
        if send && !self.summaries.kept_replies.contains_key(&key) {
            self.send_compose_default(window, cx);
        }
    }

    /// The reply to `key` written in the summary card and not sent, taken
    /// out to go into the conversation's reply box.
    pub(in crate::window) fn take_kept_reply(&mut self, key: EntryKey) -> Option<String> {
        self.summaries.kept_replies.remove(&key)
    }

    /// Keeps `text` as the start of the next reply to `key`: a reply typed
    /// into a notification, to write on in Katna Mail.
    pub(in crate::window) fn keep_reply(&mut self, key: EntryKey, text: String) {
        self.summaries.kept_replies.insert(key, text);
    }

    fn peek_reply_back(&mut self, cx: &mut Context<Self>) {
        if let Some(r) = self.peek_reply_mut() {
            r.draft = Drafted::No;
            r._draft = None;
            cx.notify();
        }
    }

    /// The card while writing: what it answers, then ideas or the draft.
    pub(super) fn render_peek_reply(
        &self,
        r: &PeekReply,
        gist: Option<String>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let service = self.ai_service_name();
        let small = |id: &'static str, name: &str, label: String| {
            icon_button_colored(id, name, 17.0, th.text_dim, th)
                .size(px(30.0))
                .tip(label, th)
        };
        let title = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .child(icon("pen-sparkle", th.accent, 16.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_size(px(14.5))
                    .child(if r.request.to.is_empty() {
                        tr!("compose-ai-write-reply")
                    } else {
                        tr!("summary-reply-to", name = r.request.to.clone())
                    }),
            )
            .child(
                small("summary-reply-close", "close", tr!("summary-close")).on_click(cx.listener(
                    |this, _, _, cx| {
                        this.close_summary_peek(cx);
                    },
                )),
            );
        let unfold = r.unfold;
        let gist = gist.map(|gist| {
            div()
                .id("summary-reply-gist")
                .flex()
                .flex_row()
                .items_start()
                .gap(px(6.0))
                .px(px(10.0))
                .py(px(6.0))
                .rounded(px(10.0))
                .bg(rgba(th.hover))
                .text_size(px(13.0))
                .line_height(px(19.0))
                .text_color(rgba(th.text_dim))
                .cursor_pointer()
                .child(
                    div()
                        .pt(px(2.0))
                        .child(icon("sparkle", th.text_faint, 14.0)),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .when(!unfold, |d| d.truncate())
                        .child(gist),
                )
                .child(div().pt(px(2.0)).child(crate::widgets::fold_arrow(
                    "summary-reply-gist-arrow",
                    &r.unfold_arrow,
                    unfold,
                    th.text_faint,
                    14.0,
                )))
                .on_click(cx.listener(|this, _, _, cx| {
                    if let Some(r) = this.peek_reply_mut() {
                        r.unfold = !r.unfold;
                        cx.notify();
                    }
                }))
        });
        let body = match &r.draft {
            Drafted::No => self.render_peek_ideas(r, &service, th, cx),
            Drafted::Loading => placeholder(th, cx.reduce_motion()),
            Drafted::Ready => self.render_peek_draft(r, th, cx),
            Drafted::Failed(problem) => self.render_peek_problem(problem, false, &service, th, cx),
        };
        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(title)
            .children(gist)
            .child(body)
            .into_any_element()
    }

    fn render_peek_ideas(
        &self,
        r: &PeekReply,
        service: &str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let label = |text: String| {
            div()
                .text_size(px(11.5))
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(rgba(th.text_faint))
                .child(text.to_uppercase())
        };
        let ideas = match &r.ideas {
            Ideas::Loading => idea_placeholder(th, cx.reduce_motion()),
            Ideas::Ready(ideas) => div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(6.0))
                .children(ideas.iter().enumerate().map(|(n, idea)| {
                    let idea = idea.clone();
                    choice_chip(("summary-reply-idea", n), &idea, false, th)
                        .rounded_full()
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.peek_reply_draft(idea.clone(), cx)
                        }))
                }))
                .into_any_element(),
            Ideas::Failed(problem) => self.render_peek_problem(problem, true, service, th, cx),
        };
        let focus = r.own.focus_handle(cx);
        let own = div()
            .id("summary-reply-own")
            .h(px(32.0))
            .px(px(10.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgba(th.outline))
            .bg(rgba(th.surface))
            .text_size(px(13.0))
            .cursor_text()
            .child(icon("pen", th.text_faint, 15.0))
            .child(div().flex_1().min_w_0().child(r.own.clone()))
            .on_click(move |_, window, cx| window.focus(&focus, cx));
        let length = r.request.length;
        let manner = r.request.manner;
        let set = |id: &'static str, text: String, on: bool| {
            choice_chip(id, &text, on, th).rounded_full()
        };
        let choices = div()
            .flex()
            .flex_row()
            .justify_between()
            .gap(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(4.0))
                    .child(
                        set(
                            "summary-reply-short",
                            tr!("compose-ai-write-short"),
                            length == Length::Short,
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.peek_reply_set(Some(Length::Short), None, cx)
                        })),
                    )
                    .child(
                        set(
                            "summary-reply-longer",
                            tr!("compose-ai-write-longer"),
                            length == Length::Longer,
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.peek_reply_set(Some(Length::Longer), None, cx)
                        })),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(4.0))
                    .child(
                        set(
                            "summary-reply-friendly",
                            tr!("compose-ai-write-friendly"),
                            manner == Manner::Friendly,
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.peek_reply_set(None, Some(Manner::Friendly), cx)
                        })),
                    )
                    .child(
                        set(
                            "summary-reply-formal",
                            tr!("compose-ai-write-formal"),
                            manner == Manner::Formal,
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.peek_reply_set(None, Some(Manner::Formal), cx)
                        })),
                    ),
            );
        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(label(tr!("compose-ai-write-ideas")))
            .child(ideas)
            .child(own)
            .child(choices)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .pt(px(6.0))
                    .border_t_1()
                    .border_color(rgba(th.faint_line(0.6)))
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_faint))
                    .child(
                        div()
                            .id("summary-reply-summary")
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(4.0))
                            .h(px(30.0))
                            .px(px(8.0))
                            .rounded_full()
                            .cursor_pointer()
                            .relative()
                            .child(crate::widgets::hover_fade("hover-glow", None, th))
                            .text_color(rgba(th.text_dim))
                            .text_size(px(13.0))
                            .child(icon("chevron-left", th.text_dim, 16.0))
                            .child(tr!("summary-reply-summary"))
                            .on_click(cx.listener(|this, _, _, cx| {
                                if let Some(peek) = &mut this.summaries.peek {
                                    peek.reply = None;
                                    cx.notify();
                                }
                            })),
                    )
                    .child(div().flex_1())
                    .child(service.to_owned()),
            )
            .into_any_element()
    }

    fn peek_reply_set(
        &mut self,
        length: Option<Length>,
        manner: Option<Manner>,
        cx: &mut Context<Self>,
    ) {
        if let Some(r) = self.peek_reply_mut() {
            if let Some(length) = length {
                r.request.length = length;
            }
            if let Some(manner) = manner {
                r.request.manner = manner;
            }
            cx.notify();
        }
    }

    fn render_peek_draft(&self, r: &PeekReply, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let small = |id: &'static str, name: &str, label: String| {
            icon_button_colored(id, name, 17.0, th.text_dim, th)
                .size(px(30.0))
                .tip(label, th)
        };
        let focus = r.area.focus_handle(cx);
        let to = (!r.request.to.is_empty()).then(|| {
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .text_size(px(13.0))
                .text_color(rgba(th.text_dim))
                .child(tr!("compose-to"))
                .child(
                    choice_chip("summary-reply-to", &r.request.to, false, th)
                        .rounded_full()
                        .h(px(24.0)),
                )
        });
        let area = div()
            .id("summary-reply-draft")
            .min_h(px(140.0))
            .max_h(px(320.0))
            .overflow_y_scroll()
            .px(px(12.0))
            .py(px(10.0))
            .rounded(px(12.0))
            .border_1()
            .border_color(rgba(th.accent))
            .bg(rgba(th.surface))
            .text_size(px(13.5))
            .line_height(px(20.0))
            .cursor_text()
            .child(r.area.clone())
            .on_click(move |_, window, cx| window.focus(&focus, cx));
        let signature = r.signature;
        let tag = div().flex().flex_row().justify_end().child(
            signature_tag(
                "summary-reply-signature",
                signature_name(self.config.sending.signature(signature)),
                r.signatures_open,
                th,
            )
            .on_click(cx.listener(|this, _, _, cx| {
                if let Some(r) = this.peek_reply_mut() {
                    r.signatures_open = !r.signatures_open;
                    cx.notify();
                }
            }))
            .when(r.signatures_open, |d| {
                // Over the card, which is drawn on top itself.
                d.child(below_end_over(
                    self.signature_menu(
                        signature,
                        |this, id, cx| {
                            if let Some(r) = this.peek_reply_mut() {
                                r.signature = id;
                                r.signatures_open = false;
                                cx.notify();
                            }
                        },
                        |this| {
                            if let Some(r) = this.peek_reply_mut() {
                                r.signatures_open = false;
                            }
                        },
                        th,
                        cx,
                    ),
                    5,
                ))
            }),
        );
        let idea = r.request.idea.clone();
        let footer = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .pt(px(8.0))
            .border_t_1()
            .border_color(rgba(th.faint_line(0.6)))
            .child(
                filled_button("summary-reply-send", tr!("summary-reply-send"), th)
                    .h(px(30.0))
                    .px(px(14.0))
                    .on_click(
                        cx.listener(|this, _, window, cx| this.peek_reply_out(true, window, cx)),
                    ),
            )
            .child(
                outlined_button("summary-reply-open", tr!("summary-reply-open"), th)
                    .h(px(30.0))
                    .px(px(12.0))
                    .on_click(
                        cx.listener(|this, _, window, cx| this.peek_reply_out(false, window, cx)),
                    ),
            )
            .child(
                small(
                    "summary-reply-again",
                    "refresh",
                    tr!("compose-ai-try-again"),
                )
                .on_click(
                    cx.listener(move |this, _, _, cx| this.peek_reply_draft(idea.clone(), cx)),
                ),
            )
            .child(
                small(
                    "summary-reply-back",
                    "chevron-left",
                    tr!("compose-ai-write-back"),
                )
                .on_click(cx.listener(|this, _, _, cx| this.peek_reply_back(cx))),
            )
            .child(div().flex_1())
            .child(
                small("summary-reply-copy", "copy", tr!("summary-copy")).on_click(cx.listener(
                    |this, _, _, cx| {
                        if let Some((_, text)) = this.peek_reply_text(cx) {
                            cx.write_to_clipboard(gpui::ClipboardItem::new_string(text));
                            this.show_snackbar(tr!("compose-ai-copied"), None, cx);
                        }
                    },
                )),
            );
        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .children(to)
            .child(div().flex().flex_col().gap(px(6.0)).child(area).child(tag))
            .child(footer)
            .into_any_element()
    }

    fn render_peek_problem(
        &self,
        problem: &str,
        ideas: bool,
        service: &str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let (text, fix) = problem_text(problem, service);
        let label = match fix {
            Fix::Retry => tr!("compose-ai-try-again"),
            Fix::Settings(_) => tr!("compose-ai-open-settings"),
        };
        div()
            .flex()
            .flex_col()
            .items_start()
            .gap(px(8.0))
            .child(
                div()
                    .text_size(px(13.5))
                    .text_color(rgba(th.text_dim))
                    .child(text),
            )
            .child(
                outlined_button("summary-reply-fix", label, th)
                    .h(px(30.0))
                    .px(px(14.0))
                    .on_click(cx.listener(move |this, _, window, cx| match fix {
                        Fix::Retry if ideas => this.peek_reply_ideas(cx),
                        Fix::Retry => {
                            let idea = this
                                .peek_reply_mut()
                                .map(|r| r.request.idea.clone())
                                .unwrap_or_default();
                            this.peek_reply_draft(idea, cx);
                        }
                        Fix::Settings(section) => {
                            this.close_summary_peek(cx);
                            this.open_settings_page(section, window, cx);
                        }
                    })),
            )
            .into_any_element()
    }
}

/// Asks the daemon for ideas or a draft: the text, or a [`problem`] name.
async fn ask(
    connection: Option<katna_dbus::zbus::Connection>,
    request: DraftRequest,
    cx: &mut gpui::AsyncApp,
) -> Result<String, String> {
    cx.background_executor()
        .spawn(async move {
            let connection = match connection {
                Some(connection) => connection,
                None => daemon::connect()
                    .await
                    .map_err(|_| problem::FAILED.to_owned())?,
            };
            daemon::ai_draft(&connection, &request)
                .await
                .map(|done| done.text)
        })
        .await
}

#[cfg(test)]
mod tests {
    use super::reads_formal;

    #[test]
    fn formal_mail_is_answered_formally() {
        assert!(reads_formal(
            "Dear Sir/Madam, please arrange to submit your offer."
        ));
        assert!(reads_formal("Thanks.\nKind regards,\nPartho"));
        assert!(!reads_formal("Running Saturday? Usual loop at 7."));
    }
}
