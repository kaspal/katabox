INF = 999


def bellman_ford(n, edges, source):
    dist = [INF] * n
    dist[source] = 0
    for _ in range(n - 1):
        changed = False
        for u, v, w in edges:
            if dist[u] != INF and dist[u] + w < dist[v]:
                dist[v] = dist[u] + w  # @step relax
                changed = True
        if not changed:
            break  # @step settled
    for u, v, w in edges:
        if dist[u] != INF and dist[u] + w < dist[v]:
            return [1]  # @step negative-cycle
    return [0] + dist
