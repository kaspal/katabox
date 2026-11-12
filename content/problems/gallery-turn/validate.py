def validate(inp):
    g = inp["grid"]
    if not isinstance(g, list) or not all(isinstance(row, list) and all(type(x) is int for x in row) for row in g):
        return "grid is a 2-D array of integers"
    if not 1 <= len(g) <= 300 or not 1 <= len(g[0]) <= 300 or any(len(row) != len(g[0]) for row in g):
        return "1 ≤ r, c ≤ 300, where r = grid.length and c = grid[i].length (every row has the same length)"
    if not all(0 <= x <= 255 for row in g for x in row):
        return "0 ≤ grid[i][j] ≤ 255"
    return None
