def trapped_rain_grid(heights: list[list[int]]) -> int:
    # Obviously correct (slowly): raise the water level everywhere, then let every cell
    # that can drain to the edge through lower water fall, until nothing changes.
    rows, cols = len(heights), len(heights[0])
    top = max(max(r) for r in heights)
    level = [[top if 0 < r < rows - 1 and 0 < c < cols - 1 else heights[r][c] for c in range(cols)] for r in range(rows)]
    changed = True
    while changed:
        changed = False
        for r in range(1, rows - 1):
            for c in range(1, cols - 1):
                low = min(level[r + 1][c], level[r - 1][c], level[r][c + 1], level[r][c - 1])
                new = max(heights[r][c], low)
                if new < level[r][c]:
                    level[r][c] = new
                    changed = True
    return sum(level[r][c] - heights[r][c] for r in range(rows) for c in range(cols))
