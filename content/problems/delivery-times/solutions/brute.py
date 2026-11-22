def delivery_times(n: int, roads: list[list[int]], start: int) -> list[int]:
    # Obviously correct: relax every road until nothing improves (Bellman-Ford).
    inf = float("inf")
    dist = [inf] * n
    dist[start] = 0
    for _ in range(n):
        for a, b, w in roads:
            if dist[a] + w < dist[b]:
                dist[b] = dist[a] + w
            if dist[b] + w < dist[a]:
                dist[a] = dist[b] + w
    return [-1 if d == inf else d for d in dist]
