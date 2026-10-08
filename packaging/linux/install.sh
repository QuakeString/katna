#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
# Installs Katna from this unpacked folder (the katna-linux-x86_64 tarball).
#
#   ./install.sh               for you only, into ~/.local
#   sudo ./install.sh --prefix /usr/local
#                              for everyone on this computer
#   ./install.sh --uninstall   (with the same --prefix) removes it again
#
# Your mail and settings in ~/.local/share/katna and ~/.config/katna stay.
set -eu

here=$(cd "$(dirname "$0")" && pwd)
prefix=
uninstall=false
while [ $# -gt 0 ]; do
  case $1 in
    --prefix) prefix=$2; shift 2 ;;
    --prefix=*) prefix=${1#--prefix=}; shift ;;
    --uninstall) uninstall=true; shift ;;
    -h | --help) sed -n '3,10s/^# \{0,1\}//p' "$0"; exit 0 ;;
    *) echo "install.sh: unknown option $1" >&2; exit 2 ;;
  esac
done
if [ -z "$prefix" ]; then
  if [ "$(id -u)" = 0 ]; then prefix=/usr/local; else prefix=$HOME/.local; fi
fi
case $prefix in /*) ;; *) prefix=$(pwd)/$prefix ;; esac

# systemd reads user units from these folders, not from PREFIX/lib.
if [ "$prefix" = "$HOME/.local" ]; then
  units=${XDG_DATA_HOME:-$HOME/.local/share}/systemd/user
else
  units=$prefix/lib/systemd/user
fi

# Every file this installs, relative to the tarball's share/ and bin/.
files=$(cd "$here" && find bin share -type f | sort)
unit_files=$(cd "$here/lib/systemd/user" && ls)

if $uninstall; then
  systemctl --user disable --now $unit_files > /dev/null 2>&1 || true
  for file in $files; do rm -f "$prefix/$file"; done
  for file in $unit_files; do rm -f "$units/$file"; done
  echo "Katna is removed from $prefix."
  exit 0
fi

for file in $files; do
  mkdir -p "$(dirname "$prefix/$file")"
  case $file in
    share/dbus-1/services/*)
      sed "s|/usr/bin/katna-daemon|$prefix/bin/katna-daemon|" "$here/$file" > "$prefix/$file" ;;
    share/applications/*)
      # Start the programs installed here even when PREFIX/bin is not on
      # PATH, as with ~/.local/bin on many desktops.
      sed "s|^Exec=katna-mail|Exec=$prefix/bin/katna-mail|" "$here/$file" > "$prefix/$file" ;;
    # Beside the old file, then in its place: a running Katna keeps its
    # program, and Katna can install its own update.
    *) cp "$here/$file" "$prefix/$file.new" && mv -f "$prefix/$file.new" "$prefix/$file" ;;
  esac
  case $file in bin/*) chmod 755 "$prefix/$file" ;; *) chmod 644 "$prefix/$file" ;; esac
done
mkdir -p "$units"
for file in $unit_files; do
  sed "s|/usr/bin/katna-daemon|$prefix/bin/katna-daemon|" "$here/lib/systemd/user/$file" > "$units/$file"
  chmod 644 "$units/$file"
done

# Menus and icons notice the new files sooner with these, where they exist.
update-desktop-database -q "$prefix/share/applications" 2> /dev/null || true
gtk-update-icon-cache -q -t "$prefix/share/icons/hicolor" 2> /dev/null || true
systemctl --user daemon-reload 2> /dev/null || true

echo "Katna is installed in $prefix. Open Katna Mail from your apps menu,"
echo "or run $prefix/bin/katna-mail."
