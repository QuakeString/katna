// SPDX-License-Identifier: GPL-3.0-or-later

//! Writes imported messages to katna-store.

use std::collections::HashMap;

use katna_core::{Account, AccountId, AccountKind};
use katna_store::{FolderId, MessageFlags, NewMessage, NewParticipant, Store};

use crate::{Added, Flags, IncomingMessage, MessageSink};

/// A [`MessageSink`] that stores messages in one account of a [`Store`].
pub struct StoreSink<'s> {
    store: &'s mut Store,
    account: AccountId,
}

impl<'s> StoreSink<'s> {
    pub fn new(store: &'s mut Store, account: AccountId) -> Self {
        Self { store, account }
    }

    /// Returns the local account named `name`, creating it if needed.
    /// Importing into the same name twice adds to the same account.
    pub fn local_account(store: &mut Store, name: &str) -> katna_store::Result<Account> {
        let existing = store
            .accounts()?
            .into_iter()
            .find(|account| account.kind == AccountKind::Local && account.display_name == name);
        match existing {
            Some(account) => Ok(account),
            None => store.add_account(AccountKind::Local, name, name),
        }
    }
}

impl MessageSink for StoreSink<'_> {
    type Error = katna_store::Error;

    fn write(&mut self, batch: &[IncomingMessage]) -> katna_store::Result<Vec<Added>> {
        let account = self.account;
        let mut mail = self.store.mail_batch()?;
        let mut folders: HashMap<&str, FolderId> = HashMap::new();
        let mut results = Vec::with_capacity(batch.len());
        let mut participants = Vec::new();
        for message in batch {
            let folder = match folders.get(message.folder.as_str()) {
                Some(&folder) => folder,
                None => {
                    let folder = mail.ensure_folder(account, &message.folder)?;
                    folders.insert(&message.folder, folder);
                    folder
                }
            };
            let parsed = &message.parsed;
            let references = parsed.reference_strs();
            participants.clear();
            participants.extend(parsed.participants.iter().map(|p| NewParticipant {
                role: p.role,
                email_norm: &p.email_norm,
                domain: &p.domain,
                display_name: p.display_name.as_deref(),
            }));
            let new = NewMessage {
                raw: &message.raw,
                message_id_hdr: parsed.message_id.as_deref(),
                subject: parsed.subject.as_deref(),
                date: parsed.date,
                flags: store_flags(message.flags),
                has_attachments: parsed.has_attachments,
                list_id: parsed.list_id.as_deref(),
                snippet: parsed.snippet.as_deref(),
                participants: &participants,
                in_reply_to: parsed.in_reply_to.as_deref(),
                references: &references,
                category: Some(parsed.category),
            };
            results.push(match mail.add_message(account, folder, &new)? {
                katna_store::Added::Message(_) => Added::New,
                katna_store::Added::Location(_) => Added::Copy,
                katna_store::Added::Duplicate(_) => Added::Duplicate,
            });
        }
        mail.commit()?;
        Ok(results)
    }
}

fn store_flags(flags: Flags) -> MessageFlags {
    let mut out = MessageFlags::empty();
    out.set(MessageFlags::SEEN, flags.seen);
    out.set(MessageFlags::ANSWERED, flags.answered);
    out.set(MessageFlags::FLAGGED, flags.flagged);
    out.set(MessageFlags::DRAFT, flags.draft);
    out.set(MessageFlags::DELETED, flags.deleted);
    out.set(MessageFlags::FORWARDED, flags.forwarded);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Options, import_maildir};
    use katna_core::Paths;
    use katna_store::{ChangeOp, DbKind, Mode, ObjectKind};
    use std::fs;

    #[test]
    fn imports_an_enron_tree_into_the_store() {
        let corpus = tempfile::tempdir().unwrap();
        let write = |rel: &str, body: &str| {
            let path = corpus.path().join(rel);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, body).unwrap();
        };
        let memo = "Message-ID: <1@thyme>\r\nFrom: kenneth.lay@enron.com\r\n\
                    To: all.worldwide@enron.com\r\nSubject: budget\r\n\r\nMemo\r\n";
        write("lay-k/inbox/1.", memo);
        write("lay-k/all_documents/1.", memo);
        write(
            "lay-k/inbox/2.",
            "From: jeff.skilling@enron.com\r\nSubject: re\r\n\r\nOk\r\n",
        );

        let data = tempfile::tempdir().unwrap();
        let paths = Paths::with_root(data.path());
        let mut store = Store::open(&paths, Mode::ReadWrite).unwrap();
        let account = StoreSink::local_account(&mut store, "enron").unwrap();
        assert_eq!(
            StoreSink::local_account(&mut store, "enron").unwrap(),
            account
        );

        let options = Options {
            batch_size: 2,
            ..Options::default()
        };
        let mut sink = StoreSink::new(&mut store, account.id);
        let stats = import_maildir(corpus.path(), &mut sink, &options, |_| {}).unwrap();
        assert_eq!((stats.imported, stats.copies, stats.duplicates), (2, 1, 0));
        assert!(stats.skipped.is_empty());

        let again = import_maildir(corpus.path(), &mut sink, &options, |_| {}).unwrap();
        assert_eq!((again.imported, again.copies, again.duplicates), (0, 0, 3));

        let reader = Store::open(&paths, Mode::ReadOnly).unwrap();
        let changes: Vec<_> = reader
            .changes_since(DbKind::Mail, 0, 100)
            .unwrap()
            .iter()
            .map(|c| (c.kind, c.op))
            .collect();
        assert_eq!(
            changes,
            [
                // lay-k/all_documents/1.
                (ObjectKind::Folder, ChangeOp::Insert),
                (ObjectKind::Thread, ChangeOp::Insert),
                (ObjectKind::Message, ChangeOp::Insert),
                // lay-k/inbox/1. is the same memo; lay-k/inbox/2. is new.
                (ObjectKind::Folder, ChangeOp::Insert),
                (ObjectKind::Message, ChangeOp::Update),
                (ObjectKind::Thread, ChangeOp::Insert),
                (ObjectKind::Message, ChangeOp::Insert),
            ]
        );
        let hash = katna_store::BlobHash::of(memo.as_bytes());
        assert_eq!(reader.blobs().get(&hash).unwrap().unwrap(), memo.as_bytes());
    }

    #[test]
    fn maps_every_flag() {
        let all = Flags {
            seen: true,
            answered: true,
            flagged: true,
            draft: true,
            deleted: true,
            forwarded: true,
        };
        assert_eq!(store_flags(all).bits(), 0b11_1111);
        assert_eq!(store_flags(Flags::default()), MessageFlags::empty());
    }
}
