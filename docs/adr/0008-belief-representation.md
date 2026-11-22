# ADR-0008 — Belief representation: small Gaussian mixture with a (mean, σ) facade

Status: Proposed · Date: 2026-11-22 · Workstream: WS-02 · Supports: P-1, P-2, SYS-005

## Context
P-2 requires one belief mechanic for tech uncertainty, intel, treaty verification, prospecting and AI reasoning. WS-05 needs multimodal outcomes ("works well or dead end", ConOps §6). Order of magnitude [D]: 45 polities × ~1,000 subjects ≈ 45,000 beliefs.

## Options
| Option | For | Against |
|---|---|---|
| Gaussian (mean, σ) | Simplest; closed-form updates; 16 B | Can't represent bimodal outcomes |
| **Gaussian mixture, ≤ 3 components, on a linear/log/logit scale** | Captures multimodality; closed-form update per component plus likelihood reweighting; inline, `Copy`, ~80 B; reduces to (mean, σ) by moment matching | Moment-matched σ of a bimodal belief can mislead; consumers that care must read components |
| Particle set | Fully general | ~100× memory; sampling noise; needs resampling logic |
| Categorical over discrete outcomes | Good for yes/no facts | Doesn't fit continuous parameters; a second mechanic |

## Decision
`Belief { scale, n, comps: [Component; 3] }`. `observe(value, noise_σ)` does an exact Bayesian update. `mean()`/`sigma()` return moment-matched values in natural units. Positive quantities use `Log`, probabilities `Logit`. All updates go through `Observation`s applied in stage I1 (ADR-0005).

## Consequences
- Binary facts (e.g., "does polity X violate clause Y") are represented as a probability on the logit scale, one mechanic.
- Hidden subjects are `None` in the polity's belief vector until discovered.
- If 3 components prove too few, raising the cap is a save-format version bump, not a redesign.
