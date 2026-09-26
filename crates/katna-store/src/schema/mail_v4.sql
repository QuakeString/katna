-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v4: one Gmail message, many labels (docs/ARCHITECTURE.md §6.2).

-- Gmail's X-GM-MSGID. Gmail shows a message in every folder it has a label
-- for (INBOX, [Gmail]/All Mail, …); with the same X-GM-MSGID those copies
-- are one message row with one message_location per folder.
ALTER TABLE message ADD COLUMN gm_msgid INTEGER;
CREATE UNIQUE INDEX message_by_gm_msgid ON message (account_id, gm_msgid)
    WHERE gm_msgid IS NOT NULL;
