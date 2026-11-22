def distinct_pairs(prices: list[int], budget: int) -> int:
    # Obviously correct: every pair of positions, kept as a set of value pairs.
    n = len(prices)
    return len({(prices[i], prices[j]) for i in range(n) for j in range(i + 1, n)
                if prices[i] + prices[j] == budget})
