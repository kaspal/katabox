def longest_with_refunds(costs: list[int], budget: int) -> int:
    # Obviously correct: every stretch.
    best = 0
    for i in range(len(costs)):
        total = 0
        for j in range(i, len(costs)):
            total += costs[j]
            if total <= budget:
                best = max(best, j - i + 1)
    return best
