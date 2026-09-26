-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v5: pinned mail (docs/ARCHITECTURE.md §13.5).

-- Messages the user pinned to the top of their folder's list. Katna's
-- own: IMAP has no pin, so pins stay on this computer. A pinned
-- conversation pins every message it had then; a line is pinned when any
-- of its messages is.
CREATE TABLE pin (
    message_id INTEGER PRIMARY KEY REFERENCES message (id) ON DELETE CASCADE,
    pinned_at  INTEGER NOT NULL
);
