-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v5: task lists, and tasks synced with each account's own
-- task service (Google Tasks, Microsoft To Do) besides the list kept on
-- this computer (docs/ARCHITECTURE.md §18.1).

-- A task list: an account's (Google Tasks, To Do) or, with no account,
-- one kept on this computer.
CREATE TABLE task_list (
    id         INTEGER PRIMARY KEY,
    account_id INTEGER REFERENCES account (id) ON DELETE CASCADE,
    remote_id  TEXT,                         -- the service's ID; NULL until created there
    title      TEXT    NOT NULL,
    is_default INTEGER NOT NULL DEFAULT 0,   -- the account's own default list
    sync_state TEXT,                         -- where the last pull ended (a time or a delta link)
    dirty      INTEGER NOT NULL DEFAULT 0,   -- renamed here, not yet on the service
    deleted    INTEGER NOT NULL DEFAULT 0,   -- deleted here, not yet on the service
    -- Its tasks move to the first synced default list once one exists
    -- (tasks added in the desktop clock before any list synced).
    move_out   INTEGER NOT NULL DEFAULT 0,
    UNIQUE (account_id, remote_id)
);

-- The list the clock's tasks were kept in until now.
INSERT INTO task_list (id, title, is_default, move_out) VALUES (1, 'My Tasks', 1, 1);

-- NULL never stays: SQLite adds a REFERENCES column only with no default.
ALTER TABLE task ADD COLUMN list_id   INTEGER REFERENCES task_list (id) ON DELETE CASCADE;
ALTER TABLE task ADD COLUMN parent_id INTEGER REFERENCES task (id) ON DELETE CASCADE;
ALTER TABLE task ADD COLUMN remote_id TEXT;
ALTER TABLE task ADD COLUMN etag      TEXT;
-- The service's order (Google's position string); '' sorts newest first.
ALTER TABLE task ADD COLUMN position  TEXT    NOT NULL DEFAULT '';
-- What Google Tasks can't keep stays here only: a time on the due day
-- (minutes after local midnight), a reminder (Unix seconds), a repeat
-- rule (RFC 5545 RRULE value) and the star.
ALTER TABLE task ADD COLUMN due_time  INTEGER;
ALTER TABLE task ADD COLUMN remind_at INTEGER;
ALTER TABLE task ADD COLUMN repeat    TEXT    NOT NULL DEFAULT '';
ALTER TABLE task ADD COLUMN starred   INTEGER NOT NULL DEFAULT 0;
-- The mail it was made from: its Message-ID, without angle brackets.
ALTER TABLE task ADD COLUMN mail      TEXT    NOT NULL DEFAULT '';
ALTER TABLE task ADD COLUMN dirty     INTEGER NOT NULL DEFAULT 0;
ALTER TABLE task ADD COLUMN deleted   INTEGER NOT NULL DEFAULT 0;

UPDATE task SET list_id = 1;

CREATE INDEX task_by_list ON task (list_id, parent_id);
CREATE UNIQUE INDEX task_by_remote ON task (list_id, remote_id) WHERE remote_id IS NOT NULL;
