// SPDX-License-Identifier: GPL-3.0-or-later

//! Starter rules: a few useful rules Settings > Folders & rules offers
//! under the user's own, switched off (`docs/ARCHITECTURE.md` §9.4).
//! Turning one on makes the folders it files into (labels, on Gmail) in
//! every mail account that lacks them, then saves it as an ordinary rule
//! for all those accounts, carrying the starter's key so the list stops
//! offering it. Its pencil opens the editor on it instead, filled in with
//! the folders that already exist.

use katna_core::MailCategory;
use katna_i18n::tr;
use katna_store::rules::{Action, Comparator, Condition, Field, MatchMode, Rule};

/// A folder a starter rule files mail into, made when it is turned on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Folder {
    Reading,
    Receipts,
    Deliveries,
    Travel,
    Social,
}

impl Folder {
    /// Its name, as it is made.
    pub(super) fn name(self) -> String {
        match self {
            Self::Reading => tr!("rules-starter-folder-reading"),
            Self::Receipts => tr!("rules-starter-folder-receipts"),
            Self::Deliveries => tr!("rules-starter-folder-deliveries"),
            Self::Travel => tr!("rules-starter-folder-travel"),
            Self::Social => tr!("rules-starter-folder-social"),
        }
    }

    /// The stand-in folder ID of [`preview`]'s rule.
    fn stand_in(self) -> i64 {
        -1 - self as i64
    }
}

/// What a starter rule does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Step {
    Do(Action),
    /// Moves it to the folder, out of the inbox.
    Move(Folder),
    /// Labels it (on Gmail; elsewhere a copy in the folder).
    Label(Folder),
}

#[derive(Debug, Clone)]
pub(super) struct Starter {
    pub key: &'static str,
    pub match_mode: MatchMode,
    pub conditions: Vec<Condition>,
    pub steps: Vec<Step>,
}

impl Starter {
    pub(super) fn name(&self) -> String {
        match self.key {
            "promotions" => tr!("rules-starter-promotions"),
            "newsletters" => tr!("rules-starter-newsletters"),
            "receipts" => tr!("rules-starter-receipts"),
            "deliveries" => tr!("rules-starter-deliveries"),
            "train" => tr!("rules-starter-train"),
            "flight" => tr!("rules-starter-flight"),
            "codes" => tr!("rules-starter-codes"),
            "security" => tr!("rules-starter-security"),
            "social" => tr!("rules-starter-social"),
            _ => tr!("rules-starter-invites"),
        }
    }

    /// The folders it files into.
    pub(super) fn folders(&self) -> Vec<Folder> {
        let mut out = Vec::new();
        for step in &self.steps {
            if let Step::Move(folder) | Step::Label(folder) = step
                && !out.contains(folder)
            {
                out.push(*folder);
            }
        }
        out
    }

    /// It as a rule for `accounts`, with `folder` the ID of a folder in
    /// an account, if there is one: each folder step names the folder in
    /// each account that has it.
    pub(super) fn rule(
        &self,
        accounts: &[i64],
        folder: impl Fn(i64, Folder) -> Option<i64>,
    ) -> Rule {
        let mut actions = Vec::new();
        for step in &self.steps {
            match step {
                Step::Do(action) => actions.push(action.clone()),
                Step::Move(f) | Step::Label(f) => {
                    for &account in accounts {
                        if let Some(id) = folder(account, *f) {
                            actions.push(match step {
                                Step::Move(_) => Action::Move { folder: id },
                                _ => Action::AddLabel { folder: id },
                            });
                        }
                    }
                }
            }
        }
        Rule {
            name: self.name(),
            enabled: true,
            match_mode: self.match_mode,
            conditions: self.conditions.clone(),
            actions,
            accounts: accounts.to_vec(),
            starter: Some(self.key.to_owned()),
            ..Rule::default()
        }
    }

    /// It as a rule for its row's line: its folders have stand-in IDs
    /// that [`preview_folder`] names.
    pub(super) fn preview(&self) -> Rule {
        self.rule(&[0], |_, folder| Some(folder.stand_in()))
    }
}

/// The name of a stand-in folder of [`Starter::preview`].
pub(super) fn preview_folder(id: i64) -> Option<String> {
    [
        Folder::Reading,
        Folder::Receipts,
        Folder::Deliveries,
        Folder::Travel,
        Folder::Social,
    ]
    .into_iter()
    .find(|f| f.stand_in() == id)
    .map(Folder::name)
}

fn when(field: Field, comparator: Comparator, value: &str) -> Condition {
    Condition {
        field,
        comparator,
        value: value.to_owned(),
    }
}

fn subject_has(words: &[&str]) -> Vec<Condition> {
    words
        .iter()
        .map(|w| when(Field::Subject, Comparator::Contains, w))
        .collect()
}

/// Train booking sites and apps, mostly Indian Railways'.
const TRAIN_SENDERS: &str = "irctc.co.in|confirmtkt.com|railyatri.in|trainman.in";
/// Airlines flying in India, and the travel sites people book them on.
const FLIGHT_SENDERS: &str = "airindia.com|airindia.in|airindiaexpress.com|goindigo.in|\
                              spicejet.com|akasaair.com|allianceair.in|makemytrip.com|\
                              cleartrip.com|ixigo.com|goibibo.com|yatra.com|easemytrip.com";
/// Words in the subject of a ticket, not of an offer.
const TICKET_WORDS: &str =
    "ticket|pnr|booking|itinerary|boarding pass|check-in|reservation|cancellation|refund";

/// Every starter rule, in the order the list offers them.
pub(super) fn starters() -> Vec<Starter> {
    use Comparator::*;
    let all = MatchMode::All;
    let any = MatchMode::Any;
    vec![
        Starter {
            key: "promotions",
            match_mode: all,
            conditions: vec![when(Field::Tab, Equals, MailCategory::Promotions.as_str())],
            steps: vec![Step::Do(Action::DontNotify)],
        },
        Starter {
            key: "newsletters",
            match_mode: all,
            conditions: vec![when(Field::MailingList, Equals, "true")],
            steps: vec![Step::Move(Folder::Reading)],
        },
        Starter {
            key: "receipts",
            match_mode: any,
            conditions: subject_has(&["receipt", "invoice", "order", "payment"]),
            steps: vec![Step::Label(Folder::Receipts)],
        },
        Starter {
            key: "deliveries",
            match_mode: any,
            conditions: subject_has(&["shipped", "out for delivery", "delivered"]),
            steps: vec![Step::Label(Folder::Deliveries)],
        },
        Starter {
            key: "train",
            match_mode: all,
            conditions: vec![
                when(Field::From, Matches, TRAIN_SENDERS),
                when(Field::Subject, Matches, TICKET_WORDS),
            ],
            steps: vec![Step::Label(Folder::Travel), Step::Do(Action::MarkImportant)],
        },
        Starter {
            key: "flight",
            match_mode: all,
            conditions: vec![
                when(Field::From, Matches, FLIGHT_SENDERS),
                when(Field::Subject, Matches, TICKET_WORDS),
            ],
            steps: vec![Step::Label(Folder::Travel), Step::Do(Action::MarkImportant)],
        },
        Starter {
            key: "codes",
            match_mode: any,
            conditions: subject_has(&["OTP", "verification code", "one-time password"]),
            steps: vec![Step::Do(Action::MarkReadAfter { days: 1 })],
        },
        Starter {
            key: "security",
            match_mode: any,
            conditions: subject_has(&["security alert", "new sign-in", "password changed"]),
            steps: vec![Step::Do(Action::Star), Step::Do(Action::MarkImportant)],
        },
        Starter {
            key: "social",
            match_mode: all,
            conditions: vec![when(Field::Tab, Equals, MailCategory::Social.as_str())],
            steps: vec![Step::Move(Folder::Social)],
        },
        Starter {
            key: "invites",
            match_mode: all,
            conditions: vec![when(Field::AttachmentName, EndsWith, ".ics")],
            steps: vec![Step::Do(Action::MarkImportant)],
        },
    ]
}

/// The starter rules not made into one of `rules` yet.
pub(super) fn offered(rules: &[Rule]) -> Vec<Starter> {
    starters()
        .into_iter()
        .filter(|s| !rules.iter().any(|r| r.starter.as_deref() == Some(s.key)))
        .collect()
}

#[cfg(test)]
mod tests {
    use katna_store::rules::{MailFacts, Matcher};
    use katna_store::{ParticipantRole, StoredParticipant};

    use super::*;

    fn mail(from: &str, subject: &str) -> MailFacts {
        MailFacts {
            account: katna_core::AccountId(1),
            subject: subject.to_owned(),
            participants: vec![StoredParticipant {
                role: ParticipantRole::From,
                email_norm: from.to_owned(),
                domain: from.rsplit('@').next().unwrap_or_default().to_owned(),
                display_name: None,
            }],
            attachment_names: Vec::new(),
            has_attachments: false,
            body: String::new(),
            category: MailCategory::Updates,
            mailing_list: false,
        }
    }

    fn starter(key: &str) -> Matcher {
        let found = starters().into_iter().find(|s| s.key == key).unwrap();
        Matcher::new(found.rule(&[1], |_, _| Some(7))).unwrap()
    }

    #[test]
    fn every_starter_is_a_valid_rule_and_has_a_name() {
        let all = starters();
        for s in &all {
            let rule = s.rule(&[1, 2], |account, _| Some(account * 10));
            assert_eq!(rule.validate(), Ok(()), "{}", s.key);
            assert_eq!(rule.starter.as_deref(), Some(s.key));
        }
        let mut keys: Vec<&str> = all.iter().map(|s| s.key).collect();
        keys.dedup();
        assert_eq!(keys.len(), all.len());
    }

    #[test]
    fn tickets_but_not_offers() {
        let train = starter("train");
        assert!(train.matches(&mail(
            "ticketadmin@irctc.co.in",
            "Booking Confirmation on IRCTC, Train: 12951, 01-Oct-2026, 3A, NDLS - MMCT"
        )));
        assert!(!train.matches(&mail(
            "noreply@irctc.co.in",
            "Explore Kashmir with IRCTC Tourism"
        )));
        let flight = starter("flight");
        assert!(flight.matches(&mail(
            "reservations@customer.goindigo.in",
            "IndiGo Booking Confirmation - PNR ABC123"
        )));
        assert!(flight.matches(&mail(
            "noreply@airindia.com",
            "Your boarding pass for AI 887"
        )));
        assert!(!flight.matches(&mail("offers@goindigo.in", "Fly to Goa from ₹1,999")));
        assert!(!flight.matches(&mail("bob@example.org", "My ticket to Goa")));
    }

    #[test]
    fn folders_go_to_each_account_that_has_them() {
        let newsletters = starters()
            .into_iter()
            .find(|s| s.key == "newsletters")
            .unwrap();
        assert_eq!(newsletters.folders(), [Folder::Reading]);
        let rule = newsletters.rule(&[1, 2], |account, _| (account == 2).then_some(5));
        assert_eq!(rule.actions, [Action::Move { folder: 5 }]);
        let shown = newsletters.preview();
        assert_eq!(
            preview_folder(shown.actions[0].folder().unwrap()).as_deref(),
            Some("Reading")
        );
    }

    #[test]
    fn a_starter_made_into_a_rule_is_not_offered_again() {
        let made = Rule {
            starter: Some("receipts".into()),
            ..Rule::default()
        };
        let keys: Vec<&str> = offered(&[made]).iter().map(|s| s.key).collect();
        assert!(!keys.contains(&"receipts"));
        assert_eq!(keys.len(), starters().len() - 1);
    }
}
