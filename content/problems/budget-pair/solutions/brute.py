def best_pair(prices: list[int], budget: int) -> int:
    # Obviously correct: every pair of different positions.
    best = -1
    for i in range(len(prices)):
        for j in range(i + 1, len(prices)):
            if prices[i] + prices[j] <= budget:
                best = max(best, prices[i] + prices[j])
    return best
