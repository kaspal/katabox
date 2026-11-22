from collections import deque


def hops_to_all(n: int, links: list[list[int]]) -> list[int]:
    adj: list[list[int]] = [[] for _ in range(n)]
    for a, b in links:
        adj[a].append(b)
        adj[b].append(a)
    dist = [-1] * n
    dist[0] = 0
    q = deque([0])
    while q:
        u = q.popleft()
        for v in adj[u]:
            if dist[v] == -1:
                dist[v] = dist[u] + 1
                q.append(v)
    return dist
