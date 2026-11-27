# WS-03 — Macroeconomy, Fiscal & Monetary Policy, Demographics
Status: Draft v0.3 · Parent: ConOps §3, §5 (P-1, P-2, P-3, P-7, P-8), §6 economy rows, §8 R-1/R-2, SYS-002, SYS-004, SYS-006, SYS-008 · Depends on: WS-02 v1.1, WS-04 Draft v0.2, WS-08a Draft v0.1 · Last updated: 2026-11-27

**v0.3 changes (2026-11-27):** wealth is now a **dynamic stock tracked for five WID-style groups** (§3.5 wealth dynamics). Each group's change breaks down into saving, returns, asset-price gains, taxes, estates and mobility. The wealth marginal is rebuilt from the group means each year, replacing the hand-written α_w and p0 laws. New instruments: mortgage LTV cap, mortgage-interest deductibility, pension funded share, and a social housing spending line. Measured (offshore excluded) and pension-augmented wealth shares are reported. The panel is now 8,192 households with oversampled tails (measured in `panel_size.py`). New ECO-140 to ECO-147, calibrated to the 1978–2012 US path, UK 1955–1975, and a Netherlands-like vs. Belgium-like pair.

**v0.2 changes (2026-11-27, after WS-04 v0.2):** §3.5 replaces the parametric income bins and the v0.1 linear key-wealth rule with a **synthetic household panel** calibrated to US and Algerian wealth data and the SCF 2022 joint income–wealth profile (new ECO-048, ECO-049; ECO-040, 042, 090 revised). Aggregate household wealth and an asset-price index are now defined. CBI is read only from WS-04 (WS-04 OQ-G7 item 2). WS-03 now cites the WS-08a start-state bands it owns (§5.1, WLD-022), and OQ-E3 is closed by WS-04's discretion-band formula. WS-04's shocks, laws and order gate are added to §2.3/§2.5.

Tags: **[V]** checked against the linked source this session · **[R]** recalled, not checked · **[D]** design choice. Every [R] value is a calibration starting point that the M2(a) prototype must confirm or replace.
Contract: WS-02 v1.0 (stages R2–R4, `Tracked`/`Breakdown`, keyed RNG streams, `Tiered`, `PolityView`). Nothing here changes WS-02.

---

## 0. Design intent: the TNO feel, made causal

Reference: a player-written TNO economy guide ([Steam guide 3111917450](https://steamcommunity.com/sharedfiles/filedetails/?id=3111917450); Steam lists it as removed, and it gives almost no numbers). Sean's stated target is slow-building pressure from debt and the credit rating, inflation eating real growth, and pressure from trade relations. Hyperinflation and critical debt should exist, austerity and bailouts should be soft and continuous, and the model should go deeper and broader than TNO.

| TNO element (guide) | Orchestra mechanism | Where |
|---|---|---|
| Real growth = nominal growth − inflation; "negative real growth" crisis | Real growth comes from the production function and the demand gap. Inflation above a threshold reduces TFP growth, so inflation literally eats growth. | §3.2, §3.4 |
| Credit rating limits debt capacity | Credibility state → nonlinear spread → displayed rating. A market absorption curve makes heavy borrowing progressively dearer, with no hard cap. | §3.7 |
| Critical debt, extremely high deficit | Continuous debt dynamics with rollover lag (pressure builds over years). Named warning bands with hysteresis emit events but never switch equations. | §3.6–3.8 |
| Inflation critical, money printing | Monetary financing → Cagan money demand. Hyperinflation emerges when financing exceeds the seigniorage maximum; it is not scripted. | §3.4 |
| Reserves, auto-repay, bloated reserves | Treasury stock, an auto-repay toggle, and a "reserves while indebted" warning band | §3.6 |
| Austerity toggles, tax cuts, investment once a year | An annual budget plus limited monthly discretion; austerity is just spending cuts with multiplier and faction effects | §3.6, §3.13 |
| Economic sphere limits trade partners | Trade access matrix from WS-06 (spheres, sanctions, tariffs) feeding demand, diffusion and import prices | §3.10 |

What we deliberately do *not* copy: opaque crisis states with fixed modifiers, and mechanics with no stated formula. Every number on screen decomposes (P-3).

## 1. Purpose & scope

WS-03 simulates each polity's real economy, public finances, money, prices, income and wealth distribution, and population. It hands the outcomes to WS-04 (factions), WS-05 (R&D and industry), WS-06 (diplomacy and AI) and WS-11 (forces).

**In (MVP):** five production sectors; capital accumulation; stochastic shocks and Markov regimes; a 20-bin income and wealth distribution; a full tax suite with behavioral response; an annual budget with monthly discretion; deficit financing through bonds, reserves, monetary financing and a bailout facility; a bond market with a nonlinear high-debt spread; monetary policy with central-bank independence (CBI); a Phillips curve with Cagan hyperinflation; cohort-component demographics; corruption and a shadow economy; planned-economy settings; a trade-access stub; a minimal FX stub; per-polity structural-parameter beliefs and a budget projection; a macro-tier reduced form.

**Out:** detailed trade networks and bilateral trade balances by good; the financial sector (banks, credit cycles, asset bubbles); FX markets and speculative attacks; corporate stocks and bonds; hard sovereign default, restructuring and haircuts (soft only, OQ-E1); migration beyond a net-rate stub; regional (sub-polity) economies.

## 2. Interfaces

### 2.1 Stage placement (WS-02 §3.3)

| Stage | WS-03 function | Writes | Reads (stage-start) | Cadence |
|---|---|---|---|---|
| R1 ApplyOrders | `econ::apply_order` | `econ` order fields | orders | monthly |
| R2 Exogenous | `econ::exogenous` | `econ.shock` (shock states, regimes, global factor) | `econ`, `gov`, `events` hooks | monthly |
| R3 Production | `econ::production` | `econ.prod` (potential, gap, output, factor incomes, capital, trade flows) | `econ`, `gov`, `rnd`, `force` | monthly; sector reallocation annual |
| R4 Finance | `econ::finance` | `econ.fin` (taxes, budget, debt, yields, policy rate, inflation, FX), `econ.dist`, `econ.demo`, observations | R3 results | monthly; distribution shape and demographics annual (fiscal-year rollover) |
| R7 Tier | `Tiered for EconBlock` | `agg.gdp`, `agg.population`, `agg.gov_debt`, `agg.treasury` | R6b results | monthly |
| D1/D2 Decide | `econ::project` (pure) | nothing | `PolityView` only | on demand |

`EconBlock { shock, prod, fin, dist, demo, macro: Option<MacroEcon>, detail flags }`. Each sub-block has one writer in one stage (ARC-021).

Explicit lags that follow from placement: the policy rate set in R4 reaches the demand gap in R3 of the next tick (one month). Faction behavior from R5 reaches investment in the next tick. War capital damage recorded in R6 is applied in R2 of the next tick. The R5b manpower draw reaches the labor force one tick late (already noted in WS-02). WS-05 in R3 sees last month's defense-sector capacity.

### 2.2 Outputs other subsystems consume

| To | Item | Unit |
|---|---|---|
| WS-04 | `EconOutcomes` per `EconKey` (§2.4): real disposable income level and 12-month growth, effective tax rate, transfers received, unemployment exposure, inflation loss, employment and profits (sector keys) | real per capita / share |
| WS-04 | Spending per budget line (WS-04 classifies public vs. private goods); corruption rents (leakage amount) available to the winning coalition | real |
| WS-04 | Stress indicators: spread, rating band, inflation, real growth, shortage index, arrears, conditionality index, active warning bands (as events) | — |
| WS-04 | Distribution summary: wealth shares (top 10/1/0.1%, bottom 50%) in three concepts (**measured**, which excludes offshore wealth; **true**; and **augmented** with pension rights), the five group shares with their flow breakdown, income shares (same cuts), homeownership rate, aggregate household wealth and wealth/income ratio (§3.5) | share / real |
| WS-04 | `project()` incidence per `EconKey` for a draft budget, so factions can preview and react before enactment | — |
| WS-05 | Funding per R&D line (civil fields, defense R&D, space program) and procurement appropriations, in real terms deflated by an R&D cost index (wage-weighted) | real |
| WS-05 | Defense and space sector capacity, utilization and wage index; skilled-labor pool (`h × L` in those sectors); FX rates and reserves | — |
| WS-11 | Recruitment pool by sex and 5-year band (ages 15–49); O&S and personnel appropriation; funded fraction of WS-11's upkeep demand | persons / real |
| WS-06 | `Aggregates` (gdp, population, gov_debt, treasury); public credit rating; real exchange rate; trade dependence by partner | reference currency (RC) |
| All | `Aggregates` additions: **`gov_debt`, `treasury`** (closes WS-02 OQ-A4) in RC at current market FX | RC |

### 2.3 Inputs consumed

| From | Item | Used in |
|---|---|---|
| WS-04 | `cbi ∈ [0,1]` (WS-04 is the only source; WS-03 has no default), `corruption ∈ [0,1]`, `stat_bias ∈ [0, 0.3]` (statistics inflation by low-accountability regimes), `FactionEconBehavior` (§2.4), budget enactment result, monthly discretion band, legality of price controls, laws read by WS-03 (pension age → §3.6 retirees and §3.5 Transfer role; women's status → female participation §3.9 and TFR), conscription law (passed to WS-11) | §3.3–3.8, §3.11, §3.12 |
| WS-04 | `shocks { capital_loss, output_loss } ∈ [0, 0.5]` from coups and revolutions: capital_loss destroys that share of every sector's K (and the matching private wealth) in R2; output_loss is a one-tick gap shock | §3.1, §3.2 |
| WS-05 | Sector TFP multipliers `M_s ≥ 0`, capital-quality and depreciation modifiers, health-tech and contraception-tech indices, procurement spending by seller polity, arms export receipts and license fees | §3.2, §3.9, §3.10 |
| WS-06 | Trade access matrix `a_ij ∈ [0,1]` (spheres, sanctions, embargoes), bailout lender availability and terms, sphere anchor polity | §3.7, §3.10 |
| WS-11 | Upkeep demand (personnel count, pay, O&S cost), manpower drawn by sex and band, war casualties by sex and band | §3.6, §3.9 |
| WS-07/11/08 events | Capital destruction by sector (war damage, disasters) | §3.3 (R2) |

### 2.4 WS-04 contract (decision 5)

P-8: factions *are* the economic interest groups. WS-04 owns the faction list, membership and utility. WS-03 owns material outcomes and how behavior moves the economy. Neither side duplicates the other.

```rust
pub enum Role { Wage, Capital, Transfer, Public }    // Public = government employees and the military
pub struct EconKey { pub bin: u8 /* 0..20 */, pub sector: Option<SectorIdx>, pub role: Role }
// sector is Some for Wage and Capital keys and None for Transfer and Public keys:
// at most 20·5·2 + 20 + 20 = 240 keys; keys with pop_share = 0 are omitted.
// WS-03 publishes, each tick (R4):
pub struct EconOutcomes { pub by_key: Vec<(EconKey, KeyOutcome)> }   // sorted by key
pub struct KeyOutcome { pub pop_share: f64, pub real_disp_income: f64, pub disp_growth_12m: f64,
                        pub eff_tax_rate: f64, pub transfers: f64, pub unemp_exposure: f64, pub inflation_loss: f64,
                        pub wealth: f64 /* real net wealth per capita, §3.5 key wealth */ }
// Corruption rents are reported separately (§2.2) and are never added to any key's real_disp_income.
// WS-04 publishes, each tick (R5):
pub struct FactionEconBehavior {
    pub membership: Vec<(FactionId, EconKey, f64)>,  // weights; Σ over factions per key ≤ 1 (split membership allowed)
    pub invest_sentiment: Vec<f64>,    // per faction, [-1, 1]
    pub labor_militancy: Vec<f64>,     // per faction, [0, 1]
    pub flight_propensity: Vec<f64>,   // per faction, [0, 1]: capital flight / emigration of the top bins
    pub compliance: Vec<f64>,          // per faction, [-1, 1]: tax compliance and directive compliance
}
```

WS-03 projects behavior onto keys: `mod_k = Σ_f membership(f,k) · behavior_f`. Key modifiers act on private investment (capital keys), labor supply, wage cost-push and strike losses (wage keys), the capital-flight share of top-bin wealth, and the shadow-economy share. WS-03 never reads faction utility. WS-04 never reads WS-03 internals beyond `EconOutcomes`, the budget lines and the stress indicators.

### 2.5 Orders (WS-03 variants of `OrderKind`)

| Order | Cadence | Effect |
|---|---|---|
| `EnactBudget(BudgetDraft)` | annual, effective at fiscal-year start (content; default January) | Full tax schedule, tariffs, appropriations by line, transfer parameters, debt rule. Gated by WS-04 (veto players). If it fails, last year's budget continues (continuing resolution). |
| `AdjustTax { tax, delta }` | monthly, within the WS-04 discretion band: 2 pp × clamp(W_exec / 0.2, 0.5, 2) per tax per fiscal year (WS-04 §3.2) | Temporary rate change; out-of-band orders are rejected with an event |
| `PublicInvestment { sector or infrastructure, amount }` | monthly | Discretionary investment within the investment envelope plus borrowing |
| `SetRegulation { kind, level }` | monthly | Regulation level, price controls, capital controls, investment directives, **mortgage LTV cap**, **mortgage-interest deductibility** (v0.3) |
| `EnactBudget` fields (v0.3) | annual | **Pension funded share** ∈ [0, 1] (the share of pension promises pre-funded in pension funds instead of pay-as-you-go; moves at most 0.05/yr) and the **social housing** spending line |
| `Supplemental { line, amount }` | monthly | Emergency appropriation (war, disaster); WS-04 charges political cost |
| `IndustrialConversion { from, to, share }` | monthly | Moves capital between industry and defense at a conversion cost and lag |
| `SetMonetary { policy_rate_pref, financing_share }` | monthly | Effective only in proportion to (1 − CBI) |
| `SetDebtRule { auto_repay, reserve_floor_months, max_yield }` | monthly | TNO-style reserve and repayment behavior; arrears threshold |
| `DrawBailout { amount }` | monthly | Draws the facility if WS-06 offers one; raises the conditionality index |

All WS-03 orders pass through WS-04's pure `gate_orders` function in R1 (WS-04 GOV-064), so both subsystems reject exactly the same orders; WS-03 adds no gate of its own beyond range validation.

## 3. Model

Notation: `t` is in months; annual rates are converted as `x_m = (1+x)^(1/12) − 1`. Real quantities are in base-year local currency (LCU); nominal = real × price level `P`. `d` = debt/GDP (nominal/nominal). All state variables shown in the UI are `Tracked`; internal arrays (cohorts, bins, fine tax grid) are plain `f64`s (WS-02 §3.5).

### 3.1 Sectors and production function (decision 1)

| Option | Pros | Cons |
|---|---|---|
| A. 3 sectors (primary, industry, services); defense and space are government purchases from industry | Cheapest; easy calibration | No defense-industrial base state for WS-05/WS-11; no mobilization trade-off |
| **B. 5 sectors**: agriculture & extraction, industry (manufacturing, construction, energy), services, defense industry, space industry | Defense and space capacity exist as stocks for WS-05; mobilization (conversion) is a real trade-off; structural transformation (Engel's law) emerges | 5 × capital stocks; defense and space need demand caps |
| C. 7 sectors (B + separate energy and resources) | Better oil shocks and resource curse | More data and calibration; energy shocks can be done as a global cost shock instead |

**Recommendation: B [D].** Energy enters as a global commodity cost shock (§3.3) and a resource-rent hook from WS-08, not as a sector.

| Form | Pros | Cons |
|---|---|---|
| Cobb-Douglas (σ = 1) | Exact log-additive breakdown; constant factor shares; stable | Empirically rejected: a meta-analysis finds the evidence "emphatically rejects" Cobb-Douglas [V: [Gechert et al., CEPR DP15687](https://repec.cepr.org/repec/cpr/ceprdp/DP15687.pdf)]; factor shares (and so inequality) can't move |
| **CES with labor-augmenting tech** | Matches evidence (σ < 1: Chirinko et al. estimate ≈ 0.40 [V: [JBES 2011](https://ideas.repec.org/a/taf/jnlbes/v29y2011i4p587-594.html)]); labor share responds to capital deepening; Cobb-Douglas is the σ = 1 case | Breakdown needs a Törnqvist split plus an explicit interaction term |
| Nested KLEM / translog | Richest | Far beyond MVP need |

**Recommendation: CES, σ per sector in content, default 0.6 [D]** (between the verified 0.40 estimate and the biased literature average of 0.9; the prototype tests sensitivity to 0.4–1.0). Per sector `s`:

```
Y*_s = Z_s · [ α_s K_s^ρ + (1−α_s) (A_s h L_s)^ρ ]^(1/ρ) · G_pub^φ,    ρ = (σ_s − 1)/σ_s
Z_s  = M_s (WS-05 tech multiplier) · (1 − x_ineff · state_own_s) · exp(z_s)   (z_s: TFP shock, §3.3)
A_s  : labor-augmenting tech; ln A_s grows by
       g_A = g_front + θ · q_abs · ln(A_frontier / A_s) − β_inf · max(0, π − π_thr)
```

- `A_frontier` = max over polities of `A_s` (public information, 12-month lag). Catch-up speed `q_abs ∈ [0,1]` = f(h, openness, 1 − corruption) [D].
- `G_pub` = public capital per worker (infrastructure line). Elasticity φ = 0.08 [R].
- The `β_inf` term is the "inflation eats growth" channel: above a threshold `π_thr`, inflation reduces TFP growth [R: threshold estimates of roughly 1–3% for advanced and 11–12% for developing economies, Khan & Senhadji].
- Capital: `K_s(t+1) = (1 − δ_s/12) K_s + I_s/12`.
- Investment: `I = i_rate · Y`, with `i_rate = i0 + β_r (MPK_net − r_real) + Σ_k invest_mod_k + regime_s − χ · deficit_share`. In planned mode the directive rate applies, scaled by compliance (§3.12).
- Annual sector reallocation: labor and new investment move toward equal net returns with partial adjustment λ_L = 0.10/yr [D]. Consumption shares follow Engel curves (food share income elasticity ≈ 0.5 [R]).
- Defense and space output = min(capacity Y*_s, demand from WS-05 procurement plus export orders). Spare capacity shows as utilization < 1. WS-05 reads capacity and utilization.
- Industrial conversion moves capital between industry and defense at a loss `c_conv` (default 30% of moved K [D]) over `T_conv` months (default 12 [D]).
- Breakdown (P-3): sector output growth = capital term + labor term + `h` term + TFP term + public-capital term + interaction, using Törnqvist (mean-of-period) factor shares. The interaction term makes the sum exact (ARC-041).

### 3.2 Demand, gap and output

A reduced-form IS curve sets the output gap; output equals potential × (1 + gap). This is deliberately not a full demand accounting model: it is cheap, stable and calibratable.

```
gap*     = m_G(regime) · (G_disc − G_base)/Y + Σ_b MPC_b · (Disp_b − Disp_base_b)/Y − ψ (r_real − r*)
           + NX_dev/Y − strike_loss + μ_d(regime)
gap(t+1) = ρ_g · gap + (1 − ρ_g) · gap* + ε_d
Y = Σ_s Y*_s · (1 + gap)            (defense and space sectors use their own utilization)
u = u* − gap / β_okun               (Okun's law)
```

`G_base` and `Disp_base` are the previous fiscal year's values (piecewise constant within a year), so a permanent fiscal change stimulates for a while and then fades as the base catches up. A sustained +1% of GDP impulse moves the gap toward `m_G × 1%`; it does not accumulate.

- MPC by bin falls from 0.9 at the bottom to 0.3 at the top [R], so transfers to poor bins stimulate more than top-rate cuts. This is the distribution-to-demand coupling.
- `m_G` is 0.8 in expansion and 1.5 in recession [R: Auerbach–Gorodnichenko range]. It is also a belief subject (§3.11).
- `ρ_g` = 0.95/month [D]; ψ = 0.5 [D]; β_okun = 2 [R].

### 3.3 Shocks and regimes (decision 4)

All draws use WS-02 streams `{tag: "eco", entity: polity, purpose: P_*}`; the global factor uses `entity = u64::MAX`.

| Variable | Distribution | Default | Why this distribution |
|---|---|---|---|
| Sector TFP `z_s` | AR(1) in logs, Normal innovations | ρ_z = 0.98/month; annual σ ≈ 1.5% [R] | Lognormal levels; persistence makes booms and slumps last |
| Demand `ε_d` | Normal | annual σ ≈ 1.5% of GDP [R] | Symmetric business-cycle noise |
| Cost-push `ε_π` | Student-t, ν = 4, scaled | annual σ ≈ 1 pp [D] | Fat tails: oil-shock-like jumps without scripting |
| Global commodity factor `f_g` | AR(1), Student-t ν = 4 | load λ = 0.5 on ε_π and ε_d [D] | Correlated world shocks (1973-style) |
| Risk premium `ε_rp` | Normal × (1 + S_nl/500 bp) | σ = 15 bp/month at low debt [D] | Markets are jumpier near the debt limit |
| Fertility, mortality levels | Normal on logit TFR / Brass α | σ = 0.02/yr [D] | Small, bounded noise |
| Regime | Markov {Expansion, Recession, Crisis} | see below | Persistent states with legible names |

Not stochastic: tax and budget formulas, accounting identities, cohort aging.

Regimes: monthly transition matrix in content (default: E→R 1.2%, R→E 8%, R→C 3%, C→R 12% [D], tuned so recessions hit about 1 year in 6 [R]). Exit from Expansion rises with a stress index `S = w1·max(0, spread − 300 bp) + w2·max(0, π − 10%) + w3·max(0, −real growth)` via `p_ER = p0 · exp(k_S · S)`, capped at 10% [D]. Regimes shift `μ_d`, `m_G` and the TFP mean. Correlation: `ε = λ f_g + √(1−λ²) ε_idio`.

### 3.4 Prices, money and monetary policy

```
π (12-month)   = π^e + κ·gap + β_mf·(monetary financing / Y) + β_imp·Δ(import price, tariffs, FX) + ε_π + wage_push
π^e            = A·π* + (1 − A)·π(t−1),     anchoring A = CBI · C_m   (monetary credibility, §3.7)
i_rule         = r* + π + 0.5(π − π*) + 0.5·gap          [R: Taylor 1993]
i_target       = CBI · i_rule + (1 − CBI) · i_gov_pref
i(t+1)         = ρ_i i + (1 − ρ_i) i_target,  floored at i_zlb
```

- κ = 0.15 [R]; ρ_i = 0.93/month [R]; r* = 2%, π* = 2% are content parameters per era (1955 settings may target price stability, not 2%).
- **Cagan block (hyperinflation):** real money balances `m = m0 · exp(−a·π^e)`, a = 4 (annual) [R]. Seigniorage revenue = μ·m, where μ is money growth. Monetary financing above the maximum seigniorage `m0/(a·e)` cannot be funded at stable inflation, so π^e and π run away. Hyperinflation is a band at monthly π ≥ 50% [V: Cagan's definition, [Banque de France](https://banque-france.fr/en/publications-and-statistics/publications/fiscal-roots-hyperinflation-historical-perspective)]. It is emergent; no hard switch.
- Price controls (§3.12) suppress measured π and accumulate a monetary overhang.

### 3.5 Income and wealth distribution and taxes

**State [D]:** three parametric marginals, each with a lognormal body below p_t = 0.90 and a Pareto tail above, matched continuously at p_t:
- labor income `L`: `(σ_L, α_L)`; mean from the wage bill
- household net wealth `W`: `(σ_w, p0)` body plus a piecewise Pareto top (`α_U9`, `α_T09`, `α_T01` above p90, p99 and p99.9), rebuilt each year from the five group means of the wealth dynamics below (generalized Pareto interpolation, [R] Blanchet–Fournier–Piketty). `p0` is the share of households with zero net wealth. The 3-parameter fits in the table below are the special case of one tail exponent and are used for the 1955 start; mean = `W_hh` / households
- aggregate household wealth `W_hh = q_A · P · Σ_s (1 − state_own_s) K_s + dom_hold · D_d + κ_h · Y_nominal`. `q_A` is an asset-price index with partial adjustment 0.2/yr toward `MPK_net / (r_real + eq_premium)`, bounded [0.5, 2.5] [D]. `dom_hold` is the domestic share of government bonds (0.8 [D]). `κ_h` stands for housing and land, calibrated so `W_hh/Y` is 2–3 in the 1955 advanced archetype. Rich countries' private wealth/national income ratios rose from about 200–300% in 1970 to 400–600% in 2010 [V: [Piketty & Zucman 2014](https://researchonline.lse.ac.uk/id/eprint/66106)], and with s/g dynamics the model should reproduce that drift when growth slows.

Laws of motion (annual, partial adjustment rate λ_d = 0.1/yr [D]):
- `σ_L → σ_L*(skill-biased tech from WS-05, sector mix, Σ labor_militancy, minimum-wage regulation)`
- `α_L → α_L*(top marginal rate, regulation)`, bounded [1.3, 3.5] [D]
- The wealth shape has no law of motion of its own (v0.3): it follows from the group stock-flow below.

**Household panel [D]** (replaces v0.1's parametric income bins and the linear key-wealth rule). Each high-fidelity polity carries `H = 8,192` synthetic households with **oversampled tails**: on each score, 70% of households sit in the bottom 90%, 20% in p90–99 and 10% in p99–100, with weights so that weighted totals are unchanged (as the SCF itself oversamples). Measured against a 262,144-household reference (`prototypes/economy/panel_size.py`), 8,192 oversampled households beat 16,384 equal-weight ones on every metric: worst bin-wealth error 4.3% vs. 5.5%, top-1% income share ±0.002 vs. ±0.006, top-bracket tax base ±1.9% vs. ±5.1%. Error budget [D]: bin wealth ≤ 5%, top-1% share ≤ 0.003, top-bracket base ≤ 2%. Each has fixed scores from a deterministic rank-1 lattice: no RNG, regenerated on load, `#[serde(skip)]`, so nothing is added to the save. The scores are `z_L` and `z_W` (Gaussian with correlation `ρ_LW`), `z_A` (age, with correlation `ρ_AW` to `z_W`), `u_pub` and `u_sec` (uniform). Each fiscal year, and whenever a marginal or a role share changes:
1. **Rank-cell values.** A household's labor income and wealth are the exact means of their marginals over its *weighted* rank cell (cumulative weight before it to cumulative weight after it). Both marginals are therefore exact (totals and shares equal the analytic values), and the ranks carry the dependence.
2. **Roles.** *Transfer* = the top `old_share` of `z_A`, where `old_share` = population at or above the WS-04 pension age ÷ adult population (§3.9). These households' labor income is replaced by their pension. *Public* = a `public_emp_share` of the remainder, chosen by `u_pub`. *Capital* = remaining households with `r_hh·W ≥ θ_cap·L` (θ_cap = 1 [D]). Everyone else is *Wage*. Sector is assigned by cutting `u_sec` on sector employment shares (Wage) or private-capital shares (Capital). Roles and sectors therefore come out of the economy, with no role multipliers.
3. **Income** = labor income or pension + `r_hh · W_domestic` + transfers. Ranking by pre-tax total income gives the 20 income vigintiles. `EconKey` = (bin, sector, role).
4. **Taxes are evaluated per household** on each household's own wage, capital income, consumption (MPC by bin), wealth and, for Transfer households via age-specific mortality, estate. This replaces v0.1's 1,000-point grid. Capital-income and wealth taxes therefore fall on the households that actually hold the wealth.
5. **Key outputs** are weight-averaged over member households. A key with `n_k < n0 = 20` households is shrunk toward its bin mean, `x_k ← (n_k x_k + n0 x_bin)/(n_k + n0)`, and the bin's keys are then rescaled so the bin total is unchanged. Σ_k pop_k·wealth_k = W_hh holds exactly.
6. **Offshore wealth.** A share `φ_off` of top-vigintile wealth, driven by `flight_propensity` and capital controls, still counts in key wealth (oligarchs stay rich and keep their clout) but leaves the domestic tax bases and the domestic investment pool.
7. **Shares.** Top 10/1/0.1% and bottom 50% wealth shares are reported analytically from the marginal; income shares come from the panel.

Calibration [V data; fits are measured with `prototypes/economy/calib_wealth.py`]:

| Target | Data | Model fit |
|---|---|---|
| US wealth shares (Fed DFA, 2026 Q2) | top 0.1% 15.0%, top 1% 32.5%, top 10% 68.9%, bottom 50% 2.3%, per [Wikipedia, citing Fed DFA](https://en.wikipedia.org/wiki/Wealth_inequality_in_the_United_States) | σ_w 1.98, α_w 1.48, p0 0.31: top 10% and bottom 50% exact; top 1% 31.0%; top 0.1% 13.1% (a single tail slightly under-fills the extreme top) |
| Algeria wealth (WIR 2022): an emerging-economy check | top 10% 57%, bottom 50% 5.9%, top 1% 24.8% ([WIR 2022 country sheets](https://wir2022.wid.world/www-site/uploads/2022/03/0098-21_WIL_RIM_COUNTRY_SHEETS.pdf)) | σ_w 1.29, α_w 1.57, p0 0.30: top 10% and bottom 50% exact; top 1% 23.9% |
| US income shares | top 1% ≈ 20%, bottom 50% ≈ 13% (2016) ([WIR 2018 summary](https://inequality.hks.harvard.edu/file_url/2572)) | Panel total income: top 10% 0.45, top 1% 0.20 |
| Joint: mean net worth by percentile of usual income ÷ overall mean (SCF 2022: <20: 129.7k; 20–40: 218.7k; 40–60: 385.4k; 60–80: 636.8k; 80–90: 1,264.7k; 90–100: 6,629.6k; all: 1,063.7k) ([Fed SCF 2022 table 2](https://s3.documentcloud.org/documents/24173157/scf-net-worth.pdf)) | 0.12, 0.21, 0.36, 0.60, 1.19, 6.23 | Panel (σ_L 0.66, α_L 1.44, ρ_LW 0.18, W/Y 5, r 5%): 0.10, 0.23, 0.40, 0.76, 1.63, 5.40, **log RMSE 0.19**. v0.1 linear rule, c = 0.6: **0.61**; best c (0.9): 0.23; Gaussian copula on income rank without the capital-income channel: 0.34 |
| Trend | US top 0.1% wealth share 7% (1978) → 22% (2012) on capitalized income [V: [Saez & Zucman](https://inequality.stanford.edu/node/7537)]; series differ by method (compare DFA's 15%) | A drift target for the advanced archetype, not a point fit |

Why the panel: the top income decile holds about 62% of US wealth (SCF 2022), because top incomes are partly capital income generated by that wealth. A rule that only correlates income rank with wealth rank misses this. The v0.1 linear rule put 4.5× mean wealth in the top income decile against 6.2× in the data, and 0.41× in the bottom quintile against 0.12×. The remaining misfit (80–90th percentile too rich, top decile not rich enough) points to heterogeneous returns (r_hh rising with wealth rank, [R] Fagereng et al.), which the prototype may add as one parameter. 1955 starting shapes are [R] (lower concentration; for example, US top 1% wealth share in the low-to-mid 20s). The prototype pulls WID series for each archetype before tuning.

**Wealth dynamics (v0.3) [D].** The distribution's state is the mean real wealth `W_g` of five rank groups with fixed population shares: B50 (p0–50), M40 (p50–90), U9 (p90–99), T09 (p99–99.9), T01 (p99.9–100). These are WID's standard cuts, so every group maps one-to-one to published data. Annual update at fiscal-year rollover:

```
W_g' = W_g + S_g + Ret_g + Gain_g − Tax_g − Est_g + Mob_g,      then rescale Σ_g pop_g W_g' = W_hh (national-accounts anchor)
S_g    = s_g · Yd_g                                           Yd_g from the panel (households currently in wealth group g)
s_g    = s0_g + s_y · ln(Yd_g / Yd) − β_p · pens_gen · 1[g ∈ B50, M40] + β_t · (1 − transfer_security) − β_c · consumption_need
Ret_g  = (1 − τ_k,g) · (r_H h_g + r_E e_g + r_B b_g) · A_g − r_M · m_g · A_g · (1 − τ_ded · deductibility)
Gain_g = (h_g Δln q_H + e_g Δln q_E) · A_g                      realized share taxed as capital gains
Tax_g  = wealth tax (per panel household, exact)
Est_g  = d_g · W_g · τ_estate,eff(g)                           d_g: deaths share of the group (age profile from §3.9)
Mob_g  = (pop_{g+1} μ_{g+1} W_{g+1} − pop_g μ_g W_g) / pop_g    downward dilution flux (estates split among heirs, turnover)
```

- **Portfolio by group** (`A_g` gross assets; shares `h` housing, `e` equity/business, `b` bonds/deposits; mortgage debt `m_g`). Content per archetype, e.g. middle groups housing-heavy and top groups equity-heavy [R; ECB HFCS/SCF portfolio tables in M2(a)].
- **Homeownership** `own` moves 0.05/yr toward `f(LTV cap, deductibility, social housing/GDP, house price/income)` [D]. It sets `h_g` and `m_g` for B50 and M40. Mortgage claims are owned by the top groups through their bonds and bank equity, so leverage moves net wealth *between* groups without changing `W_hh`.
- **Asset prices:** `q_E` (the v0.2 `q_A`) for equity and business; `q_H` for housing, with partial adjustment 0.15/yr toward `f(r_real, LTV cap, real income growth)` [D]. Falling rates raise both, and the gains go to the owners in proportion to their holdings.
- **Financial repression** appears with no special rule: `r_B` is the bond yield minus inflation, so 1945–75-style inflation with capped yields erodes bond-heavy groups.
- **Pensions:** `pens_gen` = replacement rate × coverage. A funded share `f_p` builds a pension-fund stock (held in equity and bonds and counted in `W_hh`'s capital) that is **excluded from measured household wealth** and allocated to groups pro rata to labor income for the *augmented* concept. Generous pensions lower B50/M40 saving (precautionary and retirement saving crowded out).
- **Mobility μ_g** (default 0.02/0.03/0.05/0.06 per yr for M40/U9/T09/T01 [D]) is what makes the distribution stationary. In compartment form it is the random-growth logic: the top share settles where `r_net,g + s_g·Yd_g/W_g − g` balances `μ_g`. Policy moves `r_net` and `s_g` at once, so transitions take decades rather than centuries. Standard random-growth models generate transitions an order of magnitude too slow ([V] [Gabaix, Lasry, Lions & Moll](https://img.nber.org/papers/21363)); group-specific (type-dependent) returns and saving are their proposed fix. ECO-147 tests the speed.
- **Offshore:** `φ_off` of T09+T01 wealth (§ household panel step 6) rises with estate and wealth tax rates × (1 − enforcement) and with flight propensity. *Measured* shares exclude it; *true* shares include it. England's declared top-1% share fell from over 75% in 1900 to under 20% by 1970 [V: [Cummins, VoxEU](https://cepr.org/index%2Ephp/voxeu/columns/using-history-understand-hidden-wealth-uk)], but hidden elite wealth reverses about a third of the observed top-10% decline (same source). The model reproduces both.
- **Invariants:** group means stay strictly ordered (B50 < M40 < … < T01); B50 may be ≤ 0 (net debt). The panel's wealth marginal is rebuilt from the five means so that they match exactly.
- **Breakdown (P-3):** each group's Δ share = saving + returns + price gains − taxes − estates + mobility + rescale, with an exact sum. This is the distributional-national-accounts style decomposition, so "why did the top 1% get richer?" has an answer on screen.

Calibration targets for the dynamics (data [V]; acceptance in ECO-144 to ECO-146):

| Case | Data | Policy differences scripted in the scenario |
|---|---|---|
| US 1955–2012 | Top 0.1% wealth share 7% (1978) → 22% (2012) (Saez–Zucman, above); top 1% saves ~35% of income, bottom 90% ~0 ([VoxEU](https://cepr.org/index%2Ephp/voxeu/columns/exploding-wealth-inequality-united-states)) | Top income rate 91% → 70% → 28% → ~39%; estate tax cut after 1980; repression 1945–75; equity price rise as rates fall |
| UK 1955–1975 | Concentration fell sharply 1914–1979 and the UK became less wealth-unequal than the US ([V] [Alvaredo, Atkinson & Morelli](https://tmp.gc.cuny.edu/?p=2417)); top estate tax rate almost 80% in the 1950s ([V] Cummins) | High estate tax, repression, nationalization (state ownership) |
| Netherlands-like | Income Gini 0.28 (2013) with top 1% wealth ~24% and top 10% ~60% ([V] [OECD 2015](https://www.oecd.org/netherlands/oecd2015-in-it-together-highlights-netherlands.pdf)); wealth Gini 0.82 without pension rights vs. 0.54 with them ([V] [ISI 2019](https://isi2019.org/proceeding/1.IPS/IPS%20Volume%201/files/basic-html/page101.html)) | Large funded pensions; low homeownership (large social rental sector); high LTV with full deductibility |
| Belgium-like | Top 10% 43–44%, top 1% 12% of net wealth (HFCS 2010/2014) ([V] [NBB](https://nbb.be/en/node/135991)); high homeownership and house prices ([V] [NBB](https://www.nbb.be/en/node/464029)) | Same tax-transfer schedule as NL-like; PAYG pensions; high homeownership, moderate LTV |
| Cross-check | The lowest-homeownership western EU countries (DE, AT, FR, NL) have among the highest wealth inequality ([V] [Eurofound](https://www.eurofound.europa.eu/en/publications/all/a-picture-of-wealth-inequality-across-eu-member-states)) | — |

Survey (HFCS/SCF) and capitalized-income (Saez–Zucman) or DFA series measure the top differently. Each test compares against the data for its own concept, and acceptance bands are wide enough to cover that gap.

**Cheaper fallback (if the panel misses the time budget):** keep v0.1's linear rule but with c = 0.9 and role factors normalized within each bin. It fits the SCF profile nearly as well (0.23) but gives no income/wealth consistency, no emergent roles and no per-household tax bases.

**Tax engine:** Every schedule is content data: brackets with rates and thresholds (nominal, with an `indexed` flag), base definitions and elasticities. Revenue is computed per panel household and aggregated to keys and bins, so bracket thresholds inside a bin cause no artifacts. The panel is re-evaluated when the schedule, a marginal or a role share changes, and scaled by monthly income growth otherwise.

| Tax | Base | Behavioral response |
|---|---|---|
| Personal income (progressive) | Reported income `z` | `z = z0 · ((1−τ_m)/(1−τ_m0))^e`, ETI e = 0.25 [V: SSG midpoint, [Wikipedia summary of SSG 2012](https://en.wikipedia.org/wiki/Taxable_income_elasticity)]; top vigintile e = 0.4 [R] |
| Payroll / social insurance (added; not in the kickoff list) | Wage income up to a cap | Same ETI; funds pensions |
| Corporate | Capital income × corporate share | Profit shifting elasticity with openness (0.5 [D]); raises the user cost of capital in §3.1 |
| Capital gains | Realized gains = realization rate × asset wealth × Δq_A (asset-price index, above) | Realization elasticity (lock-in), 0.7 [R] |
| VAT/sales | Consumption by bin (MPC × disposable) | Through the shadow economy |
| Wealth | Net wealth above threshold | Avoidance elasticity 0.5 [D] plus flight (`flight_propensity`) |
| Estate | Mortality-weighted wealth of old cohorts | Avoidance 0.3 [D] |
| Tariffs | Imports by partner | Import elasticity (1.5 [R]); domestic price pass-through; protection raises the industry sector's demand share |
| Subsidies (negative tax) | Sector output, or bin transfers | — |

- Laffer behavior emerges. Check: for a top bracket with Pareto parameter `a` and ETI `e`, the revenue-maximizing rate is τ* = 1/(1 + a·e) [R: Saez 2001].
- Non-indexed brackets produce bracket creep under inflation (a silent revenue gain that WS-04 bins feel).
- **Shadow economy:** share `sh = clamp(sh0 + β_T·tax_burden + β_R·regulation + β_c·corruption − β_E·enforcement − Σ compliance_mod, 0.05, 0.65)`. Calibration anchor: an average of about 32% of GDP across 158 countries in 1991–2015, ranging from 7.2% (Switzerland) to 62.3% (Bolivia) [V: [Medina & Schneider, IMF WP/18/17](https://papers.ssrn.com/abstract=3124402)]. Shadow output is real output (it counts for the gap and welfare) but is outside every tax base. Official GDP excludes it.
- **Key wealth (WS-04 OQ-G7):** from the household panel above (v0.1's linear rule was withdrawn in v0.2).
- **Outputs (incidence view):** revenue by tax × bin; effective average and marginal rates per bin; pre-tax and post-tax-and-transfer Gini; income and wealth top 10/1/0.1% and bottom 50% shares; W_hh/Y; offshore share; shadow share.

### 3.6 Budget (cadence per Sean: majority annual, limited monthly discretion)

Spending lines (content, extensible): defense personnel, defense O&S, defense procurement, defense R&D, space program, civil R&D (by WS-05 field), education, health, pensions, welfare transfers, unemployment benefits, social housing (v0.3; lowers homeownership toward renting, raises B50 real consumption), infrastructure (public capital), administration & enforcement, internal security, subsidies, interest.

- **Mandatory (formula-driven):** pensions = retirees(age ≥ retirement age, a WS-04 policy) × replacement rate × average wage × coverage; health scales with the old-age share; unemployment benefits = u × LF × benefit rate (automatic stabilizer); interest = i_eff × debt. Demographic aging builds pressure here without any script.
- **Execution:** appropriations are spent at 1/12 per month. Corruption leakage `leak = c_leak · corruption` (default c_leak = 0.4 [D]) removes part of each discretionary line's real effect. The leaked amount goes to `EconOutcomes` rents (WS-04 allocates it to the coalition as private goods).
- **Enforcement spending** raises tax compliance and statistical capacity, and lowers leakage (diminishing returns).
- **WS-11 coupling:** defense personnel and O&S lines pay min(appropriation, WS-11 demand). WS-11 sees the funded fraction and decays readiness accordingly.
- **Education** moves human capital `h` toward `h*(education spending per student)` with a 15-year lag structure (cohort-embodied, partial adjustment 0.05/yr [D]).

**Financing waterfall (monthly, continuous; no hard default):**
1. Need `N = spending + interest − revenue`.
2. Surplus: auto-repay on → repay debt (domestic first); off → reserves.
3. Deficit: draw reserves down to the floor → monetary financing (share `financing_share × (1 − CBI)`) → issue bonds at the marginal yield `y_issue = y + γ_s · max(0, issuance/Y − b_cap)` (default b_cap = 1% of GDP per month; γ_s = 5 bp per pp [D]) → draw the bailout facility if offered and ordered → when `y_issue > max_yield`, the remainder becomes **arrears**: discretionary lines are cut pro rata, and an arrears stock accrues that must be paid later and costs credibility.
4. Debt stocks: domestic-currency `D_d` and foreign-currency `D_f` (from bailouts and FX issuance), each with an effective rate.

**Maturity (minimal) [D]:** one average maturity `M` (default 6 years [R]). Each month 1/(12M) of each stock rolls over at the current yield: `i_eff(t+1) = i_eff + (1/(12M) + new/D)·(y − i_eff)`. Yield spikes therefore feed interest cost over years: the TNO slow build.

**Debt identity (exact):** `D(t+1) = D + N_bonds − repay + FX valuation(D_f) + arrears conversion`. The breakdown of `Δd` = interest–growth differential `(i − g − π)/(1 + g + π)·d` + primary deficit + inflation erosion + FX valuation + stock-flow adjustments, with an interaction term so the terms sum exactly.

### 3.7 Bond market and credibility

Shape from the Millennium Dawn precedent (linear in debt/GDP, coupled to the policy rate; m1-findings §1.2 [V]), with an inflation term and a nonlinear tail added. MD's slope (debt/GDP × 10, roughly 10 bp per pp) is about 3× the empirical level effect, so Orchestra uses the empirical linear slope and gets its drama from the tail.

```
y = max(y_floor,  ī_pol  + tp0  + β_d·d  + β_def·def_struct  + S_nl(d)  + β_v·σ_π  + ε_rp)
ī_pol   = ω·i + (1 − ω)(r* + π^e)                       ω = 0.3 [D]
S_nl(d) = S_max · [L(k(d − d_eff)) − L(−k·d_eff)],      L = logistic
d_eff   = d_lim · (0.6 + 0.4·C) · (1 − γ_f · share_FX)
y_FX    = y_world + β_d·d + S_nl(d) + ε_rp            (no domestic inflation term)
```

| Parameter | Default | Tag |
|---|---|---|
| β_d | 3 bp per pp of debt/GDP | [V] Laubach 2009: 3–4 bp ([EconPapers](https://econpapers.repec.org/RePEc:tpr:jeurec:v:7:y:2009:i:4:p:858-885)); CBO: 2–3 bp ([CBO WP](https://www.cbo.gov/system/files/2019-03/55018-Debt%20Rates%20WP.pdf)) |
| β_def | 25 bp per pp of structural deficit/GDP | [V] Laubach 2009 |
| Cross-check | IMF rule of thumb: about 4 bp per pp of debt above 60% | [V] [WEF summary](https://www.weforum.org/stories/global-risks/public-debt-and-the-risk-premium-a-dangerous-doom-loop/) |
| tp0 | 100 bp | [R] |
| S_max | 2,500 bp | [R] (euro-crisis peak spreads were of this order) |
| k | 0.06 per pp | [D] (marginal slope at d_eff is S_max·k/4 ≈ 37 bp per pp, about 12× β_d) |
| d_lim | advanced 1.6, emerging 0.7, planned/closed 0.5 | [D], informed by fiscal-fatigue debt-limit literature (Ghosh et al. 2013) [R] |
| γ_f | 0.4 | [D] (original sin) |
| β_v | 0.5 × σ_π (12-month) | [D] |

- **Credibility `C ∈ [0,1]`** (fiscal) and `C_m` (monetary): asymmetric partial adjustment toward a target set by track record. It rises with a 5-year half-life when the primary balance responds to debt and inflation is near target. It falls with a 1-year half-life on monetary financing, arrears, missed conditionality targets, failed budgets, and WS-04 instability.
- **Rating (display):** implied annual default probability `p = S_nl / (1 − R)`, R = 0.4 [R], mapped to AAA…CCC bands. The rating is public (WS-06).
- **Bailout facility:** if WS-06 marks a lender available, draws come at `y_bail` (lender's rate + 100 bp [D]) in foreign currency. The conditionality index `CI = min(1, bailout stock/Y / 0.15)` [D] sets a primary-balance path; deviation costs credibility. CI goes to WS-04 (sovereignty loss, austerity anger). All continuous.

### 3.8 Warning bands (UI and WS-04 events; no equation switches)

| Band (TNO name) | Enter | Exit |
|---|---|---|
| Negative real growth | 12-month real growth < 0 | > 0.5% |
| High / critical inflation | π > 10% / > 40% | < 8% / < 30% |
| Hyperinflation | monthly π ≥ 50% [V Cagan] | monthly < 50% for 12 months [V Cagan's end rule] |
| Extremely high deficit | deficit > 6% of GDP | < 5% |
| Critical debt | spread > 500 bp | < 400 bp |
| Fiscal crisis | arrears > 0 or bailout drawn | both zero for 6 months |
| Bloated reserves | treasury > 6 months of spending while debt > 0 | < 4 months |
| Shortages (planned) | overhang > 10% of GDP | < 7% |

Thresholds are [D] content values. Bands emit `Event`s on entry and exit and never change any state equation (ECO-066).

### 3.9 Demographics

**Deviation from ConOps wording (proposed ConOps edit, not made):** ConOps §6 says "5-year bands". Stepping 5-year bands annually means moving 1/5 of each band per year, which smears cohorts (a 5-year baby boom spreads over decades). **Recommendation [D]:** single-year ages 0–100+, two sexes, stepped annually; content schedules and every interface use 5-year bands, interpolated in and aggregated out. That is 202 cohorts per polity per year, a trivial cost.

```
pop(a+1, s, y+1) = pop(a, s, y)·(1 − q(a, s)) − war_deaths(a, s) + mig(a, s)
births(y)        = Σ_a f(a)·pop(a, F),  f = TFR · shape(a) (content);  sex ratio 1.05 [R]
e0(y)            = e_max(health tech) − (e_max − e_min)·exp(−k_e · ln(y_pc / y_ref)) + health spending term   (Preston curve [R])
q(a, s)          : Brass logit relational model from a standard schedule; level α solved so the life table hits e0 within 0.1 yr [R method]
TFR              = TFR_min + (TFR_max − TFR_min)·L(−(ln y_pc − ln y_mid)/s_f) · (1 + family policy) · (1 − contraception tech) + noise
                   bounded [1.0, 7.5] [D]
```

- Within a year, population for labor-force purposes is interpolated linearly between January values.
- Labor force = Σ participation(a, s) × pop − military personnel (WS-11). Female participation is a WS-04 social-policy input.
- Recruitment pool: ages 15–49 by sex and 5-year band to WS-11.
- Net migration: a rate stub (policy plus income gap), bounded at ±1% per year [D].

### 3.10 Trade and FX stubs (minimal)

- **Trade:** partner weights `w_ij` (content, historical ties) drift toward access-weighted demand. Exports `X_i = o_i · Y_i · Σ_j w_ij · a_ij · (1 − τ_tariff_j) · demand_j`. Imports follow the openness target and the domestic gap. NX shocks feed §3.2. Access cuts (embargo, sphere change) lower demand, slow TFP diffusion (`q_abs` uses access-weighted openness) and raise import prices. That is the "pressure from trade relations". With ~45 polities this is 2,000 pairs per month, a trivial cost.
- **FX (minimal stub, what arms purchases need):** `e_ij = (P_i/P_j)·exp(ψ_fx·(spread_i − spread_j))` relative to a reference currency (RC = a world unit at 1955 PPP [D]). There is no FX market and no pegs (OQ-E2). Each polity holds FX reserves `R_fx`, fed by exports, arms exports and borrowing, and drained by imports, foreign procurement (WS-05), license fees and FX debt service. When reserves fall below 3 months of imports, the spread rises and foreign purchases are rationed pro rata (continuous) [D].

### 3.11 Structural beliefs and the budget projection (P-1, P-2)

The player gets "an informed but imperfect" projection. The imperfection comes from three sources: unknown future shocks, uncertain structural parameters, and noisy statistics.

- **Truth:** at worldgen, each polity's structural parameters are sampled into `Truth` (`worldgen_stream(seed, polity_id, purpose)`). The parameters are potential growth trend, fiscal multiplier `m_G`, Phillips slope κ, ETI `e`, debt limit `d_lim` (the market's true tolerance), and natural rate `r*`. Defaults are sampled around the archetype values above with σ of 15–30% [D].
- **Beliefs:** a `Belief` per subject (WS-02 §3.4). Each January the polity pushes one `Observation` per subject with noise `σ_obs = σ0 / stat_capacity` (stat_capacity from enforcement and administration spending). Beliefs therefore narrow over time and narrow faster with better statistics. A polity is never told exactly where its debt cliff is.
- **Official statistics:** reported GDP, inflation and unemployment = truth × (1 + stat_bias + noise). The breakdown includes a "statistical discrepancy" term.
- **Projection:** `econ::project(view: &PolityView, draft: &BudgetDraft, horizon) -> Projection` runs the same step functions with shocks at their means, the current regime frozen, and parameters at the belief means (mid case), with a pessimistic and an optimistic case at ±1σ on the three most influential subjects. It returns summary paths, warning-band forecasts and incidence per `EconKey`. It uses `PolityView` only, so ARC-012 (no access to `&Truth`) is compile-enforced. AI budgeting uses the same function.

### 3.12 Corruption and planned-economy settings (option a, per Sean)

A planned economy is the market model with policy settings. The economy does not fully follow the state; corruption drives the gap between directive and outcome.

| Setting | Effect |
|---|---|
| `investment_directive` (share of GDP, sector split) | Replaces the market `i_rate`. Achieved = directive × (1 − slippage), slippage = 0.5·corruption + misallocation penalty [D] |
| `price_controls ∈ [0,1]` | Measured π = (1 − pc)·true π. Suppressed inflation accumulates overhang `O += pc·π·Y`; shortage index = O/Y lowers real disposable income in `EconOutcomes`. Relaxing controls releases O into π over 6–12 months (transition burst) |
| `state_ownership_s` | SOE profits go to the budget (an SOE revenue line) instead of capital-key incomes; an X-inefficiency TFP penalty `x_ineff` (default 15% [D]) and soft-budget-constraint subsidies |
| `capital_controls` | Caps flight; lowers access-weighted openness and diffusion |
| Corruption | Budget leakage (§3.6), shadow-economy share (§3.5), directive slippage, statistics bias (via WS-04 `stat_bias`) |

### 3.13 Macro tier (SYS-006, P-7)

Macro polities carry `MacroEcon { y_real, pop, debt_d, debt_f, treasury, i_eff, credibility, tfp_gap, def_share }`:

```
ln y'   = ln y + (g_pop + g_front + θ q_abs ln(y_front/y_pc) + z)/12 − β_inf max(0, π_arch − π_thr)/12
pop'    = pop · (1 + (b(y_pc) − m(y_pc) + mig)/12)          b, m from the same TFR and e0 curves applied to a template age structure
d'      : same yield function (§3.7), with a fiscal reaction function pb = pb0 + ρ_B (d − d_ref) [R: Bohn-type, ρ_B = 0.03]
def_share : archetype × WS-06 posture → defense spending to WS-11
```

Tier switch (R7): demote copies conserved values. Promote calls `disaggregate` with a content template by archetype and income level (age pyramid shape, sector shares, K/Y, distribution shape, maturity at i_eff = y), rescaled so that gdp, population, gov_debt and treasury match.

### 3.14 Damping and stability strategy (decision 3, R-2)

Runaway paths we must control: the debt spiral (i > g with deficits), hyperinflation, population explosion or collapse, inequality divergence, tax-base collapse, and FX/reserve spirals.

1. **Stable by construction:** diminishing returns plus depreciation (Solow convergence); σ < 1; every slow shape variable uses partial adjustment toward a bounded target; saturating forms (logistic spread, bounded TFR, e0, α, shadow share); every AR coefficient < 1.
2. **Behavioral damping for AI and macro polities only:** a Bohn fiscal reaction (ρ_B > 0) and the Taylor principle (effective φ_π > 1 when CBI is high). The player is not damped, so spirals are reachable by policy, but slowly: rollover over M years and credibility smoothing (TNO feel).
3. **Soft limiters with legible causes:** the market absorption curve, arrears, reserve rationing, the bailout facility. All continuous, all explained.
4. **Numerical:** explicit monthly Euler steps with every `λ·Δt < 1`. The prototype checks the spectral radius of the linearized monthly map (< 1 for baseline settings).
5. **Last-resort clamps:** finiteness (ARC-023) plus range clamps (|monthly π| ≤ 10,000%, d ≤ 10, y ≤ 100%). Each clamp hit emits `ClampHit`. Baseline runs must have zero clamp hits.

### 3.15 Decision 2 summary: update cadence

| Variable group | Cadence | Rationale |
|---|---|---|
| Output, gap, inflation, policy rate, yields, debt, revenue accrual, spending, trade flows, FX, reserves | Monthly | Player feels the monthly turn; financial variables move fast |
| Capital stocks | Monthly (cheap) | Avoids annual jumps in output |
| Sector reallocation, distribution shape, structural observations | Annual (fiscal-year rollover) | Slow; annual removes noise and cost |
| Demographics | Annual, single-year ages | Exact aging (no band diffusion); population interpolated monthly |
| Budget and tax schedule | Annual (with monthly discretion within band) | Sean's cadence decision; continuing resolution on failure |

Budget [D]: ≤ 20 ms for R2–R4 with 6 high-fidelity + 40 macro polities (WS-02 §5). Estimate: high-fidelity monthly work is ~10⁴ flops per polity; the 8,192-household panel (one sort plus per-household taxes, ≈ 0.5 ms per polity [D estimate]) and the five-group wealth update runs on schedule, shape or role-share change, normally once a year; demographics once a year. Projection runs outside the tick (UI on demand) or inside D2 for AI (WS-06 budget).

### 3.16 UI data (progressive disclosure)

- **Summary (rounded):** real GDP growth, inflation, unemployment (0.1 pp); debt/GDP, deficit/GDP (1 pp); 10-year yield and spread (10 bp); credit rating; interest/revenue (1 pp); treasury in months of spending (0.5); active warning bands; projection fan for debt/GDP and inflation (mid/low/high).
- **Detail:** full schedule editor; incidence table (bin × tax); effective average/marginal rates; pre/post Gini and top shares; spending by line with leakage; debt-change and yield breakdowns; sector table; cohort pyramid (5-year bands); structural beliefs with σ.

## 4. Requirements

Tolerances: "rel" = relative. All tests run with fixed seeds. "Shocks off" means every stochastic term is at its mean.

| ID | Requirement | Acceptance criterion (testable) | Trace |
|---|---|---|---|
| **Structure** ||||
| ECO-001 | `EconBlock` has sub-blocks `shock`, `prod`, `fin`, `dist`, `demo` (high fidelity) or `macro` (macro tier); each has one writer stage. | Compile: R2–R4 functions return only their sub-block types; ARC-021 swap test passes with a stub WS-04 in the same stages. | ARC-021, P-4 |
| ECO-002 | WS-03 runs in R2 (shocks), R3 (production) and R4 (finance, monetary, distribution, demographics) only. | ARC-020-style trace test: WS-03 functions appear only in R2–R4 and R7 (tier) in the per-tick stage log. | ARC-020 |
| ECO-003 | All WS-03 randomness uses `"eco"` streams with enumerated purposes; the global factor uses entity `u64::MAX`. | Adding a stub subsystem drawing 1,000 values leaves every WS-03 output bit-identical (ARC-032 pattern); duplicate key panics in test builds. | SYS-002, ARC-032 |
| ECO-004 | `Aggregates` gains `gov_debt` and `treasury` in reference currency (closes OQ-A4). | ARC-080 extended: after each tick every alive polity has finite `gov_debt`, `treasury` with breakdowns; values equal Σ domestic + FX debt × FX rate within 1e-9 rel. | SYS-006 |
| **Production** ||||
| ECO-010 | Sector output follows CES with labor-augmenting tech, public capital and the WS-05 multiplier; σ = 1 reduces to Cobb-Douglas. | Unit: with σ = 1 − 1e-9 and σ = 1 exactly, output matches the Cobb-Douglas formula within 1e-6 rel; for σ = 0.6, matches a hand-computed value within 1e-12 rel. | §3.1 |
| ECO-011 | Output growth has a breakdown into capital, labor, h, TFP, public capital and interaction terms that sums exactly. | For 1,000 random inputs, Σ terms = Δ ln Y within ARC-041 tolerance; interaction term < 5% of |Δ ln Y| when factor changes < 2%. | SYS-004, P-3 |
| ECO-012 | Capital converges to the Solow steady state. | Deterministic one-sector run, shocks off, constant s = 0.2, n, g, δ: K/Y within 2% of s/(δ + n + g) after 100 years (an accumulation identity, so it holds for any σ). | R-2 |
| ECO-013 | Sector reallocation produces structural transformation. | Shocks off, exogenous A growth so income per capita rises ×4: agriculture employment share falls by ≥ 50% of its start value; services share rises. | §3.1 |
| ECO-014 | Labor-augmenting tech catches up to the frontier at a speed set by absorptive capacity. | Two polities, A_follower = 0.25·A_frontier, q_abs = 1: the log gap halves in ln 2/θ years ±10%; with q_abs = 0 it does not shrink. | §3.1 |
| ECO-015 | Defense and space output = min(capacity, demand); capacity and utilization are exposed to WS-05. | Demand = 2 × capacity → output = capacity, utilization = 1, unfilled demand reported = capacity; demand = 0.5 × capacity → utilization = 0.5. | WS-05 |
| ECO-016 | Industrial conversion moves capital with loss c_conv over T_conv months. | Convert 10% of industry K: after T_conv, defense K gained = 0.1·K_ind·(1 − c_conv) within 1e-9 rel; total K lost = c_conv share. | WS-11 |
| ECO-017 | Inflation above π_thr reduces TFP growth. | Shocks off, π held at π_thr + 10 pp for 10 years: A ends lower than the π = π_thr run by β_inf·0.10·10 in log terms within 1e-9. | §0 |
| **Shocks & regimes** ||||
| ECO-020 | Regimes follow the content Markov matrix, with stress raising the exit from Expansion. | 10⁵ months at S = 0: empirical transition frequencies within ±10% rel of the matrix; at S = 1, p_ER = p0·exp(k_S) within sampling error (3σ). | §3.3 |
| ECO-021 | Shock distributions match spec. | 10⁵ draws each: Normal shocks' mean within 0.02σ and σ within 2%; Student-t (ν = 4) fraction of |x| > 3·scale within ±10% rel of the theoretical t₄ tail probability; AR(1) ρ estimate within ±0.01. | §3.3 |
| ECO-022 | Polity shocks share the global factor with loading λ. | 20 polities × 10⁴ months: mean pairwise correlation of ε_π within λ² ± 0.03. | §3.3 |
| **Demand, prices, monetary** ||||
| ECO-030 | The IS gap responds to discretionary spending with multiplier m_G. | Shocks off, Expansion, r_real = r*, gap 0, base fixed: +1% of GDP discretionary spending sustained for 12 months → gap(12) = m_G · 0.01 · (1 − ρ_g^12) within 1e-9 rel. | §3.2 |
| ECO-031 | Expectations anchoring depends on CBI and monetary credibility. | One-off +5 pp cost shock, shocks otherwise off: CBI = 1, C_m = 1 → |π − π*| < 0.5 pp within 24 months; CBI = 0 → > 0.5 pp after 36 months. | §3.4 |
| ECO-032 | The policy rate follows the blended Taylor rule with smoothing and the floor. | Unit: given inputs, i(t+1) equals the formula within 1e-12; i never below i_zlb. | §3.4 |
| ECO-033 | Hyperinflation emerges from monetary financing beyond the seigniorage maximum. | Shocks off, CBI = 0: financing at 0.8 × max seigniorage → π stays < 100% annual for 20 years; at 1.5 × → monthly π ≥ 50% within 8 years. | §3.4, R-2 |
| ECO-034 | Price controls suppress measured inflation, accumulate overhang and release it. | Planned archetype, pc = 0.8 for 20 years: measured π ≤ 0.2 × true π; overhang/GDP ≥ 10%; set pc = 0 → annual π ≥ 50% within 12 months. | §3.12 |
| ECO-035 | Unemployment follows Okun's law. | Unit: gap = −2% → u = u* + 1 pp (β_okun = 2) within 1e-12. | §3.2 |
| **Distribution & tax** ||||
| ECO-040 | Labor income and wealth marginals are lognormal-body/Pareto-tail (wealth with a zero-wealth mass p0); 20 income vigintile bins come from the panel; wealth top shares are analytic. | Bin population shares = 0.05 each (± 1/H); income shares sum to 1 within 1e-12; analytic wealth top-1% share matches a 10⁶-point quadrature within 1e-4; with the US parameter set, the four wealth shares equal the §3.5 'model fit' values within 0.002. | §3.5 |
| ECO-041 | Shape parameters move by partial adjustment toward bounded targets. | Step change in target: after 1 year the parameter has moved λ_d of the gap within 1e-9; values never leave bounds over 10⁴ random target sequences. | R-2 |
| ECO-042 | Every tax is content-defined and computed per panel household; revenue = Σ households = Σ bins. | A test schedule adding a new bracket loaded from RON changes revenue with no code change; Σ bin revenue = total within 1e-9 rel; a bracket threshold inside a bin produces revenue continuous in the threshold (max jump < 1e-6 of revenue for a 1e-6 threshold move). | P-6, SYS-007 |
| ECO-048 | The household panel is deterministic, adds nothing to the save, and has exact weighted marginals. | Two loads of the same save give bit-identical panels; weights sum to 1 within 1e-12; weighted Σ labor income and Σ wealth equal the analytic totals within 1e-9 rel; the weighted wealth top-10% share equals the analytic value within 1e-6; vs. a 2^18 equal-weight reference, `panel_size.py` metrics are within the §3.5 error budget; the `World` postcard size is unchanged with and without the panel. | §3.5, SYS-002 |
| ECO-049 | The joint income–wealth profile matches the US calibration. | With the §3.5 US parameter set: log RMSE of mean wealth by income group vs. SCF 2022 ≤ 0.25, total-income top 10% share in [0.43, 0.50], top 1% in [0.17, 0.23]; Σ_k pop_k·wealth_k = W_hh within 1e-9 rel. | §3.5, WS-04 OQ-G7 |
| ECO-043 | Reported income responds to marginal rates with the ETI; Laffer behavior emerges. | Top bracket over a Pareto tail with a = 2, e = 0.25: revenue from that bracket maximized at τ* = 1/(1 + a·e) = 0.667 within ±0.01 (grid sweep, step 0.005). | §3.5 |
| ECO-044 | Non-indexed brackets produce bracket creep. | 10% inflation, real incomes constant, non-indexed: effective rate of bin 10 rises; indexed: unchanged within 1e-12. | §3.5 |
| ECO-045 | Incidence outputs exist and are consistent. | Revenue by tax × bin sums to total revenue; post-tax Gini ≤ pre-tax Gini for any schedule with non-negative marginal rates increasing in income and positive transfers to the bottom; effective rates in [0, 1]. | §3.5, §3.16 |
| ECO-046 | The shadow-economy share follows its function and bounds. | Unit: formula reproduced within 1e-12; share ∈ [0.05, 0.65] for 10⁴ random inputs; shadow output is excluded from every tax base (revenue unchanged when shadow output changes with official base fixed). | §3.5 |
| ECO-047 | Corruption leaks discretionary spending into rents. | corruption = 0.5, c_leak = 0.4: effective real spending = 0.8 × appropriation; rents output = 0.2 × appropriation within 1e-9 rel. | §3.6, P-8 |
| **Budget & debt** ||||
| ECO-050 | The annual budget takes effect at fiscal-year start; a failed budget continues last year's. | Enacted in month 11 with FY start month 1: no change until month 1; WS-04 stub rejects → schedule identical to prior year. | §2.5 |
| ECO-051 | Monthly tax adjustments are limited to the discretion band. | Cumulative AdjustTax beyond ±band in one FY is rejected with an event; within band applied next tick. | §2.5 |
| ECO-052 | Mandatory lines follow their formulas. | Pensions doubled when retirees double, other inputs fixed (within 1e-12 rel); unemployment benefits rise with u at fixed rate; interest = i_eff × D. | §3.6 |
| ECO-053 | The budget and financing identity holds every month. | Revenue + reserves drawn + money financing + bonds + bailout + arrears = spending + interest + repayment within 1e-9 rel for 10⁴ random months. | §3.6 |
| ECO-054 | The debt-change breakdown sums exactly. | Σ terms of Δd = d(t+1) − d(t) within ARC-041 tolerance for 10⁴ random months. | SYS-004 |
| ECO-055 | Effective interest rates converge to the market yield with time constant M. | Yield stepped +2 pp, no new borrowing: after M years, i_eff has closed 1 − e^(−1) ± 0.03 of the gap. | §3.6 |
| ECO-056 | Reserves and auto-repay behave per the debt rule. | Surplus with auto_repay on: debt falls by the surplus; off: treasury rises by it (within 1e-9 rel). | §3.6 |
| **Bonds & credibility** ||||
| ECO-060 | The yield has the components in §3.7, each in the breakdown. | Σ components = y within 1e-12 (when above floor); for d ≪ d_eff, ∂y/∂d = β_d ± 0.2 bp/pp. | §3.7, P-3 |
| ECO-061 | The nonlinear spread is zero at d = 0, monotone, saturating, and steep near the limit. | S_nl(0) = 0; S_nl increasing on [0, 5]; S_nl(5) < S_max; marginal slope at d_eff ≥ 5 × β_d. | §3.7, R-2 |
| ECO-062 | Credibility moves asymmetrically toward its target. | Target 1 from 0: reaches 0.5 in 5 years ± 0.25; target 0 from 1: reaches 0.5 in 1 year ± 0.1. | §3.7 |
| ECO-063 | The rating is a monotone map of the spread. | Rating band never improves as S_nl increases over a 1,000-point sweep. | §3.7 |
| ECO-064 | Issuance beyond market capacity raises the marginal yield continuously. | y_issue is continuous and non-decreasing in issuance; equals y for issuance ≤ b_cap. | §3.6 |
| ECO-065 | Financing gaps resolve continuously with no hard default. | Sweeping need from 0 to 20% of GDP in 1,000 steps: every financing source is continuous in need (max jump < 1e-6 × need step × 10); arrears > 0 only when y_issue would exceed max_yield. | Sean: soft |
| ECO-066 | Warning bands emit events with hysteresis and never change equations. | Scripted path crossing each enter/exit threshold yields exactly one enter and one exit event; a run with bands disabled is bit-identical in state (excluding `events`). | §3.8, R-1 |
| ECO-067 | The bailout facility raises conditionality continuously. | Drawing 0–30% of GDP: CI = min(1, stock/0.15) within 1e-12; CI exported to WS-04. | §3.7 |
| **Trade & FX** ||||
| ECO-070 | Trade flows use the WS-06 access matrix. | Embargo by the largest partner (a = 0 from year 20): 2 years later GDP is 1–5% below a no-embargo run with the same seed (median over 200 seeds), and 12-month π is +1 to +4 pp higher in year 1. | §3.10 |
| ECO-071 | FX rates follow PPP plus a spread term; foreign purchases are paid from FX reserves. | Unit: e_ij formula within 1e-12; a WS-05 foreign purchase of X (seller currency) reduces reserves by X × e within 1e-9 rel; reserves < 3 months of imports → purchases rationed pro rata, continuous in reserves. | WS-05 |
| **Demographics** ||||
| ECO-080 | Single-year, two-sex cohort component, stepped annually; accounting exact. | pop(y+1) = pop + births − deaths − war deaths + net migration within 1e-9 rel; every cohort non-negative. | §3.9 |
| ECO-081 | Interfaces use 5-year bands. | Band aggregation of single-year cohorts sums exactly; content 5-year schedules interpolated to single years reproduce band totals within 1e-9 rel. | §3.9 |
| ECO-082 | Mortality hits the target life expectancy. | For 100 random (y_pc, health tech) inputs, the life table e0 equals the Preston target within 0.1 year. | §3.9 |
| ECO-083 | Fertility is bounded and falls with income. | TFR ∈ [1.0, 7.5] for 10⁴ random inputs; shocks off, income ×4 over 50 years from TFR 6 → TFR < 2.5 at the end. | §3.9, R-2 |
| ECO-084 | The recruitment pool goes to WS-11 and the manpower draw comes back. | Pool = Σ pop ages 15–49 by sex and band (exact); drawing 5% of men 20–24 cuts the labor force next tick by that count × participation (within 1e-9 rel). | WS-11 |
| ECO-085 | Aging raises mandatory spending. | Old-age dependency doubled, everything else fixed: pension + health spending rise by the formula's factor within 1e-9 rel. | §3.6 |
| **Wealth dynamics (v0.3)** ||||
| ECO-140 | Group wealth follows the stock-flow identity and the national-accounts anchor. | For 10⁴ random years: W_g' − W_g equals Σ flows within 1e-9 rel before rescale; Σ_g pop_g W_g' = W_hh within 1e-9 rel after; each group's share breakdown sums exactly (ARC-041). | §3.5, P-3 |
| ECO-141 | Group means stay ordered and shares stay bounded. | 10⁴ random policy sequences × 105 years, shocks on: B50 < M40 < U9 < T09 < T01 every year; top-10% share ∈ [0.30, 0.90]; no `ClampHit`. | R-2 |
| ECO-142 | The wealth marginal reproduces the five group means exactly. | Weighted panel group means equal W_g within 1e-9 rel; the quantile function is non-decreasing and continuous at p50, p90, p99 and p99.9. | §3.5 |
| ECO-143 | Each policy channel moves wealth in the documented direction. | Shocks off, 20 years after a one-time change vs. baseline: estate tax +20 pp → T01 share lower; funded pension share 0 → 1 → B50+M40 measured share lower and augmented share within ±0.02 of baseline; LTV cap 0.8 → 1.1 with full deductibility → B50 share lower; social housing +1% of GDP → homeownership lower; real bond return −3 pp for 20 years → M40 and U9 shares lower; q_E +50% → top-1% share higher. | §3.5 |
| ECO-144 | The US and UK paths reproduce the direction and size of the historical change. | Scripted policy paths (§3.5 table), median of 200 seeds: US top 0.1% share in 1978 ∈ [0.05, 0.10], 2012/1978 ratio ≥ 2, 2012 ∈ [0.13, 0.24], and 1955→1978 not rising by > 0.01; UK measured top-1% share in 1970 ∈ [0.15, 0.25] and below the US value. | §3.5 |
| ECO-145 | The same tax-transfer system yields NL-like and BE-like wealth outcomes from pensions and housing alone. | Two polities with identical schedules, differing only in funded pension share, social housing and mortgage regime, after 40 years: both income Ginis ∈ [0.25, 0.31]; NL-like measured top 10% ∈ [0.55, 0.65] and top 1% ∈ [0.19, 0.29]; BE-like top 10% ∈ [0.39, 0.49] and top 1% ∈ [0.08, 0.16]; NL-like augmented Gini at least 0.15 below its measured Gini. | §3.5 |
| ECO-146 | Measured and true wealth diverge under high estate taxes. | UK-like path 1955–1975: true top-10% share minus measured ∈ [0.2, 0.45] × the measured decline. | §3.5 |
| ECO-147 | Transitions take decades, not centuries. | Top rate 70% → 28% at year 0, everything else fixed: half of the eventual change in T01 share is reached in 15–35 years. | §3.5, R-1 |
| **Interfaces** ||||
| ECO-090 | WS-04 inputs (cbi, corruption, stat_bias, behavior) are consumed; `EconOutcomes` are published for every key. | Stub WS-04 setting cbi 0 → 1 changes the policy-rate breakdown's CBI term; every `EconKey` with pop_share > 0 appears in `EconOutcomes` sorted by key; Transfer and Public keys have `sector = None`; Σ_k pop_k·wealth_k equals W_hh within 1e-9 relative; mean wealth per income bin (all roles pooled) is non-decreasing in bin. (v0.1 required monotonicity within each (sector, role); that is dropped because Transfer keys are legitimately wealthier than their low incomes imply, which is the life-cycle pattern in the data.) | P-8, WS-04 OQ-G7 |
| ECO-091 | Behavior is projected onto keys by membership. | Unit: mod_k = Σ_f membership·behavior within 1e-12; a faction with invest_sentiment −1 on capital keys lowers i_rate by the content coefficient × its membership. | P-8 |
| ECO-092 | The WS-05 interface is complete in both directions. | Stub WS-05: M_s = 1.1 raises Y*_s by 10% (shocks off, within 1e-9 rel); R&D funding lines appear in its input deflated by the R&D cost index; procurement from a foreign seller moves reserves (ECO-071). | WS-05 |
| ECO-093 | WS-11 upkeep is funded up to appropriation. | Appropriation = 0.7 × demand → funded fraction 0.7 exported; spending = appropriation. | WS-11 |
| ECO-094 | WS-06 receives the public rating, RER and trade dependence; WS-06 bailout availability gates `DrawBailout`. | A DrawBailout without an available lender is rejected with an event. | WS-06 |
| **Beliefs & projection** ||||
| ECO-100 | Structural parameters are per-polity truth with polity beliefs that narrow with observation. | Same seed → same truth regardless of polity count order (keyed by polity content ID); belief σ after 20 annual observations < 50% of initial σ at default stat capacity. | SYS-005, P-1, P-2 |
| ECO-101 | `project()` uses only `PolityView`. | `compile_fail` doc test: `project` cannot access `&Truth` (ARC-012). | ARC-012 |
| ECO-102 | The projection is exact when beliefs equal truth and shocks are off, and degrades with belief error. | Beliefs set to truth, shocks off: projected 60-month GDP path equals the simulated path within 1e-9 rel. With belief σ = 30% vs 5%: median absolute 12-month GDP projection error over 200 seeds is larger in the 30% case. | Sean: projection |
| ECO-103 | Official statistics carry bias and noise. | stat_bias = 0.2, noise 0: reported GDP = 1.2 × true and the breakdown has a "statistical discrepancy" term = 0.2 × true. | §3.11 |
| **UI data** ||||
| ECO-110 | The summary view exposes rounded headline numbers; the detail view exposes the incidence table; both read the same state. | API returns the §3.16 summary set rounded per its rules and the detail set unrounded; summary values equal rounded detail values. | m1 §1.3 |
| ECO-111 | Every WS-03 displayed variable is `Tracked` with a breakdown. | `list_vars` contains every §3.16 variable; each passes the ARC-041 sum check every tick of a 120-month run. | SYS-004 |
| **Macro tier** ||||
| ECO-120 | Macro polities run the §3.13 reduced form. | Shocks off, unit test of each equation within 1e-12; macro tick for 40 polities < 1 ms (bench). | P-7 |
| ECO-121 | Tier switches conserve gdp, population, gov_debt and treasury. | Promote then demote: each within 1e-6 rel at both steps (ARC-081). | SYS-006 |
| ECO-122 | Macro and high-fidelity versions of the same polity agree statistically. | 200 seeds, 20 years from the same start: median GDP within 15% and population within 5% between tiers. | SYS-006 |
| **Stability & performance** ||||
| ECO-130 | Baseline batch runs meet every §6.1 criterion. | `cli batch` (M5) / prototype batch (M2): all §6.1 rows pass. | R-2, SYS-003 |
| ECO-131 | Each failure mode is reachable by policy, on a plausible timescale. | All §6.2 rows pass. | R-1, R-2 |
| ECO-132 | WS-03 stays within its time budget. | `bench`: R2–R4 median ≤ 20 ms on the reference machine with 6 high + 40 macro polities. | SYS-008, ARC-091 |
| ECO-133 | WS-03 is deterministic. | Two 1,260-tick runs with the same seed and inputs: identical state hashes (ARC-060 pattern). | SYS-002 |

## 5. Content/data

| File | Schema (Rust type, `deny_unknown_fields`) |
|---|---|
| `content/econ/sectors.ron` | `[Sector { id, alpha, sigma, delta, engel_elasticity, demand_capped: bool }]` |
| `content/econ/taxes.ron` | `[TaxKind { id, base: BaseKind, elasticity, indexed_default }]` |
| `content/econ/spending_lines.ron` | `[Line { id, kind: Mandatory(Formula) | Discretionary, ws_link: Option<String>, leakable: bool }]` |
| `content/econ/regimes.ron` | `Regimes { states, matrix, stress_weights, effects }` |
| `content/econ/bands.ron` | `[Band { id, var, enter, exit }]` |
| `content/econ/archetypes/*.ron` | `Archetype { id, d_lim, sigma_struct, marginals: { sigma_l, alpha_l, sigma_w, alpha_w, p0 }, rho_lw, rho_aw, kappa_h, template_pyramid, sector_shares, k_y, ... }` (no CBI: WS-04 owns it) |
| `content/econ/demography.ron` | Standard life table, fertility shape by 5-year band, participation schedules |
| `content/econ/start_bands.ron` | Band → number maps for the WS-08a fields WS-03 owns (§5.1) |
| `content/settings/<s>/econ_overrides.ron` | Optional per-polity WS-03 overrides (archetype, debt, tax schedule, budget, pyramid shape), keyed by polity ID. WS-08a `start.ron` stays enums-only (WLD-021) |

All parameter defaults in §3 live in content; the code holds formulas only (ARC-010).

### 5.1 WS-08a start-state fields owned by WS-03 (WLD-022)

| Field | WS-03 mapping (numbers set in M2(a) calibration, stored in `start_bands.ron`) |
|---|---|
| `economy.size` (Band) | Monotone band → share of world GDP; the world total is fixed by the 1955 calibration |
| `economy.development` (Band) | Monotone band → GDP per capita; with `size` it implies population. It also selects the archetype marginals (σ_w, α_w, p0, κ_h) and the `template_pyramid` |
| `economy.system` (`Market` · `Mixed` · `Planned`) | `state_own_s` (≈ 0.1 / 0.4 / 0.8 [D]), planning flags (§3.12) and the default price controls. WS-04 consumes the result, and its `gov` preset supplies CBI |

## 6. Verification

Unit and integration tests are named `eco_NNN_*`. M2(a) runs the batch criteria in Python first (`prototypes/economy/`); M5 ports them to `cli batch` tests in CI.

### 6.1 Baseline stability criteria (AI policy rules on; N = 1,000 runs per archetype, 1955–2060)

| Metric | Criterion | Basis |
|---|---|---|
| Finiteness, clamp hits | 100% finite; 0 `ClampHit` | ARC-023 |
| Real GDP per capita growth, 1955–2060 mean | 95% of runs in [0.5%, 4.5%]/yr; advanced archetype 1955–2000 median in [1.5%, 3.5%] | [R] long-run OECD ≈ 2% |
| Catch-up | Developing archetype with corruption ≤ 0.3 grows ≥ 0.5 pp/yr faster than advanced (median, 1960–2020) | [R] conditional convergence |
| Recession frequency | Years with negative real growth: 10–25% (median) | [R] |
| Detrended annual growth σ | Median in [1.5%, 3.5%] | [R] |
| Inflation, high-CBI archetype | Run median in [1%, 6%]; P(any 12-month π > 100%) < 1%; P(Cagan hyperinflation) = 0 | [V] Cagan threshold; [R] |
| Debt/GDP, advanced | 95th percentile of run maximum ≤ 200%; P(max > 250%) < 1% | [D] |
| Spread | Median < 300 bp in months with d < 90% | [R] |
| Population ×(2060/1955) | Advanced [0.7, 2.5]; developing [1.5, 12] | [R] UN history/projections |
| TFR, e0 | Always within [1.0, 7.5] and [30, 95] | §3.9 |
| Disposable-income Gini | [0.20, 0.65]; top 1% income share [3%, 35%] | [R] WID range |
| Wealth shares | Top 10% in [0.45, 0.85]; bottom 50% in [0, 0.10]; top 1% in [0.12, 0.45] | [V] the §3.5 calibration points lie inside; bounds [D] |
| W_hh / Y | [1.5, 7.0]; the advanced archetype's 1970→2010 median rises if g falls | [V] Piketty–Zucman 200–300% → 400–600% |
| Shadow share | [5%, 65%]; cross-archetype mean in [20%, 40%] | [V] Medina–Schneider |
| Labor share | [0.45, 0.80] | [R] |
| Monte Carlo convergence | Standard error of each reported median < 5% of its value | [D] |

### 6.2 Failure-mode reachability (positive tests; N = 500)

| Scenario | Criterion |
|---|---|
| Profligate: primary deficit 8% of GDP sustained, CBI 0.5 | Debt > 150% and spread > 500 bp within 10–30 years in ≥ 80% of runs; in < 5% of runs within 3 years (slow build) |
| Printing press: monetary financing 10% of GDP/yr, CBI 0 | Cagan hyperinflation within 2–8 years in ≥ 80%; ends within 24 months of stopping financing in ≥ 70% |
| Consolidation: primary surplus 4% from d = 120% | d < 90% within 10–25 years in ≥ 70%; gap negative in years 1–2 (median) |
| Laffer sweep: top MTR 0–95% | Top-bracket revenue peaks within ±3 pp of the analytic τ* |
| Price-control release | ECO-034 criteria in ≥ 90% of runs with shocks on |
| Embargo | ECO-070 criteria |
| Demographic transition | ECO-083 criteria in ≥ 90% with shocks on |
| War mobilization: defense 25% of GDP for 5 years, 8% of labor force drawn | Civil output falls 10–30%; π rises ≥ 3 pp unless taxed; recovery to pre-war trend ± 5% within 10 years in ≥ 60% |

## 7. Open questions

| ID | Question | Default if unanswered |
|---|---|---|
| OQ-E1 | Should players be able to order a debt restructuring/haircut (credibility hit, debt relief), post-MVP or MVP? | Post-MVP; arrears and bailouts only |
| OQ-E2 | FX pegs (Bretton Woods 1955–71, sphere currency blocs): needed for the MVP feel? | No; PPP + spread stub only |
| OQ-E3 | The monthly tax discretion band | **Closed (2026-11-27):** WS-04 sets it to 2 pp × clamp(W_exec/0.2, 0.5, 2) (WS-04 §3.2, GOV-066) |
| OQ-E4 | Should the shadow economy and true (vs. official) GDP ever be visible to the player? | No; only official stats and beliefs |
| OQ-E5 | Proposed ConOps edits (not made): (a) §6 Demographics "5-year bands" → "single-year cohorts, 5-year bands at interfaces"; (b) §6 Taxation adds payroll tax; (c) §6 Macroeconomy adds "IS-curve demand gap; per-polity structural beliefs (P-1) drive the budget projection". | Apply on approval of this spec |
| OQ-E6 | Reference currency for `Aggregates`: world unit at 1955 PPP (proposed) or market FX? | 1955 PPP world unit; market FX for debt |
| OQ-E7 | Is heterogeneous return by group (now in the wealth dynamics) enough to close the remaining top-end SCF misfit, or is a per-household return within the panel also needed? | Decide in M2(a); add only if ECO-049 fails with the v0.3 dynamics on |
| OQ-E8 | Should the augmented (pension-inclusive) wealth concept also go into `KeyOutcome`, for WS-04 factions that care about retirement security? | No; the distribution summary reports it, and `KeyOutcome.wealth` stays marketable wealth (oligarch clout and flight act on marketable wealth) |
