// SPDX-License-Identifier: GPL-3.0-or-later

//! Search on the server for mail that is not downloaded, whose text the
//! search index has not seen: the "More results on server" section under
//! the results found here (`docs/ARCHITECTURE.md` §7.3).
//!
//! The server answers with UIDs; everything else about the messages is
//! stored already, so only the ones not downloaded are kept.

use std::collections::{HashMap, HashSet};

use katna_core::AccountId;
use katna_store::{FolderId, FolderRole, MessageId, Store};

use crate::{MailBackend, Result};

/// At most this many folders are searched per account, the ones with the
/// most mail not downloaded first.
pub const MAX_FOLDERS: usize = 40;

/// Where a [`Criterion::Text`] looks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    /// Anywhere in the message: headers and body.
    Any,
    From,
    To,
    Cc,
    Bcc,
    Subject,
    /// An attachment's file name (Gmail only).
    Filename,
    /// The mailing list (`List-Id`).
    List,
}

/// A flag a [`Criterion::Flag`] tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    Seen,
    Answered,
    Flagged,
    Draft,
}

/// What to search the server for: a search box query, in terms both IMAP
/// `SEARCH` and Gmail's `X-GM-RAW` can say.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Criterion {
    All,
    And(Vec<Criterion>),
    Or(Vec<Criterion>),
    Not(Box<Criterion>),
    /// Words in `field`; more than one must appear as a phrase.
    Text(Field, String),
    HasAttachment,
    /// The flag is set (`true`) or not.
    Flag(Flag, bool),
    /// A Gmail label.
    Label(String),
    /// Only in the folder with this path, last path part or role.
    In(String),
    /// Dated before this Unix time.
    Before(i64),
    /// Dated at or after this Unix time.
    After(i64),
    Larger(u64),
    Smaller(u64),
    /// Something servers cannot search for, such as a sender's
    /// organization: no server is asked.
    Unsupported,
}

impl Criterion {
    /// Gmail's search syntax for `X-GM-RAW`; `None` when a part has no
    /// such form or the text needs more than plain ASCII.
    pub fn gmail(&self) -> Option<String> {
        let words = |text: &str| -> Option<String> {
            let text: String = text.chars().filter(|c| !matches!(c, '"' | '\\')).collect();
            let text = text.trim();
            if text.is_empty() || !text.is_ascii() || text.contains(['\r', '\n']) {
                return None;
            }
            Some(
                if text.contains(char::is_whitespace)
                    || text.contains(['(', ')', '{', '}', ':', '-'])
                {
                    format!("\"{text}\"")
                } else {
                    text.to_owned()
                },
            )
        };
        // An empty part matches everything: it drops out of an And and
        // makes an Or match everything.
        let joined = |items: &[Criterion], or: bool| -> Option<String> {
            let parts: Vec<String> = items.iter().map(Criterion::gmail).collect::<Option<_>>()?;
            if or && parts.iter().any(String::is_empty) {
                return Some(String::new());
            }
            let parts: Vec<String> = parts.into_iter().filter(|p| !p.is_empty()).collect();
            Some(match parts.len() {
                0 => String::new(),
                1 => parts.into_iter().next().unwrap_or_default(),
                _ => format!("({})", parts.join(if or { " OR " } else { " " })),
            })
        };
        Some(match self {
            Self::All => String::new(),
            Self::And(items) => joined(items, false)?,
            Self::Or(items) => joined(items, true)?,
            Self::Not(inner) => match inner.gmail()? {
                inner if inner.is_empty() => return None,
                inner if inner.starts_with('(') => format!("-{inner}"),
                inner => format!("-({inner})"),
            },
            Self::Text(field, text) => {
                let text = words(text)?;
                match field {
                    Field::Any => text,
                    Field::From => format!("from:{text}"),
                    Field::To => format!("to:{text}"),
                    Field::Cc => format!("cc:{text}"),
                    Field::Bcc => format!("bcc:{text}"),
                    Field::Subject => format!("subject:{text}"),
                    Field::Filename => format!("filename:{text}"),
                    Field::List => format!("list:{text}"),
                }
            }
            Self::HasAttachment => "has:attachment".into(),
            Self::Flag(flag, on) => match (flag, on) {
                (Flag::Seen, true) => "is:read".into(),
                (Flag::Seen, false) => "is:unread".into(),
                (Flag::Flagged, true) => "is:starred".into(),
                (Flag::Flagged, false) => "-is:starred".into(),
                (Flag::Draft, true) => "in:drafts".into(),
                (Flag::Draft, false) => "-in:drafts".into(),
                (Flag::Answered, _) => return None,
            },
            Self::Label(label) => format!("label:{}", words(label)?),
            // Folders are picked before searching.
            Self::In(_) => String::new(),
            Self::Before(at) => format!("before:{at}"),
            Self::After(at) => format!("after:{}", at.saturating_sub(1)),
            Self::Larger(bytes) => format!("larger:{bytes}"),
            Self::Smaller(bytes) => format!("smaller:{bytes}"),
            Self::Unsupported => return None,
        })
    }

    /// Whether the criterion has words to look for anywhere in a message,
    /// body included; only such searches are worth asking a server, as
    /// the headers of every message are here already.
    pub fn has_text(&self) -> bool {
        match self {
            Self::Text(Field::Any, _) => true,
            Self::And(items) | Self::Or(items) => items.iter().any(Self::has_text),
            _ => false,
        }
    }

    /// The `in:` folders the whole search is limited to, if any: the
    /// `In` parts of a top-level `And` (or an `In` alone).
    fn folders(&self) -> Vec<&str> {
        match self {
            Self::In(name) => vec![name.as_str()],
            Self::And(items) => items
                .iter()
                .filter_map(|item| match item {
                    Self::In(name) => Some(name.as_str()),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    /// Whether an `In` sits anywhere but where [`Self::folders`] reads
    /// it: inside an `Or` or a `Not`, which no server search can say.
    pub(crate) fn has_nested_in(&self) -> bool {
        fn inner(c: &Criterion) -> bool {
            match c {
                Criterion::In(_) => true,
                Criterion::And(items) | Criterion::Or(items) => items.iter().any(inner),
                Criterion::Not(c) => inner(c),
                _ => false,
            }
        }
        match self {
            Self::In(_) => false,
            Self::And(items) => items.iter().any(|item| match item {
                Self::In(_) => false,
                other => inner(other),
            }),
            other => inner(other),
        }
    }
}

/// Whether `folder` is the one `name` asks for: by path, last part of the
/// path or role, ignoring case, as the search box's `in:` does.
fn folder_matches(path: &str, role: Option<FolderRole>, name: &str) -> bool {
    let last = path.rsplit(['/', '.']).next().unwrap_or(path);
    path.eq_ignore_ascii_case(name)
        || last.eq_ignore_ascii_case(name)
        || role.is_some_and(|role| role.as_str().eq_ignore_ascii_case(name))
}

/// The folders of `account` to search for `criterion`, as `(id, path)`:
/// the ones holding mail not downloaded, most first, at most
/// [`MAX_FOLDERS`]. Trash and Junk only when `in:` names them; none when
/// an `in:` sits where no server search can say it.
pub fn folders(
    store: &Store,
    account: AccountId,
    criterion: &Criterion,
) -> katna_store::Result<Vec<(FolderId, String)>> {
    if criterion.has_nested_in() {
        return Ok(Vec::new());
    }
    let wanted = criterion.folders();
    let mut known: HashMap<FolderId, (String, Option<FolderRole>)> = store
        .folders(account)?
        .into_iter()
        .map(|f| (f.id, (f.path, f.role)))
        .collect();
    Ok(store
        .folders_not_downloaded(account)?
        .into_iter()
        .filter_map(|(id, _)| Some((id, known.remove(&id)?)))
        .filter(|(_, (path, role))| {
            if wanted.is_empty() {
                !matches!(role, Some(FolderRole::Trash | FolderRole::Junk))
            } else {
                wanted.iter().all(|name| folder_matches(path, *role, name))
            }
        })
        .take(MAX_FOLDERS)
        .map(|(id, (path, _))| (id, path))
        .collect())
}

/// Searches each of `folders` for `criterion`: the UIDs found in each.
/// `None` when the server cannot say the search.
pub async fn search<B: MailBackend>(
    backend: &mut B,
    folders: &[(FolderId, String)],
    criterion: &Criterion,
) -> Result<Option<Vec<(FolderId, Vec<u32>)>>> {
    let mut found = Vec::new();
    for (id, path) in folders {
        backend.select(path).await?;
        let Some(uids) = backend.search(criterion).await? else {
            return Ok(None);
        };
        if !uids.is_empty() {
            found.push((*id, uids));
        }
    }
    Ok(Some(found))
}

/// Of the messages `found` in their folders, the ones not downloaded,
/// newest first, at most `limit`.
pub fn not_downloaded(
    store: &Store,
    found: &[(FolderId, Vec<u32>)],
    limit: usize,
) -> katna_store::Result<Vec<MessageId>> {
    let mut seen = HashSet::new();
    let mut messages = Vec::new();
    for (folder, uids) in found {
        for (id, date) in store.not_downloaded_at(*folder, uids)? {
            if seen.insert(id) {
                messages.push((id, date));
            }
        }
    }
    messages.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.0.cmp(&a.0.0)));
    messages.truncate(limit);
    Ok(messages.into_iter().map(|(id, _)| id).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(field: Field, text: &str) -> Criterion {
        Criterion::Text(field, text.into())
    }

    #[test]
    fn gmail_syntax() {
        let query = Criterion::And(vec![
            text(Field::Any, "budget"),
            text(Field::From, "ada@example.org"),
            Criterion::Or(vec![
                text(Field::Subject, "q3 plan"),
                Criterion::HasAttachment,
            ]),
            Criterion::Not(Box::new(Criterion::Flag(Flag::Seen, true))),
            Criterion::After(1_700_000_000),
        ]);
        assert_eq!(
            query.gmail().as_deref(),
            Some(
                "(budget from:ada@example.org (subject:\"q3 plan\" OR has:attachment) \
                 -(is:read) after:1699999999)"
            )
        );
        // Quotes cannot break out of the phrase.
        assert_eq!(
            text(Field::Any, "say \"hi\" now").gmail().as_deref(),
            Some("\"say hi now\"")
        );
        // Non-ASCII goes through IMAP SEARCH instead.
        assert_eq!(text(Field::Any, "café").gmail(), None);
        assert_eq!(Criterion::Flag(Flag::Answered, true).gmail(), None);
        assert_eq!(
            Criterion::And(vec![text(Field::Any, "x"), Criterion::Unsupported]).gmail(),
            None
        );
    }

    #[test]
    fn text_and_folders() {
        let query = Criterion::And(vec![
            Criterion::In("Archive".into()),
            text(Field::Any, "budget"),
        ]);
        assert!(query.has_text());
        assert_eq!(query.folders(), ["Archive"]);
        assert!(!query.has_nested_in());
        assert!(!Criterion::Flag(Flag::Seen, false).has_text());
        assert!(!text(Field::From, "ada").has_text());
        let nested = Criterion::Or(vec![Criterion::In("x".into()), text(Field::Any, "y")]);
        assert!(nested.has_nested_in());
        assert!(folder_matches("INBOX/Projects", None, "projects"));
        assert!(folder_matches(
            "[Gmail]/Sent Mail",
            Some(FolderRole::Sent),
            "sent"
        ));
        assert!(!folder_matches("INBOX", Some(FolderRole::Inbox), "sent"));
    }
}
