"""Every [D] constant from WS-05 spec §4 in one place. Scenarios copy and override keys."""
import numpy as np

CLASSES = ("Established", "Promising", "Speculative", "Fringe")

P = dict(
    # §4.1.2 class table: Beta(alpha, beta) for "not hard-dead", gate probability s_c
    beta_ab=np.array([[99, 1], [23, 2], [8, 2], [3, 2]], float),
    s_gate=np.array([0.06, 0.29, 0.56, 0.83]),
    # performance mixture (disappointing, nominal, breakthrough): weights, medians, log-sd
    perf_w=np.array([0.15, 0.75, 0.10]),
    perf_median=np.array([0.75, 1.0, 1.3]),
    perf_sigma=0.10,
    sigma_kappa=0.35,
    # §4.1.3 soft gates
    p_two_enablers=0.3,
    p_requires=0.4,
    penalty_cost_range=(3.0, 10.0),      # LogUniform
    penalty_perf_range=(0.5, 0.8),       # Uniform
    enabler_offset=(-5.0, 25.0),         # years, Uniform
    # §4.1.2 Boost edges
    p_edge=0.002,
    edge_b=(0.15, 0.40),
    # §4.1.4 repeatables
    rep_g0=0.03, rep_d=0.95, rep_cv=0.3, rep_levels=30, rep_step_dev=1.35,
    # §4.2 beliefs
    sigma_c=1e-3,                        # viability component width
    info=dict(viability=1.2, perf=30.0, cost=15.0),   # per full science phase (§4.3)
    test_prec=dict(perf=100.0, cost=50.0),
    cost_obs_on_proof=0.05,
    # §4.5.3 growth
    g_mean=(1.03, 0.54), g_cv=(0.20, 0.80), s_mean=(1.04, 0.30), s_cv=(0.15, 0.30), rho=0.6,
    u_max=5 / 3, mature_share=0.16, trl_range=(3.0, 5.5),
    phase_w=np.array([0.4, 0.3, 0.3]),
    phase_var_share=np.array([0.5, 0.30, 0.20]),   # tuned M2(b): 0.6/0.25/0.15 left 66% of projects CPI-unstable
    # §4.5.5 tests
    p_fail_base=0.15, p_fail_u=0.25, p_false_pass=0.05, q_accept=0.9,
    # §4.5.7 estimate()
    n_estimate=256,
    # synthetic content (§6.3)
    start_year=1955, end_year=2055,
    domain_nodes=(20, 28, 24, 26, 20, 30, 28, 32, 18, 26, 24, 24),
    class_mix=(128, 76, 36, 15), n_historical=45, n_singletons=20, n_repeatable=12,
)


def immaturity(trl_min, u_max=P["u_max"]):
    """u = clamp((6 - TRL_min) / 3, 0, u_max)  (§4.5.3)."""
    return np.clip((6.0 - np.asarray(trl_min, float)) / 3.0, 0.0, u_max)
