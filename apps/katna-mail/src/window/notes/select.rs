// SPDX-License-Identifier: GPL-3.0-or-later

//! Several notes at once, as in Keep and the mail list (#598): Ctrl+click
//! and Shift+click tick cards, and a bar in place of "Take a note" pins,
//! reminds, colours, labels, archives or deletes them all. The ⋮ menus of
//! the bar and of an open note are here too.

use crate::widgets::Tip as _;
use std::rc::Rc;

use gpui::{
    AnyElement, Context, MouseButton, Pixels, Point, Window, deferred, div, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::anchored;
use katna_ui::px;
use katna_ui::tokens::{elevation, radius, space, text};

use super::labels::LabelPicker;
use super::{COLORS, MailWindow, NotesView, item_of, note_color};
use crate::daemon::Command;
use crate::theme::Theme;
use crate::widgets::{Check, icon, icon_button, menu_item_icon, raised};

/// A ⋮ menu: the open note's or the ticked cards'.
pub(in crate::window) struct MoreMenu {
    pub at: Point<Pixels>,
    /// For the open note; else for the ticked cards.
    pub open_note: bool,
}

/// A popover of the selection bar.
pub(in crate::window) enum Bulk {
    Colors(Point<Pixels>),
    Labels(Point<Pixels>, LabelPicker),
}

impl MailWindow {
    /// The ticked cards.
    pub(super) fn checked_notes(&self) -> Vec<katna_store::Note> {
        let Some(page) = &self.notes else {
            return Vec::new();
        };
        let Some(Ok(notes)) = &page.notes else {
            return Vec::new();
        };
        page.checked
            .iter()
            .filter_map(|id| notes.iter().find(|n| n.id == *id).cloned())
            .collect()
    }

    /// Ticks card `id`, or unticks it.
    pub(super) fn toggle_note_check(&mut self, id: i64, cx: &mut Context<Self>) {
        self.close_note_now(cx);
        let Some(page) = self.notes.as_mut() else {
            return;
        };
        if let Some(ix) = page.checked.iter().position(|c| *c == id) {
            page.checked.remove(ix);
        } else {
            page.checked.push(id);
        }
        page.check_anchor = Some(id);
        page.bulk = None;
        cx.notify();
    }

    /// Ticks every card from the last one clicked to `id`, in the order
    /// the board shows them.
    pub(super) fn check_note_range(&mut self, id: i64, shown: &[i64], cx: &mut Context<Self>) {
        let anchor = self.notes.as_ref().and_then(|p| p.check_anchor);
        let (Some(from), Some(to)) = (
            anchor.and_then(|a| shown.iter().position(|s| *s == a)),
            shown.iter().position(|s| *s == id),
        ) else {
            self.toggle_note_check(id, cx);
            return;
        };
        self.close_note_now(cx);
        let Some(page) = self.notes.as_mut() else {
            return;
        };
        for &note in &shown[from.min(to)..=from.max(to)] {
            if !page.checked.contains(&note) {
                page.checked.push(note);
            }
        }
        cx.notify();
    }

    /// Unticks every card. Returns whether any was ticked.
    pub(super) fn clear_checks(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(page) = self.notes.as_mut() else {
            return false;
        };
        page.bulk = None;
        page.more = None;
        if page.checked.is_empty() {
            return false;
        }
        page.checked.clear();
        page.check_anchor = None;
        cx.notify();
        true
    }

    /// Saves `change` made to each ticked card, as one step Undo takes
    /// back; `done` says so in the snackbar.
    fn change_checked(
        &mut self,
        done: String,
        clear: bool,
        change: impl Fn(&mut katna_dbus::NoteItem),
        cx: &mut Context<Self>,
    ) {
        let notes = self.checked_notes();
        if notes.is_empty() {
            return;
        }
        let mut undo = Vec::new();
        for note in &notes {
            let mut item = item_of(note);
            undo.push(Command::SaveNote(Box::new(item.clone())));
            change(&mut item);
            self.change_note(item, cx);
        }
        if clear {
            self.clear_checks(cx);
        }
        self.show_snackbar(done, Some(Command::Several(undo)), cx);
    }

    fn pin_checked(&mut self, cx: &mut Context<Self>) {
        let pin = !self.checked_notes().iter().all(|n| n.pinned);
        let count = self.checked_notes().len() as u64;
        let done = if pin {
            tr!("notes-pinned-count", count = count)
        } else {
            tr!("notes-unpinned-count", count = count)
        };
        self.change_checked(
            done,
            false,
            move |item| {
                item.pinned = pin;
                if pin {
                    item.archived = false;
                }
            },
            cx,
        );
    }

    fn color_checked(&mut self, color: i64, cx: &mut Context<Self>) {
        let count = self.checked_notes().len() as u64;
        self.change_checked(
            tr!("notes-colored-count", count = count),
            false,
            move |item| item.color = color,
            cx,
        );
    }

    fn archive_checked(&mut self, cx: &mut Context<Self>) {
        let notes = self.checked_notes();
        let archive = !notes.iter().all(|n| n.archived);
        let count = notes.len() as u64;
        let done = if archive {
            tr!("notes-archived-count", count = count)
        } else {
            tr!("notes-unarchived-count", count = count)
        };
        self.change_checked(
            done,
            true,
            move |item| {
                item.archived = archive;
                item.pinned = false;
            },
            cx,
        );
    }

    fn trash_checked(&mut self, trashed: bool, cx: &mut Context<Self>) {
        let ids: Vec<i64> = self.checked_notes().iter().map(|n| n.id).collect();
        if ids.is_empty() {
            return;
        }
        let count = ids.len() as u64;
        self.clear_checks(cx);
        let done = if trashed {
            tr!("notes-trashed-count", count = count)
        } else {
            tr!("notes-restored-count", count = count)
        };
        self.send(
            Command::TrashNotes(ids.clone(), trashed),
            Some(done),
            Some(Command::TrashNotes(ids, !trashed)),
            false,
            cx,
        );
    }

    /// Makes a copy of each ticked card, on top of the board.
    fn copy_checked(&mut self, cx: &mut Context<Self>) {
        let notes = self.checked_notes();
        let count = notes.len() as u64;
        // The pictures go too.
        let commands: Vec<Command> = notes
            .iter()
            .rev()
            .map(|note| {
                let mut item = item_of(note);
                item.id = 0;
                item.pinned = false;
                let pictures = crate::data::note_pictures(&self.paths, note.id).unwrap_or_default();
                if !pictures.is_empty() {
                    item.pictures_set = true;
                    item.pictures = pictures
                        .iter()
                        .map(super::pictures::item_of_picture)
                        .collect();
                }
                Command::SaveNote(Box::new(item))
            })
            .collect();
        self.clear_checks(cx);
        self.send(
            Command::Several(commands),
            Some(tr!("notes-copied-count", count = count)),
            None,
            false,
            cx,
        );
    }

    /// Puts label `label` on every ticked card, or takes it off when all
    /// have it.
    fn toggle_checked_label(&mut self, label: String, cx: &mut Context<Self>) {
        let notes = self.checked_notes();
        let all = notes.iter().all(|n| n.labels.contains(&label));
        let ids: Vec<i64> = notes.iter().map(|n| n.id).collect();
        let (old, new) = if all {
            (label.clone(), String::new())
        } else {
            (String::new(), label.clone())
        };
        // Shown at once; the store catches up.
        if let Some(Ok(notes)) = self.notes.as_mut().and_then(|p| p.notes.as_mut()) {
            for note in Rc::make_mut(notes)
                .iter_mut()
                .filter(|n| ids.contains(&n.id))
            {
                note.labels.retain(|l| *l != label);
                if !all {
                    note.labels.push(label.clone());
                }
            }
        }
        self.send(
            Command::RelabelNotes(ids.clone(), old.clone(), new.clone()),
            None,
            None,
            false,
            cx,
        );
        self.remember(super::super::UndoStep::Command(Command::RelabelNotes(
            ids, new, old,
        )));
        cx.notify();
    }

    /// Opens the colour or label popover of the selection bar.
    fn open_bulk(
        &mut self,
        labels: bool,
        at: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let accent = self.theme(window).accent;
        let picker = labels.then(|| {
            LabelPicker::new(
                accent,
                window,
                cx,
                |this, label, cx| {
                    let has = this
                        .checked_notes()
                        .iter()
                        .all(|n| n.labels.contains(&label));
                    if !has {
                        this.toggle_checked_label(label, cx);
                    }
                },
                |this, _, cx| {
                    if let Some(page) = this.notes.as_mut() {
                        page.bulk = None;
                    }
                    cx.notify();
                },
            )
        });
        if let Some(page) = self.notes.as_mut() {
            page.more = None;
            page.bulk = Some(match picker {
                Some(picker) => Bulk::Labels(at, picker),
                None => Bulk::Colors(at),
            });
        }
        cx.notify();
    }

    /// The bar in place of "Take a note" while cards are ticked: how many,
    /// then Pin, Remind, Colour, Label, Archive and More; in Trash,
    /// Restore and Delete forever.
    pub(super) fn render_note_select_bar(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let page = self.notes.as_ref()?;
        if page.checked.is_empty() {
            return None;
        }
        let notes = self.checked_notes();
        let count = page.checked.len() as u64;
        let trash = page.view == NotesView::Trash;
        let tool = |id: &'static str, name: &'static str, tip_text: String| {
            icon_button(id, name, 20.0, th)
                .size(px(36.0))
                .tip(tip_text, th)
        };
        let all_pinned = notes.iter().all(|n| n.pinned);
        let all_archived = notes.iter().all(|n| n.archived);
        let ids: Vec<i64> = notes.iter().map(|n| n.id).collect();
        let reminded = notes.iter().any(|n| n.remind_at.is_some());
        let mut bar = super::top_bar("notes-select-bar", th)
            .pl(px(space::S3))
            .pr(px(space::S2))
            .gap(px(space::S1))
            .child(
                tool("notes-select-clear", "close", tr!("notes-select-clear")).on_click(
                    cx.listener(|this, _, _, cx| {
                        this.clear_checks(cx);
                    }),
                ),
            )
            .child(
                div()
                    .flex_1()
                    .pl(px(space::S2))
                    .text_size(px(text::SUBTITLE))
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .child(tr!("notes-selected", count = count)),
            );
        if trash {
            bar = bar
                .child(
                    tool("notes-select-restore", "restore", tr!("notes-restore"))
                        .on_click(cx.listener(|this, _, _, cx| this.trash_checked(false, cx))),
                )
                .child(
                    tool("notes-select-delete", "trash", tr!("notes-delete-forever")).on_click(
                        cx.listener(move |this, _, _, cx| {
                            let ids = ids.clone();
                            this.clear_checks(cx);
                            this.delete_notes_forever(ids, cx)
                        }),
                    ),
                );
        } else {
            bar = bar
                .child(
                    tool(
                        "notes-select-pin",
                        if all_pinned { "pin-filled" } else { "pin" },
                        if all_pinned {
                            tr!("notes-unpin")
                        } else {
                            tr!("notes-pin")
                        },
                    )
                    .on_click(cx.listener(|this, _, _, cx| this.pin_checked(cx))),
                )
                .child(
                    tool("notes-select-remind", "bell", tr!("notes-remind")).on_click(cx.listener(
                        move |this, event: &gpui::ClickEvent, _, cx| {
                            this.open_remind_menu(ids.clone(), reminded, event.position(), cx)
                        },
                    )),
                )
                .child(
                    tool("notes-select-color", "palette", tr!("notes-color")).on_click(
                        cx.listener(|this, event: &gpui::ClickEvent, window, cx| {
                            this.open_bulk(false, event.position(), window, cx)
                        }),
                    ),
                )
                .child(
                    tool("notes-select-label", "label", tr!("notes-labels")).on_click(cx.listener(
                        |this, event: &gpui::ClickEvent, window, cx| {
                            this.open_bulk(true, event.position(), window, cx)
                        },
                    )),
                )
                .child(
                    tool(
                        "notes-select-archive",
                        "archive",
                        if all_archived {
                            tr!("notes-unarchive")
                        } else {
                            tr!("notes-archive")
                        },
                    )
                    .on_click(cx.listener(|this, _, _, cx| this.archive_checked(cx))),
                )
                .child(
                    tool("notes-select-more", "more", tr!("notes-more")).on_click(cx.listener(
                        |this, event: &gpui::ClickEvent, _, cx| {
                            if let Some(page) = this.notes.as_mut() {
                                page.bulk = None;
                                page.more = Some(MoreMenu {
                                    at: event.position(),
                                    open_note: false,
                                });
                            }
                            cx.notify();
                        },
                    )),
                );
        }
        Some(bar.into_any_element())
    }

    /// The open ⋮ menu or bar popover, over a scrim that closes it.
    pub(super) fn render_note_menus(
        &self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let page = self.notes.as_ref()?;
        let (at, panel) = if let Some(more) = &page.more {
            (
                more.at,
                self.render_note_more_menu(more.open_note, th, window, cx),
            )
        } else {
            match page.bulk.as_ref()? {
                Bulk::Colors(at) => (*at, self.render_bulk_colors(th, cx)),
                Bulk::Labels(at, picker) => (*at, self.render_bulk_labels(picker, th, cx)),
            }
        };
        let close = || {
            cx.listener(|this: &mut Self, _: &gpui::MouseDownEvent, _, cx| {
                if let Some(page) = this.notes.as_mut() {
                    page.more = None;
                    page.bulk = None;
                }
                cx.notify();
            })
        };
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .child(
                    deferred(
                        div()
                            .id("notes-menu-scrim")
                            .absolute()
                            .top(px(-2000.0))
                            .left(px(-4000.0))
                            .w(px(8000.0))
                            .h(px(6000.0))
                            .occlude()
                            .on_mouse_down(MouseButton::Left, close())
                            .on_mouse_down(MouseButton::Right, close()),
                    )
                    .with_priority(3),
                )
                .child(
                    deferred(
                        anchored()
                            .position(at)
                            .snap_to_window_with_margin(px(space::S3))
                            .child(div().occlude().child(panel)),
                    )
                    .with_priority(4),
                )
                .into_any_element(),
        )
    }

    fn render_bulk_colors(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let current: Option<i64> = {
            let notes = self.checked_notes();
            let first = notes.first().map(|n| n.color);
            first.filter(|c| notes.iter().all(|n| n.color == *c))
        };
        raised(
            div()
                .p(px(space::S3))
                .flex()
                .flex_row()
                .flex_wrap()
                .w(px(6.0 * 38.0 + 2.0 * space::S3))
                .gap(px(space::S1 * 3.0)),
            th,
            radius::MD,
            elevation::MENU,
        )
        .children((0..=COLORS.len()).map(|ix| {
            let color = ix as i64;
            let fill = note_color(color, th).unwrap_or(th.surface);
            let name = if ix == 0 {
                tr!("notes-color-none")
            } else {
                tr!(COLORS[ix - 1].0)
            };
            div()
                .id(("notes-bulk-color", ix))
                .size(px(32.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .bg(rgba(fill))
                .border_2()
                .border_color(rgba(if Some(color) == current {
                    th.accent
                } else if ix == 0 {
                    th.divider
                } else {
                    0x0000_0000
                }))
                .cursor_pointer()
                .hover(|s| s.border_color(rgba(th.text_dim)))
                .tip(name, th)
                .on_click(cx.listener(move |this, _, _, cx| this.color_checked(color, cx)))
                .when(ix == 0, |d| d.child(icon("close", th.text_dim, 16.0)))
                .when(Some(color) == current && ix != 0, |d| {
                    d.child(icon("check", th.text, 16.0))
                })
        }))
        .into_any_element()
    }

    fn render_bulk_labels(
        &self,
        picker: &LabelPicker,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let notes = self.checked_notes();
        let state = move |label: &str| {
            let with = notes
                .iter()
                .filter(|n| n.labels.iter().any(|l| l == label))
                .count();
            if with == 0 {
                Check::Off
            } else if with == notes.len() {
                Check::On
            } else {
                Check::Partial
            }
        };
        let list = super::labels::render_label_choices(
            "notes-bulk-label",
            tr!("notes-label-note"),
            picker,
            self.note_labels(),
            &state,
            Rc::new(|this: &mut Self, label: String, cx: &mut Context<Self>| {
                this.toggle_checked_label(label, cx)
            }),
            th,
            cx,
        );
        raised(
            div().w(px(260.0)).p(px(space::S3)).flex().flex_col(),
            th,
            radius::MD,
            elevation::MENU,
        )
        .child(list)
        .into_any_element()
    }

    /// The ⋮ menu: the open note's (Show checkboxes, Make a task, Link a
    /// note, Send as mail, Save as Markdown or PDF, Make a copy, Archive,
    /// Delete) or the ticked cards' (Make a copy, Delete).
    fn render_note_more_menu(
        &self,
        open_note: bool,
        th: &Theme,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let item = |id: &'static str, name: &'static str, label: String| {
            menu_item_icon(id, name, &label, th)
        };
        let close_then = |f: fn(&mut Self, &mut Window, &mut Context<Self>)| {
            cx.listener(move |this: &mut Self, _: &gpui::ClickEvent, window, cx| {
                if let Some(page) = this.notes.as_mut() {
                    page.more = None;
                }
                f(this, window, cx);
                cx.notify();
            })
        };
        let mut menu = raised(
            div()
                .key_context(crate::widgets::MENU_CONTEXT)
                .min_w(px(220.0))
                .py(px(space::S3))
                .flex()
                .flex_col()
                .text_size(px(text::BODY))
                .text_color(rgba(th.text)),
            th,
            radius::SM,
            elevation::MENU,
        );
        if !open_note {
            menu = menu
                .child(
                    item("notes-more-copy", "copy", tr!("notes-make-copy"))
                        .on_click(close_then(|this, _, cx| this.copy_checked(cx))),
                )
                .child(
                    item("notes-more-delete", "trash", tr!("notes-delete"))
                        .on_click(close_then(|this, _, cx| this.trash_checked(true, cx))),
                );
            return menu.into_any_element();
        }
        let editor = self.notes.as_ref().and_then(|p| p.editor.as_ref());
        let saved = editor.is_some_and(|e| e.id != 0);
        let archived = editor.is_some_and(|e| e.archived);
        menu = menu
            .child(
                item(
                    "notes-more-checkboxes",
                    "checkbox-checked",
                    tr!("notes-checkboxes"),
                )
                .on_click(close_then(|this, window, cx| {
                    this.toggle_open_checklist(window, cx)
                })),
            )
            .when(self.task_line(cx).is_some(), |m| {
                m.child(
                    item("notes-more-task", "tasks", tr!("notes-make-task")).on_click(close_then(
                        |this, window, cx| this.make_line_task(window, cx),
                    )),
                )
            })
            .child(
                item("notes-more-link", "link", tr!("notes-link-note")).on_click(close_then(
                    |this, window, cx| this.start_note_link(window, cx),
                )),
            )
            .child(div().my(px(space::S2)).h(px(1.0)).bg(rgba(th.divider)))
            .child(
                item("notes-more-send", "send", tr!("notes-send-as-mail")).on_click(close_then(
                    |this, window, cx| this.send_note_as_mail(window, cx),
                )),
            )
            .child(
                item(
                    "notes-more-markdown",
                    "document",
                    tr!("notes-save-markdown"),
                )
                .on_click(close_then(|this, _, cx| this.save_note_as_markdown(cx))),
            )
            .child(
                item("notes-more-pdf", "print", tr!("notes-save-pdf"))
                    .on_click(close_then(|this, _, cx| this.save_note_as_pdf(cx))),
            );
        if saved {
            menu = menu
                .child(div().my(px(space::S2)).h(px(1.0)).bg(rgba(th.divider)))
                .child(
                    item("notes-more-copy", "copy", tr!("notes-make-copy"))
                        .on_click(close_then(|this, _, cx| this.copy_open_note(cx))),
                )
                .child(
                    item(
                        "notes-more-archive",
                        "archive",
                        if archived {
                            tr!("notes-unarchive")
                        } else {
                            tr!("notes-archive")
                        },
                    )
                    .on_click(close_then(|this, _, cx| this.archive_open_note(cx))),
                )
                .child(
                    item("notes-more-delete", "trash", tr!("notes-delete")).on_click(close_then(
                        |this, _, cx| {
                            if let Some(id) = this
                                .notes
                                .as_ref()
                                .and_then(|p| p.editor.as_ref())
                                .map(|e| e.id)
                            {
                                this.trash_note(id, true, cx)
                            }
                        },
                    )),
                );
        }
        menu.into_any_element()
    }
}
