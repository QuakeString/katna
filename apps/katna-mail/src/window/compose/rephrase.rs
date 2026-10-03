// SPDX-License-Identifier: GPL-3.0-or-later

//! Writing help from AI in the compose window (`docs/ARCHITECTURE.md`
//! §16.5). Selecting text shows a small sparkle by its end (or Ctrl+J);
//! it opens a card that rewrites the selection in a tone through the
//! daemon, and Replace puts the new text in as one step Undo takes back.
//! Encrypted mail asks first. [`AiComplete`] finishes the sentence being
//! written, in grey like the phrase suggestions, when that is switched on.

use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    Anchor, AnyElement, App, Bounds, ClipboardItem, Context, DragMoveEvent, Entity, Focusable,
    MouseButton, MouseDownEvent, Pixels, Point, Subscription, Task, WeakEntity, Window, anchored,
    canvas, deferred, div, point, prelude::*, rgba,
};
use katna_ai::Tone;
use katna_ai::provider::{self, OTHER};
use katna_ai::wire::{plan, problem};
use katna_core::config::AiSource;
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::rich::Complete;
use katna_ui::{InputEvent, TextInput};

pub(in crate::window) mod subject;
mod write;

use super::super::MailWindow;
use super::super::settings_page::Section;
use crate::daemon::{self, Command, Rephrased};
use crate::theme::{Theme, fade};
use crate::widgets::choice_chip;
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
    /// The box the card opens over (the chat's reply box with its
    /// formatting bar, or the compose window's bottom row), measured as
    /// it draws; the card waits for it.
    room: Rc<Cell<Option<Bounds<Pixels>>>>,
    /// How far the user dragged the card from where it opened, and where
    /// a drag started: the pointer and that offset then.
    moved: Point<Pixels>,
    grab: Option<(Point<Pixels>, Point<Pixels>)>,
    /// The selected text, as sent.
    text: String,
    tone: Tone,
    state: State,
    /// The second row (Longer, and the user's own instruction) shows.
    more: bool,
    custom: Entity<TextInput>,
    _custom: Subscription,
    _task: Option<Task<()>>,
    /// Writing a first draft rather than rephrasing ([`write`]).
    write: Option<write::Write>,
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

/// Dragging the Rephrase card by its grip.
#[derive(Clone, Copy)]
struct RephraseDrag;

impl gpui::Render for RephraseDrag {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

/// What the button under a problem does.
#[derive(Clone, Copy)]
pub(in crate::window) enum Fix {
    Retry,
    Settings(Section),
}

/// The sparkle by selected text: its size.
const REPHRASE_BUTTON: f32 = 30.0;

/// Where the sparkle goes for a selection ending at `end`: under the
/// end, else beside it, always inside `shown`, the part of the text box in
/// sight.
fn rephrase_button_at(end: Bounds<Pixels>, shown: Bounds<Pixels>) -> Point<Pixels> {
    let size = px(REPHRASE_BUTTON);
    let gap = px(4.0);
    let below = end.bottom_right() + point(gap, gap);
    let at = if below.y + size + gap <= shown.bottom() {
        below
    } else {
        point(
            end.right() + gap,
            end.origin.y + (end.size.height - size) / 2.0,
        )
    };
    if shown.size.width <= size || shown.size.height <= size {
        return at;
    }
    point(
        at.x.clamp(shown.left() + gap, shown.right() - size - gap),
        at.y.clamp(shown.top() + gap, shown.bottom() - size - gap),
    )
}

impl MailWindow {
    /// Whether writing help may be used for the open message: it is on,
    /// and the message is not encrypted unless Settings allows that.
    pub(super) fn ai_allowed(&self) -> bool {
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
        let shown = c.body.read(cx).shown_bounds();
        // A solid round base under the usual icon button and its hover.
        let button = div()
            .occlude()
            .size(px(REPHRASE_BUTTON))
            .rounded_full()
            .bg(rgba(th.menu))
            .shadow(elevation(th, 2.0))
            .child(
                icon_button_colored("compose-rephrase", "sparkle", 18.0, th.accent, th)
                    .size(px(REPHRASE_BUTTON))
                    .tooltip(tip(tr!("compose-ai-rephrase-tip"), th))
                    // The text keeps its selection.
                    .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .on_click(cx.listener(|this, _, window, cx| this.open_rephrase(window, cx))),
            );
        Some(
            deferred(
                anchored()
                    .position(rephrase_button_at(at, shown))
                    .snap_to_window_with_margin(px(8.0))
                    .child(button),
            )
            .with_priority(1)
            .into_any_element(),
        )
    }

    /// The sparkle and Ctrl+J: the card for the selection, or for all the
    /// user wrote when nothing is selected, or for writing a reply when
    /// nothing is written yet; closes an open card.
    pub(super) fn toggle_rephrase(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.compose.as_ref().is_some_and(|c| c.rephrase.is_some()) {
            self.close_rephrase(window, cx);
            return;
        }
        if !self.ai_allowed() {
            return;
        }
        if let Some(kind) = self.write_kind(cx) {
            self.open_write(kind, window, cx);
            return;
        }
        if let Some(c) = &self.compose
            && !c.body.read(cx).has_selection()
        {
            c.body.update(cx, |editor, cx| editor.select_own_text(cx));
        }
        self.open_rephrase(window, cx);
    }

    /// How the sparkle in the tools looks now: Write reply (or note)
    /// while a reply or forward is empty, else Rephrase, faded while there
    /// is nothing to rephrase. Its icon, tooltip and whether it does
    /// anything.
    pub(super) fn sparkle_look(&self, cx: &App) -> (&'static str, String, bool) {
        match self.write_kind(cx) {
            Some(katna_ai::draft::DraftKind::Forward) => {
                ("pen-sparkle", tr!("compose-ai-write-note-tip"), true)
            }
            Some(_) => ("pen-sparkle", tr!("compose-ai-write-reply-tip"), true),
            None if self.can_rephrase(cx) => ("sparkle", tr!("compose-ai-rephrase-tip"), true),
            None => ("sparkle", tr!("compose-ai-rephrase-empty-tip"), false),
        }
    }

    /// Whether the sparkle can rephrase something in the open message.
    pub(super) fn can_rephrase(&self, cx: &App) -> bool {
        self.ai_allowed()
            && self
                .compose
                .as_ref()
                .is_some_and(|c| c.body.read(cx).own_text_end().is_some())
    }

    fn open_rephrase(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some((text, _)) = self.rephrase_selection(cx) else {
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
            room: Rc::default(),
            moved: Point::default(),
            grab: None,
            text,
            tone: Tone::Clearer,
            state: if ask { State::Ask } else { State::Loading },
            more: false,
            custom,
            _custom: subscription,
            _task: None,
            write: None,
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
        if let Some(r) = &mut c.rephrase
            && r.write.is_some()
        {
            r.state = State::Loading;
            self.start_ideas(cx);
            return;
        }
        let tone = c.rephrase.as_ref().map_or(Tone::Clearer, |r| r.tone);
        self.start_rephrase(tone, cx);
    }

    /// Who answers: Katna AI, or the user's own service by name.
    pub(in crate::window) fn ai_service_name(&self) -> String {
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
        let body = match (&r.state, &r.write) {
            (State::Ask, _) => self.render_rephrase_ask(&service, th, cx),
            (_, Some(w)) => self.render_write_body(r, w, &service, th, cx),
            _ => self.render_rephrase_body(r, &service, th, cx),
        };
        // The card is moved by the grip along its top.
        let grip = div()
            .id("compose-rephrase-grip")
            .h(px(12.0))
            .mt(px(-6.0))
            .mb(px(-4.0))
            .flex()
            .items_center()
            .justify_center()
            .cursor_grab()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    if let Some(r) = this.compose.as_mut().and_then(|c| c.rephrase.as_mut()) {
                        r.grab = Some((event.position, r.moved));
                    }
                    cx.stop_propagation();
                }),
            )
            .on_drag(RephraseDrag, |drag, _, _, cx| cx.new(|_| *drag))
            .child(
                div()
                    .w(px(36.0))
                    .h(px(4.0))
                    .rounded_full()
                    .bg(rgba(fade(th.text_faint, 0.45))),
            );
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
                .on_drag_move(
                    cx.listener(|this, event: &DragMoveEvent<RephraseDrag>, _, cx| {
                        if let Some(r) = this.compose.as_mut().and_then(|c| c.rephrase.as_mut())
                            && let Some((from, start)) = r.grab
                        {
                            r.moved = start + (event.event.position - from);
                            cx.notify();
                        }
                    }),
                )
                .on_drop(cx.listener(|this, _: &RephraseDrag, _, cx| {
                    if let Some(r) = this.compose.as_mut().and_then(|c| c.rephrase.as_mut()) {
                        r.grab = None;
                    }
                    cx.notify();
                }))
                .on_mouse_down_out(
                    cx.listener(|this, _, window, cx| this.close_rephrase(window, cx)),
                ),
            th,
            12.0,
            3.0,
        )
        .child(grip)
        .child(body);
        // Measures the box the card opens over: it is this element's
        // parent.
        let room = r.room.clone();
        let measure = canvas(
            move |bounds, window, _| {
                if room.get() != Some(bounds) {
                    room.set(Some(bounds));
                    window.request_animation_frame();
                }
            },
            |_, _, _, _| {},
        )
        .absolute()
        .top_0()
        .left_0()
        .size_full();
        // Over the box, centred on it, just above the formatting bar (a
        // compose window's floats over its bottom row); where it was
        // dragged to after that.
        let place = r.room.get().map(|room| {
            let lift = match self.compose.as_ref() {
                // The chat's reply box holds its formatting bar.
                Some(c)
                    if c.format_bar && !(c.mode == super::Mode::Inline && self.chat_shown()) =>
                {
                    super::tools::FORMAT_BAR_COVER
                }
                _ => 8.0,
            };
            point(
                room.center().x - px(CARD_WIDTH / 2.0),
                room.top() - px(lift),
            ) + r.moved
        });
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .child(measure)
                .children(place.map(|at| {
                    deferred(
                        anchored()
                            .anchor(Anchor::BottomLeft)
                            .position(at)
                            .snap_to_window_with_margin(px(8.0))
                            .child(card),
                    )
                    .with_priority(2)
                }))
                .into_any_element(),
        )
    }

    /// Encrypted mail: what rephrasing sends, and a choice.
    fn render_rephrase_ask(&self, service: &str, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let writing = self
            .compose
            .as_ref()
            .and_then(|c| c.rephrase.as_ref())
            .is_some_and(|r| r.write.is_some());
        let (text, go) = if writing {
            (
                tr!("compose-ai-write-encrypted", service = service),
                tr!("compose-ai-write-anyway"),
            )
        } else {
            (
                tr!("compose-ai-encrypted", service = service),
                tr!("compose-ai-rephrase"),
            )
        };
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
                            .child(text),
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
                        filled_button("compose-rephrase-anyway", go, th)
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
            choice_chip(
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
                choice_chip("compose-rephrase-more", "⋯", r.more, th)
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
            .border_color(rgba(th.outline))
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
pub(in crate::window) fn problem_text(problem: &str, service: &str) -> (String, Fix) {
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
pub(in crate::window) fn placeholder(th: &Theme, reduce: bool) -> AnyElement {
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
    pulsing("compose-rephrase-wait", lines, reduce)
}

/// Pills standing in for the reply ideas while they are asked, breathing
/// like [`placeholder`]: in the Write reply card and the summary card.
pub(in crate::window) fn idea_placeholder(th: &Theme, reduce: bool) -> AnyElement {
    pulsing(
        "compose-ideas-wait",
        div()
            .flex()
            .flex_row()
            .gap(px(6.0))
            .children([150.0, 120.0, 140.0].map(|width| {
                div()
                    .w(px(width))
                    .h(px(28.0))
                    .rounded_full()
                    .bg(rgba(fade(th.text_faint, 0.18)))
            })),
        reduce,
    )
}

/// `shapes` standing in for text still on its way, breathing slowly
/// (still with reduced motion): the waiting look of every AI card.
pub(in crate::window) fn pulsing(id: &'static str, shapes: gpui::Div, reduce: bool) -> AnyElement {
    use gpui::{Animation, AnimationExt};
    if reduce {
        return shapes.into_any_element();
    }
    shapes
        .with_animation(
            id,
            Animation::new(katna_ui::motion::time(std::time::Duration::from_millis(
                1800,
            )))
            .repeat(),
            |el, t| {
                let wave = 0.5 - 0.5 * (t * std::f32::consts::TAU).cos();
                el.opacity(0.55 + 0.45 * wave)
            },
        )
        .into_any_element()
}
