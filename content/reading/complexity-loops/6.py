def f(n):
    lo, hi = 0, n - 1
    target = n // 2
    pairs = 0
    while lo < hi:
        s = lo + hi
        if s < target:
            lo += 1
        elif s > target:
            hi -= 1
        else:
            pairs += 1
            lo += 1
            hi -= 1
    return pairs
