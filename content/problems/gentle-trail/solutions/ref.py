import heapq


def gentlest_climb(heights: list[list[int]]) -> int:
    rows, cols = len(heights), len(heights[0])
    end = rows * cols - 1
    flat = [h for row in heights for h in row]
    best = [float("inf")] * (rows * cols)
    best[0] = 0
    heap = [(0, 0)]
    while heap:
        d, i = heapq.heappop(heap)
        if i == end:
            return d
        if d > best[i]:
            continue
        r, c = divmod(i, cols)
        h = flat[i]
        for j, ok in ((i - cols, r > 0), (i + cols, r + 1 < rows), (i - 1, c > 0), (i + 1, c + 1 < cols)):
            if ok:
                nd = abs(flat[j] - h)
                if nd < d:
                    nd = d
                if nd < best[j]:
                    best[j] = nd
                    heapq.heappush(heap, (nd, j))
    return 0
