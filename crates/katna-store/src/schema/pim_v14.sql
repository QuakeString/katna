-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v14: mail rules on the mail service (docs/ARCHITECTURE.md
-- §9.4). A rule runs as a Gmail filter or in the account's Sieve script
-- when it can, else in Katna. `mail_rule_remote` holds what the daemon
-- put on each account's service, per rule: Gmail's filter IDs (no
-- foreign key: the filters of a deleted rule are deleted later), and
-- what was sent, to see when it changed. Rules with a row for an account
-- don't run in Katna on that account's mail. `mail_rule_note` says why a
-- rule stays in Katna (JSON, `katna_store::rules::RunsNote`), and
-- `mail_rule_server` whether an account's server takes Sieve scripts
-- (ManageSieve) and its Sieve extensions.

CREATE TABLE IF NOT EXISTS mail_rule_remote (
    account_id  INTEGER NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    rule_id     INTEGER NOT NULL,
    runs_on     TEXT    NOT NULL CHECK (runs_on IN ('gmail', 'sieve')),
    remote_ids  TEXT    NOT NULL DEFAULT '[]',
    spec        TEXT    NOT NULL DEFAULT '',
    PRIMARY KEY (account_id, rule_id)
);

CREATE TABLE IF NOT EXISTS mail_rule_note (
    rule_id     INTEGER PRIMARY KEY REFERENCES mail_rule(id) ON DELETE CASCADE,
    note        TEXT    NOT NULL
);

CREATE TABLE IF NOT EXISTS mail_rule_server (
    account_id  INTEGER PRIMARY KEY REFERENCES account(id) ON DELETE CASCADE,
    sieve       INTEGER NOT NULL CHECK (sieve IN (0, 1)),
    extensions  TEXT    NOT NULL DEFAULT '',
    checked_at  INTEGER NOT NULL
);
