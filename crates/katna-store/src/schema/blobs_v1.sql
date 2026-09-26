-- SPDX-License-Identifier: GPL-3.0-or-later
-- blobs.db schema v1 (docs/ARCHITECTURE.md §5.2): content-addressed blobs.
--
-- `storage` 0: `data` holds the zstd-compressed bytes.
-- `storage` 1: the bytes are a file in attachments/ and `data` is NULL.
-- `size` is the uncompressed length.

CREATE TABLE blob (
    hash    BLOB    PRIMARY KEY CHECK (length(hash) = 32),
    size    INTEGER NOT NULL,
    storage INTEGER NOT NULL CHECK (storage IN (0, 1)),
    data    BLOB,
    CHECK ((storage = 0) = (data IS NOT NULL))
);
