# Obviously correct: Floyd-Warshall gives the trail count between every pair of
# junctions; the answer is the largest one.
def main() -> None:
    t = int(input())
    for _ in range(t):
        n = int(input())
        inf = float("inf")
        dist = [[0 if i == j else inf for j in range(n + 1)] for i in range(n + 1)]
        for _ in range(n - 1):
            a, b = map(int, input().split())
            dist[a][b] = dist[b][a] = 1
        for k in range(1, n + 1):
            for i in range(1, n + 1):
                for j in range(1, n + 1):
                    if dist[i][k] + dist[k][j] < dist[i][j]:
                        dist[i][j] = dist[i][k] + dist[k][j]
        print(max(dist[i][j] for i in range(1, n + 1) for j in range(1, n + 1)))


main()
