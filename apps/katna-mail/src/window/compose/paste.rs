// SPDX-License-Identifier: GPL-3.0-or-later

//! Pasting and dropping into the message, as a desktop mail app does:
//! files copied or dragged from the file manager are attached, pictures go
//! in the text or are attached (with a choice under them), and text and
//! cells from other apps keep their formatting (the editor's paste
//! options: a table as a table, a picture or plain text).

use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use gpui::{Context, DragMoveEvent, ExternalPaths, Focusable, SharedString, Window};
use katna_i18n::tr;
use katna_preview::table;
use katna_ui::rich::{Align, Block, PasteOption, Picture, RichEditor, Transfer};

use super::super::MailWindow;
use super::attach::{Attachment, MAX_TOTAL, Place, limit_text};

/// Pictures just pasted or dropped, in the text or attached, while the
/// choice between the two shows.
pub(super) struct PictureChoice {
    /// In the text: their ids in the editor.
    inline: Vec<u64>,
    /// Attached: the attachments (the same data as in the message).
    attached: Vec<Attachment>,
}

/// Gives a new editor the paste options' labels and the table pictures.
pub(super) fn setup(editor: &mut RichEditor) {
    editor.set_paste_labels(Rc::new(|option| {
        SharedString::from(match option {
            PasteOption::KeepFormatting => tr!("compose-paste-keep-formatting"),
            PasteOption::Table => tr!("compose-paste-table"),
            PasteOption::Picture => tr!("compose-paste-picture"),
            PasteOption::PlainText => tr!("compose-paste-plain-text"),
            PasteOption::Inline => tr!("compose-paste-inline"),
            PasteOption::Attachment => tr!("compose-paste-attachment"),
        })
    }));
    editor.set_table_picture(Rc::new(table_picture));
}

/// The first table in `blocks` as a picture.
fn table_picture(blocks: &[Block]) -> Option<Picture> {
    let table = blocks.iter().find_map(|b| match b {
        Block::Table(t) => Some(t),
        _ => None,
    })?;
    let rows: Vec<Vec<table::Cell>> = table
        .rows
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| {
                    // A cell is drawn in one style: its first character's.
                    let style = cell.style_at(1.min(cell.len()));
                    table::Cell {
                        text: cell.text.clone(),
                        bold: style.bold,
                        italic: style.italic,
                        color: style.color,
                        fill: cell.style.fill,
                        align: match cell.style.align {
                            Align::Left => table::Align::Left,
                            Align::Center => table::Align::Center,
                            Align::Right => table::Align::Right,
                        },
                    }
                })
                .collect()
        })
        .collect();
    Some(Picture {
        name: "table.png".into(),
        mime: "image/png".into(),
        data: table::png(&rows)?,
    })
}

/// Whether a drag holds only pictures (files, or a picture from another
/// app), which go where they are dropped in the text.
pub(super) fn pictures_only(paths: &ExternalPaths) -> bool {
    match katna_ui::native::dropped_content(paths) {
        Some(content) => {
            content.image.is_some() && content.text.is_none() && content.html.is_none()
        }
        None => {
            !paths.paths().is_empty()
                && paths.paths().iter().all(|p| {
                    p.file_name()
                        .and_then(|n| n.to_str())
                        .and_then(katna_ui::rich::image_mime)
                        .is_some_and(shows_inline)
                })
        }
    }
}

/// Whether the editor can show a picture of this type in the text.
fn shows_inline(mime: &str) -> bool {
    matches!(
        mime,
        "image/png" | "image/jpeg" | "image/gif" | "image/webp" | "image/bmp" | "image/svg+xml"
    )
}

impl MailWindow {
    /// Files pasted in the text (copied in a file manager): pictures go in
    /// the text, the rest is attached.
    pub(super) fn paste_files(&mut self, paths: Vec<PathBuf>, cx: &mut Context<Self>) {
        self.add_files(paths, Place::Choose { inline: true }, cx);
    }

    /// Something dropped on the text: it goes where it was dropped.
    pub(in crate::window) fn drop_on_body(
        &mut self,
        paths: &ExternalPaths,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(compose) = &self.compose else {
            return;
        };
        let body = compose.body.clone();
        let at = window.mouse_position();
        body.update(cx, |editor, cx| editor.place_cursor_at(at, cx));
        window.focus(&body.focus_handle(cx), cx);
        self.take_drop(paths, true, cx);
    }

    /// A drag moving over the message text: pictures show a caret where
    /// they would land.
    pub(in crate::window) fn drag_over_body(
        &mut self,
        event: &DragMoveEvent<ExternalPaths>,
        cx: &mut Context<Self>,
    ) {
        let Some(compose) = &self.compose else {
            return;
        };
        let at = event.event.position;
        let over = event.bounds.contains(&at) && pictures_only(event.drag(cx));
        compose.body.clone().update(cx, |editor, cx| {
            editor.show_drop_caret(over.then_some(at), cx)
        });
    }

    /// Something dropped elsewhere on the message: files and pictures are
    /// attached, text goes in at the cursor.
    pub(in crate::window) fn drop_on_compose(
        &mut self,
        paths: &ExternalPaths,
        cx: &mut Context<Self>,
    ) {
        self.take_drop(paths, false, cx);
    }

    fn take_drop(&mut self, paths: &ExternalPaths, inline: bool, cx: &mut Context<Self>) {
        if let Some(content) = katna_ui::native::dropped_content(paths) {
            if let Some(compose) = &self.compose {
                compose.body.update(cx, |editor, cx| {
                    editor.take(Transfer::from_dropped(content), false, cx)
                });
            }
            return;
        }
        self.add_files(paths.paths().to_vec(), Place::Choose { inline }, cx);
    }

    /// Pictures pasted or dropped: in the text when `inline` (and the
    /// message is not plain text), else attached; either way with the
    /// choice between the two under them.
    pub(super) fn place_pictures(
        &mut self,
        pictures: Vec<Picture>,
        inline: bool,
        cx: &mut Context<Self>,
    ) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        let plain = compose.plain(cx);
        let mut total = compose.used_bytes(cx);
        let mut problem = None;
        let mut fitting = Vec::new();
        for picture in pictures {
            if total + picture.data.len() > MAX_TOTAL {
                problem = Some(tr!(
                    "compose-file-too-large",
                    name = picture.name.clone(),
                    limit = limit_text()
                ));
                continue;
            }
            total += picture.data.len();
            fitting.push(picture);
        }
        let (shown, attach): (Vec<Picture>, Vec<Picture>) = fitting
            .into_iter()
            .partition(|p| inline && !plain && shows_inline(&p.mime));
        let inline_ids = if shown.is_empty() {
            Vec::new()
        } else {
            compose
                .body
                .update(cx, |editor, cx| editor.insert_pictures(shown, cx))
        };
        let attached: Vec<Attachment> = attach.into_iter().map(attachment).collect();
        compose.attachments.extend(attached.iter().cloned());
        compose.attach_scroll.scroll_to_bottom();
        // Only pictures that could show in the text get the choice.
        let choosable =
            !plain && (!inline_ids.is_empty() || attached.iter().any(|a| shows_inline(&a.mime)));
        compose.picture_choice = Some(PictureChoice {
            inline: inline_ids,
            attached: attached
                .into_iter()
                .filter(|a| shows_inline(&a.mime))
                .collect(),
        });
        if choosable {
            let chosen = if inline {
                PasteOption::Inline
            } else {
                PasteOption::Attachment
            };
            compose.body.update(cx, |editor, cx| {
                editor.offer_choice(
                    vec![PasteOption::Inline, PasteOption::Attachment],
                    chosen,
                    cx,
                )
            });
        }
        if let Some(problem) = problem {
            self.show_snackbar(problem, None, cx);
        }
        cx.notify();
    }

    /// The choice under pasted pictures: moves them into the text or out
    /// to the attachments.
    pub(super) fn choose_picture_place(&mut self, option: PasteOption, cx: &mut Context<Self>) {
        let Some(compose) = &mut self.compose else {
            return;
        };
        let Some(choice) = compose.picture_choice.take() else {
            return;
        };
        let body = compose.body.clone();
        match option {
            PasteOption::Attachment if !choice.inline.is_empty() => {
                let pictures =
                    body.update(cx, |editor, cx| editor.remove_images(&choice.inline, cx));
                compose
                    .attachments
                    .extend(pictures.into_iter().map(attachment));
                compose.attach_scroll.scroll_to_bottom();
            }
            PasteOption::Inline if !choice.attached.is_empty() => {
                compose.attachments.retain(|a| {
                    !choice
                        .attached
                        .iter()
                        .any(|c| Arc::ptr_eq(&c.data, &a.data))
                });
                let pictures = choice
                    .attached
                    .iter()
                    .map(|a| Picture {
                        name: a.name.clone(),
                        mime: a.mime.clone(),
                        data: a.data.to_vec(),
                    })
                    .collect();
                body.update(cx, |editor, cx| editor.insert_pictures(pictures, cx));
            }
            _ => {}
        }
        // The choice is made: the options have closed.
        cx.notify();
    }
}

fn attachment(picture: Picture) -> Attachment {
    Attachment {
        name: picture.name,
        mime: picture.mime,
        data: Arc::new(picture.data),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katna_ui::rich::{Para, Table};

    #[test]
    fn tables_become_pictures() {
        let mut head = Para::plain("Name");
        head.restyle(0..4, &|s| s.bold = true);
        head.style.fill = Some(0xffff00);
        let blocks = vec![
            Block::Para(Para::plain("before")),
            Block::Table(Table {
                rows: vec![vec![head, Para::plain("1")]],
            }),
        ];
        let picture = table_picture(&blocks).unwrap();
        assert_eq!(picture.mime, "image/png");
        assert!(picture.data.starts_with(b"\x89PNG"));
        assert!(table_picture(&blocks[..1]).is_none());
    }

    #[test]
    fn only_pictures_skip_the_drop_cover() {
        let paths = |names: &[&str]| ExternalPaths(names.iter().map(|n| n.into()).collect());
        assert!(pictures_only(&paths(&["/a/photo.PNG", "/a/b.jpg"])));
        assert!(!pictures_only(&paths(&["/a/photo.png", "/a/notes.zip"])));
        assert!(!pictures_only(&paths(&["/a/scan.tiff"])));
        assert!(!pictures_only(&paths(&[])));
    }
}
