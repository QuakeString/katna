-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v13: conversations summed up by AI (docs/ARCHITECTURE.md
-- §16.5).

-- A conversation summed up by the AI service the user chose, kept on this
-- computer so opening the conversation again shows it at once and asks
-- nothing. It is found through the newest mail it covers (`message_id`),
-- like a chat pin through its mail. `kind` is 'all' (the whole
-- conversation) or 'new' (only the mails unread when it was asked).
-- `mails` is how many mails it covers; `body` is the summary as JSON
-- (`katna_ai::summary::Summary`); `service` names who answered ('katna'
-- or 'own'). A conversation keeps one of each kind: a new one replaces
-- the old.
CREATE TABLE conversation_summary (
    id         INTEGER PRIMARY KEY,
    message_id INTEGER NOT NULL REFERENCES message (id) ON DELETE CASCADE,
    kind       TEXT    NOT NULL CHECK (kind IN ('all', 'new')),
    mails      INTEGER NOT NULL,
    body       TEXT    NOT NULL,
    service    TEXT    NOT NULL DEFAULT '',
    created_at INTEGER NOT NULL
);
CREATE INDEX conversation_summary_message ON conversation_summary (message_id);
