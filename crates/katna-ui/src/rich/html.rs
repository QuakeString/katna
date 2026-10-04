// SPDX-License-Identifier: GPL-3.0-or-later

//! A [`Doc`] as mail HTML and as plain text, and back from the HTML this
//! writes (signatures are stored that way) and from plain text. The HTML
//! uses inline styles only, which mail readers keep. No GPUI here.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::ops::Range;
use std::sync::Arc;

use super::doc::{
    Align, Block, CharStyle, Doc, Font, HtmlBlock, Image, ImageSize, List, MAX_INDENT, Para,
    ParaStyle, Size, Table, image_size, list_marker, list_numbers,
};

/// Widest an image is sent at when it fits the text.
const BEST_FIT_WIDTH: f32 = 600.0;
const QUOTE_STYLE: &str =
    "margin:0px 0px 0px 0.8ex;border-left:1px solid rgb(204,204,204);padding-left:1ex";
const CELL_STYLE: &str = "border:1px solid rgb(204,204,204);padding:4px 8px;min-width:40px";
/// Around a designed block, so that it reads back as one when a draft or
/// a stored signature is opened again.
pub const HTML_START: &str = "<!--katna-html-->";
const HTML_END: &str = "<!--/katna-html-->";
/// How a designed block names its `N`th picture.
const PICTURE_SRC: &str = "cid:katna-";

/// The document as the body of an HTML message. `image_src` gives each
/// image's address (a `cid:` in mail, a `data:` URI when stored).
pub fn to_html(doc: &Doc, image_src: &dyn Fn(&Image) -> String) -> String {
    let mut out = String::from("<div dir=\"ltr\">");
    let numbers = list_numbers(doc);
    let mut writer = Writer {
        out: &mut out,
        lists: Vec::new(),
        quote: 0,
        signature: false,
    };
    for (ix, block) in doc.blocks.iter().enumerate() {
        let style = match block {
            Block::Para(para) => para.style,
            // A designed signature stays inside the signature.
            Block::Html(_) => ParaStyle {
                quote: writer.quote,
                signature: writer.signature,
                ..ParaStyle::default()
            },
            _ => ParaStyle::default(),
        };
        writer.enter(&style);
        match block {
            Block::Para(para) => writer.para(para, numbers[ix]),
            Block::Table(table) => writer.table(table),
            Block::Image(image) => {
                let width = image.display_width(BEST_FIT_WIDTH).round() as u32;
                let _ = write!(
                    writer.out,
                    "<div><img src=\"{}\" alt=\"{}\" width=\"{width}\" style=\"max-width:100%\"></div>",
                    escape(&image_src(image)),
                    escape(&image.name),
                );
            }
            Block::Html(block) => {
                writer.out.push_str(HTML_START);
                writer.out.push_str(&with_pictures(block, image_src));
                writer.out.push_str(HTML_END);
            }
        }
    }
    writer.enter(&ParaStyle::default());
    out.push_str("</div>");
    out
}

/// A designed block's HTML with each `cid:katna-N` given its picture's
/// address.
fn with_pictures(block: &HtmlBlock, image_src: &dyn Fn(&Image) -> String) -> String {
    let html = &*block.html;
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(at) = rest.find(PICTURE_SRC) {
        out.push_str(&rest[..at]);
        let after = &rest[at + PICTURE_SRC.len()..];
        let digits = after.len() - after.trim_start_matches(|c: char| c.is_ascii_digit()).len();
        match after[..digits]
            .parse::<usize>()
            .ok()
            .and_then(|n| block.images.get(n))
        {
            Some(image) => out.push_str(&escape(&image_src(image))),
            None => out.push_str(&rest[at..at + PICTURE_SRC.len() + digits]),
        }
        rest = &after[digits..];
    }
    out.push_str(rest);
    out
}

/// A designed block from mail-safe HTML whose pictures are `data:` URIs
/// (in `src` attributes): they become the block's pictures.
pub fn html_block(html: &str, next_id: &mut u64) -> HtmlBlock {
    let mut images = Vec::new();
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while let Some(at) = find_ascii_ci(rest, "src=") {
        let value = &rest[at + 4..];
        let quote = value.chars().next().filter(|c| *c == '"' || *c == '\'');
        let Some(quote) = quote else {
            out.push_str(&rest[..at + 4]);
            rest = value;
            continue;
        };
        let inner = &value[1..];
        let end = inner.find(quote).unwrap_or(inner.len());
        let src = decode_entities(&inner[..end]);
        out.push_str(&rest[..at + 4]);
        out.push(quote);
        match image_from_data_uri(src.trim(), String::new(), None, next_id) {
            Some(image) => {
                let _ = write!(out, "{PICTURE_SRC}{}", images.len());
                images.push(image);
            }
            None => out.push_str(&inner[..end]),
        }
        rest = &inner[end..];
    }
    out.push_str(rest);
    HtmlBlock {
        html: out.into(),
        text: designed_text(html).into(),
        images,
    }
}

/// What designed HTML says, a line each: its tables are layout, so each
/// cell's lines stand on their own.
fn designed_text(html: &str) -> String {
    let mut scratch = 0;
    let doc = read_html(html, &mut scratch, false);
    let mut lines: Vec<String> = Vec::new();
    for block in doc.blocks {
        match block {
            Block::Table(table) => {
                let cells = table.rows.into_iter().flatten();
                lines.extend(cells.map(|cell| plain_para(&cell)));
            }
            block => lines.push(to_plain(&Doc {
                blocks: vec![block],
            })),
        }
    }
    lines
        .iter()
        .flat_map(|l| l.lines())
        .map(str::trim_end)
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// A designed block from HTML that may also name the pictures of
/// `images` as `cid:katna-N` (a block's HTML edited by hand).
pub fn html_block_with(html: &str, images: &[Image], next_id: &mut u64) -> HtmlBlock {
    let kept = HtmlBlock {
        html: html.into(),
        text: "".into(),
        images: images.to_vec(),
    };
    html_block(&with_pictures(&kept, &data_uri), next_id)
}

struct Writer<'a> {
    out: &'a mut String,
    /// Open lists, outermost first, each with an open item.
    lists: Vec<(List, u8)>,
    quote: u8,
    signature: bool,
}

impl Writer<'_> {
    /// Opens and closes quotes, the signature and lists for a block of
    /// `style`.
    fn enter(&mut self, style: &ParaStyle) {
        let wrap_changes = style.quote != self.quote || style.signature != self.signature;
        if wrap_changes || style.list == List::None {
            self.close_lists(0);
        }
        if wrap_changes {
            if self.signature {
                self.out.push_str("</div>");
                self.signature = false;
            }
            while self.quote > style.quote {
                self.out.push_str("</blockquote>");
                self.quote -= 1;
            }
            while self.quote < style.quote {
                let _ = write!(self.out, "<blockquote style=\"{QUOTE_STYLE}\">");
                self.quote += 1;
            }
            if style.signature {
                self.out.push_str("<div class=\"katna_signature\">");
                self.signature = true;
            }
        }
    }

    fn close_lists(&mut self, keep: usize) {
        while self.lists.len() > keep {
            let (kind, _) = self.lists.pop().unwrap_or_default();
            self.out.push_str("</li>");
            self.out.push_str(if kind == List::Numbered {
                "</ol>"
            } else {
                "</ul>"
            });
        }
    }

    fn para(&mut self, para: &Para, number: usize) {
        let align = match para.style.align {
            Align::Left => "",
            Align::Center => "text-align:center",
            Align::Right => "text-align:right",
        };
        if para.style.list == List::None {
            let mut css = align.to_owned();
            if para.style.indent > 0 {
                if !css.is_empty() {
                    css.push(';');
                }
                let _ = write!(css, "margin-left:{}px", 40 * u32::from(para.style.indent));
            }
            if css.is_empty() {
                self.out.push_str("<div>");
            } else {
                let _ = write!(self.out, "<div style=\"{css}\">");
            }
            inline(self.out, para);
            self.out.push_str("</div>");
            return;
        }
        let (kind, level) = (para.style.list, para.style.indent);
        while self.lists.last().is_some_and(|&(_, l)| l > level) {
            self.close_lists(self.lists.len() - 1);
        }
        match self.lists.last() {
            Some(&(k, l)) if l == level && k == kind => self.out.push_str("</li>"),
            Some(&(_, l)) if l == level => {
                self.close_lists(self.lists.len() - 1);
                self.open_list(kind, level, number);
            }
            _ => self.open_list(kind, level, number),
        }
        if align.is_empty() {
            self.out.push_str("<li>");
        } else {
            let _ = write!(self.out, "<li style=\"{align}\">");
        }
        inline(self.out, para);
    }

    fn open_list(&mut self, kind: List, level: u8, number: usize) {
        let tag = if kind == List::Numbered { "ol" } else { "ul" };
        let kind_css = match (kind, level % 3) {
            (List::Numbered, 1) => ";list-style-type:lower-alpha",
            (List::Numbered, 2) => ";list-style-type:lower-roman",
            (List::Bullet, 1) => ";list-style-type:circle",
            (List::Bullet, 2) => ";list-style-type:square",
            _ => "",
        };
        let start = if kind == List::Numbered && number > 1 {
            format!(" start=\"{number}\"")
        } else {
            String::new()
        };
        let _ = write!(
            self.out,
            "<{tag}{start} style=\"margin:0 0 0 {}px;padding-left:1.2em{kind_css}\">",
            if self.lists.is_empty() { 15 } else { 0 },
        );
        self.lists.push((kind, level));
    }

    fn table(&mut self, table: &Table) {
        self.out.push_str(
            "<table style=\"border-collapse:collapse;border:1px solid rgb(204,204,204)\"><tbody>",
        );
        for row in &table.rows {
            self.out.push_str("<tr>");
            for cell in row {
                let align = match cell.style.align {
                    Align::Left => "",
                    Align::Center => ";text-align:center",
                    Align::Right => ";text-align:right",
                };
                let fill = cell
                    .style
                    .fill
                    .map(|c| format!(";background-color:#{c:06x}"))
                    .unwrap_or_default();
                let _ = write!(self.out, "<td style=\"{CELL_STYLE}{align}{fill}\">");
                inline(self.out, cell);
                self.out.push_str("</td>");
            }
            self.out.push_str("</tr>");
        }
        self.out.push_str("</tbody></table>");
    }
}

/// The styled text of a paragraph; an empty one is a `<br>` so it keeps
/// its height.
fn inline(out: &mut String, para: &Para) {
    if para.is_empty() {
        out.push_str("<br>");
        return;
    }
    for (range, style) in para.spans() {
        let mut close = Vec::new();
        if let Some(link) = &style.link {
            let _ = write!(out, "<a href=\"{}\">", escape(link));
            close.push("</a>");
        }
        let mut css = Vec::new();
        if style.font != Font::Sans {
            css.push(format!("font-family:{}", style.font.css()));
        }
        if let Some(size) = style.size.css() {
            css.push(format!("font-size:{size}"));
        }
        if let Some(color) = style.color {
            css.push(format!("color:#{color:06x}"));
        }
        if let Some(color) = style.background {
            css.push(format!("background-color:#{color:06x}"));
        }
        if !css.is_empty() {
            let _ = write!(out, "<span style=\"{}\">", escape(&css.join(";")));
            close.push("</span>");
        }
        for (on, open, end) in [
            (style.bold, "<b>", "</b>"),
            (style.italic, "<i>", "</i>"),
            (style.underline, "<u>", "</u>"),
            (style.strike, "<s>", "</s>"),
        ] {
            if on {
                out.push_str(open);
                close.push(end);
            }
        }
        if style.link.is_some() {
            text_html(out, &para.text, range);
        } else {
            autolinked(out, &para.text, range);
        }
        for end in close.into_iter().rev() {
            out.push_str(end);
        }
    }
}

/// Escaped text of `full[range]`; newlines become `<br>`, and spaces
/// that HTML would drop (doubled, or at a line's ends) stay as `&nbsp;`.
fn text_html(out: &mut String, full: &str, range: Range<usize>) {
    for (ix, c) in full[range.clone()].char_indices() {
        let at = range.start + ix;
        match c {
            '\n' => out.push_str("<br>"),
            ' ' => {
                let before = full[..at].chars().next_back();
                let after = full[at + 1..].chars().next();
                let keep =
                    matches!(before, None | Some(' ' | '\n')) || matches!(after, None | Some('\n'));
                out.push_str(if keep { "&nbsp;" } else { " " });
            }
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            c => out.push(c),
        }
    }
}

/// Text with the web addresses in it made links, as webmail does.
fn autolinked(out: &mut String, full: &str, range: Range<usize>) {
    let mut from = range.start;
    while let Some((start, end)) = find_url(&full[from..range.end]) {
        let (start, end) = (from + start, from + end);
        text_html(out, full, from..start);
        let url = &full[start..end];
        let href = if url.starts_with("www.") {
            format!("http://{url}")
        } else {
            url.to_owned()
        };
        let _ = write!(out, "<a href=\"{}\">", escape(&href));
        text_html(out, full, start..end);
        out.push_str("</a>");
        from = end;
    }
    text_html(out, full, from..range.end);
}

/// The first web address in `text`, as a byte range.
pub fn find_url(text: &str) -> Option<(usize, usize)> {
    let mut search = 0;
    loop {
        let found = ["https://", "http://", "www."]
            .iter()
            .filter_map(|p| text[search..].find(p).map(|at| search + at))
            .min()?;
        let starts_word = text[..found]
            .chars()
            .next_back()
            .is_none_or(|c| c.is_whitespace() || "(<[\"'".contains(c));
        let end = text[found..]
            .find(|c: char| c.is_whitespace() || "<>\"".contains(c))
            .map_or(text.len(), |e| found + e);
        // Trailing punctuation ends the sentence, not the address.
        let trimmed =
            text[found..end].trim_end_matches(['.', ',', ';', ':', '!', '?', ')', ']', '\'']);
        let end = found + trimmed.len();
        let bare = ["https://", "http://", "www."]
            .iter()
            .any(|p| trimmed.len() <= p.len());
        if starts_word && !bare {
            return Some((found, end));
        }
        search = found + 1;
        if search >= text.len() {
            return None;
        }
    }
}

pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            c => out.push(c),
        }
    }
    out
}

// Plain text.

/// The document as plain text, as the text part of a message carries it:
/// list markers, `> ` for quotes, link addresses after their text, tables
/// as rows of cells.
pub fn to_plain(doc: &Doc) -> String {
    let numbers = list_numbers(doc);
    let mut lines: Vec<String> = Vec::new();
    for (ix, block) in doc.blocks.iter().enumerate() {
        match block {
            Block::Para(para) => {
                let quote = "> ".repeat(para.style.quote as usize);
                let marker = list_marker(&para.style, numbers[ix]);
                let lead = if para.style.list == List::None {
                    String::new()
                } else {
                    format!("{}{marker} ", "   ".repeat(para.style.indent as usize))
                };
                let text = plain_para(para);
                for (n, line) in text.split('\n').enumerate() {
                    let lead = if n == 0 {
                        lead.clone()
                    } else {
                        " ".repeat(lead.chars().count())
                    };
                    let line = format!("{quote}{lead}{line}");
                    // Quote markers on an empty line keep no space.
                    lines.push(
                        if line.trim().is_empty() || line.trim_end() == quote.trim_end() {
                            line.trim_end().to_owned()
                        } else {
                            line
                        },
                    );
                }
            }
            Block::Table(table) => {
                for row in &table.rows {
                    let cells: Vec<String> = row
                        .iter()
                        .map(|cell| plain_para(cell).replace('\n', " "))
                        .collect();
                    lines.push(cells.join(" | "));
                }
            }
            Block::Image(image) => lines.push(format!("[image: {}]", image.name)),
            Block::Html(block) => lines.extend(block.text.lines().map(str::to_owned)),
        }
    }
    let mut out = lines.join("\n");
    out.push('\n');
    out
}

/// A paragraph's text with each link's address after it.
fn plain_para(para: &Para) -> String {
    let mut out = String::new();
    let mut spans = para.spans().peekable();
    while let Some((range, style)) = spans.next() {
        out.push_str(&para.text[range.clone()]);
        let Some(link) = &style.link else { continue };
        let continues = spans
            .peek()
            .is_some_and(|(_, next)| next.link.as_ref() == Some(link));
        if continues {
            continue;
        }
        let (whole, _) = para.link_at(range.start).unwrap_or((range, link.clone()));
        let text = &para.text[whole];
        let bare = link.strip_prefix("mailto:").unwrap_or(link);
        if text.trim() != bare && text.trim() != &**link {
            let _ = write!(out, " <{bare}>");
        }
    }
    out
}

/// Plain text as a document: a paragraph per line, `>` quoting as quote
/// depth.
pub fn from_plain(text: &str) -> Doc {
    let text = text.replace("\r\n", "\n");
    let blocks = text
        .split('\n')
        .map(|line| {
            let mut quote = 0u8;
            let mut rest = line;
            while let Some(r) = rest.strip_prefix('>') {
                quote = quote.saturating_add(1);
                rest = r.strip_prefix(' ').unwrap_or(r);
            }
            Block::Para(Para::plain(rest).with_style(ParaStyle {
                quote,
                ..ParaStyle::default()
            }))
        })
        .collect();
    Doc { blocks }
}

// HTML back in.

/// Reads HTML as a document: the tags this module writes (and the usual
/// inline ones), so a stored signature comes back as it was. Images must
/// be `data:` URIs; `next_id` numbers them.
pub fn from_html(html: &str, next_id: &mut u64) -> Doc {
    read_html(html, next_id, false)
}

/// Reads HTML pasted or dropped from another app: a word processor, a
/// spreadsheet or a web page. Beyond [`from_html`], pictures may come
/// from local files (Word puts them there), and the page's own near-black
/// text and white background are left out so the text follows the theme.
pub fn from_pasted_html(html: &str, next_id: &mut u64) -> Doc {
    read_html(html, next_id, true)
}

fn read_html(html: &str, next_id: &mut u64, pasted: bool) -> Doc {
    let mut reader = Reader {
        blocks: Vec::new(),
        para: Para::default(),
        open: false,
        styles: vec![CharStyle::default()],
        para_style: vec![ParaStyle::default()],
        lists: Vec::new(),
        elements: Vec::new(),
        table: None,
        inner_tables: 0,
        sheet: HashMap::new(),
        marker: None,
        next_id,
        pasted,
    };
    let mut rest = html;
    while !rest.is_empty() {
        if let Some(after) = rest.strip_prefix(HTML_START) {
            let end = after.find(HTML_END).unwrap_or(after.len());
            reader.html(&after[..end]);
            rest = after.get(end + HTML_END.len()..).unwrap_or("");
            continue;
        }
        if let Some(after) = rest.strip_prefix("<!--") {
            rest = after.find("-->").map_or("", |e| &after[e + 3..]);
            continue;
        }
        if rest.starts_with('<')
            && let Some(end) = rest.find('>')
        {
            let raw = &rest[1..end];
            rest = &rest[end + 1..];
            if let Some(name) = reader.tag(raw) {
                // What a style sheet or script holds is not text.
                let close = find_ascii_ci(rest, &format!("</{name}"));
                let (content, after) = close.map_or((rest, ""), |at| rest.split_at(at));
                if name == "style" {
                    reader.style_sheet(content);
                }
                rest = after;
            }
            continue;
        }
        // Past the first character, which may be a lone '<' or take more
        // than one byte.
        let first = rest.chars().next().map_or(1, char::len_utf8);
        let end = rest[first..].find('<').map_or(rest.len(), |e| e + first);
        reader.text(&decode_entities(&rest[..end]));
        rest = &rest[end..];
    }
    reader.close_all();
    reader.flush(false);
    if reader.blocks.is_empty() {
        reader.blocks.push(Block::Para(Para::default()));
    }
    Doc {
        blocks: reader.blocks,
    }
}

/// The byte offset of `needle` (ASCII) in `text`, ignoring ASCII case.
fn find_ascii_ci(text: &str, needle: &str) -> Option<usize> {
    let (hay, needle) = (text.as_bytes(), needle.as_bytes());
    (0..hay.len().saturating_sub(needle.len() - 1))
        .find(|&at| hay[at..at + needle.len()].eq_ignore_ascii_case(needle))
}

/// Elements without content or an end tag.
const VOID: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "wbr",
];
/// Elements whose content is not HTML.
const RAW: &[&str] = &["style", "script", "title", "xml", "template", "textarea"];
const BLOCKS: &[&str] = &[
    "div", "p", "h1", "h2", "h3", "h4", "h5", "h6", "pre", "address",
];
/// Other elements whose background is a box's, not the text's.
const BOXES: &[&str] = &[
    "html",
    "body",
    "table",
    "tbody",
    "thead",
    "tfoot",
    "tr",
    "td",
    "th",
    "ul",
    "ol",
    "li",
    "blockquote",
];

/// An element being read, and what it changed that its end undoes.
struct Element {
    name: String,
    /// It added a paragraph style.
    block: bool,
}

/// A table being read. Cells are placed on a grid so that merged cells
/// (`colspan`, `rowspan`) keep the rest in their columns; the cells they
/// cover stay empty.
struct TableReader {
    rows: Vec<Vec<Option<Para>>>,
    /// Rows started (`tr`).
    started: usize,
    /// The cell being read: its row, spans and style.
    cell: Option<(usize, usize, usize, ParaStyle)>,
}

impl TableReader {
    /// Places a cell in the first free column of `row`.
    fn place(&mut self, row: usize, cols: usize, rows: usize, cell: Para) {
        if self.rows.len() <= row {
            self.rows.resize_with(row + 1, Vec::new);
        }
        let col = self.rows[row]
            .iter()
            .position(Option::is_none)
            .unwrap_or(self.rows[row].len());
        // Huge spans are mistakes; they would make a huge table.
        let (cols, rows) = (cols.clamp(1, 64), rows.clamp(1, 256));
        let fill = cell.style;
        for r in row..row + rows {
            if self.rows.len() <= r {
                self.rows.resize_with(r + 1, Vec::new);
            }
            let cells = &mut self.rows[r];
            if cells.len() < col + cols {
                cells.resize(col + cols, None);
            }
            for slot in &mut cells[col..col + cols] {
                if slot.is_none() {
                    *slot = Some(Para::default().with_style(ParaStyle {
                        fill: fill.fill,
                        ..ParaStyle::default()
                    }));
                }
            }
        }
        self.rows[row][col] = Some(cell);
    }

    fn finish(self) -> Option<Table> {
        let rows: Vec<Vec<Option<Para>>> =
            self.rows.into_iter().filter(|r| !r.is_empty()).collect();
        let cols = rows.iter().map(Vec::len).max().unwrap_or(0);
        if cols == 0 {
            return None;
        }
        Some(Table {
            rows: rows
                .into_iter()
                .map(|row| {
                    let mut row: Vec<Para> =
                        row.into_iter().map(Option::unwrap_or_default).collect();
                    row.resize(cols, Para::default());
                    row
                })
                .collect(),
        })
    }
}

struct Reader<'a> {
    blocks: Vec<Block>,
    para: Para,
    /// A block element started this paragraph: it is kept even if empty.
    open: bool,
    /// The character style of each open element, over the default.
    styles: Vec<CharStyle>,
    para_style: Vec<ParaStyle>,
    lists: Vec<List>,
    elements: Vec<Element>,
    table: Option<TableReader>,
    /// Tables inside the table being read: their cells are read as text.
    inner_tables: usize,
    /// Style sheet rules by selector: `tag`, `.class` or `tag.class`.
    sheet: HashMap<String, String>,
    /// Word's list marker being read (the bullet or number it writes as
    /// text).
    marker: Option<String>,
    next_id: &'a mut u64,
    pasted: bool,
}

impl Reader<'_> {
    fn style(&self) -> CharStyle {
        self.styles.last().cloned().unwrap_or_default()
    }

    fn block_style(&self) -> ParaStyle {
        self.para_style.last().copied().unwrap_or_default()
    }

    fn text(&mut self, text: &str) {
        if let Some(marker) = &mut self.marker {
            marker.push_str(text);
            return;
        }
        // Layout whitespace between tags is not text.
        let collapsed = collapse(text);
        let in_cell = self.table.as_ref().is_some_and(|t| t.cell.is_some());
        let between_cells = self.table.is_some() && !in_cell;
        if between_cells {
            return;
        }
        if collapsed.trim_matches(' ').is_empty() && (self.para.is_empty() || in_cell) {
            if !self.para.is_empty() && collapsed.contains(' ') && !self.para.text.ends_with(' ') {
                let style = self.style();
                self.para.insert(self.para.len(), " ", &style);
            }
            return;
        }
        let style = self.style();
        let text = if self.para.is_empty() || self.para.text.ends_with(['\n', ' ']) {
            collapsed.trim_start_matches(' ').to_owned()
        } else {
            collapsed
        };
        let len = self.para.len();
        self.para.insert(len, &text, &style);
    }

    /// Ends the paragraph being read. `keep` keeps it even when empty.
    fn flush(&mut self, keep: bool) {
        if self.table.is_some() {
            // In a cell, paragraphs are lines.
            self.cell_line();
            return;
        }
        let mut para = finish(std::mem::take(&mut self.para), &[' ']);
        if self.pasted && para.text.trim().is_empty() {
            // Word's empty lines hold a no-break space.
            para.remove(0..para.len());
        }
        let wanted = keep || self.open || !para.is_empty();
        self.open = false;
        if !wanted {
            return;
        }
        if para.style == ParaStyle::default() {
            para.style = self.block_style();
        }
        para.style.fill = None;
        self.blocks.push(Block::Para(para));
    }

    /// A new line in the cell being read, unless it is at the start of one.
    fn cell_line(&mut self) {
        if !self.para.is_empty() && !self.para.text.ends_with('\n') {
            let style = self.style();
            let len = self.para.len();
            let trimmed = self.para.text.trim_end_matches(' ').len();
            self.para.remove(trimmed..len);
            self.para.insert(trimmed, "\n", &style);
        }
    }

    /// A block opening inside an empty one replaces it.
    fn drop_empty(&mut self) {
        if self.para.is_empty() {
            self.open = false;
        }
    }

    fn start_para(&mut self, style: ParaStyle) {
        self.drop_empty();
        self.flush(false);
        self.para.style = style;
        self.open = true;
    }

    /// The CSS for an element: its rules from the style sheet, then its
    /// own `style`.
    fn css_of(&self, name: &str, attrs: &str) -> String {
        let mut css = String::new();
        if !self.sheet.is_empty() {
            let mut add = |selector: &str| {
                if let Some(rule) = self.sheet.get(selector) {
                    css.push_str(rule);
                    css.push(';');
                }
            };
            add(name);
            for class in attr(attrs, "class").unwrap_or_default().split_whitespace() {
                add(&format!(".{class}"));
                add(&format!("{name}.{class}"));
            }
        }
        if let Some(own) = attr(attrs, "style") {
            css.push_str(&own);
        }
        css
    }

    /// Reads a style sheet's simple rules: by tag, class, or tag and class.
    fn style_sheet(&mut self, sheet: &str) {
        let mut text = sheet.replace("<!--", " ").replace("-->", " ");
        while let Some(start) = text.find("/*") {
            let end = text[start..]
                .find("*/")
                .map_or(text.len(), |e| start + e + 2);
            text.replace_range(start..end, " ");
        }
        for rule in text.split('}') {
            let Some((selectors, body)) = rule.split_once('{') else {
                continue;
            };
            // Past an at-rule's own opening brace (`@media x {`).
            let selectors = selectors.rsplit('{').next().unwrap_or_default();
            for selector in selectors.split(',') {
                let selector = selector.trim();
                let (tag, class) = selector.split_once('.').unwrap_or((selector, ""));
                let simple = |s: &str| {
                    s.chars()
                        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
                };
                if selector.is_empty()
                    || selector.starts_with('@')
                    || !simple(tag)
                    || !simple(class)
                {
                    continue;
                }
                let key = if class.is_empty() {
                    tag.to_ascii_lowercase()
                } else {
                    format!("{}.{class}", tag.to_ascii_lowercase())
                };
                let entry = self.sheet.entry(key).or_default();
                entry.push_str(body.trim());
                entry.push(';');
            }
        }
    }

    /// Reads a tag. For an element whose content is not HTML, gives its
    /// name, so the caller skips to its end.
    fn tag(&mut self, raw: &str) -> Option<String> {
        let raw = raw.trim().trim_end_matches('/').trim();
        // Word's list markers: `<![if !supportLists]>1.<![endif]>`.
        if let Some(cond) = raw.strip_prefix("![if") {
            if cond.contains("supportLists") {
                self.marker = Some(String::new());
            }
            return None;
        }
        if raw.starts_with("![endif") {
            if let Some(marker) = self.marker.take()
                && self.para.style.list != List::None
            {
                let marker = marker.trim_matches(|c: char| c.is_whitespace() || c == '\u{a0}');
                if marker.starts_with(|c: char| c.is_ascii_alphanumeric()) {
                    self.para.style.list = List::Numbered;
                }
            }
            return None;
        }
        let (closing, raw) = match raw.strip_prefix('/') {
            Some(r) => (true, r),
            None => (false, raw),
        };
        let name_end = raw.find(|c: char| c.is_whitespace()).unwrap_or(raw.len());
        let name = raw[..name_end].to_ascii_lowercase();
        if name.is_empty() || name.starts_with('!') || name.starts_with('?') {
            return None;
        }
        let attrs = &raw[name_end..];
        if closing {
            if name == "br" {
                self.line_break();
            } else {
                self.close(&name);
            }
            return None;
        }
        if RAW.contains(&name.as_str()) {
            return Some(name);
        }
        match name.as_str() {
            "br" => {
                self.line_break();
                return None;
            }
            "img" => {
                self.image(attrs);
                return None;
            }
            "hr" => {
                self.start_para(self.block_style());
                self.flush(true);
                return None;
            }
            n if VOID.contains(&n) => return None,
            _ => {}
        }
        // Elements that end the one of their kind still open.
        match name.as_str() {
            "p" => self.close_open("p", &["div", "td", "th", "li", "blockquote", "table"]),
            "li" => self.close_open("li", &["ul", "ol"]),
            "td" | "th" => self.close_open("td", &["tr", "table"]),
            "tr" => self.close_open("tr", &["table"]),
            _ => {}
        }
        if matches!(name.as_str(), "td" | "th") {
            self.close_open("th", &["tr", "table"]);
        }
        let css = self.css_of(&name, attrs);
        let mut element = Element {
            name: name.clone(),
            block: false,
        };
        // The character style inside the element.
        let mut style = self.style();
        match name.as_str() {
            "b" | "strong" | "th" => style.bold = true,
            "i" | "em" | "cite" | "var" => style.italic = true,
            "u" | "ins" => style.underline = true,
            "s" | "strike" | "del" => style.strike = true,
            "code" | "tt" | "kbd" | "samp" | "pre" => style.font = Font::Fixed,
            "a" => style.link = attr(attrs, "href").map(|h| Arc::<str>::from(decode_entities(&h))),
            "font" => {
                if let Some(c) = attr(attrs, "color").as_deref().and_then(parse_color) {
                    style.color = Some(c);
                }
                if let Some(f) = attr(attrs, "face").as_deref() {
                    style.font = font_named(f);
                }
                match attr(attrs, "size").as_deref() {
                    Some("1" | "2") => style.size = Size::Small,
                    Some("4" | "5") => style.size = Size::Large,
                    Some("6" | "7") => style.size = Size::Huge,
                    _ => {}
                }
            }
            "h1" => {
                style.bold = true;
                style.size = Size::Huge;
            }
            "h2" | "h3" | "h4" | "h5" | "h6" => {
                style.bold = true;
                style.size = Size::Large;
            }
            _ => {}
        }
        apply_css(&mut style, &css);
        if BOXES.contains(&name.as_str()) || BLOCKS.contains(&name.as_str()) {
            // A box's background fills the box (a table cell), not the
            // text in it.
            style.background = self.style().background;
        }
        if self.pasted {
            // The page's own text and background colors, not the writer's.
            if style.color.is_some_and(is_default_text) {
                style.color = None;
            }
            if style.background.is_some_and(is_default_page) {
                style.background = None;
            }
        }
        // Paragraphs, lists, quotes and tables.
        match name.as_str() {
            n if BLOCKS.contains(&n) => {
                let signature = attr(attrs, "class").is_some_and(|c| c.contains("signature"));
                let mut block = self.block_style();
                block.signature |= signature;
                apply_block_css(&mut block, &css);
                if let Some(level) = word_list_level(&css) {
                    block.list = List::Bullet;
                    block.indent = level;
                } else if self.open && self.para.is_empty() && self.para.style.list != List::None {
                    // A paragraph opening a list item (`<li><p>`, as
                    // LibreOffice writes) is the item.
                    block.list = self.para.style.list;
                    block.indent = self.para.style.indent;
                }
                self.para_style.push(ParaStyle {
                    align: Align::Left,
                    list: List::None,
                    indent: self.block_style().indent,
                    ..block
                });
                element.block = true;
                if self.table.is_none() {
                    self.start_para(block);
                } else {
                    self.cell_line();
                }
            }
            "blockquote" => {
                self.flush(false);
                let mut block = self.block_style();
                block.quote = block.quote.saturating_add(1);
                self.para_style.push(block);
                element.block = true;
            }
            "ul" | "ol" => {
                self.flush(false);
                self.lists.push(if name == "ol" {
                    List::Numbered
                } else {
                    List::Bullet
                });
            }
            "li" => {
                let mut block = self.block_style();
                block.list = self.lists.last().copied().unwrap_or(List::Bullet);
                block.indent = self.lists.len().saturating_sub(1).min(8) as u8;
                apply_block_css(&mut block, &css);
                if self.table.is_none() {
                    self.start_para(block);
                } else {
                    self.cell_line();
                }
            }
            "table" => {
                if self.table.is_some() {
                    self.inner_tables += 1;
                } else {
                    self.drop_empty();
                    self.flush(false);
                    self.table = Some(TableReader {
                        rows: Vec::new(),
                        started: 0,
                        cell: None,
                    });
                    let mut block = ParaStyle::default();
                    apply_block_css(&mut block, &css);
                    self.para_style.push(ParaStyle {
                        fill: block.fill,
                        ..ParaStyle::default()
                    });
                    element.block = true;
                }
            }
            "tr" if self.inner_tables == 0 => {
                if let Some(table) = &mut self.table {
                    table.started += 1;
                }
                let mut block = self.block_style();
                apply_block_css(&mut block, &css);
                if let Some(c) = attr(attrs, "bgcolor").as_deref().and_then(parse_color) {
                    block.fill = Some(c);
                }
                self.para_style.push(block);
                element.block = true;
            }
            "td" | "th" if self.inner_tables == 0 && self.table.is_some() => {
                let mut block = ParaStyle {
                    fill: self.block_style().fill,
                    ..ParaStyle::default()
                };
                if let Some(a) = attr(attrs, "align") {
                    apply_block_css(&mut block, &format!("text-align:{a}"));
                }
                if let Some(c) = attr(attrs, "bgcolor").as_deref().and_then(parse_color) {
                    block.fill = Some(c);
                }
                apply_block_css(&mut block, &css);
                if self.pasted && block.fill.is_some_and(is_default_page) {
                    block.fill = None;
                }
                let span = |n: &str| {
                    attr(attrs, n)
                        .and_then(|v| v.trim().parse().ok())
                        .unwrap_or(1)
                };
                let (cols, rows) = (span("colspan"), span("rowspan"));
                if let Some(table) = &mut self.table {
                    // A cell outside any row starts one.
                    table.started = table.started.max(1);
                    table.cell = Some((table.started - 1, cols, rows, block));
                }
                self.para = Para::default();
            }
            _ => {}
        }
        self.styles.push(style);
        self.elements.push(element);
        None
    }

    /// Closes the innermost open `name`, unless one of `stop` is open
    /// inside it.
    fn close_open(&mut self, name: &str, stop: &[&str]) {
        for element in self.elements.iter().rev() {
            if element.name == name {
                self.close(name);
                return;
            }
            if stop.contains(&element.name.as_str()) {
                return;
            }
        }
    }

    /// Ends the innermost open `name` and all open inside it.
    fn close(&mut self, name: &str) {
        let Some(at) = self.elements.iter().rposition(|e| e.name == name) else {
            return;
        };
        while self.elements.len() > at {
            let Some(element) = self.elements.pop() else {
                break;
            };
            self.end(&element);
            if self.styles.len() > 1 {
                self.styles.pop();
            }
            if element.block && self.para_style.len() > 1 {
                self.para_style.pop();
            }
        }
    }

    fn close_all(&mut self) {
        if let Some(first) = self.elements.first().map(|e| e.name.clone()) {
            self.close(&first);
        }
    }

    /// What ending an element does.
    fn end(&mut self, element: &Element) {
        match element.name.as_str() {
            n if BLOCKS.contains(&n) => {
                if self.table.is_none() {
                    self.flush(false);
                }
            }
            "blockquote" | "li" => self.flush(false),
            "ul" | "ol" => {
                self.flush(false);
                self.lists.pop();
            }
            "td" | "th" if self.inner_tables > 0 => {
                if !self.para.is_empty() && !self.para.text.ends_with([' ', '\n']) {
                    let style = self.style();
                    let len = self.para.len();
                    self.para.insert(len, " ", &style);
                }
            }
            "tr" if self.inner_tables > 0 => self.cell_line(),
            "td" | "th" => {
                let mut cell = finish(std::mem::take(&mut self.para), &[' ', '\n']);
                if let Some(table) = &mut self.table
                    && let Some((row, cols, rows, style)) = table.cell.take()
                {
                    cell.style = style;
                    table.place(row, cols, rows, cell);
                }
            }
            "table" if self.inner_tables > 0 => {
                self.inner_tables -= 1;
                self.cell_line();
            }
            "table" => {
                if let Some(table) = self.table.take()
                    && let Some(table) = table.finish()
                {
                    self.blocks.push(Block::Table(table));
                }
                self.para = Para::default();
                self.open = false;
            }
            _ => {}
        }
    }

    fn image(&mut self, attrs: &str) {
        let Some(src) = attr(attrs, "src") else {
            return;
        };
        let name = attr(attrs, "alt").unwrap_or_default();
        let width = attr(attrs, "width").and_then(|w| w.trim_end_matches("px").parse().ok());
        let image = if self.pasted && src.starts_with("file://") {
            image_from_file(&src, name, self.next_id)
        } else {
            image_from_data_uri(&src, name, width, self.next_id)
        };
        let Some(image) = image else {
            return;
        };
        if self.table.is_some() {
            // Tables hold text only.
            return;
        }
        self.drop_empty();
        self.flush(false);
        self.blocks.push(Block::Image(image));
    }

    /// A designed block, kept whole.
    fn html(&mut self, html: &str) {
        let block = html_block(html, self.next_id);
        if self.table.is_some() {
            return;
        }
        self.drop_empty();
        self.flush(false);
        self.blocks.push(Block::Html(block));
    }

    fn line_break(&mut self) {
        if self.table.is_some() {
            let style = self.style();
            let len = self.para.len();
            self.para.insert(len, "\n", &style);
            return;
        }
        let style = self.para.style;
        // `<div><br></div>` is one empty line, not two.
        self.flush(true);
        self.para.style = style;
    }
}

/// The list level (from 0) of a paragraph Word wrote as a list item
/// (`mso-list:l0 level2 lfo1`).
fn word_list_level(css: &str) -> Option<u8> {
    let (_, value) = css_pairs(css).find(|(k, _)| k == "mso-list")?;
    let level = value
        .split_whitespace()
        .find_map(|w| w.strip_prefix("level")?.parse::<u8>().ok())?;
    Some(level.saturating_sub(1).min(MAX_INDENT))
}

/// Near-black grey: a page's ordinary text color.
fn is_default_text(color: u32) -> bool {
    let (r, g, b) = ((color >> 16) & 0xff, (color >> 8) & 0xff, color & 0xff);
    r.max(g).max(b) < 0x50 && r.max(g).max(b) - r.min(g).min(b) < 0x18
}

/// White or near it: a page's ordinary background.
fn is_default_page(color: u32) -> bool {
    let (r, g, b) = ((color >> 16) & 0xff, (color >> 8) & 0xff, color & 0xff);
    r.min(g).min(b) >= 0xf8
}

/// A paragraph as read: collapsed spaces trimmed from its end, and kept
/// (non-breaking) spaces made plain.
fn finish(mut para: Para, trim: &[char]) -> Para {
    let trimmed = para.text.trim_end_matches(trim).len();
    para.remove(trimmed..para.len());
    while let Some(at) = para.text.rfind('\u{a0}') {
        let style = para.style_at(at + 1);
        para.remove(at..at + 2);
        para.insert(at, " ", &style);
    }
    para
}

fn image_from_data_uri(
    src: &str,
    name: String,
    width: Option<u32>,
    next_id: &mut u64,
) -> Option<Image> {
    let rest = src.strip_prefix("data:")?;
    let (meta, data) = rest.split_once(',')?;
    let mime = meta.strip_suffix(";base64")?.to_owned();
    let data = base64_decode(data)?;
    let (w, h) = image_size(&data).unwrap_or((0, 0));
    let size = match width {
        Some(width) if width <= 128 && w > 128 => ImageSize::Small,
        Some(width) if w > 0 && width == w && w as f32 > BEST_FIT_WIDTH => ImageSize::Original,
        _ => ImageSize::BestFit,
    };
    *next_id += 1;
    Some(Image {
        id: *next_id,
        name: if name.is_empty() {
            "image".into()
        } else {
            name
        },
        mime,
        data: Arc::new(data),
        width: w,
        height: h,
        size,
    })
}

/// A picture from a local file (Word puts the pictures it copies in
/// temporary files).
fn image_from_file(src: &str, name: String, next_id: &mut u64) -> Option<Image> {
    let path = percent_decode(src.strip_prefix("file://")?.trim_start_matches("localhost"));
    let meta = std::fs::metadata(&path).ok()?;
    // Larger than any message can carry.
    if !meta.is_file() || meta.len() > 25 * 1024 * 1024 {
        return None;
    }
    let data = std::fs::read(&path).ok()?;
    let (width, height) = image_size(&data)?;
    let file = path.rsplit('/').next().unwrap_or("image").to_owned();
    let mime = super::editor::image_mime(&file)?.to_owned();
    *next_id += 1;
    Some(Image {
        id: *next_id,
        name: if name.is_empty() { file } else { name },
        mime,
        data: Arc::new(data),
        width,
        height,
        size: ImageSize::BestFit,
    })
}

/// `%XX` escapes decoded.
fn percent_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && let Some(hex) = text.get(i + 1..i + 3)
            && let Ok(byte) = u8::from_str_radix(hex, 16)
        {
            out.push(byte);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// The value of attribute `name` in the attributes of a tag.
fn attr(attrs: &str, name: &str) -> Option<String> {
    let lower = attrs.to_ascii_lowercase();
    let mut from = 0;
    while let Some(found) = lower[from..].find(name) {
        let at = from + found;
        from = at + name.len();
        let before_ok = at == 0 || lower.as_bytes()[at - 1].is_ascii_whitespace();
        let rest = attrs[from..].trim_start();
        if !before_ok || !rest.starts_with('=') {
            continue;
        }
        let rest = rest[1..].trim_start();
        let value = match rest.chars().next()? {
            q @ ('"' | '\'') => rest[1..].split(q).next()?.to_owned(),
            _ => rest.split(|c: char| c.is_whitespace()).next()?.to_owned(),
        };
        return Some(decode_entities(&value));
    }
    None
}

fn css_pairs(css: &str) -> impl Iterator<Item = (String, String)> + '_ {
    css.split(';').filter_map(|decl| {
        let (k, v) = decl.split_once(':')?;
        Some((k.trim().to_ascii_lowercase(), v.trim().to_owned()))
    })
}

fn apply_css(style: &mut CharStyle, css: &str) {
    for (key, value) in css_pairs(css) {
        match key.as_str() {
            "font-weight" => {
                style.bold = matches!(value.as_str(), "bold" | "bolder")
                    || value.parse::<u32>().is_ok_and(|w| w >= 600)
            }
            "font-style" => style.italic = value == "italic" || value == "oblique",
            "text-decoration" | "text-decoration-line" => {
                style.underline |= value.contains("underline");
                style.strike |= value.contains("line-through");
            }
            "font-family" => style.font = font_named(&value),
            "font-size" => {
                style.size = match value.as_str() {
                    "x-small" | "xx-small" | "small" | "smaller" => Size::Small,
                    "large" | "larger" | "x-large" => Size::Large,
                    "xx-large" | "xxx-large" => Size::Huge,
                    v => match css_px(v) {
                        Some(px) if px < 12.0 => Size::Small,
                        Some(px) if px >= 24.0 => Size::Huge,
                        Some(px) if px >= 17.0 => Size::Large,
                        _ => Size::Normal,
                    },
                }
            }
            "color" => style.color = parse_color(&value),
            "background-color" | "background" | "mso-highlight" => {
                style.background = background_color(&value)
            }
            _ => {}
        }
    }
}

/// The color a `background` value paints, if one.
fn background_color(value: &str) -> Option<u32> {
    parse_color(value).or_else(|| value.split_whitespace().find_map(parse_color))
}

/// A CSS length as pixels: `px`, `pt`, `in`, `cm`, `mm`, `em`, `rem` or
/// `%` (of 16 px text).
fn css_px(value: &str) -> Option<f32> {
    let value = value.trim().to_ascii_lowercase();
    let split = value
        .find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-'))
        .unwrap_or(value.len());
    let (number, unit) = value.split_at(split);
    let number: f32 = number.parse().ok()?;
    let scale = match unit.trim() {
        "" | "px" => 1.0,
        "pt" => 4.0 / 3.0,
        "in" => 96.0,
        "cm" => 96.0 / 2.54,
        "mm" => 96.0 / 25.4,
        "em" | "rem" => 16.0,
        "%" => 0.16,
        _ => return None,
    };
    Some(number * scale)
}

fn apply_block_css(style: &mut ParaStyle, css: &str) {
    for (key, value) in css_pairs(css) {
        match key.as_str() {
            "text-align" => {
                style.align = match value.to_ascii_lowercase().as_str() {
                    "center" | "middle" => Align::Center,
                    "right" | "end" => Align::Right,
                    _ => Align::Left,
                }
            }
            "margin-left" => {
                if let Some(px) = css_px(&value)
                    && px >= 40.0
                    && style.list == List::None
                {
                    style.indent = ((px / 40.0).round() as u8).min(MAX_INDENT);
                }
            }
            "background-color" | "background" => style.fill = background_color(&value),
            _ => {}
        }
    }
}

/// The menu font whose CSS family list starts like `family`.
fn font_named(family: &str) -> Font {
    let first = family
        .split(',')
        .next()
        .unwrap_or_default()
        .trim()
        .trim_matches(['"', '\''])
        .to_ascii_lowercase();
    Font::ALL
        .into_iter()
        .find(|f| {
            let css = f.css().split(',').next().unwrap_or_default();
            css.trim_matches('"') == first
        })
        .or(match first.as_str() {
            "serif" | "times" => Some(Font::Serif),
            "monospace" | "courier" | "courier new" => Some(Font::Fixed),
            // Families named after their kind ("Liberation Serif", "DejaVu
            // Sans Mono", "Times New Roman").
            f if f.contains("mono") || f.contains("courier") || f.contains("consol") => {
                Some(Font::Fixed)
            }
            f if (f.contains("serif") && !f.contains("sans")) || f.starts_with("times") => {
                Some(Font::Serif)
            }
            _ => None,
        })
        .unwrap_or(Font::Sans)
}

/// `#rgb`, `#rrggbb`, `rgb(r, g, b)` or a basic color name as
/// `0xRRGGBB`.
pub fn parse_color(value: &str) -> Option<u32> {
    let value = value
        .trim()
        .trim_end_matches("!important")
        .trim()
        .to_ascii_lowercase();
    let value = value.as_str();
    let named = match value {
        "black" => Some(0x000000),
        "white" => Some(0xffffff),
        "red" => Some(0xff0000),
        "green" => Some(0x008000),
        "lime" => Some(0x00ff00),
        "blue" => Some(0x0000ff),
        "yellow" => Some(0xffff00),
        "orange" => Some(0xffa500),
        "purple" => Some(0x800080),
        "gray" | "grey" => Some(0x808080),
        "silver" => Some(0xc0c0c0),
        "maroon" => Some(0x800000),
        "navy" => Some(0x000080),
        "teal" => Some(0x008080),
        "olive" => Some(0x808000),
        "aqua" | "cyan" => Some(0x00ffff),
        "fuchsia" | "magenta" => Some(0xff00ff),
        _ => None,
    };
    if named.is_some() {
        return named;
    }
    if let Some(hex) = value.strip_prefix('#') {
        let hex = if hex.len() == 8 { &hex[..6] } else { hex };
        return match hex.len() {
            3 => {
                let v = u32::from_str_radix(hex, 16).ok()?;
                let (r, g, b) = ((v >> 8) & 0xf, (v >> 4) & 0xf, v & 0xf);
                Some(((r * 17) << 16) | ((g * 17) << 8) | (b * 17))
            }
            6 => u32::from_str_radix(hex, 16).ok(),
            _ => None,
        };
    }
    let inner = value
        .strip_prefix("rgb(")
        .or_else(|| value.strip_prefix("rgba("))?
        .strip_suffix(')')?;
    let mut parts = inner
        .split([',', ' ', '/'])
        .filter(|p| !p.trim().is_empty())
        .map(|p| p.trim().parse::<f32>().ok().map(|v| v.round() as u32));
    let (r, g, b) = (parts.next()??, parts.next()??, parts.next()??);
    Some((r.min(255)) << 16 | (g.min(255)) << 8 | b.min(255))
}

/// Whitespace runs as one space, as HTML reads them.
fn collapse(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut space = false;
    for c in text.chars() {
        if c.is_whitespace() && c != '\u{a0}' {
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

fn decode_entities(text: &str) -> String {
    if !text.contains('&') {
        return text.to_owned();
    }
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find('&') {
        out.push_str(&rest[..at]);
        rest = &rest[at..];
        let Some(end) = rest[..rest.len().min(12)].find(';') else {
            out.push('&');
            rest = &rest[1..];
            continue;
        };
        let entity = &rest[1..end];
        let decoded = match entity {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            "nbsp" => Some('\u{a0}'),
            _ => entity
                .strip_prefix("#x")
                .or_else(|| entity.strip_prefix("#X"))
                .and_then(|h| u32::from_str_radix(h, 16).ok())
                .or_else(|| entity.strip_prefix('#').and_then(|d| d.parse().ok()))
                .and_then(char::from_u32),
        };
        match decoded {
            Some(c) => {
                out.push(c);
                rest = &rest[end + 1..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

// Base64, for images in stored HTML and in messages.

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Standard base64 with padding, as one line.
pub fn base64_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b = [
            chunk[0],
            chunk.get(1).copied().unwrap_or(0),
            chunk.get(2).copied().unwrap_or(0),
        ];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(ALPHABET[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

/// Decodes base64, skipping whitespace; `None` on other bad input.
pub fn base64_decode(text: &str) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(text.len() * 3 / 4);
    let (mut acc, mut bits) = (0u32, 0u32);
    for c in text.bytes() {
        let v = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' | b'-' => 62,
            b'/' | b'_' => 63,
            b'=' => break,
            c if c.is_ascii_whitespace() => continue,
            _ => return None,
        };
        acc = (acc << 6) | u32::from(v);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }
    Some(out)
}

/// An image as a `data:` URI, for storing it inside HTML.
pub fn data_uri(image: &Image) -> String {
    format!("data:{};base64,{}", image.mime, base64_encode(&image.data))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rich::doc::{Path, Pos};

    fn doc(paras: Vec<Para>) -> Doc {
        Doc {
            blocks: paras.into_iter().map(Block::Para).collect(),
        }
    }

    #[test]
    fn reads_text_starting_with_a_wide_character() {
        let mut next = 0;
        for html in ["é<b>x</b>", "<p>a</p>€ 5", "日本<br>語", "<i>x</i>😀"] {
            let doc = from_html(html, &mut next);
            assert!(!doc.blocks.is_empty(), "{html}");
        }
        let doc = from_html("<p>€ 5 &amp; é</p>", &mut next);
        assert_eq!(to_plain(&doc).trim_end(), "€ 5 & é");
    }

    #[test]
    fn writes_styles_and_escapes() {
        let mut p = Para::plain("a <b> & c");
        p.restyle(0..1, &|s| s.bold = true);
        p.restyle(2..5, &|s| {
            s.color = Some(0xff0000);
            s.italic = true;
        });
        let html = to_html(&doc(vec![p]), &|_| String::new());
        assert_eq!(
            html,
            "<div dir=\"ltr\"><div><b>a</b> <span style=\"color:#ff0000\"><i>&lt;b&gt;</i></span> &amp; c</div></div>"
        );
    }

    #[test]
    fn writes_nested_lists_and_quotes() {
        let item = |t: &str, list, indent| {
            Para::plain(t).with_style(ParaStyle {
                list,
                indent,
                ..ParaStyle::default()
            })
        };
        let mut quoted = Para::plain("q");
        quoted.style.quote = 1;
        let d = doc(vec![
            item("a", List::Bullet, 0),
            item("b", List::Numbered, 1),
            item("c", List::Bullet, 0),
            Para::default(),
            quoted,
        ]);
        let html = to_html(&d, &|_| String::new());
        assert!(html.contains("<ul style=\"margin:0 0 0 15px;padding-left:1.2em\"><li>a<ol style=\"margin:0 0 0 0px;padding-left:1.2em;list-style-type:lower-alpha\"><li>b</li></ol></li><li>c</li></ul><div><br></div><blockquote"), "{html}");
        let plain = to_plain(&d);
        assert_eq!(plain, "\u{2022} a\n   a. b\n\u{2022} c\n\n> q\n");
    }

    #[test]
    fn autolinks_addresses() {
        let d = doc(vec![Para::plain("see https://x.org/a. or www.y.org")]);
        let html = to_html(&d, &|_| String::new());
        assert!(
            html.contains("<a href=\"https://x.org/a\">https://x.org/a</a>."),
            "{html}"
        );
        assert!(html.contains("<a href=\"http://www.y.org\">www.y.org</a>"));
        assert_eq!(find_url("nohttps://x"), None);
    }

    #[test]
    fn plain_links_show_their_address() {
        let mut p = Para::plain("our site");
        let link: Arc<str> = "https://katna.invenia.in".into();
        p.restyle(4..8, &|s| s.link = Some(link.clone()));
        assert_eq!(
            to_plain(&doc(vec![p])),
            "our site <https://katna.invenia.in>\n"
        );
    }

    #[test]
    fn tables_and_images() {
        let mut d = Doc::default();
        let pos = d.insert_table(d.start(), 1, 2);
        d.insert_text(pos, "x", &CharStyle::default());
        d.insert_text(Pos::new(Path::cell(0, 0, 1), 0), "y", &CharStyle::default());
        let at = d.end();
        d.insert_image(
            at,
            Image {
                id: 7,
                name: "cat.png".into(),
                mime: "image/png".into(),
                data: Arc::new(vec![1, 2, 3]),
                width: 900,
                height: 10,
                size: ImageSize::BestFit,
            },
        );
        let html = to_html(&d, &|i| format!("cid:{}", i.id));
        assert!(html.contains(
            "<td style=\"border:1px solid rgb(204,204,204);padding:4px 8px;min-width:40px\">x</td>"
        ));
        assert!(
            html.contains("<img src=\"cid:7\" alt=\"cat.png\" width=\"600\""),
            "{html}"
        );
        assert_eq!(to_plain(&d), "x | y\n[image: cat.png]\n\n");
    }

    #[test]
    fn html_round_trips() {
        let mut a = Para::plain("Kay Mann bold");
        a.restyle(9..13, &|s| s.bold = true);
        a.restyle(0..3, &|s| {
            s.font = Font::Georgia;
            s.size = Size::Large;
            s.color = Some(0x1155cc);
        });
        let link: Arc<str> = "https://katna.invenia.in".into();
        let mut b = Para::plain("site");
        b.restyle(0..4, &|s| s.link = Some(link.clone()));
        b.style.align = Align::Center;
        let mut c = Para::plain("item");
        c.style.list = List::Numbered;
        let mut sig = Para::plain("-- ");
        sig.style.signature = true;
        let mut original = doc(vec![a, Para::default(), b, c, sig]);
        let pos = original.insert_table(Pos::new(Path::top(1), 0), 1, 2);
        original.insert_text(pos, "a\nb", &CharStyle::default());
        let image = Image {
            id: 1,
            name: "logo.png".into(),
            mime: "image/png".into(),
            data: Arc::new(b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR\0\0\0\x10\0\0\0\x08".to_vec()),
            width: 16,
            height: 8,
            size: ImageSize::BestFit,
        };
        let end = original.end();
        original.insert_image(end, image);
        let html = to_html(&original, &data_uri);
        let mut next = 0;
        let back = from_html(&html, &mut next);
        assert_eq!(back, original, "{html}");
    }

    #[test]
    fn reads_foreign_html() {
        let mut next = 0;
        let d = from_html(
            "<p>One<br>Two</p>\n<div><strong>Bold</strong> &amp; <font color=\"#f00\">red</font></div>",
            &mut next,
        );
        let texts: Vec<_> = d
            .blocks
            .iter()
            .map(|b| match b {
                Block::Para(p) => p.text.clone(),
                _ => String::new(),
            })
            .collect();
        assert_eq!(texts, ["One", "Two", "Bold & red"]);
        let Block::Para(p) = &d.blocks[2] else {
            panic!()
        };
        assert!(p.style_at(1).bold);
        assert_eq!(p.style_at(p.len()).color, Some(0xff0000));
    }

    fn texts(d: &Doc) -> Vec<String> {
        d.blocks
            .iter()
            .map(|b| match b {
                Block::Para(p) => p.text.clone(),
                Block::Table(t) => t
                    .rows
                    .iter()
                    .map(|r| {
                        r.iter()
                            .map(|c| c.text.as_str())
                            .collect::<Vec<_>>()
                            .join("|")
                    })
                    .collect::<Vec<_>>()
                    .join("/"),
                Block::Image(i) => format!("[{}]", i.name),
                Block::Html(h) => format!("<{}>", h.text),
            })
            .collect()
    }

    #[test]
    fn reads_excel_cells() {
        let html = "<html xmlns:o=\"urn:schemas-microsoft-com:office:office\"><head>\
            <meta name=ProgId content=Excel.Sheet><style>\n<!--table\n\t{mso-displayed-decimal-separator:\"\\.\";}\n\
            @page\n\t{margin:.75in .7in .75in .7in;}\ntd\n\t{padding-top:1px;\n\tcolor:black;\n\tfont-size:11.0pt;\n\t\
            font-weight:400;\n\tfont-family:Calibri, sans-serif;}\n.xl65\n\t{font-weight:700;\n\tbackground:yellow;\n\t\
            mso-pattern:black none;}\n.xl66\n\t{text-align:right;}\n-->\n</style></head><body link=\"#0563C1\">\
            <table border=0 cellpadding=0 cellspacing=0 width=128 style='border-collapse:collapse;width:96pt'>\
            <!--StartFragment-->\n <col width=64 span=2 style='width:48pt'>\n <tr height=20 style='height:15.0pt'>\n  \
            <td height=20 class=xl65 width=64 style='height:15.0pt;width:48pt'>Name</td>\n  \
            <td class=xl65 width=64 style='width:48pt'>Amount</td>\n </tr>\n <tr height=20>\n  \
            <td height=20>Tea &amp; cake</td>\n  <td class=xl66 align=right>1,200</td>\n </tr>\n \
            <tr><td colspan=2 style='mso-ignore:colspan'>Total</td></tr>\n<!--EndFragment-->\n</table></body></html>";
        let mut next = 0;
        let d = from_pasted_html(html, &mut next);
        assert_eq!(texts(&d), ["Name|Amount/Tea & cake|1,200/Total|"]);
        let Block::Table(t) = &d.blocks[0] else {
            panic!()
        };
        assert!(t.rows[0][1].style_at(1).bold);
        assert_eq!(t.rows[0][0].style.fill, Some(0xffff00));
        assert_eq!(t.rows[0][0].style_at(1).background, None);
        assert!(!t.rows[1][0].style_at(1).bold);
        assert_eq!(t.rows[1][0].style_at(1).color, None);
        assert_eq!(t.rows[1][1].style.align, Align::Right);
    }

    #[test]
    fn reads_libreoffice_cells() {
        let html = "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.0 Transitional//EN\">\n<html><head>\
            <meta http-equiv=\"content-type\" content=\"text/html; charset=utf-8\"/><title>x</title>\
            <style type=\"text/css\">\n\t\tbody,div,table,thead,tbody,tfoot,tr,th,td,p { font-family:\"Liberation Sans\"; font-size:x-small }\n\
            \t\ta.comment-indicator:hover + comment { background:#ffd; position:absolute; display:block; border:1px solid black; padding:0.5em;  }\n\
            \t</style></head><body>\n<table cellspacing=\"0\" border=\"0\">\n\t<colgroup width=\"85\"></colgroup>\n\t<tr>\n\
            \t\t<td height=\"17\" align=\"left\" bgcolor=\"#FFFF00\"><b><font color=\"#000000\">Name</font></b></td>\n\
            \t\t<td align=\"right\" sdval=\"12\" sdnum=\"1033;\"><font color=\"#C9211E\">12</font></td>\n\t</tr>\n\
            \t<tr>\n\t\t<td rowspan=2 valign=middle>Tall</td>\n\t\t<td>a</td>\n\t</tr>\n\t<tr>\n\t\t<td>b</td>\n\t</tr>\n</table>\n</body>\n</html>";
        let mut next = 0;
        let d = from_pasted_html(html, &mut next);
        assert_eq!(texts(&d), ["Name|12/Tall|a/|b"]);
        let Block::Table(t) = &d.blocks[0] else {
            panic!()
        };
        assert_eq!(t.rows[0][0].style.fill, Some(0xffff00));
        assert!(t.rows[0][0].style_at(1).bold);
        assert_eq!(t.rows[0][1].style_at(1).color, Some(0xc9211e));
        assert_eq!(t.rows[0][1].style.align, Align::Right);
    }

    #[test]
    fn reads_libreoffice_writer_lists() {
        let html = "<html><head><style type=\"text/css\">\n\t\th2.western { font-family: \"Liberation Serif\", serif; font-size: 18pt; font-weight: bold }\n\
            \t\tp { background: transparent }\n\t</style></head><body lang=\"en-US\"><h2 class=\"western\">\nPlan</h2>\n\
            <p>Some <b>bold</b> text.</p>\n<ul>\n\t<li><p style=\"margin-bottom: 0in\">First</p></li>\n\t<li><p>Second</p></li>\n</ul>\n\
            <ol><li><p>One</p></li></ol>\n</body></html>";
        let mut next = 0;
        let d = from_pasted_html(html, &mut next);
        assert_eq!(
            texts(&d),
            ["Plan", "Some bold text.", "First", "Second", "One"]
        );
        let lists: Vec<List> = d
            .blocks
            .iter()
            .map(|b| match b {
                Block::Para(p) => p.style.list,
                _ => List::None,
            })
            .collect();
        assert_eq!(
            lists,
            [
                List::None,
                List::None,
                List::Bullet,
                List::Bullet,
                List::Numbered
            ]
        );
    }

    #[test]
    fn reads_google_sheets_cells() {
        let html = "<google-sheets-html-origin><style type=\"text/css\"><!--td {border: 1px solid #cccccc;}br {mso-data-placement:same-cell;}--></style>\
            <table xmlns=\"http://www.w3.org/1999/xhtml\" cellspacing=\"0\" cellpadding=\"0\" dir=\"ltr\" border=\"1\" style=\"table-layout:fixed;font-size:10pt;font-family:Arial;width:0px;border-collapse:collapse;border:none\">\
            <colgroup><col width=\"100\"/><col width=\"100\"/></colgroup><tbody><tr style=\"height:21px;\">\
            <td style=\"overflow:hidden;padding:2px 3px 2px 3px;vertical-align:bottom;background-color:#ffff00;font-weight:bold;\">A</td>\
            <td style=\"overflow:hidden;text-align:right;\" data-sheets-value=\"{&quot;1&quot;:3,&quot;3&quot;:1}\">1</td></tr></tbody></table>";
        let mut next = 0;
        let d = from_pasted_html(html, &mut next);
        assert_eq!(texts(&d), ["A|1"]);
        let Block::Table(t) = &d.blocks[0] else {
            panic!()
        };
        assert_eq!(t.rows[0][0].style.fill, Some(0xffff00));
        assert!(t.rows[0][0].style_at(1).bold);
        assert_eq!(t.rows[0][1].style.align, Align::Right);
    }

    #[test]
    fn reads_word_text_and_lists() {
        let html = "<html xmlns:o=\"urn:schemas-microsoft-com:office:office\"><head><style><!--\n /* Style Definitions */\n\
            p.MsoNormal, li.MsoNormal, div.MsoNormal\n\t{margin:0in;\n\tfont-size:11.0pt;\n\tfont-family:\"Calibri\",sans-serif;}\n\
            -->\n</style><!--[if gte mso 10]><style>table.MsoNormalTable{}</style><![endif]--></head>\
            <body lang=EN-US style='tab-interval:.5in'>\n<!--StartFragment-->\n\
            <p class=MsoNormal><b>Hello</b> <span style='color:#C00000'>world</span><o:p></o:p></p>\n\
            <p class=MsoListParagraphCxSpFirst style='text-indent:-.25in;mso-list:l0 level1 lfo1'><![if !supportLists]>\
            <span style='font-family:Symbol'><span style='mso-list:Ignore'>\u{b7}<span style='font:7.0pt \"Times New Roman\"'>&nbsp;&nbsp;&nbsp; </span></span></span><![endif]>First<o:p></o:p></p>\n\
            <p class=MsoListParagraphCxSpLast style='margin-left:1.0in;text-indent:-.25in;mso-list:l1 level2 lfo2'><![if !supportLists]>\
            <span style='mso-list:Ignore'>a.<span style='font:7.0pt \"Times New Roman\"'>&nbsp;&nbsp; </span></span><![endif]>Second<o:p></o:p></p>\n\
            <p class=MsoNormal><o:p>&nbsp;</o:p></p>\n<p class=MsoNormal>End<o:p></o:p></p>\n<!--EndFragment-->\n</body></html>";
        let mut next = 0;
        let d = from_pasted_html(html, &mut next);
        assert_eq!(texts(&d), ["Hello world", "First", "Second", "", "End"]);
        let para = |ix: usize| match &d.blocks[ix] {
            Block::Para(p) => p.clone(),
            _ => panic!(),
        };
        assert!(para(0).style_at(1).bold);
        assert_eq!(para(0).style_at(8).color, Some(0xc00000));
        assert_eq!(
            (para(1).style.list, para(1).style.indent),
            (List::Bullet, 0)
        );
        assert_eq!(
            (para(2).style.list, para(2).style.indent),
            (List::Numbered, 1)
        );
    }

    #[test]
    fn reads_web_pages() {
        let html = "<meta charset='utf-8'><h2 style=\"color: rgb(32, 33, 36); font-family: Roboto, Arial; \
            background-color: rgb(255, 255, 255);\">Title</h2><p style=\"color: rgb(32, 33, 36); font-size: 16px;\">\
            Some <a href=\"https://x.org\">link</a> <b style=\"font-weight:normal\">plain</b> <span style=\"color:#1a73e8\">blue</span>.</p>\
            <ul><li>one<ul><li>two</li></ul></li><li>three</li></ul><table><tr><td><table><tr><td>in</td><td>ner</td></tr></table></td><td>x</td></tr></table>\
            <script>var a = '<b>no</b>';</script>";
        let mut next = 0;
        let d = from_pasted_html(html, &mut next);
        assert_eq!(
            texts(&d),
            [
                "Title",
                "Some link plain blue.",
                "one",
                "two",
                "three",
                "in ner|x"
            ]
        );
        let para = |ix: usize| match &d.blocks[ix] {
            Block::Para(p) => p.clone(),
            _ => panic!(),
        };
        let title = para(0).style_at(1);
        assert!(title.bold);
        assert_eq!((title.color, title.background), (None, None));
        let body = para(1);
        assert_eq!(body.style_at(1).color, None);
        assert!(body.style_at(6).link.is_some());
        assert!(!body.style_at(12).bold);
        assert_eq!(body.style_at(18).color, Some(0x1a73e8));
        assert_eq!(para(3).style.indent, 1);
        assert_eq!(para(4).style.indent, 0);
    }

    #[test]
    fn plain_text_quotes_become_quote_depth() {
        let d = from_plain("hi\n> a\n>> b\n");
        let quotes: Vec<_> = d
            .blocks
            .iter()
            .map(|b| match b {
                Block::Para(p) => (p.text.clone(), p.style.quote),
                _ => (String::new(), 0),
            })
            .collect();
        assert_eq!(
            quotes,
            [
                ("hi".into(), 0),
                ("a".into(), 1),
                ("b".into(), 2),
                (String::new(), 0)
            ]
        );
        assert_eq!(to_plain(&d), "hi\n> a\n> > b\n\n");
    }

    #[test]
    fn base64_round_trips() {
        for data in [&b""[..], b"f", b"fo", b"foo", b"foob", b"hello world!"] {
            assert_eq!(base64_decode(&base64_encode(data)).unwrap(), data);
        }
        assert_eq!(base64_encode(b"foob"), "Zm9vYg==");
        assert_eq!(base64_decode("Zm9v\r\nYg==").unwrap(), b"foob");
    }

    #[test]
    fn colors() {
        assert_eq!(parse_color("#abc"), Some(0xaabbcc));
        assert_eq!(parse_color("rgb(1, 2, 3)"), Some(0x010203));
        assert_eq!(parse_color("Red"), Some(0xff0000));
        assert_eq!(parse_color("windowtext"), None);
        assert_eq!(parse_color("rgb(1 2 3 / 50%)"), Some(0x010203));
    }

    #[test]
    fn designed_blocks_round_trip() {
        let png = "data:image/png;base64,iVBORw0KGgo=";
        let designed = format!(
            "<table><tr><td style=\"background:#0b7\"><img src=\"{png}\" width=\"48\"></td>\
             <td><b>Kay Rowe</b><br>Enron</td></tr></table>"
        );
        let mut next = 0;
        let block = html_block(&designed, &mut next);
        assert_eq!(block.images.len(), 1);
        assert!(block.html.contains("src=\"cid:katna-0\""));
        assert!(!block.html.contains("base64"));
        assert!(block.text.contains("Kay Rowe"));

        let again = html_block_with(&block.html, &block.images, &mut next);
        assert_eq!(again.html, block.html);
        assert_eq!(again.images.len(), 1);

        // In a message the pictures are parts; a draft reads them back.
        let mut doc = from_plain("Hi");
        doc.blocks.push(Block::Html(block));
        let html = to_html(&doc, &|image| format!("cid:part{}", image.id));
        assert!(
            html.contains(&format!("src=\"cid:part{}\"", next - 1)),
            "{html}"
        );
        let reopened = from_html(&to_html(&doc, &data_uri), &mut next);
        assert!(matches!(reopened.blocks.last(), Some(Block::Html(h)) if h.images.len() == 1));
        assert_eq!(to_plain(&doc), "Hi\nKay Rowe\nEnron\n");
    }
}
