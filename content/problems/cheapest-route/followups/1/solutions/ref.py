def cheapest_within(n: int, roads: list[list[int]], start: int, goal: int, k: int) -> int:
    inf = float("inf")
    best = [inf] * n
    best[start] = 0
    # After round r, best[v] is the cheapest toll to v using at most r roads.
    for _ in range(k):
        nxt = best[:]
        for a, b, t in roads:
            if best[a] + t < nxt[b]:
                nxt[b] = best[a] + t
            if best[b] + t < nxt[a]:
                nxt[a] = best[b] + t
        if nxt == best:
            break
        best = nxt
    return best[goal] if best[goal] < inf else -1
