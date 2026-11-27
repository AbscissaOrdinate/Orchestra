"""Low-fidelity 1955 map of the expanded DATE world (WS-08a).

Draws approximate region cells from region centroids (Voronoi, clipped to Natural Earth land),
coloured by owner. Ownership, subjects, claims and status come straight from the RON content,
so the picture always matches the data. Cells are approximations: the region graph has no polygons.

Usage:
    python render_map.py <content_dir> <land.geojson> <out.svg> [<out.html>]

land.geojson: Natural Earth land (public domain), e.g. world-atlas@2 land-50m.json converted
with topojson-client `feature(w, w.objects.land)`.
"""
import colorsys
import html
import json
import re
import sys
from pathlib import Path

import numpy as np
from scipy.spatial import Voronoi
from shapely.geometry import Point, Polygon, box
from shapely.ops import unary_union
from shapely.validation import make_valid
from shapely.affinity import translate

LAT_TOP, LAT_BOT, SCALE = 84.0, -60.0, 5.0      # equirectangular crop; px per degree
BLOC_HUES = {"StrongWest": (200, 228), "West": (185, 240), "Neutral": (70, 140),
             "East": (15, 40), "StrongEast": (350, 380)}


def main(content_dir, land_path, out_svg, out_html=None):
    c = load_content(Path(content_dir))
    land = unary_union([unwrap_ring_polygon(rings) for rings in raw_polygons(json.load(open(land_path)))])
    frame = box(-180, LAT_BOT, 180, LAT_TOP)
    land = land.intersection(frame).simplify(0.04)   # ~4 km: keeps small islands, cuts file size
    sea = frame.difference(land)

    land_ids = [r for r in c["regions"] if not c["regions"][r]["sea"] and c["regions"][r]["lat"] > LAT_BOT]
    sea_ids = [r for r in c["regions"] if c["regions"][r]["sea"]]
    cells = {**landmass_cells(c["regions"], land_ids, land), **voronoi_cells(c["regions"], sea_ids, sea)}
    colors = polity_colors(c)
    svg = draw_svg(c, cells, colors, land)
    Path(out_svg).write_text(svg)
    if out_html:
        Path(out_html).write_text(wrap_html(svg, c, colors))
    print(f"{len(land_ids)} land cells, {len(sea_ids)} sea cells -> {out_svg}")


# ───────────────────────────── drawing ─────────────────────────────

def draw_svg(c, cells, colors, land):
    w, h = 360 * SCALE, (LAT_TOP - LAT_BOT) * SCALE
    owner, regions, pol = c["ownership"], c["regions"], c["polities"]
    subject_of = {s: o for s, o, _ in c["subjects"]}
    hatched = {r for r, k in c["status"].items() if k in ("Colonial", "Occupied", "Trust")}
    hatched |= {r for r, p in owner.items() if p in subject_of}
    claimed = {}
    for r, p, k in c["claims"]:
        claimed.setdefault(r, []).append(f"{pol[p]['short']} ({k.lower()})")

    out = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w:.0f} {h + 120:.0f}" '
           f'font-family="Helvetica, Arial, sans-serif">',
           '<defs><pattern id="hatch" width="6" height="6" patternUnits="userSpaceOnUse" '
           'patternTransform="rotate(45)"><line x1="0" y1="0" x2="0" y2="6" stroke="#fff" '
           'stroke-width="1.6" stroke-opacity="0.75"/></pattern></defs>',
           f'<rect width="{w:.0f}" height="{h + 120:.0f}" fill="#f4f1ea"/>',
           f'<rect width="{w:.0f}" height="{h:.0f}" fill="#dbe7ef"/>']
    # sea cells
    out.append('<g id="sea" stroke="none" fill="#dbe7ef">')
    for r in (r for r in cells if regions[r]["sea"]):
        out.append(f'<path d="{path_d(cells[r])}" data-region="{r}"><title>{html.escape(regions[r]["name"])} (sea zone)</title></path>')
    out.append("</g>")
    # land cells
    out.append('<g id="land" stroke="#ffffff" stroke-width="0.6">')
    for r in (r for r in cells if not regions[r]["sea"]):
        p = owner.get(r)
        fill = colors.get(p, "#bdbdbd")
        tip = region_tip(c, r, subject_of, claimed)
        out.append(f'<path d="{path_d(cells[r])}" fill="{fill}" data-region="{r}"><title>{html.escape(tip)}</title></path>')
        if r in hatched:
            out.append(f'<path d="{path_d(cells[r])}" fill="url(#hatch)" stroke="none" pointer-events="none"/>')
    out.append("</g>")
    # polity borders
    out.append('<g id="borders" fill="none" stroke="#3b3b3b" stroke-width="1.1" stroke-linejoin="round">')
    for p in pol:
        geom = unary_union([cells[r] for r, o in owner.items() if o == p and r in cells])
        if not geom.is_empty:
            out.append(f'<path d="{path_d(geom)}"/>')
    out.append("</g>")
    # contested claims
    out.append('<g id="claims" fill="none" stroke="#c62828" stroke-width="1.6" stroke-dasharray="4 3">')
    for r in claimed:
        if r in cells:
            out.append(f'<path d="{path_d(cells[r])}"/>')
    out.append("</g>")
    # coastline on top for crispness
    out.append(f'<path d="{path_d(land)}" fill="none" stroke="#6b7c88" stroke-width="0.5"/>')
    # graph layer (hidden by default in HTML)
    out.append(graph_layer(c))
    # chokepoints
    out.append('<g id="chokepoints">')
    for a, b, e in c["edges"]:
        if e["chokepoint"]:
            cp = [regions[r] for r in e["chokepoint"]]
            x, y = proj(*(midpoint(cp[0], cp[1]) if len(cp) > 1 else (cp[0]["lon"], cp[0]["lat"])))
            names = ", ".join(regions[r]["name"] for r in e["chokepoint"])
            out.append(f'<path d="M{x:.1f},{y - 6:.1f} l6,6 l-6,6 l-6,-6 z" fill="#f9a825" stroke="#5d4037" '
                       f'stroke-width="0.8"><title>Chokepoint {html.escape(regions[a]["name"])} – '
                       f'{html.escape(regions[b]["name"])}; held via {html.escape(names)}</title></path>')
    out.append("</g>")
    # sea labels
    out.append('<g id="sea-labels" font-size="10" font-style="italic" fill="#5b7a91" text-anchor="middle">')
    for r in (r for r in cells if regions[r]["sea"]):
        x, y = proj(regions[r]["lon"], regions[r]["lat"])
        out.append(f'<text x="{x:.1f}" y="{y:.1f}">{html.escape(regions[r]["name"])}</text>')
    out.append("</g>")
    # capitals and polity labels
    out.append('<g id="labels" font-size="11" text-anchor="middle" paint-order="stroke" stroke="#ffffff" '
               'stroke-width="3" stroke-linejoin="round">')
    for p, d in pol.items():
        x, y = proj(regions[d["capital"]]["lon"], regions[d["capital"]]["lat"])
        weight = "bold" if d["hf"] else "normal"
        star = "★ " if d["hf"] else "• "
        out.append(f'<text x="{x:.1f}" y="{y + 4:.1f}" font-weight="{weight}" fill="#1f1f1f">'
                   f'{star}{html.escape(d["short"])}</text>')
    out.append("</g>")
    out.append(legend(h, w))
    out.append("</svg>")
    return "\n".join(out)


def graph_layer(c):
    regions = c["regions"]
    style = {"Land": 'stroke="#424242" stroke-width="0.8"',
             "Strait": 'stroke="#e65100" stroke-width="1.4" stroke-dasharray="5 3"',
             "Coast": 'stroke="#78909c" stroke-width="0.5" stroke-opacity="0.6"',
             "SeaLane": 'stroke="#1565c0" stroke-width="1" stroke-dasharray="2 3"'}
    out = ['<g id="graph" fill="none" style="display:none">']
    for a, b, e in c["edges"]:
        for (x1, y1), (x2, y2) in segments(regions[a], regions[b]):
            out.append(f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" {style[e["kind"]]}/>')
    for r, d in regions.items():
        if LAT_BOT < d["lat"] < LAT_TOP:
            x, y = proj(d["lon"], d["lat"])
            fill = "#1565c0" if d["sea"] else "#212121"
            out.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="2.2" fill="{fill}"/>')
    out.append("</g>")
    return "\n".join(out)


def legend(h, w):
    y0 = h + 18
    items = [("#2f6fb3", "Atlantican-aligned (blue family)"), ("#d9442c", "Donovian bloc (red family)"),
             ("#7fb04a", "Neutral / nonaligned (green family)"), ("#bdbdbd", "Unowned")]
    out = [f'<g id="legend" font-size="12" fill="#222">']
    for i, (col, label) in enumerate(items):
        x = 20 + i * 290
        out.append(f'<rect x="{x}" y="{y0}" width="16" height="12" fill="{col}"/><text x="{x + 22}" y="{y0 + 11}">{label}</text>')
    y1 = y0 + 26
    out.append(f'<rect x="20" y="{y1}" width="16" height="12" fill="#2f6fb3"/><rect x="20" y="{y1}" width="16" height="12" fill="url(#hatch)"/>'
               f'<text x="42" y="{y1 + 11}">Subject polity, colony, occupation or trust</text>')
    out.append(f'<rect x="310" y="{y1}" width="16" height="12" fill="none" stroke="#c62828" stroke-width="1.6" stroke-dasharray="4 3"/>'
               f'<text x="332" y="{y1 + 11}">Claimed by another polity</text>')
    out.append(f'<path d="M606,{y1} l6,6 l-6,6 l-6,-6 z" fill="#f9a825" stroke="#5d4037" stroke-width="0.8"/>'
               f'<text x="618" y="{y1 + 11}">Chokepoint</text>')
    out.append(f'<text x="780" y="{y1 + 11}">★ high-fidelity candidate · • other polity (at capital)</text>')
    out.append(f'<text x="20" y="{y1 + 34}" font-size="11" fill="#555">Expanded DATE world, January 1955. Region cells are approximate '
               f'(nearest-centroid on Natural Earth land); Antarctica (unowned) not shown. Orchestra is not affiliated with, '
               f'sponsored by, or endorsed by the U.S. Army, the Department of Defense, or the U.S. Government.</text>')
    out.append("</g>")
    return "\n".join(out)


def region_tip(c, r, subject_of, claimed):
    d, pol = c["regions"][r], c["polities"]
    p = c["ownership"].get(r)
    parts = [d["name"]]
    if p:
        parts.append(f"held by {pol[p]['name']}")
        if p in subject_of:
            parts.append(f"subject of {pol[subject_of[p]]['short']}")
    else:
        parts.append("unowned")
    if r in c["status"]:
        parts.append(c["status"][r].lower())
    if r in claimed:
        parts.append("claimed by " + ", ".join(claimed[r]))
    return " — ".join(parts)


def wrap_html(svg, c, colors):
    rows = "".join(
        f'<tr><td><span class="sw" style="background:{colors[p]}"></span>{html.escape(d["short"])}{" ★" if d["hf"] else ""}</td>'
        f'<td>{html.escape(d["name"])}</td><td>{d["prov"]}</td></tr>'
        for p, d in sorted(c["polities"].items(), key=lambda kv: kv[1]["short"]))
    return f"""<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>Orchestra — DATE world, 1955</title>
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>
 body{{margin:0;font:14px/1.4 Helvetica,Arial,sans-serif;background:#f4f1ea;color:#222}}
 header{{padding:12px 18px}} h1{{font-size:18px;margin:0 0 4px}}
 .bar{{display:flex;gap:16px;align-items:center;flex-wrap:wrap;font-size:13px}}
 #info{{min-height:1.4em;color:#333;font-weight:600}}
 .map{{overflow:auto}} .map svg{{width:100%;min-width:1100px;height:auto;display:block}}
 #land path[data-region]:hover{{filter:brightness(1.15);cursor:pointer}}
 table{{border-collapse:collapse;margin:12px 18px 24px;font-size:13px}} td{{padding:2px 10px 2px 0}}
 .sw{{display:inline-block;width:12px;height:12px;margin-right:6px;vertical-align:-1px}}
</style></head><body>
<header><h1>Expanded DATE world — January 1955 (low-fidelity)</h1>
<div class="bar"><label><input type="checkbox" id="g"> Show region graph (land, strait, coast, sea-lane edges)</label>
<label><input type="checkbox" id="l" checked> Polity labels</label><span id="info">Hover a region for details.</span></div></header>
<div class="map">{svg}</div>
<table><tr><th align="left">Polity</th><th align="left">1955 name</th><th align="left">Provenance</th></tr>{rows}</table>
<script>
const $=s=>document.querySelector(s);
$('#g').onchange=e=>$('#graph').style.display=e.target.checked?'':'none';
$('#l').onchange=e=>$('#labels').style.display=e.target.checked?'':'none';
document.querySelectorAll('[data-region]').forEach(p=>p.addEventListener('mouseenter',()=>{{
  const t=p.querySelector('title'); $('#info').textContent=t?t.textContent:'';}}));
</script></body></html>"""


# ───────────────────────────── geometry ─────────────────────────────

def voronoi_cells(regions, ids, clip):
    """Nearest-centroid cells, with ±360° ghost points so cells wrap the antimeridian."""
    pts = np.array([[regions[r]["lon"], regions[r]["lat"]] for r in ids])
    shifts = np.array([0.0, 360.0, -360.0])
    ghosts = np.vstack([pts + [s, 0] for s in shifts])
    far = np.array([[x, y] for x in (-1000, 0, 1000) for y in (-1000, 1000)] + [[-1000, 0], [1000, 0]])
    vor = Voronoi(np.vstack([ghosts, far]))
    cells = {}
    for i, r in enumerate(ids):
        poly = Polygon(vor.vertices[vor.regions[vor.point_region[i]]])
        cells[r] = poly.buffer(0).intersection(clip)
    return cells


def landmass_cells(regions, ids, land):
    """Voronoi per landmass, so a cell never jumps a strait (e.g. Chukotka stays Eurasian).
    Landmasses with no centroid of their own (Ireland, New Zealand) fall back to the nearest region overall."""
    pieces = {r: [] for r in ids}
    fallback = voronoi_cells(regions, ids, land)
    for mass in getattr(land, "geoms", [land]):
        near = mass.buffer(0.3)
        inside = [r for r in ids if near.contains(Point(regions[r]["lon"], regions[r]["lat"]))]
        if len(inside) == 1:
            pieces[inside[0]].append(mass)
        elif inside:
            for r, cell in voronoi_cells(regions, inside, mass).items():
                pieces[r].append(cell)
        else:
            for r in ids:
                pieces[r].append(fallback[r].intersection(mass))
    return {r: unary_union(v) for r, v in pieces.items()}


def path_d(geom):
    polys = getattr(geom, "geoms", [geom])
    parts = []
    for g in polys:
        if g.geom_type == "Polygon":
            for ring in [g.exterior, *g.interiors]:
                xy = proj_array(np.asarray(ring.coords))
                parts.append("M" + "L".join(f"{x:.1f},{y:.1f}" for x, y in xy) + "Z")
        elif hasattr(g, "geoms"):
            parts.append(path_d(g))
    return "".join(parts)


def raw_polygons(geojson):
    """Every polygon (as a list of rings) in a GeoJSON FeatureCollection."""
    for f in geojson["features"]:
        g = f["geometry"]
        yield from (g["coordinates"] if g["type"] == "MultiPolygon" else [g["coordinates"]])


def unwrap_ring_polygon(rings):
    """Natural Earth rings that cross ±180° (Chukotka, Wrangel, Fiji) jump across the whole map.
    Unwrap longitudes to be continuous, then cut at 180° and shift the far part back by 360°.
    A ring with an odd number of crossings encircles a pole (Antarctica) and is dropped (outside the crop)."""
    unwrapped = []
    for ring in rings:
        xy = np.asarray(ring, dtype=float)
        jumps = np.diff(xy[:, 0])
        crossings = np.abs(jumps) > 180
        if crossings.sum() % 2:
            return Polygon()
        xy[1:, 0] -= 360 * np.cumsum(np.sign(jumps) * crossings)
        unwrapped.append(xy)
    poly = make_valid(Polygon(unwrapped[0], unwrapped[1:]))
    pieces = [poly.intersection(box(-180, -90, 180, 90))]
    for shift in (360, -360):
        pieces.append(translate(poly.intersection(box(-180 - shift, -90, 180 - shift, 90)), xoff=shift))
    return unary_union(pieces)


def label_point(geom):
    p = geom.representative_point() if not geom.is_empty else None
    return proj(p.x, p.y) if p else (0, 0)


def segments(a, b):
    """Edge segments in screen space; split edges that cross the antimeridian."""
    (lo1, la1), (lo2, la2) = (a["lon"], a["lat"]), (b["lon"], b["lat"])
    if abs(lo2 - lo1) <= 180:
        return [(proj(lo1, la1), proj(lo2, la2))]
    lo2u = lo2 - 360 if lo2 > lo1 else lo2 + 360
    edge = 180 if lo1 > 0 else -180
    t = (edge - lo1) / (lo2u - lo1)
    lam = la1 + t * (la2 - la1)
    return [(proj(lo1, la1), proj(edge, lam)), (proj(-edge, lam), proj(lo2, la2))]


def midpoint(a, b):
    lo1, lo2 = a["lon"], b["lon"]
    if abs(lo2 - lo1) > 180:
        lo2 += 360 if lo2 < lo1 else -360
    lon = (lo1 + lo2) / 2
    return ((lon + 540) % 360) - 180, (a["lat"] + b["lat"]) / 2


def proj(lon, lat):
    return (lon + 180) * SCALE, (LAT_TOP - lat) * SCALE


def proj_array(xy):
    return np.column_stack([(xy[:, 0] + 180) * SCALE, (LAT_TOP - xy[:, 1]) * SCALE])


# ───────────────────────────── colours ─────────────────────────────

def polity_colors(c):
    """Hue family by stance; spread polities within a family so neighbours differ."""
    by_stance = {}
    for p in sorted(c["polities"]):
        by_stance.setdefault(c["start"][p], []).append(p)
    colors = {}
    for stance, members in by_stance.items():
        lo, hi = BLOC_HUES[stance]
        n = len(members)
        for i, p in enumerate(members):
            hue = (lo + (hi - lo) * ((i * 0.618) % 1.0)) % 360
            light = 0.42 + 0.22 * ((i * 0.381) % 1.0)
            sat = 0.55 if stance in ("Neutral",) else 0.62
            r, g, b = colorsys.hls_to_rgb(hue / 360, light, sat)
            colors[p] = "#%02x%02x%02x" % (int(r * 255), int(g * 255), int(b * 255))
    return colors


# ───────────────────────────── content parsing ─────────────────────────────

def load_content(root):
    """Minimal RON reader for the WS-08a files (regular, machine-written shapes only)."""
    scen = (root / "scenarios/date_1955.ron").read_text()
    map_dir = root / "maps" / grab(scen, r'map: "(\w+)"')
    set_dir = root / "settings" / grab(scen, r'setting: "(\w+)"')
    sea_terrain = {"Ocean", "Littoral", "IceSea"}
    regions = {}
    for m in re.finditer(r'Region\(id: "([^"]+)", name: "([^"]+)", lat: ([-\d.]+), lon: ([-\d.]+), terrain: (\w+)',
                         (map_dir / "regions.ron").read_text()):
        rid, name, lat, lon, terrain = m.groups()
        regions[rid] = {"name": name, "lat": float(lat), "lon": float(lon), "sea": terrain in sea_terrain}
    edges = []
    for m in re.finditer(r'\("([^"]+)", "([^"]+)", RegionEdge\(kind: (\w+), cost: \d+(?:, chokepoint: \[([^\]]*)\])?\)\)',
                         (map_dir / "edges.ron").read_text()):
        a, b, kind, cp = m.groups()
        edges.append((a, b, {"kind": kind, "chokepoint": re.findall(r'"([^"]+)"', cp or "")}))
    polities = {}
    for f in sorted((set_dir / "polities").glob("*.ron")):
        t = f.read_text()
        pid = grab(t, r'id: "([^"]+)"')
        polities[pid] = {"name": grab(t, r'\bname: "([^"]+)"'), "short": grab(t, r'short_name: "([^"]+)"'),
                         "capital": grab(t, r'capital: "([^"]+)"'), "hf": "hf_candidate: true" in t,
                         "prov": grab(t, r"provenance: (\w+)")}
    pairs = lambda f: re.findall(r'\("([^"]+)", "([^"]+)"\)', (set_dir / f).read_text())
    triples = lambda f: re.findall(r'\("([^"]+)", "([^"]+)", (\w+)\)', (set_dir / f).read_text())
    status = dict(re.findall(r'\("([^"]+)", (\w+)\)', (set_dir / "status.ron").read_text()))
    start = dict(re.findall(r'polity: "([^"]+)",.*?stance: (\w+)', (set_dir / "start.ron").read_text(), re.S))
    return {"regions": regions, "edges": edges, "polities": polities, "ownership": dict(pairs("ownership.ron")),
            "subjects": triples("subjects.ron"), "claims": triples("claims.ron"), "status": status, "start": start}


def grab(text, pattern):
    m = re.search(pattern, text)
    return m.group(1) if m else ""


if __name__ == "__main__":
    main(*sys.argv[1:])
