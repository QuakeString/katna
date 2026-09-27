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
use katna_i18n::tr;
use katna_ui::px;
use katna_ui::{InputEvent, Ripple};

use super::MailWindow;
use super::keymap::SHORTCUTS;
use super::settings_page::{Section, setting_row};
use crate::theme::{Theme, fade};

/// How long a row found by a search stays lit.
const FLASH: Duration = Duration::from_millis(1800);

/// A setting a search can find: the message ids of the row's name on its
/// tab (the same as the row's own) and of a line on it, and more words it
/// answers to, in English.
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

/// Every row of every tab. The titles are the ids of the rows' names, so a
/// result can find its row.
const ENTRIES: &[Entry] = &[
    entry(
        Section::General,
        "language-setting",
        "settings-general-language-summary",
        "language translation locale english hindi bengali arabic system",
    ),
    entry(
        Section::General,
        "settings-general-conversations",
        "settings-general-conversations-group",
        "threads threading group",
    ),
    entry(
        Section::General,
        "settings-time",
        "settings-time-summary",
        "clock 24-hour 12-hour am pm time format hours",
    ),
    entry(
        Section::General,
        "settings-general-reading",
        "settings-general-reading-summary",
        "order oldest descending chronological reverse headers details from to cc names recipients first last",
    ),
    entry(
        Section::General,
        "settings-translation",
        "settings-translation-summary",
        "translate translation language foreign reading libretranslate always never",
    ),
    entry(
        Section::General,
        "settings-general-mark-read",
        "settings-general-mark-read-summary",
        "read unread seen delay mark",
    ),
    entry(
        Section::General,
        "settings-general-auto-advance",
        "settings-general-auto-advance-summary",
        "auto advance next previous older newer after delete archive move back list",
    ),
    entry(
        Section::General,
        "settings-general-reply-button",
        "settings-general-reply-button-summary",
        "reply all default behaviour behavior",
    ),
    entry(
        Section::General,
        "settings-general-remote-images",
        "settings-general-remote-images-summary",
        "remote images pictures load external content tracking privacy",
    ),
    entry(
        Section::General,
        "settings-general-sending",
        "settings-general-sending-summary",
        "undo send delay cancel",
    ),
    entry(
        Section::General,
        "settings-general-offline",
        "settings-general-offline-summary",
        "offline download sync days cache disk storage",
    ),
    entry(
        Section::General,
        "settings-general-notifications",
        "settings-general-notifications-summary",
        "notify alert sound chime popup new mail",
    ),
    entry(
        Section::General,
        "settings-general-reset-cache",
        "settings-general-reset-cache-summary",
        "cache clear local data storage disk space rebuild index redownload fix",
    ),
    entry(
        Section::General,
        "settings-general-desktop",
        "settings-general-desktop-summary",
        "tray badge unread count taskbar dock panel startup start login autostart launch boot",
    ),
    entry(
        Section::General,
        "settings-general-mail-app",
        "settings-general-mail-app-summary",
        "default client mailto links handler email program",
    ),
    entry(
        Section::Inbox,
        "settings-inbox-tabs",
        "settings-inbox-tabs-detail",
        "primary promotions social updates forums focused other categories",
    ),
    entry(
        Section::Accounts,
        "accounts-folder-pane",
        "accounts-folder-pane-detail",
        "one account all accounts switch",
    ),
    entry(
        Section::Accounts,
        "accounts-unified",
        "accounts-unified-switch-detail",
        "unified inbox all accounts combined merged together",
    ),
    entry(
        Section::Accounts,
        "accounts-row",
        "settings-accounts-accounts-summary",
        "add remove delete account picture photo avatar",
    ),
    entry(
        Section::Accounts,
        "accounts-delete-all-row",
        "accounts-delete-all-row-detail",
        "reset wipe erase remove everything",
    ),
    entry(
        Section::Appearance,
        "settings-appearance-reading-pane",
        "settings-appearance-reading-pane-detail",
        "split preview right no split layout panes",
    ),
    entry(
        Section::Appearance,
        "settings-appearance-density",
        "settings-appearance-density-summary",
        "compact spacing comfortable",
    ),
    entry(
        Section::Appearance,
        "settings-appearance-scaling",
        "settings-appearance-scaling-summary",
        "scale zoom size bigger smaller larger font text dpi magnify",
    ),
    entry(
        Section::Appearance,
        "settings-appearance-theme",
        "settings-appearance-theme-summary",
        "dark mode light mode night",
    ),
    entry(
        Section::Appearance,
        "settings-appearance-desktop-colors",
        "settings-appearance-desktop-colors-use-detail",
        "accent colour color scheme",
    ),
    entry(
        Section::Appearance,
        "settings-appearance-app-names",
        "settings-appearance-app-names-show-detail",
        "rail labels",
    ),
    entry(
        Section::Appearance,
        "settings-appearance-sender-pictures",
        "settings-appearance-sender-pictures-summary",
        "logo avatar picture image photo",
    ),
    entry(
        Section::Appearance,
        "settings-appearance-important",
        "settings-appearance-important-summary",
        "important label chevron flag priority",
    ),
    entry(
        Section::Appearance,
        "settings-appearance-message-width",
        "settings-appearance-message-width-limit",
        "narrow wide lines readable column",
    ),
    entry(
        Section::Appearance,
        "settings-appearance-mail-colors",
        "settings-appearance-mail-colors-summary",
        "dark mode night html colors colours invert",
    ),
    entry(
        Section::Appearance,
        "settings-appearance-attachment-previews",
        "settings-appearance-attachment-previews-summary",
        "thumbnails attachments files preview",
    ),
    entry(
        Section::Shortcuts,
        "settings-shortcuts-set",
        "settings-shortcuts-set-summary",
        "keyboard keys hotkeys keymap preset outlook thunderbird apple gmail inbox restore defaults",
    ),
    entry(
        Section::Shortcuts,
        "settings-shortcuts-single",
        "settings-shortcuts-single-summary",
        "keyboard keys hotkeys",
    ),
    entry(
        Section::DefaultApps,
        "settings-default-apps-pdf",
        "settings-default-apps-pdf-summary",
        "open attachment viewer app pdf",
    ),
    entry(
        Section::DefaultApps,
        "settings-default-apps-pictures",
        "settings-default-apps-pictures-summary",
        "open attachment viewer app image photo png jpeg",
    ),
    entry(
        Section::DefaultApps,
        "settings-default-apps-text",
        "settings-default-apps-text-summary",
        "open attachment viewer app txt",
    ),
    entry(
        Section::DefaultApps,
        "settings-default-apps-sheets",
        "settings-default-apps-sheets-summary",
        "open attachment viewer app xlsx xls ods csv",
    ),
    entry(
        Section::DefaultApps,
        "settings-default-apps-documents",
        "settings-default-apps-documents-summary",
        "open attachment viewer app docx doc odt word pptx ppt odp powerpoint slides presentation",
    ),
    entry(
        Section::DefaultApps,
        "settings-default-apps-after-saving",
        "settings-default-apps-after-saving-summary",
        "save download folder file manager reveal show dolphin",
    ),
    entry(
        Section::Signatures,
        "settings-compose-send-from",
        "settings-compose-send-from-summary",
        "from sender default account address identity",
    ),
    entry(
        Section::Signatures,
        "settings-compose-send-on-replies",
        "settings-compose-send-on-replies-summary",
        "send archive default behavior behaviour reply forward",
    ),
    entry(
        Section::Signatures,
        "settings-compose-grammar",
        "settings-compose-grammar-summary",
        "grammar check harper english writing mistakes proofread",
    ),
    entry(
        Section::Signatures,
        "settings-compose-suggestions",
        "settings-compose-suggestions-summary",
        "writing suggestions autocomplete complete predict phrase ghost text tab smart compose",
    ),
    entry(
        Section::Signatures,
        "settings-compose-signatures",
        "settings-compose-signatures-summary",
        "signature sign-off",
    ),
    entry(
        Section::Signatures,
        "settings-compose-for-new-mail",
        "settings-compose-for-new-mail-summary",
        "default signature",
    ),
    entry(
        Section::Signatures,
        "settings-compose-for-replies",
        "settings-compose-for-replies-summary",
        "default signature reply forward",
    ),
    entry(
        Section::Signatures,
        "settings-compose-format",
        "settings-compose-format-summary",
        "plain text html rich formatting",
    ),
    entry(
        Section::Signatures,
        "settings-compose-spelling",
        "settings-compose-spelling-summary",
        "spell check spellcheck dictionary language hunspell typos",
    ),
    entry(
        Section::Signatures,
        "settings-compose-templates",
        "settings-compose-templates-summary",
        "template canned reply snippet",
    ),
    entry(
        Section::Feedback,
        "feedback-crash-reports",
        "settings-feedback-crash-reports-summary",
        "crash report bug panic traceback stack privacy",
    ),
    entry(
        Section::Feedback,
        "feedback-saved",
        "settings-feedback-saved-summary",
        "crash report bug delete copy view",
    ),
    entry(
        Section::Feedback,
        "feedback-help-improve",
        "settings-feedback-help-improve-summary",
        "telemetry analytics anonymous sentry send share privacy opt in improve",
    ),
    entry(
        Section::Experimental,
        "look-window-frame",
        "look-window-frame-detail",
        "decoration csd title bar look feel",
    ),
    entry(
        Section::Experimental,
        "look-blurred-background",
        "settings-experimental-blur-summary",
        "blur transparency frosted glass look feel",
    ),
];

/// What a tab that is still to come will do.
fn coming(section: Section) -> Option<String> {
    Some(match section {
        Section::Subscriptions => tr!("settings-tab-subscriptions-coming"),
        Section::MailRules => tr!("settings-tab-folders-rules-coming"),
        Section::McpServer => tr!("settings-tab-mcp-server-coming"),
        _ => return None,
    })
}

/// More words a tab is found by, besides its name and its line (English
/// for now).
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
/// whose name starts with it, then whose name has it, then the rest. Names
/// and lines are matched in the current language, the extra words in
/// English.
fn search(query: &str) -> Vec<Found> {
    let words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    if words.is_empty() {
        return Vec::new();
    }
    let rows = ENTRIES.iter().map(|e| {
        let title: SharedString = tr!(e.title).into();
        (
            Found {
                section: e.section,
                title: title.clone(),
                detail: tr!(e.detail).into(),
                row: Some(title),
            },
            // The English name and line too, so English words find a row
            // in any language.
            format!(
                "{} {} {}",
                katna_i18n::english(e.title),
                katna_i18n::english(e.detail),
                e.words
            ),
        )
    });
    let shortcuts = SHORTCUTS.iter().map(|s| {
        (
            Found {
                section: Section::Shortcuts,
                title: s.title().into(),
                detail: tr!("settings-search-shortcut").into(),
                row: Some(s.title().into()),
            },
            format!("{} keyboard key shortcut", s.english_title()),
        )
    });
    let tabs = Section::ALL.into_iter().map(|section| {
        (
            Found {
                section,
                title: section.label().into(),
                detail: if is_coming(section) {
                    tr!("app-coming-soon")
                } else {
                    tr!("settings-search-tab")
                }
                .into(),
                row: None,
            },
            tab_words(section).to_owned(),
        )
    });
    let query = words.join(" ");
    let mut found: Vec<(u8, Found)> = rows
        .chain(shortcuts)
        .chain(tabs)
        .filter_map(|(found, extra)| {
            let title = found.title.to_lowercase();
            let extra = extra.to_lowercase();
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
        detail: Option<&str>,
        content: impl IntoElement,
        th: &Theme,
    ) -> Div {
        let label = label.into();
        let detail = detail.map(|d| SharedString::from(d.to_owned()));
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
                        tr!("settings-search-none", query = query)
                    } else {
                        tr!("settings-search-results", query = query)
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
        .child(tr!("app-coming-soon"))
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

    /// Every row's name and line, and every tab, has an English message.
    #[test]
    fn every_entry_has_english() {
        for e in ENTRIES {
            for id in [e.title, e.detail] {
                assert_ne!(
                    katna_i18n::lookup(id, None),
                    id,
                    "no English message for {id}"
                );
            }
        }
        for section in Section::ALL {
            let title = section.label();
            assert_ne!(title, "", "{section:?}");
            assert!(
                !title.starts_with("settings-tab-"),
                "no English for {section:?}"
            );
            assert!(coming(section).is_none_or(|c| !c.starts_with("settings-")));
        }
    }
}
