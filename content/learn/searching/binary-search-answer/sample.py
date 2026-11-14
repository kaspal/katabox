def days_needed(weights, capacity):
    days, load = 1, 0
    for w in weights:
        if load + w > capacity:
            days += 1
            load = 0
        load += w
    return days


def min_capacity(weights, days):
    lo, hi = max(weights), sum(weights)
    while lo < hi:
        mid = (lo + hi) // 2  # @step try
        if days_needed(weights, mid) <= days:  # @step check
            hi = mid
        else:
            lo = mid + 1
    return lo  # @step answer
