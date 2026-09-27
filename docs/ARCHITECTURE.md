# Katna PIM — Architecture

> Status: **Draft v0.2** (26 September 2026). This is the design reference for
> Katna Mail, Katna Calendar, the Katna background service and Katna Server.
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

- Windows / macOS builds.
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
                                               │  Katna Clock plasmoid (QML/C++)      │
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
│   ├── katna-dbus/            # D-Bus API definitions (in.invenia.katna.Pim1), client + server sides
│   ├── katna-notify/          # notification builder, actions, inline reply, grouping
│   ├── katna-platform/        # portals, desktop detection, settings, tray, badges
│   ├── katna-chrome/          # window decorations (SSD/CSD), theme tokens + presets
│   └── katna-ui/              # shared GPUI components
├── apps/
│   ├── katna-daemon/          # background service (no GUI dependencies)
│   ├── katna-mail/            # GPUI
│   └── katna-calendar/        # GPUI
├── integrations/
│   ├── plasma-calendar-plugin/    # C++ CalendarEventsPlugin → daemon over D-Bus
│   ├── plasma-clock/              # fork of Plasma's digital clock (QML + C++)
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
| `$XDG_DATA_HOME/katna/pim.db` | Shared: accounts, contacts, organizations. |
| `$XDG_DATA_HOME/katna/calendar.db` | Calendar database. |
| `$XDG_DATA_HOME/katna/blobs.db` | Raw messages, zstd-compressed, content-addressed (§5.2). |
| `$XDG_DATA_HOME/katna/attachments/` | Large attachments only (> 256 KB). |
| `$XDG_DATA_HOME/katna/index/` | tantivy index (rebuildable, but expensive, so not in cache). |
| `$XDG_STATE_HOME/katna/crashes/` | Crash reports, plain text, readable by the user (§19.2). |
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
                  blob_hash, snippet, auth_results_json,
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

### 5.4 Shared PIM schema (sketch)

```sql
account          (id, kind, display_name, address, settings_json)  -- kind: imap|jmap|pop3|caldav|carddav|local
organization     (id, name, kind, color, notes, notify_policy)  -- kind: customer|vendor|partner|other
org_alias        (org_id, alias)
org_rule         (org_id, rule_kind, value)               -- domain | subdomain | address
contact          (id, display_name, vcard_uid, notes)
contact_address  (contact_id, email_norm)
org_member       (org_id, contact_id)
suggestion       (id, kind, payload_json, state)          -- pending | accepted | dismissed
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
- **Level-1 sync (`katna_sync::engine`):** per folder, SELECT with
  CONDSTORE, reset on a new UIDVALIDITY, fetch flags changed since the stored
  HIGHESTMODSEQ, fetch headers of new UIDs in chunks of 500 (committed chunk
  by chunk), and compare UID lists only when the message count does not add
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
  body on request) is a later option. Discovery does not look for POP3
  servers yet, so `AddPop3Account` needs the server.
- **Gmail / Microsoft:** OAuth2. Google's restricted scope for full mail
  access requires app verification and a yearly security assessment.
  Launch with generic IMAP, Fastmail/JMAP and app-password accounts first.
- **Account setup** (task 1.2, `katna_sync::autoconfig`, D-Bus
  `DiscoverAccount`): the user gives an address and the daemon finds the
  servers, in Thunderbird's order. First built-in settings for Gmail,
  Yahoo, iCloud and Fastmail. Then the provider's own `config-v1.1.xml`
  (`https://autoconfig.DOMAIN/…` and `https://DOMAIN/.well-known/…`) and
  Thunderbird's ISPDB, fetched at once; the provider's file wins. Then DNS
  SRV (`_imaps`, `_imap`, `_submissions`, `_submission`; RFC 6186 and
  8314), then the ISPDB entry of the MX host's domain (hosted mail such as
  Google Workspace), then probing `imap.`, `mail.` and `smtp.DOMAIN` on
  993/143 and 465/587 for a mail greeting. Files come only over HTTPS; TLS
  beats STARTTLS beats plain; servers that only take OAuth2 are skipped.
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
  15.0 MB (then 14.3 MiB of a 15 MiB budget; it is 20 MiB now, §17.2).
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
  it shuts down gracefully and `exec`s the new binary (same PID, so systemd
  keeps tracking it). No `systemctl --user restart` after an update.

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
| Send later | `{send_at}` | Send the draft (daemon, or Katna Server if enabled and the machine is off). |
| Snooze | `{until}`, thread moved to a "Snoozed" folder | Move back to inbox, mark unread, notify. |
| Reminder | `{remind_at, if_no_reply: true}` | Notify if nobody replied. |
| Tracking | `{tracking_id, links[], events[]}` | — (events arrive from the server) |

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
- **Send later.** Schedule send queues the message with a delay until
  the chosen time; the daemon holds it like an undo-send delay and sends
  it on time with or without the app. The app lists queued messages
  (`Outbox`, `OutboxChanged`) and counts one as scheduled when it is
  still queued, has no error and is due later than the undo-send delay
  would put it. Cancel is `UndoSend`, as for undo send.
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
background, padding, border, radius and width, table rows as rows of cells,
button-like inline boxes, and images. Only inline `style` attributes and
presentational attributes are read; `<style>` sheets are ignored. The walk
is the sanitizer: scripts, style sheets, forms, frames, objects, SVG and
unknown elements never reach the tree, hidden preheaders are dropped, link
targets are limited to `http`, `https` and `mailto`, and the tree is capped
in depth and size. `cid:` and `data:` images come from the message.
In a light theme a message that sets its own colors is drawn on its own
page; one that does not follows the app's colors. In a dark theme the
message's colors are remapped (`window/dark.rs`): white becomes the reading
pane, other light backgrounds become dark ones of the same hue as dark by
eye as they were light, dark backgrounds stay, and text that falls under
3:1 contrast on its new background has its lightness flipped and raised to
4.5:1. Images are not changed. A `text/plain` part
that is really an HTML document is rendered as HTML.

Remote content is blocked by default. Tracking pixels (tiny images and
known open-tracking paths) are dropped. A banner offers "Show images" (this
message) and "Always show from this sender" (kept in
`$XDG_CONFIG_HOME/katna/trusted-senders`). Images are fetched by the daemon
(`FetchImage`, `https` only, `http` upgraded, at most 8 MB, checked to be an
image by its bytes); the app never uses the network.

Message text can be selected and copied as in a browser (`window/select.rs`):
each run of text a body draws records its layout, so a pointer position maps
to a place in the text; the selection is drawn as a highlight on those runs.
Drag, double- and triple-click, Shift+click, Ctrl+A and Ctrl+C (once the
text was clicked) and a right-click Copy work in plain and HTML mail; the
selection also goes to the primary selection for middle-click paste.

Sender pictures load without asking, since they are looked up by domain,
never by message, and kept for a week, so they cannot tell anyone that a
message was read. The daemon's `SenderPicture` looks up the organization's
BIMI logo (`default._bimi` TXT record, SVG) and falls back to the largest
icon its home page names (`<link rel="icon">`, `apple-touch-icon`), then
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
because it made every account look the same. There is no OAuth, so a
provider's profile photo (Google's needs a Google sign-in) is out of
reach; when OAuth2 comes, it goes after the picked picture in
`own_picture` (`window/remote.rs`). Libravatar or Gravatar could come
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
- Templates later.

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
  KDE). Switching keeps the window's size on screen. Where the desktop never
  draws frames (GNOME on Wayland) the choice is replaced by a note.
  `KATNA_DECORATIONS=auto|server|client` still overrides it, for testing.
- *Blurred background*: the window's page color becomes translucent
  (`katna_chrome::tokens::blur_alpha`: 75 % light, 80 % dark) and the
  compositor blurs what is behind it: `ext_background_effect_v1` (KWin 6.7),
  else `org_kde_kwin_blur`, and `_KDE_NET_WM_BLUR_BEHIND_REGION` on X11.
  The blur region is the frame less its rounded corners; the CSD shadow is
  painted only outside the frame, so it cannot darken the window. Cards
  and dialogs stay opaque, so text keeps its contrast. Offered only where
  the compositor can blur (`gpui_linux::compositor_blur`); elsewhere the
  switch is shown off with the reason. The compose pop-out stays opaque
  (it is all message).
- The same switch frosts floating panels in every window: menus (the
  right-click menu and its folder list, dropdowns), Search options and its
  date popover, and the account menu. Their color is 78 % opaque over a
  20 px blur of what is behind them in the window
  (`katna_mail::widgets::raised`, `katna_ui::frost`). GPUI has no backdrop
  filter, so Katna's copy of its renderer (`vendor/gpui-pre-wgpu`) adds
  one: a quad marked through its border color is drawn over a dual Kawase
  blur of the frame under it, clamped to the quad (as CSS
  `backdrop-filter`). The same renderer draws every drop shadow only
  outside its element, as CSS does, so a translucent panel or frame keeps
  one plain box shadow that follows its rounded corners. Where the
  window's surface cannot be copied from, panels stay opaque.

**Window state.** The mail window opens as it closed: its size, maximized
state and place (`katna_chrome::placement`, saved in
`$XDG_STATE_HOME/katna/mail-window.toml` when the app quits). The state
belongs to one run of the Katna service, named by the daemon's process id
and start time (which survive its re-exec after an update); once the
service quits (the tray's Quit, logging out), the next start opens the
window at its default size and place.

- Wayland does not let a window place itself. Katna's copy of GPUI
  (`vendor/gpui-pre-linux`) joins the window to an
  `xdg-session-management-v1` session (KWin from Plasma 6.7), and the
  compositor puts it back where it was; a fresh start removes the old
  session and begins a new one. Without the protocol only the size and
  maximized state come back.
- X11: the window opens exactly at its old position (user-specified
  position, static gravity). KWin adds the CSD shadow margin itself on X11,
  so the saved frame is asked for as is.

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
- **Counts.** Folder totals come from the location index at startup
  (5 ms for 100,000 messages). Unread counts need each message's flags
  (about 100 ms per 100,000 unread messages), so they are counted on a
  background thread and appear when ready. The daemon should keep counts
  per folder later.
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
  it and gives each account's unread count); the choice is kept in
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
  again on it. The text of each message is printed, with sender, date,
  recipients and attachment names; pictures and HTML styling are not, and
  there is no font fallback for scripts the UI font lacks. Without a print
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
  send, offline mail,
  new-mail notifications and their sound, starting at login, tray and
  badge), Inbox, Accounts, Subscription, Appearance (reading pane,
  density, scaling, theme, desktop colors, app names, sender pictures,
  Important markers, message width, dark colors for HTML mail, attachment
  previews), Shortcuts, Default apps (where each kind of attachment
  opens, and showing saved files in their folder), Folders & rules,
  Compose (signatures, plain text, spelling and its language, templates
  to come), MCP server, User feedback (turning crash reports and feedback off at any
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
  still wait to be asked for; and the new-mail sound is the notification's
  `sound-name` hint, or `suppress-sound` when off. Katna never tracks
  whether others open mail, so Mailspring's open and click tracking
  settings have no counterpart.
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
- **Removing an account, deleting all data.** Settings → Accounts
  (`window/accounts.rs`; also "Manage accounts" in the account menu) lists
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
- **Adding an account.** A dialog shaped like a web sign-in
  (`window/add_account.rs`): the address first; the daemon looks for the
  servers (`DiscoverAccount`, §6), and the next step asks for the password
  under a chip with the address, with "Show password", an optional name
  for the From line and where the servers were found. Gmail, Yahoo, iCloud
  and AOL addresses get a note that they need an app password. When
  nothing is found, or from "Server settings", the servers are entered by
  hand: host, port and SSL/TLS, STARTTLS or none for IMAP and SMTP, and
  the username. `AddImapAccount` checks the login before saving; a refused
  password is shown under the field. It opens from the first-start pages
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
  archive, delete, spam, read, star, move to, find emails from the
  sender) acting on the ticked lines or the clicked one. The "select all
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
  (checked every 2 s). A quick setting, *Desktop colors* (on by default,
  `mail.desktop_colors`), turns this off.
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
  top bar (the window's own controls stay usable): a dark page with a bar
  naming the file, "Open with another app" and Save; arrows (and ←/→) go
  through the message's other attachments; a pill at the foot zooms
  (−/+/0, 25 %–400 %, 100 % fits the window) and counts PDF pages.
  Escape closes it. It is dark in light and dark themes alike.
  - **PDF:** `hayro` (pure Rust, CPU, Apache-2.0/MIT) draws the pages.
    Only pages on screen (and one either side) are drawn, at the zoom and
    the screen's scale, one at a time on a background thread; pages far
    off screen are freed. Password-protected PDFs are not opened yet (the
    viewer says so and offers the other app).
  - **Pictures:** PNG, JPEG, GIF, WebP, BMP, TIFF through the `image`
    crate GPUI already uses, turned upright by their EXIF orientation and
    scaled to at most 4096 px; animated GIFs and SVG are drawn by GPUI.
  - **Text** (`text/*`, JSON, logs, code by extension): monospace, the
    first 512 KB and 10,000 lines.
  - **Spreadsheets:** Excel (xlsx, xlsm, xlsb, xls) and OpenDocument (ods)
    read by `calamine` (pure Rust, MIT), and CSV/TSV (separator guessed:
    comma, semicolon, tab or bar; also CSV sent as `text/plain`). A grid on
    white with column letters kept at the top, row numbers, numbers on the
    right, and a tab per sheet at the foot. Values only: formulas show
    their saved result, dates show as dates; no cell colors, merged cells
    or charts. Up to 20,000 rows, 256 columns and 2 million cells.
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
| Phone   | under 600 px  | No app rail: the apps sit in a bar along the bottom. The search box is a pill across the top bar with the menu button and account picture inside it (settings move to the drawer). The list is edge to edge, three lines a message with the sender's picture, which ticks the line when tapped; the inbox tabs move to the drawer. Compose floats at the bottom right; it folds to its pencil as the list scrolls down and grows back after a few steps up (or at the top). The search row and the list toolbar slide up out of sight once the list has scrolled past them, and come back as soon as it turns back up (or at the top); the list keeps still on screen while they move. An open conversation slides in over the list and the bottom bar sinks away; its messages use the room under the sender's picture, from the picture's left edge, and Reply, Reply all and Forward share the width equally. Composing takes a sheet over the whole window. Quick settings and the Settings page each fill the window between the top bar and the bottom bar, with no Compose button over them; the Settings page's section tabs stay on one line that scrolls sideways. |

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

- A **language button** in the top bar, just left of Settings (the gear),
  with the same size, hover and one shared gap (`TOP_BAR_GAP`) as the
  other top-bar buttons. It shows the current language's flag and a small
  chevron; its tooltip names the language ("Language: বাংলা, following
  the system" with System default).
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

The daemon does not format dates, so it links only Fluent (its 20 MB
budget): the counts in its notifications and tray tooltip are written in
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

## 14. D-Bus API (`katna-dbus`)

### 14.1 Interface `in.invenia.katna.Pim1` (object `/in/invenia/katna/Pim1`, bus name `in.invenia.katna.Daemon`)

Sketch — versioned by the interface name; breaking changes create `Pim2`.

| Kind | Members |
|---|---|
| Mail commands | `OpenMessage(id)`, `FetchBody(id)`, `SetFlags(ids, flags)`, `Move(ids, folder)`, `Archive(ids)`, `QueueSend(draft)`, `UndoSend(id)`, `ReplyAll(id, text)` |
| Search | `Search(query, limit) → results` (used by KRunner, GNOME search, apps) |
| Calendar | `EventsInRange(start, end) → events`, `CreateEvent(ical)`, `UpdateEvent(uid, ical)`, `DeleteEvent(uid)` |
| Contacts / orgs | `FindContacts(text)`, `Organizations()` |
| Sync | `SyncNow(account?)`, `SetForegroundFolders(ids)`, `Status() → per-account state` |
| Signals | `MessagesChanged(ids)`, `FoldersChanged`, `EventsChanged(range)`, `SyncStatusChanged`, `UnreadCountChanged(n)` |

Implemented so far (`katna_dbus::PimProxy`): `Accounts() → a(xssssx)`
(id, kind, name, address, state, detail, last sync),
`DiscoverAccount(address) → (account, source)`, `AddImapAccount(account,
password) → id`, `AddPop3Account(account, password) → id` (with
leave-on-server, days to keep, and delete-with-local),
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
`DeleteMessages(ax)`, `ArchiveMessages(ax)`, `QueueSend(x account, ay
message, u delay) → id`, `UndoSend(id) → b`, `DiscardSend(id) → b`,
`Outbox() → a(xxxsxss)` (id, account, message, subject, send at, state,
detail; states in `katna_dbus::send_state`), `FetchImage(url) → ay` and
`SenderPicture(address) → ay` (images for the reading pane, §12), and the
signals
`AccountsChanged`, `SyncStatusChanged(id)`, `MailChanged(id)` and
`OutboxChanged(id)`. `MailChanged` carries the
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
  avatar or organization logo), `sound-name=message-new-email`,
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

- After each sync, unread mail that reached an account's inbox (Primary tab,
  or not classified yet) since the daemon last looked, and dated within the
  last two days, becomes one notification per account and sync. One message
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
  a sync or from a change made in the app.
- Setting `notifications.new_mail` (default on); `ReloadConfig` applies it.
- Not yet: inline reply, sender pictures (`image-data`), per-organization
  policy.

### 15.2 Taskbar, tray and global menu

The count and the tray live in `katna-daemon`, so they stay while the app
is closed; the protocol code is in `katna-platform` (`launcher`, `tray`,
`dbusmenu`, `icon`), written on zbus rather than with `ksni`.

- **Unread count** on Katna Mail's taskbar or dock icon:
  `com.canonical.Unity.LauncherEntry` `Update` signals for
  `application://in.invenia.katna.Mail.desktop` from
  `/in/invenia/katna/Daemon/LauncherEntry`. The number is the unread
  messages in every account's Inbox, the same as next to Inbox in the app,
  recounted half a second after mail changes. Plasma's task manager shows
  it; on GNOME, Ubuntu Dock, Dash to Dock and Dash to Panel do (the stock
  GNOME dash shows no counts). Setting `general.unread_badge` (default on).
- **Tray icon**: a StatusNotifierItem under its own name
  (`org.kde.StatusNotifierItem-PID-N`), registered with
  `org.kde.StatusNotifierWatcher` again whenever the watcher restarts.
  Plasma shows it natively; GNOME needs the AppIndicator extension (on by
  default on Ubuntu). The icon is the app icon pre-rendered at each tray
  size (`crates/katna-platform/icons/`, from `packaging/icons/render.py`)
  with a red badge drawn in code with the count, `99+` above 99, since the
  protocol takes pixels and an SVG renderer would grow the daemon. Left click raises the
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
| Event | title, attendees, location | Open event |

Flatpak: KRunner D-Bus runners are designed to work with sandboxed apps;
verify that Flatpak exports the `krunner/dbusplugins` file. Distro
packages install it directly.

### 15.4 Plasma calendar (clock) integration

Plasma's digital clock popup only displays events from calendar-events
plugins (Akonadi uses the "PIM Events" plugin from kdepim-addons). It has a
hidden **"Add…"** button, shown only when an events plugin is enabled **and**
a default `text/calendar` application exists; it launches that app without a
date. Events in its agenda have no click or right-click actions.

Katna integrates in three layers:

**A. Calendar-events plugin (`integrations/plasma-calendar-plugin`)**

- A small C++ `CalendarEvents::CalendarEventsPlugin` that asks
  `katna-daemon` for `EventsInRange` over D-Bus and listens to
  `EventsChanged`.
- Katna Calendar registers as the `text/calendar` handler, so the stock
  clock shows **"Add…"** for Katna without any fork.
- Runs inside `plasmashell`: fully asynchronous, never blocks, minimal code
  (a crash here crashes the desktop shell).

**B. Katna Clock — fork of the official clock (`integrations/plasma-clock`)**

- Source: `plasma-workspace/applets/digital-clock` (about 5,900 lines together
  with the calendar component; GPL-2.0-or-later / LGPL / KDE-accepted GPL).
- Renamed to avoid clashes in the shared `plasmashell` process:
  applet `org.kde.plasma.digitalclock` → `in.invenia.katna.clock`;
  QML module `org.kde.plasma.private.digitalclock` → `in.invenia.katna.private.clock`.
- Declares `X-Plasma-Provides: org.kde.plasma.time, org.kde.plasma.date`, so
  it appears in the clock's **"Show Alternatives"** menu (two-click switch).
- Keeps importing the shared `org.kde.plasma.workspace.calendar` component
  (identical look, gets upstream fixes); copied only if upstream changes
  break us.
- Katna features in new files (`KatnaAgenda.qml`, `QuickAddEvent.qml`, a C++
  `KatnaBridge` for D-Bus), with minimal edits to upstream files:
  - click a day → quick-add form;
  - click an event → details; right-click → edit, delete, join meeting;
  - drag to reschedule;
  - organization badges ("Meeting with Acme") and related emails.
- Still reads events through the plugin system (A), so holidays and other
  plugins keep working.

**C. Upstream contributions to Plasma**

- "Add…" opens the calendar app on the selected date.
- Clicking an event opens it in the calendar app.
- An optional plugin hook for "create/edit event" actions.

Each accepted change shrinks the fork and improves the stock clock.

**GNOME:** the top-bar calendar reads only Evolution Data Server; showing
Katna events there needs an EDS backend (C). Deferred.

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
| Katna Clock (fork) | QML + C++ | Upstream code + small Katna additions |

They only display data and forward actions to `katna-daemon`; no business
logic lives there.

## 16. Katna Server (`server/katna-server`)

Optional. Self-hostable (container image) and offered as a hosted Pro service.

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
  on the server (or the URL is HMAC-signed), so the server can never be
  used as an open redirect.
- **Event quality:** label Apple Mail Privacy Protection fetches as "maybe
  opened"; label clicks from security scanners (data-center IPs within
  seconds of delivery) as "scanner"; Gmail/Outlook proxies hide location.
- **Privacy:** the server stores random IDs and events only — no subject,
  no recipients, no content. The app keeps the ID → message mapping.
- **Deliverability:** dedicated tracking domain; custom domains for
  business users (`t.customer.com`).
- **Consent:** tracking is opt-in per message and off by default. Legal
  review is needed before selling in the EU (GDPR/ePrivacy). Read receipts
  (MDN) are offered as a consent-based alternative.
- Tracking events arrive at the daemon over the server's event stream and
  can raise notifications ("Acme opened *Proposal v2*").

### 16.2 Stack

`axum` + PostgreSQL; WebSocket/SSE delta stream to `katna-daemon`; a
scheduler for server-side actions; shared crates with the apps where useful.

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
| `katna-daemon` binary | ≤ 20 MiB (21 MB) |
| Idle CPU (app and daemon) | ≈ 0 %; no periodic wake-ups beyond IDLE renewals |
| Cold start to usable inbox | < 500 ms |
| Search latency | p50 < 20 ms, p99 < 50 ms on 1M messages |
| Daemon memory | Measured and tracked in CI; budget set after first prototype |

With sync, bodies, the op queue, sending and the search indexer,
`katna-daemon` is 15.6 MB. tantivy is the biggest part. Its budget was
15 MiB until sending came in; it is 20 MiB (September 2026) so features
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

- CalDAV sync via `io-webdav` (discovery, sync-token, multiget) in the
  daemon; local `calendar.db`; also local-only calendars.
- `calcard` for iCalendar/vCard; `rrule` for recurrence expansion;
  `jiff` for time zones, including embedded `VTIMEZONE` definitions.
- Correct handling of recurrence exceptions (`RECURRENCE-ID`, `EXDATE`).
- Invitations (iTIP/iMIP) shared with Katna Mail: accept/decline from mail.
- Alarms fire from the daemon as notifications (§15.1).
- Views: day, week, month, agenda; organization filter.
- Desktop: Plasma clock plugin and Katna Clock (§15.4); KRunner results (§15.3).
- Server quirks: test against Google, Nextcloud, Radicale, Fastmail, Stalwart.

## 19. Security and privacy

- TLS only via `rustls`; no plain-text auth without an explicit warning.
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
- **Scrubbing.** Before a report is written, the home directory becomes
  `~`, the user name, host name and machine ID become `<user>`,
  `<host>`, `<machine>`, and anything shaped like an email address becomes
  `<email>`. The same scrubber runs again before anything is sent (Part 2),
  so a report edited by hand is checked twice.
- **Readable tracebacks.** Release binaries are stripped, which leaves
  Rust's backtrace as `<unknown>` frames. So a panic report also lists
  every frame as `module + 0xoffset` (the `backtrace` crate's return
  addresses against `/proc/self/maps`) and the executable's GNU build ID;
  `addr2line -f -C -e <unstripped binary> <offset - 1>` turns them into
  functions and lines, and later Sentry does the same with the debug files
  CI uploads. Keeping symbol names in the daemon costs 3.1 MB and would
  break its 20 MB budget (§17), so it stays stripped. Katna Mail keeps its
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

### 21.2 Update channels and safe updates (planned, not built)

Planned 26 September 2026; nothing here is built yet, because the basic
apps come first. Today the only update path is the `arch-latest`
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

#### Who updates what

Katna never replaces files that a package manager owns.

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
