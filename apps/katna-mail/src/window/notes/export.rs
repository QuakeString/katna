// SPDX-License-Identifier: GPL-3.0-or-later

//! A note going elsewhere: Send as mail opens Compose with the note as
//! its text and pictures; Save as writes it as Markdown or as a PDF. The
//! ⋮ menu's other actions on the open note are here too.

use std::fmt::Write as _;
use std::sync::Arc;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use gpui::{Context, Focusable as _, Window};
use katna_i18n::tr;
use katna_render::print::{PrintMessage, PrintOptions};
use katna_store::NOTE_LINK_SCHEME;
use katna_ui::rich::{Block, Doc, Font, Image, ImageSize, List, Para, Size, html};

use super::{MailWindow, Note, TICKED, UNTICKED, check_of, format};
use crate::daemon::Command;

/// A picture as it leaves a note: name, MIME type and bytes.
type Picture = (String, String, Vec<u8>);

/// `doc` as Markdown under `title`, its `pictures` (name, MIME type and
/// bytes) on top as Keep shows them, written into the file.
pub(super) fn markdown(title: &str, doc: &Doc, pictures: &[Picture]) -> String {
    let mut out = String::new();
    if !title.trim().is_empty() {
        let _ = writeln!(out, "# {}\n", title.trim());
    }
    for (name, mime, data) in pictures {
        let _ = writeln!(
            out,
            "![{name}](data:{mime};base64,{})\n",
            STANDARD.encode(data)
        );
    }
    let mut numbers: Vec<u32> = Vec::new();
    for block in &doc.blocks {
        match block {
            Block::Para(para) => {
                let line = para_markdown(para, &mut numbers);
                out.push_str(&line);
                out.push('\n');
            }
            Block::Html(block) if block.html.contains("<hr") => out.push_str("\n---\n\n"),
            Block::Html(block) => {
                out.push_str(&block.text);
                out.push('\n');
            }
            Block::Table(table) => {
                for (r, row) in table.rows.iter().enumerate() {
                    let cells: Vec<String> = row
                        .iter()
                        .map(|cell| cell.text.replace('|', "\\|"))
                        .collect();
                    let _ = writeln!(out, "| {} |", cells.join(" | "));
                    if r == 0 {
                        let _ = writeln!(out, "|{}", " --- |".repeat(cells.len()));
                    }
                }
            }
            Block::Image(_) => {}
        }
    }
    out.trim_end().to_owned() + "\n"
}

/// One paragraph as a Markdown line.
fn para_markdown(para: &Para, numbers: &mut Vec<u32>) -> String {
    let style = para.style;
    let indent = "  ".repeat(usize::from(style.indent));
    let mut lead = String::new();
    lead.push_str(&"> ".repeat(usize::from(style.quote)));
    lead.push_str(&indent);
    let (check, skip) = match check_of(&para.text) {
        Some((true, _)) => (Some("- [x] "), TICKED.len()),
        Some((false, _)) => (Some("- [ ] "), UNTICKED.len()),
        None => (None, 0),
    };
    let level = usize::from(style.indent);
    match (check, style.list) {
        (Some(mark), _) => lead.push_str(mark),
        (None, List::Bullet) => lead.push_str("- "),
        (None, List::Numbered) => {
            numbers.resize(level + 1, 0);
            numbers[level] += 1;
            let _ = write!(lead, "{}. ", numbers[level]);
        }
        (None, _) => {
            numbers.clear();
            match para.style_at(0).size {
                Size::Huge if para.style_at(0).bold => lead.push_str("# "),
                Size::Large if para.style_at(0).bold => lead.push_str("## "),
                _ => {}
            }
        }
    }
    let heading = lead.ends_with("# ");
    let mut text = String::new();
    for (range, char_style) in para.spans() {
        let start = range.start.max(skip);
        if start >= range.end {
            continue;
        }
        let piece = &para.text[start..range.end];
        if piece.is_empty() {
            continue;
        }
        let mut piece = piece.to_owned();
        if char_style.font == Font::Fixed {
            piece = format!("`{piece}`");
        }
        if !heading && char_style.bold {
            piece = format!("**{piece}**");
        }
        if char_style.italic {
            piece = format!("*{piece}*");
        }
        if char_style.strike {
            piece = format!("~~{piece}~~");
        }
        if let Some(link) = &char_style.link
            && !link.starts_with(NOTE_LINK_SCHEME)
        {
            piece = format!("[{piece}]({link})");
        }
        text.push_str(&piece);
    }
    format!("{lead}{text}")
}

impl MailWindow {
    /// The open note as it is now: its saved form, with what is typed.
    fn open_note_now(&self, cx: &gpui::App) -> Option<(Note, Doc, Vec<Picture>)> {
        let editor = self.notes.as_ref()?.editor.as_ref()?;
        let item = editor.item(cx);
        let note = Note {
            id: editor.id,
            title: item.title,
            body: item.body,
            html: item.html,
            color: item.color,
            pinned: item.pinned,
            archived: item.archived,
            labels: item.labels,
            link: Some(item.link).filter(|l| !l.is_empty()),
            account_id: editor.account,
            remind_at: editor.remind_at,
            ..Note::default()
        };
        let pictures = editor
            .pictures
            .iter()
            .map(|p| (p.name.clone(), p.mime.clone(), p.data.clone()))
            .collect();
        Some((note, editor.body.read(cx).doc().clone(), pictures))
    }

    /// ⋮ > Show checkboxes, or Hide checkboxes.
    pub(super) fn toggle_open_checklist(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(editor) = self.notes.as_ref().and_then(|p| p.editor.as_ref()) else {
            return;
        };
        let body = editor.body.clone();
        body.update(cx, |area, cx| {
            area.edit_doc(format::toggle_checklist_doc, cx)
        });
        window.focus(&body.focus_handle(cx), cx);
    }

    /// ⋮ > Archive, or Unarchive, on the open note.
    pub(super) fn archive_open_note(&mut self, cx: &mut Context<Self>) {
        let Some((note, _, _)) = self.open_note_now(cx) else {
            return;
        };
        let archived = note.archived;
        self.archive_note(&note, !archived, cx);
    }

    /// ⋮ > Make a copy: the open note again, pictures too, on top.
    pub(super) fn copy_open_note(&mut self, cx: &mut Context<Self>) {
        let Some(editor) = self.notes.as_ref().and_then(|p| p.editor.as_ref()) else {
            return;
        };
        let mut item = editor.item(cx);
        item.id = 0;
        item.pinned = false;
        item.pictures_set = !editor.pictures.is_empty();
        item.pictures = editor.pictures.clone();
        self.send(
            Command::SaveNote(Box::new(item)),
            Some(tr!("notes-copied-count", count = 1u64)),
            None,
            false,
            cx,
        );
    }

    /// ⋮ > Send as mail: a new message with the note's title as its
    /// subject and its text and pictures as the body, above the
    /// signature.
    pub(super) fn send_note_as_mail(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some((note, doc, pictures)) = self.open_note_now(cx) else {
            return;
        };
        self.close_note(cx);
        let mut next = 1_000_000u64;
        let mut blocks: Vec<Block> = pictures
            .into_iter()
            .map(|(name, mime, data)| {
                next += 1;
                let (width, height) = katna_ui::rich::image_size(&data).unwrap_or((0, 0));
                Block::Image(Image {
                    id: next,
                    name,
                    mime,
                    data: Arc::new(data),
                    width,
                    height,
                    size: ImageSize::default(),
                })
            })
            .collect();
        // Links to other notes mean nothing in mail: their text stays.
        let mut doc = doc;
        for path in doc.paths() {
            if let Some(para) = doc.para_mut(path) {
                let len = para.len();
                para.restyle(0..len, &|s| {
                    if s.link
                        .as_deref()
                        .is_some_and(|l| l.starts_with(NOTE_LINK_SCHEME))
                    {
                        s.link = None;
                    }
                });
            }
        }
        blocks.extend(doc.blocks);
        self.open_compose_with(note.title.clone(), blocks, window, cx);
    }

    /// ⋮ > Save as Markdown.
    pub(super) fn save_note_as_markdown(&mut self, cx: &mut Context<Self>) {
        let Some((note, doc, pictures)) = self.open_note_now(cx) else {
            return;
        };
        let text = markdown(&note.title, &doc, &pictures);
        let name = format!("{}.md", super::links::note_name(&note));
        self.save_note_file(name, text.into_bytes(), cx);
    }

    /// ⋮ > Save as PDF: the note laid out as Print lays out mail, its
    /// formatting and pictures kept.
    pub(super) fn save_note_as_pdf(&mut self, cx: &mut Context<Self>) {
        let Some((note, doc, pictures)) = self.open_note_now(cx) else {
            return;
        };
        let name = super::links::note_name(&note);
        let mut page = String::new();
        for (ix, _) in pictures.iter().enumerate() {
            let _ = write!(
                page,
                "<p><img src=\"cid:p{ix}\" style=\"max-width:100%\"></p>"
            );
        }
        page.push_str(&html::to_html(&doc, &|_| String::new()));
        let pictures: Vec<Arc<[u8]>> = pictures
            .into_iter()
            .map(|(_, _, data)| Arc::from(data.as_slice()))
            .collect();
        let document = katna_render::html::document(&page, &|cid| {
            let n: usize = cid.strip_prefix('p')?.parse().ok()?;
            pictures.get(n).cloned()
        });
        let message = PrintMessage {
            body: note.body.clone(),
            document: Some(document),
            ..PrintMessage::default()
        };
        let family = self.font.as_ref().map(ToString::to_string);
        let job = super::super::print::PrintJob::text(note.title.clone(), vec![message], family);
        let paper = super::super::print::local_paper();
        cx.spawn(async move |this, cx| {
            let pdf = cx
                .background_executor()
                .spawn(async move { job.layout(paper, PrintOptions::default()) })
                .await;
            this.update(cx, |this, cx| match pdf {
                Ok(pdf) => this.save_note_file(format!("{name}.pdf"), pdf, cx),
                Err(err) => this.show_snackbar(tr!("print-failed", error = err.as_str()), None, cx),
            })
            .ok();
        })
        .detach();
    }

    /// Asks where to save `bytes` as `name` (Downloads without a file
    /// chooser), saves it there and says so.
    fn save_note_file(&mut self, name: String, bytes: Vec<u8>, cx: &mut Context<Self>) {
        let dir = super::super::attachments::download_dir();
        let name = super::super::attachments::safe_name(&name);
        let prompt = cx.prompt_for_new_path(&dir, Some(&name));
        cx.spawn(async move |this, cx| {
            let target = match prompt.await {
                Ok(Ok(Some(path))) => path,
                Ok(Ok(None)) => return,
                _ => super::super::attachments::unique_path(&dir, &name),
            };
            let saved = cx
                .background_executor()
                .spawn(async move { std::fs::write(&target, &bytes).map(|()| target) })
                .await;
            this.update(cx, |this, cx| {
                let text = match saved {
                    Ok(path) => {
                        if this.config.mail.open_saved_folder {
                            cx.reveal_path(&path);
                        }
                        tr!("attachment-saved-to", path = path.display().to_string())
                    }
                    Err(err) => tr!(
                        "attachment-save-failed",
                        name = name.as_str(),
                        error = err.to_string()
                    ),
                };
                this.show_snackbar(text, None, cx);
            })
            .ok();
        })
        .detach();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_note_writes_as_markdown() {
        let doc = format::doc_of(&format!("{UNTICKED}Sunscreen\n{TICKED}Tickets\nPlain"), "");
        let text = markdown("Packing", &doc, &[]);
        assert_eq!(text, "# Packing\n\n- [ ] Sunscreen\n- [x] Tickets\nPlain\n");
    }

    #[test]
    fn formatting_and_links_write_as_markdown() {
        let doc = html::from_html(
            "<p><b>Bold</b> and <a href=\"https://x.org\">a link</a> and \
             <a href=\"katna-note:U1\">a note</a></p><blockquote><p>Quoted</p></blockquote>",
            &mut 0,
        );
        let text = markdown("", &doc, &[]);
        assert!(
            text.contains("**Bold** and [a link](https://x.org) and a note"),
            "{text}"
        );
        assert!(text.contains("> Quoted"), "{text}");
    }
}
