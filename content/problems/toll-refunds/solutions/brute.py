def toll_refunds(n: int, roads: list[list[int]], start: int) -> list[int]:
    # Obviously correct (small n): Floyd–Warshall on every pair; a negative loop shows as
    # a negative dist[v][v] for a town v reachable from start.
    inf = float("inf")
    d = [[0 if i == j else inf for j in range(n)] for i in range(n)]
    for a, b, c in roads:
        d[a][b] = min(d[a][b], c)
    for k in range(n):
        for i in range(n):
            for j in range(n):
                if d[i][k] + d[k][j] < d[i][j]:
                    d[i][j] = d[i][k] + d[k][j]
    if any(d[start][v] != inf and d[v][v] < 0 for v in range(n)):
        return []
    return [10**9 if d[start][v] == inf else d[start][v] for v in range(n)]
