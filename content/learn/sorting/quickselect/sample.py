def quickselect(a, k):
    lo, hi = 0, len(a) - 1
    while True:
        p = partition(a, lo, hi)
        if p == k - 1:  # @step found
            return a[p]
        if p < k - 1:  # @step side
            lo = p + 1
        else:
            hi = p - 1


def partition(a, lo, hi):
    pivot = a[hi]  # @step pivot
    i = lo
    for j in range(lo, hi):
        if a[j] < pivot:  # @step compare
            a[i], a[j] = a[j], a[i]  # @step swap
            i += 1
    a[i], a[hi] = a[hi], a[i]  # @step place
    return i
