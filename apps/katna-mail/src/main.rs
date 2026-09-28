// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna Mail. See `docs/ARCHITECTURE.md` §13.
//!
//! This first version reads the local store read-only: folders, message
//! list, reading pane (plain text) and search. Accounts, sync and sending
//! come with `katna-daemon`.

// No console window on Windows.
#![cfg_attr(all(windows, not(test)), windows_subsystem = "windows")]

mod assets;
mod autostart;
mod daemon;
mod data;
mod format;
mod grammar;
mod instance;
mod mailto;
mod outgoing;
mod placement;
mod profile;
mod receipts;
mod sidebar;
mod signatures;
mod sound;
mod spell;
mod suggest;
mod tabs;
mod templates;
mod theme;
mod updater;
mod whats_new;
mod widgets;
mod window;

use std::path::PathBuf;
use std::process::ExitCode;

use gpui::{App, AppContext, SharedString, size};
use katna_chrome::{Desktop, Environment, window_options};
use katna_core::Paths;
use katna_core::config::Config;
use katna_core::ids::{MAIL_APP_ID, MAIL_MENU_BAR_PATH};
use katna_platform::dbusmenu::Menu;
use katna_platform::font;
use katna_ui::scale::desktop_px;

const USAGE: &str = "\
Usage: katna-mail [--data-dir DIR] [--search QUERY | --compose | --inbox | --settings |
                  --message ID | --reply-all ID]
       katna-mail --background

When Katna Mail is already running, it comes to the front and does what
the options ask; a second window does not open.

Options:
  --data-dir DIR   Use DIR/data, DIR/config and DIR/cache instead of the
                   XDG directories (the same layout as katna-search-cli)
  --search QUERY   Search for QUERY (as KRunner and GNOME's search do)
  --open           Open the first conversation of the list
  --compose        Start a new message
  --inbox          Show the Inbox
  --settings       Open the settings
  --message ID     Open the message with this ID (as notifications do)
  --reply-all ID   Open the message with this ID and reply to all
  --update         Show the downloaded update of Katna, ready to install
  mailto:...       Write a new message as the link asks (Katna Mail is
                   the desktop's mail app when Settings > General says so)
  --background     Start the Katna service (sync, notifications, the tray
                   icon) without a window, as at login
  -h, --help       Show this help
  -V, --version    Show the version

Keys: Up/Down or j/k move through the list, Enter or o opens, u or
Escape closes, r replies, e archives, # deletes, s stars, x ticks, / or
Ctrl+F searches, ? lists every shortcut, Ctrl+Q quits. Settings, Keyboard
shortcuts changes them.
";

/// Katna Mail's translations, embedded by `build.rs`.
const TRANSLATIONS: katna_i18n::Sources = include!(concat!(env!("OUT_DIR"), "/translations.rs"));

fn main() -> ExitCode {
    // Grammar checking runs in a copy of the app, started by the app.
    let mut given = std::env::args().skip(1);
    if given.next().as_deref() == Some(grammar::HELPER_FLAG) {
        let language = given.next().unwrap_or_default();
        return grammar::run_helper(&language, &given.next().unwrap_or_default());
    }
    let mut data_dir: Option<PathBuf> = None;
    let mut open_first = false;
    let mut request = None;
    let mut args = std::env::args_os().skip(1);
    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("--data-dir") => match args.next() {
                Some(dir) => data_dir = Some(dir.into()),
                None => return usage_error(),
            },
            Some("--search") => match args.next().and_then(|q| q.into_string().ok()) {
                Some(query) => request = Some(instance::Request::Search(query)),
                None => return usage_error(),
            },
            Some("--open") => open_first = true,
            // At login: the Katna service only, without a window.
            Some(autostart::BACKGROUND_FLAG) => return autostart::start_service(),
            // Started by the Katna Mail it replaces after an update: waits
            // for that one to close, so this one becomes the app.
            Some(updater::AFTER_FLAG) => {
                match args.next().and_then(|pid| pid.to_str()?.parse().ok()) {
                    Some(pid) => updater::wait_for_exit(pid),
                    None => return usage_error(),
                }
            }
            Some(flag @ ("--compose" | "--inbox" | "--settings" | "--update")) => {
                request = instance::Request::from_flag(flag);
            }
            Some(flag @ ("--message" | "--reply-all")) => {
                match args.next().and_then(|id| id.to_str()?.parse().ok()) {
                    Some(id) => request = instance::Request::for_message(flag, id),
                    None => return usage_error(),
                }
            }
            // The desktop file's `%u`: a link to write to.
            Some(uri) if mailto::Mailto::parse(uri).is_some() => {
                request = Some(instance::Request::Mailto(uri.to_owned()));
            }
            Some("-h" | "--help") => {
                print!("{USAGE}");
                return ExitCode::SUCCESS;
            }
            Some("-V" | "--version") => {
                println!("katna-mail {}", whats_new::VERSION);
                return ExitCode::SUCCESS;
            }
            _ => return usage_error(),
        }
    }
    // A search wins over opening the first conversation.
    let open_first = open_first && !matches!(request, Some(instance::Request::Search(_)));
    // A private data directory gets a window of its own.
    let single = data_dir.is_none();
    let paths = match data_dir {
        Some(dir) => Paths::with_root(dir),
        None => match Paths::from_env() {
            Ok(paths) => paths,
            Err(err) => {
                eprintln!("katna-mail: {err}");
                return ExitCode::FAILURE;
            }
        },
    };
    katna_core::crash::install("katna-mail", &paths);
    if let Err(err) = katna_core::logging::init("warn") {
        eprintln!("katna-mail: {err}");
    }
    // The language, before any text is drawn (§13.10).
    katna_i18n::init(TRANSLATIONS, Some(paths.data_dir().join("i18n")));
    let general = Config::load(&paths.config_file())
        .unwrap_or_default()
        .general;
    if single && !general.start_at_login_set {
        start_at_login_by_default(&paths);
    }
    // Keeps Katna listed as a mail app in Windows' Default apps, also after
    // it moved.
    #[cfg(windows)]
    if single
        && let Ok(exe) = std::env::current_exe()
        && let Err(err) = katna_platform::mail_handler::register(&exe)
    {
        tracing::warn!(%err, "cannot register Katna Mail as a mail app");
    }
    format::set_clock(general.clock);
    katna_i18n::apply(&general.language);
    let (connection, sender, requests) = match instance::start(request, single) {
        instance::Started::HandedOff => return ExitCode::SUCCESS,
        instance::Started::First {
            connection,
            sender,
            requests,
        } => (connection, sender, requests),
    };

    gpui_platform::application()
        .with_assets(assets::Assets)
        .run(move |cx: &mut App| {
            // Before the window opens, which reads the menu bar's address.
            if let Some(connection) = &connection {
                serve_menu_bar(connection, sender, cx);
            }
            // Settings > Experimental > Look & Feel, before the first window.
            let config = Config::load(&paths.config_file()).unwrap_or_default();
            // Settings > Appearance > Scaling, before any length is made.
            katna_ui::scale::set_scale(f32::from(config.mail.scale) / 100.0);
            let look = window::look(&config);
            cx.set_global(look);
            let mut env = Environment::from_env();
            env.own_frame = look.own_frame;
            let font = ui_font(&env, cx);
            if let Some(font) = &font {
                cx.set_global(katna_ui::UiFont(font.clone()));
            }
            let mut options = window_options(
                &env,
                MAIL_APP_ID,
                "Katna Mail",
                size(desktop_px(1280.0), desktop_px(800.0)),
                cx,
            );
            // As it closed, while the Katna service runs.
            let placement = placement::MailPlacement::new(
                paths.mail_window_file(),
                env.clone(),
                connection.clone(),
            );
            let shown = placement.restore(&mut options, cx);
            let opened = cx.open_window(options, |window, cx| {
                cx.new(|cx| {
                    placement.follow(window, cx);
                    let mut view = window::MailWindow::new(env, paths, font, shown, window, cx);
                    if open_first {
                        view.open_first(window, cx);
                    }
                    view
                })
            });
            let handle = match opened {
                Ok(handle) => handle,
                Err(err) => {
                    eprintln!("katna-mail: cannot open a window: {err}");
                    cx.quit();
                    return;
                }
            };
            placement.save_on_quit(cx);
            // The window has bound the keys; show them in the menu bar.
            window::refresh_menu_bar(cx);
            cx.spawn(async move |cx| {
                // Keeps the bus name, the app interface and the menu bar
                // for as long as the app runs.
                let _connection = connection;
                while let Ok(request) = requests.recv().await {
                    let handled = handle.update(cx, |view, window, cx| {
                        view.handle_request(request, window, cx);
                    });
                    if handled.is_err() {
                        break;
                    }
                }
            })
            .detach();
            // Other windows (a message being written) close on their own;
            // the app ends with the mail window.
            let main = handle.window_id();
            cx.on_window_closed(move |cx, id| {
                if id == main {
                    cx.quit();
                }
            })
            .detach();
            cx.activate(true);
        });
    ExitCode::SUCCESS
}

/// Serves the menu bar for the KDE global menu and has GPUI's windows point
/// to it. A click becomes a [`instance::Request::Menu`] on `sender`.
fn serve_menu_bar(
    connection: &katna_dbus::zbus::Connection,
    sender: async_channel::Sender<instance::Request>,
    cx: &mut App,
) {
    let Some(service) = connection.unique_name().map(|name| name.to_string()) else {
        return;
    };
    let items = window::menu_bar(cx);
    let served = futures_lite::future::block_on(Menu::serve(
        connection,
        MAIL_MENU_BAR_PATH,
        items,
        move |action| {
            let _ = sender.try_send(instance::Request::Menu(action.to_owned()));
        },
    ));
    match served {
        Ok(menu) => {
            cx.set_global(window::MenuBar(menu));
            katna_ui::native::set_kde_appmenu(service, MAIL_MENU_BAR_PATH);
        }
        Err(err) => tracing::warn!(%err, "no menu bar for the global menu"),
    }
}

/// The desktop's UI font if it is installed, else a common one.
fn ui_font(env: &Environment, cx: &App) -> Option<SharedString> {
    let kde = env.desktop == Desktop::Kde;
    let config_home = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))?;
    let wanted = font::desktop_ui_font(kde, &config_home);
    let installed = cx.text_system().all_font_names();
    let family = font::pick_family(wanted.as_ref().map(|f| f.family.as_str()), &installed, kde);
    tracing::info!(?wanted, ?family, "UI font");
    family.map(SharedString::from)
}

/// Katna starts quietly at login unless someone turned that off: done
/// once, on the first run that has the setting, and remembered in the
/// config file so turning it off stays off. A config file that can't be
/// read is left alone.
fn start_at_login_by_default(paths: &Paths) {
    let file = paths.config_file();
    let Ok(mut config) = Config::load(&file) else {
        return;
    };
    if autostart::set_default() {
        config.general.start_at_login_set = true;
        if let Err(err) = config.save(&file) {
            tracing::warn!(%err, "cannot save the config");
        }
    }
}

fn usage_error() -> ExitCode {
    eprint!("{USAGE}");
    ExitCode::from(2)
}
