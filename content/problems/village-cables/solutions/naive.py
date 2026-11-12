def cheapest_network(n: int, cables: list[list[int]]) -> int:
    # Correct but O(n·m): grow the network from village 0, each time rescanning every
    # cable for the cheapest one that reaches a new village (a simple Prim's algorithm).
    inside = [False] * n
    inside[0] = True
    total = 0
    for _ in range(n - 1):
        best = None
        for a, b, cost in cables:
            if inside[a] != inside[b] and (best is None or cost < best[2]):
                best = (a, b, cost)
        if best is None:
            return -1
        inside[best[0]] = inside[best[1]] = True
        total += best[2]
    return total
