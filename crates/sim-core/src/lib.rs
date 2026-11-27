//! Orchestra simulation core: headless, deterministic (WS-02).
#![forbid(unsafe_code)]

#[cfg(arc_004_probe)]
mod arc_004_probe;

/// sim-core version, shown by the Tauri stub (ARC-100).
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Hash of a new world for `seed`, shown by the Tauri stub (ARC-100).
///
/// Placeholder until M4e/M4f: blake3 of the seed's little-endian bytes. It becomes
/// `state_hash` of the tick-0 world so that it matches `orchestra run --ticks 0 --seed <seed>`.
pub fn new_world_hash(seed: u64) -> String {
    blake3::hash(&seed.to_le_bytes()).to_hex().to_string()
}
