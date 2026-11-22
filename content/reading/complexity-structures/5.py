def f(n):
    grid = []
    for i in range(n):
        row = []
        for j in range(n):
            row.append((i * j) % 10)
        grid.append(row)
    return grid[n - 1][n - 1]
