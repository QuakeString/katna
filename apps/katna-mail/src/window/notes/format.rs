// SPDX-License-Identifier: GPL-3.0-or-later

//! Formatting in notes, as Keep has it: Heading 1, Heading 2 and Normal
//! text for a line, Bold, Italic and Underline for words, and Clear
//! formatting. A note keeps its plain text in `body` (for the board,
//! search and checklists) and the same text formatted in `html`, one
//! paragraph per line, empty while it has no formatting.

use gpui::{
    AnyElement, Context, FontStyle, FontWeight, HighlightStyle, StyledText, UnderlineStyle, div,
    prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::rich::{Block, CharStyle, Doc, Para, ParaStyle, RichEditor, Size, html};

use super::{MailWindow, TICKED, UNTICKED, check_of};
use crate::theme::Theme;
use crate::widgets::{icon_button, tip};

/// A line's look: Keep's three.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Heading {
    One,
    Two,
    Normal,
}

impl Heading {
    fn size(self) -> Size {
        match self {
            Heading::One => Size::Huge,
            Heading::Two => Size::Large,
            Heading::Normal => Size::Normal,
        }
    }

    fn of(style: &CharStyle) -> Self {
        match (style.size, style.bold) {
            (Size::Huge, true) => Heading::One,
            (Size::Large, true) => Heading::Two,
            _ => Heading::Normal,
        }
    }
}

/// The note's text for the editor: its formatting, or its plain lines.
pub(super) fn doc_of(body: &str, formatted: &str) -> Doc {
    if !formatted.is_empty() {
        let doc = html::from_html(formatted, &mut 0);
        if !doc.blocks.is_empty() {
            return doc;
        }
    }
    Doc {
        blocks: body
            .split('\n')
            .map(|line| Block::Para(Para::plain(line)))
            .collect(),
    }
}

/// The paragraphs of `doc`, table cells too, in order.
fn paras(doc: &Doc) -> Vec<&Para> {
    doc.paths().into_iter().filter_map(|p| doc.para(p)).collect()
}

/// `doc`'s plain text: a line per paragraph.
pub(super) fn text_of(doc: &Doc) -> String {
    paras(doc)
        .iter()
        .map(|p| p.text.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

/// `doc` as the note's HTML; empty when nothing in it is formatted.
pub(super) fn html_of(doc: &Doc) -> String {
    let plain = doc.blocks.iter().all(|block| match block {
        Block::Para(p) => p.style == ParaStyle::default() && p.spans().all(|(_, s)| s.is_plain()),
        _ => false,
    });
    if plain {
        String::new()
    } else {
        html::to_html(doc, &|_| String::new())
    }
}

/// The paragraphs of `formatted` that line up one to one with the lines
/// of `body`, for a card to draw them formatted.
pub(super) fn card_paras(body: &str, formatted: &str) -> Option<Vec<Para>> {
    if formatted.is_empty() {
        return None;
    }
    let doc = html::from_html(formatted, &mut 0);
    (text_of(&doc) == body).then(|| paras(&doc).into_iter().cloned().collect())
}

/// `formatted` with its line `ix` ticked or unticked, as a card's
/// checkbox does to `body`.
pub(super) fn toggle_html_line(body: &str, formatted: &str, ix: usize) -> String {
    let mut doc = html::from_html(formatted, &mut 0);
    if text_of(&doc) != body {
        return formatted.to_owned();
    }
    let Some(path) = doc.paths().get(ix).copied() else {
        return formatted.to_owned();
    };
    if let Some(para) = doc.para_mut(path) {
        let style = para.style_at(0);
        match check_of(&para.text) {
            Some((true, _)) => {
                para.remove(0..TICKED.len());
                para.insert(0, UNTICKED, &style);
            }
            Some((false, _)) => {
                para.remove(0..UNTICKED.len());
                para.insert(0, TICKED, &style);
            }
            None => {}
        }
    }
    html_of(&doc)
}

/// `doc` as a checklist, or back to plain lines when it is one already,
/// keeping each line's formatting (Keep's "Show checkboxes").
pub(super) fn toggle_checklist_doc(doc: &mut Doc) {
    let paths = doc.paths();
    let texts: Vec<String> = paths
        .iter()
        .filter_map(|p| doc.para(*p).map(|p| p.text.clone()))
        .collect();
    let body = texts.join("\n");
    let new = super::toggle_checklist(&body);
    for (path, line) in paths.iter().zip(new.split('\n')) {
        let Some(para) = doc.para_mut(*path) else {
            continue;
        };
        if para.text == line {
            continue;
        }
        let style = para.style_at(0);
        if let Some(rest) = line.strip_suffix(para.text.as_str()) {
            // A mark went on.
            para.insert(0, rest, &style);
        } else if let Some(cut) = para.text.len().checked_sub(line.len()) {
            // A mark came off.
            para.remove(0..cut);
        }
    }
    // An empty note becomes one empty item.
    if paths.is_empty() || (texts.len() == 1 && texts[0].is_empty() && new == UNTICKED) {
        doc.blocks = vec![Block::Para(Para::plain(UNTICKED))];
    }
}

/// A card's line of a formatted note: its text after `skip` bytes (a
/// checkbox mark), with its bold, italic and underline, and its size.
pub(super) fn card_line(para: &Para, skip: usize) -> (AnyElement, f32) {
    let text = para.text.get(skip..).unwrap_or("").to_owned();
    let mut highlights = Vec::new();
    let mut scale = 1.0_f32;
    for (range, style) in para.spans() {
        scale = scale.max(style.size.scale());
        let (start, end) = (range.start.max(skip) - skip, range.end.saturating_sub(skip));
        if start >= end {
            continue;
        }
        let highlight = HighlightStyle {
            font_weight: style.bold.then_some(FontWeight::BOLD),
            font_style: style.italic.then_some(FontStyle::Italic),
            underline: style.underline.then(|| UnderlineStyle {
                thickness: gpui::px(1.0),
                ..UnderlineStyle::default()
            }),
            ..HighlightStyle::default()
        };
        if highlight != HighlightStyle::default() {
            highlights.push((start..end, highlight));
        }
    }
    let text = if text.is_empty() { " ".to_owned() } else { text };
    (
        StyledText::new(text)
            .with_highlights(highlights)
            .into_any_element(),
        scale,
    )
}

impl MailWindow {
    fn note_body(&self) -> Option<gpui::Entity<RichEditor>> {
        Some(self.notes.as_ref()?.editor.as_ref()?.body.clone())
    }

    /// Heading 1, Heading 2 or Normal text for the lines the cursor or
    /// selection is on.
    fn set_heading(&mut self, heading: Heading, cx: &mut Context<Self>) {
        let Some(body) = self.note_body() else {
            return;
        };
        body.update(cx, |editor, cx| {
            editor.restyle_paras_text(
                move |s| {
                    s.size = heading.size();
                    s.bold = heading != Heading::Normal;
                },
                cx,
            )
        });
    }

    /// The row of formatting tools under an open note.
    pub(super) fn render_format_row(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let editor = self.notes.as_ref()?.editor.as_ref()?;
        if !editor.format {
            return None;
        }
        let area = editor.body.read(cx);
        let style = area.current_style();
        let heading = Heading::of(&style);
        let tool = |id: &'static str, name: &'static str, tip_text: String, on: bool| {
            icon_button(id, name, 18.0, th)
                .size(px(34.0))
                .when(on, |b| b.bg(rgba(th.nav_selected)))
                .tooltip(tip(tip_text, th))
        };
        let heading_button = |id: &'static str, label: String, value: Heading| {
            div()
                .id(id)
                .h(px(34.0))
                .px(px(10.0))
                .flex()
                .items_center()
                .rounded(px(8.0))
                .text_size(px(13.0))
                .when(value != Heading::Normal, |d| d.font_weight(FontWeight::BOLD))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .when(heading == value, |d| d.bg(rgba(th.nav_selected)))
                .on_click(cx.listener(move |this, _, _, cx| this.set_heading(value, cx)))
                .child(label)
        };
        Some(
            div()
                .flex_none()
                .mx(px(12.0))
                .mb(px(4.0))
                .px(px(4.0))
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(2.0))
                .rounded(px(10.0))
                .bg(rgba(th.chip))
                .child(heading_button(
                    "note-h1",
                    tr!("notes-format-heading-1"),
                    Heading::One,
                ))
                .child(heading_button(
                    "note-h2",
                    tr!("notes-format-heading-2"),
                    Heading::Two,
                ))
                .child(heading_button(
                    "note-normal",
                    tr!("notes-format-normal"),
                    Heading::Normal,
                ))
                .child(div().w(px(1.0)).h(px(20.0)).mx(px(4.0)).bg(rgba(th.divider)))
                .child(
                    tool(
                        "note-bold",
                        "format-bold",
                        tr!("notes-format-bold"),
                        style.bold,
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(body) = this.note_body() {
                            body.update(cx, |e, cx| e.toggle_bold(cx));
                        }
                    })),
                )
                .child(
                    tool(
                        "note-italic",
                        "format-italic",
                        tr!("notes-format-italic"),
                        style.italic,
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(body) = this.note_body() {
                            body.update(cx, |e, cx| e.toggle_italic(cx));
                        }
                    })),
                )
                .child(
                    tool(
                        "note-underline",
                        "format-underline",
                        tr!("notes-format-underline"),
                        style.underline,
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(body) = this.note_body() {
                            body.update(cx, |e, cx| e.toggle_underline(cx));
                        }
                    })),
                )
                .child(
                    tool(
                        "note-clear-format",
                        "clear-format",
                        tr!("notes-format-clear"),
                        false,
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(body) = this.note_body() {
                            body.update(cx, |e, cx| e.clear_formatting(cx));
                        }
                    })),
                )
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use katna_ui::rich::Pos;

    fn bold(doc: &mut Doc, line: usize) {
        let path = doc.paths()[line];
        let len = doc.para(path).unwrap().len();
        doc.restyle(Pos::new(path, 0), Pos::new(path, len), &|s| s.bold = true);
    }

    #[test]
    fn plain_lines_keep_no_html() {
        let doc = doc_of("Milk\n☐ Bread", "");
        assert_eq!(text_of(&doc), "Milk\n☐ Bread");
        assert_eq!(html_of(&doc), "");
    }

    #[test]
    fn formatting_comes_back_as_it_was() {
        let mut doc = doc_of("Plan\n☐ Book train", "");
        bold(&mut doc, 0);
        let formatted = html_of(&doc);
        assert!(formatted.contains("<b>Plan</b>") || formatted.contains("font-weight"));
        let back = doc_of("", &formatted);
        assert_eq!(text_of(&back), "Plan\n☐ Book train");
        assert_eq!(html_of(&back), formatted);
        let paras = card_paras("Plan\n☐ Book train", &formatted).unwrap();
        assert!(paras[0].style_at(1).bold);
    }

    #[test]
    fn a_card_ticks_a_formatted_line() {
        let mut doc = doc_of("Plan\n☐ Book train", "");
        bold(&mut doc, 1);
        let formatted = html_of(&doc);
        let ticked = toggle_html_line("Plan\n☐ Book train", &formatted, 1);
        let back = doc_of("", &ticked);
        assert_eq!(text_of(&back), "Plan\n☑ Book train");
        assert!(paras(&back)[1].style_at(5).bold);
    }

    #[test]
    fn checkboxes_come_and_go_with_formatting() {
        let mut doc = doc_of("milk\neggs", "");
        bold(&mut doc, 1);
        toggle_checklist_doc(&mut doc);
        assert_eq!(text_of(&doc), "☐ milk\n☐ eggs");
        assert!(paras(&doc)[1].style_at(6).bold);
        toggle_checklist_doc(&mut doc);
        assert_eq!(text_of(&doc), "milk\neggs");
        let mut empty = doc_of("", "");
        toggle_checklist_doc(&mut empty);
        assert_eq!(text_of(&empty), UNTICKED);
    }

    #[test]
    fn headings_are_known_by_their_look() {
        let style = CharStyle {
            bold: true,
            size: Size::Huge,
            ..CharStyle::default()
        };
        assert_eq!(Heading::of(&style), Heading::One);
        assert_eq!(Heading::of(&CharStyle::default()), Heading::Normal);
    }
}
