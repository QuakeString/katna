// SPDX-License-Identifier: GPL-3.0-or-later

//! Desktop detection and the decoration policy (`docs/ARCHITECTURE.md` §13.1).
//!
//! This module has no GPUI types, so the policy is unit-tested without a
//! display server.

/// Environment variable that overrides the decoration policy:
/// `auto` (default), `server` or `client`. It wins over
/// [`Environment::own_frame`].
pub const DECORATIONS_ENV: &str = "KATNA_DECORATIONS";

/// The desktop environment we are running in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Desktop {
    /// KDE Plasma (KWin).
    Kde,
    /// GNOME (Mutter), including distribution sessions such as `ubuntu:GNOME`.
    Gnome,
    /// Anything else, with the first `XDG_CURRENT_DESKTOP` entry (may be empty).
    Other(String),
}

/// The display server protocol of the session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Session {
    Wayland,
    X11,
}

/// Who draws the window frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecorationMode {
    /// The compositor draws the title bar, buttons and shadow.
    Server,
    /// Katna draws a header bar, shadow and resize edges itself.
    Client,
}

/// A user override of the decoration policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DecorationOverride {
    #[default]
    Auto,
    Server,
    Client,
}

impl DecorationOverride {
    /// Parses the value of [`DECORATIONS_ENV`]. Unknown values mean `Auto`.
    pub fn parse(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "server" | "ssd" => Self::Server,
            "client" | "csd" => Self::Client,
            _ => Self::Auto,
        }
    }
}

/// Which look the client-side frame and the toolbar use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    BreezeLike,
    AdwaitaLike,
}

/// Everything the chrome needs to know about the desktop, decided once at
/// startup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Environment {
    pub desktop: Desktop,
    pub session: Session,
    pub decoration_override: DecorationOverride,
    /// The user asked for Katna's own frame (title bar, buttons, rounded
    /// corners, shadow) where the desktop would draw its own.
    pub own_frame: bool,
}

impl Environment {
    /// Reads `XDG_CURRENT_DESKTOP`, `XDG_SESSION_TYPE`, `WAYLAND_DISPLAY` and
    /// [`DECORATIONS_ENV`] from the process environment.
    pub fn from_env() -> Self {
        Self::from_vars(|name| std::env::var(name).ok())
    }

    /// Like [`Environment::from_env`], with a custom variable lookup (for tests).
    pub fn from_vars(var: impl Fn(&str) -> Option<String>) -> Self {
        let desktop = detect_desktop(var("XDG_CURRENT_DESKTOP").as_deref().unwrap_or(""));
        let session = match var("XDG_SESSION_TYPE").as_deref() {
            Some("wayland") => Session::Wayland,
            Some("x11") => Session::X11,
            // Not set (e.g. started from a nested compositor or a script):
            // GPUI prefers Wayland when WAYLAND_DISPLAY is set.
            _ if var("WAYLAND_DISPLAY").is_some_and(|v| !v.is_empty()) => Session::Wayland,
            _ => Session::X11,
        };
        let decoration_override = var(DECORATIONS_ENV)
            .map(|v| DecorationOverride::parse(&v))
            .unwrap_or_default();
        Self {
            desktop,
            session,
            decoration_override,
            own_frame: false,
        }
    }

    /// The decoration mode to request from the compositor.
    ///
    /// The compositor has the last word: on Wayland without
    /// `xdg-decoration` (Mutter, Weston) a request for server-side
    /// decorations falls back to client-side ones. The frame therefore
    /// renders from what was negotiated, never from this value.
    pub fn requested_decorations(&self) -> DecorationMode {
        match self.decoration_override {
            DecorationOverride::Server => return DecorationMode::Server,
            DecorationOverride::Client => return DecorationMode::Client,
            DecorationOverride::Auto => {}
        }
        if self.own_frame {
            return DecorationMode::Client;
        }
        self.native_decorations()
    }

    /// The desktop's own choice: who draws the frame when the user has not
    /// asked for Katna's.
    pub fn native_decorations(&self) -> DecorationMode {
        match (self.session, &self.desktop) {
            // CSD shadows need a compositor; every X11 window manager decorates.
            (Session::X11, _) => DecorationMode::Server,
            // Mutter does not implement xdg-decoration: without CSD the window
            // has no title bar at all.
            (Session::Wayland, Desktop::Gnome) => DecorationMode::Client,
            // KWin draws the real Breeze (or user-chosen) decoration. Other
            // compositors: SSD if offered, else GPUI falls back to CSD.
            (Session::Wayland, Desktop::Kde | Desktop::Other(_)) => DecorationMode::Server,
        }
    }

    /// The look for our own chrome (toolbar, and the frame when we draw it).
    pub fn preset(&self) -> Preset {
        match self.desktop {
            Desktop::Kde => Preset::BreezeLike,
            Desktop::Gnome | Desktop::Other(_) => Preset::AdwaitaLike,
        }
    }

    /// Whether a client-side frame should be drawn in full (shadow, rounded
    /// corners) or minimal (1 px border, no shadow).
    ///
    /// GNOME and KDE composite and expect the full frame. Elsewhere (tiling
    /// compositors that declined SSD) the frame is kept minimal.
    pub fn full_client_frame(&self) -> bool {
        matches!(self.desktop, Desktop::Gnome | Desktop::Kde)
    }
}

/// Parses `XDG_CURRENT_DESKTOP`, a colon-separated list such as `ubuntu:GNOME`.
pub fn detect_desktop(xdg_current_desktop: &str) -> Desktop {
    let entries: Vec<&str> = xdg_current_desktop
        .split(':')
        .map(str::trim)
        .filter(|e| !e.is_empty())
        .collect();
    let has = |names: &[&str]| {
        entries
            .iter()
            .any(|e| names.iter().any(|n| e.eq_ignore_ascii_case(n)))
    };
    if has(&["KDE"]) {
        Desktop::Kde
    } else if has(&["GNOME", "GNOME-Classic", "GNOME-Flashback"]) {
        Desktop::Gnome
    } else {
        Desktop::Other(entries.first().copied().unwrap_or("").to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env(vars: &[(&str, &str)]) -> Environment {
        Environment::from_vars(|name| {
            vars.iter()
                .find(|(k, _)| *k == name)
                .map(|(_, v)| (*v).to_owned())
        })
    }

    #[test]
    fn detects_desktops() {
        assert_eq!(detect_desktop("KDE"), Desktop::Kde);
        assert_eq!(detect_desktop("GNOME"), Desktop::Gnome);
        assert_eq!(detect_desktop("ubuntu:GNOME"), Desktop::Gnome);
        assert_eq!(detect_desktop("GNOME-Classic:GNOME"), Desktop::Gnome);
        assert_eq!(detect_desktop("sway"), Desktop::Other("sway".into()));
        assert_eq!(detect_desktop(""), Desktop::Other(String::new()));
    }

    #[test]
    fn kde_wayland_uses_server_side_breeze() {
        let e = env(&[
            ("XDG_CURRENT_DESKTOP", "KDE"),
            ("XDG_SESSION_TYPE", "wayland"),
        ]);
        assert_eq!(e.requested_decorations(), DecorationMode::Server);
        assert_eq!(e.preset(), Preset::BreezeLike);
    }

    #[test]
    fn gnome_wayland_uses_client_side_adwaita() {
        let e = env(&[
            ("XDG_CURRENT_DESKTOP", "ubuntu:GNOME"),
            ("XDG_SESSION_TYPE", "wayland"),
        ]);
        assert_eq!(e.requested_decorations(), DecorationMode::Client);
        assert_eq!(e.preset(), Preset::AdwaitaLike);
        assert!(e.full_client_frame());
    }

    #[test]
    fn x11_always_uses_server_side() {
        let e = env(&[
            ("XDG_CURRENT_DESKTOP", "GNOME"),
            ("XDG_SESSION_TYPE", "x11"),
        ]);
        assert_eq!(e.requested_decorations(), DecorationMode::Server);
    }

    #[test]
    fn other_wayland_compositors_ask_for_ssd_and_draw_minimal_csd() {
        let e = env(&[
            ("XDG_CURRENT_DESKTOP", "Hyprland"),
            ("WAYLAND_DISPLAY", "wayland-1"),
        ]);
        assert_eq!(e.session, Session::Wayland);
        assert_eq!(e.requested_decorations(), DecorationMode::Server);
        assert!(!e.full_client_frame());
    }

    #[test]
    fn own_frame_asks_for_client_side() {
        for session in ["wayland", "x11"] {
            let mut e = env(&[
                ("XDG_CURRENT_DESKTOP", "KDE"),
                ("XDG_SESSION_TYPE", session),
            ]);
            e.own_frame = true;
            assert_eq!(e.requested_decorations(), DecorationMode::Client);
            assert_eq!(e.native_decorations(), DecorationMode::Server);
            assert_eq!(e.preset(), Preset::BreezeLike);
        }
        // The variable still wins.
        let mut e = env(&[
            ("XDG_CURRENT_DESKTOP", "KDE"),
            ("XDG_SESSION_TYPE", "wayland"),
            (DECORATIONS_ENV, "server"),
        ]);
        e.own_frame = true;
        assert_eq!(e.requested_decorations(), DecorationMode::Server);
    }

    #[test]
    fn override_wins() {
        let e = env(&[
            ("XDG_CURRENT_DESKTOP", "KDE"),
            ("XDG_SESSION_TYPE", "wayland"),
            (DECORATIONS_ENV, "client"),
        ]);
        assert_eq!(e.requested_decorations(), DecorationMode::Client);
        let e = env(&[
            ("XDG_CURRENT_DESKTOP", "GNOME"),
            ("XDG_SESSION_TYPE", "wayland"),
            (DECORATIONS_ENV, "SSD"),
        ]);
        assert_eq!(e.requested_decorations(), DecorationMode::Server);
        assert_eq!(
            DecorationOverride::parse("whatever"),
            DecorationOverride::Auto
        );
    }
}
