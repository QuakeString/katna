-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v1 (docs/ARCHITECTURE.md §5.3).
--
-- Conventions: times are Unix seconds (UTC); hashes are 32-byte blake3
-- digests; JSON is stored as TEXT. `account_id` refers to `account` in
-- pim.db, so it has no foreign key.

CREATE TABLE folder (
    id            INTEGER PRIMARY KEY,
    account_id    INTEGER NOT NULL,
    path          TEXT    NOT NULL,
    role          TEXT,       -- inbox | sent | drafts | trash | junk | archive | all | NULL
    uidvalidity   INTEGER,
    highestmodseq INTEGER,
    sync_state    TEXT,       -- JSON, owned by katna-sync
    UNIQUE (account_id, path)
);

CREATE TABLE thread (
    id            INTEGER PRIMARY KEY,
    account_id    INTEGER NOT NULL,
    subject_norm  TEXT    NOT NULL DEFAULT '',
    last_date     INTEGER,
    message_count INTEGER NOT NULL DEFAULT 0,
    flags_summary INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX thread_by_account_date ON thread (account_id, last_date DESC);

CREATE TABLE message (
    id                INTEGER PRIMARY KEY,
    account_id        INTEGER NOT NULL,
    message_id_hdr    TEXT,
    thread_id         INTEGER REFERENCES thread (id) ON DELETE SET NULL,
    subject           TEXT    NOT NULL DEFAULT '',
    date              INTEGER,
    size              INTEGER NOT NULL DEFAULT 0,
    flags             INTEGER NOT NULL DEFAULT 0,  -- bit set: seen, answered, flagged, ...
    keywords          TEXT,                        -- JSON array of IMAP keywords / labels
    has_attachments   INTEGER NOT NULL DEFAULT 0 CHECK (has_attachments IN (0, 1)),
    list_id           TEXT,
    body_state        INTEGER NOT NULL DEFAULT 0 CHECK (body_state IN (0, 1, 2)),
                                                   -- 0 headers | 1 text_indexed | 2 full
    blob_hash         BLOB CHECK (blob_hash IS NULL OR length(blob_hash) = 32),
    snippet           TEXT,
    auth_results_json TEXT
);
CREATE INDEX message_by_account_date ON message (account_id, date DESC);
CREATE INDEX message_by_thread ON message (thread_id);
CREATE INDEX message_by_message_id ON message (message_id_hdr);
CREATE INDEX message_by_blob ON message (blob_hash);

-- One message, many folders or labels.
CREATE TABLE message_location (
    message_id INTEGER NOT NULL REFERENCES message (id) ON DELETE CASCADE,
    folder_id  INTEGER NOT NULL REFERENCES folder (id) ON DELETE CASCADE,
    uid        INTEGER,
    PRIMARY KEY (message_id, folder_id)
);
CREATE INDEX message_location_by_folder_uid ON message_location (folder_id, uid);

-- Key table for organizations (§8) and address search.
CREATE TABLE participant (
    message_id   INTEGER NOT NULL REFERENCES message (id) ON DELETE CASCADE,
    role         TEXT    NOT NULL
                 CHECK (role IN ('from', 'to', 'cc', 'bcc', 'reply_to', 'sender')),
    email_norm   TEXT    NOT NULL,
    domain       TEXT    NOT NULL,
    display_name TEXT
);
CREATE INDEX participant_by_message ON participant (message_id);
CREATE INDEX participant_by_email ON participant (email_norm);
CREATE INDEX participant_by_domain ON participant (domain);

CREATE TABLE attachment (
    id         INTEGER PRIMARY KEY,
    message_id INTEGER NOT NULL REFERENCES message (id) ON DELETE CASCADE,
    part_id    TEXT    NOT NULL,
    filename   TEXT,
    mime       TEXT    NOT NULL,
    size       INTEGER NOT NULL DEFAULT 0,
    blob_hash  BLOB CHECK (blob_hash IS NULL OR length(blob_hash) = 32),
    UNIQUE (message_id, part_id)
);

CREATE TABLE op_queue (
    id          INTEGER PRIMARY KEY,
    account_id  INTEGER NOT NULL,
    op_json     TEXT    NOT NULL,
    state       TEXT    NOT NULL DEFAULT 'pending'
                CHECK (state IN ('pending', 'running', 'failed', 'done')),
    attempts    INTEGER NOT NULL DEFAULT 0,
    next_try_at INTEGER
);
CREATE INDEX op_queue_by_state ON op_queue (state, next_try_at);

CREATE TABLE outbox (
    id               INTEGER PRIMARY KEY,
    draft_message_id INTEGER NOT NULL REFERENCES message (id),
    send_at          INTEGER NOT NULL,
    state            TEXT    NOT NULL DEFAULT 'queued'
                     CHECK (state IN ('queued', 'sending', 'sent', 'failed', 'cancelled')),
    per_recipient    INTEGER NOT NULL DEFAULT 0 CHECK (per_recipient IN (0, 1)),
    attempts         INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX outbox_by_state ON outbox (state, send_at);

-- Desktop notifications we showed, so they can be closed or updated later.
CREATE TABLE notification (
    notif_id    INTEGER PRIMARY KEY,
    message_ids TEXT    NOT NULL,   -- JSON array
    account_id  INTEGER NOT NULL,
    created_at  INTEGER NOT NULL
);

-- Change journal: apps read it after a D-Bus change signal (§14.2).
CREATE TABLE change_log (
    seq         INTEGER PRIMARY KEY,
    object_kind TEXT    NOT NULL,
    object_id   INTEGER NOT NULL,
    op          TEXT    NOT NULL CHECK (op IN ('insert', 'update', 'delete')),
    changed_at  INTEGER NOT NULL
);
