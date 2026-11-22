# ADR-0001 — Content data format

Status: Accepted · Date: 2026-11-22 · Closes: OQ-2

## Context
Content (techs, components, hulls, government parameters, polities, scenarios) is data-driven (ConOps P-6). The format needs to deserialize straight into Rust types, stay compact and readable for both people and LLM sessions, and support comments. YAML and RON were both proposed.

## Options
| Option | For | Against |
|---|---|---|
| **RON** | Built for Rust's serde data model: native enums, structs, tuples and `Option`; comments; trailing commas; strict, so there are no implicit type conversions | Smaller ecosystem; few non-Rust parsers |
| YAML | Widely known; good tooling; easy for Python prototypes to read | Implicit typing pitfalls (e.g., `NO` → false); Rust enums map to it awkwardly; my recollection, not yet verified, is that `serde_yaml` is archived and unmaintained, leaving forks of uneven quality |
| Both | Flexibility | Two parsers and two sets of conventions; schema drift |

## Decision
**RON for all content consumed by the simulation. Markdown for prose** (lore, event text, docs). Use one format, not both.

## Consequences
- Rust types are the schema. `sim-core` validates content at load time and fails loudly.
- Python prototypes (M2) keep their parameters inside the scripts. Once M4 exists, the CLI can export content to JSON for any external tooling, so there's no need for a Python RON parser.
- If YAML is ever needed for modders, add an import path later. Don't add a second format now.
