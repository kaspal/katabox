from bisect import bisect_left


def find_labels(shelf: list[int], labels: list[int]) -> list[int]:
    n = len(shelf)
    # The start of the original order: the smallest number.
    lo, hi = 0, n - 1
    while lo < hi:
        m = (lo + hi) // 2
        if shelf[m] > shelf[hi]:
            lo = m + 1
        else:
            hi = m
    start = lo
    ordered = shelf[start:] + shelf[:start]
    out = []
    for x in labels:
        k = bisect_left(ordered, x)
        out.append((k + start) % n if k < n and ordered[k] == x else -1)
    return out
