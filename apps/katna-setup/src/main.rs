// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna Setup: installs Katna Mail on Windows 10 and later, updates it,
//! and removes it (`docs/ARCHITECTURE.md` §27.2).
//!
//! One small rounded window in Katna's look instead of a wizard: the logo,
//! who to install for, the folder, the shortcuts, one button, a progress
//! bar, then Open Katna Mail. It is drawn in Katna Mail's Mode, colour
//! scheme and accent, following Windows' light or dark setting by default.
//!
//! ```text
//! katna-setup                    install or update, with the window
//! katna-setup --quiet            the same without a window
//!   --all-users                  for everyone (needs an administrator)
//!   --dir <folder>               the programs folder
//!   --desktop                    add a desktop shortcut
//!   --no-start-menu              no Start menu shortcut
//!   --no-autostart               don't start Katna at sign-in
//!   --update                     keep the shortcuts as they are
//! katna-setup --uninstall        remove (Settings > Apps runs this)
//! katna-setup --uninstall --quiet [--all-users] [--delete-data]
//! ```

// No console window on Windows.
#![cfg_attr(windows, windows_subsystem = "windows")]

mod install;
mod payload;

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use gpui::{
    AnyElement, App, Bounds, BoxShadow, Context, FontWeight, Hsla, Image, ImageFormat, IntoElement,
    ParentElement, PathPromptOptions, Render, SharedString, Styled, TitlebarOptions, Window,
    WindowAppearance, WindowBackgroundAppearance, WindowBounds, WindowControlArea, WindowOptions,
    div, img, point, prelude::*, rgba, size,
};
use katna_core::Paths;
use katna_core::config::{Config, Theme as ThemeChoice};
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::schemes::{self, Side};

use install::{Choices, Layout, Scope, Step};
use payload::Payload;

/// The files Setup installs, packed by `build.rs`.
static PAYLOAD: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/payload.bin"));

/// Setup's translations, embedded by `build.rs`.
const TRANSLATIONS: katna_i18n::Sources = include!(concat!(env!("OUT_DIR"), "/translations.rs"));

/// The logo on Setup's window.
static LOGO: &[u8] =
    include_bytes!("../../../packaging/icons/hicolor/128x128/apps/in.invenia.katna.Mail.png");

/// The version Setup installs.
const VERSION: &str = katna_core::crash::VERSION;

/// The window's card, and the clear margin around it its shadow falls in.
const CARD: (f32, f32) = (580.0, 520.0);
const MARGIN: f32 = if WINDOWS_FRAME { 0.0 } else { 24.0 };
const RADIUS: f32 = 16.0;

/// On Windows the card is the whole window, and Windows draws its shadow,
/// border and, on Windows 11, round corners. A see-through window there
/// shows as a grey box behind the card, with Windows' own border and
/// shadow around that box instead.
const WINDOWS_FRAME: bool = cfg!(windows);

/// What the command line asks for.
#[derive(Debug, Default)]
struct Args {
    quiet: bool,
    uninstall: bool,
    all_users: bool,
    update: bool,
    delete_data: bool,
    dir: Option<PathBuf>,
    choices: Choices,
    /// The process ID of the Setup waiting for this one, running as
    /// administrator, to say how far it got (`install::write_progress`).
    progress: Option<u32>,
}

impl Args {
    fn parse(mut args: impl Iterator<Item = String>) -> Self {
        let mut out = Self::default();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--quiet" => out.quiet = true,
                "--uninstall" => out.uninstall = true,
                "--all-users" => out.all_users = true,
                "--update" => out.update = true,
                "--delete-data" => out.delete_data = true,
                "--desktop" => out.choices.desktop = true,
                "--no-start-menu" => out.choices.start_menu = false,
                "--no-autostart" => out.choices.autostart = false,
                "--dir" => out.dir = args.next().map(PathBuf::from),
                "--progress" => out.progress = args.next().and_then(|id| id.parse().ok()),
                _ => {}
            }
        }
        out
    }

    fn scope(&self) -> Scope {
        if self.all_users {
            Scope::Machine
        } else {
            Scope::User
        }
    }
}

/// The arguments that make a Setup running as administrator do what this
/// one would: install `layout` with `choices`, or update it.
fn elevated_args(layout: &Layout, choices: Option<Choices>, progress: u32) -> Vec<String> {
    let mut args = vec![
        "--quiet".to_owned(),
        "--all-users".to_owned(),
        "--dir".to_owned(),
        layout.programs.display().to_string(),
        "--progress".to_owned(),
        progress.to_string(),
    ];
    match choices {
        None => args.push("--update".to_owned()),
        Some(c) => {
            if c.desktop {
                args.push("--desktop".to_owned());
            }
            if !c.start_menu {
                args.push("--no-start-menu".to_owned());
            }
            if !c.autostart {
                args.push("--no-autostart".to_owned());
            }
        }
    }
    args
}

/// A step as a Setup running as administrator writes it for this one.
fn step_line(step: Step) -> String {
    match step {
        Step::Stopping => "stopping".to_owned(),
        Step::Copying(share) => format!("copying {share}"),
        Step::Registering => "registering".to_owned(),
    }
}

fn parse_step(line: &str) -> Option<Step> {
    match line.trim().split_once(' ') {
        Some(("copying", share)) => share.parse().ok().map(Step::Copying),
        _ => match line.trim() {
            "stopping" => Some(Step::Stopping),
            "registering" => Some(Step::Registering),
            _ => None,
        },
    }
}

fn main() -> ExitCode {
    let args = Args::parse(std::env::args().skip(1));
    katna_i18n::init(TRANSLATIONS, None);
    katna_i18n::apply("");
    let payload = match Payload::read(PAYLOAD) {
        Ok(payload) => payload,
        Err(err) => {
            eprintln!("katna-setup: {err}");
            return ExitCode::FAILURE;
        }
    };
    if args.quiet {
        return match quiet(&args, &payload) {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("katna-setup: {err}");
                ExitCode::FAILURE
            }
        };
    }
    let existing = Layout::existing();
    let start = if args.uninstall {
        match &existing {
            Some(_) => Screen::ConfirmRemove { data: false },
            None => Screen::Removed { data: false },
        }
    } else if payload.entries.is_empty() {
        Screen::Failed(tr!("setup-empty"))
    } else {
        Screen::Welcome
    };
    gpui_platform::application().run(move |cx: &mut App| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(CARD.0 + 2.0 * MARGIN), px(CARD.1 + 2.0 * MARGIN)),
                cx,
            ))),
            // Setup draws its own rounded card, shadow and close button,
            // the same on Windows 10 and 11.
            titlebar: Some(TitlebarOptions {
                title: Some(tr!("setup-window-title").into()),
                appears_transparent: true,
                ..Default::default()
            }),
            window_background: if WINDOWS_FRAME {
                WindowBackgroundAppearance::Opaque
            } else {
                WindowBackgroundAppearance::Transparent
            },
            app_id: Some(katna_core::ids::MAIL_APP_ID.to_owned()),
            is_resizable: false,
            ..Default::default()
        };
        let opened = cx.open_window(options, |_, cx| {
            cx.new(|_| Setup {
                existing,
                scope: Scope::User,
                folder: None,
                choices: Choices::default(),
                screen: start,
                progress: Arc::new(Mutex::new(None)),
                logo: Arc::new(Image::from_bytes(ImageFormat::Png, LOGO.to_vec())),
                look: Look::read(),
            })
        });
        if let Err(err) = opened {
            eprintln!("katna-setup: {err}");
            cx.quit();
        }
        cx.on_window_closed(|cx, _| cx.quit()).detach();
    });
    ExitCode::SUCCESS
}

/// Installs or removes without a window.
fn quiet(args: &Args, payload: &Payload<'_>) -> std::io::Result<()> {
    let mut layout = Layout::from_env(args.scope())?;
    if let Some(dir) = &args.dir {
        layout = layout.in_folder(dir);
    } else if let Some(existing) = Layout::existing().filter(|l| l.scope == args.scope()) {
        layout = existing;
    }
    if args.uninstall {
        return install::uninstall(&layout, args.delete_data);
    }
    if payload.entries.is_empty() {
        return Err(std::io::Error::other(tr!("setup-empty")));
    }
    let choices = (!args.update).then_some(args.choices);
    let installed = install::install(&layout, payload, VERSION, choices, &|step| {
        if let Some(id) = args.progress {
            install::write_progress(id, &step_line(step));
        }
    });
    if let Some(id) = args.progress {
        install::clear_progress(id);
    }
    installed
}

/// What the window shows.
#[derive(Debug, Clone, PartialEq)]
enum Screen {
    Welcome,
    Installing(Step),
    Done,
    Failed(String),
    ConfirmRemove { data: bool },
    Removing,
    Removed { data: bool },
}

struct Setup {
    /// Katna as installed already: Setup updates it in place.
    existing: Option<Layout>,
    scope: Scope,
    /// The folder the user picked, instead of the default for the scope.
    folder: Option<PathBuf>,
    choices: Choices,
    screen: Screen,
    /// The work's result, written by its thread: `None` while it runs.
    progress: Arc<Mutex<Option<Result<(), String>>>>,
    logo: Arc<Image>,
    look: Look,
}

/// Setup's colours, as Katna Mail's: its own palette in light and dark,
/// or the built-in colour scheme and accent picked in its Settings >
/// Appearance ([`Look`]).
struct Colors {
    page: Hsla,
    text: Hsla,
    dim: Hsla,
    accent: Hsla,
    on_accent: Hsla,
    track: Hsla,
    hover: Hsla,
    danger: Hsla,
    outline: Hsla,
    field: Hsla,
}

/// [`Colors`] as `0xRRGGBBAA`.
struct Palette {
    page: u32,
    text: u32,
    dim: u32,
    accent: u32,
    on_accent: u32,
    track: u32,
    hover: u32,
    danger: u32,
    outline: u32,
    field: u32,
}

impl Palette {
    /// Katna's own colours.
    fn katna(dark: bool) -> Self {
        if dark {
            Self {
                page: 0x1f1f1fff,
                text: 0xe3e3e3ff,
                dim: 0xc4c7c5ff,
                accent: 0xa8c7faff,
                on_accent: 0x062e6fff,
                track: 0xa8c7fa33,
                hover: 0xffffff14,
                danger: 0xf2b8b5ff,
                outline: 0xffffff1f,
                field: 0x2a2a2aff,
            }
        } else {
            Self {
                page: 0xffffffff,
                text: 0x1f1f1fff,
                dim: 0x444746ff,
                accent: 0x0b57d0ff,
                on_accent: 0xffffffff,
                track: 0x0b57d029,
                hover: 0x0b57d014,
                danger: 0xb3261eff,
                outline: 0x0000001a,
                field: 0xf0f4f9ff,
            }
        }
    }

    /// A built-in scheme's side: Setup's card is its cards' colour.
    fn scheme(side: &Side, dark: bool) -> Self {
        let mut palette = Self {
            page: side.card,
            text: side.text,
            dim: side.faint,
            accent: 0,
            on_accent: 0,
            track: 0,
            hover: if dark { 0xffffff14 } else { 0 },
            danger: side.error,
            outline: fade(side.text, if dark { 0.12 } else { 0.1 }),
            field: mix(side.card, side.text, 0.06),
        };
        palette.set_accent(side.accent, dark);
        palette
    }

    fn set_accent(&mut self, accent: u32, dark: bool) {
        self.accent = accent | 0xff;
        self.on_accent = if luminance(accent) > 0.4 {
            0x1f1f1fff
        } else {
            0xffffffff
        };
        self.track = fade(accent, if dark { 0.2 } else { 0.16 });
        if !dark {
            self.hover = fade(accent, 0.08);
        }
    }

    fn colors(&self) -> Colors {
        let c = |color: u32| -> Hsla { rgba(color).into() };
        Colors {
            page: c(self.page),
            text: c(self.text),
            dim: c(self.dim),
            accent: c(self.accent),
            on_accent: c(self.on_accent),
            track: c(self.track),
            hover: c(self.hover),
            danger: c(self.danger),
            outline: c(self.outline),
            field: c(self.field),
        }
    }
}

/// `color` with its alpha at `alpha`.
fn fade(color: u32, alpha: f32) -> u32 {
    (color & 0xffffff00) | ((alpha.clamp(0.0, 1.0) * 255.0).round() as u32)
}

/// `a` mixed `t` of the way to `b`, opaque.
fn mix(a: u32, b: u32, t: f32) -> u32 {
    let channel = |shift: u32| {
        let (x, y) = (((a >> shift) & 0xff) as f32, ((b >> shift) & 0xff) as f32);
        ((x + (y - x) * t).round() as u32) << shift
    };
    channel(24) | channel(16) | channel(8) | 0xff
}

/// Relative luminance, 0–1.
fn luminance(color: u32) -> f32 {
    let channel = |shift: u32| {
        let v = ((color >> shift) & 0xff) as f32 / 255.0;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * channel(24) + 0.7152 * channel(16) + 0.0722 * channel(8)
}

/// What Katna Mail's Settings > Appearance says, from the settings file of
/// the user running Setup: Mode, Colors and Accent. A scheme Setup can't
/// draw (the desktop's, one's own) is Katna's palette in the accent.
#[derive(Debug)]
struct Look {
    mode: ThemeChoice,
    colors: String,
    accent: Option<u32>,
}

impl Look {
    fn read() -> Self {
        let config = Paths::from_env()
            .ok()
            .and_then(|paths| Config::load(&paths.config_file()).ok());
        Self::from(&config.unwrap_or_default())
    }

    fn from(config: &Config) -> Self {
        let accent = &config.mail.accent;
        let accent = accent
            .strip_prefix('#')
            .filter(|hex| hex.len() == 6)
            .and_then(|hex| u32::from_str_radix(hex, 16).ok())
            .map(|rgb| (rgb << 8) | 0xff);
        Self {
            mode: config.mail.theme,
            colors: config.mail.colors().to_owned(),
            accent,
        }
    }

    /// Dark when Mode says so, or with the desktop (`desktop_dark`) when
    /// it follows it.
    fn dark(&self, desktop_dark: bool) -> bool {
        match self.mode {
            ThemeChoice::System => desktop_dark,
            ThemeChoice::Light => false,
            ThemeChoice::Dark => true,
        }
    }
}

impl Colors {
    fn new(dark: bool, look: &Look) -> Self {
        let mut palette = match schemes::built_in(&look.colors) {
            Some(scheme) => Palette::scheme(scheme.side(dark), dark),
            None => Palette::katna(dark),
        };
        if let Some(accent) = look.accent {
            palette.set_accent(accent, dark);
        }
        palette.colors()
    }
}

impl Setup {
    /// Where the install goes: the installed Katna for an update, else the
    /// scope's default folder or the one picked.
    fn target(&self) -> std::io::Result<Layout> {
        if let Some(existing) = &self.existing {
            return Ok(existing.clone());
        }
        let layout = Layout::from_env(self.scope)?;
        Ok(match &self.folder {
            Some(dir) => layout.in_folder(dir),
            None => layout,
        })
    }

    fn install(&mut self, cx: &mut Context<Self>) {
        let layout = match self.target().and_then(|l| l.check_folder().map(|()| l)) {
            Ok(layout) => layout,
            Err(err) => {
                self.screen = Screen::Failed(err.to_string());
                return;
            }
        };
        let choices = self.existing.is_none().then_some(self.choices);
        self.screen = Screen::Installing(Step::Stopping);
        let step = Arc::new(Mutex::new(Step::Stopping));
        *self.progress.lock().unwrap() = None;
        let (shared_step, result) = (step.clone(), self.progress.clone());
        if layout.scope == Scope::Machine && !install::elevated() {
            // A second Setup does the work as administrator and records
            // how far it got where this one reads it.
            let id = std::process::id();
            let args = elevated_args(&layout, choices, id);
            let stop = Arc::new(Mutex::new(false));
            let stopped = stop.clone();
            std::thread::spawn(move || {
                while !*stopped.lock().unwrap() {
                    if let Some(now) = install::read_progress(id).and_then(|l| parse_step(&l)) {
                        *shared_step.lock().unwrap() = now;
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
            });
            std::thread::spawn(move || {
                let done = match install::run_elevated(&args) {
                    Ok(0) => Ok(()),
                    Ok(_) => Err(tr!("setup-everyone-failed")),
                    Err(_) => Err(tr!("setup-everyone-refused")),
                };
                *stop.lock().unwrap() = true;
                *result.lock().unwrap() = Some(done);
            });
        } else {
            std::thread::spawn(move || {
                let payload = Payload::read(PAYLOAD).map_err(|e| e.to_string());
                let done = payload.and_then(|payload| {
                    install::install(&layout, &payload, VERSION, choices, &|s| {
                        *shared_step.lock().unwrap() = s;
                    })
                    .map_err(|e| e.to_string())
                });
                *result.lock().unwrap() = Some(done);
            });
        }
        self.follow(step, Screen::Done, cx);
    }

    fn remove(&mut self, data: bool, cx: &mut Context<Self>) {
        let Some(layout) = self.existing.clone() else {
            return;
        };
        self.screen = Screen::Removing;
        *self.progress.lock().unwrap() = None;
        let result = self.progress.clone();
        std::thread::spawn(move || {
            let done = if layout.scope == Scope::Machine && !install::elevated() {
                // The administrator removes the programs; the user's own
                // mail and passwords are deleted here, as that user.
                let args = ["--uninstall", "--quiet", "--all-users"].map(str::to_owned);
                match install::run_elevated(&args) {
                    Ok(0) if data => install::delete_data().map_err(|e| e.to_string()),
                    Ok(0) => Ok(()),
                    Ok(_) => Err(tr!("setup-everyone-failed")),
                    Err(_) => Err(tr!("setup-everyone-refused")),
                }
            } else {
                install::uninstall(&layout, data).map_err(|e| e.to_string())
            };
            *result.lock().unwrap() = Some(done);
        });
        self.follow(
            Arc::new(Mutex::new(Step::Stopping)),
            Screen::Removed { data },
            cx,
        );
    }

    /// Redraws while the work on the other thread runs, then shows `done`
    /// or what went wrong.
    fn follow(&mut self, step: Arc<Mutex<Step>>, done: Screen, cx: &mut Context<Self>) {
        let result = self.progress.clone();
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;
                let ended = result.lock().unwrap().take();
                let now = *step.lock().unwrap();
                let updated = this.update(cx, |setup, cx| {
                    match &ended {
                        Some(Ok(())) => setup.screen = done.clone(),
                        Some(Err(err)) => setup.screen = Screen::Failed(err.clone()),
                        None => {
                            if matches!(setup.screen, Screen::Installing(_)) {
                                setup.screen = Screen::Installing(now);
                            }
                        }
                    }
                    cx.notify();
                });
                if ended.is_some() || updated.is_err() {
                    return;
                }
            }
        })
        .detach();
    }

    fn open_katna(&self, cx: &mut Context<Self>) {
        if let Ok(layout) = self.target() {
            let mail = layout.programs.join(install::MAIL_EXE);
            if let Err(err) = std::process::Command::new(&mail).spawn() {
                tracing::warn!(%err, "cannot open Katna Mail");
            }
        }
        cx.quit();
    }

    /// Asks for the folder to install into.
    fn pick_folder(&mut self, cx: &mut Context<Self>) {
        let picked = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some(tr!("setup-folder-pick").into()),
        });
        cx.spawn(async move |this, cx| {
            if let Ok(Ok(Some(mut paths))) = picked.await
                && let Some(dir) = paths.pop()
            {
                let _ = this.update(cx, |setup, cx| {
                    setup.folder = Some(dir);
                    cx.notify();
                });
            }
        })
        .detach();
    }
}

impl Render for Setup {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dark = self.look.dark(matches!(
            window.appearance(),
            WindowAppearance::Dark | WindowAppearance::VibrantDark
        ));
        let c = Colors::new(dark, &self.look);
        let (title, body, bar, buttons) = self.content(&c, cx);
        let busy = matches!(self.screen, Screen::Installing(_) | Screen::Removing);
        // The choices stay put when the note under "Install for" comes and
        // goes; the short screens sit in the middle.
        let choosing = self.screen == Screen::Welcome && self.existing.is_none();
        let card = div()
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .bg(c.page)
            .text_color(c.text)
            .when(!WINDOWS_FRAME, |card| {
                card.rounded(px(RADIUS))
                    .border_1()
                    .border_color(c.outline)
                    .shadow(vec![
                        BoxShadow {
                            color: rgba(0x0000002e).into(),
                            offset: point(px(0.0), px(8.0)),
                            blur_radius: px(24.0),
                            spread_radius: px(0.0),
                            inset: false,
                        },
                        BoxShadow {
                            color: rgba(0x0000001f).into(),
                            offset: point(px(0.0), px(1.0)),
                            blur_radius: px(4.0),
                            spread_radius: px(0.0),
                            inset: false,
                        },
                    ])
            })
            .p(px(32.0))
            .child(drag_area())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(20.0))
                    .child(img(self.logo.clone()).size(px(64.0)).flex_none())
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .text_size(px(24.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child(title),
                            )
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(c.dim)
                                    .child(tr!("setup-tagline")),
                            ),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .when(choosing, |d| d.pt(px(28.0)))
                    .when(!choosing, |d| d.justify_center())
                    .gap(px(16.0))
                    .text_size(px(14.0))
                    .line_height(px(21.0))
                    .children(body)
                    .children(bar),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_end()
                    .gap(px(8.0))
                    .children(buttons),
            );
        let card = if busy {
            card
        } else {
            card.child(close_button(&c, cx))
        };
        div().size_full().p(px(MARGIN)).child(card)
    }
}

/// The top of the card, which moves the window.
fn drag_area() -> impl IntoElement {
    div()
        .id("drag")
        .absolute()
        .top_0()
        .left_0()
        .right(px(56.0))
        .h(px(56.0))
        .window_control_area(WindowControlArea::Drag)
        .on_mouse_down(gpui::MouseButton::Left, |_, window, _| {
            // Windows moves it through the control area above.
            if !cfg!(windows) {
                window.start_window_move();
            }
        })
}

/// The X at the top right.
fn close_button(c: &Colors, cx: &mut Context<Setup>) -> AnyElement {
    div()
        .id("close-window")
        .absolute()
        .top(px(12.0))
        .right(px(12.0))
        .size(px(36.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded_full()
        .text_color(c.dim)
        .text_size(px(18.0))
        .cursor_pointer()
        .hover(|s| s.bg(c.hover))
        .on_click(cx.listener(|_, _, _, cx| cx.quit()))
        .child("✕")
        .into_any_element()
}

impl Setup {
    /// The Welcome screen's choices: who for, where, and what to add.
    fn choices_view(&self, c: &Colors, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let folder = self
            .target()
            .map(|l| l.programs.display().to_string())
            .unwrap_or_default();
        let scope = self.scope;
        let pill = |id: &'static str, label: String, this: Scope, cx: &mut Context<Self>| {
            let on = scope == this;
            let mut pill = div()
                .id(id)
                .h(px(36.0))
                .px(px(16.0))
                .flex()
                .items_center()
                .rounded(px(18.0))
                .border_1()
                .cursor_pointer()
                .text_size(px(14.0))
                .on_click(cx.listener(move |setup, _, _, cx| {
                    if setup.scope != this {
                        setup.scope = this;
                        setup.folder = None;
                    }
                    cx.notify();
                }))
                .child(label);
            pill = if on {
                pill.bg(c.track).border_color(c.accent).text_color(c.text)
            } else {
                pill.border_color(c.outline)
                    .text_color(c.dim)
                    .hover(|s| s.bg(c.hover))
            };
            pill.into_any_element()
        };
        let label = |s: String| {
            div()
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(c.dim)
                .child(s)
        };
        let mut who = div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(label(tr!("setup-for")))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .child(pill("for-me", tr!("setup-for-me"), Scope::User, cx))
                    .child(pill(
                        "for-everyone",
                        tr!("setup-for-everyone"),
                        Scope::Machine,
                        cx,
                    )),
            );
        if scope == Scope::Machine {
            who = who.child(
                div()
                    .text_size(px(13.0))
                    .text_color(c.dim)
                    .child(tr!("setup-for-everyone-note")),
            );
        }
        let place = div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(label(tr!("setup-folder")))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .h(px(36.0))
                            .px(px(12.0))
                            .flex()
                            .items_center()
                            .rounded(px(8.0))
                            .bg(c.field)
                            .text_size(px(13.0))
                            .overflow_hidden()
                            .whitespace_nowrap()
                            .text_ellipsis()
                            .child(folder),
                    )
                    .child(secondary(
                        "change-folder",
                        tr!("setup-folder-change"),
                        c,
                        cx,
                        |setup, cx| setup.pick_folder(cx),
                    )),
            );
        let ch = self.choices;
        let boxes = div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(checkbox(
                "desktop",
                ch.desktop,
                tr!("setup-desktop-shortcut"),
                c,
                cx,
                |s| s.choices.desktop = !s.choices.desktop,
            ))
            .child(checkbox(
                "start-menu",
                ch.start_menu,
                tr!("setup-start-menu"),
                c,
                cx,
                |s| s.choices.start_menu = !s.choices.start_menu,
            ))
            .child(checkbox(
                "autostart",
                ch.autostart,
                tr!("setup-autostart"),
                c,
                cx,
                |s| s.choices.autostart = !s.choices.autostart,
            ));
        vec![
            who.into_any_element(),
            place.into_any_element(),
            boxes.into_any_element(),
        ]
    }

    /// The title, the text, the progress bar and the buttons of the screen.
    fn content(
        &self,
        c: &Colors,
        cx: &mut Context<Self>,
    ) -> (
        SharedString,
        Vec<AnyElement>,
        Option<AnyElement>,
        Vec<AnyElement>,
    ) {
        let text = |s: String| div().child(s).into_any_element();
        let dim = |s: String| div().text_color(c.dim).child(s).into_any_element();
        match &self.screen {
            Screen::Welcome => {
                let (label, body) = match &self.existing {
                    Some(existing) => {
                        let path = existing.programs.display().to_string();
                        (
                            tr!("setup-update"),
                            vec![
                                text(tr!("setup-update-where", path = path.as_str())),
                                dim(tr!("setup-version", version = VERSION)),
                            ],
                        )
                    }
                    None => (tr!("setup-install"), self.choices_view(c, cx)),
                };
                (
                    tr!("setup-app-name").into(),
                    body,
                    None,
                    vec![
                        secondary("cancel", tr!("setup-cancel"), c, cx, |_, cx| cx.quit()),
                        primary("install", label, c.accent, c, cx, |setup, cx| {
                            setup.install(cx)
                        }),
                    ],
                )
            }
            Screen::Installing(step) => {
                let (share, what) = match step {
                    Step::Stopping => (0.02, tr!("setup-step-stopping")),
                    Step::Copying(share) => (0.05 + share * 0.85, tr!("setup-step-copying")),
                    Step::Registering => (0.95, tr!("setup-step-registering")),
                };
                let title = if self.existing.is_some() {
                    tr!("setup-updating")
                } else {
                    tr!("setup-installing")
                };
                (
                    title.into(),
                    vec![dim(what)],
                    Some(progress_bar(share, c)),
                    Vec::new(),
                )
            }
            Screen::Done => (
                tr!("setup-done-title").into(),
                vec![text(tr!("setup-done-body")), dim(tr!("setup-pin-hint"))],
                None,
                vec![
                    secondary("close", tr!("setup-close"), c, cx, |_, cx| cx.quit()),
                    primary("open", tr!("setup-open"), c.accent, c, cx, |setup, cx| {
                        setup.open_katna(cx)
                    }),
                ],
            ),
            Screen::Failed(err) => {
                let retry = !PAYLOAD.is_empty();
                let mut buttons = vec![secondary("close", tr!("setup-close"), c, cx, |_, cx| {
                    cx.quit()
                })];
                if retry {
                    buttons.push(primary(
                        "retry",
                        tr!("setup-try-again"),
                        c.accent,
                        c,
                        cx,
                        |setup, cx| {
                            setup.screen = Screen::Welcome;
                            cx.notify();
                        },
                    ));
                }
                (
                    tr!("setup-failed-title").into(),
                    vec![dim(err.clone())],
                    None,
                    buttons,
                )
            }
            Screen::ConfirmRemove { data } => {
                let data = *data;
                (
                    tr!("setup-remove-title").into(),
                    vec![
                        text(tr!("setup-remove-body")),
                        checkbox("remove-data", data, tr!("setup-remove-data"), c, cx, |s| {
                            if let Screen::ConfirmRemove { data } = &mut s.screen {
                                *data = !*data;
                            }
                        }),
                    ],
                    None,
                    vec![
                        secondary("cancel", tr!("setup-cancel"), c, cx, |_, cx| cx.quit()),
                        primary(
                            "remove",
                            tr!("setup-remove"),
                            c.danger,
                            c,
                            cx,
                            move |s, cx| s.remove(data, cx),
                        ),
                    ],
                )
            }
            Screen::Removing => (
                tr!("setup-removing").into(),
                Vec::new(),
                Some(progress_bar(0.5, c)),
                Vec::new(),
            ),
            Screen::Removed { data } => (
                tr!("setup-removed-title").into(),
                vec![text(if *data {
                    tr!("setup-removed-deleted")
                } else {
                    tr!("setup-removed-kept")
                })],
                None,
                vec![primary(
                    "close",
                    tr!("setup-close"),
                    c.accent,
                    c,
                    cx,
                    |_, cx| cx.quit(),
                )],
            ),
        }
    }
}

/// A filled button in `fill`.
fn primary(
    id: &'static str,
    label: String,
    fill: Hsla,
    c: &Colors,
    cx: &mut Context<Setup>,
    on_click: impl Fn(&mut Setup, &mut Context<Setup>) + 'static,
) -> AnyElement {
    div()
        .id(id)
        .h(px(40.0))
        .px(px(24.0))
        .flex()
        .items_center()
        .rounded(px(20.0))
        .bg(fill)
        .text_color(c.on_accent)
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .cursor_pointer()
        .hover(|s| s.opacity(0.92))
        .on_click(cx.listener(move |setup, _, _, cx| on_click(setup, cx)))
        .child(label)
        .into_any_element()
}

/// A text button.
fn secondary(
    id: &'static str,
    label: String,
    c: &Colors,
    cx: &mut Context<Setup>,
    on_click: impl Fn(&mut Setup, &mut Context<Setup>) + 'static,
) -> AnyElement {
    div()
        .id(id)
        .h(px(40.0))
        .px(px(20.0))
        .flex()
        .items_center()
        .rounded(px(20.0))
        .text_color(c.accent)
        .text_size(px(14.0))
        .font_weight(FontWeight::MEDIUM)
        .cursor_pointer()
        .hover(|s| s.bg(c.hover))
        .on_click(cx.listener(move |setup, _, _, cx| on_click(setup, cx)))
        .child(label)
        .into_any_element()
}

/// The share done, as a bar.
fn progress_bar(share: f32, c: &Colors) -> AnyElement {
    div()
        .w_full()
        .h(px(6.0))
        .rounded(px(3.0))
        .bg(c.track)
        .child(
            div()
                .h_full()
                .w(gpui::relative(share.clamp(0.0, 1.0)))
                .rounded(px(3.0))
                .bg(c.accent),
        )
        .into_any_element()
}

/// A tick box with its label; clicking either calls `toggle`.
fn checkbox(
    id: &'static str,
    on: bool,
    label: String,
    c: &Colors,
    cx: &mut Context<Setup>,
    toggle: impl Fn(&mut Setup) + 'static,
) -> AnyElement {
    let mut tick = div()
        .size(px(18.0))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(4.0))
        .border_2()
        .border_color(if on { c.accent } else { c.dim });
    if on {
        tick = tick
            .bg(c.accent)
            .text_color(c.on_accent)
            .text_size(px(13.0))
            .child("✓");
    }
    div()
        .id(id)
        .flex()
        .flex_row()
        .items_center()
        .gap(px(12.0))
        .cursor_pointer()
        .on_click(cx.listener(move |setup, _, _, cx| {
            toggle(setup);
            cx.notify();
        }))
        .child(tick)
        .child(label)
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_follows_katna_mails_mode_scheme_and_accent() {
        let mut config = Config::default();
        let look = Look::from(&config);
        assert!(look.dark(true) && !look.dark(false), "follows the desktop");
        let katna = Colors::new(false, &look);
        assert_eq!(katna.accent, Hsla::from(rgba(0x0b57d0ff)));

        config.mail.theme = ThemeChoice::Dark;
        config.mail.set_colors("nord");
        config.mail.accent = "#e8590c".to_owned();
        let look = Look::from(&config);
        assert!(look.dark(false));
        let nord = Colors::new(true, &look);
        assert_eq!(nord.page, Hsla::from(rgba(0x3b4252ff)), "Nord's dark cards");
        assert_eq!(nord.accent, Hsla::from(rgba(0xe8590cff)));
        assert_eq!(nord.on_accent, Hsla::from(rgba(0xffffffff)));

        // Schemes Setup can't draw are Katna's.
        config.mail.set_colors("user:mine");
        config.mail.accent.clear();
        let mine = Colors::new(true, &Look::from(&config));
        assert_eq!(mine.page, Hsla::from(rgba(0x1f1f1fff)));
    }

    #[test]
    fn a_setup_for_everyone_gets_the_same_choices() {
        let layout = Layout {
            scope: Scope::Machine,
            programs: PathBuf::from("/Program Files/Katna"),
            start_menu: PathBuf::from("/Start"),
            desktop: PathBuf::from("/Desktop"),
        };
        let choices = Choices {
            desktop: true,
            start_menu: false,
            autostart: false,
        };
        let args = elevated_args(&layout, Some(choices), 42);
        let parsed = Args::parse(args.into_iter());
        assert!(parsed.quiet && parsed.all_users && !parsed.update);
        assert_eq!(parsed.choices, choices);
        assert_eq!(parsed.dir.as_deref(), Some(layout.programs.as_path()));
        assert_eq!(parsed.progress, Some(42));

        let update = elevated_args(&layout, None, 42);
        assert!(Args::parse(update.into_iter()).update);
    }

    #[test]
    fn steps_cross_to_the_waiting_setup() {
        for step in [Step::Stopping, Step::Copying(0.5), Step::Registering] {
            assert_eq!(parse_step(&step_line(step)), Some(step));
        }
        assert_eq!(parse_step(""), None);
    }
}
