def best_pair(prices: list[int], budget: int) -> int:
    lo, hi = 0, len(prices) - 1
    best = -1
    while lo < hi:
        total = prices[lo] + prices[hi]
        if total > budget:
            hi -= 1
        else:
            best = max(best, total)
            lo += 1
    return best
