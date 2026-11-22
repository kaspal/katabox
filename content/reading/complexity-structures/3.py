def f(n):
    evens = list(range(0, 2 * n, 2))
    total = 0
    for i in range(n):
        q = 2 * i + 1
        lo, hi = 0, n
        while lo < hi:
            mid = (lo + hi) // 2
            if evens[mid] < q:
                lo = mid + 1
            else:
                hi = mid
        total += lo
    return total
