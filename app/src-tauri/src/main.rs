//! Tauri 2 hello-world (WS-02 §3.10, ARC-100): two commands that call sim-core.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn sim_version() -> String {
    sim_core::version().to_string()
}

#[tauri::command]
fn new_world_hash(seed: u64) -> String {
    sim_core::new_world_hash(seed)
}

// The ARC-004 bans in the root clippy.toml are for sim-core; tauri's generated context uses HashMap.
#[allow(clippy::disallowed_types)]
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sim_version, new_world_hash])
        .run(tauri::generate_context!())
        .expect("error while running the Orchestra app");
}
