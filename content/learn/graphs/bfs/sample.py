from collections import deque


def bfs(n, adj, source):
    dist = [-1] * n
    dist[source] = 0
    queue = deque([source])
    while queue:
        u = queue.popleft()  # @step visit
        for v, _ in adj[u]:
            if dist[v] == -1:
                dist[v] = dist[u] + 1  # @step discover
                queue.append(v)
    return dist
