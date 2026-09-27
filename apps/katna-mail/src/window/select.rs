// SPDX-License-Identifier: GPL-3.0-or-later

//! Selecting and copying the text of the open conversation, as in a
//! browser: drag to select, double-click for a word, triple-click for a
//! paragraph, Shift+click to extend, Ctrl+A and Ctrl+C once the text has
//! been clicked, and Copy on the right-click menu. Works for plain and
//! HTML mail alike.
//!
//! Every run of text a message body draws is a *piece*, keyed by the
//! message and its order in that message. Drawing a piece shows the part
//! of it that is selected; laying it out records where it went, so a
//! pointer position can be turned into a place in the text.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ops::Range;
use std::rc::Rc;

use gpui::{
    AnyElement, App, ClipboardItem, Context, DispatchPhase, Div, FocusHandle, HighlightStyle,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, SharedString,
    StyledText, TextLayout, Window, actions, anchored, canvas, deferred, div, prelude::*, px, rgba,
};

use super::MailWindow;
use crate::data::EntryKey;
use crate::theme::Theme;
use crate::widgets::{menu_item, raised};

actions!(katna_mail, [CopyText, SelectAllText]);

/// The key context of the conversation's text once it was clicked.
pub(super) const TEXT_CONTEXT: &str = "MessageText";

/// A run of text drawn by message `part` of the conversation, the
/// `piece`th in it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Key {
    part: usize,
    piece: usize,
}

/// A place in the conversation's text: a byte offset in a piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Spot {
    key: Key,
    offset: usize,
}

/// What a drag selects by: the click count that started it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unit {
    Char,
    Word,
    Paragraph,
}

struct Piece {
    text: SharedString,
    layout: TextLayout,
    /// The frame that laid it out; older pieces are no longer on screen.
    frame: u64,
}

pub(super) struct TextSelection {
    pub(super) focus: FocusHandle,
    drawn: Rc<RefCell<BTreeMap<Key, Piece>>>,
    frame: u64,
    /// The conversation the selection is in.
    conversation: Option<EntryKey>,
    /// Where the selection started and where it ends now.
    anchor: Option<Spot>,
    head: Option<Spot>,
    /// A drag is extending the selection.
    selecting: bool,
    unit: Unit,
    /// What the click that started the drag selected (a word or paragraph).
    origin: Option<(Spot, Spot)>,
    /// The right-click menu, where the pointer was.
    menu: Option<Point<Pixels>>,
}

impl TextSelection {
    pub(super) fn new(cx: &mut App) -> Self {
        Self {
            focus: cx.focus_handle(),
            drawn: Rc::default(),
            frame: 0,
            conversation: None,
            anchor: None,
            head: None,
            selecting: false,
            unit: Unit::Char,
            origin: None,
            menu: None,
        }
    }

    /// Starts drawing a frame of `conversation` (call before drawing its
    /// text); a different conversation drops the selection.
    pub(super) fn begin(&mut self, conversation: EntryKey) {
        self.frame += 1;
        if self.conversation != Some(conversation) {
            self.conversation = Some(conversation);
            self.clear();
            self.drawn.borrow_mut().clear();
        }
    }

    fn clear(&mut self) {
        self.anchor = None;
        self.head = None;
        self.selecting = false;
        self.origin = None;
        self.menu = None;
    }

    /// The selection, start first; `None` when nothing is selected.
    fn ordered(&self) -> Option<(Spot, Spot)> {
        ordered(self.anchor, self.head)
    }

    pub(super) fn is_empty(&self) -> bool {
        self.ordered().is_none()
    }

    /// The selected part of the piece `key`, `len` bytes long.
    fn range_in(&self, key: Key, len: usize) -> Option<Range<usize>> {
        range_in(self.ordered()?, key, len)
    }

    /// Pieces for message `part`, drawn with the selection in `color`.
    pub(super) fn pieces(&self, part: usize, th: &Theme) -> Pieces<'_> {
        Pieces {
            selection: self,
            part,
            next: 0,
            color: selection_color(th),
        }
    }

    /// The piece and place under `at`, or the nearest one; only in `part`
    /// when given.
    fn hit(&self, at: Point<Pixels>, part: Option<usize>) -> Option<Spot> {
        let drawn = self.drawn.borrow();
        let mut best: Option<(f32, Key)> = None;
        for (key, piece) in drawn.iter() {
            if piece.frame != self.frame || part.is_some_and(|p| p != key.part) {
                continue;
            }
            let b = piece.layout.bounds();
            let dx = f32::from((b.left() - at.x).max(at.x - b.right())).max(0.0);
            let dy = f32::from((b.top() - at.y).max(at.y - b.bottom())).max(0.0);
            // Beside a line counts as on it: rows beat columns.
            let distance = dy * 8.0 + dx;
            if best.is_none_or(|(d, _)| distance < d) {
                best = Some((distance, *key));
            }
        }
        let key = best?.1;
        let piece = &drawn[&key];
        let b = piece.layout.bounds();
        let offset = if at.y < b.top() {
            0
        } else if at.y > b.bottom() {
            piece.text.len()
        } else {
            let x = at.x.clamp(b.left(), b.right());
            match piece.layout.index_for_position(gpui::point(x, at.y)) {
                Ok(ix) | Err(ix) => ix,
            }
        };
        Some(Spot {
            key,
            offset: floor_char(&piece.text, offset),
        })
    }

    /// The word or paragraph around `spot`, or `spot` itself.
    fn expand(&self, spot: Spot, unit: Unit) -> (Spot, Spot) {
        let drawn = self.drawn.borrow();
        let Some(piece) = drawn.get(&spot.key) else {
            return (spot, spot);
        };
        let range = match unit {
            Unit::Char => return (spot, spot),
            Unit::Word => word_at(&piece.text, spot.offset),
            Unit::Paragraph => paragraph_at(&piece.text, spot.offset),
        };
        let at = |offset| Spot {
            key: spot.key,
            offset,
        };
        (at(range.start), at(range.end))
    }

    /// Moves the end of the selection to `spot`, by the drag's unit.
    fn extend_to(&mut self, spot: Spot) {
        match (self.unit, self.origin) {
            (Unit::Char, _) | (_, None) => self.head = Some(spot),
            (unit, Some((start, end))) => {
                let (from, to) = self.expand(spot, unit);
                if from < start {
                    self.anchor = Some(end);
                    self.head = Some(from);
                } else {
                    self.anchor = Some(start);
                    self.head = Some(to.max(end));
                }
            }
        }
    }

    /// Selects every piece on screen.
    fn select_all(&mut self) {
        let drawn = self.drawn.borrow();
        let mut on_screen = drawn.iter().filter(|(_, p)| p.frame == self.frame);
        let first = on_screen.next().map(|(key, _)| *key);
        let last = on_screen
            .next_back()
            .map(|(key, p)| (*key, p.text.len()))
            .or_else(|| first.map(|key| (key, drawn[&key].text.len())));
        drop(drawn);
        if let (Some(first), Some((last, len))) = (first, last) {
            self.anchor = Some(Spot {
                key: first,
                offset: 0,
            });
            self.head = Some(Spot {
                key: last,
                offset: len,
            });
        }
    }

    /// The selected text: pieces on their own lines, messages apart.
    pub(super) fn text(&self) -> String {
        let Some((start, end)) = self.ordered() else {
            return String::new();
        };
        let drawn = self.drawn.borrow();
        let mut out = String::new();
        let mut previous: Option<Key> = None;
        for (key, piece) in drawn.range(start.key..=end.key) {
            if piece.frame != self.frame {
                continue;
            }
            let Some(range) = self.range_in(*key, piece.text.len()) else {
                continue;
            };
            if let Some(previous) = previous {
                out.push_str(if previous.part == key.part {
                    "\n"
                } else {
                    "\n\n"
                });
            }
            out.push_str(&piece.text[range]);
            previous = Some(*key);
        }
        out
    }
}

/// Draws the text runs of one message so they can be selected.
pub(super) struct Pieces<'a> {
    selection: &'a TextSelection,
    part: usize,
    next: usize,
    color: gpui::Hsla,
}

impl Pieces<'_> {
    /// `text` with `highlights` (sorted, not overlapping) and the
    /// selection shown, and the element to put it in, which records its
    /// layout.
    pub(super) fn piece(
        &mut self,
        text: SharedString,
        highlights: Vec<(Range<usize>, HighlightStyle)>,
    ) -> (StyledText, Div) {
        let key = Key {
            part: self.part,
            piece: self.next,
        };
        self.next += 1;
        let highlights = match self.selection.range_in(key, text.len()) {
            Some(selected) => with_selection(highlights, selected, self.color),
            None => highlights,
        };
        let styled = StyledText::new(text.clone()).with_highlights(highlights);
        let layout = styled.layout().clone();
        let drawn = self.selection.drawn.clone();
        let frame = self.selection.frame;
        let holder = div().cursor_text().on_children_prepainted(move |_, _, _| {
            drawn.borrow_mut().insert(
                key,
                Piece {
                    text: text.clone(),
                    layout: layout.clone(),
                    frame,
                },
            );
        });
        (styled, holder)
    }
}

impl MailWindow {
    /// Makes the text in `body` (the text of message `part`) selectable
    /// with the mouse.
    pub(super) fn selectable_body(&self, part: usize, body: Div, cx: &mut Context<Self>) -> Div {
        body.on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                this.text_mouse_down(part, event, window, cx);
            }),
        )
        .on_mouse_down(
            MouseButton::Right,
            cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                cx.stop_propagation();
                window.focus(&this.text.focus, cx);
                this.text.menu = Some(event.position);
                cx.notify();
            }),
        )
    }

    fn text_mouse_down(
        &mut self,
        part: usize,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.stop_propagation();
        window.focus(&self.text.focus, cx);
        let text = &mut self.text;
        text.menu = None;
        let Some(spot) = text.hit(event.position, Some(part)) else {
            text.clear();
            cx.notify();
            return;
        };
        if event.modifiers.shift && text.anchor.is_some() {
            text.unit = Unit::Char;
            text.origin = None;
            text.head = Some(spot);
        } else {
            text.unit = match event.click_count {
                0 | 1 => Unit::Char,
                2 => Unit::Word,
                _ => Unit::Paragraph,
            };
            let (start, end) = text.expand(spot, text.unit);
            text.anchor = Some(start);
            text.head = Some(end);
            text.origin = Some((start, end));
        }
        text.selecting = true;
        cx.notify();
    }

    fn text_drag(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        if event.pressed_button != Some(MouseButton::Left) {
            self.text_mouse_up(cx);
            return;
        }
        let Some(spot) = self.text.hit(event.position, None) else {
            return;
        };
        let before = (self.text.anchor, self.text.head);
        self.text.extend_to(spot);
        if before != (self.text.anchor, self.text.head) {
            cx.notify();
        }
    }

    fn text_mouse_up(&mut self, cx: &mut Context<Self>) {
        self.text.selecting = false;
        // As on any Linux desktop, the selection can be pasted with the
        // middle button.
        let selected = self.text.text();
        if !selected.is_empty() {
            cx.write_to_primary(ClipboardItem::new_string(selected));
        }
    }

    fn copy_text(&mut self, _: &CopyText, _: &mut Window, cx: &mut Context<Self>) {
        let selected = self.text.text();
        if !selected.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(selected));
        }
    }

    fn select_all_text(&mut self, _: &SelectAllText, _: &mut Window, cx: &mut Context<Self>) {
        self.text.select_all();
        cx.notify();
    }

    /// The conversation's text: focusable so Ctrl+A and Ctrl+C reach it,
    /// and following the pointer while a drag selects.
    pub(super) fn text_area(&self, body: Div, cx: &mut Context<Self>) -> Div {
        let this = cx.entity().downgrade();
        let listen = canvas(
            |_, _, _| {},
            move |_, _, window, _| {
                window.on_mouse_event({
                    let this = this.clone();
                    move |event: &MouseMoveEvent, phase, _, cx| {
                        let Some(this) = this.upgrade() else {
                            return;
                        };
                        if phase == DispatchPhase::Bubble && this.read(cx).text.selecting {
                            this.update(cx, |this, cx| this.text_drag(event, cx));
                        }
                    }
                });
                window.on_mouse_event(move |event: &MouseUpEvent, phase, _, cx| {
                    let Some(this) = this.upgrade() else {
                        return;
                    };
                    if phase == DispatchPhase::Bubble
                        && event.button == MouseButton::Left
                        && this.read(cx).text.selecting
                    {
                        this.update(cx, |this, cx| this.text_mouse_up(cx));
                    }
                });
            },
        )
        .absolute()
        .size_0();
        body.key_context(TEXT_CONTEXT)
            .track_focus(&self.text.focus)
            .on_action(cx.listener(Self::copy_text))
            .on_action(cx.listener(Self::select_all_text))
            .child(listen)
    }

    /// The right-click menu of the conversation's text.
    pub(super) fn render_text_menu(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let at = self.text.menu?;
        let close = || {
            cx.listener(|this: &mut Self, _: &MouseDownEvent, _, cx| {
                this.text.menu = None;
                cx.notify();
            })
        };
        let has_selection = !self.text.is_empty();
        let list = div()
            .w(px(200.0))
            .py(px(8.0))
            .flex()
            .flex_col()
            .map(|d| raised(d, th, 8.0, 3.0))
            .text_size(px(14.0))
            .text_color(rgba(th.text))
            .child(
                menu_item("text-copy", "Copy", th)
                    .when(!has_selection, |d| d.text_color(rgba(th.text_faint)))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.text.menu = None;
                        this.copy_text(&CopyText, window, cx);
                        cx.notify();
                    })),
            )
            .child(
                menu_item("text-select-all", "Select all", th).on_click(cx.listener(
                    |this, _, _, cx| {
                        this.text.menu = None;
                        this.text.select_all();
                        cx.notify();
                    },
                )),
            );
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .child(
                    deferred(
                        div()
                            .id("text-menu-scrim")
                            .absolute()
                            .top(px(-2000.0))
                            .left(px(-4000.0))
                            .w(px(8000.0))
                            .h(px(6000.0))
                            .occlude()
                            .on_mouse_down(MouseButton::Left, close())
                            .on_mouse_down(MouseButton::Right, close()),
                    )
                    .with_priority(3),
                )
                .child(
                    deferred(
                        anchored()
                            .position(at)
                            .snap_to_window_with_margin(px(8.0))
                            .child(div().occlude().child(list)),
                    )
                    .with_priority(4),
                )
                .into_any_element(),
        )
    }
}

/// `anchor` and `head` start first; `None` when they select nothing.
fn ordered(anchor: Option<Spot>, head: Option<Spot>) -> Option<(Spot, Spot)> {
    let (a, b) = (anchor?, head?);
    (a != b).then(|| if a < b { (a, b) } else { (b, a) })
}

/// The part of the piece `key`, `len` bytes long, from `start` to `end`.
fn range_in((start, end): (Spot, Spot), key: Key, len: usize) -> Option<Range<usize>> {
    if key < start.key || key > end.key {
        return None;
    }
    let from = if key == start.key { start.offset } else { 0 };
    let to = if key == end.key { end.offset } else { len };
    (from < to).then(|| from.min(len)..to.min(len))
}

/// The theme's accent, see-through, as text fields show a selection; a
/// little stronger on a dark page, where a faint tint gets lost.
fn selection_color(th: &Theme) -> gpui::Hsla {
    let alpha = if th.dark { 0x66 } else { 0x4d };
    rgba((th.accent & 0xffff_ff00) | alpha).into()
}

/// `highlights` with the `selected` range's background set to `color`,
/// still sorted and not overlapping.
fn with_selection(
    highlights: Vec<(Range<usize>, HighlightStyle)>,
    selected: Range<usize>,
    color: gpui::Hsla,
) -> Vec<(Range<usize>, HighlightStyle)> {
    let mut cuts = vec![selected.start, selected.end];
    for (range, _) in &highlights {
        cuts.extend([range.start, range.end]);
    }
    cuts.sort_unstable();
    cuts.dedup();
    let mut out = Vec::new();
    for pair in cuts.windows(2) {
        let (from, to) = (pair[0], pair[1]);
        let base = highlights
            .iter()
            .find(|(range, _)| range.start <= from && to <= range.end)
            .map(|(_, style)| *style);
        let chosen = selected.start <= from && to <= selected.end;
        let style = match (base, chosen) {
            (base, true) => HighlightStyle {
                background_color: Some(color),
                ..base.unwrap_or_default()
            },
            (Some(base), false) => base,
            (None, false) => continue,
        };
        out.push((from..to, style));
    }
    out
}

/// `offset`, moved back to the start of the character it is in.
fn floor_char(text: &str, offset: usize) -> usize {
    let mut offset = offset.min(text.len());
    while !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '\''
}

/// The word at `offset`, or the one character there when it is not part
/// of a word.
fn word_at(text: &str, offset: usize) -> Range<usize> {
    let offset = floor_char(text, offset);
    let start = text[..offset]
        .char_indices()
        .rev()
        .take_while(|(_, c)| is_word(*c))
        .last()
        .map_or(offset, |(ix, _)| ix);
    let end = text[offset..]
        .char_indices()
        .find(|(_, c)| !is_word(*c))
        .map_or(text.len(), |(ix, _)| offset + ix);
    if start < end {
        return start..end;
    }
    let next = text[offset..].chars().next().map_or(0, char::len_utf8);
    offset..offset + next
}

/// The line (between line breaks) at `offset`.
fn paragraph_at(text: &str, offset: usize) -> Range<usize> {
    let offset = floor_char(text, offset);
    let start = text[..offset].rfind('\n').map_or(0, |ix| ix + 1);
    let end = text[offset..]
        .find('\n')
        .map_or(text.len(), |ix| offset + ix);
    start..end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_and_paragraphs() {
        let text = "Hello, wide world.\nSecond línea here";
        assert_eq!(&text[word_at(text, 1)], "Hello");
        assert_eq!(&text[word_at(text, 5)], "Hello");
        assert_eq!(&text[word_at(text, 6)], " ");
        assert_eq!(&text[word_at(text, 9)], "wide");
        assert_eq!(&text[word_at(text, 27)], "línea");
        assert_eq!(&text[paragraph_at(text, 3)], "Hello, wide world.");
        assert_eq!(&text[paragraph_at(text, 20)], "Second línea here");
        // Inside a character: its start.
        assert_eq!(floor_char(text, 27), 27);
        assert_eq!(floor_char("é", 1), 0);
    }

    #[test]
    fn selection_splits_highlights() {
        let bold = HighlightStyle {
            font_weight: Some(gpui::FontWeight::BOLD),
            ..Default::default()
        };
        let color = gpui::Hsla::default();
        let out = with_selection(vec![(2..6, bold)], 4..9, color);
        let ranges: Vec<_> = out.iter().map(|(r, _)| r.clone()).collect();
        assert_eq!(ranges, vec![2..4, 4..6, 6..9]);
        assert_eq!(out[0].1, bold);
        assert_eq!(out[1].1.font_weight, bold.font_weight);
        assert_eq!(out[1].1.background_color, Some(color));
        assert_eq!(out[2].1.font_weight, None);
        assert_eq!(out[2].1.background_color, Some(color));
    }

    #[test]
    fn selected_ranges_follow_the_order_of_pieces() {
        let spot = |part, piece, offset| Spot {
            key: Key { part, piece },
            offset,
        };
        // Selected upward: from part 1 back into part 0.
        let sel = ordered(Some(spot(1, 2, 3)), Some(spot(0, 4, 5))).unwrap();
        let range = |part, piece| range_in(sel, Key { part, piece }, 10);
        assert_eq!(range(0, 3), None);
        assert_eq!(range(0, 4), Some(5..10));
        assert_eq!(range(1, 0), Some(0..10));
        assert_eq!(range(1, 2), Some(0..3));
        assert_eq!(range(1, 3), None);
        assert_eq!(ordered(Some(spot(1, 2, 3)), Some(spot(1, 2, 3))), None);
    }
}
