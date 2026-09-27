-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v3: open and click tracking (docs/ARCHITECTURE.md §11,
-- §16.1). The server knows only random tracking IDs; which message and
-- recipient an ID stands for stays here.

-- A message sent with tracking: one row per outbox entry.
CREATE TABLE tracked_message (
    id             INTEGER PRIMARY KEY,
    outbox_id      INTEGER NOT NULL UNIQUE,   -- mail.db outbox.id
    account_id     INTEGER NOT NULL,
    message_id_hdr TEXT    NOT NULL,          -- without angle brackets
    subject        TEXT    NOT NULL DEFAULT '',
    links_json     TEXT    NOT NULL,          -- JSON array, numbered as sent
    created_at     INTEGER NOT NULL,
    sent_at        INTEGER                    -- every copy sent
);
CREATE INDEX tracked_message_by_msgid ON tracked_message (message_id_hdr);

-- One recipient's copy.
CREATE TABLE tracked_recipient (
    tracking_id TEXT    PRIMARY KEY,
    tracked_id  INTEGER NOT NULL REFERENCES tracked_message (id) ON DELETE CASCADE,
    email       TEXT    NOT NULL,
    name        TEXT,
    sent_at     INTEGER
);
CREATE INDEX tracked_recipient_by_message ON tracked_recipient (tracked_id);

-- Opens and clicks from the server's event stream.
CREATE TABLE tracking_event (
    seq         INTEGER PRIMARY KEY,          -- the server's event number
    tracking_id TEXT    NOT NULL,
    kind        TEXT    NOT NULL CHECK (kind IN ('open', 'click')),
    link        INTEGER,
    source      TEXT    NOT NULL CHECK (source IN ('person', 'apple_proxy', 'scanner')),
    at          INTEGER NOT NULL              -- Unix milliseconds
);
CREATE INDEX tracking_event_by_id ON tracking_event (tracking_id, at);
