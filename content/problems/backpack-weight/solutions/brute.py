from itertools import combinations


def best_pack(weights: list[int], values: list[int], capacity: int) -> int:
    # Obviously correct (small n): every subset.
    n, best = len(weights), 0
    for k in range(n + 1):
        for subset in combinations(range(n), k):
            if sum(weights[i] for i in subset) <= capacity:
                best = max(best, sum(values[i] for i in subset))
    return best
