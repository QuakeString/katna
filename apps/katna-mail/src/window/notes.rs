// SPDX-License-Identifier: GPL-3.0-or-later

//! The Notes page (`docs/ARCHITECTURE.md` §13.11), after Google Keep: a
//! "Take a note…" bar, a board of cards in soft colors with pinned notes
//! first, and a note that opens as a card over the board. A note saves
//! itself as it is typed; lines starting "☐ " or "☑ " are a checklist,
//! the way Apple's Notes folder keeps them. Archive and Trash sit in the
//! list at the left; Trash empties itself after seven days.

mod format;
mod labels;
mod line_tasks;
mod meetings;

pub(super) use line_tasks::note_of_task;

use std::rc::Rc;
use std::time::Duration;

use gpui::{
    AnimationExt, AnyElement, Context, Div, ElementId, Entity, Focusable, FontWeight, SharedString,
    Subscription, Task, Window, div, prelude::*, rgba,
};
use katna_dbus::NoteItem;
use katna_i18n::tr;
use katna_store::Note;
use katna_ui::rich::{RichEditor, RichEvent};
use katna_ui::{InputEvent, TextInput, px, unpx};

use super::MailWindow;
use crate::daemon::{self, Command};
use crate::data::EntryKey;
use crate::theme::{Theme, fade};
use crate::widgets::{elevation, icon, icon_button, icon_button_colored, placeholder, tip};

/// A card's width on the board, as Keep's.
const CARD_WIDTH: f32 = 240.0;
/// The narrowest a card gets, two across a phone.
const NARROW_CARD_WIDTH: f32 = 140.0;
/// Room between cards.
const GAP: f32 = 16.0;
/// The side list's width.
const SIDE_WIDTH: f32 = 240.0;
/// The open note's width.
const EDITOR_WIDTH: f32 = 600.0;
/// How long typing rests before the note is saved.
const SAVE_DELAY: Duration = Duration::from_millis(700);
/// Lines of a note's text a card shows.
const CARD_LINES: usize = 12;

/// A checklist line's marks, as Apple's Notes folder has them.
const UNTICKED: &str = "☐ ";
const TICKED: &str = "☑ ";

/// Keep's note colors (the 2023 palette), light and dark; a note's
/// `color` is 1 + an index here.
const COLORS: [(&str, u32, u32); 11] = [
    ("notes-color-coral", 0xfaafa8ff, 0x77172eff),
    ("notes-color-peach", 0xf39f76ff, 0x692b17ff),
    ("notes-color-sand", 0xfff8b8ff, 0x7c4a03ff),
    ("notes-color-mint", 0xe2f6d3ff, 0x264d3bff),
    ("notes-color-sage", 0xb4ddd3ff, 0x0c625dff),
    ("notes-color-fog", 0xd4e4edff, 0x256377ff),
    ("notes-color-storm", 0xaeccdcff, 0x284255ff),
    ("notes-color-dusk", 0xd3bfdbff, 0x472e5bff),
    ("notes-color-blossom", 0xf6e2ddff, 0x6c394fff),
    ("notes-color-clay", 0xe9e3d4ff, 0x4b443aff),
    ("notes-color-chalk", 0xefeff1ff, 0x232427ff),
];

/// A note's background, or `None` for the plain surface.
fn note_color(color: i64, th: &Theme) -> Option<u32> {
    let ix = usize::try_from(color).ok()?.checked_sub(1)?;
    COLORS
        .get(ix)
        .map(|&(_, light, dark)| if th.dark { dark } else { light })
}

/// Which notes the board shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NotesView {
    Notes,
    Archive,
    Trash,
}

impl NotesView {
    const ALL: [Self; 3] = [Self::Notes, Self::Archive, Self::Trash];

    fn label(self) -> String {
        tr!(match self {
            Self::Notes => "notes-view-notes",
            Self::Archive => "notes-view-archive",
            Self::Trash => "notes-view-trash",
        })
    }

    fn icon(self) -> &'static str {
        match self {
            Self::Notes => "notes",
            Self::Archive => "archive",
            Self::Trash => "trash",
        }
    }

    fn shows(self, note: &Note) -> bool {
        match self {
            Self::Notes => note.trashed_at.is_none() && !note.archived,
            Self::Archive => note.trashed_at.is_none() && note.archived,
            Self::Trash => note.trashed_at.is_some(),
        }
    }
}

/// The Notes page's state, made the first time the page opens.
pub(super) struct NotesPage {
    notes: Option<Result<Rc<Vec<Note>>, String>>,
    view: NotesView,
    /// The top bar's search box searches notes; this is the mail search
    /// it had, while the page is open.
    mail_query: Option<String>,
    /// The label the board is showing notes of, over Notes.
    label: Option<String>,
    labels_dialog: Option<labels::LabelsDialog>,
    editor: Option<Editor>,
    _load: Option<Task<()>>,
}

/// The note open over the board.
struct Editor {
    /// 0 until the new note is first saved.
    id: i64,
    title: Entity<TextInput>,
    body: Entity<RichEditor>,
    color: i64,
    pinned: bool,
    archived: bool,
    labels: Vec<String>,
    link: Option<String>,
    account: Option<i64>,
    updated_at: i64,
    /// Typed since the last save.
    changed: bool,
    /// The color row is open.
    palette: bool,
    /// The account row is open.
    places: bool,
    /// The formatting row is open.
    format: bool,
    /// A new note from the "Take a note" bar, shown in the bar's place.
    in_bar: bool,
    /// The label picker, when open.
    picker: Option<labels::Picker>,
    /// A save is on its way; another waits for it.
    saving: bool,
    _save: Option<Task<()>>,
    _subscriptions: Vec<Subscription>,
}

impl Editor {
    fn item(&self, cx: &gpui::App) -> NoteItem {
        NoteItem {
            id: self.id,
            account: self.account.unwrap_or(0),
            title: self.title.read(cx).text().to_owned(),
            body: format::text_of(self.body.read(cx).doc()),
            html: format::html_of(self.body.read(cx).doc()),
            color: self.color,
            pinned: self.pinned,
            archived: self.archived,
            labels: self.labels.clone(),
            link: self.link.clone().unwrap_or_default(),
        }
    }

    fn is_empty(&self, cx: &gpui::App) -> bool {
        self.title.read(cx).text().trim().is_empty() && self.body.read(cx).is_blank()
    }
}

/// `note` as the daemon takes it.
fn item_of(note: &Note) -> NoteItem {
    NoteItem {
        id: note.id,
        account: note.account_id.unwrap_or(0),
        title: note.title.clone(),
        body: note.body.clone(),
        color: note.color,
        pinned: note.pinned,
        archived: note.archived,
        labels: note.labels.clone(),
        link: note.link.clone().unwrap_or_default(),
        html: note.html.clone(),
    }
}

/// Whether line `line` is a checklist item, and whether it is ticked.
fn check_of(line: &str) -> Option<(bool, &str)> {
    if let Some(rest) = line.strip_prefix(TICKED) {
        Some((true, rest))
    } else {
        line.strip_prefix(UNTICKED).map(|rest| (false, rest))
    }
}

/// `body` with line `ix` ticked or unticked.
fn toggle_line(body: &str, ix: usize) -> String {
    body.split('\n')
        .enumerate()
        .map(|(i, line)| {
            if i != ix {
                return line.to_owned();
            }
            match check_of(line) {
                Some((true, rest)) => format!("{UNTICKED}{rest}"),
                Some((false, rest)) => format!("{TICKED}{rest}"),
                None => line.to_owned(),
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// `body` as a checklist, or back to plain lines when it is one already
/// (Keep's "Show checkboxes" and "Hide checkboxes").
fn toggle_checklist(body: &str) -> String {
    let lines: Vec<&str> = body.split('\n').collect();
    let is_list = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .all(|l| check_of(l).is_some())
        && lines.iter().any(|l| check_of(l).is_some());
    lines
        .iter()
        .map(|line| match (is_list, check_of(line)) {
            (true, Some((_, rest))) => rest.to_owned(),
            (true, None) => (*line).to_owned(),
            (false, Some(_)) => (*line).to_owned(),
            (false, None) if line.trim().is_empty() && lines.len() > 1 => (*line).to_owned(),
            (false, None) => format!("{UNTICKED}{line}"),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Whether `note` has every word of `query` (lowercased) in its title,
/// text or labels.
fn matches(note: &Note, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let hay = format!("{}\n{}\n{}", note.title, note.body, note.labels.join("\n")).to_lowercase();
    query.split_whitespace().all(|word| hay.contains(word))
}

/// A rough height for `note`'s card, to place it in the shortest column.
fn card_height(note: &Note) -> f32 {
    let lines = note
        .body
        .split('\n')
        .map(|l| 1 + l.chars().count() / 28)
        .sum::<usize>()
        .min(CARD_LINES);
    let title = if note.title.is_empty() { 0.0 } else { 28.0 };
    let mail = if note.link.is_some() { 32.0 } else { 0.0 };
    let labels = if note.labels.is_empty() { 0.0 } else { 32.0 };
    32.0 + title + mail + labels + 20.0 * lines as f32
}

/// Cards laid out in `columns` columns, each going to the shortest one,
/// as Keep lays out its board.
fn masonry<'a>(notes: &[&'a Note], columns: usize) -> Vec<Vec<&'a Note>> {
    let columns = columns.max(1);
    let mut out: Vec<Vec<&Note>> = vec![Vec::new(); columns];
    let mut heights = vec![0.0_f32; columns];
    for note in notes {
        let (ix, _) = heights
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.total_cmp(b.1))
            .unwrap_or((0, &0.0));
        heights[ix] += card_height(note) + GAP;
        out[ix].push(note);
    }
    out
}

impl MailWindow {
    /// The Notes page, made and loaded the first time.
    pub(super) fn notes_page(&mut self, cx: &mut Context<Self>) {
        if self.notes.is_some() {
            return;
        }
        self.notes = Some(NotesPage {
            notes: None,
            view: NotesView::Notes,
            mail_query: None,
            label: None,
            labels_dialog: None,
            editor: None,
            _load: None,
        });
        self.load_notes(cx);
    }

    /// Reads the notes from the store again.
    pub(super) fn load_notes(&mut self, cx: &mut Context<Self>) {
        let Some(page) = self.notes.as_mut() else {
            return;
        };
        let paths = self.paths.clone();
        page._load = Some(cx.spawn(async move |this, cx| {
            let notes = cx
                .background_executor()
                .spawn(async move { crate::data::notes(&paths) })
                .await;
            this.update(cx, |this, cx| {
                if let Some(page) = &mut this.notes {
                    // A label no note has any more (renamed, deleted or
                    // undone) leaves its board for Notes.
                    if let (Some(label), Ok(notes)) = (&page.label, &notes)
                        && !notes.iter().any(|n| n.labels.contains(label))
                    {
                        page.label = None;
                    }
                    page.notes = Some(notes.map(Rc::new));
                }
                cx.notify();
            })
            .ok();
        }));
    }

    /// Opens `note` over the board, or a new one for `None`; `checklist`
    /// starts a new one as a list, and `about` (a subject and a
    /// `Message-ID`) one about a mail, titled with its subject.
    fn open_note(
        &mut self,
        note: Option<&Note>,
        checklist: bool,
        about: Option<(String, String)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.close_note(cx);
        // An event's card would sit over the note.
        self.calendar.open = None;
        let accent = rgba(self.theme(window).accent).into();
        let (about_title, about_link) = about.unzip();
        let in_bar = note.is_none() && about_link.is_none();
        let (title_text, (body_text, body_html)) = note
            .map(|n| (n.title.clone(), (n.body.clone(), n.html.clone())))
            .unwrap_or_else(|| {
                (
                    about_title.clone().unwrap_or_default(),
                    (
                        if checklist {
                            UNTICKED.to_owned()
                        } else {
                            String::new()
                        },
                        String::new(),
                    ),
                )
            });
        let title = cx.new(|cx| {
            let mut input = TextInput::new(tr!("notes-title"), cx);
            input.set_accent(accent);
            input.set_text(title_text, cx);
            input
        });
        let palette = super::compose::palette(&self.theme(window));
        let body = cx.new(|cx| {
            let mut area = RichEditor::new(tr!("notes-take-a-note"), cx);
            area.set_palette(palette);
            let doc = format::doc_of(&body_text, &body_html);
            let end = doc.end();
            area.set_doc(doc, end, cx);
            area
        });
        let on_title =
            cx.subscribe_in(&title, window, |this, _, event: &InputEvent, window, cx| {
                match event {
                    InputEvent::Changed => this.note_typed(cx),
                    // Enter in the title goes on to the text, as in Keep.
                    InputEvent::Submit => {
                        if let Some(editor) = &this.notes.as_ref().and_then(|p| p.editor.as_ref()) {
                            let focus = editor.body.focus_handle(cx);
                            window.focus(&focus, cx);
                        }
                    }
                    InputEvent::Cancel => this.close_note(cx),
                }
            });
        let on_body = cx.subscribe(&body, |this, _, event: &RichEvent, cx| match event {
            RichEvent::Changed => this.note_typed(cx),
            RichEvent::Cancel => this.close_note(cx),
            // Make it a task and the formatting row follow the cursor.
            RichEvent::Selection => cx.notify(),
            _ => {}
        });
        let focus = if note.is_some() || checklist || about_title.is_some() {
            body.focus_handle(cx)
        } else {
            title.focus_handle(cx)
        };
        window.focus(&focus, cx);
        let view = self.notes.as_ref().map_or(NotesView::Notes, |p| p.view);
        let editor = Editor {
            id: note.map_or(0, |n| n.id),
            title,
            body,
            color: note.map_or(0, |n| n.color),
            pinned: note.is_some_and(|n| n.pinned),
            archived: note.map_or(view == NotesView::Archive, |n| n.archived),
            // A new note in a label's board has that label, as in Keep.
            labels: match note {
                Some(n) => n.labels.clone(),
                None => self
                    .notes
                    .as_ref()
                    .and_then(|p| p.label.clone())
                    .into_iter()
                    .collect(),
            },
            link: note.and_then(|n| n.link.clone()).or(about_link),
            // A new note goes to the account whose mail was open, as a new
            // message comes from it; the picker on the note changes it.
            account: match note {
                Some(n) => n.account_id,
                None => self.notes_account(),
            },
            updated_at: note.map_or(0, |n| n.updated_at),
            changed: false,
            palette: false,
            places: false,
            format: false,
            in_bar,
            picker: None,
            saving: false,
            _save: None,
            _subscriptions: vec![on_title, on_body],
        };
        if let Some(page) = &mut self.notes {
            page.editor = Some(editor);
        }
        cx.notify();
    }

    fn note_typed(&mut self, cx: &mut Context<Self>) {
        let Some(editor) = self.notes.as_mut().and_then(|p| p.editor.as_mut()) else {
            return;
        };
        editor.changed = true;
        editor._save = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(SAVE_DELAY).await;
            this.update(cx, |this, cx| this.save_open_note(cx)).ok();
        }));
        cx.notify();
    }

    /// Saves the open note if it changed and is not empty.
    fn save_open_note(&mut self, cx: &mut Context<Self>) {
        let Some(editor) = self.notes.as_mut().and_then(|p| p.editor.as_mut()) else {
            return;
        };
        if !editor.changed || editor.saving || (editor.id == 0 && editor.is_empty(cx)) {
            return;
        }
        editor.changed = false;
        editor.saving = true;
        let item = editor.item(cx);
        let sent = item.clone();
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::save_note(&connection, &sent).await
                })
                .await;
            this.update(cx, |this, cx| {
                let again = match this.notes.as_mut().and_then(|p| p.editor.as_mut()) {
                    Some(editor) if editor.id == item.id => {
                        editor.saving = false;
                        if let Ok(id) = result {
                            editor.id = id;
                            editor.updated_at = jiff::Timestamp::now().as_second();
                        }
                        editor.changed
                    }
                    _ => false,
                };
                if let Err(err) = result {
                    this.show_snackbar(err, None, cx);
                }
                if again {
                    this.save_open_note(cx);
                }
                this.load_notes(cx);
            })
            .ok();
        })
        .detach();
    }

    /// Puts the open note back on the board, saved; an empty new one is
    /// dropped.
    pub(super) fn close_note(&mut self, cx: &mut Context<Self>) {
        let Some(editor) = self.notes.as_mut().and_then(|p| p.editor.take()) else {
            return;
        };
        if editor.changed && !(editor.id == 0 && editor.is_empty(cx)) {
            let item = editor.item(cx);
            if editor.saving && editor.id == 0 {
                // Its first save is still on the way: it will have an ID
                // only then, so send this one after it.
                self.send_note_later(item, cx);
            } else {
                self.send(Command::SaveNote(Box::new(item)), None, None, false, cx);
            }
        } else if editor.id == 0 && editor.is_empty(cx) && editor.changed {
            self.show_snackbar(tr!("notes-empty-discarded"), None, cx);
        }
        cx.notify();
    }

    /// Saves `item`, a new note being saved for the first time, once that
    /// save is in and its ID known.
    fn send_note_later(&mut self, mut item: NoteItem, cx: &mut Context<Self>) {
        let created_after = jiff::Timestamp::now().as_second() - 5;
        cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(1500))
                .await;
            this.update(cx, |this, cx| {
                let id = this
                    .notes
                    .as_ref()
                    .and_then(|p| p.notes.as_ref())
                    .and_then(|n| n.as_ref().ok())
                    .and_then(|notes| {
                        notes
                            .iter()
                            .filter(|n| n.created_at >= created_after)
                            .max_by_key(|n| n.id)
                            .map(|n| n.id)
                    });
                if let Some(id) = id {
                    item.id = id;
                    this.send(Command::SaveNote(Box::new(item)), None, None, false, cx);
                }
            })
            .ok();
        })
        .detach();
    }

    /// Changes something of the note on a card (or the open one) at once:
    /// a color, the pin, a checklist tick.
    fn change_note(&mut self, item: NoteItem, cx: &mut Context<Self>) {
        if let Some(editor) = self.notes.as_mut().and_then(|p| p.editor.as_mut())
            && editor.id == item.id
            && item.id != 0
        {
            editor.color = item.color;
            editor.account = (item.account != 0).then_some(item.account);
            editor.pinned = item.pinned;
            editor.archived = item.archived;
            editor.labels = item.labels.clone();
        }
        // Shown at once; the store catches up.
        if let Some(Ok(notes)) = self.notes.as_mut().and_then(|p| p.notes.as_mut())
            && let Some(note) = Rc::make_mut(notes).iter_mut().find(|n| n.id == item.id)
        {
            note.color = item.color;
            note.pinned = item.pinned;
            note.archived = item.archived;
            note.body = item.body.clone();
            note.labels = item.labels.clone();
        }
        self.send(Command::SaveNote(Box::new(item)), None, None, false, cx);
        cx.notify();
    }

    fn archive_note(&mut self, note: &Note, archived: bool, cx: &mut Context<Self>) {
        let mut item = item_of(note);
        item.archived = archived;
        item.pinned = false;
        let mut undo = item_of(note);
        undo.archived = !archived;
        let done = if archived {
            tr!("notes-archived")
        } else {
            tr!("notes-unarchived")
        };
        self.close_open(note.id, cx);
        self.send(
            Command::SaveNote(Box::new(item)),
            Some(done),
            Some(Command::SaveNote(Box::new(undo))),
            false,
            cx,
        );
    }

    fn trash_note(&mut self, id: i64, trashed: bool, cx: &mut Context<Self>) {
        self.close_open(id, cx);
        let done = if trashed {
            tr!("notes-trashed")
        } else {
            tr!("notes-restored")
        };
        self.send(
            Command::TrashNotes(vec![id], trashed),
            Some(done),
            Some(Command::TrashNotes(vec![id], !trashed)),
            false,
            cx,
        );
    }

    fn delete_notes_forever(&mut self, ids: Vec<i64>, cx: &mut Context<Self>) {
        if ids.is_empty() {
            return;
        }
        let count = ids.len() as u64;
        self.remember(super::UndoStep::DeletedForever);
        self.send(
            Command::DeleteNotes(ids),
            Some(tr!("notes-deleted-forever", count = count)),
            None,
            false,
            cx,
        );
    }

    /// Closes the open note first when it is `id`, saving it.
    fn close_open(&mut self, id: i64, cx: &mut Context<Self>) {
        let open = self
            .notes
            .as_ref()
            .and_then(|p| p.editor.as_ref())
            .is_some_and(|e| e.id == id);
        if open {
            self.close_note(cx);
        }
    }

    /// Add a note (the mail's ⋮ and right-click menus): a new note about
    /// the first picked conversation, titled with its subject and keeping
    /// its newest message's `Message-ID`, opened over the mail.
    pub(super) fn add_note_from(
        &mut self,
        keys: Vec<EntryKey>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((subject, header)) = keys
            .first()
            .and_then(|key| self.mail.as_ref().ok()?.task_source(*key))
        else {
            return;
        };
        self.notes_page(cx);
        self.open_note(None, false, Some((subject, header)), window, cx);
    }

    /// Opens the mail a note is about.
    fn open_note_mail(&mut self, header: &str, window: &mut Window, cx: &mut Context<Self>) {
        let found = self
            .mail
            .as_ref()
            .ok()
            .and_then(|m| m.message_with_header(header));
        match found {
            Some(message) => {
                self.close_note(cx);
                self.open_app(super::apps::App::Mail, cx);
                self.show_message(message, window, cx);
            }
            None => self.show_snackbar(tr!("notes-mail-gone"), None, cx),
        }
    }

    /// The chip of a note about a mail ("Mail") or an event ("Event"),
    /// which opens it.
    fn mail_chip(
        &self,
        id: impl Into<ElementId>,
        link: String,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> gpui::Stateful<Div> {
        let event = meetings::event_start(&link);
        div()
            .id(id)
            .mt(px(8.0))
            .h(px(24.0))
            .pl(px(6.0))
            .pr(px(10.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .rounded_full()
            .bg(rgba(fade(th.text, 0.08)))
            .text_size(px(12.0))
            .text_color(rgba(th.text))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(fade(th.text, 0.14))))
            .tooltip(tip(
                if event.is_some() {
                    tr!("notes-open-event")
                } else {
                    tr!("notes-open-mail")
                },
                th,
            ))
            .on_click(cx.listener(move |this, _, window, cx| {
                cx.stop_propagation();
                match event {
                    Some(start) => this.open_note_event(start, cx),
                    None => this.open_note_mail(&link, window, cx),
                }
            }))
            .child(icon(
                if event.is_some() { "event" } else { "mail" },
                th.text_dim,
                16.0,
            ))
            .child(if event.is_some() {
                tr!("notes-event")
            } else {
                tr!("notes-mail")
            })
    }

    /// The notes about an open conversation (its messages' `Message-ID`s
    /// are `headers`), shown under its subject, as Keep shows them beside
    /// Gmail; each opens over the mail.
    pub(super) fn render_mail_notes(
        &self,
        headers: &[String],
        indent: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let notes = self.notes.as_ref()?.notes.as_ref()?.as_ref().ok()?;
        let about: Vec<&Note> = notes
            .iter()
            .filter(|n| {
                n.trashed_at.is_none() && n.link.as_ref().is_some_and(|l| headers.contains(l))
            })
            .collect();
        if about.is_empty() {
            return None;
        }
        let header = headers.last().cloned().unwrap_or_default();
        Some(
            div()
                .pl(px(indent))
                .pr(px(16.0))
                .pb(px(12.0))
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(8.0))
                .children(about.into_iter().map(|note| self.linked_note(note, th, cx)))
                .child(
                    div()
                        .id("mail-note-add")
                        .h(px(36.0))
                        .px(px(12.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(6.0))
                        .rounded(px(8.0))
                        .text_size(px(13.0))
                        .text_color(rgba(th.text_dim))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(th.hover)))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            let subject = this
                                .reader
                                .as_ref()
                                .map(|r| r.subject().to_owned())
                                .unwrap_or_default();
                            this.open_note(None, false, Some((subject, header.clone())), window, cx)
                        }))
                        .child(icon("add", th.text_dim, 18.0))
                        .child(tr!("menu-add-note")),
                )
                .into_any_element(),
        )
    }

    /// A small card of a note about a mail or an event, shown there; it
    /// opens over it.
    pub(super) fn linked_note(
        &self,
        note: &Note,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> gpui::Stateful<Div> {
        let open = Rc::new(note.clone());
        let bg = note_color(note.color, th);
        let first = note.body.lines().find(|l| !l.trim().is_empty());
        let heading = if note.title.is_empty() {
            first.unwrap_or_default().to_owned()
        } else {
            note.title.clone()
        };
        let text = (!note.title.is_empty()).then(|| first.unwrap_or_default().to_owned());
        div()
            .id(("mail-note", note.id as usize))
            .w(px(220.0))
            .px(px(12.0))
            .py(px(8.0))
            .flex()
            .flex_row()
            .gap(px(8.0))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgba(if bg.is_some() { 0x00000000 } else { th.divider }))
            .bg(rgba(bg.unwrap_or(th.surface)))
            .cursor_pointer()
            .hover(|s| s.shadow(elevation(th, 1.0)))
            .tooltip(tip(tr!("notes-open-note"), th))
            .on_click(cx.listener(move |this, _, window, cx| {
                this.open_note(Some(&open), false, None, window, cx)
            }))
            .child(div().pt(px(2.0)).child(icon("notes", th.text_dim, 16.0)))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .truncate()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.text))
                            .child(heading),
                    )
                    .children(text.filter(|t| !t.is_empty()).map(|t| {
                        div()
                            .truncate()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_dim))
                            .child(t)
                    })),
            )
    }

    /// Where a new note goes: the account whose mail is open, if its
    /// server keeps folders, else the first that does.
    fn notes_account(&self) -> Option<i64> {
        let imap = |id: katna_core::AccountId| {
            self.accounts
                .iter()
                .any(|a| a.id == id && a.kind == katna_core::AccountKind::Imap)
        };
        self.account()
            .filter(|id| imap(*id))
            .or_else(|| {
                self.accounts
                    .iter()
                    .find(|a| a.kind == katna_core::AccountKind::Imap)
                    .map(|a| a.id)
            })
            .map(|id| id.0)
    }

    /// The name of where a note is kept.
    fn note_place(&self, account: Option<i64>) -> String {
        account
            .and_then(|id| self.accounts.iter().find(|a| a.id.0 == id))
            .map_or_else(|| tr!("notes-on-this-computer"), |a| a.display_name.clone())
    }

    /// The top bar's search box searches notes while the Notes page is
    /// open, and mail again once it closes. Call after the app changes.
    pub(super) fn sync_notes_search(&mut self, cx: &mut Context<Self>) {
        let on = self.app == super::apps::App::Notes && self.settings_page.is_none();
        if on {
            self.notes_page(cx);
        }
        let Some(page) = self.notes.as_mut() else {
            return;
        };
        if on == page.mail_query.is_some() {
            return;
        }
        if on {
            page.mail_query = Some(self.search.read(cx).text().to_owned());
            self.search.update(cx, |search, cx| {
                search.set_placeholder(tr!("notes-search"));
                search.set_text("", cx);
            });
        } else {
            self.close_note(cx);
            // The mail search comes back, as the other pages expect.
            let query = self.notes.as_mut().and_then(|p| p.mail_query.take());
            self.search.update(cx, |search, cx| {
                search.set_placeholder(tr!("search-mail"));
                search.set_text(query.unwrap_or_default(), cx);
            });
        }
    }

    /// The top bar's search box changed while it searches notes.
    pub(super) fn on_notes_search(
        &mut self,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Cancel => {
                if self.search.read(cx).text().is_empty() {
                    window.blur(cx);
                } else {
                    self.search.update(cx, |search, cx| search.set_text("", cx));
                }
            }
            InputEvent::Changed | InputEvent::Submit => {}
        }
        cx.notify();
    }

    /// The Notes page.
    pub(super) fn render_notes(
        &mut self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        self.notes_page(cx);
        let Some(page) = &self.notes else {
            return placeholder(&tr!("notes-loading"), th);
        };
        let view = page.view;
        let label = page.label.clone();
        let mut side_labels = self.render_side_labels(th, cx);
        // Keys (Ctrl+Z) keep working when the open note closes, and find
        // nothing on a focused mail pane the page hides.
        let hidden = [&self.list_focus, &self.reader_focus, &self.nav_focus];
        let lost = window
            .focused(cx)
            .is_none_or(|focused| hidden.contains(&&focused));
        if page.editor.is_none() && page.labels_dialog.is_none() && lost {
            window.focus(&self.window_focus, cx);
        }
        let side = div()
            .flex_none()
            .w(px(SIDE_WIDTH))
            .h_full()
            .pt(px(8.0))
            .pr(px(12.0))
            .flex()
            .flex_col()
            .children(NotesView::ALL.into_iter().flat_map(|v| {
                let on = v == view && label.is_none();
                let entry = div()
                    .id(("notes-view", v as usize))
                    .h(px(48.0))
                    .pl(px(24.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(20.0))
                    .rounded_r(px(24.0))
                    .cursor_pointer()
                    .text_size(px(14.0))
                    .font_weight(if on {
                        FontWeight::BOLD
                    } else {
                        FontWeight::MEDIUM
                    })
                    .text_color(rgba(if on { th.nav_selected_text } else { th.text }))
                    .when(on, |d| d.bg(rgba(th.nav_selected)))
                    .when(!on, |d| d.hover(|s| s.bg(rgba(th.hover))))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.close_note(cx);
                        if let Some(page) = &mut this.notes {
                            page.view = v;
                            page.label = None;
                        }
                        cx.notify();
                    }))
                    .child(icon(
                        v.icon(),
                        if on {
                            th.nav_selected_text
                        } else {
                            th.text_dim
                        },
                        22.0,
                    ))
                    .child(v.label())
                    .into_any_element();
                let labels = if v == NotesView::Notes {
                    std::mem::take(&mut side_labels)
                } else {
                    Vec::new()
                };
                std::iter::once(entry).chain(labels)
            }));

        let query = self.search.read(cx).text().trim().to_lowercase();
        let board = self.render_board(th, &query, window, cx);
        // A new note opens in place of the "Take a note" bar, as in Keep;
        // a saved one opens over the page.
        let editor = if self.new_note_inline() {
            None
        } else {
            self.render_editor(th, false, window, cx)
        };
        let dialog = self.render_labels_dialog(th, window, cx);
        let side = self.page_side(side.into_any_element(), SIDE_WIDTH, true, th, cx);
        div()
            .relative()
            .size_full()
            .flex()
            .flex_row()
            .children(side.docked)
            .child(board)
            .children(side.drawer)
            .children(editor)
            .children(dialog)
            .into_any_element()
    }

    fn render_board(
        &self,
        th: &Theme,
        query: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(page) = &self.notes else {
            return div().into_any_element();
        };
        let view = page.view;
        let label = page.label.clone();
        let notes = match &page.notes {
            None => return placeholder(&tr!("notes-loading"), th),
            Some(Err(err)) => return placeholder(err, th),
            Some(Ok(notes)) => notes.clone(),
        };
        // A new note being written in the bar's place joins the board when
        // it closes, as in Keep.
        let writing = page.editor.as_ref().filter(|e| e.in_bar).map(|e| e.id);
        let shown: Vec<&Note> = notes
            .iter()
            .filter(|n| {
                Some(n.id) != writing
                    && view.shows(n)
                    && matches(n, query)
                    && label.as_ref().is_none_or(|l| n.labels.contains(l))
            })
            .collect();
        // The board's width: the window less the rail, the side list (a
        // drawer on a phone or tablet), the page's margin and its own.
        let shape = self.layout.shape;
        let pad = if shape.is_phone() { GAP } else { 24.0 };
        let side = self.page_side_width(SIDE_WIDTH);
        let width = shape.width - shape.rail() - side - shape.card_margin() - 2.0 * pad;
        let columns = (((width + GAP) / (CARD_WIDTH + GAP)).floor() as usize).clamp(1, 8);
        // Too narrow for two whole cards, two narrower ones fill it, as
        // Keep's phone app does.
        let (columns, card) = if columns == 1 && width >= 2.0 * NARROW_CARD_WIDTH + GAP {
            (2, ((width - GAP) / 2.0).min(CARD_WIDTH))
        } else {
            (columns, CARD_WIDTH)
        };
        let heading = |text: String| {
            div()
                .w_full()
                .max_w(px(columns as f32 * (card + GAP) - GAP))
                .mx_auto()
                .mt(px(8.0))
                .mb(px(8.0))
                .pl(px(8.0))
                .text_size(px(11.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(th.text_dim))
                .child(text.to_uppercase())
        };
        let mut body = div()
            .id("notes-board")
            .flex_1()
            .min_w_0()
            .h_full()
            .overflow_y_scroll()
            .px(px(pad))
            .pb(px(48.0))
            .flex()
            .flex_col();
        if view == NotesView::Notes {
            body = body.child(self.render_take_note(th, window, cx));
        }
        if view == NotesView::Trash {
            let ids: Vec<i64> = shown.iter().map(|n| n.id).collect();
            body = body.child(
                div()
                    .mt(px(20.0))
                    .mb(px(12.0))
                    .flex()
                    .flex_row()
                    .justify_center()
                    .items_center()
                    .gap(px(16.0))
                    .text_size(px(14.0))
                    .italic()
                    .text_color(rgba(th.text_dim))
                    .child(tr!("notes-trash-note"))
                    .when(!ids.is_empty(), |d| {
                        d.child(
                            div()
                                .id("notes-empty-trash")
                                .px(px(16.0))
                                .h(px(36.0))
                                .flex()
                                .items_center()
                                .rounded(px(4.0))
                                .not_italic()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgba(th.accent))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgba(th.hover)))
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.delete_notes_forever(ids.clone(), cx)
                                }))
                                .child(tr!("notes-empty-trash")),
                        )
                    }),
            );
        }
        if shown.is_empty() {
            let (name, text) = match view {
                _ if !query.is_empty() => ("search", tr!("notes-none-found")),
                _ if label.is_some() => ("label", tr!("notes-label-empty")),
                NotesView::Notes => ("notes", tr!("notes-empty")),
                NotesView::Archive => ("archive", tr!("notes-archive-empty")),
                NotesView::Trash => ("trash", tr!("notes-trash-empty")),
            };
            return body
                .child(
                    div()
                        .flex_1()
                        .min_h(px(320.0))
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap(px(20.0))
                        .child(icon(name, fade(th.text_faint, 0.5), 120.0))
                        .child(
                            div()
                                .text_center()
                                .text_size(px(22.0))
                                .text_color(rgba(th.text_dim))
                                .child(text),
                        ),
                )
                .into_any_element();
        }
        let (pinned, others): (Vec<&Note>, Vec<&Note>) = shown.into_iter().partition(|n| n.pinned);
        if !pinned.is_empty() {
            body = body
                .child(heading(tr!("notes-pinned")))
                .child(self.render_grid(&pinned, "notes-pinned", columns, card, th, cx));
            if !others.is_empty() {
                body = body
                    .child(div().h(px(24.0)))
                    .child(heading(tr!("notes-others")));
            }
        } else {
            body = body.child(div().h(px(8.0)));
        }
        if !others.is_empty() {
            body = body.child(self.render_grid(&others, "notes-others", columns, card, th, cx));
        }
        body.into_any_element()
    }

    fn render_grid(
        &self,
        notes: &[&Note],
        key: &'static str,
        columns: usize,
        card: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        div()
            .id(key)
            .flex()
            .flex_row()
            .justify_center()
            .gap(px(GAP))
            .children(masonry(notes, columns).into_iter().map(|column| {
                div()
                    .w(px(card))
                    .flex()
                    .flex_col()
                    .gap(px(GAP))
                    .children(column.into_iter().map(|n| self.render_card(n, th, cx)))
            }))
            .into_any_element()
    }

    /// Keep's "Take a note…" bar, with a new list at its right.
    /// Whether the open note is a new one from the "Take a note" bar, which
    /// opens in the bar's place rather than over the page.
    fn new_note_inline(&self) -> bool {
        self.notes.as_ref().is_some_and(|page| {
            page.view == NotesView::Notes && page.editor.as_ref().is_some_and(|e| e.in_bar)
        })
    }

    fn render_take_note(
        &self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let row = div()
            .flex()
            .flex_row()
            .justify_center()
            .pt(px(32.0))
            .pb(px(24.0));
        if self.new_note_inline()
            && let Some(card) = self.render_editor(th, true, window, cx)
        {
            return row.child(card).into_any_element();
        }
        row.child(
            div()
                .id("notes-take")
                .w_full()
                .max_w(px(EDITOR_WIDTH))
                .h(px(48.0))
                .pl(px(16.0))
                .pr(px(4.0))
                .flex()
                .flex_row()
                .items_center()
                .rounded(px(8.0))
                .bg(rgba(th.surface))
                .border_1()
                .border_color(rgba(th.divider))
                .shadow(elevation(th, 1.0))
                .cursor_text()
                .on_click(
                    cx.listener(|this, _, window, cx| {
                        this.open_note(None, false, None, window, cx)
                    }),
                )
                .child(
                    div()
                        .flex_1()
                        .text_size(px(15.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgba(th.text_dim))
                        .child(tr!("notes-take-a-note")),
                )
                .child(
                    icon_button("notes-new-list", "checkbox-checked", 22.0, th)
                        .tooltip(tip(tr!("notes-new-list"), th))
                        .on_click(cx.listener(|this, _, window, cx| {
                            cx.stop_propagation();
                            this.open_note(None, true, None, window, cx)
                        })),
                ),
        )
        .into_any_element()
    }

    fn render_card(&self, note: &Note, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let bg = note_color(note.color, th);
        let id = note.id;
        let trashed = note.trashed_at.is_some();
        let group: SharedString = format!("note-{id}").into();
        let open = Rc::new(note.clone());
        let base = item_of(note);
        let mut lines = Vec::new();
        let mut ticked = 0;
        for (ix, line) in note.body.split('\n').enumerate() {
            match check_of(line) {
                Some((true, _)) => ticked += 1,
                Some((false, text)) => lines.push((ix, Some(false), text.to_owned())),
                None => lines.push((ix, None, line.to_owned())),
            }
        }
        // Trailing empty lines show nothing.
        while lines
            .last()
            .is_some_and(|(_, check, text)| check.is_none() && text.trim().is_empty())
        {
            lines.pop();
        }
        let more = lines.len().saturating_sub(CARD_LINES);
        lines.truncate(CARD_LINES);
        // A formatted note shows its headings, bold, italic and underline.
        let formatted = format::card_paras(&note.body, &note.html);
        let body = lines.into_iter().map(|(ix, check, text)| {
            let styled = formatted
                .as_ref()
                .and_then(|paras| paras.get(ix))
                .map(|para| {
                    let skip = if check.is_some() { UNTICKED.len() } else { 0 };
                    format::card_line(para, skip)
                });
            let scale = styled.as_ref().map_or(1.0, |(_, scale)| *scale);
            let row = div()
                .flex()
                .flex_row()
                .items_start()
                .gap(px(8.0))
                .text_size(px(14.0 * scale))
                .line_height(px(20.0 * scale));
            let text = match styled {
                Some((element, _)) => element,
                None if text.is_empty() => " ".to_owned().into_any_element(),
                None => text.into_any_element(),
            };
            match check {
                Some(done) => {
                    let item = base.clone();
                    row.child(
                        div()
                            .id(("note-check", ix))
                            .flex_none()
                            .mt(px(1.0))
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, _, cx| {
                                cx.stop_propagation();
                                let mut item = item.clone();
                                if !item.html.is_empty() {
                                    item.html =
                                        format::toggle_html_line(&item.body, &item.html, ix);
                                }
                                item.body = toggle_line(&item.body, ix);
                                this.change_note(item, cx)
                            }))
                            .child(icon(
                                if done { "checkbox-checked" } else { "checkbox" },
                                th.text_dim,
                                18.0,
                            )),
                    )
                    .child(div().flex_1().min_w_0().child(text))
                }
                None => row.child(div().flex_1().min_w_0().child(text)),
            }
        });
        let footer = |name: &'static str, tip_text: String| {
            icon_button(("note-act", id as usize ^ name.len()), name, 18.0, th)
                .size(px(34.0))
                .tooltip(tip(tip_text, th))
        };
        let actions = if trashed {
            div()
                .flex()
                .flex_row()
                .child(
                    footer("trash", tr!("notes-delete-forever")).on_click(cx.listener(
                        move |this, _, _, cx| {
                            cx.stop_propagation();
                            this.delete_notes_forever(vec![id], cx)
                        },
                    )),
                )
                .child(
                    footer("restore", tr!("notes-restore")).on_click(cx.listener(
                        move |this, _, _, cx| {
                            cx.stop_propagation();
                            this.trash_note(id, false, cx)
                        },
                    )),
                )
        } else {
            let for_archive = open.clone();
            let archived = note.archived;
            div()
                .flex()
                .flex_row()
                .child(
                    footer(
                        "archive",
                        if archived {
                            tr!("notes-unarchive")
                        } else {
                            tr!("notes-archive")
                        },
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        this.archive_note(&for_archive, !archived, cx)
                    })),
                )
                .child(footer("trash", tr!("notes-delete")).on_click(cx.listener(
                    move |this, _, _, cx| {
                        cx.stop_propagation();
                        this.trash_note(id, true, cx)
                    },
                )))
        };
        let pin = (!trashed).then(|| {
            let mut item = item_of(note);
            item.pinned = !note.pinned;
            if item.pinned {
                item.archived = false;
            }
            div()
                .absolute()
                .top(px(4.0))
                .right(px(4.0))
                .when(!note.pinned, |d| {
                    d.opacity(0.0)
                        .group_hover(group.clone(), |s| s.opacity(1.0))
                })
                .child(
                    icon_button_colored(
                        ("note-pin", id as usize),
                        if note.pinned { "pin-filled" } else { "pin" },
                        20.0,
                        th.text_dim,
                        th,
                    )
                    .size(px(34.0))
                    .tooltip(tip(
                        if note.pinned {
                            tr!("notes-unpin")
                        } else {
                            tr!("notes-pin")
                        },
                        th,
                    ))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        this.change_note(item.clone(), cx)
                    })),
                )
        });
        div()
            .id(("note", id as usize))
            .group(group.clone())
            .relative()
            .w_full()
            .flex()
            .flex_col()
            .rounded(px(8.0))
            .border_1()
            .border_color(rgba(if bg.is_some() { 0x00000000 } else { th.divider }))
            .bg(rgba(bg.unwrap_or(th.surface)))
            .text_color(rgba(th.text))
            .cursor_default()
            .hover(|s| s.shadow(elevation(th, 1.0)))
            .on_click(cx.listener(move |this, _, window, cx| {
                if !trashed {
                    this.open_note(Some(&open), false, None, window, cx)
                }
            }))
            .child(
                div()
                    .px(px(16.0))
                    .pt(px(12.0))
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .when(!note.title.is_empty(), |d| {
                        d.child(
                            div()
                                .pr(px(24.0))
                                .text_size(px(16.0))
                                .line_height(px(24.0))
                                .font_weight(FontWeight::MEDIUM)
                                .line_clamp(3)
                                .child(note.title.clone()),
                        )
                    })
                    .children(body)
                    .when(more > 0, |d| {
                        d.child(
                            div()
                                .text_size(px(14.0))
                                .text_color(rgba(th.text_dim))
                                .child("…"),
                        )
                    })
                    .when(ticked > 0, |d| {
                        d.child(
                            div()
                                .pt(px(4.0))
                                .text_size(px(13.0))
                                .text_color(rgba(th.text_dim))
                                .child(tr!("notes-ticked", count = ticked as u64)),
                        )
                    })
                    .when(!note.labels.is_empty(), |d| {
                        d.child(
                            div()
                                .pt(px(8.0))
                                .flex()
                                .flex_row()
                                .flex_wrap()
                                .gap(px(6.0))
                                .children(note.labels.iter().enumerate().map(|(ix, label)| {
                                    let show = label.clone();
                                    div()
                                        .id(("note-card-label", id as usize * 64 + ix))
                                        .cursor_pointer()
                                        .hover(|s| s.bg(rgba(fade(th.text, 0.14))))
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            cx.stop_propagation();
                                            this.show_label(show.clone(), cx)
                                        }))
                                        .px(px(10.0))
                                        .h(px(24.0))
                                        .flex()
                                        .items_center()
                                        .rounded_full()
                                        .bg(rgba(fade(th.text, 0.08)))
                                        .text_size(px(12.0))
                                        .child(label.clone())
                                })),
                        )
                    })
                    .children(note.link.clone().map(|link| {
                        // A row, so the chip keeps its own width.
                        div().flex().flex_row().child(self.mail_chip(
                            ("note-card-mail", id as usize),
                            link,
                            th,
                            cx,
                        ))
                    })),
            )
            .child(
                div()
                    .h(px(38.0))
                    .px(px(6.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .opacity(0.0)
                    .group_hover(group, |s| s.opacity(1.0))
                    .child(actions),
            )
            .children(pin)
            .into_any_element()
    }

    /// The open note, over a dimmed board.
    /// The open note: `inline` in the board's flow, otherwise centred over
    /// the page on a scrim.
    pub(super) fn render_editor(
        &self,
        th: &Theme,
        inline: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let editor = self.notes.as_ref()?.editor.as_ref()?;
        let bg = note_color(editor.color, th).unwrap_or(th.surface);
        let vh = unpx(window.viewport_size().height);
        let id = editor.id;
        let pinned = editor.pinned;
        let archived = editor.archived;
        let item = editor.item(cx);
        let edited = (editor.updated_at > 0)
            .then(|| {
                crate::format::local(editor.updated_at, &self.tz)
                    .zip(crate::format::local(
                        jiff::Timestamp::now().as_second(),
                        &self.tz,
                    ))
                    .map(|(d, now)| tr!("notes-edited", date = crate::format::list_date(d, now)))
            })
            .flatten();
        let where_ = self.note_place(editor.account);
        let places = editor.places.then(|| {
            let current = editor.account;
            let mut options: Vec<(Option<i64>, String)> = self
                .accounts
                .iter()
                .filter(|a| a.kind == katna_core::AccountKind::Imap)
                .map(|a| (Some(a.id.0), a.display_name.clone()))
                .collect();
            options.push((None, tr!("notes-on-this-computer")));
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .justify_end()
                .gap(px(6.0))
                .px(px(12.0))
                .pb(px(8.0))
                .children(
                    options
                        .into_iter()
                        .enumerate()
                        .map(|(ix, (account, name))| {
                            let on = account == current;
                            let mut item = item.clone();
                            item.account = account.unwrap_or(0);
                            div()
                                .id(("note-place", ix))
                                .h(px(28.0))
                                .px(px(12.0))
                                .flex()
                                .items_center()
                                .rounded_full()
                                .border_1()
                                .border_color(rgba(if on { th.accent } else { th.divider }))
                                .when(on, |d| d.bg(rgba(fade(th.accent, 0.12))))
                                .text_size(px(12.0))
                                .text_color(rgba(if on { th.accent } else { th.text_dim }))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgba(th.hover)))
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    if let Some(editor) =
                                        this.notes.as_mut().and_then(|p| p.editor.as_mut())
                                    {
                                        editor.places = false;
                                    }
                                    if item.id == 0 {
                                        if let Some(editor) =
                                            this.notes.as_mut().and_then(|p| p.editor.as_mut())
                                        {
                                            editor.account = account;
                                        }
                                        this.note_typed(cx);
                                    } else {
                                        this.change_note(item.clone(), cx);
                                    }
                                }))
                                .child(name)
                        }),
                )
        });
        let tool = |name: &'static str, tip_text: String| {
            icon_button(
                ("note-tool", name.len() * 31 + name.as_bytes()[0] as usize),
                name,
                18.0,
                th,
            )
            .size(px(34.0))
            .tooltip(tip(tip_text, th))
        };
        let palette = editor.palette.then(|| {
            let current = editor.color;
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(px(6.0))
                .px(px(12.0))
                .pb(px(8.0))
                .children((0..=COLORS.len()).map(|ix| {
                    let color = ix as i64;
                    let fill = note_color(color, th).unwrap_or(th.surface);
                    let name = if ix == 0 {
                        tr!("notes-color-none")
                    } else {
                        tr!(COLORS[ix - 1].0)
                    };
                    let mut item = item.clone();
                    item.color = color;
                    div()
                        .id(("note-color", ix))
                        .size(px(32.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .bg(rgba(fill))
                        .border_2()
                        .border_color(rgba(if color == current {
                            th.accent
                        } else if ix == 0 {
                            th.divider
                        } else {
                            0x00000000
                        }))
                        .cursor_pointer()
                        .hover(|s| s.border_color(rgba(th.text_dim)))
                        .tooltip(tip(name, th))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if item.id == 0 {
                                if let Some(editor) =
                                    this.notes.as_mut().and_then(|p| p.editor.as_mut())
                                {
                                    editor.color = item.color;
                                }
                                this.note_typed(cx);
                            } else {
                                this.change_note(item.clone(), cx);
                            }
                        }))
                        .when(ix == 0, |d| d.child(icon("close", th.text_dim, 16.0)))
                }))
        });
        let for_pin = {
            let mut item = item.clone();
            item.pinned = !pinned;
            if item.pinned {
                item.archived = false;
            }
            item
        };
        let card = div()
            .id("note-editor")
            .occlude()
            .relative()
            .when(inline, |d| d.w_full().max_w(px(EDITOR_WIDTH)))
            .when(!inline, |d| {
                d.w(px(
                    EDITOR_WIDTH.min(unpx(window.viewport_size().width) - 32.0)
                ))
            })
            .max_h(px(vh * 0.7))
            .flex()
            .flex_col()
            .rounded(px(15.0))
            .bg(rgba(bg))
            .text_color(rgba(th.text))
            .shadow(elevation(th, 3.0))
            .on_click(|_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .flex_none()
                    .pl(px(16.0))
                    .pr(px(4.0))
                    .pt(px(8.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .py(px(8.0))
                            .text_size(px(22.0))
                            .line_height(px(28.0))
                            .child(editor.title.clone()),
                    )
                    .child(
                        icon_button_colored(
                            "note-editor-pin",
                            if pinned { "pin-filled" } else { "pin" },
                            22.0,
                            th.text_dim,
                            th,
                        )
                        .tooltip(tip(
                            if pinned {
                                tr!("notes-unpin")
                            } else {
                                tr!("notes-pin")
                            },
                            th,
                        ))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if for_pin.id == 0 {
                                if let Some(editor) =
                                    this.notes.as_mut().and_then(|p| p.editor.as_mut())
                                {
                                    editor.pinned = for_pin.pinned;
                                }
                                this.note_typed(cx);
                            } else {
                                this.change_note(for_pin.clone(), cx);
                            }
                        })),
                    ),
            )
            .child(
                div()
                    .id("note-editor-body")
                    .flex_1()
                    .min_h(px(60.0))
                    .overflow_y_scroll()
                    .px(px(16.0))
                    .pb(px(12.0))
                    .text_size(px(15.0))
                    .line_height(px(22.0))
                    .child(editor.body.clone()),
            )
            .children(self.render_note_label_chips(th, cx))
            .child(
                div()
                    .flex_none()
                    .px(px(16.0))
                    .pb(px(4.0))
                    .flex()
                    .flex_row()
                    .justify_end()
                    .items_center()
                    .gap(px(4.0))
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_dim))
                    .children(
                        editor
                            .link
                            .clone()
                            .map(|link| self.mail_chip("note-editor-mail", link, th, cx)),
                    )
                    .child(div().flex_1())
                    .children(edited.map(|edited| format!("{edited} ·")))
                    .child(
                        div()
                            .id("note-place")
                            .px(px(6.0))
                            .h(px(22.0))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(2.0))
                            .rounded(px(6.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .tooltip(tip(tr!("notes-where"), th))
                            .on_click(cx.listener(|this, _, _, cx| {
                                if let Some(editor) =
                                    this.notes.as_mut().and_then(|p| p.editor.as_mut())
                                {
                                    editor.places = !editor.places;
                                    editor.palette = false;
                                    editor.picker = None;
                                }
                                cx.notify();
                            }))
                            .child(where_)
                            .child(icon("drop-down", th.text_dim, 16.0)),
                    ),
            )
            .children(places)
            .children(palette)
            .children(self.render_format_row(th, cx))
            .children(self.render_label_picker(th, cx))
            .child(
                div()
                    .flex_none()
                    .px(px(8.0))
                    .pb(px(8.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(2.0))
                    .child(
                        tool("format-text", tr!("notes-format")).on_click(cx.listener(
                            |this, _, window, cx| {
                                let Some(editor) =
                                    this.notes.as_mut().and_then(|p| p.editor.as_mut())
                                else {
                                    return;
                                };
                                editor.format = !editor.format;
                                editor.palette = false;
                                editor.places = false;
                                editor.picker = None;
                                let focus = editor.body.focus_handle(cx);
                                window.focus(&focus, cx);
                                cx.notify();
                            },
                        )),
                    )
                    .child(tool("text-color", tr!("notes-color")).on_click(cx.listener(
                        |this, _, _, cx| {
                            if let Some(editor) =
                                this.notes.as_mut().and_then(|p| p.editor.as_mut())
                            {
                                editor.palette = !editor.palette;
                                editor.places = false;
                                editor.picker = None;
                            }
                            cx.notify();
                        },
                    )))
                    .child(tool("label", tr!("notes-labels")).on_click(
                        cx.listener(|this, _, window, cx| this.toggle_label_picker(window, cx)),
                    ))
                    .child(
                        tool("checkbox-checked", tr!("notes-checkboxes")).on_click(cx.listener(
                            |this, _, window, cx| {
                                let Some(editor) =
                                    this.notes.as_ref().and_then(|p| p.editor.as_ref())
                                else {
                                    return;
                                };
                                let body = editor.body.clone();
                                body.update(cx, |area, cx| {
                                    area.edit_doc(format::toggle_checklist_doc, cx)
                                });
                                window.focus(&body.focus_handle(cx), cx);
                            },
                        )),
                    )
                    .when(self.task_line(cx).is_some(), |d| {
                        d.child(tool("tasks", tr!("notes-make-task")).on_click(
                            cx.listener(|this, _, window, cx| this.make_line_task(window, cx)),
                        ))
                    })
                    .when(id != 0, |d| {
                        let note = Note {
                            id,
                            archived,
                            ..Note::default()
                        };
                        d.child(
                            tool(
                                "archive",
                                if archived {
                                    tr!("notes-unarchive")
                                } else {
                                    tr!("notes-archive")
                                },
                            )
                            .on_click(cx.listener(
                                move |this, _, _, cx| {
                                    let Some(editor) =
                                        this.notes.as_ref().and_then(|p| p.editor.as_ref())
                                    else {
                                        return;
                                    };
                                    let mut note = note.clone();
                                    let item = editor.item(cx);
                                    note.title = item.title;
                                    note.body = item.body;
                                    note.color = item.color;
                                    note.labels = item.labels;
                                    note.link = Some(item.link).filter(|l| !l.is_empty());
                                    note.account_id = (item.account != 0).then_some(item.account);
                                    this.archive_note(&note, !archived, cx)
                                },
                            )),
                        )
                        .child(
                            tool("trash", tr!("notes-delete")).on_click(
                                cx.listener(move |this, _, _, cx| this.trash_note(id, true, cx)),
                            ),
                        )
                    })
                    .child(div().flex_1())
                    .child(
                        div()
                            .id("note-editor-close")
                            .h(px(36.0))
                            .px(px(24.0))
                            .flex()
                            .items_center()
                            .rounded(px(4.0))
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .on_click(cx.listener(|this, _, _, cx| this.close_note(cx)))
                            .child(tr!("notes-close")),
                    ),
            );
        if inline {
            // Clicking anywhere else on the page closes it, as in Keep.
            return Some(
                card.on_mouse_down_out(cx.listener(|this, _, _, cx| this.close_note(cx)))
                    .into_any_element(),
            );
        }
        Some(
            div()
                .id("note-editor-scrim")
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .flex_col()
                .items_center()
                .pt(px(48.0))
                .bg(rgba(if th.dark { 0x00000099 } else { 0x0000004d }))
                .on_click(cx.listener(|this, _, _, cx| this.close_note(cx)))
                .child(card)
                .with_animation(
                    ("note-editor-in", id as usize),
                    gpui::Animation::new(Duration::from_millis(180))
                        .with_easing(gpui::ease_out_quint()),
                    |el, t| el.opacity(t),
                )
                .into_any_element(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_line_ticks_and_unticks() {
        let body = "☐ milk\n☑ eggs\nplain";
        assert_eq!(toggle_line(body, 0), "☑ milk\n☑ eggs\nplain");
        assert_eq!(toggle_line(body, 1), "☐ milk\n☐ eggs\nplain");
        assert_eq!(toggle_line(body, 2), body);
    }

    #[test]
    fn checkboxes_come_and_go_as_in_keep() {
        assert_eq!(toggle_checklist("milk\neggs"), "☐ milk\n☐ eggs");
        assert_eq!(toggle_checklist("☐ milk\n☑ eggs"), "milk\neggs");
        assert_eq!(toggle_checklist(""), "☐ ");
    }

    #[test]
    fn search_needs_every_word() {
        let note = Note {
            title: "Goa trip".to_owned(),
            body: "☐ Book train".to_owned(),
            labels: vec!["Travel".to_owned()],
            ..Note::default()
        };
        assert!(matches(&note, "goa train"));
        assert!(matches(&note, "travel"));
        assert!(!matches(&note, "goa hotel"));
    }

    #[test]
    fn the_board_fills_the_shortest_column() {
        let long = Note {
            body: "a\n".repeat(10),
            ..Note::default()
        };
        let short = Note::default();
        let notes = [&long, &short, &short, &short];
        let columns = masonry(&notes, 2);
        assert_eq!(columns[0].len(), 1);
        assert_eq!(columns[1].len(), 3);
    }
}
