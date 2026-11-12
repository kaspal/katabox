def fewest_hops(n: int, links: list[list[int]]) -> int:
    near: list[list[int]] = [[] for _ in range(n)]
    for a, b in links:
        near[a].append(b)
        near[b].append(a)
    dist = [-1] * n
    dist[0] = 0
    queue = [0]
    for v in queue:  # the list grows while we walk it: a breadth-first order
        for u in near[v]:
            if dist[u] == -1:
                dist[u] = dist[v] + 1
                queue.append(u)
    return dist[n - 1]
