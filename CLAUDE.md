# Katna — notes for AI-assisted development

Katna is a Linux PIM suite in Rust: Katna Mail, Katna Calendar, the
`katna-daemon` background service and (later) Katna Server.

Read first:

- `docs/ARCHITECTURE.md` — what we build and why.
- `docs/IMPLEMENTATION_PLAN.md` — phases, tasks and "done when" checks.

## Commands

```sh
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
cargo deny check                         # licenses, bans, advisories
cargo build --release --workspace --bins \
  && cargo build --release -p katna-daemon -p katnactl -p katna-search-cli -p katna-bench \
  && ci/check-sizes.sh                   # non-GUI bins rebuilt without GPUI's features
ci/gen-credits.sh                        # after adding or removing a dependency
```

The toolchain is always the latest stable Rust (`rust-toolchain.toml`).

## Layout

- `crates/` — libraries (`katna-core`, `katna-store`, `katna-search`, …).
- `apps/` — `katna-daemon`, `katna-mail`, `katna-calendar`.
- `tools/` — `katna-search-cli`, `katna-bench`, `katnactl`.
- `docs/` — architecture and plan. `ci/` — CI helper scripts.

## Rules

- License: GPL-3.0-or-later. Every source file starts with
  `// SPDX-License-Identifier: GPL-3.0-or-later`.
- Only `katna-daemon` writes the databases and talks to the network; apps
  read the store read-only and send commands over D-Bus (`katna-dbus`).
- All SQL lives in `katna-store`.
- GPUI is only allowed in `katna-ui`, `katna-chrome`, `katna-mail` and
  `katna-calendar` (enforced by `deny.toml`). `katna-daemon` must never
  depend on GPUI.
- TLS is `rustls` only; no OpenSSL.
- Pimalaya and GPUI types never appear in engine or store APIs.
- App IDs and D-Bus names come from `katna_core::ids` only. The prefix is
  `in.invenia.katna` (domain `katna.invenia.in`); never hard-code it elsewhere.
- No `unsafe` code (workspace lint `unsafe_code = "forbid"`).
- Keep binary sizes within `ci/size-budgets.txt`.
- Commit messages: `area: summary` (for example `search: add date filters`).
- Update `docs/` when a design decision changes.
- A change people will notice in Katna Mail appends a highlight to
  `apps/katna-mail/src/whats_new.rs` (next id; an animated WebP only for a
  major feature), so What's new shows it after the update.
- Lengths in the GPUI crates use `katna_ui::px` and read GPUI's back with
  `katna_ui::unpx`, never `gpui::px` or `f32::from(Pixels)`, so Settings >
  Appearance > Scaling applies everywhere (`crates/katna-ui/src/scale.rs`;
  clippy's `disallowed-methods` stops `gpui::px`).
- Text people see goes through `katna_i18n::tr!("id")`, never a string
  literal, with the English message added to `i18n/en/<binary>.ftl` in the
  same PR (`cargo test -p katna-i18n` checks every id). Other languages fall
  back to English until drafted; see `i18n/README.md`. Dates and numbers go
  through `katna_i18n::format`, never `strftime` or `{}`. Existing literals
  are converted area by area (plan L.4).
