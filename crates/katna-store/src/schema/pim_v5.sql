-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v5: contacts synced from each account's address books
-- (Google People API, Microsoft Graph, CardDAV) or kept on this computer
-- (docs/ARCHITECTURE.md §8.6). The v1 contact tables were never written,
-- so they are made again with the columns sync needs.

DROP TABLE org_member;
DROP TABLE contact_address;
DROP TABLE contact;

-- One address book: a Google account's contacts, a Microsoft contact
-- folder, a CardDAV collection, or the book on this computer (no account).
CREATE TABLE address_book (
    id         INTEGER PRIMARY KEY,
    account_id INTEGER REFERENCES account (id) ON DELETE CASCADE,
    source     TEXT    NOT NULL CHECK (source IN ('google', 'microsoft', 'carddav', 'local')),
    remote_id  TEXT    NOT NULL,                -- '' (Google), folder id, collection URL
    name       TEXT    NOT NULL DEFAULT '',
    sync_token TEXT,                            -- where the next sync picks up
    synced_at  INTEGER,                         -- Unix seconds
    state      TEXT    NOT NULL DEFAULT 'ok'
               CHECK (state IN ('ok', 'needs-permission', 'failed')),
    UNIQUE (account_id, remote_id)
);

CREATE TABLE contact (
    id           INTEGER PRIMARY KEY,
    book_id      INTEGER NOT NULL REFERENCES address_book (id) ON DELETE CASCADE,
    remote_id    TEXT    NOT NULL,              -- resourceName, Graph id, CardDAV href
    etag         TEXT,
    display_name TEXT    NOT NULL DEFAULT '',
    sort_key     TEXT    NOT NULL DEFAULT '',
    job          TEXT    NOT NULL DEFAULT '',   -- "Title, Company"
    phone        TEXT    NOT NULL DEFAULT '',   -- the first one, for the list
    starred      INTEGER NOT NULL DEFAULT 0 CHECK (starred IN (0, 1)),
    card_json    TEXT    NOT NULL,              -- katna_core::contact::Card
    raw          TEXT,                          -- the source's own form (vCard, JSON)
    updated_at   INTEGER NOT NULL,
    UNIQUE (book_id, remote_id)
);
CREATE INDEX contact_by_sort ON contact (sort_key);

CREATE TABLE contact_address (
    contact_id INTEGER NOT NULL REFERENCES contact (id) ON DELETE CASCADE,
    email_norm TEXT    NOT NULL,
    position   INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (contact_id, email_norm)
);
CREATE INDEX contact_address_by_email ON contact_address (email_norm);

-- A label (Google contact group, Microsoft category, CardDAV group card).
CREATE TABLE contact_group (
    id        INTEGER PRIMARY KEY,
    book_id   INTEGER NOT NULL REFERENCES address_book (id) ON DELETE CASCADE,
    remote_id TEXT    NOT NULL,
    name      TEXT    NOT NULL,
    UNIQUE (book_id, remote_id)
);

CREATE TABLE contact_group_member (
    group_id   INTEGER NOT NULL REFERENCES contact_group (id) ON DELETE CASCADE,
    contact_id INTEGER NOT NULL REFERENCES contact (id) ON DELETE CASCADE,
    PRIMARY KEY (group_id, contact_id)
);
CREATE INDEX contact_group_member_by_contact ON contact_group_member (contact_id);

-- The contact's picture, fetched once per source value.
CREATE TABLE contact_photo (
    contact_id INTEGER PRIMARY KEY REFERENCES contact (id) ON DELETE CASCADE,
    source     TEXT    NOT NULL,                -- URL or a hash of inline data
    data       BLOB    NOT NULL
);

CREATE TABLE org_member (
    org_id     INTEGER NOT NULL REFERENCES organization (id) ON DELETE CASCADE,
    contact_id INTEGER NOT NULL REFERENCES contact (id) ON DELETE CASCADE,
    PRIMARY KEY (org_id, contact_id)
);
