def most_apples(grid: list[list[int]]) -> int:
    # Correct but exponential: explore every path recursively, with no memory of
    # trees already solved.
    rows, cols = len(grid), len(grid[0])

    def go(r, c):
        here = grid[r][c]
        if r == rows - 1 and c == cols - 1:
            return here
        best = -1
        if r + 1 < rows:
            best = max(best, go(r + 1, c))
        if c + 1 < cols:
            best = max(best, go(r, c + 1))
        return here + best

    import sys
    sys.setrecursionlimit(10_000)
    return go(0, 0)
