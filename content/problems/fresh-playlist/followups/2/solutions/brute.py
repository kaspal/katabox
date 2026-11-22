def longest_rotation(plays: list[int], k: int) -> int:
    # Obviously correct: every stretch, counting its different songs.
    best = 0
    for i in range(len(plays)):
        for j in range(i, len(plays)):
            if len(set(plays[i : j + 1])) <= k:
                best = max(best, j - i + 1)
    return best
