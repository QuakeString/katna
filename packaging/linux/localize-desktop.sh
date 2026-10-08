#!/bin/sh
# SPDX-License-Identifier: GPL-3.0-or-later
# Copies a .desktop file with its names in every language Katna has: after
# each Name=, GenericName=, Comment= and Keywords= line, a Name[de]= line
# and so on from i18n/<language>/desktop.ftl, where the line's message
# is translated and differs from English. The English text and ids are in
# i18n/en/desktop.ftl; the file names its ids with a "# i18n: <prefix>"
# line (<prefix>-name, -generic-name, -comment, -keywords, and
# -action-<action> for a [Desktop Action <action>]'s Name). A file without
# that line is copied as it is.
#
#   packaging/linux/localize-desktop.sh FILE OUT [I18N_DIR]
#
# stage.sh and the Arch PKGBUILD install the .desktop files through it.
set -eu

file=$1
out=$2
root=$(cd "$(dirname "$0")/../.." && pwd)
i18n=${3:-$root/i18n}

prefix=$(sed -n 's/^# i18n: *\([a-z0-9-]*\) *$/\1/p' "$file" | head -n 1)
if [ -z "$prefix" ]; then
  cp "$file" "$out"
  exit 0
fi

# Every translation as "locale<TAB>id<TAB>text", sorted by locale.
table=$(mktemp)
sorted=$(mktemp)
trap 'rm -f "$table" "$sorted"' EXIT
for ftl in "$i18n"/*/desktop.ftl; do
  [ -f "$ftl" ] || continue
  folder=$(basename "$(dirname "$ftl")")
  # The folder's tag as a desktop-entry locale (lang_COUNTRY@MODIFIER).
  case $folder in
    en | qps-*) continue ;;
    zh-Hans) locale=zh_CN ;;
    zh-Hant) locale=zh_TW ;;
    *-Latn) locale=${folder%-Latn}@latin ;;
    *-Cyrl) locale=${folder%-Cyrl} ;;
    *-Guru) locale=${folder%-Guru} ;;
    *) locale=$(printf '%s' "$folder" | tr - _) ;;
  esac
  # Single-line messages only: a .desktop value has no line breaks, and
  # one with a { placeable } is left to English.
  awk -v locale="$locale" '
    function flush() {
      if (id != "" && ok) print locale "\t" id "\t" text
      id = ""
    }
    /^[a-z][a-z0-9-]* *=/ {
      flush()
      id = $0; sub(/ *=.*/, "", id)
      text = $0; sub(/^[^=]*= */, "", text)
      gsub(/\t/, " ", text); sub(/[ \r]+$/, "", text)
      ok = text != "" && index(text, "{") == 0
      next
    }
    /^[ \t]/ { if (id != "") ok = 0; next }
    { flush() }
    END { flush() }
  ' "$ftl" >> "$table"
done
LC_ALL=C sort -t "$(printf '\t')" -k1,1 -s "$table" > "$sorted"

awk -v prefix="$prefix" -v table="$sorted" '
  BEGIN {
    FS = "\t"
    while ((getline line < table) > 0) {
      split(line, f, "\t")
      n[f[2]]++
      where[f[2], n[f[2]]] = f[1]
      said[f[2], n[f[2]]] = f[3]
    }
    FS = " "
  }
  /^\[/ {
    group = $0; sub(/[ \t\r]+$/, "", group)
    action = ""
    if (group ~ /^\[Desktop Action /) {
      action = group; sub(/^\[Desktop Action /, "", action); sub(/\]$/, "", action)
    }
    print
    next
  }
  /^(Name|GenericName|Comment|Keywords)=/ {
    print
    key = $0; sub(/=.*/, "", key)
    value = substr($0, length(key) + 2)
    if (group == "[Desktop Entry]") {
      id = prefix "-" (key == "GenericName" ? "generic-name" : tolower(key))
    } else if (action != "" && key == "Name") {
      id = prefix "-action-" action
    } else {
      next
    }
    for (i = 1; i <= n[id]; i++) {
      text = said[id, i]
      if (text == value) continue
      if (key == "Keywords" && text !~ /;$/) text = text ";"
      print key "[" where[id, i] "]=" text
    }
    next
  }
  { print }
' "$file" > "$out"
