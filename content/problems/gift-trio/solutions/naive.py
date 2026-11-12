def closest_trio(prices: list[int], target: int) -> int:
    # Correct but O(n³): every trio.
    n = len(prices)
    best = None
    for i in range(n):
        for j in range(i + 1, n):
            for k in range(j + 1, n):
                s = prices[i] + prices[j] + prices[k]
                if best is None or (abs(s - target), s) < (abs(best - target), best):
                    best = s
    return best
