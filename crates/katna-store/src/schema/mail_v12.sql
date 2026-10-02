-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v12: pins in a chat (docs/ARCHITECTURE.md, the chat view).

-- Something pinned to the top of a conversation shown as a chat: a whole
-- mail ('mail'), one of its files ('file': `file_order`, the file's place
-- among the mail's attachments) or text picked from it ('text': `text`).
-- Kept on this computer only; no mail service has pins inside a
-- conversation. A conversation's pins are found through its messages,
-- so they outlive its thread being merged or rebuilt; it holds up to
-- five. `position` orders them, smallest first. `label` is what the pin
-- bar shows: a subject, a file name or the text.
CREATE TABLE chat_pin (
    id         INTEGER PRIMARY KEY,
    message_id INTEGER NOT NULL REFERENCES message (id) ON DELETE CASCADE,
    kind       TEXT    NOT NULL CHECK (kind IN ('mail', 'file', 'text')),
    file_order INTEGER,
    text       TEXT,
    label      TEXT    NOT NULL DEFAULT '',
    position   INTEGER NOT NULL,
    created_at INTEGER NOT NULL
);
CREATE INDEX chat_pin_message ON chat_pin (message_id);
