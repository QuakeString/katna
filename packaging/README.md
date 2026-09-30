<!-- SPDX-License-Identifier: GPL-3.0-or-later -->

# Packaging

Files that distribution packages install, and the Arch Linux package.

| Path | Installed as |
|---|---|
| `systemd/katna-daemon.service` | `/usr/lib/systemd/user/katna-daemon.service` (systemd user unit) |
| `dbus/<daemon bus name>.service` | `/usr/share/dbus-1/services/` (D-Bus activation, starts the unit) |
| `desktop/<mail app ID>.desktop` | `/usr/share/applications/` |
| `krunner/<mail app ID>.desktop` | `/usr/share/krunner/dbusplugins/` (KRunner results from the daemon) |
| `gnome-shell/<mail app ID>.search-provider.ini` | `/usr/share/gnome-shell/search-providers/` (GNOME search results from the daemon) |
| `kio/<mail app ID>.SendFiles.desktop` | `/usr/share/kio/servicemenus/` ("Send with Katna Mail" in Dolphin; the daemon writes the user's copy with an account submenu) |
| `nautilus/katna-mail.py` | `/usr/share/nautilus-python/extensions/` ("Send with Katna Mail" in GNOME Files; needs python-nautilus) |
| `icons/<mail app ID>.svg` | `/usr/share/icons/hicolor/scalable/apps/` |
| `icons/hicolor/<N>x<N>/apps/<mail app ID>.png` | `/usr/share/icons/hicolor/<N>x<N>/apps/` |
| `arch/PKGBUILD` | Arch Linux package `katna-git` |
| `windows/` | Katna Setup for Windows (`windows/README.md`) |

The file names are the IDs from `katna_core::ids` (`in.invenia.katna.Mail`,
`in.invenia.katna.Daemon`). `crates/katna-core/tests/packaging.rs` checks
the names and the `Name`, `Exec`, `Icon`, `StartupWMClass` and `BusName`
lines against them, and a test in `apps/katna-daemon/src/install.rs` checks
that the unit, the activation file and the KRunner and GNOME search
files match what
`katna-daemon install-user-service` writes. Other files in these folders
must not spell out an ID: install them with globs.

The icon is Katna's logo, designed by Mozammel: a script k on a teal disc.
Its sources are in `icons/src/`:

- `katna.svg`, the k with its soft shadow, for 48 px and up;
- `katna-small.svg`, the same without the shadow, which blurs to mush at
  16 to 32 px. It is also the installed scalable icon, since Qt (and so
  KDE) doesn't draw blur filters;
- `katna-symbolic.svg`, the k cut out of a one-colour disc, installed as
  `hicolor/symbolic/apps/<mail app ID>-symbolic.svg` for the tray: Plasma
  (`ColorScheme-Text`) and GNOME (`-symbolic`) recolour it to suit the
  panel. With unread mail the tray shows the coloured icon with a badge;
- `katna-wordmark.svg`, "katna mail" in script on the disc, for large
  places: About, the welcome and the README. Its strokes reach past the
  disc in white.

After changing a source, run `python3 packaging/icons/render.py` (needs
`rsvg-convert` and Pillow), then `python3 packaging/windows/make-ico.py`.
They render the PNGs, which KDE prefers at their sizes, the tray's pixels
in `crates/katna-platform/icons/` and Windows' `katna.ico`. Katna Mail
draws the sources itself.

## Arch Linux

### Prebuilt package

CI builds the PKGBUILD on every push to `main`
(`.github/workflows/arch-package.yml`) and puts the package on the
[`arch-latest`](https://github.com/QuakeString/katna/releases/tag/arch-latest)
pre-release, which each build replaces. It is x86_64 only and not signed.

Install it once:

```sh
curl -LO https://github.com/QuakeString/katna/releases/download/arch-latest/katna-git-x86_64.pkg.tar.zst
sudo pacman -U katna-git-x86_64.pkg.tar.zst
```

Or let `pacman -Syu` keep it up to date: the release is also a pacman
repository. Append this to `/etc/pacman.conf`, then run
`sudo pacman -Syu katna-git`:

```ini
[katna]
SigLevel = Optional TrustAll
Server = https://github.com/QuakeString/katna/releases/download/arch-latest
```

Once installed, Katna Mail updates itself: About shows a newer build,
the daemon downloads it (by itself unless Settings > General > Updates
says otherwise), and Update installs it with `pacman -U` after the
system's password prompt, then restarts Katna Mail. The package installs
`/usr/lib/katna/katna-update-helper` and the polkit action
`/usr/share/polkit-1/actions/in.invenia.katna.update.policy` for that
(`docs/ARCHITECTURE.md` §21.2). `pacman -Syu` keeps working as before.

### Building it yourself

```sh
cd packaging/arch
makepkg -si
```

This installs `katna-mail`, `katna-daemon` and `katnactl` in `/usr/bin`,
plus the files above. Katna Calendar is not packaged yet.

- makepkg clones the repository this folder is in (`git+file://`), so it
  builds the **committed** state of the branch that was checked out when it
  first cloned. Commit before building. To build another branch, delete
  `packaging/arch/katna` (makepkg's mirror) first.
- The build needs `cargo` and Rust at least the workspace's
  `rust-version`. With `rustup` the PKGBUILD uses the `stable` toolchain;
  Arch's `rust` package works too when it is new enough.
- Account passwords live in the Secret Service, so a provider must be
  running: GNOME Keyring, KWallet or KeePassXC.

Katna Mail starts the daemon at every login once it has been opened
(Settings > General > Desktop > Start Katna at login, on by default). D-Bus
also starts it whenever `katnactl` or Katna Mail calls it. To have it start
at login without ever opening Katna Mail, as with only `katnactl`:

```sh
systemctl --user enable --now katna-daemon
```

Turning "Start Katna at login" off disables that unit too.

Add an account and watch it sync:

```sh
katnactl add-imap you@example.org --imap imap.example.org --smtp smtp.example.org
katnactl status
katnactl list 1
```

`add-imap` asks for the password (use an app password for Gmail, Yahoo,
iCloud and Fastmail, whose servers it knows; others need `--imap`).
`katnactl --help` lists every command.

To remove: `pacman -R katna-git`. Your mail and settings stay in
`~/.local/share/katna` and `~/.config/katna`.
