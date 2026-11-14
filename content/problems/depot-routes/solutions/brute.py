# Obviously correct: Bellman-Ford relaxation from the depot until nothing improves.
def main() -> None:
    n, m = map(int, input().split())
    roads = [tuple(map(int, input().split())) for _ in range(m)]
    dist: list[int | None] = [None] * (n + 1)
    dist[1] = 0
    changed = True
    while changed:
        changed = False
        for a, b, w in roads:
            for x, y in ((a, b), (b, a)):
                dx = dist[x]
                if dx is not None and (dist[y] is None or dx + w < dist[y]):
                    dist[y] = dx + w
                    changed = True
    q = int(input())
    for _ in range(q):
        d = dist[int(input())]
        print(-1 if d is None else d)


main()
