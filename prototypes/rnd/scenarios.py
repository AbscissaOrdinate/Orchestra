"""M2(b) scenarios B1 (truth), B2-lite (belief calibration and convergence), B3 (growth). Each returns a dict."""
import numpy as np
import belief as B
import projects as PR
import truth as T
from content_gen import make_content
from params import P, immaturity


# ------------------------------------------------------------------ B1 truth calibration (RND-004..008)

def b1_truth(n_worlds=10_000, seed=1, content_seed=1, chunk=1000):
    c = make_content(content_seed)
    nh = ~c.tech_hist
    cls = c.tech_cls
    cnt = np.zeros((4, 3))                                    # per class: ungated, soft, hard (non-historical techs)
    modes, n_en, kinds, kap = np.zeros(3), np.zeros(2), np.zeros(2), []
    hard, soft, hard_r, soft_r, edges, rep_sum = [], [], [], [], [], []
    p_edge_valid = None
    for lo in range(0, n_worlds, chunk):
        worlds = np.arange(lo, min(lo + chunk, n_worlds))
        t = T.sample_truth(c, seed, worlds)
        hard_m, soft_m = ~t.works & nh, t.gated & nh
        for k in range(4):
            m = (cls == k) & nh
            cnt[k] += [(t.works & ~t.gated)[:, m].sum(), soft_m[:, m].sum(), hard_m[:, m].sum()]
        modes += np.bincount(t.perf_mode[:, nh].ravel(), minlength=3)
        n_en += np.bincount(t.n_enablers[t.gated] - 1, minlength=2)
        kinds += [t.requires[t.gated].sum(), (t.gated & ~t.requires).sum()]
        kap.append(t.ln_kappa[:, nh].ravel())
        hard.append(hard_m.sum(1)); soft.append(soft_m.sum(1))
        hard_r.append((hard_m & t.reachable).sum(1)); soft_r.append((soft_m & t.reachable).sum(1))
        e = T.sample_edges(c, seed, worlds)
        edges.append(np.bincount(e.world, minlength=len(worlds)))
        rep_sum.append(T.sample_repeatable(seed, worlds).sum(-1).ravel())
    kap = np.concatenate(kap)
    pbar = P["beta_ab"][:, 0] / P["beta_ab"].sum(1)
    n_cls = np.array([((cls == k) & nh).sum() for k in range(4)])
    has_cand = (c.cands[c.tech_line] >= 0).any(1)
    exp_share = np.array([[pbar[k] * (1 - P["s_gate"][k]), pbar[k] * P["s_gate"][k], 1 - pbar[k]] for k in range(4)])
    exp_hard = float((nh * (1 - pbar[cls])).sum())
    exp_soft = float((nh * has_cand * pbar[cls] * P["s_gate"][cls]).sum())
    exp_share_gate = np.array([[pbar[k] * (1 - P["s_gate"][k] * (has_cand[(cls == k) & nh]).mean()),
                                pbar[k] * P["s_gate"][k] * (has_cand[(cls == k) & nh]).mean(), 1 - pbar[k]] for k in range(4)])
    return dict(content=c, n_worlds=n_worlds, n_cls=n_cls,
                share=cnt / cnt.sum(1, keepdims=True), exp_share=exp_share_gate,
                perf_modes=modes / modes.sum(), requires_share=kinds[0] / sum(kinds), two_share=n_en[1] / n_en.sum(),
                ln_kappa_mean=kap.mean(), ln_kappa_sd=kap.std(),
                hard=np.concatenate(hard), soft=np.concatenate(soft), hard_reach=np.concatenate(hard_r),
                soft_reach=np.concatenate(soft_r), exp_hard=exp_hard, exp_soft=exp_soft,
                edges=np.concatenate(edges), rep_sum=np.concatenate(rep_sum), rep_bound=P["rep_g0"] / (1 - P["rep_d"]))


# ------------------------------------------------------------------ B2-lite belief calibration (RND-012/013)

STAGES = ("prior", "sci 25%", "sci 50%", "sci 75%", "sci 100%", "after dev")


def b2_beliefs(n_worlds=200, seed=1, cross_gen="spec", content_seed=1, max_attempts=10, gate_evidence=False):
    """Every non-historical tech gets one full science phase in four steps, then (unless Requires-gated) a
    development project with integration and acceptance tests. Beliefs are updated with exact observations from
    the sampled truth. Generations run in order within each line, so the §4.2.5 cross-generation rule can act.
    cross_gen: 'none' | 'spec' (LR 3 and 1/3) | 'beta' (conjugate LRs from the class Beta).
    gate_evidence: a Blocked / FeasibilityFinding event proves the tech works (hard-dead techs are never gated)."""
    c = make_content(content_seed)
    rng = np.random.default_rng(seed)
    t = T.sample_truth(c, seed, np.arange(n_worlds))
    W, Tn = t.works.shape
    ab = P["beta_ab"][c.tech_cls]
    pbar = ab[:, 0] / ab.sum(1)
    out = {s: dict(sig_perf=np.full((W, Tn), np.nan), sig_cost=np.full((W, Tn), np.nan), err_perf=np.full((W, Tn), np.nan),
                   err_cost=np.full((W, Tn), np.nan), cov_perf=np.full((W, Tn), np.nan), cov_cost=np.full((W, Tn), np.nan),
                   p_works=np.full((W, Tn), np.nan)) for s in STAGES}
    proven = np.zeros((W, Tn), bool)
    proven[:, c.tech_hist] = True
    declared_dead = np.zeros((W, Tn), bool)
    v_true = np.where(t.works, 1.0, -1.0)
    n_tests_failed = np.zeros((W, Tn), int)
    p_after_k = np.full((5, W, Tn), np.nan)                     # p_works right after the k-th failed test
    for g in range(1, c.line_tech.shape[1] + 1):
        cols = np.flatnonzero((c.tech_gen == g) & ~c.tech_hist)
        if len(cols) == 0:
            continue
        bv = B.prior_viability(np.broadcast_to(pbar[cols], (W, len(cols))).copy())
        if g > 1 and cross_gen != "none":
            prev = c.line_tech[c.tech_line[cols], g - 2]
            ok, dead = proven[:, prev], declared_dead[:, prev]
            a, b = ab[cols].T
            lr_ok, lr_dead = (3.0, 1 / 3) if cross_gen == "spec" else ((a + 1) / a, b / (b + 1))
            bv = B.observe_binary(bv, np.where(ok, np.log(lr_ok), 0.0) + np.where(dead, np.log(lr_dead), 0.0) + 1e-12)
        bp, bk = B.prior_perf((W, len(cols))), B.prior_cost((W, len(cols)))
        tp, tk, vt = t.ln_perf[:, cols], t.ln_kappa[:, cols], v_true[:, cols]
        prior_sig = (B.sigma(bp), B.sigma(bk))

        def record(stage, bv, bp, bk):
            o = out[stage]
            o["sig_perf"][:, cols], o["sig_cost"][:, cols] = B.sigma(bp) / prior_sig[0], B.sigma(bk) / prior_sig[1]
            o["err_perf"][:, cols], o["err_cost"][:, cols] = np.abs(B.mean(bp) - tp), np.abs(B.mean(bk) - tk)
            for name, bb, tr in (("perf", bp, tp), ("cost", bk, tk)):
                o["cov_" + name][:, cols] = (B.quantile(bb, 0.1) <= tr) & (tr <= B.quantile(bb, 0.9))
            o["p_works"][:, cols] = B.p_works(bv)

        record("prior", bv, bp, bk)
        for step in range(4):                                     # science phase, quarterly pooled emission
            sd = lambda info: 1 / np.sqrt(info / 4)
            bv = B.observe(bv, vt + sd(P["info"]["viability"]) * rng.standard_normal(vt.shape), sd(P["info"]["viability"]))
            bp = B.observe(bp, tp + sd(P["info"]["perf"]) * rng.standard_normal(tp.shape), sd(P["info"]["perf"]))
            bk = B.observe(bk, tk + sd(P["info"]["cost"]) * rng.standard_normal(tk.shape), sd(P["info"]["cost"]))
            record(STAGES[step + 1], bv, bp, bk)
        if gate_evidence:
            bv = B.observe_binary(bv, np.where(t.gated[:, cols], np.log(1e4), 1e-12))
        # development: u = 2/3 (project starts at TRL 4)
        pf = P["p_fail_base"] + P["p_fail_u"] * (2 / 3)
        active = ~t.requires[:, cols]
        done = np.zeros_like(active)
        stage_int = np.ones_like(active)                          # True: next test is integration, else acceptance
        works = t.works[:, cols]
        for _ in range(max_attempts):
            live = active & ~done
            if not live.any():
                break
            u = rng.random(live.shape)
            ok = np.where(stage_int, np.where(works, u > pf, u < P["p_false_pass"]), works & (u < P["q_accept"]))
            ln_lr = np.where(stage_int, np.where(ok, np.log((1 - pf) / P["p_false_pass"]), np.log(pf / (1 - P["p_false_pass"]))),
                             np.where(ok, np.log(1e4), np.log(1 - P["q_accept"])))
            ln_lr = np.where(live, ln_lr, 1e-12)
            bv = B.observe_binary(bv, ln_lr)
            for name, prec in (("p", P["test_prec"]["perf"]), ("k", P["test_prec"]["cost"])):
                y = (tp if name == "p" else tk) + rng.standard_normal(tp.shape) / np.sqrt(prec)
                upd = B.observe(bp if name == "p" else bk, y, 1 / np.sqrt(prec))
                cur = bp if name == "p" else bk
                new = tuple(np.where(live[..., None], x, o) for x, o in zip(upd, cur))
                if name == "p":
                    bp = new
                else:
                    bk = new
            n_tests_failed[:, cols] += live & ~ok
            done = done | (live & ~stage_int & ok)
            stage_int = np.where(live, stage_int & ~ok, stage_int)         # integration pass -> acceptance next
            for k in range(5):
                first = live & ~ok & (n_tests_failed[:, cols] == k + 1) & np.isnan(p_after_k[k][:, cols])
                p_after_k[k][:, cols] = np.where(first, B.p_works(bv), p_after_k[k][:, cols])
        proven[:, cols] = done
        declared_dead[:, cols] = B.p_works(bv) < 0.02
        record("after dev", bv, bp, bk)
    out["_truth_works"] = t.works
    out["_requires"] = t.requires
    out["_failed"] = n_tests_failed
    out["_p_after_k"] = p_after_k
    out["_content"] = c
    return out


# ------------------------------------------------------------------ B3 cost and schedule overruns (RND-033, RND-038)

def b3_growth(n=10_000, seed=1, p=P):
    rng = np.random.default_rng(seed)
    mature = rng.random(n) < p["mature_share"]
    trl = np.where(mature, 6.0, rng.uniform(*p["trl_range"], n))
    u = immaturity(trl, p["u_max"])
    ln_k = p["sigma_kappa"] * rng.standard_normal(n)
    b = PR.start_belief(trl, ln_k, rng)
    e = np.exp(ln_k) / B.mean(b, log=True)                       # estimation error: what the belief gets wrong
    g, s = PR.growth(u, rng.standard_normal((n, 2)), p)
    ratio = e * g
    (c50, c80), (s50, s80) = PR.estimate(b, u)
    m = PR.phase_multipliers(g)
    cpi20, cpi_final = 1 / m[:, 0], 1 / g
    return dict(u=u, trl=trl, ratio=ratio, sched=s, e=e, g=g, belief_sd=B.sigma(b),
                cover80=(ratio > c80).mean(), cover50=(ratio > c50).mean(), sched80=(s > s80).mean(), sched50=(s > s50).mean(),
                cpi_unstable=(np.abs(cpi_final / cpi20 - 1) > 0.10).mean())


def b3_summary(r):
    u, x = r["u"], r["ratio"]
    return dict(mean_u0=x[u == 0].mean(), mean_imm=x[u > 0].mean(), mean_all=x.mean(), cv_all=x.std() / x.mean(),
                p50=np.quantile(x, .5), p85=np.quantile(x, .85), p95=np.quantile(x, .95),
                sched_mean=r["sched"].mean(), sched_cv=r["sched"].std() / r["sched"].mean(), corr=np.corrcoef(np.log(x), np.log(r["sched"]))[0, 1],
                exceed_p80=r["cover80"], exceed_p50=r["cover50"], sched_exceed_p80=r["sched80"], cpi_unstable=r["cpi_unstable"])


def b2_summary(o):
    nh = ~o["_content"].tech_hist
    works = np.broadcast_to(o["_truth_works"], o["prior"]["p_works"].shape)
    res = dict(stage={}, reliability={})
    for st in STAGES:
        d = o[st]
        res["stage"][st] = dict(sig_perf=np.nanmedian(d["sig_perf"]), sig_cost=np.nanmedian(d["sig_cost"]),
                                err_perf=np.nanmedian(d["err_perf"][:, nh]), err_cost=np.nanmedian(d["err_cost"][:, nh]),
                                cov_perf=np.nanmean(d["cov_perf"][:, nh]), cov_cost=np.nanmean(d["cov_cost"][:, nh]),
                                n=int(np.isfinite(d["cov_perf"][:, nh]).sum()))
    for st in ("prior", "after dev"):                           # reliability of p_works by decile
        p = o[st]["p_works"][:, nh].ravel()
        w = works[:, nh].ravel()
        edges = np.quantile(p, np.linspace(0, 1, 11))
        idx = np.clip(np.searchsorted(edges, p, side="right") - 1, 0, 9)
        res["reliability"][st] = np.array([[p[idx == k].mean(), w[idx == k].mean(), (idx == k).sum()] for k in range(10) if (idx == k).any()])
    dead = ~works & ~o["_requires"] & nh[None]
    cls = np.broadcast_to(o["_content"].tech_cls, works.shape)
    res["dead_below_010"] = np.array([[np.nanmean(o["_p_after_k"][k][dead & (cls == c)] < 0.10) for c in range(4)] for k in range(5)])
    res["dead_below_010_all"] = np.array([(o["_p_after_k"][k][dead][np.isfinite(o["_p_after_k"][k][dead])] < 0.10).mean() for k in range(5)])
    return res
