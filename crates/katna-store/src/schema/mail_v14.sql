-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v14: receipt mail shown as ticks (docs/ARCHITECTURE.md
-- §16.1).

-- Read receipts and delivery reports that answer mail in a folder here.
-- Katna Mail shows them as ticks on the mail they answer, so the lists
-- leave them out: they never become a conversation's newest message, its
-- subject or its preview. They stay in their folders and in search.
CREATE TABLE receipt_mail (
    message_id INTEGER PRIMARY KEY REFERENCES message (id) ON DELETE CASCADE
);
