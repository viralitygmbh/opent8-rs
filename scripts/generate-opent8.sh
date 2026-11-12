#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

python3 scripts/sync-schemas.py

mode=${1:-}
if [[ $# -gt 1 || ( -n "$mode" && "$mode" != "--check" ) ]]; then
  echo "Usage: $0 [--check]" >&2
  exit 2
fi

# cargo install cargo-typify --version 0.8.0 --locked
[[ "$(cargo typify --version)" == "cargo-typify 0.8.0" ]] || {
  echo "Install cargo-typify 0.8.0 (see script)" >&2
  exit 1
}
temporary=$(mktemp -d)
trap 'rm -rf "$temporary"' EXIT
schema="$temporary/schema.json"
generated="$temporary/generated.rs"
# Single-value enums preserve const semantics;
# the Rust type annotation preserves wall-clock times and offsets on serialization.
# Date/date-time formats are disjoint. Typify needs oneOf to generate their enum
# instead of an anyOf struct that incorrectly flattens scalar values.
for version in 0.7 0.8; do
  target="src/v${version//./_}/generated.rs"
  jq 'walk(if type == "object" then
  (if has("anyOf") and ([.anyOf[] | .format] | sort) == ["date", "date-time"]
    then . + {"oneOf": .anyOf} | del(.anyOf) else . end) |
  (if has("const") then . + {"enum": [.const], "title": (.const + "Kind")} | del(.const) else . end) |
  (if .format? == "date-time" then . + {"x-rust-type": {
    "crate": "chrono", "version": "0.4",
    "path": "chrono::DateTime<chrono::FixedOffset>"
  }} else . end)
  else . end)' "schema/v$version/schema.json" > "$schema"
  cargo typify "$schema" --map-type std::collections::BTreeMap \
    --crate chrono@0.4.42 -o "$generated"
  { cat <<'HEADER'
// SPDX-License-Identifier: Apache-2.0
// Generated from OpenPotato / STÜBER SYSTEMS OpenT8 schemas; see schema/sources.json.
// Adapted with cargo-typify 0.8.0: const enums, date unions, fixed-offset timestamps.
// Do not edit; regenerate with scripts/generate-opent8.sh.
HEADER
    cat "$generated"
  } > "$temporary/header.rs"
  mv "$temporary/header.rs" "$generated"
  rustfmt --edition 2024 "$generated"

  if [[ "$mode" == "--check" ]]; then
    if ! diff -u "$target" "$generated"; then
      echo "OpenT8 $version types are stale. Run: bash scripts/generate-opent8.sh" >&2
      exit 1
    fi
    echo "OpenT8 $version generated types are up to date."
  else
    cp "$generated" "$target"
  fi
done
