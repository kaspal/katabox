from itertools import combinations


def longest_rise(heights: list[int]) -> int:
    # Obviously correct (small inputs only): try every subset, longest first.
    n = len(heights)
    for k in range(n, 0, -1):
        for picked in combinations(heights, k):
            if all(a < b for a, b in zip(picked, picked[1:])):
                return k
    return 0
