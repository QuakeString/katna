# Katna PIM — Implementation Plan

> Status: **Draft v0.1** (26 September 2026). Companion to
> [ARCHITECTURE.md](ARCHITECTURE.md), which defines *what* we build. This
> document defines *in which order*, *how we know a step is done*, and *how
> we work*.

## 1. Working principles

1. **Engine first, GUI second.** Storage, search, sync and the daemon are
   built and proven from the command line before any window exists.
2. **Kill the big risks early.** Four short spikes (§4) run before or
   alongside Phase 0, so we don't discover blockers after months of work.
3. **Vertical slices.** Each milestone ends with something that runs
   end-to-end, not a half-finished layer.
4. **Every milestone has a "done when".** A milestone is finished only when
   its acceptance checks pass in CI or in a documented manual test.
5. **Real servers from day one.** Integration tests run against real mail
   and calendar servers in containers, not only mocks.
6. **Measure, don't guess.** Size, speed and resource budgets are checked in CI.

## 2. Decisions to make before coding (week 0)

These affect the first commit. Defaults are proposed; change them now or
they become the plan.

| # | Decision | Proposed default | Affects |
|---|---|---|---|
| D1 | License | **Decided: GPL-3.0-or-later** (`LICENSE` added) | `cargo-deny` config, SPDX headers, what code we may reuse |
| D2 | Repository | **Decided: `QuakeString/katna`** (renamed), one monorepo | Paths, CI, package names |
| D3 | Mail storage | **Decided:** SQLite (`rusqlite`) for metadata **and** compressed raw messages (`blobs.db`); files only for large attachments; Maildir export. Turso re-evaluated at its 1.0 (file-format compatible) | `katna-store` design |
| D4 | Rust toolchain | **Decided: latest stable** — `rust-toolchain.toml` with `channel = "stable"`; `rust-version` in `Cargo.toml` records the minimum and is raised deliberately | CI, contributors |
| D5 | Supported systems for CI | **Decided: Arch Linux and Ubuntu 26.04 LTS** — Arch = latest Plasma and GNOME; Ubuntu = GNOME, Kubuntu 26.04 = older Plasma | Test matrix, Plasma versions |
| D6 | App ID / D-Bus prefix | **Decided: `in.invenia.katna`** (domain `katna.invenia.in`): `in.invenia.katna.Mail`, `in.invenia.katna.Calendar`, `in.invenia.katna.Daemon`, interface `in.invenia.katna.Pim1` | Flatpak IDs, D-Bus names, desktop files |
| D7 | Where crash reports and feedback go | **Decided: a Sentry cloud project now, our own server later** (owner, 27 September 2026): self-hosted GlitchTip or Sentry on `katna.invenia.in` replaces it by changing the DSN. Only after the user opts in; DSN empty until the project exists (`ARCHITECTURE.md` §19.2) | Crash-report track C.5–C.9 |

D6 matters for Flathub: its app IDs must match a domain or code-hosting
account you control.

## 3. Development environment

### 3.1 Local test servers (containers)

A `dev/compose.yaml` (Podman or Docker) starts the following; see
`dev/README.md` for ports and accounts:

| Service | Purpose |
|---|---|
| **Stalwart** | IMAP, JMAP, SMTP, CalDAV, CardDAV, Sieve in one server |
| **Dovecot** | The most common real-world IMAP server; CONDSTORE/QRESYNC, NOTIFY. Its own submission service relays to Mailpit, so no Postfix is needed |
| **Radicale** | Simple CalDAV/CardDAV reference |
| **Mailpit** | Captures outgoing SMTP for send tests |

A seed script creates test accounts and loads sample mail, events and
contacts. Mail that Stalwart or Dovecot would send to another domain goes to
Mailpit.

### 3.2 Test data

- **Enron corpus** (~500k messages) for search and performance benchmarks,
  downloaded by `dev/fetch-enron.sh` (not committed) and read with
  `katna-search-cli import`.
- A **real-world email corpus** (newsletters, HTML-heavy mail, broken MIME,
  many languages) for rendering and parser tests; only mail we may
  legally use.
- Synthetic generators for organizations, threads and recurring events.

### 3.3 Continuous integration (GitHub Actions)

| Job | When | What |
|---|---|---|
| `check` | every push/PR | `cargo fmt --check`, `clippy -D warnings`, unit tests — in `archlinux:latest` and `ubuntu:26.04` containers |
| `integration` | every PR | Start containers, run protocol tests against Stalwart/Dovecot/Radicale |
| `deny` | every PR | `cargo-deny`: licenses, duplicate/banned crates, advisories |
| `size` | every PR | Release build; fail if a binary exceeds its budget |
| `bench` | nightly | Enron search benchmark; alert on regressions > 10 % |
| `upstream` | weekly | Build against latest `gpui-pre`, Pimalaya and Plasma |
| `fuzz` | nightly (short) | MIME parser wrapper, query parser, iCalendar, threading |

### 3.4 Conventions

- Branch per task, pull request per change, CI green before merge.
- Conventional commit messages (`feat(search): …`, `fix(sync): …`).
- `CLAUDE.md` in the repo with build/test commands and conventions, for
  AI-assisted work.
- Each milestone is a GitHub milestone; each task below is an issue.

## 4. Risk spikes (time-boxed, throw-away code)

Run these first. Each ends with a short written result in `docs/spikes/`.

| Spike | Question | Time box | Success means |
|---|---|---|---|
| **S1 — Window chrome** | Can GPUI give us native-looking frames? | 1 week | GPUI window with KWin server-side decorations on Plasma, and client-side header bar on GNOME with shadow, rounded corners, correct `button-layout`, resize edges and tiling behavior |
| **S2 — Pimalaya + our I/O** | Can we drive `io-imap`/`io-smtp` from our own rustls I/O on GPUI-compatible executors? | 1 week | Log in to Stalwart and Gmail (app password), list folders, fetch 1,000 envelopes, IDLE for new mail |
| **S3 — HTML rendering** | Which renderer for real-world mail? | 1–2 weeks | 50 real HTML emails rendered via (a) sanitized subset in GPUI, (b) Blitz, (c) WebKitGTK/`wry`; scored on fidelity, speed, binary size, security; decision recorded |
| **S4 — Notifications** | Do inline reply and activation tokens work from Rust? | 3 days | `zbus` notification on Plasma with click-to-focus (Wayland) and `inline-reply`; fallback button on GNOME |

If a spike fails, the architecture is updated before the related phase starts.

## 5. Milestones

Size estimates assume **one full-time developer**; part-time work
stretches them proportionally. They are rough and will be corrected after
Phase 0.

### Phase 0 — Foundations (≈ 4 weeks)

| Task | Deliverable |
|---|---|
| 0.1 Workspace scaffold | Cargo workspace (§3 of the architecture), `rust-toolchain.toml`, lints, `LICENSE`, `CLAUDE.md`, CI jobs `check`, `deny`, `size` |
| 0.2 `katna-core` | XDG paths, TOML config (`serde`), logging (`tracing`), error types, account model |
| 0.3 `katna-store` | SQLite (WAL) with migrations, schema v1 (mail + PIM tables), blob store (zstd + blake3), read-only open mode, change journal |
| 0.4 Importers | Maildir and mbox import into the store (used for Enron and for users migrating) |
| 0.5 `katna-search` | Index schema, indexing pipeline (parse → text → language → tokenize), batching, versioned schema |
| 0.6 Query language | Parser for `from: to: org: has: before: after: larger: in: label: is:` + phrases, `-`, `OR`, parentheses → tantivy queries; fuzz-tested |
| 0.7 Ranking + snippets | BM25 + recency/subject boosts, highlighted snippets |
| 0.8 `katna-search-cli` + `katna-bench` | `katna-search index <maildir>`, `katna-search query "<q>"` with timings; Enron benchmark job |

**Done when:** Enron is imported and indexed; queries like
`from:kenneth.lay has:attachment budget` return in < 50 ms (p99) on a
normal laptop; the index can be deleted and rebuilt from the store; CI is green.

Status (26 September 2026): tasks 0.1–0.8 are merged. On the real Enron
corpus (517,401 messages) the index builds in 13.7 s and the benchmark
query set runs at p50 2.4 ms, p99 9.7 ms, including subjects and snippets.

### Phase 1 — Daemon and sync core (≈ 10 weeks)

| Task | Deliverable |
|---|---|
| 1.1 I/O layer | rustls + async I/O driver for Pimalaya's sans-I/O coroutines; `MailBackend` trait |
| 1.2 Account setup | Autoconfiguration (Thunderbird ISPDB, provider autoconfig, DNS SRV per RFC 6186), password login, secrets in Secret Service (`oo7`) |
| 1.3 IMAP level 1 | Folder list, envelope/flags/`BODYSTRUCTURE` sync, CONDSTORE/QRESYNC incremental sync, UIDVALIDITY handling |
| 1.4 IMAP push | IDLE with renewal, per-folder connections within server limits, reconnect/backoff |
| 1.5 Levels 2 and 3 | Offline window (full bodies), text backfill for indexing, eviction when mail leaves the window |
| 1.6 Op queue | Optimistic local flags/move/delete, replay with retries and conflict handling |
| 1.7 Threading | JWZ threading + Gmail thread IDs; property tests |
| 1.8 SMTP + outbox | Sending, Sent-folder handling, outbox with undo delay |
| 1.9 `katna-meta` | Metadata table + scheduler (undo send first) |
| 1.10 POP3 | Client with UIDL tracking, leave-on-server, `TOP` (header-first partial download of large messages comes later) |
| 1.11 `katna-daemon` | Process, `in.invenia.katna.Pim1` D-Bus skeleton (commands + change signals), single instance, systemd user unit, D-Bus activation, graceful shutdown |
| 1.12 System events | Network changes, suspend/resume, metered connections |
| 1.13 `katnactl` | Small CLI client for the daemon (add account, sync, search, send, list) — the test harness until the GUI exists |

**Done when:** two real accounts (Stalwart + Gmail/Fastmail) sync
incrementally in the background; new mail appears within seconds via IDLE;
the daemon survives restart, suspend and network loss without losing
queued operations; `katnactl send` delivers mail with a working undo delay;
integration tests pass against all dev containers.

### Phase 2 — Organizations (≈ 3 weeks)

| Task | Deliverable |
|---|---|
| 2.1 Model + rules | Tables, domain/subdomain/address rules, public-provider list, own-domain exclusion |
| 2.2 Matching | Organization views as queries over `participant`; performance tests on 1M messages |
| 2.3 Search integration | `org:` expansion, name/alias expansion for free text, facet counts |
| 2.4 Suggestions | Domain clusters, co-occurrence in threads, display-name/signature hints |
| 2.5 CLI | `katnactl org add/rule/list/show`, suggestions review |
| 2.6 vCard mapping | Organizations/contacts as vCards (`KIND:org`, `MEMBER`) — CardDAV sync comes in Phase 6 |

**Done when:** searching a company name returns mail from personal
addresses mapped to it; adding an address updates the organization view
instantly for all old mail; suggestions have a documented precision test.

### Phase 3 — Mail UI (≈ 12 weeks)

| Task | Deliverable |
|---|---|
| 3.1 App skeleton | GPUI app (pinned `gpui-pre` + GPUI Kit), daemon client, read-only store access, change-signal handling |
| 3.2 `katna-chrome` | Production version of spike S1: SSD on KDE, CSD on GNOME, others; theme tokens; Breeze-like and Adwaita-like presets |
| 3.3 `katna-platform` | Portal settings (color scheme, accent), `kdeglobals`, system font, icon theme, file chooser |
| 3.4 Main layout | Sidebar (accounts, unified inbox, folders, organizations), virtualized thread list, conversation view |
| 3.5 Rendering | Plain text + sanitized HTML (per spike S3 result), remote-content blocking, tracker removal, auth-result banners |
| 3.6 Search UI | Search-as-you-type, filter chips, organization facets, "More results on server" section |
| 3.7 Composer v1 | Plain text + Markdown, reply/reply-all/forward, identities and signatures, attachments, spell check, undo-send toast |
| 3.8 Account setup UI | Wizard using Phase 1 autoconfiguration |
| 3.9 Organizations UI | Organization pages, "Add to organization…", suggestion review |
| 3.10 Keyboard + a11y | Gmail-style shortcuts, command palette, AccessKit labels |
| 3.11 Packaging v1 | Flatpak (with Background portal), .deb, AUR (tested); .rpm (built, not tested); desktop files, AppStream, `mailto:` handler |
| 3.12 Attachment viewer | Attachment cards with thumbnails; built-in viewer for PDF, pictures, text, spreadsheets (xlsx, xls, ods, csv) and documents (docx, odt); save, open with another app, and a default app per file type in Settings (done, `ARCHITECTURE.md` §13.8) |

Started: the first window (sidebar, message list, plain-text reading pane,
search box) reads the local store; see `ARCHITECTURE.md` §13.5. The window
now follows Gmail's layout (app rail, three panes, conversations, category
tabs, quick settings, search options) and has a composer that sends
through the daemon's outbox with undo; see §13.6. GPUI Kit is
not used yet because of the size budget.

**Done when:** you can use Katna Mail as your daily client for one account
on both Plasma and GNOME (Wayland and X11); performance budgets hold
(cold start < 500 ms, binary ≤ 30 MB, idle CPU ≈ 0 %); packages install and
run on the CI distro matrix.

### Phase 4 — Notifications and desktop search (≈ 4 weeks)

| Task | Deliverable |
|---|---|
| 4.1 `katna-notify` | Notifications with click-to-open (activation tokens), inline reply-all on Plasma, archive/mark read, fallback quick-reply window |
| 4.2 Notification rules | Grouping, Inbox/category filters, per-organization policy, closing on read elsewhere |
| 4.3 Badge + tray | Unity LauncherEntry unread count, tray icon with badge and menu, single-instance app actions, KDE global menu (done early, September 2026; §15.2) |
| 4.4 KRunner | `org.kde.krunner1` in the daemon: contacts, mail, organizations; actions |
| 4.5 GNOME search | `org.gnome.Shell.SearchProvider2` using the same backend |
| 4.6 Small integrations | Global shortcut (portal), Dolphin service menu |

**Done when:** with no window open, new mail raises a notification;
clicking it focuses the right message on Wayland; reply-all from the
Plasma notification is delivered (with undo); KRunner and GNOME search
find contacts, mail and organizations in < 50 ms.

### Release track — update channels and safe updates (before the first public release)

Planned 26 September 2026 and **not started**: the basic apps come first.
The design is `ARCHITECTURE.md` §21.2. Until this track is done, only the
nightly `arch-latest` build exists and it is for testers. It must be done
before Katna is offered as a stable release or on Flathub. U.2 and U.3 are
cheap and protect testers' data too, so they may start earlier, once the
mail schema stops changing every week.

| Task | Deliverable |
|---|---|
| U.1 Versions and channels | SemVer tags (`vX.Y.Z`, `vX.Y.Z-beta.N`); nightly, beta and stable channels per format (§21.2 table); `docs/RELEASING.md` |
| U.2 Migration fixtures | A committed database of each released schema version (mail, PIM, blobs) and a test that migrates each to the current version and compares counts and query answers |
| U.3 Backup before migrating | The daemon backs up each database before raising its `user_version`, checks free space first, keeps the last two backups |
| U.4 Schema compatibility | `schema_meta.min_reader_version`; expand-then-contract rule checked in review and by a rollback test; settings keep unknown keys |
| U.5 Running while updated | Daemon notices its binary was replaced and restarts when idle; `Version()` on D-Bus; restart pill in the apps; old/new app and daemon tests |
| U.6 Health check and safe mode | First-start self-check, `health.toml`, safe mode after three failed starts, restore from backup, export of local-only data, "Copy debug report" |
| U.7 Upgrade and rollback tests in CI | Container test: previous stable → candidate with the daemon running, against the dev servers; candidate → previous stable |
| U.8 Release workflow | On a tag: build every format once, run the checks, publish to beta; promotion to stable copies the same files after a required reviewer approves |
| U.9 Signing | GPG-signed pacman packages and repository databases (`SigLevel = Required`); minisign-signed manifests and AppImages; keys only in the release environment |
| U.10 Update checks | Daemon reads the signed channel manifest (daily, not on metered networks, can be turned off); Flatpak update monitor; AppImage self-update with staged rollout and the `pulled` flag |

**Done when:** a beta built by the release workflow upgrades a running
install of the previous stable on Arch and Ubuntu without losing a message,
setting, password or queued send; installing the previous stable again
still opens the data or restores the backup; a daemon made to fail its
self-check starts in safe mode and restores the backup from its
notification; promotion to stable ships byte-identical, signed files.

### Crash reports and feedback track

Asked for by the owner on 27 September 2026 (design: `ARCHITECTURE.md`
§19.2). Crash reports on the machine come first and need no network or
consent (C.1–C.3, C.2a). Sending, usage statistics and the feedback form (C.4–C.8)
wait until the onboarding and Settings rewrites of that week have landed.
Nothing leaves the machine before the user opts in.

| Task | Deliverable |
|---|---|
| C.1 Local crash reports ✅ | `katna_core::crash`: panic hook in every binary, `coredumpctl` lookup for native crashes, scrubber (home, user, host, machine ID, email addresses), one text report per crash in `$XDG_STATE_HOME/katna/crashes/` with raw frames and build ID, newest 20 kept; `feedback.save_crash_reports` (default on) |
| C.2 Crash notice ✅ | Next start of Katna Mail after a crash of the app or the daemon: "closed unexpectedly last time" with View report and Copy report; `katnactl crashes` lists, prints and deletes reports |
| C.2a User feedback tab ✅ | Settings > User feedback (tab before Experimental): "Save crash reports on this computer" (default on) and the saved reports with View, Copy, Delete and Delete all; the tab itself comes from the Settings rewrite |
| C.3 Readable stacks | Measure `strip = "debuginfo"` against the size budgets; CI keeps each build's debug files (by build ID) and, once the Sentry project exists, uploads them with `sentry-cli` |
| C.4 Asking | "Help improve Katna" step in onboarding (Share / Don't share, equal weight, no default); asked once after updating for existing installs; Settings > User feedback switches "Send crash reports" and "Send anonymous usage statistics", off until the user opts in, changeable at any time |
| C.5 Sending crash reports | Daemon uploads envelopes to the DSN over `rustls` only when the switch is on and the network is not metered; Send / Always send in the crash notice; an empty DSN turns sending off |
| C.6 Usage statistics | One enum of features, weekly `info` event with yes/no feature tags and bucketed facts, release-health sessions, random install ID rotated every 90 days and resettable; Settings > User feedback shows what is counted |
| C.7 Feedback form | Help > Send feedback (global menu, Quick settings > Help, Settings > User feedback): form, optional reply address, preview of exactly what is sent, Sentry User Feedback item |
| C.8 Sentry project | Project `invenia-systems/4512156171698256` created (owner, 27 September 2026); organization-wide Require Data Scrubber, Require Using Default Scrubbers and Prevent Storing of IP Addresses on (done); GitHub integration; DSN filled in `katna_core::ids`; `SENTRY_AUTH_TOKEN` secret added by the owner for C.3's debug-file upload |
| C.9 Own server (later) | GlitchTip or self-hosted Sentry on `katna.invenia.in` with the same settings as C.8; CI uploads debug files there; the DSN constant switches to it; the cloud project is closed once no supported version sends to it |

**Done when:** a panic and a segfault in Katna Mail and in the daemon each
leave a readable report with a stack and no personal data, and Katna Mail
offers it on the next start; with sharing off nothing is sent (checked with
a recording proxy); with sharing on, the same report appears in Sentry
with function names and lines, and the weekly statistics event carries
only the documented fields.

### Later: promotional website (not scheduled yet)

Asked for by the owner on 26 September 2026 so it is not forgotten; nothing
is built. A public site at `katna.invenia.in` that says what Katna is (a
fast, private mail and calendar suite for Linux desktops), shows
screenshots and short clips of Katna Mail and Katna Calendar, explains how
to install it (today the `arch-latest` pre-release and its `[katna]`
pacman repository; later the beta and stable channels, Flatpak and
AppImage) and links to the source, license and issue tracker. It should be
ready by the first public release, alongside the release track.

Open questions: static site generator and hosting (GitHub Pages or our own
server); whether the site also serves the update manifests and package
repositories (release track U.9, U.10); a Katna logo and brand look;
languages; and no trackers or third-party analytics, to match Katna's
privacy promise.

### Later: Katna on phones (not scheduled yet)

Asked about by the owner on 27 September 2026; design only, in
`ARCHITECTURE.md` §26. Nothing starts until the owner asks for it. M.1 and
M.2 also help the desktop (a daemon-free engine is easier to test), so they
may start earlier if a desktop task needs them.

| Task | Deliverable |
|---|---|
| M.1 Engine split | `katna-engine` library out of `katna-daemon` (no D-Bus, systemd or GPUI); `PimClient` trait with the D-Bus client and an in-process one; `sync_once(deadline)` |
| M.2 Portability | Sandbox paths in `katna-core`; `SecretStore` trait (Secret Service, Android Keystore, iOS Keychain); network and metered events behind a trait; OpenPGP without `gpg` |
| M.3 Spike: GPUI on Android | Time-boxed (1–2 weeks). `gpui-mobile` on Katna's GPUI 0.3.6; Katna Mail's phone layout on an emulator and a real phone. Measure start time, scrolling, APK size, idle battery, and typing in English and Bengali. Result in `docs/spikes/` |
| M.4 Android app | Host activity, lifecycle and state restore, touch gestures, IME, insets, notifications with actions, share sheet, file and photo pickers, OAuth in the browser |
| M.5 Android new mail | Foreground IDLE service (`specialUse`), WorkManager timer and Manual; Settings → New mail with honest costs |
| M.6 Push | Web Push (RFC 8030/8291) receiver; UnifiedPush on Android; JMAP push subscriptions; "Wake my phone" in `katna-daemon`; the watcher in Katna Server |
| M.7 Push gateway and iOS spike | Stateless gateway (sealed APNs tokens, no logs); `gpui_ios` or `gpui-mobile` on iOS; Notification Service Extension |
| M.8 iOS app | After the licensing decision (§26.6): TestFlight build with push, background refresh and the same features as M.4 |
| M.9 Distribution | GitHub APKs and F-Droid, then Google Play; App Store |

**Done when:** on an Android phone without Google services, with Katna
swiped away, new mail in a Stalwart account and a Gmail account raises a
notification within a minute, and a day of idle costs no more battery than
FairEmail on the same phone; on an iPhone a JMAP account notifies within a
minute through server push and a Gmail account through the owner's Katna
desktop; no Katna-run server ever holds a password, a token or a message.

### Phase 5 — Gmail-class features (≈ 8 weeks)

Labels (IMAP keywords, Gmail labels), snooze, send later, follow-up
reminders, rules engine + Sieve/ManageSieve upload, vacation responder,
one-click unsubscribe (RFC 8058), templates, mute thread, inbox categories
(local classifier), phishing/lookalike warnings, OAuth2 for Gmail/Microsoft
(verification process started in parallel).

**Done when:** the feature checklist in the architecture (§10, §11) works
against Stalwart, Dovecot and Gmail, with integration tests.

Later, not scheduled yet: notes on mail and the Workspace view
(`ARCHITECTURE.md` §13.7). Workspace builds on snooze from this phase.

### Phase 6 — Katna Calendar and Plasma calendar (≈ 14 weeks)

| Task | Deliverable |
|---|---|
| 6.1 `katna-dav` | CalDAV/CardDAV discovery and sync in the daemon; contacts/organizations sync via CardDAV |
| 6.2 Calendar core | `calcard` parsing, recurrence expansion (`rrule`), exceptions, time zones (`jiff`), alarms as notifications; property tests + fuzzing |
| 6.3 Invitations | iTIP/iMIP: accept/decline from Katna Mail, send invitations |
| 6.4 Katna Calendar app | Day/week/month/agenda views, event editor, organization filter |
| 6.5 Plasma events plugin | C++ `CalendarEventsPlugin` → daemon; Katna Calendar as `text/calendar` handler |
| 6.6 Katna Clock | Fork of `applets/digital-clock`: renames, `X-Plasma-Provides`, quick-add, click/right-click actions, drag to reschedule |
| 6.7 Upstream proposals | Merge requests to Plasma: "Add…" with date, click event to open, plugin action hook |
| 6.8 Packaging | `katna-plasma-integration` for .deb and AUR; CI against Arch's Plasma and Kubuntu 26.04's Plasma |

**Done when:** events sync with Google, Nextcloud and Fastmail; recurring
events with exceptions and time zones match a reference test set; events
appear in the stock Plasma clock; events can be added and edited from
Katna Clock.

### Phase 7 — Katna Server (≈ 8 weeks)

Server (axum + PostgreSQL), accounts/auth, metadata sync with E2E-encrypted
values, event stream to the daemon, open/link tracking with bot/scanner
labeling, per-recipient sending in the daemon, activity dashboard in Katna
Mail, optional server-side send later/snooze, container image for
self-hosting.

**Done when:** tracked mail reports opens and clicks with Apple/scanner
labeling; the server stores no content; a self-hosted instance runs from
the published container image.

### Phase 8 — Polish (ongoing)

Full-fidelity HTML rendering, WYSIWYG composer, semantic search
(local embeddings), Katna Confidential, large-attachment links, GNOME
top-bar calendar (EDS backend), more languages.

Encrypted mail (asked for early, 2026-09-26; ARCHITECTURE §19.1), through
the user's GnuPG:

| Task | Content |
|---|---|
| E.1 Read encrypted and signed mail ✅ | `katna-crypto`: PGP/MIME, inline PGP, S/MIME via `gpg`/`gpgsm`; banner in the reading view; armor left out of snippets and search |
| E.2 Sign and encrypt when sending ✅ | Compose toggles, recipient key check, encrypt to self, hidden Bcc, PGP/MIME and S/MIME; answers to encrypted mail encrypted. Later: per-account defaults in Settings |
| E.3 Keys | Autocrypt headers, WKD lookup, import keys from attachments, key details in the banner, protected (hidden) subject |

**Done when:** mail from Thunderbird and KMail (OpenPGP and S/MIME,
signed, encrypted, both) opens and verifies in Katna, and mail Katna sends
opens and verifies in both.

## 6. Timeline overview

```
Month:     1    2    3    4    5    6    7    8    9   10   11   12  …
Spikes    ██
Phase 0   ████
Phase 1        ██████████
Phase 2                  ███
Phase 3                     ████████████
Phase 4                                 ████
          ───────────── first usable release (mail only) ≈ month 9 ─────────────
Phase 5                                     ████████
Phase 6                                             ██████████████ …
Phase 7                                   (can run in parallel with a second developer)
```

Rough totals for one full-time developer: **first usable Katna Mail
release (Phases 0–4) ≈ 8–9 months**; full plan through Phase 7 ≈ 20–24
months. A second developer is most useful on Katna Server (Phase 7) or the
Plasma integration (Phase 6), which have few dependencies on the rest.

## 7. Testing strategy

| Layer | Approach |
|---|---|
| Parsers (query, MIME wrapper, iCalendar, vCard) | Unit tests, property tests (`proptest`), fuzzing (`cargo-fuzz`) |
| Store | Migration tests (every schema version upgrades cleanly), crash-safety tests |
| Updates | Migration fixtures of every released schema, upgrade and rollback tests in containers, old/new app and daemon over D-Bus (release track U.2, U.7) |
| Search | Relevance test set (queries with expected top results), Enron benchmarks |
| Sync | Integration tests against Stalwart, Dovecot, Radicale in CI; recorded sessions for Gmail/Fastmail quirks |
| Threading, recurrence | Property tests and reference data sets |
| Daemon | D-Bus API tests with a private session bus; resource tests (idle CPU, wake-ups) |
| UI | GPUI view tests (`TestAppContext`); screenshot comparisons for key screens on both presets |
| Plasma integration | Build + load tests against each supported Plasma version; manual test checklist |
| Packaging | Install-and-launch smoke test per format in CI containers |

## 8. First two weeks (concrete checklist)

1. ✅ All pre-coding decisions D1–D6 are made.
2. ✅ Repo renamed; `LICENSE`, `README`, `CLAUDE.md` added.
3. ✅ Cargo workspace with empty crates and the dependency rules.
4. ✅ Toolchain (latest stable); CI `check` (Arch + Ubuntu 26.04), `deny`, `size`.
5. ✅ Add `dev/compose.yaml` with Stalwart, Dovecot, Radicale, Mailpit and seed data.
6. Start spike **S2** (Pimalaya + I/O) and spike **S1** (window chrome).
   S1 done: `docs/spikes/S1-window-chrome.md`.
7. ✅ Implement `katna-core` and the first `katna-store` migration (secrets move to task 1.2; `calendar.db` schema to Phase 6).
8. ✅ Write the Enron download script and the Maildir importer.
9. Create GitHub milestones (Phases 0–8) and issues for Phase 0 tasks.

## 9. How this plan is maintained

- Update estimates at the end of each phase with actual numbers.
- Spike results and major decisions are recorded as short ADRs
  (architecture decision records) in `docs/adr/`.
- If a "done when" check cannot be met, the plan is changed explicitly —
  never silently skipped.
