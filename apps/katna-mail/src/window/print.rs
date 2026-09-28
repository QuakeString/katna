// SPDX-License-Identifier: GPL-3.0-or-later

//! Printing the open conversation. Print lays it out as a PDF
//! ([`katna_render::print`]) and shows the pages in Katna's print preview
//! (`print_preview`). Its Print button hands the PDF to the desktop's print
//! dialog, which asks for the printer and prints it; when the dialog picks
//! other paper, the conversation is laid out again on that paper first.
//! Without a print dialog (no portal), the PDF opens in the default app to
//! print from there.
//!
//! The PDF is written to `$XDG_RUNTIME_DIR/katna/print`, which is private
//! and emptied at logout; copies older than an hour are removed.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use gpui::{Context, Window};
use katna_i18n::tr;
use katna_render::Address;
use katna_render::print::{
    Paper, PrintFont, PrintFonts, PrintMessage, PrintOptions, conversation_pdf,
};

use super::MailWindow;
use super::reader::Printable;
use super::remote::Fetch;
use super::rich;
use crate::format;

/// How long a printed PDF is kept.
const KEEP: Duration = Duration::from_secs(60 * 60);

/// What is printed: the conversation's subject and its messages, in the
/// desktop's UI font (and the reader's monospace one).
pub(super) struct PrintJob {
    pub subject: String,
    messages: Vec<PrintMessage>,
    family: Option<String>,
    mono: Option<String>,
}

impl PrintJob {
    /// The conversation laid out on `paper`, as a PDF.
    pub(super) fn layout(&self, paper: Paper, options: PrintOptions) -> Result<Vec<u8>, String> {
        let fonts = fonts(self.family.as_deref(), self.mono.as_deref())
            .ok_or_else(|| tr!("print-no-font"))?;
        conversation_pdf(&self.subject, &self.messages, paper, &fonts, options)
            .map_err(|err| err.to_string())
    }

    /// Whether any message has formatting to print (HTML).
    pub(super) fn formatted(&self) -> bool {
        self.messages.iter().any(|m| m.document.is_some())
    }
}

impl MailWindow {
    /// Shows the print preview of the open conversation, every message of
    /// it.
    pub(super) fn print_conversation(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(reader) = &self.reader else {
            return;
        };
        let Ok(mail) = self.mail.as_ref() else {
            return;
        };
        let job = PrintJob {
            subject: reader.subject().to_owned(),
            messages: reader
                .printable(mail)
                .into_iter()
                .map(|p| self.print_message(p))
                .collect(),
            family: self.font.as_ref().map(ToString::to_string),
            mono: self.remote.mono().map(|m| m.to_string()),
        };
        self.open_print_preview(Arc::new(job), window, cx);
    }

    /// Hands `pdf`, `job` laid out on `paper`, to the desktop's print
    /// dialog.
    pub(super) fn print_pdf(
        &mut self,
        job: Arc<PrintJob>,
        pdf: Arc<Vec<u8>>,
        paper: Paper,
        options: PrintOptions,
        cx: &mut Context<Self>,
    ) {
        let dir = print_dir();
        cx.spawn(async move |this, cx| {
            let subject = job.subject.clone();
            let prepared = match portal::prepare(&subject, paper).await {
                portal::Dialog::Cancelled => return,
                portal::Dialog::Prepared(prepared) => Some(prepared),
                portal::Dialog::None => None,
            };
            let picked = prepared
                .as_ref()
                .and_then(portal::Prepared::paper)
                .unwrap_or(paper);
            let written = cx
                .background_executor()
                .spawn(async move {
                    // Other paper than the preview's: lay it out again.
                    let pdf = if same_paper(picked, paper) {
                        pdf
                    } else {
                        Arc::new(job.layout(picked, options)?)
                    };
                    save_pdf(&dir, &pdf)
                })
                .await;
            let path = match written {
                Ok(path) => path,
                Err(err) => {
                    this.update(cx, |this, cx| {
                        this.show_snackbar(tr!("print-failed", error = err.as_str()), None, cx)
                    })
                    .ok();
                    return;
                }
            };
            if let Some(prepared) = prepared {
                match prepared.print(&subject, &path).await {
                    Ok(()) => return,
                    Err(err) => tracing::warn!("printing failed: {err}"),
                }
            }
            // No print dialog to hand it to: open it to print from there.
            cx.update(|cx| cx.open_with_system(&path));
            this.update(cx, |this, cx| {
                this.show_snackbar(tr!("print-opened-as-pdf"), None, cx)
            })
            .ok();
        })
        .detach();
    }

    /// A message as it is printed: its sender, date, recipients and body,
    /// with the remote pictures the reader shows.
    fn print_message(&self, message: Printable) -> PrintMessage {
        let addresses = |list: &[Address]| {
            list.iter()
                .map(|a| match &a.name {
                    Some(name) if !name.eq_ignore_ascii_case(&a.email) => {
                        format!("{name} <{}>", a.email)
                    }
                    _ => a.email.clone(),
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        let date = |unix: Option<i64>| {
            unix.and_then(|d| format::local(d, &self.tz))
                .map(format::long_date)
                .unwrap_or_default()
        };
        let Some(view) = message.view else {
            let row = message.row.as_deref();
            return PrintMessage {
                from: row.map(|r| r.sender.clone()).unwrap_or_default(),
                date: date(row.and_then(|r| r.date)),
                body: tr!("print-not-downloaded"),
                ..PrintMessage::default()
            };
        };
        let label = |list: &[Address], text: fn(String) -> String| {
            if list.is_empty() {
                String::new()
            } else {
                text(addresses(list))
            }
        };
        let sender = view.from.first().map(|a| a.email.as_str()).unwrap_or("");
        let shown = !message.encrypted
            && self
                .remote
                .allowed(message.id, sender, message.authenticated);
        let images = message
            .doc
            .as_ref()
            .filter(|_| shown)
            .map(rich::remote_urls)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|url| match self.remote.images.get(&url) {
                Some(Fetch::Ready(picture)) => {
                    let kind = rich::kind(picture.image.format)?;
                    Some((url, (kind, Arc::from(picture.image.bytes.as_slice()))))
                }
                _ => None,
            })
            .collect();
        let count = view.attachments.len();
        PrintMessage {
            from: addresses(&view.from),
            date: date(view.date.or(message.row.as_ref().and_then(|r| r.date))),
            to: label(&view.to, |list| tr!("print-to", addresses = list)),
            cc: label(&view.cc, |list| tr!("print-cc", addresses = list)),
            body: if message.sealed {
                tr!("print-encrypted")
            } else {
                view.body
            },
            document: message.doc,
            images,
            attachments: view.attachments.into_iter().map(|a| a.name).collect(),
            attachments_label: tr!("attachment-count", count = count),
        }
    }
}

/// The paper people print on where Katna runs: Letter in the Americas that
/// use it, A4 elsewhere. `LC_PAPER` decides, else the language settings.
pub(super) fn local_paper() -> Paper {
    let locale = ["LC_ALL", "LC_PAPER", "LANG"]
        .iter()
        .filter_map(|key| std::env::var(key).ok())
        .find(|value| !value.is_empty())
        .unwrap_or_default();
    paper_for_locale(&locale)
}

fn paper_for_locale(locale: &str) -> Paper {
    // "en_US.UTF-8" -> "US".
    let region = locale
        .split(['.', '@'])
        .next()
        .and_then(|l| l.split_once('_'))
        .map(|(_, region)| region);
    match region {
        Some("US" | "CA" | "MX" | "PH" | "CL" | "CO" | "VE" | "PR" | "GT" | "CR") => Paper::LETTER,
        _ => Paper::A4,
    }
}

/// Whether two papers are the same size, give or take a rounding.
fn same_paper(a: Paper, b: Paper) -> bool {
    (a.width - b.width).abs() < 1.0 && (a.height - b.height).abs() < 1.0
}

/// Where printed PDFs are written.
fn print_dir() -> PathBuf {
    std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("katna")
        .join("print")
}

/// Writes `pdf` to a new file in `dir`, removing what earlier printing
/// left there.
fn save_pdf(dir: &Path, pdf: &[u8]) -> Result<PathBuf, String> {
    let now = SystemTime::now();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let old = entry
                .metadata()
                .and_then(|m| m.modified())
                .ok()
                .and_then(|m| now.duration_since(m).ok())
                .is_some_and(|age| age > KEEP);
            if old {
                std::fs::remove_file(entry.path()).ok();
            }
        }
    }
    let stamp = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let path = dir.join(format!("{stamp:x}.pdf"));
    std::fs::create_dir_all(dir)
        .and_then(|()| std::fs::write(&path, pdf))
        .map_err(|err| err.to_string())?;
    Ok(path)
}

/// The desktop's UI font (`family`), else a common sans-serif one, in
/// regular and, when the family has them, bold, italic and bold italic;
/// and a monospace font (`mono`, else a common one).
fn fonts(family: Option<&str>, mono: Option<&str>) -> Option<PrintFonts> {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    let families: Vec<fontdb::Family> = family
        .into_iter()
        .chain([
            "Noto Sans",
            "DejaVu Sans",
            "Cantarell",
            "Liberation Sans",
            "Segoe UI",
        ])
        .map(fontdb::Family::Name)
        .chain([fontdb::Family::SansSerif])
        .collect();
    let monos: Vec<fontdb::Family> = mono
        .into_iter()
        .chain([
            "Noto Sans Mono",
            "DejaVu Sans Mono",
            "Liberation Mono",
            "Cascadia Mono",
            "Consolas",
        ])
        .map(fontdb::Family::Name)
        .chain([fontdb::Family::Monospace])
        .collect();
    let query = |families: &[fontdb::Family], weight, style| {
        db.query(&fontdb::Query {
            families,
            weight,
            style,
            ..fontdb::Query::default()
        })
    };
    let (upright, italic) = (fontdb::Style::Normal, fontdb::Style::Italic);
    let regular = query(&families, fontdb::Weight::NORMAL, upright)
        .or_else(|| db.faces().next().map(|f| f.id))?;
    // Only a face that is what was asked for: the closest match may be
    // the regular one.
    let styled = |bold: bool, style| {
        let weight = if bold {
            fontdb::Weight::BOLD
        } else {
            fontdb::Weight::NORMAL
        };
        query(&families, weight, style).filter(|id| {
            *id != regular
                && db.face(*id).is_some_and(|f| {
                    // DejaVu's italic is an oblique.
                    let slanted = f.style != fontdb::Style::Normal;
                    slanted == (style == italic) && (f.weight >= fontdb::Weight::SEMIBOLD) == bold
                })
        })
    };
    let mono_face = query(&monos, fontdb::Weight::NORMAL, upright)
        .filter(|id| *id != regular && db.face(*id).is_some_and(|f| f.monospaced));
    let load = |id| {
        db.with_face_data(id, |data, index| PrintFont {
            data: data.to_vec(),
            index,
        })
    };
    Some(PrintFonts {
        regular: load(regular)?,
        bold: styled(true, upright).and_then(load),
        italic: styled(false, italic).and_then(load),
        bold_italic: styled(true, italic).and_then(load),
        mono: mono_face.and_then(load),
    })
}

/// The desktop's print dialog (the print portal).
#[cfg(not(windows))]
mod portal {
    use std::os::fd::AsFd;
    use std::path::Path;

    use ashpd::desktop::ResponseError;
    use ashpd::desktop::print::{
        Orientation, PageSetup, PreparePrintOptions, PrintOptions, PrintProxy,
    };
    use katna_render::print::Paper;

    /// What the print dialog said.
    pub(super) enum Dialog {
        /// The user closed it.
        Cancelled,
        /// The user picked a printer.
        Prepared(Prepared),
        /// There is no dialog to ask.
        None,
    }

    /// A printer and paper picked in the dialog.
    pub(super) struct Prepared {
        proxy: PrintProxy,
        setup: PageSetup,
        token: u32,
    }

    impl Prepared {
        /// The paper picked in the dialog, if it says.
        pub(super) fn paper(&self) -> Option<Paper> {
            paper_of(&self.setup)
        }

        /// Prints the PDF at `path`.
        pub(super) async fn print(&self, title: &str, path: &Path) -> ashpd::Result<()> {
            send(&self.proxy, title, path, self.token).await
        }
    }

    /// Shows the print dialog for `title`, set to `paper`.
    pub(super) async fn prepare(title: &str, paper: Paper) -> Dialog {
        let Ok(proxy) = PrintProxy::new().await else {
            return Dialog::None;
        };
        match ask(&proxy, title, paper).await {
            Ok((setup, token)) => Dialog::Prepared(Prepared {
                proxy,
                setup,
                token,
            }),
            Err(ashpd::Error::Response(ResponseError::Cancelled)) => Dialog::Cancelled,
            Err(err) => {
                tracing::warn!("print dialog failed: {err}");
                Dialog::None
            }
        }
    }

    /// Shows the print dialog, set to `paper`. Returns the page setup picked
    /// there and the token to print with.
    async fn ask(proxy: &PrintProxy, title: &str, paper: Paper) -> ashpd::Result<(PageSetup, u32)> {
        let prepared = proxy
            .prepare_print(
                None,
                title,
                Default::default(),
                page_setup(paper),
                PreparePrintOptions::default().set_modal(false),
            )
            .await?
            .response()?;
        Ok((prepared.page_setup, prepared.token))
    }

    /// Hands the PDF at `path` to the print dialog, which prints it.
    async fn send(proxy: &PrintProxy, title: &str, path: &Path, token: u32) -> ashpd::Result<()> {
        let file = std::fs::File::open(path).map_err(ashpd::Error::IO)?;
        proxy
            .print(
                None,
                title,
                &file.as_fd(),
                PrintOptions::default().set_token(token).set_modal(false),
            )
            .await?;
        Ok(())
    }

    /// The print dialog's page setup for `paper`, upright.
    pub(super) fn page_setup(paper: Paper) -> PageSetup {
        let (ppd, name) = if paper == Paper::LETTER {
            ("Letter", "na_letter")
        } else {
            ("A4", "iso_a4")
        };
        let mm = |points: f32| f64::from(points) * 25.4 / 72.0;
        PageSetup {
            ppdname: Some(ppd.to_owned()),
            name: Some(name.to_owned()),
            display_name: Some(ppd.to_owned()),
            width: Some(mm(paper.width)),
            height: Some(mm(paper.height)),
            orientation: Some(Orientation::Portrait),
            ..PageSetup::default()
        }
    }

    /// The paper the dialog picked, turned when it prints across.
    pub(super) fn paper_of(setup: &PageSetup) -> Option<Paper> {
        let (mut width, mut height) = (setup.width?, setup.height?);
        let across = matches!(
            setup.orientation,
            Some(Orientation::Landscape | Orientation::ReverseLandscape)
        );
        if across == (width < height) {
            std::mem::swap(&mut width, &mut height);
        }
        Paper::from_mm(width, height)
    }
}

/// On Windows the PDF opens in the default PDF app, which prints it.
#[cfg(windows)]
mod portal {
    use std::path::Path;

    use katna_render::print::Paper;

    pub(super) enum Dialog {
        #[allow(dead_code)]
        Cancelled,
        #[allow(dead_code)]
        Prepared(Prepared),
        None,
    }

    pub(super) enum Prepared {}

    impl Prepared {
        pub(super) fn paper(&self) -> Option<Paper> {
            match *self {}
        }

        pub(super) async fn print(&self, _title: &str, _path: &Path) -> Result<(), String> {
            match *self {}
        }
    }

    pub(super) async fn prepare(_title: &str, _paper: Paper) -> Dialog {
        Dialog::None
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(windows))]
    use super::portal::{page_setup, paper_of};
    use super::*;
    #[cfg(not(windows))]
    use ashpd::desktop::print::{Orientation, PageSetup};

    #[test]
    fn writes_a_pdf_in_a_system_font() {
        let dir = std::env::temp_dir().join(format!("katna-print-{}", std::process::id()));
        let job = PrintJob {
            subject: "Hello".into(),
            messages: vec![PrintMessage {
                from: "Ada <ada@example.org>".into(),
                body: "Hello.".into(),
                ..PrintMessage::default()
            }],
            family: Some("No Such Font".into()),
            mono: None,
        };
        let Ok(pdf) = job.layout(Paper::A4, PrintOptions::default()) else {
            // A system without fonts (a bare CI image) cannot print.
            assert!(fonts(None, None).is_none());
            return;
        };
        let path = save_pdf(&dir, &pdf).unwrap();
        assert!(std::fs::read(&path).unwrap().starts_with(b"%PDF-"));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn letter_where_it_is_used() {
        assert_eq!(paper_for_locale("en_US.UTF-8"), Paper::LETTER);
        assert_eq!(paper_for_locale("es_MX"), Paper::LETTER);
        assert_eq!(paper_for_locale("en_IN.UTF-8"), Paper::A4);
        assert_eq!(paper_for_locale("de_DE@euro"), Paper::A4);
        assert_eq!(paper_for_locale("C"), Paper::A4);
    }

    #[cfg(not(windows))]
    #[test]
    fn the_dialog_starts_on_the_previewed_paper() {
        let setup = page_setup(Paper::LETTER);
        assert_eq!(setup.ppdname.as_deref(), Some("Letter"));
        assert!(same_paper(paper_of(&setup).unwrap(), Paper::LETTER));
        assert!(!same_paper(Paper::A4, Paper::LETTER));
    }

    #[cfg(not(windows))]
    #[test]
    fn paper_turns_for_landscape() {
        let setup = PageSetup {
            width: Some(210.0),
            height: Some(297.0),
            orientation: Some(Orientation::Landscape),
            ..PageSetup::default()
        };
        let paper = turned(&setup);
        assert!(paper.width > paper.height);
        let setup = PageSetup {
            orientation: Some(Orientation::Portrait),
            ..setup
        };
        assert!(turned(&setup).width < turned(&setup).height);
    }

    #[cfg(not(windows))]
    fn turned(setup: &PageSetup) -> Paper {
        paper_of(setup).unwrap()
    }
}
