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
| D-Bus and desktop | `zbus`, `ashpd` (portals), `oo7` (Secret Service), `ksni` (tray) | Notifications implemented directly on `org.freedesktop.Notifications` via `zbus` (actions, inline reply, activation tokens). |
| Icons | `freedesktop-icons` + `resvg` | |
| Spell check | `spellbook` | Hunspell dictionaries. |
| Mail rules on the server | `sieve-rs` (compile) + ManageSieve | |
| OpenPGP (later) | `sequoia-openpgp` or `pgp` (rPGP) | |
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
                  blob_hash, snippet, auth_results_json)
message_location (message_id, folder_id, uid)        -- one message, many folders/labels
participant      (message_id, role, email_norm, domain, display_name)
                                                      -- role: from|to|cc|bcc|reply_to|sender
attachment       (id, message_id, part_id, filename, mime, size, blob_hash NULL)
thread           (id, account_id, subject_norm, last_date, message_count, flags_summary)
op_queue         (id, account_id, op_json, state, attempts, next_try_at)
outbox           (id, draft_message_id, send_at, state, per_recipient BOOL, attempts)
notification     (notif_id, message_ids, account_id, created_at)   -- to close/update later
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
- **Network and power:** reconnect on network changes (NetworkManager or the
  portal network monitor), after resume (logind `PrepareForSleep`), with
  exponential backoff; pause heavy background work on metered connections.

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
  up. Headers come from `BODY.PEEK[HEADER.FIELDS (…)]` and are decoded by
  the same parser as the importer; INTERNALDATE stands in for a missing
  `Date`. Each server copy of a message is its own row for now; merging
  copies (Gmail labels) comes with threading (task 1.7). `has_attachments`
  is guessed from `multipart/mixed` until `BODYSTRUCTURE` is parsed.
- **Level 3 so far (`katna_sync::bodies`):** after each full sync, and after
  each inbox catch-up, the worker fetches `BODY.PEEK[]` for messages in the
  offline window (default: the last 30 days, up to 10 MB each), newest
  first, 25 per command. The raw message goes to the blob store; the
  snippet and attachment flag are recomputed from it and `body_state` is
  set to 2. `FetchBody(id)` on D-Bus downloads any other message through
  the account's worker. Level 2 (text only) and eviction come later.
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
  - Delete moves to the `\Trash` folder, or expunges when the message is
    already there or there is no trash. Archive moves to `\Archive` (or
    Gmail's `\All`).
  - A refused operation is retried after 60 s. After three refusals it is
    marked failed (kept for inspection) and undone locally: moves at once,
    flags by forgetting the folder's HIGHESTMODSEQ so the next sync reads
    them again. A broken connection keeps the operation queued.
  - Imported (`local`) accounts only change in the store.
- **POP3:** our own small client (UIDL tracking, leave-on-server option,
  `TOP` for header preview). POP3 mail is always fully local.
- **Gmail / Microsoft:** OAuth2. Google's restricted scope for full mail
  access requires app verification and a yearly security assessment.
  Launch with generic IMAP, Fastmail/JMAP and app-password accounts first.

### 6.5 Threading

JWZ algorithm over `Message-ID` / `References` / `In-Reply-To`. Use Gmail
`X-GM-THRID` when available. Subject-only grouping is a limited fallback.

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
in:inbox  label:x  is:unread  is:starred  before:2025-01-01  after:  newer_than:30d
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
  are UTC days; `before:` excludes the day, `after:` includes it. Unknown
  `word:value` is plain text; broken parentheses are ignored; nesting is
  limited to 32 levels.
- **As you type.** `Query::parse_as_you_type` treats a final unfinished
  word as a prefix (`budg` finds `budget`; `"natural g` keeps the phrase
  order), expanded to at most 16, 32 or 64 index words for one, two or
  more letters. On the 500k synthetic corpus one-letter prefixes stay
  under 20 ms p99.
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
  15.0 MB (14.3 MiB of its 15 MiB budget).
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
- Tray icon and unread badge (optional, §15.5).

### 9.2 Lifecycle

| Environment | How it starts |
|---|---|
| systemd | `katna-daemon.service` (systemd user unit), started at login when "run in background" is on. |
| Any session | **D-Bus activation** (`in.invenia.katna.Daemon.service`): starts on demand when an app, KRunner, the clock plugin or a notification action calls it. |
| No systemd | XDG autostart `.desktop` file. |
| Flatpak | **Background portal** (`RequestBackground` with autostart). KDE and GNOME both implement it; GNOME lists it under "Background Apps". |

- User settings: "Keep running in background" (default on), optional tray
  icon, and a real "Quit" (stops the daemon until next login or activation).
- Single instance, enforced by owning the D-Bus name.
- Graceful shutdown: finish in-flight sends, flush the index, close IMAP sessions.

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

### Composer

- **Phase 1:** plain text and Markdown compose, sent as text + HTML.
- **Phase 2:** WYSIWYG rich-text editor in GPUI (significant work).
- Templates, per-identity signatures, spell check (`spellbook`), inline images.

## 13. UI

### 13.1 Window decorations (`katna-chrome`)

| Desktop | Mode | Details |
|---|---|---|
| KDE (KWin) | **Server-side** (xdg-decoration) | KWin draws the real Breeze (or user-chosen) title bar, buttons and shadow. Toolbar sits below the title bar, like other KDE apps. |
| GNOME (Mutter) | **Client-side** (must be requested explicitly, or the window has no title bar) | Adwaita-style header bar; button layout from `org.gnome.desktop.wm.preferences button-layout` (default: close only); own shadow in a transparent margin with correct window geometry; larger shadow when focused; rounded corners only when floating (none when maximized/tiled); resize edges in the shadow area; double-click maximize, right-click window menu. |
| Others (Sway, Hyprland, …) | SSD if offered, else minimal CSD; none when tiled | |
| X11 | SSD by default | CSD shadows need a compositor. |

Detection via `XDG_CURRENT_DESKTOP`, with a user override
(`KATNA_DECORATIONS=auto|server|client` until the setting exists). The frame
is drawn from the negotiated mode, not the requested one: GPUI falls back to
CSD when the compositor has no xdg-decoration. Validated by spike S1
(`docs/spikes/S1-window-chrome.md`).

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
(id, kind, name, address, state, detail, last sync), `AddImapAccount(account,
password) → id`, `SetPassword(id, password)`, `RemoveAccount(id) → b`,
`SyncNow(id)` (0 for every account), `FetchBody(message)`,
`SetFlags(ax messages, as add, as remove)` (flag names `seen`, `answered`,
`flagged`, `draft`, `forwarded`), `MoveMessages(ax, folder)`,
`DeleteMessages(ax)`, `ArchiveMessages(ax)`, and the signals
`AccountsChanged`, `SyncStatusChanged(id)` and `MailChanged(id)`. `MailChanged` carries the
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

### 15.2 Taskbar and tray

- Unread count on the Plasma task manager icon via
  `com.canonical.Unity.LauncherEntry` (also Dash-to-Dock on GNOME).
- Optional tray icon (`ksni`, StatusNotifierItem): unread count, compose,
  pause sync, quit.

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
about 2 MB (the budget in `ci/size-budgets.txt` is 30 MiB = 31.5 MB).

### 17.2 Targets (to be verified on real hardware)

| Metric | Target |
|---|---|
| Katna Mail binary | ≤ 30 MB (estimate: 20–30 MB with GPUI Kit, Pimalaya, own code) |
| `katna-daemon` binary | ≤ 15 MB |
| Idle CPU (app and daemon) | ≈ 0 %; no periodic wake-ups beyond IDLE renewals |
| Cold start to usable inbox | < 500 ms |
| Search latency | p50 < 20 ms, p99 < 50 ms on 1M messages |
| Daemon memory | Measured and tracked in CI; budget set after first prototype |

With sync, bodies, the op queue and the search indexer, `katna-daemon` is
15.6 MB of its 15 MiB budget. tantivy is the biggest part. To get there,
crates that are not hot are built with `opt-level = "s"` (root
`Cargo.toml`): D-Bus (zbus, zvariant, oo7, ashpd), IMAP parsing and
regex.

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
- OpenPGP (later): Sequoia or rPGP. S/MIME: later, Rust support is weaker.
- The search index contains message text: it is covered by the same disk
  protection as the mail itself. Decrypted PGP mail is not indexed by default.

## 20. Dependency policy

- **GPUI:** pin exact `gpui-pre` and GPUI Kit versions; GPUI types only in
  `katna-ui`, `katna-chrome` and the GUI apps.
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
does something. See `packaging/README.md`.

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
| Scope | Burnout, never shipping | Strict phases with "done when" criteria |

## 25. Open decisions

1. HTML renderer for phase 2 (§12).
2. Katna Server hosting and pricing model; Katna Server license (GPL-3.0 or AGPL-3.0).

Decided:

- License: GPL-3.0-or-later (§22).
- Repository: `QuakeString/katna`, one monorepo (§3); default branch `main`.
- App ID prefix: `in.invenia.katna` (domain `katna.invenia.in`); defined
  only in `katna_core::ids`.
- Message storage: SQLite for metadata and compressed raw messages;
  files only for large attachments (§5.2).
- Rust toolchain: latest stable (`channel = "stable"`).
- Test and support matrix: Arch Linux (latest Plasma and GNOME) and
  Ubuntu 26.04 LTS (GNOME) / Kubuntu 26.04 (Plasma). The Plasma
  integration supports the Plasma versions of these two.
