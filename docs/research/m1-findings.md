# M1 Research Findings

Date: 2026-11-22 · Workstream: WS-01 · Closes: ConOps §12 backlog (except where noted)

Confidence tags: **[V]** checked against the source linked · **[R]** recalled or inferred, not checked · **[D]** a design recommendation (judgment, not a fact)

---

## 1. Economy precedents (feeds WS-03)

### 1.1 TNO
- **[R]** I couldn't find primary documentation of TNO's current economy mechanic. The only TNO-specific source found is a community *proposal*, not shipped design. It splits taxes into business, tariff and income categories and suggests an arms-trade decision GUI. Source: [TNO Economy Proposal](https://docs.google.com/document/d/1ACrHwqMPJgQR_BYs3vK_yJAhUlHVmiRPGFFpqd_HbBY/export?format=pdf).
- **[D]** Sean's own experience of TNO is the better reference here. In WS-03, describe the *feel* to reproduce (e.g., debt and interest pressure, budget trade-offs, crisis events) instead of reverse-engineering TNO.

### 1.2 Millennium Dawn (HOI4 mod): a usable reduced-form precedent
- **[V]** Interest rate = (Debt/GDP) × 10 + (central bank policy rate × 0.5) + modifiers, with a 0.8% floor. GDP also sets power rank and the number of research slots. Source: [MD economy guide](https://millenniumdawn.github.io/Millennium-Dawn/player-tutorials/economy-guide/).
- **[D]** This is the right *shape* for ECO bond yields: linear in debt/GDP, coupled to the policy rate, plus credibility modifiers. Orchestra should add an inflation term and a nonlinear spread at high debt so debt spirals can emerge (ConOps R-2).

### 1.3 Victoria 3 treasury
- **[V]** Taxes and salaries have adjustable "level" settings. Raising taxes costs legitimacy and increases radicalization. Source: [V3 DD #12](https://admin-forum.paradoxplaza.com/forum/developer-diary/victoria-3-dev-diary-12-treasury.1488588).
- **[V]** Interest rates are deliberately low, so deficit-funded growth is viable until the debt ceiling binds (same source).
- **[V]** The design tension is real: some players explicitly don't want "the nitty gritty of tax law" (same thread).
- **[D]** Use progressive disclosure. The default view shows a few round numbers; a detailed panel exposes the full schedule and incidence by quantile. Both views read the same model.

## 2. Governance precedents (feeds WS-04)

### 2.1 Victoria 3 interest groups: precedent for P-8
- **[V]** Pops contribute political strength to interest groups. The amount depends on wealth and on the status or votes the power structure grants them, so one wealthy aristocrat in an oligarchy can outweigh a poor laborer by hundreds or thousands of times. Source: [V3 DD #6](https://forum.paradoxplaza.com/forum/developer-diary/victoria-3-dev-diary-6-interest-groups.1481423/latest).
- **[V]** Clout = an IG's political strength ÷ total political strength. Clout and Approval set an IG's active traits and effectiveness. Source: [Gamewatcher summary of DD #6](https://www.gamewatcher.com/victoria-3-dev-diary-6-interest-groups-political-parties).
- **[V]** A pop can split its support across multiple IGs. Source: [DD #6 thread p.3](https://admin-forum.paradoxplaza.com/forum/developer-diary/victoria-3-dev-diary-6-interest-groups.1481423/page-3).
- **[V]** Lobbies (IG coalitions around foreign policy) tie diplomacy to domestic approval. Source: [V3 DD #112](https://admin-forum.paradoxplaza.com/forum/developer-diary/victoria-3-dev-diary-112-political-lobbies.1662629/post-29549095).
- **[D]** Map this onto selectorate theory. *Political strength weighting* corresponds to the institution parameters that define W and S. *Clout* is a faction's power weight. *Approval* is the faction's utility from the current policy vector. Lobbies are a cheap way to connect WS-04 and WS-06.

### 2.2 Selectorate theory: a formal model small enough to implement
- **[V]** In the minimal model, the government has resources R. A winning-coalition member gets utility x + g (private plus public good). A selectorate member outside the coalition gets g. The leader keeps B + R − p·g − W·x. A small W favors private goods. Source: [Ethan Bueno de Mesquita lecture notes](https://voices.uchicago.edu/ethanbdm/files/2023/01/selectorate.pdf).
- **[V]** The empirical results are contested. Clarke & Stone argue that S and W are often unassociated with leaders' decisions, or associated in the opposite direction from what the theory predicts. Source: [Clarke & Stone](https://www.sas.rochester.edu/psc/stone/articles/LOPS3.pdf).
- **[D]** Use the selectorate model as a *game mechanic*, not as a claim about the world. It produces the right qualitative trade-off (public vs. private goods against coalition size) at minimal cost. Add veto players for legislative gridlock. Don't spend design effort defending its empirical validity.

## 3. R&D precedents and data (feeds WS-05)

### 3.1 Terra Invicta
- **[V]** Research is split into global *technologies* (pure science, shared by everyone) and private *projects* (engineering, owned by one faction). Source: [TI wiki: Technology](https://hoodedhorse.com/wiki/Terra_Invicta/Technology).
- **[V]** The faction that contributes the most to a global tech chooses the next one. Source: [TI Dev Diary #10](https://www.pavonisinteractive.com/phpBB3/viewtopic.php?p=55457).
- **[V]** Projects unlock *probabilistically*, with a base chance (e.g., Dusty Plasma Drive at 20%, 30% with a modifier). Factions get up to three private engineering project slots. Source: [TI wiki: Projects](https://wiki.hoodedhorse.com/Terra_Invicta/Projects).
- **[V]** Spreading research across targets earns efficiency bonuses (DD #10).
- **[D]** The science/engineering split maps cleanly onto Orchestra P-1:
  - *Science* narrows world-truth beliefs and can be shared or stolen.
  - *Engineering projects* turn beliefs into components and designs, with execution noise.
  - Orchestra goes further than TI: truth is uncertain, dead ends exist, and slots scale with funding rather than infrastructure.
  - TI's "random unlock chance" idea can carry Orchestra's hidden cross-field enablers.

### 3.2 Cost-growth distributions: empirical basis for RND stochastics
- **[V]** In about 60 ACAT I programs, estimate-at-completion cost growth is best fit by a **lognormal** (Weibull nearly as good). The mean growth factor is 1.36, the coefficient of variation 0.80, and the 85th percentile 3.1. Source: [Defense ARJ Vol. 32 No. 1, Spring 2025](https://online.flippingbook.com/view/638780396/47).
- **[V]** If only one distribution is used, those authors recommend the lognormal. Source: [same, p.63](https://online.flippingbook.com/view/638780396/63).
- **[V]** RAND found that completed programs grew about 46% in cost over their Milestone B estimate, with no improving trend across three decades. Source: [RAND RB236](https://www.rand.org/pubs/research_briefs/RB236.html).
- **[D]** Default RND cost-growth factor ~ Lognormal, mean ≈ 1.3–1.5, CV ≈ 0.5–0.8, scaled up by technology immaturity (low TRL means a wider σ). Treat schedule growth the same way. Exact parameters are set in the WS-05 spec.

### 3.3 Learning curves (feeds defense industry)
- **[V]** Wright's cumulative-average model (1936) remains the DoD standard. Source: [AFIT thesis, Moore 2015](https://apps.dtic.mil/sti/pdfs/ADA616177.pdf).
- **[V]** Production data show a "flattening effect" late in production, and Boone's diminishing-rate curve fits 169 DoD end-items better. Source: [NPS ARP SYM-AM-21-112](https://www.dair.nps.edu/bitstream/123456789/4291/1/AFIT-AM-20-161.pdf).
- **[V]** After a configuration change, the learning slope is steeper and differs from the stable slope. Source: [AFIT AD1016823](https://apps.dtic.mil/sti/tr/pdf/AD1016823.pdf).
- **[R]** Typical airframe slopes are around 80–85%.
- **[D]** Use Wright's curve with a flattening term. A design revision resets part of the cumulative learning, which delivers the systems-engineering lesson "spec churn is expensive" directly.

### 3.4 War Footing: defense market precedent
- **[V]** Most equipment must be licensed for a fee before it can be produced domestically. Licensing takes time, and domestic production then costs half as much. Arms deals take multiple turns, give a slight per-unit discount, and improve relations with the supplier. Spot buys exist but are limited. Source: [War Footing Dev Blog #4](https://sdg-website.pages.dev/dev-blog/04-the-economy.html).
- **[D]** Orchestra's market offers four acquisition paths: spot buy, arms deal, license, or develop in-house. Each trades cost against lead time and dependency. Licensed or bought components keep their provenance (origin polity, supply risk, embargo exposure).

## 4. Combat (feeds WS-07)
- **[V]** Hughes's salvo model (NPS, 1995) is a discrete-time model that includes defensive firepower. Lanchester's original equations are continuous-time and include only offensive fire. Source: [Wikipedia: Salvo combat model](https://en.wikipedia.org/wiki/Salvo_combat_model).
- **[V]** Armstrong (2005) published a stochastic salvo model that derives the mean and variance of surviving forces. Source: [Armstrong 2005, Operations Research 53(5)](https://core.ac.uk/download/62653817.pdf).
- **[V]** Armstrong also published area-fire and sequential-fire (first-strike) variants. Source: [Wikipedia, as above](https://en.wikipedia.org/wiki/Salvo_combat_model); [Armstrong 2014](https://core.ac.uk/download/62653809.pdf).
- **[V]** Armstrong & Powell applied the stochastic salvo model to air strikes at Coral Sea, using aircraft squadrons as the unit. That's precedent for using one kernel for air and naval combat. Source: [Carleton CSDS WP](https://www3.carleton.ca/csds/docs/working_papers/ArmstrongWP03.pdf).
- **[D]** One kernel, Armstrong's stochastic salvo, covers naval, air and space combat. Its sequential-fire variant models first-strike advantage, which matters for deterrence dynamics. Ground combat uses Lanchester.

## 5. Tech stack (feeds WS-02)
| Item | Finding |
|---|---|
| Tauri | **[V]** v2 is current; tauri@2.12.6 was released 2026-11-16 ([release notes](https://v2.tauri.app/de/release/tauri/v2.12.6/)) |
| milsymbol | **[V]** MIT license, which is compatible with GPL-3.0. Supports MIL-STD-2525C/D/E and APP-6 B/D/E; renders to 2525E/APP-6E since v3.0 ([npm](https://npmjs.com/package/milsymbol), [yarn](https://classic.yarnpkg.com/en/package/milsymbol)) |
| Deterministic RNG | **[V]** The Rand book recommends `ChaCha8Rng` for portable reproducibility and says to avoid `StdRng` and `SmallRng` when results must be reproducible ([Rand book](https://rust-random.github.io/book/print.html)). **[V]** Value-breaking changes are allowed in minor versions ([reproducibility policy](https://rust-random.github.io/book/crate-reprod.html)). **[D]** Pin exact versions of `rand_chacha` and `rand_distr`, and add a golden-value regression test (supports SYS-002). |
| YAML (ADR-0001 check) | **[V]** `serde_yaml` was archived and deprecated 2024-03-25 ([release](https://redirect.github.com/dtolnay/serde-yaml/releases)). Its fork `serde_yml` is also deprecated and had a soundness advisory ([docs.rs](https://docs.rs/serde_yml/)). A maintained fork, `yaml_serde`, now exists under the YAML organization ([docs.rs](https://docs.rs/crate/yaml_serde/latest)). **This confirms ADR-0001: RON stays.** |

## 6. Remaining open items
- OQ-4 (Project ↔ repo sync): closed 2026-11-25. The Project's GitHub source (`main`, whole repo) is searchable by path and matched the repo at PR #3. Synced files are not listed as docs and can't be read by name. Sync is manual ("Sync now"). Propagation of a later commit was not tested.
- The TNO mechanic is undocumented (§1.1). Sean will describe the intended feel at the WS-03 kickoff.
