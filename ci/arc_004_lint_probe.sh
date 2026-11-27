#!/usr/bin/env bash
# ARC-004: clippy rejects each banned item in sim-core. Builds crates/sim-core/src/arc_004_probe.rs
# (cfg-gated) and expects one error per ban.
set -euo pipefail
cd "$(dirname "$0")/.."
out=$(CARGO_TARGET_DIR=target/arc_004_probe RUSTFLAGS="--cfg arc_004_probe" \
    cargo clippy --locked -p sim-core --message-format short -- -D warnings 2>&1 || true)
status=0
for expected in \
    'disallowed type `std::collections::HashMap`' \
    'disallowed type `std::collections::HashSet`' \
    'disallowed type `std::hash::RandomState`' \
    'disallowed method `std::time::SystemTime::now`' \
    'disallowed method `std::time::Instant::now`' \
    'usage of an `unsafe` block'; do
    if grep -qF "$expected" <<<"$out"; then echo "rejected: $expected"; else echo "NOT rejected: $expected" >&2; status=1; fi
done
[ $status -eq 0 ] && echo "ARC-004 ok" || { echo "$out" >&2; echo "ARC-004 FAIL" >&2; }
exit $status
