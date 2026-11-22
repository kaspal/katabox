from collections import deque

MOD = 1_000_003


def count_routes(n: int, links: list[list[int]]) -> int:
    adj = [[] for _ in range(n)]
    for a, b in links:
        adj[a].append(b)
        adj[b].append(a)
    dist = [-1] * n
    ways = [0] * n
    dist[0], ways[0] = 0, 1
    queue = deque([0])
    while queue:
        u = queue.popleft()
        for v in adj[u]:
            if dist[v] == -1:
                dist[v] = dist[u] + 1
                ways[v] = ways[u]
                queue.append(v)
            elif dist[v] == dist[u] + 1:
                ways[v] = (ways[v] + ways[u]) % MOD
    return ways[n - 1] % MOD
