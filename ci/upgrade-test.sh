#!/bin/bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Upgrade and rollback test (docs/IMPLEMENTATION_PLAN.md U.7): the previous
# Arch package syncs an account on the dev Dovecot, the candidate is
# installed while its daemon runs, and then the previous one again.
#
#   ci/upgrade-test.sh PREVIOUS.pkg.tar.zst CANDIDATE.pkg.tar.zst
#
# Runs as root in an archlinux container sharing the host's network, with
# the dev servers up (`docker compose -f dev/compose.yaml up -d dovecot`).
# Used by .github/workflows/arch-package.yml before a build is published.
#
# Each install replaces katna-daemon while it runs, so the running daemon
# has to notice and restart into the new build by itself, without systemd
# (docs/ARCHITECTURE.md §21.2, Running while updated). The mail it synced
# must still be there after each step. Going back to the previous build
# may meet a database it is too old to open: that is reported, and the
# candidate must then open the same data again, unchanged.
set -euo pipefail

previous=$(realpath "$1")
candidate=$(realpath "$2")

if [ -z "${KATNA_TEST_SESSION:-}" ]; then
  pacman -Syu --noconfirm --needed dbus gnome-keyring curl > /dev/null
  pacman -U --noconfirm "$previous" > /dev/null
  XDG_RUNTIME_DIR=$(mktemp -d)
  chmod 700 "$XDG_RUNTIME_DIR"
  export XDG_RUNTIME_DIR KATNA_TEST_SESSION=1
  exec dbus-run-session -- "$0" "$@"
fi

version_of() {
  local version
  version=$(pacman -Qp "$1" 2> /dev/null | cut -d' ' -f2)
  echo "${version%-*}"
}
from=$(version_of "$previous")
to=$(version_of "$candidate")
echo "== upgrade $from -> $to -> $from"
if [ "$from" = "$to" ]; then
  echo "the previous build is this one; nothing to upgrade"
  exit 0
fi

# Account passwords go to the Secret Service.
mkdir -p "$HOME/.local/share/keyrings"
eval "$(printf test | gnome-keyring-daemon --unlock --components=secrets)"
export GNOME_KEYRING_CONTROL

# Dovecot takes any user name with the one password.
user="upgrade-$$-$(date +%s)@katna.test"
password=katna-dev
imap=imaps://127.0.0.1:${KATNA_DOVECOT_IMAPS_PORT:-20993}

fail() {
  echo "FAILED: $*" >&2
  exit 1
}

# Waits up to $1 seconds for the command after it to succeed.
wait_for() {
  local seconds=$1
  shift
  for _ in $(seq "$seconds"); do
    if "$@" > /dev/null 2>&1; then return 0; fi
    sleep 1
  done
  return 1
}

append() {
  printf 'From: Ada <ada@katna.test>\r\nTo: %s\r\nSubject: Upgrade test %s\r\nMessage-ID: <upgrade-%s-%s@katna.test>\r\nDate: %s\r\n\r\nHello.\r\n' \
    "$user" "$1" "$1" "$$" "$(date -R)" > "$XDG_RUNTIME_DIR/mail.eml"
  curl -sSk --url "$imap/INBOX" -u "$user:$password" -T "$XDG_RUNTIME_DIR/mail.eml"
}

# How many test messages the store has, read as Katna Mail reads it.
stored() {
  katnactl list "$account" --limit 1000 2> /dev/null | grep -c 'Upgrade test' || true
}

has_mail() {
  [ "$(stored)" = "$1" ]
}

# The build of the running daemon, as Version() answers it; "older" from
# a daemon before Version(); nothing when none answers.
running() {
  local reply
  reply=$(dbus-send --session --print-reply --reply-timeout=5000 \
    --dest=in.invenia.katna.Daemon /in/invenia/katna/Pim1 \
    in.invenia.katna.Pim1.Version 2>&1) || true
  if grep -q UnknownMethod <<< "$reply"; then
    echo older
  else
    grep -o 'string "[^"]*"' <<< "$reply" | head -n 1 | cut -d'"' -f2
  fi
}

# Whether a daemon runs, and another build than $1.
moved_from() {
  local now
  now=$(running)
  [ -n "$now" ] && [ "$now" != "$1" ]
}

synced() {
  katnactl sync "$account" > /dev/null 2>&1 || true
  wait_for 60 has_mail "$1"
}

echo "== $from: add the account and sync"
for n in 1 2 3; do append "$n"; done
wait_for 30 katnactl status || fail "katna-daemon $from did not start"
echo "$password" | katnactl add-imap "$user" --imap "127.0.0.1:${KATNA_DOVECOT_IMAPS_PORT:-20993}" \
  --security tls --insecure
account=$(katnactl status | awk -v user="$user" '$2 == user {print $1; exit}')
[ -n "$account" ] || fail "the account was not added"
synced 3 || fail "$from did not sync the three messages (has $(stored))"
before=$(running)
echo "running: $before"

echo "== install $to while the daemon runs"
pacman -U --noconfirm "$candidate" > /dev/null
wait_for 120 moved_from "$before" || fail "katna-daemon did not restart into $to"
[ "$(running)" = "$to" ] || fail "running $(running), not $to"
has_mail 3 || fail "$to lost mail: $(stored) of 3"
append 4
synced 4 || fail "$to did not sync new mail (has $(stored))"
echo "ok: $to runs and kept the mail"

echo "== go back to $from while the daemon runs"
pacman -U --noconfirm "$previous" > /dev/null 2>&1
if wait_for 120 moved_from "$to"; then
  has_mail 4 || fail "$from lost mail after going back: $(stored) of 4"
  append 5
  synced 5 || fail "$from did not sync after going back (has $(stored))"
  echo "ok: $from runs again on the data $to left"
else
  # It may refuse a database migrated past what it knows (SchemaTooNew)
  # until U.4's min_reader_version; the data must survive that.
  echo "::warning::katna-daemon $from did not run again on the data $to left; checking it is intact"
  pacman -U --noconfirm "$candidate" > /dev/null
  wait_for 60 katnactl status || fail "$to did not start again after the rollback"
  has_mail 4 || fail "the rollback lost mail: $(stored) of 4"
  echo "ok: the data survived the rollback, and $to opens it again"
fi
echo "== passed"
