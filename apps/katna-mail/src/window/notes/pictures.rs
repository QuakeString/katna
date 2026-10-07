// SPDX-License-Identifier: GPL-3.0-or-later

//! Pictures in notes, as Keep has them: picked, pasted or dropped, they
//! sit on top of the note and across the top edge of its card, and open
//! in Katna's viewer. They are kept apart from the text and travel in the
//! note's Notes-folder message as inline parts (`katna-sync`).

use std::sync::Arc;

use gpui::{AnyElement, Context, ObjectFit, SharedString, Window, div, img, prelude::*, rgba};
use katna_dbus::NotePictureItem;
use katna_i18n::tr;
use katna_store::NotePicture;
use katna_ui::px;
use katna_ui::rich::{Block, image_mime};
use katna_ui::tokens::radius;

use super::MailWindow;
use crate::theme::Theme;
use crate::widgets::{icon_button_colored, tip};

/// The biggest picture taken, in bytes (a phone photo; the whole note
/// must fit in one message).
const MAX_PICTURE: usize = 10_000_000;
/// The tallest a card's top picture draws.
pub(super) const COVER_MAX: f32 = 180.0;

/// A picture's name in the note's HTML: the same picture always gets the
/// same one (64-bit FNV-1a of its bytes).
pub(super) fn cid_of(data: &[u8]) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in data {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{hash:016x}@katna")
}

/// `picture` as the daemon takes it.
pub(super) fn item_of_picture(picture: &NotePicture) -> NotePictureItem {
    NotePictureItem {
        cid: picture.cid.clone(),
        name: picture.name.clone(),
        mime: picture.mime.clone(),
        width: picture.width,
        height: picture.height,
        data: picture.data.clone(),
    }
}

/// A picture of `data`, named `name`, if it is one the app can show.
fn picture(name: &str, mime: Option<&str>, data: Vec<u8>) -> Option<NotePictureItem> {
    let mime = mime.or_else(|| image_mime(name))?.to_owned();
    let (width, height) = katna_ui::rich::image_size(&data).unwrap_or((0, 0));
    Some(NotePictureItem {
        cid: cid_of(&data),
        name: name.to_owned(),
        mime,
        width,
        height,
        data,
    })
}

/// A decoded picture for drawing.
pub(super) fn decoded(mime: &str, data: &[u8]) -> Option<Arc<gpui::Image>> {
    let format = match mime {
        "image/png" => gpui::ImageFormat::Png,
        "image/jpeg" | "image/jpg" => gpui::ImageFormat::Jpeg,
        "image/gif" => gpui::ImageFormat::Gif,
        "image/webp" => gpui::ImageFormat::Webp,
        "image/bmp" => gpui::ImageFormat::Bmp,
        "image/svg+xml" => gpui::ImageFormat::Svg,
        _ => return None,
    };
    Some(Arc::new(gpui::Image::from_bytes(format, data.to_vec())))
}

/// How tall a picture `width` × `height` draws `room` wide, at most
/// `max`.
pub(super) fn drawn_height(width: u32, height: u32, room: f32, max: f32) -> f32 {
    if width == 0 || height == 0 {
        return max.min(room * 0.6);
    }
    (room * height as f32 / width as f32).min(max)
}

impl MailWindow {
    /// The open note's pictures, decoded for drawing.
    fn note_picture_images(&self) -> Vec<(usize, Arc<gpui::Image>, f32)> {
        let Some(editor) = self.notes.as_ref().and_then(|p| p.editor.as_ref()) else {
            return Vec::new();
        };
        editor
            .pictures
            .iter()
            .enumerate()
            .filter_map(|(ix, p)| {
                let image = editor.decoded.get(&p.cid).cloned()?;
                Some((ix, image, p.width as f32 / p.height.max(1) as f32))
            })
            .collect()
    }

    /// Adds `picture` to the open note.
    fn add_note_picture(&mut self, picture: NotePictureItem, cx: &mut Context<Self>) {
        let Some(editor) = self.notes.as_mut().and_then(|p| p.editor.as_mut()) else {
            return;
        };
        if editor.pictures.iter().any(|p| p.cid == picture.cid) {
            return;
        }
        if let Some(image) = decoded(&picture.mime, &picture.data) {
            editor.decoded.insert(picture.cid.clone(), image);
        }
        editor.pictures.push(picture);
        editor.pictures_set = true;
        self.note_typed(cx);
    }

    /// Takes picture `ix` off the open note, with Undo in the snackbar.
    fn remove_note_picture(&mut self, ix: usize, cx: &mut Context<Self>) {
        let Some(editor) = self.notes.as_mut().and_then(|p| p.editor.as_mut()) else {
            return;
        };
        if ix >= editor.pictures.len() {
            return;
        }
        editor.pictures.remove(ix);
        editor.pictures_set = true;
        self.note_typed(cx);
    }

    /// The picture button: picks pictures for the open note.
    pub(super) fn pick_note_pictures(&mut self, cx: &mut Context<Self>) {
        let chosen = cx.prompt_for_paths(gpui::PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: Some(tr!("notes-picture-choose").into()),
        });
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(paths))) = chosen.await else {
                return;
            };
            let read = cx
                .background_executor()
                .spawn(async move {
                    paths
                        .into_iter()
                        .map(|path| {
                            let name = path
                                .file_name()
                                .map(|n| n.to_string_lossy().into_owned())
                                .unwrap_or_default();
                            let data = std::fs::read(&path);
                            (name, data)
                        })
                        .collect::<Vec<_>>()
                })
                .await;
            this.update(cx, |this, cx| {
                for (name, data) in read {
                    match data {
                        Ok(data) if data.len() > MAX_PICTURE => this.show_snackbar(
                            tr!(
                                "notes-picture-too-big",
                                size = crate::format::size(MAX_PICTURE as u64)
                            ),
                            None,
                            cx,
                        ),
                        Ok(data) => match picture(&name, None, data) {
                            Some(picture) => this.add_note_picture(picture, cx),
                            None => this.show_snackbar(tr!("notes-picture-kind"), None, cx),
                        },
                        Err(err) => this.show_snackbar(
                            tr!(
                                "notes-picture-unreadable",
                                name = name.as_str(),
                                error = err.to_string()
                            ),
                            None,
                            cx,
                        ),
                    }
                }
            })
            .ok();
        })
        .detach();
    }

    /// Pictures pasted or dropped into the open note's text go on top of
    /// the note, as Keep keeps them. Returns whether any did.
    pub(super) fn lift_note_pictures(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(editor) = self.notes.as_ref().and_then(|p| p.editor.as_ref()) else {
            return false;
        };
        let body = editor.body.clone();
        let mut lifted = Vec::new();
        body.update(cx, |area, cx| {
            if !area
                .doc()
                .blocks
                .iter()
                .any(|b| matches!(b, Block::Image(_)))
            {
                return;
            }
            area.edit_doc(
                |doc| {
                    doc.blocks.retain(|block| match block {
                        Block::Image(image) => {
                            lifted.push((
                                image.name.clone(),
                                image.mime.clone(),
                                image.data.to_vec(),
                            ));
                            false
                        }
                        _ => true,
                    });
                    if doc.blocks.is_empty() {
                        doc.blocks
                            .push(Block::Para(katna_ui::rich::Para::plain("")));
                    }
                },
                cx,
            );
        });
        let any = !lifted.is_empty();
        for (name, mime, data) in lifted {
            if let Some(picture) = picture(&name, Some(&mime), data) {
                self.add_note_picture(picture, cx);
            }
        }
        any
    }

    /// Opens picture `ix` of the open note in the viewer, beside the
    /// others.
    fn view_note_picture(&mut self, ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(editor) = self.notes.as_ref().and_then(|p| p.editor.as_ref()) else {
            return;
        };
        let pictures = editor.pictures.clone();
        self.view_pictures(&pictures, ix, window, cx);
    }

    /// Opens `pictures` in the viewer at `ix`.
    pub(super) fn view_pictures(
        &mut self,
        pictures: &[NotePictureItem],
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if pictures.is_empty() {
            return;
        }
        let items = pictures
            .iter()
            .enumerate()
            .map(|(index, p)| super::super::attachments::Item {
                index,
                name: p.name.clone(),
                size: p.data.len() as u64,
                kind: katna_preview::kind(&p.mime, &p.name),
                risky: false,
            })
            .collect();
        let raw = crate::outgoing::build(&crate::outgoing::Outgoing {
            attachments: pictures
                .iter()
                .map(|p| crate::outgoing::Part {
                    name: p.name.clone(),
                    mime: p.mime.clone(),
                    data: Arc::new(p.data.clone()),
                    content_id: None,
                })
                .collect(),
            ..Default::default()
        });
        self.show_viewer(Arc::new(raw), false, items, ix, None, window, cx);
    }

    /// The open note's pictures, across its top: one fills the width,
    /// more share rows of up to three.
    pub(super) fn render_note_pictures(
        &self,
        width: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let images = self.note_picture_images();
        if images.is_empty() {
            return None;
        }
        let per_row = match images.len() {
            1 => 1,
            2 | 4 => 2,
            _ => 3,
        };
        let cell = width / per_row as f32;
        let rows = images.chunks(per_row).enumerate().map(|(r, row)| {
            // A row is as tall as its shortest picture would be, so they
            // line up; each fills its cell.
            let height = row
                .iter()
                .map(|(_, _, ratio)| {
                    (cell / ratio.max(0.1)).min(if per_row == 1 { 260.0 } else { 180.0 })
                })
                .fold(f32::MAX, f32::min);
            div()
                .id(("note-picture-row", r))
                .flex()
                .flex_row()
                .h(px(height))
                .children(row.iter().enumerate().map(|(c, (ix, image, _))| {
                    let ix = *ix;
                    // GPUI does not clip to the card's corners: the top
                    // row rounds its own.
                    let (left, right) = (r == 0 && c == 0, r == 0 && c + 1 == per_row);
                    let corner = px(15.0);
                    let group: SharedString = format!("note-picture-{ix}").into();
                    div()
                        .id(("note-picture", ix))
                        .group(group.clone())
                        .relative()
                        .flex_1()
                        .h_full()
                        .overflow_hidden()
                        .cursor_pointer()
                        .on_click(cx.listener(move |this, _, window, cx| {
                            cx.stop_propagation();
                            this.view_note_picture(ix, window, cx)
                        }))
                        .when(left, |d| d.rounded_tl(corner))
                        .when(right, |d| d.rounded_tr(corner))
                        .child(
                            img(image.clone())
                                .size_full()
                                .object_fit(ObjectFit::Cover)
                                .when(left, |d| d.rounded_tl(corner))
                                .when(right, |d| d.rounded_tr(corner)),
                        )
                        .child(
                            div()
                                .absolute()
                                .bottom(px(6.0))
                                .right(px(6.0))
                                .rounded(px(radius::SM))
                                .bg(rgba(fade_dark(th)))
                                .opacity(0.0)
                                .group_hover(group, |s| s.opacity(1.0))
                                .child(
                                    icon_button_colored(
                                        ("note-picture-remove", ix),
                                        "trash",
                                        18.0,
                                        0xffff_ffff,
                                        th,
                                    )
                                    .size(px(32.0))
                                    .tooltip(tip(tr!("notes-picture-remove"), th))
                                    .on_click(cx.listener(
                                        move |this, _, _, cx| {
                                            cx.stop_propagation();
                                            this.remove_note_picture(ix, cx)
                                        },
                                    )),
                                ),
                        )
                }))
        });
        Some(
            div()
                .flex_none()
                .w_full()
                .flex()
                .flex_col()
                .rounded_t(px(radius::LG))
                .overflow_hidden()
                .children(rows)
                .into_any_element(),
        )
    }

    /// The first picture of note `id` across the top of its card.
    pub(super) fn render_card_cover(&self, id: i64, width: f32) -> Option<AnyElement> {
        let page = self.notes.as_ref()?;
        let (image, w, h) = page.covers.get(&id)?;
        let height = drawn_height(*w, *h, width, COVER_MAX);
        Some(
            div()
                .w_full()
                .h(px(height))
                .rounded_t(px(radius::SM - 1.0))
                .overflow_hidden()
                .child(img(image.clone()).size_full().object_fit(ObjectFit::Cover))
                .into_any_element(),
        )
    }
}

/// A dark backdrop for a button over a picture.
fn fade_dark(_: &Theme) -> u32 {
    0x0000_0080
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_same_picture_gets_the_same_name() {
        assert_eq!(cid_of(b"abc"), cid_of(b"abc"));
        assert_ne!(cid_of(b"abc"), cid_of(b"abd"));
        assert!(cid_of(b"").ends_with("@katna"));
    }

    #[test]
    fn a_picture_draws_at_its_shape_up_to_a_height() {
        assert_eq!(drawn_height(400, 200, 240.0, 180.0), 120.0);
        assert_eq!(drawn_height(200, 400, 240.0, 180.0), 180.0);
    }
}
