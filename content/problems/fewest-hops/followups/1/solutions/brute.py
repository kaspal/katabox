def hops_to_all(n: int, links: list[list[int]]) -> list[int]:
    # Obviously correct: relax every link until nothing changes.
    inf = n + 1
    dist = [inf] * n
    dist[0] = 0
    changed = True
    while changed:
        changed = False
        for a, b in links:
            for u, v in ((a, b), (b, a)):
                if dist[u] + 1 < dist[v]:
                    dist[v] = dist[u] + 1
                    changed = True
    return [d if d < inf else -1 for d in dist]
