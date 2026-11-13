def ferry_times(n: int, routes: list[list[int]], queries: list[list[int]]) -> list[int]:
    inf = float("inf")
    dist = [[inf] * n for _ in range(n)]
    for i in range(n):
        dist[i][i] = 0
    for a, b, w in routes:
        if w < dist[a][b]:
            dist[a][b] = dist[b][a] = w
    for k in range(n):
        dk = dist[k]
        for i in range(n):
            dik = dist[i][k]
            if dik == inf:
                continue
            di = dist[i]
            for j in range(n):
                if dik + dk[j] < di[j]:
                    di[j] = dik + dk[j]
    return [int(dist[a][b]) if dist[a][b] != inf else -1 for a, b in queries]
