def smallest_squad(skills: int, people: list[int]) -> int:
    full = (1 << skills) - 1
    moves = sorted(set(people))
    dist = [-1] * (full + 1)
    dist[0] = 0
    frontier = [0]
    while frontier:
        nxt = []
        for mask in frontier:
            if mask == full:
                return dist[mask]
            d = dist[mask] + 1
            for p in moves:
                m = mask | p
                if dist[m] < 0:
                    dist[m] = d
                    nxt.append(m)
        frontier = nxt
    return -1
