// SPDX-License-Identifier: GPL-3.0-or-later

//! The attachment viewer: a received PDF, picture, text file, spreadsheet
//! or document shown over the mail, like webmail's preview. A dark bar on
//! top names the file and offers Save and "Open with" (the desktop's list
//! of apps); arrows at the sides go through
//! the message's other attachments; the middle of the bar (a pill at the
//! foot on a phone) zooms, turns a PDF's pages and shows the PDF page, in
//! a box that takes a page number to go to, Ctrl+G.
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

use crate::widgets::Tip as _;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, DispatchPhase, Entity, EventEmitter, FocusHandle,
    Focusable, FontWeight, ImageSource, KeyDownEvent, MouseButton, MouseDownEvent, MouseUpEvent,
    ObjectFit, PinchEvent, Pixels, Point, RenderImage, ScrollDelta, ScrollHandle, ScrollWheelEvent,
    SharedString, Subscription, Task, Window, div, ease_out_quint, img, prelude::*, rgba,
    uniform_list,
};
use katna_i18n::tr;
use katna_preview::image::{Frame, RgbaImage};
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
use crate::widgets::icon;

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
const BAR_HEIGHT: f32 = 54.0;
/// From this width the page box, zoom and turning sit in the top bar; the
/// first for a PDF, the second for zoom alone.
const BAR_CONTROLS_PDF: f32 = 900.0;
const BAR_CONTROLS: f32 = 640.0;
/// Below this width the foot pill drops the word "Page" to fit a phone.
const COMPACT_CONTROLS: f32 = 440.0;
/// A press and release further apart than this is a drag, not a click.
const CLICK_SLOP: f32 = 5.0;
/// Ctrl + wheel: one zoom step per notch, or per this many pixels of a
/// touchpad's smooth scroll.
const WHEEL_STEP: f32 = 50.0;
/// A pinch opening or closing this much (0.1 is 10 %) is one zoom step.
const PINCH_STEP: f32 = 0.12;
const LINE_SCROLL: f32 = 48.0;

// The viewer is dark in light and dark themes alike, like a photo viewer.
// When menus are frosted (Settings > Experimental) the window shows
// blurred under a dark veil, a little darker under the top bar, and the
// control pills blur the page under them. Without blur the window shows
// faintly through a darker veil.
const SCRIM: u32 = 0x0c0d0eb8;
const SCRIM_FROSTED: u32 = 0x0c0d0e73;
const BAR: u32 = 0x202124e6;
const BAR_FROSTED: u32 = 0x0c0d0e59;
const PILL_FROSTED: u32 = 0x2d2f31d9;
/// The page box, zoom and turning sit on a pill this tall in the top bar,
/// and a little taller at the foot.
const PILL_HEIGHT: f32 = 40.0;
const FOOT_PILL_HEIGHT: f32 = 44.0;
const INK: u32 = 0xffffffff;
const INK_DIM: u32 = 0xffffffb3;
const HOVER: u32 = 0xffffff1f;
const PILL: u32 = 0x2d2f31f2;
/// A side arrow under the pointer: the pill a shade lighter.
const PILL_HOVER: u32 = 0x47494cf2;

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
    /// Reply to the message with this file (a marked copy) attached.
    Reply(Arc<AttachmentFile>),
    /// Start a new mail with only this file (a marked copy, with marks)
    /// attached.
    Forward(Arc<AttachmentFile>),
    /// Show the mail the file came with (opened from the Files page).
    ShowMail,
    /// Show the file this many places on in the Files page's list
    /// (opened from there).
    Step(isize),
    /// Shift and an arrow showed another of this mail's attachments
    /// (this one, by its index): opened from the Files page.
    Paged(usize),
    /// Tick or untick the file shown (opened from the attach picker).
    Pick,
    /// The half-moon button turned dark pages on or off.
    DarkPages(bool),
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
    /// A zoom between the steps, from Fit or Real size; the steps go on
    /// from the nearest one.
    zoom_free: Option<f32>,
    /// The height of the room under the top bar, at the last frame.
    view_h: f32,
    /// The page (or slide) on show and the count, at the last frame.
    shown_page: Option<(usize, usize)>,
    /// Keeps stepping pages while a page box arrow is held.
    page_repeat: Option<Task<()>>,
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
    /// Up or Down changed the page box: the viewer goes there at once.
    goto_stepped: Arc<AtomicBool>,
    /// The page box was clicked while not focused: its number is selected
    /// when the button comes up, so typing replaces it.
    goto_click: bool,
    /// The page last gone to and the scroll offset that shows it: the page
    /// counts as on show until the PDF scrolls.
    went: Option<(usize, f32)>,
    /// Marks made on a PDF, and the tools for them.
    markup: Markup,
    /// Where the left button went down on the dim space around the file:
    /// letting go there (not a drag) closes the viewer.
    backdrop: Option<Point<Pixels>>,
    /// Ctrl + wheel and pinch movement not yet a whole zoom step.
    wheel_zoom: f32,
    pinch_zoom: f32,
    /// The viewer shows the open conversation's message, so a marked copy
    /// can go in a reply to it.
    can_reply: bool,
    /// Offers Show the mail: opened from the Files page, away from it.
    pub(super) can_show_mail: bool,
    /// Opened from the Files page: the file's place among the files it
    /// shows, and how many there are. The arrows then page through
    /// those; with Shift, through this mail's attachments.
    pub(super) library: Option<(usize, usize)>,
    /// Opened from the attach picker: whether the file shown is ticked,
    /// for the bar's Select button.
    pub(super) pick: Option<bool>,
    /// Opened before its file is here (a drive file still downloading):
    /// it waits, turning, for `arrived`.
    fetching: bool,
    /// Show bright pages dark (in a dark theme): Settings' `dark_pages`.
    pub(super) dark_pages: bool,
    /// The radius of the window content's bottom left and right corners
    /// (Katna's own rounded frame), which the viewer rounds itself to, as
    /// GPUI does not clip it there.
    pub(super) corners: (f32, f32),
    /// Where the controls' More menu was opened, while it is open.
    more_at: Option<Point<Pixels>>,
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
    fn text_focus(&self) -> Option<FocusHandle> {
        Some(self.focus.clone())
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
    /// Whether `pages` are drawn dark.
    dark: bool,
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
        can_reply: bool,
        th: Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus = cx.focus_handle();
        focus.focus(window, cx);
        let goto_stepped = Arc::new(AtomicBool::new(false));
        let goto = cx.new(|cx| {
            let mut input = TextInput::new("", cx);
            input.set_accent(rgba(th.accent).into());
            input.set_centered(true);
            // Up goes a page back, Down a page on; held, they keep going.
            let stepped = goto_stepped.clone();
            input.set_stepper(Some(Arc::new(move |text: &str, _, by| {
                let page = text.trim().parse::<usize>().unwrap_or(1);
                let page = if by > 0 {
                    page.saturating_sub(1).max(1)
                } else {
                    page + 1
                };
                stepped.store(true, Ordering::Relaxed);
                let text = page.to_string();
                let end = text.len();
                Some((text, end))
            })));
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
            InputEvent::Changed => {
                if !this.goto_stepped.swap(false, Ordering::Relaxed) {
                    return;
                }
                let Some((_, count)) = this.shown_page else {
                    return;
                };
                let count = count.max(1);
                let typed = this.goto.read(cx).text().trim().parse::<usize>();
                let page = typed.unwrap_or(1).clamp(1, count);
                // Past the last page the box stays on it.
                let number = page.to_string();
                if this.goto.read(cx).text() != number {
                    this.goto.update(cx, |input, cx| input.set_text(number, cx));
                }
                this.go_to_page(page - 1, cx);
            }
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
            zoom_free: None,
            view_h: 0.0,
            shown_page: None,
            page_repeat: None,
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
            goto_stepped,
            goto_click: false,
            went: None,
            backdrop: None,
            wheel_zoom: 0.0,
            pinch_zoom: 0.0,
            markup: Markup::new(),
            can_reply,
            can_show_mail: false,
            library: None,
            pick: None,
            fetching: false,
            dark_pages: false,
            corners: (0.0, 0.0),
            more_at: None,
            th,
        };
        this.current = current;
        this
    }

    /// Starts showing the file the viewer was made on. Called once its
    /// owner has set it up (`dark_pages`), so the first page is drawn as
    /// it will stay, never white for a frame.
    pub(super) fn start(&mut self, cx: &mut Context<Self>) {
        self.show(self.current, cx);
    }

    /// A viewer open on `item` while its file is still on the way: it
    /// waits, turning, until `arrived` hands it over.
    pub(super) fn fetching(
        item: Item,
        th: Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self::new(Arc::new(Vec::new()), Vec::new(), 0, false, th, window, cx);
        this.items = vec![item];
        this.enter(0);
        this.fetching = true;
        this
    }

    /// The file a `fetching` viewer waits for is here.
    pub(super) fn arrived(&mut self, raw: Arc<Vec<u8>>, items: Vec<Item>, cx: &mut Context<Self>) {
        self.raw = raw;
        self.items = items;
        self.seq = 0;
        self.fetching = false;
        self.show(0, cx);
    }

    /// Shows attachment `ix` of `raw`, another mail's or this one's,
    /// keeping the viewer open: paging through the Files page.
    pub(super) fn show_from(
        &mut self,
        raw: Arc<Vec<u8>>,
        items: Vec<Item>,
        ix: usize,
        cx: &mut Context<Self>,
    ) {
        if !Arc::ptr_eq(&raw, &self.raw) {
            self.raw = raw;
            self.items = items;
        }
        self.show(ix, cx);
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
        let dark = self.dark_pages_shown();
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
                            let page = if dark {
                                doc.render_dark(0, scale)
                            } else {
                                doc.render(0, scale)
                            };
                            let page = page.map(|p| (scale, bitmap(p)));
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
                            dark,
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
        self.zoom_free = None;
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

    /// Shows the file `by` places on: in the Files page's list when
    /// opened from there, else among this mail's attachments.
    fn step(&mut self, by: isize, cx: &mut Context<Self>) {
        if self.library.is_some() {
            self.leave(Leave::Out(by), cx);
        } else {
            self.step_mail(by, cx);
        }
    }

    /// Shows the attachment `ix` places on (wrapping around), asking
    /// first about unsaved marks.
    fn step_mail(&mut self, by: isize, cx: &mut Context<Self>) {
        let count = self.items.len() as isize;
        if count > 0 {
            let ix = (self.target as isize + by).rem_euclid(count) as usize;
            self.leave(Leave::Show(ix), cx);
            // (Once shown: not while asking about unsaved marks.)
            if self.library.is_some() && !self.markup.asking() {
                cx.emit(ViewerEvent::Paged(self.items[ix].index));
            }
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

    fn forward(&mut self, cx: &mut Context<Self>) {
        if self.saves_marks() {
            self.forward_marked(cx);
        } else if let Some(file) = &self.file {
            cx.emit(ViewerEvent::Forward(file.clone()));
        }
    }

    fn open_with(&mut self, cx: &mut Context<Self>) {
        if let Some(file) = &self.file {
            cx.emit(ViewerEvent::OpenWith(file.clone()));
        }
    }

    fn set_zoom(&mut self, step: usize, cx: &mut Context<Self>) {
        self.zoom_at(step, None, cx);
    }

    /// The zoom on show: 1 fits the page (or picture) to the window.
    fn zoom_value(&self) -> f32 {
        self.zoom_free.unwrap_or(ZOOMS[self.zoom])
    }

    /// The step `by` steps from the zoom on show; from a zoom between the
    /// steps, the first step is the next one that way.
    fn zoom_step(&self, by: isize) -> usize {
        let last = ZOOMS.len() as isize - 1;
        let from = match self.zoom_free {
            Some(value) if by > 0 => {
                let next = ZOOMS.iter().position(|z| *z > value + 0.001);
                next.map_or(last + 1, |ix| ix as isize) - 1
            }
            Some(value) if by < 0 => {
                let next = ZOOMS.iter().rposition(|z| *z < value - 0.001);
                next.map_or(-1, |ix| ix as isize) + 1
            }
            _ => self.zoom as isize,
        };
        (from + by).clamp(0, last) as usize
    }

    /// Zooms to `step`, keeping what is under `at` (a window position; the
    /// top left when none) in place on a PDF or picture.
    fn zoom_at(&mut self, step: usize, at: Option<Point<Pixels>>, cx: &mut Context<Self>) {
        let step = step.min(ZOOMS.len() - 1);
        self.zoom_to(ZOOMS[step], Some(step), at, cx);
    }

    /// Zooms to `value`, a step of [`ZOOMS`] when `step` names it.
    fn zoom_to(
        &mut self,
        value: f32,
        step: Option<usize>,
        at: Option<Point<Pixels>>,
        cx: &mut Context<Self>,
    ) {
        let value = value.clamp(ZOOMS[0], ZOOMS[ZOOMS.len() - 1]);
        let ratio = value / self.zoom_value();
        if (ratio - 1.0).abs() > 0.0001 {
            let offset = self.scroll.offset();
            let view = self.scroll.bounds();
            let (px_, py_) = at.filter(|at| view.contains(at)).map_or((0.0, 0.0), |at| {
                (unpx(at.x - view.origin.x), unpx(at.y - view.origin.y))
            });
            // The point under the pointer is `p - offset` into the file;
            // after zooming it is `ratio` times further in.
            let keep = |p: f32, offset: Pixels| p * (1.0 - ratio) + unpx(offset) * ratio;
            self.scroll.set_offset(gpui::point(
                px(keep(px_, offset.x)),
                px(keep(py_, offset.y)),
            ));
        }
        match step {
            Some(step) => {
                self.zoom = step;
                self.zoom_free = None;
            }
            None => self.zoom_free = Some(value),
        }
        cx.notify();
    }

    /// Ctrl + mouse wheel (or a touchpad's smooth scroll) zooms in steps,
    /// around the pointer. Whether the wheel was taken.
    fn wheel(&mut self, event: &ScrollWheelEvent, cx: &mut Context<Self>) -> bool {
        if !event.modifiers.control || !self.zoomable() {
            return false;
        }
        let dy = match event.delta {
            // One notch is one step, however many lines it scrolls.
            ScrollDelta::Lines(delta) => delta.y.clamp(-1.0, 1.0) * WHEEL_STEP,
            ScrollDelta::Pixels(delta) => unpx(delta.y),
        };
        // A turn the other way starts afresh.
        if dy * self.wheel_zoom < 0.0 {
            self.wheel_zoom = 0.0;
        }
        self.wheel_zoom += dy;
        let steps = (self.wheel_zoom / WHEEL_STEP).trunc();
        if steps != 0.0 {
            self.wheel_zoom -= steps * WHEEL_STEP;
            let step = self.zoom_step(steps as isize);
            self.zoom_at(step, Some(event.position), cx);
        }
        true
    }

    /// A touchpad pinch zooms in steps, around the fingers.
    fn pinch(&mut self, event: &PinchEvent, cx: &mut Context<Self>) {
        if !self.zoomable() {
            return;
        }
        if event.delta * self.pinch_zoom < 0.0 {
            self.pinch_zoom = 0.0;
        }
        self.pinch_zoom += event.delta;
        let steps = (self.pinch_zoom / PINCH_STEP).trunc();
        if steps != 0.0 {
            self.pinch_zoom -= steps * PINCH_STEP;
            let step = self.zoom_step(steps as isize);
            self.zoom_at(step, Some(event.position), cx);
        }
    }

    /// Whether the file on show zooms.
    fn zoomable(&self) -> bool {
        matches!(
            self.content,
            Content::Pdf(_)
                | Content::Bitmap(..)
                | Content::Drawn(..)
                | Content::Text(..)
                | Content::Sheet(_)
                | Content::Document(_)
        )
    }

    /// Listens for Ctrl + wheel and pinches before the file scrolls.
    fn follow_zoom(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.entity().downgrade();
        gpui::canvas(
            |_, _, _| {},
            move |_, _, window, _| {
                let wheel = this.clone();
                window.on_mouse_event(move |event: &ScrollWheelEvent, phase, _, cx| {
                    if phase != DispatchPhase::Capture {
                        return;
                    }
                    let taken = wheel
                        .update(cx, |this, cx| this.wheel(event, cx))
                        .unwrap_or(false);
                    if taken {
                        cx.stop_propagation();
                    }
                });
                let pinch = this.clone();
                window.on_mouse_event(move |event: &PinchEvent, phase, _, cx| {
                    if phase == DispatchPhase::Capture {
                        pinch.update(cx, |this, cx| this.pinch(event, cx)).ok();
                        cx.stop_propagation();
                    }
                });
            },
        )
        .absolute()
        .size_0()
    }

    /// Whether a click on the space around the file may close the viewer:
    /// not while a menu, a question or a note being typed is open (the
    /// click closes those instead).
    fn may_close_by_click(&self) -> bool {
        self.text.menu.is_none()
            && self.sheet_view().is_none_or(|view| view.menu.is_none())
            && !self.markup.asking()
            && !self.markup.typing()
    }

    /// Turns the PDF's pages a quarter turn, keeping the page on show.
    /// Pages already drawn are turned at once; marks turn with them.
    fn rotate(&mut self, clockwise: bool, cx: &mut Context<Self>) {
        // A picture turns only on screen; Save keeps the file as it came.
        if let Content::Bitmap(image, (w, h)) = &mut self.content {
            let turned = turn_bitmap(image, clockwise);
            self.released.push(std::mem::replace(image, turned));
            (*w, *h) = (*h, *w);
            cx.notify();
            return;
        }
        if !matches!(self.content, Content::Pdf(_)) {
            return;
        }
        self.finish_typing(true, cx);
        let Content::Pdf(pdf) = &self.content else {
            return;
        };
        // The page the box shows.
        let count = pdf.doc.pages();
        let went = self
            .went
            .filter(|&(_, y)| (unpx(self.scroll.offset().y) - y).abs() < 0.5);
        let page = match went {
            Some((page, _)) => page,
            None => self.current_page(count),
        }
        .min(count.saturating_sub(1));
        let Content::Pdf(pdf) = &mut self.content else {
            return;
        };
        let old = pdf.doc.clone();
        let doc = Arc::new(old.turned(old.turn() + if clockwise { 1 } else { 3 }));
        self.markup.marks.turn(clockwise, |p| old.page_size(p));
        self.markup.stroke_cancel();
        // A page being drawn the old way is not wanted now.
        self.drawing = None;
        for (_, image) in pdf.pages.values_mut() {
            let turned = turn_bitmap(image, clockwise);
            self.released.push(std::mem::replace(image, turned));
        }
        pdf.doc = doc.clone();
        pdf.text.clear();
        self.text.clear();
        self.text.set_all(None);
        pdf._reading = None;
        let reading = self.read_pdf_text(doc.clone(), cx);
        let z = pdf_fit(&doc, self.frame.0) * self.zoom_value();
        let top: f32 = (0..page).map(|p| doc.page_size(p).1 * z + PAGE_GAP).sum();
        if let Content::Pdf(pdf) = &mut self.content {
            pdf._reading = Some(reading);
            pdf.z = z;
        }
        // Past the end is pulled back when the pages are laid out.
        self.scroll.set_offset(gpui::point(px(0.0), px(-top)));
        self.went = Some((page, -top));
        cx.notify();
    }

    /// Whether pages show dark: turned on, in a dark theme.
    pub(super) fn dark_pages_shown(&self) -> bool {
        self.dark_pages && self.th.dark
    }

    /// The half-moon button: pages dark, or as they are. The window keeps
    /// the choice for later files and restarts.
    fn toggle_dark_pages(&mut self, cx: &mut Context<Self>) {
        self.dark_pages = !self.dark_pages;
        cx.emit(ViewerEvent::DarkPages(self.dark_pages));
        cx.notify();
    }

    /// What the controls offer for the file on show.
    fn tools(&self, pages: Option<(usize, usize)>) -> Tools {
        let slides = matches!(&self.content, Content::Document(_)) && pages.is_some();
        let (fit, real_size, rotate) = match &self.content {
            Content::Pdf(_) => (Some(Fit::Page), false, true),
            Content::Bitmap(..) => (Some(Fit::Picture), true, true),
            Content::Drawn(_, size) => (Some(Fit::Picture), size.is_some(), false),
            Content::Document(_) | Content::Sheet(_) => (Some(Fit::Width), false, false),
            _ => (None, false, false),
        };
        // Pages, not pictures; only a dark theme makes bright pages glare.
        let dark_pages = self.th.dark
            && matches!(
                &self.content,
                Content::Pdf(_) | Content::Document(_) | Content::Sheet(_) | Content::Text(..)
            );
        Tools {
            pages,
            slides,
            fit,
            real_size,
            rotate,
            dark_pages,
            folded: Folded::NONE,
        }
    }

    /// Fit: a PDF's whole page (its height) in the window, a picture in
    /// the window, a document's or sheet's width across it.
    fn fit(&mut self, cx: &mut Context<Self>) {
        let (vw, _) = self.frame;
        let value = match &self.content {
            Content::Pdf(pdf) => {
                let page = self.shown_page.map_or(0, |(page, _)| page);
                let (w, h) = pdf.doc.page_size(page);
                let base = pdf_fit(&pdf.doc, vw);
                // Room under the bar; on a phone the foot pill takes more.
                let foot = if vw >= BAR_CONTROLS_PDF { 24.0 } else { 80.0 };
                let tall = (self.view_h - 8.0 - foot) / (h * base);
                let wide = (vw - 32.0) / (w * base);
                let value = tall.min(wide);
                let doc = pdf.doc.clone();
                self.zoom_to(value, None, None, cx);
                if let Content::Pdf(pdf) = &mut self.content {
                    pdf.z = pdf_fit(&doc, vw) * self.zoom_free.unwrap_or(value);
                }
                self.go_to_page(page, cx);
                return;
            }
            Content::Document(_) => office::document_fit_width(vw),
            Content::Sheet(view) => view.fit_width(vw),
            _ => 1.0,
        };
        if (value - 1.0).abs() < 0.005 {
            self.set_zoom(fit_step(), cx);
        } else {
            self.zoom_to(value, None, None, cx);
        }
    }

    /// Shows a picture pixel for pixel.
    fn real_size(&mut self, cx: &mut Context<Self>) {
        let size = match &self.content {
            Content::Bitmap(_, size) => Some(*size),
            Content::Drawn(_, size) => *size,
            _ => None,
        };
        let Some((w, h)) = size else {
            return;
        };
        let (vw, scale) = self.frame;
        let fitted = picture_size(w, h, scale, vw, self.view_h, 1.0).0;
        let value = (w as f32 / scale) / fitted.max(1.0);
        if (value - 1.0).abs() < 0.005 {
            self.set_zoom(fit_step(), cx);
        } else {
            self.zoom_to(value, None, None, cx);
        }
    }

    /// Goes `by` pages (or slides) from the one on show, and shows it in
    /// the page box.
    fn step_page(&mut self, by: isize, cx: &mut Context<Self>) {
        let Some((page, count)) = self.shown_page else {
            return;
        };
        let to = (page as isize + by).clamp(0, count as isize - 1) as usize;
        self.go_to_page(to, cx);
        self.shown_page = Some((to, count));
        self.goto
            .update(cx, |input, cx| input.set_text((to + 1).to_string(), cx));
    }

    /// A press on a page box arrow: one page, then more while held.
    fn press_page_arrow(&mut self, by: isize, cx: &mut Context<Self>) {
        self.step_page(by, cx);
        self.page_repeat = Some(cx.spawn(async move |this, cx| {
            let mut wait = Duration::from_millis(400);
            loop {
                cx.background_executor().timer(wait).await;
                wait = Duration::from_millis(90);
                if this.update(cx, |this, cx| this.step_page(by, cx)).is_err() {
                    break;
                }
            }
        }));
    }

    /// Scrolls the PDF so `page` (from 0) starts just below the top bar.
    fn go_to_page(&mut self, page: usize, cx: &mut Context<Self>) {
        if let Content::Document(view) = &self.content {
            let starts = view.slide_starts();
            view.go_to_slide(page.min(starts.len().saturating_sub(1)), &starts);
            cx.notify();
            return;
        }
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
        // Typing a page number, a note or a text box: the keys are theirs.
        if self.goto.focus_handle(cx).is_focused(window) || self.markup.typing() {
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
                "r" => {
                    self.rotate(!shift, cx);
                    true
                }
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
            "escape" if self.more_at.is_some() => {
                self.more_at = None;
                cx.notify();
            }
            "escape" => self.close(cx),
            "left" if shift => self.step_mail(-1, cx),
            "right" if shift => self.step_mail(1, cx),
            "left" => self.step(-1, cx),
            "right" => self.step(1, cx),
            "+" | "=" => self.set_zoom(self.zoom_step(1), cx),
            "-" => self.set_zoom(self.zoom_step(-1), cx),
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
        let dark = self.dark_pages_shown();
        let Content::Pdf(pdf) = &mut self.content else {
            return;
        };
        // Dark pages turned on or off: every page is drawn again.
        if pdf.dark != dark {
            pdf.dark = dark;
            self.released
                .extend(pdf.pages.drain().map(|(_, (_, image))| image));
            self.drawing = None;
        }
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
                .spawn(async move {
                    if dark {
                        doc.render_dark(page, scale)
                    } else {
                        doc.render(page, scale)
                    }
                    .map(bitmap)
                })
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

/// A press on the file itself: not one on the space around it.
fn on_paper(this: &mut Viewer, _: &MouseDownEvent, _: &mut Window, _: &mut Context<Viewer>) {
    this.backdrop = None;
}

/// `image` (drawn BGRA) turned a quarter turn.
/// What the controls offer for the file on show.
#[derive(Clone, Copy)]
struct Tools {
    /// The page (or slide) on show and the count.
    pages: Option<(usize, usize)>,
    slides: bool,
    fit: Option<Fit>,
    /// Real size, for a picture.
    real_size: bool,
    rotate: bool,
    /// The half-moon button that shows pages dark.
    dark_pages: bool,
    /// What of these goes into the More menu: none until
    /// [`Viewer::fold_controls`] measures the room.
    folded: Folded,
}

/// What the Fit button fits.
#[derive(Clone, Copy)]
enum Fit {
    Page,
    Picture,
    Width,
}

/// The ▲▼ at the right of the page box, shown while the pointer is on
/// it: a page back or on, more while held.
fn page_arrows(th: &Theme, cx: &mut Context<Viewer>) -> impl IntoElement {
    let arrow = |id: &'static str, name: &'static str, by: isize, tip_id: SharedString| {
        div()
            .id(id)
            .flex_1()
            .w_full()
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(3.0))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(0xffffff33)))
            .tip(tip_id, th)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    cx.stop_propagation();
                    this.press_page_arrow(by, cx);
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    cx.stop_propagation();
                    this.page_repeat = None;
                }),
            )
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(|this, _, _, _| this.page_repeat = None),
            )
            .child(icon(name, INK, 10.0))
    };
    div()
        .absolute()
        .top(px(3.0))
        .bottom(px(3.0))
        .right(px(3.0))
        .w(px(14.0))
        .flex()
        .flex_col()
        // Not occluding: the box under it keeps the pointer, so the arrows
        // stay shown; their presses stop before reaching the box.
        .opacity(0.0)
        .group_hover("viewer-page-box", |s| s.opacity(1.0))
        .child(arrow(
            "viewer-page-back",
            "chevron-up",
            -1,
            tr!("viewer-page-back-tip").into(),
        ))
        .child(arrow(
            "viewer-page-on",
            "chevron-down",
            1,
            tr!("viewer-page-on-tip").into(),
        ))
}

fn turn_bitmap(image: &Arc<RenderImage>, clockwise: bool) -> Arc<RenderImage> {
    let size = image.size(0);
    let pixels = image.as_bytes(0).and_then(|bytes| {
        RgbaImage::from_raw(size.width.0 as u32, size.height.0 as u32, bytes.to_vec())
    });
    match pixels {
        Some(pixels) => Arc::new(RenderImage::new([Frame::new(
            katna_preview::pdf::turn_image(pixels, if clockwise { 1 } else { 3 }),
        )])),
        None => image.clone(),
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
/// Paints `el` with `plain`, or when frosted (`blur` > 0) with `frosted`
/// over a blur of what is behind, corners of `radius`. Call it before
/// adding children, which must draw over the glass.
fn glassy<E: Styled + ParentElement>(el: E, plain: u32, frosted: u32, radius: f32, blur: u32) -> E {
    if blur == 0 {
        el.bg(rgba(plain))
    } else {
        el.child(katna_ui::frost::glass(
            rgba(frosted).into(),
            px(radius),
            blur as f32,
        ))
    }
}

fn bar_button(id: &'static str, name: &str, th: &Theme) -> gpui::Stateful<gpui::Div> {
    bar_button_tip(id, name, tooltip_for(id).into(), th)
}

/// A side arrow: its hover is solid, as the bar's see-through one
/// would leave a white arrow on white over a page.
fn side_button(id: &'static str, name: &str, th: &Theme) -> gpui::Stateful<gpui::Div> {
    round_button(id, name, tooltip_for(id).into(), PILL_HOVER, th)
}

/// A round button on the dark bar, with its own tooltip.
fn bar_button_tip(
    id: &'static str,
    name: &str,
    tooltip: SharedString,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
    round_button(id, name, tooltip, HOVER, th)
}

/// A page's own `color`, flipped when pages show dark.
fn page_color(color: u32, dark: bool) -> gpui::Rgba {
    rgba(if dark {
        katna_preview::dark::flip_rgba(color)
    } else {
        color
    })
}

/// A round button with `hover` under the pointer.
fn round_button(
    id: &'static str,
    name: &str,
    tooltip: SharedString,
    hover: u32,
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
        .hover(move |s| s.bg(rgba(hover)))
        .child(Ripple::new(id, rgba(0xffffff33)).centered())
        .child(icon(name, INK, 22.0))
        .tip(tooltip, th)
}

fn tooltip_for(id: &str) -> String {
    match id {
        "viewer-close" => tr!("viewer-close-tip"),
        "viewer-save" => tr!("viewer-save-tip"),
        "viewer-open" => tr!("viewer-open-tip"),
        "viewer-prev" => tr!("viewer-prev-tip"),
        "viewer-next" => tr!("viewer-next-tip"),
        "viewer-zoom-in" => tr!("viewer-zoom-in-tip"),
        "viewer-zoom-out" => tr!("viewer-zoom-out-tip"),
        _ => String::new(),
    }
}

impl Render for Viewer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Text without a size of its own follows Settings > Appearance > Scaling.
        window.set_rem_size(px(16.0));
        if self.markup.take_refocus() {
            self.focus.focus(window, cx);
        }
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
        self.view_h = vh;
        let zoom = self.zoom_value();
        let item = self.items.get(self.current).cloned();
        let name = item.as_ref().map(|i| i.name.clone()).unwrap_or_default();
        let many = self.items.len() > 1 || self.library.is_some_and(|(_, n)| n > 1);
        // Which file of how many: in the Files page's list when opened
        // from there.
        let (place, count) = self.library.unwrap_or((self.current, self.items.len()));

        // The PDF's page at the middle of the screen and its page count.
        let mut pages = None;
        let mut pdf_z = None;
        let dark = self.dark_pages_shown();
        let body: AnyElement = if matches!(self.content, Content::Document(_)) {
            let body = self.document_body(zoom, vw, cx);
            if let Content::Document(view) = &self.content {
                let starts = view.slide_starts();
                if !starts.is_empty() {
                    pages = Some((view.top_slide(&starts), starts.len()));
                }
            }
            body
        } else {
            match &self.content {
                Content::Document(_) => div().into_any_element(),
                Content::Sheet(view) => self.sheet_body(view, zoom, vw, cx),
                Content::Loading => centered(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(14.0))
                        .text_color(rgba(INK_DIM))
                        .text_size(px(14.0))
                        .child(crate::widgets::spinner("viewer-opening", INK_DIM, 32.0))
                        .child(katna_i18n::tr!("viewer-opening")),
                ),
                Content::Nothing(why) => {
                    let why = why.clone();
                    let kind = item.as_ref().map(|i| i.kind).unwrap_or(Kind::Other);
                    centered(
                        div()
                            .capture_any_mouse_down(cx.listener(on_paper))
                            .w(px(460.0_f32.min(vw - 32.0)))
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
                                        .flex_wrap()
                                        .justify_center()
                                        .gap(px(8.0))
                                        .child(
                                            text_button(
                                                "viewer-forward-big",
                                                "forward",
                                                tr!("viewer-forward"),
                                            )
                                            .on_click(
                                                cx.listener(|this, _, _, cx| this.forward(cx)),
                                            ),
                                        )
                                        .child(
                                            text_button(
                                                "viewer-open-big",
                                                "open-external",
                                                tr!("viewer-open-with"),
                                            )
                                            .on_click(
                                                cx.listener(|this, _, _, cx| this.open_with(cx)),
                                            ),
                                        )
                                        .child(
                                            text_button(
                                                "viewer-save-big",
                                                "download",
                                                tr!("viewer-save"),
                                            )
                                            .on_click(cx.listener(|this, _, _, cx| this.save(cx))),
                                        ),
                                )
                            }),
                    )
                }
                Content::Bitmap(image, (w, h)) => {
                    let (w, h) = picture_size(*w, *h, window.scale_factor(), vw, vh, zoom);
                    self.picture(img(ImageSource::Render(image.clone())), w, h, cx)
                }
                Content::Drawn(image, size) => {
                    let (w, h) = match size {
                        Some((w, h)) => picture_size(*w, *h, window.scale_factor(), vw, vh, zoom),
                        None => {
                            let side = ((vw - 160.0).min(vh - 200.0)).max(120.0) * zoom;
                            (side, side)
                        }
                    };
                    self.picture(img(ImageSource::Image(image.clone())), w, h, cx)
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
                            .capture_any_mouse_down(cx.listener(on_paper))
                            .w(px(width))
                            .h_full()
                            .rounded(px(8.0))
                            .bg(page_color(0xffffffff, dark))
                            .text_color(page_color(0x202124ff, dark))
                            .font_family("monospace")
                            .text_size(px(size)),
                        None,
                        cx,
                    );
                    div()
                        .size_full()
                        .flex()
                        .justify_center()
                        .pt(px(8.0))
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
                                                None => row.child(tr!("viewer-text-cut")),
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
                                .bg(page_color(0xffffffff, dark))
                                .capture_any_mouse_down(cx.listener(on_paper))
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
                                .children(self.page_typed(p, z, cx))
                                .when(drawing, |d| {
                                    d.cursor(markup::drawing_cursor()).on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(
                                            move |this, event: &MouseDownEvent, window, cx| {
                                                if this.press_page(p, event.position, window, cx) {
                                                    cx.stop_propagation();
                                                }
                                            },
                                        ),
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
                        .pt(px(8.0))
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
        self.shown_page = pages;
        let goto_focused = self.goto.focus_handle(cx).is_focused(window);
        if let Some((current, _)) = pages
            && !goto_focused
        {
            let number = (current + 1).to_string();
            if self.goto.read(cx).text() != number {
                self.goto.update(cx, |input, cx| input.set_text(number, cx));
            }
        }

        let zoomable = self.zoomable();
        // Wide enough: pages, zoom and turning sit in the middle of the top
        // bar, off the page. Narrower (a phone), they float at the foot.
        let in_bar = self.frame.0
            >= if pages.is_some() {
                BAR_CONTROLS_PDF
            } else {
                BAR_CONTROLS
            };
        let pill = if in_bar {
            PILL_HEIGHT
        } else {
            FOOT_PILL_HEIGHT
        };
        let mut tools = self.tools(pages);
        // At the foot the pill keeps 16 px from each side of the viewer;
        // what does not fit goes into its More menu.
        let room = if in_bar {
            f32::MAX
        } else {
            self.frame.0 - 32.0
        };
        let folded = self.fold_controls(tools, room);
        tools.folded = folded;
        if !folded.any() {
            self.more_at = None;
        }
        let controls = zoomable.then(|| self.controls(tools, zoom, goto_focused, pill, &th, cx));
        let (bar_controls, foot_controls) = if in_bar {
            (controls, None)
        } else {
            (None, controls)
        };
        // With the controls in the middle both sides share the rest evenly,
        // so the controls sit centred; without, the name takes it all.
        let side_group = |grow: bool| {
            div()
                .when(grow, |d| d.flex_1().min_w_0())
                .when(!grow, |d| d.flex_none())
                .flex()
                .flex_row()
                .items_center()
                .gap(px(12.0))
        };
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
            .map(|el| glassy(el, BAR, BAR_FROSTED, 0.0, self.th.frost))
            .child(
                side_group(true)
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
                                d.child(
                                    div()
                                        .truncate()
                                        .text_size(px(12.0))
                                        .text_color(rgba(INK_DIM))
                                        .child(if many {
                                            tr!(
                                                "viewer-size-place",
                                                size = format::size(item.size),
                                                place = place + 1,
                                                count = count
                                            )
                                        } else {
                                            format::size(item.size)
                                        }),
                                )
                            }),
                    ),
            )
            .children(bar_controls)
            .child(
                side_group(in_bar)
                    .justify_end()
                    .when(
                        self.file.is_some() && matches!(self.content, Content::Pdf(_)),
                        |d| d.child(self.markup_button(&th, cx)),
                    )
                    .when_some(self.pick, |d, on| {
                        d.child(
                            div()
                                .id("viewer-pick")
                                .flex_none()
                                .h(px(32.0))
                                .pl(px(10.0))
                                .pr(px(14.0))
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap(px(6.0))
                                .rounded_full()
                                .cursor_pointer()
                                .text_size(px(13.0))
                                .font_weight(FontWeight::MEDIUM)
                                .map(|d| {
                                    if on {
                                        d.bg(rgba(th.accent)).text_color(rgba(th.on_accent))
                                    } else {
                                        d.bg(rgba(HOVER))
                                            .text_color(rgba(INK))
                                            .hover(|s| s.bg(rgba(PILL)))
                                    }
                                })
                                .on_click(cx.listener(|_, _, _, cx| cx.emit(ViewerEvent::Pick)))
                                .child(icon("check", if on { th.on_accent } else { INK_DIM }, 18.0))
                                .child(if on {
                                    tr!("viewer-picked")
                                } else {
                                    tr!("viewer-pick")
                                }),
                        )
                    })
                    .when(self.can_show_mail, |d| {
                        d.child(
                            bar_button_tip(
                                "viewer-show-mail",
                                "mail",
                                tr!("files-show-mail").into(),
                                &th,
                            )
                            .on_click(cx.listener(|_, _, _, cx| cx.emit(ViewerEvent::ShowMail))),
                        )
                    })
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
                        d.when(self.can_reply && self.saves_marks(), |d| {
                            d.child(
                                bar_button_tip(
                                    "viewer-reply-marked",
                                    "reply",
                                    tr!("viewer-reply-marked-tip").into(),
                                    &th,
                                )
                                .on_click(cx.listener(|this, _, _, cx| this.reply_marked(cx))),
                            )
                        })
                        .child(
                            bar_button_tip(
                                "viewer-forward",
                                "forward",
                                tr!("viewer-forward-tip").into(),
                                &th,
                            )
                            .on_click(cx.listener(|this, _, _, cx| this.forward(cx))),
                        )
                        .child(
                            bar_button("viewer-open", "open-external", &th)
                                .on_click(cx.listener(|this, _, _, cx| this.open_with(cx))),
                        )
                        .child(save.on_click(cx.listener(|this, _, _, cx| this.save(cx))))
                    }),
            );

        let foot = foot_controls.map(|controls| {
            div()
                .absolute()
                .bottom(px(24.0))
                .left_0()
                .right_0()
                .flex()
                .justify_center()
                .child(controls.occlude())
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
                    side_button("viewer-prev", "chevron-left", &th)
                        .on_click(cx.listener(|this, _, _, cx| this.step(-1, cx))),
                    true,
                ),
                side(
                    side_button("viewer-next", "chevron-right", &th)
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
            // Paper (a page, the picture) forgets a press first seen here,
            // so only the dim space around the file closes the viewer.
            .capture_any_mouse_down(cx.listener(|this, event: &MouseDownEvent, _, _| {
                this.backdrop = (event.button == MouseButton::Left && this.may_close_by_click())
                    .then_some(event.position);
            }))
            .capture_any_mouse_up(cx.listener(|this, event: &MouseUpEvent, _, cx| {
                this.page_repeat = None;
                if let Some(at) = this.backdrop.take()
                    && event.button == MouseButton::Left
                    && unpx(event.position.x - at.x).hypot(unpx(event.position.y - at.y))
                        < CLICK_SLOP
                    && this.may_close_by_click()
                {
                    this.close(cx);
                }
            }))
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .occlude()
            .rounded_bl(px(self.corners.0))
            .rounded_br(px(self.corners.1))
            .map(|el| {
                let corners = gpui::Corners {
                    bottom_left: px(self.corners.0),
                    bottom_right: px(self.corners.1),
                    ..Default::default()
                };
                if self.th.frost == 0 {
                    el.bg(rgba(SCRIM))
                } else {
                    el.child(katna_ui::frost::glass(
                        rgba(SCRIM_FROSTED).into(),
                        corners,
                        self.th.frost as f32,
                    ))
                }
            })
            // The file shows below the bar, so the bar only ever frosts the
            // blurred window, never a bright page scrolled under it.
            .child(
                div()
                    .absolute()
                    .top(px(BAR_HEIGHT))
                    .left_0()
                    .right_0()
                    .bottom_0()
                    .overflow_hidden()
                    .rounded_bl(px(self.corners.0))
                    .rounded_br(px(self.corners.1))
                    .child(body),
            )
            .child(select::follow_drags(cx))
            .child(self.follow_zoom(cx))
            .child(self.follow_cell_drags(cx))
            .child(self.follow_marking(cx))
            .child(top_bar)
            .children(self.markup_pill(&th, cx))
            .children(arrows.into_iter().flatten())
            .children(foot)
            .children(self.more_menu(tools, &th, cx))
            .children(select::text_menu(self, &th, cx))
            .children(self.cell_menu(&th, cx))
            .children(self.leave_dialog(&th, cx))
            .with_animation(
                ("viewer-in", self.opened),
                Animation::new(katna_ui::motion::time(Duration::from_millis(160)))
                    .with_easing(ease_out_quint()),
                |el, t| el.opacity(t),
            )
    }
}

impl Viewer {
    /// The page box, zoom and (for a PDF) turning: in the top bar, or in a
    /// pill at the foot when the window is narrow.
    fn controls(
        &self,
        tools: Tools,
        zoom: f32,
        goto_focused: bool,
        height: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> gpui::Stateful<gpui::Div> {
        let th = *th;
        let pages = tools.pages;
        let folded = tools.folded;
        let compact = self.frame.0 < COMPACT_CONTROLS;
        // A phone's pill packs its buttons a little closer.
        let button = if compact { 32.0 } else { 36.0 };
        let separator = || {
            div()
                .mx(px(4.0))
                .w(px(1.0))
                .h(px(20.0))
                .bg(rgba(0xffffff33))
        };
        div()
            .id("viewer-controls")
            .relative()
            .flex_none()
            .h(px(height))
            .px(px(6.0))
            .rounded_full()
            .map(|el| glassy(el, PILL, PILL_FROSTED, height / 2.0, th.frost))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0))
            .text_size(px(13.0))
            .text_color(rgba(INK))
            .when_some(pages, |d, (_, count)| {
                d.child(
                    div()
                        .id("viewer-page")
                        .pl(px(if compact { 6.0 } else { 12.0 }))
                        .pr(px(10.0))
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .tip(tr!("viewer-go-to-page-tip"), &th)
                        // A phone keeps the number and drops the word.
                        .when(!compact, |d| {
                            d.child(if tools.slides {
                                tr!("viewer-slide-box")
                            } else {
                                tr!("viewer-page")
                            })
                        })
                        .child(
                            div()
                                .relative()
                                .group("viewer-page-box")
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
                                                this.goto_click =
                                                    !this.goto.focus_handle(cx).is_focused(window);
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
                                .child(page_arrows(&th, cx)),
                        )
                        .child(tr!("viewer-page-count", count = count)),
                )
                .child(div().w(px(1.0)).h(px(20.0)).bg(rgba(0xffffff33)))
            })
            .when(!folded.zoom, |d| {
                d.child(
                    bar_button("viewer-zoom-out", "zoom-out", &th)
                        .size(px(button))
                        .on_click(
                            cx.listener(|this, _, _, cx| this.set_zoom(this.zoom_step(-1), cx)),
                        ),
                )
            })
            .child(
                div()
                    .id("viewer-zoom-reset")
                    .w(px(if compact { 44.0 } else { 52.0 }))
                    .flex()
                    .justify_center()
                    .cursor_pointer()
                    .tip(tr!("viewer-fit-window-tip"), &th)
                    .on_click(cx.listener(|this, _, _, cx| this.set_zoom(fit_step(), cx)))
                    .child(tr!(
                        "viewer-zoom-level",
                        percent = format::thousands((zoom * 100.0).round() as u64)
                    )),
            )
            .when(!folded.zoom, |d| {
                d.child(
                    bar_button("viewer-zoom-in", "zoom-in", &th)
                        .size(px(button))
                        .on_click(
                            cx.listener(|this, _, _, cx| this.set_zoom(this.zoom_step(1), cx)),
                        ),
                )
            })
            .when_some(tools.fit.filter(|_| !folded.fit), |d, fit| {
                let (name, tip) = match fit {
                    Fit::Page => ("fit-page", tr!("viewer-fit-page-tip")),
                    Fit::Picture => ("fit-page", tr!("viewer-fit-picture-tip")),
                    Fit::Width => ("fit-width", tr!("viewer-fit-width-tip")),
                };
                d.child(
                    bar_button_tip("viewer-fit", name, tip.into(), &th)
                        .size(px(button))
                        .on_click(cx.listener(|this, _, _, cx| this.fit(cx))),
                )
            })
            .when(tools.real_size && !folded.fit, |d| {
                d.child(
                    bar_button_tip(
                        "viewer-real-size",
                        "real-size",
                        tr!("viewer-real-size-tip").into(),
                        &th,
                    )
                    .size(px(button))
                    .on_click(cx.listener(|this, _, _, cx| this.real_size(cx))),
                )
            })
            .when(tools.rotate && !folded.rotate, |d| {
                d.child(separator())
                    .child(
                        bar_button_tip(
                            "viewer-rotate-ccw",
                            "rotate-ccw",
                            tr!("viewer-rotate-anticlockwise-tip").into(),
                            &th,
                        )
                        .size(px(button))
                        .on_click(cx.listener(|this, _, _, cx| this.rotate(false, cx))),
                    )
                    .child(
                        bar_button_tip(
                            "viewer-rotate-cw",
                            "rotate-cw",
                            tr!("viewer-rotate-clockwise-tip").into(),
                            &th,
                        )
                        .size(px(button))
                        .on_click(cx.listener(|this, _, _, cx| this.rotate(true, cx))),
                    )
            })
            .when(tools.dark_pages && !folded.dark, |d| {
                let on = self.dark_pages;
                let tip = if on {
                    tr!("viewer-light-pages-tip")
                } else {
                    tr!("viewer-dark-pages-tip")
                };
                d.child(separator()).child(
                    bar_button_tip("viewer-dark-pages", "contrast", tip.into(), &th)
                        .size(px(button))
                        .when(on, |d| d.bg(rgba(0xffffff29)))
                        .on_click(cx.listener(|this, _, _, cx| this.toggle_dark_pages(cx))),
                )
            })
            .when(folded.any(), |d| {
                d.child(separator()).child(
                    bar_button_tip("viewer-more", "more", tr!("viewer-more-tip").into(), &th)
                        .size(px(button))
                        .when(self.more_at.is_some(), |d| d.bg(rgba(HOVER)))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, event: &MouseDownEvent, _, cx| {
                                this.more_at = match this.more_at {
                                    Some(_) => None,
                                    None => Some(event.position),
                                };
                                cx.stop_propagation();
                                cx.notify();
                            }),
                        ),
                )
            })
    }

    /// What of the controls goes into the More menu for them to fit in
    /// `room`: turning first, then Fit and Real size, then Dark pages,
    /// then the zoom buttons (the percentage stays). The page box always
    /// stays.
    fn fold_controls(&self, tools: Tools, room: f32) -> Folded {
        const ORDER: [fn(&mut Folded); 4] = [
            |f| f.rotate = true,
            |f| f.fit = true,
            |f| f.dark = true,
            |f| f.zoom = true,
        ];
        let start = Folded {
            rotate: !tools.rotate,
            fit: tools.fit.is_none() && !tools.real_size,
            dark: !tools.dark_pages,
            zoom: false,
        };
        let fits = |folded: &Folded| self.controls_width(tools, folded) <= room;
        let folded = crate::widgets::fold(start, &ORDER, fits);
        // Only what the file has counts as folded.
        Folded {
            rotate: folded.rotate && tools.rotate,
            fit: folded.fit && (tools.fit.is_some() || tools.real_size),
            dark: folded.dark && tools.dark_pages,
            zoom: folded.zoom,
        }
    }

    /// The controls pill's width with `folded` in the More menu, as
    /// [`Viewer::controls`] lays it out.
    fn controls_width(&self, tools: Tools, folded: &Folded) -> f32 {
        let compact = self.frame.0 < COMPACT_CONTROLS;
        let button = if compact { 32.0 } else { 36.0 };
        const GAP: f32 = 2.0;
        const SEPARATOR: f32 = 9.0;
        let mut items: Vec<f32> = Vec::new();
        if let Some((_, count)) = tools.pages {
            // "Page", the box, "of 12": text about 7.5 px a character.
            let word = if compact { 0.0 } else { 48.0 };
            let of = 7.5 * (3 + count.to_string().len()) as f32;
            items.push(if compact { 6.0 } else { 12.0 } + word + 46.0 + 6.0 + of + 10.0);
            items.push(1.0);
        }
        if !folded.zoom {
            items.extend([button, button]);
        }
        items.push(if compact { 44.0 } else { 52.0 });
        if !folded.fit {
            if tools.fit.is_some() {
                items.push(button);
            }
            if tools.real_size {
                items.push(button);
            }
        }
        if tools.rotate && !folded.rotate {
            items.extend([SEPARATOR, button, button]);
        }
        if tools.dark_pages && !folded.dark {
            items.extend([SEPARATOR, button]);
        }
        let any = (tools.rotate && folded.rotate)
            || ((tools.fit.is_some() || tools.real_size) && folded.fit)
            || (tools.dark_pages && folded.dark)
            || folded.zoom;
        if any {
            items.extend([SEPARATOR, button]);
        }
        12.0 + items.iter().sum::<f32>() + GAP * items.len().saturating_sub(1) as f32
    }

    /// The controls' More menu, opened where its button was pressed, with
    /// what did not fit in the pill.
    fn more_menu(&self, tools: Tools, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let at = self.more_at?;
        let folded = tools.folded;
        if !folded.any() {
            return None;
        }
        // Each item does its thing and closes the menu.
        let item = |id: &'static str,
                    name: &'static str,
                    label: String,
                    act: fn(&mut Viewer, &mut Context<Viewer>)| {
            crate::widgets::menu_item_icon(id, name, &label, th).on_click(cx.listener(
                move |this, _, _, cx| {
                    this.more_at = None;
                    act(this, cx);
                    cx.notify();
                },
            ))
        };
        let mut menu = crate::widgets::menu(th);
        if folded.zoom {
            menu = menu
                .child(item(
                    "viewer-more-zoom-in",
                    "zoom-in",
                    tr!("viewer-zoom-in"),
                    |this, cx| this.set_zoom(this.zoom_step(1), cx),
                ))
                .child(item(
                    "viewer-more-zoom-out",
                    "zoom-out",
                    tr!("viewer-zoom-out"),
                    |this, cx| this.set_zoom(this.zoom_step(-1), cx),
                ));
        }
        if folded.fit {
            if let Some(fit) = tools.fit {
                let (name, label) = match fit {
                    Fit::Page => ("fit-page", tr!("viewer-fit-page-tip")),
                    Fit::Picture => ("fit-page", tr!("viewer-fit-picture-tip")),
                    Fit::Width => ("fit-width", tr!("viewer-fit-width-tip")),
                };
                menu = menu.child(item("viewer-more-fit", name, label, |this, cx| {
                    this.fit(cx)
                }));
            }
            if tools.real_size {
                menu = menu.child(item(
                    "viewer-more-real-size",
                    "real-size",
                    tr!("viewer-real-size"),
                    |this, cx| this.real_size(cx),
                ));
            }
        }
        if folded.rotate {
            menu = menu
                .child(item(
                    "viewer-more-rotate-ccw",
                    "rotate-ccw",
                    tr!("viewer-rotate-anticlockwise"),
                    |this, cx| this.rotate(false, cx),
                ))
                .child(item(
                    "viewer-more-rotate-cw",
                    "rotate-cw",
                    tr!("viewer-rotate-clockwise"),
                    |this, cx| this.rotate(true, cx),
                ));
        }
        if folded.dark {
            let label = if self.dark_pages {
                tr!("viewer-light-pages-tip")
            } else {
                tr!("viewer-dark-pages-tip")
            };
            menu = menu.child(item(
                "viewer-more-dark-pages",
                "contrast",
                label,
                |this, cx| this.toggle_dark_pages(cx),
            ));
        }
        let close = cx.listener(|this: &mut Viewer, _: &MouseDownEvent, _, cx| {
            this.more_at = None;
            cx.notify();
        });
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .child(
                    gpui::deferred(
                        div()
                            .id("viewer-more-scrim")
                            .absolute()
                            .top(px(-2000.0))
                            .left(px(-4000.0))
                            .w(px(8000.0))
                            .h(px(6000.0))
                            .occlude()
                            .on_mouse_down(MouseButton::Left, close),
                    )
                    .with_priority(3),
                )
                .child(
                    gpui::deferred(
                        katna_ui::anchored()
                            .position(at)
                            // Pressed at the foot: the menu opens upwards.
                            .anchor(gpui::Anchor::BottomRight)
                            .snap_to_window_with_margin(px(8.0))
                            .child(div().occlude().child(menu)),
                    )
                    .with_priority(4),
                )
                .into_any_element(),
        )
    }
}

/// What of the controls has gone into their More menu.
#[derive(Clone, Copy)]
struct Folded {
    rotate: bool,
    /// Fit and Real size.
    fit: bool,
    dark: bool,
    /// Zoom in and out (the percentage stays).
    zoom: bool,
}

impl Folded {
    const NONE: Self = Self {
        rotate: false,
        fit: false,
        dark: false,
        zoom: false,
    };

    fn any(&self) -> bool {
        self.rotate || self.fit || self.dark || self.zoom
    }
}

impl Viewer {
    /// A picture `w` × `h` logical pixels, centered, scrolling when zoomed
    /// past the window.
    fn picture(&self, image: gpui::Img, w: f32, h: f32, cx: &mut Context<Self>) -> AnyElement {
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
                    .pt(px(8.0))
                    .pb(px(88.0))
                    .px(px(80.0))
                    .child(
                        image
                            .capture_any_mouse_down(cx.listener(on_paper))
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
fn text_button(id: &'static str, name: &str, label: String) -> gpui::Stateful<gpui::Div> {
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
        .child(Ripple::new(id, rgba(0xffffff33)).border(1.0))
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
        assert!(w <= 1440.0 && h <= 1000.0 - BAR_HEIGHT - 104.0 + 0.01);
        assert!((w / h - 4.0 / 3.0).abs() < 0.01);
        // Zoom scales the fitted size.
        assert_eq!(
            picture_size(200, 100, 1.0, 1600.0, 1000.0, 2.0),
            (400.0, 200.0)
        );
    }
}
