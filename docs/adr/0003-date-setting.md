# ADR-0003 — Default setting: Decisive Action Training Environment (DATE)

Status: Accepted · Date: 2026-11-22 · Closes: OQ-1

## Context
The game needs a setting on real Earth geography, with enough polities to be interesting and little real-world political baggage. It must also be possible to modify or replace the setting on the same map (SYS-007).

## Decision
- Use an **expanded DATE world** as the default setting. DATE is Distribution A (approved for public release) and a U.S. Government work in the public domain.
- Every distribution (README, game credits) carries this non-endorsement disclaimer:
  > Orchestra uses material adapted from the U.S. Army Decisive Action Training Environment (DATE), a public-domain U.S. Government work. Orchestra is not affiliated with, sponsored by, or endorsed by the U.S. Army, the Department of Defense, or the U.S. Government.
- The setting is content data only, under `content/settings/date/`. The map region graph is separate (`content/maps/earth/`).
- The 1955 back-history and canon are written in WS-08 and marked as Orchestra-original, not DATE.

## Consequences
- Adding, removing, or renaming polities, or swapping in a whole setting, needs only data files.
- Original lore must stay clearly separate from adapted DATE material, so provenance is traceable.
