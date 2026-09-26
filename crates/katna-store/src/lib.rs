// SPDX-License-Identifier: GPL-3.0-or-later

//! SQLite schema, migrations and message storage. Only `katna-daemon` opens the
//! databases for writing; apps open them read-only. See `docs/ARCHITECTURE.md` §5.
