// SPDX-License-Identifier: GPL-3.0-or-later

//! A multi-line plain-text editor with soft wrapping, IME support,
//! selection and clipboard, built on GPUI's input handler like
//! [`TextInput`](crate::TextInput).
//!
//! The area is as wide as its parent and as tall as its wrapped text (at
//! least one line). It draws only its text, selection and cursor; the parent
//! gives it a frame, scrolling and the text style (font, size, line height,
//! color), which it inherits.

use std::ops::Range;

use crate::scale::px;
use gpui::{
    App, AvailableSpace, Bounds, ClipboardItem, Context, CursorStyle, DispatchPhase, ElementId,
    ElementInputHandler, Entity, EntityInputHandler, EventEmitter, FocusHandle, Focusable,
    GlobalElementId, Hsla, KeyBinding, LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, PaintQuad, Pixels, Point, SharedString, Style, TextAlign, TextRun,
    UTF16Selection, UnderlineStyle, Window, WrappedLine, actions, div, fill, point, prelude::*,
    relative, size,
};
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

use crate::text_input::InputEvent;

actions!(
    text_area,
    [
        Backspace,
        Delete,
        DeleteWordLeft,
        DeleteWordRight,
        Newline,
        Left,
        Right,
        Up,
        Down,
        SelectLeft,
        SelectRight,
        SelectUp,
        SelectDown,
        WordLeft,
        WordRight,
        SelectWordLeft,
        SelectWordRight,
        Home,
        End,
        SelectHome,
        SelectEnd,
        DocStart,
        DocEnd,
        SelectDocStart,
        SelectDocEnd,
        PageUp,
        PageDown,
        SelectPageUp,
        SelectPageDown,
        SelectAll,
        Paste,
        Cut,
        Copy,
        Submit,
        Cancel,
    ]
);

/// Key context of a focused [`TextArea`].
pub const TEXT_AREA_CONTEXT: &str = "TextArea";

/// Binds the editing keys. Call once at startup.
pub fn bind_keys(cx: &mut App) {
    let context = Some(TEXT_AREA_CONTEXT);
    cx.bind_keys([
        KeyBinding::new("backspace", Backspace, context),
        KeyBinding::new("shift-backspace", Backspace, context),
        KeyBinding::new("delete", Delete, context),
        KeyBinding::new("ctrl-backspace", DeleteWordLeft, context),
        KeyBinding::new("ctrl-delete", DeleteWordRight, context),
        KeyBinding::new("enter", Newline, context),
        KeyBinding::new("shift-enter", Newline, context),
        KeyBinding::new("left", Left, context),
        KeyBinding::new("right", Right, context),
        KeyBinding::new("up", Up, context),
        KeyBinding::new("down", Down, context),
        KeyBinding::new("shift-left", SelectLeft, context),
        KeyBinding::new("shift-right", SelectRight, context),
        KeyBinding::new("shift-up", SelectUp, context),
        KeyBinding::new("shift-down", SelectDown, context),
        KeyBinding::new("ctrl-left", WordLeft, context),
        KeyBinding::new("ctrl-right", WordRight, context),
        KeyBinding::new("ctrl-shift-left", SelectWordLeft, context),
        KeyBinding::new("ctrl-shift-right", SelectWordRight, context),
        KeyBinding::new("home", Home, context),
        KeyBinding::new("end", End, context),
        KeyBinding::new("shift-home", SelectHome, context),
        KeyBinding::new("shift-end", SelectEnd, context),
        KeyBinding::new("ctrl-home", DocStart, context),
        KeyBinding::new("ctrl-end", DocEnd, context),
        KeyBinding::new("ctrl-shift-home", SelectDocStart, context),
        KeyBinding::new("ctrl-shift-end", SelectDocEnd, context),
        KeyBinding::new("pageup", PageUp, context),
        KeyBinding::new("pagedown", PageDown, context),
        KeyBinding::new("shift-pageup", SelectPageUp, context),
        KeyBinding::new("shift-pagedown", SelectPageDown, context),
        KeyBinding::new("ctrl-a", SelectAll, context),
        KeyBinding::new("ctrl-v", Paste, context),
        KeyBinding::new("shift-insert", Paste, context),
        KeyBinding::new("ctrl-c", Copy, context),
        KeyBinding::new("ctrl-insert", Copy, context),
        KeyBinding::new("ctrl-x", Cut, context),
        KeyBinding::new("shift-delete", Cut, context),
        KeyBinding::new("ctrl-enter", Submit, context),
        KeyBinding::new("escape", Cancel, context),
    ]);
}

/// A multi-line plain-text editor. Create it with [`TextArea::new`] inside
/// `cx.new` and render the entity as a child. It emits [`InputEvent`]:
/// `Changed` on edits, `Submit` on ctrl-enter and `Cancel` on escape.
pub struct TextArea {
    focus_handle: FocusHandle,
    content: SharedString,
    placeholder: SharedString,
    accent: Hsla,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    /// The cursor is on a soft-wrap point and shows at the end of the upper
    /// row instead of the start of the lower one.
    upstream: bool,
    /// The x the cursor keeps while moving up and down.
    goal_x: Option<Pixels>,
    last_layout: Option<Layout>,
    is_selecting: bool,
    /// One wrapped line: enter submits and pasted line breaks become
    /// spaces.
    single_line: bool,
}

impl EventEmitter<InputEvent> for TextArea {}

impl TextArea {
    pub fn new(placeholder: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle().tab_stop(true),
            content: SharedString::default(),
            placeholder: placeholder.into(),
            accent: gpui::blue(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            upstream: false,
            goal_x: None,
            last_layout: None,
            is_selecting: false,
            single_line: false,
        }
    }

    /// Keeps the text one line that wraps to the area's width, for a long
    /// value in a form: enter submits like ctrl-enter, and pasted line
    /// breaks become spaces.
    pub fn set_single_line(&mut self, single_line: bool) {
        self.single_line = single_line;
    }

    pub fn text(&self) -> &str {
        &self.content
    }

    /// Replaces the text; the cursor goes to `cursor` (a byte offset,
    /// clamped to a char boundary). CRLF and lone CR become LF (the cursor
    /// shifts to match). Emits [`InputEvent::Changed`] if the text changed.
    pub fn set_text(
        &mut self,
        text: impl Into<SharedString>,
        cursor: usize,
        cx: &mut Context<Self>,
    ) {
        let mut text: SharedString = text.into();
        let mut cursor = clamp_to_char_boundary(&text, cursor);
        if text.contains('\r') {
            cursor = normalize_newlines(&text[..cursor]).len();
            text = normalize_newlines(&text).into();
        }
        let changed = text != self.content;
        self.content = text;
        self.selected_range = cursor..cursor;
        self.selection_reversed = false;
        self.marked_range = None;
        self.upstream = false;
        self.goal_x = None;
        cx.notify();
        if changed {
            cx.emit(InputEvent::Changed);
        }
    }

    /// Where the cursor is, as a byte offset into [`TextArea::text`].
    pub fn cursor(&self) -> usize {
        self.cursor_offset()
    }

    pub fn set_placeholder(&mut self, placeholder: impl Into<SharedString>) {
        self.placeholder = placeholder.into();
    }

    /// Color of the cursor and (translucent) of the selection.
    pub fn set_accent(&mut self, accent: Hsla) {
        self.accent = accent;
    }

    /// The cursor's bounds in window coordinates as of the last paint, so a
    /// scrolling parent can keep it in view.
    pub fn cursor_bounds(&self) -> Option<Bounds<Pixels>> {
        let layout = self.last_layout.as_ref()?;
        let position = layout.position_for(self.cursor_offset(), self.upstream);
        Some(Bounds::new(
            layout.bounds.origin + position,
            size(px(2.), layout.line_height),
        ))
    }

    // Movement.

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(prev_grapheme(&self.content, self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx)
        }
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(next_grapheme(&self.content, self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.end, cx)
        }
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(prev_grapheme(&self.content, self.cursor_offset()), cx);
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(next_grapheme(&self.content, self.cursor_offset()), cx);
    }

    fn word_left(&mut self, _: &WordLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(word_left(&self.content, self.cursor_offset()), cx);
    }

    fn word_right(&mut self, _: &WordRight, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(word_right(&self.content, self.cursor_offset()), cx);
    }

    fn select_word_left(&mut self, _: &SelectWordLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(word_left(&self.content, self.cursor_offset()), cx);
    }

    fn select_word_right(&mut self, _: &SelectWordRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(word_right(&self.content, self.cursor_offset()), cx);
    }

    fn up(&mut self, _: &Up, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertically(-1, false, cx);
    }

    fn down(&mut self, _: &Down, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertically(1, false, cx);
    }

    fn select_up(&mut self, _: &SelectUp, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertically(-1, true, cx);
    }

    fn select_down(&mut self, _: &SelectDown, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertically(1, true, cx);
    }

    fn page_up(&mut self, _: &PageUp, window: &mut Window, cx: &mut Context<Self>) {
        self.move_vertically(-self.page_rows(window), false, cx);
    }

    fn page_down(&mut self, _: &PageDown, window: &mut Window, cx: &mut Context<Self>) {
        self.move_vertically(self.page_rows(window), false, cx);
    }

    fn select_page_up(&mut self, _: &SelectPageUp, window: &mut Window, cx: &mut Context<Self>) {
        self.move_vertically(-self.page_rows(window), true, cx);
    }

    fn select_page_down(
        &mut self,
        _: &SelectPageDown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_vertically(self.page_rows(window), true, cx);
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        let (offset, upstream) = self.row_home();
        self.move_to_affine(offset, upstream, cx);
    }

    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        let (offset, upstream) = self.row_end();
        self.move_to_affine(offset, upstream, cx);
    }

    fn select_home(&mut self, _: &SelectHome, _: &mut Window, cx: &mut Context<Self>) {
        let (offset, upstream) = self.row_home();
        self.select_to_affine(offset, upstream, cx);
    }

    fn select_end(&mut self, _: &SelectEnd, _: &mut Window, cx: &mut Context<Self>) {
        let (offset, upstream) = self.row_end();
        self.select_to_affine(offset, upstream, cx);
    }

    fn doc_start(&mut self, _: &DocStart, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
    }

    fn doc_end(&mut self, _: &DocEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.content.len(), cx);
    }

    fn select_doc_start(&mut self, _: &SelectDocStart, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(0, cx);
    }

    fn select_doc_end(&mut self, _: &SelectDocEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.content.len(), cx);
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
        self.select_to(self.content.len(), cx)
    }

    // Editing.

    fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let prev = prev_grapheme(&self.content, self.cursor_offset());
            if self.cursor_offset() == prev {
                return;
            }
            self.select_to(prev, cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let next = next_grapheme(&self.content, self.cursor_offset());
            if self.cursor_offset() == next {
                return;
            }
            self.select_to(next, cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn delete_word_left(
        &mut self,
        _: &DeleteWordLeft,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.selected_range.is_empty() {
            let target = word_left(&self.content, self.cursor_offset());
            if target == self.cursor_offset() {
                return;
            }
            self.select_to(target, cx);
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn delete_word_right(
        &mut self,
        _: &DeleteWordRight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.selected_range.is_empty() {
            let target = word_right(&self.content, self.cursor_offset());
            if target == self.cursor_offset() {
                return;
            }
            self.select_to(target, cx);
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn newline(&mut self, _: &Newline, window: &mut Window, cx: &mut Context<Self>) {
        if self.single_line {
            cx.emit(InputEvent::Submit);
            return;
        }
        self.replace_text_in_range(None, "\n", window, cx)
    }

    fn submit(&mut self, _: &Submit, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(InputEvent::Submit);
    }

    fn cancel(&mut self, _: &Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(InputEvent::Cancel);
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            let mut text = normalize_newlines(&text);
            if self.single_line {
                text = text.trim_matches('\n').replace('\n', " ");
            }
            self.replace_text_in_range(None, &text, window, cx);
        }
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
        }
    }

    fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
            self.replace_text_in_range(None, "", window, cx)
        }
    }

    // Mouse.

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle, cx);
        let (offset, upstream) = self.offset_for_position(event.position);
        match event.click_count {
            2 => {
                self.is_selecting = false;
                let range = word_range(&self.content, offset);
                self.move_to(range.start, cx);
                self.select_to(range.end, cx);
            }
            n if n >= 3 => {
                self.is_selecting = false;
                let mut range = line_range(&self.content, offset);
                if range.end < self.content.len() {
                    range.end += 1; // the newline
                }
                self.move_to(range.start, cx);
                self.select_to(range.end, cx);
            }
            _ => {
                self.is_selecting = true;
                if event.modifiers.shift {
                    self.select_to_affine(offset, upstream, cx);
                } else {
                    self.move_to_affine(offset, upstream, cx);
                }
            }
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, _: &mut Context<Self>) {
        self.is_selecting = false;
    }

    /// Extends a drag selection; runs for moves anywhere in the window so
    /// dragging past the edges keeps selecting.
    fn drag_to(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        if event.pressed_button != Some(MouseButton::Left) {
            self.is_selecting = false;
            return;
        }
        let (offset, upstream) = self.offset_for_position(event.position);
        self.select_to_affine(offset, upstream, cx);
    }

    // Helpers.

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.move_to_affine(offset, false, cx);
    }

    fn move_to_affine(&mut self, offset: usize, upstream: bool, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        self.selection_reversed = false;
        self.upstream = upstream;
        self.goal_x = None;
        cx.notify()
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.select_to_affine(offset, false, cx);
    }

    fn select_to_affine(&mut self, offset: usize, upstream: bool, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset
        } else {
            self.selected_range.end = offset
        };
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        self.upstream = upstream;
        self.goal_x = None;
        cx.notify()
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    /// Moves the cursor `delta` visual rows, keeping its x.
    fn move_vertically(&mut self, delta: isize, select: bool, cx: &mut Context<Self>) {
        let from = if select || self.selected_range.is_empty() {
            self.cursor_offset()
        } else if delta < 0 {
            self.selected_range.start
        } else {
            self.selected_range.end
        };
        let upstream = self.upstream && from == self.cursor_offset();
        let Some(layout) = self.last_layout.as_ref() else {
            let target = if delta < 0 { 0 } else { self.content.len() };
            if select {
                self.select_to(target, cx)
            } else {
                self.move_to(target, cx)
            }
            return;
        };
        let row = layout.row_for(from, upstream);
        let goal_x = self
            .goal_x
            .unwrap_or_else(|| layout.x_in_row(row, from.min(layout.rows[row].end)));
        let target_row = row as isize + delta;
        let (offset, upstream) = if target_row < 0 {
            (0, false)
        } else if target_row as usize >= layout.rows.len() {
            (self.content.len(), false)
        } else {
            layout.offset_in_row(&self.content, target_row as usize, goal_x)
        };
        if select {
            self.select_to_affine(offset, upstream, cx);
        } else {
            self.move_to_affine(offset, upstream, cx);
        }
        self.goal_x = Some(goal_x);
    }

    fn page_rows(&self, window: &Window) -> isize {
        let line_height = self
            .last_layout
            .as_ref()
            .map_or_else(|| window.line_height(), |layout| layout.line_height);
        let rows = (window.viewport_size().height / line_height).floor() as isize;
        (rows - 2).max(1)
    }

    fn row_home(&self) -> (usize, bool) {
        let cursor = self.cursor_offset();
        match self.last_layout.as_ref() {
            // The layout may be a frame old; keep the offset valid.
            Some(layout) => (
                clamp_to_char_boundary(
                    &self.content,
                    layout.rows[layout.row_for(cursor, self.upstream)].start,
                ),
                false,
            ),
            None => (line_range(&self.content, cursor).start, false),
        }
    }

    fn row_end(&self) -> (usize, bool) {
        let cursor = self.cursor_offset();
        match self.last_layout.as_ref() {
            Some(layout) => {
                let row = &layout.rows[layout.row_for(cursor, self.upstream)];
                (clamp_to_char_boundary(&self.content, row.end), row.soft_end)
            }
            None => (line_range(&self.content, cursor).end, false),
        }
    }

    fn offset_for_position(&self, position: Point<Pixels>) -> (usize, bool) {
        match self.last_layout.as_ref() {
            Some(layout) => layout.offset_for_position(&self.content, position),
            None => (0, false),
        }
    }

    fn replace(&mut self, range: Range<usize>, new_text: &str, cx: &mut Context<Self>) {
        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                .into();
        self.upstream = false;
        self.goal_x = None;
        cx.notify();
        cx.emit(InputEvent::Changed);
    }
}

impl EntityInputHandler for TextArea {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = range_from_utf16(&self.content, &range_utf16);
        actual_range.replace(range_to_utf16(&self.content, &range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: range_to_utf16(&self.content, &self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| range_to_utf16(&self.content, range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| range_from_utf16(&self.content, range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());
        self.selected_range = range.start + new_text.len()..range.start + new_text.len();
        self.selection_reversed = false;
        self.marked_range.take();
        self.replace(range, new_text, cx);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| range_from_utf16(&self.content, range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());
        self.marked_range =
            (!new_text.is_empty()).then(|| range.start..range.start + new_text.len());
        self.replace(range.clone(), new_text, cx);
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| range_from_utf16(new_text, range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.start)
            .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());
        self.selection_reversed = false;
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let layout = self.last_layout.as_ref()?;
        let range = range_from_utf16(&self.content, &range_utf16);
        let start = layout.position_for(range.start, false);
        let end = layout.position_for(range.end, false);
        let end_x = if end.y == start.y {
            end.x.max(start.x)
        } else {
            start.x
        };
        Some(Bounds::from_corners(
            bounds.origin + start,
            bounds.origin + point(end_x, start.y + layout.line_height),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let layout = self.last_layout.as_ref()?;
        if !layout.bounds.contains(&point) {
            return None;
        }
        let (offset, _) = layout.offset_for_position(&self.content, point);
        Some(offset_to_utf16(&self.content, offset))
    }
}

/// The wrapped text of the last paint and how it maps to offsets.
struct Layout {
    /// One per logical line of the displayed text.
    lines: Vec<WrappedLine>,
    /// Visual rows of the content (a single empty row when the placeholder
    /// shows).
    rows: Vec<Row>,
    bounds: Bounds<Pixels>,
    line_height: Pixels,
    /// The placeholder is displayed, so no offset but 0 exists.
    placeholder: bool,
}

impl Layout {
    fn row_for(&self, offset: usize, upstream: bool) -> usize {
        row_index(&self.rows, offset, upstream)
    }

    /// The x of `offset` relative to the start of visual row `row_ix`.
    fn x_in_row(&self, row_ix: usize, offset: usize) -> Pixels {
        if self.placeholder {
            return px(0.);
        }
        let row = &self.rows[row_ix];
        let Some(line) = self.lines.get(row.line) else {
            return px(0.);
        };
        let layout = &line.unwrapped_layout;
        let offset = offset.clamp(row.start, row.end);
        layout.x_for_index(offset - row.line_start) - layout.x_for_index(row.start - row.line_start)
    }

    /// Where the cursor at `offset` is drawn, relative to the bounds' origin.
    fn position_for(&self, offset: usize, upstream: bool) -> Point<Pixels> {
        let row = self.row_for(offset, upstream);
        point(self.x_in_row(row, offset), self.line_height * row as f32)
    }

    /// The offset closest to `x` in visual row `row_ix`, and whether it
    /// sits at the end of a soft-wrapped row.
    fn offset_in_row(&self, text: &str, row_ix: usize, x: Pixels) -> (usize, bool) {
        if self.placeholder {
            return (0, false);
        }
        let row = &self.rows[row_ix];
        let Some(line) = self.lines.get(row.line) else {
            return (row.start, false);
        };
        let layout = &line.unwrapped_layout;
        let row_x = layout.x_for_index(row.start - row.line_start);
        let local = layout.closest_index_for_x(row_x + x.max(px(0.)));
        let offset =
            floor_grapheme(text, (row.line_start + local).clamp(row.start, row.end)).max(row.start);
        (offset, row.soft_end && offset == row.end)
    }

    /// The offset under a window position.
    fn offset_for_position(&self, text: &str, position: Point<Pixels>) -> (usize, bool) {
        let local = position - self.bounds.origin;
        if local.y < px(0.) {
            return (0, false);
        }
        let row = (local.y / self.line_height).floor() as usize;
        if row >= self.rows.len() {
            return (text.len(), false);
        }
        self.offset_in_row(text, row, local.x)
    }
}

/// A visual row: a logical line or a soft-wrapped piece of one.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Row {
    /// Content range of the row, without the newline.
    start: usize,
    end: usize,
    /// Index of the logical line and its content offset.
    line: usize,
    line_start: usize,
    /// The row starts at a soft wrap (not at the start of a logical line).
    soft_start: bool,
    /// The row ends at a soft wrap (not at the end of a logical line).
    soft_end: bool,
}

/// Builds the visual rows of `text`. `wraps[i]` holds the byte offsets,
/// relative to logical line `i`, where that line soft-wraps.
fn build_rows(text: &str, wraps: &[Vec<usize>]) -> Vec<Row> {
    let mut rows = Vec::new();
    let mut line_start = 0;
    for (line, line_text) in text.split('\n').enumerate() {
        let line_wraps = wraps.get(line).map(Vec::as_slice).unwrap_or_default();
        let mut start = 0;
        let mut breaks = line_wraps
            .iter()
            .copied()
            .filter(|&wrap| wrap < line_text.len());
        loop {
            let next = breaks.find(|&wrap| wrap > start);
            let end = next.unwrap_or(line_text.len());
            rows.push(Row {
                start: line_start + start,
                end: line_start + end,
                line,
                line_start,
                soft_start: start > 0,
                soft_end: next.is_some(),
            });
            match next {
                Some(wrap) => start = wrap,
                None => break,
            }
        }
        line_start += line_text.len() + 1;
    }
    rows
}

/// The visual row showing a cursor at `offset`. A soft-wrap point belongs
/// to the lower row unless `upstream`.
fn row_index(rows: &[Row], offset: usize, upstream: bool) -> usize {
    let ix = rows
        .partition_point(|row| row.start <= offset)
        .saturating_sub(1);
    if upstream && ix > 0 && rows[ix].soft_start && rows[ix].start == offset {
        ix - 1
    } else {
        ix
    }
}

fn clamp_to_char_boundary(text: &str, offset: usize) -> usize {
    let mut offset = offset.min(text.len());
    while !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

fn prev_grapheme(text: &str, offset: usize) -> usize {
    let offset = clamp_to_char_boundary(text, offset);
    GraphemeCursor::new(offset, text.len(), true)
        .prev_boundary(text, 0)
        .ok()
        .flatten()
        .unwrap_or(0)
}

fn next_grapheme(text: &str, offset: usize) -> usize {
    let offset = clamp_to_char_boundary(text, offset);
    GraphemeCursor::new(offset, text.len(), true)
        .next_boundary(text, 0)
        .ok()
        .flatten()
        .unwrap_or(text.len())
}

/// The grapheme boundary at or before `offset`.
fn floor_grapheme(text: &str, offset: usize) -> usize {
    let offset = clamp_to_char_boundary(text, offset);
    if GraphemeCursor::new(offset, text.len(), true)
        .is_boundary(text, 0)
        .unwrap_or(true)
    {
        offset
    } else {
        prev_grapheme(text, offset)
    }
}

fn is_word(segment: &str) -> bool {
    segment.chars().any(char::is_alphanumeric)
}

/// Start of the word before `offset` (ctrl-left).
fn word_left(text: &str, offset: usize) -> usize {
    let offset = clamp_to_char_boundary(text, offset);
    text[..offset]
        .split_word_bound_indices()
        .rev()
        .find(|(_, segment)| is_word(segment))
        .map_or(0, |(ix, _)| ix)
}

/// End of the word after `offset` (ctrl-right).
fn word_right(text: &str, offset: usize) -> usize {
    let offset = clamp_to_char_boundary(text, offset);
    text[offset..]
        .split_word_bound_indices()
        .find(|(_, segment)| is_word(segment))
        .map_or(text.len(), |(ix, segment)| offset + ix + segment.len())
}

/// The word (or run of spaces or punctuation) at `offset`, preferring the
/// word to its left when `offset` is just past one (double-click).
fn word_range(text: &str, offset: usize) -> Range<usize> {
    let offset = clamp_to_char_boundary(text, offset);
    let mut before = None;
    for (ix, segment) in text.split_word_bound_indices() {
        let range = ix..ix + segment.len();
        if range.contains(&offset) {
            if offset == range.start
                && !is_word(segment)
                && let Some((before, true)) = before
            {
                return before;
            }
            return range;
        }
        before = Some((range, is_word(segment)));
    }
    before.map_or(offset..offset, |(range, _)| range)
}

/// The logical line containing `offset`, without its newline.
fn line_range(text: &str, offset: usize) -> Range<usize> {
    let offset = clamp_to_char_boundary(text, offset);
    let start = text[..offset].rfind('\n').map_or(0, |ix| ix + 1);
    let end = text[offset..]
        .find('\n')
        .map_or(text.len(), |ix| offset + ix);
    start..end
}

/// Turns CRLF and lone CR into LF.
fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

fn offset_from_utf16(text: &str, offset: usize) -> usize {
    let mut utf8_offset = 0;
    let mut utf16_count = 0;
    for ch in text.chars() {
        if utf16_count >= offset {
            break;
        }
        utf16_count += ch.len_utf16();
        utf8_offset += ch.len_utf8();
    }
    utf8_offset
}

fn offset_to_utf16(text: &str, offset: usize) -> usize {
    let mut utf16_offset = 0;
    let mut utf8_count = 0;
    for ch in text.chars() {
        if utf8_count >= offset {
            break;
        }
        utf8_count += ch.len_utf8();
        utf16_offset += ch.len_utf16();
    }
    utf16_offset
}

fn range_to_utf16(text: &str, range: &Range<usize>) -> Range<usize> {
    offset_to_utf16(text, range.start)..offset_to_utf16(text, range.end)
}

fn range_from_utf16(text: &str, range_utf16: &Range<usize>) -> Range<usize> {
    offset_from_utf16(text, range_utf16.start)..offset_from_utf16(text, range_utf16.end)
}

/// Text runs for the displayed text, underlining the IME's marked range.
fn text_runs(len: usize, base: TextRun, marked: Option<&Range<usize>>) -> Vec<TextRun> {
    let Some(marked) = marked.filter(|marked| marked.end <= len) else {
        return vec![TextRun { len, ..base }];
    };
    [
        TextRun {
            len: marked.start,
            ..base.clone()
        },
        TextRun {
            len: marked.end - marked.start,
            underline: Some(UnderlineStyle {
                color: Some(base.color),
                thickness: px(1.0),
                wavy: false,
            }),
            ..base.clone()
        },
        TextRun {
            len: len - marked.end,
            ..base
        },
    ]
    .into_iter()
    .filter(|run| run.len > 0)
    .collect()
}

fn visual_rows(lines: &[WrappedLine]) -> usize {
    lines
        .iter()
        .map(|line| line.wrap_boundaries().len() + 1)
        .sum::<usize>()
        .max(1)
}

/// Paints the text of a [`TextArea`].
struct TextElement {
    input: Entity<TextArea>,
}

/// What [`TextElement::request_layout`] shaped with.
struct Shaping {
    text: SharedString,
    runs: Vec<TextRun>,
    font_size: Pixels,
    line_height: Pixels,
}

impl Shaping {
    fn shape(&self, window: &Window, wrap_width: Option<Pixels>) -> Vec<WrappedLine> {
        window
            .text_system()
            .shape_text(
                self.text.clone(),
                self.font_size,
                &self.runs,
                wrap_width.map(|width| width.max(px(1.))),
                None,
            )
            .map(|lines| lines.into_iter().collect())
            .unwrap_or_default()
    }
}

struct PrepaintState {
    layout: Option<Layout>,
    cursor: Option<PaintQuad>,
    selections: Vec<PaintQuad>,
}

impl IntoElement for TextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextElement {
    type RequestLayoutState = Shaping;
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let input = self.input.read(cx);
        let style = window.text_style();
        let (text, color) = if input.content.is_empty() {
            (
                input.placeholder.clone(),
                style.color.opacity(crate::PLACEHOLDER_OPACITY),
            )
        } else {
            (input.content.clone(), style.color)
        };
        let base = TextRun {
            len: text.len(),
            font: style.font(),
            color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let marked = input
            .marked_range
            .as_ref()
            .filter(|_| !input.content.is_empty());
        let shaping = Shaping {
            runs: text_runs(text.len(), base, marked),
            text,
            font_size: style.font_size.to_pixels(window.rem_size()),
            line_height: window.line_height(),
        };

        let mut layout_style = Style::default();
        layout_style.size.width = relative(1.).into();
        let measure = Shaping {
            text: shaping.text.clone(),
            runs: shaping.runs.clone(),
            ..shaping
        };
        let layout_id =
            window.request_measured_layout(layout_style, move |known, available, window, _cx| {
                let wrap_width = known.width.or(match available.width {
                    AvailableSpace::Definite(width) => Some(width),
                    _ => None,
                });
                let lines = measure.shape(window, wrap_width);
                let width = match (wrap_width, available.width) {
                    (Some(width), _) => width,
                    // Wrapping text can shrink to nothing.
                    (None, AvailableSpace::MinContent) => px(0.),
                    (None, _) => lines
                        .iter()
                        .map(|line| line.width())
                        .fold(px(0.), Pixels::max),
                };
                size(width, measure.line_height * visual_rows(&lines) as f32)
            });
        (layout_id, shaping)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        shaping: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let input = self.input.read(cx);
        let lines = shaping.shape(window, Some(bounds.size.width));
        let placeholder = input.content.is_empty();
        let rows = if placeholder {
            build_rows("", &[])
        } else {
            let wraps: Vec<Vec<usize>> = lines
                .iter()
                .map(|line| {
                    line.wrap_boundaries()
                        .iter()
                        .map(|boundary| {
                            line.runs()[boundary.run_ix].glyphs[boundary.glyph_ix].index
                        })
                        .collect()
                })
                .collect();
            build_rows(&input.content, &wraps)
        };
        let layout = Layout {
            lines,
            rows,
            bounds,
            line_height: shaping.line_height,
            placeholder,
        };

        let line_height = layout.line_height;
        let selected = input.selected_range.clone();
        let mut selections = Vec::new();
        if !selected.is_empty() {
            // Selected newlines show as a short stub past the line's end.
            let newline_width = line_height * 0.3;
            for (ix, row) in layout.rows.iter().enumerate() {
                let start = selected.start.max(row.start);
                let end = selected.end.min(row.end);
                let past_end = selected.end > row.end && !row.soft_end;
                if start > end || (start == end && !past_end) {
                    continue;
                }
                let left = layout.x_in_row(ix, start);
                let mut right = layout.x_in_row(ix, end);
                if past_end {
                    right += newline_width;
                }
                let top = bounds.top() + line_height * ix as f32;
                selections.push(fill(
                    Bounds::from_corners(
                        point(bounds.left() + left, top),
                        point(bounds.left() + right, top + line_height),
                    ),
                    input.accent.opacity(0.3),
                ));
            }
        }
        let cursor = selected.is_empty().then(|| {
            let position = layout.position_for(input.cursor_offset(), input.upstream);
            fill(
                Bounds::new(bounds.origin + position, size(px(2.), line_height)),
                input.accent,
            )
        });
        PrepaintState {
            layout: Some(layout),
            cursor,
            selections,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _shaping: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.input.clone()),
            cx,
        );
        window.on_mouse_event({
            let input = self.input.clone();
            move |event: &MouseMoveEvent, phase, _window, cx| {
                if phase == DispatchPhase::Bubble && input.read(cx).is_selecting {
                    input.update(cx, |input, cx| input.drag_to(event, cx));
                }
            }
        });
        window.on_mouse_event({
            let input = self.input.clone();
            move |event: &MouseUpEvent, phase, _window, cx| {
                if phase == DispatchPhase::Bubble
                    && event.button == MouseButton::Left
                    && input.read(cx).is_selecting
                {
                    input.update(cx, |input, _cx| input.is_selecting = false);
                }
            }
        });

        let Some(layout) = prepaint.layout.take() else {
            return;
        };
        for selection in prepaint.selections.drain(..) {
            window.paint_quad(selection);
        }
        let mut top = bounds.top();
        for line in &layout.lines {
            // A failed paint only loses this frame's text.
            let _ = line.paint(
                point(bounds.left(), top),
                layout.line_height,
                TextAlign::Left,
                None,
                window,
                cx,
            );
            top += layout.line_height * (line.wrap_boundaries().len() + 1) as f32;
        }
        if focus_handle.is_focused(window)
            && let Some(cursor) = prepaint.cursor.take()
        {
            window.paint_quad(cursor);
        }
        self.input.update(cx, |input, _cx| {
            input.last_layout = Some(layout);
        });
    }
}

impl Render for TextArea {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .min_w_0()
            .key_context(TEXT_AREA_CONTEXT)
            .track_focus(&self.focus_handle(cx))
            .cursor(CursorStyle::IBeam)
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::delete_word_left))
            .on_action(cx.listener(Self::delete_word_right))
            .on_action(cx.listener(Self::newline))
            .on_action(cx.listener(Self::left))
            .on_action(cx.listener(Self::right))
            .on_action(cx.listener(Self::up))
            .on_action(cx.listener(Self::down))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::select_right))
            .on_action(cx.listener(Self::select_up))
            .on_action(cx.listener(Self::select_down))
            .on_action(cx.listener(Self::word_left))
            .on_action(cx.listener(Self::word_right))
            .on_action(cx.listener(Self::select_word_left))
            .on_action(cx.listener(Self::select_word_right))
            .on_action(cx.listener(Self::home))
            .on_action(cx.listener(Self::end))
            .on_action(cx.listener(Self::select_home))
            .on_action(cx.listener(Self::select_end))
            .on_action(cx.listener(Self::doc_start))
            .on_action(cx.listener(Self::doc_end))
            .on_action(cx.listener(Self::select_doc_start))
            .on_action(cx.listener(Self::select_doc_end))
            .on_action(cx.listener(Self::page_up))
            .on_action(cx.listener(Self::page_down))
            .on_action(cx.listener(Self::select_page_up))
            .on_action(cx.listener(Self::select_page_down))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::paste))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::copy))
            .on_action(cx.listener(Self::submit))
            .on_action(cx.listener(Self::cancel))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .child(TextElement { input: cx.entity() })
    }
}

impl Focusable for TextArea {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(start: usize, end: usize, line: usize, line_start: usize) -> Row {
        Row {
            start,
            end,
            line,
            line_start,
            soft_start: false,
            soft_end: false,
        }
    }

    #[test]
    fn rows_follow_hard_and_soft_breaks() {
        // "hello world" wraps before "world"; "" is an empty line.
        let text = "hello world\n\nabc";
        let rows = build_rows(text, &[vec![6], vec![], vec![]]);
        assert_eq!(
            rows,
            vec![
                Row {
                    soft_end: true,
                    ..row(0, 6, 0, 0)
                },
                Row {
                    soft_start: true,
                    ..row(6, 11, 0, 0)
                },
                row(12, 12, 1, 12),
                row(13, 16, 2, 13),
            ]
        );
    }

    #[test]
    fn rows_of_empty_and_trailing_newline() {
        assert_eq!(build_rows("", &[]), vec![row(0, 0, 0, 0)]);
        assert_eq!(
            build_rows("a\n", &[]),
            vec![row(0, 1, 0, 0), row(2, 2, 1, 2)]
        );
    }

    #[test]
    fn rows_ignore_bogus_wraps() {
        let rows = build_rows("abcd", &[vec![0, 2, 2, 9]]);
        assert_eq!(rows.len(), 2);
        assert_eq!((rows[0].start, rows[0].end), (0, 2));
        assert_eq!((rows[1].start, rows[1].end), (2, 4));
    }

    #[test]
    fn row_index_resolves_wrap_points() {
        let text = "hello world\nabc";
        let rows = build_rows(text, &[vec![6]]);
        assert_eq!(row_index(&rows, 0, false), 0);
        assert_eq!(row_index(&rows, 5, false), 0);
        // The wrap point belongs to the lower row unless upstream.
        assert_eq!(row_index(&rows, 6, false), 1);
        assert_eq!(row_index(&rows, 6, true), 0);
        // End of a hard line stays on its row.
        assert_eq!(row_index(&rows, 11, false), 1);
        assert_eq!(row_index(&rows, 11, true), 1);
        assert_eq!(row_index(&rows, 12, false), 2);
        // Upstream at a hard line start does nothing.
        assert_eq!(row_index(&rows, 12, true), 2);
        assert_eq!(row_index(&rows, 15, false), 2);
    }

    #[test]
    fn grapheme_movement_with_multibyte_and_emoji() {
        // "é" as e + combining acute, a family emoji (ZWJ sequence), "ß".
        let text = "e\u{301}👨‍👩‍👧ß";
        let family = "👨‍👩‍👧".len();
        assert_eq!(next_grapheme(text, 0), 3);
        assert_eq!(next_grapheme(text, 3), 3 + family);
        assert_eq!(next_grapheme(text, 3 + family), text.len());
        assert_eq!(next_grapheme(text, text.len()), text.len());
        assert_eq!(prev_grapheme(text, text.len()), 3 + family);
        assert_eq!(prev_grapheme(text, 3 + family), 3);
        assert_eq!(prev_grapheme(text, 3), 0);
        assert_eq!(prev_grapheme(text, 0), 0);
        // Inside a cluster, snap to its start.
        assert_eq!(floor_grapheme(text, 1), 0);
        assert_eq!(floor_grapheme(text, 3 + 4), 3);
        assert_eq!(floor_grapheme(text, 3), 3);
    }

    #[test]
    fn clamps_to_char_boundaries() {
        let text = "aé€";
        assert_eq!(clamp_to_char_boundary(text, 0), 0);
        assert_eq!(clamp_to_char_boundary(text, 2), 1);
        assert_eq!(clamp_to_char_boundary(text, 4), 3);
        assert_eq!(clamp_to_char_boundary(text, 5), 3);
        assert_eq!(clamp_to_char_boundary(text, 99), text.len());
    }

    #[test]
    fn word_motion() {
        let text = "Hello, wörld!\n  foo_bar baz";
        assert_eq!(word_right(text, 0), 5);
        assert_eq!(word_right(text, 5), 13); // past "wörld"
        assert_eq!(word_right(text, 13), 24); // "foo_bar" across the newline
        assert_eq!(word_right(text, 25), text.len());
        assert_eq!(word_right(text, text.len()), text.len());
        assert_eq!(word_left(text, text.len()), 25);
        assert_eq!(word_left(text, 25), 17);
        assert_eq!(word_left(text, 17), 7);
        assert_eq!(word_left(text, 10), 7); // mid-word goes to its start
        assert_eq!(word_left(text, 7), 0);
        assert_eq!(word_left(text, 0), 0);
    }

    #[test]
    fn word_range_for_double_click() {
        let text = "one two  three";
        assert_eq!(word_range(text, 0), 0..3);
        assert_eq!(word_range(text, 5), 4..7);
        // Just past a word selects that word.
        assert_eq!(word_range(text, 3), 0..3);
        assert_eq!(word_range(text, 7), 4..7);
        // Inside a run of spaces selects the spaces.
        assert_eq!(word_range(text, 8), 7..9);
        assert_eq!(word_range(text, text.len()), 9..14);
        assert_eq!(word_range("", 0), 0..0);
        let emoji = "hi 😀😀 yo";
        let range = word_range(emoji, 3);
        assert!(emoji.is_char_boundary(range.start) && emoji.is_char_boundary(range.end));
    }

    #[test]
    fn logical_line_ranges() {
        let text = "ab\n\ncd";
        assert_eq!(line_range(text, 0), 0..2);
        assert_eq!(line_range(text, 2), 0..2);
        assert_eq!(line_range(text, 3), 3..3);
        assert_eq!(line_range(text, 4), 4..6);
        assert_eq!(line_range(text, 6), 4..6);
    }

    #[test]
    fn utf16_round_trip() {
        let text = "a😀é\nb";
        assert_eq!(offset_to_utf16(text, 1), 1);
        assert_eq!(offset_to_utf16(text, 5), 3);
        assert_eq!(offset_to_utf16(text, 7), 4);
        assert_eq!(offset_to_utf16(text, text.len()), 6);
        for offset in [0, 1, 5, 7, 8, text.len()] {
            assert_eq!(
                offset_from_utf16(text, offset_to_utf16(text, offset)),
                offset
            );
        }
        assert_eq!(range_from_utf16(text, &(1..3)), 1..5);
    }

    #[test]
    fn offsets_past_the_end_are_clamped() {
        // A layout from the previous frame may name offsets the text no
        // longer has.
        assert_eq!(floor_grapheme("ab", 7), 2);
        assert_eq!(prev_grapheme("ab", 7), 1);
        assert_eq!(next_grapheme("ab", 7), 2);
        assert_eq!(word_left("ab cd", 99), 3);
        assert_eq!(word_right("ab cd", 99), 5);
        assert_eq!(line_range("ab\ncd", 99), 3..5);
    }

    #[test]
    fn pasted_newlines_are_normalized() {
        assert_eq!(normalize_newlines("a\r\nb\rc\nd"), "a\nb\nc\nd");
    }

    #[test]
    fn marked_text_splits_runs() {
        let base = TextRun {
            len: 0,
            font: gpui::font("Sans"),
            color: gpui::black(),
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let runs = text_runs(10, base.clone(), Some(&(2..5)));
        assert_eq!(
            runs.iter().map(|run| run.len).collect::<Vec<_>>(),
            vec![2, 3, 5]
        );
        assert!(runs[1].underline.is_some());
        assert_eq!(text_runs(10, base.clone(), None).len(), 1);
        // A stale marked range past the end is ignored.
        assert_eq!(text_runs(3, base, Some(&(2..5)))[0].len, 3);
    }
}
