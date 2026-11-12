def cheapest_route(n: int, roads: list[list[int]], start: int, goal: int) -> int:
    # Obviously correct (for small graphs): Floyd-Warshall over every pair of towns.
    inf = float("inf")
    d = [[0 if i == j else inf for j in range(n)] for i in range(n)]
    for a, b, toll in roads:
        d[a][b] = min(d[a][b], toll)
        d[b][a] = min(d[b][a], toll)
    for k in range(n):
        for i in range(n):
            for j in range(n):
                if d[i][k] + d[k][j] < d[i][j]:
                    d[i][j] = d[i][k] + d[k][j]
    return -1 if d[start][goal] == inf else d[start][goal]
