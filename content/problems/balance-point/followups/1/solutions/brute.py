def two_pivots(weights: list[int]) -> int:
    # Obviously correct: every pair of pivots, the three sums added up directly.
    n = len(weights)
    count = 0
    for i in range(n):
        for j in range(i + 1, n):
            if sum(weights[:i]) == sum(weights[i + 1:j]) == sum(weights[j + 1:]):
                count += 1
    return count
