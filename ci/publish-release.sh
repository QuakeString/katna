#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Publishes every file in DIR as the GitHub release TAG on commit SHA
# (docs/RELEASING.md), for release.yml and promote.yml. Needs GH_TOKEN and
# GH_REPO.
#
#   ci/publish-release.sh [--rolling] [--prerelease] [--latest] TAG SHA NAME NOTES DIR
#
# A version tag is never moved: if it exists it must already name SHA. A
# rolling tag (beta-latest, stable-latest) moves to SHA, and its release is
# updated in place, never deleted and created again, as arch-latest is, and
# drops the files DIR does not have. The update manifests go up last, so
# none names a file that is not there yet. Fails unless the release ends up
# public with every file of DIR at its size.
set -euo pipefail
rolling=false prerelease=false latest=false
while [[ ${1-} == --* ]]; do
  case $1 in
    --rolling) rolling=true ;;
    --prerelease) prerelease=true ;;
    --latest) latest=true ;;
    *) echo "unknown option $1" >&2; exit 2 ;;
  esac
  shift
done
tag=$1 sha=$2 name=$3 notes=$4 dir=$5
api="repos/$GH_REPO"

if gh api "$api/git/ref/tags/$tag" > /dev/null 2>&1; then
  if $rolling; then
    gh api -X PATCH "$api/git/refs/tags/$tag" -f sha="$sha" -F force=true > /dev/null
  else
    at=$(gh api "$api/commits/refs/tags/$tag" --jq .sha)
    if [[ $at != "$sha" ]]; then
      echo "::error::$tag names $at, not $sha; a version tag is never moved"
      exit 1
    fi
  fi
else
  gh api -X POST "$api/git/refs" -f ref="refs/tags/$tag" -f sha="$sha" > /dev/null
fi

# Drafts included: the list is the only place they show.
ids=$(gh api --paginate "$api/releases" --jq ".[] | select(.tag_name == \"$tag\") | .id")
id=$(head -n1 <<<"$ids")
for extra in $(tail -n +2 <<<"$ids"); do
  gh api -X DELETE "$api/releases/$extra"
done
fields=(-f tag_name="$tag" -f target_commitish="$sha" -f name="$name"
        -F body=@"$notes" -F draft=false -F prerelease="$prerelease"
        -f make_latest="$latest")
if [[ -n $id ]]; then
  gh api -X PATCH "$api/releases/$id" "${fields[@]}" > /dev/null
else
  id=$(gh api -X POST "$api/releases" "${fields[@]}" --jq .id)
fi

cd "$dir"
files=() manifests=()
for file in *; do
  if [[ $file == katna-update*.json ]]; then manifests+=("$file"); else files+=("$file"); fi
done
gh release upload "$tag" --clobber -- "${files[@]}"
if ((${#manifests[@]})); then
  gh release upload "$tag" --clobber -- "${manifests[@]}"
fi
if $rolling; then
  gh api "$api/releases/$id/assets" --paginate --jq '.[] | "\(.id) \(.name)"' |
    while read -r asset file; do
      [[ -e "$file" ]] || gh api -X DELETE "$api/releases/assets/$asset"
    done
fi

release=$(gh api "$api/releases/$id")
if [[ $(jq -r .draft <<<"$release") != false ]]; then
  echo "::error::$tag is still a draft"
  exit 1
fi
uploaded=$(gh api --paginate "$api/releases/$id/assets" --jq '.[] | select(.state == "uploaded") | "\(.name) \(.size)"')
for file in *; do
  if ! grep -qxF "$file $(stat -c %s "$file")" <<<"$uploaded"; then
    echo "::error::$file is missing from $tag or has the wrong size"
    exit 1
  fi
done
echo "Published $tag: https://github.com/$GH_REPO/releases/tag/$tag"
