import heapq


def cheapest_crossing(tolls: list[list[int]]) -> int:
    # Obviously correct: for every choice of coupon district (or none), a plain
    # cheapest path with that district's toll halved.
    rows, cols = len(tolls), len(tolls[0])

    def plain(grid):
        dist = {(0, 0): grid[0][0]}
        heap = [(grid[0][0], 0, 0)]
        while heap:
            d, r, c = heapq.heappop(heap)
            if d > dist[(r, c)]:
                continue
            for nr, nc in ((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)):
                if 0 <= nr < rows and 0 <= nc < cols and d + grid[nr][nc] < dist.get((nr, nc), float("inf")):
                    dist[(nr, nc)] = d + grid[nr][nc]
                    heapq.heappush(heap, (d + grid[nr][nc], nr, nc))
        return dist[(rows - 1, cols - 1)]

    best = plain(tolls)
    for r in range(rows):
        for c in range(cols):
            grid = [row[:] for row in tolls]
            grid[r][c] //= 2
            best = min(best, plain(grid))
    return best
