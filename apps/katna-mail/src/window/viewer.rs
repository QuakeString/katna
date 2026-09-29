// SPDX-License-Identifier: GPL-3.0-or-later

//! The attachment viewer: a received PDF, picture, text file, spreadsheet
//! or document shown over the mail, like webmail's preview. A dark bar on
//! top names the file and offers Save and "Open with" (the desktop's list
//! of apps); arrows at the sides go through
//! the message's other attachments; a pill at the foot zooms (and shows
//! the PDF page, in a box that takes a page number to go to, Ctrl+G).
//! Escape closes it.
//!
//! Decoding happens off the UI thread (`katna_preview`). PDF pages are
//! drawn only while on screen (and one either side), at the zoom and the
//! screen's scale, and freed when scrolled far away. Spreadsheets and
//! documents are drawn by `office.rs`.
//!
//! Text can be selected and copied as in a message (`select.rs`): the
//! lines of a text file, the paragraphs of a document or slides, and the
//! text a PDF draws (read in the background, page by page). A
//! spreadsheet selects cells instead, and copies them tab-separated.

mod markup;
mod office;

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Entity, EventEmitter, FocusHandle, Focusable,
    FontWeight, ImageSource, KeyDownEvent, MouseButton, MouseDownEvent, ObjectFit, RenderImage,
    ScrollHandle, SharedString, Subscription, Task, Window, div, ease_out_quint, img, prelude::*,
    rgba, uniform_list,
};
use katna_i18n::tr;
use katna_preview::pdf::{self, Document, TextLine};
use katna_preview::{Kind, Picture, document, picture, sheet, slides, text};
use katna_render::AttachmentFile;
use katna_ui::px;
use katna_ui::unpx;
use katna_ui::{InputEvent, Ripple, TextInput};

use self::markup::{Leave, Markup};
use self::office::{DocumentView, SheetView};
use super::attachments::{Item, bitmap, kind_badge};
use super::select::{self, Key, Marker, SelectHost, TextSelection};
use crate::format;
use crate::theme::Theme;
use crate::widgets::{icon, tip};

/// Zoom steps; 1 fits the page (or picture) to the window.
const ZOOMS: [f32; 12] = [
    0.25, 0.5, 0.67, 0.8, 0.9, 1.0, 1.25, 1.5, 2.0, 2.5, 3.0, 4.0,
];
/// A PDF page is at most this wide at zoom 1, in logical pixels.
const PAGE_WIDTH: f32 = 880.0;
/// How long the last file stays on show while the next one opens.
const SLOW_LOAD: Duration = Duration::from_millis(300);
/// Counts viewers opened, for [`Viewer::opened`].
static OPENED: AtomicUsize = AtomicUsize::new(0);
const PAGE_GAP: f32 = 16.0;
/// Pages this far from the screen keep their bitmaps.
const KEEP_PAGES: usize = 2;
const BAR_HEIGHT: f32 = 64.0;
const LINE_SCROLL: f32 = 48.0;

// The viewer is dark in light and dark themes alike, like a photo viewer.
// The window shows faintly through its backdrop, or blurred when menus
// are frosted (Settings > Experimental).
const SCRIM: u32 = 0x0c0d0ecc;
const SCRIM_FROSTED: u32 = 0x0c0d0e99;
const BAR: u32 = 0x161718f0;
const INK: u32 = 0xffffffff;
const INK_DIM: u32 = 0xffffffb3;
const HOVER: u32 = 0xffffff1f;
const PILL: u32 = 0x2d2f31f2;

/// The viewer's key context.
pub(super) const KEY_CONTEXT: &str = "AttachmentViewer";

/// What the viewer asks the window to do.
pub(super) enum ViewerEvent {
    Close,
    Save(Arc<AttachmentFile>),
    OpenWith(Arc<AttachmentFile>),
    /// The file the viewer was opened on has no preview after all (a
    /// damaged, protected or unsupported file): open it elsewhere.
    Unreadable(Arc<AttachmentFile>),
}

pub(super) struct Viewer {
    focus: FocusHandle,
    /// The raw message the attachments come from.
    raw: Arc<Vec<u8>>,
    items: Vec<Item>,
    /// The item on show.
    current: usize,
    /// The item on show or opening, which the arrows page on from.
    target: usize,
    file: Option<Arc<AttachmentFile>>,
    content: Content,
    /// A step of [`ZOOMS`].
    zoom: usize,
    scroll: ScrollHandle,
    /// Bitmaps no longer drawn; the window frees them at the next frame.
    released: Vec<Arc<RenderImage>>,
    _load: Option<Task<()>>,
    /// Shows "Opening…" when the next file takes long.
    _wait: Option<Task<()>>,
    /// The page being drawn in the background.
    drawing: Option<(usize, Task<()>)>,
    /// Counts the files shown, so each starts with a fresh selection.
    seq: usize,
    /// Tells this viewer's fade-in from an earlier one's; paging between
    /// files keeps it, so the viewer fades in only when it opens.
    opened: usize,
    /// The viewer's width and the screen's scale at the last frame, to
    /// draw the next PDF's first page at the size it will be shown.
    frame: (f32, f32),
    /// Selected text of a text file, document or PDF.
    text: TextSelection,
    /// The PDF page number in the foot pill, typed to go to a page.
    goto: Entity<TextInput>,
    _goto: Subscription,
    /// The page box was clicked while not focused: its number is selected
    /// when the button comes up, so typing replaces it.
    goto_click: bool,
    /// The page last gone to and the scroll offset that shows it: the page
    /// counts as on show until the PDF scrolls.
    went: Option<(usize, f32)>,
    /// Marks made on a PDF, and the tools for them.
    markup: Markup,
    pub(super) th: Theme,
}

impl SelectHost for Viewer {
    fn selection(&self) -> &TextSelection {
        &self.text
    }

    fn selection_mut(&mut self) -> &mut TextSelection {
        &mut self.text
    }

    /// The viewer keeps the focus, so its keys still work.
    fn text_focus(&self) -> FocusHandle {
        self.focus.clone()
    }

    fn selected(&mut self, cx: &mut Context<Self>) {
        self.mark_selection(cx);
    }
}

enum Content {
    Loading,
    Pdf(PdfView),
    /// A decoded picture and its size in pixels.
    Bitmap(Arc<RenderImage>, (u32, u32)),
    /// Drawn by GPUI itself (animated GIF, SVG), with its size if known.
    Drawn(Arc<gpui::Image>, Option<(u32, u32)>),
    Text(Rc<Vec<SharedString>>, bool),
    Sheet(SheetView),
    Document(DocumentView),
    /// No preview; the text says why.
    Nothing(SharedString),
}

struct PdfView {
    doc: Arc<Document>,
    /// Drawn pages: the scale they were drawn at, in pixels per point.
    pages: HashMap<usize, (f32, Arc<RenderImage>)>,
    /// Each page's text, as far as it has been read.
    text: Vec<Rc<Vec<TextLine>>>,
    /// Logical pixels per point at the last frame, to find a page's place.
    z: f32,
    _reading: Option<Task<()>>,
}

/// Pages whose text is read, for selecting; enough for any mail.
const TEXT_PAGES: usize = 2000;

/// What loading found, made on a background thread.
enum Loaded {
    /// The PDF and its first page, drawn at the scale it will be shown.
    Pdf(Document, Option<(f32, Arc<RenderImage>)>),
    Bitmap(Arc<RenderImage>, (u32, u32)),
    Drawn(Arc<gpui::Image>, Option<(u32, u32)>),
    Text(Vec<SharedString>, bool),
    Sheet(sheet::Workbook),
    Document(document::Document),
    /// Why there is nothing to show: a message id, translated on the
    /// main thread.
    Nothing(&'static str),
}

impl EventEmitter<ViewerEvent> for Viewer {}

impl Viewer {
    pub(super) fn new(
        raw: Arc<Vec<u8>>,
        items: Vec<Item>,
        current: usize,
        th: Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus = cx.focus_handle();
        focus.focus(window, cx);
        let goto = cx.new(|cx| {
            let mut input = TextInput::new("", cx);
            input.set_accent(rgba(th.accent).into());
            input
        });
        let _goto = cx.subscribe_in(&goto, window, |this, _, event, window, cx| match event {
            InputEvent::Submit => {
                let typed = this.goto.read(cx).text().trim().parse::<usize>().ok();
                if let Some(page) = typed {
                    this.go_to_page(page.saturating_sub(1), cx);
                }
                this.focus.focus(window, cx);
            }
            InputEvent::Cancel => this.focus.focus(window, cx),
            InputEvent::Changed => {}
        });
        let mut this = Self {
            focus,
            raw,
            items,
            current: 0,
            target: 0,
            file: None,
            content: Content::Loading,
            zoom: fit_step(),
            scroll: ScrollHandle::new(),
            released: Vec::new(),
            _load: None,
            _wait: None,
            drawing: None,
            seq: 0,
            opened: OPENED.fetch_add(1, Ordering::Relaxed),
            frame: (unpx(window.viewport_size().width), window.scale_factor()),
            text: TextSelection::new(cx),
            goto,
            _goto,
            goto_click: false,
            went: None,
            markup: Markup::new(),
            th,
        };
        this.show(current, cx);
        this
    }

    /// Bitmaps to free, all of them when `all` (the viewer is closing).
    pub(super) fn take_released(&mut self, all: bool) -> Vec<Arc<RenderImage>> {
        if all {
            let content = std::mem::replace(&mut self.content, Content::Loading);
            self.release(content);
        }
        std::mem::take(&mut self.released)
    }

    fn release(&mut self, content: Content) {
        match content {
            Content::Pdf(pdf) => self
                .released
                .extend(pdf.pages.into_values().map(|(_, image)| image)),
            Content::Bitmap(image, _) => self.released.push(image),
            _ => {}
        }
    }

    /// Shows item `ix` (wrapping around). The file on show stays until
    /// the next one is ready (a PDF with its first page drawn), so paging
    /// swaps one for the other with nothing in between; a file that takes
    /// long shows "Opening…" meanwhile.
    fn show(&mut self, ix: usize, cx: &mut Context<Self>) {
        if self.items.is_empty() {
            return;
        }
        let ix = ix % self.items.len();
        self.target = ix;
        let item = self.items[ix].clone();
        let raw = self.raw.clone();
        // Only the file the viewer was opened on is handed to another app
        // when it cannot be shown; paging through never launches one.
        let first = self.seq == 0;
        if first {
            self.enter(ix);
        }
        let risky = item.risky;
        let (vw, scale_factor) = self.frame;
        let waiting = self.seq;
        let slow = cx.background_executor().timer(SLOW_LOAD);
        self._wait = Some(cx.spawn(async move |this, cx| {
            slow.await;
            this.update(cx, |this, cx| {
                if this.seq == waiting && this.current != ix {
                    this.enter(ix);
                    cx.notify();
                }
            })
            .ok();
        }));
        self._load = Some(cx.spawn(async move |this, cx| {
            let (file, loaded) = cx
                .background_executor()
                .spawn(async move {
                    let (file, loaded) = load(&raw, &item);
                    let loaded = match loaded {
                        Loaded::Pdf(doc, _) if vw > 0.0 => {
                            let scale = pdf_fit(&doc, vw) * ZOOMS[fit_step()] * scale_factor;
                            let page = doc.render(0, scale).map(|p| (scale, bitmap(p)));
                            Loaded::Pdf(doc, page)
                        }
                        loaded => loaded,
                    };
                    (file, loaded)
                })
                .await;
            this.update(cx, |this, cx| {
                this._wait = None;
                this.enter(ix);
                this.file = file.map(Arc::new);
                if first
                    && matches!(loaded, Loaded::Nothing(_))
                    && !risky
                    && let Some(file) = &this.file
                {
                    cx.emit(ViewerEvent::Unreadable(file.clone()));
                }
                this.content = match loaded {
                    Loaded::Pdf(doc, page) => {
                        let doc = Arc::new(doc);
                        Content::Pdf(PdfView {
                            _reading: Some(this.read_pdf_text(doc.clone(), cx)),
                            doc,
                            pages: page.into_iter().map(|page| (0, page)).collect(),
                            text: Vec::new(),
                            z: 1.0,
                        })
                    }
                    Loaded::Bitmap(image, size) => Content::Bitmap(image, size),
                    Loaded::Drawn(image, size) => Content::Drawn(image, size),
                    Loaded::Text(lines, cut) => {
                        let all = lines
                            .iter()
                            .enumerate()
                            .map(|(ix, line)| (Key::new(0, ix), line.clone()))
                            .collect();
                        this.text.set_all(Some(Rc::new(all)));
                        Content::Text(Rc::new(lines), cut)
                    }
                    Loaded::Sheet(book) => Content::Sheet(SheetView::new(book)),
                    Loaded::Document(doc) => {
                        let view = DocumentView::new(doc);
                        this.text.set_all(Some(view.all_text()));
                        this.text.set_part_gap("\n");
                        Content::Document(view)
                    }
                    Loaded::Nothing(why) => Content::Nothing(katna_i18n::tr!(why).into()),
                };
                cx.notify();
            })
            .ok();
        }));
        cx.notify();
    }

    /// Puts item `ix` on show, still opening: the last file's view, marks
    /// and selection go.
    fn enter(&mut self, ix: usize) {
        self.current = ix;
        self.seq += 1;
        self.text.begin(self.seq);
        self.zoom = fit_step();
        self.went = None;
        self.markup = Markup::new();
        self.scroll.set_offset(gpui::point(px(0.0), px(0.0)));
        self.file = None;
        self.drawing = None;
        let old = std::mem::replace(&mut self.content, Content::Loading);
        self.release(old);
    }

    /// Closes the viewer, asking first about unsaved marks.
    fn close(&mut self, cx: &mut Context<Self>) {
        self.leave(Leave::Close, cx);
    }

    /// Shows the attachment `ix` places on (wrapping around), asking
    /// first about unsaved marks.
    fn step(&mut self, by: isize, cx: &mut Context<Self>) {
        let count = self.items.len() as isize;
        if count > 0 {
            let ix = (self.target as isize + by).rem_euclid(count) as usize;
            self.leave(Leave::Show(ix), cx);
        }
    }

    /// Reads the text of `doc`'s pages in the background, a few pages at
    /// a time, so it can be selected.
    fn read_pdf_text(&mut self, doc: Arc<Document>, cx: &mut Context<Self>) -> Task<()> {
        self.text.set_part_gap("\n\n");
        cx.spawn(async move |this, cx| {
            let count = doc.pages().min(TEXT_PAGES);
            let mut next = 0;
            while next < count {
                let (from, to) = (next, (next + 8).min(count));
                let doc = doc.clone();
                let pages: Vec<Vec<TextLine>> = cx
                    .background_executor()
                    .spawn(async move { (from..to).map(|p| doc.text(p)).collect() })
                    .await;
                let done = to == count;
                let more = this.update(cx, |this, cx| {
                    let Content::Pdf(pdf) = &mut this.content else {
                        return false;
                    };
                    pdf.text.extend(pages.into_iter().map(Rc::new));
                    // Until every page is read, copying takes what is on
                    // screen.
                    if done {
                        let all = pdf
                            .text
                            .iter()
                            .enumerate()
                            .flat_map(|(page, lines)| {
                                lines.iter().enumerate().map(move |(ix, line)| {
                                    (Key::new(page, ix), SharedString::from(line.text.clone()))
                                })
                            })
                            .collect();
                        this.text.set_all(Some(Rc::new(all)));
                    }
                    cx.notify();
                    true
                });
                if !matches!(more, Ok(true)) {
                    return;
                }
                next = to;
            }
        })
    }

    /// Copies the selected text, or the selected cells of a spreadsheet.
    fn copy(&mut self, cx: &mut Context<Self>) {
        if matches!(self.content, Content::Sheet(_)) {
            self.copy_cells(cx);
        } else {
            select::copy(self, cx);
        }
    }

    /// Selects all of the text, or every cell of the sheet on show.
    fn select_all(&mut self, cx: &mut Context<Self>) {
        if matches!(self.content, Content::Sheet(_)) {
            self.select_all_cells(cx);
        } else {
            select::select_all(self, cx);
        }
    }

    /// Saves the attachment, or a copy of a PDF with the marks made on it.
    fn save(&mut self, cx: &mut Context<Self>) {
        if self.saves_marks() {
            self.save_marked(None, cx);
        } else if let Some(file) = &self.file {
            cx.emit(ViewerEvent::Save(file.clone()));
        }
    }

    fn open_with(&mut self, cx: &mut Context<Self>) {
        if let Some(file) = &self.file {
            cx.emit(ViewerEvent::OpenWith(file.clone()));
        }
    }

    fn set_zoom(&mut self, step: usize, cx: &mut Context<Self>) {
        let step = step.min(ZOOMS.len() - 1);
        if step != self.zoom {
            // Keep the same part of a PDF on screen.
            let ratio = ZOOMS[step] / ZOOMS[self.zoom];
            let offset = self.scroll.offset();
            self.zoom = step;
            self.scroll
                .set_offset(gpui::point(offset.x * ratio, offset.y * ratio));
            cx.notify();
        }
    }

    /// Scrolls the PDF so `page` (from 0) starts just below the top bar.
    fn go_to_page(&mut self, page: usize, cx: &mut Context<Self>) {
        let Content::Pdf(pdf) = &self.content else {
            return;
        };
        let count = pdf.doc.pages();
        if count == 0 {
            return;
        }
        let page = page.min(count - 1);
        let top: f32 = (0..page)
            .map(|p| pdf.doc.page_size(p).1 * pdf.z + PAGE_GAP)
            .sum();
        let offset = self.scroll.offset();
        let max = self.scroll.max_offset();
        let y = (-top).clamp(-unpx(max.y), 0.0);
        self.scroll.set_offset(gpui::point(offset.x, px(y)));
        self.went = Some((page, y));
        cx.notify();
    }

    fn scroll_by(&mut self, dy: f32, cx: &mut Context<Self>) {
        match &self.content {
            Content::Document(view) => {
                view.state.scroll_by(px(dy));
                cx.notify();
                return;
            }
            Content::Sheet(view) => {
                let handle = view.scroll.0.borrow().base_handle.clone();
                let offset = handle.offset();
                let max = handle.max_offset();
                let y = (unpx(offset.y) - dy).clamp(-unpx(max.y), 0.0);
                handle.set_offset(gpui::point(offset.x, px(y)));
                cx.notify();
                return;
            }
            _ => {}
        }
        let offset = self.scroll.offset();
        let max = self.scroll.max_offset();
        let y = (unpx(offset.y) - dy).clamp(-unpx(max.y), 0.0);
        self.scroll.set_offset(gpui::point(offset.x, px(y)));
        cx.notify();
    }

    fn on_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        // Typing a page number: the keys are the box's.
        if self.goto.focus_handle(cx).is_focused(window) {
            return;
        }
        let keystroke = &event.keystroke;
        let ctrl = keystroke.modifiers.control;
        let shift = keystroke.modifiers.shift;
        // The question about unsaved marks: Enter saves, Escape stays.
        if self.markup.asking() {
            match keystroke.key.as_str() {
                "enter" => self.answer(Some(true), cx),
                "escape" => self.answer(None, cx),
                _ => {}
            }
            cx.stop_propagation();
            return;
        }
        if matches!(self.content, Content::Pdf(_)) && ctrl {
            let done = match keystroke.key.as_str() {
                "z" if shift => self.redo_mark(cx),
                "y" => self.redo_mark(cx),
                "z" => self.undo_mark(cx),
                _ => false,
            };
            if done {
                cx.stop_propagation();
                return;
            }
        }
        let view = match &self.content {
            Content::Sheet(sheet) => sheet.scroll.0.borrow().base_handle.bounds(),
            Content::Document(doc) => doc.state.viewport_bounds(),
            _ => self.scroll.bounds(),
        };
        let page = (unpx(view.size.height) - LINE_SCROLL).max(LINE_SCROLL);
        match keystroke.key.as_str() {
            "escape" => self.close(cx),
            "left" => self.step(-1, cx),
            "right" => self.step(1, cx),
            "+" | "=" => self.set_zoom(self.zoom + 1, cx),
            "-" => self.set_zoom(self.zoom.saturating_sub(1), cx),
            "0" => self.set_zoom(fit_step(), cx),
            "s" if ctrl => self.save(cx),
            "c" if ctrl => self.copy(cx),
            "a" if ctrl => self.select_all(cx),
            "g" if ctrl && matches!(self.content, Content::Pdf(_)) => {
                window.focus(&self.goto.focus_handle(cx), cx);
                self.goto.update(cx, |input, cx| input.select_all_text(cx));
            }
            "up" => self.scroll_by(-LINE_SCROLL, cx),
            "down" => self.scroll_by(LINE_SCROLL, cx),
            "pageup" => self.scroll_by(-page, cx),
            "pagedown" | "space" => self.scroll_by(page, cx),
            "home" => self.scroll_by(f32::MIN / 4.0, cx),
            "end" => self.scroll_by(f32::MAX / 4.0, cx),
            _ => return,
        }
        cx.stop_propagation();
    }

    /// The PDF page at the middle of the screen, from the last layout.
    fn current_page(&self, count: usize) -> usize {
        let view = self.scroll.bounds();
        let middle = view.origin.y + view.size.height / 2.0;
        let top = self.scroll.top_item();
        (top..count)
            .find(|&ix| {
                self.scroll
                    .bounds_for_item(ix)
                    .is_some_and(|b| b.origin.y + b.size.height + px(PAGE_GAP) > middle)
            })
            .unwrap_or(top)
            .min(count.saturating_sub(1))
    }

    /// Draws the next page that is on screen at the wrong scale (or not
    /// at all), and frees pages far from the screen.
    fn draw_pages(&mut self, scale: f32, cx: &mut Context<Self>) {
        let Content::Pdf(pdf) = &mut self.content else {
            return;
        };
        let count = pdf.doc.pages();
        let top = self.scroll.top_item().min(count - 1);
        let bottom = self.scroll.bottom_item().clamp(top, count - 1);
        let keep = top.saturating_sub(KEEP_PAGES)..=(bottom + KEEP_PAGES).min(count - 1);
        let far: Vec<usize> = pdf
            .pages
            .keys()
            .copied()
            .filter(|p| !keep.contains(p))
            .collect();
        for page in far {
            if let Some((_, image)) = pdf.pages.remove(&page) {
                self.released.push(image);
            }
        }
        if self.drawing.is_some() {
            return;
        }
        // On screen first, then the page after, then the one before.
        let wanted = (top..=bottom)
            .chain([(bottom + 1).min(count - 1), top.saturating_sub(1)])
            .find(|p| {
                pdf.pages
                    .get(p)
                    .is_none_or(|(drawn, _)| (drawn / scale - 1.0).abs() > 0.01)
            });
        let Some(page) = wanted else {
            return;
        };
        let doc = pdf.doc.clone();
        let task = cx.spawn(async move |this, cx| {
            let image = cx
                .background_executor()
                .spawn(async move { doc.render(page, scale).map(bitmap) })
                .await;
            this.update(cx, |this, cx| {
                this.drawing = None;
                if let (Content::Pdf(pdf), Some(image)) = (&mut this.content, image)
                    && let Some((_, old)) = pdf.pages.insert(page, (scale, image))
                {
                    this.released.push(old);
                }
                cx.notify();
            })
            .ok();
        });
        self.drawing = Some((page, task));
    }
}

/// Points to logical pixels that fit `doc`'s widest page in a viewer
/// `vw` wide, at 100%.
fn pdf_fit(doc: &Document, vw: f32) -> f32 {
    let widest = (0..doc.pages())
        .map(|p| doc.page_size(p).0)
        .fold(1.0_f32, f32::max);
    ((vw - 176.0).min(PAGE_WIDTH) / widest).clamp(0.2, 4.0)
}

/// Extracts `item` from the raw message and decodes it for showing.
fn load(raw: &[u8], item: &Item) -> (Option<AttachmentFile>, Loaded) {
    let Some(file) = katna_render::attachment_file(raw, item.index) else {
        return (None, Loaded::Nothing("viewer-unreadable"));
    };
    let loaded = match katna_preview::kind(&file.mime, &file.name) {
        Kind::Pdf => match Document::open(file.bytes.clone()) {
            Ok(doc) => Loaded::Pdf(doc, None),
            Err(pdf::Error::Locked) => Loaded::Nothing("viewer-pdf-locked"),
            Err(pdf::Error::Invalid) => Loaded::Nothing("viewer-pdf-unreadable"),
        },
        // SVG too: drawn to a bitmap by katna_preview, never by GPUI, which
        // would load the files it links to.
        Kind::Picture(format) => {
            match picture::decode(&file.bytes, format, picture::VIEW_SIDE) {
                // GPUI plays animated GIFs itself.
                Ok(first) if format == Picture::Gif => Loaded::Drawn(
                    Arc::new(gpui::Image::from_bytes(
                        gpui::ImageFormat::Gif,
                        file.bytes.clone(),
                    )),
                    Some(first.dimensions()),
                ),
                Ok(image) => {
                    let size = image.dimensions();
                    Loaded::Bitmap(bitmap(image), size)
                }
                Err(_) => Loaded::Nothing("viewer-picture-unreadable"),
            }
        }
        Kind::Text => {
            let (lines, cut) = text::lines(&file.bytes);
            Loaded::Text(lines.into_iter().map(SharedString::from).collect(), cut)
        }
        Kind::Sheet { csv: true } => {
            let tabs = file.name.to_ascii_lowercase().ends_with(".tsv")
                || file.mime.to_ascii_lowercase().contains("tab-separated");
            Loaded::Sheet(sheet::csv(&file.bytes, &file.name, tabs))
        }
        Kind::Sheet { csv: false } => match sheet::open(file.bytes.clone()) {
            Ok(book) => Loaded::Sheet(book),
            Err(_) => Loaded::Nothing("viewer-sheet-unreadable"),
        },
        Kind::Document => match document::open(file.bytes.clone()) {
            Ok(doc) => Loaded::Document(doc),
            Err(_) => Loaded::Nothing("viewer-document-unreadable"),
        },
        Kind::Slides => match slides::open(file.bytes.clone()) {
            Ok(doc) => Loaded::Document(doc),
            Err(_) => Loaded::Nothing("viewer-slides-unreadable"),
        },
        Kind::Other => Loaded::Nothing("viewer-no-preview"),
    };
    (Some(file), loaded)
}

fn fit_step() -> usize {
    ZOOMS.iter().position(|z| *z == 1.0).unwrap_or(0)
}

/// A round button on the dark bar.
fn bar_button(id: &'static str, name: &str, th: &Theme) -> gpui::Stateful<gpui::Div> {
    bar_button_tip(id, name, tooltip_for(id).into(), th)
}

/// A round button on the dark bar, with its own tooltip.
fn bar_button_tip(
    id: &'static str,
    name: &str,
    tooltip: SharedString,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .relative()
        .overflow_hidden()
        .size(px(40.0))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .cursor_pointer()
        .hover(|s| s.bg(rgba(HOVER)))
        .child(Ripple::new(id, rgba(0xffffff33)).centered())
        .child(icon(name, INK, 22.0))
        .tooltip(tip(tooltip, th))
}

fn tooltip_for(id: &str) -> &'static str {
    match id {
        "viewer-close" => "Close (Esc)",
        "viewer-save" => "Save (Ctrl+S)",
        "viewer-open" => "Open with another app",
        "viewer-prev" => "Previous attachment",
        "viewer-next" => "Next attachment",
        "viewer-zoom-in" => "Zoom in (+)",
        "viewer-zoom-out" => "Zoom out (-)",
        _ => "",
    }
}

impl Render for Viewer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Text without a size of its own follows Settings > Appearance > Scaling.
        window.set_rem_size(px(16.0));
        // A new frame: text scrolled out of sight is no longer selectable
        // where it was.
        self.text.begin(self.seq);
        let th = self.th;
        let viewport = window.viewport_size();
        // The room the viewer had at the last frame: the window below the
        // app's top bar (the window itself before the first frame).
        let area = self.scroll.bounds().size;
        let (vw, vh) = if area.height > px(0.0) {
            (unpx(area.width), unpx(area.height))
        } else {
            (unpx(viewport.width), unpx(viewport.height) - BAR_HEIGHT)
        };
        self.frame = (vw, window.scale_factor());
        let zoom = ZOOMS[self.zoom];
        let item = self.items.get(self.current).cloned();
        let name = item.as_ref().map(|i| i.name.clone()).unwrap_or_default();
        let many = self.items.len() > 1;

        // The PDF's page at the middle of the screen and its page count.
        let mut pages = None;
        let mut pdf_z = None;
        let body: AnyElement = if matches!(self.content, Content::Document(_)) {
            self.document_body(zoom, vw, cx)
        } else {
            match &self.content {
                Content::Document(_) => div().into_any_element(),
                Content::Sheet(view) => self.sheet_body(view, zoom, vw, cx),
                Content::Loading => centered(
                    div()
                        .text_color(rgba(INK_DIM))
                        .text_size(px(14.0))
                        .child("Opening…"),
                ),
                Content::Nothing(why) => {
                    let why = why.clone();
                    let kind = item.as_ref().map(|i| i.kind).unwrap_or(Kind::Other);
                    centered(
                        div()
                            .w(px(360.0))
                            .p(px(28.0))
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(16.0))
                            .rounded(px(15.0))
                            .bg(rgba(PILL))
                            .child(kind_badge(kind, 48.0))
                            .child(div().text_size(px(15.0)).text_color(rgba(INK)).child(why))
                            .when(self.file.is_some(), |d| {
                                d.child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .gap(px(8.0))
                                        .child(
                                            text_button("viewer-save-big", "download", "Save")
                                                .on_click(
                                                    cx.listener(|this, _, _, cx| this.save(cx)),
                                                ),
                                        )
                                        .child(
                                            text_button(
                                                "viewer-open-big",
                                                "open-external",
                                                "Open with…",
                                            )
                                            .on_click(
                                                cx.listener(|this, _, _, cx| this.open_with(cx)),
                                            ),
                                        ),
                                )
                            }),
                    )
                }
                Content::Bitmap(image, (w, h)) => {
                    let (w, h) = picture_size(*w, *h, window.scale_factor(), vw, vh, zoom);
                    self.picture(img(ImageSource::Render(image.clone())), w, h)
                }
                Content::Drawn(image, size) => {
                    let (w, h) = match size {
                        Some((w, h)) => picture_size(*w, *h, window.scale_factor(), vw, vh, zoom),
                        None => {
                            let side = ((vw - 160.0).min(vh - 200.0)).max(120.0) * zoom;
                            (side, side)
                        }
                    };
                    self.picture(img(ImageSource::Image(image.clone())), w, h)
                }
                Content::Text(lines, cut) => {
                    let lines = lines.clone();
                    let cut = *cut;
                    let count = lines.len() + usize::from(cut);
                    let size = 13.0 * zoom;
                    let width = (vw - 160.0).clamp(300.0, 960.0);
                    let marker = self.text.marker(&th);
                    let page = select::selectable(
                        div()
                            .w(px(width))
                            .h_full()
                            .rounded(px(8.0))
                            .bg(rgba(0xffffffff))
                            .text_color(rgba(0x202124ff))
                            .font_family("monospace")
                            .text_size(px(size)),
                        None,
                        cx,
                    );
                    div()
                        .size_full()
                        .flex()
                        .justify_center()
                        .pt(px(BAR_HEIGHT + 8.0))
                        .pb(px(80.0))
                        .child(
                            page.child(
                                uniform_list("viewer-text", count, move |range, _, _| {
                                    range
                                        .map(|ix| {
                                            let row = div()
                                                .px(px(20.0))
                                                .h(px(size * 1.5))
                                                .whitespace_nowrap();
                                            match lines.get(ix) {
                                                Some(line) => {
                                                    let (styled, holder) = marker.piece(
                                                        Key::new(0, ix),
                                                        line.clone(),
                                                        Vec::new(),
                                                    );
                                                    row.child(holder.child(styled))
                                                }
                                                None => row
                                                    .child("… (the rest of the file is not shown)"),
                                            }
                                        })
                                        .collect()
                                })
                                .py(px(12.0))
                                .size_full(),
                            ),
                        )
                        .into_any_element()
                }
                Content::Pdf(pdf) => {
                    let count = pdf.doc.pages();
                    let widest = (0..count)
                        .map(|p| pdf.doc.page_size(p).0)
                        .fold(1.0_f32, f32::max);
                    let z = pdf_fit(&pdf.doc, vw) * zoom;
                    let scale = z * window.scale_factor();
                    let went = self
                        .went
                        .filter(|&(_, y)| (unpx(self.scroll.offset().y) - y).abs() < 0.5);
                    let current = match went {
                        Some((page, _)) => page,
                        None => self.current_page(count),
                    };
                    pages = Some((current.min(count.saturating_sub(1)), count));
                    pdf_z = Some(z);
                    let wide = widest * z > vw - 32.0;
                    let marker = self.text.marker(&th);
                    let drawing = self.markup.drawing();
                    let pages: Vec<AnyElement> = (0..count)
                        .map(|p| {
                            let (w, h) = pdf.doc.page_size(p);
                            let (w, h) = (w * z, h * z);
                            let text = pdf.text.get(p).filter(|lines| !lines.is_empty());
                            div()
                                .relative()
                                .flex_none()
                                .w(px(w))
                                .h(px(h))
                                .bg(rgba(0xffffffff))
                                .shadow(vec![gpui::BoxShadow {
                                    color: rgba(0x00000080).into(),
                                    offset: gpui::point(px(0.0), px(2.0)),
                                    blur_radius: px(8.0),
                                    spread_radius: px(0.0),
                                    inset: false,
                                }])
                                .when_some(pdf.pages.get(&p), |d, (_, image)| {
                                    d.child(
                                        img(ImageSource::Render(image.clone()))
                                            .w(px(w))
                                            .h(px(h))
                                            .object_fit(ObjectFit::Fill),
                                    )
                                })
                                .child(self.page_marks(p, z))
                                .when_some(text, |d, lines| {
                                    d.cursor_text().child(page_text(
                                        p,
                                        lines.clone(),
                                        z,
                                        marker.clone(),
                                    ))
                                })
                                .when(drawing, |d| {
                                    d.cursor(markup::drawing_cursor()).on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                                            if this.press_page(p, event.position, cx) {
                                                cx.stop_propagation();
                                            }
                                        }),
                                    )
                                })
                                .into_any_element()
                        })
                        .collect();
                    // Pages are drawn after this frame is laid out, once the
                    // scroll handle knows which are on screen.
                    let this = cx.entity().downgrade();
                    window.on_next_frame(move |_, cx| {
                        this.update(cx, |this, cx| this.draw_pages(scale, cx)).ok();
                    });
                    select::selectable(div(), None, cx)
                        .id("viewer-pages")
                        .size_full()
                        .overflow_scroll()
                        .track_scroll(&self.scroll)
                        .flex()
                        .flex_col()
                        .when(!wide, |d| d.items_center())
                        .gap(px(PAGE_GAP))
                        .pt(px(BAR_HEIGHT + 8.0))
                        .pb(px(96.0))
                        .px(px(16.0))
                        .children(pages)
                        .into_any_element()
                }
            }
        };

        if let (Some(z), Content::Pdf(pdf)) = (pdf_z, &mut self.content) {
            pdf.z = z;
        }
        let goto_focused = self.goto.focus_handle(cx).is_focused(window);
        if let Some((current, _)) = pages
            && !goto_focused
        {
            let number = (current + 1).to_string();
            if self.goto.read(cx).text() != number {
                self.goto.update(cx, |input, cx| input.set_text(number, cx));
            }
        }

        let top_bar = div()
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .h(px(BAR_HEIGHT))
            .px(px(12.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .occlude()
            .bg(rgba(BAR))
            .child(
                bar_button("viewer-close", "back", &th)
                    .on_click(cx.listener(|this, _, _, cx| this.close(cx))),
            )
            .child(kind_badge(
                item.as_ref().map(|i| i.kind).unwrap_or(Kind::Other),
                22.0,
            ))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .truncate()
                            .text_size(px(15.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(INK))
                            .child(name),
                    )
                    .when_some(item.as_ref(), |d, item| {
                        d.child(div().text_size(px(12.0)).text_color(rgba(INK_DIM)).child(
                            if many {
                                format!(
                                    "{} · {} of {}",
                                    format::size(item.size),
                                    self.current + 1,
                                    self.items.len()
                                )
                            } else {
                                format::size(item.size)
                            },
                        ))
                    }),
            )
            .when(
                self.file.is_some() && matches!(self.content, Content::Pdf(_)),
                |d| d.child(self.markup_button(&th, cx)),
            )
            .when(self.file.is_some(), |d| {
                let save = if self.saves_marks() {
                    bar_button_tip(
                        "viewer-save",
                        "download",
                        tr!("viewer-save-marked-tip").into(),
                        &th,
                    )
                } else {
                    bar_button("viewer-save", "download", &th)
                };
                d.child(
                    bar_button("viewer-open", "open-external", &th)
                        .on_click(cx.listener(|this, _, _, cx| this.open_with(cx))),
                )
                .child(save.on_click(cx.listener(|this, _, _, cx| this.save(cx))))
            });

        let zoomable = matches!(
            self.content,
            Content::Pdf(_)
                | Content::Bitmap(..)
                | Content::Drawn(..)
                | Content::Text(..)
                | Content::Sheet(_)
                | Content::Document(_)
        );
        let foot = zoomable.then(|| {
            div()
                .absolute()
                .bottom(px(24.0))
                .left_0()
                .right_0()
                .flex()
                .justify_center()
                .child(
                    div()
                        .id("viewer-foot")
                        .occlude()
                        .h(px(44.0))
                        .px(px(6.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(2.0))
                        .rounded_full()
                        .bg(rgba(PILL))
                        .text_size(px(13.0))
                        .text_color(rgba(INK))
                        .when_some(pages, |d, (_, count)| {
                            d.child(
                                div()
                                    .id("viewer-page")
                                    .pl(px(12.0))
                                    .pr(px(10.0))
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .tooltip(tip(tr!("viewer-go-to-page-tip"), &th))
                                    .child(tr!("viewer-page"))
                                    .child(
                                        div()
                                            .w(px(46.0))
                                            .h(px(28.0))
                                            .px(px(7.0))
                                            .flex()
                                            .items_center()
                                            .rounded(px(6.0))
                                            .border_1()
                                            .border_color(if goto_focused {
                                                rgba(th.accent)
                                            } else {
                                                rgba(0x00000000)
                                            })
                                            .bg(rgba(0xffffff1f))
                                            .capture_any_mouse_down(cx.listener(
                                                |this, _, window, cx| {
                                                    this.goto_click = !this
                                                        .goto
                                                        .focus_handle(cx)
                                                        .is_focused(window);
                                                },
                                            ))
                                            .on_mouse_up(
                                                MouseButton::Left,
                                                cx.listener(|this, _, _, cx| {
                                                    if std::mem::take(&mut this.goto_click) {
                                                        this.goto.update(cx, |input, cx| {
                                                            input.select_all_text(cx)
                                                        });
                                                    }
                                                }),
                                            )
                                            .child(self.goto.clone()),
                                    )
                                    .child(tr!("viewer-page-count", count = count)),
                            )
                            .child(div().w(px(1.0)).h(px(20.0)).bg(rgba(0xffffff33)))
                        })
                        .child(
                            bar_button("viewer-zoom-out", "zoom-out", &th)
                                .size(px(36.0))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.set_zoom(this.zoom.saturating_sub(1), cx)
                                })),
                        )
                        .child(
                            div()
                                .id("viewer-zoom-reset")
                                .w(px(52.0))
                                .flex()
                                .justify_center()
                                .cursor_pointer()
                                .tooltip(tip("Fit to window (0)", &th))
                                .on_click(
                                    cx.listener(|this, _, _, cx| this.set_zoom(fit_step(), cx)),
                                )
                                .child(format!("{:.0}%", zoom * 100.0)),
                        )
                        .child(
                            bar_button("viewer-zoom-in", "zoom-in", &th)
                                .size(px(36.0))
                                .on_click(
                                    cx.listener(|this, _, _, cx| this.set_zoom(this.zoom + 1, cx)),
                                ),
                        ),
                )
        });

        let side = |button: gpui::Stateful<gpui::Div>, left: bool| {
            div()
                .absolute()
                .top(px(BAR_HEIGHT))
                .bottom(px(BAR_HEIGHT))
                .when(left, |d| d.left(px(16.0)))
                .when(!left, |d| d.right(px(16.0)))
                .flex()
                .items_center()
                .child(button.size(px(48.0)).bg(rgba(PILL)).occlude())
        };
        let arrows = many.then(|| {
            [
                side(
                    bar_button("viewer-prev", "chevron-left", &th)
                        .on_click(cx.listener(|this, _, _, cx| this.step(-1, cx))),
                    true,
                ),
                side(
                    bar_button("viewer-next", "chevron-right", &th)
                        .on_click(cx.listener(|this, _, _, cx| this.step(1, cx))),
                    false,
                ),
            ]
        });

        div()
            .id(("viewer", self.opened))
            .track_focus(&self.focus)
            .key_context(KEY_CONTEXT)
            .on_key_down(cx.listener(Self::on_key))
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .occlude()
            .map(|el| {
                if self.th.frost == 0 {
                    el.bg(rgba(SCRIM))
                } else {
                    el.child(katna_ui::frost::glass(
                        rgba(SCRIM_FROSTED).into(),
                        px(0.0),
                        self.th.frost as f32,
                    ))
                }
            })
            .child(body)
            .child(select::follow_drags(cx))
            .child(self.follow_cell_drags(cx))
            .child(self.follow_marking(cx))
            .child(top_bar)
            .children(self.markup_pill(&th, cx))
            .children(arrows.into_iter().flatten())
            .children(foot)
            .children(select::text_menu(self, &th, cx))
            .children(self.cell_menu(&th, cx))
            .children(self.leave_dialog(&th, cx))
            .with_animation(
                ("viewer-in", self.opened),
                Animation::new(Duration::from_millis(160)).with_easing(ease_out_quint()),
                |el, t| el.opacity(t),
            )
    }
}

impl Viewer {
    /// A picture `w` × `h` logical pixels, centered, scrolling when zoomed
    /// past the window.
    fn picture(&self, image: gpui::Img, w: f32, h: f32) -> AnyElement {
        div()
            .id("viewer-picture")
            .size_full()
            .overflow_scroll()
            .track_scroll(&self.scroll)
            .child(
                div()
                    .min_w_full()
                    .min_h_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .pt(px(BAR_HEIGHT + 8.0))
                    .pb(px(88.0))
                    .px(px(80.0))
                    .child(
                        image
                            .flex_none()
                            .w(px(w))
                            .h(px(h))
                            .object_fit(ObjectFit::Contain),
                    ),
            )
            .into_any_element()
    }
}

/// A PDF page's text over its picture: invisible, but recorded where each
/// character is so it can be selected, with the selection drawn over the
/// page. `z` is logical pixels per point.
fn page_text(page: usize, lines: Rc<Vec<TextLine>>, z: f32, marker: Marker) -> AnyElement {
    let color = marker.color();
    gpui::canvas(
        move |bounds, _, _| {
            let origin = bounds.origin;
            let mut shade = Vec::new();
            for (ix, line) in lines.iter().enumerate() {
                let key = Key::new(page, ix);
                let top = origin.y + px(line.top * z);
                let height = px((line.bottom - line.top) * z);
                let left = origin.x + px(line.left() * z);
                let area = gpui::Bounds::new(
                    gpui::point(left, top),
                    gpui::size(px((line.right() - line.left()) * z), height),
                );
                let chars = line
                    .chars
                    .iter()
                    .map(|(offset, x, _)| (*offset, origin.x + px(x * z)))
                    .collect();
                marker.place(key, SharedString::from(line.text.clone()), area, chars);
                if let Some(range) = marker.range(key, line.text.len()) {
                    let from = line.chars.iter().find(|c| c.0 >= range.start);
                    let to = line.chars.iter().rev().find(|c| c.0 < range.end);
                    if let (Some(from), Some(to)) = (from, to) {
                        shade.push(gpui::Bounds::new(
                            gpui::point(origin.x + px(from.1 * z), top),
                            gpui::size(px((to.2 - from.1).max(0.0) * z), height),
                        ));
                    }
                }
            }
            shade
        },
        move |_, shade, window, _| {
            for area in shade {
                window.paint_quad(gpui::fill(area, color));
            }
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
    .into_any_element()
}

/// The logical size of a `w` × `h` pixel picture: at most its own size (at
/// `scale` pixels per logical pixel) and fitting the window, times `zoom`.
fn picture_size(w: u32, h: u32, scale: f32, vw: f32, vh: f32, zoom: f32) -> (f32, f32) {
    let (w, h) = (w as f32 / scale, h as f32 / scale);
    let room_w = (vw - 160.0).max(80.0);
    let room_h = (vh - BAR_HEIGHT - 104.0).max(80.0);
    let fit = (room_w / w).min(room_h / h).min(1.0);
    (w * fit * zoom, h * fit * zoom)
}

fn centered(child: impl IntoElement) -> AnyElement {
    div()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .child(child)
        .into_any_element()
}

/// A labelled button on the dark "no preview" card.
fn text_button(id: &'static str, name: &str, label: &'static str) -> gpui::Stateful<gpui::Div> {
    div()
        .id(id)
        .relative()
        .overflow_hidden()
        .h(px(36.0))
        .pl(px(12.0))
        .pr(px(16.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.0))
        .rounded_full()
        .border_1()
        .border_color(rgba(0xffffff4d))
        .text_size(px(14.0))
        .text_color(rgba(INK))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(HOVER)))
        .child(Ripple::new(id, rgba(0xffffff33)))
        .child(icon(name, INK, 20.0))
        .child(label)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pictures_fit_the_window_but_never_grow() {
        // A small picture keeps its size.
        assert_eq!(
            picture_size(200, 100, 1.0, 1600.0, 1000.0, 1.0),
            (200.0, 100.0)
        );
        // HiDPI: 2 pixels per logical pixel.
        assert_eq!(
            picture_size(200, 100, 2.0, 1600.0, 1000.0, 1.0),
            (100.0, 50.0)
        );
        // A large one shrinks to the room, keeping its shape.
        let (w, h) = picture_size(4000, 3000, 1.0, 1600.0, 1000.0, 1.0);
        assert!(w <= 1440.0 && h <= 1000.0 - BAR_HEIGHT - 104.0);
        assert!((w / h - 4.0 / 3.0).abs() < 0.01);
        // Zoom scales the fitted size.
        assert_eq!(
            picture_size(200, 100, 1.0, 1600.0, 1000.0, 2.0),
            (400.0, 200.0)
        );
    }
}
