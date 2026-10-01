// SPDX-License-Identifier: GPL-3.0-or-later

//! Following the desktop's color scheme and accent color (§13.2): read at
//! startup, again whenever the portal says a setting changed (accent
//! color, color scheme, any `kdeglobals` group on KDE), and whenever the
//! files they come from change (theme tools write `gtk.css` without
//! telling anyone).

use std::path::{Path, PathBuf};
use std::time::Duration;

use futures_lite::StreamExt;
use gpui::{Context, Task};
use katna_chrome::Desktop;
use katna_dbus::zbus::Connection;
use katna_platform::colors::{self, DesktopKind, DesktopScheme, SystemColors};

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
        let mut this = Self {
            kind,
            config_home,
            portal_accent: None,
            desktop: colors,
            user,
            colors: SystemColors::default(),
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

impl MailWindow {
    /// Starts following the desktop's colors.
    pub(super) fn watch_colors(&mut self, cx: &mut Context<Self>) {
        let kind = self.desktop_colors.kind;
        let home = self.desktop_colors.config_home.clone();
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
                    let (accent, colors) = cx
                        .background_executor()
                        .spawn(async move {
                            let accent = colors::portal_accent(&connection).await;
                            (accent, read(kind, home.as_ref(), accent))
                        })
                        .await;
                    let updated = this.update(cx, |this, cx| {
                        this.desktop_colors.portal_accent = accent;
                        this.set_desktop_colors(colors, cx);
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
                let colors = cx
                    .background_executor()
                    .spawn(async move { colors::read(kind, &home, accent) })
                    .await;
                if this
                    .update(cx, |this, cx| this.set_desktop_colors(colors, cx))
                    .is_err()
                {
                    return;
                }
            }
        });
        self.desktop_colors._watch = vec![portal, files];
    }

    fn set_desktop_colors(&mut self, colors: SystemColors, cx: &mut Context<Self>) {
        if self.desktop_colors.desktop != colors {
            tracing::info!(?colors, "desktop colors changed");
            self.desktop_colors.desktop = colors;
            self.desktop_colors.merge();
            cx.notify();
        }
    }
}
