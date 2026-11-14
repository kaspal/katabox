def longest_within(a, limit):
    best, best_start = 0, 0
    total, lo = 0, 0
    for hi in range(len(a)):
        total += a[hi]  # @step grow
        while total > limit:
            total -= a[lo]  # @step shrink
            lo += 1
        if hi - lo + 1 > best:  # @step check
            best, best_start = hi - lo + 1, lo
    return [best, best_start]
