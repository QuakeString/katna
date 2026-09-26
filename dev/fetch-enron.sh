#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Downloads and unpacks the Enron email corpus (CMU release of 7 May 2015,
# ~423 MB compressed, ~517k messages) for search and storage benchmarks.
# See docs/IMPLEMENTATION_PLAN.md §3.2. The corpus is never committed.
#
# Usage: dev/fetch-enron.sh [target-dir]
#
#   target-dir        Where to put the corpus. Default:
#                     ${KATNA_CORPUS_DIR:-${XDG_CACHE_HOME:-$HOME/.cache}/katna/corpora}/enron
#   ENRON_URL         Override the download URL (for a mirror).
#   ENRON_SHA256      Expected SHA-256 of the archive. When set, a mismatch aborts.
#
# Result: <target-dir>/maildir/<user>/<folder>/<n>. — one file per message.
# The script is idempotent: a finished download or extraction is not repeated.
set -euo pipefail

url="${ENRON_URL:-https://www.cs.cmu.edu/~enron/enron_mail_20150507.tar.gz}"
default_root="${KATNA_CORPUS_DIR:-${XDG_CACHE_HOME:-$HOME/.cache}/katna/corpora}"
target="${1:-$default_root/enron}"
archive="$target/enron_mail_20150507.tar.gz"
maildir="$target/maildir"

for tool in curl tar sha256sum; do
    command -v "$tool" >/dev/null || { echo "error: '$tool' is required" >&2; exit 1; }
done

mkdir -p "$target"

if [[ -f "$maildir/.complete" ]]; then
    echo "Enron corpus already present: $maildir"
    exit 0
fi

if [[ ! -f "$archive" ]]; then
    echo "Downloading $url"
    # Resume a partial download; write to .part so an interrupted run is never
    # mistaken for a complete archive.
    curl --fail --location --retry 3 --continue-at - \
        --output "$archive.part" "$url"
    mv "$archive.part" "$archive"
fi

actual="$(sha256sum "$archive" | cut -d' ' -f1)"
if [[ -n "${ENRON_SHA256:-}" ]]; then
    if [[ "$actual" != "$ENRON_SHA256" ]]; then
        echo "error: checksum mismatch for $archive" >&2
        echo "  expected $ENRON_SHA256" >&2
        echo "  actual   $actual" >&2
        echo "Delete the archive and run again." >&2
        exit 1
    fi
else
    echo "SHA-256: $actual (set ENRON_SHA256 to verify)"
fi

echo "Extracting to $target"
rm -rf "$maildir"
tar -xzf "$archive" -C "$target"
if [[ ! -d "$maildir" ]]; then
    echo "error: archive did not contain a maildir/ directory" >&2
    exit 1
fi
touch "$maildir/.complete"

count="$(find "$maildir" -type f ! -name .complete | wc -l)"
echo "Done: $count messages in $maildir"
