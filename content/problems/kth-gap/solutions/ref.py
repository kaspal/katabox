from bisect import bisect_left


def kth_gap(posts: list[int], k: int) -> int:
    a = sorted(posts)
    lo, hi = 0, a[-1] - a[0]
    while lo < hi:
        mid = (lo + hi) // 2
        # Pairs (i, j) with i < j and a[j] - a[i] <= mid.
        count = sum(j - bisect_left(a, a[j] - mid, 0, j) for j in range(len(a)))
        if count >= k:
            hi = mid
        else:
            lo = mid + 1
    return lo
