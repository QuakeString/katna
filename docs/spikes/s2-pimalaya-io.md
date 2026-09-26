# Spike S2 — Pimalaya with our own I/O

> Status: **passed on Stalwart, Dovecot and Gmail**, with two Gmail gaps noted below (26 September 2026).
> Code: [`spikes/s2-pimalaya-io/`](../../spikes/s2-pimalaya-io/) (throw-away).

## Question

Can we drive Pimalaya's sans-I/O `io-imap` and `io-smtp` from our own rustls
networking, behind our own `MailBackend` trait, without Pimalaya types leaking
out? Success means: log in to Stalwart and Gmail (app password), list folders,
fetch 1,000 envelopes, and IDLE for new mail
([plan §4](../IMPLEMENTATION_PLAN.md#4-risk-spikes-time-boxed-throw-away-code)).

## Answer

**Yes, Pimalaya fits.** Use it for IMAP and SMTP in task 1.1, with four
conditions:

1. Some io-imap command wrappers throw away server updates (below). We write
   those few commands ourselves on io-imap's public `ImapSend` building block,
   and report the problem upstream.
2. Each connection is owned by one task, and a command is never cancelled
   halfway. Dropping a command future leaves the connection in the middle of
   the protocol.
3. We pin exact versions. io-imap had seven breaking releases (0.1 to 0.6)
   between June and August 2026, and `imap-codec` is still a 2.0 alpha.
4. Our driver hands io-imap whole lines only. io-imap breaks APPEND when the
   server's `+` continuation arrives in two pieces, and Dovecot does send it
   that way (problem 10).

Gmail passed login, folders, envelopes and IDLE timeout. Two gaps remain: the
test account holds only 79 messages, and no mail arrived during the IDLE wait
(see [Gmail](#gmail)).

## What was built

- `lib.rs`: Katna-owned types (`Folder`, `FolderStatus`, `Envelope`,
  `Flags`, `FolderChange`, …) and two traits, `MailBackend` (list, select,
  fetch envelopes, append, create, poll, wait for changes, logout) and
  `MailSender` (send, quit). No Pimalaya type appears in them.
- `net.rs`: `Conn`, a TCP connection from `async-net` with optional TLS from
  `futures-rustls` (ring provider, platform certificate verifier), with read
  timeouts. Reads always end on a line break (problem 10). About 170 lines,
  most of it trait plumbing.
- `imap.rs` and `smtp.rs`: the only files that import Pimalaya. Each has a
  roughly 20-line loop that answers the coroutine's requests (`WantsRead`,
  `WantsWrite`, and for session opening `WantsTcpConnect`, `WantsTlsConnect`,
  `WantsTlsUpgrade`).
- `bin/s2.rs`: runs the checks against one server and prints a report.
- `examples/append_stress.rs`: appends thousands of messages to Dovecot, to
  reproduce problem 10.
- `tests/transcripts.rs`: scripted server replies that pin the io-imap
  behaviour described below. No server is needed; this is a real advantage of
  sans-I/O.

## Results

Local servers from `dev/compose.yaml` (draft PR #3), release build,
same machine:

| Check | Stalwart 0.16 (implicit TLS) | Dovecot 2.4.5 (STARTTLS) |
|---|---|---|
| IMAP login (TCP + TLS + SASL PLAIN) | 31 ms | 16 ms |
| List folders, with special-use roles | 6 folders, 0.4 ms | 6 folders, 0.9 ms |
| Append 1,000 messages (one at a time) | 1.2 s | 3.6 s |
| `UID FETCH 1:* (UID FLAGS RFC822.SIZE ENVELOPE)`, 1,000 messages | 50 ms | 33 ms |
| IDLE ended by our timer, connection reused | OK | OK |
| IDLE woken by an append from another connection | 1.6 ms after append | 500 ms after append ¹ |
| SMTP login (submission) | 29 ms | 14 ms |
| SMTP send to self, IDLE sees it arrive | 3 ms after send | not applicable ² |
| SMTP send to another domain (caught by Mailpit) | not tested | 6 ms |

¹ Dovecot batches IDLE notifications; this is its server-side delay.
² The Dovecot dev setup relays all mail to Mailpit rather than delivering locally.

### Gmail

Run by the project owner (the cloud sandbox cannot reach `imap.gmail.com`),
read-only `custom` profile with an app password, on 26 September 2026:

| Check | Result |
|---|---|
| IMAP login (implicit TLS, platform certificate verifier) | OK, 1.2 s |
| List folders | 11 folders in 250 ms; All Mail, Drafts, Sent, Junk, Flagged and Trash roles detected |
| Fetch envelopes from `[Gmail]/All Mail` | all 79 in 0.4–0.5 s |
| IDLE ended by our timer, then NOOP, then logout | OK |
| IDLE woken by new mail | **not tested**: no mail arrived during the 2-minute wait |
| Nothing appended, sent or created | confirmed |

Gaps: the account holds 79 messages, so the 1,000-envelope check could not
run on Gmail. It passed on Stalwart and Dovecot. `S2_ALLOW_FEW=1` now turns
that check into a warning for small accounts. IDLE wake-up on Gmail still
needs a run where someone sends a mail during the wait.

Also checked:

- **Executors.** The same code ran under `futures_lite::future::block_on` and
  as a task on smol's thread pool (`S2_EXECUTOR=smol`). All trait futures are
  `Send`. The `async-io` reactor runs on its own thread, so the code does not
  depend on any particular executor. It was not run inside GPUI (no display in
  the sandbox). That matters less than the plan assumed, because only
  `katna-daemon` talks to the network (`CLAUDE.md`), and the daemon does not
  use GPUI.
- **TLS.** rustls only. The dev servers use self-signed certificates, so the
  local runs use `Tls::insecure_for_local_tests()`. The `custom` profile
  (Gmail and other real servers) uses the platform verifier.
- **Size.** The whole test binary (rustls, smol, env_logger, io-imap,
  io-smtp) is 3.9 MiB stripped with fat LTO. The daemon's budget is 15 MiB.
- **Dependencies.** 119 crates in the tree. `cargo deny check` with our
  `deny.toml` passes (advisories, bans, licenses and sources all OK). It only
  warns about duplicate versions: `base64` 0.22/0.23, `getrandom` 0.2/0.4,
  `syn` 2/3 and `windows-sys`.

## Findings about io-imap 0.6 and io-smtp 0.3

What works well:

- The session-opening coroutine (`ImapSessionOpen`, `SmtpSessionOpen`)
  handles the order of the handshake: greeting, STARTTLS, refreshing
  CAPABILITY after TLS, PREAUTH, SASL-IR, and provider quirks. We only answer
  "connect", "upgrade to TLS", "read" and "write". It also refuses a
  STARTTLS reply with injected trailing bytes.
- Mailbox names are converted to and from modified UTF-7 in both directions.
- Error types separate authentication failures from protocol errors, so the
  engine can tell "wrong password" from "server broken".
- Command tags are unique and unpredictable in release builds.

Problems, and what we do about each:

| # | Problem | Effect | What to do |
|---|---|---|---|
| 1 | `ImapNoop` returns `()`. The EXISTS, EXPUNGE and FETCH responses that a NOOP is meant to collect are thrown away (test `noop_discards_untagged_updates`). | Polling with NOOP never sees new mail. | Build NOOP on the public `ImapSend` (done in `poll_changes`). Report upstream. |
| 2 | The IDLE coroutine drops untagged responses that arrive between our `DONE` and the server's `OK` (test `idle_drops_updates_sent_after_done`). Other command wrappers keep only their own response type, for example FETCH keeps only FETCH data. | Mail that arrives while IDLE is ending can go unnoticed until the next change. | After every IDLE, check `UIDNEXT` and `HIGHESTMODSEQ` (task 1.3 does this anyway), or send our own NOOP. Report upstream. |
| 3 | IDLE's automatic refresh (every 29 s by default) needs io-imap's `client` feature, which uses the std clock. Without it, the coroutine has no timer. | None. We want to control timing ourselves. | Our driver stops IDLE with the `done` flag when its own timer fires (`wait_for_changes`). Choose the refresh interval in task 1.4. |
| 4 | Dropping a future in the middle of a command leaves the connection out of sync (half a command sent, or a reply not yet read). | Cancelling from the UI or on shutdown can corrupt a session. | One task owns each connection and takes commands from a channel. Stop IDLE with the `done` flag, never by dropping the future. A single socket read can be cancelled safely; `Conn::read_timeout` relies on that. |
| 5 | `ImapMessageFetch` collects the whole reply in memory, keyed by sequence number. | Fine for 1,000 messages (33–50 ms). Too much for a first sync of 100,000. | Fetch in UID chunks, or use `ImapMessageFetchStreamBatch`. Decide in task 1.3. |
| 6 | SELECT does not ask for CONDSTORE, so `HIGHESTMODSEQ` was missing on both servers. | Incremental sync needs it. | Pass `CONDSTORE` or `ENABLE QRESYNC` in the select options (task 1.3). |
| 7 | ENVELOPE strings are raw, so RFC 2047 encoded words are not decoded. | Subjects and names need decoding. | Decode in our MIME layer (`mail-parser`), or fetch the header fields and parse them there. |
| 8 | Default features pull in `pimalaya-stream`, Pimalaya's own std/TLS I/O. | Extra code we do not use. | Use `default-features = false` (done here). Enable `scram` when we add SCRAM-SHA-256. |
| 9 | Seven breaking releases in about 11 weeks. `imap-codec` 2.0 is still alpha. | Upgrades will need work. | Pin exact versions. Keep Pimalaya imports in one module per protocol, as here. Rely on the weekly `upstream` CI job (plan §3.3). |
| 10 | A synchronising-literal APPEND fails with "server did not return a tagged response" when the server's `+ OK` continuation arrives split across two reads. `ImapSend` writes the literal as soon as it has no complete line to decode, then treats the late `+` as the end of the command. The real tagged OK is left unread (test `append_fails_when_continuation_is_split`). | Seen on Dovecot over STARTTLS: 2 of 5 runs of 3,000 APPENDs failed, and the failing read was a lone `+`. Stalwart never split the line in our runs. | `Conn` keeps reading until a chunk ends with a line break, so io-imap never sees half a line. After the fix, 20 runs of 3,000 APPENDs had none. Upstream fix: only send the literal after a continuation request arrives. Report upstream. |

None of these calls for a fork today. Problems 1 and 2 are small upstream
fixes: return the collected `data` and `untagged` responses. Problem 10 is a
one-condition fix in `ImapSend`. If upstream does
not take them, the light-fork policy in ARCHITECTURE.md §20 applies.

## Running it

```sh
cd dev && docker compose up -d            # from PR #3
cd ../spikes/s2-pimalaya-io
cargo run --release -- stalwart
cargo run --release -- dovecot
cargo test --release                      # transcript tests, no server needed
```

Gmail (read-only: it never appends or sends; send yourself a mail while it
waits in IDLE):

```sh
S2_IMAP=imap.gmail.com:993:tls S2_USER=you@gmail.com \
S2_PASSWORD='<app password>' S2_FOLDER='[Gmail]/All Mail' \
S2_ALLOW_FEW=1 cargo run --release -- custom
```

APPEND stress test (problem 10), against the local Dovecot:

```sh
cargo run --release --example append_stress -- 3000
```

## Next steps

- Rerun the Gmail IDLE wait and send a mail during it, to test wake-up.
- ~~Task 1.1: move `net.rs` and the driver loops into `katna-sync`, with one
  task per connection (problem 4).~~ Done: `crates/katna-sync` (NOOP and IDLE
  built on `ImapSend`, which also fixes problem 2). JMAP and POP3 still need
  their place in the `MailBackend` design.
- Report problems 1, 2 and 10 to `pimalaya/io-imap`.
