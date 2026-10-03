-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v16: pictures, reminders and history for notes
-- (docs/ARCHITECTURE.md §13.11).
--
-- A note's pictures sit in `note_picture`, named in its HTML as
-- `cid:<cid>`; they travel in its Notes-folder message as inline parts.
-- `note_reminder` says when a note reminds (UTC seconds). `note_version`
-- keeps earlier text of a note on this computer for 30 days; `source` is
-- `here`, or `sync` (written on another device) with the device's name
-- after a colon when its mail app says it.

CREATE TABLE IF NOT EXISTS note_reminder (
    note_id     INTEGER PRIMARY KEY REFERENCES note(id) ON DELETE CASCADE,
    at          INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS note_reminder_at ON note_reminder(at);

CREATE TABLE IF NOT EXISTS note_picture (
    note_id     INTEGER NOT NULL REFERENCES note(id) ON DELETE CASCADE,
    cid         TEXT    NOT NULL,
    ord         INTEGER NOT NULL,
    name        TEXT    NOT NULL,
    mime        TEXT    NOT NULL,
    width       INTEGER NOT NULL,
    height      INTEGER NOT NULL,
    data        BLOB    NOT NULL,
    PRIMARY KEY (note_id, cid)
);

CREATE TABLE IF NOT EXISTS note_version (
    id          INTEGER PRIMARY KEY,
    note_id     INTEGER NOT NULL REFERENCES note(id) ON DELETE CASCADE,
    at          INTEGER NOT NULL,
    title       TEXT    NOT NULL,
    body        TEXT    NOT NULL,
    html        TEXT    NOT NULL,
    source      TEXT    NOT NULL
);
CREATE INDEX IF NOT EXISTS note_version_note ON note_version(note_id, at);
