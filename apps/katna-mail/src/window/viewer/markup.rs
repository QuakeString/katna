// SPDX-License-Identifier: GPL-3.0-or-later

//! Marking up a PDF in the attachment viewer. "Mark up" in the top bar
//! puts a pill of tools under it: select text, highlight, underline,
//! squiggle, strike through, pen, sticky note, text box and eraser, five
//! colours, and undo and redo. The text tools mark what a drag (or a
//! double or triple click) selects; the pen draws; a click with the note
//! or text tool places one and opens it for typing (Ctrl+Enter or a click
//! elsewhere finishes, Escape drops the change); clicking a note or a
//! text box opens it again; the eraser removes the marks it touches.
//!
//! Marks are drawn over the page's picture and go into a file only when
//! saving: Save then writes a copy with them as standard PDF annotations
//! (`katna_preview::pdf::Document::with_marks`), which Okular, Acrobat
//! and browsers show too. The attachment itself never changes. Closing
//! the viewer or moving to another attachment with unsaved marks asks
//! first. Encrypted and certified PDFs are not marked up.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use gpui::{
    AnyElement, Bounds, Context, CursorStyle, DispatchPhase, Entity, Focusable, FontWeight,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PathBuilder, Pixels, Point, Rgba,
    SharedString, Stateful, Subscription, Task, Window, canvas, div, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_preview::markup::{
    Kind, LINE_HEIGHT, Mark, Marks, NOTE_SIZE, PEN_WIDTH, Quad, Shape, TEXT_SIZE, TEXT_WIDTH, wrap,
};
use katna_preview::pdf::SaveError;
use katna_render::AttachmentFile;
use katna_ui::{InputEvent, TextArea};
use katna_ui::{Ripple, px, unpx};

use super::{
    BAR_HEIGHT, Content, HOVER, INK, INK_DIM, PILL, PILL_FROSTED, PdfView, Viewer, ViewerEvent,
    glassy,
};
use crate::theme::Theme;
use crate::widgets::{ScaledEdge, icon, tip};

/// Highlighter colours: light, as they lie under the text.
const MARKERS: [(&str, u32); 5] = [
    ("viewer-color-yellow", 0xffe14d),
    ("viewer-color-green", 0x8ee08f),
    ("viewer-color-blue", 0x8ccaff),
    ("viewer-color-pink", 0xffa3d7),
    ("viewer-color-orange", 0xffb65e),
];

/// Pen and line colours.
const INKS: [(&str, u32); 5] = [
    ("viewer-color-red", 0xd93025),
    ("viewer-color-blue", 0x1a73e8),
    ("viewer-color-green", 0x188038),
    ("viewer-color-black", 0x202124),
    ("viewer-color-purple", 0x9334e6),
];

const DOTS: [&str; 5] = [
    "viewer-color-0",
    "viewer-color-1",
    "viewer-color-2",
    "viewer-color-3",
    "viewer-color-4",
];

/// On screen, a highlight lets the page show through this much.
const HIGHLIGHT_ALPHA: f32 = 0.45;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Tool {
    Select,
    Mark(Kind),
    Eraser,
}

/// The tools in the pill: the tool, its icon and its name's id.
const TOOLS: [(Tool, &str, &str, &str); 9] = [
    (
        Tool::Select,
        "viewer-tool-select",
        "pointer",
        "viewer-tool-select",
    ),
    (
        Tool::Mark(Kind::Highlight),
        "viewer-tool-highlight",
        "highlight",
        "viewer-tool-highlight",
    ),
    (
        Tool::Mark(Kind::Underline),
        "viewer-tool-underline",
        "format-underline",
        "viewer-tool-underline",
    ),
    (
        Tool::Mark(Kind::Squiggly),
        "viewer-tool-squiggly",
        "format-squiggle",
        "viewer-tool-squiggly",
    ),
    (
        Tool::Mark(Kind::StrikeOut),
        "viewer-tool-strike",
        "format-strike",
        "viewer-tool-strike",
    ),
    (
        Tool::Mark(Kind::Ink),
        "viewer-tool-pen",
        "pen",
        "viewer-tool-pen",
    ),
    (
        Tool::Mark(Kind::Note),
        "viewer-tool-note",
        "notes",
        "viewer-tool-note",
    ),
    (
        Tool::Mark(Kind::FreeText),
        "viewer-tool-text",
        "format-text",
        "viewer-tool-text",
    ),
    (
        Tool::Eraser,
        "viewer-tool-eraser",
        "eraser",
        "viewer-tool-eraser",
    ),
];

/// Where a marked copy of the PDF goes.
#[derive(Clone, Copy)]
enum Marked {
    Save,
    Reply,
    Forward,
}

/// What to do once unsaved marks are dealt with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Leave {
    Close,
    Show(usize),
    /// Another file of the Files page's list, this many places on.
    Out(isize),
}

/// A note or a text box open for typing.
struct Typing {
    page: usize,
    kind: Kind,
    /// Its top left, in points.
    at: (f32, f32),
    /// A text box's width, in points.
    width: f32,
    color: [f32; 3],
    /// The mark it changes, or none for a new one.
    editing: Option<usize>,
    area: Entity<TextArea>,
    _events: Subscription,
}

pub(super) struct Markup {
    /// The pill of tools is out.
    on: bool,
    tool: Tool,
    /// The last kind of mark chosen: its colours stay in the pill (dimmed)
    /// while selecting or erasing, so the pill keeps its size.
    last: Kind,
    /// Each kind of mark's colour: a place in its palette.
    colors: [usize; 7],
    pub(super) marks: Marks,
    /// A pen stroke being drawn: its page and points.
    stroke: Option<(usize, Vec<(f32, f32)>)>,
    /// The eraser is down.
    erasing: bool,
    /// Whether this PDF can take marks, once checked.
    allowed: Option<Result<(), SaveError>>,
    /// The last save failed.
    failed: bool,
    _check: Option<Task<()>>,
    _saving: Option<Task<()>>,
    /// Unsaved marks and a wish to leave: the question is on screen.
    ask: Option<Leave>,
    /// Where each page was on screen at the last frame.
    spots: Rc<RefCell<HashMap<usize, Bounds<Pixels>>>>,
    /// The note or text box being typed.
    typing: Option<Typing>,
    /// Typing ended: the viewer takes the keys back at the next frame.
    refocus: bool,
}

impl Markup {
    /// Drops a pen stroke or erasing still under way (the page turned).
    pub(super) fn stroke_cancel(&mut self) {
        self.stroke = None;
        self.erasing = false;
    }

    pub(super) fn new() -> Self {
        Self {
            on: false,
            tool: Tool::Select,
            last: Kind::Highlight,
            colors: [0; 7],
            marks: Marks::default(),
            stroke: None,
            erasing: false,
            allowed: None,
            failed: false,
            _check: None,
            _saving: None,
            ask: None,
            spots: Rc::default(),
            typing: None,
            refocus: false,
        }
    }

    /// A note or a text box is open for typing: the keys are its.
    pub(super) fn typing(&self) -> bool {
        self.typing.is_some()
    }

    /// Whether the viewer should take the keys back, once.
    pub(super) fn take_refocus(&mut self) -> bool {
        std::mem::take(&mut self.refocus)
    }

    /// The question about unsaved marks is on screen.
    pub(super) fn asking(&self) -> bool {
        self.ask.is_some()
    }

    /// The pen, the note or text tool or the eraser is in use: the pages
    /// take the pointer.
    pub(super) fn drawing(&self) -> bool {
        self.on
            && self.allowed == Some(Ok(()))
            && matches!(
                self.tool,
                Tool::Mark(Kind::Ink | Kind::Note | Kind::FreeText) | Tool::Eraser
            )
    }
}

fn slot(kind: Kind) -> usize {
    match kind {
        Kind::Highlight => 0,
        Kind::Underline => 1,
        Kind::Squiggly => 2,
        Kind::StrikeOut => 3,
        Kind::Ink => 4,
        Kind::Note => 5,
        Kind::FreeText => 6,
    }
}

fn palette(kind: Kind) -> &'static [(&'static str, u32); 5] {
    if matches!(kind, Kind::Highlight | Kind::Note) {
        &MARKERS
    } else {
        &INKS
    }
}

fn to_color(rgb: u32) -> [f32; 3] {
    [(rgb >> 16) & 0xff, (rgb >> 8) & 0xff, rgb & 0xff].map(|c| c as f32 / 255.0)
}

fn paint_color([r, g, b]: [f32; 3], a: f32) -> Rgba {
    Rgba { r, g, b, a }
}

impl Viewer {
    fn pdf(&self) -> Option<&PdfView> {
        match &self.content {
            Content::Pdf(pdf) => Some(pdf),
            _ => None,
        }
    }

    /// The tool that marks, when marking up is on and allowed.
    fn marking(&self) -> Option<Tool> {
        let m = &self.markup;
        (m.on && m.allowed == Some(Ok(())) && m.tool != Tool::Select).then_some(m.tool)
    }

    /// Shows or hides the tools; the first time, checks whether the PDF
    /// can take marks.
    pub(super) fn toggle_markup(&mut self, cx: &mut Context<Self>) {
        let Some(doc) = self.pdf().map(|pdf| pdf.doc.clone()) else {
            return;
        };
        self.finish_typing(true, cx);
        let m = &mut self.markup;
        m.on = !m.on;
        m.failed = false;
        m.stroke = None;
        if m.on && m.tool == Tool::Select {
            m.tool = Tool::Mark(Kind::Highlight);
        }
        if m.on && m.allowed.is_none() && m._check.is_none() {
            m._check = Some(cx.spawn(async move |this, cx| {
                let allowed = cx
                    .background_executor()
                    .spawn(async move { doc.can_mark() })
                    .await;
                this.update(cx, |this, cx| {
                    this.markup.allowed = Some(allowed);
                    cx.notify();
                })
                .ok();
            }));
        }
        cx.notify();
    }

    fn set_tool(&mut self, tool: Tool, cx: &mut Context<Self>) {
        self.finish_typing(true, cx);
        self.markup.tool = tool;
        if let Tool::Mark(kind) = tool {
            self.markup.last = kind;
        }
        self.markup.stroke = None;
        self.text.clear();
        cx.notify();
    }

    fn set_color(&mut self, ix: usize, cx: &mut Context<Self>) {
        if let Tool::Mark(kind) = self.markup.tool {
            self.markup.colors[slot(kind)] = ix;
            cx.notify();
        }
    }

    fn color_of(&self, kind: Kind) -> [f32; 3] {
        let ix = self.markup.colors[slot(kind)].min(4);
        to_color(palette(kind)[ix].1)
    }

    /// Undoes the last mark made or removed, if the marks have any.
    pub(super) fn undo_mark(&mut self, cx: &mut Context<Self>) -> bool {
        self.finish_typing(true, cx);
        let done = self.markup.marks.undo();
        if done {
            cx.notify();
        }
        done
    }

    pub(super) fn redo_mark(&mut self, cx: &mut Context<Self>) -> bool {
        self.finish_typing(true, cx);
        let done = self.markup.marks.redo();
        if done {
            cx.notify();
        }
        done
    }

    /// With a text tool, turns what was just selected into marks, one
    /// per page.
    pub(super) fn mark_selection(&mut self, cx: &mut Context<Self>) {
        let Some(Tool::Mark(kind)) = self.marking() else {
            return;
        };
        if !kind.on_text() {
            return;
        }
        let Some((start, end)) = self.text.span() else {
            return;
        };
        let Some(pdf) = self.pdf() else {
            return;
        };
        let color = self.color_of(kind);
        let mut made = Vec::new();
        for page in start.0..=end.0 {
            let Some(lines) = pdf.text.get(page) else {
                continue;
            };
            let mut quads = Vec::new();
            let mut text = String::new();
            for (ix, line) in lines.iter().enumerate() {
                let key = (page, ix);
                if key < (start.0, start.1) || key > (end.0, end.1) {
                    continue;
                }
                let from = if key == (start.0, start.1) {
                    start.2
                } else {
                    0
                };
                let to = if key == (end.0, end.1) {
                    end.2
                } else {
                    line.text.len()
                };
                let mut chars = line.chars.iter().filter(|c| c.0 >= from && c.0 < to);
                let Some(first) = chars.next() else {
                    continue;
                };
                let last = chars.next_back().unwrap_or(first);
                quads.push(Quad {
                    left: first.1,
                    top: line.top,
                    right: last.2,
                    bottom: line.bottom,
                });
                if let Some(part) = line.text.get(from..to.min(line.text.len())) {
                    if !text.is_empty() {
                        text.push('\n');
                    }
                    text.push_str(part.trim());
                }
            }
            if !quads.is_empty() {
                made.push(Mark {
                    page,
                    kind,
                    color,
                    shape: Shape::Text { quads, text },
                });
            }
        }
        self.text.clear();
        for mark in made {
            self.markup.marks.add(mark);
        }
        cx.notify();
    }

    /// `at` (in the window) as a point on `page`, in points from its top
    /// left, kept on the page.
    fn page_point(&self, page: usize, at: Point<Pixels>) -> Option<(f32, f32)> {
        let pdf = self.pdf()?;
        let bounds = *self.markup.spots.borrow().get(&page)?;
        let (w, h) = pdf.doc.page_size(page);
        let x = unpx(at.x - bounds.origin.x) / pdf.z;
        let y = unpx(at.y - bounds.origin.y) / pdf.z;
        Some((x.clamp(0.0, w), y.clamp(0.0, h)))
    }

    /// The page under `at`, from the last frame.
    fn page_at(&self, at: Point<Pixels>) -> Option<usize> {
        self.markup
            .spots
            .borrow()
            .iter()
            .find(|(_, bounds)| bounds.contains(&at))
            .map(|(page, _)| *page)
    }

    /// A press on `page` with the pen, the note or text tool or the
    /// eraser: whether it was taken. A press while typing only finishes
    /// the typing.
    pub(super) fn press_page(
        &mut self,
        page: usize,
        at: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.markup.typing.is_some() {
            self.finish_typing(true, cx);
            return true;
        }
        let Some(tool) = self.marking() else {
            return false;
        };
        let Some(point) = self.page_point(page, at) else {
            return false;
        };
        match tool {
            Tool::Mark(kind) if kind.typed() => {
                let z = self.pdf().map_or(1.0, |pdf| pdf.z);
                let (w, h) = self.pdf().map_or((0.0, 0.0), |pdf| pdf.doc.page_size(page));
                let hit = self
                    .markup
                    .marks
                    .at(page, point.0, point.1, 2.0 / z)
                    .filter(|&ix| self.markup.marks.list()[ix].kind == kind);
                match hit {
                    Some(ix) => self.open_typed(ix, window, cx),
                    None if kind == Kind::Note => {
                        let half = NOTE_SIZE / 2.0;
                        let at = (
                            (point.0 - half).clamp(0.0, (w - NOTE_SIZE).max(0.0)),
                            (point.1 - half).clamp(0.0, (h - NOTE_SIZE).max(0.0)),
                        );
                        let color = self.color_of(kind);
                        self.start_typing(page, kind, at, 0.0, color, None, "", window, cx);
                    }
                    None => {
                        let width = TEXT_WIDTH.min(w - point.0 - 4.0).max(40.0);
                        let at = (
                            point.0.min((w - width).max(0.0)),
                            (point.1 - TEXT_SIZE * LINE_HEIGHT / 2.0).max(0.0),
                        );
                        let color = self.color_of(kind);
                        self.start_typing(page, kind, at, width, color, None, "", window, cx);
                    }
                }
            }
            Tool::Mark(Kind::Ink) => self.markup.stroke = Some((page, vec![point])),
            Tool::Eraser => {
                self.markup.erasing = true;
                self.erase_at(page, point);
            }
            _ => return false,
        }
        self.text.clear();
        cx.notify();
        true
    }

    /// Opens note or text box `ix` for typing.
    fn open_typed(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(mark) = self.markup.marks.list().get(ix).cloned() else {
            return;
        };
        let (at, width, text) = match &mark.shape {
            Shape::Note { at, text } => (*at, 0.0, text.clone()),
            Shape::Box {
                at, width, text, ..
            } => (*at, *width, text.clone()),
            _ => return,
        };
        self.start_typing(
            mark.page,
            mark.kind,
            at,
            width,
            mark.color,
            Some(ix),
            &text,
            window,
            cx,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn start_typing(
        &mut self,
        page: usize,
        kind: Kind,
        at: (f32, f32),
        width: f32,
        color: [f32; 3],
        editing: Option<usize>,
        text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let placeholder = if kind == Kind::Note {
            tr!("viewer-note-placeholder")
        } else {
            tr!("viewer-text-placeholder")
        };
        let text = text.to_owned();
        let accent = self.th.accent;
        let area = cx.new(|cx| {
            let mut area = TextArea::new(placeholder, cx);
            area.set_accent(rgba(accent).into());
            let end = text.len();
            area.set_text(text, end, cx);
            area
        });
        let _events = cx.subscribe_in(&area, window, |this, _, event, _, cx| match event {
            InputEvent::Submit => this.finish_typing(true, cx),
            InputEvent::Cancel => this.finish_typing(false, cx),
            InputEvent::Changed => cx.notify(),
        });
        window.focus(&area.read(cx).focus_handle(cx), cx);
        self.text.clear();
        self.markup.typing = Some(Typing {
            page,
            kind,
            at,
            width,
            color,
            editing,
            area,
            _events,
        });
        cx.notify();
    }

    /// Closes the note or text box being typed, keeping what was typed
    /// when `keep`: an empty one goes.
    pub(super) fn finish_typing(&mut self, keep: bool, cx: &mut Context<Self>) {
        let Some(typing) = self.markup.typing.take() else {
            return;
        };
        self.markup.refocus = true;
        cx.notify();
        if !keep {
            return;
        }
        let text = typing.area.read(cx).text().trim_end().to_owned();
        let shape = match typing.kind {
            Kind::Note => Shape::Note {
                at: typing.at,
                text: text.clone(),
            },
            _ => Shape::Box {
                at: typing.at,
                width: typing.width,
                size: TEXT_SIZE,
                text: text.clone(),
            },
        };
        let mark = Mark {
            page: typing.page,
            kind: typing.kind,
            color: typing.color,
            shape,
        };
        let marks = &mut self.markup.marks;
        match (typing.editing, text.trim().is_empty()) {
            (Some(ix), true) => marks.remove(ix),
            (Some(ix), false) => marks.replace(ix, mark),
            (None, false) => marks.add(mark),
            (None, true) => {}
        }
    }

    /// Removes the note or text box being typed.
    fn delete_typing(&mut self, cx: &mut Context<Self>) {
        if let Some(ix) = self.markup.typing.take().and_then(|t| t.editing) {
            self.markup.marks.remove(ix);
        }
        self.markup.refocus = true;
        cx.notify();
    }

    /// The notes and text boxes on `page`, over its picture, and the one
    /// being typed. A click on one opens it, unless erasing.
    pub(super) fn page_typed(&self, page: usize, z: f32, cx: &Context<Self>) -> Vec<AnyElement> {
        let editing = self.markup.typing.as_ref().and_then(|t| t.editing);
        let can_open = self.markup.allowed == Some(Ok(())) && self.markup.tool != Tool::Eraser;
        let mut out = Vec::new();
        for (ix, mark) in self.markup.marks.list().iter().enumerate() {
            if mark.page != page || !mark.kind.typed() || Some(ix) == editing {
                continue;
            }
            let color = paint_color(mark.color, 1.0);
            let open = cx.listener(move |this, _: &MouseDownEvent, window, cx| {
                this.finish_typing(true, cx);
                this.open_typed(ix, window, cx);
                cx.stop_propagation();
            });
            let element = match &mark.shape {
                Shape::Note { at, text } => note_icon(*at, z, color)
                    .id(("viewer-note", ix))
                    .tooltip(tip(text.clone(), &self.th)),
                Shape::Box {
                    at,
                    width,
                    size,
                    text,
                } => div()
                    .id(("viewer-text-box", ix))
                    .absolute()
                    .left(px(at.0 * z))
                    .top(px(at.1 * z))
                    .w(px(width * z))
                    .flex()
                    .flex_col()
                    .text_size(px(size * z))
                    .line_height(px(size * LINE_HEIGHT * z))
                    .text_color(color)
                    .children(
                        wrap(text, *width, *size)
                            .into_iter()
                            .map(|line| div().whitespace_nowrap().child(line)),
                    ),
                _ => continue,
            };
            out.push(
                element
                    .when(can_open, |d| {
                        d.cursor_pointer().on_mouse_down(MouseButton::Left, open)
                    })
                    .into_any_element(),
            );
        }
        if let Some(typing) = self.markup.typing.as_ref().filter(|t| t.page == page) {
            let color = paint_color(typing.color, 1.0);
            let stop =
                |_: &MouseDownEvent, _: &mut Window, cx: &mut gpui::App| cx.stop_propagation();
            if typing.kind == Kind::Note {
                out.push(note_icon(typing.at, z, color).into_any_element());
                let button = |id: &'static str, label: String| {
                    div()
                        .id(id)
                        .px(px(10.0))
                        .py(px(4.0))
                        .rounded_full()
                        .cursor_pointer()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::MEDIUM)
                        .hover(|s| s.bg(rgba(0x0000001a)))
                        .child(label)
                };
                out.push(
                    div()
                        .id("viewer-note-card")
                        .absolute()
                        .left(px((typing.at.0 + NOTE_SIZE) * z + 6.0))
                        .top(px(typing.at.1 * z))
                        .w(px(260.0))
                        .p(px(12.0))
                        .flex()
                        .flex_col()
                        .gap(px(8.0))
                        .rounded(px(10.0))
                        .bg(rgba(0xfffbe6ff))
                        .border_1()
                        .border_color(rgba(0x0000001f))
                        .shadow_lg()
                        .cursor_text()
                        .text_size(px(14.0))
                        .line_height(px(20.0))
                        .text_color(rgba(0x202124ff))
                        .on_mouse_down(MouseButton::Left, stop)
                        .child(div().min_h(px(60.0)).child(typing.area.clone()))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .justify_end()
                                .gap(px(4.0))
                                .text_color(rgba(0x3c4043ff))
                                .child(
                                    button("viewer-note-delete", tr!("viewer-note-delete"))
                                        .on_click(
                                            cx.listener(|this, _, _, cx| this.delete_typing(cx)),
                                        ),
                                )
                                .child(
                                    button("viewer-note-done", tr!("viewer-note-done"))
                                        // The card is light in both themes.
                                        .text_color(rgba(0x1a73e8ff))
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.finish_typing(true, cx)
                                        })),
                                ),
                        )
                        .into_any_element(),
                );
            } else {
                out.push(
                    div()
                        .id("viewer-text-typing")
                        .absolute()
                        .left(px(typing.at.0 * z - 3.0))
                        .top(px(typing.at.1 * z - 3.0))
                        .w(px(typing.width * z + 6.0))
                        .p(px(2.0))
                        .rounded(px(3.0))
                        .border_1()
                        .border_color(rgba(self.th.accent))
                        .bg(rgba(0xffffffcc))
                        .cursor_text()
                        .text_size(px(TEXT_SIZE * z))
                        .line_height(px(TEXT_SIZE * LINE_HEIGHT * z))
                        .text_color(color)
                        .on_mouse_down(MouseButton::Left, stop)
                        .child(typing.area.clone())
                        .into_any_element(),
                );
            }
        }
        out
    }

    fn erase_at(&mut self, page: usize, (x, y): (f32, f32)) {
        let z = self.pdf().map_or(1.0, |pdf| pdf.z);
        if let Some(ix) = self.markup.marks.at(page, x, y, 4.0 / z) {
            self.markup.marks.remove(ix);
        }
    }

    fn marking_moved(&mut self, at: Point<Pixels>, cx: &mut Context<Self>) {
        if let Some(page) = self.markup.stroke.as_ref().map(|(page, _)| *page) {
            let z = self.pdf().map_or(1.0, |pdf| pdf.z);
            if let Some(point) = self.page_point(page, at)
                && let Some((_, points)) = &mut self.markup.stroke
            {
                let far = points
                    .last()
                    .is_none_or(|(x, y)| (x - point.0).hypot(y - point.1) * z >= 1.5);
                if far {
                    points.push(point);
                    cx.notify();
                }
            }
        } else if self.markup.erasing
            && let Some(page) = self.page_at(at)
            && let Some(point) = self.page_point(page, at)
        {
            let before = self.markup.marks.list().len();
            self.erase_at(page, point);
            if self.markup.marks.list().len() != before {
                cx.notify();
            }
        }
    }

    fn marking_ended(&mut self, cx: &mut Context<Self>) {
        self.markup.erasing = false;
        if let Some((page, points)) = self.markup.stroke.take() {
            let color = self.color_of(Kind::Ink);
            self.markup.marks.add(Mark {
                page,
                kind: Kind::Ink,
                color,
                shape: Shape::Ink(points),
            });
        }
        cx.notify();
    }

    /// Follows the pointer while the pen draws or the eraser rubs,
    /// wherever it goes.
    pub(super) fn follow_marking(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
                        let busy = {
                            let m = &this.read(cx).markup;
                            m.stroke.is_some() || m.erasing
                        };
                        if phase == DispatchPhase::Bubble && busy {
                            this.update(cx, |this, cx| {
                                if event.pressed_button == Some(MouseButton::Left) {
                                    this.marking_moved(event.position, cx);
                                } else {
                                    this.marking_ended(cx);
                                }
                            });
                        }
                    }
                });
                window.on_mouse_event(move |event: &MouseUpEvent, phase, _, cx| {
                    let Some(this) = this.upgrade() else {
                        return;
                    };
                    let busy = {
                        let m = &this.read(cx).markup;
                        m.stroke.is_some() || m.erasing
                    };
                    if phase == DispatchPhase::Bubble && event.button == MouseButton::Left && busy {
                        this.update(cx, |this, cx| this.marking_ended(cx));
                    }
                });
            },
        )
        .absolute()
        .size_0()
    }

    /// The marks on `page`, drawn over its picture at `z` logical pixels
    /// per point; also records where the page is.
    pub(super) fn page_marks(&self, page: usize, z: f32) -> AnyElement {
        let marks: Vec<Mark> = self
            .markup
            .marks
            .list()
            .iter()
            .filter(|m| m.page == page)
            .cloned()
            .collect();
        let stroke = self
            .markup
            .stroke
            .as_ref()
            .filter(|(p, _)| *p == page)
            .map(|(_, points)| (points.clone(), self.color_of(Kind::Ink)));
        let spots = self.markup.spots.clone();
        canvas(
            move |bounds, _, _| {
                spots.borrow_mut().insert(page, bounds);
            },
            move |bounds, (), window, _| {
                let origin = bounds.origin;
                let at =
                    |(x, y): (f32, f32)| gpui::point(origin.x + px(x * z), origin.y + px(y * z));
                for mark in &marks {
                    paint_mark(window, mark, z, &at);
                }
                if let Some((points, color)) = &stroke {
                    paint_line(window, points, PEN_WIDTH * z, paint_color(*color, 1.0), &at);
                }
            },
        )
        .absolute()
        .top_0()
        .left_0()
        .size_full()
        .into_any_element()
    }

    /// Leaves the PDF (closing the viewer, or showing another
    /// attachment), asking first when there are unsaved marks.
    pub(super) fn leave(&mut self, to: Leave, cx: &mut Context<Self>) {
        self.finish_typing(true, cx);
        if self.pdf().is_some() && self.markup.marks.unsaved() {
            self.markup.ask = Some(to);
            cx.notify();
            return;
        }
        self.go(to, cx);
    }

    fn go(&mut self, to: Leave, cx: &mut Context<Self>) {
        match to {
            Leave::Close => cx.emit(ViewerEvent::Close),
            Leave::Show(ix) => self.show(ix, cx),
            Leave::Out(by) => cx.emit(ViewerEvent::Step(by)),
        }
    }

    /// An answer to the question about unsaved marks: save a copy (then
    /// leave), leave without them, or stay.
    pub(super) fn answer(&mut self, save: Option<bool>, cx: &mut Context<Self>) {
        let Some(to) = self.markup.ask.take() else {
            return;
        };
        match save {
            Some(true) => self.save_marked(Some(to), cx),
            Some(false) => self.go(to, cx),
            None => cx.notify(),
        }
    }

    /// Whether Save saves a marked copy rather than the attachment.
    pub(super) fn saves_marks(&self) -> bool {
        self.pdf().is_some() && !self.markup.marks.is_empty()
    }

    /// Saves a copy of the PDF with the marks (through the window's save
    /// dialog), then does `then`.
    pub(super) fn save_marked(&mut self, then: Option<Leave>, cx: &mut Context<Self>) {
        self.write_marked(Marked::Save, then, cx);
    }

    /// Starts a reply to the message with a copy of the PDF with the
    /// marks attached.
    pub(super) fn reply_marked(&mut self, cx: &mut Context<Self>) {
        self.write_marked(Marked::Reply, None, cx);
    }

    /// Starts a new mail with a copy of the PDF with the marks attached.
    pub(super) fn forward_marked(&mut self, cx: &mut Context<Self>) {
        self.write_marked(Marked::Forward, None, cx);
    }

    fn write_marked(&mut self, to: Marked, then: Option<Leave>, cx: &mut Context<Self>) {
        self.finish_typing(true, cx);
        let (Some(pdf), Some(file)) = (self.pdf(), self.file.clone()) else {
            return;
        };
        let doc = pdf.doc.clone();
        let marks = self.markup.marks.list().to_vec();
        self.markup._saving = Some(cx.spawn(async move |this, cx| {
            let saved = cx
                .background_executor()
                .spawn(async move { doc.with_marks(&marks) })
                .await;
            this.update(cx, |this, cx| {
                match saved {
                    Ok(bytes) => {
                        this.markup.marks.saved();
                        this.markup.failed = false;
                        let marked = Arc::new(AttachmentFile {
                            name: marked_name(&file.name),
                            mime: file.mime.clone(),
                            bytes,
                        });
                        cx.emit(match to {
                            Marked::Save => ViewerEvent::Save(marked),
                            Marked::Reply => ViewerEvent::Reply(marked),
                            Marked::Forward => ViewerEvent::Forward(marked),
                        });
                        if let Some(to) = then {
                            this.go(to, cx);
                        }
                    }
                    Err(_) => {
                        this.markup.failed = true;
                        this.markup.on = true;
                    }
                }
                cx.notify();
            })
            .ok();
        }));
    }

    /// The top bar's Mark up button, lit while the tools are out.
    pub(super) fn markup_button(&self, th: &Theme, cx: &mut Context<Self>) -> Stateful<gpui::Div> {
        let on = self.markup.on;
        pill_button(
            "viewer-markup",
            "pen",
            tr!("viewer-markup-tip"),
            on,
            true,
            40.0,
            th,
        )
        .on_click(cx.listener(|this, _, _, cx| this.toggle_markup(cx)))
    }

    /// The pill of tools under the top bar, while marking up.
    pub(super) fn markup_pill(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let m = &self.markup;
        if !m.on || self.pdf().is_none() {
            return None;
        }
        let pill = div()
            .id("viewer-markup-pill")
            .occlude()
            .min_h(px(44.0))
            .max_w(px(760.0))
            .px(px(6.0))
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .justify_center()
            .gap(px(2.0))
            .relative()
            .rounded(px(22.0))
            .map(|el| glassy(el, PILL, PILL_FROSTED, 22.0, self.th.frost))
            .text_size(px(13.0))
            .text_color(rgba(INK));
        let separator = || {
            div()
                .mx(px(4.0))
                .w(px(1.0))
                .h(px(20.0))
                .bg(rgba(0xffffff33))
        };
        let body =
            match (&m.allowed, m.failed) {
                (Some(Err(_)), _) => pill.child(
                    div()
                        .px(px(14.0))
                        .py(px(10.0))
                        .child(tr!("viewer-markup-protected")),
                ),
                (_, true) => pill.child(
                    div()
                        .px(px(14.0))
                        .py(px(10.0))
                        .child(tr!("viewer-marks-save-failed")),
                ),
                _ => {
                    let ready = m.allowed == Some(Ok(()));
                    let mut pill = pill;
                    for (tool, id, name, label) in TOOLS {
                        pill = pill.child(
                            pill_button(id, name, tr!(label), m.tool == tool, ready, 36.0, th)
                                .when(ready, |d| {
                                    d.on_click(
                                        cx.listener(move |this, _, _, cx| this.set_tool(tool, cx)),
                                    )
                                }),
                        );
                    }
                    {
                        pill = pill.child(separator());
                        let (kind, live) = match m.tool {
                            Tool::Mark(kind) => (kind, ready),
                            _ => (m.last, false),
                        };
                        let chosen = m.colors[slot(kind)];
                        for (ix, (label, rgb)) in palette(kind).iter().enumerate() {
                            pill = pill.child(
                                div()
                                    .id(DOTS[ix])
                                    .size(px(30.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded_full()
                                    .when(!live, |d| d.opacity(0.35))
                                    .when(live, |d| {
                                        d.cursor_pointer()
                                            .hover(|s| s.bg(rgba(HOVER)))
                                            .tooltip(tip(tr!(*label), th))
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.set_color(ix, cx)
                                            }))
                                    })
                                    .child(
                                        div()
                                            .size(px(18.0))
                                            .rounded_full()
                                            .bg(rgba(rgb << 8 | 0xff))
                                            .when(ix == chosen, |d| {
                                                d.border_px(2.0).border_color(rgba(INK))
                                            }),
                                    ),
                            );
                        }
                    }
                    pill.child(separator())
                        .child(
                            pill_button(
                                "viewer-mark-undo",
                                "undo",
                                tr!("viewer-marks-undo-tip"),
                                false,
                                m.marks.can_undo(),
                                36.0,
                                th,
                            )
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.undo_mark(cx);
                            })),
                        )
                        .child(
                            pill_button(
                                "viewer-mark-redo",
                                "redo",
                                tr!("viewer-marks-redo-tip"),
                                false,
                                m.marks.can_redo(),
                                36.0,
                                th,
                            )
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.redo_mark(cx);
                            })),
                        )
                }
            };
        Some(
            div()
                .absolute()
                .top(px(BAR_HEIGHT + 8.0))
                .left_0()
                .right_0()
                .px(px(16.0))
                .flex()
                .justify_center()
                .child(body)
                .into_any_element(),
        )
    }

    /// The question about unsaved marks, over the viewer.
    pub(super) fn leave_dialog(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        self.markup.ask?;
        let button = |id: &'static str, label: SharedString, primary: bool| {
            div()
                .id(id)
                .relative()
                .overflow_hidden()
                .h(px(36.0))
                .px(px(16.0))
                .flex()
                .items_center()
                .rounded_full()
                .cursor_pointer()
                .text_size(px(14.0))
                .font_weight(FontWeight::MEDIUM)
                .map(|d| {
                    if primary {
                        d.bg(rgba(th.accent)).text_color(rgba(th.on_accent))
                    } else {
                        d.text_color(rgba(INK)).hover(|s| s.bg(rgba(HOVER)))
                    }
                })
                .child(Ripple::new(id, rgba(0xffffff33)))
                .child(label)
        };
        Some(
            div()
                .id("viewer-marks-ask")
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgba(0x00000073))
                .occlude()
                .child(
                    div()
                        .w(px(420.0))
                        .max_w(gpui::relative(0.9))
                        .p(px(24.0))
                        .rounded(px(15.0))
                        .bg(rgba(0x2d2f31ff))
                        .shadow_lg()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .child(
                            div()
                                .text_size(px(18.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgba(INK))
                                .child(tr!("viewer-marks-unsaved-title")),
                        )
                        .child(
                            div()
                                .text_size(px(14.0))
                                .text_color(rgba(INK_DIM))
                                .child(tr!("viewer-marks-unsaved-text")),
                        )
                        .child(
                            div()
                                .pt(px(8.0))
                                .flex()
                                .flex_row()
                                .flex_wrap()
                                .justify_end()
                                .gap(px(8.0))
                                .child(
                                    button(
                                        "viewer-marks-discard",
                                        tr!("viewer-marks-discard").into(),
                                        false,
                                    )
                                    .on_click(
                                        cx.listener(|this, _, _, cx| this.answer(Some(false), cx)),
                                    ),
                                )
                                .child(
                                    button(
                                        "viewer-marks-keep",
                                        tr!("viewer-marks-keep").into(),
                                        false,
                                    )
                                    .on_click(cx.listener(|this, _, _, cx| this.answer(None, cx))),
                                )
                                .child(
                                    button(
                                        "viewer-marks-save",
                                        tr!("viewer-marks-save").into(),
                                        true,
                                    )
                                    .on_click(
                                        cx.listener(|this, _, _, cx| this.answer(Some(true), cx)),
                                    ),
                                ),
                        ),
                )
                .into_any_element(),
        )
    }
}

/// A round icon button on the viewer's dark bars, lit when `active`,
/// greyed when not `enabled`.
fn pill_button(
    id: &'static str,
    name: &str,
    label: impl Into<SharedString>,
    active: bool,
    enabled: bool,
    size: f32,
    th: &Theme,
) -> Stateful<gpui::Div> {
    div()
        .id(id)
        .relative()
        .overflow_hidden()
        .size(px(size))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .when(active, |d| d.bg(rgba(th.accent & 0xffffff00 | 0x80)))
        .when(enabled, |d| {
            d.cursor_pointer()
                .hover(|s| s.bg(rgba(HOVER)))
                .child(Ripple::new(id, rgba(0xffffff33)).centered())
        })
        .child(icon(
            name,
            if enabled { INK } else { 0xffffff61 },
            size * 0.55,
        ))
        .tooltip(tip(label.into(), th))
}

/// The name of the marked copy of `name`: "Report (marked).pdf".
fn marked_name(name: &str) -> String {
    let stem = name
        .len()
        .checked_sub(4)
        .filter(|&at| name.is_char_boundary(at) && name[at..].eq_ignore_ascii_case(".pdf"))
        .map_or(name, |at| &name[..at]);
    format!("{}.pdf", tr!("viewer-marked-name", name = stem.to_owned()))
}

/// A sticky note's icon at `at` (points), `z` pixels per point.
fn note_icon(at: (f32, f32), z: f32, color: Rgba) -> gpui::Div {
    let size = NOTE_SIZE * z;
    div()
        .absolute()
        .left(px(at.0 * z))
        .top(px(at.1 * z))
        .size(px(size))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(3.0 * z))
        .bg(color)
        .border_1()
        .border_color(rgba(0x00000059))
        .shadow_sm()
        .child(icon("notes", 0x3c4043ff, size * 0.7))
}

fn paint_mark(window: &mut Window, mark: &Mark, z: f32, at: &dyn Fn((f32, f32)) -> Point<Pixels>) {
    let color = paint_color(mark.color, 1.0);
    match &mark.shape {
        // Drawn as elements, with their text (see `page_typed`).
        Shape::Note { .. } | Shape::Box { .. } => {}
        Shape::Ink(points) => paint_line(window, points, PEN_WIDTH * z, color, at),
        Shape::Text { quads, .. } => {
            for q in quads {
                let h = (q.bottom - q.top).max(1.0);
                let width = (h * 0.07).max(0.6);
                match mark.kind {
                    Kind::Highlight | Kind::Ink | Kind::Note | Kind::FreeText => {
                        window.paint_quad(gpui::fill(
                            Bounds::from_corners(at((q.left, q.top)), at((q.right, q.bottom))),
                            paint_color(mark.color, HIGHLIGHT_ALPHA),
                        ))
                    }
                    Kind::Underline => {
                        let y = q.bottom - h * 0.1;
                        paint_line(window, &[(q.left, y), (q.right, y)], width * z, color, at);
                    }
                    Kind::StrikeOut => {
                        let y = q.top + h * 0.55;
                        paint_line(window, &[(q.left, y), (q.right, y)], width * z, color, at);
                    }
                    Kind::Squiggly => paint_line(window, &q.squiggle(), width * z, color, at),
                }
            }
        }
    }
}

fn paint_line(
    window: &mut Window,
    points: &[(f32, f32)],
    width: f32,
    color: Rgba,
    at: &dyn Fn((f32, f32)) -> Point<Pixels>,
) {
    match points {
        [] => {}
        [only] => {
            // A dot.
            let center = at(*only);
            let r = px(width / 2.0);
            window.paint_quad(
                gpui::fill(
                    Bounds::from_corners(
                        gpui::point(center.x - r, center.y - r),
                        gpui::point(center.x + r, center.y + r),
                    ),
                    color,
                )
                .corner_radii(r),
            );
        }
        [first, rest @ ..] => {
            let mut path = PathBuilder::stroke(px(width));
            path.move_to(at(*first));
            for point in rest {
                path.line_to(at(*point));
            }
            if let Ok(path) = path.build() {
                window.paint_path(path, color);
            }
        }
    }
}

/// The pages take the pointer for the pen and the eraser.
pub(super) fn drawing_cursor() -> CursorStyle {
    CursorStyle::Crosshair
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marked_copies_keep_the_name() {
        assert_eq!(marked_name("Q3 report.pdf"), "Q3 report (marked).pdf");
        assert_eq!(marked_name("SCAN.PDF"), "SCAN (marked).pdf");
        assert_eq!(marked_name("notes"), "notes (marked).pdf");
    }
}
