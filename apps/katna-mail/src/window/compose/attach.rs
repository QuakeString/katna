// SPDX-License-Identifier: GPL-3.0-or-later

//! Files and pictures in the message: the attach and insert-photo pickers,
//! files dropped on the window, and the attachment chips above the bar.

use std::path::PathBuf;
use std::sync::Arc;

use gpui::{AnyElement, Context, ExternalPaths, PathPromptOptions, div, prelude::*, px, rgba};

use super::super::MailWindow;
use crate::format;
use crate::outgoing::Part;
use crate::theme::Theme;
use crate::widgets::{icon, tip};

/// What mail servers take in one message (Gmail's limit), counting the
/// pictures in the text.
pub(in crate::window) const MAX_TOTAL: usize = 25 * 1024 * 1024;

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
pub(super) fn mime_of(name: &str) -> String {
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
            prompt: Some(if pictures { "Insert" } else { "Attach" }.into()),
        });
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(paths))) = chosen.await else {
                return;
            };
            this.update(cx, |this, cx| this.add_files(paths, pictures, cx))
                .ok();
        })
        .detach();
    }

    /// Files dropped on the message: pictures go in the text, other files
    /// are attached.
    pub(in crate::window) fn drop_files(&mut self, paths: Vec<PathBuf>, cx: &mut Context<Self>) {
        self.add_files(paths, true, cx);
    }

    /// Reads `paths` off the main thread and adds them. With `pictures`,
    /// images go in the text (unless it is plain text); everything else is
    /// attached.
    fn add_files(&mut self, paths: Vec<PathBuf>, pictures: bool, cx: &mut Context<Self>) {
        let read = cx.background_executor().spawn(async move {
            paths
                .into_iter()
                .filter(|p| p.is_file())
                .map(|path| {
                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "attachment".to_owned());
                    let data = std::fs::read(&path).map_err(|err| format!("{name}: {err}"));
                    (name, data)
                })
                .collect::<Vec<_>>()
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
                for (name, data) in files {
                    let data = match data {
                        Ok(data) => data,
                        Err(err) => {
                            problem = Some(err);
                            continue;
                        }
                    };
                    if total + data.len() > MAX_TOTAL {
                        problem = Some(format!(
                            "{name} is too large: a message can carry up to {}.",
                            format::size(MAX_TOTAL as u64)
                        ));
                        continue;
                    }
                    total += data.len();
                    let mime = mime_of(&name);
                    if pictures && !plain && katna_ui::rich::image_mime(&name).is_some() {
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

    /// The attached files, as chips with their size and a remove button.
    pub(in crate::window) fn render_attachments(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        if compose.attachments.is_empty() {
            return div().into_any_element();
        }
        let chips = compose.attachments.iter().enumerate().map(|(ix, a)| {
            div()
                .id(("attachment", ix))
                .h(px(36.0))
                .max_w(px(240.0))
                .pl(px(10.0))
                .pr(px(4.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .rounded(px(8.0))
                .bg(rgba(th.chip))
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
                .child(
                    div()
                        .flex_none()
                        .text_color(rgba(th.text_dim))
                        .child(format!("({})", format::size(a.data.len() as u64))),
                )
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
                        .hover(|s| s.bg(rgba(th.hover)))
                        .tooltip(tip("Remove attachment", th))
                        .on_click(cx.listener(move |this, _, _, cx| this.remove_attachment(ix, cx)))
                        .child(icon("close", th.text_dim, 16.0)),
                )
        });
        div()
            .flex_none()
            .max_h(px(96.0))
            .px(px(16.0))
            .py(px(4.0))
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(8.0))
            .children(chips)
            .into_any_element()
    }

    /// The "Drop files here" cover: unseen until files are dragged over
    /// the message.
    pub(in crate::window) fn render_drop_target(&self, th: &Theme) -> AnyElement {
        let (surface, accent) = (th.surface, th.accent);
        div()
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .p(px(8.0))
            .opacity(0.0)
            .drag_over::<ExternalPaths>(|s, _, _, _| s.opacity(1.0))
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap(px(8.0))
                    .rounded(px(12.0))
                    .border_2()
                    .border_dashed()
                    .border_color(rgba(accent))
                    .bg(rgba(crate::theme::fade(surface, 0.92)))
                    .text_color(rgba(accent))
                    .text_size(px(16.0))
                    .child(icon("attachment", accent, 32.0))
                    .child("Drop files here"),
            )
            .into_any_element()
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
    fn guesses_types() {
        assert_eq!(mime_of("Report.PDF"), "application/pdf");
        assert_eq!(mime_of("a.jpeg"), "image/jpeg");
        assert_eq!(mime_of("notes"), "application/octet-stream");
    }
}
