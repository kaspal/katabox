def best_week(sales: list[int], k: int) -> int:
    window = sum(sales[:k])
    best = window
    for i in range(k, len(sales)):
        window += sales[i] - sales[i - k]
        if window > best:
            best = window
    return best
