from itertools import combinations


def most_apples(grid: list[list[int]]) -> int:
    # Obviously correct: every path is a choice of which of its moves go down.
    rows, cols = len(grid), len(grid[0])
    moves = rows + cols - 2
    best = 0
    for downs in combinations(range(moves), rows - 1):
        r = c = 0
        total = grid[0][0]
        for step in range(moves):
            if step in downs:
                r += 1
            else:
                c += 1
            total += grid[r][c]
        best = max(best, total)
    return best
