def best_pair(prices: list[int], budget: int) -> int:
    # Correct but O(n²): must run out of time on the max-size tests.
    best = -1
    n = len(prices)
    for i in range(n):
        for j in range(i + 1, n):
            total = prices[i] + prices[j]
            if total <= budget and total > best:
                best = total
    return best
