def deepest_report(manager: list[int]) -> int:
    n = len(manager)
    depth = [0] * n  # 0 = not known yet; the boss has depth 1
    best = 0
    for v in range(n):
        # Climb to the first known ancestor without recursion, then fill the path back.
        path = []
        u = v
        while u != -1 and depth[u] == 0:
            path.append(u)
            u = manager[u]
        d = 0 if u == -1 else depth[u]
        for w in reversed(path):
            d += 1
            depth[w] = d
        best = max(best, depth[v])
    return best
