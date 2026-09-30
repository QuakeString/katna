#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
# Makes demo-data.tar.gz: a Katna store holding the made-up mail of
# dev/seed/mail in a local account, and settings that skip the welcome and
# What's new, for the screenshots of .github/workflows/linux-packages.yml.
# Unpack it into a home folder (it holds .local/share/katna and
# .config/katna).
#
#   ci/linux-demo-data.sh BINDIR OUT.tar.gz
#
# BINDIR holds katna-search-cli and katna-mail of the same commit.
set -eu

bindir=$1
mkdir -p "$(dirname "$2")"
out=$(cd "$(dirname "$2")" && pwd)/$(basename "$2")
root=$(cd "$(dirname "$0")/.." && pwd)
work=$(mktemp -d)

"$bindir/katna-search-cli" import --data-dir "$work/store" \
  --account alice@katna.test "$root/dev/seed/mail"
mkdir -p "$work/home/.local/share" "$work/home/.config/katna"
mv "$work/store/data" "$work/home/.local/share/katna"
shown=$("$bindir/katna-mail" --highlights | sed 's/"name":"\([^"]*\)"/\n\1\n/g' |
  grep -E '^[0-9]{4}-[0-9]{2}-[0-9]{2}-[0-9]{4}-' | sed 's/.*/"&"/' | paste -sd, -)
cat > "$work/home/.config/katna/config.toml" <<TOML
[onboarding]
done = true
whats_new_shown = [$shown]
last_version = "$("$bindir/katna-mail" --version | cut -d' ' -f2)"

[feedback]
send_crash_reports = false
TOML
tar -C "$work/home" -czf "$out" .local .config
rm -rf "$work"
