from itertools import permutations


def shortest_round(dist: list[list[int]]) -> int:
    # Obviously correct: try every order of the stops.
    n = len(dist)
    best = None
    for order in permutations(range(1, n)):
        route = (0,) + order + (0,)
        total = sum(dist[a][b] for a, b in zip(route, route[1:]))
        best = total if best is None else min(best, total)
    return best
