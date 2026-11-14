# Obviously correct: every rectangle, its total added up cell by cell.
def main() -> None:
    rows, cols = map(int, input().split())
    grid = [list(map(int, input().split())) for _ in range(rows)]
    best = None
    for top in range(rows):
        for bottom in range(top, rows):
            for left in range(cols):
                for right in range(left, cols):
                    total = sum(grid[i][j] for i in range(top, bottom + 1) for j in range(left, right + 1))
                    if best is None or total > best:
                        best = total
    print(best)


main()
