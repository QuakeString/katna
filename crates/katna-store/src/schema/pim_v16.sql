-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v16: labels and files on tasks (docs/ARCHITECTURE.md
-- §18.1). A task's labels are the same names as the notes' labels, a JSON
-- array like `note.labels`; To Do keeps them as categories, CalDAV as
-- CATEGORIES, other services here only. A file's bytes are a blob in
-- blobs.db (`hash`); `remote_id` is its ID on the task service once sent
-- (To Do's attachment ID, `inline:<hash>` for a CalDAV ATTACH), and
-- `local_only` marks one the service can't keep (Google, Zoho, too large,
-- or refused), which stays on this computer. A file removed here while on
-- the service stays as a tombstone (`deleted`) until the service removed
-- it too.

ALTER TABLE task ADD COLUMN labels TEXT NOT NULL DEFAULT '[]';

CREATE TABLE IF NOT EXISTS task_file (
    id          INTEGER PRIMARY KEY,
    task_id     INTEGER NOT NULL REFERENCES task (id) ON DELETE CASCADE,
    name        TEXT    NOT NULL,
    mime        TEXT    NOT NULL DEFAULT 'application/octet-stream',
    size        INTEGER NOT NULL,
    hash        BLOB    NOT NULL CHECK (length(hash) = 32),
    remote_id   TEXT,
    local_only  INTEGER NOT NULL DEFAULT 0 CHECK (local_only IN (0, 1)),
    deleted     INTEGER NOT NULL DEFAULT 0 CHECK (deleted IN (0, 1)),
    created_at  INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS task_file_by_task ON task_file (task_id);
