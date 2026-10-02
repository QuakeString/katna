// SPDX-License-Identifier: GPL-3.0-or-later

//! Writing a first draft with AI: while a reply, reply all or forward is
//! still empty, the sparkle (Ctrl+J) writes from the conversation rather
//! than rephrasing. The Rephrase card opens with a few ideas of what to
//! say and a box for the user's own words; picking one writes the draft,
//! and Insert puts it where the user writes, as one step Undo takes back.
//! Once there is text, the sparkle rephrases again.

use gpui::{AnyElement, Context, Focusable, Task, Window, div, prelude::*, rgba};
use katna_ai::draft::{DraftKind, DraftRequest, Length, Manner};
use katna_ai::wire::{plan, problem};
use katna_i18n::tr;
use katna_ui::{InputEvent, TextInput, px};

use super::super::super::MailWindow;
use super::super::super::search_panel::chip;
use super::super::recipients::Field;
use super::super::{Kind, Mode};
use super::{Fix, Rephrase, State, idea_placeholder, placeholder, problem_text};
use crate::daemon::{self, Command, Rephrased};
use crate::theme::Theme;
use crate::widgets::{filled_button, icon, icon_button_colored, outlined_button, tip};

/// Why there are no ideas: the conversation is encrypted and Settings
/// keeps writing help out of encrypted mail.
const ENCRYPTED_OFF: &str = "encrypted-off";

/// The draft being written, in the Rephrase card.
pub(in crate::window) struct Write {
    request: DraftRequest,
    ideas: Ideas,
    /// The draft, once an idea is picked.
    draft: Option<Drafted>,
    _ideas: Option<Task<()>>,
    _draft: Option<Task<()>>,
}

enum Ideas {
    Loading,
    Ready(Vec<String>),
    /// A [`problem`] name, or [`ENCRYPTED_OFF`].
    Failed(String),
}

enum Drafted {
    Loading,
    Done(Rephrased),
    Failed(String),
}

impl MailWindow {
    /// What the sparkle writes in the open message, when it writes rather
    /// than rephrases: a reply, reply all or forward with nothing written
    /// yet.
    pub(in crate::window) fn write_kind(&self, cx: &gpui::App) -> Option<DraftKind> {
        if !self.ai_allowed() {
            return None;
        }
        let c = self.compose.as_ref()?;
        c.answering?;
        if c.body.read(cx).own_text_end().is_some() {
            return None;
        }
        match c.kind {
            Kind::New => None,
            Kind::Forward => Some(DraftKind::Forward),
            Kind::Reply | Kind::ReplyAll if c.mode == Mode::Inline && self.chat_shown() => {
                Some(DraftKind::Chat)
            }
            Kind::Reply | Kind::ReplyAll => Some(DraftKind::Reply),
        }
    }

    /// Opens the card for writing: asks for ideas from the conversation
    /// (encrypted mail asks first).
    pub(super) fn open_write(
        &mut self,
        kind: DraftKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(key) = self.compose.as_ref().and_then(|c| c.answering) else {
            return;
        };
        let Some(gathered) = self.gather_summary(key, false) else {
            return;
        };
        let (me, to) = match &self.compose {
            Some(c) => {
                let me = c
                    .from
                    .and_then(|id| self.accounts.iter().find(|a| a.id == id))
                    .map(|a| a.display_name.trim().to_owned())
                    .unwrap_or_default();
                (me, c.chips.names(Field::To).join(", "))
            }
            None => return,
        };
        let request = DraftRequest {
            kind,
            subject: gathered.request.subject,
            mails: gathered.request.mails,
            me,
            to,
            ideas: true,
            idea: String::new(),
            length: Length::Short,
            manner: Manner::Friendly,
        };
        let accent = rgba(self.theme(window).accent).into();
        let own = cx.new(|cx| {
            let mut input = TextInput::new(tr!("compose-ai-write-own"), cx);
            input.set_accent(accent);
            input
        });
        let subscription = cx.subscribe_in(
            &own,
            window,
            |this, input, event: &InputEvent, window, cx| match event {
                InputEvent::Submit => {
                    let idea = input.read(cx).text().trim().to_owned();
                    if !idea.is_empty() {
                        this.start_draft(idea, cx);
                    }
                }
                InputEvent::Cancel => this.close_rephrase(window, cx),
                InputEvent::Changed => {}
            },
        );
        let encrypted_off = gathered.encrypted && !self.config.ai.encrypted;
        let Some(c) = &mut self.compose else {
            return;
        };
        c.popup = None;
        let ask = !encrypted_off && (gathered.encrypted || c.sealing.encrypt) && !c.ai_encrypted_ok;
        c.rephrase = Some(Rephrase {
            room: Default::default(),
            moved: Default::default(),
            grab: None,
            text: String::new(),
            tone: katna_ai::Tone::Clearer,
            state: if ask { State::Ask } else { State::Loading },
            more: false,
            custom: own,
            _custom: subscription,
            _task: None,
            write: Some(Write {
                request,
                ideas: if encrypted_off {
                    Ideas::Failed(ENCRYPTED_OFF.to_owned())
                } else {
                    Ideas::Loading
                },
                draft: None,
                _ideas: None,
                _draft: None,
            }),
        });
        if !ask && !encrypted_off {
            self.start_ideas(cx);
        }
        cx.notify();
    }

    /// Asks for ideas of what to say.
    pub(super) fn start_ideas(&mut self, cx: &mut Context<Self>) {
        let connection = self.daemon.clone();
        let Some(w) = self.writing_mut() else {
            return;
        };
        let mut request = w.request.clone();
        request.ideas = true;
        w.ideas = Ideas::Loading;
        w._ideas = Some(cx.spawn(async move |this, cx| {
            let result = ask(connection, request, cx).await;
            this.update(cx, |this, cx| {
                if let Some(w) = this.writing_mut() {
                    w.ideas = match result {
                        Ok(done) => match serde_json::from_str::<Vec<String>>(&done.text) {
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
    fn start_draft(&mut self, idea: String, cx: &mut Context<Self>) {
        let connection = self.daemon.clone();
        let Some(w) = self.writing_mut() else {
            return;
        };
        w.request.idea = idea;
        let mut request = w.request.clone();
        request.ideas = false;
        w.draft = Some(Drafted::Loading);
        w._draft = Some(cx.spawn(async move |this, cx| {
            let asked = request.idea.clone();
            let result = ask(connection, request, cx).await;
            this.update(cx, |this, cx| {
                if let Some(w) = this.writing_mut()
                    && w.request.idea == asked
                    && w.draft.is_some()
                {
                    w.draft = Some(match result {
                        Ok(done) => Drafted::Done(done),
                        Err(problem) => Drafted::Failed(problem),
                    });
                    cx.notify();
                }
            })
            .ok();
        }));
        cx.notify();
    }

    fn writing_mut(&mut self) -> Option<&mut Write> {
        self.compose.as_mut()?.rephrase.as_mut()?.write.as_mut()
    }

    /// Back from the draft to the ideas.
    fn back_to_ideas(&mut self, cx: &mut Context<Self>) {
        if let Some(w) = self.writing_mut() {
            w.draft = None;
            w._draft = None;
        }
        cx.notify();
    }

    /// Puts the draft where the user writes.
    fn insert_draft(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        let text = match c.rephrase.as_ref().and_then(|r| r.write.as_ref()) {
            Some(Write {
                draft: Some(Drafted::Done(done)),
                ..
            }) => done.text.clone(),
            _ => return,
        };
        c.rephrase = None;
        let body = c.body.clone();
        window.focus(&body.focus_handle(cx), cx);
        // Written into in the meantime: then the draft is not put in.
        if body.read(cx).own_text_end().is_some() {
            cx.notify();
            return;
        }
        body.update(cx, |editor, cx| editor.insert_at_start(&text, cx));
        let depth = body.read(cx).undo_depth();
        if let Some(c) = &mut self.compose {
            c.rephrased = Some(depth);
        }
        self.show_snackbar(tr!("compose-ai-written"), Some(Command::UndoRephrase), cx);
        cx.notify();
    }

    fn copy_draft(&mut self, cx: &mut Context<Self>) {
        let Some(Write {
            draft: Some(Drafted::Done(done)),
            ..
        }) = self
            .compose
            .as_ref()
            .and_then(|c| c.rephrase.as_ref())
            .and_then(|r| r.write.as_ref())
        else {
            return;
        };
        cx.write_to_clipboard(gpui::ClipboardItem::new_string(done.text.clone()));
        self.show_snackbar(tr!("compose-ai-copied"), None, cx);
    }

    fn set_draft_length(&mut self, length: Length, cx: &mut Context<Self>) {
        if let Some(w) = self.writing_mut() {
            w.request.length = length;
        }
        cx.notify();
    }

    fn set_draft_manner(&mut self, manner: Manner, cx: &mut Context<Self>) {
        if let Some(w) = self.writing_mut() {
            w.request.manner = manner;
        }
        cx.notify();
    }

    /// The card's insides while writing.
    pub(super) fn render_write_body(
        &self,
        r: &Rephrase,
        w: &Write,
        service: &str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let forward = w.request.kind == DraftKind::Forward;
        let title = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .child(icon("pen-sparkle", th.accent, 17.0))
            .child(
                div()
                    .flex_none()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .child(if forward {
                        tr!("compose-ai-write-note")
                    } else {
                        tr!("compose-ai-write-reply")
                    }),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .text_size(px(12.5))
                    .text_color(rgba(th.text_faint))
                    .child(match &w.draft {
                        Some(_) => w.request.idea.clone(),
                        None => tr!(
                            "compose-ai-write-from",
                            count = w.request.mails.len().min(katna_ai::summary::MAX_MAILS) as u64
                        ),
                    }),
            );
        let body = match &w.draft {
            None => self.render_ideas(r, w, service, th, cx),
            Some(drafted) => self.render_draft(drafted, service, th, cx),
        };
        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(title)
            .child(body)
            .into_any_element()
    }

    fn render_ideas(
        &self,
        r: &Rephrase,
        w: &Write,
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
        let ideas = match &w.ideas {
            Ideas::Loading => idea_placeholder(th, cx.reduce_motion()),
            Ideas::Ready(ideas) => div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(6.0))
                .children(ideas.iter().enumerate().map(|(n, idea)| {
                    let idea = idea.clone();
                    chip(("compose-write-idea", n), &idea, false, th)
                        .rounded_full()
                        .on_click(
                            cx.listener(move |this, _, _, cx| this.start_draft(idea.clone(), cx)),
                        )
                }))
                .into_any_element(),
            Ideas::Failed(problem) => {
                let (text, fix) = if problem == ENCRYPTED_OFF {
                    (
                        tr!("compose-ai-write-encrypted-off"),
                        Fix::Settings(super::super::super::settings_page::Section::Signatures),
                    )
                } else {
                    problem_text(problem, service)
                };
                self.render_problem(text, fix, true, th, cx)
            }
        };
        let focus = r.custom.focus_handle(cx);
        let own = div()
            .id("compose-write-own")
            .h(px(32.0))
            .px(px(10.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgba(th.outline))
            .text_size(px(13.0))
            .on_click(move |_, window, cx| window.focus(&focus, cx))
            .child(icon("pen-sparkle", th.text_faint, 15.0))
            .child(div().flex_1().min_w_0().child(r.custom.clone()));
        let chat = w.request.kind == DraftKind::Chat;
        let length_chip = |length: Length, id: &'static str, label: String| {
            chip(id, &label, w.request.length == length, th)
                .rounded_full()
                .on_click(cx.listener(move |this, _, _, cx| this.set_draft_length(length, cx)))
        };
        let manner_chip = |manner: Manner, id: &'static str, label: String| {
            chip(id, &label, w.request.manner == manner, th)
                .rounded_full()
                .on_click(cx.listener(move |this, _, _, cx| this.set_draft_manner(manner, cx)))
        };
        let how = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(6.0))
            .when(!chat, |d| {
                d.child(length_chip(
                    Length::Short,
                    "compose-write-short",
                    tr!("compose-ai-write-short"),
                ))
                .child(length_chip(
                    Length::Longer,
                    "compose-write-longer",
                    tr!("compose-ai-write-longer"),
                ))
            })
            .child(div().flex_1())
            .child(manner_chip(
                Manner::Friendly,
                "compose-write-friendly",
                tr!("compose-ai-write-friendly"),
            ))
            .child(manner_chip(
                Manner::Formal,
                "compose-write-formal",
                tr!("compose-ai-write-formal"),
            ));
        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(label(tr!("compose-ai-write-ideas")))
            .child(ideas)
            .child(own)
            .child(how)
            .into_any_element()
    }

    fn render_draft(
        &self,
        drafted: &Drafted,
        service: &str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let preview = div()
            .id("compose-write-preview")
            .min_h(px(64.0))
            .max_h(px(260.0))
            .overflow_y_scroll()
            .p(px(10.0))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgba(th.outline))
            .bg(rgba(th.surface))
            .line_height(px(20.0))
            .child(match drafted {
                Drafted::Loading => placeholder(th, cx.reduce_motion()),
                Drafted::Done(done) => div()
                    .flex()
                    .flex_col()
                    .children(done.text.split('\n').map(|line| {
                        if line.trim().is_empty() {
                            div().h(px(10.0)).into_any_element()
                        } else {
                            div().child(line.to_owned()).into_any_element()
                        }
                    }))
                    .into_any_element(),
                Drafted::Failed(problem) => {
                    let (text, fix) = problem_text(problem, service);
                    self.render_problem(text, fix, false, th, cx)
                }
            });
        let done = matches!(drafted, Drafted::Done(_));
        let loading = matches!(drafted, Drafted::Loading);
        let footer = match drafted {
            Drafted::Done(Rephrased {
                plan: kind,
                days_left,
                ..
            }) if kind == plan::TRIAL && *days_left > 0 => tr!(
                "compose-ai-trial-left",
                service = service,
                days = *days_left
            ),
            _ => service.to_owned(),
        };
        let tool = |id: &'static str, name: &'static str, label: String, on: bool| {
            icon_button_colored(id, name, 18.0, th.text_dim, th)
                .size(px(36.0))
                .tooltip(tip(label, th))
                .when(!on, |d| d.opacity(0.4).cursor_default())
        };
        let actions = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .child(
                filled_button("compose-write-insert", tr!("compose-ai-write-insert"), th)
                    .mr(px(4.0))
                    .when(!done, |d| d.opacity(0.5).cursor_default())
                    .on_click(cx.listener(|this, _, window, cx| this.insert_draft(window, cx))),
            )
            .when(loading, |d| {
                d.child(
                    outlined_button("compose-write-stop", tr!("compose-ai-cancel"), th)
                        .on_click(cx.listener(|this, _, _, cx| this.back_to_ideas(cx))),
                )
            })
            .when(!loading, |d| {
                d.child(
                    tool(
                        "compose-write-again",
                        "refresh",
                        tr!("compose-ai-again"),
                        true,
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        let idea = this
                            .writing_mut()
                            .map(|w| w.request.idea.clone())
                            .unwrap_or_default();
                        this.start_draft(idea, cx);
                    })),
                )
                .child(
                    tool(
                        "compose-write-back",
                        "chevron-left",
                        tr!("compose-ai-write-back"),
                        true,
                    )
                    .on_click(cx.listener(|this, _, _, cx| this.back_to_ideas(cx))),
                )
                .child(
                    tool("compose-write-copy", "copy", tr!("compose-ai-copy"), done)
                        .on_click(cx.listener(|this, _, _, cx| this.copy_draft(cx))),
                )
            })
            .child(div().flex_1())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(4.0))
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_faint))
                    .child(icon("sparkle", th.text_faint, 14.0))
                    .child(footer),
            );
        div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(preview)
            .child(actions)
            .into_any_element()
    }

    /// A problem and its button: try again (the ideas, or the draft), or
    /// open Settings.
    fn render_problem(
        &self,
        text: String,
        fix: Fix,
        ideas: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let label = match fix {
            Fix::Retry => tr!("compose-ai-try-again"),
            Fix::Settings(_) => tr!("compose-ai-open-settings"),
        };
        div()
            .flex()
            .flex_col()
            .items_start()
            .gap(px(8.0))
            .child(div().text_color(rgba(th.text_dim)).child(text))
            .child(
                outlined_button("compose-write-fix", label, th)
                    .h(px(30.0))
                    .px(px(14.0))
                    .on_click(cx.listener(move |this, _, window, cx| match fix {
                        Fix::Retry if ideas => this.start_ideas(cx),
                        Fix::Retry => {
                            let idea = this
                                .writing_mut()
                                .map(|w| w.request.idea.clone())
                                .unwrap_or_default();
                            this.start_draft(idea, cx);
                        }
                        Fix::Settings(section) => {
                            this.close_rephrase(window, cx);
                            this.open_settings_page(section, window, cx);
                        }
                    })),
            )
            .into_any_element()
    }
}

/// Sends `request` to the daemon.
async fn ask(
    connection: Option<katna_dbus::zbus::Connection>,
    request: DraftRequest,
    cx: &mut gpui::AsyncApp,
) -> Result<Rephrased, String> {
    cx.background_executor()
        .spawn(async move {
            let connection = match connection {
                Some(connection) => connection,
                None => daemon::connect()
                    .await
                    .map_err(|_| problem::FAILED.to_owned())?,
            };
            daemon::ai_draft(&connection, &request).await
        })
        .await
}
