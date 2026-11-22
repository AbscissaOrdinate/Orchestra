# Workstream specs

One file per workstream: `WS-NN-short-name.md`. Each spec traces up to the ConOps (`SYS-` IDs, principles `P-n`) and down to tests.

## Requirement ID prefixes
`SYS` system (ConOps) · `ARC` architecture · `ECO` economy · `GOV` governance · `RND` R&D/design/defense industry · `DIP` diplomacy/intel/AI · `CBT` combat · `WLD` world/space/setting · `UI` interface · `VV` verification

## Template

```markdown
# WS-NN — <Name>
Status: Draft | Approved vX.Y · Parent: ConOps §<n>, SYS-<ids> · Last updated: YYYY-MM-DD

## 1. Purpose & scope
What this subsystem does in the MVP, and what it explicitly does not.

## 2. Interfaces
Inputs consumed from, and outputs exposed to, other subsystems (state variables, events, API).

## 3. Model
Equations, state variables, parameters (with default values and sources), stochastic elements, and their distributions.

## 4. Requirements
| ID | Requirement | Acceptance criterion (testable) | Trace |
|---|---|---|---|
| ECO-001 | ... | Given X, when Y, then Z within tolerance T | SYS-004, P-3 |

## 5. Content/data
The data files this subsystem reads, with their schemas.

## 6. Verification
Unit tests, batch Monte Carlo checks, and stability bounds.

## 7. Open questions
```
