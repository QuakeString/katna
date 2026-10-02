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
use katna_search::contacts::Suggestion;
use katna_ui::px;
use katna_ui::text_input::{Cancel, Down, Submit, Up};
use katna_ui::{InputEvent, TextInput};

use super::compose::address_suggestions;
use super::{FocusNext, MailWindow};
use crate::theme::Theme;
use crate::widgets::{filled_button, icon, icon_button, raised, tip};
use dates::{CustomDates, DateError};

/// Width of a field's label.
const LABEL: f32 = 120.0;
/// The narrowest the panel gets while the window has room.
pub(super) const MIN_WIDTH: f32 = 640.0;
/// Below this width labels go above their fields.
const STACK_BELOW: f32 = 600.0;

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

/// Attachment types offered after "Has attachment", each with the
/// extensions it stands for; the label is the first, in capitals. "ODF"
/// is an OpenDocument text file.
const TYPES: [&[&str]; 7] = [
    &["pdf"],
    &["xlsx"],
    &["odf", "odt"],
    &["xls"],
    &["ods"],
    &["ppt"],
    &["pptx"],
];

/// A typed extension as searched: lowercase letters and digits, the part
/// after the last dot. Empty when there is nothing of the sort.
fn extension(typed: &str) -> String {
    typed
        .trim()
        .rsplit('.')
        .next()
        .unwrap_or_default()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect::<String>()
        .to_ascii_lowercase()
}

/// The attachment types chosen after "Has attachment".
#[derive(Default)]
struct Types {
    /// Which of [`TYPES`] are chosen.
    chosen: [bool; TYPES.len()],
    /// Whether the field for other extensions is open.
    custom: bool,
    /// The extensions typed there, as pills.
    typed: Vec<String>,
}

impl Types {
    /// Every extension chosen, each once.
    fn extensions(&self) -> Vec<String> {
        let mut all: Vec<String> = TYPES
            .iter()
            .zip(self.chosen)
            .filter(|(_, on)| *on)
            .flat_map(|(exts, _)| exts.iter().map(|e| (*e).to_owned()))
            .collect();
        if self.custom {
            all.extend(self.typed.iter().cloned());
        }
        let mut seen = std::collections::HashSet::new();
        all.retain(|e| seen.insert(e.clone()));
        all
    }
}

/// `filename:pdf`, or `filename:(pdf OR xlsx)` for several.
fn filename_query(extensions: &[String]) -> String {
    match extensions {
        [] => String::new(),
        [one] => format!("filename:{one}"),
        many => format!("filename:({})", many.join(" OR ")),
    }
}

pub(super) struct SearchPanel {
    from: Entity<TextInput>,
    to: Entity<TextInput>,
    subject: Entity<TextInput>,
    words: Entity<TextInput>,
    without: Entity<TextInput>,
    within: usize,
    custom: CustomDates,
    attachment: bool,
    types: Types,
    /// Where other attachment extensions are typed.
    extension: Entity<TextInput>,
    /// Addresses suggested under From or To.
    suggest: Option<Suggestions>,
    _subscriptions: Vec<Subscription>,
}

/// A field that suggests addresses from the address book.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Person {
    From,
    To,
}

/// The addresses suggested under a field.
struct Suggestions {
    field: Person,
    items: Vec<Suggestion>,
    selected: usize,
}

impl SearchPanel {
    fn person(&self, field: Person) -> &Entity<TextInput> {
        match field {
            Person::From => &self.from,
            Person::To => &self.to,
        }
    }

    fn suggesting(&self, field: Person) -> bool {
        self.suggest.as_ref().is_some_and(|s| s.field == field)
    }

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
            &filename_query(&self.extensions(cx)),
            quote,
        ))
    }

    /// The extensions chosen, with one still being typed.
    fn extensions(&self, cx: &gpui::App) -> Vec<String> {
        let mut all = self.types.extensions();
        let typing = extension(self.extension.read(cx).text());
        if self.types.custom && !typing.is_empty() && !all.contains(&typing) {
            all.push(typing);
        }
        all
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
    filenames: &str,
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
    if !filenames.is_empty() {
        parts.push(filenames.to_owned());
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
        match &mut self.search_panel {
            Some(panel) if panel.suggest.is_some() => panel.suggest = None,
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
        let extension = cx.new(|cx| TextInput::new(tr!("search-attachment-custom-hint"), cx));
        let fields = [
            (&from, Some(Person::From)),
            (&to, Some(Person::To)),
            (&subject, None),
            (&words, None),
            (&without, None),
        ];
        let mut subscriptions: Vec<Subscription> = fields
            .into_iter()
            .map(|(input, person)| {
                cx.subscribe_in(
                    input,
                    window,
                    move |this, _, event: &InputEvent, window, cx| match event {
                        InputEvent::Submit => this.run_search_panel(window, cx),
                        InputEvent::Cancel => {
                            this.search_panel = None;
                            cx.notify();
                        }
                        InputEvent::Changed => {
                            if let Some(person) = person {
                                this.person_changed(person, cx);
                            }
                        }
                    },
                )
            })
            .collect();
        self.load_address_book(cx);
        // Space or Enter makes a pill of a typed extension; Enter with
        // nothing typed searches.
        subscriptions.push(cx.subscribe_in(
            &extension,
            window,
            |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Submit => {
                    if !this.add_extension(cx) {
                        this.run_search_panel(window, cx);
                    }
                }
                InputEvent::Cancel => {
                    this.search_panel = None;
                    cx.notify();
                }
                InputEvent::Changed => {
                    let spaced = this.search_panel.as_ref().is_some_and(|panel| {
                        panel
                            .extension
                            .read(cx)
                            .text()
                            .contains(char::is_whitespace)
                    });
                    if spaced {
                        this.add_extension(cx);
                    }
                }
            },
        ));
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
            types: Types::default(),
            extension,
            suggest: None,
            _subscriptions: subscriptions,
        });
        cx.notify();
    }

    /// Suggests addresses for what is typed in From or To.
    fn person_changed(&mut self, field: Person, cx: &mut Context<Self>) {
        let Some(panel) = &self.search_panel else {
            return;
        };
        let typed = panel.person(field).read(cx).text().trim().to_owned();
        let mut items = address_suggestions(&typed, None, &[], cx);
        // A picked address is not suggested again.
        if items.iter().any(|i| i.email.eq_ignore_ascii_case(&typed)) {
            items.clear();
        }
        if let Some(panel) = &mut self.search_panel {
            panel.suggest = (!items.is_empty()).then_some(Suggestions {
                field,
                items,
                selected: 0,
            });
        }
        cx.notify();
    }

    /// Puts suggested address `ix` in the field.
    fn pick_person(&mut self, ix: usize, cx: &mut Context<Self>) {
        let Some(panel) = &mut self.search_panel else {
            return;
        };
        let Some(suggest) = panel.suggest.take() else {
            return;
        };
        let Some(item) = suggest.items.into_iter().nth(ix) else {
            return;
        };
        panel
            .person(suggest.field)
            .clone()
            .update(cx, |input, cx| input.set_text(item.email, cx));
        cx.notify();
    }

    fn close_person_suggestions(&mut self, cx: &mut Context<Self>) {
        if let Some(panel) = &mut self.search_panel
            && panel.suggest.take().is_some()
        {
            cx.notify();
        }
    }

    fn move_person_suggestion(&mut self, by: isize, cx: &mut Context<Self>) {
        if let Some(suggest) = self.search_panel.as_mut().and_then(|p| p.suggest.as_mut()) {
            let len = suggest.items.len() as isize;
            suggest.selected = (suggest.selected as isize + by).rem_euclid(len) as usize;
            cx.notify();
        }
    }

    /// Gives From or To the keys for its suggestions, and draws them
    /// under it: Up and Down move, Enter or Tab picks, Escape closes.
    fn person_row(
        &self,
        row: Div,
        field: Person,
        stacked: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        let open = move |this: &Self| {
            this.search_panel
                .as_ref()
                .is_some_and(|panel| panel.suggesting(field))
        };
        let pick = move |this: &mut Self, cx: &mut Context<Self>| {
            if open(this) {
                cx.stop_propagation();
                let ix = this
                    .search_panel
                    .as_ref()
                    .and_then(|p| p.suggest.as_ref())
                    .map_or(0, |s| s.selected);
                this.pick_person(ix, cx);
            }
        };
        let list = self
            .search_panel
            .as_ref()
            .and_then(|panel| panel.suggest.as_ref())
            .filter(|s| s.field == field)
            .map(|s| {
                self.suggestion_list(
                    &s.items,
                    s.selected,
                    // Under the field, past its label unless it is above.
                    px(if stacked { 0.0 } else { LABEL + 16.0 }),
                    Self::pick_person,
                    Self::close_person_suggestions,
                    th,
                    cx,
                )
            });
        row.relative()
            .capture_action(cx.listener(move |this, _: &Submit, _, cx| pick(this, cx)))
            .capture_action(cx.listener(move |this, _: &FocusNext, _, cx| pick(this, cx)))
            .capture_action(cx.listener(move |this, _: &Cancel, _, cx| {
                if open(this) {
                    cx.stop_propagation();
                    this.close_person_suggestions(cx);
                }
            }))
            .on_action(cx.listener(move |this, _: &Up, _, cx| {
                if open(this) {
                    this.move_person_suggestion(-1, cx);
                } else {
                    cx.propagate();
                }
            }))
            .on_action(cx.listener(move |this, _: &Down, _, cx| {
                if open(this) {
                    this.move_person_suggestion(1, cx);
                } else {
                    cx.propagate();
                }
            }))
            .children(list)
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

    /// Makes pills of the extensions typed in the Custom field. Whether
    /// there was one.
    fn add_extension(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(panel) = &mut self.search_panel else {
            return false;
        };
        let text = panel.extension.read(cx).text().to_owned();
        let typed: Vec<String> = text
            .split(|c: char| c.is_whitespace() || c == ',')
            .map(extension)
            .filter(|e| !e.is_empty())
            .collect();
        for ext in &typed {
            if !panel.types.typed.contains(ext) {
                panel.types.typed.push(ext.clone());
            }
        }
        if !typed.is_empty() {
            panel.attachment = true;
        }
        if !text.is_empty() {
            panel
                .extension
                .update(cx, |input, cx| input.set_text(String::new(), cx));
        }
        cx.notify();
        !typed.is_empty()
    }

    /// Backspace in the empty Custom field takes out the last pill.
    fn extension_backspace(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(panel) = &mut self.search_panel else {
            return false;
        };
        if !panel.extension.read(cx).text().is_empty() || panel.types.typed.pop().is_none() {
            return false;
        }
        cx.notify();
        true
    }

    /// The file-type chips after "Has attachment", and the Custom field.
    fn render_attachment_types(
        &self,
        stacked: bool,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let panel = self.search_panel.as_ref()?;
        let types = &panel.types;
        let mut chips: Vec<AnyElement> = TYPES
            .iter()
            .enumerate()
            .map(|(ix, exts)| {
                chip(
                    ("attachment-type", ix),
                    &exts[0].to_uppercase(),
                    types.chosen[ix],
                    th,
                )
                .on_click(cx.listener(move |this, _, _, cx| {
                    if let Some(panel) = &mut this.search_panel {
                        let on = !panel.types.chosen[ix];
                        panel.types.chosen[ix] = on;
                        panel.attachment |= on;
                    }
                    cx.notify();
                }))
                .into_any_element()
            })
            .collect();
        chips.push(
            chip(
                ("attachment-type", TYPES.len()),
                &tr!("search-attachment-custom"),
                types.custom,
                th,
            )
            .on_click(cx.listener(|this, _, window, cx| {
                if let Some(panel) = &mut this.search_panel {
                    panel.types.custom = !panel.types.custom;
                    if panel.types.custom {
                        panel.attachment |= !panel.types.typed.is_empty();
                        window.focus(&panel.extension.focus_handle(cx), cx);
                    }
                }
                cx.notify();
            }))
            .into_any_element(),
        );
        let pills: Vec<AnyElement> = types
            .typed
            .iter()
            .enumerate()
            .map(|(ix, ext)| {
                div()
                    .id(("attachment-extension", ix))
                    .flex_none()
                    .h(px(24.0))
                    .pl(px(10.0))
                    .pr(px(2.0))
                    .flex()
                    .items_center()
                    .gap(px(2.0))
                    .rounded_full()
                    .bg(rgba(th.nav_selected))
                    .text_color(rgba(th.nav_selected_text))
                    .text_size(px(13.0))
                    .child(ext.clone())
                    .child(
                        div()
                            .id(("attachment-extension-remove", ix))
                            .size(px(20.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_full()
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .tooltip(tip(tr!("search-attachment-remove"), th))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if let Some(panel) = &mut this.search_panel
                                    && ix < panel.types.typed.len()
                                {
                                    panel.types.typed.remove(ix);
                                }
                                cx.notify();
                            }))
                            .child(icon("close", th.nav_selected_text, 14.0)),
                    )
                    .into_any_element()
            })
            .collect();
        let field = types.custom.then(|| {
            div()
                .w_full()
                .min_h(px(36.0))
                .py(px(4.0))
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(px(6.0))
                .border_b_1()
                .border_color(rgba(th.divider))
                .text_size(px(14.0))
                .capture_action(
                    cx.listener(|this, _: &katna_ui::text_input::Backspace, _, cx| {
                        if this.extension_backspace(cx) {
                            cx.stop_propagation();
                        }
                    }),
                )
                .children(pills)
                .child(
                    div()
                        .flex_1()
                        .min_w(px(160.0))
                        .child(panel.extension.clone()),
                )
        });
        Some(
            div()
                .when(stacked, |d| d.w_full())
                .when(!stacked, |d| d.flex_1().min_w_0())
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap(px(6.0))
                        .children(chips),
                )
                .children(field)
                .into_any_element(),
        )
    }

    /// The panel, `left` in from the window's start and `width` wide,
    /// at most `height` tall (it scrolls beyond); in a narrow window labels
    /// go above their fields.
    pub(super) fn render_search_panel(
        &mut self,
        th: &Theme,
        left: f32,
        width: f32,
        height: f32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let t = self.search_panel_spring.value().clamp(0.0, 1.0);
        let panel = self.search_panel.as_ref()?;
        let stacked = width < STACK_BELOW;
        let caption = |text: String| {
            div()
                .when(!stacked, |d| d.w(px(LABEL)))
                .flex_none()
                .text_size(px(14.0))
                .text_color(rgba(th.text_dim))
                .child(text)
        };
        // A label and what it labels: side by side, or one above the other.
        let row = || {
            div()
                .flex()
                .when(stacked, |d| d.flex_col().gap(px(2.0)))
                .when(!stacked, |d| d.flex_row().gap(px(16.0)))
        };
        let field = |text: String, input: &Entity<TextInput>| {
            row()
                .when(!stacked, |d| d.items_center())
                .child(caption(text))
                .child(
                    div()
                        .when(stacked, |d| d.w_full())
                        .when(!stacked, |d| d.flex_1().min_w_0())
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
        let from = field(tr!("search-from"), &panel.from);
        let to = field(tr!("search-to"), &panel.to);
        let from = self.person_row(from, Person::From, stacked, th, cx);
        let to = self.person_row(to, Person::To, stacked, th, cx);
        let popover = self.render_custom_popover(th, window, cx);
        let attachment = panel.attachment;
        let types = self.render_attachment_types(stacked, th, cx);
        let body = div()
            .id("search-panel")
            .occlude()
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(|this, _, _, _| this.search_pressed = true),
            )
            .w(px(width))
            .max_h(px(height))
            .overflow_y_scroll()
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
            .child(from)
            .child(to)
            .child(field(tr!("search-subject"), &panel.subject))
            .child(field(tr!("search-has-words"), &panel.words))
            .child(field(tr!("search-without"), &panel.without))
            .child(
                row()
                    .pt(px(8.0))
                    .when(stacked, |d| d.gap(px(8.0)))
                    .items_start()
                    .child(caption(tr!("search-date-within")).when(!stacked, |d| d.pt(px(4.0))))
                    .child(
                        div()
                            .when(stacked, |d| d.w_full())
                            .when(!stacked, |d| d.flex_1())
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .gap(px(6.0))
                            .children(chips),
                    ),
            )
            .child(
                row()
                    .pt(px(8.0))
                    .when(stacked, |d| d.gap(px(8.0)))
                    .items_start()
                    .child(
                        div()
                            .id("has-attachment")
                            .flex_none()
                            .h(px(28.0))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(12.0))
                            .cursor_pointer()
                            .text_size(px(14.0))
                            .on_click(cx.listener(|this, _, _, cx| {
                                if let Some(panel) = &mut this.search_panel {
                                    panel.attachment = !panel.attachment;
                                    // Types need an attachment to be of.
                                    if !panel.attachment {
                                        panel.types = Types::default();
                                    }
                                }
                                cx.notify();
                            }))
                            .child(crate::widgets::checkbox(
                                "search-attachment-box",
                                crate::widgets::Check::from(attachment),
                                th,
                            ))
                            .child(tr!("search-has-attachment")),
                    )
                    .children(types),
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
        .border_color(rgba(if on { th.nav_selected } else { th.outline }))
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
                "",
                quote
            ),
            "from:kay subject:\"gas deal\" price -draft -old newer_than:1w has:attachment"
        );
        assert_eq!(build_query("", "", "", "", "", "", false, "", quote), "");
    }

    #[test]
    fn asks_for_attachment_types() {
        let mut types = Types::default();
        assert_eq!(filename_query(&types.extensions()), "");
        types.chosen[0] = true;
        assert_eq!(filename_query(&types.extensions()), "filename:pdf");
        types.chosen[2] = true;
        types.custom = true;
        types.typed = vec!["png".into(), "pdf".into()];
        assert_eq!(
            filename_query(&types.extensions()),
            "filename:(pdf OR odf OR odt OR png)"
        );
        // Typed pills count only while Custom is on.
        types.custom = false;
        assert_eq!(
            filename_query(&types.extensions()),
            "filename:(pdf OR odf OR odt)"
        );
        assert_eq!(extension(" .AVIF "), "avif");
        assert_eq!(extension("tar.gz"), "gz");
        assert_eq!(extension("..."), "");
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
