from itertools import combinations


def closest_trio(prices: list[int], target: int) -> int:
    # Obviously correct: every trio, ranked by distance, then by the smaller sum.
    return min((sum(t) for t in combinations(prices, 3)), key=lambda s: (abs(s - target), s))
