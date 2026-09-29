// SPDX-License-Identifier: GPL-3.0-or-later

//! Import and Export on the Contacts page (`docs/ARCHITECTURE.md` §8.6):
//! vCard files, as every address book reads and writes them. Import saves
//! the file's people in the account in view (with an Undo), leaving out
//! anyone already saved; Export writes the people on screen.

use std::collections::BTreeSet;
use std::path::PathBuf;

use gpui::{AnyElement, Context, FontWeight, PathPromptOptions, div, prelude::*, rgba};
use katna_core::contact::Card;
use katna_dav::vcard::{self, Parsed};
use katna_i18n::tr;
use katna_store::StoredCard;
use katna_ui::{Ripple, px};

use super::MailWindow;
use super::attachments::{download_dir, unique_path};
use super::contacts_edit::visible;
use super::contacts_page::View;
use crate::daemon::{self, Command};
use crate::data::SavedBook;
use crate::theme::Theme;
use crate::widgets::icon;

/// The cards of a file worth importing: not groups, not empty, and not
/// someone already saved (`saved`, by lower-case address) or met earlier in
/// the file. Returns them with their labels, and how many were saved
/// already.
pub(super) fn to_import(
    parsed: Vec<Parsed>,
    saved: &impl Fn(&str) -> bool,
) -> (Vec<(Card, Vec<String>)>, usize) {
    let mut seen = BTreeSet::new();
    let mut skipped = 0;
    let mut out = Vec::new();
    for p in parsed {
        if p.group || p.card.is_empty() {
            continue;
        }
        let keys = p.card.email_keys();
        if keys.iter().any(|k| saved(k) || seen.contains(k)) {
            skipped += 1;
            continue;
        }
        seen.extend(keys);
        out.push((p.card, p.categories));
    }
    (out, skipped)
}

/// `cards` as one vCard file.
pub(super) fn to_vcf(cards: &[StoredCard]) -> String {
    let mut out = String::new();
    for c in cards {
        out.push_str(&vcard::write(
            &format!("katna-{}", c.id),
            &c.card,
            &c.labels,
            None,
        ));
        if !out.ends_with("\r\n") {
            out.push_str("\r\n");
        }
    }
    out
}

impl MailWindow {
    /// "Fix and manage" at the foot of the column: Merge and fix, Import,
    /// Export and Print.
    pub(super) fn contacts_manage_nav(
        &self,
        book: Option<&SavedBook>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let action = |ix: usize, glyph: &str, label: String, on: bool, count: Option<usize>| {
            div()
                .id(("contacts-manage", ix))
                .relative()
                .overflow_hidden()
                .h(px(36.0))
                .mr(px(12.0))
                .pl(px(20.0))
                .pr(px(16.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(14.0))
                .rounded_r_full()
                .cursor_pointer()
                .when(on, |d| d.bg(rgba(th.nav_selected)))
                .when(!on, |d| d.hover(|s| s.bg(rgba(th.hover))))
                .child(Ripple::new(("contacts-manage", ix), rgba(th.ripple)))
                .child(icon(
                    glyph,
                    if on {
                        th.nav_selected_text
                    } else {
                        th.text_dim
                    },
                    20.0,
                ))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .text_size(px(14.0))
                        .when(on, |d| d.font_weight(FontWeight::SEMIBOLD))
                        .text_color(rgba(if on { th.nav_selected_text } else { th.text }))
                        .child(label),
                )
                .children(count.map(|n| {
                    div()
                        .flex_none()
                        .text_size(px(12.0))
                        .text_color(rgba(if on {
                            th.nav_selected_text
                        } else {
                            th.text_faint
                        }))
                        .child(katna_i18n::format::number(n as u64))
                }))
        };
        let suggestions = book
            .map(|b| self.merge_suggestions(b).len())
            .filter(|&n| n > 0);
        div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(
                div()
                    .pt(px(18.0))
                    .pb(px(6.0))
                    .pl(px(20.0))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgba(th.text))
                    .child(tr!("contacts-manage")),
            )
            .child(
                action(
                    0,
                    "sparkle",
                    tr!("contacts-merge"),
                    self.contacts.view == View::Merge,
                    suggestions,
                )
                .on_click(cx.listener(|this, _, _, cx| this.set_contacts_view(View::Merge, cx))),
            )
            .child(
                action(1, "upload", tr!("contacts-import"), false, None)
                    .on_click(cx.listener(|this, _, _, cx| this.import_contacts_file(cx))),
            )
            .child(
                action(2, "download", tr!("contacts-export"), false, None)
                    .on_click(cx.listener(|this, _, _, cx| this.export_contacts_file(cx))),
            )
            .child(
                action(3, "print", tr!("contacts-print"), false, None).on_click(cx.listener(
                    |this, _, window, cx| {
                        let (label, people) = this.people_on_show();
                        let title = label.unwrap_or_else(|| tr!("contacts-print-title"));
                        this.print_contacts(title, people, window, cx);
                    },
                )),
            )
            .into_any_element()
    }

    /// Asks for vCard or CSV files and saves their people in the account
    /// in view.
    fn import_contacts_file(&mut self, cx: &mut Context<Self>) {
        let prompt = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: Some(tr!("contacts-import-file").into()),
        });
        let book = self.book_for_new_contact();
        let place = self
            .writable_books()
            .into_iter()
            .find(|(id, _)| *id == book)
            .map(|(_, name)| name)
            .unwrap_or_default();
        let saved: BTreeSet<String> = self.contacts.by_email.keys().cloned().collect();
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(files))) = prompt.await else {
                return;
            };
            let name = files
                .first()
                .and_then(|f| f.file_name())
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let read = cx
                .background_executor()
                .spawn(async move {
                    let mut parsed = Vec::new();
                    for file in &files {
                        let bytes = std::fs::read(file).map_err(|e| e.to_string())?;
                        let text = String::from_utf8_lossy(&bytes);
                        // A CSV file from Google, Outlook or Thunderbird.
                        if text.to_ascii_uppercase().contains("BEGIN:VCARD") {
                            parsed.extend(vcard::parse(&text));
                        } else {
                            parsed.extend(super::contacts_csv::parse(&text));
                        }
                    }
                    Ok::<_, String>(to_import(parsed, &|e| saved.contains(e)))
                })
                .await;
            let (cards, skipped) = match read {
                Ok(read) => read,
                Err(error) => {
                    this.update(cx, |this, cx| {
                        let text = tr!("contacts-import-failed", name = name, error = error);
                        this.show_snackbar(text, None, cx);
                    })
                    .ok();
                    return;
                }
            };
            if cards.is_empty() {
                this.update(cx, |this, cx| {
                    let text = if skipped > 0 {
                        tr!("contacts-import-all-saved", name = name)
                    } else {
                        tr!("contacts-import-none", name = name)
                    };
                    this.show_snackbar(text, None, cx);
                })
                .ok();
                return;
            }
            let result = async {
                let connection = match connection {
                    Some(connection) => connection,
                    None => daemon::connect().await?,
                };
                daemon::import_contacts(&connection, book, &cards).await
            }
            .await;
            this.update(cx, |this, cx| match result {
                Ok(ids) => {
                    let count = ids.len();
                    let text = if skipped > 0 {
                        tr!(
                            "contacts-imported-some",
                            count = count,
                            place = place,
                            skipped = skipped
                        )
                    } else {
                        tr!("contacts-imported", count = count, place = place)
                    };
                    this.load_contacts(cx);
                    this.show_snackbar(text, Some(Command::DeleteContacts(ids)), cx);
                }
                Err(err) => this.show_snackbar(crate::format::sentence(&err), None, cx),
            })
            .ok();
        })
        .detach();
    }

    /// Writes the people on screen (everyone, or the label in view) to a
    /// vCard file the user picks.
    /// The label on show, if one is, and the cards of each person listed
    /// under it (everyone saved without one).
    pub(super) fn people_on_show(&self) -> (Option<String>, Vec<Vec<i64>>) {
        let label = match &self.contacts.view {
            View::Label(name) => Some(name.clone()),
            _ => None,
        };
        let Some(Ok(book)) = &self.contacts.book else {
            return (label, Vec::new());
        };
        let people = visible(&book.people, &self.contacts.hidden)
            .into_iter()
            .map(|ix| &book.people[ix])
            .filter(|p| {
                label.as_ref().is_none_or(|l| {
                    self.person_labels(p.ids.first().copied(), &p.labels)
                        .contains(l)
                })
            })
            .filter(|p| !p.ids.is_empty())
            .map(|p| p.ids.clone())
            .collect();
        (label, people)
    }

    fn export_contacts_file(&mut self, cx: &mut Context<Self>) {
        let (label, people) = self.people_on_show();
        let ids: Vec<i64> = people
            .iter()
            .filter_map(|ids| ids.first().copied())
            .collect();
        if ids.is_empty() {
            self.show_snackbar(tr!("contacts-export-none"), None, cx);
            return;
        }
        let dir = download_dir();
        let name = format!(
            "{}.vcf",
            label
                .as_deref()
                .unwrap_or("contacts")
                .replace(['/', '\\'], "-")
        );
        let prompt = cx.prompt_for_new_path(&dir, Some(&name));
        let paths = self.paths.clone();
        cx.spawn(async move |this, cx| {
            let target: PathBuf = match prompt.await {
                Ok(Ok(Some(path))) => path,
                Ok(Ok(None)) => return,
                // No file chooser (no desktop portal): save to Downloads.
                _ => unique_path(&dir, &name),
            };
            let written = cx
                .background_executor()
                .spawn(async move {
                    let cards = crate::data::saved_cards(&paths, &ids)?;
                    std::fs::write(&target, to_vcf(&cards)).map_err(|e| e.to_string())?;
                    Ok::<_, String>((cards.len(), target))
                })
                .await;
            this.update(cx, |this, cx| {
                let text = match written {
                    Ok((count, path)) => {
                        if this.config.mail.open_saved_folder {
                            cx.reveal_path(&path);
                        }
                        tr!(
                            "contacts-exported",
                            count = count,
                            path = path.display().to_string()
                        )
                    }
                    Err(error) => tr!("contacts-export-failed", error = error),
                };
                this.show_snackbar(text, None, cx);
            })
            .ok();
        })
        .detach();
    }
}

#[cfg(test)]
mod tests {
    use katna_core::contact::{Name, Typed};

    use super::*;

    fn parsed(given: &str, email: &str) -> Parsed {
        Parsed {
            card: Card {
                name: Name {
                    given: given.into(),
                    ..Name::default()
                },
                emails: vec![Typed::new(email, "")],
                ..Card::default()
            },
            ..Parsed::default()
        }
    }

    #[test]
    fn imports_new_people_once() {
        let mut group = parsed("Team", "");
        group.group = true;
        let file = vec![
            parsed("Asha", "asha@x.in"),
            parsed("Bo", "Bo@Y.io"),
            parsed("Asha again", "ASHA@x.in"),
            group,
            Parsed::default(),
        ];
        let (cards, skipped) = to_import(file, &|e| e == "bo@y.io");
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].0.display_name(), "Asha");
        assert_eq!(skipped, 2);
    }

    #[test]
    fn exports_what_import_reads() {
        let card = |id: i64, given: &str, email: &str, labels: &[&str]| StoredCard {
            id,
            book: 1,
            account: None,
            source: katna_store::BookSource::Local,
            card: parsed(given, email).card,
            starred: false,
            labels: labels.iter().map(|l| l.to_string()).collect(),
        };
        let vcf = to_vcf(&[
            card(1, "Asha", "asha@x.in", &["Family"]),
            card(2, "Bo", "bo@y.io", &[]),
        ]);
        let back = vcard::parse(&vcf);
        assert_eq!(back.len(), 2);
        assert_eq!(back[0].card.display_name(), "Asha");
        assert_eq!(back[0].categories, ["Family"]);
        assert_eq!(back[1].card.emails[0].value, "bo@y.io");
    }
}
