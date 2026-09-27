//! Clipboard and drag-and-drop content beyond plain text (added for Katna;
//! not in upstream GPUI).
//!
//! GPUI reads plain text or one picture from the clipboard, and takes only
//! files in a drop. An app that wants more:
//!
//! - Reads the clipboard inside [`read_rich`]. The item then carries
//!   everything the owner offered that Katna can use: the plain text and
//!   the HTML (one string entry: the text, with the HTML in its metadata,
//!   see [`clipboard_html`]), a picture, and copied files (a file manager's
//!   Ctrl+C) as [`ExternalPaths`]. Outside `read_rich` nothing changes.
//! - Writes an item made by [`html_item`] to offer HTML beside the text.
//! - Takes content dropped from other apps (cells from a spreadsheet, text
//!   from a browser, a picture): it arrives as a file drop of one made-up
//!   path, and [`dropped_content`] gives what was dropped.

// Only the Wayland and X11 clients read and write the clipboard.
#![cfg_attr(not(any(feature = "wayland", feature = "x11")), allow(dead_code))]

use std::cell::Cell;
use std::path::PathBuf;
use std::sync::Mutex;

use gpui::{ClipboardEntry, ClipboardItem, ClipboardString, ExternalPaths, Image, ImageFormat};
use smallvec::SmallVec;
use strum::IntoEnumIterator;

/// HTML's MIME type.
pub(crate) const HTML_MIME: &str = "text/html";
/// A list of file addresses (a file manager's copy, or a file drag).
pub(crate) const URI_LIST_MIME: &str = "text/uri-list";
/// GNOME's (and KDE's, for GNOME apps) copied files: "copy" or "cut",
/// then the addresses.
pub(crate) const GNOME_FILES_MIME: &str = "x-special/gnome-copied-files";
/// Plain text types, best first.
pub(crate) const TEXT_MIMES: [&str; 4] = [
    "text/plain;charset=utf-8",
    "UTF8_STRING",
    "text/plain",
    "STRING",
];

/// Starts the metadata of a string entry that carries HTML.
const HTML_TAG: &str = "katna:text/html\n";
/// The path a content drop arrives as.
const DROPPED_PATH: &str = "/.katna-dropped-content";

thread_local! {
    static RICH: Cell<bool> = const { Cell::new(false) };
}

static DROPPED: Mutex<Option<DroppedContent>> = Mutex::new(None);

/// Runs `read` (which reads the clipboard) so that the clipboard is read
/// with everything Katna can use.
pub fn read_rich<R>(read: impl FnOnce() -> R) -> R {
    RICH.with(|r| r.set(true));
    let out = read();
    RICH.with(|r| r.set(false));
    out
}

pub(crate) fn rich_wanted() -> bool {
    RICH.with(Cell::get)
}

/// An item holding `plain` text with its `html` form.
pub fn html_item(plain: String, html: String) -> ClipboardItem {
    ClipboardItem {
        entries: vec![ClipboardEntry::String(ClipboardString {
            text: plain,
            metadata: Some(format!("{HTML_TAG}{html}")),
        })],
    }
}

/// The HTML an item holds, if any.
pub fn clipboard_html(item: &ClipboardItem) -> Option<&str> {
    item.entries().iter().find_map(|entry| match entry {
        ClipboardEntry::String(s) => s.metadata.as_deref()?.strip_prefix(HTML_TAG),
        _ => None,
    })
}

/// Content dropped from another app.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DroppedContent {
    pub text: Option<String>,
    pub html: Option<String>,
    pub image: Option<Image>,
}

/// What was dropped, when `paths` is a content drop rather than files.
pub fn dropped_content(paths: &ExternalPaths) -> Option<DroppedContent> {
    match paths.paths() {
        [path] if path.as_os_str() == DROPPED_PATH => DROPPED.lock().ok()?.clone(),
        _ => None,
    }
}

/// Keeps `content` for [`dropped_content`] and gives the paths its drop
/// arrives as.
pub(crate) fn enter_content(content: DroppedContent) -> ExternalPaths {
    if let Ok(mut dropped) = DROPPED.lock() {
        *dropped = Some(content);
    }
    ExternalPaths(SmallVec::from_iter([PathBuf::from(DROPPED_PATH)]))
}

/// Everything read from a clipboard or a drag, by MIME type.
#[derive(Debug, Default)]
pub(crate) struct Transfer {
    pub text: Option<String>,
    pub html: Option<String>,
    pub image: Option<Image>,
    pub files: Vec<PathBuf>,
}

impl Transfer {
    /// Takes the data read as `mime`.
    pub fn add(&mut self, mime: &str, bytes: Vec<u8>) {
        if mime == HTML_MIME {
            if self.html.is_none() {
                self.html = decode_text(bytes).map(|html| strip_cf_html(&html).to_owned());
            }
        } else if mime == URI_LIST_MIME || mime == GNOME_FILES_MIME {
            if self.files.is_empty()
                && let Some(list) = decode_text(bytes)
            {
                self.files = file_paths(&list);
            }
        } else if TEXT_MIMES.contains(&mime) {
            if self.text.is_none() {
                self.text = if mime == "STRING" {
                    // ISO Latin-1.
                    Some(bytes.into_iter().map(char::from).collect())
                } else {
                    decode_text(bytes)
                }
                .map(|t| t.replace("\r\n", "\n"));
            }
        } else if let Some(format) = image_format(mime)
            && self.image.is_none()
            && !bytes.is_empty()
        {
            let id = gpui::hash(&bytes);
            self.image = Some(Image { format, bytes, id });
        }
    }

    /// As a clipboard item: the text and HTML, the picture, and the files.
    pub fn into_item(self) -> Option<ClipboardItem> {
        let mut entries = Vec::new();
        if self.text.is_some() || self.html.is_some() {
            entries.push(ClipboardEntry::String(ClipboardString {
                text: self.text.unwrap_or_default(),
                metadata: self.html.map(|html| format!("{HTML_TAG}{html}")),
            }));
        }
        if let Some(image) = self.image {
            entries.push(ClipboardEntry::Image(image));
        }
        if !self.files.is_empty() {
            entries.push(ClipboardEntry::ExternalPaths(ExternalPaths(
                self.files.into_iter().collect(),
            )));
        }
        (!entries.is_empty()).then_some(ClipboardItem { entries })
    }

    /// As a content drop, when it is not files.
    pub fn into_dropped(self) -> Option<DroppedContent> {
        (self.text.is_some() || self.html.is_some() || self.image.is_some()).then_some(
            DroppedContent {
                text: self.text,
                html: self.html,
                image: self.image,
            },
        )
    }
}

/// The input for a drag moving over a window: a move with the button held.
///
/// GPUI turns `FileDropEvent::Pending` into the same move, but only after it
/// has noted what kind of input came last, so after typing it still thinks
/// the keyboard came last and no element counts as hovered: the drop then
/// lands nowhere. A plain move marks the mouse as the last input.
pub(crate) fn drag_move(position: gpui::Point<gpui::Pixels>) -> gpui::PlatformInput {
    gpui::PlatformInput::MouseMove(gpui::MouseMoveEvent {
        position,
        pressed_button: Some(gpui::MouseButton::Left),
        modifiers: gpui::Modifiers::default(),
    })
}

/// The types worth reading out of those `offered`, in the order to read
/// them: files, HTML, one plain text type and one picture type.
pub(crate) fn wanted_mimes<'a>(offered: &[&'a str]) -> Vec<&'a str> {
    let mut wanted = Vec::new();
    let has = |mime: &str| offered.iter().copied().find(|o| *o == mime);
    for mime in [URI_LIST_MIME, GNOME_FILES_MIME, HTML_MIME] {
        if let Some(found) = has(mime) {
            wanted.push(found);
        }
    }
    if let Some(text) = TEXT_MIMES.iter().find_map(|m| has(m)) {
        wanted.push(text);
    }
    if let Some(image) = ImageFormat::iter().find_map(|f| has(f.mime_type())) {
        wanted.push(image);
    }
    wanted
}

fn image_format(mime: &str) -> Option<ImageFormat> {
    ImageFormat::iter().find(|f| f.mime_type() == mime)
}

/// Text as UTF-8, or UTF-16 with a byte order mark (Firefox has written
/// HTML that way), without a trailing NUL.
pub(crate) fn decode_text(bytes: Vec<u8>) -> Option<String> {
    let text = match bytes.as_slice() {
        [0xff, 0xfe, rest @ ..] => utf16(rest, u16::from_le_bytes)?,
        [0xfe, 0xff, rest @ ..] => utf16(rest, u16::from_be_bytes)?,
        _ => match String::from_utf8(bytes) {
            Ok(text) => text,
            Err(err) => String::from_utf8_lossy(err.as_bytes()).into_owned(),
        },
    };
    Some(text.trim_end_matches('\0').to_owned())
}

fn utf16(bytes: &[u8], read: fn([u8; 2]) -> u16) -> Option<String> {
    let units = bytes.chunks_exact(2).map(|c| read([c[0], c[1]]));
    char::decode_utf16(units)
        .collect::<Result<String, _>>()
        .ok()
}

/// Windows' "HTML Format" (which Office under Wine can pass on) starts
/// with a header of offsets: the HTML is what follows it.
fn strip_cf_html(html: &str) -> &str {
    if html.starts_with("Version:")
        && let Some(start) = html.find('<')
    {
        return &html[start..];
    }
    html
}

/// The local files in a URI list (or GNOME's copied-files list, whose
/// first line says "copy" or "cut").
pub(crate) fn file_paths(list: &str) -> Vec<PathBuf> {
    list.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| url::Url::parse(line).ok())
        .filter(|url| url.scheme() == "file")
        .filter_map(|url| url.to_file_path().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_file_lists() {
        assert_eq!(
            file_paths("copy\nfile:///home/me/a%20b.pdf\r\nfile:///tmp/c.png\n"),
            vec![
                PathBuf::from("/home/me/a b.pdf"),
                PathBuf::from("/tmp/c.png")
            ]
        );
        assert!(file_paths("https://example.com/x.png").is_empty());
    }

    #[test]
    fn picks_types() {
        let offered = [
            "text/plain",
            "image/png",
            "text/html",
            "application/x-openoffice-embed-source-xml",
            "UTF8_STRING",
        ];
        assert_eq!(
            wanted_mimes(&offered),
            vec!["text/html", "UTF8_STRING", "image/png"]
        );
    }

    #[test]
    fn decodes_text() {
        let mut bytes = vec![0xff, 0xfe];
        for unit in "<b>é</b>".encode_utf16() {
            bytes.extend(unit.to_le_bytes());
        }
        assert_eq!(decode_text(bytes).as_deref(), Some("<b>é</b>"));
        assert_eq!(decode_text(b"x\0".to_vec()).as_deref(), Some("x"));
    }

    #[test]
    fn items_carry_html() {
        let mut transfer = Transfer::default();
        transfer.add(
            "text/html",
            b"Version:0.9\r\nStartHTML:0000000105\r\n<table>".to_vec(),
        );
        transfer.add("UTF8_STRING", b"a\tb\r\n".to_vec());
        let item = transfer.into_item().unwrap();
        assert_eq!(clipboard_html(&item), Some("<table>"));
        assert_eq!(item.text().as_deref(), Some("a\tb\n"));
        assert_eq!(
            clipboard_html(&html_item("x".into(), "<b>x</b>".into())),
            Some("<b>x</b>")
        );
    }
}
