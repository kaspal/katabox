def longest_stretch(costs: list[int], budget: int) -> int:
    # Obviously correct: every stretch.
    best = 0
    n = len(costs)
    for i in range(n):
        for j in range(i, n):
            if sum(costs[i:j + 1]) <= budget:
                best = max(best, j - i + 1)
    return best
