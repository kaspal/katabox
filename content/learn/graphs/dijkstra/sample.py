import heapq


def dijkstra(n, adj, source):
    dist = [None] * n
    dist[source] = 0
    heap = [(0, source)]
    while heap:
        d, u = heapq.heappop(heap)  # @step pop
        if d > dist[u]:
            continue  # @step stale
        for v, w in adj[u]:
            if dist[v] is None or d + w < dist[v]:
                dist[v] = d + w  # @step relax
                heapq.heappush(heap, (dist[v], v))
    return [-1 if x is None else x for x in dist]
