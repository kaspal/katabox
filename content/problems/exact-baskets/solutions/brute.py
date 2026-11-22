from itertools import combinations


def exact_baskets(weights: list[int], k: int, target: int) -> list[list[int]]:
    # Obviously correct: every k-subset of the sorted weights, in lexicographic order.
    return [list(c) for c in combinations(sorted(weights), k) if sum(c) == target]
