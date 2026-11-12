def validate(inp):
    grid = inp["grid"]
    if not isinstance(grid, list) or not all(isinstance(r, list) for r in grid):
        return "grid is an array of arrays"
    if not 1 <= len(grid) <= 300:
        return "1 ≤ grid.length ≤ 300"
    cols = len(grid[0])
    if not 1 <= cols <= 300 or any(len(r) != cols for r in grid):
        return "1 ≤ grid[i].length ≤ 300, all rows the same length"
    if not all(type(x) is int and 0 <= x <= 100 for r in grid for x in r):
        return "0 ≤ grid[i][j] ≤ 100"
    return None
