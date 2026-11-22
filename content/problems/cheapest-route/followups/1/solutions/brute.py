def cheapest_within(n: int, roads: list[list[int]], start: int, goal: int, k: int) -> int:
    # Obviously correct for tiny meshes: every walk of at most k roads.
    best = -1

    def walk(at: int, used: int, cost: int) -> None:
        nonlocal best
        if at == goal and (best == -1 or cost < best):
            best = cost
        if used == k:
            return
        for a, b, t in roads:
            if a == at:
                walk(b, used + 1, cost + t)
            if b == at:
                walk(a, used + 1, cost + t)

    walk(start, 0, 0)
    return best
