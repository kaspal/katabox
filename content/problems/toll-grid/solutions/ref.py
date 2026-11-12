import heapq
from collections.abc import Iterator


def neighbours(r: int, c: int, rows: int, cols: int) -> Iterator[tuple[int, int]]:
    if r > 0:
        yield r - 1, c
    if r + 1 < rows:
        yield r + 1, c
    if c > 0:
        yield r, c - 1
    if c + 1 < cols:
        yield r, c + 1


def cheapest_crossing(tolls: list[list[int]]) -> int:
    rows, cols = len(tolls), len(tolls[0])
    inf = float("inf")
    # cost[used][r][c]: the cheapest way found so far to stand on (r, c).
    cost = [[[inf] * cols for _ in range(rows)] for _ in range(2)]
    start = tolls[0][0]
    cost[0][0][0], cost[1][0][0] = start, start // 2
    heap = [(start, 0, 0, 0), (start // 2, 1, 0, 0)]
    while heap:
        d, used, r, c = heapq.heappop(heap)
        if d > cost[used][r][c]:
            continue
        for nr, nc in neighbours(r, c, rows, cols):
            toll = tolls[nr][nc]
            if d + toll < cost[used][nr][nc]:
                cost[used][nr][nc] = d + toll
                heapq.heappush(heap, (d + toll, used, nr, nc))
            if not used and d + toll // 2 < cost[1][nr][nc]:
                cost[1][nr][nc] = d + toll // 2
                heapq.heappush(heap, (d + toll // 2, 1, nr, nc))
    return min(cost[0][rows - 1][cols - 1], cost[1][rows - 1][cols - 1])
