#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
# Prints the version of the checked-out commit (docs/RELEASING.md): from the
# newest version tag and the commits since, `X.Y.Z.rN.gHASH` after `vX.Y.Z`
# and `X.Y.ZbetaB.rN.gHASH` after `vX.Y.Z-beta.B`, or 0.0.0.rCOUNT.gHASH
# before the first tag. Every package uses it (packaging/arch/PKGBUILD;
# ci/windows-package.ps1 does the same in PowerShell). A beta counts only
# while its release is not tagged yet, so a beta and the release promoted
# from it on the same commit give the release's version.
set -eu
describe() {
  git describe --long --tags --abbrev=7 "$@" 2> /dev/null || true
}
stable=$(describe --match 'v[0-9]*' --exclude 'v*-*')
beta=$(describe --match 'v[0-9]*-beta.[0-9]*')
core() {
  printf '%s\n' "$1" | sed 's/^v//;s/[-].*//'
}
if [ -n "$beta" ] && { [ -z "$stable" ] ||
  [ "$(core "$stable")" != "$(core "$beta")" ] &&
  [ "$(printf '%s\n%s\n' "$(core "$stable")" "$(core "$beta")" | sort -V | tail -n1)" = "$(core "$beta")" ]; }; then
  version=$beta
else
  version=$stable
fi
if [ -n "$version" ]; then
  printf '%s\n' "$version" | sed 's/^v//;s/-beta\./beta/;s/-\([0-9]*\)-g/.r\1.g/'
else
  printf '0.0.0.r%s.g%s\n' "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"
fi
