def binary_search(a, target):
    lo, hi = 0, len(a) - 1
    while lo <= hi:
        mid = (lo + hi) // 2  # @step mid
        if a[mid] == target:
            return mid  # @step found
        if a[mid] < target:  # @step compare
            lo = mid + 1
        else:
            hi = mid - 1
    return -1  # @step missing
