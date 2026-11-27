//! WS-08a acceptance tests, named after requirement IDs (CLAUDE.md workflow).
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use ws08a_content_check::*;

const SCENARIO: &str = "date_1955";

const DATE_OES: [&str; 29] = [
    "Ariana",
    "Atropia",
    "Donovia",
    "Gorgas",
    "Limaria",
    "Pirtuni", // Eurasia
    "Bagansait",
    "Belesia",
    "Gabal",
    "Himaldesh",
    "Khorathidin",
    "North Torbia",
    "Olvana",
    "Patani Baru",
    "Puller Islands",
    "South Torbia",
    "Sunda Baru",
    "Sungzon",
    "Volcani Tura",
    "Fanu Rai", // Indo-Pacific
    "Arnland",
    "Bothnia",
    "Framland",
    "Otso",
    "Torrike", // Europe (legacy)
    "Amari",
    "Kujenga",
    "Nyumba",
    "Ziwa", // Africa
];

// ───────────── setting & roster ─────────────

#[test]
fn wld_001_default_scenario_loads() {
    let c = real();
    assert_eq!(c.scenario.map, c.setting.map);
    assert_eq!((c.scenario.start, c.scenario.end), ((1955, 1), (2060, 12)));
}

#[test]
fn wld_002_roster_size_and_date_coverage() {
    let c = real();
    assert!(
        (40..=50).contains(&c.polities.len()),
        "{} polities",
        c.polities.len()
    );
    let date: BTreeSet<String> = c
        .polities
        .iter()
        .filter(|p| p.provenance == Provenance::Date)
        .map(canonical_date_name)
        .collect();
    let expected: BTreeSet<String> = DATE_OES.iter().map(|s| s.to_string()).collect();
    assert_eq!(date, expected);
}

#[test]
fn wld_003_polity_file_id_mismatch_rejected() {
    let dir = fixture("wld003");
    edit(
        &dir,
        "settings/date/polities/lusora.ron",
        "\"polity.lusora\"",
        "\"polity.lusoria\"",
    );
    assert_err(&dir, "does not match file name");
}

#[test]
fn wld_004_hf_candidates_and_default_high() {
    let c = real();
    let hf: BTreeSet<&str> = c
        .polities
        .iter()
        .filter(|p| p.hf_candidate)
        .map(|p| short(&p.id))
        .collect();
    let want: BTreeSet<&str> = [
        "atlantica",
        "donovia",
        "olvana",
        "himaldesh",
        "ariana",
        "sunda_baru",
        "hesperia",
        "lorande",
        "hinode",
    ]
    .into();
    assert_eq!(hf, want);
    let high: BTreeSet<&str> = c.setting.default_high.iter().map(|s| short(s)).collect();
    assert_eq!(high, ["atlantica", "donovia", "himaldesh", "olvana"].into());
}

#[test]
fn wld_006_polity_names_are_not_real_states() {
    let c = real();
    let deny = fs::read_to_string(manifest().join("tests/real_state_names.txt")).unwrap();
    for name in deny
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        for p in &c.polities {
            for field in [&p.name, &p.short_name] {
                assert!(
                    !contains_word(field, name),
                    "{}: '{field}' contains real state name '{name}'",
                    p.id
                );
            }
        }
    }
}

#[test]
fn wld_007_capital_and_territory_required() {
    let dir = fixture("wld007");
    edit(
        &dir,
        "settings/date/ownership.ron",
        "(\"region.iberia\", \"polity.lusora\")",
        "(\"region.iberia\", \"polity.lorande\")",
    );
    let errs = errors(&dir);
    assert!(has(
        &errs,
        "polity.lusora capital region.iberia is not owned"
    ));
    assert!(has(&errs, "polity.lusora owns no region"));
}

// ───────────── ownership & relations ─────────────

#[test]
fn wld_010_sea_never_owned_land_owned_once() {
    let dir = fixture("wld010");
    edit(
        &dir,
        "settings/date/ownership.ron",
        "[\n",
        "[\n    (\"region.north_sea\", \"polity.hesperia\"),\n    (\"region.crimea\", \"polity.donovia\"),\n",
    );
    let errs = errors(&dir);
    assert!(has(&errs, "region.north_sea is not a land region"));
    assert!(has(&errs, "region.crimea owned more than once"));
}

#[test]
fn wld_011_status_rules() {
    let dir = fixture("wld011");
    edit(
        &dir,
        "settings/date/status.ron",
        "[\n",
        "[\n    (\"region.absheron\", Colonial),\n",
    );
    edit(
        &dir,
        "settings/date/ownership.ron",
        "[\n",
        "[\n    (\"region.antarctica\", \"polity.hesperia\"),\n",
    );
    let errs = errors(&dir);
    assert!(has(&errs, "held by subject polity.atropia"));
    assert!(has(&errs, "region.antarctica is Unowned but has an owner"));
}

#[test]
fn wld_012_claims() {
    let c = real();
    assert!(c.claims.len() >= 12);
    let have: BTreeSet<(&str, &str)> = c
        .claims
        .iter()
        .map(|(r, p, _)| (short(r), short(p)))
        .collect();
    for want in [
        ("kashmir", "daryastan"),
        ("lesser_caucasus_east", "limaria"),
        ("formosa", "olvana"),
        ("luzon_south", "north_torbia"),
        ("luzon_north", "south_torbia"),
    ] {
        assert!(have.contains(&want), "missing claim {want:?}");
    }
    let dir = fixture("wld012");
    edit(
        &dir,
        "settings/date/claims.ron",
        "[\n",
        "[\n    (\"region.kashmir\", \"polity.himaldesh\", Contested),\n",
    );
    assert_err(&dir, "claims its own region");
}

#[test]
fn wld_013_subject_rules() {
    let dir = fixture("wld013");
    edit(
        &dir,
        "settings/date/subjects.ron",
        "[\n",
        "[\n    (\"polity.otso\", \"polity.atropia\", Colony),\n    (\"polity.ziwa\", \"polity.hesperia\", Colony),\n",
    );
    let errs = errors(&dir);
    assert!(has(&errs, "overlord polity.atropia is itself a subject"));
    assert!(has(&errs, "polity.ziwa: only a Condominium"));
}

#[test]
fn wld_014_default_subjects() {
    let c = real();
    let constituent: BTreeSet<&str> = c
        .subjects
        .iter()
        .filter(|(_, o, k)| *k == SubjectKind::ConstituentRepublic && o == "polity.donovia")
        .map(|(s, _, _)| short(s))
        .collect();
    assert_eq!(
        constituent,
        ["atropia", "gorgas", "limaria", "pirtuni"].into()
    );
    let colonial = c
        .subjects
        .iter()
        .filter(|(_, _, k)| *k != SubjectKind::ConstituentRepublic)
        .count();
    assert!(colonial >= 9, "{colonial}");
    let gabal: Vec<_> = c
        .subjects
        .iter()
        .filter(|(s, _, _)| s == "polity.gabal")
        .collect();
    assert_eq!(
        gabal.len(),
        2,
        "Gabal is a two-power condominium until its DATE-dated 1977 independence"
    );
    assert!(gabal.iter().all(|(_, _, k)| *k == SubjectKind::Condominium));
}

#[test]
fn wld_015_alliance_rules() {
    let dir = fixture("wld015");
    edit(
        &dir,
        "settings/date/alliances.ron",
        "members: [\"polity.donovia\", \"polity.olvana\"]",
        "members: [\"polity.donovia\", \"polity.nowhere\"]",
    );
    assert_err(&dir, "unknown member polity.nowhere");
}

#[test]
fn wld_016_default_alliances() {
    let c = real();
    let ids: BTreeSet<&str> = c.alliances.iter().map(|a| a.id.as_str()).collect();
    for want in [
        "alliance.northern_compact",
        "alliance.union_bloc",
        "alliance.pacific_compact",
        "alliance.donovia_olvana_treaty",
    ] {
        assert!(ids.contains(want), "{want}");
    }
    assert_eq!(c.alliances.len(), 11);
}

// ───────────── start state ─────────────

#[test]
fn wld_020_one_start_state_per_polity() {
    let dir = fixture("wld020");
    let path = dir.join("settings/date/start.ron");
    let text = fs::read_to_string(&path).unwrap();
    let start = text
        .find("    StartState(\n        polity: \"polity.zi")
        .unwrap();
    let end = start + text[start..].find("    ),\n").unwrap() + 7;
    fs::write(&path, format!("{}{}", &text[..start], &text[end..])).unwrap();
    assert_err(&dir, "no entry for polity.ziwa");
}

#[test]
fn wld_021_unknown_start_field_rejected() {
    let dir = fixture("wld021");
    edit(
        &dir,
        "settings/date/start.ron",
        "stability: Medium,",
        "stability: Medium, morale: High,",
    );
    let errs = errors(&dir);
    assert!(
        errs.iter()
            .any(|e| e.contains("start.ron") && e.contains("morale")),
        "{errs:?}"
    );
}

#[test]
fn wld_021_numeric_start_value_rejected() {
    let dir = fixture("wld021n");
    edit(
        &dir,
        "settings/date/start.ron",
        "stability: Medium,",
        "stability: 0.5,",
    );
    assert_err(&dir, "start.ron");
}

#[test]
fn wld_023_subject_start_consistency() {
    let dir = fixture("wld023");
    let path = dir.join("settings/date/start.ron");
    let text = fs::read_to_string(&path).unwrap();
    let i = text.find("polity: \"polity.ziwa\"").unwrap();
    let j = i + text[i..].find("ColonialAdministration").unwrap();
    fs::write(
        &path,
        format!(
            "{}SingleParty{}",
            &text[..j],
            &text[j + "ColonialAdministration".len()..]
        ),
    )
    .unwrap();
    assert_err(&dir, "polity.ziwa: start state inconsistent with Colony");
}

#[test]
fn wld_024_nuclear_status_1955() {
    let c = real();
    let tested: BTreeSet<&str> = c
        .start
        .iter()
        .filter(|s| s.tech.nuclear >= Nuclear::Tested)
        .map(|s| short(&s.polity))
        .collect();
    assert_eq!(tested, ["atlantica", "donovia", "hesperia"].into());
    let program: BTreeSet<&str> = c
        .start
        .iter()
        .filter(|s| s.tech.nuclear == Nuclear::Program)
        .map(|s| short(&s.polity))
        .collect();
    assert_eq!(program, ["arnland", "lorande"].into());
}

#[test]
fn wld_025_band_order_is_total() {
    use Band::*;
    let v = [VeryLow, Low, Medium, High, VeryHigh];
    assert!(v.windows(2).all(|w| w[0] < w[1]));
}

// ───────────── canon & provenance ─────────────

#[test]
fn wld_030_canon_items_tagged() {
    let canon = canon();
    let mut items = 0;
    for line in canon_items(&canon) {
        let tags = ["[DATE]", "[DATE-legacy]", "[ORIG]"]
            .iter()
            .filter(|t| line.contains(**t))
            .count();
        assert_eq!(tags, 1, "needs exactly one provenance tag: {line}");
        items += 1;
    }
    assert!(items >= 30, "{items} tagged items");
    assert!(
        !section(&canon, "## 3.").contains("[DATE]"),
        "ADR-0003: 1945–55 back-history is [ORIG] only"
    );
}

#[test]
fn wld_031_canon_covers_every_polity() {
    let (c, canon) = (real(), canon());
    for p in &c.polities {
        assert!(
            canon.contains(&format!("`{}`", p.id)),
            "canon lacks {}",
            p.id
        );
    }
}

#[test]
fn wld_032_canon_covers_relations() {
    let (c, canon) = (real(), canon());
    for a in &c.alliances {
        assert!(
            canon.contains(&format!("`{}`", a.id)),
            "canon lacks {}",
            a.id
        );
    }
    let rel = section(&canon, "## 4.");
    for (s, o, _) in &c.subjects {
        let line = rel
            .lines()
            .find(|l| l.contains(&format!("`{s}`")) && l.contains(&format!("`{o}`")));
        assert!(line.is_some(), "canon §4 lacks subject relation {s} → {o}");
    }
}

#[test]
fn wld_033_event_hooks() {
    let canon = canon();
    let hooks: Vec<&str> = section(&canon, "## 6.")
        .lines()
        .filter(|l| l.starts_with("| HK-"))
        .collect();
    assert!(hooks.len() >= 15, "{} hooks", hooks.len());
    let gabal = hooks
        .iter()
        .find(|l| l.contains("Gabal"))
        .expect("Gabal hook");
    assert!(gabal.contains("1977") && gabal.contains("[DATE]"));
    let sunda = hooks
        .iter()
        .find(|l| l.contains("Sunda Baru"))
        .expect("Sunda Baru hook");
    assert!(sunda.contains("1985") && sunda.contains("[DATE]"));
    for row in &hooks {
        let cells: Vec<&str> = row.split('|').map(str::trim).collect();
        // | ID | Window | Hook | Effect | Prov |
        assert!(
            cells.len() >= 6 && cells[2..=4].iter().all(|c| !c.is_empty()),
            "incomplete hook row: {row}"
        );
    }
    // Neither DATE-dated event is in the start state.
    let c = real();
    let sunda_p = c
        .polities
        .iter()
        .find(|p| p.id == "polity.sunda_baru")
        .unwrap();
    assert_eq!(sunda_p.short_name, "Sunda");
    let owner = |r: &str| {
        c.ownership
            .iter()
            .find(|(x, _)| x == r)
            .map(|(_, p)| p.as_str())
    };
    assert_eq!(owner("region.malay_south"), Some("polity.hesperia"));
}

#[test]
fn wld_034_provenance_on_every_entry() {
    let dir = fixture("wld034");
    edit(
        &dir,
        "settings/date/polities/otso.ron",
        "provenance: Date,",
        "provenance: Fictional,",
    );
    assert_err(&dir, "otso.ron");
}

// ───────────── disclaimer & naming ─────────────

#[test]
fn wld_035_setting_files_carry_header() {
    let dir = fixture("wld035");
    let path = dir.join("settings/date/claims.ron");
    let text = fs::read_to_string(&path).unwrap();
    let body: String = text
        .lines()
        .filter(|l| !l.starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&path, body).unwrap();
    assert_err(&dir, "claims.ron: missing disclaimer/provenance header");
}

#[test]
fn wld_035_scenario_and_canon_carry_disclaimer() {
    let c = real();
    assert!(
        canon().contains(&c.setting.disclaimer),
        "canon.md lacks the disclaimer"
    );
    let dir = fixture("wld035s");
    edit(&dir, "scenarios/date_1955.ron", &c.setting.disclaimer, "");
    assert_err(&dir, "date_1955.ron: missing disclaimer header");
}

#[test]
fn wld_036_disclaimer_matches_adr() {
    let c = real();
    let readme = fs::read_to_string(repo().join("README.md")).unwrap();
    assert!(
        readme.contains(&c.setting.disclaimer),
        "README disclaimer differs from setting.ron"
    );
    let adr = fs::read_to_string(repo().join("docs/adr/0003-date-setting.md")).unwrap();
    assert!(
        adr.contains(&c.setting.disclaimer),
        "ADR-0003 disclaimer differs from setting.ron"
    );
}

#[test]
fn wld_037_region_names_exclude_polity_names() {
    let c = real();
    for r in &c.regions {
        for p in &c.polities {
            assert!(
                !contains_word(&r.name, &p.short_name),
                "{} name contains polity name {}",
                r.id,
                p.short_name
            );
        }
    }
}

#[test]
fn wld_037_id_convention() {
    let dir = fixture("wld037b");
    edit(
        &dir,
        "settings/date/alliances.ron",
        "\"alliance.union_bloc\"",
        "\"alliance.UnionBloc\"",
    );
    assert_err(&dir, "must be alliance.<snake_case>");
}

// ───────────── map ─────────────

#[test]
fn wld_040_region_counts() {
    let c = real();
    let sea = c.regions.iter().filter(|r| r.terrain.is_sea()).count();
    let land = c.regions.len() - sea;
    assert!((88..=112).contains(&land), "{land} land");
    assert!((18..=26).contains(&sea), "{sea} sea");
}

#[test]
fn wld_041_sea_zone_cannot_be_island() {
    let dir = fixture("wld041");
    edit(
        &dir,
        "maps/earth/regions.ron",
        "terrain: IceSea)",
        "terrain: IceSea, island: true)",
    );
    assert_err(&dir, "sea zone flagged as island");
}

#[test]
fn wld_042_edge_domain_rules() {
    let dir = fixture("wld042");
    edit(
        &dir,
        "maps/earth/edges.ron",
        "(\"region.alaska\", \"region.arctic\", RegionEdge(kind: Coast",
        "(\"region.alaska\", \"region.arctic\", RegionEdge(kind: Land",
    );
    assert_err(&dir, "Land edge joins the wrong region domains");
}

#[test]
fn wld_043_edge_cost_is_great_circle() {
    assert!((haversine_km((0.0, 0.0), (0.0, 90.0)) - 10007.5).abs() < 1.0);
    let dir = fixture("wld043");
    let path = dir.join("maps/earth/edges.ron");
    let text = fs::read_to_string(&path).unwrap();
    let i = text.find("(\"region.alaska\", \"region.arctic\"").unwrap();
    let j = i + text[i..].find("cost: ").unwrap() + 6;
    fs::write(&path, format!("{}9{}", &text[..j], &text[j..])).unwrap();
    assert_err(&dir, "region.alaska–region.arctic: cost");
}

#[test]
fn wld_044_connectivity_and_coasts() {
    let dir = fixture("wld044");
    let path = dir.join("maps/earth/edges.ron");
    let text = fs::read_to_string(&path).unwrap();
    let kept: Vec<&str> = text
        .lines()
        .filter(|l| !l.contains("\"region.hawaii\""))
        .collect();
    fs::write(&path, kept.join("\n")).unwrap();
    let errs = errors(&dir);
    assert!(has(&errs, "region.hawaii has no Coast edge"));
    assert!(has(&errs, "region.hawaii unreachable"));
}

#[test]
fn wld_045_required_chokepoints() {
    let c = real();
    let lanes: Vec<BTreeSet<&str>> = c
        .edges
        .iter()
        .filter(|(_, _, e)| !e.chokepoint.is_empty())
        .map(|(_, _, e)| e.chokepoint.iter().map(|s| short(s)).collect())
        .collect();
    let required: [&[&str]; 10] = [
        &["anatolia_west"],
        &["nile_lower"],
        &["iberia", "maghreb"],
        &["arabia_west", "horn"],
        &["arabia_gulf", "iranian_plateau"],
        &["malay_south", "sumatra"],
        &["jutland", "svealand"],
        &["luzon_north", "formosa"],
        &["korean_peninsula", "honshu_south"],
        &["central_isthmus"],
    ];
    for want in required {
        let want: BTreeSet<&str> = want.iter().copied().collect();
        assert!(lanes.contains(&want), "missing chokepoint {want:?}");
    }
}

#[test]
fn wld_046_map_has_no_polity_refs() {
    let dir = fixture("wld046");
    edit(
        &dir,
        "maps/earth/regions.ron",
        "[\n",
        "// owned by polity.donovia\n[\n",
    );
    assert_err(&dir, "map file references a polity ID");
}

#[test]
fn wld_047_reserved_region_attributes_optional() {
    let dir = fixture("wld047");
    edit(
        &dir,
        "maps/earth/regions.ron",
        "terrain: Steppe),\n    Region(id: \"region.anatolia_west\"",
        "terrain: Steppe, pop_weight: Some(0.4), resource_tags: [\"oil\"], port_class: Some(2), airfield_class: Some(1)),\n    Region(id: \"region.anatolia_west\"",
    );
    load(&dir, SCENARIO).expect("reserved attributes accepted when present");
}

// ───────────── extensibility (SYS-007) ─────────────

#[test]
fn wld_048_add_polity_with_data_only() {
    let dir = fixture("wld048a");
    let n = real().polities.len();
    let set = dir.join("settings/date");
    let otso = fs::read_to_string(set.join("polities/otso.ron")).unwrap();
    fs::write(
        set.join("polities/karelia.ron"),
        otso.replace("polity.otso", "polity.karelia")
            .replace("Republic of Otso", "Karelian Republic")
            .replace("\"Otso\"", "\"Karelia\"")
            .replace("Otsoan", "Karelian")
            .replace("region.lakeland", "region.karelia_kola"),
    )
    .unwrap();
    edit(
        &dir,
        "settings/date/ownership.ron",
        "(\"region.karelia_kola\", \"polity.donovia\")",
        "(\"region.karelia_kola\", \"polity.karelia\")",
    );
    append_start_copy(&set, "polity.otso", "polity.karelia");
    let c = load(&dir, SCENARIO).unwrap_or_else(|e| panic!("{e:?}"));
    assert_eq!(c.polities.len(), n + 1);
}

#[test]
fn wld_048_rename_polity_and_region_alias() {
    let dir = fixture("wld048r");
    edit(
        &dir,
        "settings/date/polities/hesperia.ron",
        "\"Kingdom of Hesperia\"",
        "\"Hesperian Realm\"",
    );
    edit(
        &dir,
        "settings/date/region_names.ron",
        "]\n",
        "    (\"region.crimea\", \"Tavrida\", Orchestra),\n]\n",
    );
    let c = load(&dir, SCENARIO).unwrap_or_else(|e| panic!("{e:?}"));
    assert!(
        c.polities
            .iter()
            .any(|p| p.id == "polity.hesperia" && p.name == "Hesperian Realm")
    );
    let map_before = fs::read(content().join("maps/earth/regions.ron")).unwrap();
    assert_eq!(
        fs::read(dir.join("maps/earth/regions.ron")).unwrap(),
        map_before,
        "map untouched"
    );
}

#[test]
fn wld_048_replace_polity() {
    // Remove Pontara: Ausonia takes its regions; its Northern Compact seat is dropped.
    let dir = fixture("wld048x");
    let set = dir.join("settings/date");
    fs::remove_file(set.join("polities/pontara.ron")).unwrap();
    edit_all(
        &dir,
        "settings/date/ownership.ron",
        "\"polity.pontara\"",
        "\"polity.ausonia\"",
    );
    edit(
        &dir,
        "settings/date/alliances.ron",
        ", \"polity.pontara\"",
        "",
    );
    remove_start(&set, "polity.pontara");
    let c = load(&dir, SCENARIO).unwrap_or_else(|e| panic!("{e:?}"));
    assert!(c.polities.iter().all(|p| p.id != "polity.pontara"));
}

#[test]
fn wld_049_swap_setting_on_same_map() {
    // A second setting with 3 polities, written as data only, selected by scenario.
    let dir = fixture("wld049");
    let set = dir.join("settings/testworld");
    fs::create_dir_all(set.join("polities")).unwrap();
    let hdr = "// testworld fixture. Provenance tags: Orchestra.\n// Test World disclaimer.\n"
        .to_string();
    let land: Vec<String> = real()
        .regions
        .iter()
        .filter(|r| !r.terrain.is_sea())
        .map(|r| r.id.clone())
        .collect();
    let names = ["north", "middle", "south"];
    let mut own = String::from("[\n");
    for (i, r) in land.iter().enumerate() {
        own += &format!("    (\"{r}\", \"polity.{}\"),\n", names[i % 3]);
    }
    own += "]\n";
    fs::write(set.join("ownership.ron"), format!("{hdr}{own}")).unwrap();
    let mut start = String::from("[\n");
    for (i, n) in names.iter().enumerate() {
        fs::write(
            set.join(format!("polities/{n}.ron")),
            format!(
                "{hdr}Polity(id: \"polity.{n}\", name: \"{n} land\", short_name: \"{n}\", adjective: \"{n}ern\", provenance: Orchestra, analog_role: \"test\", kind: NationState, capital: \"{}\")\n",
                land[i]
            ),
        )
        .unwrap();
        start += &format!(
            "    StartState(polity: \"polity.{n}\", government: ParliamentaryRepublic, economy: (size: Medium, development: Medium, system: Market), tech: (level: Medium, nuclear: NoProgram), forces: (size: Low, navy: Low, air: Low, posture: Defensive), stance: Neutral, stability: Medium),\n"
        );
    }
    start += "]\n";
    fs::write(set.join("start.ron"), format!("{hdr}{start}")).unwrap();
    for f in [
        "status.ron",
        "claims.ron",
        "subjects.ron",
        "alliances.ron",
        "region_names.ron",
    ] {
        let body = if f == "status.ron" {
            "[(\"region.antarctica\", Unowned)]\n"
        } else {
            "[]\n"
        };
        fs::write(set.join(f), format!("{hdr}{body}")).unwrap();
    }
    // antarctica must not be owned in this world
    let own_path = set.join("ownership.ron");
    let kept: Vec<String> = fs::read_to_string(&own_path)
        .unwrap()
        .lines()
        .filter(|l| !l.contains("antarctica"))
        .map(String::from)
        .collect();
    fs::write(&own_path, kept.join("\n")).unwrap();
    fs::write(
        set.join("setting.ron"),
        format!("{hdr}Setting(id: \"testworld\", name: \"Test World\", version: \"0\", map: \"earth\", disclaimer: \"Test World disclaimer.\", default_high: [])\n"),
    )
    .unwrap();
    fs::write(
        dir.join("scenarios/testworld.ron"),
        "// Test World disclaimer.\nScenario(id: \"testworld\", map: \"earth\", setting: \"testworld\", start: (1955, 1), end: (2060, 12))\n",
    )
    .unwrap();
    let c = load(&dir, "testworld").unwrap_or_else(|e| panic!("{e:?}"));
    assert_eq!(c.polities.len(), 3);
    load(&dir, SCENARIO).expect("default setting still loads alongside");
}

#[test]
fn arc_051_all_errors_in_one_pass() {
    let dir = fixture("arc051");
    edit(
        &dir,
        "settings/date/claims.ron",
        "[\n",
        "[\n    (\"region.kashmir\", \"polity.nowhere\", Contested),\n",
    );
    edit(
        &dir,
        "settings/date/alliances.ron",
        "\"alliance.union_bloc\"",
        "\"alliance.northern_compact\"",
    );
    edit(
        &dir,
        "settings/date/polities/otso.ron",
        "region.lakeland",
        "region.atlantis",
    );
    let errs = errors(&dir);
    assert!(has(&errs, "unknown region or polity"));
    assert!(has(&errs, "duplicate id alliance.northern_compact"));
    assert!(has(
        &errs,
        "polities/otso.ron: polity.otso capital region.atlantis"
    ));
    assert_eq!(errs.len(), 3, "exactly the 3 seeded errors: {errs:#?}");
}

#[test]
fn arc_051_setting_parse_failure_does_not_cascade() {
    let dir = fixture("arc051s");
    edit(&dir, "settings/date/setting.ron", "version:", "verzion:");
    let errs = errors(&dir);
    assert_eq!(errs.len(), 1, "only the parse error: {errs:#?}");
    assert!(has(&errs, "setting.ron"));
}

#[test]
fn arc_051_parse_failure_does_not_cascade() {
    let dir = fixture("arc051p");
    edit(&dir, "settings/date/polities/otso.ron", "name:", "nmae:");
    let errs = errors(&dir);
    assert_eq!(errs.len(), 1, "only the parse error: {errs:#?}");
    assert!(has(&errs, "otso.ron"));
}

// ───────────── negative cases (grader follow-up) ─────────────

#[test]
fn wld_003_missing_field_rejected() {
    let dir = fixture("wld003m");
    edit(
        &dir,
        "settings/date/polities/otso.ron",
        "    adjective: \"Otsoan\",\n",
        "",
    );
    assert_err(&dir, "otso.ron");
}

#[test]
fn wld_004_default_high_rules() {
    let dir = fixture("wld004");
    edit(
        &dir,
        "settings/date/setting.ron",
        "\"polity.olvana\"]",
        "\"polity.olvana\", \"polity.otso\", \"polity.hesperia\"]",
    );
    let errs = errors(&dir);
    assert!(has(
        &errs,
        "default_high polity.otso is not an hf_candidate"
    ));
    assert!(has(&errs, "default_high exceeds 5"));
}

#[test]
fn wld_010_unowned_land_needs_status() {
    let dir = fixture("wld010u");
    edit(
        &dir,
        "settings/date/ownership.ron",
        "    (\"region.hellas\", \"polity.ausonia\"),\n",
        "",
    );
    assert_err(&dir, "land region region.hellas has no owner");
}

#[test]
fn wld_020_duplicate_and_unknown_start_entries() {
    let dir = fixture("wld020d");
    let set = dir.join("settings/date");
    append_start_copy(&set, "polity.otso", "polity.otso");
    append_start_copy(&set, "polity.ziwa", "polity.nowhere");
    let errs = errors(&dir);
    assert!(has(&errs, "duplicate entry for polity.otso"));
    assert!(has(&errs, "unknown polity polity.nowhere"));
}

#[test]
fn wld_025_nuclear_and_stance_order() {
    use Nuclear::*;
    use Stance::*;
    assert!(NoProgram < Program && Program < Tested && Tested < Arsenal);
    assert!(StrongWest < West && West < Neutral && Neutral < East && East < StrongEast);
}

#[test]
fn wld_041_unknown_terrain_rejected() {
    let dir = fixture("wld041t");
    edit(
        &dir,
        "maps/earth/regions.ron",
        "terrain: Tundra",
        "terrain: Swamp",
    );
    assert_err(&dir, "regions.ron");
}

#[test]
fn wld_045_chokepoint_must_be_land() {
    let dir = fixture("wld045");
    edit(
        &dir,
        "maps/earth/edges.ron",
        "chokepoint: [\"region.anatolia_west\"]",
        "chokepoint: [\"region.black_sea\"]",
    );
    assert_err(&dir, "chokepoint region.black_sea is not a land region");
}

#[test]
fn wld_037_region_names_and_ids_are_geographic() {
    // Physical features that share a name with a state are allowed (spec WLD-037 carve-out).
    let carve_out = [
        "Congo",
        "Guinea",
        "Madagascar",
        "Ceylon",
        "Iceland",
        "Australia",
        "China",
        "Japan",
        "Mexico",
    ];
    let c = real();
    let deny = fs::read_to_string(manifest().join("tests/real_state_names.txt")).unwrap();
    for name in deny
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        if carve_out.contains(&name) {
            continue;
        }
        for r in &c.regions {
            let id_words = r.id.trim_start_matches("region.").replace('_', " ");
            assert!(
                !contains_word(&r.name, name),
                "{}: name '{}' contains '{name}'",
                r.id,
                r.name
            );
            assert!(
                !contains_word(&id_words, name),
                "{}: id contains '{name}'",
                r.id
            );
        }
    }
}

// ───────────── helpers ─────────────

fn manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
fn repo() -> PathBuf {
    manifest().join("../..")
}
fn content() -> PathBuf {
    repo().join("content")
}
fn real() -> Content {
    load(&content(), SCENARIO).unwrap_or_else(|e| panic!("default content fails: {e:#?}"))
}
fn canon() -> String {
    fs::read_to_string(repo().join("docs/setting/date/canon.md")).expect("canon.md")
}
fn short(id: &str) -> &str {
    id.split_once('.').map_or(id, |(_, s)| s)
}
/// DATE name of a polity: its short name, except where the 1955 short name predates the DATE name.
fn canonical_date_name(p: &Polity) -> String {
    match p.id.as_str() {
        "polity.sunda_baru" => "Sunda Baru".into(),
        _ => p.short_name.clone(),
    }
}
/// Bullets in §3 (timeline) and rows in §6 (hooks) of canon.md.
fn canon_items(canon: &str) -> Vec<String> {
    let mut v: Vec<String> = section(canon, "## 3.")
        .lines()
        .filter(|l| l.starts_with("- "))
        .map(String::from)
        .collect();
    v.extend(
        section(canon, "## 6.")
            .lines()
            .filter(|l| l.starts_with("| HK-"))
            .map(String::from),
    );
    v
}
fn section<'a>(text: &'a str, heading: &str) -> &'a str {
    let i = text
        .find(heading)
        .unwrap_or_else(|| panic!("no section {heading}"));
    let rest = &text[i + heading.len()..];
    &rest[..rest.find("\n## ").unwrap_or(rest.len())]
}
fn errors(dir: &Path) -> Vec<String> {
    load(dir, SCENARIO).err().unwrap_or_default()
}
fn has(errs: &[String], needle: &str) -> bool {
    let hit = errs.iter().any(|e| e.contains(needle));
    if !hit {
        eprintln!("no error containing {needle:?} in {errs:#?}");
    }
    hit
}
fn assert_err(dir: &Path, needle: &str) {
    assert!(has(&errors(dir), needle));
}
fn edit(dir: &Path, rel: &str, from: &str, to: &str) {
    let path = dir.join(rel);
    let text = fs::read_to_string(&path).unwrap();
    assert!(text.contains(from), "{rel} lacks {from:?}");
    fs::write(&path, text.replacen(from, to, 1)).unwrap();
}
fn edit_all(dir: &Path, rel: &str, from: &str, to: &str) {
    let path = dir.join(rel);
    let text = fs::read_to_string(&path).unwrap();
    assert!(text.contains(from), "{rel} lacks {from:?}");
    fs::write(&path, text.replace(from, to)).unwrap();
}
fn fixture(name: &str) -> PathBuf {
    let dst = std::env::temp_dir().join(format!("ws08a-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dst);
    let src = content();
    for f in files_recursive(&src) {
        let to = dst.join(f.strip_prefix(&src).unwrap());
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&f, &to).unwrap();
    }
    dst
}
fn start_block(text: &str, polity: &str) -> (usize, usize) {
    let i = text
        .find(&format!("    StartState(\n        polity: \"{polity}\""))
        .unwrap();
    (i, i + text[i..].find("    ),\n").unwrap() + 7)
}
fn append_start_copy(set: &Path, from: &str, to: &str) {
    let path = set.join("start.ron");
    let text = fs::read_to_string(&path).unwrap();
    let (i, j) = start_block(&text, from);
    let block = text[i..j].replace(from, to);
    let end = text.rfind(']').unwrap();
    fs::write(&path, format!("{}{}{}", &text[..end], block, &text[end..])).unwrap();
}
fn remove_start(set: &Path, polity: &str) {
    let path = set.join("start.ron");
    let text = fs::read_to_string(&path).unwrap();
    let (i, j) = start_block(&text, polity);
    fs::write(&path, format!("{}{}", &text[..i], &text[j..])).unwrap();
}
