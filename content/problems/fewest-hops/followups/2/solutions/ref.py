from collections import deque


def hops_one_way(n: int, links: list[list[int]]) -> int:
    fwd: list[list[int]] = [[] for _ in range(n)]
    back: list[list[int]] = [[] for _ in range(n)]
    for a, b in links:
        fwd[a].append(b)
        back[b].append(a)
    # State (station, flipped): flipped = 1 once the one turned-around link was used.
    dist = [[-1, -1] for _ in range(n)]
    dist[0][0] = 0
    q = deque([(0, 0)])
    while q:
        u, f = q.popleft()
        d = dist[u][f]
        for v in fwd[u]:
            if dist[v][f] == -1:
                dist[v][f] = d + 1
                q.append((v, f))
        if f == 0:
            for v in back[u]:
                if dist[v][1] == -1:
                    dist[v][1] = d + 1
                    q.append((v, 1))
    a, b = dist[n - 1]
    best = [x for x in (a, b) if x != -1]
    return min(best) if best else -1
