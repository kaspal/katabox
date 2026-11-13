# Correct but slow: every rectangle. For each pair of rows, keep running column sums, then
# for each left column extend the right edge with a running total: O(R²·C²).
import sys


def main() -> None:
    data = sys.stdin.buffer.read().split()
    rows, cols = int(data[0]), int(data[1])
    scores = list(map(int, data[2 : 2 + rows * cols]))
    grid = [scores[i * cols : (i + 1) * cols] for i in range(rows)]
    best = grid[0][0]
    for top in range(rows):
        col = [0] * cols
        for bottom in range(top, rows):
            row = grid[bottom]
            for j in range(cols):
                col[j] += row[j]
            for left in range(cols):
                total = 0
                for right in range(left, cols):
                    total += col[right]
                    if total > best:
                        best = total
    print(best)


main()
