def exponential_search(a, target):
    if a[0] == target:  # @step first
        return 0
    bound = 1
    while bound < len(a) and a[bound] < target:  # @step double
        bound *= 2
    lo, hi = bound // 2, min(bound, len(a) - 1)  # @step range
    while lo <= hi:
        mid = (lo + hi) // 2  # @step mid
        if a[mid] == target:
            return mid  # @step found
        if a[mid] < target:  # @step compare
            lo = mid + 1
        else:
            hi = mid - 1
    return -1  # @step missing
