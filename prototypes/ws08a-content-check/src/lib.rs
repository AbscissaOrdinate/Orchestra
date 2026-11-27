//! WS-08a content loader and validator (prototype).
//!
//! The Rust types are the schema (ADR-0001). Loading follows WS-02 §3.6: scenario → map → setting,
//! unknown fields rejected (ARC-050), every error collected in one pass (ARC-051), setting and map
//! kept separate (ARC-052). M4 moves these types into `sim-core`; this crate is then deleted.
#![forbid(unsafe_code)]

use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

/// Marker every setting file header must contain alongside the setting's own disclaimer (WLD-035).
pub const PROVENANCE_LEGEND: &str = "Provenance tags:";

// ───────────────────────────── scenario ─────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub id: String,
    pub map: String,
    pub setting: String,
    pub start: (i32, u32),
    pub end: (i32, u32),
    #[serde(default)]
    pub overrides: Vec<String>,
}

// ───────────────────────────── map ─────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Terrain {
    Plains,
    Forest,
    Jungle,
    Desert,
    Steppe,
    Hills,
    Mountain,
    Tundra,
    Ice,
    Ocean,
    Littoral,
    IceSea,
}

impl Terrain {
    pub fn is_sea(self) -> bool {
        matches!(self, Terrain::Ocean | Terrain::Littoral | Terrain::IceSea)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub terrain: Terrain,
    #[serde(default)]
    pub island: bool,
    // Reserved attributes (WLD-047): semantics owned by WS-03 / WS-11, unset in the MVP draft.
    // Resource tags are not here: they live in maps/<map>/resources.ron (WS-08b §4.4).
    #[serde(default)]
    pub pop_weight: Option<f64>,
    #[serde(default)]
    pub port_class: Option<u8>,
    #[serde(default)]
    pub airfield_class: Option<u8>,
}

/// No `Air` kind: air reach is derived from centroid distance (WLD-042).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum EdgeKind {
    Land,
    Strait,
    Coast,
    SeaLane,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegionEdge {
    pub kind: EdgeKind,
    pub cost: u32,
    #[serde(default)]
    pub chokepoint: Vec<String>,
}

pub type Edge = (String, String, RegionEdge);

// ───────────────────────────── setting ─────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Setting {
    pub id: String,
    pub name: String,
    pub version: String,
    pub map: String,
    pub disclaimer: String,
    pub default_high: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Provenance {
    Date,
    DateLegacy,
    Orchestra,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum PolityKind {
    NationState,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Polity {
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub adjective: String,
    pub provenance: Provenance,
    pub analog_role: String,
    pub kind: PolityKind,
    pub capital: String,
    #[serde(default)]
    pub hf_candidate: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum RegionStatus {
    Colonial,
    Occupied,
    Trust,
    Unowned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum ClaimKind {
    Contested,
    Irredentist,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum SubjectKind {
    ConstituentRepublic,
    Colony,
    Protectorate,
    TrustTerritory,
    Condominium,
    Commonwealth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum AllianceKind {
    Defense,
    Bloc,
    Association,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Alliance {
    pub id: String,
    pub name: String,
    pub kind: AllianceKind,
    pub provenance: Provenance,
    pub members: Vec<String>,
}

// ───────────────────────────── start state (WLD-020..025) ─────────────────────────────

/// Qualitative band; consuming workstreams map bands to numbers (WLD-022, WLD-025).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Band {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Government {
    SingleParty,
    Personalist,
    MilitaryJunta,
    AbsoluteMonarchy,
    ConstitutionalMonarchy,
    PresidentialRepublic,
    ParliamentaryRepublic,
    ColonialAdministration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum EconSystem {
    Market,
    Mixed,
    Planned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Nuclear {
    NoProgram,
    Program,
    Tested,
    Arsenal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Posture {
    Defensive,
    Deterrent,
    Expeditionary,
    Internal,
    Minimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Stance {
    StrongWest,
    West,
    Neutral,
    East,
    StrongEast,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Economy {
    pub size: Band,
    pub development: Band,
    pub system: EconSystem,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tech {
    pub level: Band,
    pub nuclear: Nuclear,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Forces {
    pub size: Band,
    pub navy: Band,
    pub air: Band,
    pub posture: Posture,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StartState {
    pub polity: String,
    pub government: Government,
    pub economy: Economy,
    pub tech: Tech,
    pub forces: Forces,
    pub stance: Stance,
    pub stability: Band,
}

// ───────────────────────────── loaded content ─────────────────────────────

#[derive(Debug, Clone)]
pub struct Content {
    pub scenario: Scenario,
    pub regions: Vec<Region>,
    pub edges: Vec<Edge>,
    pub setting: Setting,
    pub polities: Vec<Polity>,
    pub ownership: Vec<(String, String)>,
    pub status: Vec<(String, RegionStatus)>,
    pub claims: Vec<(String, String, ClaimKind)>,
    pub subjects: Vec<(String, String, SubjectKind)>,
    pub alliances: Vec<Alliance>,
    pub start: Vec<StartState>,
    pub region_names: Vec<(String, String, Provenance)>,
    /// Map resource layer (WS-08b), `[(region, [tag])]`; empty if the map has none.
    pub resources: Vec<(String, Vec<String>)>,
    /// Setting start development per deposit (WLD-017); empty if the file is absent (all deposits start at 0).
    pub deposit_start: Vec<(String, f64)>,
    /// IDs of polity files that failed to parse; references to them don't cascade into more errors (ARC-051).
    pub unparsed_polities: Vec<String>,
    /// False if setting.ron failed to parse; setting-dependent checks are then skipped (ARC-051).
    pub setting_parsed: bool,
}

/// Load and validate a scenario. Returns every error found (ARC-051).
pub fn load(root: &Path, scenario_id: &str) -> Result<Content, Vec<String>> {
    let mut errs = Vec::new();
    let scenario: Option<Scenario> = parse(
        &root.join(format!("scenarios/{scenario_id}.ron")),
        &mut errs,
    );
    let Some(scenario) = scenario else {
        return Err(errs);
    };
    let map_dir = root.join("maps").join(&scenario.map);
    let set_dir = root.join("settings").join(&scenario.setting);

    let regions: Vec<Region> = parse(&map_dir.join("regions.ron"), &mut errs).unwrap_or_default();
    let edges: Vec<Edge> = parse(&map_dir.join("edges.ron"), &mut errs).unwrap_or_default();
    let setting: Option<Setting> = parse(&set_dir.join("setting.ron"), &mut errs);
    let mut polities = Vec::new();
    let mut unparsed_polities = Vec::new();
    for path in ron_files(&set_dir.join("polities"), &mut errs) {
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let Some(p) = parse::<Polity>(&path, &mut errs) else {
            unparsed_polities.push(format!("polity.{stem}"));
            continue;
        };
        {
            if p.id != format!("polity.{stem}") {
                errs.push(format!(
                    "{}: id {} does not match file name",
                    path.display(),
                    p.id
                ));
            }
            polities.push(p);
        }
    }
    let setting_parsed = setting.is_some();
    let content = Content {
        scenario,
        regions,
        edges,
        setting: setting.unwrap_or(Setting {
            id: String::new(),
            name: String::new(),
            version: String::new(),
            map: String::new(),
            disclaimer: String::new(),
            default_high: vec![],
        }),
        polities,
        ownership: parse(&set_dir.join("ownership.ron"), &mut errs).unwrap_or_default(),
        status: parse(&set_dir.join("status.ron"), &mut errs).unwrap_or_default(),
        claims: parse(&set_dir.join("claims.ron"), &mut errs).unwrap_or_default(),
        subjects: parse(&set_dir.join("subjects.ron"), &mut errs).unwrap_or_default(),
        alliances: parse(&set_dir.join("alliances.ron"), &mut errs).unwrap_or_default(),
        start: parse(&set_dir.join("start.ron"), &mut errs).unwrap_or_default(),
        region_names: parse(&set_dir.join("region_names.ron"), &mut errs).unwrap_or_default(),
        resources: parse_optional(&map_dir.join("resources.ron"), &mut errs),
        deposit_start: parse_optional(&set_dir.join("deposit_start.ron"), &mut errs),
        unparsed_polities,
        setting_parsed,
    };
    let scenario_path = root.join(format!("scenarios/{scenario_id}.ron"));
    let disclaimer = setting_parsed.then(|| content.setting.disclaimer.clone());
    check_text(
        &map_dir,
        &set_dir,
        &scenario_path,
        disclaimer.as_deref(),
        &mut errs,
    );
    validate(&content, &mut errs);
    if errs.is_empty() {
        Ok(content)
    } else {
        Err(errs)
    }
}

/// Raw-text checks: map files never mention polities (WLD-046). Setting files carry the setting's own
/// disclaimer and the provenance legend in their header; the scenario file carries the disclaimer (WLD-035).
fn check_text(
    map_dir: &Path,
    set_dir: &Path,
    scenario: &Path,
    disclaimer: Option<&str>,
    errs: &mut Vec<String>,
) {
    for path in files_recursive(map_dir) {
        let text = fs::read_to_string(&path).unwrap_or_default();
        if text.contains("polity.") {
            errs.push(format!(
                "{}: map file references a polity ID (ARC-052)",
                path.display()
            ));
        }
    }
    // Without a parsed setting there is no disclaimer to check against; the parse error stands alone.
    let Some(disclaimer) = disclaimer else { return };
    for path in files_recursive(set_dir) {
        let text = fs::read_to_string(&path).unwrap_or_default();
        let head: String = text.lines().take(6).collect::<Vec<_>>().join("\n");
        if disclaimer.is_empty() || !head.contains(disclaimer) || !head.contains(PROVENANCE_LEGEND)
        {
            errs.push(format!(
                "{}: missing disclaimer/provenance header",
                path.display()
            ));
        }
    }
    let text = fs::read_to_string(scenario).unwrap_or_default();
    if disclaimer.is_empty() || !text.contains(disclaimer) {
        errs.push(format!("{}: missing disclaimer header", scenario.display()));
    }
}

/// Structural and referential rules across map and setting.
pub fn validate(c: &Content, errs: &mut Vec<String>) {
    let s = &c.setting;
    if c.setting_parsed && s.map != c.scenario.map {
        errs.push(format!(
            "setting.ron: map {} != scenario map {}",
            s.map, c.scenario.map
        ));
    }
    if c.setting_parsed && s.id != c.scenario.setting {
        errs.push(format!(
            "setting.ron: id {} != scenario setting {}",
            s.id, c.scenario.setting
        ));
    }
    if c.setting_parsed && s.disclaimer.trim().is_empty() {
        errs.push("setting.ron: disclaimer text missing".into());
    }

    // regions
    let mut region: BTreeMap<&str, &Region> = BTreeMap::new();
    for r in &c.regions {
        check_id(&r.id, "region.", "regions.ron", errs);
        if region.insert(&r.id, r).is_some() {
            errs.push(format!("regions.ron: duplicate id {}", r.id));
        }
        if r.terrain.is_sea() && r.island {
            errs.push(format!("regions.ron: {}: sea zone flagged as island", r.id));
        }
    }
    let is_land = |id: &str| region.get(id).is_some_and(|r| !r.terrain.is_sea());

    // edges
    let mut seen = BTreeSet::new();
    let mut adj: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut coast: BTreeSet<&str> = BTreeSet::new();
    for (a, b, e) in &c.edges {
        let tag = format!("edges.ron: {a}–{b}");
        let (Some(ra), Some(rb)) = (region.get(a.as_str()), region.get(b.as_str())) else {
            errs.push(format!("{tag}: endpoint not a region"));
            continue;
        };
        if a == b || !seen.insert((a.min(b), a.max(b))) {
            errs.push(format!("{tag}: self-loop or duplicate edge"));
        }
        let domain_ok = match e.kind {
            EdgeKind::Land | EdgeKind::Strait => !ra.terrain.is_sea() && !rb.terrain.is_sea(),
            EdgeKind::Coast => ra.terrain.is_sea() != rb.terrain.is_sea(),
            EdgeKind::SeaLane => ra.terrain.is_sea() && rb.terrain.is_sea(),
        };
        if !domain_ok {
            errs.push(format!(
                "{tag}: {:?} edge joins the wrong region domains",
                e.kind
            ));
        }
        let km = haversine_km((ra.lat, ra.lon), (rb.lat, rb.lon));
        if (e.cost as f64 - km).abs() > 1.0 {
            errs.push(format!(
                "{tag}: cost {} != great-circle {:.0} km",
                e.cost, km
            ));
        }
        for cp in &e.chokepoint {
            if !is_land(cp) {
                errs.push(format!("{tag}: chokepoint {cp} is not a land region"));
            }
        }
        if e.kind == EdgeKind::Coast {
            coast.insert(a);
            coast.insert(b);
        }
        adj.entry(a).or_default().push(b);
        adj.entry(b).or_default().push(a);
    }
    for r in &c.regions {
        if (r.terrain.is_sea() || r.island) && !coast.contains(r.id.as_str()) {
            errs.push(format!("edges.ron: {} has no Coast edge", r.id));
        }
    }
    if let Some(first) = c.regions.first() {
        let reached = bfs(first.id.as_str(), &adj);
        for r in &c.regions {
            if !reached.contains(r.id.as_str()) {
                errs.push(format!("edges.ron: {} unreachable from {}", r.id, first.id));
            }
        }
    }

    // polities
    let mut polity: BTreeMap<&str, &Polity> = BTreeMap::new();
    for p in &c.polities {
        check_id(&p.id, "polity.", &polity_file(&p.id), errs);
        if polity.insert(&p.id, p).is_some() {
            errs.push(format!("{}: duplicate id {}", polity_file(&p.id), p.id));
        }
    }
    let unparsed: BTreeSet<&str> = c.unparsed_polities.iter().map(String::as_str).collect();
    let known_polity = |id: &str| polity.contains_key(id) || unparsed.contains(id);
    for id in &s.default_high {
        if !polity.get(id.as_str()).is_some_and(|p| p.hf_candidate) {
            errs.push(format!(
                "setting.ron: default_high {id} is not an hf_candidate polity"
            ));
        }
    }
    if s.default_high.len() > 5 {
        errs.push("setting.ron: default_high exceeds 5 AI high-fidelity polities".into());
    }

    // ownership
    let mut owner: BTreeMap<&str, &str> = BTreeMap::new();
    for (r, p) in &c.ownership {
        if !is_land(r) {
            errs.push(format!("ownership.ron: {r} is not a land region"));
        }
        if !known_polity(p) {
            errs.push(format!("ownership.ron: {r}: unknown polity {p}"));
        }
        if owner.insert(r, p).is_some() {
            errs.push(format!("ownership.ron: {r} owned more than once"));
        }
    }
    let status: BTreeMap<&str, RegionStatus> =
        c.status.iter().map(|(r, k)| (r.as_str(), *k)).collect();
    for r in c.regions.iter().filter(|r| !r.terrain.is_sea()) {
        let unowned = status.get(r.id.as_str()) == Some(&RegionStatus::Unowned);
        match (owner.contains_key(r.id.as_str()), unowned) {
            (false, false) => {
                errs.push(format!("ownership.ron: land region {} has no owner", r.id))
            }
            (true, true) => errs.push(format!("status.ron: {} is Unowned but has an owner", r.id)),
            _ => {}
        }
    }
    for p in &c.polities {
        if owner.get(p.capital.as_str()) != Some(&p.id.as_str()) {
            errs.push(format!(
                "{}: {} capital {} is not owned by it",
                polity_file(&p.id),
                p.id,
                p.capital
            ));
        }
        if !owner.values().any(|o| *o == p.id) {
            errs.push(format!("ownership.ron: {} owns no region", p.id));
        }
    }

    // subjects (WLD-013)
    let subject_ids: BTreeSet<&str> = c.subjects.iter().map(|(s, _, _)| s.as_str()).collect();
    let mut overlords: BTreeMap<&str, Vec<SubjectKind>> = BTreeMap::new();
    for (sub, over, kind) in &c.subjects {
        for id in [sub, over] {
            if !known_polity(id) {
                errs.push(format!("subjects.ron: unknown polity {id}"));
            }
        }
        if sub == over || subject_ids.contains(over.as_str()) {
            errs.push(format!(
                "subjects.ron: {sub}: overlord {over} is itself a subject or the same polity"
            ));
        }
        overlords.entry(sub).or_default().push(*kind);
    }
    for (sub, kinds) in &overlords {
        let condo = kinds.iter().all(|k| *k == SubjectKind::Condominium);
        if (kinds.len() > 1 && !condo) || (condo && kinds.len() < 2) {
            errs.push(format!(
                "subjects.ron: {sub}: only a Condominium has (and needs) several overlords"
            ));
        }
    }

    // status (WLD-011)
    for (r, k) in &c.status {
        if !is_land(r) {
            errs.push(format!("status.ron: {r} is not a land region"));
        }
        if *k != RegionStatus::Unowned {
            match owner.get(r.as_str()) {
                None => errs.push(format!("status.ron: {r} is {k:?} but has no owner")),
                Some(o) if subject_ids.contains(o) => errs.push(format!(
                    "status.ron: {r} is held by subject {o}; status applies to direct holdings only"
                )),
                _ => {}
            }
        }
    }

    // claims (WLD-012)
    let mut claim_seen = BTreeSet::new();
    for (r, p, _) in &c.claims {
        if !is_land(r) || !known_polity(p) {
            errs.push(format!("claims.ron: {r}/{p}: unknown region or polity"));
        }
        if owner.get(r.as_str()) == Some(&p.as_str()) {
            errs.push(format!("claims.ron: {p} claims its own region {r}"));
        }
        if !claim_seen.insert((r, p)) {
            errs.push(format!("claims.ron: duplicate claim {r}/{p}"));
        }
    }

    // alliances (WLD-015)
    let mut alliance_ids = BTreeSet::new();
    for a in &c.alliances {
        check_id(&a.id, "alliance.", "alliances.ron", errs);
        if !alliance_ids.insert(&a.id) {
            errs.push(format!("alliances.ron: duplicate id {}", a.id));
        }
        let members: BTreeSet<&String> = a.members.iter().collect();
        if members.len() < 2 || members.len() != a.members.len() {
            errs.push(format!("alliances.ron: {} needs ≥2 distinct members", a.id));
        }
        for m in &a.members {
            if !known_polity(m) {
                errs.push(format!("alliances.ron: {}: unknown member {m}", a.id));
            }
        }
    }

    // start state (WLD-020, WLD-023)
    let mut start: BTreeMap<&str, &StartState> = BTreeMap::new();
    for st in &c.start {
        if !known_polity(&st.polity) {
            errs.push(format!("start.ron: unknown polity {}", st.polity));
        }
        if start.insert(&st.polity, st).is_some() {
            errs.push(format!("start.ron: duplicate entry for {}", st.polity));
        }
    }
    for p in &c.polities {
        if !start.contains_key(p.id.as_str()) {
            errs.push(format!("start.ron: no entry for {}", p.id));
        }
    }
    for (sub, _, kind) in &c.subjects {
        let Some(st) = start.get(sub.as_str()) else {
            continue;
        };
        let ok = match kind {
            SubjectKind::ConstituentRepublic => {
                st.government == Government::SingleParty && st.forces.posture == Posture::Minimal
            }
            SubjectKind::Commonwealth => true,
            _ => st.government == Government::ColonialAdministration,
        };
        if !ok {
            errs.push(format!(
                "start.ron: {sub}: start state inconsistent with {kind:?} status"
            ));
        }
    }

    // display aliases (WLD-048)
    for (r, name, _) in &c.region_names {
        if !region.contains_key(r.as_str()) || name.trim().is_empty() {
            errs.push(format!("region_names.ron: bad entry {r}"));
        }
    }

    // deposit start levels (WLD-017)
    let deposits = earth_deposits(&c.resources);
    let mut seen_dep = BTreeSet::new();
    for (id, dev) in &c.deposit_start {
        match deposits.get(id.as_str()) {
            None => errs.push(format!("deposit_start.ron: unknown deposit {id}")),
            Some(tag) if tag.split('.').any(|q| q == "unknown") => errs.push(format!(
                "deposit_start.ron: {id} is tagged unknown (found after the start) and cannot be developed"
            )),
            _ => {}
        }
        if !(0.0..=1.0).contains(dev) {
            errs.push(format!("deposit_start.ron: {id}: dev {dev} outside [0, 1]"));
        }
        if !seen_dep.insert(id) {
            errs.push(format!("deposit_start.ron: duplicate entry for {id}"));
        }
    }
}

/// Earth deposit IDs per WS-08b §4.1 G-8: `deposit.<region stem>.<tag>`, with `#2`, `#3`, … on a
/// tag repeated within a region (file order). Maps ID → tag.
pub fn earth_deposits(resources: &[(String, Vec<String>)]) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for (region, tags) in resources {
        let stem = region.strip_prefix("region.").unwrap_or(region);
        let mut count: BTreeMap<&str, usize> = BTreeMap::new();
        for tag in tags {
            let n = count.entry(tag.as_str()).or_insert(0);
            *n += 1;
            let suffix = if *n > 1 {
                format!("#{n}")
            } else {
                String::new()
            };
            out.insert(format!("deposit.{stem}.{tag}{suffix}"), tag.clone());
        }
    }
    out
}

// ───────────────────────────── helpers ─────────────────────────────

/// Great-circle distance between centroids, km (WLD-043). Also the basis for derived air reach (WLD-042).
pub fn haversine_km(p: (f64, f64), q: (f64, f64)) -> f64 {
    let (la1, lo1, la2, lo2) = (
        p.0.to_radians(),
        p.1.to_radians(),
        q.0.to_radians(),
        q.1.to_radians(),
    );
    let h = ((la2 - la1) / 2.0).sin().powi(2)
        + la1.cos() * la2.cos() * ((lo2 - lo1) / 2.0).sin().powi(2);
    2.0 * 6371.0 * h.sqrt().asin()
}

/// Case-insensitive whole-word (or whole-phrase) containment.
pub fn contains_word(haystack: &str, needle: &str) -> bool {
    let words = |s: &str| -> Vec<String> {
        s.split(|ch: char| !ch.is_alphanumeric())
            .filter(|w| !w.is_empty())
            .map(str::to_lowercase)
            .collect()
    };
    let (h, n) = (words(haystack), words(needle));
    !n.is_empty() && h.windows(n.len()).any(|w| w == n.as_slice())
}

fn polity_file(id: &str) -> String {
    format!("polities/{}.ron", id.strip_prefix("polity.").unwrap_or(id))
}

fn check_id(id: &str, prefix: &str, file: &str, errs: &mut Vec<String>) {
    let rest = id.strip_prefix(prefix).unwrap_or("");
    let snake = !rest.is_empty()
        && rest
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_');
    if !snake {
        errs.push(format!("{file}: id {id} must be {prefix}<snake_case>"));
    }
}

fn bfs<'a>(start: &'a str, adj: &BTreeMap<&'a str, Vec<&'a str>>) -> BTreeSet<&'a str> {
    let mut seen = BTreeSet::from([start]);
    let mut queue = VecDeque::from([start]);
    while let Some(n) = queue.pop_front() {
        for &m in adj.get(n).map(Vec::as_slice).unwrap_or(&[]) {
            if seen.insert(m) {
                queue.push_back(m);
            }
        }
    }
    seen
}

fn parse<T: DeserializeOwned>(path: &Path, errs: &mut Vec<String>) -> Option<T> {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            errs.push(format!("{}: {e}", path.display()));
            return None;
        }
    };
    ron::from_str(&text)
        .map_err(|e| errs.push(format!("{}: {e}", path.display())))
        .ok()
}

/// Parse a file that may be absent; absence is not an error.
fn parse_optional<T: DeserializeOwned + Default>(path: &Path, errs: &mut Vec<String>) -> T {
    if path.exists() {
        parse(path, errs).unwrap_or_default()
    } else {
        T::default()
    }
}

fn ron_files(dir: &Path, errs: &mut Vec<String>) -> Vec<PathBuf> {
    match fs::read_dir(dir) {
        Ok(rd) => {
            let mut v: Vec<PathBuf> = rd
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().is_some_and(|x| x == "ron"))
                .collect();
            v.sort();
            v
        }
        Err(e) => {
            errs.push(format!("{}: {e}", dir.display()));
            vec![]
        }
    }
}

pub fn files_recursive(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(rd) = fs::read_dir(&d) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p)
            } else {
                out.push(p)
            }
        }
    }
    out.sort();
    out
}
