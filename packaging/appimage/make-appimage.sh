#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
# Builds Katna-x86_64.AppImage from built binaries.
#
#   packaging/appimage/make-appimage.sh BINDIR OUTDIR
#
# Needs appimagetool on PATH (https://github.com/AppImage/appimagetool).
# Libraries every Linux desktop has (glibc, libxcb, libxkbcommon,
# fontconfig, freetype, Wayland, Vulkan and EGL) are not bundled, so the
# binaries must be built on a system as old as the oldest one to support.
set -eu

bindir=$1
outdir=$2
here=$(cd "$(dirname "$0")" && pwd)
appdir=$(mktemp -d)/Katna.AppDir

"$here/../linux/stage.sh" "$bindir" "$appdir" /usr
install -m 755 "$here/AppRun" "$appdir/AppRun"
# appimagetool takes the desktop entry and icon from the AppDir's top.
desktop=$(ls "$appdir"/usr/share/applications/*.desktop)
cp "$desktop" "$appdir/"
icon=$(sed -n 's/^Icon=//p' "$desktop" | head -n1)
cp "$appdir/usr/share/icons/hicolor/256x256/apps/$icon.png" "$appdir/$icon.png"
ln -s "$icon.png" "$appdir/.DirIcon"

mkdir -p "$outdir"
ARCH=x86_64 appimagetool --no-appstream "$appdir" "$outdir/Katna-x86_64.AppImage"
