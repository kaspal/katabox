def turn_right(grid: list[list[int]]) -> list[list[int]]:
    # Obviously correct: place every tile at (j, r - 1 - i).
    r, c = len(grid), len(grid[0])
    out = [[None] * r for _ in range(c)]
    for i in range(r):
        for j in range(c):
            out[j][r - 1 - i] = grid[i][j]
    return out
