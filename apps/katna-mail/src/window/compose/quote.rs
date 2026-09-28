// SPDX-License-Identifier: GPL-3.0-or-later

//! The quoted message under a reply, kept out of the text behind a "..."
//! button as in Gmail. The button shows it; once shown, a line across the
//! top of the quote with the button at its left end hides it again, and
//! the text grows and shrinks smoothly. The x on the button's corner takes
//! the quote out of the reply, and Undo (or Ctrl+Z) puts it back.

use std::time::{Duration, Instant};

use gpui::{AnyElement, Context, Focusable, Window, canvas, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::rich::Block;
use katna_ui::unpx;

use super::super::MailWindow;
use crate::daemon::Command;
use crate::theme::Theme;
use crate::widgets::tip;

/// How long the quote takes to open or close.
const GLIDE: Duration = Duration::from_millis(220);
/// The height of the "..." button.
const BUTTON_HEIGHT: f32 = 16.0;

/// Where the text and its shown quote were last drawn, from the top of the
/// text.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(in crate::window) struct QuoteView {
    /// The height of the whole text.
    height: f32,
    /// The top of the quote's first line, and the middle of the line the
    /// button sits on.
    quote_top: Option<f32>,
    line_y: Option<f32>,
}

/// The text growing as the quote opens, or shrinking as it closes.
pub(in crate::window) struct Glide {
    start: Instant,
    /// The text's height when it began.
    from: f32,
    /// Closing: the height it shrinks to, before the quote is taken out.
    to: Option<f32>,
    _tick: gpui::Task<()>,
}

impl Glide {
    /// How far along, eased: 0 at the start, 1 when done.
    fn eased(&self) -> f32 {
        let t = (self.start.elapsed().as_secs_f32() / GLIDE.as_secs_f32()).min(1.0);
        1.0 - (1.0 - t).powi(3)
    }
}

/// Where the quoted message of a reply is.
#[derive(Default)]
pub(in crate::window) enum Quote {
    /// Not a reply, or one without a quote.
    #[default]
    None,
    /// Behind the "..." button; it is still sent.
    Hidden(Vec<Block>),
    /// At the end of the text: from its first block on, `len` blocks long
    /// when it was shown.
    Shown { first: Block, len: usize },
    /// Taken out with the x, by the edit that Undo takes back at `depth`.
    Removed { tail: Vec<Block>, depth: usize },
}

impl Quote {
    pub(super) fn hidden_from(blocks: Option<Vec<Block>>) -> Self {
        blocks.map_or(Self::None, Self::Hidden)
    }

    pub(super) fn is_hidden(&self) -> bool {
        matches!(self, Self::Hidden(_))
    }

    /// The quote while it is behind the button, which the message still
    /// carries.
    pub(super) fn hidden(&self) -> &[Block] {
        match self {
            Self::Hidden(blocks) => blocks,
            _ => &[],
        }
    }

    /// Takes the quote out of `blocks` when it shows at their end, and
    /// forgets it wherever it is.
    pub(super) fn take_from(&mut self, blocks: &mut Vec<Block>) {
        if let Self::Shown { first, len } = &*self
            && let Some(at) = quote_start(blocks, first, *len)
        {
            blocks.truncate(at);
        }
        *self = Self::None;
    }
}

/// Where a quote shown at the end of `blocks` starts: its first block,
/// looked for from the end so edits in the reply above don't matter, else
/// its length from the end.
fn quote_start(blocks: &[Block], first: &Block, len: usize) -> Option<usize> {
    blocks
        .iter()
        .rposition(|b| b == first)
        .filter(|at| *at > 0)
        .or_else(|| {
            let at = blocks.len().saturating_sub(len);
            (at > 0).then_some(at)
        })
}

impl MailWindow {
    /// The reply's text; with its quote shown, a line across the top of
    /// the quote with the "..." button at its left end. While the quote
    /// opens or closes the text is cut to a height that glides.
    pub(super) fn render_body_and_quote(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let view = compose.quote_view.get();
        let shown = matches!(compose.quote, Quote::Shown { .. });
        let first = match &compose.quote {
            Quote::Shown { first, len } => {
                quote_start(&compose.body.read(cx).doc().blocks, first, *len)
            }
            _ => None,
        };
        let height = compose.quote_glide.as_ref().map(|glide| {
            let to = glide.to.unwrap_or(view.height);
            glide.from + (to - glide.from) * glide.eased()
        });
        let (cell, editor, this) = (
            compose.quote_view.clone(),
            compose.body.clone(),
            cx.entity().downgrade(),
        );
        // Measures the text and where its quote starts, and draws again
        // when that moved.
        let measure = canvas(
            move |bounds, _, cx| {
                let top = bounds.top();
                let editor = editor.read(cx);
                let quote = first.and_then(|at| editor.block_bounds(at)).map(|b| {
                    let blank = editor
                        .doc()
                        .blocks
                        .get(first.unwrap_or(0))
                        .is_some_and(|b| matches!(b, Block::Para(p) if p.text.is_empty()));
                    let quote_top = unpx(b.top() - top);
                    let line_y = if blank {
                        quote_top + unpx(b.size.height) / 2.0
                    } else {
                        (quote_top - BUTTON_HEIGHT / 2.0).max(BUTTON_HEIGHT / 2.0)
                    };
                    (quote_top, line_y)
                });
                let now = QuoteView {
                    height: unpx(bounds.size.height),
                    quote_top: quote.map(|q| q.0),
                    line_y: quote.map(|q| q.1),
                };
                if now != cell.get() {
                    cell.set(now);
                    let this = this.clone();
                    cx.defer(move |cx| {
                        this.update(cx, |_, cx| cx.notify()).ok();
                    });
                }
            },
            |_, _, _, _| {},
        )
        .absolute()
        .top_0()
        .left_0()
        .size_full();
        let line = view
            .line_y
            .filter(|_| shown && compose.quote_glide.as_ref().is_none_or(|g| g.to.is_none()))
            .map(|y| {
                // The button's middle, under the x's room above it, on the
                // line.
                div()
                    .id("hide-trimmed-line")
                    .absolute()
                    .left_0()
                    .right_0()
                    .top(px(y - 15.0))
                    .h(px(23.0))
                    .flex()
                    .flex_row()
                    .items_end()
                    .cursor_pointer()
                    .tooltip(tip(tr!("compose-hide-trimmed"), th))
                    .on_click(cx.listener(|this, _, window, cx| {
                        cx.stop_propagation();
                        this.toggle_quote(window, cx);
                    }))
                    .child(self.trimmed_button(true, th, cx))
                    .child(
                        div()
                            .flex_1()
                            .h(px(BUTTON_HEIGHT))
                            .flex()
                            .items_center()
                            .child(div().w_full().h(px(1.0)).bg(rgba(th.divider))),
                    )
            });
        div()
            .relative()
            .flex_none()
            .when_some(height, |d, h| d.h(px(h.max(0.0))).overflow_hidden())
            .child(
                div()
                    .relative()
                    .flex_none()
                    .child(compose.body.clone())
                    .child(measure),
            )
            .children(line)
            .into_any_element()
    }

    /// The "..." button under a reply's text while its quote is hidden.
    pub(super) fn render_trimmed(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let compose = self.compose.as_ref()?;
        // Opening, the button stays under the text until the quote shows.
        let opening = compose.quote_glide.as_ref().is_some_and(|g| g.to.is_none());
        if !compose.quote.is_hidden() || opening {
            return None;
        }
        Some(
            div()
                .flex()
                .flex_row()
                .mt(px(5.0))
                .child(self.trimmed_button(false, th, cx))
                .into_any_element(),
        )
    }

    /// The "..." button, with an x on its corner while the pointer is
    /// over it; `shown` when the quote shows.
    fn trimmed_button(&self, shown: bool, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let dot = || div().size(px(4.0)).rounded_full().bg(rgba(th.text_dim));
        let remove = div()
            .id("remove-trimmed")
            .invisible()
            .group_hover("trimmed", |s| s.visible())
            .absolute()
            .top_0()
            .right_0()
            .size(px(16.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .bg(rgba(th.surface))
            .border_1()
            .border_color(rgba(th.divider))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .tooltip(tip(tr!("compose-remove-trimmed"), th))
            .on_click(cx.listener(|this, _, window, cx| {
                cx.stop_propagation();
                this.remove_quote(window, cx);
            }))
            .child(crate::widgets::icon("close", th.text_dim, 12.0));
        // The x sits on the button's corner, inside this so the pointer on
        // it still counts as over the button.
        div()
            .group("trimmed")
            .relative()
            .flex_none()
            .pt(px(7.0))
            .pr(px(9.0))
            .child(
                div()
                    .id("show-trimmed")
                    .w(px(30.0))
                    .h(px(BUTTON_HEIGHT))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .gap(px(3.0))
                    .rounded(px(8.0))
                    .bg(rgba(if shown { th.hover } else { th.chip }))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .tooltip(tip(
                        if shown {
                            tr!("compose-hide-trimmed")
                        } else {
                            tr!("compose-show-trimmed")
                        },
                        th,
                    ))
                    .on_click(cx.listener(|this, _, window, cx| {
                        cx.stop_propagation();
                        this.toggle_quote(window, cx);
                    }))
                    .child(dot())
                    .child(dot())
                    .child(dot()),
            )
            .child(remove)
            .into_any_element()
    }

    /// Shows the quoted message at the end of the reply's text, or hides
    /// it behind the button again; the text grows or shrinks smoothly.
    fn toggle_quote(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let reduce = cx.reduce_motion();
        let Some(compose) = &mut self.compose else {
            return;
        };
        if compose.quote_glide.is_some() {
            return;
        }
        let body = compose.body.clone();
        let view = compose.quote_view.get();
        match std::mem::take(&mut compose.quote) {
            Quote::Hidden(blocks) => {
                if let Some(first) = blocks.first().cloned() {
                    compose.quote = Quote::Shown {
                        first,
                        len: blocks.len(),
                    };
                    body.update(cx, |editor, cx| editor.append_blocks(blocks, cx));
                    if !reduce {
                        self.start_glide(view.height, None, cx);
                    }
                }
            }
            shown @ Quote::Shown { .. } => {
                compose.quote = shown;
                // Back to the reply, with the cursor in it.
                window.focus(&body.focus_handle(cx), cx);
                match view.quote_top.filter(|_| !reduce) {
                    Some(top) => self.start_glide(view.height, Some(top), cx),
                    None => self.hide_quote(cx),
                }
            }
            other => compose.quote = other,
        }
        cx.notify();
    }

    /// Glides the text's height from `from` to `to`, or to its new height
    /// when `None`; closing, the quote goes when the text is short enough.
    fn start_glide(&mut self, from: f32, to: Option<f32>, cx: &mut Context<Self>) {
        let tick = cx.spawn(async move |this, cx| {
            let start = Instant::now();
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(16))
                    .await;
                let done = start.elapsed() >= GLIDE;
                let alive = this
                    .update(cx, |this, cx| {
                        if done {
                            this.end_glide(cx);
                        }
                        cx.notify();
                    })
                    .is_ok();
                if done || !alive {
                    break;
                }
            }
        });
        if let Some(compose) = &mut self.compose {
            compose.quote_glide = Some(Glide {
                start: Instant::now(),
                from,
                to,
                _tick: tick,
            });
        }
    }

    fn end_glide(&mut self, cx: &mut Context<Self>) {
        let Some(glide) = self.compose.as_mut().and_then(|c| c.quote_glide.take()) else {
            return;
        };
        if glide.to.is_some() {
            self.hide_quote(cx);
        }
    }

    /// Takes the shown quote out of the text, behind the button again.
    fn hide_quote(&mut self, cx: &mut Context<Self>) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        let body = compose.body.clone();
        if let Quote::Shown { first, len } = std::mem::take(&mut compose.quote) {
            let at = quote_start(&body.read(cx).doc().blocks, &first, len);
            compose.quote = match at {
                Some(at) => Quote::Hidden(body.update(cx, |e, cx| e.take_tail(at, cx))),
                None => Quote::None,
            };
        }
        // The reply got shorter: back up to it if its cursor went out of
        // sight above.
        if compose.mode == super::Mode::Inline {
            self.scroll_back_to_inline_reply(cx);
        }
        cx.notify();
    }

    /// Takes the quoted message out of the reply; Undo puts it back.
    fn remove_quote(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        let body = compose.body.clone();
        // A hidden quote goes into the text first, as if it had always
        // been there, so taking it out is one edit Undo takes back.
        if let Quote::Hidden(blocks) = std::mem::take(&mut compose.quote) {
            if let Some(first) = blocks.first().cloned() {
                compose.quote = Quote::Shown {
                    first,
                    len: blocks.len(),
                };
            }
            body.update(cx, |editor, cx| editor.append_blocks(blocks, cx));
        }
        let Some(compose) = &mut self.compose else {
            return;
        };
        let Quote::Shown { first, len } = std::mem::take(&mut compose.quote) else {
            return;
        };
        let Some(at) = quote_start(&body.read(cx).doc().blocks, &first, len) else {
            return;
        };
        let tail = body.read(cx).doc().blocks[at..].to_vec();
        body.update(cx, |editor, cx| editor.remove_tail(at, cx));
        let depth = body.read(cx).undo_depth();
        if let Some(compose) = &mut self.compose {
            compose.quote = Quote::Removed { tail, depth };
        }
        window.focus(&body.focus_handle(cx), cx);
        self.show_snackbar(
            tr!("compose-trimmed-removed"),
            Some(Command::RestoreQuote),
            cx,
        );
        cx.notify();
    }

    /// Undo on the snackbar, or Ctrl+Z outside the text: puts the quote
    /// back, while taking it out is still the last change to the text.
    pub(in crate::window) fn restore_quote(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let Some(compose) = &self.compose else {
            return;
        };
        let Quote::Removed { depth, .. } = &compose.quote else {
            return;
        };
        let body = compose.body.clone();
        if body.read(cx).undo_depth() == *depth {
            body.update(cx, |editor, cx| editor.undo(cx));
        }
    }

    /// After a change to the text: a quote taken out and put back with
    /// Ctrl+Z gets its button again.
    pub(super) fn quote_changed(&mut self, cx: &mut Context<Self>) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        let Quote::Removed { tail, .. } = &compose.quote else {
            return;
        };
        let blocks = &compose.body.read(cx).doc().blocks;
        if blocks.len() > tail.len() && blocks.ends_with(tail) {
            compose.quote = Quote::Shown {
                first: tail[0].clone(),
                len: tail.len(),
            };
            cx.notify();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katna_ui::rich::Para;

    fn para(text: &str) -> Block {
        Block::Para(Para::plain(text))
    }

    #[test]
    fn finds_the_quote_by_its_first_block() {
        let blocks = [
            para("Hi"),
            para(""),
            para("On Mon, Kay wrote:"),
            para("> x"),
        ];
        let first = para("On Mon, Kay wrote:");
        assert_eq!(quote_start(&blocks, &first, 2), Some(2));
        // Gone: its length from the end.
        assert_eq!(quote_start(&blocks, &para("nope"), 1), Some(3));
        // Never the whole text.
        assert_eq!(quote_start(&blocks[..1], &para("nope"), 1), None);
    }
}
