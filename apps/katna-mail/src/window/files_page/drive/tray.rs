// SPDX-License-Identifier: GPL-3.0-or-later

//! The uploads tray: a card at the bottom right, over every page, with
//! each file going up to a drive from Files, its ring, Cancel, and a
//! click on a finished one to see it in its folder
//! (`docs/ARCHITECTURE.md` §13.8). It stays until closed.

use crate::widgets::Tip as _;
use std::time::Instant;

use gpui::{AnyElement, Context, MouseButton, Window, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_ui::motion::lerp;
use katna_ui::px;

use super::super::super::MailWindow;
use super::super::super::apps::App;
use super::{DriveView, Going, Upload};
use crate::format;
use crate::theme::{Theme, fade};
use crate::widgets::{elevation, icon, icon_button, ring};

/// The tray's corner radius.
const RADIUS: f32 = 14.0;

const WIDTH: f32 = 360.0;
const ROW_HEIGHT: f32 = 52.0;
/// Rows shown before the list scrolls.
const ROWS_SHOWN: usize = 5;

impl Upload {
    /// Where it goes: the folder's name, or the drive's.
    fn place(&self, drive: &str) -> String {
        self.crumbs
            .last()
            .map_or_else(|| drive.to_owned(), |(_, name)| name.clone())
    }
}

impl MailWindow {
    /// Stops every upload still going.
    fn cancel_uploads(&mut self, only: Option<i64>, cx: &mut Context<Self>) {
        let Some(connection) = self.daemon.clone() else {
            return;
        };
        let mut ids = Vec::new();
        for (id, upload) in &mut self.library.cloud.uploads {
            if upload.state == Going::Uploading && only.is_none_or(|o| o == *id) {
                upload.state = Going::Cancelled;
                ids.push(*id);
            }
        }
        cx.spawn(async move |_, _| {
            for id in ids {
                if let Err(err) = crate::daemon::drive_cancel(&connection, id).await {
                    tracing::warn!(upload = id, %err, "cancelling an upload");
                }
            }
        })
        .detach();
        cx.notify();
    }

    /// Opens the folder upload `id` went into, on the Files page.
    fn show_upload(&mut self, id: i64, cx: &mut Context<Self>) {
        let Some((_, upload)) = self.library.cloud.uploads.iter().find(|(u, _)| *u == id) else {
            return;
        };
        let mut view = DriveView::new(upload.key.0, false);
        view.crumbs = upload.crumbs.clone();
        self.library.cloud.view = Some(view);
        self.library.menu = None;
        self.open_app(App::Files, cx);
        self.load_drive(false, cx);
        cx.notify();
    }

    /// The tray, while there are uploads to show.
    pub(in crate::window) fn render_upload_tray(
        &mut self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let cloud = &self.library.cloud;
        if cloud.uploads.is_empty() {
            return None;
        }
        let th = &th.lifted();
        let going: Vec<&Upload> = cloud
            .uploads
            .iter()
            .map(|(_, u)| u)
            .filter(|u| u.state == Going::Uploading)
            .collect();
        let done = cloud
            .uploads
            .iter()
            .filter(|(_, u)| u.state == Going::Done)
            .count();
        let failed = cloud
            .uploads
            .iter()
            .filter(|(_, u)| matches!(u.state, Going::Failed(_) | Going::NeedsPermission))
            .count();
        let title = if !going.is_empty() {
            tr!("files-tray-uploading", count = going.len())
        } else if failed > 0 {
            tr!("files-tray-some-failed", done = done, failed = failed)
        } else {
            tr!("files-tray-done", count = done)
        };
        let folded = cloud.tray_folded;
        // How long the rest takes, at the speed so far.
        let left = (!going.is_empty()).then(|| {
            let sent: u64 = going.iter().map(|u| u.sent).sum();
            let size: u64 = going.iter().map(|u| u.size.max(u.sent)).sum();
            let since = going
                .iter()
                .map(|u| u.started)
                .min()
                .unwrap_or_else(Instant::now)
                .elapsed()
                .as_secs_f64();
            if sent == 0 || since < 2.0 {
                return tr!("files-tray-starting");
            }
            let rest = (size - sent) as f64 / (sent as f64 / since);
            if rest < 60.0 {
                tr!("files-tray-seconds-left")
            } else {
                tr!(
                    "files-tray-minutes-left",
                    minutes = (rest / 60.0).ceil() as u64
                )
            }
        });
        if !going.is_empty() {
            // The time left and the rings follow the bytes.
            window.request_animation_frame();
        }
        let head = div()
            .flex_none()
            .h(px(52.0))
            .pl(px(16.0))
            .pr(px(6.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0))
            // Its own corners follow the tray's: a child's fill is not
            // clipped to the rounded ones.
            .rounded_t(px(RADIUS))
            .when(folded, |d| d.rounded_b(px(RADIUS)))
            .bg(rgba(th.backdrop))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .text_size(px(15.0))
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .child(title),
            )
            .child(
                icon_button(
                    "files-tray-fold",
                    if folded { "chevron-up" } else { "chevron-down" },
                    20.0,
                    th,
                )
                .tip(
                    if folded {
                        tr!("files-tray-unfold")
                    } else {
                        tr!("files-tray-fold")
                    },
                    th,
                )
                .on_click(cx.listener(|this, _, _, cx| {
                    this.library.cloud.tray_folded = !this.library.cloud.tray_folded;
                    cx.notify();
                })),
            )
            .when(going.is_empty(), |d| {
                d.child(
                    icon_button("files-tray-close", "close", 20.0, th)
                        .tip(tr!("files-tray-close"), th)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.library.cloud.uploads.clear();
                            cx.notify();
                        })),
                )
            });
        let sub = left.map(|left| {
            div()
                .flex_none()
                .h(px(36.0))
                .pl(px(16.0))
                .pr(px(8.0))
                .flex()
                .flex_row()
                .items_center()
                .border_b_1()
                .border_color(rgba(th.divider))
                .text_size(px(13.0))
                .text_color(rgba(th.text_dim))
                .child(div().flex_1().min_w_0().truncate().child(left))
                .child(
                    div()
                        .id("files-tray-cancel-all")
                        .px(px(10.0))
                        .py(px(4.0))
                        .rounded_full()
                        .text_color(rgba(th.accent))
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(fade(th.accent, 0.08))))
                        .on_click(cx.listener(|this, _, _, cx| this.cancel_uploads(None, cx)))
                        .child(tr!("files-tray-cancel-all")),
                )
        });
        let rows = (!folded).then(|| {
            let rows: Vec<AnyElement> = self
                .library
                .cloud
                .uploads
                .iter()
                .map(|(id, upload)| self.render_upload_row(*id, upload, th, cx))
                .collect();
            div()
                .id("files-tray-rows")
                .flex_none()
                .max_h(px(ROW_HEIGHT * ROWS_SHOWN as f32 + 8.0))
                .overflow_y_scroll()
                .py(px(4.0))
                .flex()
                .flex_col()
                .children(rows)
        });
        let shape = self.layout.shape;
        let edge = lerp(24.0, 8.0, shape.phone);
        Some(
            div()
                .id("files-tray")
                .occlude()
                .absolute()
                .right(px(edge))
                .when(shape.is_phone(), |d| d.left(px(edge)))
                .bottom(px(shape.bottom_bar() + edge))
                .when(!shape.is_phone(), |d| d.w(px(WIDTH)))
                .flex()
                .flex_col()
                .overflow_hidden()
                .rounded(px(RADIUS))
                .bg(rgba(th.surface))
                .text_color(rgba(th.text))
                .shadow(elevation(th, 3.0))
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(head)
                .children(sub.filter(|_| !folded))
                .children(rows)
                .into_any_element(),
        )
    }

    fn render_upload_row(
        &self,
        id: i64,
        upload: &Upload,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let drive = self.drive_name(upload.key.0);
        let place = upload.place(&drive);
        let (mark, line, error) = match &upload.state {
            Going::Uploading => {
                let share = if upload.size == 0 {
                    0.0
                } else {
                    (upload.sent as f64 / upload.size as f64) as f32
                };
                (
                    div()
                        .relative()
                        .flex_none()
                        .size(px(22.0))
                        .child(ring(
                            0.0,
                            share.clamp(0.02, 1.0),
                            th.accent,
                            th.divider,
                            22.0,
                        ))
                        .into_any_element(),
                    tr!(
                        "files-tray-progress",
                        place = place.as_str(),
                        sent = format::size(upload.sent),
                        size = format::size(upload.size)
                    ),
                    false,
                )
            }
            Going::Done => (
                icon("check-circle", th.accent, 22.0),
                tr!("files-tray-in", place = place.as_str()),
                false,
            ),
            Going::Cancelled => (
                icon("close", th.text_faint, 22.0),
                tr!("files-tray-cancelled"),
                false,
            ),
            Going::NeedsPermission => (
                icon("warning", th.error, 22.0),
                tr!("files-drive-upload-needs"),
                true,
            ),
            Going::Failed(err) => (icon("warning", th.error, 22.0), err.clone(), true),
        };
        let going = upload.state == Going::Uploading;
        let finished = upload.state == Going::Done;
        let group = gpui::SharedString::from(format!("files-tray-row-{id}"));
        div()
            .id(("files-tray-row", id as u64))
            .group(group.clone())
            .flex_none()
            .h(px(ROW_HEIGHT))
            .mx(px(4.0))
            .pl(px(12.0))
            .pr(px(4.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .rounded(px(10.0))
            .when(finished, |d| {
                d.cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .on_click(cx.listener(move |this, _, _, cx| this.show_upload(id, cx)))
            })
            .when(going, |d| d.hover(|s| s.bg(rgba(th.hover))))
            .child(mark)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .truncate()
                            .text_size(px(14.0))
                            .child(upload.name.clone()),
                    )
                    .child(
                        div()
                            .truncate()
                            .text_size(px(12.0))
                            .text_color(rgba(if error { th.error } else { th.text_dim }))
                            .child(line),
                    ),
            )
            .when(going, |d| {
                d.child(
                    div().invisible().group_hover(group, |s| s.visible()).child(
                        icon_button(("files-tray-cancel", id as u64), "close", 18.0, th)
                            .tip(tr!("files-tray-cancel"), th)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                cx.stop_propagation();
                                this.cancel_uploads(Some(id), cx);
                            })),
                    ),
                )
            })
            .into_any_element()
    }
}
