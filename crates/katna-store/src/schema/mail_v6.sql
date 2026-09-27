-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v6: how full each account's mail storage is
-- (docs/ARCHITECTURE.md §13.5).

-- The server's storage quota (IMAP QUOTA), read on each full sync.
-- Accounts whose server reports none have no row.
CREATE TABLE quota (
    account_id INTEGER PRIMARY KEY,
    used       INTEGER NOT NULL,   -- bytes
    quota_max  INTEGER NOT NULL,   -- bytes
    checked_at INTEGER NOT NULL
);
