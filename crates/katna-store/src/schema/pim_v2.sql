-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v2: mail templates (docs/ARCHITECTURE.md, Compose), kept
-- on this computer. The body is HTML with pictures as data: URIs, plus its
-- plain text; attachments go with the template.

CREATE TABLE template (
    id         INTEGER PRIMARY KEY,
    name       TEXT    NOT NULL,
    subject    TEXT    NOT NULL DEFAULT '',
    html       TEXT    NOT NULL DEFAULT '',
    text       TEXT    NOT NULL DEFAULT '',
    -- Unix seconds.
    updated_at INTEGER NOT NULL
);

CREATE TABLE template_attachment (
    template_id INTEGER NOT NULL REFERENCES template (id) ON DELETE CASCADE,
    position    INTEGER NOT NULL,
    name        TEXT    NOT NULL,
    mime        TEXT    NOT NULL,
    data        BLOB    NOT NULL,
    PRIMARY KEY (template_id, position)
);
