def kinship_distances(parent: list[int], pairs: list[list[int]]) -> list[int]:
    n = len(parent)
    children: list[list[int]] = [[] for _ in range(n)]
    root = 0
    for i, p in enumerate(parent):
        if p == -1:
            root = i
        else:
            children[p].append(i)
    depth = [0] * n
    order = [root]
    for u in order:  # breadth-first: parents before children
        for v in children[u]:
            depth[v] = depth[u] + 1
            order.append(v)
    levels = max(1, (n - 1).bit_length())
    up = [[root if p == -1 else p for p in parent]]
    for _ in range(1, levels):
        prev = up[-1]
        up.append([prev[prev[v]] for v in range(n)])

    def ancestor(v: int, steps: int) -> int:
        k = 0
        while steps:
            if steps & 1:
                v = up[k][v]
            steps >>= 1
            k += 1
        return v

    out = []
    for a, b in pairs:
        x, y = (a, b) if depth[a] >= depth[b] else (b, a)
        x = ancestor(x, depth[x] - depth[y])
        if x != y:
            for k in range(levels - 1, -1, -1):
                if up[k][x] != up[k][y]:
                    x, y = up[k][x], up[k][y]
            x = up[0][x]
        out.append(depth[a] + depth[b] - 2 * depth[x])
    return out
