def cheapest_route(n: int, roads: list[list[int]], start: int, goal: int) -> int:
    # Correct but O(n·m): Bellman-Ford, relaxing every road round after round until
    # nothing changes.
    inf = float("inf")
    dist = [inf] * n
    dist[start] = 0
    changed = True
    while changed:
        changed = False
        for a, b, toll in roads:
            if dist[a] + toll < dist[b]:
                dist[b] = dist[a] + toll
                changed = True
            if dist[b] + toll < dist[a]:
                dist[a] = dist[b] + toll
                changed = True
    return -1 if dist[goal] == inf else dist[goal]
