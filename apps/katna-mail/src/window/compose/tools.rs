// SPDX-License-Identifier: GPL-3.0-or-later

//! The bars of the compose window and their menus, as in webmail: Send
//! with schedule send; the formatting bar (Aa) with undo, font, size,
//! bold, italic, underline, colors, alignment, lists, indent, quote,
//! strikethrough, remove formatting and tables; attach, link, emoji,
//! photo, signature and more options; the right-click menu and the link
//! bubble.

use std::rc::Rc;

use gpui::{
    Anchor, AnyElement, Bounds, Context, DispatchPhase, Div, Entity, FocusHandle, Focusable,
    FontWeight, Hsla, MouseButton, MouseMoveEvent, Pixels, Point, SharedString, Stateful,
    Subscription, Window, anchored, canvas, deferred, div, point, prelude::*, rgba,
};
use jiff::civil::Date;
use katna_i18n::{format, tr};
use katna_ui::rich::{Align, Font, GrammarIssue, List, Pos, RichEditor, Size, TableEdit, html};
use katna_ui::{InputEvent, TextInput};
use katna_ui::{px, unpx};

use super::super::MailWindow;
use super::checks::{Passed, SendCheck};
use super::recipients::Field;
use super::{Mode, schedule};
use crate::theme::{Theme, fade, mix};
use crate::widgets::{filled_button, icon, icon_button, icon_button_colored, menu, menu_item, tip};

/// The open menu or dialog of the compose window.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::window) enum Popup {
    /// The arrow beside Send.
    Send,
    /// The accounts to send from, under the From row.
    From,
    /// Reply, reply to all or forward, under an inline reply's icon.
    Kind,
    /// Schedule send's suggested times.
    Schedule,
    /// Schedule send's date and time picker.
    PickTime,
    Font,
    Size,
    Colors,
    Align,
    /// Formatting that does not fit the bar.
    MoreFormat,
    /// The table size grid.
    Table,
    /// Rows and columns of the table the cursor is in.
    TableEdit,
    Emoji,
    Link,
    Signature,
    /// The templates to put in the message.
    Templates,
    /// The paperclip of the chat view's reply box: pictures, files, a
    /// template or another signature.
    ChatAttach,
    /// Asks the name to save the message under as a template.
    SaveTemplate,
    More,
    Label,
    /// Asks before plain text mode drops the formatting.
    PlainText,
    /// Send asks before a message goes out; the answers so far and how
    /// it was sending.
    SendCheck {
        check: SendCheck,
        at: Option<jiff::Timestamp>,
        archive: bool,
        passed: Passed,
    },
    /// Drive would not share the message's files with these recipients.
    DriveShare {
        refused: Vec<String>,
        at: Option<jiff::Timestamp>,
        archive: bool,
        passed: Passed,
    },
    /// Send found a recipient that is not an address.
    BadAddress {
        field: Field,
        address: String,
    },
    /// The right-click menu.
    Context {
        position: Point<Pixels>,
        misspelled: Option<String>,
        grammar: Option<GrammarIssue>,
    },
    /// The fixes of marked words in the text, on a rest or a left click;
    /// it closes when the pointer leaves the words and the card.
    Hint {
        word: Bounds<Pixels>,
        at: Pos,
        misspelled: Option<String>,
        suggestions: Vec<String>,
        grammar: Option<GrammarIssue>,
    },
    /// A right click on a grammar mistake in the subject, or with `word`, a
    /// rest or a left click on it (closing like [`Popup::Hint`]).
    SubjectGrammar {
        position: Point<Pixels>,
        issue: GrammarIssue,
        word: Option<Bounds<Pixels>>,
    },
}

impl Popup {
    /// A card of fixes that opened without a right click.
    pub(in crate::window) fn is_hint(&self) -> bool {
        matches!(
            self,
            Popup::Hint { .. } | Popup::SubjectGrammar { word: Some(_), .. }
        )
    }
}

/// A change to the subject field, from its grammar menu.
type SubjectEdit = dyn Fn(&mut TextInput, &mut Context<TextInput>);

/// Fields of the compose window's dialogs.
pub(in crate::window) struct Dialog {
    link_text: Entity<TextInput>,
    link_url: Entity<TextInput>,
    /// The link dialog changes a link that is there.
    editing_link: bool,
    pub(super) emoji_search: Entity<TextInput>,
    emoji_group: usize,
    time: Entity<TextInput>,
    /// The month the date picker shows.
    month: Date,
    day: Date,
    /// The table size under the pointer in the grid.
    grid: (usize, usize),
    /// The name to save the message under as a template.
    pub(super) template_name: Entity<TextInput>,
    /// Holds the keys while a dialog without a field is open, so Enter
    /// and Esc answer it rather than type into the message.
    focus: FocusHandle,
}

impl Dialog {
    pub fn new(accent: Hsla, cx: &mut Context<MailWindow>) -> Self {
        let input = |placeholder: String, cx: &mut Context<MailWindow>| {
            cx.new(|cx| {
                let mut input = TextInput::new(placeholder, cx);
                input.set_accent(accent);
                input
            })
        };
        let today = jiff::Zoned::now().date();
        Dialog {
            link_text: input(String::new(), cx),
            link_url: input("https://".to_owned(), cx),
            editing_link: false,
            emoji_search: input(tr!("compose-tool-emoji-search"), cx),
            emoji_group: 0,
            time: {
                let time = input(schedule::clock(jiff::civil::Time::constant(8, 0, 0, 0)), cx);
                time.update(cx, |t, _| t.set_stepper(Some(schedule::time_stepper())));
                time
            },
            month: today,
            day: today,
            grid: (0, 0),
            template_name: input(tr!("compose-tool-template-name"), cx),
            focus: cx.focus_handle(),
        }
    }

    /// What Enter and Escape do in the dialogs' fields.
    pub fn subscribe(
        &self,
        window: &mut Window,
        cx: &mut Context<MailWindow>,
    ) -> Vec<Subscription> {
        let mut subscriptions = Vec::new();
        for (field, submit) in [
            (&self.link_text, Submit::Link),
            (&self.link_url, Submit::Link),
            (&self.time, Submit::Time),
            (&self.emoji_search, Submit::Emoji),
            (&self.template_name, Submit::Template),
        ] {
            subscriptions.push(cx.subscribe_in(
                field,
                window,
                move |this, _, event: &InputEvent, window, cx| match event {
                    InputEvent::Submit => match submit {
                        Submit::Link => this.apply_link(window, cx),
                        Submit::Time => this.schedule_picked(window, cx),
                        Submit::Emoji => this.insert_first_emoji(window, cx),
                        Submit::Template => this.save_template(window, cx),
                    },
                    InputEvent::Cancel => this.close_popup(window, cx),
                    InputEvent::Changed => cx.notify(),
                },
            ));
        }
        subscriptions
    }
}

/// The widest a compose dialog gets.
const DIALOG_MAX_WIDTH: f32 = 600.0;
/// The corners of the compose dialogs.
const DIALOG_RADIUS: f32 = 16.0;

/// Whether `popup` is a dialog with a text field, which has the keys.
fn has_field(popup: &Popup) -> bool {
    matches!(popup, Popup::Link | Popup::PickTime | Popup::SaveTemplate)
}

#[derive(Clone, Copy)]
enum Submit {
    Link,
    Time,
    Emoji,
    Template,
}

/// Webmail's color palette: grays, bright colors, then six shades.
pub(super) const COLORS: [u32; 64] = [
    0x000000, 0x444444, 0x666666, 0x999999, 0xcccccc, 0xeeeeee, 0xf3f3f3, 0xffffff, //
    0xff0000, 0xff9900, 0xffff00, 0x00ff00, 0x00ffff, 0x0000ff, 0x9900ff, 0xff00ff, //
    0xf4cccc, 0xfce5cd, 0xfff2cc, 0xd9ead3, 0xd0e0e3, 0xcfe2f3, 0xd9d2e9, 0xead1dc, //
    0xea9999, 0xf9cb9c, 0xffe599, 0xb6d7a8, 0xa2c4c9, 0x9fc5e8, 0xb4a7d6, 0xd5a6bd, //
    0xe06666, 0xf6b26b, 0xffd966, 0x93c47d, 0x76a5af, 0x6fa8dc, 0x8e7cc3, 0xc27ba0, //
    0xcc0000, 0xe69138, 0xf1c232, 0x6aa84f, 0x45818e, 0x3d85c6, 0x674ea7, 0xa64d79, //
    0x990000, 0xb45f06, 0xbf9000, 0x38761d, 0x134f5c, 0x0b5394, 0x351c75, 0x741b47, //
    0x660000, 0x783f04, 0x7f6000, 0x274e13, 0x0c343d, 0x073763, 0x20124d, 0x4c1130,
];

/// The emoji groups of the picker, with the emoji on their tab.
const EMOJI_GROUPS: [(emojis::Group, &str); 9] = [
    (emojis::Group::SmileysAndEmotion, "😀"),
    (emojis::Group::PeopleAndBody, "👋"),
    (emojis::Group::AnimalsAndNature, "🐻"),
    (emojis::Group::FoodAndDrink, "🍔"),
    (emojis::Group::TravelAndPlaces, "✈️"),
    (emojis::Group::Activities, "⚽"),
    (emojis::Group::Objects, "💡"),
    (emojis::Group::Symbols, "❤️"),
    (emojis::Group::Flags, "🏁"),
];

/// The name of an emoji group, on its tab.
fn emoji_group_name(group: emojis::Group) -> String {
    use emojis::Group;
    match group {
        Group::SmileysAndEmotion => tr!("compose-tool-emoji-smileys"),
        Group::PeopleAndBody => tr!("compose-tool-emoji-people"),
        Group::AnimalsAndNature => tr!("compose-tool-emoji-animals"),
        Group::FoodAndDrink => tr!("compose-tool-emoji-food"),
        Group::TravelAndPlaces => tr!("compose-tool-emoji-travel"),
        Group::Activities => tr!("compose-tool-emoji-activities"),
        Group::Objects => tr!("compose-tool-emoji-objects"),
        Group::Symbols => tr!("compose-tool-emoji-symbols"),
        Group::Flags => tr!("compose-tool-emoji-flags"),
    }
}

/// Most emoji a search shows.
const EMOJI_RESULTS: usize = 160;
const TABLE_GRID: usize = 8;

/// The emoji matching `query` by name, or those of `group`.
fn emoji_list(query: &str, group: usize) -> Vec<&'static emojis::Emoji> {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        EMOJI_GROUPS[group.min(EMOJI_GROUPS.len() - 1)]
            .0
            .emojis()
            .collect()
    } else {
        emojis::iter()
            .filter(|e| {
                e.name().contains(&query) || e.shortcodes().any(|s| s.contains(query.as_str()))
            })
            .take(EMOJI_RESULTS)
            .collect()
    }
}

/// A web address as typed: `example.com` becomes `https://example.com` and
/// a bare email address a `mailto:` link.
pub(super) fn normalize_url(url: &str) -> String {
    let url = url.trim();
    let has_scheme = url.split_once(':').is_some_and(|(scheme, _)| {
        !scheme.is_empty()
            && scheme
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "+-.".contains(c))
    });
    if url.is_empty() || has_scheme {
        url.to_owned()
    } else if url.contains('@') && !url.contains('/') {
        format!("mailto:{url}")
    } else {
        format!("https://{url}")
    }
}

/// A small square button of the formatting bar, shaded when `active`.
pub(super) fn format_button(
    id: &'static str,
    name: &'static str,
    active: bool,
    th: &Theme,
) -> Stateful<gpui::Div> {
    div()
        .id(id)
        .flex_none()
        .size(px(28.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(4.0))
        .cursor_pointer()
        .when(active, |d| d.bg(rgba(format_active(th))))
        .hover(|s| s.bg(rgba(th.hover)))
        .child(icon(
            name,
            if active {
                th.nav_selected_text
            } else {
                th.text_dim
            },
            18.0,
        ))
}

/// The formatting bar's background: the card's color tinted with the
/// accent, so it stands apart from the text under it.
pub(super) fn format_bar_bg(th: &Theme) -> u32 {
    mix(th.surface, th.accent, if th.dark { 0.16 } else { 0.10 })
}

/// A format that is on (Bold, a list, the alignment): the selection color,
/// stronger than the bar's tint.
pub(super) fn format_active(th: &Theme) -> u32 {
    mix(th.surface, th.accent, if th.dark { 0.42 } else { 0.30 })
}

/// A dropdown of the formatting bar: its content and a small arrow.
pub(super) fn format_dropdown(id: &'static str, th: &Theme) -> Stateful<gpui::Div> {
    div()
        .id(id)
        .flex_none()
        .h(px(28.0))
        .pl(px(6.0))
        .pr(px(2.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(2.0))
        .rounded(px(4.0))
        .cursor_pointer()
        .text_size(px(13.0))
        .text_color(rgba(th.text_dim))
        .hover(|s| s.bg(rgba(th.hover)))
}

pub(super) fn separator(th: &Theme) -> gpui::Div {
    div()
        .flex_none()
        .mx(px(3.0))
        .w(px(1.0))
        .h(px(20.0))
        .bg(rgba(th.divider))
}

/// A popup above the element it belongs to (in a `relative` parent),
/// kept inside the window.
/// The width of the Send row taken by its padding, Send and the bin.
const ACTIONS_FIXED: f32 = 200.0;
/// The width of each of its other buttons.
const TOOL_WIDTH: f32 = 42.0;

/// Room between the floating formatting bar and the Send row.
const FORMAT_BAR_GAP: f32 = 4.0;
/// How much of the text the open formatting bar covers.
pub(super) const FORMAT_BAR_COVER: f32 = 40.0 + FORMAT_BAR_GAP + 8.0;

/// `popup` just under its parent's bottom left corner.
pub(super) fn below(popup: impl IntoElement) -> AnyElement {
    deferred(
        div().absolute().bottom_0().left_0().child(
            anchored()
                .offset(point(px(0.0), px(4.0)))
                .snap_to_window_with_margin(px(8.0))
                .child(div().occlude().child(popup)),
        ),
    )
    .with_priority(2)
    .into_any_element()
}

pub(super) fn above(popup: impl IntoElement) -> AnyElement {
    // Anchored to the parent's top left corner.
    deferred(
        div().absolute().top_0().left_0().child(
            anchored()
                .anchor(Anchor::BottomLeft)
                .offset(point(px(0.0), px(-6.0)))
                .snap_to_window_with_margin(px(8.0))
                .child(div().occlude().child(popup)),
        ),
    )
    .with_priority(2)
    .into_any_element()
}

pub(super) fn menu_divider(th: &Theme) -> gpui::Div {
    div().my(px(6.0)).h(px(1.0)).bg(rgba(th.divider))
}

/// A menu item with an icon, a label and a shortcut or check on the right.
fn tool_item(
    id: impl Into<gpui::ElementId>,
    name: &'static str,
    label: &str,
    th: &Theme,
) -> Stateful<gpui::Div> {
    div()
        .id(id)
        .h(px(36.0))
        .px(px(16.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(14.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgba(th.hover)))
        .child(icon(name, th.text_dim, 20.0))
        .child(div().flex_1().child(label.to_owned()))
}

fn shortcut(text: &'static str, th: &Theme) -> gpui::Div {
    div()
        .flex_none()
        .pl(px(16.0))
        .text_size(px(12.0))
        .text_color(rgba(th.text_faint))
        .child(text)
}

impl MailWindow {
    pub(super) fn toggle_popup(&mut self, popup: Popup, cx: &mut Context<Self>) {
        if let Some(c) = &mut self.compose {
            c.popup = if c.popup.as_ref() == Some(&popup) {
                None
            } else {
                Some(popup)
            };
        }
        cx.notify();
    }

    pub(super) fn close_popup(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(c) = &mut self.compose
            && c.popup.take().is_some()
        {
            window.focus(&c.body.focus_handle(cx), cx);
        }
        cx.notify();
    }

    /// Runs `f` on the editor, closes any menu and puts the focus back in
    /// the text.
    fn edit_body(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        f: impl FnOnce(&mut RichEditor, &mut Context<RichEditor>),
    ) {
        let Some(c) = &mut self.compose else {
            return;
        };
        c.popup = None;
        let body = c.body.clone();
        body.update(cx, f);
        window.focus(&body.focus_handle(cx), cx);
        cx.notify();
    }

    /// A listener that runs `f` on the editor.
    fn on_body<E>(
        &self,
        cx: &mut Context<Self>,
        f: impl Fn(&mut RichEditor, &mut Context<RichEditor>) + 'static,
    ) -> impl Fn(&E, &mut Window, &mut gpui::App) + 'static {
        let f = Rc::new(f);
        cx.listener(move |this, _: &E, window, cx| {
            let f = f.clone();
            this.edit_body(window, cx, move |editor, cx| f(editor, cx))
        })
    }

    // The bottom bar.

    pub(super) fn render_compose_actions(
        &self,
        th: &Theme,
        width: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let popup = compose.popup.clone();
        let plain = compose.plain(cx);
        let open = |p: Popup| popup.as_ref() == Some(&p);
        // A reply or forward archives its conversation too when that is
        // the default; the Send menu offers the other way.
        let archives = compose.answering.is_some() && self.config.sending.send_and_archive;
        let send = div()
            .relative()
            .flex_none()
            .h(px(36.0))
            .flex()
            .flex_row()
            .items_center()
            .rounded_full()
            .bg(rgba(th.accent))
            .text_color(rgba(th.on_accent))
            .text_size(px(14.0))
            .font_weight(FontWeight::MEDIUM)
            .child(
                div()
                    .id("compose-send")
                    .h_full()
                    .pl(px(20.0))
                    .pr(px(14.0))
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .rounded_l_full()
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(0xffffff1f)))
                    .tooltip(tip(
                        if archives {
                            tr!("compose-tool-send-archive-tip")
                        } else {
                            tr!("compose-tool-send-tip")
                        },
                        th,
                    ))
                    .on_click(
                        cx.listener(|this, _, window, cx| this.send_compose_default(window, cx)),
                    )
                    .when(archives, |d| d.child(icon("archive", th.on_accent, 18.0)))
                    .child(tr!("compose-tool-send")),
            )
            .child(div().w(px(1.0)).h(px(20.0)).bg(rgba(0xffffff66)))
            .child(
                div()
                    .id("compose-send-more")
                    .h_full()
                    .pl(px(6.0))
                    .pr(px(10.0))
                    .flex()
                    .items_center()
                    .rounded_r_full()
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(0xffffff1f)))
                    .tooltip(tip(tr!("compose-tool-send-more"), th))
                    .on_click(cx.listener(|this, _, _, cx| this.toggle_popup(Popup::Send, cx)))
                    .child(icon("drop-down", th.on_accent, 20.0)),
            )
            .when(open(Popup::Send), |d| {
                d.child(above(self.render_send_menu(th, cx)))
            })
            .when(open(Popup::Schedule), |d| {
                d.child(above(self.render_schedule_menu(th, cx)))
            });
        let tool = |id: &'static str, name: &'static str, label: String| {
            icon_button(id, name, 20.0, th).tooltip(tip(label, th))
        };
        let format = icon_button_colored(
            "compose-format",
            "format-text",
            20.0,
            if compose.format_bar {
                th.nav_selected_text
            } else {
                th.text_dim
            },
            th,
        )
        .when(compose.format_bar, |d| d.bg(rgba(format_active(th))))
        .tooltip(tip(tr!("compose-tool-formatting"), th))
        .on_click(cx.listener(|this, _, window, cx| {
            if let Some(c) = &mut this.compose {
                c.format_bar = !c.format_bar;
                c.popup = None;
                window.focus(&c.body.focus_handle(cx), cx);
            }
            cx.notify();
        }));
        let emoji = div()
            .relative()
            .child(
                tool("compose-emoji", "emoji", tr!("compose-tool-emoji")).on_click(cx.listener(
                    |this, _, window, cx| {
                        this.toggle_popup(Popup::Emoji, cx);
                        if let Some(c) = &this.compose
                            && c.popup == Some(Popup::Emoji)
                        {
                            let search = c.dialog.emoji_search.clone();
                            search.update(cx, |s, cx| s.set_text("", cx));
                            window.focus(&search.focus_handle(cx), cx);
                        }
                    },
                )),
            )
            .when(open(Popup::Emoji), |d| {
                d.child(above(self.render_emoji_picker(th, cx)))
            });
        let more = div()
            .relative()
            .child(
                tool("compose-more", "more", tr!("compose-tool-more"))
                    .on_click(cx.listener(|this, _, _, cx| this.toggle_popup(Popup::More, cx))),
            )
            .when(open(Popup::More) || open(Popup::Label), |d| {
                d.child(above(self.render_more_menu(th, cx)))
            });
        // A narrow row keeps the tools that fit beside Send and the bin,
        // dropping the calendar, photo, emoji and link buttons in turn;
        // links still come with Ctrl+K. Formatting, attaching, the
        // signature, templates and More always stay.
        let pill = if archives { 24.0 } else { 0.0 };
        let fit = ((width - ACTIONS_FIXED - pill) / TOOL_WIDTH).floor() as i32 - 5;
        let (link, emoji_fits, image, event) = (fit >= 1, fit >= 2, fit >= 3, fit >= 4);
        div()
            .flex_none()
            .h(px(60.0))
            .px(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0))
            .child(send)
            .child(div().w(px(8.0)))
            .child(format)
            .child(
                tool("compose-attach", "attachment", tr!("compose-tool-attach"))
                    .on_click(cx.listener(|this, _, _, cx| this.pick_files(false, cx))),
            )
            .when(link, |d| {
                d.child(
                    tool("compose-link", "link", tr!("compose-tool-link")).on_click(
                        cx.listener(|this, _, window, cx| this.open_link_dialog(window, cx)),
                    ),
                )
            })
            .when(emoji_fits, |d| d.child(emoji))
            .when(!plain && image, |d| {
                d.child(
                    tool("compose-image", "image", tr!("compose-tool-photo"))
                        .on_click(cx.listener(|this, _, _, cx| this.pick_files(true, cx))),
                )
            })
            .when(event, |d| {
                d.child(
                    tool("compose-event", "event", tr!("compose-tool-event")).on_click(
                        cx.listener(|this, _, _, cx| {
                            this.show_snackbar(tr!("compose-tool-event-coming"), None, cx)
                        }),
                    ),
                )
            })
            .child(self.render_signature_button(th, cx))
            .child(self.render_templates_button(th, cx))
            .child(more)
            .child(div().flex_1())
            .when(
                compose.mode == Mode::Window && self.writing.popout_server_frame,
                |d| {
                    d.child(
                        icon_button_colored("compose-dock", "close-full", 20.0, th.text_dim, th)
                            .tooltip(tip(tr!("compose-tool-dock"), th))
                            .on_click(cx.listener(|this, _, _, cx| this.dock_compose(cx))),
                    )
                },
            )
            .child(
                icon_button_colored("compose-discard", "trash", 20.0, th.text_dim, th)
                    .tooltip(tip(tr!("compose-tool-discard"), th))
                    .on_click(cx.listener(|this, _, _, cx| this.discard_compose(cx))),
            )
            .children(self.render_popup_scrim(cx))
            .children(self.render_context_popup(th, cx))
            .children(self.render_hint(th, cx))
            .children(self.render_rephrase_button(th, cx))
            .children(self.render_rephrase(th, cx))
            .children(self.render_subject_grammar(th, cx))
            .children(self.render_link_bubble(th, cx))
            .into_any_element()
    }

    /// Catches a click anywhere outside the open menu, closing it.
    pub(super) fn render_popup_scrim(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let popup = self.compose.as_ref()?.popup.as_ref()?;
        // The dialogs close with their own buttons; the cards of fixes when
        // the pointer leaves them, so the text under them still takes it.
        if popup.is_hint()
            || matches!(
                popup,
                Popup::Link
                    | Popup::PickTime
                    | Popup::PlainText
                    | Popup::SendCheck { .. }
                    | Popup::DriveShare { .. }
                    | Popup::BadAddress { .. }
            )
        {
            return None;
        }
        Some(
            deferred(
                anchored().position(point(px(0.0), px(0.0))).child(
                    div()
                        .id("compose-popup-scrim")
                        // Larger than any window.
                        .w(px(16384.0))
                        .h(px(16384.0))
                        .occlude()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, window, cx| this.close_popup(window, cx)),
                        )
                        .on_mouse_down(
                            MouseButton::Right,
                            cx.listener(|this, _, window, cx| this.close_popup(window, cx)),
                        ),
                ),
            )
            .with_priority(1)
            .into_any_element(),
        )
    }

    fn render_send_menu(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let count = self.writing.scheduled.len();
        let answering = self.compose.as_ref().is_some_and(|c| c.answering.is_some());
        let archives = self.config.sending.send_and_archive;
        menu(th)
            .w(px(240.0))
            .when(answering, |d| {
                let (name, label) = if archives {
                    ("send", tr!("compose-tool-send-without-archiving"))
                } else {
                    ("archive", tr!("compose-tool-send-and-archive"))
                };
                d.child(
                    tool_item("compose-send-other", name, &label, th).on_click(cx.listener(
                        move |this, _, window, cx| {
                            this.send_compose(None, !archives, Passed::default(), window, cx)
                        },
                    )),
                )
            })
            .child(
                tool_item("compose-schedule", "schedule", &tr!("schedule-send"), th).on_click(
                    cx.listener(|this, _, _, cx| {
                        if let Some(c) = &mut this.compose {
                            c.popup = Some(Popup::Schedule);
                        }
                        this.ask_hold_limit(cx);
                        cx.notify();
                    }),
                ),
            )
            .when(count > 0, |d| {
                d.child(
                    tool_item(
                        "compose-scheduled",
                        "send",
                        &tr!("schedule-scheduled-messages", count = count),
                        th,
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(c) = &mut this.compose {
                            c.popup = None;
                        }
                        this.open_scheduled(cx);
                    })),
                )
            })
            .child(menu_divider(th))
            .child(self.render_follow_up_choice(th, cx))
            .into_any_element()
    }

    /// "Remind me if no reply" in the send menu: when to bring the
    /// conversation back if nobody answers (`docs/ARCHITECTURE.md` §10.1).
    fn render_follow_up_choice(&self, th: &Theme, cx: &mut Context<Self>) -> gpui::Div {
        const DAY: u32 = 24 * 60 * 60;
        let chosen = self.compose.as_ref().map_or(0, |c| c.follow_up);
        let choices = [
            (0, tr!("follow-up-off")),
            (DAY, tr!("follow-up-days", days = 1)),
            (3 * DAY, tr!("follow-up-days", days = 3)),
            (7 * DAY, tr!("follow-up-days", days = 7)),
        ];
        div()
            .child(
                div()
                    .px(px(16.0))
                    .py(px(6.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(14.0))
                    .text_color(rgba(th.text_dim))
                    .child(icon("reply", th.text_dim, 20.0))
                    .child(tr!("follow-up-title")),
            )
            .children(choices.into_iter().map(|(after, label)| {
                div()
                    .id(("compose-follow-up", after))
                    .h(px(32.0))
                    .pl(px(50.0))
                    .pr(px(16.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .child(div().flex_1().child(label))
                    .when(after == chosen, |d| {
                        d.child(icon("check", th.text_dim, 18.0))
                    })
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if let Some(c) = &mut this.compose {
                            c.follow_up = after;
                            c.popup = None;
                        }
                        cx.notify();
                    }))
            }))
    }

    fn render_schedule_menu(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let now = jiff::Timestamp::now().to_zoned(self.tz.clone());
        let presets = schedule::presets(&now);
        let items = presets.into_iter().enumerate().map(|(ix, preset)| {
            let at = preset.at.timestamp();
            div()
                .id(("schedule-preset", ix))
                .h(px(40.0))
                .px(px(24.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .child(div().flex_1().child(preset.label))
                .child(
                    div()
                        .text_color(rgba(th.text_dim))
                        .child(schedule::short(&preset.at)),
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.send_compose(Some(at), false, Passed::default(), window, cx)
                }))
        });
        menu(th)
            .w(px(340.0))
            .child(
                div()
                    .px(px(24.0))
                    .pt(px(8.0))
                    .pb(px(8.0))
                    .text_size(px(16.0))
                    .child(tr!("schedule-title")),
            )
            .child(
                div()
                    .px(px(24.0))
                    .pb(px(8.0))
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_dim))
                    .child({
                        let zone = self
                            .tz
                            .iana_name()
                            .map(str::to_owned)
                            .unwrap_or_else(|| tr!("schedule-local-time"));
                        // Who sends it: the mail server, or Katna here.
                        match self.server_holds_mail() {
                            Some(true) => tr!("schedule-zone-note-server", zone = zone),
                            Some(false) => tr!("schedule-zone-note-local", zone = zone),
                            None => tr!("schedule-zone-note", zone = zone),
                        }
                    }),
            )
            .children(items)
            .child(menu_divider(th))
            .child(
                tool_item("schedule-pick", "calendar", &tr!("schedule-pick"), th)
                    .on_click(cx.listener(|this, _, window, cx| this.open_time_picker(window, cx))),
            )
            .into_any_element()
    }

    fn open_time_picker(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let now = jiff::Timestamp::now().to_zoned(self.tz.clone());
        let Some(c) = &mut self.compose else {
            return;
        };
        let tomorrow = now.date().tomorrow().unwrap_or(now.date());
        c.dialog.day = tomorrow;
        c.dialog.month = tomorrow;
        c.popup = Some(Popup::PickTime);
        let time = c.dialog.time.clone();
        time.update(cx, |t, cx| {
            t.set_text(schedule::clock(jiff::civil::Time::constant(8, 0, 0, 0)), cx);
            t.select_all_text(cx);
        });
        window.focus(&time.focus_handle(cx), cx);
        cx.notify();
    }

    /// Schedules for the date and time of the picker.
    fn schedule_picked(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &self.compose else {
            return;
        };
        let text = c.dialog.time.read(cx).text().to_owned();
        let Some(time) = schedule::parse_time(&text) else {
            let example = schedule::clock(jiff::civil::Time::constant(8, 0, 0, 0));
            self.show_snackbar(
                tr!("schedule-not-a-time", text = text, example = example),
                None,
                cx,
            );
            return;
        };
        let Some(at) = schedule::moment(c.dialog.day, time, &self.tz) else {
            self.show_snackbar(tr!("schedule-no-such-time"), None, cx);
            return;
        };
        self.send_compose(Some(at), false, Passed::default(), window, cx);
    }

    // Formatting.

    /// The formatting bar (Aa), if it is open: it floats over the end of
    /// the text just above the Send row, as wide as its buttons, so opening
    /// it moves nothing. Put it just before [`Self::render_compose_actions`].
    pub(super) fn render_floating_format_bar(
        &self,
        th: &Theme,
        width: f32,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !self.compose.as_ref()?.format_bar {
            return None;
        }
        Some(
            div()
                .relative()
                .flex_none()
                .h_0()
                .child(
                    div()
                        .id("format-bar")
                        .occlude()
                        .absolute()
                        .bottom(px(FORMAT_BAR_GAP))
                        .left(px(12.0))
                        .max_w(px(width))
                        // A pill tinted apart from the text under it, with
                        // a faint edge; on a narrow window its buttons
                        // scroll sideways.
                        .rounded_full()
                        .border_1()
                        .border_color(rgba(th.divider))
                        .bg(rgba(format_bar_bg(th)))
                        .shadow(crate::widgets::elevation(th, 1.0))
                        .overflow_x_scroll()
                        .child(
                            div()
                                .flex_none()
                                .child(self.render_format_bar(th, width, cx)),
                        ),
                )
                .into_any_element(),
        )
    }

    pub(super) fn render_format_bar(
        &self,
        th: &Theme,
        width: f32,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let editor = compose.body.read(cx);
        if editor.is_plain() {
            return div()
                .h(px(40.0))
                .px(px(16.0))
                .flex()
                .items_center()
                .text_size(px(13.0))
                .text_color(rgba(th.text_dim))
                .child(tr!("compose-tool-plain-note"))
                .into_any_element();
        }
        let style = editor.current_style();
        let para = editor.para_style();
        let (can_undo, can_redo, in_table) =
            (editor.can_undo(), editor.can_redo(), editor.in_table());
        let popup = compose.popup.clone();
        let open = |p: Popup| popup.as_ref() == Some(&p);
        let wide = width >= 660.0;

        let font = div()
            .relative()
            .child(
                format_dropdown("format-font", th)
                    .w(px(100.0))
                    .tooltip(tip(tr!("compose-tool-font"), th))
                    .on_click(cx.listener(|this, _, _, cx| this.toggle_popup(Popup::Font, cx)))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .child(style.font.label()),
                    )
                    .child(icon("drop-down", th.text_dim, 18.0)),
            )
            .when(open(Popup::Font), |d| {
                let items = Font::ALL.into_iter().enumerate().map(|(ix, font)| {
                    menu_item(("font", ix), font.label(), th)
                        .when(font == style.font, |d| {
                            d.child(div().flex_1())
                                .child(icon("check", th.text_dim, 18.0))
                        })
                        .on_click(self.on_body(cx, move |e, cx| e.set_font(font, cx)))
                });
                d.child(above(menu(th).w(px(200.0)).children(items)))
            });
        let size = div()
            .relative()
            .child(
                format_dropdown("format-size", th)
                    .tooltip(tip(tr!("compose-tool-size"), th))
                    .on_click(cx.listener(|this, _, _, cx| this.toggle_popup(Popup::Size, cx)))
                    .child(icon("text-size", th.text_dim, 18.0))
                    .child(icon("drop-down", th.text_dim, 18.0)),
            )
            .when(open(Popup::Size), |d| {
                let items = Size::ALL.into_iter().enumerate().map(|(ix, size)| {
                    div()
                        .id(("size", ix))
                        .min_h(px(32.0))
                        .px(px(16.0))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(12.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(th.hover)))
                        .child(div().w(px(18.0)).when(size == style.size, |d| {
                            d.child(icon("check", th.text_dim, 18.0))
                        }))
                        .child(div().text_size(px(14.0 * size.scale())).child(size.label()))
                        .on_click(self.on_body(cx, move |e, cx| e.set_size(size, cx)))
                });
                d.child(above(menu(th).w(px(180.0)).children(items)))
            });
        let colors = div()
            .relative()
            .child(
                format_dropdown("format-color", th)
                    .tooltip(tip(tr!("compose-tool-text-color"), th))
                    .on_click(cx.listener(|this, _, _, cx| this.toggle_popup(Popup::Colors, cx)))
                    .child(icon("text-color", th.text_dim, 18.0))
                    .child(icon("drop-down", th.text_dim, 18.0)),
            )
            .when(open(Popup::Colors), |d| {
                d.child(above(self.render_colors(
                    th,
                    style.color,
                    style.background,
                    cx,
                )))
            });
        let align_icon = match para.align {
            Align::Left => "align-left",
            Align::Center => "align-center",
            Align::Right => "align-right",
        };
        let align = div()
            .relative()
            .child(
                format_dropdown("format-align", th)
                    // Centred or right-aligned text shows as on, like Bold.
                    .when(para.align != Align::Left, |d| d.bg(rgba(format_active(th))))
                    .tooltip(tip(tr!("compose-tool-align"), th))
                    .on_click(cx.listener(|this, _, _, cx| this.toggle_popup(Popup::Align, cx)))
                    .child(icon(
                        align_icon,
                        if para.align == Align::Left {
                            th.text_dim
                        } else {
                            th.nav_selected_text
                        },
                        18.0,
                    ))
                    .child(icon("drop-down", th.text_dim, 18.0)),
            )
            .when(open(Popup::Align), |d| {
                let mut button = |id, name, value: Align, label: String| {
                    format_button(id, name, para.align == value, th)
                        .tooltip(tip(label, th))
                        .on_click(self.on_body(cx, move |e, cx| e.set_align(value, cx)))
                };
                d.child(above(
                    menu(th)
                        .min_w(px(0.0))
                        .px(px(6.0))
                        .py(px(6.0))
                        .flex_row()
                        .gap(px(2.0))
                        .child(button(
                            "align-left",
                            "align-left",
                            Align::Left,
                            tr!("compose-tool-align-left"),
                        ))
                        .child(button(
                            "align-center",
                            "align-center",
                            Align::Center,
                            tr!("compose-tool-align-center"),
                        ))
                        .child(button(
                            "align-right",
                            "align-right",
                            Align::Right,
                            tr!("compose-tool-align-right"),
                        )),
                ))
            });
        // What does not fit a narrow bar goes in its "more" menu.
        let table_click = cx.listener(move |this, _, _, cx| {
            let p = if in_table {
                Popup::TableEdit
            } else {
                Popup::Table
            };
            if let Some(c) = &mut this.compose {
                c.dialog.grid = (0, 0);
            }
            this.toggle_popup(p, cx)
        });
        let rest: Vec<AnyElement> = vec![
            format_button("format-indent-less", "indent-less", false, th)
                .tooltip(tip(tr!("compose-tool-indent-less"), th))
                .on_click(self.on_body(cx, |e, cx| e.indent(false, cx)))
                .into_any_element(),
            format_button("format-indent-more", "indent-more", false, th)
                .tooltip(tip(tr!("compose-tool-indent-more"), th))
                .on_click(self.on_body(cx, |e, cx| e.indent(true, cx)))
                .into_any_element(),
            format_button("format-quote", "quote", para.quote > 0, th)
                .tooltip(tip(tr!("compose-tool-quote"), th))
                .on_click(self.on_body(cx, |e, cx| e.toggle_quote(cx)))
                .into_any_element(),
            format_button("format-strike", "format-strike", style.strike, th)
                .tooltip(tip(tr!("compose-tool-strikethrough"), th))
                .on_click(self.on_body(cx, |e, cx| e.toggle_strike(cx)))
                .into_any_element(),
            format_button("format-clear", "clear-format", false, th)
                .tooltip(tip(tr!("compose-tool-remove-formatting"), th))
                .on_click(self.on_body(cx, |e, cx| e.clear_formatting(cx)))
                .into_any_element(),
            format_button("format-table", "table", in_table, th)
                .tooltip(tip(
                    if in_table {
                        tr!("compose-tool-table")
                    } else {
                        tr!("compose-tool-insert-table")
                    },
                    th,
                ))
                .on_click(table_click)
                .into_any_element(),
        ];
        let table_popup = if open(Popup::Table) {
            Some(above(self.render_table_grid(th, cx)))
        } else if open(Popup::TableEdit) {
            Some(above(self.render_table_menu(th, cx)))
        } else {
            None
        };
        let tail = if wide {
            div()
                .relative()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(1.0))
                .children(rest)
                .children(table_popup)
        } else {
            div()
                .relative()
                .child(
                    format_button("format-more", "drop-down", open(Popup::MoreFormat), th)
                        .tooltip(tip(tr!("compose-tool-more-formatting"), th))
                        .on_click(
                            cx.listener(|this, _, _, cx| this.toggle_popup(Popup::MoreFormat, cx)),
                        ),
                )
                .when(open(Popup::MoreFormat), |d| {
                    d.child(above(
                        menu(th)
                            .min_w(px(0.0))
                            .px(px(6.0))
                            .py(px(6.0))
                            .flex_row()
                            .gap(px(2.0))
                            .children(rest),
                    ))
                })
                .children(table_popup)
        };
        div()
            .h(px(40.0))
            .px(px(12.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(1.0))
            .child(
                format_button("format-undo", "undo", false, th)
                    .when(!can_undo, |d| d.opacity(0.4))
                    .tooltip(tip(tr!("compose-tool-undo"), th))
                    .on_click(self.on_body(cx, |e, cx| e.undo(cx))),
            )
            .child(
                format_button("format-redo", "redo", false, th)
                    .when(!can_redo, |d| d.opacity(0.4))
                    .tooltip(tip(tr!("compose-tool-redo"), th))
                    .on_click(self.on_body(cx, |e, cx| e.redo(cx))),
            )
            .child(separator(th))
            .child(font)
            .child(separator(th))
            .child(size)
            .child(separator(th))
            .child(
                format_button("format-bold", "format-bold", style.bold, th)
                    .tooltip(tip(tr!("compose-tool-bold"), th))
                    .on_click(self.on_body(cx, |e, cx| e.toggle_bold(cx))),
            )
            .child(
                format_button("format-italic", "format-italic", style.italic, th)
                    .tooltip(tip(tr!("compose-tool-italic"), th))
                    .on_click(self.on_body(cx, |e, cx| e.toggle_italic(cx))),
            )
            .child(
                format_button("format-underline", "format-underline", style.underline, th)
                    .tooltip(tip(tr!("compose-tool-underline"), th))
                    .on_click(self.on_body(cx, |e, cx| e.toggle_underline(cx))),
            )
            .child(colors)
            .child(separator(th))
            .child(align)
            .child(
                format_button(
                    "format-numbered",
                    "list-numbered",
                    para.list == List::Numbered,
                    th,
                )
                .tooltip(tip(tr!("compose-tool-numbered-list"), th))
                .on_click(self.on_body(cx, |e, cx| e.toggle_list(List::Numbered, cx))),
            )
            .child(
                format_button(
                    "format-bulleted",
                    "list-bulleted",
                    para.list == List::Bullet,
                    th,
                )
                .tooltip(tip(tr!("compose-tool-bulleted-list"), th))
                .on_click(self.on_body(cx, |e, cx| e.toggle_list(List::Bullet, cx))),
            )
            .child(tail)
            .into_any_element()
    }

    fn render_colors(
        &self,
        th: &Theme,
        color: Option<u32>,
        background: Option<u32>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        div()
            .p(px(12.0))
            .flex()
            .flex_row()
            .gap(px(20.0))
            .map(|d| crate::widgets::raised(d, th, 8.0, 3.0))
            .child(self.color_palette(th, true, background, cx))
            .child(self.color_palette(th, false, color, cx))
            .into_any_element()
    }

    /// One palette of the color menu, choosing the background with
    /// `background`.
    fn color_palette(
        &self,
        th: &Theme,
        background: bool,
        current: Option<u32>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let name = if background { "bg-color" } else { "fg-color" };
        let set = move |e: &mut RichEditor, color: Option<u32>, cx: &mut Context<RichEditor>| {
            if background {
                e.set_background(color, cx)
            } else {
                e.set_color(color, cx)
            }
        };
        let swatches = COLORS.iter().enumerate().map(|(ix, &c)| {
            let selected = current == Some(c);
            let light = (c >> 16) + ((c >> 8) & 0xff) + (c & 0xff) > 384;
            div()
                .id((name, ix))
                .size(px(18.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(3.0))
                .bg(rgba((c << 8) | 0xff))
                .when(light, |d| d.border_1().border_color(rgba(th.divider)))
                .when(selected, |d| d.border_1().border_color(rgba(th.text)))
                .cursor_pointer()
                .hover(|s| s.border_1().border_color(rgba(th.text)))
                .when(selected, |d| {
                    d.child(icon(
                        "check",
                        if light { 0x000000ff } else { 0xffffffff },
                        14.0,
                    ))
                })
                .on_click(self.on_body(cx, move |e, cx| set(e, Some(c), cx)))
        });
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgba(th.text_dim))
                    .child(if background {
                        tr!("compose-tool-background-color")
                    } else {
                        tr!("compose-tool-text-color")
                    }),
            )
            .child(
                div()
                    .w(px(8.0 * 20.0))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(2.0))
                    .children(swatches),
            )
            .child(
                div()
                    .id(if background {
                        "bg-default"
                    } else {
                        "fg-default"
                    })
                    .h(px(28.0))
                    .px(px(8.0))
                    .flex()
                    .items_center()
                    .rounded(px(4.0))
                    .text_size(px(13.0))
                    .text_color(rgba(th.text_dim))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .when(current.is_none(), |d| d.font_weight(FontWeight::MEDIUM))
                    .child(if background {
                        tr!("compose-tool-no-background")
                    } else {
                        tr!("compose-tool-default-color")
                    })
                    .on_click(self.on_body(cx, move |e, cx| set(e, None, cx))),
            )
            .into_any_element()
    }

    /// The grid that picks a new table's rows and columns.
    fn render_table_grid(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let (rows, cols) = compose.dialog.grid;
        let cells = (0..TABLE_GRID * TABLE_GRID).map(|ix| {
            let (r, col) = (ix / TABLE_GRID + 1, ix % TABLE_GRID + 1);
            let on = r <= rows && col <= cols;
            div()
                .id(("table-cell", ix))
                .size(px(18.0))
                .rounded(px(2.0))
                .border_1()
                .border_color(rgba(if on { th.accent } else { th.divider }))
                .when(on, |d| d.bg(rgba(fade(th.accent, 0.25))))
                .cursor_pointer()
                .on_hover(cx.listener(move |this, hovered: &bool, _, cx| {
                    if *hovered && let Some(c) = &mut this.compose {
                        c.dialog.grid = (r, col);
                        cx.notify();
                    }
                }))
                .on_click(self.on_body(cx, move |e, cx| e.insert_table(r, col, cx)))
        });
        menu(th)
            .min_w(px(0.0))
            .px(px(12.0))
            .py(px(10.0))
            .gap(px(8.0))
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgba(th.text_dim))
                    .child(if rows == 0 {
                        tr!("compose-tool-insert-table")
                    } else {
                        tr!("compose-tool-table-size", columns = cols, rows = rows)
                    }),
            )
            .child(
                div()
                    .w(px(TABLE_GRID as f32 * 20.0))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(2.0))
                    .children(cells),
            )
            .into_any_element()
    }

    fn render_table_menu(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let mut item = |ix: usize, label: &str, edit: TableEdit| {
            menu_item(("table-edit", ix), label, th)
                .when(edit == TableEdit::DeleteTable, |d| {
                    d.text_color(rgba(th.error))
                })
                .on_click(self.on_body(cx, move |e, cx| e.edit_table(edit, cx)))
        };
        menu(th)
            .w(px(220.0))
            .child(item(0, &tr!("compose-tool-row-above"), TableEdit::RowAbove))
            .child(item(1, &tr!("compose-tool-row-below"), TableEdit::RowBelow))
            .child(item(
                2,
                &tr!("compose-tool-column-left"),
                TableEdit::ColumnLeft,
            ))
            .child(item(
                3,
                &tr!("compose-tool-column-right"),
                TableEdit::ColumnRight,
            ))
            .child(menu_divider(th))
            .child(item(
                4,
                &tr!("compose-tool-delete-row"),
                TableEdit::DeleteRow,
            ))
            .child(item(
                5,
                &tr!("compose-tool-delete-column"),
                TableEdit::DeleteColumn,
            ))
            .child(item(
                6,
                &tr!("compose-tool-delete-table"),
                TableEdit::DeleteTable,
            ))
            .into_any_element()
    }

    // Emoji.

    pub(super) fn render_emoji_picker(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let query = compose.dialog.emoji_search.read(cx).text().to_owned();
        let group = compose.dialog.emoji_group;
        let list = emoji_list(&query, group);
        let tabs = EMOJI_GROUPS
            .iter()
            .enumerate()
            .map(|(ix, &(kind, face))| {
                let selected = query.trim().is_empty() && ix == group;
                div()
                    .id(("emoji-group", ix))
                    .size(px(32.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(6.0))
                    .text_size(px(18.0))
                    .cursor_pointer()
                    .when(selected, |d| d.bg(rgba(th.chip)))
                    .hover(|s| s.bg(rgba(th.hover)))
                    .tooltip(tip(emoji_group_name(kind), th))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if let Some(c) = &mut this.compose {
                            c.dialog.emoji_group = ix;
                            c.dialog.emoji_search.update(cx, |s, cx| s.set_text("", cx));
                        }
                        cx.notify();
                    }))
                    .child(face)
            })
            .collect::<Vec<_>>();
        let cells = list.iter().enumerate().map(|(ix, emoji)| {
            let text = emoji.as_str();
            div()
                .id(("emoji", ix))
                .size(px(36.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(6.0))
                .text_size(px(22.0))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(th.hover)))
                .tooltip(tip(emoji.name().to_owned(), th))
                .on_click(self.on_body(cx, move |e, cx| e.insert(text, cx)))
                .child(text)
        });
        let title = if query.trim().is_empty() {
            emoji_group_name(EMOJI_GROUPS[group.min(EMOJI_GROUPS.len() - 1)].0)
        } else if list.is_empty() {
            tr!("compose-tool-emoji-none")
        } else {
            tr!("compose-tool-emoji-results")
        };
        div()
            .w(px(360.0))
            .h(px(380.0))
            .flex()
            .flex_col()
            .rounded(px(12.0))
            .overflow_hidden()
            .bg(rgba(th.menu))
            .shadow(crate::widgets::elevation(th, 3.0))
            .text_color(rgba(th.text))
            .child(
                div()
                    .flex_none()
                    .m(px(10.0))
                    .h(px(36.0))
                    .px(px(12.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .rounded_full()
                    .bg(rgba(th.chip))
                    .text_size(px(14.0))
                    .child(icon("search", th.text_dim, 18.0))
                    .child(div().flex_1().child(compose.dialog.emoji_search.clone())),
            )
            .child(
                div()
                    .flex_none()
                    .px(px(8.0))
                    .flex()
                    .flex_row()
                    .justify_between()
                    .children(tabs),
            )
            .child(
                div()
                    .flex_none()
                    .px(px(14.0))
                    .pt(px(8.0))
                    .pb(px(4.0))
                    .text_size(px(12.0))
                    .text_color(rgba(th.text_dim))
                    .child(title),
            )
            .child(
                div()
                    .id("emoji-grid")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .px(px(8.0))
                    .pb(px(8.0))
                    .child(div().flex().flex_row().flex_wrap().children(cells)),
            )
            .into_any_element()
    }

    /// Enter in the emoji search inserts the first match.
    fn insert_first_emoji(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &self.compose else {
            return;
        };
        let query = c.dialog.emoji_search.read(cx).text().to_owned();
        let Some(first) = emoji_list(&query, c.dialog.emoji_group).first().copied() else {
            return;
        };
        self.edit_body(window, cx, |e, cx| e.insert(first.as_str(), cx));
    }

    // Links.

    /// Opens the link dialog on the link at the cursor, or for the selected
    /// text.
    pub(super) fn open_link_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        let editor = c.body.read(cx);
        let (text, url, editing) = match editor.link_at_cursor() {
            Some((text, url)) => (text, url.to_string(), true),
            None => (editor.selected_text(), String::new(), false),
        };
        c.dialog.editing_link = editing;
        c.popup = Some(Popup::Link);
        let (text_input, url_input) = (c.dialog.link_text.clone(), c.dialog.link_url.clone());
        let focus_text = text.is_empty();
        text_input.update(cx, |i, cx| i.set_text(text, cx));
        url_input.update(cx, |i, cx| {
            i.set_text(url, cx);
            i.select_all_text(cx);
        });
        let focus = if focus_text { &text_input } else { &url_input };
        window.focus(&focus.focus_handle(cx), cx);
        cx.notify();
    }

    fn apply_link(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &self.compose else {
            return;
        };
        let text = c.dialog.link_text.read(cx).text().to_owned();
        let url = normalize_url(c.dialog.link_url.read(cx).text());
        let editing = c.dialog.editing_link;
        if url.is_empty() {
            if editing {
                self.edit_body(window, cx, |e, cx| e.remove_link(cx));
            } else {
                self.close_popup(window, cx);
            }
            return;
        }
        let text = if text.trim().is_empty() {
            url.trim_start_matches("mailto:").to_owned()
        } else {
            text
        };
        self.edit_body(window, cx, move |e, cx| e.set_link(&text, &url, cx));
    }

    /// "Go to link · Change · Remove" under a link the cursor is in.
    pub(super) fn render_link_bubble(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let compose = self.compose.as_ref()?;
        if compose.popup.is_some() || !compose.body.read(cx).had_focus() {
            return None;
        }
        let editor = compose.body.read(cx);
        if editor.has_selection() {
            return None;
        }
        let (_, url) = editor.link_at_cursor()?;
        let cursor = editor.cursor_bounds()?;
        let open_url = url.to_string();
        Some(
            deferred(
                anchored()
                    .position(point(cursor.left(), cursor.bottom() + px(6.0)))
                    .snap_to_window_with_margin(px(8.0))
                    .child(
                        div()
                            .id("link-bubble")
                            .occlude()
                            .h(px(36.0))
                            .px(px(12.0))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(12.0))
                            .map(|d| crate::widgets::raised(d, th, 8.0, 2.0))
                            .text_size(px(13.0))
                            .text_color(rgba(th.text_dim))
                            .child(tr!("compose-tool-link-go"))
                            .child(
                                div()
                                    .id("link-bubble-open")
                                    .max_w(px(240.0))
                                    .truncate()
                                    .text_color(rgba(th.accent))
                                    .cursor_pointer()
                                    .hover(|s| s.underline())
                                    .on_click(move |_, _, cx| cx.open_url(&open_url))
                                    .child(url.to_string()),
                            )
                            .child(div().text_color(rgba(th.divider)).child("|"))
                            .child(
                                div()
                                    .id("link-bubble-change")
                                    .text_color(rgba(th.accent))
                                    .cursor_pointer()
                                    .hover(|s| s.underline())
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.open_link_dialog(window, cx)
                                    }))
                                    .child(tr!("compose-tool-link-change")),
                            )
                            .child(
                                div()
                                    .id("link-bubble-remove")
                                    .text_color(rgba(th.accent))
                                    .cursor_pointer()
                                    .hover(|s| s.underline())
                                    .on_click(self.on_body(cx, |e, cx| e.remove_link(cx)))
                                    .child(tr!("compose-tool-link-remove")),
                            ),
                    ),
            )
            .with_priority(1)
            .into_any_element(),
        )
    }

    // Right-click menu.

    pub(super) fn open_compose_menu(
        &mut self,
        position: Point<Pixels>,
        misspelled: Option<String>,
        grammar: Option<GrammarIssue>,
        cx: &mut Context<Self>,
    ) {
        if let Some(c) = &mut self.compose {
            c.popup = Some(Popup::Context {
                position,
                misspelled,
                grammar,
            });
        }
        cx.notify();
    }

    pub(super) fn render_context_popup(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let compose = self.compose.as_ref()?;
        let Some(Popup::Context {
            position,
            misspelled,
            grammar,
        }) = &compose.popup
        else {
            return None;
        };
        let editor = compose.body.read(cx);
        let suggestions = if misspelled.is_some() {
            editor.suggestions()
        } else {
            Vec::new()
        };
        let in_table = editor.in_table();
        let in_link = editor.link_at_cursor().is_some();
        let selection = editor.has_selection();
        let mut items = self.fix_items(
            menu(th).w(px(240.0)),
            misspelled.clone(),
            suggestions,
            grammar.clone(),
            None,
            th,
            cx,
        );
        if misspelled.is_some() || grammar.is_some() {
            items = items.child(menu_divider(th));
        }
        let action = |ix: usize, label: &str, keys: &'static str, what: Clip| {
            menu_item(("clip", ix), label, th)
                .child(div().flex_1())
                .child(shortcut(keys, th))
                .on_click(cx.listener(move |this, _, window, cx| this.clipboard(what, window, cx)))
        };
        items = items
            .when(selection, |d| {
                d.child(action(0, &tr!("compose-tool-cut"), "Ctrl+X", Clip::Cut))
                    .child(action(1, &tr!("compose-tool-copy"), "Ctrl+C", Clip::Copy))
            })
            .child(action(2, &tr!("compose-tool-paste"), "Ctrl+V", Clip::Paste))
            .child(action(
                3,
                &tr!("compose-tool-select-all"),
                "Ctrl+A",
                Clip::SelectAll,
            ));
        if in_link {
            items = items
                .child(menu_divider(th))
                .child(
                    menu_item("context-link-edit", &tr!("compose-tool-edit-link"), th).on_click(
                        cx.listener(|this, _, window, cx| this.open_link_dialog(window, cx)),
                    ),
                )
                .child(
                    menu_item("context-link-remove", &tr!("compose-tool-remove-link"), th)
                        .on_click(self.on_body(cx, |e, cx| e.remove_link(cx))),
                );
        }
        if in_table {
            let mut edit = |ix: usize, label: &str, edit: TableEdit| {
                menu_item(("context-table", ix), label, th)
                    .on_click(self.on_body(cx, move |e, cx| e.edit_table(edit, cx)))
            };
            items = items
                .child(menu_divider(th))
                .child(edit(0, &tr!("compose-tool-row-above"), TableEdit::RowAbove))
                .child(edit(1, &tr!("compose-tool-row-below"), TableEdit::RowBelow))
                .child(edit(
                    2,
                    &tr!("compose-tool-column-left"),
                    TableEdit::ColumnLeft,
                ))
                .child(edit(
                    3,
                    &tr!("compose-tool-column-right"),
                    TableEdit::ColumnRight,
                ))
                .child(edit(
                    4,
                    &tr!("compose-tool-delete-row"),
                    TableEdit::DeleteRow,
                ))
                .child(edit(
                    5,
                    &tr!("compose-tool-delete-column"),
                    TableEdit::DeleteColumn,
                ))
                .child(edit(
                    6,
                    &tr!("compose-tool-delete-table"),
                    TableEdit::DeleteTable,
                ));
        }
        Some(
            deferred(
                anchored()
                    .position(*position)
                    .snap_to_window_with_margin(px(8.0))
                    .child(div().occlude().child(items)),
            )
            .with_priority(2)
            .into_any_element(),
        )
    }

    /// The fixes of a misspelled word (`suggestions` for `misspelled`) or a
    /// grammar mistake, added to `items`. With `at`, the cursor moves to
    /// the words first.
    #[allow(clippy::too_many_arguments)]
    fn fix_items(
        &self,
        mut items: Div,
        misspelled: Option<String>,
        suggestions: Vec<String>,
        grammar: Option<GrammarIssue>,
        at: Option<Pos>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Div {
        if let Some(word) = misspelled {
            if suggestions.is_empty() {
                items = items.child(
                    div()
                        .h(px(32.0))
                        .px(px(16.0))
                        .flex()
                        .items_center()
                        .text_color(rgba(th.text_faint))
                        .child(tr!("compose-tool-no-suggestions")),
                );
            }
            for (ix, s) in suggestions.into_iter().enumerate() {
                let word = s.clone();
                items = items.child(
                    menu_item(("spell-suggestion", ix), &s, th)
                        .font_weight(FontWeight::BOLD)
                        .on_click(self.on_words(at, cx, move |e, cx| e.replace_word(&word, cx))),
                );
            }
            items = items.child(
                menu_item("spell-add", &tr!("compose-tool-add-to-dictionary"), th).on_click(
                    cx.listener(move |this, _, window, cx| {
                        this.add_to_dictionary(&word, window, cx)
                    }),
                ),
            );
        }
        if let Some(issue) = grammar {
            items = items.child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .text_size(px(13.0))
                    .line_height(px(18.0))
                    .text_color(rgba(th.text_dim))
                    .child(issue.message.clone()),
            );
            for (ix, fix) in issue.fixes.iter().enumerate() {
                let (issue, fix) = (issue.clone(), fix.clone());
                items = items.child(
                    menu_item(("grammar-fix", ix), &fix.label, th)
                        .font_weight(FontWeight::BOLD)
                        .on_click(
                            self.on_words(at, cx, move |e, cx| e.fix_grammar(&issue, &fix, cx)),
                        ),
                );
            }
            items =
                items.child(
                    menu_item("grammar-ignore", &tr!("grammar-ignore"), th)
                        .on_click(self.on_words(at, cx, move |e, cx| e.ignore_grammar(&issue, cx))),
                );
        }
        items
    }

    /// A listener that runs `f` on the editor, with the cursor moved to `at`
    /// first if given.
    fn on_words<E>(
        &self,
        at: Option<Pos>,
        cx: &mut Context<Self>,
        f: impl Fn(&mut RichEditor, &mut Context<RichEditor>) + 'static,
    ) -> impl Fn(&E, &mut Window, &mut gpui::App) + 'static {
        self.on_body(cx, move |e, cx| {
            if let Some(at) = at {
                e.set_cursor(at, cx);
            }
            f(e, cx)
        })
    }

    /// Shows a card of fixes unless another menu or dialog is open.
    pub(super) fn show_hint(&mut self, hint: Popup, cx: &mut Context<Self>) {
        if let Some(c) = &mut self.compose
            && c.popup.as_ref().is_none_or(Popup::is_hint)
        {
            c.popup = Some(hint);
            cx.notify();
        }
    }

    /// Closes the card of fixes, if one is open.
    pub(super) fn close_hint(&mut self, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        if c.popup.as_ref().is_some_and(Popup::is_hint) {
            c.popup = None;
            c.body.update(cx, |editor, _| editor.end_hint());
            c.subject.update(cx, |input, _| input.end_hint());
            cx.notify();
        }
    }

    /// The fixes of the marked words under the pointer or a left click.
    pub(super) fn render_hint(&self, th: &Theme, cx: &mut Context<Self>) -> Option<AnyElement> {
        let compose = self.compose.as_ref()?;
        let Some(Popup::Hint {
            word,
            at,
            misspelled,
            suggestions,
            grammar,
        }) = &compose.popup
        else {
            return None;
        };
        let items = self.fix_items(
            menu(th).w(px(240.0)),
            misspelled.clone(),
            suggestions.clone(),
            grammar.clone(),
            Some(*at),
            th,
            cx,
        );
        Some(self.hint_card(*word, items, cx))
    }

    /// A card of fixes under `word`, closing when the pointer leaves both or
    /// on a click outside.
    fn hint_card(&self, word: Bounds<Pixels>, items: Div, cx: &mut Context<Self>) -> AnyElement {
        let this = cx.entity().downgrade();
        let watch = canvas(
            |_, _, _| {},
            move |card, _, window, _| {
                let this = this.clone();
                window.on_mouse_event(move |event: &MouseMoveEvent, phase, _, cx| {
                    let on_words = word.dilate(px(2.0)).contains(&event.position);
                    if phase == DispatchPhase::Bubble
                        && !on_words
                        && !card.contains(&event.position)
                    {
                        this.update(cx, |this, cx| this.close_hint(cx)).ok();
                    }
                });
            },
        )
        .absolute()
        .top_0()
        .left_0()
        .size_full();
        deferred(
            anchored()
                .position(word.bottom_left())
                .snap_to_window_with_margin(px(8.0))
                .child(
                    div()
                        .id("compose-hint")
                        .relative()
                        .occlude()
                        .on_mouse_down_out(cx.listener(|this, _, _, cx| this.close_hint(cx)))
                        .child(items)
                        .child(watch),
                ),
        )
        .with_priority(2)
        .into_any_element()
    }

    /// The fixes of a grammar mistake in the subject.
    pub(super) fn render_subject_grammar(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let compose = self.compose.as_ref()?;
        let Some(Popup::SubjectGrammar {
            position,
            issue,
            word,
        }) = &compose.popup
        else {
            return None;
        };
        let on_subject = |f: Box<SubjectEdit>| {
            cx.listener(move |this: &mut Self, _, window, cx| {
                let Some(c) = &mut this.compose else {
                    return;
                };
                c.popup = None;
                let subject = c.subject.clone();
                window.focus(&subject.focus_handle(cx), cx);
                subject.update(cx, |input, cx| f(input, cx));
                cx.notify();
            })
        };
        let mut items = menu(th).w(px(240.0)).child(
            div()
                .px(px(16.0))
                .py(px(8.0))
                .text_size(px(13.0))
                .line_height(px(18.0))
                .text_color(rgba(th.text_dim))
                .child(issue.message.clone()),
        );
        for (ix, fix) in issue.fixes.iter().enumerate() {
            let (issue, fix) = (issue.clone(), fix.clone());
            items = items.child(
                menu_item(("subject-grammar-fix", ix), &fix.label, th)
                    .font_weight(FontWeight::BOLD)
                    .on_click(on_subject(Box::new(move |input, cx| {
                        input.fix_grammar(&issue, &fix, cx)
                    }))),
            );
        }
        let ignored = issue.clone();
        items = items.child(
            menu_item("subject-grammar-ignore", &tr!("grammar-ignore"), th).on_click(on_subject(
                Box::new(move |input, cx| input.ignore_grammar(&ignored, cx)),
            )),
        );
        if let Some(word) = word {
            return Some(self.hint_card(*word, items, cx));
        }
        Some(
            deferred(
                anchored()
                    .position(*position)
                    .snap_to_window_with_margin(px(8.0))
                    .child(div().occlude().child(items)),
            )
            .with_priority(2)
            .into_any_element(),
        )
    }

    fn clipboard(&mut self, what: Clip, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        c.popup = None;
        window.focus(&c.body.focus_handle(cx), cx);
        let action: Box<dyn gpui::Action> = match what {
            Clip::Cut => Box::new(katna_ui::text_area::Cut),
            Clip::Copy => Box::new(katna_ui::text_area::Copy),
            Clip::Paste => Box::new(katna_ui::text_area::Paste),
            Clip::SelectAll => Box::new(katna_ui::text_area::SelectAll),
        };
        window.dispatch_action(action, cx);
        cx.notify();
    }

    fn add_to_dictionary(&mut self, word: &str, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(speller) = &self.writing.speller
            && let Err(err) = speller.add_word(word)
        {
            self.show_snackbar(
                tr!("compose-tool-dictionary-failed", error = err.to_string()),
                None,
                cx,
            );
        }
        // Drawn again, the word is no longer marked.
        self.edit_body(window, cx, |_, cx| cx.notify());
    }

    // More options.

    fn render_more_menu(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let sending = &self.config.sending;
        let plain = compose.plain(cx);
        let spell = sending.spell_check;
        let check = |on: bool| {
            div()
                .w(px(20.0))
                .flex_none()
                .when(on, |d| d.child(icon("check", th.text_dim, 20.0)))
        };
        let label_open = compose.popup == Some(Popup::Label);
        let label_item = div()
            .relative()
            .child(
                tool_item("more-label", "label", &tr!("compose-tool-label"), th)
                    .child(icon(
                        if label_open {
                            "chevron-down"
                        } else {
                            "chevron-right"
                        },
                        th.text_dim,
                        18.0,
                    ))
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(c) = &mut this.compose {
                            c.popup = Some(if c.popup == Some(Popup::Label) {
                                Popup::More
                            } else {
                                Popup::Label
                            });
                        }
                        cx.notify();
                    })),
            )
            .when(label_open, |d| {
                d.child(
                    div()
                        .px(px(16.0))
                        .pl(px(50.0))
                        .pb(px(8.0))
                        .text_size(px(13.0))
                        .text_color(rgba(th.text_dim))
                        .child(tr!("compose-tool-label-coming")),
                )
            });
        menu(th)
            .w(px(260.0))
            .child(
                tool_item(
                    "more-full-screen",
                    "open-full",
                    &tr!("compose-tool-full-screen"),
                    th,
                )
                .child(check(sending.compose_full_screen))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.config.sending.compose_full_screen =
                        !this.config.sending.compose_full_screen;
                    let full = this.config.sending.compose_full_screen;
                    this.save_config();
                    if let Some(c) = &mut this.compose {
                        c.popup = None;
                        if matches!(c.mode, Mode::Open | Mode::Full) {
                            c.mode = if full { Mode::Full } else { Mode::Open };
                        }
                    }
                    cx.notify();
                })),
            )
            .child(
                tool_item(
                    "more-video-call",
                    "video",
                    &tr!("compose-tool-video-call"),
                    th,
                )
                .on_click(cx.listener(|this, _, window, cx| this.add_video_call(window, cx))),
            )
            .child(menu_divider(th))
            .child(label_item)
            .child(menu_divider(th))
            .child(
                tool_item(
                    "more-plain",
                    "plain-text",
                    &tr!("compose-tool-plain-mode"),
                    th,
                )
                .child(check(plain))
                .on_click(cx.listener(|this, _, window, cx| this.toggle_plain(window, cx))),
            )
            .child(menu_divider(th))
            .child(
                tool_item("more-print", "print", &tr!("compose-tool-print"), th)
                    .on_click(cx.listener(|this, _, _, cx| this.print_compose(cx))),
            )
            .child(
                tool_item(
                    "more-spell",
                    "spell-check",
                    &tr!("compose-tool-check-spelling"),
                    th,
                )
                .child(check(spell))
                .on_click(cx.listener(|this, _, window, cx| this.toggle_spell_check(window, cx))),
            )
            .into_any_element()
    }

    /// Puts a new video call's link in the message at the cursor: the
    /// sending account's own Google Meet, else a Jitsi Meet room.
    fn add_video_call(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        c.popup = None;
        let kind = c.kind;
        let account = c.from.or_else(|| self.compose_account(kind).map(|a| a.id));
        let link = self.new_call_link(account);
        cx.notify();
        cx.spawn_in(window, async move |this, cx| {
            let link = link.await;
            this.update_in(cx, |this, window, cx| {
                this.edit_body(window, cx, |editor, cx| {
                    editor.insert(&tr!("meeting-join-line"), cx);
                    editor.insert(" ", cx);
                    editor.set_link("", &link, cx);
                });
            })
            .ok();
        })
        .detach();
    }

    /// Plain text mode on or off. Turning it on asks first when it would
    /// drop formatting.
    fn toggle_plain(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        let editor = c.body.read(cx);
        if editor.is_plain() {
            c.popup = None;
            c.body.update(cx, |e, cx| e.set_plain(false, cx));
            self.config.sending.plain_text = false;
            self.save_config();
        } else if editor.doc().has_formatting() && c.popup != Some(Popup::PlainText) {
            c.popup = Some(Popup::PlainText);
        } else {
            self.make_plain(window, cx);
            return;
        }
        cx.notify();
    }

    /// Drops the formatting, pictures and tables, keeping the text.
    fn make_plain(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        c.format_bar = false;
        let doc = html::from_plain(&html::to_plain(c.body.read(cx).doc()));
        let mut doc = doc;
        // The signature stays the signature.
        mark_signature(&mut doc);
        self.config.sending.plain_text = true;
        self.save_config();
        self.edit_body(window, cx, move |e, cx| {
            let at = e.doc().start();
            e.set_doc(doc, at, cx);
            e.set_plain(true, cx);
        });
    }

    fn toggle_spell_check(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.config.sending.spell_check = !self.config.sending.spell_check;
        self.save_config();
        let speller = self.speller(cx);
        let failed = match &self.writing.speller_state {
            super::SpellerState::Failed(err) if self.config.sending.spell_check => {
                Some(err.clone())
            }
            _ => None,
        };
        self.edit_body(window, cx, move |e, cx| e.set_spell_check(speller, cx));
        if let Some(err) = failed {
            self.show_snackbar(err, None, cx);
        }
    }

    /// Opens the message in the browser to print, as a page with its
    /// headers.
    fn print_compose(&mut self, cx: &mut Context<Self>) {
        let Some(c) = &mut self.compose else {
            return;
        };
        c.popup = None;
        let draft = c.fields(cx);
        let body = html::to_html(&draft.body, &|image| html::data_uri(image));
        let field = |label: String, value: &str| {
            if value.trim().is_empty() {
                String::new()
            } else {
                format!(
                    "<div><b>{}</b> {}</div>",
                    html::escape(&label),
                    html::escape(value.trim())
                )
            }
        };
        let no_subject = tr!("compose-tool-print-no-subject");
        let page = format!(
            "<!doctype html><html><head><meta charset=\"utf-8\"><title>{title}</title>\
             <style>body{{font-family:arial,sans-serif;font-size:14px;margin:32px}}\
             h1{{font-size:20px;font-weight:normal}}.h{{color:#444;margin-bottom:16px;\
             border-bottom:1px solid #ccc;padding-bottom:8px}}</style></head>\
             <body onload=\"window.print()\"><h1>{title}</h1><div class=\"h\">{to}{cc}{bcc}</div>\
             {body}</body></html>",
            title = html::escape(if draft.subject.trim().is_empty() {
                &no_subject
            } else {
                draft.subject.trim()
            }),
            to = field(tr!("compose-tool-print-to"), &draft.to),
            cc = field(tr!("compose-tool-print-cc"), &draft.cc),
            bcc = field(tr!("compose-tool-print-bcc"), &draft.bcc),
        );
        let dir = std::env::var_os("XDG_RUNTIME_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(std::env::temp_dir)
            .join("katna");
        let path = dir.join("print.html");
        let written = std::fs::create_dir_all(&dir).and_then(|_| std::fs::write(&path, page));
        match written {
            Ok(()) => cx.open_url(&format!("file://{}", path.display())),
            Err(err) => self.show_snackbar(tr!("print-failed", error = err.to_string()), None, cx),
        }
        cx.notify();
    }

    // Dialogs over the message.

    /// The compose dialogs (link, date and time, send checks, …), centred
    /// in the window whatever the message's length or scroll, so a long
    /// quoted forward never pushes one out of sight.
    pub(super) fn render_compose_dialog(
        &self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let compose = self.compose.as_ref()?;
        let popup = compose.popup.as_ref()?;
        let card = match popup {
            Popup::Link => self.render_link_dialog(th, cx),
            Popup::PickTime => self.render_time_picker(th, cx),
            Popup::PlainText => self.render_plain_dialog(th, cx),
            Popup::SaveTemplate => self.render_save_template_dialog(th, cx),
            Popup::SendCheck {
                check,
                at,
                archive,
                passed,
            } => self.render_send_check(*check, *at, *archive, *passed, th, cx),
            Popup::DriveShare {
                refused,
                at,
                archive,
                passed,
            } => self.render_drive_share(refused, *at, *archive, *passed, th, cx),
            Popup::BadAddress { field, address } => {
                self.render_bad_address(*field, address, th, cx)
            }
            _ => return None,
        };
        // The dialogs with a field keep the keys in it (its Enter and Esc
        // are the dialog's); the others take them here.
        let focus = compose.dialog.focus.clone();
        if !has_field(popup) && !focus.is_focused(window) {
            window.focus(&focus, cx);
        }
        let viewport = window.viewport_size();
        let (vw, vh) = (unpx(viewport.width), unpx(viewport.height));
        Some(
            div()
                .id("compose-dialog-scrim")
                .track_focus(&focus)
                .on_key_down(cx.listener(|this, event: &gpui::KeyDownEvent, window, cx| {
                    let stroke = &event.keystroke;
                    if stroke.modifiers.modified() {
                        return;
                    }
                    match stroke.key.as_str() {
                        "escape" => {
                            cx.stop_propagation();
                            this.close_popup(window, cx);
                        }
                        "enter" if this.answer_compose_dialog(window, cx) => {
                            cx.stop_propagation();
                        }
                        _ => {}
                    }
                }))
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgba(0x0000_0040))
                .child(
                    div()
                        .id("compose-dialog")
                        .occlude()
                        .max_w(px(DIALOG_MAX_WIDTH.min(vw - 32.0)))
                        .max_h(px(vh * 0.7))
                        .overflow_y_scroll()
                        .rounded(px(DIALOG_RADIUS))
                        .child(card),
                )
                .into_any_element(),
        )
    }

    /// The dialog over the main window: over the whole window, not inside
    /// the message, so it stays in sight however long the message is. The
    /// popped-out message shows its own in its window.
    pub(in crate::window) fn render_docked_compose_dialog(
        &self,
        th: &Theme,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let compose = self.compose.as_ref()?;
        if compose.mode == Mode::Window || compose.closing {
            return None;
        }
        self.render_compose_dialog(th, window, cx)
    }

    /// Enter on a dialog without a field: its main button.
    fn answer_compose_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        let Some(popup) = self.compose.as_ref().and_then(|c| c.popup.clone()) else {
            return false;
        };
        match popup {
            Popup::PlainText => self.make_plain(window, cx),
            Popup::SendCheck {
                check,
                at,
                archive,
                passed,
            } => self.send_compose(at, archive, passed.with(check), window, cx),
            Popup::DriveShare {
                at,
                archive,
                passed,
                ..
            } => self.share_with_link_and_send(at, archive, passed, window, cx),
            Popup::BadAddress { field, .. } => self.fix_bad_address(field, window, cx),
            _ => return false,
        }
        true
    }

    pub(super) fn dialog_buttons(
        &self,
        th: &Theme,
        ok: String,
        cx: &mut Context<Self>,
        on_ok: impl Fn(&mut Self, &mut Window, &mut Context<Self>) + 'static,
    ) -> gpui::Div {
        div()
            .mt(px(20.0))
            .flex()
            .flex_row()
            .justify_end()
            .gap(px(8.0))
            .child(
                div()
                    .id("dialog-cancel")
                    .h(px(36.0))
                    .px(px(16.0))
                    .flex()
                    .items_center()
                    .rounded_full()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgba(th.accent))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(th.hover)))
                    .on_click(cx.listener(|this, _, window, cx| this.close_popup(window, cx)))
                    .child(tr!("compose-tool-cancel")),
            )
            .child(
                filled_button("dialog-ok", ok, th)
                    .on_click(cx.listener(move |this, _, window, cx| on_ok(this, window, cx))),
            )
    }

    pub(super) fn dialog_card(th: &Theme, width: f32, title: impl Into<SharedString>) -> gpui::Div {
        div()
            .w(px(width))
            .max_w_full()
            .p(px(24.0))
            .flex()
            .flex_col()
            .rounded(px(DIALOG_RADIUS))
            .map(|d| crate::widgets::frosted(d, th, th.menu, DIALOG_RADIUS))
            .shadow(crate::widgets::elevation(th, 3.0))
            .text_color(rgba(th.text))
            .child(div().mb(px(16.0)).text_size(px(20.0)).child(title.into()))
    }

    fn render_link_dialog(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let field = |label: String, input: &Entity<TextInput>| {
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .mb(px(12.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgba(th.text_dim))
                        .child(label),
                )
                .child(
                    div()
                        .h(px(40.0))
                        .px(px(12.0))
                        .flex()
                        .items_center()
                        .rounded(px(6.0))
                        .border_1()
                        .border_color(rgba(th.divider))
                        .text_size(px(14.0))
                        .child(input.clone()),
                )
        };
        Self::dialog_card(
            th,
            400.0,
            if compose.dialog.editing_link {
                tr!("compose-tool-edit-link")
            } else {
                tr!("compose-tool-insert-link")
            },
        )
        .child(field(
            tr!("compose-tool-link-text"),
            &compose.dialog.link_text,
        ))
        .child(field(
            tr!("compose-tool-link-address"),
            &compose.dialog.link_url,
        ))
        .child(
            div()
                .text_size(px(12.0))
                .text_color(rgba(th.text_faint))
                .child(tr!("compose-tool-link-hint")),
        )
        .child(
            self.dialog_buttons(th, tr!("compose-tool-ok"), cx, |this, window, cx| {
                this.apply_link(window, cx)
            }),
        )
        .into_any_element()
    }

    fn render_plain_dialog(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        Self::dialog_card(th, 380.0, tr!("compose-tool-plain-title"))
            .child(
                div()
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text_dim))
                    .child(tr!("compose-tool-plain-text")),
            )
            .child(self.dialog_buttons(
                th,
                tr!("compose-tool-plain-switch"),
                cx,
                |this, window, cx| this.make_plain(window, cx),
            ))
            .into_any_element()
    }

    /// Asks before sending a message that speaks of an attachment without
    /// one, or has no subject.
    fn render_send_check(
        &self,
        check: SendCheck,
        at: Option<jiff::Timestamp>,
        archive: bool,
        passed: Passed,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let (title, text, fix) = match check {
            SendCheck::Attachment => (
                tr!("send-check-attachment-title"),
                tr!("send-check-attachment-text"),
                tr!("send-check-attach"),
            ),
            SendCheck::Subject => (
                tr!("send-check-subject-title"),
                tr!("send-check-subject-text"),
                tr!("send-check-add-subject"),
            ),
        };
        Self::dialog_card(th, 380.0, title)
            .child(
                div()
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text_dim))
                    .child(text),
            )
            .child(
                div()
                    .mt(px(20.0))
                    .flex()
                    .flex_row()
                    .justify_end()
                    .gap(px(8.0))
                    .child(
                        div()
                            .id("send-check-fix")
                            .h(px(36.0))
                            .px(px(16.0))
                            .flex()
                            .items_center()
                            .rounded_full()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgba(th.accent))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .on_click(cx.listener(move |this, _, window, cx| match check {
                                SendCheck::Attachment => this.pick_files(false, cx),
                                SendCheck::Subject => {
                                    if let Some(c) = &mut this.compose {
                                        c.popup = None;
                                        window.focus(&c.subject.focus_handle(cx), cx);
                                    }
                                    cx.notify();
                                }
                            }))
                            .child(fix),
                    )
                    .child(
                        filled_button("send-check-send", tr!("send-check-send-anyway"), th)
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.send_compose(at, archive, passed.with(check), window, cx)
                            })),
                    ),
            )
            .into_any_element()
    }

    fn render_time_picker(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        let today = jiff::Timestamp::now().to_zoned(self.tz.clone()).date();
        let (month, day) = (compose.dialog.month, compose.dialog.day);
        let days = schedule::month_grid(month, format::first_weekday())
            .into_iter()
            .enumerate()
            .map(|(ix, date)| {
                let past = date < today;
                let selected = date == day;
                let other = date.month() != month.month();
                div()
                    .id(("pick-day", ix))
                    .size(px(36.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .text_size(px(13.0))
                    .when(other, |d| d.text_color(rgba(th.text_faint)))
                    .when(past, |d| d.opacity(0.38))
                    .when(date == today && !selected, |d| {
                        d.border_1().border_color(rgba(th.accent))
                    })
                    .when(selected, |d| {
                        d.bg(rgba(th.accent)).text_color(rgba(th.on_accent))
                    })
                    .when(!past, |d| {
                        d.cursor_pointer()
                            .hover(|s| s.bg(rgba(th.hover)))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if let Some(c) = &mut this.compose {
                                    c.dialog.day = date;
                                    c.dialog.month = date;
                                }
                                cx.notify();
                            }))
                    })
                    .child(format::number(date.day() as u64))
            })
            .collect::<Vec<_>>();
        let step = |months: i32| {
            move |this: &mut Self, _: &gpui::ClickEvent, _: &mut Window, cx: &mut Context<Self>| {
                if let Some(c) = &mut this.compose
                    && let Ok(m) = c
                        .dialog
                        .month
                        .first_of_month()
                        .checked_add(jiff::Span::new().months(months))
                {
                    c.dialog.month = m;
                }
                cx.notify();
            }
        };
        let weekdays = format::weekdays_short().into_iter().map(|(_, d)| {
            div()
                .size(px(36.0))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(12.0))
                .text_color(rgba(th.text_dim))
                .child(d)
        });
        let (prev, next) = (cx.listener(step(-1)), cx.listener(step(1)));
        Self::dialog_card(th, 330.0, tr!("schedule-pick"))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .child(format::month_year(month)),
                    )
                    .child(
                        icon_button("pick-prev", "chevron-left", 20.0, th)
                            .size(px(32.0))
                            .on_click(prev),
                    )
                    .child(
                        icon_button("pick-next", "chevron-right", 20.0, th)
                            .size(px(32.0))
                            .on_click(next),
                    ),
            )
            .child(
                div()
                    .mt(px(8.0))
                    .w(px(7.0 * 40.0))
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap_x(px(4.0))
                    .children(weekdays)
                    .children(days),
            )
            .child(
                div()
                    .mt(px(12.0))
                    .flex()
                    .flex_row()
                    .gap(px(12.0))
                    .child(
                        div()
                            .flex_1()
                            .h(px(40.0))
                            .px(px(12.0))
                            .flex()
                            .items_center()
                            .rounded(px(6.0))
                            .border_1()
                            .border_color(rgba(th.divider))
                            .text_size(px(14.0))
                            .child(format::day_month_year(
                                day.to_datetime(jiff::civil::Time::midnight()),
                            )),
                    )
                    .child(
                        div()
                            .w(px(110.0))
                            .h(px(40.0))
                            .px(px(12.0))
                            .flex()
                            .items_center()
                            .rounded(px(6.0))
                            .border_1()
                            .border_color(rgba(th.accent))
                            .text_size(px(14.0))
                            .child(compose.dialog.time.clone()),
                    ),
            )
            .child(
                self.dialog_buttons(th, tr!("schedule-send"), cx, |this, window, cx| {
                    this.schedule_picked(window, cx)
                }),
            )
            .into_any_element()
    }

    // Signature.

    pub(super) fn render_signature_button(&self, th: &Theme, cx: &mut Context<Self>) -> AnyElement {
        let Some(compose) = &self.compose else {
            return div().into_any_element();
        };
        div()
            .relative()
            .child(
                icon_button("compose-signature", "signature", 20.0, th)
                    .tooltip(tip(tr!("compose-tool-signature"), th))
                    .on_click(
                        cx.listener(|this, _, _, cx| this.toggle_popup(Popup::Signature, cx)),
                    ),
            )
            .when(compose.popup == Some(Popup::Signature), |d| {
                d.child(above(self.signature_menu(th, cx)))
            })
            .into_any_element()
    }

    /// The signatures to sign with, None, and Manage.
    pub(super) fn signature_menu(&self, th: &Theme, cx: &mut Context<Self>) -> gpui::Div {
        let current = self.compose.as_ref().and_then(|c| c.signature);
        let item = |ix: usize, id: Option<u32>, label: &str| {
            menu_item(("compose-signature-item", ix), label, th)
                .gap(px(12.0))
                .child(div().flex_1())
                .when(current == id, |d| d.child(icon("check", th.text_dim, 18.0)))
                .on_click(cx.listener(move |this, _, _, cx| this.choose_signature(id, cx)))
        };
        let items = self
            .config
            .sending
            .signatures
            .iter()
            .enumerate()
            .map(|(ix, s)| {
                let name = if s.name.trim().is_empty() {
                    tr!("compose-tool-signature-untitled")
                } else {
                    s.name.clone()
                };
                item(ix + 1, Some(s.id), &name)
            })
            .collect::<Vec<_>>();
        menu(th)
            .w(px(240.0))
            .child(item(0, None, &tr!("compose-tool-signature-none")))
            .children(items)
            .child(menu_divider(th))
            .child(
                menu_item(
                    "compose-signatures-manage",
                    &tr!("compose-tool-signature-manage"),
                    th,
                )
                .on_click(cx.listener(|this, _, window, cx| {
                    if let Some(c) = &mut this.compose {
                        c.popup = None;
                    }
                    this.open_settings_page(
                        super::super::settings_page::Section::Signatures,
                        window,
                        cx,
                    );
                })),
            )
    }
}

/// Marks the lines from `-- ` on as the signature.
fn mark_signature(doc: &mut katna_ui::rich::Doc) {
    let mut on = false;
    for block in &mut doc.blocks {
        if let katna_ui::rich::Block::Para(p) = block {
            if p.text == "-- " {
                on = true;
            } else if on && p.style.quote > 0 {
                on = false;
            }
            p.style.signature = on;
        }
    }
}

#[derive(Clone, Copy)]
enum Clip {
    Cut,
    Copy,
    Paste,
    SelectAll,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_get_a_scheme() {
        assert_eq!(normalize_url("example.com/a"), "https://example.com/a");
        assert_eq!(normalize_url("http://x.org"), "http://x.org");
        assert_eq!(normalize_url("kay@enron.com"), "mailto:kay@enron.com");
        assert_eq!(
            normalize_url("mailto:kay@enron.com"),
            "mailto:kay@enron.com"
        );
        assert_eq!(normalize_url(""), "");
    }

    #[test]
    fn finds_emoji() {
        assert!(
            emoji_list("thumbs up", 0)
                .iter()
                .any(|e| e.as_str() == "👍")
        );
        assert_eq!(emoji_list("", 0)[0].as_str(), "😀");
        assert!(emoji_list("zzzzqqq", 0).is_empty());
    }
}
