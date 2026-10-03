// SPDX-License-Identifier: GPL-3.0-or-later

//! Other contacts on the Contacts page (`docs/ARCHITECTURE.md` §8.6):
//! the people a Gmail account mailed but never saved, as Google keeps
//! them. Each can be saved in one click, with an Undo.

use std::collections::BTreeSet;
use std::ops::Range;
use std::rc::Rc;

use gpui::{AnyElement, Context, div, prelude::*, rgba, uniform_list};
use katna_i18n::tr;
use katna_store::OtherContact;
use katna_ui::px;

use super::MailWindow;
use super::apps::App;
use crate::daemon::{self, Command};
use crate::data::SavedBook;
use crate::theme::Theme;
use crate::widgets::{icon_button, tip};

/// The other contacts to show: not saved since, one per address, matching
/// `query` (lower case).
pub(super) fn shown_others<'a>(
    book: &'a SavedBook,
    saved: &impl Fn(&str) -> bool,
    query: &str,
) -> Vec<&'a OtherContact> {
    let mut seen = BTreeSet::new();
    book.others
        .iter()
        .filter(|o| !o.emails.iter().any(|e| saved(e)))
        .filter(|o| {
            query.is_empty()
                || o.name.to_lowercase().contains(query)
                || o.emails.iter().any(|e| e.contains(query))
        })
        .filter(|o| seen.insert(o.emails.first().cloned().unwrap_or_else(|| o.name.clone())))
        .collect()
}

impl MailWindow {
    /// Whether `email` (lower case) is someone saved.
    fn saved_email(&self, email: &str) -> bool {
        self.contacts.by_email.contains_key(email)
    }

    /// How many other contacts the side column counts.
    pub(super) fn other_count(&self, book: &SavedBook) -> usize {
        let saving = &self.contacts.saving_others;
        shown_others(book, &|e| self.saved_email(e), "")
            .into_iter()
            .filter(|o| !saving.contains(&o.id))
            .count()
    }

    /// The Other contacts list.
    pub(super) fn render_other_contacts(
        &self,
        book: &Rc<SavedBook>,
        query: &str,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let saving = &self.contacts.saving_others;
        let others: Vec<OtherContact> = shown_others(book, &|e| self.saved_email(e), query)
            .into_iter()
            .filter(|o| !saving.contains(&o.id))
            .cloned()
            .collect();
        if others.is_empty() {
            let text = if !query.is_empty() {
                tr!("contacts-none-found")
            } else if !book.others_blocked.is_empty() {
                tr!("contacts-other-allow")
            } else {
                tr!("contacts-other-empty")
            };
            return self.placeholder(text, th);
        }
        let header = div()
            .flex_none()
            .px(px(24.0))
            .py(px(14.0))
            .flex()
            .flex_col()
            .gap(px(2.0))
            .border_b_1()
            .border_color(rgba(th.divider))
            .child(
                div()
                    .text_size(px(20.0))
                    .text_color(rgba(th.text))
                    .child(tr!("contacts-other")),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(rgba(th.text_faint))
                    .child(tr!("contacts-other-about")),
            );
        let count = others.len();
        let others = Rc::new(others);
        let list = uniform_list(
            "other-contacts",
            count,
            cx.processor(move |this, range: Range<usize>, window, cx| {
                let th = this.theme(window);
                range
                    .map(|ix| this.render_other(ix, &others[ix], &th, cx))
                    .collect::<Vec<_>>()
            }),
        )
        .size_full();
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(header)
            .child(div().flex_1().min_h_0().child(list))
            .into_any_element()
    }

    fn render_other(
        &self,
        ix: usize,
        other: &OtherContact,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let email = other.emails.first().cloned().unwrap_or_default();
        let name = if other.name.trim().is_empty() {
            email.clone()
        } else {
            other.name.clone()
        };
        let (id, shown) = (other.id, name.clone());
        let to = email.clone();
        let from = email.clone();
        div()
            .id(("other-contact", ix))
            .group("other-contact")
            .w_full()
            .h(px(52.0))
            .px(px(24.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(16.0))
            .cursor_pointer()
            .hover(|s| s.bg(rgba(th.hover)))
            .on_click(cx.listener(move |this, _, window, cx| {
                this.open_app(App::Mail, cx);
                this.search_for(format!("from:{from} OR to:{from}"), window, cx);
            }))
            .child(self.person_avatar(&name, &email, 36.0))
            .child(
                div()
                    .w(px(220.0))
                    .flex_none()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .text_size(px(14.0))
                    .text_color(rgba(th.text))
                    .child(div().min_w_0().truncate().child(name))
                    .children(self.muted_mark_any(&other.emails, 16.0, th)),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .truncate()
                    .text_size(px(14.0))
                    .text_color(rgba(th.text_dim))
                    .child(email),
            )
            .child(
                div()
                    .w(px(160.0))
                    .flex_none()
                    .truncate()
                    .text_size(px(14.0))
                    .text_color(rgba(th.text_dim))
                    .child(other.phone.clone()),
            )
            .child(
                div()
                    .flex_none()
                    .flex()
                    .flex_row()
                    .invisible()
                    .group_hover("other-contact", |s| s.visible())
                    .child(
                        icon_button(("other-contact-mail", ix), "mail", 20.0, th)
                            .tooltip(tip(tr!("contacts-other-email"), th))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                cx.stop_propagation();
                                this.open_app(App::Mail, cx);
                                let mail = crate::mailto::Mailto {
                                    to: vec![to.clone()],
                                    ..Default::default()
                                };
                                this.open_mailto(mail, window, cx);
                            })),
                    )
                    .child(
                        icon_button(("other-contact-add", ix), "person-add", 20.0, th)
                            .tooltip(tip(tr!("contact-add-to-contacts"), th))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                cx.stop_propagation();
                                this.save_other_contact(id, shown.clone(), cx);
                            })),
                    ),
            )
            .into_any_element()
    }

    /// Saves other contact `id` in its account's contacts, with an Undo.
    fn save_other_contact(&mut self, id: i64, name: String, cx: &mut Context<Self>) {
        if !self.contacts.saving_others.insert(id) {
            return;
        }
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = async {
                let connection = match connection {
                    Some(connection) => connection,
                    None => daemon::connect().await?,
                };
                daemon::save_other_contact(&connection, id).await
            }
            .await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(card) => {
                        this.load_contacts(cx);
                        // A card that comes with the next sync has no Undo yet.
                        let undo = (card != 0).then(|| Command::DeleteContacts(vec![card]));
                        this.show_snackbar(tr!("contacts-added", name = name), undo, cx);
                    }
                    Err(err) => {
                        this.contacts.saving_others.remove(&id);
                        this.show_snackbar(crate::format::sentence(&err), None, cx);
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
        cx.notify();
    }
}

#[cfg(test)]
mod tests {
    use katna_core::AccountId;

    use super::*;

    fn other(id: i64, name: &str, email: &str) -> OtherContact {
        OtherContact {
            id,
            account: AccountId(1),
            remote_id: format!("otherContacts/{id}"),
            name: name.into(),
            sort_key: name.to_lowercase(),
            emails: vec![email.into()],
            phone: String::new(),
        }
    }

    #[test]
    fn hides_saved_people_and_repeats() {
        let book = SavedBook {
            others: vec![
                other(1, "Asha", "asha@x.in"),
                other(2, "Ravi", "ravi@shop.in"),
                other(3, "Ravi K", "ravi@shop.in"),
                other(4, "Bo", "bo@y.io"),
            ],
            ..SavedBook::default()
        };
        let saved = |e: &str| e == "bo@y.io";
        let ids = |q: &str| -> Vec<i64> {
            shown_others(&book, &saved, q)
                .into_iter()
                .map(|o| o.id)
                .collect()
        };
        assert_eq!(ids(""), [1, 2]);
        assert_eq!(ids("shop"), [2]);
        assert_eq!(ids("ash"), [1]);
    }
}
