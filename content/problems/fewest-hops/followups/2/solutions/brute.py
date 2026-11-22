from collections import deque


def hops_one_way(n: int, links: list[list[int]]) -> int:
    # Obviously correct: a plain search with no link turned, then with each link turned.
    def search(edges: list[list[int]]) -> int:
        adj: list[list[int]] = [[] for _ in range(n)]
        for a, b in edges:
            adj[a].append(b)
        dist = [-1] * n
        dist[0] = 0
        q = deque([0])
        while q:
            u = q.popleft()
            for v in adj[u]:
                if dist[v] == -1:
                    dist[v] = dist[u] + 1
                    q.append(v)
        return dist[n - 1]

    found = [search(links)]
    for i in range(len(links)):
        a, b = links[i]
        found.append(search(links[:i] + [[b, a]] + links[i + 1 :]))
    found = [d for d in found if d != -1]
    return min(found) if found else -1
