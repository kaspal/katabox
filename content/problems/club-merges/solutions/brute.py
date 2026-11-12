def clubs_left(n: int, merges: list[list[int]]) -> list[int]:
    # Obviously correct: keep the clubs as sets and count them.
    clubs = [{i} for i in range(n)]
    out = []
    for a, b in merges:
        ca = next(c for c in clubs if a in c)
        cb = next(c for c in clubs if b in c)
        if ca is not cb:
            clubs.remove(cb)
            ca |= cb
        out.append(len(clubs))
    return out
