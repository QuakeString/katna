// SPDX-License-Identifier: GPL-3.0-or-later

//! The mail window opens as it closed, with the same size, place and
//! maximized state, for as long as the Katna service keeps running. Once
//! the service quits (the tray's Quit, or logging out), the next start
//! opens the window at its default size and place (`docs/ARCHITECTURE.md`
//! §13.1).

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use futures_lite::future;
use gpui::{App, Window, WindowOptions};
use katna_chrome::{Environment, Placement};
use katna_core::ids::DAEMON_BUS_NAME;
use katna_core::window::WindowState;
use katna_dbus::zbus::{self, Connection};

/// The window's name in the compositor's session.
const SESSION_NAME: &str = "mail";

/// Opens the mail window as it last closed and saves how it was when the
/// app quits.
pub struct MailPlacement {
    file: PathBuf,
    env: Environment,
    bus: Option<Connection>,
    placement: Rc<RefCell<Placement>>,
}

impl MailPlacement {
    pub fn new(file: PathBuf, env: Environment, bus: Option<Connection>) -> Self {
        Self {
            file,
            env,
            bus,
            placement: Rc::default(),
        }
    }

    /// Opens `options` as the window last closed in this run of the
    /// service.
    pub fn restore(&self, options: &mut WindowOptions, cx: &App) {
        let saved = WindowState::load(&self.file);
        let run = service_run(self.bus.as_ref());
        katna_chrome::restore_window(
            options,
            &self.env,
            saved.as_ref(),
            run.as_deref(),
            SESSION_NAME,
            cx,
        );
    }

    /// Follows the window from now on. Call from inside its first view.
    pub fn follow<V: 'static>(&self, window: &mut Window, cx: &mut gpui::Context<V>) {
        self.placement.borrow_mut().update(window);
        let placement = self.placement.clone();
        cx.observe_window_bounds(window, move |_, window, _| {
            placement.borrow_mut().update(window);
        })
        .detach();
    }

    /// Saves how the window was when the app quits, however it quits.
    pub fn save_on_quit(self, cx: &App) {
        cx.on_app_quit(move |_| {
            let state = self
                .placement
                .borrow()
                .state(&self.env, service_run(self.bus.as_ref()));
            if let Some(state) = state
                && let Err(err) = state.save(&self.file)
            {
                tracing::warn!(%err, "cannot save the window's size and place");
            }
            async {}
        })
        .detach();
    }
}

/// The current run of the Katna service, from the process that owns its
/// bus name; `None` when it is not running.
fn service_run(bus: Option<&Connection>) -> Option<String> {
    let bus = bus?;
    let pid = future::block_on(async {
        let dbus = zbus::fdo::DBusProxy::new(bus).await.ok()?;
        let name = zbus::names::BusName::try_from(DAEMON_BUS_NAME).ok()?;
        dbus.get_connection_unix_process_id(name).await.ok()
    })?;
    katna_core::window::service_run(pid)
}
