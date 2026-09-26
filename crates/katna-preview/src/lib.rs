// SPDX-License-Identifier: GPL-3.0-or-later

//! Previews of attachments for the viewer in Katna Mail: the pages of a
//! PDF ([`pdf`]), pictures ([`picture`]), text files ([`text`]),
//! spreadsheets ([`sheet`]) and word processor documents ([`document`]). All of
//! it is pure Rust and runs off the UI thread; the app only draws the
//! bitmaps it gets back. See `docs/ARCHITECTURE.md` §13.8.
//!
//! Bitmaps are [`image::RgbaImage`]s with straight (not premultiplied)
//! alpha, which is what GPUI expects after swapping red and blue.

pub mod document;
pub mod pdf;
pub mod picture;
pub mod sheet;
pub mod text;

pub use image;

/// What the viewer can show of a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Pdf,
    Picture(Picture),
    Text,
    /// Excel, OpenDocument or CSV; `csv` when it is plain text.
    Sheet {
        csv: bool,
    },
    /// Word (docx) or OpenDocument text (odt).
    Document,
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
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
        | "application/vnd.ms-excel"
        | "application/vnd.ms-excel.sheet.macroenabled.12"
        | "application/vnd.ms-excel.sheet.binary.macroenabled.12"
        | "application/vnd.oasis.opendocument.spreadsheet" => Some(Kind::Sheet { csv: false }),
        "text/csv" | "text/tab-separated-values" | "application/csv" | "text/x-csv" => {
            Some(Kind::Sheet { csv: true })
        }
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        | "application/vnd.ms-word.document.macroenabled.12"
        | "application/vnd.oasis.opendocument.text" => Some(Kind::Document),
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
    let from_name = by_name(name);
    match by_mime {
        // Senders often label CSV files as plain text and office files as
        // zip files; the name says more.
        Some(Kind::Text)
            if mime == "text/plain" && matches!(from_name, Kind::Sheet { csv: true }) =>
        {
            from_name
        }
        None if matches!(from_name, Kind::Sheet { .. } | Kind::Document)
            || !matches!(
                mime,
                "application/zip" | "application/x-zip-compressed" | "application/x-zip"
            ) =>
        {
            from_name
        }
        Some(kind) => kind,
        None => Kind::Other,
    }
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
        "xlsx" | "xlsm" | "xlsb" | "xls" | "ods" => Kind::Sheet { csv: false },
        "csv" | "tsv" => Kind::Sheet { csv: true },
        "docx" | "docm" | "odt" => Kind::Document,
        "txt" | "text" | "log" | "md" | "markdown" | "json" | "xml" | "yaml" | "yml" | "toml"
        | "ini" | "conf" | "cfg" | "sh" | "py" | "rs" | "c" | "h" | "cpp" | "hpp" | "js" | "ts"
        | "css" | "sql" | "diff" | "patch" => Kind::Text,
        _ => Kind::Other,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn kinds_from_mime_and_name() {
        assert_eq!(kind("application/pdf", "x"), Kind::Pdf);
        assert_eq!(kind("Application/PDF; name=a.pdf", "x"), Kind::Pdf);
        assert_eq!(kind("image/jpeg", "a"), Kind::Picture(Picture::Jpeg));
        assert_eq!(kind("text/csv", "a"), Kind::Sheet { csv: true });
        assert_eq!(kind("text/plain", "a.txt"), Kind::Text);
        assert_eq!(
            kind("application/octet-stream", "Budget.XLSX"),
            Kind::Sheet { csv: false }
        );
        assert_eq!(
            kind("application/vnd.oasis.opendocument.text", "x"),
            Kind::Document
        );
        assert_eq!(kind("application/msword", "old.doc"), Kind::Other);
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
    fn names_beat_vague_types() {
        assert_eq!(kind("text/plain", "list.csv"), Kind::Sheet { csv: true });
        assert_eq!(kind("application/zip", "Report.docx"), Kind::Document);
        assert_eq!(kind("application/zip", "photos.zip"), Kind::Other);
        assert_eq!(kind("application/zip", "fake.pdf"), Kind::Other);
    }

    /// A zip file of `files`, stored uncompressed.
    pub(crate) fn zip(files: &[(&str, &str)]) -> Vec<u8> {
        use std::io::Write;
        let mut out = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        for (name, body) in files {
            out.start_file(*name, options).unwrap();
            out.write_all(body.as_bytes()).unwrap();
        }
        out.finish().unwrap().into_inner()
    }

    /// A workbook with a "Budget" sheet (a formula included) and "Notes".
    pub(crate) fn xlsx() -> Vec<u8> {
        let sheet = |rows: &str| {
            format!(
                r#"<?xml version="1.0" encoding="UTF-8"?><worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData>{rows}</sheetData></worksheet>"#
            )
        };
        let budget = sheet(
            r#"<row r="1"><c r="A1" t="inlineStr"><is><t>Item</t></is></c><c r="B1" t="inlineStr"><is><t>Cost</t></is></c></row><row r="2"><c r="A2" t="inlineStr"><is><t>Paper</t></is></c><c r="B2"><v>12.5</v></c></row><row r="3"><c r="A3" t="inlineStr"><is><t>Total</t></is></c><c r="B3"><f>SUM(B2)</f><v>12.5</v></c></row>"#,
        );
        let notes = sheet(r#"<row r="1"><c r="A1" t="inlineStr"><is><t>Hi</t></is></c></row>"#);
        zip(&[
            (
                "[Content_Types].xml",
                r#"<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/><Override PartName="/xl/worksheets/sheet2.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/></Types>"#,
            ),
            (
                "_rels/.rels",
                r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>"#,
            ),
            (
                "xl/workbook.xml",
                r#"<?xml version="1.0" encoding="UTF-8"?><workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Budget" sheetId="1" r:id="rId1"/><sheet name="Notes" sheetId="2" r:id="rId2"/></sheets></workbook>"#,
            ),
            (
                "xl/_rels/workbook.xml.rels",
                r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/><Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet2.xml"/></Relationships>"#,
            ),
            ("xl/worksheets/sheet1.xml", &budget),
            ("xl/worksheets/sheet2.xml", &notes),
        ])
    }

    /// A Word document: title, heading, mixed runs, a numbered list with a
    /// sub-item, a bullet and a 2 × 2 table.
    pub(crate) fn docx() -> Vec<u8> {
        const W: &str = r#"xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main""#;
        let p = |ppr: &str, runs: &str| format!("<w:p><w:pPr>{ppr}</w:pPr>{runs}</w:p>");
        let r = |rpr: &str, text: &str| {
            format!(r#"<w:r><w:rPr>{rpr}</w:rPr><w:t xml:space="preserve">{text}</w:t></w:r>"#)
        };
        let list = |num: u8, level: u8, text: &str| {
            p(
                &format!(r#"<w:numPr><w:ilvl w:val="{level}"/><w:numId w:val="{num}"/></w:numPr>"#),
                &r("", text),
            )
        };
        let cell = |text: &str| format!("<w:tc>{}</w:tc>", p("", &r("", text)));
        let body = [
            p(
                r#"<w:pStyle w:val="Title"/><w:jc w:val="center"/>"#,
                &r("", "Quarterly report"),
            ),
            p(r#"<w:pStyle w:val="Heading1"/>"#, &r("", "Summary")),
            p(
                "",
                &[
                    r("", "Plain "),
                    r("<w:b/>", "bold"),
                    r(r#"<w:b w:val="0"/>"#, " "),
                    r("<w:i/>", "italic &amp; more"),
                ]
                .concat(),
            ),
            list(1, 0, "One"),
            list(1, 1, "Sub"),
            list(1, 0, "Two"),
            list(2, 0, "Dot"),
            format!(
                "<w:tbl><w:tr>{}{}</w:tr><w:tr>{}{}</w:tr></w:tbl>",
                cell("1"),
                cell("2"),
                cell("3"),
                cell("4")
            ),
        ]
        .concat();
        let document =
            format!(r#"<?xml version="1.0"?><w:document {W}><w:body>{body}</w:body></w:document>"#);
        let styles = format!(
            r#"<?xml version="1.0"?><w:styles {W}><w:style w:type="paragraph" w:styleId="Title"><w:name w:val="Title"/></w:style><w:style w:type="paragraph" w:styleId="Heading1"><w:name w:val="heading 1"/><w:rPr><w:b/></w:rPr></w:style></w:styles>"#
        );
        let numbering = format!(
            r#"<?xml version="1.0"?><w:numbering {W}><w:abstractNum w:abstractNumId="0"><w:lvl w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="decimal"/><w:lvlText w:val="%1."/></w:lvl><w:lvl w:ilvl="1"><w:start w:val="1"/><w:numFmt w:val="lowerLetter"/><w:lvlText w:val="%2)"/></w:lvl></w:abstractNum><w:abstractNum w:abstractNumId="1"><w:lvl w:ilvl="0"><w:numFmt w:val="bullet"/><w:lvlText w:val="-"/></w:lvl></w:abstractNum><w:num w:numId="1"><w:abstractNumId w:val="0"/></w:num><w:num w:numId="2"><w:abstractNumId w:val="1"/></w:num></w:numbering>"#
        );
        zip(&[
            ("word/document.xml", &document),
            ("word/styles.xml", &styles),
            ("word/numbering.xml", &numbering),
        ])
    }

    /// An OpenDocument text: title, heading, a bold span with a spelled-out
    /// double space, a skipped footnote, a numbered list and a 2 × 2 table.
    pub(crate) fn odt() -> Vec<u8> {
        const NS: &str = r#"xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0" xmlns:style="urn:oasis:names:tc:opendocument:xmlns:style:1.0" xmlns:fo="urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0" xmlns:table="urn:oasis:names:tc:opendocument:xmlns:table:1.0""#;
        let content = format!(
            r#"<?xml version="1.0"?><office:document-content {NS}><office:automatic-styles><style:style style:name="T1" style:family="text"><style:text-properties fo:font-weight="bold"/></style:style><text:list-style style:name="L1"><text:list-level-style-number text:level="1" style:num-format="1" style:num-suffix="."/></text:list-style></office:automatic-styles><office:body><office:text><text:sequence-decls><text:sequence-decl text:name="Figure"/></text:sequence-decls><text:p text:style-name="Title">Minutes</text:p><text:h text:outline-level="1">Agenda</text:h><text:p>We met at <text:s/><text:span text:style-name="T1">noon</text:span> &amp; ate.<text:note><text:note-body><text:p>A footnote</text:p></text:note-body></text:note></text:p><text:list text:style-name="L1"><text:list-item><text:p>First</text:p></text:list-item><text:list-item><text:p>Second</text:p></text:list-item></text:list><table:table><table:table-row><table:table-cell><text:p>a</text:p></table:table-cell><table:table-cell><text:p>b</text:p></table:table-cell></table:table-row><table:table-row><table:table-cell><text:p>c</text:p></table:table-cell><table:table-cell table:number-columns-repeated="1"><text:p>d</text:p></table:table-cell></table:table-row></table:table></office:text></office:body></office:document-content>"#
        );
        let styles = format!(
            r#"<?xml version="1.0"?><office:document-styles {NS}><office:styles><style:style style:name="Title" style:display-name="Title" style:family="paragraph"/></office:styles></office:document-styles>"#
        );
        zip(&[
            ("mimetype", "application/vnd.oasis.opendocument.text"),
            ("content.xml", &content),
            ("styles.xml", &styles),
        ])
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
