"""WS-08b Sol graph prototype: builds the delta-v graph from content/maps/sol/*.ron with the WS-08b §4 formulas,
then prints the hand-check values, launch windows and size/budget checks the spec cites.

Usage:  python sol.py [content_dir]          (default: ../../content)
NumPy only (P-9). Helper functions are at the bottom (repo convention). Statistical checks use PCG64, not ChaCha8:
the Rust port must match the formulas exactly and the distributions statistically (ADR-0006 note in WS-08b §7).
"""
import math
import sys
from pathlib import Path

import numpy as np

MU_SUN = 1.32712440018e11          # km^3/s^2 [R]
AU_KM = 149597870.7                # km [V: IAU 2012 definition]
MEAN_MONTH_D = 365.2425 / 12       # days; Gregorian mean month
JD_1955 = 2435108.5                # JD of 1955-01-01 00:00 (tick 0 start)
JD_J2000 = 2451545.0
MAX_TRANSFER_MONTHS = 240          # heliocentric edges with a longer Hohmann time are not built [D]
INC_FREE_DEG = 2.0                 # relative inclination absorbed by launch timing [D]
LOSS = {"Airless": 0.14, "Thin": 0.16, "Thick": 0.60}   # ascent losses as a fraction of v_circ [D; None calibrated to Moon 1.87]
WINDOW_HALF = 1                    # months either side of the optimal month that are open [D]
WINDOW_K = 0.15                    # Δv penalty at the window edge (fraction of Hohmann excess) [D]
TIME_WEIGHT = 0.02                 # km/s-equivalent per transit month in the planner cost [D]


def main(content):
    bodies = parse_ron((content / "maps/sol/bodies.ron").read_text())
    overrides = parse_ron((content / "maps/sol/transfers.ron").read_text())
    rules = parse_ron((content / "maps/sol/gen_rules.ron").read_text())
    by_id = {b["id"]: b for b in bodies}
    nodes, edges = build_graph(bodies, by_id, overrides)
    print(f"template graph: {len(bodies)} bodies, {len(nodes)} nodes, {len(edges)} directed edges")

    print("\n== Hand checks (km/s) ==")
    mu, re = 398600.436, 6378.1366
    leo, geo = re + 300, re + 35786
    hand_geo = hohmann_dv(mu, leo, geo)
    print(f"LEO->GEO   graph {edges[('node.earth.low', 'node.earth.synchronous')][0]:.4f}  hand {hand_geo:.4f}")
    for a, b in [("node.earth.surface", "node.moon.surface"), ("node.earth.low", "node.moon.low"),
                 ("node.earth.low", "node.mars.low"), ("node.earth.surface", "node.mars.surface"),
                 ("node.moon.surface", "node.moon.low"), ("node.earth.low", "node.ceres.low"),
                 ("node.earth.low", "node.jupiter.high"), ("node.earth.low", "node.callisto.low")]:
        for w in (0.0, TIME_WEIGHT):
            cost, path = dijkstra(nodes, edges, a, b, w)
            dv = sum(edges[(p, q)][0] for p, q in zip(path, path[1:]))
            months = sum(edges[(p, q)][1] for p, q in zip(path, path[1:]))
            print(f"w={w:.2f} {a} -> {b}: dv {dv:.3f}, {months} mo  via {' > '.join(p.split('.', 1)[1] for p in path)}")
    v_inf_dep, v_inf_arr, t_h = hohmann_vinf(MU_SUN, AU_KM * 1.00000261, AU_KM * 1.52371034)
    print(f"Earth-Mars Hohmann: v_inf dep {v_inf_dep:.3f}, arr {v_inf_arr:.3f}, T_H {t_h / 86400 / MEAN_MONTH_D:.2f} months")
    print(f"  LEO departure {oberth_dv(mu, leo, leo, v_inf_dep):.3f}; LMO capture {oberth_dv(42828.37362, 3396.19 + 300, 3396.19 + 300, v_inf_arr):.3f}")

    print("\n== Launch-site latitude penalty to GEO (km/s, added to LEO->GEO) ==")
    for lat in (0.0, 5.2, 28.5, 45.9, 62.9):
        print(f"  {lat:5.1f} deg: +{abs(site_penalty(mu, leo, geo, lat)):.3f}")

    print("\n== Earth->Mars windows (circular orbits, real J2000 phases) vs historical launches ==")
    history = {"1960-10": "Mars 1960A [R]", "1962-11": "Mars 1 [V]", "1964-11": "Mariner 4 [V]",
               "1969-02": "Mariner 6 [V]", "1971-05": "Mariner 9 [V]", "1973-07": "Mars 4 [R]", "1975-08": "Viking 1 [V]"}
    opt = optimal_departures(by_id["body.earth"], by_id["body.mars"], 0, 12 * 26)
    for t in opt:
        print(f"  optimal {tick_to_ym(int(t))} (t* = {t:.2f})   open months: {', '.join(tick_to_ym(m) for m in open_months(t))}")
    errs = []
    for ym, name in history.items():
        m = ym_to_tick(ym)
        nearest = min(opt, key=lambda t: abs(t - (m + 0.5)))
        errs.append(m + 0.5 - nearest)
        print(f"  {ym} {name:15s} offset from optimum {m + 0.5 - nearest:+.2f} months")
    print(f"  max |offset| = {max(abs(e) for e in errs):.2f} months")
    print(f"  synodic period {synodic_months(by_id['body.earth'], by_id['body.mars']):.2f} months")

    print("\n== Hypothetical bodies over 2,000 seeds ==")
    counts, violations, sizes, groups, belt = [], 0, [], {}, []
    for seed in range(2000):
        gen = generate_bodies(rules, by_id, np.random.Generator(np.random.PCG64(seed)))
        counts.append(len(gen))
        violations += sum(not g["ok"] for g in gen)
        for g in gen:
            if g["group"]:
                groups.setdefault(g["cls"], []).append(g["group"])
            if g["cls"] == "gen.belt":
                belt.append((g["body"]["orbit"]["a"][1][0], g["body"]["composition"]))
        all_bodies = bodies + [g["body"] for g in gen]
        n, e = build_graph(all_bodies, {b["id"]: b for b in all_bodies}, overrides)
        sizes.append((len(all_bodies), len(n), len(e)))
    sizes = np.array(sizes)
    print(f"  generated per world: mean {np.mean(counts):.2f}, max {max(counts)}; constraint violations {violations}")
    print(f"  bodies max {sizes[:, 0].max()}, nodes max {sizes[:, 1].max()}, directed edges max {sizes[:, 2].max()}")
    for cid, gs in groups.items():
        shares = {k: round(gs.count(k) / len(gs), 3) for k in sorted(set(gs))}
        print(f"  {cid}: n = {len(gs)}, shares {shares}")
    inner = [c for a, c in belt if a < 2.5]
    outer = [c for a, c in belt if a > 2.82]
    print(f"  belt Stony share: inner (<2.5 AU) {inner.count('Stony') / len(inner):.2f}, outer (>2.82 AU) {outer.count('Stony') / len(outer):.2f}")

    print("\n== Resource candidates and belief subjects ==")
    earth = parse_ron((content / "maps/earth/resources.ron").read_text())
    n_earth = sum(len(tags) for _, tags in earth)
    deposits = parse_ron((content / "core/resources/body_deposits.ron").read_text())
    n_body = len(deposits["by_body"])
    max_gen = sum(c["max"] for c in rules["classes"]) * 2   # ≤ 2 composition rules match any one composition
    print(f"  Earth tags {n_earth}; real off-Earth candidates {n_body}; generated ≤ {max_gen}")
    print(f"  subjects ≤ {n_earth + 2 * (n_body + max_gen)} (Earth 1 each, off-Earth 2 each)")


# ───────────────────────────── helpers ─────────────────────────────

def build_graph(bodies, by_id, overrides):
    """Nodes and directed edges {(from, to): (dv km/s, months)} per WS-08b §4.2."""
    nodes, edges = {}, {}
    for b in bodies:
        for nd in b.get("nodes", []):
            nodes[node_id(b, nd)] = (b, nd)
    hub = {b["id"]: next(node_id(b, nd) for nd in b["nodes"] if nd.get("hub")) for b in bodies if b.get("nodes")}
    for b in bodies:
        if not b.get("nodes"):
            continue
        h = hub[b["id"]]
        hn = nodes[h][1]
        for nd in b["nodes"]:
            nid = node_id(b, nd)
            if nid == h or nd["regime"] == "Lagrange":
                continue
            if nd["regime"] == "Surface":
                v = math.sqrt(b["gm"] / periapsis(b, hn))
                up = nd.get("ascent_dv") or v * (1 + LOSS[b["atmosphere"]])
                down = nd.get("descent_dv") or (0.1 * v if b.get("aerocapture") else up)
                edges[(nid, h)], edges[(h, nid)] = (up, 0), (down, 0)
            else:
                dv = orbit_change_dv(b["gm"], node_orbit(b, hn), node_orbit(b, nd))
                edges[(nid, h)] = edges[(h, nid)] = (dv, 0)
    for b in bodies:                                   # moons ↔ parent hub
        p = b.get("parent")
        if not p or p == "body.sun" or not b.get("nodes"):
            continue
        parent = by_id[p]
        rp, ra = node_orbit(parent, nodes[hub[p]][1])
        dv = moon_edge_dv(parent["gm"], rp, ra, orbit_a_km(b), b, nodes[hub[b["id"]]][1])
        edges[(hub[p], hub[b["id"]])] = edges[(hub[b["id"]], hub[p])] = (dv, 0)
    helio = [b for b in bodies if b.get("parent") == "body.sun" and b.get("nodes")]
    for i, a in enumerate(helio):                      # heliocentric hub ↔ hub
        for c in helio[i + 1:]:
            v1, v2, t = hohmann_vinf(MU_SUN, orbit_a_km(a), orbit_a_km(c))
            months = math.ceil(t / 86400 / MEAN_MONTH_D)
            if months > MAX_TRANSFER_MONTHS:
                continue
            di = math.radians(max(0.0, abs(a["orbit"]["inc_deg"] - c["orbit"]["inc_deg"]) - INC_FREE_DEG))
            ha, hc = nodes[hub[a["id"]]][1], nodes[hub[c["id"]]][1]
            # plane change folded into the v_inf at whichever end costs less (usually the slower, outer end) [D]
            pa, pc = math.sqrt(MU_SUN / orbit_a_km(a)) * di, math.sqrt(MU_SUN / orbit_a_km(c)) * di
            fwd = min(depart_dv(a, ha, math.hypot(v1, pa)) + depart_dv(c, hc, v2),
                      depart_dv(a, ha, v1) + depart_dv(c, hc, math.hypot(v2, pc)))
            back = min(depart_dv(c, hc, math.hypot(v2, pc)) + depart_dv(a, ha, v1),
                       depart_dv(c, hc, v2) + depart_dv(a, ha, math.hypot(v1, pa)))
            edges[(hub[a["id"]], hub[c["id"]])] = (fwd, months)
            edges[(hub[c["id"]], hub[a["id"]])] = (back, months)
    for o in overrides:
        both = [(o["from"], o["to"])] + ([] if o.get("one_way") else [(o["to"], o["from"])])
        for k in both:
            edges[k] = (o["dv"], o["months"])
    return nodes, edges


def node_id(b, nd):
    stem = b["id"].split(".", 1)[1]
    reg = snake(nd["regime"])
    return f"node.{stem}.{reg}" + (f"_{nd['label']}" if nd.get("label") else "")


def snake(name):
    return "".join("_" + c.lower() if c.isupper() else c for c in name).lstrip("_")


def node_orbit(b, nd):
    rp = b["radius_km"] + nd.get("alt_km", 0.0)
    ra = b["radius_km"] + nd["apo_alt_km"] if nd.get("apo_alt_km") else rp
    return rp, ra


def periapsis(b, nd):
    return node_orbit(b, nd)[0]


def orbit_a_km(b):
    kind, val = b["orbit"]["a"]
    return val[0] * AU_KM if kind == "Au" else val[0]


def vis_viva(mu, r, a):
    return math.sqrt(mu * (2 / r - 1 / a))


def hohmann_dv(mu, r1, r2):
    a = (r1 + r2) / 2
    return abs(vis_viva(mu, r1, a) - math.sqrt(mu / r1)) + abs(math.sqrt(mu / r2) - vis_viva(mu, r2, a))


def orbit_change_dv(mu, o1, o2):
    """Circular↔circular: Hohmann. Circular↔ellipse sharing the periapsis radius: one burn. Else Hohmann on periapses."""
    (p1, a1), (p2, a2) = o1, o2
    if p1 == a1 and p2 == a2:
        return hohmann_dv(mu, p1, p2)
    if abs(p1 - p2) < 1e-6:
        return abs(vis_viva(mu, p1, (p1 + a1) / 2) - vis_viva(mu, p2, (p2 + a2) / 2))
    return hohmann_dv(mu, p1, p2)


def hohmann_vinf(mu, r1, r2):
    a = (r1 + r2) / 2
    return (abs(vis_viva(mu, r1, a) - math.sqrt(mu / r1)), abs(math.sqrt(mu / r2) - vis_viva(mu, r2, a)),
            math.pi * math.sqrt(a ** 3 / mu))


def oberth_dv(mu, rp, ra, v_inf):
    """Burn at periapsis rp between a hyperbola with v_inf and the bound orbit (rp, ra)."""
    return math.sqrt(v_inf ** 2 + 2 * mu / rp) - vis_viva(mu, rp, (rp + ra) / 2)


def depart_dv(b, hub_node, v_inf):
    rp, ra = node_orbit(b, hub_node)
    if b["gm"] < 1.0 and hub_node["regime"] == "Surface":
        return v_inf                                  # small body: rendezvous = match v_inf [D]
    return oberth_dv(b["gm"], rp, ra, v_inf)


def moon_edge_dv(mu, rp, ra, a_m, moon, moon_hub):
    """Parent hub (rp, ra) → moon orbit radius a_m: one burn at the apse that leaves the other apse at a_m, then capture."""
    if a_m >= ra:
        r_s, r_t = rp, a_m             # raise apoapsis from periapsis
    else:
        r_s, r_t = ra, a_m             # lower periapsis from apoapsis
    dv1 = abs(vis_viva(mu, r_s, (r_s + r_t) / 2) - vis_viva(mu, r_s, (rp + ra) / 2))
    v_inf = abs(vis_viva(mu, r_t, (r_s + r_t) / 2) - math.sqrt(mu / a_m))
    return dv1 + depart_dv(moon, moon_hub, v_inf)


def site_penalty(mu, r_leo, r_geo, lat_deg):
    """Extra Δv to remove inclination = launch latitude, combined with the GEO circularisation burn."""
    v_a = vis_viva(mu, r_geo, (r_leo + r_geo) / 2)
    v_geo = math.sqrt(mu / r_geo)
    i = math.radians(lat_deg)
    return math.sqrt(v_a ** 2 + v_geo ** 2 - 2 * v_a * v_geo * math.cos(i)) - (v_geo - v_a)


def dijkstra(nodes, edges, src, dst, time_weight=0.0):
    adj = {}
    for (a, b), (dv, months) in sorted(edges.items()):
        adj.setdefault(a, []).append((b, dv + time_weight * months))
    dist, prev, done = {src: 0.0}, {}, set()
    while True:
        cand = [(d, n) for n, d in dist.items() if n not in done]
        if not cand:
            break
        d, n = min(cand)                               # ties broken by node ID (deterministic)
        if n == dst:
            break
        done.add(n)
        for m, w in adj.get(n, []):
            if d + w < dist.get(m, math.inf) - 1e-12:
                dist[m], prev[m] = d + w, n
    path, n = [dst], dst
    while n != src:
        n = prev[n]
        path.append(n)
    return dist[dst], path[::-1]


def mean_lon_rad(b, tick):
    l0, rate = b["orbit"]["mean_lon"]
    jd = JD_1955 + tick * MEAN_MONTH_D
    return math.radians(l0 + rate * (jd - JD_J2000) / 36525)


def synodic_months(a, b):
    na, nb = (math.degrees(math.sqrt(MU_SUN / orbit_a_km(x) ** 3)) * 86400 * MEAN_MONTH_D for x in (a, b))
    return 360 / abs(na - nb)


def optimal_departures(a, b, t0, t1):
    """Fractional ticks where the B-minus-A phase equals the Hohmann requirement π − n_B·T_H."""
    _, _, t_h = hohmann_vinf(MU_SUN, orbit_a_km(a), orbit_a_km(b))
    req = math.pi - math.sqrt(MU_SUN / orbit_a_km(b) ** 3) * t_h
    out, prev = [], None
    for k in range(t0 * 10, t1 * 10 + 1):
        t = k / 10
        err = (mean_lon_rad(b, t) - mean_lon_rad(a, t) - req + math.pi) % (2 * math.pi) - math.pi
        if prev is not None and prev > 0 >= err and prev - err < math.pi:   # relative phase falls through zero
            out.append(t - 0.1 * err / (err - prev))
        prev = err
    return out


def open_months(t_star):
    m = math.floor(t_star)
    return [k for k in range(m - WINDOW_HALF, m + WINDOW_HALF + 1)]


def tick_to_ym(t):
    return f"{1955 + t // 12}-{t % 12 + 1:02d}"


def ym_to_tick(ym):
    y, m = map(int, ym.split("-"))
    return (y - 1955) * 12 + m - 1


def generate_bodies(rules, by_id, rng):
    """Seeded hypothetical bodies per gen_rules.ron; returns [{'body', 'cls', 'group', 'ok'}]."""
    out = []
    for cls in rules["classes"]:
        n = min(rng.poisson(cls["count_mean"]), cls["max"])
        parent = by_id[cls["parent"]]
        siblings = [orbit_a_km(b) for b in by_id.values() if b.get("parent") == cls["parent"]]
        kind = cls["orbit"]["_t"] if isinstance(cls["orbit"], dict) else cls["orbit"][0]
        for k in range(n):
            d_lo, d_hi = cls["diameter_km"]
            s = cls["size_slope"]
            d = (d_lo ** -s - rng.random() * (d_lo ** -s - d_hi ** -s)) ** (-1 / s)    # truncated power law
            inc = rng.rayleigh(cls["inc_rayleigh_deg"])
            group, mean_lon = None, None
            if kind == "Belt":
                a, e, ok = sample_belt(cls["orbit"], rng)
                inc = min(inc, cls["orbit"].get("inc_max_deg") or 180.0)   # Hildas: i < 20°
                orbit = {"a": ("Au", [a]), "inc_deg": inc, "mean_lon": None, "ecc": e}
            elif kind == "NeoGroups":
                group = choose(rng, cls["orbit"][1][0])
                a, e, ok = sample_neo(group, rng)
                orbit = {"a": ("Au", [a]), "inc_deg": inc, "mean_lon": None, "ecc": e}
            elif kind == "Trojan":
                host = by_id[cls["orbit"]["of"]]
                group = "L4" if rng.random() < cls["orbit"]["l4_share"] else "L5"
                lib = rng.uniform(-1, 1) * cls["orbit"]["libration_deg"]
                l0, rate = host["orbit"]["mean_lon"]
                mean_lon = (l0 + (60.0 if group == "L4" else -60.0) + lib, rate)    # leads (L4) or trails (L5) the host
                a, ok = orbit_a_km(host) / AU_KM, True
                orbit = {"a": ("Au", [a]), "inc_deg": inc, "mean_lon": mean_lon, "ecc": 0.0}
            else:                                                                   # Moon
                a, ok = sample_moon(cls["orbit"], parent, siblings, rng)
                siblings.append(a)
                orbit = {"a": ("Km", [a]), "inc_deg": inc, "mean_lon": None, "ecc": 0.0}
            comp = sample_composition(cls["composition"], orbit, rng)
            rho = dict(rules["densities_g_cm3"])[comp] * 1000
            body = {"id": f"body.{cls['id'].split('.', 1)[1]}_{k}", "parent": cls["parent"],
                    "gm": 6.6743e-20 * 4 / 3 * math.pi * (d / 2 * 1e3) ** 3 * rho,
                    "radius_km": d / 2, "orbit": orbit, "composition": comp, "atmosphere": "Airless",
                    "nodes": [{"regime": "Surface", "hub": True}]}
            ok = ok and d_lo <= d <= d_hi
            out.append({"body": body, "cls": cls["id"], "group": group, "ok": ok})
    return out


def sample_belt(o, rng):
    lo, hi = o["a_au"]
    for _ in range(100):                                # rejection sampling on the plausibility rules
        a = rng.uniform(lo, hi)
        if any(abs(a - g) < w for g, w in o.get("gaps_au", [])):
            continue
        e = rng.rayleigh(o["ecc_rayleigh"])
        if e < (o.get("ecc_max") or 0.9):
            return a, e, True
    return a, e, False


def sample_neo(group, rng):
    """(a, e) inside the NEO group definitions: Amor 1.017 < q ≤ 1.3; Apollo a > 1, q < 1.017;
    Aten a < 1, Q > 0.983; Atira Q < 0.983. All NEOs: q < 1.3, a < 4.2 AU."""
    for _ in range(100):
        if group == "Amor":
            q, a = rng.uniform(1.017, 1.3), rng.uniform(1.05, 3.0)
        elif group == "Apollo":
            q, a = rng.uniform(0.3, 1.017), rng.uniform(1.0, 3.0)
        elif group == "Aten":
            a = rng.uniform(0.7, 1.0)
            e_min = max(0.0, 0.983 / a - 1)
            q = a * (1 - rng.uniform(e_min, min(0.9, e_min + 0.6)))
        else:
            a = rng.uniform(0.5, 0.95)
            q = a * (1 - rng.uniform(0.0, max(0.0, 0.983 / a - 1)))
        if q >= a:
            continue
        e = 1 - q / a
        if neo_group(a, e) == group:
            return a, e, True
    return a, e, False


def neo_group(a, e):
    q, big_q = a * (1 - e), a * (1 + e)
    if q >= 1.3 or a >= 4.2:
        return None
    if a < 1.0:
        return "Atira" if big_q < 0.983 else "Aten"
    return "Apollo" if q < 1.017 else "Amor"


def sample_composition(spec, orbit, rng):
    kind, val = (spec["_t"], spec) if isinstance(spec, dict) else (spec[0], spec[1][0])
    if kind == "Fixed":
        return choose(rng, val)
    a = orbit["a"][1][0]
    (lo, hi), (s_lo, s_hi) = val["a_au"], val["stony"]
    f = min(1.0, max(0.0, (a - lo) / (hi - lo)))
    p_s = s_lo + f * (s_hi - s_lo)
    return choose(rng, [("Stony", p_s), ("Metallic", val["metallic"]), ("Carbonaceous", 1 - p_s - val["metallic"])])


def sample_moon(o, parent, siblings, rng):
    r_hill = orbit_a_km(parent) * (parent["gm"] / (3 * MU_SUN)) ** (1 / 3)
    lo, hi = o["hill_fraction"]
    r_min = max(lo * r_hill, o["roche_factor"] * 2.44 * parent["radius_km"])
    for _ in range(100):
        a = rng.uniform(r_min, hi * r_hill)
        if all(abs(a - s) / s >= o["min_rel_spacing"] for s in siblings):
            return a, True
    return a, False


def choose(rng, weighted):
    names = [n for n, _ in weighted]
    w = np.array([p for _, p in weighted], dtype=float)
    return names[rng.choice(len(names), p=w / w.sum())]


def parse_ron(text):
    """Minimal RON reader for this repo's content: structs → dict, enum(args) → (name, [args]), Some(x) → x."""
    import re
    text = re.sub(r"//[^\n]*", "", text)
    toks = re.findall(r'"[^"]*"|-?\d+\.?\d*(?:e-?\d+)?|[A-Za-z_][A-Za-z0-9_]*|[()\[\],:]', text)
    pos = [0]

    def peek():
        return toks[pos[0]] if pos[0] < len(toks) else None

    def take():
        pos[0] += 1
        return toks[pos[0] - 1]

    def value():
        t = take()
        if t.startswith('"'):
            return t[1:-1]
        if re.match(r"-?\d", t):
            return float(t) if any(c in t for c in ".e") else int(t)
        if t == "[":
            return seq("]")
        if t == "(":
            items = seq(")")
            return tuple(items) if not isinstance(items, dict) else items
        if t in ("true", "false"):
            return t == "true"
        if t == "None":
            return None
        if peek() == "(":
            take()
            inner = seq(")")
            if t == "Some":
                return inner[0]
            if isinstance(inner, dict):
                inner["_t"] = t
                return inner
            return (t, inner)
        return t

    def seq(close):
        items, fields = [], {}
        while peek() != close:
            if toks[pos[0] + 1] == ":" and re.match(r"[A-Za-z_]", peek()):
                k = take()
                take()
                fields[k] = value()
            else:
                items.append(value())
            if peek() == ",":
                take()
        take()
        return fields if fields else items

    return value()


if __name__ == "__main__":
    main(Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parents[2] / "content")
