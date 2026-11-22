grid = [[0] * 3] * 2
grid[0][1] = 5
for row in grid:
    print(" ".join(map(str, row)))
