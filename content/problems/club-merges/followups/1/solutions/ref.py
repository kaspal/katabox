def _find(parent: list[int], x: int) -> int:
    while parent[x] != x:
        parent[x] = parent[parent[x]]
        x = parent[x]
    return x


def biggest_club(n: int, merges: list[list[int]]) -> list[int]:
    parent = list(range(n))
    size = [1] * n
    biggest = 1
    out = []
    for a, b in merges:
        ra, rb = _find(parent, a), _find(parent, b)
        if ra != rb:
            if size[ra] < size[rb]:
                ra, rb = rb, ra
            parent[rb] = ra
            size[ra] += size[rb]
            biggest = max(biggest, size[ra])
        out.append(biggest)
    return out
