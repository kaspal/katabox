def free_numbers(shelf: list[int], queries: list[int]) -> list[int]:
    n = len(shelf)
    lo, hi = 0, n - 1
    while lo < hi:  # the rotation point: the smallest folder
        mid = (lo + hi) // 2
        if shelf[mid] > shelf[hi]:
            lo = mid + 1
        else:
            hi = mid
    p = lo

    def free_below(i):
        return shelf[(p + i) % n] - (i + 1)

    out = []
    for k in queries:
        lo, hi = 0, n  # first i with free_below(i) >= k, or n
        while lo < hi:
            mid = (lo + hi) // 2
            if free_below(mid) >= k:
                hi = mid
            else:
                lo = mid + 1
        out.append(k + lo)
    return out
