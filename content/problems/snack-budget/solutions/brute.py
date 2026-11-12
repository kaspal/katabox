from itertools import combinations


def most_snacks(prices: list[int], budget: int) -> int:
    # Obviously correct: the largest k for which some k snacks fit.
    for k in range(len(prices), 0, -1):
        if any(sum(c) <= budget for c in combinations(prices, k)):
            return k
    return 0
