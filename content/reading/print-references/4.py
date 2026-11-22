grid = [[0] * 3 for _ in range(2)]
grid[1][2] = 7
row = grid[0]
row[0] = 1
for r in grid:
    print(" ".join(map(str, r)))
