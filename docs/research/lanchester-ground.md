# Ground Combat Kernel (Lanchester): Research and Design Note

| Field | Value |
|---|---|
| Status | v0.1 research note. Foundational for WS-07, which hasn't started; nothing here is approved. Open questions OQ-L1–L5 (§10) |
| Feeds | WS-07 (combat kernel spec, later), WS-11 (`GroundStats`, proposal P-11-L1) |
| Parent | ConOps §3 Combat row, §6 Combat row; `salvo-model.md` G-6, G-9, T-3, T-4, M-3, M-5; WS-11 Draft v0.1 §3.5 `UnitCombatView`, §4.3 rollup |
| Date | 2026-11-28 |

Confidence tags follow `m1-findings.md`: **[V]** checked against the linked source, **[R]** recalled or inferred and not checked, **[D]** design recommendation.

---

## 1. Summary

- Ground maneuver combat stays in Lanchester (ConOps §6, salvo F-1). The base law covers neither fires, air support, EW, frontage nor force mix. Each has a published modification, and each fits as a **multiplier or an extra term**, not a new model (§3).
- **Hard/soft attack should be vectorized**, using the same content-matrix pattern as the salvo kernel. A lethality matrix `K[weapon_class, target_class]` and a per-unit composition vector `c[t]` replace scalar firepower and protection. HOI4-style hard/soft is the special case W = T = 2 with K = I (§5).
- **One lethality matrix serves both kernels.** The salvo kernel's hits on a ground unit (CAS, missiles, G-6) use the same `K` rows against the brigade's composition, so a precision munition and a tank gun see the same armor.
- Aimed vs. area fire is a per-weapon-class exponent λ ∈ {0, 1}, the Bracken/Helmbold generalization reduced to a flag. Calibration decides the law; it isn't hard-coded (§2, §4).
- Artillery goes in two tiers (§6). Unguided tube and rocket fire is an area-fire class inside the Lanchester step that uses supply, not munition lots. Guided long-range fires and CAS use the salvo kernel with lots and air-defense exposure.
- EW enters through the **acquisition time** inside the attrition-rate coefficient (Bonder), so jamming hurts fire that depends on observers and radar more than direct fire. It uses only the salvo §6 ratings (T-3 holds).
- Stochastic: Poisson hits per sub-round, with a gamma common-mode draw (salvo M-5's analogue). The deterministic square and linear laws are the verification oracle (§8).

## 2. Literature verified

| Source | What it establishes | What we take |
|---|---|---|
| [Taylor, *Lanchester-Type Models of Warfare*, NPS 1980 (DTIC ADA090843)](https://apps.dtic.mil/sti/pdfs/ADA090843.pdf), vol. II [ADA090842](https://apps.dtic.mil/sti/pdfs/ADA090842.pdf) | **[V]** Comprehensive treatment of deterministic and stochastic Lanchester models, from platoon-level combat up to theater-level air-ground combat. It has a chapter on attrition-rate coefficients, including area-fire weapons and range dependence | Reference text; coefficient derivations |
| [Taylor 1980, NRLQ 27(1)](https://ideas.repec.org/a/wly/navlog/v27y1980i1p109-121.html) | **[V]** Supporting fires that aren't subject to attrition enter variable-coefficient Lanchester equations as an extra term. The system can be transformed back into one without them | Supporting fire is additive. It needs no new solution machinery |
| Bonder (1967) and [the *Operations Research* comment on it](https://pubsonline.informs.org/doi/10.1287/opre.17.5.888) | **[V]** A valid average attrition rate uses the harmonic mean of the variable rates. The constant coefficient is the reciprocal of the expected time to kill a target | `a = 1 / E[t_acquire + t_kill]` is how EW, ISR and terrain enter (§4.3) |
| [Taylor & Neta 2001, NPS](https://faculty.nps.edu/bneta/papers/LanchesterAttrition-RateCoefficientTaylor&Neta.pdf) | **[V]** Analytical attrition-rate coefficient for direct fire in a heterogeneous-target environment with serial target acquisition, verified against simulation | Precedent for coefficients that depend on target mix and acquisition |
| [Kress, *Lanchester Models for Irregular Warfare*, Mathematics 8 (2020)](https://faculty.nps.edu/mkress/docs/Lanchester_Models.pdf) | **[V]** There is one square law (aimed fire) but two linear laws: the "ancient battle" of serial one-on-one duels, and unaimed area fire | λ = 1 means area fire, not ancient battle |
| [Atkinson, Kress & MacKay, *Op. Res.* 2020](https://faculty.nps.edu/mpatkins/docs/lanchester.pdf) | **[V]** The aimed-fire square law implicitly assumes no fire is wasted on targets already out of action. They add simple targeting constraints | Frontage cap and allocation (§4.4), not raw concentration |
| [Fowler, NPS (DTIC ADA315995)](https://apps.dtic.mil/sti/pdfs/ADA315995.pdf); [Part III (ADA364013)](https://apps.dtic.mil/sti/pdfs/ADA364013.pdf) | **[V]** Survey covering Helmbold attrition, the Epstein model, stochastic Lanchester and heterogeneous (mixed-force) attrition, with fire allocation across unit types | Heterogeneous form; allocation |
| [DTIC AD0761759 (NPS thesis, 1973)](https://apps.dtic.mil/sti/pdfs/AD0761759.pdf) | **[V]** Revised Lanchester equations with lethal vs. suppressive weapons, applied to EW and point-defense systems acting as suppressive weapons | Suppression state σ (§4.2) |
| [Bracken 1995, NRL 42(4)](https://econpapers.repec.org/RePEc:wly:navres:v:42:y:1995:i:4:p:559-577) | **[V]** Four Lanchester models fitted to an Ardennes campaign database. The linear model fits in all four cases | Calibration case; don't assume the square law |
| [Fricker 1998, NRL 45(1)](https://vtechworks.lib.vt.edu/items/2c1aee7a-7a02-47da-8937-76e1efdbb4d6) | **[V]** Refits the Ardennes data by regression, adds air sortie data, and finds that neither the linear nor the square law fits | Air sorties matter in fitted attrition; λ stays a calibrated flag |
| [Hung et al. 2005, JORS 56(8)](https://ideas.repec.org/a/pal/jorsoc/v56y2005i8d10.1057_palgrave.jors.2601878.html) | **[V]** Argues for the square law on the Ardennes as a direct-fire campaign, adding tactical-factor and time-shift variables | The square-vs-linear question is unsettled for that data |
| [Kursk heterogeneous Lanchester, arXiv 1903.06666](https://arxiv.org/pdf/1903.06666) | **[V]** A heterogeneous fit attributes tank casualties to both tank and artillery engagements, and uses linear law for area-fire artillery | Calibration case for splitting `K` into direct and indirect rows |
| [Bracken's generalized model, as summarized in arXiv 2207.00861](https://arxiv.org/pdf/2207.00861) | **[V]** Exponent parameters capture nonlinearity, at the cost of more data dependence | λ as a per-class exponent |
| [McLennan-Smith et al. 2019](https://core.ac.uk/download/322736837.pdf) | **[V]** The Epstein ground-combat model has an evolving front through a withdrawal mechanism that couples attrition and movement | Withdrawal via doctrine `break_loss` (§4.5) |
| [Kearney & Martin 2019, arXiv 1905.03122](https://arxiv.org/pdf/1905.03122) | **[V]** Standard Lanchester is deficient because it neglects random fluctuations; they give a stochastic formulation | Stochastic draws, not ODE integration (§4.6) |
| [NRC, *Modeling Human and Organizational Behavior*, App. H](https://nap.nationalacademies.org/read/5869/chapter/19) | **[V]** Close-combat attrition in a time step is sometimes approximated by a local Lanchester equation, but the coefficients are highly situation-dependent | Coefficients are content × multipliers, tuned in the prototype |

**Not found:** a published Lanchester variant with a weapon × target-class lethality matrix in the hard/soft sense used by commercial wargames. Heterogeneous Lanchester has a coefficient per (shooter type, target type) pair, and our matrix is that, factored as weapon class × target class. **[R]** HOI4 combines hard and soft attack by the target's hardness share; that's recalled, not checked against game files.

## 3. What the base law misses, and the fix

| # | Missing effect | Modification [D] | Literature |
|---|---|---|---|
| L-1 | Fires (artillery) | Weapon class with λ = 1 (area fire): hits scale with target density. Unguided, so no salvo lots (§6) | Taylor area-fire coefficients; Kursk fit |
| L-2 | Supporting fire immune to return fire (CAS, long-range fires) | Additive hits from the salvo kernel, converted through staying power (G-6). The air battle sets how many sorties get through | Taylor 1980 supporting fires |
| L-3 | EW and ISR | Acquisition-time multiplier per weapon class (§4.3); suppression σ | Bonder; AD0761759 |
| L-4 | Force mix (hard/soft) | `K[w,t]` × composition `c[t]` (§5) | Heterogeneous Lanchester |
| L-5 | Concentration beyond what the ground allows | Frontage cap per region (§4.4) | Atkinson, Kress & MacKay |
| L-6 | Posture, terrain and fortification | Multipliers on incoming hits by (terrain, posture); the `fortified` posture raises protection | NRC App. H (situation-dependent coefficients) |
| L-7 | Movement and breakthrough | Withdrawal when the loss rate exceeds doctrine `break_loss`; region control is WS-07/WS-11 OQ-F7 | Epstein |
| L-8 | Small-number randomness | Poisson draws with common mode | Kearney & Martin |

## 4. Methodology: one ground engagement

An engagement is per region per tick, with `n_sub` sub-rounds (default 4 [D], tuned in the prototype). All quantities are arrays over (unit × weapon class) or (unit × target class). `s`, `ρ` (readiness × supply) and `ε` (experience) are the WS-11 separate multipliers.

### 4.1 Fire and hits

```
F[u,w]   = o[u,w] · s_u · ρ_u · ε_u · (1 − σ_u) · m_acq[side(u), w] · front_u      (effective fire of unit u, weapon class w)
h[w,v]   = Σ_{u ∈ enemy} F[u,w] · φ[u,w,v] · L[w,v]                              (expected hits on target unit v)
L[w,v]   = Σ_t K[w,t] · c_v[t] · π_v · (d_v / d_ref)^λ_w                          (lethality vs. v's mix)
N[w,v]   ~ Poisson( g[side,w] · h[w,v] ),   g ~ Gamma(κ, 1/κ)                     (common mode; κ → ∞ is independent)
Δs_v     = min( s_v,  Σ_w N[w,v] / S_v )                                          (S_v = WS-11 staying_power; G-6 units)
```

- `o[u,w]`: offense per weapon class at s = 1 (P-11-L1).
- `φ[u,w,v]`: allocation share over legal enemy units, weighted by doctrine priority × `L[w,v]` × presence. This is salvo M-3 with a different weight.
- `π_v`: terrain × posture × fortification multiplier on incoming fire (content, ≤ 1).
- `d_v`: target density = s_v · S_v / frontage(region). It's used only when λ_w = 1.
- Aimed fire (λ = 0): the hits a shooter can make don't depend on how many targets there are, which gives square-law behavior. Area fire (λ = 1): hits grow with target density, which gives linear-law behavior.

### 4.2 Suppression

Suppression σ_v ∈ [0, 1) is the fraction of v's fire lost next sub-round. It decays fully each sub-round [D]:

```
σ_v ← 1 − exp( −α · Σ_w q_w · h[w,v] / S_v )
```

`q_w` is content: high for indirect HE, low for AT. It models the lethal-vs-suppressive split (AD0761759). Suppressive effect needs expected hits, not kills, so it's deterministic given `h`.

### 4.3 EW and ISR through acquisition time

From Bonder's coefficient, a = 1 / (t_acq + t_kill). If jamming or ISR denial multiplies t_acq by j ≥ 1, then:

```
m_acq[side, w] = 1 / ( 1 + f_acq[w] · (j_side − 1) )        f_acq[w] = t_acq / (t_acq + t_kill) at baseline (content)
j_side        = 1 + η · max(0, ew_enemy − eccm_side)          using the salvo §6 ratings and ISR coverage (WS-11 isr_coverage)
```

Indirect fire, which depends on observers, counter-battery radar and links, gets a high `f_acq`. Direct fire gets a low one. So EW stays a modifier (T-3) but isn't uniform. Pushing aimed fire toward area fire under C2 loss (λ drift) is deferred (OQ-L4).

### 4.4 Frontage

Only units whose cumulative frontage fits the region's frontage capacity (WS-08a terrain content) fire in a sub-round. `front_u` ∈ [0, 1] is the engaged share. Units are ordered by posture and doctrine; reserves engage as front-line strength is lost. This bounds the square law's concentration payoff (Atkinson, Kress & MacKay).

### 4.5 Termination

The engagement ends at `n_sub`, or when a side's cumulative loss share exceeds its doctrine `break_loss`. That side withdraws, and region control follows OQ-F7. The WS-11 doctrine presets already carry `break_loss` (salvo §7.3).

### 4.6 Why Poisson and not binomial

Ground fire is an aggregate rate from many weapons, not countable shots from a magazine. Poisson with a gamma common mode (negative binomial) mirrors salvo M-5's beta-binomial: κ → ∞ recovers independent fire, and finite κ fattens the tails for "the attack stalled" and "the line broke" outcomes. Draws per tick: engagements × `n_sub` × enemy units × W, which is about 10⁴ at wartime scale [D], well inside R6 (salvo §10).

## 5. Hard/soft vectorization

| Option | Form | Assessment |
|---|---|---|
| (a) Scalar hardness (HOI4-style [R]) | `attack = h·hard + (1−h)·soft` | Legible, but only two classes. Can't express "AT missiles are wasted on infantry" separately from "infantry is resilient to tank guns" |
| (b) **Matrix × composition** [D, recommended] | `L = Σ_t K[w,t] c[t]` | (a) is the case W = T = 2, K = I. Same code path for any content size; shared with the salvo kernel |
| (c) Per-element-type heterogeneous Lanchester | One state per element type in a brigade | Most faithful, but multiplies state per tracked unit and breaks the WS-11 rollup to one unit (R-6). Rejected |

**MVP content [D]:**
- Weapon classes W = 3 in the Lanchester step: `direct_at` (tank guns, ATGM), `direct_ap` (small arms, autocannon, HE direct fire) and `indirect` (λ = 1).
- The salvo kernel's ground-attack munition classes add K rows for CAS and precision fires.
- Target classes T = 2: `soft` (personnel, trucks, towed guns) and `hard` (armored vehicles). `fortified` is a posture multiplier on π, not a class. More classes, such as a split into `light_armor` and `heavy_armor`, are content later.

**Rollup (P-11-L1).** WS-11 §4.3.2 already sums `firepower` and `protection` per role. The change: `firepower` becomes `o[w]`, summed per weapon class (still one breakdown term per role); `c[t]` is the staying-power share of the roles tagged with each target class; `protection` folds into `S` (vehicle armor is already in vehicle staying power) and π (posture). The headline `offense` scalar becomes Σ_w o[w] · K[w, ref] for a reference mix, so the WS-11 §4.3.3 headline formulas survive.

**Composition under losses.** The MVP keeps `c` fixed and losses proportional across classes, because WS-11 tracks one value-weighted `s` per unit. Tracking per-class strength `s[t]` lets an armored brigade lose its infantry first. It's a later refinement (OQ-L3).

## 6. Fires: which kernel

| Fires | Kernel [D] | Why |
|---|---|---|
| Organic mortars, tube and unguided rocket artillery | Lanchester step, class `indirect` (λ = 1), stock = supply | Volume fire; per-shell lots add nothing. Ammunition tonnage is the WS-11 `supply` multiplier [R: artillery ammunition dominates ground resupply tonnage] |
| Guided MLRS, TBM and cruise missiles, CAS and interdiction munitions | Salvo kernel, munition lots, layers and M-9 exposure | Inventory, interception and air-defense exposure are the strategic levers (ConOps §3). Hits convert through `S` (G-6) using K rows vs. `c` |
| Counter-battery | Both. Artillery brigades are ground units with `c` mostly `soft`; acquisition depends on counter-battery radar via `f_acq` | No special case |

This narrows salvo G-9 (area fire deferred "unless G-6 needs it"). G-6 needs area fire only for **guided** munitions with area effect, such as cluster munitions. Unguided artillery doesn't go through the salvo kernel, so G-9 can stay deferred (OQ-L2).

## 7. Inputs needed

**From WS-11 (P-11-L1, not made):** `GroundStats { o: [f64; W], c: [f64; T], mobility, frontage }` in place of `{ firepower, protection, mobility }`, with `Σ c = 1`. Both are rolled up by `sum` per role (`c` from the staying-power shares). Element content tags each role's weapon class and target class.

**From WS-08a:** region `frontage` capacity and `terrain` key (content).

**WS-07 content (owner WS-07 once it exists):** `K[w,t]` (ground rows plus the salvo ground-attack munition rows), λ_w, q_w, f_acq[w], π[terrain, posture], κ_ground, α, η, `n_sub`.

## 8. Verification

| Check | Criterion |
|---|---|
| Square-law oracle | One class, λ = 0, T = 1, κ = ∞, no frontage or suppression, large forces: MC mean trajectory conserves α A² − β B² within 1% over the engagement |
| Linear-law oracle | One area class (λ = 1) on both sides: conserves α A − β B within 1% |
| Hard/soft reduction | W = T = 2, K = I: per-hit lethality equals the scalar-hardness formula exactly (option a) |
| EW isolation | A class with f_acq = 0 is unchanged by any j |
| Supporting fire | Salvo hits on a brigade change only Δs via `S`, and add to the same tick's Lanchester losses without double counting (G-6) |
| Small-number spread | Finite κ raises loss SD at equal mean; MC differs from the ODE for small forces as expected (Kearney & Martin) |
| Determinism | ARC-060 double-run hash |

**Calibration (prototype):** Ardennes (Bracken; Fricker with air sorties) and Kursk (heterogeneous tank/artillery split). Fit direct-fire K **excluding** losses already attributed to artillery and air, otherwise fires are counted twice.

## 9. Gaps: address or accept

| # | Gap | Recommendation |
|---|---|---|
| GL-1 | Space, movement and maneuver beyond withdrawal | **Accept:** region-level only (ConOps: no tactical layer) |
| GL-2 | Composition drift under losses | **Defer** (OQ-L3) |
| GL-3 | C2 collapse (aimed → area drift) | **Defer** (OQ-L4) |
| GL-4 | Combined-arms synergy beyond the mix | **Accept thin:** doctrine multiplier from the WS-11 doctrine presets (combined arms, networked maneuver) |
| GL-5 | Mines, obstacles, engineers | **Accept thin:** a posture/terrain π term for the defender |
| GL-6 | Irregular warfare and insurgency (Deitchman) | **Out** of this note; WS-04 internal stability may want it |

## 10. Open questions

| ID | Question | Recommendation [D] |
|---|---|---|
| OQ-L1 | Adopt matrix × composition (§5 b) for ground combat and share `K` with the salvo kernel? | Yes |
| OQ-L2 | Unguided artillery in the Lanchester step, guided fires in salvo (§6)? | Yes. Keeps G-9 deferred |
| OQ-L3 | Track per-class strength `s[t]`? | Not in MVP |
| OQ-L4 | Model C2 loss as λ drift? | Not in MVP; EW stays acquisition plus suppression |
| OQ-L5 | `n_sub` and κ_ground defaults | 4 and ≈ 20, tuned in the prototype |

## 11. Proposals (not made)

| ID | Target | Change |
|---|---|---|
| P-11-L1 | WS-11 §3.5 `GroundStats`, §4.3.2–4.3.3 | `firepower` → `o[w]`, `protection` → `c[t]` (+ π via posture), add `frontage`. Rollup by `sum` per role. Headline offense = Σ_w o[w] · K[w, ref] |
| P-07-L1 | WS-07 kickoff (`prompts/WS-07.md`, POAM #52) | Read this note; WS-07 owns K, λ, q, f_acq, π, κ_ground, n_sub, and the §8 oracles |

IDs carry the `-L` suffix, following the `P-CON-F1` pattern, because the `P-07`/`P-11` letter series are already used by several sources.

## Sources
Listed inline in §2.
