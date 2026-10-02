-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v13: mail rules (docs/ARCHITECTURE.md §9.4). The daemon
-- runs them on new incoming mail in `position` order. Conditions, actions
-- and accounts are JSON (`katna_store::rules`); `runs_on` is `katna`
-- today, `gmail` (filters) and `sieve` later. A rule whose action failed
-- is switched off with the reason in `last_error`.

CREATE TABLE mail_rule (
    id              INTEGER PRIMARY KEY,
    name            TEXT    NOT NULL,
    enabled         INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
    position        INTEGER NOT NULL DEFAULT 0,
    match_mode      TEXT    NOT NULL DEFAULT 'all' CHECK (match_mode IN ('all', 'any')),
    conditions_json TEXT    NOT NULL DEFAULT '[]',
    actions_json    TEXT    NOT NULL DEFAULT '[]',
    stop            INTEGER NOT NULL DEFAULT 0 CHECK (stop IN (0, 1)),
    accounts_json   TEXT    NOT NULL DEFAULT '[]',
    runs_on         TEXT    NOT NULL DEFAULT 'katna'
                    CHECK (runs_on IN ('katna', 'gmail', 'sieve')),
    last_error      TEXT,
    created_at      INTEGER NOT NULL,
    updated_at      INTEGER NOT NULL
);
CREATE INDEX mail_rule_by_position ON mail_rule (position, id);
