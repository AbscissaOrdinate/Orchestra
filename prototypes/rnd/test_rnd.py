"""M2(b) acceptance tests, named after the WS-05 requirement they check (spec v0.3 §5)."""
import numpy as np
import pytest
import belief as B
import projects as PR
import scenarios as S
import truth as T
from content_gen import make_content
from params import P, immaturity, rep_bound


@pytest.fixture(scope="session")
def b1():
    return S.b1_truth(n_worlds=4000, seed=11)


@pytest.fixture(scope="session")
def b2():
    """Spec v0.3: per-line Beta counts (§4.2.5) and gate events as viability evidence (§0)."""
    o = S.b2_beliefs(200, seed=12, cross_gen="counts", gate_evidence=True)
    return o, S.b2_summary(o)


@pytest.fixture(scope="session")
def b3():
    return S.b3_growth(10_000, seed=13)


def rnd_004_truth_follows_priors(b1):
    assert np.abs(b1["share"] - b1["exp_share"]).max() < 0.015
    assert abs(b1["requires_share"] - P["p_requires"]) < 0.015
    assert abs(b1["two_share"] - P["p_two_enablers"]) < 0.015
    assert np.abs(b1["perf_modes"] - P["perf_w"]).max() < 0.015
    assert abs(b1["ln_kappa_mean"]) < 0.01 and abs(b1["ln_kappa_sd"] - P["sigma_kappa"]) < 0.01


def rnd_005_boost_edges():
    c = make_content(1)
    worlds = np.arange(1000)
    e = T.sample_edges(c, 5, worlds)
    counts = np.bincount(e.world, minlength=len(worlds))
    same = c.domain[:, None] == c.domain[None, :]
    first_d = c.line_D[:, 0]
    valid = np.array([[np.mean(first_d[a] <= c.line_D[b, :c.ngen[b]]) for b in range(c.n_lines)] for a in range(c.n_lines)])
    expected = P["p_edge"] * (valid * ~same).sum()
    assert abs(counts.mean() - expected) < 3 * counts.std() / np.sqrt(len(worlds))
    assert (c.tech_D[e.src] <= c.tech_D[e.dst]).all()
    ids = list(c.line_id)
    h_small, h_big = T.pair_hashes(ids[:10]), T.pair_hashes(ids + ["line99"])
    assert (h_small == h_big[:10, :10]).all()            # a pair's draws never depend on which other lines exist


def rnd_006_dead_end_frequency(b1):
    for key, exp in (("hard", b1["exp_hard"]), ("soft", b1["exp_soft"])):
        x = b1[key]
        assert abs(x.mean() - exp) < 3 * x.std() / np.sqrt(len(x))
    assert b1["soft"].mean() / b1["hard"].mean() >= 2


def rnd_007_gates_from_candidates_only():
    c, seed = make_content(1), 3
    t = T.sample_truth(c, seed, np.arange(300))
    line_of = np.where(t.enabler >= 0, c.tech_line[np.maximum(t.enabler, 0)], -1)
    allowed = np.broadcast_to(c.cands[c.tech_line][None, :, None, :], line_of.shape + (c.cands.shape[1],))
    assert ((line_of[..., None] == allowed).any(-1) | (line_of < 0)).all()
    sub = T.sample_truth(c, seed, np.array([5, 6]))       # world-keyed: a world's truth is independent of the batch
    assert (sub.gated == t.gated[[5, 6]]).all() and (sub.enabler == t.enabler[[5, 6]]).all()
    cand_rank = c.rank[np.where(c.cands >= 0, c.cands, 0)]
    assert ((c.cands < 0) | (cand_rank < c.rank[:, None])).all()      # candidates only from lower rank: no cycles


def rnd_008_repeatable_gain_bound():
    g = T.sample_repeatable(5, np.arange(100_000))
    assert g.sum(-1).max() <= rep_bound()
    assert abs((g / (P["rep_g0"] * P["rep_d"] ** np.arange(P["rep_levels"]))).mean() - 0.999) < 0.002   # clip leaves the mean ~1


def rnd_010_priors_equal_sampling_distributions():
    p = np.array([0.99, 0.92])
    w, mu, sd = B.prior_viability(p)
    assert np.allclose(w[:, :2], np.stack([p, 1 - p], -1), atol=1e-12) and (mu[:, :2] == [1, -1]).all() and (sd[:, :2] == P["sigma_c"]).all()
    w, mu, sd = B.prior_perf()
    assert np.allclose(w, P["perf_w"], atol=1e-12) and np.allclose(mu, np.log(P["perf_median"]), atol=1e-12) and (sd == P["perf_sigma"]).all()
    w, mu, sd = B.prior_cost()
    assert w[0] == 1 and mu[0] == 0 and abs(sd[0] - P["sigma_kappa"]) < 1e-12


@pytest.mark.parametrize("p", (0.1, 0.4, 0.8))
@pytest.mark.parametrize("lr", (1 / 20, 1 / 3, 3, 13.6, 50))
def rnd_011_binary_evidence_is_exact_bayes(p, lr):
    post = B.p_works(B.observe_binary(B.prior_viability(np.array(p)), np.log(lr)))
    assert abs(post - p * lr / (p * lr + 1 - p)) < 1e-6


def rnd_012_interval_coverage(b2):
    _, r = b2
    for st, d in r["stage"].items():
        for k in ("cov_perf", "cov_cost"):
            assert abs(d[k] - 0.80) < 0.03, (st, k)


def rnd_012_p_works_reliability(b2):
    _, r = b2
    for st in ("prior", "after dev"):
        rel = r["reliability"][st]
        assert np.abs(rel[:, 0] - rel[:, 1]).max() < 0.05, st


def rnd_016_counts_prior_and_late_update():
    a, b = 23.0, 2.0                                              # Promising
    p_full = (a + 1) / (a + b + 2)                                # g1 proven, g2 dead -> 24/27
    assert abs(p_full - 24 / 27) < 1e-12
    p_old = a / (a + b + 1)                                       # g3 known with only g2 counted: 23/26
    odds = lambda p: p / (1 - p)
    bv = B.observe_binary(B.prior_viability(np.array(p_old)), np.log(odds(p_full) / odds(p_old)))
    assert abs(B.p_works(bv) - 24 / 27) < 1e-12 and abs(odds(p_full) / odds(p_old) - 24 / 23) < 1e-12


def rnd_023_gate_event_proves_viability():
    for p in (0.08, 0.6, 0.92):
        post = B.p_works(B.observe_binary(B.prior_viability(np.array(p)), np.log(1e4)))
        assert abs(post - p * 1e4 / (p * 1e4 + 1 - p)) < 1e-9
    assert B.p_works(B.observe_binary(B.prior_viability(np.array(0.6)), np.log(1e4))) > 0.9999


def rnd_012_gate_evidence_closes_mid_bin_gap():
    gap_on, n_on = S.mid_gap(S.b2_beliefs(200, seed=12))
    gap_off, n_off = S.mid_gap(S.b2_beliefs(200, seed=12, gate_evidence=False))
    assert gap_off > 20 and n_off >= 30                            # v0.2 behaviour: techs that work stuck at p ~ 0.72
    assert n_on < 30 or gap_on < 5                                 # v0.3: the bin empties or is calibrated
def rnd_013_convergence(b2):
    _, r = b2
    st = r["stage"]
    assert st["sci 100%"]["sig_perf"] <= 0.80 and st["after dev"]["sig_perf"] <= 0.45
    errs = [st[s]["err_perf"] for s in S.STAGES]
    assert all(a >= b - 1e-3 for a, b in zip(errs, errs[1:]))
    errs = [st[s]["err_cost"] for s in S.STAGES]
    assert all(a >= b - 1e-3 for a, b in zip(errs, errs[1:]))


def rnd_013_dead_end_recognition_design_intent(b2):
    r = b2[1]
    assert r["dead_below_010_all"][3] >= 0.80                     # after 4 failed tests
    assert r["dead_below_010"][1][3] >= 0.70                      # Fringe after 2


def rnd_033_growth_calibrated(b3):
    s = S.b3_summary(b3)
    assert abs(s["mean_u0"] - 1.03) < 0.03 and abs(s["mean_imm"] - 1.35) < 0.05
    assert abs(s["cv_all"] - 0.80) < 0.10 and 1.25 <= s["mean_all"] <= 1.40
    assert 1.10 <= s["sched_mean"] <= 1.30


def rnd_038_estimate_calibrated(b3):
    s = S.b3_summary(b3)
    assert abs(s["exceed_p80"] - 0.20) < 0.04 and abs(s["exceed_p50"] - 0.50) < 0.05
    u, trl = b3["u"][:50], b3["trl"][:50]
    rng = np.random.default_rng(0)
    b = PR.start_belief(trl, 0.35 * rng.standard_normal(50), rng)
    a, c = PR.estimate(b, u), PR.estimate(b, u)
    assert all((x == y).all() for x, y in zip(a[0] + a[1], c[0] + c[1]))


@pytest.mark.parametrize("stream", ("committed", "sobol"))
def rnd_038_estimate_s1_candidates_pass_20_seeds(stream):
    """§4.5.7: the committed point set, and 20 alternative scrambles (robustness to regenerating it)."""
    n = 10_000
    rng = np.random.default_rng(5)
    trl = np.where(rng.random(n) < 0.16, 6.0, rng.uniform(3, 5.5, n))
    u = immaturity(trl)
    lk = 0.35 * rng.standard_normal(n)
    b = PR.start_belief(trl, lk, rng)
    ratio = np.exp(lk) / B.mean(b, log=True) * PR.growth(u, rng.standard_normal((n, 2)))[0]
    for seed in range(20):
        (c50, c80), _ = PR.estimate(b, u, seed=seed, stream=stream)
        assert abs((ratio > c50).mean() - 0.5) < 0.05 and abs((ratio > c80).mean() - 0.2) < 0.04
