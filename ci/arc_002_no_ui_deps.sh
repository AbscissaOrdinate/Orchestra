#!/usr/bin/env bash
# ARC-002: sim-core has no UI, async-runtime or I/O-framework dependency.
set -euo pipefail
cd "$(dirname "$0")/.."
tree=$(cargo tree --locked -p sim-core -e normal --prefix none)
if grep -E '^(tauri|wry|tokio|serde_json|clap)\b' <<<"$tree"; then
    echo "ARC-002 FAIL: banned crate(s) above are in sim-core's dependency tree" >&2
    exit 1
fi
echo "ARC-002 ok"
