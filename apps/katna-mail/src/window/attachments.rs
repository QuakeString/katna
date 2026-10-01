// SPDX-License-Identifier: GPL-3.0-or-later

//! Attachments of received mail: the cards under a message (a thumbnail
//! of pictures and of a PDF's first page, like webmail), the viewer they
//! open (`viewer.rs`), saving through the desktop's file chooser and
//! opening in another app: the desktop's default one, or one picked from
//! its "Open with" list. Settings → Default apps says, for each kind of
//! file, whether clicking a card opens the viewer or another app.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use gpui::{
    AnyElement, Context, Entity, FocusHandle, FontWeight, ImageSource, ObjectFit,
    PathPromptOptions, RenderImage, SharedString, Subscription, Task, Window, div, img, prelude::*,
    rgba,
};
use katna_core::config::{FileGroup, OpenIn};
use katna_i18n::tr;
use katna_preview::Kind;
use katna_preview::glance::{Glance, glance};
use katna_preview::image::{Frame, RgbaImage, imageops};
use katna_render::{Attachment, AttachmentFile};
use katna_store::MessageId;
use katna_ui::px;

use super::MailWindow;
use super::reader::AttachmentSource;
use super::viewer::{Viewer, ViewerEvent};
use crate::data::RowFile;
use crate::format;
use crate::theme::Theme;
use crate::widgets::{icon, tip};

const CARD_WIDTH: f32 = 180.0;
const THUMB_HEIGHT: f32 = 84.0;
/// Thumbnails are drawn at twice the card's size, sharp on HiDPI screens.
const THUMB_PIXELS: (u32, u32) = (2 * CARD_WIDTH as u32, 2 * THUMB_HEIGHT as u32);
/// The card's corner radius; its contents are rounded one pixel less, to
/// sit inside its border.
pub(super) const CARD_RADIUS: f32 = 8.0;
/// Files handed to another app are removed after this long.
const OPENED_KEEP: Duration = Duration::from_secs(24 * 60 * 60);

/// One attachment of a message, as the viewer lists it.
#[derive(Debug, Clone)]
pub(super) struct Item {
    /// Its place among the message's attachments.
    pub index: usize,
    pub name: String,
    pub size: u64,
    pub kind: Kind,
    /// Opening it in another app could run a program.
    pub risky: bool,
}

impl Item {
    pub(super) fn new(index: usize, attachment: &Attachment) -> Self {
        Self {
            index,
            name: attachment.name.clone(),
            size: attachment.size,
            kind: katna_preview::kind(&attachment.mime, &attachment.name),
            risky: katna_preview::risky(&attachment.mime, &attachment.name),
        }
    }
}

/// What the top of a card shows.
#[derive(Clone)]
pub(super) enum Thumb {
    /// A PDF's first page or a picture, and a blurred copy of it shown like
    /// frosted glass behind the name and Save button when the pointer is
    /// over the card.
    Picture {
        sharp: Arc<RenderImage>,
        frosted: Arc<RenderImage>,
    },
    /// The first cells or lines of a spreadsheet, text file or document,
    /// drawn small on a white page.
    Glance(Arc<Glance>),
}

impl Thumb {
    /// Its blurred copy, for the hover panel.
    pub(super) fn frosted(&self) -> Option<Arc<RenderImage>> {
        match self {
            Thumb::Picture { frosted, .. } => Some(frosted.clone()),
            Thumb::Glance(_) => None,
        }
    }

    /// Its bitmaps, to free when it is no longer drawn.
    pub(super) fn bitmaps(self) -> Vec<Arc<RenderImage>> {
        match self {
            Thumb::Picture { sharp, frosted } => vec![sharp, frosted],
            Thumb::Glance(_) => Vec::new(),
        }
    }
}

/// The window's attachment state.
#[derive(Default)]
pub(super) struct Files {
    thumbs: HashMap<(MessageId, usize), Thumb>,
    /// Messages whose thumbnails were made or are being made.
    asked: HashMap<MessageId, Option<Task<()>>>,
    /// Bitmaps no longer drawn, freed at the next frame.
    pub(super) released: Vec<Arc<RenderImage>>,
    pub(super) viewer: Option<Entity<Viewer>>,
    /// What had the keyboard before the viewer opened.
    restore: Option<FocusHandle>,
    /// The viewer shows the attachments of a decrypted message.
    pub(super) viewer_encrypted: bool,
    /// The open conversation's message the viewer shows, which a marked
    /// copy can be sent back to in a reply.
    viewer_message: Option<MessageId>,
    /// The mail of the file the viewer was opened on from the Files page,
    /// for its Show the mail button.
    pub(super) viewer_mail: Option<MessageId>,
    _viewer_events: Option<Subscription>,
}

impl Files {
    /// Forgets the thumbnails of messages other than `keep`.
    fn keep_only(&mut self, keep: &HashSet<MessageId>) {
        self.asked.retain(|id, _| keep.contains(id));
        let gone: Vec<_> = self
            .thumbs
            .keys()
            .filter(|(id, _)| !keep.contains(id))
            .copied()
            .collect();
        for key in gone {
            if let Some(thumb) = self.thumbs.remove(&key) {
                self.released.extend(thumb.bitmaps());
            }
        }
    }
}

/// A decoded picture as GPUI draws it (BGRA).
pub(super) fn bitmap(mut image: RgbaImage) -> Arc<RenderImage> {
    for pixel in image.pixels_mut() {
        pixel.0.swap(0, 2);
    }
    Arc::new(RenderImage::new([Frame::new(image)]))
}

/// `thumb` made small and blurred. Drawn stretched over the whole card,
/// it reads as the thumbnail seen through frosted glass (GPUI has no
/// backdrop blur).
fn frosted(thumb: &RgbaImage) -> RgbaImage {
    let (w, h) = thumb.dimensions();
    let small = imageops::thumbnail(thumb, (w / 4).max(1), (h / 4).max(1));
    imageops::fast_blur(&small, 3.0)
}

/// Where a list chip's attachment is among the message's parsed ones: the
/// same name (the `nth` of that name), else the same place among the named
/// ones, as the list read them from the message's structure.
pub(super) fn row_file_index(attachments: &[Attachment], file: &RowFile) -> Option<usize> {
    attachments
        .iter()
        .enumerate()
        .filter(|(_, a)| a.name == file.name)
        .nth(file.nth)
        .or_else(|| {
            attachments
                .iter()
                .enumerate()
                .find(|(_, a)| a.name == file.name)
        })
        .map(|(ix, _)| ix)
        .or_else(|| (file.order < attachments.len()).then_some(file.order))
}

/// A colored square with the file type's icon.
pub(super) fn kind_badge(kind: Kind, size: f32) -> AnyElement {
    let (color, name) = match kind {
        Kind::Pdf => (0xd93025ff, "file"),
        Kind::Picture(_) => (0xd93025ff, "image"),
        Kind::Text => (0x5f6368ff, "notes"),
        Kind::Sheet { .. } => (0x188038ff, "sheet"),
        Kind::Document => (0x1a73e8ff, "document"),
        Kind::Slides => (0xe8710aff, "slides"),
        Kind::Other => (0x5f6368ff, "file"),
    };
    div()
        .size(px(size))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(size * 0.2))
        .bg(rgba(color))
        .child(icon(name, 0xffffffff, size * 0.72))
        .into_any_element()
}

/// Whether the cards show a thumbnail of this kind.
pub(super) fn has_thumbnail(kind: Kind) -> bool {
    match kind {
        Kind::Pdf
        | Kind::Picture(_)
        | Kind::Text
        | Kind::Sheet { .. }
        | Kind::Document
        | Kind::Slides => true,
        Kind::Other => false,
    }
}

/// Which Default apps setting covers `kind`; none for files without a
/// preview, which always open in the viewer.
fn group(kind: Kind) -> Option<FileGroup> {
    match kind {
        Kind::Pdf => Some(FileGroup::Pdf),
        Kind::Picture(_) => Some(FileGroup::Pictures),
        Kind::Text => Some(FileGroup::Text),
        Kind::Sheet { .. } => Some(FileGroup::Spreadsheets),
        Kind::Document | Kind::Slides => Some(FileGroup::Documents),
        Kind::Other => None,
    }
}

/// Files larger than this get no glance: reading a whole workbook for a
/// few cells is not worth it.
const GLANCE_MAX_BYTES: usize = 20 * 1024 * 1024;

/// The thumbnail of attachment `index` of `raw`.
pub(super) fn thumbnail(raw: &[u8], index: usize, kind: Kind) -> Option<Thumb> {
    let file = katna_render::attachment_file(raw, index)?;
    let (w, h) = THUMB_PIXELS;
    let picture = |sharp: RgbaImage| {
        let frosted = bitmap(frosted(&sharp));
        Thumb::Picture {
            sharp: bitmap(sharp),
            frosted,
        }
    };
    match kind {
        Kind::Pdf => katna_preview::pdf::thumbnail(file.bytes, w, h).map(picture),
        Kind::Picture(format) => katna_preview::picture::thumbnail(&file.bytes, format, w, h)
            .ok()
            .map(picture),
        Kind::Text | Kind::Sheet { .. } | Kind::Document | Kind::Slides => {
            if file.bytes.len() > GLANCE_MAX_BYTES {
                return None;
            }
            let tabs = file.name.to_ascii_lowercase().ends_with(".tsv")
                || file.mime.to_ascii_lowercase().contains("tab-separated");
            glance(kind, file.bytes, &file.name, tabs).map(|g| Thumb::Glance(Arc::new(g)))
        }
        Kind::Other => None,
    }
}

/// The top of a card: its thumbnail, else the file type's badge `badge`
/// wide on a tinted ground.
pub(super) fn card_top(thumb: Option<Thumb>, kind: Kind, badge: f32, th: &Theme) -> gpui::Div {
    let inner = px(CARD_RADIUS - 1.0);
    match thumb {
        Some(Thumb::Picture { sharp, .. }) => div().size_full().child(
            img(ImageSource::Render(sharp))
                .size_full()
                .rounded_t(inner)
                .object_fit(ObjectFit::Cover),
        ),
        Some(Thumb::Glance(glance)) => div().size_full().child(glance_page(&glance, inner)),
        None => div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba(th.read_row))
            .child(kind_badge(kind, badge)),
    }
}

/// What a card shows while the pointer is over it (a member of `group`):
/// its name and size on frosted glass, with `buttons` (Save, and on the
/// Files page Show the mail) at the bottom right.
pub(super) fn hover_panel(
    group: SharedString,
    name: String,
    size: u64,
    frost: Option<Arc<RenderImage>>,
    buttons: Vec<AnyElement>,
    th: &Theme,
) -> gpui::Div {
    let inner = px(CARD_RADIUS - 1.0);
    // Frosted glass in the theme's own color: the blurred thumbnail
    // under a veil of the card's surface, text in the theme's ink.
    // Without a thumbnail there is nothing to blur, so the veil
    // is thicker and hides the file-type badge under it.
    let veil = (th.surface & 0xffff_ff00) | if frost.is_some() { 0xa6 } else { 0xf0 };
    let (ink, ink_dim) = panel_ink(th);
    div()
        .absolute()
        .top_0()
        .left_0()
        .size_full()
        // Shown on hover. Not `hidden()`: GPUI cannot switch
        // `display` on hover between layout and paint.
        .opacity(0.0)
        .group_hover(group, |s| s.opacity(1.0))
        .rounded(inner)
        .overflow_hidden()
        .children(frost.map(|image| {
            img(ImageSource::Render(image))
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .rounded(inner)
                .object_fit(ObjectFit::Fill)
        }))
        .child(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .rounded(inner)
                .bg(rgba(veil)),
        )
        .flex()
        .flex_col()
        .justify_between()
        .p(px(10.0))
        .text_color(rgba(ink))
        .child(
            div()
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .line_clamp(2)
                .child(name),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(6.0))
                .child(
                    div()
                        .flex_1()
                        .text_size(px(12.0))
                        .text_color(rgba(ink_dim))
                        .child(format::size(size)),
                )
                .children(buttons),
        )
}

/// The ink of a hover panel, and its dimmer ink.
fn panel_ink(th: &Theme) -> (u32, u32) {
    if th.dark {
        (0xffffffff, 0xffffffcc)
    } else {
        (th.text, th.text_dim)
    }
}

/// A round button on a card's hover panel.
pub(super) fn panel_button(
    id: impl Into<gpui::ElementId>,
    name: &'static str,
    label: String,
    th: &Theme,
) -> gpui::Stateful<gpui::Div> {
    let (ink, _) = panel_ink(th);
    let (button, button_hover) = if th.dark {
        (0xffffff26, 0xffffff4d)
    } else {
        (0x0000001a, 0x00000033)
    };
    div()
        .id(id)
        .size(px(32.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .bg(rgba(button))
        .hover(move |s| s.bg(rgba(button_hover)))
        .tooltip(tip(label, th))
        .child(icon(name, ink, 18.0))
}

/// Ink of a glance: it sits on a white page in every theme, like a PDF.
const PAGE_INK: u32 = 0x3c4043ff;
const GRID_LINE: u32 = 0xe0e3e7ff;
const GRID_HEADER: u32 = 0xf1f3f4ff;
const GRID_HEADER_INK: u32 = 0x80868bff;

/// A glance drawn on a white page the size of a card's top.
pub(super) fn glance_page(glance: &Glance, radius: gpui::Pixels) -> AnyElement {
    let page = div()
        .size_full()
        .overflow_hidden()
        .rounded_t(radius)
        .bg(rgba(0xffffffff))
        .text_color(rgba(PAGE_INK))
        .text_size(px(7.0))
        .line_height(px(9.0));
    match glance {
        Glance::Cells(rows) => {
            let columns = rows.first().map_or(1, Vec::len).max(1);
            let numbers = 14.0;
            let width = ((CARD_WIDTH - numbers) / columns as f32).max(28.0);
            let cell = |text: SharedString, header: bool| {
                div()
                    .w(px(width))
                    .flex_none()
                    .h_full()
                    .px(px(2.0))
                    .flex()
                    .items_center()
                    .border_r_1()
                    .border_color(rgba(GRID_LINE))
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .when(header, |c| {
                        c.justify_center().text_color(rgba(GRID_HEADER_INK))
                    })
                    .child(text)
            };
            let row = |number: SharedString, header: bool| {
                div()
                    .h(px(11.0))
                    .flex_none()
                    .flex()
                    .flex_row()
                    .border_b_1()
                    .border_color(rgba(GRID_LINE))
                    .when(header, |r| r.bg(rgba(GRID_HEADER)))
                    .child(
                        div()
                            .w(px(numbers))
                            .flex_none()
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(rgba(GRID_HEADER))
                            .text_color(rgba(GRID_HEADER_INK))
                            .border_r_1()
                            .border_color(rgba(GRID_LINE))
                            .child(number),
                    )
            };
            let header = row(SharedString::default(), true).children(
                (0..columns)
                    .map(|ix| cell(katna_preview::sheet::column_name(ix as u32).into(), true)),
            );
            page.flex()
                .flex_col()
                .child(header)
                .children(rows.iter().enumerate().map(|(n, cells)| {
                    row((n + 1).to_string().into(), false)
                        .children(cells.iter().map(|c| cell(c.clone().into(), false)))
                }))
                .into_any_element()
        }
        Glance::Lines(lines) => page
            .px(px(10.0))
            .py(px(8.0))
            .flex()
            .flex_col()
            .children(lines.iter().map(|line| {
                div()
                    .whitespace_nowrap()
                    .overflow_hidden()
                    .when(line.heading, |l| {
                        l.font_weight(FontWeight::BOLD).text_size(px(8.0))
                    })
                    // Keeps blank lines.
                    .min_h(px(9.0))
                    .child(line.text.clone())
            }))
            .into_any_element(),
        Glance::Slide(lines) => page
            .p(px(12.0))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(2.0))
            .text_center()
            .children(lines.iter().map(|line| {
                div()
                    .max_w_full()
                    .whitespace_nowrap()
                    .overflow_hidden()
                    .when(line.heading, |l| {
                        l.font_weight(FontWeight::BOLD).text_size(px(10.0))
                    })
                    .child(line.text.clone())
            }))
            .into_any_element(),
    }
}

impl MailWindow {
    /// Makes the thumbnails of the open messages' attachments in the
    /// background, and forgets those of messages no longer open.
    pub(super) fn request_thumbnails(&mut self, cx: &mut Context<Self>) {
        // Settings > Appearance > Attachment previews: none are made, and
        // those made are let go.
        if !self.config.mail.attachment_previews {
            self.files.keep_only(&HashSet::new());
            return;
        }
        let open: Vec<(MessageId, Vec<(usize, Kind)>)> = self
            .reader
            .iter()
            .flat_map(|reader| reader.open_views())
            .map(|(id, view)| {
                let list = view
                    .attachments
                    .iter()
                    .enumerate()
                    .map(|(ix, a)| (ix, katna_preview::kind(&a.mime, &a.name)))
                    .filter(|(_, kind)| has_thumbnail(*kind))
                    .collect::<Vec<_>>();
                (id, list)
            })
            .filter(|(_, list)| !list.is_empty())
            .collect();
        self.files
            .keep_only(&open.iter().map(|(id, _)| *id).collect());
        for (id, list) in open {
            if self.files.asked.contains_key(&id) {
                continue;
            }
            // Thumbnails of decrypted attachments stay in memory like the
            // rest of the decrypted message.
            let Some((raw, _)) = self.attachment_raw(id) else {
                continue;
            };
            let task = cx.spawn(async move |this, cx| {
                let thumbs: Vec<(usize, Thumb)> = cx
                    .background_executor()
                    .spawn(async move {
                        list.into_iter()
                            .filter_map(|(ix, kind)| Some((ix, thumbnail(&raw, ix, kind)?)))
                            .collect()
                    })
                    .await;
                this.update(cx, |this, cx| {
                    let files = &mut this.files;
                    if let Some(task) = files.asked.get_mut(&id) {
                        *task = None;
                        files
                            .thumbs
                            .extend(thumbs.into_iter().map(|(ix, t)| ((id, ix), t)));
                    } else {
                        // The message closed meanwhile.
                        files
                            .released
                            .extend(thumbs.into_iter().flat_map(|(_, t)| t.bitmaps()));
                    }
                    cx.notify();
                })
                .ok();
            });
            self.files.asked.insert(id, Some(task));
        }
    }

    /// Frees bitmaps nothing draws any more. Call at the start of a frame.
    pub(super) fn release_images(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let mut released = std::mem::take(&mut self.files.released);
        if let Some(viewer) = &self.files.viewer {
            released.extend(viewer.update(cx, |viewer, _| viewer.take_released(false)));
        }
        for image in released {
            window.drop_image(image).ok();
        }
    }

    /// The cards of message `id`'s attachments (`list` pairs each with its
    /// place among all of them).
    pub(super) fn attachment_cards(
        &self,
        id: MessageId,
        list: &[(usize, &Attachment)],
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if list.is_empty() {
            return None;
        }
        let count = list.len();
        let indices: Vec<usize> = list.iter().map(|&(ix, _)| ix).collect();
        let cards = list.iter().map(|&(ix, attachment)| {
            let item = Item::new(ix, attachment);
            let group = SharedString::from(format!("attachment-{}-{ix}", id.0));
            let thumb = self
                .files
                .thumbs
                .get(&(id, ix))
                .filter(|_| self.config.mail.attachment_previews)
                .cloned();
            let name = item.name.clone();
            let frost = thumb.as_ref().and_then(Thumb::frosted);
            let top = card_top(thumb, item.kind, 36.0, th);
            let save_name = name.clone();
            let save = panel_button(
                ("attachment-save", ix),
                "download",
                tr!("attachment-save"),
                th,
            )
            .on_click(cx.listener(move |this, _, _, cx| {
                cx.stop_propagation();
                this.save_from_message(id, ix, &save_name, cx);
            }));
            let forward_name = name.clone();
            let forward = panel_button(
                ("attachment-forward", ix),
                "forward",
                tr!("attachment-forward"),
                th,
            )
            .on_click(cx.listener(move |this, _, window, cx| {
                cx.stop_propagation();
                this.forward_from_message(id, ix, &forward_name, window, cx);
            }));
            let overlay = hover_panel(
                group.clone(),
                name.clone(),
                item.size,
                frost,
                vec![forward.into_any_element(), save.into_any_element()],
                th,
            );
            div()
                .id(("attachment", ix))
                .group(group)
                .relative()
                .w(px(CARD_WIDTH))
                .flex_none()
                .flex()
                .flex_col()
                .overflow_hidden()
                .rounded(px(CARD_RADIUS))
                .border_1()
                .border_color(rgba(th.divider))
                .cursor_pointer()
                .on_click(cx.listener(move |this, _, window, cx| {
                    cx.stop_propagation();
                    this.open_attachment(id, ix, window, cx);
                }))
                .child(div().h(px(THUMB_HEIGHT)).w_full().child(top))
                .child(
                    div()
                        .h(px(40.0))
                        .px(px(10.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.0))
                        .border_t_1()
                        .border_color(rgba(th.divider))
                        .child(kind_badge(item.kind, 18.0))
                        .child(
                            div()
                                .min_w_0()
                                .truncate()
                                .text_size(px(13.0))
                                .text_color(rgba(th.text))
                                .child(name),
                        ),
                )
                .child(overlay)
        });
        Some(
            div()
                .pt(px(16.0))
                .mt(px(16.0))
                .border_t_1()
                .border_color(rgba(th.divider))
                .child(
                    div()
                        .mb(px(12.0))
                        .h(px(32.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(13.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgba(th.text_dim))
                                .child(tr!("attachment-count", count = count)),
                        )
                        .when(count > 1, |row| {
                            row.child(
                                div()
                                    .id(("attachments-save-all", id.0 as usize))
                                    .h(px(32.0))
                                    .px(px(12.0))
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(6.0))
                                    .rounded_full()
                                    .cursor_pointer()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(rgba(th.text_dim))
                                    .hover(|s| s.bg(rgba(th.hover)))
                                    .tooltip(tip(tr!("attachment-save-all-tooltip"), th))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        cx.stop_propagation();
                                        this.save_all(id, indices.clone(), cx);
                                    }))
                                    .child(icon("download", th.text_dim, 18.0))
                                    .child(tr!("attachment-save-all")),
                            )
                        }),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap(px(12.0))
                        .children(cards),
                )
                .into_any_element(),
        )
    }

    /// Opens attachment `index` of message `id` in the viewer.
    fn open_attachment(
        &mut self,
        id: MessageId,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(view) = self.reader.as_ref().and_then(|r| r.view(Some(id))) else {
            return;
        };
        let items: Vec<Item> = view
            .attachments
            .iter()
            .enumerate()
            .map(|(ix, a)| Item::new(ix, a))
            .collect();
        let open_in = items
            .get(index)
            .map_or(OpenIn::Katna, |item| self.open_in(item));
        if open_in != OpenIn::Katna {
            let name = items.get(index).map(|i| i.name.clone()).unwrap_or_default();
            self.open_elsewhere(id, index, &name, open_in == OpenIn::Ask, cx);
            return;
        }
        let Some((raw, encrypted)) = self.attachment_raw(id) else {
            self.show_snackbar(tr!("attachment-not-downloaded"), None, cx);
            return;
        };
        self.show_viewer(raw, encrypted, items, index, Some(id), window, cx);
    }

    /// Opens an attachment chip of the message list in the viewer, with
    /// the message's other attachments a click of the arrows away.
    pub(super) fn open_row_file(
        &mut self,
        file: &RowFile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(AttachmentSource::Sealed) = self
            .reader
            .as_ref()
            .map(|r| r.attachment_source(file.message))
        {
            self.files_menu = None;
            self.show_snackbar(tr!("attachment-open-message"), None, cx);
            return;
        }
        let Some((raw, encrypted)) = self.attachment_raw(file.message) else {
            // Not downloaded yet: the chip fills while it downloads.
            self.download_row_file(file, window, cx);
            return;
        };
        self.files_menu = None;
        let view = katna_render::message_view(&raw);
        let Some(index) = row_file_index(&view.attachments, file) else {
            self.show_snackbar(tr!("attachment-not-found"), None, cx);
            return;
        };
        let items: Vec<Item> = view
            .attachments
            .iter()
            .enumerate()
            .map(|(ix, a)| Item::new(ix, a))
            .collect();
        let open_in = self.open_in(&items[index]);
        if open_in != OpenIn::Katna {
            let name = items[index].name.clone();
            self.open_elsewhere(file.message, index, &name, open_in == OpenIn::Ask, cx);
            return;
        }
        self.show_viewer(raw, encrypted, items, index, None, window, cx);
        // From the Files page the viewer can show the file's mail.
        // Its arrows page through the page's files.
        if self.app == super::apps::App::Files
            && let Some(viewer) = &self.files.viewer
        {
            self.files.viewer_mail = Some(file.message);
            let place = self.library_place(file);
            viewer.update(cx, |viewer, cx| {
                viewer.can_show_mail = true;
                viewer.library = place;
                cx.notify();
            });
        }
    }

    /// Where a click opens `item`: as Default apps says for its type; a
    /// file Katna has no preview for goes straight to the desktop's default
    /// app, unless it could run a program (the viewer then offers only
    /// Save).
    fn open_in(&self, item: &Item) -> OpenIn {
        match group(item.kind) {
            Some(group) => self.config.mail.open.get(group),
            None if item.risky => OpenIn::Katna,
            None => OpenIn::System,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn show_viewer(
        &mut self,
        raw: Arc<Vec<u8>>,
        encrypted: bool,
        items: Vec<Item>,
        index: usize,
        message: Option<MessageId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.close_viewer(window, cx);
        self.files.restore = window.focused(cx);
        self.files.viewer_encrypted = encrypted;
        self.files.viewer_message = message;
        let th = self.theme(window);
        let reply = message.is_some();
        let viewer = cx.new(|cx| Viewer::new(raw, items, index, reply, th, window, cx));
        self.files._viewer_events = Some(cx.subscribe_in(&viewer, window, Self::on_viewer));
        self.files.viewer = Some(viewer);
        cx.notify();
    }

    fn on_viewer(
        &mut self,
        _: &Entity<Viewer>,
        event: &ViewerEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            ViewerEvent::Close => self.close_viewer(window, cx),
            ViewerEvent::Save(file) => self.save_attachment(file.clone(), cx),
            ViewerEvent::OpenWith(file) => {
                let encrypted = self.files.viewer_encrypted;
                self.open_attachment_with(file.clone(), true, encrypted, cx)
            }
            ViewerEvent::Unreadable(file) => {
                // Katna could not show the file it was asked to open: the
                // desktop's app gets it instead (or the choice of app, when
                // Default apps asks for this type).
                let encrypted = self.files.viewer_encrypted;
                let kind = katna_preview::kind(&file.mime, &file.name);
                let ask = group(kind).is_some_and(|g| self.config.mail.open.get(g) == OpenIn::Ask);
                self.close_viewer(window, cx);
                self.open_attachment_with(file.clone(), ask, encrypted, cx)
            }
            ViewerEvent::Step(by) => self.step_library(*by, window, cx),
            ViewerEvent::Paged(index) => self.paged_library(*index, cx),
            ViewerEvent::ShowMail => {
                let mail = self.files.viewer_mail;
                self.close_viewer(window, cx);
                if let Some(id) = mail {
                    self.show_file_mail(id, window, cx);
                }
            }
            ViewerEvent::Forward(file) => {
                let file = file.clone();
                self.close_viewer(window, cx);
                self.new_mail_with_file(&file, window, cx);
            }
            ViewerEvent::Reply(file) => {
                let message = self.files.viewer_message;
                self.close_viewer(window, cx);
                if let Some(id) = message {
                    self.reply_with_file(id, file, window, cx);
                }
            }
        }
    }

    pub(super) fn close_viewer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(viewer) = self.files.viewer.take() else {
            return;
        };
        let released = viewer.update(cx, |viewer, _| viewer.take_released(true));
        self.files.released.extend(released);
        self.files._viewer_events = None;
        self.files.viewer_mail = None;
        match self.files.restore.take() {
            Some(focus) => focus.focus(window, cx),
            None => self.list_focus.focus(window, cx),
        }
        cx.notify();
    }

    /// Saves attachment `index` of message `id`.
    pub(super) fn save_from_message(
        &mut self,
        id: MessageId,
        index: usize,
        name: &str,
        cx: &mut Context<Self>,
    ) {
        let Some((raw, _)) = self.attachment_raw(id) else {
            self.show_snackbar(tr!("attachment-not-downloaded"), None, cx);
            return;
        };
        let name = name.to_owned();
        cx.spawn(async move |this, cx| {
            let file = cx
                .background_executor()
                .spawn(async move { katna_render::attachment_file(&raw, index) })
                .await;
            this.update(cx, |this, cx| match file {
                Some(file) => this.save_attachment(Arc::new(file), cx),
                None => this.show_snackbar(
                    tr!("attachment-read-failed", name = name.as_str()),
                    None,
                    cx,
                ),
            })
            .ok();
        })
        .detach();
    }

    /// Starts a new mail with only attachment `index` of message `id`.
    fn forward_from_message(
        &mut self,
        id: MessageId,
        index: usize,
        name: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((raw, _)) = self.attachment_raw(id) else {
            self.show_snackbar(tr!("attachment-not-downloaded"), None, cx);
            return;
        };
        let name = name.to_owned();
        cx.spawn_in(window, async move |this, cx| {
            let file = cx
                .background_executor()
                .spawn(async move { katna_render::attachment_file(&raw, index) })
                .await;
            this.update_in(cx, |this, window, cx| match file {
                Some(file) => this.new_mail_with_file(&file, window, cx),
                None => this.show_snackbar(
                    tr!("attachment-read-failed", name = name.as_str()),
                    None,
                    cx,
                ),
            })
            .ok();
        })
        .detach();
    }

    /// Asks for a folder (the desktop's folder chooser, else Downloads),
    /// then saves attachments `indices` of message `id` there, each as its
    /// own file. Files already there are kept: a new file of the same name
    /// is saved as "name (1).ext".
    fn save_all(&mut self, id: MessageId, indices: Vec<usize>, cx: &mut Context<Self>) {
        let Some((raw, _)) = self.attachment_raw(id) else {
            self.show_snackbar(tr!("attachment-not-downloaded"), None, cx);
            return;
        };
        let prompt = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some(tr!("attachment-save-here").into()),
        });
        cx.spawn(async move |this, cx| {
            let dir = match prompt.await {
                Ok(Ok(Some(mut dirs))) if !dirs.is_empty() => dirs.swap_remove(0),
                Ok(Ok(_)) => return,
                // No folder chooser (no desktop portal): save to Downloads.
                _ => download_dir(),
            };
            let total = indices.len();
            let (saved, failed, dir) = cx
                .background_executor()
                .spawn(async move {
                    let mut saved = 0;
                    let mut failed = Vec::new();
                    for index in indices {
                        let Some(file) = katna_render::attachment_file(&raw, index) else {
                            failed.push(tr!("attachment-numbered", number = index + 1));
                            continue;
                        };
                        match save_new(&dir, &safe_name(&file.name), &file.bytes) {
                            Ok(_) => saved += 1,
                            Err(err) => failed.push(format!("{}: {err}", file.name)),
                        }
                    }
                    (saved, failed, dir)
                })
                .await;
            this.update(cx, |this, cx| {
                let place = folder_label(&dir);
                let text = match failed.first() {
                    None => tr!("attachment-saved-all", count = saved, place = place),
                    Some(first) => tr!(
                        "attachment-saved-some",
                        saved = saved,
                        total = total,
                        place = place,
                        failed = first.as_str()
                    ),
                };
                this.show_snackbar(text, None, cx);
                // Settings > Default apps > After saving.
                if saved > 0 && this.config.mail.open_saved_folder {
                    cx.open_with_system(&dir);
                }
            })
            .ok();
        })
        .detach();
    }

    /// Asks where to save `file` (the desktop's file chooser), then saves it.
    fn save_attachment(&mut self, file: Arc<AttachmentFile>, cx: &mut Context<Self>) {
        let dir = download_dir();
        let name = safe_name(&file.name);
        let prompt = cx.prompt_for_new_path(&dir, Some(&name));
        cx.spawn(async move |this, cx| {
            let target = match prompt.await {
                Ok(Ok(Some(path))) => path,
                Ok(Ok(None)) => return,
                // No file chooser (no desktop portal): save to Downloads.
                _ => unique_path(&dir, &name),
            };
            let bytes = file.clone();
            let saved = cx
                .background_executor()
                .spawn(async move { std::fs::write(&target, &bytes.bytes).map(|()| target) })
                .await;
            this.update(cx, |this, cx| {
                let text = match saved {
                    Ok(path) => {
                        // Settings > Default apps > After saving.
                        if this.config.mail.open_saved_folder {
                            cx.reveal_path(&path);
                        }
                        tr!("attachment-saved-to", path = path.display().to_string())
                    }
                    Err(err) => tr!(
                        "attachment-save-failed",
                        name = file.name.as_str(),
                        error = err.to_string()
                    ),
                };
                this.show_snackbar(text, None, cx);
            })
            .ok();
        })
        .detach();
    }

    /// Opens attachment `index` of message `id` in another app, without the
    /// viewer: the desktop's default app, or one it asks for when `ask`.
    fn open_elsewhere(
        &mut self,
        id: MessageId,
        index: usize,
        name: &str,
        ask: bool,
        cx: &mut Context<Self>,
    ) {
        let Some((raw, encrypted)) = self.attachment_raw(id) else {
            self.show_snackbar(tr!("attachment-not-downloaded"), None, cx);
            return;
        };
        let name = name.to_owned();
        cx.spawn(async move |this, cx| {
            let file = cx
                .background_executor()
                .spawn(async move { katna_render::attachment_file(&raw, index) })
                .await;
            this.update(cx, |this, cx| match file {
                Some(file) => this.open_attachment_with(Arc::new(file), ask, encrypted, cx),
                None => this.show_snackbar(
                    tr!("attachment-read-failed", name = name.as_str()),
                    None,
                    cx,
                ),
            })
            .ok();
        })
        .detach();
    }

    /// Hands a read-only copy of `file` to another app: one the user picks
    /// from the desktop's "Open with" list when `ask` (the default app if
    /// the desktop cannot ask), else the default app for its type. The copy
    /// is in the cache, or, for an `encrypted` message's attachment, in
    /// memory (never on disk). Programs and scripts are never handed over.
    pub(super) fn open_attachment_with(
        &mut self,
        file: Arc<AttachmentFile>,
        ask: bool,
        encrypted: bool,
        cx: &mut Context<Self>,
    ) {
        if katna_preview::risky(&file.mime, &file.name) {
            self.show_snackbar(tr!("attachment-risky"), None, cx);
            return;
        }
        let dir = if encrypted {
            match memory_dir() {
                Some(dir) => dir,
                None => {
                    self.show_snackbar(tr!("attachment-encrypted-open"), None, cx);
                    return;
                }
            }
        } else {
            self.paths.cache_dir().join("opened")
        };
        cx.spawn(async move |this, cx| {
            let name = file.name.clone();
            let written = cx
                .background_executor()
                .spawn(async move { write_for_opening(&dir, &file, SystemTime::now()) })
                .await;
            let path = match written {
                Ok(path) => path,
                Err(err) => {
                    let text = tr!(
                        "attachment-open-failed",
                        name = name.as_str(),
                        error = err.to_string()
                    );
                    this.update(cx, |this, cx| this.show_snackbar(text, None, cx))
                        .ok();
                    return;
                }
            };
            if ask && choose_app(&path).await {
                return;
            }
            cx.update(|cx| cx.open_with_system(&path));
        })
        .detach();
    }
}

impl MailWindow {
    /// The message the attachments of `id` are read from (as GnuPG opened
    /// it, for protected mail), and whether it was encrypted. `None` when
    /// it is not downloaded, or protected and not opened yet.
    pub(super) fn attachment_raw(&self, id: MessageId) -> Option<(Arc<Vec<u8>>, bool)> {
        match self.reader.as_ref().map(|r| r.attachment_source(id)) {
            Some(AttachmentSource::Opened { raw, encrypted }) => Some((raw, encrypted)),
            Some(AttachmentSource::Sealed) => None,
            Some(AttachmentSource::Stored) | None => {
                let raw = self.mail.as_ref().ok()?.raw(id)?;
                Some((Arc::new(raw), false))
            }
        }
    }
}

/// Where decrypted attachments are handed to other apps from: a folder in
/// `XDG_RUNTIME_DIR` when that is in memory (tmpfs), so they never reach
/// the disk. It is private to the user and emptied at logout.
fn memory_dir() -> Option<PathBuf> {
    let runtime = PathBuf::from(std::env::var_os("XDG_RUNTIME_DIR")?);
    let mounts = std::fs::read_to_string("/proc/self/mounts").ok()?;
    in_memory(&runtime, &mounts).then(|| runtime.join("katna").join("opened"))
}

/// Whether `path` lies on a file system kept in memory, by the mount
/// table (`/proc/self/mounts`): the deepest mount point holding it.
fn in_memory(path: &Path, mounts: &str) -> bool {
    mounts
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let _device = fields.next()?;
            let point = fields.next()?.replace("\\040", " ");
            let kind = fields.next()?;
            Some((PathBuf::from(point), kind))
        })
        .filter(|(point, _)| path.starts_with(point))
        .max_by_key(|(point, _)| point.as_os_str().len())
        .is_some_and(|(_, kind)| kind == "tmpfs" || kind == "ramfs")
}

/// Asks the desktop to open `path` with an app the user picks (the "Open
/// with" portal). False when there is no portal to ask.
#[cfg(not(windows))]
async fn choose_app(path: &Path) -> bool {
    let Ok(file) = std::fs::File::open(path) else {
        return false;
    };
    ashpd::desktop::open_uri::OpenFileRequest::default()
        .ask(true)
        .writeable(false)
        .send_file(&file)
        .await
        .is_ok()
}

/// Windows' "Open with" dialog, from `rundll32 shell32.dll,OpenAs_RunDLL`.
#[cfg(windows)]
async fn choose_app(path: &Path) -> bool {
    std::process::Command::new("rundll32.exe")
        .arg("shell32.dll,OpenAs_RunDLL")
        .arg(path)
        .spawn()
        .is_ok()
}

/// Writes `file` read-only into a fresh folder under `dir`, first removing
/// what earlier openings left there.
fn write_for_opening(
    dir: &Path,
    file: &AttachmentFile,
    now: SystemTime,
) -> std::io::Result<PathBuf> {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let old = entry
                .metadata()
                .and_then(|m| m.modified())
                .ok()
                .and_then(|m| now.duration_since(m).ok())
                .is_some_and(|age| age > OPENED_KEEP);
            if old {
                std::fs::remove_dir_all(entry.path()).ok();
            }
        }
    }
    let stamp = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let folder = dir.join(format!("{stamp:x}"));
    std::fs::create_dir_all(&folder)?;
    let path = folder.join(safe_name(&file.name));
    std::fs::write(&path, &file.bytes)?;
    let mut permissions = std::fs::metadata(&path)?.permissions();
    permissions.set_readonly(true);
    std::fs::set_permissions(&path, permissions)?;
    Ok(path)
}

/// `name` as a file name: no folders, no hidden files, not too long.
fn safe_name(name: &str) -> String {
    let name: String = name
        .chars()
        .filter(|&c| !katna_preview::invisible(c))
        .map(|c| {
            if matches!(c, '/' | '\\' | '\0') || c.is_control() {
                '_'
            } else {
                c
            }
        })
        .collect();
    let name = name.trim().trim_start_matches('.').trim();
    let mut name = if name.is_empty() {
        "attachment".to_owned()
    } else {
        name.to_owned()
    };
    // Most file systems allow 255 bytes; keep the extension.
    while name.len() > 200 {
        let cut = match name.rsplit_once('.') {
            Some((stem, ext)) if ext.len() < 16 && stem.len() > 1 => {
                let mut stem = stem.to_owned();
                stem.pop();
                format!("{stem}.{ext}")
            }
            _ => {
                let mut name = name.clone();
                name.pop();
                name
            }
        };
        name = cut;
    }
    name
}

/// `dir/name`, then `dir/name (1)`, `dir/name (2)` and so on.
fn candidates(dir: &Path, name: &str) -> impl Iterator<Item = PathBuf> {
    let (stem, ext) = match name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() => (stem.to_owned(), format!(".{ext}")),
        _ => (name.to_owned(), String::new()),
    };
    let dir = dir.to_owned();
    std::iter::once(dir.join(name))
        .chain((1..).map(move |n| dir.join(format!("{stem} ({n}){ext}"))))
}

/// The first of `candidates` that does not exist yet.
pub(super) fn unique_path(dir: &Path, name: &str) -> PathBuf {
    candidates(dir, name)
        .find(|p| !p.exists())
        .unwrap_or_else(|| dir.join(name))
}

/// Writes `bytes` to a new file `dir/name`, or `dir/name (1)` and so on,
/// never replacing a file that is there (even one made meanwhile).
fn save_new(dir: &Path, name: &str, bytes: &[u8]) -> std::io::Result<PathBuf> {
    use std::io::{ErrorKind, Write};
    for path in candidates(dir, name).take(10_000) {
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut file) => return file.write_all(bytes).map(|()| path),
            Err(err) if err.kind() == ErrorKind::AlreadyExists => continue,
            Err(err) => return Err(err),
        }
    }
    Err(ErrorKind::AlreadyExists.into())
}

/// How a toast names folder `dir`: "Downloads" for the download folder,
/// else its path.
fn folder_label(dir: &Path) -> String {
    if dir == download_dir() {
        dir.file_name().map_or_else(
            || dir.display().to_string(),
            |n| n.to_string_lossy().into_owned(),
        )
    } else {
        dir.display().to_string()
    }
}

/// The user's download folder (`XDG_DOWNLOAD_DIR` of `user-dirs.dirs`),
/// else `~/Downloads`, else home.
pub(super) fn download_dir() -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| "/".into());
    let config = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".config"));
    std::fs::read_to_string(config.join("user-dirs.dirs"))
        .ok()
        .and_then(|text| user_dir(&text, "XDG_DOWNLOAD_DIR", &home))
        .filter(|dir| dir.is_dir())
        .or_else(|| Some(home.join("Downloads")).filter(|d| d.is_dir()))
        .unwrap_or(home)
}

/// A folder of `user-dirs.dirs` (lines like `XDG_DOWNLOAD_DIR="$HOME/Downloads"`).
fn user_dir(text: &str, key: &str, home: &Path) -> Option<PathBuf> {
    text.lines().find_map(|line| {
        let value = line.trim().strip_prefix(key)?.trim().strip_prefix('=')?;
        let value = value.trim().trim_matches('"');
        match value.strip_prefix("$HOME") {
            Some(rest) => Some(home.join(rest.trim_start_matches('/'))),
            None if value.starts_with('/') => Some(PathBuf::from(value)),
            None => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_file_systems_are_found() {
        let mounts = "/dev/vda / ext4 rw 0 0\n\
tmpfs /run tmpfs rw,nosuid 0 0\n\
tmpfs /run/user/1000 tmpfs rw,nosuid,mode=700 0 0\n\
/dev/vdb /run/user/1000/doc fuse rw 0 0\n\
/dev/vdc /mnt/my\\040disk ext4 rw 0 0\n";
        assert!(in_memory(Path::new("/run/user/1000"), mounts));
        assert!(in_memory(Path::new("/run/user/1000/katna/opened"), mounts));
        assert!(!in_memory(Path::new("/run/user/1000/doc/x"), mounts));
        assert!(!in_memory(Path::new("/home/sara/.cache"), mounts));
        assert!(!in_memory(Path::new("/mnt/my disk/x"), mounts));
        // `/runner` is not under `/run`.
        assert!(!in_memory(Path::new("/runner"), mounts));
    }

    #[test]
    fn list_chips_find_their_attachment() {
        let attachment = |name: &str| Attachment {
            name: name.into(),
            size: 1,
            mime: "application/pdf".into(),
            content_id: None,
        };
        let parsed = [
            attachment("a.pdf"),
            attachment("b.pdf"),
            attachment("a.pdf"),
        ];
        let file = |name: &str, nth, order| RowFile {
            message: MessageId(1),
            name: name.into(),
            mime: "application/pdf".into(),
            size: 1,
            nth,
            order,
        };
        assert_eq!(row_file_index(&parsed, &file("a.pdf", 1, 2)), Some(2));
        assert_eq!(row_file_index(&parsed, &file("b.pdf", 3, 1)), Some(1));
        // Decoded differently than the structure named it: by place.
        assert_eq!(
            row_file_index(&parsed, &file("=?utf-8?q?b?=", 0, 1)),
            Some(1)
        );
        assert_eq!(row_file_index(&parsed, &file("x", 0, 5)), None);
    }

    #[test]
    fn names_are_made_safe() {
        assert_eq!(safe_name("report.pdf"), "report.pdf");
        assert_eq!(safe_name("../../etc/passwd"), "_.._etc_passwd");
        assert_eq!(safe_name(".bashrc"), "bashrc");
        assert_eq!(safe_name("  "), "attachment");
        assert_eq!(safe_name("a\nb"), "a_b");
        assert_eq!(safe_name("report\u{202e}fdp.exe"), "reportfdp.exe");
        let long = format!("{}.pdf", "x".repeat(300));
        let short = safe_name(&long);
        assert!(short.len() <= 200 && short.ends_with(".pdf"), "{short}");
    }

    #[test]
    fn user_dirs_file() {
        let text =
            "# comment\nXDG_DESKTOP_DIR=\"$HOME/Desktop\"\nXDG_DOWNLOAD_DIR=\"$HOME/Hämtningar\"\n";
        let home = Path::new("/home/sara");
        assert_eq!(
            user_dir(text, "XDG_DOWNLOAD_DIR", home),
            Some(PathBuf::from("/home/sara/Hämtningar"))
        );
        assert_eq!(
            user_dir("XDG_DOWNLOAD_DIR=\"/data/dl\"", "XDG_DOWNLOAD_DIR", home),
            Some(PathBuf::from("/data/dl"))
        );
        assert_eq!(user_dir(text, "XDG_MUSIC_DIR", home), None);
    }

    #[test]
    fn saving_never_overwrites_in_the_fallback() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(unique_path(dir.path(), "a.pdf"), dir.path().join("a.pdf"));
        std::fs::write(dir.path().join("a.pdf"), b"1").unwrap();
        std::fs::write(dir.path().join("a (1).pdf"), b"2").unwrap();
        assert_eq!(
            unique_path(dir.path(), "a.pdf"),
            dir.path().join("a (2).pdf")
        );
    }

    #[test]
    fn saving_all_keeps_files_already_there() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.pdf"), b"old").unwrap();
        let first = save_new(dir.path(), "a.pdf", b"new").unwrap();
        let second = save_new(dir.path(), "a.pdf", b"newer").unwrap();
        let bare = save_new(dir.path(), "README", b"x").unwrap();
        assert_eq!(first, dir.path().join("a (1).pdf"));
        assert_eq!(second, dir.path().join("a (2).pdf"));
        assert_eq!(bare, dir.path().join("README"));
        assert_eq!(std::fs::read(dir.path().join("a.pdf")).unwrap(), b"old");
        assert_eq!(std::fs::read(&second).unwrap(), b"newer");
    }

    #[test]
    fn opened_files_are_read_only_and_cleaned_up() {
        let dir = tempfile::tempdir().unwrap();
        let file = AttachmentFile {
            name: "q3.pdf".into(),
            mime: "application/pdf".into(),
            bytes: b"%PDF".to_vec(),
        };
        let now = SystemTime::now();
        let first = write_for_opening(dir.path(), &file, now).unwrap();
        assert_eq!(std::fs::read(&first).unwrap(), b"%PDF");
        assert!(std::fs::metadata(&first).unwrap().permissions().readonly());
        // A day later the first copy is removed.
        let later = now + OPENED_KEEP + Duration::from_secs(60);
        let second = write_for_opening(dir.path(), &file, later).unwrap();
        assert!(!first.exists());
        assert!(second.exists());
    }
}
