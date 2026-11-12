def min_trips(weights: list[int], limit: int) -> int:
    w = sorted(weights)
    lo, hi = 0, len(w) - 1
    trips = 0
    while lo <= hi:
        if w[lo] + w[hi] <= limit:
            lo += 1
        hi -= 1
        trips += 1
    return trips
