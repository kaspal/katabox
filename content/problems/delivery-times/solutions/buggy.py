import heapq


def delivery_times(n: int, roads: list[list[int]], start: int) -> list[int]:
    adj = [[] for _ in range(n)]
    for a, b, w in roads:
        adj[a].append((b, w))
        adj[b].append((a, w))
    dist = [-1] * n
    dist[start] = 0
    heap = [(0, start)]
    while heap:
        d, v = heapq.heappop(heap)
        for u, w in adj[v]:
            if dist[u] == -1:
                dist[u] = d + w
                heapq.heappush(heap, (d + w, u))
    return dist
