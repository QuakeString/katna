// SPDX-License-Identifier: GPL-3.0-or-later

//! The Tasks page (`docs/ARCHITECTURE.md` §18.1), laid out like Google
//! Tasks: on the left, Create, All tasks, Starred and every list grouped
//! by account; on the right, each list as a card side by side, or one
//! list or the starred tasks alone. A task is a round tick, its title,
//! notes, due day and a star; its steps sit under it. Done tasks fold
//! into "Completed" at the bottom of each list.
//!
//! The page reads the store and sends changes to the daemon, which sends
//! them on to Google Tasks or To Do; Undo and Ctrl+Z take them back.

use std::collections::{HashMap, HashSet};

use gpui::{
    Animation, AnimationExt, AnyElement, Bounds, Context, Entity, FocusHandle, Focusable,
    FontWeight, KeyDownEvent, MouseButton, Pixels, Point, ScrollHandle, SharedString, Subscription,
    Task, Window, anchored, deferred, div, ease_out_quint, prelude::*, rgba,
};
use katna_core::{AccountId, AccountKind};
use katna_i18n::tr;
use katna_store::tasks::Task as TaskItem;
use katna_ui::px;
use katna_ui::text_input::{InputEvent, TextInput};

mod details;

use super::MailWindow;
use super::account_status::{AccountStatus, Of, Say};
use crate::daemon::{self, Command};
use crate::data::EntryKey;
use crate::tasks::{Board, Column, TaskCommand, TaskEdit};
use crate::theme::{Theme, fade};
use crate::widgets::{ScaledEdge, icon, raised, tip};

/// The width of the lists on the left.
const NAV_WIDTH: f32 = 256.0;
/// The width of one list's card.
const CARD_WIDTH: f32 = 360.0;
/// The widest a single list gets.
const SINGLE_WIDTH: f32 = 640.0;
const MENU_WIDTH: f32 = 220.0;

/// What the right side shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(super) enum View {
    /// Every list, side by side.
    #[default]
    All,
    /// Due today or overdue, from every list.
    Today,
    Starred,
    List(i64),
}

/// A task typed into a list's "Add a task" row.
struct Adding {
    /// `0` for the default list.
    list: i64,
    parent: Option<i64>,
    /// `YYYY-MM-DD` the new task is due, or empty.
    due: String,
    input: Entity<TextInput>,
    _subscription: Subscription,
}

/// A title being changed in place.
struct Editing {
    id: i64,
    input: Entity<TextInput>,
    _subscription: Subscription,
}

/// A list's name being typed: a new list, or a new name.
struct Naming {
    /// The list renamed; `None` for a new list.
    list: Option<i64>,
    account: Option<AccountId>,
    input: Entity<TextInput>,
    _subscription: Subscription,
}

/// A list's ⋮ menu, or a task's right-click menu.
enum Menu {
    List { list: i64, at: Point<Pixels> },
    Task { id: i64, at: Point<Pixels> },
}

/// Changes sent but not read back yet, shown at once.
#[derive(Debug, Default, Clone, Copy)]
struct Pending {
    done: Option<bool>,
    starred: Option<bool>,
}

/// A task with the list it is in.
type Placed<'a> = (&'a Column, &'a TaskItem);

/// The Tasks page's state, kept while other apps show.
#[derive(Default)]
pub(super) struct TasksPage {
    /// `None` while the first read runs.
    board: Option<Result<Board, String>>,
    view: View,
    adding: Option<Adding>,
    editing: Option<Editing>,
    naming: Option<Naming>,
    details: Option<details::Details>,
    menu: Option<Menu>,
    /// Lists whose Completed section is open.
    open_done: HashSet<i64>,
    /// The task picked by click or keys.
    picked: Option<i64>,
    pending: HashMap<i64, Pending>,
    pub(super) focus: Option<FocusHandle>,
    loading: Option<Task<()>>,
    /// Reads again whenever the daemon says tasks changed.
    watching: Option<Task<()>>,
    /// The open task made from each mail line's mail, for its chip in the
    /// mail list.
    from_mail: HashMap<EntryKey, i64>,
    /// The mails (`Message-ID`s) of open tasks made from mail, sorted, for
    /// the contact panel's Tasks.
    pub(super) open_mails: Vec<String>,
    /// What the top bar's search box holds while the page shows: only
    /// tasks with every word show.
    query: String,
    /// The mail search's words, kept while the box searches tasks.
    mail_query: Option<String>,
    /// Where each account's task sync stands, for the line under it.
    pub(super) accounts: AccountStatus,
    /// The All tasks board, to show a list just made.
    board_scroll: ScrollHandle,
    /// The lists there were when a new one was sent: the one not among
    /// them is scrolled into view once it is read back.
    reveal_new: Option<HashSet<i64>>,
    /// A task being dragged over the lists, while it is.
    drag: Option<Drag>,
}

/// A task being dragged: the lists open a gap where it would land, as in
/// Google Tasks. Only while GPUI has a [`TaskDragged`] under way.
struct Drag {
    id: i64,
    /// Its list, and its slot there among the open tasks shown.
    from: (i64, usize),
    /// The height of its row with its steps: the gap it leaves and opens.
    height: f32,
    /// The list and slot where it lands if let go now: before the slot's
    /// task among the list's open tasks shown (itself left out), or after
    /// the last. `None` over no list.
    to: Option<(i64, usize)>,
    /// Whether the gap at `to` opens at once: the one it left, as it
    /// lifts.
    at_once: bool,
    /// Gaps closing where it no longer lands, each with its animation's
    /// number.
    closing: Vec<(i64, usize, usize)>,
    /// The number of the gap opening at `to`; each new place has its own,
    /// so its animation starts afresh.
    serial: usize,
    /// Each task's row with its steps, as last seen during the drag.
    rows: HashMap<i64, Bounds<Pixels>>,
}

/// How long a gap takes to open or close.
const GAP_MS: u64 = 150;

/// A task being dragged to another place in its list or another: it
/// follows the pointer as a lifted card, as in Google Tasks.
#[derive(Clone)]
struct TaskDragged {
    id: i64,
    list: i64,
    title: String,
    th: Theme,
}

impl Render for TaskDragged {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let th = &self.th;
        div()
            .w(px(CARD_WIDTH - 32.0))
            .py(px(10.0))
            .px(px(16.0))
            .rounded(px(8.0))
            .bg(rgba(th.surface))
            .shadow(crate::widgets::elevation(th, 3.0))
            .text_size(px(14.0))
            .line_height(px(20.0))
            .text_color(rgba(th.text))
            .truncate()
            .child(self.title.clone())
    }
}

impl TasksPage {
    /// Whether `task` shows for the search: its title or notes hold every
    /// word, or its task's do (a step shows with its task), or one of its
    /// steps' do.
    fn found(&self, task: &TaskItem) -> bool {
        let words: Vec<String> = self
            .query
            .split_whitespace()
            .map(str::to_lowercase)
            .collect();
        if words.is_empty() {
            return true;
        }
        let has = |t: &TaskItem| {
            let text = format!("{}\n{}", t.title, t.notes).to_lowercase();
            words.iter().all(|w| text.contains(w.as_str()))
        };
        has(task)
            || task.parent.and_then(|p| self.task(p)).is_some_and(has)
            || (task.parent.is_none()
                && self
                    .columns()
                    .iter()
                    .flat_map(|c| c.tasks.iter())
                    .any(|t| t.parent == Some(task.id) && has(t)))
    }

    fn searching(&self) -> bool {
        !self.query.trim().is_empty()
    }

    fn done(&self, task: &TaskItem) -> bool {
        self.pending
            .get(&task.id)
            .and_then(|p| p.done)
            .unwrap_or(task.done_at.is_some())
    }

    fn starred(&self, task: &TaskItem) -> bool {
        self.pending
            .get(&task.id)
            .and_then(|p| p.starred)
            .unwrap_or(task.starred)
    }

    pub(super) fn columns(&self) -> &[Column] {
        match &self.board {
            Some(Ok(board)) => &board.columns,
            _ => &[],
        }
    }

    fn task(&self, id: i64) -> Option<&TaskItem> {
        match &self.board {
            Some(Ok(board)) => board.task(id),
            _ => None,
        }
    }

    fn task_mut(&mut self, id: i64) -> Option<&mut TaskItem> {
        match &mut self.board {
            Some(Ok(board)) => board
                .columns
                .iter_mut()
                .flat_map(|c| c.tasks.iter_mut())
                .find(|t| t.id == id),
            _ => None,
        }
    }

    /// Where `task` goes when it is ticked off, as the daemon moves it: its
    /// next due day and rule, if it repeats. Not in To Do, which makes the
    /// next one itself.
    fn next_due(&self, task: &TaskItem) -> Option<(String, String)> {
        if task.parent.is_some() || task.repeat.is_empty() || task.done_at.is_some() {
            return None;
        }
        if self
            .columns()
            .iter()
            .any(|c| c.list.id == task.list && c.to_do)
        {
            return None;
        }
        katna_dav::todo::next_due(&task.due, &task.repeat, today())
    }

    /// The open tasks shown, in order, for keys to move through.
    fn shown(&self) -> Vec<i64> {
        let mut shown = Vec::new();
        for column in self.shown_columns() {
            for task in &column.tasks {
                if !self.done(task) && self.parent_open(task) && self.found(task) {
                    shown.push(task.id);
                }
            }
        }
        match self.view {
            View::Starred => {
                shown.retain(|id| self.task(*id).is_some_and(|t| self.starred(t)));
            }
            View::Today => {
                let (overdue, due) = self.due_now(today());
                return overdue
                    .into_iter()
                    .chain(due)
                    .filter(|(_, t)| self.found(t))
                    .map(|(_, t)| t.id)
                    .collect();
            }
            View::All | View::List(_) => {}
        }
        shown
    }

    /// The open tasks due before `today` and on it, from every list, each
    /// with its list: by day, then by time (those without one last).
    fn due_now(&self, today: jiff::civil::Date) -> (Vec<Placed<'_>>, Vec<Placed<'_>>) {
        let mut found: Vec<(jiff::civil::Date, &Column, &TaskItem)> = self
            .columns()
            .iter()
            .flat_map(|c| c.tasks.iter().map(move |t| (c, t)))
            .filter(|(_, t)| !self.done(t) && self.parent_open(t))
            .filter_map(|(c, t)| {
                let day: jiff::civil::Date = t.due.parse().ok()?;
                (day <= today).then_some((day, c, t))
            })
            .collect();
        found.sort_by_key(|(day, _, t)| (*day, t.due_time.is_none(), t.due_time));
        let mut overdue = Vec::new();
        let mut due = Vec::new();
        for (day, column, task) in found {
            if day < today {
                overdue.push((column, task));
            } else {
                due.push((column, task));
            }
        }
        (overdue, due)
    }

    /// Whether a step's task is still open (a done task hides its steps
    /// with it).
    fn parent_open(&self, task: &TaskItem) -> bool {
        task.parent
            .and_then(|p| self.task(p))
            .is_none_or(|p| !self.done(p))
    }

    /// Scrolls the board to a list just made, once it is read back.
    fn reveal_list(&mut self) {
        let Some(known) = &self.reveal_new else {
            return;
        };
        let columns = self.shown_columns();
        if let Some(ix) = columns.iter().position(|c| !known.contains(&c.list.id)) {
            self.board_scroll.scroll_to_item(ix);
            self.reveal_new = None;
        }
    }

    /// The open tasks (not steps) list `list` shows, in order, less task
    /// `except`: the slots a dragged task lands between.
    fn slots(&self, list: i64, except: i64) -> Vec<i64> {
        self.columns()
            .iter()
            .filter(|c| c.list.id == list)
            .flat_map(|c| c.tasks.iter())
            .filter(|t| t.parent.is_none() && t.id != except && !self.done(t) && self.found(t))
            .map(|t| t.id)
            .collect()
    }

    /// The drag of `dragged` as tracked here: a new one when it isn't yet.
    fn drag_of(&mut self, dragged: &TaskDragged) -> &mut Drag {
        if self.drag.as_ref().is_none_or(|d| d.id != dragged.id) {
            let slots = self.slots(dragged.list, -1);
            let slot = slots.iter().position(|t| *t == dragged.id).unwrap_or(0);
            self.drag = Some(Drag {
                id: dragged.id,
                from: (dragged.list, slot),
                height: 44.0,
                to: Some((dragged.list, slot)),
                at_once: true,
                closing: Vec::new(),
                serial: 0,
                rows: HashMap::new(),
            });
        }
        self.drag.as_mut().expect("just set")
    }

    /// The dragged task would land at `to` now; a gap opens there and the
    /// one it left closes. Returns whether that changed.
    fn drag_to(&mut self, dragged: &TaskDragged, to: Option<(i64, usize)>) -> bool {
        let drag = self.drag_of(dragged);
        if drag.to == to {
            return false;
        }
        if let Some((list, slot)) = drag.to {
            drag.closing.push((list, slot, drag.serial));
        }
        drag.serial += 1;
        drag.to = to;
        drag.at_once = false;
        true
    }

    fn shown_columns(&self) -> Vec<&Column> {
        match self.view {
            // While searching, only the lists with a task found.
            View::All => self
                .columns()
                .iter()
                .filter(|c| !self.searching() || c.tasks.iter().any(|t| self.found(t)))
                .collect(),
            View::Today | View::Starred => self.columns().iter().collect(),
            View::List(id) => self.columns().iter().filter(|c| c.list.id == id).collect(),
        }
    }
}

/// What typed quick add found in a new task's title.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TypedTask {
    title: String,
    /// `YYYY-MM-DD`; today when only a time or a repeat was typed.
    due: Option<String>,
    due_time: Option<u32>,
    repeat: Option<String>,
}

/// Reads a day, a time and a repeat from a new task's title
/// (`katna_core::quick_add`, shared with Calendar's events); `None` when it
/// says none, or nothing would be left of the title. Tasks have no place,
/// so "at" is left in the title.
fn typed_task(text: &str, today: jiff::civil::Date) -> Option<TypedTask> {
    use katna_core::quick_add;
    let language = katna_i18n::current().language.tag.clone();
    let words = quick_add::Words {
        at: &[],
        label_marks: &["#"],
        ..*quick_add::Words::for_language(&language)
    };
    let typed = quick_add::parse(text, today, &words);
    let title = typed.title.trim().to_owned();
    // The parser keeps a title made only of such words ("tomorrow") whole.
    if !typed.found() || title.is_empty() || title == text.trim() {
        return None;
    }
    let due_time = typed
        .start
        .map(|t| u32::try_from(i32::from(t.hour()) * 60 + i32::from(t.minute())).unwrap_or(0));
    // A repeat without a day starts on its first day from today ("every
    // Monday" typed on a Tuesday: next Monday); a time alone is today.
    let first_repeat = typed.repeat.as_deref().and_then(|rule| {
        let yesterday = today.yesterday().ok()?;
        let (day, _) = katna_dav::todo::next_due(&yesterday.to_string(), rule, yesterday)?;
        day.parse().ok()
    });
    let due = typed
        .day
        .or(first_repeat)
        .or_else(|| (due_time.is_some() || typed.repeat.is_some()).then_some(today))
        .map(|d| d.to_string());
    Some(TypedTask {
        title,
        due,
        due_time,
        repeat: typed.repeat,
    })
}

/// A day and month, with the year when it isn't this one.
fn day_text(day: jiff::civil::Date, today: jiff::civil::Date) -> String {
    let at = day.to_datetime(jiff::civil::Time::midnight());
    if day.year() == today.year() {
        katna_i18n::format::day_month(at)
    } else {
        katna_i18n::format::day_month_year(at)
    }
}

/// A due day as the page shows it: "Today", "Tomorrow", a weekday this
/// week, else the day and month; with the time if it has one. The flag
/// says it is past.
pub(super) fn due_label(task: &TaskItem, today: jiff::civil::Date) -> Option<(String, bool)> {
    let day: jiff::civil::Date = task.due.parse().ok()?;
    let days = (day - today).get_days();
    let at = day.to_datetime(jiff::civil::Time::midnight());
    let mut label = match days {
        0 => tr!("tasks-due-today"),
        1 => tr!("tasks-due-tomorrow"),
        -1 => tr!("tasks-due-yesterday"),
        2..=6 => katna_i18n::format::weekday(at),
        _ => day_text(day, today),
    };
    if let Some(minutes) = task.due_time {
        let time = jiff::civil::Time::new(
            i8::try_from(minutes / 60).unwrap_or(0),
            i8::try_from(minutes % 60).unwrap_or(0),
            0,
            0,
        )
        .unwrap_or_default();
        label = tr!(
            "tasks-due-at",
            day = label,
            time = katna_i18n::format::time(day.to_datetime(time))
        );
    }
    let past = days < 0 || (days == 0 && task.due_time.is_some_and(|m| now_minutes() > m));
    Some((label, past))
}

pub(super) fn today() -> jiff::civil::Date {
    jiff::Zoned::now().date()
}

fn now_minutes() -> u32 {
    let now = jiff::Zoned::now();
    u32::try_from(i32::from(now.hour()) * 60 + i32::from(now.minute())).unwrap_or(0)
}

impl MailWindow {
    /// Opening the page: reads the lists, and keeps them read as they
    /// change.
    pub(super) fn open_tasks_page(&mut self, cx: &mut Context<Self>) {
        if self.tasks.focus.is_none() {
            self.tasks.focus = Some(cx.focus_handle());
        }
        self.watch_tasks(cx);
    }

    /// Reads the lists, and keeps them read as they change: from the
    /// start, for the task chips in the mail list.
    pub(super) fn watch_tasks(&mut self, cx: &mut Context<Self>) {
        self.load_tasks(cx);
        if self.tasks.watching.is_none() {
            let connection = self.daemon.clone();
            self.tasks.watching = Some(cx.spawn(async move |this, cx| {
                let connection = match connection {
                    Some(connection) => connection,
                    None => match daemon::connect().await {
                        Ok(connection) => connection,
                        Err(_) => return,
                    },
                };
                let Ok(mut changes) = crate::tasks::changes(&connection).await else {
                    return;
                };
                use futures_lite::StreamExt;
                while changes.next().await.is_some() {
                    if this.update(cx, |this, cx| this.load_tasks(cx)).is_err() {
                        return;
                    }
                }
            }));
        }
    }

    /// Finds the mail line of each open task made from a mail. When a
    /// line has several, the one due first wins.
    pub(super) fn map_task_mails(&mut self) {
        let mut from_mail: HashMap<EntryKey, i64> = HashMap::new();
        if let (Some(Ok(board)), Ok(mail)) = (&self.tasks.board, &self.mail) {
            let mut tasks: Vec<&TaskItem> = board
                .columns
                .iter()
                .flat_map(|c| c.tasks.iter())
                .filter(|t| t.done_at.is_none() && !t.mail.is_empty())
                .filter(|t| super::notes::note_of_task(&t.mail).is_none())
                .collect();
            // Undated last, each key keeping the first it gets.
            tasks.sort_by_key(|t| (t.due.is_empty(), t.due.clone(), t.due_time));
            for task in tasks {
                let Some(message) = mail.message_with_header(&task.mail) else {
                    continue;
                };
                from_mail
                    .entry(EntryKey::Message(message))
                    .or_insert(task.id);
                if let Some(thread) = mail.message_thread(message) {
                    from_mail.entry(EntryKey::Thread(thread)).or_insert(task.id);
                }
            }
        }
        self.tasks.from_mail = from_mail;
        let mut open_mails: Vec<String> = match &self.tasks.board {
            Some(Ok(board)) => board
                .columns
                .iter()
                .flat_map(|c| c.tasks.iter())
                .filter(|t| t.done_at.is_none() && !t.mail.is_empty())
                .filter(|t| super::notes::note_of_task(&t.mail).is_none())
                .map(|t| t.mail.clone())
                .collect(),
            _ => Vec::new(),
        };
        open_mails.sort_unstable();
        open_mails.dedup();
        // The contact panel reads whose they are again.
        if open_mails != self.tasks.open_mails {
            self.tasks.open_mails = open_mails;
            self.contact.forget_profiles();
        }
    }

    /// Turns the top bar's search box to tasks while the page shows, and
    /// back to mail after, each keeping its own words.
    pub(super) fn swap_tasks_search(&mut self, entering: bool, cx: &mut Context<Self>) {
        let (placeholder, text) = if entering {
            self.tasks.mail_query = Some(self.search.read(cx).text().to_owned());
            (tr!("tasks-search"), self.tasks.query.clone())
        } else {
            let text = self.tasks.mail_query.take().unwrap_or_default();
            (tr!("search-mail"), text)
        };
        self.search.update(cx, |search, cx| {
            search.set_placeholder(placeholder);
            search.set_text(text, cx);
        });
    }

    /// The top bar's search box changed while the page shows.
    pub(super) fn on_tasks_search(
        &mut self,
        search: &Entity<TextInput>,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Changed => {
                self.tasks.query = search.read(cx).text().to_owned();
                // A picked task the search hides is let go.
                if let Some(id) = self.tasks.picked
                    && !self.tasks.shown().contains(&id)
                {
                    self.tasks.picked = None;
                }
            }
            InputEvent::Cancel => {
                self.tasks.query.clear();
                search.update(cx, |search, cx| search.set_text("", cx));
            }
            // Enter goes to the tasks found, for the arrow keys.
            InputEvent::Submit => {
                if let Some(focus) = &self.tasks.focus {
                    window.focus(focus, cx);
                }
            }
        }
        cx.notify();
    }

    /// The open tasks made from `mails`, due first first, and whether each
    /// is ticked, counting ticks not yet read back.
    pub(super) fn tasks_of_mails(&self, mails: &[String]) -> Vec<(&TaskItem, bool)> {
        let Some(Ok(board)) = &self.tasks.board else {
            return Vec::new();
        };
        let mut tasks: Vec<&TaskItem> = board
            .columns
            .iter()
            .flat_map(|c| c.tasks.iter())
            .filter(|t| t.done_at.is_none() && mails.contains(&t.mail))
            .collect();
        tasks.sort_by_key(|t| (t.due.is_empty(), t.due.clone(), t.due_time));
        tasks.into_iter().map(|t| (t, self.tasks.done(t))).collect()
    }

    /// The chip on a mail line with an open task made from its mail: the
    /// task's due day, or "Task"; it opens the task.
    pub(super) fn render_row_task(
        &self,
        ix: usize,
        key: EntryKey,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let id = *self.tasks.from_mail.get(&key)?;
        let task = self.tasks.task(id)?;
        let (label, past) = due_label(task, today()).unwrap_or((tr!("row-task"), false));
        let color = if past { th.error } else { th.text_dim };
        Some(
            div()
                .id(("row-task", ix))
                .flex_none()
                .h(px(22.0))
                .pl(px(6.0))
                .pr(px(8.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(4.0))
                .rounded_full()
                .border_1()
                .border_color(rgba(fade(th.text, 0.16)))
                .text_size(px(12.0))
                .text_color(rgba(color))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(fade(th.text, 0.08))))
                .tooltip(tip(tr!("row-task-open", title = task.title.clone()), th))
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .on_click(cx.listener(move |this, _, window, cx| {
                    cx.stop_propagation();
                    this.show_page(super::apps::App::Tasks, window, cx);
                    this.task_open_details(id, window, cx);
                }))
                .child(icon("tasks", color, 14.0))
                .child(label)
                .into_any_element(),
        )
    }

    fn load_tasks(&mut self, cx: &mut Context<Self>) {
        self.load_account_status(Of::Tasks, cx);
        let paths = self.paths.clone();
        self.tasks.loading = Some(cx.spawn(async move |this, cx| {
            let board = cx
                .background_executor()
                .spawn(async move { crate::tasks::load(&paths) })
                .await;
            this.update(cx, |this, cx| {
                let page = &mut this.tasks;
                if let (Ok(board), View::List(id)) = (&board, page.view)
                    && !board.columns.iter().any(|c| c.list.id == id)
                {
                    page.view = View::All;
                }
                // What the store shows now needs no pending marks.
                if let Ok(board) = &board {
                    page.pending.retain(|id, pending| {
                        board.task(*id).is_some_and(|task| {
                            pending.done.is_some_and(|d| d != task.done_at.is_some())
                                || pending.starred.is_some_and(|s| s != task.starred)
                        })
                    });
                }
                page.board = Some(board);
                page.reveal_list();
                this.map_task_mails();
                cx.notify();
            })
            .ok();
        }));
    }

    fn send_task(
        &mut self,
        command: TaskCommand,
        done: Option<String>,
        undo: Option<TaskCommand>,
        cx: &mut Context<Self>,
    ) {
        self.send(
            Command::Task(Box::new(command)),
            done,
            undo.map(|undo| Command::Task(Box::new(undo))),
            false,
            cx,
        );
    }

    fn task_set_view(&mut self, view: View, cx: &mut Context<Self>) {
        self.tasks.view = view;
        self.tasks.menu = None;
        self.tasks.adding = None;
        cx.notify();
    }

    // --- Changes -----------------------------------------------------------

    /// Opens task `id`'s details, once the tasks are read if they aren't
    /// yet (Katna Mail started from the desktop's search or a reminder).
    pub(super) fn task_open_when_read(
        &mut self,
        id: i64,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.tasks.board.is_some() {
            self.task_open_details(id, window, cx);
            return;
        }
        cx.spawn_in(window, async move |this, cx| {
            // At most a few seconds: reading tasks takes milliseconds.
            for _ in 0..100 {
                cx.background_executor()
                    .timer(std::time::Duration::from_millis(50))
                    .await;
                let read = this
                    .update_in(cx, |this, window, cx| {
                        let read = this.tasks.board.is_some();
                        if read {
                            this.task_open_details(id, window, cx);
                        }
                        read
                    })
                    .unwrap_or(true);
                if read {
                    break;
                }
            }
        })
        .detach();
    }

    pub(super) fn task_toggle_done(&mut self, id: i64, cx: &mut Context<Self>) {
        let Some(task) = self.tasks.task(id) else {
            return;
        };
        let done = !self.tasks.done(task);
        // A repeating task moves to its next day and stays open.
        if done && let Some((due, repeat)) = self.tasks.next_due(task) {
            let old = task.clone();
            let remind_at = katna_dav::todo::moved_reminder(
                old.remind_at,
                (&old.due, old.due_time),
                (&due, old.due_time),
                &jiff::tz::TimeZone::system(),
            );
            let date = due
                .parse::<jiff::civil::Date>()
                .map(|d| day_text(d, today()))
                .unwrap_or_default();
            // Shown at once; the store follows.
            if let Some(shown) = self.tasks.task_mut(id) {
                shown.due = due;
                shown.repeat = repeat;
                shown.remind_at = remind_at;
            }
            let text = tr!("tasks-toast-next", date = date);
            let undo = TaskCommand::Edit(id, TaskEdit::all_of(&old));
            self.send_task(TaskCommand::SetDone(id, true), Some(text), Some(undo), cx);
            cx.notify();
            return;
        }
        self.tasks.pending.entry(id).or_default().done = Some(done);
        if self.tasks.picked == Some(id) && done {
            self.tasks.picked = None;
        }
        let text = done.then(|| tr!("tasks-toast-done"));
        let undo = done.then_some(TaskCommand::SetDone(id, false));
        self.send_task(TaskCommand::SetDone(id, done), text, undo, cx);
        cx.notify();
    }

    /// Task `id` with whether it is ticked off and starred, counting
    /// changes not yet read back.
    pub(super) fn tasks_task(&self, id: i64) -> Option<(&TaskItem, bool, bool)> {
        let task = self.tasks.task(id)?;
        Some((task, self.tasks.done(task), self.tasks.starred(task)))
    }

    /// The tasks with a due day, as the Calendar shows them: ticked ones
    /// too, and whether each is ticked, counting ticks not yet read back.
    pub(super) fn dated_tasks(&self) -> Vec<(&TaskItem, bool)> {
        let Some(Ok(board)) = &self.tasks.board else {
            return Vec::new();
        };
        board
            .columns
            .iter()
            .flat_map(|c| c.tasks.iter())
            .filter(|t| !t.due.is_empty())
            .map(|t| (t, self.tasks.done(t)))
            .collect()
    }

    /// Gives task `id` a new due day and time (`None`: the whole day), as
    /// dragging it on the Calendar does; Undo puts them back.
    pub(super) fn task_move_due(
        &mut self,
        id: i64,
        due: String,
        time: Option<u32>,
        cx: &mut Context<Self>,
    ) {
        let Some(task) = self.tasks.task(id) else {
            return;
        };
        if task.due == due && task.due_time == time {
            return;
        }
        // A reminder moves with it, as long before as it was.
        let remind_at = task.remind_at.map(|_| {
            katna_dav::todo::moved_reminder(
                task.remind_at,
                (&task.due, task.due_time),
                (&due, time),
                &jiff::tz::TimeZone::system(),
            )
        });
        let undo = TaskCommand::Edit(
            id,
            TaskEdit {
                due: Some(task.due.clone()),
                due_time: Some(task.due_time),
                remind_at: remind_at.map(|_| task.remind_at),
                ..TaskEdit::default()
            },
        );
        let edit = TaskCommand::Edit(
            id,
            TaskEdit {
                due: Some(due),
                due_time: Some(time),
                remind_at,
                ..TaskEdit::default()
            },
        );
        self.send_task(edit, Some(tr!("tasks-toast-rescheduled")), Some(undo), cx);
    }

    pub(super) fn task_toggle_star(&mut self, id: i64, cx: &mut Context<Self>) {
        let Some(task) = self.tasks.task(id) else {
            return;
        };
        let starred = !self.tasks.starred(task);
        self.tasks.pending.entry(id).or_default().starred = Some(starred);
        let edit = |starred| {
            TaskCommand::Edit(
                id,
                TaskEdit {
                    starred: Some(starred),
                    ..TaskEdit::default()
                },
            )
        };
        self.send_task(edit(starred), None, None, cx);
        cx.notify();
    }

    pub(super) fn task_delete(&mut self, id: i64, cx: &mut Context<Self>) {
        let Some(Ok(board)) = &self.tasks.board else {
            return;
        };
        let Some(task) = board.task(id).cloned() else {
            return;
        };
        let steps = board.steps(id);
        // Gone from the page at once; the store follows.
        if let Some(Ok(board)) = &mut self.tasks.board {
            for column in &mut board.columns {
                column.tasks.retain(|t| t.id != id && t.parent != Some(id));
            }
        }
        if self.tasks.picked == Some(id) {
            self.tasks.picked = None;
        }
        self.tasks.menu = None;
        self.send_task(
            TaskCommand::Delete(id),
            Some(tr!("tasks-toast-deleted")),
            Some(TaskCommand::Restore { task, steps }),
            cx,
        );
        cx.notify();
    }

    fn move_task_to(&mut self, id: i64, list: i64, cx: &mut Context<Self>) {
        let Some(from) = self.tasks.task(id).map(|t| t.list) else {
            return;
        };
        self.tasks.menu = None;
        let name = self
            .tasks
            .columns()
            .iter()
            .find(|c| c.list.id == list)
            .map(list_title)
            .unwrap_or_default();
        self.send_task(
            TaskCommand::Move(id, list),
            Some(tr!("tasks-toast-moved", list = name)),
            Some(TaskCommand::Move(id, from)),
            cx,
        );
        cx.notify();
    }

    /// A dragged task moved over list `list`'s card (`card`, drawn there):
    /// the gap follows the pointer, between the tasks it is between.
    fn task_drag_over(
        &mut self,
        list: i64,
        card: Bounds<Pixels>,
        at: Point<Pixels>,
        dragged: &TaskDragged,
        cx: &mut Context<Self>,
    ) {
        let page = &mut self.tasks;
        let slots = page.slots(list, dragged.id);
        let drag = page.drag_of(dragged);
        if drag.rows.is_empty() {
            // Not a row seen yet: the gap stays where the task was.
            return;
        }
        let to = if card.contains(&at) {
            let slot = slots
                .iter()
                .filter(|t| drag.rows.get(t).is_some_and(|b| b.center().y < at.y))
                .count();
            Some((list, slot))
        } else if drag.to.is_some_and(|(l, _)| l == list) {
            // Out of this list, over none yet.
            None
        } else {
            return;
        };
        if page.drag_to(dragged, to) {
            cx.notify();
        }
    }

    /// Where task `task`'s row, with its steps, is drawn while a task is
    /// dragged.
    fn task_drag_row(&mut self, task: i64, row: Bounds<Pixels>, dragged: &TaskDragged) {
        let drag = self.tasks.drag_of(dragged);
        if task == dragged.id {
            drag.height = katna_ui::unpx(row.size.height);
        }
        drag.rows.insert(task, row);
    }

    /// The dragged task was let go on list `list`: it goes where the gap
    /// is, at once here, and Undo puts it back where it was.
    fn task_drop(&mut self, list: i64, dragged: &TaskDragged, cx: &mut Context<Self>) {
        let Some(drag) = self.tasks.drag.take() else {
            return;
        };
        let Some((to_list, slot)) = drag.to.filter(|(l, _)| *l == list) else {
            return;
        };
        if drag.id != dragged.id || (to_list, slot) == drag.from {
            cx.notify();
            return;
        }
        let id = drag.id;
        let Some(task) = self.tasks.task(id).cloned() else {
            return;
        };
        let slots = self.tasks.slots(list, id);
        let after = slot.checked_sub(1).and_then(|ix| slots.get(ix).copied());
        // Where it was: after the task before it in its list, done ones
        // and those the search hides too.
        let before = self
            .tasks
            .columns()
            .iter()
            .filter(|c| c.list.id == task.list)
            .flat_map(|c| c.tasks.iter())
            .filter(|t| t.parent.is_none())
            .take_while(|t| t.id != id)
            .last()
            .map(|t| t.id);
        let from = task.list;
        // Shown there at once; the store follows.
        if let Some(Ok(board)) = &mut self.tasks.board {
            let mut moving = Vec::new();
            for column in &mut board.columns {
                column.tasks.retain(|t| {
                    let ours = t.id == id || t.parent == Some(id);
                    if ours {
                        moving.push(t.clone());
                    }
                    !ours
                });
            }
            for t in &mut moving {
                t.list = list;
            }
            if let Some(column) = board.columns.iter_mut().find(|c| c.list.id == list) {
                let at = match after {
                    None => 0,
                    Some(after) => {
                        column
                            .tasks
                            .iter()
                            .position(|t| t.id == after)
                            .map_or(0, |ix| {
                                // After its steps too.
                                ix + 1
                                    + column.tasks[ix + 1..]
                                        .iter()
                                        .take_while(|t| t.parent == Some(after))
                                        .count()
                            })
                    }
                };
                column.tasks.splice(at..at, moving);
            }
        }
        let text = if from == list {
            tr!("tasks-toast-placed")
        } else {
            let name = self
                .tasks
                .columns()
                .iter()
                .find(|c| c.list.id == list)
                .map(list_title)
                .unwrap_or_default();
            tr!("tasks-toast-moved", list = name)
        };
        self.send_task(
            TaskCommand::Place { id, list, after },
            Some(text),
            Some(TaskCommand::Place {
                id,
                list: from,
                after: before,
            }),
            cx,
        );
        cx.notify();
    }

    fn task_start_adding(
        &mut self,
        list: i64,
        parent: Option<i64>,
        due: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let accent = rgba(self.theme(window).accent).into();
        let placeholder = if parent.is_some() {
            tr!("tasks-add-step")
        } else {
            tr!("tasks-title-placeholder")
        };
        let input = cx.new(|cx| {
            let mut input = TextInput::new(placeholder, cx);
            input.set_accent(accent);
            input
        });
        let subscription = cx.subscribe_in(
            &input,
            window,
            |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Submit => this.task_finish_adding(true, window, cx),
                InputEvent::Cancel => this.task_finish_adding(false, window, cx),
                // What the words say shows under the row as it is typed.
                InputEvent::Changed => cx.notify(),
            },
        );
        window.focus(&input.focus_handle(cx), cx);
        self.tasks.editing = None;
        self.tasks.adding = Some(Adding {
            list,
            parent,
            due,
            input,
            _subscription: subscription,
        });
        cx.notify();
    }

    /// Adds the typed task (`save`); Enter keeps the row open for the next
    /// one, as Google Tasks does.
    fn task_finish_adding(&mut self, save: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(adding) = &self.tasks.adding else {
            return;
        };
        let title = adding.input.read(cx).text().trim().to_owned();
        if !save || title.is_empty() {
            self.tasks.adding = None;
            if let Some(focus) = &self.tasks.focus {
                window.focus(focus, cx);
            }
            cx.notify();
            return;
        }
        // "Pay rent every month on the 1st": the day, time and repeat come
        // from the words, as in Todoist.
        let typed = typed_task(&title, today());
        let (title, due, fields) = match typed {
            Some(typed) => {
                let due = typed.due.unwrap_or_else(|| adding.due.clone());
                let fields = TaskEdit {
                    due_time: typed.due_time.map(Some),
                    repeat: typed.repeat,
                    ..TaskEdit::default()
                };
                (typed.title, due, fields)
            }
            None => (title, adding.due.clone(), TaskEdit::default()),
        };
        let add = TaskCommand::Add {
            list: adding.list,
            parent: adding.parent,
            title,
            due,
            mail: String::new(),
        };
        let command = if fields == TaskEdit::default() {
            add
        } else {
            TaskCommand::AddThen(Box::new(add), fields)
        };
        adding.input.update(cx, |input, cx| input.set_text("", cx));
        self.send_task(command, None, None, cx);
        cx.notify();
    }

    fn task_start_editing(&mut self, id: i64, window: &mut Window, cx: &mut Context<Self>) {
        let Some(task) = self.tasks.task(id) else {
            return;
        };
        let text = task.title.clone();
        let accent = rgba(self.theme(window).accent).into();
        let input = cx.new(|cx| {
            let mut input = TextInput::new(tr!("tasks-title-placeholder"), cx);
            input.set_accent(accent);
            input.set_text(text, cx);
            input
        });
        let subscription = cx.subscribe_in(
            &input,
            window,
            |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Submit => this.task_finish_editing(true, window, cx),
                InputEvent::Cancel => this.task_finish_editing(false, window, cx),
                InputEvent::Changed => {}
            },
        );
        window.focus(&input.focus_handle(cx), cx);
        self.tasks.adding = None;
        self.tasks.picked = Some(id);
        self.tasks.editing = Some(Editing {
            id,
            input,
            _subscription: subscription,
        });
        cx.notify();
    }

    fn task_finish_editing(&mut self, save: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(editing) = self.tasks.editing.take() else {
            return;
        };
        if let Some(focus) = &self.tasks.focus {
            window.focus(focus, cx);
        }
        cx.notify();
        let title = editing.input.read(cx).text().trim().to_owned();
        let Some(task) = self.tasks.task(editing.id) else {
            return;
        };
        if !save || title.is_empty() || title == task.title {
            return;
        }
        let old = task.title.clone();
        let id = editing.id;
        // Shown at once; the store follows.
        if let Some(Ok(board)) = &mut self.tasks.board
            && let Some(task) = board
                .columns
                .iter_mut()
                .flat_map(|c| c.tasks.iter_mut())
                .find(|t| t.id == id)
        {
            task.title = title.clone();
        }
        let edit = |title: String| {
            TaskCommand::Edit(
                id,
                TaskEdit {
                    title: Some(title),
                    ..TaskEdit::default()
                },
            )
        };
        self.send_task(edit(title), None, None, cx);
        // Ctrl+Z takes the old title back.
        self.remember(super::UndoStep::Command(Command::Task(Box::new(edit(old)))));
    }

    fn task_start_naming(
        &mut self,
        list: Option<i64>,
        account: Option<AccountId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let text = list
            .and_then(|id| self.tasks.columns().iter().find(|c| c.list.id == id))
            .map(list_title)
            .unwrap_or_default();
        let accent = rgba(self.theme(window).accent).into();
        let input = cx.new(|cx| {
            let mut input = TextInput::new(tr!("tasks-list-name-placeholder"), cx);
            input.set_accent(accent);
            input.set_text(text, cx);
            input.select_all_text(cx);
            input
        });
        let subscription = cx.subscribe_in(
            &input,
            window,
            |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Submit => this.task_finish_naming(true, window, cx),
                InputEvent::Cancel => this.task_finish_naming(false, window, cx),
                InputEvent::Changed => {}
            },
        );
        window.focus(&input.focus_handle(cx), cx);
        self.tasks.menu = None;
        self.tasks.naming = Some(Naming {
            list,
            account,
            input,
            _subscription: subscription,
        });
        cx.notify();
    }

    fn task_finish_naming(&mut self, save: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(naming) = self.tasks.naming.take() else {
            return;
        };
        if let Some(focus) = &self.tasks.focus {
            window.focus(focus, cx);
        }
        cx.notify();
        let title = naming.input.read(cx).text().trim().to_owned();
        if !save || title.is_empty() {
            return;
        }
        match naming.list {
            Some(list) => {
                let old = self
                    .tasks
                    .columns()
                    .iter()
                    .find(|c| c.list.id == list)
                    .map(|c| c.list.title.clone());
                if old.as_deref() == Some(title.as_str()) {
                    return;
                }
                let undo = old.map(|old| TaskCommand::RenameList(list, old));
                self.send_task(TaskCommand::RenameList(list, title), None, None, cx);
                if let Some(undo) = undo {
                    self.remember(super::UndoStep::Command(Command::Task(Box::new(undo))));
                }
            }
            None => {
                let known = self.tasks.columns().iter().map(|c| c.list.id).collect();
                self.tasks.reveal_new = Some(known);
                self.send_task(TaskCommand::AddList(naming.account, title), None, None, cx)
            }
        }
    }

    fn task_delete_list(&mut self, list: i64, cx: &mut Context<Self>) {
        self.tasks.menu = None;
        if self.tasks.view == View::List(list) {
            self.tasks.view = View::All;
        }
        if let Some(Ok(board)) = &mut self.tasks.board {
            board.columns.retain(|c| c.list.id != list);
        }
        // A list with its tasks can't come back from the service: no Undo.
        self.send_task(
            TaskCommand::DeleteList(list),
            Some(tr!("tasks-toast-list-deleted")),
            None,
            cx,
        );
        cx.notify();
    }

    // --- From mail -----------------------------------------------------------

    /// Add to Tasks (Shift+T, as in Gmail): a task for each picked line in
    /// the default list, titled with its subject, leading back to the mail.
    pub(super) fn add_to_tasks(
        &mut self,
        _: &super::AddToTasks,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let keys = self.target_keys();
        self.add_to_tasks_from(keys, cx);
    }

    pub(super) fn add_to_tasks_from(&mut self, keys: Vec<EntryKey>, cx: &mut Context<Self>) {
        let Ok(mail) = self.mail.as_ref() else {
            return;
        };
        let sources: Vec<(String, String)> =
            keys.iter().filter_map(|k| mail.task_source(*k)).collect();
        let count = sources.len();
        let mails: Vec<String> = sources.iter().map(|(_, m)| m.clone()).collect();
        for (ix, (subject, header)) in sources.into_iter().enumerate() {
            let title = if subject.is_empty() {
                tr!("tasks-no-subject")
            } else {
                subject
            };
            // One note and one Undo for them all, with the last.
            let last = ix + 1 == count;
            self.send_task(
                TaskCommand::Add {
                    list: 0,
                    parent: None,
                    title,
                    due: String::new(),
                    mail: header,
                },
                last.then(|| tr!("tasks-toast-added", count = count as u64)),
                last.then(|| TaskCommand::RemoveFromMail(mails.clone())),
                cx,
            );
        }
    }

    /// Opens the mail a task was made from.
    fn open_task_mail(&mut self, header: &str, window: &mut Window, cx: &mut Context<Self>) {
        let found = self
            .mail
            .as_ref()
            .ok()
            .and_then(|m| m.message_with_header(header));
        match found {
            Some(message) => {
                self.open_app(super::apps::App::Mail, cx);
                self.show_message(message, window, cx);
            }
            None => self.show_snackbar(tr!("tasks-mail-gone"), None, cx),
        }
    }

    // --- Keys ----------------------------------------------------------------

    fn tasks_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        // Esc with the focus away from an empty new task or list name
        // box puts it away, as Esc in the box does.
        if event.keystroke.key == "escape" && !event.keystroke.modifiers.modified() {
            if self.tasks.naming.is_some() {
                self.task_finish_naming(false, window, cx);
                cx.stop_propagation();
                return;
            }
            if self.tasks.adding.is_some() {
                self.task_finish_adding(false, window, cx);
                cx.stop_propagation();
                return;
            }
        }
        if self.tasks.adding.is_some()
            || self.tasks.editing.is_some()
            || self.tasks.naming.is_some()
            || self.tasks.details.is_some()
        {
            return;
        }
        let keys = &event.keystroke;
        if keys.modifiers.control || keys.modifiers.alt || keys.modifiers.platform {
            return;
        }
        let shown = self.tasks.shown();
        let at = self
            .tasks
            .picked
            .and_then(|id| shown.iter().position(|s| *s == id));
        match keys.key.as_str() {
            "down" | "up" => {
                let next = match (at, keys.key.as_str()) {
                    (None, _) => 0,
                    (Some(i), "down") => (i + 1).min(shown.len().saturating_sub(1)),
                    (Some(i), _) => i.saturating_sub(1),
                };
                self.tasks.picked = shown.get(next).copied();
            }
            "space" => {
                if let Some(id) = self.tasks.picked {
                    // The next one is picked once this one folds away.
                    let next = at.and_then(|i| {
                        shown
                            .get(i + 1)
                            .or(i.checked_sub(1).and_then(|p| shown.get(p)))
                    });
                    let next = next.copied();
                    self.task_toggle_done(id, cx);
                    self.tasks.picked = next;
                }
            }
            "enter" => {
                if let Some(id) = self.tasks.picked {
                    self.task_open_details(id, window, cx);
                }
            }
            "f2" => {
                if let Some(id) = self.tasks.picked {
                    self.task_start_editing(id, window, cx);
                }
            }
            "delete" => {
                if let Some(id) = self.tasks.picked {
                    let next = at.and_then(|i| {
                        shown
                            .get(i + 1)
                            .or(i.checked_sub(1).and_then(|p| shown.get(p)))
                    });
                    let next = next.copied();
                    self.task_delete(id, cx);
                    self.tasks.picked = next;
                }
            }
            "escape" => {
                self.tasks.picked = None;
                self.tasks.menu = None;
            }
            _ => return,
        }
        cx.stop_propagation();
        cx.notify();
    }

    // --- Drawing -------------------------------------------------------------

    pub(super) fn render_tasks(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let page = &self.tasks;
        let body = match &page.board {
            None => self.placeholder(tr!("tasks-loading"), th),
            Some(Err(err)) => self.placeholder(err.clone(), th),
            Some(Ok(_)) => self.render_task_board(th, cx),
        };
        let menu = self.render_tasks_menu(th, cx);
        let details = self.render_task_details(th, cx);
        let side = self.render_tasks_nav(th, cx);
        let side = self.page_side(side, NAV_WIDTH, true, th, cx);
        div()
            .id("tasks-page")
            .relative()
            .size_full()
            .flex()
            .flex_row()
            .when_some(page.focus.as_ref(), |d, focus| d.track_focus(focus))
            .on_key_down(cx.listener(Self::tasks_key))
            .children(side.docked)
            .child(div().flex_1().min_w_0().h_full().child(body))
            .children(side.drawer)
            .children(menu)
            .children(details)
            .with_animation(
                "tasks-page-in",
                Animation::new(katna_ui::motion::time(std::time::Duration::from_millis(
                    220,
                )))
                .with_easing(ease_out_quint()),
                |el, t| el.opacity(t),
            )
            .into_any_element()
    }

    /// The left bar's New task: a new task in the open list, or due
    /// today in the default list from Today.
    pub(super) fn tasks_create(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let list = match self.tasks.view {
            View::List(id) => Some(id),
            View::Today => {
                let due = today().to_string();
                return self.task_start_adding(0, None, due, window, cx);
            }
            _ => self.tasks.columns().first().map(|c| c.list.id),
        };
        if let Some(list) = list {
            self.task_start_adding(list, None, String::new(), window, cx);
        }
    }

    fn render_tasks_nav(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let page = &self.tasks;
        let row = |id: SharedString, icon_name: &'static str, label: String, on: bool| {
            super::nav::side_row(id, icon_name, label, on, th)
        };
        let open_count = |column: &Column| {
            column
                .tasks
                .iter()
                .filter(|t| !page.done(t) && t.parent.is_none())
                .count()
        };
        let starred = page
            .columns()
            .iter()
            .flat_map(|c| c.tasks.iter())
            .filter(|t| page.starred(t) && !page.done(t))
            .count();
        let due_now = {
            let (overdue, due) = page.due_now(today());
            overdue.len() + due.len()
        };
        // Rows keep their heights: a scrolling flex column would squeeze
        // them first when there are many lists and accounts.
        let mut nav = div()
            .flex_none()
            .pb(px(16.0))
            .flex()
            .flex_col()
            .child(
                row(
                    "tasks-all".into(),
                    "tasks",
                    tr!("tasks-all"),
                    page.view == View::All,
                )
                .on_click(cx.listener(|this, _, _, cx| this.task_set_view(View::All, cx))),
            )
            .child(
                row(
                    "tasks-today".into(),
                    "today",
                    tr!("tasks-today"),
                    page.view == View::Today,
                )
                .when(due_now > 0, |d| {
                    d.child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_faint))
                            .child(katna_i18n::format::number(due_now as u64)),
                    )
                })
                .on_click(cx.listener(|this, _, _, cx| this.task_set_view(View::Today, cx))),
            )
            .child(
                row(
                    "tasks-starred".into(),
                    "star",
                    tr!("tasks-starred"),
                    page.view == View::Starred,
                )
                .when(starred > 0, |d| {
                    d.child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_faint))
                            .child(katna_i18n::format::number(starred as u64)),
                    )
                })
                .on_click(cx.listener(|this, _, _, cx| this.task_set_view(View::Starred, cx))),
            )
            .child(
                div()
                    .mt(px(16.0))
                    .mx(px(24.0))
                    .mb(px(4.0))
                    .h(px(1.0))
                    .bg(rgba(th.divider)),
            );
        // Lists by account, Mailspring-style: the address, then its lists.
        // Every account shows, also one whose lists did not come, with the
        // reason under it.
        let mut groups: Vec<(Option<AccountId>, String, Vec<&Column>)> = Vec::new();
        for column in page.columns() {
            match groups
                .iter_mut()
                .find(|(k, _, _)| *k == column.list.account)
            {
                Some((_, _, columns)) => columns.push(column),
                None => groups.push((column.list.account, column.account.clone(), vec![column])),
            }
        }
        // A local account keeps its lists on this computer.
        let accounts = self
            .accounts
            .iter()
            .filter(|a| a.kind != AccountKind::Local);
        for account in accounts {
            if !groups.iter().any(|(k, _, _)| *k == Some(account.id)) {
                groups.push((Some(account.id), account.address.clone(), Vec::new()));
            }
        }
        for (account, address, columns) in groups {
            let heading = if address.is_empty() {
                tr!("tasks-on-this-computer")
            } else {
                address
            };
            nav = nav.child(
                div()
                    .mt(px(12.0))
                    .mb(px(4.0))
                    .px(px(24.0))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(th.text_faint))
                    .truncate()
                    .child(heading),
            );
            let note = account
                .map(|a| a.0)
                .filter(|&id| columns.is_empty() || page.accounts.failing(id));
            nav = nav.children(note.map(|id| self.render_account_status(Of::Tasks, id, th, cx)));
            for column in columns {
                let id = column.list.id;
                let count = open_count(column);
                let naming_this = page.naming.as_ref().is_some_and(|n| n.list == Some(id));
                if naming_this {
                    nav = nav.children(page.naming.as_ref().map(|n| self.naming_row(n, th, cx)));
                    continue;
                }
                nav = nav.child(
                    row(
                        format!("tasks-list-{id}").into(),
                        "list-bulleted",
                        list_title(column),
                        page.view == View::List(id),
                    )
                    .when(count > 0, |d| {
                        d.child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgba(th.text_faint))
                                .child(katna_i18n::format::number(count as u64)),
                        )
                    })
                    .on_click(
                        cx.listener(move |this, _, _, cx| this.task_set_view(View::List(id), cx)),
                    ),
                );
            }
        }
        let new_list = page.naming.as_ref().filter(|n| n.list.is_none());
        nav = nav.children(new_list.map(|n| self.naming_row(n, th, cx)));
        // A new list goes to the account of the list in view, else the
        // first account's.
        let account = match page.view {
            View::List(id) => page
                .columns()
                .iter()
                .find(|c| c.list.id == id)
                .and_then(|c| c.list.account),
            _ => page.columns().iter().find_map(|c| c.list.account),
        };
        nav = nav.child(
            row("tasks-new-list".into(), "add", tr!("tasks-new-list"), false).on_click(
                cx.listener(move |this, _, window, cx| {
                    this.task_start_naming(None, account, window, cx)
                }),
            ),
        );
        div()
            .id("tasks-nav")
            .flex_none()
            .w(px(NAV_WIDTH))
            .h_full()
            .overflow_y_scroll()
            .child(nav)
            .into_any_element()
    }

    fn naming_row(&self, naming: &Naming, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        div()
            // A click elsewhere with no name typed puts the box away, as
            // Esc does; a typed name waits for Enter.
            .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                let empty = this
                    .tasks
                    .naming
                    .as_ref()
                    .is_some_and(|n| n.input.read(cx).text().trim().is_empty());
                if empty {
                    this.task_finish_naming(false, window, cx);
                }
            }))
            .mx(px(8.0))
            .h(px(40.0))
            .px(px(16.0))
            .flex()
            .items_center()
            .rounded_full()
            .border_1()
            .border_color(rgba(th.accent))
            .text_size(px(14.0))
            .child(div().flex_1().child(naming.input.clone()))
            .into_any_element()
    }

    fn render_task_board(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let page = &self.tasks;
        let columns = page.shown_columns();
        if columns.is_empty() && page.searching() {
            return self.placeholder(tr!("tasks-search-none"), th);
        }
        if columns.is_empty() {
            return self.placeholder(tr!("tasks-no-lists"), th);
        }
        match page.view {
            // As many lists side by side as fit, then more rows below,
            // scrolling down: none is ever out of reach to the right.
            View::All => div()
                .id("tasks-board")
                .size_full()
                .overflow_y_scroll()
                .track_scroll(&page.board_scroll)
                .p(px(16.0))
                .flex()
                .flex_row()
                .flex_wrap()
                .items_start()
                .content_start()
                .gap(px(16.0))
                .children(
                    columns
                        .into_iter()
                        .map(|c| self.render_task_card(c, CARD_WIDTH, th, cx)),
                )
                .into_any_element(),
            View::List(_) => div()
                .size_full()
                .p(px(16.0))
                .flex()
                .items_start()
                .justify_center()
                .children(
                    columns
                        .into_iter()
                        .map(|c| self.render_task_card(c, SINGLE_WIDTH, th, cx)),
                )
                .into_any_element(),
            View::Starred => self.render_starred(columns, th, cx),
            View::Today => self.render_today(th, cx),
        }
    }

    /// A card of tasks, `width` wide, that scrolls once taller than the
    /// page. `rows` keep their heights: in a column that scrolls, a flex
    /// column would squeeze them first.
    fn card_frame(
        &self,
        id: SharedString,
        width: f32,
        th: &Theme,
        rows: gpui::Div,
    ) -> gpui::Stateful<gpui::Div> {
        // No wider than the page, less the board's margins: on a phone a
        // card fills it.
        let shape = self.layout.shape;
        let side = self.page_side_width(NAV_WIDTH);
        let room = shape.width - shape.rail() - side - shape.card_margin() - 32.0;
        div()
            .id(id)
            .flex_none()
            .w(px(width.min(room)))
            .max_h_full()
            .overflow_y_scroll()
            .rounded(px(16.0))
            .bg(rgba(if th.dark { th.read_row } else { th.surface }))
            .border_1()
            .border_color(rgba(th.outline))
            .child(rows.flex_none().pb(px(8.0)).flex().flex_col())
    }

    fn render_starred(
        &self,
        columns: Vec<&Column>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let page = &self.tasks;
        let today = today();
        let rows: Vec<AnyElement> = columns
            .iter()
            .flat_map(|c| c.tasks.iter().map(move |t| (*c, t)))
            .filter(|(_, t)| page.starred(t) && !page.done(t) && page.found(t))
            .map(|(c, t)| self.render_task_row(t, Some(&list_title(c)), today, th, cx))
            .collect();
        let empty = rows.is_empty();
        div()
            .size_full()
            .p(px(16.0))
            .flex()
            .items_start()
            .justify_center()
            .child(
                self.card_frame(
                    "tasks-starred-card".into(),
                    SINGLE_WIDTH,
                    th,
                    div()
                        .child(card_heading(tr!("tasks-starred"), th))
                        .when(empty, |d| {
                            d.child(
                                div()
                                    .py(px(32.0))
                                    .px(px(24.0))
                                    .text_center()
                                    .text_size(px(14.0))
                                    .text_color(rgba(th.text_faint))
                                    .child(tr!("tasks-starred-empty")),
                            )
                        })
                        .children(rows),
                ),
            )
            .into_any_element()
    }

    /// Today: what is overdue, then what is due today, from every list,
    /// with a row to add a task due today.
    fn render_today(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let page = &self.tasks;
        let day = today();
        let (mut overdue, mut due) = page.due_now(day);
        overdue.retain(|(_, t)| page.found(t));
        due.retain(|(_, t)| page.found(t));
        // A task's row, and the row adding a step to it.
        let mut row = |(c, t): (&Column, &TaskItem)| {
            let mut rows = vec![self.render_task_row(t, Some(&list_title(c)), day, th, cx)];
            if let Some(adding) = page.adding.as_ref().filter(|a| a.parent == Some(t.id)) {
                rows.push(self.adding_row(adding, true, th, cx));
            }
            rows
        };
        let section = |label: String, color: u32| {
            div()
                .mt(px(8.0))
                .px(px(20.0))
                .h(px(32.0))
                .flex()
                .items_center()
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgba(color))
                .child(label)
        };
        let empty = overdue.is_empty() && due.is_empty();
        let has_overdue = !overdue.is_empty();
        let overdue_rows: Vec<AnyElement> = overdue.into_iter().flat_map(&mut row).collect();
        let due_rows: Vec<AnyElement> = due.into_iter().flat_map(&mut row).collect();
        let at = day.to_datetime(jiff::civil::Time::midnight());
        let date = tr!(
            "tasks-today-date",
            weekday = katna_i18n::format::weekday(at),
            day = katna_i18n::format::day_month(at)
        );
        let adding = page
            .adding
            .as_ref()
            .filter(|a| a.parent.is_none() && !a.due.is_empty());
        let add = match adding {
            Some(adding) => self.adding_row(adding, false, th, cx),
            None => self
                .add_row("tasks-today-add".into(), th)
                .on_click(cx.listener(|this, _, window, cx| {
                    this.task_start_adding(0, None, today().to_string(), window, cx)
                }))
                .into_any_element(),
        };
        div()
            .size_full()
            .p(px(16.0))
            .flex()
            .items_start()
            .justify_center()
            .child(
                self.card_frame(
                    "tasks-today-card".into(),
                    SINGLE_WIDTH,
                    th,
                    div()
                        .child(card_heading(tr!("tasks-today"), th))
                        .child(
                            div()
                                .px(px(20.0))
                                .mt(px(-8.0))
                                .mb(px(4.0))
                                .text_size(px(12.0))
                                .text_color(rgba(th.text_faint))
                                .child(date),
                        )
                        .child(add)
                        .when(empty, |d| {
                            d.child(
                                div()
                                    .py(px(24.0))
                                    .px(px(24.0))
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .gap(px(8.0))
                                    .text_center()
                                    .child(icon("check-circle", th.text_faint, 40.0))
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .text_color(rgba(th.text_dim))
                                            .child(tr!("tasks-today-empty")),
                                    ),
                            )
                        })
                        .when(has_overdue, |d| {
                            d.child(section(tr!("tasks-overdue"), th.error))
                                .children(overdue_rows)
                                .when(!due_rows.is_empty(), |d| {
                                    d.child(section(tr!("tasks-due-today"), th.text_dim))
                                })
                        })
                        .children(due_rows),
                ),
            )
            .into_any_element()
    }

    /// The "Add a task" row at the top of a card.
    fn add_row(&self, id: gpui::ElementId, th: &Theme) -> gpui::Stateful<gpui::Div> {
        div()
            .id(id)
            .h(px(40.0))
            .px(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(16.0))
            .cursor_pointer()
            .text_size(px(14.0))
            .text_color(rgba(th.accent))
            .hover(|s| s.bg(rgba(th.hover)))
            .child(
                div()
                    .size(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(icon("add", th.accent, 20.0)),
            )
            .child(tr!("tasks-add"))
    }

    fn render_task_card(
        &self,
        column: &Column,
        width: f32,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let page = &self.tasks;
        let id = column.list.id;
        let today = today();
        let (done, open): (Vec<&TaskItem>, Vec<&TaskItem>) = column
            .tasks
            .iter()
            .filter(|t| page.parent_open(t) || !page.done(t))
            .filter(|t| page.found(t))
            .partition(|t| page.done(t));
        // Open tasks, each with its steps; a step whose task is done shows
        // among the done.
        let mut groups: Vec<(Option<i64>, Vec<AnyElement>)> = Vec::new();
        for t in &open {
            if t.parent.is_none() || groups.is_empty() {
                groups.push((t.parent.is_none().then_some(t.id), Vec::new()));
            }
            let rows = &mut groups.last_mut().expect("just pushed").1;
            rows.push(self.render_task_row(t, None, today, th, cx));
            if let Some(adding) = page.adding.as_ref().filter(|a| a.parent == Some(t.id)) {
                rows.push(self.adding_row(adding, true, th, cx));
            }
        }
        let drag = page.drag.as_ref().filter(|_| cx.has_active_drag());
        let mut open_rows: Vec<AnyElement> = Vec::new();
        let mut slot = 0;
        for (task, rows) in groups {
            let Some(task) = task else {
                open_rows.extend(rows);
                continue;
            };
            if let Some(drag) = drag {
                // The dragged task leaves its place; a gap follows the
                // pointer.
                if task == drag.id {
                    continue;
                }
                open_rows.extend(drag_gaps(drag, id, slot, th));
            }
            slot += 1;
            // Each task with its steps says where it is to a drag.
            open_rows.push(
                div()
                    .id(("task-group", task as usize))
                    .flex()
                    .flex_col()
                    .on_drag_move(cx.listener(
                        move |this, event: &gpui::DragMoveEvent<TaskDragged>, _, cx| {
                            let dragged = event.drag(cx).clone();
                            this.task_drag_row(task, event.bounds, &dragged);
                        },
                    ))
                    .children(rows)
                    .into_any_element(),
            );
        }
        if let Some(drag) = drag {
            open_rows.extend(drag_gaps(drag, id, slot, th));
        }
        let done_open = page.open_done.contains(&id);
        let done_count = done.len();
        let done_rows: Vec<AnyElement> = if done_open {
            done.iter()
                .map(|t| self.render_task_row(t, None, today, th, cx))
                .collect()
        } else {
            Vec::new()
        };
        let adding_top = page
            .adding
            .as_ref()
            .filter(|a| a.list == id && a.parent.is_none());
        let empty = open_rows.is_empty() && adding_top.is_none();
        let heading = card_heading(list_title(column), th).child(
            div()
                .id(("tasks-list-menu", id as usize))
                .size(px(32.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .tooltip(tip(tr!("tasks-list-options"), th))
                .child(icon("more", th.text_dim, 20.0))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, event: &gpui::MouseDownEvent, _, cx| {
                        this.tasks.menu = Some(Menu::List {
                            list: id,
                            at: event.position,
                        });
                        cx.stop_propagation();
                        cx.notify();
                    }),
                ),
        );
        let add = match adding_top {
            Some(adding) => self.adding_row(adding, false, th, cx),
            None => self
                .add_row(("tasks-add", id as usize).into(), th)
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.task_start_adding(id, None, String::new(), window, cx)
                }))
                .into_any_element(),
        };
        self.card_frame(
            format!("tasks-card-{id}").into(),
            width,
            th,
            div()
                .child(heading)
                .when(!column.account.is_empty() && page.view != View::All, |d| {
                    d.child(
                        div()
                            .px(px(20.0))
                            .mt(px(-8.0))
                            .mb(px(4.0))
                            .text_size(px(12.0))
                            .text_color(rgba(th.text_faint))
                            .child(column.account.clone()),
                    )
                })
                .child(add)
                .when(empty, |d| {
                    d.child(
                        div()
                            .py(px(24.0))
                            .px(px(24.0))
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(8.0))
                            .text_center()
                            .child(icon("check-circle", th.text_faint, 40.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(rgba(th.text_dim))
                                    .child(tr!("tasks-empty")),
                            ),
                    )
                })
                .children(open_rows)
                .when(done_count > 0, |d| {
                    d.child(
                        div()
                            .id(("tasks-done-fold", id as usize))
                            .mt(px(4.0))
                            .h(px(40.0))
                            .px(px(16.0))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(12.0))
                            .border_t_1()
                            .border_color(rgba(th.divider))
                            .cursor_pointer()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.text_dim))
                            .hover(|s| s.bg(rgba(th.hover)))
                            .child(icon(
                                if done_open {
                                    "chevron-down"
                                } else {
                                    "chevron-right"
                                },
                                th.text_dim,
                                20.0,
                            ))
                            .child(tr!("tasks-completed", count = done_count as u64))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if !this.tasks.open_done.remove(&id) {
                                    this.tasks.open_done.insert(id);
                                }
                                cx.notify();
                            })),
                    )
                })
                .children(done_rows),
        )
        // A task dragged here, from this list or another, lands where the
        // gap opened.
        .drag_over::<TaskDragged>({
            let accent = th.accent;
            move |style, dragged, _, _| {
                if dragged.list == id {
                    style
                } else {
                    style.border_color(rgba(accent))
                }
            }
        })
        .on_drag_move(cx.listener(
            move |this, event: &gpui::DragMoveEvent<TaskDragged>, _, cx| {
                let dragged = event.drag(cx).clone();
                this.task_drag_over(id, event.bounds, event.event.position, &dragged, cx);
            },
        ))
        .on_drop(cx.listener(move |this, dragged: &TaskDragged, _, cx| {
            this.task_drop(id, dragged, cx);
        }))
        .into_any_element()
    }

    fn adding_row(
        &self,
        adding: &Adding,
        step: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let today = today();
        // What typed quick add understood, as the task's chip will say it.
        let understood = typed_task(adding.input.read(cx).text(), today).and_then(|typed| {
            let shown = TaskItem {
                due: typed.due?,
                due_time: typed.due_time,
                ..TaskItem::default()
            };
            let (label, _) = due_label(&shown, today)?;
            Some((label, typed.repeat.is_some()))
        });
        div()
            // A click elsewhere with nothing typed puts the new task away.
            .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                let empty = this
                    .tasks
                    .adding
                    .as_ref()
                    .is_some_and(|a| a.input.read(cx).text().trim().is_empty());
                if empty {
                    this.task_finish_adding(false, window, cx);
                }
            }))
            .min_h(px(44.0))
            .pl(px(if step { 48.0 } else { 16.0 }))
            .pr(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(16.0))
            .bg(rgba(fade(th.accent, 0.06)))
            .child(round_tick(false, false, th))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .text_size(px(14.0))
                    .child(adding.input.clone())
                    .children(understood.map(|(label, repeats)| {
                        div()
                            .pb(px(6.0))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(4.0))
                            .text_size(px(12.0))
                            .text_color(rgba(th.accent))
                            .child(icon("calendar", th.accent, 14.0))
                            .child(label)
                            .when(repeats, |d| d.child(icon("refresh", th.accent, 14.0)))
                    })),
            )
            .into_any_element()
    }

    fn render_task_row(
        &self,
        task: &TaskItem,
        list: Option<&str>,
        today: jiff::civil::Date,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let page = &self.tasks;
        let id = task.id;
        let done = page.done(task);
        let starred = page.starred(task);
        let picked = page.picked == Some(id);
        let step = task.parent.is_some() && !done;
        let editing = page.editing.as_ref().filter(|e| e.id == id);
        let due = due_label(task, today);
        let title: AnyElement = match editing {
            Some(editing) => div()
                .text_size(px(14.0))
                .child(editing.input.clone())
                .into_any_element(),
            None => div()
                .text_size(px(14.0))
                .line_height(px(20.0))
                .text_color(rgba(if done { th.text_faint } else { th.text }))
                .when(done, |d| d.line_through())
                .child(task.title.clone())
                .into_any_element(),
        };
        let notes = (!task.notes.is_empty() && !done).then(|| {
            div()
                .text_size(px(12.0))
                .line_height(px(16.0))
                .text_color(rgba(th.text_dim))
                .truncate()
                .child(task.notes.lines().next().unwrap_or_default().to_owned())
        });
        let chips = {
            let mut chips = div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(6.0))
                .pt(px(4.0));
            let mut any = false;
            if let Some((label, past)) = due.filter(|_| !done) {
                any = true;
                let color = if past { th.error } else { th.text_dim };
                chips = chips.child(
                    div()
                        .id(("task-due", id as usize))
                        .h(px(24.0))
                        .px(px(8.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(4.0))
                        .rounded(px(8.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(th.hover)))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            cx.stop_propagation();
                            this.task_open_details(id, window, cx)
                        }))
                        .border_1()
                        .border_color(rgba(if past {
                            fade(th.error, 0.5)
                        } else {
                            th.divider
                        }))
                        .text_size(px(12.0))
                        .text_color(rgba(color))
                        .child(icon("calendar", color, 14.0))
                        .child(label)
                        .when(!task.repeat.is_empty(), |chip| {
                            chip.child(icon("refresh", color, 14.0))
                        }),
                );
            } else if !task.repeat.is_empty() && !done {
                any = true;
                chips = chips.child(icon("refresh", th.text_faint, 14.0));
            }
            if let Some(note) = super::notes::note_of_task(&task.mail) {
                any = true;
                chips = chips.child(
                    div()
                        .id(("task-note", id as usize))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(th.hover)))
                        .tooltip(tip(tr!("tasks-open-note"), th))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            cx.stop_propagation();
                            this.open_task_note(note, window, cx)
                        }))
                        .h(px(24.0))
                        .px(px(8.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(4.0))
                        .rounded(px(8.0))
                        .bg(rgba(th.chip))
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim))
                        .child(icon("notes", th.text_dim, 14.0))
                        .child(tr!("tasks-from-note")),
                );
            } else if !task.mail.is_empty() {
                any = true;
                let header = task.mail.clone();
                chips = chips.child(
                    div()
                        .id(("task-mail", id as usize))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(th.hover)))
                        .tooltip(tip(tr!("tasks-open-mail"), th))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            cx.stop_propagation();
                            this.open_task_mail(&header, window, cx)
                        }))
                        .h(px(24.0))
                        .px(px(8.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(4.0))
                        .rounded(px(8.0))
                        .bg(rgba(th.chip))
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim))
                        .child(icon("mail", th.text_dim, 14.0))
                        .child(tr!("tasks-from-mail")),
                );
            }
            if let Some(list) = list {
                any = true;
                chips = chips.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_faint))
                        .child(list.to_owned()),
                );
            }
            any.then_some(chips)
        };
        let star = div()
            .id(("task-star", id as usize))
            .size(px(32.0))
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .tooltip(tip(
                if starred {
                    tr!("tasks-unstar")
                } else {
                    tr!("tasks-star")
                },
                th,
            ))
            .when(!starred, |d| {
                d.invisible().group_hover("task-row", |s| s.visible())
            })
            .child(icon(
                if starred { "star-filled" } else { "star" },
                if starred { th.star } else { th.text_dim },
                20.0,
            ))
            .on_click(cx.listener(move |this, _, _, cx| {
                cx.stop_propagation();
                this.task_toggle_star(id, cx)
            }));
        let tick = div()
            .id(("task-tick", id as usize))
            .flex_none()
            .size(px(24.0))
            .mt(px(-2.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_full()
            .cursor_pointer()
            .tooltip(tip(
                if done {
                    tr!("tasks-mark-open")
                } else {
                    tr!("tasks-mark-done")
                },
                th,
            ))
            .child(round_tick(done, true, th))
            .on_click(cx.listener(move |this, _, _, cx| {
                cx.stop_propagation();
                this.task_toggle_done(id, cx)
            }));
        div()
            .id(("task-row", id as usize))
            .group("task-row")
            .min_h(px(44.0))
            .py(px(10.0))
            .pl(px(if step { 48.0 } else { 16.0 }))
            .pr(px(8.0))
            .flex()
            .flex_row()
            .items_start()
            .gap(px(16.0))
            .cursor_pointer()
            .when(picked, |d| d.bg(rgba(fade(th.accent, 0.12))))
            .when(!picked, |d| d.hover(|s| s.bg(rgba(th.hover))))
            .child(tick)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(title)
                    .children(notes)
                    .children(chips),
            )
            .when(!done, |d| d.child(star))
            // An open task, not a step, drags to another place in its list
            // or in another, on the lists' cards.
            .when(
                !done && task.parent.is_none() && editing.is_none() && list.is_none(),
                |d| {
                    d.on_drag(
                        TaskDragged {
                            id,
                            list: task.list,
                            title: task.title.clone(),
                            th: *th,
                        },
                        |drag, _, _, cx| cx.new(|_| drag.clone()),
                    )
                    // A drag let go over no list is forgotten by the next.
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, _| this.tasks.drag = None),
                    )
                },
            )
            .on_click(
                cx.listener(move |this, event: &gpui::ClickEvent, window, cx| {
                    if event.click_count() >= 2 {
                        this.task_open_details(id, window, cx);
                    } else if this.tasks.picked == Some(id) {
                        this.task_start_editing(id, window, cx);
                    } else {
                        this.tasks.picked = Some(id);
                        if let Some(focus) = &this.tasks.focus {
                            window.focus(focus, cx);
                        }
                        cx.notify();
                    }
                }),
            )
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(move |this, event: &gpui::MouseDownEvent, _, cx| {
                    this.tasks.picked = Some(id);
                    this.tasks.menu = Some(Menu::Task {
                        id,
                        at: event.position,
                    });
                    cx.stop_propagation();
                    cx.notify();
                }),
            )
            .into_any_element()
    }

    fn render_tasks_menu(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let menu = self.tasks.menu.as_ref()?;
        let item = |id: SharedString, name: &'static str, label: String| {
            div()
                .id(id)
                .h(px(36.0))
                .pl(px(16.0))
                .pr(px(24.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .child(icon(name, th.text_dim, 20.0))
                .child(div().flex_1().min_w_0().truncate().child(label))
        };
        let separator = || div().my(px(6.0)).h(px(1.0)).bg(rgba(th.divider));
        let (at, items): (Point<Pixels>, Vec<AnyElement>) = match *menu {
            Menu::List { list, at } => {
                let account = self
                    .tasks
                    .columns()
                    .iter()
                    .find(|c| c.list.id == list)
                    .and_then(|c| c.list.account);
                let is_default = self
                    .tasks
                    .columns()
                    .iter()
                    .find(|c| c.list.id == list)
                    .is_some_and(|c| c.list.is_default && c.list.account.is_some());
                let mut items = vec![
                    item(
                        "tasks-menu-rename".into(),
                        "compose",
                        tr!("tasks-rename-list"),
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.task_start_naming(Some(list), account, window, cx)
                    }))
                    .into_any_element(),
                ];
                // The service's default list can't be deleted.
                if !is_default {
                    items.push(
                        item(
                            "tasks-menu-delete-list".into(),
                            "trash",
                            tr!("tasks-delete-list"),
                        )
                        .on_click(
                            cx.listener(move |this, _, _, cx| this.task_delete_list(list, cx)),
                        )
                        .into_any_element(),
                    );
                }
                (at, items)
            }
            Menu::Task { id, at } => {
                let task = self.tasks.task(id)?;
                let done = self.tasks.done(task);
                let starred = self.tasks.starred(task);
                let from = task.list;
                let top = task.parent.is_none();
                let mut items = vec![
                    item(
                        "tasks-menu-done".into(),
                        "check",
                        if done {
                            tr!("tasks-mark-open")
                        } else {
                            tr!("tasks-mark-done")
                        },
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.tasks.menu = None;
                        this.task_toggle_done(id, cx)
                    }))
                    .into_any_element(),
                    item(
                        "tasks-menu-star".into(),
                        if starred { "star-filled" } else { "star" },
                        if starred {
                            tr!("tasks-unstar")
                        } else {
                            tr!("tasks-star")
                        },
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.tasks.menu = None;
                        this.task_toggle_star(id, cx)
                    }))
                    .into_any_element(),
                    item(
                        "tasks-menu-rename-task".into(),
                        "compose",
                        tr!("tasks-edit-title"),
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.tasks.menu = None;
                        this.task_start_editing(id, window, cx)
                    }))
                    .into_any_element(),
                    item("tasks-menu-details".into(), "notes", tr!("tasks-details"))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.task_open_details(id, window, cx)
                        }))
                        .into_any_element(),
                ];
                if top {
                    items.push(
                        item(
                            "tasks-menu-step".into(),
                            "indent-more",
                            tr!("tasks-add-step"),
                        )
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.tasks.menu = None;
                            this.task_start_adding(from, Some(id), String::new(), window, cx)
                        }))
                        .into_any_element(),
                    );
                    let others: Vec<(i64, String)> = self
                        .tasks
                        .columns()
                        .iter()
                        .filter(|c| c.list.id != from)
                        .map(|c| (c.list.id, list_title(c)))
                        .collect();
                    if !others.is_empty() {
                        items.push(separator().into_any_element());
                        for (list, name) in others {
                            items.push(
                                item(
                                    format!("tasks-menu-move-{list}").into(),
                                    "move-to",
                                    tr!("tasks-move-to", list = name),
                                )
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.move_task_to(id, list, cx)
                                }))
                                .into_any_element(),
                            );
                        }
                    }
                }
                items.push(separator().into_any_element());
                items.push(
                    item("tasks-menu-delete".into(), "trash", tr!("tasks-delete"))
                        .on_click(cx.listener(move |this, _, _, cx| this.task_delete(id, cx)))
                        .into_any_element(),
                );
                (at, items)
            }
        };
        let list = div()
            .w(px(MENU_WIDTH))
            .py(px(8.0))
            .flex()
            .flex_col()
            .map(|d| raised(d, th, 8.0, 3.0))
            .text_size(px(14.0))
            .text_color(rgba(th.text))
            .children(items)
            .with_animation(
                "tasks-menu",
                Animation::new(katna_ui::motion::time(std::time::Duration::from_millis(
                    140,
                )))
                .with_easing(ease_out_quint()),
                |el, t| el.opacity(t).mt(px(-4.0 * (1.0 - t))),
            );
        let close = || {
            cx.listener(|this: &mut Self, _: &gpui::MouseDownEvent, _, cx| {
                this.tasks.menu = None;
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
                            .id("tasks-menu-scrim")
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
                            .snap_to_window_with_margin(px(8.0))
                            .child(div().occlude().child(list)),
                    )
                    .with_priority(4),
                )
                .into_any_element(),
        )
    }
}

/// What the line under an account in the side list says on this page.
pub(super) fn say(say: Say<'_>) -> String {
    match say {
        Say::SignIn => tr!("tasks-account-sign-in"),
        Say::SignInRefused { provider } => {
            tr!("tasks-account-sign-in-refused", provider = provider)
        }
        Say::SignedIn { address } => tr!("tasks-account-signed-in", address = address),
        Say::Refused => tr!("tasks-account-refused"),
        Say::ChangePassword => tr!("tasks-account-change-password"),
        Say::ChangePasswordTooltip => tr!("tasks-account-change-password-tooltip"),
        Say::NotEnabled => tr!("tasks-account-not-enabled"),
        Say::Error { reason } => tr!("tasks-account-error", reason = reason),
        Say::Failed => tr!("tasks-account-failed"),
        Say::None => tr!("tasks-account-none"),
        Say::NoneWhy { reason } => tr!("tasks-account-none-why", reason = reason),
        Say::UseSignIn { provider } => tr!("tasks-account-use-sign-in", provider = provider),
        Say::SignInWith { provider } => tr!("tasks-account-sign-in-with", provider = provider),
        Say::Looking => tr!("tasks-account-looking"),
        Say::TryAgain => tr!("tasks-account-try-again"),
        Say::TryAgainTooltip => tr!("tasks-account-try-again-tooltip"),
        Say::Fixing => tr!("tasks-account-fixing"),
    }
}

/// A list's name; the one on this computer is named in the app's language.
pub(super) fn list_title(column: &Column) -> String {
    if column.list.account.is_none() && column.list.is_default && column.list.title == "My Tasks" {
        tr!("tasks-my-tasks")
    } else {
        column.list.title.clone()
    }
}

/// What list `list` shows before its slot `slot` while `drag` is under way:
/// the gap opening where the task would land, and gaps closing where it no
/// longer would. The tasks below slide as they open and close.
fn drag_gaps(drag: &Drag, list: i64, slot: usize, th: &Theme) -> Vec<AnyElement> {
    let height = drag.height;
    let timing = || {
        Animation::new(katna_ui::motion::time(std::time::Duration::from_millis(
            GAP_MS,
        )))
        .with_easing(gpui::ease_in_out)
    };
    // A faint place for the task, as tall as the gap is.
    let gap = |open: f32| {
        div()
            .flex_none()
            .overflow_hidden()
            .h(px(height * open))
            .child(
                div().h(px(height)).py(px(2.0)).px(px(8.0)).child(
                    div()
                        .size_full()
                        .rounded(px(8.0))
                        .bg(rgba(fade(th.accent, 0.08))),
                ),
            )
    };
    let mut gaps = Vec::new();
    for &(l, s, serial) in &drag.closing {
        if (l, s) == (list, slot) {
            gaps.push(
                gap(1.0)
                    .with_animation(("task-gap-close", serial), timing(), move |el, t| {
                        el.h(px(height * (1.0 - t)))
                    })
                    .into_any_element(),
            );
        }
    }
    if drag.to == Some((list, slot)) {
        gaps.push(if drag.at_once {
            gap(1.0).into_any_element()
        } else {
            gap(0.0)
                .with_animation(("task-gap-open", drag.serial), timing(), move |el, t| {
                    el.h(px(height * t))
                })
                .into_any_element()
        });
    }
    gaps
}

fn card_heading(title: String, th: &Theme) -> gpui::Div {
    div()
        .h(px(56.0))
        .pl(px(20.0))
        .pr(px(8.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.0))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .truncate()
                .text_size(px(18.0))
                .text_color(rgba(th.text))
                .child(title),
        )
}

/// The round tick of a task: an empty ring, a check on hover, and a
/// filled disc with a check once done.
pub(super) fn round_tick(done: bool, hover: bool, th: &Theme) -> AnyElement {
    let ring = div()
        .size(px(20.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded_full();
    if done {
        return ring
            .bg(rgba(th.accent))
            .child(icon("check", th.on_accent, 16.0))
            .with_animation(
                "tick-done",
                Animation::new(katna_ui::motion::time(std::time::Duration::from_millis(
                    180,
                )))
                .with_easing(ease_out_quint()),
                |el, t| el.opacity(0.4 + 0.6 * t),
            )
            .into_any_element();
    }
    ring.border_px(2.0)
        .border_color(rgba(th.text_dim))
        .when(hover, |d| {
            d.child(
                div()
                    .invisible()
                    .group_hover("task-row", |s| s.visible())
                    .child(icon("check", th.text_dim, 14.0)),
            )
        })
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_tasks_get_a_day_a_time_and_a_repeat() {
        let today: jiff::civil::Date = "2026-09-29".parse().unwrap();
        let typed = |text: &str| typed_task(text, today);
        assert_eq!(
            typed("Call the bank tomorrow 3pm"),
            Some(TypedTask {
                title: "Call the bank".into(),
                due: Some("2026-09-30".into()),
                due_time: Some(15 * 60),
                repeat: None,
            })
        );
        // A repeat alone starts on its first day from today (a Tuesday).
        let plants = typed("Water the plants every Monday").unwrap();
        assert_eq!(plants.title, "Water the plants");
        assert!(plants.repeat.is_some_and(|r| r.contains("FREQ=WEEKLY")));
        assert_eq!(plants.due.as_deref(), Some("2026-10-05"));
        assert_eq!(
            typed("Stretch every day").and_then(|t| t.due).as_deref(),
            Some("2026-09-29")
        );
        // Tasks have no place: "at" stays in the title.
        assert_eq!(typed("Pick up the parcel at the post office"), None);
        assert_eq!(typed("Buy milk"), None);
        // Nothing would be left of the title.
        assert_eq!(typed("tomorrow"), None);
    }
}
