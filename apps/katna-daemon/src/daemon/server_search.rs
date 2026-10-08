// SPDX-License-Identifier: GPL-3.0-or-later

//! "More results on server" (`docs/ARCHITECTURE.md` §7.3): a search box
//! query asked of the IMAP servers, for mail that is not downloaded and so
//! not in the search index by its text. Gmail gets its own `X-GM-RAW`
//! syntax, other servers IMAP `SEARCH`. JMAP accounts come with JMAP sync.

use std::sync::Arc;

use katna_core::{AccountId, AccountKind};
use katna_search::{Filter, Query, TextField};
use katna_store::{MessageFlags, MessageId, Mode, Store};
use katna_sync::server_search::{self, Criterion, Field, Flag};

use super::{CommandError, Daemon, unix_now};
use crate::on_demand::Session;

/// At most this many messages are found per search.
pub const LIMIT: usize = 100;

impl Daemon {
    /// Searches the servers of `account` (of every account when `None`)
    /// for `text`, a search box query; the messages found that are not
    /// downloaded, newest first. Empty for a query with no words to look
    /// for, and for accounts that are offline or have every message
    /// downloaded.
    pub async fn search_server(
        self: &Arc<Self>,
        text: &str,
        account: Option<AccountId>,
    ) -> Result<Vec<MessageId>, CommandError> {
        let query = Query::parse_at(text, unix_now())
            .map_err(|err| CommandError::Failed(err.to_string()))?;
        let criterion = criterion(&query);
        if !criterion.has_text() {
            return Ok(Vec::new());
        }
        let accounts: Vec<AccountId> = {
            let store = self.store();
            store
                .accounts()?
                .into_iter()
                .filter(|a| a.kind == AccountKind::Imap)
                .filter(|a| account.is_none_or(|id| id == a.id))
                .map(|a| a.id)
                .filter(|&id| store.has_mail_not_downloaded(Some(id)).unwrap_or(false))
                .collect()
        };
        let mut found = Vec::new();
        let mut failed = None;
        for account in accounts {
            if self.is_offline(account) {
                continue;
            }
            let session = self.on_demand.session(account);
            match self.search_account(account, &session, &criterion).await {
                Ok(ids) => found.extend(ids),
                Err(err) => {
                    tracing::info!(%account, %err, "searching the server");
                    failed = Some(err);
                }
            }
            self.on_demand.used(account, &session);
        }
        // One server that answered is enough; all failing is an error.
        if found.is_empty()
            && let Some(err) = failed
        {
            return Err(err);
        }
        let store = Store::open(&self.paths, Mode::ReadOnly)?;
        let mut dated: Vec<(MessageId, Option<i64>)> = store
            .messages_by_id(&found)?
            .into_iter()
            .map(|m| (m.id, m.date))
            .collect();
        dated.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.0.cmp(&a.0.0)));
        dated.truncate(LIMIT);
        Ok(dated.into_iter().map(|(id, _)| id).collect())
    }

    async fn search_account(
        self: &Arc<Self>,
        account: AccountId,
        session: &Session,
        criterion: &Criterion,
    ) -> Result<Vec<MessageId>, CommandError> {
        let folders = {
            let store = Store::open(&self.paths, Mode::ReadOnly)?;
            server_search::folders(&store, account, criterion)?
        };
        if folders.is_empty() {
            return Ok(Vec::new());
        }
        let mut connection = session.connection.lock().await;
        // A kept connection may have been closed by the server meanwhile:
        // then once more on a new one.
        let mut result = Ok(None);
        for _ in 0..2 {
            let fresh = connection
                .as_ref()
                .is_none_or(katna_sync::connection::Connection::is_closed);
            if fresh {
                *connection = Some(self.connect_on_demand(account).await?);
            }
            let mut handle = connection.clone().expect("connected above");
            result = server_search::search(&mut handle, &folders, criterion).await;
            if !result.as_ref().is_err_and(katna_sync::Error::is_fatal) {
                break;
            }
            *connection = None;
            if fresh {
                break;
            }
        }
        drop(connection);
        let found = result
            .map_err(|err| CommandError::Failed(format!("the mail server did not search ({err})")))?
            .unwrap_or_default();
        let store = Store::open(&self.paths, Mode::ReadOnly)?;
        Ok(server_search::not_downloaded(&store, &found, LIMIT)?)
    }
}

/// A search box query in the terms servers search by.
pub fn criterion(query: &Query) -> Criterion {
    let field = |field: TextField| match field {
        TextField::Any => Field::Any,
        TextField::From => Field::From,
        TextField::To => Field::To,
        TextField::Cc => Field::Cc,
        TextField::Bcc => Field::Bcc,
        TextField::Subject => Field::Subject,
        TextField::Filename => Field::Filename,
        TextField::List => Field::List,
    };
    match query {
        Query::All => Criterion::All,
        Query::And(items) => Criterion::And(items.iter().map(criterion).collect()),
        Query::Or(items) => Criterion::Or(items.iter().map(criterion).collect()),
        Query::Not(inner) => Criterion::Not(Box::new(criterion(inner))),
        // Servers match words wherever they start; a word still being
        // typed is searched as it stands.
        Query::Text { field: f, text } | Query::Prefix { field: f, text } => {
            Criterion::Text(field(*f), text.clone())
        }
        Query::Filter(filter) => match filter {
            Filter::HasAttachment => Criterion::HasAttachment,
            Filter::Flag(flag, on) => {
                let flag = match *flag {
                    MessageFlags::SEEN => Flag::Seen,
                    MessageFlags::ANSWERED => Flag::Answered,
                    MessageFlags::FLAGGED => Flag::Flagged,
                    MessageFlags::DRAFT => Flag::Draft,
                    _ => return Criterion::Unsupported,
                };
                Criterion::Flag(flag, *on)
            }
            Filter::In(folder) => Criterion::In(folder.clone()),
            Filter::Label(label) => Criterion::Label(label.clone()),
            Filter::Org(domain) => {
                let at = format!("@{domain}");
                Criterion::Or(vec![
                    Criterion::Text(Field::From, at.clone()),
                    Criterion::Text(Field::To, at.clone()),
                    Criterion::Text(Field::Cc, at),
                ])
            }
            Filter::Before(at) => Criterion::Before(*at),
            Filter::After(at) => Criterion::After(*at),
            Filter::Larger(bytes) => Criterion::Larger(*bytes),
            Filter::Smaller(bytes) => Criterion::Smaller(*bytes),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queries_become_criteria() {
        let query = Query::parse_at("budget from:ada has:attachment is:unread", 0).unwrap();
        assert_eq!(
            criterion(&query),
            Criterion::And(vec![
                Criterion::Text(Field::Any, "budget".into()),
                Criterion::Text(Field::From, "ada".into()),
                Criterion::HasAttachment,
                Criterion::Flag(Flag::Seen, false),
            ])
        );
        let query = Query::parse_at("budget OR plan", 0).unwrap();
        assert!(criterion(&query).has_text());
        // Only operators: the headers they look at are all here already.
        let query = Query::parse_at("from:ada subject:plan", 0).unwrap();
        assert!(!criterion(&query).has_text());
        let query = Query::parse_at("is:unread", 0).unwrap();
        assert!(!criterion(&query).has_text());
    }
}
