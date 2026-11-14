# Obviously correct: walk through every stretch and keep the lowest level.
def main() -> None:
    n = int(input())
    levels = list(map(int, input().split()))
    assert len(levels) == n
    q = int(input())
    for _ in range(q):
        l, r = map(int, input().split())
        lowest = levels[l - 1]
        for i in range(l - 1, r):
            if levels[i] < lowest:
                lowest = levels[i]
        print(lowest)


main()
