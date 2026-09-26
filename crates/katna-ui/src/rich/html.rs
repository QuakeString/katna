// SPDX-License-Identifier: GPL-3.0-or-later

//! A [`Doc`] as mail HTML and as plain text, and back from the HTML this
//! writes (signatures are stored that way) and from plain text. The HTML
//! uses inline styles only, which mail readers keep. No GPUI here.

use std::fmt::Write as _;
use std::ops::Range;
use std::sync::Arc;

use super::doc::{
    Align, Block, CharStyle, Doc, Font, Image, ImageSize, List, Para, ParaStyle, Size, Table,
    image_size, list_marker, list_numbers,
};

/// Widest an image is sent at when it fits the text.
const BEST_FIT_WIDTH: f32 = 600.0;
const QUOTE_STYLE: &str =
    "margin:0px 0px 0px 0.8ex;border-left:1px solid rgb(204,204,204);padding-left:1ex";
const CELL_STYLE: &str = "border:1px solid rgb(204,204,204);padding:4px 8px;min-width:40px";

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
        }
    }
    writer.enter(&ParaStyle::default());
    out.push_str("</div>");
    out
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
                let _ = write!(self.out, "<td style=\"{CELL_STYLE}{align}\">");
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
    let mut reader = Reader {
        blocks: Vec::new(),
        para: Para::default(),
        open: false,
        styles: vec![CharStyle::default()],
        para_style: vec![ParaStyle::default()],
        lists: Vec::new(),
        divs: Vec::new(),
        table: None,
        next_id,
    };
    let mut rest = html;
    while !rest.is_empty() {
        if let Some(after) = rest.strip_prefix("<!--") {
            rest = after.find("-->").map_or("", |e| &after[e + 3..]);
            continue;
        }
        if rest.starts_with('<')
            && let Some(end) = rest.find('>')
        {
            reader.tag(&rest[1..end]);
            rest = &rest[end + 1..];
            continue;
        }
        // Past the first character, which may be a lone '<' or take more
        // than one byte.
        let first = rest.chars().next().map_or(1, char::len_utf8);
        let end = rest[first..].find('<').map_or(rest.len(), |e| e + first);
        reader.text(&decode_entities(&rest[..end]));
        rest = &rest[end..];
    }
    reader.flush(false);
    if reader.blocks.is_empty() {
        reader.blocks.push(Block::Para(Para::default()));
    }
    Doc {
        blocks: reader.blocks,
    }
}

struct Reader<'a> {
    blocks: Vec<Block>,
    para: Para,
    /// A block element started this paragraph: it is kept even if empty.
    open: bool,
    styles: Vec<CharStyle>,
    para_style: Vec<ParaStyle>,
    lists: Vec<List>,
    /// Whether each open `div` was a signature.
    divs: Vec<bool>,
    table: Option<Table>,
    next_id: &'a mut u64,
}

impl Reader<'_> {
    fn style(&self) -> CharStyle {
        self.styles.last().cloned().unwrap_or_default()
    }

    fn block_style(&self) -> ParaStyle {
        self.para_style.last().copied().unwrap_or_default()
    }

    fn text(&mut self, text: &str) {
        // Layout whitespace between tags is not text.
        let collapsed = collapse(text);
        if collapsed.trim_matches(' ').is_empty() && (self.para.is_empty() || self.table.is_some())
        {
            if !self.para.is_empty() && collapsed.contains(' ') {
                let style = self.style();
                self.para.insert(self.para.len(), " ", &style);
            }
            return;
        }
        let style = self.style();
        let text = if self.para.is_empty() {
            collapsed.trim_start_matches(' ').to_owned()
        } else {
            collapsed
        };
        let len = self.para.len();
        self.para.insert(len, &text, &style);
    }

    /// Ends the paragraph being read. `keep` keeps it even when empty.
    fn flush(&mut self, keep: bool) {
        let mut para = finish(std::mem::take(&mut self.para), &[' ']);
        let wanted = keep || self.open || !para.is_empty();
        self.open = false;
        if !wanted {
            return;
        }
        if let Some(table) = &mut self.table {
            // Text in a table outside a cell: into the last cell.
            if let Some(cell) = table.rows.last_mut().and_then(|r| r.last_mut()) {
                cell.append(para);
            }
            return;
        }
        if para.style == ParaStyle::default() {
            para.style = self.block_style();
        }
        self.blocks.push(Block::Para(para));
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

    fn tag(&mut self, raw: &str) {
        let raw = raw.trim().trim_end_matches('/').trim();
        let (closing, raw) = match raw.strip_prefix('/') {
            Some(r) => (true, r),
            None => (false, raw),
        };
        let name_end = raw.find(|c: char| c.is_whitespace()).unwrap_or(raw.len());
        let name = raw[..name_end].to_ascii_lowercase();
        let attrs = &raw[name_end..];
        match (name.as_str(), closing) {
            ("br", _) => self.line_break(),
            ("b" | "strong", false) => self.push_style(|s| s.bold = true),
            ("i" | "em", false) => self.push_style(|s| s.italic = true),
            ("u" | "ins", false) => self.push_style(|s| s.underline = true),
            ("s" | "strike" | "del", false) => self.push_style(|s| s.strike = true),
            ("a", false) => {
                let href = attr(attrs, "href").map(|h| Arc::<str>::from(decode_entities(&h)));
                self.push_style(|s| s.link = href.clone());
            }
            ("span" | "font", false) => {
                let css = attr(attrs, "style").unwrap_or_default();
                let color = attr(attrs, "color");
                let face = attr(attrs, "face");
                let size = attr(attrs, "size");
                self.push_style(|s| {
                    apply_css(s, &css);
                    if let Some(c) = color.as_deref().and_then(parse_color) {
                        s.color = Some(c);
                    }
                    if let Some(f) = face.as_deref() {
                        s.font = font_named(f);
                    }
                    match size.as_deref() {
                        Some("1" | "2") => s.size = Size::Small,
                        Some("4" | "5") => s.size = Size::Large,
                        Some("6" | "7") => s.size = Size::Huge,
                        _ => {}
                    }
                });
            }
            (
                "b" | "strong" | "i" | "em" | "u" | "ins" | "s" | "strike" | "del" | "a" | "span"
                | "font",
                true,
            ) => {
                if self.styles.len() > 1 {
                    self.styles.pop();
                }
            }
            ("div" | "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6", false) => {
                let css = attr(attrs, "style").unwrap_or_default();
                let class = attr(attrs, "class").unwrap_or_default();
                let signature = class.contains("signature");
                self.divs.push(signature);
                let mut style = self.block_style();
                style.signature |= signature;
                apply_block_css(&mut style, &css);
                if name.starts_with('h') {
                    let size = if name == "h1" {
                        Size::Huge
                    } else {
                        Size::Large
                    };
                    self.push_style(|s| {
                        s.bold = true;
                        s.size = size;
                    });
                }
                self.para_style.push(ParaStyle {
                    align: Align::Left,
                    indent: self.block_style().indent,
                    ..style
                });
                if self.table.is_none() {
                    self.start_para(style);
                }
            }
            ("div" | "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6", true) => {
                if self.table.is_none() {
                    self.flush(false);
                }
                self.divs.pop();
                if self.para_style.len() > 1 {
                    self.para_style.pop();
                }
                if name.starts_with('h') && self.styles.len() > 1 {
                    self.styles.pop();
                }
            }
            ("blockquote", false) => {
                self.flush(false);
                let mut style = self.block_style();
                style.quote = style.quote.saturating_add(1);
                self.para_style.push(style);
            }
            ("blockquote", true) => {
                self.flush(false);
                if self.para_style.len() > 1 {
                    self.para_style.pop();
                }
            }
            ("ul" | "ol", false) => {
                self.flush(false);
                self.lists.push(if name == "ol" {
                    List::Numbered
                } else {
                    List::Bullet
                });
            }
            ("ul" | "ol", true) => {
                self.flush(false);
                self.lists.pop();
            }
            ("li", false) => {
                let mut style = self.block_style();
                style.list = self.lists.last().copied().unwrap_or(List::Bullet);
                style.indent = self.lists.len().saturating_sub(1).min(8) as u8;
                if let Some(css) = attr(attrs, "style") {
                    apply_block_css(&mut style, &css);
                }
                self.start_para(style);
            }
            ("li", true) => self.flush(false),
            ("table", false) => {
                self.drop_empty();
                self.flush(false);
                self.table = Some(Table { rows: Vec::new() });
            }
            ("table", true) => {
                if let Some(mut table) = self.table.take() {
                    table.rows.retain(|r| !r.is_empty());
                    let cols = table.rows.iter().map(Vec::len).max().unwrap_or(0);
                    if cols > 0 {
                        for row in &mut table.rows {
                            row.resize(cols, Para::default());
                        }
                        self.blocks.push(Block::Table(table));
                    }
                }
                self.para = Para::default();
                self.open = false;
            }
            ("tr", false) => {
                if let Some(table) = &mut self.table {
                    table.rows.push(Vec::new());
                }
            }
            ("td" | "th", false) => {
                let mut style = ParaStyle::default();
                if let Some(css) = attr(attrs, "style") {
                    apply_block_css(&mut style, &css);
                }
                self.para = Para::default().with_style(style);
                if name == "th" {
                    self.push_style(|s| s.bold = true);
                }
            }
            ("td" | "th", true) => {
                let cell = std::mem::take(&mut self.para);
                if let Some(table) = &mut self.table {
                    if table.rows.is_empty() {
                        table.rows.push(Vec::new());
                    }
                    if let Some(row) = table.rows.last_mut() {
                        row.push(finish(cell, &[' ', '\n']));
                    }
                }
                if name == "th" && self.styles.len() > 1 {
                    self.styles.pop();
                }
            }
            ("img", false) => {
                if let Some(image) = attr(attrs, "src").and_then(|src| {
                    image_from_data_uri(
                        &src,
                        attr(attrs, "alt").unwrap_or_default(),
                        attr(attrs, "width").and_then(|w| w.parse().ok()),
                        self.next_id,
                    )
                }) {
                    self.drop_empty();
                    self.flush(false);
                    self.blocks.push(Block::Image(image));
                }
            }
            ("style" | "script" | "head" | "title", false) => {}
            _ => {}
        }
    }

    fn push_style(&mut self, f: impl Fn(&mut CharStyle)) {
        let mut style = self.style();
        f(&mut style);
        self.styles.push(style);
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
                style.bold = value == "bold" || value.parse::<u32>().is_ok_and(|w| w >= 600)
            }
            "font-style" => style.italic = value == "italic",
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
                    v => match v.trim_end_matches("px").parse::<f32>() {
                        Ok(px) if px < 12.0 => Size::Small,
                        Ok(px) if px >= 24.0 => Size::Huge,
                        Ok(px) if px >= 17.0 => Size::Large,
                        _ => Size::Normal,
                    },
                }
            }
            "color" => style.color = parse_color(&value),
            "background-color" | "background" => style.background = parse_color(&value),
            _ => {}
        }
    }
}

fn apply_block_css(style: &mut ParaStyle, css: &str) {
    for (key, value) in css_pairs(css) {
        match key.as_str() {
            "text-align" => {
                style.align = match value.as_str() {
                    "center" => Align::Center,
                    "right" => Align::Right,
                    _ => Align::Left,
                }
            }
            "margin-left" => {
                if let Ok(px) = value.trim_end_matches("px").parse::<f32>()
                    && px >= 40.0
                    && style.list == List::None
                {
                    style.indent = ((px / 40.0).round() as u8).min(8);
                }
            }
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
            _ => None,
        })
        .unwrap_or(Font::Sans)
}

/// `#rgb`, `#rrggbb` or `rgb(r, g, b)` as `0xRRGGBB`.
pub fn parse_color(value: &str) -> Option<u32> {
    let value = value.trim();
    if let Some(hex) = value.strip_prefix('#') {
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
    let mut parts = inner.split(',').map(|p| p.trim().parse::<u32>().ok());
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
        assert_eq!(parse_color("red"), None);
    }
}
