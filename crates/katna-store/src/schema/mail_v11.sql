-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v11: which folders ring and count, and what is muted
-- (docs/ARCHITECTURE.md §15.1.1).

-- A folder's bell where it differs from the default (an inbox's Primary
-- tab notifies and counts; nothing else does). `category` is the inbox
-- tab (`message.category`, Primary for unclassified mail) for an inbox
-- and 0 for any other folder.
CREATE TABLE folder_alert (
    folder_id INTEGER NOT NULL REFERENCES folder (id) ON DELETE CASCADE,
    category  INTEGER NOT NULL,
    notify    INTEGER NOT NULL CHECK (notify IN (0, 1)),
    count     INTEGER NOT NULL CHECK (count IN (0, 1)),
    PRIMARY KEY (folder_id, category)
) WITHOUT ROWID;

-- Muted things: their new mail never notifies and is left out of the
-- taskbar and tray count. One of `account_id` (kind 'account'),
-- `folder_id` ('folder'), `thread_id` ('thread') or `address` ('sender',
-- lower case, every account) names what. `until` ends a timed mute;
-- NULL lasts until unmuted. `server` is 1 when the mail service keeps
-- the mute too (Gmail's mute, the `$muted` keyword), so unmuting there
-- unmutes here. `label` is what Settings shows: a subject or a name.
CREATE TABLE mute (
    id         INTEGER PRIMARY KEY,
    kind       TEXT    NOT NULL CHECK (kind IN ('account', 'folder', 'thread', 'sender')),
    account_id INTEGER,
    folder_id  INTEGER REFERENCES folder (id) ON DELETE CASCADE,
    thread_id  INTEGER REFERENCES thread (id) ON DELETE CASCADE,
    address    TEXT,
    label      TEXT    NOT NULL DEFAULT '',
    until      INTEGER,
    server     INTEGER NOT NULL DEFAULT 0 CHECK (server IN (0, 1)),
    created_at INTEGER NOT NULL
);
CREATE UNIQUE INDEX mute_account ON mute (account_id) WHERE kind = 'account';
CREATE UNIQUE INDEX mute_folder ON mute (folder_id) WHERE kind = 'folder';
CREATE UNIQUE INDEX mute_thread ON mute (thread_id) WHERE kind = 'thread';
CREATE UNIQUE INDEX mute_sender ON mute (address) WHERE kind = 'sender';
