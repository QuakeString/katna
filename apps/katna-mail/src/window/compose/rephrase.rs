// SPDX-License-Identifier: GPL-3.0-or-later

//! Writing help from AI in the compose window (`docs/ARCHITECTURE.md`
//! §16.5). Selecting text shows a small sparkle by its end (or Ctrl+J);
//! it opens a card that rewrites the selection in a tone through the
//! daemon, and Replace puts the new text in as one step Undo takes back.
//! Encrypted mail asks first. [`AiComplete`] finishes the sentence being
//! written, in grey like the phrase suggestions, when that is switched on.

use std::rc::Rc;

use gpui::{
    AnyElement, App, Bounds, ClipboardItem, Context, Entity, Focusable, Pixels, Subscription, Task,
    WeakEntity, Window, anchored, deferred, div, point, prelude::*, rgba,
};
use katna_ai::Tone;
use katna_ai::provider::{self, OTHER};
use katna_ai::wire::{plan, problem};
use katna_core::config::AiSource;
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::rich::Complete;
use katna_ui::{InputEvent, TextInput};

use super::super::MailWindow;
use super::super::search_panel::chip;
use super::super::settings_page::Section;
use crate::daemon::{self, Command, Rephrased};
use crate::theme::{Theme, fade};
use crate::widgets::{
    elevation, filled_button, icon, icon_button_colored, outlined_button, raised, tip,
};

/// The card's width.
const CARD_WIDTH: f32 = 488.0;
/// The tones on the card's first row; the rest wait behind "⋯".
const FIRST_TONES: [Tone; 5] = [
    Tone::Clearer,
    Tone::Shorter,
    Tone::Friendlier,
    Tone::Formal,
    Tone::Grammar,
];

/// The Rephrase card over the message.
pub(in crate::window) struct Rephrase {
    /// Where the selection ends, in window coordinates.
    at: Bounds<Pixels>,
    /// The selected text, as sent.
    text: String,
    tone: Tone,
    state: State,
    /// The second row (Longer, and the user's own instruction) shows.
    more: bool,
    custom: Entity<TextInput>,
    _custom: Subscription,
    _task: Option<Task<()>>,
}

enum State {
    /// Encrypted mail: the selection would leave unencrypted, so the card
    /// asks first.
    Ask,
    Loading,
    Done(Rephrased),
    /// A [`problem`] name.
    Failed(String),
}

/// What the button under a problem does.
#[derive(Clone, Copy)]
enum Fix {
    Retry,
    Settings(Section),
}

impl MailWindow {
    /// Whether writing help may be used for the open message: it is on,
    /// and the message is not encrypted unless Settings allows that.
    fn ai_allowed(&self) -> bool {
        let ai = &self.config.ai;
        ai.source != AiSource::Off
            && self
                .compose
                .as_ref()
                .is_some_and(|c| !c.closing && (!c.sealing.encrypt || ai.encrypted))
    }

    /// The selection that can be rephrased, and where it ends.
    fn rephrase_selection(&self, cx: &App) -> Option<(String, Bounds<Pixels>)> {
        if !self.ai_allowed() {
            return None;
        }
        self.compose.as_ref()?.body.read(cx).rephrasable()
    }

    /// The small sparkle by the end of selected text.
    pub(super) fn render_rephrase_button(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let c = self.compose.as_ref()?;
        if c.popup.is_some() || c.rephrase.is_some() || !c.body.read(cx).had_focus() {
            return None;
        }
        let (_, at) = self.rephrase_selection(cx)?;
        let button = div()
            .id("compose-rephrase")
            .occlude()
            .size(px(30.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .bg(rgba(th.menu))
            .shadow(elevation(th, 2.0))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .tooltip(tip(tr!("compose-ai-rephrase-tip"), th))
            // The text keeps its selection.
            .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_click(cx.listener(|this, _, window, cx| this.open_rephrase(window, cx)))
            .child(icon("sparkle", th.accent, 18.0));
        Some(
            deferred(
                anchored()
                    .position(at.bottom_right() + point(px(4.0), px(4.0)))
                    .snap_to_window_with_margin(px(8.0))
                    .child(button),
            )
            .with_priority(1)
            .into_any_element(),
        )
    }

    /// Ctrl+J: opens the card for the selection, or closes it.
    pub(super) fn toggle_rephrase(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.compose.as_ref().is_some_and(|c| c.rephrase.is_some()) {
            self.close_rephrase(window, cx);
        } else {
            self.open_rephrase(window, cx);
        }
    }

    fn open_rephrase(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some((text, at)) = self.rephrase_selection(cx) else {
            return;
        };
        let accent = rgba(self.theme(window).accent).into();
        let custom = cx.new(|cx| {
            let mut input = TextInput::new(tr!("compose-ai-custom"), cx);
            input.set_accent(accent);
            input
        });
        let subscription = cx.subscribe_in(
            &custom,
            window,
            |this, input, event: &InputEvent, window, cx| match event {
                InputEvent::Submit => {
                    if !input.read(cx).text().trim().is_empty() {
                        this.start_rephrase(Tone::Custom, cx);
                    }
                }
                InputEvent::Cancel => this.close_rephrase(window, cx),
                InputEvent::Changed => {}
            },
        );
        let Some(c) = &mut self.compose else {
            return;
        };
        c.popup = None;
        let ask = c.sealing.encrypt && !c.ai_encrypted_ok;
        c.rephrase = Some(Rephrase {
            at,
            text,
            tone: Tone::Clearer,
            state: if ask { State::Ask } else { State::Loading },
            more: false,
            custom,
            _custom: subscription,
            _task: None,
        });
        if !ask {
            self.start_rephrase(Tone::Clearer, cx);
        }
        cx.notify();
    }

    /// Asks for the selection in `tone`.
    fn start_rephrase(&mut self, tone: Tone, cx: &mut Context<Self>) {
        let connection = self.daemon.clone();
        let Some(r) = self.compose.as_mut().and_then(|c| c.rephrase.as_mut()) else {
            return;
        };
        let instruction = if tone == Tone::Custom {
            r.custom.read(cx).text().trim().to_owned()
        } else {
            String::new()
        };
        r.tone = tone;
        r.state = State::Loading;
        let text = r.text.clone();
        r._task = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect()
                            .await
                            .map_err(|_| problem::FAILED.to_owned())?,
                    };
                    daemon::ai_rephrase(&connection, &text, tone.id(), &instruction).await
                })
                .await;
            this.update(cx, |this, cx| {
                if let Some(r) = this.compose.as_mut().and_then(|c| c.rephrase.as_mut())
                    && r.tone == tone
                {
                    r.state = match result {
                        Ok(done) => State::Done(done),
                        Err(problem) => State::Failed(problem),
                    };
                    cx.notify();
                }
            })
            .ok();
        }));
        cx.notify();
    }

    /// Closes the card, the text keeping the focus.
    pub(super) fn close_rephrase(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(c) = &mut self.compose
            && c.rephrase.take().is_some()
        {
            window.focus(&c.body.focus_handle(cx), cx);
            cx.notify();
        }
    }

    /// Puts the new text in place of the selection, or `below` it.
    fn use_rephrased(&mut self, below: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        let Some(Rephrase {
            text: asked,
            state: State::Done(done),
            ..
        }) = c.rephrase.take()
        else {
            return;
        };
        let body = c.body.clone();
        window.focus(&body.focus_handle(cx), cx);
        // The selection may have moved since; then nothing is replaced.
        if body
            .read(cx)
            .rephrasable()
            .is_none_or(|(now, _)| now != asked)
        {
            cx.notify();
            return;
        }
        body.update(cx, |editor, cx| {
            if below {
                editor.insert_below_selection(&done.text, cx)
            } else {
                editor.replace_selection(&done.text, cx)
            }
        });
        let depth = body.read(cx).undo_depth();
        if let Some(c) = &mut self.compose {
            c.rephrased = Some(depth);
        }
        let text = if below {
            tr!("compose-ai-added")
        } else {
            tr!("compose-ai-replaced")
        };
        self.show_snackbar(text, Some(Command::UndoRephrase), cx);
        cx.notify();
    }

    /// Undo on the snackbar: takes the new text out again, while putting
    /// it in is still the last change to the message.
    pub(in crate::window) fn undo_rephrase(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        let Some(depth) = c.rephrased.take() else {
            return;
        };
        let body = c.body.clone();
        if body.read(cx).undo_depth() == depth {
            body.update(cx, |editor, cx| editor.undo(cx));
        }
        window.focus(&body.focus_handle(cx), cx);
    }

    fn copy_rephrased(&mut self, cx: &mut Context<Self>) {
        let Some(Rephrase {
            state: State::Done(done),
            ..
        }) = self.compose.as_ref().and_then(|c| c.rephrase.as_ref())
        else {
            return;
        };
        cx.write_to_clipboard(ClipboardItem::new_string(done.text.clone()));
        self.show_snackbar(tr!("compose-ai-copied"), None, cx);
    }

    /// Encrypted mail: the user agreed to send the selection, for this
    /// message.
    fn allow_encrypted(&mut self, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        c.ai_encrypted_ok = true;
        let tone = c.rephrase.as_ref().map_or(Tone::Clearer, |r| r.tone);
        self.start_rephrase(tone, cx);
    }

    /// Who answers: Katna AI, or the user's own service by name.
    fn ai_service_name(&self) -> String {
        let ai = &self.config.ai;
        match ai.source {
            AiSource::Own => {
                let preset = provider::preset(&ai.provider);
                if preset.id == OTHER {
                    match ai.model.trim() {
                        "" => tr!("compose-ai-own"),
                        model => model.to_owned(),
                    }
                } else {
                    preset.name.to_owned()
                }
            }
            _ => tr!("compose-ai-katna"),
        }
    }

    /// The card under the selection.
    pub(super) fn render_rephrase(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let r = self.compose.as_ref()?.rephrase.as_ref()?;
        let service = self.ai_service_name();
        let body = match &r.state {
            State::Ask => self.render_rephrase_ask(&service, th, cx),
            _ => self.render_rephrase_body(r, &service, th, cx),
        };
        // The frosted glass is the card's first child, under the text.
        let card = raised(
            div()
                .id("compose-rephrase-card")
                .occlude()
                .w(px(CARD_WIDTH))
                .p(px(12.0))
                .flex()
                .flex_col()
                .gap(px(10.0))
                .text_size(px(14.0))
                .text_color(rgba(th.text))
                .on_mouse_down_out(
                    cx.listener(|this, _, window, cx| this.close_rephrase(window, cx)),
                ),
            th,
            12.0,
            3.0,
        )
        .child(body);
        let at = r.at;
        Some(
            deferred(
                anchored()
                    .position(point(
                        at.right() - px(CARD_WIDTH / 2.0),
                        at.bottom() + px(8.0),
                    ))
                    .snap_to_window_with_margin(px(8.0))
                    .child(card),
            )
            .with_priority(2)
            .into_any_element(),
        )
    }

    /// Encrypted mail: what rephrasing sends, and a choice.
    fn render_rephrase_ask(&self, service: &str, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(10.0))
                    .child(icon("lock", th.text_dim, 20.0))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_color(rgba(th.text_dim))
                            .child(tr!("compose-ai-encrypted", service = service)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .justify_end()
                    .child(
                        outlined_button("compose-rephrase-cancel", tr!("compose-ai-cancel"), th)
                            .on_click(
                                cx.listener(|this, _, window, cx| this.close_rephrase(window, cx)),
                            ),
                    )
                    .child(
                        filled_button("compose-rephrase-anyway", tr!("compose-ai-rephrase"), th)
                            .on_click(cx.listener(|this, _, _, cx| this.allow_encrypted(cx))),
                    ),
            )
            .into_any_element()
    }

    fn render_rephrase_body(
        &self,
        r: &Rephrase,
        service: &str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let tone_chip = |tone: Tone, label: String| {
            chip(
                ("compose-rephrase-tone", tone as usize),
                &label,
                r.tone == tone,
                th,
            )
            .rounded_full()
            .on_click(cx.listener(move |this, _, _, cx| this.start_rephrase(tone, cx)))
        };
        let tones = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(6.0))
            .children(FIRST_TONES.map(|tone| tone_chip(tone, tone_label(tone))))
            .child(
                chip("compose-rephrase-more", "⋯", r.more, th)
                    .rounded_full()
                    .tooltip(tip(tr!("compose-ai-more"), th))
                    .on_click(cx.listener(|this, _, window, cx| {
                        if let Some(r) = this.compose.as_mut().and_then(|c| c.rephrase.as_mut()) {
                            r.more = !r.more;
                            if r.more {
                                window.focus(&r.custom.focus_handle(cx), cx);
                            }
                        }
                        cx.notify();
                    })),
            );
        let more = r.more.then(|| {
            let focus = r.custom.focus_handle(cx);
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(6.0))
                .child(tone_chip(Tone::Longer, tone_label(Tone::Longer)))
                .child(
                    div()
                        .id("compose-rephrase-custom")
                        .flex_1()
                        .h(px(28.0))
                        .px(px(10.0))
                        .flex()
                        .items_center()
                        .rounded_full()
                        .border_1()
                        .border_color(rgba(if r.tone == Tone::Custom {
                            th.accent
                        } else {
                            th.divider
                        }))
                        .text_size(px(13.0))
                        .on_click(move |_, window, cx| window.focus(&focus, cx))
                        .child(div().flex_1().child(r.custom.clone())),
                )
        });
        let preview = div()
            .id("compose-rephrase-preview")
            .min_h(px(64.0))
            .max_h(px(220.0))
            .overflow_y_scroll()
            .p(px(10.0))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgba(th.divider))
            .bg(rgba(th.surface))
            .line_height(px(20.0))
            .child(match &r.state {
                State::Loading | State::Ask => placeholder(th, cx.reduce_motion()),
                State::Done(done) => div().child(done.text.clone()).into_any_element(),
                State::Failed(problem) => {
                    let (text, fix) = problem_text(problem, service);
                    let (label, fix) = match fix {
                        Fix::Retry => (tr!("compose-ai-try-again"), fix),
                        Fix::Settings(_) => (tr!("compose-ai-open-settings"), fix),
                    };
                    div()
                        .flex()
                        .flex_col()
                        .items_start()
                        .gap(px(8.0))
                        .child(div().text_color(rgba(th.text_dim)).child(text))
                        .child(
                            outlined_button("compose-rephrase-fix", label, th)
                                .h(px(30.0))
                                .px(px(14.0))
                                .on_click(cx.listener(move |this, _, window, cx| match fix {
                                    Fix::Retry => {
                                        let tone = this
                                            .compose
                                            .as_ref()
                                            .and_then(|c| c.rephrase.as_ref())
                                            .map_or(Tone::Clearer, |r| r.tone);
                                        this.start_rephrase(tone, cx);
                                    }
                                    Fix::Settings(section) => {
                                        this.close_rephrase(window, cx);
                                        this.open_settings_page(section, window, cx);
                                    }
                                })),
                        )
                        .into_any_element()
                }
            });
        let done = matches!(r.state, State::Done(_));
        let footer = match &r.state {
            State::Done(Rephrased {
                plan: kind,
                days_left,
                ..
            }) if kind == plan::TRIAL && *days_left > 0 => {
                tr!(
                    "compose-ai-trial-left",
                    service = service,
                    days = *days_left
                )
            }
            _ => service.to_owned(),
        };
        let tool = |id: &'static str, name: &'static str, label: String| {
            icon_button_colored(id, name, 18.0, th.text_dim, th)
                .size(px(36.0))
                .tooltip(tip(label, th))
                .when(!done, |d| d.opacity(0.4).cursor_default())
        };
        let actions = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .child(
                filled_button("compose-rephrase-replace", tr!("compose-ai-replace"), th)
                    .mr(px(4.0))
                    .when(!done, |d| d.opacity(0.5).cursor_default())
                    .on_click(
                        cx.listener(|this, _, window, cx| this.use_rephrased(false, window, cx)),
                    ),
            )
            .when(matches!(r.state, State::Loading), |d| {
                d.child(
                    outlined_button("compose-rephrase-stop", tr!("compose-ai-cancel"), th)
                        .on_click(
                            cx.listener(|this, _, window, cx| this.close_rephrase(window, cx)),
                        ),
                )
            })
            .when(!matches!(r.state, State::Loading), |d| {
                d.child(
                    tool("compose-rephrase-again", "refresh", tr!("compose-ai-again")).on_click(
                        cx.listener(|this, _, _, cx| {
                            let tone = this
                                .compose
                                .as_ref()
                                .and_then(|c| c.rephrase.as_ref())
                                .map_or(Tone::Clearer, |r| r.tone);
                            this.start_rephrase(tone, cx);
                        }),
                    ),
                )
                .child(
                    tool(
                        "compose-rephrase-below",
                        "insert-below",
                        tr!("compose-ai-below"),
                    )
                    .on_click(
                        cx.listener(|this, _, window, cx| this.use_rephrased(true, window, cx)),
                    ),
                )
                .child(
                    tool("compose-rephrase-copy", "copy", tr!("compose-ai-copy"))
                        .on_click(cx.listener(|this, _, _, cx| this.copy_rephrased(cx))),
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
            .child(tones)
            .children(more)
            .child(preview)
            .child(actions)
            .into_any_element()
    }

    /// The longer AI suggestions for the open message, when switched on.
    pub(super) fn ai_complete(&self, cx: &Context<Self>) -> Option<Rc<dyn Complete>> {
        let ai = &self.config.ai;
        (ai.autocomplete && ai.source != AiSource::Off).then(|| {
            Rc::new(AiComplete {
                window: cx.entity().downgrade(),
            }) as Rc<dyn Complete>
        })
    }
}

/// Finishes the sentence being written through the daemon, unless the
/// message is encrypted or the switch went off since.
struct AiComplete {
    window: WeakEntity<MailWindow>,
}

impl Complete for AiComplete {
    fn complete(&self, before: String, cx: &mut App) -> Task<Option<String>> {
        let Some(window) = self.window.upgrade() else {
            return Task::ready(None);
        };
        let window = window.read(cx);
        let ai = &window.config.ai;
        let Some(c) = &window.compose else {
            return Task::ready(None);
        };
        if c.sealing.encrypt || !ai.autocomplete || ai.source == AiSource::Off {
            return Task::ready(None);
        }
        let answered = if ai.autocomplete_answered {
            c.answered.clone()
        } else {
            String::new()
        };
        let connection = window.daemon.clone();
        cx.background_executor().spawn(async move {
            let connection = match connection {
                Some(connection) => connection,
                None => daemon::connect().await.ok()?,
            };
            daemon::ai_complete(&connection, &before, &answered)
                .await
                .ok()
                .filter(|text| !text.trim().is_empty())
        })
    }
}

fn tone_label(tone: Tone) -> String {
    match tone {
        Tone::Clearer => tr!("compose-ai-tone-clearer"),
        Tone::Shorter => tr!("compose-ai-tone-shorter"),
        Tone::Friendlier => tr!("compose-ai-tone-friendlier"),
        Tone::Formal => tr!("compose-ai-tone-formal"),
        Tone::Grammar => tr!("compose-ai-tone-grammar"),
        Tone::Longer => tr!("compose-ai-tone-longer"),
        Tone::Custom => tr!("compose-ai-custom"),
    }
}

/// What to tell the user about `problem`, and what the button does.
fn problem_text(problem: &str, service: &str) -> (String, Fix) {
    match problem {
        problem::SIGN_IN => (
            tr!("compose-ai-sign-in"),
            Fix::Settings(Section::Subscriptions),
        ),
        problem::PAY => (tr!("compose-ai-pay"), Fix::Settings(Section::Signatures)),
        problem::TOO_MANY => (tr!("compose-ai-too-many"), Fix::Retry),
        problem::NO_KEY => (
            tr!("compose-ai-no-key", service = service),
            Fix::Settings(Section::Signatures),
        ),
        problem::BAD_KEY => (
            tr!("compose-ai-bad-key", service = service),
            Fix::Settings(Section::Signatures),
        ),
        problem::OFF => (tr!("compose-ai-off"), Fix::Settings(Section::Signatures)),
        _ => (tr!("compose-ai-failed", service = service), Fix::Retry),
    }
}

/// Grey lines breathing while the text is rewritten.
fn placeholder(th: &Theme, reduce: bool) -> AnyElement {
    use gpui::{Animation, AnimationExt};
    let line = |width: f32| {
        div()
            .h(px(10.0))
            .my(px(5.0))
            .w(gpui::relative(width))
            .rounded_full()
            .bg(rgba(fade(th.text_faint, 0.25)))
    };
    let lines = div()
        .flex()
        .flex_col()
        .child(line(0.92))
        .child(line(0.78))
        .child(line(0.55));
    if reduce {
        return lines.into_any_element();
    }
    lines
        .with_animation(
            "compose-rephrase-wait",
            Animation::new(std::time::Duration::from_millis(1800)).repeat(),
            |el, t| {
                let wave = 0.5 - 0.5 * (t * std::f32::consts::TAU).cos();
                el.opacity(0.55 + 0.45 * wave)
            },
        )
        .into_any_element()
}
