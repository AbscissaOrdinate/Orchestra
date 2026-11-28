"""World truth (P-1), spec §4.1. Every draw is a pure function of (seed, world, content key, slot), the
analogue of ARC-034's worldgen_stream: adding content never changes another item's draws (RND-007)."""
from hashlib import blake2b
from types import SimpleNamespace
import numpy as np
from scipy.special import betaincinv, ndtri
from params import P

SLOT = dict(works=0, gated=1, n_en=2, en1=3, en2=4, d1=5, d2=6, kind=7, cmult=8, pmult=9, pmode=10,
            pnoise=11, kappa=12, edge=13, etarget=14, eb=15, pline=16, rep=17)


def sample_truth(c, seed, worlds):
    """Truth arrays of shape (W, T) for the given world indices; T = techs in content `c`."""
    w = np.asarray(worlds, np.uint64)[:, None]
    th = key_hashes(c.tech_id)[None, :]
    u = lambda name: keyed_u(seed, w, th, SLOT[name])
    cls, line, hist = c.tech_cls, c.tech_line, c.tech_hist

    a, b = P["beta_ab"][c.cls].T
    p_line = betaincinv(a, b, keyed_u(seed, w, key_hashes(c.line_id)[None, :], SLOT["pline"]))     # (W, L)
    works = hist | (u("works") < p_line[:, line])
    n_cand = (c.cands[line] >= 0).sum(1)                                                          # (T,)
    gated = works & ~hist & (u("gated") < P["s_gate"][cls]) & (n_cand > 0)

    two = (u("n_en") < P["p_two_enablers"]) & (n_cand >= 2)
    pick1 = np.minimum((u("en1") * n_cand).astype(int), np.maximum(n_cand - 1, 0))
    pick2 = (u("en2") * np.maximum(n_cand - 1, 1)).astype(int)
    pick2 = pick2 + (pick2 >= pick1)
    rows = np.arange(c.n_techs)
    enab = -np.ones(works.shape + (2,), int)
    for j, (pick, d_slot) in enumerate(((pick1, "d1"), (pick2, "d2"))):
        el = c.cands[line[None, :], np.minimum(pick, c.cands.shape[1] - 1)]                       # enabler lines
        target = c.tech_D + P["enabler_offset"][0] + u(d_slot) * np.ptp(P["enabler_offset"])
        gen = np.abs(c.line_D[np.maximum(el, 0)] - target[..., None]).argmin(-1)
        tech = c.line_tech[np.maximum(el, 0), gen]
        enab[..., j] = np.where(gated & ((j == 0) | two), tech, -1)

    requires = gated & (u("kind") < P["p_requires"])
    lo, hi = np.log(P["penalty_cost_range"])
    cost_mult = np.where(gated & ~requires, np.exp(lo + u("cmult") * (hi - lo)), 1.0)
    p0, p1 = P["penalty_perf_range"]
    perf_mult = np.where(gated & ~requires, p0 + u("pmult") * (p1 - p0), 1.0)

    mode = np.where(hist, 1, np.searchsorted(np.cumsum(P["perf_w"]), u("pmode")).clip(0, 2))
    ln_perf = np.log(P["perf_median"][mode]) + P["perf_sigma"] * ndtri(u("pnoise"))
    ln_kappa = P["sigma_kappa"] * ndtri(u("kappa"))

    reach = np.ones_like(works)
    for k in range(1, c.line_tech.shape[1]):            # a generation is reachable if every earlier one works
        t = c.line_tech[:, k]
        t, prev = t[t >= 0], c.line_tech[:, k - 1][c.line_tech[:, k] >= 0]
        reach[:, t] = reach[:, prev] & works[:, prev]
    return SimpleNamespace(works=works, gated=gated, requires=requires, enabler=enab, n_enablers=(enab >= 0).sum(-1),
                           cost_mult=cost_mult, perf_mult=perf_mult, perf_mode=mode, ln_perf=ln_perf,
                           ln_kappa=ln_kappa, reachable=reach, p_line=p_line)


def sample_edges(c, seed, worlds):
    """Procedural Boost edges (§4.1.2): list of (world, src_tech, dst_tech, b) arrays. Authored edges: none here."""
    L = c.n_lines
    w = np.asarray(worlds, np.uint64)[:, None, None]
    ph = pair_hashes(c.line_id)[None]
    exists = keyed_u(seed, w, ph, SLOT["edge"]) < P["p_edge"]
    exists &= (c.domain[:, None] != c.domain[None, :])[None]                 # different fields (domain proxy)
    wi, a, b = np.nonzero(exists)
    wu = np.asarray(worlds, np.uint64)[wi]
    ph_sel = pair_hashes(c.line_id)[a, b]
    gen = (keyed_u(seed, wu, ph_sel, SLOT["etarget"]) * c.ngen[b]).astype(int)
    d_tgt = c.line_D[b, gen]
    src_ok = c.line_D[a] <= d_tgt[:, None]                                    # (E, maxgen)
    has = src_ok.any(1)
    src_gen = np.where(src_ok, np.arange(src_ok.shape[1]), -1).max(1)
    bb = P["edge_b"][0] + keyed_u(seed, wu, ph_sel, SLOT["eb"]) * np.ptp(P["edge_b"])
    return SimpleNamespace(world=wi[has], src=c.line_tech[a[has], src_gen[has]], dst=c.line_tech[b[has], gen[has]], b=bb[has],
                           p_total=P["p_edge"] * (c.domain[:, None] != c.domain[None, :]).sum())


def sample_repeatable(seed, worlds, n_lines=P["n_repeatable"], levels=P["rep_levels"]):
    """True gains g_k = g0 d^(k-1) min(LogNormal(mean 1, CV rep_cv), rep_clip), shape (W, lines, levels)  (§4.1.4)."""
    w = np.asarray(worlds, np.uint64)[:, None, None]
    h = key_hashes([f"rep{i:02d}.r{k + 1:02d}" for i in range(n_lines) for k in range(levels)]).reshape(1, n_lines, levels)
    s2 = np.log1p(P["rep_cv"] ** 2)
    m = np.exp(-s2 / 2 + np.sqrt(s2) * ndtri(keyed_u(seed, w, h, SLOT["rep"])))
    return P["rep_g0"] * P["rep_d"] ** np.arange(levels) * np.minimum(m, P["rep_clip"])


# ---------------------------------------------------------------- helpers

def key_hashes(ids):
    return np.array([int.from_bytes(blake2b(s.encode(), digest_size=8).digest(), "little") for s in ids], np.uint64)


def pair_hashes(line_ids):
    h = key_hashes(line_ids)
    return splitmix(h[:, None] * np.uint64(31) ^ splitmix(h[None, :]))


def splitmix(x):
    with np.errstate(over="ignore"):
        x = np.asarray(x, np.uint64) + np.uint64(0x9E3779B97F4A7C15)
        x = (x ^ (x >> np.uint64(30))) * np.uint64(0xBF58476D1CE4E5B9)
        x = (x ^ (x >> np.uint64(27))) * np.uint64(0x94D049BB133111EB)
        return x ^ (x >> np.uint64(31))


def keyed_u(seed, world, key, slot):
    """Uniform(0,1) as a pure function of (seed, world, key, slot); arrays broadcast."""
    h = splitmix(splitmix(splitmix(np.uint64(seed)) ^ world) ^ key)
    h = splitmix(h ^ np.uint64(slot + 1))
    return ((h >> np.uint64(11)).astype(float) + 0.5) / float(1 << 53)
