def most_apples(grid: list[list[int]]) -> int:
    cols = len(grid[0])
    best = [0] * cols
    for r, row in enumerate(grid):
        for c in range(cols):
            if r == 0:
                prev = best[c - 1] if c > 0 else 0
            elif c == 0:
                prev = best[0]
            else:
                prev = best[c] if best[c] > best[c - 1] else best[c - 1]
            best[c] = row[c] + prev
    return best[-1]
