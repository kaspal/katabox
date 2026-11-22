def cheapest_with_pass(n: int, roads: list[list[int]], start: int, goal: int) -> int:
    # Obviously correct: plain shortest paths with no free road, then with each road free.
    def shortest(rs: list[list[int]]) -> float:
        inf = float("inf")
        dist = [inf] * n
        dist[start] = 0
        for _ in range(n):
            for a, b, t in rs:
                dist[b] = min(dist[b], dist[a] + t)
                dist[a] = min(dist[a], dist[b] + t)
        return dist[goal]

    found = [shortest(roads)] + [shortest(roads[:i] + [[a, b, 0]] + roads[i + 1 :]) for i, (a, b, _) in enumerate(roads)]
    best = min(found)
    return -1 if best == float("inf") else int(best)
