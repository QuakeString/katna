// SPDX-License-Identifier: GPL-3.0-or-later

//! Mail rules as Gmail filters (`docs/ARCHITECTURE.md` §9.4), through the
//! Gmail API's `users.settings.filters`, scope [`GOOGLE_GMAIL_SETTINGS`].
//! Accounts signed in before Katna asked for it keep their rules in
//! Katna until they sign in again.
//!
//! A filter's criteria are a Gmail search (`query`). Gmail matches words
//! where Katna matches text, so only what means the same goes there:
//! an address field holding an address or a domain (`from:(bob@x.org)`),
//! a subject holding a phrase (`subject:"the words"`), "doesn't contain"
//! as `-from:(…)`, and attachments (`has:attachment`). Gmail has no "any
//! of": such a rule becomes one filter per condition. Its actions are
//! labels put on and taken off (`INBOX` off skips the inbox; `TRASH`,
//! `STARRED`, `IMPORTANT`, `UNREAD` off marks read, the user's labels)
//! and forwarding to an address Gmail verified. Gmail runs every filter
//! that matches, so a rule with "stop" and later rules stays in Katna.
//!
//! Katna remembers the filters it made, by rule (`mail_rule_remote`);
//! changing a rule deletes them and makes new ones (Gmail can't change a
//! filter), and filters Katna didn't make are never touched.

use std::{collections::HashMap, sync::Arc, time::Duration};

use katna_core::AccountId;
use katna_store::{
    remote::{FolderRole, StoredFolder},
    rules::{Action, Comparator, Condition, Field, MatchMode, RemoteRule, Rule, RunsNote, RunsOn},
};
use serde::{Deserialize, Serialize};

use crate::{
    Error, Result,
    autoconfig::http::{self, Reply},
    google_api,
    net::Tls,
    oauth::{GOOGLE_GMAIL_SETTINGS, TokenSource},
    rules_remote::{self, Verdict},
};

/// The Gmail API's host.
pub const GMAIL_API: &str = "https://gmail.googleapis.com";

const TIMEOUT: Duration = Duration::from_secs(60);

/// A filter as the Gmail API takes it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Filter {
    pub criteria: Criteria,
    pub action: FilterAction,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Criteria {
    pub query: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterAction {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_label_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_label_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forward: Option<String>,
}

/// What the filters of an account can use.
#[derive(Debug, Clone, Default)]
pub struct GmailAccount {
    pub folders: Vec<StoredFolder>,
    /// The user's labels: name → ID.
    pub labels: HashMap<String, String>,
    /// Forwarding addresses Gmail verified, lowercase.
    pub forwarding: Vec<String>,
}

/// `rule` as Gmail filters, or why Gmail can't run it as Katna does.
/// `later`: a later rule of the account follows it.
pub fn translate(rule: &Rule, gmail: &GmailAccount, later: bool) -> Result<Vec<Filter>, RunsNote> {
    if rule.stop && later {
        return Err(RunsNote::Stop {
            service: RunsOn::Gmail,
        });
    }
    let terms = rule
        .conditions
        .iter()
        .map(term)
        .collect::<Result<Vec<_>, _>>()?;
    let action = filter_action(rule, gmail)?;
    let queries = match rule.match_mode {
        MatchMode::All => vec![terms.join(" ")],
        MatchMode::Any => terms,
    };
    Ok(queries
        .into_iter()
        .map(|query| Filter {
            criteria: Criteria { query },
            action: action.clone(),
        })
        .collect())
}

/// Whether `value` is an address or a domain, which Gmail matches as
/// Katna does.
fn address_like(value: &str) -> bool {
    let plain = value
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || ".@_+-".contains(c));
    let domain = value.trim_start_matches('@');
    plain
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
        && value.matches('@').count() <= 1
        && !value.ends_with('@')
}

/// One condition as a Gmail search term.
fn term(condition: &Condition) -> Result<String, RunsNote> {
    let cant = || RunsNote::Condition {
        service: RunsOn::Gmail,
        field: condition.field,
        comparator: condition.comparator,
    };
    let value = condition.value.trim();
    if condition.field == Field::HasAttachment {
        let has = !matches!(value.to_lowercase().as_str(), "false" | "no" | "0");
        return Ok(if has {
            "has:attachment"
        } else {
            "-has:attachment"
        }
        .into());
    }
    let ops: &[&str] = match condition.field {
        Field::From => &["from"],
        Field::To => &["to"],
        Field::Cc => &["cc"],
        Field::AnyRecipient => &["to", "cc", "bcc"],
        Field::Subject => &["subject"],
        Field::ReplyTo | Field::Body | Field::AttachmentName | Field::HasAttachment => {
            return Err(cant());
        }
    };
    let positive = if condition.field == Field::Subject {
        // A phrase of words: Gmail finds the words, in that order.
        let words = value
            .chars()
            .all(|c| c.is_alphanumeric() || c == ' ' || c == '\'' || c == '-');
        if !matches!(
            condition.comparator,
            Comparator::Contains | Comparator::NotContains
        ) || !words
            || value.is_empty()
        {
            return Err(cant());
        }
        format!("subject:\"{value}\"")
    } else {
        let fits = match condition.comparator {
            Comparator::Contains | Comparator::NotContains => address_like(value),
            // The whole address.
            Comparator::Equals => {
                address_like(value) && value.contains('@') && !value.starts_with('@')
            }
            // A domain: everyone at it.
            Comparator::EndsWith => {
                address_like(value) && (value.starts_with('@') || !value.contains('@'))
            }
            Comparator::BeginsWith | Comparator::Matches => false,
        };
        if !fits {
            return Err(cant());
        }
        let value = value.trim_start_matches('@');
        let each: Vec<String> = ops.iter().map(|op| format!("{op}:({value})")).collect();
        if each.len() == 1 {
            each.into_iter().next().unwrap_or_default()
        } else {
            format!("{{{}}}", each.join(" "))
        }
    };
    Ok(if condition.comparator == Comparator::NotContains {
        format!("-{positive}")
    } else {
        positive
    })
}

/// The rule's actions as one filter action.
fn filter_action(rule: &Rule, gmail: &GmailAccount) -> Result<FilterAction, RunsNote> {
    let mut out = FilterAction::default();
    let folder_gone = || RunsNote::Folder {
        service: RunsOn::Gmail,
    };
    let add = |out: &mut FilterAction, id: &str| {
        if !out.add_label_ids.iter().any(|l| l == id) {
            out.add_label_ids.push(id.to_owned());
        }
    };
    let remove = |out: &mut FilterAction, id: &str| {
        if !out.remove_label_ids.iter().any(|l| l == id) {
            out.remove_label_ids.push(id.to_owned());
        }
    };
    for action in &rule.actions {
        let cant = || RunsNote::Action {
            service: RunsOn::Gmail,
            action: action.clone(),
        };
        match action {
            Action::Archive => remove(&mut out, "INBOX"),
            Action::Trash => add(&mut out, "TRASH"),
            Action::MarkRead => remove(&mut out, "UNREAD"),
            Action::Star => add(&mut out, "STARRED"),
            Action::MarkImportant => add(&mut out, "IMPORTANT"),
            Action::Move { folder } => {
                let folder = gmail
                    .folders
                    .iter()
                    .find(|f| f.id.0 == *folder)
                    .ok_or_else(folder_gone)?;
                match folder.role {
                    Some(FolderRole::Trash) => add(&mut out, "TRASH"),
                    Some(FolderRole::Junk) => add(&mut out, "SPAM"),
                    Some(FolderRole::All | FolderRole::Archive) => remove(&mut out, "INBOX"),
                    Some(_) => return Err(cant()),
                    None => {
                        let id = gmail.labels.get(&folder.path).ok_or_else(folder_gone)?;
                        add(&mut out, id);
                        remove(&mut out, "INBOX");
                    }
                }
            }
            Action::AddLabel { folder } => {
                let folder = gmail
                    .folders
                    .iter()
                    .find(|f| f.id.0 == *folder)
                    .ok_or_else(folder_gone)?;
                let id = gmail.labels.get(&folder.path).ok_or_else(folder_gone)?;
                add(&mut out, id);
            }
            Action::Forward { to } => {
                let to = to.trim();
                if out.forward.is_some() {
                    return Err(cant());
                }
                if !gmail.forwarding.iter().any(|a| a.eq_ignore_ascii_case(to)) {
                    return Err(RunsNote::ForwardAddress { to: to.to_owned() });
                }
                out.forward = Some(to.to_owned());
            }
            Action::DontNotify | Action::MarkReadAfter { .. } => return Err(cant()),
        }
    }
    Ok(out)
}

/// One Google account's Gmail settings.
#[derive(Clone)]
pub struct GmailSettings {
    tokens: Arc<TokenSource>,
    tls: Tls,
    /// [`GMAIL_API`], or a server under test.
    api: String,
}

#[derive(Deserialize)]
struct Labels {
    #[serde(default)]
    labels: Vec<Label>,
}

#[derive(Deserialize)]
struct Label {
    id: String,
    #[serde(default)]
    name: String,
    #[serde(rename = "type", default)]
    kind: String,
}

#[derive(Deserialize)]
struct Forwarding {
    #[serde(rename = "forwardingAddresses", default)]
    addresses: Vec<ForwardingAddress>,
}

#[derive(Deserialize)]
struct ForwardingAddress {
    #[serde(rename = "forwardingEmail", default)]
    email: String,
    #[serde(rename = "verificationStatus", default)]
    status: String,
}

#[derive(Deserialize)]
struct Made {
    id: String,
}

impl GmailSettings {
    /// Google's, or the server under test in `KATNA_GOOGLE_API_URL`.
    pub fn new(tokens: Arc<TokenSource>, tls: Tls) -> Self {
        let api = http::test_url("KATNA_GOOGLE_API_URL");
        Self::with_api(tokens, tls, api.as_deref().unwrap_or(GMAIL_API))
    }

    /// Talks to `api` instead of Google, for tests.
    pub fn with_api(tokens: Arc<TokenSource>, tls: Tls, api: &str) -> Self {
        Self {
            tokens,
            tls,
            api: api.trim_end_matches('/').to_owned(),
        }
    }

    /// Whether the account's sign-in allowed Katna to make filters.
    pub async fn allowed(&self) -> Result<bool> {
        self.tokens.has_scope(GOOGLE_GMAIL_SETTINGS).await
    }

    async fn call(&self, method: &str, path: &str, body: Option<&[u8]>) -> Result<Reply> {
        let url = format!("{}/gmail/v1/users/me/{path}", self.api);
        loop {
            let token = format!("Bearer {}", self.tokens.access_token().await?);
            let headers = [("Authorization", token.as_str())];
            let body = body.map(|b| ("application/json; charset=UTF-8", b));
            let reply =
                http::exchange(method, &url, &headers, body, None, &self.tls, TIMEOUT).await?;
            if reply.status == 401 && self.tokens.forget_access_token() {
                continue;
            }
            return Ok(reply);
        }
    }

    /// The user's labels: name → ID.
    pub async fn labels(&self) -> Result<HashMap<String, String>> {
        let reply = self.call("GET", "labels", None).await?;
        let labels: Labels = parse(&reply, "reading the labels")?;
        Ok(labels
            .labels
            .into_iter()
            .filter(|l| l.kind != "system")
            .map(|l| (l.name, l.id))
            .collect())
    }

    /// The forwarding addresses Gmail verified, lowercase.
    pub async fn forwarding(&self) -> Result<Vec<String>> {
        let reply = self
            .call("GET", "settings/forwardingAddresses", None)
            .await?;
        let forwarding: Forwarding = parse(&reply, "reading the forwarding addresses")?;
        Ok(forwarding
            .addresses
            .into_iter()
            .filter(|a| a.status == "accepted")
            .map(|a| a.email.to_lowercase())
            .collect())
    }

    /// Makes `filter`; returns its ID.
    pub async fn create(&self, filter: &Filter) -> Result<String> {
        let body = serde_json::to_vec(filter).map_err(|e| Error::Protocol(e.to_string()))?;
        let reply = self.call("POST", "settings/filters", Some(&body)).await?;
        let made: Made = parse(&reply, "making a filter")?;
        Ok(made.id)
    }

    /// Deletes filter `id`; one already gone is fine.
    pub async fn delete(&self, id: &str) -> Result<()> {
        let path = format!("settings/filters/{}", http::escape(id));
        let reply = self.call("DELETE", &path, None).await?;
        if reply.status == 404 {
            return Ok(());
        }
        check(&reply, "deleting a filter")
    }
}

/// Google's own words for a refusal.
fn refusal(reply: &Reply, doing: &str) -> Error {
    if let Some(off) = google_api::switched_off(reply.status, &reply.body) {
        return off;
    }
    #[derive(Deserialize, Default)]
    #[serde(default)]
    struct Answer {
        error: Body,
    }
    #[derive(Deserialize, Default)]
    #[serde(default)]
    struct Body {
        message: String,
    }
    let message = serde_json::from_slice::<Answer>(&reply.body)
        .map(|a| a.error.message)
        .unwrap_or_default();
    match reply.status {
        401 | 403 if message.is_empty() => Error::Auth("Gmail refused access".into()),
        401 => Error::Auth(format!("Gmail refused access: {message}")),
        _ if message.is_empty() => {
            Error::Rejected(format!("Gmail, {doing}: status {}", reply.status))
        }
        _ => Error::Rejected(format!("Gmail, {doing}: {message}")),
    }
}

fn check(reply: &Reply, doing: &str) -> Result<()> {
    if (200..300).contains(&reply.status) {
        Ok(())
    } else {
        Err(refusal(reply, doing))
    }
}

fn parse<T: for<'de> Deserialize<'de>>(reply: &Reply, doing: &str) -> Result<T> {
    check(reply, doing)?;
    serde_json::from_slice(&reply.body)
        .map_err(|err| Error::Protocol(format!("Gmail's answer {doing}: {err}")))
}

/// What [`push`] did on one account.
#[derive(Debug)]
pub struct Push {
    /// What is on Gmail now, by rule (the account's `mail_rule_remote`
    /// rows for Gmail).
    pub rows: Vec<RemoteRule>,
    /// Each rule of the account: on Gmail, or why not.
    pub verdicts: Vec<(i64, Verdict<()>)>,
    /// Gmail stopped answering, or refused, part way.
    pub error: Option<Error>,
}

/// Brings the Katna filters of `account` in line with `rules`: deletes
/// the filters of rules that changed, are gone or can't run on Gmail
/// (from `existing`, what is there now), then makes the missing ones.
/// Stops at the first error; [`Push::rows`] is then what Gmail has.
pub async fn push(
    client: &GmailSettings,
    rules: &[Rule],
    account: AccountId,
    folders: Vec<StoredFolder>,
    existing: Vec<RemoteRule>,
) -> Push {
    let mut rows: HashMap<i64, RemoteRule> = existing
        .into_iter()
        .filter(|r| r.runs_on == RunsOn::Gmail)
        .map(|r| (r.rule_id, r))
        .collect();
    let gmail = match async {
        Ok::<_, Error>(GmailAccount {
            folders,
            labels: client.labels().await?,
            forwarding: client.forwarding().await?,
        })
    }
    .await
    {
        Ok(gmail) => gmail,
        Err(err) => return failed(rules, account, rows, err),
    };
    let plan = rules_remote::plan(rules, account, RunsOn::Gmail, |rule, later| {
        translate(rule, &gmail, later)
    });
    let wanted: HashMap<i64, (String, Vec<Filter>)> = plan
        .iter()
        .filter_map(|(id, verdict)| {
            let filters = verdict.as_ref().ok()?;
            let spec = serde_json::to_string(filters).unwrap_or_default();
            Some((*id, (spec, filters.clone())))
        })
        .collect();

    // Out with the old.
    let mut stale: Vec<i64> = rows
        .iter()
        .filter(|(id, row)| wanted.get(*id).is_none_or(|(spec, _)| *spec != row.spec))
        .map(|(id, _)| *id)
        .collect();
    stale.sort_unstable();
    for id in stale {
        let Some(row) = rows.get_mut(&id) else {
            continue;
        };
        while let Some(filter) = row.remote_ids.first().cloned() {
            if let Err(err) = client.delete(&filter).await {
                return finish(plan, rows, Some(err));
            }
            row.remote_ids.remove(0);
        }
        rows.remove(&id);
    }

    // In with the new.
    for (id, verdict) in &plan {
        if verdict.is_err() || rows.contains_key(id) {
            continue;
        }
        let Some((spec, filters)) = wanted.get(id) else {
            continue;
        };
        let mut made = Vec::new();
        for filter in filters {
            match client.create(filter).await {
                Ok(filter_id) => made.push(filter_id),
                Err(err) => {
                    // Not half a rule: what was made goes again.
                    let mut left = Vec::new();
                    for filter_id in made {
                        if client.delete(&filter_id).await.is_err() {
                            left.push(filter_id);
                        }
                    }
                    if !left.is_empty() {
                        rows.insert(
                            *id,
                            RemoteRule {
                                rule_id: *id,
                                runs_on: RunsOn::Gmail,
                                remote_ids: left,
                                spec: String::new(),
                            },
                        );
                    }
                    return finish(plan, rows, Some(err));
                }
            }
        }
        rows.insert(
            *id,
            RemoteRule {
                rule_id: *id,
                runs_on: RunsOn::Gmail,
                remote_ids: made,
                spec: spec.clone(),
            },
        );
    }
    finish(plan, rows, None)
}

/// The verdicts once Gmail holds `rows`: a rule Gmail was to run but
/// doesn't hold failed with `error`.
fn finish(
    plan: Vec<(i64, Verdict<Vec<Filter>>)>,
    rows: HashMap<i64, RemoteRule>,
    error: Option<Error>,
) -> Push {
    let verdicts = plan
        .into_iter()
        .map(|(id, verdict)| {
            let verdict = match verdict {
                Ok(_) if rows.get(&id).is_some_and(|r| !r.spec.is_empty()) => Ok(()),
                Ok(_) => Err(RunsNote::Failed {
                    service: RunsOn::Gmail,
                    error: error
                        .as_ref()
                        .map_or_else(|| "not sent".to_owned(), ToString::to_string),
                }),
                Err(note) => Err(note),
            };
            (id, verdict)
        })
        .collect();
    let mut rows: Vec<RemoteRule> = rows.into_values().collect();
    rows.sort_by_key(|r| r.rule_id);
    Push {
        rows,
        verdicts,
        error,
    }
}

/// Every rule of the account failed with `error` before anything was
/// sent; `rows` stay as they were.
fn failed(
    rules: &[Rule],
    account: AccountId,
    rows: HashMap<i64, RemoteRule>,
    error: Error,
) -> Push {
    let note = match &error {
        Error::Auth(_) => RunsNote::SignIn,
        other => RunsNote::Failed {
            service: RunsOn::Gmail,
            error: other.to_string(),
        },
    };
    let verdicts = rules
        .iter()
        .filter(|r| r.enabled && r.covers(account))
        .map(|r| {
            if rows.get(&r.id).is_some_and(|row| !row.spec.is_empty()) {
                (r.id, Ok(()))
            } else {
                (r.id, Err(note.clone()))
            }
        })
        .collect();
    let mut rows: Vec<RemoteRule> = rows.into_values().collect();
    rows.sort_by_key(|r| r.rule_id);
    Push {
        rows,
        verdicts,
        error: Some(error),
    }
}

#[cfg(test)]
mod tests;
