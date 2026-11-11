def validate(inp):
    grid = inp["grid"]
    if not isinstance(grid, list) or not all(isinstance(row, list) for row in grid):
        return "grid is a 2-D array of integers"
    if not 1 <= len(grid) <= 200:
        return "1 ≤ rows ≤ 200"
    cols = len(grid[0])
    if not 1 <= cols <= 200 or any(len(row) != cols for row in grid):
        return "1 ≤ cols ≤ 200, the same for every row"
    if not all(type(x) is int and x in (0, 1) for row in grid for x in row):
        return "grid[r][c] is 0 or 1"
    return None
