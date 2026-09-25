# Katna PIM — Architecture

> Status: **Draft v0.1** (September 2026). This is the design reference for
> Katna Mail, Katna Calendar and Katna Server. Nothing here is built yet;
> sections marked **Decision needed** are open.

## 1. Goals

Katna is a modern, fast personal-information-management suite for Linux,
written in Rust, as an alternative to KDE PIM (KMail, Kontact, Akonadi,
Merkuro).

| Goal | What it means in practice |
|---|---|
| **Best-in-class search** | Instant (<50 ms) full-text search across very large mailboxes, including mail that is not fully downloaded. |
| **Organizations first** | Mail is grouped by company/customer/party, not only by address. Searching a company name finds all its mail, even from personal addresses. |
| **Looks native on KDE and GNOME** | Real KWin decorations on KDE; Adwaita-style header bar, shadows and button layout on GNOME. |
| **Gmail-class features** | Undo send, send later, snooze, reminders, labels, threading, rules, templates, one-click unsubscribe. |
| **Sales/pro features** | Open and link tracking, activity dashboard (via Katna Server). |
| **Local-first and private** | Works offline, data stays on the machine, incoming trackers are blocked. |
| **Rust only** | All Katna code is Rust. Only unavoidable system C libraries are linked (Wayland, xkbcommon, fontconfig, Vulkan loader). |
| **Packaged everywhere** | Flatpak, .deb, .rpm, AUR, AppImage, Nix, others. |

### Non-goals (for now)

- Windows / macOS builds.
- Being a general Akonadi replacement that other apps plug into.
- Exchange (EWS) support in the first releases.

## 2. System overview

```
┌──────────────────────┐   ┌──────────────────────┐
│     Katna Mail       │   │    Katna Calendar    │     apps (GPUI)
└──────────┬───────────┘   └──────────┬───────────┘
           │   shared UI crates: katna-ui, katna-chrome, katna-platform
┌──────────┴──────────────────────────┴───────────┐
│  Engines: katna-sync · katna-search · katna-org │     Rust library crates
│           katna-meta · katna-dav · katna-render │
├─────────────────────────────────────────────────┤
│  katna-store: SQLite (WAL) + blob store + index │     local data
├─────────────────────────────────────────────────┤
│  Protocols: Pimalaya io-* (IMAP, SMTP, JMAP,    │     sans-I/O libraries
│  WebDAV/CalDAV/CardDAV) + own POP3              │     + our I/O driver (rustls)
└──────────────────────┬──────────────────────────┘
                       │ HTTPS / WebSocket (optional)
              ┌────────┴────────┐
              │  Katna Server   │  tracking · metadata sync · scheduled actions
              └─────────────────┘
```

Principles:

1. **The UI never talks to the network.** It reads from the local store and
   sends *commands* to the engines. Engines update the store; the UI reacts
   to store change events.
2. **Local-first, optimistic.** User actions (archive, flag, move) apply to
   the local store immediately and are queued for the server.
3. **The search index and caches are disposable.** Everything can be rebuilt
   from the SQLite database and the blob store.
4. **Protocols are behind our own traits.** Pimalaya and GPUI types never leak
   into engine or store APIs, so either can be replaced.

## 3. Repository and crate layout

One Cargo workspace (monorepo) for all apps, shared crates and the server.

```
katna/
├── Cargo.toml                 # workspace, shared dependency versions
├── crates/
│   ├── katna-core/            # config, XDG paths, accounts, secrets, errors, logging
│   ├── katna-store/           # SQLite schema + migrations, blob store, change events
│   ├── katna-meta/            # metadata-with-expiration layer (§9)
│   ├── katna-sync/            # account workers, IMAP/JMAP/POP3/SMTP, outbox, op queue
│   ├── katna-search/          # tantivy index, query language, ranking
│   ├── katna-org/             # organizations, matching rules, suggestions
│   ├── katna-render/          # HTML sanitizing and message rendering
│   ├── katna-dav/             # CalDAV/CardDAV sync, iCalendar/vCard, recurrence
│   ├── katna-platform/        # portals, desktop detection, settings, notifications, tray
│   ├── katna-chrome/          # window decorations (SSD/CSD), theme tokens + presets
│   └── katna-ui/              # shared GPUI components
├── apps/
│   ├── katna-mail/
│   └── katna-calendar/
├── server/
│   └── katna-server/          # axum; tracking, metadata stream, scheduled actions
├── tools/
│   ├── katna-search-cli/      # index/query from the terminal, benchmarks
│   └── katna-bench/           # Enron-corpus benchmarks
├── packaging/                 # flatpak, deb, rpm, aur, appimage, nix, desktop/appstream files
└── docs/
```

Crate dependency direction (no cycles, UI at the top):

```
apps → katna-ui, katna-chrome, katna-platform → engines → katna-store → katna-core
```

`katna-search-cli` depends only on engines and the store. This keeps search
and sync testable and benchmarkable without a GUI.

## 4. Technology choices

| Area | Choice | Notes |
|---|---|---|
| GUI | **GPUI** via `gpui-pre` + **GPUI Kit** (formerly gpui-component) | Official `gpui` on crates.io is stuck at 0.2.2 (Blade renderer). `gpui-pre` tracks Zed's wgpu renderer and AccessKit. Pin exact versions (§17). |
| Async / I/O | GPUI executors in apps; `smol`-compatible I/O in engines; `rustls` | Pimalaya is sans-I/O, so we choose the runtime. Avoid pulling in Tokio in the apps. The server uses Tokio (axum). |
| Protocols | Pimalaya `io-email`, `io-imap`, `io-smtp`, `io-jmap`, `io-webdav`, `io-calendar` | Light forks where needed (§17). POP3 is our own (not in `io-email`). |
| MIME | `mail-parser`, `mail-builder` | |
| Database | SQLite via `rusqlite` (WAL mode) | |
| Search | `tantivy` | Plus `rust-stemmers`, `lindera`/`jieba` tokenizers, `whatlang`. |
| Blob compression / hashing | `zstd`, `blake3` | |
| HTML safety | `ammonia` | |
| Calendar data | `calcard` (iCalendar + vCard), `rrule`, `jiff` (time zones) | |
| Desktop integration | `zbus`, `ashpd` (portals), `oo7` (Secret Service), `notify-rust`, `ksni` (tray) | |
| Icons | `freedesktop-icons` + `resvg` | |
| Spell check | `spellbook` | Hunspell dictionaries. |
| Mail rules on the server | `sieve-rs` (compile) + ManageSieve | |
| OpenPGP (later) | `sequoia-openpgp` or `pgp` (rPGP) | |
| Server | `axum`, PostgreSQL (`sqlx`) | |

## 5. Local data

### 5.1 Locations (XDG)

| Path | Content |
|---|---|
| `$XDG_CONFIG_HOME/katna/` | Settings (TOML). |
| `$XDG_DATA_HOME/katna/mail.db` | Mail database. |
| `$XDG_DATA_HOME/katna/pim.db` | Shared: accounts, contacts, organizations. Used by Mail and Calendar. |
| `$XDG_DATA_HOME/katna/calendar.db` | Calendar database. |
| `$XDG_DATA_HOME/katna/blobs/` | Raw messages and attachments. |
| `$XDG_DATA_HOME/katna/index/` | tantivy index (rebuildable, but expensive, so not in cache). |
| Secret Service (`oo7`) | Passwords and OAuth tokens. Never in files. |

Mail and Calendar are separate processes. They share `pim.db` (SQLite WAL
allows concurrent readers and one writer). They tell each other about
changes with a D-Bus signal (`org.katna.Pim.Changed`).

### 5.2 Blob store

- Raw RFC 822 messages stored content-addressed:
  `blobs/<first 2 hex>/<blake3>.eml.zst` (zstd-compressed).
- The same message in several folders or labels is stored once.
- Large attachments can be stored as separate blobs and fetched on demand.
- Maildir **export** is offered for interoperability with notmuch/mu/mutt.

**Decision needed:** own blob store (smaller, deduplicated) vs. plain
Maildir (interoperable). The proposal is the blob store plus Maildir export.

### 5.3 Mail schema (sketch)

```sql
account          (id, kind, display_name, settings_json)
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
```

`participant` is the key table for organizations (§8) and address search.
`body_state` drives sync, search and UI (§6, §7).

### 5.4 Shared PIM schema (sketch)

```sql
organization     (id, name, kind, color, notes)           -- kind: customer|vendor|partner|other
org_alias        (org_id, alias)
org_rule         (org_id, rule_kind, value)               -- domain | subdomain | address
contact          (id, display_name, vcard_uid, notes)
contact_address  (contact_id, email_norm)
org_member       (org_id, contact_id)
suggestion       (id, kind, payload_json, state)          -- pending | accepted | dismissed
meta             (object_kind, object_id, plugin, value_json, version,
                  expires_at NULL, dirty BOOL)            -- see §9
```

## 6. Sync engine (`katna-sync`)

### 6.1 Account workers

One worker per account, running on a background executor:

- **Foreground loop:** push (IMAP IDLE / JMAP push) on the inbox, quick
  refresh of visible folders.
- **Background loop:** backfill, text indexing downloads, other folders.
- **Op queue:** replays local changes (flags, moves, deletes) to the server
  with retries and conflict handling.
- Incremental sync uses **CONDSTORE / QRESYNC** when available.
  Gmail uses `X-GM-EXT-1` (labels, thread IDs, `X-GM-RAW` search).

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

- Fetch on click, save the blob, set `body_state = 2`.
- Pre-fetch neighbors while the user scrolls the list.
- Offline: show headers and indexed text with a "not available offline" note.
- Large attachments (> 5 MB default) are always fetched on click.

### 6.4 Protocol notes

- **IMAP / SMTP / JMAP:** Pimalaya `io-*` crates behind our `MailBackend` trait.
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

One tantivy index for all accounts. Documents store only IDs and fields
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

## 8. Organizations (`katna-org`)

### 8.1 Model

An organization has a name, aliases, a kind, a color, and **rules**:
domain (`@acme.com`), subdomains (`*.acme.com`) and exact addresses
(`john.k@gmail.com`). People (contacts) can have many addresses and belong
to many organizations.

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
`MEMBER`) and can sync via CardDAV. Katna Calendar uses the same matching on
event attendees ("all meetings with Acme").

## 9. Metadata with expiration (`katna-meta`)

Inspired by Mailspring's plugin metadata. Any object (message, thread,
draft, event) can have a JSON value per feature, with an optional
`expires_at`.

- Stored in the `meta` table; versioned; `dirty` rows are uploaded to
  Katna Server when the user has an account there.
- A **scheduler** sleeps until the next `expires_at` and emits an
  `Expired(object, plugin)` event. The handler must clear or move the expiry
  so it does not fire again.
- Values the server does not need are **end-to-end encrypted** before upload;
  the server only sees object IDs, `expires_at` and what a server-side
  action requires.

Features built on it:

| Feature | Metadata | On expiry |
|---|---|---|
| Undo send | `{send_at: now + N s, undo: true}` on the draft (N = 5/10/20/30 s) | Send the draft. Undo = delete the metadata. The draft stays saved, so a crash does not lose it. |
| Send later | `{send_at}` | Send the draft (app, or Katna Server if enabled). |
| Snooze | `{until}`, thread moved to a "Snoozed" folder | Move back to inbox, mark unread. |
| Reminder | `{remind_at, if_no_reply: true}` | Notify if nobody replied. |
| Tracking | `{tracking_id, links[], events[]}` | — (events arrive from the server) |

## 10. Sending (outbox)

- Every send goes through the persistent `outbox` table.
- **Undo send** is a delay in the outbox, not a recall. True "unsend" after
  delivery is impossible over SMTP. Katna Confidential (§13) is the
  alternative.
- **Per-recipient sending** (needed for tracking who opened): one tracked
  copy per recipient, each sent in its own SMTP transaction to that
  recipient only; To/Cc headers unchanged. Cap on recipient count.
  After sending, remove tracked copies the server placed in Sent (Gmail:
  also from All Mail) and append the clean copy. Report partial failures
  with the list of who received it.
- Sent-folder cleanup must be robust: find copies by `Message-ID` with
  retries, and never leave a tracked copy where the user will open it.

## 11. Message rendering (`katna-render`)

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

## 12. UI

### 12.1 Window decorations (`katna-chrome`)

| Desktop | Mode | Details |
|---|---|---|
| KDE (KWin) | **Server-side** (xdg-decoration) | KWin draws the real Breeze (or user-chosen) title bar, buttons and shadow. Toolbar sits below the title bar, like other KDE apps. |
| GNOME (Mutter) | **Client-side** (must be requested explicitly, or the window has no title bar) | Adwaita-style header bar; button layout from `org.gnome.desktop.wm.preferences button-layout` (default: close only); own shadow in a transparent margin with correct window geometry; larger shadow when focused; rounded corners only when floating (none when maximized/tiled); resize edges in the shadow area; double-click maximize, right-click window menu. |
| Others (Sway, Hyprland, …) | SSD if offered, else minimal CSD; none when tiled | |
| X11 | SSD by default | CSD shadows need a compositor. |

Detection via `XDG_CURRENT_DESKTOP`, with a user override.

### 12.2 Look and feel

- One Katna design language on a **token layer** (radius, spacing, button
  style, colors, fonts) with two presets: **Breeze-like** and **Adwaita-like**.
- System integration: color scheme and accent color via the portal
  Settings interface (`ashpd`, live updates); full KDE palette from
  `kdeglobals`; system UI font; freedesktop icon theme; portal file
  chooser; notifications; tray (`ksni`).
- Keyboard-first: Gmail-compatible shortcuts, command palette.
- Accessibility via AccessKit (AT-SPI) in current GPUI.

### 12.3 Main mail layout

Sidebar (accounts, unified inbox, folders, **organizations**, smart views) →
thread list (organization color badges, availability icon) → reading pane
(conversation view). Organization page: people, timeline, attachments,
awaiting-reply threads, engagement stats.

## 13. Katna Server (`server/katna-server`)

Optional. Self-hostable (container image) and offered as a hosted Pro service.

| Function | Needs user mail credentials? |
|---|---|
| Open/link tracking + event stream | No |
| Metadata sync between devices (E2E-encrypted values) | No |
| Snooze / reminders while the app is closed (push notification or IMAP move) | IMAP move: yes (optional) |
| Send later while the app is closed | Yes: SMTP credentials or a send-only OAuth scope (`gmail.send`); opt-in per account |
| Katna Confidential (revocable / expiring mail via link) | No |
| Large-attachment links | No |

### 13.1 Tracking design

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

### 13.2 Stack

`axum` + PostgreSQL; WebSocket/SSE delta stream to clients; a scheduler
for server-side actions; shared crates with the apps where useful.

## 14. Katna Calendar

- CalDAV sync via `io-webdav` (discovery, sync-token, multiget); local
  `calendar.db`; also local-only calendars.
- `calcard` for iCalendar/vCard; `rrule` for recurrence expansion;
  `jiff` for time zones, including embedded `VTIMEZONE` definitions.
- Correct handling of recurrence exceptions (`RECURRENCE-ID`, `EXDATE`).
- Invitations (iTIP/iMIP) shared with Katna Mail: accept/decline from mail.
- Views: day, week, month, agenda; organization filter.
- Server quirks: test against Google, Nextcloud, Radicale, Fastmail, Stalwart.

## 15. Security and privacy

- TLS only via `rustls`; no plain-text auth without an explicit warning.
- Secrets only in the Secret Service (via portal inside Flatpak).
- Remote content blocked by default; HTML always sanitized.
- Incoming tracker removal.
- Metadata uploaded to Katna Server is E2E-encrypted where possible.
- OpenPGP (later): Sequoia or rPGP. S/MIME: later, Rust support is weaker.
- The search index contains message text: it is covered by the same disk
  protection as the mail itself. Decrypted PGP mail is not indexed by default.

## 16. Packaging

| Format | Tooling | Priority |
|---|---|---|
| Flatpak (Flathub) | `flatpak-cargo-generator` for offline builds | 1 |
| .deb | `cargo-deb` | 1 |
| .rpm | `cargo-generate-rpm` | 1 |
| AUR | PKGBUILD | 1 |
| AppImage, Nix flake | standard tooling | 2 |
| Gentoo, Alpine, Void | `cargo-ebuild`, APKBUILD, templates | 3 |

Every package ships: `.desktop` files, AppStream metainfo, icons, and
MIME handlers for `x-scheme-handler/mailto` and `text/calendar`.

Official Debian/Fedora repositories require every crate to be packaged
separately and do not accept git dependencies. Third-party repositories
(Flathub, AUR, OBS, Copr, PPA) are the realistic path.

## 17. Dependency policy

- **GPUI:** pin exact `gpui-pre` and GPUI Kit versions; GPUI types only in
  `katna-ui`, `katna-chrome` and the apps.
- **Pimalaya: light forks.** Fork only crates we change. Fork `master`
  mirrors upstream; our changes live on a `katna` branch. Use
  `[patch.crates-io]` in the workspace; drop the patch when upstream merges
  our fix. Keep `PATCHES.md` in each fork. Upstream fixes early.
- **Weekly CI job** builds against the latest upstream `gpui-pre` and
  Pimalaya to detect breaking changes early.
- Before a release: no git dependencies (upstream merged, or forks
  published as `katna-*` crates).
- Pimalaya code is AI-assisted by its own disclosure: we test it against
  real servers (Dovecot, Gmail, Fastmail, Stalwart, Nextcloud).

## 18. Licensing — Decision needed

Proposal: **GPL-3.0-or-later** for the apps and shared crates.

- Compatible with our MIT/Apache dependencies (Pimalaya, tantivy, GPUI).
- Allows reusing code from GPLv3 projects such as Mailspring and
  Mailspring-Sync. With a permissive license, only their ideas could be used.
- Katna Server: GPL-3.0 or AGPL-3.0 (AGPL keeps hosted forks open).

## 19. Roadmap

| Phase | Deliverable | Done when |
|---|---|---|
| **0. Foundations** | Workspace, CI, `katna-core`, `katna-store` schema, `katna-search-cli` on the Enron corpus | Queries like `from:alice has:attachment invoice` return in < 50 ms on Enron |
| **1. Sync core** | IMAP metadata + text + offline window, POP3, SMTP, op queue, threading | Two real accounts sync incrementally and survive restarts offline |
| **2. Organizations** | Model, rules, `org:` search, suggestions (CLI first) | Searching a company name finds mail from personal addresses |
| **3. Mail UI** | GPUI app, window chrome on KDE and GNOME, 3-pane layout, text/HTML-subset rendering, Markdown composer, undo send | Daily-drivable for one account on KDE and GNOME |
| **4. Gmail-class features** | Labels, snooze, send later, reminders, rules (+ Sieve), unsubscribe, templates, tabs/categories | Feature checklist complete |
| **5. Katna Server** | Tracking, activity dashboard, metadata sync, server-side scheduled actions | Tracking with bot/scanner labeling in production |
| **6. Katna Calendar** | CalDAV, recurrence, invitations, organization filter | Syncs with Google, Nextcloud and Fastmail |
| **7. Polish** | Full HTML rendering, WYSIWYG composer, OpenPGP, semantic search (local embeddings) | — |

Packaging (Flatpak, deb, rpm, AUR) starts from Phase 3.

## 20. Risks

| Risk | Impact | Mitigation |
|---|---|---|
| GPUI has no official release since 0.2.2; `gpui-pre` has one maintainer | Breakage, stuck on old version | Pin versions, isolate UI crates, weekly upstream CI, be ready to maintain a fork |
| Pimalaya libraries are young and changing | API churn, bugs | Light forks, own traits, real-server tests, upstream contributions |
| HTML email rendering | Poor display of real-world mail | Phased approach; decide Blitz vs WebKitGTK early with a prototype |
| Rich-text composer in GPUI | Large effort | Markdown composer first |
| Google/Microsoft OAuth verification | Blocks Gmail/Outlook for public users | Launch with other providers; plan verification budget |
| Tracking law and spam filters | Legal risk, deliverability | Opt-in, privacy-first server, legal review, dedicated domains |
| Calendar edge cases (recurrence, time zones) | Wrong event times | Test corpus from real servers, fuzzing |
| Scope | Burnout, never shipping | Strict phases with "done when" criteria |

## 21. Open decisions

1. License (§18).
2. Blob store vs. Maildir (§5.2).
3. HTML renderer for phase 2 (§11).
4. Repository name/structure: one `katna` monorepo (proposed) vs. per-app repos.
5. Katna Server hosting and pricing model.
