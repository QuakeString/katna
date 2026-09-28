// SPDX-License-Identifier: GPL-3.0-or-later

//! Open and click tracking (`pim.db`, schema v3): which message and
//! recipient each random tracking ID stands for, and the events the
//! server reported (`docs/ARCHITECTURE.md` §16.1).

use std::collections::HashMap;

use katna_core::AccountId;
use rusqlite::{OptionalExtension, params};

use crate::Store;
use crate::error::Result;

/// A recipient of a message about to be tracked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewRecipient<'a> {
    pub tracking_id: &'a str,
    pub email: &'a str,
    pub name: Option<&'a str>,
}

/// One recipient's tracked copy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackedRecipient {
    pub tracking_id: String,
    pub email: String,
    pub name: Option<String>,
    /// When this copy went out (Unix seconds); `None` while waiting.
    pub sent_at: Option<i64>,
}

/// A message sent with tracking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackedMessage {
    pub id: i64,
    pub outbox_id: i64,
    pub account: AccountId,
    /// `Message-ID`, without angle brackets.
    pub message_id: String,
    pub subject: String,
    /// Link targets, numbered as in the tracked copies.
    pub links: Vec<String>,
    /// When every copy went out (Unix seconds).
    pub sent_at: Option<i64>,
    pub recipients: Vec<TrackedRecipient>,
}

/// An open or click.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackingEvent {
    /// The server's event number.
    pub seq: i64,
    pub tracking_id: String,
    /// `open` or `click`.
    pub kind: String,
    /// For a click, the link's number.
    pub link: Option<i64>,
    /// `person`, `apple_proxy` or `scanner`.
    pub source: String,
    /// Unix milliseconds.
    pub at: i64,
}

/// What a new event means, for a notification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackingNews {
    pub message: TrackedMessage,
    pub recipient: TrackedRecipient,
    /// The first event of this kind from a person for this recipient.
    pub first: bool,
    /// For a click, the link's target.
    pub link: Option<String>,
}

/// What one recipient did with their tracked copy. Scanner events are
/// left out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipientActivity {
    pub email: String,
    pub name: Option<String>,
    /// Opens by a person.
    pub opens: u32,
    /// Opens through Apple's mail privacy proxy, which fetches pictures
    /// whether or not the mail is read: "maybe opened".
    pub maybe_opens: u32,
    /// Links followed by a person.
    pub clicks: u32,
    /// The latest open or click (Unix milliseconds).
    pub last: Option<i64>,
}

impl RecipientActivity {
    pub fn opened(&self) -> bool {
        self.opens > 0 || self.clicks > 0
    }
}

/// One open or click by a person, for the Activity feed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityItem {
    /// The server's event number: newer events have larger ones.
    pub seq: i64,
    /// `true` for a followed link, `false` for an open.
    pub click: bool,
    /// Through Apple's mail privacy proxy: "maybe opened".
    pub maybe: bool,
    /// For a click, the link's target.
    pub link: Option<String>,
    /// Unix milliseconds.
    pub at: i64,
    pub email: String,
    pub name: Option<String>,
    pub subject: String,
    pub account: AccountId,
    /// The message's `Message-ID`, without angle brackets.
    pub message_id: String,
}

/// What the recipients of a tracked message did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageActivity {
    pub subject: String,
    /// When every copy went out (Unix seconds).
    pub sent_at: Option<i64>,
    pub links: usize,
    pub recipients: Vec<RecipientActivity>,
}

impl MessageActivity {
    /// Recipients who opened it (a click counts as an open).
    pub fn opened(&self) -> usize {
        self.recipients.iter().filter(|r| r.opened()).count()
    }

    /// Recipients who followed a link.
    pub fn clicked(&self) -> usize {
        self.recipients.iter().filter(|r| r.clicks > 0).count()
    }
}

impl Store {
    /// Remembers that outbox entry `outbox_id` is sent with tracking, with
    /// one tracking ID per recipient. Returns the row's ID.
    #[allow(clippy::too_many_arguments)]
    pub fn start_tracking(
        &mut self,
        outbox_id: i64,
        account: AccountId,
        message_id: &str,
        subject: &str,
        links: &[String],
        recipients: &[NewRecipient<'_>],
        now: i64,
    ) -> Result<i64> {
        self.check_writable()?;
        let links = serde_json::to_string(links).unwrap_or_else(|_| "[]".into());
        let tx = self.pim.transaction()?;
        tx.execute(
            "INSERT INTO tracked_message
                 (outbox_id, account_id, message_id_hdr, subject, links_json, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![outbox_id, account.0, message_id, subject, links, now],
        )?;
        let id = tx.last_insert_rowid();
        {
            let mut insert = tx.prepare_cached(
                "INSERT INTO tracked_recipient (tracking_id, tracked_id, email, name)
                 VALUES (?1, ?2, ?3, ?4)",
            )?;
            for recipient in recipients {
                insert.execute(params![
                    recipient.tracking_id,
                    id,
                    recipient.email,
                    recipient.name
                ])?;
            }
        }
        tx.commit()?;
        Ok(id)
    }

    /// The tracking of outbox entry `outbox_id`, if it has any yet.
    pub fn tracking_for_outbox(&self, outbox_id: i64) -> Result<Option<TrackedMessage>> {
        self.tracked_where("outbox_id = ?1", params![outbox_id])
            .map(|mut found| found.pop())
    }

    /// The tracking of the sent message with `Message-ID` `message_id`
    /// (without angle brackets).
    pub fn tracking_for_message(&self, message_id: &str) -> Result<Option<TrackedMessage>> {
        self.tracked_where("message_id_hdr = ?1", params![message_id])
            .map(|mut found| found.pop())
    }

    /// Every tracked message, newest first, at most `limit`.
    pub fn tracked_messages(&self, limit: u32) -> Result<Vec<TrackedMessage>> {
        let mut found = self.tracked_where("1 = 1", params![])?;
        found.reverse();
        found.truncate(limit as usize);
        Ok(found)
    }

    fn tracked_where(
        &self,
        condition: &str,
        args: &[&dyn rusqlite::ToSql],
    ) -> Result<Vec<TrackedMessage>> {
        let mut stmt = self.pim.prepare(&format!(
            "SELECT id, outbox_id, account_id, message_id_hdr, subject, links_json, sent_at
             FROM tracked_message WHERE {condition} ORDER BY id"
        ))?;
        let rows = stmt.query_map(args, |row| {
            let links: String = row.get(5)?;
            Ok(TrackedMessage {
                id: row.get(0)?,
                outbox_id: row.get(1)?,
                account: AccountId(row.get(2)?),
                message_id: row.get(3)?,
                subject: row.get(4)?,
                links: serde_json::from_str(&links).unwrap_or_default(),
                sent_at: row.get(6)?,
                recipients: Vec::new(),
            })
        })?;
        let mut found: Vec<TrackedMessage> = rows.collect::<rusqlite::Result<_>>()?;
        let mut stmt = self.pim.prepare_cached(
            "SELECT tracking_id, email, name, sent_at FROM tracked_recipient
             WHERE tracked_id = ?1 ORDER BY rowid",
        )?;
        for message in &mut found {
            let rows = stmt.query_map([message.id], recipient)?;
            message.recipients = rows.collect::<rusqlite::Result<_>>()?;
        }
        Ok(found)
    }

    /// Notes that the copy for `tracking_id` went out at `now`.
    pub fn tracked_copy_sent(&mut self, tracking_id: &str, now: i64) -> Result<()> {
        self.check_writable()?;
        self.pim.execute(
            "UPDATE tracked_recipient SET sent_at = ?2 WHERE tracking_id = ?1",
            params![tracking_id, now],
        )?;
        Ok(())
    }

    /// Notes that every copy of tracked message `id` went out.
    pub fn tracking_sent(&mut self, id: i64, now: i64) -> Result<()> {
        self.check_writable()?;
        self.pim.execute(
            "UPDATE tracked_message SET sent_at = ?2 WHERE id = ?1",
            params![id, now],
        )?;
        Ok(())
    }

    /// Forgets the tracking of outbox entry `outbox_id` (sent without it,
    /// or discarded). Returns the tracking IDs it had.
    pub fn forget_tracking(&mut self, outbox_id: i64) -> Result<Vec<String>> {
        self.check_writable()?;
        let ids = self
            .tracking_for_outbox(outbox_id)?
            .map(|m| m.recipients.into_iter().map(|r| r.tracking_id).collect())
            .unwrap_or_default();
        self.pim.execute(
            "DELETE FROM tracked_message WHERE outbox_id = ?1",
            [outbox_id],
        )?;
        Ok(ids)
    }

    /// The number of the newest event kept, 0 when none.
    pub fn last_tracking_seq(&self) -> Result<i64> {
        Ok(self.pim.query_row(
            "SELECT coalesce(max(seq), 0) FROM tracking_event",
            [],
            |row| row.get(0),
        )?)
    }

    /// Whether any message was sent with tracking, so the event stream is
    /// worth keeping open.
    pub fn has_tracking(&self) -> Result<bool> {
        Ok(self
            .pim
            .query_row("SELECT EXISTS (SELECT 1 FROM tracked_message)", [], |row| {
                row.get(0)
            })?)
    }

    /// Keeps an event. Returns what it means when it is new and its
    /// tracking ID is known here.
    pub fn add_tracking_event(&mut self, event: &TrackingEvent) -> Result<Option<TrackingNews>> {
        self.check_writable()?;
        let added = self.pim.execute(
            "INSERT OR IGNORE INTO tracking_event (seq, tracking_id, kind, link, source, at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                event.seq,
                event.tracking_id,
                event.kind,
                event.link,
                event.source,
                event.at
            ],
        )?;
        if added == 0 {
            return Ok(None);
        }
        let tracked: Option<i64> = self
            .pim
            .query_row(
                "SELECT tracked_id FROM tracked_recipient WHERE tracking_id = ?1",
                [&event.tracking_id],
                |row| row.get(0),
            )
            .optional()?;
        let Some(tracked) = tracked else {
            return Ok(None);
        };
        let Some(message) = self.tracked_where("id = ?1", params![tracked])?.pop() else {
            return Ok(None);
        };
        let Some(recipient) = message
            .recipients
            .iter()
            .find(|r| r.tracking_id == event.tracking_id)
            .cloned()
        else {
            return Ok(None);
        };
        let earlier: i64 = self.pim.query_row(
            "SELECT count(*) FROM tracking_event
             WHERE tracking_id = ?1 AND kind = ?2 AND source = 'person' AND seq < ?3",
            params![event.tracking_id, event.kind, event.seq],
            |row| row.get(0),
        )?;
        let link = event
            .link
            .and_then(|n| usize::try_from(n).ok())
            .and_then(|n| message.links.get(n).cloned());
        Ok(Some(TrackingNews {
            first: event.source == "person" && earlier == 0,
            link,
            message,
            recipient,
        }))
    }

    /// The newest stored message of `account` with `Message-ID`
    /// `message_id` (without angle brackets) that is in a folder: the
    /// copy in Sent, once synced.
    pub fn filed_message(
        &self,
        account: AccountId,
        message_id: &str,
    ) -> Result<Option<crate::MessageId>> {
        Ok(self
            .mail
            .query_row(
                "SELECT m.id FROM message m
                 WHERE m.account_id = ?1 AND m.message_id_hdr = ?2
                   AND EXISTS (SELECT 1 FROM message_location l WHERE l.message_id = m.id)
                 ORDER BY m.id DESC LIMIT 1",
                params![account.0, message_id],
                |row| row.get(0).map(crate::MessageId),
            )
            .optional()?)
    }

    /// The activity of the tracked ones among `messages`.
    pub fn tracking_activity(
        &self,
        messages: &[crate::MessageId],
    ) -> Result<HashMap<crate::MessageId, MessageActivity>> {
        let mut found = HashMap::new();
        if messages.is_empty() || !self.has_tracking()? {
            return Ok(found);
        }
        for &message in messages {
            let Some(id) = self.message_id_header(message)? else {
                continue;
            };
            if let Some(tracked) = self.tracking_for_message(&id)? {
                found.insert(message, self.activity(tracked)?);
            }
        }
        Ok(found)
    }

    /// The activity of `tracked`.
    pub fn activity(&self, tracked: TrackedMessage) -> Result<MessageActivity> {
        let mut recipients = Vec::with_capacity(tracked.recipients.len());
        for recipient in tracked.recipients {
            let mut activity = RecipientActivity {
                email: recipient.email,
                name: recipient.name,
                opens: 0,
                maybe_opens: 0,
                clicks: 0,
                last: None,
            };
            for event in self.tracking_events(&recipient.tracking_id)? {
                match (event.kind.as_str(), event.source.as_str()) {
                    ("open", "person") => activity.opens += 1,
                    ("open", "apple_proxy") => activity.maybe_opens += 1,
                    ("click", "person" | "apple_proxy") => activity.clicks += 1,
                    _ => continue,
                }
                activity.last = activity.last.max(Some(event.at));
            }
            recipients.push(activity);
        }
        Ok(MessageActivity {
            subject: tracked.subject,
            sent_at: tracked.sent_at,
            links: tracked.links.len(),
            recipients,
        })
    }

    /// Opens and clicks by people (and Apple's proxy) at or after `since`
    /// (Unix milliseconds) and after event `after`, newest first, at most
    /// `limit`.
    pub fn activity_feed(&self, since: i64, after: i64, limit: u32) -> Result<Vec<ActivityItem>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT e.seq, e.kind, e.source, e.link, e.at, r.email, r.name,
                    m.subject, m.account_id, m.message_id_hdr, m.links_json
             FROM tracking_event e
             JOIN tracked_recipient r ON r.tracking_id = e.tracking_id
             JOIN tracked_message m ON m.id = r.tracked_id
             WHERE e.source IN ('person', 'apple_proxy') AND e.at >= ?1 AND e.seq > ?3
             ORDER BY e.at DESC, e.seq DESC LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![since, limit, after], |row| {
            let kind: String = row.get(1)?;
            let source: String = row.get(2)?;
            let link: Option<i64> = row.get(3)?;
            let links: String = row.get(10)?;
            let link = link.and_then(|n| {
                let links: Vec<String> = serde_json::from_str(&links).unwrap_or_default();
                usize::try_from(n).ok().and_then(|n| links.get(n).cloned())
            });
            let click = kind == "click";
            Ok(ActivityItem {
                seq: row.get(0)?,
                click,
                // A followed link is a person, whoever fetched it.
                maybe: !click && source == "apple_proxy",
                link,
                at: row.get(4)?,
                email: row.get(5)?,
                name: row.get(6)?,
                subject: row.get(7)?,
                account: AccountId(row.get(8)?),
                message_id: row.get(9)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// How many opens and clicks by people came after event `seq`.
    pub fn activity_after(&self, seq: i64) -> Result<usize> {
        let count: i64 = self.pim.query_row(
            "SELECT count(*) FROM tracking_event e
             WHERE e.seq > ?1 AND e.source = 'person'
               AND EXISTS (SELECT 1 FROM tracked_recipient r
                           WHERE r.tracking_id = e.tracking_id)",
            [seq],
            |row| row.get(0),
        )?;
        Ok(usize::try_from(count).unwrap_or(0))
    }

    /// The events of `tracking_id`, oldest first.
    pub fn tracking_events(&self, tracking_id: &str) -> Result<Vec<TrackingEvent>> {
        let mut stmt = self.pim.prepare_cached(
            "SELECT seq, tracking_id, kind, link, source, at FROM tracking_event
             WHERE tracking_id = ?1 ORDER BY at, seq",
        )?;
        let rows = stmt.query_map([tracking_id], |row| {
            Ok(TrackingEvent {
                seq: row.get(0)?,
                tracking_id: row.get(1)?,
                kind: row.get(2)?,
                link: row.get(3)?,
                source: row.get(4)?,
                at: row.get(5)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }
}

fn recipient(row: &rusqlite::Row<'_>) -> rusqlite::Result<TrackedRecipient> {
    Ok(TrackedRecipient {
        tracking_id: row.get(0)?,
        email: row.get(1)?,
        name: row.get(2)?,
        sent_at: row.get(3)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Mode;
    use katna_core::Paths;

    fn store() -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(dir.path());
        let store = Store::open(&paths, Mode::ReadWrite).unwrap();
        (dir, store)
    }

    fn event(seq: i64, id: &str, kind: &str, source: &str) -> TrackingEvent {
        TrackingEvent {
            seq,
            tracking_id: id.into(),
            kind: kind.into(),
            link: (kind == "click").then_some(0),
            source: source.into(),
            at: seq * 1000,
        }
    }

    #[test]
    fn tracks_copies_and_events() {
        let (_dir, mut store) = store();
        assert!(!store.has_tracking().unwrap());
        let links = vec!["https://example.com/".to_owned()];
        let recipients = [
            NewRecipient {
                tracking_id: "aa",
                email: "b@y.org",
                name: Some("Bea"),
            },
            NewRecipient {
                tracking_id: "bb",
                email: "c@z.org",
                name: None,
            },
        ];
        let id = store
            .start_tracking(
                5,
                AccountId(1),
                "m1@x.org",
                "Proposal v2",
                &links,
                &recipients,
                100,
            )
            .unwrap();
        assert!(store.has_tracking().unwrap());
        store.tracked_copy_sent("aa", 101).unwrap();
        let found = store.tracking_for_outbox(5).unwrap().unwrap();
        assert_eq!(found.id, id);
        assert_eq!(found.recipients[0].sent_at, Some(101));
        assert_eq!(found.recipients[1].sent_at, None);
        store.tracking_sent(id, 102).unwrap();
        assert_eq!(
            store
                .tracking_for_message("m1@x.org")
                .unwrap()
                .unwrap()
                .sent_at,
            Some(102)
        );

        // A scanner first, then the person: the person's open is "first".
        let news = store
            .add_tracking_event(&event(1, "aa", "open", "scanner"))
            .unwrap()
            .unwrap();
        assert!(!news.first);
        let news = store
            .add_tracking_event(&event(2, "aa", "open", "person"))
            .unwrap()
            .unwrap();
        assert!(news.first);
        assert_eq!(news.recipient.name.as_deref(), Some("Bea"));
        assert_eq!(news.message.subject, "Proposal v2");
        assert!(
            !store
                .add_tracking_event(&event(3, "aa", "open", "person"))
                .unwrap()
                .unwrap()
                .first
        );
        let click = store
            .add_tracking_event(&event(4, "aa", "click", "person"))
            .unwrap()
            .unwrap();
        assert!(click.first);
        assert_eq!(click.link.as_deref(), Some("https://example.com/"));
        // Seen before, or not ours: nothing to tell.
        assert_eq!(
            store
                .add_tracking_event(&event(4, "aa", "click", "person"))
                .unwrap(),
            None
        );
        assert_eq!(
            store
                .add_tracking_event(&event(5, "zz", "open", "person"))
                .unwrap(),
            None
        );
        assert_eq!(store.last_tracking_seq().unwrap(), 5);
        assert_eq!(store.tracking_events("aa").unwrap().len(), 4);
        assert_eq!(store.tracked_messages(10).unwrap().len(), 1);

        let activity = store
            .activity(store.tracking_for_outbox(5).unwrap().unwrap())
            .unwrap();
        assert_eq!(activity.subject, "Proposal v2");
        assert_eq!((activity.opened(), activity.clicked()), (1, 1));
        let bea = &activity.recipients[0];
        // The scanner's open does not count.
        assert_eq!((bea.opens, bea.clicks, bea.last), (2, 1, Some(4000)));
        assert!(!activity.recipients[1].opened());

        // The feed: people only, newest first, with the link's target.
        let feed = store.activity_feed(0, 0, 10).unwrap();
        let seqs: Vec<i64> = feed.iter().map(|item| item.seq).collect();
        assert_eq!(seqs, [4, 3, 2]);
        assert!(feed[0].click);
        assert_eq!(feed[0].link.as_deref(), Some("https://example.com/"));
        assert_eq!(feed[0].name.as_deref(), Some("Bea"));
        assert_eq!(feed[0].message_id, "m1@x.org");
        assert_eq!(store.activity_feed(3500, 0, 10).unwrap().len(), 1);
        assert_eq!(store.activity_feed(0, 0, 1).unwrap().len(), 1);
        assert_eq!(store.activity_feed(0, 3, 10).unwrap().len(), 1);
        assert_eq!(store.activity_after(0).unwrap(), 3);
        assert_eq!(store.activity_after(3).unwrap(), 1);

        assert_eq!(store.forget_tracking(5).unwrap(), ["aa", "bb"]);
        assert!(store.tracking_for_outbox(5).unwrap().is_none());
    }
}
