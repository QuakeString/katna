// SPDX-License-Identifier: GPL-3.0-or-later

//! Search options: the panel the tune button in the search box opens, with
//! fields that build a search query (`from:`, `subject:`, `newer_than:`,
//! ...), as webmail's advanced search does. "Custom" dates open a popover
//! from their chip (`dates`).

mod dates;

use gpui::{
    AnyElement, Context, Div, Entity, Focusable, FontWeight, Stateful, Subscription, Window,
    canvas, div, prelude::*, rgba,
};
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::{InputEvent, TextInput};

use super::MailWindow;
use crate::theme::Theme;
use crate::widgets::{filled_button, icon, icon_button, raised, tip};
use dates::{CustomDates, DateError};

/// "Date within" choices: `newer_than:` values, labeled by
/// [`within_label`].
const WITHIN: [&str; 8] = ["", "1d", "3d", "1w", "2w", "1m", "6m", "1y"];

/// What a "Date within" chip says: "Any time", "3 days".
fn within_label(age: &str) -> String {
    let (count, unit) = age.split_at(age.len().saturating_sub(1));
    let count: u32 = count.parse().unwrap_or(0);
    match unit {
        "d" => tr!("search-within-days", count = count),
        "w" => tr!("search-within-weeks", count = count),
        "m" => tr!("search-within-months", count = count),
        "y" => tr!("search-within-years", count = count),
        _ => tr!("search-within-any"),
    }
}

/// The "Custom" chip, after [`WITHIN`].
const CUSTOM: usize = WITHIN.len();

pub(super) struct SearchPanel {
    from: Entity<TextInput>,
    to: Entity<TextInput>,
    subject: Entity<TextInput>,
    words: Entity<TextInput>,
    without: Entity<TextInput>,
    within: usize,
    custom: CustomDates,
    attachment: bool,
    _subscriptions: Vec<Subscription>,
}

impl SearchPanel {
    /// The search query the fields make, or why the custom dates are wrong.
    fn query(&self, cx: &gpui::App) -> Result<String, DateError> {
        let text = |input: &Entity<TextInput>| input.read(cx).text().trim().to_owned();
        let quote = |value: &str| {
            if value.contains(char::is_whitespace) {
                format!("\"{value}\"")
            } else {
                value.to_owned()
            }
        };
        let dates = match WITHIN.get(self.within) {
            Some(&"") => String::new(),
            Some(age) => format!("newer_than:{age}"),
            None => self.custom.query(cx)?,
        };
        Ok(build_query(
            &text(&self.from),
            &text(&self.to),
            &text(&self.subject),
            &text(&self.words),
            &text(&self.without),
            &dates,
            self.attachment,
            quote,
        ))
    }
}

#[allow(clippy::too_many_arguments)]
fn build_query(
    from: &str,
    to: &str,
    subject: &str,
    words: &str,
    without: &str,
    dates: &str,
    attachment: bool,
    quote: impl Fn(&str) -> String,
) -> String {
    let mut parts = Vec::new();
    if !from.is_empty() {
        parts.push(format!("from:{}", quote(from)));
    }
    if !to.is_empty() {
        parts.push(format!("to:{}", quote(to)));
    }
    if !subject.is_empty() {
        parts.push(format!("subject:{}", quote(subject)));
    }
    if !words.is_empty() {
        parts.push(words.to_owned());
    }
    parts.extend(without.split_whitespace().map(|w| format!("-{w}")));
    if !dates.is_empty() {
        parts.push(dates.to_owned());
    }
    if attachment {
        parts.push("has:attachment".to_owned());
    }
    parts.join(" ")
}

impl MailWindow {
    /// Escape: closes the custom dates' popover if it is open, else the
    /// panel.
    pub(super) fn dismiss_search_panel(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        match &self.search_panel {
            Some(panel) if panel.custom.open => self.custom_cancel(window, cx),
            Some(_) => self.search_panel = None,
            None => return false,
        }
        true
    }

    pub(super) fn toggle_search_panel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.search_panel.take().is_some() {
            cx.notify();
            return;
        }
        self.settings_open = false;
        let input = |cx: &mut Context<Self>| cx.new(|cx| TextInput::new("", cx));
        let (from, to, subject, words, without) =
            (input(cx), input(cx), input(cx), input(cx), input(cx));
        let custom = CustomDates::new(cx);
        let mut subscriptions: Vec<Subscription> =
            [&from, &to, &subject, &words, &without]
                .into_iter()
                .map(|input| {
                    cx.subscribe_in(input, window, |this, _, event: &InputEvent, window, cx| {
                        match event {
                            InputEvent::Submit => this.run_search_panel(window, cx),
                            InputEvent::Cancel => {
                                this.search_panel = None;
                                cx.notify();
                            }
                            InputEvent::Changed => {}
                        }
                    })
                })
                .collect();
        // In the custom dates' popover, Enter is Done and Escape Cancel.
        subscriptions.extend(custom.dates.iter().map(|input| {
            cx.subscribe_in(
                input,
                window,
                |this, _, event: &InputEvent, window, cx| match event {
                    InputEvent::Submit => this.custom_done(window, cx),
                    InputEvent::Cancel => this.custom_cancel(window, cx),
                    InputEvent::Changed => {
                        if let Some(panel) = &mut this.search_panel {
                            panel.custom.error = None;
                        }
                    }
                },
            )
        }));
        window.focus(&from.focus_handle(cx), cx);
        self.search_panel = Some(SearchPanel {
            from,
            to,
            subject,
            words,
            without,
            within: 0,
            custom,
            attachment: false,
            _subscriptions: subscriptions,
        });
        cx.notify();
    }

    fn run_search_panel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(panel) = &mut self.search_panel else {
            return;
        };
        let query = match panel.query(cx) {
            Ok(query) => query,
            Err(error) => {
                self.open_custom_dates(window, cx);
                if let Some(panel) = &mut self.search_panel {
                    panel.custom.error = Some(error);
                }
                return;
            }
        };
        self.search_panel = None;
        if query.is_empty() {
            cx.notify();
            return;
        }
        self.search_for(query, window, cx);
        self.focus_list(&super::FocusList, window, cx);
    }

    pub(super) fn render_search_panel(
        &mut self,
        th: &Theme,
        left: f32,
        width: f32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let t = self.search_panel_spring.value().clamp(0.0, 1.0);
        let panel = self.search_panel.as_ref()?;
        let field = |label: String, input: &Entity<TextInput>| {
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .child(
                    div()
                        .w(px(120.0))
                        .flex_none()
                        .text_size(px(14.0))
                        .text_color(rgba(th.text_dim))
                        .child(label),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .h(px(36.0))
                        .flex()
                        .items_center()
                        .border_b_1()
                        .border_color(rgba(th.divider))
                        .text_size(px(14.0))
                        .child(input.clone()),
                )
        };
        let within = panel.within;
        let mut chips: Vec<AnyElement> = WITHIN
            .iter()
            .enumerate()
            .map(|(ix, age)| {
                chip(("within", ix), &within_label(age), ix == within, th)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if let Some(panel) = &mut this.search_panel {
                            panel.within = ix;
                        }
                        cx.notify();
                    }))
                    .into_any_element()
            })
            .collect();
        // "Custom", or the dates once picked; it opens the popover, which
        // needs to know where it is.
        let label = (within == CUSTOM)
            .then(|| panel.custom.label(cx))
            .flatten()
            .unwrap_or_else(|| tr!("search-within-custom"));
        let chip_bounds = panel.custom.chip.clone();
        chips.push(
            div()
                .relative()
                .child(
                    chip(("within", CUSTOM), &label, within == CUSTOM, th).on_click(
                        cx.listener(|this, _, window, cx| this.open_custom_dates(window, cx)),
                    ),
                )
                .child(
                    canvas(
                        move |bounds, _, _| chip_bounds.set(Some(bounds)),
                        |_, _, _, _| {},
                    )
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full(),
                )
                .into_any_element(),
        );
        let popover = self.render_custom_popover(th, window, cx);
        let attachment = panel.attachment;
        let body = div()
            .id("search-panel")
            .occlude()
            .w(px(width))
            .p(px(24.0))
            .pt(px(12.0))
            .flex()
            .flex_col()
            .gap(px(8.0))
            .map(|d| raised(d, th, 15.0, 3.0))
            .text_color(rgba(th.text))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::MEDIUM)
                            .child(tr!("search-options")),
                    )
                    .child(
                        // Out into the side padding, so the button sits as
                        // far from the right edge as from the top.
                        icon_button("search-panel-close", "close", 20.0, th)
                            .mr(px(-12.0))
                            .tooltip(tip(tr!("search-options-close"), th))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.search_panel = None;
                                cx.notify();
                            })),
                    ),
            )
            .child(field(tr!("search-from"), &panel.from))
            .child(field(tr!("search-to"), &panel.to))
            .child(field(tr!("search-subject"), &panel.subject))
            .child(field(tr!("search-has-words"), &panel.words))
            .child(field(tr!("search-without"), &panel.without))
            .child(
                div()
                    .pt(px(8.0))
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap(px(16.0))
                    .child(
                        div()
                            .w(px(120.0))
                            .pt(px(4.0))
                            .flex_none()
                            .text_size(px(14.0))
                            .text_color(rgba(th.text_dim))
                            .child(tr!("search-date-within")),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .gap(px(6.0))
                            .children(chips),
                    ),
            )
            .child(
                div()
                    .id("has-attachment")
                    .pt(px(8.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .cursor_pointer()
                    .text_size(px(14.0))
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(panel) = &mut this.search_panel {
                            panel.attachment = !panel.attachment;
                        }
                        cx.notify();
                    }))
                    .child(if attachment {
                        icon("checkbox-checked", th.accent, 20.0)
                    } else {
                        icon("checkbox", th.text_dim, 20.0)
                    })
                    .child(tr!("search-has-attachment")),
            )
            .child(
                div()
                    .pt(px(16.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_end()
                    .gap(px(16.0))
                    .child(
                        div()
                            .id("search-panel-clear")
                            .px(px(12.0))
                            .h(px(36.0))
                            .flex()
                            .items_center()
                            .rounded_full()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.accent))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.search_panel = None;
                                this.toggle_search_panel(window, cx);
                            }))
                            .child(tr!("search-clear-filter")),
                    )
                    .child(
                        filled_button("search-panel-go", tr!("search"), th).on_click(
                            cx.listener(|this, _, window, cx| this.run_search_panel(window, cx)),
                        ),
                    ),
            );
        Some(
            div()
                .absolute()
                .top(px(-4.0 + 8.0 * (1.0 - t)))
                .left(px(left.max(0.0)))
                .opacity(t)
                .child(body)
                .children(popover)
                .into_any_element(),
        )
    }
}

/// A chip in a row of choices.
pub(super) fn chip(
    id: impl Into<gpui::ElementId>,
    label: &str,
    on: bool,
    th: &Theme,
) -> Stateful<Div> {
    div()
        .id(id)
        .px(px(10.0))
        .h(px(28.0))
        .flex()
        .items_center()
        .rounded(px(8.0))
        .border_1()
        .border_color(rgba(if on { th.nav_selected } else { th.divider }))
        .bg(rgba(if on { th.nav_selected } else { th.surface }))
        .text_color(rgba(if on {
            th.nav_selected_text
        } else {
            th.text_dim
        }))
        .text_size(px(13.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .child(label.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_queries() {
        let quote = |v: &str| {
            if v.contains(' ') {
                format!("\"{v}\"")
            } else {
                v.to_owned()
            }
        };
        assert_eq!(
            build_query(
                "kay",
                "",
                "gas deal",
                "price",
                "draft old",
                "newer_than:1w",
                true,
                quote
            ),
            "from:kay subject:\"gas deal\" price -draft -old newer_than:1w has:attachment"
        );
        assert_eq!(build_query("", "", "", "", "", "", false, quote), "");
    }

    #[test]
    fn labels_within() {
        let labels: Vec<String> = WITHIN.iter().map(|age| within_label(age)).collect();
        assert_eq!(labels[0], "Any time");
        assert_eq!(labels[1], "1 day");
        assert_eq!(labels[2], "3 days");
        assert_eq!(labels[7], "1 year");
    }
}
