#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
# Installs built Katna binaries and the files in packaging/ into a staging
# tree, as the Arch PKGBUILD's package() does, for the other Linux packages
# (Fedora, Nix, AppImage, Snap, Flatpak and the plain tarball).
#
#   packaging/linux/stage.sh BINDIR DESTDIR [PREFIX]
#
# BINDIR holds katna-mail, katna-daemon and katnactl. Files go to
# DESTDIR/PREFIX (PREFIX defaults to /usr); the D-Bus activation file and
# the systemd user unit name PREFIX/bin/katna-daemon. Nothing here spells
# out an ID from katna_core::ids: files are installed with globs.
set -eu

bindir=$1
destdir=$2
prefix=${3:-/usr}
root=$(cd "$(dirname "$0")/../.." && pwd)
out="$destdir$prefix"

put() { # MODE DIR FILE...
  mode=$1 dir=$2
  shift 2
  mkdir -p "$dir"
  for file in "$@"; do
    install -m "$mode" "$file" "$dir/"
  done
}

# .desktop files, with their names in every language (localize-desktop.sh).
put_desktop() { # DIR FILE...
  dir=$1
  shift
  mkdir -p "$dir"
  for file in "$@"; do
    sh "$root/packaging/linux/localize-desktop.sh" "$file" "$dir/$(basename "$file")"
    chmod 644 "$dir/$(basename "$file")"
  done
}

for bin in katna-mail katna-daemon katnactl; do
  put 755 "$out/bin" "$bindir/$bin"
done

# The daemon's unit and D-Bus activation file, pointing at this prefix.
mkdir -p "$out/lib/systemd/user" "$out/share/dbus-1/services"
for file in "$root"/packaging/systemd/*.service; do
  sed "s|/usr/bin/katna-daemon|$prefix/bin/katna-daemon|" "$file" \
    > "$out/lib/systemd/user/$(basename "$file")"
done
for file in "$root"/packaging/dbus/*.service; do
  sed "s|/usr/bin/katna-daemon|$prefix/bin/katna-daemon|" "$file" \
    > "$out/share/dbus-1/services/$(basename "$file")"
done
chmod 644 "$out"/lib/systemd/user/* "$out"/share/dbus-1/services/*

put_desktop "$out/share/applications" "$root"/packaging/desktop/*.desktop
put_desktop "$out/share/krunner/dbusplugins" "$root"/packaging/krunner/*.desktop
put 644 "$out/share/gnome-shell/search-providers" "$root"/packaging/gnome-shell/*.ini
# "Send with Katna Mail" in Dolphin and GNOME Files.
put_desktop "$out/share/kio/servicemenus" "$root"/packaging/kio/*.desktop
put 644 "$out/share/nautilus-python/extensions" "$root"/packaging/nautilus/*.py
put 644 "$out/share/icons/hicolor/scalable/apps" "$root"/packaging/icons/*.svg
for dir in "$root"/packaging/icons/hicolor/*/apps; do
  size=${dir#"$root/packaging/icons/hicolor/"}
  put 644 "$out/share/icons/hicolor/$size" "$dir"/*
done

# The desktop clock (integrations/README.md). The Plasma widget's folder is
# named by its plugin ID.
clock=$(sed -n 's/^ *"Id": "\(.*\)",\{0,1\}$/\1/p' \
  "$root/integrations/plasma-clock/package/metadata.json")
(cd "$root/integrations/plasma-clock/package" && find . -type f) | while read -r file; do
  put 644 "$out/share/plasma/plasmoids/$clock/$(dirname "${file#./}")" \
    "$root/integrations/plasma-clock/package/$file"
done
(cd "$root/integrations/gnome-shell-extension" && find . -type f) | while read -r file; do
  put 644 "$out/share/gnome-shell/extensions/$(dirname "${file#./}")" \
    "$root/integrations/gnome-shell-extension/$file"
done
if command -v msgfmt > /dev/null; then
  for file in "$root"/integrations/po/*.po; do
    [ -e "$file" ] || continue
    lang=$(basename "$file" .po)
    mkdir -p "$out/share/locale/$lang/LC_MESSAGES"
    msgfmt -o "$out/share/locale/$lang/LC_MESSAGES/katna-clock.mo" "$file"
  done
fi

put 644 "$out/share/licenses/katna" "$root/LICENSE"
