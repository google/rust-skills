#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 DESTINATION" >&2
  exit 2
fi

bundle_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
destination="$1"

if [[ -z "$destination" || "$destination" == "/" ]]; then
  echo "refusing unsafe destination: ${destination:-<empty>}" >&2
  exit 2
fi

mkdir -p -- "$(dirname -- "$destination")"
destination_parent="$(
  cd -- "$(dirname -- "$destination")" &&
    pwd -P
)"
destination_abs="${destination_parent}/$(basename -- "$destination")"

case "${destination_abs}/" in
  "${bundle_root}/"|"${bundle_root}/"*)
    echo "destination must be outside the source bundle" >&2
    exit 2
    ;;
esac

rm -rf -- "$destination_abs"
mkdir -p -- "$destination_abs"

for path in SKILL.md agents assets references; do
  cp -a -- "$bundle_root/$path" "$destination_abs/"
done

for excluded in README.md UPSTREAM.md evals maintainers; do
  test ! -e "$destination_abs/$excluded"
done
