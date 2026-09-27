// SPDX-License-Identifier: GPL-3.0-or-later

//! Reply, Reply all and Forward at the foot of a conversation.
//!
//! The row never wraps. As the pane narrows the buttons drop their words
//! one at a time, Reply all first, then Reply, then Forward, and keep
//! their icons (with the word as a tooltip). On a phone the three share
//! the width equally, as in Gmail's app, and fold together.

use gpui::{
    AnyElement, Context, FontWeight, SharedString, TextRun, Window, black, div, prelude::*, px,
};
use katna_ui::motion::{self, Spring, lerp};

use super::MailWindow;
use super::compose::Kind;
use crate::data::EntryKey;
use crate::theme::Theme;
use crate::widgets::{pill_button, tip};

/// The buttons: id, icon, word and what they write.
const BUTTONS: [(&str, &str, &str, Kind); 3] = [
    ("reply", "reply", "Reply", Kind::Reply),
    ("reply-all", "reply-all", "Reply all", Kind::ReplyAll),
    ("forward", "forward", "Forward", Kind::Forward),
];
/// Which buttons lose their words first.
const FOLD_ORDER: [usize; 3] = [1, 0, 2];

const GAP: f32 = 12.0;
const PHONE_GAP: f32 = 8.0;
/// A button's width without its word: border, padding, icon and gap.
const FRAME: f32 = 2.0 + 16.0 + 20.0 + 8.0 + 22.0;
/// A button with the icon alone.
const ICON_ONLY: f32 = 2.0 + 17.0 + 20.0 + 17.0;
/// On a phone a button's content is centered in its share; this is the
/// least room around the icon and word it needs.
const PHONE_FRAME: f32 = 2.0 + 8.0 + 20.0 + 8.0 + 8.0;
/// The row's padding on the right.
const RIGHT: f32 = 24.0;
const PHONE_RIGHT: f32 = 16.0;
/// A conversation pane narrower than this lays out as on a phone: the
/// text starts at the sender picture's edge and the reply row shares its
/// width equally.
const COMPACT_BELOW: f32 = 420.0;

pub(super) struct ReplyRow {
    /// The width of each button's word in the UI font.
    words: [f32; 3],
    /// How much of each word shows: 1 = all of it, 0 = the icon alone.
    shown: [Spring; 3],
    /// The conversation the row was last laid out for: another one takes
    /// its row without motion.
    key: Option<EntryKey>,
    /// The conversation pane's width.
    width: f32,
    /// 1 when the pane is narrow enough to lay out as on a phone.
    compact: Spring,
}

impl ReplyRow {
    pub(super) fn new() -> Self {
        Self {
            words: [0.0; 3],
            shown: std::array::from_fn(|_| Spring::new(motion::SMOOTH, 1.0)),
            key: None,
            width: f32::MAX,
            compact: Spring::new(motion::SMOOTH, 0.0),
        }
    }
}

/// Which buttons show only their icons for the words to fit in `room`.
fn folds(words: [f32; 3], room: f32, phone: bool) -> [bool; 3] {
    // On a phone the buttons share the width equally, and all three keep
    // their words or all three drop them.
    if phone {
        let share = (room - 2.0 * PHONE_GAP) / 3.0;
        let fit = words.iter().all(|word| PHONE_FRAME + word <= share);
        return [!fit; 3];
    }
    let fits = |folded: &[bool; 3]| {
        let buttons: f32 = (0..3)
            .map(|ix| {
                if folded[ix] {
                    ICON_ONLY
                } else {
                    FRAME + words[ix]
                }
            })
            .sum();
        buttons + 2.0 * GAP <= room
    };
    let mut folded = [false; 3];
    for ix in FOLD_ORDER {
        if fits(&folded) {
            break;
        }
        folded[ix] = true;
    }
    folded
}

impl MailWindow {
    /// Lays out the reply row for a conversation pane `width` wide.
    pub(super) fn update_reply_row(&mut self, width: f32, window: &Window, reduce: bool) {
        self.reply_row.width = width;
        let Some(key) = self.reader.as_ref().map(|r| r.key) else {
            self.reply_row.key = None;
            return;
        };
        let fresh = self.reply_row.key != Some(key);
        let compact = &mut self.reply_row.compact;
        compact.set(if width < COMPACT_BELOW { 1.0 } else { 0.0 });
        if fresh {
            compact.snap(compact.target());
        }
        compact.tick(window, reduce);
        let mut font = window.text_style().font();
        if let Some(family) = &self.font {
            font.family = family.clone();
        }
        font.weight = FontWeight::MEDIUM;
        let system = window.text_system();
        for (width, (_, _, word, _)) in self.reply_row.words.iter_mut().zip(BUTTONS) {
            let run = TextRun {
                len: word.len(),
                font: font.clone(),
                color: black(),
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            *width = f32::from(
                system
                    .shape_line(SharedString::new_static(word), px(14.0), &[run], None)
                    .width,
            );
        }
        let phone = self.layout.shape.is_phone();
        let shared = self.reply_row_shared();
        let right = if shared { PHONE_RIGHT } else { RIGHT };
        // A little slack for the card's edge and rounding; a phone's
        // conversation has no card around it.
        let slack = if phone { 0.0 } else { 4.0 };
        let room = width - self.reader_indent() - right - slack;
        let folded = folds(self.reply_row.words, room, shared);
        self.reply_row.key = Some(key);
        for (spring, folded) in self.reply_row.shown.iter_mut().zip(folded) {
            let target = if folded { 0.0 } else { 1.0 };
            if fresh {
                spring.snap(target);
            } else {
                spring.set(target);
            }
            spring.tick(window, reduce);
        }
    }

    /// The conversation pane's width, as last laid out.
    pub(super) fn reader_width(&self) -> f32 {
        self.reply_row.width
    }

    /// How far the conversation lays out as on a phone: on a phone, and
    /// in a narrow pane. 0 to 1.
    pub(super) fn reader_compact(&self) -> f32 {
        self.layout
            .shape
            .phone
            .max(self.reply_row.compact.value())
            .clamp(0.0, 1.0)
    }

    /// How far a conversation's text is indented from the card's edge.
    pub(super) fn reader_indent(&self) -> f32 {
        lerp(72.0, 16.0, self.reader_compact())
    }

    /// The reply row's buttons share its width equally, as on a phone.
    fn reply_row_shared(&self) -> bool {
        self.layout.shape.is_phone() || self.reply_row.compact.target() > 0.5
    }

    /// The row itself.
    pub(super) fn render_reply_row(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let shared = self.reply_row_shared();
        div()
            .flex()
            .flex_row()
            .gap(px(if shared { PHONE_GAP } else { GAP }))
            .pl(px(self.reader_indent()))
            .pr(px(lerp(RIGHT, PHONE_RIGHT, self.reader_compact())))
            .py(px(14.0))
            .children(
                BUTTONS
                    .into_iter()
                    .enumerate()
                    .map(|(ix, (id, name, word, kind))| {
                        let shown = self.reply_row.shown[ix].value();
                        pill_button(id, name, word, self.reply_row.words[ix], shown, th)
                            .flex_none()
                            .when(shared, |d| {
                                d.flex_1()
                                    .min_w_0()
                                    .justify_center()
                                    .pl(px(8.0))
                                    .pr(px(8.0))
                            })
                            .when(shown < 0.5, |d| d.tooltip(tip(word, th)))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.open_compose(kind, None, window, cx)
                            }))
                            .into_any_element()
                    }),
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORDS: [f32; 3] = [36.0, 60.0, 54.0];

    #[test]
    fn words_fold_one_at_a_time() {
        // All three words: 3 × 68 + 150 + 24 = 378.
        assert_eq!(folds(WORDS, 400.0, false), [false, false, false]);
        assert_eq!(folds(WORDS, 370.0, false), [false, true, false]);
        // Reply all folded: 56 + 104 + 122 + 24 = 306.
        assert_eq!(folds(WORDS, 300.0, false), [true, true, false]);
        // Reply folded too: 56 + 56 + 122 + 24 = 258.
        assert_eq!(folds(WORDS, 250.0, false), [true, true, true]);
        assert_eq!(folds(WORDS, 100.0, false), [true, true, true]);
    }

    #[test]
    fn phone_buttons_share_the_width() {
        // Each share must hold 46 + the word: Reply all needs 106.
        assert_eq!(folds(WORDS, 3.0 * 106.0 + 16.0, true), [false; 3]);
        assert_eq!(folds(WORDS, 3.0 * 104.0 + 16.0, true), [true; 3]);
    }
}
