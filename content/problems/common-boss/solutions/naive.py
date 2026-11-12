def common_boss(boss: list[int], pairs: list[list[int]]) -> list[int]:
    # Correct but O(q · depth): mark everyone above a, then walk up from b.
    out = []
    for a, b in pairs:
        above = set()
        v = a
        while v >= 0:
            above.add(v)
            v = boss[v]
        v = b
        while v not in above:
            v = boss[v]
        out.append(v)
    return out
