#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Fails if a release binary is larger than its budget in ci/size-budgets.txt.
set -euo pipefail

budgets="$(dirname "$0")/size-budgets.txt"
target_dir="${CARGO_TARGET_DIR:-target}/release"
status=0

while read -r name max; do
    [[ -z "$name" || "$name" == \#* ]] && continue
    binary="$target_dir/$name"
    if [[ ! -f "$binary" ]]; then
        echo "MISSING  $name ($binary)"
        status=1
        continue
    fi
    size=$(stat -c %s "$binary")
    if (( size > max )); then
        echo "OVER     $name: $size bytes (budget $max)"
        status=1
    else
        echo "OK       $name: $size bytes (budget $max)"
    fi
done < "$budgets"

exit "$status"
