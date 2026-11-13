def fresh_stretch(plays: list[int]) -> int:
    # Obviously correct: every stretch, checked with a set.
    n = len(plays)
    return max(j - i for i in range(n) for j in range(i + 1, n + 1) if len(set(plays[i:j])) == j - i)
