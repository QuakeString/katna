// SPDX-License-Identifier: GPL-3.0-or-later

//! Pasting and dropping into a [`RichEditor`]: text with its formatting
//! (HTML from a word processor, a spreadsheet or a browser), tables, and
//! the paste options shown after a paste (a table as a table, a picture or
//! plain text). Pictures and files go to the owner, which puts them in the
//! text or attaches them.

use std::path::PathBuf;

use gpui::{Anchor, anchored, deferred};

use super::*;
use crate::rich::doc::Table;

/// A way to have what was just pasted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PasteOption {
    /// With the formatting it was copied with.
    KeepFormatting,
    /// Cells as a table.
    Table,
    /// Cells as a picture of them.
    Picture,
    PlainText,
    /// A picture in the text (the owner places it).
    Inline,
    /// A picture as an attachment (the owner attaches it).
    Attachment,
}

/// A picture pasted or dropped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Picture {
    pub name: String,
    pub mime: String,
    pub data: Vec<u8>,
}

impl Picture {
    fn from_image(image: &gpui::Image) -> Self {
        let (mime, ext) = mime_of(image.format);
        Picture {
            name: format!("image.{ext}"),
            mime: mime.to_owned(),
            data: image.bytes.clone(),
        }
    }
}

/// What a paste or drop brought: any of text, its HTML form, a picture
/// and files.
#[derive(Debug, Clone, Default)]
pub struct Transfer {
    pub text: Option<String>,
    pub html: Option<String>,
    pub picture: Option<Picture>,
    pub files: Vec<PathBuf>,
}

impl Transfer {
    /// What a clipboard item holds (read with `gpui_linux::read_rich`).
    pub fn from_clipboard(item: &ClipboardItem) -> Self {
        let mut transfer = Transfer {
            html: gpui_linux::clipboard_html(item).map(str::to_owned),
            ..Transfer::default()
        };
        let mut text = String::new();
        for entry in item.entries() {
            match entry {
                ClipboardEntry::String(s) => text.push_str(&s.text),
                ClipboardEntry::Image(image) if transfer.picture.is_none() => {
                    transfer.picture = Some(Picture::from_image(image));
                }
                ClipboardEntry::ExternalPaths(paths) => {
                    transfer.files.extend(paths.paths().iter().cloned());
                }
                _ => {}
            }
        }
        if !text.is_empty() {
            transfer.text = Some(text.replace("\r\n", "\n").replace('\r', "\n"));
        }
        transfer
    }

    /// What was dropped from another app.
    pub fn from_dropped(content: gpui_linux::DroppedContent) -> Self {
        Transfer {
            text: content.text,
            html: content.html,
            picture: content.image.as_ref().map(Picture::from_image),
            files: Vec::new(),
        }
    }
}

/// The paste options showing under what was pasted, until the next edit.
pub(super) struct PasteOffer {
    /// The document and selection before the paste.
    before: Snapshot,
    options: Vec<PasteOption>,
    chosen: PasteOption,
    formatted: Vec<Block>,
    plain: String,
    /// The picture the source app offered.
    picture: Option<Picture>,
    /// The block the options show under.
    block: usize,
    /// The owner carries out a choice.
    owner: bool,
}

/// Gives each paste option its label.
pub type PasteLabels = Rc<dyn Fn(PasteOption) -> SharedString>;
/// Draws cells as a picture.
pub type TablePicture = Rc<dyn Fn(&[Block]) -> Option<Picture>>;

impl RichEditor {
    pub(super) fn paste(&mut self, _: &Paste, _: &mut Window, cx: &mut Context<Self>) {
        self.paste_clipboard(false, cx);
    }

    pub(super) fn paste_plain(&mut self, _: &PastePlain, _: &mut Window, cx: &mut Context<Self>) {
        self.paste_clipboard(true, cx);
    }

    fn paste_clipboard(&mut self, plain: bool, cx: &mut Context<Self>) {
        let Some(item) = gpui_linux::read_rich(|| cx.read_from_clipboard()) else {
            return;
        };
        let transfer = Transfer::from_clipboard(&item);
        // Copied here: back with its formatting.
        if let (Some((copied, fragment)), Some(text), false) =
            (&self.copied, &transfer.text, plain || self.plain)
            && copied == text
            && transfer.files.is_empty()
        {
            let fragment = fragment.clone();
            self.edit(EditKind::Other, cx, |doc, (start, end)| {
                let at = doc.delete(start, end);
                doc.insert_fragment(at, fragment)
            });
            return;
        }
        self.take(transfer, plain, cx);
    }

    /// Puts a paste or drop at the cursor: files and pictures go to the
    /// owner ([`RichEvent::PasteFiles`], [`RichEvent::PastePictures`]);
    /// formatted text keeps its formatting unless `plain`, with paste
    /// options to change how it came in.
    pub fn take(&mut self, transfer: Transfer, plain: bool, cx: &mut Context<Self>) {
        if !transfer.files.is_empty() {
            cx.emit(RichEvent::PasteFiles(transfer.files));
            return;
        }
        let rich = !plain && !self.plain;
        let text = transfer.text.clone().unwrap_or_default();
        if rich && let Some(html) = &transfer.html {
            let mut doc = html::from_pasted_html(html, &mut self.next_image_id);
            trim_empty(&mut doc.blocks);
            let has_text = doc.blocks.iter().any(|b| match b {
                Block::Para(p) => !p.text.trim().is_empty(),
                Block::Table(_) => true,
                Block::Image(_) => false,
            });
            if has_text {
                let plain_text = if text.trim().is_empty() {
                    html::to_plain(&doc).trim_end_matches('\n').to_owned()
                } else {
                    text
                };
                let options = if doc.blocks.iter().any(|b| matches!(b, Block::Table(_))) {
                    vec![
                        PasteOption::Table,
                        PasteOption::Picture,
                        PasteOption::PlainText,
                    ]
                } else if doc.has_formatting() {
                    vec![PasteOption::KeepFormatting, PasteOption::PlainText]
                } else {
                    Vec::new()
                };
                let chosen = options
                    .first()
                    .copied()
                    .unwrap_or(PasteOption::KeepFormatting);
                self.offer_paste(
                    doc.blocks,
                    plain_text,
                    transfer.picture,
                    options,
                    chosen,
                    cx,
                );
                return;
            }
            // Only pictures (a browser's copied picture): as pictures.
            if transfer.picture.is_none() {
                let pictures: Vec<Picture> = doc
                    .blocks
                    .into_iter()
                    .filter_map(|b| match b {
                        Block::Image(i) => Some(Picture {
                            name: i.name,
                            mime: i.mime,
                            data: i.data.to_vec(),
                        }),
                        _ => None,
                    })
                    .collect();
                if !pictures.is_empty() {
                    cx.emit(RichEvent::PastePictures(pictures));
                    return;
                }
            }
        }
        if let Some(picture) = transfer.picture
            && text.trim().is_empty()
        {
            cx.emit(RichEvent::PastePictures(vec![picture]));
            return;
        }
        if text.is_empty() {
            return;
        }
        // Cells copied as tab-separated text (a spreadsheet without HTML,
        // or Katna's own sheet viewer).
        if rich && let Some(table) = tab_separated(&text) {
            let options = vec![PasteOption::Table, PasteOption::PlainText];
            self.offer_paste(
                vec![Block::Table(table)],
                text,
                None,
                options,
                PasteOption::Table,
                cx,
            );
            return;
        }
        let style = self.style_for_typing();
        self.edit(EditKind::Other, cx, |doc, (start, end)| {
            let at = doc.delete(start, end);
            doc.insert_text(at, &text, &style)
        });
    }

    /// Pastes `formatted` and offers `options` for it.
    fn offer_paste(
        &mut self,
        formatted: Vec<Block>,
        plain: String,
        picture: Option<Picture>,
        options: Vec<PasteOption>,
        chosen: PasteOption,
        cx: &mut Context<Self>,
    ) {
        let offer = PasteOffer {
            before: self.snapshot(),
            options,
            chosen,
            formatted,
            plain,
            picture,
            block: 0,
            owner: false,
        };
        self.apply_paste(offer, chosen, cx);
    }

    /// Pastes the offer's content as `option` in place of the paste.
    fn apply_paste(&mut self, mut offer: PasteOffer, option: PasteOption, cx: &mut Context<Self>) {
        let picture = match option {
            PasteOption::Picture => {
                let picture = offer.picture.clone().or_else(|| {
                    self.table_picture
                        .as_ref()
                        .and_then(|draw| draw(&offer.formatted))
                });
                let Some(picture) = picture else {
                    // Nothing to show it with: stay as it is.
                    self.paste_offer = Some(offer);
                    return;
                };
                let (width, height) = image_size(&picture.data).unwrap_or((0, 0));
                self.next_image_id += 1;
                Some(Image {
                    id: self.next_image_id,
                    name: picture.name,
                    mime: picture.mime,
                    data: Arc::new(picture.data),
                    width,
                    height,
                    size: ImageSize::BestFit,
                })
            }
            _ => None,
        };
        let before = offer.before.clone();
        let style = self.style_for_typing_at(&before);
        let (formatted, plain) = (offer.formatted.clone(), offer.plain.clone());
        self.edit(EditKind::Other, cx, move |doc, _| {
            *doc = before.doc;
            let (start, end) = order(before.anchor, before.head);
            let at = doc.delete(start, end);
            match (option, picture) {
                (PasteOption::PlainText, _) => doc.insert_text(at, &plain, &style),
                (PasteOption::Picture, Some(image)) => doc.insert_image(at, image),
                _ => doc.insert_fragment(at, formatted),
            }
        });
        offer.chosen = option;
        offer.block = self.head.path.block.saturating_sub(
            // The cursor lands in the paragraph after a pasted table or
            // picture: the options go under the block itself.
            usize::from(
                self.head.offset == 0
                    && self.head.path.block > 0
                    && !matches!(
                        self.doc.blocks.get(self.head.path.block - 1),
                        Some(Block::Para(_))
                    ),
            ),
        );
        if offer.options.len() > 1 {
            self.paste_offer = Some(offer);
        }
    }

    /// The style typing would have at the cursor of `snapshot`.
    fn style_for_typing_at(&self, snapshot: &Snapshot) -> CharStyle {
        let (start, _) = order(snapshot.anchor, snapshot.head);
        let style = snapshot
            .doc
            .para(start.path)
            .map(|p| p.style_at(start.offset))
            .unwrap_or_default();
        style_without_link(&style)
    }

    /// Shows options the owner carries out (a picture inline or as an
    /// attachment) under the cursor's block, `chosen` marked.
    pub fn offer_choice(
        &mut self,
        options: Vec<PasteOption>,
        chosen: PasteOption,
        cx: &mut Context<Self>,
    ) {
        let (start, _) = self.ordered();
        let mut block = start.path.block;
        // A picture just put in: under the picture.
        if start.offset == 0
            && block > 0
            && matches!(self.doc.blocks.get(block - 1), Some(Block::Image(_)))
        {
            block -= 1;
        }
        self.paste_offer = Some(PasteOffer {
            before: self.snapshot(),
            options,
            chosen,
            formatted: Vec::new(),
            plain: String::new(),
            picture: None,
            block,
            owner: true,
        });
        cx.notify();
    }

    /// Hides the paste options.
    pub fn dismiss_paste_options(&mut self, cx: &mut Context<Self>) {
        if self.paste_offer.take().is_some() {
            cx.notify();
        }
    }

    pub fn set_paste_labels(&mut self, labels: PasteLabels) {
        self.paste_labels = Some(labels);
    }

    pub fn set_table_picture(&mut self, draw: TablePicture) {
        self.table_picture = Some(draw);
    }

    fn choose_paste(&mut self, option: PasteOption, cx: &mut Context<Self>) {
        let Some(offer) = self.paste_offer.take() else {
            return;
        };
        if offer.owner {
            let chosen = offer.chosen;
            self.paste_offer = Some(offer);
            if chosen != option {
                cx.emit(RichEvent::PasteChoice(option));
            }
            return;
        }
        if offer.chosen == option {
            self.paste_offer = Some(offer);
            return;
        }
        self.apply_paste(offer, option, cx);
    }

    /// Puts pictures in the text at the cursor; gives their ids (none in
    /// plain text, or for a kind of picture it cannot show).
    pub fn insert_pictures(&mut self, pictures: Vec<Picture>, cx: &mut Context<Self>) -> Vec<u64> {
        if self.plain {
            return Vec::new();
        }
        let mut images = Vec::new();
        for picture in pictures {
            if format_of(&picture.mime).is_none() {
                continue;
            }
            let (width, height) = image_size(&picture.data).unwrap_or((0, 0));
            self.next_image_id += 1;
            images.push(Image {
                id: self.next_image_id,
                name: picture.name,
                mime: picture.mime,
                data: Arc::new(picture.data),
                width,
                height,
                size: ImageSize::BestFit,
            });
        }
        let ids: Vec<u64> = images.iter().map(|i| i.id).collect();
        if !images.is_empty() {
            self.edit(EditKind::Other, cx, |doc, (start, end)| {
                let mut at = doc.delete(start, end);
                for image in images {
                    at = doc.insert_image(at, image);
                }
                at
            });
        }
        ids
    }

    /// Takes the pictures with `ids` out of the text; gives them back.
    pub fn remove_images(&mut self, ids: &[u64], cx: &mut Context<Self>) -> Vec<Picture> {
        let found: Vec<usize> = (0..self.doc.blocks.len())
            .filter(|&ix| matches!(&self.doc.blocks[ix], Block::Image(i) if ids.contains(&i.id)))
            .collect();
        let pictures = found
            .iter()
            .filter_map(|&ix| match &self.doc.blocks[ix] {
                Block::Image(i) => Some(Picture {
                    name: i.name.clone(),
                    mime: i.mime.clone(),
                    data: i.data.to_vec(),
                }),
                _ => None,
            })
            .collect();
        if !found.is_empty() {
            self.edit(EditKind::Other, cx, |doc, (start, _)| {
                let mut cursor = start;
                for &ix in found.iter().rev() {
                    cursor = doc.remove_block(ix);
                }
                cursor
            });
        }
        pictures
    }

    /// Moves the cursor to the text under window point `p` (where
    /// something is dropped).
    pub fn place_cursor_at(&mut self, p: Point<Pixels>, cx: &mut Context<Self>) {
        if let Some((pos, upstream)) = self.hit(p) {
            self.set_selection(pos, pos, cx);
            self.upstream = upstream;
        }
    }

    /// The paste options under block `ix`, if they show there.
    pub(super) fn render_paste_options(
        &self,
        ix: usize,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let offer = self.paste_offer.as_ref().filter(|o| o.block == ix)?;
        // Floating above everything, it would cover a drop area.
        if cx.has_active_drag() {
            return None;
        }
        let palette = self.palette;
        let buttons = offer.options.iter().map(|&option| {
            let label = match &self.paste_labels {
                Some(labels) => labels(option),
                None => SharedString::from(format!("{option:?}")),
            };
            let on = option == offer.chosen;
            div()
                .id(("paste-option", option as usize))
                .px(px(10.0))
                .py(px(4.0))
                .rounded(px(6.0))
                .text_size(px(13.0))
                .text_color(if on { palette.accent } else { palette.text })
                .when(on, |d| {
                    d.bg(palette.hover).font_weight(gpui::FontWeight::MEDIUM)
                })
                .cursor_pointer()
                .hover(|s| s.bg(palette.hover))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        this.choose_paste(option, cx);
                    }),
                )
                .child(label)
        });
        let bar = div()
            .flex()
            .flex_row()
            .gap(px(2.0))
            .p(px(3.0))
            .rounded(px(8.0))
            .bg(palette.surface)
            .border_1()
            .border_color(palette.rule)
            .shadow_md()
            .cursor(CursorStyle::Arrow)
            .children(buttons);
        Some(
            deferred(
                div().absolute().bottom_0().left_0().child(
                    anchored()
                        .anchor(Anchor::TopLeft)
                        .offset(point(px(0.0), px(4.0)))
                        .snap_to_window_with_margin(px(8.0))
                        .child(div().occlude().child(bar)),
                ),
            )
            .with_priority(1)
            .into_any_element(),
        )
    }
}

/// Drops empty paragraphs at both ends (a copy often has them).
fn trim_empty(blocks: &mut Vec<Block>) {
    let empty = |b: &Block| matches!(b, Block::Para(p) if p.text.trim().is_empty());
    while blocks.len() > 1 && blocks.last().is_some_and(empty) {
        blocks.pop();
    }
    while blocks.len() > 1 && blocks.first().is_some_and(empty) {
        blocks.remove(0);
    }
}

/// Text that is rows of tab-separated cells, all with the same number of
/// cells: as a table.
fn tab_separated(text: &str) -> Option<Table> {
    let text = text.trim_end_matches('\n');
    let lines: Vec<&str> = text.split('\n').collect();
    let cols = lines.first()?.split('\t').count();
    if cols < 2 || lines.len() > 1000 || lines.iter().any(|l| l.trim().is_empty()) {
        return None;
    }
    // Lines indented with tabs (code) are not cells.
    if lines.iter().all(|l| l.starts_with('\t')) {
        return None;
    }
    if lines.iter().any(|l| l.split('\t').count() != cols) {
        return None;
    }
    Some(Table {
        rows: lines
            .iter()
            .map(|line| {
                line.split('\t')
                    .map(|cell| Para::plain(cell.trim()))
                    .collect()
            })
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tab_separated_cells_are_tables() {
        let table = tab_separated("a\tb\n1\t2\n").unwrap();
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[1][1].text, "2");
        assert!(tab_separated("a\tb\n1").is_none());
        assert!(tab_separated("just text\nlines").is_none());
        assert!(tab_separated("one\ttwo").is_some());
        assert!(tab_separated("\tfn a() {\n\t}").is_none());
    }
}
