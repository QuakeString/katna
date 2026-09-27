// SPDX-License-Identifier: GPL-3.0-or-later

//! The attachment viewer: a received PDF, picture, text file, spreadsheet
//! or document shown over the mail, like webmail's preview. A dark bar on
//! top names the file and offers Save and "Open with" (the desktop's list
//! of apps); arrows at the sides go through
//! the message's other attachments; a pill at the foot zooms (and counts
//! PDF pages). Escape closes it.
//!
//! Decoding happens off the UI thread (`katna_preview`). PDF pages are
//! drawn only while on screen (and one either side), at the zoom and the
//! screen's scale, and freed when scrolled far away. Spreadsheets and
//! documents are drawn by `office.rs`.

mod office;

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, EventEmitter, FocusHandle, FontWeight,
    ImageSource, KeyDownEvent, ObjectFit, RenderImage, ScrollHandle, SharedString, Task, Window,
    div, ease_out_quint, img, prelude::*, rgba, uniform_list,
};
use katna_preview::pdf::{self, Document};
use katna_preview::{Kind, Picture, document, picture, sheet, text};
use katna_render::AttachmentFile;
use katna_ui::Ripple;
use katna_ui::px;
use katna_ui::unpx;

use self::office::{DocumentView, SheetView};
use super::attachments::{Item, bitmap, kind_badge};
use crate::format;
use crate::theme::Theme;
use crate::widgets::{icon, tip};

/// Zoom steps; 1 fits the page (or picture) to the window.
const ZOOMS: [f32; 12] = [
    0.25, 0.5, 0.67, 0.8, 0.9, 1.0, 1.25, 1.5, 2.0, 2.5, 3.0, 4.0,
];
/// A PDF page is at most this wide at zoom 1, in logical pixels.
const PAGE_WIDTH: f32 = 880.0;
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

/// What the viewer asks the window to do.
pub(super) enum ViewerEvent {
    Close,
    Save(Arc<AttachmentFile>),
    OpenWith(Arc<AttachmentFile>),
}

pub(super) struct Viewer {
    focus: FocusHandle,
    /// The raw message the attachments come from.
    raw: Arc<Vec<u8>>,
    items: Vec<Item>,
    /// The item on show.
    current: usize,
    file: Option<Arc<AttachmentFile>>,
    content: Content,
    /// A step of [`ZOOMS`].
    zoom: usize,
    scroll: ScrollHandle,
    /// Bitmaps no longer drawn; the window frees them at the next frame.
    released: Vec<Arc<RenderImage>>,
    _load: Option<Task<()>>,
    /// The page being drawn in the background.
    drawing: Option<(usize, Task<()>)>,
    /// Counts openings, to replay the fade-in.
    seq: usize,
    pub(super) th: Theme,
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
}

/// What loading found, made on a background thread.
enum Loaded {
    Pdf(Document),
    Bitmap(Arc<RenderImage>, (u32, u32)),
    Drawn(Arc<gpui::Image>, Option<(u32, u32)>),
    Text(Vec<SharedString>, bool),
    Sheet(sheet::Workbook),
    Document(document::Document),
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
        let mut this = Self {
            focus,
            raw,
            items,
            current: 0,
            file: None,
            content: Content::Loading,
            zoom: fit_step(),
            scroll: ScrollHandle::new(),
            released: Vec::new(),
            _load: None,
            drawing: None,
            seq: 0,
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

    /// Shows item `ix` (wrapping around).
    fn show(&mut self, ix: usize, cx: &mut Context<Self>) {
        if self.items.is_empty() {
            return;
        }
        let ix = ix % self.items.len();
        self.current = ix;
        self.seq += 1;
        self.zoom = fit_step();
        self.scroll.set_offset(gpui::point(px(0.0), px(0.0)));
        self.file = None;
        self.drawing = None;
        let old = std::mem::replace(&mut self.content, Content::Loading);
        self.release(old);
        let item = self.items[ix].clone();
        let raw = self.raw.clone();
        self._load = Some(cx.spawn(async move |this, cx| {
            let (file, loaded) = cx
                .background_executor()
                .spawn(async move { load(&raw, &item) })
                .await;
            this.update(cx, |this, cx| {
                this.file = file.map(Arc::new);
                this.content = match loaded {
                    Loaded::Pdf(doc) => Content::Pdf(PdfView {
                        doc: Arc::new(doc),
                        pages: HashMap::new(),
                    }),
                    Loaded::Bitmap(image, size) => Content::Bitmap(image, size),
                    Loaded::Drawn(image, size) => Content::Drawn(image, size),
                    Loaded::Text(lines, cut) => Content::Text(Rc::new(lines), cut),
                    Loaded::Sheet(book) => Content::Sheet(SheetView::new(book)),
                    Loaded::Document(doc) => Content::Document(DocumentView::new(doc)),
                    Loaded::Nothing(why) => Content::Nothing(why.into()),
                };
                cx.notify();
            })
            .ok();
        }));
        cx.notify();
    }

    fn close(&mut self, cx: &mut Context<Self>) {
        cx.emit(ViewerEvent::Close);
    }

    fn save(&mut self, cx: &mut Context<Self>) {
        if let Some(file) = &self.file {
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

    fn on_key(&mut self, event: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let keystroke = &event.keystroke;
        let ctrl = keystroke.modifiers.control;
        let view = match &self.content {
            Content::Sheet(sheet) => sheet.scroll.0.borrow().base_handle.bounds(),
            Content::Document(doc) => doc.state.viewport_bounds(),
            _ => self.scroll.bounds(),
        };
        let page = (unpx(view.size.height) - LINE_SCROLL).max(LINE_SCROLL);
        match keystroke.key.as_str() {
            "escape" => self.close(cx),
            "left" => self.show(self.current + self.items.len() - 1, cx),
            "right" => self.show(self.current + 1, cx),
            "+" | "=" => self.set_zoom(self.zoom + 1, cx),
            "-" => self.set_zoom(self.zoom.saturating_sub(1), cx),
            "0" => self.set_zoom(fit_step(), cx),
            "s" if ctrl => self.save(cx),
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

/// Extracts `item` from the raw message and decodes it for showing.
fn load(raw: &[u8], item: &Item) -> (Option<AttachmentFile>, Loaded) {
    let Some(file) = katna_render::attachment_file(raw, item.index) else {
        return (None, Loaded::Nothing("This attachment could not be read."));
    };
    let loaded = match katna_preview::kind(&file.mime, &file.name) {
        Kind::Pdf => match Document::open(file.bytes.clone()) {
            Ok(doc) => Loaded::Pdf(doc),
            Err(pdf::Error::Locked) => Loaded::Nothing("This PDF is protected with a password."),
            Err(pdf::Error::Invalid) => Loaded::Nothing("This PDF could not be read."),
        },
        Kind::Picture(Picture::Svg) => Loaded::Drawn(
            Arc::new(gpui::Image::from_bytes(
                gpui::ImageFormat::Svg,
                file.bytes.clone(),
            )),
            None,
        ),
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
                Err(_) => Loaded::Nothing("This picture could not be read."),
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
            Err(_) => Loaded::Nothing("This spreadsheet could not be read."),
        },
        Kind::Document => match document::open(file.bytes.clone()) {
            Ok(doc) => Loaded::Document(doc),
            Err(_) => Loaded::Nothing("This document could not be read."),
        },
        Kind::Other if is_old_office(&file.name) => {
            Loaded::Nothing("Old Word files (.doc) and slides have no preview yet.")
        }
        Kind::Other => Loaded::Nothing("No preview available"),
    };
    (Some(file), loaded)
}

/// Word 97–2003 and RTF documents and slides, which have no preview.
fn is_old_office(name: &str) -> bool {
    name.rsplit_once('.').is_some_and(|(_, ext)| {
        ["doc", "dot", "ppt", "pps", "pptx", "rtf"]
            .iter()
            .any(|e| ext.eq_ignore_ascii_case(e))
    })
}

fn fit_step() -> usize {
    ZOOMS.iter().position(|z| *z == 1.0).unwrap_or(0)
}

/// A round button on the dark bar.
fn bar_button(id: &'static str, name: &str, th: &Theme) -> gpui::Stateful<gpui::Div> {
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
        .tooltip(tip(tooltip_for(id), th))
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
        let zoom = ZOOMS[self.zoom];
        let item = self.items.get(self.current).cloned();
        let name = item.as_ref().map(|i| i.name.clone()).unwrap_or_default();
        let many = self.items.len() > 1;

        let mut pages_label = None;
        let body: AnyElement = if matches!(self.content, Content::Document(_)) {
            self.document_body(zoom, vw)
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
                    div()
                        .size_full()
                        .flex()
                        .justify_center()
                        .pt(px(BAR_HEIGHT + 8.0))
                        .pb(px(80.0))
                        .child(
                            div()
                                .w(px(width))
                                .h_full()
                                .rounded(px(8.0))
                                .bg(rgba(0xffffffff))
                                .text_color(rgba(0x202124ff))
                                .font_family("monospace")
                                .text_size(px(size))
                                .child(
                                    uniform_list("viewer-text", count, move |range, _, _| {
                                        range
                                            .map(|ix| {
                                                let line = match lines.get(ix) {
                                                    Some(line) => line.clone(),
                                                    None => "… (the rest of the file is not shown)"
                                                        .into(),
                                                };
                                                div()
                                                    .px(px(20.0))
                                                    .h(px(size * 1.5))
                                                    .whitespace_nowrap()
                                                    .child(line)
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
                    let fit = ((vw - 176.0).min(PAGE_WIDTH) / widest).clamp(0.2, 4.0);
                    let z = fit * zoom;
                    let scale = z * window.scale_factor();
                    let current = self.current_page(count);
                    pages_label = Some(format!("Page {} of {}", current + 1, count));
                    let wide = widest * z > vw - 32.0;
                    let pages: Vec<AnyElement> = (0..count)
                        .map(|p| {
                            let (w, h) = pdf.doc.page_size(p);
                            let (w, h) = (w * z, h * z);
                            div()
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
                                .into_any_element()
                        })
                        .collect();
                    // Pages are drawn after this frame is laid out, once the
                    // scroll handle knows which are on screen.
                    let this = cx.entity().downgrade();
                    window.on_next_frame(move |_, cx| {
                        this.update(cx, |this, cx| this.draw_pages(scale, cx)).ok();
                    });
                    div()
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
            .when(self.file.is_some(), |d| {
                d.child(
                    bar_button("viewer-open", "open-external", &th)
                        .on_click(cx.listener(|this, _, _, cx| this.open_with(cx))),
                )
                .child(
                    bar_button("viewer-save", "download", &th)
                        .on_click(cx.listener(|this, _, _, cx| this.save(cx))),
                )
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
                        .when_some(pages_label, |d, label| {
                            d.child(div().px(px(12.0)).child(label))
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
                    bar_button("viewer-prev", "chevron-left", &th).on_click(cx.listener(
                        |this, _, _, cx| this.show(this.current + this.items.len() - 1, cx),
                    )),
                    true,
                ),
                side(
                    bar_button("viewer-next", "chevron-right", &th)
                        .on_click(cx.listener(|this, _, _, cx| this.show(this.current + 1, cx))),
                    false,
                ),
            ]
        });

        div()
            .id(("viewer", self.seq))
            .track_focus(&self.focus)
            .key_context("AttachmentViewer")
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
            .child(top_bar)
            .children(arrows.into_iter().flatten())
            .children(foot)
            .with_animation(
                ("viewer-in", self.seq),
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
