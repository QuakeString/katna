// SPDX-License-Identifier: GPL-3.0-or-later

//! Merge and fix on the Contacts page (`docs/ARCHITECTURE.md` §8.6), as in
//! Google Contacts: people who look like the same person (the same name,
//! or the same phone number) are suggested for merging. Merging combines
//! their cards into one card per address book, with every address, number
//! and label, and has an Undo; a suggestion can be dismissed for good.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::Rc;

use gpui::{AnyElement, Context, FontWeight, div, prelude::*, rgba};
use katna_core::contact::{Card, Typed};
use katna_i18n::tr;
use katna_store::{SavedContact, StoredCard};
use katna_ui::px;

use super::MailWindow;
use super::contacts_edit::visible;
use crate::daemon::{self, Command, WriteCard};
use crate::data::SavedBook;
use crate::theme::Theme;
use crate::widgets::{filled_button, outlined_button, placeholder};

/// The name two people must share to look like one: in lower case with
/// single spaces, and not just an address or a word of two letters.
fn name_key(name: &str) -> Option<String> {
    let key = name
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    (key.chars().filter(|c| c.is_alphabetic()).count() >= 3 && !key.contains('@')).then_some(key)
}

/// A phone number's last ten digits, so "+91 98450 11223" and
/// "098450 11223" match; short numbers (extensions) never do.
fn phone_key(phone: &str) -> Option<String> {
    let digits: String = phone.chars().filter(char::is_ascii_digit).collect();
    (digits.len() >= 8).then(|| digits[digits.len().saturating_sub(10)..].to_owned())
}

/// The key a suggestion is dismissed by: its people's first cards.
pub(super) fn group_key(people: &[SavedContact], group: &[usize]) -> String {
    let mut ids: Vec<i64> = group
        .iter()
        .filter_map(|&ix| people[ix].ids.first().copied())
        .collect();
    ids.sort_unstable();
    ids.iter().map(i64::to_string).collect::<Vec<_>>().join(",")
}

/// People among `shown` who look like the same person, in groups of two or
/// more, in list order.
pub(super) fn duplicate_groups(people: &[SavedContact], shown: &[usize]) -> Vec<Vec<usize>> {
    let mut parent: HashMap<usize, usize> = shown.iter().map(|&ix| (ix, ix)).collect();
    fn root(parent: &mut HashMap<usize, usize>, mut ix: usize) -> usize {
        while parent[&ix] != ix {
            ix = parent[&ix];
        }
        ix
    }
    let mut owner: HashMap<String, usize> = HashMap::new();
    for &ix in shown {
        let person = &people[ix];
        let keys = name_key(&person.name)
            .map(|k| format!("n:{k}"))
            .into_iter()
            .chain(phone_key(&person.phone).map(|k| format!("p:{k}")));
        for key in keys {
            match owner.get(&key) {
                Some(&other) => {
                    let (a, b) = (root(&mut parent, ix), root(&mut parent, other));
                    if a != b {
                        parent.insert(a.max(b), a.min(b));
                    }
                }
                None => {
                    owner.insert(key, ix);
                }
            }
        }
    }
    let mut groups: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &ix in shown {
        let r = root(&mut parent, ix);
        groups.entry(r).or_default().push(ix);
    }
    let mut out: Vec<Vec<usize>> = groups.into_values().filter(|g| g.len() > 1).collect();
    out.sort_by_key(|g| {
        g.iter()
            .filter_map(|&ix| shown.iter().position(|&s| s == ix))
            .min()
    });
    out
}

/// Adds the values of `more` not already in `list` (by value, ignoring
/// case and spaces).
fn add_typed(list: &mut Vec<Typed>, more: &[Typed]) {
    let key = |t: &Typed| {
        t.value
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .to_lowercase()
    };
    for t in more {
        if !t.value.trim().is_empty() && !list.iter().any(|l| key(l) == key(t)) {
            list.push(t.clone());
        }
    }
}

/// Fills `into` when empty; two different texts are both kept for a note.
fn fill(into: &mut String, from: &str) {
    if into.trim().is_empty() {
        *into = from.to_owned();
    }
}

/// One card holding everything in `cards`: the first card's name and
/// details, then what the others add.
pub(super) fn combine(cards: &[&Card]) -> Card {
    let Some(first) = cards.first() else {
        return Card::default();
    };
    let mut out = (*first).clone();
    for card in &cards[1..] {
        if out.name.shown().is_empty() {
            out.name = card.name.clone();
        }
        fill(&mut out.nickname, &card.nickname);
        add_typed(&mut out.emails, &card.emails);
        add_typed(&mut out.phones, &card.phones);
        add_typed(&mut out.urls, &card.urls);
        for address in &card.addresses {
            if !address.is_empty() && !out.addresses.contains(address) {
                out.addresses.push(address.clone());
            }
        }
        fill(&mut out.organization, &card.organization);
        fill(&mut out.department, &card.department);
        fill(&mut out.title, &card.title);
        fill(&mut out.birthday, &card.birthday);
        fill(&mut out.photo_url, &card.photo_url);
        let note = card.note.trim();
        if !note.is_empty() && !out.note.contains(note) {
            if out.note.trim().is_empty() {
                out.note = note.to_owned();
            } else {
                out.note = format!("{}\n\n{note}", out.note.trim_end());
            }
        }
    }
    out
}

/// What merging `cards` writes, and what puts them back: one card per
/// address book keeps the combined card and every label; the book's other
/// cards go.
pub(super) fn merge_plan(cards: &[StoredCard]) -> (Vec<WriteCard>, Vec<i64>, Vec<WriteCard>) {
    let merged = combine(&cards.iter().map(|c| &c.card).collect::<Vec<_>>());
    let mut labels: Vec<String> = Vec::new();
    for c in cards {
        for l in &c.labels {
            if !labels.contains(l) {
                labels.push(l.clone());
            }
        }
    }
    let mut kept: BTreeSet<i64> = BTreeSet::new();
    let mut write = Vec::new();
    let mut delete = Vec::new();
    let mut undo = Vec::new();
    for c in cards {
        if !kept.insert(c.book) {
            delete.push(c.id);
            undo.push(WriteCard {
                id: 0,
                book: c.book,
                card: c.card.clone(),
                labels: c.labels.clone(),
            });
        } else {
            write.push(WriteCard {
                id: c.id,
                book: c.book,
                card: merged.clone(),
                labels: labels.clone(),
            });
            undo.push(WriteCard {
                id: c.id,
                book: c.book,
                card: c.card.clone(),
                labels: c.labels.clone(),
            });
        }
    }
    (write, delete, undo)
}

impl MailWindow {
    /// The suggestions not dismissed.
    pub(super) fn merge_suggestions(&self, book: &SavedBook) -> Vec<Vec<usize>> {
        let shown = visible(&book.people, &self.contacts.hidden);
        let dismissed = &self.config.contacts.dismissed_duplicates;
        duplicate_groups(&book.people, &shown)
            .into_iter()
            .filter(|g| !dismissed.contains(&group_key(&book.people, g)))
            .collect()
    }

    /// The Merge and fix page.
    pub(super) fn render_merge_page(
        &self,
        book: &Rc<SavedBook>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let groups = self.merge_suggestions(book);
        if groups.is_empty() {
            return placeholder(&tr!("contacts-merge-none"), th);
        }
        let all = groups.clone();
        let header = div()
            .flex_none()
            .px(px(24.0))
            .py(px(14.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(16.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_size(px(20.0))
                            .text_color(rgba(th.text))
                            .child(tr!("contacts-merge")),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgba(th.text_faint))
                            .child(tr!("contacts-merge-about", count = groups.len())),
                    ),
            )
            .when(groups.len() > 1, |d| {
                d.child(
                    filled_button("contacts-merge-all", tr!("contacts-merge-all"), th).on_click(
                        cx.listener(move |this, _, _, cx| this.merge_people(all.clone(), cx)),
                    ),
                )
            });
        let cards = groups
            .into_iter()
            .enumerate()
            .map(|(ix, group)| self.render_merge_suggestion(ix, &book.people, group, th, cx));
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(header)
            .child(
                div()
                    .id("contacts-merge-list")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .px(px(24.0))
                    .pb(px(24.0))
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .children(cards),
            )
            .into_any_element()
    }

    fn render_merge_suggestion(
        &self,
        ix: usize,
        people: &[SavedContact],
        group: Vec<usize>,
        th: &Theme,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let key = group_key(people, &group);
        let rows = group.iter().map(|&p| {
            let person = &people[p];
            let email = person.emails.first().cloned().unwrap_or_default();
            div()
                .h(px(52.0))
                .px(px(16.0))
                .flex()
                .flex_row()
                .items_center()
                .gap(px(16.0))
                .child(self.person_avatar(&person.name, &email, 36.0))
                .child(
                    div()
                        .w(px(200.0))
                        .flex_none()
                        .truncate()
                        .text_size(px(14.0))
                        .text_color(rgba(th.text))
                        .child(person.name.clone()),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .text_size(px(14.0))
                        .text_color(rgba(th.text_dim))
                        .child(person.emails.join(", ")),
                )
                .child(
                    div()
                        .w(px(160.0))
                        .flex_none()
                        .truncate()
                        .text_size(px(14.0))
                        .text_color(rgba(th.text_dim))
                        .child(person.phone.clone()),
                )
        });
        div()
            .flex_none()
            .rounded(px(12.0))
            .border_1()
            .border_color(rgba(th.outline))
            .py(px(8.0))
            .flex()
            .flex_col()
            .child(
                div()
                    .px(px(16.0))
                    .pb(px(4.0))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgba(th.text_faint))
                    .child(tr!("contacts-merge-count", count = group.len())),
            )
            .children(rows)
            .child(
                div()
                    .px(px(16.0))
                    .pt(px(8.0))
                    .flex()
                    .flex_row()
                    .justify_end()
                    .gap(px(8.0))
                    .child(
                        outlined_button(
                            ("contacts-merge-dismiss", ix),
                            tr!("contacts-merge-dismiss"),
                            th,
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.dismiss_suggestion(key.clone(), cx)
                        })),
                    )
                    .child(
                        filled_button(("contacts-merge-one", ix), tr!("contacts-merge-button"), th)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.merge_people(vec![group.clone()], cx)
                            })),
                    ),
            )
            .into_any_element()
    }

    /// Stops suggesting the group `key`.
    fn dismiss_suggestion(&mut self, key: String, cx: &mut Context<Self>) {
        let dismissed = &mut self.config.contacts.dismissed_duplicates;
        if !dismissed.contains(&key) {
            dismissed.push(key);
            self.save_config();
        }
        cx.notify();
    }

    /// Merges each group of people (by their place in the saved book), with
    /// one Undo for all.
    fn merge_people(&mut self, groups: Vec<Vec<usize>>, cx: &mut Context<Self>) {
        let Some(Ok(book)) = &self.contacts.book else {
            return;
        };
        let sets: Vec<Vec<i64>> = groups
            .iter()
            .map(|g| {
                g.iter()
                    .flat_map(|&ix| book.people[ix].ids.iter().copied())
                    .collect()
            })
            .collect();
        let merged: usize = groups.len();
        // Hidden at once; the list shows them as one once read back.
        for g in &groups {
            for &ix in g.iter().skip(1) {
                if let Some(&first) = book.people[ix].ids.first() {
                    self.contacts.hidden.insert(first);
                }
            }
        }
        let paths = self.paths.clone();
        let connection = self.daemon.clone();
        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let mut steps = Vec::new();
                    let mut undo = Vec::new();
                    for ids in &sets {
                        let cards = crate::data::saved_cards(&paths, ids)?;
                        let (write, delete, back) = merge_plan(&cards);
                        steps.push(Command::WriteCards(write));
                        if !delete.is_empty() {
                            steps.push(Command::DeleteContacts(delete));
                        }
                        undo.extend(back);
                    }
                    let connection = match connection {
                        Some(connection) => connection,
                        None => daemon::connect().await?,
                    };
                    daemon::send(&connection, &Command::Several(steps)).await?;
                    Ok::<_, String>(Command::WriteCards(undo))
                })
                .await;
            this.update(cx, |this, cx| {
                this.contacts.hidden.clear();
                this.load_contacts(cx);
                match result {
                    Ok(undo) => {
                        this.show_snackbar(tr!("contacts-merged", count = merged), Some(undo), cx)
                    }
                    Err(err) => this.show_snackbar(crate::format::sentence(&err), None, cx),
                }
            })
            .ok();
        })
        .detach();
        cx.notify();
    }
}

#[cfg(test)]
mod tests {
    use katna_core::contact::Name;
    use katna_store::BookSource;

    use super::*;

    fn person(id: i64, name: &str, email: &str, phone: &str) -> SavedContact {
        SavedContact {
            ids: vec![id],
            name: name.into(),
            emails: vec![email.into()],
            phone: phone.into(),
            ..SavedContact::default()
        }
    }

    #[test]
    fn suggests_same_names_and_numbers() {
        let people = vec![
            person(1, "Asha Rao", "asha@x.in", ""),
            person(2, "Bo", "bo@y.io", "+91 98450 11223"),
            person(3, "asha  rao", "asha.rao@work.in", ""),
            person(4, "Bo Chen", "bo@z.io", "098450 11223"),
            person(5, "Bo", "bo@w.io", ""),
            person(6, "Chen Wei", "chen@x.cn", "123"),
            person(7, "Dev", "dev@x.in", "4567"),
        ];
        let shown: Vec<usize> = (0..people.len()).collect();
        let groups = duplicate_groups(&people, &shown);
        assert_eq!(groups, [vec![0, 2], vec![1, 3]]);
        assert_eq!(group_key(&people, &groups[1]), "2,4");
    }

    #[test]
    fn combines_everything_once() {
        let a = Card {
            name: Name {
                given: "Asha".into(),
                family: "Rao".into(),
                ..Name::default()
            },
            emails: vec![Typed::new("asha@x.in", "home")],
            phones: vec![Typed::new("+91 98450 11223", "mobile")],
            note: "Met at the fair".into(),
            ..Card::default()
        };
        let b = Card {
            emails: vec![
                Typed::new("ASHA@x.in", ""),
                Typed::new("asha@work.in", "work"),
            ],
            phones: vec![Typed::new("+9198450 11223", "")],
            organization: "Invenia".into(),
            note: "Prefers mail".into(),
            ..Card::default()
        };
        let c = combine(&[&a, &b]);
        assert_eq!(c.name.shown(), "Asha Rao");
        assert_eq!(c.emails.len(), 2);
        assert_eq!(c.phones.len(), 1);
        assert_eq!(c.organization, "Invenia");
        assert_eq!(c.note, "Met at the fair\n\nPrefers mail");
    }

    #[test]
    fn keeps_one_card_per_book_and_undoes() {
        let card = |id: i64, book: i64, email: &str, labels: &[&str]| StoredCard {
            id,
            book,
            account: None,
            source: BookSource::Local,
            card: Card {
                emails: vec![Typed::new(email, "")],
                ..Card::default()
            },
            starred: false,
            labels: labels.iter().map(|l| l.to_string()).collect(),
        };
        let cards = [
            card(1, 10, "a@x.in", &["Family"]),
            card(2, 10, "b@x.in", &["Work"]),
            card(3, 20, "c@x.in", &[]),
        ];
        let (write, delete, undo) = merge_plan(&cards);
        assert_eq!(write.iter().map(|w| w.id).collect::<Vec<_>>(), [1, 3]);
        assert_eq!(write[0].card.emails.len(), 3);
        assert_eq!(write[1].labels, ["Family", "Work"]);
        assert_eq!(delete, [2]);
        assert_eq!(undo.len(), 3);
        assert_eq!((undo[1].id, undo[1].book), (0, 10));
        assert_eq!(undo[1].labels, ["Work"]);
    }
}
