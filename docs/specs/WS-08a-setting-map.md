# WS-08a — Setting, Canon & Earth Map
Status: Draft v0.2 · Parent: ConOps §3, §4, P-6, P-7, SYS-006, SYS-007; ADR-0003; WS-02 §3.6, §6, ARC-050–052 · Last updated: 2026-11-27

Tags: **[V]** verified (source given) · **[R]** recalled or secondary source, not checked against DATE primary material · **[D]** design choice.
The companion canon document is `docs/setting/date/canon.md`. Draft content lives in `content/maps/earth/`, `content/settings/date/` and `content/scenarios/date_1955.ron`. Until M4 it is validated by the prototype `prototypes/ws08a-content-check/`.

> Orchestra uses material adapted from the U.S. Army Decisive Action Training Environment (DATE), a public-domain U.S. Government work. Orchestra is not affiliated with, sponsored by, or endorsed by the U.S. Army, the Department of Defense, or the U.S. Government.

---

## 1. Purpose & scope

WS-08a defines the default setting: an expanded DATE world on real Earth geography from 1955 to 2060. It covers the polity roster, the 1945–1955 back-history (the canon), each polity's qualitative start state, the Earth region graph, the 1955 ownership map, and the rules that keep setting and map pure data (SYS-007).

**In scope (MVP):** the roster of 47 polities and the high-fidelity candidate pool; canon with provenance tags; the start-state schema in qualitative bands; the region graph (110 land regions, 22 sea zones, 366 edges) with terrain, edge kinds and chokepoints; ownership, subjects, claims, region status and alliances at 1955-01; disclaimer placement; extensibility checks; a list of event hooks, as documentation only.

**Out of scope:** the Solar System and resources (WS-08b); any event system (hooks are listed in canon §6 only); polygon map art (WS-09); the numbers behind the start-state bands (WS-03, WS-04, WS-05, WS-06, WS-11); creating polities at runtime (post-MVP, D2).

## 2. Decisions

| # | Decision | Options considered | Trade-offs | Choice |
|---|---|---|---|---|
| D1 | Polity count and high-fidelity pool | (a) 29 DATE only; (b) ~35 with heavy merging; (c) ~47: all 29 DATE plus originals for the gaps | (a) leaves no North America, Western Europe, Japan, Arab world or Latin America, so there is no Western bloc. (b) drops DATE micro-states that are cheap as macro polities. (c) fills the board and stays close to the WS-02 bench (46). | **(c) 47 polities**: 29 DATE, 5 legacy-OE names, 13 originals. High-fidelity candidates (user-chosen): Atlantica, Donovia, Olvana, Himaldesh, Ariana, Sunda Baru, Hesperia, Lorande, Hinode. Default AI high-fidelity set: Atlantica, Donovia, Himaldesh, Olvana. |
| D2 | Projecting DATE's near-present framing back to 1955 | (a) DATE borders at 1955; (b) back-cast with every polity present at t0, many as subjects; (c) back-cast with runtime spawning | (a) has no decolonization or bloc formation and is anachronistic. (c) is the most faithful but needs runtime polity creation, which is new WS-02 machinery. (b) keeps play while independence is only a change of relation. | **(b)** (user). DATE events dated after 1955 (Gabal 1977, Sunda Baru 1985 [V]) become event hooks. |
| D3 | Region granularity | Uniform-area grid vs. strategy-weighted regions | A uniform grid wastes regions on Siberia, the Sahara and open ocean. | **Strategy-weighted, ~100 land + ~20 sea** (user). Regions are concentrated in the Caucasus (5), the Black Sea rim, the central European front, Luzon and the Southeast Asian straits (18), the Gulf and the chokepoints. Siberia, the Sahara, Antarctica and the oceans are merged. |
| D4 | Naming and provenance | Originals only vs. reuse of legacy OE names | Legacy names are recognisable and keep DATE's flavour; originals have cleaner provenance. | **Legacy OE names where the role fits, originals elsewhere** (user). DATE names are used verbatim. Regions take real physical-geography names. Three provenance tags (`Date`, `DateLegacy`, `Orchestra`) apply in data and in canon. |
| D5 | Start-state schema | Numeric values now vs. qualitative bands | Numbers now would pre-empt WS-03, WS-04, WS-05 and WS-11 and need rework. | **Qualitative enums and 5-level bands.** Each field names the workstream that maps it to numbers (§3.5). |
| D6 | Torbia footprint | Keep DATE's Luzon placement vs. move Torbia to Korea | Moving it departs from DATE's geography. | **Keep Luzon** (user). Korea goes to the original polity Kyeran. Olvana holds all of mainland Olvana, and an island rump (the Republic of Olvana) holds Formosa. |
| D7 | Air movement | Explicit `Air` edges vs. reach derived from distance | Explicit edges would explode combinatorially or be arbitrary. | **Derived.** No `Air` edge kind; air reach = great-circle distance between centroids against range (WS-07, WS-11). |
| D8 | Validator before M4 | Defer "loads under the validator" to M4 vs. a prototype loader now | Deferring leaves the schema untested. | **Prototype loader** in `prototypes/` (Rust, not the Python/MATLAB default, because the Rust types are the schema per ADR-0001). M4 ports the types and tests into `sim-core`. |

## 3. Interfaces

### 3.1 Content layout (extends WS-02 §6)

```
content/scenarios/date_1955.ron         Scenario { id, map: "earth", setting: "date", start: (1955,1), end: (2060,12), overrides }
content/maps/earth/regions.ron          [Region]                       — geography only
content/maps/earth/edges.ron            [(from, to, RegionEdge)]       — geography only
content/settings/date/setting.ron       Setting manifest
content/settings/date/polities/<stem>.ron   one Polity per file; id = "polity.<stem>"
content/settings/date/ownership.ron     [(region, polity)]             (WS-02 shape)
content/settings/date/status.ron        [(region, RegionStatus)]
content/settings/date/claims.ron        [(region, claimant, ClaimKind)]
content/settings/date/subjects.ron      [(subject, overlord, SubjectKind)]
content/settings/date/alliances.ron     [Alliance]
content/settings/date/start.ron         [StartState]                   (keyed by `polity`)
content/settings/date/region_names.ron  [(region, display name, Provenance)]  setting-level aliases
```

Load order and validation follow WS-02 §3.6. Setting files reference map region IDs and never define geography. Map files never mention polities (ARC-052).

### 3.2 Map schema

```rust
enum Terrain { Plains, Forest, Jungle, Desert, Steppe, Hills, Mountain, Tundra, Ice,   // land
               Ocean, Littoral, IceSea }                                             // sea
struct Region { id, name, lat: f64, lon: f64, terrain: Terrain, island: bool = false,
                // reserved, unset in MVP draft (WLD-047):
                pop_weight: Option<f64>,        // WS-03: share of owner's population
                resource_tags: Vec<String>,     // WS-03 vocabulary
                port_class: Option<u8>,         // WS-11
                airfield_class: Option<u8> }    // WS-11
enum EdgeKind { Land, Strait, Coast, SeaLane }  // no Air (D7)
struct RegionEdge { kind: EdgeKind, cost: u32 /* km */, chokepoint: Vec<RegionId> = [] }
```

Edge semantics [D]:

| Kind | Joins | Meaning for consumers |
|---|---|---|
| `Land` | land–land | Ground movement and supply |
| `Strait` | land–land across narrow water | Ground crossing with an amphibious or bridging penalty (WS-07/11) |
| `Coast` | land–sea | Port access and naval support; landings |
| `SeaLane` | sea–sea | Naval movement and trade routes. `chokepoint` lists the land regions whose owners can contest passage. |

A region is a sea zone if and only if its terrain is `Ocean`, `Littoral` or `IceSea`. `cost` is the great-circle distance between centroids in km. Movement multipliers by kind and terrain belong to WS-07 and WS-11.

### 3.3 Setting schema

```rust
struct Setting  { id, name, version, map: String, disclaimer: String, default_high: Vec<PolityId> }
// region_names.ron: [(RegionId, display name, Provenance)]
struct Polity   { id, name, short_name, adjective, provenance: Provenance, analog_role: String,
                  kind: PolityKind /* NationState */, capital: RegionId, hf_candidate: bool = false }
enum Provenance { Date, DateLegacy, Orchestra }
enum RegionStatus { Colonial, Occupied, Trust, Unowned }      // for directly held regions only
enum ClaimKind { Contested, Irredentist }
enum SubjectKind { ConstituentRepublic, Colony, Protectorate, TrustTerritory, Condominium, Commonwealth }
struct Alliance { id, name, kind: AllianceKind /* Defense | Bloc | Association */, provenance, members }
```

`analog_role` is a design note for authors and the AI archetype (WS-06) and is never shown as a real-world mapping. `region_names` lets a setting rename map regions for display without touching the map.

### 3.4 Start-state schema (D5)

```rust
enum Band { VeryLow, Low, Medium, High, VeryHigh }          // totally ordered
struct StartState {
    polity: PolityId,
    government: Government,   // SingleParty | Personalist | MilitaryJunta | AbsoluteMonarchy |
                              // ConstitutionalMonarchy | PresidentialRepublic | ParliamentaryRepublic |
                              // ColonialAdministration
    economy: Economy { size: Band, development: Band, system: EconSystem /* Market | Mixed | Planned */ },
    tech:    Tech    { level: Band, nuclear: Nuclear /* NoProgram < Program < Tested < Arsenal */ },
    forces:  Forces  { size: Band, navy: Band, air: Band, posture: Posture
                       /* Defensive | Deterrent | Expeditionary | Internal | Minimal */ },
    stance:  Stance,          // StrongWest < West < Neutral < East < StrongEast
    stability: Band,
}
```

### 3.5 Field ownership: who turns bands into numbers

Each field has exactly one **primary** owner, who defines the band → number mapping. Other workstreams may *consume* the result but don't redefine it.

| Field | Meaning at 1955-01 | Primary owner (consumers) | Consumer obligation |
|---|---|---|---|
| `government` | Institutional archetype | **WS-04**: archetype → government-type content ID (selectorate S/W, veto players) | Every variant maps to exactly one gov type |
| `economy.size` | Share of world GDP | **WS-03** | Monotone band → GDP range; world total matches WS-03's 1955 calibration |
| `economy.development` | GDP per capita | **WS-03** | Monotone band → per-capita range; with `size`, implies population |
| `economy.system` | Market, mixed or planned | **WS-03** (WS-04) | Sets state-sector share and planning flags |
| `tech.level` | General technological standing | **WS-05** | Band → initial known techs and belief σ |
| `tech.nuclear` | Weapons status | **WS-11** (WS-05 runs `Program` as an R&D project) | `Program` = a running R&D project; `Tested` = a small stockpile |
| `forces.size`, `navy`, `air` | Relative force scale | **WS-11** | Band → count of era-preset templates (§6.1 echelons) |
| `forces.posture` | Default deployment stance | **WS-11** (WS-06 uses it as an AI prior) | Maps to WS-11 posture orders |
| `stance` | Bloc alignment lean | **WS-06** | AI alignment prior; alliance data is authoritative |
| `stability` | Regime stability | **WS-04** | Band → initial stability and faction tension |

A band never carries a hidden number in content (WLD-021). Two polities in the same band start with identical values unless a consuming workstream adds per-polity overrides in its own content.

### 3.6 Outputs to other workstreams

- **WS-02/M4:** the schema types above move into `sim-core::content`. The WLD tests move with them, keeping their names.
- **WS-04:** the subject relations are the first use of a polity-to-polity hierarchy, and independence is a change of relation. Constituent republics have `SingleParty` government and `Minimal` forces.
- **WS-06:** alliances, claims, chokepoints and `stance`. A chokepoint owner's access policy is a WS-06 treaty or AI concern.
- **WS-07 / WS-11:** edge kinds, terrain classes, `cost` in km, and derived air reach (D7); `port_class` and `airfield_class` reserved.
- **WS-09:** `setting.disclaimer` on the credits screen; `region_names` as display aliases; provenance shown in a polity tooltip.

## 4. Model

There is no time-stepped model. WS-08a supplies initial conditions and a static graph. Rules [D]:

1. **Projection (D2).** Every polity exists at tick 0 and owns at least one region. A subject polity owns its own regions; its overlord relation is a separate fact. A future event ends a relation and never creates a polity.
2. **Ownership.** Each land region has exactly one owner at start, or is `Unowned` (Antarctica only). Sea zones are never owned. Colonies without a polity of their own are regions held directly by the metropole, flagged `Colonial`.
3. **Subjects.** An overlord is never itself a subject, so there are no chains or cycles in the MVP. Only a `Condominium` has more than one overlord, and it needs at least two.
4. **Edge cost.** `cost = round(haversine(centroid_a, centroid_b))`, with R = 6371 km.
5. **Air reach.** `air_distance(a, b) = haversine(centroid_a, centroid_b)`. WS-07 and WS-11 compare it to aircraft range; no graph search is used.

## 5. Requirements

Verification column: **C** = prototype checker test (named `wld_NNN_*` in `prototypes/ws08a-content-check/tests/wld.rs`); **M4** = sim-core test once the loader exists; **R** = grader review.

| ID | Requirement | Acceptance criterion (testable) | Trace | Ver. |
|---|---|---|---|---|
| **Setting & roster** |||||
| WLD-001 | The default scenario `date_1955` selects map `earth` and setting `date`, runs 1955-01 to 2060-12, and loads with no errors. | `load(content, "date_1955")` returns Ok; the setting's map equals the scenario's map; start = (1955,1) and end = (2060,12). | SYS-001, SYS-007 | C |
| WLD-002 | The roster has 40–50 polities and includes all 29 DATE OE polities under their DATE names with provenance `Date`. | Polity count ∈ [40, 50]; the set of DATE-name values of `Date` polities equals the 29-name list in §5.1. Sunda Baru's DATE name is checked by ID because its 1955 short name is "Sunda". | ConOps §3, ADR-0003 | C |
| WLD-003 | Each polity is one file, `polities/<stem>.ron`, with `id = "polity.<stem>"` and every §3.3 field. | A fixture whose ID differs from its file name fails with "does not match file name"; a missing field fails to parse. | WS-02 §6, ARC-050 | C |
| WLD-004 | Exactly the nine user-chosen polities are high-fidelity candidates. `default_high` is Atlantica, Donovia, Himaldesh and Olvana, all of them candidates, with at most 5 entries. | The `hf_candidate` set equals the D1 list; the `default_high` set equals the 4 listed; a non-candidate in `default_high` is an error. | P-7, SYS-006, ConOps §3 | C |
| WLD-005 | Every nation-state is playable. The player's chosen polity starts at High tier; the AI High set is `default_high` minus the player. | M4: for 3 sample player choices (one in `default_high`, two not), the tick-0 tiers match the rule and the High count ≤ 6. | SYS-001, SYS-006 | M4 |
| WLD-006 | No polity `name` or `short_name` contains the name of a real sovereign state as a whole word or phrase. | Checked against `tests/real_state_names.txt` (UN members plus major 1945–60 states); no matches. | ConOps §4, canon §7 | C |
| WLD-007 | Each polity's capital is a land region it owns, and each polity owns at least one region. | A fixture that reassigns Lusora's only region reports both "capital … is not owned" and "owns no region". | SYS-007 | C |
| **Ownership & relations** |||||
| WLD-010 | Every land region has exactly one owner or is `Unowned`; sea zones have no owner; all references resolve. | A fixture adding a sea zone and a second owner for Crimea reports "is not a land region" and "owned more than once"; a land region with no owner and no `Unowned` status is an error. | WS-02 §3.6 | C |
| WLD-011 | `Colonial`, `Occupied` and `Trust` apply only to regions held directly by a non-subject polity; `Unowned` regions have no owner. | A fixture marking a subject's region `Colonial` reports "held by subject"; a fixture giving Antarctica an owner reports "Unowned but has an owner". | §4 rule 2 | C |
| WLD-012 | Claims reference known land regions and polities, never claim one's own region, and are unique. The default setting has ≥ 12 claims, including Kashmir (Daryastan), the Eastern Lesser Caucasus (Limaria), Formosa (Olvana) and the mutual Torbian claims. | The default content passes; a fixture where Himaldesh claims Kashmir reports "claims its own region". | ConOps §3, DATE [V] | C |
| WLD-013 | Subject relations obey §4 rule 3. | A fixture with an overlord that is itself a subject, and a non-condominium with 2 overlords, reports both errors. | §4 | C |
| WLD-014 | The default setting makes Atropia, Gorgas, Limaria and Pirtuni constituent republics of Donovia, has ≥ 9 other subject relations, and makes Gabal a two-power condominium. | Set and count assertions on the default content. | D2, canon §4.2 | C |
| WLD-015 | Alliances have unique `alliance.<snake>` IDs and ≥ 2 distinct known members, and use the `AllianceKind` vocabulary. | A fixture with an unknown member reports "unknown member"; a duplicate ID is reported. | WS-06 interface | C |
| WLD-016 | The default setting has the 11 alliances in canon §4.3, including the Northern Compact, the Union Bloc, the Pacific Compact and the Donovia–Olvana treaty. | ID set and count assertions. | canon §3 | C |
| **Start state** |||||
| WLD-020 | Each polity has exactly one `StartState`, and there are no entries for unknown polities. | A fixture deleting Ziwa's entry reports "no entry for polity.ziwa"; duplicates and unknown polities are reported. | WS-02 §6 | C |
| WLD-021 | `StartState` has exactly the §3.4 fields and holds enums only: no numeric values. | A fixture adding `morale` fails in start.ron naming the field; a fixture with `stability: 0.5` fails to parse. | D5, ARC-050 | C |
| WLD-022 | Each start-state field has exactly one primary owning workstream that maps it to numbers (§3.5). | R: the §3.5 table lists every §3.4 field with exactly one primary owner. Later, each owning spec (WS-03, 04, 05, 06, 11) cites the field it maps; that half is checked when those specs are graded. | D5 | R |
| WLD-023 | Start state matches subject status: constituent republics are `SingleParty` with `Minimal` forces; colonies, protectorates, trust territories and condominium subjects are `ColonialAdministration`. | A fixture setting Ziwa to `SingleParty` reports "inconsistent with Colony". | §3.6 WS-04 | C |
| WLD-024 | Nuclear status at 1955-01: only Atlantica, Donovia and Hesperia are `Tested` or above; only Arnland and Lorande are at `Program`. | Set assertions on the default content. | canon §3 | C |
| WLD-025 | Bands, `Nuclear` and `Stance` are totally ordered, so consumers can map them monotonically. | A unit test checks VeryLow < Low < Medium < High < VeryHigh; M4 consumers test that their mapping is monotone. | D5 | C, M4 |
| **Canon & provenance** |||||
| WLD-030 | Every timeline item (canon §3) and event hook (§6) carries exactly one of [DATE], [DATE-legacy] or [ORIG]. Per ADR-0003 all 1945–1955 back-history is [ORIG], and an item that projects a DATE fact cites it (F-n in canon §1). A hook is [DATE] only if DATE dates it. | A parser test over canon.md finds ≥ 30 items, each with exactly one tag, and no [DATE] tag in §3. | ADR-0003 provenance | C |
| WLD-031 | Canon has an entry for every polity in the setting, keyed by its ID. | Every `polity.<id>` appears in canon.md in backticks. | ADR-0003 | C |
| WLD-032 | Canon explains every alliance and every subject relation. | Every `alliance.<id>` appears; each subject → overlord pair appears on one line of canon §4. | canon §3–4 | C |
| WLD-033 | Canon §6 lists ≥ 15 event hooks, each with a non-empty window, hook text, effect and provenance tag. It includes the DATE-dated Gabal independence (1977) and Sunda Baru federation (1985) as [DATE] hooks, and neither event is in the start state. | Parser test on canon §6; WLD-014 shows Gabal is still a subject at start. | D2 | C |
| WLD-034 | Every polity and alliance entry carries a `provenance` value from the vocabulary. | A fixture with `provenance: Fictional` fails to parse. | ADR-0003 | C |
| **Disclaimer & naming** |||||
| WLD-035 | Every file of a setting begins (within its first 6 lines) with a comment header that contains the setting's own `disclaimer` text verbatim and a provenance-legend line (`Provenance tags:`). The scenario file carries the disclaimer, and so does canon.md. For `date` the disclaimer is the ADR-0003 text (WLD-036); other settings supply their own. | A fixture with the header stripped from claims.ron reports "missing disclaimer/provenance header"; a fixture without the disclaimer in date_1955.ron reports "missing disclaimer header"; a test checks canon.md contains the disclaimer; the `testworld` fixture (WLD-049) passes with its own disclaimer. | ADR-0003 | C |
| WLD-036 | `setting.disclaimer` equals the ADR-0003 text exactly; the README carries the same text; the WS-09 credits screen displays `setting.disclaimer`. | Test: string equality with the README and ADR-0003. WS-09: a credits-screen test checks for the string. | ADR-0003 | C, WS-09 |
| WLD-037 | IDs follow `region.<snake>`, `polity.<snake>` and `alliance.<snake>`. Default region names and IDs are physical geography: they contain no polity short name and no real-state name from the WLD-006 deny list, except standard feature names in the carve-out (Congo, Guinea, Madagascar, Ceylon, Iceland, Australia, China, Japan, Mexico). | A fixture with `alliance.UnionBloc` is rejected. Tests over the default content find no region name containing a polity short name, and no region name or ID word sequence matching a deny-list entry outside the carve-out. | P-6, canon §7 | C |
| WLD-038 | DATE polity names are spelled as DATE spells them; original names are provisional and changeable by data. | Covered by the WLD-002 name list; renaming is covered by WLD-048. | ADR-0003 | C |
| **Earth map** |||||
| WLD-040 | The map has 100 ± 12 land regions and 18–26 sea zones. | Count assertion (draft: 110 land, 22 sea). | ConOps §3, D3 | C |
| WLD-041 | Terrain uses the §3.2 vocabulary; sea terrain marks sea zones; sea zones are never islands. | An unknown terrain fails to parse; a fixture with an island sea zone is rejected. | WS-07 interface | C |
| WLD-042 | Edges use `Land`/`Strait` (land–land), `Coast` (land–sea) and `SeaLane` (sea–sea) only, with no self-loops or duplicate pairs. There is no `Air` kind; air reach is derived (§4 rule 5). | A fixture turning a Coast edge into Land reports "wrong region domains"; the `EdgeKind` enum has no `Air` variant. | D7 | C |
| WLD-043 | Edge `cost` equals the great-circle distance between centroids in km, ±1. | haversine((0,0),(0,90)) = 10 007.5 ± 1; a fixture with a corrupted cost is reported. | §4 rule 4 | C |
| WLD-044 | The graph is connected; every sea zone and every island has ≥ 1 Coast edge. | A fixture that removes Hawaii's edges reports "has no Coast edge" and "unreachable". | ARC-015 | C |
| WLD-045 | Chokepoints name land regions. The default map has these chokepoints: the Turkish Straits, Suez, Gibraltar, Bab-el-Mandeb, Hormuz, Malacca, the Danish straits, the Luzon Strait, Tsushima and Panama. | A non-land chokepoint is an error; a set assertion covers the 10 listed. | WS-06, WS-07 | C |
| WLD-046 | No map file contains a polity ID. | A fixture adding "polity.donovia" to regions.ron reports "map file references a polity ID". | ARC-052 | C |
| WLD-047 | The reserved region attributes (`pop_weight`, `resource_tags`, `port_class`, `airfield_class`) are optional and accepted whether absent or present; their meaning belongs to WS-03 and WS-11. | The default content omits them and loads; a fixture setting all four loads. | §3.2 | C |
| **Extensibility (SYS-007)** |||||
| WLD-048 | Adding, renaming or replacing a polity, or adding a region display alias, needs data edits only, and the map stays untouched. | Three fixtures load with no code change: (a) a new polity file plus an ownership change plus a start entry gives N+1 polities; (b) a changed name plus a `region_names` alias loads, the ID is unchanged and the map bytes are identical; (c) deleting Pontara, reassigning its regions, alliance seat and start entry loads without it. | SYS-007, ARC-052 | C |
| WLD-049 | A second setting on the same map, chosen only by a scenario file, loads alongside the default setting. | Fixture `testworld` (3 polities, written as data only) loads with 3 polities, and `date_1955` still loads from the same content tree. | SYS-007 | C |

### 5.1 DATE OE names (29) [V: date.army.gov.au region pages; secondary: Wikipedia "DATE World"]
Eurasia: Ariana, Atropia, Donovia, Gorgas, Limaria, Pirtuni. Indo-Pacific: Bagansait, Belesia, Fanu Rai, Gabal, Himaldesh, Khorathidin, North Torbia, Olvana, Patani Baru, Puller Islands, South Torbia, Sunda Baru, Sungzon, Volcani Tura. Europe (legacy DATE): Arnland, Bothnia, Framland, Otso, Torrike. Africa: Amari, Kujenga, Nyumba, Ziwa.

DATE facts used as constraints, with confidence: South Torbia holds most of southern Luzon [V]; Gabal emerged from the joint rule of two Western European countries in 1977 [V]; the Sunda Baru Federation formed in 1985 from Indonesia plus Malayan provinces and calls Patani Baru illegitimate [V]; Volcani Tura holds eastern New Guinea [V]; each Torbia claims the other and the Song family has ruled North Torbia since its founding [V: DATE MNF-OBA JIPOE]; Olvana is a communist would-be regional hegemon with the region's largest military and disputes territory with South Torbia [V: DATE Indo-Pacific overview; MNF-OBA JIPOE]; Himaldesh is a nonaligned, nuclear-armed democracy [V]; Bagansait is junta-led [V]; Belesia's independence from colonial rule was turbulent [V]; Donovia was its region's dominant power until about two decades before DATE's present [V]; Olvana covers eastern and south-central China, Arnland, Framland and Torrike share Sweden's footprint, and South Torbia ranks among the wealthiest economies [R: secondary summaries].

## 6. Content/data

The draft files listed in §3.1 are complete for the MVP start state:

| File | Entries |
|---|---|
| `maps/earth/regions.ron` | 132 (110 land, 22 sea) |
| `maps/earth/edges.ron` | 366 (Land, Strait, Coast, SeaLane; 19 with chokepoints) |
| `settings/date/polities/*.ron` | 47 (29 `Date`, 5 `DateLegacy`, 13 `Orchestra`) |
| `settings/date/ownership.ron` | 109 land regions owned; Antarctica `Unowned` |
| `settings/date/subjects.ron` | 14 rows (13 subjects; Gabal has 2 overlords) |
| `settings/date/claims.ron` | 19 |
| `settings/date/alliances.ron` | 11 |
| `settings/date/start.ron` | 47 |
| `settings/date/status.ron` | 14 |
| `settings/date/region_names.ron` | 1 (Pineland, `DateLegacy`) |

**Derived map:** `docs/setting/date/map-1955.svg`, rendered from the content by `prototypes/ws08a-map-render/render_map.py` (Voronoi cells per landmass on Natural Earth 1:50m land, public domain). It is a picture of the data, not data: regenerate it after content changes.

**Disclaimer placement (ADR-0003):** README (already present); `setting.ron` `disclaimer` field, which WS-09 shows on the credits screen; the header of every setting and scenario file; and the header of canon.md and of this spec. Map files contain only real geography and carry no DATE material.

## 7. Verification

- `cargo test` in `prototypes/ws08a-content-check/` runs 52 tests, covering every WLD requirement marked C plus ARC-051 (exact error count; no cascade after a polity or setting.ron parse failure).
- `cargo run -- ../../content date_1955` prints `ok: scenario date_1955 — 132 regions, 366 edges, 47 polities`.
- `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check` pass.
- In M4 the schema types and tests move into `sim-core` under the same test names, and the prototype is deleted.

## 8. Open questions

| ID | Question | Proposed resolution |
|---|---|---|
| OQ-W1 | **Closed (accepted 2026-11-27):** ConOps v0.4 allows ~20–45 macro polities; ARC-090 bench updated to 47 polities and 132 regions (WS-02 v1.1). Original question: the ARC-090 bench uses 46 polities and 100 regions; the draft has 47 polities and 132 regions (110 land, 22 sea). With the player and 4 high-fidelity AI, 42 polities are macro, above ConOps §3's "~20–40". | Accept 42 macro polities, since macro polities cost little per tick [D], and update the ARC-090 synthetic scenario to 47 polities and 132 regions (a WS-02 edit and a ConOps note, approved in the Project). Alternative: fold 2–4 DATE micro-states (Puller Islands, Fanu Rai, Gabal, Patani Baru) into their overlords. |
| OQ-W2 | **Closed (accepted):** added to WS-02 §6 in v1.1. Original question: `RegionEdge` gains an optional `chokepoint` field beyond the WS-02 §6 `{ kind, cost }` shape. | Treat it as a compatible extension (default empty) and note it in WS-02 §6 at the next revision. |
| OQ-W3 | **Open.** ODIN is reachable only through the Internet Archive, which Claude's web tools cannot fetch; Sean to attach saved pages. Facts tagged [R] (European footprints, Bothnia's analog, Gabal on Palawan) were not checked against ODIN, which was unreachable from this session. | Verify when ODIN is reachable and adjust canon §5; data changes only. |
| OQ-W4 | **Closed (accepted):** names kept. Legacy OE names (Atlantica, Krasnovia, Centralia, Attica, Ellisia, Pineland) come from training environments whose release status is less clearly documented than DATE's Distribution A [R]. | Only names are used. Keep them unless review says otherwise; the swap is a data edit (WLD-048). |
| OQ-W5 | Directly held colonies (e.g. Hesperia's Guinea Coast) cannot become independent without runtime polity creation. | Post-MVP: either spawn latent polities or pre-create them as subjects. The MVP keeps them as `Colonial` regions. |
| OQ-W6 | **Closed (accepted):** Rust exception for content-schema prototypes. The prototype is Rust; CLAUDE.md names Python or MATLAB for prototypes. | Accept the exception for content-schema prototypes, because the Rust types are the schema (ADR-0001), or move the crate to `crates/` in M4. |
