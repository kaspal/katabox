def toll_refunds(n: int, roads: list[list[int]], start: int) -> list[int]:
    unreached = 10**9
    inf = float("inf")
    cost = [inf] * n
    cost[start] = 0
    for _ in range(n - 1):
        changed = False
        for a, b, c in roads:
            if cost[a] != inf and cost[a] + c < cost[b]:
                cost[b] = cost[a] + c
                changed = True
        if not changed:
            break
    for a, b, c in roads:
        if cost[a] != inf and cost[a] + c < cost[b]:
            return []
    return [unreached if x == inf else int(x) for x in cost]
