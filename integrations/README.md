<!-- SPDX-License-Identifier: GPL-3.0-or-later -->

# Desktop integrations

Katna's parts that live inside the desktop itself, in the desktop's own
languages (QML, GNOME Shell's JavaScript). They only show data from
`katna-daemon` and send actions back to it; nothing is stored or decided
here (`docs/ARCHITECTURE.md` §15.4, §15.6). The Arch package installs all
of them; each desktop ignores the other's files.

| Path | What | Installed as |
|---|---|---|
| `plasma-clock/package/` | Katna Digital Clock, a Plasma widget | `/usr/share/plasma/plasmoids/<plugin ID>/` |
| `gnome-shell-extension/<UUID>/` | Katna in GNOME Shell's clock menu | `/usr/share/gnome-shell/extensions/<UUID>/` |
| `po/` | Their translations (gettext domain `katna-clock`) | `/usr/share/locale/<lang>/LC_MESSAGES/katna-clock.mo` |

The plugin ID and UUID are `katna_core::ids::CLOCK_APPLET_ID` and
`CLOCK_EXTENSION_UUID`; `crates/katna-core/tests/packaging.rs` checks the
files against them.

Both read `in.invenia.katna.Agenda1` from the daemon
(`crates/katna-dbus/src/agenda.rs`): events for a range of days, and
tasks, which they add and tick off. A call starts the daemon (D-Bus
activation). Until Katna syncs calendars there are no Katna events; the
tasks are Katna's own, kept in `pim.db`.

## Katna Digital Clock (Plasma)

A copy of Plasma's own Digital Clock (`applets/digital-clock` in
plasma-workspace), so it looks and behaves exactly like the clock people
know, with Katna's additions:

- a Tasks list under the day's events: add a task (due on the day picked
  in the month, when that isn't today), tick one off;
- Katna's events open in Katna, and a meeting gets a Join button.

It declares the same `X-Plasma-Provides` as Plasma's clock, so right-click
the clock > Show Alternatives swaps it in.

Base: **plasma-workspace v6.7.5**. The first commit that added this folder
is the unchanged copy, so `git log -p integrations/plasma-clock` shows every
Katna change. Katna's edits to the copied files are marked `// Katna:`;
Katna's own code is in the `Katna*.qml` files. The copied files keep their
licences (GPL-2.0-or-later, some GPL-2.0-only OR GPL-3.0-only, one
LGPL-2.0-or-later), all of which may be combined into Katna under
GPL-3.0-or-later.

It imports Plasma's shared modules from the system rather than copying
them: the month view (`org.kde.plasma.workspace.calendar`), the clock's
helpers (`org.kde.plasma.private.digitalclock`) and D-Bus
(`org.kde.plasma.workspace.dbus`). So it gets KDE's fixes to them, and it
is pure QML: nothing to compile. The upstream text calls
`i18nd("plasma_applet_org.kde.plasma.digitalclock", …)`, so it keeps
Plasma's translations; Katna's text uses the `katna-clock` domain.

To move to a newer Plasma: copy the new release's QML files over
`contents/ui/` and `contents/config/`, then put back the `// Katna:` parts
and the `i18nd` domain (compare with the previous base's diff).

Try it without installing:
`plasmoidviewer -a integrations/plasma-clock/package` (from plasma-sdk).

## Katna in the clock (GNOME Shell)

An extension that adds Katna's events to GNOME's own clock menu (GNOME's
Evolution Data Server events stay) and a Tasks card under them. It uses
GNOME's own event list and styles, only wrapping the menu's event source.
`katna-daemon` switches it on once, the first time GNOME Shell knows it
(a newly installed extension is found at the next sign-in); turning it
off after that is the user's choice. Shell versions: 48 and later.

Try it: copy the `<UUID>` folder to `~/.local/share/gnome-shell/extensions/`,
sign in again, then `gnome-extensions enable <UUID>`.

## Translations

`po/update-pot.sh` rewrites `po/katna-clock.pot` from the Katna strings of
both (needs gettext). A language is `po/<lang>.po`; the package compiles
each one. Other languages fall back to English. Katna Digital Clock's name
and description are in its `metadata.json` (`Name[<lang>]`,
`Description[<lang>]`).
