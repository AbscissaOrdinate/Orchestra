"""WS-03 §3.5 wealth calibration: marginal fits and the joint income-wealth fit.

Reproduces the numbers quoted in WS-03 v0.2 §3.5. Run: python calib_wealth.py
Data (all [V], see spec §3.5 table): Fed DFA 2026Q2 US wealth shares; WIR2022 Algeria;
SCF 2022 mean net worth by percentile of usual income; WIR US income top shares.
"""
import numpy as np
from scipy.stats import norm, qmc
from scipy.optimize import brentq, minimize, minimize_scalar

P_T = 0.90                                       # body/tail junction
GRID = (np.arange(20000) + 0.5) / 20000          # quantile midpoints for marginal fits
SCF_2022 = np.array([129.7, 218.7, 385.4, 636.8, 1264.7, 6629.6]) / 1063.7
GROUPS = [(0, 4), (4, 8), (8, 12), (12, 16), (16, 18), (18, 20)]   # income vigintile ranges of the SCF groups


def main():
    us = fit_wealth(top10=0.689, top1=0.325, bot50=0.023)        # Fed DFA 2026Q2
    dz = fit_wealth(top10=0.570, top1=0.248, bot50=0.059)        # WIR2022 Algeria
    for name, f in (("US (Fed DFA 2026Q2)", us), ("Algeria (WIR2022)", dz)):
        print(f"{name}: sigma_w={f[0]:.3f} alpha_w={f[1]:.3f} p0={f[2]:.3f} ->", rounded(shares(wealth_q(GRID, *f))))

    z = lattice_scores(2**15)
    beta, r = 5.0, 0.05                                          # W/Y ~ 500% [V PZ], net return [R]
    res = min((minimize(joint_loss, x0, args=(z, us, beta, r), method="Nelder-Mead",
                        options=dict(xatol=1e-3, fatol=1e-7, maxiter=500))
               for x0 in ([0.9, 1.8, 0.4], [0.8, 1.6, 0.25])), key=lambda o: o.fun)
    sL, aL, rho = res.x
    g, inc, cap = joint(z, rho, us, (sL, aL), beta, r)
    print(f"joint fit: sigma_L={sL:.3f} alpha_L={aL:.3f} rho_LW={rho:.3f}")
    print("  mean wealth by income group / mean:", np.round(g, 3), "SCF 2022:", np.round(SCF_2022, 3),
          f"log RMSE {log_rmse(g):.3f}")
    print(f"  total-income top10 {inc[0]:.3f}, top1 {inc[1]:.3f} (US ~0.47, ~0.20); capital-role share of top vigintile {cap:.2f}")

    # comparisons: v0.1 linear key-wealth rule and a Gaussian copula without the capital-income channel
    wv = vigintile_means(wealth_q(GRID, *us))
    for c in (0.6, 0.9):
        print(f"linear rule c={c}: log RMSE {log_rmse(group_means(c * wv + (1 - c))):.3f}")
    cop = minimize_scalar(lambda rr: log_rmse(group_means(copula_vigintiles(us, rr))), bounds=(0, 0.99), method="bounded")
    print(f"Gaussian copula on income rank (best rho={cop.x:.2f}): log RMSE {cop.fun:.3f}")


# ---------------- helpers ----------------
def body_tail_q(u, sigma, alpha):
    """Lognormal body below P_T, Pareto tail above, continuous at P_T (unnormalized)."""
    qt = np.exp(sigma * norm.ppf(P_T))
    body = np.exp(sigma * norm.ppf(np.minimum(u, P_T)))
    tail = qt * ((1 - P_T) / np.maximum(1 - u, 1e-15)) ** (1 / alpha)
    return np.where(u < P_T, body, tail)


def wealth_q(u, sigma, alpha, p0):
    q = np.where(u < p0, 0.0, body_tail_q(u, sigma, alpha))
    return q / q.mean() if q.ndim and q.size == GRID.size else q


def shares(q):
    tot = q.sum()
    top = lambda f: q[GRID >= 1 - f].sum() / tot
    return dict(top10=top(0.1), top1=top(0.01), top01=top(0.001), bot50=q[GRID < 0.5].sum() / tot)


def fit_wealth(top10, top1, bot50):
    alpha = 1 / (1 - np.log(top1 / top10) / np.log(0.1))        # Pareto: top1/top10 = 0.1^(1-1/alpha)
    sig = lambda p0: brentq(lambda s: shares(wealth_q(GRID, s, alpha, p0))["top10"] - top10, 0.05, 4.0)
    p0 = brentq(lambda p0: shares(wealth_q(GRID, sig(p0), alpha, p0))["bot50"] - bot50, 0.0, 0.45)
    return sig(p0), alpha, p0


def lattice_scores(n):
    u = qmc.Sobol(d=2, scramble=False).random(n)[1:]            # deterministic; spec uses a rank-1 lattice
    return norm.ppf(u)


def joint(z, rho, wpar, lpar, beta, r):
    zl, zw = z[:, 0], rho * z[:, 0] + np.sqrt(1 - rho**2) * z[:, 1]
    lab = rank_cell_values(zl, lambda u: body_tail_q(u, *lpar)); lab /= lab.mean()
    w = rank_cell_values(zw, lambda u: wealth_q(u, *wpar)); w *= beta / w.mean()
    y = lab + r * w
    bins = np.array_split(np.argsort(y, kind="stable"), 20)
    g = group_means(np.array([w[b].mean() for b in bins]) / w.mean())
    ys = np.sort(y)[::-1]
    inc = (ys[: len(y) // 10].sum() / ys.sum(), ys[: len(y) // 100].sum() / ys.sum())
    cap = np.mean(r * w[bins[-1]] > lab[bins[-1]])
    return g, inc, cap


def rank_cell_values(score, qfun, sub=8):
    """Each household gets the mean of the marginal over its rank cell (margins exact by construction)."""
    n = len(score)
    ranks = np.empty(n, int); ranks[np.argsort(score, kind="stable")] = np.arange(n)
    u = (ranks[:, None] + (np.arange(sub) + 0.5) / sub) / n
    return qfun(u).mean(axis=1)


def joint_loss(x, z, wpar, beta, r):
    sL, aL, rho = x
    if not (0.3 < sL < 1.5 and 1.2 < aL < 4 and 0 < rho < 0.95):
        return 1e3
    g, inc, _ = joint(z, rho, wpar, (sL, aL), beta, r)
    return np.mean(np.log(g / SCF_2022) ** 2) + 4 * (np.log(inc[0] / 0.47) ** 2 + np.log(inc[1] / 0.20) ** 2)


def copula_vigintiles(wpar, rho, n_y=20, n_e=400):
    zy = norm.ppf((np.arange(n_y * n_e) + 0.5) / (n_y * n_e))
    e = norm.ppf((np.arange(n_e) + 0.5) / n_e)
    w = wealth_q(norm.cdf(rho * zy[:, None] + np.sqrt(1 - rho**2) * e[None, :]), *wpar)
    m = w.mean(axis=1)
    return (m / m.mean()).reshape(n_y, n_e).mean(axis=1)


def vigintile_means(q):
    v = q.reshape(20, -1).mean(axis=1)
    return v / v.mean()


def group_means(vig):
    return np.array([vig[a:b].mean() for a, b in GROUPS])


def log_rmse(g):
    return float(np.sqrt(np.mean(np.log(g / SCF_2022) ** 2)))


def rounded(d):
    return {k: round(float(v), 3) for k, v in d.items()}


if __name__ == "__main__":
    main()
