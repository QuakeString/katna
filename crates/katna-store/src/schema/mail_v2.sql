-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v2: threading (docs/ARCHITECTURE.md §6.5).

-- Message-IDs of the ancestors, oldest first, separated by spaces; the last
-- is the parent. NULL until read from the headers.
ALTER TABLE message ADD COLUMN refs TEXT;

-- Every Message-ID a thread's messages have or refer to, so a new message
-- finds its thread even when its parent was never downloaded.
CREATE TABLE thread_key (
    account_id INTEGER NOT NULL,
    key        TEXT    NOT NULL,
    thread_id  INTEGER NOT NULL REFERENCES thread (id) ON DELETE CASCADE,
    PRIMARY KEY (account_id, key)
);
CREATE INDEX thread_key_by_thread ON thread_key (thread_id);
