from itertools import permutations


def shortest_round(dist: list[list[int]]) -> int:
    # Correct but O(n!): try every order of the stops.
    n = len(dist)
    best = None
    for order in permutations(range(1, n)):
        total, at = 0, 0
        for j in order:
            total += dist[at][j]
            at = j
        total += dist[at][0]
        if best is None or total < best:
            best = total
    return best
