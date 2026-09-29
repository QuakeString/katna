-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v5: notes (docs/ARCHITECTURE.md §13.11). A note of a mail
-- account is kept in that account's Notes folder in Apple's format; one
-- with no account stays on this computer.

CREATE TABLE note (
    id           INTEGER PRIMARY KEY,
    account_id   INTEGER,                    -- NULL: on this computer only
    uuid         TEXT    NOT NULL UNIQUE,    -- X-Universally-Unique-Identifier
    title        TEXT    NOT NULL DEFAULT '',
    body         TEXT    NOT NULL DEFAULT '', -- plain text; "☐ " / "☑ " lines are a checklist
    color        INTEGER NOT NULL DEFAULT 0, -- 0 none, else a palette number
    pinned       INTEGER NOT NULL DEFAULT 0,
    archived     INTEGER NOT NULL DEFAULT 0,
    labels       TEXT    NOT NULL DEFAULT '[]', -- JSON array of label names
    link         TEXT,                       -- Message-ID of the mail the note is about
    position     INTEGER NOT NULL DEFAULT 0, -- larger shows first
    created_at   INTEGER NOT NULL,
    updated_at   INTEGER NOT NULL,
    trashed_at   INTEGER,                    -- in Trash since; gone for good 7 days later
    server_uid   INTEGER,                    -- UID in the account's Notes folder
    dirty        INTEGER NOT NULL DEFAULT 0  -- changed here, the server copy not yet
);
CREATE INDEX note_by_account ON note (account_id, server_uid);
CREATE INDEX note_by_order ON note (trashed_at, archived, pinned DESC, position DESC);

-- Notes deleted here whose server copy is still to be deleted.
CREATE TABLE note_gone (
    account_id INTEGER NOT NULL,
    server_uid INTEGER NOT NULL,
    PRIMARY KEY (account_id, server_uid)
);
