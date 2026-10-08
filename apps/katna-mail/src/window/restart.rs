// SPDX-License-Identifier: GPL-3.0-or-later

//! The pill that asks for a restart after Katna was updated while this
//! window was open (`docs/ARCHITECTURE.md` §21.2, Running while updated).
//!
//! It floats at the bottom centre of the mail list once the service runs
//! a newer build than this window, or the store is past what this build
//! reads. Restart opens the new Katna Mail in this one's place; × hides
//! the pill until the next update.

use gpui::{AnyElement, Context, Window, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;
use katna_ui::tokens::{elevation as lift, space, text};

use super::MailWindow;
use super::layout::FAB_SIZE;
use crate::data::OpenError;
use crate::theme::Theme;
use crate::updater;
use crate::widgets::Tip as _;
use crate::widgets::{elevation, filled_button, icon, icon_button};

/// The pill's height: a button and the room around it.
const HEIGHT: f32 = 48.0;
/// The close button inside it, smaller than a toolbar's.
const CLOSE_SIZE: f32 = 32.0;

pub(super) struct Restart {
    /// The newer build the service runs, when it does.
    newer: Option<String>,
    /// The build whose pill was closed; a later one shows it again.
    closed: Option<String>,
    shown: Spring,
}

impl Default for Restart {
    fn default() -> Self {
        Self {
            newer: None,
            closed: None,
            shown: Spring::new(motion::SMOOTH, 0.0),
        }
    }
}

/// What the store's state says about this build's age.
const STORE_TOO_NEW: &str = "store";

impl MailWindow {
    /// The service answered which build it is: `newer` when that is a
    /// later one than this window.
    pub(super) fn service_version(&mut self, newer: Option<String>, cx: &mut Context<Self>) {
        if newer.is_some() && newer != self.restart.newer {
            tracing::info!(newer, "Katna was updated; offering a restart");
        }
        self.restart.newer = newer;
        cx.notify();
    }

    /// The update the pill is for, if it shows.
    fn restart_for(&self) -> Option<&str> {
        let update =
            self.restart.newer.as_deref().or_else(|| {
                matches!(self.mail, Err(OpenError::TooNew(_))).then_some(STORE_TOO_NEW)
            })?;
        (self.restart.closed.as_deref() != Some(update)).then_some(update)
    }

    /// Moves the pill toward shown or hidden; once a frame.
    pub(super) fn tick_restart(&mut self, window: &Window, reduce: bool) {
        let target = if self.restart_for().is_some() {
            1.0
        } else {
            0.0
        };
        self.restart.shown.set(target);
        self.restart.shown.tick(window, reduce);
    }

    fn restart_now(&mut self, cx: &mut Context<Self>) {
        match updater::restart() {
            Ok(()) => {
                tracing::info!("restarting after an update");
                cx.quit();
            }
            Err(err) => {
                tracing::warn!(%err, "cannot start the new Katna Mail");
                let text = tr!("restart-failed", error = err.to_string());
                self.show_snackbar(text, None, cx);
            }
        }
    }

    fn close_restart(&mut self, cx: &mut Context<Self>) {
        self.restart.closed = self.restart_for().map(str::to_owned);
        cx.notify();
    }

    /// How far the pill lifts what else floats at the bottom of the list,
    /// such as Back to top, so they don't meet.
    pub(super) fn restart_lift(&self) -> f32 {
        (HEIGHT + space::S3) * self.restart.shown.value().clamp(0.0, 1.0)
    }

    /// The pill, over the bottom of the list (or of whatever stands in for
    /// it), above a phone's Compose button and any note at the bottom.
    pub(super) fn render_restart(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let t = self.restart.shown.value().clamp(0.0, 1.0);
        if t <= 0.001 {
            return None;
        }
        let shape = self.layout.shape;
        let snackbar = self
            .snackbar
            .as_ref()
            .map_or(0.0, |s| s.shown.value().clamp(0.0, 1.0));
        let above = space::S5 + shape.phone * (FAB_SIZE + space::S5) + 64.0 * snackbar;
        let pill = div()
            .id("restart-pill")
            // The rows under it take no hover or click.
            .occlude()
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .h(px(HEIGHT))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(space::S3))
            .pl(px(space::S5))
            .pr(px(space::S2))
            .rounded_full()
            .bg(rgba(th.raised))
            .shadow(elevation(th, lift::FLOAT))
            .child(icon("refresh", th.text_dim, 18.0))
            .child(
                div()
                    .text_size(px(text::BODY))
                    .text_color(rgba(th.text))
                    .whitespace_nowrap()
                    .child(tr!("restart-updated")),
            )
            .child(
                filled_button("restart-now", tr!("restart-button"), th)
                    .on_click(cx.listener(|this, _, _, cx| this.restart_now(cx))),
            )
            .child(
                icon_button("restart-close", "close", 18.0, th)
                    .size(px(CLOSE_SIZE))
                    .tip(tr!("restart-close"), th)
                    .on_click(cx.listener(|this, _, _, cx| this.close_restart(cx))),
            );
        Some(
            div()
                .absolute()
                .left_0()
                .right_0()
                .bottom(px(above + lerp(-12.0, 0.0, t)))
                .flex()
                .justify_center()
                .opacity(t)
                .child(pill)
                .into_any_element(),
        )
    }
}
