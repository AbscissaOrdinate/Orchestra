#!/usr/bin/env bash
# ARC-003: sim-core's direct dependencies are exactly ci/sim-core-deps.allow (WS-02 §3.1).
set -euo pipefail
cd "$(dirname "$0")/.."
actual=$(cargo tree --locked -p sim-core --depth 1 -e normal --prefix none | tail -n +2 | sort)
if ! diff <(grep -v '^#' ci/sim-core-deps.allow | sort) <(echo "$actual"); then
    echo "ARC-003 FAIL: sim-core direct dependencies differ from ci/sim-core-deps.allow (> = not allowed)" >&2
    exit 1
fi
echo "ARC-003 ok"
