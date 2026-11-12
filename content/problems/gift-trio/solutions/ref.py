def closest_trio(prices: list[int], target: int) -> int:
    p = sorted(prices)
    n = len(p)
    best = p[0] + p[1] + p[2]
    best_gap = abs(best - target)
    for i in range(n - 2):
        a = p[i]
        lo, hi = i + 1, n - 1
        while lo < hi:
            s = a + p[lo] + p[hi]
            gap = s - target
            if gap < 0:
                if -gap < best_gap or (-gap == best_gap and s < best):
                    best, best_gap = s, -gap
                lo += 1
            elif gap > 0:
                if gap < best_gap:
                    best, best_gap = s, gap
                hi -= 1
            else:
                return s
    return best
