def lower_bound(a, target):
    lo, hi = 0, len(a)
    while lo < hi:
        mid = (lo + hi) // 2
        if a[mid] < target:  # @step lower
            lo = mid + 1
        else:
            hi = mid
    return lo  # @step lower-done


def upper_bound(a, target):
    lo, hi = 0, len(a)
    while lo < hi:
        mid = (lo + hi) // 2
        if a[mid] <= target:  # @step upper
            lo = mid + 1
        else:
            hi = mid
    return lo  # @step upper-done
