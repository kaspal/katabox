def longest_stretch(costs: list[int], budget: int) -> int:
    # Correct but O(n²) when the budget is large: every start is extended to the end.
    best = 0
    n = len(costs)
    for i in range(n):
        total = 0
        for j in range(i, n):
            total += costs[j]
            if total > budget:
                break
            if j - i + 1 > best:
                best = j - i + 1
    return best
