-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v15: the oldest schema a reader needs (docs/ARCHITECTURE.md
-- §21.2, "Protecting local data").

-- `min_reader_version` is the oldest schema version whose Katna can still
-- open this database. A migration that only adds tables, columns or indexes
-- leaves it alone, so the previous release keeps working after a rollback;
-- one that removes or reshapes something raises it to its own version.
CREATE TABLE schema_meta (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
) WITHOUT ROWID;

-- Adding this table changes nothing an older reader uses.
INSERT INTO schema_meta (key, value) VALUES ('min_reader_version', '14');
