def digit_budget(limit: int, total: int) -> int:
    # Correct but O(limit): check every ticket.
    count = 0
    for t in range(1, limit + 1):
        s, x = 0, t
        while x:
            s += x % 10
            x //= 10
        if s == total:
            count += 1
    return count
