// SPDX-License-Identifier: GPL-3.0-or-later

//! Signatures made from one of Katna's twelve layouts: the fields filled
//! in once, written as mail-safe HTML (tables and inline styles, which
//! Outlook's Word engine also draws) with a plain text twin. Pictures
//! travel inside the mail as `data:` URIs, which sending turns into
//! `cid:` parts. Numbers are labelled "M:" and "O:", which Katna's own
//! person card reads. No GPUI here.

use std::cell::RefCell;
use std::collections::HashMap;

use katna_core::config::{LayoutStyle, SignatureLayout};
use katna_i18n::tr;
use katna_render::signature::Site;
use katna_ui::rich::html::{base64_decode, base64_encode};

/// The colours offered, as in the study; the first is the default.
pub const COLOURS: [u32; 6] = [0x0e7c86, 0x1a56db, 0x7c3aed, 0xc2410c, 0x15803d, 0x334155];

/// Text, secondary text and lines, fixed: a dark reader remaps them as it
/// does any mail.
const INK: &str = "#202124";
const DIM: &str = "#5f6368";
const LINE: &str = "#e0e3e7";
/// Page marks are shown this many pixels square, drawn at twice that.
const MARK: u32 = 18;
/// Photos and monograms are shown this many pixels across.
const PHOTO: u32 = 68;

/// What a layout is called.
pub fn name(style: LayoutStyle) -> String {
    match style {
        LayoutStyle::Classic => tr!("signature-layout-classic"),
        LayoutStyle::LogoLeft => tr!("signature-layout-logo-left"),
        LayoutStyle::Photo => tr!("signature-layout-photo"),
        LayoutStyle::Band => tr!("signature-layout-band"),
        LayoutStyle::OneLine => tr!("signature-layout-one-line"),
        LayoutStyle::Centred => tr!("signature-layout-centred"),
        LayoutStyle::Banner => tr!("signature-layout-banner"),
        LayoutStyle::Underline => tr!("signature-layout-underline"),
        LayoutStyle::SideBar => tr!("signature-layout-side-bar"),
        LayoutStyle::Card => tr!("signature-layout-card"),
        LayoutStyle::Monogram => tr!("signature-layout-monogram"),
        LayoutStyle::Plain => tr!("signature-layout-plain"),
    }
}

/// Which pictures a layout shows.
pub fn shows(style: LayoutStyle) -> (bool, bool) {
    use LayoutStyle::*;
    let logo = matches!(style, LogoLeft | Centred | Card);
    let photo = matches!(style, Photo);
    (logo, photo)
}

/// The layout's colour as `0xrrggbb`.
pub fn colour(layout: &SignatureLayout) -> u32 {
    let hex = layout.colour.trim().trim_start_matches('#');
    match u32::from_str_radix(hex, 16) {
        Ok(rgb) if hex.len() == 6 => rgb,
        _ => COLOURS[0],
    }
}

/// `0xrrggbb` as `#rrggbb`.
pub fn hex(rgb: u32) -> String {
    format!("#{rgb:06x}")
}

/// The signature as `(text, html)`; the HTML is empty for the plain text
/// layout.
pub fn write(layout: &SignatureLayout) -> (String, String) {
    let html = if layout.style == LayoutStyle::Plain {
        String::new()
    } else {
        html(layout)
    };
    (text(layout), html)
}

/// The plain text twin.
pub fn text(l: &SignatureLayout) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut push = |line: String| {
        if !line.trim().is_empty() {
            lines.push(line);
        }
    };
    push(l.name.trim().to_owned());
    push(joined(&[&l.title, &l.company], ", "));
    if l.style == LayoutStyle::OneLine {
        push(joined(&[&l.mobile, &l.website], " · "));
        return lines.join("\n");
    }
    for (label, value) in labelled(l) {
        push(format!("{label} {value}"));
    }
    push(l.website.trim().to_owned());
    for page in pages(l) {
        push(page);
    }
    push(l.address.trim().to_owned());
    lines.join("\n")
}

/// The numbers and address with their labels: "M: +91 …".
fn labelled(l: &SignatureLayout) -> Vec<(String, String)> {
    [
        (tr!("signature-layout-mobile-label"), &l.mobile),
        (tr!("signature-layout-office-label"), &l.office),
        (tr!("signature-layout-email-label"), &l.email),
    ]
    .into_iter()
    .filter(|(_, v)| !v.trim().is_empty())
    .map(|(k, v)| (k, v.trim().to_owned()))
    .collect()
}

fn pages(l: &SignatureLayout) -> Vec<String> {
    l.pages
        .iter()
        .map(|p| p.trim().to_owned())
        .filter(|p| !p.is_empty())
        .collect()
}

/// The non-empty `parts`, trimmed, joined by `with`.
fn joined(parts: &[&String], with: &str) -> String {
    parts
        .iter()
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join(with)
}

/// Up to two initials of `name`.
pub fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|w| w.chars().find(|c| c.is_alphanumeric()))
        .take(2)
        .flat_map(char::to_uppercase)
        .collect()
}

/// The HTML.
fn html(l: &SignatureLayout) -> String {
    let c = hex(colour(l));
    let w = Writer { l, c: &c };
    let body = match l.style {
        LayoutStyle::Classic => w.classic(),
        LayoutStyle::LogoLeft => w.logo_left(),
        LayoutStyle::Photo => w.photo(),
        LayoutStyle::Band => w.band(),
        LayoutStyle::OneLine => w.one_line(),
        LayoutStyle::Centred => w.centred(),
        LayoutStyle::Banner => w.banner_layout(),
        LayoutStyle::Underline => w.underline(),
        LayoutStyle::SideBar => w.side_bar(),
        LayoutStyle::Card => w.card(),
        LayoutStyle::Monogram => w.monogram(),
        LayoutStyle::Plain => String::new(),
    };
    // A banner goes under any layout; the banner layout places its own.
    let banner = if l.style == LayoutStyle::Banner || l.style == LayoutStyle::OneLine {
        String::new()
    } else {
        w.banner()
            .map(|b| format!(r#"<tr><td style="padding-top:10px">{b}</td></tr>"#))
            .unwrap_or_default()
    };
    format!(
        r#"<table cellpadding="0" cellspacing="0" border="0" style="border-collapse:collapse;font-family:Arial,Helvetica,sans-serif"><tr><td>{body}</td></tr>{banner}</table>"#
    )
}

struct Writer<'a> {
    l: &'a SignatureLayout,
    /// The colour, `#rrggbb`.
    c: &'a str,
}

impl Writer<'_> {
    fn name(&self, size: f32) -> String {
        let name = esc(&self.l.name);
        if name.is_empty() {
            return String::new();
        }
        format!(r#"<div style="font-size:{size}px;font-weight:bold;color:{INK}">{name}</div>"#)
    }

    /// "Title · Company" in grey.
    fn role(&self, gap_below: u32) -> String {
        let role = esc(&joined(&[&self.l.title, &self.l.company], " · "));
        if role.is_empty() {
            return String::new();
        }
        format!(
            r#"<div style="font-size:12.5px;color:{DIM};margin-bottom:{gap_below}px">{role}</div>"#
        )
    }

    /// Labelled numbers and address, one a line.
    fn lines(&self, which: &[Field]) -> String {
        which
            .iter()
            .filter_map(|f| self.line(*f))
            .collect::<Vec<_>>()
            .join("")
    }

    fn line(&self, field: Field) -> Option<String> {
        let c = self.c;
        let (label, value) = match field {
            Field::Mobile => (tr!("signature-layout-mobile-label"), &self.l.mobile),
            Field::Office => (tr!("signature-layout-office-label"), &self.l.office),
            Field::Email => (tr!("signature-layout-email-label"), &self.l.email),
            Field::Website => return self.website(),
            Field::Address => {
                let address = esc(&self.l.address);
                return (!address.is_empty()).then(|| {
                    format!(
                        r#"<div style="font-size:12px;line-height:18px;color:{DIM}">{address}</div>"#
                    )
                });
            }
        };
        let value = value.trim();
        if value.is_empty() {
            return None;
        }
        let shown = if field == Field::Email {
            self.mail_link(value)
        } else {
            esc(value)
        };
        Some(format!(
            r#"<div style="font-size:12.5px;line-height:19px;color:{INK}"><span style="color:{c};font-weight:bold">{label}</span>&nbsp;{shown}</div>"#,
            label = esc(&label)
        ))
    }

    fn website(&self) -> Option<String> {
        let site = self.l.website.trim();
        (!site.is_empty()).then(|| {
            format!(
                r#"<div style="font-size:12.5px;line-height:19px">{}</div>"#,
                self.link(site)
            )
        })
    }

    fn link(&self, site: &str) -> String {
        let c = self.c;
        format!(
            r#"<a href="{href}" style="color:{c};text-decoration:none">{shown}</a>"#,
            href = esc(&web_address(site)),
            shown = esc(site
                .trim_start_matches("https://")
                .trim_start_matches("http://"))
        )
    }

    fn mail_link(&self, address: &str) -> String {
        let c = self.c;
        format!(
            r#"<a href="mailto:{a}" style="color:{c};text-decoration:none">{a}</a>"#,
            a = esc(address)
        )
    }

    /// The page marks in a row, `gap` pixels apart; empty for none.
    fn marks(&self, gap: u32, centred: bool) -> String {
        let cells: Vec<String> = pages(self.l)
            .iter()
            .filter_map(|page| {
                let png = mark(Site::of(page), colour(self.l))?;
                let site = match Site::of(page) {
                    Site::Web => katna_render::signature::host(page),
                    site => site.name().to_owned(),
                };
                Some(format!(
                    r#"<a href="{href}"><img alt="{alt}" width="{MARK}" height="{MARK}" style="display:block;border:0" src="data:image/png;base64,{png}"></a>"#,
                    href = esc(&web_address(page)),
                    alt = esc(&site),
                    png = base64_encode(&png)
                ))
            })
            .collect();
        if cells.is_empty() {
            return String::new();
        }
        let n = cells.len();
        let cells: String = cells
            .into_iter()
            .enumerate()
            .map(|(i, cell)| {
                let pad = if centred {
                    format!("padding:0 {}px", gap / 2)
                } else if i + 1 < n {
                    format!("padding-right:{gap}px")
                } else {
                    String::new()
                };
                format!(r#"<td style="{pad}">{cell}</td>"#)
            })
            .collect();
        let margin = if centred { ";margin:0 auto" } else { "" };
        format!(
            r#"<table cellpadding="0" cellspacing="0" border="0" style="border-collapse:collapse;margin-top:6px{margin}"><tr>{cells}</tr></table>"#
        )
    }

    /// The logo fitted in `(width, height)`.
    fn logo(&self, (width, height): (u32, u32)) -> Option<String> {
        let (w, h) = shown_size(&self.l.logo, (width, height))?;
        Some(format!(
            r#"<img alt="{alt}" width="{w}" height="{h}" style="display:block;border:0" src="{src}">"#,
            alt = esc(&self.l.company),
            src = self.l.logo
        ))
    }

    /// The photo, or the initials in a circle of the colour.
    fn photo_or_initials(&self) -> String {
        if !self.l.photo.is_empty() {
            return format!(
                r#"<img alt="{alt}" width="{PHOTO}" height="{PHOTO}" style="display:block;border:0;border-radius:50%" src="{src}">"#,
                alt = esc(&self.l.name),
                src = self.l.photo
            );
        }
        self.initials(PHOTO)
    }

    /// The initials in a circle of the colour, as a picture: round in
    /// every reader. Empty without a name.
    fn initials(&self, side: u32) -> String {
        let ini = initials(&self.l.name);
        let Some(png) = drawn(Drawn::Monogram(ini.clone(), colour(self.l), side * 2)) else {
            return String::new();
        };
        format!(
            r#"<img alt="{alt}" width="{side}" height="{side}" style="display:block;border:0" src="data:image/png;base64,{png}">"#,
            alt = esc(&ini),
            png = base64_encode(&png)
        )
    }

    fn banner(&self) -> Option<String> {
        let (w, h) = shown_size(&self.l.banner, (480, 200))?;
        Some(format!(
            r#"<img alt="" width="{w}" height="{h}" style="display:block;border:0;border-radius:8px" src="{src}">"#,
            src = self.l.banner
        ))
    }

    fn classic(&self) -> String {
        use Field::*;
        format!(
            "{}{}{}{}",
            self.name(15.0),
            self.role(6),
            self.lines(&[Mobile, Office, Email, Website, Address]),
            self.marks(6, false)
        )
    }

    fn logo_left(&self) -> String {
        use Field::*;
        let c = self.c;
        let details = format!(
            "{}{}{}{}",
            self.name(15.0),
            self.role(6),
            self.lines(&[Mobile, Office, Email, Website, Address]),
            self.marks(6, false)
        );
        match self.logo((120, 64)) {
            Some(logo) => format!(
                r#"<table cellpadding="0" cellspacing="0" border="0" style="border-collapse:collapse"><tr><td style="padding-right:14px;vertical-align:middle">{logo}</td><td style="border-left:2px solid {c};padding-left:14px;vertical-align:top">{details}</td></tr></table>"#
            ),
            None => format!(
                r#"<table cellpadding="0" cellspacing="0" border="0" style="border-collapse:collapse"><tr><td style="border-left:2px solid {c};padding-left:14px;vertical-align:top">{details}</td></tr></table>"#
            ),
        }
    }

    fn photo(&self) -> String {
        use Field::*;
        let c = self.c;
        let title = esc(&self.l.title);
        let title = if title.is_empty() {
            String::new()
        } else {
            format!(
                r#"<div style="font-size:12.5px;color:{c};font-weight:bold;margin-bottom:5px">{title}</div>"#
            )
        };
        let company = esc(&self.l.company);
        let company = if company.is_empty() {
            String::new()
        } else {
            format!(r#"<div style="font-size:12.5px;line-height:19px;color:{DIM}">{company}</div>"#)
        };
        format!(
            r#"<table cellpadding="0" cellspacing="0" border="0" style="border-collapse:collapse"><tr><td style="padding-right:14px;vertical-align:middle">{photo}</td><td style="vertical-align:middle">{name}{title}{company}{lines}{marks}</td></tr></table>"#,
            photo = self.photo_or_initials(),
            name = self.name(15.0),
            lines = self.lines(&[Mobile, Office, Email, Website]),
            marks = self.marks(6, false)
        )
    }

    fn band(&self) -> String {
        use Field::*;
        let c = self.c;
        let title = esc(&self.l.title);
        let title = if title.is_empty() {
            String::new()
        } else {
            format!(r#"<span style="font-size:12px;color:#ffffff">&nbsp;&nbsp;{title}</span>"#)
        };
        let company = esc(&self.l.company);
        let company = if company.is_empty() {
            String::new()
        } else {
            format!(r#"<div style="font-size:12px;color:{DIM};margin-top:4px">{company}</div>"#)
        };
        format!(
            r#"<table cellpadding="0" cellspacing="0" border="0" style="border-collapse:separate;min-width:300px"><tr><td style="background:{c};padding:8px 12px;border-radius:6px 6px 0 0"><span style="font-size:14.5px;font-weight:bold;color:#ffffff">{name}</span>{title}</td></tr><tr><td style="padding:8px 12px;border:1px solid {LINE};border-top:0;border-radius:0 0 6px 6px"><table cellpadding="0" cellspacing="0" border="0" style="border-collapse:collapse"><tr><td style="padding-right:18px;vertical-align:top">{left}</td><td style="vertical-align:top">{right}</td></tr></table>{company}{address}{marks}</td></tr></table>"#,
            name = esc(&self.l.name),
            left = self.lines(&[Mobile, Office]),
            right = self.lines(&[Email, Website]),
            address = self.lines(&[Address]),
            marks = self.marks(6, false)
        )
    }

    fn one_line(&self) -> String {
        let name = esc(&self.l.name);
        let role = esc(&joined(&[&self.l.title, &self.l.company], ", "));
        let mut first = String::new();
        if !name.is_empty() {
            first.push_str(&format!("<b>{name}</b>"));
        }
        if !role.is_empty() {
            if !first.is_empty() {
                first.push(' ');
            }
            first.push_str(&format!(r#"<span style="color:{DIM}">· {role}</span>"#));
        }
        let mut second = Vec::new();
        if !self.l.mobile.trim().is_empty() {
            second.push(format!(
                r#"<span style="color:{DIM}">{}</span>"#,
                esc(&self.l.mobile)
            ));
        }
        if !self.l.website.trim().is_empty() {
            second.push(self.link(self.l.website.trim()));
        }
        let second = second.join(&format!(r#"<span style="color:{DIM}"> · </span>"#));
        let br = if first.is_empty() || second.is_empty() {
            ""
        } else {
            "<br>"
        };
        format!(
            r#"<div style="font-size:13px;line-height:20px;color:{INK}">{first}{br}{second}</div>"#
        )
    }

    fn centred(&self) -> String {
        let logo = self
            .logo((96, 40))
            .map(|logo| {
                format!(
                    r#"<table cellpadding="0" cellspacing="0" border="0" style="border-collapse:collapse;margin:0 auto 6px"><tr><td>{logo}</td></tr></table>"#
                )
            })
            .unwrap_or_default();
        let mut contact = Vec::new();
        if !self.l.mobile.trim().is_empty() {
            contact.push(esc(&self.l.mobile));
        }
        if !self.l.website.trim().is_empty() {
            contact.push(self.link(self.l.website.trim()));
        }
        let contact = if contact.is_empty() {
            String::new()
        } else {
            format!(
                r#"<div style="font-size:12.5px;color:{INK};margin-top:4px">{}</div>"#,
                contact.join(" · ")
            )
        };
        format!(
            r#"<table cellpadding="0" cellspacing="0" border="0" style="border-collapse:collapse"><tr><td align="center" style="text-align:center">{logo}{name}{role}{contact}{marks}</td></tr></table>"#,
            name = self.name(15.0),
            role = self.role(0),
            marks = self.marks(6, true)
        )
    }

    fn banner_layout(&self) -> String {
        let role = esc(&joined(
            &[&self.l.title, &self.l.company, &self.l.mobile],
            " · ",
        ));
        let role = if role.is_empty() {
            String::new()
        } else {
            format!(r#"<div style="font-size:12.5px;color:{DIM}">{role}</div>"#)
        };
        let banner = self
            .banner()
            .map(|b| format!(r#"<div style="padding-top:10px">{b}</div>"#))
            .unwrap_or_default();
        format!("{}{role}{}{banner}", self.name(15.0), self.marks(6, false))
    }

    fn underline(&self) -> String {
        let role = esc(&joined(&[&self.l.title, &self.l.company], ", "));
        let role = if role.is_empty() {
            String::new()
        } else {
            format!(r#"<div style="font-size:12.5px;color:{DIM}">{role}</div>"#)
        };
        let mut contact = Vec::new();
        if !self.l.mobile.trim().is_empty() {
            contact.push(esc(&self.l.mobile));
        }
        if !self.l.email.trim().is_empty() {
            contact.push(self.mail_link(self.l.email.trim()));
        }
        let contact = if contact.is_empty() {
            String::new()
        } else {
            format!(
                r#"<div style="font-size:12.5px;line-height:19px;color:{INK}">{}</div>"#,
                contact.join(" · ")
            )
        };
        format!(
            r#"{name}<div style="padding:5px 0 7px">{bar}</div>{role}{contact}{web}{marks}"#,
            name = self.name(18.0),
            bar = drawn(Drawn::Bar(colour(self.l), 72, 6))
                .map(|png| format!(
                    r#"<img alt="" width="36" height="3" style="display:block;border:0" src="data:image/png;base64,{}">"#,
                    base64_encode(&png)
                ))
                .unwrap_or_default(),
            web = self.lines(&[Field::Website]),
            marks = self.marks(6, false)
        )
    }

    fn side_bar(&self) -> String {
        use Field::*;
        let c = self.c;
        let title = esc(&self.l.title);
        let title = if title.is_empty() {
            String::new()
        } else {
            format!(r#"<div style="font-size:12.5px;color:{c};font-weight:bold">{title}</div>"#)
        };
        let company = esc(&self.l.company);
        let company = if company.is_empty() {
            String::new()
        } else {
            format!(
                r#"<div style="font-size:12.5px;color:{DIM};margin-bottom:5px">{company}</div>"#
            )
        };
        format!(
            r#"<table cellpadding="0" cellspacing="0" border="0" style="border-collapse:collapse"><tr><td style="border-left:4px solid {c};padding-left:12px">{name}{title}{company}{lines}{marks}</td></tr></table>"#,
            name = self.name(15.0),
            lines = self.lines(&[Mobile, Office, Email, Website, Address]),
            marks = self.marks(6, false)
        )
    }

    fn card(&self) -> String {
        use Field::*;
        let c = self.c;
        let company = esc(&self.l.company);
        let head = match (self.logo((90, 30)), company.is_empty()) {
            (None, true) => String::new(),
            (logo, _) => format!(
                r#"<table cellpadding="0" cellspacing="0" border="0" style="border-collapse:collapse;margin-bottom:8px"><tr>{logo}<td style="vertical-align:middle;font-size:12.5px;font-weight:bold;color:{c}">{company}</td></tr></table>"#,
                logo = logo
                    .map(|l| format!(
                        r#"<td style="padding-right:10px;vertical-align:middle">{l}</td>"#
                    ))
                    .unwrap_or_default()
            ),
        };
        let title = esc(&self.l.title);
        let title = if title.is_empty() {
            String::new()
        } else {
            format!(r#"<div style="font-size:12.5px;color:{DIM};margin-bottom:6px">{title}</div>"#)
        };
        format!(
            r#"<table cellpadding="0" cellspacing="0" border="0" style="border-collapse:separate;border:1px solid {LINE};border-radius:10px"><tr><td style="padding:12px 16px">{head}{name}{title}<table cellpadding="0" cellspacing="0" border="0" style="border-collapse:collapse"><tr><td style="padding-right:16px;vertical-align:top">{left}</td><td style="vertical-align:top">{right}</td></tr></table>{address}{marks}</td></tr></table>"#,
            name = self.name(15.0),
            left = self.lines(&[Mobile, Office]),
            right = self.lines(&[Email, Website]),
            address = self.lines(&[Address]),
            marks = self.marks(6, false)
        )
    }

    fn monogram(&self) -> String {
        let mut contact = Vec::new();
        if !self.l.mobile.trim().is_empty() {
            contact.push(esc(&self.l.mobile));
        }
        if !self.l.website.trim().is_empty() {
            contact.push(self.link(self.l.website.trim()));
        }
        let contact = if contact.is_empty() {
            String::new()
        } else {
            format!(
                r#"<div style="font-size:12.5px;line-height:19px;color:{INK}">{}</div>"#,
                contact.join(" · ")
            )
        };
        format!(
            r#"<table cellpadding="0" cellspacing="0" border="0" style="border-collapse:collapse"><tr><td style="padding-right:12px;vertical-align:middle">{ini}</td><td style="vertical-align:middle">{name}{role}{contact}{marks}</td></tr></table>"#,
            ini = self.initials(52),
            name = self.name(15.0),
            role = self.role(0),
            marks = self.marks(6, false)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Mobile,
    Office,
    Email,
    Website,
    Address,
}

/// `site` as a link target: `https://` added when it has no scheme.
fn web_address(site: &str) -> String {
    let site = site.trim();
    if site.contains("://") || site.starts_with("mailto:") {
        site.to_owned()
    } else {
        format!("https://{site}")
    }
}

/// `text` escaped for HTML, trimmed.
fn esc(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.trim().chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
    out
}

/// The size to show a picture made at twice its size (a `data:` URI) at,
/// fitted in `(width, height)`.
fn shown_size(uri: &str, (width, height): (u32, u32)) -> Option<(u32, u32)> {
    let (w, h) = picture_size(uri)?;
    let (w, h) = (w.div_ceil(2).max(1), h.div_ceil(2).max(1));
    let scale = (width as f32 / w as f32)
        .min(height as f32 / h as f32)
        .min(1.0);
    Some((
        ((w as f32 * scale).round() as u32).max(1),
        ((h as f32 * scale).round() as u32).max(1),
    ))
}

/// The pixel size of a picture in a `data:` URI.
pub fn picture_size(uri: &str) -> Option<(u32, u32)> {
    let bytes = picture_bytes(uri)?;
    katna_preview::signature::size(&bytes)
}

/// The bytes of a picture in a `data:` URI.
pub fn picture_bytes(uri: &str) -> Option<Vec<u8>> {
    let (_, data) = uri.strip_prefix("data:")?.split_once(";base64,")?;
    base64_decode(data)
}

/// A picture Katna draws for a layout, at twice its shown size.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Drawn {
    /// A page's mark in a colour.
    Mark(Site, u32),
    /// Initials on a circle of a colour, so many pixels across.
    Monogram(String, u32, u32),
    /// A bar of a colour, width × height.
    Bar(u32, u32, u32),
}

thread_local! {
    /// Pictures already drawn: the layout is written again on every key.
    static DRAWN: RefCell<HashMap<Drawn, Option<Vec<u8>>>> = RefCell::new(HashMap::new());
}

/// `what`, drawn as PNG.
fn drawn(what: Drawn) -> Option<Vec<u8>> {
    use katna_preview::signature as draw;
    DRAWN.with(|cache| {
        cache
            .borrow_mut()
            .entry(what.clone())
            .or_insert_with(|| match what {
                Drawn::Mark(site, rgb) => {
                    let svg = crate::assets::icon_svg(mark_icon(site))?;
                    draw::tinted_mark(svg, rgb, MARK * 2).ok()
                }
                Drawn::Monogram(initials, _, _) if initials.is_empty() => None,
                Drawn::Monogram(initials, rgb, side) => draw::monogram(&initials, rgb, side).ok(),
                Drawn::Bar(rgb, width, height) => draw::bar(rgb, width, height).ok(),
            })
            .clone()
    })
}

/// The mark of `site` in `rgb`, at twice its shown size, as PNG.
fn mark(site: Site, rgb: u32) -> Option<Vec<u8>> {
    drawn(Drawn::Mark(site, rgb))
}

/// How many bytes the pictures inside `html` (as `data:` URIs) take.
pub fn pictures_size(html: &str) -> usize {
    html.split("data:")
        .skip(1)
        .filter_map(|rest| {
            let (_, data) = rest.split_once(";base64,")?;
            let end = data.find(['"', '\'', ')', ' ']).unwrap_or(data.len());
            Some(data[..end].len() * 3 / 4)
        })
        .sum()
}

/// The icon for a page on `site`.
pub fn mark_icon(site: Site) -> &'static str {
    match site {
        Site::Web => "language",
        Site::LinkedIn => "brand-linkedin",
        Site::X => "brand-x",
        Site::Facebook => "brand-facebook",
        Site::Instagram => "brand-instagram",
        Site::YouTube => "brand-youtube",
        Site::GitHub => "brand-github",
        Site::WhatsApp => "brand-whatsapp",
        Site::Telegram => "brand-telegram",
    }
}

#[cfg(test)]
mod tests;
