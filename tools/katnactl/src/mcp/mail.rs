// SPDX-License-Identifier: GPL-3.0-or-later

//! The tools of `katnactl mcp`, on the store and the search index (both
//! read-only) and the daemon.

use std::collections::HashMap;

use katna_core::{AccountId, Paths, mime::Mailbox};
use katna_dbus::PimProxy;
use katna_search::{Query, SearchIndex, SearchOptions, Sort};
use katna_store::{
    MessageFlags, MessageId, Mode, ParticipantRole, Store, StoredMessage, StoredParticipant,
};
use serde_json::{Value, json};

use super::Tools;
use super::draft::{self, Draft};

/// The most results one search gives.
pub const MAX_RESULTS: usize = 50;
/// The most of one message's text `read_message` gives, in characters.
const MAX_TEXT: usize = 60_000;
/// The most of each message's text `read_conversation` gives.
const MAX_TEXT_IN_CONVERSATION: usize = 8_000;
/// The newest messages of a long conversation `read_conversation` gives.
const MAX_CONVERSATION: usize = 30;
/// Characters of text around the words found, in search results.
const SNIPPET_CHARS: usize = 200;

/// The mail of this computer's Katna.
pub struct Mail {
    paths: Paths,
    index: Option<SearchIndex>,
}

type Outcome = Result<Value, String>;

impl Tools for Mail {
    fn call(&mut self, name: &str, arguments: &Value) -> Option<Outcome> {
        Some(match name {
            "list_accounts" => self.list_accounts(),
            "list_folders" => self.list_folders(arguments),
            "search_mail" => self.search(arguments),
            "read_message" => self.read_message(arguments),
            "read_conversation" => self.read_conversation(arguments),
            "create_draft" => self.create_draft(arguments),
            _ => return None,
        })
    }
}

impl Mail {
    pub fn open() -> crate::Result<Self> {
        let paths = Paths::from_env().map_err(crate::error)?;
        Ok(Self { paths, index: None })
    }

    fn store(&self) -> Result<Store, String> {
        Store::open(&self.paths, Mode::ReadOnly).map_err(|err| {
            format!("Katna's mail could not be opened ({err}); is Katna set up on this computer?")
        })
    }

    fn list_accounts(&self) -> Outcome {
        let store = self.store()?;
        let accounts: Vec<Value> = mail_accounts(&store)?
            .into_iter()
            .map(|a| {
                json!({
                    "account": a.id.0,
                    "name": a.display_name,
                    "address": a.address,
                    "kind": a.kind.as_str(),
                })
            })
            .collect();
        Ok(json!({ "accounts": accounts }))
    }

    fn list_folders(&self, arguments: &Value) -> Outcome {
        let store = self.store()?;
        let only = optional_integer(arguments, "account")?;
        let accounts = mail_accounts(&store)?;
        if let Some(only) = only
            && !accounts.iter().any(|a| a.id.0 == only)
        {
            return Err(format!(
                "There is no mail account {only}; see list_accounts."
            ));
        }
        let folders: Vec<Value> = store
            .folder_summaries()
            .map_err(read_failed)?
            .into_iter()
            .filter(|f| only.is_none_or(|only| f.account.0 == only))
            .map(|f| {
                json!({
                    "account": f.account.0,
                    "path": f.path,
                    "role": f.role,
                    "messages": f.total,
                })
            })
            .collect();
        Ok(json!({ "folders": folders }))
    }

    fn search(&mut self, arguments: &Value) -> Outcome {
        let text = optional_string(arguments, "query")?.unwrap_or_default();
        let limit = optional_integer(arguments, "limit")?
            .unwrap_or(20)
            .clamp(1, MAX_RESULTS as i64) as usize;
        let offset = optional_integer(arguments, "offset")?.unwrap_or(0).max(0) as usize;
        let sort = match optional_string(arguments, "sort")?.as_deref() {
            None | Some("auto") => Sort::Auto,
            Some("relevance") => Sort::Relevance,
            Some("newest") => Sort::Newest,
            Some("oldest") => Sort::Oldest,
            Some(other) => {
                return Err(format!(
                    "sort {other:?} is not auto, relevance, newest or oldest"
                ));
            }
        };
        let query = Query::parse(&text).map_err(|err| format!("The query has a mistake: {err}"))?;
        let store = self.store()?;
        let index = self.index()?;
        let options = SearchOptions {
            limit,
            offset,
            sort,
            count: true,
        };
        let results = index
            .search(&query, &options)
            .map_err(|err| format!("Searching failed: {err}"))?;
        let ids: Vec<MessageId> = results.hits.iter().map(|hit| hit.message).collect();
        let messages = store.messages_by_id(&ids).map_err(read_failed)?;
        let snippets = index
            .snippets(&store, &query, &ids, SNIPPET_CHARS)
            .map_err(|err| format!("Searching failed: {err}"))?;
        let found: Vec<Value> = messages
            .iter()
            .zip(&snippets)
            .map(|(message, snippet)| {
                let mut summary = summary(message);
                let text = if snippet.text.trim().is_empty() {
                    message.snippet.clone().unwrap_or_default()
                } else {
                    snippet.text.clone()
                };
                summary["snippet"] = json!(one_line(&text));
                summary
            })
            .collect();
        let mut answer = json!({ "messages": found });
        if let Some(total) = results.total {
            answer["total"] = json!(total);
        }
        if results.fuzzy {
            answer["note"] =
                json!("Nothing matched the words as written; these match similar words.");
        }
        Ok(answer)
    }

    /// The search index, opened once and brought up to date each search.
    fn index(&mut self) -> Result<&SearchIndex, String> {
        let not_ready = |err: katna_search::Error| match err {
            katna_search::Error::NotFound(_) => {
                "Search is not ready: Katna has not built its search index yet.".to_owned()
            }
            err => format!("Search is not ready: {err}"),
        };
        match &mut self.index {
            Some(index) => index.reload().map_err(not_ready)?,
            None => {
                self.index =
                    Some(SearchIndex::open_read_only(&self.paths.index_dir()).map_err(not_ready)?);
            }
        }
        Ok(self.index.as_ref().expect("opened above"))
    }

    fn read_message(&self, arguments: &Value) -> Outcome {
        let id = MessageId(integer(arguments, "id")?);
        let store = self.store()?;
        let message = one_message(&store, id)?;
        let raw = self.raw(&[&message])?;
        let mut answer = summary(&message);
        add_text(&mut answer, raw.get(&id).map(Vec::as_slice), MAX_TEXT);
        Ok(answer)
    }

    fn read_conversation(&self, arguments: &Value) -> Outcome {
        let id = MessageId(integer(arguments, "id")?);
        let store = self.store()?;
        let message = one_message(&store, id)?;
        let ids = match message.thread_id {
            Some(thread) => store.thread_messages(thread).map_err(read_failed)?,
            None => vec![id],
        };
        let left_out = ids.len().saturating_sub(MAX_CONVERSATION);
        let messages = store
            .messages_by_id(&ids[left_out..])
            .map_err(read_failed)?;
        let raw = self.raw(&messages.iter().collect::<Vec<_>>())?;
        let messages: Vec<Value> = messages
            .iter()
            .map(|message| {
                let mut entry = summary(message);
                add_text(
                    &mut entry,
                    raw.get(&message.id).map(Vec::as_slice),
                    MAX_TEXT_IN_CONVERSATION,
                );
                entry
            })
            .collect();
        let mut answer = json!({
            "conversation": message.thread_id.map(|t| t.0),
            "subject": message.subject,
            "messages": messages,
        });
        if left_out > 0 {
            answer["earlier_messages_left_out"] = json!(left_out);
        }
        Ok(answer)
    }

    /// The stored messages of `messages`, asking the daemon to download
    /// the ones not stored yet.
    fn raw(&self, messages: &[&StoredMessage]) -> Result<HashMap<MessageId, Vec<u8>>, String> {
        let read = |store: &Store, message: &StoredMessage| -> Option<Vec<u8>> {
            store.blobs().get(message.blob_hash.as_ref()?).ok()?
        };
        let store = self.store()?;
        let mut out = HashMap::new();
        let mut missing = Vec::new();
        for message in messages {
            match read(&store, message) {
                Some(raw) => {
                    out.insert(message.id, raw);
                }
                None => missing.push(message.id),
            }
        }
        if missing.is_empty() {
            return Ok(out);
        }
        // Downloaded by the daemon, which only Katna lets talk to servers.
        let fetched = with_daemon(|pim| async move {
            for id in &missing {
                pim.fetch_body(id.0).await?;
            }
            Ok(missing)
        });
        if let Ok(fetched) = fetched {
            let store = self.store()?;
            for message in store.messages_by_id(&fetched).map_err(read_failed)? {
                if let Some(raw) = read(&store, &message) {
                    out.insert(message.id, raw);
                }
            }
        }
        Ok(out)
    }

    fn create_draft(&self, arguments: &Value) -> Outcome {
        let body = optional_string(arguments, "body")?.ok_or("create_draft needs a body")?;
        let field = |name: &str| -> Result<Vec<Mailbox>, String> {
            draft::addresses(&optional_string(arguments, name)?.unwrap_or_default())
                .map_err(|err| format!("{name}: {err}"))
        };
        let mut draft = Draft {
            to: field("to")?,
            cc: field("cc")?,
            bcc: field("bcc")?,
            subject: optional_string(arguments, "subject")?.unwrap_or_default(),
            body,
            ..Draft::default()
        };
        let store = self.store()?;
        let accounts = mail_accounts(&store)?;
        let mut account = optional_integer(arguments, "account")?.map(AccountId);
        if let Some(answered) = optional_integer(arguments, "reply_to")? {
            let message = one_message(&store, MessageId(answered))?;
            let raw = self.raw(&[&message])?;
            let original = raw
                .get(&message.id)
                .and_then(|raw| draft::original(raw))
                .ok_or_else(|| {
                    format!("Message {answered} could not be read to answer it; is Katna's service running?")
                })?;
            draft.answer(&original);
            account.get_or_insert(message.account);
        }
        let account = match (account, accounts.as_slice()) {
            (Some(id), _) => accounts
                .iter()
                .find(|a| a.id == id)
                .ok_or_else(|| format!("There is no mail account {id}; see list_accounts."))?,
            (None, [only]) => only,
            (None, []) => return Err("Katna has no mail account yet.".to_owned()),
            (None, _) => {
                return Err(
                    "Say which account the draft is from (account; see list_accounts).".to_owned(),
                );
            }
        };
        if draft.to.is_empty()
            && draft.cc.is_empty()
            && draft.bcc.is_empty()
            && draft.subject.trim().is_empty()
        {
            return Err("A draft needs someone to send it to or a subject.".to_owned());
        }
        draft.from = Some(Mailbox {
            name: Some(account.display_name.trim().to_owned()).filter(|n| !n.is_empty()),
            email: account.address.clone(),
        });
        let raw = draft.build();
        let id = account.id.0;
        let saved = with_daemon(|pim| async move { pim.save_draft(id, &raw).await })
            .map_err(|err| format!("The draft was not saved: {err}"))?;
        Ok(json!({
            "draft": saved,
            "account": id,
            "from": account.address,
            "saved_in": "Drafts",
            "sent": false,
            "note": "Saved as a draft only. The person opens it in Katna Mail's Drafts to check and send it.",
        }))
    }
}

/// The accounts that carry mail.
fn mail_accounts(store: &Store) -> Result<Vec<katna_core::account::Account>, String> {
    Ok(store
        .accounts()
        .map_err(read_failed)?
        .into_iter()
        .filter(|a| a.kind.is_mail())
        .collect())
}

fn one_message(store: &Store, id: MessageId) -> Result<StoredMessage, String> {
    store
        .messages_by_id(&[id])
        .map_err(read_failed)?
        .pop()
        .ok_or_else(|| {
            format!(
                "There is no message {}; message numbers come from search_mail.",
                id.0
            )
        })
}

fn read_failed(err: katna_store::Error) -> String {
    format!("Reading Katna's mail failed: {err}")
}

/// What a list shows of `message`.
fn summary(message: &StoredMessage) -> Value {
    let people = |role: ParticipantRole| -> Vec<Value> {
        message
            .participants
            .iter()
            .filter(|p| p.role == role)
            .map(person)
            .collect()
    };
    let flag = |flag: MessageFlags| message.flags.contains(flag);
    let mut out = json!({
        "id": message.id.0,
        "account": message.account.0,
        "date": message.date.map(iso_time),
        "from": message.first(ParticipantRole::From).map(person),
        "to": people(ParticipantRole::To),
        "subject": message.subject,
        "folders": message.locations.iter().map(|l| l.path.as_str()).collect::<Vec<_>>(),
        "unread": !flag(MessageFlags::SEEN),
        "starred": flag(MessageFlags::FLAGGED),
        "has_attachments": message.has_attachments,
        "conversation": message.thread_id.map(|t| t.0),
    });
    let cc = people(ParticipantRole::Cc);
    if !cc.is_empty() {
        out["cc"] = json!(cc);
    }
    for (name, flag) in [
        ("important", MessageFlags::IMPORTANT),
        ("answered", MessageFlags::ANSWERED),
        ("draft", MessageFlags::DRAFT),
    ] {
        if message.flags.contains(flag) {
            out[name] = json!(true);
        }
    }
    let labels: Vec<&str> = message
        .keywords
        .iter()
        .map(String::as_str)
        .filter(|k| !k.starts_with('\\') && !k.starts_with('$'))
        .collect();
    if !labels.is_empty() {
        out["labels"] = json!(labels);
    }
    out
}

fn person(p: &StoredParticipant) -> Value {
    match p
        .display_name
        .as_deref()
        .map(str::trim)
        .filter(|n| !n.is_empty())
    {
        Some(name) => json!({ "name": name, "address": p.email_norm }),
        None => json!({ "address": p.email_norm }),
    }
}

/// Adds the text and attachment names of `raw` to `entry`, at most `max`
/// characters of text.
fn add_text(entry: &mut Value, raw: Option<&[u8]>, max: usize) {
    let Some(raw) = raw else {
        entry["text"] = json!(null);
        entry["note"] = json!("Not downloaded yet, and Katna's service could not download it now.");
        return;
    };
    let text = katna_search::document::message_text(raw);
    let body = text.body.replace("\r\n", "\n");
    let body = body.trim();
    match body.char_indices().nth(max) {
        Some((at, _)) => {
            entry["text"] = json!(&body[..at]);
            entry["text_cut_short"] = json!(true);
        }
        None => entry["text"] = json!(body),
    }
    if !text.attachment_names.is_empty() {
        entry["attachments"] = json!(text.attachment_names);
    }
}

fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Unix seconds as `YYYY-MM-DDTHH:MM:SSZ`.
fn iso_time(unix: i64) -> String {
    let (year, month, day) = crate::civil_from_days(unix.div_euclid(86_400));
    let seconds = unix.rem_euclid(86_400);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        seconds / 3600,
        seconds % 3600 / 60,
        seconds % 60
    )
}

fn integer(arguments: &Value, name: &str) -> Result<i64, String> {
    optional_integer(arguments, name)?.ok_or_else(|| format!("{name} is needed"))
}

fn optional_integer(arguments: &Value, name: &str) -> Result<Option<i64>, String> {
    match arguments.get(name) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => value
            .as_i64()
            // Some assistants send numbers as strings.
            .or_else(|| value.as_str().and_then(|s| s.trim().parse().ok()))
            .map(Some)
            .ok_or_else(|| format!("{name} must be a whole number")),
    }
}

fn optional_string(arguments: &Value, name: &str) -> Result<Option<String>, String> {
    match arguments.get(name) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(text)) => Ok(Some(text.clone())),
        Some(_) => Err(format!("{name} must be text")),
    }
}

/// Runs `f` with the daemon, starting it if needed.
fn with_daemon<T, F, Fut>(f: F) -> Result<T, String>
where
    F: FnOnce(PimProxy<'static>) -> Fut,
    Fut: Future<Output = zbus::Result<T>>,
{
    futures_lite::future::block_on(async {
        let connection = katna_dbus::session()
            .await
            .map_err(|err| format!("Katna's service could not be reached ({err})."))?;
        katna_dbus::ensure_daemon(&connection).await;
        let pim = PimProxy::new(&connection)
            .await
            .map_err(|err| err.to_string())?;
        f(pim).await.map_err(|err| match err {
            zbus::Error::MethodError(_, Some(message), _) => message,
            err => err.to_string(),
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn times_are_utc() {
        assert_eq!(iso_time(0), "1970-01-01T00:00:00Z");
        assert_eq!(iso_time(1_791_335_914), "2026-10-07T01:18:34Z");
    }

    #[test]
    fn numbers_may_come_as_text() {
        let arguments = json!({ "a": 3, "b": "4", "c": "x", "d": null });
        assert_eq!(optional_integer(&arguments, "a"), Ok(Some(3)));
        assert_eq!(optional_integer(&arguments, "b"), Ok(Some(4)));
        assert!(optional_integer(&arguments, "c").is_err());
        assert_eq!(optional_integer(&arguments, "d"), Ok(None));
        assert_eq!(optional_integer(&arguments, "e"), Ok(None));
    }

    #[test]
    fn long_text_is_cut_on_a_character() {
        let raw = format!(
            "Subject: x\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n{}\r\n",
            "é".repeat(20)
        );
        let mut entry = json!({});
        add_text(&mut entry, Some(raw.as_bytes()), 5);
        assert_eq!(entry["text"], "ééééé");
        assert_eq!(entry["text_cut_short"], true);
    }
}
