<!-- SPDX-License-Identifier: GPL-3.0-or-later -->

# Packaging

Files that distribution packages install, and the Arch Linux package.

| Path | Installed as |
|---|---|
| `systemd/katna-daemon.service` | `/usr/lib/systemd/user/katna-daemon.service` (systemd user unit) |
| `dbus/<daemon bus name>.service` | `/usr/share/dbus-1/services/` (D-Bus activation, starts the unit) |
| `desktop/<mail app ID>.desktop` | `/usr/share/applications/` |
| `desktop/<mail app ID>.Notifications.desktop` | `/usr/share/applications/` (hidden; what notifications name as their app, without launch feedback) |
| `krunner/<mail app ID>.desktop` | `/usr/share/krunner/dbusplugins/` (KRunner results from the daemon) |
| `gnome-shell/<mail app ID>.search-provider.ini` | `/usr/share/gnome-shell/search-providers/` (GNOME search results from the daemon) |
| `kio/<mail app ID>.SendFiles.desktop` | `/usr/share/kio/servicemenus/` ("Send with Katna Mail" in Dolphin; the daemon writes the user's copy with an account submenu) |
| `nautilus/katna-mail.py` | `/usr/share/nautilus-python/extensions/` ("Send with Katna Mail" in GNOME Files; needs python-nautilus) |
| `icons/<mail app ID>.svg` | `/usr/share/icons/hicolor/scalable/apps/` |
| `icons/hicolor/<N>x<N>/apps/<mail app ID>.png` | `/usr/share/icons/hicolor/<N>x<N>/apps/` |
| `arch/PKGBUILD` | Arch Linux package `katna-git` |
| `windows/` | Katna Setup for Windows (`windows/README.md`) |
| `linux/` | `stage.sh` (the files above under a prefix, for every package below), the plain tarball and its `install.sh` |
| `fedora/katna.spec` | Fedora RPM `katna` |
| `nix/package.nix` | Nix package (`flake.nix` at the top builds it) |
| `appimage/` | `Katna-x86_64.AppImage` |
| `snap/snapcraft.yaml` | Snap `katna` |
| `flatpak/<ID prefix>.yml` | Flatpak |

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

## Other Linux packages

CI builds these on every push to `main`
(`.github/workflows/linux-packages.yml`), installs each on its own platform
and tries it there: D-Bus starts `katna-daemon` for `katnactl status`, and
Katna Mail opens a window (`ci/linux-package-test.sh`). Once CI on `main`
has passed, they replace the files on the
[`linux-latest`](https://github.com/QuakeString/katna/releases/tag/linux-latest)
pre-release, with a screenshot of each running. They are x86_64 only, not
signed, in no store, and do not update themselves: their own package
manager, or a new download, updates them.

The AppImage, Snap, Flatpak and tarball share one build made on Ubuntu
22.04, so they need glibc 2.35 or newer. The RPM and the Nix package are
built from source by Fedora and Nix.

### Fedora

```sh
sudo dnf install https://github.com/QuakeString/katna/releases/download/linux-latest/katna-x86_64.rpm
```

To build it yourself (Fedora's Rust must be at least the workspace's
`rust-version`):

```sh
sudo dnf install rpm-build rpmdevtools dnf-plugins-core
sudo dnf builddep packaging/fedora/katna.spec
rpmdev-setuptree
git archive --prefix=katna/ -o ~/rpmbuild/SOURCES/katna.tar.gz HEAD
rpmbuild -bb packaging/fedora/katna.spec \
  --define "katna_version $(packaging/linux/version.sh)" \
  --define "katna_built $(git log -1 --format=%ct)"
```

### Nix

```sh
nix run github:QuakeString/katna                # try it
nix profile install github:QuakeString/katna    # install it
```

Nix builds it from source (there is no binary cache yet), without the
Sign in with Google, Microsoft and Zoho buttons, whose app keys only CI
has. On NixOS, add the package to `environment.systemPackages` and
`services.dbus.packages` so D-Bus finds the service. Elsewhere the session
bus finds it through `~/.nix-profile/share` in `XDG_DATA_DIRS`.

### AppImage

Download `Katna-x86_64.AppImage`, make it executable and run it. It needs
FUSE 2 (`libfuse2`). Each start writes
`~/.local/share/dbus-1/services/<daemon bus name>.service`, which runs the
AppImage as `katna-daemon` wherever it now is, so keep it in one place
(such as `~/Applications`). `Katna-x86_64.AppImage katnactl status` runs
the command-line tool.

### Snap

```sh
sudo snap install --dangerous katna_amd64.snap
sudo snap connect katna:password-manager-service
sudo snap connect katna:daemon-client katna:daemon-dbus
sudo snap connect katna:mail-client katna:mail-dbus
```

The store would make those connections itself once it approves them; a
local install does not. `katna.katnactl` is the command-line tool. snapd's
user daemons are still experimental, so the Snap has no D-Bus activation
file: Katna Mail and `katnactl` start the service beside them when it is
not running (`katna_dbus::ensure_daemon`), and "Start Katna at login"
starts it through Katna Mail.

### Flatpak

```sh
flatpak install --user katna-x86_64.flatpak
```

The bundle fetches the Freedesktop runtime from Flathub. The Flatpak's ID
is the prefix of Katna's IDs, so it may own Katna Mail's and the service's
D-Bus names and export the service's activation file; D-Bus starts the
service inside the sandbox. "Start Katna at login" does not work from the
Flatpak yet (it needs the Background portal, `docs/ARCHITECTURE.md` §9.2).

### Any other Linux

`katna-linux-x86_64.tar.gz` holds the programs and the files above, laid
out as under `/usr`:

```sh
tar xzf katna-linux-x86_64.tar.gz
katna-linux-x86_64/install.sh                            # into ~/.local
sudo katna-linux-x86_64/install.sh --prefix /usr/local   # or for everyone
```

`install.sh --uninstall` (with the same `--prefix`) removes it. Katna
Mail also runs straight from the unpacked folder: with no activation file
installed, it starts the `katna-daemon` beside it.
