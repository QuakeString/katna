// SPDX-License-Identifier: GPL-3.0-or-later

//! Nudges: mail the user sent that asked something and got no answer in a
//! few days comes back to the top of the Inbox with "Sent 3 days ago.
//! Follow up?", without anything set on it (like Gmail's nudges). The
//! daemon looks at each sent message once and keeps what it found beside
//! it; the app shows the ones that still wait. See `docs/ARCHITECTURE.md`
//! §10.1.

use katna_store::{MessageId, Store};
use serde::{Deserialize, Serialize};

use crate::{counts_as_reply, kind, plugin, to_json};

/// How long after it was sent a question with no answer gets a nudge.
pub const NUDGE_AFTER: i64 = 3 * 24 * 3600;
/// Mail older than this gets no nudge: by then it is not news.
pub const NUDGE_UNTIL: i64 = 14 * 24 * 3600;

/// What the daemon found on a sent message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Nudge {
    /// When it was sent (Unix seconds).
    pub sent: i64,
    /// It asked something, so it gets a nudge once [`NUDGE_AFTER`] passed.
    pub asks: bool,
    /// The user dismissed it.
    #[serde(default)]
    pub dismissed: bool,
}

/// Keeps what the daemon found on sent `message`.
pub fn set_nudge(store: &mut Store, message: MessageId, nudge: &Nudge) -> katna_store::Result<()> {
    store.set_meta(
        kind::MESSAGE,
        message.0,
        plugin::NUDGE,
        &to_json(nudge),
        None,
    )
}

/// What the daemon found on `message`, if it looked.
pub fn nudge_of(store: &Store, message: MessageId) -> katna_store::Result<Option<Nudge>> {
    Ok(store
        .meta(kind::MESSAGE, message.0, plugin::NUDGE)?
        .and_then(|row| serde_json::from_str(&row.value_json).ok()))
}

/// Every sent message the daemon looked at, with what it found.
pub fn nudges(store: &Store) -> katna_store::Result<Vec<(MessageId, Nudge)>> {
    Ok(store
        .meta_of(plugin::NUDGE)?
        .into_iter()
        .filter(|row| row.object_kind == kind::MESSAGE)
        .filter_map(|row| {
            let value: Nudge = serde_json::from_str(&row.value_json).ok()?;
            Some((MessageId(row.object_id), value))
        })
        .collect())
}

pub fn clear_nudge(store: &mut Store, message: MessageId) -> katna_store::Result<bool> {
    store.remove_meta(kind::MESSAGE, message.0, plugin::NUDGE)
}

/// The nudges to show at `now`: questions sent between [`NUDGE_UNTIL`] and
/// [`NUDGE_AFTER`] ago, not dismissed, that nobody answered yet (an
/// automatic answer such as an out-of-office does not count).
pub fn waiting_nudges(store: &Store, now: i64) -> katna_store::Result<Vec<(MessageId, Nudge)>> {
    let mut list = Vec::new();
    for (message, nudge) in nudges(store)? {
        if !nudge.asks || nudge.dismissed || !due(nudge.sent, now) {
            continue;
        }
        let later = store.later_in_thread(message)?;
        if later.iter().any(|l| counts_as_reply(l, &[])) {
            continue;
        }
        list.push((message, nudge));
    }
    // Newest question first.
    list.sort_by_key(|(_, n)| std::cmp::Reverse(n.sent));
    Ok(list)
}

/// Whether mail sent at `sent` is old enough for a nudge at `now`, and
/// not too old.
pub fn due(sent: i64, now: i64) -> bool {
    let age = now - sent;
    (NUDGE_AFTER..=NUDGE_UNTIL).contains(&age)
}

/// Whether the user's own words in `text` (a message's body, as plain
/// text) ask something: a question mark before the quoted mail it
/// answers.
pub fn asks(text: &str) -> bool {
    let mut own = String::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('>') || quote_header(trimmed) || trimmed.starts_with("-----") {
            break;
        }
        // A signature ends what the user wrote.
        if line == "-- " || line == "--" {
            break;
        }
        own.push_str(line);
        own.push('\n');
    }
    own.contains('?') || own.contains('？')
}

/// "On Mon, 5 Oct 2026, Sara wrote:", the line over a quoted mail.
fn quote_header(line: &str) -> bool {
    let line = line.trim_end();
    line.ends_with("wrote:") || line.ends_with("a écrit :") || line.ends_with("schrieb:")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_question_in_own_words_asks() {
        assert!(asks("Hi Sara,\nCould you send the form?\nThanks"));
        assert!(asks("能帮忙吗？"));
        assert!(!asks("Hi Sara,\nHere is the form.\nThanks"));
    }

    #[test]
    fn a_question_in_the_quote_or_signature_does_not() {
        let reply = "Sure, done.\n\nOn Mon, 5 Oct 2026, Sara wrote:\n> Can you send it?";
        assert!(!asks(reply));
        assert!(!asks("Done.\n> Can you send it?"));
        assert!(!asks("Done.\n-- \nWhy not visit example.org?"));
        assert!(!asks(
            "See below.\n---------- Forwarded message ---------\nAny news?"
        ));
    }

    #[test]
    fn nudges_wait_three_days_and_end_after_two_weeks() {
        let day = 24 * 3600;
        assert!(!due(0, 2 * day));
        assert!(due(0, 3 * day));
        assert!(due(0, 14 * day));
        assert!(!due(0, 15 * day));
    }
}
