// SPDX-License-Identifier: GPL-3.0-or-later

//! The quoted message under a reply, kept out of the text behind a "..."
//! button as in Gmail. The button shows it and hides it again; the x on
//! its corner takes it out of the reply, and Undo (or Ctrl+Z) puts it back.

use gpui::{AnyElement, Context, Focusable, Window, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::rich::Block;

use super::super::MailWindow;
use crate::daemon::Command;
use crate::theme::Theme;
use crate::widgets::tip;

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

    fn has_button(&self) -> bool {
        matches!(self, Self::Hidden(_) | Self::Shown { .. })
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
    /// The "..." button under a reply's text, with an x on its corner
    /// while the pointer is over it.
    pub(super) fn render_trimmed(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let quote = &self.compose.as_ref()?.quote;
        if !quote.has_button() {
            return None;
        }
        let shown = matches!(quote, Quote::Shown { .. });
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
        Some(
            div()
                .flex()
                .flex_row()
                .child(
                    // The x sits on the button's corner, inside this so
                    // the pointer on it still counts as over the button.
                    div()
                        .group("trimmed")
                        .relative()
                        .mt(px(5.0))
                        .pt(px(7.0))
                        .pr(px(9.0))
                        .child(
                            div()
                                .id("show-trimmed")
                                .w(px(30.0))
                                .h(px(16.0))
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
                        .child(remove),
                )
                .into_any_element(),
        )
    }

    /// Shows the quoted message at the end of the reply's text, or hides
    /// it behind the button again.
    fn toggle_quote(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        let body = compose.body.clone();
        match std::mem::take(&mut compose.quote) {
            Quote::Hidden(blocks) => {
                if let Some(first) = blocks.first().cloned() {
                    compose.quote = Quote::Shown {
                        first,
                        len: blocks.len(),
                    };
                    body.update(cx, |editor, cx| editor.append_blocks(blocks, cx));
                }
            }
            Quote::Shown { first, len } => {
                let at = quote_start(&body.read(cx).doc().blocks, &first, len);
                compose.quote = match at {
                    Some(at) => Quote::Hidden(body.update(cx, |e, cx| e.take_tail(at, cx))),
                    None => Quote::None,
                };
                // Back up to the reply, which got shorter, with the cursor
                // in it.
                window.focus(&body.focus_handle(cx), cx);
                if compose.mode == super::Mode::Inline {
                    self.scroll_back_to_inline_reply(cx);
                }
            }
            other => compose.quote = other,
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
