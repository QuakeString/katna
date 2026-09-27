// SPDX-License-Identifier: GPL-3.0-or-later

//! Printing the open conversation: the desktop's print dialog asks for
//! the printer and paper, then the conversation is laid out as a PDF on
//! that paper ([`katna_render::print`]) and handed back to the dialog,
//! which prints it. Without a print dialog (no portal), the PDF opens in
//! the default app to print from there.
//!
//! The PDF is written to `$XDG_RUNTIME_DIR/katna/print`, which is private
//! and emptied at logout; copies older than an hour are removed.

use std::os::fd::AsFd;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use ashpd::desktop::ResponseError;
use ashpd::desktop::print::{
    Orientation, PageSetup, PreparePrintOptions, PrintOptions, PrintProxy,
};
use gpui::Context;
use katna_i18n::tr;
use katna_render::Address;
use katna_render::print::{Paper, PrintFont, PrintMessage, conversation_pdf};

use super::MailWindow;
use super::reader::Printable;
use crate::format;

/// How long a printed PDF is kept.
const KEEP: Duration = Duration::from_secs(60 * 60);

impl MailWindow {
    /// Prints the open conversation, every message of it.
    pub(super) fn print_conversation(&mut self, cx: &mut Context<Self>) {
        let Some(reader) = &self.reader else {
            return;
        };
        let Ok(mail) = self.mail.as_ref() else {
            return;
        };
        let subject = reader.subject().to_owned();
        let messages: Vec<PrintMessage> = reader
            .printable(mail)
            .into_iter()
            .map(|p| self.print_message(p))
            .collect();
        let family = self.font.as_ref().map(ToString::to_string);
        let dir = print_dir();
        cx.spawn(async move |this, cx| {
            let proxy = PrintProxy::new().await.ok();
            // The dialog first, so it shows at once; the PDF is laid out
            // on the paper picked there.
            let mut prepared = None;
            if let Some(proxy) = &proxy {
                match ask(proxy, &subject).await {
                    Ok(setup) => prepared = Some(setup),
                    Err(ashpd::Error::Response(ResponseError::Cancelled)) => return,
                    Err(err) => tracing::warn!("print dialog failed: {err}"),
                }
            }
            let paper = prepared
                .as_ref()
                .and_then(|(setup, _)| paper(setup))
                .unwrap_or(Paper::A4);
            let title = subject.clone();
            let written = cx
                .background_executor()
                .spawn(async move { write_pdf(&dir, &title, &messages, paper, family.as_deref()) })
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
            if let (Some(proxy), Some((_, token))) = (&proxy, prepared) {
                match send(proxy, &subject, &path, token).await {
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

    /// A message as it is printed: its sender, date, recipients and text.
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
            attachments: view.attachments.into_iter().map(|a| a.name).collect(),
        }
    }
}

/// Shows the print dialog. Returns the page setup picked there and the
/// token to print with.
async fn ask(proxy: &PrintProxy, title: &str) -> ashpd::Result<(PageSetup, u32)> {
    let prepared = proxy
        .prepare_print(
            None,
            title,
            Default::default(),
            Default::default(),
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

/// The paper the dialog picked, turned when it prints across.
fn paper(setup: &PageSetup) -> Option<Paper> {
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

/// Where printed PDFs are written.
fn print_dir() -> PathBuf {
    std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("katna")
        .join("print")
}

/// Lays the conversation out and writes it to a new file in `dir`,
/// removing what earlier printing left there.
fn write_pdf(
    dir: &Path,
    subject: &str,
    messages: &[PrintMessage],
    paper: Paper,
    family: Option<&str>,
) -> Result<PathBuf, String> {
    let (regular, bold) = fonts(family).ok_or_else(|| tr!("print-no-font"))?;
    let pdf = conversation_pdf(subject, messages, paper, &regular, bold.as_ref())
        .map_err(|err| err.to_string())?;
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
/// regular and, when the family has one, bold.
fn fonts(family: Option<&str>) -> Option<(PrintFont, Option<PrintFont>)> {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    let families: Vec<fontdb::Family> = family
        .into_iter()
        .chain(["Noto Sans", "DejaVu Sans", "Cantarell", "Liberation Sans"])
        .map(fontdb::Family::Name)
        .chain([fontdb::Family::SansSerif])
        .collect();
    let face = |weight| {
        db.query(&fontdb::Query {
            families: &families,
            weight,
            ..fontdb::Query::default()
        })
    };
    let regular = face(fontdb::Weight::NORMAL).or_else(|| db.faces().next().map(|f| f.id))?;
    let bold = face(fontdb::Weight::BOLD).filter(|bold| *bold != regular);
    let load = |id| {
        db.with_face_data(id, |data, index| PrintFont {
            data: data.to_vec(),
            index,
        })
    };
    Some((load(regular)?, bold.and_then(load)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_a_pdf_in_a_system_font() {
        let dir = std::env::temp_dir().join(format!("katna-print-{}", std::process::id()));
        let messages = [PrintMessage {
            from: "Ada <ada@example.org>".into(),
            body: "Hello.".into(),
            ..PrintMessage::default()
        }];
        let Ok(path) = write_pdf(&dir, "Hello", &messages, Paper::A4, Some("No Such Font")) else {
            // A system without fonts (a bare CI image) cannot print.
            assert!(fonts(None).is_none());
            return;
        };
        let pdf = std::fs::read(&path).unwrap();
        assert!(pdf.starts_with(b"%PDF-"));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn paper_turns_for_landscape() {
        let setup = PageSetup {
            width: Some(210.0),
            height: Some(297.0),
            orientation: Some(Orientation::Landscape),
            ..PageSetup::default()
        };
        let paper = paper(&setup).unwrap();
        assert!(paper.width > paper.height);
        let setup = PageSetup {
            orientation: Some(Orientation::Portrait),
            ..setup
        };
        assert!(paper_of(&setup).width < paper_of(&setup).height);
    }

    fn paper_of(setup: &PageSetup) -> Paper {
        paper(setup).unwrap()
    }
}
