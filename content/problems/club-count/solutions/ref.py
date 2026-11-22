def club_count(n: int, friendships: list[list[int]]) -> list[int]:
    parent = list(range(n))

    def find(x: int) -> int:
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    clubs = n
    out = []
    for a, b in friendships:
        ra, rb = find(a), find(b)
        if ra != rb:
            parent[ra] = rb  # link the roots, so both whole clubs join
            clubs -= 1
        out.append(clubs)
    return out
