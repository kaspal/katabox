def peak_index(heights: list[int]) -> int:
    lo, hi = 0, len(heights) - 1
    while lo < hi:
        m = (lo + hi) // 2
        if heights[m] < heights[m + 1]:
            lo = m + 1
        else:
            hi = m
    return lo
