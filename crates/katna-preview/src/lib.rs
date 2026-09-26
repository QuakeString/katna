// SPDX-License-Identifier: GPL-3.0-or-later

//! Previews of attachments for the viewer in Katna Mail: the pages of a
//! PDF ([`pdf`]), pictures ([`picture`]) and text files ([`text`]). All of
//! it is pure Rust and runs off the UI thread; the app only draws the
//! bitmaps it gets back. See `docs/ARCHITECTURE.md` §13.8.
//!
//! Bitmaps are [`image::RgbaImage`]s with straight (not premultiplied)
//! alpha, which is what GPUI expects after swapping red and blue.

pub mod pdf;
pub mod picture;
pub mod text;

pub use image;

/// What the viewer can show of a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Pdf,
    Picture(Picture),
    Text,
    /// No preview: the viewer offers to save or open it elsewhere.
    Other,
}

/// A picture format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Picture {
    Png,
    Jpeg,
    Gif,
    Webp,
    Bmp,
    Tiff,
    Svg,
}

impl Picture {
    /// Whether [`picture::decode`] reads it. SVG and animated GIFs are
    /// drawn by the UI toolkit itself.
    pub fn decodable(self) -> bool {
        !matches!(self, Picture::Svg)
    }
}

/// What a file is, from its MIME type and, for the vague
/// `application/octet-stream` many senders use, its name.
pub fn kind(mime: &str, name: &str) -> Kind {
    let mime = mime.trim().to_ascii_lowercase();
    let mime = mime.split(';').next().unwrap_or_default().trim();
    let by_mime = match mime {
        "application/pdf" | "application/x-pdf" => Some(Kind::Pdf),
        "image/png" | "image/apng" => Some(Kind::Picture(Picture::Png)),
        "image/jpeg" | "image/jpg" | "image/pjpeg" => Some(Kind::Picture(Picture::Jpeg)),
        "image/gif" => Some(Kind::Picture(Picture::Gif)),
        "image/webp" => Some(Kind::Picture(Picture::Webp)),
        "image/bmp" | "image/x-bmp" | "image/x-ms-bmp" => Some(Kind::Picture(Picture::Bmp)),
        "image/tiff" => Some(Kind::Picture(Picture::Tiff)),
        "image/svg+xml" => Some(Kind::Picture(Picture::Svg)),
        "application/json"
        | "application/xml"
        | "application/x-sh"
        | "application/x-yaml"
        | "application/yaml"
        | "application/toml"
        | "application/x-shellscript" => Some(Kind::Text),
        // Calendar invitations and contact cards get their own views later.
        "text/calendar" | "text/vcard" | "text/x-vcard" => Some(Kind::Other),
        _ if mime.starts_with("text/") => Some(Kind::Text),
        _ => None,
    };
    by_mime.unwrap_or_else(|| by_name(name))
}

/// Whether opening the file in another app could run a program: the
/// viewer then offers only to save it.
pub fn risky(mime: &str, name: &str) -> bool {
    const TYPES: [&str; 8] = [
        "application/x-desktop",
        "application/x-executable",
        "application/x-sharedlib",
        "application/x-msdownload",
        "application/x-sh",
        "application/x-shellscript",
        "application/java-archive",
        "application/vnd.flatpak.ref",
    ];
    const EXTENSIONS: [&str; 20] = [
        "desktop",
        "sh",
        "bash",
        "zsh",
        "csh",
        "run",
        "bin",
        "appimage",
        "exe",
        "msi",
        "bat",
        "cmd",
        "com",
        "scr",
        "jar",
        "py",
        "pl",
        "flatpakref",
        "flatpakrepo",
        "ps1",
    ];
    let mime = mime.trim().to_ascii_lowercase();
    if TYPES.iter().any(|t| mime.starts_with(t)) {
        return true;
    }
    name.rsplit_once('.')
        .is_some_and(|(_, ext)| EXTENSIONS.iter().any(|e| ext.eq_ignore_ascii_case(e)))
}

fn by_name(name: &str) -> Kind {
    let Some((_, ext)) = name.rsplit_once('.') else {
        return Kind::Other;
    };
    match ext.to_ascii_lowercase().as_str() {
        "pdf" => Kind::Pdf,
        "png" => Kind::Picture(Picture::Png),
        "jpg" | "jpeg" | "jpe" | "jfif" => Kind::Picture(Picture::Jpeg),
        "gif" => Kind::Picture(Picture::Gif),
        "webp" => Kind::Picture(Picture::Webp),
        "bmp" => Kind::Picture(Picture::Bmp),
        "tif" | "tiff" => Kind::Picture(Picture::Tiff),
        "svg" => Kind::Picture(Picture::Svg),
        "txt" | "text" | "log" | "csv" | "tsv" | "md" | "markdown" | "json" | "xml" | "yaml"
        | "yml" | "toml" | "ini" | "conf" | "cfg" | "sh" | "py" | "rs" | "c" | "h" | "cpp"
        | "hpp" | "js" | "ts" | "css" | "sql" | "diff" | "patch" => Kind::Text,
        _ => Kind::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kinds_from_mime_and_name() {
        assert_eq!(kind("application/pdf", "x"), Kind::Pdf);
        assert_eq!(kind("Application/PDF; name=a.pdf", "x"), Kind::Pdf);
        assert_eq!(kind("image/jpeg", "a"), Kind::Picture(Picture::Jpeg));
        assert_eq!(kind("text/csv", "a"), Kind::Text);
        assert_eq!(kind("text/calendar", "invite.ics"), Kind::Other);
        assert_eq!(kind("application/octet-stream", "Scan.PDF"), Kind::Pdf);
        assert_eq!(
            kind("application/octet-stream", "photo.JPG"),
            Kind::Picture(Picture::Jpeg)
        );
        assert_eq!(kind("application/octet-stream", "notes.txt"), Kind::Text);
        assert_eq!(kind("application/zip", "a.zip"), Kind::Other);
        assert_eq!(kind("", "no-extension"), Kind::Other);
    }

    #[test]
    fn programs_are_risky() {
        assert!(risky("application/octet-stream", "invoice.desktop"));
        assert!(risky("text/plain", "setup.SH"));
        assert!(risky("application/x-desktop", "x"));
        assert!(!risky("application/pdf", "invoice.pdf"));
        assert!(!risky("image/png", "desktop"));
    }
}
