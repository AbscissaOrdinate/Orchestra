"""M2(b) seed script: fit the WS-05 design-project growth model to the [V] anchors.

Model (WS-05 spec §3.5): realized growth vs. the engineering point estimate is
    R = E * G
E = C_true / C_hat : estimation error, lognormal, mean 1, log-sd sigma_b(u)
G               : execution growth, lognormal, mean m_G(u), CV c_G(u)
u = clamp((6 - TRL_min) / 3, 0, 1)   immaturity of the least mature critical tech

Anchors [V]:
  GAO-06-391: RDT&E growth 4.8% (mature at start) vs 34.9% (immature)
  GAO-07-406SP: 2.6% vs 32.3%; 16% of systems fully mature at development start
  Defense ARJ 32(1), 2025 (via m1-findings §3.2): ACAT I mean 1.36, CV 0.80
  RAND (1971), DTIC AD0730921: ~15% average schedule slip

Run:  python calib_growth.py
"""
import numpy as np

N = 400_000
SEED = 20261127
TARGET = dict(mature_mean=1.04, immature_mean=1.34, overall_mean=1.36, overall_cv=0.80)


def main():
    rng = np.random.default_rng(SEED)
    u = start_mix(rng, N)
    z = rng.standard_normal((3, N))

    # grid-fit the immaturity slopes of G (mean slope a1, CV slope c1) and belief error slope
    best = None
    for a1 in np.arange(0.20, 0.60, 0.02):
        for c1 in np.arange(0.40, 1.40, 0.05):
            r, _ = growth(u, z, a1, c1)
            st = summary(r, u)
            err = ((st["mature_mean"] - TARGET["mature_mean"]) / 0.02) ** 2 \
                + ((st["immature_mean"] - TARGET["immature_mean"]) / 0.03) ** 2 \
                + ((st["overall_cv"] - TARGET["overall_cv"]) / 0.05) ** 2
            if best is None or err < best[0]:
                best = (err, a1, c1, st)

    _, a1, c1, st = best
    r, s = growth(u, z, a1, c1)
    print("fitted: m_G(u) = 1.03 + %.2f u ; c_G(u) = 0.20 + %.2f u" % (a1, c1))
    print("cost growth  R : " + fmt(st))
    print("  p50 %.3f  p85 %.3f  p95 %.3f" % tuple(np.quantile(r, [0.5, 0.85, 0.95])))
    print("schedule     S : mean %.3f  cv %.3f  corr(lnR, lnS) %.2f"
          % (s.mean(), s.std() / s.mean(), np.corrcoef(np.log(r), np.log(s))[0, 1]))
    print("by TRL_min:")
    for trl in (3.0, 4.0, 5.0, 6.0):
        uu = np.full(N, immaturity(trl))
        rr, ss = growth(uu, z, a1, c1)
        print("  TRL %.0f  R mean %.3f cv %.3f p85 %.3f | S mean %.3f"
              % (trl, rr.mean(), rr.std() / rr.mean(), np.quantile(rr, 0.85), ss.mean()))
    check_binary_evidence()


# ---------------------------------------------------------------- helpers

def start_mix(rng, n):
    """16% start with every critical tech mature (GAO-07-406SP); the rest TRL_min ~ U[3, 5.5]."""
    mature = rng.random(n) < 0.16
    trl = np.where(mature, 6.0, rng.uniform(3.0, 5.5, n))
    return immaturity(trl)


def immaturity(trl):
    return np.clip((6.0 - np.asarray(trl, float)) / 3.0, 0.0, 1.0)


def lognormal(mean, cv, z):
    s2 = np.log1p(cv ** 2)
    return np.exp(np.log(mean) - s2 / 2 + np.sqrt(s2) * z)


def growth(u, z, a1, c1, rho=0.6):
    """Realized cost growth R = E*G and schedule factor S (corr rho with ln G)."""
    sigma_b = 0.10 + 0.25 * u                      # belief log-sd at start [D]
    e = np.exp(-sigma_b ** 2 / 2 + sigma_b * z[0])  # mean-1 estimation error
    g = lognormal(1.03 + a1 * u, 0.20 + c1 * u, z[1])
    zs = rho * z[1] + np.sqrt(1 - rho ** 2) * z[2]
    s = lognormal(1.04 + 0.30 * u, 0.15 + 0.30 * u, zs)
    return e * g, s


def summary(r, u):
    return dict(mature_mean=r[u == 0].mean(), immature_mean=r[u > 0].mean(),
                overall_mean=r.mean(), overall_cv=r.std() / r.mean())


def fmt(st):
    return "  ".join("%s %.3f (target %.2f)" % (k, v, TARGET[k]) for k, v in st.items())


def check_binary_evidence():
    """RND-011: a Gaussian observation on the +/-1 viability latent reproduces Bernoulli Bayes."""
    sc = 1e-3
    for prior, lr in ((0.4, 9.0), (0.8, 1 / 6.0), (0.5, 50.0)):
        obs = 1.0 if lr > 1 else -1.0
        noise2 = 2.0 / abs(np.log(lr)) - sc ** 2
        var = sc ** 2 + noise2
        like_w = np.exp(-(obs - 1) ** 2 / (2 * var))
        like_d = np.exp(-(obs + 1) ** 2 / (2 * var))
        post = prior * like_w / (prior * like_w + (1 - prior) * like_d)
        exact = prior * lr / (prior * lr + 1 - prior)
        print("evidence: prior %.2f LR %6.3f -> mixture %.6f exact %.6f" % (prior, lr, post, exact))


if __name__ == "__main__":
    main()
