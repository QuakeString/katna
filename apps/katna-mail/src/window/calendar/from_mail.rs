// SPDX-License-Identifier: GPL-3.0-or-later

//! Open the mail an event came from. Gmail makes events from the mail it
//! reads (trains, flights, hotels) and links back with an id only Gmail
//! understands, so Katna looks for that mail in its own search: the
//! event's words, in mail that came before it. When none is found, the
//! Gmail link opens in the browser.

use gpui::{Context, Window};
use jiff::ToSpan;
use jiff::civil::Date;
use katna_dav::Occurrence;

use super::super::MailWindow;
use super::super::apps::App;
use super::description;
use crate::data;

/// Words that name the kind of booking, not the booking.
const COMMON: &[&str] = &[
    "a",
    "an",
    "and",
    "at",
    "by",
    "for",
    "from",
    "in",
    "of",
    "on",
    "the",
    "to",
    "with",
    "train",
    "flight",
    "bus",
    "stay",
    "hotel",
    "reservation",
    "booking",
    "trip",
    "event",
    "check",
    "jn",
];

/// Most words searched for.
const MAX_WORDS: usize = 8;

/// The link Gmail puts in an event it made from a mail, if it is one.
pub(super) fn gmail_link(description: &str) -> Option<String> {
    description::tidy(description)
        .1
        .into_iter()
        .map(|(_, url)| url)
        .find(|url| url.starts_with("https://mail.google.com/mail") && url.contains("extsrc=cal"))
}

/// The words of `text` worth searching for.
fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.chars().count() >= 2)
        .filter(|w| !COMMON.contains(&w.to_lowercase().as_str()))
        .map(str::to_owned)
        .collect()
}

/// The searches to try, closest first: the title and place, then the
/// title alone, each in the year of mail before the event's day.
pub(super) fn queries(title: &str, location: &str, day: Date) -> Vec<String> {
    let dates = format!(
        "after:{} before:{}",
        day.saturating_sub(1.year()),
        day.saturating_add(1.day())
    );
    let mut all = words(title);
    for word in words(location) {
        if !all.contains(&word) {
            all.push(word);
        }
    }
    all.truncate(MAX_WORDS);
    let mut title = words(title);
    title.truncate(MAX_WORDS);
    let mut out = Vec::new();
    for set in [all, title] {
        if set.is_empty() {
            continue;
        }
        let query = format!("{} {dates}", set.join(" "));
        if !out.contains(&query) {
            out.push(query);
        }
    }
    out
}

impl MailWindow {
    /// Open the mail (an event Gmail made from one): the mail in Katna
    /// when its search finds it in the event's account, else Gmail's link.
    pub(super) fn open_event_mail(
        &mut self,
        occurrence: &Occurrence,
        gmail: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let data = &occurrence.event.data;
        let day = super::civil(occurrence.start, &self.tz).date();
        let queries = queries(&data.title, &data.location, day);
        let account = self
            .calendar
            .calendars
            .iter()
            .find(|c| c.id == occurrence.event.calendar_id)
            .and_then(|c| c.account);
        let Some(index) = self.mail.as_mut().ok().and_then(|m| m.index()) else {
            cx.open_url(&gmail);
            return;
        };
        cx.spawn_in(window, async move |this, cx| {
            let now = jiff::Timestamp::now().as_second();
            let found = cx
                .background_executor()
                .spawn(async move {
                    queries
                        .iter()
                        .filter_map(|q| data::search(&index, q, now, false).ok())
                        .map(|(results, _)| results.hits)
                        .find(|hits| !hits.is_empty())
                        .unwrap_or_default()
                })
                .await;
            this.update_in(cx, |this, window, cx| {
                let message = this.mail.as_ref().ok().and_then(|mail| {
                    found
                        .iter()
                        .map(|hit| hit.message)
                        .find(|m| account.is_none() || mail.message_account(*m) == account)
                });
                match message {
                    Some(message) => {
                        this.calendar.open = None;
                        this.open_app(App::Mail, cx);
                        this.show_message(message, window, cx);
                    }
                    None => cx.open_url(&gmail),
                }
            })
            .ok();
        })
        .detach();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gmail_s_own_link_is_found() {
        let text = "This event was created from an email that you received in Gmail. \
                    https://mail.google.com/mail?extsrc=cal&plid=ACUX6DM16ZEJ\n";
        assert_eq!(
            gmail_link(text).as_deref(),
            Some("https://mail.google.com/mail?extsrc=cal&plid=ACUX6DM16ZEJ")
        );
        assert_eq!(
            gmail_link("See https://mail.google.com/mail/u/0/#inbox"),
            None
        );
    }

    #[test]
    fn a_train_is_searched_by_its_stations() {
        let day = Date::new(2026, 9, 15).unwrap();
        assert_eq!(
            queries("Train to HOWRAH JN (HWH)", "RANIGANJ (RNG)", day),
            [
                "HOWRAH HWH RANIGANJ RNG after:2025-09-15 before:2026-09-16",
                "HOWRAH HWH after:2025-09-15 before:2026-09-16",
            ]
        );
        assert_eq!(
            queries("Flight to Kolkata", "", day),
            ["Kolkata after:2025-09-15 before:2026-09-16"]
        );
        assert!(queries("Train to", "", day).is_empty());
    }
}
