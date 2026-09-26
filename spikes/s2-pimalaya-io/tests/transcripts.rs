// SPDX-License-Identifier: GPL-3.0-or-later

//! Scripted server transcripts: the sans-I/O design lets us test protocol
//! edge cases without a server. These pin down two io-imap 0.6 behaviours
//! the findings document depends on.

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use io_imap::{
    codec::fragmentizer::Fragmentizer,
    coroutine::{ImapCoroutine, ImapCoroutineState as S, ImapYield},
    rfc2177::idle::{ImapIdle, ImapIdleOptions, ImapIdleYield},
    rfc3501::noop::ImapNoop,
};

fn tag_of(command: &[u8]) -> String {
    let line = String::from_utf8_lossy(command);
    line.split(' ').next().unwrap().to_owned()
}

/// Untagged EXISTS that arrives between our DONE and the server's tagged OK
/// is dropped: the IDLE coroutine completes without yielding it.
#[test]
fn idle_drops_updates_sent_after_done() {
    let done = Arc::new(AtomicBool::new(false));
    let mut co = ImapIdle::new(done.clone(), ImapIdleOptions::default());
    let mut frag = Fragmentizer::new(1 << 20);

    let S::Yielded(ImapIdleYield::WantsWrite(idle)) = co.resume(&mut frag, None) else {
        panic!("expected IDLE command");
    };
    let tag = tag_of(&idle);
    assert!(matches!(
        co.resume(&mut frag, None),
        S::Yielded(ImapIdleYield::WantsRead)
    ));
    // The server accepts IDLE; nothing happens yet.
    let mut state = co.resume(&mut frag, Some(b"+ idling\r\n"));
    while let S::Yielded(ImapIdleYield::Event(_)) = state {
        state = co.resume(&mut frag, None);
    }
    assert!(matches!(state, S::Yielded(ImapIdleYield::WantsRead)));

    // We decide to stop (timer or shutdown) and send DONE.
    done.store(true, Ordering::SeqCst);
    assert!(matches!(
        co.resume(&mut frag, None),
        S::Yielded(ImapIdleYield::WantsWrite(ref b)) if b == b"DONE\r\n"
    ));
    assert!(matches!(
        co.resume(&mut frag, None),
        S::Yielded(ImapIdleYield::WantsRead)
    ));

    // New mail arrives in the same packet as the tagged OK.
    let reply = format!("* 9 EXISTS\r\n{tag} OK IDLE terminated\r\n");
    let mut events = 0;
    let mut state = co.resume(&mut frag, Some(reply.as_bytes()));
    loop {
        match state {
            S::Yielded(ImapIdleYield::Event(_)) => {
                events += 1;
                state = co.resume(&mut frag, None);
            }
            S::Complete(result) => {
                result.unwrap();
                break;
            }
            other => panic!("unexpected {other:?}"),
        }
    }
    assert_eq!(
        events, 0,
        "io-imap now surfaces the EXISTS; update the findings"
    );
}

/// `ImapNoop` returns `()`: the EXISTS/EXPUNGE/FETCH responses a NOOP exists
/// to collect are thrown away.
#[test]
fn noop_discards_untagged_updates() {
    let mut co = ImapNoop::new();
    let mut frag = Fragmentizer::new(1 << 20);
    let S::Yielded(ImapYield::WantsWrite(noop)) = co.resume(&mut frag, None) else {
        panic!("expected NOOP command");
    };
    let tag = tag_of(&noop);
    assert!(matches!(
        co.resume(&mut frag, None),
        S::Yielded(ImapYield::WantsRead)
    ));
    let reply = format!("* 12 EXISTS\r\n* 3 EXPUNGE\r\n{tag} OK NOOP completed\r\n");
    match co.resume(&mut frag, Some(reply.as_bytes())) {
        S::Complete(Ok(())) => {}
        other => panic!("unexpected {other:?}"),
    }
}

/// A synchronising-literal APPEND fails when the server's `+` continuation
/// line arrives split across two reads: io-imap sends the literal as soon
/// as it has no complete line to decode, then treats the late `+` as the
/// end of the command and returns before the tagged OK.
#[test]
fn append_fails_when_continuation_is_split() {
    use io_imap::rfc3501::append::{
        ImapMessageAppend, ImapMessageAppendError, ImapMessageAppendOptions,
    };

    let mut co = ImapMessageAppend::new(
        "INBOX".try_into().unwrap(),
        b"hi".to_vec(),
        ImapMessageAppendOptions::default(),
    );
    let mut frag = Fragmentizer::new(1 << 20);
    let S::Yielded(ImapYield::WantsWrite(header)) = co.resume(&mut frag, None) else {
        panic!("expected APPEND command");
    };
    assert!(header.ends_with(b"{2}\r\n"));
    assert!(matches!(
        co.resume(&mut frag, None),
        S::Yielded(ImapYield::WantsRead)
    ));

    // First half of "+ OK\r\n": io-imap already writes the literal.
    assert!(matches!(
        co.resume(&mut frag, Some(b"+ O")),
        S::Yielded(ImapYield::WantsWrite(ref b)) if b == b"hi\r\n"
    ));
    assert!(matches!(
        co.resume(&mut frag, None),
        S::Yielded(ImapYield::WantsRead)
    ));

    // Second half: the command "completes" without its tagged response,
    // which is still on the wire and will confuse the next command.
    match co.resume(&mut frag, Some(b"K\r\n")) {
        S::Complete(Err(ImapMessageAppendError::MissingTagged)) => {}
        other => panic!("io-imap behaviour changed, update the findings: {other:?}"),
    }
}
