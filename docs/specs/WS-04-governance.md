# WS-04 — Governance & Factions
Status: Draft v0.1 · Parent: ConOps §3, §5 (P-3, P-4, P-6, P-7, P-8), §6 governance/factions/services rows, SYS-002, SYS-004, SYS-006, SYS-007, SYS-008 · Depends on: WS-02 v1.0 · Last updated: 2026-11-27

Tags: **[V]** verified (source linked) · **[R]** recalled, not checked · **[D]** design choice (to be tuned in the M2(c) prototype).
Prototype plan: `prototypes/governance/PLAN.md`. WS-03 has no spec yet, so §3.2 is the **proposed** WS-03 contract written from this side; the WS-03 kickoff must ratify it or propose edits (OQ-G5).

---

## 1. Purpose & scope

WS-04 turns the economy's population into political actors and turns institutions into numbers the player can change. Each tick it computes who has power (clout, coercion), how satisfied each faction is with policy and outcomes (approval, loyalty), whether the government keeps its coalition, and whether that ends in an election loss, a coup or a revolution. It also gates policy changes (veto players, ethics, political capital) and runs reform of the government itself as a project.

**In scope (MVP):** government type as a parameter set; data-driven factions over WS-03 cells and status groups; a six-axis policy space; selectorate mechanics (public vs. private goods, loyalty, challenger risk); legitimacy and unrest; four consequences (election/coalition turnover, coup, revolution, unrest's economic drag); political capital; veto players and ethics; reform as a staged project, including composing a custom government; foreign-policy lobbies (link to WS-06); macro-tier reduced form.

**Out of scope:** characters, leaders and dynasties (succession is an abstract hazard); parties and detailed elections (factions vote as blocs); civil war and secession; freeform scripted events; foreign subversion of factions (post-MVP, WS-06); player-authored laws (laws are content).

**Player stance.** The player always steers the state. Losing an election or suffering a coup changes the coalition and institutions the player must govern through; it never ends the game (ConOps: sandbox) [D].

## 2. Decisions

| # | Decision | Recommendation |
|---|---|---|
| 1 | Policy space | **Six axes**: five derived from instruments (Redistribution, Intervention, Security, Order, Modernization) and one from institutions (Inclusion). Every instrument maps onto axes through one content table; material effects bypass the axes and come straight from WS-03 incidence. |
| 2 | Faction set | **Data-driven schema with a default archetype set (12) in content**, plus dormant factions with activation conditions and bounded ideal-point drift. No runtime-spawned factions. |
| 3 | Stability & crises | **Three stocks (legitimacy, unrest, political capital) plus logistic monthly hazards** for coup and revolution; coalition loss and elections are threshold rules. The displayed *stability index* is a summary, not a driver. |
| 4 | Reform | **A reform is a diff of institution parameters**: Deliberation (progress ∝ clout-weighted support under the *new* rules) → Ratification (reform veto players) → Transition (legitimacy dip, raised hazards). Failure modes: stall, veto, backlash. An extraconstitutional decree path exists at a legitimacy and coup-risk price. |
| 5 | WS-03 contract | WS-04 reads per-cell, per-instrument incidence **in the same tick** (R5 after R4). WS-03 reads WS-04 outputs **next tick** (central bank independence, patronage targets, unrest drag, political risk, behavior modifiers, crisis shocks). §3.2. |

### 2.1 Options and trade-offs (condensed)

**D1 — Policy space.** (a) V3-style law groups only: legible, but utility then needs a hand-authored opinion of every faction on every law (≈12 × 30 entries per faction set), and budgets and taxes don't fit. (b) A 2-D political compass: cheap, but it collapses militarism and authority, which are separate mechanics here (defense budget vs. repression and coup-proofing). (c) **A few derived axes + direct material incidence:** every instrument contributes to axes through one sparse matrix, so adding a law or budget line is one row of data; *who gets paid* comes from WS-03's incidence, so the axes only carry ideology, not money (no double counting, P-8). → (c) with six axes, each tied to a mechanic:

| Axis | − pole / + pole | Instruments feeding it (default map) | Mechanic it drives |
|---|---|---|---|
| Redistribution (R) | market / egalitarian | tax progressivity index, transfers/GDP, wealth & estate tax rates | Labor/Rural vs. capital factions; WS-03 incidence tools |
| Intervention (I) | laissez-faire / dirigiste | subsidies/GDP, mean tariff, property law, −central bank independence | Industrialists/Agrarians; WS-03 tariffs, subsidies, monetary |
| Security (S) | dovish / militarist | defense/GDP, conscription law | Service factions; WS-11 budget and manpower |
| Order (O) | liberty / authority | press, assembly, internal-security and civil-military laws; internal security/GDP | Repression of unrest, coup-proofing, intel opacity (WS-06 noise) |
| Modernization (M) | tradition / modernization | R&D/GDP, space/GDP, education/GDP, religion-and-state law | Technocrats vs. Traditionalists; WS-05 funding |
| Inclusion (N) | exclusive / inclusive | institutions: selectorate share, coalition share, election fairness, vote weight share | Reform evaluation; ideological legitimacy |

A separate *openness* axis was dropped: foreign orientation is per target polity, so it lives in lobbies (§4.9), and tariffs already sit on Intervention.

**D2 — Factions.** (a) Fixed archetypes in code: violates P-6. (b) Fully emergent (cluster the population each tick): illegible, unstable identities, untestable. (c) **Content-defined archetypes, membership computed from WS-03 cells each tick, dormant factions that activate on conditions (e.g., a Space Force faction once the service exists), and ideal points that drift when a faction is unhappy.** Clout shifts emerge from the economy (industrialization grows Labor), radicalization emerges from drift, and the set stays legible. → (c). Services are separate factions because the ConOps lists them as interest groups competing for budget share; merging them is a content edit (OQ-G2).

**D3 — Stability.** (a) One stability scalar driving everything (V3 legitimacy/radicals): legible, but it can't distinguish a junta about to be couped from a democracy about to lose an election. (b) Agent-threshold cascades (Granovetter) for revolution: realistic tipping, but noisy and hard to explain. (c) **Stocks plus per-actor logistic hazards**: each crisis has its own drivers, every driver is a logit term in a breakdown ("coup risk 3.1%/month: army disloyal +2.4, unrest +0.8, …"), and base rates can be calibrated. → (c).

**D4 — Reform.** (a) Instant change with a PC cost: no politics. (b) V3-style monthly pass chance: a slot machine, illegible. (c) **Staged project with deterministic progress and a few stochastic setbacks**, where support is evaluated under the proposed rules (factions that would lose clout oppose). This makes "extend the franchise" (UK Reform Acts, 1832–1884 [R]) or a regime passing its own dissolution through existing veto players (Spain's 1976 Political Reform Law [R]) playable, and the decree path covers a self-coup (Peru 1992 [R]). → (c).

**D5 — S and W.** The selectorate S and winning coalition W could be derived from political strength (effective number of people with power), but that ties loyalty to wealth concentration in ways players can't predict, and it can't represent a hegemonic party's large nominal S. → **S comes from the franchise rule, W is a parameter** (`coalition_share` = W/S = ρ). Political-strength weights decide *which* factions the coalition must include; ρ decides how cheaply private goods buy their loyalty. Both are visible sliders when composing a government.

## 3. Interfaces

### 3.1 Stage placement and lags

| What | Stage | Reads | Lag |
|---|---|---|---|
| Order gating (policy, law, coalition, reform) | R1 | stage-start world + all of this tick's orders | none |
| Factions, approval, coalition, elections, hazards, reform progress, lobbies | R5 | R4-committed `econ` (this tick); `force`, WS-06 relations (previous tick) | economy → politics same tick; forces/diplomacy → politics 1 tick |
| WS-03 consumes WS-04 outputs | R2–R4 | committed `gov` | 1 tick |
| WS-11 consumes conscription, coercion-relevant loyalty | R5b | R5-committed `gov` | same tick |
| WS-06 consumes summaries, lobbies, crisis events | D2, R6 | committed `gov` (previous tick for D2) | D2: 1 tick; R6: same tick |

### 3.2 WS-03 contract (proposed; both directions)

**WS-03 → WS-04** (read in R5 from R4 results):

| Field | Meaning | Unit |
|---|---|---|
| `cells[c] = {quantile, sector, pop, wealth, income: [(component, value)]}` | One cell per (income quantile, sector). `income` components are market income, each tax instrument (negative), each transfer line, and `patronage`; they sum to post-tax income | persons; money/person/month |
| `budget[line]` | Spending per budget line | money/month |
| `instruments[i]` | Every tax/tariff/subsidy parameter in policy-map units (e.g., progressivity index, mean tariff) | as declared in `policy_map.ron` |
| `gdp`, `population`, `wealth_mean` | Totals | money/month, persons, money/person |
| `macro = {growth_12m, inflation_12m, unemployment, debt_gdp, spread}` | For performance legitimacy and expectations | fractions |
| `public_employment[source]` (optional) | Headcount for status groups sourced from WS-03 | persons |

**WS-04 → WS-03** (read next tick):

| Field | Range | WS-03 use |
|---|---|---|
| `cb_independence` | [0, 1] | Weight on the Taylor rule; player rate orders accepted only where WS-03's rule allows |
| `patronage_targets: [(cell, share)]` | shares ≥ 0, Σ = 1 | Distributes the `patronage` budget line as a transfer component named `patronage` |
| `unrest_output_factor` | [1 − u_y, 1] | Multiplies output (strikes, disorder) |
| `political_risk` | [0, 1] | Credibility term in the bond yield |
| `behavior[name]` | per `behavior.ron` bounds | Investment, labor and R&D-talent multipliers by sector (P-8); R&D talent goes to WS-05 |
| `shocks: [Shock {kind, capital_loss, output_loss, decay_months}]` | losses in [0, 0.5] | Applied in R2 next tick (coup, revolution) |

WS-04 excludes the `patronage` income component from the material term and counts it as private goods instead (§4.4), so patronage is never counted twice.

### 3.3 WS-06, WS-11, WS-05 contracts

| Direction | Field | Notes |
|---|---|---|
| WS-06 → WS-04 | `alignment[j]` ∈ [−1, 1] | The polity's current stance toward polity j (ally +1 … war −1) |
| WS-06 → WS-04 | `war_burden` ≥ 0, `war_started_tick` | Casualties per 1,000 population over 12 months; rally term |
| WS-06 → WS-04 | `threat[j]` ∈ [0, 1] | From the polity's beliefs (P-2), never truth |
| WS-04 → WS-06 | `GovSummary` (public): institutions, ethics, regime age, coalition ideal, crisis events | Public information, readable through `PolityView` |
| WS-04 → WS-06 | `stability`, `legitimacy` (truth) + `public_noise_sigma` | WS-06 intel pushes `Observation`s; closed societies are noisier: σ = 0.05 + 0.20·(1 + p_O)/2 [D] |
| WS-04 → WS-06 | `lobbies: [(target j, pressure Π_j, members)]` | Domestic pressure on foreign policy (§4.9) |
| WS-11 → WS-04 | `personnel[service]`, `readiness[service]`, `security_personnel` | Status-group headcounts and coercion (previous tick) |
| WS-11 → WS-04 | `service_budget_share[service]` | Service rivalry term |
| WS-04 → WS-11 | `law.conscription` option, `coercion_loyalty[service]` ∈ [−1, 1] | Manpower policy; which units follow plotters in a coup (WS-11 decides how units react) |
| WS-04 → WS-05 | `behavior[rnd_talent]` | R&D efficiency multiplier from Technocrats' approval (via `behavior.ron`) |
| WS-05 → WS-04 | `arms_dependency[j]` ∈ [0, 1] | Share of the polity's in-service equipment value with provenance j (lobby input) |

### 3.4 Orders (`OrderKind` variants added by WS-04)

`SetLaw { group, option }` · `FormCoalition { factions }` · `SetPatronageAllocation { shares }` · `AuthorizeOverride { veto_player }` · `ProposeReform { target: Institutions }` · `CancelReform` · `DecreeReform { target: Institutions }`.

Budget, tax and monetary orders stay WS-03's. They are gated by one pure function both subsystems call in R1:

```rust
pub fn gate_orders(world: &World, orders: &[Order]) -> Vec<Gate>;   // one Gate per order, in log order
pub enum Gate { Allowed { pc_cost: f64 }, Blocked { reason: BlockReason } }
pub enum BlockReason { Unconstitutional { axis: AxisIdx }, Veto { player: VetoIdx, overridable: bool },
                       NoPoliticalCapital, InvalidTarget(String) }
```

Political-capital costs accumulate in log order within the tick, so an order that would overdraw PC is blocked even if each order alone is affordable. WS-03's `apply_order` rejects `Blocked` orders as events; WS-04's R1 function charges PC for `Allowed` ones. Because both call the same function on the same inputs, they always agree (GOV-064).

### 3.5 Block layout

```rust
pub struct GovBlock {
    pub inst: Institutions,              // all tiers; public
    pub laws: Vec<LawOption>,            // all tiers; indexed by law group
    pub summary: GovSummary,             // all tiers: legitimacy, unrest, stability, rho, regime_age,
                                         // coup_history, coalition_ideal[6], crisis flags (Tracked)
    pub out: GovOutputs,                 // all tiers: §3.2 / §3.3 outputs
    pub detail: Option<Box<GovDetail>>,  // High only: factions, coalition, PC, reform, EMAs, lobbies
}
```

No WS-02 `Aggregates` change is needed: `GovSummary` is present for every tier, so WS-06 compares regimes across tiers through it.

## 4. Model

All quantities are per polity per monthly tick. Indices: c cell, f faction, k axis, i instrument, g status group, j foreign polity.

### 4.1 Institutions (the government parameter set)

| Parameter | Type / range | Meaning |
|---|---|---|
| `franchise` | `Universal` · `Property { min_quantile }` · `Status { groups }` · `None` | Who has formal voice; gives v_c ∈ [0, 1] per cell and S_people |
| `coalition_share` ρ = W/S | (0, 1] | Winning coalition as a share of the selectorate |
| `strength` | w_v, w_w, w_s ≥ 0 (not all 0); α ∈ [0.5, 2] | Political strength weights on votes, wealth, status (V3 precedent) |
| `status_grants` | group → [0, 1] | Status the constitution grants (aristocracy, clergy, officers, party) |
| `coalition_threshold` θ | [0.3, 0.8] | Clout share a coalition needs (0.5 majority) |
| `succession` | `Elected { period_months, fairness φ ∈ [0,1] }` · `Hereditary` · `Appointed { council }` | How leaders change |
| `legitimacy_weights` | simplex over {electoral, performance, traditional, ideological} | Legitimacy source |
| `ethics` | center e ∈ [−1,1]^5, half-widths h ∈ (0, 2]^5, forbidden law options | Constitutional policy box |
| `veto_players` | list of `VetoPlayer { scope, controllers, tolerance τ_v, override }` | §4.7 |
| `cb_independence` | [0, 1] | Consumed by WS-03 |
| `decree_powers` | bool | Whether veto overrides are constitutional |
| `amendment_threshold` θ_r | [0.5, 0.8] | Support needed for reform progress |

Derived: S_people = Σ_c N_c·v_c (status-franchise cells count their members); W_people = ρ·S_people. Presets in content (§6.1) are named parameter sets; a custom government is any valid set.

### 4.2 Population, membership, clout, coercion

**Cells.** WS-03 cells plus status-group cells. Each status group g (content) has a headcount from a source (`force.personnel[service]`, `security_personnel`, `public_employment`, or a constant population share), a source sector and a quantile profile. Its headcount is removed from those base cells in proportion to the profile, so Σ_c N_c = population.

**Political strength** per capita: π_c = w_v·v_c + w_w·(wealth_c / wealth_mean)^α + w_s·σ_c, where σ_c is the status grant (0 for ordinary cells). Strength PS_c = N_c·π_c. Vote strength uses only the first term.

**Membership.** Content gives each faction an affinity a_fc ∈ [0, 1] (product of quantile-band, sector and status-group affinities), zero for dormant factions. Shares m_fc = a_fc / max(1, Σ_f a_fc); the remainder of a cell is apolitical. A cell can split across factions (V3 precedent [V: m1-findings §2.1]).

| Faction quantity | Formula |
|---|---|
| Clout c_f | Σ_c m_fc·PS_c / Σ_f Σ_c m_fc·PS_c |
| Vote clout c_f^v | same with vote strength |
| Population share h_f | Σ_c m_fc·N_c / population |
| Reference income y_f | strength-weighted mean post-tax income of members (the politically relevant core) |
| Coercion κ_f | Σ_g m_fg·K_g / Σ_g K_g, with K_g = N_g·readiness_g·coercion_weight_g (content) |

Example: a Prussian-style three-class franchise [R] is `Property`-like voting with high w_w; an oligarchic republic is `Property { min_quantile: 0.9 }` with w_w = 1.

**Ideal-point drift.** If A_f < 0, f's ideal moves away from current policy: i_fk ← clamp(i_fk + δ·(−A_f)·sign(i_fk − p_k), −1, 1), δ = 0.002/month [D]. If A_f > 0.3, it relaxes toward its content default at δ/2. This is the only source of radicalization.

### 4.3 Policy axes

p_k = tanh( Σ_i M_ki·(x_i − x_i⁰) ), over instruments x (budget lines as shares of GDP, WS-03 instrument values, law options one-hot, institution parameters for axis N). M, x⁰ and the scale are content (`policy_map.ron`). Every instrument also carries one **channel** tag: `public` (value weight v_l), `transfer` or `sectoral` (effect arrives via WS-03 incidence), `patronage`, or `none`. Validation fails if any instrument lacks a map row or channel (GOV-021).

### 4.4 Utility, approval, loyalty (selectorate mechanics)

All terms are in units of a fraction of the faction's reference income per month. Ē[·] is an exponential moving average with time constant τ_E.

| Term | Formula | Default |
|---|---|---|
| Material | mat_f = Σ_j (y_f^j − Ē[y_f^j]) / Σ_j Ē[y_f^j] over income components j except `patronage` | τ_E = 36 months [D]; fully adaptive (only changes matter) |
| Public goods | pub_f = β_f·G_pc / y_f, G_pc = Σ_{l∈public} v_l·spend_l / population | β_f content, default 2 [D] |
| Private goods | prv_f = 1[f∈C]·P·a_f / (W_people·κ_f^C·y_f) | P = patronage line; a_f allocation (default κ_f^C, f's share of coalition clout) |
| Adapted level | for X ∈ {pub, prv}: X̃_f = X_f − κ·Ē[X_f] | κ = 0.7, τ_E = 36 [D]: a permanent change keeps 30% of its effect |
| Ideology | ide_f = −Σ_k ω_fk·((p_k − i_fk)² − r²) | ω_fk content (default 0.02), r = 0.5 [D] |
| Line interest | lin_f = Σ_l λ_fl·ln(spend_l / Ē[spend_l]) | e.g., Navy on its budget share; λ content |
| Foreign | for_f = ω_for·Σ_{j: f in a lobby on j} φ_fj·alignment_j | ω_for = 0.03 [D] |
| War | war_f = −ω_war,f·war_burden + 0.05·exp(−months at war / 6) | ω_war content [D]; short rally [R] |
| Grievance | grv_f: event-driven (purge, clout lost in reform), decays with τ = 24 months | §4.6, §4.8 |
| Baseline | b_f | content |

U_f = mat_f + pũb_f + prṽ_f + ide_f + lin_f + for_f + war_f + grv_f + b_f.

**Approval** A_f = tanh(U_f / u_s), u_s = 0.10 [D]. **Loyalty** (coalition members only; Λ_f = A_f otherwise):
Λ_f = tanh((U_f − ρ·prṽ_f) / u_s).

The ρ discount is the selectorate mechanic as a game rule (m1-findings §2.2 [V]): a challenger can promise the same private goods, and a defector has probability ρ of being in the next coalition, so only (1 − ρ) of private goods is *secured* by staying loyal. Small-ρ regimes buy loyalty cheaply with patronage; large-ρ regimes must deliver public goods and policy. With equal incomes, patronage beats public goods per unit of spending for coalition loyalty when (1 − ρ)/(β·ρ·S_share) > 1. With defaults this holds for every autocratic preset (ratio ≈ 5–450 at the default S shares: junta 1%, one-party 6%, oligarchy 10%) and fails for liberal democracy (≈ 0.77). In electoral systems public goods also win votes from non-coalition factions, which widens the gap (GOV-035).

**Support** s_f = (1 + Λ_f)/2. **Stability index** (display and macro tier) Σ = Σ_f c_f·A_f ∈ (−1, 1).

**Breakdowns.** U_f is a plain sum. A_f, Λ_f and every probability are monotone transforms; their breakdown uses the *squash rule*: for y = g(x), x = x₀ + Σ t_i, term_i = t_i·(g(x) − g(x₀))/(x − x₀) (0 if x = x₀), plus a base term g(x₀). Terms sum exactly to y and keep their sign and labels.

### 4.5 Coalition, elections, succession

- **Formation.** C is valid if Σ_{f∈C} c_f ≥ θ. `FormCoalition` requires every joining faction to have Λ_f ≥ Λ_join = −0.1 (evaluated as if joined) and costs PC. In electoral systems the electorate forms C at elections; between elections the player may re-form it.
- **Departure.** A member with Λ_f < Λ_leave = −0.3 for 3 consecutive ticks leaves (event). Deterministic.
- **Minority.** If Σ_{f∈C} c_f < θ: electoral systems hold an early election 3 ticks later unless C is re-formed; other systems enter *regime crisis* (coup and revolution logits + ln 3; PC income 0) until re-formed.
- **Elections** (`Elected`): every `period_months`, or early. Each faction's bloc votes for the incumbents with π_f = σ(k_e·A_f + b_e·(2·1[f∈C] − 1)), k_e = 3, b_e = 0.5 [D]. V = Σ_f c_f^v·π_f + ε, ε ~ N(0, 0.04) [D]. Reported V_rep = φ·V + (1 − φ)·max(V, θ + 0.1). If V_rep < θ the opposition forms C: seed with the faction with the largest c_f^v·(1 − π_f), then add factions in order of ideal-point distance to the growing coalition's clout-weighted ideal (ties by content ID) until Σ c_f ≥ θ. φ < 1 models hegemonic parties (Mexico's PRI [R]) and costs electoral legitimacy.
- **Succession** (`Hereditary`, `Appointed`): a leader-change event occurs with hazard 1/μ per month (μ = 300 hereditary, 120 appointed by a party, 72 by a junta [D]; juntas are least durable [V: [Geddes, Frantz & Wright 2014](https://bpb-us-e1.wpmucdn.com/sites.psu.edu/dist/0/12570/files/2014/06/GeddesFrantzWrightAR.pdf)]). The event opens a 3-tick succession window: coup logits + ln 3, and coalition members with Λ_f < 0 leave. Single-party and monarchic regimes institutionalize succession; military and personalist ones don't [V: [Frantz](https://bpb-us-e1.wpmucdn.com/sites.psu.edu/dist/0/12570/files/2014/06/FrantzCPL.pdf)], which μ and the window encode.

### 4.6 Legitimacy, unrest, crises

**Legitimacy** (stock, [0, 1]): L ← L + (L* − L)/24 + event jumps. L* = Σ_src w_src·D_src:
- D_elec = φ·√S_share if `Elected` and the last election was on schedule, else 0
- D_perf = σ(4·(g − ḡ) − 3·(π − π̄) − 2·(u − ū)), with ḡ, π̄, ū EMAs (τ = 60) of WS-03 growth, inflation, unemployment
- D_trad = 1 − exp(−age/240), age = months since the last irregular change
- D_ideo = 1 − Σ_f c_f·‖i_f − e‖ / (2√5), distance over the five instrument axes

**Unrest** (stock, [0, 1]): R ← R + (R* − R)/6, R* = (1 − ψ)·(1 − 0.5·L)·Σ_f h_f·μ_f·max(0, −A_f), clamped to [0, 1]. μ_f is mobilization (content). Repression ψ = min(0.6, ψ_law + 0.1·(internal security/GDP)/0.01) [D]. Repression laws also move Order, so liberal factions' approval and ideological legitimacy fall automatically. Effect on WS-03: `unrest_output_factor` = 1 − u_y·R, u_y = 0.10 [D].

**Coup hazard** for each faction with κ_f > 0 (logit per month):
ℓ_f = −8.5 + 3·(−Λ_f) + 2·κ_f + 2·R + 2·(1 − L) + 0.5·H − 2·χ + crisis terms
H is past coups (each decays with τ = 120 months; the "coup trap" [V: [Wikipedia, citing the coup-trap literature](https://wikipedia.com/wiki/Coup)]); χ ∈ {0, 0.5, 1} is the civil-military law (praetorian, autonomous, civilian control). *Crisis terms* (coup and revolution logits alike) are the sum of active additive shifts: regime crisis + ln 3, succession window + ln 3 (coups only), reform transition + ln(1 + D), decree aftermath + ln(1 + 2D). Factions are evaluated in content-ID order; at most one coup per tick.

**Coup resolution.** Plotters = the initiating faction plus every coercive faction with Λ < 0. P_success = K_p^1.5 / (K_p^1.5 + (K_l + 0.3·L)^1.5) with K_p, K_l the plotters' and loyalists' coercion shares (the 0.3·L term is civilian resistance, e.g., the failed Soviet 1991 coup [R]). Calibration target: about half of attempts succeed [V: 457 attempts 1950–2010, about half successful, [Wikipedia citing Powell & Thyne](https://wikipedia.com/wiki/Coup)].
- Success → institutions := the plotter faction's `coup_constitution` preset (default `military_junta`); C re-formed from plotters plus factions with Λ > 0; L := 0.25; H += 1; age := 0; shock {output_loss 0.02}.
- Failure → plotters get grv −0.3 and coercion × 0.5 for 24 ticks (purge); H += 1; L += 0.02 (failed coups tend to strengthen autocrats [V: same source]).

**Revolution hazard:** ℓ_rev = −9 + 6·R + 2·(1 − L) − 3·κ_loyal + crisis terms, κ_loyal = coercion share with Λ ≥ 0. Success P = R / (R + κ_loyal + 0.1).
- Success → the revolutionary faction is argmax_f h_f·μ_f·max(0, −A_f); institutions := its `revolution_constitution` preset; status grants of the old preset's ruling groups → 0; C formed from factions nearest its ideal; L := 0.4; age := 0; shock {capital_loss 0.10, output_loss 0.08, decay 12} [D].
- Failure → crackdown: R × 0.5, L − 0.1, grv −0.2 for factions with A < 0.

Coups and revolutions are forced reforms (§4.8): they skip deliberation.

### 4.7 Political capital, ethics, veto players

**Political capital** (stock, [0, 100]): income = 4·(Σ_{f∈C} c_f·s_f / θ)·(0.5 + L) per month [D]. Costs [D]: law change 10 per option step; `FormCoalition` 15; `ProposeReform` 10·D plus 1 per month in deliberation; `AuthorizeOverride` 20 (and L − 0.05). Each cost × (1 + 0.25·n_veto). Budget and tax changes cost no PC: they are gated only by ethics and veto players, and they move approval directly.

**Ethics (ruling class constraint).** A policy or law order is *unconstitutional* if it would put any instrument axis k outside [e_k − h_k, e_k + h_k], or select a forbidden law option. Changing ethics is a reform. The ruling class is whoever the constitution grants status and whoever controls veto players.

**Veto players** (Tsebelis-style). `controllers` ∈ {`coalition` (clout of C), `vote_clout`, `wealth_clout` (w_w only), `coercion`, `factions[...]`}; `scope` ⊆ {budget, tax, law, monetary, reform}. A player approves a change iff ΔU_v = Σ_f w_vf·Δide_f ≥ −τ_v, where Δide_f is the ideology-term change of the proposed axes (no WS-03 counterfactual needed); for reform scope ΔU_v uses ΔU_f^ref (§4.8). A block names the veto player. Override needs `decree_powers` (constitutional, PC + legitimacy) or is unavailable.

### 4.8 Reform

A reform target is a full `Institutions` value; its distance is D = Σ_j weight_j·|z_j^new − z_j| / range_j (content weights; categorical changes count 1·weight).

1. **Deliberation.** Progress π starts at 0; each tick π += 0.08·(Sup − θ_r)/max(D, 0.25), where Sup = Σ_f c_f·σ(3·ΔU_f^ref/u_s) and ΔU_f^ref = 0.1·ln(c_f^new/c_f) + Δide_f (including axis N). c_f^new is clout recomputed on current cells under the target rules, so factions that would lose power oppose. A backlash hazard σ(−5 + 3R) per tick sets π −= 0.1.
2. **Ratification** at π ≥ 1: every reform-scope veto player must approve (overridable only with `decree_powers`).
3. **Transition** (T = ⌈6 + 12·D⌉ ticks): the new parameters apply at the end of the ratifying tick's R5; L −= 0.15·D; age ← age·(1 − min(1, D)); factions whose clout fell get grv = 0.2·ln(c_new/c_old) (doubled for loss aversion); coup and revolution logits + ln(1 + D) during T.

**Failure modes.** Stall (π ≤ 0 after ≥ 6 ticks, or Sup < θ_r for 12 consecutive ticks) → collapse, L −= 0.03·D, PC lost. Veto at ratification → collapse, same penalty. Backlash → setbacks and higher hazards.

**Decree reform** (extraconstitutional, always available): applies at once; L −= 0.2 + 0.2·D; factions controlling veto players that would have blocked get grv −0.2; coup logits + ln(1 + 2D) for 24 ticks; age := 0.

**Custom governments.** Any `Institutions` value that passes validation (GOV-002) can be the target, so the player composes a government parameter by parameter (e.g., a property franchise with high coalition share and an independent central bank). Presets are starting points only.

### 4.9 Lobbies (link to WS-06)

Affinity of faction f toward foreign polity j: φ_fj = clamp(φ⁰_fj + 0.8·(0.5 − d_fj) + 0.5·arms_fj·1[coercive or defense-sector f] − 0.8·threat_j, −1, 1), where d_fj is the normalized distance between f's ideal and j's public ethics center and φ⁰ is setting content (history, diaspora). Pro-lobby on j = factions with φ_fj ≥ 0.4; anti-lobby = φ_fj ≤ −0.4; a lobby exists if its members' clout ≥ 0.1. Pressure Π_j = Σ_{members} c_f·φ_fj. At most 5 lobbies per polity (largest |Π_j|) [D]. WS-06 reads Π_j as domestic pressure; the polity's alignment toward j feeds back through for_f (V3 precedent [V: m1-findings §2.1]).

### 4.10 Macro tier

`GovSummary` carries the macro state for every tier: institutions, laws, L, R, Σ, H, age, coalition ideal, ρ. Macro dynamics (no factions): Σ ← Σ + (Σ* − Σ)/12 + N(0, 0.02), Σ* = clamp(0.2 + 3·(g − ḡ) − 2·(π − π̄) + 0.4·(L − 0.5) − 0.5·1[war], −0.95, 0.95) [D]. L and R use §4.6 with Σ in place of the faction sum (R* uses max(0, −Σ)·0.8). Coup hazard uses §4.6 with Λ := Σ and κ := the preset's `macro_military_share`. Elections: incumbents lose if σ(3Σ) + ε < θ. Macro polities reform only through coups and revolutions in the MVP.

**Tier switch.** Demotion: summary already holds the state. Promotion: build factions from the polity's content template, cells from WS-03 `disaggregate`, then add one uniform shift Δ to every U_f, found by bisection, so that Σ_f c_f·tanh((U_f + Δ)/u_s) equals the macro Σ within 1e-9; then form C greedily. Conserved within 1e-6: L, R, Σ, H, institutions (exact), laws (exact).

### 4.11 Tracked variables

Per faction: clout, vote clout, coercion share, U (all terms), A, Λ. Per polity: six axes, L, R, PC, Σ, ρ, S_people, W_people, coalition clout, coup probability per coercive faction, revolution probability, reform progress π, lobby pressures. ≈ 100 `Tracked` per high-fidelity polity with ≈ 10 terms each ≈ 40 KB per polity [D], inside the WS-02 memory budget.

### 4.12 RNG streams

Tag `"gov"`, entity = polity ID, purposes: election noise, succession, coup (base + faction index), coup resolution, revolution, revolution resolution, reform backlash, macro Σ noise. No other randomness.

## 5. Requirements

| ID | Requirement | Acceptance criterion (testable) | Trace |
|---|---|---|---|
| **Institutions** |||
| GOV-001 | A government type is the §4.1 parameter set, loaded from content; presets are data. | Test: a content-only government type not among the presets loads and runs 12 ticks; CI grep finds no preset ID in `crates/*/src/`. | P-6, SYS-007 |
| GOV-002 | Institution parameters are validated for range and consistency, all errors in one pass. | Fixture with exactly 4 errors (ρ = 0; all strength weights 0; legitimacy weights sum 0.9; `Elected` with franchise `None`) reports exactly those 4, each naming file and field. | P-6, ARC-051 |
| GOV-003 | Any valid parameter set is playable. | Property test: 1,000 seeded random valid `Institutions` each run 120 ticks on the test scenario with the econ stub: no panic, every gov `f64` finite, Σ c_f = 1 ± 1e-9 every tick. | SYS-001, R-2 |
| GOV-004 | S_people, W_people and ρ are derived as in §4.1 and displayed. | 3-cell fixture: values equal hand-computed closed forms within 1e-12 for each franchise variant. | P-3 |
| GOV-005 | `cb_independence` is an institution parameter exported to WS-03 and changes only through reform, coup or revolution. | Test: no order other than reform paths changes it; after a ratified reform the next tick's WS-03 input equals the target value. | §3.2 |
| **Factions & power** |||
| GOV-010 | Factions, status groups and membership rules are content; dormant factions activate on content conditions. | Test: adding a faction file adds a faction with no code change; a fixture faction with condition "space personnel > 0" has zero membership until the first tick the condition holds and nonzero from that tick. | P-6, P-8 |
| GOV-011 | Membership allows split shares and conserves headcount. | Property test over random affinities: m_fc ∈ [0, 1], Σ_f m_fc ≤ 1, and Σ_f h_f + apolitical share = 1 within 1e-12; status-group carving keeps Σ_c N_c = population within 1e-9 relative. | P-8 |
| GOV-012 | Clout follows the §4.2 strength formula. | 2-cell fixture (poor majority, rich minority): clout under universal-vote weights and under w_w = 1, α = 1 each match closed form within 1e-12. | P-8, SYS-004 |
| GOV-013 | Clout responds to the economy's distribution. | Test: with a wealth-weighted preset, raising the econ stub's top-decile wealth share from 0.3 to 0.5 strictly increases the clout of a faction whose affinity is the top decile; with w_w = 0 it doesn't change. | P-8 |
| GOV-014 | Coercion shares come from WS-11 personnel × readiness × content weight. | Fixture with stub force values: κ_f equals the hand computation within 1e-12 and Σ κ_f = 1 when any K_g > 0. | §3.3 |
| GOV-015 | Ideal-point drift is bounded and follows §4.2. | Test: a faction held at A = −0.5 for 240 ticks moves its distance from policy monotonically up, at δ·0.5 per tick until clamped, and every i_fk stays in [−1, 1]. | P-3 |
| **Policy space** |||
| GOV-020 | Six content-defined axes computed as p_k = tanh(Σ M(x − x⁰)). | Fixture map: each axis equals the closed form within 1e-12; axis count and names come from `axes.ron`. | P-6 |
| GOV-021 | Every instrument (budget line, tax instrument, law option, institution parameter) has a policy-map row and a channel tag. | Fixture missing one row and one channel reports exactly 2 errors naming them. | P-8 |
| GOV-022 | Axis breakdowns have one term per contributing instrument. | ARC-041 sum tolerance holds for every axis via the squash rule; each nonzero M_ki·(x_i − x_i⁰) appears as a term labeled with instrument i. | SYS-004 |
| **Utility, approval, selectorate** |||
| GOV-030 | U_f is the sum of the §4.4 terms, each a labeled breakdown term. | Hand fixture: every term and the total match hand values within 1e-12. | SYS-004, P-3 |
| GOV-031 | The material term comes from WS-03 per-instrument incidence and excludes patronage. | Test: raising one tax's incidence on a cell by Δ changes mat_f only for factions with members in that cell, by −m-weighted Δ / Σ Ē within 1e-12, labeled with that tax; changing the `patronage` component leaves mat unchanged. | P-8, §3.2 |
| GOV-032 | Expectations adapt as specified. | Step test: a permanent public-goods step gives pũb at 360 months (10·τ_E) = 0.3 × the initial jump within 1%; a permanent income step gives mat → 0 within 1% by 10·τ_E. | R-2 |
| GOV-033 | Private goods dilute with W and reach only the coalition. | Test: doubling ρ (so W_people doubles) halves prv_f for every coalition faction within 1e-12; non-coalition prv = 0. | m1 §2.2 |
| GOV-034 | Loyalty discounts secured private goods by ρ. | Fixture with ω = 0, b = 0 and every term but prṽ zero: atanh(Λ_f)·u_s = (1 − ρ)·prṽ_f within 1e-12. | m1 §2.2 |
| GOV-035 | The public/private trade-off emerges by regime. | At the default start state of the test scenario, increase patronage vs. public spending by 0.1% of GDP each. Survival support (non-electoral: Σ_C c_f·s_f; electoral: expected V) rises more from patronage for `military_junta`, `one_party_state`, `absolute_monarchy`, `oligarchic_republic`, and more from public goods for `liberal_democracy`. | m1 §2.2, P-8 |
| GOV-036 | Approval, loyalty and probabilities use the squash rule. | ARC-041 tolerance for every A, Λ and probability after a 120-tick run; term signs equal the signs of the underlying utility or logit terms. | SYS-004 |
| **Coalition, elections, succession** |||
| GOV-040 | Coalition formation is validated (θ, Λ_join, PC). | Tests: `FormCoalition` with Σ c < θ, with a joiner at Λ = −0.2, and with PC < cost each yields a rejection event and no state change; a valid one costs exactly the §4.7 PC. | §3.4, ARC-022 |
| GOV-041 | Members leave after 3 ticks below Λ_leave. | Fixture: Λ = −0.35 for ticks 1–3 → departure event at tick 3, not before; a recovery at tick 2 resets the count. | P-3 |
| GOV-042 | Minority governments trigger the §4.5 consequences. | Electoral fixture: early election 3 ticks after falling below θ; non-electoral fixture: logits increase by ln 3 and PC income = 0 until re-formed. | §4.5 |
| GOV-043 | Elections follow §4.5 and form opposition coalitions deterministically. | With σ_ε = 0 (test override): V matches closed form within 1e-12; the new coalition equals the hand-built greedy result; φ = 0 never loses; ties resolve by content ID. | P-4 |
| GOV-044 | Unhappier electorates remove incumbents more often. | Batch, 500 runs × 4 elections at mean approval −0.3, 0, +0.3: P(incumbent loss) strictly decreasing, adjacent levels separated by > 3 standard errors. | §4.5 |
| GOV-045 | Non-electoral succession events occur at rate 1/μ with a 3-tick window. | 10,000 seeded draws: mean interval within 3 SE of μ; logits during the window exceed the baseline by ln 3 exactly. | §4.5 |
| **Legitimacy, unrest, crises** |||
| GOV-050 | Legitimacy is a stock driven by the weighted sources. | Fixtures with each single source weight = 1 converge to that D_src within 1e-3 after 240 ticks; breakdown is `stock` form with one flow per source plus events. | SYS-004 |
| GOV-051 | Unrest is a stock; repression lowers R* and moves Order. | Fixture: raising ψ from 0 to 0.6 scales R* by 0.4 within 1e-12; the repression law changes p_O by its map entry. | §4.6 |
| GOV-052 | Coup and revolution hazards are logistic in the §4.6 terms, displayed as monthly probabilities with breakdowns. | Analytic fixture matches each logit within 1e-12; 100,000 seeded draws at p = 0.02 give a frequency within 3 SE. | SYS-004, P-4 |
| GOV-053 | Coup outcomes follow §4.6. | Forced-success fixture (no loyal coercion, test override P = 1): institutions = plotter preset, L = 0.25, H + 1, shock emitted for WS-03; forced failure: purge multipliers and grv applied for 24 ticks. | §4.6 |
| GOV-054 | Revolution outcomes follow §4.6. | Forced-success fixture: revolutionary faction = the argmax; its preset applied; old ruling status grants = 0; shock emitted; forced failure: R halved, L − 0.1. | §4.6 |
| GOV-055 | Past coups raise coup risk (coup trap). | Test: identical states differing only in H = 0 vs. H = 2 differ in coup logit by exactly 1.0. | §4.6 |
| GOV-056 | Baseline crisis rates fall in the calibration bands of §7.2. | Batch per preset (200 runs × 1,260 ticks, neutral scripted policy, econ stub): attempted-coup and revolution rates per polity-year and mean regime duration inside each band; coup success share in [0.35, 0.65]. | R-1, R-2 |
| GOV-057 | Governance dynamics stay bounded (R-2). | Same batch: every `f64` finite every tick; share of run-months with R > 0.9 < 5% for every preset; no faction's A pinned at ±0.99 for > 120 consecutive ticks in > 5% of runs. | R-2, ARC-023 |
| GOV-058 | At most one irregular regime change per polity per tick, evaluated in a fixed order. | Fixture where both coup and revolution logits are +20: exactly one change occurs (coups first, in content-ID order, then revolution). | P-4 |
| **Gating** |||
| GOV-060 | Political capital accrues and is spent as in §4.7; overdrafts are blocked. | Tests: income matches the formula within 1e-12; two orders each affordable alone but not together: the second is blocked with `NoPoliticalCapital`. | §3.4 |
| GOV-061 | Ethics bounds policy. | Order moving p_S outside the box → rejected `Unconstitutional{S}` and no state change; forbidden law option → rejected. | §4.7 |
| GOV-062 | Veto players block per their rule; overrides need `decree_powers`. | Fixture: a change with ΔU_v = −τ_v − 0.01 is blocked naming v; with `AuthorizeOverride` in the same tick and `decree_powers` it passes and charges PC and L; without `decree_powers` it stays blocked. | §4.7 |
| GOV-063 | More veto players means fewer accepted changes. | Batch: 10,000 random policy proposals at the test start state with 0, 1, 2, 3 veto players: acceptance rate non-increasing. | §4.7 |
| GOV-064 | WS-03 and WS-04 apply identical gates. | Property test: for 1,000 random order lists, the set WS-03 rejects equals the set `gate_orders` blocks, and PC charged equals Σ `pc_cost` of allowed orders. | P-4, §3.4 |
| **Reform & custom government** |||
| GOV-070 | A reform is a target `Institutions` with distance D and PC cost. | Fixture: D matches hand value within 1e-12; proposal charges 10·D; invalid target → `InvalidTarget` rejection. | §4.8 |
| GOV-071 | Deliberation progress uses support under the target rules. | Fixture: c_f^new equals clout recomputed under the target; π increments match the formula within 1e-12; a reform that halves a faction's clout gives that faction ΔU^ref = 0.1·ln 0.5 plus its Δide. | §4.8, P-8 |
| GOV-072 | Reforms fail by stall or veto with the §4.8 penalties. | Fixtures for each failure: collapse at the specified tick, L − 0.03·D, PC not refunded. | §4.8 |
| GOV-073 | Ratified reforms apply with transition effects. | Fixture: parameters change at the end of the ratifying tick's R5; L − 0.15·D; logit increase ln(1 + D) for exactly T ticks; loser grievance as specified. | §4.8 |
| GOV-074 | Decree reforms apply at once with the extraconstitutional penalties. | Fixture: parameters change the same tick; L − (0.2 + 0.2·D); age = 0; logit increase ln(1 + 2D) for 24 ticks. | §4.8 |
| GOV-075 | A player can build a custom government from parameters. | End-to-end test: an input log proposes a target equal to no preset (property franchise q ≥ 0.8, ρ = 0.4, w = (1, 0.5, 0), `Elected{60, 0.9}`, cbi = 0.9), support is forced above θ_r, it ratifies, runs 60 more ticks, saves, loads and continues to a hash identical to an uninterrupted run. | SYS-002, P-6 |
| GOV-076 | Reform success falls with distance and veto players. | Batch: success rate non-increasing in D bins (0.25, 0.5, 1, 2) and in reform veto count (0–3). | §4.8 |
| **Lobbies** |||
| GOV-080 | Affinities and lobby formation follow §4.9. | Fixture: φ matches the formula within 1e-12; a 0.09-clout bloc forms no lobby, a 0.10 bloc does; at most 5 lobbies retained, ranked by \|Π\|. | §4.9 |
| GOV-081 | Lobby pressure reaches WS-06 and alignment feeds back. | Test: Π_j in `out.lobbies` equals Σ c_f·φ_fj; setting the stub WS-06 alignment toward j from 0 to +1 changes each member's for_f by ω_for·φ_fj. | §3.3, P-8 |
| **Interfaces** |||
| GOV-085 | WS-03 inputs are read same-tick. | Test: a tax order at tick t changes mat for affected factions at tick t (not t + 1). | §3.1 |
| GOV-086 | WS-04 outputs to WS-03 are bounded and read next tick. | Test: outputs stay within the §3.2 ranges over a 120-tick run; the econ stub sees tick-t values at tick t + 1. | §3.1, R-2 |
| GOV-087 | Behavior modifiers are data-driven. | Test: adding an entry to `behavior.ron` adds an output with no code change; its value = 1 + coefficient × strength-weighted approval of the selected cells, clamped to its bounds. | P-6, P-8 |
| GOV-088 | WS-11 exchange per §3.3. | Test with stub force: coercion uses previous-tick personnel; the conscription option and `coercion_loyalty` appear in outputs read by the R5b stub the same tick. | §3.1 |
| GOV-089 | WS-06 exchange per §3.3. | Test: `GovSummary` is reachable through `PolityView` for another polity; truth stability and legitimacy are not (compile-fail doc test); `public_noise_sigma` matches the formula. | ARC-012, P-2 |
| **Macro tier** |||
| GOV-090 | Macro polities run the §4.10 reduced form. | Test: a macro polity produces finite Σ, L, R and hazards for 1,260 ticks; Σ* matches the formula within 1e-12 at fixture inputs. | P-7, SYS-006 |
| GOV-091 | Tier switches conserve the governance state. | Promote then demote a test polity: L, R, Σ, H within 1e-6 relative (absolute where the value is 0), institutions and laws equal. | SYS-006, ARC-081 |
| GOV-092 | Macro and high-fidelity crisis rates agree. | Batch per preset: annual coup and revolution rates of the macro form are within a factor 1.5 of the high-fidelity rates from the same start state. | P-7 |
| **Cross-cutting** |||
| GOV-095 | Governance is deterministic and draws only from §4.12 streams. | Two 120-tick runs give identical gov hashes every tick; adding a faction to polity A leaves polity B's gov state bit-identical; ARC-033 never fires. | SYS-002, P-4 |
| GOV-096 | Every §4.11 variable is listed and explainable. | `list_vars` contains each; ARC-041 tolerance holds for each after 120 ticks. | SYS-004 |
| GOV-097 | R5 fits its budget. | `orchestra bench` on the ARC-090 scenario with 12 factions per high-fidelity polity: R5 median ≤ 10 ms on the reference machine (CI 20 ms). | SYS-008, ARC-091 |
| GOV-098 | No faction, axis, law, preset or status group is hard-coded. | CI grep for every governance content ID in `crates/*/src/` returns nothing. | P-6, ARC-010 |

## 6. Content/data

```
content/core/gov/axes.ron          [Axis { id, name, neg_label, pos_label }]                  // 6 in default content
content/core/gov/policy_map.ron    [Row { instrument, channel, value_weight?, x0, terms: [(axis, M)] }]
content/core/gov/laws.ron          [LawGroup { id, options: [LawOption { id, effects }] }]
content/core/gov/status_groups.ron [StatusGroup { id, source, sector, quantile_profile, coercion_weight }]
content/core/gov/factions.ron      [Faction { id, affinity: { quantiles, sectors, status }, ideal: [f64; 6],
                                              salience: [f64; 6], beta_public, mobilization, line_interest,
                                              war_weight, baseline, activation?, coup_constitution,
                                              revolution_constitution }]
content/core/gov/gov_types.ron     [Institutions { id, ... §4.1 ..., macro_military_share }]
content/core/gov/behavior.ron      [Modifier { id, target, cells: Selector, coefficient, bounds }]
content/core/gov/params.ron        every [D] constant in §4 (one place to tune)
content/settings/<s>/start.ron     per polity: gov type, laws, coalition, L, H, age, φ⁰ lobby seeds
```

### 6.1 Default presets [D]

| Preset | Franchise | ρ | (w_v, w_w, w_s) | Succession | Legitimacy (elec, perf, trad, ideo) | Veto players | cbi | Examples [R] |
|---|---|---|---|---|---|---|---|---|
| `liberal_democracy` | Universal | 0.5 | (1, 0.1, 0) | Elected 48, φ 1 | .5 .3 .1 .1 | legislature (coalition), court (reform) | 0.8 | postwar W. Europe |
| `presidential_democracy` | Universal | 0.5 | (1, 0.1, 0) | Elected 48, φ 1 | .5 .3 .1 .1 | legislature (vote clout), senate (wealth clout) | 0.7 | U.S., postwar |
| `hegemonic_party` | Universal | 0.05 | (0.3, 0.3, 1) party | Elected 72, φ 0.3 | .2 .4 .2 .2 | party council (reform) | 0.2 | Mexico (PRI) |
| `oligarchic_republic` | Property q ≥ 0.9 | 0.5 | (1, 1, 0.3) | Elected 48, φ 0.7 | .3 .3 .3 .1 | senate (wealth clout) | 0.5 | Prussia's three-class era |
| `one_party_state` | Status: party | 0.02 | (0.05, 0.2, 1) party, security, army | Appointed 120 | 0 .4 .2 .4 | politburo (coalition; reform) | 0 | USSR |
| `military_junta` | Status: officers, elite | 0.1 | (0, 0.3, 1) officers | Appointed 72 | 0 .6 .1 .3 | junta council (coercion; all) | 0.2 | Argentina 1976 |
| `absolute_monarchy` | Status: aristocracy | 0.2 | (0, 0.5, 1) aristocracy, clergy, army | Hereditary | 0 .2 .6 .2 | none (decree) | 0 | Gulf monarchies |
| `constitutional_monarchy` | Universal | 0.5 | (1, 0.1, 0.1) | Elected 48, φ 1 | .4 .3 .2 .1 | legislature, monarch (reform) | 0.7 | UK, Thailand |
| `theocracy` | Universal | 0.1 | (0.3, 0.2, 1) clergy | Appointed 120 | .1 .2 .3 .4 | guardian council (law, reform) | 0.1 | Iran post-1979 |

### 6.2 Default factions [D]

Landowners · Rural Masses · Industrialists · Organized Labor · Professionals · Technocrats · Traditionalists · State Apparatus (bureaucracy, party, internal security) · Army · Navy · Air Force · Space Force (dormant until the service exists). Each has content ideals on the six axes; for example, Labor leans + Redistribution, + Inclusion; Industrialists − Redistribution, mild − Intervention; Army + Security, + Order; Technocrats + Modernization.

### 6.3 Laws (MVP) [D]

Conscription (none / selective / universal) · Press (free / restricted / censored) · Assembly & labor (free unions / regulated / banned) · Internal security (civilian police / gendarmerie / secret police) · Civil-military relations (civilian control / autonomous / praetorian) · Religion & state (secular / established / theocratic) · Property (private / mixed / state).

## 7. Verification

### 7.1 Tests and stubs
- Unit and integration tests named `gov_NNN_*`, one or more per requirement, on the WS-02 test scenario extended with a governance fixture (3 polities: democracy, junta, one-party; 12 factions).
- Stubs: `econ` stub (20 quantiles × 4 sectors, lognormal body + Pareto tail, AR(1) growth, flat and progressive tax instruments) implementing the §3.2 fields; `force` and `diplo` stubs for §3.3. The same stub interface is used in the Python prototype.
- Batch tests (GOV-044, 056, 057, 063, 076, 092) run in CI at reduced counts (50 runs) and in full nightly (WS-10).

### 7.2 Calibration bands (GOV-056) [D, anchored on [V]]

Anchors: 457 coup attempts worldwide 1950–2010, about half successful [V: [Wikipedia citing Powell & Thyne](https://wikipedia.com/wiki/Coup)]. Over roughly 8,500 country-years [R], that is about 5% per country-year on average, concentrated in a minority of states. Military regimes are the least durable regime type [V: [Geddes, Frantz & Wright 2014](https://bpb-us-e1.wpmucdn.com/sites.psu.edu/dist/0/12570/files/2014/06/GeddesFrantzWrightAR.pdf)].

| Preset (neutral policy, stub growth 3% ± 2%) | Coup attempts / polity-year | Revolutions / polity-year | Mean regime duration |
|---|---|---|---|
| `liberal_democracy`, `presidential_democracy`, `constitutional_monarchy` | < 0.3% | < 0.1% | > 60 years |
| `hegemonic_party`, `one_party_state` | 0.5–3% | 0.1–0.5% | 20–50 years |
| `oligarchic_republic`, `absolute_monarchy`, `theocracy` | 1–4% | 0.2–1% | 15–40 years |
| `military_junta` | 4–12% | 0.3–1.5% | 5–15 years |

### 7.3 Performance
Estimated R5 cost per high-fidelity polity: 12 factions × ~100 cells membership + ~40 instruments × 6 axes + hazards ≈ 10⁴ flops; ≪ 1 ms total [D]. GOV-097 sets the bench threshold at the WS-02 budget (10 ms).

## 8. Open questions

| ID | Question | Default if unanswered |
|---|---|---|
| OQ-G1 | In non-electoral systems, does the player pick the coalition (V3-style appointment), or is it fixed by the regime's ruling groups? | The player picks, subject to θ, Λ_join and PC |
| OQ-G2 | Services: four separate factions (ConOps wording) or one Armed Forces faction? | Four, dormant until each service exists; merging is a content edit |
| OQ-G3 | Political capital as a currency: keep it for laws, coalitions, reforms and overrides only, or also charge for budget and tax changes? | Laws/coalitions/reforms/overrides only; budget and tax changes are free but gated |
| OQ-G4 | Can a coup or revolution change the player's polity into one the player no longer controls (e.g., an AI-run successor)? | No; the player always keeps the state (sandbox) |
| OQ-G5 | WS-03 must ratify §3.2: per-cell per-instrument incidence, a `patronage` budget line and income component, and the next-tick outputs. | Treated as the contract until WS-03 is approved; WS-03 may rename fields by spec revision |
| OQ-G6 | Macro polities reform only through coups and revolutions. Should WS-06 AI also run peaceful reforms for macro polities? | No, in the MVP |
