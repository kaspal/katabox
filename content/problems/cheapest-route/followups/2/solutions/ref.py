import heapq


def cheapest_with_pass(n: int, roads: list[list[int]], start: int, goal: int) -> int:
    adj: list[list[tuple[int, int]]] = [[] for _ in range(n)]
    for a, b, t in roads:
        adj[a].append((b, t))
        adj[b].append((a, t))
    inf = float("inf")
    dist = [[inf, inf] for _ in range(n)]  # [no pass used, pass used]
    dist[start][0] = 0
    heap = [(0, start, 0)]
    while heap:
        d, u, used = heapq.heappop(heap)
        if d > dist[u][used]:
            continue
        for v, t in adj[u]:
            if d + t < dist[v][used]:
                dist[v][used] = d + t
                heapq.heappush(heap, (d + t, v, used))
            if used == 0 and d < dist[v][1]:
                dist[v][1] = d
                heapq.heappush(heap, (d, v, 1))
    best = min(dist[goal])
    return best if best < inf else -1
