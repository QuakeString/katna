// SPDX-License-Identifier: GPL-3.0-or-later

//! What a follow-up needs to know, for the daemon that sends it and the
//! app that shows it: whether the conversation got an answer, and when
//! working hours let it go out.

use jiff::tz::TimeZone;
use katna_core::{AccountId, MailCategory};
use katna_store::{LaterMessage, Store};

use crate::FollowUp;

/// Working hours, when follow-ups go out: weekdays from 9:00 to 17:00
/// local time, as the Calendar's free time has them.
pub const WORK_START: i8 = 9;
pub const WORK_END: i8 = 17;
/// Subjects of automatic answers (out of office, vacation, bounces) that
/// carry no `Auto-Submitted` header, lower case.
const AUTOMATIC_SUBJECTS: &[&str] = &[
    "automatic reply",
    "auto-reply",
    "auto reply",
    "autoreply",
    "auto:",
    "out of office",
    "out of the office",
    "vacation reply",
    "undeliverable",
    "delivery status notification",
    "mail delivery failed",
    "returned mail",
    "abwesenheitsnotiz",
    "réponse automatique",
    "respuesta automática",
    "risposta automatica",
    "automatisch antwoord",
    "resposta automática",
];

/// Whether a message later in the conversation is an answer: not one of
/// the follow-ups Katna sent (`ours`), and not an automatic one.
pub fn counts_as_reply(later: &LaterMessage, ours: &[String]) -> bool {
    if later
        .message_id
        .as_ref()
        .is_some_and(|id| ours.contains(id))
    {
        return false;
    }
    // Mail with `Auto-Submitted` is sorted into Updates; a person's reply
    // never is.
    if later.category == Some(MailCategory::Updates) {
        return false;
    }
    let subject = later.subject.trim_start().to_lowercase();
    !AUTOMATIC_SUBJECTS.iter().any(|s| subject.starts_with(s))
}

/// The first moment at or after `at` (Unix seconds) in working hours.
pub fn working_time(at: i64, tz: &TimeZone) -> i64 {
    use jiff::civil::Weekday;
    let Ok(stamp) = jiff::Timestamp::from_second(at) else {
        return at;
    };
    let second = |day: jiff::civil::Date, hour: i8| {
        day.at(hour, 0, 0, 0)
            .to_zoned(tz.clone())
            .map(|z| z.timestamp().as_second())
    };
    let mut day = stamp.to_zoned(tz.clone()).date();
    for _ in 0..7 {
        if !matches!(day.weekday(), Weekday::Saturday | Weekday::Sunday) {
            let (Ok(start), Ok(end)) = (second(day, WORK_START), second(day, WORK_END)) else {
                return at;
            };
            if at < start {
                return start;
            }
            if at < end {
                return at;
            }
        }
        let Ok(next) = day.tomorrow() else {
            return at;
        };
        day = next;
    }
    at
}

/// Whether anyone answered the message of `follow_up`: a message later in
/// its conversation that [counts as a reply](counts_as_reply).
pub fn replied(store: &Store, follow_up: &FollowUp) -> katna_store::Result<bool> {
    let account = AccountId(follow_up.account);
    for copy in store.messages_with_header(account, &follow_up.message_id)? {
        if store
            .later_in_thread(copy)?
            .iter()
            .any(|later| counts_as_reply(later, &follow_up.sent))
        {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follow_ups_go_out_in_working_hours() {
        let tz = TimeZone::get("Asia/Kolkata").unwrap();
        let at = |text: &str| {
            text.parse::<jiff::civil::DateTime>()
                .unwrap()
                .to_zoned(tz.clone())
                .unwrap()
                .timestamp()
                .as_second()
        };
        // Wednesday 7 Oct 2026.
        assert_eq!(
            working_time(at("2026-10-07T10:30"), &tz),
            at("2026-10-07T10:30")
        );
        assert_eq!(
            working_time(at("2026-10-07T06:00"), &tz),
            at("2026-10-07T09:00")
        );
        assert_eq!(
            working_time(at("2026-10-07T17:00"), &tz),
            at("2026-10-08T09:00")
        );
        // Friday evening and the weekend wait for Monday.
        assert_eq!(
            working_time(at("2026-10-09T18:00"), &tz),
            at("2026-10-12T09:00")
        );
        assert_eq!(
            working_time(at("2026-10-11T12:00"), &tz),
            at("2026-10-12T09:00")
        );
    }

    #[test]
    fn auto_replies_and_own_follow_ups_are_not_replies() {
        let later = |id: &str, subject: &str, category| LaterMessage {
            message_id: Some(id.to_owned()),
            subject: subject.to_owned(),
            category,
        };
        let ours = ["f1@katna".to_owned()];
        let primary = Some(MailCategory::Primary);
        assert!(counts_as_reply(&later("r@x", "Re: Offer", primary), &ours));
        assert!(counts_as_reply(&later("r@x", "Re: Offer", None), &ours));
        assert!(!counts_as_reply(
            &later("f1@katna", "Re: Offer", primary),
            &ours
        ));
        assert!(!counts_as_reply(
            &later("a@x", "Automatic reply: Offer", primary),
            &ours
        ));
        assert!(!counts_as_reply(
            &later("a@x", "Out of Office: Offer", primary),
            &ours
        ));
        assert!(!counts_as_reply(
            &later("a@x", "Re: Offer", Some(MailCategory::Updates)),
            &ours
        ));
    }
}
