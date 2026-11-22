def deepest_report(manager: list[int]) -> int:
    # Obviously correct: walk every person up to the boss and count the steps.
    best = 0
    for v in range(len(manager)):
        d = 1
        while manager[v] != -1:
            v = manager[v]
            d += 1
        best = max(best, d)
    return best
