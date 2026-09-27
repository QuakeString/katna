-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v6: scheduled mail the SMTP server holds (docs/ARCHITECTURE.md §11).

-- When scheduled mail goes out (Unix seconds). `send_at` is then when the
-- outbox hands it over: after the undo delay, to a server that holds mail
-- (RFC 4865 FUTURERELEASE), or at `hold_until` itself. A `sent` entry
-- with `hold_until` waits on the server and is filed in Sent at that time.
ALTER TABLE outbox ADD COLUMN hold_until INTEGER;
