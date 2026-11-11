def shortest_route(grid: list[list[int]]) -> int:
    # Obviously correct: relax every cell from its neighbours until nothing improves.
    rows, cols = len(grid), len(grid[0])
    inf = float("inf")
    dist = [[inf] * cols for _ in range(rows)]
    if grid[0][0] == 0:
        dist[0][0] = 0
    changed = True
    while changed:
        changed = False
        for r in range(rows):
            for c in range(cols):
                if grid[r][c] == 1:
                    continue
                for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                    if 0 <= nr < rows and 0 <= nc < cols and dist[nr][nc] + 1 < dist[r][c]:
                        dist[r][c] = dist[nr][nc] + 1
                        changed = True
    d = dist[rows - 1][cols - 1]
    return -1 if d == inf else int(d)
