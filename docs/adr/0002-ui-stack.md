# ADR-0002 — UI stack

Status: Accepted · Date: 2026-11-22 · Closes: OQ-5

## Context
The UI is text-first, with SVG vector graphics, MIL-STD-2525 symbology, and ASCII fallback (ConOps §3). The simulation core is headless Rust (P-5).

## Options
| Option | For | Against |
|---|---|---|
| **Tauri (Rust + webview, HTML/CSS/JS)** | HTML/CSS is best at dense text, tables, and tooltips (needed for P-3 breakdowns); SVG is native; a JS 2525 library (e.g., `milsymbol`) can be reused; already installed on the dev machine | Two languages; IPC boundary |
| egui / iced (pure Rust) | One language; simple build | Weaker rich-text and table layout; would need a 2525 implementation of our own |

## Decision
**Tauri.** `crates/sim-core` stays UI-agnostic. `app/` talks to it through Tauri commands that return serializable state snapshots and breakdowns.

## Consequences
- Keep the IPC surface coarse: one snapshot per tick plus breakdown queries made on demand.
- The `milsymbol` license and the 2525 revision it supports still need to be verified (M1 research).
- The front end needs no framework at first. Use plain JS and SVG until complexity forces a choice (lazy-first).
