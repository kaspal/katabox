import heapq


def ferry_times(n: int, routes: list[list[int]], queries: list[list[int]]) -> list[int]:
    # Obviously correct: Dijkstra from the start of every query.
    adj = [[] for _ in range(n)]
    for a, b, w in routes:
        adj[a].append((b, w))
        adj[b].append((a, w))
    out = []
    for s, t in queries:
        dist = {s: 0}
        heap = [(0, s)]
        while heap:
            d, x = heapq.heappop(heap)
            if d > dist[x]:
                continue
            for y, w in adj[x]:
                if d + w < dist.get(y, float("inf")):
                    dist[y] = d + w
                    heapq.heappush(heap, (d + w, y))
        out.append(dist.get(t, -1))
    return out
