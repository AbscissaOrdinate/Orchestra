# WS-04 — Governance & Factions
Status: Draft v0.2 · Parent: ConOps §3, §5 (P-3, P-4, P-6, P-7, P-8), §6 governance/factions/services rows, SYS-002, SYS-004, SYS-006, SYS-007, SYS-008 · Depends on: WS-02 v1.1, WS-03 Draft v0.1 (§2.4 contract), WS-08a Draft v0.1 (§3.5 field ownership) · Last updated: 2026-11-27

Tags: **[V]** verified (source linked) · **[R]** recalled, not checked · **[D]** design choice (to be tuned in the M2(c) prototype).
Prototype plan: `prototypes/governance/PLAN.md`.

**v0.2 changes (Sean's review, 2026-11-27):** the policy space is now eight TI-inspired axes, and each faction cares about 2–3 of them. Unity is now a discretionary program rather than an axis. Faction power now comes from nine **institutions** (control points) whose weights vary by government archetype, and these institutions also replace the separate veto-player list. Services are now three factions (Military, Navy, Aerospace), and Navy and Aerospace split off only when their service is large enough. The WS-03 interface now follows the WS-03 §2.4 contract exactly: `EconKey`, `EconOutcomes`, corruption rents as private goods, `FactionEconBehavior`. Presets now cover every WS-08a `Government` variant. OQ-G1–G4 and G6 are closed.

---

## 1. Purpose & scope

WS-04 turns the economy's population into political actors and turns institutions into numbers the player can change. Each tick it works out the following:
- who holds which institutions and how much power that gives each faction;
- how satisfied each faction is;
- whether the government keeps its coalition;
- whether trouble ends in an election loss, a coup or a revolution.

It also gates policy changes (institutional vetoes, ethics, political capital), sets corruption and other inputs WS-03 needs, and runs reform of the government itself as a project.

**In scope (MVP):** government type as a parameter set; institutions with archetype weights and faction control; data-driven factions over WS-03 `EconKey`s; eight policy axes; selectorate mechanics (public goods vs. corruption rents, loyalty, challenger risk); legitimacy, unrest and corruption; elections, coups and revolutions; political capital; a Unity program; reform as a staged project, including composing a custom government; foreign-policy lobbies (WS-06); a macro-tier reduced form; the WS-08a start-state mapping.

**Out of scope:** characters, leaders and dynasties (succession is an abstract hazard); parties and detailed elections (factions vote as blocs); civil war and secession; freeform scripted events; foreign subversion of factions (post-MVP, WS-06); player-authored laws; playable colonies (colonial administrations run macro tier only).

**Player stance (OQ-G4 closed).** The player always steers the state. Losing an election or suffering a coup changes the coalition and the institutions the player governs through, but it never ends the game.

## 2. Decisions

| # | Decision | Recommendation |
|---|---|---|
| 1 | Policy space | **Eight axes after Terra Invicta's priorities**: Economy, Welfare, Knowledge, Government, Oppression, Military, Nuclear, Space. Each faction sets an ideal on 2–3 of them. **Unity** is a discretionary program, not a position. Every instrument maps onto axes through one content table. Money effects come straight from WS-03 `EconOutcomes`, not from the axes. |
| 2 | Faction set | **A data-driven schema with 12 default archetypes** (§6.2), dormant factions that activate on conditions, and bounded drift of ideal points. Factions are never spawned at runtime. |
| 3 | Power | **Nine institutions** (executive, legislature, judiciary, bureaucracy, security, media, industry, oligarchs, clergy). Each government archetype gives every institution a weight. Each institution's control is shared among factions by a content rule. A faction's clout is Σ weight × control. Institutions with a constitutional veto scope are the veto players. |
| 4 | Stability & crises | **Stocks** (legitimacy, unrest, corruption, political capital) **plus logistic monthly hazards** for coup and revolution. Coalition loss and elections are threshold rules. |
| 5 | Reform | **A reform is a diff of institution parameters.** It moves through Deliberation (progress ∝ support under the *new* rules), Ratification (institutional vetoes) and Transition. It can fail by stalling, by veto or through backlash. A decree path also exists. |
| 6 | WS-03 contract | WS-03 §2.4 as written. WS-04 asks for one field (`wealth` in `KeyOutcome`), one ownership fix (CBI) and one clarification (rents are never key income) (§3.2, OQ-G7). |

### 2.1 Options and trade-offs (condensed)

**D1 — Policy space.** Terra Invicta's national priorities include Economy, Welfare, Knowledge, Government, Unity, Oppression and Spoils [V: [TI wiki, Control Points](https://wiki.hoodedhorse.com/Terra_Invicta/Control_Points)]. They're intuitive because each names *what the state is doing*. Two cautions. First, TI priorities are effort allocations, not ideologies: in TI, Government is a democracy score that the Government priority raises and the Oppression and Spoils priorities lower [V: [TI wiki, Priorities](https://wiki.hoodedhorse.com/Terra_Invicta/Priorities)]. In Orchestra the budget and laws *are* the allocation, so the axes are read off them, and there are no separate priority sliders. Second, an axis earns its place only if it drives a mechanic. Applying that test to your proposal:

| Axis | − pole / + pole | Instruments (default map) | Mechanic it drives | Change from proposal |
|---|---|---|---|---|
| **Economy** | competitive / cooperative | tariffs, subsidies, state ownership, price controls, investment directives, capital controls, −CBI | Industrialists vs. Labor and Rural; WS-03 `SetRegulation`, tariffs, monetary | As proposed |
| **Welfare** | elite-serving / broad-based | tax progressivity, transfers, pensions and pension age, unemployment benefit, health/GDP | Redistribution conflict; WS-03 incidence | Poles reworded. *Pluralism* is political, so it moved to Government and Oppression; Welfare is about who the state's money serves |
| **Knowledge** | traditional / modern | civil R&D/GDP, education/GDP, religion-and-state law, women's-status law | Technocrats vs. Traditionalists; WS-05 funding; WS-03 human capital | As proposed |
| **Government** | patronage / accountable | corruption level (Spoils), administration & enforcement/GDP, judicial independence, election fairness, franchise breadth | **The selectorate trade-off**: rents buy loyalty but leak public spending; reform evaluation; ideological legitimacy | Takes in anti-corruption and the v0.1 Inclusion axis. TI's Spoils becomes this axis's instrument |
| **Oppression** | libertarian / authoritarian | press, assembly, internal-security and civil-military laws; internal security/GDP | Repression of unrest, coup-proofing, intel opacity (WS-06 noise) | As proposed |
| **Military** | antimilitarist / militarist | conventional defense lines/GDP (personnel, O&S, procurement, defense R&D minus strategic share), conscription | Services; WS-11 budget and manpower | Split from v0.1 Security |
| **Nuclear** | disarmament / deterrence | strategic share of defense spending (WS-11 tag), nuclear-posture law | Strategic forces and deterrence credibility (WS-11, WS-06) | Split as you asked: both "nukes instead of divisions" (the U.S. New Look, the UK's 1957 Sandys White Paper [R]) and "divisions without nukes" are real coalitions |
| **Space** | earthbound / expansionist | space program/GDP | Aerospace and Technocrats vs. Welfare constituencies; WS-05/WS-08 | Recommend this rather than extraction vs. ecocentric. An ecology axis has no mechanic until there's an environment model (post-MVP; WS-08b resources could carry "extraction vs. conservation" later) |
| ~~Unity~~ | — | — | Becomes the **Unity program** (§4.8): it spends PC to raise legitimacy, damp unrest and pull factions' ideals toward the government's | Your description (consolidating factions and legitimacy) is an action, not a stance |

Eight axes is more than v0.1's six. But each faction weighs only 2–3, so a player reading one faction sees 2–3 numbers, and every axis maps to its own mechanic. Merging Military and Nuclear would save an axis but erase a real political cleavage.

**D2 — Factions.** The options were (a) fixed archetypes in code, which violates P-6; (b) emergent clustering, which is illegible and untestable; (c) **content archetypes with membership computed from `EconKey`s each tick, dormant factions, and drift**. → (c). The defaults draw on real political families (§6.2). Services (OQ-G2 closed): one Military faction holds every service, and Navy and Aerospace (air plus space) become factions only when their service passes 15% of military personnel. Small militaries keep one faction; large ones get three.

**D3 — Power.** v0.1 weighted political strength by votes, wealth and status (the V3 precedent) and kept veto players as a separate list. TI instead gives nations control points that factions hold, with named types such as Bureaucracy and Oligarchs [V: [TI wiki, Missions](https://hoodedhorse.com/wiki/Terra_Invicta/Missions)], and their count scales with GDP [V: [TI wiki, CP](https://hoodedhorse.com/wiki/Terra_Invicta/CP)]. → **Institutions as weighted control points, with continuous control shares rather than discrete slots.** A domestic faction's grip on the media or the bureaucracy shifts gradually, and discrete slots would flip on rounding. The V3 idea survives inside the control rules: the legislature is shared by votes, the oligarchs by wealth, and security by coercion. Archetypes differ by weights: a junta's security weight is 0.45, a parliamentary republic's is 0.05. The same institutions are the veto players and carry TI-style effects (§4.7). I added judiciary and clergy to your seven: courts are the natural reform veto, and clergy are needed for theocracies and traditional monarchies.

**D4 — Stability.** Options: one stability scalar, Granovetter cascades, or **stocks plus per-actor logistic hazards** (legible, calibratable). → The last.

**D5 — Reform.** Options: instant reform, a monthly pass chance, or **a staged project scored under the proposed rules**. → Staged. Extending the franchise (the UK Reform Acts [R]), a regime voting itself out through existing veto players (Spain 1976 [R]) and a self-coup (Peru 1992 [R]) are all playable.

**D6 — S and W.** S comes from the franchise, and W = ρ·S with ρ (`coalition_share`) as a parameter. Institution control decides *which* factions the coalition needs; ρ decides how cheaply rents buy their loyalty.

## 3. Interfaces

### 3.1 Stage placement and lags

| What | Stage | Reads | Lag |
|---|---|---|---|
| Order gating (budget, tax band, laws, coalition, reform, Spoils, Unity) | R1 | stage-start world + this tick's orders; `econ::project` for `EnactBudget` | none |
| Factions, institutions, approval, coalition, elections, corruption, hazards, reform, lobbies | R5 | R4-committed `econ` (this tick); `force`, WS-06 relations (previous tick) | economy → politics same tick; forces/diplomacy → politics 1 tick |
| WS-03 consumes WS-04 outputs | R2–R4 | committed `gov` | 1 tick (WS-03 §2.1) |
| WS-11 consumes conscription, coup loyalty | R5b | R5-committed `gov` | same tick |
| WS-06 consumes summaries, lobbies, crisis events | D2, R6 | committed `gov` | D2: 1 tick; R6: same tick |

### 3.2 WS-03 contract (conforms to WS-03 §2.4)

**WS-03 → WS-04** (read in R5): `EconOutcomes` per `EconKey { bin, sector, role }` (pop_share, real_disp_income, disp_growth_12m, eff_tax_rate, transfers, unemp_exposure, inflation_loss); spending per budget line; **corruption rents** (the leakage amount); stress indicators (spread, rating band, inflation, real growth, shortage index, arrears, conditionality index, warning-band events); `project()` incidence per key for a draft budget.

**WS-04 → WS-03** (read next tick):

| Field | Range | Rule (§) |
|---|---|---|
| `cbi` | [0, 1] | Institution parameter (§4.1) |
| `corruption` | [0, 1] | Corruption stock K (§4.6) |
| `stat_bias` | [0, 0.3] | 0.3·(1 − accountability) (§4.10) |
| `FactionEconBehavior { membership, invest_sentiment, labor_militancy, flight_propensity, compliance }` | per WS-03 §2.4 | §4.10 |
| Budget enactment result | allowed / blocked(reason) | `gate_orders` (§3.4) |
| Monthly discretion band | pp per tax per fiscal year | 2 pp × clamp(W_exec / 0.2, 0.5, 2) [D] (answers WS-03 OQ-E3) |
| Price-control legality | bool | false if the ethics box forbids `price_controls` |
| Laws: conscription, pension age, women's status | law options | §6.3 |
| `shocks` (coup, revolution) | capital_loss, output_loss ∈ [0, 0.5] | §4.6; WS-03 applies them in R2 via its event-capital-destruction input |

**Requested WS-03 edits (OQ-G7; WS-04 does not change WS-03):**
1. ~~Add `wealth: f64` to `KeyOutcome`.~~ **Done (2026-11-27):** WS-03 §2.4 and §3.5 (key-wealth rule), ECO-090. Transfer and Public keys carry no sector, so there are at most 240 keys.
2. CBI ownership. WS-03 lists `cbi_default` in archetypes and CBI in `start.ron`. Under WLD-022 (one primary owner per start field), CBI belongs to the WS-04 government type. WS-03 should read `cbi` only from WS-04 and drop its own default, or keep the default as a macro-tier fallback only.
3. **Done (2026-11-27, WS-03 §2.4 note).** Rents: WS-04 assumes corruption rents are *not* added to any key's `real_disp_income`. WS-03 §3.6 says WS-04 allocates them, so WS-04 counts them as private goods. WS-03 should state this, so rents are never counted twice.

### 3.3 WS-06, WS-11, WS-05, WS-08a contracts

| Direction | Field | Notes |
|---|---|---|
| WS-06 → WS-04 | `alignment[j]` ∈ [−1, 1], `war_burden`, `war_started_tick`, `threat[j]` (from beliefs, P-2) | previous tick |
| WS-04 → WS-06 | `GovSummary` (public): institutions, weights and controllers, ethics, regime age, coalition ideal, crisis events | via `PolityView` |
| WS-04 → WS-06 | stability, legitimacy (truth) + `public_noise_sigma` = 0.05 + 0.20·(1 + p_Opp)/2 [D] | WS-06 intel pushes `Observation`s |
| WS-04 → WS-06 | `lobbies: [(j, Π_j, members)]`; `IndependenceDeclared { subject }` event | §4.11; colonial revolution success (§4.12) |
| WS-11 → WS-04 | personnel and readiness per service (army, navy, air, space, internal security), strategic share of defense spending | previous tick |
| WS-04 → WS-11 | conscription and nuclear-posture law options; `coercion_loyalty[faction]` ∈ [−1, 1] | WS-11 decides how units follow a coup |
| WS-05 ↔ WS-04 | in: `arms_dependency[j]`; out: `rnd_morale` = 1 + 0.1·A_Technocrats [D] | R&D efficiency multiplier |
| WS-08a → WS-04 | `start.government` (8 variants), `start.stability` (5 bands), subject relations | §4.12, `archetype_map.ron` |

### 3.4 Orders and the gate

WS-04 adds these `OrderKind` variants: `SetLaw { group, option }` · `FormCoalition { factions }` · `SetRentAllocation { shares }` · `SetSpoils { level }` · `SetUnity { intensity }` · `AuthorizeOverride { institution }` · `ProposeReform { target }` · `CancelReform` · `DecreeReform { target }`.

WS-03 orders are gated by one pure function, which both subsystems call in R1:

```rust
pub fn gate_orders(world: &World, orders: &[Order]) -> Vec<Gate>;   // one per order, in log order
pub enum Gate { Allowed { pc_cost: f64 }, Blocked { reason: BlockReason } }
pub enum BlockReason { Unconstitutional { axis: AxisIdx }, Veto { institution: InstIdx, overridable: bool },
                       NoPoliticalCapital, OutOfBand, InvalidTarget(String) }
```

| WS-03 order | Gate |
|---|---|
| `EnactBudget` | Ethics box; then every institution with `budget` veto scope scores the draft with `econ::project` (material) plus the axis change (ideology). Blocked → WS-03 keeps last year's budget |
| `AdjustTax` | Ethics box and the discretion band only (the band is the executive's own latitude) |
| `Supplemental` | PC cost 5 per 1% of GDP [D] |
| `SetRegulation`, `PublicInvestment`, `IndustrialConversion` | Ethics box; `law` veto scope |
| `SetMonetary` | None (WS-03 scales it by 1 − CBI) |

PC accumulates in log order within a tick. Both subsystems call the same function on the same inputs, so they always agree (GOV-064).

### 3.5 Block layout

```rust
pub struct GovBlock {
    pub inst: Institutions,              // all tiers; public
    pub laws: Vec<LawOption>,            // all tiers
    pub summary: GovSummary,             // all tiers: L, R, K, Σ, rho, regime_age, coup_history,
                                         // coalition_ideal[8], crisis flags (Tracked)
    pub out: GovOutputs,                 // all tiers: §3.2 / §3.3 outputs
    pub detail: Option<Box<GovDetail>>,  // High only: factions, control matrix, coalition, PC, reform, lobbies
}
```

## 4. Model

All quantities are per polity per monthly tick. Indices: k EconKey, f faction, i institution, a axis, j foreign polity.

### 4.1 Institutions (the government parameter set)

| Parameter | Type / range | Meaning |
|---|---|---|
| `franchise` | `Universal` · `Property { min_bin }` · `Restricted { roles, min_bin }` · `None` | Formal voice: v_k ∈ [0, 1] per key; S_people = Σ_k pop_k·v_k |
| `coalition_share` ρ = W/S | (0, 1] | W_people = ρ·S_people |
| `institutions[i]` | weight W_i ≥ 0 (Σ = 1); `veto_scope` ⊆ {budget, law, reform}; tolerance τ_i; discipline λ_i ∈ [0, 1]; optional control-rule override | §4.2, §4.7 |
| `coalition_threshold` θ | [0.3, 0.8] | Non-executive clout a coalition needs |
| `succession` | `Elected { period_months, fairness φ }` · `Hereditary` · `Appointed { mu_months, window_mult }` | §4.5 |
| `legitimacy_weights` | simplex over {electoral, performance, traditional, ideological} | §4.6 |
| `ethics` | center e ∈ [−1, 1]^8, half-widths h ∈ (0, 2]^8, forbidden instruments/law options | Constitutional policy box |
| `cbi` | [0, 1] | Output to WS-03 |
| `decree_powers` | bool | Vetoes are constitutionally overridable |
| `amendment_threshold` θ_r | [0.5, 0.8] | Reform support needed |
| `macro_military_share` | [0, 1] | Security-control proxy for macro polities |

Presets (§6.1) are named parameter sets. A custom government is any valid set.

### 4.2 Keys, membership, institution control, clout

**Keys.** At most 240 `EconKey`s per polity (20 bins × 5 sectors × Wage/Capital, plus 20 Transfer and 20 Public keys without a sector; empty keys are omitted). Each carries income, tax, transfer, unemployment, inflation and wealth fields, and the player never sees them directly.

**Membership.** Content gives each faction an affinity a_fk ∈ [0, 1] over `EconKey` selectors (bin ranges, sectors, roles). Military factions' affinity on `Role::Public` keys equals each service's personnel ÷ public employment (WS-11 counts). Shares m_fk = a_fk / max(1, Σ_f a_fk); the remainder is apolitical; a key can split across factions (WS-03 §2.4, V3 precedent). WS-04 publishes m_fk as `FactionEconBehavior.membership`. A faction's population share is h_f = Σ_k m_fk·pop_k; its *core weight* on key k is w_fk ∝ m_fk·pop_k·real_disp_income_k (normalized; the politically relevant members).

**Control rules** (content per institution; laws may override). Each gives control shares q_fi ≥ 0 with Σ_f q_fi = 1:

| Rule | Basis per faction | Default institution |
|---|---|---|
| `Votes` | Σ_k m_fk·pop_k·v_k, mixed with coalition share by election fairness: φ·votes + (1 − φ)·coalition | legislature |
| `Keys { selector, by: Pop · Income · Wealth }` | Σ_{k ∈ selector} m_fk·pop_k·{1, income_k, wealth_k} | bureaucracy (Public, non-military, by income); media under free press (bins ≥ 10, services, by income); industry (Capital role in industry, defense, space, by income); oligarchs (bins ≥ 18, by wealth) |
| `Coercion` | Σ_service m_f,service·personnel·readiness·coercion_weight | security |
| `Tag { tag }` | membership of factions carrying the tag | clergy (Traditionalists) |
| `Lagged { of, tau }` | EMA of another institution's shares | judiciary = EMA of executive with τ = 120 months (independent) or 12 (captured) by the judicial-independence law |
| `Coalition` | f ∈ C: share of C's non-executive clout | executive |

Law overrides: under restricted press, media = 0.5·`Keys` + 0.5·`Coalition`; under censorship, media = `Coalition`. Under corruption K, bureaucracy = (1 − K)·`Keys` + K·`Coalition` (patronage politicizes the civil service). If an institution has no active controllers, its weight is spread pro rata over the rest.

**Clout.** Non-executive clout c⁻_f = Σ_{i ≠ exec} W_i·q_fi / (1 − W_exec). Then executive control q_f,exec = 1[f∈C]·c⁻_f / Σ_{g∈C} c⁻_g, and clout c_f = Σ_i W_i·q_fi. This is two passes, with no fixed point to solve. **Coercion** κ_f = q_f,security. **Vote clout** c^v_f = the `Votes` basis share before the fairness mix.

**Drift.** If A_f < 0, f's ideals on its salient axes move away from current policy at δ·(−A_f) per month (δ = 0.002 [D]), clamped to [−1, 1]. If A_f > 0.3, they relax toward the content default at δ/2. The Unity program (§4.8) adds a pull toward the coalition ideal.

**Service factions.** Navy (or Aerospace) is active while its service holds ≥ 15% of military personnel. It deactivates below 10% (hysteresis), and its members return to Military.

### 4.3 Policy axes

p_a = tanh(Σ_x M_ax·(x − x⁰)) over instruments x: budget lines as shares of GDP, WS-03 tax and regulation values, law options (one-hot), Spoils/corruption, and institution parameters (Government axis). M, x⁰ and the scale are content (`policy_map.ron`). Every instrument also has one **channel** tag: `public` (value weight v_l), `incidence` (its effect arrives through `EconOutcomes`), `rents`, or `none`. Validation fails if any instrument lacks a map row or a channel (GOV-021).

### 4.4 Utility, approval, loyalty (selectorate mechanics)

Units are a fraction of the faction's core income per month. Ē[·] is an EMA with τ_E = 36 months [D].

| Term | Formula | Default |
|---|---|---|
| Material | mat_f = Σ_k w_fk·disp_growth_12m_k (WS-03). Breakdown: 12-month changes of the faction's core-weighted eff_tax_rate, transfers, inflation_loss and unemp_exposure, plus a residual "market income and membership change" term. The terms sum exactly. WS-04 keeps a 12-month ring of each faction's 5 core-weighted fields (≈ 720 values per polity). | fully adaptive (12-month window) |
| Public goods | pub_f = β_f·G_pc / y_f, G_pc = Σ_{l∈public} v_l·spend_l / population | β_f content, default 2 [D] |
| Private goods | prv_f = 1[f∈C]·Rents·r_f / (W_people·κ^C_f·y_f), where Rents is the WS-03 leakage amount, r_f the player's allocation and κ^C_f f's share of coalition clout | default r_f = κ^C_f |
| Adapted level | X̃_f = X_f − κ·Ē[X_f] for X ∈ {pub, prv} | κ = 0.7 [D]: a permanent change keeps 30% of its effect |
| Ideology | ide_f = −Σ_{a ∈ salient(f)} ω_fa·((p_a − i_fa)² − r²) | 2–3 salient axes per faction (GOV-016); ω default 0.03, r = 0.5 [D] |
| Line interest | lin_f = Σ_l λ_fl·ln(spend_l / Ē[spend_l]) | e.g., Navy on its procurement share |
| Foreign | for_f = 0.03·Σ_{j: f in a lobby on j} φ_fj·alignment_j | §4.11 |
| War | war_f = −ω_war,f·war_burden + 0.05·exp(−months at war / 6) | short rally [R] |
| Grievance | event-driven (purge, clout lost in a reform), decays with τ = 24 | §4.6, §4.9 |
| Baseline | b_f (content), plus the start-state shift (§4.12) | |

U_f = mat + pũb + prṽ + ide + lin + for + war + grv + b. **Approval** A_f = tanh(U_f / 0.10) [D]. **Loyalty** for coalition members: Λ_f = tanh((U_f − ρ·prṽ_f) / 0.10). For everyone else, Λ_f = A_f. **Support** s_f = (1 + Λ_f)/2. **Stability index** Σ = Σ_f c_f·A_f.

The ρ discount is the selectorate mechanic as a game rule (m1-findings §2.2 [V]). A challenger can promise the same rents, and a defector has probability ρ of joining the next coalition, so only (1 − ρ) of private goods is secured by staying loyal. In Orchestra the private good is **corruption rents**, and rents leak from public spending (WS-03 §3.6). Tolerating corruption therefore buys coalition loyalty at the cost of public goods and growth, which is the core BdM trade-off with no separate patronage line. With equal incomes, a unit of rents beats a unit of public spending for coalition loyalty when (1 − ρ)/(β·ρ·S_share) > 1. That holds in every autocratic preset (ratio ≈ 5–450) and fails in the democracies (≈ 0.8). Elections widen the gap, because public goods also win votes outside the coalition (GOV-035).

**Breakdowns: the squash rule.** For any monotone transform y = g(x) with x = x₀ + Σ t_i, set term_i = t_i·(g(x) − g(x₀))/(x − x₀) (0 if x = x₀) and add a base term g(x₀). The terms sum exactly to y and keep their sign and labels. This applies to A, Λ, the axes and every probability.

### 4.5 Coalition, elections, succession

- **Formation (OQ-G1 closed: the player picks).** C is valid if Σ_{f∈C} c⁻_f ≥ θ. `FormCoalition` requires every joining faction to have Λ_f ≥ −0.1 (evaluated as if joined) and costs PC. In electoral systems an election result replaces C; between elections the player may re-form it.
- **Departure.** A member with Λ_f < −0.3 for 3 consecutive ticks leaves (event).
- **Minority.** If Σ_{f∈C} c⁻_f < θ: electoral systems hold an early election 3 ticks later unless C is re-formed. Other systems enter regime crisis (coup and revolution logits + ln 3; PC income 0) until re-formed.
- **Elections.** Held every `period_months`, or early. π_f = σ(3·A_f + 0.5·(2·1[f∈C] − 1)); V = Σ_f c^v_f·π_f + ε, ε ~ N(0, 0.04) [D]; reported V_rep = φ·V + (1 − φ)·max(V, θ + 0.1). If V_rep < θ, the opposition forms C. It seeds with the faction with the largest c^v_f·(1 − π_f), then adds factions in order of ideal distance on the union of their salient axes (ties by content ID) until the formation rule holds. φ < 1 models hegemonic parties (Mexico's PRI [R]).
- **Succession** (`Hereditary` μ = 300; `Appointed` μ per preset). Leader-change hazard is 1/μ per month. Each event opens a 3-tick window: coup logits + ln(window_mult) (default 3; personalist 5), and coalition members with Λ < 0 leave. Juntas are least durable [V: [Geddes, Frantz & Wright 2014](https://bpb-us-e1.wpmucdn.com/sites.psu.edu/dist/0/12570/files/2014/06/GeddesFrantzWrightAR.pdf)]. Single-party and monarchic regimes institutionalize succession while military and personalist ones don't [V: [Frantz](https://bpb-us-e1.wpmucdn.com/sites.psu.edu/dist/0/12570/files/2014/06/FrantzCPL.pdf)], which μ and window_mult encode.

### 4.6 Legitimacy, unrest, corruption, crises

**Legitimacy** L ∈ [0, 1]: L ← L + (L* − L)/24 + events + Unity. L* = Σ_src w_src·D_src:
- D_elec = φ·√S_share if `Elected` and the last election was on schedule, else 0.
- D_perf = σ(4·(g − ḡ) − 3·(π − π̄) − 2·(u − ū) − 3·shortage), using WS-03 stress indicators with EMAs (τ = 60).
- D_trad = (1 − exp(−age/240))·(0.5 + 0.5·s̄_clergy), where s̄_i = Σ_f q_fi·s_f.
- D_ideo = 1 − Σ_f c_f·‖i_f − e‖_salient / (2·√(n_salient)).

**Unrest** R ∈ [0, 1]: R ← R + (R* − R)/6. R* = clamp((1 − ψ)·(1 − 0.5·L)·Σ_f h_f·μ_f·max(0, −A_f), 0, 1). Repression is ψ = min(0.6, ψ_law + 0.1·(internal security/GDP)/0.01)·s̄_security [D]: repression works only through a loyal security apparatus.

**Corruption** K ∈ [0, 1] (output to WS-03): K ← K + (max(K_target, K_floor) − K)/36. K_target is set by `SetSpoils`. K_floor = 0.6·(1 − capacity), capacity = clamp(0.4·(enforcement/GDP)/0.01 + 0.3·judicial independence + 0.3·s̄_bureaucracy·(1 − q_C,bureaucracy), 0, 1) [D]. The player can raise corruption at will but lower it only as fast as state capacity allows. When rents fall, coalition factions lose prṽ through the adaptation term, which mirrors TI's elite unhappiness when spoils fall below corruption [V: [TI wiki, CP](https://hoodedhorse.com/wiki/Terra_Invicta/CP)].

**Coup hazard** for each faction with κ_f > 0:
ℓ_f = −8.5 + 3·(−Λ_f) + 2·κ_f + 2·R + 2·(1 − L) + 0.5·H − 2·χ + crisis terms
H counts past coups, each decaying with τ = 120 months: the coup trap [V: [Wikipedia, Coup d'état](https://wikipedia.com/wiki/Coup)]. χ ∈ {0, 0.5, 1} is the civil-military law. *Crisis terms* are the active additive shifts: regime crisis + ln 3, succession window + ln(window_mult) (coups only), reform transition + ln(1 + D), decree aftermath + ln(1 + 2D). Factions are evaluated in content-ID order, and at most one coup happens per tick.

**Coup resolution.** The plotters are the initiator plus every coercive faction with Λ < 0. P_success = K_p^1.5 / (K_p^1.5 + (K_l + 0.3·L)^1.5). The target is about half of attempts succeeding [V: 457 attempts 1950–2010, about half successful, [Wikipedia citing Powell & Thyne](https://wikipedia.com/wiki/Coup)].
- Success → institutions := the plotter faction's `coup_constitution` (default `military_junta`). C is re-formed from the plotters plus factions with Λ > 0. L := 0.25; H += 1; age := 0; shock {output_loss 0.02}.
- Failure → plotters get grv −0.3 and coercion × 0.5 for 24 ticks; H += 1; L += 0.02. Failed coups tend to strengthen autocrats [V: same source].

**Revolution hazard:** ℓ_rev = −9 + 6·R + 2·(1 − L) − 3·κ_loyal + crisis terms, where κ_loyal = Σ_f κ_f·1[Λ_f ≥ 0]. Success P = R / (R + κ_loyal + 0.1).
- Success → the revolutionary faction is argmax_f h_f·μ_f·max(0, −A_f); institutions := its `revolution_constitution`; C is formed from the factions nearest its ideal; L := 0.4; age := 0; shock {capital_loss 0.10, output_loss 0.08, decay 12} [D].
- Failure → R × 0.5, L − 0.1, grv −0.2 for factions with A < 0.

Coups and revolutions are forced reforms (§4.9).

### 4.7 Institution effects, political capital, ethics, vetoes

| Institution | Effect (TI-style), scaled by weight W_i and controllers' support s̄_i |
|---|---|
| Executive | PC income; discretion band (§3.2); decree if `decree_powers` |
| Legislature | budget/law veto where scoped; vote clout for elections |
| Judiciary | reform/law veto where scoped (constitutional review enforces the ethics box); capacity term in K_floor via judicial independence |
| Bureaucracy | corruption capacity (§4.6); compliance (§4.10) |
| Security | coercion in coups and revolutions; repression effectiveness |
| Media | legitimacy; Unity effectiveness; `stat_bias`; intel noise |
| Industry | power; budget/law veto in corporatist presets |
| Oligarchs | power; flight amplification (§4.10); veto in oligarchic presets |
| Clergy | traditional legitimacy; law veto in theocracies |

**Political capital** P ∈ [0, 100]: income 4·(Σ_{f∈C} c_f·s_f / θ)·(0.5 + L)·(0.5 + W_exec/0.4) per month [D]. Costs [D]: law change 10 per option step; `FormCoalition` 15; `ProposeReform` 10·D + 1 per month; `AuthorizeOverride` 20 and L − 0.05; `SetUnity` 2 per intensity per month; `Supplemental` 5 per 1% of GDP. Every cost is multiplied by (1 + 0.25·n_veto). Tax changes inside the band and the annual budget cost no PC: they are gated by ethics and vetoes and move approval directly (OQ-G3 closed).

**Ethics.** An order is *unconstitutional* if it would push any axis outside [e_a − h_a, e_a + h_a] or use a forbidden instrument or law option. Changing ethics requires a reform. The ruling class is whoever controls the vetoing institutions.

**Vetoes.** Institution i with scope s approves a change iff Σ_f q_fi·(ΔU_f + λ_i·1[f∈C]·Λ_f·0.10) ≥ −τ_i. ΔU_f is the ideology change from the proposed axes, plus the projected material change for `EnactBudget` (from `econ::project`), plus ΔU^ref for reforms. λ_i is party discipline: 1.0 for parliamentary legislatures and 0.3 for presidential ones [D] (the Linz contrast [R]). A block names the institution. An override needs `decree_powers`.

### 4.8 Unity program

`SetUnity { intensity ∈ {0, 1, 2, 3} }` costs 2·intensity PC per month. With media effectiveness m = 0.3 + 0.7·q_C,media (state media amplifies, a free press damps):
- L* += 0.05·intensity·m;
- R* × (1 − 0.1·intensity·m);
- each faction's salient ideals move toward the coalition ideal at 0.001·intensity·m per month.

This is "consolidation of factions and legitimacy" as an action with a running cost.

### 4.9 Reform

The target is a full `Institutions` value. Distance D = Σ_j weight_j·|z_j^new − z_j| / range_j, with content weights; institution weight vectors count by L1/2.

1. **Deliberation.** Progress starts at π = 0 and moves by π += 0.08·(Sup − θ_r)/max(D, 0.25) per tick. Sup = Σ_f c_f·σ(3·ΔU^ref_f/0.10), where ΔU^ref_f = 0.1·ln(c^new_f/c_f) + Δide_f. c^new is clout recomputed under the target rules on the current keys, so factions that would lose power oppose. A backlash hazard σ(−5 + 3R) per tick sets π −= 0.1.
2. **Ratification** at π ≥ 1: every institution with `reform` scope must approve (§4.7).
3. **Transition** lasts T = ⌈6 + 12·D⌉ ticks. The parameters apply at the end of the ratifying tick's R5. L −= 0.15·D; age ← age·(1 − min(1, D)); losers get grv = 0.4·ln(c_new/c_old); logits + ln(1 + D) for T ticks.

Failure modes: **stall** (π ≤ 0 after ≥ 6 ticks, or Sup < θ_r for 12 consecutive ticks) and **veto** both end in collapse with L −= 0.03·D and the PC lost. **Backlash** causes setbacks and raised hazards.

**Decree reform** applies immediately: L −= 0.2 + 0.2·D; controllers of institutions that would have blocked get grv −0.2; logits + ln(1 + 2D) for 24 ticks; age := 0.

**Custom governments.** Any `Institutions` value that passes validation (GOV-002) can be a target. The player composes a government parameter by parameter (franchise, ρ, institution weights and vetoes, succession, ethics, CBI). Presets are only starting points.

### 4.10 Outputs to WS-03 [D]

- `invest_sentiment_f` = clamp(A_f − 0.5·R, −1, 1)
- `labor_militancy_f` = max(0, −A_f)·μ_f·u_law, with u_law = 1 / 0.6 / 0.2 for free / regulated / banned unions (banning moves Oppression and raises R*)
- `flight_propensity_f` = clamp(0.5·max(0, −A_f) + 2·p_rev,12m + 0.3·1[transition], 0, 1)·(0.5 + W_oligarchs/0.2·0.5)
- `compliance_f` = clamp(0.5·A_f + 0.5·(2L − 1), −1, 1)·(0.5 + 0.5·s̄_bureaucracy)
- `stat_bias` = 0.3·(1 − accountability), accountability = 0.4·φ_eff + 0.3·(1 − q_C,media) + 0.3·judicial independence, with φ_eff = φ if `Elected`, else 0
- `cbi`, `corruption` = K, band, price-control legality, laws, shocks as in §3.2

### 4.11 Lobbies (link to WS-06)

φ_fj = clamp(φ⁰_fj + 0.8·(0.5 − d_fj) + 0.5·arms_fj·1[military or Industry-controlling f] − 0.8·threat_j, −1, 1). Here d_fj is the normalized distance between f's salient ideals and j's public ethics center, and φ⁰ is setting content. A pro-lobby forms from factions with φ ≥ 0.4 and an anti-lobby from factions with φ ≤ −0.4; a lobby exists if its members' clout ≥ 0.1. Its pressure is Π_j = Σ_members c_f·φ_fj. At most 5 lobbies per polity (largest |Π_j|) [D]. WS-06 reads Π_j, and alignment feeds back through for_f (V3 lobby precedent [V: m1-findings §2.1]).

### 4.12 Macro tier and start state

**Macro.** `GovSummary` carries institutions, laws, L, R, K, Σ, H, age, coalition ideal and ρ for every tier. Σ ← Σ + (Σ* − Σ)/12 + N(0, 0.02), where Σ* = clamp(0.2 + 3·(g − ḡ) − 2·(π − π̄) + 0.4·(L − 0.5) − 0.5·1[war] − 0.3·K, −0.95, 0.95) [D]. L, R and K follow §4.6 with Σ in place of the faction sum (R* uses 0.8·max(0, −Σ)). The coup hazard uses Λ := Σ and κ := `macro_military_share`. Incumbents lose elections if σ(3Σ) + ε < θ. Macro polities reform only through coups and revolutions (OQ-G6 closed).

**Colonial administrations** (WS-08a subjects) run macro only. A successful revolution emits `IndependenceDeclared`; WS-06/WS-08a change the relation, and the polity adopts its content `independence_constitution`.

**Tier switch.** Demotion copies the summary. Promotion builds factions from the content template and keys from WS-03 `disaggregate`. It then adds one uniform shift Δ to every U_f, found by bisection, so that Σ_f c_f·tanh((U_f + Δ)/0.10) equals the macro Σ within 1e-9, and forms C greedily. L, R, K, Σ and H are conserved within 1e-6; institutions and laws exactly.

**Start state (WS-08a §3.5: WS-04 is primary owner of `government` and `stability`).** `archetype_map.ron` maps each of the 8 `Government` variants to exactly one preset. `stability` bands VeryLow…VeryHigh map to L₀ ∈ {0.20, 0.35, 0.50, 0.65, 0.80} and Σ₀ ∈ {−0.4, −0.2, 0, 0.2, 0.4} [D]. Σ₀ is applied by the same uniform-shift solve. A preset's content may add per-polity overrides in WS-04's own content (WLD-021).

### 4.13 Tracked variables and RNG

**Tracked:** per faction: clout, vote clout, coercion, U (all terms), A, Λ. Per institution: weight, control shares, s̄_i. Per polity: eight axes, L, R, K, P, Σ, ρ, S_people, W_people, coalition clout, coup probability per coercive faction, revolution probability, reform progress, lobby pressures, the four behavior outputs per faction. That is ≈ 160 `Tracked` values per high-fidelity polity with ≈ 10 terms each ≈ 65 KB [D], inside the WS-02 budget.

**RNG:** tag `"gov"`, entity = polity ID; purposes: election noise, succession, coup (base + faction index), coup resolution, revolution, revolution resolution, reform backlash, macro Σ noise.

## 5. Requirements

| ID | Requirement | Acceptance criterion (testable) | Trace |
|---|---|---|---|
| **Institutions & government type** |||
| GOV-001 | A government type is the §4.1 parameter set, loaded from content; presets are data. | A content-only government type not among the presets loads and runs 12 ticks; CI grep finds no preset or institution ID in `crates/*/src/`. | P-6, SYS-007 |
| GOV-002 | Institution parameters are validated for range and consistency, all errors in one pass. | Fixture with exactly 5 errors (ρ = 0; institution weights sum 0.9; legitimacy weights sum 0.9; `Elected` with franchise `None`; a veto scope on a zero-weight institution) reports exactly those 5, naming file and field. | P-6, ARC-051 |
| GOV-003 | Any valid parameter set is playable. | Property test: 1,000 seeded random valid `Institutions` each run 120 ticks on the test scenario with the econ stub: no panic, every gov `f64` finite, Σ_f c_f = 1 ± 1e-9 and Σ_f q_fi = 1 ± 1e-9 every tick. | SYS-001, R-2 |
| GOV-004 | S_people, W_people and ρ are derived and displayed. | 3-key fixture: values equal closed forms within 1e-12 for each franchise variant. | P-3 |
| GOV-005 | `cbi` changes only through reform, coup or revolution, and reaches WS-03 the next tick. | No other order changes it; after a ratified reform the econ stub sees the target value at t + 1. | §3.2 |
| GOV-006 | Institution control follows the §4.2 rules, including law overrides. | Fixture with one faction per rule: q matches hand values within 1e-12; switching the press law from free to censored makes media control equal coalition control; K = 0.5 mixes bureaucracy 50/50. | P-8, SYS-004 |
| GOV-007 | Clout is Σ_i W_i·q_fi, with executive control computed in a second pass. | Fixture: c⁻, executive shares and c match the formulas within 1e-12; removing a faction from C moves exactly W_exec of clout. | P-3 |
| GOV-008 | Institutions with no active controllers spread their weight pro rata. | Fixture with no clergy-tagged faction: the clergy weight is spread and Σ c_f = 1 within 1e-12. | R-2 |
| **Factions** |||
| GOV-010 | Factions, membership selectors and activation conditions are content. | Adding a faction file adds a faction with no code change; a dormant faction is zero-membership until the first tick its condition holds. | P-6, P-8 |
| GOV-011 | Membership allows split shares, conserves headcount, and is published to WS-03. | Property test: m_fk ∈ [0, 1], Σ_f m_fk ≤ 1, Σ_f h_f + apolitical = 1 within 1e-12; `FactionEconBehavior.membership` equals m_fk exactly. | P-8, WS-03 §2.4 |
| GOV-012 | Navy and Aerospace activate and deactivate on personnel share with hysteresis. | Stub personnel ramp: Navy appears the first tick its share ≥ 0.15, persists down to 0.10, disappears below; its members return to Military; headcount is conserved throughout. | OQ-G2 |
| GOV-013 | Power responds to the economy's distribution. | Under `oligarchic_republic`, raising the stub's top-decile wealth share 0.3 → 0.5 strictly increases the oligarchs-weighted clout of the top-bin faction; with W_oligarchs = 0 it changes only through income-based bases. | P-8 |
| GOV-014 | Coercion comes from WS-11 personnel × readiness × weight. | Fixture: κ_f equals the hand computation within 1e-12; Σ κ_f = 1 when any personnel > 0. | §3.3 |
| GOV-015 | Ideal drift and Unity pull are bounded and follow §4.2/§4.8. | A faction held at A = −0.5 for 240 ticks moves monotonically away at δ·0.5 per tick until clamped; with Unity intensity 3 and m = 1, a faction at A = 0 moves toward the coalition ideal at 0.003 per tick; all ideals stay in [−1, 1]. | P-3 |
| GOV-016 | Each faction has 2–3 salient axes. | Validation rejects a faction with 1 or 4 salient axes; ideology terms are nonzero only on salient axes. | Sean v0.2 |
| **Policy space** |||
| GOV-020 | Eight content-defined axes computed as p_a = tanh(Σ M(x − x⁰)). | Fixture map: each axis equals the closed form within 1e-12; axis count and names come from `axes.ron`. | P-6 |
| GOV-021 | Every instrument (budget line, tax and regulation value, law option, Spoils, institution parameter) has a map row and a channel. | Fixture missing one row and one channel reports exactly 2 errors naming them. | P-8 |
| GOV-022 | Axis breakdowns have one term per contributing instrument. | ARC-041 tolerance via the squash rule; every nonzero M_ax·(x − x⁰) appears, labeled. | SYS-004 |
| **Utility & selectorate** |||
| GOV-030 | U_f is the sum of the §4.4 terms, each a labeled breakdown term. | Hand fixture: every term and the total match within 1e-12. | SYS-004 |
| GOV-031 | The material term comes from `EconOutcomes` and decomposes exactly. | Stub raising one key's eff_tax_rate changes mat only for factions with core weight on that key; the "tax" term equals the core-weighted change; Σ terms = mat within 1e-12. | P-8, WS-03 §2.4 |
| GOV-032 | Expectations adapt as specified. | A permanent public-goods step gives pũb at 360 months = 0.3 × the initial jump within 1%. | R-2 |
| GOV-033 | Rents dilute with W and reach only the coalition. | Doubling ρ halves prv_f for every coalition faction within 1e-12; non-coalition prv = 0; rents never appear in mat. | m1 §2.2, §3.2 |
| GOV-034 | Loyalty secures only (1 − ρ) of private goods. | Fixture with every other term zero (ω = 0, b = 0): atanh(Λ_f)·0.10 = (1 − ρ)·prṽ_f within 1e-12. | m1 §2.2 |
| GOV-035 | The rents/public-goods trade-off emerges by regime. | At each preset's default start, add 0.1% of GDP as rents (via K) vs. as public spending. Survival support (non-electoral: Σ_C c_f·s_f; electoral: expected V) rises more from rents for `military_junta`, `single_party`, `personalist`, `absolute_monarchy`, `oligarchic_republic`, and more from public goods for `parliamentary_republic`, `presidential_republic`, `constitutional_monarchy`. | m1 §2.2, P-8 |
| GOV-036 | A, Λ, axes and probabilities use the squash rule. | ARC-041 tolerance for each after 120 ticks; term signs equal the underlying term signs. | SYS-004 |
| **Coalition, elections, succession** |||
| GOV-040 | Coalition formation is validated (θ on c⁻, Λ ≥ −0.1, PC). | `FormCoalition` failing each check gives a rejection event and no state change; a valid one costs exactly the PC in §4.7. | ARC-022 |
| GOV-041 | Members leave after 3 ticks below −0.3. | Departure event at tick 3, not before; a recovery at tick 2 resets the count. | P-3 |
| GOV-042 | Minority governments trigger the §4.5 consequences. | Electoral: early election 3 ticks after falling below θ. Non-electoral: logits + ln 3 and PC income 0 until re-formed. | §4.5 |
| GOV-043 | Elections follow §4.5 and form opposition coalitions deterministically. | With σ_ε = 0 (test override): V matches within 1e-12; new C equals the hand-built greedy result; φ = 0 never loses; ties by content ID. | P-4 |
| GOV-044 | Unhappier electorates remove incumbents more often. | Batch 500 runs × 4 elections at mean approval −0.3, 0, +0.3: P(loss) strictly decreasing, adjacent levels > 3 SE apart. | §4.5 |
| GOV-045 | Succession events occur at 1/μ with the window multiplier. | 10,000 seeded draws: mean interval within 3 SE of μ; logits in the window exceed baseline by ln(window_mult) exactly. | §4.5 |
| **Legitimacy, unrest, corruption, crises** |||
| GOV-050 | Legitimacy is a stock driven by the weighted sources. | Each single source weight = 1 converges to D_src within 1e-3 after 240 ticks; `stock` breakdown with one flow per source plus events and Unity. | SYS-004 |
| GOV-051 | Unrest is a stock; repression works only through loyal security. | ψ_law 0.6 with s̄_security = 1 scales R* by 0.4 within 1e-12; with s̄_security = 0.5, by 0.7; the repression law changes p_Opp by its map entry. | §4.6 |
| GOV-052 | Corruption moves toward max(target, floor) and is exported. | Fixture: K follows the stock law within 1e-12; a target below the floor converges to the floor; raising enforcement lowers the floor per formula; WS-03 stub sees K at t + 1. | §3.2, §4.6 |
| GOV-053 | Coup and revolution hazards are logistic in the §4.6 terms, displayed as monthly probabilities with breakdowns. | Analytic fixture matches each logit within 1e-12; 100,000 draws at p = 0.02 give a frequency within 3 SE. | SYS-004 |
| GOV-054 | Coup outcomes follow §4.6. | Forced success (test override P = 1): plotter preset applied, L = 0.25, H + 1, shock emitted; forced failure: purge multipliers and grievance for 24 ticks. | §4.6 |
| GOV-055 | Revolution outcomes follow §4.6, including colonial independence. | Forced success: argmax faction's preset; shock; for a colonial macro polity, `IndependenceDeclared` emitted and the independence preset applied; forced failure: R halved, L − 0.1. | §4.6, §4.12 |
| GOV-056 | Past coups raise coup risk. | States differing only in H = 0 vs. 2 differ in coup logit by exactly 1.0. | §4.6 |
| GOV-057 | Baseline crisis rates fall in the §7.2 bands. | Batch per preset (200 runs × 1,260 ticks, neutral policy, econ stub): attempted coups and revolutions per polity-year and mean regime duration in band; coup success share in [0.35, 0.65]. | R-1, R-2 |
| GOV-058 | Governance dynamics stay bounded. | Same batch: all finite; share of run-months with R > 0.9 < 5% per preset; no faction's A pinned at ±0.99 for > 120 consecutive ticks in > 5% of runs; K never pinned at 1 under neutral policy in > 5% of runs. | R-2, ARC-023 |
| GOV-059 | At most one irregular regime change per polity per tick, in a fixed order. | Coup and revolution logits both +20: exactly one change (coups first, content-ID order). | P-4 |
| **Gating, PC, Unity** |||
| GOV-060 | PC accrues and is spent per §4.7; overdrafts are blocked. | Income matches within 1e-12; two orders affordable alone but not together: the second is `NoPoliticalCapital`. | §3.4 |
| GOV-061 | Ethics bounds policy. | An order moving p_Mil outside the box → `Unconstitutional{Military}`; a forbidden instrument (`price_controls`) → rejected and exported as illegal. | §4.7 |
| GOV-062 | Institutional vetoes follow §4.7; overrides need `decree_powers`. | ΔU = −τ_i − 0.01 is blocked naming i; with `AuthorizeOverride` and `decree_powers` it passes and charges PC and L; without `decree_powers` it stays blocked; λ_i = 1 vs. 0.3 changes the outcome in a fixture where coalition loyalty is pivotal. | §4.7 |
| GOV-063 | More vetoing institutions means fewer accepted changes. | 10,000 random proposals at the test start with 0–3 vetoing institutions: acceptance rate non-increasing. | §4.7 |
| GOV-064 | WS-03 and WS-04 apply identical gates. | For 1,000 random order lists, the set WS-03 rejects equals the set `gate_orders` blocks, and PC charged equals Σ `pc_cost` of allowed orders. | P-4, §3.4 |
| GOV-065 | `EnactBudget` is scored with `econ::project`; a block keeps last year's budget. | Stub `project` returning a 5% income cut for the legislature's majority faction blocks the draft; the WS-03 stub keeps the old schedule (ECO-050 path). | WS-03 §2.5 |
| GOV-066 | The discretion band scales with executive weight. | W_exec = 0.1, 0.2, 0.4, 0.8 → band 1, 2, 4, 4 pp; out-of-band `AdjustTax` → `OutOfBand`. | WS-03 OQ-E3 |
| GOV-067 | The Unity program follows §4.8. | Intensity 2 with censored media (m = 1): L* + 0.1, R* × 0.8, PC −4/month; with free press (q_C,media = 0): m = 0.3. | Sean v0.2 |
| **Reform & custom government** |||
| GOV-070 | A reform is a target `Institutions` with distance D and PC cost. | D matches the hand value within 1e-12; proposal charges 10·D; invalid target → `InvalidTarget`. | §4.9 |
| GOV-071 | Deliberation uses support under the target rules. | c^new equals clout recomputed under the target; π increments match within 1e-12; halving a faction's clout gives it ΔU^ref = 0.1·ln 0.5 + Δide. | §4.9, P-8 |
| GOV-072 | Reforms fail by stall or veto with §4.9 penalties. | Each failure collapses at the specified tick; L − 0.03·D; PC not refunded. | §4.9 |
| GOV-073 | Ratified reforms apply with transition effects. | Parameters change at the end of the ratifying R5; L − 0.15·D; + ln(1 + D) for exactly T ticks; loser grievance as specified. | §4.9 |
| GOV-074 | Decree reforms apply at once with the extraconstitutional penalties. | Same-tick change; L − (0.2 + 0.2·D); age = 0; + ln(1 + 2D) for 24 ticks. | §4.9 |
| GOV-075 | A player can build a custom government from parameters. | End-to-end: an input log proposes a target equal to no preset (property franchise bin ≥ 16, ρ = 0.4, institution weights exec .2 / leg .3 / jud .1 / bur .1 / sec .1 / med .1 / ind .1, legislature budget veto, `Elected{60, 0.9}`, cbi 0.9). With support forced above θ_r it ratifies, runs 60 ticks, saves, loads and continues to a hash identical to an uninterrupted run. | SYS-002, P-6 |
| GOV-076 | Reform success falls with distance and vetoing institutions. | Batch: success non-increasing in D bins (0.25, 0.5, 1, 2) and reform-veto count (0–3). | §4.9 |
| **Lobbies** |||
| GOV-080 | Affinities and lobby formation follow §4.11. | φ within 1e-12; a 0.09-clout bloc forms no lobby, 0.10 does; ≤ 5 retained, ranked by \|Π\|. | §4.11 |
| GOV-081 | Lobby pressure reaches WS-06 and alignment feeds back. | Π_j = Σ c_f·φ_fj; stub alignment 0 → +1 changes each member's for_f by 0.03·φ_fj. | §3.3, P-8 |
| **Interfaces** |||
| GOV-085 | WS-03 inputs are read same-tick. | A tax order at tick t changes mat at tick t. | §3.1 |
| GOV-086 | WS-04 outputs to WS-03 match §3.2 and §4.10 and are read next tick. | Each output equals its formula within 1e-12 and stays in range over 120 ticks; the econ stub sees tick-t values at t + 1. | WS-03 §2.4, ECO-090 |
| GOV-087 | WS-11 exchange per §3.3. | Coercion uses previous-tick personnel; conscription, nuclear posture and `coercion_loyalty` appear in outputs read by the R5b stub the same tick. | §3.1 |
| GOV-088 | WS-06 exchange per §3.3. | `GovSummary` is reachable through `PolityView` for another polity; truth stability and legitimacy are not (compile-fail doc test); `public_noise_sigma` matches its formula. | ARC-012, P-2 |
| GOV-089 | WS-08a start fields map completely. | Validation fails if any `Government` variant lacks a preset in `archetype_map.ron`; loading `date_1955` gives every polity its mapped preset, L₀ per band, and Σ = Σ₀ within 1e-9 at tick 0. | WS-08a §3.5, WLD-022 |
| **Macro tier** |||
| GOV-090 | Macro polities run the §4.12 reduced form. | A macro polity is finite for 1,260 ticks; Σ* matches within 1e-12 at fixture inputs. | P-7, SYS-006 |
| GOV-091 | Tier switches conserve the governance state. | Promote then demote: L, R, K, Σ, H within 1e-6 (absolute at 0); institutions and laws equal. | SYS-006, ARC-081 |
| GOV-092 | Macro and high-fidelity crisis rates agree. | Batch per preset: macro annual coup and revolution rates within a factor 1.5 of high-fidelity rates from the same start. | P-7 |
| **Cross-cutting** |||
| GOV-095 | Governance is deterministic and draws only from §4.13 streams. | Two 120-tick runs give identical gov hashes every tick; adding a faction to polity A leaves polity B bit-identical; ARC-033 never fires. | SYS-002, P-4 |
| GOV-096 | Every §4.13 variable is listed and explainable. | `list_vars` contains each; ARC-041 tolerance after 120 ticks. | SYS-004 |
| GOV-097 | R5 fits its budget. | `orchestra bench` on the ARC-090 scenario with 12 factions × 9 institutions per high-fidelity polity: R5 median ≤ 10 ms (CI 20 ms). | SYS-008, ARC-091 |
| GOV-098 | No faction, axis, institution, law, preset or status group is hard-coded. | CI grep for every governance content ID in `crates/*/src/` returns nothing. | P-6, ARC-010 |

## 6. Content/data

```
content/core/gov/axes.ron           [Axis { id, name, neg_label, pos_label }]                      // 8
content/core/gov/policy_map.ron     [Row { instrument, channel, value_weight?, x0, terms: [(axis, M)] }]
content/core/gov/institutions.ron   [Institution { id, name, default_rule: ControlRule, effects }]  // 9
content/core/gov/laws.ron           [LawGroup { id, options: [LawOption { id, effects, rule_overrides }] }]
content/core/gov/factions.ron       [Faction { id, affinity: [KeySelector], tags, salient: [(axis, ideal, weight)],
                                               beta_public, mobilization, line_interest, war_weight, baseline,
                                               activation?, coup_constitution, revolution_constitution }]
content/core/gov/gov_types.ron      [Institutions { id, ... §4.1 ... }]
content/core/gov/archetype_map.ron  [(Government variant, gov_type id)]; stability band → (L0, Sigma0)
content/core/gov/params.ron         every [D] constant in §4
content/settings/<s>/gov_start.ron  optional per-polity overrides: coalition, laws, K, H, age, lobby seeds φ⁰,
                                    independence_constitution (subjects)
```

### 6.1 Presets [D]

Institution weights (exec, leg, jud, bur, sec, med, ind, oli, clg); **bold** = has a veto scope.

| Preset | WS-08a variant | Franchise | ρ | Weights | Vetoes | Succession | Legitimacy (el, perf, trad, ideo) | cbi | Reference [R] |
|---|---|---|---|---|---|---|---|---|---|
| `parliamentary_republic` | ParliamentaryRepublic | Universal | 0.5 | .15 **.30** **.10** .10 .05 .10 .10 .05 .05 | leg budget/law λ 1; jud reform/law | Elected 48, φ 1 | .5 .3 .1 .1 | 0.7 | postwar W. Germany |
| `presidential_republic` | PresidentialRepublic | Universal | 0.5 | .25 **.25** **.10** .10 .05 .10 .10 .05 0 | leg budget/law λ 0.3; jud reform/law | Elected 48, φ 1 | .5 .3 .1 .1 | 0.7 | U.S. |
| `constitutional_monarchy` | ConstitutionalMonarchy | Universal | 0.5 | .10 **.30** **.10** .10 .05 .10 .10 .05 .10 | leg budget/law λ 1; jud reform | Elected 48, φ 1 | .4 .3 .2 .1 | 0.6 | UK, Sweden |
| `absolute_monarchy` | AbsoluteMonarchy | Restricted (elite bins ≥ 19) | 0.2 | .35 0 .05 .10 .20 .05 .05 .10 .10 | none; decree | Hereditary | 0 .2 .6 .2 | 0 | Saudi Arabia |
| `single_party` | SingleParty | Restricted (party: Public role) | 0.02 | .25 .05 .05 **.25** .20 .10 .10 0 0 | bur reform | Appointed μ 120, ×3 | 0 .4 .2 .4 | 0 | USSR |
| `personalist` | Personalist | Restricted (elite) | 0.02 | .40 0 0 .10 .25 .10 .05 .10 0 | none; decree | Appointed μ 180, ×5 | 0 .5 .1 .4 | 0 | Trujillo's DR |
| `military_junta` | MilitaryJunta | Restricted (officers) | 0.1 | .20 0 .05 .15 **.45** .05 .05 .05 0 | sec budget/law/reform | Appointed μ 72, ×3 | 0 .6 .1 .3 | 0.2 | Argentina 1976 |
| `colonial_administration` | ColonialAdministration | None | 1 | .40 0 .05 .25 .30 0 0 0 0 | none | Appointed μ 60 | 0 .5 .5 0 | 0 | macro only |
| `hegemonic_party` | — (reform/revolution target) | Universal | 0.05 | .25 .15 .05 **.20** .15 .10 .10 0 0 | bur reform | Elected 72, φ 0.3 | .2 .4 .2 .2 | 0.2 | Mexico (PRI) |
| `oligarchic_republic` | — | Property (bin ≥ 18) | 0.5 | .15 **.25** .05 .10 .05 .05 .15 **.20** 0 | leg budget/law λ 0.5; oli budget | Elected 48, φ 0.7 | .3 .3 .3 .1 | 0.5 | Prussian three-class era |
| `theocracy` | — | Universal | 0.1 | .20 .10 .10 .10 .15 .05 .05 .05 **.15** | clg law/reform | Appointed μ 120, ×3 | .1 .2 .3 .4 | 0.1 | Iran post-1979 |

### 6.2 Default factions [D] (2–3 salient axes each; ideal in [−1, 1])

| Faction | Members (EconKey selectors) | Salient axes: ideal | Real-world family [R] |
|---|---|---|---|
| Landowners | Capital, agriculture, bins ≥ 15 | Welfare −0.6, Knowledge −0.5, Oppression +0.3 | landed conservatives |
| Rural Masses | Wage, agriculture, bins ≤ 10 | Welfare +0.5, Economy +0.4, Knowledge −0.3 | agrarian/peasant parties |
| Industrialists | Capital, industry/defense/space | Economy −0.6, Welfare −0.5, Government +0.2 | business liberals |
| Organized Labor | Wage, industry/defense, bins 5–15 | Welfare +0.7, Economy +0.5, Oppression −0.4 | trade unions, social democrats |
| Professionals | Wage, services, bins ≥ 12 | Oppression −0.6, Government +0.5, Economy −0.3 | urban liberals |
| Technocrats | Wage, space/defense + services bins ≥ 15 (tag: research) | Knowledge +0.7, Space +0.6, Government +0.3 | scientific-technical intelligentsia |
| Traditionalists (tag: clergy) | All roles, weighted to agriculture and bins ≤ 12 | Knowledge −0.7, Oppression +0.3, Welfare +0.2 | religious conservatives |
| State Apparatus | Public, non-military, + internal security | Government −0.5, Oppression +0.6, Economy +0.4 | nomenklatura, bureaucratic elites |
| Military | Public, army + inactive services | Military +0.7, Oppression +0.3, Nuclear +0.2 | officer corps |
| Navy (dormant < 15%) | Public, navy | Military +0.6, Nuclear +0.3 | naval establishment |
| Aerospace (dormant < 15%) | Public, air + space forces | Military +0.5, Space +0.5, Nuclear +0.5 | air and space forces |
| Disarmament Movement (dormant until a nuclear test anywhere) | Wage, services, bins 8–18 | Nuclear −0.8, Military −0.4 | peace movements (UK's CND, 1958) |

### 6.3 Laws (MVP) [D]

Conscription (none / selective / universal) · Press (free / restricted / censored) · Assembly & labor (free unions / regulated / banned) · Internal security (civilian police / gendarmerie / secret police) · Civil-military relations (civilian control / autonomous / praetorian) · Judicial independence (independent / captured) · Religion & state (secular / established / theocratic) · Nuclear posture (no program / minimal deterrence / assured destruction) · Pension age (55 / 60 / 65) · Women's status (restricted / partial / equal) · Price controls (banned / permitted). State ownership, regulation and capital controls are WS-03 `SetRegulation` instruments, not laws.

## 7. Verification

### 7.1 Tests and stubs
- Tests named `gov_NNN_*` on the WS-02 test scenario plus a governance fixture (3 polities: parliamentary, junta, single-party; 12 factions; 9 institutions).
- Stubs: an `econ` stub implementing the WS-03 §2.4 types (`EconKey`, `KeyOutcome` with `wealth`, rents, budget lines, a linear `project`), plus `force` and `diplo` stubs. The Python prototype uses the same fields.
- Batch tests (GOV-044, 057, 058, 063, 076, 092) run in CI at 50 runs and in full nightly (WS-10).

### 7.2 Calibration bands (GOV-057) [D, anchored on [V]]

Anchors: 457 coup attempts worldwide 1950–2010, about half successful [V: [Wikipedia citing Powell & Thyne](https://wikipedia.com/wiki/Coup)]. Over roughly 8,500 country-years [R], that is about 5% per country-year, concentrated in a minority of states. Military regimes are the least durable type [V: [Geddes, Frantz & Wright 2014](https://bpb-us-e1.wpmucdn.com/sites.psu.edu/dist/0/12570/files/2014/06/GeddesFrantzWrightAR.pdf)].

| Preset (neutral policy, stub growth 3% ± 2%) | Coup attempts / polity-year | Revolutions / polity-year | Mean regime duration |
|---|---|---|---|
| parliamentary, presidential, constitutional monarchy | < 0.3% | < 0.1% | > 60 years |
| single_party, hegemonic_party | 0.5–3% | 0.1–0.5% | 20–50 years |
| absolute_monarchy, oligarchic_republic, theocracy, personalist | 1–5% | 0.2–1% | 10–40 years |
| military_junta | 4–12% | 0.3–1.5% | 5–15 years |

### 7.3 Performance
Estimated R5 cost per high-fidelity polity: 12 factions × ~240 keys membership, 9 institution rules, ~50 instruments × 8 axes, hazards ≈ 5·10⁴ flops ≪ 1 ms [D]. GOV-097 uses the WS-02 budget (10 ms).

## 8. Open questions

| ID | Question | Default if unanswered |
|---|---|---|
| OQ-G1 | Player-picked coalition? | **Closed (2026-11-27): yes** |
| OQ-G2 | Services as factions | **Closed: one Military faction, plus Navy and Aerospace (air + space) when their service is large** (§4.2) |
| OQ-G3 | PC only for laws, coalitions, reforms, overrides, Unity, supplementals | **Closed: yes** |
| OQ-G4 | The player always keeps the state | **Closed: yes** |
| OQ-G5 | WS-03 contract | **Closed: WS-04 conforms to WS-03 §2.4**; remaining asks moved to OQ-G7 |
| OQ-G6 | Macro polities reform only by coup or revolution | **Closed: yes** |
| OQ-G7 | WS-03 edits requested (§3.2): `wealth` in `KeyOutcome`; CBI owned by WS-04 (drop or demote WS-03's `cbi_default`); rents never added to key incomes | **Accepted (2026-11-27).** Wealth and the rents note are done in WS-03 (this PR); CBI ownership remains for WS-03 v0.2 |
| OQ-G8 | Space axis poles: earthbound/expansionist (recommended) vs. extraction/ecocentric | **Closed (2026-11-27): earthbound/expansionist**; revisit when WS-08b adds resources |
| OQ-G9 | WS-11 must expose a strategic (nuclear) share of defense spending for the Nuclear axis | **Accepted (2026-11-27)**; a WS-11 kickoff input. Until then the Nuclear axis reads only the posture law |
