"""ADR-0008 belief: Gaussian mixture of up to 3 components on a linear or log scale, spec §4.2.
Arrays hold (..., 3) of weight w, mean mu, sd sd; unused components carry w = 0. Observations are exact."""
import numpy as np
from scipy.special import ndtr
from params import P


def prior_viability(p_bar):
    """Two components at +1 (not hard-dead) and -1 (hard-dead), width sigma_c (§4.2.1)."""
    p_bar = np.asarray(p_bar, float)
    z = np.zeros_like(p_bar)
    w = np.stack([p_bar, 1 - p_bar, z], -1)
    return w, np.stack([z + 1, z - 1, z], -1), np.full(w.shape, P["sigma_c"])


def prior_perf(shape=()):
    """Log-scale mixture (disappointing, nominal, breakthrough)."""
    w = np.broadcast_to(P["perf_w"], shape + (3,)).copy()
    return w, np.broadcast_to(np.log(P["perf_median"]), shape + (3,)).copy(), np.full(shape + (3,), P["perf_sigma"])


def prior_cost(shape=()):
    """Single Normal(0, sigma_kappa) on the log scale (two zero-weight spare components)."""
    w = np.zeros(shape + (3,))
    w[..., 0] = 1.0
    return w, np.zeros(shape + (3,)), np.full(shape + (3,), P["sigma_kappa"])


def observe(b, y, noise_sd):
    """Exact Bayesian update of belief b = (w, mu, sd) with value y observed through Gaussian noise noise_sd."""
    w, mu, sd = b
    y, noise_sd = np.asarray(y, float)[..., None], np.asarray(noise_sd, float)[..., None]
    var = sd ** 2 + noise_sd ** 2
    with np.errstate(divide="ignore"):
        logw = np.log(w) - (y - mu) ** 2 / (2 * var) - 0.5 * np.log(var)
    logw -= logw.max(-1, keepdims=True)
    w_new = np.exp(logw)
    w_new /= w_new.sum(-1, keepdims=True)
    tau = 1 / sd ** 2 + 1 / noise_sd ** 2
    return w_new, (mu / sd ** 2 + y / noise_sd ** 2) / tau, tau ** -0.5


def observe_binary(b, ln_lr):
    """Binary test outcome with likelihood ratio exp(ln_lr) (works vs dead) as a +-1 observation (§4.2.3)."""
    ln_lr = np.asarray(ln_lr, float)
    noise2 = 2.0 / np.abs(ln_lr) - P["sigma_c"] ** 2
    return observe(b, np.sign(ln_lr), np.sqrt(noise2))


def p_works(b):
    w, mu, _ = b
    return (w * (mu > 0)).sum(-1)


def mean(b, log=False):
    """Mixture mean in natural units (log=True: E[exp(x)])."""
    w, mu, sd = b
    return (w * (np.exp(mu + sd ** 2 / 2) if log else mu)).sum(-1)


def sigma(b):
    """Moment-matched sd on the belief's own scale."""
    w, mu, sd = b
    m = (w * mu).sum(-1, keepdims=True)
    return np.sqrt((w * (sd ** 2 + (mu - m) ** 2)).sum(-1))


def quantile(b, q, iters=60):
    """Mixture quantile by bisection (vectorized)."""
    w, mu, sd = b
    lo = (mu - 8 * sd).min(-1)
    hi = (mu + 8 * sd).max(-1)
    for _ in range(iters):
        mid = (lo + hi) / 2
        cdf = (w * ndtr((mid[..., None] - mu) / sd)).sum(-1)
        lo, hi = np.where(cdf < q, mid, lo), np.where(cdf < q, hi, mid)
    return (lo + hi) / 2


def sample(b, rng):
    """Draw from the belief (used by estimate())."""
    w, mu, sd = b
    k = (rng.random(w.shape[:-1])[..., None] > np.cumsum(w, -1)).sum(-1).clip(0, 2)
    pick = lambda a: np.take_along_axis(a, k[..., None], -1)[..., 0]
    return pick(mu) + pick(sd) * rng.standard_normal(k.shape)


def copy(b):
    return tuple(x.copy() for x in b)
