from collections import deque


def zero_one_bfs(n, adj, source):
    dist = [-1] * n
    dist[source] = 0
    queue = deque([source])
    while queue:
        u = queue.popleft()  # @step pop
        for v, w in adj[u]:
            if dist[v] == -1 or dist[u] + w < dist[v]:
                dist[v] = dist[u] + w
                if w == 0:
                    queue.appendleft(v)  # @step front
                else:
                    queue.append(v)  # @step back
    return dist
