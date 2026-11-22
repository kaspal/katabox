from itertools import product


def best_pack_huge(weights: list[int], values: list[int], capacity: int) -> int:
    # Obviously correct for a few items: every subset.
    best = 0
    for pick in product((0, 1), repeat=len(weights)):
        if sum(w for w, p in zip(weights, pick) if p) <= capacity:
            best = max(best, sum(v for v, p in zip(values, pick) if p))
    return best
