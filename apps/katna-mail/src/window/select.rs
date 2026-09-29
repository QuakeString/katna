// SPDX-License-Identifier: GPL-3.0-or-later

//! Selecting and copying text, as in a browser: drag to select,
//! double-click for a word, triple-click for a paragraph, Shift+click to
//! extend, Ctrl+A and Ctrl+C once the text has been clicked, and Copy on
//! the right-click menu. The open conversation (plain and HTML mail alike)
//! and the attachment viewer's text files, documents, slides and PDFs
//! share it; each is a [`SelectHost`].
//!
//! Every run of text a host draws is a *piece*, keyed by its *part* (a
//! message, a PDF page) and its order in that part. Drawing a piece shows
//! the part of it that is selected; laying it out records where it went,
//! so a pointer position can be turned into a place in the text. Text
//! laid out by GPUI records its `TextLayout`; text drawn some other way (a
//! PDF's letters, on a picture of the page) records where each character
//! is.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Range;
use std::rc::Rc;

use gpui::{
    AnyElement, App, Bounds, ClipboardItem, Context, DispatchPhase, Div, FocusHandle,
    HighlightStyle, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point,
    SharedString, StyledText, TextLayout, Window, actions, anchored, canvas, deferred, div,
    prelude::*, rgba,
};

use katna_i18n::tr;
use katna_ui::{px, unpx};

use super::MailWindow;
use crate::theme::Theme;
use crate::widgets::{menu_item, raised};

actions!(katna_mail, [CopyText, SelectAllText]);

/// The key context of the conversation's text once it was clicked.
pub(super) const TEXT_CONTEXT: &str = "MessageText";

/// A run of text drawn by `part` of the text (a message of the
/// conversation, a page of a PDF), the `piece`th in it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Key {
    part: usize,
    piece: usize,
}

impl Key {
    pub(super) fn new(part: usize, piece: usize) -> Self {
        Self { part, piece }
    }
}

/// A place in the text, from outside: the part, the piece in it and a
/// byte offset in the piece.
pub(super) type TextPlace = (usize, usize, usize);

/// What owns a [`TextSelection`]: the conversation's window, the
/// attachment viewer.
pub(super) trait SelectHost: 'static + Sized {
    fn selection(&self) -> &TextSelection;
    fn selection_mut(&mut self) -> &mut TextSelection;
    /// What a click in the text focuses, so Ctrl+C and Ctrl+A reach it.
    fn text_focus(&self) -> FocusHandle {
        self.selection().focus.clone()
    }

    /// A drag or click that selects has ended (the PDF viewer turns the
    /// selection into a highlight when marking up).
    fn selected(&mut self, _cx: &mut Context<Self>) {}
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
    place: Place,
    /// The frame that laid it out; older pieces are no longer on screen.
    frame: u64,
}

/// Where a piece went on screen.
enum Place {
    /// Laid out by GPUI.
    Laid(TextLayout),
    /// Drawn some other way: its bounds, and where each character starts
    /// (a byte offset) and how far from the window's left it is drawn.
    Drawn(Bounds<Pixels>, Rc<Vec<(usize, Pixels)>>),
}

impl Place {
    fn bounds(&self) -> Bounds<Pixels> {
        match self {
            Place::Laid(layout) => layout.bounds(),
            Place::Drawn(bounds, _) => *bounds,
        }
    }

    /// The offset in `text` nearest to `at`, inside the bounds.
    fn index_at(&self, text: &str, at: Point<Pixels>) -> usize {
        match self {
            Place::Laid(layout) => match layout.index_for_position(at) {
                Ok(ix) | Err(ix) => ix,
            },
            Place::Drawn(bounds, chars) => chars
                .iter()
                .enumerate()
                .find(|(ix, (_, x))| {
                    let next = chars.get(ix + 1).map_or(bounds.right(), |(_, x)| *x);
                    at.x < *x + (next - *x) / 2.0
                })
                .map_or(text.len(), |(_, (offset, _))| *offset),
        }
    }
}

pub(super) struct TextSelection {
    pub(super) focus: FocusHandle,
    drawn: Rc<RefCell<BTreeMap<Key, Piece>>>,
    frame: u64,
    /// What the selection is in (a conversation, an attachment), hashed.
    owner: Option<u64>,
    /// All of the text in order, when the host knows it: copying and
    /// Select all then reach what is scrolled out of sight too.
    all: Option<Rc<Vec<(Key, SharedString)>>>,
    /// What goes between the text of two parts when copying.
    part_gap: &'static str,
    /// Where the selection started and where it ends now.
    anchor: Option<Spot>,
    head: Option<Spot>,
    /// A drag is extending the selection.
    selecting: bool,
    unit: Unit,
    /// What the click that started the drag selected (a word or paragraph).
    origin: Option<(Spot, Spot)>,
    /// The right-click menu, where the pointer was.
    pub(super) menu: Option<Point<Pixels>>,
}

impl TextSelection {
    pub(super) fn new(cx: &mut App) -> Self {
        Self {
            focus: cx.focus_handle(),
            drawn: Rc::default(),
            frame: 0,
            owner: None,
            all: None,
            part_gap: "\n\n",
            anchor: None,
            head: None,
            selecting: false,
            unit: Unit::Char,
            origin: None,
            menu: None,
        }
    }

    /// Starts drawing a frame of `owner`'s text (call before drawing it);
    /// a different owner drops the selection.
    pub(super) fn begin(&mut self, owner: impl Hash) {
        let mut hasher = DefaultHasher::new();
        owner.hash(&mut hasher);
        let owner = hasher.finish();
        self.frame += 1;
        if self.owner != Some(owner) {
            self.owner = Some(owner);
            self.clear();
            self.all = None;
            self.drawn.borrow_mut().clear();
        }
    }

    /// All of the text, in order, for copying what is out of sight.
    pub(super) fn set_all(&mut self, all: Option<Rc<Vec<(Key, SharedString)>>>) {
        self.all = all;
    }

    /// What goes between the text of two parts when copying.
    pub(super) fn set_part_gap(&mut self, gap: &'static str) {
        self.part_gap = gap;
    }

    pub(super) fn clear(&mut self) {
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

    /// Where the selection starts and ends, start first.
    pub(super) fn span(&self) -> Option<(TextPlace, TextPlace)> {
        let (start, end) = self.ordered()?;
        Some((
            (start.key.part, start.key.piece, start.offset),
            (end.key.part, end.key.piece, end.offset),
        ))
    }

    /// The selected part of the piece `key`, `len` bytes long.
    fn range_in(&self, key: Key, len: usize) -> Option<Range<usize>> {
        range_in(self.ordered()?, key, len)
    }

    /// What draws pieces with the selection shown, kept by elements that
    /// are laid out later (the rows of a list).
    pub(super) fn marker(&self, th: &Theme) -> Marker {
        Marker {
            drawn: self.drawn.clone(),
            frame: self.frame,
            selected: self.ordered(),
            color: selection_color(th),
        }
    }

    /// Pieces for `part`, drawn with the selection shown.
    pub(super) fn pieces(&self, part: usize, th: &Theme) -> Pieces {
        Pieces {
            marker: self.marker(th),
            part,
            next: 0,
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
            let b = piece.place.bounds();
            let dx = unpx((b.left() - at.x).max(at.x - b.right())).max(0.0);
            let dy = unpx((b.top() - at.y).max(at.y - b.bottom())).max(0.0);
            // Beside a line counts as on it: rows beat columns.
            let distance = dy * 8.0 + dx;
            if best.is_none_or(|(d, _)| distance < d) {
                best = Some((distance, *key));
            }
        }
        let key = best?.1;
        let piece = &drawn[&key];
        let b = piece.place.bounds();
        let offset = if at.y < b.top() {
            0
        } else if at.y > b.bottom() {
            piece.text.len()
        } else {
            let x = at.x.clamp(b.left(), b.right());
            piece.place.index_at(&piece.text, gpui::point(x, at.y))
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

    /// Selects all of the text, or every piece on screen when the host
    /// does not know it all.
    fn select_all(&mut self) {
        if let Some(all) = &self.all {
            if let (Some((first, _)), Some((last, text))) = (all.first(), all.last()) {
                self.anchor = Some(Spot {
                    key: *first,
                    offset: 0,
                });
                self.head = Some(Spot {
                    key: *last,
                    offset: text.len(),
                });
            }
            return;
        }
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

    /// The selected text: pieces on their own lines, parts apart.
    pub(super) fn text(&self) -> String {
        let Some((start, end)) = self.ordered() else {
            return String::new();
        };
        let mut out = String::new();
        let mut previous: Option<Key> = None;
        let mut add = |key: Key, text: &str| {
            let Some(range) = self.range_in(key, text.len()) else {
                return;
            };
            if let Some(previous) = previous {
                out.push_str(if previous.part == key.part {
                    "\n"
                } else {
                    self.part_gap
                });
            }
            out.push_str(&text[range]);
            previous = Some(key);
        };
        let mut texts: BTreeMap<Key, &str> = BTreeMap::new();
        // All of the host's text, out of sight too, when it gives it.
        if let Some(all) = &self.all {
            let from = all.partition_point(|(key, _)| *key < start.key);
            for (key, text) in all[from..].iter().take_while(|(key, _)| *key <= end.key) {
                texts.insert(*key, text.as_ref());
            }
        }
        // Else as drawn, as is what is drawn beside it (the contact panel
        // beside a conversation).
        let drawn = self.drawn.borrow();
        let given: std::collections::BTreeSet<usize> = match &self.all {
            Some(all) => all.iter().map(|(key, _)| key.part).collect(),
            None => Default::default(),
        };
        for (key, piece) in drawn.range(start.key..=end.key) {
            if piece.frame == self.frame && !given.contains(&key.part) {
                texts.insert(*key, piece.text.as_ref());
            }
        }
        for (key, text) in texts {
            add(key, text);
        }
        out
    }
}

/// Draws pieces with the selection shown and records where they went.
/// Cheap to clone, and free of borrows, for rows laid out later.
#[derive(Clone)]
pub(super) struct Marker {
    drawn: Rc<RefCell<BTreeMap<Key, Piece>>>,
    frame: u64,
    selected: Option<(Spot, Spot)>,
    color: gpui::Hsla,
}

impl Marker {
    /// The selected part of the piece `key`, `len` bytes long.
    pub(super) fn range(&self, key: Key, len: usize) -> Option<Range<usize>> {
        range_in(self.selected?, key, len)
    }

    pub(super) fn color(&self) -> gpui::Hsla {
        self.color
    }

    /// `text` with `highlights` (sorted, not overlapping) and the
    /// selection shown, and the element to put it in, which records its
    /// layout.
    pub(super) fn piece(
        &self,
        key: Key,
        text: SharedString,
        highlights: Vec<(Range<usize>, HighlightStyle)>,
    ) -> (StyledText, Div) {
        let highlights = match self.range(key, text.len()) {
            Some(selected) => with_selection(highlights, selected, self.color),
            None => highlights,
        };
        let styled = StyledText::new(text.clone()).with_highlights(highlights);
        let layout = styled.layout().clone();
        let drawn = self.drawn.clone();
        let frame = self.frame;
        let holder = div().cursor_text().on_children_prepainted(move |_, _, _| {
            drawn.borrow_mut().insert(
                key,
                Piece {
                    text: text.clone(),
                    place: Place::Laid(layout.clone()),
                    frame,
                },
            );
        });
        (styled, holder)
    }

    /// Records text drawn some other way: within `bounds`, each
    /// character starting (a byte offset) at a distance from the window's
    /// left. Call while painting.
    pub(super) fn place(
        &self,
        key: Key,
        text: SharedString,
        bounds: Bounds<Pixels>,
        chars: Vec<(usize, Pixels)>,
    ) {
        self.drawn.borrow_mut().insert(
            key,
            Piece {
                text,
                place: Place::Drawn(bounds, Rc::new(chars)),
                frame: self.frame,
            },
        );
    }
}

/// Draws the text runs of one part in order so they can be selected.
pub(super) struct Pieces {
    marker: Marker,
    part: usize,
    next: usize,
}

impl Pieces {
    /// The next run: see [`Marker::piece`].
    pub(super) fn piece(
        &mut self,
        text: SharedString,
        highlights: Vec<(Range<usize>, HighlightStyle)>,
    ) -> (StyledText, Div) {
        let key = Key::new(self.part, self.next);
        self.next += 1;
        self.marker.piece(key, text, highlights)
    }
}

impl SelectHost for MailWindow {
    fn selection(&self) -> &TextSelection {
        &self.text
    }

    fn selection_mut(&mut self) -> &mut TextSelection {
        &mut self.text
    }
}

impl MailWindow {
    /// Makes the text in `body` (the text of message `part`) selectable
    /// with the mouse.
    pub(super) fn selectable_body(&self, part: usize, body: Div, cx: &mut Context<Self>) -> Div {
        selectable(body, Some(part), cx)
    }

    /// The conversation's text: focusable so Ctrl+A and Ctrl+C reach it,
    /// and following the pointer while a drag selects.
    pub(super) fn text_area(&self, body: Div, cx: &mut Context<Self>) -> Div {
        body.key_context(TEXT_CONTEXT)
            .track_focus(&self.text.focus)
            .on_action(cx.listener(|this, _: &CopyText, _, cx| copy(this, cx)))
            .on_action(cx.listener(|this, _: &SelectAllText, _, cx| select_all(this, cx)))
            .child(follow_drags(cx))
    }

    /// The right-click menu of the conversation's text.
    pub(super) fn render_text_menu(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        text_menu(self, th, cx)
    }
}

/// Makes the text in `body` selectable with the mouse: the pieces of
/// `part` only, or all of them.
pub(super) fn selectable<T: SelectHost>(
    body: Div,
    part: Option<usize>,
    cx: &mut Context<T>,
) -> Div {
    body.on_mouse_down(
        MouseButton::Left,
        cx.listener(move |this, event: &MouseDownEvent, window, cx| {
            press(this, part, event, window, cx);
        }),
    )
    .on_mouse_down(
        MouseButton::Right,
        cx.listener(move |this, event: &MouseDownEvent, window, cx| {
            cx.stop_propagation();
            window.focus(&this.text_focus(), cx);
            this.selection_mut().menu = Some(event.position);
            cx.notify();
        }),
    )
}

fn press<T: SelectHost>(
    this: &mut T,
    part: Option<usize>,
    event: &MouseDownEvent,
    window: &mut Window,
    cx: &mut Context<T>,
) {
    cx.stop_propagation();
    window.focus(&this.text_focus(), cx);
    let text = this.selection_mut();
    text.menu = None;
    let Some(spot) = text.hit(event.position, part) else {
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

fn drag<T: SelectHost>(this: &mut T, event: &MouseMoveEvent, cx: &mut Context<T>) {
    if event.pressed_button != Some(MouseButton::Left) {
        release(this, cx);
        return;
    }
    let text = this.selection_mut();
    let Some(spot) = text.hit(event.position, None) else {
        return;
    };
    let before = (text.anchor, text.head);
    text.extend_to(spot);
    if before != (text.anchor, text.head) {
        cx.notify();
    }
}

fn release<T: SelectHost>(this: &mut T, cx: &mut Context<T>) {
    let text = this.selection_mut();
    text.selecting = false;
    // As on any Linux desktop, the selection can be pasted with the
    // middle button.
    let selected = text.text();
    if !selected.is_empty() {
        katna_ui::native::write_to_primary(cx, ClipboardItem::new_string(selected));
    }
    this.selected(cx);
}

/// Copies the selection, if any.
pub(super) fn copy<T: SelectHost>(this: &mut T, cx: &mut Context<T>) {
    let selected = this.selection().text();
    if !selected.is_empty() {
        cx.write_to_clipboard(ClipboardItem::new_string(selected));
    }
}

/// Selects all of the text.
pub(super) fn select_all<T: SelectHost>(this: &mut T, cx: &mut Context<T>) {
    this.selection_mut().select_all();
    cx.notify();
}

/// Follows the pointer while a drag selects, wherever it goes. Put it
/// anywhere in the host's element.
pub(super) fn follow_drags<T: SelectHost>(cx: &mut Context<T>) -> impl IntoElement {
    let this = cx.entity().downgrade();
    canvas(
        |_, _, _| {},
        move |_, _, window, _| {
            window.on_mouse_event({
                let this = this.clone();
                move |event: &MouseMoveEvent, phase, _, cx| {
                    let Some(this) = this.upgrade() else {
                        return;
                    };
                    if phase == DispatchPhase::Bubble && this.read(cx).selection().selecting {
                        this.update(cx, |this, cx| drag(this, event, cx));
                    }
                }
            });
            window.on_mouse_event(move |event: &MouseUpEvent, phase, _, cx| {
                let Some(this) = this.upgrade() else {
                    return;
                };
                if phase == DispatchPhase::Bubble
                    && event.button == MouseButton::Left
                    && this.read(cx).selection().selecting
                {
                    this.update(cx, |this, cx| release(this, cx));
                }
            });
        },
    )
    .absolute()
    .size_0()
}

/// The right-click menu of a host's text, when open.
pub(super) fn text_menu<T: SelectHost>(
    this: &T,
    th: &Theme,
    cx: &mut Context<T>,
) -> Option<AnyElement> {
    let at = this.selection().menu?;
    Some(copy_menu(
        at,
        !this.selection().is_empty(),
        th,
        cx,
        |this: &mut T, act, cx| {
            this.selection_mut().menu = None;
            match act {
                MenuAct::Close => {}
                MenuAct::Copy => copy(this, cx),
                MenuAct::SelectAll => this.selection_mut().select_all(),
            }
            cx.notify();
        },
    ))
}

/// A choice on [`copy_menu`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MenuAct {
    Close,
    Copy,
    SelectAll,
}

/// A right-click menu at `at` with Copy (greyed without a selection) and
/// Select all; `act` does what was chosen, or closes it.
pub(super) fn copy_menu<T: 'static>(
    at: Point<Pixels>,
    can_copy: bool,
    th: &Theme,
    cx: &mut Context<T>,
    act: fn(&mut T, MenuAct, &mut Context<T>),
) -> AnyElement {
    let close = || {
        cx.listener(move |this: &mut T, _: &MouseDownEvent, _, cx| act(this, MenuAct::Close, cx))
    };
    let list = div()
        .key_context(crate::widgets::MENU_CONTEXT)
        .w(px(200.0))
        .py(px(8.0))
        .flex()
        .flex_col()
        .map(|d| raised(d, th, 8.0, 3.0))
        .text_size(px(14.0))
        .text_color(rgba(th.text))
        .child(
            menu_item("text-copy", &tr!("text-copy"), th)
                .when(!can_copy, |d| d.text_color(rgba(th.text_faint)))
                .on_click(cx.listener(move |this, _, _, cx| act(this, MenuAct::Copy, cx))),
        )
        .child(
            menu_item("text-select-all", &tr!("text-select-all"), th)
                .on_click(cx.listener(move |this, _, _, cx| act(this, MenuAct::SelectAll, cx))),
        );
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
        .into_any_element()
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
