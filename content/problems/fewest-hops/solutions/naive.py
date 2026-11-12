def fewest_hops(n: int, links: list[list[int]]) -> int:
    # Correct but O(n·m): relax every link in both directions until nothing improves.
    inf = n + 1
    dist = [inf] * n
    dist[0] = 0
    changed = True
    while changed:
        changed = False
        for a, b in links:
            if dist[a] + 1 < dist[b]:
                dist[b] = dist[a] + 1
                changed = True
            if dist[b] + 1 < dist[a]:
                dist[a] = dist[b] + 1
                changed = True
    return dist[n - 1] if dist[n - 1] < inf else -1
