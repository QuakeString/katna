#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Counts corner radii, text sizes and spacing typed as raw numbers in the
# GPUI crates, instead of taken from katna_ui::tokens (docs/DESIGN.md).
# Fails if a count is above its budget in ci/token-budgets.txt, so the
# counts only go down. After moving code onto tokens, lower the budget to
# the new count (the script prints it).
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
budgets="$root/ci/token-budgets.txt"
dirs=(apps/katna-mail/src apps/katna-calendar/src apps/katna-setup/src
      crates/katna-ui/src crates/katna-chrome/src)

count() {
    (cd "$root" && grep -rhoE --include='*.rs' "$1" "${dirs[@]}" || true) | wc -l
}

declare -A found
found[radius]=$(count 'rounded(_[a-z_]+)?\(px\([0-9][0-9.]*\)\)')
found[text_size]=$(count 'text_size\(px\([0-9][0-9.]*\)\)')
found[spacing]=$(count '\.(gap|gap_x|gap_y|p|px|py|pt|pb|pl|pr|m|mx|my|mt|mb|ml|mr)\(px\([0-9][0-9.]*\)\)')

status=0
while read -r name max; do
    [[ -z "$name" || "$name" == \#* ]] && continue
    n=${found[$name]:-}
    if [[ -z "$n" ]]; then
        echo "UNKNOWN  $name"
        status=1
    elif (( n > max )); then
        echo "OVER     $name: $n raw numbers (budget $max); use katna_ui::tokens"
        status=1
    elif (( n < max )); then
        echo "LOWER    $name: $n raw numbers (budget $max); lower the budget to $n"
    else
        echo "OK       $name: $n raw numbers (budget $max)"
    fi
done < "$budgets"

exit "$status"
