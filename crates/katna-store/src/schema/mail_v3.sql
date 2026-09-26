-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v3: POP3 accounts (docs/ARCHITECTURE.md §6.4).

-- Server messages a POP3 account has downloaded, by UIDL. The row outlives
-- the local message (message_id becomes NULL) so a message deleted in
-- Katna is not downloaded again and can be removed from the server.
CREATE TABLE pop3_uidl (
    account_id INTEGER NOT NULL,
    uidl       TEXT    NOT NULL,
    message_id INTEGER REFERENCES message (id) ON DELETE SET NULL,
    first_seen INTEGER NOT NULL,
    PRIMARY KEY (account_id, uidl)
);
CREATE INDEX pop3_uidl_by_message ON pop3_uidl (message_id);
