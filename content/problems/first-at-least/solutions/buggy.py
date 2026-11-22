def first_at_least(sorted: list[int], x: int) -> int:
    lo, hi = 0, len(sorted) - 1
    while lo < hi:
        mid = (lo + hi) // 2
        if sorted[mid] < x:
            lo = mid + 1
        else:
            hi = mid
    return lo
