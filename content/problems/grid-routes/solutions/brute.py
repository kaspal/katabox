MOD = 1_000_000_007


def grid_routes(floor: list[str]) -> int:
    # Obviously correct: push route counts forward, cell by cell, in reading order.
    rows, cols = len(floor), len(floor[0])
    ways = [[0] * cols for _ in range(rows)]
    if floor[0][0] == ".":
        ways[0][0] = 1
    for r in range(rows):
        for c in range(cols):
            if floor[r][c] == "#":
                ways[r][c] = 0
                continue
            if r > 0:
                ways[r][c] += ways[r - 1][c]
            if c > 0:
                ways[r][c] += ways[r][c - 1]
            ways[r][c] %= MOD
    return ways[rows - 1][cols - 1]
