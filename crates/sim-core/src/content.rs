//! Content loading: only what ARC-010 and ARC-016 need (M4b). Each kind reads the fields M4b
//! uses and ignores the rest; `deny_unknown_fields`, full validation, all-errors reporting and
//! the content hash are M4d (ARC-050–053). Load order: scenario → map → setting → core (§3.6).
use crate::state::{EdgeKind, OrbitRegime, PolityKind};
use serde::Deserialize;
use serde::de::{DeserializeOwned, IgnoredAny};
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ContentError {
    Io { file: PathBuf, message: String },
    Parse { file: PathBuf, message: String },
    UnknownRef { file: PathBuf, id: String },
}

impl fmt::Display for ContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io { file, message } => write!(f, "{}: {message}", file.display()),
            Self::Parse { file, message } => write!(f, "{}: {message}", file.display()),
            Self::UnknownRef { file, id } => write!(f, "{}: unknown ID {id}", file.display()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Scenario {
    pub id: String,
    pub map: String,
    pub setting: String,
}

#[derive(Debug, Deserialize)]
pub struct Setting {
    pub default_high: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Polity")]
pub struct PolityDef {
    pub id: String,
    pub kind: PolityKind,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Region")]
pub struct RegionDef {
    pub id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "RegionEdge")]
pub struct RegionEdgeDef {
    pub kind: EdgeKind,
    pub cost: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Body")]
pub struct BodyDef {
    pub id: String,
    pub nodes: Vec<NodeDef>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Node")]
pub struct NodeDef {
    pub regime: OrbitRegime,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Edge")]
pub struct TransferDef {
    pub from: String,
    pub to: String,
    pub dv: f64,
    pub months: u16,
    #[serde(default)]
    pub one_way: bool,
}

/// An entry of `core/rnd/lines/<domain>.ron` (WS-05 §6.2). M4b reads singleton techs only.
#[derive(Debug, Deserialize)]
enum LineEntry {
    Tech { id: String },
}

#[derive(Debug, Default, Deserialize)]
struct BodyDeposits {
    by_body: Vec<(String, String, IgnoredAny)>,
}

/// Loaded content, entries in file order (worldgen sorts by ID).
#[derive(Debug)]
pub struct Content {
    pub scenario: Scenario,
    pub setting: Setting,
    pub polities: Vec<PolityDef>,
    pub regions: Vec<RegionDef>,
    pub edges: Vec<(String, String, RegionEdgeDef)>,
    pub bodies: Vec<BodyDef>,
    pub transfers: Vec<TransferDef>,
    pub tech_ids: Vec<String>,
    /// One deposit per Earth resource tag (WS-08b G-8).
    pub earth_deposits: usize,
    /// One deposit per `body_deposits.ron` rule whose body is in the map (WS-08b G-8).
    pub off_earth_deposits: usize,
}

impl Content {
    pub fn load(root: &Path, scenario_id: &str) -> Result<Self, ContentError> {
        let scenario: Scenario = read(&root.join("scenarios").join(format!("{scenario_id}.ron")))?;
        let map = root.join("maps").join(&scenario.map);
        let sol = root.join("maps/sol");
        let setting_dir = root.join("settings").join(&scenario.setting);
        let regions = read(&map.join("regions.ron"))?;
        let edges = read(&map.join("edges.ron"))?;
        let earth: Vec<(String, Vec<String>)> = read_optional(&map.join("resources.ron"))?;
        let bodies: Vec<BodyDef> = read(&sol.join("bodies.ron"))?;
        let transfers = read_optional(&sol.join("transfers.ron"))?;
        let setting = read(&setting_dir.join("setting.ron"))?;
        let polities = read_dir_each(&setting_dir.join("polities"))?;
        let lines: Vec<Vec<LineEntry>> = read_dir_each(&root.join("core/rnd/lines"))?;
        let deposits: BodyDeposits = read_optional(&root.join("core/resources/body_deposits.ron"))?;
        let off_earth_deposits = deposits
            .by_body
            .iter()
            .filter(|(body, _, _)| bodies.iter().any(|b| &b.id == body))
            .count();
        Ok(Self {
            scenario,
            setting,
            polities,
            regions,
            edges,
            transfers,
            tech_ids: lines
                .into_iter()
                .flatten()
                .map(|LineEntry::Tech { id }| id)
                .collect(),
            earth_deposits: earth.iter().map(|(_, tags)| tags.len()).sum(),
            off_earth_deposits,
            bodies,
        })
    }
}

fn read<T: DeserializeOwned>(file: &Path) -> Result<T, ContentError> {
    let text = std::fs::read_to_string(file).map_err(|e| ContentError::Io {
        file: file.into(),
        message: e.to_string(),
    })?;
    ron::from_str(&text).map_err(|e| ContentError::Parse {
        file: file.into(),
        message: e.to_string(),
    })
}

/// Absent file → `T::default()`.
fn read_optional<T: DeserializeOwned + Default>(file: &Path) -> Result<T, ContentError> {
    if file.exists() {
        read(file)
    } else {
        Ok(T::default())
    }
}

/// One `T` per `.ron` file in `dir`, in file-name order. Absent dir → empty.
fn read_dir_each<T: DeserializeOwned>(dir: &Path) -> Result<Vec<T>, ContentError> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Ok(Vec::new());
    };
    let mut files: Vec<PathBuf> = entries
        .filter_map(|e| Some(e.ok()?.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "ron"))
        .collect();
    files.sort();
    files.iter().map(|f| read(f)).collect()
}
