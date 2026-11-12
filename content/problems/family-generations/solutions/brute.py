def generations(parent: list[int]) -> int:
    # Obviously correct: count the people on the way up from everyone to the founder.
    best = 0
    for i in range(len(parent)):
        steps, p = 1, parent[i]
        while p >= 0:
            steps += 1
            p = parent[p]
        best = max(best, steps)
    return best
