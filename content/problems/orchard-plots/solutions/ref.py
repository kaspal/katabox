def plot_totals(grid: list[list[int]], queries: list[list[int]]) -> list[int]:
    cols = len(grid[0])
    pre = [[0] * (cols + 1)]
    for row in grid:
        above, cur, run = pre[-1], [0] * (cols + 1), 0
        for c in range(cols):
            run += row[c]
            cur[c + 1] = above[c + 1] + run
        pre.append(cur)
    return [pre[b + 1][r + 1] - pre[t][r + 1] - pre[b + 1][l] + pre[t][l] for t, l, b, r in queries]
