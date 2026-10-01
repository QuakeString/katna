# Katna PIM — Architecture

> Status: **Draft v0.2** (26 September 2026). This is the design reference for
> Katna Mail (with its Calendar, Tasks, Notes and Contacts pages), the Katna
> background service and Katna Server.
> Nothing here is built yet; sections marked **Decision needed** are open.
>
> Changes since v0.1: background service (`katna-daemon`) as the single owner
> of sync and data; desktop integration (notifications with actions and
> inline reply, KRunner, GNOME search, Plasma calendar plugin and clock fork);
> measured performance budget; updated packaging, roadmap and risks.

## 1. Goals

Katna is a modern, fast personal-information-management suite for Linux,
written in Rust, as an alternative to KDE PIM (KMail, Kontact, Akonadi,
Merkuro).

| Goal | What it means in practice |
|---|---|
| **Best-in-class search** | Instant (<50 ms) full-text search across very large mailboxes, including mail that is not fully downloaded. |
| **Organizations first** | Mail is grouped by company/customer/party, not only by address. Searching a company name finds all its mail, even from personal addresses. |
| **Looks native on KDE and GNOME** | Real KWin decorations on KDE; Adwaita-style header bar, shadows and button layout on GNOME. |
| **Deep Plasma integration** | Mail, contacts and events in KRunner; events in the Plasma clock, with add/edit; rich notifications with inline reply. GNOME equivalents where GNOME allows. |
| **Works when the app is closed** | A background service syncs, notifies, sends scheduled mail and fires reminders without any window open. |
| **Gmail-class features** | Undo send, send later, snooze, reminders, labels, threading, rules, templates, one-click unsubscribe. |
| **Sales/pro features** | Open and link tracking, activity dashboard (via Katna Server). |
| **Fast and light** | Near-zero idle CPU, small binaries, low memory (§17). |
| **Local-first and private** | Works offline, data stays on the machine, incoming trackers are blocked. |
| **Rust only** | All Katna code is Rust, except the thin Plasma extensions that Plasma requires to be C++/QML (§15.6). Only unavoidable system C libraries are linked. |
| **Packaged everywhere** | Flatpak, .deb, .rpm, AUR, AppImage, Nix, others. |

### Non-goals (for now)

- macOS builds. Windows 10 and later is planned (owner, 27 September
  2026; the Windows track in `IMPLEMENTATION_PLAN.md` §5).
- Android / iOS builds for now; the design for later is §26.
- Being a general Akonadi replacement that other apps plug into.
- Exchange (EWS) support in the first releases.

## 2. System overview

```
                         ┌──────────────────────────────────────────────┐
                         │ katna-daemon (background service, no GUI)     │
                         │  sync (IDLE/push) · indexing · scheduler      │
                         │  notifications · KRunner runner · GNOME       │
                         │  search provider · D-Bus API                  │
                         │  in.invenia.katna.Pim1                        │
                         └───────┬───────────────────────────┬──────────┘
                     writes      │                           │  D-Bus
           ┌─────────────────────┴─────┐      ┌──────────────┴───────────────────────┐
           │ katna-store               │      │ Clients                              │
           │ SQLite (WAL) · blobs ·    │◄─────┤  Katna Mail, Katna Calendar (GPUI)   │
           │ tantivy index             │ read │  KRunner, GNOME Shell search         │
           └───────────────────────────┘ only │  Plasma calendar-events plugin (C++) │
                                               │  Katna Digital Clock (QML), GNOME ext│
                                               │  notification server (actions/reply) │
                                               └──────────────────────────────────────┘
           Protocols inside the daemon: Pimalaya io-* (IMAP, SMTP, JMAP,
           WebDAV/CalDAV/CardDAV) + own POP3, driven by our rustls I/O layer.
                                   │ HTTPS / WebSocket (optional)
                          ┌────────┴────────┐
                          │  Katna Server   │  tracking · metadata sync · scheduled actions
                          └─────────────────┘
```

Principles:

1. **The daemon owns the network and all writes.** It is the only process
   that syncs, sends, indexes or writes to the databases.
2. **Apps are clients.** Katna Mail and Katna Calendar read SQLite and the
   index directly in read-only mode (fast), send *commands* to the daemon
   over D-Bus, and react to *change signals*. If the daemon is not running,
   D-Bus activation starts it.
3. **Local-first, optimistic.** User actions (archive, flag, move) apply to
   the local store immediately and are queued for the server.
4. **The search index and caches are disposable.** Everything can be rebuilt
   from the SQLite database and the blob store.
5. **Protocols and toolkits are behind our own traits.** Pimalaya and GPUI
   types never leak into engine or store APIs, so either can be replaced.

## 3. Repository and crate layout

One Cargo workspace (monorepo) for all apps, shared crates and the server.
Repository: `github.com/QuakeString/katna` (renamed from `katna-mail`).

```
katna/
├── Cargo.toml                 # workspace, shared dependency versions
├── crates/
│   ├── katna-core/            # config, XDG paths, accounts, secrets, errors, logging
│   ├── katna-store/           # SQLite schema + migrations, blob store, read-only views
│   ├── katna-import/          # Maildir and mbox import into the store (Enron, migration)
│   ├── katna-meta/            # metadata-with-expiration layer (§10)
│   ├── katna-sync/            # account workers, IMAP/JMAP/POP3/SMTP, outbox, op queue
│   ├── katna-search/          # tantivy index, query language, ranking
│   ├── katna-org/             # organizations, matching rules, suggestions
│   ├── katna-render/          # HTML sanitizing and message rendering
│   ├── katna-preview/         # attachment previews: PDF, pictures, text, sheets, documents
│   ├── katna-dav/             # CalDAV/CardDAV sync, iCalendar/vCard, recurrence
│   ├── katna-ai/              # writing help with AI: prompts, services, wire (§16.5)
│   ├── katna-dbus/            # D-Bus API definitions (in.invenia.katna.Pim1), client + server sides
│   ├── katna-notify/          # notification builder, actions, inline reply, grouping
│   ├── katna-platform/        # portals, desktop detection, settings, tray, badges
│   ├── katna-chrome/          # window decorations (SSD/CSD), theme tokens + presets
│   └── katna-ui/              # shared GPUI components
├── apps/
│   ├── katna-daemon/          # background service (no GUI dependencies)
│   ├── katna-mail/            # GPUI
│   └── katna-calendar/        # unused stub: Calendar is a page of katna-mail (§18)
├── integrations/
│   ├── plasma-calendar-plugin/    # C++ CalendarEventsPlugin → daemon over D-Bus
│   ├── plasma-clock/              # Katna Digital Clock: Plasma's clock (QML) + Tasks
│   ├── gnome-shell-extension/     # Katna events and Tasks in GNOME's clock menu
│   ├── krunner/                   # dbusplugin .desktop metadata
│   ├── gnome-search-provider/     # search-provider .ini
│   └── dolphin-servicemenu/       # "Send as attachment with Katna"
├── server/
│   └── katna-server/          # axum; tracking, metadata stream, scheduled actions
├── tools/
│   ├── katna-search-cli/      # index/query from the terminal, benchmarks
│   ├── katna-bench/           # Enron-corpus benchmarks
│   └── katnactl/              # command-line client for katna-daemon
├── packaging/                 # flatpak, deb, rpm, aur, appimage, nix, desktop/appstream, systemd units
└── docs/
```

Crate dependency direction (no cycles):

```
katna-mail / katna-calendar → katna-ui, katna-chrome, katna-platform, katna-dbus (client), katna-store (read-only)
katna-daemon                → engines, katna-notify, katna-platform, katna-dbus (server) → katna-store → katna-core
```

`katna-daemon` must not depend on GPUI, so it stays small and needs no GPU.
`katna-search-cli` depends only on engines and the store, so search and sync
are testable and benchmarkable without a GUI.

## 4. Technology choices

| Area | Choice | Notes |
|---|---|---|
| GUI | **GPUI** via `gpui-pre` + **GPUI Kit** (formerly gpui-component) | Chosen for speed, near-zero idle CPU, pure Rust and small binaries (measured in §17). Official `gpui` on crates.io is stuck at 0.2.2 (Blade renderer); `gpui-pre` tracks Zed's wgpu renderer and AccessKit. Needs recent stable Rust (1.94 failed, 1.98 works). Pin exact versions (§20). |
| Async / I/O | GPUI executors in apps; `smol`-compatible I/O in the daemon; `rustls` | Pimalaya is sans-I/O, so we choose the runtime. The server uses Tokio (axum). |
| Protocols | Pimalaya `io-email`, `io-imap`, `io-smtp`, `io-jmap`, `io-webdav`, `io-calendar` | Light forks where needed (§20). POP3 is our own (not in `io-email`). |
| MIME | `mail-parser`, `mail-builder` | |
| Database | SQLite via `rusqlite` (bundled, WAL mode) | Turso (Rust rewrite of SQLite, same file format) evaluated: pre-1.0, not fully SQLite-compatible yet. Re-evaluate at 1.0; keep all SQL inside `katna-store` and avoid features Turso lacks, so switching needs no data migration. |
| Search | `tantivy` | Plus `rust-stemmers`, `whatlang`; `lindera`/`jieba` for CJK as optional downloads (large dictionaries). |
| Blob compression / hashing | `zstd`, `blake3` | |
| HTML safety | `ammonia` | |
| Calendar data | `calcard` (iCalendar + vCard), `rrule`, `jiff` (time zones) | |
| D-Bus and desktop | `zbus`, `ashpd` (portals), `oo7` (Secret Service) | Notifications implemented directly on `org.freedesktop.Notifications` via `zbus` (actions, inline reply, activation tokens); the tray (StatusNotifierItem), dbusmenu and the taskbar count too (§15.2). |
| Icons | `freedesktop-icons` + `resvg` | |
| Spell check | `spellbook` | Hunspell dictionaries. |
| Languages | Fluent (`fluent-bundle`) + ICU4X | UI text in `.ftl` files per language, dates, numbers and plurals from CLDR; RTL mirroring in the vendored GPUI (§13.10). |
| Mail rules on the server | `sieve-rs` (compile) + ManageSieve | |
| OpenPGP and S/MIME | The user's GnuPG: `gpg` and `gpgsm` (`katna-crypto`) | Like KMail: existing keys, trust, gpg-agent, pinentry and smartcards work unchanged (§19.1). Sequoia/rPGP kept in reserve. |
| Server | `axum`, PostgreSQL (`sqlx`) | |
| Plasma extensions | C++ / Qt 6 / QML, KF6, libplasma | Only in `integrations/plasma-*` (§15.6). |

Alternatives considered for the GUI: iced/libcosmic, Slint, GTK4
(`gtk4-rs`), Qt (`cxx-qt`). GPUI is kept; its real costs are development
effort (no HTML engine, no rich-text editor, unofficial release channel),
not runtime performance.

## 5. Local data

### 5.1 Locations (XDG)

| Path | Content |
|---|---|
| `$XDG_CONFIG_HOME/katna/` | Settings (TOML). |
| `$XDG_DATA_HOME/katna/mail.db` | Mail database. |
| `$XDG_DATA_HOME/katna/pim.db` | Shared: accounts, contacts, organizations, templates, tracking, tasks. |
| `$XDG_DATA_HOME/katna/calendar.db` | Calendar database. |
| `$XDG_DATA_HOME/katna/blobs.db` | Raw messages, zstd-compressed, content-addressed (§5.2). |
| `$XDG_DATA_HOME/katna/attachments/` | Large attachments only (> 256 KB). |
| `$XDG_DATA_HOME/katna/index/` | tantivy index (rebuildable, but expensive, so not in cache). |
| `$XDG_STATE_HOME/katna/crashes/` | Crash reports, plain text, readable by the user only (`0700`, files `0600`; §19.2). |
| Secret Service (`oo7`) | Passwords and OAuth tokens. Never in files. |

Only `katna-daemon` writes these databases. Apps open them read-only
(SQLite WAL allows concurrent readers while the daemon writes) and learn
about changes from D-Bus signals (§14.2).

### 5.2 Message storage

- Raw RFC 822 messages are stored **inside SQLite**, in a separate
  `blobs.db`, zstd-compressed and keyed by their blake3 hash. The same
  message in several folders or labels is stored once.
- Attachments larger than ~256 KB are split out and stored as files in
  `attachments/` (SQLite is faster than the filesystem for small blobs,
  slower for large ones).
- Why not one file per message (Maildir): millions of tiny files waste
  inodes and make backups, disk-usage scans and Flatpak sandbox access slow.
- `blobs.db` uses `auto_vacuum = INCREMENTAL`, so space is returned when
  messages leave the offline window.
- Write order: blob first, then the metadata row that references it
  (atomic commits across attached databases are not guaranteed in WAL
  mode); a background job removes unreferenced blobs.
- Maildir **export** is offered for interoperability with notmuch/mu/mutt.

Decided (implementation plan, D3).

### 5.3 Mail schema (sketch)

```sql
folder           (id, account_id, path, role, uidvalidity, highestmodseq, sync_state)
message          (id, account_id, message_id_hdr, thread_id, subject, date,
                  size, flags, keywords, has_attachments, list_id,
                  body_state,          -- 0 headers | 1 text_indexed | 2 full
                  blob_hash, snippet,
                  auth_results_json,   -- provider's verdict on From: {"dmarc", "aligned"}
                  category)            -- inbox tab, katna_core::MailCategory (v2)
message_location (message_id, folder_id, uid)        -- one message, many folders/labels
participant      (message_id, role, email_norm, domain, display_name)
                                                      -- role: from|to|cc|bcc|reply_to|sender
attachment       (id, message_id, part_id, filename, mime, size, blob_hash NULL)
thread           (id, account_id, subject_norm, last_date, message_count, flags_summary,
                  gm_thrid)            -- Gmail X-GM-THRID (v2)
thread_ref       (account_id, message_id_hdr, thread_id)  -- referenced, not yet seen (v2)
op_queue         (id, account_id, op_json, state, attempts, next_try_at)
outbox           (id, draft_message_id, send_at, state, per_recipient BOOL, attempts)
notification     (notif_id, message_ids, account_id, created_at)   -- to close/update later
pop3_uidl        (account_id, uidl, message_id NULL, first_seen)   -- POP3 downloads (v3)
pin              (message_id, pinned_at)   -- pinned to the top of the list (v5)
translation      (message_id, target, source, source_hash, text, created_at)
                                            -- kept translations (v8, §16.4)
```

`participant` is the key table for organizations (§8) and address search.
`body_state` drives sync, search and UI (§6, §7).

Schema v1 is implemented in `crates/katna-store/src/schema/`. Conventions:
times are Unix seconds (UTC), hashes are 32-byte blake3 digests, JSON is
`TEXT`. The `account` table lives in `pim.db` (§5.4), shared by mail and
calendar; `account_id` columns in `mail.db` therefore have no foreign key.
Each database version is `PRAGMA user_version`; migrations are append-only.
Each of `mail.db` and `pim.db` has a `change_log` table (change journal):
every daemon write appends `(seq, object_kind, object_id, op)` in the same
transaction, and apps read entries after the last `seq` they saw when a
change signal arrives (§14.2).

Mail schema v2 (`mail_v2.sql`) adds threads and inbox categories:
`message.category`, `thread.gm_thrid` (unique per account), `thread_ref`,
indexes for thread lists and subject matching, and triggers that keep
`thread.message_count` and `last_date` right on every insert, move and
delete (a thread with no messages left is deleted). The category numbers
are stable: 1 Primary, 2 Promotions, 3 Social, 4 Updates, 5 Forums; NULL
means "not classified yet" and reads as Primary.

Mail schema v3 (`mail_v3.sql`) adds `pop3_uidl`: the server messages each
POP3 account has downloaded. `message_id` becomes NULL when the local
message is deleted, so it is not downloaded again and can be removed from
the server (§6.4).

Mail schema v5 (`mail_v5.sql`) adds `pin`: messages pinned to the top of
their folder's list (§13.5). Pins are Katna's own (IMAP has none), so they
stay on this computer; a pinned conversation pins each message it had.

Mail schema v6 (`mail_v6.sql`) adds `quota`: how full each account's mail
storage is, from IMAP QUOTA (`GETQUOTAROOT INBOX`, the STORAGE resource),
read on each full sync. The foot of the folder pane shows it for the
account whose folder is open ("34% of 15 GB used"); accounts whose server
reports no quota show nothing there. Sizes count in 1024s, as providers
sell storage.

Mail schema v7 (`mail_v7.sql`) adds `outbox.hold_until`: when scheduled
mail goes out, for mail an SMTP server holds (§11, Send later).

### 5.4 Shared PIM schema (sketch)

```sql
account          (id, kind, display_name, address, settings_json)  -- kind: imap|jmap|pop3|caldav|carddav|local
organization     (id, name, kind, color, notes, notify_policy)  -- kind: customer|vendor|partner|other
org_alias        (org_id, alias)
org_rule         (org_id, rule_kind, value)               -- domain | subdomain | address
address_book     (id, account_id NULL, source, remote_id, name, sync_token,
                  synced_at, state)                       -- source: google|microsoft|carddav|local (v6)
contact          (id, book_id, remote_id, etag, display_name, sort_key, job,
                  phone, starred, card_json, raw, updated_at)   -- §8.6 (v5)
contact_address  (contact_id, email_norm, position)
contact_group    (id, book_id, remote_id, name)           -- labels
contact_group_member (group_id, contact_id)
contact_photo    (contact_id, source, data)
org_member       (org_id, contact_id)
suggestion       (id, kind, payload_json, state)          -- pending | accepted | dismissed
template         (id, name, subject, html, text, updated_at)   -- mail templates (v2)
template_attachment (template_id, position, name, mime, data)
meta             (object_kind, object_id, plugin, value_json, version,
                  expires_at NULL, dirty BOOL)            -- see §10
```

## 6. Sync engine (`katna-sync`, runs in `katna-daemon`)

### 6.1 Account workers

One worker per account inside the daemon:

- **Foreground loop:** push on the inbox (IMAP IDLE / JMAP push), quick
  refresh of folders the user is looking at (apps tell the daemon over D-Bus).
- **Background loop:** backfill, text downloads for indexing, other folders.
- **Op queue:** replays local changes (flags, moves, deletes) to the server
  with retries and conflict handling.
- Incremental sync uses **CONDSTORE / QRESYNC** when available.
  Gmail uses `X-GM-EXT-1` (labels, thread IDs, `X-GM-RAW` search).
- **IDLE hygiene:** re-issue IDLE before the 29-minute limit; watching several
  folders needs several connections (or the NOTIFY extension); respect
  per-server connection limits (Gmail: about 15).
  Built so far (`katna_sync::worker`): the worker's connection IDLEs on the
  inbox, and one more connection per account asks the server for STATUS
  (message count, UIDNEXT, UIDVALIDITY, HIGHESTMODSEQ) of every other
  folder every 2 minutes (`WorkerConfig::watch_interval`). Folders whose
  numbers differ from their last sync (`engine::stale_folders`) are synced
  at once on the IDLE connection; a folder the store does not know yet
  starts a full sync. So mail a server-side filter files away shows up in
  about 2 minutes instead of at the 15-minute full sync. Without CONDSTORE
  a flag change does not show in STATUS and waits for the full sync.
  NOTIFY (RFC 5465) could replace the polling where servers have it.
- **Network and power:** reconnect on network changes (NetworkManager or the
  portal network monitor), after resume (logind `PrepareForSleep`), with
  exponential backoff; pause heavy background work on metered connections.
  Built so far (task 1.12, `katna_daemon::system`): the daemon watches the
  system bus for logind's `PrepareForSleep(false)` and NetworkManager's
  `StateChanged` rising to "connected, local" or better. Either makes
  every worker drop its connection without waiting on it (after a resume
  it may be dead, and an IDLE on it would hang until the read timeout) and
  connect again at once (`worker::Handle::reconnect`); a worker waiting to
  retry connects at once too. A refused password stays refused. While
  NetworkManager's `Metered` is "yes" or "guess yes" (a phone hotspot),
  workers keep headers, flags and queued changes in sync but download no
  bodies ahead of time; a message the user opens is still fetched, and
  when the network stops being metered every worker syncs and catches up
  (`worker::Handle::set_metered`). POP3 has no headers-only download, so
  it checks as usual. Without a system bus, logind or NetworkManager the
  daemon runs as before. The `sync.metered` setting (`auto`, `always`,
  `never`) wins over NetworkManager; Katna Mail saves it and calls
  `ReloadConfig`, and the daemon answers `Metered` and signals
  `MeteredChanged`. Not built yet: the portal network monitor (for
  Flatpak).

### 6.2 Sync levels (per account)

| Level | Downloaded | Scope | Default |
|---|---|---|---|
| **1. Metadata** | Envelope, flags, size, `BODYSTRUCTURE`, threading headers (`References`, `In-Reply-To`, `List-Id`), auth results | All mail | Always on |
| **2. Searchable text** | Text parts only (chosen via `BODYSTRUCTURE`), indexed, raw text then discarded | All mail, newest first | On (paused on metered networks) |
| **3. Offline copy** | Full message incl. attachments up to a size limit | User window: 7 d / 30 d / 1 y / all, plus "always keep" rules (starred, selected organizations) | 30 days |

Initial sync order: level-1 for the window → level-3 for the window →
level-1 for older mail (backwards in time) → level-2 for older mail.

When a message leaves the offline window, its blob is deleted but its
metadata and index entry stay (`body_state` goes from 2 to 1).

### 6.3 Opening a message that is not downloaded

- The app asks the daemon to fetch it; the daemon saves the blob, sets
  `body_state = 2` and signals the change.
- Pre-fetch neighbors while the user scrolls the list.
- Offline: show headers and indexed text with a "not available offline" note.
- Large attachments (> 5 MB default) are always fetched on click.
- An attachment chip in the list of such a message downloads it too: the
  chip (or its row in the "+N" list) fills from left to right, then the
  attachment opens. `FetchBody` reports no byte progress, so the fill is
  an estimate from the attachment's size that eases towards 90 % and runs
  out when the download ends. A failure says why in a toast; clicking
  again retries.

### 6.4 Protocol notes

- **IMAP / SMTP / JMAP:** Pimalaya `io-*` crates behind our `MailBackend` trait.
  Our own I/O drives them: `async-net` sockets, rustls, `async-io` timers, so
  `katna-sync` runs on any executor.
- **One task per connection:** an IMAP command cannot be stopped halfway, so
  each connection lives in its own task (`katna_sync::connection`) and the
  rest of the daemon holds cloneable handles. Dropping a caller only drops
  the answer. A request from any handle ends an IDLE wait cleanly (DONE),
  then runs.
- **A thread per account:** the engine calls the synchronous store from
  async code, so a call that waits (a big first sync, a slow disk, SQLite's
  busy timeout) blocks whatever runs it. The daemon runs each account's
  worker, the outbox and the contacts, tasks, notes, calendar, alarm and
  reminder loops on threads of their own (`katna-daemon/src/threads.rs`);
  smol's shared executor keeps only short work such as event forwarding and
  D-Bus signals.
- **Level-1 sync (`katna_sync::engine`):** per folder, SELECT with
  CONDSTORE, reset on a new UIDVALIDITY, fetch flags changed since the stored
  HIGHESTMODSEQ, fetch headers of new UIDs newest first in chunks of 500
  going back in time (committed chunk by chunk; the UID ranges still missing
  are saved in the folder's sync state, so a cut-off first sync fills in the
  older mail next time), and compare UID lists only when the message count does not add
  up. Servers with QRESYNC (Stalwart and Dovecot here; Gmail has none) get
  `ENABLE QRESYNC` after login: the flag fetch then carries the
  `VANISHED` modifier, so the expunges since the stored HIGHESTMODSEQ come
  with the flag changes and the UID list is only fetched if the count
  still does not add up. Pushed expunges arrive as `VANISHED` and wake the
  worker like `EXPUNGE` did. Headers come from `BODY.PEEK[HEADER.FIELDS (…)]` and are decoded by
  the same parser as the importer; INTERNALDATE stands in for a missing
  `Date`. The header list includes the threading headers and the ones the
  category classifier reads (`katna_core::category::CLASSIFIER_HEADERS`);
  each new message is threaded and classified as it is saved (§6.5). Each
  server copy of a message is its own row; copies share a thread, and the
  conversation reads show one per `Message-ID`.
- **Attachments before the body:** each header chunk is followed by
  `UID FETCH … (UID BODYSTRUCTURE)`, a command of its own so that a
  structure imap-codec cannot parse costs only that message's attachment
  list, never the message. The `attachment` table gets each attachment's
  body section (`2`, `1.3`), type, decoded file name (RFC 2231 and 2047)
  and estimated decoded size; `has_attachments` follows it. One rule,
  `katna_import::mime::is_attachment`, is shared with whole-message
  parsing so the paperclip does not change when the body arrives:
  anything marked `attachment` or with a file name, otherwise every part
  except the body text (text/plain, text/html, Gmail's text/x-amp-html and
  the like), the versions of a `multipart/alternative`, and pictures with
  a `Content-ID` (shown inline by the HTML). An attached message is one
  attachment; its parts are not listed. Without a usable structure,
  `has_attachments` is guessed from `multipart/mixed`.
- **Attachment lists for older mail:** a message flagged with attachments
  but without rows (synced before structures were read, a structure that
  did not parse, POP3 and imported mail) gets its list from the first source that
  has it: the body when it is downloaded (the list is read with the same
  body sections IMAP uses), the structure fetched again at the next sync
  when there is no body (up to 5,000 per folder per sync), or the daemon's
  background pass over bodies already stored. A list already stored is
  never replaced, and an empty one clears the flag, so each message is
  repaired once. The same pass drops unnamed body-text rows that older
  versions listed as files (Gmail's AMP body).
- **Header refresh:** messages synced before threading (no thread or no
  category, and no body to parse) get their headers fetched again, up to
  5,000 per folder per sync, and only the missing fields are filled.
- **Gmail (`X-GM-EXT-1`):** header fetches also ask for `X-GM-THRID`, and
  Gmail's thread id decides the thread. After syncing each folder, the
  engine runs `UID SEARCH X-GM-RAW "category:social"` (then promotions,
  updates, forums) over the UIDs not yet categorized; the rest of the
  folder is Primary. Every folder, not only the inbox: each server copy is
  its own message row, and the copy in All Mail must land in the same tab
  as the one in the inbox (found by `examples/thread_check.rs` on a real
  Gmail account).
  Header fetches also ask for `X-GM-MSGID` (mail schema v4,
  `message.gm_msgid`): Gmail shows one message in every folder it has a
  label for, and those copies are stored as one message row with one
  `message_location` per folder. Flags, tabs, threads and search see it
  once; removing a label removes only that location. Moving a message
  takes it from one of its folders other than All Mail, since leaving All
  Mail only adds a label on Gmail. Stores synced before v4 have a row per label, all without `gm_msgid`: the
  first sync after the upgrade fetches `UID FETCH … (X-GM-MSGID)` for them,
  once per folder, and merges the copies into one row
  (`MailBatch::adopt_gm_msgid`): its folders, flags and keywords are the
  union of the copies', it keeps a downloaded body and a pin, and a copy
  with a change still queued waits for the next sync. Nothing changes on
  the server. io-imap cannot express these extensions, so they are written as
  raw commands on the connection. Progress lives in `folder.sync_state`.
- **Level 3 so far (`katna_sync::bodies`):** after each full sync, and after
  each inbox catch-up, the worker fetches `BODY.PEEK[]` for messages in the
  offline window (`sync.offline_days`, Settings › General › Offline mail:
  7, 30 (default), 90 or 365 days, or all mail; up to 10 MB each), newest
  first, 25 per command. The raw message goes to the blob store; the
  snippet and attachment flag are recomputed from it and `body_state` is
  set to 2. A new window applies at once (`ReloadConfig`); a longer one
  starts a sync, a shorter one keeps what is downloaded. `FetchBody(id)` on D-Bus downloads any other message at once
  on a second, on-demand connection per account (so it never waits behind
  a running sync), which closes after two idle minutes. Katna Mail calls it
  when the reader shows a message that has no body yet and shows
  "Downloading…" meanwhile, or the reason and Try again if it fails.
  Level 2 (text only) and eviction come later.
- **Waiting for changes:** every wait starts with a NOOP, then IDLEs (or
  sleeps and NOOPs on servers without IDLE). Stalwart 0.16 reports changes
  made between two commands on NOOP only, never when IDLE starts.
- **Account worker (`katna_sync::worker`):** one per account. It syncs every
  folder, then loops: catch up on the inbox, IDLE on it (renewed every
  25 minutes), and sync every folder again every 15 minutes. Any network
  or protocol error ends the session; it reconnects after 2 s, doubling up
  to 5 minutes, and a session that synced resets the wait. A refused
  password is never retried (it would lock the account on many servers);
  the worker reports it and waits to be stopped. It talks to the daemon
  through an event channel and a stop handle.
- **Op queue (`katna_sync::ops`):** `SetFlags`, `MoveMessages`,
  `DeleteMessages` and `ArchiveMessages` change the store at once and queue
  one operation per server copy of a message (table `op_queue`, JSON). The
  worker replays the queue before each sync step and when the daemon asks,
  so a sync never overwrites a change still on its way.
  - Flags go out as `UID STORE +FLAGS.SILENT` / `-FLAGS.SILENT` with only
    the bits that change, so replays do not undo other clients' changes.
  - Moves use `UID MOVE`, or `COPY`, `\Deleted` and `UID EXPUNGE`. Without
    UIDPLUS the originals stay marked `\Deleted`, because a plain `EXPUNGE`
    would also remove what other clients marked. The `COPYUID` answer gives
    the new UID; without it the target folder is synced again.
  - Delete moves to the `\Trash` folder, else to a top-level (or
    `INBOX/`) folder named like one ("Deleted Items"), or expunges when
    the message is already there or there is no trash
    (`Store::trash_folder`, which the app also reads: a delete for good
    says "deleted forever" and offers no Undo). Archive moves to
    `\Archive` (or Gmail's `\All`).
  - A move out of a folder the message was only just moved into (Undo
    right after Archive) queues with no UID; when the earlier move runs,
    its `COPYUID` answer is handed to the waiting one, so the pair
    replays in order even offline.
  - A refused operation is retried after 60 s. After three refusals it is
    marked failed (kept for inspection) and undone locally: moves at once,
    flags by forgetting the folder's HIGHESTMODSEQ so the next sync reads
    them again. A broken connection keeps the operation queued.
  - Imported (`local`) accounts only change in the store.
- **POP3** (task 1.10, `katna_sync::pop3`): our own small client (RFC 1939
  with CAPA and STLS; USER/PASS login). POP3 mail is always fully local and
  stored like imported mail, in local folders `INBOX`, `Sent` and `Trash`;
  flag changes, moves and deletes stay in the store, and sent mail is filed
  in the local Sent folder. POP3 has no push, so the worker checks every 5
  minutes and on `SyncNow`, and holds a connection only while checking (the
  server locks the maildrop for a session). A check lists `UIDL` and
  `LIST`, downloads (`RETR`) every UIDL not in `pop3_uidl`, newest first,
  committing each message at once, then deletes on the server what the
  account's `Pop3Keep` says: everything once stored (leave-on-server off),
  mail older than N days, or mail deleted for good in Katna (the default,
  as in Thunderbird). Deletions happen at `QUIT`; only then does the store
  forget those UIDLs and the ones the server no longer lists. A broken
  session therefore never loses mail or deletes what is not stored. The
  client has `TOP`; partial download of very large messages (header first,
  body on request) is a later option. Discovery reports a POP3 server
  beside IMAP; Add account takes POP3 only when there is no IMAP server,
  or when the user picks it under Server settings. `SetPop3Keep` changes
  what stays on the server later (Settings > Accounts, "Mail on the
  server").
- **Gmail / Microsoft:** OAuth2 (`katna_sync::oauth`, daemon
  `daemon/sign_in.rs`, D-Bus `SignIn`): the installed-app flow with PKCE
  (RFC 7636) and a loopback redirect (RFC 8252). The daemon listens on a
  free port of `127.0.0.1` (`http://127.0.0.1:PORT/` for Google;
  `http://localhost:PORT/` for Microsoft, which registers loopback
  redirects under `localhost`, so `::1` is listened on too), opens the
  provider's page in the default browser (the OpenURI portal, else
  `xdg-open`), and trades the code for tokens with our own HTTPS client
  (rustls). Scopes: Google `https://mail.google.com/ drive.file openid
  email profile` (with `access_type=offline` and `prompt=consent`, so every
  sign-in brings a refresh token; `drive.file` is for large attachments,
  §6.6, and Google refreshes are sent without scopes so grants from before
  it keep working); Microsoft `IMAP.AccessAsUser.All SMTP.Send
  offline_access openid email profile` on `outlook.office.com`, and Graph
  `Files.ReadWrite` allowed on the same screen for OneDrive (§6.6). The ID token
  names the address (Microsoft's personal accounts only in
  `preferred_username`), the name and, for Google, a picture. The refresh
  token goes to the Secret Service in the account's password slot, and
  `AccountSettings.oauth` names the provider. A `TokenSource` per account,
  shared by its connections, hands out access tokens and refreshes them
  two minutes before they run out (one refresh at a time), saving the
  refresh tokens Microsoft replaces. IMAP and SMTP log in with SASL XOAUTH2,
  which both providers take; a refused access token is dropped and a fresh
  one tried once. A refused refresh token (`invalid_grant`) is an auth
  failure: the account stops syncing and Katna Mail shows "Sign in again",
  which runs `SignIn` for that account. Network trouble while refreshing is
  not, and retries like any other. The client IDs live in
  `katna_core::ids` (`GOOGLE_OAUTH_CLIENT_ID`, with Google's non-secret
  desktop `GOOGLE_OAUTH_CLIENT_SECRET`, and `MICROSOFT_OAUTH_CLIENT_ID`),
  filled at build time from `KATNA_`-prefixed environment variables of the
  same names, which the package build takes from GitHub secrets, so none is
  in the repository. Refresh tokens are kept like passwords (Secret Service,
  or Credential Manager on Windows, where a token too long for one entry is
  split over `<user>~1`, `<user>~2`, …); an empty one hides that provider's button and makes `SignIn` fail.
  Google's restricted scope for full mail access requires app verification
  and a yearly security assessment; until then Google lets only test users
  in. Tests use a local fake OAuth server and a fake IMAP server, never a
  real provider. App passwords keep working for Gmail; setting a password on
  an OAuth2 account (`SetPassword`) switches it back to the password.
- **Zoho** (tasks and calendars; `AccountSettings::linked`,
  `daemon/linked.rs`): a sign-in linked to an account, not its mail login,
  because Zoho lets only "self client" apps use XOAUTH2 for IMAP, so Zoho
  Mail keeps its password. `SignIn("zoho", account)` asks at once for
  `ZohoMail.tasks.ALL` (Zoho Mail's Tasks API, which Zoho ToDo serves),
  `ZohoMail.accounts.READ` (the Mail `accountId` and `zuid` task calls take),
  `ZohoCalendar.calendar.ALL` and `ZohoCalendar.event.ALL`, and
  `AaaServer.profile.READ` (who signed in, from `/oauth/user/info`; Zoho
  gives no ID token). Zoho scopes are separated by commas. Zoho takes
  only registered redirect URIs, port included, so its loopback is the
  fixed `http://localhost:53710/` (as rclone registers its own). It starts at the data centre of the
  account's mail host or domain (`accounts.zoho.com`, `.eu`, `.in`,
  `.com.au`, `.jp`, `.sa`, `.uk`, `zohocloud.ca`). Zoho sends every other
  user on and names their `accounts-server` in the answer: the code goes
  there, but only if it is one of those servers. The account keeps that
  server and the `api_domain` of the token answer. Zoho answers a refused
  token with 200 and `invalid_code`, which counts as signing in again.
  The refresh token is a Secret Service item of its own
  (`linked-account` attribute, so a password lookup never finds it).
  `Daemon::linked_tokens` hands it to the tasks and calendar syncs. The
  client ID and secret come from `KATNA_ZOHO_OAUTH_CLIENT_ID` and
  `KATNA_ZOHO_OAUTH_CLIENT_SECRET` at build time.
- **Account setup** (task 1.2, `katna_sync::autoconfig`, D-Bus
  `DiscoverAccount`): the user gives an address and the daemon finds the
  servers, in Thunderbird's order. First built-in settings for Gmail,
  Yahoo, iCloud and Fastmail. Then the provider's own `config-v1.1.xml`
  (`https://autoconfig.DOMAIN/…` and `https://DOMAIN/.well-known/…`) and
  Thunderbird's ISPDB, fetched at once; the provider's file wins. Then DNS
  SRV (`_imaps`, `_imap`, `_pop3s`, `_pop3`, `_submissions`,
  `_submission`; RFC 6186 and 8314), then the ISPDB entry of the MX host's
  domain (hosted mail such as Google Workspace), then probing `imap.`,
  `pop.`, `pop3.`, `mail.` and `smtp.DOMAIN` on 993/143, 995/110 and
  465/587 for a mail greeting. POP3 servers are kept beside IMAP
  (`Discovered::pop3`); IMAP stays the default when both exist. Files come only over HTTPS; TLS
  beats STARTTLS beats plain, and a cleartext server is only taken when
  the file offers nothing else (logged as a warning; the dialog shows the
  security). DNS answers are not authenticated, so an SRV record is only
  used when its target is the domain itself, a host under it, or a server
  of a provider Katna knows (the built-in list, Google, Microsoft), as
  RFC 6186 §6 asks; the resolver uses a random transaction ID and checks
  that the answer echoes the question's name, type and class. Servers of Google and Microsoft are marked
  for OAuth2 sign-in (`Discovered::oauth`); Outlook.com, Hotmail, Live and
  MSN addresses and files whose IMAP server takes only OAuth2 at a known
  provider get that provider's servers and no password step; other
  OAuth2-only servers are skipped.
  The HTTP client and DNS resolver are our own few hundred lines (UDP to
  `/etc/resolv.conf` servers), not a crate, to keep the daemon small.
  Adding the account still checks the login.

### 6.5 Threading

Threads are assigned incrementally when a message is inserted
(`katna-store`, `thread.rs`), not rebuilt with a full JWZ pass:

1. **Gmail:** with an `X-GM-THRID`, the thread holding that id wins. A
   thread built from references takes the id over; two Gmail threads are
   never merged.
2. **References:** the threads of the messages named in `References` and
   `In-Reply-To`, plus threads waiting for this message's own `Message-ID`
   in `thread_ref` (a reply that arrived before its parent). When a message
   links two or more threads they are merged into a Gmail one if there is
   one, else the oldest; the others are deleted and journaled.
3. **Subject:** a message whose subject has a reply or forward prefix
   (`katna_core::subject`, many languages, `[list]` tags removed) joins the
   newest thread with the same normalized subject whose last message is
   within 30 days.
4. Otherwise a new thread. References not found yet (up to 16) go into
   `thread_ref`, so a later parent finds the thread.

Thread changes are journaled as `Thread` entries rather than `Message`
entries, so search does not reindex messages whose thread changed.

**Reads** (work on a read-only store): `folder_threads` lists a folder's
conversations newest first, optionally for one inbox tab (the tab of the
conversation's newest message in the folder); `thread_summaries` gives
counts, unread and flag state and senders for a page; `thread_messages`
lists a conversation oldest first, one copy per `Message-ID`, hiding
trash and junk copies unless the whole conversation is there;
`category_unread` counts unread conversations per tab.

**Inbox categories:** `katna_core::classify` is a pure function of a few
headers (sender domain, `List-Id`, `List-Post`, `List-Unsubscribe`,
`Precedence`, `Auto-Submitted`, bulk-mail service headers, reply prefix).
It returns Social, Forums, Promotions, Updates or Primary. On Gmail the
server's categories replace it for the inbox (§6.4).

**Old stores:** the daemon threads and classifies messages stored before
schema v2 on a background thread (`katna_import::backfill`), 500 messages
per transaction with a short pause between batches, parsing the stored
blob's headers. It fills only fields still NULL, so it is idempotent, and
it resumes after a restart from the messages still unthreaded (a partial
index keeps finding them cheap). On a synthetic 100k-message store it takes
about 14 s. Messages without a blob are covered by the header refresh (§6.4).

### 6.6 Large attachments through Google Drive and OneDrive

Gmail takes messages of up to 25 MB and sends larger files as Drive links;
Katna does the same for accounts signed in with Google, with no extra
steps in the common case.

- **Permission.** Google sign-in asks for `drive.file`: only files Katna
  itself put in the Drive, never the rest of it. The token answer names
  the granted scopes and `TokenSource` keeps them; an account signed in
  before (or with the Drive box unticked) shows **Allow Drive** on the
  file's chip, which signs it in again.
- **Upload.** A file that would take the message past the limit is never
  read into memory: Katna Mail asks the daemon (`DriveUpload`) to upload
  it the moment it is attached. `katna_sync::drive` uses Drive v3's
  resumable upload in 8 MiB chunks over our own HTTPS client; after a
  dropped connection it asks Drive how much arrived and goes on from
  there. `DriveChanged` signals report progress (each percent) and the
  end, and the chip shows it.
- **Send.** Send waits for uploads under way and then goes by itself. It
  shares each file with every recipient as a viewer, without Google's
  own sharing mail. Only when Drive refuses some address (no Google
  account, or an organisation's rule) does it ask: share with anyone who
  has the link, send without sharing, or cancel. The message carries each
  file as a link card under the text (and the link in the plain text).
- **Removing** a chip before sending moves its file to the Drive's bin.
  Saved drafts keep the links.
- **Tests.** `KATNA_GOOGLE_TOKEN_URL` and `KATNA_GOOGLE_API_URL` point the
  daemon at a fake Google on this computer; only `http://127.0.0.1:…` and
  `http://localhost:…` are taken, so they can never send tokens elsewhere.

**OneDrive.** Accounts signed in with Microsoft (Outlook.com, Hotmail,
Microsoft 365) do the same through OneDrive, as Outlook does, in
`katna_sync::onedrive` over Microsoft Graph:

- Microsoft sign-in also asks for `Files.ReadWrite` (Graph), on the same
  consent screen. Microsoft tokens are for one resource at a time, so the
  code still buys IMAP and SMTP tokens, and `TokenSource::access_token_for`
  trades the same refresh token for Graph's when a file goes up (saving a
  rotated refresh token as usual). An account whose sign-in never allowed
  it gets `invalid_grant` there, and its chip shows **Allow OneDrive**.
  `Files.ReadWrite.AppFolder` would be narrower, but Graph does not
  promise sharing (`invite`, `createLink`) under it.
- Files go to Katna's app folder (`Apps/Katna`) in an upload session, in
  10 MiB pieces (a multiple of 320 KiB). The session address carries its
  own authorisation, so pieces go without a token; after a dropped
  connection the session's `nextExpectedRanges` says where to go on.
- Send invites each recipient as a reader with `sendInvitation: false`
  (no Microsoft mail). A refused address asks the same question; sharing
  with the link creates an anonymous view link, which replaces the file's
  own address in the message (`DriveShareWithLink` returns the links).
  Removing a chip deletes the file (to the recycle bin).
- `KATNA_MICROSOFT_TOKEN_URL` and `KATNA_GRAPH_API_URL` point the daemon
  at a fake Microsoft, loopback only, like Google's.

## 7. Search (`katna-search`)

### 7.1 Index design

One tantivy index for all accounts, written by the daemon and read by apps,
KRunner and the GNOME search provider. Documents store only IDs and fields
needed for filtering and ranking; display data comes from SQLite.

| Field | Type | Notes |
|---|---|---|
| `msg_id` | u64, fast, stored | Key into SQLite. |
| `account`, `folder`, `labels` | facet / string | |
| `from`, `to`, `cc` | text (names + addresses) | Tokenized for free text. |
| `from_exact`, `rcpt_exact`, `domain` | string (raw) | Exact matching for `from:` and organizations. |
| `subject` | text, boosted | |
| `body` | text | Only when `body_state >= 1`. |
| `attachment_names`, `attachment_text` | text | Text from PDF / docx / odt, with size limits. |
| `date`, `size` | i64, fast | Range filters, recency boost. |
| `flags`, `has_attachment`, `list_id`, `lang` | string / fast | |

### 7.2 Query language (Gmail-compatible where possible)

```
from:alice  to:bob  cc:  org:acme  subject:  has:attachment  filename:pdf
in:inbox  label:x  is:unread  is:starred  is:important  before:2025-01-01  after:  newer_than:30d
larger:5M  smaller:  list:  "exact phrase"  -exclude  OR  ( )
```

Search options build these queries. Its file types after "Has attachment"
(PDF, XLSX, ODF for `.odf`/`.odt`, XLS, ODS, PPT, PPTX, and typed
extensions under Custom) become `filename:pdf` or
`filename:(pdf OR xlsx)`: mail with an attachment of any chosen type.
From and To suggest addresses from the same address book as compose's
recipient fields (§7.8); a picked one fills the field.
The panel is at least 640 px wide while the window allows, puts labels
above fields below 600 px and scrolls when taller than the window.

Free text that matches an organization name or alias is expanded to
"any participant matches that organization" **plus** normal text matching (§8.3).

### 7.3 Execution

1. **Local search**, always, on every keystroke (debounced ~30 ms, older
   queries cancelled). Results stream in; counts come after.
2. **Server search**, only for mail whose text is not indexed yet (level-2
   not finished or disabled). Starts on Enter or a typing pause, only for
   queries with free text, restricted to the non-indexed date range.
   Uses IMAP `SEARCH`, Gmail `X-GM-RAW`, JMAP `Email/query`. Results show
   in a separate "More results on server" section. The server returns IDs;
   metadata is already local, so results render and rank instantly.
3. Messages found on the server are downloaded and indexed.

Results replace the list and close the open conversation. The list, its
scroll place, the cursor and the open conversation are kept as the search
starts: cancelling it (Escape or clearing the box) with no result opened
brings them back; opening a result drops them.

### 7.4 Ranking

BM25 plus boosts: subject match, recency, people you reply to often,
organization membership, starred/important. Exact-phrase and field matches
rank above loose matches. Highlighted snippets via `SnippetGenerator`.

### 7.5 Indexing pipeline

- Parse (`mail-parser`) → HTML to text → language detection → per-language
  tokenizer/stemmer → attachment text extraction → index writer.
- Parallel workers with a memory budget; commit in batches; tantivy commits
  are atomic, so a crash never corrupts the index.
- Search works on the already-indexed part while initial indexing runs.
- Index schema is versioned; a version change triggers a background rebuild.

### 7.6 Targets and benchmarks

- Query latency: p50 < 20 ms, p99 < 50 ms on 1M messages (target, to be measured).
- Benchmark corpus: Enron (~500k messages). Compare with notmuch, mu and
  Thunderbird, and publish the numbers.
- `katna-search-cli` is built **first**, before the GUI.

### 7.7 Phase 0 implementation

Implemented in `crates/katna-search` (plan tasks 0.5–0.8). What differs
from or adds to the sketch above:

- **Fields.** As in §7.1, except: `domain` holds every participant domain
  and its parent domains (`mail.enron.com` → also `enron.com`) and backs
  `org:` until organizations exist (Phase 2); `folder` holds each folder
  path, its components (`lay-k/inbox` → `lay-k`, `inbox`) and its role;
  `flags` is a fast field for ranking. `lang` and attachment text are not
  indexed yet.
- **No text stored.** Snippets are made at query time from the raw message
  in `blobs.db` (first 64 KB), so the index stays small and holds no copy
  of the mail. Twenty snippets take a few milliseconds.
- **Tokenizer.** One analyzer for all text fields: Unicode words,
  lower-cased, folded to ASCII, tokens over 40 bytes dropped. Addresses
  split into words, so `from:kenneth.lay` is the phrase "kenneth lay" and
  also matches the display name. No CJK segmentation yet.
- **Stemming.** Subject and body are also indexed through the English
  Snowball stemmer, in `subject_stem` and `body_stem`, with frequencies but
  no positions. A single whole word searches the stemmed subject (boost
  1.5, on top of the subject's 3, so the word as written ranks first) and
  the stemmed body instead of the body; `contract` finds `contracts` and
  `contracted`. Phrases and the word being typed match as written, since
  the stemmed fields have no positions and a stem is not a prefix of every
  longer form. Snippets highlight other forms too, stemming only the words
  that start like a searched stem. Every message is stemmed as English
  until languages are detected (`whatlang`, later). Cost on the 500k
  synthetic corpus: index 348 → 475 MiB (+36 %), indexing time unchanged,
  query latency within noise (p50 6.1 ms, p99 20 ms).
- **Query language.** `OR` binds tighter than the implicit AND, as in
  Gmail (`a OR b c` is `(a OR b) c`). `to:` matches To, Cc and Bcc. Dates
  are UTC days; `before:` excludes the day, `after:` includes it. A time
  and offset may follow (`after:2001-05-14T09:30+06:00`); the app's
  custom date filter writes local midnight that way. Unknown
  `word:value` is plain text; broken parentheses are ignored; nesting is
  limited to 32 levels.
- **As you type.** `Query::parse_as_you_type` treats a final unfinished
  word as a prefix (`budg` finds `budget`; `"natural g` keeps the phrase
  order). In the body, and in phrases, the prefix expands to at most 16,
  32 or 64 index words for one, two or more letters. In the other fields
  (names, subject, file names) it expands to every word: expansion takes
  words in alphabetical order, so a cap made `hasina b` miss Hasina Banu
  whenever enough other names started with "ba…". On the 500k synthetic
  corpus that costs up to ~12 ms for one letter (`k`: p50 18 ms).
- **Typos.** One word of four or more letters that no searched field has
  as typed also matches words one typo away (four letters) or two
  (longer), a swap of neighbours counting as one, in From, To, Cc and Bcc
  (`haskina banu` finds Hasina Banu). Near matches score 0.3 of an exact
  one. If a search finds nothing, it runs again with such words matching
  near words in every field (`scool fees`), and `SearchResults::fuzzy`
  tells the app to say so. Words that exist as typed never match near
  words, which keeps correctly spelled searches as fast as before (within
  noise on the synthetic corpus); a misspelled search costs about as much
  as a correct one (4–25 ms p50 on the synthetic corpus). Snippets do not
  highlight near matches yet.
- **Did you mean.** `SearchIndex::suggest` rewrites the typed text with
  each word that is not in the mail as typed (four or more letters) swapped
  for the nearest word that is: fewest typos, then the same first letter
  (`kenet` → kenneth, not genex), then the most messages, in
  the fields that word searches (`from:Hasnia` looks only at senders).
  Quoted phrases, `-words` and `OR` are left alone. As you type, the
  unfinished last word counts as found if any word starts with it, and is
  otherwise completed from a word that starts one typo from it (`haskin` →
  hasina). The mail app works like a web search: it searches the corrected
  text right away if that finds anything, shows "Showing results for …",
  and offers "Search instead for …" to search the text as typed.
- **Ranking.** BM25 with field boosts (subject 3, from 2, attachment names
  1.5, others 1), times a recency factor `1 + 0.5 · 2^(−age/60 days)` where
  age is measured from the newest indexed message (so an old archive still
  ranks by recency), times 1.2 for starred mail. Queries without free text
  (only operators such as `from:`) are sorted newest first instead.
- **Keeping up to date.** The tantivy commit payload stores the schema
  version and the `mail.db` change-journal sequence number the index
  covers, atomically with the documents. The first run scans all messages
  (resumable, committing every 100,000); later runs re-index the messages
  the journal lists. An index of another schema version is deleted and
  rebuilt when opened for writing; opened read-only (as apps do) it is an
  error until the daemon has rebuilt it.
- **In the daemon.** `katna-daemon` starts the indexer once it owns its bus
  name (so a second instance never writes the index) and wakes it on every
  `MailChanged` notice. If the index cannot be opened, mail still syncs
  and the error is logged. Linking tantivy grows the daemon from 11.6 to
  15.0 MB (then 14.3 MiB of a 15 MiB budget; it is 50 MB now, §17.2).
  `katna_search::Indexer` runs updates on its own thread
  with its own read-only store connection: once at start, then on
  `Indexer::changed()` (the daemon calls it after each sync) and every 5 s
  as a fallback for other writers. Calls while it is busy coalesce into one
  more update. `IndexEvent`s report progress, finished updates and
  failures (retried at the next wake). `Indexer::stop` makes a running
  update commit what it has done and return within one batch; the next
  start carries on. Apps open the index with `SearchIndex::open_read_only`,
  which reloads by itself within about 0.5 s of each commit (tantivy's
  `OnCommitWithDelay`), so no D-Bus signal is needed for search.
- **Tools.** `katna-search-cli index|query` and `katna-bench search|synth`
  (synthetic corpus of Enron's shape for machines without Enron). The
  nightly `bench` job fails on a p99 over 50 ms or on overall p50/p99 more
  than 10 % (and 1 ms) slower than the last good run; the nightly `fuzz`
  job runs `fuzz/` (query parser and compiler) for 10 minutes.

### 7.8 Recipient suggestions

To, Cc and Bcc suggest addresses as the user types, like Gmail.

- **Source.** `Store::correspondents` counts, per account and address, the
  mail the user sent to it, received from it and was copied on with it,
  with the newest date of each. Mail counts as sent when its From is the
  account's own address or it sits in a folder with the `sent` role. The
  app builds a `katna_search::contacts::ContactBook` from it in the
  background, keeps a copy in `cache_dir/addresses.json` (mode 0600) so the
  next start has it at once, rebuilds it after 10 minutes, and counts each
  message the moment it is sent.
- **Matching.** Each typed word must match the start of a word in the name
  or address (address words split on `.`, `_`, `-`, `@` and the like). From
  three letters on one typo is allowed, from six two, never in the first
  letter. A match at the very start beats a word start, which beats a typo.
- **Ranking.** score = fit × (1 + affinity), where affinity adds, over the
  user's accounts, 3 × sent + received + 0.3 × copied, each as
  ln(1 + count) halved for every year since the last such mail. Accounts
  other than the one writing count half. Eight rows are shown; people
  already in To, Cc or Bcc are left out.
- **Speed.** Candidates come from an index by first letter, and the marks
  that bold the matched text are worked out for the shown rows only: under
  6 ms a key on 100,000 contacts in a release build.
- **Chips.** A finished recipient becomes a chip (comma, semicolon, Enter,
  Tab, leaving the field, picking a suggestion, or pasting several). A chip
  shows the name, or the address when there is none; a named chip has an
  arrow that opens a card with the address, and a double-click puts it back
  into the field for editing, in place. Anything `outgoing::valid_email`
  rejects stays as a red chip, and Send, Send and archive and scheduled send
  stop with a "Check the address" dialog until it is fixed or removed. The
  chips live in `Compose.chips` (`compose/chips.rs`); drafts and sending
  still read the fields as one "a, b, c" text.
  A chip can be dragged to another of To, Cc and Bcc (hidden Cc and Bcc
  rows open while a chip is dragged), and every chip ends in an x that
  removes it.

## 8. Organizations (`katna-org`)

### 8.1 Model

An organization has a name, aliases, a kind, a color, a notification
policy, and **rules**: domain (`@acme.com`), subdomains (`*.acme.com`) and
exact addresses (`john.k@gmail.com`). People (contacts) can have many
addresses and belong to many organizations.

### 8.2 Matching at query time

Organizations are **not** stored on messages. An organization view is a
query: "messages where any `participant` matches the organization's rules".

- Adding an address to an organization instantly includes all old mail
  from and to that address, in all accounts and folders, sent and received.
- New mail is covered as soon as its participants are stored.
- Only addresses are needed, so it works for every message from sync level 1.

### 8.3 Search integration

- `org:acme` → expanded to a term set over `from_exact`, `rcpt_exact` and `domain`.
- Plain text "acme" that matches an organization name or alias → the same
  expansion **OR** normal text matching.
- Organization facet chips on results ("Acme (132)").
- Organizations are also KRunner / GNOME search results (§15.3).

### 8.4 Auto-sorting without breaking other devices

- Each organization is a **smart folder** in the sidebar (a live view).
- Optional: write an IMAP keyword (e.g. `$Katna_Acme`) so other clients see it.
- Optional rule action: really move mail on the server.

### 8.5 Suggestions (user confirms; nothing changes silently)

- Many senders from one non-public domain → "Create organization?"
- Unknown address often in threads with an organization → "Add to Acme?"
- Company name in display name or signature → suggestion.
- Never create domain rules for public providers (gmail.com, outlook.com,
  yahoo.com, …; bundled list). Exclude the user's own domains.

### 8.6 Sync and reuse

Organizations and contacts are stored as vCards (`KIND:org`, `ORG`,
`MEMBER`) and can sync via CardDAV. Katna Calendar and the Plasma clock use
the same matching on event attendees ("Meeting with Acme").

**Saved contacts** (study: `research/contacts/katna-contacts-study.html`
in the project files, 2026-09-29). Each account's own address book is
synced, following the feature rule: Google's People API for Gmail (labels
are contact groups, starred is Google's `starred` group, pictures are
fetched once per URL), Microsoft Graph for Outlook (the main contacts
folder, categories as labels), and CardDAV (RFC 6352) for the rest, found
from the provider's known server or the `.well-known/carddav` of the mail
and IMAP domains, read with `sync-collection` and `addressbook-multiget`
(`katna_sync::{contacts, carddav}`; vCard 3.0/4.0 in `katna_dav::vcard`).
Each account tries its best way first and the others when that one is not
available (`katna_sync::methods`, `Data::Contacts`): a Google sign-in uses
the People API, then Google's CardDAV server with the same token (scope
`carddav`, CardDAV API enabled in the Cloud project); Outlook
has only Graph; a password account has CardDAV. The way that worked is
remembered and replaces the account's address books from any other way.
The daemon syncs 20 s after start, every 15 minutes, on Sync now and after
a sign-in, and signals `ContactsChanged`. Every source is read into one
`katna_core::contact::Card`, kept as JSON beside the source's own form
(`contact.raw`), which is what writing back will edit. Google and Microsoft
accounts signed in before Katna asked for contacts get an "Allow" banner on
the Contacts page (sign in again with the added scope).

The same person saved in several accounts is one entry, linked by a shared
email address (`Store::saved_contacts`). Saved names win in the address
suggestions and saved addresses are suggested even without mail; saved
pictures show beside the person's mail. The Contacts page (app rail) is
laid out like Google Contacts: Contacts, Frequent (the people from the
mail) and the labels at the left, a list with Name, Email, Phone, Job
title & company and Labels, and a contact's page with tinted cards
(details, the accounts that keep it, notes) and Email, Mail and Call
buttons.

Changes go to the account first (`SaveContact`, `DeleteContacts` on
D-Bus): People API `createContact` / `updateContact` (with the card's
etag; labels and the star left alone) / `deleteContact`, Graph `POST` /
`PATCH` / `DELETE /me/contacts`, and CardDAV `PUT` (`If-None-Match: *` for
a new card, `If-Match` with the ETag for a change, so a change made
elsewhere is not overwritten; the old vCard's other properties are kept)
and `DELETE`. What the service answers is saved in `pim.db`. A new contact
goes to the book picked under "Save to" (an account, or this computer); a
person kept in several accounts is changed in the first one. Delete hides
the person and waits for its Undo to go before it is sent; Google and
Outlook keep deleted contacts in their trash.

Labels (`SetContactLabels`, `RenameContactLabel`) are written the way each
service keeps them: Google contact groups (`contactGroups` create, rename
with the group's etag, `members:modify`, delete with `deleteContacts=false`;
Google's own groups such as My Contacts are not shown as labels), Graph
`categories` on each contact, CardDAV `CATEGORIES` on each card (Apple-style
group cards are read, not changed), and a card on this computer by name. A
label ticked on a person goes on each of their cards; renaming or deleting
a label changes it in every account, and its people stay. A label nobody
has is forgotten except at Google, which keeps empty labels. Each change
has an Undo; "Email everyone" on a label starts a message to all of them.

Other contacts are Google's list of people a Gmail account mailed but never
saved (People API `otherContacts`, scope `contacts.other.readonly`, read
with a sync token each pass whatever way the account's own contacts come).
They live apart from saved cards (`other_contact`, pim.db v10), so they
never merge into people or labels. The page lists them under Other
contacts, leaving out anyone saved since; Add to contacts copies one with
`copyOtherContactToMyContactsGroup` (`SaveOtherContact`) and has an Undo.
Outlook and CardDAV have no such list.

"Fix and manage" at the foot of the column has Merge and fix, Import,
Export and Print. Merge and fix suggests people who look like the same person (the
same name, or a phone number ending in the same ten digits; people who
share an address are one person already). Merging combines their cards
(the first card's name, then every address, number, link and label the
others add) and keeps one card per address book, deleting the rest there;
the Undo writes the old cards back. Dismissed suggestions are kept in the
settings file (`[contacts] dismissed_duplicates`). Import reads vCard files,
and CSV files as Google Contacts, Outlook and Thunderbird export them (each
column known by its heading), in the app and saves the new people with their categories as labels in
the account in view (`ImportContacts`, with an Undo), leaving out anyone
already saved; Export writes the people on screen (everyone or a label) as
one vCard 3.0 file, and Print prints them (their name, job and details, in
mail's print preview). A person's page prints them alone and shows them as
a QR code of their vCard, without notes or picture (addresses and links are
dropped when it would not fit), which a phone's camera saves.

The column also lists every mail account under Accounts, with how many
people are saved in it (a click lists only those). The daemon keeps where
each account's contacts sync stands (`Pim1.ContactsStatus()`, a
`contacts_state`: ok, needs-sign-in, use-sign-in, error or none, sending
`ContactsChanged` when one changes); an account whose contacts did not
come shows one line under it (`window/account_status.rs`, shared with
Calendar and Tasks) with why and its fix: "Sign in again to show
contacts" (OAuth2 without the contacts scopes), "Sign in with Google" (a
Gmail or Outlook account added with a password, which their contacts
need), Change password (a server that refused the password; many need an
app password), or Try again (`SyncNow`, which looks for the address books
from scratch); "none" carries what the server answered.

Saved people's birthdays show on the Calendar and the agenda as a
Birthdays calendar made on this computer (id -1, read-only, never stored):
a yearly whole-day event per person, built from the cards each time the
calendar is read, leaving out one a mail service's own calendar already has
(a birthday event that day with their first name). Unticking it is kept in
the settings (`[contacts] hide_birthdays`); a click opens the person's page.
KRunner and GNOME search suggest saved people too, with their saved names
(`ContactBook::with_saved`), read again when contacts change.

## 9. Background service (`katna-daemon`)

### 9.1 Responsibilities

- Sync for all accounts (mail, CalDAV, CardDAV), push/IDLE.
- Search indexing.
- The metadata scheduler (§10): undo send, send later, snooze, reminders.
- Calendar alarms.
- Notifications with actions and inline reply (§15.1).
- D-Bus API for the apps and desktop integrations (§14).
- KRunner runner and GNOME Shell search provider (§15.3), served in-process.
- Tray icon and unread count on the taskbar icon (§15.2).

### 9.2 Lifecycle

| Environment | How it starts |
|---|---|
| Any desktop | "Start Katna at login" (Settings > General > Desktop, on by default): an XDG autostart entry running `katna-mail --background`, which starts the daemon by D-Bus activation and exits without a window; "Open the window too" drops the flag. Turning it off also disables the systemd unit. |
| systemd | `katna-daemon.service` (systemd user unit); D-Bus activation goes through it. Enabling it by hand starts the daemon at login without Katna Mail. |
| Any session | **D-Bus activation** (`in.invenia.katna.Daemon.service`): starts on demand when an app, KRunner, the clock plugin or a notification action calls it. |
| No systemd | XDG autostart `.desktop` file. |
| Flatpak | **Background portal** (`RequestBackground` with autostart). KDE and GNOME both implement it; GNOME lists it under "Background Apps". |

- User settings: "Keep running in background" (default on), optional tray
  icon, and a real "Quit" (stops the daemon until next login or activation).
- Single instance, enforced by owning the D-Bus name.
- Graceful shutdown: finish in-flight sends, flush the index, close IMAP sessions.
- Updates: a package update replaces the binary while the old one runs.
  Every 30 s the daemon checks `/proc/self/exe`; once the file was replaced
  it asks systemd (`RestartUnit` on the service it runs as) to restart it,
  which stops it with SIGTERM and starts the new binary. Without systemd it
  shuts down gracefully and `exec`s the new binary. It used to `exec` under
  systemd too, but shutting down releases the bus name, systemd stops a
  `Type=dbus` service that loses its name, and its SIGTERM got lost in the
  exec, so 90 s later systemd killed the new daemon and its whole group.
  No `systemctl --user restart` after an update.
- A Katna Mail the daemon starts (tray, notification, search) moves to its
  own `app-in.invenia.katna.Mail-<pid>.scope` (`StartTransientUnit`), so the
  daemon stopping or restarting never closes its windows.

### 9.2.1 What runs today (Phase 1)

- `katna-daemon` opens the store for writing, serves `Pim1`, then takes the
  bus name with `DoNotQueue`; a second daemon exits with "already running".
- One sync worker (`katna_sync::worker`) per IMAP account, each with its own
  store handle. Worker events become the account's status and D-Bus signals.
- Account server settings are JSON in `account.settings_json`
  (`katna_core::AccountSettings`); passwords are in the Secret Service
  under the attributes `application=in.invenia.katna` and `account=<id>`,
  labelled "Katna: <address>". Only the daemon links `oo7`, so the apps stay
  small.
- Adding an account or changing its password logs in once first; a
  refused login is an error to the caller and nothing is saved.
- SIGTERM and SIGINT stop every worker; each ends its IDLE and logs out.
- After the workers start, a `katna-backfill` thread threads and classifies
  mail stored before schema v2 (§6.5) with its own store handle, sends
  `MailChanged` every 2 s while it works, and is stopped on shutdown.
- `katna-daemon install-user-service` writes the systemd user unit and the
  D-Bus activation file for a binary installed by hand. Packages install
  the same files from `packaging/` system-wide (§21.1).
- `katnactl` (task 1.13) drives it: `add-imap`, `status`, `sync`, `watch`,
  `password`, `remove`, and store reads (`folders`, `list`, `show`).

### 9.3 Resource targets

- Idle CPU ≈ 0 % (event-driven; no polling loops except IDLE renewals).
- No GPU use (no GPUI dependency).
- Memory budget and wake-up counts are measured in CI (§17).

## 10. Metadata with expiration (`katna-meta`)

Inspired by Mailspring's plugin metadata. Any object (message, thread,
draft, event) can have a JSON value per feature, with an optional
`expires_at`.

- Stored in the `meta` table; versioned; `dirty` rows are uploaded to
  Katna Server when the user has an account there.
- The **scheduler runs in `katna-daemon`**, so expiries fire even when no
  app window is open. It sleeps until the next `expires_at` and emits
  `Expired(object, plugin)`. The handler must clear or move the expiry so it
  does not fire again.
- Values the server does not need are **end-to-end encrypted** before upload;
  the server only sees object IDs, `expires_at` and what a server-side
  action requires.

Features built on it:

| Feature | Metadata | On expiry |
|---|---|---|
| Undo send | `{send_at: now + N s, undo: true}` on the draft (N = 5/10/20/30 s) | Send the draft. Undo = delete the metadata. The draft stays saved, so a crash does not lose it. |
| Send later | `{send_at}` | Send the draft (the SMTP server holds it when it has `FUTURERELEASE`; else the daemon, or Katna Server if enabled and the machine is off). |
| Snooze | `{until}`, thread moved to a "Snoozed" folder | Move back to inbox, mark unread, notify. |
| Reminder | `{remind_at, if_no_reply: true}` | Notify if nobody replied. |
| Tracking | `{tracking_id, links[], events[]}` | — (events arrive from the server) |

### 10.1 What runs today: snooze and follow-up reminders

Decided September 2026: snooze, reminders and send later run **only while
the computer is on**, and no server ever holds a mail password. So all of
this is local; Katna Server only adds opened/clicked events (§16).

- `katna-meta` types the values and runs the scheduler: it sleeps until
  the next `expires_at`, but never more than a minute, because timers stop
  while the computer sleeps and the wall clock does not; resuming also
  wakes it. Values are in `pim.db`, so they survive restarts; one that fell
  due while the computer was off fires when the daemon starts.
- **Snooze** (`message`/`snooze`: `{until, back_to, snoozed_in}`): the
  messages of the conversation in the folder it was snoozed from (the
  Inbox, a label or Archive, never Sent, Drafts, Trash, Spam or All Mail)
  move to the account's `Snoozed` folder, made on the server the first
  time (a label on Gmail; a local folder for POP3). The folder is always
  called `Snoozed` on the server so any language finds it; the app shows
  its name translated. At `until` the messages that are still there move
  back, unread, with one notification per account. `Unsnooze` (Undo)
  moves them back at once without marking them unread. Gmail, Outlook.com,
  Zoho and Yahoo do not share their own snooze over IMAP, so a snooze set
  on their websites stays there.
- **Follow-up** (`outbox`/`follow-up`: `{account, message_id, subject,
  remind_at, after}`): set on an outbox entry right after `QueueSend`,
  `after` seconds from when it is sent (1, 3, 7 days or custom). When due,
  it finds the sent copy by `Message-ID`; if its conversation has anything
  newer (a reply, or another message of the user's), it is dropped.
  Otherwise the message is copied into the Inbox too (a label on Gmail),
  marked unread and notified. `UndoSend` drops it.
- **Surfaced** (`message`/`surfaced`: `{at}`, expires after 14 days): mail
  back from snooze or a reminder is listed as if it arrived at `at`, so it
  sits on top of the Inbox like new mail.

## 11. Sending (outbox)

- Every send goes through the persistent `outbox` table in the daemon.
- **Undo send** is a delay in the outbox, not a recall. True "unsend" after
  delivery is impossible over SMTP. Katna Confidential (§16) is the
  alternative.
- **Per-recipient sending** (needed for tracking who opened): one tracked
  copy per recipient, each sent in its own SMTP transaction to that
  recipient only; To/Cc headers unchanged. Cap on recipient count.
  After sending, remove tracked copies the server placed in Sent (Gmail:
  also from All Mail) and append the clean copy. Report partial failures
  with the list of who received it.
- Sent-folder cleanup must be robust: find copies by `Message-ID` with
  retries, and never leave a tracked copy where the user will open it.
- Replies created from a notification (§15.1) use the same outbox, with the
  undo delay.

**Implemented (task 1.8, `katna_sync::outbox`, store `outbox.rs`):**

- `QueueSend` stores the message as a `message` row in no folder (so it
  shows in no mailbox and is not journaled) and an `outbox` row due at
  now + the undo delay. `Date` and `Message-ID` are added when missing.
  `UndoSend` works while the entry is `queued`; once `sending` it is too
  late. Cancelled and failed entries stay until `DiscardSend`.
- The daemon runs one outbox task with its own store connection. It sends
  due entries one SMTP connection each (password from the Secret Service
  at every send). The envelope is the first `From` address and every
  `To`, `Cc` and `Bcc` address once; the `Bcc` header is removed from the
  copy on the wire and kept in the sender's copy.
- Network and TLS failures requeue the entry after 30 s without counting
  a try, so mail waits in the outbox while offline. A refused message or
  login counts; after three the entry is `failed`, with the server's reply
  in `Outbox()`'s detail. Entries left `sending` by a crash are queued
  again at start: sending twice beats losing mail.
- Once sent, the copy is filed in the account's Sent folder by an `Append`
  operation in the op queue (flag `\Seen`), which the account's worker
  replays at once; then the local copy is forgotten and the next sync
  brings the server's. Gmail files sent mail itself, so for an IMAP host
  under `gmail.com` or `googlemail.com` the copy is only forgotten. With
  no Sent folder, nothing is filed.
- **Send later.** The mail service's own feature comes first (plan 7.7):
  `ScheduleSend(account, raw, delay, at)` queues the message for the
  undo-send delay (`send_at`) with the chosen time in `hold_until`. After
  the delay the outbox logs in to the SMTP server; one that lists
  `FUTURERELEASE` (RFC 4865; Stalwart does, Gmail does not; some list it
  only after login, so the daemon says EHLO again) gets it at once with
  `MAIL FROM … HOLDUNTIL=<UTC time>` and sends it on time with this
  computer off. The entry is then `sent` with `hold_until` kept, shown as
  `held`, filed in Sent when that time passes, and it can no longer be
  cancelled: SMTP cannot take mail back. A time beyond the server's
  longest hold is handed over once it is within it; a server without
  `FUTURERELEASE`, or a time less than a minute away, keeps the message
  here, sent at `hold_until` while the daemon runs (the old path).
  `ServerHoldLimit(account)` tells the schedule menu which applies: "Your
  mail server will send it…" or "Katna will send it … while this
  computer is on". The app lists queued and held messages (`Outbox`,
  `OutboxChanged`, `send_at` being when it goes out) and counts one as
  scheduled when it is held, or still queued with no error and due later
  than the undo-send delay would put it. Cancel is `UndoSend`, as for
  undo send, for mail not handed over yet.
- **Drafts.** Closing a message saves it (`SaveDraft(account, raw)`): the
  app keeps one `Message-ID` for a message while it is written, and the
  daemon replaces every copy in the Drafts folder with that `Message-ID`,
  stores the new one there (flags `\Draft \Seen`, no UID yet) and queues
  a `SaveDraft` operation. Replayed, it deletes the server's copies with
  that `Message-ID` (read from the folder's headers, since Drafts is
  small), APPENDs the new one and forgets the local copy; the next sync
  brings the server's. A newer save drops the older queued upload.
  `DiscardDraft(account, message_id)` deletes the copies here and queues
  `DropDraft` for the server; the app calls it for Discard and after
  Send. A draft opened from Drafts opens in compose. Accounts without a
  server (POP3) get a local Drafts folder.
- Not yet: per-recipient sending.

## 12. Message rendering (`katna-render`)

This is a major risk: GPUI has no HTML engine.

- **Always:** sanitize with `ammonia`, block remote content by default,
  strip known trackers (blocklist) from incoming mail, per-sender
  "load images" allowlist.
- **Phase 1:** plain text, plus a sanitized HTML subset rendered with GPUI
  text/layout; "Open in browser" fallback for complex newsletters.
- **Phase 2 — Decision needed:** evaluate Blitz (pure-Rust HTML/CSS) vs.
  WebKitGTK via `wry` (GPUI Kit webview) for full fidelity.
- Show authentication results (SPF/DKIM/DMARC), lookalike-domain warnings,
  and "external sender" banners.

**What runs today (decided September 2026).** HTML mail is drawn with
GPUI's own elements, not a browser engine. `katna_render::message_document`
parses the HTML body with `html5ever` (browser-grade error recovery) and
walks it once into a small layout tree (`katna_render::html::Document`):
paragraphs of styled runs (bold, italic, underline, strike, colors,
monospace, links), headings, lists, quotes, `<pre>`, rules, boxes with
background, padding, border (all round, or a top or bottom divider),
radius and width, table rows as rows of cells, button-like inline boxes,
and images. As in a browser, a table cell is never narrower than its
longest word or image (estimated from the font size, or its whole line
under `nowrap`), and the rows of one table with the same number of cells
share their columns' widths, so `width="1%"` columns hold their headings
on one line and line up. Only inline `style` attributes and
presentational attributes are read; `<style>` sheets are ignored. The walk
is the sanitizer: scripts, style sheets, forms, frames, objects, SVG and
unknown elements never reach the tree, hidden preheaders are dropped, link
targets are limited to `http`, `https` and `mailto`, and the tree is capped
in depth and size. `cid:` and `data:` images come from the message.
SVG pictures from mail (carried in it, on the web, or attached) are never
handed to GPUI as they are: GPUI's SVG support reads any local file an
`<image href>` names (`/dev/zero` never ends). `katna_preview::svg` draws
them to bitmaps on a background thread instead, with nothing they link to
loaded, `svgz` refused, pictures inside them size-checked, and the bitmap
at twice the SVG's size but at most 2048 px on a side in the reading pane
(4096 px and 16 MP in the viewer). A message's links open the address in
their `href`, whatever their text says, so while the pointer is on a link
its real address shows at the foot of the reading pane, as in a browser:
the host stands out (an internationalized one as the punycode the network
sees, a name and password before it left out), the rest is quieter
(`rich::link_status`).
In a light theme a message that sets its own colors is drawn on its own
page; one that does not follows the app's colors. In a dark theme the
message's colors are remapped (`window/dark.rs`): white becomes the reading
pane, other light backgrounds become dark ones of the same hue as dark by
eye as they were light, dark backgrounds stay, and text that falls under
3:1 contrast on its new background has its lightness flipped and raised to
4.5:1. Images are not changed. Where the open conversation has such mail,
a half-circle button on the reader's toolbar shows it in its sender's
original colors on its own light page, and back; it holds for that
conversation only. A `text/plain` part
that is really an HTML document is rendered as HTML.

Remote content is blocked by default. Tracking pixels (tiny images and
known open-tracking paths) are dropped. A banner offers "Show images" (this
message) and "Always show from this sender" (kept in
`$XDG_CONFIG_HOME/katna/trusted-senders`). Anyone can write any `From`, so
a trusted sender's images load only when the user's provider vouched for
the address: its `Authentication-Results` (the topmost field, and others
from the same server) show DMARC passing for the `From` domain, or DKIM
passing for a domain aligned with it (`katna_render::sender_authenticated`).
Otherwise the banner says the message may not be from that sender and
offers "Show images" for it. A provider that adds no such field leaves the
topmost one to the sender, which is no worse than trusting `From` alone.
Images are fetched by the daemon (`FetchImage`, `https` only, `http`
upgraded, at most 8 MB, checked to be an image by its bytes), at most 200
different ones per message and 6 at a time; the app never uses the
network. Remote images and
sender pictures go only to port 443 of public addresses
(`katna_sync::net::Reach::Public`): the name is resolved once, loopback,
private, link-local, shared (CGNAT), unique-local, multicast and other
special addresses are refused, and the connection goes to the address that
was checked, on every redirect too, so mail cannot make the daemon reach
this computer or its network. Configuration, OAuth2, Katna Server and
update requests are not limited this way (tests run them on localhost).

Message text can be selected and copied as in a browser (`window/select.rs`):
each run of text a body draws records its layout, so a pointer position maps
to a place in the text; the selection is drawn as a highlight on those runs.
Drag, double- and triple-click, Shift+click, Ctrl+A and Ctrl+C (once the
text was clicked) and a right-click Copy work in plain and HTML mail; the
selection also goes to the primary selection for middle-click paste.

Sender pictures load without asking. Looking one up does reach the
network (a DNS query and HTTPS requests from this computer to the
organization), so it is kept narrow (security audit of 28 September 2026):
the daemon looks up only the sender's organizational domain
(`katna_sync::pictures::organizational_domain`: the last two labels, or
three under a two-letter country domain with a second level such as `co`,
`com`, `org`, `net`, `ac`, `gov` or `edu`; a heuristic, as the Public
Suffix List is not in the tree), never the sender's own subdomain, which
could be unique to one recipient, and keys the week-long cache on it. It
follows a BIMI `l=` URL or an icon a home page names only when its host is
that domain or under it. And `SenderPicture` returns nothing unless the
user's provider authenticated mail from the address's domain: when a
message arrives, the daemon reads its topmost `Authentication-Results`
field (`katna_sync::auth_results`) and keeps `{"dmarc", "aligned"}` in
`message.auth_results_json`, `aligned` meaning DMARC passed for the `From`
domain or DKIM passed for a domain of the same organization; a picture is
looked up only when some message `From` that domain is `aligned`. Mail
stored before this has no verdict, so its senders get a picture once new
mail from them arrives. The lookup itself is the organization's BIMI logo
(`default._bimi` TXT record, SVG), falling back to the largest icon its
home page names (`<link rel="icon">`, `apple-touch-icon`), then
`apple-touch-icon.png` and `favicon.ico`. Free-mail domains get none, and
answers are cached in `$XDG_CACHE_HOME/katna/pictures` for a week. The
General setting "Sender pictures" (`mail.sender_pictures`) turns them off;
then only trusted senders get one. The same pictures show in the reading
pane, the phone list and Contacts. Each picture is made to fill its circle
(`katna_preview::avatar`): a transparent or single-color margin is trimmed.
An icon that is solid edge to edge is then cropped to the circle. A mark on
a tile of one color sits on a disc of that color, and a see-through glyph
sits on a white disc, or a dark one when the glyph is light.

The user's own accounts show the picture picked in Settings → Accounts
(kept in `$XDG_DATA_HOME/katna/account-pictures/<account id>`), else each
its own coloured letter, so accounts tell apart. "Use desktop picture"
copies the desktop user's picture (`~/.face.icon`, the AccountsService
icon, or `~/.face`) in as the account's picture; it is not the default,
because it made every account look the same. An account that signed in
with Google shows its Google picture (the daemon saves it under
`account-pictures/provider/<account id>`) when none was picked
(`own_picture`, `window/remote.rs`); Microsoft's needs Microsoft Graph,
which Katna does not ask for. Libravatar or Gravatar could come
later as an opt-in. Settings → Accounts also renames an account and sets
the order accounts are listed in everywhere (Move up, Move down, or a
drag by the handle; `mail.account_order` in `config.toml`), the first
being the default.

Size: this renderer added 2.7 MB to the release app (31.3 → 34.0 MB). For
comparison, a minimal program with Blitz (`blitz-html` + `blitz-paint` +
`anyrender_vello_cpu`) is 12.9 MB, and it paints to a bitmap, so text,
links and selection would need their own plumbing. WebKitGTK cannot be
embedded in a GPUI window. Blitz stays the candidate if newsletters that
depend on `<style>` sheets turn out to matter.

### Composer

- A WYSIWYG rich-text editor in GPUI (`katna-ui::rich`): paragraphs with
  bold, italic, underline, strikethrough, font, size, text and background
  color, links, alignment, lists, indent and quote levels, inline images
  and tables. The document model is GPUI-free (`rich/doc.rs`); it reads
  and writes a small HTML subset (`rich/html.rs`), which is what mail
  sends and signatures store.
- A message goes out as `multipart/alternative` (text and HTML) with
  inline pictures as `multipart/related` parts (`cid:`), plus attachments.
  In plain text mode only the text part goes.
- Spell check with `spellbook` and the system's Hunspell dictionaries
  (`spell.rs`); added words are kept in `$XDG_CONFIG_HOME/katna/dictionary`.
- Templates (`window/compose/templates.rs`, `window/settings_page/templates.rs`):
  the compose bar's Templates button lists them, puts one in (its text
  replaces the empty lines above the signature, or goes at the cursor; its
  subject fills an empty one; its files join the attachments) and saves
  the message as one (same name replaces). `{first name}`, `{name}` and
  `{my name}` are filled from the first recipient and the sender when it
  is put in, and again on Send for a recipient added later
  (`templates.rs`). Settings > Compose edits, renames and deletes them.
  They live in `pim.db` (subject, HTML with pictures as `data:` URIs,
  plain text, attachments as BLOBs, 20 MB at most), written by the daemon
  (`SaveTemplate`, `DeleteTemplate`) and read by the app from the store.

## 13. UI

### 13.1 Window decorations (`katna-chrome`)

| Desktop | Mode | Details |
|---|---|---|
| KDE (KWin) | **Server-side** (xdg-decoration) | KWin draws the real Breeze (or user-chosen) title bar, buttons and shadow. Toolbar sits below the title bar, like other KDE apps. |
| GNOME (Mutter) | **Client-side** (must be requested explicitly, or the window has no title bar) | Adwaita-style header bar; button layout from `org.gnome.desktop.wm.preferences button-layout` (default: close only); own shadow in a transparent margin with correct window geometry; larger shadow when focused; rounded corners only when floating (none when maximized/tiled); resize edges in the shadow area; double-click maximize, right-click window menu. |
| Others (Sway, Hyprland, …) | SSD if offered, else minimal CSD; none when tiled | |
| X11 | SSD by default | CSD shadows need a compositor. |

Detection via `XDG_CURRENT_DESKTOP`. The frame is drawn from the negotiated
mode, not the requested one: GPUI falls back to CSD when the compositor has
no xdg-decoration. Validated by spike S1 (`docs/spikes/S1-window-chrome.md`).

**Settings > Experimental > Look & Feel** (config `[experimental]`, both
off by default, applied live to every window through the `katna_chrome::Look`
GPUI global):

- *Window frame*: `native` (the table above) or `katna`, which asks for CSD
  everywhere: on KDE Wayland through xdg-decoration, on X11 with no WM frame
  (`_MOTIF_WM_HINTS`) and `_GTK_FRAME_EXTENTS` for the shadow margin. The
  frame keeps the desktop's preset (Breeze-like buttons and 5 px corners on
  KDE). On KDE the buttons take Breeze's size: 24 px with a 10 pt font,
  following the `kdeglobals` font and Breeze's "Button size" (`breezerc`
  `[Windeco] ButtonSize`; `katna_chrome::breeze`). They sit centred in the
  header bar, as far from the window's side as from its top. Switching keeps the window's size on screen. Where the desktop never
  draws frames (GNOME on Wayland) the choice is replaced by a note.
  `KATNA_DECORATIONS=auto|server|client` still overrides it, for testing.
  On Windows GPUI fixes the title bar when a window opens, so the choice
  applies to windows opened after it (Settings says so): `katna` hides
  Windows' title bar (`TitlebarOptions::appears_transparent`) and the top
  bar's empty space and window buttons become Windows' caption and buttons
  (`WindowControlArea`), so Windows moves, snaps and maximizes the window;
  Windows keeps drawing the corners, shadow and resize edges.
- *Blurred background*: the window's page color becomes translucent
  (`katna_chrome::tokens::blur_alpha`: 75 % light, 80 % dark; the idle
  search box is 40 % glass over it with a faint edge and turns solid while
  focused) and the
  compositor blurs what is behind it: `ext_background_effect_v1` (KWin 6.7),
  else `org_kde_kwin_blur`, and `_KDE_NET_WM_BLUR_BEHIND_REGION` on X11.
  The blur region is the frame less its rounded corners; the CSD shadow is
  painted only outside the frame, so it cannot darken the window. Cards
  and dialogs stay opaque, so text keeps its contrast. Offered only where
  the compositor can blur (`gpui_linux::compositor_blur`; always on
  Windows, through GPUI's `WindowBackgroundAppearance::Blurred`, the
  acrylic blur behind the window); elsewhere the
  switch is shown off with the reason. The compose pop-out stays opaque
  (it is all message).
- A second switch, *Frosted menus and dialogs*
  (`experimental.frosted_popups`, on by default and independent of the
  window blur, which needs no compositor since Katna draws it), frosts
  floating panels in every window: menus (the
  right-click menu and its folder list, dropdowns), Search options and its
  date popover, and the account menu. Their color is 78 % opaque over a
  20 px blur of what is behind them in the window
  (`katna_mail::widgets::raised`, `katna_ui::frost`). Dialogs and
  floating cards (Add account, About, What's new, confirmations, label and
  share dialogs, the first-run card) frost the same way with their own
  color (`katna_mail::widgets::frosted`); a test keeps every frosted
  panel's glass ahead of its content. The compose window, notes and the
  task details (a scrolling card) stay opaque. GPUI has no backdrop
  filter, so Katna's copy of its renderer (`vendor/gpui-pre-wgpu`) adds
  one: a quad marked through its border color is drawn over a dual Kawase
  blur of the frame under it, clamped to the quad (as CSS
  `backdrop-filter`). The same renderer draws every drop shadow only
  outside its element, as CSS does, so a translucent panel or frame keeps
  one plain box shadow that follows its rounded corners. Where the
  window's surface cannot be copied from, panels stay opaque.

**Settings > Experimental > Reading** (config `[experimental] chat_view`,
off by default): conversations between people show as a group chat
(`window/reader/chat.rs`). Each mail is a bubble with only what its sender
wrote, the user's own on the right, grouped when one person writes again
within ten minutes; days, and people a mail brings in, show between them.
`katna_render::trim` splits a body into what was said, the quoted mail, the
signature and a forwarded mail (`trim::plain`, and `html::trimmed` for HTML
bodies, cutting at Gmail, Outlook, Apple Mail and Thunderbird quote markers
and the usual attribution, forward and `-- ` lines). Signatures without a
`-- ` line are read from what people write: a sign-off ("Best regards,")
over a short name block, a rule (`_____`, `-----`) over contact details, a
block of contact details of two kinds (phone, address, email, web), and
footers that offer to unsubscribe, say why the mail came or carry a
confidentiality notice; lines a person ends two of their mails in the
conversation with are their signature too (`trim::shared_tail`). HTML
mail reaches the chat as its text and is read the same way. The quote and
signature wait behind a ··· pill, a forward is a small card. Attachments are
chat media: pictures in a grid of their thumbnails, other files as cards,
both opening the viewer; inline pictures under 12 KB (logos) are left out.
A conversation opens as a chat unless a message from someone else is bulk
mail (`MessageView::bulk`: `Precedence: bulk|junk`, or `List-Unsubscribe`
outside a mailing list); the header's Chat | Mail switch (also above the
mail view) overrides that for the open conversation. The reply box at the
foot is the inline reply as Reply all (`compose/chat_box.rs`): Ctrl+Enter
sends, Aa opens the full formatting bar above it, the paperclip offers
pictures, files, a template or another signature, and the signature is held
out of the text and added on Send. Sending never archives the chat, and the
undo-send countdown shows as a ring with Undo beside the new bubble instead
of the snackbar. Hover shows Reply all and ⋯; right-click offers Reply to
all, Reply to the sender only, Forward, Copy text and Show as mail, and
answering an older bubble aims the reply at it (quote and threading) with a
"Replying to" strip, keeping what was written. A name or picture opens the
contact panel on that person, with the signature they last used in the
conversation. The paperclip's From Files opens a picker over the feed with
the Files page's filters, this conversation's files first; ticked files
weigh against the 25 MB a mail carries, and those past it go by Google
Drive or OneDrive when the account has one. Up to five things can be pinned
to the top of a chat: a mail (hover Pin, or the right-click menu) or one of
its files (right-click on its card). Pins live in the mail store's
`chat_pin` table (`katna_store::chat_pins`, mail.db v12), written by the
daemon (`Pim1.PinInChat`, `UnpinInChat`, `OrderChatPins`) and read through
the conversation's messages, so they survive the thread being rebuilt;
nothing goes to the mail service, which has no such thing. A bar under the
header shows one pin; a click jumps to its bubble and moves to the next,
and its list button lists all of them, to drag into a new order or unpin.
A sixth pin asks which one it replaces, the oldest picked. Pinning text
waits for bubbles' text to be selectable; company details are a later step.

**Window state.** The mail window opens as it closed: its size, maximized
state and place (`katna_chrome::placement`), and what it showed: the app of
the rail, the folder or unified list, the inbox tab, the folders opened in
the folder pane and whether the pane was folded (`katna_core::window::
ViewState`). Both are saved in `$XDG_STATE_HOME/katna/mail-window.toml`
when the app quits, however it quits. Settings such as the reading pane,
its width and the density live in the config file as before. The state
belongs to one run of the Katna service, named by the daemon's process id
and start time (which survive its re-exec after an update); once the
service quits (the tray's Quit, logging out), the next start opens the
window as on the first start.

- Wayland does not let a window place itself. Katna's copy of GPUI
  (`vendor/gpui-pre-linux`) joins the window to an
  `xdg-session-management-v1` session (KWin from Plasma 6.7), and the
  compositor puts it back where it was; a fresh start removes the old
  session and begins a new one. Without the protocol only the size and
  maximized state come back.
- X11: the window opens exactly at its old position (user-specified
  position, static gravity). KWin adds the CSD shadow margin itself on X11,
  so the saved frame is asked for as is.
- Windows: GPUI's bounds are the inside of the window and Windows adds
  its title bar and borders around them, so sizes are fitted to the main
  display's work area less 16 × 48 px. A first window that does not fit
  (1280 × 800 on a small screen) opens maximized; a saved size that does
  not fit shrinks, centred.

### 13.2 Look and feel

- One Katna design language on a **token layer** (radius, spacing, button
  style, colors, fonts) with two presets: **Breeze-like** and **Adwaita-like**.
- System integration: color scheme and accent color via the portal
  Settings interface (`ashpd`, live updates); full KDE palette from
  `kdeglobals`; system UI font; freedesktop icon theme; portal file
  chooser.
- Keyboard-first: Gmail-compatible shortcuts, command palette.
- Accessibility via AccessKit (AT-SPI) in current GPUI.

### 13.3 Main mail layout

Sidebar (accounts, unified inbox, folders, **organizations**, smart views) →
thread list (organization color badges, availability icon) → reading pane
(conversation view). Organization page: people, timeline, attachments,
awaiting-reply threads, engagement stats.

### 13.4 Quick-reply window

A small GPUI window used where the notification server has no inline reply
(GNOME) and from KRunner's "Reply all" action. Reply-all prefilled, send
with undo delay.

### 13.5 First Katna Mail window (Phase 3 start)

The first slice of tasks 3.1, 3.3, 3.4, 3.5 and 3.6 (`apps/katna-mail`):
sidebar, message list and reading pane over the local store, and a search
box. What it does and what we decided:

- **Read-only.** The app opens `mail.db`, `pim.db`, `blobs.db` and the
  search index read-only (§5.1). It sends no commands yet; F5 re-reads the
  store until the daemon's change signals are wired in. `--data-dir DIR`
  uses the same layout as `katna-search-cli`.
- **Sidebar.** Accounts, then folders nested by path (`/` separator; the
  store does not record each account's delimiter yet). Special folders
  (from `folder.role`, else from the name) come first. Accounts with more
  than 40 folders start collapsed, opened at their first inbox with mail.
- **Message list.** A virtualized list of the folder's message IDs, newest
  first; rows are read in visible ranges and cached. Sent and draft folders
  show recipients. Threads (conversation view) wait for task 1.7.
  GPUI's `List` holds a page of 1,200 lines around the ones on show, not
  all of them (`window/lines.rs`): it keeps state per line and builds it
  again whole on each reset and change of width. The page moves before a
  frame when the top line on show comes within 300 lines of its edge,
  keeping the scroll position to the pixel.
- **Counts.** Folder totals come from the location index at startup
  (5 ms for 100,000 messages). Unread counts need each message's flags
  (about 100 ms per 100,000 unread messages), so they are counted on a
  background thread and appear when ready. The daemon should keep counts
  per folder later.
- **Start.** Opening the window and the GPU takes longer than reading the
  first list, so `main` reads the list shown on the last start (kept in
  the cache directory) and the folder totals on a thread of its own while
  GPUI starts (`apps/katna-mail/src/data/preload.rs`). The window takes
  them unless the mail journal moved on meanwhile, it asks for another
  list, or the thread failed or took over a second; then it reads as
  before.
- **Reading pane.** `katna-render` gives a plain-text view: headers,
  text parts (HTML converted to text, with a note), quoted lines dimmed,
  and the attachment list. At most 256 KB and 4,000 lines are shown.
- **Search.** As-you-type (`Query::parse_as_you_type`), 60 ms after the
  last keystroke, on a background thread; up to 1,000 results with a total
  count. `/` or Ctrl+F focuses the box, Escape clears it.
- **No GPUI Kit yet.** It would not fit the size budget next to GPUI
  (§17.1), so `katna-ui` has its own small components (a text input with
  IME support so far).
- **UI font.** GPUI's system font on Linux is IBM Plex Sans, which few
  systems have; the fallback face has no bold. `katna-platform` reads the
  desktop's font (`kdeglobals` on KDE, the GSettings `font-name` elsewhere)
  and checks it is installed, falling back to common fonts per desktop.
- **Colors.** `katna-chrome` tokens gained `view_bg` and `sidebar_bg`
  (libadwaita and Breeze values).
- **Icons.** A few symbolic SVG icons are built into the binary until the
  freedesktop icon theme is read (task 3.3).

### 13.6 Webmail layout and motion

After the first window, the owner asked for a layout as close to Gmail as
possible, with the search box in the middle and smooth animation. The
window keeps the desktop's frame (§13.1) and changes what is inside it:

- **Top bar.** The header bar (CSD) or toolbar (SSD) is 64 px tall in the
  page color, with no border: menu button and app name on the left, the
  search box (at most 720 px wide), the account avatar on the right. As
  in Gmail, on a desktop the search box starts where the mail list does
  with the folders open, and stays there when they fold (it does not
  follow the list). It moves left only when the window is too narrow for
  that place, and then always sits one gap after the app's name, whose
  width is set for this: the Katna mark, "Katna" and the longest app name,
  measured in the desktop's font (a narrow tablet, under 760 px, folds the
  words away and keeps the mark). The top bar uses that one
  16 px gap between all its items: menu button, the app's name, search
  box, Settings and account picture. `katna_chrome::Bar` gives the bar a center slot,
  height and background for this.
- **Navigation.** The folders as full pills, rounded at both ends and
  set 8 px in from the pane's edge (the drawers' lines too). The menu
  button folds it away (it first folded to a rail of icons; see below).
  With one account the account heading is left out. With several, the
  owner asked for one account at a time by default, as Gmail does:
  "Folder pane" in Settings > Accounts (`mail.accounts_shown`, `one` or
  `all`) picks between the shown account's folders only and every account
  one after another. The account card switches the shown account (it marks
  it and gives each account's unread count, under an icon row of
  Settings (the General page), the language button and the ☰ application
  menu, with "Add another account" as the last row); the choice is kept in
  `mail.current_account`. The list, search results, Go to, compose's From
  and the top-bar picture follow the shown account, and opening a message
  of another account (from a notification) switches to it. The taskbar
  badge, tray and notifications still count every account, so no new mail
  goes unseen. Each account heading has an arrow that folds its folders.
  The unified inbox (Settings > Accounts, `mail.unified_inbox`, off by
  default) puts an "All Accounts" section over the accounts, which then
  start folded: Inbox, Unread, Starred, Important, Sent, All mail, Spam,
  Trash and Drafts across every account (`window/unified.rs`). The special
  folders list each account's folder of that role together; Unread,
  Starred and Important list mail with that flag in every folder but trash
  and spam. Each opens to one line per account. The lists are read like
  search results (no one listed folder), merged by date in
  `Store::spread_threads` and `spread_message_ids`, which show server
  copies of one message once.
- **One card.** The list and the open message share a white card with
  rounded corners on a tinted page. The list is one line per message:
  star, sender, subject in bold if unread with the snippet after it, and
  the date. Read rows are tinted. Clicking a row (or Enter/`o`) opens the
  message in the card; `u`/Escape goes back, `j`/`k` move to the next or
  previous message in both views, as in Gmail. The toolbar shows the
  visible range ("1–19 of 72") and paging arrows.
- **Menus and popovers close alike.** The account menu, the toolbar and
  right-click menus, the "+N" attachments list, Search options and its date
  picker all close on Escape (the top one first, before any shortcut, so
  Escape never also goes back to the list) and on a press anywhere outside
  them, the top bar included: each sits over a scrim that covers the whole
  window (`window/popovers.rs`). Escape also closes quick settings; a click
  beside them does not, since they sit beside the list rather than over it.
- **Line extras (Gmail's).** Next to the star, an importance marker
  (`+`/`=` mark important, `-` not important; also in the More and
  right-click menus). Importance is the `$Important` keyword (RFC 8457,
  `MessageFlags::IMPORTANT`); on Gmail it is the Important label, read
  with `X-GM-LABELS` along with flags and set with `STORE X-GM-LABELS`,
  and `is:important` searches it. Lines with named attachments grow a
  second row of chips (file-type badge and name, the full name as a
  tooltip; as many as fit, at most three) and a round "+N" that lists
  the rest; a chip opens the built-in viewer (§13.8) on that file, with
  the message's other attachments a click of the arrows away. Lines differ
  in height, so the list is GPUI's `list` (measured lines) rather than
  `uniform_list`. Hovering a line shows Archive, Delete, Mark as
  read/unread and Pin in place of the date. Star, importance and pin
  changes show a snackbar with Undo.
- **Undo.** Every change to mail (archive, delete to Trash, move, spam,
  read/unread, star, importance, pin, send while its undo delay runs)
  shows a snackbar with Undo, and each window keeps its last 50 as a
  history that Ctrl+Z (and the set's own key, like Z) walks back through
  after the snackbar is gone. Moves out of search results or a
  conversation window go back per message to the folder each left.
  Deletes for good and mail already sent cannot be undone; Ctrl+Z says so.
  In text fields Ctrl+Z is about the text.
- **Pins.** Pin to top (hover button, More and right-click menus) keeps a
  conversation, or a single message in message view, above the rest of
  every folder it is listed in, newest pin first, with a pin next to the
  date; search results keep their order. Pinning takes the whole
  conversation, so its later replies stay pinned too. At most ten
  conversations are pinned (`katna_sync::ops::MAX_PINS`): the daemon's
  `SetPinned` refuses an eleventh with "You can pin up to 10
  conversations. Unpin one to pin another.", which the snackbar shows.
  Pins are local (schema v5, §5.3).
- **Reading view.** Subject with the folder as a chip, a letter avatar
  (color from the address), sender, recipients, date with "(2 hours ago)",
  the body, attachments as cards, and Reply/Forward buttons. Opening or
  folding a message of a conversation animates its height from the old
  one; the sender picture stays in place and only the text fades.
- **Conversation windows.** Shift+click on a line, "Open in new window"
  on its right-click menu, or the "In new window" button on the open
  conversation's toolbar opens it in a window of its own, as in Gmail. A
  plain click or double-click always opens it in place (a double-click
  opening a window felt like a glitch: its first click had already opened
  the conversation). The window is a second `MailWindow` in a detached
  mode that shows only the reading view, with no close button of its own
  (the window frame has one): it reads the store and follows
  `MailChanged` itself. Archiving, deleting or moving the conversation
  closes it, and the main window shows the snackbar with Undo.
- **Printing.** "Print all" on the open conversation's toolbar (and its
  More menu) prints every message. Katna's own print preview opens first
  (`window/print_preview.rs`): `katna_render::print` lays the conversation
  out as a PDF (krilla, text shaped and measured with rustybuzz, in the
  desktop's UI font found with fontdb) on A4, or Letter where the locale
  uses it (`LC_PAPER`), and hayro (`katna_preview::pdf`) draws the pages;
  an A4 or Letter switch lays them out again. Print hands off to the
  desktop's print dialog (XDG print portal), which starts on the previewed
  paper; if a different paper is picked there, the pages are laid out
  again on it. Each message prints with sender, date, recipients and
  attachment names, and an HTML body as the reader draws it
  (`print/flow.rs` follows `window/rich.rs`): boxes, table rows as cells
  (stretched to the row's height), colors, borders, lists, quotes, links,
  bold, italic and monospace faces, and pictures (the message's own, and
  remote ones the reader has shown), at 10.5 pt for the mail's 16 px.
  Everything is placed on one strip and cut into pages between lines,
  never through a line, a picture or a short table row. The preview's
  Layout switch picks As shown (the default) or Simple text (a line per
  paragraph, a row's cells on one line); its Backgrounds switch leaves out
  page, box and text backgrounds (light text is darkened) while the
  formatting stays. There is no font fallback for scripts the UI font
  lacks. Without a print
  portal the PDF opens in the default app. PDFs are written to
  `$XDG_RUNTIME_DIR/katna/print` and removed after an hour. Print and In
  new window sit right of the actions and move to the More menu when the
  reading pane is under 600 px. The More menus open right under their
  button, with an icon beside each item.
- **Reading options.** Settings > General > Reading, taken from
  Mailspring: *Newest message first* shows a conversation's latest reply on
  top, with a reply written above it (`mail.newest_first`); *Show full
  headers* opens the from, to, cc, date and subject box on every message,
  and clicking "to" turns it the other way (`mail.full_headers`); the "to"
  line names recipients by first name ("to me, Ada", as Gmail does) unless
  *Full names of recipients* is on or two share a first name
  (`mail.full_names`). All three are off by default.
- **Auto-advance.** Deleting, archiving, moving or reporting the open
  conversation opens the next one in its place, in the same frame, so the
  reading pane never closes and reopens (the owner, 2026-09-27). Settings >
  General > Auto-advance (`mail.auto_advance`, as Gmail's) picks the line
  below (the default; the one above when it was the last), the line above
  (the one below when it was the first), or the list. Undo, on the
  snackbar or with Ctrl+Z, brings the conversation back and opens it again
  once it is back in the list.
- **Motion.** Springs (`katna_ui::motion::Spring`, on GPUI's spring
  solver) drive values that shape several elements: the navigation width,
  the search box turning white with a shadow when focused, the snackbar.
  Per-element motion uses GPUI's `with_spring` (row lift on hover, the
  list cursor bar growing from the middle, the selected folder's pill
  fading in) and `with_animation` (the card fading between list and
  message, the message sliding up as it opens). `katna_ui::Ripple` draws
  the Material ink ripple from the pointer on buttons, folders and rows.
  Everything honors the desktop's reduce-motion setting.
- **Colors.** `theme.rs` has Katna's light and dark palettes. The owner
  later asked for the desktop's colors as well; see "Desktop colors" below.
- **Depth in dark colors.** Shadows barely show on a dark background, so in
  dark colors (Katna's or the desktop's) things that float are lighter the
  higher they sit: floating buttons, dialogs, popovers and the Compose
  window use `Theme::raised` (the card lifted by 10% of the text color),
  menus `Theme::menu` (13%), and `widgets::elevation` adds a faint 1 px
  light edge (`Theme::rim`) to every shadow. A dialog draws its contents
  with `Theme::lifted`, so fields and chips inside it keep their contrast.
  Light colors keep the white card and its shadow (the owner, 2026-09-30).

The owner then asked for the rest of Gmail's pattern, with Katna's own
icons and name and without Google-only features (no Chat, Meet, Drive,
Gemini or confidential mode):

- **App rail.** A 72 px column at the far left holds Mail, Calendar,
  Contacts, Tasks, Notes and Feeds (RSS and Atom), with Settings at the
  bottom. Their names show under the icons unless "App names" is off in
  quick settings (`mail.app_labels`); then the icons have tooltips. Each
  app is a page (`window/apps.rs`), so new ones
  plug in. Mail is the only app so far; Contacts lists the people the mail
  was exchanged with, most written with first, and a click searches their
  mail; the others show a "coming soon" page saying what they will do.
- **Top bar.** Settings gear on the right; the search box has a search
  options button at its right end that opens a panel (from, to, subject,
  has the words, doesn't have, date within, has attachment) which builds
  the query.
- **Panes.** Quick settings choose the reading pane: *right of the list*
  (three panes, the default) or *no split* (two panes). With three panes
  the list takes the whole card until a message is opened; the message then
  slides in on the right, and the divider between them can be dragged
  (the share is saved). With two panes the message replaces the list.
  With three panes the keys follow the pane that has them: the list keeps
  them when a line is clicked, so Up and Down move in it and show each
  conversation beside it; Tab or Enter gives them to the conversation,
  where Up and Down scroll, and Esc (or U, Backspace) and Shift+Tab give
  them back. Tab and Shift+Tab (or F6 and Shift+F6, which also leave a
  field) go round the folder pane, the list, the open conversation and
  the search box. The list's cursor turns grey and the conversation's
  edge takes a faint accent while the conversation has the keys. Esc in
  the list closes the conversation beside it.
- **Conversations.** The list shows one line per conversation by default
  (senders, a count, the newest subject and snippet); a setting shows
  single messages instead.
- **Category tabs.** The inbox has tabs with "N new" badges, the set its
  provider's webmail uses (`tabs.rs`): Primary, Promotions, Social, Updates
  and Forums for Gmail; Focused and Other for Outlook; Inbox, Newsletters
  and Notifications for Zoho Mail; Gmail's five for everyone else. The
  provider is told from the address and the IMAP host. Every tab is a set
  of the five stored categories. Gmail accounts use Gmail's own
  categories; other accounts use header rules: mailing lists go to Forums,
  newsletters and marketing to Promotions, automated notices to Updates,
  social networks to Social, and people to Primary. Settings, Inbox picks
  another set per account or none, and turns single tabs off (their mail
  shows in the first tab); a switch turns tabs off for every account
  (`[mail] inbox_tabs`, `[mail.account_tabs."address"]`).
- **List toolbar.** A select-all checkbox with a menu (all, none, read,
  unread, starred, unstarred), refresh and more; with lines ticked it shows
  archive, report spam, delete, mark read or unread, move to and more.
  Hovering a row shows archive, delete and mark read. Changes are shown at
  once and sent to the daemon; the snackbar offers Undo.
- **Open conversation.** A toolbar with back (or close with three panes),
  archive, spam, delete, mark unread, move to, more, and "3 of 72" with
  previous and next; the subject with folder chips; each message with an
  avatar, sender, "to ..." with a details drop-down, date with "(ago)",
  star and reply; earlier messages folded to one line, and a run of three
  or more folded into a count; Reply, Reply all and Forward buttons below.
- **Quick settings.** A panel that slides in from the right and pushes the
  cards: "See all settings", reading pane (with small drawings of the two
  layouts), density, theme (desktop, light or dark, the window frame
  included), inbox tabs, undo-send delay, signatures and conversation view.
  Changes apply at once and are saved to `config.toml` (`[mail]`,
  `[sending]` and `[shortcuts]`).
- **Scaling.** Settings > Appearance > Scaling makes the whole interface
  75% to 200% of its size, on top of the desktop's scale, and applies at
  once. GPUI takes the display's scale from the desktop and cannot add to
  it, so Katna scales its own lengths: every length goes through
  `katna_ui::px`, which multiplies by the scale, and every length read
  back from GPUI (layout bounds, the window's size, the pointer) through
  `katna_ui::unpx`, which divides by it (`crates/katna-ui/src/scale.rs`).
  The layouts follow the scaled width, as a web page's do when zoomed:
  at 200% a 1400 px window lays out as a 700 px one. The slider previews
  while dragged and applies when let go, so it doesn't grow under the
  pointer. Mail you send keeps its own font size.
- **Settings page.** "See all settings", the rail's gear or `?` open it in
  place of the list (`window/settings_page.rs`). Its tabs, in the owner's
  order: General (language, 12- or 24-hour time, conversation view,
  reading order and headers, when mail
  is marked read, what the reply button does, images from the web, undo
  send, offline mail, starting at login, tray and badge), Notifications
  (new-mail notifications, Sounds, taskbar count, which folders notify
  and count, muted list), Inbox, Accounts, Subscription, Appearance (reading pane,
  density, scaling, theme, desktop colors, app names, sender pictures,
  Important markers, message width, dark colors for HTML mail, attachment
  previews), Shortcuts, Default apps (where each kind of attachment
  opens, and showing saved files in their folder), Folders & rules,
  Compose (signatures, plain text, spelling and its language,
  templates), MCP server, User feedback (turning crash reports and feedback off at any
  time) and Experimental, always last. Subscription, Folders & rules and
  MCP server are still to come: their tabs are fainter and each shows a
  "Coming soon" page saying what it will do. The tabs always stay on one line (`window/tab_strip.rs`): when
  they don't fit, the row scrolls sideways by wheel or touchpad, arrows
  show at an edge with more tabs past it (not on a phone, where the row is
  swiped), and the arrows and picking a half-hidden tab glide the row. A
  setting's line that would take more than one line under its name (over
  about 40 characters) sits behind an (i) button beside the name: its
  tooltip on hover, and shown under the name after a click, Enter or a tap.
  The General, Appearance and Compose rows added after comparing with
  Mailspring's settings each change real behaviour: "Start Katna at
  login" is a desktop entry in `$XDG_CONFIG_HOME/autostart` (the file is
  the setting, so the desktop's own autostart settings agree with it),
  written once by default on the first run (`general.start_at_login_set`
  keeps an explicit off off) and starting only the service unless "Open
  the Katna Mail window too" is on;
  marking read after 1 or 3 seconds only happens if the conversation is
  still open then; with "Always show images" off, each message's images
  still wait to be asked for. Sounds (`[sounds]`, `window/sounds.rs`)
  has a line per event: new mail, event and task reminders, mail back in
  the inbox (snooze, no reply), mail sent and mail not sent, each with a
  sound to pick (a menu that plays each as it is picked), a play button
  and a switch. The sounds are the desktop's own (`katna_platform::sound`):
  on Linux freedesktop names found in the KDE sound theme, Ocean or
  freedesktop (with fallbacks, e.g. New email falls back to
  `message-new-instant`), played with `pw-play`, `paplay` or
  `canberra-gtk-play`; on Windows the toast sounds (Mail, Reminder, …),
  whose `.wav` in `%SystemRoot%\Media` a `MediaPlayer` plays outside a
  toast. Katna plays a notification's sound itself on Linux, because
  servers such as Plasma's leave `sound-name` unplayed, and sends
  `suppress-sound`; it stays silent while the server's `Inhibited` (Do not
  disturb) is true. On Windows the toast plays it, so Focus Assist
  silences it. Muted folders, conversations and senders never notify, so
  they make no sound. Older `notifications.sound` and `sending.sent_sound`
  switches carry over when off. Open and click
  tracking is not a setting: it, a read receipt and a delivery receipt
  are on for every new message and reply and turned off per message in compose (§16.1), so
  Mailspring's tracking defaults have no counterpart.
- **Searching settings.** While the Settings page is open the top bar's
  search box searches settings ("Search settings"; `window/settings_search.rs`):
  matching rows from every tab replace the open tab, each with its tab and
  a line on it (row names, what they do, other words people use, and every
  shortcut's name). A result, or Enter for the first, opens its tab,
  scrolls the row into view and lights it up for a moment. Closing the page
  puts mail search back, with its words if the list still shows its
  results.
- **Tab between controls.** Tab and Shift+Tab move the focus in the order
  things are drawn, as in any desktop form: fields (`TextInput`,
  `RichEditor`) are always Tab stops, and the Settings page's tabs, rows,
  chips and buttons are too (`widgets::FocusRing`). Enter or Space presses
  the focused control, a tint with a ring shows it (only after a key, not
  a click), and the page scrolls to keep it in view. The page's open tab
  takes the focus when it opens. In the rich editor Tab still moves
  between table cells and indents list items. Controls in the quick
  settings panel and toolbars stay out of the Tab order, since focusing
  them on a click would take the keys away from the list or the editor.
- **Menus, dialogs and New Message by keyboard.** The usual desktop keys,
  nothing new to learn. Up, Down, Home and End move through any open
  menu's items, Enter or Space picks one (`popovers::MenuKey`: items sit
  last in the Tab order, and a menu's frame has the `Menu` key context).
  Shift+F10 and the Menu key open the selected line's right-click menu. A
  dialog takes the keys when it opens: Enter presses its main button (the
  delete question deletes), Esc cancels (the account dialogs too), and Tab
  goes round its own fields and buttons only (`keep_tab_inside`); filled
  buttons show the ring round them. In a message being written Ctrl+Enter
  sends from any field, and Esc closes it and keeps a draft, as in Gmail,
  Outlook and Thunderbird (a reply in the conversation stays). In the
  folder pane Up and Down open each folder, Right and Left unfold and
  fold, and Enter goes to its mail, as in Thunderbird and Outlook. Whenever
  the keys lose their place (a message sent, a menu or dialog gone) they
  come back to the list, or to the Settings page while it is open.
- **Removing an account, deleting all data.** Settings → Accounts
  (`window/accounts.rs`) lists
  the accounts, each with Remove, and has "Delete all Katna data". Both
  only touch this computer: they ask first in a dialog that lists in red
  what is deleted, says the mail stays on the server (or, for imported
  mail, that Katna has the only copy), and deleting everything also needs
  "delete" typed. The daemon does the work (`RemoveAccount`,
  `DeleteAllData`); after deleting everything the app starts over with
  the default settings.
- **Reset cache.** Settings → General has "Reset cache", as in Mailspring:
  it deletes what Katna downloaded and can download again (bodies and
  attachments of mail still on an IMAP server, sender pictures, the
  search index) and syncs, so the offline window downloads again and older
  mail downloads when opened. A dialog says what goes and what stays:
  accounts, settings, flags, labels, pins, drafts, the outbox, changes not
  yet on the server, and POP3 or imported mail, which may have no other
  copy. The daemon stops the workers meanwhile (`ResetCache`,
  `Store::forget_downloaded_mail`); the indexer empties the index in place
  so apps searching it never lose it.
- **Signatures.** Any number, each with a name; one default for new mail
  and one for replies and forwards. Each is edited with the rich editor
  and its own small toolbar (font, size, colors, link, picture, table,
  align, lists) and is stored as text plus HTML when it has formatting.
  The compose bar's signature button swaps the signature in the body. A reply starts with the signature the
  user signed their newest message in the conversation with, found by
  comparing the text after its `-- ` line (`signatures.rs`); otherwise the
  reply default. The single signature of older versions becomes the first.
- **Grammar.** Harper (`harper-core`, Apache-2.0) checks English drafts,
  text and subject, on this computer as you write (`grammar.rs`), on by default, under
  Settings → Compose → Grammar. Paragraphs are checked off the UI thread
  half a second after typing pauses, cached by their text; paragraphs
  that do not look English, quotes and the signature are skipped, and
  Harper's own spelling rule is off (spelling is Hunspell's). Mistakes
  get a straight amber underline, apart from spelling's red wave; a right
  click shows the message, up to four fixes and Ignore (for that draft).
  The dialect follows the spelling language (British, Canadian,
  Australian, Indian, else American). Harper's dictionary takes about
  135 MB and stays loaded for the life of a process once any rule touches
  it, so Harper runs in a helper process: the app binary started with
  `--grammar-helper <language>`, one paragraph in and its mistakes out
  as a JSON line each way over its pipes. It starts when a message opens
  and is stopped when the last one closes (or grammar checking is turned
  off), so the app itself stays small. The app binary grows about 10 MB.
  Other languages are for Harper upstream.
- **Writing suggestions.** While the cursor is at the end of a paragraph
  the user writes (not a quote or the signature), the likely rest of the
  phrase shows in grey after it, laid out and wrapped like text but not
  in the document; Tab (or Right) takes it, Escape or typing on drops it
  (`suggest.rs`, `RichEditor::set_suggest`). It is a table of which word
  followed which one or two words, learned in the background on the
  first message written from the newest 3,000 sent messages (the user's
  own text only: quotes, "On … wrote:" and the signature are cut) plus
  about forty phrases common in mail. A reply adds a second table from
  the conversation it answers, where a phrase seen once counts, and that
  conversation's names and longer words complete as they are typed
  ("Thursday at" → "3pm" when the mail asked for Thursday at 3pm). A
  word is offered only when it followed its context at least twice
  (counting both tables) and at least 60% of the time, up to five words,
  so it stays quiet when unsure and in languages it has not seen.
  Looking up is a few hash lookups per keystroke on the UI thread;
  nothing leaves the computer. On by default, Settings → Compose →
  Writing suggestions.
  - *Planned, second layer:* whole sentences that answer the mail
    ("Thursday works for me, see you then") need a language model. It
    would be an optional small on-device model, downloaded only when the
    user turns it on, run by a helper process (`katna-suggest`) that
    starts while a message is being written and exits after, fed the
    conversation and the text so far, and answering after a pause in
    typing; the phrase tables stay the instant answer. The app and the
    daemon never load the model, so their size and memory budgets hold.
    No cloud service.
- **Sending account, Send and archive.** Settings → Compose picks the
  account new mail goes out from (`sending.send_from`): the first account
  in Settings → Accounts order (default, when unset), always the same
  address, or the one whose mail is open (`"current"`). Replies and
  forwards go out from the account whose mail is open. The account is set
  when the message opens; the compose window's From row shows it, and its
  arrow picks another account for this message. "Send on replies" makes Send on a reply or forward also archive
  the conversation (`sending.send_and_archive`); the menu beside Send
  offers the other way. The archive happens once the message is queued,
  and Undo on "Sent and archived" brings the conversation back as well.
- **Send checks.** Before any send (Send, Send and archive, schedule
  send) the compose window asks, as webmail does
  (`window/compose/checks.rs`): when the subject or the user's own words
  (not the quoted or forwarded message, not the signature) speak of an
  attachment ("attached", "attachment", "enclosed", "PFA") and no file or
  picture is attached, "Did you mean to attach files?" with Attach a
  file or Send anyway; then, for an empty subject, "Send without a
  subject?" with Add subject (the cursor goes to Subject) or Send
  anyway. English words only for now.
- **Keyboard shortcuts.** Every action has one (`window/keymap.rs`), with
  Gmail's keys as defaults: j/k, o, u, c, r, a, f, e, #, !, v, s, x,
  Shift+I/U, `* a`, `* n`, z, `g i`/`g s`/`g t`/`g d`/`g a`, /, ?, and Ctrl
  keys for search, quick settings, reload and quit. A shortcut set starts
  them from another mail app's keys instead, as in Mailspring: Gmail,
  Inbox by Gmail, Apple Mail (Ctrl for Cmd, Alt for Control), Outlook or
  Thunderbird (`[shortcuts] set`); an action that app has no key for keeps
  Katna's. The Settings page lists them all in two columns; a click on a
  key (or +) and the new keys change it, a key used elsewhere moves over
  with a note, and each shortcut or all ("Restore defaults") can go back
  to the set's keys. The user's changes sit on top of the set and survive
  a change of set. Keys without Ctrl or Alt only work in the list and the
  open conversation, never while typing, and a switch turns them off, as in
  Gmail. Only changes are saved (`[shortcuts.keys]`).
- **Compose.** A "New Message" window docked at the bottom right, as in
  Gmail: title bar with minimize, full size and close; To (with Cc and Bcc
  links), Subject, and the rich body with the signature after a `-- `
  line. The bottom bar has the Send button with a menu (schedule send:
  suggested times and a date and time picker; the scheduled count), the
  formatting bar toggle (undo, redo, font, size, bold, italic, underline,
  colors, align, lists, indent, quote, strikethrough, clear formatting,
  table; the tail moves into a menu when narrow), attach (the file picker
  or files dropped on the message, 25 MB in all), link (Ctrl+K), emoji,
  photo, calendar event (says it comes with Katna Calendar), signature,
  More (default to full screen, label (coming soon), plain text mode,
  print, check spelling) and discard. A right-click gives spelling
  suggestions, clipboard, link and table actions. Resting the pointer
  on an underlined word (600 ms) or a left click on it shows just its
  fixes in a card under it, in the text and the subject; the card closes
  when the pointer leaves the word and the card, on a click outside, on
  Esc or when typing resumes. Mail waiting to be sent
  later gets a *Scheduled* row in the folder list after Sent, which opens
  a list with Cancel send; a cancelled message opens again as written.
  Paste and drop work as in a desktop mail app (`compose/paste.rs`,
  `katna-ui` `rich/editor/paste.rs`): the clipboard is read with its HTML,
  copied files and pictures (`gpui_linux::read_rich`, a Katna patch to
  GPUI's Linux clipboard, vendor/gpui-pre-linux/KATNA.md), so text from
  Word, LibreOffice or a browser keeps its formatting (`html::from_pasted_html`:
  style sheet classes, Word's lists, merged cells, cell colors; the page's
  own near-black text and white background are dropped so the text follows
  the theme), and spreadsheet cells (or tab-separated rows) become a
  table. A bar under the paste offers Keep formatting or Plain text, or for
  cells Table, Picture (the source app's picture, else one drawn with
  `katna_preview::table`) or Plain text, until the next edit; Ctrl+Shift+V
  pastes plain text. Copying offers HTML too. Files copied in a file
  manager or dropped are attached; pictures pasted or dropped go in the
  text (attached when dropped outside the text or in plain text mode) with
  an Inline / Attachment choice under them. Text, cells or a picture
  dragged from another app arrive as a content drop
  (`gpui_linux::dropped_content`) and go in where they are dropped; the
  dashed drop cover says "Drop here" for those and "Drop files here" for
  files. The 25 MB total counts pasted and dropped pictures.
  The expand button in the compose title bar moves the message into a
  normal window of its own (`compose/popout.rs`), framed like the mail
  window: Katna's header bar with the window buttons, rounded corners and
  shadow where Katna draws the frame (GNOME), or the desktop's own title
  bar where the desktop draws it (KDE). A button docks it back: in the
  header bar, or in the bottom bar under the desktop's title bar, where
  no toolbar repeats the title. The message stays in the mail window's state and the new
  window only draws it, so sending and the snackbar work the same. Closing
  that window closes the message as its close button does; the app quits
  only when the mail window closes.
  Compose, Reply, Reply all and Forward all open it, filled in (recipients,
  `Re:`/`Fwd:`, the quoted message, `In-Reply-To` and `References`). Send
  builds an RFC 5322 message (`outgoing.rs`) and hands it to the
  daemon's outbox (`QueueSend`) with the undo-send delay; the snackbar's
  Undo takes it back (`UndoSend`, then `DiscardSend`) and opens it again. A
  message the server refuses for good raises a snackbar
  (`OutboxChanged`).
- **Adding an account.** A dialog in steps (`window/add_account.rs`),
  shaped after Mailspring's and Thunderbird's. First a grid of provider
  tiles (`window/mail_providers.rs`): Google, Microsoft (only when this
  build has its client ID), Yahoo, iCloud, Zoho, Fastmail, GMX, Yandex and
  "Other mail" for any IMAP or POP3 server, each with its own mark. Google
  and Microsoft sign in in the browser (`SignIn`; "Continue in your
  browser" until the provider's page answers, Back or Cancel ends it with
  `CancelSignIn`). The others lead to one form: name for the From line,
  address and password with "Show password", and a help box saying what
  the provider needs first (an app password, or IMAP turned on for GMX)
  with a link to its page. Add account looks for the servers
  (`DiscoverAccount`, §6) and checks the login, showing each stage with a
  spinner. When nothing is found, or from "Server settings", the servers
  are entered by hand: IMAP or POP3 for incoming mail, then host, port,
  SSL/TLS, STARTTLS or none, and the username, for incoming and SMTP.
  `AddImapAccount` or `AddPop3Account` checks the login before saving; a
  refused password is shown under the field. The last step shows what was
  set up (receiving and sending servers, and for POP3 what stays on the
  server) with "Add another account"; a Zoho account is offered "Sign in
  with Zoho" there for its tasks and calendars. An
  OAuth2 account whose sign-in stopped working shows a note at the bottom
  of the window with "Sign in" (`window/sign_in_again.rs`). It opens from the first-start pages
  (no account yet), the account card above the rail's account picture ("Add
  another account", which also lists the accounts and opens their
  inboxes), and Send without an account. The daemon signals `MailChanged`
  after each account's first sync, so a new account's folders show even
  when they are empty.
- **First start.** With no account, pages fill the window instead of an
  empty list (`window/onboarding.rs`): Welcome (what Katna does), Account
  (checks that the background service answers, which D-Bus activation
  also starts, and says how to start it when it does not; then the Add
  account dialog), Look (reading pane, theme and density, applied at once)
  and Ready, which offers the tour. The pages slide in and keep one height
  so nothing jumps. While an account waits for its first sync, an empty
  folder says the mail is on its way instead of "No mail".
- **Tour.** A walk through the window (`window/tour.rs`): the page dims
  around one part at a time (Compose, search, the menu button, the apps,
  the tabs, the list, quick settings, the account) with a card saying what
  it is for, Back and Next (or the arrow keys) and Skip (or Escape). The
  lit box glides from part to part. Parts not on screen are left out. It
  follows the first-start pages; people who already had an account get an
  offer of it once (`onboarding.done` in `config.toml`), and quick
  settings starts it again.
- **What's new.** The owner asked that an update not look like a first
  start. After an update the window shows What's new once
  (`window/whats_new.rs`): the version now running (the package version,
  `0.0.0.r90.gabc1234` until there are tagged releases; the PKGBUILD
  passes it as `KATNA_VERSION`), the highlights not shown before (at
  most six: a major one first even when older, then the newest), and Full changelog (GitHub's comparison of the
  previous build's commit with this one). The highlights are curated one
  TOML file each in `apps/katna-mail/whats-new/highlights/`, named
  `YYYY-MM-DD-HHMM-slug` by the time they were written, and `build.rs`
  builds them into the app in name order: a change people will notice
  adds a file. A file per highlight means changes merged side by side
  never touch the same lines (a shared list with numbered entries made
  every pair of pull requests conflict). A major feature may
  carry a short looping animation: two animated WebPs, light and dark
  theme (`apps/katna-mail/whats-new/`, at most 600 KB each, recorded at
  the size they are drawn, 560 px wide), shown across the top of the
  dialog. Its frames are decoded only while the dialog is open (about
  20 MB for a 50-frame clip) and freed when it closes. `config.toml`
  keeps `onboarding.whats_new_shown` (the names of the highlights shown,
  so one merged after newer ones still shows) and
  `onboarding.last_version`, both written as soon as the window opens, so
  nothing shows twice. Files from when the highlights were numbered have
  `onboarding.whats_new_seen` instead: the first 26 names, in their old
  order, stand for those numbers, and it is replaced on the next start. A first start (no account, or no settings file yet)
  gets onboarding or the tour and marks every highlight seen. Settings
  written by versions before What's new count as an update, which is why
  such a user no longer sees the tour again. Updates without new
  highlights show nothing. Quick settings > Help > What's new opens the
  newest highlights at any time. On a phone the dialog fills the window.
- **About Katna.** Help > About Katna in the global menu, Quick settings >
  Help > About Katna, and the version pill at the top right of the
  Settings header open the About dialog (`window/about.rs`): the version,
  What's new, the changelog and source links, "Buy me a coffee", a Follow
  row, a line of thanks to Rust, KDE and Linux, and the free software
  Katna is built on, each with its license. The coffee link and each
  Follow link are one constant (`SUPPORT_URL`, `FOLLOW`); unset ones show
  as "Coming soon" or stay hidden. `CREDITS` in the same file picks the
  heart of Katna by hand; "Every library Katna uses" below it lists each
  direct dependency with its version, authors, license and repository
  from `docs/credits.json`, which `ci/gen-credits.sh` writes from
  `cargo metadata` together with CREDITS.md. A short "A personal project"
  note says where Katna's ideas come from (Gmail, Mailspring,
  Thunderbird) and that LLMs made it possible. On a phone it fills the
  window.
- **After the first real install.** The owner's first run on KDE brought
  these changes. The account picture moved
  to the top right, beside the settings gear, with its card below it; the
  search box is 40 px tall. Compose first sat in the top bar in place of
  the app name; the owner later moved it (2026-09-27): it is a 56 px
  pill at the very top of the folders, above the account's name when there is
  one, and while the folders are folded (and always on a tablet or on
  another app's page) it is a 56 px square at the top of the app rail.
  It slides between the two as the folders open or fold, while the
  rail's apps move down to make room, and resting on it in the rail
  opens the folders over the list, as resting on Mail does (Escape or
  leaving closes them). The top bar shows the Katna mark and "Katna
  Mail" in its place, or Katna Calendar, Contacts, Tasks, Notes or
  Feeds; switching apps rolls the second word, the old one down and out
  and the new one down into its place. The menu button (a panel icon, not a
  hamburger: its left part is filled while the folders show and fades to
  an outline as they fold, following the drawer on a tablet or phone;
  "Hide folders" / "Show folders") folds the folders away completely; resting on Mail in the
  app rail opens them over the list as a floating panel with rounded
  corners and a bottom margin. Ripples keep to the shape of the element
  they are on (`Ripple::rounded`), since GPUI clips children to
  rectangles. Where the element cuts the growing circle, its ends are the
  circle's shallow curve, so a wave in a wide, low tab fills it as one
  rectangle rather than showing a pill. Icon buttons have tooltips after GPUI's hover delay
  (`katna_ui::Tooltip`). Dialogs, panels and cards use 15 px corners.
  Reply, Reply all and Forward stay pinned at the foot of the open
  conversation. Answering writes inline at the end of the conversation,
  as in Gmail: a card with the recipients, the text and the Send row,
  which pops out into a desktop window of its own; docking it there brings it back to the conversation. The card grows with its text and
  scrolls with the messages; opening it scrolls smoothly to its first
  line, and typing keeps the cursor in view. Its Send row sticks to the
  bottom of the pane while the text runs on under it. A reply's quoted
  message starts folded behind a "..." button (it is still sent). The
  formatting bar (Aa) floats over the end of the text, tinted and as wide
  as its buttons, so opening it moves nothing. On a phone, and a tablet
  too narrow for the reading pane, New Message covers the whole window;
  the list's single-letter keys are switched off inside text
  fields. The list has a right-click menu (reply, reply all, forward,
  archive, delete, read, snooze, star, then the submenus Move to, Follow
  up (tasks, notes, meetings, calls) and More (spam, importance, pin),
  and find emails from the sender) acting on the ticked lines or the
  clicked one. It opens at the pointer, flips left or up where there is
  no room and else is pushed in from the edge; where the window is too
  narrow for a submenu beside it, or too short for the whole menu, a
  submenu opens in its place under a row back. In a short window its
  items first come closer together (36 px down to 28 px), and only then
  does the menu scroll; a long submenu does the same. The Calendar page
  has right-click menus in the same card (`calendar/menu.rs`): on a free
  time or day (a new event, a task on the Task tab, focus time or out of
  office there, and Open day), on an event (details, edit, duplicate, delete, Going?, join,
  email guests, Google's eleven colors and Move to another calendar,
  the browser or the contact) and on a task (details, done, star, Date:
  today, tomorrow, in a week, all day or no date, and delete). Changes
  go through the same paths as the event card and the Tasks page, so a
  repeating event asks which occurrences and each change has Undo. In
  the side panel (`calendar/side_menu.rs`) a calendar's menu has Show
  only this, Color, Rename and Delete (Remove from list for one shared
  with the person), and an account heading's has New calendar, Show or
  Hide all and Account settings. These change the calendar on the
  account's service first, by its best method (Google Calendar API
  `calendars` and `calendarList`, Microsoft Graph `/me/calendars` with
  the nearest Outlook colour, CalDAV `MKCALENDAR`, `PROPPATCH` and
  `DELETE`, Zoho's `calendars` for one's own), and only then in the store
  (`katna_sync::calendar::manage`, the daemon's `calendar/manage.rs`);
  what a service can't do shows dimmed with a short reason. Rename and
  colour have Undo; delete asks first. The "select all
  on screen" banner no longer blinks (it depends on what was ticked, not on
  how many lines fit), inbox tabs switch without a fade, and the reading
  pane choices in quick settings play a small demo under the pointer.
- **Desktop colors.** The layout stays the webmail one, but its colors
  come from the desktop (`katna_platform::colors`, `window/colors.rs`):
  - *KDE*: the active color scheme from `kdeglobals` (`[Colors:Window]`,
    `[Colors:View]`, `[Colors:Selection]`, with KDE's built-in Breeze
    Light for missing keys) and `AccentColor` from `[General]`.
  - *GNOME*: the libadwaita light or dark palette with the accent color,
    and any `@define-color` a theme tool wrote to
    `~/.config/gtk-4.0/gtk.css` (inside `@media (prefers-color-scheme)`
    blocks too).
  - *Accent color*: the Settings portal (`org.freedesktop.appearance
    accent-color`), else GNOME's `accent-color` GSettings key.

  The page takes the window color, the cards the view color, and
  highlights (selected folder, Compose, ticked rows, the first tab) are
  tints of the accent; text and accents are darkened or lightened until
  they read. A scheme is used only when it is as dark as the window asks
  (the theme setting may force light or dark); otherwise, and on desktops
  without a scheme, Katna's palette is drawn in the accent color, or left
  as it is without one. The window frame (`katna_chrome::ChromeColors`)
  follows the scheme too. Colors are read at startup, again on the
  portal's `SettingChanged`, and when `kdeglobals` or `gtk.css` change
  (checked every 2 s).
- **Mode, colors and accent (2026-10-01).** Three separate choices in
  Settings > Appearance: *Mode* (`mail.theme`: System, Light or Dark),
  *Colors* (`mail.colors`: `system` for the desktop's scheme above,
  `katna` for Katna's palette, or a built-in scheme, `schemes.rs`) and
  *Accent* (`mail.accent`: the scheme's own, `system` for the desktop's,
  or `#rrggbb`). Every built-in scheme has a light and a dark side, made of
  six colors (page, cards, text, faint text, accent, error) that
  `Theme::from_scheme` turns into the full theme, so any mode works with
  any scheme; `Theme::pick` puts the three together. The built-ins are
  Katna's own (Katna, Clear after Apple's system colors, Graphite) and
  eleven MIT-licensed editor and desktop palettes (Nord, Solarized,
  Dracula, Gruvbox, Catppuccin, Tokyo Night, One, Rosé Pine, Everforest,
  Kanagawa, Ayu), credited in About and the README. Settings shows each
  as a card with a small mail window on its light and dark side in the
  picked accent, grouped as Built in and From your system. Files from
  before schemes have no `colors`: `mail.desktop_colors` decides (on is
  `system`, off is `katna`), and every pick keeps it in step for older
  versions. The quick setting *Desktop colors* switches between `system`
  and `katna`.

  *From your system* lists the desktop's other schemes
  (`DesktopScheme` in `katna_platform::colors`): on KDE every installed
  `*.colors` file (`$XDG_DATA_HOME` and `$XDG_DATA_DIRS`, `color-schemes/`),
  with a light and a dark scheme whose file names differ only by `Light`
  and `Dark` paired into one (`kde:Breeze`); on Windows its own light and
  dark colors and the Contrast themes in `%WINDIR%\Resources\Ease of
  Access Themes`. The scheme in use on KDE gets its installed partner, so
  System works in either mode (Breeze Dark on the desktop and Mode Light
  draws Breeze Light). Windows' accent is `DWM\AccentColor`; while a
  Contrast theme is on, System draws its colors. A scheme with one side
  (a Contrast theme, a KDE scheme without a partner) decides light or
  dark itself, whatever Mode says (`Theme::forced_dark`), as KDE does.
  Windows' colors are read at startup.

  *Yours* lists the schemes people make (`user_schemes.rs`): one TOML file
  each in `<config>/colors/`, ids `user:<file stem>`, with the eight
  colors of a light side, a dark side or both (the six above plus top bar
  text and text on the accent). *Customize…* copies the selected scheme,
  as drawn in the picked accent, into the editor; *Import…* reads a Katna
  file or a KDE `.colors` file (one side). The editor shows both sides
  next to each other, each color with a hex field and a swatch, the mail
  window drawn in them and what reads badly (text under 4.5:1, faint text
  and text on the accent under 3:1); *Make dark from light* works a dark
  side out from the light one's hues. A card's right-click menu has
  Customize, or for one's own Edit, Duplicate, Export and Delete (with
  Undo: the file comes back, in use again if it was).
  A swatch opens a color picker beside it (a popover, its notch pointing
  at the swatch, so the dialog keeps its size): a saturation and
  brightness square, a hue bar, a hex field, the side's colors and the
  ones picked lately. Linux has no portal for a color dialog, so the
  picker is Katna's own; its dropper is the Screenshot portal's
  PickColor (KDE, GNOME), and *System picker…* runs `kdialog --getcolor`
  or `zenity --color-selection` where installed. Windows has neither yet
  (ChooseColor needs unsafe FFI, and there is no system dropper).
- **Contact panel.** On a desktop, a card beside the open conversation
  (300 px, the usual 16 px card gap, sliding in with the reading pane's
  spring) shows one of its people: the newest sender other than the user,
  or whoever is picked under "In this conversation". It shows their
  picture (the sender pictures above), name and address; phone, title and
  company from the signatures of their newest stored messages
  (`profile.rs`: after a `-- ` line or a sign-off such as "Best regards",
  never in quoted text); their time of day from the UTC offset of their
  latest `Date` header; the mail exchanged over all accounts (server
  copies counted once by `Message-ID`); the five newest conversations and
  six newest files, which open the conversation or the viewer; and the
  open tasks made from mail they take part in (the task's mail or another
  message of its conversation, `contact_on_mail`), with a tick to complete
  one and a click to open it on the Tasks page; and the next three
  meetings they organise or are invited to within 60 days (not
  cancelled, not declined; a series shows its next time once), which
  open on the Calendar's day view with the event's card. Everything
  is local (`katna_store::Store::contact_*`); outside data (LinkedIn, X,
  company facts) is left for the Katna Server plan. It shows only while
  the list and reader keep 900 px (600 px with the reader alone), never on
  tablets and phones or in a conversation window; a button on the reader
  toolbar turns it off (`mail.contact_panel`). When it fits only with the
  folder pane folded, it folds the pane as it slides in and unfolds it once
  it goes (hidden, the mail closed, or the window grown wide enough for
  both); a pane folded by hand stays folded, and one opened by hand beside
  it wins until the panel is next shown (`fold_nav_for_contact`).
- **Day's agenda.** A Calendar button on the top bar, beside Settings
  (the Mail page of a desktop window only), opens a card at the
  right of the mail with one day's events, as Gmail's side panel has it
  (`window/agenda.rs`, `mail.agenda_panel`). It and the contact panel
  take turns.
- **Not there yet.** Drafts are not saved (closing a written message
  discards it and says so). Labels on a message being written and
  calendar invitations wait for their features.

### 13.7 Later: notes on mail and Workspace

Two ideas from the owner for a later phase. Nothing is built for them yet.

- **Notes on mail.** Attach a note to a message or conversation for later
  reference. The notes live in the Notes app (rail), so a note can be found
  from the mail and the mail from the note. The owner plans more Notes
  features around this.
- **Workspace.** A view that shows only the mail the user has to act on.
  Replying to a conversation takes it out of the Workspace. The user can
  give mail a priority or a marker, or snooze it to come back into the
  Workspace at a set date and time.

### 13.8 Attachments and the attachment viewer

Received attachments open inside Katna Mail, like webmail's preview; the
desktop's own app stays one click away.

- **Cards.** Under each open message, one card per attachment (the
  webmail layout): a thumbnail (pictures, and the top of a PDF's first
  page), a glance drawn small on a white page (the top-left cells of a
  spreadsheet or CSV, the first lines of a text file or document, the
  first slide's text centered; `katna_preview::glance`, skipped above
  20 MB), or a colored type badge,
  and the file name. Hovering shows the name, the size and a Save button
  on frosted glass; "Save all" saves every attachment to a folder.
  Thumbnails are made in the background from the stored raw message and
  freed when the conversation closes.
- **Viewer.** Clicking a card opens the viewer over the window below the
  top bar (the window's own controls stay usable): with frosted menus on,
  the window shows blurred under a dark veil and the viewer's bar (a
  shade darker), its controls pill, foot pill and markup pill are frosted
  too; the file shows below the bar, never under it; with frost off, a
  plain darker veil. The bar names the file, Forward, "Open with another
  app" and Save (Forward starts a new mail with only that file attached,
  the marked copy for a marked PDF; the no-preview page and an open
  mail's attachment cards, beside Save, offer it too); arrows (and ←/→) go
  through the message's other attachments, the viewer staying open: the
  file on show stays until the next one is ready (a PDF with its first
  page drawn) and they swap in one frame, or "Opening…" shows after
  300 ms if it takes longer; the viewer fades in only when it opens. The
  middle of the top bar zooms (−/+/0, Ctrl + mouse wheel or a touchpad
  pinch around the pointer, 25 %–400 %, 100 % fits the window)
  and shows a PDF's page (or a presentation's slide) as "Page [n] of N":
  typing a number in the box (click it or Ctrl+G) and Enter goes to that
  page, Up/Down in it or its ▲▼ (shown on hover, repeating while held)
  go a page back or on, Escape leaves the box. Fit fits a PDF page's
  height (the whole page), a picture to the window, and a document's,
  presentation's or sheet's width; a picture also has Real size (1:1)
  and turns on screen only (Save keeps the file). Zoom between the
  steps (from Fit or 1:1) goes on from the nearest step. A PDF also turns a quarter turn either way (Ctrl+R, Ctrl+Shift+R):
  every page turns, the page on show stays, marks turn with it, and a
  marked copy is saved turned (`/Rotate`). Too narrow for the bar (a
  phone), these float in a pill at the foot instead. A click on the dim
  space around the file closes the viewer, as in Gmail; a click on the
  page, a control or the bar, or a drag, does not (nor while a menu, the
  unsaved-marks question or a note being typed is open). Escape closes the viewer. It is dark in light and dark themes alike.
  - **PDF:** `hayro` (pure Rust, CPU, Apache-2.0/MIT) draws the pages.
    Only pages on screen (and one either side) are drawn, at the zoom and
    the screen's scale, one at a time on a background thread; pages far
    off screen are freed. Password-protected PDFs are not opened yet (the
    viewer says so and offers the other app).
  - **Pictures:** PNG, JPEG, GIF, WebP, BMP, TIFF through the `image`
    crate GPUI already uses, turned upright by their EXIF orientation and
    scaled to at most 4096 px; animated GIFs are drawn by GPUI. SVG is
    drawn by `katna_preview::svg` (resvg with nothing linked loaded; see
    §12), never by GPUI.
  - **Text** (`text/*`, JSON, logs, code by extension): monospace, the
    first 512 KB and 10,000 lines.
  - **Spreadsheets:** Excel (xlsx, xlsm, xlsb, xls) and OpenDocument (ods)
    read by `calamine` (pure Rust, MIT), and CSV/TSV (separator guessed:
    comma, semicolon, tab or bar; also CSV sent as `text/plain`). A grid on
    white with column letters kept at the top, row numbers, numbers on the
    right, and a tab per sheet at the foot. Values only: formulas show
    their saved result, dates show as dates; no cell colors, merged cells
    or charts. Up to 20,000 rows, 256 columns and 2 million cells.
    calamine lays a sheet out from its first cell to its last, so xlsx
    and xlsb sheets are read cell by cell and cut to those limits (one
    cell in A1 and one in XFD1048576 would otherwise ask for 17 billion
    cells), and an xls file, which calamine lays out whole while opening
    it, is refused when a sheet spans more than 4 million cells.
  - **Documents:** Word (docx) and OpenDocument text (odt), read by
    `katna-preview` itself (the zip through `zip`, the XML through
    `quick-xml`, both MIT): one long white page with the title, headings,
    numbered and bulleted lists (Word numbering and list styles, ODF list
    styles), tables, alignment and bold/italic/underline/strike-through.
    Pictures, headers, footers, notes, comments and text boxes are left
    out. Only paragraphs on screen are laid out. Word 97–2003 (.doc) is
    read into the same model (`katna_preview::word`): the OLE compound file
    through `cfb` (MIT), then the FIB, the piece table (UTF-16 or
    Windows-1252 text), the character and paragraph property pages, the
    style sheet (built-in heading, title and subtitle styles) and the list
    tables, giving the same headings, lists, tables, alignment and looks;
    fields show their result, hidden text is dropped, and encrypted or
    Word 6/95 files are not read. RTF has no preview.
  - **Slides:** PowerPoint (pptx, ppt) and OpenDocument (odp),
    `katna_preview::slides`, shown as text: each slide is its own white
    page under a "Slide N" label, title first, then its text (bulleted
    body placeholders, numbered lists), and its tables. pptx follows the
    presentation's slide list; ppt follows the persist directory from the
    last edit to the document's slide list and reads each slide's text
    atoms, falling back to the texts kept in the slide list; odp reads
    `draw:page`s. Pictures, charts, layout and speaker notes are left out.
    A slide without a title placeholder takes a short first line as its
    title.
  - **Selecting and copying.** Text files, documents, slides and PDFs
    select like message text (§ "Message text can be selected" above,
    `window/select.rs`, shared with the reader): drag, double- and
    triple-click, Shift+click, Ctrl+A, Ctrl+C and a right-click Copy,
    also to the primary selection. Copying and Ctrl+A reach text scrolled
    out of sight. A PDF's text comes from the page itself: hayro reads each
    page with a device that keeps every glyph with a known character
    (ToUnicode, glyph names) and where it is drawn, and glyphs on one
    baseline become a line (`katna_preview::pdf::TextLine`); pages are read
    in the background, eight at a time, up to 2,000. The selection is drawn
    over the page's picture. Scanned PDFs have no text to select.
    Spreadsheets select cells instead: click, drag or Shift+click for a
    range, a column letter or row number for all of it; Ctrl+C copies
    them tab-separated (cells with tabs, line breaks or quotes quoted), so
    they paste as cells into other spreadsheets.
  - **Marking up a PDF.** The pen in the viewer's top bar shows a pill of
    tools: Select, Highlight, Underline, Squiggle, Strike, Pen, Sticky
    note, Text box and Eraser, five colours each, and Undo and Redo
    (Ctrl+Z, Ctrl+Shift+Z). Text marks are made by selecting text; the
    pen draws freehand; a click with the note or text tool places one and
    opens it for typing (Ctrl+Enter, Done or a click elsewhere finishes,
    Escape drops the change), and clicking one opens it again; a note's
    text shows on hover; the eraser removes the mark under it. Text boxes
    are laid out and drawn in Helvetica, which every PDF reader has, so
    characters outside Latin-1 show as "?" in the saved copy (the full
    text stays in the annotation's Contents). Reply (in the bar, for the
    open conversation's attachments) attaches the marked copy to a reply
    to that message. Marks are
    kept in points as the page is drawn (`katna_preview::markup`) and Save
    writes a copy, "<name> (marked).pdf", with them as standard
    annotations (Highlight, Underline, Squiggly, StrikeOut, Ink, Text,
    FreeText), each
    with its own appearance, added to the end of the original file as an
    incremental update with `lopdf` (`katna_preview::pdf_marks`), so
    signatures stay valid. Encrypted or certified PDFs can't be marked.
    Closing or paging away with unsaved marks asks: Discard, Keep marking,
    or Save a copy. No redaction.
  - Anything else opens straight in the desktop's default app, and so
    does a file of a previewable type that turns out unreadable (damaged,
    encrypted, Word 6/95; the viewer closes and hands it over, or asks
    which app when Default apps says Ask). Files that could run a program
    never do: they show "No preview available" with Save only. Paging to
    such a file with the viewer's arrows shows that page with "Open with…"
    rather than launching an app.
- **Default apps** (Settings → Default apps, `[mail.open]` in
  `config.toml`): for PDFs, pictures, text, spreadsheets and documents
  (slides included),
  clicking a card opens Katna Mail's viewer (the default), the desktop's
  default app for the type, or asks which app each time. Files without a
  preview always open in the desktop's default app (see above). Which app is the desktop's default
  is set in the desktop's own settings.
- **Save** asks where through the desktop's file chooser (portal),
  starting in the download folder (`XDG_DOWNLOAD_DIR`); without a portal
  it saves there under a free name. **Open with another app** writes a
  read-only copy to `$XDG_CACHE_HOME/katna/opened/` (removed after a day)
  and asks the desktop's "Open with" portal (`org.freedesktop.portal.
  OpenURI` with `ask`), which lists the apps for the type; without a
  portal, the default app opens it (`xdg-open`). "The desktop's default
  app" in Default apps skips the question. Files that could run a program
  (`.desktop`, scripts, executables, `.jar`, Flatpak refs) are never
  handed over; they can only be saved.
- `katna-preview` holds the decoding (no GPUI); `katna_render::
  attachment_file` extracts an attachment's bytes from the raw message.
  The app reads only the store, like the rest of the reader.
- **Size:** the viewer adds 7.5 MB to the Katna Mail release binary
  (31.55 → 39.10 MB, measured on the same main). Nearly all of it is
  `hayro` and its CPU rasterizer (`vello_cpu`, compiled for several SIMD
  levels, and `pic-scale`); pictures use the `image` crate GPUI already
  links. A pure-Rust renderer was chosen over PDFium or Poppler so the
  package needs no C library and the app keeps `unsafe` out.
- **Encrypted mail:** attachments of an encrypted or signed message are
  read from the message as GnuPG opened it, held in memory with the rest
  of the decrypted message. Their thumbnails and the viewer stay in
  memory. "Open with another app" hands a decrypted attachment over only
  from `XDG_RUNTIME_DIR` when that is in memory (tmpfs, checked in the
  mount table), never from the cache on disk; otherwise it says to save
  the file instead. Save writes where the user chooses.
- Not yet: text search in PDFs, printing, pictures inside documents,
  old Word files and slides.
- **Files page** (the attachment library, from the HEY study's Files):
  the last app of the rail (after Feeds, Ctrl+7, `--page files`) shows
  every named attachment of every account as the cards above, newest
  first under month headings, or as a list. It reads the attachment lists
  sync keeps (`Store::library_files`, `katna-store/src/library.rs`): no
  schema change, no server, works offline. Mail in Trash or Spam is left
  out, a file sent again (same name and size) shows once, and small
  pictures (signature logos) are left out: by default those under 12 KB,
  or under 100 px wide or tall (Settings > Default apps > Files page,
  `mail.files`). With them go signature pictures of any size: one its
  sender sent in three or more conversations (same name and size), or a
  social network's icon by name. Pixel sizes are read in the background from downloaded
  mail and kept in the cache directory (`files-picture-sizes.json`); they
  apply the next time the page opens, so cards never move under the
  pointer. It reads at most 20,000 files. The side column (a drawer and chips on a phone) narrows it to a
  kind of file, an account, or received or sent; chips pick a sender,
  days (quick picks over a two-month calendar: click a day, drag across
  days or Shift+click; the files follow the drag and the calendar closes
  on release; the wheel over the chip moves the days, keeping their
  length, whole months by months) and the order; the top bar's search box matches names, subjects
  and senders. A click opens a file as the list's chips do (downloading
  its mail first); the hover panel, the right-click menu and the viewer
  (opened from this page) offer **Show the mail**, and the menu also
  opens the mail in a new window, forwards the file in a new mail, and
  shows the sender's files. Thumbnails are made in the background only
  for cards on show whose mail is downloaded, and at most 96 are kept.

### 13.9 Window sizes

The owner asked for the window to follow its size: a phone-sized window
looks like Gmail's mobile app, a tablet-sized one like its tablet app, and
moving between them animates rather than jumps. `window/layout.rs` picks
one of three layouts by the width inside the window frame
(`WindowChrome::inner_width`, which leaves out the CSD shadow margins):

| Layout  | Width         | What changes |
|---------|---------------|--------------|
| Desktop | 1080 px and up | §13.6 as is. |
| Tablet  | 600–1080 px   | The folders fold into a drawer the menu button opens over a dimmed list; Compose is a square at the top of the app rail, and the top bar shows the Katna mark and the app's name beside the menu button, the name folding away below 760 px; the reading pane (three-pane setting) stays beside the list from 840 px, and narrower the conversation slides in over the list. |
| Phone   | under 600 px  | No app rail: the apps sit in a bar along the bottom. The search box is a pill across the top bar with the menu button and account picture inside it (settings move to the drawer). The list is edge to edge, three lines a message with the sender's picture, which ticks the line when tapped; the inbox tabs move to the drawer. Compose floats at the bottom right; it folds to its pencil as the list scrolls down and grows back after a few steps up (or at the top). The search row and the list toolbar slide up out of sight once the list has scrolled past them, and come back as soon as it turns back up (or at the top); the list keeps still on screen while they move. An open conversation slides in over the list and the bottom bar sinks away; its messages use the room under the sender's picture, from the picture's left edge, and Reply, Reply all and Forward share the width equally. Composing takes a sheet over the whole window (below the top bar with Katna's own frame, whose window buttons sit there). Quick settings and the Settings page each fill the window between the top bar and the bottom bar, with no Compose button over them; the Settings page's section tabs stay on one line that scrolls sideways. |

The other apps' pages (Calendar, Contacts, Tasks, Notes) fold the same
way: on a tablet or phone their side column (calendars, labels, lists)
becomes a drawer the menu button opens over the dimmed page, and on a
phone the page is edge to edge. Contacts drops its columns to a name with
the address under it when the list is narrower than 640 px; Notes lays two
narrower cards across a phone; Tasks' cards and Calendar's event cards
never grow wider than the window. The ☰ application menu opens each menu
to the left of its card where the window has room, and otherwise (a
phone) in the card itself under a Back row (Left or Escape goes back).

Settings rows put the name beside the controls and wrap on width alone,
not on the layout: where the controls would get less than 300 px beside
the name, the name goes above them and both span the row, as in Gmail's
mobile settings. So a narrow tablet stacks them too. Long choices wrap to a
second line instead of being cut off.

The open conversation reads its own pane's width, not the layout. A pane
narrower than 420 px (a narrow reading pane, or the reading pane dragged
small) lays out as on a phone: the text uses the room under the sender's
picture and Reply, Reply all and Forward share the width, folding to icons
together. Under 260 px each message's date and star step aside (the date
stays under "to", starring in ⋮) and the subject takes a smaller size.
As the pane narrows the toolbar first drops Newer/Older (J and K still
step), then leaves Mark unread, Report spam and finally Delete to its ⋮
menu, so nothing is ever cut off at the card's edge.

A layout changes only 12 px past its threshold, so a window resized right
at a threshold does not flicker between two layouts. The GNOME minimum
window size (360 px) is the smallest phone layout.

Motion: two springs follow the layout (phone, desktop), and every part
reads them rather than switching: the rail slides out as the bottom bar
rises, the search box grows into the pill, the cards' margins and corners
melt away, the top-bar Compose shrinks into the phone's floating one. A third spring
slides the conversation over the list. All of them honor reduce motion.

At every size, Reply, Reply all and Forward stay on one line at the foot
of a conversation: as the pane narrows they drop their words one at a
time (Reply all first, then Reply, then Forward) and keep their icons,
with the word as a tooltip. The words are measured in the desktop's font.

### 13.10 Languages

Asked for by the owner on 27 September 2026. Until then every label,
menu, notification and tray item was English typed into the code, dates
used English month and day names in fixed formats, and numbers always
grouped by commas. Only the spell-check dictionary followed the desktop's
language (`spell.rs`).

**Languages.** 51 entries in the picker, 49 translations (the three
English entries share one text and differ only in formats). Each has a
BCP 47 tag, its own name, its English name and a flag:

| Group | Entries (tag, flag) |
|---|---|
| English | English (India) `en-IN` 🇮🇳, English (UK) `en-GB` 🇬🇧, English (US) `en-US` 🇺🇸 |
| South Asia | Hindi `hi` 🇮🇳, Bengali `bn` 🇧🇩, Tamil `ta` 🇮🇳, Telugu `te` 🇮🇳, Marathi `mr` 🇮🇳, Gujarati `gu` 🇮🇳, Kannada `kn` 🇮🇳, Malayalam `ml` 🇮🇳, Punjabi `pa` 🇮🇳, Odia `or` 🇮🇳, Assamese `as` 🇮🇳, Urdu `ur` 🇵🇰, Nepali `ne` 🇳🇵 |
| Himalaya and Sri Lanka | Sinhala `si` 🇱🇰, Dzongkha `dz` 🇧🇹 |
| East Asia | Chinese (Simplified) `zh-Hans` 🇨🇳, Chinese (Traditional) `zh-Hant` 🇹🇼, Japanese `ja` 🇯🇵, Korean `ko` 🇰🇷 |
| Southeast Asia | Thai `th` 🇹🇭, Vietnamese `vi` 🇻🇳, Indonesian `id` 🇮🇩, Malay `ms` 🇲🇾, Filipino `fil` 🇵🇭, Khmer `km` 🇰🇭, Burmese `my` 🇲🇲, Lao `lo` 🇱🇦 |
| Middle East | Arabic `ar` 🇸🇦, Persian `fa` 🇮🇷, Hebrew `he` 🇮🇱, Turkish `tr` 🇹🇷 |
| Europe | Russian `ru` 🇷🇺, Ukrainian `uk` 🇺🇦, German `de` 🇩🇪, French `fr` 🇫🇷, Spanish `es` 🇪🇸, Portuguese `pt-BR` 🇧🇷, Italian `it` 🇮🇹, Dutch `nl` 🇳🇱, Polish `pl` 🇵🇱, Swedish `sv` 🇸🇪 |
| Africa | Swahili `sw` 🇰🇪, Amharic `am` 🇪🇹, Hausa `ha` 🇳🇬, Yoruba `yo` 🇳🇬, Igbo `ig` 🇳🇬, Zulu `zu` 🇿🇦, Afrikaans `af` 🇿🇦 |

Portuguese is Brazilian Portuguese (most speakers); European Portuguese
can be added as its own entry later. Punjabi is Gurmukhi (`pa-Guru`).
Arabic, Persian, Hebrew and Urdu read right to left and mirror the whole
layout.

**Tooling: Fluent.** Strings live in Fluent files (`fluent-bundle`,
Mozilla's Project Fluent), one folder per binary per language with one
file per area, so changes made side by side add lines to different files:
`i18n/<tag>/katna-mail/<area>.ftl` (`list.ftl`, `reader.ftl`,
`settings.ftl`, …), `katna-ui.ftl` (shared widgets, one file),
`katna-daemon/` (notifications, tray, dock menu). Chosen over gettext
because:

- It is pure Rust with no `libintl`, and small (about 0.3 MB).
- Plurals and other variants are chosen inside each message with CLDR's
  categories, so Arabic's six plural forms, the Slavic few/many forms and
  languages with no plural all work without code changes. Translators can
  also vary a message by other values (the kind of folder, say), which
  gettext cannot.
- Variables are wrapped in Unicode isolation marks (FSI…PDI) by default,
  so a Latin name or address inside an Arabic or Hebrew sentence keeps the
  sentence's direction.
- Weblate and Pontoon both edit `.ftl` files, and it is the choice of
  other Rust desktops (COSMIC), so the tooling is proven.

gettext has more translators who know it and is KDE's own format, but in
Rust it needs a C library or an extractor that does not understand Rust
macros, and its plural handling is one formula per file. Weblate serves
either, so contributors lose nothing with Fluent.

All code goes through `katna-i18n` (a new crate without GPUI, used by the
apps and the daemon): `tr!("message-id")` and `tr!("message-id", count =
n, name = sender)` return the text in the current language, falling back
message by message to English, so a partly translated language still
shows everything. A unit test checks that every id the code uses exists
in English, and that each translation's variables match English. Log
messages, `katnactl` and D-Bus error names stay English.

The English files are embedded in every binary; the others are embedded
compressed with `zstd` (already a dependency) and unpacked on first use.
A file in `$XDG_DATA_HOME/katna/i18n/<tag>/` is loaded over the built-in
one message by message, so a reviewer can try a correction without
building Katna.

**Choosing the language.**

- **System default** (the default) follows the desktop: the first
  language in `LANGUAGE` (a list, as `bn:en_US`) that Katna has, else
  `LC_ALL`, `LC_MESSAGES`, `LANG`. On Plasma it also reads
  `~/.config/plasma-localerc` (`[Translations] LANGUAGE`, `[Formats]`),
  because the daemon, started by systemd, may not have the session's
  variables. POSIX names are mapped to the tags above (`bn_IN.UTF-8` →
  `bn`, `zh_TW` and `zh_HK` → `zh-Hant`, `zh_CN` → `zh-Hans`, `tl_PH` and
  `fil_PH` → `fil`, `pt_PT` → `pt-BR` until European Portuguese exists,
  `iw` → `he`). A language Katna does not have falls back to English (US).
- The user's choice is `general.language` in `config.toml` (empty =
  System default). The app and the daemon read the same key; the app tells
  the daemon over D-Bus when it changes, so notifications, the tray and
  the dock menu switch too. `KATNA_LANGUAGE` overrides everything, for
  testing.
- Formats: with System default, dates and numbers follow `LC_TIME` and
  `LC_NUMERIC` (on Plasma, `[Formats]`), as the desktop does, so a user can
  read English with Indian formats. A language picked in Katna brings its
  own formats; this is what makes the three English entries differ.
- Changing the language applies at once, without a restart: every string
  is looked up at render, cached measurements (such as the Compose button's
  width) are measured again, and the layout flips direction if needed.

**Language picker.** Two places change the same setting:

- A **language button** in the icon row at the top of the account card
  (opened from the account picture), beside Settings, as
  the owner asked, which keeps the top bar to Settings and the picture. It
  shows the current language's flag and a small chevron; its tooltip names
  the language ("Language: বাংলা, following the system" with System
  default). The popover opens under the account picture.
- **Settings > General > Language**, a row with the same choices.

The button opens a popover (the popover rules of §13.6: closes on Esc and
any outside click, stays inside the window, frosted when frosted menus are
on). At the top a "Search language" box, focused when it opens, matching
own names, English names and tags, ignoring case and accents. Below it
the list: "System default" first (with the language it resolves to as its
second line), then the 51 entries in the order of the table above. Each
row has the flag, the language's own name, its English name under it
(each in the other language's script, never transliterated), and a check
on the current choice. Arrow keys move, Enter picks. Under a
machine-translated language a line at the foot of the popover says so and
links to how to help (see below).

On a phone the top bar has no room (the search pill holds the menu and the
account picture), so Language is a row in the navigation drawer next to
Settings, and the picker opens as a sheet over the window like Quick
settings.

Flags are bundled SVGs (from `flag-icons`, MIT, in the app's assets),
drawn as colour images with rounded corners. Colour emoji are not used:
they depend on an installed emoji font and GPUI's colour-glyph support.
Language is not a country, so the flag only helps to find the row; the
names carry the meaning.

**Dates, numbers and plurals.** `katna-i18n` formats with ICU4X
(`icu_datetime`, `icu_decimal`, `icu_calendar`; `icu_locale_core` and
`icu_properties` are already in the tree) using CLDR data for the 51
locales only, baked with `icu4x-datagen` so the binary does not carry
every locale. `format.rs` keeps its rules (time today, weekday this week,
day and month this year, full date otherwise) and asks ICU4X for each
length, so month and day names, the order (`27/09/2026`, `9/27/2026`,
`2026/09/27`), 12- or 24-hour time and digits follow the locale:

- English (India) groups numbers in lakhs (`12,34,567`), UK and US by
  thousands; India and UK write day before month, the US month first.
- Digits follow CLDR's default for the locale (Bengali digits for Bengali,
  Arabic-Indic for Arabic, Extended Arabic-Indic for Persian and Urdu,
  Latin elsewhere).
- The calendar follows the locale's CLDR default: Buddhist years for Thai,
  Solar Hijri for Persian, Gregorian elsewhere. A setting to always use
  Gregorian comes with the date format settings later.
- The first day of the week (search's date picker) follows the locale.
- Relative times ("2 hours ago") and sizes ("12 KB") are Fluent messages
  with plural forms and a formatted number.
- Folder and label names sort with `icu_collator` in the chosen language.

The daemon does not format dates, so it links only Fluent (it was on a
tight budget): the counts in its notifications and tray tooltip are written in
Western digits whatever the language. Its text is in
`i18n/<tag>/katna-daemon/`, embedded by its own build script; it applies
`general.language` at start and again when Katna Mail asks it to reload
the settings, rebuilding the tray menu.

**Text shaping and fonts.** The vendored GPUI draws text with
`cosmic-text`, which shapes every script with `harfrust` (HarfBuzz's
rules) and reorders mixed-direction lines with `unicode-bidi`, so
Devanagari, Bengali, Tamil and the other Indic scripts, Thai, Khmer,
Burmese, Lao, Tibetan (Dzongkha), Ethiopic (Amharic), Arabic and Hebrew
join and reorder correctly when a font for the script is installed. Katna
does not bundle fonts: `cosmic-text` falls back per script to the Noto
family (`Noto Sans Bengali`, `Noto Serif Tibetan`, `Noto Sans Ethiopic`,
`Noto Sans CJK SC/TC/JP/KR`, …), and packages recommend `noto-fonts` and
`noto-fonts-cjk` (Arch optdepends; Ubuntu Recommends). Two fixes are
needed in the vendored GPUI:

- **Line breaking.** GPUI wraps at spaces for a short list of scripts and
  anywhere at all for the rest, which splits Hindi, Arabic or Tamil words
  in the middle, even inside a letter cluster. The patch breaks only at
  Unicode line-break opportunities (UAX #14) and never inside a grapheme
  cluster; Thai, Lao, Khmer and Burmese, which put no spaces between
  words, break with `icu_segmenter`'s dictionaries; Chinese and Japanese
  may break between characters but not before closing punctuation.
- **Han glyphs.** `cosmic-text` picks Chinese, Japanese or Korean forms of
  shared characters from the system locale at start; the patch passes
  Katna's language instead, so Japanese UI text uses Japanese forms.

The picker needs each language's own name to render in any language, so
it is the first place these are checked. Urdu uses the Naskh style of
Noto Sans Arabic; Nastaliq (`Noto Nastaliq Urdu`) is used when installed.

**Right to left.** GPUI has no layout direction, and converting every
`flex_row`, padding and position in the code by hand would touch every
file. Instead Katna vendors `gpui-pre` (like `gpui-pre-linux` and
`gpui-pre-wgpu`, with the patch described in its `KATNA.md`) and adds a
window-wide layout direction:

- In right-to-left windows, every element's horizontal position is
  mirrored inside its parent when layout bounds are computed
  (`x' = parent width − x − width`). Rows, paddings, margins, absolute
  positions and the springs that move them all mirror at once, and hit
  testing follows because it uses the same bounds.
- Text alignment reads as start and end: `text_left` means start (right
  in RTL), `text_right` means end.
- A subtree can opt out and keep left-to-right (`.layout_ltr()`), for
  things that are not text: the attachment viewer's pages and pictures,
  media controls, the colour picker, the mail body (below), phone numbers
  and code.
- Icons that point somewhere are drawn mirrored: back, forward, reply,
  reply all, forward, send, undo, redo, the panel icon, list navigation
  chevrons, the Compose FAB's position. Icons that do not (search, star,
  gear, check, clock, logos) are not.
- Horizontal scrolling starts at the right; the phone's drawer slides in
  from the right and a conversation from the left; menus open towards the
  start edge.
- Left and Right arrow keys follow what is on screen; Newer and Older keep
  their meaning. J, K and the other letter shortcuts are unchanged.
- Carets and selection in text boxes follow the visual order of mixed
  text (GPUI assumes glyphs run left to right in index order; the patch
  maps positions through the bidi runs).

**Mail content in other scripts.**

- Charsets: `mail-parser` decodes with `encoding_rs`, which covers the
  legacy charsets these languages used (ISO-2022-JP, Shift_JIS, EUC-KR,
  GB18030, Big5, windows-874/TIS-620, windows-1256, ISO-8859-6 and -8,
  KOI8-R/U). Katna always sends UTF-8.
- Direction is the message's, not the UI's: an HTML body honours `dir`
  and `dir="auto"`; a plain-text body sets each paragraph's direction from
  its first strong letter. The subject, sender and snippet in the list do
  the same per line, aligned to the UI's start edge, so Arabic mail reads
  right to left in an English UI and English mail left to right in an
  Arabic one.
- Compose: each paragraph's direction follows what is typed (first strong
  letter); the format bar has "Right to left" and "Left to right" buttons
  when an RTL language is the UI or keyboard layout, and sent HTML carries
  `dir`. The quote header ("On 27 Sep 2026, Rahim wrote:") and the
  forwarded-message header are written in the UI language with its date
  format; `Re:` and `Fwd:` stay as they are (they are protocol, and
  localized prefixes are already recognised for threading,
  `katna_core::subject`).
- Input methods: typing Chinese, Japanese, Korean and Indic scripts goes
  through the desktop's input method (IBus or Fcitx5) via GPUI's
  `text-input-v3` (Wayland) and XIM (X11) support; every Katna text box
  implements GPUI's input handler, so composition works everywhere text
  is typed.
- Keyboard shortcuts: on a non-Latin keyboard layout (Russian, Arabic,
  Hebrew, …) letter shortcuts use the key in the same place on the US
  layout, as Gmail does, so J and K still step.
- Search: words in Thai, Lao, Khmer and Burmese are split with the same
  segmenter as line breaking (§7), and Chinese and Japanese with the
  optional dictionaries.

**Translations.** English is the source. All 48 other translations are
first drafted by AI, and are marked as such until a native speaker has
reviewed them:

- `i18n/languages.toml` lists each entry: tag, own name, English name,
  flag, direction, formats locale, and `status = "machine"` or
  `"reviewed"` with the reviewers' names. Each drafted file starts with a
  comment saying it is machine-drafted and needs review.
- In the app, a machine-drafted language shows "Translated by machine.
  Help improve it" at the foot of the picker, linking to the repository's
  translation guide.
- Corrections: now, anyone can edit `i18n/<tag>/*.ftl` on GitHub in the
  browser and open a pull request, or file a "Translation correction"
  issue (template with language, where, current and better text); the
  local override folder above lets them check it in the app first. Later,
  hosted Weblate (free for libre projects; the owner applies) takes over
  the same files, with English as the source and CI unchanged.
- New English strings: a pull request that adds UI text adds it to the
  English file only; a follow-up drafts the other languages, so no
  feature waits on 48 translations. Missing messages show in English
  meanwhile, and CI reports each language's coverage.
- Pseudo-locales for testing: `KATNA_LANGUAGE=qps-ploc` shows every
  string accented and 40 % longer (finds hard-coded text and clipped
  labels); `qps-plocm` also mirrors the layout (finds RTL bugs without
  reading Arabic).

**Size.** Fluent about 0.3 MB; compressed translations about 2 MB; ICU4X
code and baked data for 51 locales, measured when added (expected a few
MB). The app is about 56 MB of its 100 MB budget, so this fits; the daemon
adds only Fluent and its own strings.

### 13.11 Katna Notes

The owner asked for Notes on 2026-09-29, study first
(`/mnt/project-files/research/notes/katna-notes-study.html`, published as
an artifact). The study's recommended options were taken while he was
away; he can still change them.

- **Look.** Google Keep: the Notes page of the main window (the rail's
  Notes button) has a side list (Notes, Archive, Trash), a "Take a note…"
  bar with a New list button, and a board of 240 px cards in columns,
  each card going to the shortest column by its drawn height, so no row
  lines up and no gap opens. Pinned notes come first under "Pinned".
  Dragging a card moves it among the notes of its section: the others
  glide aside as it passes (they jump when motion is reduced), and
  dropping it keeps the order (`OrderNotes`, with Ctrl+Z; the order stays
  on this computer). Cards glide to their new places whenever the board
  changes. Cards take Keep's eleven colors (light and dark). Hovering a
  card shows its pin and its Archive and Delete buttons; checklist items
  tick right on the card, and ticked ones fold into "+ N ticked items".
  A card opens as a note over the dimmed board (600 px, 15 px corners):
  title, text, pin, colors, Show/Hide checkboxes, Archive, Delete, Close.
  It saves as it is typed (0.7 s after the last key) and on Close or Esc;
  an empty new note is discarded. The top bar's search box searches notes
  (every word in the title, text or labels) while the page is open, and
  mail again after.
- **Checklists.** A note is plain text. Lines starting "☐ " or "☑ " are
  checklist items, the way a note in an IMAP Notes folder carries them,
  so Apple Notes shows them as readable lines.
- **Trash.** Deleting moves a note to Trash for 7 days (Keep's time), with
  Undo; the daemon deletes older ones whenever notes are trashed. Trash
  has Restore, Delete forever and Empty Trash.
- **Store.** `note` in `pim.db` (schema v7, `pim_v7.sql`): account (NULL
  for this computer only), a UUID, title, body, color, pinned, archived,
  labels (JSON), link (the `Message-ID` of a mail it is about), position,
  times, `trashed_at`, `server_uid` and `dirty`. `note_gone` lists server
  copies still to delete. Changes are journaled as `note`.
- **D-Bus.** `SaveNote(NoteItem)` (ID 0 makes a new one on top) returns
  the ID; `TrashNotes(ids, trashed)`; `DeleteNotes(ids)`;
  `OrderNotes(ids)` puts notes in that order in the places they had;
  `RelabelNotes(ids, old, new)` renames, deletes or adds a label (at
  most 50 characters). Apps read notes from the store.
- **Sync.** A note of a mail account is kept in that account's `Notes`
  folder in Apple's format (`katna_sync::notes`): one message per note
  with `X-Uniform-Type-Identifier: com.apple.mail-note`,
  `X-Universally-Unique-Identifier`, the title as Subject and the text as
  simple HTML, one `<div>` per line with the title first, stored as
  \Seen so the folder shows no unread count. Katna's extras travel in
  `X-Katna-Title`, `X-Katna-Color`, `X-Katna-Pinned`,
  `X-Katna-Archived`, `X-Katna-Labels` (base64 JSON) and `X-Katna-Link`,
  which other apps ignore. Editing appends the new copy and deletes the
  old one; trashing, deleting or moving a note to another place deletes
  the server copy (`note_gone`). Coming in, an unknown message becomes a
  note (by its UUID), and a note whose message is gone from the folder is
  deleted here, unless it was changed here since, which wins. The daemon
  syncs an account 3 s after a note of it changes and looks at every
  IMAP account every 10 minutes; the folder is made only when there is a
  note to put in it. The daemon sends `MailChanged` when notes came in,
  and the Notes page reloads.
- **Where a note is kept.** A new note goes to the account whose mail
  was open (else the first IMAP account); POP and Graph-only accounts
  have no folders. The note's footer ("Edited … · Dev Dovecot") opens a
  row of accounts plus "On this computer" to move it. Google Keep
  (Workspace only) and OneNote (work and school accounts only) have no
  API for personal accounts, so the Notes folder is the mail service's
  own feature that every IMAP account has.
- **On a mail.** Add a note (the mail's right-click and ⋮ menus) opens a
  new note over the mail, titled with the conversation's subject and
  keeping its newest message's `Message-ID` in `link`. The notes whose
  `link` is any message of the open conversation show as small cards
  under its subject, each opening over the mail, with "Add a note" after
  them. A note with a link has a Mail chip (on its card and in the open
  note) that opens the mail again. Gmail has no key for Keep, so there
  is none.
- **Labels.** As in Keep. The label button on an open note opens "Label
  note": a box to find or make a label over the labels to tick; Enter
  makes the one typed. A note shows its labels as chips (× takes one off
  in the open note; a chip on a card opens that label's board). The side
  list lists every label between Notes and Edit labels, and a label's
  board shows its notes, with "Take a note…" making notes that have it.
  Edit labels renames labels (Enter or Done) and deletes them, with Undo;
  a label lives only on its notes, so there is no empty label, and a
  label's board goes back to Notes once no note has it. Labels travel in
  `X-Katna-Labels`; there is no schema change.
- **Meeting notes.** As Google Calendar's "Take meeting notes": an
  event's card on the Calendar page lists the notes about it and has
  Take meeting notes, which opens a new note over the Calendar titled
  "<event> · <day>" and started with "Attendees: …", "Notes" and
  "Action items" with a checklist line; left as it is, it is not kept.
  Its `link` is `event:<start>:<UID>`, so each occurrence of a repeating
  event has its own notes. Such a note has an Event chip that opens the
  Calendar's Day view on that day.
- **Tasks from checklist lines.** "Make it a task" in a note's toolbar,
  shown while the cursor is on an unticked checklist line of a saved note,
  adds the line to the default task list. The task keeps `note:<id>` where
  a task made from a mail keeps its Message-ID (the field stays on this
  computer), so its Note chip on the Tasks page opens the note; Undo takes
  the task back.
- **Formatting.** Keep's set: Heading 1, Heading 2 and Normal text for a
  line, Bold, Italic and Underline, Clear formatting (Formatting in the
  note's toolbar; Ctrl+B, I and U). The note is edited with the compose
  window's `RichEditor`. `body` stays the plain text, one line per
  paragraph, for the board, search and checklists; `html` (pim.db v11)
  holds the same paragraphs formatted, empty while nothing is. The Notes
  folder copy carries that HTML after the title line, and formatting from
  another app (an iPhone's bold or headings) is kept when read. Cards draw
  the formatting when the HTML's paragraphs line up with `body`.
- **Later.** Pictures.

## 14. D-Bus API (`katna-dbus`)

### 14.1 Interface `in.invenia.katna.Pim1` (object `/in/invenia/katna/Pim1`, bus name `in.invenia.katna.Daemon`)

Sketch — versioned by the interface name; breaking changes create `Pim2`.

| Kind | Members |
|---|---|
| Mail commands | `OpenMessage(id)`, `FetchBody(id)`, `SetFlags(ids, flags)`, `Move(ids, folder)`, `Archive(ids)`, `QueueSend(draft)`, `UndoSend(id)`, `ReplyAll(id, text)` |
| Search | `Search(query, limit) → results` (used by KRunner, GNOME search, apps) |
| Calendar | `EventsInRange(start, end) → events`, `CreateEvent(ical)`, `UpdateEvent(uid, ical)`, `DeleteEvent(uid)` |
| Contacts / orgs | `FindContacts(text)`, `Organizations()` |
| Sync | `SyncNow(account?)`, `SyncFolder(folder)` (only that folder, for a folder's "Check for new mail"), `SetForegroundFolders(ids)`, `Status() → per-account state` |
| Signals | `MessagesChanged(ids)`, `FoldersChanged`, `EventsChanged(range)`, `SyncStatusChanged`, `UnreadCountChanged(n)` |

Implemented so far (`katna_dbus::PimProxy`): `Accounts() → a(xssssxs)`
(id, kind, name, address, state, detail, last sync, OAuth2 provider),
`DiscoverAccount(address) → (account, POP3 server, source, provider,
password works)` (an empty POP3 host when there is none),
`SignIn(provider, id, address) → id` (OAuth2 in the browser; adds the
account, or signs one in again), `CancelSignIn() → b`, `AddImapAccount(account,
password) → id`, `AddPop3Account(account, password) → id` (with
leave-on-server, days to keep, and delete-with-local), `SetPop3Keep(id,
leave on server, days, delete with local)`,
`SetPassword(id, password)`, `RenameAccount(id, name)` (an empty name
goes back to the name the account's own sent mail uses, which a name-less
account also takes after its first sync), `RemoveAccount(id) → b`,
`DeleteAllData()` (stops every account, deletes every saved password,
the data directory, the cache and `config.toml`, then the daemon exits;
the next call starts a new one), `ResetCache() → (tt)` (messages that lost
their body, bytes deleted; see Settings above), `SyncNow(id)` (0 for every account), `FetchBody(message)`,
`SetFlags(ax messages, as add, as remove)` (flag names `seen`, `answered`,
`flagged`, `draft`, `forwarded`, `important`), `SetPinned(ax messages, b
on)` (local only; more than ten pinned conversations is an error),
`MoveMessages(ax, folder)`,
`Snooze(ax messages, x until)`, `Unsnooze(ax messages)` and
`SetFollowUp(x outbox, x after)` (§10.1),
`DeleteMessages(ax)`, `ArchiveMessages(ax)`, `QueueSend(x account, ay
message, u delay) → id`, `UndoSend(id) → b`, `DiscardSend(id) → b`,
`Outbox() → a(xxxsxss)` (id, account, message, subject, send at, state,
detail; states in `katna_dbus::send_state`), `SaveTemplate((xssssa(ssay)))
→ x`, `RenameTemplate(id, name) → b`, `DeleteTemplate(id) → b` (mail
templates in `pim.db`; apps read them from the store), `FetchImage(url) → ay` and
`SenderPicture(address) → ay` (images for the reading pane, §12),
`SetCalendarHidden(x id, b hidden)`, `CalendarStatus() → a(xss)`
(account, state, detail; §18) and `EditEvent(s json) → x` (a
`katna_store::calendar::EventChange` as JSON, tagged by `op`: `add`,
`change`, `delete`, `restore` or `respond`; written to `pim.db` at once,
then sent to the calendar's service; returns the event row added or
changed, or 0; `InvalidArgs` for a calendar that can't be changed; §18),
and the
signals
`AccountsChanged`, `SyncStatusChanged(id)`, `MailChanged(id)`,
`OutboxChanged(id)` and `CalendarChanged()`. `MailChanged` carries the
account, not message IDs: clients read the change journal. Errors use the
standard names `org.freedesktop.DBus.Error.AuthFailed`, `InvalidArgs`,
`UnknownObject` and `Failed`. zbus needs the interface name as a literal, so
`katna_core::with_dbus_names!` hands it to the attribute macros.

### 14.2 Rules

- Commands are asynchronous and idempotent where possible (retries are safe).
- Change signals carry IDs only; clients re-read from SQLite.
- Access is limited to the user's session bus. Inside Flatpak, only the
  Katna apps own/talk to `in.invenia.katna.*` names.
- Each app also implements `org.freedesktop.Application` (`Activate`,
  `ActivateAction`, `Open`) so notifications and KRunner can open a
  specific message or event, with an activation token.

## 15. Desktop integration

### 15.1 Notifications

Implemented by the daemon on `org.freedesktop.Notifications` (`zbus`).

| Interaction | Behavior |
|---|---|
| **Click** (`default` action) | Opens Katna Mail on that message (`org.freedesktop.Application.ActivateAction("open-message", id)`), starting the app if needed. The **activation token** from the notification (`ActivationToken` signal) is passed to the app so Wayland focuses the window instead of only flashing it. |
| **Reply all** | On Plasma: action id `inline-reply` with hints `x-kde-reply-placeholder-text` ("Reply to all…") and `x-kde-reply-submit-button-text` ("Send"). The `NotificationReplied(id, text)` signal gives the text; the daemon builds the reply-all (recipients = From + To + Cc minus own addresses, `Re:` subject, `In-Reply-To`/`References`, quoted original), queues it with the undo delay, and shows "Reply sent · Undo". Elsewhere: a normal "Reply all" button that opens the quick-reply window (§13.4). Support is detected at runtime with `GetCapabilities` (`inline-reply`). |
| **Archive / Mark read** | Buttons handled by the daemon without opening the app. |

Content and behavior:

- Hints: `desktop-entry`, `category=email.arrived`, `image-data` (sender
  avatar or organization logo), the Sounds setting's sound (§13 Settings;
  Katna plays it itself on Linux and sends `suppress-sound`),
  `x-kde-origin-name` (account name).
- **Grouping:** bursts become one notification ("5 new emails from Acme").
- **Filtering:** notify for Inbox / important categories only by default;
  per-organization policy (always / never / normal); respect Do Not Disturb
  (handled by the notification server).
- **Lifecycle:** notifications are closed (`CloseNotification`) when the
  message is read, archived or deleted anywhere, including other devices.
- Calendar alarms and snooze/reminder expiries use the same system.
- Flatpak: verify inline-reply support through the notification portal;
  otherwise request `--talk-name=org.freedesktop.Notifications`.
- KDE Connect mirrors these notifications to the user's phone automatically.

Built so far (`katna-notify`, `apps/katna-daemon/src/notify.rs`):

- After each sync, unread mail that rings (§15.1.1; by default mail that
  reached an account's inbox, Primary tab or not classified yet, and is not
  muted) since the daemon last looked, and dated within the last two days,
  becomes one notification per account and sync. One message
  shows sender, subject and the start of its text; more show "N new emails"
  with up to four "Sender: Subject" lines.
- Mail already stored when the daemon starts, and a new account's first
  sync, are not news.
- Buttons: Open (click), Reply all (one message only), Mark as read / Mark
  all as read, Archive. Open calls `ActivateAction("open-message", [id])`
  on the app's `org.freedesktop.Application` object
  (`/in/invenia/katna/Mail`) with the activation token; Reply all calls
  `reply-all`, which opens the message with an inline reply to all. When
  the app does not answer, the daemon starts `katna-mail --message ID` (or
  `--reply-all ID`) with `XDG_ACTIVATION_TOKEN`. The app looks for the
  message in every inbox tab. The Plasma inline-reply field in the table
  above is not built yet.
- A notification closes when all its mail is read or out of the inbox, from
  a sync or from a change made in the app, and a new-mail notification
  also when its mail is muted or its folder stops notifying.
- Setting `notifications.new_mail` (default on); `ReloadConfig` applies it.
- Not yet: inline reply, sender pictures (`image-data`), per-organization
  policy.
- **Event reminders** (`apps/katna-daemon/src/daemon/alarms.rs`): each
  reminder of an event in a shown calendar (not cancelled, not declined)
  becomes a "Katna Calendar" notification at its time: the title, how soon
  it starts ("In 10 minutes"; the daemon has no ICU, so no clock times)
  and the place, with Open (the Calendar page), Join (the event's
  `https://` video link, in the browser) and Snooze 5 min.
  `category=x-katna.event`, the Reminders sound (Alarm unless picked), no timeout.
  One task reads the next eight days of events, sleeps until the next
  reminder (at most a minute, so edits count) and keeps up to when it
  looked in `pim.db` meta (`calendar`/`alarms`), so a restart repeats
  none; reminders missed while the computer was off show only when they
  fell due in the last ten minutes. Snoozes live in memory. Tasks'
  reminders come through the same loop (§18.1).

#### 15.1.1 What rings and counts: bells and mutes

One rule decides both what notifies and what the taskbar and tray count
(`katna-store` `alerts.rs`), so they never disagree (before October 2026
the badge counted the whole Inbox, Promotions included, while only Primary
notified).

- **Bells.** Each folder, and each inbox tab, has a bell with two switches:
  Notify (new mail shows a notification) and Count (unread mail counts on
  the taskbar and tray). Default: an inbox's Primary tab (and unclassified
  mail) both on, everything else off. `mail.db` `folder_alert` keeps only
  bells that differ from the default (`category` 0 for a whole folder).
  Mail rings when any folder it is in rings, so a Gmail message labelled
  Clients rings when Clients' bell is on.
- **Mutes** (`mail.db` `mute`): an account, a folder, a conversation or a
  sender (an address, every account), for a while or until unmuted. A
  muted thing's mail still arrives and stays unread; it only never
  notifies and is not counted. Mutes win over bells. Snooze and follow-up
  reminders still show, because they were asked for. A muted sender
  carries a crossed bell after their name wherever people show (mail
  list, reader, contact card, Contacts, recipient chips and suggestions,
  Files, event guests, Activity; `MailWindow::muted_mark`), so quiet mail
  is never a mystery. Refusing a muted sender's mail outright is a later
  step.
- **Mail services first.** A conversation muted for good is muted at the
  service too: Gmail's mute (the `\Muted` label in `X-GM-LABELS`, set on
  every message of the conversation; Gmail then keeps later replies out of
  the Inbox), elsewhere the `$muted` keyword (RFC 9979). Both read back as
  `MessageFlags::MUTED`. After each sync the daemon follows the service:
  a conversation with a muted message is muted (`server` = 1), and one
  muted by the service whose messages all lost the flag is unmuted.
  Katna's own mute of a conversation on a server that keeps no keywords
  stays Katna's (`server` = 0). Microsoft's Ignore deletes mail, so it is
  not used. Bells, folder, account and sender mutes are Katna's own: no
  service keeps them for other apps.
- **Daemon** (`daemon/mutes.rs`): D-Bus `Mute(kind, id, address, until)`,
  `Unmute` and `SetBell(folder, category, notify, count)`; each closes
  notifications that no longer ring, sends `MailChanged` for every account
  (the apps and the taskbar count look again) and wakes the scheduler,
  which drops timed mutes when they end. A muted conversation follows
  thread merges.

### 15.2 Taskbar, tray and global menu

The count and the tray live in `katna-daemon`, so they stay while the app
is closed; the protocol code is in `katna-platform` (`launcher`, `tray`,
`dbusmenu`, `icon`), written on zbus rather than with `ksni`.

- **Unread count** on Katna Mail's taskbar or dock icon:
  `com.canonical.Unity.LauncherEntry` `Update` signals for
  `application://in.invenia.katna.Mail.desktop` from
  `/in/invenia/katna/Daemon/LauncherEntry`. The number is the unread
  messages that count (§15.1.1; by default every account's Inbox, Primary
  tab, less anything muted), recounted half a second after mail changes. Plasma's task manager shows
  it; on GNOME, Ubuntu Dock, Dash to Dock and Dash to Panel do (the stock
  GNOME dash shows no counts). Setting `general.unread_badge` (default on).
- **Tray icon**: a StatusNotifierItem under its own name
  (`org.kde.StatusNotifierItem-PID-N`), registered with
  `org.kde.StatusNotifierWatcher` again whenever the watcher restarts.
  Plasma shows it natively; GNOME needs the AppIndicator extension (on by
  default on Ubuntu). `general.tray_style` picks Colour or Monochrome
  (Settings → General → Desktop → "Tray icon in color"; default Monochrome
  on Linux, Colour on Windows, 2026-10-01). Without unread mail the panel
  draws the named icon: the one-colour k (`<mail app ID>-symbolic`), which
  it recolours, or the coloured app icon. With unread mail the icon is
  pixels, pre-rendered at each tray size (`crates/katna-platform/icons/`,
  from `packaging/icons/render.py`), with a red badge drawn in code with
  the count, `99+` above 99, since the protocol takes pixels and an SVG
  renderer would grow the daemon. Monochrome stays monochrome then: the
  disc in the panel's text colour with the k cut out (its whiteness in the
  coloured pixels), and only the badge is red. Panels can't be asked their
  colour, so it is inferred (`colors::panel_text`): Plasma's from the
  scheme's window text, white on GNOME and other panels, and on Windows
  from `SystemUsesLightTheme`; it is read again with each count. Left click raises the
  app, middle click starts a new message. The right-click menu
  (`com.canonical.dbusmenu`) has Open Inbox, New Message, Preferences and
  Quit. Quit closes the app and stops the daemon until the next login or
  until the app starts it again (D-Bus activation). Setting
  `general.show_in_tray` (default on; the older `tray_icon` key is ignored
  because versions without a tray saved it as `false`). Both switches are
  under Settings → General → Desktop; the app calls `ReloadConfig` after
  saving so the daemon applies them at once.
- **Single instance and actions**: Katna Mail owns `in.invenia.katna.Mail`
  and serves `org.freedesktop.Application` at `/in/invenia/katna/Mail` with
  the actions `open-inbox`, `compose`, `preferences`, `open-message` and
  `reply-all` (a message ID) and `quit` (`katna_dbus::app_action`). A second `katna-mail`
  hands its request to the first and exits. The tray, notifications and
  the desktop file use this: its actions New Message, Open Inbox and
  Preferences (right-click on the taskbar icon in Plasma and GNOME) run
  `katna-mail --compose`, `--inbox` and `--settings`. With `--data-dir` the
  app stands alone.
- **Default mail app**: the desktop file declares
  `MimeType=x-scheme-handler/mailto;` and `Exec=katna-mail %u`. A `mailto:`
  link (RFC 6068: to, cc, bcc, subject, body) opens a new message filled
  in; a running app gets it through `org.freedesktop.Application.Open`.
  Settings > General > Default mail app shows whether the desktop's
  `mimeapps.list` names Katna Mail for `x-scheme-handler/mailto` and can
  set it (`katna_platform::mimeapps`, in the user's `mimeapps.list` and any
  desktop-specific list that names another app). Plasma and GNOME read
  these files. Under Flatpak this needs the OpenURI portal instead (later).
- **Send with Katna Mail** in the file managers' right-click menus on files
  and folders runs `katna-mail --attach [--from ADDRESS] FILE…`: a new
  message with them attached, a folder as a zip of it; a running app gets
  it as the `attach` action, and files arriving within two seconds join the
  same message (Explorer starts one process per file). With several mail
  accounts the entry is a submenu of them (`katna_platform::file_menus`).
  Dolphin: the package's service menu in `/usr/share/kio/servicemenus`,
  and the daemon's copy with the submenu in the user's
  `~/.local/share/kio/servicemenus` (same name, so it wins) while there are
  several accounts. GNOME Files: a nautilus-python extension that reads
  `send-menu.json`, which the daemon writes in Katna's data folder. The
  daemon rewrites both at start and when accounts change.
- **KDE global menu**: the app serves its menu bar (File, Edit, View, Go,
  Message, Settings, Help) with `com.canonical.dbusmenu` at
  `/in/invenia/katna/Mail/MenuBar`, built from its GPUI actions and their
  key bindings; items for actions a build lacks are left out, and a click
  dispatches the action in the window. GPUI cannot announce a menu on
  Linux, so `vendor/gpui-pre-linux` patches its Linux backend
  (`[patch.crates-io]`, see `KATNA.md` there): `set_kde_appmenu` gives
  every normal window `org_kde_kwin_appmenu` on Wayland and the
  `_KDE_NET_WM_APPMENU_*` properties on X11. Plasma's Global Menu applet and
  the title-bar menu button then show it.

### 15.3 KRunner and GNOME Shell search

Served by the daemon, pure Rust, from the same search index.

- **KRunner:** implements `org.kde.krunner1` (`Match`, `Actions`, `Run`,
  `Teardown`). Metadata file in `share/krunner/dbusplugins/` names the
  daemon's D-Bus service, so KRunner activates it on demand. `Match` answers
  from tantivy in milliseconds.
- **GNOME:** implements `org.gnome.Shell.SearchProvider2`
  (`GetInitialResultSet`, `GetSubsearchResultSet`, `GetResultMetas`,
  `ActivateResult`, `LaunchSearch`) with an `.ini` in
  `share/gnome-shell/search-providers/`.

| Result type | Matches on | Actions |
|---|---|---|
| Contact | name, address, organization | Compose email, copy address, open contact |
| Email | subject, sender, text (confident matches only, or with a `mail:` prefix) | Open, reply all |
| Organization | name, alias | Open organization view |
| Event | title, location, details (the coming year) | Open its day in Calendar |
| Task | title, details (open tasks) | Open the task |

As built (`apps/katna-daemon/src/desktop_search.rs`): people come from the
addresses in the mail (the recipient-suggestion `ContactBook`, read in the
background 20 s after start and again when mail changed, at most every 10
minutes). Mail shows only when every word (three letters or more) starts a
word of its subject or sender, outside Trash and Spam, one message per
conversation; `mail:`, or a trigger word and a space (`k budget`;
`general.search_triggers`, "k" and "m" by default, set in Settings >
General and applied at `ReloadConfig`), runs the search box's query
instead. Enter on a
person writes to them (a `mailto:` link to Katna Mail); KRunner's buttons
are Reply all on mail, Copy address (through Klipper) and Find mail on
people. GNOME's "search in app" opens Katna Mail with the words in its
search box (app action `search`). Organization results come with Phase 2.
Answers take a few milliseconds on 60,000 messages.

Open tasks (§18.1) come up too, under Tasks, when every word starts a word
of their title or details: at most three, those due first first, between
people and mail, with the list they are in. Enter opens Katna Mail on the
Tasks page with the task's details (app action `open-page` with
`tasks:<id>`, which a task's reminder uses too).

Events come up the same way, under Events, by their title, place or
details: the next occurrence of each, from now to a year ahead, at most
three, soonest first, after tasks. The line under says how soon (Now,
Today, Tomorrow, In 3 days; the service formats no dates) and the place,
or else the calendar. Enter opens the Calendar on that day
(`calendar:YYYY-MM-DD`, as the clock does).

Flatpak: KRunner D-Bus runners are designed to work with sandboxed apps;
verify that Flatpak exports the `krunner/dbusplugins` file. Distro
packages install it directly.

### 15.4 Plasma calendar (clock) integration

Plasma's digital clock popup only displays events from calendar-events
plugins (Akonadi uses the "PIM Events" plugin from kdepim-addons). It has a
hidden **"Add…"** button, shown only when an events plugin is enabled **and**
a default `text/calendar` application exists; it launches that app without a
date. Events in its agenda have no click or right-click actions.

Katna integrates in three layers. All of them read the daemon's
`in.invenia.katna.Agenda1` (`crates/katna-dbus/src/agenda.rs`): events for
a range of days and tasks, which they add and tick off; `Changed` says to
read again. Events are those of the calendars Katna syncs; tasks are
those of every task list, synced with each account's own service
(§18.1), and a task added in the clock goes to the first account's
default list. `Open` shows an event (the Calendar on its day) or a task
(the Tasks page) in Katna Mail, and `NewEvent` starts an event on a day
there, both through `katna-mail --page` (`calendar:YYYY-MM-DD[:new]`).
`integrations/README.md` has the details.

**A. Calendar-events plugin (planned, `integrations/plasma-calendar-plugin`)**

- A small C++ `CalendarEvents::CalendarEventsPlugin` that reads `Events`
  over D-Bus and listens to `Changed`, so the stock clock shows Katna's
  events and their dots in the month.
- Katna Calendar registers as the `text/calendar` handler, so the stock
  clock shows **"Add…"** for Katna without any fork.
- Runs inside `plasmashell`: fully asynchronous, never blocks, minimal code
  (a crash here crashes the desktop shell). Comes with calendar sync, when
  there are events to show.

**B. Katna Digital Clock, a copy of the official clock (`integrations/plasma-clock`, built)**

- Source: `plasma-workspace/applets/digital-clock` at v6.7.5, its QML only
  (GPL-2.0-or-later and compatible licences). The first commit is the
  unchanged copy; Katna's edits are marked `// Katna:`.
- Plugin ID `in.invenia.katna.digitalclock` (`ids::CLOCK_APPLET_ID`),
  installed as a QML package: nothing to compile.
- Declares `X-Plasma-Provides: org.kde.plasma.time, org.kde.plasma.date`, so
  it appears in the clock's **"Show Alternatives"** menu (two-click switch).
- Imports Plasma's shared modules from the system instead of copying them:
  the calendar component (`org.kde.plasma.workspace.calendar`), the
  clock's helpers (`org.kde.plasma.private.digitalclock`) and D-Bus
  (`org.kde.plasma.workspace.dbus`). Identical look, upstream fixes; copied
  only if upstream changes break us. Upstream text keeps Plasma's
  translation domain.
- Katna features in new files (`KatnaAgenda.qml` for D-Bus, `KatnaTasks.qml`,
  `KatnaJoinButton.qml`), with minimal edits to upstream files:
  - a Tasks list under the day's events: add a task (due on the day
    picked, when that isn't today), tick one off;
  - click a Katna event to open Katna's Calendar on its day; a Join
    button for its video call;
  - **"Add…"** starts a new event in Katna's Calendar on the day picked,
    without needing a `text/calendar` app;
  - right-click a day in the month (`KatnaDayMenu.qml`): Add a Task for
    that day, Add an Event on it;
  - later: right-click edit, delete, drag to reschedule, organization
    badges and related emails.
- Still reads events through the plugin system (A), so holidays and other
  plugins keep working. The plugins' event data carries no ID in Plasma
  6.7, so a Katna event is matched by title and start.

**C. Upstream contributions to Plasma**

- "Add…" opens the calendar app on the selected date.
- Clicking an event opens it in the calendar app (the event data needs its
  ID for that).
- An optional plugin hook for "create/edit event" actions.

Each accepted change shrinks the fork and improves the stock clock.

**GNOME:** the top-bar calendar reads only Evolution Data Server. Instead
of an EDS backend (C), Katna's GNOME Shell extension
(`integrations/gnome-shell-extension`, UUID `ids::CLOCK_EXTENSION_UUID`,
GNOME 48 and later) wraps the date menu's event source so Katna's events
join EDS's, and adds a Tasks card under the day's events, in GNOME's own
styles. `katna-daemon` switches it on once, the first time GNOME Shell
knows it; after that turning it off is the user's choice.

### 15.5 Other integration

- Default handler for `mailto:` and `text/calendar` (`.ics`).
- Global shortcut "Compose new email" via the GlobalShortcuts portal.
- Dolphin service menu "Send as email attachment with Katna"
  (`share/kio/servicemenus/`, no code).
- Portal file chooser, color scheme, accent color (§13.2).

### 15.6 Non-Rust components

Plasma's extension points are C++/QML, so these are the only non-Rust parts:

| Component | Language | Size goal |
|---|---|---|
| Calendar-events plugin | C++ / Qt 6 | A few hundred lines |
| Katna Digital Clock (copy of Plasma's) | QML | Upstream code + small Katna additions |
| GNOME Shell extension | JavaScript | A few hundred lines |

They only display data and forward actions to `katna-daemon`; no business
logic lives there.

## 16. Katna Server (`server/katna-server`)

Optional. Self-hostable (container image) and offered as a hosted Pro service.

Where a feature lives (owner, 27 September 2026): first the mail
service's own feature when Katna can reach it over the protocols it speaks
(IMAP, SMTP, Sieve, CardDAV, later JMAP; for example SMTP FUTURERELEASE for
send later); otherwise locally in `katna-daemon`; Katna Server only for
what can work neither way (open and link tracking, translation, Katna
accounts). The server never holds mail logins or tokens; send later,
snooze and reminders without server support run while the computer is on.
Plan: `IMPLEMENTATION_PLAN.md` Phase 7.

| Function | Needs user mail credentials? |
|---|---|
| Open/link tracking + event stream | No |
| Metadata sync between devices (E2E-encrypted values) | No |
| Snooze / reminders while the machine is off (push notification or IMAP move) | IMAP move: yes (optional) |
| Send later while the machine is off | Yes: SMTP credentials or a send-only OAuth scope (`gmail.send`); opt-in per account |
| Katna Confidential (revocable / expiring mail via link) | No |
| Large-attachment links | No |

### 16.1 Tracking design

- **Open:** `<img alt="" src="https://<tracking-domain>/o/<id>.png">`,
  inserted before quoted text. `<id>` is a random 128-bit value; the server
  stores the mapping. No decodable data (Mailspring's base64-JSON token leaks
  the recipient and can be forged).
- **Links:** `https://<tracking-domain>/l/<id>/<n>`. The destination is stored
  on the server and the link answers with a plain `302`, so recipients
  never see a page in between. Anyone with a confirmed Katna account can
  store a destination, so the server can still be misused as a redirect
  (security audit, 28 September 2026); that is contained by the daily
  limits per account and by `KATNA_SERVER_BLOCKED_HOSTS`, hosts the server
  refuses to store or redirect to (decided 28 September 2026, over an
  interstitial page or signed URLs).
- **Event quality:** label Apple Mail Privacy Protection fetches as "maybe
  opened"; label clicks from security scanners (data-center IPs within
  seconds of delivery) as "scanner"; Gmail/Outlook proxies hide location.
- **Privacy:** the server stores random IDs and events only — no subject,
  no recipients, no content. The app keeps the ID → message mapping.
- **Deliverability:** dedicated tracking domain; custom domains for
  business users (`t.customer.com`).
- **Consent:** tracking, a read receipt and a delivery receipt are on by
  default for each new message and reply (the owner's choice, 2026-09-28)
  and turned off per message; without a Katna account mail goes out
  untracked.
- **Delivery receipts** need no Katna account: the daemon asks the
  account's SMTP server for a delivery status notification per recipient
  (RFC 3461: `RET=HDRS` on `MAIL FROM`, `NOTIFY=SUCCESS,FAILURE,DELAY` on
  each `RCPT TO`), and the server mails them back to the sender. Only
  servers that list `DSN` after EHLO offer them; Gmail does not.
  `ServerDeliveryReceipts(account)` tells compose, which greys the switch
  where they are not offered. The app asks with an
  `X-Katna-Delivery-Receipt` header that the daemon takes out when it
  queues the message and keeps as `outbox.delivery_receipt` (mail.db v9).
  With it, each `RCPT TO` carries `ORCPT=rfc822;<address>`, so a report
  names the address as written even after forwarding.
- **Ticks:** receipts that come back are read by the sync as their bodies
  download (`katna_import::report`): a DSN's recipients whose `Action` is
  `delivered`, `relayed` or `expanded`, and an MDN whose disposition is
  `displayed`, matched to the sent message by `Original-Message-ID` or the
  returned headers' `Message-ID`, and to the recipient by
  `Original-Recipient`, else `Final-Recipient`. They are kept per
  `(Message-ID, recipient)` in `mail.db`'s `receipt` table (v10), with
  the time the outbox sent it to each recipient and when a DSN said it
  bounced. The reading view shows a grey tick beside a recipient of the
  user's mail once delivered, two accent ticks once read (by a read
  receipt or an open seen by tracking; the tooltip says which) and a
  warning once it bounced. An eye left of the message's star (accent once
  anyone has opened it) opens, on hover or click, a popover with a notch
  pointing at it that lists only who opened the message or followed a
  link, and read receipts; with none, it says so. Where no delivery
  receipt comes (Gmail sends
  none), the grey tick appears half an hour after sending if no bounce
  came back, and its tooltip says that is what it means: only the sending
  server knows whether mail arrived, and relaying through Katna Server
  would fail SPF and DKIM and need the mail login. Receipt mail
  (`multipart/report`) stays in the mailbox, under Updates, so it raises
  no notification. Legal
  review is needed before selling in the EU (GDPR/ePrivacy). Read receipts
  (MDN) are offered as a consent-based alternative.
- Tracking events arrive at the daemon over the server's event stream and
  can raise notifications ("Acme opened *Proposal v2*").

**Implemented (server, `server/katna-server`):** axum + PostgreSQL behind
Caddy (TLS), shipped as `ghcr.io/quakestring/katna-server` with a compose
file; the owner runs it on his own server at `server.katna.invenia.in`
(`katna_core::ids::TRACKING_SERVER_URL`; September 2026). Unknown pixel IDs still get the picture; links redirect
only to targets stored with the ID (`http`/`https` only). Installs register
without an account and get a bearer token (stored hashed); limits are 10
new installs per address per hour (an IPv6 address counts by its /64) and,
per Katna account and day, 5000 tracked copies and 16 MiB of link targets
(at most 256 KiB per request; the targets are stored once per message, not
per copy). Each event is labelled `person`, `apple_proxy` (Apple's network or a
bare `Mozilla/5.0` agent) or `scanner` (`HEAD`, bot-like agents, opens
within 5 s or clicks within 30 s of sending); the address and user agent
are read for the label and never stored. Opens and clicks always get the
picture or the redirect, but only 300 per hour per client network (IPv4
address or IPv6 /64) and 20 per hour per tracking ID are recorded. Events
stream to the daemon as server-sent events numbered in order, resumed with
`Last-Event-ID`; an install may have 4 streams open, and a stream ends as
soon as its install is signed out, deleted or its password changed
elsewhere (and is checked every minute). In-memory limits hold at most
100,000 keys each and refuse new keys when full; a request waits at most
5 s for a database connection. Caddy adds HSTS, `nosniff`,
`Referrer-Policy: no-referrer`, `X-Frame-Options: DENY` and a strict CSP,
and caps request bodies at 1 MB.
Everything is deleted after 180 days, and an install can delete its data.
One server process (events are ordered within it). The API is in
`server/katna-server/README.md`.

**Implemented (daemon, `katna-sync::tracking`, `apps/katna-daemon/src/tracking.rs`):**
a message queued with tracking (D-Bus `QueueTrackedSend`) with at most 50
recipients (To, Cc and Bcc) goes out as one copy per recipient, each in
its own SMTP transaction with the headers unchanged. Each copy's HTML gets
the pixel before the first `<blockquote` and its links outside quotes
rewritten; plain-text parts are left alone. Mail with no HTML part has the
`http`/`https` addresses of its plain text (before the first `>` line)
rewritten instead: clicks are seen, opens are not, and plain text without
links goes out untracked. Signed or encrypted mail (sealed by the app
before queueing) is opened with the user's GnuPG at send time (the sender
is always among an encrypted message's recipients), tracked, and each copy
signed and/or encrypted again in the same standard, encrypted to its
recipient and the sender only; each such copy carries an
`X-Katna-Copy: <tracking ID>` header, since its links are out of sight.
One signature per copy means one passphrase or smartcard touch each where
the agent does not cache it. Opens of encrypted mail are rare: most
clients load no remote pictures in it. A refused recipient does not stop
the others and the send retries only the rest. Sent keeps one clean copy
(the sealed original); on Gmail the tracked copies Gmail filed are found
by `rfc822msgid:` plus a marker (the tracking server's address, or
`X-Katna-Copy:`) and moved to Trash and deleted there (op
`PurgeTracked`). Anything that stops tracking (no server, server error,
too many recipients, a sealed message that can't be opened) sends the
message once, untracked. The recipient mapping and events live in
`pim.db` (schema v3). Tracking uses the Katna account token
(`katna_account::Session::token`, §16.2) and server (`server_url`); the
server takes it only while this computer is signed in to an account with a
confirmed address, and otherwise the message goes out untracked. The first
open or click by a person raises a notification whose Open shows the Sent
copy.

### 16.2 Katna accounts

Every server feature needs a **Katna account**, like a Mailspring ID
(decided September 2026). It is an email address and a password of its
own on Katna Server; mail logins never go to the server.

- **Server:** accounts with Argon2id password hashes; a six-digit code
  mailed through an SMTP relay the owner sets (`KATNA_SERVER_SMTP_URL`;
  without one the server does not start unless `KATNA_SERVER_DEV_MAILER=log`
  asks for codes in the log, for local testing) confirms the address and
  resets a forgotten password (30 minutes, 5 wrong tries, stored hashed;
  and at most 10 wrong tries per account in 24 hours over all its codes,
  counted in PostgreSQL so new codes and restarts do not reset it).
  `reset` answers `202` at once whether or not the address has an account
  and mails afterwards; `reset/confirm` answers an unknown address like a
  wrong code. Sign-up still answers `409` for a taken address (a notice to
  the owner instead is not built yet). Argon2 hashing runs one per CPU at
  a time (at least two). An install signed in to an account is one
  of its **devices**; any device can sign the others out, and changing or
  resetting the password signs them out. Feature routes take the
  `SignedIn` extractor, which needs a confirmed address. Unconfirmed
  accounts go after a week; deleting an account deletes its devices and
  their data. No plans or payments yet.
- **Daemon:** `katna_account::Session` registers the install once, signs
  in and out, and keeps the token and account address in the Secret
  Service (`Secrets::server_token`). Other server features take their token
  from `Session::token`. D-Bus: `KatnaAccount`, `KatnaSignUp`,
  `KatnaSignIn`, `KatnaVerify`, `KatnaResendCode`, `KatnaSignOut`,
  `KatnaDevices`, `KatnaSignOutDevice`, `KatnaChangePassword`,
  `KatnaResetPassword`, `KatnaConfirmReset`, `KatnaDeleteAccount`, signal
  `KatnaAccountChanged`; errors carry `katna_dbus::katna_error` names.
- **App:** Settings > Subscription. Features check
  `MailWindow::katna_signed_in` and show `katna_sign_in_needed` ("Sign in
  to use this") when not.

### 16.3 Stack

`axum` + PostgreSQL; WebSocket/SSE delta stream to `katna-daemon`; a
scheduler for server-side actions; shared crates with the apps where useful.

### 16.4 Automatic translation

Katna Mail offers to translate a message that is not in the reading
language (plan 7.8; decided 27 September 2026: LibreTranslate on the
owner's server, over on-device models or DeepL).

- **Server:** LibreTranslate (AGPL-3.0, upstream image, unmodified) runs as
  its own container beside `katna-server`, on the compose file's internal
  network only; which language models load is set there
  (`LT_LOAD_ONLY`). `katna-server` passes `GET /api/v1/languages` and
  `POST /api/v1/translate` / `/api/v1/detect` through for computers signed
  in to a Katna account with a confirmed address (§16.2), with a daily
  limit per account and at most `KATNA_SERVER_TRANSLATE_CONCURRENCY` (8)
  requests passed on at once (more are answered 503), and logs and keeps
  neither the text nor the translation. LibreTranslate shares a network
  only with `katna-server`, never with PostgreSQL, and its image is
  pinned to a release.
- **Daemon:** `Translate(message, text, source, target)` on D-Bus. Katna
  Mail finds the message's language on this computer (`katna-translate`,
  whatlang; in the app, as its models would crowd the daemon's size
  budget) and sends its plain text (HTML made plain, quotes and signature
  kept; never attachments, headers or addresses). Mail already in the
  reading language is never sent, and the daemon refuses it too. It sends
  the text in
  pieces of at most 4000 characters (40,000 in all) over rustls to
  `katna_core::ids::TRACKING_SERVER_URL` (empty turns translation off),
  with the Katna account's token (`katna_account::Session::token`), and
  keeps the translation in `mail.db` (`translation`, keyed by message,
  target and a hash of the text). Reset cache forgets them. Encrypted mail
  is never offered for translation.
- **App:** a bar above a message in another language: "Translate to
  <reading language>", then "Show original"; while signed out it says to
  sign in to a Katna account, with a button to Settings > Subscription.
  Settings > General >
  Translation: offer translations (on), the reading language (the UI
  language by default), languages always translated (none by default, one
  click from the bar) and languages never offered. The Settings text says
  the mail's text goes to Katna's server.

### 16.5 Writing help with AI

Katna Mail rephrases the text the user selects in a message and can finish
the sentence being written (decided 1 October 2026: Katna AI on Katna
Server and the user's own key, both; Gemini 3.5 Flash-Lite by default; Google closed 2.5 Flash-Lite to new keys).

- **Shared crate:** `katna-ai` (no network, no GPUI) holds the prompts
  (`prompt`: the tones Clearer, Shorter, Friendlier, Formal, Fix grammar,
  Longer and the user's own instruction; size limits; cleaning the
  answer), the services a key can be brought for and the one HTTPS
  request each takes (`provider`: Gemini, OpenAI, Claude, Mistral,
  DeepSeek, OpenRouter, and Other for anything speaking OpenAI's API, such
  as Ollama or LM Studio on `localhost`), and what the daemon and Katna
  Server say to each other (`wire`). Katna Server uses the same crate.
- **Settings** (`[ai]` in `config.toml`): `source` = `katna` (default),
  `own` or `off`; `provider`, `model` (empty for the service's usual one)
  and `address` (Other only); `autocomplete` (off) and
  `autocomplete_answered` (off); `encrypted` (on: Rephrase is offered in
  encrypted mail, asking each time). The key of the user's own service is
  in the Secret Service (`ai-key`), saved and removed through the daemon
  (`SetAiKey`, `AiKeySaved`), never in the settings file.
- **Daemon:** `AiRephrase(text, tone, instruction)` and
  `AiComplete(before, answered)` on D-Bus, read the settings per call and
  send over rustls either to Katna Server (`POST /api/v1/ai/rephrase`,
  `/api/v1/ai/complete`, with the Katna account's token; 401/403 = sign
  in, 402 = the free month is over, 429 = over a limit) or to the user's
  service with the key. They answer the text, the plan (`trial` with days
  left, `paid`, `own`) and a problem name (`katna_ai::wire::problem`).
  Nothing is logged but that it happened.
- **App:** selecting text in the message's own paragraphs (not the quote,
  signature, tables or pictures) shows a sparkle by its end; it, the
  sparkle beside Formatting in the compose bar (and in the chat view's
  reply box) or Ctrl+J opens the Rephrase card, for the selection or,
  with nothing selected, for all those paragraphs: tones, a preview, Replace (one undo step, a
  snackbar with Undo), Try again, Add below, Copy, and who answered.
  Encrypted mail asks before sending the selection, once per message.
  Longer suggestions use the grey writing suggestion and its Tab
  (`katna_ui::rich::Complete`), after a 600 ms pause at the end of a paragraph, ending in
  a small "✦ Tab" key; never for encrypted mail. They ask for as little
  thinking as the model allows (`Prompt::quick`: Gemini 3 Flash models
  `thinkingLevel: minimal`). Settings lists the models the service
  offers to the key (`AiModels` over D-Bus) under the editable Model
  field, filtered as the user types.
- **Katna AI** (Katna Server, `server/katna-server/src/ai.rs`): for
  confirmed Katna accounts, 30 days free from the first use, then $5 a
  month through Razorpay Subscriptions (to come; until then the server
  answers 402 after the free month). The server builds the prompt with
  `katna_ai::prompt` from the request, so a client cannot send the
  service anything else, and asks the service set in
  `KATNA_SERVER_AI_PROVIDER`/`_MODEL`/`_KEY` (Gemini 3.5 Flash-Lite by
  default), then `KATNA_SERVER_AI_FALLBACK_*` when that fails. It counts
  each answer's cost from the tokens the service reports at the prices
  set (`_PRICE_IN_USD`, `_PRICE_OUT_USD`) per account and calendar month
  (UTC): an account stops at `_ACCOUNT_CAP_USD` (1.00) and everyone at
  `_BUDGET_USD` (50; 0 is off), with 300 requests an hour per account.
  Text and answers are neither logged nor kept. Keys and the Razorpay
  secrets come from environment variables only.
- **Admin page** (`/admin`, `server/katna-server/src/admin.rs`): for
  whoever runs the server, never a Katna account. The addresses in
  `KATNA_SERVER_ADMIN_EMAILS` (none: 404) sign in with their own password
  (an Argon2 hash in `admins`), then a code mailed to that address. The
  first password is chosen on the page after a code mailed to the address
  (so whoever finds the page first cannot claim it); a forgotten one is set
  on the server with `katna-server admin-password`. The session is a
  12-hour `__Host-` cookie (`Secure`, `HttpOnly`, `SameSite=Strict`) kept
  in memory, and every call also carries `X-Katna-Admin: 1`. It shows this
  month's cost against the budget, requests, accounts in the free month,
  paid and at their cap, and the last six months; it chooses the service
  asked first and the fallback, with a model each, among those with a key
  in the environment (`KATNA_SERVER_AI_<SERVICE>_KEY`; keys never show
  there), the limits and prices, and an off switch; "Test" asks each
  chosen service a short question. Its settings are saved in the database
  (`ai_settings`) over the environment's. The page is static HTML, CSS
  and script served by the server under a strict Content Security Policy.

## 17. Performance budget

### 17.1 Measured (September 2026)

Release build, `lto = "fat"`, `codegen-units = 1`, `strip = true`,
`panic = "abort"`, Rust 1.98.1, x86-64 Linux.

| Build | Binary size | xz-compressed |
|---|---|---|
| GPUI window "hello world" (`gpui-pre` 0.3.6), **no Linux backend** (does not run) | 8.4 MB | 2.4 MB |
| GPUI window "hello world" with the Wayland and X11 backends (spike S1) | 21.3 MB | 6.2 MB |
| Window chrome spike (`katna-chrome` example, spike S1) | 21.5 MB | 6.2 MB |
| Engine libraries: tantivy + SQLite (bundled) + mail-parser + rustls | 6.2 MB | 2.2 MB |
| Katna Mail, first window (§13.5): GPUI + chrome + store + search + render | 29.2 MB | 8.8 MB |

The first GPUI row was built without the `wayland`/`x11` features of
`gpui-pre-platform`, so it had no backend and panics at start. With the
backends the binary links `libc`, `libxkbcommon`, `libxkbcommon-x11` and
`libxcb`; Wayland and Vulkan libraries are loaded at runtime. GPUI alone
takes about 21 MB of the 30 MB Katna Mail budget
(`docs/spikes/S1-window-chrome.md`); the first real window (§13.5) leaves
about 2 MB of the first 30 MiB (31.5 MB) budget.

### 17.2 Targets (to be verified on real hardware)

| Metric | Target |
|---|---|
| Katna Mail binary | ≤ 100 MB (100,000,000 bytes) |
| `katna-daemon` binary | ≤ 50 MB |
| Idle CPU (app and daemon) | ≈ 0 %; no periodic wake-ups beyond IDLE renewals |
| Cold start to usable inbox | < 500 ms |
| Search latency | p50 < 20 ms, p99 < 50 ms on 1M messages |
| Daemon memory | Measured and tracked in CI; budget set after first prototype |

With sync, bodies, the op queue, sending and the search indexer,
`katna-daemon` is 15.6 MB. tantivy is the biggest part. Its budget was
15 MiB until sending came in, then 20 MiB, and 50 MB since Katna
Server's tracking and translation came in (September 2026), so features
are not trimmed to fit. Katna Mail's budget was 30 MiB until the fixes
after the first real install, when the app reached it; then 50 MB, and
100 MB since the attachment viewers (September 2026), so features are
not trimmed to fit; light crates are still preferred. Crates that are not hot are built with
`opt-level = "s"` (root `Cargo.toml`): D-Bus (zbus, zvariant, oo7,
ashpd), IMAP parsing and regex.

### 17.3 Rules

- CI fails if a binary grows beyond its budget.
- CJK tokenizer dictionaries are optional downloads, not built in.
- Test early on old hardware and on machines without working Vulkan (GPUI
  falls back to CPU rendering there, which costs CPU).

## 18. Katna Calendar

**Pages of one window (decided 2026-09-29).** Calendar, Tasks, Notes and
Contacts are pages of the Katna Mail window, not separate programs: the
app rail switches them, as do Ctrl+1 to Ctrl+5 (Outlook's keys: Mail,
Calendar, Contacts, Tasks, Notes), the Go menu, the desktop file's
actions (right-click on the taskbar icon) and `katna-mail --page NAME`,
which a running Katna Mail receives as
`org.freedesktop.Application.ActivateAction("open-page", [NAME])`. The
window reopens on the page it closed on. One GPUI program keeps switching
instant, lets Mail show the day's agenda beside the inbox, and saves about
20 MB over a second one. Each page lives in its own module under
`apps/katna-mail/src/window/`, plugged in at `apps.rs`
(`MailWindow::render_app_page`, `open_app`); a page not built yet shows
what it will do. `apps/katna-calendar` stays an unused stub until it is
removed.

**Sync through each service's own API.** Following the feature rule
(the mail service's own feature first): Gmail accounts use the Google
Calendar API (Meet links, event types, colors; Google sends the
invitations), Microsoft accounts use Microsoft Graph, other servers CalDAV
(`katna-dav`), and there are local calendars for no account. Google's
CalDAV endpoint can't make Meet links or event types, so it is only the
fallback.

**Best way first, others when it is not available** (`katna_sync::methods`,
shared by Calendar, Contacts and Tasks). Each kind of account has a best
way, tried first; when it answers "not enabled", "not offered" or "sign in
again", the next is tried, and the way that worked is remembered per
account and kind (`meta` rows, object `account`, plugin
`sync-method:calendar|contacts|tasks`) and tried first for 7 days, after
which the best way goes first again. A new sign-in forgets it. Network or
server errors never switch ways. Google sign-ins: the Google API, then
Google's CalDAV/CardDAV with the same token. Microsoft sign-ins: Graph
(Outlook.com has no CalDAV). Password accounts linked to a Zoho sign-in:
Zoho Calendar's API for calendars, then CalDAV. Password accounts: CalDAV/CardDAV looked for
on the provider's known server (Yahoo, Zoho by region, iCloud, Fastmail,
mailbox.org, Posteo, GMX, web.de, Yandex, AOL; by mail domain or IMAP host), then
`.well-known` on the mail domain, the IMAP server's domain and the IMAP
server, and its root (`methods::dav_start_urls`). Credentials go only over
TLS to hosts of the domains the search started on. When no way works, the
most useful reason is shown. Changes go back the way their calendar came
(`calendar.source`).

- Calendars and events live in `pim.db`, synced by the daemon; apps read
  them read-only, as with mail.
- How the daemon syncs (`katna_sync::calendar`, driven by
  `apps/katna-daemon/src/daemon/calendar.rs`): one task goes through the
  accounts at start (after 10 s), every 5 minutes and on `SyncNow`, with
  its own store connection so mail sync never waits. **Google**: Calendar
  API v3, `calendarList` then each calendar's `events` with
  `singleEvents=false&showDeleted=true` (series once, with their
  `recurrence` lines; changed and cancelled occurrences as rows with
  `recurrence_id`), incremental through the `nextSyncToken` kept in
  `calendar.sync_token`; `410 Gone` lists everything again. Scope
  `https://www.googleapis.com/auth/calendar`, asked at sign-in with
  mail's; accounts signed in before show "sign in again", and
  `403 accessNotConfigured` shows that the Calendar API is not enabled in
  Katna's Google Cloud project. **Microsoft**: Graph `/me/calendars` and
  each calendar's `/events` (paged, `Prefer: outlook.timezone="UTC"`),
  read in full each time (Graph has no delta for stored events) and
  written only where an etag changed; a series' `recurrence` pattern and
  range become an `RRULE` in the zone its Windows name maps to; changed
  occurrences come through `$expand=exceptionOccurrences` and cancelled
  ones from `cancelledOccurrences` where Graph gives them (a calendar
  that refuses the expansion shows its series without changed
  occurrences). Scope `https://graph.microsoft.com/Calendars.ReadWrite`,
  consented at sign-in beside `Files.ReadWrite`, its tokens separate.
  **Zoho** (`calendar::zoho`, accounts linked to a Zoho sign-in, scope
  `ZOHO_CALENDAR`): Zoho's CalDAV answers only on port 543, which many
  networks block, so calendars come from Zoho Calendar's REST API
  (`calendar.zoho.<dc>/api/v1`, the data centre of the sign-in's
  accounts server, on port 443). `/calendars` lists them with a `ctag`;
  a calendar whose `ctag` didn't change today is skipped, otherwise its
  events are read with `byinstance=true` in 31-day ranges (Zoho's limit)
  from about 3 months back to a year ahead, each occurrence a row of its
  own, and written where their etag changed. Source `zoho` (pim.db v12,
  which widens `calendar.source`'s CHECK in place so events are kept).
  Read-only for now: a change to a Zoho event is refused and undone. A
  password Zoho account whose calendars don't sync shows `use-sign-in`
  (detail `zoho`, or `zoho: <why CalDAV failed>`) until it is linked, and
  after, when the Zoho sign-in is refused, so its line offers "Sign in
  with Zoho" rather than a password change.
  **CalDAV** (password accounts, and Google's fallback): found from the
  places above, with the IMAP password (or Google's token) over TLS; a
  calendar whose `getctag`/`sync-token` didn't change is
  skipped, otherwise the etags of its `VEVENT`s are compared with the
  store and only changed ones fetched by `calendar-multiget`
  (`katna_dav::ical` reads them). A server without CalDAV is asked again
  after 6 hours; one that took no connection, stalled in the TLS
  handshake or didn't answer in time is not taken for one without
  CalDAV: the account shows `error` with which step stalled
  (`katna_sync::Error::Unreachable`), and the next round asks again. Each account's state (`ok`, `needs-sign-in`,
  `not-enabled`, `error`, `none`) is `CalendarStatus()` on `Pim1`;
  `CalendarChanged()` (and the clock's `Agenda1.Changed()`) says when to
  read again; `SetCalendarHidden(id, hidden)` ticks calendars on and off.
  The Calendar page's side list shows every account, also one without
  calendars: one line under it gives that state's reason with its fix
  ("Sign in again to show calendars" for an OAuth2 account missing the
  calendar scope, else "Try again", which is `SyncNow` and looks for the
  calendars from scratch). `none` carries what the server answered
  ("calendar.zoho.in answered 404"); a Gmail or Outlook account added
  with a password gets `use-sign-in` (detail: the provider) and "Sign in
  with Google", since those providers let Katna into calendars only
  through their own sign-in. A CalDAV server that answers without naming
  the user's principal is asked for the calendar home itself.
- How edits flow (`Pim1.EditEvent`, `katna_sync::calendar::edit`): the
  daemon writes the change to `pim.db` at once and says
  `CalendarChanged`, so the app shows it on reload; rows the service
  doesn't have yet are marked (`event.pending`: 1 changed, 2 deleted and
  hidden until the service deleted it too), and a sync leaves marked rows
  (and their CalDAV resource) alone. A second task then sends the change,
  one change after another, with the services the sync uses and never
  while it syncs. Once sent, the marks go; if the service refuses, the
  marks go with the rows' etags and the calendar's sync token, and the
  calendar syncs again, which undoes the change here. Marks left when the
  daemon stopped are dropped the same way at start. New events get a UID
  `…@katna`, and where the service lets Katna choose (Google's event ID,
  the CalDAV resource name) their remote ID too. One occurrence of a
  series becomes a changed occurrence (Google patches its instance
  `ID_YYYYMMDDTHHMMSSZ`, Graph finds it through the series' `instances`,
  CalDAV adds a `VEVENT` with `RECURRENCE-ID` to the resource); deleting
  one skips it (`EXDATE` here, the instance deleted on the service);
  "this and following" ends the series the day before (`UNTIL`) and adds
  a new series; "all" moves the series by as much as the occurrence
  moved. Moving to another calendar is Google's `move` within an account,
  else a delete and an add. **Google**: `insert` with `recurrence` lines,
  reminder overrides and `transparency`, `patch`, `delete`;
  `sendUpdates=all` when there are attendees; a Meet call through
  `conferenceData.createRequest`. **Graph**: `POST`/`PATCH`/`DELETE`, the
  `RRULE`s the editor makes mapped back to Graph's `recurrence` (times in
  the zone's Windows name, else UTC), Teams through `isOnlineMeeting`;
  Graph can't bring back a deleted occurrence, so Undo there is refused
  and synced over. **CalDAV**: the whole resource written by
  `katna_dav::ical::write_calendar` (escaped, folded, with `VTIMEZONE`s)
  and `PUT` with `If-Match` (or `If-None-Match: *` for a new one), the
  server's new `ETag` kept; the user is `ORGANIZER` of events with
  attendees, so the server sends the invitations. Answers to invitations
  (`respond`) set the user's status here, then Google patches the
  attendees, Graph `accept`s, `tentativelyAccept`s or `decline`s with
  `sendResponse`, and CalDAV writes the user's `PARTSTAT`. Calendars on
  this computer (`source` `local`, no account) only store; the daemon
  makes one, without a name (apps show "On this computer"), when there
  is no calendar at all.
- `jiff` for time zones; recurrence is expanded when read, with
  exceptions (`RECURRENCE-ID`, `EXDATE`).
- Invitations (iTIP/iMIP) shared with Katna Mail: a mail with a
  `text/calendar` part (`katna_render::calendar_part`) shows a card at the
  top of the message (`apps/katna-mail/src/window/reader/invite.rs`):
  `METHOD:REQUEST` shows the event, the user's day two hours either side
  (clashes with busy events marked), Join, Open in Calendar and "Going? Yes
  No Maybe"; `REPLY` says who answered and how; `CANCEL` crosses it out.
  The card reads the calendars read-only and finds the event by its UID:
  Google, Graph and scheduling CalDAV servers put invitations in the
  calendar themselves, so answering is the calendar's own `respond`
  (the whole series for an invitation to one). Until the calendar has it,
  the card says so. An invitation that came to an account without
  calendars (plain IMAP) is answered by iMIP mail (RFC 6047): a reply to
  it, from that account, to the organizer, with a `METHOD:REPLY`
  calendar part holding only the user's `ATTENDEE` and the invitation's
  UID, times and sequence (`katna_dav::ical::reply_calendar`).
- Schedule a meeting (a conversation's ⋮ or right-click menu) opens the
  whole event editor on the Calendar page with the subject, without
  `Re:`/`Fwd:`, as the title and everyone in the conversation but the user
  (From, To, Cc) as guests; saving sends the invitations as any new event.
- An event's card has Email guests, and from an hour before its start
  to its end, Running late: a new mail to the other guests from the
  calendar's account, with a short "running late" line (Google Calendar's
  Email guests and Running late).
- Event kinds: a new event can be Focus time, Out of office or a Working
  location (tabs above its times, as Google's; fixed once made, as Google
  fixes `eventType`). Google gets the event type and its properties
  (Do not disturb, declining new invitations while out, a custom place);
  when Google refuses the type (not every account or calendar has it),
  the event is saved plain. Graph gets `showAs` `oof` or
  `workingElsewhere`; CalDAV `X-MICROSOFT-CDO-BUSYSTATUS:OOF`, or
  Katna's `X-KATNA-KIND` for the other two, which Katna reads back.
- The small new-event card also has a Task tab (Google's): the title,
  start day and time (none when all day), description and a repeat typed
  into the title make a task due then, in a task list picked on the card
  (the default list of the calendar's account at first). It goes to the
  daemon as the Tasks page's Add does. With no calendar to add events to
  but task lists, the card opens on Task.
- Alarms fire from the daemon as notifications (§15.1).
- Views: Day, Week (the default), Month, Year (Y or 5: twelve small
  months with a dot under days with events or tasks; a day opens Day, a
  month's name opens Month; resting the pointer on a dotted day, or
  tapping it on a phone, shows its events and tasks in a popover with a
  notch pointing at it, `calendar/year_peek.rs`, placed as the search's
  date popover is by `window/notched.rs`), Schedule and a custom view (X or 6: 2 to 7 days
  from the day picked, 4 by default, chosen in the options menu as
  `custom_days`), like Google Calendar, with calendars grouped by account; the week starts as the
  language says, with a choice in Settings. Below 1000 px for the bar,
  the view buttons fold into one button with a menu, as Google's do, so
  Today, the arrows and the dates stay; every control in the bar and the
  side column takes Tab.
- The bar's options button (⚙ in Google, a tune icon here, beside the
  app's own gear) has Density and Second time zone (`[calendar]` in
  `config.toml`). Density: Responsive (default; an hour is a twelfth of
  the grid's height, 40 to 72 px), Comfortable (48 px) or Compact
  (36 px); the grid keeps the same time at its top when it changes. A
  second time zone adds a column of its hours at the left of Day and
  Week, each column headed by its offset ("GMT-4"); the menu offers
  sixteen common zones, and any IANA name typed into the file works.
- Typed quick add, as in Fantastical and Todoist: one parser for events
  and tasks, `katna_core::quick_add::parse(text, today, words)`. The new
  event's title "Lunch with Anita Friday 1pm at Cafe Mocha" or "Standup
  every weekday 9:30 for 15 min" fills the day, times ("1-2pm", "11am to
  1pm"), length ("for 30 min"), repeat (an RRULE: "daily", "every 2
  weeks", "every Mon and Thu", "every weekday") and place ("at …") as it
  is typed; the card shows the place and repeat, and the rest is saved as
  the title. Deleting the words puts the fields back. Days: today,
  tonight, tomorrow, weekdays ("next Friday"), "Oct 5", "5th of
  October". The words come from a `Words` table per language; only
  English has one so far, and other languages use it. Bare numbers
  ("Buy 3 books") stay in the title.
- Desktop: Katna Digital Clock (§15.4) through the daemon's
  `in.invenia.katna.Agenda1`; KRunner results (§15.3).
- No booking pages: free times are shared as text in a mail. The options
  menu's Share free times (`window/calendar/free.rs`) opens a new
  message listing the gaps of at least 30 minutes between busy events
  (shown calendars, not cancelled or declined) from 9:00 to 17:00 on the
  next five weekdays, from the next half hour today, with the UTC
  offset.
- Calendar sets, as Fantastical has them (`window/calendar/sets.rs`):
  named groups of calendars above the calendar list. + saves the
  calendars on show under a name (the same name again replaces it), a
  click shows a set's calendars and hides the rest (the same
  `SetCalendarHidden` as the ticks), and the set matching what is on
  show is highlighted. Kept in `config.toml` (`[[calendar.sets]]`, the
  page's calendar IDs), not synced.
- Search (`window/calendar/search.rs`): on the Calendar page the top
  bar's box says "Search events" and finds events, as Google Calendar's
  does, keeping the mail search's words for when Mail shows again. An
  event matches when its title, place, notes, organizer or guests hold
  every word. The results take the view's place, by date, over two years
  either side: coming events soonest first, then Past events newest
  first, a repeating event once in each. A click (or Enter, for the
  first) opens the event on its day with its card; Esc, or any move of
  the page, puts the results away, and Esc brings them back.
- Server quirks: test against Google, Nextcloud, Radicale, Fastmail, Stalwart.

### 18.1 Katna Tasks

Tasks live in each account's own task service, so they show on the
phone and in the web apps: Google Tasks for Google accounts, Microsoft
To Do (Graph) for Microsoft accounts, Zoho Mail's tasks (the Zoho Mail
Tasks API) for accounts signed in with Zoho, to-dos (`VTODO`) on the
CalDAV server of an account with a password, and lists kept on this
computer. Google's, Microsoft's and Zoho's CalDAV servers keep no to-dos,
so their accounts use their own APIs only. Tasks go through the same ways and remembered
choice as calendars (§18, `katna_sync::methods`, `Data::Tasks`): a way
whose sign-in refuses Katna is passed over for the next; a network or
server error is not.

- **Store** (`pim.db` v5, `katna_store::tasks`): `task_list` (an
  account's list, or one on this computer) and `task`. A change made in
  Katna marks the row dirty; a deleted row stays as a tombstone until the
  service deleted it too. Moving a task to another list is a delete there
  and an insert here, as Google's own apps do.
- **What the service can't keep stays here.** Google Tasks keeps title,
  notes, a due day (never a time), done, its place in the list and one
  level of subtasks. A due time, reminders, repeat and the star are kept
  in `pim.db` only. To Do keeps reminders, repeat (mapped to and from an
  RFC 5545 `RRULE`) and importance (the star); its due is a day too, so the
  time stays in Katna there as well. Steps of a To Do task are its
  checklist items (ID `task|item` in `pim.db`), which keep only a title
  and a tick. The delta holds no steps, so each task it brings has its
  steps read again, and steps of that task not among them are dropped.
  CalDAV keeps it all, the due time too (written in UTC); a step is a
  to-do with `RELATED-TO;RELTYPE=PARENT`. A change is written over the
  server's own text of the to-do (`katna_dav::todo`), so categories,
  attachments, other alarms and a client's own fields stay.
- **Zoho** (`katna_sync::tasks::zoho`, `https://mail.zoho.<dc>/api/tasks`,
  header `Zoho-oauthtoken`) goes through the Zoho sign-in linked to the
  password account (`AccountSettings::linked`, `daemon/linked.rs`); the
  mail stays on its password. Zoho keeps title, description (notes), a
  due day (`DD/MM/YYYY`), done and one level of subtasks (steps); its
  priority, reminder and repeat are not mapped, so the star, reminders
  and repeat stay in Katna as with Google. Its personal tasks (`me`) are
  the default list and each group with tasks is a list (`group:<zgid>`).
  Zoho makes no lists of one's own: a list made in Katna stays on this
  computer (refused and logged each round), and a Zoho list renamed or
  deleted in Katna is not sent, so the next round brings it back. Each
  field has its own `PUT` in Zoho's reference, so a change reads the task
  and sends only what differs. A Zoho account (`oauth::is_zoho_host`)
  without the linked sign-in shows `USE_SIGN_IN` ("Sign in with Zoho")
  and keeps its lists on this computer; a linked sign-in that stops
  working shows the same, not the password's "Change password".
  `USE_SIGN_IN`'s detail is `provider` or `provider: why the other way
  failed`, and the side list shows that reason in small type under it.
- **Sync** (`katna_sync::tasks`, run by the daemon's `daemon/tasks.rs`):
  every 5 minutes, and 2 seconds after a change in Katna. Each round sends
  list changes, takes the service's lists, then per list sends task
  changes and pulls: Google by `updatedMin` (everything once a day), To Do
  by its delta link, Zoho by reading every task of the list (it has no
  changes feed; subtasks are read for tasks that have some), CalDAV by the list's `getctag` and then the etags of
  its to-dos (only changed ones are downloaded; the list's sync state
  keeps each to-do's etag and `UID`, so a missing one is a deletion). The
  CalDAV server is found the way its calendars are (§18, so Yahoo, Zoho,
  iCloud, Fastmail and the rest whose CalDAV is on another host than
  their mail) and kept between rounds; a collection that holds to-dos is a list, the first holding
  only to-dos the default. A change made in Katna and not yet sent wins over the
  service's. Busy or failing services (429, 5xx) wait for the next round;
  a refused change is logged and left dirty.
- **Sign-in**: the scopes are `https://www.googleapis.com/auth/tasks` for
  Google and `Tasks.ReadWrite` (Graph, asked at sign-in beside OneDrive's)
  for Microsoft, and `ZohoMail.tasks.ALL` for Zoho (a linked sign-in
  beside a mail password). Accounts signed in before Katna asked for them are
  skipped until they sign in again. Zoho's token answers may not name
  their scopes, so Zoho's sign-in is checked with one small read, and a
  401 or `INVALID_OAUTHSCOPE` means it doesn't allow tasks.
- **Each account's state**: every round keeps where each account's tasks
  stand (`Pim1.TasksStatus`, the states of `katna_dbus::task_state`, the
  same as a calendar's): synced; a sign-in without tasks, or a refused
  password (`needs-sign-in`); the Google Tasks API switched off for
  Katna's Cloud project (`not-enabled`, from Google's
  `accessNotConfigured`); a failed sync with the server's words
  (`error`); or no task service (`none`). Every mail account shows in the
  Tasks page's side list, also one without lists, and one line under it
  says why its lists are missing, with the click that fixes it: "Sign in
  again to show tasks" for an OAuth2 account; for a refused password, app
  password advice and "Change password" (Settings > Accounts), as on
  Contacts; else the reason and "Try again", which syncs the account now
  (`SyncNow` wakes the task sync too). The line is
  `window/account_status.rs`, shared by the pages whose lists come per
  account; each page gives its own words.
- **Default list**: new tasks without a list (the desktop clock's) go to
  the first account's default list once it has synced, else to the list
  on this computer. Tasks the clock kept on this computer before any
  account's list synced move to that list once.
- **The Tasks page** (`window/tasks_page.rs`) is a page of the mail
  window, laid out like Google Tasks. It reads `pim.db` read-only and
  sends changes over `Agenda1` (`AddTaskTo`, `EditTask`, `MoveTask`, `PlaceTask`, the
  list calls), then reads again on `Changed`. Beside All tasks and
  Starred, Today (as in To Do's My Day and TickTick) gathers the open
  tasks due today or before from every list: Overdue first, then Today,
  by day and time. A task added there goes to the default list, due today.
  All tasks puts as many lists side by side as the window fits, then
  more rows below that scroll down (one list per row on a phone), so no
  list is out of reach; a list just made is scrolled into view.
- **From mail**: Add to Tasks (Shift+T, as in Gmail, and in the mail's
  right-click and ⋮ menus) makes a task in the default list titled with the
  conversation's subject, keeping the newest message's `Message-ID` in
  `task.mail`; the task's Mail chip opens that mail again. Back in the
  mail list, a line whose mail has an open task shows a chip with the
  task's due day (red when past; "Task" with none), which opens the task.
  The window reads the tasks from the start and maps each `task.mail` to
  its line (the one due first wins) whenever tasks or mail change.
- **On the Calendar**, as in Google Calendar: a task due on a day sits
  with that day's whole-day events (Day and Week) and one due at a time
  sits at that time for half an hour, beside any event it overlaps; Month
  and Schedule list them with the events. Its circle ticks it off, a
  click opens it over the Calendar, and dragging it to another day, time
  or the whole-day row moves its due day and time (a quarter hour at a
  time, with Undo), blocking that time for it. A reminder moves with it.
- **Reminders**: the task's details offer Don't remind, At the time (on
  the day at 9 AM for a task without a time), An hour before (with a
  time) and The day before; a time set elsewhere (To Do) shows as itself
  and stays unless another is picked. `task.remind_at` is an instant.
  The daemon's reminder loop (§15.1, `daemon/alarms.rs`) also reads open
  tasks and shows a "Katna Tasks" notification at `remind_at`: the title
  and the first line of its details, with Open (the Tasks page), Mark as
  done and Snooze 5 min (`category=x-katna.task`). For Google Tasks the
  reminder lives on this computer only (decision 3 of the study).
- **Typed quick add**: a new task's title is read with Calendar's
  parser (`katna_core::quick_add`, §18) as it is typed, and what it
  found shows under the row as the task's chip will ("Mon, 8:00 AM ↻").
  On Enter the words become the due day, time and repeat ("Water the
  plants every Monday 8am"); a repeat without a day starts on its first
  day from today. Tasks have no place, so "at …" stays in the title, and
  a title that is only such words ("tomorrow") stays as typed.
- **Search**: on the Tasks page the top bar's box says "Search tasks"
  and shows only tasks whose title or notes hold every word typed, with
  a step's task and a task's matching steps; lists with none found hide
  in All tasks. The mail search's words come back on leaving the page,
  as with Notes and Contacts.
- **Drag and drop**, as in Google Tasks: an open task (not a step) drags
  up or down its own list, or into another list's card (outlined while
  the task is over it), in All tasks and in a list shown alone. It leaves
  its place as it lifts, and the list under the pointer opens a gap where
  it would land (150 ms, easing in and out), which follows the pointer
  between the tasks there (each task's row with its steps reports where
  it is drawn during the drag); the gap it leaves closes as the new one
  opens. Let go, it lands in the gap at once (the store follows) with a
  toast ("Task moved", or "Moved to …") and Undo, which puts it back
  after the task it was after; let go over no list, nothing changes.
  Done tasks and steps stay where they are. `PlaceTask(id, list,
  after)` (`Store::place_task`) puts it right after `after`, or first:
  - **Google Tasks** keeps the order. The task takes a position between
    its neighbours' (`tasks::between`: digits, compared as text as
    Google's are) and bit 2 of `task.dirty` (moved; bit 1 is a change of
    its fields), so no schema change was needed. Sync sends `tasks.move`
    with `previous` (the service's ID of the task before it here; none:
    first) and keeps Google's own position from the answer; a change of
    its fields goes too only when there is one. From another list it is
    first the delete there and the insert here, then the move. When the
    task before it is not on Google yet the move waits a round.
  - **CalDAV, To Do and lists on this computer** keep no order Katna can
    set (Katna does not write `X-APPLE-SORT-ORDER`; Graph has no order
    for tasks), so the order is kept in `pim.db` only: the list's tasks
    are numbered anew and nothing is sent. A service's answer or pull
    without a position leaves the one here.
- **Repeating tasks**: ticking one off moves it to its next day after
  both its due day and today, and it stays open (Google Tasks, CalDAV and
  lists on this computer; `katna_dav::todo::next_due`, done by the
  daemon's `set_task_done` so the clock and notifications do it too). A
  `COUNT` goes down by the days used; an ended rule ticks it off. Its
  reminder moves with it. To Do makes the next one itself, so there the
  task is ticked off as usual. The toast names the next day, and Undo
  puts the day back.

### 18.2 Video calls

Calls stay with the services people already use (study 2026-09-29; the
owner chose links now, and a call window inside Katna maybe later). Katna
makes and finds call links; the call itself opens in the browser or the
service's own app (`xdg-open`, or Windows' default). No Katna Server, no
media code, nothing added to startup.

- **Start a video call** (a conversation's right-click and ⋮ menus) and
  **Add a video call** (compose's More menu). A Gmail account signed in
  with scope `meetings.space.created` gets a Google Meet space from the
  Meet REST API (`spaces.create`, `katna_sync::meet`, over D-Bus
  `MeetingLink`); any other account, or a Gmail one signed in before Katna
  asked for Meet, gets a Jitsi Meet room made in Katna Mail with 16 random
  characters in its name, on the server in Settings > General > Video
  calls (`meetings.jitsi_server`, `https://meet.jit.si` by default).
  Start a video call opens the call and a new mail with its link to
  everyone in the conversation, from its account; Add a video call puts
  the link at the cursor. Teams links for Microsoft accounts come through
  Schedule a meeting (§18) only: Graph's `onlineMeetings` needs a
  permission personal accounts can't grant.
- **Join**: a mail with a call link (in its HTML links or written out) of
  Google Meet, Teams, Zoom, Webex, Jitsi Meet (`meet.jit.si`, `8x8.vc`),
  WhatsApp (`call.whatsapp.com`) or Telegram (`t.me/call/`, group video
  chats) shows a Join button per call, at most three, above its text
  (`katna_core::meeting`). Invitations skip it: their card has Join.
  WhatsApp and Telegram have no way for other apps to make calls, so their
  links are only joined.
- An event's card joins the same way: its own conference link, else the
  first call link in its place or description (a Teams invitation read
  over CalDAV or from mail), as "Join with <service>" at the top. The
  description shows its web addresses as links (Outlook's
  `text<https://…>` as `text`) without the rules of underscores
  (`calendar/description.rs`); only the details scroll, the title and
  Going? stay in sight.
- An event Gmail made from a mail (its description links to
  `mail.google.com/mail?extsrc=cal&plid=…`, an id only Gmail reads) gets
  Open the mail on its card: Katna searches its own index for the event's
  title and place words in the year of mail before the event's day,
  keeps a hit in the event's account, and opens it; with no hit, Gmail's
  link opens in the browser (`calendar/from_mail.rs`). A task made from a
  mail keeps its `Message-ID` and opens the mail from its card as well.

## 19. Security and privacy

- TLS only via `rustls`; no plain-text auth without an explicit warning.
- "Accept invalid certificates" on an account (`ServerSpec`,
  `katnactl --insecure`) is for test servers only: the connection refuses
  the TLS handshake when the address it reached is not on this computer or
  the local network (`katna_sync::net::is_internal`).
- Secrets only in the Secret Service (via portal inside Flatpak).
- Remote content blocked by default; HTML always sanitized.
- Incoming tracker removal.
- Metadata uploaded to Katna Server is E2E-encrypted where possible.
- The D-Bus API is session-bus only; inline replies and actions go through
  the same outbox and undo delay as normal sends.
- OpenPGP and S/MIME through the user's GnuPG (§19.1).
- The search index contains message text: it is covered by the same disk
  protection as the mail itself. Decrypted mail is never indexed.

### 19.1 Encrypted and signed mail

**Approach.** Katna does not keep keys. Like KMail (which uses GPGME),
`katna-crypto` runs the user's `gpg` for OpenPGP and `gpgsm` for S/MIME,
reading their machine-readable status lines (`--status-fd`). So the keys
people already have, their trust settings (web of trust, TOFU, certificate
chains), the gpg-agent passphrase cache, pinentry (KDE's or GNOME's) and
smartcards all work with no setup, and no key material passes through
Katna. `gnupg` is always installed on Arch (pacman needs it). A pure-Rust
engine (Sequoia, rPGP) stays in reserve for Flatpak or systems without
GnuPG; it would sit behind the same `katna_crypto::open` API.

**Reading (done).** `katna_crypto::protection(raw)` tells cheaply whether a
message is encrypted or signed; `katna_crypto::open(raw)` runs GnuPG and
returns the message with the protected part replaced by its content, plus a
report: decrypted or why not (no secret key, passphrase prompt closed,
damaged data, GnuPG missing), and each signature with its state (good, bad,
expired, key expired or revoked, key missing), the signer, the key's
addresses and GnuPG's validity. Supported: PGP/MIME (RFC 3156), inline PGP
(encrypted and clear-signed, in the part's own charset), S/MIME (RFC 8551)
enveloped, opaque-signed and detached-signed, and layers of these (signed
then encrypted). Protected headers (`protected-headers="v1"`) bring the
real Subject; other inner headers are ignored.

- **Where.** Katna Mail opens a message when it is shown, on a background
  thread (pinentry may be waiting for the user). Plaintext lives only in
  the app's memory: it is never written to the store, the blob store, the
  search index or a temporary file (detached signatures go to a temporary
  file in `$XDG_RUNTIME_DIR`; they are not secret). The daemon never
  decrypts, so no passphrase prompt appears without the user opening a
  message.
- **Banner.** Above the body: "Encrypted message", "Signed by … · verified"
  (green), "the key is not verified", "who is not the sender", "Bad
  signature", "Signed with a key you don't have", and failures with a "Try
  again" when the prompt was closed.
- **Verified** means: a good signature, from a key GnuPG fully trusts,
  whose addresses include the `From` address. A good signature by someone
  else is shown as a warning, not as a signature by the sender.
- **EFAIL.** Encrypted content is only opened when it is the whole message
  (the root part); GnuPG refuses unauthenticated (no MDC) ciphertext. Signed
  parts are checked wherever they are; when protection covers only a part
  (a mailing list footer), the banner says the rest could come from anyone.
  Remote content stays blocked in encrypted mail whatever the setting
  (for the HTML view).
- **GnuPG's word.** Decrypted means `DECRYPTION_OKAY`, then
  `END_DECRYPTION` (gpg; gpgsm sends neither), with no `BADMDC`, `NODATA`
  or `DECRYPTION_FAILED` on the way; a stream that is damaged or cut short
  never shows. The exit status does not decide, as gpg exits with an error
  for a signature whose key is missing, or when one of several keys a
  message is encrypted to fails though another opened it. GnuPG runs with
  `--no-verbose`, so a message cannot make it print lines that look like
  status lines (which share stderr with its log; a separate status pipe
  would need `unsafe` fd passing), and gpg with `--no-auto-key-retrieve
  --auto-key-locate local`, so checking a signature never fetches a key
  from the network (the fetch would tell the sender it was read).
- **Snippets and search.** Inline armor is left out of list snippets and
  the index (`katna_crypto::without_armor`): encrypted blocks are dropped,
  clear-signed text is kept without its armor.

**Sending (done).** Compose has Encrypt (lock) and Sign (shield) toggles
at the end of the recipients row. Answers to and forwards of encrypted mail
start encrypted and signed, in the same standard. The app builds the
message as usual, then `katna_crypto::protect` wraps it (PGP/MIME
`multipart/signed` or `multipart/encrypted`; S/MIME `multipart/signed` or
`application/pkcs7-mime` enveloped, signed inside first) before it is
queued, so the outbox and Sent hold only what was sent. Details:

- Keys are chosen by exact address in the local keyring
  (`katna_crypto::encryption_keys`: usable for encryption, a verified key
  before an unverified one) and passed to GnuPG by fingerprint, so GnuPG
  never looks a recipient up on the network (WKD) while sending. A key the
  user has not certified is still used (`--trust-model always`); a
  recipient without any key stops the send with "no key for …" and the
  message comes back.
- The sender is always a recipient too, so Sent stays readable. Bcc
  recipients are hidden recipients in OpenPGP (`--hidden-recipient`); CMS
  has no such thing.
- OpenPGP by default; S/MIME when answering S/MIME mail or when the sender
  only has an S/MIME certificate.
- Routing headers (From, To, Subject) stay outside the protection; hiding
  the subject (protected headers) and Autocrypt headers come later (E.3).


### 19.2 Crash reports and feedback

Asked for by the owner on 27 September 2026: when Katna crashes, the
traceback and a bug report should exist, and the project should learn what
to improve, but only with the user's consent and without tracking anyone.
Katna sends nothing until the user says yes. There are no trackers or
analytics libraries in the apps; the one outside service is the Sentry
project the owner chose for crash reports (§25), reached only after
consent.

**Part 1: crash reports on this machine (no network).**

- **Rust panics.** `katna_core::crash::install(app)` runs first in
  `main` of `katna-mail`, `katna-calendar`, `katna-daemon` and `katnactl`.
  It sets a panic hook that writes a report and then calls the previous
  hook, so the journal still gets the panic line. A panic on a worker
  thread that the app survives is still reported.
- **Native crashes** (a segfault in a GPU driver, `wgpu`, a C library, an
  abort). Katna forbids `unsafe`, so it does not install signal handlers.
  Instead, when Katna Mail starts (and on `katnactl crashes`),
  `katna_core::crash::collect_core_dumps` asks `systemd-coredump` for core
  dumps of `katna-mail` and `katna-daemon` newer than the last look
  (`coredumpctl --json=short list COREDUMP_COMM=<app>`, then
  `coredumpctl info` for each), on a background thread with a 10 s limit.
  Each becomes a report with the signal, the package line and the modules
  and stacks `coredumpctl` printed (at most 400 lines); the host name, boot
  and machine IDs, units and command line are left out. A process that
  ended without a core dump (killed at logout, out of memory, power loss)
  is not reported: Katna only claims a crash it can show. The first look
  goes three days back; the time of the newest dump seen is kept in
  `crashes/last-core`. Systems without `systemd-coredump` (Ubuntu uses
  apport) get panic reports only.
- **Report file.** One plain-text file per crash in
  `$XDG_STATE_HOME/katna/crashes/<UTC time>-<app>-<pid>.txt` (for example
  `20260927T031603Z-katna-mail-4242.txt`, so names sort by time), at most
  20 kept (oldest removed), at most three per run so a panic loop cannot
  fill the folder. Contents: app and version (`KATNA_VERSION` at build
  time), kind (panic, or native crash and signal), time, OS release
  (`/etc/os-release` `PRETTY_NAME`), desktop and session type
  (`XDG_CURRENT_DESKTOP`, `XDG_SESSION_TYPE`); for a panic the thread name,
  location, message, backtrace, raw frames and the last 50 log lines of
  that process (kept in memory by `katna_core::logging`). Never: mail
  content, subjects, account names, file names of attachments, passwords.
- **Scrubbing.** Before a report is written, percent-escapes are decoded,
  every `scheme://…` URL is cut to its scheme and host, the home directory
  becomes `~`, the user name, host name and machine ID become `<user>`,
  `<host>`, `<machine>`, and anything shaped like an email address, in any
  script, becomes `<email>`. A panic message loses the text it quotes
  (`` `…` `` and `"…"`, where Rust puts the string or value involved) and
  is cut to 200 characters. The crash directory and the state directory
  are created `0700` and reports `0600`. The same scrubber runs again before anything is sent (Part 2),
  so a report edited by hand is checked twice.
- **Readable tracebacks.** Release binaries are stripped, which leaves
  Rust's backtrace as `<unknown>` frames. So a panic report also lists
  every frame as `module + 0xoffset` (the `backtrace` crate's return
  addresses against `/proc/self/maps`) and the executable's GNU build ID;
  `addr2line -f -C -e <unstripped binary> <offset - 1>` turns them into
  functions and lines, and later Sentry does the same with the debug files
  CI uploads. Keeping symbol names in the daemon costs 3.1 MB and would
  cost more than it is worth (§17), so it stays stripped. Katna Mail keeps its
  function names (`strip = "debuginfo"` for that package only: 48 MB to
  56 MB of its 100 MB budget, no change in memory use since the symbol
  table is not loaded), so its panic backtraces and `coredumpctl` stacks
  name functions without any debug file.
- **Telling the user.** On the next start after a crash of Katna Mail or
  of the daemon, Katna Mail shows a quiet notice: "Katna Mail closed
  unexpectedly last time" (or "Katna's background service stopped
  unexpectedly") with **View report** (opens the text file) and
  **Copy report** (to paste into a GitHub issue). Dismissing it marks the
  report as seen; View and Copy do too. Older unseen reports are counted
  in the same note. `katnactl crashes` lists them, `crashes show [NAME]`
  prints one and `crashes delete` deletes them all.
- **Settings > User feedback** (a tab of its own, just before
  Experimental, asked for by the owner so all of this can be turned off at
  any time). Now: "Save crash reports on this computer" (on by default,
  since nothing leaves the machine; config key
  `feedback.save_crash_reports`, read at the moment of a panic, so it
  takes effect at once; off means the panic hook and the core-dump check
  write nothing), and the list of saved reports with
  **View**, **Copy** and **Delete** (and Delete all), each marked "Sent"
  once it went to the crash tracker. Part 2 adds "Send crash reports"
  (built) and later the usage statistics switch and **Send feedback**.

**Part 2: sending, only with consent.** Reports go to a Sentry cloud
project (decided by the owner on 27 September 2026, §25). Sending crash
reports is built; usage statistics and the feedback form come later.

- **Asking.** The first-run screen (onboarding) has a step, "Help improve
  Katna", between the look and the tour: what is sent, what is never sent
  and where it goes, with **Don't send** and **Send crash reports** given
  equal weight (the same outlined buttons) and no default. People who
  installed before this existed get the same words once in a dialog after
  updating, after What's new if that shows. Closing the dialog without an
  answer asks again on the next start; until an answer is given
  (`feedback.send_crash_reports` unset), nothing is sent. Settings > User
  feedback has the "Send crash reports" switch afterwards, changeable at
  any time; "Send anonymous usage statistics" joins it with C.6.
- **Crash reports.** With consent, new reports are sent without asking
  again, and the saved-reports list marks each one "Sent". The daemon
  looks 20 seconds after it starts, every 15 minutes, and at once when
  Katna Mail saves settings (`ReloadConfig`), never on a metered
  connection. It first turns new core dumps of Katna Mail and itself
  into reports (a daemon crash is reported even if Katna Mail is not
  opened), then sends the reports of the last seven days not sent yet,
  oldest first, at most five per look. `crashes/sent` lists what went. A
  2xx answer or a refusal (another 4xx) marks the report done; a 429,
  a server error or no network leaves it for the next look. The text sent
  is the saved file, scrubbed once more; "Save crash reports on this
  computer" off means there is nothing to send.
- **Usage statistics.** Once a week at most, a small JSON document:
  app version, OS release family (Arch, Ubuntu, …), desktop and session
  type, screen scale bucket, number of accounts in buckets (1, 2–3, 4+),
  and for a fixed list of features whether they were used that week (yes or
  no, never counts of messages or times): search options, pins, labels,
  scheduled send, encrypted mail, built-in viewers, phone layout, own frame,
  and so on. The exact list lives in one Rust enum; each entry is described
  in Settings > User feedback so users can see what is counted. No message counts, no
  addresses, no domains, no search terms, no timestamps finer than a week.
- **Identity.** No user ID and no account ID. Each upload carries a random
  **install ID** only so that one machine's weekly reports are not counted
  twice; it is regenerated every 90 days and by "Reset" in Settings > User feedback, and it
  is never sent with crash reports. The Sentry project stores no IP
  addresses (server side, below).
- **Feedback.** Help > **Send feedback** (global menu, Quick settings >
  Help and Settings > User feedback) opens a short form: what worked, what did not, optional email for a
  reply (clearly optional, never filled in from the account). It shows
  exactly what will be sent before sending. This is independent of the
  switches: sending feedback is itself the consent for that one message.
- **Protocol, no SDK.** Everything uses Sentry's envelope format
  (`POST /api/<project>/envelope/`), written by hand in
  `katna_core::sentry` and posted with Katna's own small HTTPS client
  (`katna_sync::autoconfig::http::post`, `rustls`), so sending adds no
  dependency and nothing to the daemon's size. An envelope holds an error
  event read back from the report's text and the report itself as a
  `text/plain` attachment, so the upload is exactly what the user could
  read. The event ID is a hash of the report, so a report sent twice is
  kept once. Frames: a panic in a build with function names (Katna Mail)
  uses Rust's backtrace (function, file, line), without the frames of the
  panic machinery; a stripped build (the daemon) sends each frame as an
  address relative to its module (`addr_mode: "rel:N"`) with the module's
  ELF image in `debug_meta` (debug ID from the GNU build ID, as
  `sentry-cli` computes it), for the debug files of C.3; a native crash
  uses the `coredumpctl` stack of the crashed thread the same way, with
  the build IDs `coredumpctl` lists. Level `fatal`, release
  `katna@<version>`, tags `app` and `kind`, the OS line as
  `contexts.os.raw_description`. Checked on 27 September 2026: Sentry
  answered 200 to a test envelope. Sentry's minidump handler
  (`sentry-rust-minidump`, an extra process) is not used unless the stacks
  from core dumps turn out not to be enough. Later, feedback uses Sentry's
  User Feedback item; usage statistics are one `info` event per week whose
  tags are the feature flags above, plus release-health sessions for
  crash-free rates.
- **Client settings.** Nothing like the SDK's `send_default_pii`: no user
  object, no IP (the project is also set to not store IP addresses and to
  scrub data server-side), no `server_name`, no device ID; the recent log
  lines go only inside the scrubbed report. The DSN is one constant,
  `katna_core::ids::SENTRY_DSN` (a DSN is the project's public address,
  not a secret); an empty DSN turns sending off, and `feedback.dsn` in the
  settings file can point it at a self-hosted Sentry or GlitchTip. The
  daemon does the upload (only it talks to the network, §9); Katna Mail
  only writes the setting, and the daemon reads the reports from the
  crash folder.

**Server side.** The Sentry cloud project exists (organization
`invenia-systems`, project ID `4512156171698256`, created by the owner on
27 September 2026). Its settings:

- **Done** (owner, 27 September 2026), organization-wide under Security &
  Privacy > Data Scrubbing, so every project must follow them: **Require
  Data Scrubber**, **Require Using Default Scrubbers** and **Prevent
  Storing of IP Addresses**. The server drops IPs and scrubs again
  whatever the client missed; the Katna scrubber's placeholders (`<user>`,
  `<host>`, `<email>`) need no safe-field entries;
- CI uploads each published build's debug files (`sentry-cli
  debug-files upload`, keyed by build ID) so native and panic stacks get
  function names and lines; the auth token (scope `project:write`) is a
  GitHub Actions secret, `SENTRY_AUTH_TOKEN`, that the owner adds, and only
  the package workflow reads it;
- the Sentry GitHub integration links crash groups to GitHub issues, so
  crashes land where bugs are already tracked;
- retention at the plan's default (90 days) for events; usage statistics
  are read as weekly totals per version and may be published on the
  website so users see what their data is used for.

**Later: our own server.** The owner plans to move from Sentry cloud to a
self-hosted receiver on `katna.invenia.in` (GlitchTip or self-hosted
Sentry, for example at `crash.katna.invenia.in`). The envelope format is the
same, so the move changes only the DSN constant and the debug-file upload
target; reports already sent stay in the cloud project until it is closed.

## 20. Dependency policy

- **GPUI:** pin exact `gpui-pre` and GPUI Kit versions; GPUI types only in
  `katna-ui`, `katna-chrome` and the GUI apps. `gpui-pre-linux` is a
  vendored copy with the KDE global menu patch (§15.2); upgrading GPUI means
  re-applying it (`vendor/gpui-pre-linux/KATNA.md`). `gpui-pre-wgpu` and,
  for layout direction and line breaking (§13.10), `gpui-pre` are vendored
  the same way, each with its own `KATNA.md`.
- **Pimalaya: light forks.** Fork only crates we change. Fork `master`
  mirrors upstream; our changes live on a `katna` branch. Use
  `[patch.crates-io]` in the workspace; drop the patch when upstream merges
  our fix. Keep `PATCHES.md` in each fork. Upstream fixes early.
- **Plasma clock fork:** same light-fork model. Merge upstream
  `applets/digital-clock` changes on each Plasma release; keep Katna code in
  separate files; CI builds against every supported Plasma version.
- **Weekly CI job** builds against the latest upstream `gpui-pre`, Pimalaya
  and Plasma to detect breaking changes early.
- Before a release: no git dependencies (upstream merged, or forks
  published as `katna-*` crates).
- Pimalaya code is AI-assisted by its own disclosure: we test it against
  real servers (Dovecot, Gmail, Fastmail, Stalwart, Nextcloud).

## 21. Packaging

| Package | Contents | Formats |
|---|---|---|
| `katna` | Katna Mail, Katna Calendar, `katna-daemon`, systemd user unit, D-Bus service files, KRunner and GNOME search-provider files, `.desktop` files, AppStream metainfo, icons, MIME handlers (`mailto`, `text/calendar`), Dolphin service menu | Flatpak, .deb, .rpm, AUR, AppImage, Nix |
| `katna-plasma-integration` | Calendar-events plugin, Katna Clock | .deb (Kubuntu), AUR (Arch) — **not** Flatpak or KDE Store (C++ loaded into `plasmashell`); built against each distro's Plasma |

| Format | Tooling | Priority |
|---|---|---|
| Flatpak (Flathub) | `flatpak-cargo-generator` for offline builds | 1 |
| .deb | `cargo-deb` (+ CMake for the Plasma package) | 1 |
| .rpm | `cargo-generate-rpm` | 2 (built in CI, not tested) |
| AUR | PKGBUILD | 1 |
| AppImage, Nix flake | standard tooling | 2 |
| Gentoo, Alpine, Void | `cargo-ebuild`, APKBUILD, templates | 3 |

Notes:

- Flatpak uses the Background portal for the daemon (§9.2) and must be
  checked for KRunner file export and notification inline reply.
- Official Debian/Fedora repositories require every crate to be packaged
  separately and do not accept git dependencies. Third-party repositories
  (Flathub, AUR, OBS, Copr, PPA) are the realistic path.

### 21.1 What exists today

`packaging/` holds the files every package installs, named after the IDs in
`katna_core::ids`: the systemd user unit (`systemd/katna-daemon.service`,
`Type=dbus`), the D-Bus activation file
(`dbus/in.invenia.katna.Daemon.service`, which starts that unit), and the
desktop entry and scalable icon of Katna Mail (`desktop/`, `icons/`, named
`in.invenia.katna.Mail`). Tests in `katna-core` and `katna-daemon` fail if
the file names or their `Name`, `Exec`, `Icon`, `StartupWMClass` and
`BusName` lines drift from the IDs or from what
`katna-daemon install-user-service` writes. Package scripts install these
files with globs, so they never spell out an ID.

`packaging/arch/PKGBUILD` builds a `katna-git` package (provides `katna`)
from the checkout it sits in: `cd packaging/arch && makepkg -si`. It ships
`katna-mail`, `katna-daemon` and `katnactl`; Katna Calendar joins once it
does something. CI builds it on every push to `main` and publishes it, with
a pacman repository database, as the `arch-latest` pre-release, so Arch
users can install and update without building. See `packaging/README.md`.

Since 30 September 2026 CI also builds, on every push to `main`, a Fedora
RPM (`packaging/fedora/katna.spec`, built from source with Fedora's Rust),
a Nix flake (`flake.nix`, `packaging/nix/package.nix`), an AppImage, a
Snap, a Flatpak and a plain tarball with an `install.sh`
(`.github/workflows/linux-packages.yml`). The last four share one build on
Ubuntu 22.04's glibc (2.35), so they run on older systems too. Each is
installed and tried on its own platform (Fedora, Nix, Ubuntu with FUSE,
snapd, Flatpak, Debian): D-Bus must start the daemon for `katnactl`, and
Katna Mail must open a window, whose screenshot is published with the
files on the `linux-latest` pre-release. `packaging/linux/stage.sh` lays out
the same files the PKGBUILD installs for all of them. None of these
updates itself yet (`Package::Other`): their own tools, or a new download,
update them. They are unsigned and in no store; Flathub, the Snap Store,
Copr and nixpkgs are later steps. The Flatpak's ID is the ID prefix
(`in.invenia.katna`), so it may own and export both the app's and the
daemon's names; it has no systemd unit, so D-Bus runs the daemon directly.
The AppImage writes a user D-Bus activation file that runs the AppImage
itself wherever it is, and its "Start Katna at login" entry names the
AppImage. snapd's user daemons are experimental, so the Snap has no
activation file; where the daemon's name has no owner and cannot be
activated, Katna Mail and `katnactl` start the `katna-daemon` beside them
(`katna_dbus::ensure_daemon`), as in an unpacked tarball.

### 21.2 Update channels and safe updates (partly built)

Planned 26 September 2026. Built so far (28 September 2026): **in-app
updates** for the Arch package, below; the rest is still planned. Today the only update path is the `arch-latest`
pre-release (§21.1): every push to `main` replaces it, with no gate beyond
the pull request's CI. That is fine for testers, not for people who rely
on Katna for their mail. The work is in `IMPLEMENTATION_PLAN.md`,
"Release track".

The rule behind all of it: **an update must never lose or corrupt mail,
settings or passwords, and a broken update must be recoverable by the
user without a terminal.** Mail on the server can always be downloaded
again; local-only data (outbox, drafts not yet saved on the server,
offline changes in `op_queue`, organizations, notes, metadata) cannot, so
it gets the most care.

#### Channels

| Channel | Built from | Who it is for | How often |
|---|---|---|---|
| **Nightly** | every push to `main` (today's `arch-latest`) | developers and testers | every merge |
| **Beta** | a tag `vX.Y.Z-beta.N` cut from `main` after the release checks pass | people who want new features early and report bugs | every few weeks |
| **Stable** | a beta promoted unchanged after its soak | everyone else; the default | after the soak |

- Stable and beta are the **same files**: promotion copies the beta's
  packages and signatures, it never rebuilds, so what was tested is what
  ships. A fix found during the soak makes a new beta (`-beta.N+1`) and
  restarts the soak.
- Versions follow SemVer. Patch releases (`X.Y.Z+1`) are bug fixes only and
  need no schema migration; migrations land in minor or major releases.
- Each package format maps the channels onto what it already has:

| Format | Stable | Beta | Nightly |
|---|---|---|---|
| Flatpak | Flathub | Flathub beta | Katna's own Flatpak repository |
| Arch | pacman repo `[katna]`; AUR `katna` | repo `[katna-testing]` | repo `[katna-nightly]` (today's `arch-latest`); AUR `katna-git` |
| .deb / .rpm | apt/dnf repository, `stable` component | `beta` component | `nightly` component |
| AppImage | update info points at the stable feed | beta feed | nightly feed |

- The channel is chosen in Settings → About → Updates (Flatpak and AppImage)
  or by the repository the user added (distribution packages; the page then
  shows how to switch).
- Moving to a safer channel (nightly → beta → stable) never downgrades. The
  installed version stays until the new channel catches up, because an older
  version may not read the newer database (§5.3, `SchemaTooNew`).

#### In-app updates (built for the Arch package)

The owner asked for Katna to update itself from the app on every package
it ships: check, download, ask for the password, install and restart.
This replaces "Katna only notifies" below for packages that plug into it;
Arch is the first, Windows and the others follow the same flow.

- **One flow, one plug per package.** `katna_core::update::Package` names
  the package a build came in (`$KATNA_PACKAGE` at build time; the
  PKGBUILD sets `arch`), the release its newest build is published in,
  and in Katna Mail (`updater.rs`) how a downloaded file is installed.
  Builds from source and packages without a plug (`Package::Other`) show
  no updates and are never checked.
- **Manifest.** CI writes `katna-update.json` beside the package on every
  build of `main`: version, file name, SHA-256 and size, and for the
  Update dialog the commit, when it was made, the What's new highlights
  (`katna-mail --highlights`) and the last 200 commits' first lines. One
  channel for now, the latest build (today's `arch-latest`). The download
  comes over TLS from GitHub and is checked against the manifest's SHA-256
  and size.
  **Signing** (security audit, 28 September 2026): once
  `packaging/keys/katna-update.pub` exists, CI's publish job signs the
  package with minisign (the private half is the
  `KATNA_UPDATE_SIGNING_KEY` secret, `packaging/keys/README.md`) and puts
  the signature in the manifest (`minisig`); the daemon saves it beside the
  download as `<file>.minisig`, and the root helper refuses a package
  without a good signature. Until the key exists updates are unsigned, and
  any program running as the user could pass its own `katna-git` package
  to the helper. The publish jobs also wait for CI on the same commit and
  publish only when it passed. The manifest names
  the package under its versioned file name, which CI uploads once and
  never replaces, and the daemon tries a failed download three times,
  reading the manifest again before each new try, because a new build
  can publish while a download runs. A download that still fails shows
  as such in the Update dialog, with Try again.
- **The daemon checks and downloads** (the only network user): two
  minutes after it starts, then every six hours, never on a metered
  connection unless the user presses Check for updates. With
  `updates.auto_download` (Settings > General > Updates, on by default)
  it downloads a newer build at once into
  `$XDG_CACHE_HOME/katna/updates/`, checks it, and shows a notification
  with an Update button. `UpdateStatus`, `CheckForUpdate`,
  `DownloadUpdate` and `UpdateChanged` on `Pim1` let Katna Mail follow;
  `UpdateDetails` hands over the offered manifest as JSON, so it can
  grow without changing the interface.
- **The Update dialog** (Help > Check for Updates, Quick settings > Help,
  and the notification's Update button) shows the installed version
  beside the new one, each with when it was built and its commit, the
  download's size, the new version's highlights this build does not
  have, the commits since the installed one and a link comparing the two
  on GitHub. About shows only the version.
- **Katna Mail installs**, because the password prompt (the polkit agent)
  belongs to the desktop session and a user service has none. The
  dialog says Katna Mail will close, install and open again, and Update
  and restart runs `pkexec /usr/lib/katna/katna-update-helper <file>
  <sha256>`. The polkit action `in.invenia.katna.update` (`auth_admin`,
  never remembered) allows only that helper. As root, the helper copies
  the file where only root can write, checks the copy's SHA-256 again,
  its signature (above), that it is the `katna-git` package and newer
  than the one installed (never a downgrade), and runs `pacman -U`.
- **Restart.** Katna Mail starts the new binary with
  `--after-update <pid>`, which waits for the old one to quit (the window
  state is saved on quit), and quits; the new one opens as the old one
  closed. The daemon restarts itself once its binary was replaced (§9.2).

#### Who updates what

Katna never replaces files that a package manager owns, except through
in-app updates above, where the package manager itself installs the
update after the user's password.

| Install | Who installs the update | What Katna does |
|---|---|---|
| Flatpak | Flatpak (GNOME Software, Discover, `flatpak update`) | The daemon watches the Flatpak portal's update monitor (`CreateUpdateMonitor`) and can ask it to update (`Update`) when the user allows automatic updates. |
| pacman, apt, dnf | the package manager | Nothing by default: the user's usual system update applies it. An optional check (off by default) notifies when the channel has a newer version and opens Discover or GNOME Software. |
| AppImage | Katna | The daemon downloads the new AppImage (zsync, delta), checks its signature, swaps it in atomically and keeps the old file until the new one has started healthy. |
| Built from source | the user | Nothing. |

- Only `katna-daemon` talks to the network (CLAUDE.md), so the daemon does
  the checks. It checks at most once a day, never on a metered connection
  (§6.1), sends no account or device identifier, and can be
  turned off.
- For self-updating formats, the daemon reads a **signed update manifest**
  per channel (`https://katna.invenia.in/updates/<channel>.json`, mirrored
  on the GitHub release). It lists the version, release notes link,
  per-file SHA-256, the oldest version that can update to it directly, the
  schema versions it migrates to, a rollout percentage and a `pulled` flag.
  It is signed with an Ed25519 (minisign) key whose public half is built into
  the daemon; an unsigned or wrongly signed manifest is ignored.
- **Staged rollout** for self-updating installs: each install keeps a random
  number from 0 to 99 in its local settings and takes the update once the
  rollout percentage passes it (for example 10 %, 50 %, 100 % over a few
  days). Nothing about the install is sent. Package-manager channels cannot
  stage, so the beta soak is their safety net.

#### Running while updated

A package manager replaces binaries while Katna runs. Every combination of
old and new daemon and app must keep working:

- The daemon notices its own binary was replaced (`/proc/self/exe` ends in
  ` (deleted)`, checked on a timer and on each D-Bus call). It restarts
  itself only when it is idle: no send inside the undo delay, no migration
  or index write running, the op queue flushed. With systemd it asks the
  user manager to restart its unit; without systemd it re-executes itself.
- A new `Version() → (version, api, schemas)` D-Bus method lets the app and
  the daemon find out what the other side speaks. `Pim1` only ever gains
  members; anything else becomes `Pim2` (§14.1). A new app that meets an old
  daemon asks it to restart; an old app that meets a new daemon keeps working
  on `Pim1` and shows a "Katna was updated, restart" pill.
- An app that opens a store and gets `SchemaTooNew` shows the same restart
  pill instead of an error.
- Search index versions already rebuild in the background when they differ
  (`katna-search` deletes an index built with another `SCHEMA_VERSION`);
  search falls back to the store's plain lookups while that runs.

#### Protecting local data

- **Backup before migrating.** When the daemon is about to raise a
  database's `user_version`, it first copies each database it will change
  with SQLite's online backup into
  `$XDG_DATA_HOME/katna/backup/<old version>/`, after checking there is room
  for it. No room, no migration: the daemon stays on read-only duty and
  says why. The last two backups are kept.
- **Expand, then contract.** A minor release's migrations only add tables,
  columns and indexes, so the previous stable release can still read the
  database. Removing or renaming happens one release later, once nothing
  reads the old shape. A migration that cannot follow this rule is only
  allowed in a major release.
- Each database records the oldest Katna version that can open it
  (`min_reader_version` in a small `schema_meta` table), so an older version
  can tell "newer but still readable" from "too new".
- Settings: `config.toml` keys are only added; unknown keys written by a
  newer version are kept, not dropped, when an older version saves.
- Secrets in the Secret Service keep their attributes (§9.2.1) across
  versions; a change of attributes needs a migration with the same care.

#### After an update: health check and safe mode

- The first start of a new version runs a short **self-check**: every
  database opens and passes `PRAGMA quick_check`, the index opens or starts
  a rebuild, the Secret Service answers, D-Bus names are owned. The result
  and the version are written to `$XDG_STATE_HOME/katna/health.toml`.
- If the daemon fails to reach "healthy" three times within ten minutes, it
  starts in **safe mode**: no sync and no writes except the outbox, and a
  notification with "Restore previous data" (from the backup above) and
  "Copy debug report". Local-only data (outbox, `op_queue`, organizations,
  metadata) is exported to a file before any restore.
- A **downgrade** (the user installs an older package after a bad update)
  meets `SchemaTooNew` only if the newer release broke the expand-then-contract
  rule; the older daemon then offers the same restore.
- No telemetry unless the user opts in (§19.2). Crash details go to the
  journal and to the local crash reports of §19.2; "Copy debug report"
  collects version, health file, recent logs and those reports, without
  mail content, for the user to attach to a bug report.

#### Checks before a release ships

A release candidate goes to beta, and a beta to stable, only when all of
these pass. The upgrade and migration checks run in CI on every pull
request that touches a migration, not only at release time.

| Check | What it proves |
|---|---|
| Pull-request CI on the tagged commit | `fmt`, `clippy`, tests on Arch and Ubuntu 26.04, `cargo deny`, size budgets |
| Migration fixtures | A committed `mail.db`, `pim.db` and `blobs.db` of every released schema version migrates to the new one; row counts, threads, categories and a fixed set of queries give the same answers |
| Upgrade test | In a container: install the previous stable (and the one before it), add an account on the dev servers (Stalwart, Dovecot), sync, queue a send, create organizations and settings; upgrade to the candidate while the daemon runs; check the daemon restarts itself, migrations apply, nothing is re-downloaded or lost, the queued send goes out once, passwords still work |
| Rollback test | Install the candidate, then the previous stable: it opens the data (expand-then-contract), or restores the backup cleanly |
| Mixed versions | Old app against new daemon and new app against old daemon over D-Bus |
| Large-store migration | Migrating the Enron store stays within a time budget and needs no more free disk than the backup |
| Real servers | The dev server runs the candidate's read-only checks against Gmail (`thread_check`), Stalwart and Dovecot |
| Desktop smoke test | Each package installs and launches on Arch Plasma, Arch GNOME and Ubuntu 26.04 GNOME; screenshots of key screens compared with the last release |
| Performance | The Enron benchmark does not regress by more than 10 %; binary sizes within budget |
| Beta soak | At least 7 days on beta (longer for a release with migrations) with no open `release-blocker` issue |

- The steps live in `docs/RELEASING.md`; the release workflow runs the
  automated ones and a person approves promotion to stable (a GitHub
  environment with a required reviewer).
- Packages and manifests are **signed**: pacman packages and repository
  databases with a Katna GPG key (so the repositories use
  `SigLevel = Required`), Flatpak by Flathub, AppImages and manifests with
  the minisign key. Signing keys live only in the release environment.
- Risky new features ship behind a setting that is on in nightly and beta
  and off in stable until they have soaked.

#### When a release is bad anyway

- **Roll forward**: the fix goes out as a patch release through beta with a
  short soak (at least one day). If the fix is not ready fast, the previous
  release's code is re-released under a new patch version, so no package
  manager has to downgrade.
- The manifest's `pulled` flag stops staged rollouts at once. Installs that
  read the manifest and are already on the release say in the window that a
  fix is coming.
- Each bad release gets a short write-up in `docs/releases/` and a new
  check in the table above that would have caught it.

## 22. Licensing — Decided

**GPL-3.0-or-later** for the apps, daemon and shared crates (decided
26 September 2026; full text in `LICENSE`). Every source file carries
`SPDX-License-Identifier: GPL-3.0-or-later`; each crate sets
`license = "GPL-3.0-or-later"`.

- Compatible with our MIT/Apache dependencies (Pimalaya, tantivy, GPUI).
- Allows reusing code from GPLv3 projects such as Mailspring and
  Mailspring-Sync. With a permissive license, only their ideas could be used.
- The Plasma clock fork stays under its upstream licenses
  (GPL-2.0-or-later / LGPL / KDE-accepted GPL), compatible with the above.
- Katna Server: GPL-3.0 or AGPL-3.0 (AGPL keeps hosted forks open).

## 23. Roadmap

| Phase | Deliverable | Done when |
|---|---|---|
| **0. Foundations** | Workspace, CI (size budgets), `katna-core`, `katna-store` schema, `katna-search-cli` on the Enron corpus | Queries like `from:alice has:attachment invoice` return in < 50 ms on Enron |
| **1. Daemon and sync core** | `katna-daemon` with D-Bus API skeleton, IMAP metadata + text + offline window, POP3, SMTP, op queue, threading, systemd/D-Bus activation | Two real accounts sync incrementally in the background and survive restarts, suspend and network changes |
| **2. Organizations** | Model, rules, `org:` search, suggestions (CLI first) | Searching a company name finds mail from personal addresses |
| **3. Mail UI** | GPUI app as a daemon client, window chrome on KDE and GNOME, 3-pane layout, text/HTML-subset rendering, Markdown composer, undo send | Daily-drivable for one account on KDE and GNOME |
| **4. Notifications and desktop search** | Notifications with click-to-open, inline reply-all, archive/read; grouping; taskbar badge; tray; KRunner runner; GNOME search provider | New mail notifies with the app closed; reply-all from the notification works on Plasma; KRunner finds contacts and mail |
| **5. Gmail-class features** | Labels, snooze, send later, reminders, rules (+ Sieve), unsubscribe, templates, tabs/categories | Feature checklist complete |
| **6. Katna Calendar + Plasma calendar** | CalDAV, recurrence, invitations, alarms; calendar-events plugin; Katna Clock fork; upstream proposals to Plasma | Events show in the Plasma clock; add/edit from Katna Clock; syncs with Google, Nextcloud and Fastmail |
| **7. Katna Server** | Tracking, activity dashboard, metadata sync, server-side scheduled actions | Tracking with bot/scanner labeling in production |
| **8. Polish** | Full HTML rendering, WYSIWYG composer, OpenPGP, semantic search (local embeddings), GNOME top-bar calendar (EDS) | — |

Packaging (Flatpak, deb, rpm, AUR) starts from Phase 3; the
`katna-plasma-integration` package from Phase 6.

## 24. Risks

| Risk | Impact | Mitigation |
|---|---|---|
| GPUI has no official release since 0.2.2; `gpui-pre` has one maintainer | Breakage, stuck on old version | Pin versions, isolate UI crates, weekly upstream CI, be ready to maintain a fork |
| Pimalaya libraries are young and changing | API churn, bugs | Light forks, own traits, real-server tests, upstream contributions |
| HTML email rendering | Poor display of real-world mail | Phased approach; decide Blitz vs WebKitGTK early with a prototype |
| Rich-text composer in GPUI | Large effort | Markdown composer first |
| Google/Microsoft OAuth verification | Blocks Gmail/Outlook for public users | Launch with other providers; plan verification budget |
| Tracking law and spam filters | Legal risk, deliverability | Opt-in, privacy-first server, legal review, dedicated domains |
| Calendar edge cases (recurrence, time zones) | Wrong event times | Test corpus from real servers, fuzzing |
| Plasma private/internal QML APIs change every release | Katna Clock breaks on Plasma upgrades | Light fork, minimal edits, CI per supported Plasma version, upstream the generic parts |
| C++ plugin runs inside `plasmashell` | A bug crashes the desktop shell | Tiny, asynchronous plugin; no logic; crash tests |
| Wayland focus-stealing prevention | Click on notification does not raise the window | Pass activation tokens end-to-end (§15.1) |
| GNOME lacks inline reply; Flatpak portal gaps | Uneven notification experience | Capability detection; quick-reply window fallback |
| Background daemon drains battery or leaks memory | Users disable it | Event-driven design, CI resource budgets, power/metered awareness |
| GPU/Vulkan missing on old hardware | High CPU from software rendering | Test early on old machines; document requirements |
| An update corrupts or loses local data, or leaves Katna unable to start | Users lose mail they cannot re-download and stop trusting updates | Channels with a beta soak, upgrade and migration tests, backup before migrating, expand-then-contract schema changes, health check and safe mode (§21.2) |
| No official GPUI phone backend (§26.2) | A phone port rests on an experimental community backend | Vendor and patch it like `gpui-pre-linux`, contribute upstream, Android first |
| Phones stop background mail (§26.4) | Late or missing new-mail notifications | Foreground service or push per platform; say the delay plainly in Settings |
| Scope | Burnout, never shipping | Strict phases with "done when" criteria |

## 25. Open decisions

1. HTML renderer for phase 2 (§12).
2. Katna Server hosting and pricing model; Katna Server license (GPL-3.0 or AGPL-3.0).
3. Updates (§21.2): where the update manifests and package repositories are
   hosted (`katna.invenia.in` or GitHub releases only), who holds the signing
   keys, and how long the beta soak is.
4. Phones (§26.7): GPUI phone base, Android first, iOS licensing, the push
   gateway, and never holding mail logins on a Katna-run server.

Decided:

- License: GPL-3.0-or-later (§22).
- Repository: `QuakeString/katna`, one monorepo (§3); default branch `main`.
- App ID prefix: `in.invenia.katna` (domain `katna.invenia.in`); defined
  only in `katna_core::ids`.
- Message storage: SQLite for metadata and compressed raw messages;
  files only for large attachments (§5.2).
- Rust toolchain: latest stable (`channel = "stable"`).
- Crash reports and feedback (§19.2): sent, only after the user opts in,
  to a Sentry cloud project for now (owner's choice, 27 September 2026,
  over our own receiver and GitHub issues only), moving later to a
  self-hosted GlitchTip or Sentry on `katna.invenia.in`. The DSN stays
  empty until the cloud project exists.
- Languages (§13.10): Fluent for UI text over gettext, ICU4X for dates and
  numbers, system fonts (Noto) rather than bundled ones, and AI-drafted
  translations marked for native review (owner's request, 27 September
  2026).
- Test and support matrix: Arch Linux (latest Plasma and GNOME) and
  Ubuntu 26.04 LTS (GNOME) / Kubuntu 26.04 (Plasma). The Plasma
  integration supports the Plasma versions of these two.

## 26. Katna on phones (planned, not built)

Asked about by the owner on 27 September 2026. This is design only: nothing
here is built or scheduled, and the Linux desktop apps come first. The work
is in `IMPLEMENTATION_PLAN.md`, "Later: Katna on phones".

The goal is Katna Mail on Android and iOS with the same look (the phone
layout of §13.9 already follows Gmail's mobile app), the same local-first
store and instant search, and the same privacy promise: no trackers, no
analytics, no Google-only features, and no server that reads the user's
mail.

Two things stand in the way. GPUI has no official phone backend (§26.2),
and phones do not let an app keep a background service like `katna-daemon`
running (§26.3, §26.4).

### 26.1 What carries over

Most of Katna is plain Rust with no desktop ties and builds for Android and
iOS as it is.

| Part | On a phone |
|---|---|
| `katna-store` (SQLite, bundled), `katna-search` (tantivy), `katna-import`, `katna-meta`, `katna-org` | As is |
| `katna-render` (HTML mail drawn with GPUI elements, §12), `katna-preview` (pure-Rust PDF, sheets, pictures) | As is |
| `katna-sync` (IMAP, SMTP, POP3, JMAP over rustls; `rustls-platform-verifier` already supports Android and iOS) | As is, plus a "sync once before a deadline" entry point (§26.3) |
| `katna-core` | Paths come from the app's sandbox instead of XDG directories |
| Sync workers, op queue, outbox, scheduler, indexer inside `katna-daemon` | Move into a `katna-engine` library (§26.3) |
| `katna-daemon` shell: systemd unit, D-Bus name, `/proc/self/exe` re-exec, logind and NetworkManager events | Desktop only |
| `katna-dbus` | Desktop only; the phone app calls the engine in-process |
| Passwords in the Secret Service (`oo7`) | Android Keystore (a key that encrypts the secrets) and the iOS Keychain, behind a small `SecretStore` trait |
| `katna-notify` (freedesktop notifications) | Android notification channels and iOS `UserNotifications`, with the same actions (Reply, Mark read, Archive) |
| `katna-crypto` (runs the user's `gpg` and `gpgsm`) | Phones have no GnuPG. OpenPGP through a Rust library (rPGP or Sequoia) or OpenKeychain on Android; S/MIME later |
| `katna-platform`, `katna-chrome` (window frames, blur, tray, KDE global menu, portals) | Desktop only |
| `katna-ui`, `katna-mail` views | Carry over through GPUI; the phone layout exists, touch and text input are new (§26.2, §26.5) |

### 26.2 What GPUI is missing

**State in September 2026.** Zed's GPUI ships backends for macOS, Linux
(X11 and Wayland) and Windows only. Katna's copy (`vendor/gpui-pre-wgpu`,
`vendor/gpui-pre-linux`, 0.3.6) draws through wgpu, and wgpu runs on
Android (Vulkan, GLES) and iOS (Metal), so drawing is not the problem. The
missing part is everything around it.

- **Upstream iOS:** Zed pull request
  [#63068](https://github.com/zed-industries/zed/pull/63068) adds a
  `gpui_ios` crate: UIKit scenes and window lifecycle, CoreText, Metal,
  safe-area and keyboard insets, native text input, keychain credentials,
  touch and drag scrolling, and a simulator example. It was opened on
  22 August 2026, is still open, and its author calls it a side project.
  It leaves out momentum scrolling, edit menus and keyboard accessories.
  Nothing for Android upstream.
- **Community:** [`longbridge/gpui-mobile`](https://github.com/longbridge/gpui-mobile),
  from the authors of GPUI Kit (which Katna uses), is published as
  `gpui-pre-mobile` against `gpui-pre` 0.3.4 (Katna uses 0.3.6). It covers
  iOS (Metal, CoreText) and Android (Vulkan or GLES, cosmic-text), touch with
  momentum scrolling, safe areas, dark mode, an Android input activity for
  IME composition, and a file picker. It calls itself experimental;
  accessibility, full IME composition and lifecycle hooks are not done.
  Several forks of it exist.

What Katna would need from a phone backend, whichever one it starts from:

1. **Lifecycle.** Android destroys the drawing surface whenever the app
   leaves the screen and may kill the process at any time; iOS suspends it
   seconds after it is backgrounded. The backend drops and recreates the
   wgpu surface; the app saves what is open (conversation, draft, scroll
   position) and restores it after the process was killed.
2. **Touch.** GPUI's input model is mouse and keyboard. Phones need tap,
   long press (selection mode), fling with momentum, pull to refresh, swipe
   on a row to archive or delete (as in Gmail), the system back gesture
   (Android predictive back, iOS edge swipe) and pinch zoom in the viewers
   and HTML mail. Anything only reachable by hover (row hover actions,
   tooltips) needs a touch path; the phone layout already hides the hover
   toolbar (§13.9).
3. **Text input.** The hardest part: the on-screen keyboard, IME composition
   (Bengali, Hindi, Chinese and others), autocorrect and suggestions,
   selection handles, the copy and paste menu, and moving the compose field
   above the keyboard.
4. **Screen insets.** Status bar, notch or camera cut-out, gesture bar and
   keyboard.
5. **Accessibility.** TalkBack and VoiceOver through AccessKit, which GPUI
   already uses on the desktop.
6. **Fonts.** System fonts, emoji and complex-script shaping, honouring the
   system text size.
7. **Platform services.** Notifications, the share sheet (share a file into
   Katna as an attachment, share an attachment out), the system file and
   photo pickers, opening an attachment in another app (instead of
   "Open with", §13.8), `mailto:` links, network and metered-network state
   (instead of NetworkManager), OAuth in the system browser, and the unread
   badge.

**Recommendation.** Do not write a phone backend from scratch. Start from
`gpui-mobile`, vendored and patched the way `gpui-pre-linux` is, and move to
upstream `gpui_ios` for iOS if it lands. Send fixes upstream. **Android
first**: it builds and tests from Linux (cargo-ndk, Gradle and an emulator),
it allows real background mail on the device (§26.4), and F-Droid users are
the audience most likely to want a private mail app. iOS needs a Mac with
Xcode for building and signing, an Apple developer account, and a push
gateway (§26.4).

### 26.3 The engine without a daemon

On the desktop, `katna-daemon` owns the network and every write, and the
apps read the store and send commands over D-Bus (§2). On a phone there is
one app process (plus, on iOS, small extension processes), and the system
decides when it may run.

The same rules still hold; only the process boundary moves:

- **`katna-engine`.** The sync workers, op queue, outbox, scheduler,
  indexer and new-mail policy move out of `katna-daemon` into a library
  with no D-Bus, systemd or GPUI. `katna-daemon` becomes that engine plus
  its desktop shell (D-Bus, systemd, tray, updates). The phone app runs the
  engine on its own threads.
- **One client API, two transports.** The app talks to the engine through
  a `PimClient` trait: today's commands (`apps/katna-mail/src/daemon.rs`)
  and change signals. On the desktop it is the D-Bus proxy; on a phone it
  is an in-process channel. The views do not know which one they have.
- **One writer.** The engine's thread is still the only writer; the views
  read the store read-only. All SQL stays in `katna-store`.
- **Killed at any moment is normal.** The op queue and outbox are already
  on disk, so a killed process loses nothing. The engine must also start
  in well under a second, because on a phone it starts every time the app
  or a background task runs.
- **Sync once, with a deadline.** Besides "run until stopped" (the app is on
  screen: IDLE on each account, as on the desktop), the engine gets
  `sync_once(deadline)`: send what is queued in the outbox, fetch new
  headers for the Inbox and the folders that notify, raise notifications,
  then stop. Background runs are short (Android WorkManager work is limited
  to about 10 minutes; iOS background refresh gives about 30 seconds).
  Large jobs (indexing a first sync, downloading old bodies) wait for the
  app to be open or for the phone to be charging (Android WorkManager
  constraints, iOS `BGProcessingTask`).
- **iOS notification extension.** The Notification Service Extension that
  finishes a push (§26.4) is a separate process with a small memory limit
  (about 24 MB) and a few seconds of time. It reads the store read-only
  from the shared App Group folder, fetches the new message's headers
  itself, and never writes `mail.db`; it leaves a small note the engine
  picks up on its next start. The app must also end every write
  transaction before it is suspended: iOS terminates a suspended app that
  holds a file lock in a shared folder.
- **Send later and snooze with the app closed.** Android can run work at a
  set time (exact alarms need the user's permission from Android 14 on;
  WorkManager is late by minutes in Doze). iOS cannot run code at a set
  time at all. Scheduled sends there go out when the app next runs, or
  from Katna Server when the user has one (§16 already plans "send later
  while the machine is off"). Undo send is fine on both: the app is open
  during the delay, and a send started just before the app is closed
  finishes in expedited work (Android) or a background task (iOS).
- **Phone defaults.** Bodies are downloaded for fewer days than on the
  desktop, nothing big is downloaded on a metered network (the
  `sync.metered` setting, §6), and the first sync fetches recent mail first.

### 26.4 How new mail reaches a phone

On the desktop, the daemon keeps an IMAP IDLE connection open and the
server tells it at once about new mail. A phone app cannot count on staying
alive to hold that connection. iOS suspends it within seconds; Android
allows it only in a foreground service with a permanent notification, and
some phone makers kill even that. So something has to wake the app.

#### How other mail apps do it

| App | How new mail arrives | Does a server hold your mail login? |
|---|---|---|
| **Gmail** (Android and iOS), Gmail accounts | Google's mail servers see the message arrive and send a push through Firebase Cloud Messaging (Android: one shared connection that Google Play services keeps open for every app) or Apple's push service (iOS); the app then syncs. | No extra one: Google runs the mailbox and the push sender. This is what only a mail provider can do. |
| **Gmail**, other IMAP accounts | Checked on a timer ("Sync frequency"), or Gmailify, where Google's servers fetch the other account. | With Gmailify, yes: Google's. |
| **Apple Mail** (iOS) | Push only for providers Apple supports (iCloud, Exchange). Other IMAP accounts, Gmail included, are fetched every 15, 30 or 60 minutes or by hand. | No; it simply is not instant. |
| **Spark, Outlook and similar** | Their servers hold an OAuth token (or password), watch the mailbox, and push through APNs or FCM. Spark says it copies the subject and part of the message, encrypted, and deletes it 4 hours after notifying. | **Yes.** |
| **FairEmail, Thunderbird for Android (K-9)** | No server. A foreground service with a permanent "monitoring" notification keeps IMAP IDLE open; periodic sync as a fallback. FairEmail uses the `specialUse` foreground-service type because `dataSync` is limited to 6 hours a day from Android 15. Missed or late mail on aggressive phones is the most common complaint in both projects. | No |
| **Delta Chat** (chat over email) | The app stores an encrypted device token on its mail server with IMAP METADATA. When mail arrives the server sends that token to Delta Chat's notification proxy, which decrypts it and forwards an empty wake-up to Apple or Google; the app then fetches the mail itself. Only works with servers that cooperate (chatmail). | No. The proxy sees no mail data and forgets the token at once. |
| **Tuta** (Android) | Its own server-sent-events connection instead of Firebase, with a 15-minute job as backup. | Tuta is the mail provider. |

The lesson: Gmail's instant, battery-free notifications come from being the
mail provider *and* using the phone maker's push service. A third-party app
gets instant mail on iOS only if some server sends it an Apple push, and
that server must know when the mail arrived. Either the mail server says so
itself, or something has to watch the mailbox, which means holding the
login.

#### Katna's design: one wake-up format, several sources

Every device gets one **push address**, a standard Web Push URL (RFC 8030,
payloads encrypted to the device with RFC 8291, so nothing in between can
read them). Anything that knows about new mail sends a wake-up to that
address. What sits behind the address differs per platform:

- **Android:** a UnifiedPush distributor the user picks (ntfy, Sunup,
  NextPush, or a Google-based one on phones with Google services). No Katna
  server, no Firebase library in the app, fine for F-Droid.
- **iOS:** only a sender holding Katna's Apple push key can reach the app,
  so the address points at the **Katna push gateway**, a tiny Rust service
  (part of Katna Server, run by the project at `katna.invenia.in`). The
  device token is sealed inside the URL, encrypted to the gateway's key, as
  in Delta Chat, so the gateway stores nothing. It unwraps the token and
  forwards the still-encrypted payload as a mutable-content Apple push. The
  Notification Service Extension decrypts it on the phone and fetches the
  headers from the mail server directly. The gateway sees an opaque token,
  the time and the sender's IP address. It never sees credentials,
  addresses, subjects or content, and it keeps no logs of tokens.

The wake-up sources, from most to least private:

1. **The mail server itself.** JMAP servers with push subscriptions
   (RFC 8620 §7.2 with RFC 8291 encryption; Stalwart and Fastmail have them)
   post straight to the push address. Nobody but the provider is involved.
   Dovecot-based servers with METADATA push, as chatmail uses, can follow
   later. Gmail offers no push to other apps over IMAP (its API push needs
   Google Cloud Pub/Sub, a Google-only feature Katna does not use).
2. **The user's own Katna desktop.** `katna-daemon` already holds an IDLE
   connection to every account. With "Wake my phone" on, it sends a wake-up
   (optionally with the sender and subject, encrypted to the phone) when new
   mail arrives. No new place holds a password, and it works for every
   provider, Gmail included. It only works while that computer is on and
   awake.
3. **A Katna Server the user hosts** (§16): the same IDLE watcher in a
   container on the user's own server or VPS. The login stays on hardware
   the user controls. The same server can send later and snooze while the
   phone is off.
4. **On the phone alone.** Android: a foreground service holding IDLE
   ("Instant", with its permanent notification, which the user can hide by
   turning off that notification channel), or a timer (every 15, 30 or 60
   minutes via WorkManager), or by hand. iOS: background app refresh, when
   the system allows it (Apple warns that it may run rarely, or in common
   cases not at all), plus a full sync whenever the app opens.

**Not offered: a Katna-hosted server that holds logins** (Spark's model).
It works with every provider without any setup, which is why most
commercial apps do it. But it makes Katna a store of thousands of mail
passwords or tokens, costs money to run forever, and breaks "your data stays
on your machine". If it is ever reconsidered, it must be opt-in per account,
use app passwords or narrow OAuth scopes, send content-free wake-ups only,
forget the login on disconnect, and be open source and audited.

#### Recommendation per platform

- **Android:** default to "Instant" through the on-phone foreground service
  (the FairEmail model), because it needs no server and works with every
  provider on day one. Settings → New mail offers Instant, every 15 / 30 /
  60 minutes, or Manual, and "Use push" once a UnifiedPush distributor is
  installed, which switches JMAP accounts to server push and IMAP accounts
  to the desktop or self-hosted watcher and lets the permanent notification
  go away. The Settings page says plainly what each choice costs in battery
  and delay, and links to the phone's battery-optimization setting.
- **iOS:** instant mail only through push, so the push gateway ships with
  the first iOS build. JMAP accounts use server push; IMAP accounts (Gmail
  included) use the user's Katna desktop or self-hosted Katna Server; any
  account without either says, in Settings → New mail, "Checked when iOS
  allows it; may be delayed", and syncs fully whenever the app is opened.

### 26.5 Phone-specific UI work

The phone layout (§13.9) is the starting point. Still to do: 48 dp touch
targets; swipe actions on list rows (set in Settings, as in Gmail); long
press to select; pull to refresh; bottom sheets for menus; the system back
gesture closing the conversation, drawer or sheet in that order; attaching
from the camera and photo library; the system share sheet; the system font
size; tablets and foldables switching between the phone and tablet layouts
as they already do on the desktop. Desktop-only features are hidden: the
window frame and blur, tray, global menu, KRunner, and opening mail in a
new window.

### 26.6 Distribution and licensing

- **Android:** APKs on GitHub releases and F-Droid first (no Google
  libraries, so F-Droid accepts it), Google Play later (Play asks why an app
  uses a `specialUse` foreground service; FairEmail's reason was accepted).
  Built on Linux CI with cargo-ndk and Gradle; the Android host code
  (activity, services, notification actions, `rustls-platform-verifier`'s
  helper) is a few hundred lines of Kotlin, like the Plasma C++ in §15.6.
- **iOS:** TestFlight, then the App Store. Needs macOS CI runners, an Apple
  developer account and a small Swift or `objc2` shim for the notification
  extension. **Licensing needs a decision first:** the App Store's terms are
  widely held to conflict with the GPL (VLC was removed in 2011 over this).
  The owner, as copyright holder, can publish their own GPL code there, but
  outside contributions and any GPL code taken from Mailspring would need
  their authors' permission, a contributor agreement, or an App Store
  exception added to the license. This should be settled before Katna
  takes outside contributions.
- Crash reports on phones follow the desktop plan (opt-in, Sentry, no
  tracking).

### 26.7 Open questions

1. Which GPUI phone base: `gpui-mobile` now, or wait for upstream?
2. Android first (recommended) or both at once?
3. The iOS licensing path (§26.6).
4. Who runs the push gateway and where (`katna.invenia.in` recommended).
5. Whether a Katna-hosted login-holding watcher is ever offered
   (recommended: no).

## 27. Katna on Windows (in progress)

Katna Mail and `katna-daemon` also run on Windows 10 (version 1903 or
later, whose `icuuc.dll` GPUI's text needs) and 11, installed by
a Katna Setup.exe. Linux stays the main platform: nothing here changes how
Katna behaves there.

### 27.1 What stays and what changes

The engine crates (`katna-core`, `katna-store`, `katna-search`, `katna-sync`
and the rest below the apps) build on Windows unchanged. GPUI uses its own
Windows backend (Direct3D) there; Katna's patched Linux backend and renderer
(`vendor/`) are Linux only, and `katna_ui::native` gives the apps one API
for both, with Windows fallbacks: plain-text clipboard, drops as files only,
no compositor blur or frosted panels, no primary selection and no global
menu.

| Linux | Windows |
|---|---|
| XDG base directories | settings in `%APPDATA%\Katna`, everything else in `%LOCALAPPDATA%\Katna\{Data,Cache,State}` (`Paths::from_windows_lookup`) |
| Secret Service (`oo7`) | Credential Manager, generic credentials `<user>.in.invenia.katna`, kept on this computer |
| D-Bus session bus | Katna's own: the reference `dbus-daemon.exe` (built with vcpkg, `ci/windows-dbus.ps1`) beside Katna's programs, started by the first Katna program that needs it (`katna_dbus::session`), on nonce-TCP at 127.0.0.1; its address in `%LOCALAPPDATA%\Katna\State\bus\address`. Every name, call and signal stays as on Linux |
| systemd user unit, D-Bus activation | the `Katna` value of `HKCU\…\Run` runs `katna-mail.exe --background` at login; the bus starts `katna-daemon.exe` on demand from the activation file Katna writes beside its configuration |
| SNI tray, badge on the launcher | notification-area icon with the same menu and the unread count drawn on it (`tray-icon` on a `winit` loop). A taskbar overlay badge needs COM calls the workspace's `unsafe_code = "forbid"` rules out, so it waits for a safe wrapper |
| freedesktop notifications | toasts, under the AppUserModelID Setup registers: the daemon serves `org.freedesktop.Notifications` on Katna's bus itself (`katna_platform::toasts`), so `katna-notify` is unchanged |
| XDG mimeapps (mailto) | `Katna.Mailto` under `HKCU\Software\Classes`, with Capabilities so Katna is listed in Settings > Default apps. Windows only lets people pick the default there, so Katna's "Make default" opens that page |
| file manager menus (§15.2) | Explorer's `*\shell\KatnaMail.Send` and `Directory\shell\KatnaMail.Send` under `HKCU\Software\Classes`, one entry from Setup, rewritten by the daemon with an `ExtendedSubCommandsKey` submenu when there are several accounts. Windows 11 shows it under Show more options (its short menu needs a packaged app's `IExplorerCommand`); Setup also puts Katna Mail in Send to |
| print portal | the PDF opens in the default PDF app to print from there |
| "Open with" portal | Windows' Open with dialog |
| KRunner, GNOME search | no third-party results in Start search; a PowerToys Run plugin later |
| KDE global menu, compositor blur | none |

### 27.2 Setup

Katna Setup.exe is Katna's own installer, written in Rust with GPUI in
Katna's look: one window with its own close button, whose shadow, border
and corners Windows draws (round on Windows 11, square on Windows 10; a
see-through window with a card drawn inside showed as a grey box), the logo, the
choices, Install, a progress bar and Open Katna, in Katna Mail's Mode,
built-in color scheme and accent when its settings file is there
(`katna_ui::schemes`; the desktop's and one's own schemes draw Katna's
palette), else light or dark as Windows is set. The choices: install for just me (the default, into
`%LOCALAPPDATA%\Programs\Katna`, no administrator prompt) or for everyone
(into `%ProgramFiles%\Katna`, with the machine's Start menu, public desktop
and `HKLM` entries; Setup starts a second copy of itself as administrator,
so Windows asks once, and follows its progress through a value under
`HKLM\SOFTWARE\Katna\Setup` that only administrators can write); the
folder (a picked folder gets its own `Katna` folder, so removing Katna never
deletes the user's folder; Setup refuses one that already holds other
files or is a link, and for everyone a folder outside `%ProgramFiles%` gets
Program Files' rules: owned by Administrators, changed only by
Administrators and SYSTEM, read and run by Users, so no user can replace
programs every other user starts; a new install there also needs every
folder above it to be owned by Administrators, SYSTEM or TrustedInstaller
and to give nobody else Delete, Delete subfolders, Change permissions or
Take ownership, since renaming a folder above would swap Katna out too);
a desktop shortcut (off), the Start menu (on)
and start at sign-in (on). An update keeps the folder and what was chosen.
Windows does not let installers pin to the taskbar (Windows 11 only for
apps Microsoft approves), so the last screen says how to pin from Start.
MSI and NSIS installers are not used: their wizard dialogs look like
Windows XP. An MSI for managed deployment can be added later if an
organization needs one.

`packaging/windows/README.md` lists the files. Setup carries Katna's
programs packed with zstd; a newer Setup closes Katna, replaces them and
keeps settings, mail and passwords. Removing Katna asks whether to delete
mail and passwords too. `--quiet` installs without a window (`--all-users`, `--dir`,
`--desktop`, `--no-start-menu`, `--no-autostart`) and `--uninstall`
removes. Setup starts PowerShell and icacls by their full System32 paths
and links with `/DEPENDENTLOADFLAG:0x800`, so files left beside it in
Downloads are never run or loaded as administrator.

Arch Linux is the primary platform: its CI (`ci.yml`) alone gates pull
requests and the Arch package. Ubuntu and Windows are secondary: after
each push to `main` the Secondary workflow (`secondary.yml`) runs their
tests beside Arch without blocking it (a run always finishes, and only the
newest push that arrived meanwhile runs next), and once the Windows tests pass it builds Setup.exe with the faster
`quick` profile (thin LTO) and publishes it as the `windows-latest`
pre-release. The Windows package workflow can also be run by hand
(Actions > Windows package > Run workflow; tick Full build for the
`release` profile). A Claude thread follows Secondary's results and fixes
what breaks there. Without a code-signing
certificate Windows SmartScreen warns on first run; the certificate is the
owner's and goes into GitHub secrets.
