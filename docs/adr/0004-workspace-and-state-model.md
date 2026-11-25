# ADR-0004 — Workspace layout and state model (no ECS)

Status: Accepted · Date: 2026-11-22 · Workstream: WS-02 · Confirms: ConOps §7

## Context
ConOps §7 lists a provisional layout (`sim-core`, `cli`, `app/`, `content/`, `prototypes/`) to be confirmed in WS-02. The state model has to be deterministic (P-4), hashable and serializable as one value (SYS-002), and small enough for a solo developer to keep in his head. The MVP holds ~45 polities, ~100 regions, ~50 bodies and a few hundred techs [D].

## Options
| Option | For | Against |
|---|---|---|
| **Plain structs of `Vec`s indexed by typed IDs** | Zero dependencies; one `#[derive(Serialize)]` gives saves, export and the hash; the borrow checker enforces read-old/write-new (ADR-0005); easy to read | No automatic parallel scheduling; adding a field to `World` is a code change (acceptable: each WS adds one block) |
| ECS (`bevy_ecs` 0.20.1 [V: crates.io 2026-11-22], headless) | Fast iteration over huge entity counts; system scheduler; change detection | Entity counts here are 3–4 orders of magnitude too small to benefit; parallel scheduler needs care to stay deterministic; frequent breaking releases [R]; world-as-one-value serialization is awkward; large compile-time cost |
| Generic key-value store (`VarKey → f64`) | Uniform explain API | Loses static typing; slow lookups; every model becomes stringly typed |

## Decision
**Plain structs.** `World` owns one `Vec` of blocks per subsystem, indexed by `PolityId` (or `RegionId`/`BodyId`). Collections are `Vec` or `BTreeMap` only. The workspace is `crates/sim-core`, `crates/cli`, `app/src-tauri` (non-default member), with content under `content/{core,maps,settings,scenarios}`. sim-core dependencies are limited to serde, ron, the rand family, postcard and blake3 (spec WS-02 §3.1).

## Consequences
- Each WS adds its block type and one `World` field. That's a reviewed code change, which is the point.
- If profiling ever shows iteration cost matters, blocks can be reorganized into struct-of-arrays without changing the stage interface.
- ConOps §7 can drop "provisional" and cite this ADR (proposed edit, OQ-A5).
