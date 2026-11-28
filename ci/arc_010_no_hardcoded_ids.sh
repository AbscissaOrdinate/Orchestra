#!/usr/bin/env bash
# ARC-010: no content ID is hard-coded in code. Collects every quoted dotted ID
# ("polity.x", "region.y", "tech.z", ...) and every `id: "..."` value from all content
# (real and the sim-core test setting), then fails if any appears quoted in crates/*/src/.
set -euo pipefail
cd "$(dirname "$0")/.."

ids=$(
  {
    grep -rhoE '"[a-z][a-z0-9_]*\.[a-z0-9_.#]+"' content crates/sim-core/tests/content --include='*.ron'
    grep -rhoE '\bid: *"[^"]+"' content crates/sim-core/tests/content --include='*.ron' | sed -E 's/^id: *//'
  } | sort -u
)
count=$(wc -l <<<"$ids")
hits=$(grep -rnF -f <(printf '%s\n' "$ids") crates/*/src || true)
if [[ -n "$hits" ]]; then
  echo "ARC-010: content IDs hard-coded in crates/*/src:"
  echo "$hits"
  exit 1
fi
echo "ARC-010: none of $count content IDs appear in crates/*/src"
