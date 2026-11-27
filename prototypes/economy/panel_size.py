"""WS-03 §3.5 household-panel size study (v0.3): equal-weight vs. top-oversampled panels.

Compares bin-level wealth, top-1% income share and top-bracket tax base against a 2^18-household
reference using the US calibration from calib_wealth.py. Run: python panel_size.py
"""
import os, sys
import numpy as np
from scipy.stats import norm, qmc
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from calib_wealth import body_tail_q, wealth_q

US_W, LAB, RHO, BETA, R = (1.978, 1.484, 0.306), (0.664, 1.441), 0.176, 5.0, 0.05
STRATA = [(0.0, 0.9, 0.7), (0.9, 0.99, 0.2), (0.99, 1.0, 0.1)]   # (score range lo, hi, share of households)


def main():
    ref = metrics(2**18, stratified=False)
    print(" H      design      | worst bin-wealth err | top-1% income share err | top-bracket base err")
    for n in (4096, 8192, 16384, 32768):
        for strat in (False, True):
            m = metrics(n, strat)
            print(f"{n:6d}  {'oversampled' if strat else 'equal      '} | {np.max(np.abs(m[0] / ref[0] - 1)):19.1%} | "
                  f"{abs(m[1] - ref[1]):23.4f} | {abs(m[2] / ref[2] - 1):19.1%}")


# ---------------- helpers ----------------
def metrics(n, stratified):
    v = np.clip(qmc.Sobol(d=2, scramble=False).random_base2(int(np.log2(n))), 0.5 / n, 1 - 0.5 / n)
    (u1, w1), (u2, w2) = (oversample(v[:, 0]), oversample(v[:, 1])) if stratified else ((v[:, 0], 1), (v[:, 1], 1))
    wt = np.broadcast_to(np.asarray(w1 * w2, float), (n,)).copy(); wt /= wt.sum()
    zl = norm.ppf(u1); zw = RHO * zl + np.sqrt(1 - RHO**2) * norm.ppf(u2)
    lab = cell_values(zl, wt, lambda q: body_tail_q(q, *LAB)); lab /= wt @ lab
    w = cell_values(zw, wt, lambda q: wealth_q(q, *US_W)); w *= BETA / (wt @ w)
    y = lab + R * w
    o = np.argsort(y, kind="stable"); cw = np.cumsum(wt[o])
    b = np.minimum((cw * 20 - 1e-12).astype(int), 19)
    bin_w = np.bincount(b, wt[o] * w[o], 20) / np.bincount(b, wt[o], 20) / (wt @ w)
    top = cw > 0.99
    top1 = (wt[o][top] @ y[o][top]) / (wt @ y)
    base = wt @ np.maximum(y - y[o][np.argmax(top)], 0) / (wt @ y)
    return bin_w, top1, base


def oversample(v):
    """Map lattice coordinate v to score quantile u, putting STRATA shares of households in each range; return weights."""
    u, wt, start = np.empty_like(v), np.empty_like(v), 0.0
    for lo, hi, share in STRATA:
        m = (v >= start) & (v < start + share)
        u[m] = lo + (v[m] - start) / share * (hi - lo)
        wt[m] = (hi - lo) / share
        start += share
    return u, wt


def cell_values(score, wt, qfun, sub=8):
    """Each household = mean of the marginal over its weighted rank cell, so margins are exact."""
    o = np.argsort(score, kind="stable")
    c = np.concatenate([[0.0], np.cumsum(wt[o])]); c /= c[-1]
    u = c[:-1, None] + np.diff(c)[:, None] * (np.arange(sub) + 0.5) / sub
    vals = np.empty(len(score)); vals[o] = qfun(np.minimum(u, 1 - 1e-12)).mean(axis=1)
    return vals


if __name__ == "__main__":
    main()
