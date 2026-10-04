// SPDX-License-Identifier: GPL-3.0-or-later

//! A note's version history: the clock button shows the versions kept on
//! this computer for 30 days (`katna-store`), each with what it changed
//! from the one before, removed text struck through in red and added text
//! tinted green. Restore puts a version back, with Undo.

use gpui::{AnyElement, Context, FontWeight, Window, deferred, div, prelude::*, rgba};
use jiff::Timestamp;
use jiff::tz::TimeZone;
use katna_i18n::{format, tr};
use katna_store::{Note, NoteVersion, VersionSource};
use katna_ui::tokens::{radius, space, text};
use katna_ui::{px, unpx};

use super::{MailWindow, check_of, item_of};
use crate::daemon::Command;
use crate::theme::{Theme, fade};
use crate::widgets::{dialog, filled_button, outlined_button};

/// The red of removed text and the green of added text.
const REMOVED: u32 = 0xd930_25ff;
const ADDED: u32 = 0x1880_38ff;

/// The history dialog of a note.
pub(super) struct History {
    note: Note,
    versions: Vec<NoteVersion>,
    /// The version shown.
    selected: usize,
}

/// A line of a version against the one before it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Line {
    Same(String),
    Removed(String),
    Added(String),
    /// A line rewritten: the old text struck, the new tinted.
    Changed(String, String),
}

/// What changed from `old` to `new`, line by line (longest common
/// subsequence); a removed line followed by an added one is one changed
/// line.
pub(super) fn diff(old: &str, new: &str) -> Vec<Line> {
    let a: Vec<&str> = old.split('\n').collect();
    let b: Vec<&str> = new.split('\n').collect();
    let (n, m) = (a.len(), b.len());
    let mut lcs = vec![vec![0u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            lcs[i][j] = if a[i] == b[j] {
                lcs[i + 1][j + 1] + 1
            } else {
                lcs[i + 1][j].max(lcs[i][j + 1])
            };
        }
    }
    let mut out = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < n || j < m {
        if i < n && j < m && a[i] == b[j] {
            out.push(Line::Same(a[i].to_owned()));
            i += 1;
            j += 1;
        } else if i < n && (j >= m || lcs[i + 1][j] >= lcs[i][j + 1]) {
            out.push(Line::Removed(a[i].to_owned()));
            i += 1;
        } else {
            match out.last_mut() {
                Some(last @ Line::Removed(_)) => {
                    let Line::Removed(old) = last.clone() else {
                        unreachable!()
                    };
                    *last = Line::Changed(old, b[j].to_owned());
                }
                _ => out.push(Line::Added(b[j].to_owned())),
            }
            j += 1;
        }
    }
    out
}

/// How many lines changed.
fn changes(lines: &[Line]) -> usize {
    lines.iter().filter(|l| !matches!(l, Line::Same(_))).count()
}

/// When a version was written: "Today, 11:40", "Yesterday, 21:15",
/// "Wed, 30 Sep".
fn when(at: i64, now: i64, tz: &TimeZone) -> String {
    let (Some(at), Some(now)) = (crate::format::local(at, tz), crate::format::local(now, tz))
    else {
        return String::new();
    };
    match (now.date() - at.date()).get_days() {
        0 => tr!("notes-remind-today", time = format::time(at)),
        1 => tr!("notes-version-yesterday", time = format::time(at)),
        _ => tr!(
            "notes-remind-weekday",
            day = format::weekday(at),
            time = format::day_month(at)
        ),
    }
}

/// A version's whole text, its title on top.
fn text_of(version: &NoteVersion) -> String {
    version.body.clone()
}

impl MailWindow {
    /// The clock button: the open note's versions.
    pub(super) fn open_note_history(&mut self, cx: &mut Context<Self>) {
        // What is typed goes in first, so "Now" is now.
        self.save_open_note(cx);
        let Some(editor) = self.notes.as_ref().and_then(|p| p.editor.as_ref()) else {
            return;
        };
        if editor.id == 0 {
            self.show_snackbar(tr!("notes-history-none"), None, cx);
            return;
        }
        let id = editor.id;
        let mut note = self
            .notes
            .as_ref()
            .and_then(|p| p.notes.as_ref())
            .and_then(|n| n.as_ref().ok())
            .and_then(|notes| notes.iter().find(|n| n.id == id).cloned())
            .unwrap_or_default();
        let item = editor.item(cx);
        note.id = id;
        note.title = item.title;
        note.body = item.body;
        note.html = item.html;
        let mut versions = crate::data::note_versions(&self.paths, id).unwrap_or_default();
        // "Now" is always first, as the note is open.
        let now_same = versions
            .first()
            .is_some_and(|v| v.title == note.title && v.body == note.body && v.html == note.html);
        if !now_same {
            versions.insert(
                0,
                NoteVersion {
                    id: 0,
                    at: Timestamp::now().as_second(),
                    title: note.title.clone(),
                    body: note.body.clone(),
                    html: note.html.clone(),
                    source: VersionSource::Here,
                },
            );
        }
        if versions.len() < 2 {
            self.show_snackbar(tr!("notes-history-none"), None, cx);
            return;
        }
        if let Some(page) = self.notes.as_mut() {
            page.history = Some(History {
                note,
                versions,
                selected: 1,
            });
        }
        cx.notify();
    }

    fn close_note_history(&mut self, cx: &mut Context<Self>) {
        if let Some(page) = self.notes.as_mut() {
            page.history = None;
        }
        cx.notify();
    }

    /// Restore this version: the note gets the shown version's text back,
    /// with Undo.
    fn restore_note_version(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(history) = self.notes.as_mut().and_then(|p| p.history.take()) else {
            return;
        };
        let Some(version) = history.versions.get(history.selected) else {
            return;
        };
        self.close_note(cx);
        let mut restored = history.note.clone();
        restored.title = version.title.clone();
        restored.body = version.body.clone();
        restored.html = version.html.clone();
        let undo = item_of(&history.note);
        self.send(
            Command::SaveNote(Box::new(item_of(&restored))),
            Some(tr!("notes-version-restored")),
            Some(Command::SaveNote(Box::new(undo))),
            false,
            cx,
        );
        self.open_note(Some(&restored), false, None, window, cx);
    }

    /// The history dialog, over the page.
    pub(super) fn render_note_history(
        &self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let history = self.notes.as_ref()?.history.as_ref()?;
        let now = Timestamp::now().as_second();
        let shown = history.versions.get(history.selected)?;
        let before = history.versions.get(history.selected + 1);
        let lines = match before {
            Some(before) => diff(&text_of(before), &text_of(shown)),
            None => text_of(shown)
                .split('\n')
                .map(|l| Line::Same(l.to_owned()))
                .collect(),
        };
        let last = history.versions.len() - 1;
        let size = window.viewport_size();
        let width = (unpx(size.width) - 32.0).min(760.0);
        let height = (unpx(size.height) - 96.0).min(480.0);
        let removed = |t: String| {
            div()
                .px(px(space::S1))
                .rounded(px(radius::XS))
                .bg(rgba(fade(REMOVED, 0.14)))
                .text_color(rgba(th.text_faint))
                .line_through()
                .child(t)
        };
        let added = |t: String| {
            div()
                .px(px(space::S1))
                .rounded(px(radius::XS))
                .bg(rgba(fade(ADDED, 0.18)))
                .child(t)
        };
        let row = |ix: usize, check: Option<bool>| {
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_start()
                .gap(px(space::S3))
                .text_size(px(text::BODY))
                .line_height(px(22.0))
                .when_some(check, |d, done| {
                    d.child(div().mt(px(3.0)).child(crate::widgets::checkbox_colored(
                        ("note-history-check", ix),
                        crate::widgets::Check::from(done),
                        th.text_faint,
                        th,
                    )))
                })
        };
        let strip = |line: &str| -> (Option<bool>, String) {
            match check_of(line) {
                Some((done, rest)) => (Some(done), rest.to_owned()),
                None => (None, line.to_owned()),
            }
        };
        let body = lines.into_iter().enumerate().map(|(ix, line)| match line {
            Line::Same(t) => {
                let (check, t) = strip(&t);
                row(ix, check).child(if t.is_empty() { " ".to_owned() } else { t })
            }
            Line::Removed(t) => {
                let (check, t) = strip(&t);
                row(ix, check).child(removed(t))
            }
            Line::Added(t) => {
                let (check, t) = strip(&t);
                row(ix, check).child(added(t))
            }
            Line::Changed(old, new) => {
                let (check, old) = strip(&old);
                let (_, new) = strip(&new);
                row(ix, check).child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap(px(space::S2))
                        .child(removed(old))
                        .child(added(new)),
                )
            }
        });
        let entries = history.versions.iter().enumerate().map(|(ix, version)| {
            let on = ix == history.selected;
            let (name, detail) = if ix == 0 {
                (tr!("notes-version-now"), tr!("notes-version-here"))
            } else {
                let detail = if ix == last {
                    tr!("notes-version-created")
                } else {
                    match &version.source {
                        VersionSource::Elsewhere(Some(device)) => {
                            tr!("notes-version-from", device = device.clone())
                        }
                        VersionSource::Elsewhere(None) => tr!("notes-version-elsewhere"),
                        VersionSource::Here => {
                            let count = history
                                .versions
                                .get(ix + 1)
                                .map_or(0, |b| changes(&diff(&text_of(b), &text_of(version))));
                            tr!("notes-version-changes", count = count as u64)
                        }
                    }
                };
                (when(version.at, now, &self.tz), detail)
            };
            div()
                .id(("note-version", ix))
                .px(px(space::S3))
                .py(px(space::S2 + space::S1))
                .flex()
                .flex_col()
                .rounded(px(radius::SM))
                .when(on, |d| d.bg(rgba(th.row_selected)))
                .when(!on, |d| d.cursor_pointer().hover(|s| s.bg(rgba(th.hover))))
                .on_click(cx.listener(move |this, _, _, cx| {
                    if let Some(history) = this.notes.as_mut().and_then(|p| p.history.as_mut()) {
                        history.selected = ix;
                    }
                    cx.notify();
                }))
                .child(div().text_size(px(text::BODY)).child(name))
                .child(
                    div()
                        .text_size(px(text::MICRO))
                        .text_color(rgba(th.text_faint))
                        .child(detail),
                )
        });
        let panel = dialog(
            div()
                .id("note-history")
                .occlude()
                .w(px(width))
                .h(px(height))
                .flex()
                .flex_row()
                .text_color(rgba(th.text))
                .on_click(|_, _, cx| cx.stop_propagation()),
            th,
            th.surface,
        )
        .child(
            div()
                .flex_1()
                .min_w_0()
                .h_full()
                .flex()
                .flex_col()
                .child(
                    div()
                        .id("note-history-text")
                        .flex_1()
                        .min_h_0()
                        .overflow_y_scroll()
                        .px(px(space::S6))
                        .pt(px(space::S6 - 2.0))
                        .pb(px(space::S3))
                        .flex()
                        .flex_col()
                        .gap(px(space::S3 - 2.0))
                        .child(
                            div()
                                .text_size(px(text::SUBTITLE + 1.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(shown.title.clone()),
                        )
                        .children(body),
                )
                .child(
                    div()
                        .flex_none()
                        .px(px(space::S5))
                        .py(px(space::S4))
                        .flex()
                        .flex_row()
                        .justify_end()
                        .gap(px(space::S3))
                        .child(
                            outlined_button("note-history-close", tr!("notes-close"), th).on_click(
                                cx.listener(|this, _, _, cx| this.close_note_history(cx)),
                            ),
                        )
                        .when(history.selected != 0, |d| {
                            d.child(
                                filled_button(
                                    "note-history-restore",
                                    tr!("notes-version-restore"),
                                    th,
                                )
                                .on_click(cx.listener(
                                    |this, _, window, cx| this.restore_note_version(window, cx),
                                )),
                            )
                        }),
                ),
        )
        .child(
            div()
                .id("note-history-versions")
                .flex_none()
                .w(px(210.0))
                .h_full()
                .overflow_y_scroll()
                .px(px(space::S3))
                .py(px(space::S4))
                .flex()
                .flex_col()
                .gap(px(space::S1))
                .border_l_1()
                .border_color(rgba(th.divider))
                .child(
                    div()
                        .px(px(space::S3))
                        .pb(px(space::S2))
                        .text_size(px(text::MICRO))
                        .text_color(rgba(th.text_faint))
                        .child(tr!("notes-versions").to_uppercase()),
                )
                .children(entries),
        );
        Some(
            deferred(
                div()
                    .id("note-history-scrim")
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .occlude()
                    .flex()
                    .items_center()
                    .justify_center()
                    .on_click(cx.listener(|this, _, _, cx| this.close_note_history(cx)))
                    .child(panel),
            )
            .with_priority(4)
            .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::{TICKED, UNTICKED};

    #[test]
    fn a_version_says_when() {
        let tz = TimeZone::UTC;
        // Saturday, 3 October 2026, 12:00 UTC.
        let now = 1_791_028_800;
        let time = |at: i64| format::time(crate::format::local(at, &tz).unwrap());
        let morning = now - 3600;
        assert_eq!(when(morning, now, &tz), format!("Today, {}", time(morning)));
        let friday = now - 86_400;
        assert_eq!(
            when(friday, now, &tz),
            format!("Yesterday, {}", time(friday))
        );
    }
    use super::*;

    #[test]
    fn a_rewritten_line_shows_both_and_new_lines_are_added() {
        let old = format!("{UNTICKED}Sunscreen\n{UNTICKED}Phone charger\n{UNTICKED}Tickets");
        let new = format!(
            "{UNTICKED}Sunscreen\n{UNTICKED}Chargers (phone, laptop)\n{UNTICKED}Tickets\n{TICKED}Goggles"
        );
        let lines = diff(&old, &new);
        assert_eq!(
            lines,
            [
                Line::Same(format!("{UNTICKED}Sunscreen")),
                Line::Changed(
                    format!("{UNTICKED}Phone charger"),
                    format!("{UNTICKED}Chargers (phone, laptop)")
                ),
                Line::Same(format!("{UNTICKED}Tickets")),
                Line::Added(format!("{TICKED}Goggles")),
            ]
        );
        assert_eq!(changes(&lines), 2);
    }

    #[test]
    fn removed_lines_stay_removed() {
        assert_eq!(
            diff("a\nb\nc", "a\nc"),
            [
                Line::Same("a".into()),
                Line::Removed("b".into()),
                Line::Same("c".into())
            ]
        );
    }
}
