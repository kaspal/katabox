def total(a, lo, hi):
    if lo == hi:  # @step check
        return a[lo]  # @step leaf
    mid = (lo + hi) // 2  # @step split
    left = total(a, lo, mid)
    right = total(a, mid + 1, hi)
    return left + right  # @step combine
