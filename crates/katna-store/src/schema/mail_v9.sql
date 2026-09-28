-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v9: delivery receipts (docs/ARCHITECTURE.md §16.1).

-- Ask the mail server for a delivery status notification per recipient
-- (SMTP DSN, RFC 3461) where it offers them.
ALTER TABLE outbox ADD COLUMN delivery_receipt INTEGER NOT NULL DEFAULT 0
    CHECK (delivery_receipt IN (0, 1));
