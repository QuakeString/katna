// SPDX-License-Identifier: GPL-3.0-or-later

//! The outbox with a scripted SMTP server, filing in Sent on the in-memory
//! IMAP server.

mod common;

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Duration,
};

use common::{FakeServer, store as setup};
use futures_lite::FutureExt;
use katna_core::{AccountId, Paths};
use katna_store::{Mode, SendState, Store};
use katna_sync::{
    Error, MailSender, Result, engine, ops,
    outbox::{self, OutboxConfig, OutboxEvent, Outgoing},
};

const MESSAGE: &[u8] = b"From: Alice <alice@example.org>\r\nTo: bob@example.org\r\n\
    Bcc: carol@example.org,\r\n dave@example.org\r\nSubject: Lunch\r\n\r\nNoon?\r\n";

/// What the next connection does.
#[derive(Clone, Copy)]
enum Answer {
    Accept,
    Offline,
    Refuse,
}

/// One message as the SMTP server got it.
#[derive(Debug)]
struct Received {
    from: String,
    to: Vec<String>,
    message: String,
}

#[derive(Clone, Default)]
struct FakeSmtp {
    /// Answers for the next connections; accepts when empty.
    script: Arc<Mutex<VecDeque<Answer>>>,
    received: Arc<Mutex<Vec<Received>>>,
    gmail: bool,
}

struct FakeSender {
    answer: Answer,
    received: Arc<Mutex<Vec<Received>>>,
}

impl Outgoing for FakeSmtp {
    type Sender = FakeSender;

    async fn connect(&self, _account: AccountId) -> Result<FakeSender> {
        let answer = self
            .script
            .lock()
            .unwrap()
            .pop_front()
            .unwrap_or(Answer::Accept);
        if let Answer::Offline = answer {
            return Err(Error::Io(std::io::ErrorKind::ConnectionRefused.into()));
        }
        Ok(FakeSender {
            answer,
            received: self.received.clone(),
        })
    }

    fn files_sent_mail(&self, _account: AccountId) -> bool {
        self.gmail
    }
}

impl MailSender for FakeSender {
    async fn send(&mut self, from: &str, to: &[&str], message: Vec<u8>) -> Result<()> {
        if let Answer::Refuse = self.answer {
            return Err(Error::Rejected("550 no such user".into()));
        }
        self.received.lock().unwrap().push(Received {
            from: from.into(),
            to: to.iter().map(|to| to.to_string()).collect(),
            message: String::from_utf8(message).unwrap(),
        });
        Ok(())
    }

    async fn quit(self) -> Result<()> {
        Ok(())
    }
}

fn config(max_refusals: u32) -> OutboxConfig {
    OutboxConfig {
        retry_min: Duration::from_secs(1),
        retry_max: Duration::from_secs(1),
        max_refusals,
    }
}

fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

/// Runs the outbox until an entry reaches `until`, and returns every event.
fn run_until(
    smtp: &FakeSmtp,
    tmp: &tempfile::TempDir,
    config: OutboxConfig,
    until: SendState,
) -> Vec<OutboxEvent> {
    let store = Store::open(&Paths::with_root(tmp.path()), Mode::ReadWrite).unwrap();
    let (events_tx, events) = async_channel::unbounded::<OutboxEvent>();
    let (handle, control) = outbox::control();
    let collect = async {
        let mut seen = Vec::new();
        while let Ok(event) = events.recv().await {
            let done = event.state == until;
            seen.push(event);
            if done {
                break;
            }
        }
        seen
    };
    let runner = async {
        outbox::run(smtp.clone(), store, config, events_tx, control).await;
        Vec::new()
    };
    let timeout = async {
        async_io::Timer::after(Duration::from_secs(20)).await;
        panic!("no {until:?} event");
    };
    let seen = smol::block_on(collect.or(runner).or(timeout));
    drop(handle);
    seen
}

#[test]
fn sends_without_bcc_and_files_in_sent() {
    let (tmp, mut store, account) = setup();
    let server = FakeServer::default();
    server.create("INBOX", 1);
    server.create("Sent", 1);
    let mut conn = server.connection();
    smol::block_on(engine::sync_account(&mut conn, &mut store, account)).unwrap();

    let id = outbox::queue(&mut store, account, MESSAGE, 0, now()).unwrap();
    let entry = store.outbox_entry(id).unwrap().unwrap();
    assert_eq!(entry.subject, "Lunch");
    let smtp = FakeSmtp::default();
    let events = run_until(&smtp, &tmp, config(3), SendState::Sent);
    assert_eq!(
        events.iter().map(|e| e.state).collect::<Vec<_>>(),
        [SendState::Sending, SendState::Sent]
    );

    let received = smtp.received.lock().unwrap();
    assert_eq!(received.len(), 1);
    assert_eq!(received[0].from, "alice@example.org");
    assert_eq!(
        received[0].to,
        ["bob@example.org", "carol@example.org", "dave@example.org"]
    );
    let wire = &received[0].message;
    assert!(!wire.contains("Bcc") && !wire.contains("dave"), "{wire}");
    assert!(wire.starts_with("Date: ") && wire.contains("\r\nMessage-ID: <"));
    assert!(wire.ends_with("Subject: Lunch\r\n\r\nNoon?\r\n"), "{wire}");

    // The worker's replay files the stored copy, Bcc and all, as seen.
    assert_eq!(store.next_op_due(account).unwrap(), Some(0));
    let mut conn = server.connection();
    let report = smol::block_on(ops::replay(&mut conn, &mut store, account, now())).unwrap();
    assert_eq!(report.done, 1);
    assert!(server.log().contains(&"APPEND Sent \\Seen".to_owned()));
    let uid = server.uids("Sent")[0];
    let state = server.state();
    let filed = &state.folders["Sent"].messages[&uid];
    assert!(String::from_utf8_lossy(&filed.header).contains("dave@example.org"));
    drop(state);
    assert!(store.outbox().unwrap().is_empty());
    assert!(store.messages_by_id(&[entry.message]).unwrap().is_empty());
}

#[test]
fn gmail_files_sent_mail_itself() {
    let (tmp, mut store, account) = setup();
    let id = outbox::queue(&mut store, account, MESSAGE, 0, now()).unwrap();
    let message = store.outbox_entry(id).unwrap().unwrap().message;
    let smtp = FakeSmtp {
        gmail: true,
        ..FakeSmtp::default()
    };
    run_until(&smtp, &tmp, config(3), SendState::Sent);
    assert_eq!(store.next_op_due(account).unwrap(), None);
    assert!(store.outbox().unwrap().is_empty());
    assert!(store.messages_by_id(&[message]).unwrap().is_empty());
}

#[test]
fn undo_works_until_sending() {
    let (_tmp, mut store, account) = setup();
    let id = outbox::queue(&mut store, account, MESSAGE, 30, now()).unwrap();
    assert!(store.due_sends(now(), 10).unwrap().is_empty());
    assert!(outbox::cancel(&mut store, id).unwrap());
    assert!(!outbox::cancel(&mut store, id).unwrap());
    assert_eq!(
        store.outbox_entry(id).unwrap().unwrap().state,
        SendState::Cancelled
    );
    assert_eq!(store.next_send_at().unwrap(), None);
}

#[test]
fn offline_is_retried_without_counting() {
    let (tmp, mut store, account) = setup();
    let id = outbox::queue(&mut store, account, MESSAGE, 0, now()).unwrap();
    let smtp = FakeSmtp::default();
    smtp.script.lock().unwrap().push_back(Answer::Offline);
    let events = run_until(&smtp, &tmp, config(1), SendState::Sent);
    assert_eq!(events[1].state, SendState::Queued);
    assert!(events[1].detail.starts_with("offline"), "{:?}", events[1]);
    assert_eq!(events.last().unwrap().id, id);
    assert_eq!(smtp.received.lock().unwrap().len(), 1);
}

#[test]
fn refused_mail_fails_and_stays() {
    let (tmp, mut store, account) = setup();
    let id = outbox::queue(&mut store, account, MESSAGE, 0, now()).unwrap();
    let smtp = FakeSmtp::default();
    smtp.script
        .lock()
        .unwrap()
        .extend([Answer::Refuse, Answer::Refuse]);
    let events = run_until(&smtp, &tmp, config(2), SendState::Failed);
    assert_eq!(
        events.iter().map(|e| e.state).collect::<Vec<_>>(),
        [
            SendState::Sending,
            SendState::Queued,
            SendState::Sending,
            SendState::Failed
        ]
    );
    assert!(events[3].detail.contains("550"));
    let entry = store.outbox_entry(id).unwrap().unwrap();
    assert_eq!((entry.state, entry.attempts), (SendState::Failed, 2));
    assert!(smtp.received.lock().unwrap().is_empty());
}

#[test]
fn messages_without_recipients_are_refused() {
    let (_tmp, mut store, account) = setup();
    let error = outbox::queue(&mut store, account, b"From: a@x.org\r\n\r\nHi\r\n", 0, 0);
    assert!(matches!(error, Err(outbox::QueueError::Invalid(_))));
    assert!(store.outbox().unwrap().is_empty());
}
