// SPDX-License-Identifier: GPL-3.0-or-later

//! Following the desktop's color scheme and accent color (§13.2): read at
//! startup, again whenever the portal says a setting changed (accent
//! color, color scheme, any `kdeglobals` group on KDE), and whenever the
//! files they come from change (theme tools write `gtk.css` without
//! telling anyone). On KDE it also follows the Blur effect's strength in
//! `kwinrc`, which the frosted menus and dialogs take on, and the
//! desktop's animation speed (`katna_platform::motion`), which motion
//! follows unless Settings > Appearance sets Katna's own.

use std::path::{Path, PathBuf};
use std::time::Duration;

use futures_lite::StreamExt;
use gpui::{App, Context, Task};
use katna_chrome::Desktop;
use katna_core::config::{MailView, ReduceMotion};
use katna_dbus::zbus::Connection;
use katna_platform::blur;
use katna_platform::colors::{self, DesktopKind, DesktopScheme, SystemColors};
use katna_platform::motion::{self, DesktopMotion};

use super::MailWindow;
use crate::user_schemes;

/// How often the color files are checked for changes.
const FILE_POLL: Duration = Duration::from_secs(2);
/// Desktops change several settings at once; read them once they settle.
const SETTLE: Duration = Duration::from_millis(150);

pub(super) struct DesktopColors {
    kind: DesktopKind,
    config_home: Option<PathBuf>,
    /// The portal's accent color, kept for reading again when a file
    /// changes.
    portal_accent: Option<u32>,
    /// What the desktop gives.
    desktop: SystemColors,
    /// The schemes people made (`user_schemes`).
    pub(super) user: Vec<DesktopScheme>,
    /// The desktop's colors with the schemes people made listed too, as
    /// `Theme::pick` looks them up.
    pub(super) colors: SystemColors,
    /// KDE's blur strength, 1 to 15; `None` off KDE or with its Blur
    /// effect off.
    pub(super) kde_blur: Option<u8>,
    /// The desktop's animation speed.
    pub(super) motion: DesktopMotion,
    _watch: Vec<Task<()>>,
}

impl DesktopColors {
    /// Reads the colors the files and GSettings give, so the first frame
    /// already has them; the portal's accent color follows.
    pub(super) fn new(desktop: &Desktop, config_dir: &Path) -> Self {
        let kind = match desktop {
            _ if cfg!(windows) => DesktopKind::Windows,
            Desktop::Kde => DesktopKind::Kde,
            Desktop::Gnome => DesktopKind::Gnome,
            Desktop::Other(_) => DesktopKind::Other,
        };
        let config_home = colors::config_home();
        let colors = read(kind, config_home.as_ref(), None);
        tracing::info!(?kind, ?colors, "desktop colors");
        let user = user_schemes::load_all(&user_schemes::dir(config_dir));
        let kde_blur = read_kde_blur(kind, config_home.as_deref());
        let motion = read_motion(kind, config_home.as_deref());
        tracing::info!(?motion, "desktop motion");
        let mut this = Self {
            kind,
            config_home,
            portal_accent: None,
            desktop: colors,
            user,
            colors: SystemColors::default(),
            kde_blur,
            motion,
            _watch: Vec::new(),
        };
        this.merge();
        this
    }

    fn merge(&mut self) {
        let mut colors = self.desktop.clone();
        colors.schemes.extend(self.user.iter().cloned());
        self.colors = colors;
    }

    /// Lists `user` as the schemes people made.
    pub(super) fn set_user(&mut self, user: Vec<DesktopScheme>) {
        self.user = user;
        self.merge();
    }
}

fn read(
    kind: DesktopKind,
    config_home: Option<&PathBuf>,
    portal_accent: Option<u32>,
) -> SystemColors {
    match config_home {
        _ if kind == DesktopKind::Windows => colors::windows::read(),
        Some(home) => colors::read(kind, home, portal_accent),
        None => SystemColors::accent_only(portal_accent),
    }
}

fn read_kde_blur(kind: DesktopKind, config_home: Option<&Path>) -> Option<u8> {
    match config_home {
        Some(home) if kind == DesktopKind::Kde => blur::read_kde(home),
        _ => None,
    }
}

fn read_motion(kind: DesktopKind, config_home: Option<&Path>) -> DesktopMotion {
    motion::read(kind == DesktopKind::Kde, config_home)
}

/// Sets how fast motion runs and whether it runs at all, from `view`'s
/// settings and `desktop`'s.
pub(super) fn apply_motion(view: &MailView, desktop: DesktopMotion, cx: &mut App) {
    let speed = view.animation_speed.unwrap_or(desktop.duration_factor);
    if katna_ui::motion::speed() != speed {
        katna_ui::motion::set_speed(speed);
        cx.refresh_windows();
    }
    cx.set_reduce_motion(match view.reduce_motion {
        ReduceMotion::Desktop => desktop.off(),
        ReduceMotion::On => true,
        ReduceMotion::Off => false,
    });
}

impl MailWindow {
    /// Starts following the desktop's colors.
    pub(super) fn watch_colors(&mut self, cx: &mut Context<Self>) {
        let kind = self.desktop_colors.kind;
        let home = self.desktop_colors.config_home.clone();
        let home_for_blur = home.clone();
        let portal = cx.spawn({
            let home = home.clone();
            async move |this, cx| {
                let connection = match cx.background_executor().spawn(Connection::session()).await {
                    Ok(connection) => connection,
                    Err(err) => {
                        tracing::info!("no desktop portal for colors: {err}");
                        return;
                    }
                };
                let mut changes = match colors::setting_changes(&connection).await {
                    Ok(changes) => Some(Box::pin(changes)),
                    Err(err) => {
                        tracing::info!("not following desktop settings: {err}");
                        None
                    }
                };
                loop {
                    let (connection, home) = (connection.clone(), home.clone());
                    let (accent, colors, motion) = cx
                        .background_executor()
                        .spawn(async move {
                            let accent = colors::portal_accent(&connection).await;
                            let motion = read_motion(kind, home.as_deref());
                            (accent, read(kind, home.as_ref(), accent), motion)
                        })
                        .await;
                    let updated = this.update(cx, |this, cx| {
                        this.desktop_colors.portal_accent = accent;
                        this.set_desktop_colors(colors, cx);
                        this.set_desktop_motion(motion, cx);
                    });
                    if updated.is_err() {
                        return;
                    }
                    let Some(changes) = changes.as_mut() else {
                        return;
                    };
                    if changes.next().await.is_none() {
                        return;
                    }
                    cx.background_executor().timer(SETTLE).await;
                }
            }
        });
        let files = cx.spawn(async move |this, cx| {
            let Some(home) = home else {
                return;
            };
            let files = colors::watched_files(kind, &home);
            if files.is_empty() {
                return;
            }
            let mut last = colors::stamps(&files);
            loop {
                cx.background_executor().timer(FILE_POLL).await;
                let now = colors::stamps(&files);
                if now == last {
                    continue;
                }
                last = now;
                cx.background_executor().timer(SETTLE).await;
                let Ok(accent) = this.update(cx, |this, _| this.desktop_colors.portal_accent)
                else {
                    return;
                };
                let home = home.clone();
                let (colors, motion) = cx
                    .background_executor()
                    .spawn(async move {
                        (
                            colors::read(kind, &home, accent),
                            read_motion(kind, Some(&home)),
                        )
                    })
                    .await;
                if this
                    .update(cx, |this, cx| {
                        this.set_desktop_colors(colors, cx);
                        this.set_desktop_motion(motion, cx);
                    })
                    .is_err()
                {
                    return;
                }
            }
        });
        let kwin = cx.spawn(async move |this, cx| {
            let Some(home) = home_for_blur else {
                return;
            };
            if kind != DesktopKind::Kde {
                return;
            }
            let files = [blur::kwinrc(&home)];
            let mut last = colors::stamps(&files);
            loop {
                cx.background_executor().timer(FILE_POLL).await;
                let now = colors::stamps(&files);
                if now == last {
                    continue;
                }
                last = now;
                cx.background_executor().timer(SETTLE).await;
                let strength = read_kde_blur(kind, Some(&home));
                let updated = this.update(cx, |this, cx| {
                    if this.desktop_colors.kde_blur != strength {
                        tracing::info!(?strength, "KDE blur strength changed");
                        this.desktop_colors.kde_blur = strength;
                        cx.notify();
                    }
                });
                if updated.is_err() {
                    return;
                }
            }
        });
        self.desktop_colors._watch = vec![portal, files, kwin];
    }

    fn set_desktop_colors(&mut self, colors: SystemColors, cx: &mut Context<Self>) {
        if self.desktop_colors.desktop != colors {
            tracing::info!(?colors, "desktop colors changed");
            self.desktop_colors.desktop = colors;
            self.desktop_colors.merge();
            cx.notify();
        }
    }

    fn set_desktop_motion(&mut self, motion: DesktopMotion, cx: &mut Context<Self>) {
        if self.desktop_colors.motion != motion {
            tracing::info!(?motion, "desktop motion changed");
            self.desktop_colors.motion = motion;
            apply_motion(&self.config.mail, motion, cx);
        }
    }
}
