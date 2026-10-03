// SPDX-License-Identifier: GPL-3.0-or-later

//! Mail rules as a Sieve script (RFC 5228) on the account's mail server
//! (`docs/ARCHITECTURE.md` §9.4), sent with ManageSieve
//! ([`managesieve`], RFC 5804).
//!
//! Katna keeps one script per account, named [`SCRIPT`], and never
//! changes or deletes another. A rule goes in it when Sieve does what
//! Katna does with it:
//!
//! - **Tests**: `header` for subject and the address fields (Katna matches
//!   an address field on the name or the address, and so does a header
//!   test); `address :all` where the value is an address; `body :text`
//!   with the `body` extension; `:regex` with the `regex` extension, for
//!   patterns that mean the same in Rust and POSIX. Text compares without
//!   case (`i;ascii-casemap`, or `i;unicode-casemap` when the server has
//!   it and the value needs it). Attachments stay in Katna.
//! - **Actions**: `fileinto` (move, archive, trash), `addflag` with
//!   `imap4flags` (mark read, star), `redirect :copy` (forward), `stop`.
//!   Mark important, labels, "don't notify" and "mark read after" stay in
//!   Katna.
//!
//! When another script is active, Katna's includes it first (`include
//! :personal`, with the `include` extension) and takes its place, so it
//! still runs; without `include` the rules stay in Katna.

use std::collections::{BTreeSet, HashSet};

use katna_core::AccountId;
use katna_store::{
    remote::{FolderRole, StoredFolder},
    rules::{Action, Comparator, Condition, Field, MatchMode, Rule, RunsNote, RunsOn},
};

use crate::rules_remote::{self, Verdict};

pub mod managesieve;

/// The name of Katna's script on the server.
pub const SCRIPT: &str = "katna";

/// What the server's Sieve has, lowercase (`"fileinto"`, `"regex"`,
/// `"comparator-i;unicode-casemap"`…).
#[derive(Debug, Clone, Default)]
pub struct Extensions(HashSet<String>);

impl Extensions {
    pub fn new<'a>(names: impl IntoIterator<Item = &'a str>) -> Self {
        Self(names.into_iter().map(str::to_ascii_lowercase).collect())
    }

    pub fn has(&self, name: &str) -> bool {
        self.0.contains(name)
    }
}

/// One rule in Sieve: the extensions it needs and its `if` block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SieveRule {
    pub requires: BTreeSet<&'static str>,
    pub text: String,
}

/// The script for `account`, with what was decided for each of its rules:
/// those that run on the server are in it, in list order. `include` is
/// the script that was active before Katna's, run first.
pub fn account_script(
    rules: &[Rule],
    account: AccountId,
    folders: &[StoredFolder],
    extensions: &Extensions,
    include: Option<&str>,
) -> (String, Vec<(i64, Verdict<()>)>) {
    let plan = rules_remote::plan(rules, account, RunsOn::Sieve, |rule, _| {
        translate(rule, folders, extensions)
    });
    let mut requires = BTreeSet::new();
    if include.is_some() {
        requires.insert("include");
    }
    let mut blocks = Vec::new();
    let verdicts = plan
        .into_iter()
        .map(|(id, verdict)| match verdict {
            Ok(rule) => {
                requires.extend(rule.requires);
                blocks.push(rule.text);
                (id, Ok(()))
            }
            Err(note) => (id, Err(note)),
        })
        .collect();
    let mut script = String::from(
        "# Katna Mail's rules. Katna writes this script again whenever a rule\n\
         # changes: change them in Katna Mail, Settings > Folders & rules.\n",
    );
    if !requires.is_empty() {
        let list: Vec<String> = requires.iter().map(|r| quote(r)).collect();
        script.push_str(&format!("require [{}];\n", list.join(", ")));
    }
    if let Some(old) = include {
        script.push_str(&format!("include :personal {};\n", quote(old)));
    }
    for block in blocks {
        script.push('\n');
        script.push_str(&block);
    }
    (script, verdicts)
}

/// Writes Katna's script for `account` on the server of `session` and
/// makes it the active one; returns what was decided for each rule.
///
/// Another active script is included first when the server has
/// `include`; otherwise nothing is written and every rule stays in Katna
/// ([`RunsNote::OtherScript`]). Nothing is written either while no rule
/// runs on the server and Katna has no script there yet.
pub async fn push(
    session: &mut managesieve::ManageSieve,
    rules: &[Rule],
    account: AccountId,
    folders: &[StoredFolder],
) -> crate::Result<Vec<(i64, Verdict<()>)>> {
    let extensions = Extensions::new(session.capabilities().sieve.iter().map(String::as_str));
    let scripts = session.list().await?;
    let ours = scripts.iter().any(|(name, _)| name == SCRIPT);
    let active = scripts
        .iter()
        .find(|(_, active)| *active)
        .map(|(name, _)| name.clone());
    let include = match active.as_deref() {
        Some(SCRIPT) => included(&session.get(SCRIPT).await?),
        Some(other) => {
            if !extensions.has("include") {
                let note = RunsNote::OtherScript {
                    name: other.to_owned(),
                };
                return Ok(rules
                    .iter()
                    .filter(|r| r.enabled && r.covers(account))
                    .map(|r| (r.id, Err(note.clone())))
                    .collect());
            }
            Some(other.to_owned())
        }
        None => None,
    };
    let (script, verdicts) =
        account_script(rules, account, folders, &extensions, include.as_deref());
    if !ours && verdicts.iter().all(|(_, v)| v.is_err()) {
        return Ok(verdicts);
    }
    session.put(SCRIPT, &script).await?;
    if active.as_deref() != Some(SCRIPT) {
        session.set_active(SCRIPT).await?;
    }
    Ok(verdicts)
}

/// The script that `include`s, if Katna's script `text` runs one first.
pub fn included(text: &str) -> Option<String> {
    let rest = text
        .lines()
        .map(str::trim)
        .find_map(|line| line.strip_prefix("include :personal "))?;
    let rest = rest.strip_suffix(';')?.trim();
    let inner = rest.strip_prefix('"')?.strip_suffix('"')?;
    Some(inner.replace("\\\"", "\"").replace("\\\\", "\\"))
}

/// `rule` in Sieve for an account with `folders`, or why the server
/// can't run it as Katna does.
pub fn translate(
    rule: &Rule,
    folders: &[StoredFolder],
    extensions: &Extensions,
) -> Result<SieveRule, RunsNote> {
    let mut requires = BTreeSet::new();
    let tests = rule
        .conditions
        .iter()
        .map(|c| test(c, extensions, &mut requires))
        .collect::<Result<Vec<_>, _>>()?;
    let test = match (rule.match_mode, tests.as_slice()) {
        (_, [one]) => one.clone(),
        (MatchMode::All, _) => format!("allof ({})", tests.join(", ")),
        (MatchMode::Any, _) => format!("anyof ({})", tests.join(", ")),
    };

    let mut flags = Vec::new();
    let mut sends = Vec::new();
    let mut files = Vec::new();
    for action in &rule.actions {
        let cant = || RunsNote::Action {
            service: RunsOn::Sieve,
            action: action.clone(),
        };
        let folder_gone = || RunsNote::Folder {
            service: RunsOn::Sieve,
        };
        match action {
            Action::MarkRead | Action::Star => {
                if !extensions.has("imap4flags") {
                    return Err(cant());
                }
                requires.insert("imap4flags");
                let flag = if *action == Action::MarkRead {
                    "\\Seen"
                } else {
                    "\\Flagged"
                };
                flags.push(format!("addflag {};", quote(flag)));
            }
            Action::Forward { to } => {
                if !extensions.has("copy") {
                    return Err(cant());
                }
                requires.insert("copy");
                sends.push(format!("redirect :copy {};", quote(to.trim())));
            }
            Action::Move { folder } => {
                let folder = folders
                    .iter()
                    .find(|f| f.id.0 == *folder)
                    .ok_or_else(folder_gone)?;
                files.push(file_into(folder, extensions, &mut requires).ok_or_else(cant)?);
            }
            Action::Archive | Action::Trash => {
                let role = if *action == Action::Archive {
                    FolderRole::Archive
                } else {
                    FolderRole::Trash
                };
                let folder = folders
                    .iter()
                    .find(|f| f.role == Some(role))
                    .ok_or_else(folder_gone)?;
                files.push(file_into(folder, extensions, &mut requires).ok_or_else(cant)?);
            }
            Action::MarkImportant
            | Action::AddLabel { .. }
            | Action::DontNotify
            | Action::MarkReadAfter { .. } => return Err(cant()),
        }
    }

    let mut text = format!("# {}\nif {test} {{\n", one_line(&rule.name));
    for line in flags.iter().chain(&sends).chain(&files) {
        text.push_str(&format!("    {line}\n"));
    }
    if rule.stop {
        text.push_str("    stop;\n");
    }
    text.push_str("}\n");
    Ok(SieveRule { requires, text })
}

/// `fileinto` the folder; the inbox is where mail stays (`keep`).
fn file_into(
    folder: &StoredFolder,
    extensions: &Extensions,
    requires: &mut BTreeSet<&'static str>,
) -> Option<String> {
    if folder.role == Some(FolderRole::Inbox) || folder.path.eq_ignore_ascii_case("INBOX") {
        return Some("keep;".into());
    }
    if !extensions.has("fileinto") {
        return None;
    }
    requires.insert("fileinto");
    Some(format!("fileinto {};", quote(&folder.path)))
}

/// The headers an address field is in.
fn headers(field: Field) -> &'static str {
    match field {
        Field::From => "\"from\"",
        Field::To => "\"to\"",
        Field::Cc => "\"cc\"",
        Field::AnyRecipient => "[\"to\", \"cc\", \"bcc\"]",
        Field::ReplyTo => "\"reply-to\"",
        Field::Subject => "\"subject\"",
        Field::Body
        | Field::AttachmentName
        | Field::HasAttachment
        | Field::Tab
        | Field::MailingList => "",
    }
}

/// One condition as a Sieve test.
fn test(
    condition: &Condition,
    extensions: &Extensions,
    requires: &mut BTreeSet<&'static str>,
) -> Result<String, RunsNote> {
    let cant = || RunsNote::Condition {
        service: RunsOn::Sieve,
        field: condition.field,
        comparator: condition.comparator,
    };
    let value = condition.value.trim();
    if condition.field == Field::MailingList {
        let test = "exists \"list-id\"";
        return Ok(if katna_store::rules::says_no(value) {
            format!("not {test}")
        } else {
            test.to_owned()
        });
    }
    if value.chars().any(char::is_control) {
        return Err(cant());
    }
    // Katna compares without case in every script; ascii-casemap only
    // folds ASCII letters.
    let comparator = if value.is_ascii() {
        ""
    } else if extensions.has("comparator-i;unicode-casemap") {
        requires.insert("comparator-i;unicode-casemap");
        " :comparator \"i;unicode-casemap\""
    } else {
        return Err(cant());
    };
    let address_field = matches!(
        condition.field,
        Field::From | Field::To | Field::Cc | Field::AnyRecipient | Field::ReplyTo
    );
    let positive = match (condition.field, condition.comparator) {
        // Inbox tabs are Katna's own.
        (Field::HasAttachment | Field::AttachmentName | Field::Tab, _) => return Err(cant()),
        (Field::MailingList, _) => return Err(cant()),
        (Field::Body, Comparator::Contains | Comparator::NotContains) => {
            if !extensions.has("body") {
                return Err(cant());
            }
            requires.insert("body");
            format!("body{comparator} :text :contains {}", quote(value))
        }
        (Field::Body, _) => return Err(cant()),
        (field, Comparator::Contains | Comparator::NotContains) => {
            format!(
                "header{comparator} :contains {} {}",
                headers(field),
                quote(value)
            )
        }
        (Field::Subject, Comparator::Equals) => {
            format!("header{comparator} :is \"subject\" {}", quote(value))
        }
        (Field::Subject, Comparator::BeginsWith) => format!(
            "header{comparator} :matches \"subject\" {}",
            quote(&format!("{}*", wildcard_escape(value)))
        ),
        (Field::Subject, Comparator::EndsWith) => format!(
            "header{comparator} :matches \"subject\" {}",
            quote(&format!("*{}", wildcard_escape(value)))
        ),
        // A name equal to, or starting with, the value is not something an
        // address test sees: only values that are addresses.
        (field, Comparator::Equals) if address_field && value.contains('@') => format!(
            "address{comparator} :all :is {} {}",
            headers(field),
            quote(value)
        ),
        (field, Comparator::BeginsWith) if address_field && value.contains('@') => format!(
            "address{comparator} :all :matches {} {}",
            headers(field),
            quote(&format!("{}*", wildcard_escape(value)))
        ),
        (field, Comparator::EndsWith)
            if address_field && (value.contains('@') || value.contains('.')) =>
        {
            format!(
                "address{comparator} :all :matches {} {}",
                headers(field),
                quote(&format!("*{}", wildcard_escape(value)))
            )
        }
        (_, Comparator::Equals | Comparator::BeginsWith | Comparator::EndsWith) => {
            return Err(cant());
        }
        (field, Comparator::Matches) => {
            // Katna tests the name and the address apart; a header test
            // sees "Name <address>", where anchors mean something else.
            let anchored = value.contains('^') || value.contains('$');
            if !extensions.has("regex") || !portable_regex(value) || (address_field && anchored) {
                return Err(cant());
            }
            requires.insert("regex");
            // `i;ascii-casemap` is what servers allow with :regex.
            format!(
                "header :comparator \"i;ascii-casemap\" :regex {} {}",
                headers(field),
                quote(value)
            )
        }
    };
    Ok(if condition.comparator == Comparator::NotContains {
        format!("not {positive}")
    } else {
        positive
    })
}

/// Whether `pattern` means the same as a POSIX extended regular
/// expression (what Sieve's `regex` runs) as in Rust's `regex`, which
/// Katna runs: ASCII, no classes like `\d`, no `(?…)` groups or lazy
/// repeats, and no non-ASCII.
pub fn portable_regex(pattern: &str) -> bool {
    if !pattern.is_ascii() || pattern.contains("(?") {
        return false;
    }
    let bytes = pattern.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => {
                // Only escaped punctuation: `\.` is a dot in both.
                match bytes.get(i + 1) {
                    Some(next) if b".*+?|()[]{}^$\\-/@".contains(next) => i += 2,
                    _ => return false,
                }
                continue;
            }
            b'*' | b'+' | b'?' | b'}' if bytes.get(i + 1) == Some(&b'?') => return false,
            b'*' | b'+' if matches!(bytes.get(i + 1), Some(b'*' | b'+')) => return false,
            c if c.is_ascii_control() => return false,
            _ => {}
        }
        i += 1;
    }
    true
}

/// `text` with `*`, `?` and `\` taken literally by `:matches`.
fn wildcard_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if matches!(c, '*' | '?' | '\\') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// A Sieve quoted string.
pub fn quote(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        if matches!(c, '"' | '\\') {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('"');
    out
}

/// A rule name as one comment line.
fn one_line(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect::<String>()
        .trim()
        .to_owned()
}

#[cfg(test)]
mod tests;
