//! ARC-004 lint probe. Compiled only with `--cfg arc_004_probe` by `ci/arc_004_lint_probe.sh`,
//! which expects clippy to reject every banned item used here.
#![allow(dead_code, unused_unsafe)]

fn banned() {
    let _ = std::collections::HashMap::<u8, u8>::new();
    let _ = std::collections::HashSet::<u8>::new();
    let _ = std::hash::RandomState::new();
    let _ = std::time::SystemTime::now();
    let _ = std::time::Instant::now();
    unsafe {}
}
