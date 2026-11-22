def biggest_club(n: int, merges: list[list[int]]) -> list[int]:
    # Obviously correct: keep each player's club label and relabel on every merge.
    club = list(range(n))
    out = []
    for a, b in merges:
        old, new = club[b], club[a]
        club = [new if c == old else c for c in club]
        out.append(max(club.count(c) for c in set(club)))
    return out
