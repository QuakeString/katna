-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v4: tasks Katna keeps on this computer, listed and ticked
-- off in the desktop clock (docs/ARCHITECTURE.md §15.4). Tasks synced
-- with a mail service's own list come later and add their columns then.

CREATE TABLE task (
    id         INTEGER PRIMARY KEY,
    title      TEXT    NOT NULL,
    notes      TEXT    NOT NULL DEFAULT '',
    due        TEXT,                        -- YYYY-MM-DD, or NULL
    done_at    INTEGER,                     -- Unix seconds; NULL while open
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
CREATE INDEX task_open ON task (done_at, due);
