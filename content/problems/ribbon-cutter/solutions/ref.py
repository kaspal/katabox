def longest_piece(ribbons: list[int], pieces: int) -> int:
    if sum(ribbons) < pieces:
        return 0
    lo, hi = 1, max(ribbons)
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if sum(r // mid for r in ribbons) >= pieces:
            lo = mid
        else:
            hi = mid - 1
    return lo
