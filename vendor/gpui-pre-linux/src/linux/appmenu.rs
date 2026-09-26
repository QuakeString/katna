//! KDE global menu support (added for Katna; not in upstream GPUI).
//!
//! An app exports its menu bar with `com.canonical.dbusmenu` and tells GPUI
//! where it is with [`set_kde_appmenu`]. Every normal window opened after
//! that carries the address: through `org_kde_kwin_appmenu` on Wayland, and
//! the `_KDE_NET_WM_APPMENU_*` properties on X11. Plasma's Global Menu
//! applet and window-title menu button then show the menu.

use std::sync::Mutex;

static ADDRESS: Mutex<Option<(String, String)>> = Mutex::new(None);

/// Names the D-Bus service and object path of the app's menu bar for the
/// windows opened from now on.
pub fn set_kde_appmenu(service: impl Into<String>, object_path: impl Into<String>) {
    *ADDRESS.lock().unwrap() = Some((service.into(), object_path.into()));
}

/// The menu bar's service and object path, if the app set one.
pub(crate) fn kde_appmenu() -> Option<(String, String)> {
    ADDRESS.lock().unwrap().clone()
}
