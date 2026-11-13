def find_label(shelf: list[int], label: int) -> int:
    lo, hi = 0, len(shelf) - 1
    while lo <= hi:
        m = (lo + hi) // 2
        if shelf[m] == label:
            return m
        if shelf[lo] <= shelf[m]:
            if shelf[lo] <= label < shelf[m]:
                hi = m - 1
            else:
                lo = m + 1
        elif shelf[m] < label <= shelf[hi]:
            lo = m + 1
        else:
            hi = m - 1
    return -1
