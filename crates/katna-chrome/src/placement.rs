// SPDX-License-Identifier: GPL-3.0-or-later

//! Opening a window where it was: its size, maximized state and place
//! (`docs/ARCHITECTURE.md` §13.1). Size and maximized state come back
//! through GPUI's window bounds; the place through the compositor's session
//! on Wayland and an exact position on X11 (`katna_ui::native::restore_placement`).

use gpui::{App, Bounds, Pixels, Size, Window, WindowBounds, WindowOptions, point, size};
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
    katna_ui::native::restore_placement(katna_ui::native::Placement {
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
    // A size saved on a bigger screen, or from before Windows' title bar
    // was left room for, shrinks to fit.
    if let Some(area) = windows_work_area(cx) {
        bounds = Bounds::centered_at(area.center(), fit(surface, area.size).0);
    }
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

/// Where a new window with a `surface` this big opens, centred on the main
/// display. On Windows a window that would not fit with its title bar opens
/// maximized, so its buttons are never off the screen; elsewhere the
/// desktop fits it.
pub(crate) fn fitted(surface: Size<Pixels>, cx: &App) -> WindowBounds {
    let Some(area) = windows_work_area(cx) else {
        return WindowBounds::Windowed(Bounds::centered(None, surface, cx));
    };
    let (size, maximize) = fit(surface, area.size);
    let bounds = Bounds::centered_at(area.center(), size);
    if maximize {
        WindowBounds::Maximized(bounds)
    } else {
        WindowBounds::Windowed(bounds)
    }
}

/// The main display's area for windows, without the taskbar, on Windows.
fn windows_work_area(cx: &App) -> Option<Bounds<Pixels>> {
    if !cfg!(windows) {
        return None;
    }
    cx.primary_display().map(|display| display.visible_bounds())
}

/// `wanted` if it fits in `work_area` with Windows' title bar and borders
/// around it; otherwise a size that does, and whether to open maximized.
fn fit(wanted: Size<Pixels>, work_area: Size<Pixels>) -> (Size<Pixels>, bool) {
    // Room Windows adds around the bounds GPUI opens a window at, which
    // are its inside: the borders, and the title bar with them.
    let room = size(
        (work_area.width - desktop_px(16.0)).max(MIN_WINDOW_SIZE.width),
        (work_area.height - desktop_px(48.0)).max(MIN_WINDOW_SIZE.height),
    );
    if wanted.width <= room.width && wanted.height <= room.height {
        return (wanted, false);
    }
    let restored = size(
        (room.width * 0.9).max(MIN_WINDOW_SIZE.width),
        (room.height * 0.9).max(MIN_WINDOW_SIZE.height),
    );
    (restored, true)
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
            session: katna_ui::native::placement_session(),
            service: run,
            view: Default::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use katna_ui::scale::desktop_px;

    use super::*;

    fn at(width: f32, height: f32) -> Size<Pixels> {
        size(desktop_px(width), desktop_px(height))
    }

    #[test]
    fn a_window_that_fits_keeps_its_size() {
        assert_eq!(
            fit(at(1280.0, 800.0), at(1920.0, 1040.0)),
            (at(1280.0, 800.0), false)
        );
    }

    #[test]
    fn a_window_too_big_for_the_screen_opens_maximized() {
        // 1280 × 800 with a 48 px taskbar: 752 px high, less the title bar.
        let (restored, maximize) = fit(at(1280.0, 800.0), at(1280.0, 752.0));
        assert!(maximize);
        assert!(restored.width <= desktop_px(1264.0) && restored.height <= desktop_px(704.0));
    }
}
