import heapq


def cheapest_route(n: int, roads: list[list[int]], start: int, goal: int) -> int:
    adj = [[] for _ in range(n)]
    for a, b, toll in roads:
        adj[a].append((b, toll))
        adj[b].append((a, toll))
    inf = float("inf")
    dist = [inf] * n
    dist[start] = 0
    heap = [(0, start)]
    while heap:
        d, t = heapq.heappop(heap)
        if d > dist[t]:
            continue
        if t == goal:
            return d
        for u, toll in adj[t]:
            nd = d + toll
            if nd < dist[u]:
                dist[u] = nd
                heapq.heappush(heap, (nd, u))
    return -1
