import sys
from operator import add


def main() -> None:
    data = sys.stdin.buffer.read().split()
    rows, cols = int(data[0]), int(data[1])
    scores = list(map(int, data[2 : 2 + rows * cols]))
    grid = [scores[i * cols : (i + 1) * cols] for i in range(rows)]
    if rows > cols:
        # Put the shorter side in the outer loops.
        grid = [list(column) for column in zip(*grid)]
        rows, cols = cols, rows
    best = grid[0][0]
    for top in range(rows):
        col = [0] * cols
        for bottom in range(top, rows):
            col = list(map(add, col, grid[bottom]))  # column sums, in C
            cur = 0  # Kadane: the best run of columns ending at x
            for x in col:
                cur = cur + x if cur > 0 else x
                if cur > best:
                    best = cur
    print(best)


main()
