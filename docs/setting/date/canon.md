# Expanded DATE World — Canon, 1945–1955 back-history and 1955 start
Status: Draft v0.1 · Parent: WS-08a (`docs/specs/WS-08a-setting-map.md`), ADR-0003 · Last updated: 2026-11-27

> Orchestra uses material adapted from the U.S. Army Decisive Action Training Environment (DATE), a public-domain U.S. Government work. Orchestra is not affiliated with, sponsored by, or endorsed by the U.S. Army, the Department of Defense, or the U.S. Government.

All polities are fictional. Real-world history is used as inspiration only; no polity represents, or makes claims about, a real state or people.

## 1. Provenance and conventions

Every timeline item and event hook carries exactly one tag (WLD-030):

| Tag | Content value | Meaning |
|---|---|---|
| **[DATE]** | `Date` | Adapted from a current DATE operational environment (country names, roles, DATE-stated facts and dates). |
| **[DATE-legacy]** | `DateLegacy` | A name taken from an older U.S. Army training environment that is not one of the 29 DATE OEs: Robin Sage's Atlantica and Pineland, the Fort Irwin OEs (Krasnovia, Mojavia), Quantico's Centralia, and the Joint Modernization Command's Attica and Ellisia. Only the names are used; roles and lore are Orchestra's own. |
| **[ORIG]** | `Orchestra` | Written for Orchestra. Per ADR-0003 this covers **all** 1945–1955 back-history, including items that project a DATE fact onto a date or a place; those cite the fact (F-n) below. |

An event hook (§6) is tagged [DATE] only if DATE itself gives its date (F-2, F-3). A hook whose outcome is DATE's present but whose window Orchestra chose is tagged [ORIG] and cites the DATE fact it moves toward.

**DATE facts relied on.** These are adapted DATE material, kept apart from the Orchestra projections that use them:

| ID | DATE fact | Source |
|---|---|---|
| F-1 | Donovia was its region's dominant political, economic, military and social power until about two decades before DATE's present | [V] DATE Donovia country overview |
| F-2 | Gabal emerged from joint rule by two Western European countries in 1977 | [V] DATE Indo-Pacific overview |
| F-3 | The Sunda Baru Federation was formed in 1985 from Indonesia and several Malaysian provinces, became an expansionist regional power by 2005, and calls Patani Baru illegitimate | [V] DATE Sunda Baru overview |
| F-4 | South Torbia occupies most of southern Luzon. Each Torbia claims the other, and the Song family has ruled North Torbia since its founding | [V] DATE South Torbia overview; DATE OPORD JIPOE |
| F-5 | Belesia is a young democracy whose independence from colonial rule was turbulent | [V] DATE Indo-Pacific overview |
| F-6 | Himaldesh is a nonaligned, nuclear-armed democracy; Bagansait is led by a military junta | [V] DATE overviews |
| F-7 | Olvana is a communist would-be regional hegemon with the region's largest military, and has disputed territory with South Torbia | [V] DATE Olvana overview; DATE OPORD |
| F-8 | Present-day roles of the other DATE polities (Atropia, Gorgas, Limaria, Pirtuni, Ariana, Nyumba, Kujenga, Amari, Ziwa, Fanu Rai, Volcani Tura) as listed in §4.1, and South Torbia's standing among the wealthiest economies | [V] Volcani Tura (DATE); the rest [R], from secondary summaries |

The DATE source is the Australian Army's DATE site (date.army.gov.au) and its published country studies and operation orders. The TRADOC ODIN knowledge base could not be reached when this was written (OQ-C1).

**Projection rule.** DATE describes a near-present world, which is treated as *one plausible 2020s outcome* of the game. In 1955 many DATE polities are not yet sovereign. They exist from tick 0 as **subjects**: constituent republics, colonies, protectorates, trust territories, a condominium or a commonwealth. Independence changes a relation and creates no new polity (WS-08a D2). DATE events dated after 1955 are **event hooks** (§6), not start state.

## 2. The world in January 1955

The World War ended ten years ago. Two victors now lead rival blocs:

- **The Atlantican side.** The United Provinces of Atlantica holds a nuclear arsenal and the world's largest economy and navy. It leads the Northern Compact in the Atlantic and Mediterranean, the Pacific Compact in Asia, and a ring of bilateral treaties around Olvana.
- **The Donovian side.** The Union of Donovian Republics holds the second nuclear arsenal, and its constituent republics include Atropia, Gorgas, Limaria and Pirtuni. Through the Union Bloc it controls Krasnovia and Bothnia. It is allied with revolutionary Olvana, which has just fought Atlantica to a standstill in the Torbian War.

Hesperia and Lorande still hold large empires, but both are exhausted and facing insurgencies. Himaldesh, Sunda, Ellisia and Bagansait lead a new nonaligned current. The defeated industrial powers, Westmark and Hinode, are rebuilding under Atlantican protection. Westmark is about to rearm.

## 3. Timeline 1945–1955

- **1945** [ORIG] The World War ends with the defeat of Hinode, the pre-war Westmark state and Ausonia's wartime regime by a coalition led by Atlantica, Donovia and Hesperia, with the Lorandese resistance.
- **1945** [ORIG] Donovia emerges as its region's dominant political, economic, military and social power, the starting point of F-1.
- **1945** [ORIG] The defeated northern-European state is divided into occupation zones. The western zones become the Federal Republic of Westmark in 1949, and the Donovian zone becomes the Bothnian People's Republic.
- **1945** [ORIG] Kyeran is freed from Hinodese colonial rule. After a short trusteeship it becomes the Republic of Kyeran in 1948.
- **1945** [ORIG] Communist governments across Central Europe and the northern Balkans are merged into the single Krasnovian People's Republic under Donovian tutelage (a composite; see §5).
- **1945** [ORIG] Atlantica occupies Hinode until the 1952 peace treaty and keeps administering the Ryukyu Islands. It also takes the former Hinodese mandate islands of Micronesia as a trust.
- **1946** [ORIG] A Donovian-backed autonomous government in Ariana's Urmia Basin collapses. This is the origin of Atropia's dormant irredentist claim on the basin.
- **1946** [ORIG] Atlantica ends colonial rule over Luzon. Olvanan-backed guerrillas hold northern Luzon, so two rival governments emerge: the Democratic People's Republic of Torbia in the north and the Republic of Torbia in the south. The Visayas and Mindanao stay under Atlantican tutelage as the Commonwealth of Belesia.
- **1946** [ORIG] The Song family takes power in North Torbia (F-4).
- **1947** [ORIG] Hesperia partitions its subcontinental empire into Himaldesh and two-winged Daryastan. The 1947–49 Kashmir war leaves Kashmir held by Himaldesh and claimed by Daryastan.
- **1947** [ORIG] The Hemispheric Defense Pact (`alliance.hemispheric_pact`) binds Atlantica, Centralia and Guaranda.
- **1948** [ORIG] Bagansait gains independence from Hesperia, and ethnic insurgencies begin almost at once.
- **1949** [ORIG] The Olvanan civil war ends. The People's Republic of Olvana holds the whole mainland, and the Republic of Olvana withdraws to Formosa, still claiming the mainland.
- **1949** [ORIG] The Northern Compact (`alliance.northern_compact`) is founded by Atlantica, Hesperia, Lorande, Ausonia, Framland and Torrike. Pontara joins in 1952.
- **1949** [ORIG] Donovia tests its first fission device, ending Atlantica's monopoly.
- **1949** [ORIG] After a four-year war, the Sunda Republic wins independence from Lorande. Lorande keeps Western New Guinea, which Sunda contests.
- **1949** [ORIG] The Hesperian Commonwealth (`alliance.hesperian_commonwealth`) is re-formed as an association of independent members: Hesperia, Meridia, Himaldesh, Daryastan and Drakensia.
- **1950** [ORIG] Donovia and Olvana sign the Treaty of Friendship and Alliance (`alliance.donovia_olvana_treaty`). Olvana occupies the Tibetan plateau in 1950–51.
- **1950** [ORIG] Donovia's bilateral treaties with Krasnovia and Bothnia are grouped as the Union Bloc (`alliance.union_bloc`), which is not yet a formal multilateral pact.
- **1950** [ORIG] The Torbian War begins when North Torbia invades the south. An Atlantican-led coalition intervenes, and Olvanan "volunteers" enter the war late in 1950.
- **1950** [ORIG] Each Torbia claims the whole of Torbia, and neither recognizes the other (F-4).
- **1951** [ORIG] Atlantica signs the Atlantica-Torbia Mutual Defense Treaty (`alliance.torbian_defense_treaty`). The Atlantica–Hinode Security Treaty (`alliance.atlantica_hinode_treaty`) is signed alongside the peace treaty and takes effect in 1952.
- **1951** [ORIG] Ariana nationalizes its oil. In 1953 a royalist restoration backed by Atlantica and Hesperia ends the crisis, and the Shah-analog monarch has ruled by decree since.
- **1952** [ORIG] Army officers overthrow the Ellisian monarchy and proclaim the Ellisian Republic, a pan-national union of the Nile, the Levant and Mesopotamia (a composite).
- **1952** [ORIG] Hesperia tests a fission device. An uprising in Amari brings a Hesperian state of emergency.
- **1953** [ORIG] The Torbian War ends in an armistice along a military demarcation line across central Luzon. Atlantica signs a mutual defense treaty with Kyeran (`alliance.atlantica_kyeran_treaty`). North Torbia and Olvana sign the Torbian-Olvanan Mutual Aid Pact (`alliance.torbian_olvanan_pact`).
- **1953** [ORIG] Donovia's founding dictator dies, and a collective leadership takes over.
- **1954** [ORIG] Sungzon defeats Lorande. A peace conference ends Lorandese rule on the Annamite Coast, and Lorande's former protectorates on the middle Mekong pass to Khorathidin.
- **1954** [ORIG] Donovia transfers Crimea from its core to the Pirtunian Union Republic.
- **1954** [ORIG] The Pacific Compact (`alliance.pacific_compact`) is signed. In December, Atlantica signs a defense treaty with the Republic of Olvana (`alliance.atlantica_formosa_treaty`).
- **1955** [ORIG] Gabal is under the joint rule of Hesperia and Lorande, Orchestra's choice of the two Western European powers in F-2.

## 4. Polities, subjects and alliances at the start

### 4.1 Roster (47)

The high-fidelity candidates are marked ★; Atlantica, Donovia, Himaldesh and Olvana are the default AI high-fidelity set. Every polity is a playable nation-state. "DATE present" is where DATE's near-present describes each polity; it is not a forced outcome.

| ID | 1955 name | 1955 state | DATE present / trajectory | Prov |
|---|---|---|---|---|
| `polity.donovia` ★ | Union of Donovian Republics | Soviet-type union and bloc hegemon; nuclear arsenal | Resurgent former hegemon after decline and breakup | [DATE] |
| `polity.ariana` ★ | Kingdom of Ariana | Restored monarchy ruling by decree; oil power aligned with the West | Revolutionary power hostile to Western influence | [DATE] |
| `polity.atropia` | Atropian Union Republic | Donovian constituent republic; Absheron oil | Independent, oil-rich, corrupt pro-Western state | [DATE] |
| `polity.gorgas` | Gorgan Union Republic | Donovian constituent republic | Small pro-Western state with separatist regions | [DATE] |
| `polity.limaria` | Limarian Union Republic | Donovian constituent republic; claims the Eastern Lesser Caucasus | Landlocked Donovian ally in a territorial conflict with Atropia | [DATE] |
| `polity.pirtuni` | Pirtunian Union Republic | Populous industrial constituent republic; holds Crimea since 1954 | Western-leaning state threatened by Donovia | [DATE] |
| `polity.arnland` | Kingdom of Arnland | Armed-neutral democracy with a weapons program | Independence-minded democracy | [DATE] |
| `polity.framland` | Kingdom of Framland | Northern Compact member on the northern flank | — | [DATE] |
| `polity.torrike` | Kingdom of Torrike | Compact member holding the Danish straits and Greenland–Iceland | — | [DATE] |
| `polity.bothnia` | Bothnian People's Republic | Front-line Donovian satellite in a divided nation | — | [DATE] |
| `polity.otso` | Republic of Otso | Neutral republic on Donovia's border | — | [DATE] |
| `polity.olvana` ★ | People's Republic of Olvana | Revolutionary state holding all of mainland Olvana; Donovia's ally | Communist regional hegemon with the largest military in its region | [DATE] |
| `polity.north_torbia` | Democratic People's Republic of Torbia | Garrison state ruled by the Song family | Totalitarian single-party state with a personality cult | [DATE] |
| `polity.south_torbia` | Republic of Torbia | Poor, authoritarian front-line republic | Wealthy constitutional republic with weak institutions | [DATE] |
| `polity.belesia` | Commonwealth of Belesia | Atlantican commonwealth (Visayas, Mindanao) | Federated States of Belesia, a young federal democracy | [DATE] |
| `polity.gabal` | Gabal Condominium | Ruled jointly by Hesperia and Lorande | Republic of Gabal, independent from 1977 | [DATE] |
| `polity.himaldesh` ★ | Republic of Himaldesh | Large nonaligned democracy | Nuclear-armed nonaligned democracy | [DATE] |
| `polity.bagansait` | Union of Bagansait | Fragile post-colonial union facing insurgency | Republic in name, led by a military junta | [DATE] |
| `polity.khorathidin` | Kingdom of Khorathidin | Military-dominated monarchy aligned with Atlantica | Coup-prone, Atlantica-friendly monarchy | [DATE] |
| `polity.sungzon` | Sungzon Democratic Republic | Victorious anticolonial communist state | Communist state wary of Olvana's maritime power | [DATE] |
| `polity.sunda_baru` ★ | Sunda Republic | Nonaligned archipelagic republic | Sunda Baru Federation (1985), an expansionist regional power | [DATE] |
| `polity.patani_baru` | Patani Protectorate | Hesperian protectorate facing a communist insurgency | Weak decentralized federation that Sunda Baru calls illegitimate | [DATE] |
| `polity.volcani_tura` | Territory of Volcani Tura | Meridian trust territory | Resource-rich, weak Melanesian state | [DATE] |
| `polity.puller_islands` | Puller Islands Colony | Hesperian colony | Small post-conflict island state | [DATE] |
| `polity.fanu_rai` | Fanu Rai | Lusoran colony | Young democracy with offshore oil and gas | [DATE] |
| `polity.amari` | Amari Colony | Hesperian settler colony in revolt | Stable pro-Western democracy | [DATE] |
| `polity.kujenga` | Kujenga Trust Territory | Hesperian trust territory | Dominant-party state drifting toward authoritarianism | [DATE] |
| `polity.nyumba` | Empire of Nyumba | Independent highland empire of the Horn | Failed state with an Islamist government | [DATE] |
| `polity.ziwa` | Ziwa | Mineral-rich Lorandese colony | Mineral-rich, fragile landlocked state | [DATE] |
| `polity.atlantica` ★ | United Provinces of Atlantica | Maritime superpower and Compact leader | — | [DATE-legacy] |
| `polity.centralia` | Centralian Federation | Hemispheric middle power (Mexico, the isthmus, the Antilles, the Andes) | — | [DATE-legacy] |
| `polity.krasnovia` | Krasnovian People's Republic | Donovian satellite belt across Central Europe and the Balkans | — | [DATE-legacy] |
| `polity.attica` | Kingdom of Attica | Oil monarchy allied with Atlantica | — | [DATE-legacy] |
| `polity.ellisia` | Ellisian Republic | Revolutionary pan-national republic holding Suez | — | [DATE-legacy] |
| `polity.hesperia` ★ | Kingdom of Hesperia | Declining maritime empire; nuclear power | — | [ORIG] |
| `polity.lorande` ★ | Republic of Lorande | Unstable colonial republic with a weapons program | — | [ORIG] |
| `polity.westmark` | Federal Republic of Westmark | Divided industrial heartland about to rearm | — | [ORIG] |
| `polity.ausonia` | Ausonian Republic | Mediterranean Compact member (Italian and Hellenic peninsulas) | — | [ORIG] |
| `polity.lusora` | Lusoran State | Authoritarian Iberian regime with late colonies | — | [ORIG] |
| `polity.pontara` | Pontaran Republic | Compact member holding the Straits | — | [ORIG] |
| `polity.hinode` ★ | State of Hinode | Defeated industrial power, protected by Atlantica | — | [ORIG] |
| `polity.kyeran` | Republic of Kyeran | Atlantica-aligned former Hinodese colony | — | [ORIG] |
| `polity.olvana_republic` | Republic of Olvana | Nationalist government-in-exile on Formosa | — | [ORIG] |
| `polity.daryastan` | Dominion of Daryastan | Two-wing partition state; Pacific Compact member | — | [ORIG] |
| `polity.meridia` | Commonwealth of Meridia | Southern dominion; trustee of Volcani Tura | — | [ORIG] |
| `polity.guaranda` | Guarandan Republic | Large developing South American federation | — | [ORIG] |
| `polity.drakensia` | Union of Drakensia | Minority-ruled southern dominion | — | [ORIG] |

### 4.2 Subject relations

- `polity.atropia` → `polity.donovia`: constituent republic. Each of the four republics formally has the right to secede, which drives HK-15. [ORIG]
- `polity.gorgas` → `polity.donovia`: constituent republic. [ORIG]
- `polity.limaria` → `polity.donovia`: constituent republic. [ORIG]
- `polity.pirtuni` → `polity.donovia`: constituent republic. [ORIG]
- `polity.belesia` → `polity.atlantica`: commonwealth, self-governing and on a path to federal independence (HK-10). [ORIG]
- `polity.gabal` → `polity.hesperia` and `polity.lorande`: condominium until 1977 (HK-11; F-2). [ORIG]
- `polity.patani_baru` → `polity.hesperia`: protectorate. [ORIG]
- `polity.puller_islands` → `polity.hesperia`: colony. [ORIG]
- `polity.amari` → `polity.hesperia`: colony under emergency rule. [ORIG]
- `polity.kujenga` → `polity.hesperia`: trust territory. [ORIG]
- `polity.ziwa` → `polity.lorande`: colony. [ORIG]
- `polity.fanu_rai` → `polity.lusora`: colony. [ORIG]
- `polity.volcani_tura` → `polity.meridia`: trust territory. [ORIG]

Directly held colonial regions, which have no polity of their own in the MVP, are listed in `status.ron`. Hesperia holds the Guinea Coast, the Upper Nile, the Zambezi Basin, Ceylon, the Southern Malay Peninsula and North Borneo. Lorande holds the Maghreb, the Sahara and Sahel, the Congo Basin and Bie Plateau, Madagascar and Western New Guinea. Atlantica occupies the Ryukyus and holds the Marianas and Carolines as a trust. Antarctica is unowned.

### 4.3 Alliances in force

`alliance.northern_compact` · `alliance.union_bloc` · `alliance.donovia_olvana_treaty` · `alliance.torbian_olvanan_pact` · `alliance.pacific_compact` · `alliance.atlantica_hinode_treaty` · `alliance.atlantica_kyeran_treaty` · `alliance.atlantica_formosa_treaty` · `alliance.hemispheric_pact` · `alliance.hesperian_commonwealth` · `alliance.torbian_defense_treaty`. §3 gives the origin of each.

### 4.4 Territorial claims

Contested claims: Kashmir (Daryastan), the Eastern Lesser Caucasus (Limaria), Formosa (Olvana), Southern Luzon (North Torbia), Northern Luzon (South Torbia) and Western New Guinea (Sunda). Irredentist claims: all six mainland regions (Republic of Olvana), the Southern Malay Peninsula and North Borneo (Sunda), the Elbe Plain (Westmark), the Rhine Basin (Bothnia), the Urmia Basin (Atropia), the Upper Nile (Ellisia) and the Ryukyus (Hinode).

## 5. DATE footprint adaptations

DATE's geography is mapped onto real Earth with these deliberate choices [D]:

- **Torbia stays on Luzon, as in DATE.** South Torbia holds most of southern Luzon [V: DATE South Torbia overview]. The Korean peninsula therefore goes to the original polity Kyeran.
- **Olvana** holds all of mainland Olvana in 1955 (north, centre, south, Manchuria, Tibet and Tarim), more than DATE's present-day footprint. The Republic of Olvana holds only Formosa.
- **Arnland, Framland and Torrike** share Sweden's footprint in DATE [R]. Here they are spread across Sweden, Norway and Denmark/Greenland–Iceland so that each has its own board presence. **Otso** sits on Finland [R]. **Bothnia** is the Donovian half of a divided nation on the Elbe; the East Germany analog is attested only secondarily [R, low confidence].
- **Belesia** holds the Visayas and Mindanao; DATE places a theatre entry point at General Santos City [V]. **Gabal** holds Palawan [R].
- **Sunda Baru** in DATE is Indonesia plus several Malayan provinces, united in 1985 [V]. In 1955 it is the Sunda Republic (Java, Sumatra, Kalimantan and Celebes), and the 1985 union is HK-13. Its content ID is already `polity.sunda_baru`, so the rename is a data change.
- **Khorathidin** includes the middle Mekong, following DATE's Thailand-plus footprint [R].
- **Nyumba**, DATE's Horn-of-Africa composite failed state, is projected back as the Horn's independent highland empire.
- **Composites** cover gaps with few regions: Krasnovia (Central Europe and the Balkans), Ausonia (Italy and Greece), Centralia (Mexico to the Andes), Ellisia (Nile, Levant and Mesopotamia), Guaranda (Brazil and the Plate). Each can be split later with data only (WLD-048).

## 6. Event hooks (for later event workstreams)

Hooks are content for a future event system. In the MVP they are documentation only. The windows are soft.

| ID | Window | Hook | Effect if it fires | Prov (basis) |
|---|---|---|---|---|
| HK-01 | 1955-05 | Westmark joins the Northern Compact and begins rearming | Membership change; Westmark forces grow | [ORIG] |
| HK-02 | 1955-05 | The Union Bloc becomes a formal multilateral pact | Alliance kind Bloc → Defense | [ORIG] |
| HK-03 | 1955-04 | Conference of nonaligned states (Himaldesh, Sunda, Ellisia, Bagansait) | Creates a nonaligned association | [ORIG] |
| HK-04 | 1956–1960 | Ellisia nationalizes the Suez canal company | Access crisis at the Suez chokepoint | [ORIG] |
| HK-05 | 1957–1965 | Decolonization wave: Patani Baru, Amari, Kujenga, Ziwa, Puller Islands | Subject relations end | [ORIG] |
| HK-06 | 1957–1962 | First artificial satellite opens the space race | Unlocks WS-08b space content | [ORIG] |
| HK-07 | 1959–1965 | Olvanan–Donovian split | Treaty of Friendship lapses | [ORIG] |
| HK-08 | 1960–1970 | Olvana tests a fission device | Nuclear status changes | [ORIG] |
| HK-09 | 1962–1969 | Lorande transfers Western New Guinea to Sunda | Ownership change; claim cleared | [ORIG] |
| HK-10 | 1958–1975 | Belesia becomes the Federated States of Belesia, independent amid post-colonial turbulence | Subject relation ends; rename | [ORIG] (F-5) |
| HK-11 | 1977 | Gabal becomes independent from the condominium | Subject relations end; rename to Republic of Gabal | [DATE] (F-2) |
| HK-12 | 1965–1990 | Ariana's monarchy falls to a revolutionary state | Government archetype and stance change | [ORIG] (F-8) |
| HK-13 | 1985 | Sunda Baru Federation: Sunda unites with the Malayan provinces | Rename; the Southern Malay Peninsula and North Borneo change owner | [DATE] (F-3) |
| HK-14 | 1985–2005 | Sunda Baru becomes an expansionist regional power and rejects Patani Baru's legitimacy | Claims harden; military growth | [ORIG] (F-3) |
| HK-15 | 1985–1995 | The Donovian Union dissolves and its four constituent republics become independent | Subject relations end; Donovia declines | [ORIG] (F-1) |
| HK-16 | after HK-15 | Atropia–Limaria war over the Eastern Lesser Caucasus | Contested claim becomes a war | [ORIG] (F-8) |
| HK-17 | after HK-15 | Separatist regions in Gorgas, backed by Donovia | Internal instability; Donovian pressure | [ORIG] (F-8) |
| HK-18 | 2000–2030 | Donovia resurges and pressures Pirtuni and Gorgas | Stance and force posture shift | [ORIG] (F-1) |
| HK-19 | 1958–1965 | A military junta takes power in Bagansait | Government archetype changes | [ORIG] (F-6) |
| HK-20 | 1985–2000 | Nyumba collapses into a fractured state with an Islamist government | Stability collapse; government change | [ORIG] (F-8) |
| HK-21 | 1960–1970 | Kujenga becomes independent as a dominant-party state | Subject relation ends; government set | [ORIG] (F-8) |
| HK-22 | 1960–1975 | Himaldesh tests a fission device | Nuclear status changes | [ORIG] (F-6) |
| HK-23 | 1970–2000 | South Torbia rises to rank among the wealthiest economies | Economic development grows | [ORIG] (F-8) |
| HK-24 | 2000–2030 | Olvana pursues regional hegemony and disputes territory with South Torbia | New claims; naval growth | [ORIG] (F-7) |
| HK-25 | 1974–2005 | Fanu Rai becomes independent and develops its offshore oil and gas | Subject relation ends | [ORIG] (F-8) |
| HK-26 | 1958–1962 | Lorande tests a fission device and seeks its own deterrent | Nuclear status; Compact friction | [ORIG] |
| HK-27 | 1968–1975 | Daryastan's eastern wing secedes | Bengal changes owner (needs runtime polity creation, so post-MVP) | [ORIG] |

## 7. Naming and lore conventions

- **IDs** are stable and never renamed: `polity.<snake>`, `region.<snake>`, `alliance.<snake>`. Display names are data and can change by edit or event.
- **DATE names** are spelled exactly as DATE spells them (WLD-038). DATE dates and facts are tagged [DATE] and cited in this document or the spec.
- **Original names** are invented. They must not contain the name of a real sovereign state (WLD-006 deny list) and should sound plausible for their region. All original names are provisional, and the user can rename any of them in data.
- **Regions** use real physical-geography names (Kartli, Absheron, Lingnan) and never a polity name or a political name (WLD-037). Features whose standard name coincides with a state (Congo Basin, Guinea Coast, Madagascar, Ceylon, Iceland, Australia, the North China Plain, the Sea of Japan, the Gulf of Mexico) are allowed. IDs follow the same rule. A setting can give a display alias through `region_names.ron`, for example Pineland for the Southeastern Piedmont [DATE-legacy name].
- **Lore depth** is low by design (ConOps §4): one clause of 1955 state per polity, and roughly one line per timeline item or hook.
- **Sensitivity.** The canon leaves out analogs of real genocides and of some real-world ethnic and religious conflicts beyond those DATE itself depicts. Real conflicts are inspiration, never a mapping.

## 8. Open canon questions

| ID | Question |
|---|---|
| OQ-C1 | Open: Sean to attach saved ODIN pages (Internet Archive), then verify the DATE facts tagged [R] (European footprints, Bothnia's analog, Gabal on Palawan, Khorathidin's footprint) against ODIN when it is reachable, and adjust §5. |
| OQ-C2 | **Closed (accepted 2026-11-27):** keep the legacy OE names. Legacy OE names (Atlantica, Krasnovia, Centralia, Attica, Ellisia; Pineland as a region alias) come from Army training environments whose release status is less clearly documented than DATE's Distribution A [R]. Only names are used. Confirm this is acceptable, or swap them for originals by data edit. In the legacy material Atlantica was a Cold War adversary; its role here is reassigned. |
| OQ-C3 | Mojavia and Parumphia are unused and held in reserve for splitting a composite later, such as Centralia. |
