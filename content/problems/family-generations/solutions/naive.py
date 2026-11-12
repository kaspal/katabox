def generations(parent: list[int]) -> int:
    # Correct but O(n · depth): walk up from every person to the founder.
    best = 0
    for i in range(len(parent)):
        steps, p = 1, parent[i]
        while p >= 0:
            steps += 1
            p = parent[p]
        if steps > best:
            best = steps
    return best
