// SPDX-License-Identifier: GPL-3.0-or-later

//! Opening a message that is not downloaded yet (older than the offline
//! window) downloads it at once: the reading view says so, then shows it,
//! or says why it could not and offers to try again.
//!
//! An attachment chip of such a message in the list downloads it too: the
//! chip fills from left to right while it does, then the attachment opens.
//! The daemon does not report how far a download is, so the fill follows
//! an estimate from the attachment's size and completes when it is done.

use gpui::{
    Animation, AnimationExt, AnyElement, Context, FontWeight, Window, div, prelude::*, px,
    relative, rgba,
};
use katna_store::MessageId;
use std::time::{Duration, Instant};

use super::MailWindow;
use super::list::CHIP_HEIGHT;
use crate::data::RowFile;
use crate::theme::{Theme, fade};
use crate::widgets::icon;
use crate::{daemon, format};

pub(super) enum Download {
    /// Being downloaded. `done` closes when it ends, either way.
    Running {
        done: async_channel::Receiver<()>,
        _end: async_channel::Sender<()>,
    },
    Failed(String),
}

/// An attachment chip that was clicked while its message was not
/// downloaded: it opens once the message is.
pub(super) struct ChipDownload {
    pub(super) file: RowFile,
    started: Instant,
    /// When the download ended, and how far the fill was then.
    finished: Option<(Instant, f32)>,
}

/// How long the fill takes to run out once the download is done.
const FINISH: Duration = Duration::from_millis(220);

impl ChipDownload {
    /// How much of the chip is filled, 0 to 1.
    fn progress(&self, now: Instant) -> f32 {
        match self.finished {
            Some((at, from)) => {
                let t = (now.saturating_duration_since(at).as_secs_f32() / FINISH.as_secs_f32())
                    .min(1.0);
                from + (1.0 - from) * (1.0 - (1.0 - t).powi(3))
            }
            None => estimate(now.saturating_duration_since(self.started), self.file.size),
        }
    }
}

/// The estimated share of a download of an attachment of `size` bytes
/// done after `elapsed`: quick at first, easing towards 90 % over about
/// the time the attachment takes at 500 kB/s.
fn estimate(elapsed: Duration, size: u64) -> f32 {
    let scale = 0.8 + size as f32 / 500_000.0;
    0.9 * (1.0 - (-elapsed.as_secs_f32() / scale).exp())
}

impl MailWindow {
    /// Starts downloading every open message of the conversation that is
    /// not stored yet, unless it is being downloaded or failed.
    pub(super) fn download_bodies(&mut self, cx: &mut Context<Self>) {
        let Some(reader) = &self.reader else {
            return;
        };
        let missing: Vec<MessageId> = reader
            .missing_bodies()
            .into_iter()
            .filter(|id| !self.downloads.contains_key(id))
            .collect();
        for id in missing {
            self.download(id, cx);
        }
    }

    /// Starts downloading message `id`, unless it is being downloaded.
    /// Returns what closes when the download ends.
    fn download(&mut self, id: MessageId, cx: &mut Context<Self>) -> async_channel::Receiver<()> {
        if let Some(Download::Running { done, .. }) = self.downloads.get(&id) {
            return done.clone();
        }
        let (end, done) = async_channel::bounded(1);
        self.downloads.insert(
            id,
            Download::Running {
                done: done.clone(),
                _end: end,
            },
        );
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::fetch_body(&connection, id.0).await
                })
                .await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(()) => {
                        this.downloads.remove(&id);
                        if let (Some(reader), Ok(mail)) = (&mut this.reader, &this.mail) {
                            reader.reload_body(id, mail);
                        }
                    }
                    Err(reason) => {
                        tracing::warn!(message = id.0, %reason, "download failed");
                        this.downloads.insert(id, Download::Failed(reason));
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
        done
    }

    /// Downloads the message of attachment chip `file`, filling the chip
    /// meanwhile, then opens the attachment.
    pub(super) fn download_row_file(
        &mut self,
        file: &RowFile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.chip_downloading(file) {
            return;
        }
        if let Some(Download::Failed(_)) = self.downloads.get(&file.message) {
            self.downloads.remove(&file.message);
        }
        let done = self.download(file.message, cx);
        self.chip_download = Some(ChipDownload {
            file: file.clone(),
            started: Instant::now(),
            finished: None,
        });
        let file = file.clone();
        cx.spawn_in(window, async move |this, cx| {
            done.recv().await.ok();
            let ours =
                |this: &MailWindow| this.chip_download.as_ref().is_some_and(|c| c.file == file);
            let failed = this
                .update(cx, |this, cx| {
                    if !ours(this) {
                        return None;
                    }
                    if let Some(Download::Failed(reason)) = this.downloads.get(&file.message) {
                        this.chip_download = None;
                        cx.notify();
                        return Some(reason.clone());
                    }
                    let now = Instant::now();
                    if let Some(chip) = &mut this.chip_download {
                        chip.finished = Some((now, chip.progress(now)));
                    }
                    cx.notify();
                    None
                })
                .ok()
                .flatten();
            if let Some(reason) = failed {
                this.update(cx, |this, cx| {
                    this.show_snackbar(
                        format!(
                            "Could not download this message. {}",
                            format::sentence(&reason)
                        ),
                        None,
                        cx,
                    )
                })
                .ok();
                return;
            }
            cx.background_executor().timer(FINISH).await;
            this.update_in(cx, |this, window, cx| {
                if !ours(this) {
                    return;
                }
                this.chip_download = None;
                if this.attachment_raw(file.message).is_some() {
                    this.open_row_file(&file, window, cx);
                } else {
                    // Never download it again and again.
                    this.show_snackbar("Could not download this message.", None, cx);
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
        cx.notify();
    }

    /// The fill behind attachment chip `file` (or its row in the "+N"
    /// list, `pill` false) while its message downloads for it; put first,
    /// in a relative parent.
    pub(super) fn chip_fill(&self, file: &RowFile, pill: bool, th: &Theme) -> Option<AnyElement> {
        let download = self.chip_download.as_ref().filter(|c| c.file == *file)?;
        let at = ChipDownload {
            file: file.clone(),
            started: download.started,
            finished: download.finished,
        };
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .h_full()
                .bg(rgba(fade(th.accent, 0.22)))
                .when(pill, |d| d.rounded_full().min_w(px(CHIP_HEIGHT - 2.0)))
                .with_animation(
                    "chip-fill",
                    Animation::new(Duration::from_secs(1)).repeat(),
                    move |fill, _| fill.w(relative(at.progress(Instant::now()))),
                )
                .into_any_element(),
        )
    }

    /// Whether attachment chip `file` is downloading its message.
    pub(super) fn chip_downloading(&self, file: &RowFile) -> bool {
        self.chip_download.as_ref().is_some_and(|c| c.file == *file)
    }

    /// What the reading view shows for message `id` while it has no body.
    pub(super) fn download_note(
        &self,
        id: MessageId,
        ix: usize,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let note = div()
            .pt(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(10.0))
            .text_size(px(14.0))
            .text_color(rgba(th.text_faint));
        match self.downloads.get(&id) {
            Some(Download::Failed(reason)) => note
                .child(icon("warning", th.error, 18.0))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_color(rgba(th.text))
                                .child("Could not download this message."),
                        )
                        .child(div().text_size(px(12.0)).child(format::sentence(reason))),
                )
                .child(
                    div()
                        .id(("download-retry", ix))
                        .flex_none()
                        .px(px(12.0))
                        .py(px(4.0))
                        .rounded_full()
                        .text_color(rgba(th.accent))
                        .font_weight(FontWeight::MEDIUM)
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(th.hover)))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.downloads.remove(&id);
                            this.download_bodies(cx);
                            cx.notify();
                        }))
                        .child("Try again"),
                )
                .into_any_element(),
            // Starting, or started on the next frame.
            _ => note
                .child(
                    div()
                        .flex_none()
                        .child(icon("download", th.accent, 18.0))
                        .with_animation(
                            ("downloading", ix),
                            Animation::new(Duration::from_millis(900))
                                .repeat()
                                .with_easing(gpui::pulsating_between(0.35, 1.0)),
                            |icon, t| icon.opacity(t),
                        ),
                )
                .child("Downloading this message from the server\u{2026}")
                .into_any_element(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_estimate_grows_and_stays_short_of_done() {
        let at = |ms| estimate(Duration::from_millis(ms), 150_000);
        assert_eq!(at(0), 0.0);
        assert!(at(500) > 0.2);
        assert!(at(500) < at(2_000) && at(2_000) < at(10_000));
        assert!(at(600_000) <= 0.9);
        // A larger attachment fills more slowly.
        assert!(estimate(Duration::from_secs(1), 5_000_000) < at(1_000));
    }

    #[test]
    fn the_fill_runs_out_once_done() {
        let start = Instant::now();
        let file = RowFile {
            message: MessageId(1),
            name: "a.pdf".into(),
            mime: "application/pdf".into(),
            size: 1_000,
            nth: 0,
            order: 0,
        };
        let chip = ChipDownload {
            file,
            started: start,
            finished: Some((start, 0.4)),
        };
        assert!((chip.progress(start) - 0.4).abs() < 1e-6);
        assert!(chip.progress(start + FINISH / 2) > 0.4);
        assert!((chip.progress(start + FINISH) - 1.0).abs() < 1e-6);
    }
}
