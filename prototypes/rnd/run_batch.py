"""Run one scenario and save its summary: python run_batch.py B1|B2|B3 [--worlds N] [--seed S]"""
import argparse
import pickle
import pathlib
import scenarios as S

RUN = dict(B1=lambda a: S.b1_truth(a.worlds or 10_000, a.seed),
           B2=lambda a: S.b2_beliefs(a.worlds or 200, a.seed, a.cross_gen, gate_evidence=a.gate_evidence),
           B3=lambda a: S.b3_growth(a.worlds or 10_000, a.seed))

if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("scenario", choices=RUN)
    ap.add_argument("--worlds", type=int, default=0)
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--cross-gen", default="beta", choices=("none", "spec", "beta"))
    ap.add_argument("--gate-evidence", action="store_true")
    a = ap.parse_args()
    out = pathlib.Path("out"); out.mkdir(exist_ok=True)
    res = RUN[a.scenario](a)
    pickle.dump(res, open(out / f"{a.scenario}_s{a.seed}.pkl", "wb"))
    print("saved", out / f"{a.scenario}_s{a.seed}.pkl")
