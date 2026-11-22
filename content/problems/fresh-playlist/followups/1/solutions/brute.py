def fresh_with_encores(plays: list[int]) -> int:
    # Obviously correct: every stretch, counted directly.
    best = 0
    for i in range(len(plays)):
        for j in range(i, len(plays)):
            part = plays[i : j + 1]
            if all(part.count(s) <= 2 for s in part):
                best = max(best, j - i + 1)
    return best
