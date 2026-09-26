-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v2: conversations and inbox categories
-- (docs/ARCHITECTURE.md §5.3 and §6.5).

-- Inbox tab (katna_core::MailCategory storage number); NULL = not
-- classified yet, shown as Primary.
ALTER TABLE message ADD COLUMN category INTEGER;

-- Gmail's X-GM-THRID: messages with the same one share a thread.
ALTER TABLE thread ADD COLUMN gm_thrid INTEGER;
CREATE UNIQUE INDEX thread_by_gm_thrid ON thread (account_id, gm_thrid)
    WHERE gm_thrid IS NOT NULL;
-- Subject fallback: the newest thread with the same normalized subject.
CREATE INDEX thread_by_subject ON thread (account_id, subject_norm, last_date);

-- Newest message of a thread in O(log n), for keeping last_date right.
DROP INDEX message_by_thread;
CREATE INDEX message_by_thread_date ON message (thread_id, date);

-- Message-IDs that stored messages refer to (In-Reply-To, References) but
-- that were not threaded when those messages arrived. When the referenced
-- message comes, it joins (or merges with) `thread_id`.
CREATE TABLE thread_ref (
    account_id     INTEGER NOT NULL,
    message_id_hdr TEXT    NOT NULL,
    thread_id      INTEGER NOT NULL REFERENCES thread (id) ON DELETE CASCADE,
    PRIMARY KEY (account_id, message_id_hdr, thread_id)
) WITHOUT ROWID;
CREATE INDEX thread_ref_by_thread ON thread_ref (thread_id);

-- thread.message_count and thread.last_date follow the message rows.
-- Empty threads are removed.
CREATE TRIGGER message_thread_insert AFTER INSERT ON message
WHEN new.thread_id IS NOT NULL
BEGIN
    UPDATE thread SET
        message_count = message_count + 1,
        last_date = max(coalesce(last_date, new.date), coalesce(new.date, last_date))
    WHERE id = new.thread_id;
END;

CREATE TRIGGER message_thread_delete AFTER DELETE ON message
WHEN old.thread_id IS NOT NULL
BEGIN
    UPDATE thread SET
        message_count = message_count - 1,
        last_date = CASE WHEN old.date IS NOT NULL AND old.date >= last_date
            THEN (SELECT max(date) FROM message WHERE thread_id = old.thread_id)
            ELSE last_date END
    WHERE id = old.thread_id;
    DELETE FROM thread WHERE id = old.thread_id AND message_count <= 0;
END;

CREATE TRIGGER message_thread_update AFTER UPDATE OF thread_id ON message
WHEN old.thread_id IS NOT new.thread_id
BEGIN
    UPDATE thread SET
        message_count = message_count - 1,
        last_date = CASE WHEN old.date IS NOT NULL AND old.date >= last_date
            THEN (SELECT max(date) FROM message WHERE thread_id = old.thread_id)
            ELSE last_date END
    WHERE old.thread_id IS NOT NULL AND id = old.thread_id;
    DELETE FROM thread
    WHERE old.thread_id IS NOT NULL AND id = old.thread_id AND message_count <= 0;
    UPDATE thread SET
        message_count = message_count + 1,
        last_date = max(coalesce(last_date, new.date), coalesce(new.date, last_date))
    WHERE new.thread_id IS NOT NULL AND id = new.thread_id;
END;

-- Existing messages have no thread yet; katna-daemon threads them in the
-- background (katna_import::backfill). This finds them quickly.
CREATE INDEX message_unthreaded ON message (id)
    WHERE thread_id IS NULL OR category IS NULL;
