import heapq


def delivery_times(n: int, roads: list[list[int]], start: int) -> list[int]:
    adj = [[] for _ in range(n)]
    for a, b, w in roads:
        adj[a].append((b, w))
        adj[b].append((a, w))
    dist = [-1] * n
    heap = [(0, start)]
    while heap:
        d, v = heapq.heappop(heap)
        if dist[v] != -1:
            continue
        dist[v] = d  # final only when it leaves the heap: nothing shorter is left
        for u, w in adj[v]:
            if dist[u] == -1:
                heapq.heappush(heap, (d + w, u))
    return dist
