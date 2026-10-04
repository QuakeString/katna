# Katna PIM — Implementation Plan

> Status: **v0.2** (updated 4 October 2026, through PR #688). Companion to
> [ARCHITECTURE.md](ARCHITECTURE.md), which defines *what* we build. This
> document defines *in which order*, *how we know a step is done*, and *how
> we work*.

## 0. Where we are (4 October 2026)

✅ marks a task merged to `main`, with its pull requests. ◐ marks a task
that is partly done; the table says what is left. Rows without a mark have
not started. The plan is refreshed after each batch of merges. The
"Daily use" track (D.1–D.58) and the "Added along the way" track (A.1–A.19)
in §5 list work the owner asked for while using Katna that the phases did
not name.

- **Done:** spikes S1, S2; Phase 0; Phase 1; most of Phase 3 (Katna Mail is in daily
  use on the owner's Plasma 6.7 Wayland laptop with a real Gmail account
  through the `arch-latest` package); notifications, badge, tray, global
  menu, KRunner and GNOME search from Phase 4; templates, snooze,
  follow-up reminders, send later and mail rules from Phases 5 and 7; Katna Server
  with accounts, open and click tracking, Activity with mailbox insights
  and automatic translation (Phase 7);
  local crash reports and opt-in sending; the language framework and most
  of the UI translated; reading and sending encrypted mail; Phase 6
  except the upstream proposals: Calendar, Tasks, Notes and Contacts pages that sync with each
  account's own service; the Windows build and installer.
- **Merged since the last refresh (#523–#688):** Tasks with Upcoming,
  Completed, sorting, labels, files and a synced star, and quick capture
  of a task or note from anywhere (6.4, 4.6); Notes with pictures,
  reminders, links, history, export and AI (6.9); Settings sorted into all
  apps and each app's pages (A.10; a Settings window of its own was tried
  in #676 and taken back in #681); text selectable across the app (A.10);
  signatures imported from Gmail, Thunderbird, Evolution and KMail, twelve
  signature layouts, and designed signatures kept as one block (A.17); animation speed and reduced motion
  from the desktop (A.6); a shared design system
  with tokens, `docs/DESIGN.md`, one widget per element and a Gallery (A.19); every
  Linux package offers updates, AppImage and tarball install their own
  (U.10, #634); Katna on Windows updates
  itself (U.10, #632); sound sets with Birds by default (A.5); who saw a
  chat message (A.1); wildcards in Files search (A.3); mail rules from Phase 5
  that run in Katna, as Gmail filters or as Sieve on the mail server, with
  Settings > Folders & rules, a rule editor, Make a rule and starter rules
  that start switched off (#608, #609, #619, #622, #625, #628);
  renaming, deleting and dragging to folders, Move to with search and
  Label as (A.10, #607). The rest is outside the
  plan, added to the "Added along the way" track: Google Drive and OneDrive
  in Files and the pickers (A.16), compose that saves drafts as you write
  (A.17), inbox tabs and a colour for each account in the unified inbox
  (A.18), AI summaries and Write reply (A.2), peek and reply inside
  notifications (A.5), faint lines, Gmail's hover, accent logo and frosted
  panes (A.6), Ctrl and Shift selection, folder keys and window dragging
  (A.10), the Activity report kept inside the window (#604), and update patches with an hourly check (U.10). The Feeds page
  was dropped (#551).
- **Before that (#342–#522):** most of it was not in
  the plan and is now the "Added along the way" track (A.1–A.15): a chat
  reading view (Experimental), writing help with AI on Katna Server or the
  user's own key, the Files page, PDF markup in the viewer, mutes, bells
  and sounds for notifications, color schemes and frosted glass, a new Add
  account dialog with POP3, Zoho calendars and tasks, Send with Katna Mail
  from file managers, Fedora, Nix, AppImage, Snap, Flatpak and tarball
  builds, and much polish on every page. In the plan's own rows: formatting
  in notes (6.9, #354), notification rules (4.2), the file manager entry
  (4.6), the daemon's restart after an update under systemd (U.5, #420),
  meetings, tasks and company details in the contact panel (7.9).
- **In progress:** requests from the owner's daily use, each in its own
  thread.
- **Next:** the owner's live two-way checks with a Microsoft account and a
  CalDAV server (Phase 6); payments for Katna AI after the free month
  (A.2); usage statistics, feedback form and debug-file upload (C.3, C.6,
  C.7); right-to-left layout (L.2, L.3); the rest of the release track
  before any public release; Organizations (Phase 2) later.
- **Size:** the owner raised the daemon's budget from 20 MiB to 50 MB
  (27 September 2026).
- **Needs from the owner:** the Google Cloud and Microsoft Entra setup on
  the setup card in the project chat (APIs, scopes and Graph permissions
  for Drive, Contacts, Tasks, Calendar and Meet), then signing in to each
  account again; a code-signing certificate for Windows.
- **Later:** Organizations (Phase 2), phones, Workspace, own crash server,
  a server check that recipient addresses exist.
- **Dropped:** Feeds (RSS and Atom), decided by the owner on 2 October 2026
  (#551).

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
| 0.1 Workspace scaffold ✅ | Cargo workspace (§3 of the architecture), `rust-toolchain.toml`, lints, `LICENSE`, `CLAUDE.md`, CI jobs `check`, `deny`, `size` |
| 0.2 `katna-core` ✅ | XDG paths, TOML config (`serde`), logging (`tracing`), error types, account model |
| 0.3 `katna-store` ✅ | SQLite (WAL) with migrations, schema v1 (mail + PIM tables), blob store (zstd + blake3), read-only open mode, change journal |
| 0.4 Importers ✅ | Maildir and mbox import into the store (used for Enron and for users migrating) |
| 0.5 `katna-search` ✅ | Index schema, indexing pipeline (parse → text → language → tokenize), batching, versioned schema |
| 0.6 Query language ✅ | Parser for `from: to: org: has: before: after: larger: in: label: is:` + phrases, `-`, `OR`, parentheses → tantivy queries; fuzz-tested |
| 0.7 Ranking + snippets ✅ | BM25 + recency/subject boosts, highlighted snippets |
| 0.8 `katna-search-cli` + `katna-bench` ✅ | `katna-search index <maildir>`, `katna-search query "<q>"` with timings; Enron benchmark job |

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
| 1.9 `katna-meta` ✅ #172; undo send and scheduled send stay in the outbox (#22, #53) | Metadata table + scheduler (undo send first) |
| 1.10 POP3 ✅ #27 | Client with UIDL tracking, leave-on-server, `TOP` (header-first partial download of large messages comes later) |
| 1.11 `katna-daemon` ✅ #15, #17, #19, #65 | Process, `in.invenia.katna.Pim1` D-Bus skeleton (commands + change signals), single instance, systemd user unit, D-Bus activation, graceful shutdown |
| 1.12 System events ✅ #30, #38 | Network changes, suspend/resume, metered connections |
| 1.13 `katnactl` ✅ #15, #28, #33 | Small CLI client for the daemon (add account, sync, search, send, list) — the test harness until the GUI exists |

Status (27 September 2026): all tasks merged; `katna-meta` came with
snooze and reminders (#172). Gmail syncs in the background on the owner's laptop; the
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
| 3.11 Packaging v1 ◐ Arch package and `[katna]` repository #19, #32; `mailto:` handler #106; Fedora (.rpm), Nix, AppImage, Snap, Flatpak and tarball builds on every push, not gating (#462); .deb and AppStream pending | Flatpak (with Background portal), .deb, AUR (tested); .rpm (built, not tested); desktop files, AppStream, `mailto:` handler |
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
command palette and AccessKit, a .deb package and AppStream data, and
checks on GNOME and X11. Fedora, Nix, AppImage, Snap, Flatpak and tarball
builds are published on `linux-latest` (#462). The binary budget was raised to 100 MB (57 MB today)
because GPUI with its Linux backends is larger than planned (spike S1).

**Done when:** you can use Katna Mail as your daily client for one account
on both Plasma and GNOME (Wayland and X11); performance budgets hold
(cold start < 500 ms, binary ≤ 30 MB, idle CPU ≈ 0 %); packages install and
run on the CI distro matrix.

### Phase 4 — Notifications and desktop search (≈ 4 weeks)

| Task | Deliverable |
|---|---|
| 4.1 `katna-notify` ✅ #41, #80 | Notifications with click-to-open (activation tokens), inline reply-all on Plasma, archive/mark read, fallback quick-reply window |
| 4.2 Notification rules ◐ which mail notifies and counts per folder, inbox tab and account, mutes for accounts, folders, conversations and senders #471, #473, #475, #482, #486; per-organization policy waits for Phase 2 | Grouping, Inbox/category filters, per-organization policy, closing on read elsewhere |
| 4.3 Badge + tray ✅ #48, #65, #70 | Unity LauncherEntry unread count, tray icon with badge and menu, single-instance app actions, KDE global menu (done early, September 2026; §15.2) |
| 4.4 KRunner ✅ #166 | `org.kde.krunner1` in the daemon: contacts, mail, organizations; actions |
| 4.5 GNOME search ✅ #166 | `org.gnome.Shell.SearchProvider2` using the same backend |
| 4.6 Small integrations ◐ Send with Katna Mail in Dolphin, GNOME Files and Explorer #432; quick capture shortcuts Meta+Alt+T/N on Plasma (kglobalaccel) and Windows; GNOME (GlobalShortcuts portal) pending | Global shortcut (portal), Dolphin service menu |

Status (2 October 2026): 4.1 and 4.3 are done and translated (#129);
4.4 and 4.5 are done for people, mail, events and tasks (#166, #334,
#337, #341; organizations come with Phase 2); 4.2 and 4.6 are partly done.

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
| D.18 Support links ✅ | Buy me a coffee and a GitHub Sponsor button in the README (#144), a button and QR code in About (#145) |
| D.19 Compose above the account ✅ | Compose sits above the account's name at the top of the folders (#148) |
| D.20 From row ✅ | Compose has a From row with an account picker; new mail starts from the first account (#150) |
| D.21 Unified inbox ✅ | One inbox over all accounts, and arrows that fold each account in the folder pane (#151) |
| D.22 Folder menus and Settings ✅ | Right-click menus fit each folder, Move to moves, Not spam in Spam (#152); picking a folder or app leaves Settings (#153); About opens over any view (#161) |
| D.23 Select menu ✅ | Choices tick the matching mail on screen, with a link to select the rest, as in Gmail (#154) |
| D.24 Drafts on close ✅ | Closing compose saves the draft here and on the server; only Discard drops it, with Undo (#155) |
| D.25 Account drag ✅ | The whole account row drags in Settings > Accounts, the others slide aside; arrows on hover (#156) |
| D.26 Storage bar ✅ | Storage used at the foot of the folder pane (#158) |
| D.27 Prompt Quit ✅ | Quit from the tray ends the daemon promptly, even mid-sync (#162) |
| D.28 Softer hover shadow ✅ | A lighter shadow on the hovered mail line (#169) |
| D.29 Chip drag ✅ | Address chips drag between To, Cc and Bcc; an x on hover removes one (#171) |
| D.30 Reopen where left ✅ | The window reopens on the same app, folder, tab and folder pane (#173) |
| D.31 Receipts on by default ✅ | Tracking, read receipts and delivery receipts are on for new mail (#193); delivered and read ticks beside each recipient of sent mail; on Gmail one tick means no bounce within 30 minutes (#198) |
| D.32 Subscription tab ✅ | The Katna account lives inside the Subscription tab (#192); About has one Buy me a coffee button and no QR code (#191) |
| D.33 Print as shown ✅ | Printing keeps the mail's formatting; original colors in dark themes; the top bar fits (#196) |
| D.34 Gmail notifications ✅ | New mail is counted when it reaches the inbox, so Gmail's is not missed (#197) |
| D.35 Delete confirmation ✅ | Deleting several conversations or deleting for good asks first (#200) |
| D.36 Folder arrows ✅ | Each folder's arrow stays inside its pill (#201) |
| D.37 Attachment types in search ✅ | Search options filter by attachment type: PDF, XLSX, ODF and others, or a custom one (#202) |
| D.38 Chips and format bar ✅ | The remove button always shows on recipient chips (#205); the formatting bar is a pill with a faint border (#206) |
| D.39 Fast nested quotes ✅ | Long threads of nested quoted HTML lay out in linear time instead of slowing the reader (#207) |
| D.40 Who opened ✅ | An eye beside the star opens a popover of who opened a sent message and when (#210) |
| D.41 Contact panel actions ✅ | Call or copy the phone number; the panel's text is selectable (#211) |
| D.42 Floating folder pane ✅ | The folded folder pane floats out with a notch; search widens while it is folded; the open folder no longer blinks (#212) |
| D.43 Quoted mail in replies ✅ | Hide or remove the quoted mail in a reply; the Send row stays still (#213) |
| D.44 Storage bar for unlimited quotas ✅ | No storage bar when the account has no quota (#214) |
| D.45 Search options ✅ | Address suggestions in From and To (#215); the panel fits narrow and short windows (#219) |
| D.46 Main menu ✅ | The main menu sits behind ☰ in the account card; Reply all from a notification opens its own window (#216) |
| D.47 Open from tray and notifications ✅ | Clicking the tray or a notification brings the window forward and opens the mail (#217) |
| D.48 Sending feedback ✅ | A countdown ring on Send, a sent toast and sound, the reply staying in place and a simpler reply head (#220) |
| D.49 Reply from the right account ✅ | A reply starts from the account the conversation is in (#221) |
| D.50 Update from the app ✅ | Help > Check for Updates and an update notification install the new package (pkexec), with a retry and an Update window (#232, #242, #262, #271, #272); only minisign-signed packages install (#270) |
| D.51 Security audit ✅ | The audit's high findings (#251) and the rest (#260) fixed; a warning before using a server without encryption (#263) |
| D.52 Large files ✅ | Files over 25 MB go as a Google Drive (#252) or OneDrive (#256) link |
| D.53 Faster start ✅ | Katna account loading bar (#258), a faster start (#264), the first list read at startup (#266), GPUI list paging (#269) |
| D.54 Keyboard ✅ | Keys follow the focused pane (#273); folder pane, menus and dialogs by keyboard with industry-standard keys (#276); folders slide (#275); folder pane right-click (#257) |
| D.55 Compose sheet ✅ | Compose below the top bar (#239); closing a popped-out compose frees Reply (#241); recipients use the full width, fold (#246) and scroll (#254); a tidier reply box (#226) |
| D.56 Activity and tracking ✅ | Opens and clicks moved out of Activity (#225); an account picker (#248); tracking for signed, encrypted and plain-text mail (#227) |
| D.57 Look ✅ | The owner's round logo (#229); softer contact panel cards (#230) and the panel for one's own mail (#255); a tighter window shadow (#231); search bar blur (#243); a narrow reader toolbar (#249); shorter What's new and About dialogs (#259); What's new shows only the newest (#240); the coffee button's play (#274) |
| D.58 Small conveniences ✅ | A search box shortcut and floating folders (#224); the start menu line (#238); scrolling the account picture switches accounts (#245) |

Not yet checked on a real desktop: Open with (#55), Gmail Important sync
(#56), scheduled send (#53), the badge count with one account (#61), the
print hand-off to KDE's print dialog (#122) and reopening the window in
place on Plasma 6.7 Wayland (#105).

### Added along the way — work outside the plan (29 September – 4 October 2026, through #688)

From #342 the owner asked for much that no phase named. Each row is
merged; the pull requests say what changed.

| Task | Deliverable |
|---|---|
| A.1 Chat reading view ✅ #488, #497, #502, #505, #509, #512, #514, #517, #519, #520, #526, #580, #583, #593, #616, #626, #629, #637, #638, #642, #674 | Settings > Experimental > Reading: conversations between people as chat bubbles with a reply box, pins for up to five things, a person's card with Company and signature details, a line when the subject changes, attach from Files |
| A.2 Writing help with AI ✅ #494, #499, #501, #504, #510, #511, #528, #535, #538, #544, #546, #547, #552, #571, #660 | Rephrase a selection or the whole message (Ctrl+J) and grey autocomplete with Tab, a summary of a conversation with reply ideas, Write reply, and the subject rephrased, through Katna AI on Katna Server (30 days free) or the user's own key (Gemini, OpenAI, Claude, Mistral, DeepSeek, OpenRouter, local); the server's admin page with keys, models and spending caps (`ARCHITECTURE.md` §16.5). Payments after the free month are still to come |
| A.3 Files page ✅ #431, #449, #464, #467, #468, #480, #483, #484, #498, #633 | Every attachment of every account in one place (Ctrl+7), by kind, account, sender and time, with arrow keys and a two-month calendar; signature pictures left out |
| A.4 Viewer ✅ #371, #380, #385, #408, #414, #416, #436, #451, #455, #490, #577, #591, #597, #601, #614 | PDF markup (highlight, pen, sticky notes, text boxes) saved as a marked copy and replied with; page box, zoom, pinch, rotate, fit and real size; a frosted top bar; Forward from the viewer; dark pages in dark mode; controls that fold into More when narrow |
| A.5 Notifications ✅ #413, #466, #471, #473, #475, #482, #486, #487, #543, #636, #639 | One rule for what notifies and counts (Primary tab by default), bells per folder and tab, mutes for accounts, folders, conversations and senders, a sound for each event played by Katna, Settings > Notifications; peek and reply inside the notification; sound sets with Birds by default and one's own file; Copy code and verify links in notifications |
| A.6 Look ✅ #445, #448, #469, #472, #474, #476, #477, #492, #493, #500, #506, #508, #513, #516, #521, #522, #527, #530, #533, #548, #562, #572, #582, #596, #599, #610, #613, #623, #630, #661 | Mode, color scheme and accent chosen apart; the desktop's schemes, 13 built in and the user's own (made, imported, exported); frosted menus, dialogs, panes and search box with their own switches and blur amount; window roundness and border; faint lines in light and dark; Gmail's grow-in hover on buttons; the logo in the accent; a colour or one-colour tray icon |
| A.7 Accounts ✅ #361, #392, #415, #442, #447 | Add account with provider tiles and POP3; an account menu with sync state, storage and sign in again; a new account's inbox first; a first start that shows the window and offers a Katna account; switching account keeps the page |
| A.8 Zoho ✅ #393, #397, #399 | Sign in with Zoho for Zoho's calendars and tasks |
| A.9 Calendar, Tasks, Notes and Contacts polish ✅ #342, #345, #346, #347, #348, #350, #351, #352, #358, #360, #363, #364, #367, #372, #374, #381, #382, #383, #384, #388, #410, #412, #417, #418, #463, #465, #478, #518 | Phone layouts and folding side panels; every account listed with why its calendars, tasks or contacts are missing; right-click menus; dragging tasks and notes into order; the top search box on each page; typed quick add for tasks; open tasks and meetings in the contact panel |
| A.10 Mail window polish ✅ #357, #359, #368, #375, #376, #378, #391, #395, #401, #402, #411, #421, #424, #428, #429, #430, #434, #435, #437, #439, #440, #441, #444, #450, #452, #454, #456, #457, #458, #459, #461, #470, #479, #481, #485, #491, #495, #496, #503, #507, #515, #532, #551, #554, #574, #575, #578, #581, #590, #594, #598, #600, #602, #603, #604, #607, #611, #615, #617, #618, #620, #621, #627, #640, #641, #643, #644, #645, #662, #664, #665, #666, #667, #668, #669, #673, #675, #676, #679, #681, #682 | Inbox tabs as a pill bar; coloured folder icons with faint count pills; one checkbox at every scale; a right-click menu that fits the window; tables that keep their columns; Back to top; selectable header details; touchpad glide on Wayland; one left-bar button and fold on every page; a soft search box; reply arrows in the list; Ctrl+click and Shift+click selection; Space and arrows on folders; the window moved from any empty space; the Feeds page removed; rename, delete and drag to folders, Move to with search, Label as; Settings sorted into all apps and each app's pages (its own window, #676, was reverted in #681); menus and popovers fade out; text selectable and copyable across the app |
| A.11 Sending files ✅ #432, #489, #490 | Send with Katna Mail from Dolphin, GNOME Files and Explorer; Forward keeps the original's attachments |
| A.12 More Linux packages ✅ #462, #634 | Fedora, Nix, AppImage, Snap, Flatpak and tarball builds tested and published on `linux-latest` |
| A.13 Windows fixes ✅ #369, #425, #426, #443, #446, #460, #632 | Windows draws the shadow and corners; windows fit the screen; Katna's frame and blur; sign-in keys and DNS; cleaner uninstall; Katna updates itself on Windows |
| A.14 Sync and daemon ✅ #390, #398, #405, #406, #407, #409, #420, #423, #453, #671 | Newest mail first; a locked keyring waited for at login; a switched-off Google API named with a Turn on button; a refused certificate explained; restart through systemd after an update |
| A.15 CI and README ✅ #404, #419, #422, #427, #433, #438, #529, #563 | Tests on Arch only, Ubuntu and Windows in a Secondary workflow; README says why Katna exists and that it is at a very early stage |
| A.16 Drives in Files ✅ #534, #537, #540, #541, #542, #550, #553, #573 | Google Drive and OneDrive in the Files page and the Compose and chat pickers; upload files and folders; Move to bin, Rename, Share and an uploads tray |
| A.17 Compose ✅ #531, #536, #539, #579, #595, #663, #672, #677, #680, #683 | A calmer Quiet look; drafts saved while writing with "Draft saved"; a signature tag; Compose opens beside a half-written reply; click an attached file to open it; an emptied message keeps no draft; signatures imported from Gmail, Thunderbird, Evolution and KMail, designed signatures as one block, Paste HTML; twelve signature layouts with a fields form |
| A.18 Unified inbox and account colours ✅ #549, #576, #592, #624, #631, #670 | Inbox tabs in the unified inbox shared by every account; a colour for each account, shown as a dot after the sender, with eight colours and a colour wheel |
| A.19 Shared design system ✅ #646, #647, #648, #649, #650, #651, #652, #653, #654, #656, #657, #658, #659 | Design tokens (`katna_ui::tokens`) and `docs/DESIGN.md`, with a CI check that raw sizes only go down; one shared popover, dialog surface, Button, choice chip, tag, settings row, field and card; a Gallery of the controls in development builds |

### Release track — update channels and safe updates (before the first public release)

Planned 26 September 2026 (#39). Done so far: the daemon's restart after
updates (#65), updating the Arch package from the app (#232, #242, #262,
#271, #272) and minisign-signed packages that the app checks before
installing (#270; part of U.9 and U.10).
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
| U.5 Running while updated ◐ the daemon restarts itself after an update (#65), through systemd (#420); the rest pending | Daemon notices its binary was replaced and restarts when idle; `Version()` on D-Bus; restart pill in the apps; old/new app and daemon tests |
| U.6 Health check and safe mode | First-start self-check, `health.toml`, safe mode after three failed starts, restore from backup, export of local-only data, "Copy debug report" |
| U.7 Upgrade and rollback tests in CI | Container test: previous stable → candidate with the daemon running, against the dev servers; candidate → previous stable |
| U.8 Release workflow | On a tag: build every format once, run the checks, publish to beta; promotion to stable copies the same files after a required reviewer approves |
| U.9 Signing ◐ minisign-signed Arch packages and manifest, checked by the app before installing (#270); GPG and AppImage signing pending | GPG-signed pacman packages and repository databases (`SigLevel = Required`); minisign-signed manifests and AppImages; keys only in the release environment |
| U.10 Update checks ◐ the app checks the signed `arch-latest` manifest hourly and after waking (#270, #555) and downloads a zstd patch from the installed build (#545, #570); Windows Setup installs check and update themselves too, with the full Setup (#632); every `linux-latest` package offers updates: AppImage and tarball install them, RPM, Snap, Flatpak and Nix show the command (#634); channels, metered networks, Flatpak and AppImage pending | Daemon reads the signed channel manifest (daily, not on metered networks, can be turned off); Flatpak update monitor; AppImage self-update with staged rollout and the `pulled` flag |

**Done when:** a beta built by the release workflow upgrades a running
install of the previous stable on Arch and Ubuntu without losing a message,
setting, password or queued send; installing the previous stable again
still opens the data or restores the backup; a daemon made to fail its
self-check starts in safe mode and restores the backup from its
notification; promotion to stable ships byte-identical, signed files.

### Windows track (asked by the owner, 27 September 2026)

Katna Mail and the daemon on Windows 10 version 1903 and later (GPUI
needs the system's `icuuc.dll`), with Windows
integration that works (start at login and so on), installed by a modern
installer rather than NSIS. The Windows installer thread owns it. KDE's
global menu, KRunner and GNOME search and the window blur stay Linux-only.

| Task | Deliverable |
|---|---|
| W.1 Windows build ✅ #190 | A Windows CI job; data and settings under `AppData`; Unix-only file calls kept to Linux |
| W.2 App–daemon link ✅ #190 | Katna's own session bus (a bundled `dbus-daemon`), so `katna-dbus` works unchanged; the daemon starts from the `Run` key |
| W.3 Passwords ✅ #190 | Windows Credential Manager instead of the Secret Service keyring |
| W.4 Windows integration ✅ #194 | Tray with the unread count, notifications, start at login, Katna as the default mail app, printing |
| W.5 Installer ✅ #194 | Katna's own GPUI "Katna Setup.exe": per user, no administrator prompt, a Start menu entry, an uninstall entry in Settings > Apps, the `mailto:` handler; an MSI for offices maybe later |
| W.6 Pre-release ✅ #194 | A `windows-latest` pre-release built on every push to `main`, like `arch-latest` |

Status (28 September 2026): W.1–W.6 are merged and a Windows 11 virtual
machine test is in progress. The files are unsigned until the certificate
below is in place.

Needs from the owner: a code-signing certificate (SignPath Foundation or
Certum's open-source certificate), kept in GitHub secrets, and testing on
the owner's own Windows PC.

Later: a taskbar overlay badge (needs COM, which `unsafe_code = "forbid"`
rules out today); a PowerToys Run search plugin; Windows window chrome
(the system title bar or Katna's own); signed files once the certificate
secret is in place.

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
| L.4 Strings, by area ◐ main window #114, Settings #116, dialogs and search options #119, compose #121, notifications and tray #129; What's new #134; account settings, paste and drop and newer text #141, #142, #147, #157, #165, #174, #175, #177, #179, #186, #189, #195, #199, #204, #208, #218, #222, #228, #234, #236, #237, #244, #247, #250, #253, #261, #265, #267, #268, #279, #282, #284, #292, #294, #296, #298, #300, #303, #306, #310, #311, #314, #316, #319, #329, #335, #340, #344, #349, #353, #355, #356, #362, #365, #366, #370, #373, #377, #379, #386, #387, #389, #394, #396, #400, #403, #556, #557, #558, #559, #560, #561, #564, #565, #566, #567, #568, #569, #584, #585, #586, #587, #588, #589; onboarding, About, viewers and `.desktop` names pending | One pull request per area, each with its 48 drafted translations: message list and toolbar; reader and attachments; compose and signatures; search and search options; Settings (each tab); accounts, onboarding, What's new, About, crash notice; viewers; global menu, dock menu, tray, notifications and the daemon; `.desktop` file names and actions |
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
send later (#53, handed to the server with FUTURERELEASE in #167), pins
(#57), Undo on every action (#125), templates (#168), snooze and
follow-up reminders (#172), mail rules run in Katna, as Gmail filters or
as Sieve on the server, with starter rules (#608, #609, #619, #622,
#625, #628; 3 October 2026). Not started: vacation
responder, one-click unsubscribe, a local category classifier,
phishing warnings. Mute is done as part of notification rules (4.2, A.5:
mutes for accounts, folders, conversations and senders). **OAuth2** for Google and Microsoft is merged (#163): the
installed-app flow with PKCE, tokens in the Secret Service or Credential
Manager, SASL XOAUTH2; its buttons appear once the owner adds the client
IDs as GitHub secrets.

**Done when:** the feature checklist in the architecture (§10, §11) works
against Stalwart, Dovecot and Gmail, with integration tests.

Notes on mail are done in Phase 6 (6.9: "Add a note" on a conversation).
Later, not scheduled yet: the Workspace view (`ARCHITECTURE.md` §13.7). Workspace builds on snooze from this phase.

### Phase 6 — Katna Calendar, Tasks, Notes and Contacts

Reshaped 29 September 2026 after the calendar and tasks study
(Google Calendar, Proton, Outlook, Fantastical, Notion Calendar, Morgen,
Thunderbird, Merkuro, Google Tasks, To Do, Todoist, TickTick, Things). The
owner's decisions, taken as the study recommended:

1. **Pages, not programs.** Calendar, Tasks, Notes and Contacts are pages
   of the Katna window, on the app rail beside Mail, each with its own
   menu entry and icon that opens straight to that page. One GPUI
   program: instant switching, a shared side panel, about 20 MB less
   memory than a second program. This replaces the separate
   `katna-calendar` app in `ARCHITECTURE.md`.
2. **Each service's own calendar.** Google Calendar API for Gmail accounts
   (Meet links, focus time, out of office, working location, labels;
   Google sends invitations), Microsoft Graph for Microsoft accounts,
   CalDAV for the rest, local calendars without an account. This follows
   the feature rule and replaces "CalDAV only".
3. **Gmail tasks in Google Tasks.** Due time, reminders and repeat, which
   the Google Tasks API cannot hold, are kept on this computer; Microsoft
   To Do through Graph; VTODO over CalDAV.
4. **Calendar before Tasks.**
5. **Week start from the language setting**, with a choice in Settings.
6. **No booking pages.** "Share free times" pastes free slots into a mail
   instead; Katna Server stays with tracking, translation and accounts.

Look: Google Calendar's week grid, pale Create button, one-step create
popover and density setting; calendars grouped by account with colors
(Mailspring, Morgen); tasks dragged onto the grid to block time (Morgen).

| Task | Deliverable |
|---|---|
| 6.1 See it ✅ #283, #290 | Page switcher on the app rail and in the menus; calendar tables in `katna-store`; daemon sync through Google Calendar API, Microsoft Graph and CalDAV (`katna-dav`), and local calendars; `calcard` parsing, recurrence with exceptions (`rrule`), time zones (`jiff`); Week, Day, Month and Schedule views, read-only, with account groups and colors; agenda side panel in Katna Mail; events in the Plasma clock |
| 6.2 Change it ✅ #299, #323, #325 | Create popover (press C, or click or drag on the grid) and full editor; drag to move and resize; repeat rules and exceptions; Undo and Ctrl+Z; reminder notifications with Join and Snooze; density, second time zone, keyboard shortcuts |
| 6.3 Meetings ✅ #307, #312, #315, #318 | Invitation card in the reader with Yes, Maybe, No and the day around it; replies through Google or Graph, else iMIP mail; guests and busy times (freeBusy, getSchedule); Meet and Teams links; Schedule meeting from a conversation; running-late mail; focus time, out of office, working location |
| 6.4 Tasks ✅ #285, #291, #293, #297, #302, #304, #309, #313, #321; reminders and repeat #333, desktop search #337; Upcoming, Completed, sort, labels, files, synced star #684, #685, #688; quick capture #686 | Google Tasks, To Do and CalDAV task sync with local extras; Tasks page and Today view; Add to tasks from mail (Shift+T) with the link back; task chips in the mail list; tasks on the calendar and drag to block time |
| 6.5 Polish ✅ #336, #341; birthdays #334 | Typed quick add for events and tasks, calendar sets, Year view, share free times as text, birthdays, holidays, events in KRunner and GNOME search |
| 6.6 Katna Digital Clock ✅ #278, #332 | Plasma: an alternative to the digital clock with Plasma's calendar and a task list under the day; GNOME: a Tasks card under the calendar; tasks stored locally in `pim.db` (v4); installed by the Arch package; the day menu adds events and tasks, and events open Katna's Calendar (#332) |
| 6.7 Notes page ✅ #286 | A note table in `pim.db` with a change journal, "On this computer" notes with no account; the Notes page in Google Keep's look: Take a note bar, board of cards (grid and list), pinned first, a note that opens over the board and saves as you type, checklists with ticked items folding down, colors, archive, Trash for 7 days, Undo, search |
| 6.8 Notes sync ✅ #286 | Each IMAP account's Notes folder in Apple's format (`X-Uniform-Type-Identifier: com.apple.mail-note`, one HTML message per note), so notes show in Apple Notes and Thunderbird; colors, pins, labels and links in Katna's own `X-Katna-*` headers; checklists as ☐/☑ lines; new notes go to the account last looked at, with a picker; checked against Stalwart and Dovecot. Google Keep and OneNote have no API for personal accounts |
| 6.9 Notes ties ✅ #301, #308, #320, #327, #354, #687 | Labels; "Add a note" on a conversation with a "Your note" card in the reader; meeting notes from an event; a checklist line made a task; bold, italic, headings, lists and links. pictures, reminders, multi-select, links between notes, version history, export and AI (#687). Later: Nextcloud Notes |
| 6.10 Contacts: see them ✅ #289, #317 | Contact tables in `pim.db` (phones, addresses, dates, labels, photo, raw vCard, source); daemon sync through the Google People API, Microsoft Graph and CardDAV, and a local address book; the Contacts page in Google Contacts' look (list with A–Z and starred, search, contact page with mail history and "Where it's saved"); one person across accounts, linked by email address; saved names in Mail and ranked first in address suggestions. Today's mail-derived list becomes Frequent |
| 6.11 Contacts: edit ✅ #295, #305 | Create (Ctrl+N, "Save to" picker), edit in place (F2), delete with the Undo toast then the service's own Trash; Add to contacts from the contact panel and the reader; pictures |
| 6.12 Contacts: labels and tidy ✅ #305, #317, #324 | Labels (Google groups, Graph categories, CardDAV group vCards) and mailing a label; Google's Other contacts, read-only with Add to contacts; Merge & fix; vCard and CSV import, vCard export. People written to are not saved on their own |
| 6.13 Contacts: polish ✅ #334 | Birthdays in Calendar, QR code share, print, saved contacts in KRunner and GNOME search |
| 6.14 Upstream proposals | Merge requests to Plasma: "Add…" with date, click event to open, plugin action hook |
| 6.15 Video meetings ◐ step 1 ✅ #328 | Step 1: meeting links from each account's own service (Google Meet, Microsoft Teams) plus Jitsi for the rest, decided by the owner 29 September 2026. Calls inside Katna come later |

**Done when:** on the Katna window's Calendar page, events from Gmail,
a Microsoft account and a CalDAV server (Nextcloud or Fastmail) sync both
ways; recurring events with exceptions and time zones match a reference
test set; an invitation answered from Katna Mail reaches the organizer; a
Gmail task made in Katna appears in Google Tasks on the phone; a note
written in Katna opens in Apple Notes from the same Gmail account and back;
a contact edited in Katna shows the change in Google Contacts, and the
same person saved in two accounts shows once; and the app stays within its
size budget.

Status (2 October 2026): all rows but 6.14 and 6.15's later steps are
merged; Zoho calendars and tasks were added (A.8). The owner's live two-way checks with a Microsoft
account and a CalDAV server are still to do.

### Phase 7 — Katna Server (≈ 8 weeks)

Server (axum + PostgreSQL), accounts/auth, metadata sync with E2E-encrypted
values, event stream to the daemon, open/link tracking with bot/scanner
labeling, per-recipient sending in the daemon, activity dashboard in Katna
Mail, optional server-side send later/snooze, container image for
self-hosting.

Started 27 September 2026, ahead of Phases 2, 5 and 6, at the owner's
request. The owner wants the ten features of Mailspring Pro, backed by Katna
Server on the owner's own server (`server.katna.invenia.in`, tracking on a
separate domain; the owner deploys the container, ARCHITECTURE.md §16). Not all
of them need the server: most work in the daemon on this computer, and the
server adds only what a computer that is switched off cannot do.

**Rule (owner, 27 September 2026):** use the mail service's own feature
when Katna can reach it over the protocols it speaks (IMAP, SMTP, Sieve,
CardDAV, later JMAP). Otherwise do it locally on the user's computer. Use
Katna Server only for what can work neither way: tracking, translation and
Katna accounts, and (decided 1 October 2026) Katna AI, the writing help in
A.2, which the user can point at their own key instead. Nothing
puts the user's mail, passwords or tokens on the server unless a row below
says so and the owner has decided it.

| Task | Where it runs | Deliverable | Status |
|---|---|---|---|
| 7.0 Katna accounts ✅ #170 | Server + app | A Katna account on the server, like Mailspring ID (owner, 27 September 2026): sign-up with an email address and password, address verification, per-device tokens that replace the per-install token, a Katna account page in Settings, and every server feature (7.1–7.3, 7.8) behind sign-in. No payments yet. The account holds no mail logins (7.10) | Done |
| 7.1 Read receipts ✅ #160, #182 | Server + daemon + app | Opens per recipient through a tracking picture; a standard read-receipt request (MDN, RFC 8098) is offered as the no-server choice, which the recipient may decline; per-recipient sending (ARCHITECTURE.md §11); Apple Mail Privacy Protection shown as "maybe", scanners as "scanner"; off by default, per message | Done |
| 7.2 Link tracking ✅ #160, #182 | Server + daemon + app | Clicks through `/l/<id>/<n>` redirects stored on the server (never an open redirect); shown per recipient and link | Done |
| 7.3 Mailbox insights ✅ #184, #187 | App, with 7.1–7.2 events | An Activity view: open and click rates of tracked mail, reply rates and times, busiest senders and hours, subject lines that got replies; counted from the local store, only tracking events come from the server | Done: an Activity button beside search with a feed of opens and clicks, a Details report for the last 7 or 30 days, all time or chosen dates, each recipient's opens and clicks in the reader, and mail sent and received, reply rates and times, top correspondents and a weekday by hour grid from `mail.db` |
| 7.4 Mail templates ✅ #168 | App (local) | Save mail as a template, start new mail or a reply from one, fields such as the first name; Settings > Compose > Templates; stored in `pim.db`. Sync between devices later with 7.12 | Done |
| 7.5 Follow-up reminders ✅ #172 | Daemon (local) | "Remind me if nobody replies in N days" in compose; `katna-meta` (task 1.9) with the reply check, a notification and the thread back on top of the Inbox. Works while the computer is on; 7.10 covers a switched-off computer | Done |
| 7.6 Snooze ✅ #172 | Daemon (local; Gmail, Outlook.com and Zoho offer no snooze over IMAP) | Snooze a conversation until a time; it moves to a "Snoozed" label and comes back unread (ARCHITECTURE.md §10, Phase 5). Same `katna-meta` scheduler as 7.5 | Done |
| 7.7 Send later ✅ #53, #167 | Mail server, else daemon | Scheduled send exists and sends while the computer is on. Where the account's SMTP server offers FUTURERELEASE (RFC 4865; Stalwart does, Gmail does not) or, later, JMAP's `sendAt`, hand the mail to the server so it goes out on time with the computer off | Done |
| 7.8 Automatic translation ✅ #176 | Server + daemon + app | Translate a message into the reading language, with the original one click away. LibreTranslate (AGPL-3.0, its own container) on the owner's server, chosen 27 September 2026; the daemon sends only the text of a message the user asks to translate (or of languages the user chose to always translate), over TLS with the install's token; no logs kept | Done |
| 7.9 Rich contact profiles ◐ #164, #346, #348, #514, #519 | App (local) + decision | A right-hand panel for the sender: picture, all mail exchanged, attachments, first and last contact, signature details (phone, title) read from their mail, and the account's own contacts over CardDAV once Phase 6 syncs them. Outside profiles (LinkedIn, X) need a data source | Local panel done (#164) with open tasks, upcoming meetings and a Company section (#346, #348, #514, #519); saved contacts from Phase 6; outside data to decide |
| 7.10 While the computer is off | — | Send later, snooze and reminders run only while the computer is on; the server holds no logins or tokens (owner, 27 September 2026). Revisit later if wanted | Decided: computer only |
| 7.11 Company overviews | App + Phase 2 | A company page: people, mail and files exchanged, and the local time from their mail. It is the organization view of Phase 2 (2.1–2.4, 3.9). Size and funding need a data source | After Phase 2; outside data to decide |
| 7.12 Metadata sync | Server | Templates, reminders, snoozes and tracking IDs shared between the owner's devices, end-to-end encrypted (ARCHITECTURE.md §16) | Later |

Decisions (asked in the plan thread, 27 September 2026):

- **7.10 (decided):** computer only; the server holds no logins.
- **7.8 (decided):** LibreTranslate on the owner's server, over
  on-device models or DeepL.
- **7.9 and 7.11:** where outside profile and company data would come
  from. LinkedIn and X offer no API for this and forbid scraping, and
  paid data services (such as People Data Labs or Crunchbase) send the
  address to a third party. Starting with what the user's own mail says
  needs no decision.

**Done when:** tracked mail reports opens and clicks with Apple/scanner
labeling; the server stores no content; a self-hosted instance runs from
the published container image.

### Phase 8 — Polish (ongoing)

Started early: HTML rendering (#44), a formatting composer with tables
(#53), spelling, grammar with Harper (#111) and writing suggestions (#118),
scaling 75–200 % (#104), large attachments as Drive or OneDrive links (D.52); the composer
is a WYSIWYG editor since #53. Still open: `<style>` sheets in HTML mail
(Blitz, only if such mail turns out to matter), semantic search
(local embeddings), Katna Confidential, GNOME
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
