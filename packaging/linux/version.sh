#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
# Prints the version of the checked-out commit the way the Arch package
# names it (packaging/arch/PKGBUILD, pkgver): the newest version tag and
# the commits since, or 0.0.0.rCOUNT.gHASH before the first tag.
set -eu
if version=$(git describe --long --tags --abbrev=7 --match 'v[0-9]*' 2> /dev/null); then
  printf '%s\n' "${version#v}" | sed 's/\([^-]*-g\)/r\1/;s/-/./g'
else
  printf '0.0.0.r%s.g%s\n' "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"
fi
