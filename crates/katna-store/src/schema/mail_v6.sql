-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v6: translations of messages (docs/ARCHITECTURE.md §16.3).

-- A message's text translated by Katna Server, kept so it is shown again
-- without asking the server. `source_hash` is the BLAKE3 hash of the text
-- that was translated: a different text (a message downloaded again in
-- full) is translated again.
CREATE TABLE translation (
    message_id  INTEGER NOT NULL REFERENCES message (id) ON DELETE CASCADE,
    target      TEXT    NOT NULL,
    source      TEXT    NOT NULL,
    source_hash BLOB    NOT NULL,
    text        TEXT    NOT NULL,
    created_at  INTEGER NOT NULL,
    PRIMARY KEY (message_id, target)
) WITHOUT ROWID;
