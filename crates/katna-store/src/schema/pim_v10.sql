-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v10: Google's "Other contacts", the people a Gmail
-- account mailed but never saved (docs/ARCHITECTURE.md §8.6). Read-only:
-- saving one copies it into the account's contacts.

CREATE TABLE other_contact (
    id         INTEGER PRIMARY KEY,
    account_id INTEGER NOT NULL REFERENCES account (id) ON DELETE CASCADE,
    remote_id  TEXT    NOT NULL,              -- otherContacts/…
    name       TEXT    NOT NULL DEFAULT '',
    sort_key   TEXT    NOT NULL DEFAULT '',
    emails     TEXT    NOT NULL DEFAULT '[]', -- JSON list, lower case
    phone      TEXT    NOT NULL DEFAULT '',
    UNIQUE (account_id, remote_id)
);

-- Where each account's last read left off, and how it went.
CREATE TABLE other_contact_sync (
    account_id INTEGER PRIMARY KEY REFERENCES account (id) ON DELETE CASCADE,
    sync_token TEXT,
    state      TEXT    NOT NULL DEFAULT 'ok'
               CHECK (state IN ('ok', 'needs-permission', 'failed'))
);
