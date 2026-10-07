// SPDX-License-Identifier: GPL-3.0-or-later

//! Running mail rules (`docs/ARCHITECTURE.md` §9.4) on new incoming mail.
//!
//! The model and the matcher are `katna_store::rules`; this module carries
//! out what matched, through the same functions as the user's own changes
//! ([`ops`], [`outbox`], `katna_meta`), so every change reaches the server
//! by the operation queue.
//!
//! [`Watch`] keeps, per account, which mail is new: mail stored after it
//! started watching, never the mail of a first sync. The daemon runs it
//! after each sync and after bodies arrive, before new-mail notifications.

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use katna_core::AccountId;
use katna_store::rules::{Action, MailFacts, Matcher, Rule, matching};
use katna_store::{FolderId, FolderRole, MessageFlags, MessageId, Store, StoredMessage};
use mail_parser::{MessageParser, MimeHeaders};

use crate::ops::{self, ChangeError};
use crate::outbox::{self, QueueError};
use crate::quick_reply::{Mailbox, address, encode_words};

/// Mail older than this is not new, even when it is new to the store (a
/// folder synced for the first time, older mail fetched later).
pub const NEWER_THAN: i64 = 2 * 24 * 3600;
/// How long new mail waits for its body when a rule needs it: bodies
/// usually arrive a few seconds after the headers.
pub const HOLD: i64 = 120;
/// At most this much text is matched per message.
const MAX_BODY_BYTES: usize = 256 * 1024;
const DAY: i64 = 86_400;

/// How rules run.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// Unix seconds.
    pub now: i64,
    /// On mail as it arrives. Otherwise on mail already here ("also apply
    /// to these"): nothing is forwarded and notifications don't matter.
    pub live: bool,
    /// Whether the account can send mail (forwarding).
    pub can_send: bool,
}

/// What a run of the rules did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Outcome {
    /// Accounts whose mail changed: their workers send the changes.
    pub accounts: Vec<AccountId>,
    /// How many messages a rule changed.
    pub changed: u32,
    /// Messages not to notify about.
    pub quiet: Vec<MessageId>,
    /// Messages that wait for their body: don't notify about them yet.
    pub held: Vec<MessageId>,
    /// Outbox entries of forwarded mail.
    pub outbox: Vec<i64>,
    /// Rules whose action failed, and why. A live run switched them off.
    pub failed: Vec<(i64, String)>,
}

/// The text of the raw message `raw`: its body (HTML as text) and its
/// attachments' names.
pub fn mail_text(raw: &[u8]) -> (String, Vec<String>) {
    let Some(message) = MessageParser::default().parse(raw) else {
        return (String::new(), Vec::new());
    };
    let mut body = String::new();
    for index in 0..message.text_body.len() {
        if body.len() >= MAX_BODY_BYTES {
            break;
        }
        if let Some(text) = message.body_text(index) {
            if !body.is_empty() {
                body.push('\n');
            }
            body.push_str(&katna_crypto::without_armor(&text));
        }
    }
    let names = message
        .attachments()
        .filter_map(|part| part.attachment_name())
        .map(str::to_owned)
        .collect();
    (body, names)
}

/// What rules look at in `message`: its stored body when there is one,
/// else its snippet.
pub fn facts(store: &Store, message: &StoredMessage) -> katna_store::Result<MailFacts> {
    let mut names = store.attachment_names(message.id)?;
    let raw = match &message.blob_hash {
        Some(hash) => store.blobs().get(hash)?,
        None => None,
    };
    let body = raw.map(|raw| {
        let (body, more) = mail_text(&raw);
        for name in more {
            if !names.contains(&name) {
                names.push(name);
            }
        }
        body
    });
    Ok(MailFacts::of(message, names, body))
}

/// Which mail of one account is new, for its rules.
#[derive(Debug, Clone)]
pub struct Watch {
    /// Mail up to this message was stored before watching began.
    after: MessageId,
    /// `false` until the account's first sync: its mail is not new.
    primed: bool,
    /// Newer messages the rules already ran on. A message is new when it
    /// reaches the inbox, not when it is stored (Gmail's mail can land in
    /// All Mail a sync before its inbox copy), and only once: mail moved
    /// back into the inbox is not new again.
    done: HashSet<MessageId>,
    /// Messages waiting for their body, since when.
    waiting: HashMap<MessageId, i64>,
}

impl Watch {
    /// Starts watching `account`: mail stored from now on is new. An
    /// account without mail yet waits for its first sync.
    pub fn new(store: &Store, account: AccountId) -> katna_store::Result<Self> {
        let after = store.latest_message(account)?;
        Ok(Self {
            after,
            primed: after.0 > 0,
            done: HashSet::new(),
            waiting: HashMap::new(),
        })
    }

    /// Whether new mail waits for its body before the rules run on it.
    pub fn waiting(&self) -> bool {
        !self.waiting.is_empty()
    }

    /// Runs the rules of `account` on its new mail. With
    /// `wait_for_bodies`, mail a rule needs the body of waits for it, for
    /// up to [`HOLD`] seconds; else the snippet stands in.
    pub fn run(
        &mut self,
        store: &mut Store,
        account: AccountId,
        context: &Context,
        wait_for_bodies: bool,
    ) -> katna_store::Result<Outcome> {
        let mut out = Outcome::default();
        if !self.primed {
            self.after = store.latest_message(account)?;
            self.primed = true;
            return Ok(out);
        }
        let matchers = compile(store, account, &mut out)?;
        let candidates =
            store.rule_candidates(account, self.after, context.now.saturating_sub(NEWER_THAN))?;
        let needs_body = matchers.iter().any(|m| {
            m.rule().needs_body()
                || m.rule()
                    .actions
                    .iter()
                    .any(|a| matches!(a, Action::Forward { .. }))
        });
        let mut failed = HashSet::new();
        for message in candidates {
            if self.done.contains(&message.id) {
                continue;
            }
            if needs_body && message.blob_hash.is_none() && wait_for_bodies {
                let since = *self.waiting.entry(message.id).or_insert(context.now);
                if context.now - since < HOLD {
                    out.held.push(message.id);
                    continue;
                }
            }
            self.waiting.remove(&message.id);
            self.done.insert(message.id);
            if matchers.is_empty() {
                continue;
            }
            let facts = facts(store, &message)?;
            let mut changed = false;
            for matcher in matching(&matchers, &facts) {
                let rule = matcher.rule();
                if failed.contains(&rule.id) {
                    continue;
                }
                match apply(store, rule, &message, context, &mut out)? {
                    Ok(did) => changed |= did,
                    Err(reason) => {
                        tracing::warn!(rule = rule.id, name = rule.name, %reason, "rule switched off");
                        store.fail_rule(rule.id, &reason)?;
                        failed.insert(rule.id);
                        out.failed.push((rule.id, reason));
                    }
                }
            }
            if changed {
                out.changed += 1;
            }
        }
        Ok(out)
    }
}

/// The rules that run here on mail of `account`, compiled: not those its
/// mail service runs ([`crate::rules_remote`]). A rule that no longer
/// compiles is switched off.
fn compile(
    store: &mut Store,
    account: AccountId,
    out: &mut Outcome,
) -> katna_store::Result<Vec<Matcher>> {
    let mut matchers = Vec::new();
    let elsewhere = store.rules_on_service(account)?;
    for rule in store.rules()? {
        if !rule.enabled || !rule.covers(account) || elsewhere.contains(&rule.id) {
            continue;
        }
        let id = rule.id;
        match Matcher::new(rule) {
            Ok(matcher) => matchers.push(matcher),
            Err(reason) => {
                store.fail_rule(id, &reason)?;
                out.failed.push((id, reason));
            }
        }
    }
    Ok(matchers)
}

/// Runs `rule`, on or off, once over the inbox mail of its accounts from
/// the last `days` days ("also apply to these"). Nothing is forwarded.
/// Stops at the first action that fails, without switching the rule off.
pub fn apply_recent(
    store: &mut Store,
    rule: &Rule,
    days: u32,
    now: i64,
) -> katna_store::Result<Outcome> {
    let mut out = Outcome::default();
    let matcher = match Matcher::new(Rule {
        enabled: true,
        stop: false,
        ..rule.clone()
    }) {
        Ok(matcher) => matcher,
        Err(reason) => {
            out.failed.push((rule.id, reason));
            return Ok(out);
        }
    };
    let context = Context {
        now,
        live: false,
        can_send: false,
    };
    let since = now.saturating_sub(i64::from(days) * DAY);
    for &account in &rule.accounts {
        for message in store.rule_candidates(AccountId(account), MessageId(0), since)? {
            if !matcher.matches(&facts(store, &message)?) {
                continue;
            }
            match apply(store, matcher.rule(), &message, &context, &mut out)? {
                Ok(true) => out.changed += 1,
                Ok(false) => {}
                Err(reason) => {
                    out.failed.push((rule.id, reason));
                    return Ok(out);
                }
            }
        }
    }
    Ok(out)
}

/// `rule` as it runs on the mail of `account`: without the folders and
/// labels it names in its other accounts.
pub fn for_account(store: &Store, rule: &Rule, account: AccountId) -> katna_store::Result<Rule> {
    let mut out = rule.clone();
    let mut actions = Vec::with_capacity(rule.actions.len());
    for action in &rule.actions {
        let elsewhere = match action.folder() {
            Some(folder) => matches!(place(store, rule, folder, account)?, Place::Elsewhere),
            None => false,
        };
        if !elsewhere {
            actions.push(action.clone());
        }
    }
    out.actions = actions;
    Ok(out)
}

/// Where a folder an action names is.
enum Place {
    /// A folder of the message's account.
    Here(FolderId),
    /// A folder of another account of the rule: not for this message.
    Elsewhere,
    Gone,
}

fn place(
    store: &Store,
    rule: &Rule,
    folder: i64,
    account: AccountId,
) -> katna_store::Result<Place> {
    if store.folders(account)?.iter().any(|f| f.id.0 == folder) {
        return Ok(Place::Here(FolderId(folder)));
    }
    for &other in &rule.accounts {
        if other != account.0
            && store
                .folders(AccountId(other))?
                .iter()
                .any(|f| f.id.0 == folder)
        {
            return Ok(Place::Elsewhere);
        }
    }
    Ok(Place::Gone)
}

/// Carries out `rule`'s actions on `message`: flags and labels first, the
/// move last. Returns whether anything changed, or why an action failed
/// (`Err` inside).
fn apply(
    store: &mut Store,
    rule: &Rule,
    message: &StoredMessage,
    context: &Context,
    out: &mut Outcome,
) -> katna_store::Result<Result<bool, String>> {
    let account = message.account;
    let ids = [message.id];
    let gone = |folder: i64| format!("folder {folder} no longer exists");
    let mut changed = false;
    let mut moves = None;
    for action in &rule.actions {
        let result = match action {
            Action::MarkRead => {
                ops::set_flags(store, &ids, MessageFlags::SEEN, MessageFlags::empty())
            }
            Action::Star => {
                ops::set_flags(store, &ids, MessageFlags::FLAGGED, MessageFlags::empty())
            }
            Action::MarkImportant => {
                ops::set_flags(store, &ids, MessageFlags::IMPORTANT, MessageFlags::empty())
            }
            // As Label as does (`SetLabels`) on Gmail; elsewhere, a copy
            // in the folder.
            Action::AddLabel { folder } => match place(store, rule, *folder, account)? {
                Place::Here(to) if crate::folders::is_gmail(&store.folders(account)?) => {
                    ops::set_labels(store, &ids, &[to], &[])
                }
                Place::Here(to) => ops::copy_messages(store, &ids, to),
                Place::Elsewhere => continue,
                Place::Gone => return Ok(Err(gone(*folder))),
            },
            Action::DontNotify => {
                if context.live {
                    out.quiet.push(message.id);
                }
                continue;
            }
            Action::Forward { to } => {
                if !context.live {
                    continue;
                }
                if !context.can_send {
                    return Ok(Err("the account cannot send mail".into()));
                }
                match forward(store, message, to, context.now)? {
                    Ok(Some(id)) => {
                        out.outbox.push(id);
                        changed = true;
                    }
                    Ok(None) => {
                        tracing::info!(message = %message.id, "not forwarded: no body stored");
                    }
                    Err(reason) => return Ok(Err(reason)),
                }
                continue;
            }
            Action::MarkReadAfter { days } => {
                let from = if context.live {
                    context.now
                } else {
                    message.date.unwrap_or(context.now)
                };
                let at = (from + i64::from(*days) * DAY).max(context.now);
                katna_meta::set_read_after(store, message.id, at)?;
                changed = true;
                continue;
            }
            // A rule for several accounts names a folder in each: the one
            // of this account.
            Action::Move { folder } => {
                match place(store, rule, *folder, account)? {
                    Place::Here(_) | Place::Gone => moves = Some(action),
                    Place::Elsewhere => {}
                }
                continue;
            }
            Action::Archive | Action::Trash => {
                moves = Some(action);
                continue;
            }
        };
        match done(result, out)? {
            Ok(did) => changed |= did,
            Err(reason) => return Ok(Err(reason)),
        }
    }
    let to = match moves {
        None => return Ok(Ok(changed)),
        Some(Action::Move { folder }) => match place(store, rule, *folder, account)? {
            Place::Here(to) => to,
            Place::Elsewhere => return Ok(Ok(changed)),
            Place::Gone => return Ok(Err(gone(*folder))),
        },
        // As Archive does: `\Archive`, or All Mail on Gmail.
        Some(Action::Archive) => {
            let folders = store.folders(account)?;
            let archive = [FolderRole::Archive, FolderRole::All]
                .into_iter()
                .find_map(|role| folders.iter().find(|f| f.role == Some(role)));
            match archive {
                Some(folder) => folder.id,
                None => return Ok(Err("the account has no archive folder".into())),
            }
        }
        // Never "delete for good", which Delete does in an account
        // without a Trash folder or for mail already there.
        Some(_) => match store.trash_folder(account)? {
            Some(trash) => trash,
            None => return Ok(Err("the account has no Trash folder".into())),
        },
    };
    // Out of the inbox, not out of a label a rule just added.
    let inbox = store
        .folders(account)?
        .into_iter()
        .find(|f| f.role == Some(FolderRole::Inbox))
        .map(|f| f.id)
        .filter(|inbox| *inbox != to);
    let in_inbox = match inbox {
        Some(inbox) => store
            .locations(message.id)?
            .iter()
            .any(|l| l.folder == inbox),
        None => false,
    };
    let result = if in_inbox {
        ops::move_messages_from(store, &ids, inbox, to)
    } else {
        ops::move_messages(store, &ids, to)
    };
    Ok(done(result, out)?.map(|did| did || changed))
}

/// The result of one change: whether it changed anything (noting the
/// accounts), or why it could not be made.
fn done(
    result: Result<Vec<AccountId>, ChangeError>,
    out: &mut Outcome,
) -> katna_store::Result<Result<bool, String>> {
    match result {
        Ok(accounts) => {
            let did = !accounts.is_empty();
            for account in accounts {
                if !out.accounts.contains(&account) {
                    out.accounts.push(account);
                }
            }
            Ok(Ok(did))
        }
        // Gone meanwhile (expunged elsewhere): nothing to do.
        Err(ChangeError::UnknownMessage(_)) => Ok(Ok(false)),
        Err(ChangeError::UnknownFolder(folder)) => {
            Ok(Err(format!("folder {folder} no longer exists")))
        }
        Err(ChangeError::NotPossible(reason) | ChangeError::Invalid(reason)) => Ok(Err(reason)),
        Err(ChangeError::Store(err)) => Err(err),
    }
}

/// Queues `message` forwarded to `to`, as an attachment, from its account.
/// `None` when its body is not stored.
fn forward(
    store: &mut Store,
    message: &StoredMessage,
    to: &str,
    now: i64,
) -> katna_store::Result<Result<Option<i64>, String>> {
    let raw = match &message.blob_hash {
        Some(hash) => store.blobs().get(hash)?,
        None => None,
    };
    let Some(raw) = raw else {
        return Ok(Ok(None));
    };
    let Some(account) = store
        .accounts()?
        .into_iter()
        .find(|a| a.id == message.account)
    else {
        return Ok(Ok(None));
    };
    let from = Mailbox {
        name: Some(account.display_name.clone()),
        email: account.address.clone(),
    };
    let to = Mailbox {
        name: None,
        email: to.trim().to_owned(),
    };
    let built = forwarded(&raw, &from, &to, &message.subject);
    match outbox::queue(store, account.id, &built, 0, now) {
        Ok(id) => Ok(Ok(Some(id))),
        Err(QueueError::Invalid(reason)) => Ok(Err(reason)),
        Err(QueueError::Store(err)) => Err(err),
    }
}

/// `raw` forwarded from `from` to `to` as a `message/rfc822` attachment,
/// under "Fwd: `subject`". The outbox adds `Date` and `Message-ID`.
pub fn forwarded(raw: &[u8], from: &Mailbox, to: &Mailbox, subject: &str) -> Vec<u8> {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    raw.hash(&mut hasher);
    let boundary = format!("katna-forward-{:016x}", hasher.finish());
    let subject = subject.trim();
    let already = subject
        .get(..4)
        .is_some_and(|start| start.eq_ignore_ascii_case("fwd:"));
    let subject = if already {
        subject.to_owned()
    } else {
        format!("Fwd: {subject}")
    };
    let mut out = String::new();
    out.push_str(&format!("From: {}\r\n", address(from)));
    out.push_str(&format!("To: {}\r\n", address(to)));
    out.push_str(&format!("Subject: {}\r\n", encode_words(&subject)));
    out.push_str("MIME-Version: 1.0\r\n");
    out.push_str(&format!(
        "Content-Type: multipart/mixed; boundary=\"{boundary}\"\r\n\r\n"
    ));
    out.push_str(&format!("--{boundary}\r\n"));
    out.push_str("Content-Type: message/rfc822\r\n");
    out.push_str("Content-Disposition: attachment; filename=\"forwarded.eml\"\r\n");
    out.push_str("Content-Transfer-Encoding: 8bit\r\n\r\n");
    let mut bytes = out.into_bytes();
    bytes.extend_from_slice(raw);
    if !raw.ends_with(b"\r\n") {
        bytes.extend_from_slice(b"\r\n");
    }
    bytes.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    bytes
}
