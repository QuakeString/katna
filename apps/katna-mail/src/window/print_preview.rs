// SPDX-License-Identifier: GPL-3.0-or-later

//! The print preview: the conversation's pages as they will print, on A4
//! or Letter, before the desktop's print dialog. Pages picks all of them
//! or the ones typed (`3-5`, `1-3, 8`); the others fade in the preview and
//! are left out. Print hands the pages to that dialog (`print`); Cancel,
//! Escape or a click beside the dialog closes it.

use std::sync::Arc;

use gpui::{
    AnyElement, Context, Entity, FocusHandle, FontWeight, ImageSource, KeyDownEvent, MouseButton,
    ObjectFit, RenderImage, Subscription, Task, Window, div, img, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_preview::pdf::Document;
use katna_render::print::{PagesError, Paper, PrintOptions, parse_pages};
use katna_ui::motion::{self, Spring, lerp};
use katna_ui::tokens::{space, state, text};
use katna_ui::{InputEvent, TextInput, px};

use super::attachments::bitmap;
use super::print::{PrintJob, local_paper};
use super::{MailWindow, PANEL_RADIUS};
use crate::theme::{Theme, fade};
use crate::widgets::{
    CHIP_HEIGHT, FocusRing, elevation, field, filled_button, outlined_button, switch,
};

const WIDTH: f32 = 720.0;

/// How wide a page shows, at most.
const PAGE_WIDTH: f32 = 560.0;

/// Pages drawn in the preview; a longer conversation says how many more
/// it prints.
const MAX_PAGES: usize = 30;

/// How wide the box for typing pages is.
const PAGES_FIELD_WIDTH: f32 = 150.0;

pub(super) struct PrintPreview {
    focus: FocusHandle,
    shown: Spring,
    closing: bool,
    job: Arc<PrintJob>,
    paper: Paper,
    options: PrintOptions,
    pages: Pages,
    /// The pages typed to print; blank prints them all.
    range: Entity<TextInput>,
    _range_events: Subscription,
    /// Page bitmaps no longer shown, for the window to free.
    released: Vec<Arc<RenderImage>>,
    _layout: Task<()>,
}

enum Pages {
    /// Being laid out.
    Laying,
    Ready {
        pdf: Arc<Vec<u8>>,
        /// The first [`MAX_PAGES`] pages.
        images: Vec<Arc<RenderImage>>,
        total: usize,
    },
    Failed(String),
}

impl MailWindow {
    pub(super) fn open_print_preview(
        &mut self,
        job: Arc<PrintJob>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.menu = None;
        let focus = cx.focus_handle();
        window.focus(&focus, cx);
        let mut shown = Spring::new(motion::SMOOTH, 0.0);
        shown.set(1.0);
        let paper = local_paper();
        // As the reader shows it.
        let options = PrintOptions::default();
        let task = self.lay_out_preview(job.clone(), paper, options, window, cx);
        let released = self
            .print_preview
            .take()
            .map(|old| old.images())
            .unwrap_or_default();
        let accent = rgba(self.theme(window).accent).into();
        let range = cx.new(|cx| {
            let mut input = TextInput::new(tr!("print-preview-pages-hint"), cx);
            input.set_accent(accent);
            input
        });
        let range_events = cx.subscribe_in(
            &range,
            window,
            |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Changed => cx.notify(),
                InputEvent::Submit => this.print_previewed(window, cx),
                InputEvent::Cancel => this.close_print_preview(window, cx),
            },
        );
        self.print_preview = Some(PrintPreview {
            focus,
            shown,
            closing: false,
            job,
            paper,
            options,
            pages: Pages::Laying,
            range,
            _range_events: range_events,
            released,
            _layout: task,
        });
        cx.notify();
    }

    /// The preview is open and not on its way out.
    pub(super) fn print_preview_open(&self) -> bool {
        self.print_preview.as_ref().is_some_and(|p| !p.closing)
    }

    pub(super) fn close_print_preview(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(preview) = &mut self.print_preview
            && !preview.closing
        {
            preview.closing = true;
            preview.shown.set(0.0);
            window.focus(&self.list_focus, cx);
        }
        cx.notify();
    }

    /// Lays `job` out on `paper` and draws its pages, off the UI thread.
    fn lay_out_preview(
        &self,
        job: Arc<PrintJob>,
        paper: Paper,
        options: PrintOptions,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        // Sharp on this screen at the page's width.
        let scale = PAGE_WIDTH * katna_ui::scale::scale() * window.scale_factor() / paper.width;
        cx.spawn(async move |this, cx| {
            let laid = cx
                .background_executor()
                .spawn(async move {
                    let pdf = job.layout(paper, options)?;
                    let doc =
                        Document::open(pdf.clone()).map_err(|_| tr!("print-preview-failed"))?;
                    let images = (0..doc.pages().min(MAX_PAGES))
                        .filter_map(|page| doc.render(page, scale).map(bitmap))
                        .collect::<Vec<_>>();
                    Ok::<_, String>((Arc::new(pdf), images, doc.pages()))
                })
                .await;
            this.update(cx, |this, cx| {
                let Some(preview) = &mut this.print_preview else {
                    return;
                };
                if preview.paper != paper || preview.options != options {
                    return;
                }
                let old = std::mem::replace(
                    &mut preview.pages,
                    match laid {
                        Ok((pdf, images, total)) => Pages::Ready { pdf, images, total },
                        Err(err) => Pages::Failed(err),
                    },
                );
                if let Pages::Ready { images, .. } = old {
                    preview.released.extend(images);
                }
                cx.notify();
            })
            .ok();
        })
    }

    /// Lays the preview out again on `paper` with `options`.
    fn set_print_setup(
        &mut self,
        paper: Paper,
        options: PrintOptions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(job) = self
            .print_preview
            .as_ref()
            .filter(|p| p.paper != paper || p.options != options)
            .map(|p| p.job.clone())
        else {
            return;
        };
        let task = self.lay_out_preview(job, paper, options, window, cx);
        if let Some(preview) = &mut self.print_preview {
            preview.paper = paper;
            preview.options = options;
            if let Pages::Ready { images, .. } =
                std::mem::replace(&mut preview.pages, Pages::Laying)
            {
                preview.released.extend(images);
            }
            preview._layout = task;
        }
        cx.notify();
    }

    /// Print: the pages go to the desktop's print dialog.
    fn print_previewed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(preview) = &self.print_preview else {
            return;
        };
        let Pages::Ready { pdf, total, .. } = &preview.pages else {
            return;
        };
        let Ok(chosen) = preview.chosen(*total, cx) else {
            return;
        };
        // Every page: the PDF as it is.
        let pages = (chosen.len() < *total).then(|| Arc::from(chosen));
        let (job, pdf, paper, options) = (
            preview.job.clone(),
            pdf.clone(),
            preview.paper,
            preview.options,
        );
        self.close_print_preview(window, cx);
        self.print_pdf(job, pdf, paper, options, pages, cx);
    }

    /// All: the typed pages are cleared.
    fn print_all_pages(&mut self, cx: &mut Context<Self>) {
        if let Some(preview) = &self.print_preview {
            preview.range.update(cx, |input, cx| input.set_text("", cx));
        }
        cx.notify();
    }

    fn print_preview_focused(&self, window: &Window) -> bool {
        self.print_preview
            .as_ref()
            .is_some_and(|p| p.focus.is_focused(window))
    }

    fn print_preview_key(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event.keystroke.key.as_str() {
            // Escape reaches `popovers` first.
            "escape" => self.close_print_preview(window, cx),
            // Unless a focused button takes it.
            "enter" if self.print_preview_focused(window) => self.print_previewed(window, cx),
            _ => return,
        }
        cx.stop_propagation();
    }

    pub(super) fn render_print_preview(
        &mut self,
        th: &Theme,
        window: &mut Window,
        reduce: bool,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        // It floats: its surface is a step lighter in dark colors.
        let th = &th.lifted();
        let preview = self.print_preview.as_mut()?;
        for image in preview.released.drain(..) {
            window.drop_image(image).ok();
        }
        let t = preview.shown.tick(window, reduce);
        if preview.closing && preview.shown.settled() {
            if let Some(preview) = self.print_preview.take() {
                for image in preview.images() {
                    window.drop_image(image).ok();
                }
            }
            return None;
        }
        let t = t.clamp(0.0, 1.0);
        let focus = preview.focus.clone();
        let paper = preview.paper;
        let phone = self.layout.shape.is_phone();
        // `room_width`, while the preview is borrowed.
        let vw = self.layout.shape.width;
        let width = if phone { vw } else { WIDTH.min(vw - 48.0) };
        let page_width = PAGE_WIDTH.min(width - 48.0).max(120.0);
        let page_height = page_width * paper.height / paper.width;
        // The pages to print, once it is known how many there are.
        let chosen = match &preview.pages {
            Pages::Ready { total, .. } => Some((preview.chosen(*total, cx), *total)),
            _ => None,
        };
        let all = preview.range.read(cx).text().trim().is_empty();
        let printed = |n: usize| match &chosen {
            Some((Ok(pages), _)) => pages.contains(&n),
            _ => true,
        };
        let ready = matches!(chosen, Some((Ok(_), _)));
        let page = |child: Option<AnyElement>| {
            div()
                .flex_none()
                .w(px(page_width))
                .h(px(page_height))
                .bg(rgba(0xffff_ffff))
                .shadow(elevation(th, 1.0))
                .flex()
                .items_center()
                .justify_center()
                .children(child)
        };
        let (count, pages): (String, Vec<AnyElement>) = match &preview.pages {
            Pages::Laying => (
                tr!("print-preview-laying-out"),
                vec![page(None).into_any_element()],
            ),
            Pages::Failed(err) => (
                String::new(),
                vec![
                    div()
                        .py(px(48.0))
                        .text_color(rgba(th.text_dim))
                        .child(tr!("print-failed", error = err.as_str()))
                        .into_any_element(),
                ],
            ),
            Pages::Ready { images, total, .. } => {
                let mut pages: Vec<AnyElement> = images
                    .iter()
                    .enumerate()
                    .map(|(ix, image)| {
                        let n = ix + 1;
                        let on = printed(n);
                        // Its number under it, to type in Pages; one left
                        // out fades back.
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(space::S3))
                            .child(
                                page(Some(
                                    img(ImageSource::Render(image.clone()))
                                        .size_full()
                                        .object_fit(ObjectFit::Fill)
                                        .into_any_element(),
                                ))
                                .when(!on, |d| d.opacity(LEFT_OUT)),
                            )
                            .child(
                                div()
                                    .text_size(px(text::CAPTION))
                                    .text_color(rgba(if on { th.text_dim } else { th.text_faint }))
                                    .child(crate::format::thousands(n as u64)),
                            )
                            .into_any_element()
                    })
                    .collect();
                if *total > images.len() {
                    pages.push(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgba(th.text_dim))
                            .child(tr!("print-preview-more", count = total - images.len()))
                            .into_any_element(),
                    );
                }
                (tr!("print-preview-pages", count = *total), pages)
            }
        };
        // Some pages: "3 of 12 pages", the 3 in the accent.
        let count = match &chosen {
            Some((Ok(pages), total)) if pages.len() < *total => div()
                .flex()
                .flex_row()
                .gap(px(space::S2))
                .child(
                    div()
                        .text_color(rgba(th.accent))
                        .font_weight(FontWeight::MEDIUM)
                        .child(crate::format::thousands(pages.len() as u64)),
                )
                .child(tr!("print-preview-of-pages", total = *total)),
            _ => div().child(count),
        };
        let header = div()
            .flex_none()
            .px(px(24.0))
            .pt(px(20.0))
            .pb(px(16.0))
            .flex()
            .flex_row()
            .items_end()
            .gap(px(16.0))
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
                            .line_height(px(28.0))
                            .child(tr!("print-preview-title")),
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(rgba(th.text_dim))
                            .truncate()
                            .child(preview.job.subject.clone()),
                    ),
            )
            .child(
                div()
                    .flex_none()
                    .text_size(px(13.0))
                    .text_color(rgba(th.text_faint))
                    .child(count),
            );
        let body = div()
            .id("print-preview-pages")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .bg(rgba(th.backdrop))
            .border_t_1()
            .border_b_1()
            .border_color(rgba(th.divider))
            .child(
                div()
                    .flex_none()
                    .w_full()
                    .py(px(24.0))
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(16.0))
                    .children(pages),
            );
        let options = preview.options;
        let formatted = preview.job.formatted();
        let chip = |id: &'static str, label: String, on: bool| {
            div()
                .id(id)
                .h(px(32.0))
                .px(px(14.0))
                .flex()
                .items_center()
                .rounded_full()
                .border_1()
                .border_color(rgba(if on { th.accent } else { th.outline }))
                .when(on, |d| d.bg(rgba(fade(th.accent, 0.12))))
                .text_size(px(13.0))
                .font_weight(if on {
                    FontWeight::MEDIUM
                } else {
                    FontWeight::NORMAL
                })
                .text_color(rgba(if on { th.accent } else { th.text_dim }))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .child(label)
        };
        let caption = |text: String| {
            div()
                .text_size(px(13.0))
                .text_color(rgba(th.text_dim))
                .pr(px(4.0))
                .child(text)
        };
        // The first caption of each row, as wide in both, so the chips after
        // them line up.
        let lead = |text: String| caption(text).min_w(px(space::S8));
        let paper_choice = |id: &'static str, label: String, choice: Paper| {
            chip(id, label, paper == choice).on_click(cx.listener(move |this, _, window, cx| {
                this.set_print_setup(choice, options, window, cx)
            }))
        };
        let layout_choice = |id: &'static str, label: String, simple: bool| {
            chip(id, label, options.simple == simple).on_click(cx.listener(
                move |this, _, window, cx| {
                    this.set_print_setup(paper, PrintOptions { simple, ..options }, window, cx)
                },
            ))
        };
        // Simple text has no backgrounds to leave out.
        let backgrounds_on = options.backgrounds && !options.simple;
        let backgrounds = div()
            .id("print-backgrounds")
            .h(px(32.0))
            .px(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(10.0))
            .rounded_full()
            .text_size(px(13.0))
            .text_color(rgba(th.text_dim))
            .when(options.simple, |d| d.opacity(0.5))
            .when(!options.simple, |d| {
                d.cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        let options = PrintOptions {
                            backgrounds: !options.backgrounds,
                            ..options
                        };
                        this.set_print_setup(paper, options, window, cx)
                    }))
            })
            .child(switch(if backgrounds_on { 1.0 } else { 0.0 }, th))
            .child(tr!("print-preview-backgrounds"));
        let wrong = match &chosen {
            Some((Err(err), total)) => Some(match err {
                PagesError::Beyond => tr!("print-preview-pages-beyond", count = *total),
                PagesError::Unreadable => tr!("print-preview-pages-unreadable"),
            }),
            _ => None,
        };
        // Red while it names pages that can't print.
        let field_th = Theme {
            accent: if wrong.is_some() { th.error } else { th.accent },
            outline: if wrong.is_some() {
                th.error
            } else {
                th.outline
            },
            ..*th
        };
        let range_focus = gpui::Focusable::focus_handle(preview.range.read(cx), cx);
        let pages_row = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(space::S3))
            .child(lead(tr!("print-preview-page-choice")))
            .child(
                chip("print-all-pages", tr!("print-preview-all-pages"), all)
                    .on_click(cx.listener(|this, _, _, cx| this.print_all_pages(cx))),
            )
            .child(
                field("print-pages-field", &range_focus, &field_th)
                    .w(px(PAGES_FIELD_WIDTH))
                    .h(px(CHIP_HEIGHT))
                    .text_size(px(text::SMALL))
                    .flex()
                    .items_center()
                    .child(div().flex_1().min_w_0().child(preview.range.clone())),
            )
            .children(wrong.map(|wrong| {
                div()
                    .text_size(px(text::CAPTION))
                    .text_color(rgba(th.error))
                    .child(wrong)
            }));
        let settings = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(8.0))
            .child(lead(tr!("print-preview-paper")))
            .child(paper_choice("print-a4", tr!("print-preview-a4"), Paper::A4))
            .child(paper_choice(
                "print-letter",
                tr!("print-preview-letter"),
                Paper::LETTER,
            ))
            .when(formatted, |d| {
                d.child(div().w(px(12.0)))
                    .child(caption(tr!("print-preview-layout")))
                    .child(layout_choice(
                        "print-as-shown",
                        tr!("print-preview-as-shown"),
                        false,
                    ))
                    .child(layout_choice(
                        "print-simple",
                        tr!("print-preview-simple"),
                        true,
                    ))
                    .child(div().w(px(12.0)))
                    .child(backgrounds)
            });
        let footer = div()
            .flex_none()
            .px(px(24.0))
            .py(px(16.0))
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(pages_row)
            .child(settings)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_end()
                    .gap(px(8.0))
                    .child(
                        outlined_button("print-cancel", tr!("print-preview-cancel"), th)
                            .focus_ring(th)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.close_print_preview(window, cx)
                            })),
                    )
                    .child(
                        filled_button(
                            "print-go",
                            match &chosen {
                                Some((Ok(pages), total)) if pages.len() < *total => {
                                    tr!("print-preview-print-pages", count = pages.len())
                                }
                                _ => tr!("print-preview-print"),
                            },
                            th,
                        )
                        .focus_ring_filled(th)
                        .when(!ready, |d| d.opacity(0.5).cursor_default())
                        .on_click(
                            cx.listener(|this, _, window, cx| this.print_previewed(window, cx)),
                        ),
                    ),
            );
        let card = div()
            .id("print-preview")
            .track_focus(&focus)
            .map(|d| super::popovers::keep_tab_inside(d, &focus))
            .on_key_down(cx.listener(Self::print_preview_key))
            .occlude()
            .w(px(width))
            .h_full()
            .when(!phone, |d| d.max_h(px(920.0)))
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
                .bg(rgba(fade(0x0000_0066, t)))
                .child(
                    div()
                        .id("print-preview-scrim")
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .on_click(
                            cx.listener(|this, _, window, cx| this.close_print_preview(window, cx)),
                        ),
                )
                .child(
                    div()
                        .h_full()
                        .flex()
                        .flex_col()
                        .justify_center()
                        .opacity(t)
                        .mt(px(lerp(24.0, 0.0, t)))
                        .child(card),
                )
                .into_any_element(),
        )
    }
}

/// How much a page left out shows.
const LEFT_OUT: f32 = state::DISABLED;

impl PrintPreview {
    /// The pages to print of `total`, from what is typed.
    fn chosen(&self, total: usize, cx: &gpui::App) -> Result<Vec<usize>, PagesError> {
        parse_pages(self.range.read(cx).text(), total)
    }

    /// Every page bitmap it holds.
    fn images(self) -> Vec<Arc<RenderImage>> {
        let mut images = self.released;
        if let Pages::Ready { images: pages, .. } = self.pages {
            images.extend(pages);
        }
        images
    }
}
