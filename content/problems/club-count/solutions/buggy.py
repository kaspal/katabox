def club_count(n: int, friendships: list[list[int]]) -> list[int]:
    parent = list(range(n))

    def find(x: int) -> int:
        while parent[x] != x:
            x = parent[x]
        return x

    clubs = n
    out = []
    for a, b in friendships:
        if find(a) != find(b):
            parent[a] = b
            clubs -= 1
        out.append(clubs)
    return out
