-- SPDX-License-Identifier: GPL-3.0-or-later
-- mail.db schema v10: delivery and read receipts per recipient
-- (docs/ARCHITECTURE.md §16.1).

-- What became of mail sent from here, by its Message-ID (without angle
-- brackets) and a recipient's address (lower case): when the mail server
-- took it for them, when their mail server took it or said it could not
-- (delivery status notifications) and when their app showed it (a read
-- receipt). The receipt mail itself stays where it is.
CREATE TABLE receipt (
    message_id_hdr TEXT    NOT NULL,
    recipient      TEXT    NOT NULL,
    sent_at        INTEGER,
    delivered_at   INTEGER,
    failed_at      INTEGER,
    read_at        INTEGER,
    PRIMARY KEY (message_id_hdr, recipient)
) WITHOUT ROWID;
