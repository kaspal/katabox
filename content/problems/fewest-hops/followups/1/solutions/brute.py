def count_routes(n: int, links: list[list[int]]) -> int:
    # Obviously correct for small meshes: every simple route, keep the shortest.
    adj = [set() for _ in range(n)]
    for a, b in links:
        adj[a].add(b)
        adj[b].add(a)
    lengths = []

    def walk(u, seen, hops):
        if u == n - 1:
            lengths.append(hops)
            return
        for v in sorted(adj[u]):
            if v not in seen:
                seen.add(v)
                walk(v, seen, hops + 1)
                seen.remove(v)

    walk(0, {0}, 0)
    if not lengths:
        return 0
    best = min(lengths)
    return lengths.count(best) % 1_000_003
