// SPDX-License-Identifier: GPL-3.0-or-later

//! Katna Mail. See `docs/ARCHITECTURE.md` §13.
//!
//! This first version reads the local store read-only: folders, message
//! list, reading pane (plain text) and search. Accounts, sync and sending
//! come with `katna-daemon`.

mod assets;
mod daemon;
mod data;
mod format;
mod instance;
mod outgoing;
mod sidebar;
mod theme;
mod widgets;
mod window;

use std::path::PathBuf;
use std::process::ExitCode;

use gpui::{App, AppContext, SharedString, px, size};
use katna_chrome::{Desktop, Environment, window_options};
use katna_core::Paths;
use katna_core::ids::{MAIL_APP_ID, MAIL_MENU_BAR_PATH};
use katna_platform::dbusmenu::Menu;
use katna_platform::font;

const USAGE: &str = "\
Usage: katna-mail [--data-dir DIR] [--search QUERY] [--compose | --inbox | --settings]

When Katna Mail is already running, it comes to the front and does what
the options ask; a second window does not open.

Options:
  --data-dir DIR   Use DIR/data, DIR/config and DIR/cache instead of the
                   XDG directories (the same layout as katna-search-cli)
  --search QUERY   Start with QUERY in the search box
  --open           Open the first conversation of the list
  --compose        Start a new message
  --inbox          Show the Inbox
  --settings       Open the settings
  -h, --help       Show this help
  -V, --version    Show the version

Keys: Up/Down or j/k move through the list, Enter or o opens, u or
Escape closes, e archives, # deletes, s stars, x ticks, Shift+I and
Shift+U mark read and unread, / or Ctrl+F searches, Ctrl+, opens quick
settings, F5 reloads, Ctrl+Q quits.
";

fn main() -> ExitCode {
    let mut data_dir: Option<PathBuf> = None;
    let mut search: Option<String> = None;
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
                Some(query) => search = Some(query),
                None => return usage_error(),
            },
            Some("--open") => open_first = true,
            Some(flag @ ("--compose" | "--inbox" | "--settings")) => {
                request = instance::Request::from_flag(flag);
            }
            Some("-h" | "--help") => {
                print!("{USAGE}");
                return ExitCode::SUCCESS;
            }
            Some("-V" | "--version") => {
                println!("katna-mail {}", env!("CARGO_PKG_VERSION"));
                return ExitCode::SUCCESS;
            }
            _ => return usage_error(),
        }
    }
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
    if let Err(err) = katna_core::logging::init("warn") {
        eprintln!("katna-mail: {err}");
    }
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
            window::bind_keys(cx);
            if let Some(connection) = &connection {
                serve_menu_bar(connection, sender, cx);
            }
            let env = Environment::from_env();
            let font = ui_font(&env, cx);
            if let Some(font) = &font {
                cx.set_global(katna_ui::UiFont(font.clone()));
            }
            let options = window_options(
                &env,
                MAIL_APP_ID,
                "Katna Mail",
                size(px(1280.0), px(800.0)),
                cx,
            );
            let opened = cx.open_window(options, |window, cx| {
                cx.new(|cx| {
                    let mut view = window::MailWindow::new(env, paths, font, window, cx);
                    if let Some(query) = search {
                        view.search_for(query, window, cx);
                    } else if open_first {
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
            cx.on_window_closed(|cx, _| cx.quit()).detach();
            cx.activate(true);
        });
    ExitCode::SUCCESS
}

/// Serves the menu bar for the KDE global menu and has GPUI's windows point
/// to it. A click becomes a [`instance::Request::Menu`] on `sender`.
fn serve_menu_bar(
    connection: &katna_dbus::zbus::Connection,
    sender: async_channel::Sender<instance::Request>,
    cx: &App,
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
        Ok(_) => gpui_linux::set_kde_appmenu(service, MAIL_MENU_BAR_PATH),
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

fn usage_error() -> ExitCode {
    eprint!("{USAGE}");
    ExitCode::from(2)
}
