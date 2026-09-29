// SPDX-License-Identifier: GPL-3.0-or-later

//! Marking up a PDF in the attachment viewer. "Mark up" in the top bar
//! puts a pill of tools under it: select text, highlight, underline,
//! squiggle, strike through, pen and eraser, five colours, and undo and
//! redo. The text tools mark what a drag (or a double or triple click)
//! selects; the pen draws; the eraser removes the marks it touches.
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
    AnyElement, Bounds, Context, CursorStyle, DispatchPhase, FontWeight, MouseButton,
    MouseMoveEvent, MouseUpEvent, PathBuilder, Pixels, Point, Rgba, SharedString, Stateful, Task,
    Window, canvas, div, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_preview::markup::{Kind, Mark, Marks, PEN_WIDTH, Quad, Shape};
use katna_preview::pdf::SaveError;
use katna_render::AttachmentFile;
use katna_ui::{Ripple, px, unpx};

use super::{BAR_HEIGHT, Content, HOVER, INK, INK_DIM, PILL, PdfView, Viewer, ViewerEvent};
use crate::theme::Theme;
use crate::widgets::{icon, tip};

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
const TOOLS: [(Tool, &str, &str, &str); 7] = [
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
        Tool::Eraser,
        "viewer-tool-eraser",
        "eraser",
        "viewer-tool-eraser",
    ),
];

/// What to do once unsaved marks are dealt with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Leave {
    Close,
    Show(usize),
}

pub(super) struct Markup {
    /// The pill of tools is out.
    on: bool,
    tool: Tool,
    /// The last kind of mark chosen: its colours stay in the pill (dimmed)
    /// while selecting or erasing, so the pill keeps its size.
    last: Kind,
    /// Each kind of mark's colour: a place in its palette.
    colors: [usize; 5],
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
}

impl Markup {
    pub(super) fn new() -> Self {
        Self {
            on: false,
            tool: Tool::Select,
            last: Kind::Highlight,
            colors: [0; 5],
            marks: Marks::default(),
            stroke: None,
            erasing: false,
            allowed: None,
            failed: false,
            _check: None,
            _saving: None,
            ask: None,
            spots: Rc::default(),
        }
    }

    /// The question about unsaved marks is on screen.
    pub(super) fn asking(&self) -> bool {
        self.ask.is_some()
    }

    /// The pen or the eraser is in use: the pages take the pointer.
    pub(super) fn drawing(&self) -> bool {
        self.on
            && self.allowed == Some(Ok(()))
            && matches!(self.tool, Tool::Mark(Kind::Ink) | Tool::Eraser)
    }
}

fn slot(kind: Kind) -> usize {
    match kind {
        Kind::Highlight => 0,
        Kind::Underline => 1,
        Kind::Squiggly => 2,
        Kind::StrikeOut => 3,
        Kind::Ink => 4,
    }
}

fn palette(kind: Kind) -> &'static [(&'static str, u32); 5] {
    if kind == Kind::Highlight {
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
        let done = self.markup.marks.undo();
        if done {
            cx.notify();
        }
        done
    }

    pub(super) fn redo_mark(&mut self, cx: &mut Context<Self>) -> bool {
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

    /// A press on `page` with the pen or the eraser: whether it was taken.
    pub(super) fn press_page(
        &mut self,
        page: usize,
        at: Point<Pixels>,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(tool) = self.marking() else {
            return false;
        };
        let Some(point) = self.page_point(page, at) else {
            return false;
        };
        match tool {
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
                        cx.emit(ViewerEvent::Save(Arc::new(AttachmentFile {
                            name: marked_name(&file.name),
                            mime: file.mime.clone(),
                            bytes,
                        })));
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
            .rounded(px(22.0))
            .bg(rgba(PILL))
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
                                                d.border_2().border_color(rgba(INK))
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

fn paint_mark(window: &mut Window, mark: &Mark, z: f32, at: &dyn Fn((f32, f32)) -> Point<Pixels>) {
    let color = paint_color(mark.color, 1.0);
    match &mark.shape {
        Shape::Ink(points) => paint_line(window, points, PEN_WIDTH * z, color, at),
        Shape::Text { quads, .. } => {
            for q in quads {
                let h = (q.bottom - q.top).max(1.0);
                let width = (h * 0.07).max(0.6);
                match mark.kind {
                    Kind::Highlight | Kind::Ink => window.paint_quad(gpui::fill(
                        Bounds::from_corners(at((q.left, q.top)), at((q.right, q.bottom))),
                        paint_color(mark.color, HIGHLIGHT_ALPHA),
                    )),
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
