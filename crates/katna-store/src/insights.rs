// SPDX-License-Identifier: GPL-3.0-or-later

//! Mailbox insights for the Activity report, counted from `mail.db` alone:
//! mail sent and received in a period, how often and how fast replies
//! come, the people most written with, and when mail arrives
//! (`docs/ARCHITECTURE.md` §16.1).

use std::collections::HashMap;

use rusqlite::params;

use katna_core::AccountId;

use crate::Store;
use crate::error::Result;

/// How far after a message its reply is still looked for (Unix seconds).
const REPLY_WINDOW: i64 = 14 * 24 * 3600;
/// At most this many people are listed.
const TOP: usize = 10;

/// Replies to one side's mail.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Replies {
    /// Messages that could be answered.
    pub messages: usize,
    /// Of those, the ones answered within two weeks.
    pub replied: usize,
    /// The middle time to answer, in seconds.
    pub median: Option<i64>,
}

/// Someone mail was exchanged with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Partner {
    pub email: String,
    pub name: Option<String>,
    /// Messages from them.
    pub received: usize,
    /// Messages to them.
    pub sent: usize,
}

/// What a mailbox did in a period.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Insights {
    pub sent: usize,
    pub received: usize,
    /// Replies from the user to mail from others.
    pub mine: Replies,
    /// Replies from others to the user's mail.
    pub theirs: Replies,
    /// Most mail exchanged first.
    pub people: Vec<Partner>,
    /// Mail received by weekday (Monday first) and local hour.
    pub hours: [[u32; 24]; 7],
}

/// One message, as the counting needs it.
struct Seen {
    thread: Option<i64>,
    date: i64,
    mine: bool,
    list: bool,
}

impl Store {
    /// Counts the mail dated from `since` to before `until` (Unix seconds),
    /// in `account` only or in every account (`None`). `me` are the user's own addresses (lower case); `local` gives a
    /// time's weekday (0 = Monday) and hour in the user's time zone. Mail
    /// only in Drafts, Junk or Trash, and mail not yet filed, is left out.
    pub fn mailbox_insights(
        &self,
        account: Option<AccountId>,
        me: &[String],
        since: i64,
        until: i64,
        local: impl Fn(i64) -> (usize, usize),
    ) -> Result<Insights> {
        let mut insights = Insights::default();
        // Replies to mail near the end of the period come after it.
        let mut stmt = self.mail.prepare(
            "SELECT m.id, m.thread_id, m.date, m.list_id IS NOT NULL,
                    (SELECT p.email_norm FROM participant p
                     WHERE p.message_id = m.id AND p.role = 'from' LIMIT 1)
             FROM message m
             WHERE m.date >= ?1 AND m.date < ?2
               AND (?3 IS NULL OR m.account_id = ?3)
               AND EXISTS (SELECT 1 FROM message_location l
                           JOIN folder f ON f.id = l.folder_id
                           WHERE l.message_id = m.id
                             AND coalesce(f.role, '') NOT IN ('drafts', 'junk', 'trash'))
             ORDER BY m.date",
        )?;
        let rows = stmt.query_map(
            params![
                since,
                until.saturating_add(REPLY_WINDOW),
                account.map(|a| a.0)
            ],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, Option<i64>>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, bool>(3)?,
                    row.get::<_, Option<String>>(4)?,
                ))
            },
        )?;
        let mut messages = Vec::new();
        let mut ids = Vec::new();
        for row in rows {
            let (id, thread, date, list, from) = row?;
            let mine = from.as_ref().is_some_and(|f| me.iter().any(|m| m == f));
            messages.push(Seen {
                thread,
                date,
                mine,
                list,
            });
            if date < until {
                ids.push((id, mine, from));
            }
        }
        drop(stmt);

        // Counts, people and hours over the period itself.
        let mut people: HashMap<String, Partner> = HashMap::new();
        let mut names = self.mail.prepare_cached(
            "SELECT role, email_norm, display_name FROM participant WHERE message_id = ?1",
        )?;
        for (id, mine, _) in &ids {
            let rows = names.query_map([id], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                ))
            })?;
            for row in rows {
                let (role, email, name) = row?;
                let counts = if *mine {
                    matches!(role.as_str(), "to" | "cc" | "bcc")
                } else {
                    role == "from"
                };
                if !counts || me.contains(&email) {
                    continue;
                }
                let person = people.entry(email.clone()).or_insert(Partner {
                    email,
                    name: None,
                    received: 0,
                    sent: 0,
                });
                if let Some(name) = name.filter(|n| !n.trim().is_empty()) {
                    person.name = Some(name);
                }
                if *mine {
                    person.sent += 1;
                } else {
                    person.received += 1;
                }
            }
        }
        for message in messages.iter().filter(|m| m.date < until) {
            if message.mine {
                insights.sent += 1;
            } else {
                insights.received += 1;
                let (day, hour) = local(message.date);
                if let Some(cell) = insights.hours.get_mut(day).and_then(|d| d.get_mut(hour)) {
                    *cell += 1;
                }
            }
        }
        let mut people: Vec<Partner> = people.into_values().collect();
        people.sort_by(|a, b| {
            (b.sent + b.received)
                .cmp(&(a.sent + a.received))
                .then_with(|| a.email.cmp(&b.email))
        });
        people.truncate(TOP);
        insights.people = people;
        (insights.mine, insights.theirs) = replies(&messages, until);
        Ok(insights)
    }
}

/// Replies by the user to others' mail and by others to the user's, for
/// mail dated before `until`: the next message in the same conversation
/// from the other side within two weeks. Mailing-list mail is not
/// expected to be answered.
fn replies(messages: &[Seen], until: i64) -> (Replies, Replies) {
    let mut threads: HashMap<i64, Vec<&Seen>> = HashMap::new();
    for message in messages {
        if let Some(thread) = message.thread {
            threads.entry(thread).or_default().push(message);
        }
    }
    let mut mine = (0, Vec::new());
    let mut theirs = (0, Vec::new());
    for message in messages.iter().filter(|m| m.date < until && !m.list) {
        let side = if message.mine { &mut theirs } else { &mut mine };
        side.0 += 1;
        let answer = message
            .thread
            .and_then(|t| threads.get(&t))
            .and_then(|thread| {
                thread
                    .iter()
                    .filter(|m| m.mine != message.mine && m.date > message.date)
                    .map(|m| m.date - message.date)
                    .filter(|&after| after <= REPLY_WINDOW)
                    .min()
            });
        if let Some(after) = answer {
            side.1.push(after);
        }
    }
    let done = |(messages, mut times): (usize, Vec<i64>)| {
        times.sort_unstable();
        Replies {
            messages,
            replied: times.len(),
            median: (!times.is_empty()).then(|| times[times.len() / 2]),
        }
    };
    (done(mine), done(theirs))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seen(thread: i64, date: i64, mine: bool) -> Seen {
        Seen {
            thread: Some(thread),
            date,
            mine,
            list: false,
        }
    }

    #[test]
    fn replies_count_the_other_side() {
        let hour = 3600;
        let messages = [
            // They wrote, I answered an hour later, they answered in two.
            seen(1, 0, false),
            seen(1, hour, true),
            seen(1, 3 * hour, false),
            // I wrote, nobody answered.
            seen(2, 10 * hour, true),
            // They wrote, I answered after three weeks: too late.
            seen(3, 0, false),
            seen(3, 21 * 24 * hour, true),
            // A list message is not expected to be answered.
            Seen {
                thread: Some(4),
                date: 0,
                mine: false,
                list: true,
            },
        ];
        // The late answer is after the period.
        let (mine, theirs) = replies(&messages, 20 * hour);
        assert_eq!(
            mine,
            Replies {
                messages: 3,
                replied: 1,
                median: Some(hour)
            }
        );
        assert_eq!(
            theirs,
            Replies {
                messages: 2,
                replied: 1,
                median: Some(2 * hour)
            }
        );
    }
}
