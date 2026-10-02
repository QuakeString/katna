// SPDX-License-Identifier: GPL-3.0-or-later

//! A single-line text input with IME support, selection and clipboard,
//! built on GPUI's input handler (after GPUI's `input` example).
//!
//! The input draws only its text, selection and cursor; the parent gives it
//! a frame and the text style (font, size, color), which it inherits.

use std::collections::HashSet;
use std::ops::Range;
use std::sync::Arc;

use crate::scale::px;
use gpui::{
    App, Bounds, ClipboardItem, Context, CursorStyle, ElementId, ElementInputHandler, Entity,
    EntityInputHandler, EventEmitter, FocusHandle, Focusable, GlobalElementId, Hsla, KeyBinding,
    LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PaintQuad, Pixels, Point,
    ShapedLine, SharedString, Style, Task, TextRun, UTF16Selection, UnderlineStyle, Window,
    actions, div, fill, point, prelude::*, relative, size,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::rich::{GRAMMAR_WAIT, GrammarCheck, GrammarFix, GrammarIssue, HINT_WAIT};

actions!(
    text_input,
    [
        Backspace,
        Delete,
        Left,
        Right,
        SelectLeft,
        SelectRight,
        SelectAll,
        Home,
        End,
        Paste,
        Cut,
        Copy,
        Submit,
        Cancel,
        Up,
        Down,
    ]
);

/// Key context of a focused [`TextInput`].
pub const KEY_CONTEXT: &str = "TextInput";

/// Binds the editing keys. Call once at startup.
pub fn bind_keys(cx: &mut App) {
    let context = Some(KEY_CONTEXT);
    cx.bind_keys([
        KeyBinding::new("backspace", Backspace, context),
        KeyBinding::new("delete", Delete, context),
        KeyBinding::new("left", Left, context),
        KeyBinding::new("right", Right, context),
        KeyBinding::new("shift-left", SelectLeft, context),
        KeyBinding::new("shift-right", SelectRight, context),
        KeyBinding::new("home", Home, context),
        KeyBinding::new("end", End, context),
        KeyBinding::new("ctrl-a", SelectAll, context),
        KeyBinding::new("ctrl-v", Paste, context),
        KeyBinding::new("shift-insert", Paste, context),
        KeyBinding::new("ctrl-c", Copy, context),
        KeyBinding::new("ctrl-x", Cut, context),
        KeyBinding::new("enter", Submit, context),
        KeyBinding::new("escape", Cancel, context),
        // Handled by the input only with a stepper (a time field); else
        // for a parent's list of suggestions, going on to the next binding.
        KeyBinding::new("up", Up, context),
        KeyBinding::new("down", Down, context),
    ]);
}

/// What a [`TextInput`] tells its parent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
    /// The text changed.
    Changed,
    /// Enter was pressed.
    Submit,
    /// Escape was pressed.
    Cancel,
}

/// A right click on a grammar mistake in a [`TextInput`] that checks
/// grammar, or the pointer resting or a left click on one: the owner shows
/// the fixes.
#[derive(Debug, Clone, PartialEq)]
pub struct InputGrammarMenu {
    pub position: Point<Pixels>,
    pub issue: GrammarIssue,
    /// For a rest or a left click, the mistake's window bounds: the card
    /// shows under them and closes when the pointer leaves both.
    pub word: Option<Bounds<Pixels>>,
}

/// What Up (`1`) and Down (`-1`) do in a field like a time: given the
/// text and the cursor, the new text and cursor, or `None` to leave it.
pub type Stepper = Arc<dyn Fn(&str, usize, i32) -> Option<(String, usize)>>;

/// A single-line text input. Create it with [`TextInput::new`] inside
/// `cx.new` and render the entity as a child.
pub struct TextInput {
    focus_handle: FocusHandle,
    content: SharedString,
    placeholder: SharedString,
    accent: Hsla,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    last_layout: Option<ShapedLine>,
    last_bounds: Option<Bounds<Pixels>>,
    /// How far the text is scrolled left so the cursor stays visible.
    scroll_x: Pixels,
    is_selecting: bool,
    /// Draws a dot for each character, for passwords.
    masked: bool,
    /// Text shorter than the box sits in its middle.
    centered: bool,
    grammar: Option<Arc<dyn GrammarCheck>>,
    /// The underline of grammar mistakes.
    grammar_color: Hsla,
    /// The text last checked, and its mistakes.
    grammar_found: Option<(SharedString, Vec<GrammarIssue>)>,
    grammar_asked: Option<SharedString>,
    grammar_task: Option<Task<()>>,
    /// Mistakes the user chose to ignore: the words and the message.
    grammar_ignored: HashSet<(String, String)>,
    /// The mistake under the pointer, and the wait before its fixes show.
    hover: Option<Range<usize>>,
    hover_task: Option<Task<()>>,
    /// The mistake a left click went down on.
    clicked: Option<Range<usize>>,
    /// What Up and Down change, if they change the text.
    stepper: Option<Stepper>,
}

/// What a masked input shows for each character.
const MASK: char = '\u{2022}';

impl EventEmitter<InputEvent> for TextInput {}
impl EventEmitter<InputGrammarMenu> for TextInput {}

impl TextInput {
    pub fn new(placeholder: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        Self {
            // Tab moves between fields (the app binds it to focus_next).
            focus_handle: cx.focus_handle().tab_stop(true),
            content: SharedString::default(),
            placeholder: placeholder.into(),
            accent: gpui::blue(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            scroll_x: px(0.0),
            is_selecting: false,
            masked: false,
            centered: false,
            grammar: None,
            grammar_color: gpui::blue(),
            grammar_found: None,
            grammar_asked: None,
            grammar_task: None,
            grammar_ignored: HashSet::new(),
            hover: None,
            hover_task: None,
            clicked: None,
            stepper: None,
        }
    }

    /// Makes Up and Down change the text with `stepper`, like the part of
    /// a time the cursor is in; with `None` they go to the parent.
    pub fn set_stepper(&mut self, stepper: Option<Stepper>) {
        self.stepper = stepper;
    }

    fn step(&mut self, by: i32, cx: &mut Context<Self>) {
        let Some(stepper) = &self.stepper else {
            return;
        };
        let Some((text, cursor)) = stepper(&self.content, self.cursor_offset(), by) else {
            return;
        };
        let cursor = cursor.min(text.len());
        let changed = *text != *self.content;
        self.content = text.into();
        self.selected_range = cursor..cursor;
        self.selection_reversed = false;
        self.marked_range = None;
        cx.notify();
        if changed {
            cx.emit(InputEvent::Changed);
        }
    }

    /// Turns grammar checking on with `checker`, underlining mistakes in
    /// `color`, or off.
    pub fn set_grammar_check(
        &mut self,
        checker: Option<Arc<dyn GrammarCheck>>,
        color: Hsla,
        cx: &mut Context<Self>,
    ) {
        self.grammar = checker;
        self.grammar_color = color;
        self.grammar_found = None;
        self.grammar_asked = None;
        self.grammar_task = None;
        cx.notify();
    }

    /// Checks the text once typing pauses. Called on every render.
    fn request_grammar(&mut self, cx: &mut Context<Self>) {
        let Some(checker) = self.grammar.clone() else {
            return;
        };
        let text = self.content.clone();
        let checked = self.grammar_found.as_ref().is_some_and(|(t, _)| *t == text);
        if checked || self.grammar_asked.as_ref() == Some(&text) || self.masked {
            return;
        }
        self.grammar_asked = Some(text.clone());
        self.grammar_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(GRAMMAR_WAIT).await;
            let found = cx
                .background_executor()
                .spawn({
                    let text = text.clone();
                    async move { checker.check(&text) }
                })
                .await;
            this.update(cx, |this, cx| {
                this.grammar_found = Some((text, found));
                this.grammar_asked = None;
                this.grammar_task = None;
                cx.notify();
            })
            .ok();
        }));
    }

    /// Mistakes in the text as it is, without the ignored ones.
    fn grammar_issues(&self) -> Vec<&GrammarIssue> {
        let Some((text, issues)) = &self.grammar_found else {
            return Vec::new();
        };
        if self.grammar.is_none() || *text != self.content {
            return Vec::new();
        }
        issues
            .iter()
            .filter(|issue| {
                self.content.get(issue.range.clone()).is_some_and(|words| {
                    !self
                        .grammar_ignored
                        .contains(&(words.to_owned(), issue.message.clone()))
                })
            })
            .collect()
    }

    /// Fixes grammar mistake `issue` with `fix`.
    pub fn fix_grammar(&mut self, issue: &GrammarIssue, fix: &GrammarFix, cx: &mut Context<Self>) {
        let range = issue.range.clone();
        if self.content.get(range.clone()).is_none() {
            return;
        }
        let mut text = self.content.to_string();
        text.replace_range(range.clone(), &fix.replacement);
        self.set_text(text, cx);
        let at = range.start + fix.replacement.len();
        self.move_to(at, cx);
    }

    /// Stops marking `issue`'s words with its message.
    pub fn ignore_grammar(&mut self, issue: &GrammarIssue, cx: &mut Context<Self>) {
        if let Some(words) = self.content.get(issue.range.clone()) {
            self.grammar_ignored
                .insert((words.to_owned(), issue.message.clone()));
            cx.notify();
        }
    }

    fn on_right_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.grammar.is_none() {
            return;
        }
        window.focus(&self.focus_handle, cx);
        let at = self.index_for_mouse_position(event.position);
        let issue = self
            .grammar_issues()
            .into_iter()
            .find(|issue| issue.range.start <= at && at <= issue.range.end)
            .cloned();
        if let Some(issue) = issue {
            self.move_to(at, cx);
            cx.emit(InputGrammarMenu {
                position: event.position,
                issue,
                word: None,
            });
        }
    }

    /// The grammar mistake under window point `p`, and its bounds.
    fn mistake_at(&self, p: Point<Pixels>) -> Option<(GrammarIssue, Bounds<Pixels>)> {
        if self.grammar.is_none() || self.masked {
            return None;
        }
        let (bounds, line) = (self.last_bounds?, self.last_layout.as_ref()?);
        let at = self.index_for_mouse_position(p);
        let issue = self
            .grammar_issues()
            .into_iter()
            .find(|issue| issue.range.start <= at && at <= issue.range.end)?;
        let left = bounds.left() - self.scroll_x;
        let x = |offset| left + line.x_for_index(self.display_offset(offset));
        let word = Bounds::from_corners(
            point(x(issue.range.start), bounds.top()),
            point(x(issue.range.end), bounds.bottom()),
        );
        word.contains(&p).then(|| (issue.clone(), word))
    }

    /// Resting the pointer on a grammar mistake shows its fixes after a
    /// wait.
    fn hover_mistake(&mut self, p: Point<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        let key = self.mistake_at(p).map(|(issue, _)| issue.range);
        if key == self.hover {
            return;
        }
        self.hover = key.clone();
        self.hover_task = key.map(|key| {
            cx.spawn_in(window, async move |this, cx| {
                cx.background_executor().timer(HINT_WAIT).await;
                this.update_in(cx, |this, window, cx| {
                    this.hover_task = None;
                    let position = window.mouse_position();
                    match this.mistake_at(position) {
                        Some((issue, word)) if issue.range == key => cx.emit(InputGrammarMenu {
                            position,
                            issue,
                            word: Some(word),
                        }),
                        // The pointer left meanwhile.
                        _ => this.hover = None,
                    }
                })
                .ok();
            })
        });
    }

    /// Forgets the mistake the pointer was on, so resting on it again shows
    /// its fixes again (the owner calls it on closing the card).
    pub fn end_hint(&mut self) {
        self.hover = None;
        self.hover_task = None;
    }

    /// Whether the caret is at the start of the text, nothing selected.
    pub fn caret_at_start(&self) -> bool {
        self.selected_range == (0..0)
    }

    pub fn text(&self) -> &str {
        &self.content
    }

    /// Replaces the text and puts the cursor at its end. Emits
    /// [`InputEvent::Changed`] if the text changed.
    pub fn set_text(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        let text = text.into();
        let changed = text != self.content;
        self.content = text;
        self.selected_range = self.content.len()..self.content.len();
        self.selection_reversed = false;
        self.marked_range = None;
        cx.notify();
        if changed {
            cx.emit(InputEvent::Changed);
        }
    }

    /// Puts the cursor at the start, showing the text from its beginning:
    /// a long title opened to read, not to type at its end.
    pub fn caret_to_start(&mut self, cx: &mut Context<Self>) {
        self.selected_range = 0..0;
        self.selection_reversed = false;
        self.scroll_x = px(0.0);
        cx.notify();
    }

    /// Shows a dot for each character instead of the text, and keeps the
    /// text off the clipboard: for passwords.
    pub fn set_masked(&mut self, masked: bool, cx: &mut Context<Self>) {
        if self.masked != masked {
            self.masked = masked;
            cx.notify();
        }
    }

    /// The offset in the drawn text of `offset` in the content.
    fn display_offset(&self, offset: usize) -> usize {
        if self.masked {
            self.content[..offset].chars().count() * MASK.len_utf8()
        } else {
            offset
        }
    }

    /// The offset in the content of `offset` in the drawn text.
    fn content_offset(&self, offset: usize) -> usize {
        if self.masked {
            self.content
                .char_indices()
                .nth(offset / MASK.len_utf8())
                .map_or(self.content.len(), |(ix, _)| ix)
        } else {
            offset
        }
    }

    pub fn set_placeholder(&mut self, placeholder: impl Into<SharedString>) {
        self.placeholder = placeholder.into();
    }

    /// Centres text shorter than the box, for short fields such as a page
    /// number.
    pub fn set_centered(&mut self, centered: bool) {
        self.centered = centered;
    }

    /// Color of the cursor and (translucent) of the selection.
    pub fn set_accent(&mut self, accent: Hsla) {
        self.accent = accent;
    }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx)
        }
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), cx);
        } else {
            self.move_to(self.selected_range.end, cx)
        }
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.select_all_text(cx)
    }

    /// Selects bytes `range` of the text (clamped to it).
    pub fn select_range(&mut self, range: Range<usize>, cx: &mut Context<Self>) {
        let end = range.end.min(self.content.len());
        self.move_to(range.start.min(end), cx);
        self.select_to(end, cx)
    }

    /// Selects all the text, so typing replaces it.
    pub fn select_all_text(&mut self, cx: &mut Context<Self>) {
        self.move_to(0, cx);
        self.select_to(self.content.len(), cx)
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
    }

    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.content.len(), cx);
    }

    fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let prev = self.previous_boundary(self.cursor_offset());
            if self.cursor_offset() == prev {
                return;
            }
            self.select_to(prev, cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let next = self.next_boundary(self.cursor_offset());
            if self.cursor_offset() == next {
                return;
            }
            self.select_to(next, cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn submit(&mut self, _: &Submit, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(InputEvent::Submit);
    }

    fn cancel(&mut self, _: &Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(InputEvent::Cancel);
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.replace_text_in_range(None, &text.replace(['\r', '\n'], " "), window, cx);
        }
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() && !self.masked {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
        }
    }

    fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() && !self.masked {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
            self.replace_text_in_range(None, "", window, cx)
        }
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle, cx);
        self.is_selecting = true;
        self.clicked = (!event.modifiers.shift)
            .then(|| self.mistake_at(event.position))
            .flatten()
            .map(|(issue, _)| issue.range);
        if event.modifiers.shift {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        } else {
            self.move_to(self.index_for_mouse_position(event.position), cx)
        }
    }

    /// A left click that came up where it went down on a grammar mistake
    /// shows its fixes.
    fn on_mouse_up(&mut self, event: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.is_selecting = false;
        let Some(clicked) = self.clicked.take() else {
            return;
        };
        if !self.selected_range.is_empty() {
            return;
        }
        if let Some((issue, word)) = self.mistake_at(event.position)
            && issue.range == clicked
        {
            self.hover = Some(clicked);
            self.hover_task = None;
            cx.emit(InputGrammarMenu {
                position: event.position,
                issue,
                word: Some(word),
            });
        }
    }

    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_selecting {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        } else if event.pressed_button.is_none() {
            self.hover_mistake(event.position, window, cx);
        }
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        cx.notify()
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }
        let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_layout.as_ref())
        else {
            return 0;
        };
        if position.y < bounds.top() {
            return 0;
        }
        if position.y > bounds.bottom() {
            return self.content.len();
        }
        self.content_offset(line.closest_index_for_x(position.x - bounds.left() + self.scroll_x))
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset
        } else {
            self.selected_range.end = offset
        };
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        cx.notify()
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;
        for ch in self.content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }
        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;
        for ch in self.content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }
        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .rev()
            .find_map(|(idx, _)| (idx < offset).then_some(idx))
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .find_map(|(idx, _)| (idx > offset).then_some(idx))
            .unwrap_or(self.content.len())
    }

    fn replace(&mut self, range: Range<usize>, new_text: &str, cx: &mut Context<Self>) {
        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                .into();
        cx.notify();
        cx.emit(InputEvent::Changed);
    }
}

impl EntityInputHandler for TextInput {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
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
            .map(|range| self.range_to_utf16(range))
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
            .map(|range_utf16| self.range_from_utf16(range_utf16))
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
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());
        self.marked_range =
            (!new_text.is_empty()).then(|| range.start..range.start + new_text.len());
        self.replace(range.clone(), new_text, cx);
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.start)
            .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let last_layout = self.last_layout.as_ref()?;
        let range = self.range_from_utf16(&range_utf16);
        let left = bounds.left() - self.scroll_x;
        Some(Bounds::from_corners(
            point(
                left + last_layout.x_for_index(self.display_offset(range.start)),
                bounds.top(),
            ),
            point(
                left + last_layout.x_for_index(self.display_offset(range.end)),
                bounds.bottom(),
            ),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let line_point = self.last_bounds?.localize(&point)?;
        let last_layout = self.last_layout.as_ref()?;
        let utf8_index = last_layout.index_for_x(line_point.x + self.scroll_x)?;
        Some(self.offset_to_utf16(self.content_offset(utf8_index)))
    }
}

/// Paints the text of a [`TextInput`].
struct TextElement {
    input: Entity<TextInput>,
}

struct PrepaintState {
    line: Option<ShapedLine>,
    scroll_x: Pixels,
    cursor: Option<PaintQuad>,
    selection: Option<PaintQuad>,
}

impl IntoElement for TextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextElement {
    type RequestLayoutState = ();
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
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let input = self.input.read(cx);
        let content = if input.masked {
            SharedString::from(MASK.to_string().repeat(input.content.chars().count()))
        } else {
            input.content.clone()
        };
        let selected_range = input.display_offset(input.selected_range.start)
            ..input.display_offset(input.selected_range.end);
        let cursor = input.display_offset(input.cursor_offset());
        let accent = input.accent;
        let style = window.text_style();

        let (display_text, text_color) = if content.is_empty() {
            (
                input.placeholder.clone(),
                style.color.opacity(crate::PLACEHOLDER_OPACITY),
            )
        } else {
            (content, style.color)
        };
        let run = TextRun {
            len: display_text.len(),
            font: style.font(),
            color: text_color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let marked_range = input
            .marked_range
            .as_ref()
            .map(|range| input.display_offset(range.start)..input.display_offset(range.end));
        let grammar: Vec<Range<usize>> = if input.content.is_empty() || input.masked {
            Vec::new()
        } else {
            input
                .grammar_issues()
                .into_iter()
                .map(|issue| issue.range.clone())
                .filter(|range| {
                    marked_range
                        .as_ref()
                        .is_none_or(|m| range.end <= m.start || m.end <= range.start)
                })
                .collect()
        };
        let runs = if !grammar.is_empty() {
            let underline = UnderlineStyle {
                color: Some(input.grammar_color),
                thickness: px(2.0),
                wavy: false,
            };
            let mut runs = Vec::new();
            let mut at = 0;
            for range in grammar {
                if range.start < at || range.end > display_text.len() {
                    continue;
                }
                runs.push(TextRun {
                    len: range.start - at,
                    ..run.clone()
                });
                runs.push(TextRun {
                    len: range.end - range.start,
                    underline: Some(underline),
                    ..run.clone()
                });
                at = range.end;
            }
            runs.push(TextRun {
                len: display_text.len() - at,
                ..run
            });
            runs.into_iter().filter(|run| run.len > 0).collect()
        } else if let Some(marked_range) = marked_range.as_ref() {
            vec![
                TextRun {
                    len: marked_range.start,
                    ..run.clone()
                },
                TextRun {
                    len: marked_range.end - marked_range.start,
                    underline: Some(UnderlineStyle {
                        color: Some(run.color),
                        thickness: px(1.0),
                        wavy: false,
                    }),
                    ..run.clone()
                },
                TextRun {
                    len: display_text.len() - marked_range.end,
                    ..run
                },
            ]
            .into_iter()
            .filter(|run| run.len > 0)
            .collect()
        } else {
            vec![run]
        };

        let font_size = style.font_size.to_pixels(window.rem_size());
        let line = window
            .text_system()
            .shape_line(display_text, font_size, &runs, None);

        // Keep the cursor in view: scroll just enough, never past the start.
        let cursor_x = if input.content.is_empty() {
            px(0.0)
        } else {
            line.x_for_index(cursor)
        };
        let width = bounds.size.width - px(2.0);
        let mut scroll_x = input.scroll_x;
        if cursor_x - scroll_x > width {
            scroll_x = cursor_x - width;
        } else if cursor_x < scroll_x {
            scroll_x = cursor_x;
        }
        if line.width - scroll_x < width {
            scroll_x = (line.width - width).max(px(0.0));
        }
        // Centred text scrolls "left" by a negative amount, so clicks and the
        // cursor follow it without anything else changing.
        if input.centered && line.width < width {
            scroll_x = (line.width - width) / 2.0;
        }
        let left = bounds.left() - scroll_x;

        let (selection, cursor) = if selected_range.is_empty() {
            (
                None,
                Some(fill(
                    Bounds::new(
                        point(left + cursor_x, bounds.top()),
                        size(px(2.), bounds.bottom() - bounds.top()),
                    ),
                    accent,
                )),
            )
        } else {
            (
                Some(fill(
                    Bounds::from_corners(
                        point(left + line.x_for_index(selected_range.start), bounds.top()),
                        point(left + line.x_for_index(selected_range.end), bounds.bottom()),
                    ),
                    accent.opacity(0.3),
                )),
                None,
            )
        };
        PrepaintState {
            line: Some(line),
            scroll_x,
            cursor,
            selection,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
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
        let Some(line) = prepaint.line.take() else {
            return;
        };
        let scroll_x = prepaint.scroll_x;
        window.with_content_mask(
            Some(gpui::ContentMask {
                bounds: Bounds::new(
                    bounds.origin,
                    size(bounds.size.width + px(2.0), bounds.size.height),
                ),
            }),
            |window| {
                if let Some(selection) = prepaint.selection.take() {
                    window.paint_quad(selection)
                }
                // A failed paint only loses this frame's text.
                let _ = line.paint(
                    point(bounds.left() - scroll_x, bounds.top()),
                    window.line_height(),
                    gpui::TextAlign::Left,
                    None,
                    window,
                    cx,
                );
                if focus_handle.is_focused(window)
                    && let Some(cursor) = prepaint.cursor.take()
                {
                    window.paint_quad(cursor);
                }
            },
        );
        self.input.update(cx, |input, _cx| {
            input.last_layout = Some(line);
            input.last_bounds = Some(bounds);
            input.scroll_x = scroll_x;
        });
    }
}

impl Render for TextInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.request_grammar(cx);
        div()
            .flex()
            .flex_1()
            .min_w_0()
            .overflow_hidden()
            .key_context(KEY_CONTEXT)
            .track_focus(&self.focus_handle(cx))
            .cursor(CursorStyle::IBeam)
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::left))
            .on_action(cx.listener(Self::right))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::select_right))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::home))
            .on_action(cx.listener(Self::end))
            .on_action(cx.listener(Self::paste))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::copy))
            .on_action(cx.listener(Self::submit))
            .on_action(cx.listener(Self::cancel))
            .when(self.stepper.is_some(), |d| {
                d.on_action(cx.listener(|this, _: &Up, _, cx| this.step(1, cx)))
                    .on_action(cx.listener(|this, _: &Down, _, cx| this.step(-1, cx)))
            })
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_down(MouseButton::Right, cx.listener(Self::on_right_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .child(TextElement { input: cx.entity() })
    }
}

impl Focusable for TextInput {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
