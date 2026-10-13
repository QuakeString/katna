// SPDX-License-Identifier: GPL-3.0-or-later

//! Quick capture, "Catch a thought from anywhere" (`docs/ARCHITECTURE.md`
//! §18.1): a small frosted card over whatever is on screen, in a window of
//! its own, with Task and Note tabs and one line to type in. The global
//! shortcuts (Meta+Alt+T, Meta+Alt+N), the tray's New task and New note
//! and KRunner's `task:` and `note:` open it (`app_action::CAPTURE`); when
//! Katna Mail is not running it starts with only the card.
//!
//! A task is read as the Tasks page's typed add reads it
//! ([`katna_dav::quick_task`]): what was understood shows as chips while
//! typing (the day and time, labels, the list and account, the reminder),
//! and a click on a chip changes it. Tab switches between Task and Note,
//! Enter saves and closes, Escape closes. A task goes to the default list,
//! or the one picked, through the daemon (`AddTaskTo`, then `EditTask`); a
//! note through `SaveNote`, kept in the first mail account's Notes folder
//! unless another place is picked.

use std::cell::Cell;
use std::rc::Rc;

use async_channel::Sender;

use gpui::{
    AnyElement, App, AppContext, Bounds, Context, Entity, FocusHandle, Focusable, FontWeight,
    Global, KeyBinding, Pixels, SharedString, Subscription, Task, WeakEntity, Window,
    WindowBackgroundAppearance, WindowBounds, WindowDecorations, WindowHandle, WindowKind,
    WindowOptions, actions, canvas, div, point, prelude::*, rgba, size,
};
use jiff::civil::Date;
use katna_chrome::{Environment, Look, Session, WindowChrome};
use katna_core::config::{Config, FROST_OPACITY, Theme as ThemeChoice};
use katna_core::ids::MAIL_APP_ID;
use katna_core::{AccountKind, Paths};
use katna_dav::quick_task;
use katna_dbus::app_action;
use katna_dbus::zbus::Connection;
use katna_i18n::{format, tr};
use katna_store::{Mode, Store};
use katna_ui::tokens::{elevation as level, radius, space, text};
use katna_ui::{InputEvent, TextInput, WindowDrag, px, unpx};

use super::MailWindow;
use super::colors::DesktopColors;
use crate::instance::Request;
use crate::tasks::{TaskCommand, TaskEdit};
use crate::theme::{Accent, Theme, fade};
use crate::widgets::{dialog_tint, elevation, icon};

actions!(katna_mail, [CaptureSwitchKind]);

/// The card's key context: Tab switches between Task and Note.
const CONTEXT: &str = "QuickCapture";

/// The card's width, as in the study's mockup.
const WIDTH: f32 = 430.0;
/// Room around the card for its shadow; outside the window's frame, so
/// the compositor neither blurs it nor takes clicks there.
const MARGIN: f32 = 24.0;
/// How many times the lists are read, half a second apart, before the
/// card goes on without them.
const PLACES_TRIES: u32 = 10;
/// The card's height before its first frame says otherwise.
const FIRST_HEIGHT: f32 = 168.0;
/// Where the card opens: this far down the screen.
const FROM_TOP: f32 = 0.22;
/// How tall a chip is.
const CHIP_HEIGHT: f32 = 24.0;
/// How tall the line typed in is.
const FIELD_HEIGHT: f32 = 40.0;

/// Binds the card's keys; part of the keymap, so call it wherever the
/// keymap is bound again.
pub(super) fn bind_keys(bindings: &mut Vec<KeyBinding>) {
    bindings.push(KeyBinding::new("tab", CaptureSwitchKind, Some(CONTEXT)));
    bindings.push(KeyBinding::new(
        "shift-tab",
        CaptureSwitchKind,
        Some(CONTEXT),
    ));
}

/// What opening the card needs, kept from the app's start.
pub struct CaptureHost {
    env: Environment,
    paths: Paths,
    font: Option<SharedString>,
    connection: Option<Connection>,
    /// The card, while it is open.
    open: Option<WindowHandle<CaptureCard>>,
    /// Where the card was last, moved or not, so it opens there again
    /// while Katna runs. Wayland keeps the place to the compositor.
    last: Option<Bounds<Pixels>>,
    /// The mail window, once it is open: it reads its notes again after a
    /// note was saved here.
    main: Option<WeakEntity<MailWindow>>,
    /// Asks the app for what the card can't do itself: the mail window
    /// (opened if need be) for a new event's More options.
    requests: Option<Sender<Request>>,
    /// The New event window, while it is open.
    event: Option<WindowHandle<MailWindow>>,
}

impl Global for CaptureHost {}

/// What the New event window needs from the card's host: the app's look,
/// its paths and font, and where to send More options.
pub(super) fn host(cx: &App) -> Option<(Environment, Paths, Option<SharedString>)> {
    let host = cx.try_global::<CaptureHost>()?;
    Some((host.env.clone(), host.paths.clone(), host.font.clone()))
}

/// The open New event window, if any.
pub(super) fn event_window(cx: &App) -> Option<WindowHandle<MailWindow>> {
    cx.try_global::<CaptureHost>()?.event
}

pub(super) fn set_event_window(handle: Option<WindowHandle<MailWindow>>, cx: &mut App) {
    if cx.has_global::<CaptureHost>() {
        cx.global_mut::<CaptureHost>().event = handle;
    }
}

/// Sends `request` to the app, as another launch would.
pub(super) fn ask_app(request: Request, cx: &App) {
    if let Some(sender) = cx
        .try_global::<CaptureHost>()
        .and_then(|h| h.requests.as_ref())
    {
        let _ = sender.try_send(request);
    }
}

/// Keeps what the card needs. Call once, before any request is handled.
pub fn install(
    env: Environment,
    paths: Paths,
    font: Option<SharedString>,
    connection: Option<Connection>,
    requests: Option<Sender<Request>>,
    cx: &mut App,
) {
    // The card's keys (and the fields'), also when it opens before the
    // mail window binds the rest.
    let config = Config::load(&paths.config_file()).unwrap_or_default();
    super::keymap::bind(&config.shortcuts, cx);
    cx.set_global(CaptureHost {
        env,
        paths,
        font,
        connection,
        open: None,
        last: None,
        main: None,
        requests,
        event: None,
    });
}

/// Tells the card about the mail window.
pub fn set_main(main: WeakEntity<MailWindow>, cx: &mut App) {
    if cx.has_global::<CaptureHost>() {
        cx.global_mut::<CaptureHost>().main = Some(main);
    }
}

/// Opens the card as `param` asks (`app_action::capture`): on Task or on
/// Note, with the text typed if any; an open card comes forward and
/// switches.
pub fn open(param: &str, cx: &mut App) {
    // The desktop clock's Add…: a new event, in a window of its own.
    if let Some(day) = app_action::capture_event_day(param) {
        super::event_window::open(day, cx);
        return;
    }
    let (note, text) = app_action::capture_parts(param);
    let text = text.to_owned();
    let Some(open) = cx.try_global::<CaptureHost>().map(|host| host.open) else {
        return;
    };
    if let Some(handle) = open {
        let shown = handle.update(cx, |card, window, cx| {
            card.set_kind(note, cx);
            if !text.is_empty() {
                card.input
                    .update(cx, |input, cx| input.set_text(text.clone(), cx));
            }
            window.activate_window();
            window.focus(&card.input.focus_handle(cx), cx);
        });
        if shown.is_ok() {
            return;
        }
    }
    let host = cx.global::<CaptureHost>();
    let env = host.env.clone();
    let paths = host.paths.clone();
    let font = host.font.clone();
    let options = window_options(cx);
    let opened = cx.open_window(options, |window, cx| {
        cx.new(|cx| CaptureCard::new(env, paths, font, note, text, window, cx))
    });
    match opened {
        Ok(handle) => {
            cx.global_mut::<CaptureHost>().open = Some(handle);
            let _ = handle.update(cx, |_, window, _| window.activate_window());
        }
        Err(err) => tracing::warn!("cannot open quick capture: {err}"),
    }
}

/// A small window without the desktop's frame, over the middle of the
/// screen's top part, blurred behind where the compositor can. On a
/// screen narrower than the card (a phone), it is as wide as the screen.
fn window_options(cx: &App) -> WindowOptions {
    let mut surface = size(px(WIDTH + 2.0 * MARGIN), px(FIRST_HEIGHT + 2.0 * MARGIN));
    let host = cx.global::<CaptureHost>();
    let last = host
        .last
        .filter(|_| host.env.session != Session::Wayland)
        .map(|last| last.origin);
    let bounds = match (last, cx.primary_display()) {
        (Some(origin), _) => Bounds::new(origin, surface),
        (None, Some(display)) => {
            let screen = display.bounds();
            surface.width = surface.width.min(screen.size.width);
            Bounds::new(
                point(
                    screen.origin.x + (screen.size.width - surface.width) / 2.0,
                    screen.origin.y + screen.size.height * FROM_TOP,
                ),
                surface,
            )
        }
        (None, None) => Bounds::centered(None, surface, cx),
    };
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: None,
        focus: true,
        show: true,
        kind: WindowKind::Normal,
        is_movable: true,
        is_resizable: false,
        is_minimizable: false,
        app_id: Some(MAIL_APP_ID.to_owned()),
        window_background: if Look::blur_available() {
            WindowBackgroundAppearance::Blurred
        } else {
            WindowBackgroundAppearance::Transparent
        },
        window_decorations: Some(WindowDecorations::Client),
        window_min_size: Some(size(px(2.0 * MARGIN), px(CHIP_HEIGHT))),
        ..Default::default()
    }
}

/// When the reminder of a task goes off, counted from its due day at its
/// time, or 9 AM without one (as in the task's details).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Remind {
    Off,
    OnTime,
    HourBefore,
    DayBefore,
}

impl Remind {
    const ALL: [Self; 4] = [Self::OnTime, Self::HourBefore, Self::DayBefore, Self::Off];

    fn label(self, timed: bool) -> String {
        match self {
            Self::Off => tr!("tasks-remind-off"),
            Self::OnTime if timed => tr!("tasks-remind-on-time"),
            Self::OnTime => tr!(
                "tasks-remind-morning",
                time = clock(katna_dav::todo::DAY_START)
            ),
            Self::HourBefore => tr!("tasks-remind-hour-before"),
            Self::DayBefore => tr!("tasks-remind-day-before"),
        }
    }

    /// The reminder's time for a task due on `day` at `time`.
    fn at(self, day: &str, time: Option<u32>) -> Option<i64> {
        let offset = match self {
            Self::Off => return None,
            Self::OnTime => 0,
            Self::HourBefore => -3600,
            Self::DayBefore => -86_400,
        };
        let zone = jiff::tz::TimeZone::system();
        katna_dav::todo::due_at(day, time, &zone).map(|due| due + offset)
    }
}

/// A day picked on the date chip, over the one typed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DueChoice {
    Day(Date),
    None,
}

/// The chip whose choices show in place of the chips.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Chip {
    Due,
    Labels,
    List,
    Remind,
    Place,
}

impl Chip {
    fn id(self) -> &'static str {
        match self {
            Chip::Due => "capture-due",
            Chip::Labels => "capture-label",
            Chip::List => "capture-list",
            Chip::Remind => "capture-remind",
            Chip::Place => "capture-place",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Chip::Due => "calendar",
            Chip::Labels => "label",
            Chip::List => "list-bulleted",
            Chip::Remind => "bell",
            Chip::Place => "notes",
        }
    }
}

/// A task list, as the list chip names it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ListChoice {
    id: i64,
    title: String,
    /// The account's name; empty for a list on this computer.
    account: String,
}

/// Where tasks and notes can go, read from the store.
#[derive(Debug, Clone, Default)]
struct Places {
    lists: Vec<ListChoice>,
    default_list: Option<i64>,
    /// The mail accounts whose server keeps a Notes folder.
    note_accounts: Vec<(i64, String)>,
    /// The labels on notes and tasks: one set.
    labels: Vec<String>,
}

impl Places {
    fn read(paths: &Paths) -> katna_store::Result<Self> {
        let store = Store::open(paths, Mode::ReadOnly)?;
        let accounts = store.accounts()?;
        let name = |id: Option<katna_core::AccountId>| {
            id.and_then(|id| accounts.iter().find(|a| a.id == id))
                .map(|a| a.display_name.clone())
                .unwrap_or_default()
        };
        let task_lists = store.task_lists()?;
        let default_list = quick_task::default_list(&task_lists).map(|l| l.id);
        let lists = task_lists
            .iter()
            .map(|l| ListChoice {
                id: l.id,
                title: l.title.clone(),
                account: name(l.account),
            })
            .collect();
        let note_accounts = accounts
            .iter()
            .filter(|a| a.kind == AccountKind::Imap)
            .map(|a| (a.id.0, a.display_name.clone()))
            .collect();
        let labels = store.labels_in_use()?;
        Ok(Self {
            lists,
            default_list,
            note_accounts,
            labels,
        })
    }
}

/// The card's root view.
pub struct CaptureCard {
    note: bool,
    input: Entity<TextInput>,
    config: Config,
    desktop_colors: DesktopColors,
    font: Option<SharedString>,
    places: Places,
    due: Option<DueChoice>,
    labels: Option<Vec<String>>,
    list: Option<i64>,
    remind: Option<Remind>,
    /// `Some(None)`: this computer.
    place: Option<Option<i64>>,
    choosing: Option<Chip>,
    saving: bool,
    error: Option<String>,
    /// The card's height as last drawn.
    height: Rc<Cell<f32>>,
    focus: FocusHandle,
    _subscriptions: Vec<Subscription>,
    _task: Option<Task<()>>,
}

impl CaptureCard {
    fn new(
        env: Environment,
        paths: Paths,
        font: Option<SharedString>,
        note: bool,
        text: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let config = Config::load(&paths.config_file()).unwrap_or_default();
        let desktop_colors = DesktopColors::new(&env.desktop, paths.config_dir());
        let input = cx.new(|cx| {
            let mut input = TextInput::new(placeholder(note), cx);
            if !text.is_empty() {
                input.set_text(text, cx);
            }
            input
        });
        let typed = cx.subscribe_in(
            &input,
            window,
            |this: &mut Self, _, event: &InputEvent, window, cx| match event {
                InputEvent::Submit => this.save(window, cx),
                InputEvent::Cancel => {
                    if this.choosing.take().is_none() {
                        window.remove_window();
                    }
                    cx.notify();
                }
                InputEvent::Changed => {
                    this.error = None;
                    cx.notify();
                }
            },
        );
        let appearance = cx.observe_window_appearance(window, |_, _, cx| cx.notify());
        let moved = cx.observe_window_bounds(window, |_, window, cx| {
            cx.global_mut::<CaptureHost>().last = Some(window.bounds());
        });
        window.focus(&input.focus_handle(cx), cx);
        // Read in the background; the chips name the list once it is. A
        // daemon that has just started may still be updating the store,
        // so a failed read is tried again for a few seconds.
        let load_paths = paths.clone();
        let task = cx.spawn(async move |this, cx| {
            for attempt in 0..PLACES_TRIES {
                let paths = load_paths.clone();
                let places = cx
                    .background_executor()
                    .spawn(async move { Places::read(&paths) })
                    .await;
                match places {
                    Ok(places) => {
                        this.update(cx, |this, cx| {
                            this.places = places;
                            cx.notify();
                        })
                        .ok();
                        return;
                    }
                    Err(err) if attempt + 1 == PLACES_TRIES => {
                        tracing::warn!(%err, "quick capture: reading the lists");
                    }
                    Err(_) => {
                        cx.background_executor()
                            .timer(std::time::Duration::from_millis(500))
                            .await;
                    }
                }
            }
        });
        cx.on_release(|_, cx| {
            if cx.has_global::<CaptureHost>() {
                cx.global_mut::<CaptureHost>().open = None;
            }
        })
        .detach();
        Self {
            note,
            input,
            config,
            desktop_colors,
            font,
            places: Places::default(),
            due: None,
            labels: None,
            list: None,
            remind: None,
            place: None,
            choosing: None,
            saving: false,
            error: None,
            height: Rc::new(Cell::new(0.0)),
            focus: cx.focus_handle(),
            _subscriptions: vec![typed, appearance, moved],
            _task: Some(task),
        }
    }

    fn set_kind(&mut self, note: bool, cx: &mut Context<Self>) {
        if self.note == note {
            return;
        }
        self.note = note;
        self.choosing = None;
        self.error = None;
        self.input
            .update(cx, |input, _| input.set_placeholder(placeholder(note)));
        cx.notify();
    }

    fn switch_kind(&mut self, _: &CaptureSwitchKind, _: &mut Window, cx: &mut Context<Self>) {
        self.set_kind(!self.note, cx);
    }

    /// The colors, as the mail window picks them.
    fn theme(&self, window: &Window) -> Theme {
        let view = &self.config.mail;
        let choice = match view.theme {
            ThemeChoice::System => None,
            ThemeChoice::Light => Some(false),
            ThemeChoice::Dark => Some(true),
        };
        let system = &self.desktop_colors.colors;
        let choice = Theme::forced_dark(view.colors(), system).or(choice);
        let dark = choice.unwrap_or_else(|| WindowChrome::desktop_dark(window));
        Theme::pick(dark, view.colors(), Accent::parse(&view.accent), system)
    }

    /// The card's fill: solid, or the surface tinted over the blur behind
    /// the window as a frosted dialog is (Settings > Experimental > Blur).
    fn fill(&self, th: &Theme) -> u32 {
        if !Look::blur_available() {
            return th.surface;
        }
        let experimental = &self.config.experimental;
        let tint = if experimental.custom_frost {
            experimental.frost_opacity
        } else {
            FROST_OPACITY
        };
        fade(th.surface, dialog_tint(f32::from(tint) / 100.0))
    }

    fn typed(&self, cx: &App) -> quick_task::TypedTask {
        let language = katna_i18n::current().language.tag.clone();
        quick_task::parse(self.input.read(cx).text(), today(), &language)
    }

    /// The labels typed, each spelled as a label the notes have if one
    /// is, or those picked on the chip.
    fn labels(&self, typed: &[String]) -> Vec<String> {
        if let Some(picked) = &self.labels {
            return picked.clone();
        }
        typed
            .iter()
            .map(|label| {
                self.places
                    .labels
                    .iter()
                    .find(|k| k.to_lowercase() == label.to_lowercase())
                    .cloned()
                    .unwrap_or_else(|| label.clone())
            })
            .collect()
    }

    /// The due day and time: as typed, or the day picked.
    fn due(&self, typed: &quick_task::TypedTask) -> (Option<Date>, Option<u32>) {
        let typed_day = typed.due.as_deref().and_then(|d| d.parse().ok());
        match self.due {
            None => (typed_day, typed.due_time),
            Some(DueChoice::None) => (None, None),
            Some(DueChoice::Day(day)) => (Some(day), typed.due_time),
        }
    }

    fn remind(&self, time: Option<u32>) -> Remind {
        self.remind.unwrap_or(if time.is_some() {
            Remind::OnTime
        } else {
            Remind::Off
        })
    }

    fn list(&self) -> Option<&ListChoice> {
        let id = self.list.or(self.places.default_list)?;
        self.places.lists.iter().find(|l| l.id == id)
    }

    /// Where a note goes: the account picked, else the first mail account
    /// with a Notes folder; `None` for this computer.
    fn place(&self) -> Option<i64> {
        self.place
            .unwrap_or_else(|| self.places.note_accounts.first().map(|(id, _)| *id))
    }

    fn place_name(&self, place: Option<i64>) -> String {
        place
            .and_then(|id| self.places.note_accounts.iter().find(|(a, _)| *a == id))
            .map_or_else(|| tr!("notes-on-this-computer"), |(_, name)| name.clone())
    }

    fn choose(&mut self, chip: Chip, cx: &mut Context<Self>) {
        self.choosing = if self.choosing == Some(chip) {
            None
        } else {
            Some(chip)
        };
        cx.notify();
    }

    /// Saves the task or note, and closes the card once the daemon has it.
    fn save(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.saving {
            return;
        }
        let text = self.input.read(cx).text().trim().to_owned();
        if text.is_empty() {
            return;
        }
        let typed = self.typed(cx);
        let labels = self.labels(&typed.labels);
        enum Saving {
            Task(TaskCommand, Vec<String>),
            Note(katna_dbus::NoteItem),
        }
        let saving = if self.note {
            let (body, _) = quick_task::take_labels(&text);
            Saving::Note(katna_dbus::NoteItem {
                account: self.place().unwrap_or(0),
                body: if body.is_empty() { text } else { body },
                labels,
                ..Default::default()
            })
        } else {
            let (day, time) = self.due(&typed);
            let due = day.map(|d| d.to_string()).unwrap_or_default();
            let remind_at = day.and_then(|_| self.remind(time).at(&due, time));
            let add = TaskCommand::Add {
                list: self.list().map_or(0, |l| l.id),
                parent: None,
                title: typed.title.clone(),
                due: due.clone(),
                mail: String::new(),
            };
            let fields = TaskEdit {
                due_time: (!due.is_empty() && time.is_some()).then_some(time),
                remind_at: remind_at.map(Some),
                repeat: typed.repeat.clone().filter(|_| !due.is_empty()),
                ..TaskEdit::default()
            };
            Saving::Task(TaskCommand::AddThen(Box::new(add), fields), labels)
        };
        self.saving = true;
        self.choosing = None;
        cx.notify();
        let connection = cx
            .try_global::<CaptureHost>()
            .and_then(|h| h.connection.clone());
        let handle = window.window_handle();
        self._task = Some(cx.spawn(async move |this, cx| {
            let saved = async {
                let connection = match connection {
                    Some(connection) => connection,
                    None => crate::daemon::connect().await?,
                };
                match saving {
                    Saving::Task(command, labels) => {
                        let id = crate::tasks::send(&connection, &command).await?;
                        if let Some(id) = id
                            && !labels.is_empty()
                        {
                            set_task_labels(&connection, id, labels).await?;
                        }
                        Ok::<bool, String>(false)
                    }
                    Saving::Note(note) => {
                        crate::daemon::save_note(&connection, &note).await?;
                        Ok(true)
                    }
                }
            }
            .await;
            let _ = this.update(cx, |this, cx| {
                this.saving = false;
                match saved {
                    Ok(note) => {
                        if note {
                            notes_changed(cx);
                        }
                        cx.defer(move |cx| {
                            let _ = handle.update(cx, |_, window, _| window.remove_window());
                        });
                    }
                    Err(err) => {
                        tracing::warn!(%err, "quick capture: not saved");
                        this.error = Some(tr!("capture-not-saved"));
                    }
                }
                cx.notify();
            });
        }));
    }

    fn render_tabs(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let tab = |id: &'static str, label: String, on: bool, note: bool| {
            div()
                .id(id)
                .h(px(CHIP_HEIGHT))
                .px(px(space::S4))
                .flex()
                .items_center()
                .rounded_full()
                .cursor_pointer()
                .text_size(px(text::CAPTION))
                .when(on, |d| {
                    d.bg(rgba(th.surface))
                        .text_color(rgba(th.text))
                        .font_weight(FontWeight::SEMIBOLD)
                        .shadow(elevation(th, level::CARD))
                })
                .when(!on, |d| {
                    d.text_color(rgba(th.text_dim))
                        .hover(|s| s.text_color(rgba(th.text)))
                })
                .child(label)
                .keeps_press()
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.set_kind(note, cx);
                    window.focus(&this.input.focus_handle(cx), cx);
                }))
        };
        div()
            .flex()
            .flex_row()
            .flex_none()
            .self_start()
            .p(px(space::S1))
            .gap(px(space::S1))
            .rounded_full()
            .bg(rgba(th.chip))
            .child(tab("capture-task", tr!("capture-task"), !self.note, false))
            .child(tab("capture-note", tr!("capture-note"), self.note, true))
            .into_any_element()
    }

    fn render_field(&self, th: &Theme) -> AnyElement {
        div()
            .keeps_press()
            .h(px(FIELD_HEIGHT))
            .px(px(space::S4))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(space::S3))
            .rounded(px(radius::SM))
            .bg(rgba(th.surface))
            .border_2()
            .border_color(rgba(th.accent))
            .child(icon(
                if self.note { "notes" } else { "add" },
                th.accent,
                16.0,
            ))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .text_size(px(text::BODY))
                    .text_color(rgba(th.text))
                    .child(self.input.clone()),
            )
            .into_any_element()
    }

    /// One chip (the `ix`th of its kind): filled in the accent for what
    /// the words said, outlined for what Katna picked.
    fn chip(
        &self,
        chip: Chip,
        ix: usize,
        label: String,
        typed: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let on = self.choosing == Some(chip);
        div()
            .id((chip.id(), ix))
            .h(px(CHIP_HEIGHT))
            .px(px(space::S3))
            .flex()
            .flex_row()
            .flex_none()
            .items_center()
            .gap(px(space::S2))
            .rounded_full()
            .cursor_pointer()
            .text_size(px(text::CAPTION))
            .when(typed, |d| d.bg(rgba(th.chip)).text_color(rgba(th.accent)))
            .when(!typed, |d| {
                d.border_1()
                    .border_color(rgba(th.outline))
                    .text_color(rgba(th.text_dim))
            })
            .when(on, |d| d.bg(rgba(th.nav_selected)))
            .hover(|s| s.bg(rgba(th.chip_hover())))
            .child(icon(
                chip.icon(),
                if typed { th.accent } else { th.text_dim },
                14.0,
            ))
            .child(label)
            .keeps_press()
            .on_click(cx.listener(move |this, _, _, cx| this.choose(chip, cx)))
            .into_any_element()
    }

    fn render_chips(&self, th: &Theme, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let typed = self.typed(cx);
        let labels = self.labels(&typed.labels);
        let mut chips = Vec::new();
        if self.note {
            for (ix, label) in labels.iter().enumerate() {
                chips.push(self.chip(Chip::Labels, ix, label.clone(), true, th, cx));
            }
            let place = self.place_name(self.place());
            chips.push(self.chip(
                Chip::Place,
                0,
                tr!("capture-note-place", place = place),
                self.place.is_some(),
                th,
                cx,
            ));
            return chips;
        }
        let (day, time) = self.due(&typed);
        if let Some(day) = day {
            chips.push(self.chip(Chip::Due, 0, due_text(day, time), true, th, cx));
        }
        for (ix, label) in labels.iter().enumerate() {
            chips.push(self.chip(Chip::Labels, ix, label.clone(), true, th, cx));
        }
        if let Some(list) = self.list() {
            let label = if list.account.is_empty() {
                tr!("capture-list-here", list = list.title.as_str())
            } else {
                tr!(
                    "capture-list",
                    list = list.title.as_str(),
                    account = list.account.as_str()
                )
            };
            chips.push(self.chip(Chip::List, 0, label, self.list.is_some(), th, cx));
        }
        if day.is_some() {
            chips.push(self.chip(
                Chip::Remind,
                0,
                self.remind(time).label(time.is_some()),
                self.remind.is_some(),
                th,
                cx,
            ));
        } else if typed.due.is_none() && self.due.is_none() {
            // A day to pick, when none was typed.
            chips.insert(
                0,
                self.chip(Chip::Due, 0, tr!("capture-add-date"), false, th, cx),
            );
        }
        chips
    }

    /// The choices of the chip being changed, in place of the chips.
    fn render_choices(&self, chip: Chip, th: &Theme, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let typed = self.typed(cx);
        let (day, time) = self.due(&typed);
        let choice = |id: (&'static str, usize), label: String, on: bool| {
            div()
                .id(id)
                .h(px(CHIP_HEIGHT))
                .px(px(space::S3))
                .flex()
                .flex_none()
                .items_center()
                .rounded_full()
                .cursor_pointer()
                .text_size(px(text::CAPTION))
                .border_1()
                .border_color(rgba(if on { th.nav_selected } else { th.outline }))
                .bg(rgba(if on { th.nav_selected } else { 0 }))
                .text_color(rgba(if on { th.nav_selected_text } else { th.text }))
                .hover(|s| s.bg(rgba(th.chip_hover())))
                .child(label)
        };
        let back = div()
            .id("capture-back")
            .size(px(CHIP_HEIGHT))
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .rounded_full()
            .cursor_pointer()
            .relative()
            .child(crate::widgets::hover_fade("hover-glow", None, th))
            .child(icon("chevron-left", th.text_dim, 16.0))
            .keeps_press()
            .on_click(cx.listener(|this, _, _, cx| {
                this.choosing = None;
                cx.notify();
            }))
            .into_any_element();
        let mut out = vec![back];
        match chip {
            Chip::Due => {
                let today = today();
                let days = [
                    (tr!("tasks-due-today"), Some(today)),
                    (tr!("tasks-due-tomorrow"), today.tomorrow().ok()),
                    (tr!("capture-next-week"), next_monday(today)),
                    (tr!("capture-no-date"), None),
                ];
                for (ix, (label, pick)) in days.into_iter().enumerate() {
                    let on = pick == day;
                    out.push(
                        choice(("capture-day", ix), label, on)
                            .keeps_press()
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.due = Some(match pick {
                                    Some(day) => DueChoice::Day(day),
                                    None => DueChoice::None,
                                });
                                this.choosing = None;
                                cx.notify();
                            }))
                            .into_any_element(),
                    );
                }
            }
            Chip::Remind => {
                let now = self.remind(time);
                for (ix, remind) in Remind::ALL.into_iter().enumerate() {
                    out.push(
                        choice(
                            ("capture-reminder", ix),
                            remind.label(time.is_some()),
                            remind == now,
                        )
                        .keeps_press()
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.remind = Some(remind);
                            this.choosing = None;
                            cx.notify();
                        }))
                        .into_any_element(),
                    );
                }
            }
            Chip::List => {
                let now = self.list().map(|l| l.id);
                for (ix, list) in self.places.lists.iter().enumerate() {
                    let label = if list.account.is_empty() {
                        tr!("capture-list-here", list = list.title.as_str())
                    } else {
                        tr!(
                            "capture-list",
                            list = list.title.as_str(),
                            account = list.account.as_str()
                        )
                    };
                    let id = list.id;
                    out.push(
                        choice(("capture-list-choice", ix), label, now == Some(id))
                            .keeps_press()
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.list = Some(id);
                                this.choosing = None;
                                cx.notify();
                            }))
                            .into_any_element(),
                    );
                }
            }
            Chip::Labels => {
                let now = self.labels(&typed.labels);
                let mut names = self.places.labels.clone();
                for label in &now {
                    if !names.iter().any(|n| n == label) {
                        names.push(label.clone());
                    }
                }
                for (ix, name) in names.into_iter().enumerate() {
                    let on = now.contains(&name);
                    out.push(
                        choice(("capture-label-choice", ix), name.clone(), on)
                            .keeps_press()
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.labels = Some(vec![name.clone()]);
                                this.choosing = None;
                                cx.notify();
                            }))
                            .into_any_element(),
                    );
                }
                out.push(
                    choice(
                        ("capture-label-choice", usize::MAX),
                        tr!("capture-no-label"),
                        now.is_empty(),
                    )
                    .keeps_press()
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.labels = Some(Vec::new());
                        this.choosing = None;
                        cx.notify();
                    }))
                    .into_any_element(),
                );
            }
            Chip::Place => {
                let now = self.place();
                let places = self
                    .places
                    .note_accounts
                    .iter()
                    .map(|(id, _)| Some(*id))
                    .chain([None]);
                for (ix, place) in places.enumerate() {
                    out.push(
                        choice(
                            ("capture-place-choice", ix),
                            self.place_name(place),
                            now == place,
                        )
                        .keeps_press()
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.place = Some(place);
                            this.choosing = None;
                            cx.notify();
                        }))
                        .into_any_element(),
                    );
                }
            }
        }
        out
    }

    fn render_hint(&self, th: &Theme) -> AnyElement {
        let (left, color) = match &self.error {
            Some(error) => (error.clone(), th.error),
            None if self.note => (tr!("capture-hint-note"), th.text_faint),
            None => (tr!("capture-hint-task"), th.text_faint),
        };
        div()
            .flex()
            .flex_row()
            .justify_between()
            .gap(px(space::S3))
            .text_size(px(text::MICRO))
            .text_color(rgba(th.text_faint))
            .child(div().text_color(rgba(color)).child(left))
            .child(tr!("capture-esc"))
            .into_any_element()
    }

    /// Fits the window to the card as last drawn, so a second row of
    /// chips has room.
    fn fit(&self, window: &mut Window) {
        let height = self.height.get();
        if height <= 0.0 {
            return;
        }
        let want = height + 2.0 * MARGIN;
        let now = unpx(window.viewport_size().height);
        if (want - now).abs() >= 1.0 {
            window.resize(size(window.viewport_size().width, px(want)));
        }
    }
}

impl Focusable for CaptureCard {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for CaptureCard {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        crate::widgets::follow_direction(window);
        let th = self.theme(window);
        let accent = rgba(th.accent).into();
        self.input.update(cx, |input, _| input.set_accent(accent));
        window.set_window_title(&tr!("capture-title"));
        // Only the card is the window: the shadow around it is not.
        window.set_client_inset(px(MARGIN));
        let viewport = window.viewport_size();
        window.set_input_region(Some(&[Bounds::new(
            point(px(MARGIN), px(MARGIN)),
            size(
                viewport.width - px(2.0 * MARGIN),
                viewport.height - px(2.0 * MARGIN),
            ),
        )]));
        katna_ui::native::set_client_corner_radius(radius::LG);
        self.fit(window);
        let row = match self.choosing {
            Some(chip) => self.render_choices(chip, &th, cx),
            None => self.render_chips(&th, cx),
        };
        let height = self.height.clone();
        let card = div()
            .id("capture-card")
            .key_context(CONTEXT)
            // Empty space moves the card, as a title bar would.
            .window_drag()
            .on_action(cx.listener(Self::switch_kind))
            .relative()
            .w_full()
            .max_w(px(WIDTH))
            .flex_none()
            .flex()
            .flex_col()
            .gap(px(space::S3))
            .p(px(space::S4))
            .rounded(px(radius::LG))
            .bg(rgba(self.fill(&th)))
            .shadow(elevation(&th, level::POPOVER))
            .text_color(rgba(th.text))
            .child(
                canvas(
                    move |bounds, window, _| {
                        // A new height fits the window to it next frame
                        // (a refresh asked for while drawing is dropped).
                        let drawn = unpx(bounds.size.height);
                        if (height.replace(drawn) - drawn).abs() >= 1.0 {
                            window.on_next_frame(|window, _| window.refresh());
                        }
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            )
            .child(self.render_tabs(&th, cx))
            .child(self.render_field(&th))
            .when(!row.is_empty(), |d| {
                d.child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .items_center()
                        .gap(px(space::S2))
                        .children(row),
                )
            })
            .child(self.render_hint(&th));
        // The card keeps its own height, not the window's, so the
        // window can be fitted to it.
        let root = div()
            .size_full()
            .flex()
            .flex_col()
            .items_start()
            .p(px(MARGIN))
            .child(card);
        match &self.font {
            Some(font) => root.font_family(font.clone()).into_any_element(),
            None => root.into_any_element(),
        }
    }
}

/// Sends a task's labels, which come with task labels in `pim.db`; a
/// daemon without them leaves them out.
async fn set_task_labels(
    connection: &Connection,
    id: i64,
    labels: Vec<String>,
) -> Result<(), String> {
    use katna_dbus::agenda::{AgendaProxy, Item};
    use katna_dbus::zbus::zvariant::{OwnedValue, Value};
    let agenda = AgendaProxy::new(connection)
        .await
        .map_err(|err| crate::daemon::describe(&err))?;
    let mut fields = Item::new();
    if let Ok(value) = OwnedValue::try_from(Value::from(labels)) {
        fields.insert(katna_dbus::agenda::edit::LABELS.to_owned(), value);
    }
    agenda
        .edit_task(&crate::tasks::wire_id(id), fields)
        .await
        .map_err(|err| crate::daemon::describe(&err))
}

/// The mail window reads its notes again.
fn notes_changed(cx: &mut App) {
    let main = cx
        .try_global::<CaptureHost>()
        .and_then(|h| h.main.clone())
        .and_then(|m| m.upgrade());
    if let Some(main) = main {
        main.update(cx, |main, cx| main.load_notes(cx));
    }
}

fn placeholder(note: bool) -> String {
    if note {
        tr!("notes-take-a-note")
    } else {
        tr!("capture-task-placeholder")
    }
}

fn today() -> Date {
    jiff::Zoned::now().date()
}

/// The Monday after `today`.
fn next_monday(today: Date) -> Option<Date> {
    let ahead = 7 - i64::from(today.weekday().to_monday_zero_offset());
    today.checked_add(jiff::Span::new().days(ahead)).ok()
}

/// A time of day, minutes after midnight, as the language writes it.
fn clock(minutes: u32) -> String {
    let time = jiff::civil::Time::new(
        i8::try_from(minutes / 60).unwrap_or(0),
        i8::try_from(minutes % 60).unwrap_or(0),
        0,
        0,
    )
    .unwrap_or_default();
    format::time(today().to_datetime(time))
}

/// The date chip: "Fri 9 Oct", with the time if one was typed.
fn due_text(day: Date, time: Option<u32>) -> String {
    let at = day.to_datetime(jiff::civil::Time::midnight());
    let day_text = tr!(
        "capture-day",
        weekday = format::weekday(at),
        day = format::day_month(at)
    );
    match time {
        Some(minutes) => tr!("tasks-due-at", day = day_text, time = clock(minutes)),
        None => day_text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_week_starts_on_monday() {
        let day = |text: &str| text.parse::<Date>().unwrap();
        assert_eq!(next_monday(day("2026-10-03")), Some(day("2026-10-05")));
        assert_eq!(next_monday(day("2026-10-05")), Some(day("2026-10-12")));
        assert_eq!(next_monday(day("2026-10-04")), Some(day("2026-10-05")));
    }

    #[test]
    fn reminders_count_from_the_due_time() {
        let due =
            katna_dav::todo::due_at("2026-10-09", Some(18 * 60), &jiff::tz::TimeZone::system())
                .unwrap();
        assert_eq!(Remind::OnTime.at("2026-10-09", Some(18 * 60)), Some(due));
        assert_eq!(
            Remind::HourBefore.at("2026-10-09", Some(18 * 60)),
            Some(due - 3600)
        );
        assert_eq!(Remind::Off.at("2026-10-09", Some(18 * 60)), None);
    }
}
