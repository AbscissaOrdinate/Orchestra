"""RND-028 fixture: the factored basis update (spec §4.2.8) equals exact enumeration.

Run: python basis_check.py   (prints exact and factored p_works; asserts they agree to 1e-9)
"""
import itertools
import numpy as np

P_BASIS, P_OWN = 0.2, np.array([0.8, 0.6])
P_PASS_WORKS, LR_PASS = 0.9, 13.6
EVIDENCE = [(0, "pass"), (0, "fail"), (0, "pass")]   # (member, outcome)


def main():
    exact = exact_posterior()
    factored = factored_posterior()
    print("exact   ", np.round(exact, 4))
    print("factored", np.round(factored, 4))
    assert np.allclose(exact, factored, atol=1e-9)


# ---------------------------------------------------------------- helpers

def likelihood(works, outcome):
    p_pass = P_PASS_WORKS if works else P_PASS_WORKS / LR_PASS
    return p_pass if outcome == "pass" else 1 - p_pass


def exact_posterior():
    """Enumerate (basis, own_1, own_2); a member works iff basis and own."""
    joint = {}
    for b, *own in itertools.product([0, 1], repeat=1 + len(P_OWN)):
        p = (P_BASIS if b else 1 - P_BASIS) * np.prod([q if o else 1 - q for q, o in zip(P_OWN, own)])
        for j, outcome in EVIDENCE:
            p *= likelihood(b and own[j], outcome)
        joint[(b, *own)] = p
    total = sum(joint.values())
    return np.array([sum(v for k, v in joint.items() if k[0] and k[1 + j]) / total for j in range(len(P_OWN))])


def factored_posterior():
    """§4.2.8: L_j += ln LR; L_B += ln(p_j LR + 1 - p_j); p_works = σ(L_B) σ(L_j)."""
    logit = lambda p: np.log(p / (1 - p))
    sigmoid = lambda x: 1 / (1 + np.exp(-x))
    l_basis, l_own = logit(P_BASIS), logit(P_OWN)
    for j, outcome in EVIDENCE:
        lr = likelihood(1, outcome) / likelihood(0, outcome)
        p_j = sigmoid(l_own[j])
        l_basis += np.log(p_j * lr + 1 - p_j)
        l_own[j] += np.log(lr)
    return sigmoid(l_basis) * sigmoid(l_own)


if __name__ == "__main__":
    main()
