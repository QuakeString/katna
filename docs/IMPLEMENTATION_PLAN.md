# Katna PIM — Implementation Plan

> Status: **v0.2** (updated 27 September 2026, through PR #143). Companion to
> [ARCHITECTURE.md](ARCHITECTURE.md), which defines *what* we build. This
> document defines *in which order*, *how we know a step is done*, and *how
> we work*.

## 0. Where we are (27 September 2026)

✅ marks a task merged to `main`, with its pull requests. ◐ marks a task
that is partly done; the table says what is left. Rows without a mark have
not started. The plan is refreshed after each batch of merges. The
"Daily use" track in §5 lists work the owner asked for while using Katna
Mail that the phases did not name.

- **Done:** spikes S1, S2; Phase 0; Phase 1 except `katna-meta` as its own
  crate; most of Phase 3 (Katna Mail is in daily use on the owner's Plasma
  6.7 Wayland laptop with a real Gmail account through the `arch-latest`
  package); notifications, badge, tray and global menu from Phase 4;
  local crash reports and opt-in sending; the language framework and most
  of the UI translated; reading and sending encrypted mail.
- **Merged since the last refresh:** select all matching (#127),
  recipient chips (#130), What's new in the chosen language (#134),
  Autostart on by default (#136), rich paste and drag and drop (#137), the
  new Katna logo (#138), the next conversation after delete, archive or
  move (#139), account names, pictures and order (#140), translations of
  the newer text (#141, #142), dimmed list markers until hover (#143) and
  Reset cache (#109).
- **In review:** Buy me a coffee in the README (#144) and in About (#145).
- **Next:** usage statistics, feedback form and debug-file upload (C.3,
  C.6, C.7); right-to-left layout (L.2, L.3); OAuth2; organizations
  (Phase 2); KRunner and GNOME search (4.4, 4.5); the release track before
  any public release.
- **Later:** Katna Calendar (Phase 6), Contacts, Tasks, Notes, Feeds,
  phones, notes on mail, Workspace, Katna Server, own crash server, a
  server check that recipient addresses exist.

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
| D7 | Where crash reports and feedback go | **Decided: a Sentry cloud project now, our own server later** (owner, 27 September 2026): self-hosted GlitchTip or Sentry on `katna.invenia.in` replaces it by changing the DSN. Only after the user opts in; DSN in `katna_core::ids::SENTRY_DSN` since 27 September 2026 (`ARCHITECTURE.md` §19.2) | Crash-report track C.5–C.9 |

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
| 1.1 I/O layer ✅ #4, #8 | rustls + async I/O driver for Pimalaya's sans-I/O coroutines; `MailBackend` trait |
| 1.2 Account setup ✅ #23, #24, #29 | Autoconfiguration (Thunderbird ISPDB, provider autoconfig, DNS SRV per RFC 6186), password login, secrets in Secret Service (`oo7`) |
| 1.3 IMAP level 1 ✅ #11, #31, #34, #37 | Folder list, envelope/flags/`BODYSTRUCTURE` sync, CONDSTORE/QRESYNC incremental sync, UIDVALIDITY handling |
| 1.4 IMAP push ✅ #13 | IDLE with renewal, per-folder connections within server limits, reconnect/backoff |
| 1.5 Levels 2 and 3 ✅ #16, #36, #59, #87 | Offline window (full bodies), text backfill for indexing, eviction when mail leaves the window |
| 1.6 Op queue ✅ #20 | Optimistic local flags/move/delete, replay with retries and conflict handling |
| 1.7 Threading ✅ #25, #34, #72 | JWZ threading + Gmail thread IDs; property tests |
| 1.8 SMTP + outbox ✅ #22 | Sending, Sent-folder handling, outbox with undo delay |
| 1.9 `katna-meta` ◐ undo send and scheduled send live in the outbox (#22, #53); the crate is still empty | Metadata table + scheduler (undo send first) |
| 1.10 POP3 ✅ #27 | Client with UIDL tracking, leave-on-server, `TOP` (header-first partial download of large messages comes later) |
| 1.11 `katna-daemon` ✅ #15, #17, #19, #65 | Process, `in.invenia.katna.Pim1` D-Bus skeleton (commands + change signals), single instance, systemd user unit, D-Bus activation, graceful shutdown |
| 1.12 System events ✅ #30, #38 | Network changes, suspend/resume, metered connections |
| 1.13 `katnactl` ✅ #15, #28, #33 | Small CLI client for the daemon (add account, sync, search, send, list) — the test harness until the GUI exists |

Status (27 September 2026): all tasks merged except `katna-meta` as its
own crate. Gmail syncs in the background on the owner's laptop; the
integration tests run against Stalwart in CI.

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

Status (27 September 2026): not started; `katna-org` is an empty crate.

**Done when:** searching a company name returns mail from personal
addresses mapped to it; adding an address updates the organization view
instantly for all old mail; suggestions have a documented precision test.

### Phase 3 — Mail UI (≈ 12 weeks)

| Task | Deliverable |
|---|---|
| 3.1 App skeleton ✅ #18 | GPUI app (pinned `gpui-pre` + GPUI Kit), daemon client, read-only store access, change-signal handling |
| 3.2 `katna-chrome` ✅ #7, #60, #75, #77, #83 | Production version of spike S1: SSD on KDE, CSD on GNOME, others; theme tokens; Breeze-like and Adwaita-like presets |
| 3.3 `katna-platform` ✅ #40 | Portal settings (color scheme, accent), `kdeglobals`, system font, icon theme, file chooser |
| 3.4 Main layout ◐ #18, #21, #25, #49, #57, #61, #68, #85, #90, #127, #132, #139, #143; organizations in the sidebar wait for Phase 2 | Sidebar (accounts, unified inbox, folders, organizations), virtualized thread list, conversation view |
| 3.5 Rendering ◐ #44, #73, #103; auth-result banners pending | Plain text + sanitized HTML (per spike S3 result), remote-content blocking, tracker removal, auth-result banners |
| 3.6 Search UI ◐ #18, #50, #54, #88; organization facets and "More results on server" pending | Search-as-you-type, filter chips, organization facets, "More results on server" section |
| 3.7 Composer v1 ✅ #25, #42, #53, #69, #84, #107, #110, #111, #117, #118, #126, #130, #133, #137 | Plain text + Markdown, reply/reply-all/forward, identities and signatures, attachments, spell check, undo-send toast |
| 3.8 Account setup UI ✅ #29, #47, #51 | Wizard using Phase 1 autoconfiguration |
| 3.9 Organizations UI | Organization pages, "Add to organization…", suggestion review |
| 3.10 Keyboard + a11y ◐ shortcuts #42, #71, #112; command palette and AccessKit labels pending | Gmail-style shortcuts, command palette, AccessKit labels |
| 3.11 Packaging v1 ◐ Arch package and `[katna]` repository #19, #32; `mailto:` handler #106; Flatpak, .deb, .rpm, AppStream pending | Flatpak (with Background portal), .deb, AUR (tested); .rpm (built, not tested); desktop files, AppStream, `mailto:` handler |
| 3.12 Attachment viewer ✅ #52, #55, #66, #79, #99, #102, #131 | Attachment cards with thumbnails; built-in viewer for PDF, pictures, text, spreadsheets (xlsx, xls, ods, csv), documents (docx, doc, odt) and slides as text (pptx, ppt, odp); save, open with another app, and a default app per file type in Settings (done, `ARCHITECTURE.md` §13.8) |

Started: the first window (sidebar, message list, plain-text reading pane,
search box) reads the local store; see `ARCHITECTURE.md` §13.5. The window
now follows Gmail's layout (app rail, three panes, conversations, category
tabs, quick settings, search options) and has a composer that sends
through the daemon's outbox with undo; see §13.6. GPUI Kit is
not used yet because of the size budget.

Status (27 September 2026): the owner uses Katna Mail daily with a Gmail
account on Plasma 6.7 Wayland. Left before the "done when": organizations
UI (3.9, after Phase 2), auth-result banners, server search results,
command palette and AccessKit, packages other than Arch, and checks on
GNOME and X11. The binary budget was raised to 100 MB (57 MB today)
because GPUI with its Linux backends is larger than planned (spike S1).

**Done when:** you can use Katna Mail as your daily client for one account
on both Plasma and GNOME (Wayland and X11); performance budgets hold
(cold start < 500 ms, binary ≤ 30 MB, idle CPU ≈ 0 %); packages install and
run on the CI distro matrix.

### Phase 4 — Notifications and desktop search (≈ 4 weeks)

| Task | Deliverable |
|---|---|
| 4.1 `katna-notify` ✅ #41, #80 | Notifications with click-to-open (activation tokens), inline reply-all on Plasma, archive/mark read, fallback quick-reply window |
| 4.2 Notification rules | Grouping, Inbox/category filters, per-organization policy, closing on read elsewhere |
| 4.3 Badge + tray ✅ #48, #65, #70 | Unity LauncherEntry unread count, tray icon with badge and menu, single-instance app actions, KDE global menu (done early, September 2026; §15.2) |
| 4.4 KRunner | `org.kde.krunner1` in the daemon: contacts, mail, organizations; actions |
| 4.5 GNOME search | `org.gnome.Shell.SearchProvider2` using the same backend |
| 4.6 Small integrations | Global shortcut (portal), Dolphin service menu |

Status (27 September 2026): 4.1 and 4.3 are done and translated (#129);
4.2 is not planned in detail yet; 4.4–4.6 not started.

**Done when:** with no window open, new mail raises a notification;
clicking it focuses the right message on Wayland; reply-all from the
Plasma notification is delivered (with undo); KRunner and GNOME search
find contacts, mail and organizations in < 50 ms.

### Daily use track — requests from the owner's own use (September 2026)

Since #43 the owner has used each `arch-latest` build on Plasma 6.7
Wayland with a busy Gmail account and asked for changes along the way. Most
belong to Phase 3 tasks above; this track records them so none is lost.

| Task | Deliverable |
|---|---|
| D.1 Gmail look and motion ✅ | Gmail-style layout, conversations, tabs and motion (#21, #25, #45, #64, #68, #78, #81); folder pane icon (#90); card outline, shadow and equal 16 px gaps (#85, #120, #128); one account at a time (#61); pins (#57) |
| D.2 Settings ✅ | Settings page, tabs, search, (i) help, long names (#42, #47, #82, #88, #96, #124); 12/24-hour clock (#115); reading options (#108); shortcut sets and Mailspring settings (#112); scaling (#104) |
| D.3 Onboarding, What's new, About ✅ | First-start pages and tour (#51), What's new after updates, one file per highlight (#89, #93, #113, #123), About Katna (#93) |
| D.4 Window ✅ | Open in a new window (#57, #86), window memory (#105), print preview before the print dialog, More menu (#122), own frame and blur, Experimental (#75, #77) |
| D.5 Undo everywhere ✅ | Undo in the snackbar and Ctrl+Z for archive, delete, move, spam, read, star, important, pin and a send during its undo delay (#76, #125) |
| D.6 Selecting text ✅ | Select and copy mail text (#103) and viewer text; spreadsheet cells copy as cells (#131) |
| D.7 Compose placement ✅ | Compose under the account name in the folder pane, sliding into the rail when folded; "Katna Mail" at the top left with the app word changing on app switch (#132) |
| D.8 Select all matching ✅ | Select menu choices tick every matching conversation in the folder or tab, count line and Clear, in batches of 500 (#127) |
| D.9 Recipient chips ✅ | Chips in To, Cc and Bcc, name first with the address on demand, double-click to edit, invalid addresses in red blocking Send (#130) |
| D.10 Rich paste and drag and drop ✅ | Paste keeps formatting; spreadsheet tables paste as Table, Picture or Plain; files pasted or dragged from Dolphin attach; pictures ask Inline or Attachment (#137) |
| D.11 New Katna logo ✅ | The owner's own logo everywhere: app, hicolor, window and tray icons, top bar, About, onboarding, README (#138); the website page follows in `invenia_website` |
| D.12 Autostart on by default ✅ | The daemon and tray start with the session unless turned off, quietly in the tray (#136) |
| D.13 Reset cache ✅ | Settings > Reset cache, as in Mailspring (#109) |
| D.14 Whole-sentence suggestions (later) | Optional, downloaded small local model in a helper process; nothing sent to a cloud service without asking |
| D.15 Next conversation after an action ✅ | Delete, archive or move opens the next conversation instead of going back to the list (#139) |
| D.16 Accounts in Settings ✅ | Account names, own pictures and order in Settings > Accounts (#140) |
| D.17 Quiet list markers ✅ | Tick boxes, stars and Important markers dim until the row is hovered or they are on, as in Gmail (#143) |
| D.18 Support links | Buy me a coffee and a GitHub Sponsor button in the README (#144), a button and QR code in About (#145) (in review) |

Not yet checked on a real desktop: Open with (#55), Gmail Important sync
(#56), scheduled send (#53), the badge count with one account (#61), the
print hand-off to KDE's print dialog (#122) and reopening the window in
place on Plasma 6.7 Wayland (#105).

### Release track — update channels and safe updates (before the first public release)

Planned 26 September 2026 (#39) and **not started** apart from the
daemon's restart after updates (#65): the basic apps come first.
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
| U.5 Running while updated ◐ the daemon restarts itself after an update (#65); the rest pending | Daemon notices its binary was replaced and restarts when idle; `Version()` on D-Bus; restart pill in the apps; old/new app and daemon tests |
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
consent (C.1–C.3, C.2a). Asking and sending crash reports (C.4, C.5) came
next; usage statistics and the feedback form (C.6, C.7) follow. Nothing
leaves the machine before the user opts in.

| Task | Deliverable |
|---|---|
| C.1 Local crash reports ✅ #95 | `katna_core::crash`: panic hook in every binary, `coredumpctl` lookup for native crashes, scrubber (home, user, host, machine ID, email addresses), one text report per crash in `$XDG_STATE_HOME/katna/crashes/` with raw frames and build ID, newest 20 kept; `feedback.save_crash_reports` (default on) |
| C.2 Crash notice ✅ #95 | Next start of Katna Mail after a crash of the app or the daemon: "closed unexpectedly last time" with View report and Copy report; `katnactl crashes` lists, prints and deletes reports |
| C.2a User feedback tab ✅ #96, #101 | Settings > User feedback (tab before Experimental): "Save crash reports on this computer" (default on) and the saved reports with View, Copy, Delete and Delete all; the tab itself comes from the Settings rewrite |
| C.3 Readable stacks and debug-file upload (next) | Measure `strip = "debuginfo"` against the size budgets; CI keeps each build's debug files (by build ID) and, once the Sentry project exists, uploads them with `sentry-cli` |
| C.4 Asking ✅ #101 | "Help improve Katna" step in onboarding (Don't send / Send crash reports, equal weight, no default); asked once after updating for existing installs; Settings > User feedback switch "Send crash reports", off until the user opts in, changeable at any time ("Send anonymous usage statistics" comes with C.6) |
| C.5 Sending crash reports ✅ #101 | Daemon sends new reports as hand-written envelopes (no SDK) over `rustls` only when the switch is on and the network is not metered: 20 s after start, every 15 minutes and when settings are saved; reports of the last 7 days, marked "Sent" in the list; an empty DSN turns sending off |
| C.6 Usage statistics (next) | One enum of features, weekly `info` event with yes/no feature tags and bucketed facts, release-health sessions, random install ID rotated every 90 days and resettable; Settings > User feedback shows what is counted |
| C.7 Feedback form (next) | Help > Send feedback (global menu, Quick settings > Help, Settings > User feedback): form, optional reply address, preview of exactly what is sent, Sentry User Feedback item |
| C.8 Sentry project ◐ waits only for the `SENTRY_AUTH_TOKEN` secret | Project `invenia-systems/4512156171698256` created (owner, 27 September 2026); organization-wide Require Data Scrubber, Require Using Default Scrubbers and Prevent Storing of IP Addresses on (done); GitHub integration; DSN filled in `katna_core::ids` (done); `SENTRY_AUTH_TOKEN` secret added by the owner for C.3's debug-file upload |
| C.9 Own server (later) | GlitchTip or self-hosted Sentry on `katna.invenia.in` with the same settings as C.8; CI uploads debug files there; the DSN constant switches to it; the cloud project is closed once no supported version sends to it |

**Done when:** a panic and a segfault in Katna Mail and in the daemon each
leave a readable report with a stack and no personal data, and Katna Mail
offers it on the next start; with sharing off nothing is sent (checked with
a recording proxy); with sharing on, the same report appears in Sentry
with function names and lines, and the weekly statistics event carries
only the documented fields.

### Languages track

Asked for by the owner on 27 September 2026 (design: `ARCHITECTURE.md`
§13.10): 51 picker entries, 49 translations, the whole layout mirrored for
Arabic, Persian, Hebrew and Urdu. Many threads change the UI at the same
time, so strings move to Fluent area by area in small pull requests, each
merging `main` first, rather than in one large one.

| Task | Deliverable |
|---|---|
| L.1 Framework and picker ✅ #100 | `katna-i18n` crate: language list (`i18n/languages.toml`), system language (`LANGUAGE`, `LC_*`, `plasma-localerc`), `general.language` setting, Fluent bundles with English fallback, `tr!`, override folder, pseudo-locales, id and variable checks; ICU4X dates, numbers and plurals in `format.rs`; top-bar language button and picker popover, Settings > General row, phone drawer row; bundled flags; the top bar and the picker translated into all 48 languages; CLAUDE.md rule that new UI text goes through `tr!` |
| L.2 GPUI patches | `gpui-pre` vendored with `KATNA.md`: window layout direction with mirrored bounds, start/end text alignment, `.layout_ltr()`, UAX #14 and grapheme-safe line breaking with dictionary breaks for Thai, Lao, Khmer and Burmese, Han forms from Katna's language, bidi-aware carets |
| L.3 Mirroring | RTL switch in `window/layout.rs` `Shape`; mirrored directional icons; drawer, conversation and menus from the right; arrow keys; checked with `qps-plocm` |
| L.4 Strings, by area ◐ main window #114, Settings #116, dialogs and search options #119, compose #121, notifications and tray #129; What's new #134; account settings, paste and drop and newer text #141, #142; onboarding, About, viewers and `.desktop` names pending | One pull request per area, each with its 48 drafted translations: message list and toolbar; reader and attachments; compose and signatures; search and search options; Settings (each tab); accounts, onboarding, What's new, About, crash notice; viewers; global menu, dock menu, tray, notifications and the daemon; `.desktop` file names and actions |
| L.5 Mail content | Per-message and per-paragraph direction in the reader and list; compose direction buttons and `dir` in sent HTML; quote and forward headers in the UI language; shortcuts by key position on non-Latin layouts; input method check (Fcitx5, IBus) |
| L.6 Search in unspaced scripts | Thai, Lao, Khmer and Burmese word splitting in `katna-search` with the shared segmenter |
| L.7 Corrections | Translation guide (`i18n/README.md`), "Translation correction" issue template, coverage report in CI; later hosted Weblate on the same files (owner applies) |
| L.8 Review | Native speakers review the machine drafts; `languages.toml` marks each reviewed language and the picker drops its "Translated by machine" note |

**Done when:** with the desktop set to Bengali, Katna Mail, its
notifications, tray and global menu start in Bengali with Bengali dates
and digits, and picking English (US), English (UK) and English (India) in
the top-bar picker switches at once to their formats; in Arabic, Hebrew,
Persian and Urdu the whole window is mirrored with nothing overlapping or
clipped, while an English mail still reads left to right; screenshots of
Hindi, Bengali, Tamil, Thai, Khmer, Burmese, Lao, Dzongkha, Amharic,
Arabic, Japanese and Korean show correctly joined text that wraps only
between words; `qps-ploc` finds no untranslated text in the converted
areas; and the app and the daemon stay within their size budgets.

### Later: promotional website (not scheduled yet)

Asked for by the owner on 26 September 2026 (#62). Started 27 September
2026 as a `/katna` page on the owner's own site (repository
`QuakeString/invenia_website`, PR #1, in 49 languages), which the owner
builds and deploys; `katna.invenia.in` pointing there is the owner's
server setting. Not merged by design until the owner is ready. A public site at `katna.invenia.in` that says what Katna is (a
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

### Later: check that a recipient's address exists (not scheduled yet)

Asked for by the owner on 27 September 2026 so it is kept for later;
nothing is built. The owner wants to run
[check-if-email-exists](https://github.com/reacherhq/check-if-email-exists)
(Reacher) on our own server to check that a recipient's mailbox really
exists before sending, so a typo does not send mail to the wrong person.
It checks the syntax, the domain's MX records, then asks the recipient's
mail server over SMTP whether the mailbox exists, without sending a
message. It also flags disposable, role, catch-all, full and disabled
mailboxes. It is a Rust crate, a CLI, and an HTTP backend in Docker
(`POST /v0/check_email`).

Where it would sit: recipient chips (#130) already check the address
format as it is typed and show invalid addresses in red, blocking Send.
This check would add a second, slower step after a chip is made: the
daemon (the only part that talks to the network) asks our server and the
chip gets a quiet mark for "address not found" or "could not check".
Before sending, the send checks (#117) would ask "This address may not
exist. Send anyway?", as they do for a missing attachment, and never block
the send, because the answer is often "unknown". Addresses already in the
user's own mail (address suggestions, #110) would not be checked again.

Open questions:

- **Licence.** AGPL-3.0, or a paid commercial licence. Katna is
  GPL-3.0-or-later, which may be combined with AGPL-3.0 code. Running it
  as a separate server we host keeps its source obligations on that
  server (we publish our changes to it). Linking the crate into
  `katna-daemon` would put AGPL terms on the daemon, so the server route
  is preferred.
- **What the server needs.** Outbound port 25 open, which many cloud hosts
  block; a clean IP address with reverse DNS. Large providers rate-limit
  or block servers that probe many mailboxes. The README says anything
  beyond small volumes needs SMTP proxies (a paid third-party service).
- **How reliable.** Gmail, Outlook and Yahoo often answer "unknown" or
  accept every address (catch-all), so a "not found" is a hint, never a
  hard error.
- **Privacy.** Every checked address would reach our server and the
  recipient's mail server would see our server asking. It must be off
  until the user turns it on (like crash reports, C.4). The request should
  carry only the address, with no logs kept. The Gravatar and Have I Been
  Pwned lookups stay off.
- **Abuse.** The endpoint must only answer Katna users (a per-install
  token and rate limits), or it becomes a free address-harvesting service.
- **Hosting.** Probably on `katna.invenia.in` next to the self-hosted
  crash server (C.9) and Katna Server (Phase 7), whose accounts could
  authenticate it.

### Later: Katna on phones (not scheduled yet)

Asked about by the owner on 27 September 2026; design only (#94), in
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

Status (27 September 2026): done early: labels and folders created on the
server (#58), Gmail labels stored once (#34), Gmail's inbox tabs (#31),
send later (#53), pins (#57), Undo on every action (#125). Not started:
snooze, follow-up reminders, rules and Sieve, vacation responder,
one-click unsubscribe, templates, mute, a local category classifier,
phishing warnings and **OAuth2** (Gmail works with an app password today).

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

Started early: HTML rendering (#44), a formatting composer with tables
(#53), spelling, grammar with Harper (#111) and writing suggestions (#118),
scaling 75–200 % (#104). Still open: Full-fidelity HTML rendering, WYSIWYG composer, semantic search
(local embeddings), Katna Confidential, large-attachment links, GNOME
top-bar calendar (EDS backend), more languages.

Encrypted mail (asked for early, 2026-09-26; ARCHITECTURE §19.1), through
the user's GnuPG:

| Task | Content |
|---|---|
| E.1 Read encrypted and signed mail ✅ #46 | `katna-crypto`: PGP/MIME, inline PGP, S/MIME via `gpg`/`gpgsm`; banner in the reading view; armor left out of snippets and search |
| E.2 Sign and encrypt when sending ✅ #46 | Compose toggles, recipient key check, encrypt to self, hidden Bcc, PGP/MIME and S/MIME; answers to encrypted mail encrypted. Later: per-account defaults in Settings |
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
6. ✅ Spike **S2** (Pimalaya + I/O, #4) and spike **S1** (window chrome, #7):
   `docs/spikes/`. S3 and S4 were answered by building (#44, #41).
7. ✅ Implement `katna-core` and the first `katna-store` migration (secrets move to task 1.2; `calendar.db` schema to Phase 6).
8. ✅ Write the Enron download script and the Maildir importer.
9. Create GitHub milestones (Phases 0–8) and issues for Phase 0 tasks.

## 9. How this plan is maintained

- Update estimates at the end of each phase with actual numbers.
- §0 and the status marks (✅ with pull request numbers, ◐ partly done) are
  refreshed after each batch of merges, in one pull request at a time, so
  parallel work does not race on this file.
- Spike results and major decisions are recorded as short ADRs
  (architecture decision records) in `docs/adr/`.
- If a "done when" check cannot be met, the plan is changed explicitly —
  never silently skipped.
