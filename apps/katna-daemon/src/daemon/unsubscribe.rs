// SPDX-License-Identifier: GPL-3.0-or-later

//! Unsubscribing in the background (`docs/ARCHITECTURE.md` §9.5): the
//! sender's own way, from this computer. A one-click POST to the
//! `List-Unsubscribe` address when `List-Unsubscribe-Post` offers it
//! (RFC 8058), else the list's unsubscribe mail, sent from the account
//! the mail came to. Web pages are never followed: a list that only has
//! one is left to the browser.

use std::time::Duration;

use katna_core::mime::Mailbox;
use katna_core::unsubscribe::{ONE_CLICK_BODY, Ways, list_headers};
use katna_dbus::unsubscribed;
use katna_store::MessageId;
use katna_sync::{autoconfig::http, net::Tls};

use super::{CommandError, Daemon};

/// How long a list's address has to answer.
const TIMEOUT: Duration = Duration::from_secs(20);

/// At most this many messages in one call.
const MAX_MESSAGES: usize = 200;

/// What one message asks for.
struct Request {
    id: MessageId,
    account: katna_core::AccountId,
    /// The `List-Unsubscribe` value: one list, asked once.
    list: String,
    ways: Ways,
}

impl Daemon {
    /// Unsubscribes from the lists of `messages`; see `Pim1.Unsubscribe`.
    pub async fn unsubscribe(
        &self,
        messages: &[MessageId],
    ) -> Result<Vec<(i64, String)>, CommandError> {
        if messages.len() > MAX_MESSAGES {
            return Err(CommandError::InvalidArgs(format!(
                "at most {MAX_MESSAGES} messages"
            )));
        }
        let requests = self.unsubscribe_requests(messages)?;
        let mut out: Vec<(i64, String)> = Vec::new();
        let mut asked: Vec<(String, &'static str)> = Vec::new();
        let mut tls = None;
        for request in &requests {
            let outcome = match asked.iter().find(|(list, _)| *list == request.list) {
                Some((_, outcome)) => *outcome,
                None => {
                    let outcome = self.unsubscribe_one(request, &mut tls).await;
                    asked.push((request.list.clone(), outcome));
                    outcome
                }
            };
            out.push((request.id.0, outcome.to_owned()));
        }
        for id in messages {
            if !out.iter().any(|(m, _)| *m == id.0) {
                out.push((id.0, unsubscribed::NONE.to_owned()));
            }
        }
        Ok(out)
    }

    /// Reads what each message says.
    fn unsubscribe_requests(&self, messages: &[MessageId]) -> Result<Vec<Request>, CommandError> {
        let store = self.store();
        let mut requests = Vec::new();
        for message in store.messages_by_id(messages)? {
            let Some(hash) = message.blob_hash else {
                continue;
            };
            let Some(raw) = store.blobs().get(&hash)? else {
                continue;
            };
            let (header, post) = list_headers(&raw);
            let Some(list) = header.clone() else {
                continue;
            };
            requests.push(Request {
                id: message.id,
                account: message.account,
                list,
                ways: Ways::read(header.as_deref(), post.as_deref(), None),
            });
        }
        Ok(requests)
    }

    /// Unsubscribes as `request` asks: the one-click POST, else the mail.
    async fn unsubscribe_one(&self, request: &Request, tls: &mut Option<Tls>) -> &'static str {
        if let Some(url) = &request.ways.one_click {
            if tls.is_none() {
                *tls = Tls::system()
                    .map_err(|err| tracing::warn!(%err, "TLS setup failed"))
                    .ok();
            }
            if let Some(tls) = tls.as_ref() {
                let headers = [("Content-Type", "application/x-www-form-urlencoded")];
                match http::post_public(url, &headers, ONE_CLICK_BODY.as_bytes(), tls, TIMEOUT)
                    .await
                {
                    // A redirect still took the request.
                    Ok(status) if status < 400 => {
                        tracing::info!(message = request.id.0, status, "unsubscribed");
                        return unsubscribed::DONE;
                    }
                    Ok(status) => {
                        tracing::warn!(message = request.id.0, status, "unsubscribe refused");
                    }
                    Err(err) => {
                        tracing::warn!(message = request.id.0, %err, "unsubscribe failed");
                    }
                }
            }
            // The list's mail may still work.
            if request.ways.mail.is_none() {
                return unsubscribed::FAILED;
            }
        }
        let Some(mail) = &request.ways.mail else {
            return unsubscribed::NONE;
        };
        match self.unsubscribe_by_mail(request, mail) {
            Ok(()) => unsubscribed::SENT,
            Err(err) => {
                tracing::warn!(message = request.id.0, %err, "unsubscribe mail not queued");
                unsubscribed::FAILED
            }
        }
    }

    fn unsubscribe_by_mail(
        &self,
        request: &Request,
        mail: &katna_core::mailto::Mailto,
    ) -> Result<(), CommandError> {
        let account = self
            .store()
            .accounts()?
            .into_iter()
            .find(|a| a.id == request.account)
            .ok_or(CommandError::UnknownAccount(request.account.0))?;
        let from = Mailbox {
            name: Some(account.display_name.clone()).filter(|n| !n.is_empty()),
            email: account.address.clone(),
        };
        let raw = katna_core::unsubscribe::message(mail, &from);
        self.queue_send(request.account, &raw, 0)?;
        Ok(())
    }
}
