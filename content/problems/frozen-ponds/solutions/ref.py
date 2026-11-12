def pond_sizes(ice: list[str]) -> list[int]:
    h, w = len(ice), len(ice[0])
    parent = list(range(h * w))
    size = [1] * (h * w)

    def find(x: int) -> int:
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def union(a: int, b: int) -> None:
        a, b = find(a), find(b)
        if a != b:
            if size[a] < size[b]:
                a, b = b, a
            parent[b] = a
            size[a] += size[b]

    for r in range(h):
        row = ice[r]
        below = ice[r + 1] if r + 1 < h else None
        for c in range(w):
            if row[c] != "#":
                continue
            if c + 1 < w and row[c + 1] == "#":
                union(r * w + c, r * w + c + 1)
            if below is not None and below[c] == "#":
                union(r * w + c, (r + 1) * w + c)
    sizes = [size[r * w + c] for r in range(h) for c in range(w)
             if ice[r][c] == "#" and find(r * w + c) == r * w + c]
    return sorted(sizes, reverse=True)
