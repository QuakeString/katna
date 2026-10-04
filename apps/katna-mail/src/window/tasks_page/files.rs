// SPDX-License-Identifier: GPL-3.0-or-later

//! Files on tasks: dropped on a task's row or its details, picked there,
//! or kept from the mail a task is made from (Add to Tasks). Each opens
//! in Katna's viewer, or the app Default apps names, as a mail's
//! attachment does. The daemon sends them to To Do or the CalDAV server
//! where it can, and keeps the rest on this computer.

use std::path::PathBuf;
use std::sync::Arc;

use gpui::{AnyElement, Context, ExternalPaths, PathPromptOptions, Window, div, prelude::*, rgba};
use katna_core::config::OpenIn;
use katna_dbus::agenda::MAX_TASK_FILE;
use katna_i18n::tr;
use katna_render::AttachmentFile;
use katna_store::tasks::TaskFile;
use katna_store::{MessageId, Mode, Store};
use katna_ui::px;
use katna_ui::tokens::{space, text};

use super::super::MailWindow;
use super::super::attachments::Item;
use super::super::compose::attach::mime_of;
use crate::tasks::{NewFile, TaskCommand};
use crate::theme::{Theme, fade};
use crate::widgets::{icon, icon_button, row, tag, tip};

/// Reads `paths` as files to put on a task: folders and files too large
/// are left out, and named in the second list.
fn read_files(paths: Vec<PathBuf>) -> (Vec<NewFile>, Vec<String>) {
    let mut files = Vec::new();
    let mut left = Vec::new();
    for path in paths {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let size = std::fs::metadata(&path).map_or(u64::MAX, |m| m.len());
        if !path.is_file() || size > MAX_TASK_FILE as u64 {
            left.push(name);
            continue;
        }
        match std::fs::read(&path) {
            Ok(data) => files.push(NewFile {
                mime: mime_of(&name),
                name,
                data,
            }),
            Err(err) => {
                tracing::warn!(%err, "reading a file for a task");
                left.push(name);
            }
        }
    }
    (files, left)
}

/// The files of the mails a task is made from (Add to Tasks): every
/// attachment, each once, but pictures the mail shows in its text.
pub(super) fn mail_files(raws: &[Vec<u8>]) -> Vec<NewFile> {
    let mut files: Vec<NewFile> = Vec::new();
    for raw in raws {
        let view = katna_render::message_view(raw);
        for (ix, attachment) in view.attachments.iter().enumerate() {
            let inline = attachment.content_id.is_some() && attachment.mime.starts_with("image/");
            if inline || attachment.size > MAX_TASK_FILE as u64 {
                continue;
            }
            let Some(file) = katna_render::attachment_file(raw, ix) else {
                continue;
            };
            if files
                .iter()
                .any(|f| f.name == file.name && f.data == file.bytes)
            {
                continue;
            }
            files.push(NewFile {
                name: file.name,
                mime: file.mime,
                data: file.bytes,
            });
        }
    }
    files
}

impl MailWindow {
    /// How many files task `task` has, if any.
    pub(super) fn task_file_count(&self, task: &katna_store::tasks::Task) -> Option<usize> {
        let count = self.tasks.board().map_or(0, |b| b.files_of(task.id).len());
        (count > 0).then_some(count)
    }

    /// Puts the files at `paths` on task `id`: those dropped on it or
    /// picked for it.
    pub(super) fn task_attach_paths(
        &mut self,
        id: i64,
        paths: Vec<PathBuf>,
        cx: &mut Context<Self>,
    ) {
        let read = cx
            .background_executor()
            .spawn(async move { read_files(paths) });
        cx.spawn(async move |this, cx| {
            let (files, left) = read.await;
            this.update(cx, |this, cx| {
                if !left.is_empty() {
                    let limit = crate::format::size(MAX_TASK_FILE as u64);
                    this.show_snackbar(
                        tr!(
                            "tasks-files-left-out",
                            names = left.join(", "),
                            limit = limit
                        ),
                        None,
                        cx,
                    );
                }
                if files.is_empty() {
                    return;
                }
                let count = files.len() as u64;
                this.send_task(
                    TaskCommand::AddFiles(id, files),
                    Some(tr!("tasks-files-added", count = count)),
                    None,
                    cx,
                );
            })
            .ok();
        })
        .detach();
    }

    /// Asks for files to put on task `id`.
    pub(super) fn task_pick_files(&mut self, id: i64, cx: &mut Context<Self>) {
        let chosen = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: Some(tr!("tasks-files-pick").into()),
        });
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(paths))) = chosen.await else {
                return;
            };
            this.update(cx, |this, cx| this.task_attach_paths(id, paths, cx))
                .ok();
        })
        .detach();
    }

    /// Takes file `file` off its task, with Undo.
    pub(super) fn task_remove_file(&mut self, file: &TaskFile, cx: &mut Context<Self>) {
        // Read now, for Undo; the store keeps the bytes meanwhile.
        let data = Store::open(&self.paths, Mode::ReadOnly)
            .ok()
            .and_then(|store| store.task_file_data(file.id).ok().flatten());
        if let Some(board) = self.tasks.board_mut()
            && let Some(files) = board.files.get_mut(&file.task)
        {
            files.retain(|f| f.id != file.id);
        }
        let undo = data.map(|data| {
            TaskCommand::AddFiles(
                file.task,
                vec![NewFile {
                    name: file.name.clone(),
                    mime: file.mime.clone(),
                    data,
                }],
            )
        });
        self.send_task(
            TaskCommand::RemoveFile(file.id),
            Some(tr!("tasks-file-removed", name = file.name.clone())),
            undo,
            cx,
        );
        cx.notify();
    }

    /// The files of task `id` with their bytes, to put back with it when
    /// its deletion is undone.
    pub(super) fn task_files_for_undo(&self, id: i64) -> Vec<NewFile> {
        let Some(files) = self.tasks.board().map(|b| b.files_of(id).to_vec()) else {
            return Vec::new();
        };
        if files.is_empty() {
            return Vec::new();
        }
        let Ok(store) = Store::open(&self.paths, Mode::ReadOnly) else {
            return Vec::new();
        };
        files
            .into_iter()
            .filter_map(|file| {
                let data = store.task_file_data(file.id).ok().flatten()?;
                Some(NewFile {
                    name: file.name,
                    mime: file.mime,
                    data,
                })
            })
            .collect()
    }

    /// Opens file `file` of a task: in Katna's viewer, with the task's
    /// other files a click of the arrows away, or in another app, as
    /// Settings > Default apps says for its kind.
    pub(super) fn task_open_file(
        &mut self,
        file: &TaskFile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let files: Vec<TaskFile> = self
            .tasks
            .board()
            .map(|b| b.files_of(file.task).to_vec())
            .unwrap_or_default();
        let index = files.iter().position(|f| f.id == file.id).unwrap_or(0);
        let paths = self.paths.clone();
        let ids: Vec<i64> = files.iter().map(|f| f.id).collect();
        let read = cx.background_executor().spawn(async move {
            let store = Store::open(&paths, Mode::ReadOnly).ok()?;
            ids.iter()
                .map(|id| store.task_file_data(*id).ok().flatten())
                .collect::<Option<Vec<Vec<u8>>>>()
        });
        let window_handle = window.window_handle();
        cx.spawn(async move |this, cx| {
            let Some(datas) = read.await else {
                this.update(cx, |this, cx| {
                    this.show_snackbar(tr!("tasks-file-missing"), None, cx)
                })
                .ok();
                return;
            };
            let parts: Vec<crate::outgoing::Part> = files
                .iter()
                .zip(datas)
                .map(|(f, data)| crate::outgoing::Part {
                    name: f.name.clone(),
                    mime: f.mime.clone(),
                    data: Arc::new(data),
                    content_id: None,
                })
                .collect();
            cx.update_window(window_handle, |_, window, cx| {
                this.update(cx, |this, cx| {
                    this.task_show_files(parts, index, window, cx)
                })
                .ok();
            })
            .ok();
        })
        .detach();
    }

    /// Shows `parts` (a task's files, read) in the viewer at `index`, or
    /// hands that one to another app.
    fn task_show_files(
        &mut self,
        parts: Vec<crate::outgoing::Part>,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let items: Vec<Item> = parts
            .iter()
            .enumerate()
            .map(|(ix, p)| Item {
                index: ix,
                name: p.name.clone(),
                size: p.data.len() as u64,
                kind: katna_preview::kind(&p.mime, &p.name),
                risky: katna_preview::risky(&p.mime, &p.name),
            })
            .collect();
        let Some(item) = items.get(index) else {
            return;
        };
        let open_in = self.open_in(item);
        if open_in != OpenIn::Katna {
            let part = &parts[index];
            let file = AttachmentFile {
                name: part.name.clone(),
                mime: part.mime.clone(),
                bytes: part.data.to_vec(),
            };
            self.open_attachment_with(Arc::new(file), open_in == OpenIn::Ask, false, cx);
            return;
        }
        // The viewer reads files from a message: the task's, in order,
        // make one.
        let raw = Arc::new(crate::outgoing::build(&crate::outgoing::Outgoing {
            attachments: parts,
            ..Default::default()
        }));
        self.show_viewer(raw, false, items, index, None::<MessageId>, window, cx);
    }

    /// The files in a task's details: each opens with a click and has a
    /// button to take it off; the paper clip picks more.
    pub(super) fn render_details_files(
        &self,
        id: i64,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let files: Vec<TaskFile> = self
            .tasks
            .board()
            .map(|b| b.files_of(id).to_vec())
            .unwrap_or_default();
        let rows = files.into_iter().enumerate().map(|(ix, file)| {
            let open = file.clone();
            let gone = file.clone();
            let kind = katna_preview::kind(&file.mime, &file.name);
            let mark = if matches!(kind, katna_preview::Kind::Picture(_)) {
                "image"
            } else {
                "file"
            };
            // A row with an edge, as it can be clicked.
            row(("task-file", ix), false, th)
                .pl(px(space::S4))
                .pr(px(space::S2))
                .py(px(space::S1))
                .border_1()
                .border_color(rgba(th.outline))
                .tooltip(tip(tr!("tasks-file-open"), th))
                .on_click(
                    cx.listener(move |this, _, window, cx| this.task_open_file(&open, window, cx)),
                )
                .child(icon(mark, th.text_dim, 18.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .text_size(px(text::BODY))
                        .child(file.name.clone()),
                )
                .when(file.local_only, |d| {
                    d.child(tag(tr!("tasks-file-here"), th))
                })
                .child(
                    div()
                        .flex_none()
                        .text_size(px(text::CAPTION))
                        .text_color(rgba(th.text_faint))
                        .child(crate::format::size(file.size)),
                )
                .child(
                    icon_button(("task-file-remove", ix), "close", 18.0, th)
                        .size(px(28.0))
                        .tooltip(tip(tr!("tasks-file-remove"), th))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            cx.stop_propagation();
                            this.task_remove_file(&gone, cx)
                        })),
                )
        });
        div()
            .flex()
            .flex_col()
            .gap(px(space::S2))
            .children(rows)
            .into_any_element()
    }
}

/// Lets `row` take files dropped on it for task `id`, tinted while files
/// are over it.
pub(super) fn takes_files<E: InteractiveElement + StatefulInteractiveElement>(
    row: E,
    id: i64,
    th: &Theme,
    cx: &mut Context<MailWindow>,
) -> E {
    let tint = fade(th.accent, 0.12);
    row.drag_over::<ExternalPaths>(move |s, _, _, _| s.bg(rgba(tint)))
        .on_drop(cx.listener(move |this, paths: &ExternalPaths, _, cx| {
            cx.stop_propagation();
            this.task_attach_paths(id, paths.paths().to_vec(), cx)
        }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::outgoing::{Outgoing, Part, build};

    fn part(name: &str, mime: &str, data: &[u8], content_id: Option<&str>) -> Part {
        Part {
            name: name.into(),
            mime: mime.into(),
            data: Arc::new(data.to_vec()),
            content_id: content_id.map(Into::into),
        }
    }

    #[test]
    fn a_mails_attachments_become_files_once() {
        let first = build(&Outgoing {
            subject: "Trip budget".into(),
            body: "See the budget.".into(),
            html: Some("<p>See <img src=\"cid:logo\"></p>".into()),
            inline: vec![part("logo.png", "image/png", b"\x89PNG logo", Some("logo"))],
            attachments: vec![part("budget.pdf", "application/pdf", b"%PDF budget", None)],
            ..Default::default()
        });
        // A reply with the same file again, and another.
        let reply = build(&Outgoing {
            subject: "Re: Trip budget".into(),
            body: "And the list.".into(),
            attachments: vec![
                part("budget.pdf", "application/pdf", b"%PDF budget", None),
                part("list.txt", "text/plain", b"sunscreen", None),
            ],
            ..Default::default()
        });
        let files = mail_files(&[first, reply]);
        let names: Vec<&str> = files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, ["budget.pdf", "list.txt"]);
        assert_eq!(files[0].mime, "application/pdf");
        assert_eq!(files[0].data, b"%PDF budget");
    }
}
