# Obviously correct: fill a big grid cell by cell; big cell (y, x) comes from (y // K, x // K).
def main() -> None:
    r, c, k = map(int, input().split())
    rows = [input() for _ in range(r)]
    big = [[rows[y // k][x // k] for x in range(c * k)] for y in range(r * k)]
    for line in big:
        print("".join(line))


main()
