// SPDX-License-Identifier: GPL-3.0-or-later

//! The tray icon on Windows (`docs/ARCHITECTURE.md` §27.1): an icon in the
//! notification area with the same right-click menu and unread badge as
//! the StatusNotifierItem on Linux, and the same API.
//!
//! Windows sends the icon's clicks to a window on the thread that made the
//! icon, so the icon lives on a thread of its own that runs a message loop
//! (winit's) for as long as the process runs. [`Tray`] talks to it through
//! the loop's proxy. The global shortcuts ([`serve_shortcuts`]) live on
//! the same thread, whose loop also receives their presses.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use global_hotkey::hotkey::HotKey;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use tray_icon::menu::{self, IsMenuItem, MenuEvent, PredefinedMenuItem};
use tray_icon::{Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::WindowId;
use zbus::Connection;

use crate::dbusmenu::MenuItem;
use crate::icon::{self, Style};
use crate::shortcuts::{self, Keys, Shortcut};

/// What the tray calls its handler with, besides menu actions.
pub const ACTIVATE: &str = "activate";
/// A middle click.
pub const SECONDARY_ACTIVATE: &str = "secondary-activate";

/// The size the icon is drawn at; Windows scales it to the taskbar.
const SIZE: u32 = 32;

type Handler = Box<dyn Fn(&str, Option<String>) + Send + Sync>;

/// What the icon's thread is asked to do.
enum Command {
    Show {
        title: String,
        style: Style,
        menu: Vec<MenuItem>,
    },
    Style(Style),
    Unread {
        badge: Option<String>,
        status: String,
    },
    Menu(Vec<MenuItem>),
    Hide,
    /// Registers these global shortcuts, each with its action.
    Shortcuts(Vec<(HotKey, String)>),
}

/// The icon's thread, started with the first icon.
static LOOP: OnceLock<Option<EventLoopProxy<Command>>> = OnceLock::new();

/// Where clicks go: the handler of the icon that is up.
static HANDLER: Mutex<Option<Handler>> = Mutex::new(None);

/// Where global shortcuts go, and the action of each by its ID.
static SHORTCUTS: Mutex<Option<(shortcuts::Handler, HashMap<u32, String>)>> = Mutex::new(None);

/// A tray icon in the notification area. [`Tray::hide`] takes it down.
pub struct Tray {
    proxy: EventLoopProxy<Command>,
}

impl Tray {
    /// Shows the app's icon in `style` with `menu` on right click and
    /// `title` as the tooltip. Clicks call `handler`. `connection` and `id`
    /// are for the Linux tray.
    pub async fn show(
        _connection: &Connection,
        _id: &str,
        title: &str,
        style: Style,
        menu: Vec<MenuItem>,
        handler: impl Fn(&str, Option<String>) + Send + Sync + 'static,
    ) -> zbus::Result<Self> {
        let proxy = LOOP
            .get_or_init(start)
            .clone()
            .ok_or_else(|| zbus::Error::Failure("no tray thread".into()))?;
        *HANDLER.lock().unwrap() = Some(Box::new(handler));
        let tray = Self { proxy };
        tray.send(Command::Show {
            title: title.to_owned(),
            style,
            menu,
        })?;
        Ok(tray)
    }

    /// Shows `count` in a badge (none for 0) and `status` in the tooltip.
    pub async fn set_unread(&self, count: u64, status: &str) -> zbus::Result<()> {
        self.send(Command::Unread {
            badge: (count > 0).then(|| icon::badge_text(count)),
            status: status.to_owned(),
        })
    }

    /// Draws the icon in `style` from now on.
    pub async fn set_style(&self, style: Style) -> zbus::Result<()> {
        self.send(Command::Style(style))
    }

    /// Replaces the right-click menu.
    pub async fn set_menu(&self, items: Vec<MenuItem>) -> zbus::Result<()> {
        self.send(Command::Menu(items))
    }

    /// Takes the icon down.
    pub async fn hide(self) -> zbus::Result<()> {
        *HANDLER.lock().unwrap() = None;
        self.send(Command::Hide)
    }

    fn send(&self, command: Command) -> zbus::Result<()> {
        self.proxy
            .send_event(command)
            .map_err(|_| zbus::Error::Failure("the tray thread stopped".into()))
    }
}

/// Registers `shortcuts` with Windows and calls `handler` with the action
/// of each pressed; the same API as on Linux (`crate::shortcuts`), where
/// `connection`, `component` and `component_name` are for KDE. A key that
/// another program holds is left out. Waits for as long as the process
/// runs.
pub async fn serve_shortcuts(
    _connection: &Connection,
    _component: &str,
    _component_name: &str,
    shortcuts: Vec<Shortcut>,
    handler: shortcuts::Handler,
) -> zbus::Result<()> {
    let proxy = LOOP
        .get_or_init(start)
        .clone()
        .ok_or_else(|| zbus::Error::Failure("no tray thread".into()))?;
    let keys: Vec<(HotKey, String)> = shortcuts
        .into_iter()
        .filter_map(|s| Some((hot_key(s.keys)?, s.action)))
        .collect();
    let actions = keys.iter().map(|(key, action)| (key.id(), action.clone()));
    *SHORTCUTS.lock().unwrap() = Some((handler, actions.collect()));
    proxy
        .send_event(Command::Shortcuts(keys))
        .map_err(|_| zbus::Error::Failure("the tray thread stopped".into()))?;
    std::future::pending::<()>().await;
    Ok(())
}

/// `keys` as the hotkey crate has it.
fn hot_key(keys: Keys) -> Option<HotKey> {
    let mut text = String::new();
    for (on, name) in [
        (keys.meta, "super+"),
        (keys.ctrl, "control+"),
        (keys.alt, "alt+"),
        (keys.shift, "shift+"),
    ] {
        if on {
            text.push_str(name);
        }
    }
    text.push(keys.key.to_ascii_uppercase());
    text.parse().ok()
}

fn on_shortcut(event: GlobalHotKeyEvent) {
    if event.state() != HotKeyState::Pressed {
        return;
    }
    let shortcuts = SHORTCUTS.lock().unwrap();
    if let Some((handler, actions)) = shortcuts.as_ref()
        && let Some(action) = actions.get(&event.id())
    {
        handler(action);
    }
}

/// Starts the icon's thread and returns its loop's proxy.
fn start() -> Option<EventLoopProxy<Command>> {
    let (sender, receiver) = std::sync::mpsc::channel();
    let spawned = std::thread::Builder::new()
        .name("katna-tray".into())
        .spawn(move || {
            let event_loop = match EventLoop::<Command>::with_user_event()
                .with_any_thread(true)
                .build()
            {
                Ok(event_loop) => event_loop,
                Err(err) => {
                    tracing::warn!(%err, "no message loop for the tray icon");
                    let _ = sender.send(None);
                    return;
                }
            };
            event_loop.set_control_flow(ControlFlow::Wait);
            let _ = sender.send(Some(event_loop.create_proxy()));
            TrayIconEvent::set_event_handler(Some(on_icon));
            GlobalHotKeyEvent::set_event_handler(Some(on_shortcut));
            MenuEvent::set_event_handler(Some(|event: MenuEvent| click(event.id().as_ref())));
            let mut app = App::default();
            if let Err(err) = event_loop.run_app(&mut app) {
                tracing::warn!(%err, "the tray's message loop stopped");
            }
        });
    if let Err(err) = spawned {
        tracing::warn!(%err, "cannot start the tray thread");
        return None;
    }
    receiver.recv().ok().flatten()
}

fn on_icon(event: TrayIconEvent) {
    if let TrayIconEvent::Click {
        button,
        button_state: MouseButtonState::Up,
        ..
    } = event
    {
        match button {
            MouseButton::Left => click(ACTIVATE),
            MouseButton::Middle => click(SECONDARY_ACTIVATE),
            // Windows shows the menu itself.
            MouseButton::Right => {}
        }
    }
}

fn click(action: &str) {
    if let Some(handler) = HANDLER.lock().unwrap().as_ref() {
        handler(action, None);
    }
}

/// The icon's thread: holds the icon while it is up.
#[derive(Default)]
struct App {
    icon: Option<TrayIcon>,
    title: String,
    style: Style,
    badge: Option<String>,
    /// Holds the global shortcuts; made on this thread, whose loop gets
    /// their presses.
    hotkeys: Option<GlobalHotKeyManager>,
}

impl ApplicationHandler<Command> for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, _: WindowEvent) {}

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, command: Command) {
        match command {
            Command::Show { title, style, menu } => {
                self.title = title;
                self.style = style;
                self.badge = None;
                let built = TrayIconBuilder::new()
                    .with_icon(draw(style, None))
                    .with_tooltip(&self.title)
                    .with_menu(Box::new(native_menu(&menu)))
                    .with_menu_on_left_click(false)
                    .build();
                match built {
                    Ok(icon) => self.icon = Some(icon),
                    Err(err) => tracing::warn!(%err, "no tray icon"),
                }
            }
            Command::Style(style) => {
                self.style = style;
                if let Some(icon) = &self.icon {
                    let _ = icon.set_icon(Some(draw(style, self.badge.as_deref())));
                }
            }
            Command::Unread { badge, status } => {
                self.badge = badge;
                if let Some(icon) = &self.icon {
                    let _ = icon.set_icon(Some(draw(self.style, self.badge.as_deref())));
                    let _ = icon.set_tooltip(Some(format!("{}\n{status}", self.title)));
                }
            }
            Command::Menu(items) => {
                if let Some(icon) = &self.icon {
                    icon.set_menu(Some(Box::new(native_menu(&items))));
                }
            }
            Command::Hide => self.icon = None,
            Command::Shortcuts(keys) => {
                if self.hotkeys.is_none() {
                    match GlobalHotKeyManager::new() {
                        Ok(manager) => self.hotkeys = Some(manager),
                        Err(err) => tracing::warn!(%err, "no global shortcuts"),
                    }
                }
                if let Some(manager) = &self.hotkeys {
                    for (key, action) in keys {
                        if let Err(err) = manager.register(key) {
                            tracing::warn!(%err, action, "global shortcut taken");
                        }
                    }
                }
            }
        }
    }
}

/// The app icon in `style` with `badge`, as the tray wants it.
fn draw(style: Style, badge: Option<&str>) -> Icon {
    let rgba = argb_to_rgba(&icon::tray_icon_argb(SIZE, style, badge));
    Icon::from_rgba(rgba, SIZE, SIZE).expect("an icon of the right size")
}

/// ARGB32 in network byte order to RGBA.
fn argb_to_rgba(argb: &[u8]) -> Vec<u8> {
    argb.as_chunks::<4>()
        .0
        .iter()
        .flat_map(|&[a, r, g, b]| [r, g, b, a])
        .collect()
}

/// Windows marks the mnemonic with `&` where Katna's menus use `_`.
fn label(text: &str) -> String {
    text.replace('&', "&&").replacen('_', "&", 1)
}

fn native_menu(items: &[MenuItem]) -> menu::Menu {
    let menu = menu::Menu::new();
    for item in native_items(items) {
        let _ = menu.append(item.as_ref());
    }
    menu
}

fn native_items(items: &[MenuItem]) -> Vec<Box<dyn IsMenuItem>> {
    items
        .iter()
        .map(|item| -> Box<dyn IsMenuItem> {
            match item {
                MenuItem::Action {
                    label: text,
                    action,
                    enabled,
                    ..
                } => Box::new(menu::MenuItem::with_id(
                    action.as_str(),
                    label(text),
                    *enabled,
                    None,
                )),
                MenuItem::Submenu { label: text, items } => {
                    let submenu = menu::Submenu::new(label(text), true);
                    for item in native_items(items) {
                        let _ = submenu.append(item.as_ref());
                    }
                    Box::new(submenu)
                }
                MenuItem::Separator => Box::new(PredefinedMenuItem::separator()),
            }
        })
        .collect()
}
