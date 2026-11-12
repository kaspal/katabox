def best_week(sales: list[int], k: int) -> int:
    # Correct but O(n·k): every window summed from scratch.
    best = 0
    for start in range(len(sales) - k + 1):
        total = 0
        for i in range(start, start + k):
            total += sales[i]
        if total > best:
            best = total
    return best
