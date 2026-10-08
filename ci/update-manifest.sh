#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Writes DIR/katna-update.json, what Katna checks for updates
# (katna_core::update::Manifest, docs/ARCHITECTURE.md §21.2), for Katna
# Setup on Windows and for the Linux packages: the version, FILE (in DIR)
# with its SHA-256 and size, when SRC's newest commit was made and which it
# is, the first 50 What's new highlights (HIGHLIGHTS, from
# `katna-mail --highlights`) and SRC's latest 200 commits for the Update
# dialog. Each PACKAGE=FILE after it goes under `files`, which the Linux
# packages pick theirs from. The Arch package writes its own (signed, with
# patches) in arch-package.yml.
#
#   ci/update-manifest.sh SRC DIR VERSION HIGHLIGHTS FILE [PACKAGE=FILE...]
set -euo pipefail
src=$1 dir=$2 version=$3 highlights=$4 file=$5
shift 5

entry() {
  jq -n --arg file "$1" --arg sha256 "$(sha256sum "$dir/$1" | cut -d' ' -f1)" \
    --argjson size "$(stat -c %s "$dir/$1")" '{$file, $sha256, $size}'
}
files='{}'
for package in "$@"; do
  files=$(jq --arg name "${package%%=*}" --argjson entry "$(entry "${package#*=}")" \
    '. + {($name): $entry}' <<<"$files")
done
changes=$(git -C "$src" log -200 --format='%h%x09%s' HEAD |
  jq -R 'split("\t") | {commit: .[0], title: (.[1:] | join("\t"))}' | jq -sc .)
jq -n \
  --argjson entry "$(entry "$file")" \
  --arg version "$version" \
  --argjson built "$(git -C "$src" log -1 --format=%ct HEAD)" \
  --arg commit "$(git -C "$src" rev-parse HEAD)" \
  --argjson highlights "$(jq -c '.[:50]' "$highlights" 2> /dev/null || echo '[]')" \
  --argjson changes "$changes" \
  --argjson files "$files" \
  '$entry + {$version, $built, $commit, $highlights, $changes}
    + if $files == {} then {} else {$files} end' > "$dir/katna-update.json"
jq '{version, file, files: ((.files // {}) | map_values(.size)),
     highlights: (.highlights | length), changes: (.changes | length)}' \
  "$dir/katna-update.json"
