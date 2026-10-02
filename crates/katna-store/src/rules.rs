// SPDX-License-Identifier: GPL-3.0-or-later

//! Mail rules (`mail_rule` in `pim.db`; `docs/ARCHITECTURE.md` §9.4):
//! "when new mail matches these conditions, do these things".
//!
//! This module holds the model, the matcher and the queries. Matching is a
//! pure function of a [`Matcher`] (a rule with its regular expressions
//! compiled once) and a message's [`MailFacts`], so Katna Mail can preview
//! a rule on the read-only store ([`Store::rule_preview`]) exactly as the
//! daemon runs it. The daemon (`katna_sync::rules`) carries out the actions
//! through the same code paths as the user's own changes.

use katna_core::AccountId;
use regex::{Regex, RegexBuilder};
use rusqlite::{OptionalExtension, Row, TransactionBehavior, named_params, params};
use serde::{Deserialize, Serialize};

use crate::Store;
use crate::db::unix_now;
use crate::error::{Error, Result};
use crate::mail::{MessageFlags, MessageId, ParticipantRole};
use crate::mail_read::{StoredMessage, StoredParticipant};

/// The longest rule name, in characters.
pub const MAX_NAME: usize = 200;
/// The most conditions, and the most actions, one rule has.
pub const MAX_PARTS: usize = 50;
/// The longest value a condition compares with, in characters.
pub const MAX_VALUE: usize = 1000;
/// The longest "mark read after" wait, in days.
pub const MAX_READ_AFTER_DAYS: u32 = 3650;
/// How much memory a compiled regular expression may take.
const REGEX_SIZE_LIMIT: usize = 1 << 20;

/// Whether every condition must hold, or one is enough.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchMode {
    #[default]
    All,
    Any,
}

impl MatchMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Any => "any",
        }
    }

    fn parse(text: &str) -> Result<Self> {
        match text {
            "all" => Ok(Self::All),
            "any" => Ok(Self::Any),
            other => Err(Error::InvalidData(format!("rule match mode {other:?}"))),
        }
    }
}

/// What part of a message a condition looks at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    From,
    To,
    Cc,
    /// To, Cc or Bcc.
    AnyRecipient,
    ReplyTo,
    Subject,
    /// The text of the message (HTML as text).
    Body,
    /// The name of any attachment.
    AttachmentName,
    /// Whether it has attachments. The comparator is ignored; the value
    /// `false` (or `no`) means "has none", anything else "has some".
    HasAttachment,
}

impl Field {
    /// The participant roles an address field looks at.
    fn roles(self) -> &'static [ParticipantRole] {
        match self {
            Self::From => &[ParticipantRole::From],
            Self::To => &[ParticipantRole::To],
            Self::Cc => &[ParticipantRole::Cc],
            Self::AnyRecipient => &[
                ParticipantRole::To,
                ParticipantRole::Cc,
                ParticipantRole::Bcc,
            ],
            Self::ReplyTo => &[ParticipantRole::ReplyTo],
            Self::Subject | Self::Body | Self::AttachmentName | Self::HasAttachment => &[],
        }
    }
}

/// How a condition compares. Text compares without regard to case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Comparator {
    Contains,
    NotContains,
    BeginsWith,
    EndsWith,
    Equals,
    /// A regular expression (Rust `regex` syntax) found anywhere.
    Matches,
}

/// One condition: `field` `comparator` `value`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    pub field: Field,
    pub comparator: Comparator,
    #[serde(default)]
    pub value: String,
}

/// What a rule does to the mail it matches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Action {
    /// Moves it to a folder (`folder.id`) of its account.
    Move {
        folder: i64,
    },
    /// Skips the inbox: moves it to the archive.
    Archive,
    /// Moves it to the trash.
    Trash,
    MarkRead,
    Star,
    MarkImportant,
    /// Puts a Gmail label (`folder.id`) on it too, keeping it where it is.
    AddLabel {
        folder: i64,
    },
    /// Forwards it, as an attachment, to an address.
    Forward {
        to: String,
    },
    /// Shows no new-mail notification for it.
    DontNotify,
    /// Marks it read this many days after it arrived.
    MarkReadAfter {
        days: u32,
    },
}

impl Action {
    /// Whether the action takes the message out of the inbox.
    pub fn moves(&self) -> bool {
        matches!(self, Self::Move { .. } | Self::Archive | Self::Trash)
    }

    /// The folder it names, if any.
    pub fn folder(&self) -> Option<i64> {
        match self {
            Self::Move { folder } | Self::AddLabel { folder } => Some(*folder),
            _ => None,
        }
    }
}

/// Where a rule runs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunsOn {
    /// In `katna-daemon`, on this computer.
    #[default]
    Katna,
    /// As a Gmail filter (later).
    Gmail,
    /// As a Sieve script on the server (later).
    Sieve,
}

impl RunsOn {
    fn as_str(self) -> &'static str {
        match self {
            Self::Katna => "katna",
            Self::Gmail => "gmail",
            Self::Sieve => "sieve",
        }
    }

    fn parse(text: &str) -> Result<Self> {
        match text {
            "katna" => Ok(Self::Katna),
            "gmail" => Ok(Self::Gmail),
            "sieve" => Ok(Self::Sieve),
            other => Err(Error::InvalidData(format!("rule runs on {other:?}"))),
        }
    }
}

fn yes() -> bool {
    true
}

/// A mail rule. Its JSON form is what `SaveRule` takes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rule {
    /// 0 for one not saved yet.
    #[serde(default)]
    pub id: i64,
    pub name: String,
    #[serde(default = "yes")]
    pub enabled: bool,
    /// Its place in the list; rules run in this order. Set by the store.
    #[serde(default)]
    pub position: i64,
    #[serde(default)]
    pub match_mode: MatchMode,
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
    /// Later rules don't run on mail this one matched.
    #[serde(default)]
    pub stop: bool,
    /// The accounts (`account.id`) whose mail it looks at.
    pub accounts: Vec<i64>,
    #[serde(default)]
    pub runs_on: RunsOn,
    /// Why it was switched off, when an action failed. Set by the daemon.
    #[serde(default)]
    pub last_error: Option<String>,
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            enabled: true,
            position: 0,
            match_mode: MatchMode::All,
            conditions: Vec::new(),
            actions: Vec::new(),
            stop: false,
            accounts: Vec::new(),
            runs_on: RunsOn::Katna,
            last_error: None,
        }
    }
}

impl Rule {
    /// Whether it looks at mail of `account`.
    pub fn covers(&self, account: AccountId) -> bool {
        self.accounts.contains(&account.0)
    }

    /// Whether a condition looks at the message's text.
    pub fn needs_body(&self) -> bool {
        self.conditions.iter().any(|c| c.field == Field::Body)
    }

    /// Checks what can be checked without the store: a name, at least one
    /// condition, action and account, values that make sense, regular
    /// expressions that compile, and at most one action that moves the
    /// mail. Returns why not, in English, for `InvalidArgs`.
    pub fn validate(&self) -> std::result::Result<(), String> {
        let name = self.name.trim();
        if name.is_empty() {
            return Err("a rule needs a name".into());
        }
        if name.chars().count() > MAX_NAME {
            return Err(format!("a rule name is at most {MAX_NAME} characters"));
        }
        if self.conditions.is_empty() {
            return Err("a rule needs at least one condition".into());
        }
        if self.actions.is_empty() {
            return Err("a rule needs at least one action".into());
        }
        if self.conditions.len() > MAX_PARTS || self.actions.len() > MAX_PARTS {
            return Err(format!(
                "a rule has at most {MAX_PARTS} conditions and {MAX_PARTS} actions"
            ));
        }
        if self.accounts.is_empty() {
            return Err("a rule needs at least one account".into());
        }
        for condition in &self.conditions {
            if condition.value.chars().count() > MAX_VALUE {
                return Err(format!(
                    "a condition's value is at most {MAX_VALUE} characters"
                ));
            }
            if condition.field != Field::HasAttachment && condition.value.trim().is_empty() {
                return Err("a condition needs a value".into());
            }
            Test::new(condition)?;
        }
        if self.actions.iter().filter(|a| a.moves()).count() > 1 {
            return Err("a rule moves mail to one place at most".into());
        }
        for action in &self.actions {
            match action {
                Action::Forward { to } => {
                    let to = to.trim();
                    let looks_right = to.split_once('@').is_some_and(|(local, domain)| {
                        !local.is_empty() && domain.contains('.') && !domain.ends_with('.')
                    });
                    if !looks_right || to.chars().any(|c| c.is_whitespace() || c == ',') {
                        return Err(format!("{to:?} is not an email address"));
                    }
                }
                Action::MarkReadAfter { days } if *days == 0 || *days > MAX_READ_AFTER_DAYS => {
                    return Err(format!("mark read after 1 to {MAX_READ_AFTER_DAYS} days"));
                }
                _ => {}
            }
        }
        Ok(())
    }
}

/// What a rule looks at in one message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MailFacts {
    pub account: AccountId,
    pub subject: String,
    pub participants: Vec<StoredParticipant>,
    pub attachment_names: Vec<String>,
    pub has_attachments: bool,
    /// The text of the message; its snippet while the body is not stored.
    pub body: String,
}

impl MailFacts {
    /// The facts of `message`: `attachment_names` from its structure or
    /// its body, and `body` its text, else its snippet.
    pub fn of(
        message: &StoredMessage,
        attachment_names: Vec<String>,
        body: Option<String>,
    ) -> Self {
        Self {
            account: message.account,
            subject: message.subject.clone(),
            participants: message.participants.clone(),
            has_attachments: message.has_attachments || !attachment_names.is_empty(),
            attachment_names,
            body: body.or_else(|| message.snippet.clone()).unwrap_or_default(),
        }
    }
}

/// One condition, ready to test.
#[derive(Debug, Clone)]
enum Test {
    /// Lowercase text to compare with.
    Text(Comparator, String),
    Regex(Regex),
    Has(bool),
}

impl Test {
    fn new(condition: &Condition) -> std::result::Result<Self, String> {
        if condition.field == Field::HasAttachment {
            let value = condition.value.trim().to_lowercase();
            return Ok(Self::Has(!matches!(value.as_str(), "false" | "no" | "0")));
        }
        if condition.comparator == Comparator::Matches {
            return RegexBuilder::new(&condition.value)
                .case_insensitive(true)
                .size_limit(REGEX_SIZE_LIMIT)
                .build()
                .map(Self::Regex)
                .map_err(|err| {
                    format!("{:?} is not a regular expression: {err}", condition.value)
                });
        }
        Ok(Self::Text(
            condition.comparator,
            condition.value.trim().to_lowercase(),
        ))
    }

    /// Whether `text` matches, as a positive test: "doesn't contain" tests
    /// "contains" here and is negated by the caller.
    fn hits(&self, text: &str) -> bool {
        match self {
            Self::Text(comparator, value) => {
                let text = text.to_lowercase();
                match comparator {
                    Comparator::Contains | Comparator::NotContains => text.contains(value),
                    Comparator::BeginsWith => text.trim_start().starts_with(value.as_str()),
                    Comparator::EndsWith => text.trim_end().ends_with(value.as_str()),
                    Comparator::Equals => text.trim() == value,
                    Comparator::Matches => false,
                }
            }
            Self::Regex(regex) => regex.is_match(text),
            Self::Has(_) => false,
        }
    }

    fn negated(&self) -> bool {
        matches!(self, Self::Text(Comparator::NotContains, _))
    }
}

/// A rule with its conditions compiled, ready to match many messages.
#[derive(Debug, Clone)]
pub struct Matcher {
    rule: Rule,
    tests: Vec<(Field, Test)>,
}

impl Matcher {
    /// Compiles `rule`. Fails, saying why, on a bad regular expression.
    pub fn new(rule: Rule) -> std::result::Result<Self, String> {
        let tests = rule
            .conditions
            .iter()
            .map(|c| Test::new(c).map(|test| (c.field, test)))
            .collect::<std::result::Result<_, _>>()?;
        Ok(Self { rule, tests })
    }

    pub fn rule(&self) -> &Rule {
        &self.rule
    }

    /// Whether `mail` matches the conditions (not whether the rule is on,
    /// or covers the mail's account: see [`matching`]).
    pub fn matches(&self, mail: &MailFacts) -> bool {
        let mut results = self
            .tests
            .iter()
            .map(|(field, test)| holds(*field, test, mail));
        match self.rule.match_mode {
            MatchMode::All => results.all(|hit| hit),
            MatchMode::Any => results.any(|hit| hit),
        }
    }
}

/// Whether one condition holds for `mail`.
fn holds(field: Field, test: &Test, mail: &MailFacts) -> bool {
    if let Test::Has(want) = test {
        return mail.has_attachments == *want;
    }
    let hit = match field {
        Field::Subject => test.hits(&mail.subject),
        Field::Body => test.hits(&mail.body),
        Field::AttachmentName => mail.attachment_names.iter().any(|n| test.hits(n)),
        Field::HasAttachment => false,
        Field::From | Field::To | Field::Cc | Field::AnyRecipient | Field::ReplyTo => {
            let roles = field.roles();
            mail.participants
                .iter()
                .filter(|p| roles.contains(&p.role))
                .any(|p| {
                    test.hits(&p.email_norm)
                        || p.display_name
                            .as_deref()
                            .is_some_and(|name| test.hits(name))
                })
        }
    };
    hit != test.negated()
}

/// The rules among `matchers` that act on `mail`, in list order: those
/// switched on, covering its account and matching it, up to and including
/// the first that matched with "stop".
pub fn matching<'a>(matchers: &'a [Matcher], mail: &MailFacts) -> Vec<&'a Matcher> {
    let mut out = Vec::new();
    for matcher in matchers {
        let rule = &matcher.rule;
        if !rule.enabled || !rule.covers(mail.account) || !matcher.matches(mail) {
            continue;
        }
        out.push(matcher);
        if rule.stop {
            break;
        }
    }
    out
}

const RULE_COLUMNS: &str = "id, name, enabled, position, match_mode, conditions_json,
                            actions_json, stop, accounts_json, runs_on, last_error";

fn rule_row(row: &Row<'_>) -> rusqlite::Result<Result<Rule>> {
    let id: i64 = row.get(0)?;
    let match_mode: String = row.get(4)?;
    let conditions: String = row.get(5)?;
    let actions: String = row.get(6)?;
    let accounts: String = row.get(8)?;
    let runs_on: String = row.get(9)?;
    let json = |what: &str, err: serde_json::Error| {
        Error::InvalidData(format!("rule {id}: {what}: {err}"))
    };
    let name = row.get(1)?;
    let enabled = row.get(2)?;
    let position = row.get(3)?;
    let stop = row.get(7)?;
    let last_error = row.get(10)?;
    Ok((|| {
        Ok(Rule {
            id,
            name,
            enabled,
            position,
            match_mode: MatchMode::parse(&match_mode)?,
            conditions: serde_json::from_str(&conditions).map_err(|e| json("conditions", e))?,
            actions: serde_json::from_str(&actions).map_err(|e| json("actions", e))?,
            stop,
            accounts: serde_json::from_str(&accounts).map_err(|e| json("accounts", e))?,
            runs_on: RunsOn::parse(&runs_on)?,
            last_error,
        })
    })())
}

fn to_json(value: &impl Serialize) -> String {
    serde_json::to_string(value).expect("rule parts serialize")
}

impl Store {
    /// Every rule, in the order they run.
    pub fn rules(&self) -> Result<Vec<Rule>> {
        let mut stmt = self.pim.prepare_cached(&format!(
            "SELECT {RULE_COLUMNS} FROM mail_rule ORDER BY position, id"
        ))?;
        stmt.query_map([], rule_row)?.map(|row| row?).collect()
    }

    /// Rule `id`, if it exists.
    pub fn rule(&self, id: i64) -> Result<Option<Rule>> {
        self.pim
            .prepare_cached(&format!(
                "SELECT {RULE_COLUMNS} FROM mail_rule WHERE id = ?1"
            ))?
            .query_row([id], rule_row)
            .optional()?
            .transpose()
    }

    /// Saves `rule`: a new one at the end of the list when its ID is 0 (or
    /// no longer exists), else in place of the old one, keeping its place.
    /// Saving clears `last_error`. Returns its ID. Does not validate: the
    /// daemon does ([`Rule::validate`]).
    pub fn save_rule(&mut self, rule: &Rule) -> Result<i64> {
        self.check_writable()?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let now = unix_now();
        let updated = rule.id != 0
            && tx.execute(
                "UPDATE mail_rule SET name = :name, enabled = :enabled,
                        match_mode = :match_mode, conditions_json = :conditions,
                        actions_json = :actions, stop = :stop, accounts_json = :accounts,
                        runs_on = :runs_on, last_error = NULL, updated_at = :now
                 WHERE id = :id",
                named_params! {
                    ":id": rule.id,
                    ":name": rule.name.trim(),
                    ":enabled": rule.enabled,
                    ":match_mode": rule.match_mode.as_str(),
                    ":conditions": to_json(&rule.conditions),
                    ":actions": to_json(&rule.actions),
                    ":stop": rule.stop,
                    ":accounts": to_json(&rule.accounts),
                    ":runs_on": rule.runs_on.as_str(),
                    ":now": now,
                },
            )? > 0;
        let id = if updated {
            rule.id
        } else {
            tx.execute(
                "INSERT INTO mail_rule (name, enabled, position, match_mode, conditions_json,
                                        actions_json, stop, accounts_json, runs_on,
                                        created_at, updated_at)
                 VALUES (:name, :enabled,
                         (SELECT coalesce(max(position) + 1, 0) FROM mail_rule),
                         :match_mode, :conditions, :actions, :stop, :accounts, :runs_on,
                         :now, :now)",
                named_params! {
                    ":name": rule.name.trim(),
                    ":enabled": rule.enabled,
                    ":match_mode": rule.match_mode.as_str(),
                    ":conditions": to_json(&rule.conditions),
                    ":actions": to_json(&rule.actions),
                    ":stop": rule.stop,
                    ":accounts": to_json(&rule.accounts),
                    ":runs_on": rule.runs_on.as_str(),
                    ":now": now,
                },
            )?;
            tx.last_insert_rowid()
        };
        tx.commit()?;
        Ok(id)
    }

    /// Deletes rule `id`. Returns whether it existed.
    pub fn delete_rule(&mut self, id: i64) -> Result<bool> {
        self.check_writable()?;
        Ok(self
            .pim
            .execute("DELETE FROM mail_rule WHERE id = ?1", [id])?
            > 0)
    }

    /// Puts the rules `ids` first, in this order; the others follow in
    /// the order they had. Unknown IDs are skipped.
    pub fn reorder_rules(&mut self, ids: &[i64]) -> Result<()> {
        self.check_writable()?;
        let tx = self
            .pim
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let old: Vec<i64> = tx
            .prepare("SELECT id FROM mail_rule ORDER BY position, id")?
            .query_map([], |row| row.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        let mut order: Vec<i64> = Vec::with_capacity(old.len());
        for id in ids {
            if old.contains(id) && !order.contains(id) {
                order.push(*id);
            }
        }
        for id in old {
            if !order.contains(&id) {
                order.push(id);
            }
        }
        {
            let mut stmt =
                tx.prepare("UPDATE mail_rule SET position = ?2, updated_at = ?3 WHERE id = ?1")?;
            let now = unix_now();
            for (position, id) in order.iter().enumerate() {
                stmt.execute(params![id, position as i64, now])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    /// Switches rule `id` on or off; switching it on clears its
    /// `last_error`. Returns whether it exists.
    pub fn set_rule_enabled(&mut self, id: i64, on: bool) -> Result<bool> {
        self.check_writable()?;
        Ok(self.pim.execute(
            "UPDATE mail_rule SET enabled = ?2,
                    last_error = CASE WHEN ?2 THEN NULL ELSE last_error END,
                    updated_at = ?3
             WHERE id = ?1",
            params![id, on, unix_now()],
        )? > 0)
    }

    /// Switches rule `id` off because an action failed, saying why.
    /// Returns whether it exists.
    pub fn fail_rule(&mut self, id: i64, error: &str) -> Result<bool> {
        self.check_writable()?;
        Ok(self.pim.execute(
            "UPDATE mail_rule SET enabled = 0, last_error = ?2, updated_at = ?3 WHERE id = ?1",
            params![id, error, unix_now()],
        )? > 0)
    }

    /// Mail of `account` that rules look at: in its inbox, newer than
    /// message `after`, dated `since` or later (or undated), not a draft,
    /// not deleted and not from the account's own address; oldest first.
    pub fn rule_candidates(
        &self,
        account: AccountId,
        after: MessageId,
        since: i64,
    ) -> Result<Vec<StoredMessage>> {
        let own: Option<String> = self
            .pim
            .prepare_cached("SELECT address FROM account WHERE id = ?1")?
            .query_row([account.0], |row| row.get(0))
            .optional()?;
        let own = own.map(|a| a.trim().to_lowercase()).unwrap_or_default();
        let unwanted = (MessageFlags::DRAFT | MessageFlags::DELETED).bits();
        let ids: Vec<MessageId> = self
            .mail
            .prepare_cached(
                "SELECT m.id FROM message m
                 WHERE m.account_id = :account AND m.id > :after
                   AND (m.date IS NULL OR m.date >= :since) AND (m.flags & :unwanted) = 0
                   AND EXISTS (SELECT 1 FROM message_location l
                               JOIN folder f ON f.id = l.folder_id
                               WHERE l.message_id = m.id AND f.role = 'inbox')
                 ORDER BY m.id",
            )?
            .query_map(
                named_params! {
                    ":account": account.0,
                    ":after": after.0,
                    ":since": since,
                    ":unwanted": unwanted,
                },
                |row| row.get(0).map(MessageId),
            )?
            .collect::<rusqlite::Result<_>>()?;
        let mut messages = crate::mail_read::messages_by_id(&self.mail, &ids)?;
        messages.retain(|m| {
            own.is_empty()
                || !m
                    .participants
                    .iter()
                    .any(|p| p.role == ParticipantRole::From && p.email_norm == own)
        });
        Ok(messages)
    }

    /// The names of the attachments of `message` its structure names.
    pub fn attachment_names(&self, message: MessageId) -> Result<Vec<String>> {
        Ok(self
            .attachments(message)?
            .into_iter()
            .filter_map(|a| a.filename)
            .filter(|name| !name.is_empty())
            .collect())
    }

    /// How many inbox messages of the rule's accounts from the last `days`
    /// days (before `now`) the rule matches, whether it is on or not: for
    /// "Matches 34 mails from the last 30 days". `body` gives a message's
    /// text when a condition needs it (for example
    /// `katna_search::message_text` of its stored body); without it, the
    /// snippet stands in.
    pub fn rule_preview(
        &self,
        rule: &Rule,
        days: u32,
        now: i64,
        mut body: impl FnMut(&StoredMessage) -> Option<String>,
    ) -> Result<u32> {
        let Ok(matcher) = Matcher::new(Rule {
            enabled: true,
            stop: false,
            ..rule.clone()
        }) else {
            return Ok(0);
        };
        let since = now.saturating_sub(i64::from(days) * 86_400);
        let needs_body = rule.needs_body();
        let mut count = 0;
        for &account in &rule.accounts {
            for message in self.rule_candidates(AccountId(account), MessageId(0), since)? {
                let attachments = self.attachment_names(message.id)?;
                let text = if needs_body { body(&message) } else { None };
                let facts = MailFacts::of(&message, attachments, text);
                if !matching(std::slice::from_ref(&matcher), &facts).is_empty() {
                    count += 1;
                }
            }
        }
        Ok(count)
    }
}

#[cfg(test)]
mod tests;
