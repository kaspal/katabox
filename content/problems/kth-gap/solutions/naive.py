def kth_gap(posts: list[int], k: int) -> int:
    # Correct but O(n² + W): count every gap in a table indexed by its length.
    lo = min(posts)
    table = [0] * (max(posts) - lo + 1)
    n = len(posts)
    for i in range(n):
        for j in range(i + 1, n):
            table[abs(posts[i] - posts[j])] += 1
    for gap, c in enumerate(table):
        k -= c
        if k <= 0:
            return gap
    return -1
