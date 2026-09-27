// SPDX-License-Identifier: GPL-3.0-or-later

//! [`RichEditor`]: edits a [`Doc`] with the keyboard, mouse and IME, and
//! offers the formatting the compose toolbar needs.
//!
//! It takes the plain text area's editing keys (its key context includes
//! [`TEXT_AREA_CONTEXT`]) and adds its own formatting keys in
//! [`RICH_TEXT_CONTEXT`].

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::scale::px;
use crate::scale::unpx;
use gpui::{
    AnyElement, App, Bounds, ClipboardEntry, ClipboardItem, Context, CursorStyle, DispatchPhase,
    ElementId, ElementInputHandler, Entity, EntityInputHandler, EventEmitter, FocusHandle,
    Focusable, GlobalElementId, Hsla, ImageFormat, KeyBinding, LayoutId, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, ObjectFit, Pixels, Point, Position, SharedString,
    Style, UTF16Selection, Window, actions, div, img, point, prelude::*, relative, size,
};

use super::doc::{
    self, Align, Block, CharStyle, Doc, Font, Image, ImageSize, List, MAX_INDENT, Para, ParaStyle,
    Path, Pos, Size, image_size, list_marker, list_numbers, next_grapheme, order, prev_grapheme,
    word_left, word_range, word_right,
};
use super::html;
use super::layout::{Deco, ParaElement, ParaLayout, TextBase};
use crate::TEXT_AREA_CONTEXT;
mod paste;
use crate::text_area::{
    Backspace, Cancel, Copy, Cut, Delete, DeleteWordLeft, DeleteWordRight, DocEnd, DocStart, Down,
    End, Home, Left, Newline, PageDown, PageUp, Paste, Right, SelectAll, SelectDocEnd,
    SelectDocStart, SelectDown, SelectEnd, SelectHome, SelectLeft, SelectPageDown, SelectPageUp,
    SelectRight, SelectUp, SelectWordLeft, SelectWordRight, Submit, Up, WordLeft, WordRight,
};
use paste::PasteOffer;
pub use paste::{PasteLabels, PasteOption, Picture, TablePicture, Transfer};

actions!(
    rich_text,
    [
        Bold,
        Italic,
        Underline,
        Strikethrough,
        NumberedList,
        BulletList,
        Quote,
        IndentMore,
        IndentLess,
        AlignLeft,
        AlignCenter,
        AlignRight,
        ClearFormatting,
        Undo,
        Redo,
        InsertLink,
        NextCell,
        PrevCell,
        AcceptSuggestion,
        DismissSuggestion,
        /// Pastes as plain text.
        PastePlain,
    ]
);

/// Key context of a focused [`RichEditor`], besides [`TEXT_AREA_CONTEXT`].
pub const RICH_TEXT_CONTEXT: &str = "RichText";
/// Added to [`RICH_TEXT_CONTEXT`] while a writing suggestion shows.
const SUGGESTING_CONTEXT: &str = "Suggesting";

/// Binds the formatting keys (webmail's). Call after the text area's.
pub fn bind_keys(cx: &mut App) {
    let context = Some(RICH_TEXT_CONTEXT);
    cx.bind_keys([
        KeyBinding::new("ctrl-b", Bold, context),
        KeyBinding::new("ctrl-i", Italic, context),
        KeyBinding::new("ctrl-u", Underline, context),
        // Shifted digits arrive as the symbol they type on Linux
        // (Ctrl+Shift+8 is "ctrl-*" on a US layout).
        KeyBinding::new("alt-shift-5", Strikethrough, context),
        KeyBinding::new("alt-%", Strikethrough, context),
        KeyBinding::new("ctrl-shift-7", NumberedList, context),
        KeyBinding::new("ctrl-&", NumberedList, context),
        KeyBinding::new("ctrl-shift-8", BulletList, context),
        KeyBinding::new("ctrl-*", BulletList, context),
        KeyBinding::new("ctrl-shift-9", Quote, context),
        KeyBinding::new("ctrl-(", Quote, context),
        KeyBinding::new("ctrl-]", IndentMore, context),
        KeyBinding::new("ctrl-[", IndentLess, context),
        KeyBinding::new("ctrl-shift-l", AlignLeft, context),
        KeyBinding::new("ctrl-shift-e", AlignCenter, context),
        KeyBinding::new("ctrl-shift-r", AlignRight, context),
        KeyBinding::new("ctrl-\\", ClearFormatting, context),
        KeyBinding::new("ctrl-z", Undo, context),
        KeyBinding::new("ctrl-y", Redo, context),
        KeyBinding::new("ctrl-shift-z", Redo, context),
        KeyBinding::new("ctrl-k", InsertLink, context),
        KeyBinding::new("ctrl-shift-v", PastePlain, context),
        KeyBinding::new("tab", NextCell, context),
        KeyBinding::new("shift-tab", PrevCell, context),
        // Last, so they win over Tab and Right while a suggestion shows.
        KeyBinding::new("tab", AcceptSuggestion, Some("RichText && Suggesting")),
        KeyBinding::new("right", AcceptSuggestion, Some("RichText && Suggesting")),
        KeyBinding::new("escape", DismissSuggestion, Some("RichText && Suggesting")),
    ]);
}

/// What a [`RichEditor`] tells its owner.
#[derive(Debug, Clone, PartialEq)]
pub enum RichEvent {
    /// The text changed.
    Changed,
    /// The cursor or selection moved (the toolbar shows its style).
    Selection,
    /// Ctrl+Enter.
    Submit,
    Cancel,
    /// Ctrl+K: the owner opens its link dialog.
    EditLink,
    /// A right click at a window position, on a misspelled word or a
    /// grammar mistake if any.
    ContextMenu {
        position: Point<Pixels>,
        misspelled: Option<String>,
        grammar: Option<GrammarIssue>,
    },
    /// Files were pasted (copied in a file manager): the owner attaches
    /// them.
    PasteFiles(Vec<std::path::PathBuf>),
    /// Pictures were pasted or dropped: the owner puts them in the text or
    /// attaches them.
    PastePictures(Vec<Picture>),
    /// A paste option the owner carries out was chosen (see
    /// [`RichEditor::offer_choice`]).
    PasteChoice(PasteOption),
    /// The pointer rested on, or a left click landed on, a misspelled word
    /// or a grammar mistake: the owner shows the fixes in a card under
    /// `word` (window bounds). [`RichEditor::set_cursor`] with `at` makes
    /// [`RichEditor::replace_word`] and the grammar fixes apply to them.
    Hint {
        word: Bounds<Pixels>,
        at: Pos,
        misspelled: Option<String>,
        suggestions: Vec<String>,
        grammar: Option<GrammarIssue>,
    },
}

/// A spelling dictionary the owner provides.
pub trait SpellCheck {
    /// Whether `word` is spelled right.
    fn check(&self, word: &str) -> bool;
    /// Replacements for a misspelled word, best first.
    fn suggest(&self, word: &str) -> Vec<String>;
}

/// A grammar checker the owner provides. It runs off the UI thread, one
/// paragraph at a time.
pub trait GrammarCheck: Send + Sync {
    /// The mistakes in a paragraph's text.
    fn check(&self, text: &str) -> Vec<GrammarIssue>;
}

/// A grammar mistake in a paragraph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammarIssue {
    /// Where, in bytes of the paragraph's text.
    pub range: Range<usize>,
    /// What is wrong, in a sentence.
    pub message: String,
    /// Ways to fix it, best first.
    pub fixes: Vec<GrammarFix>,
}

/// One way to fix a [`GrammarIssue`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammarFix {
    /// What the menu says, as "Replace with \u{201c}an\u{201d}".
    pub label: String,
    /// The text that takes the place of the mistake.
    pub replacement: String,
}

/// Writing suggestions the owner provides: the likely rest of a phrase.
/// Asked on every keystroke, so it must answer at once.
pub trait Suggest {
    /// What likely follows `before`, the paragraph's text up to the
    /// cursor, with the space it needs first; `None` when unsure.
    fn suggest(&self, before: &str) -> Option<String>;
}

/// How long typing pauses before its paragraph's grammar is checked.
pub(crate) const GRAMMAR_WAIT: Duration = Duration::from_millis(500);
/// How long the pointer rests on marked words before their fixes show.
pub(crate) const HINT_WAIT: Duration = Duration::from_millis(600);

/// Colors of what the editor draws beside text.
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub accent: Hsla,
    pub link: Hsla,
    pub misspelled: Hsla,
    /// The underline of grammar mistakes.
    pub grammar: Hsla,
    /// Table borders and quote bars.
    pub rule: Hsla,
    /// The image size bar.
    pub surface: Hsla,
    pub text: Hsla,
    pub hover: Hsla,
}

impl Default for Palette {
    fn default() -> Self {
        Palette {
            accent: gpui::blue(),
            link: gpui::blue(),
            misspelled: gpui::red(),
            grammar: gpui::blue(),
            rule: gpui::rgb(0xcccccc).into(),
            surface: gpui::white(),
            text: gpui::black(),
            hover: gpui::rgba(0x0000_0014).into(),
        }
    }
}

/// Table edits of the table menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableEdit {
    RowAbove,
    RowBelow,
    ColumnLeft,
    ColumnRight,
    DeleteRow,
    DeleteColumn,
    DeleteTable,
}

/// Words marked as misspelled or as a grammar mistake, under a point.
struct Mark {
    path: Path,
    range: Range<usize>,
    /// In window coordinates.
    bounds: Bounds<Pixels>,
    misspelled: bool,
    grammar: Option<GrammarIssue>,
}

#[derive(Clone)]
struct Snapshot {
    doc: Doc,
    anchor: Pos,
    head: Pos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EditKind {
    Typing,
    Deleting,
    Other,
}

const MAX_UNDO: usize = 200;
/// Typing within this time of the last keystroke undoes together.
const TYPING_GROUP: Duration = Duration::from_millis(1200);

/// A rich text editor. Create it with [`RichEditor::new`] inside `cx.new`
/// and render the entity as a child; it is as tall as its content.
pub struct RichEditor {
    pub(crate) focus_handle: FocusHandle,
    pub(crate) doc: Doc,
    anchor: Pos,
    head: Pos,
    upstream: bool,
    /// The x the cursor keeps while moving up and down (window x).
    goal_x: Option<Pixels>,
    /// The style the next typed text takes (after toggling bold with
    /// nothing selected, say).
    typing: Option<CharStyle>,
    /// The IME's composition, in the head paragraph.
    marked: Option<Range<usize>>,
    selecting: bool,
    selected_image: Option<usize>,
    pub(crate) layouts: HashMap<Path, ParaLayout>,
    numbers: Vec<usize>,
    undo: Vec<Snapshot>,
    redo: Vec<Snapshot>,
    last_edit: Option<(EditKind, Instant)>,
    placeholder: SharedString,
    palette: Palette,
    plain: bool,
    spell: Option<Rc<dyn SpellCheck>>,
    misspellings: RefCell<HashMap<String, Vec<Range<usize>>>>,
    grammar: Option<Arc<dyn GrammarCheck>>,
    /// Grammar mistakes by paragraph text, as far as checked.
    grammar_found: HashMap<String, Vec<GrammarIssue>>,
    /// The paragraphs being checked.
    grammar_asked: Vec<String>,
    grammar_task: Option<gpui::Task<()>>,
    /// Mistakes the user chose to ignore in this text: the words and the
    /// message.
    grammar_ignored: std::collections::HashSet<(String, String)>,
    suggest: Option<Rc<dyn Suggest>>,
    /// The writing suggestion shown after the cursor, while the cursor
    /// stays where it was made.
    ghost: Option<(Pos, String)>,
    families: RefCell<Option<Rc<HashMap<Font, SharedString>>>>,
    /// What was copied, to paste it back with its formatting.
    copied: Option<(String, Vec<Block>)>,
    paste_offer: Option<PasteOffer>,
    /// Where a picture dragged over the text would land.
    pub(crate) drop_caret: Option<(Pos, bool)>,
    paste_labels: Option<PasteLabels>,
    table_picture: Option<TablePicture>,
    images: HashMap<u64, Arc<gpui::Image>>,
    next_image_id: u64,
    /// The editor's width at the last paint, for sizing images.
    width: Pixels,
    /// The editor had the focus when last drawn.
    pub(crate) drawn_focused: std::cell::Cell<bool>,
    /// The marked words under the pointer, and the wait before their fixes
    /// show.
    hover: Option<(Path, Range<usize>)>,
    hover_task: Option<gpui::Task<()>>,
    /// The marked words a left click went down on, to show their fixes if
    /// it comes up without dragging.
    clicked: Option<(Path, Range<usize>)>,
}

impl EventEmitter<RichEvent> for RichEditor {}

impl RichEditor {
    pub fn new(placeholder: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        let doc = Doc::default();
        let start = doc.start();
        Self {
            // Tab outside a table or list moves on to the next field.
            focus_handle: cx.focus_handle().tab_stop(true),
            doc,
            anchor: start,
            head: start,
            upstream: false,
            goal_x: None,
            typing: None,
            marked: None,
            selecting: false,
            selected_image: None,
            layouts: HashMap::new(),
            numbers: Vec::new(),
            undo: Vec::new(),
            redo: Vec::new(),
            last_edit: None,
            placeholder: placeholder.into(),
            palette: Palette::default(),
            plain: false,
            spell: None,
            misspellings: RefCell::new(HashMap::new()),
            grammar: None,
            grammar_found: HashMap::new(),
            grammar_asked: Vec::new(),
            grammar_task: None,
            grammar_ignored: Default::default(),
            suggest: None,
            ghost: None,
            families: RefCell::new(None),
            copied: None,
            paste_offer: None,
            drop_caret: None,
            paste_labels: None,
            table_picture: None,
            images: HashMap::new(),
            drawn_focused: std::cell::Cell::new(false),
            next_image_id: 0,
            width: px(0.0),
            hover: None,
            hover_task: None,
            clicked: None,
        }
    }

    // Content.

    pub fn doc(&self) -> &Doc {
        &self.doc
    }

    /// Replaces the content; the cursor goes to `cursor` and the undo
    /// history starts over.
    pub fn set_doc(&mut self, doc: Doc, cursor: Pos, cx: &mut Context<Self>) {
        self.doc = doc;
        if self.doc.blocks.is_empty() {
            self.doc = Doc::default();
        }
        self.next_image_id = self
            .doc
            .images()
            .map(|i| i.id)
            .max()
            .unwrap_or(0)
            .max(self.next_image_id);
        let cursor = self.doc.clamp(cursor);
        self.anchor = cursor;
        self.head = cursor;
        self.undo.clear();
        self.redo.clear();
        self.last_edit = None;
        self.selected_image = None;
        self.typing = None;
        self.marked = None;
        cx.emit(RichEvent::Changed);
        cx.emit(RichEvent::Selection);
        cx.notify();
    }

    /// Adds `blocks` at the end of the content, as if they had always been
    /// there: the selection stays and undo does not take them away.
    pub fn append_blocks(&mut self, blocks: Vec<Block>, cx: &mut Context<Self>) {
        for snapshot in self.undo.iter_mut().chain(self.redo.iter_mut()) {
            snapshot.doc.blocks.extend(blocks.iter().cloned());
        }
        self.doc.blocks.extend(blocks);
        cx.emit(RichEvent::Changed);
        cx.notify();
    }

    /// The content as mail HTML.
    pub fn html(&self, image_src: &dyn Fn(&Image) -> String) -> String {
        html::to_html(&self.doc, image_src)
    }

    /// The content as plain text.
    pub fn plain_text(&self) -> String {
        html::to_plain(&self.doc)
    }

    pub fn is_blank(&self) -> bool {
        self.doc.is_blank()
    }

    pub fn set_placeholder(&mut self, placeholder: impl Into<SharedString>) {
        self.placeholder = placeholder.into();
    }

    pub fn set_palette(&mut self, palette: Palette) {
        self.palette = palette;
    }

    /// Plain text mode: formatting is off and the text draws plain.
    pub fn set_plain(&mut self, plain: bool, cx: &mut Context<Self>) {
        self.plain = plain;
        cx.notify();
    }

    pub fn is_plain(&self) -> bool {
        self.plain
    }

    /// Turns spell checking on with `dictionary`, or off.
    pub fn set_spell_check(
        &mut self,
        dictionary: Option<Rc<dyn SpellCheck>>,
        cx: &mut Context<Self>,
    ) {
        self.spell = dictionary;
        self.misspellings.borrow_mut().clear();
        cx.notify();
    }

    pub fn spell_check(&self) -> bool {
        self.spell.is_some()
    }

    /// Turns grammar checking on with `checker`, or off.
    pub fn set_grammar_check(
        &mut self,
        checker: Option<Arc<dyn GrammarCheck>>,
        cx: &mut Context<Self>,
    ) {
        self.grammar = checker;
        self.grammar_found.clear();
        self.grammar_asked.clear();
        self.grammar_task = None;
        cx.notify();
    }

    /// Turns writing suggestions on with `suggest`, or off.
    pub fn set_suggest(&mut self, suggest: Option<Rc<dyn Suggest>>, cx: &mut Context<Self>) {
        self.suggest = suggest;
        self.ghost = None;
        cx.notify();
    }

    /// Asks for a writing suggestion after what was just typed: only at
    /// the end of a paragraph the user writes (not a quote or the
    /// signature), with nothing selected or being composed.
    fn request_suggestion(&mut self) {
        self.ghost = None;
        let Some(suggest) = &self.suggest else {
            return;
        };
        if self.has_selection() || self.marked.is_some() || self.selected_image.is_some() {
            return;
        }
        let Some(para) = self.doc.para(self.head.path) else {
            return;
        };
        if self.plain_blocked(para) || self.head.offset != para.len() {
            return;
        }
        if let Some(text) = suggest.suggest(&para.text).filter(|t| !t.trim().is_empty()) {
            self.ghost = Some((self.head, text));
        }
    }

    /// The writing suggestion shown in paragraph `path`, and its style.
    pub(crate) fn ghost_in(&self, path: Path) -> Option<(usize, &str, CharStyle)> {
        let (pos, text) = self.ghost.as_ref()?;
        (pos.path == path && *pos == self.head && !self.has_selection())
            .then(|| (pos.offset, text.as_str(), self.style_for_typing()))
    }

    fn showing_suggestion(&self) -> bool {
        self.ghost
            .as_ref()
            .is_some_and(|(pos, _)| *pos == self.head && !self.has_selection())
    }

    fn accept_suggestion(&mut self, _: &AcceptSuggestion, _: &mut Window, cx: &mut Context<Self>) {
        match self.ghost.take() {
            Some((pos, text)) if pos == self.head && !self.has_selection() => {
                self.insert(&text, cx);
            }
            _ => cx.propagate(),
        }
    }

    fn dismiss_suggestion(
        &mut self,
        _: &DismissSuggestion,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.ghost.take().is_none() {
            cx.propagate();
        }
        cx.notify();
    }

    /// Checks the grammar of paragraphs not checked yet, once typing
    /// pauses. Called on every render; a change starts the wait again.
    fn request_grammar(&mut self, cx: &mut Context<Self>) {
        let Some(checker) = self.grammar.clone() else {
            return;
        };
        let mut wanted: Vec<String> = Vec::new();
        for path in self.doc.paths() {
            let Some(para) = self.doc.para(path) else {
                continue;
            };
            if !self.plain_blocked(para)
                && para.text.chars().any(char::is_alphabetic)
                && !self.grammar_found.contains_key(&para.text)
                && !wanted.contains(&para.text)
            {
                wanted.push(para.text.clone());
            }
        }
        if wanted.is_empty() || wanted == self.grammar_asked {
            return;
        }
        self.grammar_asked = wanted.clone();
        self.grammar_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(GRAMMAR_WAIT).await;
            let found = cx
                .background_executor()
                .spawn(async move {
                    wanted
                        .into_iter()
                        .map(|text| {
                            let issues = checker.check(&text);
                            (text, issues)
                        })
                        .collect::<Vec<_>>()
                })
                .await;
            this.update(cx, |this, cx| {
                this.grammar_found.extend(found);
                // Forget paragraphs that are gone, as typed on the way.
                let texts: std::collections::HashSet<&str> = this
                    .doc
                    .paths()
                    .into_iter()
                    .filter_map(|p| this.doc.para(p))
                    .map(|p| p.text.as_str())
                    .collect();
                this.grammar_found.retain(|t, _| texts.contains(t.as_str()));
                this.grammar_asked.clear();
                this.grammar_task = None;
                cx.notify();
            })
            .ok();
        }));
    }

    /// Mistakes found in `para`'s text, without the ignored ones.
    fn grammar_issues(&self, para: &Para) -> Vec<&GrammarIssue> {
        if self.grammar.is_none() || self.plain_blocked(para) {
            return Vec::new();
        }
        self.grammar_found
            .get(&para.text)
            .into_iter()
            .flatten()
            .filter(|issue| {
                para.text.get(issue.range.clone()).is_some_and(|words| {
                    !self
                        .grammar_ignored
                        .contains(&(words.to_owned(), issue.message.clone()))
                })
            })
            .collect()
    }

    /// The grammar mistake at the cursor, if any.
    fn grammar_at_cursor(&self) -> Option<GrammarIssue> {
        let para = self.doc.para(self.head.path)?;
        let at = self.head.offset;
        self.grammar_issues(para)
            .into_iter()
            .find(|issue| issue.range.start <= at && at <= issue.range.end)
            .cloned()
    }

    /// Fixes grammar mistake `issue` in the cursor's paragraph with `fix`.
    pub fn fix_grammar(&mut self, issue: &GrammarIssue, fix: &GrammarFix, cx: &mut Context<Self>) {
        let path = self.head.path;
        let range = issue.range.clone();
        let current = self.doc.para(path).and_then(|p| p.text.get(range.clone()));
        if current.is_none() {
            return;
        }
        let replacement = fix.replacement.clone();
        self.edit(EditKind::Other, cx, |doc, _| {
            let style = doc
                .para(path)
                .map(|p| p.style_at(range.start))
                .unwrap_or_default();
            let start = Pos::new(path, range.start);
            doc.delete(start, Pos::new(path, range.end));
            doc.insert_text(start, &replacement, &style)
        });
    }

    /// Stops marking `issue`'s words with its message in this text.
    pub fn ignore_grammar(&mut self, issue: &GrammarIssue, cx: &mut Context<Self>) {
        let words = self
            .doc
            .para(self.head.path)
            .and_then(|p| p.text.get(issue.range.clone()))
            .map(str::to_owned);
        if let Some(words) = words {
            self.grammar_ignored.insert((words, issue.message.clone()));
            cx.notify();
        }
    }

    /// Suggestions for the misspelled word at the cursor.
    pub fn suggestions(&self) -> Vec<String> {
        let (Some(spell), Some(word)) = (&self.spell, self.word_at_cursor()) else {
            return Vec::new();
        };
        spell.suggest(&word.1)
    }

    fn word_at_cursor(&self) -> Option<(Range<usize>, String)> {
        let para = self.doc.para(self.head.path)?;
        let range = word_range(&para.text, self.head.offset);
        let word = para.text[range.clone()].to_owned();
        word.chars()
            .any(char::is_alphabetic)
            .then_some((range, word))
    }

    /// Puts `replacement` in place of the word at the cursor.
    pub fn replace_word(&mut self, replacement: &str, cx: &mut Context<Self>) {
        let Some((range, _)) = self.word_at_cursor() else {
            return;
        };
        let path = self.head.path;
        self.edit(EditKind::Other, cx, |doc, _| {
            let style = doc
                .para(path)
                .map(|p| p.style_at(range.start))
                .unwrap_or_default();
            let start = Pos::new(path, range.start);
            doc.delete(start, Pos::new(path, range.end));
            doc.insert_text(start, replacement, &style)
        });
    }

    /// Puts a template in: its formatted body `html` (or its `text` in plain
    /// text mode), with each `(field, value)` filled in. It goes at the
    /// top when nothing is written above the signature, else in place of
    /// the selection.
    pub fn insert_template(
        &mut self,
        html: &str,
        text: &str,
        fields: &[(&str, String)],
        cx: &mut Context<Self>,
    ) {
        let mut doc = if self.plain || html.trim().is_empty() {
            html::from_plain(text.trim_end())
        } else {
            html::from_html(html, &mut self.next_image_id)
        };
        for (field, value) in fields {
            doc.replace_text(field, value);
        }
        let at_top = self.doc.is_blank_above_signature();
        self.edit(EditKind::Other, cx, |doc_now, (start, end)| {
            let at = if at_top {
                // The empty lines above the signature give way to it.
                let blank = doc_now
                    .blocks
                    .iter()
                    .take_while(|b| !matches!(b, Block::Para(p) if p.style.signature))
                    .count();
                let signed = blank < doc_now.blocks.len();
                doc_now.blocks.drain(..blank);
                doc_now.blocks.insert(0, Block::Para(Para::default()));
                // An empty line stays between the text and the signature.
                if signed {
                    doc_now.blocks.insert(1, Block::Para(Para::default()));
                }
                Pos::new(Path::top(0), 0)
            } else {
                doc_now.delete(start, end)
            };
            doc_now.insert_fragment(at, doc.blocks)
        });
    }

    /// Fills each `(field, value)` left in the text. Returns whether there
    /// was one.
    pub fn fill_fields(&mut self, fields: &[(&str, String)], cx: &mut Context<Self>) -> bool {
        let present = fields.iter().any(|(field, _)| {
            self.doc
                .paths()
                .iter()
                .filter_map(|p| self.doc.para(*p))
                .any(|p| p.text.contains(field))
        });
        if present {
            self.edit(EditKind::Other, cx, |doc, (_, end)| {
                for (field, value) in fields {
                    doc.replace_text(field, value);
                }
                end
            });
        }
        present
    }

    /// Puts the cursor at `at`, such as the words of a [`RichEvent::Hint`].
    pub fn set_cursor(&mut self, at: Pos, cx: &mut Context<Self>) {
        self.set_selection(at, at, cx);
    }

    // Selection.

    fn ordered(&self) -> (Pos, Pos) {
        order(self.anchor, self.head)
    }

    pub fn has_selection(&self) -> bool {
        self.anchor != self.head
    }

    /// The selected text, plain.
    pub fn selected_text(&self) -> String {
        let (start, end) = self.ordered();
        let fragment = Doc {
            blocks: self.doc.fragment(start, end),
        };
        html::to_plain(&fragment).trim_end_matches('\n').to_owned()
    }

    /// The selected range in paragraph `path`, and whether the selection
    /// goes on past its end.
    pub(crate) fn selection_in(&self, path: Path) -> Option<(Range<usize>, bool)> {
        if !self.has_selection() {
            return None;
        }
        let (start, end) = self.ordered();
        self.doc
            .covered(start, end)
            .into_iter()
            .find(|(p, _)| *p == path)
            .map(|(_, range)| (range, path != end.path))
    }

    pub(crate) fn cursor_in(&self, path: Path) -> Option<(usize, bool)> {
        (self.head.path == path && !self.has_selection() && self.selected_image.is_none())
            .then_some((self.head.offset, self.upstream))
    }

    pub(crate) fn marker(&self, path: Path) -> Option<String> {
        if path.cell.is_some() {
            return None;
        }
        let para = self.doc.para(path)?;
        (para.style.list != List::None).then(|| {
            list_marker(
                &para.style,
                self.numbers.get(path.block).copied().unwrap_or(1),
            )
        })
    }

    pub(crate) fn decorations(&self, path: Path, para: &Para) -> Vec<(Range<usize>, Deco)> {
        let mut decos = Vec::new();
        if self.head.path == path
            && let Some(marked) = &self.marked
            && marked.end <= para.len()
        {
            decos.push((
                marked.clone(),
                Deco {
                    marked: true,
                    ..Deco::default()
                },
            ));
        }
        if let Some(spell) = &self.spell
            && !self.plain_blocked(para)
        {
            let mut cache = self.misspellings.borrow_mut();
            let found = cache
                .entry(para.text.clone())
                .or_insert_with(|| misspelled(&para.text, spell.as_ref()))
                .clone();
            // The word being typed is not marked until it is done.
            let typing = (self.head.path == path).then(|| word_range(&para.text, self.head.offset));
            for range in found {
                let open = typing
                    .as_ref()
                    .is_some_and(|t| *t == range && self.head.offset == range.end);
                let overlaps = decos.iter().any(|(r, _): &(Range<usize>, Deco)| {
                    r.start < range.end && range.start < r.end
                });
                if !open && !overlaps {
                    decos.push((
                        range,
                        Deco {
                            misspelled: true,
                            ..Deco::default()
                        },
                    ));
                }
            }
        }
        for issue in self.grammar_issues(para) {
            let range = issue.range.clone();
            let overlaps = decos
                .iter()
                .any(|(r, _): &(Range<usize>, Deco)| r.start < range.end && range.start < r.end);
            if !overlaps && range.start < range.end {
                decos.push((
                    range,
                    Deco {
                        grammar: true,
                        ..Deco::default()
                    },
                ));
            }
        }
        decos.sort_by_key(|(r, _)| r.start);
        decos
    }

    fn plain_blocked(&self, para: &Para) -> bool {
        para.style.signature || para.style.quote > 0
    }

    pub(crate) fn text_base(&self, window: &Window) -> TextBase {
        let style = window.text_style();
        let font = style.font();
        let families = self
            .families
            .borrow_mut()
            .get_or_insert_with(|| Rc::new(resolve_families(window)))
            .clone();
        TextBase {
            font,
            size: style.font_size.to_pixels(window.rem_size()),
            line_height: window.line_height(),
            color: style.color,
            link: self.palette.link,
            accent: self.palette.accent,
            misspelled: self.palette.misspelled,
            grammar: self.palette.grammar,
            families,
            plain: self.plain,
        }
    }

    /// The cursor's bounds in window coordinates as of the last paint.
    /// Whether the editor had the focus when it was last drawn, for its
    /// owner's popups (which draw without the window at hand).
    pub fn had_focus(&self) -> bool {
        self.drawn_focused.get()
    }

    pub fn cursor_bounds(&self) -> Option<Bounds<Pixels>> {
        let layout = self.layouts.get(&self.head.path)?;
        let (at, height) = layout.caret(self.head.offset, self.upstream);
        Some(Bounds::new(
            layout.bounds.origin + at,
            size(px(2.0), height),
        ))
    }

    fn set_selection(&mut self, anchor: Pos, head: Pos, cx: &mut Context<Self>) {
        let anchor = self.doc.clamp(anchor);
        let head = self.doc.clamp(head);
        let changed = anchor != self.anchor || head != self.head;
        self.anchor = anchor;
        self.head = head;
        self.selected_image = None;
        self.marked = None;
        if changed {
            self.typing = None;
            self.ghost = None;
            cx.emit(RichEvent::Selection);
        }
        cx.notify();
    }

    fn move_to(&mut self, pos: Pos, cx: &mut Context<Self>) {
        self.upstream = false;
        self.goal_x = None;
        self.set_selection(pos, pos, cx);
    }

    fn select_to(&mut self, pos: Pos, cx: &mut Context<Self>) {
        self.upstream = false;
        self.goal_x = None;
        self.set_selection(self.anchor, pos, cx);
    }

    // Editing.

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            doc: self.doc.clone(),
            anchor: self.anchor,
            head: self.head,
        }
    }

    /// Runs an edit on the document with the ordered selection; it returns
    /// where the cursor goes.
    fn edit(
        &mut self,
        kind: EditKind,
        cx: &mut Context<Self>,
        f: impl FnOnce(&mut Doc, (Pos, Pos)) -> Pos,
    ) {
        let now = Instant::now();
        let grouped = matches!(
            self.last_edit,
            Some((last, at)) if last == kind && kind != EditKind::Other && now - at < TYPING_GROUP
        );
        if !grouped {
            self.undo.push(self.snapshot());
            if self.undo.len() > MAX_UNDO {
                self.undo.remove(0);
            }
        }
        self.redo.clear();
        self.ghost = None;
        self.paste_offer = None;
        self.last_edit = Some((kind, now));
        let selection = self.ordered();
        let cursor = f(&mut self.doc, selection);
        if self.doc.blocks.is_empty() {
            self.doc = Doc::default();
        }
        let cursor = self.doc.clamp(cursor);
        self.anchor = cursor;
        self.head = cursor;
        self.upstream = false;
        self.goal_x = None;
        self.selected_image = None;
        self.marked = None;
        cx.emit(RichEvent::Changed);
        cx.emit(RichEvent::Selection);
        cx.notify();
    }

    /// Like [`Self::edit`] but keeps the selection (formatting).
    fn format(&mut self, cx: &mut Context<Self>, f: impl FnOnce(&mut Doc, (Pos, Pos))) {
        let (anchor, head) = (self.anchor, self.head);
        self.edit(EditKind::Other, cx, |doc, sel| {
            f(doc, sel);
            head
        });
        self.anchor = self.doc.clamp(anchor);
        self.head = self.doc.clamp(head);
    }

    /// Types `text` over the selection, in the typing style.
    pub fn insert(&mut self, text: &str, cx: &mut Context<Self>) {
        let style = self.style_for_typing();
        let kind = if text.chars().any(char::is_whitespace) {
            EditKind::Other
        } else {
            EditKind::Typing
        };
        let text = text.replace("\r\n", "\n").replace('\r', "\n");
        self.edit(kind, cx, |doc, (start, end)| {
            let at = doc.delete(start, end);
            doc.insert_text(at, &text, &style)
        });
    }

    /// The style text typed now takes.
    fn style_for_typing(&self) -> CharStyle {
        if self.plain {
            return CharStyle::default();
        }
        if let Some(style) = &self.typing {
            return style.clone();
        }
        let (start, _) = self.ordered();
        let Some(para) = self.doc.para(start.path) else {
            return CharStyle::default();
        };
        let mut style = para.style_at(start.offset);
        // Typing right after a link does not extend it.
        if style.link.is_some()
            && para
                .link_at(start.offset)
                .is_none_or(|(range, _)| range.end == start.offset)
        {
            style.link = None;
        }
        style
    }

    fn delete_selection_or(&mut self, cx: &mut Context<Self>, target: impl FnOnce(&Self) -> Pos) {
        if let Some(block) = self.selected_image.take() {
            self.edit(EditKind::Other, cx, |doc, _| doc.remove_block(block));
            return;
        }
        if !self.has_selection() {
            let to = target(self);
            if to == self.head {
                return;
            }
            self.anchor = to;
        }
        self.edit(EditKind::Deleting, cx, |doc, (start, end)| {
            doc.delete(start, end)
        });
    }

    fn backspace(&mut self, _: &Backspace, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_image.is_none() && !self.has_selection() && self.head.offset == 0 {
            let path = self.head.path;
            self.edit(EditKind::Other, cx, |doc, _| doc.join_backward(path));
            return;
        }
        self.delete_selection_or(cx, |this| {
            let para = this.doc.para(this.head.path);
            let text = para.map_or("", |p| p.text.as_str());
            Pos::new(this.head.path, prev_grapheme(text, this.head.offset))
        });
    }

    fn delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        let len = self.doc.para(self.head.path).map_or(0, Para::len);
        if self.selected_image.is_none() && !self.has_selection() && self.head.offset == len {
            let path = self.head.path;
            self.edit(EditKind::Other, cx, |doc, _| doc.join_forward(path));
            return;
        }
        self.delete_selection_or(cx, |this| {
            let text = this
                .doc
                .para(this.head.path)
                .map_or("", |p| p.text.as_str());
            Pos::new(this.head.path, next_grapheme(text, this.head.offset))
        });
    }

    fn delete_word_left(&mut self, _: &DeleteWordLeft, w: &mut Window, cx: &mut Context<Self>) {
        if !self.has_selection() && self.head.offset == 0 {
            return self.backspace(&Backspace, w, cx);
        }
        self.delete_selection_or(cx, |this| {
            let text = this
                .doc
                .para(this.head.path)
                .map_or("", |p| p.text.as_str());
            Pos::new(this.head.path, word_left(text, this.head.offset))
        });
    }

    fn delete_word_right(&mut self, _: &DeleteWordRight, w: &mut Window, cx: &mut Context<Self>) {
        let len = self.doc.para(self.head.path).map_or(0, Para::len);
        if !self.has_selection() && self.head.offset == len {
            return self.delete(&Delete, w, cx);
        }
        self.delete_selection_or(cx, |this| {
            let text = this
                .doc
                .para(this.head.path)
                .map_or("", |p| p.text.as_str());
            Pos::new(this.head.path, word_right(text, this.head.offset))
        });
    }

    fn newline(&mut self, _: &Newline, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(block) = self.selected_image.take() {
            // Enter on a picture starts a line after it.
            let pos = Pos::new(Path::top(block + 1), 0);
            self.move_to(pos, cx);
            return;
        }
        let typing = self.style_for_typing();
        self.edit(EditKind::Other, cx, |doc, (start, end)| {
            let at = doc.delete(start, end);
            if at.path.cell.is_none()
                && let Some(para) = doc.para_mut(at.path)
                && para.is_empty()
            {
                // Enter on an empty item ends the list (or the quote).
                if para.style.list != List::None {
                    para.style.list = List::None;
                    para.style.indent = 0;
                    return at;
                }
                if para.style.quote > 0 && !para.style.signature {
                    para.style.quote -= 1;
                    return at;
                }
            }
            doc.split(at)
        });
        if !typing.is_plain() {
            self.typing = Some(typing);
        }
    }

    fn submit(&mut self, _: &Submit, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(RichEvent::Submit);
    }

    fn cancel(&mut self, _: &Cancel, _: &mut Window, cx: &mut Context<Self>) {
        if self.paste_offer.take().is_some() {
            cx.notify();
            return;
        }
        if self.selected_image.take().is_some() {
            cx.notify();
            return;
        }
        cx.emit(RichEvent::Cancel);
    }

    // Clipboard.

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        let (start, end) = self.ordered();
        let fragment = match self.selected_image {
            Some(block) => self.doc.blocks.get(block).cloned().into_iter().collect(),
            None if self.has_selection() => self.doc.fragment(start, end),
            None => return,
        };
        let text = html::to_plain(&Doc {
            blocks: fragment.clone(),
        })
        .trim_end_matches('\n')
        .to_owned();
        // Other apps get the HTML form too, with pictures inside it.
        let html = html::to_html(
            &Doc {
                blocks: fragment.clone(),
            },
            &html::data_uri,
        );
        cx.write_to_clipboard(crate::native::html_item(text.clone(), html));
        self.copied = Some((text, fragment));
    }

    fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        self.copy(&Copy, window, cx);
        if self.has_selection() || self.selected_image.is_some() {
            self.delete_selection_or(cx, |this| this.head);
        }
    }

    // Movement.

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.has_selection() {
            let (start, _) = self.ordered();
            self.move_to(start, cx);
        } else {
            self.move_to(self.doc.prev_pos(self.head), cx);
        }
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.has_selection() {
            let (_, end) = self.ordered();
            self.move_to(end, cx);
        } else {
            self.move_to(self.doc.next_pos(self.head), cx);
        }
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.doc.prev_pos(self.head), cx);
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.doc.next_pos(self.head), cx);
    }

    fn word_target(&self, right: bool) -> Pos {
        let head = self.head;
        let text = self.doc.para(head.path).map_or("", |p| p.text.as_str());
        let target = if right {
            word_right(text, head.offset)
        } else {
            word_left(text, head.offset)
        };
        if target == head.offset {
            // At a paragraph's edge: into the next one.
            if right {
                self.doc.next_pos(head)
            } else {
                self.doc.prev_pos(head)
            }
        } else {
            Pos::new(head.path, target)
        }
    }

    fn word_left(&mut self, _: &WordLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.word_target(false), cx);
    }

    fn word_right(&mut self, _: &WordRight, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.word_target(true), cx);
    }

    fn select_word_left(&mut self, _: &SelectWordLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.word_target(false), cx);
    }

    fn select_word_right(&mut self, _: &SelectWordRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.word_target(true), cx);
    }

    /// Start (or end) of the visual line the cursor is on.
    fn line_edge(&self, end: bool) -> (Pos, bool) {
        let head = self.head;
        let Some(layout) = self.layouts.get(&head.path) else {
            let len = self.doc.para(head.path).map_or(0, Para::len);
            return (Pos::new(head.path, if end { len } else { 0 }), false);
        };
        let ix = layout.line_for(head.offset, self.upstream);
        let Some(line) = layout.lines.get(ix) else {
            return (head, false);
        };
        let len = self.doc.para(head.path).map_or(0, Para::len);
        if end {
            (Pos::new(head.path, line.range.end.min(len)), line.soft_end)
        } else {
            (Pos::new(head.path, line.range.start.min(len)), false)
        }
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        let (pos, _) = self.line_edge(false);
        self.move_to(pos, cx);
    }

    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        let (pos, upstream) = self.line_edge(true);
        self.move_to(pos, cx);
        self.upstream = upstream;
    }

    fn select_home(&mut self, _: &SelectHome, _: &mut Window, cx: &mut Context<Self>) {
        let (pos, _) = self.line_edge(false);
        self.select_to(pos, cx);
    }

    fn select_end(&mut self, _: &SelectEnd, _: &mut Window, cx: &mut Context<Self>) {
        let (pos, upstream) = self.line_edge(true);
        self.select_to(pos, cx);
        self.upstream = upstream;
    }

    fn doc_start(&mut self, _: &DocStart, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.doc.start(), cx);
    }

    fn doc_end(&mut self, _: &DocEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.doc.end(), cx);
    }

    fn select_doc_start(&mut self, _: &SelectDocStart, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.doc.start(), cx);
    }

    fn select_doc_end(&mut self, _: &SelectDocEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.doc.end(), cx);
    }

    pub fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.set_selection(self.doc.start(), self.doc.end(), cx);
    }

    /// Moves `dy` pixels up or down from the cursor, keeping its x.
    fn vertical(&mut self, dy: Pixels, select: bool, cx: &mut Context<Self>) {
        let Some(caret) = self.cursor_bounds() else {
            let to = if dy < px(0.0) {
                self.doc.start()
            } else {
                self.doc.end()
            };
            if select {
                self.select_to(to, cx)
            } else {
                self.move_to(to, cx)
            }
            return;
        };
        let x = self.goal_x.unwrap_or(caret.left());
        let y = if dy < px(0.0) {
            caret.top() + dy
        } else {
            caret.bottom() + dy
        };
        let (pos, upstream) = match self.hit_toward(point(x, y), dy < px(0.0)) {
            Some(hit) => hit,
            None if dy < px(0.0) => (self.doc.start(), false),
            None => (self.doc.end(), false),
        };
        if select {
            self.set_selection(self.anchor, pos, cx);
        } else {
            self.set_selection(pos, pos, cx);
        }
        self.upstream = upstream;
        self.goal_x = Some(x);
    }

    fn up(&mut self, _: &Up, _: &mut Window, cx: &mut Context<Self>) {
        if self.has_selection() {
            let (start, _) = self.ordered();
            self.move_to(start, cx);
            return;
        }
        self.vertical(px(-2.0), false, cx);
    }

    fn down(&mut self, _: &Down, _: &mut Window, cx: &mut Context<Self>) {
        if self.has_selection() {
            let (_, end) = self.ordered();
            self.move_to(end, cx);
            return;
        }
        self.vertical(px(2.0), false, cx);
    }

    fn select_up(&mut self, _: &SelectUp, _: &mut Window, cx: &mut Context<Self>) {
        self.vertical(px(-2.0), true, cx);
    }

    fn select_down(&mut self, _: &SelectDown, _: &mut Window, cx: &mut Context<Self>) {
        self.vertical(px(2.0), true, cx);
    }

    fn page(&self, window: &Window) -> Pixels {
        (window.viewport_size().height * 0.8).max(px(40.0))
    }

    fn page_up(&mut self, _: &PageUp, window: &mut Window, cx: &mut Context<Self>) {
        let page = self.page(window);
        self.vertical(-page, false, cx);
    }

    fn page_down(&mut self, _: &PageDown, window: &mut Window, cx: &mut Context<Self>) {
        let page = self.page(window);
        self.vertical(page, false, cx);
    }

    fn select_page_up(&mut self, _: &SelectPageUp, window: &mut Window, cx: &mut Context<Self>) {
        let page = self.page(window);
        self.vertical(-page, true, cx);
    }

    fn select_page_down(
        &mut self,
        _: &SelectPageDown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let page = self.page(window);
        self.vertical(page, true, cx);
    }

    // Hit testing over the paragraphs drawn last.

    /// The position under window point `p`.
    fn hit(&self, p: Point<Pixels>) -> Option<(Pos, bool)> {
        let best = self
            .layouts
            .iter()
            .filter(|(path, _)| self.doc.para(**path).is_some())
            .min_by(|(_, a), (_, b)| distance(a.bounds, p).total_cmp(&distance(b.bounds, p)))?;
        let (path, layout) = best;
        let text = self.doc.para(*path).map_or("", |p| p.text.as_str());
        let (offset, upstream) = layout.hit(text, p);
        Some((Pos::new(*path, offset), upstream))
    }

    /// The position under `p`, or in the nearest paragraph above it (`up`)
    /// or below it.
    fn hit_toward(&self, p: Point<Pixels>, up: bool) -> Option<(Pos, bool)> {
        let candidates = self
            .layouts
            .iter()
            .filter(|(path, _)| self.doc.para(**path).is_some());
        let covering: Vec<_> = candidates
            .clone()
            .filter(|(_, l)| l.bounds.top() <= p.y && p.y < l.bounds.bottom())
            .collect();
        let chosen = if covering.is_empty() {
            if up {
                candidates
                    .filter(|(_, l)| l.bounds.bottom() <= p.y)
                    .max_by(|(_, a), (_, b)| {
                        a.bounds
                            .bottom()
                            .partial_cmp(&b.bounds.bottom())
                            .unwrap_or(std::cmp::Ordering::Equal)
                            .then_with(|| {
                                horizontal(b.bounds, p.x).total_cmp(&horizontal(a.bounds, p.x))
                            })
                    })
            } else {
                candidates
                    .filter(|(_, l)| l.bounds.top() > p.y)
                    .min_by(|(_, a), (_, b)| {
                        a.bounds
                            .top()
                            .partial_cmp(&b.bounds.top())
                            .unwrap_or(std::cmp::Ordering::Equal)
                            .then_with(|| {
                                horizontal(a.bounds, p.x).total_cmp(&horizontal(b.bounds, p.x))
                            })
                    })
            }
        } else {
            covering.into_iter().min_by(|(_, a), (_, b)| {
                horizontal(a.bounds, p.x).total_cmp(&horizontal(b.bounds, p.x))
            })
        }?;
        let (path, layout) = chosen;
        let text = self.doc.para(*path).map_or("", |p| p.text.as_str());
        let y =
            p.y.clamp(layout.bounds.top(), layout.bounds.bottom() - px(1.0));
        let (offset, upstream) = layout.hit(text, point(p.x, y));
        Some((Pos::new(*path, offset), upstream))
    }

    // Mouse.

    fn mouse_down(&mut self, event: &MouseDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.focus_handle, cx);
        let Some((pos, upstream)) = self.hit(event.position) else {
            return;
        };
        match event.click_count {
            2 => {
                let text = self.doc.para(pos.path).map_or("", |p| p.text.as_str());
                let range = word_range(text, pos.offset);
                self.set_selection(
                    Pos::new(pos.path, range.start),
                    Pos::new(pos.path, range.end),
                    cx,
                );
            }
            n if n >= 3 => {
                let len = self.doc.para(pos.path).map_or(0, Para::len);
                self.set_selection(Pos::new(pos.path, 0), Pos::new(pos.path, len), cx);
            }
            _ => {
                self.selecting = true;
                self.clicked = (!event.modifiers.shift)
                    .then(|| self.mark_at(event.position))
                    .flatten()
                    .map(|mark| (mark.path, mark.range));
                if event.modifiers.shift {
                    self.set_selection(self.anchor, pos, cx);
                } else {
                    self.set_selection(pos, pos, cx);
                }
                self.upstream = upstream;
                self.goal_x = None;
            }
        }
    }

    fn right_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle, cx);
        // A click outside the selection moves the cursor there.
        if let Some((pos, _)) = self.hit(event.position) {
            let (start, end) = self.ordered();
            if !(start <= pos && pos <= end) {
                self.set_selection(pos, pos, cx);
            }
        }
        let misspelled = self.spell.as_ref().and_then(|spell| {
            let (_, word) = self.word_at_cursor()?;
            (!spell.check(&word)).then_some(word)
        });
        let grammar = if misspelled.is_none() {
            self.grammar_at_cursor()
        } else {
            None
        };
        cx.emit(RichEvent::ContextMenu {
            position: event.position,
            misspelled,
            grammar,
        });
    }

    /// A left click that came up where it went down on marked words shows
    /// their fixes.
    fn mouse_up(&mut self, event: &MouseUpEvent, cx: &mut Context<Self>) {
        self.selecting = false;
        let Some(clicked) = self.clicked.take() else {
            return;
        };
        if self.has_selection() {
            return;
        }
        if let Some(mark) = self.mark_at(event.position)
            && (mark.path, mark.range.clone()) == clicked
        {
            self.hover = Some(clicked);
            self.hover_task = None;
            self.emit_hint(mark, cx);
        }
    }

    /// Resting the pointer on marked words shows their fixes after a wait.
    fn mouse_move(&mut self, event: &MouseMoveEvent, window: &mut Window, cx: &mut Context<Self>) {
        if event.pressed_button.is_some() {
            return;
        }
        let key = self
            .mark_at(event.position)
            .map(|mark| (mark.path, mark.range));
        if key == self.hover {
            return;
        }
        self.hover = key.clone();
        self.hover_task = key.map(|key| {
            cx.spawn_in(window, async move |this, cx| {
                cx.background_executor().timer(HINT_WAIT).await;
                this.update_in(cx, |this, window, cx| {
                    this.hover_task = None;
                    match this.mark_at(window.mouse_position()) {
                        Some(mark) if (mark.path, mark.range.clone()) == key => {
                            this.emit_hint(mark, cx)
                        }
                        // The pointer left the editor meanwhile.
                        _ => this.hover = None,
                    }
                })
                .ok();
            })
        });
    }

    /// Forgets the words the pointer was on, so resting on them again
    /// shows their fixes again (the owner calls it on closing the card).
    pub fn end_hint(&mut self) {
        self.hover = None;
        self.hover_task = None;
    }

    fn emit_hint(&self, mark: Mark, cx: &mut Context<Self>) {
        let text = self.doc.para(mark.path).map_or("", |p| p.text.as_str());
        let word = text.get(mark.range.clone()).unwrap_or_default().to_owned();
        let suggestions = match &self.spell {
            Some(spell) if mark.misspelled => spell.suggest(&word),
            _ => Vec::new(),
        };
        cx.emit(RichEvent::Hint {
            word: mark.bounds,
            at: Pos::new(mark.path, mark.range.start),
            misspelled: mark.misspelled.then_some(word),
            suggestions,
            grammar: mark.grammar,
        });
    }

    /// The marked words under window point `p`, as drawn last.
    fn mark_at(&self, p: Point<Pixels>) -> Option<Mark> {
        let (pos, _) = self.hit(p)?;
        let para = self.doc.para(pos.path)?;
        let layout = self.layouts.get(&pos.path)?;
        let (range, deco) = self
            .decorations(pos.path, para)
            .into_iter()
            .find(|(r, d)| {
                (d.misspelled || d.grammar) && r.start <= pos.offset && pos.offset <= r.end
            })?;
        let rects: Vec<Bounds<Pixels>> = layout
            .selection_rects(range.clone(), false)
            .into_iter()
            .map(|r| Bounds::new(r.origin + layout.bounds.origin, r.size))
            .collect();
        if !rects.iter().any(|r| r.contains(&p)) {
            return None;
        }
        let bounds = rects.into_iter().reduce(|a, b| a.union(&b))?;
        let grammar = if deco.grammar {
            self.grammar_issues(para)
                .into_iter()
                .find(|issue| issue.range == range)
                .cloned()
        } else {
            None
        };
        Some(Mark {
            path: pos.path,
            range,
            bounds,
            misspelled: deco.misspelled,
            grammar,
        })
    }

    fn drag_to(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        if event.pressed_button != Some(MouseButton::Left) {
            self.selecting = false;
            return;
        }
        if let Some((pos, upstream)) = self.hit(event.position) {
            self.set_selection(self.anchor, pos, cx);
            self.upstream = upstream;
        }
    }

    // Formatting.

    /// The style the toolbar shows: of the selection's start, or what the
    /// next typed text gets.
    pub fn current_style(&self) -> CharStyle {
        if !self.has_selection() {
            return self.style_for_typing_raw();
        }
        let (start, _) = self.ordered();
        let para = self.doc.para(start.path);
        para.map(|p| {
            if start.offset < p.len() {
                p.style_at(start.offset + next_grapheme(&p.text, start.offset) - start.offset)
            } else {
                p.style_at(start.offset)
            }
        })
        .unwrap_or_default()
    }

    fn style_for_typing_raw(&self) -> CharStyle {
        if let Some(style) = &self.typing {
            return style.clone();
        }
        self.doc
            .para(self.head.path)
            .map(|p| p.style_at(self.head.offset))
            .unwrap_or_default()
    }

    /// Whether all selected text (or the typing style) matches `test`.
    pub fn all(&self, test: impl Fn(&CharStyle) -> bool) -> bool {
        if !self.has_selection() {
            return test(&self.style_for_typing_raw());
        }
        let (start, end) = self.ordered();
        self.doc.all(start, end, &test)
    }

    /// Changes the character style of the selection, or of what is typed
    /// next when nothing is selected.
    pub fn restyle(&mut self, f: impl Fn(&mut CharStyle) + 'static, cx: &mut Context<Self>) {
        if self.plain {
            return;
        }
        if !self.has_selection() && self.selected_image.is_none() {
            let mut style = self.style_for_typing_raw();
            f(&mut style);
            self.typing = Some(style);
            cx.emit(RichEvent::Selection);
            cx.notify();
            return;
        }
        self.format(cx, |doc, (start, end)| doc.restyle(start, end, &f));
    }

    fn toggle(
        &mut self,
        get: fn(&CharStyle) -> bool,
        set: fn(&mut CharStyle, bool),
        cx: &mut Context<Self>,
    ) {
        let on = !self.all(get);
        self.restyle(move |s| set(s, on), cx);
    }

    pub fn toggle_bold(&mut self, cx: &mut Context<Self>) {
        self.toggle(|s| s.bold, |s, v| s.bold = v, cx);
    }

    pub fn toggle_italic(&mut self, cx: &mut Context<Self>) {
        self.toggle(|s| s.italic, |s, v| s.italic = v, cx);
    }

    pub fn toggle_underline(&mut self, cx: &mut Context<Self>) {
        self.toggle(|s| s.underline, |s, v| s.underline = v, cx);
    }

    pub fn toggle_strike(&mut self, cx: &mut Context<Self>) {
        self.toggle(|s| s.strike, |s, v| s.strike = v, cx);
    }

    pub fn set_font(&mut self, font: Font, cx: &mut Context<Self>) {
        self.restyle(move |s| s.font = font, cx);
    }

    pub fn set_size(&mut self, size: Size, cx: &mut Context<Self>) {
        self.restyle(move |s| s.size = size, cx);
    }

    pub fn set_color(&mut self, color: Option<u32>, cx: &mut Context<Self>) {
        self.restyle(move |s| s.color = color, cx);
    }

    pub fn set_background(&mut self, color: Option<u32>, cx: &mut Context<Self>) {
        self.restyle(move |s| s.background = color, cx);
    }

    /// Removes character formatting (but keeps links) and paragraph
    /// formatting other than quotes and the signature mark.
    pub fn clear_formatting(&mut self, cx: &mut Context<Self>) {
        if !self.has_selection() {
            self.typing = Some(CharStyle::default());
        }
        self.format(cx, |doc, (start, end)| {
            doc.restyle(start, end, &|s| {
                *s = CharStyle {
                    link: s.link.clone(),
                    ..CharStyle::default()
                }
            });
            doc.restyle_paras(start, end, &|p| {
                *p = ParaStyle {
                    quote: p.quote,
                    signature: p.signature,
                    ..ParaStyle::default()
                }
            });
        });
    }

    /// The paragraph style where the cursor is.
    pub fn para_style(&self) -> ParaStyle {
        self.doc
            .para(self.head.path)
            .map(|p| p.style)
            .unwrap_or_default()
    }

    pub fn set_align(&mut self, align: Align, cx: &mut Context<Self>) {
        if self.plain {
            return;
        }
        self.format(cx, |doc, (start, end)| {
            doc.restyle_paras(start, end, &|p| p.align = align)
        });
    }

    /// Makes the selected paragraphs a list of `kind`, or plain again if
    /// they already are one.
    pub fn toggle_list(&mut self, kind: List, cx: &mut Context<Self>) {
        if self.plain || self.head.path.cell.is_some() {
            return;
        }
        let (start, end) = self.ordered();
        let all = self
            .doc
            .paras_in(start, end)
            .iter()
            .all(|p| p.style.list == kind);
        self.format(cx, |doc, (start, end)| {
            doc.restyle_paras(start, end, &|p| {
                p.list = if all { List::None } else { kind };
            })
        });
    }

    pub fn indent(&mut self, more: bool, cx: &mut Context<Self>) {
        if self.plain {
            return;
        }
        self.format(cx, |doc, (start, end)| {
            doc.restyle_paras(start, end, &|p| {
                p.indent = if more {
                    (p.indent + 1).min(MAX_INDENT)
                } else {
                    p.indent.saturating_sub(1)
                };
            })
        });
    }

    pub fn toggle_quote(&mut self, cx: &mut Context<Self>) {
        if self.head.path.cell.is_some() {
            return;
        }
        let quoted = self.para_style().quote > 0;
        self.format(cx, |doc, (start, end)| {
            doc.restyle_paras(start, end, &|p| {
                p.quote = if quoted {
                    p.quote.saturating_sub(1)
                } else {
                    p.quote + 1
                };
            })
        });
    }

    // Links.

    /// The link at the cursor: its text and address.
    pub fn link_at_cursor(&self) -> Option<(String, Arc<str>)> {
        let para = self.doc.para(self.head.path)?;
        let (range, url) = para.link_at(self.head.offset)?;
        Some((para.text[range].to_owned(), url))
    }

    /// Links `text` to `url`: replaces the link at the cursor, or the
    /// selection, or inserts it at the cursor.
    pub fn set_link(&mut self, text: &str, url: &str, cx: &mut Context<Self>) {
        let url: Arc<str> = url.into();
        let text = if text.trim().is_empty() {
            url.to_string()
        } else {
            text.replace('\n', " ")
        };
        if let Some(para) = self.doc.para(self.head.path)
            && let Some((range, _)) = para.link_at(self.head.offset)
            && !self.has_selection()
        {
            let path = self.head.path;
            self.anchor = Pos::new(path, range.start);
            self.head = Pos::new(path, range.end);
        }
        let mut style = self.style_for_typing_raw();
        style.link = Some(url);
        self.edit(EditKind::Other, cx, |doc, (start, end)| {
            let at = doc.delete(start, end);
            doc.insert_text(at, &text, &style)
        });
        // Text typed after the link is not part of it.
        let mut after = style_without_link(&self.style_for_typing_raw());
        after.link = None;
        self.typing = Some(after);
    }

    /// Unlinks the link at the cursor (or the selection).
    pub fn remove_link(&mut self, cx: &mut Context<Self>) {
        if !self.has_selection()
            && let Some(para) = self.doc.para(self.head.path)
            && let Some((range, _)) = para.link_at(self.head.offset)
        {
            let path = self.head.path;
            let head = self.head;
            self.edit(EditKind::Other, cx, |doc, _| {
                doc.restyle(
                    Pos::new(path, range.start),
                    Pos::new(path, range.end),
                    &|s| s.link = None,
                );
                head
            });
            return;
        }
        self.format(cx, |doc, (start, end)| {
            doc.restyle(start, end, &|s| s.link = None)
        });
    }

    // Tables and images.

    pub fn insert_table(&mut self, rows: usize, cols: usize, cx: &mut Context<Self>) {
        if self.plain {
            return;
        }
        self.edit(EditKind::Other, cx, |doc, (start, end)| {
            let at = doc.delete(start, end);
            doc.insert_table(at, rows, cols)
        });
    }

    pub fn in_table(&self) -> bool {
        self.head.path.cell.is_some()
    }

    pub fn edit_table(&mut self, edit: TableEdit, cx: &mut Context<Self>) {
        let path = self.head.path;
        if path.cell.is_none() {
            return;
        }
        self.edit(EditKind::Other, cx, |doc, _| match edit {
            TableEdit::RowAbove => doc.insert_row(path, false),
            TableEdit::RowBelow => doc.insert_row(path, true),
            TableEdit::ColumnLeft => doc.insert_col(path, false),
            TableEdit::ColumnRight => doc.insert_col(path, true),
            TableEdit::DeleteRow => doc.delete_row(path),
            TableEdit::DeleteColumn => doc.delete_col(path),
            TableEdit::DeleteTable => doc.remove_block(path.block),
        });
    }

    /// Inserts a picture at the cursor. Returns whether its format is one
    /// the editor can show.
    pub fn insert_image(
        &mut self,
        name: String,
        mime: String,
        data: Vec<u8>,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.plain || format_of(&mime).is_none() {
            return false;
        }
        let (width, height) = image_size(&data).unwrap_or((0, 0));
        self.next_image_id += 1;
        let image = Image {
            id: self.next_image_id,
            name,
            mime,
            data: Arc::new(data),
            width,
            height,
            size: ImageSize::BestFit,
        };
        self.edit(EditKind::Other, cx, |doc, (start, end)| {
            let at = doc.delete(start, end);
            doc.insert_image(at, image)
        });
        true
    }

    pub fn selected_image(&self) -> Option<&Image> {
        match self.doc.blocks.get(self.selected_image?)? {
            Block::Image(image) => Some(image),
            _ => None,
        }
    }

    pub fn set_image_size(&mut self, size: ImageSize, cx: &mut Context<Self>) {
        let Some(block) = self.selected_image else {
            return;
        };
        let head = self.head;
        self.edit(EditKind::Other, cx, |doc, _| {
            if let Some(image) = doc.image_mut(block) {
                image.size = size;
            }
            head
        });
        self.selected_image = Some(block);
    }

    pub fn remove_selected_image(&mut self, cx: &mut Context<Self>) {
        if let Some(block) = self.selected_image.take() {
            self.edit(EditKind::Other, cx, |doc, _| doc.remove_block(block));
        }
    }

    fn select_image(&mut self, block: usize, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.focus_handle, cx);
        let after = Pos::new(Path::top(block + 1), 0);
        self.set_selection(after, after, cx);
        self.selected_image = Some(block);
        cx.emit(RichEvent::Selection);
        cx.notify();
    }

    // Signature.

    /// Replaces the signature (the paragraphs marked as such) with
    /// `signature`, put where the old one was or else after the text the
    /// user wrote, before any quoted message.
    pub fn replace_signature(&mut self, signature: Option<Doc>, cx: &mut Context<Self>) {
        let head = self.head;
        self.edit(EditKind::Other, cx, |doc, _| {
            let at = doc
                .remove_signature()
                .unwrap_or_else(|| signature_place(doc));
            if let Some(signature) = signature {
                insert_signature_doc(doc, at, signature);
            }
            head
        });
        let used = self.doc.images().map(|i| i.id).max().unwrap_or(0);
        self.next_image_id = self.next_image_id.max(used);
    }

    // Undo.

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    pub fn undo(&mut self, cx: &mut Context<Self>) {
        let Some(snapshot) = self.undo.pop() else {
            return;
        };
        self.redo.push(self.snapshot());
        self.restore(snapshot, cx);
    }

    pub fn redo(&mut self, cx: &mut Context<Self>) {
        let Some(snapshot) = self.redo.pop() else {
            return;
        };
        self.undo.push(self.snapshot());
        self.restore(snapshot, cx);
    }

    fn restore(&mut self, snapshot: Snapshot, cx: &mut Context<Self>) {
        self.doc = snapshot.doc;
        self.paste_offer = None;
        self.anchor = self.doc.clamp(snapshot.anchor);
        self.head = self.doc.clamp(snapshot.head);
        self.last_edit = None;
        self.selected_image = None;
        self.typing = None;
        cx.emit(RichEvent::Changed);
        cx.emit(RichEvent::Selection);
        cx.notify();
    }

    // Tab in tables and lists.

    fn next_cell(&mut self, _: &NextCell, _: &mut Window, cx: &mut Context<Self>) {
        let path = self.head.path;
        if let Some((row, col)) = path.cell {
            let table = self.doc.table(path.block);
            let (rows, cols) = table.map_or((0, 0), |t| (t.rows.len(), t.cols()));
            if col + 1 < cols {
                self.move_to(Pos::new(Path::cell(path.block, row, col + 1), 0), cx);
            } else if row + 1 < rows {
                self.move_to(Pos::new(Path::cell(path.block, row + 1, 0), 0), cx);
            } else {
                // Tab in the last cell adds a row.
                let at = Path::cell(path.block, row, 0);
                self.edit(EditKind::Other, cx, |doc, _| doc.insert_row(at, true));
            }
            return;
        }
        if self.para_style().list != List::None {
            self.indent(true, cx);
            return;
        }
        cx.propagate();
    }

    fn prev_cell(&mut self, _: &PrevCell, _: &mut Window, cx: &mut Context<Self>) {
        let path = self.head.path;
        if let Some((row, col)) = path.cell {
            let cols = self.doc.table(path.block).map_or(0, |t| t.cols());
            let target = if col > 0 {
                Some(Path::cell(path.block, row, col - 1))
            } else if row > 0 {
                Some(Path::cell(path.block, row - 1, cols.saturating_sub(1)))
            } else {
                None
            };
            if let Some(target) = target {
                self.move_to(Pos::new(target, 0), cx);
            }
            return;
        }
        if self.para_style().list != List::None || self.para_style().indent > 0 {
            self.indent(false, cx);
            return;
        }
        cx.propagate();
    }

    // Rendering.

    fn render_block(&self, ix: usize, block: &Block, cx: &mut Context<Self>) -> AnyElement {
        let editor = cx.entity();
        let palette = self.palette;
        let only = self.doc.blocks.len() == 1;
        match block {
            Block::Para(para) => {
                let style = para.style;
                let placeholder = (only && para.is_empty() && !self.placeholder.is_empty())
                    .then(|| self.placeholder.clone());
                let mut el = div()
                    .w_full()
                    .min_w_0()
                    .child(ParaElement {
                        editor,
                        path: Path::top(ix),
                        placeholder,
                    })
                    .into_any_element();
                let indent = if style.list == List::None {
                    40.0 * f32::from(style.indent)
                } else {
                    24.0 + 24.0 * f32::from(style.indent)
                };
                if indent > 0.0 {
                    el = div().w_full().pl(px(indent)).child(el).into_any_element();
                }
                for _ in 0..style.quote {
                    el = div()
                        .w_full()
                        .pl(px(10.0))
                        .border_l_1()
                        .border_color(palette.rule)
                        .child(el)
                        .into_any_element();
                }
                el
            }
            Block::Table(table) => {
                let rows = table.rows.iter().enumerate().map(|(r, row)| {
                    div()
                        .flex()
                        .flex_row()
                        .w_full()
                        .children(row.iter().enumerate().map(|(c, cell)| {
                            div()
                                .flex_1()
                                .min_w(px(40.0))
                                .px(px(8.0))
                                .py(px(4.0))
                                .border_1()
                                .border_color(palette.rule)
                                .when_some(cell.style.fill, |d, fill| {
                                    // Dimmed under light text (a dark
                                    // theme), so the text stays readable.
                                    let dim = if palette.text.l > 0.5 { 0.35 } else { 1.0 };
                                    d.bg(super::layout::rgb(fill).opacity(dim))
                                })
                                .child(ParaElement {
                                    editor: editor.clone(),
                                    path: Path::cell(ix, r, c),
                                    placeholder: None,
                                })
                        }))
                });
                div()
                    .w_full()
                    .my(px(4.0))
                    .flex()
                    .flex_col()
                    .children(rows)
                    .into_any_element()
            }
            Block::Image(image) => self.render_image(ix, image, cx),
        }
    }

    fn render_image(&self, ix: usize, image: &Image, cx: &mut Context<Self>) -> AnyElement {
        let room = unpx(self.width).max(120.0);
        let width = image.display_width(room);
        let height = if image.width > 0 {
            width * image.height as f32 / image.width as f32
        } else {
            width * 0.6
        };
        let selected = self.selected_image == Some(ix);
        let palette = self.palette;
        let source = self.images.get(&image.id).cloned();
        let picture = match source {
            Some(source) => img(source)
                .w(px(width))
                .h(px(height))
                .object_fit(ObjectFit::Contain)
                .into_any_element(),
            None => div().w(px(width)).h(px(height)).into_any_element(),
        };
        let button = |id: &'static str, label: &'static str, on: bool| {
            div()
                .id(id)
                .px(px(8.0))
                .py(px(4.0))
                .rounded(px(4.0))
                .text_size(px(13.0))
                .text_color(if on { palette.accent } else { palette.text })
                .cursor_pointer()
                .hover(|s| s.bg(palette.hover))
                .child(label)
        };
        div()
            .id(("rich-image", ix))
            .relative()
            .my(px(4.0))
            .w(px(width + 4.0))
            .p(px(2.0))
            .border_2()
            .border_color(if selected {
                palette.accent
            } else {
                gpui::transparent_black()
            })
            .cursor(CursorStyle::PointingHand)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, window, cx| {
                    cx.stop_propagation();
                    this.select_image(ix, window, cx);
                }),
            )
            .child(picture)
            .when(selected, |d| {
                d.child(
                    div()
                        .absolute()
                        .left(px(8.0))
                        .bottom(px(8.0))
                        .flex()
                        .flex_row()
                        .gap(px(2.0))
                        .p(px(4.0))
                        .rounded(px(8.0))
                        .bg(palette.surface)
                        .shadow_md()
                        .child(
                            button("rich-image-small", "Small", image.size == ImageSize::Small)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _, _, cx| {
                                        cx.stop_propagation();
                                        this.set_image_size(ImageSize::Small, cx)
                                    }),
                                ),
                        )
                        .child(
                            button(
                                "rich-image-fit",
                                "Best fit",
                                image.size == ImageSize::BestFit,
                            )
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    cx.stop_propagation();
                                    this.set_image_size(ImageSize::BestFit, cx)
                                }),
                            ),
                        )
                        .child(
                            button(
                                "rich-image-original",
                                "Original size",
                                image.size == ImageSize::Original,
                            )
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    cx.stop_propagation();
                                    this.set_image_size(ImageSize::Original, cx)
                                }),
                            ),
                        )
                        .child(button("rich-image-remove", "Remove", false).on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, cx| {
                                cx.stop_propagation();
                                this.remove_selected_image(cx)
                            }),
                        )),
                )
            })
            .into_any_element()
    }

    /// Keeps decoded pictures for the images in the document.
    fn sync_images(&mut self) {
        let ids: Vec<u64> = self.doc.images().map(|i| i.id).collect();
        self.images.retain(|id, _| ids.contains(id));
        for image in self.doc.images() {
            if self.images.contains_key(&image.id) {
                continue;
            }
            if let Some(format) = format_of(&image.mime) {
                self.images.insert(
                    image.id,
                    Arc::new(gpui::Image::from_bytes(format, image.data.to_vec())),
                );
            }
        }
    }
}

fn style_without_link(style: &CharStyle) -> CharStyle {
    CharStyle {
        link: None,
        ..style.clone()
    }
}

/// Where a new signature goes: after the last paragraph the user wrote,
/// before a quoted message or forwarded header.
fn signature_place(doc: &Doc) -> usize {
    let quote = doc.blocks.iter().position(|b| match b {
        Block::Para(p) => {
            p.style.quote > 0
                || p.text.contains("Forwarded message")
                || (p.text.starts_with("On ") && p.text.trim_end().ends_with("wrote:"))
        }
        _ => false,
    });
    let end = quote.unwrap_or(doc.blocks.len());
    // After the text, keeping one empty line.
    let mut at = end;
    while at > 0 && matches!(&doc.blocks[at - 1], Block::Para(p) if p.is_empty()) {
        at -= 1;
    }
    (at + 1).min(end.max(at))
}

/// Puts a signature document at block `at`, marked as the signature and
/// after a `-- ` line.
pub fn insert_signature_doc(doc: &mut Doc, at: usize, signature: Doc) {
    let mut blocks = vec![Block::Para(Para::plain("-- "))];
    blocks.extend(signature.blocks);
    // Trailing empty lines are not part of it.
    while matches!(blocks.last(), Some(Block::Para(p)) if p.text.trim().is_empty())
        && blocks.len() > 1
    {
        blocks.pop();
    }
    // It ends on a marked paragraph, so that removing it takes the
    // pictures and tables at its end too.
    if !matches!(blocks.last(), Some(Block::Para(_))) {
        blocks.push(Block::Para(Para::default()));
    }
    // Its pictures get IDs of their own in the message.
    let mut next = doc.images().map(|i| i.id).max().unwrap_or(0);
    for block in &mut blocks {
        if let Block::Image(image) = block {
            next += 1;
            image.id = next;
        }
    }
    let at = at.min(doc.blocks.len());
    for (ix, mut block) in blocks.into_iter().enumerate() {
        if let Block::Para(p) = &mut block {
            p.style.signature = true;
        }
        doc.blocks.insert(at + ix, block);
    }
}

/// Distance from `p` to `bounds` (0 inside), weighting vertical distance
/// so the row under the pointer wins.
fn distance(bounds: Bounds<Pixels>, p: Point<Pixels>) -> f32 {
    let dy = if p.y < bounds.top() {
        bounds.top() - p.y
    } else if p.y >= bounds.bottom() {
        p.y - bounds.bottom() + px(0.5)
    } else {
        px(0.0)
    };
    unpx(dy) * 4.0 + horizontal(bounds, p.x)
}

fn horizontal(bounds: Bounds<Pixels>, x: Pixels) -> f32 {
    unpx(if x < bounds.left() {
        bounds.left() - x
    } else if x > bounds.right() {
        x - bounds.right()
    } else {
        px(0.0)
    })
}

/// Misspelled words of `text`.
fn misspelled(text: &str, spell: &dyn SpellCheck) -> Vec<Range<usize>> {
    use unicode_segmentation::UnicodeSegmentation;
    text.split_word_bound_indices()
        .filter(|(_, w)| w.chars().any(char::is_alphabetic))
        // Addresses and the like are not words.
        .filter(|(_, w)| {
            !w.contains(['@', '/', '.', '_']) && !w.chars().any(|c| c.is_ascii_digit())
        })
        .filter(|(_, w)| !spell.check(w))
        .map(|(ix, w)| ix..ix + w.len())
        .collect()
}

/// The installed family to draw each typeface with.
fn resolve_families(window: &Window) -> HashMap<Font, SharedString> {
    let installed: std::collections::HashSet<String> = window
        .text_system()
        .all_font_names()
        .into_iter()
        .map(|n| n.to_lowercase())
        .collect();
    Font::ALL
        .into_iter()
        .filter_map(|font| {
            let family = font
                .families()
                .iter()
                .find(|f| installed.contains(&f.to_lowercase()))?;
            Some((font, SharedString::from(*family)))
        })
        .collect()
}

fn format_of(mime: &str) -> Option<ImageFormat> {
    Some(match mime {
        "image/png" => ImageFormat::Png,
        "image/jpeg" | "image/jpg" => ImageFormat::Jpeg,
        "image/gif" => ImageFormat::Gif,
        "image/webp" => ImageFormat::Webp,
        "image/bmp" => ImageFormat::Bmp,
        "image/svg+xml" => ImageFormat::Svg,
        _ => return None,
    })
}

fn mime_of(format: ImageFormat) -> (&'static str, &'static str) {
    match format {
        ImageFormat::Jpeg => ("image/jpeg", "jpg"),
        ImageFormat::Gif => ("image/gif", "gif"),
        ImageFormat::Webp => ("image/webp", "webp"),
        ImageFormat::Bmp => ("image/bmp", "bmp"),
        ImageFormat::Svg => ("image/svg+xml", "svg"),
        _ => ("image/png", "png"),
    }
}

/// The MIME type of an image file name, if the editor can show it.
pub fn image_mime(name: &str) -> Option<&'static str> {
    let ext = name.rsplit_once('.')?.1.to_ascii_lowercase();
    Some(match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        _ => return None,
    })
}

impl EntityInputHandler for RichEditor {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let text = &self.doc.para(self.head.path)?.text;
        let range = range_from_utf16(text, &range_utf16);
        actual_range.replace(range_to_utf16(text, &range));
        Some(text[range].to_owned())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let text = &self.doc.para(self.head.path)?.text;
        let range = match self.selection_in(self.head.path) {
            Some((range, _)) if self.anchor.path == self.head.path => range,
            _ => self.head.offset..self.head.offset,
        };
        Some(UTF16Selection {
            range: range_to_utf16(text, &range),
            reversed: self.head < self.anchor,
        })
    }

    fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
        let text = &self.doc.para(self.head.path)?.text;
        self.marked.as_ref().map(|r| range_to_utf16(text, r))
    }

    fn unmark_text(&mut self, _: &mut Window, _: &mut Context<Self>) {
        self.marked = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(block) = self.selected_image.take() {
            // Typing on a picture types on the line after it.
            let pos = Pos::new(Path::top(block + 1), 0);
            self.anchor = pos;
            self.head = pos;
        }
        let text = self
            .doc
            .para(self.head.path)
            .map_or("", |p| p.text.as_str());
        let range = range_utf16
            .map(|r| range_from_utf16(text, &r))
            .or(self.marked.clone());
        if let Some(range) = range {
            self.anchor = Pos::new(self.head.path, range.start);
            self.head = Pos::new(self.head.path, range.end);
        }
        self.marked = None;
        let typing = self.typing.take();
        self.typing = typing;
        self.insert(new_text, cx);
        if !new_text.contains('\n') {
            self.request_suggestion();
        }
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let text = self
            .doc
            .para(self.head.path)
            .map_or("", |p| p.text.as_str());
        let range = range_utf16
            .map(|r| range_from_utf16(text, &r))
            .or(self.marked.clone())
            .unwrap_or_else(|| {
                self.selection_in(self.head.path)
                    .filter(|_| self.anchor.path == self.head.path)
                    .map_or(self.head.offset..self.head.offset, |(r, _)| r)
            });
        let path = self.head.path;
        let start = range.start;
        self.anchor = Pos::new(path, range.start);
        self.head = Pos::new(path, range.end);
        self.marked = None;
        self.replace_text_in_range(None, new_text, window, cx);
        // No suggestion while a character is being composed.
        self.ghost = None;
        self.marked = (!new_text.is_empty()).then(|| start..start + new_text.len());
        if let Some(sel) = new_selected_range_utf16 {
            let sel = range_from_utf16(new_text, &sel);
            self.anchor = Pos::new(path, start + sel.start);
            self.head = Pos::new(path, start + sel.end);
        }
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        _bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let layout = self.layouts.get(&self.head.path)?;
        let text = &self.doc.para(self.head.path)?.text;
        let range = range_from_utf16(text, &range_utf16);
        let (start, height) = layout.caret(range.start, false);
        let (end, _) = layout.caret(range.end, false);
        let end_x = if end.y == start.y {
            end.x.max(start.x)
        } else {
            start.x
        };
        Some(Bounds::from_corners(
            layout.bounds.origin + start,
            layout.bounds.origin + point(end_x, start.y + height),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let layout = self.layouts.get(&self.head.path)?;
        if !layout.bounds.contains(&point) {
            return None;
        }
        let text = &self.doc.para(self.head.path)?.text;
        let (offset, _) = layout.hit(text, point);
        Some(offset_to_utf16(text, offset))
    }
}

fn offset_from_utf16(text: &str, offset: usize) -> usize {
    let mut utf8 = 0;
    let mut utf16 = 0;
    for ch in text.chars() {
        if utf16 >= offset {
            break;
        }
        utf16 += ch.len_utf16();
        utf8 += ch.len_utf8();
    }
    utf8
}

fn offset_to_utf16(text: &str, offset: usize) -> usize {
    let mut utf16 = 0;
    let mut utf8 = 0;
    for ch in text.chars() {
        if utf8 >= offset {
            break;
        }
        utf8 += ch.len_utf8();
        utf16 += ch.len_utf16();
    }
    utf16
}

fn range_to_utf16(text: &str, range: &Range<usize>) -> Range<usize> {
    offset_to_utf16(text, range.start)..offset_to_utf16(text, range.end)
}

fn range_from_utf16(text: &str, range: &Range<usize>) -> Range<usize> {
    let start = doc::clamp(text, offset_from_utf16(text, range.start));
    let end = doc::clamp(text, offset_from_utf16(text, range.end));
    start..end.max(start)
}

/// Registers the editor's input handler and drag tracking; takes the
/// editor's whole area.
struct Anchor {
    editor: Entity<RichEditor>,
}

impl IntoElement for Anchor {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for Anchor {
    type RequestLayoutState = ();
    type PrepaintState = ();

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
    ) -> (LayoutId, ()) {
        let mut style = Style {
            position: Position::Absolute,
            ..Style::default()
        };
        style.inset.top = px(0.0).into();
        style.inset.left = px(0.0).into();
        style.size.width = relative(1.).into();
        style.size.height = relative(1.).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _: &mut (),
        _window: &mut Window,
        _cx: &mut App,
    ) {
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus = self.editor.read(cx).focus_handle.clone();
        window.handle_input(
            &focus,
            ElementInputHandler::new(bounds, self.editor.clone()),
            cx,
        );
        window.on_mouse_event({
            let editor = self.editor.clone();
            move |event: &MouseMoveEvent, phase, _window, cx| {
                if phase == DispatchPhase::Bubble && editor.read(cx).selecting {
                    editor.update(cx, |editor, cx| editor.drag_to(event, cx));
                }
            }
        });
        window.on_mouse_event({
            let editor = self.editor.clone();
            move |event: &MouseUpEvent, phase, _window, cx| {
                if phase == DispatchPhase::Bubble
                    && event.button == MouseButton::Left
                    && editor.read(cx).selecting
                {
                    editor.update(cx, |editor, cx| editor.mouse_up(event, cx));
                }
            }
        });
        let width = bounds.size.width;
        self.editor.update(cx, |editor, cx| {
            if editor.width != width {
                editor.width = width;
                // Images size to the width; draw again with it.
                if editor.doc.images().next().is_some() {
                    cx.notify();
                }
            }
        });
    }
}

impl Render for RichEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.numbers = list_numbers(&self.doc);
        self.layouts.clear();
        self.sync_images();
        self.request_grammar(cx);
        let blocks: Vec<AnyElement> = self
            .doc
            .blocks
            .clone()
            .iter()
            .enumerate()
            .map(|(ix, block)| {
                let el = self.render_block(ix, block, cx);
                match self.render_paste_options(ix, cx) {
                    Some(options) => div()
                        .relative()
                        .w_full()
                        .min_w_0()
                        .child(el)
                        .child(options)
                        .into_any_element(),
                    None => el,
                }
            })
            .collect();
        let suggesting = if self.showing_suggestion() {
            format!(" {SUGGESTING_CONTEXT}")
        } else {
            String::new()
        };
        div()
            .relative()
            .w_full()
            .min_w_0()
            .flex()
            .flex_col()
            .key_context(format!("{TEXT_AREA_CONTEXT} {RICH_TEXT_CONTEXT}{suggesting}").as_str())
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
            .on_action(cx.listener(Self::paste_plain))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::copy))
            .on_action(cx.listener(Self::submit))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::next_cell))
            .on_action(cx.listener(Self::prev_cell))
            .on_action(cx.listener(Self::accept_suggestion))
            .on_action(cx.listener(Self::dismiss_suggestion))
            .on_action(cx.listener(|this, _: &Bold, _, cx| this.toggle_bold(cx)))
            .on_action(cx.listener(|this, _: &Italic, _, cx| this.toggle_italic(cx)))
            .on_action(cx.listener(|this, _: &Underline, _, cx| this.toggle_underline(cx)))
            .on_action(cx.listener(|this, _: &Strikethrough, _, cx| this.toggle_strike(cx)))
            .on_action(
                cx.listener(|this, _: &NumberedList, _, cx| this.toggle_list(List::Numbered, cx)),
            )
            .on_action(
                cx.listener(|this, _: &BulletList, _, cx| this.toggle_list(List::Bullet, cx)),
            )
            .on_action(cx.listener(|this, _: &Quote, _, cx| this.toggle_quote(cx)))
            .on_action(cx.listener(|this, _: &IndentMore, _, cx| this.indent(true, cx)))
            .on_action(cx.listener(|this, _: &IndentLess, _, cx| this.indent(false, cx)))
            .on_action(cx.listener(|this, _: &AlignLeft, _, cx| this.set_align(Align::Left, cx)))
            .on_action(
                cx.listener(|this, _: &AlignCenter, _, cx| this.set_align(Align::Center, cx)),
            )
            .on_action(cx.listener(|this, _: &AlignRight, _, cx| this.set_align(Align::Right, cx)))
            .on_action(cx.listener(|this, _: &ClearFormatting, _, cx| this.clear_formatting(cx)))
            .on_action(cx.listener(|this, _: &Undo, _, cx| this.undo(cx)))
            .on_action(cx.listener(|this, _: &Redo, _, cx| this.redo(cx)))
            .on_action(cx.listener(|_, _: &InsertLink, _, cx| cx.emit(RichEvent::EditLink)))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::mouse_down))
            .on_mouse_down(MouseButton::Right, cx.listener(Self::right_mouse_down))
            .on_mouse_move(cx.listener(Self::mouse_move))
            .child(Anchor {
                editor: cx.entity(),
            })
            .children(blocks)
    }
}

impl Focusable for RichEditor {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_goes_before_the_quote() {
        let mut doc = html::from_plain("Hi\n\nOn Tue, Kay wrote:\n> hello");
        let at = signature_place(&doc);
        insert_signature_doc(&mut doc, at, html::from_plain("Kay"));
        let texts: Vec<_> = doc
            .blocks
            .iter()
            .map(|b| match b {
                Block::Para(p) => p.text.clone(),
                _ => String::new(),
            })
            .collect();
        assert_eq!(
            texts,
            ["Hi", "", "-- ", "Kay", "On Tue, Kay wrote:", "hello"]
        );
    }

    #[test]
    fn utf16_ranges() {
        let text = "a😀é";
        assert_eq!(range_to_utf16(text, &(1..5)), 1..3);
        assert_eq!(range_from_utf16(text, &(1..3)), 1..5);
    }
}
