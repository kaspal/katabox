# Obviously correct: read line by line and scan the row or column for every question.
def main() -> None:
    r, c = map(int, input().split())
    grid = [input() for _ in range(r)]
    q = int(input())
    for _ in range(q):
        kind, index = input().split()
        k = int(index)
        if kind == "row":
            print(sum(1 for j in range(c) if grid[k][j] == "#"))
        else:
            print(sum(1 for i in range(r) if grid[i][k] == "#"))


main()
