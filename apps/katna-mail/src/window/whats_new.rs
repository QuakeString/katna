// SPDX-License-Identifier: GPL-3.0-or-later

//! The What's new dialog: after an update, once, the version now running,
//! the highlights not shown before (newest first, a major one with a short
//! animation) and a link to every change. Quick settings opens it again.
//! A first start gets onboarding or the tour instead, and nothing older
//! counts as new afterwards. The highlights live in `crate::whats_new`.

use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;

use gpui::{
    AnyElement, Context, FocusHandle, FontWeight, ImageSource, KeyDownEvent, MouseButton,
    ObjectFit, RenderImage, ScrollHandle, Task, Window, div, img, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_preview::image::codecs::webp::WebPDecoder;
use katna_preview::image::{AnimationDecoder, Frame};
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::px;
use katna_ui::tokens::space;
use katna_ui::unpx;

use super::add_account::text_button;
use super::{MailWindow, PANEL_RADIUS};
use crate::theme::{Theme, fade};
use crate::whats_new::{self, Highlight, Seen, Start};
use crate::widgets::{FocusRing, elevation, filled_button, icon};

const WIDTH: f32 = 560.0;

pub(super) struct WhatsNew {
    highlights: Vec<&'static Highlight>,
    /// Highlights past the ones shown.
    more: usize,
    /// Opened because the app was updated, not from quick settings.
    updated: bool,
    /// The version that ran before, for the changelog link.
    from: Option<String>,
    /// The theme was dark when it opened: which animations to show.
    dark: bool,
    /// Decoded animations by highlight name.
    animations: HashMap<&'static str, Arc<RenderImage>>,
    _decode: Option<Task<()>>,
    focus: FocusHandle,
    /// The highlights' scroll, which slides under the header.
    scroll: ScrollHandle,
    closing: bool,
    shown: Spring,
}

impl MailWindow {
    /// Decides between the first-start help and What's new, and records
    /// this version. Called once, when the window opens.
    pub(super) fn welcome_or_whats_new(
        &mut self,
        config_existed: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let onboarding = &self.config.onboarding;
        let seen = Seen::new(&onboarding.whats_new_shown, onboarding.whats_new_seen);
        let from = onboarding.last_version.clone();
        let start = whats_new::start(onboarding.done, seen.any(), config_existed);
        let first_run = self.needs_account() || start == Start::FirstRun;
        if first_run {
            if !self.needs_account() {
                self.start_tour(true, window, cx);
            }
        } else {
            let (highlights, more) = whats_new::unseen(&seen);
            let shown = !highlights.is_empty();
            if shown {
                self.open_whats_new(highlights, more, true, from, window, cx);
            }
            // Installed before "Help improve Katna" was asked: ask once,
            // after What's new.
            if self.share_unanswered() {
                if shown {
                    self.share_ask_later = true;
                } else {
                    self.open_share_ask(window, cx);
                }
            }
        }
        // Shown once: a start that ends before the dialog or the tour is
        // closed does not bring them back.
        let names = whats_new::names();
        let onboarding = &mut self.config.onboarding;
        let changed = onboarding.whats_new_shown != names
            || onboarding.whats_new_seen.is_some()
            || onboarding.last_version.as_deref() != Some(whats_new::VERSION);
        onboarding.whats_new_shown = names;
        onboarding.whats_new_seen = None;
        onboarding.last_version = Some(whats_new::VERSION.to_owned());
        if changed {
            self.save_config();
        }
    }

    /// Opens What's new with the newest highlights, from quick settings.
    pub(super) fn show_whats_new(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.settings_open = false;
        let (highlights, more) = whats_new::recent();
        self.open_whats_new(highlights, more, false, None, window, cx);
    }

    fn open_whats_new(
        &mut self,
        highlights: Vec<&'static Highlight>,
        more: usize,
        updated: bool,
        from: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let focus = cx.focus_handle();
        window.focus(&focus, cx);
        let dark = self.theme(window).dark;
        let wanted: Vec<_> = highlights
            .iter()
            .filter_map(|h| Some((h.name, h.animation.as_ref()?.for_theme(dark))))
            .collect();
        let decode = (!wanted.is_empty()).then(|| {
            cx.spawn(async move |this, cx| {
                let decoded = cx
                    .background_executor()
                    .spawn(async move {
                        wanted
                            .into_iter()
                            .filter_map(|(id, bytes)| Some((id, decode(bytes)?)))
                            .collect::<Vec<_>>()
                    })
                    .await;
                this.update(cx, |this, cx| match &mut this.whats_new {
                    Some(dialog) if !dialog.closing => {
                        dialog.animations.extend(decoded);
                        cx.notify();
                    }
                    _ => this
                        .files
                        .released
                        .extend(decoded.into_iter().map(|(_, image)| image)),
                })
                .ok();
            })
        });
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        if let Some(old) = self.whats_new.take() {
            self.files.released.extend(old.animations.into_values());
        }
        self.whats_new = Some(WhatsNew {
            highlights,
            more,
            updated,
            from,
            dark,
            animations: HashMap::new(),
            _decode: decode,
            focus,
            scroll: ScrollHandle::new(),
            closing: false,
            shown,
        });
        cx.notify();
    }

    /// What's new is open and not on its way out.
    pub(super) fn whats_new_open(&self) -> bool {
        self.whats_new.as_ref().is_some_and(|d| !d.closing)
    }

    pub(super) fn close_whats_new(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(dialog) = &mut self.whats_new
            && !dialog.closing
        {
            dialog.closing = true;
            dialog.shown.set(0.0);
            window.focus(&self.list_focus, cx);
        }
        if self.share_ask_later {
            self.open_share_ask(window, cx);
        }
        cx.notify();
    }

    fn whats_new_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        // Escape reaches `popovers` first; this is for Enter, unless a
        // focused button takes it.
        let on_dialog = self
            .whats_new
            .as_ref()
            .is_some_and(|d| d.focus.is_focused(window));
        if event.keystroke.key == "escape" || (event.keystroke.key == "enter" && on_dialog) {
            self.close_whats_new(window, cx);
            cx.stop_propagation();
        }
    }

    pub(super) fn render_whats_new(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let dialog = self.whats_new.as_mut()?;
        let t = dialog.shown.tick(window, reduce);
        if dialog.closing && dialog.shown.settled() {
            if let Some(dialog) = self.whats_new.take() {
                self.files.released.extend(dialog.animations.into_values());
            }
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let dialog = self.whats_new.as_ref()?;
        let phone = self.layout.shape.is_phone();
        let vw = unpx(window.viewport_size().width);
        let width = if phone { vw } else { WIDTH.min(vw - 48.0) };
        // Room for the animations: the card less its padding.
        let inner = width - 48.0;
        // The first highlight's animation spans the top of the card, with
        // the card's top corners, and stays put while the highlights scroll
        // (scrolled under the card's corners, it would show square ones).
        let corner = if phone { 0.0 } else { PANEL_RADIUS };
        let hero = dialog
            .highlights
            .first()
            .and_then(|h| Some((h.name, h.animation.as_ref()?.for_theme(dialog.dark))))
            .map(|(id, bytes)| {
                let (w, h) = size(bytes, width);
                // A narrower animation sits clear of the corners.
                let r = if w + 0.5 >= width { corner } else { 0.0 };
                div()
                    .flex_none()
                    .w_full()
                    .h(px(h))
                    .flex()
                    .justify_center()
                    .rounded_t(px(corner))
                    .bg(rgba(th.backdrop))
                    .border_b_1()
                    .border_color(rgba(th.divider))
                    .child(animation(id, dialog.animations.get(id), (w, h), (r, 0.0)))
            });

        let has_hero = hero.is_some();
        // The header stays put and the highlights scroll under it; a line
        // fades in below it once they have moved.
        let scrolled = (-unpx(dialog.scroll.offset().y) / 12.0).clamp(0.0, 1.0);
        let header = div()
            .flex_none()
            .border_b_1()
            .border_color(rgba(fade(th.divider, scrolled)))
            .px(px(24.0))
            .pt(px(if has_hero { 20.0 } else { 24.0 }))
            .pb(px(16.0))
            .flex()
            .flex_row()
            .items_start()
            .gap(px(16.0))
            .child(crate::widgets::katna_mark(48.0, th))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(
                        div()
                            .text_size(px(22.0))
                            .line_height(px(30.0))
                            .child(tr!("whats-new-title")),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .line_height(px(18.0))
                            .text_color(rgba(th.text_dim))
                            .child(if dialog.updated {
                                tr!("whats-new-updated", version = whats_new::VERSION)
                            } else {
                                tr!("whats-new-version", version = whats_new::VERSION)
                            }),
                    ),
            );

        let items = dialog.highlights.iter().enumerate().map(|(ix, highlight)| {
            let inline = highlight
                .animation
                .as_ref()
                .filter(|_| ix > 0 || !has_hero)
                .map(|a| a.for_theme(dialog.dark));
            let animation = inline.map(|bytes| {
                let (w, h) = size(bytes, inner);
                div()
                    .mb(px(12.0))
                    .overflow_hidden()
                    .rounded(px(12.0))
                    .border_1()
                    .border_color(rgba(th.outline))
                    .child(animation(
                        highlight.name,
                        dialog.animations.get(highlight.name),
                        (w, h),
                        // Inside the 12 px frame and its 1 px border.
                        (11.0, 11.0),
                    ))
            });
            div()
                .flex_none()
                .px(px(24.0))
                .when(ix > 0, |d| d.mt(px(20.0)))
                .flex()
                .flex_col()
                .children(animation)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_start()
                        .gap(px(12.0))
                        .child(div().flex_none().mt(px(1.0)).child(icon(
                            "check-circle",
                            th.accent,
                            20.0,
                        )))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .flex()
                                .flex_col()
                                .gap(px(2.0))
                                .child(
                                    div()
                                        .text_size(px(15.0))
                                        .line_height(px(22.0))
                                        .font_weight(FontWeight::MEDIUM)
                                        .child(highlight.title()),
                                )
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .line_height(px(21.0))
                                        .text_color(rgba(th.text_dim))
                                        .child(highlight.text()),
                                ),
                        ),
                )
        });
        let more = (dialog.more > 0).then(|| {
            div()
                .mt(px(20.0))
                .px(px(24.0))
                .text_size(px(14.0))
                .text_color(rgba(th.text_dim))
                .child(tr!("whats-new-more", count = dialog.more))
        });
        let body = div()
            .id("whats-new-body")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .track_scroll(&dialog.scroll)
            .pt(px(space::S2))
            .pb(px(8.0))
            .flex()
            .flex_col()
            .children(items)
            .children(more);

        let url = whats_new::changelog_url(dialog.from.as_deref());
        let footer = div()
            .flex_none()
            .px(px(16.0))
            .pt(px(12.0))
            .pb(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .child(
                text_button("whats-new-changelog", tr!("whats-new-changelog"), th)
                    .focus_ring(th)
                    .gap(px(8.0))
                    .child(icon("open-external", th.accent, 18.0))
                    .on_click(move |_, _, cx| cx.open_url(&url)),
            )
            .child(div().flex_1())
            .child(
                filled_button("whats-new-close", tr!("whats-new-got-it"), th)
                    .focus_ring_filled(th)
                    .on_click(cx.listener(|this, _, window, cx| this.close_whats_new(window, cx))),
            );

        let card = div()
            .id("whats-new")
            .track_focus(&dialog.focus)
            .map(|d| super::popovers::keep_tab_inside(d, &dialog.focus))
            .on_key_down(cx.listener(Self::whats_new_key))
            .occlude()
            .w(px(width))
            .when(phone, |d| d.h_full())
            .when(!phone, |d| {
                d.max_h(px(super::about::dialog_max_height(window)))
                    .min_h_0()
            })
            .flex()
            .flex_col()
            .overflow_hidden()
            .when(!phone, |d| {
                d.rounded(px(PANEL_RADIUS)).shadow(elevation(th, 3.0))
            })
            .map(|d| {
                let radius = if phone { 0.0 } else { PANEL_RADIUS };
                crate::widgets::frosted(d, th, th.surface, radius)
            })
            .text_color(rgba(th.text))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .children(hero)
            .child(header)
            .child(body)
            .child(footer);
        Some(
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .when(!phone, |d| d.p(px(24.0)))
                // No veil: the window stays as it is around the dialog.
                .child(
                    div()
                        .id("whats-new-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(
                            cx.listener(|this, _, window, cx| this.close_whats_new(window, cx)),
                        ),
                )
                .child(
                    // Kept within the room below the top bar; the list of
                    // highlights scrolls when it does not fit.
                    div()
                        .max_h_full()
                        .when(phone, |d| d.h_full())
                        .flex()
                        .flex_col()
                        .opacity(t)
                        .mt(px(lerp(24.0, 0.0, t)))
                        .child(card),
                )
                .into_any_element(),
        )
    }
}

/// An animation at `w` by `h`, or its room while it is decoded, with its
/// top and bottom corners rounded by `top` and `bottom`: GPUI clips a child
/// to a square, so the image rounds itself to fit a rounded frame.
fn animation(
    name: &'static str,
    image: Option<&Arc<RenderImage>>,
    (w, h): (f32, f32),
    (top, bottom): (f32, f32),
) -> gpui::Div {
    div()
        .flex_none()
        .w(px(w))
        .h(px(h))
        .children(image.map(|image| {
            // GPUI keeps an image's frame in its element state, so only an
            // image with an id plays.
            img(ImageSource::Render(image.clone()))
                .id(name)
                .size_full()
                .object_fit(ObjectFit::Contain)
                .rounded_t(px(top))
                .rounded_b(px(bottom))
        }))
}

/// The size to draw an animation: its own pixels (recorded at the size
/// they should appear), smaller if `room` is narrower.
fn size(bytes: &[u8], room: f32) -> (f32, f32) {
    let (w, h) = webp_size(bytes).unwrap_or((560, 200));
    let shown = (w as f32).min(room.max(1.0));
    (shown, shown * h as f32 / w.max(1) as f32)
}

/// An animated WebP as GPUI draws it (BGRA frames with their delays).
fn decode(bytes: &[u8]) -> Option<Arc<RenderImage>> {
    let decoder = WebPDecoder::new(Cursor::new(bytes)).ok()?;
    let frames: Vec<Frame> = decoder
        .into_frames()
        .filter_map(|frame| frame.ok())
        .map(|mut frame| {
            for pixel in frame.buffer_mut().pixels_mut() {
                pixel.0.swap(0, 2);
            }
            frame
        })
        .collect();
    if frames.is_empty() {
        tracing::warn!("a What's new animation could not be decoded");
        return None;
    }
    Some(Arc::new(RenderImage::new(frames)))
}

/// The width and height of a WebP, from its header, so the room for an
/// animation is kept before it is decoded.
fn webp_size(bytes: &[u8]) -> Option<(u32, u32)> {
    WebPDecoder::new(Cursor::new(bytes))
        .ok()
        .map(|d| katna_preview::image::ImageDecoder::dimensions(&d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_animation_decodes() {
        for highlight in whats_new::HIGHLIGHTS {
            let Some(animation) = &highlight.animation else {
                continue;
            };
            for bytes in [animation.light, animation.dark] {
                let image = decode(bytes).expect(highlight.name);
                assert!(
                    image.frame_count() > 1,
                    "{} is not animated",
                    highlight.name
                );
                assert_eq!(webp_size(bytes), webp_size(animation.light));
            }
        }
    }
}
