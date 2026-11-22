def clubs_with_undo(n: int, ops: list[list[int]]) -> list[int]:
    # Obviously correct: keep the list of merges in effect and recount from scratch.
    active: list[tuple[int, int]] = []
    out = []
    for op in ops:
        if op[0] == 0:
            if active:
                active.pop()
        else:
            active.append((op[1], op[2]))
        club = list(range(n))
        for a, b in active:
            old, new = club[b], club[a]
            club = [new if c == old else c for c in club]
        out.append(len(set(club)))
    return out
