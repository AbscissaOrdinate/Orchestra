"""Engineering projects, spec §4.5: execution growth, schedule growth, estimate(), phase cost split."""
import numpy as np
from scipy.special import ndtri
from scipy.stats import qmc
import belief as B
from params import P, immaturity


def lognormal(mean, cv, z):
    s2 = np.log1p(cv ** 2)
    return np.exp(np.log(mean) - s2 / 2 + np.sqrt(s2) * z)


def growth(u, z, p=P):
    """(G, S) from standard normals z[..., 0:2] given immaturity u; corr(ln G, ln S) = rho (§4.5.3)."""
    g = lognormal(p["g_mean"][0] + p["g_mean"][1] * u, p["g_cv"][0] + p["g_cv"][1] * u, z[..., 0])
    zs = p["rho"] * z[..., 0] + np.sqrt(1 - p["rho"] ** 2) * z[..., 1]
    return g, lognormal(p["s_mean"][0] + p["s_mean"][1] * u, p["s_cv"][0] + p["s_cv"][1] * u, zs)


def start_belief(trl, ln_kappa, rng):
    """Cost belief of a project's critical tech at start: prior + science to TRL, tests above TRL 5.4,
    proof observation for a mature tech (§4.3, §4.5.5). Returns the belief tuple."""
    m_sci = np.clip((trl - 1) / 3, 0, 1)
    tests = np.where(trl >= 6, 2, (trl >= 5.4).astype(int))
    sd = 1 / np.sqrt(P["info"]["cost"] * m_sci + P["test_prec"]["cost"] * tests)
    b1 = B.observe(B.prior_cost(trl.shape), ln_kappa + sd * rng.standard_normal(trl.shape), sd)
    sd_p = P["cost_obs_on_proof"]
    b2 = B.observe(b1, ln_kappa + sd_p * rng.standard_normal(trl.shape), sd_p)
    return tuple(np.where((trl >= 6)[..., None], x2, x1) for x1, x2 in zip(b1, b2))


def estimate(b_cost, u, qs=(0.5, 0.8), seed=0):
    """P-quantile budget multipliers (cost, schedule) relative to the engineering point estimate (§4.5.7).
    Same 256 standard normals for every call, so the result is a pure function of (belief, u)."""
    z = ndtri(qmc.Sobol(3, scramble=True, seed=seed).random(P["n_estimate"]))   # fixed low-discrepancy stream
    g, s = growth(np.asarray(u)[..., None], z)                                   # (..., n)
    w, mu, sd = b_cost
    ln_k = mu[..., 0:1] + sd[..., 0:1] * z[:, 2]                                 # cost belief is single-component
    cost = np.exp(ln_k) / B.mean(b_cost, log=True)[..., None] * g
    return [np.quantile(cost, q, axis=-1) for q in qs], [np.quantile(s, q, axis=-1) for q in qs]


def phase_multipliers(g):
    """Cost-rate multiplier per phase when the excess growth (G - 1) is allocated by variance share (§4.5.4)."""
    return 1 + (g[..., None] - 1) * P["phase_var_share"] / P["phase_w"]
