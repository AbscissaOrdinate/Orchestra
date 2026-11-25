# ADR-0009 — Save format and state hash: postcard + blake3

Status: Proposed · Date: 2026-11-22 (revised 2026-11-25) · Workstream: WS-02 · Supports: SYS-002

## Context
SYS-002 requires byte-identical state hashes for identical seed and inputs. Saves need to be compact, fast and bit-exact for `f64`. Content is RON (ADR-0001), but saves have no requirement to be hand-edited.

## Options
| Option | For | Against |
|---|---|---|
| RON | Same format as content; readable | Large and slow for multi-MB states; float text formatting adds a failure mode for the hash |
| bincode | Familiar | 3.0.0 is a tombstone (`compile_error!`) [V: build attempt 2026-11-22]; 2.0.1 is final |
| **postcard 1.2** | Maintained [V: crates.io, 1.2.0 released 2026-11-21]; compact; serde; `f64` as exact little-endian bytes [V: measured] | Not self-describing: type changes break old saves |
| JSON | Universal | Large; float formatting; reserved for export |

## Decision
Saves are `postcard(SaveHeader, World, InputLog)`. `SaveHeader` carries magic `ORCH`, `SAVE_FORMAT_VERSION`, sim-core version, content hash, seed, tick and platform (OS/arch). A save from another platform loads; replay reports any divergence as a platform difference (OQ-A1). `state_hash = blake3(postcard(World))`. JSON (`serde_json`, CLI only) is for export. No migration in MVP: mismatched versions are rejected. A committed fixture save catches type changes made without a version bump.

## Consequences
- Every change to a serialized type needs a version bump and a new fixture; old saves stop loading. That's acceptable before release.
- Post-MVP migration can be added by keeping old type definitions behind the version number.
