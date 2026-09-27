//! Opening the main window where it was (added for Katna; not in upstream
//! GPUI).
//!
//! An app that remembers its window calls [`restore_placement`] before it
//! opens it, and gives its size and maximized state through the window's
//! bounds as usual. The next normal window then:
//!
//! - Wayland: joins an `xdg_session_v1` (xdg-session-management, which KWin
//!   has from Plasma 6.7) under the given name. With `restore`, the
//!   compositor puts it back where the session last had it; without, the
//!   old session is removed and a new one starts. [`placement_session`] is
//!   the session's id, to keep for next time. Wayland does not let a client
//!   place its own window, so this is the only way back to the same place.
//! - X11: with `restore`, opens exactly at the origin of its bounds (a
//!   user-specified position with static gravity, which window managers
//!   honour).

use std::sync::Mutex;

/// How the next normal window is placed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Placement {
    /// The Wayland session the window was in, if any.
    pub session: Option<String>,
    /// The window's name in the session, the same every run.
    pub name: String,
    /// Put the window back where it was: `false` starts afresh.
    pub restore: bool,
}

static REQUEST: Mutex<Option<Placement>> = Mutex::new(None);
static SESSION: Mutex<Option<String>> = Mutex::new(None);

/// Places the next normal window as `placement` says.
pub fn restore_placement(placement: Placement) {
    *REQUEST.lock().unwrap() = Some(placement);
}

/// The id of the Wayland session the window is in, once the compositor has
/// one; `None` elsewhere.
pub fn placement_session() -> Option<String> {
    SESSION.lock().unwrap().clone()
}

/// The request, for the first normal window that opens after it.
#[allow(dead_code)]
pub(crate) fn take_placement() -> Option<Placement> {
    REQUEST.lock().unwrap().take()
}

#[allow(dead_code)]
pub(crate) fn set_placement_session(id: String) {
    *SESSION.lock().unwrap() = Some(id);
}
