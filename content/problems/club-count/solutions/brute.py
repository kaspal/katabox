def club_count(n: int, friendships: list[list[int]]) -> list[int]:
    # Obviously correct: keep each club as a set and merge the two sets.
    club = {i: {i} for i in range(n)}
    out = []
    for a, b in friendships:
        if club[a] is not club[b]:
            merged = club[a] | club[b]
            for x in merged:
                club[x] = merged
        out.append(len({id(s) for s in club.values()}))
    return out
