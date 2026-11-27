"""M2(b) acceptance tests, named after the WS-05 requirement they check (spec §5). Findings that currently
contradict the spec are xfail(strict) with the proposal ID from RESULTS.md, so a fix turns them green."""
import numpy as np
import pytest
import belief as B
import projects as PR
import scenarios as S
import truth as T
from content_gen import make_content
from params import P


@pytest.fixture(scope="session")
def b1():
    return S.b1_truth(n_worlds=4000, seed=11)


@pytest.fixture(scope="session")
def b2():
    o = S.b2_beliefs(200, seed=12, cross_gen="beta", gate_evidence=True)
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


@pytest.mark.xfail(strict=True, reason="S-4: lognormal multipliers can exceed g0/(1-d); clip them (RESULTS.md)")
def rnd_008_repeatable_gain_bound():
    s = T.sample_repeatable(5, np.arange(20_000)).sum(-1)
    assert s.max() <= P["rep_g0"] / (1 - P["rep_d"])


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


@pytest.mark.xfail(strict=True, reason="S-2: fixed LR 3 and 1/3 overstate cross-generation evidence")
def rnd_012_cross_generation_lr_as_specified():
    o = S.b2_beliefs(200, seed=12, cross_gen="spec", gate_evidence=True)
    rel = S.b2_summary(o)["reliability"]["prior"]
    assert np.abs(rel[:, 0] - rel[:, 1]).max() < 0.05


def rnd_013_convergence(b2):
    _, r = b2
    st = r["stage"]
    assert st["sci 100%"]["sig_perf"] <= 0.80 and st["after dev"]["sig_perf"] <= 0.45
    errs = [st[s]["err_perf"] for s in S.STAGES]
    assert all(a >= b - 1e-3 for a, b in zip(errs, errs[1:]))
    errs = [st[s]["err_cost"] for s in S.STAGES]
    assert all(a >= b - 1e-3 for a, b in zip(errs, errs[1:]))


@pytest.mark.xfail(strict=True, reason="S-6: two failed tests expose only ~61% of hard dead ends")
def rnd_013_dead_end_after_two_failed_tests(b2):
    assert b2[1]["dead_below_010_all"][1] >= 0.90


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
