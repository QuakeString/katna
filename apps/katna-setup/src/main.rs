// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna Setup: installs Katna Mail on Windows 10 and later for the current
//! user, updates it, and removes it (`docs/ARCHITECTURE.md` §27.2).
//!
//! One small window in Katna's look instead of a wizard: the logo, one
//! button, a progress bar, then Open Katna Mail. It follows Windows' light
//! or dark setting.
//!
//! ```text
//! katna-setup                    install or update, with the window
//! katna-setup --quiet            the same without a window
//! katna-setup --uninstall        remove (Settings > Apps runs this)
//! katna-setup --uninstall --quiet
//! ```

// No console window on Windows.
#![cfg_attr(windows, windows_subsystem = "windows")]

mod install;
mod payload;

use std::process::ExitCode;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use gpui::{
    AnyElement, App, Bounds, Context, FontWeight, Hsla, Image, ImageFormat, IntoElement,
    ParentElement, Render, SharedString, Styled, TitlebarOptions, Window, WindowAppearance,
    WindowBounds, WindowOptions, div, img, prelude::*, rgba, size,
};
use katna_i18n::tr;
use katna_ui::px;

use install::{Layout, Step};
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

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let quiet = args.iter().any(|a| a == "--quiet");
    let uninstall = args.iter().any(|a| a == "--uninstall");
    katna_i18n::init(TRANSLATIONS, None);
    katna_i18n::apply("");
    let layout = match Layout::from_env() {
        Ok(layout) => layout,
        Err(err) => {
            eprintln!("katna-setup: {err}");
            return ExitCode::FAILURE;
        }
    };
    let payload = match Payload::read(PAYLOAD) {
        Ok(payload) => payload,
        Err(err) => {
            eprintln!("katna-setup: {err}");
            return ExitCode::FAILURE;
        }
    };
    if quiet {
        let done = if uninstall {
            install::uninstall(&layout, false)
        } else if payload.entries.is_empty() {
            Err(std::io::Error::other(tr!("setup-empty")))
        } else {
            install::install(&layout, &payload, VERSION, &|_| {})
        };
        return match done {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("katna-setup: {err}");
                ExitCode::FAILURE
            }
        };
    }
    let start = if uninstall {
        Screen::ConfirmRemove { data: false }
    } else if payload.entries.is_empty() {
        Screen::Failed(tr!("setup-empty"))
    } else {
        Screen::Welcome
    };
    gpui_platform::application().run(move |cx: &mut App| {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(560.0), px(400.0)),
                cx,
            ))),
            titlebar: Some(TitlebarOptions {
                title: Some(tr!("setup-window-title").into()),
                ..Default::default()
            }),
            app_id: Some(katna_core::ids::MAIL_APP_ID.to_owned()),
            is_resizable: false,
            ..Default::default()
        };
        let opened = cx.open_window(options, |_, cx| {
            cx.new(|_| Setup {
                layout,
                update: false,
                screen: start,
                progress: Arc::new(Mutex::new(None)),
                logo: Arc::new(Image::from_bytes(ImageFormat::Png, LOGO.to_vec())),
            })
        });
        match opened {
            Ok(handle) => {
                let _ = handle.update(cx, |setup, _, _| setup.update = setup.layout.installed());
            }
            Err(err) => {
                eprintln!("katna-setup: {err}");
                cx.quit();
            }
        }
        cx.on_window_closed(|cx, _| cx.quit()).detach();
    });
    ExitCode::SUCCESS
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
    layout: Layout,
    /// Katna is installed already: the button says Update.
    update: bool,
    screen: Screen,
    /// The install's latest step, written by its thread: `Ok(None)` while
    /// it runs, the result once it ended.
    progress: Arc<Mutex<Option<Result<(), String>>>>,
    logo: Arc<Image>,
}

/// Setup's colours, as Katna Mail's in light and dark.
struct Colors {
    page: Hsla,
    text: Hsla,
    dim: Hsla,
    accent: Hsla,
    on_accent: Hsla,
    track: Hsla,
    hover: Hsla,
    danger: Hsla,
}

impl Colors {
    fn new(dark: bool) -> Self {
        if dark {
            Self {
                page: rgba(0x1f1f1fff).into(),
                text: rgba(0xe3e3e3ff).into(),
                dim: rgba(0xc4c7c5ff).into(),
                accent: rgba(0xa8c7faff).into(),
                on_accent: rgba(0x062e6fff).into(),
                track: rgba(0xa8c7fa33).into(),
                hover: rgba(0xffffff14).into(),
                danger: rgba(0xf2b8b5ff).into(),
            }
        } else {
            Self {
                page: rgba(0xffffffff).into(),
                text: rgba(0x1f1f1fff).into(),
                dim: rgba(0x444746ff).into(),
                accent: rgba(0x0b57d0ff).into(),
                on_accent: rgba(0xffffffff).into(),
                track: rgba(0x0b57d029).into(),
                hover: rgba(0x0b57d014).into(),
                danger: rgba(0xb3261eff).into(),
            }
        }
    }
}

impl Setup {
    fn install(&mut self, cx: &mut Context<Self>) {
        self.screen = Screen::Installing(Step::Stopping);
        let step = Arc::new(Mutex::new(Step::Stopping));
        *self.progress.lock().unwrap() = None;
        let layout = self.layout.clone();
        let (shared_step, result) = (step.clone(), self.progress.clone());
        std::thread::spawn(move || {
            let payload = Payload::read(PAYLOAD).map_err(|e| e.to_string());
            let done = payload.and_then(|payload| {
                install::install(&layout, &payload, VERSION, &|s| {
                    *shared_step.lock().unwrap() = s;
                })
                .map_err(|e| e.to_string())
            });
            *result.lock().unwrap() = Some(done);
        });
        self.follow(step, Screen::Done, cx);
    }

    fn remove(&mut self, data: bool, cx: &mut Context<Self>) {
        self.screen = Screen::Removing;
        *self.progress.lock().unwrap() = None;
        let layout = self.layout.clone();
        let result = self.progress.clone();
        std::thread::spawn(move || {
            let done = install::uninstall(&layout, data).map_err(|e| e.to_string());
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
        let mail = self.layout.programs.join(install::MAIL_EXE);
        if let Err(err) = std::process::Command::new(&mail).spawn() {
            tracing::warn!(%err, "cannot open Katna Mail");
        }
        cx.quit();
    }
}

impl Render for Setup {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dark = matches!(
            window.appearance(),
            WindowAppearance::Dark | WindowAppearance::VibrantDark
        );
        let c = Colors::new(dark);
        let (title, body, bar, buttons) = self.content(&c, cx);
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(c.page)
            .text_color(c.text)
            .p(px(32.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(20.0))
                    .child(img(self.logo.clone()).size(px(72.0)).flex_none())
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
                    .justify_center()
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
            )
    }
}

impl Setup {
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
                let path = self.layout.programs.display().to_string();
                let label = if self.update {
                    tr!("setup-update")
                } else {
                    tr!("setup-install")
                };
                (
                    tr!("setup-app-name").into(),
                    vec![
                        text(tr!("setup-where", path = path.as_str())),
                        dim(tr!("setup-version", version = VERSION)),
                    ],
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
                let title = if self.update {
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
                vec![text(tr!("setup-done-body"))],
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
                        checkbox(data, tr!("setup-remove-data"), c, cx),
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

/// "Also delete my data", on the remove screen.
fn checkbox(on: bool, label: String, c: &Colors, cx: &mut Context<Setup>) -> AnyElement {
    let mut tick = div()
        .size(px(18.0))
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(3.0))
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
        .id("remove-data")
        .flex()
        .flex_row()
        .items_center()
        .gap(px(12.0))
        .cursor_pointer()
        .on_click(cx.listener(|setup, _, _, cx| {
            if let Screen::ConfirmRemove { data } = &mut setup.screen {
                *data = !*data;
            }
            cx.notify();
        }))
        .child(tick)
        .child(label)
        .into_any_element()
}
