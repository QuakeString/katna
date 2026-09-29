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
    Animation, AnimationExt, AnyElement, Context, Entity, FocusHandle, Focusable, FontWeight,
    KeyDownEvent, MouseButton, Pixels, Point, SharedString, Subscription, Task, Window, anchored,
    deferred, div, ease_out_quint, prelude::*, rgba,
};
use katna_core::AccountId;
use katna_i18n::tr;
use katna_store::tasks::Task as TaskItem;
use katna_ui::px;
use katna_ui::text_input::{InputEvent, TextInput};

mod details;

use super::MailWindow;
use crate::daemon::{self, Command};
use crate::data::EntryKey;
use crate::tasks::{Board, Column, TaskCommand, TaskEdit};
use crate::theme::{Theme, fade};
use crate::widgets::{icon, placeholder, raised, tip};

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
    Starred,
    List(i64),
}

/// A task typed into a list's "Add a task" row.
struct Adding {
    list: i64,
    parent: Option<i64>,
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
    focus: Option<FocusHandle>,
    loading: Option<Task<()>>,
    /// Reads again whenever the daemon says tasks changed.
    watching: Option<Task<()>>,
}

impl TasksPage {
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

    fn columns(&self) -> &[Column] {
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

    /// The open tasks shown, in order, for keys to move through.
    fn shown(&self) -> Vec<i64> {
        let mut shown = Vec::new();
        for column in self.shown_columns() {
            for task in &column.tasks {
                if !self.done(task) && self.parent_open(task) {
                    shown.push(task.id);
                }
            }
        }
        if self.view == View::Starred {
            shown.retain(|id| self.task(*id).is_some_and(|t| self.starred(t)));
        }
        shown
    }

    /// Whether a step's task is still open (a done task hides its steps
    /// with it).
    fn parent_open(&self, task: &TaskItem) -> bool {
        task.parent
            .and_then(|p| self.task(p))
            .is_none_or(|p| !self.done(p))
    }

    fn shown_columns(&self) -> Vec<&Column> {
        match self.view {
            View::All | View::Starred => self.columns().iter().collect(),
            View::List(id) => self.columns().iter().filter(|c| c.list.id == id).collect(),
        }
    }
}

/// A due day as the page shows it: "Today", "Tomorrow", a weekday this
/// week, else the day and month; with the time if it has one. The flag
/// says it is past.
fn due_label(task: &TaskItem, today: jiff::civil::Date) -> Option<(String, bool)> {
    let day: jiff::civil::Date = task.due.parse().ok()?;
    let days = (day - today).get_days();
    let at = day.to_datetime(jiff::civil::Time::midnight());
    let mut label = match days {
        0 => tr!("tasks-due-today"),
        1 => tr!("tasks-due-tomorrow"),
        -1 => tr!("tasks-due-yesterday"),
        2..=6 => katna_i18n::format::weekday(at),
        _ => katna_i18n::format::day_month(at),
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

fn today() -> jiff::civil::Date {
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

    fn load_tasks(&mut self, cx: &mut Context<Self>) {
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
        cx.notify();
    }

    // --- Changes -----------------------------------------------------------

    fn task_toggle_done(&mut self, id: i64, cx: &mut Context<Self>) {
        let Some(task) = self.tasks.task(id) else {
            return;
        };
        let done = !self.tasks.done(task);
        self.tasks.pending.entry(id).or_default().done = Some(done);
        if self.tasks.picked == Some(id) && done {
            self.tasks.picked = None;
        }
        let text = done.then(|| tr!("tasks-toast-done"));
        let undo = done.then_some(TaskCommand::SetDone(id, false));
        self.send_task(TaskCommand::SetDone(id, done), text, undo, cx);
        cx.notify();
    }

    fn task_toggle_star(&mut self, id: i64, cx: &mut Context<Self>) {
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

    fn task_delete(&mut self, id: i64, cx: &mut Context<Self>) {
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

    fn task_start_adding(
        &mut self,
        list: i64,
        parent: Option<i64>,
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
                InputEvent::Changed => {}
            },
        );
        window.focus(&input.focus_handle(cx), cx);
        self.tasks.editing = None;
        self.tasks.adding = Some(Adding {
            list,
            parent,
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
        let command = TaskCommand::Add {
            list: adding.list,
            parent: adding.parent,
            title,
            mail: String::new(),
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
            None => self.send_task(TaskCommand::AddList(naming.account, title), None, None, cx),
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
            None => placeholder(&tr!("tasks-loading"), th),
            Some(Err(err)) => placeholder(err, th),
            Some(Ok(_)) => self.render_task_board(th, cx),
        };
        let menu = self.render_tasks_menu(th, cx);
        let details = self.render_task_details(th, cx);
        div()
            .id("tasks-page")
            .size_full()
            .flex()
            .flex_row()
            .when_some(page.focus.as_ref(), |d, focus| d.track_focus(focus))
            .on_key_down(cx.listener(Self::tasks_key))
            .child(self.render_tasks_nav(th, cx))
            .child(div().flex_1().min_w_0().h_full().child(body))
            .children(menu)
            .children(details)
            .with_animation(
                "tasks-page-in",
                Animation::new(std::time::Duration::from_millis(220)).with_easing(ease_out_quint()),
                |el, t| el.opacity(t),
            )
            .into_any_element()
    }

    fn render_tasks_nav(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let page = &self.tasks;
        let row = |id: SharedString, icon_name: &'static str, label: String, on: bool| {
            div()
                .id(id)
                .h(px(40.0))
                .mx(px(8.0))
                .pl(px(16.0))
                .pr(px(12.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .rounded_full()
                .cursor_pointer()
                .text_size(px(14.0))
                .when(on, |d| {
                    d.bg(rgba(th.nav_selected))
                        .text_color(rgba(th.nav_selected_text))
                        .font_weight(FontWeight::BOLD)
                })
                .when(!on, |d| {
                    d.text_color(rgba(th.text)).hover(|s| s.bg(rgba(th.hover)))
                })
                .child(icon(
                    icon_name,
                    if on {
                        th.nav_selected_text
                    } else {
                        th.text_dim
                    },
                    20.0,
                ))
                .child(div().flex_1().min_w_0().truncate().child(label))
        };
        let create = div()
            .id("tasks-create")
            .ml(px(8.0))
            .mt(px(8.0))
            .mb(px(12.0))
            .h(px(56.0))
            .pl(px(16.0))
            .pr(px(20.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .rounded(px(16.0))
            .bg(rgba(th.compose))
            .text_color(rgba(th.compose_text))
            .text_size(px(14.0))
            .font_weight(FontWeight::MEDIUM)
            .cursor_pointer()
            .shadow(crate::widgets::elevation(th, 1.0))
            .hover(|s| s.shadow(crate::widgets::elevation(th, 2.0)))
            .child(icon("add", th.compose_text, 24.0))
            .child(tr!("tasks-create"))
            .on_click(cx.listener(|this, _, window, cx| {
                let list = match this.tasks.view {
                    View::List(id) => Some(id),
                    _ => this.tasks.columns().first().map(|c| c.list.id),
                };
                if let Some(list) = list {
                    this.task_start_adding(list, None, window, cx);
                }
            }));
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
        let mut nav = div()
            .id("tasks-nav")
            .flex_none()
            .w(px(NAV_WIDTH))
            .h_full()
            .pb(px(16.0))
            .overflow_y_scroll()
            .flex()
            .flex_col()
            .child(div().child(create))
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
        let mut last: Option<Option<AccountId>> = None;
        for column in page.columns() {
            if last != Some(column.list.account) {
                last = Some(column.list.account);
                let heading = if column.account.is_empty() {
                    tr!("tasks-on-this-computer")
                } else {
                    column.account.clone()
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
            }
            let id = column.list.id;
            let count = open_count(column);
            let naming_this = page.naming.as_ref().is_some_and(|n| n.list == Some(id));
            if naming_this {
                nav = nav.children(page.naming.as_ref().map(|n| self.naming_row(n, th)));
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
        let new_list = page.naming.as_ref().filter(|n| n.list.is_none());
        nav = nav.children(new_list.map(|n| self.naming_row(n, th)));
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
        nav.into_any_element()
    }

    fn naming_row(&self, naming: &Naming, th: &Theme) -> AnyElement {
        div()
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
        if columns.is_empty() {
            return placeholder(&tr!("tasks-no-lists"), th);
        }
        match page.view {
            View::All => div()
                .id("tasks-board")
                .size_full()
                .overflow_x_scroll()
                .p(px(16.0))
                .flex()
                .flex_row()
                .items_start()
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
                .justify_center()
                .children(
                    columns
                        .into_iter()
                        .map(|c| self.render_task_card(c, SINGLE_WIDTH, th, cx)),
                )
                .into_any_element(),
            View::Starred => self.render_starred(columns, th, cx),
        }
    }

    fn card_frame(&self, id: SharedString, width: f32, th: &Theme) -> gpui::Stateful<gpui::Div> {
        div()
            .id(id)
            .flex_none()
            .w(px(width))
            .max_h_full()
            .overflow_y_scroll()
            .pb(px(8.0))
            .flex()
            .flex_col()
            .rounded(px(16.0))
            .bg(rgba(if th.dark { th.read_row } else { th.surface }))
            .border_1()
            .border_color(rgba(th.divider))
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
            .filter(|(_, t)| page.starred(t) && !page.done(t))
            .map(|(c, t)| self.render_task_row(t, Some(&list_title(c)), today, th, cx))
            .collect();
        let empty = rows.is_empty();
        div()
            .size_full()
            .p(px(16.0))
            .flex()
            .justify_center()
            .child(
                self.card_frame("tasks-starred-card".into(), SINGLE_WIDTH, th)
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
            )
            .into_any_element()
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
            .partition(|t| page.done(t));
        // Open tasks; a step whose task is done shows among the done.
        let open_rows: Vec<AnyElement> = open
            .iter()
            .flat_map(|t| {
                let mut rows = vec![self.render_task_row(t, None, today, th, cx)];
                if let Some(adding) = page.adding.as_ref().filter(|a| a.parent == Some(t.id)) {
                    rows.push(self.adding_row(adding, true, th, cx));
                }
                rows
            })
            .collect();
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
            None => div()
                .id(("tasks-add", id as usize))
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
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.task_start_adding(id, None, window, cx)
                }))
                .into_any_element(),
        };
        self.card_frame(format!("tasks-card-{id}").into(), width, th)
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
            .children(done_rows)
            .into_any_element()
    }

    fn adding_row(
        &self,
        adding: &Adding,
        step: bool,
        th: &Theme,
        _cx: &mut Context<Self>,
    ) -> AnyElement {
        div()
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
                    .text_size(px(14.0))
                    .child(adding.input.clone()),
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
            if !task.mail.is_empty() {
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
                            this.task_start_adding(from, Some(id), window, cx)
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
                Animation::new(std::time::Duration::from_millis(140)).with_easing(ease_out_quint()),
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

/// A list's name; the one on this computer is named in the app's language.
fn list_title(column: &Column) -> String {
    if column.list.account.is_none() && column.list.is_default && column.list.title == "My Tasks" {
        tr!("tasks-my-tasks")
    } else {
        column.list.title.clone()
    }
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
fn round_tick(done: bool, hover: bool, th: &Theme) -> AnyElement {
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
                Animation::new(std::time::Duration::from_millis(180)).with_easing(ease_out_quint()),
                |el, t| el.opacity(0.4 + 0.6 * t),
            )
            .into_any_element();
    }
    ring.border_2()
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
