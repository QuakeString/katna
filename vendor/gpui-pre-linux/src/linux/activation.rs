//! Raising a window with a token from another app (added for Katna; not in
//! upstream GPUI).
//!
//! On Wayland a window may only take focus with an `xdg_activation_v1`
//! token the compositor handed out for a user's click. When the click was
//! in another app (a tray icon, a notification), that app passes its token
//! along, for example in `org.freedesktop.Application`'s `platform_data`.
//! [`set_activation_token`] keeps it for the next `Window::activate`.
//! Without one GPUI asks for a token of its own, which KWin and Mutter
//! refuse, so the window only asks for attention.

use std::sync::Mutex;

static TOKEN: Mutex<Option<String>> = Mutex::new(None);

/// Keeps `token` for the next window activation.
pub fn set_activation_token(token: impl Into<String>) {
    *TOKEN.lock().unwrap() = Some(token.into());
}

/// The token for this activation, if an app passed one.
pub(crate) fn take_activation_token() -> Option<String> {
    TOKEN.lock().unwrap().take()
}
