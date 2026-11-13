def blocked_walks(rows: int, cols: int, blocked: list[list[int]]) -> int:
    # Obviously correct (small grids): count walks crossing by crossing.
    closed = set(map(tuple, blocked))
    ways = [[0] * (cols + 1) for _ in range(rows + 1)]
    for r in range(rows + 1):
        for c in range(cols + 1):
            if (r, c) in closed:
                continue
            if r == 0 and c == 0:
                ways[r][c] = 1
            else:
                ways[r][c] = (ways[r - 1][c] if r else 0) + (ways[r][c - 1] if c else 0)
    return ways[rows][cols] % 1_000_003
