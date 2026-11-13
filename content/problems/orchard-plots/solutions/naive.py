def plot_totals(grid: list[list[int]], queries: list[list[int]]) -> list[int]:
    # Correct but O(rows · columns) per query: sum each plot row by row.
    return [sum(sum(grid[i][l:r + 1]) for i in range(t, b + 1)) for t, l, b, r in queries]
