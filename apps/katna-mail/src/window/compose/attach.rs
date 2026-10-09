// SPDX-License-Identifier: GPL-3.0-or-later

//! Files and pictures in the message: the attach and insert-photo pickers,
//! files dropped on the window, and the attachment chips above the bar.

use crate::widgets::Tip as _;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use gpui::{AnyElement, Context, ExternalPaths, PathPromptOptions, Window, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_render::AttachmentFile;
use katna_ui::px;

use super::super::MailWindow;
use super::super::attachments::{Item, ViewerPlace};
use super::{Kind, Mode};
use crate::format;
use crate::outgoing::Part;
use crate::theme::Theme;
use crate::widgets::{ScaledEdge, icon};
use katna_core::config::OpenIn;

/// What mail servers take in one message (Gmail's limit), counting the
/// pictures in the text.
pub(in crate::window) const MAX_TOTAL: usize = 25 * 1024 * 1024;

/// The limit as mail services name it ("25 MB"); [`format::size`] of
/// [`MAX_TOTAL`] would round its mebibytes up to 26 MB.
pub(in crate::window) fn limit_text() -> String {
    format::size(25 * 1000 * 1000)
}

/// Files that reach the app this soon after a file manager opened a new
/// message with files join that message: Explorer starts Katna Mail once
/// for each file chosen.
const JOIN: Duration = Duration::from_secs(2);

/// An attachment chip's height, and the gap between chips.
const ROW: f32 = 36.0;
const GAP: f32 = 8.0;

/// `attachments` as one message, which the viewer reads them from by
/// their place in the list.
fn attached_as_message(attachments: &[Attachment]) -> Vec<u8> {
    crate::outgoing::build(&crate::outgoing::Outgoing {
        attachments: attachments.iter().map(Attachment::part).collect(),
        ..Default::default()
    })
}

/// Where added files go.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Place {
    /// All attached (the attach picker).
    Attach,
    /// Pictures in the text (the insert-photo picker).
    Insert,
    /// Pasted or dropped: pictures in the text when `inline`, else
    /// attached, with the choice between the two under them.
    Choose { inline: bool },
}

/// A file attached to the message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::window) struct Attachment {
    pub name: String,
    pub mime: String,
    pub data: Arc<Vec<u8>>,
}

impl Attachment {
    pub fn part(&self) -> Part {
        Part {
            name: self.name.clone(),
            mime: self.mime.clone(),
            data: self.data.clone(),
            content_id: None,
        }
    }
}

/// The MIME type of a file, from its name.
pub(in crate::window) fn mime_of(name: &str) -> String {
    if let Some(image) = katna_ui::rich::image_mime(name) {
        return image.to_owned();
    }
    let ext = name
        .rsplit_once('.')
        .map(|(_, e)| e.to_ascii_lowercase())
        .unwrap_or_default();
    match ext.as_str() {
        "pdf" => "application/pdf",
        "txt" | "log" | "md" => "text/plain",
        "csv" => "text/csv",
        "html" | "htm" => "text/html",
        "ics" => "text/calendar",
        "vcf" => "text/vcard",
        "json" => "application/json",
        "xml" => "application/xml",
        "zip" => "application/zip",
        "gz" => "application/gzip",
        "tar" => "application/x-tar",
        "7z" => "application/x-7z-compressed",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "ppt" => "application/vnd.ms-powerpoint",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "odt" => "application/vnd.oasis.opendocument.text",
        "ods" => "application/vnd.oasis.opendocument.spreadsheet",
        "odp" => "application/vnd.oasis.opendocument.presentation",
        "mp3" => "audio/mpeg",
        "ogg" => "audio/ogg",
        "wav" => "audio/wav",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "mov" => "video/quicktime",
        "tif" | "tiff" => "image/tiff",
        "heic" => "image/heic",
        _ => "application/octet-stream",
    }
    .to_owned()
}

/// The icon of an attachment chip.
fn icon_of(mime: &str) -> &'static str {
    if mime.starts_with("image/") {
        "image"
    } else {
        "file"
    }
}

impl MailWindow {
    /// Asks for files to attach, or with `pictures` for photos to put in
    /// the text.
    pub(in crate::window) fn pick_files(&mut self, pictures: bool, cx: &mut Context<Self>) {
        if let Some(c) = &mut self.compose {
            c.popup = None;
        }
        let chosen = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: Some(
                if pictures {
                    tr!("compose-picker-insert")
                } else {
                    tr!("compose-picker-attach")
                }
                .into(),
            ),
        });
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(paths))) = chosen.await else {
                return;
            };
            let place = if pictures {
                Place::Insert
            } else {
                Place::Attach
            };
            this.update(cx, |this, cx| this.add_files(paths, place, cx))
                .ok();
        })
        .detach();
    }

    /// "Send with Katna Mail" in a file manager: a new message with `paths`
    /// attached (folders as zips), from the account with the address
    /// `from` when one is given. A message being written is kept in
    /// Drafts, so the files get one of their own.
    pub(in crate::window) fn open_with_files(
        &mut self,
        from: Option<String>,
        paths: Vec<PathBuf>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let account = from
            .as_deref()
            .and_then(|address| {
                self.accounts
                    .iter()
                    .find(|a| a.address.eq_ignore_ascii_case(address))
            })
            .map(|a| a.id);
        let joins = self
            .writing
            .files_opened
            .is_some_and(|at| at.elapsed() < JOIN)
            && self.compose.as_ref().is_some_and(|c| {
                c.kind == Kind::New && !c.closing && (account.is_none() || c.from == account)
            });
        if !joins {
            self.close_compose_saving(cx);
            self.open_compose(Kind::New, None, window, cx);
            if let Some(account) = account {
                self.send_compose_from(account, cx);
                self.ask_delivery_receipts(cx);
            }
        }
        self.writing.files_opened = Some(Instant::now());
        self.add_files(paths, Place::Attach, cx);
    }

    /// Attaches the files at `paths`, as the paperclip's file chooser
    /// does: those that do not fit go through the account's cloud.
    pub(in crate::window) fn attach_paths(&mut self, paths: Vec<PathBuf>, cx: &mut Context<Self>) {
        self.add_files(paths, Place::Attach, cx);
    }

    /// Reads `paths` off the main thread and adds them where `place` says:
    /// pictures may go in the text (unless it is plain text); everything
    /// else is attached.
    pub(super) fn add_files(&mut self, paths: Vec<PathBuf>, place: Place, cx: &mut Context<Self>) {
        let zips = self.paths.cache_dir().join("attach");
        let read = cx.background_executor().spawn(async move {
            let name_of = |path: &std::path::Path| {
                path.file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "attachment".to_owned())
            };
            paths
                .into_iter()
                .filter_map(|path| {
                    if path.is_file() {
                        return Some(Ok((path, false)));
                    }
                    // A folder goes as a zip of it.
                    path.is_dir().then(|| {
                        crate::folder_zip::zip_folder(&path, &zips)
                            .map(|zip| (zip, true))
                            .map_err(|err| format!("{}: {err}", name_of(&path)))
                    })
                })
                .map(|file| {
                    let (path, zipped) = match file {
                        Ok(file) => file,
                        Err(err) => return (PathBuf::new(), String::new(), 0, Some(Err(err))),
                    };
                    let name = name_of(&path);
                    let size = std::fs::metadata(&path).map_or(0, |m| m.len());
                    // Larger files go through Drive, never into memory.
                    let data = (size <= MAX_TOTAL as u64)
                        .then(|| std::fs::read(&path).map_err(|err| format!("{name}: {err}")));
                    // A zip that went into memory is not needed any more;
                    // one on its way to Drive is.
                    if zipped
                        && data.is_some()
                        && let Some(dir) = path.parent()
                    {
                        let _ = std::fs::remove_dir_all(dir);
                    }
                    (path, name, size, data)
                })
                .collect::<Vec<_>>()
        });
        // Where files that do not fit the message go: the Google Drive or
        // OneDrive of the account it goes out from, if that signs in with
        // Google or Microsoft.
        let drive_account = self.compose.as_ref().and_then(|c| {
            let id = c
                .from
                .or_else(|| self.compose_account(c.kind).map(|a| a.id))?;
            Some((id, super::drive::drive_provider(self, id)?))
        });
        cx.spawn(async move |this, cx| {
            let files = read.await;
            this.update(cx, |this, cx| {
                let Some(compose) = &mut this.compose else {
                    return;
                };
                let plain = compose.plain(cx);
                let mut total = compose.used_bytes(cx);
                let mut problem = None;
                // Pictures to place with a choice, after the rest.
                let mut chosen = Vec::new();
                let mut to_drive = Vec::new();
                for (path, name, size, data) in files {
                    if total as u64 + size > MAX_TOTAL as u64 {
                        match drive_account {
                            Some((account, _)) => to_drive.push((path, name, size, account)),
                            None => {
                                problem = Some(tr!(
                                    "compose-file-too-large",
                                    name = name,
                                    limit = limit_text()
                                ));
                            }
                        }
                        continue;
                    }
                    let data = match data {
                        Some(Ok(data)) => data,
                        Some(Err(err)) => {
                            problem = Some(err);
                            continue;
                        }
                        None => continue,
                    };
                    let mime = mime_of(&name);
                    if matches!(place, Place::Choose { .. }) && mime.starts_with("image/") {
                        chosen.push(katna_ui::rich::Picture { name, mime, data });
                        continue;
                    }
                    total += data.len();
                    if place == Place::Insert
                        && !plain
                        && katna_ui::rich::image_mime(&name).is_some()
                    {
                        let (n, m) = (name.clone(), mime.clone());
                        let inserted = compose
                            .body
                            .update(cx, |editor, cx| editor.insert_image(n, m, data.clone(), cx));
                        if inserted {
                            continue;
                        }
                    }
                    compose.attachments.push(Attachment {
                        name,
                        mime,
                        data: Arc::new(data),
                    });
                }
                // The newest file shows, however long the list.
                compose.attach_scroll.scroll_to_bottom();
                if let (Some((_, name, ..)), Some((_, provider))) =
                    (to_drive.first(), drive_account)
                {
                    let note = super::drive::drive_note(provider, name.clone(), limit_text());
                    this.show_snackbar(note, None, cx);
                }
                for (path, name, size, account) in to_drive {
                    this.upload_to_drive(path, name, size, account, cx);
                }
                if let Place::Choose { inline } = place
                    && !chosen.is_empty()
                {
                    this.place_pictures(chosen, inline, cx);
                }
                if let Some(problem) = problem {
                    this.show_snackbar(problem, None, cx);
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn remove_attachment(&mut self, ix: usize, cx: &mut Context<Self>) {
        if let Some(c) = &mut self.compose
            && ix < c.attachments.len()
        {
            c.attachments.remove(ix);
        }
        cx.notify();
    }

    /// Opens attached file `ix` where its type opens (Katna's viewer, as
    /// received files do, unless Default apps names another app), the
    /// message's other files a click of the arrows away: to check the right
    /// file is attached.
    fn open_attached(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(compose) = &self.compose else {
            return;
        };
        let Some(file) = compose.attachments.get(ix) else {
            return;
        };
        let items: Vec<Item> = compose
            .attachments
            .iter()
            .enumerate()
            .map(|(index, a)| Item {
                index,
                name: a.name.clone(),
                size: a.data.len() as u64,
                kind: katna_preview::kind(&a.mime, &a.name),
                risky: katna_preview::risky(&a.mime, &a.name),
            })
            .collect();
        let open_in = self.open_in(&items[ix]);
        if open_in != OpenIn::Katna {
            let file = AttachmentFile {
                name: file.name.clone(),
                mime: file.mime.clone(),
                bytes: file.data.to_vec(),
            };
            self.open_attachment_with(Arc::new(file), open_in == OpenIn::Ask, false, cx);
            return;
        }
        let place = if compose.mode == Mode::Window {
            ViewerPlace::Popout
        } else {
            ViewerPlace::OverCompose
        };
        // The viewer reads files from a message: the attached ones, in
        // order, make one.
        let raw = Arc::new(attached_as_message(&compose.attachments));
        self.show_viewer(raw, false, items, ix, None, window, cx);
        self.files.viewer_place = place;
    }

    /// The attached files, as chips with their size and a remove button.
    pub(in crate::window) fn render_attachments(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        if compose.attachments.is_empty() && compose.drive.is_empty() {
            return div().into_any_element();
        }
        let chips = compose.attachments.iter().enumerate().map(|(ix, a)| {
            div()
                .id(("attachment", ix))
                .h(px(ROW))
                .max_w(px(240.0))
                .pl(px(10.0))
                .pr(px(4.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .rounded(px(8.0))
                .bg(rgba(th.chip))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.chip_hover())))
                .tip(tr!("compose-attachment-open-tip"), th)
                .on_click(
                    cx.listener(move |this, _, window, cx| this.open_attached(ix, window, cx)),
                )
                .text_size(px(13.0))
                .child(icon(icon_of(&a.mime), th.accent, 18.0))
                .child(
                    div()
                        .min_w_0()
                        .truncate()
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .text_color(rgba(th.accent))
                        .child(a.name.clone()),
                )
                .child(div().flex_none().text_color(rgba(th.text_dim)).child(tr!(
                    "compose-attachment-size",
                    size = format::size(a.data.len() as u64)
                )))
                .child(
                    div()
                        .id(("attachment-remove", ix))
                        .flex_none()
                        .size(px(24.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .cursor_pointer()
                        .relative()
                        .child(crate::widgets::hover_fade("hover-glow", None, th))
                        .tip(tr!("compose-remove-attachment"), th)
                        .on_click(cx.listener(move |this, _, _, cx| {
                            cx.stop_propagation();
                            this.remove_attachment(ix, cx)
                        }))
                        .child(icon("close", th.text_dim, 16.0)),
                )
        });
        let mut chips: Vec<AnyElement> = chips.map(|chip| chip.into_any_element()).collect();
        for (ix, file) in compose.drive.iter().enumerate() {
            chips.push(self.render_drive_chip(ix, file, th, cx));
        }
        let count = compose.attachments.len() + compose.drive.len();
        let total = compose
            .attachments
            .iter()
            .map(|a| a.data.len() as u64)
            .sum::<u64>()
            + compose.drive.iter().map(|f| f.size).sum::<u64>();
        // Two rows and part of a third show; the rest scrolls, so the list
        // never reaches the Send bar however many files there are.
        let list = div()
            .id("attachments")
            .max_h(px(ROW * 2.5 + GAP * 2.0))
            .overflow_y_scroll()
            .track_scroll(&compose.attach_scroll)
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(GAP))
            .children(chips);
        div()
            .flex_none()
            .px(px(16.0))
            .py(px(4.0))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .when(count > 1, |d| {
                d.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim))
                        .child(tr!(
                            "compose-attachments-total",
                            count = count as u64,
                            size = format::size(total)
                        )),
                )
            })
            .child(list)
            .into_any_element()
    }

    /// The "Drop files here" cover: unseen until files, or text, cells or
    /// a picture from another app ("Drop here"), are dragged over the
    /// message.
    pub(in crate::window) fn render_drop_target(&self, th: &Theme) -> AnyElement {
        let (surface, accent) = (th.surface, th.accent);
        let content = |paths: &ExternalPaths| katna_ui::native::dropped_content(paths).is_some();
        // One label over the other; the drag shows the one that fits it.
        let label = |id: &'static str, text: String, for_content: bool| {
            div()
                .id(id)
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(8.0))
                .opacity(if for_content { 0.0 } else { 1.0 })
                .drag_over::<ExternalPaths>(move |s, paths, _, _| {
                    if content(paths) == for_content {
                        s.opacity(1.0)
                    } else {
                        s.opacity(0.0)
                    }
                })
                .child(icon(
                    if for_content {
                        "format-text"
                    } else {
                        "attachment"
                    },
                    accent,
                    32.0,
                ))
                .child(text)
        };
        div()
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .p(px(8.0))
            .opacity(0.0)
            // Pictures go where they are dropped in the text, shown by a
            // caret there (`drag_over_body`), so they get no cover.
            .drag_over::<ExternalPaths>(|s, paths, _, _| {
                if super::paste::pictures_only(paths) {
                    s
                } else {
                    s.opacity(1.0)
                }
            })
            .child(
                div()
                    .relative()
                    .size_full()
                    .rounded(px(12.0))
                    .border_px(2.0)
                    .border_dashed()
                    .border_color(rgba(accent))
                    .bg(rgba(crate::theme::fade(surface, 0.92)))
                    .text_color(rgba(accent))
                    .text_size(px(16.0))
                    .child(label("drop-files", tr!("compose-drop-files"), false))
                    .child(label("drop-content", tr!("compose-drop-here"), true)),
            )
            .into_any_element()
    }
}

impl MailWindow {
    /// Puts the files of the message being forwarded under it, as chips
    /// that can be taken off before sending. They are read off the main
    /// thread; the message may have changed by then.
    pub(super) fn attach_forwarded(&mut self, cx: &mut Context<Self>) {
        let Some(compose) = &self.compose else {
            return;
        };
        let source = compose.source;
        let Some(id) = source else {
            return;
        };
        let count = self
            .reader
            .as_ref()
            .and_then(|r| r.view(source))
            .map_or(0, |view| view.attachments.len());
        if count == 0 {
            return;
        }
        let Some((raw, _)) = self.attachment_raw(id) else {
            self.show_snackbar(tr!("compose-forward-files-missing"), None, cx);
            return;
        };
        cx.spawn(async move |this, cx| {
            let files: Vec<_> = cx
                .background_executor()
                .spawn(async move {
                    (0..count)
                        .filter_map(|index| katna_render::attachment_file(&raw, index))
                        .collect()
                })
                .await;
            this.update(cx, |this, cx| this.add_forwarded(source, files, cx))
                .ok();
        })
        .detach();
    }

    fn add_forwarded(
        &mut self,
        source: Option<katna_store::MessageId>,
        files: Vec<katna_render::AttachmentFile>,
        cx: &mut Context<Self>,
    ) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        if compose.kind != Kind::Forward || compose.source != source {
            return;
        }
        let mut used = compose.used_bytes(cx);
        let mut left_out = None;
        for file in files {
            if used + file.bytes.len() > MAX_TOTAL {
                left_out.get_or_insert(file.name);
                continue;
            }
            used += file.bytes.len();
            let data = Arc::new(file.bytes);
            compose.forwarded.push(data.clone());
            compose.attachments.push(Attachment {
                name: file.name,
                mime: file.mime,
                data,
            });
        }
        compose.attach_scroll.scroll_to_bottom();
        if let Some(name) = left_out {
            let problem = tr!("compose-file-too-large", name = name, limit = limit_text());
            self.show_snackbar(problem, None, cx);
        }
        cx.notify();
    }
}

/// Which of files `sizes` go through the cloud so the rest fit with the
/// `used` bytes under `limit`: the biggest first, as few as need to.
pub(in crate::window) fn cloud_bound(sizes: &[u64], used: u64, limit: u64) -> Vec<bool> {
    let mut bound = vec![false; sizes.len()];
    let mut total = used + sizes.iter().sum::<u64>();
    let mut order: Vec<usize> = (0..sizes.len()).collect();
    order.sort_by_key(|&ix| std::cmp::Reverse(sizes[ix]));
    for ix in order {
        if total <= limit {
            break;
        }
        bound[ix] = true;
        total -= sizes[ix];
    }
    bound
}

impl MailWindow {
    /// What the message being written carries against the 25 MB limit,
    /// and the cloud larger files would go to (Google Drive or OneDrive
    /// of the account it goes out from).
    pub(in crate::window) fn compose_room(
        &self,
        cx: &gpui::App,
    ) -> (u64, Option<katna_core::OAuthProvider>) {
        let Some(compose) = &self.compose else {
            return (0, None);
        };
        (
            compose.used_bytes(cx) as u64,
            self.compose_cloud().map(|(_, p)| p),
        )
    }

    fn compose_cloud(&self) -> Option<(katna_core::AccountId, katna_core::OAuthProvider)> {
        let compose = self.compose.as_ref()?;
        let id = compose
            .from
            .or_else(|| self.compose_account(compose.kind).map(|a| a.id))?;
        Some((id, super::drive::drive_provider(self, id)?))
    }

    /// Attaches files picked on the Files page (each with whether its mail
    /// was decrypted). Those that do not fit go through the account's
    /// cloud, biggest first; with none, they are left out. `failed` files
    /// could not be read.
    pub(in crate::window) fn attach_picked(
        &mut self,
        files: Vec<(katna_render::AttachmentFile, bool)>,
        failed: usize,
        cx: &mut Context<Self>,
    ) {
        let cloud = self.compose_cloud();
        let cache = self.paths.cache_dir().join("attach");
        let Some(compose) = &mut self.compose else {
            return;
        };
        let used = compose.used_bytes(cx) as u64;
        let sizes: Vec<u64> = files.iter().map(|(f, _)| f.bytes.len() as u64).collect();
        let bound = cloud_bound(&sizes, used, MAX_TOTAL as u64);
        let mut left_out = None;
        let mut to_cloud = Vec::new();
        for ((file, encrypted), bound) in files.into_iter().zip(bound) {
            if !bound {
                compose.attachments.push(Attachment {
                    name: file.name,
                    mime: file.mime,
                    data: Arc::new(file.bytes),
                });
                continue;
            }
            // A decrypted file never reaches the disk unasked.
            match cloud {
                Some((account, _)) if !encrypted => to_cloud.push((file, account)),
                _ => {
                    left_out.get_or_insert(file.name);
                }
            }
        }
        compose.attach_scroll.scroll_to_bottom();
        for (file, account) in to_cloud {
            let dir = cache.join(format!("picked-{}", jiff::Timestamp::now().as_nanosecond()));
            let path = dir.join(&file.name);
            let size = file.bytes.len() as u64;
            let written =
                std::fs::create_dir_all(&dir).and_then(|()| std::fs::write(&path, &file.bytes));
            match written {
                Ok(()) => self.upload_to_drive(path, file.name, size, account, cx),
                Err(err) => {
                    let problem = format!("{}: {err}", file.name);
                    self.show_snackbar(problem, None, cx);
                }
            }
        }
        if let Some(name) = left_out {
            let problem = tr!("compose-file-too-large", name = name, limit = limit_text());
            self.show_snackbar(problem, None, cx);
        } else if failed > 0 {
            self.show_snackbar(tr!("picker-some-failed", count = failed), None, cx);
        }
        cx.notify();
    }
}

impl super::Compose {
    /// Takes off the files a forward brought along, when the message
    /// becomes a reply.
    pub(super) fn drop_forwarded(&mut self) {
        let forwarded = std::mem::take(&mut self.forwarded);
        self.attachments
            .retain(|a| !forwarded.iter().any(|f| Arc::ptr_eq(f, &a.data)));
    }

    /// The files are other than the ones a forward brought along.
    pub(super) fn files_changed(&self) -> bool {
        self.attachments.len() != self.forwarded.len()
            || self
                .attachments
                .iter()
                .zip(&self.forwarded)
                .any(|(a, f)| !Arc::ptr_eq(&a.data, f))
    }
}

impl super::Compose {
    /// Bytes of attachments and pictures in the message.
    pub(super) fn used_bytes(&self, cx: &gpui::App) -> usize {
        self.attachments.iter().map(|a| a.data.len()).sum::<usize>()
            + self
                .body
                .read(cx)
                .doc()
                .images()
                .map(|i| i.data.len())
                .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_biggest_files_go_to_the_cloud() {
        const MB: u64 = 1024 * 1024;
        let limit = 25 * MB;
        assert_eq!(cloud_bound(&[MB, 2 * MB], 0, limit), [false, false]);
        assert_eq!(
            cloud_bound(&[5 * MB, 31 * MB, 10 * MB, 4 * MB], 0, limit),
            [false, true, false, false]
        );
        assert_eq!(
            cloud_bound(&[20 * MB, 12 * MB], 3 * MB, limit),
            [true, false]
        );
    }

    #[test]
    fn the_viewer_finds_each_attached_file_by_its_place() {
        let file = |name: &str, data: &[u8]| Attachment {
            name: name.to_owned(),
            mime: mime_of(name),
            data: Arc::new(data.to_vec()),
        };
        let files = [
            file("Report.pdf", b"%PDF-1.4"),
            file("notes.txt", b"hello"),
            file("photo.png", b"\x89PNG"),
        ];
        let raw = attached_as_message(&files);
        for (ix, f) in files.iter().enumerate() {
            let read = katna_render::attachment_file(&raw, ix).expect("attached file");
            assert_eq!(read.name, f.name);
            assert_eq!(read.bytes, *f.data);
        }
    }

    #[test]
    fn guesses_types() {
        assert_eq!(mime_of("Report.PDF"), "application/pdf");
        assert_eq!(mime_of("a.jpeg"), "image/jpeg");
        assert_eq!(mime_of("notes"), "application/octet-stream");
    }
}
