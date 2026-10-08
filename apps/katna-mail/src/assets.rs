// SPDX-License-Identifier: GPL-3.0-or-later

//! Icons built into the binary, served to GPUI's `svg()` element, and the
//! language picker's flags (`flags/`, from `flag-icons`, MIT) and Katna's
//! logo (`logo/`), served to `img()`. Brand marks (`icons/brand-*.svg`)
//! are from Simple Icons (CC0), except Yahoo's and Yandex's, from Font
//! Awesome Free (CC BY 4.0), and Fastmail's, from Dashboard Icons
//! (Apache-2.0); each file names its source. The marks stay their owners'
//! trademarks and only stand for their own services.

use std::borrow::Cow;

use gpui::{AssetSource, SharedString};

macro_rules! icons {
    ($($name:literal),* $(,)?) => {
        const ICONS: &[(&str, &[u8])] = &[
            $((concat!("icons/", $name, ".svg"), include_bytes!(concat!("../icons/", $name, ".svg")))),*
        ];
    };
}

icons!(
    "activity",
    "add",
    "align-center",
    "align-left",
    "align-right",
    "all-mail",
    "apps",
    "archive",
    "arrow-down",
    "arrow-up",
    "attachment",
    "back",
    "bell-off",
    "bell-plus",
    "bell",
    "bird",
    "bolt",
    "brand-facebook",
    "brand-fastmail-blue",
    "brand-fastmail-ink",
    "brand-fastmail-sky",
    "brand-fastmail-yellow",
    "brand-github",
    "brand-gmx",
    "brand-icloud",
    "brand-instagram",
    "brand-linkedin",
    "brand-telegram",
    "brand-whatsapp",
    "brand-x",
    "brand-yahoo",
    "brand-yandex",
    "brand-youtube",
    "brand-zoho",
    "bug",
    "cake",
    "calendar",
    "chat",
    "check-circle",
    "check",
    "checkbox-checked",
    "checkbox-partial",
    "checkbox",
    "chevron-down",
    "chevron-left",
    "chevron-right",
    "chevron-up",
    "chip",
    "clear-format",
    "close-full",
    "close",
    "cloud-off",
    "cloud",
    "code",
    "coffee",
    "color-wheel",
    "compose",
    "contacts",
    "contrast",
    "copy",
    "delivery-receipt",
    "divider",
    "document",
    "done-all",
    "download",
    "drafts",
    "drag-handle",
    "drop-down",
    "emoji",
    "eraser",
    "event",
    "expand",
    "eye-off",
    "eye",
    "eyedropper",
    "file",
    "filter",
    "fit-page",
    "fit-width",
    "flight",
    "folder-add",
    "folder",
    "folders-pane-fill",
    "folders-pane",
    "font",
    "format-bold",
    "format-italic",
    "format-squiggle",
    "format-strike",
    "format-text",
    "format-underline",
    "forum",
    "forward",
    "google-drive-blue",
    "google-drive-green",
    "google-drive-yellow",
    "google-g-blue",
    "google-g-green",
    "google-g-red",
    "google-g-yellow",
    "headphones",
    "heart",
    "highlight",
    "history",
    "home",
    "image",
    "important-filled",
    "important",
    "inbox",
    "indent-less",
    "indent-more",
    "info",
    "insert-below",
    "junk",
    "key",
    "label",
    "language",
    "link",
    "list-bulleted",
    "list-numbered",
    "location",
    "lock",
    "mail",
    "mark-read",
    "mark-unread",
    "menu",
    "minimize",
    "more",
    "move-to",
    "no-fill",
    "notch",
    "notes",
    "onedrive",
    "open-external",
    "open-full",
    "outbox",
    "palette",
    "paw",
    "pen-sparkle",
    "pen",
    "people",
    "person-add",
    "phone",
    "pin-filled",
    "pin",
    "plain-text",
    "play",
    "pointer",
    "power",
    "print",
    "pulse",
    "qr-code",
    "quote",
    "read-receipt",
    "real-size",
    "redo",
    "refresh",
    "remove",
    "repeat",
    "reply-all",
    "reply",
    "restore",
    "rotate-ccw",
    "rotate-cw",
    "schedule",
    "search",
    "send",
    "sent",
    "settings",
    "sheet",
    "shield-alert",
    "shield-check",
    "shield",
    "signature",
    "slides",
    "snooze",
    "sparkle",
    "spell-check",
    "spinner",
    "star-filled",
    "star",
    "subtask",
    "sunrise",
    "table",
    "tag",
    "tasks",
    "template",
    "text-color",
    "text-size",
    "today",
    "tour",
    "translate",
    "trash",
    "tune",
    "undo",
    "unread",
    "upload",
    "video",
    "volume",
    "warning",
    "water-drop",
    "window-restore",
    "work",
    "zoom-in",
    "zoom-out",
);

macro_rules! flags {
    ($($code:literal),* $(,)?) => {
        const FLAGS: &[(&str, &[u8])] = &[
            $((concat!("flags/", $code, ".svg"), include_bytes!(concat!("../flags/", $code, ".svg")))),*
        ];
    };
}

flags!(
    "bd", "br", "bt", "cn", "de", "es", "et", "fr", "gb", "id", "il", "in", "ir", "it", "jp", "ke",
    "kh", "kr", "la", "lk", "mm", "my", "ng", "nl", "np", "ph", "pk", "pl", "ru", "sa", "se", "th",
    "tr", "tw", "ua", "us", "vn", "za",
);

/// The flags are 4:3 with only a view box; GPUI draws an SVG picture at
/// its own size, so each is given 48 × 36, twice the size it is shown.
fn flag(data: &[u8]) -> Vec<u8> {
    let text = String::from_utf8_lossy(data);
    text.replacen("<svg ", r#"<svg width="48" height="36" "#, 1)
        .into_bytes()
}

/// Katna's logo, the k on its teal disc: with its shadow, and without it
/// for under 48 px, where the shadow blurs to mush. And Katna Mail's
/// wordmark, for the large places (About, the welcome).
const LOGO: &[u8] = include_bytes!("../../../packaging/icons/src/katna.svg");
const LOGO_SMALL: &[u8] = include_bytes!("../../../packaging/icons/src/katna-small.svg");
const WORDMARK: &[u8] = include_bytes!("../../../packaging/icons/src/katna-wordmark.svg");
/// The wordmark's own width and height, as its `<svg>` gives them.
pub const WORDMARK_SIZE: (u32, u32) = (527, 506);

/// The logo's own disc and mark colours, which [`logo_path`] and
/// [`wordmark_path`] swap for a tint's.
const DISC: &str = "#008080";
const MARK: &str = "#f9f9f9";

/// The disc and mark colours to draw the logo in, as 0xRRGGBBAA; `None`
/// keeps Katna's teal and white.
pub type Tint = Option<(u32, u32)>;

/// Where [`Assets`] serves the logo for a `size` px square (GPUI pixels).
pub fn logo_path(size: f32, tint: Tint) -> String {
    let size = size.round().clamp(1.0, 1024.0) as u32;
    let form = if size < 48 { "small" } else { "full" };
    format!("logo/{form}-{size}{}.svg", tint_suffix(tint))
}

/// Where [`Assets`] serves the wordmark `height` px tall (GPUI pixels).
pub fn wordmark_path(height: f32, tint: Tint) -> String {
    let height = height.round().clamp(1.0, 1024.0) as u32;
    format!("logo/wordmark-{height}{}.svg", tint_suffix(tint))
}

fn tint_suffix(tint: Tint) -> String {
    tint.map(|(disc, mark)| format!("-{:06x}-{:06x}", disc >> 8, mark >> 8))
        .unwrap_or_default()
}

/// The logo for `logo/{full,small}-<size>.svg` and the wordmark for
/// `logo/wordmark-<height>.svg`, each with an optional `-<disc>-<mark>`
/// tint in RRGGBB before `.svg`. GPUI draws an SVG picture at twice its
/// own size, so giving it the size it is shown at renders it sharp on
/// double-density screens without sampling it down much.
fn logo(path: &str) -> Option<Vec<u8>> {
    let rest = path.strip_prefix("logo/")?.strip_suffix(".svg")?;
    let mut parts = rest.split('-');
    let (form, size) = (parts.next()?, parts.next()?);
    let size: u32 = size.parse().ok().filter(|s| (1..=1024).contains(s))?;
    let tint = match (parts.next(), parts.next(), parts.next()) {
        (None, None, None) => None,
        (Some(disc), Some(mark), None) if is_rgb(disc) && is_rgb(mark) => Some((disc, mark)),
        _ => return None,
    };
    let (data, from, to) = match form {
        "full" => (LOGO, (128, 128), (size, size)),
        "small" => (LOGO_SMALL, (128, 128), (size, size)),
        "wordmark" => {
            let (w, h) = WORDMARK_SIZE;
            (WORDMARK, (w, h), ((size * w).div_ceil(h), size))
        }
        _ => return None,
    };
    let mut text = String::from_utf8_lossy(data).into_owned();
    if let Some((disc, mark)) = tint {
        text = text
            .replace(DISC, &format!("#{disc}"))
            .replace(MARK, &format!("#{mark}"));
    }
    Some(
        text.replacen(
            &format!(r#"width="{}" height="{}""#, from.0, from.1),
            &format!(r#"width="{}" height="{}""#, to.0, to.1),
            1,
        )
        .into_bytes(),
    )
}

fn is_rgb(part: &str) -> bool {
    part.len() == 6 && part.bytes().all(|b| b.is_ascii_hexdigit())
}

/// A built-in icon's SVG by name (`brand-linkedin`).
pub fn icon_svg(name: &str) -> Option<&'static [u8]> {
    let path = format!("icons/{name}.svg");
    ICONS
        .iter()
        .find(|(n, _)| *n == path)
        .map(|(_, data)| *data)
}

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if path.starts_with("flags/") {
            return Ok(FLAGS
                .iter()
                .find(|(name, _)| *name == path)
                .map(|(_, data)| Cow::Owned(flag(data))));
        }
        if path.starts_with("logo/") {
            return Ok(logo(path).map(Cow::Owned));
        }
        Ok(ICONS
            .iter()
            .find(|(name, _)| *name == path)
            .map(|(_, data)| Cow::Borrowed(*data)))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(ICONS
            .iter()
            .filter(|(name, _)| name.starts_with(path))
            .map(|(name, _)| SharedString::from(*name))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_icon_file_is_built_in() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/icons");
        let mut files: Vec<String> = std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        files.sort();
        let built_in: Vec<String> = ICONS
            .iter()
            .map(|(name, _)| name.trim_start_matches("icons/").to_owned())
            .collect();
        assert_eq!(files, built_in);
        assert!(Assets.load("icons/star.svg").unwrap().is_some());
        assert!(Assets.load("icons/nope.svg").unwrap().is_none());
    }

    #[test]
    fn every_language_has_its_flag() {
        for language in katna_i18n::picker() {
            let path = format!("flags/{}.svg", language.flag);
            let data = Assets.load(&path).unwrap();
            assert!(data.is_some(), "{path}");
            assert!(String::from_utf8_lossy(&data.unwrap()).contains(r#"width="48""#));
        }
    }

    #[test]
    fn logo_is_served_at_its_size_in_the_form_for_it() {
        assert_eq!(logo_path(32.0, None), "logo/small-32.svg");
        assert_eq!(logo_path(64.0, None), "logo/full-64.svg");
        let small = Assets.load(&logo_path(40.0, None)).unwrap().unwrap();
        let small = String::from_utf8_lossy(&small);
        assert!(small.contains(r#"width="40" height="40""#));
        assert!(!small.contains("<filter"));
        let full = Assets.load("logo/full-96.svg").unwrap().unwrap();
        let full = String::from_utf8_lossy(&full);
        assert!(full.contains(r#"width="96" height="96""#) && full.contains("<filter"));
        let wordmark = Assets.load(&wordmark_path(120.0, None)).unwrap().unwrap();
        assert!(String::from_utf8_lossy(&wordmark).contains(r#"width="125" height="120""#));
        for bad in [
            "logo/full-0.svg",
            "logo/big-64.svg",
            "logo/full.svg",
            "logo/full-64-123456.svg",
            "logo/full-64-12345g-ffffff.svg",
            "logo/full-64-123456-ffffff-000000.svg",
        ] {
            assert!(Assets.load(bad).unwrap().is_none(), "{bad}");
        }
    }

    #[test]
    fn tinted_logo_swaps_disc_and_mark_colours() {
        let tint = Some((0x0b57d0ff, 0xffffffff));
        assert_eq!(logo_path(32.0, tint), "logo/small-32-0b57d0-ffffff.svg");
        for path in [
            logo_path(32.0, tint),
            logo_path(96.0, tint),
            wordmark_path(96.0, tint),
        ] {
            let svg = Assets.load(&path).unwrap().unwrap();
            let svg = String::from_utf8_lossy(&svg);
            assert!(
                svg.contains(r##"fill="#0b57d0""##) && svg.contains(r##"fill="#ffffff""##),
                "{path}"
            );
            assert!(!svg.contains(DISC) && !svg.contains(MARK), "{path}");
        }
    }

    #[test]
    fn installed_icon_has_no_filters_for_qt() {
        // KDE draws the scalable icon with Qt SVG, which skips blur filters.
        let icon = include_str!("../../../packaging/icons/in.invenia.katna.Mail.svg");
        assert!(!icon.contains("filter"));
        assert!(!include_str!("../../../packaging/icons/src/katna-small.svg").contains("filter"));
    }
}
