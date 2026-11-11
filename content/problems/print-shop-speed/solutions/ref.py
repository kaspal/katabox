def min_speed(pages: list[int], hours: int) -> int:
    lo, hi = 1, max(pages)
    while lo < hi:
        mid = (lo + hi) // 2
        need = sum((p + mid - 1) // mid for p in pages)
        if need <= hours:
            hi = mid
        else:
            lo = mid + 1
    return lo
