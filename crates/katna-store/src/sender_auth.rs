// SPDX-License-Identifier: GPL-3.0-or-later

//! What the user's own mail provider said about each message's sender
//! (its `Authentication-Results` header), in `message.auth_results_json`.
//! The daemon writes it when a message arrives; a sender's picture is only
//! looked up for a domain whose mail passed DMARC or aligned DKIM, so a
//! forged `From` cannot make the daemon fetch anything
//! (`docs/ARCHITECTURE.md` §12).

use rusqlite::params;

use crate::Store;
use crate::error::Result;
use crate::mail::{MailBatch, MessageId};

impl MailBatch<'_> {
    /// Records the authentication verdict of `message` as JSON, such as
    /// `{"dmarc":"pass","aligned":true}`. `aligned` means the provider
    /// vouched for the `From` domain (DMARC pass, or a DKIM pass aligned
    /// with it).
    pub fn set_auth_results(&mut self, message: MessageId, json: &str) -> Result<()> {
        self.tx()
            .prepare_cached("UPDATE message SET auth_results_json = ?2 WHERE id = ?1")?
            .execute(params![message.0, json])?;
        Ok(())
    }
}

impl Store {
    /// Whether some message `From` an address at `domain` (lower case) was
    /// authenticated by the user's provider (`aligned` in
    /// [`MailBatch::set_auth_results`]).
    pub fn sender_domain_authenticated(&self, domain: &str) -> Result<bool> {
        Ok(self
            .mail
            .prepare_cached(
                "SELECT EXISTS (
                     SELECT 1 FROM participant p JOIN message m ON m.id = p.message_id
                     WHERE p.domain = ?1 AND p.role = 'from'
                       AND json_extract(m.auth_results_json, '$.aligned') = 1)",
            )?
            .query_row([domain], |row| row.get(0))?)
    }

    /// Whether the user's provider authenticated the `From` of `message`
    /// (`aligned`, as for [`Self::sender_domain_authenticated`]).
    pub fn message_authenticated(&self, message: MessageId) -> Result<bool> {
        Ok(self
            .mail
            .prepare_cached(
                "SELECT EXISTS (SELECT 1 FROM message
                     WHERE id = ?1 AND json_extract(auth_results_json, '$.aligned') = 1)",
            )?
            .query_row([message.0], |row| row.get(0))?)
    }
}

#[cfg(test)]
mod tests {
    use katna_core::{AccountKind, Paths};

    use crate::remote::{FolderRole, RemoteMessage};
    use crate::{Added, MessageFlags, Mode, NewParticipant, ParticipantRole, Store};

    #[test]
    fn only_authenticated_senders_count() {
        let tmp = tempfile::tempdir().unwrap();
        let mut store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
        let account = store
            .add_account(AccountKind::Imap, "Work", "ada@example.org")
            .unwrap()
            .id;
        let from = |email: &'static str, domain: &'static str| NewParticipant {
            role: ParticipantRole::From,
            email_norm: email,
            domain,
            display_name: None,
        };
        let signed = [from("news@shop.example", "shop.example")];
        let forged = [from("ceo@bank.example", "bank.example")];
        let mut ids = Vec::new();
        let mut batch = store.mail_batch().unwrap();
        let inbox = batch
            .upsert_folder(account, "INBOX", Some(FolderRole::Inbox))
            .unwrap();
        for (uid, participants, json) in [
            (1, &signed[..], Some(r#"{"dmarc":"pass","aligned":true}"#)),
            (2, &forged[..], Some(r#"{"dmarc":"fail","aligned":false}"#)),
            (3, &forged[..], None),
        ] {
            let message = RemoteMessage {
                uid,
                message_id_hdr: None,
                subject: Some("Hi"),
                date: Some(1_790_000_000 + i64::from(uid)),
                size: 10,
                flags: MessageFlags::empty(),
                keywords: &[],
                has_attachments: false,
                list_id: None,
                participants,
                in_reply_to: None,
                references: &[],
                gm_thread_id: None,
                gm_msgid: None,
                category: None,
                attachments: &[],
            };
            let Added::Message(id) = batch.add_remote_message(account, inbox, &message).unwrap()
            else {
                panic!("a new message");
            };
            ids.push(id);
            if let Some(json) = json {
                batch.set_auth_results(id, json).unwrap();
            }
        }
        batch.commit().unwrap();
        assert!(store.sender_domain_authenticated("shop.example").unwrap());
        assert!(!store.sender_domain_authenticated("bank.example").unwrap());
        assert!(!store.sender_domain_authenticated("other.example").unwrap());
        let authenticated: Vec<bool> = ids
            .iter()
            .map(|id| store.message_authenticated(*id).unwrap())
            .collect();
        assert_eq!(authenticated, [true, false, false]);
    }
}
