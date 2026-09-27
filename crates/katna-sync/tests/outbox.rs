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
    Error, MailSender, Result, engine,
    net::Tls,
    ops,
    outbox::{self, OutboxConfig, OutboxEvent, Outgoing},
    tracking,
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
    /// `HOLDUNTIL`, for mail the server holds.
    held_until: Option<i64>,
}

#[derive(Clone, Default)]
struct FakeSmtp {
    /// Answers for the next connections; accepts when empty.
    script: Arc<Mutex<VecDeque<Answer>>>,
    received: Arc<Mutex<Vec<Received>>>,
    gmail: bool,
    /// `FUTURERELEASE`'s longest hold.
    hold: Option<u64>,
    /// The tracking server and this install's token, when tracking is on.
    tracking: Option<(String, String)>,
}

struct FakeSender {
    answer: Answer,
    received: Arc<Mutex<Vec<Received>>>,
    hold: Option<u64>,
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
            hold: self.hold,
        })
    }

    fn files_sent_mail(&self, _account: AccountId) -> bool {
        self.gmail
    }

    fn tracking(&self) -> Option<tracking::Client> {
        let (url, _) = self.tracking.as_ref()?;
        let server = tracking::Server::parse(url)?;
        Some(tracking::Client::new(
            server,
            Tls::insecure_for_local_tests(),
        ))
    }

    async fn tracking_token(&self) -> Result<String> {
        self.tracking
            .as_ref()
            .map(|(_, token)| token.clone())
            .ok_or_else(|| Error::Rejected("tracking is off".into()))
    }
}

impl FakeSender {
    fn take(&self, from: &str, to: &[&str], message: Vec<u8>, held: Option<i64>) -> Result<()> {
        if let Answer::Refuse = self.answer {
            return Err(Error::Rejected("550 no such user".into()));
        }
        self.received.lock().unwrap().push(Received {
            from: from.into(),
            to: to.iter().map(|to| to.to_string()).collect(),
            message: String::from_utf8(message).unwrap(),
            held_until: held,
        });
        Ok(())
    }
}

impl MailSender for FakeSender {
    async fn send(&mut self, from: &str, to: &[&str], message: Vec<u8>) -> Result<()> {
        self.take(from, to, message, None)
    }

    async fn hold_limit(&mut self) -> Result<Option<u64>> {
        Ok(self.hold)
    }

    async fn send_held(
        &mut self,
        from: &str,
        to: &[&str],
        message: Vec<u8>,
        until: i64,
    ) -> Result<()> {
        assert!(self.hold.is_some(), "held without FUTURERELEASE");
        self.take(from, to, message, Some(until))
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

#[test]
fn scheduled_mail_goes_to_a_server_that_holds_it() {
    let (tmp, mut store, account) = setup();
    let at = now() + 3600;
    let id = outbox::schedule(&mut store, account, MESSAGE, 0, at, now()).unwrap();
    let smtp = FakeSmtp {
        hold: Some(7 * 24 * 3600),
        ..FakeSmtp::default()
    };
    run_until(&smtp, &tmp, config(3), SendState::Sent);
    let received = smtp.received.lock().unwrap();
    assert_eq!(received.len(), 1);
    assert_eq!(received[0].held_until, Some(at));
    assert!(!received[0].message.contains("Bcc"));
    // Not filed yet, and too late to take back.
    let entry = store.outbox_entry(id).unwrap().unwrap();
    assert_eq!((entry.state, entry.hold_until), (SendState::Sent, Some(at)));
    assert_eq!(store.next_op_due(account).unwrap(), None);
    assert_eq!(store.next_send_at().unwrap(), Some(at));
    assert!(!outbox::cancel(&mut store, id).unwrap());
}

#[test]
fn scheduled_mail_waits_here_when_the_server_cannot_hold_it() {
    let (tmp, mut store, account) = setup();
    let at = now() + 3600;
    let id = outbox::schedule(&mut store, account, MESSAGE, 0, at, now()).unwrap();
    let smtp = FakeSmtp::default();
    run_until(&smtp, &tmp, config(3), SendState::Queued);
    assert!(smtp.received.lock().unwrap().is_empty());
    let entry = store.outbox_entry(id).unwrap().unwrap();
    assert_eq!((entry.state, entry.send_at), (SendState::Queued, at));
    // Undo still works while it waits here.
    assert!(outbox::cancel(&mut store, id).unwrap());
}

#[test]
fn scheduled_mail_too_far_off_is_handed_over_later() {
    let (tmp, mut store, account) = setup();
    let at = now() + 3600;
    let id = outbox::schedule(&mut store, account, MESSAGE, 0, at, now()).unwrap();
    let smtp = FakeSmtp {
        hold: Some(600),
        ..FakeSmtp::default()
    };
    run_until(&smtp, &tmp, config(3), SendState::Queued);
    assert!(smtp.received.lock().unwrap().is_empty());
    let entry = store.outbox_entry(id).unwrap().unwrap();
    assert_eq!(entry.send_at, at - 600 + 60);
    assert_eq!(entry.hold_until, Some(at));
}

#[test]
fn undo_delay_comes_before_the_hand_over() {
    let (_tmp, mut store, account) = setup();
    let at = now() + 3600;
    let id = outbox::schedule(&mut store, account, MESSAGE, 30, at, now()).unwrap();
    let entry = store.outbox_entry(id).unwrap().unwrap();
    assert!(entry.send_at <= now() + 30 && entry.hold_until == Some(at));
    assert!(store.due_sends(now(), 10).unwrap().is_empty());
    assert!(outbox::cancel(&mut store, id).unwrap());
    // Scheduled for sooner than the undo delay: an ordinary send.
    let soon = outbox::schedule(&mut store, account, MESSAGE, 30, now() + 10, now()).unwrap();
    assert_eq!(store.outbox_entry(soon).unwrap().unwrap().hold_until, None);
}

#[test]
fn held_mail_is_filed_once_it_went_out() {
    let (tmp, mut store, account) = setup();
    // Handed over a while ago, due a second ago.
    let at = now() - 1;
    let id = outbox::schedule(&mut store, account, MESSAGE, 0, at, at - 600).unwrap();
    let message = store.outbox_entry(id).unwrap().unwrap().message;
    let mut batch = store.mail_batch().unwrap();
    batch
        .set_send_state(id, SendState::Sent, None, None)
        .unwrap();
    batch.commit().unwrap();
    let smtp = FakeSmtp {
        gmail: true,
        hold: Some(7 * 24 * 3600),
        ..FakeSmtp::default()
    };
    let events = run_until(&smtp, &tmp, config(3), SendState::Sent);
    assert_eq!(events[0].id, id);
    assert!(smtp.received.lock().unwrap().is_empty(), "not sent twice");
    assert!(store.outbox().unwrap().is_empty());
    assert!(store.messages_by_id(&[message]).unwrap().is_empty());
}

/// A message with an HTML version, as Katna Mail writes it.
const HTML_MESSAGE: &[u8] = b"From: Alice <alice@example.org>\r\nTo: Bob <bob@example.org>, carol@example.org\r\n\
    Bcc: dave@example.org\r\nSubject: Proposal v2\r\nMessage-ID: <m@example.org>\r\nMIME-Version: 1.0\r\n\
    Content-Type: multipart/alternative; boundary=\"b\"\r\n\r\n\
    --b\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nSee https://example.com/p\r\n\
    --b\r\nContent-Type: text/html; charset=utf-8\r\nContent-Transfer-Encoding: quoted-printable\r\n\r\n\
    <p>See <a href=3D\"https://example.com/p\">the proposal</a>.</p>\r\n--b--\r\n";

const TEST_SERVER: &str = "http://127.0.0.1:9";

/// Queues `raw` tracked, with tracking IDs already given (as after a retry),
/// so no tracking server is needed.
fn queue_tracked(store: &mut Store, account: AccountId, raw: &[u8]) -> (i64, Vec<String>) {
    let id = outbox::queue_with(store, account, raw, 0, now(), true).unwrap();
    let entry = store.outbox_entry(id).unwrap().unwrap();
    assert!(entry.per_recipient);
    let ids: Vec<String> = (0..3).map(|n| format!("{n:032x}")).collect();
    let emails = ["bob@example.org", "carol@example.org", "dave@example.org"];
    let recipients: Vec<katna_store::NewRecipient<'_>> = ids
        .iter()
        .zip(emails)
        .map(|(id, email)| katna_store::NewRecipient {
            tracking_id: id,
            email,
            name: None,
        })
        .collect();
    store
        .start_tracking(
            id,
            account,
            "m@example.org",
            "Proposal v2",
            &["https://example.com/p".to_owned()],
            &recipients,
            now(),
        )
        .unwrap();
    (id, ids)
}

#[test]
fn tracked_mail_goes_to_each_recipient_alone() {
    let (tmp, mut store, account) = setup();
    let (id, ids) = queue_tracked(&mut store, account, HTML_MESSAGE);
    let smtp = FakeSmtp {
        tracking: Some((TEST_SERVER.into(), "t".into())),
        ..FakeSmtp::default()
    };
    run_until(&smtp, &tmp, config(3), SendState::Sent);
    let received = smtp.received.lock().unwrap();
    assert_eq!(received.len(), 3);
    let header = |wire: &str| wire[..wire.find("\r\n\r\n").unwrap()].to_owned();
    for (copy, (tracking_id, email)) in received.iter().zip(ids.iter().zip([
        "bob@example.org",
        "carol@example.org",
        "dave@example.org",
    ])) {
        assert_eq!(copy.to, [email]);
        assert_eq!(header(&copy.message), header(&received[0].message));
        assert!(!copy.message.contains("Bcc"), "{}", copy.message);
        let html = copy.message.replace("=\r\n", "");
        assert!(
            html.contains(&format!("{TEST_SERVER}/o/{tracking_id}.png")),
            "{html}"
        );
        assert!(
            html.contains(&format!("{TEST_SERVER}/l/{tracking_id}/0")),
            "{html}"
        );
        assert!(copy.message.contains("See https://example.com/p\r\n"));
    }
    let tracked = store.tracking_for_outbox(id).unwrap().unwrap();
    assert!(tracked.sent_at.is_some());
    assert!(tracked.recipients.iter().all(|r| r.sent_at.is_some()));
}

#[test]
fn a_refused_recipient_does_not_stop_the_others() {
    let (tmp, mut store, account) = setup();
    let (id, _) = queue_tracked(&mut store, account, HTML_MESSAGE);
    let smtp = FakeSmtp {
        tracking: Some((TEST_SERVER.into(), "t".into())),
        ..FakeSmtp::default()
    };
    // Bob's connection refuses; Carol and Dave get theirs on a new one,
    // and Bob his on the retry.
    smtp.script.lock().unwrap().push_back(Answer::Refuse);
    let events = run_until(&smtp, &tmp, config(3), SendState::Sent);
    let retry = events
        .iter()
        .find(|e| e.state == SendState::Queued)
        .unwrap();
    assert!(
        retry.detail.contains("not delivered to bob@example.org"),
        "{retry:?}"
    );
    assert!(
        retry
            .detail
            .contains("delivered to carol@example.org, dave@example.org"),
        "{retry:?}"
    );
    let received = smtp.received.lock().unwrap();
    let to: Vec<&str> = received.iter().map(|r| r.to[0].as_str()).collect();
    assert_eq!(
        to,
        ["carol@example.org", "dave@example.org", "bob@example.org"]
    );
    assert!(
        store
            .tracking_for_outbox(id)
            .unwrap()
            .unwrap()
            .sent_at
            .is_some()
    );
}

#[test]
fn without_tracking_mail_goes_out_once() {
    let (tmp, mut store, account) = setup();
    let id = outbox::queue_with(&mut store, account, HTML_MESSAGE, 0, now(), true).unwrap();
    let smtp = FakeSmtp::default();
    run_until(&smtp, &tmp, config(3), SendState::Sent);
    let received = smtp.received.lock().unwrap();
    assert_eq!(received.len(), 1);
    assert_eq!(received[0].to.len(), 3);
    assert!(!received[0].message.contains("/o/"));
    assert!(store.tracking_for_outbox(id).unwrap().is_none());
}

#[test]
fn gmail_tracked_copies_are_purged() {
    let (tmp, mut store, account) = setup();
    let server = FakeServer::default();
    for folder in ["INBOX", "Sent", "Trash", "[Gmail]/All Mail"] {
        server.create(folder, 1);
    }
    server.state().gmail = true;
    let mut conn = server.connection();
    smol::block_on(engine::sync_account(&mut conn, &mut store, account)).unwrap();
    queue_tracked(&mut store, account, HTML_MESSAGE);
    let smtp = FakeSmtp {
        gmail: true,
        tracking: Some((TEST_SERVER.into(), "t".into())),
        ..FakeSmtp::default()
    };
    run_until(&smtp, &tmp, config(3), SendState::Sent);

    // Gmail filed each copy it sent; an unrelated message stays.
    let copies: Vec<String> = smtp
        .received
        .lock()
        .unwrap()
        .iter()
        .map(|r| r.message.clone())
        .collect();
    let message_id = copies[0]
        .lines()
        .find_map(|l| l.strip_prefix("Message-ID: "))
        .unwrap()
        .to_owned();
    for copy in &copies {
        let (header, body) = copy.split_once("\r\n\r\n").unwrap();
        server.deliver_header("[Gmail]/All Mail", &format!("{header}\r\n\r\n"), body);
    }
    server.deliver("[Gmail]/All Mail", "Unrelated");

    let mut conn = server.connection();
    let report = smol::block_on(ops::replay(&mut conn, &mut store, account, now())).unwrap();
    assert_eq!((report.done, report.failed), (2, 0), "{report:?}");
    let state = server.state();
    let left: Vec<String> = state.folders["[Gmail]/All Mail"]
        .messages
        .values()
        .map(|m| String::from_utf8_lossy(&m.header).into_owned())
        .collect();
    assert_eq!(left.len(), 1, "{left:?}");
    assert!(left[0].contains("Unrelated"));
    assert!(state.folders["Trash"].messages.is_empty());
    // The clean copy is filed in Sent, without the pixel.
    let sent: Vec<_> = state.folders["Sent"].messages.values().collect();
    assert_eq!(sent.len(), 1);
    assert!(String::from_utf8_lossy(&sent[0].header).contains(&message_id));
    assert!(!String::from_utf8_lossy(&sent[0].body).contains("/o/"));
}

/// Registers an install on the server at `url` and signs it in to a new
/// Katna account with a confirmed address, reading the code from the
/// server's log (it has no SMTP relay). Returns the install's token.
fn signed_in_token(url: &str, client: &tracking::Client, log: &str) -> String {
    use katna_sync::autoconfig::http;
    let registration = smol::block_on(client.register()).unwrap();
    let bearer = format!("Bearer {}", registration.token);
    let tls = Tls::insecure_for_local_tests();
    let post = |path: &str, body: String| {
        smol::block_on(http::request(
            "POST",
            &format!("{url}{path}"),
            &[("Authorization", bearer.as_str())],
            Some(body.as_bytes()),
            &tls,
            Duration::from_secs(10),
        ))
        .unwrap()
    };
    let email = format!("tracking-{}@example.org", registration.install);
    let (status, body) = post(
        "/api/v1/account",
        format!(r#"{{"email":"{email}","password":"correct horse","device":"ci"}}"#),
    );
    assert_eq!(status, 201, "{}", String::from_utf8_lossy(&body));
    // The server logs `… code not mailed to=… purpose=… code="123456"`:
    // the code is the line's last run of six digits (the address before it
    // holds hex digits too).
    let text = std::fs::read_to_string(log).unwrap();
    let line = text
        .lines()
        .rev()
        .find(|line| line.contains("code not mailed") && line.contains(&email))
        .expect("the code in the server's log");
    let code: String = line
        .split(|c: char| !c.is_ascii_digit())
        .rev()
        .find(|run| run.len() == 6)
        .expect("a six-digit code")
        .to_owned();
    let (status, body) = post("/api/v1/account/verify", format!(r#"{{"code":"{code}"}}"#));
    assert_eq!(status, 204, "{}", String::from_utf8_lossy(&body));
    registration.token
}

/// Against a running Katna Server (CI's `server` job starts one):
/// `KATNA_TEST_TRACKING_URL=http://127.0.0.1:8080`, with its log in
/// `KATNA_TEST_SERVER_LOG`.
#[test]
#[ignore = "needs katna-server at KATNA_TEST_TRACKING_URL"]
fn tracked_mail_with_a_real_server() {
    let url = std::env::var("KATNA_TEST_TRACKING_URL").expect("KATNA_TEST_TRACKING_URL");
    let log = std::env::var("KATNA_TEST_SERVER_LOG").expect("KATNA_TEST_SERVER_LOG");
    let client = tracking::Client::new(
        tracking::Server::parse(&url).unwrap(),
        Tls::insecure_for_local_tests(),
    );
    let token = signed_in_token(&url, &client, &log);
    let (tmp, mut store, account) = setup();
    let id = outbox::queue_with(&mut store, account, HTML_MESSAGE, 0, now(), true).unwrap();
    let smtp = FakeSmtp {
        tracking: Some((url.clone(), token.clone())),
        ..FakeSmtp::default()
    };
    run_until(&smtp, &tmp, config(3), SendState::Sent);
    assert_eq!(smtp.received.lock().unwrap().len(), 3);
    let tracked = store.tracking_for_outbox(id).unwrap().unwrap();
    assert_eq!(tracked.links, ["https://example.com/p"]);
    let carol = &tracked.recipients[1];
    assert_eq!(carol.email, "carol@example.org");

    // Carol's mail program fetches the pixel and follows the link.
    let (host, port) = url.trim_start_matches("http://").split_once(':').unwrap();
    let port: u16 = port.parse().unwrap();
    let get = |path: String| {
        smol::block_on(async {
            use futures_lite::{AsyncReadExt, AsyncWriteExt};
            let mut tcp = async_net::TcpStream::connect((host, port)).await.unwrap();
            let request = format!(
                "GET {path} HTTP/1.1\r\nHost: {host}\r\nUser-Agent: Mozilla/5.0 (X11; Linux x86_64) Firefox/140.0\r\nConnection: close\r\n\r\n"
            );
            tcp.write_all(request.as_bytes()).await.unwrap();
            let mut answer = String::new();
            let mut bytes = Vec::new();
            tcp.read_to_end(&mut bytes).await.unwrap();
            answer.push_str(&String::from_utf8_lossy(&bytes));
            answer
        })
    };
    assert!(get(format!("/o/{}.png", carol.tracking_id)).starts_with("HTTP/1.1 200"));
    let redirect = get(format!("/l/{}/0", carol.tracking_id));
    assert!(redirect.starts_with("HTTP/1.1 302"), "{redirect}");
    assert!(
        redirect
            .to_ascii_lowercase()
            .contains("location: https://example.com/p")
    );

    let events = smol::block_on(async {
        let mut stream = client.events(&token, 0).await.unwrap();
        let mut events = Vec::new();
        while events.len() < 2 {
            events.push(stream.next(Duration::from_secs(10)).await.unwrap().unwrap());
        }
        events
    });
    assert_eq!(events[0].id, carol.tracking_id);
    assert_eq!(events[0].kind, "open");
    // Seconds after sending: a scanner, by the server's rules.
    assert_eq!(events[0].source, "scanner");
    assert_eq!(
        (events[1].kind.as_str(), events[1].link),
        ("click", Some(0))
    );
}
