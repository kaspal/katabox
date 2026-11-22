def within_fixes(typed: str, meant: str, k: int) -> bool:
    n, m = len(typed), len(meant)
    if abs(n - m) > k:
        return False
    big = k + 1
    # Row i keeps only columns j with |i - j| <= k (the band); outside it the distance > k.
    prev = {j: j for j in range(0, min(m, k) + 1)}
    for i in range(1, n + 1):
        cur = {}
        lo, hi = max(0, i - k), min(m, i + k)
        for j in range(lo, hi + 1):
            if j == 0:
                cur[j] = i
                continue
            best = prev.get(j - 1, big) + (typed[i - 1] != meant[j - 1])
            best = min(best, prev.get(j, big) + 1, cur.get(j - 1, big) + 1)
            cur[j] = min(best, big)
        prev = cur
    return prev.get(m, big) <= k
