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
    AnyElement, Context, Entity, FocusHandle, FontWeight, ImageSource, ObjectFit, RenderImage,
    SharedString, Subscription, Task, Window, div, img, prelude::*, px, rgba,
};
use katna_core::config::{FileGroup, OpenIn};
use katna_preview::Kind;
use katna_preview::image::{Frame, RgbaImage, imageops};
use katna_render::{Attachment, AttachmentFile};
use katna_store::MessageId;

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
const CARD_RADIUS: f32 = 8.0;
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
}

impl Item {
    fn new(index: usize, attachment: &Attachment) -> Self {
        Self {
            index,
            name: attachment.name.clone(),
            size: attachment.size,
            kind: katna_preview::kind(&attachment.mime, &attachment.name),
        }
    }
}

/// A card's thumbnail, and a blurred copy of it shown like frosted glass
/// behind the name and Save button when the pointer is over the card.
#[derive(Clone)]
struct Thumb {
    sharp: Arc<RenderImage>,
    frosted: Arc<RenderImage>,
}

/// The window's attachment state.
#[derive(Default)]
pub(super) struct Files {
    thumbs: HashMap<(MessageId, usize), Thumb>,
    /// Messages whose thumbnails were made or are being made.
    asked: HashMap<MessageId, Option<Task<()>>>,
    /// Bitmaps no longer drawn, freed at the next frame.
    released: Vec<Arc<RenderImage>>,
    pub(super) viewer: Option<Entity<Viewer>>,
    /// What had the keyboard before the viewer opened.
    restore: Option<FocusHandle>,
    /// The viewer shows the attachments of a decrypted message.
    viewer_encrypted: bool,
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
                self.released.extend([thumb.sharp, thumb.frosted]);
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
fn row_file_index(attachments: &[Attachment], file: &RowFile) -> Option<usize> {
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
fn has_thumbnail(kind: Kind) -> bool {
    match kind {
        Kind::Pdf => true,
        Kind::Picture(picture) => picture.decodable(),
        Kind::Text | Kind::Sheet { .. } | Kind::Document | Kind::Other => false,
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
        Kind::Document => Some(FileGroup::Documents),
        Kind::Other => None,
    }
}

/// The thumbnail of attachment `index` of `raw`.
fn thumbnail(raw: &[u8], index: usize, kind: Kind) -> Option<RgbaImage> {
    let file = katna_render::attachment_file(raw, index)?;
    let (w, h) = THUMB_PIXELS;
    match kind {
        Kind::Pdf => katna_preview::pdf::thumbnail(file.bytes, w, h),
        Kind::Picture(picture) => {
            katna_preview::picture::thumbnail(&file.bytes, picture, w, h).ok()
        }
        Kind::Text | Kind::Sheet { .. } | Kind::Document | Kind::Other => None,
    }
}

impl MailWindow {
    /// Makes the thumbnails of the open messages' attachments in the
    /// background, and forgets those of messages no longer open.
    pub(super) fn request_thumbnails(&mut self, cx: &mut Context<Self>) {
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
                            .filter_map(|(ix, kind)| {
                                let sharp = thumbnail(&raw, ix, kind)?;
                                let frosted = bitmap(frosted(&sharp));
                                let sharp = bitmap(sharp);
                                Some((ix, Thumb { sharp, frosted }))
                            })
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
                            .extend(thumbs.into_iter().flat_map(|(_, t)| [t.sharp, t.frosted]));
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
        let cards = list.iter().map(|&(ix, attachment)| {
            let item = Item::new(ix, attachment);
            let group = SharedString::from(format!("attachment-{}-{ix}", id.0));
            let thumb = self.files.thumbs.get(&(id, ix)).cloned();
            let name = item.name.clone();
            let inner = px(CARD_RADIUS - 1.0);
            let frost = thumb.as_ref().map(|t| t.frosted.clone());
            let top = match thumb {
                Some(thumb) => div().size_full().child(
                    img(ImageSource::Render(thumb.sharp))
                        .size_full()
                        .rounded_t(inner)
                        .object_fit(ObjectFit::Cover),
                ),
                None => div()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(rgba(th.read_row))
                    .child(kind_badge(item.kind, 36.0)),
            };
            let save_name = name.clone();
            // Frosted glass in the theme's own color: the blurred thumbnail
            // under a veil of the card's surface, text in the theme's ink.
            // Without a thumbnail there is nothing to blur, so the veil
            // is thicker and hides the file-type badge under it.
            let veil = (th.surface & 0xffff_ff00) | if frost.is_some() { 0xa6 } else { 0xf0 };
            let (ink, ink_dim, button, button_hover) = if th.dark {
                (0xffffffff, 0xffffffcc, 0xffffff26, 0xffffff4d)
            } else {
                (th.text, th.text_dim, 0x0000001a, 0x00000033)
            };
            let overlay = div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                // Shown on hover. Not `hidden()`: GPUI cannot switch
                // `display` on hover between layout and paint.
                .opacity(0.0)
                .group_hover(group.clone(), |s| s.opacity(1.0))
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
                        .child(name.clone()),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgba(ink_dim))
                                .child(format::size(item.size)),
                        )
                        .child(
                            div()
                                .id(("attachment-save", ix))
                                .size(px(32.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded_full()
                                .bg(rgba(button))
                                .hover(move |s| s.bg(rgba(button_hover)))
                                .tooltip(tip("Save", th))
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    cx.stop_propagation();
                                    this.save_from_message(id, ix, &save_name, cx);
                                }))
                                .child(icon("download", ink, 18.0)),
                        ),
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
                        .text_size(px(13.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(th.text_dim))
                        .child(if count == 1 {
                            "One attachment".to_owned()
                        } else {
                            format!("{count} attachments")
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
            .and_then(|item| group(item.kind))
            .map_or(OpenIn::Katna, |g| self.config.mail.open.get(g));
        if open_in != OpenIn::Katna {
            let name = items.get(index).map(|i| i.name.clone()).unwrap_or_default();
            self.open_elsewhere(id, index, &name, open_in == OpenIn::Ask, cx);
            return;
        }
        let Some((raw, encrypted)) = self.attachment_raw(id) else {
            self.show_snackbar("This message is not downloaded.", None, cx);
            return;
        };
        self.show_viewer(raw, encrypted, items, index, window, cx);
    }

    /// Opens an attachment chip of the message list in the viewer, with
    /// the message's other attachments a click of the arrows away.
    pub(super) fn open_row_file(
        &mut self,
        file: &RowFile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.files_menu = None;
        let Some((raw, encrypted)) = self.attachment_raw(file.message) else {
            self.show_snackbar("This message has not been downloaded yet.", None, cx);
            return;
        };
        let view = katna_render::message_view(&raw);
        let Some(index) = row_file_index(&view.attachments, file) else {
            self.show_snackbar(
                "This attachment could not be found in the message.",
                None,
                cx,
            );
            return;
        };
        let items: Vec<Item> = view
            .attachments
            .iter()
            .enumerate()
            .map(|(ix, a)| Item::new(ix, a))
            .collect();
        let open_in =
            group(items[index].kind).map_or(OpenIn::Katna, |g| self.config.mail.open.get(g));
        if open_in != OpenIn::Katna {
            let name = items[index].name.clone();
            self.open_elsewhere(file.message, index, &name, open_in == OpenIn::Ask, cx);
            return;
        }
        self.show_viewer(raw, encrypted, items, index, window, cx);
    }

    fn show_viewer(
        &mut self,
        raw: Arc<Vec<u8>>,
        encrypted: bool,
        items: Vec<Item>,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.close_viewer(window, cx);
        self.files.restore = window.focused(cx);
        self.files.viewer_encrypted = encrypted;
        let th = self.theme(window);
        let viewer = cx.new(|cx| Viewer::new(raw, items, index, th, window, cx));
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
        }
    }

    pub(super) fn close_viewer(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(viewer) = self.files.viewer.take() else {
            return;
        };
        let released = viewer.update(cx, |viewer, _| viewer.take_released(true));
        self.files.released.extend(released);
        self.files._viewer_events = None;
        match self.files.restore.take() {
            Some(focus) => focus.focus(window, cx),
            None => self.list_focus.focus(window, cx),
        }
        cx.notify();
    }

    /// Saves attachment `index` of message `id`.
    fn save_from_message(
        &mut self,
        id: MessageId,
        index: usize,
        name: &str,
        cx: &mut Context<Self>,
    ) {
        let Some((raw, _)) = self.attachment_raw(id) else {
            self.show_snackbar("This message is not downloaded.", None, cx);
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
                None => this.show_snackbar(format!("Could not read {name}"), None, cx),
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
                    Ok(path) => format!("Saved to {}", path.display()),
                    Err(err) => format!("Could not save {}: {err}", file.name),
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
            self.show_snackbar("This message is not downloaded.", None, cx);
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
                None => this.show_snackbar(format!("Could not read {name}"), None, cx),
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
    fn open_attachment_with(
        &mut self,
        file: Arc<AttachmentFile>,
        ask: bool,
        encrypted: bool,
        cx: &mut Context<Self>,
    ) {
        if katna_preview::risky(&file.mime, &file.name) {
            self.show_snackbar(
                "This file could run a program, so Katna does not open it. Save it instead.",
                None,
                cx,
            );
            return;
        }
        let dir = if encrypted {
            match memory_dir() {
                Some(dir) => dir,
                None => {
                    self.show_snackbar(
                        "This file came encrypted. Save it to open it elsewhere.",
                        None,
                        cx,
                    );
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
                    this.update(cx, |this, cx| {
                        this.show_snackbar(format!("Could not open {name}: {err}"), None, cx)
                    })
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
    fn attachment_raw(&self, id: MessageId) -> Option<(Arc<Vec<u8>>, bool)> {
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

/// `dir/name`, or `dir/name (2)` and so on when that exists.
fn unique_path(dir: &Path, name: &str) -> PathBuf {
    let path = dir.join(name);
    if !path.exists() {
        return path;
    }
    let (stem, ext) = match name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() => (stem, format!(".{ext}")),
        _ => (name, String::new()),
    };
    (2..)
        .map(|n| dir.join(format!("{stem} ({n}){ext}")))
        .find(|p| !p.exists())
        .unwrap_or(path)
}

/// The user's download folder (`XDG_DOWNLOAD_DIR` of `user-dirs.dirs`),
/// else `~/Downloads`, else home.
fn download_dir() -> PathBuf {
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
        std::fs::write(dir.path().join("a (2).pdf"), b"2").unwrap();
        assert_eq!(
            unique_path(dir.path(), "a.pdf"),
            dir.path().join("a (3).pdf")
        );
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
