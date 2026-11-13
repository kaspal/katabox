from itertools import product


def two_teams(n: int, rivalries: list[list[int]]) -> bool:
    # Obviously correct (small n): try every assignment of players to teams.
    return any(all(t[a] != t[b] for a, b in rivalries) for t in product((0, 1), repeat=n))
