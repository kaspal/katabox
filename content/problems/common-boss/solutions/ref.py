def common_boss(boss: list[int], pairs: list[list[int]]) -> list[int]:
    n = len(boss)
    children: list[list[int]] = [[] for _ in range(n)]
    president = 0
    for v, b in enumerate(boss):
        if b < 0:
            president = v
        else:
            children[b].append(v)
    depth = [0] * n
    order = [president]
    for v in order:  # breadth-first: the list grows while we walk it
        for c in children[v]:
            depth[c] = depth[v] + 1
            order.append(c)
    levels = max(1, (n - 1).bit_length())
    up = [[president if b < 0 else b for b in boss]]
    for _ in range(1, levels):
        prev = up[-1]
        up.append([prev[prev[v]] for v in range(n)])

    def decide(a: int, b: int) -> int:
        if depth[a] < depth[b]:
            a, b = b, a
        d, k = depth[a] - depth[b], 0
        while d:
            if d & 1:
                a = up[k][a]
            d >>= 1
            k += 1
        if a == b:
            return a
        for k in range(levels - 1, -1, -1):
            if up[k][a] != up[k][b]:
                a, b = up[k][a], up[k][b]
        return up[0][a]

    return [decide(a, b) for a, b in pairs]
