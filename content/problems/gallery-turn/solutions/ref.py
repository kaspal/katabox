def turn_right(grid: list[list[int]]) -> list[list[int]]:
    r, c = len(grid), len(grid[0])
    return [[grid[r - 1 - i][j] for i in range(r)] for j in range(c)]
