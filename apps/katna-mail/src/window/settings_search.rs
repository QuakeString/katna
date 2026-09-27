// SPDX-License-Identifier: GPL-3.0-or-later

//! Searching the Settings page from the top bar's search box, which
//! searches settings instead of mail while the page is open. A result opens
//! its tab, scrolls its row into view and lights the row up for a moment.
//! Also the pages of the tabs that are still to come.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use gpui::{
    Animation, AnimationExt, AnyElement, Context, Div, FontWeight, SharedString, Window, canvas,
    div, point, prelude::*, rgba,
};
use katna_ui::px;
use katna_ui::{InputEvent, Ripple};

use super::MailWindow;
use super::keymap::SHORTCUTS;
use super::settings_page::{Section, setting_row};
use crate::theme::{Theme, fade};

/// How long a row found by a search stays lit.
const FLASH: Duration = Duration::from_millis(1800);

/// A setting a search can find: the row's name on its tab, a line on it and
/// more words it answers to.
struct Entry {
    section: Section,
    title: &'static str,
    detail: &'static str,
    words: &'static str,
}

const fn entry(
    section: Section,
    title: &'static str,
    detail: &'static str,
    words: &'static str,
) -> Entry {
    Entry {
        section,
        title,
        detail,
        words,
    }
}

/// Every row of every tab. The titles are the rows' names, so a result can
/// find its row.
const ENTRIES: &[Entry] = &[
    entry(
        Section::General,
        "Language",
        "Language of the app, dates and numbers",
        "language translation locale english hindi bengali arabic system",
    ),
    entry(
        Section::General,
        "Conversation view",
        "Group replies to the same mail",
        "threads threading group",
    ),
    entry(
        Section::General,
        "Time",
        "12-hour or 24-hour clock, or as the language writes it",
        "clock 24-hour 12-hour am pm time format hours",
    ),
    entry(
        Section::General,
        "Reading",
        "Newest message first, full headers, full names of recipients",
        "order oldest descending chronological reverse headers details from to cc names recipients first last",
    ),
    entry(
        Section::General,
        "Mark as read",
        "When an opened conversation is marked read: at once, after 1 or 3 seconds, or by hand",
        "read unread seen delay mark",
    ),
    entry(
        Section::General,
        "Reply button",
        "The reply button beside each message replies to everyone",
        "reply all default behaviour behavior",
    ),
    entry(
        Section::General,
        "Images from the web",
        "Always show the images of every message",
        "remote images pictures load external content tracking privacy",
    ),
    entry(
        Section::General,
        "Sending",
        "Undo send: how long a sent message waits, so it can be taken back",
        "undo send delay cancel",
    ),
    entry(
        Section::General,
        "Offline mail",
        "How many days of recent mail are downloaded whole, to read without a connection",
        "offline download sync days cache disk storage",
    ),
    entry(
        Section::General,
        "Notifications",
        "New-mail notifications and their sound",
        "notify alert sound chime popup new mail",
    ),
    entry(
        Section::General,
        "Desktop",
        "Open Katna Mail at login, the system tray icon and the unread count on the taskbar icon",
        "tray badge unread count taskbar dock panel startup start login autostart launch boot",
    ),
    entry(
        Section::General,
        "Default mail app",
        "Open email links from other apps and websites in Katna Mail",
        "default client mailto links handler email program",
    ),
    entry(
        Section::Inbox,
        "Inbox tabs",
        "Sort the inbox into tabs, as your mail provider's website does",
        "primary promotions social updates forums focused other categories",
    ),
    entry(
        Section::Accounts,
        "Folder pane",
        "Which accounts' folders the pane on the left shows",
        "one account all accounts switch",
    ),
    entry(
        Section::Accounts,
        "Accounts",
        "Add or remove an account, or change its picture",
        "add remove delete account picture photo avatar",
    ),
    entry(
        Section::Accounts,
        "Delete all data",
        "Start over, as on a new install",
        "reset wipe erase remove everything",
    ),
    entry(
        Section::Appearance,
        "Reading pane",
        "Where an opened conversation shows",
        "split preview right no split layout panes",
    ),
    entry(
        Section::Appearance,
        "Density",
        "Default or compact lines in the list",
        "compact spacing comfortable",
    ),
    entry(
        Section::Appearance,
        "Scaling",
        "Make everything bigger or smaller: text, icons, spacing and dividers",
        "scale zoom size bigger smaller larger font text dpi magnify",
    ),
    entry(
        Section::Appearance,
        "Theme",
        "Same as the desktop, light or dark",
        "dark mode light mode night",
    ),
    entry(
        Section::Appearance,
        "Desktop colors",
        "The color scheme and accent color of the desktop",
        "accent colour color scheme",
    ),
    entry(
        Section::Appearance,
        "App names",
        "Names under the app icons at the far left",
        "rail labels",
    ),
    entry(
        Section::Appearance,
        "Sender pictures",
        "Company logos, looked up by the sender's domain",
        "logo avatar picture image photo",
    ),
    entry(
        Section::Appearance,
        "Important markers",
        "The Important marker beside each message in the list",
        "important label chevron flag priority",
    ),
    entry(
        Section::Appearance,
        "Message width",
        "Limit the width of messages",
        "narrow wide lines readable column",
    ),
    entry(
        Section::Appearance,
        "Mail colors",
        "Dark colors for HTML mail in a dark theme, or its sender's colors",
        "dark mode night html colors colours invert",
    ),
    entry(
        Section::Appearance,
        "Attachment previews",
        "A small picture of each attachment's content",
        "thumbnails attachments files preview",
    ),
    entry(
        Section::Shortcuts,
        "Shortcut set",
        "Start from the keys of Gmail, Inbox by Gmail, Apple Mail, Outlook or Thunderbird",
        "keyboard keys hotkeys keymap preset outlook thunderbird apple gmail inbox restore defaults",
    ),
    entry(
        Section::Shortcuts,
        "Single-key shortcuts",
        "Keys without Ctrl or Alt, as in webmail",
        "keyboard keys hotkeys",
    ),
    entry(
        Section::DefaultApps,
        "PDF files",
        "Where PDF attachments open",
        "open attachment viewer app pdf",
    ),
    entry(
        Section::DefaultApps,
        "Pictures",
        "Where photos and pictures open",
        "open attachment viewer app image photo png jpeg",
    ),
    entry(
        Section::DefaultApps,
        "Text files",
        "Where plain text, logs and code open",
        "open attachment viewer app txt",
    ),
    entry(
        Section::DefaultApps,
        "Spreadsheets",
        "Where Excel, OpenDocument and CSV files open",
        "open attachment viewer app xlsx xls ods csv",
    ),
    entry(
        Section::DefaultApps,
        "Documents",
        "Where Word and OpenDocument text open",
        "open attachment viewer app docx odt word",
    ),
    entry(
        Section::DefaultApps,
        "After saving",
        "Show saved attachments in their folder",
        "save download folder file manager reveal show dolphin",
    ),
    entry(
        Section::Signatures,
        "Send new messages from",
        "The account new mail goes out from: the one you are in, or always the same one",
        "from sender default account address identity",
    ),
    entry(
        Section::Signatures,
        "Send on replies",
        "Send, or Send and archive the conversation, on replies and forwards",
        "send archive default behavior behaviour reply forward",
    ),
    entry(
        Section::Signatures,
        "Signatures",
        "Added below your message, after a \u{201c}--\u{201d} line",
        "signature sign-off",
    ),
    entry(
        Section::Signatures,
        "For new mail",
        "The signature new mail starts with",
        "default signature",
    ),
    entry(
        Section::Signatures,
        "For replies and forwards",
        "The signature replies and forwards start with",
        "default signature reply forward",
    ),
    entry(
        Section::Signatures,
        "Format",
        "Write new mail in plain text",
        "plain text html rich formatting",
    ),
    entry(
        Section::Signatures,
        "Spelling",
        "Check spelling while writing, and the dictionary's language",
        "spell check spellcheck dictionary language hunspell typos",
    ),
    entry(
        Section::Signatures,
        "Templates",
        "Coming soon: save mail you write often, and start new mail or a reply from it",
        "template canned reply snippet",
    ),
    entry(
        Section::Feedback,
        "Crash reports",
        "Save crash reports on this computer when Katna Mail or its background service crashes",
        "crash report bug panic traceback stack privacy",
    ),
    entry(
        Section::Feedback,
        "Saved crash reports",
        "View, copy or delete the crash reports saved on this computer",
        "crash report bug delete copy view",
    ),
    entry(
        Section::Feedback,
        "Help improve Katna",
        "Send crash reports to help fix what went wrong; off unless you turn it on",
        "telemetry analytics anonymous sentry send share privacy opt in improve",
    ),
    entry(
        Section::Experimental,
        "Window frame",
        "Who draws the title bar, the window buttons, the corners and the shadow",
        "decoration csd title bar look feel",
    ),
    entry(
        Section::Experimental,
        "Blurred background",
        "The desktop shows through the top bar, blurred, and menus are frosted",
        "blur transparency frosted glass look feel",
    ),
];

/// What a tab that is still to come will do.
fn coming(section: Section) -> Option<&'static str> {
    Some(match section {
        Section::Subscriptions => {
            "See the newsletters and mailing lists you get, and unsubscribe in one click."
        }
        Section::MailRules => {
            "Create, rename, move and hide folders and labels, and choose which ones sync. \
             Rules sort, label, forward or delete new mail by itself, by sender, subject or words."
        }
        Section::McpServer => {
            "Let AI assistants on this computer search, read and draft your mail, with your say."
        }
        _ => return None,
    })
}

/// More words a tab is found by, besides its name and its line.
fn tab_words(section: Section) -> &'static str {
    match section {
        Section::MailRules => "mail rules filters folders labels",
        Section::Signatures => "signature templates write",
        Section::Feedback => "crash report feedback privacy anonymous sentry telemetry",
        _ => "",
    }
}

/// A result: a row of a tab, or the tab itself.
#[derive(Clone)]
struct Found {
    section: Section,
    title: SharedString,
    detail: SharedString,
    /// The row to light up; `None` for a tab.
    row: Option<SharedString>,
}

/// The settings that have every word of `query`, the best first: those
/// whose name starts with it, then whose name has it, then the rest.
fn search(query: &str) -> Vec<Found> {
    let words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    if words.is_empty() {
        return Vec::new();
    }
    let rows = ENTRIES.iter().map(|e| {
        (
            Found {
                section: e.section,
                title: e.title.into(),
                detail: e.detail.into(),
                row: Some(e.title.into()),
            },
            e.words,
        )
    });
    let shortcuts = SHORTCUTS.iter().map(|s| {
        (
            Found {
                section: Section::Shortcuts,
                title: s.label.into(),
                detail: "Keyboard shortcut".into(),
                row: Some(s.label.into()),
            },
            "keyboard key shortcut",
        )
    });
    let tabs = Section::ALL.into_iter().map(|section| {
        (
            Found {
                section,
                title: section.label().into(),
                detail: coming(section)
                    .map_or("Settings tab", |_| "Coming soon")
                    .into(),
                row: None,
            },
            tab_words(section),
        )
    });
    let query = words.join(" ");
    let mut found: Vec<(u8, Found)> = rows
        .chain(shortcuts)
        .chain(tabs)
        .filter_map(|(found, extra)| {
            let title = found.title.to_lowercase();
            let text = format!(
                "{title} {} {extra} {}",
                found.detail.to_lowercase(),
                found.section.label().to_lowercase()
            );
            if !words.iter().all(|w| text.contains(w.as_str())) {
                return None;
            }
            let rank = if title.starts_with(&query) {
                0
            } else if title.contains(&query) {
                1
            } else {
                2
            };
            Some((rank, found))
        })
        .collect();
    // A tab and its row of the same name: the row, which says more.
    let mut seen = Vec::new();
    found.retain(|(_, f)| {
        let key = (f.section, f.title.clone());
        let new = !seen.contains(&key);
        seen.push(key);
        new
    });
    found.sort_by_key(|(rank, _)| *rank);
    found.into_iter().map(|(_, f)| f).collect()
}

/// A row found by a search, lit up once.
pub(super) struct Flash {
    section: Section,
    row: SharedString,
    seq: usize,
    /// Frames left to bring the row into view; two, as the first frame of
    /// a tab still has the old tab's scroll size.
    reveal: Rc<Cell<u8>>,
}

impl MailWindow {
    /// The search box while the Settings page is open.
    pub(super) fn on_settings_search(
        &mut self,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let text = self.search.read(cx).text().trim().to_owned();
        match event {
            InputEvent::Changed => {
                if let Some(page) = &mut self.settings_page {
                    page.query = text.into();
                    page.scroll.set_offset(point(px(0.0), px(0.0)));
                }
                cx.notify();
            }
            InputEvent::Submit => {
                if let Some(first) = search(&text).into_iter().next() {
                    self.go_to_setting(first, window, cx);
                }
            }
            InputEvent::Cancel => {
                if text.is_empty() {
                    if let Some(page) = &self.settings_page {
                        window.focus(&page.focus, cx);
                    }
                } else {
                    self.search.update(cx, |search, cx| search.set_text("", cx));
                }
            }
        }
    }

    /// The search box searches settings while the Settings page is open,
    /// and mail again once it closes. The mail search is put back if its
    /// results are still what the list shows.
    pub(super) fn sync_search_box(&mut self, cx: &mut Context<Self>) {
        let open = self.settings_page.is_some();
        if open == self.mail_query.is_some() {
            return;
        }
        if open {
            self.search_panel = None;
            self.mail_query = Some(self.search.read(cx).text().to_owned());
            self.search.update(cx, |search, cx| {
                search.set_placeholder(katna_i18n::tr!("search-settings"));
                search.set_text("", cx);
            });
        } else {
            let query = self.mail_query.take().unwrap_or_default();
            let searching = matches!(self.listing, Some(super::Listing::Search { .. }));
            self.search.update(cx, |search, cx| {
                search.set_placeholder(katna_i18n::tr!("search-mail"));
                search.set_text(if searching { query } else { String::new() }, cx);
            });
        }
    }

    /// Opens the tab of a result and lights up its row.
    fn go_to_setting(&mut self, found: Found, window: &mut Window, cx: &mut Context<Self>) {
        self.search.update(cx, |search, cx| search.set_text("", cx));
        self.open_settings_page(found.section, window, cx);
        self.flash_seq += 1;
        let seq = self.flash_seq;
        if let Some(page) = &mut self.settings_page {
            page.query = SharedString::default();
            page.flash = found.row.map(|row| Flash {
                section: found.section,
                row,
                seq,
                reveal: Rc::new(Cell::new(2)),
            });
            window.focus(&page.focus, cx);
        }
        cx.notify();
    }

    /// A setting's row on the Settings page; see [`setting_row`].
    pub(super) fn row(
        &self,
        label: impl Into<SharedString>,
        detail: Option<&'static str>,
        content: impl IntoElement,
        th: &Theme,
    ) -> Div {
        let label = label.into();
        let flash = self.flash_mark(&label, th);
        let info = self
            .settings_page
            .as_ref()
            .map(|p| p.info.clone())
            .unwrap_or_default();
        setting_row(label, detail, content, &info, flash, th)
    }

    /// Under the row named `label` when a search has just led to it: a
    /// tint that fades, and the row scrolled into view.
    pub(super) fn flash_mark(&self, label: &str, th: &Theme) -> Option<AnyElement> {
        let page = self.settings_page.as_ref()?;
        let flash = page.flash.as_ref()?;
        if flash.section != page.section || flash.row != label {
            return None;
        }
        let scroll = page.scroll.clone();
        let reveal = flash.reveal.clone();
        let tint = th.accent;
        Some(
            div()
                .absolute()
                .top(px(4.0))
                .bottom(px(4.0))
                .left(px(-12.0))
                .right(px(-12.0))
                .rounded(px(12.0))
                .child(
                    canvas(
                        move |bounds, window, _| {
                            if reveal.get() == 0 {
                                return;
                            }
                            reveal.set(reveal.get() - 1);
                            // The row's top a little below the top of the
                            // page's scrolling part.
                            let view = scroll.bounds();
                            let offset = scroll.offset();
                            let max = scroll.max_offset().y;
                            let y = (offset.y - (bounds.top() - view.top() - px(24.0)))
                                .min(px(0.0))
                                .max(-max);
                            scroll.set_offset(point(offset.x, y));
                            window.request_animation_frame();
                        },
                        |_, _, _, _| {},
                    )
                    .size_full(),
                )
                .with_animation(
                    ("settings-flash", flash.seq),
                    Animation::new(FLASH),
                    move |el, t| {
                        // Holds, then fades.
                        let left = ((1.0 - t) / 0.6).min(1.0);
                        el.bg(rgba(fade(tint, 0.16 * left)))
                    },
                )
                .into_any_element(),
        )
    }

    /// The results of the search box, in place of the open tab.
    pub(super) fn render_settings_results(
        &self,
        query: &str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let found = search(query);
        let empty = found.is_empty();
        let rows = found.into_iter().enumerate().map(|(ix, found)| {
            let title = found.title.clone();
            let place = format!("{} \u{b7} {}", found.section.label(), found.detail);
            div()
                .id(("settings-result", ix))
                .relative()
                .overflow_hidden()
                .px(px(12.0))
                .py(px(10.0))
                .flex()
                .flex_col()
                .gap(px(2.0))
                .rounded(px(8.0))
                .cursor_pointer()
                .hover(|d| d.bg(rgba(th.hover)))
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.go_to_setting(found.clone(), window, cx)
                }))
                .child(Ripple::new(("settings-result-ripple", ix), rgba(th.ripple)).rounded(8.0))
                .child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::MEDIUM)
                        .child(title),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .line_height(px(17.0))
                        .text_color(rgba(th.text_faint))
                        .child(place),
                )
        });
        div()
            .pt(px(12.0))
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(
                div()
                    .pb(px(8.0))
                    .text_size(px(13.0))
                    .text_color(rgba(th.text_dim))
                    .child(if empty {
                        format!("No settings match \u{201c}{query}\u{201d}.")
                    } else {
                        format!("Settings that match \u{201c}{query}\u{201d}")
                    }),
            )
            .children(rows)
            .into_any_element()
    }

    /// The page of a tab still to come.
    pub(super) fn coming_soon_section(&self, section: Section, th: &Theme) -> AnyElement {
        div()
            .pt(px(40.0))
            .flex()
            .flex_col()
            .items_start()
            .gap(px(12.0))
            .child(coming_pill(th))
            .child(div().text_size(px(20.0)).child(section.label()))
            .child(
                div()
                    .text_size(px(14.0))
                    .line_height(px(20.0))
                    .text_color(rgba(th.text_dim))
                    .child(coming(section).unwrap_or_default()),
            )
            .into_any_element()
    }
}

/// The "Coming soon" pill.
pub(super) fn coming_pill(th: &Theme) -> Div {
    div()
        .flex_none()
        .px(px(10.0))
        .h(px(24.0))
        .flex()
        .items_center()
        .rounded_full()
        .bg(rgba(th.nav_selected))
        .text_color(rgba(th.nav_selected_text))
        .text_size(px(12.0))
        .font_weight(FontWeight::SEMIBOLD)
        .child("Coming soon")
}

/// Whether `section` is a tab still to come.
pub(super) fn is_coming(section: Section) -> bool {
    coming(section).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_rows_tabs_and_shortcuts() {
        let titles = |q: &str| {
            search(q)
                .into_iter()
                .map(|f| f.title.to_string())
                .collect::<Vec<_>>()
        };
        assert_eq!(titles("dark")[0], "Theme");
        assert_eq!(titles("reading pane")[0], "Reading pane");
        assert!(titles("undo").contains(&"Sending".to_owned()));
        assert!(titles("rules").contains(&"Folders & rules".to_owned()));
        assert!(titles("folders").contains(&"Folders & rules".to_owned()));
        assert!(titles("template").contains(&"Templates".to_owned()));
        assert!(titles("archive").iter().any(|t| t.contains("Archive")));
        assert!(titles("sentry").contains(&"User feedback".to_owned()));
        assert!(titles("zzzz").is_empty());
        assert!(titles("  ").is_empty());
        // The Accounts tab and its Accounts row come once.
        assert_eq!(
            titles("accounts")
                .iter()
                .filter(|t| t.as_str() == "Accounts")
                .count(),
            1
        );
    }
}
