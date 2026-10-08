-- SPDX-License-Identifier: GPL-3.0-or-later
-- pim.db schema v15: starter rules (docs/ARCHITECTURE.md §9.4). Katna Mail
-- offers a few rules switched off; one the user turns on or edits is
-- saved as an ordinary rule, with the starter's key here, so the list
-- stops offering it. Deleting the rule offers the starter again.

CREATE TABLE IF NOT EXISTS mail_rule_starter (
    rule_id     INTEGER PRIMARY KEY REFERENCES mail_rule(id) ON DELETE CASCADE,
    starter     TEXT    NOT NULL
);
