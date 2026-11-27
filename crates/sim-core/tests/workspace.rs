//! M4a workspace checks (WS-02 §3.1, ARC-001/003/004). The CI scripts in `ci/` cover the
//! `cargo tree` and clippy parts of ARC-002–004.

const ROOT_MANIFEST: &str = include_str!("../../../Cargo.toml");
const APP_MANIFEST: &str = include_str!("../../../app/src-tauri/Cargo.toml");
const TOOLCHAIN: &str = include_str!("../../../rust-toolchain.toml");
const CORE_MANIFEST: &str = include_str!("../Cargo.toml");
const LOCKFILE: &str = include_str!("../../../Cargo.lock");
const CLIPPY: &str = include_str!("../../../clippy.toml");
const CORE_LIB: &str = include_str!("../src/lib.rs");

/// The value of the first `key = value` line in a TOML file, as written.
fn value<'a>(toml: &'a str, key: &str) -> &'a str {
    toml.lines()
        .find_map(|line| line.strip_prefix(key)?.trim_start().strip_prefix('='))
        .unwrap_or_else(|| panic!("`{key}` not found"))
        .trim()
}

#[test]
fn arc_001_workspace_layout() {
    assert_eq!(
        value(ROOT_MANIFEST, "members"),
        r#"["crates/sim-core", "crates/cli", "app/src-tauri"]"#
    );
    assert_eq!(
        value(ROOT_MANIFEST, "default-members"),
        r#"["crates/sim-core", "crates/cli"]"#
    );
    assert_eq!(value(ROOT_MANIFEST, "edition"), r#""2024""#);
    assert_eq!(value(APP_MANIFEST, "name"), r#""orchestra-app""#);
}

#[test]
fn arc_001_toolchain_pinned_stable_release() {
    let channel = value(TOOLCHAIN, "channel").trim_matches('"');
    let parts: Vec<&str> = channel.split('.').collect();
    assert!(
        parts.len() == 3 && parts.iter().all(|p| p.parse::<u32>().is_ok()),
        "channel must be an exact stable release such as 1.100.1, got {channel}"
    );
}

#[test]
fn arc_003_rng_pins_exact() {
    for (name, version) in [
        ("rand_core", "0.10.1"),
        ("rand_chacha", "0.10.0"),
        ("rand", "0.10.4"),
        ("rand_distr", "0.6.0"),
    ] {
        let pin = format!(r#""={version}""#);
        let requirement = value(CORE_MANIFEST, &format!("{name} "));
        assert!(
            requirement.starts_with(&pin) || requirement.contains(&format!("version = {pin}")),
            "{name} must be pinned with ={version} in sim-core, got {requirement}"
        );
        let locked = format!("name = \"{name}\"\nversion = \"{version}\"\n");
        assert!(
            LOCKFILE.contains(&locked),
            "Cargo.lock must lock {name} {version}"
        );
    }
}

#[test]
fn arc_004_clippy_bans_listed() {
    for path in [
        "std::collections::HashMap",
        "std::collections::HashSet",
        "std::time::SystemTime",
        "std::time::Instant",
        "std::time::SystemTime::now",
        "std::time::Instant::now",
        "rand::rngs::ThreadRng",
        "rand::rngs::SysRng",
        "rand::rng",
        "rand::random",
    ] {
        assert!(
            CLIPPY.contains(&format!(r#"path = "{path}""#)),
            "clippy.toml must ban {path}"
        );
    }
}

#[test]
fn arc_004_forbid_unsafe() {
    assert!(
        CORE_LIB
            .lines()
            .any(|l| l.trim() == "#![forbid(unsafe_code)]")
    );
}

#[test]
fn arc_004_os_rng_not_compiled_in() {
    // rand's thread_rng / sys_rng features pull in getrandom; sim-core keeps them off.
    let rand_line = value(CORE_MANIFEST, "rand ");
    assert!(
        rand_line.contains("default-features = false"),
        "{rand_line}"
    );
    assert!(
        !rand_line.contains("thread_rng") && !rand_line.contains("sys_rng"),
        "{rand_line}"
    );
}
