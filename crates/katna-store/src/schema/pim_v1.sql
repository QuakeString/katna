-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v1 (docs/ARCHITECTURE.md §5.4): data shared by Katna Mail
-- and Katna Calendar. Same conventions as mail_v1.sql.

CREATE TABLE account (
    id            INTEGER PRIMARY KEY,
    kind          TEXT    NOT NULL
                  CHECK (kind IN ('imap', 'jmap', 'pop3', 'caldav', 'carddav', 'local')),
    display_name  TEXT    NOT NULL,
    address       TEXT    NOT NULL,
    settings_json TEXT    NOT NULL DEFAULT '{}'
);

CREATE TABLE organization (
    id            INTEGER PRIMARY KEY,
    name          TEXT    NOT NULL,
    kind          TEXT    NOT NULL DEFAULT 'other'
                  CHECK (kind IN ('customer', 'vendor', 'partner', 'other')),
    color         TEXT,
    notes         TEXT,
    notify_policy TEXT
);

CREATE TABLE org_alias (
    org_id INTEGER NOT NULL REFERENCES organization (id) ON DELETE CASCADE,
    alias  TEXT    NOT NULL,
    PRIMARY KEY (org_id, alias)
);

CREATE TABLE org_rule (
    org_id    INTEGER NOT NULL REFERENCES organization (id) ON DELETE CASCADE,
    rule_kind TEXT    NOT NULL CHECK (rule_kind IN ('domain', 'subdomain', 'address')),
    value     TEXT    NOT NULL,
    PRIMARY KEY (org_id, rule_kind, value)
);
CREATE INDEX org_rule_by_value ON org_rule (rule_kind, value);

CREATE TABLE contact (
    id           INTEGER PRIMARY KEY,
    display_name TEXT NOT NULL DEFAULT '',
    vcard_uid    TEXT UNIQUE,
    notes        TEXT
);

CREATE TABLE contact_address (
    contact_id INTEGER NOT NULL REFERENCES contact (id) ON DELETE CASCADE,
    email_norm TEXT    NOT NULL,
    PRIMARY KEY (contact_id, email_norm)
);
CREATE INDEX contact_address_by_email ON contact_address (email_norm);

CREATE TABLE org_member (
    org_id     INTEGER NOT NULL REFERENCES organization (id) ON DELETE CASCADE,
    contact_id INTEGER NOT NULL REFERENCES contact (id) ON DELETE CASCADE,
    PRIMARY KEY (org_id, contact_id)
);

CREATE TABLE suggestion (
    id           INTEGER PRIMARY KEY,
    kind         TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    state        TEXT NOT NULL DEFAULT 'pending'
                 CHECK (state IN ('pending', 'accepted', 'dismissed'))
);

-- Metadata with expiration (§10).
CREATE TABLE meta (
    object_kind TEXT    NOT NULL,
    object_id   INTEGER NOT NULL,
    plugin      TEXT    NOT NULL,
    value_json  TEXT    NOT NULL,
    version     INTEGER NOT NULL DEFAULT 0,
    expires_at  INTEGER,
    dirty       INTEGER NOT NULL DEFAULT 0 CHECK (dirty IN (0, 1)),
    PRIMARY KEY (object_kind, object_id, plugin)
);
CREATE INDEX meta_by_expiry ON meta (expires_at);

CREATE TABLE change_log (
    seq         INTEGER PRIMARY KEY,
    object_kind TEXT    NOT NULL,
    object_id   INTEGER NOT NULL,
    op          TEXT    NOT NULL CHECK (op IN ('insert', 'update', 'delete')),
    changed_at  INTEGER NOT NULL
);
