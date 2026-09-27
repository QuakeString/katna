// SPDX-License-Identifier: GPL-3.0-or-later

//! Opening a window where it was: its size, maximized state and place
//! (`docs/ARCHITECTURE.md` §13.1). Size and maximized state come back
//! through GPUI's window bounds; the place through the compositor's session
//! on Wayland and an exact position on X11 (`gpui_linux::restore_placement`).

use gpui::{App, Bounds, Pixels, Window, WindowBounds, WindowOptions, point, size};
use katna_core::window::WindowState;
use katna_ui::px;
use katna_ui::scale::desktop_px;

use crate::desktop::{Environment, Session};
use crate::frame::{MIN_WINDOW_SIZE, surface_margin};

/// Opens the window of `options` as `state` left it when that belongs to
/// the service's `run`; otherwise at its default size and place. `name`
/// names the window in the compositor's session, the same every run.
pub fn restore_window(
    options: &mut WindowOptions,
    env: &Environment,
    state: Option<&WindowState>,
    run: Option<&str>,
    name: &str,
    cx: &App,
) {
    let restored = state.filter(|state| state.same_run(run));
    gpui_linux::restore_placement(gpui_linux::Placement {
        session: state.and_then(|state| state.session.clone()),
        name: name.to_owned(),
        restore: restored.is_some(),
    });
    let Some(state) = restored else {
        return;
    };
    // The surface holds the shadow margin of client-side decorations on
    // Wayland. KWin on X11 adds it itself once the window says how wide it
    // is (`_GTK_FRAME_EXTENTS`), keeping the frame at the size and place
    // asked for.
    let margin = match env.session {
        Session::Wayland => surface_margin(env),
        Session::X11 => 0.0,
    };
    let frame = size(
        desktop_px(state.width).max(MIN_WINDOW_SIZE.width),
        desktop_px(state.height).max(MIN_WINDOW_SIZE.height),
    );
    let surface = size(
        frame.width + px(2.0 * margin),
        frame.height + px(2.0 * margin),
    );
    let mut bounds = Bounds::centered(None, surface, cx);
    if let Some((x, y)) = state.position
        && on_a_display(point(desktop_px(x), desktop_px(y)), cx)
    {
        bounds.origin = point(desktop_px(x) - px(margin), desktop_px(y) - px(margin));
    }
    options.window_bounds = Some(if state.maximized {
        WindowBounds::Maximized(bounds)
    } else {
        WindowBounds::Windowed(bounds)
    });
}

/// Whether the top of a window whose frame starts at `origin` would be on
/// one of the displays, so it can be grabbed.
fn on_a_display(origin: gpui::Point<Pixels>, cx: &App) -> bool {
    let grip = point(origin.x + desktop_px(48.0), origin.y + desktop_px(16.0));
    cx.displays()
        .iter()
        .any(|display| display.bounds().contains(&grip))
}

/// Follows a window's size, place and maximized state while it is open,
/// so they can be saved when it closes. Call [`Placement::update`] when it
/// opens and whenever its bounds change.
#[derive(Debug, Default, Clone)]
pub struct Placement {
    /// The visible frame the last time the window was neither maximized
    /// nor full screen.
    windowed: Option<Bounds<Pixels>>,
    maximized: bool,
}

impl Placement {
    pub fn update(&mut self, window: &Window) {
        match window.inner_window_bounds() {
            WindowBounds::Windowed(bounds) => {
                self.windowed = Some(bounds);
                self.maximized = false;
            }
            WindowBounds::Maximized(_) => self.maximized = true,
            WindowBounds::Fullscreen(_) => {}
        }
    }

    /// The state to save, in the service's `run`; `None` before the window
    /// had a size.
    pub fn state(&self, env: &Environment, run: Option<String>) -> Option<WindowState> {
        let windowed = self.windowed?;
        // Window sizes and places are in the desktop's pixels, whatever
        // the interface scale, so they are read as they are.
        // Wayland does not tell a window where it is; the compositor's
        // session remembers that.
        let position = (env.session == Session::X11)
            .then(|| (f32::from(windowed.origin.x), f32::from(windowed.origin.y)));
        Some(WindowState {
            width: f32::from(windowed.size.width),
            height: f32::from(windowed.size.height),
            position,
            maximized: self.maximized,
            session: gpui_linux::placement_session(),
            service: run,
        })
    }
}
