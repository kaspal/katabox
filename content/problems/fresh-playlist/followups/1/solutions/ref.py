def fresh_in_range(plays: list[int], requests: list[list[int]]) -> list[int]:
    n = len(plays)
    start = [0] * n
    last = {}
    left = 0
    for r, song in enumerate(plays):
        if last.get(song, -1) >= left:
            left = last[song] + 1
        last[song] = r
        start[r] = left
    table = [[r - start[r] + 1 for r in range(n)]]  # sparse table of maxima
    j = 1
    while 1 << j <= n:
        prev = table[-1]
        half = 1 << (j - 1)
        table.append([max(prev[i], prev[i + half]) for i in range(n - (1 << j) + 1)])
        j += 1
    out = []
    for a, b in requests:
        lo, hi = a, b + 1  # first r >= a with start[r] >= a
        while lo < hi:
            mid = (lo + hi) // 2
            if start[mid] >= a:
                hi = mid
            else:
                lo = mid + 1
        best = lo - a
        if lo <= b:
            k = (b - lo + 1).bit_length() - 1
            best = max(best, table[k][lo], table[k][b - (1 << k) + 1])
        out.append(best)
    return out
