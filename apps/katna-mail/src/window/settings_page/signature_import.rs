// SPDX-License-Identifier: GPL-3.0-or-later

//! Settings > Compose > Signatures > Import: the signatures Gmail adds
//! for the Gmail accounts signed in with Google, and those Thunderbird,
//! Evolution and KMail keep on this computer
//! ([`crate::signatures::import`]). Each is cleaned as pasted HTML is,
//! its pictures on the web downloaded through the daemon and carried
//! inside the mail; those already in Katna are shown but not ticked.

use std::collections::HashMap;
use std::rc::Rc;

use gpui::{AnyElement, Context, Task, Window, div, prelude::*, rgba};
use katna_i18n::tr;
use katna_render::html::{self as mail_html, Document};
use katna_ui::px;
use katna_ui::rich::{Block, Doc, html};
use katna_ui::tokens::{space, text};

use super::{MailWindow, control_column};
use crate::daemon;
use crate::signatures::import::{self, Found, Source};
use crate::theme::Theme;
use crate::widgets::{Check, checkbox, filled_button, icon, outlined_button};

/// Most pictures on the web one signature brings inside.
const MAX_PICTURES: usize = 10;
/// Largest picture brought inside, as for pasted HTML.
const MAX_PICTURE: usize = 512 * 1024;

/// The Import panel.
pub(super) struct ImportSignatures {
    /// Still looking.
    looking: bool,
    items: Vec<Item>,
    /// What went wrong with an account, one line each.
    notes: Vec<String>,
    _task: Option<Task<()>>,
}

struct Item {
    source: Source,
    name: String,
    /// The signature as it would be saved.
    doc: Doc,
    /// Its HTML laid out, for the preview; `None` for plain text.
    shown: Option<Rc<Document>>,
    text: String,
    already: bool,
    picked: bool,
}

/// What the search found, before cleaning.
struct Search {
    found: Vec<Found>,
    notes: Vec<String>,
    fetched: HashMap<String, Vec<u8>>,
}

impl MailWindow {
    /// Opens the panel and looks for signatures.
    pub(super) fn open_import_signatures(&mut self, cx: &mut Context<Self>) {
        let gmail: Vec<(i64, String)> = match self.mail.as_ref() {
            Ok(mail) => self
                .accounts
                .iter()
                .filter(|a| mail.sign_in_provider(a.id) == Some(katna_core::OAuthProvider::Google))
                .map(|a| (a.id.0, a.address.clone()))
                .collect(),
            Err(_) => Vec::new(),
        };
        let connection = self.daemon.clone();
        let task = cx.spawn(async move |this, cx| {
            let search = cx
                .background_executor()
                .spawn(search(connection, gmail))
                .await;
            this.update(cx, |this, cx| this.import_found(search, cx))
                .ok();
        });
        if let Some(page) = &mut self.settings_page {
            page.pasting = None;
            page.editing = None;
            page.importing = Some(ImportSignatures {
                looking: true,
                items: Vec::new(),
                notes: Vec::new(),
                _task: Some(task),
            });
        }
        cx.notify();
    }

    fn importing(&mut self) -> Option<&mut ImportSignatures> {
        self.settings_page.as_mut()?.importing.as_mut()
    }

    fn close_import_signatures(&mut self, cx: &mut Context<Self>) {
        if let Some(page) = &mut self.settings_page {
            page.importing = None;
        }
        cx.notify();
    }

    /// Cleans what was found and shows it.
    fn import_found(&mut self, search: Search, cx: &mut Context<Self>) {
        let existing: Vec<(String, String)> = self
            .config
            .sending
            .signatures
            .iter()
            .map(|s| (s.text.trim().to_owned(), s.html.clone()))
            .collect();
        let items: Vec<Item> = search
            .found
            .into_iter()
            .filter_map(|found| {
                let (doc, shown) = if found.html.is_empty() {
                    (html::from_plain(&found.text), None)
                } else {
                    let cleaned = mail_html::clean(&found.html, &search.fetched);
                    if cleaned.html.is_empty() {
                        return None;
                    }
                    let mut next = 0;
                    let block = html::html_block(&cleaned.html, &mut next);
                    let shown = Rc::new(crate::window::rich::designed_document(&block));
                    (
                        Doc {
                            blocks: vec![Block::Html(block)],
                        },
                        Some(shown),
                    )
                };
                let (text, saved) = crate::window::compose::signature_content(&doc);
                let already = existing.iter().any(|(t, h)| {
                    if saved.is_empty() {
                        h.is_empty() && *t == text.trim()
                    } else {
                        *h == saved
                    }
                });
                Some(Item {
                    source: found.source,
                    name: found.name,
                    doc,
                    shown,
                    text,
                    already,
                    picked: !already,
                })
            })
            .collect();
        if let Some(i) = self.importing() {
            i.looking = false;
            i.items = items;
            i.notes = search.notes;
        }
        cx.notify();
    }

    fn toggle_import(&mut self, n: usize, cx: &mut Context<Self>) {
        if let Some(item) = self.importing().and_then(|i| i.items.get_mut(n)) {
            item.picked = !item.picked;
        }
        cx.notify();
    }

    /// Saves the ticked signatures and opens the first.
    fn import_signatures(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(importing) = self.settings_page.as_mut().and_then(|p| p.importing.take()) else {
            return;
        };
        let mut first = None;
        for item in importing.items.into_iter().filter(|i| i.picked) {
            let (text, html) = crate::window::compose::signature_content(&item.doc);
            let name = tr!(
                "signature-import-name",
                name = item.name,
                app = item.source.name()
            );
            let id = self.config.sending.add_signature(name, String::new());
            if let Some(signature) = self
                .config
                .sending
                .signatures
                .iter_mut()
                .find(|s| s.id == id)
            {
                signature.text = text;
                signature.html = html;
            }
            first.get_or_insert(id);
        }
        if first.is_some() {
            self.save_config();
        }
        self.edit_signature(first, window, cx);
        cx.notify();
    }

    /// The panel, in place of the signature editor while it is open.
    pub(super) fn import_signatures_panel(
        &self,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let i = self.settings_page.as_ref()?.importing.as_ref()?;
        let faint = rgba(th.text_faint);
        let small = |said: String| {
            div()
                .text_size(px(text::SMALL))
                .text_color(faint)
                .child(said)
        };
        let title = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_baseline()
            .gap(px(space::S3))
            .child(
                div()
                    .text_size(px(text::SUBTITLE))
                    .child(tr!("signature-import-title")),
            )
            .child(small(tr!("signature-import-subtitle")));
        let items = i.items.iter().enumerate().map(|(n, item)| {
            let preview = match &item.shown {
                Some(doc) => div()
                    .id(("page-signature-import-preview", n))
                    .max_h(px(160.0))
                    .overflow_y_scroll()
                    .child(crate::window::rich::designed(th, doc))
                    .into_any_element(),
                None => small(item.text.clone())
                    .whitespace_normal()
                    .into_any_element(),
            };
            let mut about = tr!("signature-import-from", app = item.source.name());
            if item.already {
                about = format!("{about} · {}", tr!("signature-import-already"));
            }
            self.page_control(
                crate::widgets::row(("page-signature-import", n), false, th),
                th,
                cx,
            )
            .items_start()
            .on_click(cx.listener(move |this, _, _, cx| this.toggle_import(n, cx)))
            .child(div().pt(px(space::S1)).child(checkbox(
                ("page-signature-import-check", n),
                if item.picked { Check::On } else { Check::Off },
                th,
            )))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(space::S2))
                    .child(div().truncate().child(item.name.clone()))
                    .child(small(about))
                    .child(preview),
            )
        });
        let line = |name: &str, said: String| {
            div()
                .flex()
                .flex_row()
                .items_start()
                .gap(px(space::S3))
                .text_size(px(text::SMALL))
                .text_color(faint)
                .child(
                    div()
                        .pt(px(space::S1))
                        .child(icon(name, th.text_faint, 14.0)),
                )
                .child(div().flex_1().min_w_0().child(said))
        };
        let mut notes: Vec<_> = i.notes.iter().map(|n| line("info", n.clone())).collect();
        if i.looking {
            notes.insert(0, line("search", tr!("signature-import-looking")));
        } else if i.items.is_empty() {
            notes.insert(0, line("info", tr!("signature-import-none")));
        }
        let count = i.items.iter().filter(|i| i.picked).count();
        let buttons = div()
            .flex()
            .flex_row()
            .justify_end()
            .gap(px(space::S3))
            .child(
                outlined_button(
                    "page-signature-import-cancel",
                    tr!("signature-import-cancel"),
                    th,
                )
                .map(|d| self.page_control(d, th, cx))
                .on_click(cx.listener(|this, _, _, cx| this.close_import_signatures(cx))),
            )
            .when(!i.items.is_empty(), |d| {
                d.child(
                    filled_button(
                        "page-signature-import-do",
                        tr!("signature-import-do", count = count),
                        th,
                    )
                    .map(|d| self.page_control(d, th, cx))
                    .when(count == 0, |d| d.opacity(katna_ui::tokens::state::DISABLED))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        if count > 0 {
                            this.import_signatures(window, cx);
                        }
                    })),
                )
            });
        Some(
            control_column(240.0)
                .flex()
                .flex_col()
                .gap(px(space::S3))
                .child(title)
                .child(div().flex().flex_col().gap(px(space::S1)).children(items))
                .child(div().flex().flex_col().gap(px(space::S2)).children(notes))
                .child(buttons)
                .into_any_element(),
        )
    }
}

/// Looks on this computer and asks the daemon for Gmail's, then
/// downloads the pictures on the web they show.
async fn search(
    connection: Option<katna_dbus::zbus::Connection>,
    gmail: Vec<(i64, String)>,
) -> Search {
    let mut found = here();
    let mut notes = Vec::new();
    let connection = match connection {
        Some(connection) => Some(connection),
        None if gmail.is_empty() => None,
        None => daemon::connect().await.ok(),
    };
    let mut from_gmail = Vec::new();
    if let Some(connection) = &connection {
        for (account, address) in &gmail {
            match daemon::gmail_signatures(connection, *account).await {
                Ok(all) => from_gmail.extend(all.into_iter().map(|(sends_as, _, html)| Found {
                    source: Source::Gmail,
                    name: sends_as,
                    html,
                    text: String::new(),
                })),
                Err(None) => notes.push(tr!(
                    "signature-import-gmail-sign-in",
                    address = address.clone()
                )),
                Err(Some(error)) => notes.push(tr!(
                    "signature-import-gmail-failed",
                    address = address.clone(),
                    error = error
                )),
            }
        }
    }
    from_gmail.append(&mut found);
    let found = from_gmail;
    let mut fetched = HashMap::new();
    if let Some(connection) = &connection {
        for f in &found {
            for url in mail_html::web_pictures(&f.html)
                .into_iter()
                .take(MAX_PICTURES)
            {
                if fetched.contains_key(&url) {
                    continue;
                }
                if let Ok(bytes) = daemon::fetch_image(connection, &url).await
                    && bytes.len() <= MAX_PICTURE
                {
                    fetched.insert(url, bytes);
                }
            }
        }
    }
    Search {
        found,
        notes,
        fetched,
    }
}

/// The signatures of the mail apps of this user, under the usual XDG
/// directories.
fn here() -> Vec<Found> {
    let Some(home) = std::env::var_os("HOME").map(std::path::PathBuf::from) else {
        return Vec::new();
    };
    let xdg = |name: &str, fallback: &str| {
        std::env::var_os(name)
            .map(std::path::PathBuf::from)
            .filter(|p| p.is_absolute())
            .unwrap_or_else(|| home.join(fallback))
    };
    import::find(
        &home,
        &xdg("XDG_CONFIG_HOME", ".config"),
        &xdg("XDG_DATA_HOME", ".local/share"),
    )
}
