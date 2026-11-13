def kth_gap(posts: list[int], k: int) -> int:
    # Obviously correct: list every gap and sort.
    n = len(posts)
    gaps = sorted(abs(posts[i] - posts[j]) for i in range(n) for j in range(i + 1, n))
    return gaps[k - 1]
