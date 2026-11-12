def best_week(sales: list[int], k: int) -> int:
    # Obviously correct: the sum of every window.
    return max(sum(sales[i:i + k]) for i in range(len(sales) - k + 1))
