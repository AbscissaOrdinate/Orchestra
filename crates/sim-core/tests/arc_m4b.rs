//! M4b acceptance tests: WS-02 ARC-010–016 (state model) and ARC-020–024 (pipeline).
//! Determinism comparisons are same-platform (WS-02 §7).
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_distr::{Distribution, Normal};
use serde::Serialize;
use sim_core::stubs::Knobs;
use sim_core::subsystems::StubUnit;
use sim_core::*;
use std::path::{Path, PathBuf};

// ── helpers ─────────────────────────────────────────────────────────────────

fn content_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/content")
}

fn load(root: &Path) -> World {
    worldgen(&Content::load(root, "test").unwrap(), 42).unwrap()
}

fn test_world() -> World {
    load(&content_root())
}

fn hash<T: Serialize + ?Sized>(v: &T) -> blake3::Hash {
    blake3::hash(&postcard::to_stdvec(v).unwrap())
}

fn run(world: &mut World, ticks: u32) {
    for _ in 0..ticks {
        step(world, &[]);
    }
}

fn knobs(world: &mut World) -> &mut Knobs {
    &mut world.stub.knobs
}

/// State hash ignoring this tick's events (for "state unchanged" checks).
fn state_hash_without_events(world: &World) -> blake3::Hash {
    let mut w = world.clone();
    w.events.clear();
    hash(&w)
}

fn polity(world: &World, content_id: &str) -> PolityId {
    world
        .polities
        .iter()
        .find(|p| p.content_id == content_id)
        .unwrap()
        .id
}

/// Copy of the test content with every list's one-line entries reversed and the files of each
/// multi-file directory renamed so that their read order reverses.
fn shuffled_copy(name: &str) -> PathBuf {
    let dst = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dst);
    copy_shuffled(&content_root(), &dst);
    dst
}

fn copy_shuffled(src: &Path, dst: &Path) {
    std::fs::create_dir_all(dst).unwrap();
    let mut entries: Vec<_> = std::fs::read_dir(src)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    entries.sort();
    let n = entries.len();
    for (i, path) in entries.iter().enumerate() {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if path.is_dir() {
            copy_shuffled(path, &dst.join(file_name));
            continue;
        }
        let text = std::fs::read_to_string(path).unwrap();
        let mut lines: Vec<&str> = text.lines().collect();
        // A list entry: indented 4, starting with `(` or a struct name (fields start lowercase).
        let entry = |l: &&str| {
            l.starts_with("    ")
                && l[4..].starts_with(|c: char| c == '(' || c.is_ascii_uppercase())
        };
        let positions: Vec<usize> = (0..lines.len()).filter(|&j| entry(&lines[j])).collect();
        let reversed: Vec<&str> = positions.iter().rev().map(|&j| lines[j]).collect();
        for (&j, l) in positions.iter().zip(reversed) {
            lines[j] = l;
        }
        // Reverse file-name order in directories holding several .ron files.
        let renamed = if n > 1 {
            format!("{}_{file_name}", n - i)
        } else {
            file_name.to_string()
        };
        let renamed = if src.ends_with("polities") || src.ends_with("lines") {
            renamed
        } else {
            file_name.to_string()
        };
        std::fs::write(dst.join(renamed), lines.join("\n") + "\n").unwrap();
    }
}

// ── ARC-010 ─────────────────────────────────────────────────────────────────

/// ARC-010: polities come only from data (one per polity file), as one generic type with
/// `kind` and `tier` taken from content. The CI grep half is `ci/arc_010_no_hardcoded_ids.sh`.
#[test]
fn arc_010_polities_from_data_only() {
    let world = test_world();
    let mut ids: Vec<String> = std::fs::read_dir(content_root().join("settings/test/polities"))
        .unwrap()
        .map(|e| {
            let stem = e
                .unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            format!("polity.{stem}")
        })
        .collect();
    ids.sort();
    let loaded: Vec<&str> = world
        .polities
        .iter()
        .map(|p| p.content_id.as_str())
        .collect();
    assert_eq!(loaded, ids);
    for (i, p) in world.polities.iter().enumerate() {
        assert_eq!(p.id, PolityId(i as u16));
        assert_eq!(p.kind, PolityKind::NationState);
        assert!(p.alive);
    }
    let tiers: Vec<Tier> = world.polities.iter().map(|p| p.tier).collect();
    assert_eq!(
        tiers,
        [Tier::High, Tier::High, Tier::Macro],
        "default_high in setting.ron"
    );
    assert_eq!(world.econ.len(), 3);
    assert_eq!(world.beliefs().len(), 3);
}

// ── ARC-011 ─────────────────────────────────────────────────────────────────

/// ARC-011: truth is immutable after worldgen (the `compile_fail` half is on `Truth`).
#[test]
fn arc_011_truth_hash_unchanged_after_120_ticks() {
    let mut world = test_world();
    let before = hash(world.truth());
    run(&mut world, 120);
    assert_eq!(world.header.tick, Tick(120));
    assert_eq!(hash(world.truth()), before);
}

// ── ARC-012 ─────────────────────────────────────────────────────────────────

/// ARC-012: a view exposes the polity's own state and beliefs and public information. The
/// `compile_fail` doc tests on `PolityView` show there is no path to `Truth` or another polity's
/// detail blocks.
#[test]
fn arc_012_view_sees_own_state_and_public_info() {
    let world = test_world();
    let id = polity(&world, "polity.birch");
    let view = PolityView::new(&world, id);
    assert_eq!(view.polity().id, id);
    assert!(std::ptr::eq(view.beliefs(), &world.beliefs()[id.index()]));
    assert!(std::ptr::eq(view.force(), &world.force[id.index()]));
    assert_eq!(view.polities().len(), 3);
    assert_eq!(view.map().nodes.len(), 6);
}

// ── ARC-013 ─────────────────────────────────────────────────────────────────

fn close(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol * b.abs().max(1.0)
}

/// ARC-013(a): one component matches the Kalman closed form within 1e-12.
#[test]
fn arc_013a_kalman_single_component() {
    let (mu0, s0, z, r) = (2.0, 1.5, 3.7, 0.8);
    let mut b = Belief::normal(Scale::Linear, mu0, s0);
    b.observe(z, r);
    let k = s0 * s0 / (s0 * s0 + r * r);
    let c = b.components()[0];
    assert!(close(c.mu, mu0 + k * (z - mu0), 1e-12));
    assert!(close(c.sigma, (s0 * s0 * (1.0 - k)).sqrt(), 1e-12));
    assert!(close(b.mean(), c.mu, 1e-12) && close(b.sigma(), c.sigma, 1e-12));
    assert_eq!(c.w, 1.0);
}

/// ARC-013(b): an observation near one mode moves > 90% of the weight to it.
#[test]
fn arc_013b_mixture_reweights_to_mode() {
    let comps = [
        Component {
            w: 0.5,
            mu: -5.0,
            sigma: 1.0,
        },
        Component {
            w: 0.5,
            mu: 5.0,
            sigma: 1.0,
        },
    ];
    let mut b = Belief::mixture(Scale::Linear, &comps).unwrap();
    b.observe(4.8, 1.0);
    assert!(b.components()[1].w > 0.9, "{:?}", b.components());
    assert!(close(b.components().iter().map(|c| c.w).sum(), 1.0, 1e-12));
}

/// ARC-013(c): mixture mean and σ equal the analytic moments within 1e-12, on the Linear and
/// Log scales (Logit has no closed form and uses a fixed grid; not held to 1e-12).
#[test]
fn arc_013c_mixture_moments_analytic() {
    let comps = [
        Component {
            w: 0.2,
            mu: -1.0,
            sigma: 0.5,
        },
        Component {
            w: 0.5,
            mu: 0.3,
            sigma: 0.2,
        },
        Component {
            w: 0.3,
            mu: 1.2,
            sigma: 0.9,
        },
    ];
    for scale in [Scale::Linear, Scale::Log] {
        let b = Belief::mixture(scale, &comps).unwrap();
        let (mut m1, mut m2) = (0.0, 0.0);
        for c in &comps {
            let v = c.sigma * c.sigma;
            let (e1, e2) = match scale {
                Scale::Linear => (c.mu, v + c.mu * c.mu),
                _ => ((c.mu + v / 2.0).exp(), (2.0 * c.mu + 2.0 * v).exp()),
            };
            m1 += c.w * e1;
            m2 += c.w * e2;
        }
        assert!(close(b.mean(), m1, 1e-12), "{scale:?}");
        assert!(close(b.sigma(), (m2 - m1 * m1).sqrt(), 1e-12), "{scale:?}");
    }
    // Logit: sanity only. A symmetric belief around logit 0 has mean 0.5.
    let b = Belief::normal(Scale::Logit, 0.0, 1.0);
    assert!(close(b.mean(), 0.5, 1e-9));
    assert!(b.sigma() > 0.0 && b.sigma() < 0.5);
    assert_eq!(Belief::mixture(Scale::Linear, &[]), Err(BeliefError::Count));
    let bad = [Component {
        w: 0.7,
        mu: 0.0,
        sigma: 1.0,
    }];
    assert_eq!(
        Belief::mixture(Scale::Linear, &bad),
        Err(BeliefError::Weights)
    );
    let bad = [Component {
        w: 1.0,
        mu: 0.0,
        sigma: 0.0,
    }];
    assert_eq!(
        Belief::mixture(Scale::Linear, &bad),
        Err(BeliefError::Sigma)
    );
}

/// ARC-013(d): 100 unbiased observations bring σ below 10% of the prior and the mean within 3σ.
#[test]
fn arc_013d_hundred_obs_narrow_sigma() {
    let (truth, prior_sigma, noise) = (3.0, 10.0, 1.0);
    let mut b = Belief::normal(Scale::Linear, 0.0, prior_sigma);
    let mut rng = ChaCha8Rng::seed_from_u64(13);
    let n = Normal::new(0.0, noise).unwrap();
    for _ in 0..100 {
        b.observe(truth + n.sample(&mut rng), noise);
    }
    assert!(b.sigma() < 0.1 * prior_sigma, "σ = {}", b.sigma());
    assert!(
        (b.mean() - truth).abs() < 3.0 * b.sigma(),
        "mean = {}",
        b.mean()
    );
}

/// ARC-013(e): on a Log belief, observations truth × exp(N(0, 0.3)) with log-space noise 0.3
/// bring the log-space mean within 3σ of ln(truth).
#[test]
fn arc_013e_log_scale_obs() {
    let truth: f64 = 50.0;
    let mut b = Belief::normal(Scale::Log, 0.0, 2.0);
    let mut rng = ChaCha8Rng::seed_from_u64(14);
    let n: Normal<f64> = Normal::new(0.0, 0.3).unwrap();
    for _ in 0..100 {
        b.observe(truth * n.sample(&mut rng).exp(), 0.3);
    }
    let c = b.components()[0];
    assert!((c.mu - truth.ln()).abs() < 3.0 * c.sigma, "{c:?}");
    assert!(c.sigma < 0.1 * 2.0);
}

// ── ARC-014 ─────────────────────────────────────────────────────────────────

/// ARC-014: a stub subsystem's observation (pushed in R3) changes `beliefs` only in I1. The stub
/// logs the belief as each stage starts: unchanged through the start of I1, changed by I2.
/// No other path writes beliefs: the field is private and its writer is `pub(crate)`.
#[test]
fn arc_014_beliefs_change_only_in_i1() {
    let mut world = test_world();
    let subject = SubjectIdx(0); // WS-03 range; the stubs give it a N(0, 1) prior
    knobs(&mut world).observe = Some(Observation {
        polity: PolityId(0),
        subject,
        value: 2.0,
        noise_sigma: 1.0,
        source: "stub".into(),
    });
    let others: Vec<_> = world.beliefs()[1..].to_vec();
    step(&mut world, &[]);
    let seen = &world.stub.seen;
    assert_eq!(seen.len(), STAGES.len());
    for (stage, mean) in seen {
        let expected = if *stage == Stage::I2Report { 1.0 } else { 0.0 };
        assert_eq!(*mean, Some(expected), "{stage:?}");
    }
    assert_eq!(world.beliefs()[0].get(subject).unwrap().mean(), 1.0);
    assert_eq!(&world.beliefs()[1..], &others[..]);
}

// ── ARC-015 ─────────────────────────────────────────────────────────────────

/// ARC-015: on the 5-node Sol fixture (surface/orbit nodes of the test bodies), `shortest_path`
/// returns the known optimum Δv path and cost.
#[test]
fn arc_015_sol_fixture_shortest_path() {
    let world = test_world();
    let g = &world.sol.graph;
    assert_eq!(g.nodes.len(), 5);
    let node = |id: &str| g.nodes.iter().position(|n| n.content_id == id).unwrap() as u16;
    let (cost, path) = g
        .shortest_path(
            node("node.planet.surface"),
            node("node.rock.surface"),
            |t| t.dv,
        )
        .unwrap();
    assert!((cost - 14.9).abs() < 1e-12, "{cost}");
    let names: Vec<&str> = path
        .iter()
        .map(|&i| g.nodes[usize::from(i)].content_id.as_str())
        .collect();
    assert_eq!(
        names,
        [
            "node.planet.surface",
            "node.planet.low",
            "node.satellite.low",
            "node.rock.surface"
        ]
    );
    // One-way legs: landing costs 0.1, not the 9.4 ascent.
    let (down, _) = g
        .shortest_path(node("node.planet.low"), node("node.planet.surface"), |t| {
            t.dv
        })
        .unwrap();
    assert!((down - 0.1).abs() < 1e-12);
    let body = g.nodes[usize::from(node("node.satellite.low"))].body;
    assert_eq!(
        world.sol.bodies[usize::from(body.0)].content_id,
        "body.satellite"
    );
}

/// ARC-015: adjacency lists of both graphs are sorted by target.
#[test]
fn arc_015_adjacency_sorted() {
    let world = test_world();
    let sorted = |targets: Vec<u16>| targets.windows(2).all(|w| w[0] <= w[1]);
    for n in 0..world.map.nodes.len() as u16 {
        assert!(sorted(world.map.neighbors(n).map(|(t, _)| t).collect()));
    }
    for n in 0..world.sol.graph.nodes.len() as u16 {
        assert!(sorted(
            world.sol.graph.neighbors(n).map(|(t, _)| t).collect()
        ));
    }
    let e = &world.map.edges;
    assert!(e.windows(2).all(|w| (w[0].0, w[0].1) <= (w[1].0, w[1].1)));
}

/// ARC-015: region neighbor queries and minimum-cost paths on the test map.
#[test]
fn arc_015_region_neighbors_and_path() {
    let world = test_world();
    let g = &world.map;
    let idx = |id: &str| g.nodes.iter().position(|r| r.content_id == id).unwrap() as u16;
    let mut names: Vec<&str> = g
        .neighbors(idx("region.vale"))
        .map(|(t, _)| g.nodes[usize::from(t)].content_id.as_str())
        .collect();
    names.sort();
    assert_eq!(names, ["region.delta", "region.ridge", "region.steppe"]);
    // Undirected: the reverse edge exists.
    assert!(
        g.neighbors(idx("region.ridge"))
            .any(|(t, _)| t == idx("region.vale"))
    );
    // vale → sound: via ridge and coast 480 + 450 + 300 = 1230 beats via delta 610 + 690 = 1300.
    let (cost, path) = g
        .shortest_path(idx("region.vale"), idx("region.sound"), |e| {
            f64::from(e.cost)
        })
        .unwrap();
    assert_eq!(cost, 1230.0);
    assert_eq!(
        path,
        [
            idx("region.vale"),
            idx("region.ridge"),
            idx("region.coast"),
            idx("region.sound")
        ]
    );
}

// ── ARC-016 ─────────────────────────────────────────────────────────────────

/// ARC-016(a): ranges are WS-03, WS-05, WS-08b in that order, contiguous from 0, and identical
/// (as is the whole world) across worldgens from shuffled content files.
#[test]
fn arc_016a_ranges_ordered_contiguous_shuffle_invariant() {
    let world = test_world();
    let r = &world.header.subject_ranges;
    let owners: Vec<&str> = r.iter().map(|r| r.owner.as_ref()).collect();
    assert_eq!(owners, ["econ", "rnd", "resources"]); // WS-03, WS-05, WS-08b
    assert_eq!(r[0].start, 0);
    assert!(r.windows(2).all(|w| w[1].start == w[0].start + w[0].len));
    // 6 × 3 polities, 3 × 5 techs, 1 Earth + 2 × 1 off-Earth deposit.
    let lens: Vec<u32> = r.iter().map(|r| r.len).collect();
    assert_eq!(lens, [18, 15, 3]);

    let dir = shuffled_copy("arc_016a");
    let shuffled = load(&dir);
    assert_ne!(
        std::fs::read_to_string(dir.join("maps/earth/regions.ron")).unwrap(),
        std::fs::read_to_string(content_root().join("maps/earth/regions.ron")).unwrap(),
        "the shuffle must change file contents"
    );
    assert_eq!(shuffled.header.subject_ranges, *r);
    assert_eq!(hash(&shuffled), hash(&world));
}

/// ARC-016(b): a macro polity's `get` is `None` for every WS-05 subject (the storage half is the
/// unit test `arc_016b_macro_stores_no_tech_slice`).
#[test]
fn arc_016b_macro_omits_tech_range() {
    let world = test_world();
    let rnd = world
        .header
        .subject_ranges
        .iter()
        .find(|r| r.owner == "rnd")
        .unwrap();
    let cedar = polity(&world, "polity.cedar");
    assert_eq!(world.polities[cedar.index()].tier, Tier::Macro);
    let beliefs = &world.beliefs()[cedar.index()];
    assert!((rnd.start..rnd.start + rnd.len).all(|s| beliefs.get(SubjectIdx(s)).is_none()));
    // Its WS-03 subjects (stub priors) are still readable.
    assert!(beliefs.get(SubjectIdx(0)).is_some());
}

// ── ARC-020 ─────────────────────────────────────────────────────────────────

/// ARC-020: the per-tick stage sequence equals the WS-02 §3.3 table.
#[test]
fn arc_020_stage_order() {
    let mut world = test_world();
    for _ in 0..3 {
        step(&mut world, &[]);
        let ran: Vec<Stage> = world
            .events
            .iter()
            .filter_map(|e| match e.kind {
                EventKind::StageRan(s) => Some(s),
                _ => None,
            })
            .collect();
        use Stage::*;
        assert_eq!(
            ran,
            [
                D1Orders,
                D2AiDecide,
                R1ApplyOrders,
                R2Exogenous,
                R3Production,
                R4Finance,
                R5Politics,
                R5bForces,
                R6External,
                R6bLosses,
                R7Tier,
                I1Observe,
                I2Report
            ]
        );
    }
}

// ── ARC-021 ─────────────────────────────────────────────────────────────────

/// ARC-021: two stub subsystems in R3 each read the other's block; swapping their call order
/// gives identical results, and each saw the stage-start value (a = b₀ + 1, b = 2·a₀).
#[test]
fn arc_021_swap_order_same_result() {
    let mut forward = test_world();
    let mut swapped = test_world();
    knobs(&mut swapped).swap_pair = true;
    step(&mut forward, &[]);
    step(&mut swapped, &[]);
    assert_eq!(forward.stub.a, [2.0; 3]);
    assert_eq!(forward.stub.b, [2.0; 3]);
    run(&mut forward, 10);
    run(&mut swapped, 10);
    assert_eq!(forward.stub.a, swapped.stub.a);
    assert_eq!(forward.stub.b, swapped.stub.b);
    assert_eq!(hash(&forward), hash(&swapped));
}

// ── ARC-022 ─────────────────────────────────────────────────────────────────

fn assert_rejected(mut world: World, order: Order, reason: &str) {
    let mut control = world.clone();
    step(&mut world, std::slice::from_ref(&order));
    step(&mut control, &[]);
    let rejected: Vec<_> = world
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::OrderRejected {
                polity,
                index,
                ai: false,
                reason,
            } => Some((*polity, *index, reason.to_string())),
            _ => None,
        })
        .collect();
    assert_eq!(rejected, [(order.polity, 0, reason.to_string())]);
    assert_eq!(
        state_hash_without_events(&world),
        state_hash_without_events(&control)
    );
}

/// ARC-022: an order for a dead polity is rejected as an event and leaves state unchanged.
#[test]
fn arc_022_dead_polity_rejected() {
    let mut world = test_world();
    world.polities[1].alive = false;
    let order = Order {
        polity: PolityId(1),
        tick: Tick(0),
        kind: OrderKind::StubSet { value: 0.5 },
    };
    assert_rejected(world, order, "polity is dead");
}

/// ARC-022: an order for a polity index that doesn't exist is rejected.
#[test]
fn arc_022_unknown_polity_rejected() {
    let order = Order {
        polity: PolityId(99),
        tick: Tick(0),
        kind: OrderKind::StubSet { value: 0.5 },
    };
    assert_rejected(test_world(), order, "unknown polity");
}

/// ARC-022: AI orders (D2) are validated like player orders: an invalid one becomes a rejection
/// event, never reaches R1, and doesn't panic; a valid one applies.
#[test]
fn arc_022_ai_order_validated() {
    for value in [f64::NAN, 2.0] {
        let mut world = test_world();
        let mut control = world.clone();
        knobs(&mut world).ai_order = Some((PolityId(1), value));
        step(&mut world, &[]);
        step(&mut control, &[]);
        let rejected: Vec<_> = world
            .events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::OrderRejected {
                    polity,
                    index,
                    ai: true,
                    ..
                } => Some((*polity, *index)),
                _ => None,
            })
            .collect();
        assert_eq!(rejected, [(PolityId(1), 0)]);
        assert_eq!(world.stub.set, control.stub.set);
    }
    let mut world = test_world();
    knobs(&mut world).ai_order = Some((PolityId(1), 0.75));
    step(&mut world, &[]);
    assert_eq!(world.stub.set, [0.0, 0.75, 0.0]);
}

/// ARC-022: an order for a past tick is rejected.
#[test]
fn arc_022_past_tick_rejected() {
    let mut world = test_world();
    run(&mut world, 2);
    let order = Order {
        polity: PolityId(0),
        tick: Tick(1),
        kind: OrderKind::StubSet { value: 0.5 },
    };
    assert_rejected(world, order, "order is not for this tick");
}

/// ARC-022: out-of-range values (including NaN) are rejected without a panic; a valid one applies.
#[test]
fn arc_022_out_of_range_rejected() {
    for value in [2.0, -0.1, f64::NAN] {
        let order = Order {
            polity: PolityId(0),
            tick: Tick(0),
            kind: OrderKind::StubSet { value },
        };
        assert_rejected(test_world(), order, "value out of range [0, 1]");
    }
    let mut world = test_world();
    let order = Order {
        polity: PolityId(2),
        tick: Tick(0),
        kind: OrderKind::StubSet { value: 0.25 },
    };
    step(&mut world, &[order]);
    assert_eq!(world.stub.set, [0.0, 0.0, 0.25]);
    assert!(
        !world
            .events
            .iter()
            .any(|e| matches!(e.kind, EventKind::OrderRejected { .. }))
    );
}

// ── ARC-023 ─────────────────────────────────────────────────────────────────

/// ARC-023: a stub that writes NaN panics, naming the stage and the block. The check runs in
/// debug builds and in every build with the test stubs, so this also runs under `--release`.
#[test]
fn arc_023_nan_panics_naming_stage_and_block() {
    for stage in [Stage::D1Orders, Stage::R4Finance, Stage::I2Report] {
        let mut world = test_world();
        knobs(&mut world).nan_stage = Some(stage);
        let err = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| step(&mut world, &[])))
            .unwrap_err();
        let msg = err.downcast_ref::<String>().unwrap();
        assert!(msg.contains(&format!("stage {stage:?}")), "{msg}");
        assert!(msg.contains("block `stub`"), "{msg}");
    }
}

// ── ARC-024 ─────────────────────────────────────────────────────────────────

/// ARC-024: a stub combat loss in R6 reaches `force` and `agg.military_power` in the same tick.
/// M4b checks the change with the pure `Breakdown::change` on the world before and after the
/// tick; M4c reroutes this check through `explain_change` (ARC-045). The grader half: combat
/// (`subsystems::combat::engage`) returns only `Vec<EngagementRecord>`, never a `ForceBlock`.
#[test]
fn arc_024_combat_loss_same_tick() {
    let mut world = test_world();
    let alder = polity(&world, "polity.alder");
    let strengths = [(1, 0.5), (2, 0.75), (3, 1.0)];
    world.force[alder.index()].units = strengths
        .iter()
        .map(|&(id, strength)| StubUnit {
            id: UnitId(id),
            strength,
        })
        .collect();
    step(&mut world, &[]);
    assert_eq!(world.polities[alder.index()].agg.military_power.get(), 2.25);

    let before = world.clone();
    knobs(&mut world).kill = Some((Tick(1), alder, UnitId(2)));
    step(&mut world, &[]);

    // The engagement record, then the unit gone from `force` after R7 of the same tick.
    assert_eq!(
        world.engagements,
        [EngagementRecord {
            polity: alder,
            unit: UnitId(2),
            destroyed: true
        }]
    );
    let units: Vec<UnitId> = world.force[alder.index()]
        .units
        .iter()
        .map(|u| u.id)
        .collect();
    assert_eq!(units, [UnitId(1), UnitId(3)]);
    // The R6b removal event cites the engagement record.
    assert!(world.events.iter().any(|e| e.tick == Tick(1)
        && e.kind
            == EventKind::UnitRemoved {
                polity: alder,
                unit: UnitId(2),
                engagement: 0
            }));

    let prev = &before.polities[alder.index()].agg.military_power;
    let cur = &world.polities[alder.index()].agg.military_power;
    let change = Breakdown::change(prev.why(), cur.why());
    assert_eq!(change.total, cur.get() - prev.get());
    let unit_key = VarKey {
        owner: Owner::Unit(UnitId(2)),
        block: "force".into(),
        field: "strength".into(),
    };
    let term = change
        .terms
        .iter()
        .find(|t| t.link.as_ref() == Some(&unit_key))
        .unwrap();
    assert!((term.value + 0.75).abs() <= 1e-9 * 0.75, "{term:?}");
    let sum: f64 = change.terms.iter().map(|t| t.value).sum();
    assert!((sum - change.total).abs() <= 1e-9 * change.total.abs().max(1.0));
    assert!(
        change
            .terms
            .iter()
            .filter(|t| t.link != Some(unit_key.clone()))
            .all(|t| t.value == 0.0)
    );
}
