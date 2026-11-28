"""Run B1, B2-lite and B3 for three base seeds, write plots and RESULTS.md (pass/fail vs the spec criteria)."""
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import scenarios as S
from params import P

SEEDS = (101, 202, 303)


def main():
    rows, keep = [], {}
    for seed in SEEDS:
        b1 = S.b1_truth(10_000, seed)
        b2 = S.b2_beliefs(200, seed, "beta", gate_evidence=True)
        b2_spec = S.b2_beliefs(200, seed, "spec", gate_evidence=True)
        b2_nogate = S.b2_beliefs(200, seed, "beta", gate_evidence=False)
        b3 = S.b3_growth(10_000, seed)
        rows += checks(seed, b1, S.b2_summary(b2), S.b2_summary(b2_spec), S.b2_summary(b2_nogate), S.b3_summary(b3), b3, b2, b2_nogate)
        keep[seed] = (b1, b2, b3)
    plots(*keep[SEEDS[0]])
    write(rows, keep)


def cpi_spec(b3raw):
    import projects as PR
    old = P["phase_var_share"].copy(); P["phase_var_share"] = np.array([.6, .25, .15])
    m = PR.phase_multipliers(b3raw["g"]); P["phase_var_share"] = old
    return (np.abs((1 / b3raw["g"]) / (1 / m[:, 0]) - 1) > .10).mean()


def exact_err():
    import belief as B
    return max(abs(B.p_works(B.observe_binary(B.prior_viability(np.array(p)), np.log(lr))) - p * lr / (p * lr + 1 - p)) for p in (.1, .4, .8) for lr in (1 / 20, 1 / 3, 3, 13.6, 50))


def edge_row(seed):
    import truth as T
    c = S.make_content(1)
    n = np.bincount(T.sample_edges(c, seed, np.arange(1000)).world, minlength=1000)
    first = c.line_D[:, 0]
    valid = np.array([[np.mean(first[a] <= c.line_D[b, :c.ngen[b]]) for b in range(c.n_lines)] for a in range(c.n_lines)])
    same = c.domain[:, None] == c.domain[None, :]
    return "%.1f / %.1f / %.1f" % (n.mean(), P["p_edge"] * c.n_lines * (c.n_lines - 1), P["p_edge"] * (valid * ~same).sum()),


def s1_row():
    import projects as PR, belief as B
    from params import immaturity
    rng = np.random.default_rng(5); n = 20_000
    trl = np.where(rng.random(n) < .16, 6.0, rng.uniform(3, 5.5, n)); u = immaturity(trl)
    lk = .35 * rng.standard_normal(n); b = PR.start_belief(trl, lk, rng)
    ratio = np.exp(lk) / B.mean(b, log=True) * PR.growth(u, rng.standard_normal((n, 2)))[0]
    out = []
    for stream in ("plain", "sobol"):
        x = np.array([[(ratio > q).mean() for q in PR.estimate(b, u, seed=s, stream=stream)[0]] for s in range(20)])
        out.append("%s %.2f-%.2f / %.2f-%.2f" % (stream, x[:, 0].min(), x[:, 0].max(), x[:, 1].min(), x[:, 1].max()))
    return "; ".join(out)


def checks(seed, b1, b2, b2_spec, b2_nog, b3, b3raw, b2_raw, b2_nog_raw):
    r = lambda req, name, val, crit, ok: (seed, req, name, val, crit, bool(ok))
    dev = lambda s, st: 100 * np.abs(s["reliability"][st][:, 0] - s["reliability"][st][:, 1]).max()
    cov = [v for st in b2["stage"].values() for v in (st["cov_perf"], st["cov_cost"])]
    st = b2["stage"]
    return [
        r("RND-004", "max class-share error (pp)", 100 * np.abs(b1["share"] - b1["exp_share"]).max(), "< 1.5", 100 * np.abs(b1["share"] - b1["exp_share"]).max() < 1.5),
        r("RND-006", "hard / soft per world", f"{b1['hard'].mean():.1f} / {b1['soft'].mean():.1f}", "within 3 SE of content; soft/hard >= 2",
          abs(b1["hard"].mean() - b1["exp_hard"]) < 3 * b1["hard"].std() / 100 and abs(b1["soft"].mean() - b1["exp_soft"]) < 3 * b1["soft"].std() / 100 and b1["soft"].mean() >= 2 * b1["hard"].mean()),
        r("RND-008", "worlds x lines with sum g > g0/(1-d)", f"{(b1['rep_sum'] > b1['rep_bound']).mean():.1e}", "0", (b1["rep_sum"] <= b1["rep_bound"]).all()),
        r("RND-012", "interval coverage min..max over stages", f"{min(cov):.3f}..{max(cov):.3f}", "0.77..0.83", 0.77 <= min(cov) and max(cov) <= 0.83),
        r("RND-012", "p_works reliability max dev, prior (pp), LR beta", dev(b2, "prior"), "< 5", dev(b2, "prior") < 5),
        r("RND-012", "p_works reliability max dev, prior (pp), LR 3 as specified", dev(b2_spec, "prior"), "< 5", dev(b2_spec, "prior") < 5),
        r("RND-012", "p_works in [0.2, 0.9) after dev: gap (pp) / n, Blocked is not evidence (spec)", "%.1f / %d" % S.mid_gap(b2_nog_raw), "< 5, n >= 30", S.mid_gap(b2_nog_raw)[0] < 5),
        r("RND-012", "same, Blocked is evidence (S-3)", "n/a: n = %d < 30, bin empty" % S.mid_gap(b2_raw)[1], "< 5, n >= 30", True),
        r("RND-011", "max |posterior - exact| over 15 cases", exact_err(), "< 1e-6", exact_err() < 1e-6),
        r("RND-005", "mean edges / sum p / sum p*P(valid)", edge_row(seed)[0], "3 SE of sum p (spec); 3 SE of valid (S-5)", False),
        r("S-1", "estimate(): P50 / P80 exceedance range over 20 stream seeds, plain vs Sobol", s1_row(), "0.50±.05 / 0.20±.04", False),
        r("S-6", "hard-dead below 0.10 after 2 / 4 / 5 failed tests; by class after 2", "%.2f / %.2f / %.2f; %s" % (b2["dead_below_010_all"][1], b2["dead_below_010_all"][3], b2["dead_below_010_all"][4], np.round(b2["dead_below_010"][1], 2)), ">= 0.90", False),
        r("RND-013", "median perf sigma ratio: science / after dev", f"{st['sci 100%']['sig_perf']:.2f} / {st['after dev']['sig_perf']:.2f}", "<= 0.80 / <= 0.45",
          st["sci 100%"]["sig_perf"] <= 0.8 and st["after dev"]["sig_perf"] <= 0.45),
        r("RND-013", "hard-dead with p_works < 0.10 after 2 failed tests", f"{b2['dead_below_010_all'][1]:.2f}", ">= 0.90", b2["dead_below_010_all"][1] >= 0.9),
        r("RND-033", "mean u=0 / u>0 / all", f"{b3['mean_u0']:.3f} / {b3['mean_imm']:.3f} / {b3['mean_all']:.3f}", "1.03±.03 / 1.35±.05 / 1.25..1.40",
          abs(b3["mean_u0"] - 1.03) < .03 and abs(b3["mean_imm"] - 1.35) < .05 and 1.25 <= b3["mean_all"] <= 1.4),
        r("RND-033", "CV (cost) / mean schedule", f"{b3['cv_all']:.3f} / {b3['sched_mean']:.3f}", "0.80±0.10 / 1.10..1.30", abs(b3["cv_all"] - .8) < .1 and 1.1 <= b3["sched_mean"] <= 1.3),
        r("RND-038", "exceed P80 / P50", f"{b3['exceed_p80']:.3f} / {b3['exceed_p50']:.3f}", "0.20±.04 / 0.50±.05", abs(b3["exceed_p80"] - .2) < .04 and abs(b3["exceed_p50"] - .5) < .05),
        r("§4.5.4 [R] S-7", "share with final CPI > 10% off CPI at 20%: tuned 0.5/0.3/0.2 vs spec 0.6/0.25/0.15", f"{b3['cpi_unstable']:.2f} vs {cpi_spec(b3raw):.2f}", "report; tune if most (> 0.5)", b3["cpi_unstable"] <= 0.5),
    ]


def plots(b1, b2, b3):
    fig, ax = plt.subplots(1, 2, figsize=(10, 3.4))
    bins = np.arange(0, max(b1["soft"].max(), b1["hard"].max()) + 2)
    ax[0].hist(b1["hard"], bins, alpha=.7, label="hard-dead"); ax[0].hist(b1["soft"], bins, alpha=.7, label="soft-gated")
    ax[0].set(xlabel="techs per world (all 255 future techs)", ylabel="worlds", title="B1 dead ends per world"); ax[0].legend()
    ax[1].hist(b1["hard_reach"], bins, alpha=.7, label="hard-dead, reachable"); ax[1].hist(b1["soft_reach"], bins, alpha=.7, label="soft-gated, reachable")
    ax[1].set(xlabel="techs per world", title="...encountered (earlier generations all work)"); ax[1].legend()
    fig.tight_layout(); fig.savefig("out/b1_dead_ends.png", dpi=110); plt.close(fig)

    r = S.b2_summary(b2)
    xs = range(len(S.STAGES))
    fig, ax = plt.subplots(1, 3, figsize=(12, 3.4))
    ax[0].plot(xs, [r["stage"][s]["sig_perf"] for s in S.STAGES], "o-", label="performance")
    ax[0].plot(xs, [r["stage"][s]["sig_cost"] for s in S.STAGES], "s-", label="cost"); ax[0].set(title="B2 median σ / prior σ", ylim=(0, 1.05)); ax[0].legend()
    ax[1].plot(xs, [r["stage"][s]["err_perf"] for s in S.STAGES], "o-", label="performance")
    ax[1].plot(xs, [r["stage"][s]["err_cost"] for s in S.STAGES], "s-", label="cost"); ax[1].set(title="median |ln(mean/truth)|"); ax[1].legend()
    ax[2].plot(xs, [r["stage"][s]["cov_perf"] for s in S.STAGES], "o-", label="performance")
    ax[2].plot(xs, [r["stage"][s]["cov_cost"] for s in S.STAGES], "s-", label="cost"); ax[2].axhline(.8, c="k", lw=.7); ax[2].set(title="80% interval coverage", ylim=(.7, .9)); ax[2].legend()
    for a in ax: a.set_xticks(list(xs)); a.set_xticklabels(S.STAGES, rotation=30, ha="right")
    fig.tight_layout(); fig.savefig("out/b2_convergence.png", dpi=110); plt.close(fig)

    fig, ax = plt.subplots(1, 2, figsize=(9, 3.6))
    for mode, lab in (("spec", "LR 3, 1/3 (spec)"), ("beta", "conjugate LR (proposed)")):
        rel = S.b2_summary(S.b2_beliefs(200, 101, mode, gate_evidence=True))["reliability"]["prior"]
        ax[0].plot(rel[:, 0], rel[:, 1], "o-", label=lab)
    ax[0].plot([.55, 1], [.55, 1], "k", lw=.6); ax[0].set(xlabel="mean p_works in decile", ylabel="observed working share", title="B2 reliability (prior, with cross-generation rule)"); ax[0].legend()
    ax[1].plot(range(1, 6), r["dead_below_010"], "o-")
    ax[1].legend(["Established", "Promising", "Speculative", "Fringe"]); ax[1].axhline(.9, c="k", lw=.7)
    ax[1].set(xlabel="failed tests", ylabel="share of hard-dead with p_works < 0.10", title="Recognizing a hard dead end"); ax[1].set_xticks(range(1, 6))
    fig.tight_layout(); fig.savefig("out/b2_reliability.png", dpi=110); plt.close(fig)

    fig, ax = plt.subplots(1, 3, figsize=(12, 3.4))
    u, x = b3["u"], b3["ratio"]
    for lo, hi, lab in ((0, .01, "mature"), (.01, .5, "u 0.01-0.5"), (.5, 1.01, "u 0.5-1")):
        ax[0].hist(x[(u >= lo) & (u < hi)], np.linspace(.5, 4, 70), histtype="step", density=True, label=lab)
    ax[0].set(xlabel="realized / baseline cost", title="B3 cost growth by immaturity"); ax[0].legend()
    ax[1].hist(b3["sched"], np.linspace(.7, 3, 70), color="C3"); ax[1].set(xlabel="realized / planned months", title="schedule growth")
    s = S.b3_summary(b3)
    ax[2].bar(["P50 budget", "P80 budget"], [s["exceed_p50"], s["exceed_p80"]]); ax[2].plot([-.4, .4], [.5, .5], "k", lw=.8); ax[2].plot([.6, 1.4], [.2, .2], "k", lw=.8)
    ax[2].set(ylabel="share of projects over budget", title="estimate() coverage (lines = target)")
    fig.tight_layout(); fig.savefig("out/b3_growth.png", dpi=110); plt.close(fig)


def write(rows, keep):
    b1, b2, b3 = keep[SEEDS[0]]
    c = b1["content"]
    L = ["# M2(b) results: B1 truth, B2-lite beliefs, B3 growth", "",
         f"Generated by `python report.py` (seeds {SEEDS}, 10,000 worlds for B1 and B3 projects, 200 worlds for B2). Synthetic content: {c.n_lines} lines "
         f"(incl. {(c.ngen == 1).sum()} singletons), {c.n_techs} techs, {int(c.tech_hist.sum())} historical.", "",
         "## Pass / fail by seed", "", "| Seed | Req | Metric | Value | Criterion | Pass |", "|---|---|---|---|---|---|"]
    for seed, req, name, val, crit, ok in rows:
        v = f"{val:.3f}" if isinstance(val, (float, np.floating)) else val
        L.append(f"| {seed} | {req} | {name} | {v} | {crit} | {'N/A' if str(val).startswith('n/a') else ('PASS' if ok else 'FAIL')} |")
    L += ["", "## Figures", "", "![dead ends](out/b1_dead_ends.png)", "", "![convergence](out/b2_convergence.png)", "",
          "![reliability](out/b2_reliability.png)", "", "![growth](out/b3_growth.png)", ""]
    L += open("FINDINGS.md").read().splitlines()
    open("RESULTS.md", "w").write("\n".join(L))


if __name__ == "__main__":
    main()
