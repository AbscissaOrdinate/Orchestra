"""Synthetic default content with the spec §6.3 counts and class mix (stands in for content/core/rnd/).

Lines and singletons (1-generation lines) are ranked by first date; gate candidates come only from
lower-ranked lines, so the prerequisite + candidate graph is acyclic by construction (RND-007).
"""
from types import SimpleNamespace
import numpy as np
from params import P

GEN_WEIGHTS = {2: 0.20, 3: 0.30, 4: 0.25, 5: 0.15, 6: 0.10}


def make_content(seed=1):
    rng = np.random.default_rng(seed)
    dom, ngen = split_domains(rng)
    first, step = draw_dates(rng, ngen)
    first, step = add_legacy(rng, ngen, first, step)
    L = len(ngen)
    rank = np.argsort(np.argsort(first + rng.random(L)))            # 0 = earliest line
    cls = assign_class(rng, ngen, first, step)
    maxgen = ngen.max()
    line_tech = -np.ones((L, maxgen), int)
    line_D = np.full((L, maxgen), np.inf)
    tech_line, tech_gen = [], []
    for i in range(L):
        for k in range(ngen[i]):
            line_tech[i, k] = len(tech_line)
            line_D[i, k] = first[i] + k * step[i]
            tech_line.append(i)
            tech_gen.append(k + 1)
    tech_line, tech_gen = np.array(tech_line), np.array(tech_gen)
    base_dev = rng.uniform(80, 400, L) * np.where(ngen == 1, 1.0, 1.0)
    step_dev = rng.uniform(1.3, 1.7, L)
    cands = pick_candidates(rng, rank)
    return SimpleNamespace(
        n_lines=L, n_techs=len(tech_line), domain=dom, ngen=ngen, first=first, step=step, cls=cls, rank=rank,
        line_tech=line_tech, line_D=line_D, tech_line=tech_line, tech_gen=tech_gen,
        tech_D=line_D[tech_line, tech_gen - 1], tech_cls=cls[tech_line], tech_domain=dom[tech_line],
        tech_hist=line_D[tech_line, tech_gen - 1] <= P["start_year"],
        base_dev=base_dev, step_dev=step_dev, cands=cands,
        line_id=np.array([f"line{i:02d}" for i in range(L)]),
        tech_id=np.array([f"tech.line{tech_line[t]:02d}.g{tech_gen[t]}" for t in range(len(tech_line))]),
    )


# ---------------------------------------------------------------- helpers

def split_domains(rng):
    """Per domain: 1-2 singletons plus lines of 2-6 generations summing to the §6.3 node count."""
    dom, ngen = [], []
    gens, probs = np.array(list(GEN_WEIGHTS)), np.array(list(GEN_WEIGHTS.values()))
    for d, nodes in enumerate(P["domain_nodes"]):
        n_single = 2 if d < 8 else 1
        dom += [d] * n_single
        ngen += [1] * n_single
        left = nodes - n_single
        while left > 0:
            g = min(rng.choice(gens, p=probs), left)
            if left - g == 1:                       # avoid a stray 1-generation remainder
                g = g + 1 if g < 6 else g - 1
            dom.append(d)
            ngen.append(g)
            left -= g
    return np.array(dom), np.array(ngen)


def draw_dates(rng, ngen):
    step = rng.uniform(8, 18, len(ngen))
    last_ok = P["end_year"] - (ngen - 1) * step
    first = rng.uniform(P["start_year"] - 2, np.maximum(last_ok, P["start_year"] - 1))
    return first, step


def add_legacy(rng, ngen, first, step):
    """Pull random multi-generation lines back before 1955 until about 45 historical techs exist."""
    first, step = first.copy(), step.copy()
    for i in rng.permutation(np.flatnonzero(ngen >= 3)):
        if n_historical(ngen, first, step) >= P["n_historical"]:
            break
        first[i], step[i] = rng.uniform(1925, 1950), rng.uniform(6, 12)
    return first, step


def n_historical(ngen, first, step):
    return sum(int(((first[i] + np.arange(ngen[i]) * step[i]) <= P["start_year"]).sum()) for i in range(len(ngen)))


def assign_class(rng, ngen, first, step):
    """Early lines tend to be Established, late lines Fringe; quotas follow the §6.3 class mix."""
    nonhist = np.array([int(((first[i] + np.arange(ngen[i]) * step[i]) > P["start_year"]).sum()) for i in range(len(ngen))])
    quota = np.cumsum(np.array(P["class_mix"], float) / sum(P["class_mix"]))[:-1] * nonhist.sum()
    order = np.argsort(first + rng.normal(0, 12, len(ngen)))
    mid = np.cumsum(nonhist[order]) - nonhist[order] / 2
    cls = np.empty(len(ngen), int)
    cls[order] = np.searchsorted(quota, mid)
    return cls


def pick_candidates(rng, rank, width=5):
    """2-5 gate candidates per line from lower-ranked lines (padded with -1)."""
    L = len(rank)
    by_rank = np.argsort(rank)
    out = -np.ones((L, width), int)
    for i in range(L):
        earlier = by_rank[:rank[i]]
        if len(earlier):
            n = min(rng.integers(2, width + 1), len(earlier))
            out[i, :n] = rng.choice(earlier, n, replace=False)
    return out


if __name__ == "__main__":
    c = make_content()
    print("lines", c.n_lines, "techs", c.n_techs, "historical", int(c.tech_hist.sum()))
    print("class mix (non-historical techs):", [int(((c.tech_cls == k) & ~c.tech_hist).sum()) for k in range(4)])
