def plot_totals(grid: list[list[int]], queries: list[list[int]]) -> list[int]:
    # Obviously correct: add up every tree of every plot.
    out = []
    for t, l, b, r in queries:
        total = 0
        for i in range(t, b + 1):
            for j in range(l, r + 1):
                total += grid[i][j]
        out.append(total)
    return out
