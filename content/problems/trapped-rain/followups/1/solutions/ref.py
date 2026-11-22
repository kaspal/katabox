import heapq


def trapped_rain_grid(heights: list[list[int]]) -> int:
    rows, cols = len(heights), len(heights[0])
    seen = [[False] * cols for _ in range(rows)]
    heap: list[tuple[int, int, int]] = []
    for r in range(rows):
        for c in range(cols):
            if r in (0, rows - 1) or c in (0, cols - 1):
                seen[r][c] = True
                heap.append((heights[r][c], r, c))
    heapq.heapify(heap)
    water = 0
    # Always grow from the lowest point of the wall around the unseen inside.
    while heap:
        level, r, c = heapq.heappop(heap)
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < rows and 0 <= nc < cols and not seen[nr][nc]:
                seen[nr][nc] = True
                h = heights[nr][nc]
                water += max(0, level - h)
                heapq.heappush(heap, (max(level, h), nr, nc))
    return water
