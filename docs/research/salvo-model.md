# Salvo Combat Kernel: Research and Design Note

| Field | Value |
|---|---|
| Status | Draft v0.2. Framework and gap recommendations confirmed 2026-11-26, except F-5/G-5 (revised below, pending); T-4 decided |
| Feeds | WS-07 (combat kernel spec), WS-11 (unit stats and munitions) |
| Parent | ConOps §3 (Combat, Munitions rows), §5 P-4/P-9/P-10, §6 Combat row, SYS-009; WS-02 §3.3, ARC-024 |
| Date | 2026-11-26 |

Confidence tags follow `m1-findings.md`: **[V]** checked against the linked source, **[R]** recalled or inferred and not checked, **[D]** design recommendation.

---

## 1. Summary

- Armstrong's stochastic salvo model is the base, run as a **vectorized Monte Carlo over typed pools**. His closed forms are used only as a verification oracle and as the cheap forecast that AI and doctrine use on *beliefs*.
- One kernel covers ships, aircraft squadrons, missile/air-defense batteries, spacecraft, and fixed facilities, attacking each other in any legal combination. Legality and effectiveness come from content matrices, not code.
- Inventories are state. Magazines sit on the tracked unit (ship, battery, spacecraft). Aircraft draw a per-sortie loadout from base stock and return what they don't fire. Doctrine is a preference-and-threshold parameter set, so "out of AShM, switch to glide bombs" comes out of the rules with no special case.
- EW is slim: one `ew` rating per unit, one `eccm` rating per munition class, a countermeasure stock, and a salvo-level common-mode draw. That draw also fixes the correlation weakness Armstrong found in his own model.
- Neither Hughes nor Armstrong models magazine depth. Both assume every surviving unit fires its full salvo capacity. Doctrine is our addition (§7).

## 2. Literature verified

| Source | What it establishes | What we take |
|---|---|---|
| Hughes 1995, NRL 42(2) | **[V]** Deterministic salvo equations: offense minus defense, damage scaled by staying power. Later forms add scouting effectiveness σ and defensive readiness τ ([Tiah, NPS thesis](https://apps.dtic.mil/sti/pdfs/ADA467282.pdf)) | Parameter vocabulary; σ and τ map to detection and readiness multipliers |
| [Armstrong 2005, Op. Res. 53(5)](https://core.ac.uk/download/62653817.pdf) | **[V]** Offense per ship ~ Binomial(n, p) and interceptors ~ Binomial(n_z, p_z). Force totals and damage use normal approximations. Survivors are clipped to [0, B] with a v/2 continuity correction. Staying power can be random (v per hit). Fire is assumed spread evenly over targets. One simultaneous salvo. Each ship fires up to n per salvo; there is no magazine depth | Binomial structure; random survivors; even-spread baseline (replaced) |
| [Armstrong 2007, NRL 54(1)](https://core.ac.uk/download/62653813.pdf) | **[V]** Derives how many missiles to fire and at how many targets, using expected loss and P(eliminate) as measures. Attack effectiveness depends on the balance between a target's active defense and its staying power | The **Measured** doctrine sizes salvos with this logic, applied to the shooter's beliefs |
| [Armstrong 2011, Ann. OR 186](https://brocku.scholaris.ca/server/api/core/bitstreams/9b35d70f-992b-4e1a-832f-cbdbd911993b/content) | **[V]** Against Monte Carlo the normal approximations held, even at 2-missile salvos, and the shape of the damage distribution didn't matter. **Correlated** missiles were the weakness: with correlation, the loss-SD regression slope fell from 0.987 to 0.706. Target allocation and overkill were not tested | Deterministic lethality per hit is enough. Add a common-mode draw (§6). Allocation and overkill are ours to own |
| [Armstrong 2014, JORS 65(10)](https://core.ac.uk/download/62653809.pdf) | **[V]** Sequential fire: the first striker fires at full strength and only the defender's survivors return fire. On average, first > simultaneous > second, but not in every battle. Validated against an @Risk Monte Carlo (81 scenarios, 10,000 trials each); tail probabilities fit less well. Coral Sea: a US first strike with 2 carriers beat adding a third carrier | Round 1 is one-sided when a side wins the detection contest |
| [Armstrong 2013, NRL 60(8)](https://core.ac.uk/download/62649756.pdf) | **[V]** Area-fire salvo variants: mixed aimed/area fire shows a mix of square-law and linear-law behavior | A munition flag for area fire against soft or dispersed targets (deferred, §11) |
| [Armstrong 2014, Op. Res. 62(5)](https://econpapers.repec.org/RePEc:inm:oropre:v:62:y:2014:i:5:p:1028-1039) | **[V]** Short-range BMD (Iron Dome). Covers inaccurate missiles, failed intercepts, civil defense, attacks on the interceptor system, salvo-assembly difficulty, and imperfect threat classification. Performance is highly sensitive to salvo size, and higher-Pk systems are more fragile when overloaded | Channel-limited layers (saturation); engage-only-threatening doctrine; facilities as targets |
| [Armstrong & Powell 2005, MOR 10(4)](https://www3.carleton.ca/csds/docs/working_papers/ArmstrongWP03.pdf) | **[V]** Stochastic salvo model applied to carrier air strikes, with squadrons as the unit | Precedent for using one kernel for air and naval combat |
| [Atkinson & Kress, *Hard and Soft Defense Against a Sequence of Aerial Threats*](https://faculty.nps.edu/mkress/docs/salvo_paper_FINAL.pdf) (NPS manuscript, accepted to *Operations Research* Feb 2025) | **[V]** Interceptor inventory M with Pk p, shoot-look-shoot over K engagement opportunities, inventory truncation. Soft measures (jamming, decoys, DE) are modeled as unlimited with a per-round survival probability. Optimal policy: one interceptor per salvo when inventory is the binding limit; soft measures push interceptor fire later | Soft-kill layer placed ahead of hard kill; `shots_per_threat` doctrine; inventory truncation |

**Not found:** any Armstrong work on layered multi-type defense, heterogeneous forces, or space. Our extension is original and must be validated in the prototype (P-9).

## 3. Framework to confirm

| ID | Decision [D] | Confirm? |
|---|---|---|
| F-1 | One salvo kernel for naval, air, space, air/missile defense, and strikes on facilities. Ground maneuver stays in Lanchester (ConOps §6) | ☑ |
| F-2 | Truth comes from **sampling**: one stochastic realization per engagement, with vectorized binomial and multinomial draws over arrays (units × munition classes × layers). Armstrong closed forms are used only for (a) the verification oracle and (b) AI and doctrine forecasts on beliefs (P-2) | ☑ |
| F-3 | An engagement is a set of rounds (salvo pulses), default `R_max = 3` per engagement per tick. The engagement generator (who meets whom, range band, first strike) is a separate WS-07 function fed by WS-11 deployments and WS-08 adjacency and launch windows | ☑ |
| F-4 | Damage reduces the WS-02 strength fraction `s`. A unit is out of action at `s ≤ s_min` (content; default 0). Its offense and defense scale with `s` in the next round | ☑ |
| F-5 | Munitions are tracked as **~8–12 aggregate classes** (§8.4). Each holder keeps per-class **lots** with their own stats (§7.4), so new research changes only new deliveries, never stock already held | ☐ pending §7.4 choice |
| F-6 | Stock lives on the tracked unit for ships, batteries, and spacecraft. Air uses base stock with a per-sortie loadout | ☑ |
| F-7 | Doctrine is a preset parameter set per service, unlocked by research (ConOps §3: no player-authored doctrine) | ☑ |
| F-8 | EW is limited to the §6 terms | ☑ |

## 4. Modifications to Armstrong

| # | Armstrong baseline | Our change [D] | Why |
|---|---|---|---|
| M-1 | Homogeneous ships, one missile type | Typed pools: unit target class × munition class, with legality and Pk matrices | Cross-domain combat |
| M-2 | Pooled defense: total interceptions are subtracted from total offense | Ordered layers: soft kill → area hard kill (pooled over the group) → point hard kill (per unit). Each layer is limited by **channels** per round and by stock | Saturation and fragility (Armstrong 2014 BMD) |
| M-3 | Fire spread evenly | Multinomial allocation over legal targets, weighted by doctrine priority × signature | Armstrong never tested allocation; even spread is unrealistic across domains |
| M-4 | One salvo, no depletion | Multiple rounds; stock decrements; doctrine reserve | Inventory is a strategic lever (ConOps §3) |
| M-5 | Independent missiles | Beta-binomial common mode per (side, class, round), concentration κ. κ → ∞ recovers Armstrong | Fixes the 2011 correlation weakness; carries EW and environment |
| M-6 | Normal approximations | Exact binomial and multinomial draws | Cheap at our scale; correct tails |
| M-7 | Random loss per hit (v) | Deterministic `lethality / staying_power` per hit | Armstrong 2011: damage shape had no significant effect |
| M-8 | Simultaneous, or sequential as a separate model | Round 1 is one-sided if one side wins the detection contest; otherwise simultaneous | Armstrong 2014 first strike within one engagement |
| M-9 | Platforms are never engaged before firing | A launch platform whose munition's range band is ≤ the defender's area band passes through the area layer **as a threat** before release | Standoff vs. penetrating strikes; makes running out of AShM costly |

## 5. Methodology: one round

All quantities are arrays. `s` = strength, `ρ` = readiness × supply multiplier, `ε` = experience multiplier (WS-11). Symbols are defined in §8.

1. **Fire plan (doctrine).** For each shooter u and munition class w:
   `fire[u,w] = min( floor(rate[u,w] · s_u · ρ_u),  stock[u,w] − reserve_frac · cap[u,w],  want[u,w] )`.
   `want` is all available for Saturation, or the Armstrong-2007 salvo size on beliefs for Measured (§7). Aircraft: `rate` = ready aircraft × sorties × loadout (§7.2).
2. **Allocate.** Split `fire[u,w]` over legal targets t ~ Multinomial(π), where `π_t ∝ priority[class_t] · signature_t · legal[w, class_t]`.
3. **Aim.** `q = σ_side · p_rel[w] · ε_u`. Draw `q̃ ~ Beta(κq, κ(1−q))` once per (side, w, round), then `aimed ~ Binomial(fire, q̃)`. `σ_side` is the detection contest result (§6).
4. **Platform exposure (M-9).** For launch platforms that must enter the area band, the platforms themselves are threats to the area layers. They release only if they survive, and the stream shrinks in proportion.
5. **Soft kill.** Per target t: `p_sk = sk_max · logistic(ew_t − eccm_w)` while `cm_stock_t > 0`. Survivors ~ Binomial(m, 1 − p_sk). CM spent = engaged threats × `cm_per_threat`.
6. **Area layers (group-pooled, in order).** `cap_l = Σ_units channels[l] · s · ρ`. `engaged = min(eligible inbound, cap_l, floor(stock_l / spt))`. Engaged threats are spread over targets in proportion to their inbound counts (hypergeometric). Each engaged threat is killed with probability `1 − (1 − pk[l, threat_w])^spt`. Interceptors spent = `engaged · spt`. Anything not engaged leaks, which is the saturation effect.
7. **Point layers (per target unit).** Same rule with the unit's own channels and stock.
8. **Damage.** `Δs_t = min(s_t, Σ_w hits[t,w] · lethality[w, class_t] / staying_t)`. Hits beyond `s_t` are recorded as overkill.
9. **Update and terminate.** Decrement stocks, then recompute rate and channels from the new `s`. Stop at `R_max`, when either side's loss passes its doctrine `break_loss`, or when no legal offensive stock remains.

RNG: one ADR-0006 stream per (engagement id, round, purpose ∈ {alloc, aim, soft, area, point}). Draw order is fixed by sorted unit ID and class ID (P-4).

## 6. EW (slim)

| Term | Effect | Inventory |
|---|---|---|
| `ew` (unit rating) | Drives soft-kill probability vs. the incoming `eccm`; also reduces the enemy's σ in the detection contest | `cm` stock (chaff, flares, expendable decoys) gates soft kill |
| `eccm` (munition class) | Resists soft kill | — |
| Common mode κ (per engagement, content default per era) | Correlates all missiles in a salvo, because jamming or weather succeeds or fails together | — |
| Decoy munitions (optional class, `lethality = 0`) | Absorb channels and interceptors unless discriminated (`p_discriminate` from defender `ew`) | Consumed like any munition |

Detection contest: `σ_side = clamp(sensor_best − signature_target_mean − ew_enemy_mean, σ_min, 1)` on a logistic scale. The side with the higher margin by more than `first_strike_margin` fires round 1 alone. This keeps EW at the modifier level (ConOps §3) while covering soft kill, deception, and saturation by cheap decoys.

Excluded: ESM/SIGINT, emission control (beyond a posture flag that lowers signature and sensor together), cyber, and DEW beyond "a layer with `consumes = false`".

## 7. Inventory and doctrine

### 7.1 Answer: do the models fire whole magazines?

**No magazine exists in either model.** Hughes's deterministic model delivers all offensive power each salvo. Armstrong caps each ship at n missiles per salvo, and the number delivered is random through p. His sequential variant implicitly assumes each survivor fires a full salvo, with one salvo per side and no depletion **[V: 2005, 2014 papers]**. Salvo size as a *choice* appears only in Armstrong 2007 (how many to fire at how many targets) and in Atkinson & Kress (interceptor firing policy under finite inventory). So different doctrines are not built in, but the structure supports them cleanly: doctrine sets `want`, `reserve_frac`, `spt`, and priorities, and the kernel never changes.

### 7.2 Holding model

| Holder | Stock | Refill |
|---|---|---|
| Ship, battery, spacecraft | `stock[u,w]` ≤ `cap[u,w]` (unit magazine) | From base/depot stock when in port or at depot, `reload_per_tick` (WS-11). Ships at sea only with an UNREP capability flag. Spacecraft never, unless WS-08 logistics allows |
| Air squadron | None at rest. Per sortie it carries `loadout[k]` drawn from the **base** stock | Unexpended rounds return to base stock after the engagement. Turnaround is in `sortie_rate` |
| Base / depot (WS-11 facility) | `stock[b,w]` | WS-05 production deliveries; transfers along supply edges |

Weapon selection: each unit class's doctrine has a preference list per mission, for example `strike_ship: [AShM, glide_bomb, gun]`. The first class with stock > 0, a legal target, and `range_band ≥ required` is used. When AShM runs out, glide bombs are chosen, and because their range band falls below the defender's area band, M-9 exposes the aircraft. That is the intended cost.

### 7.4 Munition lots (G-5)

Requirement (Sean): researching a new design must not change stock already built. Options:

| Option | State per holder | Fidelity | Cost |
|---|---|---|---|
| A. Track every design | One count per design per class | Exact | Unbounded over 105 years; breaks the ConOps munitions scope |
| B. One blended lot per class (Sean's proposal) | Count + count-weighted stats per class | Exact for the mean where stats enter linearly (`p_rel`, `lethality`); small bias where they don't (`eccm` inside the logistic, Pk via threat class) | 1× |
| **C. B, plus a categorical split (recommended)** | Up to **2 lots** per class: blend only lots whose categorical fields match (`threat_class`, `range_band`, `legal_targets`); a third lot merges into its nearest match by those fields | Exact for categorical changes (a subsonic AShM and a hypersonic one never average into a fake "transonic" missile); B's small bias otherwise | ≤ 2× B; still a few classes |

Rules for C [D]:
- Deliveries from WS-05 production join the holder's matching lot by count-weighted average, or open the second lot.
- Blended fields: `p_rel`, `lethality`, `eccm`. Categorical fields are never blended.
- Lot choice when firing is a doctrine field: `best_first` (default), or `oldest_first` for Conserve, which burns obsolete stock in low-threat engagements.
- Transfers between holders move lots with their stats, so a base's mixed stock goes onto a ship as it is.
- Refit/modernization (WS-11) may convert a lot at a cost. It never happens automatically.

### 7.3 Doctrine parameters (preset per service, research-unlocked)

| Parameter | Saturation | Measured | Conserve |
|---|---|---|---|
| `want` rule | All available | Smallest salvo with forecast P(kill) ≥ `p_target`, using Armstrong closed forms on the shooter's belief of the target (P-2) | Measured with `p_target` lowered |
| `reserve_frac` | 0 | 0.25 | 0.5 |
| `spt` (shots per threat) | 2 | 1–2 by threat class | 1 |
| `engage_threshold` | Engage all | Engage predicted-threatening only (Iron Dome classification) | Same |
| `break_loss` | 0.6 | 0.4 | 0.25 |
| Target `priority` | By class weights | Same | Same |

Values are placeholders for prototype tuning. Measured couples belief error to real outcomes: a polity that underestimates enemy defenses fires too little.

## 8. Per-unit inputs WS-11 must provide

This is the contract. WS-11's rollup produces these from templates and designs; WS-07 reads nothing else (P-10, SYS-009).

### 8.1 Unit aggregates (every tracked unit and facility)

| Field | Type | Meaning |
|---|---|---|
| `target_class` | enum: `surface`, `subsurface`, `air`, `ground_mobile`, `ground_fixed`, `facility`, `space` (+ `orbit_regime`) | Legality and Pk lookups |
| `signature_fired` | f64 multiplier | Signature after the unit fires (a submarine's datum, a battery's launch plume); applies from the next round |
| `s` | f64 (0, 1] | Strength fraction (WS-02) |
| `staying_power` | f64, standard hits at s = 1 | Hits of lethality 1.0 that put the unit out of action |
| `signature` | f64 ≥ 0 | Allocation weight and detection penalty |
| `sensor` | f64 | Best detection and targeting quality (feeds σ) |
| `ew` | f64 | Soft-kill and anti-scouting rating |
| `cm_stock`, `cm_cap`, `cm_per_threat` | u32, u32, f64 | Countermeasure inventory |
| `readiness`, `supply`, `experience` | f64 multipliers | ρ = readiness × supply; ε = experience |
| `mounts` | list of §8.2 | Offensive weapons |
| `layers` | ordered list of §8.3 | Defensive layers |
| `reload` | {`at_base`, `unrep`, `none`}, `reload_per_tick` | Refill rule |
| `doctrine` | preset ID | §7.3 |

### 8.2 Mount (offense)

| Field | Meaning |
|---|---|
| `munition_class` | §8.4 ID |
| `rate` | Max rounds per salvo round at s = 1 (VLS cycle rate, launcher count, hardpoints × aircraft) |
| `cap`, `stock` | Magazine (absent for aircraft) |
| Aircraft only: `aircraft`, `sortie_rate`, `loadouts` | Ready airframes, sorties per engagement, and loadout presets: per-sortie counts by class plus carried CM |

### 8.3 Defense layer

| Field | Meaning |
|---|---|
| `kind` | `soft`, `area`, `point` |
| `munition_class` or `consumes = false` | Interceptor class, or guns/DEW (channel-limited only) |
| `channels` | Threats engaged per round at s = 1 |
| `range_band` | Ordinal: `point` < `area` < `extended` < `theater` |
| `stock` / `cap` | When `consumes` is true |

### 8.4 Munition class (content; stats from the polity's current WS-05 design)

| Field | Meaning |
|---|---|
| `id`, `role` | offensive / interceptor / both / decoy |
| `legal_targets` | Set of target classes (interceptors also list threat classes) |
| `threat_class` | When in flight: `subsonic`, `supersonic`, `ballistic`, `hypersonic_glide`, `aircraft`, `orbital_kv`, `torpedo` |
| `p_rel` | Launch and guidance reliability (Armstrong p) |
| `lethality[target_class]` | Standard hits per impact |
| `eccm`, `range_band`, `area_fire` (deferred) | — |

Indicative classes (F-5): AShM, LACM, ballistic SRBM/MRBM, HGV, AAM, area SAM/ABM, point SAM/CIWS, ASAT (direct-ascent), co-orbital KV, PGM/glide bomb, heavyweight torpedo, lightweight torpedo/ASW weapon, decoy.

### 8.5 Subsurface (submarines and torpedoes)

The kernel applies without new mechanics [D]:
- `subsurface` is a target class. Only ASW-legal classes can engage it (lightweight torpedoes from ships, helicopters and MPA; heavyweight torpedoes from submarines).
- Torpedoes are a `torpedo` threat class. Ship defense against them is mostly the **soft layer** (towed decoys, consuming `cm`) with few hard-kill layers. Lethality against surface targets is high.
- A submarine's low `signature` dominates the detection contest, so it usually gets round 1 alone (M-8). `signature_fired` then exposes it to counterattack in later rounds.
- A submarine firing AShM or LACM is an ordinary mount. Ballistic-missile submarines stay in WS-11 strategic forces, outside the kernel.

The real limit sits outside the kernel **[R]**: ASW outcomes are dominated by search and detection over time (classical search theory), not by salvo exchange. The WS-07 engagement generator (G-1) must produce submarine contacts at a low per-tick probability driven by ASW sensor coverage versus signature. Otherwise submarines will fight as often as surface ships, and too visibly.

### 8.6 Content matrices (WS-07 owns)

- `pk[interceptor_class, threat_class]`
- `legal[munition_class, target_class]`
- Era defaults for κ, `sk_max`, `σ_min`, `first_strike_margin`

**Scale guidance [R, unverified]:** Hughes related ship staying power to displacement in his 1995 analysis. WS-11 should check that source before deriving `staying_power` from hull size.

## 9. Outputs (WS-07 writes `engagements` only; ARC-024)

Per engagement: participants; rounds run; first-strike side. Per unit: Δs, out-of-action flag, munitions spent by class, CM spent, hits received by (attacker unit, class), overkill, and leakers per layer. WS-11 applies these in R6b. Breakdowns are `Lazy` and rebuilt by capture replay.

## 10. Verification and performance

| Check | Criterion |
|---|---|
| Armstrong oracle | Degenerate config (one class, one pooled layer, κ = ∞, no stock limit, simultaneous, even spread): MC survivor mean, SD, and P(wipe) match a per-integer continuity-corrected normal. **[D: computed 2026-11-26]** A = 4 × 4 missiles, p = 0.7; B = 3 × 3 interceptors, p_z = 0.6; staying power 2; 200k trials. MC 0.512 / 0.723 / 0.556 vs. closed form 0.514 / 0.720 / 0.551. A naive whole-distribution v/2 shift gave mean 0.396, so the oracle must apply the correction per integer |
| First-strike ordering | Mean loss: first ≤ simultaneous ≤ second (Armstrong 2014) |
| Saturation | Leakers vs. salvo size are flat below channel capacity and rise steeply above it (Armstrong 2014 BMD) |
| Correlation | Finite κ raises loss SD at equal mean |
| Conservation | Stock ≥ 0; expended equals the stock delta; returned aircraft rounds equal loadout minus fired |
| Doctrine | With AShM at 0, a strike uses glide bombs and incurs area-layer exposure |
| Determinism | ARC-060 double-run hash |

**Performance [D]:** a wartime tick of about 50 engagements × 3 rounds × (~30 units × ~10 classes + layers) is about 10⁵ draws. At ~50–100 ns each that is 5–10 ms, the whole R6 budget (WS-02 §5). Mitigation: skip zero cells, and use the sparse (unit, class) pairs that are actually present. Measure in the prototype.

## 11. Gaps: address or accept

| # | Gap | Recommendation |
|---|---|---|
| G-1 | Time, geometry, and range beyond ordinal bands | **Accept** in the kernel; the WS-07 engagement generator handles it |
| G-2 | Detection contest is crude | **Address thin** (§6); belief-driven σ is a later WS-06 refinement |
| G-3 | Target allocation realism | **Accept** multinomial-by-priority; tune in the prototype |
| G-4 | BDA between rounds | **Accept** perfect BDA in MVP; imperfect BDA (Atkinson & Kress) deferred |
| G-5 | Munition vintages within a class | **Sean:** new designs must not upgrade existing stock. Options and recommendation in §7.4 (pending) |
| G-6 | Salvo hits on ground brigades → Lanchester strength | **Address** in WS-07: hits convert to the Lanchester strength loss via `staying_power` |
| G-7 | Space: debris, regimes, co-orbital transit time, maneuver as defense | **Accept thin:** maneuver = soft layer spending a Δv stock; debris = a WS-08 hazard increment per kill. Flag for WS-08 |
| G-8 | Strikes on facilities and the economy (bases, infrastructure, cities) | **Address:** `facility` target class whose damage feeds WS-11 capacity and WS-03 capital |
| G-9 | Area fire against dispersed targets | **Defer** (Armstrong 2013 variant) unless G-6 needs it |
| G-10 | Nuclear use | **Out** (ConOps) |
| G-11 | Calibration data | **Accept:** tune in the prototype against Falklands, Coral Sea, Moskva, and Red Sea style cases |
| G-12 | R6 performance margin | **Address:** prototype benchmark before the spec is approved |

## 12. Tensions with approved documents (proposed edits, not made)

- **T-1 Munitions (ConOps §3):** "a few aggregate classes" holds if F-5 caps the list at about 12 classes with class-level stats. "Per-munition-type tracking" stays deferred. Proposed edit: add "held per unit or base".
- **T-2 P-10:** offense and defense become small per-class structures (§8.2–8.3), not scalars. Proposed edit: "offense and defense as per-munition-class mounts and layers".
- **T-3 EW:** §6 is meant to stay at modifier level. Confirm that it does.
- **T-4 Batteries (§6.1): decided 2026-11-26.** Missile and air-defense units are **brigade templates** (R-6 holds). A brigade with `mounts`/`layers` takes part in the salvo kernel; its maneuver elements stay in Lanchester (G-6).

## Sources
Listed inline in §2. Additional: [Wikipedia: Salvo combat model](https://en.wikipedia.org/wiki/Salvo_combat_model) (bibliography cross-check).
