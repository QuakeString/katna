// SPDX-License-Identifier: GPL-3.0-or-later

//! Word processor documents for the viewer: Word (docx) and OpenDocument
//! text (odt). Both are zip files of XML; this reads the text with its
//! headings, lists, tables and bold/italic/underline/strike-through, and
//! leaves out pictures, headers, footers, notes and comments. The viewer
//! lays the result out as one long page.

use std::collections::HashMap;
use std::io::{Cursor, Read};

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use zip::ZipArchive;

/// At most this many paragraphs are shown...
pub const MAX_PARAGRAPHS: usize = 20_000;
/// ...and this many characters.
pub const MAX_CHARS: usize = 2_000_000;
/// No XML part larger than this is read (unpacked).
const MAX_PART_BYTES: u64 = 64 * 1024 * 1024;
/// Table rows are at most this many cells wide.
const MAX_TABLE_COLUMNS: usize = 64;

/// A document as blocks: paragraphs and tables.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Document {
    pub blocks: Vec<Block>,
    /// Paragraphs or text past the limits were left out.
    pub cut: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Paragraph(Paragraph),
    /// Rows of cells; each cell is its paragraphs.
    Table(Vec<Vec<Vec<Paragraph>>>),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Paragraph {
    pub style: Style,
    pub align: Align,
    /// A list item: its level (from 0) and marker ("•", "3.", "b)").
    pub list: Option<(u8, String)>,
    pub runs: Vec<Run>,
}

impl Paragraph {
    pub fn text(&self) -> String {
        self.runs.iter().map(|r| r.text.as_str()).collect()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Style {
    #[default]
    Normal,
    Title,
    Subtitle,
    /// Heading 1 to 6.
    Heading(u8),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
}

/// A stretch of text in one look.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Run {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
}

impl Run {
    fn same_look(&self, other: &Run) -> bool {
        (self.bold, self.italic, self.underline, self.strike)
            == (other.bold, other.italic, other.underline, other.strike)
    }
}

/// Why a document cannot be shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Invalid;

/// Reads a docx or odt file; the format is found from what is inside.
pub fn open(bytes: Vec<u8>) -> Result<Document, Invalid> {
    let mut zip = ZipArchive::new(Cursor::new(bytes)).map_err(|_| Invalid)?;
    if zip.index_for_name("word/document.xml").is_some() {
        let body = part(&mut zip, "word/document.xml").ok_or(Invalid)?;
        let styles = part(&mut zip, "word/styles.xml").unwrap_or_default();
        let numbering = part(&mut zip, "word/numbering.xml").unwrap_or_default();
        Ok(docx(&body, &styles, &numbering))
    } else if zip.index_for_name("content.xml").is_some() {
        let content = part(&mut zip, "content.xml").ok_or(Invalid)?;
        let styles = part(&mut zip, "styles.xml").unwrap_or_default();
        Ok(odt(&content, &styles))
    } else {
        Err(Invalid)
    }
}

/// One file of the zip as text, if it is there and not too large.
fn part(zip: &mut ZipArchive<Cursor<Vec<u8>>>, name: &str) -> Option<String> {
    let file = zip.by_name(name).ok()?;
    let mut text = String::new();
    file.take(MAX_PART_BYTES).read_to_string(&mut text).ok()?;
    Some(text)
}

/// The value of attribute `name` (without its prefix).
fn attr(e: &BytesStart, name: &str) -> Option<String> {
    e.attributes()
        .flatten()
        .find(|a| a.key.local_name().as_ref() == name.as_bytes())
        .and_then(|a| {
            a.normalized_value(quick_xml::XmlVersion::Implicit1_0)
                .ok()
                .map(|v| v.into_owned())
        })
}

fn local(e: &BytesStart) -> String {
    String::from_utf8_lossy(e.local_name().as_ref()).into_owned()
}

/// A Word on/off property: on unless `w:val` says otherwise.
fn toggle(e: &BytesStart) -> bool {
    !matches!(
        attr(e, "val").as_deref(),
        Some("0" | "false" | "off" | "none")
    )
}

/// Collects paragraphs into blocks, tables included, within the limits.
#[derive(Default)]
struct Builder {
    doc: Document,
    chars: usize,
    paragraph: Option<Paragraph>,
    /// Open tables, innermost last: rows of cells of paragraphs.
    tables: Vec<Vec<Vec<Vec<Paragraph>>>>,
    paragraphs: usize,
}

impl Builder {
    fn full(&self) -> bool {
        self.paragraphs >= MAX_PARAGRAPHS || self.chars >= MAX_CHARS
    }

    fn start_paragraph(&mut self) {
        self.end_paragraph();
        self.paragraph = Some(Paragraph::default());
    }

    fn text(&mut self, text: &str, look: &Run) {
        let Some(paragraph) = &mut self.paragraph else {
            return;
        };
        if self.chars >= MAX_CHARS {
            self.doc.cut = true;
            return;
        }
        self.chars += text.len();
        match paragraph.runs.last_mut() {
            Some(last) if last.same_look(look) => last.text.push_str(text),
            _ => paragraph.runs.push(Run {
                text: text.to_owned(),
                ..look.clone()
            }),
        }
    }

    fn end_paragraph(&mut self) {
        let Some(paragraph) = self.paragraph.take() else {
            return;
        };
        if self.full() {
            self.doc.cut = true;
            return;
        }
        self.paragraphs += 1;
        match self.tables.last_mut() {
            Some(table) => {
                if table.is_empty() {
                    table.push(Vec::new());
                }
                let row = table.last_mut().expect("a row");
                if row.is_empty() {
                    row.push(Vec::new());
                }
                row.last_mut().expect("a cell").push(paragraph);
            }
            None => self.doc.blocks.push(Block::Paragraph(paragraph)),
        }
    }

    fn start_table(&mut self) {
        self.end_paragraph();
        self.tables.push(Vec::new());
    }

    fn start_row(&mut self) {
        if let Some(table) = self.tables.last_mut() {
            table.push(Vec::new());
        }
    }

    fn start_cell(&mut self, repeat: usize) {
        self.end_paragraph();
        if let Some(row) = self.tables.last_mut().and_then(|t| t.last_mut()) {
            for _ in 0..repeat.max(1) {
                if row.len() < MAX_TABLE_COLUMNS {
                    row.push(Vec::new());
                }
            }
        }
    }

    fn end_table(&mut self) {
        self.end_paragraph();
        let Some(table) = self.tables.pop() else {
            return;
        };
        let table: Vec<_> = table.into_iter().filter(|row| !row.is_empty()).collect();
        if table.is_empty() {
            return;
        }
        match self.tables.last_mut() {
            // A table in a table: its cells' paragraphs join the outer cell.
            Some(outer) => {
                let paragraphs = table.into_iter().flatten().flatten();
                if let Some(cell) = outer.last_mut().and_then(|row| row.last_mut()) {
                    cell.extend(paragraphs);
                }
            }
            None => self.doc.blocks.push(Block::Table(table)),
        }
    }

    fn finish(mut self) -> Document {
        self.end_paragraph();
        while !self.tables.is_empty() {
            self.end_table();
        }
        self.doc
    }
}

// ---- Word (docx) ----

/// What a Word style gives a paragraph or run.
#[derive(Debug, Clone, Default)]
struct WordStyle {
    name: String,
    based_on: Option<String>,
    outline: Option<u8>,
    bold: Option<bool>,
    italic: Option<bool>,
    /// List styles ("List Number") carry the list and level themselves.
    num_id: Option<String>,
    level: Option<u8>,
}

fn word_styles(xml: &str) -> HashMap<String, WordStyle> {
    let mut styles = HashMap::new();
    let mut reader = Reader::from_str(xml);
    let mut current: Option<(String, WordStyle)> = None;
    let mut in_rpr = false;
    loop {
        let event = reader.read_event();
        let empty = matches!(event, Ok(Event::Empty(_)));
        match event {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => match local(&e).as_str() {
                "style" => {
                    if let Some((id, style)) = current.take() {
                        styles.insert(id, style);
                    }
                    current = attr(&e, "styleId").map(|id| (id, WordStyle::default()));
                }
                "name" => {
                    if let Some((_, style)) = &mut current {
                        style.name = attr(&e, "val").unwrap_or_default().to_lowercase();
                    }
                }
                "basedOn" => {
                    if let Some((_, style)) = &mut current {
                        style.based_on = attr(&e, "val");
                    }
                }
                "outlineLvl" => {
                    if let Some((_, style)) = &mut current {
                        style.outline = attr(&e, "val").and_then(|v| v.parse().ok());
                    }
                }
                "numId" => {
                    if let Some((_, style)) = &mut current {
                        style.num_id = attr(&e, "val");
                    }
                }
                "ilvl" => {
                    if let Some((_, style)) = &mut current {
                        style.level = attr(&e, "val").and_then(|v| v.parse().ok());
                    }
                }
                "rPr" => in_rpr = !empty,
                "b" if in_rpr => {
                    if let Some((_, style)) = &mut current {
                        style.bold = Some(toggle(&e));
                    }
                }
                "i" if in_rpr => {
                    if let Some((_, style)) = &mut current {
                        style.italic = Some(toggle(&e));
                    }
                }
                _ => {}
            },
            Ok(Event::End(e)) => match e.local_name().as_ref() {
                b"rPr" => in_rpr = false,
                b"style" => {
                    if let Some((id, style)) = current.take() {
                        styles.insert(id, style);
                    }
                }
                _ => {}
            },
            Ok(Event::Eof) | Err(_) => break,
            _ => {}
        }
    }
    styles
}

/// A style's value, looking through the styles it is based on.
fn inherited<T>(
    styles: &HashMap<String, WordStyle>,
    id: &str,
    get: impl Fn(&WordStyle) -> Option<T>,
) -> Option<T> {
    let mut id = id.to_owned();
    for _ in 0..8 {
        let style = styles.get(&id)?;
        if let Some(value) = get(style) {
            return Some(value);
        }
        id = style.based_on.clone()?;
    }
    None
}

fn word_paragraph_style(styles: &HashMap<String, WordStyle>, id: &str) -> Style {
    let name = styles
        .get(id)
        .map(|s| s.name.clone())
        .unwrap_or_else(|| id.to_lowercase());
    let compact = name.replace(' ', "");
    if compact == "title" {
        return Style::Title;
    }
    if compact == "subtitle" {
        return Style::Subtitle;
    }
    if let Some(n) = compact
        .strip_prefix("heading")
        .and_then(|n| n.parse::<u8>().ok())
    {
        return Style::Heading(n.clamp(1, 6));
    }
    match inherited(styles, id, |s| s.outline) {
        Some(level) if level < 9 => Style::Heading((level + 1).min(6)),
        _ => Style::Normal,
    }
}

/// Word list numbering: for each list (`numId`), each level's format and
/// text ("%1.").
#[derive(Default)]
struct Numbering {
    /// abstractNumId → level → (format, text, start)
    abstracts: HashMap<String, HashMap<u8, (String, String, u32)>>,
    /// numId → abstractNumId
    lists: HashMap<String, String>,
}

fn word_numbering(xml: &str) -> Numbering {
    let mut numbering = Numbering::default();
    let mut reader = Reader::from_str(xml);
    let mut abstract_id: Option<String> = None;
    let mut level: Option<(u8, String, String, u32)> = None;
    let mut num_id: Option<String> = None;
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => match local(&e).as_str() {
                "abstractNum" => abstract_id = attr(&e, "abstractNumId"),
                "lvl" => {
                    level = attr(&e, "ilvl")
                        .and_then(|v| v.parse().ok())
                        .map(|l| (l, "decimal".to_owned(), format!("%{}.", l + 1), 1));
                }
                "numFmt" => {
                    if let Some(level) = &mut level {
                        level.1 = attr(&e, "val").unwrap_or_default();
                    }
                }
                "lvlText" => {
                    if let Some(level) = &mut level {
                        level.2 = attr(&e, "val").unwrap_or_default();
                    }
                }
                "start" => {
                    if let Some(level) = &mut level {
                        level.3 = attr(&e, "val").and_then(|v| v.parse().ok()).unwrap_or(1);
                    }
                }
                "num" => num_id = attr(&e, "numId"),
                "abstractNumId" => {
                    if let (Some(num), Some(target)) = (num_id.clone(), attr(&e, "val")) {
                        numbering.lists.insert(num, target);
                    }
                }
                _ => {}
            },
            Ok(Event::End(e)) => match e.local_name().as_ref() {
                b"lvl" => {
                    if let (Some(id), Some((l, format, text, start))) =
                        (abstract_id.clone(), level.take())
                    {
                        numbering
                            .abstracts
                            .entry(id)
                            .or_default()
                            .insert(l, (format, text, start));
                    }
                }
                b"abstractNum" => abstract_id = None,
                b"num" => num_id = None,
                _ => {}
            },
            Ok(Event::Eof) | Err(_) => break,
            _ => {}
        }
    }
    numbering
}

/// Counts list items so each gets its number.
#[derive(Default)]
struct Counters(HashMap<String, [u32; 10]>);

impl Counters {
    /// The marker of the next item of list `key` at `level`, given each
    /// level's (format, text, start).
    fn next(
        &mut self,
        key: &str,
        level: u8,
        levels: &dyn Fn(u8) -> (String, String, u32),
    ) -> String {
        let level = level.min(9);
        let counts = self.0.entry(key.to_owned()).or_insert([0; 10]);
        let (format, text, start) = levels(level);
        counts[level as usize] = if counts[level as usize] == 0 {
            start
        } else {
            counts[level as usize] + 1
        };
        for deeper in counts.iter_mut().skip(level as usize + 1) {
            *deeper = 0;
        }
        if format == "bullet" || format == "none" {
            return bullet(level).to_owned();
        }
        let mut marker = text;
        for l in 0..=level {
            let (format, _, start) = levels(l);
            let n = counts[l as usize].max(start);
            marker = marker.replace(&format!("%{}", l + 1), &number_as(&format, n));
        }
        if marker.trim().is_empty() {
            bullet(level).to_owned()
        } else {
            marker
        }
    }
}

fn bullet(level: u8) -> &'static str {
    ["•", "◦", "▪"][level as usize % 3]
}

/// `n` written as a list format asks: 3, c, C, iii, III.
fn number_as(format: &str, n: u32) -> String {
    match format {
        "lowerLetter" | "a" => letters(n).to_lowercase(),
        "upperLetter" | "A" => letters(n),
        "lowerRoman" | "i" => roman(n).to_lowercase(),
        "upperRoman" | "I" => roman(n),
        _ => n.to_string(),
    }
}

fn letters(n: u32) -> String {
    let n = n.max(1) - 1;
    let letter = char::from(b'A' + (n % 26) as u8);
    letter.to_string().repeat(n as usize / 26 + 1)
}

fn roman(mut n: u32) -> String {
    const DIGITS: [(u32, &str); 13] = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    if n == 0 || n > 3999 {
        return n.to_string();
    }
    let mut out = String::new();
    for (value, digits) in DIGITS {
        while n >= value {
            out.push_str(digits);
            n -= value;
        }
    }
    out
}

/// Elements whose content is not part of the running text.
const WORD_SKIP: [&str; 9] = [
    "drawing",
    "pict",
    "object",
    "AlternateContent",
    "delText",
    "instrText",
    "footnoteReference",
    "endnoteReference",
    "commentReference",
];

fn docx(body: &str, styles: &str, numbering: &str) -> Document {
    let styles = word_styles(styles);
    let numbering = word_numbering(numbering);
    let mut counters = Counters::default();
    let mut out = Builder::default();
    let mut reader = Reader::from_str(body);
    let mut skip = 0usize;
    // Where we are: in a paragraph's properties, a run, its properties,
    // its text.
    let (mut in_ppr, mut in_run, mut in_rpr, mut in_text) = (false, false, false, false);
    let mut look = Run::default();
    // The paragraph's own list and level, if it names them.
    let mut list: (Option<String>, Option<u8>) = (None, None);
    let mut style_id = String::new();
    loop {
        let event = match reader.read_event() {
            Ok(Event::Eof) | Err(_) => break,
            Ok(event) => event,
        };
        if out.full() {
            out.doc.cut = true;
            break;
        }
        let empty = matches!(event, Event::Empty(_));
        match event {
            Event::Start(_) if skip > 0 => skip += 1,
            Event::End(_) if skip > 0 => skip -= 1,
            _ if skip > 0 => {}
            Event::Start(e) | Event::Empty(e) => {
                let name = local(&e);
                if WORD_SKIP.contains(&name.as_str()) {
                    if !empty {
                        skip = 1;
                    }
                    continue;
                }
                match name.as_str() {
                    "p" => {
                        out.start_paragraph();
                        list = (None, None);
                        style_id.clear();
                        if empty {
                            out.end_paragraph();
                        }
                    }
                    "pPr" => in_ppr = !empty,
                    "pStyle" if in_ppr => {
                        style_id = attr(&e, "val").unwrap_or_default();
                        if let Some(p) = &mut out.paragraph {
                            p.style = word_paragraph_style(&styles, &style_id);
                        }
                    }
                    "outlineLvl" if in_ppr => {
                        let level = attr(&e, "val").and_then(|v| v.parse::<u8>().ok());
                        if let (Some(p), Some(level)) = (&mut out.paragraph, level)
                            && level < 9
                        {
                            p.style = Style::Heading((level + 1).min(6));
                        }
                    }
                    "jc" if in_ppr && !in_rpr => {
                        if let Some(p) = &mut out.paragraph {
                            p.align = match attr(&e, "val").as_deref() {
                                Some("center") => Align::Center,
                                Some("right" | "end") => Align::End,
                                _ => Align::Start,
                            };
                        }
                    }
                    "ilvl" if in_ppr => {
                        let level = attr(&e, "val").and_then(|v| v.parse().ok()).unwrap_or(0);
                        list.1 = Some(level);
                    }
                    "numId" if in_ppr => {
                        let id = attr(&e, "val").unwrap_or_default();
                        list.0 = Some(id);
                    }
                    "r" => {
                        in_run = !empty;
                        look = Run {
                            bold: inherited(&styles, &style_id, |s| s.bold).unwrap_or(false),
                            italic: inherited(&styles, &style_id, |s| s.italic).unwrap_or(false),
                            ..Run::default()
                        };
                    }
                    "rPr" => in_rpr = !empty,
                    "rStyle" if in_rpr && in_run => {
                        let id = attr(&e, "val").unwrap_or_default();
                        if let Some(b) = inherited(&styles, &id, |s| s.bold) {
                            look.bold = b;
                        }
                        if let Some(i) = inherited(&styles, &id, |s| s.italic) {
                            look.italic = i;
                        }
                    }
                    "b" if in_rpr && in_run => look.bold = toggle(&e),
                    "i" if in_rpr && in_run => look.italic = toggle(&e),
                    "u" if in_rpr && in_run => look.underline = toggle(&e),
                    "strike" | "dstrike" if in_rpr && in_run => look.strike = toggle(&e),
                    "t" if in_run => in_text = !empty,
                    "tab" if in_run && !in_rpr => out.text("    ", &look),
                    "br" | "cr" if in_run && !in_rpr => {
                        if attr(&e, "type").as_deref() != Some("page") {
                            out.text("\n", &look);
                        }
                    }
                    "noBreakHyphen" if in_run => out.text("-", &look),
                    "softHyphen" => {}
                    "tbl" => out.start_table(),
                    "tr" => out.start_row(),
                    "tc" => out.start_cell(1),
                    _ => {}
                }
            }
            Event::End(e) => match e.local_name().as_ref() {
                b"p" => {
                    let (id, level) = std::mem::take(&mut list);
                    let id = id.or_else(|| inherited(&styles, &style_id, |s| s.num_id.clone()));
                    let level = level
                        .or_else(|| inherited(&styles, &style_id, |s| s.level))
                        .unwrap_or(0);
                    if let (Some(p), Some(id)) = (&mut out.paragraph, id)
                        && id != "0"
                        && !id.is_empty()
                    {
                        let abstract_id = numbering.lists.get(&id).cloned().unwrap_or_default();
                        let levels = numbering.abstracts.get(&abstract_id);
                        let get = |l: u8| {
                            levels
                                .and_then(|levels| levels.get(&l).cloned())
                                .unwrap_or_else(|| ("bullet".to_owned(), String::new(), 1))
                        };
                        p.list = Some((level.min(8), counters.next(&id, level, &get)));
                    }
                    out.end_paragraph();
                }
                b"pPr" => in_ppr = false,
                b"r" => in_run = false,
                b"rPr" => in_rpr = false,
                b"t" => in_text = false,
                b"tbl" => out.end_table(),
                _ => {}
            },
            Event::Text(t) if in_text => {
                if let Ok(text) = t.xml10_content() {
                    out.text(&text, &look);
                }
            }
            Event::GeneralRef(r) if in_text => {
                if let Some(c) = entity(&r) {
                    out.text(c.encode_utf8(&mut [0; 4]), &look);
                }
            }
            Event::CData(t) if in_text => {
                if let Ok(text) = t.decode() {
                    out.text(&text, &look);
                }
            }
            _ => {}
        }
    }
    out.finish()
}

/// `&amp;`, `&#233;` and the other references XML knows.
fn entity(r: &quick_xml::events::BytesRef) -> Option<char> {
    if let Ok(Some(c)) = r.resolve_char_ref() {
        return Some(c);
    }
    match r.decode().ok()?.as_ref() {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        _ => None,
    }
}

// ---- OpenDocument (odt) ----

#[derive(Debug, Clone, Default)]
struct OdfStyle {
    parent: Option<String>,
    display: Option<String>,
    bold: Option<bool>,
    italic: Option<bool>,
    underline: Option<bool>,
    strike: Option<bool>,
    align: Option<Align>,
}

#[derive(Default)]
struct OdfStyles {
    styles: HashMap<String, OdfStyle>,
    /// List style → numbered levels (from 1) and their suffix (".").
    lists: HashMap<String, HashMap<u8, Option<(String, String)>>>,
}

impl OdfStyles {
    fn get<T>(&self, name: &str, get: impl Fn(&OdfStyle) -> Option<T>) -> Option<T> {
        let mut name = name.to_owned();
        for _ in 0..8 {
            let style = self.styles.get(&name)?;
            if let Some(value) = get(style) {
                return Some(value);
            }
            name = style.parent.clone()?;
        }
        None
    }

    fn look(&self, name: &str, mut base: Run) -> Run {
        if let Some(v) = self.get(name, |s| s.bold) {
            base.bold = v;
        }
        if let Some(v) = self.get(name, |s| s.italic) {
            base.italic = v;
        }
        if let Some(v) = self.get(name, |s| s.underline) {
            base.underline = v;
        }
        if let Some(v) = self.get(name, |s| s.strike) {
            base.strike = v;
        }
        base
    }

    /// Title and Subtitle, from the style or those it is based on.
    fn paragraph_style(&self, name: &str) -> Style {
        let mut name = name.to_owned();
        for _ in 0..8 {
            let shown = self
                .styles
                .get(&name)
                .and_then(|s| s.display.clone())
                .unwrap_or_else(|| name.replace("_20_", " "))
                .to_lowercase();
            match shown.as_str() {
                "title" => return Style::Title,
                "subtitle" => return Style::Subtitle,
                _ => {}
            }
            match self.styles.get(&name).and_then(|s| s.parent.clone()) {
                Some(parent) => name = parent,
                None => break,
            }
        }
        Style::Normal
    }
}

fn odf_styles(xmls: &[&str]) -> OdfStyles {
    let mut out = OdfStyles::default();
    for xml in xmls {
        let mut reader = Reader::from_str(xml);
        let mut current: Option<String> = None;
        let mut list: Option<String> = None;
        let mut in_body = false;
        loop {
            match reader.read_event() {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => match local(&e).as_str() {
                    // Styles come before the body; stop there.
                    "body" => in_body = true,
                    _ if in_body => {}
                    "style" => {
                        current = attr(&e, "name");
                        if let Some(name) = &current {
                            out.styles.insert(
                                name.clone(),
                                OdfStyle {
                                    parent: attr(&e, "parent-style-name"),
                                    display: attr(&e, "display-name"),
                                    ..Default::default()
                                },
                            );
                        }
                    }
                    "text-properties" => {
                        let Some(style) = current.as_ref().and_then(|n| out.styles.get_mut(n))
                        else {
                            continue;
                        };
                        if let Some(w) = attr(&e, "font-weight") {
                            style.bold =
                                Some(w == "bold" || w.parse::<u32>().is_ok_and(|w| w >= 600));
                        }
                        if let Some(s) = attr(&e, "font-style") {
                            style.italic = Some(s == "italic" || s == "oblique");
                        }
                        if let Some(u) = attr(&e, "text-underline-style") {
                            style.underline = Some(u != "none");
                        }
                        if let Some(s) = attr(&e, "text-line-through-style") {
                            style.strike = Some(s != "none");
                        }
                    }
                    "paragraph-properties" => {
                        let Some(style) = current.as_ref().and_then(|n| out.styles.get_mut(n))
                        else {
                            continue;
                        };
                        style.align = attr(&e, "text-align").map(|a| match a.as_str() {
                            "center" => Align::Center,
                            "end" | "right" => Align::End,
                            _ => Align::Start,
                        });
                    }
                    "list-style" => {
                        list = attr(&e, "name");
                        if let Some(name) = &list {
                            out.lists.insert(name.clone(), HashMap::new());
                        }
                    }
                    "list-level-style-number" | "list-level-style-bullet" => {
                        let numbered = local(&e) == "list-level-style-number";
                        let level = attr(&e, "level").and_then(|l| l.parse().ok()).unwrap_or(1);
                        if let Some(levels) = list.as_ref().and_then(|l| out.lists.get_mut(l)) {
                            let format = attr(&e, "num-format").unwrap_or_default();
                            let suffix = attr(&e, "num-suffix").unwrap_or_default();
                            levels.insert(
                                level,
                                (numbered && !format.is_empty()).then_some((format, suffix)),
                            );
                        }
                    }
                    _ => {}
                },
                Ok(Event::End(e)) => match e.local_name().as_ref() {
                    b"style" => current = None,
                    b"list-style" => list = None,
                    _ => {}
                },
                Ok(Event::Eof) | Err(_) => break,
                _ => {}
            }
        }
    }
    out
}

/// Elements whose content is not part of the running text.
const ODF_SKIP: [&str; 6] = [
    "note",
    "annotation",
    "tracked-changes",
    "frame",
    "sequence-decls",
    "bookmark-ref",
];

fn odt(content: &str, styles: &str) -> Document {
    let odf = odf_styles(&[styles, content]);
    let mut out = Builder::default();
    let mut reader = Reader::from_str(content);
    let mut skip = 0usize;
    let mut in_body = false;
    // Span looks, innermost last.
    let mut looks: Vec<Run> = Vec::new();
    // Open lists' styles, outermost first; a list item waiting for its
    // marker; each level's count.
    let mut lists: Vec<String> = Vec::new();
    let mut item_pending = false;
    let mut counters: Vec<u32> = Vec::new();
    loop {
        let event = match reader.read_event() {
            Ok(Event::Eof) | Err(_) => break,
            Ok(event) => event,
        };
        if out.full() {
            out.doc.cut = true;
            break;
        }
        let empty = matches!(event, Event::Empty(_));
        match event {
            Event::Start(_) if skip > 0 => skip += 1,
            Event::End(_) if skip > 0 => skip -= 1,
            _ if skip > 0 => {}
            Event::Start(e) | Event::Empty(e) => {
                let name = local(&e);
                if name == "body" {
                    in_body = true;
                    continue;
                }
                if !in_body {
                    continue;
                }
                if ODF_SKIP.contains(&name.as_str()) {
                    if !empty {
                        skip = 1;
                    }
                    continue;
                }
                match name.as_str() {
                    "p" | "h" => {
                        out.start_paragraph();
                        let style_name = attr(&e, "style-name").unwrap_or_default();
                        let base = odf.look(&style_name, Run::default());
                        looks.clear();
                        looks.push(base);
                        if let Some(p) = &mut out.paragraph {
                            p.style = if name == "h" {
                                let level = attr(&e, "outline-level")
                                    .and_then(|l| l.parse::<u8>().ok())
                                    .unwrap_or(1);
                                Style::Heading(level.clamp(1, 6))
                            } else {
                                odf.paragraph_style(&style_name)
                            };
                            p.align = odf.get(&style_name, |s| s.align).unwrap_or_default();
                            if item_pending && !lists.is_empty() {
                                item_pending = false;
                                let level = lists.len() - 1;
                                let style = &lists[0];
                                let format = odf
                                    .lists
                                    .get(style)
                                    .and_then(|levels| levels.get(&(level as u8 + 1)).cloned())
                                    .flatten();
                                counters.resize(lists.len(), 0);
                                counters[level] += 1;
                                let marker = match format {
                                    Some((format, suffix)) => {
                                        format!("{}{suffix}", number_as(&format, counters[level]))
                                    }
                                    None => bullet(level as u8).to_owned(),
                                };
                                p.list = Some((level.min(8) as u8, marker));
                            }
                        }
                        if empty {
                            out.end_paragraph();
                        }
                    }
                    "span" | "a" => {
                        let base = looks.last().cloned().unwrap_or_default();
                        let look = match attr(&e, "style-name") {
                            Some(style) => odf.look(&style, base),
                            None => base,
                        };
                        if !empty {
                            looks.push(look);
                        }
                    }
                    "s" => {
                        let count = attr(&e, "c").and_then(|c| c.parse().ok()).unwrap_or(1usize);
                        let look = looks.last().cloned().unwrap_or_default();
                        out.text(&" ".repeat(count.min(100)), &look);
                    }
                    "tab" => {
                        let look = looks.last().cloned().unwrap_or_default();
                        out.text("    ", &look);
                    }
                    "line-break" => {
                        let look = looks.last().cloned().unwrap_or_default();
                        out.text("\n", &look);
                    }
                    "list" if !empty => {
                        // A new list starts counting from 1 unless it
                        // carries on from the one before.
                        let carries_on = attr(&e, "continue-numbering").as_deref() == Some("true")
                            || attr(&e, "continue-list").is_some();
                        if lists.is_empty() && !carries_on {
                            counters.clear();
                        }
                        let style = attr(&e, "style-name")
                            .or_else(|| lists.first().cloned())
                            .unwrap_or_default();
                        lists.push(style);
                        counters.truncate(lists.len() - 1);
                    }
                    "list-item" => item_pending = true,
                    "list-header" => item_pending = false,
                    "table" if !empty => out.start_table(),
                    "table-row" => out.start_row(),
                    "table-cell" | "covered-table-cell" => {
                        let repeat = attr(&e, "number-columns-repeated")
                            .and_then(|n| n.parse().ok())
                            .unwrap_or(1usize);
                        // Long runs of repeated empty cells pad the row's end.
                        let repeat = if empty { repeat.min(1) } else { repeat };
                        out.start_cell(repeat);
                    }
                    _ => {}
                }
            }
            Event::End(e) => match e.local_name().as_ref() {
                b"p" | b"h" => {
                    out.end_paragraph();
                    looks.clear();
                }
                b"span" | b"a" => {
                    if looks.len() > 1 {
                        looks.pop();
                    }
                }
                b"list" => {
                    lists.pop();
                }
                b"table" => out.end_table(),
                _ => {}
            },
            Event::Text(t) if out.paragraph.is_some() => {
                if let Ok(text) = t.xml10_content() {
                    // ODF collapses runs of white space; text:s spells them.
                    let text = collapse(&text);
                    let look = looks.last().cloned().unwrap_or_default();
                    out.text(&text, &look);
                }
            }
            Event::GeneralRef(r) if out.paragraph.is_some() => {
                if let Some(c) = entity(&r) {
                    let look = looks.last().cloned().unwrap_or_default();
                    out.text(c.encode_utf8(&mut [0; 4]), &look);
                }
            }
            _ => {}
        }
    }
    out.finish()
}

fn collapse(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut space = false;
    for c in text.chars() {
        if c.is_whitespace() {
            if !space {
                out.push(' ');
            }
            space = true;
        } else {
            out.push(c);
            space = false;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_documents() {
        let doc = open(crate::tests::docx()).unwrap();
        let paragraphs: Vec<&Paragraph> = doc
            .blocks
            .iter()
            .filter_map(|b| match b {
                Block::Paragraph(p) => Some(p),
                Block::Table(_) => None,
            })
            .collect();
        assert_eq!(paragraphs[0].style, Style::Title);
        assert_eq!(paragraphs[0].align, Align::Center);
        assert_eq!(paragraphs[0].text(), "Quarterly report");
        assert_eq!(paragraphs[1].style, Style::Heading(1));
        let mixed = &paragraphs[2];
        assert_eq!(mixed.text(), "Plain bold italic & more");
        assert!(mixed.runs.iter().any(|r| r.bold && r.text == "bold"));
        assert!(
            mixed
                .runs
                .iter()
                .any(|r| r.italic && r.text.contains("italic"))
        );
        assert_eq!(paragraphs[3].list, Some((0, "1.".to_owned())));
        assert_eq!(paragraphs[4].list, Some((1, "a)".to_owned())));
        assert_eq!(paragraphs[5].list, Some((0, "2.".to_owned())));
        assert_eq!(paragraphs[6].list, Some((0, "•".to_owned())));
        let table = doc
            .blocks
            .iter()
            .find_map(|b| match b {
                Block::Table(t) => Some(t),
                Block::Paragraph(_) => None,
            })
            .unwrap();
        assert_eq!(table.len(), 2);
        assert_eq!(table[1][1][0].text(), "4");
        assert!(!doc.cut);
    }

    #[test]
    fn opendocument_text() {
        let doc = open(crate::tests::odt()).unwrap();
        let texts: Vec<String> = doc
            .blocks
            .iter()
            .map(|b| match b {
                Block::Paragraph(p) => p.text(),
                Block::Table(t) => format!("table {}x{}", t.len(), t[0].len()),
            })
            .collect();
        assert_eq!(
            texts,
            [
                "Minutes",
                "Agenda",
                "We met at  noon & ate.",
                "First",
                "Second",
                "table 2x2",
            ]
        );
        let Block::Paragraph(first) = &doc.blocks[0] else {
            panic!()
        };
        assert_eq!(first.style, Style::Title);
        let Block::Paragraph(heading) = &doc.blocks[1] else {
            panic!()
        };
        assert_eq!(heading.style, Style::Heading(1));
        let Block::Paragraph(body) = &doc.blocks[2] else {
            panic!()
        };
        assert!(body.runs.iter().any(|r| r.bold && r.text == "noon"));
        let Block::Paragraph(second) = &doc.blocks[4] else {
            panic!()
        };
        assert_eq!(second.list, Some((0, "2.".to_owned())));
    }

    #[test]
    fn numbers_as_letters_and_roman() {
        assert_eq!(number_as("lowerLetter", 3), "c");
        assert_eq!(number_as("upperLetter", 27), "AA");
        assert_eq!(number_as("lowerRoman", 14), "xiv");
        assert_eq!(number_as("decimal", 7), "7");
    }

    #[test]
    fn garbage_is_invalid() {
        assert_eq!(open(b"no zip here".to_vec()), Err(Invalid));
    }
}
