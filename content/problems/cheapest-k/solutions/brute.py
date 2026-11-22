def cheapest_k(prices: list[int], k: int) -> int:
    # Obviously correct: sort and take the first k.
    return sum(sorted(prices)[:k])
