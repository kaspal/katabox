def digit_budget(limit: int, total: int) -> int:
    # Obviously correct (small limits): check every ticket.
    return sum(1 for t in range(1, limit + 1) if sum(map(int, str(t))) == total)
