# Obviously correct: every rectangle, checked square by square.
def main() -> None:
    r, c = map(int, input().split())
    rows = [input() for _ in range(r)]
    best = 0
    for top in range(r):
        for bottom in range(top, r):
            for left in range(c):
                for right in range(left, c):
                    if all(rows[y][x] == "." for y in range(top, bottom + 1) for x in range(left, right + 1)):
                        best = max(best, (bottom - top + 1) * (right - left + 1))
    print(best)


main()
