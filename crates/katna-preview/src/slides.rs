// SPDX-License-Identifier: GPL-3.0-or-later

//! Slides for the viewer: PowerPoint (pptx and the older ppt) and
//! OpenDocument presentations (odp). Each slide's text becomes a
//! [`Block::Slide`] followed by its title, its other text and its tables,
//! in the same [`Document`] model as word processor files, so the viewer
//! and the attachment cards show them the same way. Pictures, charts,
//! layout and speaker notes are left out.

use std::collections::{HashMap, HashSet};
use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::events::Event;
use zip::ZipArchive;

use crate::document::{
    Block, Builder, Document, Invalid, MAX_CHARS, MAX_PARAGRAPHS, Paragraph, Run, Style, attr,
    bullet, entity, local, number_as, odp, part,
};
use crate::ole::{self, cp1252, symbol, u16_at, u32_at};

/// At most this many slides are read.
const MAX_SLIDES: usize = 2_000;
/// A first line up to this long can stand in for a missing title.
const TITLE_CHARS: usize = 100;

/// Reads a pptx, ppt or odp file; the format is found from what is inside.
pub fn open(bytes: Vec<u8>) -> Result<Document, Invalid> {
    if ole::is_ole(&bytes) {
        return ppt(bytes);
    }
    let mut zip = ZipArchive::new(Cursor::new(bytes)).map_err(|_| Invalid)?;
    if zip.index_for_name("ppt/presentation.xml").is_some() {
        pptx(&mut zip)
    } else if zip.index_for_name("content.xml").is_some() {
        let content = part(&mut zip, "content.xml").ok_or(Invalid)?;
        let styles = part(&mut zip, "styles.xml").unwrap_or_default();
        Ok(odp(&content, &styles))
    } else {
        Err(Invalid)
    }
}

/// Collects slides, keeping to the size limits.
#[derive(Default)]
struct Deck {
    doc: Document,
    paragraphs: usize,
    chars: usize,
}

impl Deck {
    /// Adds a slide: its title's blocks first, then the rest.
    fn slide(&mut self, mut title: Vec<Block>, mut rest: Vec<Block>) {
        // Slides made of text boxes have no title placeholder; a short
        // plain line on top reads as one.
        if title.is_empty()
            && let Some(Block::Paragraph(p)) = rest.first_mut()
            && p.style == Style::Normal
            && p.list.is_none()
            && p.text().chars().count() <= TITLE_CHARS
        {
            p.style = Style::Heading(1);
            title.push(rest.remove(0));
        }
        let n = self
            .doc
            .blocks
            .iter()
            .filter(|b| matches!(b, Block::Slide(_)))
            .count();
        if n >= MAX_SLIDES {
            self.doc.cut = true;
            return;
        }
        self.doc.blocks.push(Block::Slide(n as u32 + 1));
        for block in title.into_iter().chain(rest) {
            let (paragraphs, chars) = match &block {
                Block::Paragraph(p) => (1, p.text().len()),
                Block::Table(rows) => {
                    let cells = rows.iter().flatten().flatten();
                    (cells.clone().count(), cells.map(|p| p.text().len()).sum())
                }
                Block::Slide(_) => (0, 0),
            };
            if self.paragraphs + paragraphs > MAX_PARAGRAPHS || self.chars + chars > MAX_CHARS {
                self.doc.cut = true;
                return;
            }
            self.paragraphs += paragraphs;
            self.chars += chars;
            self.doc.blocks.push(block);
        }
    }

    fn finish(self) -> Result<Document, Invalid> {
        if self.doc.blocks.is_empty() {
            Err(Invalid)
        } else {
            Ok(self.doc)
        }
    }
}

// ---- PowerPoint (pptx) ----

fn pptx(zip: &mut ZipArchive<Cursor<Vec<u8>>>) -> Result<Document, Invalid> {
    let mut deck = Deck::default();
    for name in slide_parts(zip) {
        let Some(xml) = part(zip, &name) else {
            continue;
        };
        let (title, rest) = pptx_slide(&xml);
        deck.slide(title, rest);
        if deck.doc.cut {
            break;
        }
    }
    deck.finish()
}

/// The slides' parts in show order: the presentation's list of slides,
/// or else the slide files by number.
fn slide_parts(zip: &mut ZipArchive<Cursor<Vec<u8>>>) -> Vec<String> {
    let presentation = part(zip, "ppt/presentation.xml").unwrap_or_default();
    let rels = part(zip, "ppt/_rels/presentation.xml.rels").unwrap_or_default();
    let mut targets = HashMap::new();
    let mut reader = Reader::from_str(&rels);
    while let Ok(event) = reader.read_event() {
        match event {
            Event::Eof => break,
            Event::Start(e) | Event::Empty(e) if local(&e) == "Relationship" => {
                if let (Some(id), Some(target)) = (attr(&e, "Id"), attr(&e, "Target")) {
                    targets.insert(id, target);
                }
            }
            _ => {}
        }
    }
    let mut parts = Vec::new();
    let mut reader = Reader::from_str(&presentation);
    while let Ok(event) = reader.read_event() {
        match event {
            Event::Eof => break,
            Event::Start(e) | Event::Empty(e) if local(&e) == "sldId" => {
                // The relationship id (r:id), not the slide's own number (id).
                let rel = e
                    .attributes()
                    .flatten()
                    .find(|a| a.key.prefix().is_some() && a.key.local_name().as_ref() == b"id");
                if let Some(target) = rel
                    .and_then(|a| String::from_utf8(a.value.into_owned()).ok())
                    .and_then(|id| targets.get(&id))
                {
                    let target = target.trim_start_matches('/');
                    parts.push(match target.strip_prefix("ppt/") {
                        Some(_) => target.to_owned(),
                        None => format!("ppt/{target}"),
                    });
                }
            }
            _ => {}
        }
    }
    if parts.is_empty() {
        let mut numbered: Vec<(u32, String)> = zip
            .file_names()
            .filter_map(|name| {
                let n = name
                    .strip_prefix("ppt/slides/slide")?
                    .strip_suffix(".xml")?
                    .parse()
                    .ok()?;
                Some((n, name.to_owned()))
            })
            .collect();
        numbered.sort();
        parts = numbered.into_iter().map(|(_, name)| name).collect();
    }
    parts.truncate(MAX_SLIDES);
    parts
}

/// What kind of text a shape holds.
#[derive(Clone, Copy, PartialEq)]
enum Placeholder {
    /// Not a placeholder: a text box or shape.
    None,
    Title,
    CenterTitle,
    Subtitle,
    /// Body text, bulleted by the slide master.
    Body,
    /// The date, footer or slide number: not shown.
    Hidden,
}

/// A slide's title blocks and its other blocks.
fn pptx_slide(xml: &str) -> (Vec<Block>, Vec<Block>) {
    let mut title = Vec::new();
    let mut rest = Vec::new();
    let mut reader = Reader::from_str(xml);
    // The shape being read, its kind, and its text.
    let mut shape: Option<(Placeholder, Builder)> = None;
    let mut depth = 0usize;
    let mut shape_depth = 0usize;
    let mut look = Run::default();
    let (mut in_rpr, mut in_text) = (false, false);
    // Paragraph's level and bullet: none, a character, or numbered.
    let mut level = 0u8;
    let mut bullet_kind: Option<Option<String>> = None;
    let mut counters: HashMap<u8, u32> = HashMap::new();
    let mut tables = 0usize;
    loop {
        let event = match reader.read_event() {
            Ok(Event::Eof) | Err(_) => break,
            Ok(event) => event,
        };
        let empty = matches!(event, Event::Empty(_));
        match event {
            Event::Start(e) | Event::Empty(e) => {
                if !empty {
                    depth += 1;
                }
                let name = local(&e);
                match name.as_str() {
                    // A shape with text, or a frame holding a table.
                    "sp" | "graphicFrame" if shape.is_none() && !empty => {
                        shape = Some((Placeholder::None, Builder::default()));
                        shape_depth = depth;
                        counters.clear();
                    }
                    "ph" => {
                        if let Some((kind, _)) = &mut shape {
                            *kind = match attr(&e, "type").as_deref() {
                                Some("title") => Placeholder::Title,
                                Some("ctrTitle") => Placeholder::CenterTitle,
                                Some("subTitle") => Placeholder::Subtitle,
                                Some("body" | "obj") | None => Placeholder::Body,
                                Some("dt" | "ftr" | "sldNum" | "hdr") => Placeholder::Hidden,
                                Some(_) => Placeholder::None,
                            };
                        }
                    }
                    "tbl" if !empty => {
                        if let Some((_, out)) = &mut shape {
                            out.start_table();
                            tables += 1;
                        }
                    }
                    "tr" => {
                        if let Some((_, out)) = &mut shape {
                            out.start_row();
                        }
                    }
                    "tc" => {
                        if let Some((_, out)) = &mut shape {
                            out.start_cell(1);
                        }
                    }
                    "p" if !empty => {
                        if let Some((_, out)) = &mut shape {
                            out.start_paragraph();
                            level = 0;
                            bullet_kind = None;
                        }
                    }
                    "pPr" => {
                        level = attr(&e, "lvl")
                            .and_then(|l| l.parse::<u8>().ok())
                            .unwrap_or(0)
                            .min(8);
                    }
                    "buNone" => bullet_kind = Some(None),
                    "buChar" => {
                        let c = attr(&e, "char")
                            .and_then(|c| c.chars().next())
                            .map_or('•', symbol);
                        bullet_kind = Some(Some(c.to_string()));
                    }
                    "buAutoNum" => {
                        let scheme = attr(&e, "type").unwrap_or_default();
                        bullet_kind = Some(Some(format!("#{scheme}")));
                    }
                    "r" | "fld" => look = Run::default(),
                    "rPr" => {
                        let on = |key| attr(&e, key).is_some_and(|v| v == "1" || v == "true");
                        look.bold = on("b");
                        look.italic = on("i");
                        look.underline = attr(&e, "u").is_some_and(|u| u != "none");
                        look.strike = attr(&e, "strike").is_some_and(|s| s != "noStrike");
                        in_rpr = !empty;
                    }
                    "t" if !empty && !in_rpr => in_text = true,
                    "br" => {
                        if let Some((_, out)) = &mut shape {
                            out.text("\n", &look);
                        }
                    }
                    _ => {}
                }
            }
            Event::End(e) => {
                let name = e.local_name();
                match name.as_ref() {
                    b"rPr" => in_rpr = false,
                    b"t" => in_text = false,
                    b"tbl" => {
                        if let Some((_, out)) = &mut shape {
                            out.end_table();
                            tables = tables.saturating_sub(1);
                        }
                    }
                    b"p" => {
                        if let Some((kind, out)) = &mut shape {
                            let marker = match (&bullet_kind, *kind) {
                                (Some(Some(m)), _) => Some(match m.strip_prefix('#') {
                                    Some(scheme) => {
                                        counters.retain(|l, _| *l <= level);
                                        let n = counters.entry(level).or_insert(0);
                                        *n += 1;
                                        numbered(scheme, *n)
                                    }
                                    None => m.clone(),
                                }),
                                (None, Placeholder::Body) => Some(bullet(level).to_owned()),
                                _ => None,
                            };
                            if let Some(p) = out.paragraph.as_mut() {
                                p.style = match kind {
                                    Placeholder::Title => Style::Heading(1),
                                    Placeholder::CenterTitle => Style::Title,
                                    Placeholder::Subtitle => Style::Subtitle,
                                    _ => Style::Normal,
                                };
                                // Table cells carry no bullets.
                                if tables == 0 {
                                    p.list = marker.map(|m| (level, m));
                                }
                            }
                            out.end_paragraph_unless_blank();
                        }
                    }
                    _ => {}
                }
                if depth == shape_depth
                    && matches!(name.as_ref(), b"sp" | b"graphicFrame")
                    && let Some((kind, out)) = shape.take()
                {
                    let blocks = out.finish().blocks;
                    match kind {
                        Placeholder::Title | Placeholder::CenterTitle => title.extend(blocks),
                        Placeholder::Hidden => {}
                        _ => rest.extend(blocks),
                    }
                }
                depth = depth.saturating_sub(1);
            }
            Event::Text(t) if in_text => {
                if let (Some((_, out)), Ok(text)) = (&mut shape, t.xml10_content()) {
                    out.text(&text, &look);
                }
            }
            Event::GeneralRef(r) if in_text => {
                if let (Some((_, out)), Some(c)) = (&mut shape, entity(&r)) {
                    out.text(c.encode_utf8(&mut [0; 4]), &look);
                }
            }
            _ => {}
        }
    }
    (title, rest)
}

/// Number `n` in a DrawingML numbering scheme such as "arabicPeriod" or
/// "alphaLcParenR".
fn numbered(scheme: &str, n: u32) -> String {
    let format = if scheme.starts_with("alphaLc") {
        "lowerLetter"
    } else if scheme.starts_with("alphaUc") {
        "upperLetter"
    } else if scheme.starts_with("romanLc") {
        "lowerRoman"
    } else if scheme.starts_with("romanUc") {
        "upperRoman"
    } else {
        "decimal"
    };
    let n = number_as(format, n);
    if scheme.ends_with("ParenBoth") {
        format!("({n})")
    } else if scheme.ends_with("ParenR") {
        format!("{n})")
    } else if scheme.ends_with("Plain") {
        n
    } else {
        format!("{n}.")
    }
}

// ---- PowerPoint 97–2003 (ppt) ----

/// Record types of the "PowerPoint Document" stream ([MS-PPT]).
const DOCUMENT: u16 = 0x03e8;
const SLIDE: u16 = 0x03ee;
const SLIDE_LIST_WITH_TEXT: u16 = 0x0ff0;
const SLIDE_PERSIST_ATOM: u16 = 0x03f3;
const TEXT_HEADER_ATOM: u16 = 0x0f9f;
const TEXT_CHARS_ATOM: u16 = 0x0fa0;
const TEXT_BYTES_ATOM: u16 = 0x0fa8;
const USER_EDIT_ATOM: u16 = 0x0ff5;
const PERSIST_DIRECTORY_ATOM: u16 = 0x1772;
/// Marks a file whose streams are encrypted.
const ENCRYPTED_TOKEN: u32 = 0xf3d1_c4df;

/// A record: its type, instance, whether it holds other records, and its
/// body's range in the stream.
#[derive(Clone, Copy)]
struct Record {
    kind: u16,
    instance: u16,
    container: bool,
    start: usize,
    end: usize,
}

fn record(stream: &[u8], at: usize) -> Option<Record> {
    let ver_instance = u16_at(stream, at)?;
    let kind = u16_at(stream, at + 2)?;
    let len = u32_at(stream, at + 4)? as usize;
    let start = at + 8;
    let end = start.checked_add(len)?;
    (end <= stream.len()).then_some(Record {
        kind,
        instance: ver_instance >> 4,
        container: ver_instance & 0x0f == 0x0f,
        start,
        end,
    })
}

/// The records directly inside `range` of the stream.
fn children(stream: &[u8], start: usize, end: usize) -> Vec<Record> {
    let mut out = Vec::new();
    let mut at = start;
    while at + 8 <= end {
        let Some(r) = record(stream, at) else {
            break;
        };
        if r.end > end {
            break;
        }
        out.push(r);
        at = r.end;
    }
    out
}

/// A text of a slide: its kind (title, body...) and its characters.
struct SlideText {
    kind: u32,
    text: String,
}

fn ppt(bytes: Vec<u8>) -> Result<Document, Invalid> {
    let mut file = ole::open(bytes).ok_or(Invalid)?;
    let stream = ole::stream(&mut file, "PowerPoint Document").ok_or(Invalid)?;
    let user = ole::stream(&mut file, "Current User").unwrap_or_default();
    if u32_at(&user, 12) == Some(ENCRYPTED_TOKEN) {
        return Err(Invalid);
    }
    let slides = match persist_directory(&stream, &user) {
        Some((directory, document)) => slides_in_order(&stream, &directory, document),
        None => Vec::new(),
    };
    // Without a usable directory, the slides as they sit in the stream.
    let slides = if slides.is_empty() {
        children(&stream, 0, stream.len())
            .into_iter()
            .filter(|r| r.kind == SLIDE)
            .map(|r| (Some(r), Vec::new()))
            .collect()
    } else {
        slides
    };
    let mut deck = Deck::default();
    for (container, listed) in slides.into_iter().take(MAX_SLIDES) {
        let mut texts = Vec::new();
        if let Some(r) = container {
            collect_texts(&stream, r, &mut texts, 0);
        }
        if texts.is_empty() {
            texts = listed;
        }
        let (title, rest) = ppt_blocks(texts);
        deck.slide(title, rest);
        if deck.doc.cut {
            break;
        }
    }
    deck.finish()
}

/// Where each persistent object is (persist id → offset), newest edit
/// winning, and the id of the document record.
fn persist_directory(stream: &[u8], user: &[u8]) -> Option<(HashMap<u32, usize>, u32)> {
    let mut edit = u32_at(user, 16)? as usize;
    let mut directory = HashMap::new();
    let mut document = None;
    let mut seen = HashSet::new();
    while seen.insert(edit) && seen.len() < 256 {
        let r = record(stream, edit)?;
        if r.kind != USER_EDIT_ATOM {
            return None;
        }
        let body = &stream[r.start..r.end];
        let last_edit = u32_at(body, 8)? as usize;
        let dir_at = u32_at(body, 12)? as usize;
        document.get_or_insert(u32_at(body, 16)?);
        let dir = record(stream, dir_at)?;
        if dir.kind != PERSIST_DIRECTORY_ATOM {
            return None;
        }
        let entries = &stream[dir.start..dir.end];
        let mut at = 0;
        while let Some(info) = u32_at(entries, at) {
            let first = info & 0x000f_ffff;
            let count = info >> 20;
            at += 4;
            for i in 0..count {
                let Some(offset) = u32_at(entries, at) else {
                    break;
                };
                directory.entry(first + i).or_insert(offset as usize);
                at += 4;
            }
        }
        if last_edit == 0 {
            break;
        }
        edit = last_edit;
    }
    Some((directory, document?))
}

/// The slides in show order: each one's record, if found, and the texts
/// the slide list keeps for it (older files keep them there).
fn slides_in_order(
    stream: &[u8],
    directory: &HashMap<u32, usize>,
    document: u32,
) -> Vec<(Option<Record>, Vec<SlideText>)> {
    let Some(doc) = directory
        .get(&document)
        .and_then(|&at| record(stream, at))
        .filter(|r| r.kind == DOCUMENT)
    else {
        return Vec::new();
    };
    let Some(list) = children(stream, doc.start, doc.end)
        .into_iter()
        .find(|r| r.kind == SLIDE_LIST_WITH_TEXT && r.instance == 0)
    else {
        return Vec::new();
    };
    let mut slides: Vec<(Option<Record>, Vec<SlideText>)> = Vec::new();
    let mut kind = 4;
    for r in children(stream, list.start, list.end) {
        match r.kind {
            SLIDE_PERSIST_ATOM => {
                let slide = u32_at(stream, r.start)
                    .and_then(|id| directory.get(&id))
                    .and_then(|&at| record(stream, at))
                    .filter(|r| r.kind == SLIDE);
                slides.push((slide, Vec::new()));
            }
            TEXT_HEADER_ATOM => kind = u32_at(stream, r.start).unwrap_or(4),
            TEXT_CHARS_ATOM | TEXT_BYTES_ATOM => {
                if let Some((_, texts)) = slides.last_mut() {
                    texts.push(SlideText {
                        kind,
                        text: atom_text(stream, r),
                    });
                }
            }
            _ => {}
        }
    }
    slides
}

/// The texts inside a slide's record, in order, looking into its shapes.
fn collect_texts(stream: &[u8], container: Record, out: &mut Vec<SlideText>, depth: usize) {
    if depth > 32 {
        return;
    }
    let mut kind = 4;
    for r in children(stream, container.start, container.end) {
        match r.kind {
            TEXT_HEADER_ATOM => kind = u32_at(stream, r.start).unwrap_or(4),
            TEXT_CHARS_ATOM | TEXT_BYTES_ATOM => out.push(SlideText {
                kind,
                text: atom_text(stream, r),
            }),
            _ if r.container => collect_texts(stream, r, out, depth + 1),
            _ => {}
        }
    }
}

fn atom_text(stream: &[u8], r: Record) -> String {
    let body = &stream[r.start..r.end];
    if r.kind == TEXT_CHARS_ATOM {
        let units: Vec<u16> = body
            .as_chunks::<2>()
            .0
            .iter()
            .map(|c| u16::from_le_bytes(*c))
            .collect();
        String::from_utf16_lossy(&units)
    } else {
        body.iter().copied().map(cp1252).collect()
    }
}

/// A slide's texts as its title's blocks and the rest.
fn ppt_blocks(texts: Vec<SlideText>) -> (Vec<Block>, Vec<Block>) {
    let mut title = Vec::new();
    let mut rest = Vec::new();
    for SlideText { kind, text } in texts {
        // 0 title, 6 centered title, 5 centered body (subtitle), 1, 7 and
        // 8 body, 2 notes, 4 other text.
        let (style, bulleted) = match kind {
            0 => (Style::Heading(1), false),
            6 => (Style::Title, false),
            5 => (Style::Subtitle, false),
            1 | 7 | 8 => (Style::Normal, true),
            2 => continue,
            _ => (Style::Normal, false),
        };
        for line in text.split('\r') {
            let line: String = line
                .chars()
                .map(|c| if c == '\u{0b}' { '\n' } else { symbol(c) })
                .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
                .collect();
            if line.trim().is_empty() {
                continue;
            }
            let block = Block::Paragraph(Paragraph {
                style,
                list: bulleted.then(|| (0, bullet(0).to_owned())),
                runs: vec![Run {
                    text: line,
                    ..Run::default()
                }],
                ..Paragraph::default()
            });
            if matches!(kind, 0 | 6) {
                title.push(block);
            } else {
                rest.push(block);
            }
        }
    }
    (title, rest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{odp as odp_file, ppt as ppt_file, pptx as pptx_file};

    /// Each block as a line: "== 1" for a slide, the style, list marker
    /// and text of a paragraph, the cells of a table.
    fn lines(doc: &Document) -> Vec<String> {
        doc.blocks
            .iter()
            .map(|b| match b {
                Block::Slide(n) => format!("== {n}"),
                Block::Paragraph(p) => format!(
                    "{:?} {}{}",
                    p.style,
                    p.list
                        .as_ref()
                        .map_or(String::new(), |(l, m)| format!("{l}{m} ")),
                    p.text()
                ),
                Block::Table(rows) => rows
                    .iter()
                    .map(|r| {
                        r.iter()
                            .map(|c| c.iter().map(Paragraph::text).collect::<String>())
                            .collect::<Vec<_>>()
                            .join("|")
                    })
                    .collect::<Vec<_>>()
                    .join("/"),
            })
            .collect()
    }

    #[test]
    fn pptx_slides_in_show_order_with_titles_first() {
        let doc = open(pptx_file()).unwrap();
        assert_eq!(
            lines(&doc),
            [
                "== 1",
                "Title Roadmap",
                "Q1|42",
                "== 2",
                "Heading(1) Goals",
                "Normal 0• Grow 20%",
                "Normal 1◦ In Pune & Delhi",
            ]
        );
        let Block::Paragraph(p) = &doc.blocks[5] else {
            panic!();
        };
        assert!(p.runs.iter().any(|r| r.bold && r.text == "20%"));
    }

    #[test]
    fn ppt_slides_read_from_their_shapes() {
        let doc = open(ppt_file()).unwrap();
        assert_eq!(
            lines(&doc),
            ["== 1", "Heading(1) Plan", "Normal 0• One", "Normal 0• Two"]
        );
    }

    #[test]
    fn odp_pages_without_notes() {
        let doc = open(odp_file()).unwrap();
        assert_eq!(
            lines(&doc),
            [
                "== 1",
                "Heading(1) Roadmap",
                "Subtitle October",
                "== 2",
                "Heading(1) Goals",
                "Normal 0• Grow",
            ]
        );
    }

    #[test]
    fn broken_and_encrypted_slides_are_invalid() {
        assert_eq!(open(b"junk".to_vec()), Err(Invalid));
        let mut ppt = ppt_file();
        ppt.truncate(ppt.len() / 2);
        assert!(open(ppt).is_err());
        assert_eq!(open(crate::tests::docx()), Err(Invalid));
    }

    #[test]
    fn damaged_files_never_panic() {
        for file in [ppt_file(), crate::tests::doc()] {
            for at in (0..file.len()).step_by(97) {
                let mut bytes = file.clone();
                bytes[at] ^= 0xa5;
                let _ = open(bytes.clone());
                let _ = crate::word::open(bytes);
            }
        }
    }

    #[test]
    fn numbering_schemes() {
        assert_eq!(numbered("arabicPeriod", 3), "3.");
        assert_eq!(numbered("alphaLcParenR", 2), "b)");
        assert_eq!(numbered("romanUcParenBoth", 4), "(IV)");
        assert_eq!(numbered("arabicPlain", 7), "7");
    }
}
