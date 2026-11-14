from collections import deque


def farthest(adj, start):
    dist = [-1] * len(adj)
    dist[start] = 0
    queue, best = deque([start]), start
    while queue:
        node = queue.popleft()  # @step visit
        if dist[node] > dist[best]:
            best = node
        for nxt in adj[node]:
            if dist[nxt] == -1:
                dist[nxt] = dist[node] + 1  # @step reach
                queue.append(nxt)
    return best, dist[best]


def diameter(parents):
    n = len(parents) + 1
    adj = [[] for _ in range(n)]
    for child, parent in enumerate(parents, start=1):
        adj[parent].append(child)
        adj[child].append(parent)
    end, _ = farthest(adj, 0)  # @step first-bfs
    _, length = farthest(adj, end)  # @step second-bfs
    return [length]
